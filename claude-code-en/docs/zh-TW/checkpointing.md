> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Checkpointing

> 追蹤、回溯和總結 Claude 的編輯和對話以管理會話狀態。

Claude Code 會自動追蹤 Claude 在您工作時所做的檔案編輯，讓您可以快速撤銷變更並回溯到先前的狀態，以防任何事情出現偏差。

<h2 id="how-checkpoints-work">
  Checkpointing 的運作方式
</h2>

當您與 Claude 合作時，checkpointing 會自動捕捉每次使用者提示前的程式碼狀態。這個安全網讓您可以進行雄心勃勃的大規模任務，同時知道您可以隨時回到先前的程式碼狀態。

<h3 id="automatic-tracking">
  自動追蹤
</h3>

Claude Code 追蹤由其檔案編輯工具所做的所有變更：

* 每個使用者提示都會建立一個新的 checkpoint
* Claude Code 在一個會話中保留最近 100 個 checkpoint 的檔案快照。捨棄較舊的 checkpoint 會刪除沒有其他 checkpoint 參考的快照檔案，除了每個檔案的第一個快照，VS Code 擴充功能將其用作會話差異的基準。在 v2.1.208 之前，這些被取代的快照檔案會保留在磁碟上，直到會話被清理。
* Checkpoints 與對話一起儲存，因此恢復的會話仍然可以 `/rewind` 回到它們
* 自動清理，與會話一起在 30 天後刪除（可配置）

<h3 id="rewind-and-summarize">
  回溯和總結
</h3>

執行 `/rewind`，或在提示輸入為空時按兩次 `Esc`，以開啟回溯選單。

<Note>
  如果提示輸入包含文字，雙 `Esc` 會清除它而不是開啟選單。清除的文字會儲存到您的輸入歷史記錄中，因此在您完成回溯選單後，按 `Up` 可以召回它。
</Note>

回溯選單列出您在會話期間傳送的每個提示。選擇您想要操作的點，然後選擇一個動作：

* **恢復程式碼和對話**：將程式碼和對話都回復到該點
* **恢復對話**：回溯到該訊息，同時保持目前程式碼
* **恢復程式碼**：回復檔案變更，同時保持對話
* **從此處總結**：將此點之後的對話壓縮為摘要，釋放 context window 空間
* **從此處之前總結**：將此點之前的對話壓縮為摘要，保持後續訊息完整
* **算了**：返回訊息清單而不進行任何變更

恢復對話或選擇「從此處總結」後，所選訊息的原始提示會恢復到輸入欄位中，以便您可以重新傳送或編輯它。

選擇「從此處之前總結」會讓您留在對話末尾，輸入為空。

<h4 id="rewind-past-a-cleared-conversation">
  回溯過去已清除的對話
</h4>

如果您在同一個 Claude Code 程序中較早執行了 `/clear`，回溯選單會在清單頂部顯示一個額外的項目，標記為 `/resume <session-id> (previous session)`。選擇它以恢復在 `/clear` 執行前活躍的對話。該項目在您退出 Claude Code 或恢復不同會話之前可用，並且需要 Claude Code v2.1.191 或更新版本。在較早的版本上，執行 `/resume` 並從清單中選擇前一個會話。

<h4 id="restore-vs-summarize">
  恢復與總結
</h4>

恢復選項會回復狀態：它們撤銷程式碼變更、對話歷史或兩者。總結選項會將對話的一部分壓縮為 AI 生成的摘要，而不改變磁碟上的檔案：

* **從此處總結**：所選訊息之前的訊息保持完整。所選訊息及其後的所有訊息都被替換為摘要。使用此選項可以捨棄旁支討論，同時保持早期上下文的完整詳細資訊。
* **從此處之前總結**：所選訊息之前的訊息被替換為摘要。所選訊息及其後的所有訊息保持完整，您保持在對話末尾。使用此選項可以壓縮早期設定討論，同時保持最近工作的完整詳細資訊。

在這兩種情況下，原始訊息都保存在會話記錄中，因此 Claude 可以在需要時參考詳細資訊。您可以輸入可選指示來引導摘要的重點。這類似於 `/compact`，但更有針對性：您不是總結整個對話，而是選擇所選訊息的哪一側要壓縮。

<Note>
  總結讓您保持在同一會話中並壓縮上下文。如果您想嘗試不同的方法，同時保持原始會話完整，請改用 [fork](/docs/zh-TW/sessions#branch-a-session)（`claude --continue --fork-session`）。
</Note>

<h2 id="common-use-cases">
  常見使用案例
</h2>

Checkpoints 在以下情況下特別有用：

* **探索替代方案**：嘗試不同的實現方法，而不會失去起點
* **從錯誤中恢復**：快速撤銷引入錯誤或破壞功能的變更
* **迭代功能**：進行變化實驗，同時知道您可以回復到工作狀態
* **釋放上下文空間**：從中點開始總結冗長的除錯會話，保持初始指示完整

<h2 id="limitations">
  限制
</h2>

<h3 id="bash-command-changes-not-tracked">
  Bash 命令變更未追蹤
</h3>

Checkpointing 不追蹤由 bash 命令修改的檔案。例如，如果 Claude Code 執行：

```bash theme={null}
rm file.txt
mv old.txt new.txt
cp source.txt dest.txt
```

這些檔案修改無法透過回溯撤銷。只有透過 Claude 的檔案編輯工具進行的直接檔案編輯才會被追蹤。

<h3 id="external-changes-not-tracked">
  外部變更未追蹤
</h3>

Checkpointing 只追蹤在目前會話中已編輯的檔案。您在 Claude Code 外部對檔案所做的手動變更以及來自其他並行會話的編輯通常不會被捕捉，除非它們碰巧修改與目前會話相同的檔案。

<h3 id="not-a-replacement-for-version-control">
  不是版本控制的替代品
</h3>

Checkpoints 設計用於快速的會話級恢復。對於永久版本歷史和協作：

* 繼續使用版本控制（例如 Git）進行提交、分支和長期歷史
* Checkpoints 補充但不替代適當的版本控制
* 將 checkpoints 視為「本地撤銷」，Git 視為「永久歷史」

<h2 id="see-also">
  另請參閱
</h2>

* [Interactive mode](/docs/zh-TW/interactive-mode) - 快捷鍵和會話控制
* [Commands](/docs/zh-TW/commands) - 使用 `/rewind` 存取 checkpoints
* [CLI reference](/docs/zh-TW/cli-reference) - 命令列選項
