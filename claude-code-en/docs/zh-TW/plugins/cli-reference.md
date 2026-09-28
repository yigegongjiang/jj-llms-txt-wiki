> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Plugin 命令參考

> claude plugin shell 命令的完整參考，包括在工作階段中的 /plugin 和 /reload-plugins，以及在單一工作階段中載入 plugin 的旗標。

您可以從 shell 或指令碼執行 plugin 命令，方式為 `claude plugin`，或在 Claude Code 工作階段內執行 `/plugin` 和 `/reload-plugins`。本參考提供每個命令的旗標、預設值、輸出和結束代碼，以及在單一工作階段中載入 plugin 的兩個旗標。

在您的組建上執行 `claude plugin --help` 以確認您的版本有哪些子命令。

<Note>
  這些情況涵蓋在其他頁面上：

  * **安裝和管理步驟，以及 `/plugin` 執行的位置**：請參閱 [安裝和管理 plugins](/docs/zh-TW/plugins/install)
  * **命令在磁碟上變更的內容以及哪個範圍優先**：請參閱 [Plugin 載入參考](/docs/zh-TW/plugins/loading)
  * **錯誤訊息的含義**：請參閱 [Troubleshoot plugins](/docs/zh-TW/plugins/troubleshooting)
</Note>

<h2 id="claude-plugin-commands">
  claude plugin 命令
</h2>

從 shell 或指令碼執行 `claude plugin <subcommand>`，在 Claude Code 工作階段外。這些子命令安裝和管理 plugins，而不開啟 [`/plugin`](#plugin-in-a-session) 面板。

`claude plugins` 是 `claude plugin` 的別名。

每個子命令共享這些結束代碼、plugin 引數和範圍值：

* **結束代碼**：成功時為 `0`，失敗時為 `1`。`validate` 為非預期錯誤新增結束 `2`，`eval` 新增 [其部分](#plugin-eval) 中列出的代碼。
* **Plugin 引數**：`<plugin>` 引數是 plugin `name` 或 `name@marketplace`。當兩個市場提供相同名稱時，使用限定形式。
* **範圍**：`--scope` 接受 `user`、`project` 或 `local`，並命名命令寫入的設定檔。`update` 也接受 `managed`。

<h3 id="plugin-init">
  plugin init
</h3>

在 `~/.claude/skills/<name>/` 建立新 plugin 的架構。它在您的下一個工作階段中以 `<name>@skills-dir` 的形式載入，無需安裝步驟。

`new` 是 `init` 的別名。

對於以此命令開始的建立、測試和編輯工作流程，請參閱 [建立 plugin](/docs/zh-TW/plugins/create)。

```bash theme={null}
claude plugin init <name> [options]
```

`<name>` 成為 `~/.claude/skills/` 下的目錄名稱和 plugin 的 manifest 中的 `name`。

該命令沒有另一個位置的旗標。若要改為在專案內建立架構，請參閱 [建立 plugin](/docs/zh-TW/plugins/create)。

| 旗標                       | 說明                                                                            |
| :----------------------- | :---------------------------------------------------------------------------- |
| `--description <text>`   | Manifest 說明                                                                   |
| `--author <name>`        | 作者名稱。預設為 `git config user.name`                                               |
| `--author-email <email>` | 作者電子郵件。預設為 `git config user.email`                                            |
| `--with <components...>` | 也為 `skills`、`agents`、`hooks`、`mcp`、`lsp`、`output-style` 或 `channel` 建立起始檔案的架構 |
| `-f, --force`            | 覆寫目標處的現有 `.claude-plugin/`                                                    |

使用起始 skill 和 hook 檔案建立 plugin 的架構：

```bash theme={null}
claude plugin init my-helper --with skills hooks
```

Claude Code 驗證其寫入的內容，並列印 `Created plugin "my-helper" at ~/.claude/skills/my-helper`，後面跟著它載入的 id 和關閉它的 `claude plugin disable` 命令。

當 Claude Code 無法安全地建立架構時，它會結束 `1` 而不寫入，訊息會命名原因。這些是常見原因：

* 未知的 `--with` 值
* 目標處現有的架構，沒有 `--force`
* 阻止 skills-directory plugins 的受管設定

<h3 id="plugin-install">
  plugin install
</h3>

從您已新增的市場安裝 plugin。`i` 是 `install` 的別名。

```bash theme={null}
claude plugin install <plugin> [options]
```

大多數 plugins 無需提示即可安裝。對於其市場項目 [執行命令以安裝它](/docs/zh-TW/plugins/host-marketplace) 或 [為其下載設定 `headersHelper`](/docs/zh-TW/plugins/host-marketplace#how-users-accept-a-headershelper-command) 的 plugin，Claude Code 首先列印命令並詢問 `Run this command now? [y/N]`。

| 旗標                          | 說明                                                                                                                                                                                      |
| :-------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>`       | 安裝範圍：`user`、`project` 或 `local`。預設為 `user`                                                                                                                                              |
| `--config <key=value>`      | 設定 plugin 的 manifest 宣告的 [`userConfig`](/docs/zh-TW/plugins/manifest-reference) 選項。為每個選項重複旗標。需要 Claude Code v2.1.147 或更新版本                                                                   |
| `-y, --yes`                 | 接受顯示的安裝命令，無需 `Run this command now?` 提示。當命令在 Claude Code 工作階段內執行時（例如從 Bash 工具或 hook）被忽略。需要 Claude Code v2.1.229 或更新版本                                                                   |
| `--accept-command <sha256>` | 接受顯示的安裝命令，其 `sha256` 先前的 [`--json` 執行](#plugin-json-result) 在 `shownCommand` 中報告，代替 `-y`。無法與 `-y` 結合。請參閱 [接受顯示的安裝命令](#accept-a-displayed-install-command)。需要 Claude Code v2.1.271 或更新版本 |
| `--json`                    | 將結果列印為 stdout 最後一行的一個 JSON 物件，而不是人類可讀的訊息，供指令碼使用。請參閱 [JSON 結果格式](#plugin-json-result)。需要 Claude Code v2.1.268 或更新版本                                                                      |

從您自己的終端傳遞 `-y` 以接受顯示的命令，無需提示。以下是沒有 TTY 和 Claude 執行命令時發生的情況：

* **stdin 或 stdout 不是 TTY，且您既不傳遞 `-y` 也不傳遞 `--accept-command`**：安裝被拒絕。輸出說命令只是顯示，結束代碼為 `1`
* **Claude 透過其 Bash 工具執行命令**：`-y` 被忽略。改為從您自己的終端執行命令

為複製專案的每個人安裝 plugin：

```bash theme={null}
claude plugin install formatter@my-marketplace --scope project
```

Claude Code 列印 `Successfully installed plugin: formatter@my-marketplace (scope: project)`。當沒有新安裝時，輸出說明原因：

* **已在該範圍安裝**：輸出為 `Plugin "formatter@my-marketplace" is already installed (scope: project)`，結束代碼為 `0`
* **您拒絕命令來源提示**：輸出為 `Aborted.`，結束代碼為 `1`
* **您拒絕 `headersHelper` 提示，或無法在沒有 TTY 的情況下確認**：輸出為 `Aborted — the command was not run.`，結束代碼為 `1`

<h4 id="plugin-json-result">
  JSON 結果格式
</h4>

當您將 `--json` 傳遞給 `plugin install` 時，stdout 的最後一行是一個 JSON 物件。只解析該行，因為 Claude Code 在其前面列印市場宣告的任何命令。

三個欄位始終存在：

* `command`：執行的子命令，例如 `install`
* `outcome`：`ok` 或 `failed`
* `message`：結果的人類可讀說明

其他欄位（例如 `pluginId`、`scope` 和 `failureCode`）僅在適用時出現。

使用錯誤（例如無效的 `--scope`）不列印結果行，結束 `1`，stderr 上有原因。

<h4 id="accept-a-displayed-install-command">
  接受顯示的安裝命令
</h4>

當 `--json` 執行顯示市場宣告的命令且不執行它時，`failed` 結果也會帶有 `shownCommand` 物件。其欄位包括顯示的命令、它所屬的 plugin 和命令的 `sha256`。

若要接受完全相同的命令，從您自己的終端使用該 `sha256` 作為 `--accept-command` 重新執行，因為旗標在 Claude Code 工作階段內無效。需要 Claude Code v2.1.271 或更新版本。

`sha256` 計為完全相同的命令、plugin 和市場目錄的接受。如果自命令顯示以來其中任何一個已變更，Claude Code 不接受 `sha256` 並再次顯示命令。執行自己的市場重新整理擷取的變更也計為此類變更。

如果 `shownCommand.acceptCommandMatched` 為 `false`，您傳遞的 `sha256` 與現在顯示的命令不符。在使用其 `sha256` 重新執行之前，檢查該命令。

<h3 id="plugin-uninstall">
  plugin uninstall
</h3>

從一個範圍移除已安裝的 plugin。`remove` 和 `rm` 是 `uninstall` 的別名。

```bash theme={null}
claude plugin uninstall <plugin> [options]
```

| 旗標                    | 說明                                                                                                                                 |
| :-------------------- | :--------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>` | 從範圍卸載：`user`、`project` 或 `local`。預設為 `user`                                                                                        |
| `--keep-data`         | 保留 plugin 的持久資料目錄 `~/.claude/plugins/data/<id>/`                                                                                   |
| `--prune`             | 也移除自動安裝的 [dependencies](/docs/zh-TW/plugins/dependencies)，沒有剩餘 plugin 需要                                                                |
| `-y, --yes`           | 跳過 `--prune` 確認提示。當 stdin 或 stdout 不是 TTY 時，需要與 `--prune` 一起使用                                                                     |
| `--json`              | 將結果列印為 stdout 最後一行的一個 JSON 物件，格式與 [`plugin install --json`](#plugin-json-result) 相同。無法與 `--prune` 結合。需要 Claude Code v2.1.268 或更新版本 |

從專案範圍卸載 plugin：

```bash theme={null}
claude plugin uninstall formatter@my-marketplace --scope project
```

Claude Code 列印 `Successfully uninstalled plugin: formatter (scope: project)`。當 plugin 未在該範圍安裝時，命令列印以 `Failed to uninstall plugin "formatter@my-marketplace":` 開頭的行，並結束 `1`。

<h3 id="plugin-enable">
  plugin enable
</h3>

啟用已停用的 plugin。對於 [從 claude.ai 同步的 plugin](/docs/zh-TW/plugins/loading#synced-plugins)，將 `<name>@synced` 作為 plugin 傳遞。

```bash theme={null}
claude plugin enable <plugin> [options]
```

| 旗標                    | 說明                                                                                                                |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>` | 啟用的範圍：`user`、`project` 或 `local`。省略時自動偵測                                                                          |
| `--json`              | 將結果列印為 stdout 最後一行的一個 JSON 物件，格式與 [`plugin install --json`](#plugin-json-result) 相同。需要 Claude Code v2.1.268 或更新版本 |

不使用 `--scope`，命令按本地、專案、使用者的順序檢查您的設定檔，並使用第一個提及 plugin 的範圍。

如果您傳遞 plugin 未宣告的 `--scope`，命令要麼寫入覆寫，要麼失敗：

* **[優先於](/docs/zh-TW/plugins/loading) 宣告範圍的範圍**：Claude Code 在您傳遞的範圍寫入覆寫。例如，`claude plugin disable formatter --scope local` 為您單獨關閉專案啟用的 plugin
* **任何其他範圍**：命令失敗，訊息為 `Plugin "formatter" is installed at project scope, not user. Use --scope project or omit --scope to auto-detect.`

如果 plugin 已在解析的範圍啟用，命令列印 `Plugin "formatter" is already enabled` 並結束 `1`。使用 `--json`，結果有 `"failureCode": "already_in_goal_state"` 和 `"alreadyInGoalState": true`，因此指令碼可以將該情況視為成功。

當 plugin 宣告 [dependencies](/docs/zh-TW/plugins/dependencies) 時，Claude Code 也啟用它們。命令在這些情況下失敗：

* **dependency 未安裝**：啟用失敗並列印每個遺漏 dependency 的 `claude plugin install` 命令
* **dependency 被您組織的 plugin 原則阻止**：啟用失敗並命名被阻止的 dependency
* **dependency 在優先於目標範圍的範圍設定為 `false`**：啟用失敗。在該範圍啟用 dependency，或傳遞 `--scope` 以在那裡寫入

在宣告它的任何地方重新啟用 plugin：

```bash theme={null}
claude plugin enable formatter
```

Claude Code 列印 `Successfully enabled plugin: formatter (scope: project)`，命名它偵測到的範圍。

<h3 id="plugin-disable">
  plugin disable
</h3>

停用 plugin 而不卸載它。對於 [從 claude.ai 同步的 plugin](/docs/zh-TW/plugins/loading#synced-plugins)，將 `<name>@synced` 作為 plugin 傳遞。

```bash theme={null}
claude plugin disable [plugin] [options]
```

| 旗標                    | 說明                                                                                                                |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------- |
| `-a, --all`           | 停用每個啟用的 plugin。無法與 plugin 名稱或 `--scope` 結合                                                                        |
| `-s, --scope <scope>` | 停用的範圍：`user`、`project` 或 `local`。省略時自動偵測                                                                          |
| `--json`              | 將結果列印為 stdout 最後一行的一個 JSON 物件，格式與 [`plugin install --json`](#plugin-json-result) 相同。需要 Claude Code v2.1.268 或更新版本 |

不使用 `--scope`，範圍以與 [`plugin enable`](#plugin-enable) 相同的本地、專案、使用者順序自動偵測。

如果您既不傳遞 plugin 名稱也不傳遞 `--all`，Claude Code 列印 `Please specify a plugin name or use --all to disable all plugins` 並結束 `1`。停用已停用的 plugin 列印 `Plugin "formatter" is already disabled` 並結束 `1`，如 [`plugin enable`](#plugin-enable) 對已啟用 plugin 所做的那樣。

命令對仍然需要的 plugin 失敗：

* **另一個啟用的 plugin [depends on](/docs/zh-TW/plugins/dependencies) 它**：命令失敗並命名要先停用的相依項
* **您的組織要求它作為同步 plugin**：命令失敗並保存任何內容

停用一個 plugin：

```bash theme={null}
claude plugin disable formatter
```

Claude Code 列印 `Successfully disabled plugin: formatter (scope: project)`。

<h3 id="plugin-update">
  plugin update
</h3>

將 plugin 更新到其市場提供的最新版本。新版本在您的下一個工作階段中載入，或在執行中的工作階段中執行 `/reload-plugins` 後載入。

```bash theme={null}
claude plugin update <plugin> [options]
```

| 旗標                          | 說明                                                                                                                                                             |
| :-------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>`       | 更新的範圍：`user`、`project`、`local` 或 `managed`。預設為 plugin 安裝的範圍                                                                                                    |
| `-y, --yes`                 | 接受來自 [command-source](/docs/zh-TW/plugins/host-marketplace) plugin 的已變更安裝命令，無需提示。當 stdin 或 stdout 不是 TTY 時需要，除非您傳遞 `--accept-command`。需要 Claude Code v2.1.229 或更新版本 |
| `--accept-command <sha256>` | 接受市場宣告的命令，其 `sha256` 先前的 [`--json` 執行](#plugin-json-result) 在 `shownCommand` 中報告，代替 `-y`。無法與 `-y` 結合。需要 Claude Code v2.1.271 或更新版本                             |
| `--json`                    | 將結果列印為 stdout 最後一行的一個 JSON 物件，格式與 [`plugin install --json`](#plugin-json-result) 相同。需要 Claude Code v2.1.268 或更新版本                                              |

`managed` 是您可以更新但不能安裝的唯一範圍。對於管理員安裝的 plugins，請參閱 [為您的組織管理 plugins](/docs/zh-TW/plugins/org)。

更新 plugin：

```bash theme={null}
claude plugin update formatter@my-marketplace
```

Claude Code 列印 `Checking for updates for plugin "formatter@my-marketplace"…`，然後是結果。當沒有更新時，它列印 `formatter is already at the latest version (1.0.0).` 並結束 `0`。

您可以傳遞裸 plugin 名稱，命令會根據您安裝的 plugins 進行比對。當來自不同市場的已安裝 plugins 共享名稱時，命令拒絕更新並列出要執行的限定 `plugin-name@marketplace-name` 命令。按裸名稱更新需要 Claude Code v2.1.246 或更新版本。

<h3 id="plugin-list">
  plugin list
</h3>

列出已安裝的 plugins，包括其版本、範圍和狀態。

```bash theme={null}
claude plugin list [options]
```

| 旗標            | 說明                                      |
| :------------ | :-------------------------------------- |
| `--json`      | 將列表列印為 JSON                             |
| `--available` | 也列出您的市場提供但您未安裝的 plugins。沒有 `--json` 時無效 |

Claude Code 按每個 plugin 的載入方式對人類可讀的輸出進行分組：

* **`Installed plugins:`**：您從市場安裝的 plugins
* **`Session-only plugins (--plugin-dir / --plugin-url):`**：由同一命令中的這些旗標載入的 plugins，如 `claude --plugin-dir ./my-plugin plugin list`
* **`Skills-directory plugins (.claude/skills/*):`**：Claude Code 在 skills 目錄中找到的 plugins
* **`Synced from claude.ai`**：[從您的 claude.ai 帳戶同步的 plugins](/docs/zh-TW/plugins/loading#synced-plugins)

當任何群組中都沒有任何內容時，Claude Code 列印 ``No plugins installed. Use `claude plugin install` to install a plugin.``

<h4 id="json-output">
  JSON 輸出
</h4>

使用 `--json`，Claude Code 列印一個陣列，每個安裝一個物件。每個物件帶有下面的欄位。`id`、`version`、`scope`、`enabled` 和 `installPath` 始終存在，其他欄位僅在適用時出現。

| 欄位             | 類型               | 說明                                                                                                                                                      |
| :------------- | :--------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `id`           | string           | 安裝為 `name@marketplace`，工作階段專用 plugins 為 `name@inline`，skills-directory plugins 為 `name@skills-dir`，從 claude.ai 同步的 plugins 為 `name@synced`              |
| `version`      | string           | 對於市場安裝，[Claude Code 在安裝時計算的](/docs/zh-TW/plugins/loading#versions-and-updates) 版本。對於工作階段專用、skills-directory 或同步 plugin，manifest 的 `version`，或未宣告時為 `unknown` |
| `scope`        | string           | 安裝為 `user`、`project`、`local` 或 `managed`；skills-directory plugins 為 `user` 或 `project`；工作階段專用 plugins 為 `session`；從 claude.ai 同步的 plugins 為 `synced`    |
| `enabled`      | boolean          | plugin 在您的合併設定中是否啟用                                                                                                                                     |
| `installPath`  | string           | plugin 載入的目錄                                                                                                                                            |
| `installedAt`  | string           | 安裝的 ISO 時間戳。僅市場安裝                                                                                                                                       |
| `lastUpdated`  | string           | 上次更新的 ISO 時間戳。僅市場安裝                                                                                                                                     |
| `projectPath`  | string           | 安裝所屬的專案。僅 `project` 和 `local` 範圍                                                                                                                        |
| `mcpServers`   | object           | plugin 的 MCP 伺服器定義，當市場安裝的 plugin 有任何時                                                                                                                   |
| `errors`       | array of strings | 載入錯誤，當 plugin 無法載入時                                                                                                                                     |
| `notes`        | array of strings | plugin 已載入並正常運作的編寫警告                                                                                                                                    |
| `errorDetails` | array of objects | 每個 `errors` 項目一個物件，給出其診斷 `type` 和它引用的名稱，例如 plugin、市場、伺服器或檔案。需要 Claude Code v2.1.268 或更新版本                                                               |
| `noteDetails`  | array of objects | 每個 `notes` 項目的相同詳細物件。需要 Claude Code v2.1.268 或更新版本                                                                                                      |

使用 `--json --available`，Claude Code 列印一個物件而不是陣列。其 `installed` 欄位保存已安裝 plugin 物件的陣列，其 `available` 欄位保存每個未安裝市場 plugin 的一個物件，欄位如下。

| 欄位                | 類型               | 說明                                                                 |
| :---------------- | :--------------- | :----------------------------------------------------------------- |
| `pluginId`        | string           | `name@marketplace`                                                 |
| `name`            | string           | plugin 在市場中的名稱                                                     |
| `marketplaceName` | string           | 提供它的市場                                                             |
| `source`          | string or object | 市場項目的 [source](/docs/zh-TW/plugins/marketplace-reference)：相對路徑為字串，否則為物件 |
| `description`     | string           | 項目的說明，當它有時                                                         |
| `version`         | string           | 項目的版本，當它宣告時                                                        |
| `installCount`    | number           | 安裝計數，當 Claude Code 有 plugin 的計數時                                   |

<h3 id="plugin-details">
  plugin details
</h3>

顯示 plugin 的元件清單及其預計的 token 成本。

plugin 必須已載入：已安裝、在 skills 目錄中找到，或在同一命令中使用 `--plugin-dir` 或 `--plugin-url` 傳遞。`<name>` 是 plugin `name` 或 `name@marketplace`。

```bash theme={null}
claude plugin details <name>
```

命令除了 `--help` 外不接受任何旗標。

顯示已安裝 plugin 的貢獻：

```bash theme={null}
claude plugin details formatter
```

Claude Code 列印 plugin 的名稱、版本、說明和來源，然後是這些部分：

* **`Component inventory`**：plugin 的 skills、agents、hooks、MCP 伺服器和 LSP 伺服器
* **`Projected token cost`**：plugin 添加到每個工作階段的始終開啟 tokens
* **`Per-component (rounded)`**：每個 skill、agent 和命令的始終開啟和按調用估計。當 plugin 沒有時省略

對於兩個成本數字的含義，請參閱 [測量 plugin 成本和使用](/docs/zh-TW/plugins/measure)。

對於未載入的 plugin，Claude Code 列印 ``Plugin "formatter" not found. Run `claude plugin list` to see installed plugins, or pass --plugin-dir <path> to load one from disk.`` 並結束 `1`。

<h3 id="plugin-prune">
  plugin prune
</h3>

移除自動安裝的 [dependencies](/docs/zh-TW/plugins/dependencies)，沒有已安裝的 plugin 再需要。命令永遠不會移除您自己安裝的 plugin。`autoremove` 是 `prune` 的別名。

```bash theme={null}
claude plugin prune [options]
```

| 旗標                    | 說明                                          |
| :-------------------- | :------------------------------------------ |
| `-s, --scope <scope>` | 修剪的範圍：`user`、`project` 或 `local`。預設為 `user` |
| `--dry-run`           | 列出將移除的內容而不移除它                               |
| `-y, --yes`           | 跳過確認提示。當 stdin 或 stdout 不是 TTY 時需要          |

預覽修剪將移除的內容：

```bash theme={null}
claude plugin prune --dry-run
```

Claude Code 列出孤立的 dependencies 並以 `(dry run — nothing removed)` 結尾。沒有要移除的內容時，它列印以 `Nothing to prune` 開頭的行。

不使用 `--dry-run`，命令僅在您在提示處確認或傳遞 `-y` 後移除孤立的 dependencies。

無論您在提示處的答案如何，結束代碼都是 `0`。

`prune` 的作用取決於是否附加了終端以及您是否傳遞了 `-y`：

| 終端和旗標                       | 發生的情況                                                                 |
| :-------------------------- | :-------------------------------------------------------------------- |
| 互動式終端，無 `-y`                | 列出孤立的 dependencies 並詢問 `Remove? [y/N]`                                |
| 任何終端，`-y`                   | 移除它們並列印 `Removed N auto-installed plugins: <names>`                   |
| 非 TTY stdin 或 stdout，無 `-y` | 列印列表並 ``Not a TTY — run `claude plugin prune -y` to remove.``，不移除任何內容 |

<h3 id="plugin-eval">
  plugin eval
</h3>

執行 plugin 的 [eval cases](/docs/zh-TW/plugin-evals) 並報告評分結果。需要 Claude Code v2.1.269 或更新版本。

每個案例是一個提示加上評分者。Claude Code 在隔離的工作階段中執行它多次，僅載入目標 plugin，預設情況下也不載入 plugin，以便報告顯示差異。

請參閱 [使用 evals 測試 plugins](/docs/zh-TW/plugin-evals) 以了解案例格式、評分者、結果和 CI 使用。

```bash theme={null}
claude plugin eval [target] [options]
```

可選的 `target` 預設為目前目錄，並採用以下任何形式：

* plugin 目錄
* 單個 `prompt.md` 或 `case.yaml` 檔案
* 已安裝的 plugin，如 `name` 或 `name@marketplace`
* `name@skills-dir`

將目標放在 `--tag`、`--allow-tools` 和 `--json` 之前。這些選項中的每一個都將其後面的單詞作為其值，因此在其中一個之後寫入的目標被讀作標籤、工具名稱或 JSON 輸出路徑，而不是目標。

此表列出大多數執行使用的選項。執行 `claude plugin eval --help` 以獲得完整集合，包括 `--case`、`--tag`、`--output-dir`、`--report`、`--allow-real-servers`、`--keep-temp` 和 `--verbose`。

| 選項                         | 說明                                                                                                                      | 預設                                                           |
| :------------------------- | :---------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------- |
| `--runs <n>`               | 每個 [arm](/docs/zh-TW/plugin-evals#compare-against-a-no-plugin-baseline) 中每個案例的執行                                             | 每個案例的 `runs`，否則 3                                            |
| `-j, --concurrency <n>`    | 同時執行的代理工作階段，1 到 8。它們共享您的速率限制                                                                                            | `1`                                                          |
| `--model <model>`          | 測試中的代理的模型                                                                                                               | 每個案例的 `model`，否則 `ANTHROPIC_MODEL`（如果設定），否則 Claude Code 的預設值 |
| `--judge-model <model>`    | `llm` 和 `baseline` 評分者的模型                                                                                               | 一個小的快速模型                                                     |
| `--ablation <mode>`        | `none` 或 `with-without`。請參閱 [與無 plugin 基線比較](/docs/zh-TW/plugin-evals#compare-against-a-no-plugin-baseline)                  | 當 plugin 解析時為 `with-without`，否則為 `none`                      |
| `--threshold <0..1>`       | 如果任何案例評分低於此，結束 1                                                                                                        | `1.0`                                                        |
| `--max-cost-usd <usd>`     | 一旦支出達到此值，停止下一次執行，結束 2，並報告部分結果                                                                                           | 無限制                                                          |
| `--allow-tools <tools...>` | 授予超出唯讀集合的工具，例如 `Bash`、`Write`、`Edit` 或 `"mcp__plugin_<plugin>_<server>__*"`。請參閱 [授予工具](/docs/zh-TW/plugin-evals#grant-tools) |                                                              |
| `--scaffold`               | 執行每個案例的 [`scaffold_script`](/docs/zh-TW/plugin-evals#add-setup-or-history-with-case-yaml)                                    | 關閉                                                           |
| `--trust-plugin`           | 跳過首次執行信任提示，用於 CI。請參閱 [執行可以存取的內容](/docs/zh-TW/plugin-evals#security)                                                          | 關閉                                                           |
| `--mocks <mode>`           | `record` 或 `off`。請參閱 [模擬 MCP 伺服器](/docs/zh-TW/plugin-evals#mock-mcp-servers)                                                 | `record`                                                     |
| `--eval-dir <dir>`         | plugin 下方保存案例的目錄                                                                                                        | manifest 的 `experimental.evals`，否則 `evals`                   |
| `--json [path]`            | 將 [結果文件](/docs/zh-TW/plugin-evals#json-result) 列印到 stdout，或寫入 `.json` 路徑                                                     |                                                              |
| `--no-publish`             | 保持 HTML 報告本地                                                                                                            |                                                              |

結束代碼報告執行如何結束。若要在管道中對其進行操作，請參閱 [在 CI 中執行 evals](/docs/zh-TW/plugin-evals#run-evals-in-ci)。

| 結束代碼  | 含義                         |
| :---- | :------------------------- |
| `0`   | 每個案例都符合閾值                  |
| `1`   | 失敗的案例、載入錯誤或不受信任的 plugin 目錄 |
| `2`   | 部分執行                       |
| `130` | 已中斷                        |
| `143` | 已終止                        |

<h3 id="plugin-eval-init">
  plugin eval init
</h3>

為目前目錄中的 plugin 建立 eval 套件。需要 Claude Code v2.1.269 或更新版本。請參閱 [建立您的第一個 eval 套件](/docs/zh-TW/plugin-evals#create-your-first-eval-suite)。

```bash theme={null}
claude plugin eval init [name] [options]
```

在終端中，命令開啟互動式 Claude Code 工作階段以進行編寫訪談。在訪談中，Claude 執行以下操作：

1. 讀取 plugin
2. 詢問您它應該做什麼
3. 提議案例和評分者
4. 寫入案例檔案
5. 執行案例並與您檢查評分，以確認評分者按您的方式評分

使用 `--bare` 或沒有終端，命令改為寫入空白單案例範本。當 Claude 從 Claude Code 工作階段內執行命令時，命令列印該工作階段要遵循的訪談說明，而不是寫入範本。

可選的 `name` 是案例名稱。它在 `--bare` 或沒有終端時需要，因為命令為該案例寫入空白範本。訪談不需要。

命令接受這些選項：

| 選項                  | 說明                                                          | 預設                                         |
| :------------------ | :---------------------------------------------------------- | :----------------------------------------- |
| `--bare`            | 為 `<name>` 寫入空白 `prompt.md` 和 `graders/criteria.md`，而不是執行訪談 |                                            |
| `-i, --interactive` | 需要訪談。沒有終端時失敗，而不是寫入範本                                        |                                            |
| `--eval-dir <dir>`  | 目前目錄下方寫入案例的目錄                                               | manifest 的 `experimental.evals`，否則 `evals` |

<h3 id="plugin-tag">
  plugin tag
</h3>

為 plugin 發佈建立名為 `<name>--v<version>` 的帶註解 git 標籤。標籤前，命令檢查 plugin 的 `plugin.json` 和任何列出它的市場項目是否同意版本。

有關何時標籤發佈，請參閱 [發佈 plugin](/docs/zh-TW/plugins/publish)。

```bash theme={null}
claude plugin tag [path] [options]
```

`[path]` 是 plugin 目錄，預設為目前目錄。命令透過從該目錄向上走到列出 plugin 的 `.claude-plugin/marketplace.json` 來找到市場項目。

| 旗標                    | 說明                                      |
| :-------------------- | :-------------------------------------- |
| `--push`              | 建立後將標籤推送到 `--remote`                    |
| `--dry-run`           | 列印將標籤化的內容而不建立標籤                         |
| `-f, --force`         | 跳過髒工作樹和標籤已存在檢查                          |
| `-m, --message <msg>` | 標籤註解訊息。`%s` 代表版本。預設為 `<name> <version>` |
| `--remote <name>`     | 使用 `--push` 推送到的遠端。預設為 `origin`         |

預覽市場簽出中 plugin 的標籤：

```bash theme={null}
claude plugin tag plugins/formatter --dry-run
```

Claude Code 列印計畫：

* plugin 名稱
* 版本及其來自的檔案
* 匹配的市場項目，當有時
* 標籤名稱
* 它將執行的 `git tag` 和 `git push` 命令

不使用 `--dry-run`，Claude Code 列印 `Created tag formatter--v1.0.0` 並列印 `Pushed to origin` 或您自己執行的推送命令。如果推送失敗，標籤仍在本地建立，命令以錯誤結束。

當命令無法安全地標籤時，它結束 `1` 並列印原因。常見原因是：

* `plugin.json` 或市場項目中沒有 `version`
* 標籤已存在
* 工作樹是髒的

<h3 id="plugin-validate">
  plugin validate
</h3>

驗證 plugin manifest、市場 manifest 或目錄中的 skills、agents 和命令，並以 CI 工作可以對其進行操作的代碼結束。對於建立、測試和編輯工作流程，請參閱 [建立 plugin](/docs/zh-TW/plugins/create)。對於驗證器在每個 manifest 中檢查的內容，請參閱 [plugin manifest 參考](/docs/zh-TW/plugins/manifest-reference) 和 [市場參考](/docs/zh-TW/plugins/marketplace-reference)。

```bash theme={null}
claude plugin validate <path> [options]
```

| 旗標         | 說明                                                             |
| :--------- | :------------------------------------------------------------- |
| `--strict` | 將警告視為錯誤，因此執行時容許的未識別欄位和遺漏中繼資料失敗執行。需要 Claude Code v2.1.145 或更新版本 |
| `--json`   | 將驗證報告輸出為具有相同結束代碼的一個 JSON 物件。需要 Claude Code v2.1.259 或更新版本      |

在提交前驗證 plugin：

```bash theme={null}
claude plugin validate ./my-plugin --strict
```

<h4 id="validate-a-directory">
  驗證目錄
</h4>

`<path>` 是 manifest 檔案或目錄。給定目錄，Claude Code 透過在其中找到的內容選擇要驗證的內容：

* `.claude-plugin/marketplace.json`，當它存在時
* 否則 `.claude-plugin/plugin.json`
* 否則元件檔案，由目錄的名稱選擇。驗證沒有 manifest 的元件檔案需要 Claude Code v2.1.233 或更新版本：
  * 名為 `skills`、`agents` 或 `commands` 的目錄：其中的檔案
  * 名為 `.claude` 的目錄：其中的 `skills`、`agents` 和 `commands` 目錄
  * 任何其他目錄：其 `.claude` 下的這三個目錄

Claude Code 不遵循您命名的目錄內的符號連結。它的作用取決於連結的位置：

* **plugin 或 `.claude` 根下的連結 `skills`、`agents` 或 `commands` 目錄**：Claude Code 警告其中的任何內容都未被讀取。
* **`skills`、`agents` 或 `commands` 目錄內的連結項目**：Claude Code 跳過它並警告，每個目錄，它跳過了多少項目，工作階段會載入。
* **您命名的 `skills`、`agents` 或 `commands` 目錄本身是符號連結，或其父 `.claude` 目錄是**：Claude Code 報告錯誤並檢查其中的任何內容。改為命名真實目錄。

驗證執行不讀取幾個檔案：

* **plugin 根處的 `SKILL.md`**：當您針對 plugin 目錄執行 `claude plugin validate` 時，Claude Code 不檢查 plugin 根處的 `SKILL.md`
* **plugin 根處的 `CLAUDE.md`**：在 plugin 執行中，Claude Code 也警告 plugin 根處的 `CLAUDE.md`
* **市場執行中的 Plugin 檔案**：從市場目錄，Claude Code 不開啟 plugins 的 skill、agent、command 或 hook 檔案。若要在這些檔案中找到錯誤，驗證每個 plugin 目錄

<h4 id="output-and-exit-codes">
  輸出和結束代碼
</h4>

Claude Code 列印它驗證的檔案、任何帶有其路徑的錯誤和警告，以及判決行。結束代碼遵循判決：

| 結束代碼 | 判決行                                                                            | 含義                              |
| :--- | :----------------------------------------------------------------------------- | :------------------------------ |
| `0`  | `Validation passed` 或 `Validation passed with warnings`                        | manifest 載入。使用 `--strict`，也沒有警告 |
| `1`  | `Validation failed` 或 `Validation failed (--strict treats warnings as errors)` | 錯誤，或 `--strict` 下的警告            |
| `2`  | `Unexpected error during validation: <reason>`                                 | 驗證器本身失敗，例如在不可讀的路徑上              |

使用 `--json`，Claude Code 將報告寫入 stdout 作為具有這些頂級欄位的一個 JSON 物件：

* `success`：結束代碼給出的相同判決
* `strict`：執行是否將警告視為錯誤
* `target`：Claude Code 驗證的解析路徑
* `manifest`：manifest 自己的結果，或沒有 manifest 的執行為 `null`
* `contents`：每個檔案的結果，命名其 `file` 並帶有 `errors`、`warnings` 和 `notes` 陣列

在結束 `2` 時，命令不向 stdout 寫入任何內容。錯誤訊息進入 stderr。

<h2 id="claude-plugin-marketplace-commands">
  claude plugin marketplace 命令
</h2>

從 shell 執行 `claude plugin marketplace <subcommand>` 以新增、列出、重新整理和移除您安裝 plugins 的市場。

* **結束代碼**：這些子命令遵循 plugin 命令的 [exit-code convention](#claude-plugin-commands)
* **範圍**：它們的 `--scope` 旗標沒有 `-s` 短形式

有關市場是什麼以及 Claude Code 如何快取它，請參閱 [Plugin 載入參考](/docs/zh-TW/plugins/loading)。

<h3 id="plugin-marketplace-add">
  plugin marketplace add
</h3>

從 GitHub 儲存庫、git URL、託管 `marketplace.json` 或本地路徑新增市場，並在設定檔中宣告它。

新增後，Claude Code 安裝您已安裝 plugins 遺漏的任何 [dependencies](/docs/zh-TW/plugins/dependencies)。

```bash theme={null}
claude plugin marketplace add <source> [options]
```

| 旗標                    | 說明                                                                                                          |
| :-------------------- | :---------------------------------------------------------------------------------------------------------- |
| `--scope <scope>`     | 在其中宣告市場的設定檔：`user`、`project` 或 `local`。預設為 `user`                                                           |
| `--sparse <paths...>` | 將 git 簽出限制為這些目錄，用於 monorepos。僅 `github` 和 `git` 來源                                                          |
| `--claudeai`          | 將引數讀作 [claude.ai 上託管的市場](/docs/zh-TW/plugins/install#add-from-claude-ai) 的名稱，而不是來源。需要 Claude Code v2.1.273 或更新版本 |

`<source>` 採用下表中的任何形式，其形式決定來源類型以及 Claude Code 如何擷取市場。對於結果來源物件，請參閱 [市場參考](/docs/zh-TW/plugins/marketplace-reference)。

| 您輸入                                                                      | 來源類型        | Claude Code 如何擷取它                                   |
| :----------------------------------------------------------------------- | :---------- | :-------------------------------------------------- |
| `owner/repo`、`owner/repo#ref` 或 `owner/repo@ref`                         | `github`    | 複製 GitHub 儲存庫，給定時固定到 `ref`。所有者和儲存庫必須遵循 GitHub 命名規則  |
| `user@host:path[.git][#ref]`                                             | `git`       | 透過 SSH 複製                                           |
| `https://example.com/repo.git[#ref]` 或包含 `/_git/` 的 URL                  | `git`       | 透過 HTTPS 複製，包括 Azure DevOps URL                     |
| `https://github.com/owner/repo` 或 `https://gitlab.com/namespace/project` | `git`       | 在附加 `.git` 後透過 HTTPS 複製                             |
| 任何其他 `http://` 或 `https://` URL，包括沒有 `.git` 的自託管 git 主機                  | `url`       | 將 URL 作為 `marketplace.json` 擷取。若要改為複製儲存庫，請附加 `.git` |
| `./path`、`../path`、`/path` 或 `~/path` 到目錄                                | `directory` | 就地讀取目錄。在 Windows 上，`.\`、`..\` 和 `C:\` 形式也有效         |
| 相同的路徑形式，到 `.json` 檔案                                                     | `file`      | 就地讀取檔案                                              |

對於其複製 URL 不帶 `.git` 尾碼的主機（例如 AWS CodeCommit），改為在 [`extraKnownMarketplaces`](/docs/zh-TW/settings-reference#extraknownmarketplaces) 中將市場新增為 git 項目。Claude Code 複製 git 項目，無論其 URL 是否以 `.git` 結尾。

Claude Code 也複製具有嵌套子群組的 `gitlab.com` URL，例如 `https://gitlab.com/group/subgroup/project`。

新增市場並與專案共享：

```bash theme={null}
claude plugin marketplace add your-org/your-marketplace --scope project
```

Claude Code 列印 `Successfully added marketplace: your-marketplace (declared in project settings)`，使用市場自己的 manifest 中的 `name`。重複新增或無效來源列印以下結果之一：

* **市場已在磁碟上**：輸出為 `Marketplace 'your-marketplace' already on disk — declared in project settings`，結束代碼為 `0`
* **無法識別的來源**：輸出為 `Invalid marketplace source format. Try: owner/repo, https://..., or ./path`，結束代碼為 `1`
* **裸主機，例如 `gitlab.example.com/team/plugins`**：新增失敗，因為無效的 `owner/repo` 速記，訊息告訴您新增 `https://` 或使用本地路徑

按 `claude plugin marketplace list` 的 `From claude.ai:` 部分中列印的名稱新增 [claude.ai 上託管的市場](/docs/zh-TW/plugins/install#add-from-claude-ai)：

```bash theme={null}
claude plugin marketplace add --claudeai claudeai-organization-library
```

使用 `--claudeai`，命令拒絕 `--scope` 和 `--sparse`。市場為您的帳戶託管，未在設定檔中宣告，因此您無法透過專案的 `.claude/settings.json` 共享它。

<h3 id="plugin-marketplace-list">
  plugin marketplace list
</h3>

列出您新增的每個市場及其來源。

```bash theme={null}
claude plugin marketplace list [options]
```

| 旗標       | 說明          |
| :------- | :---------- |
| `--json` | 將列表列印為 JSON |

Claude Code 列印 `Configured marketplaces:` 和每個市場一個 `Source:` 行，或 `No marketplaces configured`。

使用 `--json`，Claude Code 列印一個陣列，每個市場一個物件，帶有下面的欄位。每個欄位都是字串。

| 欄位                | 說明                                                   |
| :---------------- | :--------------------------------------------------- |
| `name`            | 市場的名稱                                                |
| `source`          | `github`、`git`、`url`、`directory`、`file` 或 `claudeai` |
| `repo`            | `owner/repo`。僅 `github` 來源                           |
| `url`             | 複製或擷取 URL。僅 `git` 和 `url` 來源                         |
| `path`            | 本地路徑。僅 `directory` 和 `file` 來源                       |
| `ref`             | 固定的分支或標籤。`github` 和 `git` 來源，僅當固定時                   |
| `installLocation` | Claude Code 快取市場的位置                                  |

已新增的 [claude.ai 市場](/docs/zh-TW/plugins/install#add-from-claude-ai) 沒有本地複製，因此其項目帶有其 claude.ai 識別碼 `marketplaceId` 和 `organizationUuid`，代替 `installLocation`。它也帶有 `scope`（當記錄時）和 `status`。

如果您的終端工作階段 [從您的 claude.ai 帳戶同步 plugins](/docs/zh-TW/plugins/loading#synced-plugins)，文字列表以 `From claude.ai:` 部分結尾。該部分命名 claude.ai 為您的帳戶列出的市場，您未新增的市場，包括基於 git 的和託管的。它需要 Claude Code v2.1.273 或更新版本。

若要從該部分新增市場，請參閱 [從 claude.ai 新增市場](/docs/zh-TW/plugins/install#add-from-claude-ai)。

`--json` 輸出僅涵蓋已配置的市場，並將部分留出。

<h3 id="plugin-marketplace-remove">
  plugin marketplace remove
</h3>

從您的設定中移除市場的宣告。`rm` 是 `remove` 的別名。

<Warning>
  當您從最後一個宣告它的範圍移除市場時，Claude Code 也刪除其快取並卸載您從它安裝的每個 plugin。不使用 `--scope`，命令從每個範圍移除宣告。若要重新整理市場而不失去其 plugins，改為執行 `plugin marketplace update`。
</Warning>

```bash theme={null}
claude plugin marketplace remove <name> [options]
```

`<name>` 是 `plugin marketplace list` 顯示的市場名稱，而不是您傳遞給 `add` 的來源。

| 旗標                | 說明                                                                |
| :---------------- | :---------------------------------------------------------------- |
| `--scope <scope>` | 從一個設定範圍移除宣告：`user`、`project` 或 `local`。不使用它，Claude Code 從每個範圍移除宣告 |

從每個範圍移除市場：

```bash theme={null}
claude plugin marketplace remove your-marketplace
```

Claude Code 列印 `Successfully removed marketplace: your-marketplace`，當您限定範圍時新增 `(from project settings)`。如果您限定範圍到不宣告市場的設定檔，命令失敗，訊息為 `Marketplace 'your-marketplace' is not declared in project settings. Omit --scope to remove it from all scopes.`

<h3 id="plugin-marketplace-update">
  plugin marketplace update
</h3>

重新整理一個市場或每個市場，從其來源擷取新 plugins 和版本。使用分支或標籤 `ref` 新增的市場更新到該 ref 的最新提交，而不是儲存庫的預設分支。

```bash theme={null}
claude plugin marketplace update [name]
```

命令除了 `--help` 外不接受任何旗標。

重新整理一個市場：

```bash theme={null}
claude plugin marketplace update your-marketplace
```

Claude Code 列印 `Successfully updated marketplace: your-marketplace`。當您省略名稱時，它列印計數，例如 `Successfully updated 2 marketplaces`。沒有新增市場時，它列印 `No marketplaces configured` 並結束 `0`。

<h2 id="plugin-in-a-session">
  /plugin 在工作階段中
</h2>

在互動式工作階段內，`/plugin` 開啟 plugin 面板。每個子命令在標籤上開啟面板、在那裡執行操作或內聯列印結果。`/plugins` 和 `/marketplace` 是 `/plugin` 的別名。

您只能在互動式終端工作階段中執行這些命令。在非互動式執行（例如 `claude -p`）中，Claude Code 回覆 `/plugin` 在此環境中不可用。

有關哪些表面有 `/plugin`、如何在沒有它的情況下安裝以及每個面板標籤顯示的內容，請參閱 [安裝和管理 plugins](/docs/zh-TW/plugins/install)。

`<plugin>` 是 plugin `name` 或 `name@marketplace`。

下表列出每個工作階段形式。shell 子命令 `init`、`update`、`details`、`prune`、`eval` 和 `eval init` 沒有工作階段形式。

| 命令                                                  | 別名                                           | 它的作用                                                                                                                                                                       |
| :-------------------------------------------------- | :------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/plugin`                                           |                                              | 在 **Discover** 標籤上開啟面板。`/plugin` 後的任何無法識別的第一個單詞執行相同操作                                                                                                                      |
| `/plugin help`                                      | `/plugin --help`、`/plugin -h`                | 顯示 `/plugin` 子命令的使用列表                                                                                                                                                      |
| `/plugin list [--enabled\|--disabled]`              | `ls`                                         | 內聯列印您的市場安裝 plugins，包括版本、範圍和狀態。篩選旗標僅顯示該狀態。啟用狀態尚未應用的 plugin 標記為 `— run /reload-plugins to apply`。需要 Claude Code v2.1.163 或更新版本                                               |
| `/plugin install`                                   | `i`                                          | 開啟 **Discover** 標籤                                                                                                                                                         |
| `/plugin install <plugin>`                          | `i`                                          | 在 **Discover** 標籤中開啟 plugin 的詳細資訊。使用 `name@marketplace`，在該市場的列表中開啟它們                                                                                                       |
| `/plugin install <plugin> --marketplace <source>`   | `i`                                          | 當您尚未新增市場時在 `<source>` 新增市場，要求您先確認，然後開啟 plugin 的詳細資訊。請參閱 [在一個命令中新增市場和安裝](/docs/zh-TW/plugins/install#add-a-marketplace-and-install-in-one-command)。需要 Claude Code v2.1.275 或更新版本 |
| `/plugin manage`                                    |                                              | 開啟 **Installed** 標籤                                                                                                                                                        |
| `/plugin stats`                                     |                                              | 在 [`/skill-doctor`](/docs/zh-TW/skills#find-unused-skills) 可用的工作階段中開啟 **Stats** 標籤。在其他任何地方，它在 **Discover** 標籤上開啟面板                                                              |
| `/plugin enable <plugin>`                           |                                              | 在 plugin 處開啟 **Installed** 標籤並啟用它                                                                                                                                          |
| `/plugin disable <plugin>`                          |                                              | 在 plugin 處開啟 **Installed** 標籤並停用它                                                                                                                                          |
| `/plugin uninstall <plugin>`                        |                                              | 在 plugin 處開啟 **Installed** 標籤並卸載它                                                                                                                                          |
| `/plugin configure <plugin>`                        | `config`                                     | 開啟 plugin 的 [`userConfig`](/docs/zh-TW/plugins/manifest-reference) 對話框，或報告 plugin 未宣告任何。需要 Claude Code v2.1.147 或更新版本                                                           |
| `/plugin validate <path>`                           |                                              | 列印與 `claude plugin validate` 相同的報告，內聯                                                                                                                                      |
| `/plugin tag [path] [--push] [--dry-run] [--force]` |                                              | 建立發佈標籤，如 `claude plugin tag` 所做的那樣。接受 `--push`、`--dry-run` 和 `--force` 或 `-f`；使用任何其他旗標或額外引數，Claude Code 改為列印使用                                                             |
| `/plugin marketplace`                               | `market`                                     | 不執行任何可見操作。傳遞 `add`、`list`、`update` 或 `remove`                                                                                                                              |
| `/plugin marketplace add [source]`                  | `market add`                                 | 使用來源，新增它並報告結果。沒有來源，開啟 **Add marketplace** 輸入                                                                                                                               |
| `/plugin marketplace list`                          | `market list`                                | 內聯列印您的市場名稱                                                                                                                                                                 |
| `/plugin marketplace update [name]`                 | `market update`                              | 開啟 **Marketplaces** 標籤。使用名稱，在那裡重新整理該市場                                                                                                                                     |
| `/plugin marketplace remove [name]`                 | `market remove`、`market rm`、`marketplace rm` | 開啟 **Marketplaces** 標籤。使用名稱，在那裡移除該市場                                                                                                                                       |

如果您在 `/plugin enable`、`disable`、`uninstall` 或 `configure` 中命名目前專案中未安裝的 plugin，Claude Code 列印 `Plugin "<plugin>" is not installed in this project` 而不是執行操作。

<h2 id="reload-plugins">
  /reload-plugins
</h2>

在不重新啟動工作階段的情況下，將待處理的外掛程式變更套用到執行中的工作階段。待處理的變更是指自工作階段啟動以來，您在磁碟上安裝、更新、啟用、停用或編輯的外掛程式。

當您關閉 `/plugin` 面板且在其中進行了待處理的變更時，Claude Code 會為您執行 `/reload-plugins`。在外掛程式面板外發生的變更（例如您在另一個終端機中執行的 `claude plugin` 命令）之後，請自行執行此命令。

```text theme={null}
/reload-plugins [--force]
```

| 旗標        | 說明                                         |
| :-------- | :----------------------------------------- |
| `--force` | 即使重新載入會使提示快取失效，也要套用重新載入。不加破折號的 `force` 也可以 |

<h3 id="reload-summary">
  重新載入摘要
</h3>

Claude Code 重新載入每個作用中的外掛程式，並列印一行摘要 `Reloaded: N plugins · N skills · N agents · N hooks · N plugin MCP servers · N plugin LSP servers`，在沒有互動式終端機的工作階段中省略外掛程式 MCP 伺服器計數。當任何外掛程式失敗時，摘要會新增 `N errors during load. Run /plugin for details.`

技能計數涵蓋外掛程式提供的每項技能，包括其 `commands/` 項目和其 SKILL.md 技能。代理計數是在工作階段中載入的代理數量，包括不來自外掛程式的代理。

當重新載入的外掛程式的[相依性](/docs/zh-TW/plugins/dependencies)遺失時，Claude Code 會安裝它們、重新載入，並在摘要中附加 `(+ N dependencies: <names>) resolved`。

<h3 id="reloads-that-change-mcp-tools">
  變更 MCP 工具的重新載入
</h3>

當重新載入會新增或移除外掛程式 MCP 伺服器或 `LSP` 工具，且該變更會使[提示快取](/docs/zh-TW/prompt-caching#enabling-or-disabling-a-plugin)失效時，Claude Code 不會套用重新載入。它會列印類似 `This reload changes MCP tools (<server>) — your next message will re-read the whole conversation instead of using the cache. Run /reload-plugins --force to apply.` 的一行。傳遞 `--force` 以無論如何套用它。

<h3 id="sessions-without-an-interactive-terminal">
  沒有互動式終端機的工作階段
</h3>

`/reload-plugins` 也在沒有互動式終端機的工作階段中執行，例如桌面應用程式、Agent SDK 和[非互動模式](/docs/zh-TW/headless)搭配 `-p`。需要 Claude Code v2.1.260 或更新版本。

在這些工作階段中，命令只在您自行將其輸入到工作階段時執行，例如在 `-p` 提示或桌面應用程式的提示方塊中。當它以其他方式到達時，例如透過[遠端控制](/docs/zh-TW/remote-control)或從 Slack 轉送的訊息，命令會回覆 `/reload-plugins isn't available over a remote connection in this session.` 並且不重新載入任何內容。

這些工作階段中的重新載入不會連接或斷開外掛程式 MCP 伺服器。這些變更會在您的下一個工作階段中生效。

<h2 id="flags-that-load-a-plugin-for-one-session">
  為單一工作階段載入 plugin 的旗標
</h2>

兩個 `claude` 旗標為單一工作階段載入 plugin，無需安裝它。兩者都可重複。

Plugin 作者使用它們在發佈前測試 plugin。對於載入-編輯-重新載入工作流程，請參閱 [開發而不使用市場](/docs/zh-TW/plugins/create#develop-without-a-marketplace)。

| 旗標                    | 說明                                                                                       | 範例                                                                          |
| :-------------------- | :--------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------- |
| `--plugin-dir <path>` | 從目錄或其 `.zip` 存檔載入 plugin。plugins 資料夾載入每個保存 `.claude-plugin/plugin.json` 的子資料夾。每個旗標採用一個路徑 | `claude --plugin-dir ./my-plugin --plugin-dir ./other.zip`                  |
| `--plugin-url <url>`  | 從 URL 擷取 plugin `.zip` 存檔。重複旗標，或在一個引用值中傳遞多個 URL 空格分隔                                     | `claude --plugin-url "https://example.com/a.zip https://example.com/b.zip"` |

任一旗標載入的 plugin 是工作階段專用 plugin。`claude plugin list` 將其顯示為 `<name>@inline`，範圍為 `session`，但僅當相同旗標在子命令前時。例如，執行 `claude --plugin-dir ./my-plugin plugin list`。

當工作階段專用 plugin 與已安裝的 plugin 共享名稱時，Claude Code 為該工作階段載入工作階段專用複製並跳過已安裝的複製。如果您使用 `claude plugin disable <name>@inline` 停用工作階段專用複製，或受管設定鎖定該 plugin 名稱，已安裝的複製改為載入。對於優先順序，請參閱 [Plugin 載入參考](/docs/zh-TW/plugins/loading)。

管理員可以拒絕兩個旗標和 [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/zh-TW/env-vars#variables) 變數中命名的資料夾，使用受管 [`disableSideloadFlags`](/docs/zh-TW/settings-reference#disablesideloadflags) 設定。Claude Code 然後列印旗標被您組織的受管設定停用，並結束 `1` 而不啟動。

從 Agent SDK，[`plugins`](/docs/zh-TW/agent-sdk/plugins) 選項等同於 `--plugin-dir`。

<h2 id="next-steps">
  後續步驟
</h2>

* [安裝和管理 plugins](/docs/zh-TW/plugins/install)：與步驟相同的操作，以及您在每一個看到的內容
* [Plugin 載入參考](/docs/zh-TW/plugins/loading)：每個命令在磁碟上變更的內容以及哪個範圍生效
* [Troubleshoot plugins](/docs/zh-TW/plugins/troubleshooting)：安裝、市場、載入和驗證錯誤訊息及其修復
* [Plugin manifest 參考](/docs/zh-TW/plugins/manifest-reference)：`claude plugin validate` 檢查的欄位
