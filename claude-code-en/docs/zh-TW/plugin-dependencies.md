> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 限制 plugin 依賴版本

> 在 plugin 依賴上聲明版本約束，並將精選 plugin 集合捆綁在一個安裝後面。

一個 plugin 可以通過在 `plugin.json` 或其 marketplace 條目中列出其他 plugin 來依賴它們。預設情況下，依賴會追蹤最新可用版本，因此上游版本發佈可能會在沒有警告的情況下更改您 plugin 的依賴。版本約束讓您可以將依賴保持在經過測試的版本範圍內，直到您選擇升級。

當您安裝聲明依賴的 plugin 時，Claude Code 會自動解析並安裝它們，並在安裝輸出的末尾列出已添加的依賴。如果依賴後來遺失，`/reload-plugins` 和背景 plugin 自動更新會重新安裝它，前提是其 marketplace 已在您設定的 marketplace 中。重新執行 `claude plugin install` 在依賴的 plugin 上，或使用 `claude plugin marketplace add` 新增 marketplace，也會解析任何未解決的遺失依賴。來自您尚未新增的 marketplace 的依賴將保持未解決狀態。

本指南適用於在 `plugin.json` 中聲明依賴的 plugin 作者，以及標記版本發佈的 marketplace 維護者。若要安裝具有依賴的 plugin，請參閱[發現並安裝 plugin](/docs/zh-TW/discover-plugins)。如需完整的 manifest 架構，請參閱 [Plugins 參考](/docs/zh-TW/plugins-reference)。

<h2 id="why-constrain-dependency-versions">
  為什麼要限制依賴版本
</h2>

考慮一個內部 marketplace，其中兩個團隊發佈 plugin。平台團隊維護 `secrets-vault`，這是一個包裝 secrets 後端的 MCP 伺服器。部署團隊維護 `deploy-kit`，它在部署期間調用 `secrets-vault` 來獲取認證。

`deploy-kit` 已針對 `secrets-vault` v2.1.0 進行測試。沒有版本約束的情況下，下次平台團隊標記重新命名 MCP 工具的版本發佈時，自動更新會將每個工程師的 `secrets-vault` 移至新版本，`deploy-kit` 就會損壞。

使用版本約束，`deploy-kit` 聲明它需要 `~2.1.0` 範圍內的 `secrets-vault`。安裝了 `deploy-kit` 的工程師會保持在最高匹配的 `2.1.x` 修補程式版本。部署團隊通過發佈具有更寬鬆約束的新 `deploy-kit` 版本，按照自己的時間表進行升級。

<h2 id="declare-a-dependency-with-a-version-constraint">
  聲明具有版本約束的依賴
</h2>

在 plugin 的 `.claude-plugin/plugin.json` 的 `dependencies` 陣列中列出依賴。每個條目要麼是 plugin 名稱，要麼是具有版本約束的物件。

以下 manifest 聲明了一個未版本化的依賴和一個受約束的依賴：

```json .claude-plugin/plugin.json theme={null}
{
  "name": "deploy-kit",
  "version": "3.1.0",
  "dependencies": [
    "audit-logger",
    { "name": "secrets-vault", "version": "~2.1.0" }
  ]
}
```

一個條目可以是只包含 plugin 名稱的純字符串，如上例中的 `"audit-logger"`，它依賴於該 plugin 的 marketplace 提供的任何版本。為了獲得更多控制，請使用具有以下欄位的物件：

| 欄位            | 類型     | 描述                                                                                                                                                                                                     |
| :------------ | :----- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | string | Plugin 名稱。在與聲明 plugin 相同的 marketplace 內解析。必需。                                                                                                                                                          |
| `version`     | string | 一個 [semver 範圍](https://github.com/npm/node-semver#ranges)，例如 `~2.1.0`、`^2.0`、`>=1.4` 或 `=2.1.0`。依賴會在滿足此範圍的最高標記版本處獲取。                                                                                   |
| `marketplace` | string | 一個不同的 marketplace 來在其中解析 `name`。跨 marketplace 依賴被阻止，除非目標 marketplace 在根 marketplace 的 `marketplace.json` 中的 [`allowCrossMarketplaceDependenciesOn`](#depend-on-a-plugin-from-another-marketplace) 中列出。 |

`version` 欄位接受 Node 的 `semver` 套件支援的任何表達式，包括 caret、tilde、hyphen 和 comparator 範圍。預發佈版本（如 `2.0.0-beta.1`）被排除，除非您的範圍使用預發佈後綴（如 `^2.0.0-0`）選擇加入。

<h2 id="bundle-plugins-for-a-team">
  為團隊組合外掛程式
</h2>

除了必需的 `name` 之外，外掛程式資訊清單可以只包含一個 `dependencies` 陣列。安裝它會拉入每個依賴項，這使其成為在一個安裝後面打包精選外掛程式集的方式。

例如，平台團隊可以在內部市場中發佈角色特定的組合，以便工程師執行一個 `claude plugin install` 而不是分別安裝每個工具：

```json .claude-plugin/plugin.json theme={null}
{
  "name": "backend-standard",
  "version": "1.0.0",
  "description": "Standard plugin set for backend engineers",
  "dependencies": [
    "secrets-vault",
    "deploy-kit",
    { "name": "db-migrate", "version": "^3.0" },
    "oncall-runbook"
  ]
}
```

安裝 `backend-standard` 會解析並安裝所有四個依賴項。

若要稍後將工具新增至標準集，請發佈新的 `backend-standard` 版本並包含額外的依賴項。對於非 Anthropic 市場，自動更新預設為關閉，因此工程師可以透過以下兩種方式之一取得新版本：

* 在 `/plugin` 中為市場啟用自動更新。下一次自動更新會將組合移至新版本並安裝它新增的任何依賴項。
* 執行 `claude plugin update backend-standard`，然後執行 `/reload-plugins` 以安裝新增的依賴項。

若要在整個組織中推出組合，請將組合外掛程式新增至[受管設定](/docs/zh-TW/settings#enabledplugins)中的 `enabledPlugins`。

<h2 id="depend-on-a-plugin-from-another-marketplace">
  依賴來自另一個 marketplace 的 plugin
</h2>

預設情況下，Claude Code 拒絕自動安裝位於與聲明它的 plugin 不同的 marketplace 中的依賴。這可防止一個 marketplace 無聲地從您未審查的來源拉入 plugin。

若要允許此操作，根 marketplace 的維護者將目標 marketplace 名稱添加到 `marketplace.json` 中的 `allowCrossMarketplaceDependenciesOn`。根 marketplace 是託管用戶正在安裝的 plugin 的 marketplace；只有其允許清單被查詢，因此信任不會通過中間 marketplace 鏈接。

以下 `marketplace.json` 允許 `deploy-kit` 依賴來自 `acme-shared` 的 plugin：

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "acme-tools",
  "owner": { "name": "Acme" },
  "allowCrossMarketplaceDependenciesOn": ["acme-shared"],
  "plugins": [
    {
      "name": "deploy-kit",
      "source": "./deploy-kit",
      "dependencies": [
        { "name": "audit-logger", "marketplace": "acme-shared" }
      ]
    }
  ]
}
```

如果欄位缺失或不包含目標 marketplace，安裝將失敗，並出現 `cross-marketplace` 錯誤，命名要設置的欄位。用戶仍然可以先手動安裝依賴，這會滿足約束而無需更改允許清單。

<h2 id="tag-plugin-releases-for-version-resolution">
  標記 plugin 版本發佈以進行版本解析
</h2>

版本約束針對 marketplace 儲存庫上的 git 標籤進行解析。為了讓 Claude Code 找到依賴的可用版本，上游 plugin 的版本發佈必須使用特定的命名約定進行標記。

將每個版本發佈標記為 `{plugin-name}--v{version}`，其中 `{version}` 與該提交的 `plugin.json` 中的 `version` 欄位匹配。從 plugin 目錄中，執行：

```bash theme={null}
claude plugin tag --push
```

`claude plugin tag` 命令從 plugin 的清單和封閉的 marketplace 項目衍生標籤名稱。在建立標籤之前，它驗證 plugin 內容，檢查 `plugin.json` 和 marketplace 項目在版本上是否一致，要求 plugin 目錄下的工作樹乾淨，如果標籤已存在則拒絕。添加 `--dry-run` 以查看將被標記的內容而不建立它。如果您自己保持 `plugin.json` 和 marketplace 項目同步，直接執行 `git tag secrets-vault--v2.1.0` 是等效的。

plugin 名稱前綴讓一個 marketplace 儲存庫可以託管多個具有獨立版本線的 plugin。`--v` 分隔符被解析為完整 plugin 名稱上的前綴匹配，因此包含連字符的 plugin 名稱會被正確處理。

當您安裝聲明 `{ "name": "secrets-vault", "version": "~2.1.0" }` 的 plugin 時，Claude Code 會列出 marketplace 的標籤，篩選以 `secrets-vault--v` 開頭的標籤，並獲取滿足 `~2.1.0` 的最高版本。如果不存在匹配的標籤，依賴 plugin 將被禁用，並出現列出可用版本的錯誤。

已新增為本機資料夾路徑的 marketplace 在該資料夾是 git 儲存庫時會以相同方式解析標籤。這需要 Claude Code v2.1.196 或更新版本。在兩種情況下，Claude Code 會從資料夾的目前內容安裝依賴：

* 較早的版本不會從本機資料夾 marketplace 讀取標籤，因此受約束的依賴只有在該副本滿足範圍時才會載入。
* 不是 git 儲存庫的本機資料夾沒有標籤，無論版本如何。

已解析標籤的 semver 與 `plugin.json` 的 `version` 分開記錄，因此約束檢查使用實際獲取的標籤，即使該提交的 `plugin.json` 有過時的值。標籤解析安裝的快取目錄名稱包含 12 字元的提交 SHA 後綴，因此如果維護者強制移動標籤到不同的提交，下次安裝會獲得新的快取目錄，而不是重複使用過時的內容。

<Note>
  對於 `npm` marketplace 來源，約束不控制獲取的版本，因為基於標籤的解析僅適用於 git 支援的來源。約束仍在載入時檢查，如果已安裝的版本不滿足它，依賴 plugin 將被禁用，並出現 `dependency-version-unsatisfied` 錯誤。
</Note>

<h2 id="how-constraints-interact">
  約束如何相互作用
</h2>

當多個已安裝的 plugins 限制同一依賴時，Claude Code 會交集它們的範圍，並將依賴解析為滿足所有範圍的最高版本。下表顯示常見組合如何解析。

| Plugin A 要求 | Plugin B 要求 | 結果                                                       |
| :---------- | :---------- | :------------------------------------------------------- |
| `^2.0`      | `>=2.1`     | 在最高 `2.x` 標籤處進行一次安裝，該標籤位於 `2.1.0` 或更高版本。兩個 plugins 都會載入。 |
| `~2.1`      | `~3.0`      | Plugin B 的安裝失敗，出現 `range-conflict` 錯誤。Plugin A 和依賴保持原樣。  |
| `=2.1.0`    | 無           | 依賴保持在 `2.1.0`。在安裝了 Plugin A 時，自動更新會跳過較新版本。               |

自動更新會在滿足每個已安裝 plugin 範圍的最高 git 標籤處取得受約束的依賴，而不是在 marketplace 的最新版本處，因此依賴會在其允許的範圍內繼續接收更新。如果沒有標籤滿足所有範圍，自動更新會跳過該依賴，並在 `/plugin` 錯誤標籤中列出跳過，並命名限制 plugin。

當您卸載最後一個限制依賴的 plugin 時，依賴不再被保持，並在下次更新時恢復追蹤其 marketplace 條目。

<h2 id="enable-or-disable-a-plugin-with-dependencies">
  啟用或禁用具有依賴的 plugin
</h2>

啟用 plugin 也會啟用它所依賴的 plugins，禁用 plugin 如果另一個已啟用的 plugin 仍然需要它則會被阻止。這兩種行為都需要 Claude Code v2.1.143 或更新版本。較早的版本只會啟用或禁用命名的 plugin，並在下次載入時出現 `dependency-unsatisfied` 錯誤。

當您啟用 plugin 時，Claude Code 也會在相同的範圍內啟用其依賴。如果依賴有其自己的依賴，Claude Code 也會啟用那些。成功訊息會列出隨著您命名的 plugin 一起啟用的其他內容。如果依賴無法啟用，命令會拒絕並告訴您什麼在阻止以及如何修復：

| 條件                          | 結果                                          |
| :-------------------------- | :------------------------------------------ |
| 依賴未安裝                       | 啟用失敗並為每個遺失的依賴列印 `claude plugin install` 命令。 |
| 依賴被您組織的 plugin 政策阻止         | 啟用失敗並命名被阻止的依賴。                              |
| 依賴在優先級高於目標範圍的範圍內設置為 `false` | 啟用失敗。在該範圍內啟用依賴，或傳遞 `--scope` 以在那裡寫入。        |
| 所有依賴都已安裝且被允許                | 啟用成功並為 plugin 和每個在目標範圍內尚未啟用的依賴寫入 `true`。    |

這甚至在依賴在其 manifest 中設置 [`defaultEnabled: false`](/docs/zh-TW/plugins-reference#default-enablement) 時也成立，因為 Claude Code 為其寫入明確的 `true`。同樣的情況也適用於安裝：為滿足活躍 plugin 而引入的依賴會安裝 `true`，無論其自己的預設值如何。

當您禁用 plugin 時，Claude Code 拒絕如果另一個已啟用的 plugin 仍然依賴它。錯誤命名依賴它的 plugins 並給您一個鏈接命令，以正確的順序禁用它們，以您要求的那個結尾。

例如，如果 `deploy-kit` 依賴 `secrets-vault`，單獨禁用 `secrets-vault` 失敗，輸出類似於以下內容：

```text theme={null}
secrets-vault is still required by deploy-kit. Disable that plugin first, or
disable everything together: claude plugin disable deploy-kit@acme-tools && claude plugin disable secrets-vault@acme-tools
```

從錯誤複製鏈接命令以在一個步驟中禁用完整集合。

<h2 id="remove-orphaned-auto-installed-dependencies">
  移除孤立的自動安裝依賴
</h2>

自動安裝的依賴在安裝它們的 plugins 被卸載後仍會保留在磁碟上，以防您重新安裝依賴 plugin 或想要直接繼續使用依賴。若要清理它們，請執行 `claude plugin prune` 以列出不再有任何已安裝 plugin 需要的自動安裝依賴，並在確認提示後移除它們。這需要 Claude Code v2.1.121 或更新版本。

```bash theme={null}
claude plugin prune
```

預設情況下，prune 在用戶範圍內運作。使用 `--scope project` 或 `--scope local` 來針對不同的範圍。傳遞 `--dry-run` 以列出將被移除的內容而不更改任何內容。傳遞 `-y` 以跳過確認提示。當 stdin 或 stdout 不是終端時，prune 會列出孤立項並退出，除非傳遞了 `-y`。

若要在卸載時進行 prune，請將 `--prune` 傳遞給 `claude plugin uninstall`。移除命名的 plugin 後，Claude Code 會掃描並移除現在孤立的任何自動安裝依賴。您自己安裝的 plugins 永遠不會被 prune，只有通過另一個 plugin 的 `dependencies` 陣列自動安裝的 plugins 才會被 prune。

例如，若要卸載 `deploy-kit` 並清理它留下的依賴：

```bash theme={null}
claude plugin uninstall deploy-kit --prune
```

<h2 id="resolve-dependency-errors">
  解析依賴錯誤
</h2>

依賴問題會在 `claude plugin list` 和 `/plugin` 介面中出現。Claude Code 會禁用受影響的 plugin，直到您解析錯誤。下表列出最常見的錯誤及其解決方法。

| 錯誤                               | 含義                                                                | 如何解析                                                                                                                                   |
| :------------------------------- | :---------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------- |
| `dependency-unsatisfied`         | 聲明的依賴未安裝，或已安裝但被禁用。                                                | 執行錯誤訊息中顯示的 `claude plugin install` 命令。如果依賴的 marketplace 尚未配置，請使用 `claude plugin marketplace add` 添加它，Claude Code 將自動解析依賴。如果依賴被禁用，請啟用它。 |
| `range-conflict`                 | 依賴的版本要求無法組合。錯誤訊息命名原因：沒有版本滿足所有範圍、範圍不是有效的 semver 語法，或組合的範圍太複雜而無法交集。 | 卸載或更新其中一個衝突的 plugin，修復任何無效的 `version` 字符串，簡化長 `\|\|` 鏈，或要求上游作者擴寬其約束。                                                                   |
| `dependency-version-unsatisfied` | 已安裝的依賴版本在此 plugin 的聲明範圍之外。                                        | 執行 `claude plugin install <dependency>@<marketplace>` 以根據所有當前約束重新解析依賴。                                                                 |
| `no-matching-tag`                | 依賴的儲存庫沒有滿足範圍的 `{name}--v*` 標籤。                                    | 檢查上游是否使用上述約定標記了版本發佈，或放寬您的範圍。                                                                                                           |

若要以程式設計方式檢查這些錯誤，請執行 `claude plugin list --json` 並讀取每個 plugin 上的 `errors` 欄位。

<h2 id="see-also">
  另請參閱
</h2>

* [建立 plugins](/docs/zh-TW/plugins)：使用 skills、agents 和 hooks 建立 plugins
* [建立並分發 plugin marketplace](/docs/zh-TW/plugin-marketplaces)：為您的團隊託管 plugins
* [Plugins 參考](/docs/zh-TW/plugins-reference#plugin-manifest-schema)：完整的 `plugin.json` 架構
* [版本管理](/docs/zh-TW/plugins-reference#version-management)：plugin 版本如何被解析並用作快取金鑰
