> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 設定權限

> 使用權限模式、hooks 和宣告式允許/拒絕規則來控制您的代理程式如何使用工具。

Claude Agent SDK 提供權限控制來管理 Claude 如何使用工具。使用權限模式和規則來定義自動允許的內容，並使用 [`canUseTool` 回呼](/docs/zh-TW/agent-sdk/user-input) 在執行時處理其他所有情況。

<Note>
  本頁涵蓋權限模式和規則。若要建立互動式核准流程，讓使用者在執行時核准或拒絕工具請求，請參閱 [處理核准和使用者輸入](/docs/zh-TW/agent-sdk/user-input)。
</Note>

<h2 id="how-permissions-are-evaluated">
  權限如何被評估
</h2>

當 Claude 請求工具時，SDK 會按照以下順序檢查權限：

<Steps>
  <Step title="Hooks">
    首先執行 [hooks](/docs/zh-TW/agent-sdk/hooks)。Hook 可以直接拒絕呼叫或將其傳遞。返回 `allow` 的 hook 不會跳過下面的拒絕和詢問規則；無論 hook 結果如何，這些規則都會被評估。
  </Step>

  <Step title="拒絕規則">
    檢查 `deny` 規則（來自 `disallowed_tools` 和 [settings.json](/docs/zh-TW/settings#permission-settings)）。如果拒絕規則符合，工具會被阻止，即使在 `bypassPermissions` 模式下也是如此。裸名稱拒絕規則（如 `Bash`）會在此評估開始前將工具從 Claude 的上下文中移除，因此只有範圍規則（如 `Bash(rm *)`）會在此步驟中被檢查。
  </Step>

  <Step title="詢問規則">
    檢查來自 [settings.json](/docs/zh-TW/settings#permission-settings) 的 `ask` 規則。如果詢問規則符合，呼叫會傳遞到您的 [`canUseTool` 回呼](/docs/zh-TW/agent-sdk/user-input) 以進行確認，即使在 `bypassPermissions` 模式下也是如此。

    需要使用者互動的工具行為相同：`AskUserQuestion` 和 MCP 工具（其伺服器設定 [`_meta["anthropic/requiresUserInteraction"]`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool)）總是會傳遞到回呼，即使允許規則符合時也是如此。在 `dontAsk` 模式下，兩種情況都會被拒絕，因為該模式永遠不會提示。MCP 註解需要 Claude Code v2.1.199 或更新版本。

    [claude.ai 連接器](/docs/zh-TW/mcp#organization-controls-on-connector-tools) 工具（您的組織已設定為 `ask`）也會在此步驟離開流程。每個呼叫都會傳遞到回呼，即使在 `bypassPermissions` 模式下，即使允許規則符合時也是如此。回呼會收到原因 `Your organization requires approval for this tool`。在 `dontAsk` 模式下，呼叫會被拒絕，因為該模式永遠不會提示。
  </Step>

  <Step title="權限模式">
    應用活躍的 [權限模式](#permission-modes)。`bypassPermissions` 批准到達此步驟的所有內容。`acceptEdits` 批准檔案操作。`plan` 將檔案編輯和 shell 寫入工具路由到您的 `canUseTool` 回呼，無論允許規則如何，因此在規劃時寫入操作無法自動批准。其他模式會通過。
  </Step>

  <Step title="允許規則">
    檢查 `allow` 規則（來自 `allowed_tools` 和 settings.json）。如果規則符合，工具會被批准。
  </Step>

  <Step title="canUseTool 回呼">
    如果上述任何步驟都未解決，請呼叫您的 [`canUseTool` 回呼](/docs/zh-TW/agent-sdk/user-input) 以做出決定。在 `dontAsk` 模式下，此步驟會被跳過，工具會被拒絕。
  </Step>
</Steps>

<img src="https://mintcdn.com/claude-code/jYgs7qigNjO1Badj/images/agent-sdk/permissions-flow.svg?fit=max&auto=format&n=jYgs7qigNjO1Badj&q=85&s=c771ad9085b1277d3708027a49c744bc" alt="六步驟權限評估流程圖，與上述步驟相符：工具請求通過 hooks、拒絕規則、詢問規則、權限模式、允許規則和 canUseTool。Hooks、拒絕規則和 canUseTool 可以路由到'已阻止'；權限模式繞過、允許規則和 canUseTool 可以路由到'執行'；詢問規則路由到 canUseTool。" width="1180" height="260" data-path="images/agent-sdk/permissions-flow.svg" />

自 v2.1.198 起，如果您傳遞一個 `canUseTool` 回呼，而此評估順序永遠無法到達，TypeScript SDK 會在建構查詢時發出一次 Node.js 程序警告。警告的代碼是 `CLAUDE_SDK_CAN_USE_TOOL_SHADOWED`。兩種配置會觸發它：

* `permissionMode: 'bypassPermissions'`，它會自動批准到達權限模式步驟的每個呼叫
* 每個裸 `allowedTools` 項目，例如 `"Read"`，它會在諮詢回呼之前自動批准整個工具

具有指定符的項目（例如 `Bash(ls *)`）和 `acceptEdits` 模式不會觸發它，來自設定檔的允許規則對檢查不可見。

使用 `process.on('warning', ...)` 進行監聽，並匹配代碼以記錄或抑制它。若要無論模式和規則如何都控制每個工具呼叫，請改用 [`PreToolUse` hook](/docs/zh-TW/agent-sdk/hooks)。

本頁重點關注 **允許和拒絕規則** 以及 **權限模式**。對於其他步驟：

* **Hooks：** 執行自訂程式碼以允許、拒絕或修改工具請求。請參閱 [使用 hooks 控制執行](/docs/zh-TW/agent-sdk/hooks)。
* **canUseTool 回呼：** 在執行時提示使用者核准，當沒有較早的步驟解決呼叫時。請參閱 [處理核准和使用者輸入](/docs/zh-TW/agent-sdk/user-input)。

<h2 id="allow-and-deny-rules">
  允許和拒絕規則
</h2>

`allowed_tools` 和 `disallowed_tools`（TypeScript：`allowedTools` / `disallowedTools`）將條目新增到上述評估流程中的允許和拒絕規則清單。允許規則只影響批准：未列在 `allowed_tools` 中的工具仍然可供 Claude 使用，並會通過權限模式。拒絕規則的行為取決於它們是命名工具還是在工具內限定模式。

| 選項                                | 效果                                                                                 |
| :-------------------------------- | :--------------------------------------------------------------------------------- |
| `allowed_tools=["Read", "Grep"]`  | `Read` 和 `Grep` 會自動批准。此處未列出的工具仍然存在，並會通過權限模式和 `canUseTool`。                         |
| `disallowed_tools=["Bash"]`       | `Bash` 工具定義會從請求中移除。Claude 看不到該工具，無法嘗試使用它。                                          |
| `disallowed_tools=["Bash(rm *)"]` | `Bash` 保持可用。符合 `rm *` 的呼叫在每個權限模式中都會被拒絕，包括 `bypassPermissions`。其他 `Bash` 呼叫會通過權限模式。 |
| `disallowed_tools=["*"]`          | 每個工具定義都會從請求中移除。工具名稱萬用字元在拒絕規則中受支援：`"*"` 符合每個工具，`"mcp__*"` 符合所有伺服器上的每個 MCP 工具。       |

允許規則只在字面 `mcp__<server>__` 前綴之後接受工具名稱萬用字元。伺服器段必須不含萬用字元，以便規則命名您設定的特定伺服器：`mcp__puppeteer__*` 符合來自 `puppeteer` 伺服器的每個工具，`mcp__github__get_*` 符合其 `get_` 工具。未錨定的條目（如 `allowed_tools=["*"]` 或 `allowed_tools=["mcp__*"]`）會被忽略並顯示啟動警告，不會自動批准任何內容。

`Read` 和 `Edit` 的限定規則採用路徑模式。`Edit(path)` 規則管理所有寫入檔案的內建工具，包括 `Write` 和 `NotebookEdit`；`Write(path)` 規則永遠不會被檔案權限檢查符合。

使用 `//path` 表示絕對檔案系統路徑：`Edit(//secrets/**)` 的拒絕規則會阻止在磁碟上 `/secrets` 下任何位置的寫入。使用單個前導斜線，`Edit(/secrets/**)` 會在規則的來源處錨定。對於通過 `allowed_tools` 或 `disallowed_tools` 傳遞的規則，這表示工作階段的工作目錄，因此規則不會阻止磁碟上的 `/secrets`。請參閱 [Read 和 Edit 規則](/docs/zh-TW/permissions#read-and-edit) 以了解四種錨定形式以及來自設定檔案的規則如何解析。

<Warning>
  **自動批准的工具永遠不會到達 `canUseTool`。** 在任何較早步驟中批准的工具呼叫，由 `acceptEdits` 或 `bypassPermissions` 或允許規則批准，會跳過您的 `canUseTool` 回呼，因此您在那裡放置的權限檢查會被該工具無聲地略過。`AskUserQuestion`、標記為 [`_meta["anthropic/requiresUserInteraction"]`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 的 MCP 工具，以及連接器工具[您的組織設定為 `ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools) 仍會到達回呼，即使允許規則符合時也是如此。

  涵蓋範圍取決於條目的形式：像 `Read` 或 `mcp__github__get_issue` 這樣的裸名稱會自動批准對該工具的每個呼叫，而像 `Bash(ls *)` 這樣的限定規則只會自動批准符合的呼叫，其他 `Bash` 呼叫仍會通過回呼。對於必須在每個工具呼叫上執行的檢查，請使用 [`PreToolUse` hook](/docs/zh-TW/agent-sdk/hooks)：hook 在每個其他步驟之前執行，hook 拒絕甚至在 `bypassPermissions` 模式中也適用。
</Warning>

對於鎖定的代理程式，將 `allowedTools` 與 `permissionMode: "dontAsk"` 配對。列出的工具會被批准，除了上述警告中的始終提示工具外；其他任何工具都會被直接拒絕，而不是提示：

```typescript theme={null}
const options = {
  allowedTools: ["Read", "Glob", "Grep"],
  permissionMode: "dontAsk"
};
```

<Warning>
  **`allowed_tools` 不會限制 `bypassPermissions`。** `allowed_tools` 只會預先批准您列出的工具。未列出的工具不會被任何允許規則符合，並會通過權限模式，其中 `bypassPermissions` 會批准它們。將 `allowed_tools=["Read"]` 與 `permission_mode="bypassPermissions"` 一起設定仍然會批准每個工具，包括 `Bash`、`Write` 和 `Edit`。如果您需要 `bypassPermissions` 但想要阻止特定工具，請使用 `disallowed_tools`。
</Warning>

您也可以在 `.claude/settings.json` 中宣告式地設定允許、拒絕和詢問規則。當啟用 `project` 設定來源時，這些規則會被讀取，預設 `query()` 選項就是這樣。如果您明確設定 `setting_sources`（TypeScript：`settingSources`），請包含 `"project"` 以便它們適用。請參閱 [權限設定](/docs/zh-TW/settings#permission-settings) 以了解規則語法。

<h2 id="permission-modes">
  權限模式
</h2>

權限模式提供對 Claude 如何使用工具的全域控制。您可以在呼叫 `query()` 時設定權限模式，或在串流會話期間動態更改它。

<h3 id="available-modes">
  可用模式
</h3>

SDK 支援這些權限模式：

| 模式                  | 描述       | 工具行為                                                                                                                                                          |
| :------------------ | :------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `default`           | 標準權限行為   | 無自動批准；不符合的工具會觸發您的 `canUseTool` 回呼                                                                                                                             |
| `dontAsk`           | 拒絕而不是提示  | 任何未被 `allowed_tools` 或規則預先批准的內容都會被拒絕；連接器工具[您的組織設定為 `ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools)和需要使用者互動的工具即使您已預先批准它們也會被拒絕。`canUseTool` 永遠不會被呼叫 |
| `acceptEdits`       | 自動接受檔案編輯 | 檔案編輯和[檔案系統操作](#accept-edits-mode-acceptedits)（`mkdir`、`rm`、`mv` 等）會自動被批准                                                                                      |
| `bypassPermissions` | 繞過權限檢查   | 工具執行時無需權限提示，除了明確的[`ask` 規則](#how-permissions-are-evaluated)符合的工具、連接器工具[您的組織設定為 `ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools)和需要使用者互動的工具（謹慎使用）   |
| `plan`              | 規劃模式     | Claude 在不編輯您的原始檔案的情況下探索和規劃；檔案編輯永遠不會自動批准，並透過您的 `canUseTool` 回呼提示                                                                                               |
| `auto`              | 模型分類批准   | 模型分類器批准或拒絕每個工具呼叫。請參閱 [Auto 模式](/docs/zh-TW/permission-modes#eliminate-prompts-with-auto-mode) 以了解可用性                                                               |

<Warning>
  **子代理程式繼承：** 當父代理程式使用 `bypassPermissions`、`acceptEdits` 或 `auto` 時，所有子代理程式都會繼承該模式，且無法按子代理程式覆蓋。子代理程式可能有不同的系統提示和行為限制較少，因此繼承 `bypassPermissions` 會授予它們完整的自主系統存取權。明確的[`ask` 規則](#how-permissions-are-evaluated)、連接器工具[您的組織設定為 `ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools)和需要使用者互動的工具仍然會強制提示。
</Warning>

<h3 id="set-permission-mode">
  設定權限模式
</h3>

您可以在開始查詢時設定一次權限模式，或在會話活躍時動態更改它。

<Tabs>
  <Tab title="在查詢時">
    在建立查詢時傳遞 `permission_mode`（Python）或 `permissionMode`（TypeScript）。此模式適用於整個會話，除非動態更改。

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import query, ClaudeAgentOptions


      async def main():
          async for message in query(
              prompt="Help me refactor this code",
              options=ClaudeAgentOptions(
                  permission_mode="default",  # Set the mode here
              ),
          ):
              if hasattr(message, "result"):
                  print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      async function main() {
        for await (const message of query({
          prompt: "Help me refactor this code",
          options: {
            permissionMode: "default" // Set the mode here
          }
        })) {
          if ("result" in message) {
            console.log(message.result);
          }
        }
      }

      main();
      ```
    </CodeGroup>
  </Tab>

  <Tab title="在串流期間">
    呼叫 `set_permission_mode()`（Python）或 `setPermissionMode()`（TypeScript）以在會話中途更改模式。新模式會立即對所有後續工具請求生效。這讓您可以從限制性開始，並隨著信任建立而放寬權限，例如在檢查 Claude 的初始方法後切換到 `acceptEdits`。

    <CodeGroup>
      ```python Python theme={null}
      import asyncio
      from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions


      async def main():
          async with ClaudeSDKClient(
              options=ClaudeAgentOptions(
                  permission_mode="default",  # Start in default mode
              )
          ) as client:
              await client.query("Help me refactor this code")

              # Change mode dynamically mid-session
              await client.set_permission_mode("acceptEdits")

              # Process messages with the new permission mode
              async for message in client.receive_response():
                  if hasattr(message, "result"):
                      print(message.result)


      asyncio.run(main())
      ```

      ```typescript TypeScript theme={null}
      import { query } from "@anthropic-ai/claude-agent-sdk";

      async function main() {
        const q = query({
          prompt: "Help me refactor this code",
          options: {
            permissionMode: "default" // Start in default mode
          }
        });

        // Change mode dynamically mid-session
        await q.setPermissionMode("acceptEdits");

        // Process messages with the new permission mode
        for await (const message of q) {
          if ("result" in message) {
            console.log(message.result);
          }
        }
      }

      main();
      ```
    </CodeGroup>
  </Tab>
</Tabs>

<h3 id="mode-details">
  模式詳細資訊
</h3>

<h4 id="accept-edits-mode-acceptedits">
  接受編輯模式（`acceptEdits`）
</h4>

自動批准檔案操作，以便 Claude 可以編輯程式碼而無需提示。其他工具（例如不是檔案系統操作的 Bash 命令）仍然需要正常權限。

**自動批准的操作：**

* 檔案編輯（Edit、Write 工具）
* 檔案系統命令：`mkdir`、`touch`、`rm`、`rmdir`、`mv`、`cp`、`sed`

兩者都只適用於工作目錄或 `additionalDirectories` 內的路徑。該範圍外的路徑和對受保護路徑的寫入仍然會提示。

**使用時機：** 您信任 Claude 的編輯並想要更快的迭代，例如在原型設計期間或在隔離目錄中工作時。

<h4 id="don’t-ask-mode-dontask">
  不詢問模式（`dontAsk`）
</h4>

將任何權限提示轉換為拒絕。由 `allowed_tools`、`settings.json` 允許規則或作為 hook 執行的工具會正常執行。連接器工具[您的組織設定為 `ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools)和需要使用者互動的工具即使允許規則符合也會被拒絕。其他所有內容都會被拒絕，而不呼叫 `canUseTool`。

**使用時機：** 您想要為無頭代理程式提供固定的明確工具表面，並且更喜歡硬拒絕而不是無聲依賴 `canUseTool` 不存在。

<h4 id="bypass-permissions-mode-bypasspermissions">
  繞過權限模式（`bypassPermissions`）
</h4>

自動批准所有工具使用而無需提示。Hooks 仍然執行，如果需要可以阻止操作。

<Warning>
  謹慎使用。Claude 在此模式下具有完整的系統存取權。僅在您信任所有可能操作的受控環境中使用。

  `allowed_tools` 不會限制此模式。每個工具都會被批准，而不僅僅是您列出的工具。拒絕規則（`disallowed_tools`）、明確的 `ask` 規則和 hooks 會在模式檢查之前被評估，仍然可以阻止工具。連接器工具[您的組織設定為 `ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools)和需要使用者互動的工具仍然會透過您的 `canUseTool` 回呼進行。
</Warning>

<h4 id="plan-mode-plan">
  規劃模式（`plan`）
</h4>

Claude 探索程式碼庫並產生計畫而不編輯您的原始檔案。唯讀工具在預設模式下執行。檔案編輯在規劃模式下永遠不會自動批准，即使允許規則符合。它們改為透過您的 `canUseTool` 回呼提示。Claude 可能會使用 `AskUserQuestion` 在最終確定計畫之前澄清需求。請參閱[處理核准和使用者輸入](/docs/zh-TW/agent-sdk/user-input#handle-clarifying-questions)以處理這些提示。

**使用時機：** 您想要 Claude 提出變更建議而不執行它們，例如在程式碼審查期間或當您需要在進行變更之前核准變更時。

<h2 id="related-resources">
  相關資源
</h2>

對於權限評估流程中的其他步驟：

* [處理核准和使用者輸入](/docs/zh-TW/agent-sdk/user-input)：互動式核准提示和澄清問題
* [Hooks 指南](/docs/zh-TW/agent-sdk/hooks)：在代理程式生命週期中的關鍵點執行自訂程式碼
* [權限規則](/docs/zh-TW/settings#permission-settings)：`settings.json` 中的宣告式允許/拒絕規則
