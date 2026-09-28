> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 外掛程式相依性

> 宣告您的外掛程式所依賴的外掛程式，使用版本範圍如 ^1.2，並查看 Claude Code 如何安裝、解析和修剪它們。

外掛程式相依性是您的外掛程式所依賴的另一個外掛程式，例如您呼叫其 MCP 伺服器或技能的外掛程式。每個相依性會追蹤其市集提供的最新版本，除非您宣告版本約束，即您已測試過的語義版本範圍，例如 `^2.0` 或 `~2.1.0`。

本頁面適用於在 `plugin.json` 中宣告相依性的外掛程式作者，以及標記發行版本的市集維護者。

<Note>
  這些情況涵蓋在其他頁面上：

  * **安裝具有相依性的外掛程式**：請參閱[管理已安裝的外掛程式](/docs/zh-TW/plugins/install#manage-installed-plugins)
  * **讀取相依性錯誤**：請參閱[相依性錯誤](/docs/zh-TW/plugins/troubleshooting#dependency-errors)
  * **宣告您的外掛程式自身程式碼所需的 npm 和 Bun 套件**：請參閱 [Node.js 套件相依性](/docs/zh-TW/plugins/loading#node-js-package-dependencies)
</Note>

若要新增約束，請從[使用版本約束宣告相依性](#declare-a-dependency-with-a-version-constraint)開始。如果您維護其他人依賴的外掛程式，請[標記您的發行版本](#tag-plugin-releases-for-version-resolution)，以便其約束可以解析。

<h2 id="declare-dependencies">
  宣告依賴
</h2>

<span id="decide-whether-to-constrain-dependency-versions" />沒有版本約束的情況下，依賴會在使用者下次更新時移至其 marketplace 發佈的每個新發行版本。如果該發行版本重新命名了你的 plugin 呼叫的 MCP 工具，你的 plugin 會對所有更新的人中斷。

使用約束（例如來自 git 支援來源的依賴上的 `~2.1.0`），已安裝你的 plugin 的使用者會持續接收依賴的 `2.1.x` 修補程式，永遠不會移至 `2.2`。若要按自己的時間表升級，請針對較新的發行版本進行測試，然後發佈你的 plugin 的新版本，其中包含更寬的約束。

<h3 id="declare-a-dependency-with-a-version-constraint">
  使用版本約束宣告依賴
</h3>

在你的 plugin 的 `.claude-plugin/plugin.json` 的 `dependencies` 陣列中列出依賴。以下資訊清單宣告一個未版本化的依賴和一個受約束的依賴：

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

一個項目可以是字串：僅 plugin 名稱，例如此資訊清單中的 `"audit-logger"`，或 `"name@marketplace"` 以在另一個 marketplace 中解析它。使用裸字串，你的 plugin 依賴於該 plugin 的 marketplace 提供的任何版本。

若要設定版本約束，請使用具有這些欄位的物件，每個都是字串：

| 欄位            | 說明                                                                                                                                                                                |
| :------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | 依賴的 plugin 名稱，如其 marketplace 項目中所示。Claude Code 在與宣告 plugin 相同的 marketplace 中查詢它，除非你設定 `marketplace`。必需。                                                                           |
| `version`     | [語義版本範圍](https://github.com/npm/node-semver#ranges)，例如 `~2.1.0`、`^2.0`、`>=1.4` 或 `=2.1.0`。依賴會安裝在滿足此範圍的最高 git 標籤，因此依賴的維護者必須 [標記發行版本](#tag-plugin-releases-for-version-resolution)。 |
| `marketplace` | 用於解析 `name` 的不同 marketplace。允許清單控制跨 marketplace 依賴，詳見 [依賴來自另一個 marketplace 的 plugin](#depend-on-a-plugin-from-another-marketplace)。                                               |

範圍不符合預發行版本，例如 `2.0.0-beta.1`，除非你使用預發行後綴（例如 `^2.0.0-0`）選擇加入。

<h3 id="bundle-plugins-for-a-team">
  為團隊組合 plugin
</h3>

若要讓工程師使用一個命令安裝精選的 plugin 集合，請發佈一個資訊清單包含 `name` 和 `dependencies` 陣列的 plugin。Plugin 資訊清單只需要 `name`，所以這是一個有效的 plugin，安裝它會安裝每個依賴。

例如，平台團隊可以在內部 marketplace 中發佈角色特定的組合，以便工程師執行一個 `claude plugin install` 而不是分別安裝每個 plugin：

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

若要稍後將 plugin 新增至標準集合，請發佈新的 `backend-standard` 版本，其中包含額外的依賴。當 marketplace 不 [預設自動更新](/docs/zh-TW/plugins/loading#which-marketplaces-and-plugins-auto-update) 時，工程師要麼為 marketplace 開啟自動更新，要麼手動更新：

* **為 marketplace 開啟自動更新**：下一次自動更新會將組合移至新版本並安裝它新增的任何依賴。
* **手動更新**：在 shell 中執行 `claude plugin update backend-standard`，然後在開啟的工作階段中執行 `/reload-plugins` 以安裝新增的依賴。

有關工程師端的步驟，請參閱 [保持 plugin 更新](/docs/zh-TW/plugins/install#keep-plugins-updated)。

若要將組合部署給組織中的每個人，管理員會將其新增至受管設定中的 `enabledPlugins`。請參閱 [預先安裝並要求 plugin](/docs/zh-TW/plugins/org#pre-install-and-require-plugins)。

<h3 id="depend-on-a-plugin-from-another-marketplace">
  依賴來自另一個 marketplace 的 plugin
</h3>

預設情況下，Claude Code 不會從與宣告 plugin 自身不同的 marketplace 安裝依賴，除非使用者已在相同範圍內安裝並啟用該依賴。此預設值可防止一個 marketplace 從使用者未審查的來源無聲地安裝 plugin。

若要允許安裝，請將目標 marketplace 的名稱新增至根 marketplace 的 `marketplace.json` 中的 `allowCrossMarketplaceDependenciesOn`。根 marketplace 是託管使用者正在安裝的 plugin 的 marketplace。只有根 marketplace 的允許清單適用。

以下 `marketplace.json` 允許 `deploy-kit` 依賴來自 `your-shared-marketplace` 的 plugin：

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "allowCrossMarketplaceDependenciesOn": ["your-shared-marketplace"],
  "plugins": [
    {
      "name": "deploy-kit",
      "source": "./deploy-kit",
      "dependencies": [
        { "name": "audit-logger", "marketplace": "your-shared-marketplace" }
      ]
    }
  ]
}
```

如果 `allowCrossMarketplaceDependenciesOn` 遺失或不包含目標 marketplace，Claude Code 不會安裝依賴。當依賴在 marketplace 項目中宣告時，安裝本身會被拒絕，並顯示以 `Dependency "audit-logger@your-shared-marketplace" (required by deploy-kit@your-marketplace) is in marketplace "your-shared-marketplace", which is not in the allowlist` 開頭的訊息，並命名要設定的欄位。當它在 `plugin.json` 中宣告時，安裝完成而不包含依賴，然後你的 plugin 無法載入。

允許清單檢查不適用於已啟用的依賴。如果使用者首先在相同範圍內從 `your-shared-marketplace` 自行安裝 `audit-logger`，`deploy-kit` 隨後會安裝而無需對允許清單進行任何變更。

<h3 id="test-a-plugin-and-its-dependency-locally">
  在本機測試 plugin 及其依賴
</h3>

如果你同時開發 plugin 及其依賴的 plugin，請從 shell 啟動 Claude Code 並使用 [`--plugin-dir`](/docs/zh-TW/plugins/cli-reference#flags-that-load-a-plugin-for-one-session) 載入兩者：

```bash theme={null}
claude --plugin-dir ./my-dependency --plugin-dir ./my-plugin
```

依賴的本機副本滿足你的 plugin 的依賴項目，因此你不需要從其 marketplace 安裝依賴。

* **不需要 `version`**：本機 `plugin.json` 也不需要 `version`，因為 [版本約束](#declare-a-dependency-with-a-version-constraint) 不會針對本機副本進行檢查。
* **命名 marketplace 的項目**：命名 marketplace 的項目也符合 Claude Code v2.1.242 或更新版本上的本機副本。

在你從其 marketplace 安裝依賴之前，每當本機副本被停用或不存在時，你的 plugin 都會停止載入：

* **你停用了本機副本**：你的 plugin 在下一次 plugin 載入時被停用，並顯示以 `is disabled — enable it or remove the dependency` 結尾的錯誤。當錯誤將依賴命名為 `<name>@inline` 時，該識別碼指的是 `--plugin-dir` 副本。
* **你啟動了沒有依賴的 `--plugin-dir` 旗標的工作階段**：錯誤報告依賴未安裝。再次傳遞旗標，或從其 marketplace 安裝依賴。

當兩個 plugin 都在一個父資料夾中時，你可以一次將該資料夾傳遞給 `--plugin-dir`。如果資料夾本身不是 plugin，Claude Code 會載入每個具有 `.claude-plugin/plugin.json` 的子資料夾。需要 Claude Code v2.1.265 或更新版本。

<h2 id="tag-plugin-releases-for-version-resolution">
  發行其他人依賴的 plugin
</h2>

如果你維護其他 plugin 使用版本約束依賴的 plugin，請標記其發行版本，以便這些約束可以解析。約束會針對託管 plugin 的儲存庫上的 git 標籤進行解析。標記 plugin 的 [plugin 來源](/docs/zh-TW/plugins/marketplace-reference#plugin-sources) 在 `marketplace.json` 中指向的儲存庫：

* **`github`、`url` 或 `git-subdir` 來源**：plugin 自身的儲存庫，因此 plugin 的作者建立標籤
* **相對路徑，例如 `./plugins/secrets-vault`**：marketplace 儲存庫，因此 marketplace 維護者建立標籤

<h3 id="create-a-release-tag">
  建立發行標籤
</h3>

將每個發行版本標記為 `<plugin-name>--v<version>`，其中 `<version>` 符合該提交的 `plugin.json` 中的 `version` 欄位。plugin-name 前綴讓一個 marketplace 儲存庫可以託管多個具有獨立版本歷史的 plugin。

從 plugin 目錄建立標籤，並配置 `origin` 遠端以接收推送的標籤，使用 [`claude plugin tag`](/docs/zh-TW/plugins/cli-reference#plugin-tag)：

```bash theme={null}
claude plugin tag --push
```

該命令從 plugin 的資訊清單建立標籤名稱。在建立標籤之前，它執行這些檢查：

* 驗證 plugin
* 當 plugin 目錄在 marketplace 簽出內時，檢查 `plugin.json` 和 marketplace 項目是否同意版本
* 需要 plugin 目錄下的乾淨工作樹
* 如果標籤已存在，則拒絕

成功執行會列印 `Created tag secrets-vault--v2.1.0`。使用 `--push`，它也會列印 `Pushed to origin`。沒有 `--push`，它會列印你自己執行的 `git push` 命令。

傳遞 `--dry-run` 以查看計畫而不建立任何內容。

[`claude plugin tag` 參考](/docs/zh-TW/plugins/cli-reference#plugin-tag) 列出其餘旗標。

你也可以直接執行 `git tag secrets-vault--v2.1.0`，只要你自己保持 `plugin.json` 中的 `version` 和 marketplace 項目中的版本同步。

<h3 id="constrain-a-dependency-that-has-a-non-git-source">
  約束具有非 git 來源的依賴
</h3>

標籤型解析僅適用於 git 支援的來源。對於具有 `npm`、`archive` 或 `command` [plugin 來源](/docs/zh-TW/plugins/marketplace-reference#plugin-sources) 的依賴，約束不控制擷取哪個版本。當 plugin 載入時仍會檢查它，如果已安裝的版本不滿足它，則依賴 plugin 會被停用。

對於 `npm`、`archive` 和 `command` 來源，檢查的版本是依賴的 `plugin.json` 中的 `version`。在約束該依賴之前在那裡設定一個，因為不設定版本的 `plugin.json` 不滿足任何約束。

Claude Code 永遠不會自行安裝具有 `command` 來源的依賴，因此使用者 [首先安裝它](/docs/zh-TW/plugins/marketplace-reference#command-plugin-source)。它也永遠不會執行依賴的 [`headersHelper`](/docs/zh-TW/plugins/host-marketplace#authenticate-archive-downloads)，因此使用者也在安裝你的 plugin 之前安裝其 marketplace 項目設定的依賴。

除了 `claude plugin install` 之外，這些操作也會安裝任何遺失的宣告依賴，`command` 和 `headersHelper` 限制也適用於它們：

* `/reload-plugins`
* 依賴 plugin 的 marketplace 自動更新
* 在依賴 plugin 上重新執行 `claude plugin install`
* `claude plugin marketplace add`

<h2 id="how-dependencies-behave-for-your-users">
  依賴如何為你的使用者表現
</h2>

這些部分說明一旦你的 plugin 與其他 plugin 一起安裝，Claude Code 如何解析、檢查和組合你宣告的約束。

<h3 id="how-a-constraint-resolves-against-tags">
  約束如何針對標籤進行解析
</h3>

當使用者安裝宣告 `{ "name": "secrets-vault", "version": "~2.1.0" }` 的 plugin 時，依賴會從滿足 `~2.1.0` 的最高 `secrets-vault--v` 標籤安裝在託管 `secrets-vault` 的儲存庫上。當沒有標籤滿足範圍時，安裝要麼失敗，要麼使用 marketplace 的目前副本：

* **具有自身儲存庫的 plugin**：安裝失敗，訊息包含 `Dependency "secrets-vault@your-marketplace" has no git tag satisfying`。
* **由相對路徑參考的 plugin**：安裝改為使用 marketplace 的目前副本，並在 plugin 載入時檢查約束。如果該副本超出範圍，依賴 plugin 保持停用，`claude plugin list` 顯示 `Requires "secrets-vault@your-marketplace" ~2.1.0, installed 3.0.0`。

對於 marketplace 由相對路徑參考的 plugin，你新增為本機資料夾路徑的 marketplace 也會針對該資料夾的 git 標籤解析約束，當資料夾是 git 儲存庫時。這需要 Claude Code v2.1.196 或更新版本。不是 git 儲存庫的本機資料夾沒有標籤，因此 Claude Code 改為從資料夾的目前內容安裝依賴。

<h3 id="confirm-the-resolved-version">
  確認解析的版本
</h3>

若要確認約束解析到哪個版本，請在 shell 中執行 `claude plugin list`。標籤解析的依賴會顯示其版本，帶有 12 字元提交後綴，例如 `2.1.0-8713c5b11005`。

約束檢查使用標籤的版本而不是 `plugin.json` 中的 `version`，即使該提交的 `plugin.json` 落後。

如果你強制移動標籤到不同的提交，下一次安裝會擷取該提交的內容而不是重複使用陳舊的快取副本。請參閱 [版本和更新](/docs/zh-TW/plugins/loading#versions-and-updates) 以了解 plugin 的版本如何成為其快取鍵。

<h3 id="combine-constraints-from-several-plugins">
  組合來自多個 plugin 的約束
</h3>

當多個已安裝的 plugin 約束相同的依賴時，依賴會解析到滿足所有其範圍的最高版本。常見的組合解析如下：

| Plugin A 要求 | Plugin B 要求 | 結果                                                                             |
| :---------- | :---------- | :----------------------------------------------------------------------------- |
| `^2.0`      | `>=2.1`     | 在最高 `2.x` 標籤處進行一次安裝，位於或高於 `2.1.0`。兩個 plugin 都載入。                               |
| `~2.1`      | `~3.0`      | 安裝 plugin B 失敗，並顯示 `has conflicting version requirements` 訊息。Plugin A 和依賴保持原樣。 |
| `=2.1.0`    | 無           | 依賴保持在 `2.1.0`。自動更新在安裝 plugin A 時跳過較新的版本。                                       |

自動更新會在滿足每個已安裝 plugin 範圍的最高 git 標籤處擷取受約束的依賴，而不是在 marketplace 的最新版本處。如果已安裝 plugin 的範圍不重疊，自動更新會將該依賴保留在其目前版本，`/plugin` **Errors** 標籤會顯示命名約束 plugin 的項目。如果它們重疊但沒有標籤落在範圍內，自動更新會擷取 marketplace 的目前副本，並在該副本的 `version` 落在任何已安裝 plugin 的範圍之外時跳過更新。

當使用者卸載最後一個約束依賴的 plugin 時，依賴不再受約束於版本範圍，並在下一次更新時恢復追蹤其 marketplace 項目。

<h2 id="see-also">
  另請參閱
</h2>

* [`claude plugin prune`](/docs/zh-TW/plugins/cli-reference#plugin-prune)：移除任何 plugin 不再需要的自動安裝依賴
* [託管 marketplace](/docs/zh-TW/plugins/host-marketplace)：發行通道和推薦其他 plugin
