> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用 Agent view 管理多個代理

> 從一個螢幕分派和管理許多 Claude Code 工作階段。Agent view 顯示每個工作階段正在做什麼，以及哪些需要您的輸入。

Agent view（使用 `claude agents` 開啟）是所有背景工作階段的一個螢幕：什麼正在執行、什麼需要您的輸入，以及什麼已完成。分派新工作階段，一目瞭然地查看它們的狀態，而不是滾動瀏覽記錄，並且只在需要時才介入。每個背景工作階段都是一個完整的 Claude Code 對話，在沒有終端連接的情況下持續執行，因此您可以隨時開啟、回覆和離開。

<img src="https://mintcdn.com/claude-code/1B48Qz2Z9hac4SLG/images/agent-view-light.png?fit=max&auto=format&n=1B48Qz2Z9hac4SLG&q=85&s=7a186c96ed47d6700d084d77e786be65" className="dark:hidden" alt="終端中的 Agent view：標題顯示 Claude Code v2.1.140、模型、工作目錄和摘要計數。工作階段分組在'需要輸入'、'執行中'和'已完成'下，底部有分派輸入，頁尾有快捷鍵提示。" width="1772" height="780" data-path="images/agent-view-light.png" />

<img src="https://mintcdn.com/claude-code/1B48Qz2Z9hac4SLG/images/agent-view-dark.png?fit=max&auto=format&n=1B48Qz2Z9hac4SLG&q=85&s=a5bed7434bae368faea3a8f023b52aa2" className="hidden dark:block" alt="終端中的 Agent view：標題顯示 Claude Code v2.1.140、模型、工作目錄和摘要計數。工作階段分組在'需要輸入'、'執行中'和'已完成'下，底部有分派輸入，頁尾有快捷鍵提示。" width="1772" height="780" data-path="images/agent-view-dark.png" />

當您有多個獨立任務 Claude 可以在不需要您監看每一步的情況下執行時，請使用 agent view。分派一個錯誤修復、一個拉取請求審查和一個不穩定測試調查作為三行，在另一個視窗中繼續工作，並在某一行顯示需要您或有結果時檢查。

當您想在任何代理的工作階段中更直接地工作時，附加到該行以進入完整對話。

若要比較 agent view 與 subagents、agent teams 和 worktrees，請參閱 [平行執行代理](/docs/zh-TW/agents)。

<Note>
  Agent view 是研究預覽版本，需要 Claude Code v2.1.139 或更新版本。使用 `claude --version` 檢查您的版本。隨著功能的發展，介面和快捷鍵可能會改變。
</Note>

本頁涵蓋：

* [快速開始](#quick-start)：給 Claude 一個在背景中執行的任務，檢查它，並在需要時介入
* [使用 agent view 監控工作階段](#monitor-sessions-with-agent-view)，包括狀態圖示、查看和回覆、附加、組織和快捷鍵
* [分派新代理](#dispatch-new-agents)，從 agent view、從工作階段內部或從 shell
* [從 shell 管理工作階段](#manage-sessions-from-the-shell)，使用 `claude agents`、`claude attach` 和相關命令
* [背景工作階段如何被託管](#how-background-sessions-are-hosted)，由監督程序

<h2 id="quick-start">
  快速開始
</h2>

本逐步解說涵蓋核心 agent view 迴圈：分派工作、觀看其列更新（Claude 正在工作）、查看以檢查並回覆，以及附加到完整對話。您分派的工作階段在關閉 agent view 後會繼續執行，因此您可以離開並稍後返回。

<Steps>
  <Step title="開啟 agent view">
    從您的 shell，執行：

    ```bash theme={null}
    claude agents
    ```

    Agent view 開啟，底部有輸入框，隨著工作階段啟動，表格會填入。隨時按 `Esc` 返回您的 shell。您的工作階段在您離開時繼續執行，下次開啟 agent view 時會重新出現。
  </Step>

  <Step title="分派工作階段">
    輸入描述工作的提示並按 `Enter`。新的背景工作階段在該工作上啟動並顯示為一列，顯示它是否正在工作、等待您或已完成。新工作階段使用 agent view 標題中顯示的模型和在該目錄中執行 `claude` 時會獲得的相同[權限模式](#permission-mode-model-and-effort)。

    您在此輸入的每個提示都會啟動自己的新工作階段。輸入另一個提示並按 `Enter` 會在第一個工作階段旁邊啟動第二個工作階段，而不是向其發送後續訊息。您可以以這種方式並行執行多個工作階段。

    每個工作階段獨立使用您的訂閱配額，因此在一次分派許多工作階段之前，請參閱[限制](#limitations)。
  </Step>

  <Step title="查看和回覆">
    使用箭頭鍵選擇一列，然後按 `Space` 開啟查看面板。它顯示工作階段的最新輸出或它正在等待的問題，而不是完整的文字記錄。輸入回覆並按 `Enter` 發送，無需離開 agent view。
  </Step>

  <Step title="附加和分離">
    在一列上按 `Enter` 或 `→` 以在需要完整對話時附加。工作階段接管終端，就像完整的互動式 Claude Code 工作階段一樣。在空提示上按 `←` 分離並返回表格。
  </Step>

  <Step title="帶入現有工作階段">
    這個步驟需要一個執行中的工作階段。如果您遵循了之前的步驟，您在此終端中沒有開啟的工作階段，因此請在另一個終端中開啟一個常規 `claude` 工作階段並先向其發送訊息。要將您已開啟的工作階段移入 agent view，在其中執行 `/bg`，或在空提示上按 `←` 以在一個步驟中背景化工作階段並開啟 agent view。工作階段繼續執行並顯示為一列，與您分派的工作階段並排。
  </Step>
</Steps>

您可以使用 `claude agents` 作為主要進入點而不是 `claude`：從 agent view 分派每個工作，在需要完整對話時附加，然後按 `←` 返回表格。

在常規 `claude` 工作階段內，提示頁尾的 `←` 提示會計算正在等待您的背景 agent 數量，例如 `← 2 agents`，當沒有任何 agent 需要輸入時會返回 `← for agents`。超過 99 的計數顯示為 `99+`。當終端獲得焦點時，計數大約每十秒刷新一次，當焦點返回時立即刷新。當計數移動時以及當 agent 完成時，它會短暫改變顏色，除非啟用了 [`prefersReducedMotion` 設定](/docs/zh-TW/settings#available-settings)，並且在[螢幕閱讀器模式](/docs/zh-TW/accessibility)中隱藏。在 [Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry](/docs/zh-TW/third-party-integrations) 上，提示保持其純 `← for agents` 形式，不顯示計數。需要 Claude Code v2.1.205 或更新版本。

<h2 id="monitor-sessions-with-agent-view">
  使用 agent view 監控工作階段
</h2>

執行 `claude agents` 開啟 agent view。它接管整個終端並列出按狀態分組的每個工作階段，固定的工作階段和需要您的工作階段在頂部。每行顯示工作階段的名稱、當前活動和其年齡，從工作階段建立時開始計算；已完成的工作階段的年齡會凍結在執行花費的時間。

名稱以該工作階段中由 [`/color`](/docs/zh-TW/commands) 設定的顏色著色。自 v2.1.199 起，當您使用 `←` 或 `/background` [背景化工作階段](#from-inside-a-session)時，顏色會保留。

根據預設，該列表顯示您啟動的每個背景工作階段，跨越所有您的專案。在一個儲存庫中工作的工作階段和在不同 worktree 中工作的另一個工作階段都會出現在這裡，無論您從哪個目錄開啟 agent view。要將檢視範圍限制在一個專案，請傳遞 `--cwd`：

```bash theme={null}
claude agents --cwd ~/projects/my-app
```

這只會顯示在該目錄下啟動的工作階段。已[移入 worktree](#how-file-edits-are-isolated)在 `~/projects/my-app/.claude/worktrees/` 下的工作階段仍然算作屬於 `~/projects/my-app`。

您在其他終端中開啟的互動工作階段在您[背景化它們](#from-inside-a-session)之前不會出現。[Subagents](/docs/zh-TW/sub-agents) 和 [teammates](/docs/zh-TW/agent-teams) 工作階段產生的不會列為單獨的行。

```text theme={null}
Pinned
  ✽ clawd walk cycle          Drawing the walk-cycle sprite frames          3m

Ready for review
  ∙ jump physics              Opened PR with collision fix                 #2048  2h

Needs input
  ✻ power-up design           double jump or wall climb?                    1m

Working
  ✽ collision detection       Adding swept-AABB checks to CollisionSystem   2m
  ✢ playtest level 3          run 12 · all checkpoints cleared           in 4m

Completed
  ✻ title screen              result: menu, options, and credits done       9m
  ∙ sound effects             result: 14 SFX exported to assets/audio       4h
  … 6 more
```

<h3 id="read-session-state">
  讀取工作階段狀態
</h3>

每行開始的圖示，其顏色和動畫顯示工作階段的狀態：

| 狀態   | 圖示顯示為 | 含義                                  |
| :--- | :---- | :---------------------------------- |
| 工作中  | 動畫    | Claude 正在主動執行工具或生成回應                |
| 需要輸入 | 黃色    | Claude 正在等待您的特定問題或權限決定              |
| 閒置   | 淡化    | 工作階段沒有事情要做，準備好接收您的下一個提示             |
| 已完成  | 綠色    | 任務成功完成                              |
| 失敗   | 紅色    | 任務以錯誤結束                             |
| 已停止  | 灰色    | 工作階段已使用 `Ctrl+X` 或 `claude stop` 停止 |

另外，圖示的形狀顯示底層程序是否正在執行：

| 形狀          | 含義                                                             |
| :---------- | :------------------------------------------------------------- |
| `✻` 或動畫 `✽` | 工作階段程序處於活動狀態並立即回覆                                              |
| `∙`         | 程序已退出。您仍然可以查看、回覆或附加，Claude 從中斷的地方重新啟動                          |
| `✢`         | 一個 [`/loop`](/docs/zh-TW/scheduled-tasks) 工作階段在迭代之間休眠。該行顯示其執行計數和倒計時 |

出現在行右邊緣的 `#N` 標籤是[工作階段開啟的拉取請求](#pull-request-status)，不是狀態圖示的一部分。

終端標籤標題在 agent view 開啟時顯示等待輸入計數：當工作階段需要輸入時為 `2 awaiting input · claude agents`，或當沒有工作階段需要輸入時為 `claude agents`。

自 v2.1.198 起，當 agent view 開啟時，Claude Code 也會通過您配置的[終端通知頻道](/docs/zh-TW/terminal-config#get-a-terminal-bell-or-notification)發送通知，當本機背景工作階段開始需要您的輸入、完成或失敗時。在排程上執行的工作階段，例如 [`/loop`](/docs/zh-TW/scheduled-tasks) 工作階段，只在需要您的輸入時通知。通知使用與 Claude Code 其餘部分相同的 [`preferredNotifChannel` 設定](/docs/zh-TW/settings#available-settings)，並使用 `agent_needs_input` 或 `agent_completed` 類型觸發 [`Notification` hook](/docs/zh-TW/hooks#notification)。

背景工作階段不需要任何開啟的終端即可繼續工作。單獨的[監督程序](#the-supervisor-process)執行它們，因此您可以關閉 agent view、關閉 shell 或啟動新的互動工作階段，您分派的工作會繼續進行。

工作階段狀態通過自動更新和監督程序重新啟動在磁碟上持久化。工作階段也會在您的機器進入睡眠時保留。它們的程序在喚醒時恢復，監督程序會重新連接到它們，而不是將時間間隔視為閒置。關閉仍會停止執行中的工作階段；請參閱[工作階段在關閉後顯示為失敗](#sessions-show-as-failed-after-shutdown)以了解如何恢復它們。

當您開啟已停止回應的工作階段時，監督程序會重新啟動其程序，工作階段會從中斷的地方繼續中斷的回應。當機器在中途回應時進入睡眠時，工作階段可能會陷入該狀態。需要 Claude Code v2.1.200 或更新版本。

<h3 id="row-summaries">
  行摘要
</h3>

每行中的單行摘要由 [Haiku-class 模型](/docs/zh-TW/model-config)生成，因此該行可以告訴您工作階段正在做什麼、需要什麼或生成了什麼，無需開啟記錄。當工作階段主動工作時，該行文字最多每 15 秒從工作階段自己的最近輸出更新一次，無需發送模型請求，模型在每個回合結束時寫入新摘要。

工作中的行顯示工作階段說它正在做什麼，被阻止的行顯示它正在詢問的問題。在長回合期間，模型也會大約每分鐘重寫一次摘要，每次重寫後等待時間加倍，最多四分鐘，因此繁忙的行不會持續顯示過時的摘要。摘要文字填充該行的剩餘寬度，只在終端的右邊緣截斷；開啟[查看面板](#peek-and-reply)以讀取邊緣裁剪的句子。在 v2.1.206 之前，文字在 64 列處截斷，無論終端寬度如何。

當列表[按目錄分組](#organize-the-list)時，摘要以工作階段的狀態作為著色詞開頭，例如 `Needs input · double jump or wall climb?`。在預設狀態分組中，組標題已命名狀態，因此該行只顯示摘要。在 v2.1.205 之前，按目錄分組的行不帶狀態詞。

整個輸出不包含字母或數字的回合，例如在安靜迭代中列印單個符號的 [`/loop`](/docs/zh-TW/scheduled-tasks) 工作階段，保持該行的先前摘要和狀態。在 v2.1.205 之前，該回合被重新分類，可能會將等待您輸入的工作階段翻轉回 `Working`。

回合結束摘要和每次中途重寫都是通過您的正常提供者的一個簡短 Haiku-class 請求，按照與工作階段本身相同的[資料使用條款](/docs/zh-TW/data-usage)計費和處理。15 秒的模型重寫之間的更新重用工作階段自己的輸出，不發送請求。在第三方提供者（例如 Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 和自訂閘道）上，當未配置 Haiku 模型時，請求會回退到工作階段的主要模型。設定 [`ANTHROPIC_DEFAULT_HAIKU_MODEL`](/docs/zh-TW/model-config#environment-variables)以在這些提供者上為這些摘要選擇模型。

<h3 id="pull-request-status">
  拉取請求狀態
</h3>

當工作階段開啟拉取請求時，`#1234` 標籤會出現在該行的右邊緣，在支援超連結的終端中連結到拉取請求。當您向工作階段發送後續訊息時標籤會保留，因此拉取請求在該行恢復為即時進度時保持可見。隔離其變更在 worktree 中的背景工作階段會自己開啟這些拉取請求；[檔案編輯如何隔離](#how-file-edits-are-isolated)涵蓋何時發生以及工作階段在未詢問的情況下永遠不會做什麼。

在現有拉取請求上工作的工作階段以相同方式連結到它。使用 `gh` 編輯、評論、關閉或標記拉取請求為準備好會連結該命令自己的輸出命名的拉取請求，因此其捕獲的輸出不命名拉取請求的 `gh` 命令不會建立連結；`gh pr merge` 是常見情況，因為它只將其結果列印到互動終端。使用 `gh pr checkout` 簽出拉取請求，或推送到具有開啟拉取請求的分支，會改為使用 `gh pr view` 查詢該分支來連結它。在 v2.1.205 之前，只有工作階段建立或簽出的拉取請求被連結，推送只在本機分支名稱匹配時連結一個。

Claude Code 從完整命令輸出讀取拉取請求，包括當命令輸出超過內聯限制時保存到檔案的部分。在 v2.1.205 之前，在 Bash 呼叫中建立的拉取請求，其輸出超過約 30,000 個字元，未被連結。

當工作階段連結到多個拉取請求時，標籤會顯示計數，例如 `3 PRs`，按最需要關注的開啟拉取請求著色。開啟[查看面板](#peek-and-reply)以查看它們全部。

拉取請求編號按其狀態著色：

| 顏色 | 拉取請求狀態        |
| :- | :------------ |
| 黃色 | 等待檢查或審查，或檢查失敗 |
| 綠色 | 檢查通過且沒有審查阻止   |
| 紫色 | 已合併           |
| 灰色 | 草稿或已關閉        |

對於大多數任務，此欄是您收集結果的地方：當拉取請求編號變綠時審查並合併拉取請求。

<h3 id="peek-and-reply">
  查看和回覆
</h3>

在選定的行上按 `Space` 開啟查看面板。它開啟時顯示該行在終端邊緣截斷的句子，該句子是哪一個取決於工作階段的狀態：

* 等待您的工作階段：它正在詢問的確切問題，在回覆輸入上方
* 已完成的工作階段：其結果
* 工作中的工作階段：其完整狀態句子

任何連結到工作階段的拉取請求都會列在下方。對於等待您的工作階段，下方的一行，例如 `waiting 3m` 顯示它已等待多長時間，這是面板中唯一顯示的時間。行右邊緣的年齡是不同的數字：它從工作階段開始時計算。

大多數時候查看面板就足夠了，您不需要開啟完整記錄。

在 v2.1.207 之前，每次查看都以狀態句子和裸時間戳開啟，被阻止的工作階段的問題出現在它們下方，前綴為相同的時間戳第二次。

在查看面板中輸入回覆並按 `Enter` 將其發送到該工作階段。當工作階段詢問多選問題時，查看面板顯示選項，您可以按數字鍵選擇一個。對於其他被阻止的工作階段，按 `Tab` 用建議的回覆填充輸入，您可以在發送前編輯。使用 `!` 前綴回覆以發送 Bash 命令。

無法傳遞的回覆，因為背景服務無法連接或發送失敗，會被保存並在其程序再次啟動時作為其下一個提示發送到工作階段，錯誤訊息說回覆已保存。以 `!` 前綴的回覆不會被保存，因為保存的文字會作為純提示而不是 Bash 命令到達工作階段。

啟用[語音聽寫](/docs/zh-TW/voice-dictation)後，在回覆輸入有焦點時按住或點擊您的推送通話鍵以聽寫回覆，而不是輸入。同樣的方式也適用於 agent view 底部的分派輸入。

使用 `↑` 和 `↓` 查看相鄰工作階段而無需關閉面板，或按 `→` 附加。

<h3 id="attach-to-a-session">
  附加到工作階段
</h3>

在選定的行上按 `Enter` 或 `→` 附加。Agent view 被完整的互動工作階段替換。附加時，Claude 發佈您離開時發生的簡短回顧。

附加時，工作階段的行為與任何其他 Claude Code 工作階段相同：[命令](/docs/zh-TW/commands)、快捷鍵和功能都有效，下面列出的例外除外。

背景工作階段拒絕 `/install-github-app` 和 [`/mcp`](/docs/zh-TW/mcp) 設定列表，包括其驗證動作，無論您是附加還是從查看面板回覆。訊息會引導您到常規 `claude` 工作階段，而 `/mcp reconnect <server>`、`/mcp enable` 和 `/mcp disable` 仍然有效。

附加的工作階段始終以[全螢幕模式](/docs/zh-TW/fullscreen)呈現，無論您的 `tui` 設定如何，因為背景工作階段沒有終端滾動回溯可附加。使用 `PgUp`、`PgDn` 或滑鼠滾輪滾動，並按 `Ctrl+O` 進入記錄模式。您終端的原生滾動和 tmux 複製模式只顯示當前視口，與執行任何全螢幕應用程式時相同。

在空提示上按 `←` 或執行 `/exit` 以分離並返回 agent view。自 v2.1.198 起，無論您是從 agent view 開啟工作階段還是從 shell 執行 `claude attach <id>`，這都以相同的方式工作。

`Ctrl+Z` 也會分離但會回到您開始的地方：如果您從 agent view 附加則返回 agent view，或如果您執行 `claude attach` 則返回 shell。當對話框有焦點且不響應 `←` 時使用 `Ctrl+Z`。

`Ctrl+C` 在附加時保持其標準中斷行為：它取消執行中的回應或 `!` shell 命令，而不是分離。在空提示上按 `Ctrl+C` 兩次會分離，與任何工作階段中相同。

分離永遠不會停止背景工作階段：`←`、`Ctrl+Z`、`/exit` 和雙 `Ctrl+C` 或雙 `Ctrl+D` 都會讓它繼續執行。要從內部結束工作階段，執行 `/stop`。

在前景中執行的工作階段中，您在終端中啟動的工作階段而不是從 agent view 附加的工作階段，在空提示上按 `←` 會背景化它並開啟 agent view，預先選擇該行，因此您可以在不離開終端的情況下切換工作階段。同一個按鍵會分離附加的工作階段。

如果工具在您按 `←` 時執行，Claude Code 會等待最多約十秒鐘讓它完成然後背景化，回應在背景工作階段中繼續。再次按 `←` 以立即背景化，而不是等待。當進行中的工作無法轉移到背景工作階段時，`Background this session?` 對話會首先出現，與 [`/background`](#from-inside-a-session) 相同。

當 [subagents](/docs/zh-TW/sub-agents) 執行時，十秒限制不適用。Claude Code 會繼續等待，以便它們的工作能夠轉移，並在等待時顯示 `Still backgrounding after the current tool` 通知；再次按 `←` 以立即背景化而不等待，這會從頭開始重新啟動 subagents。在 v2.1.203 之前，等待在十秒後結束，執行中的 subagents 會在沒有警告的情況下從頭開始重新啟動。

該行會被建立，即使是從沒有對話歷史的全新工作階段，所以 `→` 會返回到它。在 v2.1.203 之前，當該行是唯一的行時，agent view 會在其下方顯示一個入門提示。

您可以在 `/config` 中使用 `leftArrowOpensAgents` 設定關閉此快捷鍵。

<h3 id="organize-the-list">
  組織列表
</h3>

Agent view 按狀態分組工作階段，需要輸入的工作階段在頂部，`Ready for review` 和 `Needs input` 在 `Working` 和 `Completed` 上方。這些組名稱不與上面的[狀態](#read-session-state)一一對應：當工作階段有開啟的拉取請求時，它會移動到 `Ready for review`，`Completed` 收集已完成、失敗和已停止的工作階段。

按 `Ctrl+S` 改為按目錄分組。您的選擇在執行中保存。

在組內：

* 按 `Ctrl+T` 將工作階段固定到頂部並[在閒置時保持其程序執行](#the-supervisor-process)
* 按 `Shift+↑` 或 `Shift+↓` 重新排序工作階段
* 按 `Ctrl+R` 重命名工作階段
* 在組標題上按 `Enter` 摺疊它

要移除工作階段，按 `Ctrl+X` 停止它，然後在兩秒內再次按 `Ctrl+X` 刪除它。在組標題上按 `Ctrl+X` 會在確認後刪除該組中的每個工作階段。

刪除會從 agent view 中移除工作階段。如果 Claude[為工作階段建立了 worktree](#how-file-edits-are-isolated)，刪除會移除該 worktree，包括其中的任何未提交的更改，因此在刪除前推送或提交您想保留的工作。您自己建立的 worktree 並在其中啟動工作階段的會保留在原地。對話記錄會保留在您的本機上，並且仍然可以通過 `claude --resume` 存取。

刪除永遠不會移除具有未推送到任何地方的提交的 worktree，或另一個執行中的工作階段聲稱或已鎖定的 worktree。Claude Code 會保留 worktree 和工作階段，頁腳會命名保留的路徑和原因。推送提交或關閉另一個工作階段，然後再次刪除。

刪除也會從[監督程序](#the-supervisor-process)的工作階段列表中清除工作階段，無論您使用 `Ctrl+X` 刪除還是從 shell 使用 [`claude rm`](#manage-sessions-from-the-shell) 刪除，因此移除在監督程序重新啟動時保存。在 v2.1.206 之前，在監督程序重新啟動或無法連接時移除工作階段會將其留在該列表中，下一個監督程序會重新啟動其程序並再次顯示該行。

不適合螢幕的已完成工作階段摺疊為 `… N more` 行。失敗和具有開啟拉取請求的工作階段始終保持可見。`Completed` 組填充在即時組之後剩餘的垂直空間，在短終端上，標題會壓縮為單行摘要，以便正在工作或需要輸入的工作階段保持可見。

<h3 id="filter-sessions">
  篩選工作階段
</h3>

在分派輸入中輸入以篩選而不是分派：

| 篩選                   | 顯示                                                     |
| :------------------- | :----------------------------------------------------- |
| `a:<name>`           | 執行命名代理的工作階段                                            |
| `s:<state>`          | 給定狀態中的工作階段，例如 `s:working`。也接受 `s:blocked` 表示等待您的所有工作階段 |
| `#<number>` 或 PR URL | 在該拉取請求上工作的工作階段                                         |
| 任何其他 URL             | 其第一個提示包含該 URL 的工作階段                                    |

<h3 id="keyboard-shortcuts">
  快捷鍵
</h3>

在 agent view 中按 `?` 查看每個快捷鍵。下表總結了它們。

| 快捷鍵                   | 動作                                |
| :-------------------- | :-------------------------------- |
| `↑` / `↓`             | 在行之間移動                            |
| `Enter`               | 附加到選定的工作階段，或如果輸入中有文字則分派           |
| `Space`               | 開啟或關閉選定工作階段的查看面板                  |
| `Shift+Enter`         | 分派並立即附加                           |
| `→`                   | 附加到選定的工作階段                        |
| `Alt+1`..`Alt+9`      | 附加到焦點工作階段目錄中的工作階段 1–9             |
| `Tab`                 | 在空輸入上，瀏覽所有 subagents。否則應用突出顯示的建議  |
| `Ctrl+S`              | 在狀態和目錄之間切換分組                      |
| `Ctrl+T`              | 固定或取消固定選定的工作階段                    |
| `Ctrl+R`              | 重命名選定的工作階段                        |
| `Ctrl+G`              | 在您的 `$VISUAL` 或 `$EDITOR` 中開啟分派提示 |
| `Ctrl+X`              | 停止工作階段；在兩秒內再次按以刪除它                |
| `Shift+↑` / `Shift+↓` | 重新排序選定的工作階段                       |
| `Esc`                 | 關閉查看面板、清除輸入或退出                    |
| `Ctrl+C`              | 清除輸入；按兩次退出                        |
| `?`                   | 顯示所有快捷鍵                           |

<h2 id="dispatch-new-agents">
  分派新代理
</h2>

您可以從 agent view 分派新的背景工作階段、將現有互動工作階段發送到背景，或直接從 shell 啟動一個。

<h3 id="from-agent-view">
  從 agent view
</h3>

在 agent view 底部的輸入框中輸入提示，然後按 `Enter` 啟動新的背景工作階段。工作階段從提示自動命名；稍後可以使用 `Ctrl+R` 重命名它。

工作階段稍後獲得的名稱也會出現在其行上，包括當您在該工作階段中[接受計畫](/docs/zh-TW/permission-modes#review-and-approve-a-plan)時 Claude 衍生的名稱。在 v2.1.207 之前，通過接受計畫命名的背景工作階段在 `/status` 中顯示該名稱，但在您自己重命名之前不會在其 agent-view 行上顯示。

將圖像粘貼到提示中以包含螢幕截圖或圖表與任務。

粘貼的文字超過 800 個字符或超過兩行會摺疊為 `[Pasted text #N]` 佔位符，以便輸入保持在一行；完整文字會在您分派時發送。要在分派前檢查或編輯摺疊的文字，請再次粘貼相同的文字，佔位符會展開回輸入框。在至少 90 列寬的終端上，粘貼後會在輸入下方出現 `paste again to expand` 提醒，持續幾秒鐘。在 v2.1.207 之前，再次粘貼相同的文字會新增第二個佔位符，而不是展開第一個。

前綴或提及提示的部分以控制工作階段如何啟動：

| 輸入                      | 效果                                                                                        |
| :---------------------- | :---------------------------------------------------------------------------------------- |
| `<agent-name> <prompt>` | 如果第一個單詞與自訂 [subagent](/docs/zh-TW/sub-agents) 名稱匹配，該 subagent 以工作階段的主代理身份執行，其 frontmatter 中的配置 |
| `@<agent-name>`         | 在提示中的任何地方提及自訂 subagent 以將其作為主代理執行                                                         |
| `@<repo>`               | 提及儲存庫以在那裡執行工作階段。請參閱[分派到特定目錄](#dispatch-to-a-specific-directory)以了解列出哪些儲存庫                 |
| `/<command>`            | 建議 [skills](/docs/zh-TW/skills) 和 [commands](/docs/zh-TW/commands) 作為提示分派                           |
| `! <command>`           | 執行 shell 命令作為背景工作而不是啟動 Claude 工作階段。該工作顯示為一行，您可以附加到、監視和分離                                  |
| `#<number>` 或拉取請求 URL   | 如果工作階段已在該 PR 上工作，選擇它而不是分派                                                                 |
| `Shift+Enter`           | 分派並立即附加到新工作階段                                                                             |

一小組命令在 agent view 本身中執行，而不是分派：

* `/exit` 和 `/quit` 關閉 agent view
* `/logout` 將您登出
* `/model` 設定[分派模型](#set-the-model)
* 自 v2.1.198 起，`/login` 開啟登入對話框，讓您無需附加到工作階段即可再次登入

Skills、您自己的命令和提示擴展內建命令（例如 `/init`）會作為新背景工作階段的第一個提示發送。其他內建命令會顯示 `attach to a session to run it` 提示。您輸入的所有內容都會保留在提示旁邊的輸入框中，以便您可以編輯它。在 v2.1.203 之前，提示會清除輸入，輸入的文字會遺失。

將重複任務打包為 [skill](/docs/zh-TW/skills)可讓您從 agent view 多次啟動相同的工作流程，無需重新輸入提示。

當相同的 `@name` 同時與 subagent 和同級儲存庫匹配時，subagent 優先。不帶 `@` 的第一個單詞形式也適用，因此以與您的 subagent 名稱之一匹配的單詞開頭的提示會分派該 subagent 而不是將該單詞視為純文本。當您想要明確時，請使用 `@` 形式，或以不同的單詞開頭提示以避免匹配。

<h4 id="dispatch-to-a-specific-directory">
  分派到特定目錄
</h4>

新工作階段在您開啟 agent view 的目錄中執行。要針對不同的目錄，請使用以下任何方法：

* 在該目錄中開啟 `claude agents`。
* 在父目錄中開啟 `claude agents`，並在提示中使用 `@<repo>` 提及子儲存庫。輸入 `@` 會列出這些目標：

  * 啟動目錄下一級的 Git 儲存庫
  * 您啟動的儲存庫的已註冊 [git worktrees](/docs/zh-TW/worktrees)，位於其目錄樹內，例如 Claude 在 `.claude/worktrees/` 下建立的那些，標記有其簽出的分支。使用 `git worktree add ../feature` 等方式在儲存庫外新增的 Worktrees 不會被列出
  * 任何已在列表中有工作階段的目錄

  名稱包含空格的目錄不會被列出。在 v2.1.203 之前，已註冊的 worktrees 不會被列出，因此分派到其中意味著從該 worktree 的目錄執行 `claude --bg`。
* 從 shell，`cd` 進入目錄並執行 `claude --bg "<prompt>"`。

當 agent view 按目錄分組時，突出顯示的行的目錄成為分派目標，因此您可以滾動到組並在其中分派，無需重新輸入路徑。

<h3 id="from-inside-a-session">
  從工作階段內部
</h3>

執行 `/background` 或其別名 `/bg` 將當前對話移動到背景工作階段。傳遞提示，例如 `/bg run the test suite and fix any failures`，以在分派前發送一個額外的指令。如果 Claude 在您執行 `/bg` 時正在回應，回應會在背景工作階段中繼續。

退出仍有背景工作執行的互動工作階段（例如 subagents、背景 shell 命令、工作流程或 [monitors](/docs/zh-TW/tools-reference#monitor-tool)）會顯示 `Background work is running` 對話而不是立即退出。自 v2.1.198 起，對話框提供 `Move to background and exit` 以及 `Exit anyway` 和 `Stay`。選擇它會以與 `/background` 相同的方式將工作階段移動到背景，然後返回您的 shell，因此可以繼續的工作會保持執行，工作階段會出現在 agent view 中。當 agent view [關閉](#turn-off-agent-view)時，不會顯示此選項。

從互動工作階段背景化會啟動一個新的進程，該進程從保存的對話恢復，進行中的工作會轉移到它：執行中的背景 shell 命令、背景化的 subagents、動態工作流程和您使用 [`/loop`](/docs/zh-TW/scheduled-tasks)建立的排定任務會轉移到背景工作階段並在那裡繼續執行。Subagent 與它啟動的所有內容一起移動，因此只有當所有工作都能轉移時它才會轉移，包括在 Windows 上。要停止進行中的工作而不是轉移它，請設定 [`CLAUDE_DISABLE_ADOPT=1`](/docs/zh-TW/env-vars#variables)環境變數；Claude Code 隨後會要求您在背景化前確認。

無法轉移的工作，例如執行中的 [monitor](/docs/zh-TW/tools-reference#monitor-tool)，會被停止。擁有監視器的背景化 subagent 會與它一起被停止。當任何此類工作執行時，Claude Code 會顯示 `Background this session?` 對話，以便您可以在停止前確認。

進入背景後，工作階段可以啟動新的 subagents、monitors 和背景命令，這些命令在稍後分離和重新附加時保持執行。

來自原始啟動的配置標誌會傳遞到背景化工作階段，因此其 MCP servers、settings 和備用模型保持有效：

* `--mcp-config` 和 `--strict-mcp-config`
* `--settings`
* `--add-dir`
* `--plugin-dir`
* `--fallback-model`
* `--allow-dangerously-skip-permissions`

您在工作階段期間使用 [`/add-dir`](/docs/zh-TW/permissions#additional-directories-grant-file-access-not-configuration)新增的目錄也會傳遞。

傳遞 `--allow-dangerously-skip-permissions` 會在背景化工作階段中保持 `bypassPermissions` 可達，但它不會授予任何新的權限。該模式仍然需要在任何工作階段使用它之前進行相同的一次性互動接受，如[Permission mode, model, and effort](#permission-mode-model-and-effort)中所述。

<h3 id="from-your-shell">
  從您的 shell
</h3>

傳遞 `--bg` 或其長形式 `--background` 啟動直接進入背景的工作階段：

```bash theme={null}
claude --bg "investigate the flaky SettingsChangeDetector test"
```

提示是位置引數，不是 `-p` 值。自 v2.1.198 起，將 `--bg` 與 `-p` 或 `--print` 結合會在建立任何工作階段前被拒絕並出現錯誤，因為 `--print` 永遠不會啟動 `claude agents` 附加到的互動工作階段。

要執行特定 subagent 作為工作階段的主代理，將 `--bg` 與 `--agent` 結合：

```bash theme={null}
claude --agent code-reviewer --bg "address review comments on PR 1234"
```

傳遞 `--name` 以在 agent view 中設定工作階段的顯示名稱，而不是自動生成的名稱：

```bash theme={null}
claude --bg --name "flaky-test-fix" "investigate the flaky SettingsChangeDetector test"
```

背景化後，Claude 列印工作階段的短 ID 和管理它的命令。當主機背景工作階段的服務尚未執行時，`--bg` 可能會先列印 `Starting background service…`。當您傳遞 `--name` 時，名稱會出現在短 ID 之後：

```text theme={null}
backgrounded · 7c5dcf5d · flaky-test-fix
  claude agents             list sessions
  claude attach 7c5dcf5d    open in this terminal
  claude logs 7c5dcf5d      show recent output
  claude stop 7c5dcf5d      stop this session
```

<h4 id="run-a-shell-command">
  執行 shell 命令
</h4>

要執行 shell 命令作為背景工作而不是 Claude 工作階段，請在 agent view 分派輸入的第一個字符中輸入 `!`。`!` 顯示為前綴，您在其後輸入的所有內容都是命令。以下示例從 agent view 輸入框分派 `pytest -x`：

```text theme={null}
! pytest -x
```

按 `Enter` 啟動工作。相同的工作也可以直接從您的 shell 使用 `--exec` 啟動：

```bash theme={null}
claude --bg --exec 'pytest -x'
```

該命令作為 PTY 支持的工作執行，並在 agent view 中顯示為一行，其最近的輸出行作為其狀態。shell 工作執行命令代替 Claude，因此不調用任何模型，輸出不發送到任何工作階段。

要查看輸出，附加到該行，按 `Space` 以在不附加的情況下查看，或從您的 shell 執行 `claude logs <id>`。捕獲的輸出保留在記憶體中，不寫入磁碟。該行及其輸出在命令退出後約五分鐘自動清理，因此如果您需要結果，請在那之前讀取它。

<h3 id="how-file-edits-are-isolated">
  檔案編輯如何隔離
</h3>

每個背景工作階段，無論是從 agent view、`/bg` 或 `claude --bg` 啟動，都在您的工作目錄中啟動。編輯檔案前，Claude 將工作階段移動到 `.claude/worktrees/` 下的隔離 [git worktrees](/docs/zh-TW/worktrees)中，因此並行工作階段可以讀取相同的檢出，但每個都寫入自己的。

Claude 在以下情況下跳過 worktree：

* 工作階段已在連結的 git worktree 內，無論 Claude 是在 `.claude/worktrees/` 下建立它，還是您使用 `git worktree add` 在其他地方建立它
* 工作目錄不是 git 儲存庫且沒有配置 [`WorktreeCreate` hook](/docs/zh-TW/hooks#worktreecreate)
* 寫入在工作目錄外

要為 git worktrees 不實用的儲存庫關閉 worktree 隔離，請將 [`worktree.bgIsolation`](/docs/zh-TW/settings#worktree-settings)設定為 `"none"`。背景工作階段隨後直接編輯您的工作副本，無需先移動到 worktree。將設定新增到專案的 `.claude/settings.json`：

```json theme={null}
{
  "worktree": {
    "bgIsolation": "none"
  }
}
```

在 git 儲存庫外，工作階段直接寫入工作目錄，彼此之間不隔離，因此避免分派編輯相同檔案的並行工作階段。如果您使用不同的版本控制系統，請配置 [`WorktreeCreate` hook](/docs/zh-TW/worktrees#non-git-version-control)，Claude 會以與 git 相同的方式隔離編輯。

當 hook 在不是 git 儲存庫的目錄中失敗時，工作階段會跳過該目錄的隔離並就地編輯工作目錄。在 git 儲存庫內，寫入會保持被阻止，直到工作階段隔離。在 v2.1.203 之前，處於該狀態的背景工作階段無法編輯任何檔案：每次寫入都被拒絕，直到它隔離，hook 永遠無法隔離該目錄。

刪除工作階段會移除或保留 Claude 為其建立的 worktree，取決於您如何刪除它以及 worktree 包含的內容：

* 在 agent view 中使用 `Ctrl+X` 兩次刪除會移除 worktree，包括任何未提交的更改，因此請先提交您想保留的更改。
* 從 shell 使用 [`claude rm`](#manage-sessions-from-the-shell)刪除會保留具有未提交更改的 worktree，以及其工作階段行。
* 兩種方式都不會移除具有未推送到任何地方的提交的 worktree：worktree 會[與其工作階段一起保留](#organize-the-list)，輸出會命名保留的路徑和原因。
* 您自己建立並在其中啟動工作階段的 worktree 無論如何都會保留在原位。

要找到工作階段的 worktree 路徑，查看工作階段或附加並檢查其工作目錄。

[subagent](/docs/zh-TW/sub-agents)背景工作階段生成的會繼承工作階段的工作目錄，因此其檔案編輯會進入工作階段的 worktree 而不是您的工作副本。要給 subagent 其自己的單獨 worktree，請在其 frontmatter 中設定 [`isolation: worktree`](/docs/zh-TW/sub-agents#supported-frontmatter-fields)或在生成它時傳遞 `isolation: "worktree"`。

自 v2.1.198 起，在隔離 worktree 中隔離其程式碼更改的背景工作階段也會提交、推送其自己的分支，並開啟草稿拉取請求而無需停止詢問。當拉取請求開啟時，[`#N` 標籤](#pull-request-status)會出現在其行上。它永遠不會推送到 `main` 或 `master`，永遠不會強制推送或合併，並且當您告訴它不要開啟拉取請求或儲存庫沒有遠端時會跳過拉取請求。

編輯未自行隔離的檢出的工作階段在提交或切換分支前仍會詢問。這適用於隔離設定為 `"none"` 時、worktree 移動失敗時，或工作階段在已存在的 worktree 內啟動時。

<h3 id="set-the-model">
  設定模型
</h3>

agent view 標題中顯示的模型名稱是分派預設值。您從輸入啟動的新工作階段使用此模型，這來自您使用者設定中的 [`model` setting](/docs/zh-TW/settings#available-settings)。通過在 [`/model` picker](/docs/zh-TW/model-config)中選擇模型來設定它，或直接編輯設定。

要為整個 agent view 工作階段覆蓋分派預設值，請在開啟 agent view 時傳遞 `--model`。請參閱[Permission mode, model, and effort](#permission-mode-model-and-effort)。

要從 agent view 內部更改分派預設值，請在分派輸入中輸入 `/model` 後跟模型名稱，然後按 `Enter`。標題會更新以顯示該模型，並帶有 `(session)` 標記，之後您分派的工作階段會使用它。輸入 `/model default` 以清除覆蓋並返回分派預設值。此覆蓋會持續到當前 `claude agents` 執行的其餘部分，不會寫入您的設定檔案。以下示例在 Opus 上分派一個工作階段，在 Sonnet 上分派下一個：

```text theme={null}
/model opus
refactor auth
/model sonnet
run the test suite
```

每個背景工作階段可以在不同的模型上執行。要為一個工作階段覆蓋它：

* 從 shell，使用 `claude --bg` 傳遞 `--model`。
* 附加到執行中的工作階段並執行 `/model` 以切換：從選擇器中選擇，或輸入 `/model <name>`，會保存為您的新工作階段預設值，除非您在選擇器中按 `s` 進行僅工作階段切換。如果工作階段被重新生成，僅工作階段切換會持續。
* 分派一個 [subagent](/docs/zh-TW/sub-agents)，其 frontmatter 設定 `model` 欄位。

<h3 id="permission-mode-model-and-effort">
  Permission mode, model, and effort
</h3>

背景工作階段從它執行的目錄讀取其 [settings](/docs/zh-TW/settings)，就像您在那裡啟動了 `claude` 一樣。這包括專案設定中的 [`env` values](/docs/zh-TW/settings#available-settings)，因此在那裡設定的 `ANTHROPIC_MODEL` 或提供者變數適用於該目錄中的背景工作階段。

雲提供者選擇，例如 `CLAUDE_CODE_USE_BEDROCK` 或 `CLAUDE_CODE_USE_VERTEX`，以及 `ANTHROPIC_DEFAULT_*_MODEL` 別名遵循分派工作階段的 shell。如果您在該 shell 中匯出 [`CLAUDE_CODE_EXTRA_BODY`](/docs/zh-TW/env-vars)請求體覆蓋，它也會以相同方式到達工作階段。在 v2.1.206 之前，背景工作者忽略了 shell 匯出的 `CLAUDE_CODE_EXTRA_BODY`。

如果您在分派 shell 中匯出閘道 `ANTHROPIC_BASE_URL`，它也會到達工作階段，以及 `ANTHROPIC_CUSTOM_HEADERS`，當監督者使用相同的閘道環境執行且工作階段在您分派的目錄中執行或是您自己的工作階段使用 `←` 或 `/background` 背景化時。這是當第一個開啟 agent view 或分派背景工作階段的 shell 是閘道 shell 時的正常情況。使用 `@repo` 或 `--cwd` 分派到不同目錄不會攜帶 shell 的閘道；該專案的 [settings](/docs/zh-TW/settings)提供端點。請參閱[the supervisor process](#the-supervisor-process)以了解背景工作階段如何源自提供者設定和認證。

[permission mode](/docs/zh-TW/permissions)取決於您如何啟動工作階段。使用 `/bg` 或 `←` 背景化現有工作階段會保持當前權限模式，因此您切換到 `acceptEdits` 或 `auto` 的工作階段在分離後仍保持該模式。從 agent view 輸入分派或從 shell 執行 `claude --bg` 使用該目錄設定中的 `defaultMode`，或分派的 [subagent 的 frontmatter](/docs/zh-TW/sub-agents#supported-frontmatter-fields)中的 `permissionMode`。

背景工作階段啟動時的權限模式、模型和努力，以及它攜帶的 [configuration flags](#from-inside-a-session)，在監督者稍後 [stops and restarts](#the-supervisor-process)其進程時都會持續。您使用 `claude --bg --dangerously-skip-permissions` 或 `claude --bg --permission-mode bypassPermissions` 啟動的工作階段在該重新啟動後保持 `bypassPermissions`，而不是回退到目錄的 `defaultMode`，並且您使用 `/model` 或 `/effort` 在工作階段中途更改的模型或努力會被保留。

工作階段從 [`effortLevel` setting](/docs/zh-TW/settings#available-settings)而不是從 `--effort` 或 `/effort` 取得的努力不會在分派時固定：為工作階段啟動的每個進程都會再次讀取設定，因此編輯 `settings.json` 中的 `effortLevel` 會到達您使用 `←` 或 `/bg` 背景化的工作階段及其稍後的重新啟動。在 v2.1.203 之前，背景化工作階段會記錄其設定衍生的努力，就像您傳遞了 `--effort` 一樣，因此稍後的 `effortLevel` 編輯永遠無法到達它。

您使用 [`/rename`](/docs/zh-TW/commands) 或 `Ctrl+R` 設定的名稱也會在該重新啟動時持續，因此 [`claude --resume <name>`](/docs/zh-TW/sessions#name-your-sessions) 仍會解析工作階段。在 v2.1.202 之前，重新啟動會將工作階段還原為分派時的名稱，新名稱停止解析。

要為您從 agent view 分派的每個工作階段設定預設值，請在開啟它時傳遞 `--permission-mode`、`--model`、`--effort` 或 `--agent` 中的任何一個：

```bash theme={null}
claude agents --permission-mode plan --model opus --effort high
```

`--agent` 設定 [subagent](/docs/zh-TW/sub-agents)，當分派提示未使用 `@name` 或作為第一個單詞命名時使用。如果設定了 [`agent` setting](/docs/zh-TW/settings#available-settings)，則預設為該設定，否則為內建的全能 `claude` 代理。在分派輸入中命名 subagent 會覆蓋兩者。

`claude agents` 也接受 `--dangerously-skip-permissions` 作為 `--permission-mode bypassPermissions` 的簡寫，以及 `--allow-dangerously-skip-permissions` 以在每個分派工作階段的 `Shift+Tab` 循環中提供 `bypassPermissions`，而不是以該模式啟動。兩者都與 [top-level CLI flags](/docs/zh-TW/cli-reference)相符。

活動預設值出現在分派輸入下方的頁腳中。

沒有這些標誌，工作階段使用該目錄設定中的 `defaultMode` 或分派的 [subagent 的 frontmatter](/docs/zh-TW/sub-agents#supported-frontmatter-fields)中的 `permissionMode`，以及 agent view 標題中顯示的模型。

使用 `bypassPermissions` 與 `claude --bg --permission-mode` 被拒絕，直到您通過執行 `claude --dangerously-skip-permissions` 一次互動式接受了繞過免責聲明，因為該模式讓您未監視的工作階段無需批准即可行動。將 `--dangerously-skip-permissions` 或 `--permission-mode bypassPermissions` 傳遞給 `claude agents` 會在您之前未接受時顯示相同的免責聲明，接受會將 `bypassPermissions` 應用於您從檢視啟動的工作階段。傳遞 `--allow-dangerously-skip-permissions` 也會顯示相同的免責聲明，接受會在這些工作階段的 `Shift+Tab` 循環中提供 `bypassPermissions`，而不是以它啟動它們。

<h3 id="settings-plugins-and-mcp-servers">
  Settings, plugins, and MCP servers
</h3>

Agent view 接受與 `claude` 相同的配置標誌，用於載入 settings、plugins、MCP servers 和額外目錄。每個標誌適用於 agent view 本身，並傳遞給您從它分派的每個工作階段，因此以這種方式載入的 plugin 或 MCP server 在這些工作階段中也可用。

| 標誌                                                                                                  | 效果                                             |
| :-------------------------------------------------------------------------------------------------- | :--------------------------------------------- |
| [`--settings <file-or-json>`](/docs/zh-TW/settings)                                                      | 覆蓋 agent view 和分派工作階段的 settings                |
| [`--add-dir <path>`](/docs/zh-TW/permissions#additional-directories-grant-file-access-not-configuration) | 授予對額外目錄的檔案存取權限                                 |
| [`--plugin-dir <path>`](/docs/zh-TW/plugins)                                                             | 從本地目錄載入 plugin                                 |
| [`--mcp-config <file-or-json>`](/docs/zh-TW/mcp)                                                         | 從配置檔案或 JSON 字符串載入 MCP servers                  |
| `--strict-mcp-config`                                                                               | 僅使用來自 `--mcp-config` 的 MCP servers，忽略其他 MCP 配置 |

每個值重複 `--add-dir`、`--plugin-dir` 或 `--mcp-config` 一次。空格分隔的形式，例如 `--add-dir a b c`，不支援與 `claude agents` 一起使用。

以下示例使用 settings 覆蓋和一個額外目錄開啟 agent view：

```bash theme={null}
claude agents --settings ./ci-settings.json --add-dir ../shared-lib
```

<h2 id="manage-sessions-from-the-shell">
  從 shell 管理工作階段
</h2>

每個背景工作階段都有一個短 ID，您可以從 shell 使用。當您使用 `claude --bg` 啟動工作階段時會列印該 ID，每個工作階段的 ID 是其在 `~/.claude/jobs/` 下的目錄名稱。這些命令對於指令碼編寫或當您不想開啟 agent view 時很有用。

| 命令                           | 目的                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| :--------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `claude agents`              | 開啟 agent view                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `claude agents --cwd <path>` | 開啟 agent view，範圍限定於在 `<path>` 下啟動的工作階段                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `claude agents --json`       | 將即時工作階段列印為 JSON 陣列並結束：每個即時工作階段，加上仍在執行或被阻止的背景工作階段，即使其程序已結束。新增 `--all` 以同時包含已完成的背景工作階段。每個項目都有 `cwd`、`kind` 和 `startedAt`。背景項目也有 `id`（可與 `claude attach`/`logs`/`stop` 搭配使用）和 `state`：`working`、`blocked`、`done`、`failed` 或 `stopped` 之一。`pid` 和 `status` 僅在程序執行時出現，加上當 status 為 `waiting` 時的 `waitingFor`，其說明工作階段被阻止的原因，例如 `permission prompt` 或 `input needed`；`sessionId` 和 `name` 在設定時出現。互動項目您從未命名的會帶有預設 `name`，由其工作目錄的名稱加上兩個字符後綴組成，例如 `my-app-3f`。與 `--cwd <path>` 結合以篩選 |
| `claude attach <id>`         | 在此終端中附加到工作階段                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `claude logs <id>`           | 列印工作階段的最近輸出                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `claude stop <id>`           | 停止工作階段。也接受 `claude kill`                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `claude respawn <id>`        | 重新啟動工作階段（執行中或已停止），保持其對話完整，例如用於採用更新的 Claude Code 二進位檔案                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `claude respawn --all`       | 重新啟動每個執行中的工作階段，例如一次將所有工作階段移至更新的 Claude Code 二進位檔案                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `claude rm <id>`             | 從清單中移除工作階段。如果沒有未提交的變更且沒有未推送到任何地方的提交，會移除 Claude 為工作階段建立的 worktree；否則會保留工作階段，命令會列印 worktree 路徑和原因，以便您解決並再次執行 `claude rm`。保留您自己建立的 worktree。對話記錄會保留在您的本機上，並可透過 `claude --resume` 繼續使用                                                                                                                                                                                                                                                                                         |
| `claude daemon status`       | 列印 [supervisor](#the-supervisor-process)的狀態、版本、socket 目錄和 worker 計數                                                                                                                                                                                                                                                                                                                                                                                                        |
| `claude daemon stop --any`   | 停止 supervisor 程序及其託管的背景工作階段。傳遞 `--keep-workers` 以保持背景工作階段執行中，以便下一個 supervisor 可以重新連接到它們。下一個 `claude agents` 或 `claude --bg` 會啟動全新的 supervisor                                                                                                                                                                                                                                                                                                                              |

<h2 id="how-background-sessions-are-hosted">
  背景工作階段如何被託管
</h2>

agent view 中列出的每個工作階段都被視為背景工作階段，無論您目前是否連接到它。相比之下，直接執行 `claude` 啟動的工作階段與該終端相關聯，並在終端關閉時結束，除非您[將其發送到背景](#from-inside-a-session)。

<h3 id="the-supervisor-process">
  監督程序
</h3>

背景工作階段由每個使用者的監督程序託管，與您的終端和 agent view 分開。監督程序在您第一次背景化工作階段或開啟 agent view 時自動啟動，您不直接管理它。

當更新已替換或移除執行中的 Claude Code 程序啟動的二進位檔案時，該程序會從另一個已安裝的副本（例如已安裝的 `claude` 啟動器或磁碟上的最新版本）啟動監督程序。

監督程序保持一個預熱的工作程序就緒，以便來自 agent view 或 `claude --bg` 的分派無需冷啟動的延遲即可開始。當您分派時，監督程序將預熱的工作程序分配給您的工作階段，將該工作階段的目錄、設定和認證應用於它，然後為下一次分派啟動替代程序。如果沒有可用的健康預熱工作程序，監督程序會改為啟動新程序。

監督程序及其工作階段使用與互動工作階段相同的認證進行身份驗證，並且除了模型 API 外不進行額外的網路連接。提供者選擇變數（例如 `CLAUDE_CODE_USE_BEDROCK` 和 `ANTHROPIC_DEFAULT_*_MODEL` 別名）從分派每個工作階段的 shell 中讀取，並應用於其工作程序。

分派 shell 的 `PATH` 以相同方式應用於工作程序，因此工作階段執行的 shell 命令會找到您的終端所擁有的相同工具。在 v2.1.203 之前，背景工作階段保持啟動監督程序的 shell 的 `PATH`，因此自那時以來添加到您 `PATH` 的工具可能會遺失，最常見的是在 Windows 上。

背景工作階段不會繼承閘道端點變數（例如 `ANTHROPIC_BASE_URL` 或等效的 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 基礎 URL 變數）來自啟動監督程序的 shell。如果您分派的 shell 中未匯出閘道，工作階段會使用您的儲存認證和專案目錄的[設定](/docs/zh-TW/settings)中 `env` 區塊中的任何 `env` 值。要在專案中指向每個工作階段到 [LLM 閘道](/docs/zh-TW/llm-gateway)，請在該專案的 `.claude/settings.json` `env` 區塊中設定 `ANTHROPIC_BASE_URL`。

在您分派的 shell 中匯出的閘道 `ANTHROPIC_BASE_URL` 會到達該工作階段的工作程序。`ANTHROPIC_CUSTOM_HEADERS` 和與它們一起匯出的認證會與它一起轉發。這發生在監督程序從具有相同閘道的環境啟動時。監督程序從開啟 agent view 或分派背景工作階段的第一個 shell 捕獲其環境，因此從閘道 shell 啟動會給予它該環境。轉發也僅適用於分派到您分派的目錄或使用 `←` 或 `/background` 從您自己的工作階段背景化的工作階段：使用 `@repo` 或 `--cwd` 分派到不同目錄不會攜帶 shell 的閘道，該專案的 `settings.json` `env` 區塊改為提供端點。當監督程序的環境攜帶不同的閘道或沒有閘道時，工作程序會針對預設端點保持您的儲存認證，而不是混合一個環境的認證與另一個環境的端點。在 v2.1.203 之前，分派 shell 的 `ANTHROPIC_BASE_URL` 被丟棄，而與它一起匯出的 `ANTHROPIC_API_KEY` 被保留，因此閘道的金鑰被發送到預設端點，每個請求都失敗並出現 401。

轉發的端點僅適用於該活動程序，永遠不會寫入磁碟。當監督程序停止閒置工作階段並稍後重新啟動它時，重新啟動的程序會再次從您的設定中讀取其端點：使用閘道 `ANTHROPIC_AUTH_TOKEN` 時，它會回退到您的儲存認證，使用閘道發行的 `ANTHROPIC_API_KEY` 時，在設定中設定閘道之前可能無法進行身份驗證。

每個背景工作階段都是其自己的 Claude Code 程序，由監督程序管理而不是與您的終端相關聯。正在主動工作、等待您的輸入或已連接終端的工作階段保持其程序執行。執行中的背景 shell 命令、子代理、動態工作流程或監視器計為主動工作，因此長時間執行的程序（例如開發伺服器）會保持工作階段活躍。

一旦工作階段完成並在未附加的情況下閒置約一小時，監督程序停止其程序以釋放資源。您使用 `Ctrl+T` [釘選](#organize-the-list)的工作階段不受此限制，在閒置時保持其程序執行。無論如何，文字記錄和狀態保留在磁碟上，下次您附加、查看或回覆已停止的工作階段時，監督程序從中斷的地方啟動新程序。當每個工作階段都完成且沒有終端連接時，監督程序本身退出，並在下次您需要它時再次啟動。

工作階段本身在頂層啟動的背景工作會在其程序被停止、重新啟動或更新時交付，包括在 Windows 上。為該工作階段啟動的下一個程序會接收它們：

* 在此期間完成的背景 shell 命令會報告為已完成及其輸出
* 動態工作流程會從中斷的地方恢復
* [背景子代理](/docs/zh-TW/sub-agents#run-subagents-in-foreground-or-background)會從其自己的文字記錄恢復

自 v2.1.198 起，交付涵蓋所有三項。在 v2.1.198 之前，它只涵蓋 shell 命令和工作流程，因此背景子代理會與程序一起停止，並在下次喚醒時報告為失敗。

其狀態僅存在於程序內部的工作會與它一起停止，而不是被交付。那是子代理啟動的 shell 命令，恢復的子代理可以再次啟動，以及執行中的[監視器](/docs/zh-TW/tools-reference#monitor-tool)，其事件流無法移動到另一個程序。

刪除工作階段會停止它交付的所有內容。要讓所有工作階段的背景工作與程序一起停止而不是被交付，請將 [`CLAUDE_CODE_DISABLE_BG_EXIT_HANDOFF`](/docs/zh-TW/env-vars#variables) 環境變數設定為 `1`。

重新啟動的程序會找到[移入 worktree](#how-file-edits-are-isolated) 中途任務的工作階段的對話：當文字記錄不在工作階段啟動的位置時，Claude Code 也會在儲存庫的已註冊 worktrees 下查看。在 v2.1.207 之前，在其程序停止後從 agent view 重新開啟該工作階段可能會顯示只有其原始提示的空白對話，文字記錄仍完整保存在磁碟上；在 v2.1.207 或更新版本上再次開啟工作階段會恢復它。

如果重新啟動的工作階段回來時只顯示其原始提示，因為 Claude Code 誤讀其文字記錄為空，對話文字記錄會被重命名為 `.orphaned-` 後綴，而不是被刪除，因此它會保留在您的機器上。

按 `←` 留下的空白列，從未被給予提示，會在約五分鐘後被完全移除，以便列表自動清除。使用 `claude --bg` 啟動的工作階段和等待設定提示（例如信任對話）的工作階段不會以這種方式被移除。

當主機記憶體不足時，監督程序首先停止閒置的未釘選工作階段，只有在釋放任何資源時才停止閒置的釘選工作階段。

監督程序監視磁碟上已安裝的 Claude Code 二進位檔案，並在常規[自動更新程序](/docs/zh-TW/setup#auto-updates)替換它後重新啟動到新版本。這是本地檔案監視，不是網路檢查。背景工作階段是分離的程序，因此它們在重新啟動期間繼續執行，新監督程序重新連接到它們。閒置的釘選工作階段也會就地重新啟動到新版本，以便它在您不重新附加的情況下獲取更新。

一旦新監督程序接管，它也會在短暫延遲後在背景中一次重新啟動幾個剩餘的閒置工作階段到新版本，該延遲讓在重新啟動期間連接的終端有時間先重新連接。正在工作、等待您的輸入或已連接終端的工作階段不會被中斷；它會在其程序下次重新啟動時移動到新版本。在 v2.1.206 之前，監督程序每分鐘只移動幾個閒置工作階段到新版本，因此工作階段在更新後可能會繼續執行舊版本一段時間。

這些重新啟動只會將工作階段移動到較新版本。執行比工作階段程序啟動時的版本更舊的 Claude Code 版本的監督程序會單獨保留該程序；工作階段會繼續執行較新版本，直到較新的監督程序接管。

在監督程序重新啟動工作階段時執行 `claude attach`，無論是為了更新、停滯或遷移，會等待替換程序而不是失敗。狀態行（例如 `Agent is updating to the new Claude Code…`）會命名它正在等待的內容並計算經過的秒數，命令會在工作階段準備好時立即連接。大約 60 秒後，它會停止等待並報告錯誤。在 v2.1.205 之前，`claude attach` 在幾秒後停止重試並列印錯誤，而工作階段仍在重新啟動。

<h3 id="where-state-is-stored">
  狀態存儲位置
</h3>

工作階段狀態存儲在您的 Claude Code 配置目錄下。如果您設定 [`CLAUDE_CONFIG_DIR`](/docs/zh-TW/env-vars)，監督程序改用該目錄而不是 `~/.claude`，並作為具有其自己工作階段的單獨實例執行。

| 路徑                               | 內容                               |
| :------------------------------- | :------------------------------- |
| `~/.claude/daemon.log`           | 監督程序日誌                           |
| `~/.claude/daemon/roster.json`   | 執行中的背景工作階段列表，用於在重新啟動後重新連接        |
| `~/.claude/jobs/<id>/state.json` | 在 agent view 中顯示的每個工作階段狀態        |
| `~/.claude/jobs/<id>/tmp/`       | 每個工作階段的暫存目錄。寫入此處不會提示權限。工作階段刪除時移除 |

每個背景工作階段都設定了 `CLAUDE_JOB_DIR` 環境變數，指向其 `~/.claude/jobs/<id>` 目錄，因此工作階段執行的 shell 命令可以將臨時檔案寫入 `$CLAUDE_JOB_DIR/tmp`，而不會與平行工作階段衝突。

要在不直接讀取檔案的情況下檢查此狀態，請執行 `claude daemon status`。它報告監督程序是否可達、其程序 ID 和版本、socket 目錄，以及有多少背景工作階段處於活動狀態。

該命令也會在執行中的監督程序版本與您叫用的 `claude` 版本不同時發出警告，這會在監督程序尚未重新啟動到新版本的更新後發生。警告會顯示兩個版本，並告訴您執行 `claude daemon stop --any` 以採用新版本。當 Claude Code 安裝為作業系統服務時，建議的命令是 `claude daemon stop`，不帶該旗標。

工作階段在該版本不匹配時保持完整：較舊的 Claude Code 版本更新工作階段的 `state.json` 時會保留它不識別的欄位，並保持工作階段列出。在 `roster.json` 中的工作階段列表遵循相同規則：較舊的版本在重寫時會保留較新版本寫入的欄位，因此由較新版本啟動的工作階段保持可達，並在監督程序重新啟動後繼續接受輸入。在 v2.1.200 之前，較舊的版本在重寫時可能會丟棄這些欄位。

在 Windows 上，當 daemon 的 pipe-key 檔案被鎖定或無法讀取時，`claude daemon status` 會顯示基礎檔案錯誤，而不是報告通用連接失敗。

<h3 id="turn-off-agent-view">
  關閉 agent view
</h3>

要完全關閉背景代理和 agent view，將 `disableAgentView` [設定](/docs/zh-TW/settings)設為 `true` 或設定 `CLAUDE_CODE_DISABLE_AGENT_VIEW` 環境變數。管理員可以通過[受管設定](/docs/zh-TW/permissions#managed-settings)強制執行此操作。

<h2 id="troubleshooting">
  故障排除
</h2>

<h3 id="claude-agents-lists-subagents-instead-of-opening-agent-view">
  `claude agents` 列出子代理而不是開啟代理檢視
</h3>

如果 `claude agents` 列印計數後跟著您設定的子代理，然後退出，代理檢視在您的環境中不可用。執行 `claude update` 以安裝最新版本。

如果更新後代理檢視仍未開啟，請檢查它是否已被設定或環境變數[關閉](#turn-off-agent-view)。

<h3 id="agent-view-opens-with-no-sessions">
  Agent view 開啟時沒有工作階段
</h3>

在您分派第一個工作階段之前，agent view 會顯示空的區段標題，每個標題下方有描述，以及輸入上方有一行說明，代替工作階段清單。在底部的輸入框中輸入提示並按 `Enter` 以分派您的第一個工作階段。

<h3 id="backgrounding-shows-a-background-this-session-dialog">
  背景化顯示 `Background this session?` 對話
</h3>

如果按 `←` 將當前工作階段放在背景中顯示 `Background this session?` 對話，工作階段有進行中的工作無法轉移到背景工作階段，例如執行中的 [monitor](/docs/zh-TW/tools-reference#monitor-tool)，Claude Code 不會無聲地停止它。對話命名將被停止的工作，並分別計算轉移的任務。執行 `/tasks` 以查看正在執行的內容，然後確認以無論如何背景化或選擇 `Stay` 讓工作先完成。請參閱[從工作階段內部](#from-inside-a-session)以了解哪些任務類型轉移，哪些被停止。

<h3 id="prompt-rejected-as-too-short">
  提示被拒絕為過短
</h3>

分派輸入期望任務描述，而不是對話開場白。短於四個字元的提示會被拒絕並顯示 `Too short` 提示，以便隨意按鍵不會啟動工作階段。描述您希望工作階段執行的操作，例如 `investigate the flaky checkout test`。

<h3 id="sessions-show-as-failed-after-shutdown">
  工作階段在關閉後顯示為失敗
</h3>

關閉或重新啟動您的機器會停止執行中的背景工作階段，因此當您下次開啟 agent view 時，它們會顯示為失敗。附加、查看或回覆任何工作階段，工作階段會從中斷的地方重新啟動。

睡眠單獨不會導致這種情況。工作階段在睡眠期間會被保留，監督程序在喚醒時會重新連接到它們。

<h3 id="opening-a-session-says-the-conversation-is-already-open">
  開啟工作階段時顯示對話已開啟
</h3>

開啟一個已停止的列，其對話也由另一個執行中的非互動式 Claude Code 程序持有，例如同一對話的背景工作程序仍在關閉中，會顯示 `This conversation is already open in another running Claude session` 而不是啟動該列的程序，因為兩個程序無法寫入同一個文字記錄。在已經持有對話開啟的工作階段中回覆，或退出它並再次開啟該列。您在拒絕嘗試時輸入的回覆不會遺失；它會在工作階段下次啟動時發送。

在 v2.1.203 之前，此狀態無論如何都會啟動第二個程序。該程序以 `currently running as a background agent` 錯誤退出，該列顯示為失敗。

<h3 id="a-session-fails-before-starting-with-a-possibly-low-memory-note">
  工作階段在啟動前失敗，並出現 `possibly low memory` 註記
</h3>

自 v2.1.199 起，當背景工作階段的程序在完成啟動前退出，且主機記憶體不足時，該列的狀態會命名退出並新增 `possibly low memory — free some up and retry`。較早的版本只顯示此失敗的裸退出原因。

該註記是一個假設，而不是確認的原因。Claude Code 只在程序無聲退出時新增它，沒有寫入錯誤，也沒有被信號停止，且主機在該時刻報告記憶體不足。當程序在退出前確實寫入了錯誤時，該列會改為顯示該錯誤。

釋放機器上的記憶體，然後附加、查看或回覆該列，監督程序會為工作階段啟動新的程序。當記憶體保持不足時，監督程序也會[停止閒置工作階段](#the-supervisor-process)以自行釋放資源。

<h3 id="agent-view-says-the-background-service-did-not-respond">
  Agent view 表示背景服務未回應
</h3>

如果附加、查看或 `claude logs` 報告背景服務未回應，監督程序可能已停止回應。停止它並讓下一個 `claude agents` 啟動新的程序。若要在重新啟動期間保持背景工作階段執行，請傳遞 `--keep-workers`：

```bash theme={null}
claude daemon stop --any --keep-workers
```

新的監督程序會重新連接到執行中的工作階段。如果沒有 `--keep-workers`，該命令也會結束背景工作階段。`--any` 旗標確認您想要停止按需啟動的監督程序，而不是作為已安裝的服務啟動的程序，這是預設值。

啟動但無法接受連接的監督程序會自行退出並釋放其鎖定，因此下一個 `claude agents` 會啟動新的程序，無需此手動停止。上述步驟適用於執行中的監督程序停止回應的情況。

在 Windows 上，如果監督程序未回應停止請求，該命令會列印其程序 ID。使用 `taskkill /PID <pid>` 結束該程序以完成復原。當您傳遞 `--keep-workers` 時，背景工作階段仍會被保留。

<h3 id="dispatch-fails-with-could-not-resolve-authentication-method">
  背景分派失敗，出現 `Could not resolve authentication method`
</h3>

如果背景分派失敗，出現 `Could not resolve authentication method`，而互動式工作階段正常驗證，接收分派的背景工作程序未取得認證。監督程序在指派[預先準備的背景工作程序](#the-supervisor-process)時提供新的認證快照，因此此錯誤表示監督程序本身沒有可用的已儲存認證。確認您已執行 `/login` 或設定 API 金鑰，然後停止監督程序：

```bash theme={null}
claude daemon stop --any --keep-workers
```

下一個 `claude agents` 或 `claude --bg` 啟動新的監督程序，該程序會讀取您的已儲存認證。如果您使用環境變數（例如 `ANTHROPIC_API_KEY`）而不是 `/login` 進行驗證，請從設定該變數的 shell 執行下一個命令。

請參閱[錯誤參考](/docs/zh-TW/errors#could-not-resolve-authentication-method)以取得完整的原因和修復清單。

<h3 id="background-sessions-can’t-read-desktop-documents-or-downloads-on-macos">
  背景工作階段無法在 macOS 上讀取 Desktop、Documents 或 Downloads
</h3>

在 macOS 上，背景工作階段主機作為其自己的程序執行，並與您的終端分開請求對受保護資料夾的存取。如果背景工作階段在讀取 `~/Desktop`、`~/Documents`、`~/Downloads` 或其他受保護位置時報告 `Operation not permitted`，請在系統設定中的隱私與安全性 > 檔案和資料夾下授予存取權限，或為該項目啟用完整磁碟存取。

使用原生安裝程式，該項目會顯示為 Claude Code，授予的權限在更新後會保留。使用其他安裝方法（例如 Homebrew 或 npm），該項目會顯示二進位檔路徑，更新後可能需要再次授予。

<h3 id="background-sessions-can’t-reach-local-network-hosts-on-macos">
  背景工作階段無法在 macOS 上連接到本機網路主機
</h3>

在 macOS 15 及更新版本上，系統會阻止程序連接到您本機網路上的裝置，直到您授予本機網路權限。在 v2.1.198 之前，背景工作階段主機從未請求該權限，因此針對 LAN 位址的命令失敗，出現 `connect: no route to host`，即使相同的命令在前景終端中有效。自 v2.1.198 起，背景工作階段中連接到本機網路位址的第一個命令會觸發 Claude Code 的 macOS 本機網路權限提示。授予一次，這些命令就能像在前景終端中一樣連接到 LAN 主機。

<h3 id="a-session-is-slow-to-respond-after-attaching">
  工作階段在附加後響應緩慢
</h3>

一旦工作階段完成並在未附加的情況下閒置約一小時，監督程序會停止其程序以釋放資源。附加會啟動從中斷的地方開始的新程序，並立即切換到工作階段，而程序重新啟動。正在工作、等待您或[釘選](#organize-the-list)的工作階段不會以這種方式停止，因此使用 `Ctrl+T` 釘選工作階段以保持其回應性。

當程序啟動時，工作階段文字記錄的最後一個螢幕會顯示，下方有 `Session is starting` 註記，當工作階段準備好時，即時工作階段會立即取代它。

<h3 id="claude/worktrees/-is-filling-up">
  `.claude/worktrees/` 正在填滿
</h3>

在 agent view 中刪除工作階段會移除 Claude 為其建立的 worktree，而無法安全移除的 worktree 會[保留其工作階段列](#organize-the-list)，以便不會被孤立。`claude rm` 會保留具有未提交變更的 worktree，並列印保留的路徑。在專案目錄中使用 `git worktree list` 列出剩餘條目，並使用 `git worktree remove <path>` 移除每個。請參閱[清理 worktrees](/docs/zh-TW/worktrees#clean-up-worktrees)。

<h2 id="limitations">
  限制
</h2>

Agent view 是研究預覽版本，具有以下限制：

* **速率限制適用**：背景工作階段與互動工作階段一樣消耗您的訂閱使用量，因此並行執行十個代理的使用配額速度快十倍。
* **工作階段是本地的**：背景工作階段在您的機器上執行。它們在睡眠時保留，但如果機器關閉則停止。
* **Claude 建立的 worktrees 在 agent view 中隨工作階段刪除**：在刪除在其自己的 worktree 中編輯檔案的工作階段之前，提交變更。具有未推送任何地方的提交的 worktree 會與工作階段一起保留。`claude rm` 也會保留具有未提交變更的 worktree 與其工作階段一起，而您自己建立的 worktree 會保留在原位。

<h2 id="related-resources">
  相關資源
</h2>

如需了解在平行中執行 Claude 的其他方式，請參閱：

* [在平行中執行代理](/docs/zh-TW/agents)：比較 agent view 與 subagents、agent teams 和 worktrees
* [Agent teams](/docs/zh-TW/agent-teams)：協調相互傳遞訊息的多個工作階段
* [Claude Code on the web](/docs/zh-TW/claude-code-on-the-web)：在受管雲環境中執行工作階段，而不是本地執行

<h2 id="version-history">
  版本歷史
</h2>

Agent view 在研究預覽期間發展迅速。如果您使用較舊的 Claude Code 版本，本頁上的某些行為可能會有所不同；特別是，`claude agents` 會以 `unknown option` 錯誤拒絕它尚不支援的旗標。下表列出了每個旗標和行為何時新增。

| 版本       | 變更                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| v2.1.208 | 附加到其程序已停止的工作階段會顯示其記錄的最後一屏，同時程序啟動，而不是只顯示 `Session is starting` 備註。無法傳遞的回覆（因為背景服務無法連線或傳送失敗）會被保存，並在其程序再次啟動時作為工作階段的下一個提示傳送；在此版本之前，背景服務無法連線時遺失的回覆會被丟棄。其自身二進位檔被更新取代的程序仍然可以啟動監督程序，從已安裝的 `claude` 啟動器或磁碟上的最新版本，而不是失敗直到 Claude Code 重新啟動。執行較舊版本的監督程序永遠不會將由較新版本啟動的閒置工作階段重新啟動到其自身較舊的二進位檔。刪除工作階段會移除其 worktree，即使工作階段將 worktree 移到不同的分支，並在 worktree 有未推送到任何地方的提交或另一個工作階段聲稱它時將 worktree 與工作階段列保持在一起，而不是銷毀提交或孤立 worktree。`/install-github-app` 和 `/mcp` 設定清單及其驗證動作在背景工作階段中被拒絕，並顯示命名替代方案的訊息；在 v2.1.208 中，`/model` 選擇器以相同方式被拒絕，輸入的 `/model <name>` 只切換該工作階段，而不是也保存您的預設模型。 |
| v2.1.207 | 查看面板以列截斷的句子開啟，例如等待您的工作階段的確切問題，並顯示被阻止的工作階段已等待多長時間，作為單一 `waiting 3m` 行，而不是將相同的時間戳記前綴到狀態句子和問題。在分派輸入中再次貼上相同的文字會展開摺疊的 `[Pasted text #N]` 預留位置，而不是新增第二個。按名稱接受計畫的背景工作階段會在其列上顯示該名稱。移入 worktree 的背景工作階段在其程序從 agent view 重新啟動時會保留其對話。                                                                                                                                                                                                                                                                                                                              |
| v2.1.206 | 列摘要填充列的剩餘寬度，並僅在終端的右邊緣截斷，而不是在 64 欄處。監督程序重新啟動到新的 Claude Code 版本後，它會在背景中將剩餘的閒置背景工作階段重新啟動到該版本，而不是每分鐘幾個。使用 `Ctrl+X` 或 `claude rm` 刪除工作階段也會從監督程序的工作階段清單中清除它，因此列在監督程序重新啟動後不再重新出現。                                                                                                                                                                                                                                                                                                                                                                               |
| v2.1.205 | 列摘要顯示工作階段自己的單行報告（在 64 欄處截斷），而不是原始工具叫用或 `done/total` 計數；目錄分組列以彩色狀態字開啟。查看面板以完整狀態句子開啟，對於等待您的工作階段，其確切問題顯示在回覆輸入上方。編輯、評論、關閉或使用 `gh` 標記拉取請求為就緒的工作階段會連結到它，不僅是建立或簽出拉取請求的工作階段，推送會連結拉取請求，即使本機分支名稱不符，建立命令的輸出超過內聯限制的拉取請求也會連結。沒有可讀文字的轉向會保留工作階段的先前狀態，而不是將其翻轉回 `Working`。`claude attach` 會等待最多約 60 秒以重新啟動的工作階段，並顯示狀態行說明原因，而不是失敗。                                                                                                                                                                                                                                         |
| v2.1.203 | 在分派 shell 中匯出的閘道 `ANTHROPIC_BASE_URL` 會到達從它分派的工作階段進入同一目錄，當監督程序共享該閘道環境時，而不是在保留隨之匯出的 API 金鑰時被丟棄。分派 shell 的 `PATH` 會套用到每個工作階段的工作程序。在子代理執行時按 `←` 會等待它們，而不是在十秒後重新啟動它們。空清單始終顯示區段標題，每個標題下方有描述。在分派輸入中輸入 `@` 也會列出啟動儲存庫的已註冊 git worktrees，這些 worktrees 位於其目錄樹內。從 `effortLevel` 設定繼承的努力會在稍後編輯該設定時跟隨，而不是在分派時固定。開啟其對話已在另一個執行中工作階段中開啟的已停止工作階段會被拒絕並顯示訊息，而不是使列失敗。在 agent view 中不可用的命令會在輸入中保留輸入的文字。在 git 儲存庫外失敗的 `WorktreeCreate` hook 不再阻止工作階段編輯檔案。                                                                                                                      |
| v2.1.202 | 使用 `/rename` 或 `Ctrl+R` 在背景工作階段上設定的名稱在監督程序停止並重新啟動其程序時會保留，而不是還原為工作階段分派時的名稱。                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| v2.1.200 | 較舊的 Claude Code 版本在 `roster.json` 中重寫工作階段清單時會保留較新版本寫入的欄位，符合現有的 `state.json` 保證，因此由較新版本啟動的工作階段在監督程序重新啟動後繼續接受輸入。當您開啟已停止回應的工作階段時，監督程序會重新啟動其程序，工作階段會從中斷的地方繼續中斷的回應。                                                                                                                                                                                                                                                                                                                                                                                           |
| v2.1.199 | 背景工作階段的程序在低記憶體主機上完成啟動前退出時，其列狀態會顯示 `possibly low memory — free some up and retry`，而不是只顯示裸露的退出原因。使用 `←` 或 `/background` 背景化工作階段會將其 `/color` 帶到新列。                                                                                                                                                                                                                                                                                                                                                                                                          |
| v2.1.198 | Agent view 在背景工作階段需要輸入、完成或失敗時透過 `preferredNotifChannel` 傳送通知，並使用 `agent_needs_input` 或 `agent_completed` 類型觸發 `Notification` hook。`←` 和 `/exit` 在 `claude attach <id>` 內返回 agent view 而不是退出到 shell；`Ctrl+Z` 返回到 shell。隔離其工作在 worktree 中的背景工作階段會提交、推送其自己的隔離分支（絕不是 `main` 或 `master`），並在完成時開啟草稿拉取請求，而不是先詢問。`/login` 在 agent view 中執行並開啟登入對話框。`Background work is running` 退出對話框提供 `Move to background and exit`。退出交付也涵蓋背景子代理，它們在下次喚醒時從其記錄恢復，而不是被報告為失敗。`claude --bg` 與 `-p` 或 `--print` 結合會被拒絕並出現錯誤。                                                        |
| v2.1.196 | 單一 `←` 按下會背景化前景工作階段；較早的版本需要兩次按下，帶有頁尾提示和確認。傳遞給 `claude agents` 的 `--dangerously-skip-permissions` 會顯示繞過免責聲明，而不是被無聲地丟棄。您從未命名的互動工作階段在工作階段清單和 `claude agents --json` 中帶有預設名稱，例如 `my-app-3f`。背景 shell 命令和動態工作流程在工作階段的程序被停止、重新啟動或更新時存活，包括在 Windows 上；設定 `CLAUDE_CODE_DISABLE_BG_EXIT_HANDOFF=1` 以關閉交付。在重新啟動時誤讀為空的記錄會被重命名為 `.orphaned-` 後綴，而不是被刪除。                                                                                                                                                                                                              |
| v2.1.195 | 進行中的工作在您背景化 Windows 上的工作階段時也會轉移；設定 `CLAUDE_DISABLE_ADOPT=1` 以改為停止它。`Completed` 組填充剩餘的垂直空間，標題在短終端上壓縮。較舊的 Claude Code 版本不再丟棄較新工作階段的 `state.json` 欄位或隱藏這些工作階段。附加到已停止的工作階段會立即切換，而不是顯示空白螢幕最多五秒。無法接受連接的監督程序會自行退出並釋放其鎖定。                                                                                                                                                                                                                                                                                                                                        |
| v2.1.174 | 背景工作階段不再繼承閘道端點變數，例如來自監督程序啟動 shell 的 `ANTHROPIC_BASE_URL`；監督程序向預先準備的工作程序提供新的認證快照，修復虛假的 `Could not resolve authentication method` 錯誤。                                                                                                                                                                                                                                                                                                                                                                                                                      |
| v2.1.172 | 分派輸入中的 `/model` 設定工作階段範圍的分派模型覆蓋。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| v2.1.161 | 列摘要顯示平行工作項目的 `done/total` 計數；查看面板命名最長執行的平行工作項目。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| v2.1.157 | `claude agents` 接受 `--agent`；分派的工作階段尊重 `agent` 設定。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| v2.1.145 | 查看面板回覆輸入和分派輸入中支援語音聽寫。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| v2.1.143 | `worktree.bgIsolation` 設定新增；`claude agents` 接受 `--allow-dangerously-skip-permissions`。                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| v2.1.142 | `claude agents` 接受 `--permission-mode`、`--model`、`--effort`、`--dangerously-skip-permissions`、`--settings`、`--add-dir`、`--plugin-dir`、`--mcp-config` 和 `--strict-mcp-config`。                                                                                                                                                                                                                                                                                                                                                                             |
| v2.1.141 | `claude agents` 接受 `--cwd` 以將清單範圍限定為一個專案。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| v2.1.139 | Agent view 作為研究預覽版本引入。                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
