> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 在 Claude Code Desktop 中排程定期任務

> 在 Claude Code Desktop 中設定排程任務，以定期自動執行 Claude 進行每日程式碼審查、相依性稽核或早晨簡報。

排程任務會在您選擇的時間和頻率自動啟動新的工作階段。使用它們進行定期工作，例如每日程式碼審查、相依性更新檢查，或從您的日曆和收件匣提取資訊的早晨簡報。

Desktop 應用程式的 **Routines** 頁面可讓您建立本機排程任務和遠端 [routines](/docs/zh-TW/routines)。本機任務在您的機器上執行，可直接存取您的檔案和工具，但只有在應用程式開啟且您的電腦處於喚醒狀態時才會觸發。遠端 routine 在 Anthropic 管理的雲端基礎設施上執行，即使您的電腦關閉也能執行，並且也可以透過 API 呼叫或 GitHub 事件觸發。本頁涵蓋本機排程任務；如需遠端 routine 及其觸發選項，請參閱 [Routines](/docs/zh-TW/routines)。

<h2 id="compare-scheduling-options">
  比較排程選項
</h2>

Claude Code offers three ways to schedule recurring or one-off work:

|                            | [Cloud](/docs/en/routines)               | [Desktop](/docs/en/desktop-scheduled-tasks) | [`/loop`](/docs/en/scheduled-tasks)      |
| :------------------------- | :---------------------------------- | :------------------------------------- | :---------------------------------- |
| Runs on                    | Cloud, Anthropic-managed by default | Your machine                           | Your machine                        |
| Requires machine on        | No                                  | Yes                                    | Yes                                 |
| Requires open session      | No                                  | No                                     | Yes                                 |
| Persistent across restarts | Yes                                 | Yes                                    | Restored on `--resume` if unexpired |
| Access to local files      | No (fresh clone)                    | Yes                                    | Yes                                 |
| MCP servers                | Connectors configured per task      | [Config files](/docs/en/mcp) and connectors | Inherits from session               |
| Permission prompts         | No (runs autonomously)              | Configurable per task                  | Inherits from session               |
| Customizable schedule      | Via `/schedule` in the CLI          | Yes                                    | Yes                                 |
| Minimum interval           | 1 hour                              | 1 minute                               | 1 minute                            |

<Tip>
  Use **cloud tasks** for work that should run reliably without your machine. Use **Desktop tasks** when you need access to local files and tools. Use **`/loop`** for quick polling during a session.
</Tip>

<Note>
  根據預設，排程任務會針對您的工作目錄的任何狀態執行，包括未提交的變更。在建立任務時啟用 worktree 切換，為每次執行提供其自己的隔離 Git worktree，與 [parallel sessions](/docs/zh-TW/desktop#work-in-parallel-with-sessions) 的工作方式相同。
</Note>

<h2 id="create-a-scheduled-task">
  建立排程任務
</h2>

按一下側邊欄中的 **Routines**，然後按一下 **New routine** 並選擇 **Local**。設定這些欄位：

| 欄位           | 說明                                                                                                          |
| ------------ | ----------------------------------------------------------------------------------------------------------- |
| Name         | 任務的識別碼。轉換為小寫 kebab-case 並用作磁碟上的資料夾名稱。在您的任務中必須是唯一的。                                                          |
| Description  | 在任務清單中顯示的簡短摘要。                                                                                              |
| Instructions | Claude 執行任務時應執行的操作。以您在提示框中撰寫任何訊息的相同方式撰寫此項。instructions 輸入包括權限模式和模型的選擇器，在其下方您可以選擇工作資料夾以及是否在隔離的 worktree 中執行。 |
| Schedule     | 任務執行的頻率。請參閱下方的 [schedule options](#schedule-options)。                                                       |

您必須先有一個資料夾才能儲存任務。如果您尚未信任該資料夾，Desktop 會在儲存前提示您信任它。

您也可以透過在任何工作階段中描述您想要的內容來建立任務。例如，「設定每天早上 9 點執行的每日程式碼審查」會建立定期任務，而「提醒我明天下午 3 點檢查部署」會建立一次性任務，在觸發後會自動停用。

<h2 id="schedule-options">
  排程選項
</h2>

從 Schedule 控制項中選擇預設值：

* **Manual**：無排程，只有在您按一下 **Run now** 時才執行。適用於儲存您按需觸發的提示
* **Hourly**：每小時執行一次
* **Daily**：顯示時間選擇器，預設為本機時間上午 9:00
* **Weekdays**：與 Daily 相同，但跳過星期六和星期日
* **Weekly**：顯示時間選擇器和日期選擇器

對於選擇器不提供的間隔，例如每 15 分鐘、每月的第一天或在特定未來時間的單次執行，請在任何 Desktop 工作階段中詢問 Claude 以設定排程。使用純文字；例如，「排程任務每 6 小時執行一次所有測試」。

<h2 id="how-scheduled-tasks-run">
  排程任務如何執行
</h2>

排程任務在您的機器上執行。Desktop 在應用程式開啟時每分鐘檢查一次排程，並在任務到期時啟動新的工作階段，獨立於您開啟的任何手動工作階段。每個任務在排程時間後會有幾分鐘的小延遲，以錯開 API 流量。延遲是確定性的：同一任務始終在相同的偏移量處啟動。

當任務觸發時，您會收到桌面通知，新的工作階段會在側邊欄的 **Scheduled** 部分下出現。開啟它以查看 Claude 執行的操作、審查變更或回應權限提示。工作階段的工作方式與任何其他工作階段相同：Claude 可以編輯檔案、執行命令、建立提交和開啟提取請求。

任務只有在 desktop 應用程式執行且您的電腦處於喚醒狀態時才會執行。如果您的電腦在排程時間內進入睡眠狀態，該執行會被跳過。若要防止閒置睡眠，請在 Settings 中的 **Desktop app → General** 下啟用 **Keep computer awake**。關閉筆記型電腦蓋仍會使其進入睡眠狀態。對於需要在電腦關閉時執行或應該透過 API 呼叫或 GitHub 事件觸發的任務，請改為建立遠端 [routine](/docs/zh-TW/routines)。

<h2 id="missed-runs">
  錯過的執行
</h2>

當應用程式啟動或您的電腦喚醒時，Desktop 會檢查每個任務是否在過去七天內錯過了任何執行。如果有，Desktop 會為最近錯過的時間啟動恰好一次的追趕執行，並丟棄任何較舊的執行。錯過六天的每日任務在喚醒時執行一次。Desktop 會在追趕執行啟動時顯示通知。

在撰寫提示時請記住這一點。排程在上午 9 點的任務可能在晚上 11 點執行，如果您的電腦整天都在睡眠狀態。如果時間很重要，請在提示本身中新增護欄，例如：「只審查今天的提交。如果已經過下午 5 點，請跳過審查，只發佈錯過內容的摘要。」

<h2 id="permissions-for-scheduled-tasks">
  排程任務的權限
</h2>

每個任務都有其自己的權限模式，您在建立或編輯任務時設定。來自 `~/.claude/settings.json` 的允許規則也適用於排程任務工作階段。如果任務在 Ask 模式下執行，並且需要執行它沒有權限的工具，執行會停滯，直到您批准它。工作階段保持在側邊欄中開啟，以便您稍後可以回答。

為了避免停滯，在建立任務後按一下 **Run now**，監視權限提示，並為每個提示選擇「always allow」。該任務的未來執行會自動批准相同的工具，無需提示。您可以從任務的詳細資料頁面審查和撤銷這些批准。

您的組織設定為 `ask` 的 Connector 工具和標記為 [`requiresUserInteraction`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 的 MCP 工具會在每次呼叫時提示，並且不提供 always-allow 選項。呼叫這些工具的執行每次都會停滯。

<h2 id="manage-scheduled-tasks">
  管理排程任務
</h2>

按一下 **Routines** 清單中的任務以開啟其詳細資料頁面。從這裡您可以：

* **Run now**：立即啟動任務，無需等待下一個排程時間
* **Status**：在 Active 和 Paused 之間切換，以暫停或繼續排程執行，無需刪除任務
* **Edit**：變更 instructions、排程、資料夾或其他設定
* **Review history**：查看每次過去的執行，包括跳過的執行。將滑鼠懸停在跳過的項目上以查看原因：您的電腦在睡眠狀態、前一次執行仍在進行中，或其他排程任務已在執行。按一下 **Show more** 以載入較舊的項目。
* **Review allowed permissions**：從 **Always allowed** 面板查看和撤銷此任務的已儲存工具批准
* **Delete**：移除任務並封存它建立的所有工作階段。確認對話方塊中會出現 **Also delete files on disk** 核取方塊；勾選它也會移除任務的 `SKILL.md` 檔案和 `~/.claude/scheduled-tasks/` 中的相關資料。

您也可以透過在任何 Desktop 工作階段中詢問 Claude 來列出、建立、編輯和暫停任務。例如，「暫停我的 dependency-audit 任務」或「顯示我的排程任務」。若要刪除任務，請使用其詳細資料頁面上的 **Delete** 按鈕。

排程任務也可以使用 `update_scheduled_task` MCP 工具在執行中的工作階段內修改其自己的排程或提示。這可讓任務根據其發現重新排程自己，例如，當它偵測到已建立發行分支時，重新排程程式碼審查以更早執行。

若要在磁碟上編輯任務的提示，請開啟 `~/.claude/scheduled-tasks/<task-name>/SKILL.md`（如果設定了 [`CLAUDE_CONFIG_DIR`](/docs/zh-TW/env-vars)，則在其下）。該檔案使用 YAML frontmatter 作為 `name` 和 `description`，提示作為主體。變更在下一次執行時生效。Schedule、資料夾、模型和啟用狀態不在此檔案中：透過 Edit 表單變更它們或詢問 Claude。

<h2 id="related-resources">
  相關資源
</h2>

* [Routines](/docs/zh-TW/routines)：在 Anthropic 管理的基礎設施上按排程、透過 API 呼叫或回應 GitHub 事件執行任務，即使您的電腦關閉也能執行
* [在排程上執行提示](/docs/zh-TW/scheduled-tasks)：在 CLI 中使用 `/loop` 的工作階段範圍排程
* [Claude Code GitHub Actions](/docs/zh-TW/github-actions)：在 CI 中按排程執行 Claude，而不是在您的機器上執行
* [使用 Claude Code Desktop](/docs/zh-TW/desktop)：完整的 Desktop 應用程式指南
