> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 修改系統提示詞

> 在 `claude_code` 預設和自訂系統提示詞之間選擇，並使用 CLAUDE.md、輸出樣式、append 或完全自訂提示詞來自訂行為。

系統提示詞定義了 Claude 的行為、功能和回應風格。從 `claude_code` 預設開始，適用於 CLI 或類似 IDE 的編碼工具，其中人類監督並引導工作。為具有不同介面、身份或權限模型的代理編寫您自己的提示詞。

本頁涵蓋：

* [系統提示詞如何運作](#how-system-prompts-work)，包含一個決策表，用於在預設、帶有 `append` 的預設和自訂提示詞之間進行選擇
* [自訂代理行為](#customize-agent-behavior)，使用 CLAUDE.md 檔案、輸出樣式、`append` 或自訂字串
* [比較四種方法](#compare-the-four-approaches)，按持久性、範圍和它們保留的內容進行比較
* [組合方法](#combine-approaches)，將自訂方法分層組合在一起

<h2 id="how-system-prompts-work">
  系統提示詞如何運作
</h2>

系統提示詞是初始指令集，塑造了 Claude 在整個對話中的行為方式。Agent SDK 有三個起點：

* **最小預設**：當您在 TypeScript 中未設定 `systemPrompt` 或在 Python 中未設定 `system_prompt` 時，SDK 使用最小提示詞，涵蓋工具呼叫但省略了 Claude Code 的編碼指南、回應風格和專案上下文。這與 `claude -p` 不同，後者預設使用完整的 Claude Code 提示詞。如果您正在從 CLI 遷移並想要相符的行為，請設定 `claude_code` 預設。
* **`claude_code` 預設**：Claude Code CLI 使用的完整系統提示詞，包含工具使用指令、程式碼風格和格式化指南、回應語氣和詳細程度規則、安全和防護指令，以及關於工作目錄和環境的上下文。在 TypeScript 中設定 `systemPrompt: { type: "preset", preset: "claude_code" }`，或在 Python 中設定 `system_prompt={"type": "preset", "preset": "claude_code"}`，可選擇使用 `append` 在末尾新增您自己的指令。
* **自訂字串**：您自己撰寫的提示詞。SDK 只會傳送您提供的內容。

<h3 id="decide-on-a-starting-point">
  決定起點
</h3>

決定因素是您的代理與 Claude Code 的相似程度：一個在儲存庫中運作的編碼代理，有人類監看串流輸出並引導工作。您的產品離該情況越遠，您就越想撰寫自己的提示詞。

| 您正在建置                                                  | 使用                          | 您會得到                                           |
| :----------------------------------------------------- | :-------------------------- | :--------------------------------------------- |
| 一個 CLI 或類似 IDE 的編碼工具，其中人類監看並引導，而 Claude Code 的預設值是您想要的 | `claude_code` 預設            | 完整的 Claude Code 提示詞：工具指導、安全規則、終端機友善的回應、儲存庫慣例感知 |
| 相同類型的工具，加上產品特定的規則，如編碼標準、輸出格式或領域上下文                     | `claude_code` 預設搭配 `append` | 上述所有內容，加上您的指令新增在預設之後。沒有任何內容被移除，所以這是風險最低的自訂     |
| 具有不同表面、身份或權限模型的代理，或非編碼代理                               | 自訂提示詞字串                     | 僅您撰寫的內容。您負責替換您的代理仍需要的工具指導和安全指令                 |
| 一個薄型工具呼叫迴圈，沒有代理角色，您在使用者提示詞中提供所有行為                      | 無 `systemPrompt` 選項         | 最小預設：工具呼叫支援，沒有其他                               |

「不同於 Claude Code」通常意味著以下其中之一：

* **不同的表面**：輸出不是由觸發它的人在終端機中讀取。聊天 UI、結構化輸出消費者和非編碼自動化各自需要一個與其輸出呈現和審查方式相符的提示詞。無人值守的編碼自動化，如修復 lint 錯誤或審查差異的 CI 工作，仍然符合預設，因為工作本身是預設為其編寫的。
* **不同的身份**：代理不應將自己呈現為 Claude Code。支援機器人、資料分析助手或任何特定領域的代理需要自己的名稱、範圍和角色。
* **不同的權限模型**：代理自主運作，無需人類批准每一步，或在一組狹隘的資源上運作。Claude Code 的提示詞假設人類在迴圈中，可以存取完整的工具集。
* **非編碼任務**：Claude Code 提示詞的大部分是編碼指導。對於研究、內容或運營代理，該指導與您實際需要的指令競爭。

[比較表](#compare-the-four-approaches)顯示每種自訂方法保留的內容。

<h2 id="customize-agent-behavior">
  自訂代理程式行為
</h2>

輸出樣式、`append` 和自訂提示詞字串各自直接改變系統提示詞。CLAUDE.md 採用不同的路徑：SDK 讀取它並將其內容作為專案上下文注入對話中，而不是注入系統提示詞，因此它與您選擇的任何系統提示詞一起塑造行為。[Skills](/docs/zh-TW/agent-sdk/skills)、[hooks](/docs/zh-TW/agent-sdk/hooks) 和 [permissions](/docs/zh-TW/agent-sdk/permissions) 也在系統提示詞之外塑造行為，並在各自的頁面上涵蓋。

<h3 id="claude-md-files-for-project-level-instructions">
  專案級指令的 CLAUDE.md 檔案
</h3>

CLAUDE.md 檔案為 Claude 提供持久的專案上下文和指令。SDK 將其內容注入對話中，而不是注入系統提示詞，因此它們適用於任何系統提示詞配置。關於在 CLAUDE.md 中放入什麼、放在哪裡以及如何編寫有效的指令，請參閱 [Claude 如何記住您的專案](/docs/zh-TW/memory)。本節涵蓋 SDK 特定的內容：CLAUDE.md 如何載入。

當匹配的設定來源已啟用時，SDK 讀取 CLAUDE.md：`'project'` 從工作目錄載入 `CLAUDE.md` 或 `.claude/CLAUDE.md`，`'user'` 載入 `~/.claude/CLAUDE.md`。預設 `query()` 選項啟用兩個來源，因此 CLAUDE.md 會自動載入。如果您在 TypeScript 中明確設定 `settingSources` 或在 Python 中設定 `setting_sources`，請包含您需要的來源。CLAUDE.md 載入由設定來源控制，而不是由 `claude_code` 預設值控制。

<h4 id="load-claude-md-with-the-sdk">
  使用 SDK 載入 CLAUDE.md
</h4>

要載入 CLAUDE.md，請設定 `settingSources` 以包含您的 CLAUDE.md 所在的級別。下面的範例載入專案級 CLAUDE.md 以及 `claude_code` 預設值，因此 Claude 同時具有完整的編碼代理程式提示詞和您專案的約定：

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const messages = [];

  for await (const message of query({
    prompt: "Add a new React component for user profiles",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code" // 使用 Claude Code 的系統提示詞
      },
      settingSources: ["project"] // 從專案載入 CLAUDE.md
    }
  })) {
    messages.push(message);
  }

  // 現在 Claude 可以存取您來自 CLAUDE.md 的專案指南
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  messages = []

  async for message in query(
      prompt="Add a new React component for user profiles",
      options=ClaudeAgentOptions(
          system_prompt={
              "type": "preset",
              "preset": "claude_code",  # 使用 Claude Code 的系統提示詞
          },
          setting_sources=["project"],  # 從專案載入 CLAUDE.md
      ),
  ):
      messages.append(message)

  # 現在 Claude 可以存取您來自 CLAUDE.md 的專案指南
  ```
</CodeGroup>

CLAUDE.md 在專案的所有會話中持續存在，通過 git 與您的團隊共享，並自動發現而無需程式碼變更。如果您傳遞空的 `settingSources` 陣列，則不會載入。

<h3 id="output-styles-for-persistent-configurations">
  輸出樣式用於持久配置
</h3>

輸出樣式是修改 Claude 系統提示詞的已儲存配置。它們儲存為 markdown 檔案，可以在會話和專案中重複使用。

<h4 id="create-an-output-style">
  建立輸出樣式
</h4>

輸出樣式是一個 markdown 檔案，其 [frontmatter](/docs/zh-TW/output-styles#frontmatter) 中有中繼資料，後面跟著提示詞內容。將其儲存到 `~/.claude/output-styles/` 以獲得在每個專案中可用的使用者級樣式，或儲存到您的儲存庫中的 `.claude/output-styles/` 以獲得可以提交並與您的團隊共享的專案級樣式。

預設情況下，自訂輸出樣式會用您自己的指令替換 `claude_code` 預設值的軟體工程指令。要保留它們並在其上分層您的指令，請在 frontmatter 中設定 `keep-coding-instructions: true`。當您的代理程式仍在進行軟體工程工作時保留它們。當您完全替換角色時省略它們。

下面的範例定義了程式碼審查角色，該角色保留編碼指令，因為審查程式碼仍然受益於 Claude Code 的安全性和程式碼品質指導。將其儲存為 `~/.claude/output-styles/code-reviewer.md` 以在專案中提供：

```markdown ~/.claude/output-styles/code-reviewer.md theme={null}
---
name: Code Reviewer
description: Thorough code review assistant
keep-coding-instructions: true
---

You are an expert code reviewer.

For every code submission:
1. Check for bugs and security issues
2. Evaluate performance
3. Suggest improvements
4. Rate code quality (1-10)
```

<h4 id="activate-an-output-style">
  啟用輸出樣式
</h4>

建立後，通過以下方式啟用輸出樣式：

* **CLI**：執行 `/config` 並選擇輸出樣式
* **設定**：在 `.claude/settings.local.json` 中設定 `outputStyle`
* **TypeScript SDK**：在傳遞給 `query()` 的內聯 `settings` 物件內設定 `outputStyle`，或將 `settings` 指向設定該值的設定檔。`outputStyle` 不是頂級 `Options` 欄位：

  ```typescript theme={null}
  const options = { settings: { outputStyle: "Explanatory" } };
  ```

Python SDK 沒有以程式設計方式選擇輸出樣式的選項。對於無法寫入 `.claude/settings.local.json` 的僅程式碼部署，請改用 `append` 或自訂提示詞字串。

**SDK 使用者注意：** 當您在選項中包含 `settingSources: ['user']` 或 `settingSources: ['project']`（TypeScript）/ `setting_sources=["user"]` 或 `setting_sources=["project"]`（Python）時，輸出樣式會被載入。

<h3 id="append-to-the-claude_code-preset">
  附加到 `claude_code` 預設值
</h3>

您可以使用 Claude Code 預設值搭配 `append` 屬性來新增自訂指令，同時保留所有內建功能。

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const messages = [];

  for await (const message of query({
    prompt: "Help me write a Python function to calculate fibonacci numbers",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: "Always include detailed docstrings and type hints in Python code."
      }
    }
  })) {
    messages.push(message);
    if (message.type === "assistant") {
      console.log(message.message.content);
    }
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  messages = []

  async for message in query(
      prompt="Help me write a Python function to calculate fibonacci numbers",
      options=ClaudeAgentOptions(
          system_prompt={
              "type": "preset",
              "preset": "claude_code",
              "append": "Always include detailed docstrings and type hints in Python code.",
          }
      ),
  ):
      messages.append(message)
      if isinstance(message, AssistantMessage):
          print(message.content)
  ```
</CodeGroup>

<h4 id="improve-prompt-caching-across-users-and-machines">
  改進跨使用者和機器的提示詞快取
</h4>

預設情況下，使用相同 `claude_code` 預設值和 `append` 文字的兩個會話，如果從不同的工作目錄執行，仍然無法共享提示詞快取項目。這是因為預設值在您的 `append` 文字之前在系統提示詞中嵌入了每個會話的上下文：工作目錄、它是否為 git 儲存庫、平台、活動 shell、OS 版本和自動記憶路徑。該上下文中的任何差異都會產生不同的系統提示詞和快取未命中。CLAUDE.md 內容不會影響系統提示詞快取，因為 SDK 將其注入對話中，而不是系統提示詞。

要使系統提示詞在會話中相同，請在 TypeScript 中設定 `excludeDynamicSections: true`，或在 Python 中設定 `"exclude_dynamic_sections": True`。每個會話的上下文移動到第一個使用者訊息中，只在系統提示詞中保留靜態預設值和您的 `append` 文字，以便相同的配置可以在使用者和機器之間共享快取項目。

<Note>
  `excludeDynamicSections` 需要 `@anthropic-ai/claude-agent-sdk` v0.2.98 或更新版本，或 Python 的 `claude-agent-sdk` v0.1.58 或更新版本。它僅適用於預設物件形式，當 `systemPrompt` 是字串時無效。
</Note>

以下範例將共享 `append` 區塊與 `excludeDynamicSections` 配對，以便從不同目錄執行的代理程式群隊可以重複使用相同的快取系統提示詞：

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Triage the open issues in this repo",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: "You operate Acme's internal triage workflow. Label issues by component and severity.",
        excludeDynamicSections: true
      }
    }
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt="Triage the open issues in this repo",
      options=ClaudeAgentOptions(
          system_prompt={
              "type": "preset",
              "preset": "claude_code",
              "append": "You operate Acme's internal triage workflow. Label issues by component and severity.",
              "exclude_dynamic_sections": True,
          },
      ),
  ):
      ...
  ```
</CodeGroup>

**權衡：** 工作目錄、git 儲存庫旗標、平台、活動 shell、OS 版本和自動記憶路徑仍然會到達 Claude，但作為第一個使用者訊息的一部分，而不是系統提示詞。使用者訊息中的指令比系統提示詞中的相同文字的權重略低，所以 Claude 在推理目前目錄或自動記憶路徑時可能會較少依賴它們。當跨會話快取重複使用比最大化權威環境上下文更重要時，請啟用此選項。

對於非互動式 CLI 模式中的等效旗標，請參閱 [`--exclude-dynamic-system-prompt-sections`](/docs/zh-TW/cli-reference)。

<h3 id="custom-system-prompts">
  自訂系統提示詞
</h3>

您可以提供自訂字串作為 `systemPrompt` 以完全用您自己的指令替換預設值。

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const customPrompt = `You are a Python coding specialist.
  Follow these guidelines:
  - Write clean, well-documented code
  - Use type hints for all functions
  - Include comprehensive docstrings
  - Prefer functional programming patterns when appropriate
  - Always explain your code choices`;

  const messages = [];

  for await (const message of query({
    prompt: "Create a data processing pipeline",
    options: {
      systemPrompt: customPrompt
    }
  })) {
    messages.push(message);
    if (message.type === "assistant") {
      console.log(message.message.content);
    }
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage

  custom_prompt = """You are a Python coding specialist.
  Follow these guidelines:
  - Write clean, well-documented code
  - Use type hints for all functions
  - Include comprehensive docstrings
  - Prefer functional programming patterns when appropriate
  - Always explain your code choices"""

  messages = []

  async for message in query(
      prompt="Create a data processing pipeline",
      options=ClaudeAgentOptions(system_prompt=custom_prompt),
  ):
      messages.append(message)
      if isinstance(message, AssistantMessage):
          print(message.content)
  ```
</CodeGroup>

<h2 id="compare-the-four-approaches">
  比較四種方法
</h2>

四種自訂方法在位置、共享方式和從 `claude_code` 預設保留的內容方面有所不同。

| 功能         | CLAUDE.md | 輸出樣式     | `systemPrompt` 附加 | 自訂 `systemPrompt` |
| ---------- | --------- | -------- | ----------------- | ----------------- |
| **持久性**    | 每個專案檔案    | 儲存為檔案    | 僅限會話              | 僅限會話              |
| **可重複使用性** | 每個專案      | 跨專案      | 程式碼重複             | 程式碼重複             |
| **管理**     | 在檔案系統上    | CLI + 檔案 | 在程式碼中             | 在程式碼中             |
| **預設工具**   | 保留        | 保留       | 保留                | 遺失（除非包含）          |
| **內建安全**   | 維持        | 維持       | 維持                | 必須新增              |
| **環境上下文**  | 自動        | 自動       | 自動                | 必須提供              |
| **自訂程度**   | 僅新增       | 替換或擴展預設  | 僅新增               | 完全控制              |
| **版本控制**   | 與專案一起     | 是        | 與程式碼一起            | 與程式碼一起            |
| **範圍**     | 專案特定      | 使用者或專案   | 程式碼會話             | 程式碼會話             |

「附加」是指在 TypeScript 中使用 `systemPrompt: { type: "preset", preset: "claude_code", append: "..." }`，或在 Python 中使用 `system_prompt={"type": "preset", "preset": "claude_code", "append": "..."}`。CLAUDE.md 不會變更系統提示本身：SDK 會將其內容作為專案上下文注入到對話中。

<h2 id="use-cases-and-best-practices">
  使用案例和最佳實踐
</h2>

<h3 id="when-to-use-claude-md">
  何時使用 CLAUDE.md
</h3>

使用 CLAUDE.md 來記錄應該適用於專案中每個會話的指令，無論該會話使用哪個系統提示詞：編碼標準、常見命令、架構上下文和團隊約定。CLAUDE.md 被提交到您的儲存庫，因此它與它描述的程式碼保持同步。請參閱 [何時新增到 CLAUDE.md](/docs/zh-TW/memory#when-to-add-to-claude-md) 以獲得完整指導。

CLAUDE.md 檔案在啟用 `project` 設定來源時載入，預設 `query()` 選項已啟用此功能。如果您在 TypeScript 中明確設定 `settingSources` 或在 Python 中設定 `setting_sources`，請包含 `'project'` 以繼續載入專案級 CLAUDE.md。

<h3 id="when-to-use-output-styles">
  何時使用輸出樣式
</h3>

輸出樣式適用於您想在 CLI 和 SDK 中重複使用的角色，無需變更應用程式程式碼。因為它們作為檔案存在於 `.claude/output-styles` 中，相同的角色可從 CLI 中的 `/config` 以及載入相符設定來源的任何 SDK 會話中使用。

**最適合：**

* 跨會話的持久行為變更
* 團隊共享配置
* 專門助手，例如程式碼審查者、資料科學家或 DevOps 助手
* 需要版本控制的複雜提示詞修改

**範例：**

* 建立專用的 SQL 最佳化助手
* 建置安全焦點的程式碼審查者
* 開發具有特定教學法的教學助手

<h3 id="when-to-use-systemprompt-with-append">
  何時使用 `systemPrompt` 附加
</h3>

在 `claude_code` 預設已經符合您的產品需求，而您只需要疊加額外指令時，使用 `append`。您保留預設的工具指導、安全規則和編碼約定，無需重新實現它們。

**最適合：**

* 新增特定的編碼標準或偏好
* 自訂輸出格式
* 新增領域特定知識
* 修改回應詳細程度
* 在不失去工具指令的情況下增強 Claude Code 的預設行為

<h3 id="when-to-use-custom-systemprompt">
  何時使用自訂 `systemPrompt`
</h3>

當您的代理程式的表面、身份或權限模型與 Claude Code 的不同時，使用自訂提示詞，如 [決定起點](#decide-on-a-starting-point) 中所述。您定義完整的指令集，包括您的代理程式需要的任何工具指導和安全規則。

**最適合：**

* 完全控制 Claude 的行為
* 專門的單一會話任務
* 測試新的提示詞策略
* 不需要預設工具的情況
* 建置具有獨特行為的專門代理程式

<h2 id="combine-approaches">
  結合方法
</h2>

這些方法可以組合使用。持久輸出樣式或 CLAUDE.md 設定長期行為，而 `append` 在不觸及已保存配置的情況下，在頂部疊加會話特定的指令。

<h3 id="combine-an-output-style-with-session-specific-additions">
  結合輸出樣式與會話特定新增
</h3>

下面的範例假設 Code Reviewer 輸出樣式已經啟用。`append` 區塊在角色上疊加會話特定的焦點區域，因此單一審查會話可以優先考慮 OAuth 和令牌存儲，而無需更改已保存的輸出樣式：

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // 假設「Code Reviewer」輸出樣式已啟用（通過 /config 或設定）
  // 新增會話特定的焦點區域
  const messages = [];

  for await (const message of query({
    prompt: "Review this authentication module",
    options: {
      systemPrompt: {
        type: "preset",
        preset: "claude_code",
        append: `
          For this review, prioritize:
          - OAuth 2.0 compliance
          - Token storage security
          - Session management
        `
      }
    }
  })) {
    messages.push(message);
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  # 假設「Code Reviewer」輸出樣式已啟用（通過 /config 或設定）
  # 新增會話特定的焦點區域
  messages = []

  async for message in query(
      prompt="Review this authentication module",
      options=ClaudeAgentOptions(
          system_prompt={
              "type": "preset",
              "preset": "claude_code",
              "append": """
              For this review, prioritize:
              - OAuth 2.0 compliance
              - Token storage security
              - Session management
              """,
          }
      ),
  ):
      messages.append(message)
  ```
</CodeGroup>

<h2 id="see-also">
  另請參閱
</h2>

* [輸出樣式](/docs/zh-TW/output-styles)：為 CLI 建立、管理和分享輸出樣式，包括檔案格式和儲存位置
* [Claude 如何記住您的專案](/docs/zh-TW/memory)：CLAUDE.md 中應放入的內容、放置位置，以及如何撰寫有效的專案指示
* [TypeScript SDK 參考](/docs/zh-TW/agent-sdk/typescript)：完整的 `Options` 類型，包括 `systemPrompt`、`settingSources` 和 `settings`
* [Python SDK 參考](/docs/zh-TW/agent-sdk/python)：完整的 `ClaudeAgentOptions` 類型，包括 `system_prompt` 和 `setting_sources`
* [設定](/docs/zh-TW/settings)：`settings.json` 參考，包括輸出樣式和其他配置的儲存位置
