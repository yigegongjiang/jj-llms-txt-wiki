> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 互動模式

> Claude Code 會話中鍵盤快捷鍵、輸入模式和互動功能的完整參考。

<h2 id="keyboard-shortcuts">
  鍵盤快捷鍵
</h2>

<Note>
  鍵盤快捷鍵可能因平台和終端而異。在[全螢幕渲染](/docs/zh-TW/fullscreen)中，在文字記錄檢視器中按 `?` 以查看可用的快捷鍵。

  **macOS 使用者**：Option/Alt 鍵快捷鍵（`Alt+B`、`Alt+F`、`Alt+Y`、`Alt+M`、`Alt+P`）需要在終端中將 Option 配置為 Meta：

  * **iTerm2**：設定 → Profiles → Keys → General → 將 Left/Right Option 鍵設定為「Esc+」
  * **Apple Terminal**：設定 → Profiles → Keyboard → 勾選「Use Option as Meta Key」
  * **VS Code**：在 VS Code 設定中設定 `"terminal.integrated.macOptionIsMeta": true`

  詳見[終端配置](/docs/zh-TW/terminal-config)。
</Note>

<h3 id="general-controls">
  一般控制
</h3>

| 快捷鍵                                                | 說明                                                                                            | 上下文                                                                                                                                       |
| :------------------------------------------------- | :-------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| `Ctrl+C`                                           | 中斷，或清除輸入                                                                                      | 中斷執行中的操作。如果沒有任何操作執行中，第一次按下會清除提示輸入，第二次按下會退出 Claude Code                                                                                    |
| `Ctrl+X Ctrl+K`                                    | 終止此會話中所有執行中的[背景子代理](/docs/zh-TW/sub-agents#run-subagents-in-foreground-or-background)。在 3 秒內按兩次以確認 | 子代理控制                                                                                                                                     |
| `Ctrl+D`                                           | 退出 Claude Code 會話                                                                             | EOF 信號                                                                                                                                    |
| `Ctrl+G` 或 `Ctrl+X Ctrl+E`                         | 在預設文字編輯器中開啟                                                                                   | 在預設文字編輯器中編輯您的提示或自訂回應。`Ctrl+X Ctrl+E` 是 readline 原生繫結。在 `/config` 中開啟「在外部編輯器中顯示最後回應」，以在您的提示上方將 Claude 的先前回覆作為 `#` 註解上下文預先加入；當您儲存時，註解區塊會被移除 |
| `Ctrl+L`                                           | 重繪螢幕                                                                                          | 強制完整終端重繪。輸入和對話歷史會保留。如果顯示變得混亂或部分空白，請使用此選項恢復                                                                                                |
| `Ctrl+O`                                           | 切換文字記錄檢視器                                                                                     | 顯示詳細的工具使用和執行情況，每個助手訊息都有時間戳記和使用的模型。也會展開 MCP 呼叫，預設情況下會摺疊為單行，例如「Called slack 3 times」                                                        |
| `Ctrl+R`                                           | 反向搜尋命令歷史                                                                                      | 以互動方式搜尋先前的命令                                                                                                                              |
| `Ctrl+V` 或 `Cmd+V`（iTerm2）或 `Alt+V`（Windows 和 WSL） | 從剪貼簿貼上影像                                                                                      | 在游標處插入 `[Image #N]` 晶片，以便您可以在提示中按位置參考它。在 WSL 上，`Ctrl+V` 和 `Alt+V` 都已繫結；如果您的終端攔截 `Ctrl+V`，請使用 `Alt+V`                                      |
| `Ctrl+B`                                           | 背景執行工作                                                                                        | 將 bash 命令和代理放在背景執行。Tmux 使用者按兩次                                                                                                            |
| `Ctrl+T`                                           | 切換 Claude 的工作清單                                                                               | 在狀態區域中顯示或隱藏 [Claude 的待辦清單](#task-list)。這不是背景工作檢視；使用 [`/tasks`](/docs/zh-TW/commands) 以查看執行中的 shell 和子代理                                        |
| `Left/Right arrows`                                | 在對話框標籤之間循環                                                                                    | 在權限對話框和選單中的標籤之間導航                                                                                                                         |
| `Up/Down arrows` 或 `Ctrl+P`/`Ctrl+N`               | 移動游標或導航命令歷史                                                                                   | 當輸入跨越多個視覺行時，無論是換行還是多行，首先在提示內移動游標。一旦游標已在第一個或最後一個視覺行，再次按下會導航命令歷史。自 v2.1.169 起，換行的單行輸入行為與多行相同                                                |
| `Esc`                                              | 中斷 Claude，或關閉對話框                                                                              | 停止目前回應或工具呼叫中途，以便您可以重新導向。Claude 會保留迄今為止完成的工作。當對話框（例如權限提示）開啟時，`Esc` 會關閉對話框，而不是中斷 Claude。在 v2.1.202 之前，某些對話框上的 `Esc` 會中斷 Claude 並保持對話框開啟     |
| `Esc` + `Esc`                                      | 清除輸入草稿，或回溯                                                                                    | 當提示輸入包含文字時，雙 `Esc` 會清除它並將草稿儲存到歷史，以便 `Up` 可以回憶它。當輸入為空時，雙 `Esc` 會開啟[回溯選單](/docs/zh-TW/checkpointing)以從先前的點還原或摘要程式碼和對話                            |
| `Shift+Tab` 或 `Alt+M`（某些配置）                        | 循環權限模式                                                                                        | 在 `default`（在模式指示器中標記為 Manual）、`acceptEdits`、`plan` 和您啟用的任何模式（例如 `auto` 或 `bypassPermissions`）之間循環。詳見[權限模式](/docs/zh-TW/permission-modes)。     |
| `Option+P`（macOS）或 `Alt+P`（Windows/Linux）          | 切換模型                                                                                          | 在不清除提示的情況下切換模型                                                                                                                            |
| `Option+T`（macOS）或 `Alt+T`（Windows/Linux）          | 切換擴展思考                                                                                        | 啟用或停用擴展思考模式。在 Fable 5 上無效，其始終使用擴展思考。自 v2.1.132 起，此快捷鍵在 macOS 上無需配置 Option 為 Meta 即可運作                                                     |
| `Option+O`（macOS）或 `Alt+O`（Windows/Linux）          | 切換快速模式                                                                                        | 啟用或停用[快速模式](/docs/zh-TW/fast-mode)                                                                                                             |

<h3 id="text-editing">
  文字編輯
</h3>

| 快捷鍵                    | 說明          | 上下文                                                                                          |
| :--------------------- | :---------- | :------------------------------------------------------------------------------------------- |
| `Ctrl+A`               | 將游標移至目前行的開始 | 在多行輸入中，移至目前邏輯行的開始                                                                            |
| `Ctrl+E`               | 將游標移至目前行的結尾 | 在多行輸入中，移至目前邏輯行的結尾                                                                            |
| `Ctrl+K`               | 刪除到行尾       | 儲存已刪除的文字以供貼上                                                                                 |
| `Ctrl+U`               | 從游標刪除到行首    | 儲存已刪除的文字以供貼上。重複以清除多行輸入中的行。在 macOS 上，終端模擬器（包括 iTerm2 和 Terminal.app）將 `Cmd+Backspace` 對應到此快捷鍵 |
| `Ctrl+W`               | 刪除上一個單字     | 儲存已刪除的文字以供貼上。在 Windows 上，`Ctrl+Backspace` 也會刪除上一個單字                                          |
| `Ctrl+Y`               | 貼上已刪除的文字    | 貼上使用 `Ctrl+K`、`Ctrl+U` 或 `Ctrl+W` 刪除的文字                                                      |
| `Alt+Y`（在 `Ctrl+Y` 之後） | 循環貼上歷史      | 貼上後，循環瀏覽先前刪除的文字。在 macOS 上需要[將 Option 設定為 Meta](#keyboard-shortcuts)                          |
| `Alt+B`                | 將游標向後移動一個單字 | 單字導航。在 macOS 上需要[將 Option 設定為 Meta](#keyboard-shortcuts)                                     |
| `Alt+F`                | 將游標向前移動一個單字 | 單字導航。在 macOS 上需要[將 Option 設定為 Meta](#keyboard-shortcuts)                                     |

<h3 id="theme-and-display">
  主題和顯示
</h3>

| 快捷鍵      | 說明             | 上下文                                            |
| :------- | :------------- | :--------------------------------------------- |
| `Ctrl+T` | 切換程式碼區塊的語法醒目提示 | 僅在 `/theme` 選擇器選單內有效。控制 Claude 回應中的程式碼是否使用語法著色 |

<h3 id="multiline-input">
  多行輸入
</h3>

| 方法          | 快捷鍵            | 上下文                                                                                          |
| :---------- | :------------- | :------------------------------------------------------------------------------------------- |
| 快速逃脫        | `\` + `Enter`  | 適用於所有終端                                                                                      |
| Option 鍵    | `Option+Enter` | 在 macOS 上啟用[將 Option 設定為 Meta](/docs/zh-TW/terminal-config#enable-option-key-shortcuts-on-macos)後 |
| Shift+Enter | `Shift+Enter`  | 在 iTerm2、WezTerm、Ghostty、Kitty、Warp、Apple Terminal、Windows Terminal 中開箱即用                    |
| 控制序列        | `Ctrl+J`       | 在任何終端中無需配置即可使用                                                                               |
| 貼上模式        | 直接貼上           | 適用於程式碼區塊、日誌                                                                                  |

<Tip>
  Shift+Enter 在 iTerm2、WezTerm、Ghostty、Kitty、Warp、Apple Terminal 和 Windows Terminal 中無需配置即可使用。對於 VS Code、Cursor、Devin Desktop、Alacritty 和 Zed，執行 `/terminal-setup` 以安裝繫結。
</Tip>

<h3 id="quick-commands">
  快速命令
</h3>

| 快捷鍵     | 說明        | 備註                                         |
| :------ | :-------- | :----------------------------------------- |
| `/` 在開始 | 命令或 skill | 詳見[命令](#commands)和 [skills](/docs/zh-TW/skills) |
| `!` 在開始 | Bash 模式   | 直接執行命令並將執行輸出新增到會話                          |
| `@`     | 檔案路徑提及    | 觸發檔案路徑自動完成                                 |

<h3 id="transcript-viewer">
  文字記錄檢視器
</h3>

當文字記錄檢視器開啟時（使用 `Ctrl+O` 切換），這些快捷鍵可用。在[全螢幕渲染](/docs/zh-TW/fullscreen)中，按 `?` 以在檢視器內顯示完整的快捷鍵參考面板。`Ctrl+E` 可以透過 [`transcript:toggleShowAll`](/docs/zh-TW/keybindings) 重新繫結。

| 快捷鍵                | 說明                                                                                                                |
| :----------------- | :---------------------------------------------------------------------------------------------------------------- |
| `?`                | 切換鍵盤快捷鍵說明面板。需要[全螢幕渲染](/docs/zh-TW/fullscreen)                                                                          |
| `{` / `}`          | 跳至上一個或下一個使用者提示，類似 vim 段落動作。需要[全螢幕渲染](/docs/zh-TW/fullscreen)                                                           |
| `Ctrl+E`           | 切換顯示所有內容                                                                                                          |
| `[`                | 將完整對話寫入終端的原生滾動回溯，以便 `Cmd+F`、tmux 複製模式和其他原生工具可以搜尋它。需要[全螢幕渲染](/docs/zh-TW/fullscreen#search-and-review-the-conversation) |
| `v`                | 將對話寫入臨時檔案並在 `$VISUAL` 或 `$EDITOR` 中開啟它。需要[全螢幕渲染](/docs/zh-TW/fullscreen)                                               |
| `q`、`Ctrl+C`、`Esc` | 退出文字記錄檢視。所有三個都可以透過 [`transcript:exit`](/docs/zh-TW/keybindings) 重新繫結                                                   |

<h3 id="voice-input">
  語音輸入
</h3>

| 快捷鍵           | 說明   | 備註                                                                                                                         |
| :------------ | :--- | :------------------------------------------------------------------------------------------------------------------------- |
| 按住或點擊 `Space` | 語音聽寫 | 需要啟用[語音聽寫](/docs/zh-TW/voice-dictation)。按住以錄製，或執行 `/voice tap` 以進行點擊切換。[可重新繫結](/docs/zh-TW/voice-dictation#rebind-the-dictation-key) |

<h2 id="commands">
  命令
</h2>

在 Claude Code 中輸入 `/` 以查看所有可用命令，或輸入 `/` 後跟任何字母以篩選。`/` 選單顯示您可以呼叫的所有內容：內建命令、捆綁和使用者撰寫的 [skills](/docs/zh-TW/skills)，以及由 [plugins](/docs/zh-TW/plugins) 和 [MCP servers](/docs/zh-TW/mcp#use-mcp-prompts-as-commands) 貢獻的命令。並非所有內建命令對每個使用者都可見，因為某些命令取決於您的平台或計畫。

在[全螢幕呈現](/docs/zh-TW/fullscreen#use-the-mouse)中，`/` 命令和 `@` 檔案建議清單也會回應滑鼠：懸停會反白顯示一列，點擊會接受它。

詳見[命令參考](/docs/zh-TW/commands)以取得 Claude Code 中包含的命令的完整清單。

<h2 id="vim-editor-mode">
  Vim 編輯器模式
</h2>

透過 `/config` → Editor mode 啟用 vim 風格編輯。

<h3 id="mode-switching">
  模式切換
</h3>

| 命令    | 動作           | 來自模式          |
| :---- | :----------- | :------------ |
| `Esc` | 進入 NORMAL 模式 | INSERT、VISUAL |
| `i`   | 在游標前插入       | NORMAL        |
| `I`   | 在行首插入        | NORMAL        |
| `a`   | 在游標後插入       | NORMAL        |
| `A`   | 在行尾插入        | NORMAL        |
| `o`   | 在下方開啟行       | NORMAL        |
| `O`   | 在上方開啟行       | NORMAL        |
| `v`   | 開始字元式視覺選擇    | NORMAL        |
| `V`   | 開始行式視覺選擇     | NORMAL        |

<h3 id="remap-insert-mode-key-sequences">
  重新對應 INSERT 模式快捷鍵序列
</h3>

[`vimInsertModeRemaps`](/docs/zh-TW/settings#available-settings) 設定會將兩個按鍵的 INSERT 模式序列對應到 Escape，因此像 `jj` 這樣的對應會讓您回到 NORMAL 模式。需要 Claude Code v2.1.208 或更新版本。

以下 `~/.claude/settings.json` 範例會開啟 vim 模式並將 `jj` 對應到 Escape：

```json theme={null}
{
  "editorMode": "vim",
  "vimInsertModeRemaps": { "jj": "<Esc>" }
}
```

每個鍵恰好是按順序輸入的兩個可列印字元，而 `"<Esc>"` 是唯一支援的目標。具有不同長度或目標的項目會被忽略。

輸入序列的第一個字元會正常插入。在一秒內按下第二個字元會移除該待處理字元並切換到 NORMAL 模式，在您的輸入中不留下任何字元。在一秒視窗之後，或如果按下不同的鍵，兩個字元都會保留為字面文字，因此您仍然可以透過在兩個鍵之間暫停來輸入包含該序列的單字。

Claude Code 只會從您的使用者設定檔案、`--settings` 旗標和[受管設定](/docs/zh-TW/permissions#managed-settings)讀取此設定。專案的 `.claude/settings.json` 或 `.claude/settings.local.json` 中的項目會被忽略，因此簽出的儲存庫無法重新對應您的按鍵。

<h3 id="navigation-normal-mode">
  導航（NORMAL 模式）
</h3>

| 命令              | 動作                                                                           |
| :-------------- | :--------------------------------------------------------------------------- |
| `h`/`j`/`k`/`l` | 向左/向下/向上/向右移動                                                                |
| `Space`         | 向右移動                                                                         |
| `w`             | 下一個單字                                                                        |
| `e`             | 單字結尾                                                                         |
| `b`             | 上一個單字                                                                        |
| `0`             | 行首                                                                           |
| `$`             | 行尾                                                                           |
| `^`             | 第一個非空白字元                                                                     |
| `gg`            | 輸入開始                                                                         |
| `G`             | 輸入結尾                                                                         |
| `f{char}`       | 跳到下一個字元出現位置                                                                  |
| `F{char}`       | 跳到上一個字元出現位置                                                                  |
| `t{char}`       | 跳到下一個字元出現位置之前                                                                |
| `T{char}`       | 跳到上一個字元出現位置之後                                                                |
| `;`             | 重複上一個 f/F/t/T 動作                                                             |
| `,`             | 反向重複上一個 f/F/t/T 動作                                                           |
| `/`             | 開啟反向歷史搜尋，與 `Ctrl+R` 相同。自 v2.1.191 起，空搜尋提示會顯示提示：按 `Esc` 然後 `i` 然後 `/` 以開啟命令選單 |

<Note>
  在 vim 正常模式中，如果游標位於輸入的開始或結尾且無法進一步移動，`j`/`k` 和箭頭鍵將導航命令歷史。
</Note>

<h3 id="editing-normal-mode">
  編輯（NORMAL 模式）
</h3>

| 命令             | 動作          |
| :------------- | :---------- |
| `x`            | 刪除字元        |
| `dd`           | 刪除行         |
| `D`            | 刪除到行尾       |
| `dw`/`de`/`db` | 刪除單字/到結尾/向後 |
| `cc`           | 變更行         |
| `C`            | 變更到行尾       |
| `cw`/`ce`/`cb` | 變更單字/到結尾/向後 |
| `yy`/`Y`       | 複製行         |
| `yw`/`ye`/`yb` | 複製單字/到結尾/向後 |
| `p`            | 在游標後貼上      |
| `P`            | 在游標前貼上      |
| `>>`           | 縮排行         |
| `<<`           | 取消縮排行       |
| `J`            | 合併行         |
| `u`            | 復原          |
| `.`            | 重複上一個變更     |

<h3 id="text-objects-normal-mode">
  文字物件（NORMAL 模式）
</h3>

文字物件與運算子（如 `d`、`c` 和 `y`）搭配使用：

| 命令        | 動作                |
| :-------- | :---------------- |
| `iw`/`aw` | 內部/周圍單字           |
| `iW`/`aW` | 內部/周圍 WORD（以空白分隔） |
| `i"`/`a"` | 內部/周圍雙引號          |
| `i'`/`a'` | 內部/周圍單引號          |
| `i(`/`a(` | 內部/周圍括號           |
| `i[`/`a[` | 內部/周圍方括號          |
| `i{`/`a{` | 內部/周圍大括號          |

<h3 id="visual-mode">
  視覺模式
</h3>

按 `v` 進行字元式選擇或按 `V` 進行行式選擇。動作會擴展選擇，運算子直接作用於選擇。

| 命令               | 動作                   |
| :--------------- | :------------------- |
| `d`/`x`          | 刪除選擇                 |
| `y`              | 複製選擇                 |
| `c`/`s`          | 變更選擇                 |
| `p`              | 用暫存器內容取代選擇           |
| `r{char}`        | 將每個選定的字元取代為 `{char}` |
| `~`/`u`/`U`      | 切換、小寫或大寫選擇           |
| `>`/`<`          | 縮排或取消縮排選定的行          |
| `J`              | 合併選定的行               |
| `o`              | 交換游標和錨點              |
| `iw`/`aw`/`i"`/… | 選擇文字物件               |
| `v`/`V`          | 在字元式和行式之間切換，或退出      |

不支援使用 `Ctrl+V` 的區塊式視覺模式。

<h2 id="command-history">
  命令歷史
</h2>

Claude Code 維護目前會話的命令歷史：

* 輸入歷史按工作目錄儲存
* 當您執行 `/clear` 以開始新會話時，輸入歷史會重設。先前會話的對話會被保留，可以繼續進行。
* 連續提交相同的提示兩次會記錄一個歷史項目，因此按向上鍵會跳到先前的不同提示
* 使用向上/向下箭頭導航（請參閱上面的快捷鍵）
* 歷史擴展（`!`）預設停用

<h3 id="reverse-search-with-ctrl-r">
  使用 Ctrl+R 進行反向搜尋
</h3>

按 `Ctrl+R` 以互動方式搜尋您的命令歷史：

1. **開始搜尋**：按 `Ctrl+R` 啟動反向歷史搜尋
2. **輸入查詢**：輸入文字以在先前的命令中搜尋。搜尋詞在匹配結果中醒目提示
3. **導航匹配項**：再次按 `Ctrl+R` 以循環瀏覽較舊的匹配項
4. **變更範圍**：搜尋預設為所有專案的提示。按 `Ctrl+S` 以在此會話、此專案和所有專案之間循環範圍
5. **接受匹配項**：
   * 按 `Tab` 或 `Esc` 以接受目前匹配項並繼續編輯
   * 按 `Enter` 以接受並立即執行命令
6. **取消搜尋**：
   * 按 `Ctrl+C` 以取消並恢復您的原始輸入
   * 在空搜尋上按 `Backspace` 以取消

搜尋會載入所選範圍中最近的 100 個唯一提示，重複項目會摺疊至最新出現。匹配的提示會顯示醒目提示的搜尋詞，因此您可以找到並重複使用先前的輸入。

接受匹配項或取消搜尋會立即生效，即使 Claude Code 仍在載入歷史時也是如此。在 v2.1.202 之前，在該載入期間接受或取消可能會報告內部錯誤。

<h2 id="background-bash-commands">
  背景 Bash 命令
</h2>

Claude Code 支援在背景執行 Bash 命令，允許您在長時間執行的程序執行時繼續工作。

<h3 id="how-backgrounding-works">
  背景執行的工作原理
</h3>

當 Claude Code 在背景執行命令時，它會非同步執行命令並立即傳回背景工作 ID。Claude Code 可以在命令在背景繼續執行時回應新提示。

若要在背景執行命令，您可以：

* 提示 Claude Code 在背景執行命令
* 按 `Ctrl+B` 將常規 Bash 工具呼叫移到背景。Tmux 使用者必須按 `Ctrl+B` 兩次，因為 tmux 的前綴鍵。

**主要功能：**

* 輸出被寫入檔案，Claude 可以使用 Read 工具檢索它
* 背景工作有唯一的 ID 用於追蹤和輸出檢索
* 背景工作在 Claude Code 退出時會自動清理。將工作階段背景執行而不是退出會將它們交給背景工作階段，它們會繼續執行。請參閱[背景執行執行中的工作階段](/docs/zh-TW/agent-view#from-inside-a-session)
* 如果輸出超過 5GB，背景工作會自動終止，stderr 中會有說明原因的備註
* 自 v2.1.193 起，在 macOS 和 Linux 上，當作業系統發出記憶體壓力信號時，執行中的背景工作會被終止，前提是工作階段已閒置至少 30 分鐘，沒有任何轉換或子代理執行。將 [`CLAUDE_CODE_DISABLE_BG_SHELL_PRESSURE_REAP`](/docs/zh-TW/env-vars) 設定為 `1` 以關閉此功能

若要停用所有背景工作功能，請將 `CLAUDE_CODE_DISABLE_BACKGROUND_TASKS` 環境變數設定為 `1`。詳見[環境變數](/docs/zh-TW/env-vars)。

**常見的背景執行命令：**

* 建置工具（webpack、vite、make）
* 套件管理器（npm、yarn、pnpm）
* 測試執行器（jest、pytest）
* 開發伺服器
* 長時間執行的程序（docker、terraform）

<h3 id="shell-mode-with-prefix">
  使用 `!` 前綴的 Shell 模式
</h3>

透過在輸入前加上 `!` 直接執行 shell 命令，無需透過 Claude：

```bash theme={null}
! npm test
! git status
! ls -la
```

Shell 模式：

* 將命令及其輸出新增到對話上下文
* 顯示即時進度和輸出
* 支援相同的 `Ctrl+B` 背景執行，用於長時間執行的命令
* 不需要 Claude 解釋或批准命令
* 支援基於歷史的自動完成：輸入部分命令並按 `Tab` 以從目前專案中的先前 `!` 命令完成
* 自 v2.1.193 起在所有平台上支援即時檔案路徑自動完成：輸入包含正斜線的權杖，例如 `./src/` 或 `~/`，以查看符合的檔案和目錄的下拉清單，然後按 `Tab` 以接受。在 Windows 上也使用正斜線；下拉清單由 `/` 觸發，而不是 `\`
* 在空提示上使用 `Escape`、`Backspace` 或 `Ctrl+U` 退出
* 將以 `!` 開頭的貼上文字貼到空提示中會自動進入 shell 模式，符合輸入的 `!` 行為

自 v2.1.186 起，Claude 會在命令輸出進入文字記錄後自動回應，因此您可以執行 `! npm test` 並獲得失敗的說明，無需第二個提示。回應成本與傳送一般提示相同。若要恢復先前的行為，其中輸出被新增到上下文而不進行回應，請在 `settings.json` 中將 [`respondToBashCommands`](/docs/zh-TW/settings#available-settings) 設定為 `false`。在 v2.1.186 之前，shell 模式始終將輸出新增到上下文而不進行回應。

這對於快速 shell 操作同時維護對話上下文很有用。

<h2 id="prompt-suggestions">
  提示建議
</h2>

當您首次開啟會話時，提示輸入中會出現灰色的範例命令以幫助您開始。Claude Code 從您的專案的 git 歷史中選擇此項，因此它反映您最近一直在處理的檔案。

Claude 回應後，建議會根據您的對話歷史繼續出現，例如多部分請求的後續步驟或工作流程的自然延續。

* 按 `Tab` 或 `Right arrow` 以將建議放入提示輸入中，然後按 `Enter` 以提交
* 開始輸入以關閉它

建議作為背景請求執行，該請求重複使用父對話的提示快取，因此額外成本最少。當快取冷時，Claude Code 會跳過建議生成以避免不必要的成本。

在對話的第一輪之後以及在 Plan Mode 中，建議會自動跳過。在列印模式中，預設情況下它們是關閉的。傳遞 [`--prompt-suggestions`](/docs/zh-TW/cli-reference#cli-flags) 搭配 `--output-format stream-json --verbose` 以在每一輪之後改為發出 `prompt_suggestion` 訊息。

若要完全停用提示建議，請設定環境變數或在 `/config` 中切換設定：

```bash theme={null}
export CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION=false
```

<h2 id="side-questions-with-/btw">
  使用 /btw 的側面問題
</h2>

使用 `/btw` 詢問有關您目前工作的快速問題，而不將其新增到對話歷史。當您想要快速答案但不想雜亂主要上下文或使 Claude 偏離長時間執行的工作時，這很有用。

```
/btw what was the name of that config file again?
```

側面問題可以完全看到目前對話，因此您可以詢問 Claude 已經讀過的程式碼、它之前做出的決定或會話中的任何其他內容。問題和答案是短暫的：它們出現在可關閉的覆蓋層中，永遠不會進入對話歷史。

* **Claude 工作時可用**：即使 Claude 正在處理回應時，您也可以執行 `/btw`。側面問題獨立執行，不會中斷主要輪次。
* **無工具存取**：側面問題僅從已在上下文中的內容回答。Claude 在回答側面問題時無法讀取檔案、執行命令或搜尋。
* **單一回應**：覆蓋層中沒有後續輪次。若要繼續執行緒，請使用 `f` 將其分支到自己的會話中。
* **低成本**：側面問題重複使用父對話的提示快取，因此額外成本最少。

答案出現後，覆蓋層接受這些按鍵。來自同一會話的較早側面問題會顯示為目前答案上方的淡色列表；它們保持在對話歷史之外，但在覆蓋層中保持可見，直到您清除它們。

| 按鍵                       | 動作                                                                                                |
| :----------------------- | :------------------------------------------------------------------------------------------------ |
| `Space`、`Enter`、`Escape` | 關閉答案並返回提示                                                                                         |
| `Up` / `Down`            | 捲動答案                                                                                              |
| `Left` / `Right`         | 在此答案和您來自會話的較早 `/btw` 答案之間切換。`Left` 移至較舊的答案，`Right` 返回目前的答案。需要 Claude Code v2.1.187 或更新版本          |
| `c`                      | 將答案複製到您的剪貼簿作為原始 Markdown。使用此方式而不是滑鼠選取，後者會擷取硬換行的終端機呈現而非原始文字                                        |
| `f`                      | 分支到新會話。分支繼承父對話加上此問題和答案作為真實文字記錄輪次，因此您可以繼續進行完整工具存取。原始會話保留在 [`/resume`](/docs/zh-TW/commands) 下。僅在本機會話中可用 |
| `x`                      | 清除目前答案上方顯示的較早 `/btw` 交換列表                                                                         |

`/btw` 是 [subagent](/docs/zh-TW/sub-agents) 的反面：它看到您的完整對話但沒有工具，而 subagent 有完整工具但以空上下文開始。使用 `/btw` 詢問 Claude 從此會話已知的內容；使用 subagent 去發現新的東西。

<h2 id="task-list">
  工作清單
</h2>

工作清單是 Claude 的待辦事項檢查清單：Claude 建立的項目用於規劃多步驟工作，並有指示器顯示待處理、進行中或完成的內容。它與背景工作檢視分開。若要查看執行中的 shell 和子代理，請改用 [`/tasks`](/docs/zh-TW/commands)。

* 按 `Ctrl+T` 以切換工作清單檢視。顯示一次最多五個工作。當 Claude 尚未建立任何檢查清單項目時，切換沒有可見效果，因為沒有任何內容可顯示
* 若要查看所有工作或清除它們，直接詢問 Claude：「show me all tasks」或「clear all tasks」
* 工作在上下文壓縮中持續存在，幫助 Claude 在較大的專案上保持組織
* 若要在會話之間共享工作清單，請設定 `CLAUDE_CODE_TASK_LIST_ID` 以使用 `~/.claude/tasks/` 中的命名目錄：`CLAUDE_CODE_TASK_LIST_ID=my-project claude`

<h2 id="session-recap">
  會話摘要
</h2>

當您在離開終端後返回時，Claude Code 會顯示到目前為止會話中發生的情況的單行摘要。摘要在背景中生成，一旦自上次完成的輪次以來至少已過三分鐘且終端未聚焦，就會準備好。摘要僅在會話至少有三個輪次後出現，且永遠不會連續出現兩次。

執行 `/recap` 以按需生成摘要。若要關閉自動摘要，請開啟 `/config` 並停用**會話摘要**。

會話摘要在每個計畫和提供者上預設開啟。摘要在非互動模式中始終被跳過。

<h2 id="pr-review-status">
  PR 審查狀態
</h2>

在處理具有開啟拉取請求的分支時，Claude Code 在頁尾顯示可點擊的 PR 連結（例如「PR #446」）。連結有一個彩色底線，指示審查狀態：

* 綠色：已批准
* 黃色：待審查
* 紅色：要求變更
* 灰色：草稿

拉取請求合併或關閉後，徽章會消失。`Cmd+click`（macOS）或 `Ctrl+click`（Windows/Linux）連結以在瀏覽器中開啟拉取請求。狀態每 60 秒自動更新，並在工作階段中執行 `gh pr` 或 `git push` 命令後立即更新。

<Note>
  PR 狀態需要安裝並驗證 `gh` CLI（`gh auth login`）。
</Note>

<h2 id="see-also">
  另請參閱
</h2>

* [Skills](/docs/zh-TW/skills) - 自訂提示和工作流程
* [Checkpointing](/docs/zh-TW/checkpointing) - 回溯 Claude 的編輯並恢復先前的狀態
* [CLI 參考](/docs/zh-TW/cli-reference) - 命令列旗標和選項
* [設定](/docs/zh-TW/settings) - 配置選項
* [記憶體管理](/docs/zh-TW/memory) - 管理 CLAUDE.md 檔案
