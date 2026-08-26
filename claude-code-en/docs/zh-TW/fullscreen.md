> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 全螢幕渲染

> 啟用更平順、無閃爍的渲染模式，具有滑鼠支援和穩定的記憶體使用，適用於長對話。

<Note>
  全螢幕渲染是一個選擇加入的[研究預覽](#research-preview)。在您目前的對話中執行 `/tui fullscreen` 以切換。行為可能會根據回饋而改變。
</Note>

全螢幕渲染是 Claude Code CLI 的替代渲染路徑，可消除閃爍、在長對話中保持記憶體使用平穩，並新增滑鼠支援。它在終端的替代螢幕緩衝區上繪製介面，就像 `vim` 或 `htop` 一樣，並且只渲染目前可見的訊息。這減少了每次更新時傳送到終端的資料量。

在渲染吞吐量是瓶頸的終端模擬器中，差異最為明顯，例如 VS Code 整合終端、tmux 和 iTerm2。如果您的終端捲動位置在 Claude 工作時跳到頂部，或者工具輸出串流進來時螢幕閃爍，此模式可以解決這些問題。

<Note>
  全螢幕一詞描述的是 Claude Code 如何接管終端的繪製表面，就像 `vim` 一樣。它與最大化終端視窗無關，並且在任何視窗大小下都能運作。
</Note>

<h2 id="enable-fullscreen-rendering">
  啟用全螢幕渲染
</h2>

在任何 Claude Code 對話中執行 `/tui fullscreen`。CLI 會儲存 [`tui` 設定](/docs/zh-TW/settings#available-settings)並重新啟動進入全螢幕模式，您的對話保持完整，因此您可以在工作階段中途切換而不會失去上下文。執行 `/tui default` 以切換回經典渲染器，或執行 `/tui` 不帶任何引數以列印哪個渲染器處於活動狀態。

重新啟動的工作階段會保持對話在螢幕上顯示的樣子。如果您在工作階段中較早執行過 [`/rewind`](/docs/zh-TW/checkpointing#rewind-and-summarize)，重新啟動會從倒帶點而不是儲存在磁碟上的較長文字記錄繼續。在 v2.1.207 之前，在倒帶後切換渲染器會還原倒帶已移除的對話。

您也可以在啟動 Claude Code 之前設定 `CLAUDE_CODE_NO_FLICKER` 環境變數：

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 claude
```

`tui` 設定和環境變數是等效的。`/tui` 命令會從重新啟動的程序中清除 `CLAUDE_CODE_NO_FLICKER`，以便它寫入的設定生效。

<h2 id="what-changes">
  變更內容
</h2>

全螢幕渲染改變了 CLI 如何繪製到您的終端。輸入框保持固定在螢幕底部，而不是在輸出串流進來時移動。如果輸入在 Claude 工作時保持不動，則全螢幕渲染處於活動狀態。只有可見的訊息保留在渲染樹中，因此無論對話長度如何，記憶體都保持恆定。

因為對話存在於替代螢幕緩衝區而不是終端的捲動回溯，所以有幾件事的運作方式不同：

| 之前                     | 現在                                      | 詳細資訊                                           |
| :--------------------- | :-------------------------------------- | :--------------------------------------------- |
| `Cmd+f` 或 tmux 搜尋來尋找文字 | `Ctrl+o` 進入文字記錄模式，然後 `/` 搜尋或 `[` 寫入捲動回溯 | [搜尋和檢閱對話](#search-and-review-the-conversation) |
| 終端的原生點擊並拖曳來選擇和複製       | 應用程式內選擇，在滑鼠釋放時自動複製                      | [使用滑鼠](#use-the-mouse)                         |
| `Cmd` 點擊來開啟 URL        | macOS 上的 `Cmd` 點擊，其他地方的 `Ctrl` 點擊       | [使用滑鼠](#use-the-mouse)                         |

如果滑鼠捕捉干擾您的工作流程，您可以[關閉它](#keep-native-text-selection)，同時保持無閃爍渲染。

<h2 id="use-the-mouse">
  使用滑鼠
</h2>

全螢幕渲染捕捉滑鼠事件並在 Claude Code 內處理它們：

* **在提示輸入中點擊**以在您輸入的文字中的任何位置定位游標。
* **點擊 `/` 命令或 `@` 檔案清單中的建議**以接受它。懸停會突顯游標下的列。
* **點擊選擇功能表中的選項**以選擇它。這涵蓋權限提示、`/model`、`/config` 和其他顯示選項清單的對話框。懸停會在游標下的列上顯示指標。需要 Claude Code v2.1.187 或更新版本。
* **點擊多選功能表中的選項**以切換它，然後點擊提交按鈕以確認您的選擇。點擊自由文字列（例如多選題中的 `Other` 列）會聚焦其輸入欄位，以便您可以輸入答案。需要 Claude Code v2.1.208 或更新版本。
* **點擊摺疊的工具結果**以展開它並查看完整輸出。再次點擊以摺疊。工具呼叫及其結果一起展開。只有有更多內容要顯示的訊息才可點擊。
* **在 macOS 上按住 `Cmd`，或在 Linux 和 Windows 上按住 `Ctrl`，然後點擊 URL 或檔案路徑**以開啟它。工具輸出中的檔案路徑（例如在 Edit 或 Write 後列印的路徑）在您的預設應用程式中開啟。純 `http://` 和 `https://` URL 在您的瀏覽器中開啟。自 v2.1.181 起，不按住 `Cmd` 或 `Ctrl` 的純點擊不再開啟連結，符合原生終端行為。某些 macOS 終端會將 `Cmd`+點擊轉發給執行中的應用程式，而不是自己開啟連結，且終端滑鼠協議無法編碼 `Cmd` 鍵，所以 Claude Code 將其接收為純點擊。在 Ghostty 中，以及自 v2.1.198 起在 macOS 上的 Warp 中，Claude Code 偵測到這一點並讓純點擊連結開啟它，按住 `Cmd` 仍然有效。在 VS Code 整合終端和類似的基於 xterm.js 的終端中，Claude Code 遵從終端自己的連結處理程式，該處理程式使用相同的手勢。
* **點擊並拖曳**以在對話中的任何位置選擇文字。雙擊選擇一個單詞，符合 iTerm2 的單詞邊界，因此檔案路徑選擇為一個單位。自 v2.1.198 起，雙擊 URL 會選擇整個 URL，包括配置。三擊選擇該行。
* **使用滑鼠滾輪捲動**以在對話中移動。

選定的文字在滑鼠釋放時自動複製到您的剪貼簿。若要關閉此功能，請在 `/config` 中切換「選擇時複製」。

關閉「選擇時複製」後，按 `Ctrl+Shift+c` 手動複製。在支援 kitty 鍵盤協議的終端上，例如 kitty、WezTerm、Ghostty 和 iTerm2，`Cmd+c` 也可以運作。如果您有活動選擇，`Ctrl+c` 複製而不是取消。

有活動選擇時，按住 `Shift` 並按方向鍵以從鍵盤延伸它。`Shift+↑` 和 `Shift+↓` 在選擇到達頂部或底部邊緣時捲動檢視區。`Shift+Home` 和 `Shift+End` 延伸到目前行的開始或結束。

<h2 id="scroll-the-conversation">
  捲動對話
</h2>

全螢幕渲染在應用程式內處理捲動。使用這些快捷鍵來導航：

| 快捷鍵             | 動作              |
| :-------------- | :-------------- |
| `PgUp` / `PgDn` | 向上或向下捲動半個螢幕     |
| `Ctrl+Home`     | 跳到對話的開始         |
| `Ctrl+End`      | 跳到最新訊息並重新啟用自動跟隨 |
| 滑鼠滾輪            | 一次捲動幾行          |

在沒有專用 `PgUp`、`PgDn`、`Home` 或 `End` 鍵的鍵盤上，例如 MacBook 鍵盤，按住 `Fn` 並使用方向鍵：`Fn+↑` 傳送 `PgUp`、`Fn+↓` 傳送 `PgDn`、`Fn+←` 傳送 `Home`、`Fn+→` 傳送 `End`。`Ctrl+Fn+→` 在 macOS 上無法到達 Claude Code，因此 MacBook 鍵盤預設沒有可用的跳到底部快捷鍵。請改用以下其中一個選項：

* 點擊[跳到底部按鈕](#auto-follow)。
* 使用滑鼠滾輪捲動到底部以恢復跟隨。
* 將 `scroll:bottom` 重新繫結到您的鍵盤可以傳送的快捷鍵。

這些動作是可重新繫結的。請參閱[捲動動作](/docs/zh-TW/keybindings#scroll-actions)以取得完整的動作名稱清單，包括沒有預設繫結的半頁和整頁變體。

<h3 id="auto-follow">
  自動跟隨
</h3>

向上捲動會暫停自動跟隨，以便新輸出不會將您拉回底部。當您向上捲動時，一個`跳到底部`按鈕會浮動在文字記錄的底部邊緣，當新輸出到達時會顯示計數，例如 `3 條新訊息`。點擊它、按 `Ctrl+End` 或捲動到底部以恢復跟隨。

自動跟隨暫停時，當回應完成串流時，檢視也會保持在您捲動的位置。在 v2.1.207 之前，當長回應完成串流時，檢視可能會跳到答案開始之上。

按鈕的鍵盤提示反映您的鍵盤可以傳送的內容。在 macOS 上，它建議點擊或 `Fn+↓` 來捲動，因為 `Ctrl+End` 無法從 Mac 鍵盤到達 Claude Code。重新繫結 [`scroll:bottom`](/docs/zh-TW/keybindings#scroll-actions)，按鈕會在每個平台上顯示您的快捷鍵。在 v2.1.206 之前，按鈕在 macOS 上建議 `Ctrl+End`。

在終端太窄而無法容納完整標籤的情況下，按鈕會縮短提示，而不是換行到下面的文字記錄行。在 v2.1.206 之前，長標籤可能會換行到文字記錄上。

若要完全關閉自動跟隨，使檢視保持在您留下的位置，請開啟 `/config` 並將「自動捲動」設定為關閉。禁用自動捲動後，檢視永遠不會自動跳到底部。需要回應的權限提示和其他對話框無論此設定如何都仍會捲動到檢視中。

<h3 id="mouse-wheel-scrolling">
  滑鼠滾輪捲動
</h3>

滑鼠滾輪捲動需要您的終端將滑鼠事件轉發給 Claude Code。大多數終端在應用程式要求時都會執行此操作。iTerm2 將其設為每個設定檔的設定：如果滾輪無法執行任何操作，但 `PgUp` 和 `PgDn` 運作，請開啟「設定」→「設定檔」→「終端」並開啟「啟用滑鼠報告」。點擊展開和文字選擇也需要相同的設定。

如果滑鼠滾輪捲動感覺很慢，您的終端可能每個物理凹口傳送一個捲動事件，沒有乘數。某些終端（例如 Ghostty 和啟用更快捲動的 iTerm2）已經放大了滾輪事件。其他終端（包括 VS Code 整合終端）每個凹口傳送恰好一個事件。Claude Code 無法偵測哪個。

設定 `CLAUDE_CODE_SCROLL_SPEED` 以乘以基本捲動距離：

```bash theme={null}
export CLAUDE_CODE_SCROLL_SPEED=3
```

值 `3` 符合 `vim` 和類似應用程式中的預設值。該設定接受 1 到 20 的值，以及低於 1 的分數值（例如 `0.5`）以減緩終端上原本放大的加速觸控板和滾輪捲動。

若要以互動方式調整捲動速度，請執行 `/scroll-speed`。對話框顯示一個尺標，您可以在其開啟時捲動，以便您可以立即感受到變化。按 `←` 和 `→` 以調整，按 `r` 以重設為自動偵測的預設值，按 `Enter` 以儲存。

該命令寫入與 `CLAUDE_CODE_SCROLL_SPEED` 環境變數設定相同的值，持久化到 `~/.claude/settings.json`。該命令在 JetBrains IDE 終端中不可用。

另外，Claude Code 會在您快速旋轉滾輪時加速捲動速率，因此快速旋轉覆蓋的距離比相同數量的慢凹口更遠。若要關閉加速並保持每個凹口的恆定速率，請在 [`settings.json`](/docs/zh-TW/settings#available-settings) 中將 `wheelScrollAccelerationEnabled` 設定為 `false`。此設定需要 Claude Code v2.1.174 或更新版本。

<h3 id="scroll-in-the-jetbrains-ide-terminal">
  JetBrains IDE 終端中的捲動
</h3>

在 JetBrains IDE 終端中，Claude Code 應用其自己的捲動處理並忽略 `CLAUDE_CODE_SCROLL_SPEED`。終端以比其他模擬器高得多的速率傳送捲動事件，因此在其他地方調整的乘數會在此處超出。

在 2025.2 中，終端也有捲動滾輪錯誤，會產生虛假的方向鍵和錯誤方向的事件。Claude Code 在執行時偵測這些錯誤並自動減輕它們，因此觸控板和滑鼠滾輪捲動無需設定即可運作。為了獲得最佳捲動體驗，請升級到 2025.3 或更新版本。如果 Claude Code 偵測到該錯誤，它會在您第一次捲動時顯示提示。

<h2 id="search-and-review-the-conversation">
  搜尋和檢閱對話
</h2>

`Ctrl+o` 在正常提示和文字記錄模式之間切換。

若要取得更安靜的檢視，只顯示您的最後一個提示、工具呼叫的單行摘要（含編輯 diffstats）和最終回應，請執行 `/focus`。該設定在工作階段之間保持。再次執行 `/focus` 以關閉它。

文字記錄模式獲得 `less` 風格的導航和搜尋：

| 鍵                                   | 動作                                         |
| :---------------------------------- | :----------------------------------------- |
| `/`                                 | 開啟搜尋。輸入以尋找符合項，`Enter` 接受，`Esc` 取消並恢復您的捲動位置 |
| `n` / `N`                           | 跳到下一個或上一個符合項。在您關閉搜尋列後運作                    |
| `j` / `k` 或 `↑` / `↓`               | 捲動一行                                       |
| `g` / `G` 或 `Home` / `End`          | 跳到頂部或底部                                    |
| `Ctrl+u` / `Ctrl+d`                 | 捲動半頁                                       |
| `Ctrl+b` / `Ctrl+f` 或 `Space` / `b` | 捲動整頁                                       |
| `Ctrl+o`、`Esc` 或 `q`                | 退出文字記錄模式並返回提示                              |

您的終端的 `Cmd+f` 和 tmux 搜尋看不到對話，因為它存在於替代螢幕緩衝區，而不是原生捲動回溯。若要將內容交還給您的終端，請先按 `Ctrl+o` 進入文字記錄模式，然後：

* **`[`**：將完整對話寫入您的終端的原生捲動回溯緩衝區，所有工具輸出都已展開。對話現在是終端中的普通文字，因此 `Cmd+f`、tmux 複製模式和任何其他原生工具都可以搜尋或選擇它。長工作階段在發生此情況時可能會暫停片刻。這會持續到您使用 `Esc` 或 `q` 退出文字記錄模式，這會讓您回到全螢幕渲染。下一個 `Ctrl+o` 重新開始。
* **`v`**：將對話寫入臨時檔案並在 `$VISUAL` 或 `$EDITOR` 中開啟它。

按 `Esc` 或 `q` 返回提示。

<h2 id="clear-the-conversation">
  清除對話
</h2>

在兩秒內按 `Ctrl+L` 兩次以執行 `/clear` 並開始新對話。第一次按下會重新繪製螢幕並顯示提示；第二次按下會清除對話。在 macOS 上，雙擊 `Cmd+K` 也會執行 `/clear`。

<h2 id="use-with-tmux">
  與 tmux 搭配使用
</h2>

全螢幕渲染在 tmux 內運作，有三個注意事項。

滑鼠滾輪捲動需要 tmux 的滑鼠模式。如果您的 `~/.tmux.conf` 尚未啟用它，請新增此行並重新載入您的設定：

```bash theme={null}
set -g mouse on
```

沒有滑鼠模式，滾輪事件會進入 tmux 而不是 Claude Code。使用 `PgUp` 和 `PgDn` 的鍵盤捲動無論如何都能運作。如果 Claude Code 偵測到 tmux 且滑鼠模式關閉，它會在啟動時列印一次性提示。

全螢幕渲染與 iTerm2 的 tmux 整合模式不相容，這是您使用 `tmux -CC` 進入的模式。在整合模式中，iTerm2 將每個 tmux 窗格渲染為原生分割，而不是讓 tmux 繪製到終端。替代螢幕緩衝區和滑鼠追蹤在那裡無法正確運作：滑鼠滾輪無法執行任何操作，雙擊可能會損壞終端狀態。不要在 `tmux -CC` 工作階段中啟用全螢幕渲染。在 iTerm2 內的常規 tmux（沒有 `-CC`）運作良好。

並非每個 tmux 版本都會套用應用程式的同步輸出，因此在 tmux 下重繪期間您可能會看到比直接在終端中執行 Claude Code 時更多的閃爍。如果閃爍明顯，特別是在 SSH 上，請升級到最新的 tmux 或在 tmux 外的自己的終端標籤中執行 Claude Code。使用 `tmux -V` 檢查您的 tmux 版本。

Claude Code 在偵測到來自 `TERM_PROGRAM_VERSION` 變數的 tmux 3.4 或更新版本時會自動開啟同步輸出，當無法判斷版本時會回退到直接查詢終端以取得同步輸出支援。重繪是否實際上變成原子操作取決於您的 tmux 版本是否遵守同步輸出；如果您在 tmux 3.4 或更新版本下仍然看到閃爍，請升級到最新的 tmux。此偵測需要 Claude Code v2.1.200 或更新版本。

<h2 id="keep-native-text-selection">
  保持原生文字選擇
</h2>

滑鼠捕捉是最常見的摩擦點，特別是在 SSH 上或 tmux 內。當 Claude Code 捕捉滑鼠事件時，您的終端的原生複製選擇停止運作。您使用點擊並拖曳進行的選擇存在於 Claude Code 內，而不是在您的終端的選擇緩衝區中，因此 tmux 複製模式、Kitty 提示和類似工具看不到它。

Claude Code 將選擇寫入您的系統剪貼簿，它使用的路徑取決於您的設定。在本機工作階段上，它執行原生剪貼簿工具：

* **macOS**: `pbcopy`
* **Linux**: Wayland 上的 `wl-copy`，或 X11 上已安裝的 `xclip` 或 `xsel`。Claude Code 同時寫入剪貼簿和 PRIMARY 選擇，因此中鍵貼上可以運作。
* **Windows 和 WSL**: PowerShell `Set-Clipboard`

在 tmux 內，它也寫入 tmux 貼上緩衝區。在 SSH 上，它回退到 OSC 52 逃逸序列。Claude Code 在每次複製後列印一個快顯通知，告訴您它使用了哪個路徑。

某些終端預設會阻止 OSC 52。iTerm2 會阻止它，直到您開啟 Settings → General → Selection → Applications in terminal may access clipboard；在 iTerm2 中執行 [`/terminal-setup`](/docs/zh-TW/terminal-config) 會為您啟用此功能。

如果您想進行一次性的原生選擇，要使用的按鍵取決於您的終端：

* **Terminal.app**: `Fn`
* **iTerm2**: `Option`
* **VS Code、Cursor 和 Devin Desktop**: `Shift`，或在啟用 `terminal.integrated.macOptionClickForcesSelection` 設定的 macOS 上使用 `Option`
* **大多數其他終端**: `Shift`

按住該按鍵同時點擊並拖曳。您的終端自己處理選擇，而不是將其傳遞給 Claude Code，因此複製快捷鍵如 `Cmd+C` 可以在您選擇的內容上運作。Claude Code 也會在其螢幕上的提示中顯示正確的按鍵。

在 SSH 上或 tmux 內，Claude Code 無法總是偵測您連接的終端，因此提示會列出候選按鍵。

如果您一直依賴原生選擇，請設定 `CLAUDE_CODE_DISABLE_MOUSE=1` 以選擇退出滑鼠捕捉，同時保持無閃爍渲染和平穩記憶體：

```bash theme={null}
CLAUDE_CODE_NO_FLICKER=1 CLAUDE_CODE_DISABLE_MOUSE=1 claude
```

禁用滑鼠捕捉後，使用 `PgUp`、`PgDn`、`Ctrl+Home` 和 `Ctrl+End` 的鍵盤捲動仍然運作，您的終端原生處理選擇。您會失去點擊定位游標、點擊展開工具輸出、URL 點擊和 Claude Code 內的滾輪捲動。

若要保持滾輪捲動但關閉點擊、拖曳和懸停處理，請改為設定 `CLAUDE_CODE_DISABLE_MOUSE_CLICKS=1`。需要 Claude Code v2.1.195 或更新版本。當兩個變數都設定時，`CLAUDE_CODE_DISABLE_MOUSE` 優先。

禁用點擊後，Claude Code 仍然捕捉滑鼠，因此滾輪和觸控板捲動對話，但左鍵點擊在 Claude Code 內不執行任何操作。您仍然需要按住終端的按鍵進行原生點擊並拖曳選擇。右鍵點擊和中鍵貼上在支援它們的終端上繼續運作。

<h2 id="troubleshooting">
  疑難排解
</h2>

<h3 id="stale-or-misplaced-text-on-screen">
  螢幕上出現過時或位置錯誤的文字
</h3>

全螢幕渲染只會傳送幀之間變更的儲存格。某些終端機（最常見的是 Windows Terminal 和其他 ConPTY 支援的主機）會不正確地合併這些定位寫入，並在您調整視窗大小之前，在螢幕上留下較早輸出的片段。

設定 [`CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT=1`](/docs/zh-TW/env-vars) 以在每一幀上重新繪製每個儲存格，而不是傳送增量更新。

在 Windows PowerShell 上：

```powershell theme={null}
$env:CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT = "1"
claude
```

在 macOS 或 Linux 上：

```bash theme={null}
CLAUDE_CODE_ALT_SCREEN_FULL_REPAINT=1 claude
```

在 Windows 上，Claude Code 已自動為背景工作階段和 [agent view](/docs/zh-TW/agent-view) 啟用完整重繪，因此您只需要為直接啟動的互動式全螢幕工作階段設定該變數。

<h2 id="research-preview">
  研究預覽
</h2>

全螢幕渲染是一個研究預覽功能。它已在常見終端模擬器上進行測試，但您可能會在不太常見的終端或不尋常的設定上遇到渲染問題。

如果您遇到問題，請在 Claude Code 內執行 `/feedback` 以報告它，或在 [claude-code GitHub 儲存庫](https://github.com/anthropics/claude-code/issues)上開啟問題。包括您的終端模擬器名稱和版本。

若要關閉全螢幕渲染，請執行 `/tui default`，或如果您以該方式啟用它，請取消設定 `CLAUDE_CODE_NO_FLICKER`。若要不論已儲存的 `tui` 設定為何都強制使用經典渲染器，請設定 `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1`。經典渲染器將對話保留在您終端的原生捲軸中，因此 `Cmd+f` 和 tmux 複製模式可以照常運作。

從 [agent view](/docs/zh-TW/agent-view) 或 `claude attach` 開啟的背景工作階段始終使用全螢幕渲染。附加終端進入替代螢幕緩衝區以顯示工作階段，經典渲染器在那裡沒有捲軸或滑鼠處理，因此 `tui` 設定和 `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN` 不適用於它們。
