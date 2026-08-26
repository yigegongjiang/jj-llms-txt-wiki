> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用 worktrees 執行平行會話

> 在獨立的 git worktrees 中隔離平行的 Claude Code 會話，使變更不會相互衝突。涵蓋 `--worktree` 旗標、子代理隔離、`.worktreeinclude`、清理和非 git VCS hooks。

[git worktree](https://git-scm.com/docs/git-worktree) 是一個獨立的工作目錄，具有自己的檔案和分支，但與主要檢出共享相同的儲存庫歷史記錄和遠端。在自己的 worktree 中執行每個 Claude Code 會話意味著一個會話中的編輯永遠不會觸及另一個會話中的檔案，因此您可以讓 Claude 在一個終端中建置功能，同時在第二個終端中修復錯誤。

本頁涵蓋 CLI 中的 worktree 隔離。下面的所有內容都假設使用 git 儲存庫。對於其他版本控制系統，請參閱[非 git 版本控制](#non-git-version-control)。[桌面應用程式](/docs/zh-TW/desktop#work-in-parallel-with-sessions)會自動為每個新會話建立一個 worktree。

Worktrees 是執行 Claude 平行處理的幾種方式之一。它們隔離檔案編輯，而[子代理](/docs/zh-TW/sub-agents)和[代理團隊](/docs/zh-TW/agent-teams)協調工作本身。請參閱[平行執行代理](/docs/zh-TW/agents)以比較這些方法，或跳到[使用 worktrees 隔離子代理](#isolate-subagents-with-worktrees)以同時使用 worktrees 和子代理。

<h2 id="start-claude-in-a-worktree">
  在 worktree 中啟動 Claude
</h2>

傳遞 `--worktree` 或 `-w` 以建立隔離的 worktree 並在其中啟動 Claude。預設情況下，worktree 在您的儲存庫根目錄下的 `.claude/worktrees/<value>/` 下建立，在名為 `worktree-<value>` 的新分支上：

```bash theme={null}
claude --worktree feature-auth
```

要將 worktrees 放在其他地方，請配置 [`WorktreeCreate` hook](#non-git-version-control)。在另一個終端中使用不同的名稱再次執行該命令以啟動第二個隔離的會話：

```bash theme={null}
claude --worktree bugfix-123
```

如果您省略名稱，Claude 會生成一個名稱，例如 `bright-running-fox`：

```bash theme={null}
claude --worktree
```

您也可以在會話期間要求 Claude「在 worktree 中工作」，它將使用 [`EnterWorktree`](/docs/zh-TW/tools-reference) 工具建立一個。進入 worktree 後，Claude 可以透過呼叫 `EnterWorktree` 並指定目標路徑，直接切換到 `.claude/worktrees/` 下的另一個 worktree。前一個 worktree 保持在磁碟上未被觸及。

進入儲存庫的 `.claude/worktrees/` 目錄外的路徑會先要求您的批准，因為它會移動會話的工作目錄、寫入存取權限和專案配置（例如 `CLAUDE.md` 和設定）到該位置。`EnterWorktree` [權限規則](/docs/zh-TW/permissions)或選擇「不再詢問」不會抑制此提示；只有 `bypassPermissions` 模式會跳過它。在 v2.1.206 之前，Claude 可以進入任何現有的 worktree 路徑而無需詢問。

自 v2.1.198 起，進入或退出 worktree 也會將會話記錄重新定位到該目錄的專案儲存空間，與 [`/cd`](/docs/zh-TW/commands) 的方式相同，因此 `/desktop` 和 `--resume` 之後會在該處找到會話。由 [`WorktreeCreate` hook](#non-git-version-control) 建立的 Worktrees 被排除在外，並將記錄保留在啟動目錄中。

Worktrees 在啟用[沙箱化](/docs/zh-TW/sandboxing#filesystem-isolation)的情況下工作：沙箱允許寫入主儲存庫的共享 `.git` 目錄，以便 `git commit` 等命令可以從連結的 worktree 內部更新參考和索引。

在第一次在目錄中使用 `--worktree` 之前，請透過在該目錄中執行一次 `claude` 來接受工作區信任對話。如果尚未接受信任，`--worktree` 將以錯誤退出並提示您先在目錄中執行 `claude`。非互動式執行搭配 `-p` 會跳過[信任檢查](/docs/zh-TW/security)，因此 `claude -p --worktree` 會在沒有信任檢查的情況下進行。

如果 Claude Code 在啟動時無法進入 worktree 目錄，例如因為 [`WorktreeCreate` hook](/docs/zh-TW/hooks#worktreecreate) 列印了建立的目錄以外的內容，或因為目錄在設定後被刪除，Claude Code 會列印一個錯誤，命名該路徑並以代碼 1 退出。在 v2.1.205 之前，這會導致會話崩潰，使用 `-p` 時會停滯約 30 秒，然後以代碼 0 退出。

在[專案範圍](/docs/zh-TW/plugins-reference#plugin-installation-scopes)從主要檢出安裝的外掛程式也會在同一儲存庫的 worktrees 中載入，因此您不需要為每個 worktree 重新安裝它們。無論您使用 `--worktree` 還是使用 `git worktree add` 建立 worktree，這都適用。需要 Claude Code v2.1.200 或更新版本。

<Tip>
  將 `.claude/worktrees/` 新增到您的 `.gitignore`，以便 worktree 內容不會在您的主要檢出中顯示為未追蹤的檔案。
</Tip>

<h3 id="choose-the-base-branch">
  選擇基礎分支
</h3>

Worktrees 從您的儲存庫的預設分支 `origin/HEAD` 分支，因此它們從與遠端相符的乾淨樹開始。當在過去 24 小時內沒有任何東西提取儲存庫時，Claude Code 會以預設分支的提取來重新整理 `origin/HEAD`，上限為五秒，如果提取失敗，則使用本地快取的參考。如果未配置遠端，或 `origin/HEAD` 未在本地快取且無法提取，worktree 會回退到您目前的本地 `HEAD`。

重新整理需要 Claude Code v2.1.208 或更新版本；在此之前，新的 worktree 使用已在本地快取的任何 `origin/HEAD`。

要始終從本地 `HEAD` 分支，請在[設定](/docs/zh-TW/settings#worktree-settings)中將 `worktree.baseRef` 設定為 `"head"`。將 `baseRef` 設定為 `"head"` 會使新 worktrees 帶有您未推送的提交和功能分支狀態，這在隔離需要在進行中的工作上操作的子代理時很有用。當會話在連結的 worktree 內執行時，`"head"` 解析為該 worktree 的 `HEAD`，而不是主要檢出的。該設定僅接受 `"fresh"` 或 `"head"`，不接受任意 git refs：

```json theme={null}
{
  "worktree": {
    "baseRef": "head"
  }
}
```

要從特定的拉取請求分支，請傳遞以 `#` 為前綴的 PR 編號或完整的 GitHub 拉取請求 URL。Claude Code 從 `origin` 提取 `pull/<number>/head` 並在 `.claude/worktrees/pr-<number>` 建立 worktree：

```bash theme={null}
claude --worktree "#1234"
```

為了完全控制 worktrees 的建立方式，請配置 [`WorktreeCreate` hook](/docs/zh-TW/hooks#worktreecreate)，它完全取代預設的 `git worktree` 邏輯。

<h3 id="reuse-a-worktree-name">
  重複使用 worktree 名稱
</h3>

重複使用其目錄已存在的 worktree 名稱會恢復該 worktree。

當以下所有條件都成立時，恢復的 worktree 會重設為[目前的基礎](#choose-the-base-branch)，而不是在其舊的頂端恢復：

* 它沒有未提交的變更或未追蹤的檔案。
* 它仍在 Claude Code 為其建立的分支上。
* 它從未提交，或其拉取請求已合併且其遠端分支已刪除。

在 v2.1.208 之前，重複使用的名稱總是在其舊的頂端恢復舊的 worktree。

<h2 id="copy-gitignored-files-into-worktrees">
  將 gitignored 檔案複製到 worktrees
</h2>

Worktree 是一個新的檢出，因此來自您主要儲存庫的未追蹤檔案（如 `.env` 或 `.env.local`）不存在。要在 Claude 建立 worktree 時自動複製它們，請將 `.worktreeinclude` 檔案新增到您的專案根目錄。

該檔案使用 `.gitignore` 語法。只有符合模式且也被 gitignored 的檔案才會被複製，因此追蹤的檔案永遠不會被重複。

此 `.worktreeinclude` 將兩個環境檔案和一個秘密配置複製到每個新 worktree：

```text .worktreeinclude theme={null}
.env
.env.local
config/secrets.json
```

這適用於使用 `--worktree` 建立的 worktrees、[子代理 worktrees](#isolate-subagents-with-worktrees) 和[桌面應用程式](/docs/zh-TW/desktop#work-in-parallel-with-sessions)中的平行會話。

<h2 id="isolate-subagents-with-worktrees">
  使用 worktrees 隔離子代理
</h2>

子代理可以在自己的 worktrees 中執行，以便平行編輯不會衝突。要求 Claude「為您的代理使用 worktrees」，或通過將 `isolation: worktree` 新增到 frontmatter 在[自訂子代理](/docs/zh-TW/sub-agents#supported-frontmatter-fields)上永久設定它。每個子代理都會獲得一個臨時 worktree，當子代理完成而沒有變更時會自動移除。

子代理 worktrees 使用與 `--worktree` 相同的[基礎分支](#choose-the-base-branch)，因此它們從您的儲存庫的預設分支分支，除非 `worktree.baseRef` 設定為 `"head"`。

<h2 id="clean-up-worktrees">
  清理 worktrees
</h2>

當您退出 worktree 會話時，清理取決於您是否進行了變更：

* **無未提交的變更、無未追蹤的檔案且無新提交**：worktree 及其分支會自動移除。如果會話有[名稱](/docs/zh-TW/sessions#name-your-sessions)，Claude 會改為提示您，以便您可以稍後保留 worktree
* **存在未提交的變更、未追蹤的檔案或新提交**：Claude 會提示您保留或移除 worktree。保留會保留目錄和分支，以便您稍後可以返回。移除會刪除 worktree 目錄及其分支，丟棄所有未提交的變更、未追蹤的檔案和提交
* **非互動式執行**：使用 `--worktree` 與 `-p` 一起建立的 worktrees 不會自動清理，因為沒有退出提示。使用 `git worktree remove` 移除它們

Claude 為子代理和[背景會話](/docs/zh-TW/agent-view#how-file-edits-are-isolated)建立的 Worktrees 一旦超過您的 [`cleanupPeriodDays`](/docs/zh-TW/settings#available-settings) 設定，就會自動移除，前提是它們沒有未提交的變更、沒有未追蹤的檔案和沒有未推送的提交。您使用 `--worktree` 建立的 Worktrees 永遠不會被此掃描移除。

當代理正在執行時，Claude 會在其 worktree 上執行 `git worktree lock`，以便並行清理無法將其移除。當代理完成時，鎖定會被釋放。若要清理掃描保留的 worktree，請執行 `git worktree remove`，如果 worktree 有未提交的變更或未追蹤的檔案，請新增 `--force`。

在 Windows 上，移除 worktree 之前，Claude Code 會移除其內任何深度的任何 NTFS 連接點或目錄符號連結作為連結項目，以便移除 worktree 不會刪除連結指向的檔案。在 v2.1.205 之前，Claude Code 只移除頂層連結作為連結項目，移除具有嵌套在子目錄中的連接點的 worktree 可能會刪除 worktree 外連結指向的目錄內容。

<h2 id="manage-worktrees-manually">
  手動管理 worktrees
</h2>

為了完全控制 worktree 位置和分支配置，請直接使用 Git 建立 worktrees。當您需要檢出特定的現有分支或將 worktree 放在儲存庫外時，這很有用。

在新分支上建立 worktree：

```bash theme={null}
git worktree add ../project-feature-a -b feature-a
```

從現有分支建立 worktree：

```bash theme={null}
git worktree add ../project-bugfix bugfix-123
```

在 worktree 中啟動 Claude：

```bash theme={null}
cd ../project-feature-a && claude
```

列出您的 worktrees：

```bash theme={null}
git worktree list
```

完成後移除一個：

```bash theme={null}
git worktree remove ../project-feature-a
```

有關完整的命令參考，請參閱 [Git worktree 文件](https://git-scm.com/docs/git-worktree)。記住在每個新 worktree 中初始化您的開發環境：安裝依賴項、設定虛擬環境或執行您的專案設定所需的任何操作。

<h2 id="non-git-version-control">
  非 git 版本控制
</h2>

Worktree 隔離預設使用 git。對於 SVN、Perforce、Mercurial 或其他系統，請配置 [`WorktreeCreate` 和 `WorktreeRemove` hooks](/docs/zh-TW/hooks#worktreecreate) 以提供自訂建立和清理邏輯。因為 hook 取代了預設的 git 行為，當您使用 `--worktree` 時，[`.worktreeinclude`](#copy-gitignored-files-into-worktrees) 不會被處理。改為在您的 hook 指令碼內複製任何本地配置檔案。

此 `WorktreeCreate` hook 從 stdin 讀取 worktree 名稱，檢出新的 SVN 工作副本，並列印目錄路徑，以便 Claude Code 可以將其用作會話的工作目錄：

```json theme={null}
{
  "hooks": {
    "WorktreeCreate": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "bash -c 'NAME=$(jq -r .name); DIR=\"$HOME/.claude/worktrees/$NAME\"; svn checkout https://svn.example.com/repo/trunk \"$DIR\" >&2 && echo \"$DIR\"'"
          }
        ]
      }
    ]
  }
}
```

將其與 `WorktreeRemove` hook 配對以在會話結束時進行清理。有關輸入架構和移除範例，請參閱 [hooks 參考](/docs/zh-TW/hooks#worktreecreate)。

<h2 id="see-also">
  另請參閱
</h2>

Worktrees 處理檔案隔離。下面的相關頁面涵蓋將工作委派到這些隔離的檢出中以及在您建立的會話之間切換：

* [子代理](/docs/zh-TW/sub-agents)：在會話內將工作委派給隔離的代理
* [代理團隊](/docs/zh-TW/agent-teams)：自動協調多個 Claude 會話
* [管理會話](/docs/zh-TW/sessions)：命名、恢復和在對話之間切換
* [桌面平行會話](/docs/zh-TW/desktop#work-in-parallel-with-sessions)：桌面應用程式中由 worktree 支援的會話
