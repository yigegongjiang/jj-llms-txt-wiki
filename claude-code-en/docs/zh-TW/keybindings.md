> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 自訂鍵盤快捷鍵

> 使用快捷鍵配置檔案在 Claude Code 中自訂鍵盤快捷鍵。

Claude Code 支援可自訂的鍵盤快捷鍵。執行 `/keybindings` 以在 `~/.claude/keybindings.json` 建立或開啟您的配置檔案。

<h2 id="configuration-file">
  配置檔案
</h2>

快捷鍵配置檔案是一個包含 `bindings` 陣列的物件。每個區塊指定一個上下文和一個按鍵組合到動作的對應。

<Note>快捷鍵檔案的變更會自動偵測並套用，無需重新啟動 Claude Code。</Note>

| 欄位         | 說明                            |
| :--------- | :---------------------------- |
| `$schema`  | 選用的 JSON Schema URL，用於編輯器自動完成 |
| `$docs`    | 選用的文件 URL                     |
| `bindings` | 按上下文分組的繫結區塊陣列                 |

此範例在聊天上下文中將 `Ctrl+E` 繫結到開啟外部編輯器，並取消繫結 `Ctrl+U`：

```json theme={null}
{
  "$schema": "https://www.schemastore.org/claude-code-keybindings.json",
  "$docs": "https://code.claude.com/docs/zh-TW/keybindings",
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "ctrl+e": "chat:externalEditor",
        "ctrl+u": null
      }
    }
  ]
}
```

<h2 id="contexts">
  上下文
</h2>

每個繫結區塊指定一個**上下文**，其中快捷鍵適用：

| 上下文               | 說明                                             |
| :---------------- | :--------------------------------------------- |
| `Global`          | 在應用程式的任何地方適用                                   |
| `Chat`            | 主聊天輸入區域                                        |
| `Autocomplete`    | 自動完成選單已開啟                                      |
| `Settings`        | 設定選單                                           |
| `Confirmation`    | 權限和確認對話框                                       |
| `Tabs`            | 標籤導覽元件                                         |
| `Help`            | 說明選單可見                                         |
| `Transcript`      | 文字記錄檢視器                                        |
| `HistorySearch`   | 歷史記錄搜尋模式 (Ctrl+R)                              |
| `Task`            | 背景工作正在執行                                       |
| `ThemePicker`     | 主題選擇器對話框                                       |
| `Attachments`     | 影像附件導覽在選擇對話框中                                  |
| `Footer`          | 頁尾指示器導覽（工作、團隊、差異、成品）                           |
| `MessageSelector` | 回溯和摘要對話框訊息選擇                                   |
| `DiffDialog`      | 差異檢視器導覽                                        |
| `DiffPanel`       | [差異面板](/docs/zh-TW/interactive-mode#diff-panel)已開啟  |
| `ModelPicker`     | 模型選擇器努力程度                                      |
| `EffortSlider`    | 由 `/effort` 開啟的努力程度滑桿                          |
| `Select`          | 通用選擇/清單元件                                      |
| `Plugin`          | Plugin 對話框（瀏覽、探索、管理）                           |
| `Agents`          | [Agent 檢視](/docs/zh-TW/agent-view)（`claude agents`） |
| `Scroll`          | 對話滾動和全螢幕模式中的文字選擇                               |

在 v2.1.205 之前，`/doctor` 診斷螢幕存在 `Doctor` 上下文和 `doctor:fix` 動作。

<h2 id="available-actions">
  可用的動作
</h2>

動作遵循 `namespace:action` 格式，例如 `chat:submit` 用於傳送訊息，或 `app:toggleTodos` 用於顯示工作清單。每個上下文都有特定的可用動作。

<h3 id="app-actions">
  App 動作
</h3>

在 `Global` 上下文中可用的動作：

| 動作                     | 預設     | 說明                                                          |
| :--------------------- | :----- | :---------------------------------------------------------- |
| `app:interrupt`        | Ctrl+C | 取消目前的操作                                                     |
| `app:exit`             | Ctrl+D | 結束 Claude Code。在 800ms 內按兩次以確認                              |
| `app:redraw`           | (未綁定)  | 強制終端機重繪                                                     |
| `app:toggleTodos`      | Ctrl+T | 切換 Claude 待辦事項清單的可見性。這不是 [`/tasks`](/docs/zh-TW/commands) 背景工作檢視 |
| `app:toggleTranscript` | Ctrl+O | 切換詳細文字記錄                                                    |

<h3 id="history-actions">
  History 動作
</h3>

用於導覽命令歷史記錄的動作：

| 動作                 | 預設     | 說明        |
| :----------------- | :----- | :-------- |
| `history:search`   | Ctrl+R | 開啟歷史記錄搜尋  |
| `history:previous` | Up     | 上一個歷史記錄項目 |
| `history:next`     | Down   | 下一個歷史記錄項目 |

<h3 id="chat-actions">
  Chat 動作
</h3>

在 `Chat` 上下文中可用的動作：

| 動作                    | 預設                              | 說明                                                                                                                                                                                                                                                                                                                                                                                  |
| :-------------------- | :------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `chat:cancel`         | Escape                          | 取消目前的輸入                                                                                                                                                                                                                                                                                                                                                                             |
| `chat:clearInput`     | Ctrl+L                          | 強制進行完整螢幕重繪，保留輸入和對話                                                                                                                                                                                                                                                                                                                                                                  |
| `chat:clearScreen`    | Cmd+K                           | 與 `chat:clearInput` 相同。請參閱 [清除對話](/docs/zh-TW/fullscreen#clear-the-conversation) 以了解 Cmd+K 在 iTerm2 和 Terminal.app 上的行為                                                                                                                                                                                                                                                                  |
| `chat:killAgents`     | Ctrl+X Ctrl+K                   | 停止此工作階段中所有執行中的 [背景子代理](/docs/zh-TW/sub-agents#run-subagents-in-foreground-or-background)，並關閉此工作階段其餘部分的 [成品自動回覆](/docs/zh-TW/artifacts#let-claude-reply-to-comments-on-its-own)                                                                                                                                                                                                                |
| `chat:cycleMode`      | Shift+Tab\*                     | 循環權限模式                                                                                                                                                                                                                                                                                                                                                                              |
| `chat:modelPicker`    | Meta+P                          | 開啟模型選擇器                                                                                                                                                                                                                                                                                                                                                                             |
| `chat:fastMode`       | Meta+O                          | 切換快速模式                                                                                                                                                                                                                                                                                                                                                                              |
| `chat:thinkingToggle` | Meta+T                          | 切換延伸思考                                                                                                                                                                                                                                                                                                                                                                              |
| `chat:submit`         | Enter                           | 提交訊息                                                                                                                                                                                                                                                                                                                                                                                |
| `chat:queueSubmit`    | Ctrl+X Enter                    | 提交訊息，標記為等待輪次：當 Claude 正在工作時，Claude Code [將其排隊](/docs/zh-TW/interactive-mode#queue-messages-while-claude-works)，永遠不會中斷輪次。與 `chat:submit` 不同，即使自動完成建議被突出顯示，它也會提交草稿。需要 v2.1.247 或更新版本                                                                                                                                                                                                       |
| `chat:sendNow`        | Ctrl+Enter, Ctrl+X Ctrl+S       | 立即傳送您的 [排隊訊息](/docs/zh-TW/interactive-mode#queue-messages-while-claude-works) 和您的草稿。[Claude Code 傳送您排隊的內容時](/docs/zh-TW/interactive-mode#when-claude-code-sends-what-you-queued) 涵蓋了 Claude 正在處理的輪次會發生什麼。當沒有任何操作執行時，該鍵會提交草稿，在 [shell 模式](/docs/zh-TW/interactive-mode#shell-mode-with-prefix) 中它只會排隊命令。不報告延伸鍵的終端機會將 `Ctrl+Enter` 傳遞為純 `Enter`，因此 `Ctrl+X Ctrl+S` 是在任何終端機中都能運作的綁定。需要 v2.1.275 或更新版本 |
| `chat:newline`        | Ctrl+J                          | 插入新行而不提交                                                                                                                                                                                                                                                                                                                                                                            |
| `chat:undo`           | Ctrl+\_, Ctrl+Shift+-           | 復原上一個動作                                                                                                                                                                                                                                                                                                                                                                             |
| `chat:externalEditor` | Ctrl+G, Ctrl+X Ctrl+E           | 在外部編輯器中開啟。[代理檢視分派輸入](/docs/zh-TW/agent-view#keyboard-shortcuts) 也遵循此動作的單鍵擊綁定                                                                                                                                                                                                                                                                                                             |
| `chat:stash`          | Ctrl+S                          | 隱藏目前的提示                                                                                                                                                                                                                                                                                                                                                                             |
| `chat:imagePaste`     | Ctrl+V (Windows 和 WSL 上為 Alt+V) | 從剪貼簿貼上影像。在 WSL 上，預設會綁定兩個快捷鍵                                                                                                                                                                                                                                                                                                                                                         |

\*在沒有 VT 模式的 Windows 上 (Node \<24.2.0/\<22.17.0, Bun \<1.2.23)，預設為 Meta+M。

<h3 id="autocomplete-actions">
  Autocomplete 動作
</h3>

在 `Autocomplete` 上下文中可用的動作：

| 動作                      | 預設     | 說明    |
| :---------------------- | :----- | :---- |
| `autocomplete:accept`   | Tab    | 接受建議  |
| `autocomplete:dismiss`  | Escape | 關閉選單  |
| `autocomplete:previous` | Up     | 上一個建議 |
| `autocomplete:next`     | Down   | 下一個建議 |

<h3 id="confirmation-actions">
  Confirmation 動作
</h3>

在 `Confirmation` 上下文中可用的動作：

| 動作                      | 預設          | 說明                                                                                                                                        |
| :---------------------- | :---------- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| `confirm:yes`           | Enter       | 確認動作                                                                                                                                      |
| `confirm:no`            | Escape      | 拒絕動作                                                                                                                                      |
| `confirm:previous`      | Up          | 上一個選項                                                                                                                                     |
| `confirm:next`          | Down        | 下一個選項                                                                                                                                     |
| `confirm:nextField`     | Tab         | 下一個欄位                                                                                                                                     |
| `confirm:previousField` | (未綁定)       | 上一個欄位                                                                                                                                     |
| `confirm:toggle`        | Space       | 切換選擇                                                                                                                                      |
| `confirm:cycleMode`     | Shift+Tab\* | 循環權限模式。在檔案權限提示上，關閉開啟的 [評論欄位](/docs/zh-TW/permissions#add-a-comment-when-you-answer-a-permission-prompt)；沒有開啟的欄位時，選擇允許此工作階段其餘部分的動作的選項，當提示提供該選項時 |

\*在沒有 VT 模式的 Windows 上 (Node \<24.2.0/\<22.17.0, Bun \<1.2.23)，預設為 Meta+M。

在 v2.1.257 之前，`confirm:toggleExplanation` 動作綁定到 `Ctrl+E`，預設情況下在 Bash 和 PowerShell 權限提示上顯示模型生成的命令說明。

對話框使用 `confirm:yes` 和 `confirm:no` 來接受和取消，即使它們不提出是或否的問題。如果您在此上下文中綁定裸字母（例如 `y` 或 `n`），該字母也會作用於從不將其顯示為鍵的對話框。顯示 `y` 和 `n` 作為其鍵的對話框會自行讀取這些字母，不需要綁定。

此範例將 `y` 綁定到 `confirm:yes`，將 `n` 綁定到 `confirm:no`：

```json theme={null}
{
  "bindings": [
    {
      "context": "Confirmation",
      "bindings": {
        "y": "confirm:yes",
        "n": "confirm:no"
      }
    }
  ]
}
```

使用這些綁定，當 [文字欄位](#text-fields) 有焦點時，`y` 和 `n` 仍然會輸入為字母。

在 v2.1.280 之前，`y` 也預設綁定到 `confirm:yes`，`n` 綁定到 `confirm:no`。如果您在 v2.1.280 之前使用 `/keybindings` 建立了 `keybindings.json`，該檔案會列出兩個綁定，它們會保持有效，直到您刪除這兩行。

<h3 id="permission-actions">
  Permission 動作
</h3>

在 `Confirmation` 上下文中可用於權限對話框的動作：

| 動作                       | 預設    | 說明                                                      |
| :----------------------- | :---- | :------------------------------------------------------ |
| `permission:toggleDebug` | (未綁定) | 切換權限偵錯資訊。Ctrl+D 的先前預設值在 v2.1.146 中被移除，因為它遮蔽了 `app:exit` |

<h3 id="transcript-actions">
  Transcript 動作
</h3>

在 `Transcript` 上下文中可用的動作：

| 動作                         | 預設                | 說明       |
| :------------------------- | :---------------- | :------- |
| `transcript:toggleShowAll` | Ctrl+E            | 切換顯示所有內容 |
| `transcript:exit`          | q, Ctrl+C, Escape | 結束文字記錄檢視 |

`transcript:toggleShowAll` 僅適用於經典轉譯器；在 [全螢幕轉譯](/docs/zh-TW/fullscreen) 中，文字記錄檢視器不提供顯示全部切換。

<h3 id="history-search-actions">
  History search 動作
</h3>

在 `HistorySearch` 上下文中可用的動作：

| 動作                         | 預設          | 說明                |
| :------------------------- | :---------- | :---------------- |
| `historySearch:next`       | Ctrl+R      | 下一個符合項            |
| `historySearch:accept`     | Escape, Tab | 接受選擇              |
| `historySearch:cancel`     | Ctrl+C      | 取消搜尋              |
| `historySearch:execute`    | Enter       | 執行選定的命令           |
| `historySearch:cycleScope` | Ctrl+S      | 循環範圍：工作階段、專案、任何地方 |

`historySearch:next`、`historySearch:accept`、`historySearch:cancel` 和 `historySearch:execute` 預設值適用於經典轉譯器中的內嵌歷史記錄搜尋，它始終搜尋來自所有專案的提示。`historySearch:cycleScope` 僅在 [全螢幕轉譯](/docs/zh-TW/fullscreen) 中生效，其中 `Ctrl+R` 開啟搜尋對話框，`Ctrl+S` 循環其範圍。對話框的其他鍵是固定的，無法重新綁定：`Enter` 或 `Tab` 將突出顯示的符合項放在提示輸入中，`Esc` 取消。

<h3 id="task-actions">
  Task 動作
</h3>

在 `Task` 上下文中可用的動作：

| 動作                | 預設                    | 說明                                     |
| :---------------- | :-------------------- | :------------------------------------- |
| `task:background` | Ctrl+B, Ctrl+X Ctrl+B | 背景化目前的工作。Ctrl+X Ctrl+B 和弦避免了 tmux 前綴衝突 |

<h3 id="theme-actions">
  Theme 動作
</h3>

在 `ThemePicker` 上下文中可用的動作：

| 動作                               | 預設     | 說明       |
| :------------------------------- | :----- | :------- |
| `theme:toggleSyntaxHighlighting` | Ctrl+T | 切換語法醒目提示 |

<h3 id="help-actions">
  Help 動作
</h3>

在 `Help` 上下文中可用的動作：

| 動作             | 預設     | 說明     |
| :------------- | :----- | :----- |
| `help:dismiss` | Escape | 關閉說明選單 |

<h3 id="tabs-actions">
  Tabs 動作
</h3>

在 `Tabs` 上下文中可用的動作：

| 動作              | 預設              | 說明    |
| :-------------- | :-------------- | :---- |
| `tabs:next`     | Tab, Right      | 下一個標籤 |
| `tabs:previous` | Shift+Tab, Left | 上一個標籤 |

<h3 id="attachments-actions">
  Attachments 動作
</h3>

在 `Attachments` 上下文中可用的動作：

| 動作                     | 預設                | 說明      |
| :--------------------- | :---------------- | :------ |
| `attachments:next`     | Right             | 下一個附件   |
| `attachments:previous` | Left              | 上一個附件   |
| `attachments:remove`   | Backspace, Delete | 移除選定的附件 |
| `attachments:exit`     | Down, Escape      | 結束附件導覽  |

<h3 id="footer-actions">
  Footer 動作
</h3>

在 `Footer` 上下文中可用的動作：

| 動作                      | 預設                | 說明                                                                               |
| :---------------------- | :---------------- | :------------------------------------------------------------------------------- |
| `footer:next`           | Right             | 下一個頁尾項目                                                                          |
| `footer:previous`       | Left              | 上一個頁尾項目                                                                          |
| `footer:up`             | Up                | 在頁尾中向上導覽（在頂部取消選擇）                                                                |
| `footer:down`           | Down              | 在頁尾中向下導覽                                                                         |
| `footer:openSelected`   | Enter             | 開啟選定的頁尾項目                                                                        |
| `footer:clearSelection` | Escape            | 清除頁尾選擇                                                                           |
| `footer:dismiss`        | Backspace, Delete | 從頁尾中關閉選定的 [成品](/docs/zh-TW/artifacts) 連結；已發佈的成品本身不受影響。在其他頁尾列上，這些鍵無效。需要 v2.1.217 或更新版本 |

當選定頁尾項目時（例如提示下方代理面板中的列），即使您在 `Chat` 上下文中將 `Enter` 重新綁定到 `chat:queueSubmit` 或 `chat:newline`，`Enter` 也會開啟它。

`Chat` 在 `Footer` 上下文未綁定的鍵上的綁定（例如 `Shift+Tab` 用於 `chat:cycleMode`）在選定項目時保持有效。

<h3 id="message-selector-actions">
  Message selector 動作
</h3>

在 `MessageSelector` 上下文中可用的動作：

| 動作                       | 預設                                        | 說明       |
| :----------------------- | :---------------------------------------- | :------- |
| `messageSelector:up`     | Up, K, Ctrl+P                             | 在清單中向上移動 |
| `messageSelector:down`   | Down, J, Ctrl+N                           | 在清單中向下移動 |
| `messageSelector:top`    | Ctrl+Up, Shift+Up, Meta+Up, Shift+K       | 跳到頂部     |
| `messageSelector:bottom` | Ctrl+Down, Shift+Down, Meta+Down, Shift+J | 跳到底部     |
| `messageSelector:select` | Enter                                     | 選擇訊息     |

<h3 id="diff-actions">
  Diff 動作
</h3>

在 `DiffDialog` 上下文中可用的動作：

| 動作                    | 預設      | 說明                                                                         |
| :-------------------- | :------ | :------------------------------------------------------------------------- |
| `diff:dismiss`        | Escape  | 關閉差異檢視器；從詳細檢視中，返回到檔案清單                                                     |
| `diff:previousSource` | Left    | 上一個差異來源                                                                    |
| `diff:nextSource`     | Right   | 下一個差異來源                                                                    |
| `diff:previousFile`   | Up, K   | 檔案清單中的上一個檔案；在詳細檢視中向上捲動一行                                                   |
| `diff:nextFile`       | Down, J | 檔案清單中的下一個檔案；在詳細檢視中向下捲動一行                                                   |
| `diff:viewDetails`    | Enter   | 檢視差異詳細資訊                                                                   |
| `diff:back`           | (未綁定)   | 在差異檢視器中返回。Escape 透過 `diff:dismiss` 執行返回動作。詳細檢視中 Left 的先前預設值在 v2.1.203 中被移除 |

差異詳細檢視也將尋呼機樣式鍵綁定到標準 [捲動動作](#scroll-actions)。這些綁定是 `DiffDialog` 上下文的一部分，僅適用於詳細檢視；[捲動動作](#scroll-actions) 下列出的 `Scroll` 上下文預設值保持不變。

| 動作                    | 預設             | 說明        |
| :-------------------- | :------------- | :-------- |
| `scroll:pageUp`       | PageUp         | 向上捲動半個檢視區 |
| `scroll:pageDown`     | PageDown       | 向下捲動半個檢視區 |
| `scroll:fullPageUp`   | Shift+Space, B | 向上捲動整個檢視區 |
| `scroll:fullPageDown` | Space          | 向下捲動整個檢視區 |
| `scroll:top`          | G, Home        | 跳到頂部      |
| `scroll:bottom`       | Shift+G, End   | 跳到底部      |

<h3 id="diff-panel-actions">
  Diff panel 動作
</h3>

用於 [差異面板](/docs/zh-TW/interactive-mode#diff-panel) 的動作，`/diff` 在全螢幕轉譯中開啟。`app:cycleDiffBase` 在 `DiffPanel` 上下文中，在面板開啟時有效；其他的在 `Global` 中。該面板需要 Claude Code v2.1.260 或更新版本。

| 動作                          | 預設                   | 說明                       |
| :-------------------------- | :------------------- | :----------------------- |
| `app:toggleReplTab`         | (未綁定)                | 開啟或關閉差異面板，與執行 `/diff` 相同 |
| `app:cycleDiffBase`         | Ctrl+X B             | 循環面板的比較基礎：此工作階段、未提交、然後分支 |
| `app:diffFileListUp`        | Ctrl+Up, Meta+Up     | 當面板的檔案清單溢出時向上捲動          |
| `app:diffFileListDown`      | Ctrl+Down, Meta+Down | 當面板的檔案清單溢出時向下捲動          |
| `app:toggleDiffNoiseFilter` | (未綁定)                | 在面板中顯示或隱藏測試和生成的檔案        |
| `app:toggleDiffPreSession`  | (未綁定)                | 展開或摺疊此工作階段之前的變更          |

<h3 id="model-picker-actions">
  Model picker 動作
</h3>

在 `ModelPicker` 上下文中可用的動作：

| 動作                            | 預設    | 說明               |
| :---------------------------- | :---- | :--------------- |
| `modelPicker:decreaseEffort`  | Left  | 降低努力等級           |
| `modelPicker:increaseEffort`  | Right | 提高努力等級           |
| `modelPicker:thisSessionOnly` | s     | 將突出顯示的模型套用到此工作階段 |

<h3 id="effort-slider-actions">
  Effort slider 動作
</h3>

在 `EffortSlider` 上下文中可用的動作，當您執行不帶引數的 `/effort` 時開啟的滑塊。滑塊的 Left、Right、Enter 和 Escape 鍵無法重新綁定。

| 動作                             | 預設 | 說明                                                                             |
| :----------------------------- | :- | :----------------------------------------------------------------------------- |
| `effortSlider:thisSessionOnly` | s  | 將焦點 [努力等級](/docs/zh-TW/model-config#adjust-effort-level) 套用到此工作階段。需要 v2.1.257 或更新版本 |

<h3 id="select-actions">
  Select 動作
</h3>

在 `Select` 上下文中可用的動作：

| 動作                | 預設              | 說明       |
| :---------------- | :-------------- | :------- |
| `select:next`     | Down, J, Ctrl+N | 下一個選項    |
| `select:previous` | Up, K, Ctrl+P   | 上一個選項    |
| `select:pageUp`   | PageUp          | 向上移動一頁選項 |
| `select:pageDown` | PageDown        | 向下移動一頁選項 |
| `select:first`    | Home            | 第一個選項    |
| `select:last`     | End             | 最後一個選項   |
| `select:accept`   | Enter           | 接受選擇     |
| `select:cancel`   | Escape          | 取消選擇     |

Claude Code 在 `/skills` 選單中套用您的 `select:pageUp`、`select:pageDown`、`select:first` 和 `select:last` 綁定。在大多數其他清單中，例如 `/model` 選擇器，您的 `select:first` 和 `select:last` 綁定適用。PageUp 和 PageDown 在這些清單中分頁選項，無論您的綁定如何。

在 v2.1.280 之前，這些其他清單忽略了 Home、End 和您的 `select:first` 和 `select:last` 綁定。

<h3 id="plugin-actions">
  Plugin 動作
</h3>

在 `Plugin` 上下文中可用的動作：

| 動作                | 預設    | 說明                          |
| :---------------- | :---- | :-------------------------- |
| `plugin:toggle`   | Space | 切換外掛程式選擇                    |
| `plugin:install`  | I     | 安裝選定的外掛程式                   |
| `plugin:favorite` | F     | 將選定的外掛程式設為最愛，使其在已安裝標籤頂部附近排序 |

<h3 id="settings-actions">
  Settings 動作
</h3>

在 `Settings` 上下文中可用的動作。`select:accept` 和 `confirm:no` 動作從 [Select](#select-actions) 和 [Confirmation](#confirmation-actions) 上下文重複使用，具有特定於設定的行為：變更會在您變更時立即套用到每個設定，因此 Escape 會關閉面板並保存您的變更，而不是拒絕。

| 動作                | 預設           | 說明             |
| :---------------- | :----------- | :------------- |
| `settings:search` | /            | 進入搜尋模式         |
| `settings:retry`  | R            | 在錯誤時重試載入使用量資料  |
| `select:accept`   | Enter, Space | 變更選定的設定或開啟其子選單 |
| `confirm:no`      | Escape       | 關閉面板。變更已保存     |

<h3 id="agents-actions">
  Agents 動作
</h3>

在 `Agents` 上下文中可用的動作，適用於 [代理檢視](/docs/zh-TW/agent-view)，使用 `claude agents` 開啟。需要 v2.1.257 或更新版本。

| 動作                  | 預設     | 說明                                                       |
| :------------------ | :----- | :------------------------------------------------------- |
| `agents:switchView` | Ctrl+S | 在狀態和目錄之間切換 [工作階段分組](/docs/zh-TW/agent-view#organize-the-list) |
| `agents:togglePin`  | Ctrl+T | [釘選或取消釘選](/docs/zh-TW/agent-view#organize-the-list) 選定的工作階段   |

當代理檢視開啟時，Claude Code 對 `Agents` 上下文綁定的任何鍵使用 `Agents` 綁定，並忽略同一鍵上的 `Chat` 或 `Global` 綁定。例如，在代理檢視中按 Ctrl+S 會切換工作階段分組，而不是觸發預設的 `chat:stash`。

分派輸入的外部編輯器快捷鍵不是 `Agents` 動作。代理檢視遵循 `Chat` 上下文的 `chat:externalEditor` 綁定，預設為 Ctrl+G。

在代理檢視中，綁定在單個按鍵上觸發，因此綁定到 `chat:externalEditor` 的 Ctrl+X Ctrl+E 和弦不會在那裡開啟編輯器。

<h3 id="voice-actions">
  Voice 動作
</h3>

當 [語音聽寫](/docs/zh-TW/voice-dictation) 啟用時，在 `Chat` 上下文中可用的動作：

| 動作                 | 預設    | 說明                       |
| :----------------- | :---- | :----------------------- |
| `voice:pushToTalk` | Space | 聽寫提示。根據 `/voice` 模式按住或點擊 |

<h3 id="scroll-actions">
  Scroll 動作
</h3>

當 [全螢幕轉譯](/docs/zh-TW/fullscreen) 啟用時，在 `Scroll` 上下文中可用的動作：

| 動作                          | 預設                   | 說明                                                   |
| :-------------------------- | :------------------- | :--------------------------------------------------- |
| `scroll:lineUp`             | `wheelup`            | 向上捲動一行。滑鼠滾輪捲動觸發此動作                                   |
| `scroll:lineDown`           | `wheeldown`          | 向下捲動一行。滑鼠滾輪捲動觸發此動作                                   |
| `scroll:pageUp`             | PageUp               | 向上捲動檢視區高度的一半                                         |
| `scroll:pageDown`           | PageDown             | 向下捲動檢視區高度的一半                                         |
| `scroll:top`                | Ctrl+Home            | 跳到對話的開始                                              |
| `scroll:bottom`             | Ctrl+End             | 跳到最新訊息並重新啟用自動跟隨                                      |
| `scroll:halfPageUp`         | (未綁定)                | 向上捲動檢視區高度的一半。與 `scroll:pageUp` 相同的行為，為 vi 樣式重新綁定提供   |
| `scroll:halfPageDown`       | (未綁定)                | 向下捲動檢視區高度的一半。與 `scroll:pageDown` 相同的行為，為 vi 樣式重新綁定提供 |
| `scroll:fullPageUp`         | (未綁定)                | 向上捲動整個檢視區高度                                          |
| `scroll:fullPageDown`       | (未綁定)                | 向下捲動整個檢視區高度                                          |
| `selection:copy`            | Ctrl+Shift+C / Cmd+C | 將選定的文字複製到剪貼簿                                         |
| `selection:clear`           | (未綁定)                | 清除有效的文字選擇。需要 v2.1.234 或更新版本                          |
| `selection:extendLeft`      | Shift+Left           | 將有效選擇向左延伸一欄                                          |
| `selection:extendRight`     | Shift+Right          | 將有效選擇向右延伸一欄                                          |
| `selection:extendUp`        | Shift+Up             | 將有效選擇向上延伸一列。當選擇到達頂部邊緣時捲動檢視區                          |
| `selection:extendDown`      | Shift+Down           | 將有效選擇向下延伸一列。當選擇到達底部邊緣時捲動檢視區                          |
| `selection:extendLineStart` | Shift+Home           | 將有效選擇延伸到行的開始                                         |
| `selection:extendLineEnd`   | Shift+End            | 將有效選擇延伸到行的結尾                                         |

<h2 id="keystroke-syntax">
  按鍵組合語法
</h2>

<h3 id="modifiers">
  修飾鍵
</h3>

使用 `+` 分隔符搭配修飾鍵：

* `ctrl` 或 `control` - Control 鍵
* `shift` - Shift 鍵
* `alt`、`opt`、`option` 或 `meta` - Windows 和 Linux 上的 Alt 鍵，macOS 上的 Option 鍵
* `cmd`、`command`、`super` 或 `win` - macOS 上的 Command 鍵，Windows 上的 Windows 鍵，Linux 上的 Super 鍵

`cmd` 群組只在報告 Super 修飾鍵的終端機中被偵測，例如支援 Kitty 鍵盤協議或 xterm 的 `modifyOtherKeys` 模式的終端機。大多數終端機不會發送它，因此對於您想在任何地方都能運作的繫結，請使用 `ctrl` 或 `meta`。

例如：

```text theme={null}
ctrl+k          Ctrl + K
shift+tab       Shift + Tab
meta+p          macOS 上的 Option + P，其他地方為 Alt + P
ctrl+shift+c    多個修飾鍵
```

<h3 id="uppercase-letters">
  大寫字母
</h3>

Claude Code 不區分大小寫地解析按鍵名稱，因此 `K` 與 `k` 的繫結相同，`ctrl+K` 與 `ctrl+k` 相同。若要繫結 Shift 和一個字母，請寫 `shift+k`。

<h3 id="non-us-keyboard-layouts">
  非美式鍵盤配置
</h3>

即使您的作用中鍵盤配置輸入其他字元，也請將 Ctrl 快捷鍵的按鍵名稱寫成拉丁字元。

Claude Code 如何將您按下的按鍵與繫結相符取決於配置的類型：

* 在西里爾字母等非拉丁配置下，當終端機使用 Kitty 鍵盤協議並報告該位置時，Claude Code 會根據按鍵的美式配置位置來符合 Ctrl 快捷鍵。在這樣的終端機中，使用俄文配置時，按下 Ctrl 和實體 W 鍵會觸發 `ctrl+w`。在不報告位置的終端機中，Claude Code 會符合終端機為按鍵發送的任何內容：ASCII 控制碼會觸發拉丁快捷鍵，而作為西里爾字元到達的按鍵不符合任何繫結
* 在重新排列拉丁字母的配置下，例如 AZERTY，Claude Code 會符合按鍵輸入的字母，因此按下 Ctrl 和標記為 A 的按鍵會觸發 `ctrl+a`

在 v2.1.247 之前，在使用 Kitty 鍵盤協議的終端機（例如 Ghostty、Kitty、WezTerm 和 iTerm2）中，在非拉丁配置下按下 Ctrl 快捷鍵不會觸發其繫結。

<h3 id="chords">
  和弦
</h3>

和弦是由空格分隔的按鍵組合序列：

```text theme={null}
ctrl+k ctrl+s   按 Ctrl+K，放開，然後按 Ctrl+S
```

在每個按鍵組合的 3 秒內按下下一個。如果您等待更長時間，Claude Code 會取消和弦並顯示簡短通知。

<h3 id="special-keys">
  特殊鍵
</h3>

* `escape` 或 `esc` - Escape 鍵
* `enter` 或 `return` - Enter 鍵
* `tab` - Tab 鍵
* `space` - 空格鍵
* `up`、`down`、`left`、`right` - 方向鍵
* `pageup`、`pagedown` - Page Up 和 Page Down 鍵
* `home`、`end` - Home 和 End 鍵
* `backspace`、`delete` - 刪除鍵
* `wheelup`、`wheeldown` - 滑鼠滾輪捲動事件

<h2 id="unbind-default-shortcuts">
  取消繫結預設快捷鍵
</h2>

將動作設定為 `null` 以取消繫結預設快捷鍵：

```json theme={null}
{
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "ctrl+s": null
      }
    }
  ]
}
```

這也適用於和弦繫結。取消繫結共享前綴的每個和弦會釋放該前綴以用作單一鍵繫結。任何作用中的內容中的和弦會保留其前綴的保留狀態，因此您必須在定義該和弦的內容中取消繫結每個和弦。

Claude Code 在 `ctrl+x` 前綴上繫結這些預設和弦：`Chat` 中的 `ctrl+x ctrl+k`、`ctrl+x ctrl+e`、`ctrl+x enter`、`ctrl+x ctrl+a`、`ctrl+x ctrl+s` 和 `ctrl+x tab`，`Task` 中的 `ctrl+x ctrl+b`，以及 `DiffPanel` 中的 `ctrl+x b`。`ctrl+x enter` 和弦需要 v2.1.247 或更新版本，`ctrl+x b`、`ctrl+x ctrl+a` 和 `ctrl+x tab` 需要 v2.1.260 或更新版本，而 `ctrl+x ctrl+s` 需要 v2.1.275 或更新版本。

若要將 `ctrl+x` 本身回收為單一鍵繫結，請取消繫結所有這些：

```json theme={null}
{
  "bindings": [
    {
      "context": "Task",
      "bindings": {
        "ctrl+x ctrl+b": null
      }
    },
    {
      "context": "DiffPanel",
      "bindings": {
        "ctrl+x b": null
      }
    },
    {
      "context": "Chat",
      "bindings": {
        "ctrl+x ctrl+k": null,
        "ctrl+x ctrl+e": null,
        "ctrl+x enter": null,
        "ctrl+x ctrl+a": null,
        "ctrl+x ctrl+s": null,
        "ctrl+x tab": null,
        "ctrl+x": "chat:newline"
      }
    }
  ]
}
```

如果您取消繫結前綴上的某些但不是全部和弦，按下前綴仍會進入和弦等待模式以進行剩餘的繫結。

<h2 id="reserved-shortcuts">
  保留的快捷鍵
</h2>

這些快捷鍵無法重新繫結：

| 快捷鍵       | 原因                                                                                                                                                                                       |
| :-------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Ctrl+C    | 硬編碼的中斷/取消                                                                                                                                                                                |
| Ctrl+D    | 硬編碼的結束                                                                                                                                                                                   |
| Ctrl+M    | Claude Code 始終將其接收為 Enter                                                                                                                                                                |
| Ctrl+\[   | Claude Code 始終將其接收為 Escape。在使用 Kitty 鍵盤協議的終端機中，這需要 v2.1.242 或更新版本                                                                                                                        |
| Ctrl+I    | Claude Code 始終將其接收為 Tab                                                                                                                                                                  |
| Ctrl+H    | 傳送 ASCII 退格位元組。[Claude Code 在 Windows 上如何讀取它](/docs/zh-TW/terminal-config#fix-backspace-deleting-a-whole-word-on-windows)取決於您的終端機和 [`CLAUDE_CODE_BS_AS_CTRL_BACKSPACE`](/docs/zh-TW/env-vars) 環境變數 |
| Caps Lock | 未傳遞至終端機應用程式                                                                                                                                                                              |

<h2 id="terminal-conflicts">
  終端機衝突
</h2>

某些快捷鍵可能與終端機多工器衝突：

| 快捷鍵    | 衝突                  |
| :----- | :------------------ |
| Ctrl+B | tmux 前綴（按兩次以傳送）     |
| Ctrl+A | GNU screen 前綴       |
| Ctrl+Z | Unix 程序暫停 (SIGTSTP) |

<h2 id="text-fields">
  文字欄位
</h2>

如果你綁定一個裸露的字母、數字或空格鍵，你仍然可以在對話框或面板內的文字欄位中輸入該字元。其中一個欄位是 Claude 提出問題時的 `Other` 答案。當欄位獲得焦點時，你按下的可列印鍵（不含 Ctrl、Alt 或 Cmd）會進入該欄位，Claude Code 不會根據你的綁定來匹配它。

這些鍵在欄位獲得焦點時仍會執行其綁定：

* 不輸入字元的鍵，例如 Enter、Escape、Tab 和方向鍵
* 任何使用 Ctrl、Alt 或 Cmd 按下的鍵
* 已在進行中的[和弦](#chords)的第二個按鍵

在主提示符處，Claude Code 根據作用中的內容（例如 `Chat`）來匹配每個鍵，只有在沒有綁定取用該鍵時才會輸入該鍵。

<h2 id="vim-mode-interaction">
  Vim 模式互動
</h2>

啟用 vim 模式時（透過 `/config` → 編輯器模式），快捷鍵和 vim 模式獨立運作：

* **Vim 模式**在文字輸入層級處理輸入（游標移動、模式、動作）
* **快捷鍵**在元件層級處理動作（切換待辦事項、提交等）
* vim 模式中的 Escape 鍵從 INSERT 切換到 NORMAL 模式；它不會觸發 `chat:cancel`
* 大多數 Ctrl+鍵快捷鍵通過 vim 模式傳遞到快捷鍵系統
* Vim 鍵無法透過快捷鍵檔案重新對應。若要對應兩鍵 INSERT 模式序列（例如 `jj`）至 Escape，請使用 [`vimInsertModeRemaps`](/docs/zh-TW/interactive-mode#remap-insert-mode-key-sequences) 設定
* 在 vim NORMAL 模式中，`?` 顯示說明選單（vim 行為）
* 在 vim NORMAL 模式中，`/` 開啟歷史搜尋，與標準模式中的 Ctrl+R 相同

<h2 id="validation">
  驗證
</h2>

Claude Code 驗證您的快捷鍵並顯示以下警告：

* 解析錯誤（無效的 JSON 或結構）
* 無效的上下文名稱
* 無效的動作值，例如不是字串或 `null` 的動作
* 未知的動作名稱，例如已註冊動作的拼寫錯誤。Claude Code 會跳過該繫結並保持該按鍵的任何預設繫結有效。在 v2.1.246 之前，具有未知動作名稱的繫結會無聲地停用該按鍵
* 保留快捷鍵衝突
* 同一上下文中的重複繫結

Claude Code 在檔案載入時報告警告，並將每個警告寫入偵錯日誌。使用 [`--debug`](/docs/zh-TW/cli-reference#cli-flags) 啟動 Claude Code 以查看詳細資訊。
