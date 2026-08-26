> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code 最佳實踐

> 從配置環境到跨平行會話擴展，充分利用 Claude Code 的提示和模式。

Claude Code 是一個代理式編碼環境。與等待回答問題的聊天機器人不同，Claude Code 可以讀取您的文件、運行命令、進行更改，並在您觀看、重定向或完全離開時自主解決問題。

這改變了您的工作方式。與其自己編寫代碼並要求 Claude 審查，不如描述您想要的內容，讓 Claude 找出如何構建它。Claude 會探索、規劃和實施。

但這種自主性仍然伴隨著學習曲線。Claude 在您需要理解的某些約束條件下工作。

本指南涵蓋了在 Anthropic 內部團隊和在各種代碼庫、語言和環境中使用 Claude Code 的工程師中已被證明有效的模式。有關代理循環如何在幕後工作的信息，請參閱 [Claude Code 如何工作](/docs/zh-TW/how-claude-code-works)。

***

大多數最佳實踐都基於一個約束：Claude 的 context window 填滿得很快，隨著填滿，性能會下降。

Claude 的 context window 保存您的整個對話，包括每條消息、Claude 讀取的每個文件和每個命令輸出。但是，這可能會很快填滿。單個調試會話或代碼庫探索可能會生成並消耗數萬個令牌。

這很重要，因為隨著 context 填滿，LLM 性能會下降。當 context window 即將滿時，Claude 可能會開始「遺忘」早期的指令或犯更多錯誤。context window 是最重要的資源來管理。要查看會話在實踐中如何填滿，請 [觀看互動式演練](/docs/zh-TW/context-window)，了解啟動時加載的內容以及每次文件讀取的成本。使用 [自定義狀態行](/docs/zh-TW/statusline) 持續跟蹤 context 使用情況，並查看 [減少令牌使用](/docs/zh-TW/costs#reduce-token-usage) 以了解減少令牌使用的策略。

***

<h2 id="give-claude-a-way-to-verify-its-work">
  給 Claude 一種驗證其工作的方式
</h2>

<Tip>
  給 Claude 一個可以運行的檢查：測試、構建、截圖比較。這是您可以觀看的會話和可以離開的會話之間的區別。
</Tip>

Claude 在工作看起來完成時停止。沒有可以運行的檢查，「看起來完成」是唯一可用的信號，您成為驗證循環：每個錯誤都在等待您注意到它。給 Claude 一些能產生通過或失敗的東西，循環就會自動關閉。Claude 完成工作、運行檢查、讀取結果，並迭代直到檢查通過。

檢查是任何在對話中返回 Claude 可以讀取的信號的東西：測試套件、構建退出代碼、linter、針對固定裝置比較輸出的腳本，或與設計進行比較的[瀏覽器截圖](/docs/zh-TW/chrome)。

| 策略                | 之前                  | 之後                                                                                                                                  |
| ----------------- | ------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| **提供驗證標準**        | *「實現一個驗證電子郵件地址的函數」* | *「編寫一個 validateEmail 函數。示例測試用例：[user@example.com](mailto:user@example.com) 為真，invalid 為假，[user@.com](mailto:user@.com) 為假。實施後運行測試」* |
| **以視覺方式驗證 UI 更改** | *「使儀表板看起來更好」*       | *「\[粘貼截圖] 實施此設計。對結果進行截圖並與原始設計進行比較。列出差異並修復它們」*                                                                                       |
| **解決根本原因，而不是症狀**  | *「構建失敗」*            | *「構建失敗，出現此錯誤：\[粘貼錯誤]。修復它並驗證構建成功。解決根本原因，不要抑制錯誤」*                                                                                     |

檢查存在後，決定它對停止的限制有多嚴格：

* **在一個提示中**：要求 Claude 運行檢查並在同一消息中迭代，如上表所示。
* **在整個會話中**：將檢查設置為 [`/goal` 條件](/docs/zh-TW/goal)。單獨的評估器在每次轉換後重新檢查它，Claude 繼續工作直到它成立。
* **作為確定性門**：[Stop hook](/docs/zh-TW/hooks#stop) 將您的檢查作為腳本運行，並阻止轉換結束直到它通過。Claude Code 覆蓋該 hook 並在 8 次連續阻止後結束轉換。
* **由第二意見**：[驗證子代理](/docs/zh-TW/sub-agents)或[動態工作流](/docs/zh-TW/workflows)檢查自己的發現，有一個新鮮的模型嘗試反駁結果，所以做工作的代理不是給它評分的那個。

每一步都用設置換取關注。提示版本適用於今天的任何任務。`/goal` 和 Stop hook 版本是讓無人值守運行正確完成而無需您的版本。

讓 Claude 展示證據而不是聲稱成功：測試輸出、它運行的命令及其返回的內容，或結果的截圖。審查證據比自己重新運行驗證要快得多，並且它適用於您沒有觀看的會話。

***

<h2 id="explore-first-then-plan-then-code">
  先探索，然後規劃，然後編碼
</h2>

<Tip>
  將研究和規劃與實施分開，以避免解決錯誤的問題。
</Tip>

讓 Claude 直接跳到編碼可能會產生解決錯誤問題的代碼。使用 [Plan Mode](/docs/zh-TW/permission-modes#analyze-before-you-edit-with-plan-mode) 將探索與執行分開。

推薦的工作流程有四個階段：

<Steps>
  <Step title="探索">
    進入 Plan Mode。Claude 讀取文件並回答問題，不進行任何更改。

    ```txt claude (plan mode) theme={null}
    read /src/auth and understand how we handle sessions and login.
    also look at how we manage environment variables for secrets.
    ```
  </Step>

  <Step title="規劃">
    要求 Claude 創建詳細的實施計劃。

    ```txt claude (plan mode) theme={null}
    I want to add Google OAuth. What files need to change?
    What's the session flow? Create a plan.
    ```

    按 `Ctrl+G` 在文本編輯器中打開計劃進行直接編輯，然後 Claude 再繼續。
  </Step>

  <Step title="實施">
    切換出 Plan Mode，讓 Claude 編碼，根據其計劃進行驗證。

    ```txt claude (default mode) theme={null}
    implement the OAuth flow from your plan. write tests for the
    callback handler, run the test suite and fix any failures.
    ```
  </Step>

  <Step title="提交">
    要求 Claude 使用描述性消息進行提交並創建 PR。

    ```txt claude (default mode) theme={null}
    commit with a descriptive message and open a PR
    ```
  </Step>
</Steps>

<Callout>
  Plan Mode 很有用，但也增加了開銷。

  對於範圍明確且修復很小的任務（如修復拼寫錯誤、添加日誌行或重命名變量），直接要求 Claude 執行。

  當您對方法不確定、更改修改多個文件或您不熟悉被修改的代碼時，規劃最有用。如果您可以用一句話描述 diff，請跳過計劃。
</Callout>

***

<h2 id="provide-specific-context-in-your-prompts">
  在提示中提供具體的上下文
</h2>

<Tip>
  您的指令越精確，您需要的更正就越少。
</Tip>

Claude 可以推斷意圖，但無法讀心術。參考特定文件、提及約束條件並指出示例模式。

| 策略                              | 之前                                   | 之後                                                                                                                  |
| ------------------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------- |
| **限定任務範圍。** 指定哪個文件、什麼場景和測試偏好。   | *「為 foo.py 添加測試」*                    | *「為 foo.py 編寫測試，涵蓋用戶已登出的邊界情況。避免使用 mocks。」*                                                                          |
| **指向來源。** 指導 Claude 到可以回答問題的來源。 | *「為什麼 ExecutionFactory 有這樣奇怪的 api？」* | *「查看 ExecutionFactory 的 git 歷史記錄並總結其 api 是如何形成的」*                                                                   |
| **參考現有模式。** 指向代碼庫中的模式。          | *「添加日曆小部件」*                          | *「查看主頁上現有小部件的實施方式以了解模式。HotDogWidget.php 是一個很好的例子。按照模式實施一個新的日曆小部件，讓用戶選擇月份並向前/向後分頁以選擇年份。從頭開始構建，除了代碼庫中已使用的庫外，不使用其他庫。」* |
| **描述症狀。** 提供症狀、可能的位置以及「修復」的樣子。  | *「修復登錄錯誤」*                           | *「用戶報告會話超時後登錄失敗。檢查 src/auth/ 中的身份驗證流程，特別是令牌刷新。編寫一個失敗的測試來重現問題，然後修復它」*                                                |

當您在探索並可以承受改正時，模糊的提示可能很有用。像 `「您會改進此文件中的什麼？」` 這樣的提示可以表面您沒有想到要詢問的內容。

<h3 id="provide-rich-content">
  提供豐富的內容
</h3>

<Tip>
  使用 `@` 參考文件、粘貼截圖/圖像或直接管道數據。
</Tip>

您可以通過多種方式向 Claude 提供豐富的數據：

* **使用 `@` 參考文件**，而不是描述代碼的位置。Claude 在回應前讀取文件。
* **直接粘貼圖像**。複製/粘貼或將圖像拖放到提示中。
* **提供 URL** 用於文檔和 API 參考。使用 `/permissions` 將常用域名列入白名單。
* **通過運行 `cat error.log | claude` 管道數據**，直接發送文件內容。
* **讓 Claude 獲取它需要的內容**。告訴 Claude 使用 Bash 命令、MCP 工具或通過讀取文件自己拉取上下文。

***

<h2 id="configure-your-environment">
  配置您的環境
</h2>

一些設置步驟使 Claude Code 在所有會話中的效果顯著提高。有關擴展功能的完整概述和何時使用每個功能，請參閱 [擴展 Claude Code](/docs/zh-TW/features-overview)。

<h3 id="write-an-effective-claude-md">
  編寫有效的 CLAUDE.md
</h3>

<Tip>
  運行 `/init` 根據您當前的項目結構生成一個啟動 CLAUDE.md 文件，然後隨著時間推移進行改進。
</Tip>

CLAUDE.md 是一個特殊文件，Claude 在每次對話開始時都會讀取。包括 Bash 命令、代碼風格和工作流規則。這給 Claude 提供了它無法從代碼中推斷出的持久上下文。

`/init` 命令分析您的代碼庫以檢測構建系統、測試框架和代碼模式，為您提供堅實的基礎進行改進。

CLAUDE.md 文件沒有必需的格式，但要保持簡短和易於閱讀。例如：

```markdown CLAUDE.md theme={null}
# Code style
- Use ES modules (import/export) syntax, not CommonJS (require)
- Destructure imports when possible (eg. import { foo } from 'bar')

# Workflow
- Be sure to typecheck when you're done making a series of code changes
- Prefer running single tests, and not the whole test suite, for performance
```

CLAUDE.md 在每個會話中加載，因此只包括廣泛適用的內容。對於僅在某些時候相關的域知識或工作流，請改用 [skills](/docs/zh-TW/skills)。Claude 按需加載它們，不會使每次對話都變得臃腫。

保持簡潔。對於每一行，問自己：*「刪除這一行會導致 Claude 犯錯誤嗎？」* 如果不會，刪除它。臃腫的 CLAUDE.md 文件會導致 Claude 忽略您的實際指令！

| ✅ 包括                 | ❌ 排除                   |
| -------------------- | ---------------------- |
| Claude 無法猜測的 Bash 命令 | Claude 可以通過讀取代碼找出的任何內容 |
| 與默認值不同的代碼風格規則        | Claude 已經知道的標準語言約定     |
| 測試指令和首選測試運行器         | 詳細的 API 文檔（改為鏈接到文檔）    |
| 存儲庫禮儀（分支命名、PR 約定）    | 經常變化的信息                |
| 特定於您項目的架構決策          | 長篇解釋或教程                |
| 開發人員環境怪癖（必需的環境變量）    | 文件逐個描述代碼庫              |
| 常見陷阱或非顯而易見的行為        | 自明的實踐，如「編寫乾淨代碼」        |

如果 Claude 儘管有反對規則仍然不斷做您不想要的事情，該文件可能太長，規則被遺漏了。如果 Claude 詢問您在 CLAUDE.md 中回答的問題，措辭可能不明確。像對待代碼一樣對待 CLAUDE.md：當事情出錯時進行審查，定期修剪，並通過觀察 Claude 的行為是否實際改變來測試更改。

您可以通過添加強調（例如「IMPORTANT」或「YOU MUST」）來調整指令以改進遵守。將文件簽入 git，以便您的團隊可以貢獻。該文件的價值隨著時間的推移而複合。

CLAUDE.md 文件可以使用 `@path/to/import` 語法導入其他文件：

```markdown CLAUDE.md theme={null}
See @README.md for project overview and @package.json for available npm commands.

# Additional Instructions
- Git workflow: @docs/git-instructions.md
- Personal overrides: @~/.claude/my-project-instructions.md
```

您可以將 CLAUDE.md 文件放在多個位置：

* **主文件夾（`~/.claude/CLAUDE.md`）**：適用於所有 Claude 會話
* **項目根目錄（`./CLAUDE.md`）**：簽入 git 以與您的團隊共享
* **項目根目錄（`./CLAUDE.local.md`）**：個人項目特定的筆記；將此文件添加到您的 `.gitignore`，以便不與您的團隊共享
* **父目錄**：對於 monorepos 很有用，其中 `root/CLAUDE.md` 和 `root/foo/CLAUDE.md` 都會自動拉入
* **子目錄**：當在這些目錄中的文件上工作時，Claude 按需拉入子 CLAUDE.md 文件

<h3 id="configure-permissions">
  配置權限
</h3>

<Tip>
  使用 [auto mode](/docs/zh-TW/permission-modes#eliminate-prompts-with-auto-mode) 讓分類器處理批准，使用 `/permissions` 將特定命令列入白名單，或使用 `/sandbox` 進行操作系統級隔離。每種方式都減少了中斷，同時讓您保持控制。
</Tip>

默認情況下，Claude Code 請求可能修改您的系統的操作的權限：文件寫入、Bash 命令、MCP 工具等。這是安全的但很繁瑣。在第十次批准後，您實際上不是在審查，而是在點擊。有三種方法可以減少這些中斷：

* **Auto mode**：一個單獨的分類器模型審查命令並僅阻止看起來有風險的內容：範圍升級、未知基礎設施或由敵對內容驅動的操作。最適合當您信任任務的總體方向但不想點擊每一步時
* **權限白名單**：允許您知道安全的特定工具，如 `npm run lint` 或 `git commit`
* **沙箱**：啟用操作系統級隔離，限制文件系統和網絡訪問，允許 Claude 在定義的邊界內更自由地工作

閱讀更多關於 [permission modes](/docs/zh-TW/permission-modes)、[permission rules](/docs/zh-TW/permissions) 和 [sandboxing](/docs/zh-TW/sandboxing)。

<h3 id="use-cli-tools">
  使用 CLI 工具
</h3>

<Tip>
  告訴 Claude Code 在與外部服務交互時使用 CLI 工具，如 `gh`、`aws`、`gcloud` 和 `sentry-cli`。
</Tip>

CLI 工具是與外部服務交互的最 context 高效的方式。如果您使用 GitHub，請安裝 `gh` CLI。Claude 知道如何使用它來創建問題、打開拉取請求和讀取評論。沒有 `gh`，Claude 仍然可以使用 GitHub API，但未經身份驗證的請求經常會達到速率限制。

Claude 也很擅長學習它不知道的 CLI 工具。嘗試像 `Use 'foo-cli-tool --help' to learn about foo tool, then use it to solve A, B, C.` 這樣的提示。

<h3 id="connect-mcp-servers">
  連接 MCP servers
</h3>

<Tip>
  運行 `claude mcp add` 以連接外部工具，如 Notion、Figma 或您的數據庫。
</Tip>

使用 [MCP servers](/docs/zh-TW/mcp)，您可以要求 Claude 從問題跟蹤器實施功能、查詢數據庫、分析監控數據、集成來自 Figma 的設計並自動化工作流。

<h3 id="set-up-hooks">
  設置 hooks
</h3>

<Tip>
  使用 hooks 進行必須每次發生且沒有例外的操作。
</Tip>

[Hooks](/docs/zh-TW/hooks-guide) 在 Claude 工作流中的特定點自動運行腳本。與建議性的 CLAUDE.md 指令不同，hooks 是確定性的，保證操作發生。

Claude 可以為您編寫 hooks。嘗試像 *「編寫一個在每次文件編輯後運行 eslint 的 hook」* 或 *「編寫一個阻止寫入遷移文件夾的 hook。」* 這樣的提示。編輯 `.claude/settings.json` 直接配置 hooks，並運行 `/hooks` 瀏覽已配置的內容。

<h3 id="create-skills">
  創建 skills
</h3>

<Tip>
  在 `.claude/skills/` 中創建 `SKILL.md` 文件，為 Claude 提供域知識和可重用工作流。
</Tip>

[Skills](/docs/zh-TW/skills) 使用特定於您的項目、團隊或域的信息擴展 Claude 的知識。Claude 在相關時自動應用它們，或者您可以使用 `/skill-name` 直接調用它們。

通過將目錄與 `SKILL.md` 添加到 `.claude/skills/` 來創建 skill：

```markdown .claude/skills/api-conventions/SKILL.md theme={null}
---
name: api-conventions
description: REST API design conventions for our services
---
# API Conventions
- Use kebab-case for URL paths
- Use camelCase for JSON properties
- Always include pagination for list endpoints
- Version APIs in the URL path (/v1/, /v2/)
```

Skills 也可以定義您直接調用的可重複工作流：

```markdown .claude/skills/fix-issue/SKILL.md theme={null}
---
name: fix-issue
description: Fix a GitHub issue
disable-model-invocation: true
---
Analyze and fix the GitHub issue: $ARGUMENTS.

1. Use `gh issue view` to get the issue details
2. Understand the problem described in the issue
3. Search the codebase for relevant files
4. Implement the necessary changes to fix the issue
5. Write and run tests to verify the fix
6. Ensure code passes linting and type checking
7. Create a descriptive commit message
8. Push and create a PR
```

運行 `/fix-issue 1234` 來調用它。對於具有您想手動觸發的副作用的工作流，使用 `disable-model-invocation: true`。

<h3 id="create-custom-subagents">
  創建自定義 subagents
</h3>

<Tip>
  在 `.claude/agents/` 中定義專門的助手，Claude 可以委派給它們進行隔離的任務。
</Tip>

[Subagents](/docs/zh-TW/sub-agents) 在自己的 context 中運行，具有自己的一組允許的工具。它們對於讀取許多文件或需要專門關注而不會使主對話變得混亂的任務很有用。

```markdown .claude/agents/security-reviewer.md theme={null}
---
name: security-reviewer
description: Reviews code for security vulnerabilities
tools: Read, Grep, Glob, Bash
model: opus
---
You are a senior security engineer. Review code for:
- Injection vulnerabilities (SQL, XSS, command injection)
- Authentication and authorization flaws
- Secrets or credentials in code
- Insecure data handling

Provide specific line references and suggested fixes.
```

明確告訴 Claude 使用 subagents：*「使用 subagent 審查此代碼以查找安全問題。」*

<h3 id="install-plugins">
  安裝 plugins
</h3>

<Tip>
  運行 `/plugin` 瀏覽市場。Plugins 無需配置即可添加 skills、工具和集成。
</Tip>

[Plugins](/docs/zh-TW/plugins) 將 skills、hooks、subagents 和 MCP servers 捆綁到來自社區和 Anthropic 的單個可安裝單元中。如果您使用類型化語言，請安裝 [代碼智能 plugin](/docs/zh-TW/discover-plugins#code-intelligence) 以為 Claude 提供精確的符號導航和編輯後的自動錯誤檢測。

有關在 skills、subagents、hooks 和 MCP 之間選擇的指導，請參閱 [擴展 Claude Code](/docs/zh-TW/features-overview#match-features-to-your-goal)。

***

<h2 id="communicate-effectively">
  有效溝通
</h2>

您與 Claude Code 溝通的方式會顯著影響結果的質量。

<h3 id="ask-codebase-questions">
  詢問代碼庫問題
</h3>

<Tip>
  詢問 Claude 您會問資深工程師的問題。
</Tip>

當加入新代碼庫時，使用 Claude Code 進行學習和探索。您可以詢問 Claude 與詢問另一位工程師相同類型的問題：

* 日誌記錄如何工作？
* 我如何創建新的 API 端點？
* `foo.rs` 第 134 行的 `async move { ... }` 做什麼？
* `CustomerOnboardingFlowImpl` 處理哪些邊界情況？
* 為什麼此代碼在第 333 行調用 `foo()` 而不是 `bar()`？

以這種方式使用 Claude Code 是一個有效的入職工作流程，改進了入職時間並減少了對其他工程師的負擔。無需特殊提示：直接提出問題。

<h3 id="let-claude-interview-you">
  讓 Claude 採訪您
</h3>

<Tip>
  對於較大的功能，讓 Claude 先採訪您。從最小的提示開始，並要求 Claude 使用 `AskUserQuestion` 工具採訪您。
</Tip>

Claude 會詢問您可能還沒有考慮的事情，包括技術實施、UI/UX、邊界情況和權衡。

```text theme={null}
I want to build [brief description]. Interview me in detail using the AskUserQuestion tool.

Ask about technical implementation, UI/UX, edge cases, concerns, and tradeoffs. Don't ask obvious questions, dig into the hard parts I might not have considered.

Keep interviewing until we've covered everything, then write a complete spec to SPEC.md.
```

規格完成後，開始新會話以執行它。新會話具有完全專注於實施的乾淨 context，您有一個書面規格可供參考。

最有用的規格是自包含的：它們命名涉及的文件和介面、說明什麼超出範圍，並以端到端驗證步驟結束，證明該功能有效。花費時間使規格精確的回報遠大於花費時間觀看實施的回報。

***

<h2 id="manage-your-session">
  管理您的會話
</h2>

對話是持久的和可逆的。利用這一點！

<h3 id="course-correct-early-and-often">
  及早且經常改正方向
</h3>

<Tip>
  一旦您注意到 Claude 偏離軌道，立即改正。
</Tip>

最好的結果來自緊密的反饋循環。儘管 Claude 有時會在第一次嘗試時完美地解決問題，但快速改正通常會更快地產生更好的解決方案。

* **`Esc`**：使用 `Esc` 鍵在中途停止 Claude。Context 被保留，所以您可以重定向。
* **`Esc + Esc` 或 `/rewind`**：按 `Esc` 兩次或運行 `/rewind` 打開倒帶菜單並恢復之前的對話和代碼狀態，或從選定的消息進行總結。
* **`「撤銷那個」`**：讓 Claude 恢復其更改。
* **`/clear`**：在不相關的任務之間重置 context。具有不相關 context 的長會話可能會降低性能。

如果您在一個會話中對同一問題改正了 Claude 超過兩次，context 就會被失敗的方法所污染。運行 `/clear` 並使用更具體的提示重新開始，該提示包含您學到的內容。具有更好提示的乾淨會話幾乎總是優於具有累積改正的長會話。

<h3 id="manage-context-aggressively">
  積極管理 context
</h3>

<Tip>
  在不相關的任務之間運行 `/clear` 以重置 context。
</Tip>

當您接近 context 限制時，Claude Code 會自動壓縮對話歷史記錄，這保留了重要的代碼和決策，同時釋放空間。

在長會話期間，Claude 的 context window 可能會充滿不相關的對話、文件內容和命令。這可能會降低性能，有時會分散 Claude 的注意力。

* 在任務之間頻繁使用 `/clear` 以完全重置 context window
* 當自動壓縮觸發時，Claude 總結最重要的內容，包括代碼模式、文件狀態和關鍵決策
* 為了更好地控制，運行 `/compact <instructions>`，如 `/compact Focus on the API changes`
* 要僅壓縮對話的一部分，使用 `Esc + Esc` 或 `/rewind`，選擇消息檢查點，然後選擇 **從此處進行總結** 或 **總結到此處**。第一個會壓縮該點之後的消息，同時保持早期 context 完整；第二個會壓縮早期消息，同時保持最近的消息完整。請參閱 [恢復與總結](/docs/zh-TW/checkpointing#restore-vs-summarize)。
* 在 CLAUDE.md 中使用像 `「壓縮時，始終保留完整的修改文件列表和任何測試命令」` 這樣的指令自定義壓縮行為，以確保關鍵 context 在總結中存活
* 對於不需要留在 context 中的快速問題，使用 [`/btw`](/docs/zh-TW/interactive-mode#side-questions-with-%2Fbtw)。答案出現在可關閉的覆蓋層中，永遠不會進入對話歷史記錄，所以您可以檢查詳細信息而不會增加 context。

<h3 id="use-subagents-for-investigation">
  使用 subagents 進行調查
</h3>

<Tip>
  使用 `「使用 subagents 調查 X」` 委派研究。他們在單獨的 context 中探索，為實施保持您的主對話乾淨。
</Tip>

由於 context 是您的基本約束，subagents 是可用的最強大的工具之一。當 Claude 研究代碼庫時，它讀取許多文件，所有這些都會消耗您的 context。Subagents 在單獨的 context windows 中運行並報告回摘要：

```text theme={null}
Use subagents to investigate how our authentication system handles token
refresh, and whether we have any existing OAuth utilities I should reuse.
```

subagent 探索代碼庫、讀取相關文件並報告發現，所有這些都不會使您的主對話變得混亂。

您也可以在 Claude 實施某些內容後使用 subagents 進行驗證：

```text theme={null}
use a subagent to review this code for edge cases
```

<h3 id="rewind-with-checkpoints">
  使用檢查點倒帶
</h3>

<Tip>
  您發送的每個提示都會創建一個檢查點。您可以將對話、代碼或兩者恢復到任何之前的檢查點。
</Tip>

Claude 在每次更改前自動快照文件，所以檢查點可以將它們恢復。雙擊 `Escape` 或運行 `/rewind` 打開倒帶菜單。您可以僅恢復對話、僅恢復代碼、恢復兩者或從選定的消息進行總結。有關詳細信息，請參閱 [Checkpointing](/docs/zh-TW/checkpointing)。

與其仔細規劃每一步，不如告訴 Claude 嘗試一些冒險的事情。如果不起作用，倒帶並嘗試不同的方法。檢查點在會話之間持續，所以您可以關閉終端並稍後仍然倒帶。

<Warning>
  檢查點僅跟蹤 Claude 進行的更改，不跟蹤外部進程。這不是 git 的替代品。
</Warning>

<h3 id="resume-conversations">
  恢復對話
</h3>

<Tip>
  使用 `/rename` 給會話命名，並像對待分支一樣對待它們：每個工作流都有自己的持久 context。
</Tip>

Claude Code 在本地保存對話，所以當任務跨越多個會話時，您不必重新解釋 context。運行 `claude --continue` 以選擇最近的會話，或 `claude --resume` 以從列表中選擇。給會話起描述性名稱，如 `oauth-migration`，以便您稍後可以找到它們。有關完整的恢復、分支和命名控制集，請參閱 [管理會話](/docs/zh-TW/sessions)。

***

<h2 id="automate-and-scale">
  自動化和擴展
</h2>

一旦您對一個 Claude 有效，通過平行會話、非交互模式和扇出模式將您的輸出乘以倍數。

到目前為止，一切都假設一個人、一個 Claude 和一個對話。但 Claude Code 水平擴展。本節中的技術展示了您如何完成更多工作。

<h3 id="run-non-interactive-mode">
  運行非交互模式
</h3>

<Tip>
  在 CI、pre-commit hooks 或腳本中使用 `claude -p "prompt"`。添加 `--output-format stream-json --verbose` 以獲得流式 JSON 輸出。
</Tip>

使用 `claude -p "your prompt"`，您可以非交互地運行 Claude，不需要會話。[非交互模式](/docs/zh-TW/headless)是您將 Claude 集成到 CI 管道、pre-commit hooks 或任何自動化工作流中的方式。輸出格式讓您以編程方式解析結果：純文本、JSON 或流式 JSON。

```bash theme={null}
# One-off queries
claude -p "Explain what this project does"

# Structured output for scripts
claude -p "List all API endpoints" --output-format json

# Streaming for real-time processing
claude -p "Analyze this log file" --output-format stream-json --verbose
```

<h3 id="run-multiple-claude-sessions">
  運行多個 Claude 會話
</h3>

<Tip>
  並行運行多個 Claude 會話以加快開發、運行隔離的實驗或啟動複雜的工作流。
</Tip>

選擇適合您想要自己進行多少協調的平行方法：

* [Worktrees](/docs/zh-TW/worktrees)：在隔離的 git 檢出中運行單獨的 CLI 會話，以便編輯不會衝突
* [桌面應用](/docs/zh-TW/desktop#work-in-parallel-with-sessions)：以視覺方式管理多個本地會話，每個會話都在自己的 worktree 中
* [Claude Code 在網絡上](/docs/zh-TW/claude-code-on-the-web)：在 Anthropic 管理的雲基礎設施中在隔離的 VM 中運行會話
* [Agent teams](/docs/zh-TW/agent-teams)：多個會話的自動協調，具有共享任務、消息和團隊領導

除了並行化工作外，多個會話還支持質量聚焦的工作流。新鮮的 context 改進代碼審查，因為 Claude 不會偏向於它剛剛編寫的代碼。

例如，使用 Writer/Reviewer 模式：

| 會話 A（Writer）               | 會話 B（Reviewer）                                                            |
| -------------------------- | ------------------------------------------------------------------------- |
| `實施我們 API 端點的速率限制器`        |                                                                           |
|                            | `審查 @src/middleware/rateLimiter.ts 中的速率限制器實施。查找邊界情況、競態條件和與我們現有中間件模式的一致性。` |
| `這是審查反饋：[會話 B 輸出]。解決這些問題。` |                                                                           |

您可以對測試做類似的事情：讓一個 Claude 編寫測試，然後另一個編寫代碼來通過它們。

<h3 id="fan-out-across-files">
  跨文件扇出
</h3>

<Tip>
  循環遍歷任務，為每個任務調用 `claude -p`。使用 `--allowedTools` 為批量操作限定權限。
</Tip>

對於大型遷移或分析，您可以在許多平行 Claude 調用中分配工作：

<Steps>
  <Step title="生成任務列表">
    讓 Claude 列出所有需要遷移的文件（例如，`列出所有 2,000 個需要遷移的 Python 文件`）
  </Step>

  <Step title="編寫腳本以循環遍歷列表">
    ```bash theme={null}
    for file in $(cat files.txt); do
      claude -p "Migrate $file from React to Vue. Return OK or FAIL." \
        --allowedTools "Edit,Bash(git commit *)"
    done
    ```
  </Step>

  <Step title="在幾個文件上測試，然後大規模運行">
    根據前 2-3 個文件出現的問題改進您的提示，然後在完整集合上運行。`--allowedTools` 標誌限制 Claude 可以做什麼，這在您無人值守運行時很重要。
  </Step>
</Steps>

您也可以將 Claude 集成到現有的數據/處理管道中：

```bash theme={null}
claude -p "<your prompt>" --output-format json | your_command
```

在開發期間使用 `--verbose` 進行調試，在生產中關閉它。

<h3 id="run-autonomously-with-auto-mode">
  使用 auto mode 自主運行
</h3>

對於不間斷的執行和背景安全檢查，使用 [auto mode](/docs/zh-TW/permission-modes#eliminate-prompts-with-auto-mode)。分類器模型在命令運行前審查它們，阻止範圍升級、未知基礎設施和由敵對內容驅動的操作，同時讓常規工作無提示進行。

```bash theme={null}
claude --permission-mode auto -p "fix all lint errors"
```

對於使用 `-p` 標誌的非交互運行，如果分類器重複阻止操作，auto mode 會中止，因為沒有用戶可以回退到。請參閱 [auto mode 何時回退](/docs/zh-TW/permission-modes#when-auto-mode-falls-back) 以了解閾值。

<h3 id="add-an-adversarial-review-step">
  添加對抗性審查步驟
</h3>

<Tip>
  在將任務視為完成之前，讓一個子代理在新鮮的 context 中審查差異並報告缺陷。
</Tip>

Claude 無人值守工作的時間越長，在您將工作視為完成之前進行獨立檢查就越重要。在新鮮的 [subagent](/docs/zh-TW/sub-agents) context 中運行的審查者只看到差異和您給它的標準，而不是產生更改的推理，因此它按自己的條款評估結果。

對於正確性檢查，運行捆綁的 [`/code-review` skill](/docs/zh-TW/commands)，它在新鮮的子代理中審查當前差異以查找錯誤，並將發現返回到會話。要檢查差異是否符合您的計劃，請自己編寫審查提示。命名要檢查的工作、要檢查的計劃以及什麼算作發現：

```text theme={null}
使用子代理根據 PLAN.md 審查速率限制器差異。檢查
每個要求都已實施、列出的邊界情況都有測試，以及
任務範圍之外沒有任何內容更改。報告缺陷，而不是風格偏好。
```

因為審查者作為子代理運行，實施會話直接接收缺陷，可以修復它們並重新審查，而無需您在窗口之間複製發現。對於較長的自主運行，[agent team](/docs/zh-TW/agent-teams) 可以在許多任務中保持此循環進行，同時您對記錄的發現進行抽查。

<Callout>
  被提示尋找缺陷的審查者通常會報告一些，即使工作是健全的，因為那是它被要求做的。追逐每個發現會導致過度工程：額外的抽象層、防禦性代碼和無法發生的情況的測試。告訴審查者只標記影響正確性或陳述要求的缺陷，並將其餘的視為可選。
</Callout>

***

<h2 id="avoid-common-failure-patterns">
  避免常見的失敗模式
</h2>

這些是常見的錯誤。及早識別它們可以節省時間：

* **廚房水槽會話。** 您從一個任務開始，然後詢問 Claude 不相關的事情，然後回到第一個任務。Context 充滿了不相關的信息。
  > **修復**：在不相關的任務之間使用 `/clear`。
* **一次又一次地改正。** Claude 做錯了什麼，您改正它，它仍然是錯的，您再次改正。Context 被失敗的方法所污染。
  > **修復**：在兩次失敗的改正後，`/clear` 並編寫一個更好的初始提示，包含您學到的內容。
* **過度指定的 CLAUDE.md。** 如果您的 CLAUDE.md 太長，Claude 會忽略其中的一半，因為重要的規則在噪音中丟失了。
  > **修復**：無情地修剪。如果 Claude 已經在沒有指令的情況下正確地做某事，刪除它或將其轉換為 hook。
* **信任然後驗證的差距。** Claude 產生看起來合理的實施，但不處理邊界情況。
  > **修復**：始終提供驗證（測試、腳本、截圖）。如果您無法驗證它，不要發布它。
* **無限探索。** 您要求 Claude「調查」某些內容而不限定範圍。Claude 讀取數百個文件，填滿 context。
  > **修復**：狹隘地限定調查範圍或使用 subagents，以便探索不會消耗您的主 context。

***

<h2 id="develop-your-intuition">
  培養您的直覺
</h2>

本指南中的模式不是一成不變的。它們是通常效果很好的起點，但可能不是每種情況的最優選擇。

有時您\_應該\_讓 context 累積，因為您深入一個複雜的問題，歷史很有價值。有時您應該跳過規劃，讓 Claude 找出答案，因為任務是探索性的。有時模糊的提示正是您想要的，因為您想在限制它之前看到 Claude 如何解釋問題。

注意什麼有效。當 Claude 產生出色的輸出時，注意您做了什麼：提示結構、您提供的 context、您所在的模式。當 Claude 遇到困難時，問為什麼。Context 太嘈雜了嗎？提示太模糊了嗎？任務對於一次通過來說太大了嗎？

隨著時間的推移，您將培養沒有指南可以捕捉的直覺。您將知道何時具體以及何時開放，何時規劃以及何時探索，何時清除 context 以及何時讓它累積。

<h2 id="related-resources">
  相關資源
</h2>

* [Claude Code 如何工作](/docs/zh-TW/how-claude-code-works)：代理循環、工具和 context 管理
* [擴展 Claude Code](/docs/zh-TW/features-overview)：skills、hooks、MCP、subagents 和 plugins
* [常見工作流](/docs/zh-TW/common-workflows)：調試、測試、PR 等的分步配方
* [CLAUDE.md](/docs/zh-TW/memory)：存儲項目約定和持久 context
