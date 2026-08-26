> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 輸出樣式

> 將 Claude Code 適配用於軟體工程以外的用途

輸出樣式改變 Claude 的回應方式，而不是 Claude 知道什麼。它們修改系統提示以設定角色、語氣和輸出格式。當您每次都重新提示相同的語音或格式，或者當您希望 Claude 充當軟體工程師以外的角色時，請使用一個。

自訂輸出樣式將您的指令添加到系統提示，並讓您選擇是否保留 Claude Code 的內建軟體工程指令。當您改變 Claude 的溝通方式但仍在編碼時（例如始終用圖表回答），請保留它們。當 Claude 根本不進行軟體工程時（例如寫作助手或數據分析師），請省略它們。

有關您的專案、慣例或程式碼庫的說明，請改用 [CLAUDE.md](/docs/zh-TW/memory)。

<h2 id="built-in-output-styles">
  內建輸出樣式
</h2>

Claude Code 的**預設**輸出樣式是現有的系統提示，旨在幫助您有效地完成軟體工程任務。

還有三種額外的內建輸出樣式：

* **Proactive**：Claude 立即執行，做出合理的假設而不是暫停進行例行決策，並偏好行動而非規劃。這比[自動模式](/docs/zh-TW/permission-modes#eliminate-prompts-with-auto-mode)提供更強的自主執行指導，且無需改變您的權限模式，因此您在工具運行前仍會看到權限提示。

* **Explanatory**：在幫助您完成軟體工程任務的同時提供教育性的「Insights」。幫助您理解實現選擇和程式碼庫模式。

* **Learning**：協作式的邊做邊學模式，Claude 不僅會在編碼時分享「Insights」，還會要求您自己貢獻小的、策略性的程式碼片段。Claude Code 將在您的程式碼中添加 `TODO(human)` 標記供您實現。

<h2 id="change-your-output-style">
  變更您的輸出樣式
</h2>

執行 `/config` 並選擇**輸出樣式**以從選單中選擇樣式。您的選擇會儲存到[本地專案層級](/docs/zh-TW/settings)的 `.claude/settings.local.json`。

<Note>獨立的 `/output-style` 命令已在 v2.1.73 中棄用，並在 v2.1.91 中移除。請使用 `/config` 或直接編輯 `outputStyle` 設定。</Note>

若要在不使用選單的情況下設定樣式，請直接編輯設定檔中的 `outputStyle` 欄位：

```json theme={null}
{
  "outputStyle": "Explanatory"
}
```

輸出樣式是系統提示的一部分，Claude Code 在工作階段開始時會讀取一次。變更會在執行 `/clear` 或新工作階段後生效。請參閱[Claude Code 如何使用 prompt caching](/docs/zh-TW/prompt-caching#changing-output-style)以了解輸出樣式變更對快取的影響。

<h2 id="create-a-custom-output-style">
  建立自訂輸出樣式
</h2>

自訂輸出樣式是一個 Markdown 檔案：frontmatter 用於中繼資料，然後是要添加到系統提示的指令。

<Steps>
  <Step title="建立 Markdown 檔案">
    將其儲存在三個層級之一。檔案名稱成為樣式名稱，除非您在 frontmatter 中設定 `name`。

    * 使用者：`~/.claude/output-styles`
    * 專案：`.claude/output-styles`
    * 受管原則：[受管設定目錄](/docs/zh-TW/settings#settings-files)內的 `.claude/output-styles`

    專案輸出樣式會從工作目錄和儲存庫根目錄之間的每個 `.claude/output-styles/` 載入。自 v2.1.178 起，當多個這些巢狀目錄定義同名樣式時，Claude Code 會使用最接近工作目錄的那個。
  </Step>

  <Step title="添加 frontmatter 和指令">
    決定是否保留 Claude Code 的軟體工程指令。如果您改變 Claude 的溝通方式但仍希望它以相同方式編碼，請設定 `keep-coding-instructions: true`。如果 Claude 不會進行軟體工程，請省略它。

    此範例在保留 Claude 編碼行為的同時，在每個說明前面加上圖表：

    ```markdown theme={null}
    ---
    name: Diagrams first
    description: Lead every explanation with a diagram
    keep-coding-instructions: true
    ---

    When explaining code, architecture, or data flow, start with a Mermaid diagram showing the structure, then explain in prose.

    ## Diagram conventions

    Use `flowchart TD` for control flow and `sequenceDiagram` for request paths. Keep diagrams under 15 nodes.
    ```
  </Step>

  <Step title="切換到您的樣式">
    執行 `/config` 並在**輸出樣式**下選擇您的樣式。它將在 `/clear` 之後或下次啟動工作階段時生效。
  </Step>
</Steps>

[Plugins](/docs/zh-TW/plugins-reference) 也可以在 `output-styles/` 目錄中提供輸出樣式。

<h3 id="frontmatter">
  Frontmatter
</h3>

輸出樣式檔案支援這些 frontmatter 欄位：

| Frontmatter                | 用途                                                                                                              | 預設      |
| :------------------------- | :-------------------------------------------------------------------------------------------------------------- | :------ |
| `name`                     | 輸出樣式的名稱，如果不是檔案名稱                                                                                                | 繼承自檔案名稱 |
| `description`              | 輸出樣式的描述，在 `/config` 選擇器中顯示                                                                                      | 無       |
| `keep-coding-instructions` | 保留 Claude Code 的內建軟體工程指令                                                                                        | `false` |
| `force-for-plugin`         | 僅限 Plugin 輸出樣式：在啟用 plugin 時自動應用此樣式，無需要求使用者選擇它。覆蓋使用者的 `outputStyle` 設定。如果多個啟用的 plugin 設定此項，Claude Code 使用第一個載入的。 | `false` |

<h2 id="how-output-styles-work">
  輸出樣式的工作原理
</h2>

輸出樣式直接修改 Claude Code 的系統提示。

* 所有輸出樣式都在系統提示的末尾添加了自己的自訂指令。
* 所有輸出樣式都會在對話期間觸發提醒，讓 Claude 遵守輸出樣式指令。
* 自訂輸出樣式排除了 Claude Code 的內建軟體工程指令，例如如何限定變更範圍、編寫註解和驗證工作，除非 `keep-coding-instructions` 設定為 `true`。

Token 使用量取決於樣式。將指令添加到系統提示會增加輸入 token，儘管 prompt caching 在工作階段中的第一個請求之後會降低此成本。內建的 Explanatory 和 Learning 樣式按設計會產生比預設更長的回應，這會增加輸出 token。對於自訂樣式，輸出 token 使用量取決於您的指令告訴 Claude 要產生什麼。

<h2 id="comparisons-to-related-features">
  與相關功能的比較
</h2>

多個功能自訂 Claude Code 的行為方式。輸出樣式直接修改系統提示並應用於每個回應。其他功能添加指令而不改變預設系統提示，或將其限定於特定任務。

| 功能                          | 工作原理                 | 使用時機                        |
| :-------------------------- | :------------------- | :-------------------------- |
| 輸出樣式                        | 修改系統提示               | 您希望每次都有不同的角色、語氣或預設回應格式      |
| [CLAUDE.md](/docs/zh-TW/memory)  | 在系統提示之後添加使用者訊息       | Claude 應該始終知道您的專案慣例和程式碼庫上下文 |
| `--append-system-prompt`    | 附加到系統提示而不移除任何內容      | 您希望為單個呼叫進行一次性添加             |
| [Agents](/docs/zh-TW/sub-agents) | 使用自己的系統提示、模型和工具運行子代理 | 您希望為專注任務提供單獨作用域的幫助程式        |
| [Skills](/docs/zh-TW/skills)     | 在呼叫或相關時載入特定於任務的指令    | 您有可重複使用的工作流程                |

<h2 id="related-resources">
  相關資源
</h2>

* [Settings](/docs/zh-TW/settings)：`outputStyle` 欄位所在位置以及設定優先順序的工作原理
* [Permission modes](/docs/zh-TW/permission-modes)：Proactive 樣式與自動模式的比較方式
* [Plugins](/docs/zh-TW/plugins)：與 skills、hooks 和 agents 一起打包和分發輸出樣式
* [Debug your configuration](/docs/zh-TW/debug-your-config)：診斷為什麼輸出樣式沒有生效
