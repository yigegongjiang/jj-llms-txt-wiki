> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用 Claude Code 搭配螢幕閱讀器

> 為 VoiceOver 和 NVDA 等螢幕閱讀器設定 Claude Code，以及螢幕放大鏡、減少動畫和色盲友善主題的設定。

Claude Code 具有螢幕閱讀器模式，可將其視覺終端介面替換為純文字、線性文字。該模式不使用方框、進度動畫和就地重繪，而是列印標記的行，螢幕閱讀器（例如 VoiceOver 或 NVDA）會依序讀取這些行，讓您可以進行完整對話、批准工具權限並從頭到尾檢查輸出。

螢幕閱讀器模式是選擇性加入的。如果您使用螢幕放大鏡、減少動畫或色盲友善主題而不是螢幕閱讀器，請參閱[螢幕閱讀器模式以外的無障礙設定](#accessibility-settings-beyond-screen-reader-mode)。

<Note>
  螢幕閱讀器模式需要 Claude Code v2.1.181 或更新版本。較早版本會以 `error: unknown option '--ax-screen-reader'` 拒絕 `--ax-screen-reader` 旗標。
</Note>

<h2 id="turn-on-screen-reader-mode">
  開啟螢幕閱讀器模式
</h2>

選擇與您使用螢幕閱讀器頻率相符的方法：

* 針對一個工作階段：執行 `claude --ax-screen-reader`。
* 針對從一個 shell 啟動的工作階段：將 `CLAUDE_AX_SCREEN_READER` 環境變數設定為 `1`。在 Bash 或 Zsh 中，執行 `export CLAUDE_AX_SCREEN_READER=1`；在 PowerShell 中，執行 `$env:CLAUDE_AX_SCREEN_READER = "1"`。將該行新增至您的 shell 設定檔以涵蓋每個 shell。
* 針對機器上的每個工作階段：將 `"axScreenReader": true` 新增至您的使用者[設定檔](/docs/zh-TW/settings)。這涵蓋任何終端，包括 VS Code 整合終端。

<Note>
  這些方法按優先順序列出：[`--ax-screen-reader`](/docs/zh-TW/cli-reference#cli-flags) 旗標會覆寫 [`CLAUDE_AX_SCREEN_READER`](/docs/zh-TW/env-vars) 環境變數，而環境變數會覆寫 [`axScreenReader`](/docs/zh-TW/settings#available-settings) 設定。
</Note>

如果您透過 SSH 使用 Claude Code，請在執行 Claude Code 的遠端機器上設定環境變數或設定。

當模式開啟時，Claude Code 列印的第一件事是確認行，命名開啟它的方法：`[Screen Reader Mode: on via flag]`、`[Screen Reader Mode: on via env]` 或 `[Screen Reader Mode: on via settings]`。此方法命名格式需要 Claude Code v2.1.206 或更新版本。當 Claude Code 重新啟動自身時（例如完成安裝更新），新程序會透過 `CLAUDE_AX_SCREEN_READER` 環境變數繼承該模式，因此其確認行會讀取 `[Screen Reader Mode: on via env]`，無論您使用了哪種方法。
較早版本會列印 `[Accessible screen reader mode: on]`。

<h2 id="turn-off-screen-reader-mode">
  關閉螢幕閱讀器模式
</h2>

反轉開啟模式的任何方法：不使用旗標啟動、取消設定環境變數，或將 `axScreenReader` 設定為 `false`。設定 `CLAUDE_AX_SCREEN_READER=0` 即使設定為 `true` 也會保持模式關閉。

<h2 id="what-your-screen-reader-hears">
  您的螢幕閱讀器聽到的內容
</h2>

在螢幕閱讀器模式中，Claude Code 寫入平面文字：

* 介面 chrome 沒有方框繪製字元
* 沒有僅限顏色的提示
* 沒有未變更內容的重繪；進度微調器呈現為靜態文字
* Claude 回覆中的表格讀作 `Header: value` 句子而不是方框字元網格。需要 Claude Code v2.1.198 或更新版本；較早版本即使在螢幕閱讀器模式下也會將表格繪製為網格。

輸出會累積在您終端的回滾中，因此您可以使用螢幕閱讀器的檢查命令或終端的搜尋功能重新閱讀較早的回合。

螢幕閱讀器模式呈現為純滾動文字，即使您已使用 [`tui` 設定](/docs/zh-TW/settings#available-settings)開啟[全螢幕呈現](/docs/zh-TW/fullscreen)；當模式啟用時，該設定無效。附加的背景工作階段仍會全螢幕呈現；請參閱[已知限制](#known-limitations)。

文字記錄中的每條訊息都以標籤開頭，您的螢幕閱讀器會宣佈該標籤，命名其內容：您的訊息、Claude 的回覆、工具活動、錯誤和提示。這些標籤也可搜尋，因此您可以透過搜尋終端的回滾在文字記錄的各個部分之間跳躍：

| 標籤                     | 含義                                                   |
| :--------------------- | :--------------------------------------------------- |
| `you:`                 | 您的訊息                                                 |
| `claude:`              | Claude 的回覆                                           |
| `tool:`                | 工具活動，例如檔案編輯或執行的命令                                    |
| `tool error:`          | 失敗的工具                                                |
| `error:`               | 對話中的錯誤，例如失敗的 API 請求                                  |
| `Permission Required:` | 等待您回答的權限提示                                           |
| `Cost:`                | Claude Code 結束時的工作階段成本摘要（如果您的帳戶[顯示成本](/docs/zh-TW/costs)） |

終端游標跟隨輸入插入符號，因此螢幕閱讀器的讀取目前行命令會以您正在編輯的提示回答「我在哪裡」。

<h3 id="jump-between-turns">
  在回合之間跳躍
</h3>

Claude Code 在回合邊界處發出 OSC 133 shell 整合標記，因此您終端的跳至上一個提示鍵會在回合之間移動，而無需讀取整個文字記錄：

* iTerm2：Cmd+Shift+Up
* VS Code 終端：Windows 上的 Ctrl+Up，macOS 上的 Cmd+Up
* Windows Terminal：預設沒有鍵；在其設定中繫結 `scrollToMark` 動作
* Kitty 和 Ghostty：檢查終端的文件以了解其跳至提示鍵

macOS Terminal 不會對標記進行操作，Claude Code 在 WezTerm 中不會發出標記。在這些終端中，改為搜尋回滾中的 `you:` 標籤。

<h2 id="answer-menus-and-prompts">
  回答選單和提示
</h2>

在螢幕閱讀器模式中，您通常使用方向鍵導覽的選單（包括權限提示）會變成編號清單。每個選項都會宣佈為編號行，後面跟著 `Enter selection` 提示，該提示命名有效範圍。輸入您想要的選項編號並按 Enter。

* 若要取消可關閉的選單：按 Escape。其提示以 `or Escape to cancel` 結尾。
* 如果您輸入清單上沒有的編號：Claude Code 會宣佈有效範圍並讓您重試。

是或否提示要求輸入答案而不是兩選項選單。回答 `y` 或 `n` 並按 Enter。`yes` 和 `no` 也可以。

<h2 id="hear-when-claude-code-needs-you">
  聽到 Claude Code 何時需要您
</h2>

在螢幕閱讀器模式中，Claude Code 會在需要您注意時響起終端鈴聲，因此您不必持續檢查文字記錄。鈴聲在以下情況下響起：

* Claude 完成回覆
* 出現權限提示
* 執行時間超過 5 秒的工具完成

鈴聲是您終端的標準警報。若要將其靜音，請變更您終端應用程式中的鈴聲設定。鈴聲不需要螢幕閱讀器模式：在模式外，將 [`preferredNotifChannel`](/docs/zh-TW/settings#available-settings) 設定為 `"terminal_bell"` 以在 Claude 等待您時獲得類似警報。請參閱[取得終端鈴聲或通知](/docs/zh-TW/terminal-config#get-a-terminal-bell-or-notification)。

<h2 id="accessibility-settings-beyond-screen-reader-mode">
  螢幕閱讀器模式以外的無障礙設定
</h2>

這些選項可解決螢幕閱讀器模式以外的無障礙需求。它們都可與其一起使用。

* `CLAUDE_CODE_ACCESSIBILITY` [環境變數](/docs/zh-TW/env-vars)適用於螢幕放大鏡。設定 `CLAUDE_CODE_ACCESSIBILITY=1` 以保持原生終端游標可見，以便放大鏡（例如 macOS Zoom）可以追蹤游標位置。
* `prefersReducedMotion` [設定](/docs/zh-TW/settings#available-settings)可減少或停用微調器、閃爍和其他動畫，而不會變更介面的其餘部分。
* `theme` [設定](/docs/zh-TW/settings#available-settings)選擇介面顏色，包括色盲友善的 `dark-daltonized` 和 `light-daltonized` 主題。

<h2 id="known-limitations">
  已知限制
</h2>

某些行為未針對螢幕閱讀器模式進行調整：

* 當螢幕閱讀器執行時，螢幕閱讀器模式不會自動開啟。
* 模式變更（例如進入[計畫模式](/docs/zh-TW/permission-modes#analyze-before-you-edit-with-plan-mode)）尚未宣佈。
* 使用 `claude attach` 或從代理檢視附加到[背景工作階段](/docs/zh-TW/agent-view)會進入終端的替代螢幕，該螢幕沒有原生回滾。這與[其他附加工作階段的行為相同](/docs/zh-TW/fullscreen)。若要返回，請在空提示上按左箭頭，或如果對話框有焦點，請按 Ctrl+Z。
* Claude Code 在其在結束時列印的摘要中宣佈成本，而不是按回合。
* 螢幕閱讀器模式不會使用 `-p` 旗標變更[非互動模式](/docs/zh-TW/headless)。非互動模式已寫入純文字，並保持為指令碼的替代方案。

<h2 id="report-an-issue">
  報告問題
</h2>

如果螢幕閱讀器、放大鏡或終端出現問題，請在 [Claude Code 問題追蹤器](https://github.com/anthropics/claude-code/issues)上開啟問題，並在標題中提及您的輔助技術。在報告中包含您的作業系統、終端應用程式以及輔助技術名稱和版本。

<h2 id="related-resources">
  相關資源
</h2>

這些頁面包含此頁面涵蓋內容的完整參考項目和相關設定：

* [設定](/docs/zh-TW/settings#available-settings)：`axScreenReader`、`prefersReducedMotion`、`theme` 和 `preferredNotifChannel` 項目
* [環境變數](/docs/zh-TW/env-vars)：`CLAUDE_AX_SCREEN_READER` 和 `CLAUDE_CODE_ACCESSIBILITY` 項目
* [CLI 參考](/docs/zh-TW/cli-reference#cli-flags)：`--ax-screen-reader` 旗標
* [終端配置](/docs/zh-TW/terminal-config)：螢幕閱讀器模式外的鈴聲、通知和主題
* [非互動模式](/docs/zh-TW/headless)：指令碼化 `claude -p` 執行，不使用螢幕閱讀器模式寫入純文字
