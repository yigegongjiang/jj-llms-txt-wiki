> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 設定自動模式

> 告訴自動模式分類器您的組織信任哪些儲存庫、儲存桶和網域。設定環境內容、覆蓋預設的封鎖和允許規則，並使用自動模式 CLI 子命令檢查您的有效設定。

[自動模式](/docs/zh-TW/permission-modes#eliminate-prompts-with-auto-mode)讓 Claude Code 無需例行權限提示即可執行，方法是透過分類器路由工具呼叫，該分類器會封鎖任何不可逆、破壞性或針對您環境外的操作。拒絕和明確要求規則在分類器之前進行評估，仍然會封鎖或提示。使用 `autoMode` 設定區塊告訴該分類器您的組織信任哪些儲存庫、儲存桶和網域，以便它停止封鎖例行內部操作。

<Note>
  自動模式適用於所有提供者上的所有使用者，包括 Anthropic API、Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 和已登入的 [Claude 應用程式閘道](/docs/zh-TW/claude-apps-gateway)工作階段。如果 Claude Code 報告您的帳戶無法使用自動模式，請檢查[完整要求](/docs/zh-TW/permission-modes#eliminate-prompts-with-auto-mode)，其中也涵蓋支援的模型和 Team 及 Enterprise 方案上的擁有者啟用。在 v2.1.158 至 v2.1.206 中，Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 和 Claude 應用程式閘道工作階段上的自動模式需要設定 `CLAUDE_CODE_ENABLE_AUTO_MODE=1`；v2.1.207 移除了該要求。
</Note>

根據預設，分類器只信任工作目錄和目前儲存庫的已設定遠端。推送到您公司的原始碼控制組織或寫入團隊雲端儲存桶等操作會被封鎖，直到您將它們新增到 `autoMode.environment`。

如需了解如何啟用自動模式及其預設封鎖的內容，請參閱[權限模式](/docs/zh-TW/permission-modes#eliminate-prompts-with-auto-mode)。此頁面是設定參考。

此頁面涵蓋如何：

* [為推送和提取請求新增人工檢查點](#common-boundaries)，使用 `permissions.ask`
* [選擇在何處設定規則](#where-the-classifier-reads-configuration)，跨越 CLAUDE.md、使用者設定和受管設定
* [定義受信任的基礎結構](#define-trusted-infrastructure)，使用 `autoMode.environment`
* [覆蓋封鎖和允許規則](#override-the-block-and-allow-rules)，當預設值不符合您的管道時
* [透過分類器路由所有 shell 命令](#route-all-shell-commands-through-the-classifier)，使用 `autoMode.classifyAllShell`
* [檢查您的有效設定](#inspect-the-defaults-and-your-effective-config)，使用 `claude auto-mode` 子命令
* [檢查拒絕](#review-denials)，以便您知道接下來要新增什麼

<h2 id="common-boundaries">
  常見的邊界
</h2>

自動模式預設允許推送到您的工作分支、例行推送到儲存庫預設分支，以及建立拉取請求。分類器僅在推送存在風險時才會阻止，例如強制推送或繞過您設定的審查的內容。如果您想在每次推送或拉取請求前進行人工檢查點，請新增權限規則：下面的配方會保持自動模式對所有其他操作開啟。

最直接的機制是 [`permissions.ask`](/docs/zh-TW/permissions#permission-rule-syntax)。內容範圍的 ask 規則（如下面的規則）在分類器之前進行評估，並且始終強制權限提示，即使在自動模式下也是如此，因為明確的 ask 規則是您要求提示該操作的明確意圖。在您的 [設定](/docs/zh-TW/settings#settings-files) 中新增規則：

```json theme={null}
{
  "permissions": {
    "ask": [
      "Bash(git push *)",
      "Bash(gh pr create *)"
    ]
  }
}
```

選擇與邊界需要有多堅定相符的機制：

| 邊界          | 機制                    | 自動模式中的行為                                                                                             |
| :---------- | :-------------------- | :--------------------------------------------------------------------------------------------------- |
| 在操作前提示      | `permissions.ask`     | 始終為內容範圍的規則（如上面的配方）提示。分類器無法自動批准匹配的操作。                                                                 |
| 永不執行操作      | `permissions.deny`    | 在諮詢分類器之前阻止。分類器和使用者意圖都無法覆蓋它。                                                                          |
| 此工作階段的一次性邊界 | 在對話中陳述，例如「在我審查之前不要推送」 | 分類器會阻止匹配的操作，但如果 [內容壓縮](/docs/zh-TW/costs#reduce-token-usage) 移除了陳述該邊界的訊息，邊界可能會遺失。使用 ask 或 deny 規則以獲得持久保證。 |

<h2 id="where-the-classifier-reads-configuration">
  分類器讀取設定的位置
</h2>

分類器讀取與 Claude 本身載入相同的 [CLAUDE.md](/docs/zh-TW/memory) 內容，因此在您專案的 CLAUDE.md 中的指令（例如「永遠不要強制推送」）會同時引導 Claude 和分類器。請從該處開始了解專案慣例和行為規則。

對於跨專案適用的規則，例如受信任的基礎設施或組織範圍的拒絕規則，請使用 `autoMode` 設定區塊。分類器從以下範圍讀取 `autoMode`：

| 範圍                         | 檔案                                       | 用途               |
| :------------------------- | :--------------------------------------- | :--------------- |
| 單一開發者                      | `~/.claude/settings.json`                | 個人受信任的基礎設施       |
| 組織範圍                       | [受管理的設定](/docs/zh-TW/server-managed-settings) | 分散給所有開發者的受信任基礎設施 |
| `--settings` 旗標或 Agent SDK | 內嵌 JSON                                  | 自動化的每次調用覆蓋       |

分類器不會從 `.claude/settings.json` 或 `.claude/settings.local.json` 中的專案設定讀取 `autoMode`。兩個檔案都位於儲存庫目錄中，因此已簽入的儲存庫或建置步驟可能會注入自己的允許規則。在 v2.1.207 之前，分類器也會讀取 `.claude/settings.local.json`；請將該檔案中的任何 `autoMode` 區塊移至 `~/.claude/settings.json`。排除 `.claude/settings.local.json` 也會關閉儲存庫提交該檔案或本機工具或建置步驟寫入該檔案的情況。

來自每個範圍的項目會被合併。開發者可以使用個人項目擴展 `environment`、`allow`、`soft_deny` 和 `hard_deny`，但無法移除受管理設定提供的項目。由於允許規則在分類器內部充當軟區塊規則的例外，開發者新增的 `allow` 項目可以覆蓋組織的 `soft_deny` 項目：組合是累加的，而不是硬政策邊界。

<Note>
  分類器是在[權限系統](/docs/zh-TW/permissions)之後執行的第二道閘門。對於無論使用者意圖或分類器設定如何都必須永遠不執行的動作，請在受管理設定中使用 `permissions.deny`，它會在諮詢分類器之前阻止該動作，且無法被覆蓋。
</Note>

<h2 id="define-trusted-infrastructure">
  定義受信任的基礎設施
</h2>

對於大多數組織，`autoMode.environment` 是您唯一需要設定的欄位。它告訴分類器哪些儲存庫、儲存桶和網域是受信任的：分類器使用它來決定「外部」的含義，因此任何未列出的目的地都是潛在的資料外洩目標。

自 Claude Code v2.1.198 起，`claude auto-mode defaults` 列印三種環境項目。v2.1.195 之前的版本只列印前五個信任槽位。

* **上下文槽位**：描述您的組織、堆疊和安全態勢，以便分類器讀取您上下文中的其他規則。與其他兩種不同，上下文槽位沒有針對它們的規則。每個預設為 `None configured` 或預設為旁邊命名的保守假設：
  * **組織**
  * **Claude Code 的主要用途**：預設為軟體開發
  * **雲端提供者**
  * **儲存庫可見性**：除非其遠端主機和名稱另有指示，或工作階段中較早的可見性檢查分類器讀取顯示它是公開的。分類器讀取您的訊息和 Claude 執行的命令，而不是它們的輸出，因此證據必須是它能讀取的東西，例如您自己的訊息將儲存庫命名為公開；單獨執行 `gh repo view` 的輸出無法到達它。成績單證據檢查需要 Claude Code v2.1.200 或更新版本
  * **內部共享 / 程式碼片段託管**：公開貼上和 gist 服務被視為在信任邊界外，直到您命名一個
  * **組織特定的 CLI**
  * **祕密管理**
  * **預設 / 受保護的分支**：`main` 和 `master` 被視為受保護，直到您命名其他分支
  * **CI/CD 部署目標**
  * **網路態勢**
  * **受保護的部署命名空間 / 環境**：回退到敏感遠端目標啟發式方法，直到您命名它們
  * **資料保留 / 解密**
* **信任槽位**：命名分類器視為在您邊界內的內容。槽位是受信任儲存庫、原始碼控制、受信任內部網域、受信任雲端儲存桶、關鍵內部服務和內部套件登錄。儲存庫和原始碼控制項目預設為工作儲存庫及其設定的遠端。所有其他信任槽位預設為 `None configured`，因此在您新增之前沒有其他內容是受信任的。儲存庫的可見性僅限於機密材料：私人儲存庫是機密材料的可接受目的地，但將儲存庫設為私人永遠不會清除祕密或個人或受信任的資料到其中，分類器將從工作儲存庫外部移植、重新指向或首次讀取的內容視為不是該儲存庫自己的工作。此範圍設定需要 Claude Code v2.1.203 或更新版本。
* **敏感度槽位**：命名保護規則視為高風險的內容。槽位是敏感資料位置與受眾、敏感遠端目標和受保護的 IaC 範圍。每個預設為廣泛的啟發式方法，例如將任何名稱包含 `prod` 或 `production` 的主機或命名空間視為敏感遠端目標，因此保護規則在您設定任何內容之前就處於活動狀態。在敏感度槽位中命名具體目標會使這些規則應用於命名的目標而不是啟發式方法。

要在預設值旁邊新增您自己的項目，請在陣列中包含字面字串 `"$defaults"`。預設項目會在該位置被插入，因此您的自訂項目可以在它們之前或之後。

下列範例保留預設項目並新增組織的儲存庫、儲存桶、網域和服務。

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Source control: github.example.com/acme-corp and all repos under it",
      "Trusted cloud buckets: s3://acme-build-artifacts, gs://acme-ml-datasets",
      "Trusted internal domains: *.corp.example.com, api.internal.example.com",
      "Key internal services: Jenkins at ci.example.com, Artifactory at artifacts.example.com"
    ]
  }
}
```

項目是散文，不是正規表達式或工具模式。分類器將它們讀取為自然語言規則。按照您向新工程師描述基礎設施的方式編寫它們。徹底的環境部分涵蓋：

* **組織**：您的公司名稱以及 Claude Code 的主要用途，例如軟體開發、基礎設施自動化或資料工程
* **原始碼控制**：您的開發人員推送到的每個 GitHub、GitLab 或 Bitbucket 組織
* **雲端提供者和受信任的儲存桶**：Claude 應該能夠讀取和寫入的儲存桶名稱或前綴
* **受信任的內部網域**：您網路內的 API、儀表板和服務的主機名稱，例如 `*.internal.example.com`
* **關鍵內部服務**：CI、工件登錄、內部套件索引、事件工具
* **內部套件登錄**：私人 npm、PyPI 或其他登錄，安裝應該透過它路由，因此繞過它以使用公開登錄的安裝會被阻止
* **敏感資料位置與受眾**：保存個人資料、機密業務資料、認證、受管制資料或類似敏感材料的儲存桶、資料庫或路徑，以及每個位置中的資料可能被共享的受眾，因此分類器保護這些位置而不是從內容猜測。Claude Code v2.1.195 至 v2.1.197 將此項目命名為 PII / 受管制資料位置，並且僅涵蓋保存個人或受管制資料的位置，不包括受眾維度
* **敏感遠端目標**：計為生產的命名空間、主機或容器，因此遠端 shell 和連接埠轉發到它們需要您的明確批准
* **受保護的 IaC 範圍**：其應用或銷毀應始終要求您命名變更的基礎設施資源
* **其他上下文**：受管制行業的限制、多租戶基礎設施或影響分類器應將什麼視為風險的合規要求

內部套件登錄、敏感資料位置與受眾、敏感遠端目標和受保護的 IaC 範圍項目需要 Claude Code v2.1.195 或更新版本。較早的版本仍將它們讀取為純上下文，但沒有針對它們的內建規則。

一個有用的起始範本：填入括號中的欄位並移除任何不適用的行。

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Organization: {COMPANY_NAME}. Primary use: {PRIMARY_USE_CASE, e.g. software development, infrastructure automation}",
      "Source control: {SOURCE_CONTROL, e.g. GitHub org github.example.com/acme-corp}",
      "Cloud provider(s): {CLOUD_PROVIDERS, e.g. AWS, GCP, Azure}",
      "Trusted cloud buckets: {TRUSTED_BUCKETS, e.g. s3://acme-builds, gs://acme-datasets}",
      "Trusted internal domains: {TRUSTED_DOMAINS, e.g. *.internal.example.com, api.example.com}",
      "Key internal services: {SERVICES, e.g. Jenkins at ci.example.com, Artifactory at artifacts.example.com}",
      "Additional context: {EXTRA, e.g. regulated industry, multi-tenant infrastructure, compliance requirements}"
    ]
  }
}
```

您提供的上下文越具體，分類器就越能區分常規內部操作和資料外洩嘗試。

您不需要一次性填入所有內容。合理的推出：從預設值開始，新增您的原始碼控制組織和關鍵內部服務，這解決了最常見的誤報，例如推送到您自己的儲存庫。接下來新增受信任的網域和雲端儲存桶。隨著阻止出現，填入其餘部分。

<h2 id="override-the-block-and-allow-rules">
  覆蓋阻止和允許規則
</h2>

三個額外的欄位讓您取代分類器的內建規則清單：

* `autoMode.hard_deny`：無條件安全邊界
* `autoMode.soft_deny`：使用者意圖可以清除的破壞性操作
* `autoMode.allow`：軟阻止規則的例外

每個都是散文描述的陣列，讀取為自然語言規則。對於在分類器之前執行的工具模式型硬阻止，請使用 [`permissions.deny`](/docs/zh-TW/permissions)。

在分類器內，優先順序分為四個層級：

* `hard_deny` 規則無條件阻止。使用者意圖和 `allow` 例外不適用。
* `soft_deny` 規則接著阻止。使用者意圖和 `allow` 例外可以覆蓋這些。
* `allow` 規則然後覆蓋匹配的 `soft_deny` 規則作為例外。
* 明確的使用者意圖覆蓋剩餘的軟阻止：如果使用者的訊息直接且具體地描述 Claude 即將採取的確切操作，分類器允許它，即使 `soft_deny` 規則匹配。

一般請求不算作明確意圖。要求 Claude「清理儲存庫」不授權強制推送，但要求 Claude「強制推送此分支」則授權。

要放寬，當分類器重複標記預設例外不涵蓋的常規模式時，新增到 `allow`。要加強，對於預設值遺漏的特定於您環境的破壞性風險，新增到 `soft_deny`，或對於必須永遠不能跨越的安全邊界，新增到 `hard_deny`。

要保留內建規則同時新增您自己的規則，請在陣列中包含字面字串 `"$defaults"`。預設規則會在該位置拼接，因此您的自訂規則可以在它們之前或之後，並且當內建清單在版本發佈中變更時，您繼續繼承更新。

下列範例在所有四個清單中保留預設值，並將組織特定的規則新增到每個清單。

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Source control: github.example.com/acme-corp and all repos under it"
    ],
    "allow": [
      "$defaults",
      "Deploying to the staging namespace is allowed: staging is isolated from production and resets nightly",
      "Writing to s3://acme-scratch/ is allowed: ephemeral bucket with a 7-day lifecycle policy"
    ],
    "soft_deny": [
      "$defaults",
      "Never run database migrations outside the migrations CLI, even against dev databases",
      "Never modify files under infra/terraform/prod/: production infrastructure changes go through the review workflow"
    ],
    "hard_deny": [
      "$defaults",
      "Never send repository contents to third-party code-review APIs"
    ]
  }
}
```

<Danger>
  設定 `environment`、`allow`、`soft_deny` 或 `hard_deny` 中的任何一個而不包含 `"$defaults"` 會取代該部分的整個預設清單。如果您設定沒有 `"$defaults"` 的陣列，您會捨棄該部分的內建規則：

  * `soft_deny`：每個內建軟阻止規則，包括強制推送、`curl | bash`、生產部署和自動模式繞過
  * `hard_deny`：內建資料外洩規則
</Danger>

每個部分獨立評估，因此單獨設定 `environment` 會保持預設 `allow`、`soft_deny` 和 `hard_deny` 清單完整。

只在您打算完全掌控清單時才省略 `"$defaults"`。要安全地執行此操作，執行 `claude auto-mode defaults` 列印內建規則，將它們複製到您的設定檔案中，然後根據您自己的管道和風險容限檢查每個規則。

<h2 id="route-all-shell-commands-through-the-classifier">
  透過分類器路由所有 shell 命令
</h2>

根據預設，狹隘的 Bash 和 PowerShell 允許規則，例如 `Bash(npm test)` 會進入自動模式並在分類器執行之前解決。自動模式只暫停授予任意程式碼執行的廣泛規則，例如 `Bash(*)` 或萬用字元解釋器。這意味著狹隘的規則仍然可以讓破壞性引數通過而不讓分類器看到它，例如規則的前綴未預期的指令碼路徑或旗標。

將 `autoMode.classifyAllShell` 設定為 `true` 以在自動模式處於活動狀態時暫停每個 Bash 和 PowerShell 允許規則，以便分類器評估每個 shell 命令，無論您的允許清單如何。

```json theme={null}
{
  "autoMode": {
    "classifyAllShell": true
  }
}
```

這用延遲換取涵蓋範圍：允許規則會立即批准的命令現在等待分類器決定，每個 shell 命令計為一個分類器呼叫。

該設定僅在自動模式處於活動狀態時適用，您的允許規則在其他權限模式中正常運作。

<Note>
  `autoMode.classifyAllShell` 需要 Claude Code v2.1.193 或更新版本。較早的版本忽略該鍵並繼續將狹隘的 shell 允許規則進入自動模式。
</Note>

<h2 id="inspect-the-defaults-and-your-effective-config">
  檢查預設值和您的有效設定
</h2>

三個 CLI 子命令幫助您檢查和驗證您的設定。

將內建 `environment`、`allow`、`soft_deny` 和 `hard_deny` 規則列印為 JSON：

```bash theme={null}
claude auto-mode defaults
```

若要讀取一個規則的完整措辭而不透過 `jq` 管道，請傳遞 `--label` 搭配規則標籤的開頭，例如 `claude auto-mode defaults --label 'Git Destructive'`。比對是對每個規則標籤的不區分大小寫前綴，沒有比對的部分會列印為空清單。需要 Claude Code v2.1.208 或更新版本。

列印分類器實際使用的內容為 JSON，在設定的地方應用您的設定，否則使用預設值：

```bash theme={null}
claude auto-mode config
```

獲得關於您的自訂 `allow`、`soft_deny` 和 `hard_deny` 規則的 AI 反饋：

```bash theme={null}
claude auto-mode critique
```

在儲存設定後執行 `claude auto-mode config` 以確認有效規則是您期望的，並且 `"$defaults"` 已展開到位。如果您已編寫自訂規則，`claude auto-mode critique` 會檢查它們並標記模糊、冗餘或可能導致誤報的項目。

如果您需要移除或重寫內建規則而不是在其旁邊新增，請將 `claude auto-mode defaults` 的輸出儲存到檔案，編輯清單，並將結果貼到您的設定檔案中以取代 `"$defaults"`。

<h2 id="review-denials">
  檢查拒絕
</h2>

當自動模式拒絕工具呼叫時，拒絕會記錄在 `/permissions` 下的「最近拒絕」標籤中。在拒絕的操作上按 `r` 將其標記為重試：當您退出對話框時，Claude Code 會傳送一條訊息告訴模型它可能重試該工具呼叫並繼續對話。

在 Claude Code v2.1.193 及更新版本中，分類器對每個拒絕的原因會出現在文字記錄中被阻止的工具呼叫旁邊、拒絕通知中以及「最近拒絕」標籤上的每個項目下。使用原因來決定修復是 `environment` 項目、`allow` 例外還是在您的下一條訊息中使用明確意圖重試。

對同一目的地的重複拒絕通常意味著分類器缺少上下文。將該目的地新增到 `autoMode.environment`，然後執行 `claude auto-mode config` 確認它生效。

要以程式設計方式對拒絕做出反應，請使用 [`PermissionDenied` hook](/docs/zh-TW/hooks#permissiondenied)。

<h2 id="see-also">
  另請參閱
</h2>

* [權限模式](/docs/zh-TW/permission-modes#eliminate-prompts-with-auto-mode)：自動模式是什麼、它預設阻止什麼以及如何啟用它
* [受管設定](/docs/zh-TW/server-managed-settings)：在您的組織中部署 `autoMode` 設定
* [權限](/docs/zh-TW/permissions)：在分類器執行之前應用的允許、詢問和拒絕規則
* [設定](/docs/zh-TW/settings)：完整的設定參考，包括 `autoMode` 鍵
