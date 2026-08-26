> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 並行運行代理

> 比較 Claude Code 同時處理多個任務的方式：子代理、代理視圖、代理團隊和動態工作流。

[子代理](/docs/zh-TW/sub-agents)、[代理視圖](/docs/zh-TW/agent-view)、[代理團隊](/docs/zh-TW/agent-teams) 和 [動態工作流](/docs/zh-TW/workflows) 各自以不同的方式並行化工作。正確的選擇取決於您是否想要自己留在每個對話中、交付任務並稍後檢查，或讓 Claude 為您協調一組工作人員。

| 方法                         | 它提供什麼                                            | 何時使用                                                                     |
| :------------------------- | :----------------------------------------------- | :----------------------------------------------------------------------- |
| [子代理](/docs/zh-TW/sub-agents)   | 在一個會話內的委派工作人員，在自己的上下文中執行側任務並返回摘要                 | 側任務會用搜尋結果、日誌或您不會再次參考的文件內容淹沒您的主要對話                                        |
| [代理視圖](/docs/zh-TW/agent-view)  | 一個屏幕來調度和監控在後台運行的會話，使用 `claude agents` 打開。研究預覽    | 您有多個獨立任務，想要交付它們，一目了然地檢查狀態，並且只在需要時介入                                      |
| [代理團隊](/docs/zh-TW/agent-teams) | 多個協調的會話，具有共享任務列表和代理間消息傳遞，由領導者管理。實驗性功能，默認禁用       | 您希望 Claude 將項目分成多個部分、分配它們並保持工作人員同步                                       |
| [動態工作流](/docs/zh-TW/workflows)  | 一個運行許多子代理並檢查其結果的腳本，用於一個太大而無法一次協調的工作或需要多於單一次通過的工作 | 一個任務對於少數子代理來說太大了，或您想要驗證發現相互對抗：一個代碼庫範圍的審計、一個 500 文件遷移、交叉檢查的研究，或從多個角度起草的計劃 |

在每種方法中，工作人員都是 Claude 會話。要涉及不同的工具，請將其作為 [MCP server](/docs/zh-TW/mcp) 公開給 Claude。

還有兩個工具支持這項工作，但它們本身不是運行代理的方式：

* [Worktrees](/docs/zh-TW/worktrees) 為每個會話提供單獨的 git 檢出，因此並行會話永遠不會編輯相同的文件。將它們用於您自己運行的會話。代理視圖會自動將每個調度的會話移動到自己的 worktree 中，您生成的子代理也可以各自獲得一個。
* [`/batch`](/docs/zh-TW/commands) 是一個 [skill](/docs/zh-TW/skills)，它讓 Claude 將一個大型更改分成 5 到 30 個 worktree 隔離的子代理，每個都打開一個拉取請求。它是子代理和 worktrees 的打包使用，不是一個單獨的協調風格。

還有一些其他功能在沒有您驅動每一步的情況下運行 Claude，但它們解決的問題與在代理之間分割工作不同：

* 一個 [background bash command](/docs/zh-TW/interactive-mode#background-bash-commands) 運行一個 shell 命令而不阻止對話。它不會生成一個代理。
* 一個 [forked subagent](/docs/zh-TW/sub-agents#fork-the-current-conversation) 是一個繼承您完整對話上下文而不是從頭開始的子代理。它是一種生成子代理的方式，不是一個單獨的表面。
* 一個 [routine](/docs/zh-TW/routines) 在 Anthropic 的雲中按計劃運行一個會話，而不是在您的機器上並行運行。

<Note>
  同時運行多個會話或子代理會增加令牌使用量。有關使用情況和速率限制詳細信息，請參閱 [Costs](/docs/zh-TW/costs)。
</Note>

<h2 id="choose-an-approach">
  選擇一種方法
</h2>

正確的方法取決於誰協調工作、工作人員是否需要通信以及他們是否編輯相同的文件：

* **誰協調工作？**
  * Claude 在一個對話中委派和收集結果：[subagents](/docs/zh-TW/sub-agents)
  * 您交付獨立任務並稍後檢查：[agent view](/docs/zh-TW/agent-view)
  * Claude 計劃、分配和監督一組工作人員：[agent teams](/docs/zh-TW/agent-teams)，實驗性且默認禁用
  * 一個腳本而不是 Claude 的逐輪判斷來保持協調：[dynamic workflows](/docs/zh-TW/workflows)。請參閱 [workflows 與 subagents 和 skills 的比較方式](/docs/zh-TW/workflows#when-to-use-a-workflow)
* **工作人員需要相互交談嗎？** Subagents 將結果報告回生成它們的對話，agent view 會話只向您報告。agent team 中的隊友共享任務列表並直接相互發送消息。
* **任務是否涉及相同的文件？** 使用 [worktrees](/docs/zh-TW/worktrees) 隔離工作。Subagents 和您自己運行的會話可以各自使用單獨的 worktree。Agent teams 不會在 worktrees 中隔離隊友，因此 [分區工作](/docs/zh-TW/agent-teams#avoid-file-conflicts)，以便每個隊友擁有不同的文件集。

<h2 id="check-on-running-work">
  檢查運行中的工作
</h2>

檢查運行中工作的命令取決於您使用的方法：

* 對於後台會話，`claude agents` 打開 [代理視圖](/docs/zh-TW/agent-view)：一個屏幕顯示每個會話、其狀態以及哪些需要您的輸入。
* 對於當前會話中的子代理，命名的後台子代理出現在 @-mention 類型提前中，並顯示其狀態。從 v2.1.198 開始，`/agents` 不再打開面板；它打印一個通知，指向子代理文件位置。要 [創建和編輯自定義子代理](/docs/zh-TW/sub-agents#configure-subagents)，請詢問 Claude 或直接編輯文件。儘管名稱相似，但 `/agents` 與 `claude agents` 分開。
* 對於當前會話後台運行的任何內容，`/tasks` 列出每個項目，並讓您檢查、附加到或停止它。該列表還包括已完成的子代理。
* 對於動態工作流程，`/workflows` 列出運行和已完成的運行、每個運行所處的階段，以及有多少代理已完成。

有關所有會話的桌面視圖，請參閱 [桌面應用中的並行會話](/docs/zh-TW/desktop#work-in-parallel-with-sessions)。

<h2 id="learn-more">
  了解更多
</h2>

下面的每個指南涵蓋一種方法的設置和配置：

* [創建自定義子代理](/docs/zh-TW/sub-agents)：定義可重用的專家並控制他們可以使用的工具。
* [使用代理視圖管理代理](/docs/zh-TW/agent-view)：調度會話、監視其狀態並在需要時附加。
* [協調代理團隊](/docs/zh-TW/agent-teams)：設置領導者和隊友、分配任務並審查他們的工作。
* [協調動態工作流](/docs/zh-TW/workflows)：運行捆綁的工作流或讓 Claude 編寫一個運行許多子代理並驗證其發現相互對比的工作流。
* [使用 worktrees 運行並行會話](/docs/zh-TW/worktrees)：在隔離的檢出中啟動 Claude、控制複製的內容並在之後進行清理。
