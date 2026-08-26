> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 讓 Claude 朝著目標持續工作

> 使用 /goal 設定完成條件，Claude 會在多個回合中持續工作直到條件滿足。

<Note>
  `/goal` 需要 Claude Code v2.1.139 或更新版本。
</Note>

`/goal` 命令設定完成條件，Claude 會朝著該目標持續工作，無需你在每一步進行提示。在每個回合後，一個小型快速模型會檢查條件是否滿足。如果未滿足，Claude 會開始另一個回合，而不是將控制權返回給你。一旦條件滿足，目標會自動清除。

在具有可驗證終止狀態的實質性工作中使用目標：

* 將模組遷移到新 API，直到每個呼叫位置都編譯並通過測試
* 實現設計文件，直到所有驗收標準都滿足
* 將大型檔案分割成專注的模組，直到每個模組都在大小預算內
* 處理標記的問題待辦清單，直到佇列為空

<h2 id="compare-ways-to-keep-a-session-running">
  比較保持工作階段執行的方式
</h2>

三種方法在提示之間保持目前工作階段執行。根據應該開始下一個回合的內容進行選擇：

| 方法                                                                     | 下一個回合開始於 | 停止於                   |
| :--------------------------------------------------------------------- | :------- | :-------------------- |
| `/goal`                                                                | 上一個回合完成時 | 模型確認條件已滿足時            |
| [`/loop`](/docs/zh-TW/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop) | 時間間隔經過時  | 你停止它，或 Claude 決定工作完成時 |
| [Stop hook](/docs/zh-TW/hooks-guide#prompt-based-hooks)                     | 上一個回合完成時 | 你自己的指令碼或提示決定時         |

`/goal` 和 Stop hook 都在每個回合後觸發。`/goal` 是一個工作階段範圍的快捷方式：你輸入條件，它僅在目前工作階段中有效。Stop hook 存在於你的設定檔案中，適用於其範圍內的每個工作階段，可以執行指令碼進行確定性檢查或執行提示進行模型評估的檢查。

[自動模式](/docs/zh-TW/auto-mode-config)本身在單個回合內批准工具呼叫，但不會開始新的回合。Claude 在判斷工作完成時停止。`/goal` 添加了一個單獨的評估器，在每個回合後檢查你的條件，因此完成由新鮮模型而不是執行工作的模型決定。這兩者是互補的：自動模式移除每個工具的提示，`/goal` 移除每個回合的提示。

<Tip>
  上述方法保持目前工作階段執行。你也可以排程獨立於任何開啟工作階段的工作，例如夜間測試或早晨分類。有關雲端例程和桌面排程任務，請參閱[排程選項](/docs/zh-TW/scheduled-tasks#compare-scheduling-options)。
</Tip>

<h2 id="use-/goal">
  使用 `/goal`
</h2>

每個工作階段可以有一個活躍的目標。相同的命令根據引數設定、檢查和清除它。

<h3 id="set-a-goal">
  設定目標
</h3>

執行 `/goal` 後跟你想要滿足的條件。如果已有活躍的目標，新目標會替換它。

```text theme={null}
/goal all tests in test/auth pass and the lint step is clean
```

設定目標會立即開始一個回合，條件本身作為指令。你無需發送單獨的提示。當目標活躍時，`◎ /goal active` 指示器顯示目標已執行多長時間。

目標不會改變權限。在預設權限模式中，Claude 仍會在工具呼叫前詢問，這些工具呼叫是你的設定尚未允許的，例如上面的測試命令。要讓目標回合無人值守地執行，請將 `/goal` 與 [auto mode](/docs/zh-TW/auto-mode-config) 配對。

在每個回合後，評估器返回簡短的原因，說明條件是否滿足。最新的原因出現在狀態檢視和文字記錄中，因此你可以看到 Claude 接下來要朝著什麼工作。

<Note>
  目標會持續執行，直到條件滿足或你執行 `/goal clear`。執行不帶引數的 `/goal` 以查看迄今為止花費的回合和令牌。
</Note>

<h3 id="write-an-effective-condition">
  編寫有效條件
</h3>

[評估器](#how-evaluation-works)根據 Claude 在對話中呈現的內容判斷你的條件。它不會獨立執行命令或讀取檔案，因此將條件編寫為 Claude 自己的輸出可以演示的內容。「`test/auth` 中的所有測試都通過」有效，因為 Claude 執行測試，結果出現在文字記錄中供評估器讀取。

在許多回合中保持的條件通常具有：

* **一個可測量的終止狀態**：測試結果、建置退出代碼、檔案計數、空佇列
* **一個陳述的檢查**：Claude 應該如何證明它，例如「`npm test` 退出 0」或「`git status` 是乾淨的」
* **重要的約束**：在此過程中必須不改變的任何內容，例如「沒有其他測試檔案被修改」

條件最多可以是 4,000 個字元。

要限制目標執行的時間，在條件中包含回合或時間子句，例如 `or stop after 20 turns`。Claude 每個回合都報告針對該子句的進度，評估器從對話中判斷它。

<h3 id="check-status">
  檢查狀態
</h3>

執行不帶引數的 `/goal` 以查看目前狀態。

```text theme={null}
/goal
```

如果目標活躍，狀態顯示：

* 條件
* 它已執行多長時間
* 已評估多少個回合
* 目前令牌支出
* 評估器最新的原因

回合計數和最新的原因會在第一次評估執行後出現。

如果沒有活躍的目標，但在工作階段中較早時已達成一個目標，狀態會顯示已達成的條件及其持續時間、回合計數和令牌支出。

<h3 id="clear-a-goal">
  清除目標
</h3>

執行 `/goal clear` 以在條件滿足前移除活躍的目標。

```text theme={null}
/goal clear
```

Claude 列印 `Goal cleared:` 後跟條件以確認，或如果沒有活躍的目標則列印 `No goal set`。

`stop`、`off`、`reset`、`none` 和 `cancel` 被接受為 `clear` 的別名。執行 `/clear` 以開始新對話也會移除任何活躍的目標。

<h3 id="resume-with-an-active-goal">
  使用活躍目標繼續
</h3>

當工作階段結束時仍然活躍的目標會在你使用 `--resume` 或 `--continue` 繼續該工作階段時恢復。條件會保留，但回合計數、計時器和令牌支出基線在繼續時都會重置。已達成或已清除的目標不會恢復。

<h3 id="run-non-interactively">
  非互動式執行
</h3>

`/goal` 在[非互動式模式](/docs/zh-TW/headless)、[桌面應用程式](/docs/zh-TW/desktop)中工作，並透過[遠端控制](/docs/zh-TW/remote-control)工作。使用 `-p` 設定目標會在單個呼叫中執行迴圈至完成：

```bash theme={null}
claude -p "/goal CHANGELOG.md has an entry for every PR merged this week"
```

使用預設文字輸出時，在條件滿足前不會列印任何內容，因此執行許多回合的目標可能看起來卡住了。新增 `--output-format stream-json --verbose` 以在迴圈執行時發出每個訊息。

使用 Ctrl+C 中斷程序以在條件滿足前停止非互動式目標。

<h2 id="how-evaluation-works">
  評估如何運作
</h2>

`/goal` 是工作階段範圍[基於提示的 Stop hook](/docs/zh-TW/hooks#prompt-based-hooks)的包裝器。每次 Claude 完成回合時，條件和迄今為止的對話都會發送到你配置的[小型快速模型](/docs/zh-TW/model-config)，預設為 Haiku。模型返回是或否決定和簡短原因。「否」告訴 Claude 繼續工作，並將原因作為下一個回合的指導。「是」清除目標並在文字記錄中記錄已達成的條目。

評估器在你的工作階段配置的任何提供者上執行。它不呼叫工具，因此只能判斷 Claude 已在對話中呈現的內容。

<Note>
  評估令牌在為你的提供者配置的小型快速模型上計費，與主要回合支出相比通常可以忽略不計。
</Note>

<h2 id="requirements">
  要求
</h2>

`/goal` 僅在你已接受信任對話框的工作區中執行，因為評估器是 hooks 系統的一部分。當在任何設定層級設定了 [`disableAllHooks`](/docs/zh-TW/hooks#disable-or-remove-hooks) 時，或當在受管設定中設定了 [`allowManagedHooksOnly`](/docs/zh-TW/settings#hook-configuration) 時，`/goal` 也不可用。在每種情況下，命令會告訴你原因，而不是默默地什麼都不做。

<h2 id="see-also">
  另請參閱
</h2>

* [使用 `/loop` 重複執行提示](/docs/zh-TW/scheduled-tasks#run-a-prompt-repeatedly-with-%2Floop)：按時間間隔重新執行，而不是直到條件滿足
* [基於提示的 hooks](/docs/zh-TW/hooks-guide#prompt-based-hooks)：當你需要自訂評估邏輯時編寫你自己的 Stop hook
* [自動模式](/docs/zh-TW/auto-mode-config)：自動批准工具呼叫，以便每個目標回合無人值守執行
* [排程比較](/docs/zh-TW/scheduled-tasks#compare-scheduling-options)：獨立於任何開啟工作階段在排程上執行工作
