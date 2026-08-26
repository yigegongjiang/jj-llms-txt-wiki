> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 選擇權限模式

> 控制 Claude 在編輯檔案或執行命令前是否詢問。在 CLI 中使用 Shift+Tab 循環切換模式，或在 VS Code、Desktop 和 claude.ai 中使用模式選擇器。

當 Claude 想要編輯檔案、執行 shell 命令或發出網路請求時，它會暫停並要求您批准該操作。權限模式控制暫停發生的頻率。您選擇的模式會影響工作階段的流程：手動模式讓您逐一檢查每個操作，而較寬鬆的模式則讓 Claude 能夠進行較長的不間斷工作，並在完成時回報。針對敏感工作選擇更多監督，或在您信任方向時選擇較少的中斷。

<h2 id="available-modes">
  可用的模式
</h2>

每種模式在便利性和監督之間做出不同的權衡。下表顯示在每種模式中 Claude 無需權限提示即可執行的操作。

| 模式                                                                  | 無需詢問即可執行                                       | 最適合           |
| :------------------------------------------------------------------ | :--------------------------------------------- | :------------ |
| `default`                                                           | 僅讀取                                            | 入門、敏感工作       |
| [`acceptEdits`](#auto-approve-file-edits-with-acceptedits-mode)     | 讀取、檔案編輯和常見的檔案系統命令（`mkdir`、`touch`、`mv`、`cp` 等） | 迭代您正在審查的程式碼   |
| [`plan`](#analyze-before-you-edit-with-plan-mode)                   | 僅讀取                                            | 在變更程式碼前探索程式碼庫 |
| [`auto`](#eliminate-prompts-with-auto-mode)                         | 所有操作，具有背景安全檢查                                  | 長期任務、減少提示疲勞   |
| [`dontAsk`](#allow-only-pre-approved-tools-with-dontask-mode)       | 僅預先批准的工具                                       | 鎖定的 CI 和指令碼   |
| [`bypassPermissions`](#skip-all-checks-with-bypasspermissions-mode) | 所有操作                                           | 僅限隔離的容器和虛擬機器  |

在 CLI 中、`claude --help` 中、VS Code 和 JetBrains 擴充功能中以及桌面應用程式中，審查每個操作的模式名為 **Manual**。其設定值為 `default`，這是 hooks 和 SDK 整合使用的值。CLI 接受 `manual` 作為別名，無論您在何處輸入該值，例如 `claude --permission-mode manual` 或 `"defaultMode": "manual"`。Manual 標籤和 `manual` 別名需要 Claude Code v2.1.200 或更新版本。桌面應用程式的標籤不取決於您的 CLI 版本。

在除了 `bypassPermissions` 之外的每種模式中，寫入[受保護的路徑](#protected-paths)永遠不會自動批准，以防止儲存庫狀態和 Claude 自身設定遭到意外損毀。

模式設定基準。在頂部分層[權限規則](/docs/zh-TW/permissions#manage-permissions)以預先批准或阻止特定工具。拒絕規則、明確詢問規則、[連接器工具上的組織 `ask` 設定](/docs/zh-TW/mcp#organization-controls-on-connector-tools)和 [`requiresUserInteraction`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 標記適用於每種模式，包括 `bypassPermissions`。允許規則在該模式中無效，因為其他所有操作都已經被批准。

<h2 id="switch-permission-modes">
  切換權限模式
</h2>

您可以在工作階段中途、啟動時或設定為持久預設值時切換模式。模式是透過這些控制項設定的，而不是透過在聊天中詢問 Claude。請在下方選擇您的介面，以查看如何變更模式。

<Tabs>
  <Tab title="CLI">
    **在工作階段期間**：按 `Shift+Tab` 循環切換 `default` → `acceptEdits` → `plan`。目前模式會顯示在狀態列中。手動模式（該循環中的 `default`）會顯示灰色的 `⏸ manual mode on` 徽章。在 v2.1.203 之前，狀態列在手動模式中不顯示徽章。

    並非每個模式都在預設循環中：

    * `auto`：當您的帳戶符合 [auto 模式要求](#eliminate-prompts-with-auto-mode) 時出現；循環切換到它會在不需要確認提示的情況下切換模式
    * `bypassPermissions`：在您使用 `--permission-mode bypassPermissions`、`--dangerously-skip-permissions` 或 `--allow-dangerously-skip-permissions` 啟動後出現；`--allow-` 變體會將模式新增到循環中而不啟動它
    * `dontAsk`：永遠不會在循環中出現；使用 `--permission-mode dontAsk` 設定它

    啟用的選用模式會在 `plan` 之後插入，`bypassPermissions` 優先，`auto` 最後。如果您同時啟用了兩者，您將在循環到 `auto` 的途中循環通過 `bypassPermissions`。

    **在啟動時**：將模式作為旗標傳遞。

    ```bash theme={null}
    claude --permission-mode plan
    ```

    **作為預設值**：在 [settings](/docs/zh-TW/settings#settings-files) 中設定 `defaultMode`。

    ```json theme={null}
    {
      "permissions": {
        "defaultMode": "acceptEdits"
      }
    }
    ```

    相同的 `--permission-mode` 旗標適用於 `-p` 用於 [非互動式執行](/docs/zh-TW/headless)。
  </Tab>

  <Tab title="VS Code">
    **在工作階段期間**：點擊提示框底部的模式指示器。

    **作為預設值**：在 VS Code 設定中設定 `claudeCode.initialPermissionMode`，或使用 Claude Code 擴充功能設定面板。

    模式指示器顯示這些標籤，對應到每個標籤適用的模式：

    | UI 標籤              | 模式                  |
    | :----------------- | :------------------ |
    | Manual             | `default`           |
    | Edit automatically | `acceptEdits`       |
    | Plan               | `plan`              |
    | Auto               | `auto`              |
    | Bypass permissions | `bypassPermissions` |

    在 v2.1.205 之前，擴充功能將 `plan` 標記為 Plan mode，將 `auto` 標記為 Auto mode。

    當您的帳戶符合 [auto 模式部分](#eliminate-prompts-with-auto-mode) 中列出的每項要求時，Auto 模式會在模式指示器中出現。`claudeCode.initialPermissionMode` 設定不接受 `auto`。若要預設以 auto 模式啟動，請改為在您的 [使用者設定](/docs/zh-TW/settings#settings-files) 中設定 `defaultMode`。Claude Code 會忽略專案和本機設定中的 `defaultMode: "auto"`。

    略過權限需要擴充功能設定中的 **Allow dangerously skip permissions** 切換，才能在模式指示器中出現。

    請參閱 [VS Code 指南](/docs/zh-TW/vs-code) 以取得擴充功能特定的詳細資訊。
  </Tab>

  <Tab title="JetBrains">
    JetBrains 外掛程式在 IDE 終端中執行 Claude Code，因此切換模式的方式與 CLI 中相同：按 `Shift+Tab` 循環切換，或在啟動時傳遞 `--permission-mode`。
  </Tab>

  <Tab title="Desktop">
    **在工作階段期間**：使用傳送按鈕旁邊的模式選擇器。並非每個模式都會在選擇器中出現：

    * **Auto**：當您的帳戶符合 [auto 模式要求](#eliminate-prompts-with-auto-mode) 時出現
    * **Bypass permissions**：在 Pro 和 Max 方案上需要桌面設定中的 **Allow bypass permissions mode** 切換；在 Team 和 Enterprise 方案上，組織政策會改為控制它

    如需桌面特定的詳細資訊，請參閱桌面指南中的 [選擇權限模式](/docs/zh-TW/desktop#choose-a-permission-mode)。

    **作為預設值**：在 [settings](/docs/zh-TW/settings#settings-files) 中設定 `defaultMode`。桌面應用程式讀取與 CLI 相同的設定檔，並將模式套用到新的本機工作階段。

    您在模式選擇器中選擇的模式會按資料夾記住，並優先於該資料夾的 `defaultMode`。Plan 是例外：選擇它只會套用到目前工作階段。

    此範例將 Plan 模式設定為新本機工作階段的預設值：

    ```json theme={null}
    {
      "permissions": {
        "defaultMode": "plan"
      }
    }
    ```
  </Tab>

  <Tab title="Web and mobile">
    在 [claude.ai/code](https://claude.ai/code) 或行動應用程式中使用提示框旁邊的模式下拉式選單。權限提示會在 claude.ai 中出現以供核准。出現的模式取決於工作階段在何處執行：

    * **Cloud sessions** 在 [Claude Code on the web](/docs/zh-TW/claude-code-on-the-web) 上：接受編輯、Plan 和 Auto。接受編輯對應於 `default` 模式：雲端環境預先核准檔案編輯，無論模式為何，因此下拉式選單會顯示接受編輯而不是手動。雲端工作階段仍然遵守設定中的 `defaultMode: "acceptEdits"`。Auto 模式僅在您的組織允許且選定的模型支援時出現。略過權限不可用。
    * **[Remote Control](/docs/zh-TW/remote-control) sessions** 在您的本機機器上：手動、接受編輯和 Plan。您無法從應用程式選擇 Auto 或 Bypass permissions。下拉式選單顯示本機工作階段所在的模式，包括從終端設定的模式，並在應用程式或終端中模式變更時更新。唯一的例外是 Bypass permissions：工作階段永遠不會向 claude.ai 報告該模式，因此從終端切換到它不會變更下拉式選單顯示的內容。在 v2.1.202 之前，使用 `/remote-control` 或 `claude --remote-control` 連線的工作階段根本不報告其模式，因此 claude.ai 和行動應用程式可能會顯示工作階段不在的模式。不匹配只影響標籤：Claude Code 從工作階段的實際模式產生權限提示，它們仍然在應用程式中出現以供核准。

    對於 Remote Control，您也可以在啟動主機時設定起始模式：

    ```bash theme={null}
    claude remote-control --permission-mode acceptEdits
    ```
  </Tab>
</Tabs>

<h2 id="auto-approve-file-edits-with-acceptedits-mode">
  使用 acceptEdits 模式自動批准檔案編輯
</h2>

`acceptEdits` 模式讓 Claude 在您的工作目錄中建立和編輯檔案，無需提示。當此模式處於活動狀態時，狀態列會顯示 `⏵⏵ accept edits on`。

除了檔案編輯外，`acceptEdits` 模式還會自動批准常見的檔案系統 Bash 命令：`mkdir`、`touch`、`rm`、`rmdir`、`mv`、`cp` 和 `sed`。當這些命令以安全環境變數（例如 `LANG=C` 或 `NO_COLOR=1`）或程序包裝器（例如 `timeout`、`nice` 或 `nohup`）作為前綴時，也會自動批准。與檔案編輯一樣，自動批准僅適用於工作目錄或 `additionalDirectories` 內的路徑。超出該範圍的路徑、寫入[受保護路徑](#protected-paths)以及所有其他 Bash 命令（除了[內建唯讀集合](/docs/zh-TW/permissions#read-only-commands)）仍會提示。

當[PowerShell 工具](/docs/zh-TW/tools-reference#powershell-tool)啟用時，`acceptEdits` 模式也會自動批准 `Set-Content`、`Add-Content`、`Clear-Content` 和 `Remove-Item` 在範圍內的路徑上，以及它們的常見別名。相同的範圍和受保護路徑規則適用。

當您想在編輯器中或透過 `git diff` 事後檢查變更，而不是逐個批准每個編輯時，請使用 `acceptEdits`。

從手動模式按一次 `Shift+Tab` 進入它，或直接啟動它：

```bash theme={null}
claude --permission-mode acceptEdits
```

<h2 id="analyze-before-you-edit-with-plan-mode">
  使用計畫模式在編輯前進行分析
</h2>

計畫模式會告訴 Claude 在進行變更前先研究並提出建議。Claude 會讀取檔案、執行 shell 命令進行探索，並撰寫計畫，但不會編輯您的原始碼。權限提示的應用方式與手動模式相同，除非 [自動模式](/docs/zh-TW/auto-mode-config) 可用且 `useAutoModeDuringPlan` 已開啟（預設為開啟）。啟用自動模式後，分類器會核准搜尋和檔案讀取等唯讀命令，無需提示。無論如何，編輯都會保持被阻止，直到您核准計畫為止。

按下 `Shift+Tab` 或在單一提示前加上 `/plan` 即可進入計畫模式。您也可以從 CLI 開始使用計畫模式：

```bash theme={null}
claude --permission-mode plan
```

再次按下 `Shift+Tab` 即可在不核准計畫的情況下離開計畫模式。

<h3 id="review-and-approve-a-plan">
  檢視並核准計畫
</h3>

計畫準備好後，Claude 會呈現計畫並詢問如何進行。從該提示中，您可以：

* 核准並在自動模式中開始
* 核准並接受編輯
* 核准並手動檢視每項編輯
* 透過回饋繼續規劃
* 使用 [Ultraplan](/docs/zh-TW/ultraplan) 進行瀏覽器型檢視以進行精煉

核准計畫會退出計畫模式，並將工作階段切換至每個核准選項所描述的權限模式，以便 Claude 開始編輯。若要再次規劃，請使用 `Shift+Tab` 循環回到計畫模式，或在下一個提示前加上 `/plan`。

按下 `Ctrl+G` 即可在預設文字編輯器中開啟提議的計畫並直接編輯，然後 Claude 才會繼續進行。當啟用 [`showClearContextOnPlanAccept`](/docs/zh-TW/settings#available-settings) 時，每個核准選項也會提供在核准計畫前清除規劃上下文的選項。

接受計畫也會根據計畫內容自動為工作階段命名，除非您已使用 `--name` 或 `/rename` 設定名稱。

<h3 id="set-plan-mode-as-the-default">
  將計畫模式設定為預設值
</h3>

若要將計畫模式設定為專案的預設值，請在 `.claude/settings.json` 中設定 `defaultMode`：

```json theme={null}
{
  "permissions": {
    "defaultMode": "plan"
  }
}
```

<h2 id="eliminate-prompts-with-auto-mode">
  使用自動模式消除權限提示
</h2>

自動模式讓 Claude 無需例行權限提示即可執行。一個獨立的分類器模型在操作執行前進行審查，阻止任何超出您請求範圍、針對無法識別的基礎設施或似乎由 Claude 讀取的惡意內容驅動的操作。明確的[詢問規則](/docs/zh-TW/permissions#manage-permissions)仍會強制提示。

針對檔案系統根目錄或主目錄的移除，例如 `rm -rf /` 和 `rm -rf ~`，會提示批准而不是進入分類器。當命令包含使用 `$(...)` 或反引號的命令替換，或使用 `<(...)` 的程序替換時，此提示也會觸發，無論移除是在替換內部（如 `echo "$(rm -rf ~)"`），還是在同一命令的其他地方。在 v2.1.208 之前，包含這些形式的命令進入分類器而不是提示。

自動模式也會促使 Claude 繼續工作而不停下來提出澄清問題，儘管當您的提示或技能明確依賴時 Claude 仍會詢問。為了在保持權限提示的同時獲得更強的自主行為，請改為設定[主動輸出風格](/docs/zh-TW/output-styles)。

<Warning>
  自動模式減少了權限提示，但不保證安全性。將其用於您信任一般方向的任務，而不是作為敏感操作審查的替代品。
</Warning>

自動模式僅在您的帳戶滿足以下所有要求時才可用：

* **方案**：所有方案。
* **擁有者**：在 Team 和 Enterprise 上，擁有者必須在 [Claude Code 管理員設定](https://claude.ai/admin-settings/claude-code)中啟用它，使用者才能開啟。管理員也可以通過在[受管設定](/docs/zh-TW/permissions#managed-settings)中將 `permissions.disableAutoMode` 設定為 `"disable"` 來關閉自動模式。對於桌面應用程式的 Code 標籤，`disableAutoMode` 是組織級控制，管理員設定切換不適用。
* **模型**：在 Anthropic API 上，Claude Opus 4.6 或更新版本，或 Sonnet 4.6 或更新版本。在 Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 和已登入的 [Claude 應用程式閘道](/docs/zh-TW/claude-apps-gateway)工作階段上，僅支援 Claude Sonnet 5、Opus 4.7 和 Opus 4.8。較舊的模型，包括 Sonnet 4.5、Opus 4.5、Haiku 和 claude-3 模型，在任何提供者上都不受支援。
* **提供者**：在 Anthropic API、Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 和已登入的 Claude 應用程式閘道工作階段上預設可用。在 v2.1.158 到 v2.1.206 中，自動模式在除了 Anthropic API 之外的所有這些提供者上都是關閉的，直到您設定 `CLAUDE_CODE_ENABLE_AUTO_MODE=1`；v2.1.207 移除了該要求。

如果 Claude Code 報告自動模式不可用，則其中一項要求未滿足；這不是暫時性中斷。一個單獨的訊息，命名一個模型並說自動模式「無法確定」操作的安全性，是暫時性分類器中斷；請參閱[錯誤參考](/docs/zh-TW/errors#auto-mode-cannot-determine-the-safety-of-an-action)。

如果您在[設定](/docs/zh-TW/settings#available-settings)中設定 `defaultMode: "auto"`，且工作階段以 `default` 模式啟動且沒有錯誤，該設定可能在 `.claude/settings.json` 或 `.claude/settings.local.json` 中。Claude Code v2.1.142 及更新版本會忽略來自這些檔案的 `auto`，因此儲存庫無法授予自己自動模式。將其移至 `~/.claude/settings.json`。

<h3 id="enable-auto-mode-on-bedrock-agent-platform-or-foundry">
  Bedrock、Agent Platform 或 Foundry 上的自動模式
</h3>

在 [Amazon Bedrock](/docs/zh-TW/amazon-bedrock)、[Google Cloud 的 Agent Platform](/docs/zh-TW/google-vertex-ai)、[Microsoft Foundry](/docs/zh-TW/microsoft-foundry) 和已登入的 [Claude 應用程式閘道](/docs/zh-TW/claude-apps-gateway)工作階段上，自動模式預設會出現在 `Shift+Tab` 循環中。出現在循環中不會改變工作階段啟動的模式：工作階段仍會以您的 [`defaultMode`](/docs/zh-TW/settings#available-settings)啟動，除非您更改，否則為 Manual。這些提供者上僅支援 Claude Sonnet 5、Opus 4.7 和 Opus 4.8。

要使自動模式成為預設啟動模式，請在使用者或受管設定中設定 `"permissions": {"defaultMode": "auto"}`。

要防止開發人員使用自動模式，請在[受管設定](/docs/zh-TW/permissions#managed-settings)中將 `disableAutoMode` 設定為 `"disable"`。這會從 `Shift+Tab` 循環中移除 `auto`，並在啟動時拒絕 `--permission-mode auto`。

在 v2.1.158 到 v2.1.206 中，自動模式在這些提供者上是關閉的，直到您設定 `CLAUDE_CODE_ENABLE_AUTO_MODE=1`，且 Claude Code 在這些提供者上忽略 `defaultMode: "auto"`，除非也設定了該變數。該變數仍被接受以保持相容性，從 v2.1.207 開始沒有效果。

<h3 id="what-the-classifier-blocks-by-default">
  分類器預設阻止的內容
</h3>

分類器信任您的工作目錄和為其配置的遠端，這些遠端是在工作階段啟動時配置的。使用 `git remote add` 或 `git remote set-url` 在工作階段期間新增或重新指向的遠端不受信任，其他所有內容都被視為外部，直到您[配置受信任的基礎設施](/docs/zh-TW/auto-mode-config)。在 v2.1.200 之前，中途新增的遠端也受信任。

**預設阻止**：

* 下載並執行程式碼，例如 `curl | bash`
* 將敏感資料傳送到外部端點
* 生產部署和遷移
* 雲端儲存上的大量刪除
* 授予 IAM 或儲存庫權限
* 修改共享基礎設施
* 不可逆地銷毀工作階段前存在的檔案
* 強制推送
* 當推送包含敏感內容（如祕密或個人或受託資料）、包含相對於您要求的隱藏或誤述的變更、包含從儲存庫外部移植或首次讀取的內容，或繞過您要求的拉取請求、審查或檢查時，推送到儲存庫的預設分支。純粹推送到預設分支本身不會被阻止，清除標記的推送需要命名標記的內容或繞過的審查，而不僅僅是推送。分類器是一層：[`permissions.deny` 規則](/docs/zh-TW/permissions#manage-permissions)適用於每種模式，可以完全阻止推送到預設分支，遠端自己的分支保護仍然適用。在 v2.1.203 之前，任何直接推送到預設分支都被阻止
* `git reset --hard`、`git checkout -- .`、`git restore .`、`git clean -fd`、`git stash drop` 或 `git stash clear`，分類器假設會丟棄未提交的變更
* 當 HEAD 的提交不是在此工作階段中建立時的 `git commit --amend`
* 從 v2.1.198 開始，當 HEAD 的提交已經被推送時的 `git commit --amend`。僅訊息重述不被阻止：`--amend -m` 沒有新暫存的內容，在 Claude 在此工作階段期間建立的提交上
* `terraform destroy`、`pulumi destroy`、`cdk destroy` 或 `terragrunt destroy`，以及應用銷毀資源的計畫

Claude Code v2.1.195 及更新版本預設阻止更多類別。有些取決於[環境](/docs/zh-TW/auto-mode-config#define-trusted-infrastructure)條目，例如敏感遠端目標和受保護的 IaC 範圍，您可以將其縮小到具體名稱。

* 寫入祕密管理器，或更改 DNS 記錄或 TLS 憑證
* 合併沒有人類批准的拉取請求、批准 Claude 自己的拉取請求或禁用 CI 檢查
* 發佈本身是自動化命令的評論，例如 `atlantis apply` 或機器人的 `/deploy` 或 `/merge`
* 切換、調整或刪除生產功能標誌
* 將基礎設施變更應用於受保護的 IaC 範圍，或排空並移除叢集節點
* 寫入超出您命名的資源的共享計算叢集，例如標籤選擇器或 `--all` 捕捉其他使用者的工作
* 建立在每個節點上執行或攔截叢集流量的 Kubernetes 資源，例如 DaemonSets 和准入 webhooks
* 互動式 shell 或埠轉發到敏感遠端目標
* 開啟隧道或反向 shell，使本地服務可從公共網際網路存取
* 將即時認證或令牌列印到文字記錄或檔案
* 存取在您的[環境](/docs/zh-TW/auto-mode-config#define-trusted-infrastructure)中列為敏感資料位置的位置，或從中複製資料。從 v2.1.198 開始，這也會阻止從一個位置向該條目排除的受眾傳送資料
* 繞過您的內部套件登錄將套件安裝路由到公共登錄。從 v2.1.198 開始，這也適用於您在對話中告訴 Claude 內部登錄或鏡像存在的情況，而不僅僅是在您的環境中列出的情況
* 使用禁用安全防護的標誌執行命令，例如 `--insecure`
* 啟動在沒有人類批准或沙箱的情況下執行的自主代理迴圈，例如使用 `--dangerously-skip-permissions` 或 `--no-sandbox` 啟動的迴圈。從 v2.1.198 開始，這也涵蓋執行禁用隔離和每操作批准的第三方代理或評估工具，例如使用 `--yes-always` 啟動的執行器
* [Chrome 中的 Claude](/docs/zh-TW/chrome)瀏覽器操作，可能會將頁面內容、Cookie 或認證傳送到跨源

Claude Code v2.1.198 及更新版本也預設阻止這些：

* 通過萬用字元、glob 或年齡篩選器而不是特定命名路徑刪除 `/tmp`、`$TMPDIR` 或其他共享暫存或快取目錄中的檔案
* 當您自己的訊息未授權這些詳細資訊給該收件人時，在傳送、上傳、發佈或寫入其他人或共享系統的內容中包含敏感詳細資訊。當儲存庫在信任邊界外或公開時，PR 和問題正文、提交訊息和評論算作這種類型的出站內容，包括您組織自己的公開儲存庫；內部檔案路徑、代碼名稱、即時 API 回應資料（如電子郵件或帳戶識別碼）和基礎設施識別碼算作敏感詳細資訊。PR、問題和提交訊息範圍需要 Claude Code v2.1.200 或更新版本。PR 或問題正文中的即時個人資料（如電子郵件地址、帳戶或組織識別碼或使用指標）需要您命名這些詳細資訊和收件人，無論儲存庫的可見性或信任邊界如何。該檢查需要 Claude Code v2.1.203 或更新版本
* 向 Claude Code 自己的 tmux 窗格傳送按鍵以驅動其自己的介面，分類器將其視為 Claude 更改自己的權限或監督

Claude Code v2.1.200 及更新版本也預設阻止這些：

* 註解掉、刪除或強制通過保護安全行為的測試或斷言，例如驗證、存取控制、輸入驗證或沙箱
* 刪除或拆除 Claude 在工作階段中未建立的有狀態資源，當沒有更具體的刪除規則適用且您未命名該資源時
* 將 API 基礎 URL、代理端點、webhook 接收器或登錄鏡像重新指向不適合任務的第三方主機，包括在 `.env.example` 等範例檔案中
* 使用 `git remote set-url` 或 `git remote add` 更改推送的去向，除非您命名了新遠端
* 推送祕密或個人或受託資料到已知為公開的儲存庫，或推送不是該儲存庫自己工作一部分的機密材料。dotfiles 儲存庫自己的主題是個人或受託資料的唯一例外，來自私有儲存庫到任何公開表面的內容以相同方式被阻止；兩項改進都需要 Claude Code v2.1.203 或更新版本。在 v2.1.203 之前，個人資料與機密材料分組，僅當它不是該儲存庫自己工作的一部分時才被阻止。當儲存庫的可見性未確定時，分類器不會單獨阻止；它改為根據其他規則判斷內容
* 針對不同儲存庫或組織開啟拉取請求、使用 `gh repo fork` 進行分叉或推送到第三方儲存庫，除非您命名了該外部目標

Claude Code v2.1.203 及更新版本也預設阻止這些：

* 來自敏感本地儲存或其名稱、路徑或類型將其標記為敏感的檔案的內容進入提交、推送、PR 或問題文字、gist 或貼上或套件發佈，除非您命名了來源和目的地。工作階段文字記錄和對話日誌、認證和配置點資料夾（如 SSH 金鑰、雲端認證、瀏覽器設定檔和 shell 歷史記錄）以及使用者資料匯出都算作，儲存庫為私有不會清除它

Claude Code v2.1.205 及更新版本也預設阻止這些：

* 寫入 Claude Code 工作階段文字記錄、`~/.claude/projects/` 下的 `.jsonl` 歷史檔案或您配置的配置目錄，無論是直接還是通過 shell 命令。該規則也涵蓋 Claude Code 為其自己的檢查附加到每個文字記錄條目的中繼資料行。文字記錄是 Claude Code 寫入的工作階段狀態，而不是工作檔案，篡改的條目在您恢復工作階段後到達每個後續檢查，因此自動模式作為深度防禦阻止這些寫入。讀取文字記錄不被阻止
* 遞迴強制刪除，例如 `rm -rf "$VAR"` 或 `Remove-Item -Recurse -Force $dir`，其目標是 shell 變數或以其為根的 glob，在分類器看到的對話中的任何地方都未指派。該值僅來自較早的命令輸出，分類器永遠不會收到，因此分類器無法根據其他刪除規則驗證刪除目標。分類器根據設計讀取對話而不是命令輸出，因此它阻止呼叫而不是猜測目標。當您命名被刪除的確切路徑或 Claude 使用寫入命令的已解析文字路徑重新執行刪除時，該阻止會清除。分類器可以解析其目標的刪除不受影響

**預設允許**：

* 您工作目錄中的本地檔案操作
* 安裝在您的鎖定檔案或清單中宣告的依賴項
* 讀取 `.env` 並將認證傳送到其匹配的 API
* 唯讀 HTTP 請求
* 推送到您啟動的分支或 Claude 建立的分支
* 例行推送到儲存庫預設分支。在 v2.1.203 之前，任何直接推送到預設分支都被阻止

Claude Code v2.1.195 及更新版本也預設允許這些：

* 刪除 Claude 在同一工作階段中較早建立的確切工作
* 作為您的任務的一部分讀取、審查或編寫安全相關程式碼、配置和威脅模型
* 在同一多代理工作階段中一起工作的代理之間的訊息
* 將資料傳送到您在 [`environment`](/docs/zh-TW/auto-mode-config#define-trusted-infrastructure)中列出的受信任網域、儲存桶和服務。這僅涵蓋資料流，而不是相同基礎設施上的破壞性或認證操作
* [Chrome 中的 Claude](/docs/zh-TW/chrome)導航到受信任的內部網域、localhost 或您命名的 URL

沙箱網路存取請求通過分類器路由，而不是預設允許。從 v2.1.198 開始，分類器重複使用其對網路主機和埠的判決，而不是在每次連線時重新執行：

* 允許被重複使用，直到新內容進入對話，此時該主機被再次檢查
* 在互動式 CLI 中，拒絕在輪次結束時被丟棄
* 在[非互動式模式](/docs/zh-TW/headless)和 Agent SDK 工作階段中沒有輪次邊界，因此拒絕在執行的其餘部分被重複使用
* 更改您的權限模式或規則會丟棄所有快取判決

執行 `claude auto-mode defaults` 以查看完整規則清單。如果例行操作被阻止，管理員可以通過 `autoMode.environment` 設定新增受信任的儲存庫、儲存桶和服務：請參閱[配置自動模式](/docs/zh-TW/auto-mode-config)。

推送到您的工作分支、進行例行推送到儲存庫預設分支，以及建立與您的請求相符的拉取請求都無需提示即可執行。分類器僅在推送帶有風險時才阻止推送，例如強制推送或繞過您設定的審查的內容。要在保持自動模式的同時要求在這些操作前進行人工檢查點，請新增 `permissions.ask` 規則：請參閱[常見邊界](/docs/zh-TW/auto-mode-config#common-boundaries)。

<h3 id="boundaries-you-state-in-conversation">
  您在對話中陳述的邊界
</h3>

分類器將您在對話中陳述的邊界視為阻止信號。如果您告訴 Claude「不要推送」或「在我審查後再部署」，分類器會阻止匹配的操作，即使預設規則會允許它們。邊界保持有效，直到您在後續訊息中解除它。Claude 自己的判斷條件已滿足不會解除它。

邊界不作為規則儲存。分類器在每次檢查時從文字記錄重新讀取它們，因此如果[上下文壓縮](/docs/zh-TW/costs#reduce-token-usage)移除陳述邊界的訊息，邊界可能會丟失。為了獲得硬保證，請改為新增[拒絕規則](/docs/zh-TW/permissions#permission-rule-syntax)。

<h3 id="when-auto-mode-falls-back">
  自動模式何時回退
</h3>

每個被拒絕的操作都會顯示通知，並在 `/permissions` 下的「最近拒絕」標籤中出現，您可以按 `r` 以手動批准重試它。

如果分類器連續阻止操作 3 次或總共 20 次，自動模式暫停，Claude Code 恢復提示。批准提示的操作會恢復自動模式。這些閾值不可配置。任何允許的操作都會重置連續計數器，而總計數器在工作階段中持續，僅在其自己的限制觸發回退時重置。

在[非互動式模式](/docs/zh-TW/headless)中使用 `-p` 標誌，重複阻止會中止工作階段，因為沒有使用者可提示。

重複阻止通常意味著分類器缺少有關您的基礎設施的上下文。使用 `/feedback` 報告誤報，或讓管理員[配置受信任的基礎設施](/docs/zh-TW/auto-mode-config)。

<AccordionGroup>
  <Accordion title="分類器如何評估操作">
    每個操作都經過固定的決策順序。第一個匹配的步驟獲勝：

    1. 與您的[允許、詢問或拒絕規則](/docs/zh-TW/permissions#manage-permissions)匹配的操作立即解決。寫入[受保護路徑](#protected-paths)即使允許規則匹配也會路由到分類器。您的組織設定為 `ask` 的[連接器工具](/docs/zh-TW/mcp#organization-controls-on-connector-tools)和標記為 [`requiresUserInteraction`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 的 MCP 工具即使允許規則匹配也會直接提示您。內容範圍的詢問規則回退到權限提示
    2. 唯讀操作和工作目錄中的檔案編輯會自動批准，除了[受保護路徑](#protected-paths)的寫入
    3. 其他所有內容都進入分類器。您的組織設定為 `ask` 的[連接器工具](/docs/zh-TW/mcp#organization-controls-on-connector-tools)跳過分類器並直接提示您，因此組織要求的批准永遠不會自動批准。從 v2.1.199 開始，標記有 [`_meta["anthropic/requiresUserInteraction"]`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 的 MCP 工具也跳過分類器並直接提示您，因此同意步驟永遠不會代表工具作者自動批准
    4. 如果分類器阻止，Claude 會收到原因並嘗試替代方案

    進入自動模式時，授予任意程式碼執行的廣泛允許規則被丟棄：

    * 全面 `Bash(*)` 或 `PowerShell(*)`
    * 萬用字元解釋器，例如 `Bash(python*)`
    * 套件管理器執行命令
    * `Agent` 允許規則

    狹義規則，例如 `Bash(npm test)` 會保留。丟棄的規則在您離開自動模式時恢復。

    分類器看到使用者訊息、工具呼叫和您的 CLAUDE.md 內容。工具結果被剝離，因此檔案或網頁中的惡意內容無法直接操縱它。一個單獨的伺服器端探針掃描傳入的工具結果，並在 Claude 讀取之前標記可疑內容。有關這些層如何協同工作的更多資訊，請參閱[自動模式公告](https://claude.com/blog/auto-mode)和[工程深度探討](https://www.anthropic.com/engineering/claude-code-auto-mode)。
  </Accordion>

  <Accordion title="自動模式如何處理子代理">
    分類器在三個點檢查[子代理](/docs/zh-TW/sub-agents)工作：

    1. 在子代理啟動之前，委派的任務描述被評估，因此危險看起來的任務在生成時被阻止。
    2. 當子代理執行時，其每個操作都通過分類器，使用與父工作階段相同的規則，子代理前言中的任何 `permissionMode` 都被忽略。
    3. 當子代理完成時，分類器審查其完整操作歷史；如果該返回檢查標記了一個問題，安全警告被前置到子代理的結果。

    步驟 1 需要 Claude Code v2.1.178 或更新版本。較早的版本在步驟 2 和 3 應用分類器，但在子代理啟動前未評估任務描述。
  </Accordion>

  <Accordion title="成本和延遲">
    分類器在獨立於您的 `/model` 選擇的伺服器配置模型上執行，因此切換模型不會改變分類器可用性。分類器呼叫計入您的令牌使用。每次檢查發送文字記錄的一部分加上待執行操作，在執行前新增往返。受保護路徑外的讀取和工作目錄編輯跳過分類器，因此開銷主要來自 shell 命令和網路操作。從 v2.1.198 開始，沙箱網路判決對於主機和埠被重複使用，而不是在每次連線時重新分類，因此重複連線到同一主機不會各自新增檢查。[分類器預設阻止的內容](#what-the-classifier-blocks-by-default)描述允許和拒絕持續多長時間。
  </Accordion>
</AccordionGroup>

<h2 id="allow-only-pre-approved-tools-with-dontask-mode">
  使用 dontAsk 模式僅允許預先核准的工具
</h2>

如果您設定 `dontAsk` 模式，Claude Code 會自動拒絕所有原本會提示的工具呼叫。Claude 只執行符合您的 `permissions.allow` 規則、[唯讀 Bash 命令](/docs/zh-TW/permissions#read-only-commands)的動作，以及由 [PreToolUse hook](/docs/zh-TW/permissions#extend-permissions-with-hooks) 核准的呼叫。在您預先定義 Claude 可以執行的確切操作的 CI 管道或受限環境中使用此模式；工作階段永遠不會等待輸入。此模式啟用時，狀態列會顯示 `⏵⏵ don't ask on`。

Claude Code 會拒絕符合您明確 [`ask` 規則](/docs/zh-TW/permissions#manage-permissions) 的呼叫，而不是提示。它也會拒絕內建的 `AskUserQuestion` 工具和連接器工具[您的組織設定為 `ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools)，即使您的允許規則符合它們。它以相同方式拒絕標記為 [`_meta["anthropic/requiresUserInteraction"]`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 的 MCP 工具，因為它們的核准卡需要此模式永遠不會收集的答案；這需要 Claude Code v2.1.199 或更新版本。

[Claude Code on the web](/docs/zh-TW/claude-code-on-the-web) 上的雲端工作階段會忽略 `defaultMode: "dontAsk"`；詳見 [bypassPermissions](#skip-all-checks-with-bypasspermissions-mode) 以了解詳情。

在啟動時使用旗標設定：

```bash theme={null}
claude --permission-mode dontAsk
```

<h2 id="skip-all-checks-with-bypasspermissions-mode">
  使用 bypassPermissions 模式跳過所有檢查
</h2>

`bypassPermissions` 模式會停用權限提示和安全檢查，以便工具呼叫立即執行，包括寫入[受保護的路徑](#protected-paths)。在 v2.1.126 之前，受保護路徑的寫入在此模式中仍會提示。

明確的[詢問規則](/docs/zh-TW/permissions#manage-permissions)和連接器工具[您的組織設定為 `ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools)仍會在此模式中強制提示。標記有 [`_meta["anthropic/requiresUserInteraction"]`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 的 MCP 工具也仍會提示；這需要 Claude Code v2.1.199 或更新版本。

針對檔案系統根目錄或主目錄的移除操作，例如 `rm -rf /` 和 `rm -rf ~`，仍會作為針對模型錯誤的斷路器而提示。當命令包含使用 `$(...)` 或反引號的命令替換，或使用 `<(...)` 的程序替換時，斷路器也會觸發，無論移除操作位於替換內部（如 `echo "$(rm -rf ~)"`），還是位於同一命令中的其他位置。純形式（作為其自身命令輸入）自引入斷路器以來在此模式中已提示；在 v2.1.208 之前，包含這些形式的命令不會提示。

<Warning>
  僅在隔離環境（例如容器、虛擬機或無網際網路存取的開發容器）中使用此模式，其中 Claude Code 無法損害您的主機系統。
</Warning>

您無法從未使用啟用旗標啟動的工作階段進入 `bypassPermissions`；使用啟用旗標重新啟動以啟用它：

```bash theme={null}
claude --permission-mode bypassPermissions
```

`--dangerously-skip-permissions` 旗標是等效的。

在 Linux 和 macOS 上，當以 root 身份或在 `sudo` 下執行時，Claude Code 拒絕以此模式啟動：

```text theme={null}
--dangerously-skip-permissions cannot be used with root/sudo privileges for security reasons
```

檢查會在識別的沙箱內自動跳過。若要在容器中自主執行，請使用[開發容器](/docs/zh-TW/devcontainer)配置，該配置以非 root 使用者身份執行 Claude Code。

[網路上的 Claude Code](/docs/zh-TW/claude-code-on-the-web) 不會遵守您設定檔案中的 `defaultMode: "bypassPermissions"` 或 `"dontAsk"`，因此儲存庫的簽入設定無法在略過權限模式下啟動雲端工作階段。該設定會被無聲地忽略，工作階段會改為以模式下拉式選單中顯示的模式啟動。請參閱[切換權限模式](#switch-permission-modes)以了解雲端工作階段提供的模式。

<Warning>
  `bypassPermissions` 不提供針對提示注入或意外操作的保護。若要使用背景安全檢查且權限提示大幅減少，請改用[自動模式](#eliminate-prompts-with-auto-mode)。管理員可以透過在[受管設定](/docs/zh-TW/permissions#managed-settings)中將 `permissions.disableBypassPermissionsMode` 設定為 `"disable"` 來封鎖此模式。
</Warning>

<h2 id="protected-paths">
  受保護的路徑
</h2>

對於一小組路徑的寫入操作在除了 `bypassPermissions` 之外的所有模式中都不會自動批准。這可以防止意外損壞儲存庫狀態和 Claude 自身的設定。

| 模式                             | 受保護路徑寫入 |
| :----------------------------- | :------ |
| `default`、`acceptEdits`、`plan` | 提示      |
| `auto`                         | 路由至分類器  |
| `dontAsk`                      | 拒絕      |
| `bypassPermissions`            | 允許      |

設定檔案中的 [`permissions.allow`](/docs/zh-TW/permissions#manage-permissions) 規則不會預先批准受保護路徑的寫入。安全檢查在 Claude Code 評估設定中的允許規則之前執行，因此在 `~/.claude/settings.json` 或 `.claude/settings.json` 中的 `Edit(.claude/**)` 之類的項目不會改變上表中的每個模式結果。在提示的模式中，`.claude/` 寫入的提示會提供**是的，並允許 Claude 在此工作階段編輯其自身設定**，這會在該工作階段中批准後續的 `.claude/` 寫入而無需再次提示。

受保護的目錄：

* `.git`
* `.config/git`
* `.vscode`
* `.idea`
* `.husky`
* `.cargo`
* `.devcontainer`
* `.yarn`
* `.mvn`
* `.claude`，除了 `.claude/worktrees` 其中 Claude 儲存其自身的 git worktrees

受保護的檔案：

* `.gitconfig`、`.gitmodules`
* `.bashrc`、`.bash_profile`、`.bash_login`、`.bash_aliases`、`.bash_logout`、`.zshrc`、`.zprofile`、`.zshenv`、`.zlogin`、`.zlogout`、`.profile`、`.envrc`
* `.npmrc`、`.yarnrc`、`.yarnrc.yml`、`.pnp.cjs`、`.pnp.loader.mjs`、`.pnpmfile.cjs`、`bunfig.toml`、`.bunfig.toml`
* `.bazelrc`、`.bazelversion`、`.bazeliskrc`
* `.pre-commit-config.yaml`、`lefthook.yml`、`lefthook.yaml`、`.lefthook.yml`、`.lefthook.yaml`
* `gradle-wrapper.properties`、`maven-wrapper.properties`
* `.devcontainer.json`
* `.ripgreprc`、`pyrightconfig.json`
* `.mcp.json`、`.claude.json`

<h2 id="see-also">
  另請參閱
</h2>

* [Permissions](/docs/zh-TW/permissions)：allow、ask 和 deny 規則；受管理的原則
* [Configure auto mode](/docs/zh-TW/auto-mode-config)：告訴分類器您的組織信任哪些基礎設施
* [Hooks](/docs/zh-TW/hooks)：透過 `PreToolUse` 和 `PermissionRequest` hooks 的自訂權限邏輯
* [Ultraplan](/docs/zh-TW/ultraplan)：在 Claude Code 網頁工作階段中執行計畫模式，並進行瀏覽器型審查
* [Security](/docs/zh-TW/security)：保護措施和最佳實踐
* [Sandboxing](/docs/zh-TW/sandboxing)：Bash 命令的檔案系統和網路隔離
* [Non-interactive mode](/docs/zh-TW/headless)：使用 `-p` 旗標執行 Claude Code
