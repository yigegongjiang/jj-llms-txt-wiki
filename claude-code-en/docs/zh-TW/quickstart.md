> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 快速入門

> 歡迎使用 Claude Code！

本快速入門指南將在幾分鐘內讓您使用 AI 驅動的編碼協助。完成後，您將了解如何使用 Claude Code 進行常見的開發任務。

<h2 id="before-you-begin">
  開始前
</h2>

確保您擁有：

* 已開啟的終端或命令提示字元
  * 如果您從未使用過終端，請查看[終端指南](/docs/zh-TW/terminal-guide)
* 一個可以使用的程式碼專案
* 一個 [Claude 訂閱](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_prereq)（Pro、Max、Team 或 Enterprise）、[Claude Console](https://console.anthropic.com/) 帳戶，或透過[支援的雲端提供商](/docs/zh-TW/third-party-integrations)存取

<Note>
  本指南涵蓋終端 CLI。Claude Code 也可在[網頁](https://claude.ai/code)、[桌面應用程式](/docs/zh-TW/desktop)、[VS Code](/docs/zh-TW/vs-code) 和 [JetBrains IDE](/docs/zh-TW/jetbrains)、[Slack](/docs/zh-TW/slack) 中使用，以及透過 [GitHub Actions](/docs/zh-TW/github-actions) 和 [GitLab](/docs/zh-TW/gitlab-ci-cd) 進行 CI/CD。請參閱[所有介面](/docs/zh-TW/overview#use-claude-code-everywhere)。
</Note>

<h2 id="step-1-install-claude-code">
  步驟 1：安裝 Claude Code
</h2>

To install Claude Code, use one of the following methods:

<Tabs>
  <Tab title="Native Install (Recommended)">
    **macOS, Linux, WSL:**

    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash
    ```

    **Windows PowerShell:**

    ```powershell theme={null}
    irm https://claude.ai/install.ps1 | iex
    ```

    **Windows CMD:**

    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
    ```

    If you see `The token '&&' is not a valid statement separator`, you're in PowerShell, not CMD. If you see `'irm' is not recognized as an internal or external command`, you're in CMD, not PowerShell. Your prompt shows `PS C:\` when you're in PowerShell and `C:\` without the `PS` when you're in CMD.

    If the install command fails with `syntax error near unexpected token '<'`, a `403`, or another curl error, see [Troubleshoot installation](/docs/en/troubleshoot-install#find-your-error) to match the error to a fix and for alternative install methods.

    [Git for Windows](https://git-scm.com/downloads/win) is recommended on native Windows so Claude Code can use the Bash tool. If Git for Windows is not installed, Claude Code uses PowerShell as the shell tool instead. WSL setups do not need Git for Windows.

    <Info>
      Native installations automatically update in the background to keep you on the latest version.
    </Info>
  </Tab>

  <Tab title="Homebrew">
    ```bash theme={null}
    brew install --cask claude-code
    ```

    Homebrew offers two casks. `claude-code` tracks the stable release channel, which is typically about a week behind and skips releases with major regressions. `claude-code@latest` tracks the latest channel and receives new versions as soon as they ship.

    <Info>
      Homebrew installations do not auto-update. Run `brew upgrade claude-code` or `brew upgrade claude-code@latest`, depending on which cask you installed, to get the latest features and security fixes.
    </Info>
  </Tab>

  <Tab title="WinGet">
    ```powershell theme={null}
    winget install Anthropic.ClaudeCode
    ```

    <Info>
      WinGet installations do not auto-update. Run `winget upgrade Anthropic.ClaudeCode` periodically to get the latest features and security fixes.
    </Info>
  </Tab>
</Tabs>

You can also install with [apt, dnf, or apk](/docs/en/setup#install-with-linux-package-managers) on Debian, Fedora, RHEL, and Alpine.

<h2 id="step-2-log-in-to-your-account">
  步驟 2：登入您的帳戶
</h2>

Claude Code 需要帳戶才能使用。使用 `claude` 命令啟動互動式工作階段，首次使用時系統會提示您登入：

```bash theme={null}
claude
```

對於 Claude 訂閱或 Console 帳戶，請按照提示在瀏覽器中完成驗證。若要稍後切換帳戶或重新驗證，請在執行中的工作階段內輸入 `/login`：

```text theme={null}
/login
```

您可以使用以下任何帳戶類型登入：

* [Claude Pro、Max、Team 或 Enterprise](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_login)（推薦）
* [Claude Console](https://console.anthropic.com/)（具有預付額度的 API 存取）。首次登入時，Console 中會自動建立「Claude Code」工作區以進行集中成本追蹤。
* [Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry](/docs/zh-TW/third-party-integrations)（企業雲端提供商）
* 自行託管的 [Claude 應用程式閘道](/docs/zh-TW/claude-apps-gateway)（如果您的組織執行一個的話）：您的管理員會預先設定閘道 URL，`/login` 會直接在 **Cloud gateway** 畫面上開啟，供您使用公司 SSO 登入

登入後，您的認證將被儲存，您無需再次登入。

<h2 id="step-3-start-your-first-session">
  步驟 3：啟動您的第一個工作階段
</h2>

在任何專案目錄中開啟您的終端並啟動 Claude Code：

```bash theme={null}
cd /path/to/your/project
claude
```

您將看到 Claude Code 提示，其中顯示版本、目前的模型和工作目錄。輸入 `/help` 以查看可用命令，或輸入 `/resume` 以繼續之前的對話。

<Tip>
  登入後（步驟 2），您的認證將儲存在您的系統上。在[認證管理](/docs/zh-TW/authentication#credential-management)中了解更多。
</Tip>

<h2 id="step-4-ask-your-first-question">
  步驟 4：提出您的第一個問題
</h2>

讓我們從了解您的程式碼庫開始。嘗試以下命令之一：

```text theme={null}
what does this project do?
```

Claude 將分析您的檔案並提供摘要。您也可以提出更具體的問題：

```text theme={null}
what technologies does this project use?
```

```text theme={null}
where is the main entry point?
```

```text theme={null}
explain the folder structure
```

您也可以詢問 Claude 其自身的功能：

```text theme={null}
what can Claude Code do?
```

```text theme={null}
how do I create custom skills in Claude Code?
```

```text theme={null}
can Claude Code work with Docker?
```

<Note>
  Claude Code 會根據需要讀取您的專案檔案。您無需手動新增內容。
</Note>

<h2 id="step-5-make-your-first-code-change">
  步驟 5：進行您的第一次程式碼變更
</h2>

現在讓我們讓 Claude Code 進行一些實際的編碼。嘗試一個簡單的任務：

```text theme={null}
add a hello world function to the main file
```

Claude Code 將：

1. 找到適當的檔案
2. 向您顯示建議的變更
3. 要求您的批准
4. 進行編輯

<Note>
  Claude Code 在修改檔案前始終要求許可。您可以批准個別變更或為工作階段啟用「全部接受」模式。
</Note>

<h2 id="step-6-use-git-with-claude-code">
  步驟 6：使用 Git 與 Claude Code
</h2>

Claude Code 使 Git 操作變得對話式：

```text theme={null}
what files have I changed?
```

```text theme={null}
commit my changes with a descriptive message
```

您也可以提示進行更複雜的 Git 操作：

```text theme={null}
create a new branch called feature/quickstart
```

```text theme={null}
show me the last 5 commits
```

```text theme={null}
help me resolve merge conflicts
```

<h2 id="step-7-fix-a-bug-or-add-a-feature">
  步驟 7：修復錯誤或新增功能
</h2>

Claude 擅長除錯和功能實現。

用自然語言描述您想要的內容：

```text theme={null}
add input validation to the user registration form
```

或修復現有問題：

```text theme={null}
there's a bug where users can submit empty forms - fix it
```

Claude Code 將：

* 定位相關程式碼
* 理解上下文
* 實現解決方案
* 如果可用，執行測試

<h2 id="step-8-test-out-other-common-workflows">
  步驟 8：測試其他常見工作流程
</h2>

有許多方式可以與 Claude 合作：

**重構程式碼**

```text theme={null}
refactor the authentication module to use async/await instead of callbacks
```

**編寫測試**

```text theme={null}
write unit tests for the calculator functions
```

**更新文件**

```text theme={null}
update the README with installation instructions
```

**程式碼審查**

```text theme={null}
review my changes and suggest improvements
```

<Tip>
  像與有幫助的同事交談一樣與 Claude 交談。描述您想要達成的目標，它將幫助您實現。
</Tip>

<h2 id="essential-commands">
  基本命令
</h2>

以下是日常使用中最重要的命令。Shell 命令從您的終端機執行以啟動或繼續 Claude Code。工作階段命令在 Claude Code 啟動後在其內部執行。

**Shell 命令**

| 命令                  | 功能            | 範例                                  |
| ------------------- | ------------- | ----------------------------------- |
| `claude`            | 啟動互動模式        | `claude`                            |
| `claude "task"`     | 執行一次性任務       | `claude "fix the build error"`      |
| `claude -p "query"` | 執行一次性查詢，然後退出  | `claude -p "explain this function"` |
| `claude -c`         | 在目前目錄中繼續最近的對話 | `claude -c`                         |
| `claude -r`         | 恢復之前的對話       | `claude -r`                         |

**工作階段命令**

| 命令               | 功能             | 範例       |
| ---------------- | -------------- | -------- |
| `/clear`         | 清除對話歷史         | `/clear` |
| `/help`          | 顯示可用命令         | `/help`  |
| `/exit` 或 Ctrl+D | 退出 Claude Code | `/exit`  |

請參閱 [CLI 參考](/docs/zh-TW/cli-reference)以取得完整的 shell 命令清單，以及 [命令參考](/docs/zh-TW/commands)以取得完整的工作階段命令清單。

<h2 id="pro-tips-for-beginners">
  初學者的專業提示
</h2>

如需更多資訊，請參閱[最佳實踐](/docs/zh-TW/best-practices)和[常見工作流程](/docs/zh-TW/common-workflows)。

<AccordionGroup>
  <Accordion title="對您的請求要具體">
    不要這樣做："修復錯誤"

    試試這樣："修復登入錯誤，使用者輸入錯誤認證後看到空白畫面"
  </Accordion>

  <Accordion title="使用逐步說明">
    將複雜任務分解為步驟：

    ```text theme={null}
    1. create a new database table for user profiles
    2. create an API endpoint to get and update user profiles
    3. build a webpage that allows users to see and edit their information
    ```
  </Accordion>

  <Accordion title="讓 Claude 先探索">
    在進行變更之前，讓 Claude 了解您的程式碼：

    ```text theme={null}
    analyze the database schema
    ```

    ```text theme={null}
    build a dashboard showing products that are most frequently returned by our UK customers
    ```
  </Accordion>

  <Accordion title="使用快捷方式節省時間">
    * 輸入 `/` 查看所有命令和 skills
    * 使用 Tab 進行命令完成
    * 按 ↑ 查看命令歷史
    * 按 `Shift+Tab` 循環切換權限模式
  </Accordion>
</AccordionGroup>

<h2 id="what’s-next">
  接下來呢？
</h2>

現在您已經學習了基礎知識，請探索更多進階功能：

<CardGroup cols={2}>
  <Card title="Claude Code 如何運作" icon="microchip" href="/docs/zh-TW/how-claude-code-works">
    了解代理迴圈、內建工具以及 Claude Code 如何與您的專案互動
  </Card>

  <Card title="最佳實踐" icon="star" href="/docs/zh-TW/best-practices">
    透過有效的提示和專案設定獲得更好的結果
  </Card>

  <Card title="常見工作流程" icon="graduation-cap" href="/docs/zh-TW/common-workflows">
    常見任務的逐步指南
  </Card>

  <Card title="擴展 Claude Code" icon="puzzle-piece" href="/docs/zh-TW/features-overview">
    使用 CLAUDE.md、skills、hooks、MCP 等進行自訂
  </Card>
</CardGroup>

<h2 id="getting-help">
  獲取幫助
</h2>

* **在 Claude Code 中**：輸入 `/help` 或詢問「how do I...」
* **文件**：您在這裡！瀏覽其他指南
* **社群**：加入我們的 [Discord](https://www.anthropic.com/discord) 以獲取提示和支援
