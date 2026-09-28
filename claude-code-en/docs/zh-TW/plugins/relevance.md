> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 為您的組織推薦 plugins

> 在 marketplace plugin 項目中新增相關性區塊，以便當使用者的工作符合時 Claude Code 會建議安裝，並在受管設定中將 marketplace 加入允許清單。

Claude Code 可以在使用者的工作階段符合您為該 plugin 定義的訊號時，建議從您組織的 marketplace 安裝 plugin。訊號包括工作目錄、Claude 已讀取的檔案，以及 Claude 已執行的命令。您可以透過在 `marketplace.json` 中的 plugin 項目新增 `relevance` 區塊來定義這些訊號。

marketplace 操作員會撰寫 `relevance` 項目。管理員隨後在受管設定中將 marketplace 加入允許清單。在 marketplace 被加入允許清單之前，使用者看不到來自該 marketplace 的任何建議。

<Note>
  這些情況涵蓋在其他頁面上：

  * **您想要安裝 plugins**：請參閱 [安裝和管理 plugins](/docs/zh-TW/plugins/install)
  * **您想要關閉建議**：請參閱 [了解 plugin 相關性的運作方式](#understand-how-plugin-relevance-works)
</Note>

從適合您角色的部分開始：

* **Marketplace 操作員**：閱讀 [建議的運作方式](#understand-how-plugin-relevance-works)，然後 [將相關性新增至 plugin 項目](#add-relevance-to-a-plugin-entry) 並 [驗證您的 marketplace](#validate-your-marketplace)
* **管理員**：[在受管設定中啟用建議](#enable-suggestions-in-managed-settings)

<h2 id="understand-how-plugin-relevance-works">
  了解 plugin 相關性的運作方式
</h2>

`marketplace.json` 中的每個 plugin 項目都可以包含 `relevance` 物件。該物件命名一個主題和一個或多個訊號。訊號是 Claude Code 針對目前工作階段測試的模式，例如工作目錄或 Claude 已讀取的檔案。

訊號比對在使用者的機器上本地進行，不會增加任何網路流量。Claude Code 不會向 Anthropic 或 marketplace 操作員報告哪些訊號相符或其值。

當訊號相符且 plugin 尚未安裝時，Claude Code 會在以下位置建議該 plugin：

* **Spinner 提示**：當 Claude 正在回應時，包含 `/plugin install` 命令的訊息會出現在 spinner 下方。
* **工作階段開始通知**：如果 `cwd` 訊號符合工作目錄，在使用者傳送第一則訊息之前會出現一行通知。
* **`/plugin` Discover 標籤**：plugin 會釘選在 Discover 清單的頂部。

[預覽使用者看到的內容](#preview-what-the-user-sees) 顯示每個的確切文字以及它們重複的頻率。

Claude Code 永遠不會自動安裝 plugin。使用者始終確認。

當使用者或專案將 [`spinnerTipsEnabled`](/docs/zh-TW/settings-reference#spinnertipsenabled) 設定為 `false`，或當 [`spinnerTipsOverride`](/docs/zh-TW/settings-reference#spinnertipsoverride) 搭配 `excludeDefault` 取代內建提示時，spinner 提示和工作階段開始通知都會停止出現。Discover 標籤釘選不受任一設定影響。

<h2 id="add-relevance-to-a-plugin-entry">
  將相關性新增至 plugin 項目
</h2>

將 `relevance` 物件新增至您 `marketplace.json` 中的 plugin 項目。下列範例宣告當 Claude 讀取 `.tf` 檔案或執行 `terraform` 時，`terraform-helpers` plugin 是相關的：

```json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "plugins": [
    {
      "name": "terraform-helpers",
      "source": "./plugins/terraform-helpers",
      "description": "Your organization's Terraform conventions and helpers",
      "relevance": {
        "topic": "Terraform",
        "signals": {
          "cli": ["terraform"],
          "filesRead": ["**/*.tf"]
        }
      }
    }
  ]
}
```

當其訊號都不相符時，plugin 會保持在 Discover 清單中的正常位置，不會顯示為 spinner 提示。

若要在發佈前檢查該區塊，請 [驗證您的 marketplace](#validate-your-marketplace)。

<h2 id="field-reference">
  欄位參考
</h2>

`relevance` 物件及其巢狀 `signals` 物件接受下列表格中的欄位。

較舊的用戶端仍會載入使用它們無法識別的 `relevance` 欄位的 marketplace，因為在載入時會忽略 `relevance` 和 `relevance.signals` 下的未知欄位。已識別的欄位其值超過 [欄位參考](#field-reference) 中的限制會使整個 plugin 項目失效，使用者無法從 marketplace 安裝該 plugin，直到您修復它；`claude plugin validate` 會報告相同的限制。

<h3 id="relevance">
  `relevance`
</h3>

| 欄位        | 類型 | 說明                                                                                                     |
| :-------- | :- | :----------------------------------------------------------------------------------------------------- |
| `topic`   | 字串 | 選用。填入 spinner 提示中「使用 *topic*？」的片語。預設為 plugin 名稱，每個連字號區段首字大寫。最多 64 個字元。                                 |
| `signals` | 物件 | 決定 plugin 何時相關的比對器。Claude Code 只有在至少設定一個訊號時才會建議該 plugin。請參閱 [`relevance.signals`](#relevance-signals)。 |

`topic` 通常是產品名稱，例如 `Terraform`。當 plugin 名稱作為主題聽起來不自然時，使用 `design` 之類的領域。

<h3 id="relevance-signals">
  `relevance.signals`
</h3>

`signals` 物件接受下列欄位。

| 欄位             | 類型   | 說明                                                                                                                                     | 限制                                         |
| :------------- | :--- | :------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------- |
| `cwd`          | 字串陣列 | 針對工作階段工作目錄比對的 Glob 模式。請參閱 [工作目錄比對](#working-directory-matching)。                                                                       | 10 個模式，每個 256 個字元                          |
| `cli`          | 字串陣列 | Claude 在此工作階段執行的 shell 命令中的命令名稱，例如 `["terraform"]`。完全相符。請參閱 [命令名稱比對](#command-name-matching)。                                          | 10 個項目，每個 64 個字元                           |
| `hosts`        | 字串陣列 | 此工作階段中 Bash 命令中 `http://` 或 `https://` URL 中看到的主機名稱，例如 `["registry.terraform.io"]`。僅限裸露小寫主機名稱：無配置、連接埠或路徑。完全不區分大小寫的相符。                  | 20 個項目，每個 128 個字元                          |
| `filesRead`    | 字串陣列 | 針對 Claude 在此工作階段讀取的檔案路徑比對的 Glob 模式，例如 `["**/*.tf"]`。正斜線正規化且不區分大小寫。                                                                     | 10 個模式，每個 256 個字元                          |
| `manifestDeps` | 物件陣列 | Claude 在此工作階段讀取的套件資訊清單中宣告的相依性。每個項目是 `{ "file": "...", "pattern": "..." }`，其中兩個值都是正規表達式。請參閱 [資訊清單相依性比對](#manifest-dependency-matching)。 | 10 個項目，每個值最多 256 個字元。大於 512 KB 的資訊清單檔案會被略過 |

`filesRead` 和 `manifestDeps` 訊號也會比對 Claude 在此工作階段中寫入或編輯的檔案，以及專案的自動載入 `CLAUDE.md` 記憶體檔案。

<h4 id="working-directory-matching">
  工作目錄比對
</h4>

`cwd` 是唯一可以在工作階段開始時比對的訊號，在使用者傳送第一則訊息之前。

Claude Code 比對每個 `cwd` 模式如下：

* 模式會針對工作目錄作為絕對路徑進行比對。當工作階段在 git 儲存庫內時，它也會針對相對於儲存庫根目錄的工作目錄路徑進行比對。
* 比對是正斜線正規化且不區分大小寫的。
* 每個模式都會比對目錄本身及其下的所有內容，因此 `infra`、`infra/` 和 `infra/**` 的行為相同。

<h4 id="command-name-matching">
  命令名稱比對
</h4>

Claude Code 為 Claude 執行的每個 shell 命令記錄一個命令名稱：任何前導環境變數指派和 `sudo` 之後的第一個權杖。複合命令只貢獻其前導命令，因此 `cd infra && terraform plan` 記錄 `cd`，而不是 `terraform`。

<h4 id="manifest-dependency-matching">
  資訊清單相依性比對
</h4>

每個 `manifestDeps` 項目配對兩個 JavaScript `RegExp` 來源字串：

* `file`：不區分大小寫地針對資訊清單檔案的路徑進行比對。路徑通常是絕對的，因此請在結尾而不是開頭錨定模式。路徑不會針對此訊號進行分隔符號正規化，因此 Windows 路徑使用反斜線。
* `pattern`：區分大小寫地針對該檔案的內容進行比對。

下列範例使用 `manifestDeps` 在 Claude 讀取了依賴您 SDK npm 套件（此處名為 `your-sdk`）的 `package.json` 後建議您的 plugin。

```json theme={null}
{
  "name": "your-plugin",
  "source": "./plugins/your-plugin",
  "relevance": {
    "signals": {
      "manifestDeps": [
        {
          "file": "[/\\\\]package\\.json$",
          "pattern": "\"your-sdk\"\\s*:"
        }
      ]
    }
  }
}
```

在此範例中，`file` 模式使用 `[/\\\\]` 以便同時比對正斜線和反斜線路徑分隔符號，以及 `\\.` 使點是字面意思。在 JSON 中，正規表達式中的每個反斜線都寫兩次。

<h2 id="validate-your-marketplace">
  驗證您的 marketplace
</h2>

在您的 shell 中，針對您的 marketplace 目錄執行 `claude plugin validate` 以在發佈前檢查 `relevance` 區塊：

```bash theme={null}
claude plugin validate ./my-marketplace
```

驗證器會報告 `relevance` 區塊上的錯誤和警告，包括這些：

* 將 `relevance` 和 `relevance.signals` 下的未知鍵報告為警告
* 標記不是物件的 `relevance` 值
* 拒絕包含配置、連接埠或路徑的 `signals.hosts` 項目

每個發現都會列印它所涉及的欄位的路徑，輸出以 `Validation passed`、`Validation passed with warnings` 或 `Validation failed` 結尾。

<h2 id="enable-suggestions-in-managed-settings">
  在受管設定中啟用建議
</h2>

使用者看不到來自 marketplace 的任何建議，直到管理員在 [受管設定](/docs/zh-TW/plugins/org) 中將其加入允許清單，即使其 `marketplace.json` 宣告了 `relevance`。

若要將 marketplace 加入允許清單，請編輯您的受管設定如下：

* 將 marketplace 名稱新增至 `pluginSuggestionMarketplaces`。
* 對於官方 Anthropic marketplace 以外的任何 marketplace，也請宣告 marketplace 來源，可以是 [`extraKnownMarketplaces`](/docs/zh-TW/plugins/org#require-a-marketplace-and-its-plugins) 中該名稱的項目，或 [`strictKnownMarketplaces`](/docs/zh-TW/plugins/org#allowlist-with-strictknownmarketplaces) 中的項目。

在未註冊 marketplace 的機器上，或從不同來源以允許清單名稱註冊的機器上，不會出現來自它的任何建議。來源檢查會阻止無關的來源以允許清單名稱註冊以在您的組織中建議其 plugins。

下列 `managed-settings.json` 從 GitHub 儲存庫註冊組織 marketplace 並啟用其建議：

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": {
        "source": "github",
        "repo": "your-org/your-marketplace"
      }
    }
  },
  "pluginSuggestionMarketplaces": ["your-marketplace"]
}
```

官方 marketplace 的名稱只能從官方 Anthropic 來源註冊，因此不需要來源宣告。對於官方 marketplace，僅將名稱加入允許清單：

```json theme={null}
{
  "pluginSuggestionMarketplaces": ["claude-plugins-official"]
}
```

<h2 id="preview-what-the-user-sees">
  預覽使用者看到的內容
</h2>

當 plugin 的 `relevance` 訊號在工作階段期間相符時，spinner 下方的提示讀取：

```text theme={null}
Working with Terraform? Install the terraform-helpers plugin:
/plugin install terraform-helpers@your-marketplace
```

當 `cwd` 訊號在工作階段開始時相符時，一行通知讀取：

```text theme={null}
plugin suggestion: terraform-helpers@your-marketplace · /plugin
```

在 `/plugin` Discover 標籤中，plugin 會釘選在其他結果上方，並帶有命名相符訊號的註釋，例如 `suggested for this directory` 或 `suggested for terraform commands`。

Claude Code 限制建議給定 plugin 的頻率：

* 建議在 spinner 提示和工作階段開始通知的組合中最多每三個工作階段出現一次。
* 一旦 spinner 提示和通知已組合顯示該 plugin 兩次，工作階段開始通知就會停止出現。
* 一旦安裝了 plugin，spinner 提示和工作階段開始通知都不會重複。
* Discover 標籤會在使用者在 plugin 訊號相符時首次開啟標籤時釘選該 plugin。Claude Code 會在 `~/.claude.json` 中記錄這一點，因此每次使用者稍後在該機器上開啟 `/plugin` 時，該 plugin 都會以正常順序出現。

<h2 id="see-also">
  另請參閱
</h2>

* [託管 marketplace](/docs/zh-TW/plugins/host-marketplace)：執行託管您的 plugins 的 marketplace
* [Marketplace 參考](/docs/zh-TW/plugins/marketplace-reference#plugin-entries)：plugin 項目接受的每個欄位
* [從您的 CLI 推薦您的 plugin](/docs/zh-TW/plugins/cli-hints)：從您自己的 CLI 而不是從 Claude Code 的工作階段訊號提示使用者
* [為您的組織管理 plugins](/docs/zh-TW/plugins/org)：`extraKnownMarketplaces`、`strictKnownMarketplaces` 和其餘 plugin 原則鍵
