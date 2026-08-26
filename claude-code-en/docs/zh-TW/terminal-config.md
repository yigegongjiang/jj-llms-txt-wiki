> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 為 Claude Code 配置您的終端機

> 修復 Shift+Enter 以插入換行符、在 Claude 完成時獲得終端機鈴聲、配置 tmux、匹配色彩主題，以及在 Claude Code CLI 中啟用 Vim 模式。

Claude Code 在任何終端機中都可以無需配置而運作。此頁面適用於當某些特定功能的行為不符合您的預期時。在下方找到您的症狀。如果一切已經感覺正確，您不需要此頁面。

* [Shift+Enter 提交而不是插入換行符](#enter-multiline-prompts)
* [Option 鍵快捷鍵在 macOS 上無效](#enable-option-key-shortcuts-on-macos)
* [Claude 完成時沒有聲音或警報](#get-a-terminal-bell-or-notification)
* [您在 tmux 內執行 Claude Code](#configure-tmux)
* [顯示閃爍或捲動位置跳躍](#switch-to-fullscreen-rendering)
* [您想在提示中使用 Vim 快捷鍵](#edit-prompts-with-vim-keybindings)

此頁面是關於讓您的終端機向 Claude Code 發送正確的信號。若要更改 Claude Code 本身回應的快捷鍵，請改為參閱[快捷鍵](/docs/zh-TW/keybindings)。

<h2 id="enter-multiline-prompts">
  輸入多行提示
</h2>

按 Enter 提交您的訊息。若要在不提交的情況下新增換行符，請按 Ctrl+J，或輸入 `\` 然後按 Enter。兩者都在每個終端機中無需設置即可運作。

在大多數終端機中，您也可以按 Shift+Enter，但支援因終端機模擬器而異：

| 終端機                                                               | Shift+Enter 用於換行符            |
| :---------------------------------------------------------------- | :--------------------------- |
| Ghostty、Kitty、iTerm2、WezTerm、Warp、Apple Terminal、Windows Terminal | 無需設置即可運作                     |
| VS Code、Cursor、Devin Desktop、Alacritty、Zed                        | 執行一次 `/terminal-setup`       |
| gnome-terminal、JetBrains IDE（例如 PyCharm 和 Android Studio）         | 不可用；使用 Ctrl+J 或 `\` 然後 Enter |

對於 VS Code、Cursor、Devin Desktop、Alacritty 和 Zed，`/terminal-setup` 將 Shift+Enter 和其他快捷鍵寫入終端機的配置檔案。現有的綁定會保留在原位；如果您看到類似 `VSCode terminal Shift+Enter key binding already configured` 的訊息，則未進行任何變更。直接在主機終端機中執行 `/terminal-setup` 而不是在 tmux 或 screen 內，因為它需要寫入主機終端機的配置。

在 VS Code、Cursor 和 Devin Desktop 中，`/terminal-setup` 也會更新兩個編輯器設定：它將 `terminal.integrated.gpuAcceleration` 設定為 `"off"` 以防止整合終端機中的文字亂碼，並設定 `terminal.integrated.mouseWheelScrollSensitivity` 以在[全螢幕模式](/docs/zh-TW/fullscreen)中實現更平順的滾動。若要復原 GPU 加速變更，請將其設回 `"auto"` 並重新載入編輯器視窗。

如果您在 tmux 內執行，即使外部終端機支援，Shift+Enter 也需要下面的 [tmux 配置](#configure-tmux)。

若要將換行符綁定到不同的快捷鍵，或交換行為使 Enter 插入換行符而 Shift+Enter 提交，請在您的[快捷鍵檔案](/docs/zh-TW/keybindings)中對應 `chat:newline` 和 `chat:submit` 動作。

<h2 id="enable-option-key-shortcuts-on-macos">
  在 macOS 上啟用 Option 快捷鍵
</h2>

某些 Claude Code 快捷鍵使用 Option 快捷鍵，例如 Option+Enter 用於換行符或 Option+P 用於切換模型。在 macOS 上，大多數終端機預設不會將 Option 作為修飾符發送，因此這些快捷鍵在您啟用它之前無法運作。終端機設定通常標記為「使用 Option 作為 Meta 快捷鍵」；Meta 是現在標記為 Option 或 Alt 的快捷鍵的歷史 Unix 名稱。

<Tabs>
  <Tab title="Apple Terminal">
    開啟設定 → 設定檔 → 鍵盤並勾選'使用 Option 作為 Meta 快捷鍵'。

    如果您接受了 Claude Code 的首次執行提示，該提示提供'Option+Enter 用於換行符和視覺鈴聲'，這已經完成。該提示為您執行 `/terminal-setup`，它在您的 Apple Terminal 設定檔中啟用 Option 作為 Meta 並將音訊鈴聲切換為視覺螢幕閃爍。
  </Tab>

  <Tab title="iTerm2">
    開啟設定 → 設定檔 → 快捷鍵 → 一般並將左 Option 快捷鍵和右 Option 快捷鍵設置為「Esc+」。

    在 iTerm2 中執行 `/terminal-setup` 會在設定 → 一般 → 選取範圍下啟用「終端機中的應用程式可以存取剪貼簿」，以便 `/copy` 命令可以寫入您的系統剪貼簿。該命令即使在 tmux 內執行時也能偵測 iTerm2。重新啟動 iTerm2 以使變更生效。
  </Tab>

  <Tab title="VS Code">
    將 `"terminal.integrated.macOptionIsMeta": true` 新增至您的 VS Code 設定。
  </Tab>
</Tabs>

對於 Ghostty、Kitty 和其他終端機，請在終端機的配置檔案中尋找 Option-as-Alt 或 Option-as-Meta 設定。

<h2 id="get-a-terminal-bell-or-notification">
  獲得終端機鈴聲或通知
</h2>

當 Claude 完成工作或暫停以進行權限提示時，它會觸發通知事件。將其顯示為終端機鈴聲或桌面通知可讓您在長工作執行時切換到其他工作。

Claude Code 預設僅在 Ghostty、Kitty 和 iTerm2 中發送桌面通知。在其他終端機中，將 [`preferredNotifChannel`](/docs/zh-TW/settings#available-settings) 設定為 `"terminal_bell"` 以改為響起終端機鈴聲，或配置[通知 hook](#play-a-sound-with-a-notification-hook) 以獲得自訂聲音或命令。

桌面通知透過 SSH 到達您的本機，因此遠端工作階段仍然可以提醒您。Ghostty 和 Kitty 無需進一步設置即可將其轉發到您的 OS 通知中心。iTerm2 要求您啟用轉發：

<Steps>
  <Step title="開啟 iTerm2 通知設定">
    前往設定 → 設定檔 → 終端機。
  </Step>

  <Step title="啟用警報">
    勾選「通知中心警報」，然後點擊「篩選警報」並啟用「傳送逃脫序列產生的警報」。
  </Step>
</Steps>

如果通知仍未出現，請確認您的終端機應用程式在您的 OS 設定中具有通知權限，如果您在 tmux 內執行，請[啟用通過](#configure-tmux)。

<h3 id="play-a-sound-with-a-notification-hook">
  使用通知 hook 播放聲音
</h3>

在任何終端機中，您可以配置[通知 hook](/docs/zh-TW/hooks-guide#get-notified-when-claude-needs-input) 以在 Claude 需要您的注意時播放聲音或執行自訂命令。Hooks 與內建通知一起執行，而不是替代它，因此不會收到桌面通知的終端機（例如 Warp 或 VS Code 整合終端機）可以使用 hook 或將 `preferredNotifChannel` 設定為 `"terminal_bell"` 代替。

下面的範例在 macOS 上播放系統聲音。連結的指南包含 macOS、Linux 和 Windows 的桌面通知命令。

```json ~/.claude/settings.json theme={null}
{
  "hooks": {
    "Notification": [
      {
        "hooks": [{ "type": "command", "command": "afplay /System/Library/Sounds/Glass.aiff" }]
      }
    ]
  }
}
```

<h2 id="configure-tmux">
  配置 tmux
</h2>

當 Claude Code 在 tmux 內執行時，預設情況下會發生兩件事：Shift+Enter 提交而不是插入換行符，桌面通知和[進度列](/docs/zh-TW/settings#available-settings)永遠無法到達外部終端機。將這些行新增至 `~/.tmux.conf`，然後執行 `tmux source-file ~/.tmux.conf` 以將它們應用到執行中的伺服器：

```bash ~/.tmux.conf theme={null}
set -g allow-passthrough on
set -s extended-keys on
set -as terminal-features 'xterm*:extkeys'
```

`allow-passthrough` 行讓通知和進度更新到達外部終端機，而不是被 tmux 吞沒。`extended-keys` 行讓 tmux 區分 Shift+Enter 和純 Enter，以便換行符快捷鍵運作。

<h2 id="match-the-color-theme">
  匹配色彩主題
</h2>

使用 `/theme` 命令或 `/config` 中的主題選擇器來選擇與您的終端機相匹配的 Claude Code 主題。選擇自動選項會偵測您的終端機的淺色或深色背景，因此主題會在您的終端機執行時跟隨 OS 外觀變更。Claude Code 不控制終端機自己的色彩配置，該配置由終端機應用程式設定。

若要自訂介面底部出現的內容，請配置[自訂狀態列](/docs/zh-TW/statusline)，顯示目前的模型、工作目錄、git 分支或其他上下文。

<h3 id="create-a-custom-theme">
  建立自訂主題
</h3>

<Note>
  自訂主題需要 Claude Code v2.1.118 或更新版本。
</Note>

除了內建預設值外，`/theme` 還會列出您已定義的任何自訂主題以及已安裝 [plugins](/docs/zh-TW/plugins-reference#themes) 貢獻的任何主題。選擇清單末尾的 **New custom theme…** 以互動方式建立一個：您命名主題，然後選擇要覆蓋的個別色彩令牌。當自訂主題被突出顯示時，按 `Ctrl+E` 以編輯它。

每個自訂主題都是 `~/.claude/themes/` 中的 JSON 檔案。不含 `.json` 副檔名的檔案名稱是主題的 slug，選擇主題會將 `custom:<slug>` 儲存為您的主題偏好設定。該檔案有三個選用欄位：

| 欄位          | 類型     | 描述                                                                                                   |
| :---------- | :----- | :--------------------------------------------------------------------------------------------------- |
| `name`      | string | 在 `/theme` 中顯示的標籤。預設為檔案名稱 slug                                                                       |
| `base`      | string | 主題開始的內建預設值：`dark`、`light`、`dark-daltonized`、`light-daltonized`、`dark-ansi` 或 `light-ansi`。預設為 `dark` |
| `overrides` | object | 色彩令牌名稱到色彩值的對應。此處未列出的令牌會落回到基礎預設值                                                                      |

色彩值接受 `#rrggbb`、`#rgb`、`rgb(r,g,b)`、`ansi256(n)` 或 `ansi:<name>`，其中 `<name>` 是 16 個標準 ANSI 色彩名稱之一，例如 `red` 或 `cyanBright`。未知的令牌和無效的色彩值會被忽略，因此打字錯誤無法破壞呈現。

以下範例定義了一個保留深色預設值但重新著色提示符號重點、錯誤文字和成功文字的主題：

```json ~/.claude/themes/dracula.json theme={null}
{
  "name": "Dracula",
  "base": "dark",
  "overrides": {
    "claude": "#bd93f9",
    "error": "#ff5555",
    "success": "#50fa7b"
  }
}
```

Claude Code 監視 `~/.claude/themes/` 並在檔案變更時重新載入，因此在您的編輯器中所做的編輯會在執行中的工作階段中應用，無需重新啟動。

以下參考涵蓋了您可以在 `overrides` 中設定的令牌。`/theme` 中的互動編輯器顯示相同的令牌，並提供即時預覽，加上此處未涵蓋的少數單一用途重點，例如上線畫面色彩。

<Accordion title="色彩令牌參考">
  以下範例結合了下列幾個群組中的令牌：品牌重點、Plan Mode 邊框、diff 背景和全螢幕訊息背景。

  ```json ~/.claude/themes/midnight.json theme={null}
  {
    "name": "Midnight",
    "base": "dark",
    "overrides": {
      "claude": "#a78bfa",
      "planMode": "#38bdf8",
      "diffAdded": "#14532d",
      "diffRemoved": "#7f1d1d",
      "userMessageBackground": "#1e1b4b"
    }
  }
  ```

  <h4 id="text-and-accent-colors">
    文字和重點色彩
  </h4>

  控制整個介面中使用的主要品牌重點和前景文字陰影。

  | 令牌            | 控制項                  |
  | :------------ | :------------------- |
  | `claude`      | 主要品牌重點，用於微調器和助手標籤    |
  | `text`        | 預設前景文字               |
  | `inverseText` | 繪製在彩色背景上的文字，例如狀態徽章   |
  | `inactive`    | 次要文字，例如提示、時間戳記和停用的項目 |
  | `subtle`      | 淡色邊框和去強調的次要文字        |
  | `suggestion`  | 自動完成建議和選擇器中的選擇突出顯示   |
  | `permission`  | 對話方塊邊框，包括權限提示和選擇器    |
  | `remember`    | 記憶體和 `CLAUDE.md` 指示器 |

  <h4 id="status-colors">
    狀態色彩
  </h4>

  在訊息和指示器中發出成功、失敗和警告狀態的信號。

  | 令牌        | 控制項            |
  | :-------- | :------------- |
  | `success` | 成功訊息和通過的檢查     |
  | `error`   | 錯誤訊息和失敗        |
  | `warning` | 警告、注意訊息和自動模式邊框 |
  | `merged`  | 合併的提取要求狀態      |

  <h4 id="input-box-and-mode-indicators">
    輸入框和模式指示器
  </h4>

  設定輸入框邊框色彩和權限模式或指示器作用中時顯示的重點。

  | 令牌             | 控制項                    |
  | :------------- | :--------------------- |
  | `promptBorder` | 預設權限模式中的輸入框邊框          |
  | `planMode`     | Plan Mode 重點和邊框        |
  | `autoAccept`   | Accept-edits 模式重點和邊框   |
  | `bashBorder`   | 輸入 `!` shell 命令時的輸入框邊框 |
  | `ide`          | IDE 連線指示器              |
  | `fastMode`     | 快速模式指示器                |

  <h4 id="diff-rendering">
    Diff 呈現
  </h4>

  在檔案編輯和審查中著色新增和移除的程式碼。

  | 令牌                  | 控制項            |
  | :------------------ | :------------- |
  | `diffAdded`         | 新增行的背景         |
  | `diffRemoved`       | 移除行的背景         |
  | `diffAddedDimmed`   | 新增行附近未變更上下文的背景 |
  | `diffRemovedDimmed` | 移除行附近未變更上下文的背景 |
  | `diffAddedWord`     | 新增行內的字級突出顯示    |
  | `diffRemovedWord`   | 移除行內的字級突出顯示    |

  <h4 id="fullscreen-mode">
    全螢幕模式
  </h4>

  僅在[全螢幕呈現模式](/docs/zh-TW/fullscreen)中套用，其中訊息具有背景填充。

  | 令牌                           | 控制項                       |
  | :--------------------------- | :------------------------ |
  | `userMessageBackground`      | 文字記錄中您的訊息後面的背景            |
  | `userMessageBackgroundHover` | 訊息被懸停或展開時其後面的背景           |
  | `messageActionsBackground`   | 動作列開啟時所選訊息後面的背景           |
  | `bashMessageBackgroundColor` | 文字記錄中 `!` shell 命令項目後面的背景 |
  | `memoryBackgroundColor`      | 文字記錄中 `#` 記憶體項目後面的背景      |
  | `selectionBg`                | 使用滑鼠選取的文字背景               |

  <h4 id="usage-meter-and-speaker-labels">
    使用量計量和說話者標籤
  </h4>

  調整 `/usage` 檢視中顯示的列，以及區分您的訊息與 Claude 訊息的標籤。

  | 令牌                 | 控制項                  |
  | :----------------- | :------------------- |
  | `rate_limit_fill`  | 使用量計量的填充部分           |
  | `rate_limit_empty` | 使用量計量的未填充部分          |
  | `briefLabelYou`    | 您的訊息上 `You` 標籤的色彩    |
  | `briefLabelClaude` | 助手訊息上 `Claude` 標籤的色彩 |

  <h4 id="shimmer-variants-and-subagent-colors">
    微光變體和子代理色彩
  </h4>

  多個令牌具有配對的微光變體，可提供微調器動畫漸層中使用的較淺色彩。如果動畫看起來不相符，請與其基礎令牌一起覆蓋微光。

  * `claude` 和 `claudeShimmer`
  * `warning` 和 `warningShimmer`
  * `permission` 和 `permissionShimmer`
  * `promptBorder` 和 `promptBorderShimmer`
  * `inactive` 和 `inactiveShimmer`
  * `fastMode` 和 `fastModeShimmer`

  每個[子代理](/docs/zh-TW/sub-agents)和平行工作都以八個命名色彩之一顯示，以便您可以在文字記錄中區分它們。令牌名稱遵循 `<color>_FOR_SUBAGENTS_ONLY` 的模式，其中 `<color>` 是 `red`、`blue`、`green`、`yellow`、`purple`、`orange`、`pink` 或 `cyan`。覆蓋這些以變更每個命名色彩的外觀。例如，定義中具有 `color: blue` 的子代理使用 `blue_FOR_SUBAGENTS_ONLY` 值繪製。

  [`ultrathink`](/docs/zh-TW/model-config#use-ultrathink-for-one-off-deep-reasoning) 和 [`ultraplan`](/docs/zh-TW/ultraplan) 提示輸入中的關鍵字使用七色彩虹漸層呈現。令牌名稱遵循 `rainbow_<color>` 和 `rainbow_<color>_shimmer` 的模式，其中 `<color>` 是 `red`、`orange`、`yellow`、`green`、`blue`、`indigo` 或 `violet`。
</Accordion>

<h2 id="switch-to-fullscreen-rendering">
  切換到全螢幕渲染
</h2>

如果顯示閃爍或捲動位置在 Claude 工作時跳躍，請切換到[全螢幕渲染模式](/docs/zh-TW/fullscreen)。它繪製到終端機為全螢幕應用程式保留的單獨螢幕，而不是附加到您的正常捲動，這保持記憶體使用平穩並新增滑鼠支援以進行捲動和選擇。在此模式中，您使用滑鼠或 PageUp 在 Claude Code 內捲動，而不是使用您的終端機的原生捲動；請參閱[全螢幕頁面](/docs/zh-TW/fullscreen#search-and-review-the-conversation)以瞭解如何搜尋和複製。

如果閃爍是唯一的問題，且您的終端機支援同步輸出但未被自動偵測，例如 Emacs `eat`，請設定 [`CLAUDE_CODE_FORCE_SYNC_OUTPUT=1`](/docs/zh-TW/env-vars) 以停止閃爍而不改變渲染器。

執行 `/tui fullscreen` 以切換並儲存偏好設定。您的對話會完整重新啟動，未來的工作階段會以全螢幕開始。您也可以在啟動 Claude Code 之前設置 `CLAUDE_CODE_NO_FLICKER` 環境變數：

<CodeGroup>
  ```bash Bash and Zsh theme={null}
  CLAUDE_CODE_NO_FLICKER=1 claude
  ```

  ```powershell PowerShell theme={null}
  $env:CLAUDE_CODE_NO_FLICKER = "1"; claude
  ```

  ```json ~/.claude/settings.json theme={null}
  {
    "env": {
      "CLAUDE_CODE_NO_FLICKER": "1"
    }
  }
  ```
</CodeGroup>

<h2 id="paste-large-content">
  貼上大型內容
</h2>

當您將超過 10,000 個字元貼上到提示中時，Claude Code 會將輸入摺疊為 `[Pasted text]` 預留位置，以便輸入框保持可用。完整內容在您提交時仍會發送到 Claude。

VS Code 整合終端機可能會在非常大的貼上中丟棄字元，然後才能到達 Claude Code，因此在那裡更喜歡基於檔案的工作流程。對於非常大的輸入（例如整個檔案或長日誌），請將內容寫入檔案並要求 Claude 讀取它，而不是貼上。這保持對話記錄可讀，並讓 Claude 在稍後的回合中按路徑參考檔案。

<h2 id="edit-prompts-with-vim-keybindings">
  使用 Vim 快捷鍵編輯提示
</h2>

Claude Code 包括提示輸入的 Vim 風格編輯模式。透過 `/config` → 編輯器模式啟用它，或透過在 `~/.claude/settings.json` 中將 [`editorMode`](/docs/zh-TW/settings#available-settings) 設置為 `"vim"` 啟用它。將編輯器模式設置回 `normal` 以將其關閉。

Vim 模式支援 NORMAL 模式和 VISUAL 模式動作和運算子的子集，例如 `hjkl` 導覽、`v`/`V` 選取，以及 `d`/`c`/`y` 搭配文字物件。請參閱 [Vim 編輯器模式參考](/docs/zh-TW/interactive-mode#vim-editor-mode)以取得完整快捷鍵表。

Vim 動作無法透過快捷鍵檔案重新對應。若要將兩個按鍵的 INSERT 模式序列（例如 `jj`）對應到 Escape，請在您的使用者設定中設置 [`vimInsertModeRemaps`](/docs/zh-TW/interactive-mode#remap-insert-mode-key-sequences)。

在 INSERT 模式中按 Enter 仍會提交您的提示，不同於標準 Vim。在 NORMAL 模式中使用 `o` 或 `O`，或 Ctrl+J，以插入換行符。

<h2 id="related-resources">
  相關資源
</h2>

* [互動模式](/docs/zh-TW/interactive-mode)：完整鍵盤快捷鍵參考和 Vim 快捷鍵表
* [快捷鍵](/docs/zh-TW/keybindings)：重新對應任何 Claude Code 快捷鍵，包括 Enter 和 Shift+Enter
* [全螢幕渲染](/docs/zh-TW/fullscreen)：全螢幕模式中捲動、搜尋和複製的詳細資訊
* [Hooks 指南](/docs/zh-TW/hooks-guide)：Linux 和 Windows 的更多通知 hook 範例
* [疑難排解](/docs/zh-TW/troubleshooting)：終端機配置外部問題的修復
