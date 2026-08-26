> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 錯誤參考

> 查詢 Claude Code 執行時錯誤訊息，了解每個錯誤的含義及修復方法。

本頁列出 Claude Code 顯示的執行時錯誤及如何從每個錯誤中恢復，以及當回應似乎有問題但沒有錯誤時要檢查的內容。如需安裝錯誤（例如 `command not found` 或設定期間的 TLS 失敗），請參閱 [Troubleshoot installation and login](/docs/zh-TW/troubleshoot-install)。

這些錯誤和恢復命令適用於 CLI、[Desktop app](/docs/zh-TW/desktop) 和 [Claude Code on the web](/docs/zh-TW/claude-code-on-the-web)，因為這三者都包裝相同的 Claude Code CLI。如需特定表面的問題，請參閱該表面頁面上的疑難排解部分。

<Note>
  Claude Code 呼叫 Claude API 以取得模型回應，因此大多數執行時錯誤對應到基礎 API 錯誤代碼。本頁涵蓋每個錯誤在 Claude Code 中的含義及如何恢復。如需原始 HTTP 狀態代碼定義，請參閱 [Claude Platform error reference](https://platform.claude.com/docs/en/api/errors)。
</Note>

<h2 id="find-your-error">
  尋找您的錯誤
</h2>

將您在終端中看到的訊息與下方的部分相符。

| 訊息                                                                                                 | 部分                                                                                          |
| :------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------ |
| `API Error: 500 Internal server error`                                                             | [Server errors](#api-error-500-internal-server-error)                                       |
| `API Error: Repeated 529 Overloaded errors`                                                        | [Server errors](#api-error-repeated-529-overloaded-errors)                                  |
| `Request timed out`                                                                                | [Server errors](#request-timed-out)，或如果訊息提及您的網際網路連線，則為 [Network](#unable-to-connect-to-api) |
| `Server error mid-response. The response above may be incomplete.`                                 | [Server errors](#the-response-above-may-be-incomplete)                                      |
| `Connection closed mid-response` / `Response stalled mid-stream`                                   | [Server errors](#the-response-above-may-be-incomplete)                                      |
| `<model> is temporarily unavailable, so auto mode cannot determine the safety of...`               | [Server errors](#auto-mode-cannot-determine-the-safety-of-an-action)                        |
| `Auto mode could not evaluate this action and is blocking it for safety`                           | [Server errors](#auto-mode-cannot-determine-the-safety-of-an-action)                        |
| `Auto mode classifier transcript exceeded context window`                                          | [Server errors](#auto-mode-cannot-determine-the-safety-of-an-action)                        |
| `Agent terminated early due to an API error`                                                       | [Server errors](#agent-terminated-early-due-to-an-api-error)                                |
| `You've hit your session limit` / `You've hit your weekly limit`                                   | [Usage limits](#youve-hit-your-session-limit)                                               |
| `Usage credits required for 1M context`                                                            | [Usage limits](#usage-credits-required-for-1m-context)                                      |
| `Server is temporarily limiting requests`                                                          | [Usage limits](#server-is-temporarily-limiting-requests)                                    |
| `Request rejected (429)`                                                                           | [Usage limits](#request-rejected-429)                                                       |
| `Credit balance is too low`                                                                        | [Usage limits](#credit-balance-is-too-low)                                                  |
| `Not logged in · Please run /login`                                                                | [Authentication](#not-logged-in)                                                            |
| `Could not resolve authentication method`                                                          | [Authentication](#could-not-resolve-authentication-method)                                  |
| `Invalid API key`                                                                                  | [Authentication](#invalid-api-key)                                                          |
| `Your apiKeyHelper script is failing`                                                              | [Authentication](#your-apikeyhelper-script-is-failing)                                      |
| `This organization has been disabled`                                                              | [Authentication](#this-organization-has-been-disabled)                                      |
| `Your organization has disabled API key authentication`                                            | [Authentication](#your-organization-has-disabled-api-key-authentication)                    |
| `Your organization has disabled Claude subscription access`                                        | [Authentication](#your-organization-has-disabled-claude-subscription-access)                |
| `Routines are disabled by your organization's policy`                                              | [Authentication](#routines-are-disabled-by-your-organizations-policy)                       |
| `Remote Control is only available when using Claude via api.anthropic.com`                         | [Authentication](#remote-control-requires-the-anthropic-api)                                |
| `OAuth token revoked` / `OAuth token has expired`                                                  | [Authentication](#oauth-token-revoked-or-expired)                                           |
| `Login expired · Please run /login`                                                                | [Authentication](#login-expired)                                                            |
| `Failed to authenticate: OAuth session expired and could not be refreshed`                         | [Authentication](#login-expired)                                                            |
| `does not meet scope requirement user:profile`                                                     | [Authentication](#oauth-scope-requirement)                                                  |
| `AWS credentials expired or invalid`                                                               | [Authentication](#aws-credentials-expired-or-invalid)                                       |
| `AWS authentication failed`                                                                        | [Authentication](#aws-authentication-failed)                                                |
| `AWS default-chain credential resolve timed out`                                                   | [Authentication](#aws-default-chain-credential-resolve-timed-out)                           |
| `Unable to connect to API`                                                                         | [Network](#unable-to-connect-to-api)                                                        |
| `Waiting for API response · will retry in`                                                         | [Automatic retries](#automatic-retries)，或如果持續發生，則為 [Network](#unable-to-connect-to-api)     |
| `Bedrock streaming response has content-type "..."; expected "application/vnd.amazon.eventstream"` | [Network](#bedrock-streaming-response-has-an-unexpected-content-type)                       |
| `SSL certificate verification failed`                                                              | [Network](#ssl-certificate-errors)                                                          |
| `SSL certificate error (...)` during login or startup                                              | [Network](#ssl-certificate-errors)                                                          |
| `403` with `x-deny-reason: host_not_allowed` in a cloud or routine session                         | [Network](#host-not-allowed-in-a-cloud-session)                                             |
| `Couldn't reconnect to your Remote Control session`                                                | [Network](#couldnt-reconnect-to-your-remote-control-session)                                |
| `Prompt is too long`                                                                               | [Request errors](#prompt-is-too-long)                                                       |
| `Error during compaction: Conversation too long`                                                   | [Request errors](#error-during-compaction-conversation-too-long)                            |
| `Request too large`                                                                                | [Request errors](#request-too-large)                                                        |
| `Image was too large`                                                                              | [Request errors](#image-was-too-large)                                                      |
| `Unable to resize image`                                                                           | [Request errors](#unable-to-resize-image)                                                   |
| `PDF too large` / `PDF is password protected`                                                      | [Request errors](#pdf-errors)                                                               |
| `Extra inputs are not permitted`                                                                   | [Request errors](#extra-inputs-are-not-permitted)                                           |
| `There's an issue with the selected model`                                                         | [Request errors](#theres-an-issue-with-the-selected-model)                                  |
| `Model ... is not a recognized model id`                                                           | [Request errors](#model-is-not-a-recognized-model-id)                                       |
| `Claude Opus is not available with the Claude Pro plan`                                            | [Request errors](#claude-opus-is-not-available-with-the-claude-pro-plan)                    |
| `Model ... is restricted by your organization's settings`                                          | [Request errors](#model-is-restricted-by-your-organizations-settings)                       |
| `thinking.type.enabled is not supported for this model`                                            | [Request errors](#thinking-type-enabled-is-not-supported-for-this-model)                    |
| `max_tokens must be greater than thinking.budget_tokens`                                           | [Request errors](#thinking-budget-exceeds-output-limit)                                     |
| `API Error: 400 due to tool use concurrency issues`                                                | [Request errors](#tool-use-or-thinking-block-mismatch)                                      |
| `Claude Code is unable to respond to this request, which appears to violate our Usage Policy`      | [Request errors](#usage-policy-refusal)                                                     |
| `<model> has safety measures that flagged this message for a cybersecurity topic`                  | [Request errors](#safety-measures-flagged-a-cybersecurity-topic)                            |
| `Installation was killed before it could finish (exit code 137)`                                   | [Installation errors](#installation-was-killed-before-it-could-finish)                      |
| `The connection dropped while downloading the update`                                              | [Installation errors](#the-connection-dropped-while-downloading-the-update)                 |
| `Download timed out: exceeded the total deadline`                                                  | [Installation errors](#the-connection-dropped-while-downloading-the-update)                 |
| `--bg and --print conflict`                                                                        | [Command-line errors](#command-line-errors)                                                 |
| `Error: --json-schema is not a valid JSON Schema`                                                  | [Command-line errors](#command-line-errors)                                                 |
| `Could not import <server>: <reason>`                                                              | [Command-line errors](#could-not-import-a-server-from-claude-desktop)                       |
| `Error: MCP tool <name> (passed via --permission-prompt-tool) not found`                           | [Command-line errors](#mcp-permission-prompt-tool-not-found)                                |
| `Marketplace "<name>" is registered from an untrusted source`                                      | [Plugin errors](#marketplace-is-registered-from-an-untrusted-source)                        |
| `references ${user_config.*} in a shell-form command`                                              | [Plugin errors](#plugin-command-references-user-config)                                     |
| `Monitor "<name>" from plugin <plugin> references ${user_config.*} in its command`                 | [Plugin errors](#plugin-command-references-user-config)                                     |
| `headersHelper for MCP server '<name>' references ${user_config.*}`                                | [Plugin errors](#plugin-command-references-user-config)                                     |
| `would be spawned with zero tools — refusing`                                                      | [Tool errors](#agent-would-be-spawned-with-zero-tools)                                      |
| `File is covered by a Read deny rule in your permission settings`                                  | [Tool errors](#file-is-covered-by-a-read-deny-rule)                                         |
| `Can't open MCP settings in a background session`                                                  | [Background session errors](#commands-refused-in-a-background-session)                      |
| `CLAUDE_CODE_PROCESS_WRAPPER: launcher ...`                                                        | [Background session errors](#claude_code_process_wrapper-launcher-errors)                   |
| `Ignoring N permissions.allow entries from ... this workspace has not been trusted`                | [Configuration warnings](#workspace-has-not-been-trusted)                                   |
| 回應品質似乎低於平常                                                                                         | [Response quality](#responses-seem-lower-quality-than-usual)                                |

<h2 id="automatic-retries">
  自動重試
</h2>

Claude Code 在向您顯示錯誤之前會重試暫時性失敗。伺服器錯誤、過載回應、請求逾時、臨時 429 節流和中斷的連線都會以指數退避方式重試最多 10 次。自 v2.1.198 起，這涵蓋在任何可見輸出串流之前在回應中途中斷的連線：Claude Code 使用相同的退避重新發出請求，轉向繼續而不是停止並出現連線錯誤。自 v2.1.199 起，不帶您計畫配額標頭的臨時 429 節流在您使用 claude.ai 訂閱登入時也會重試；較早的版本僅針對 API 金鑰和 Enterprise 登入重試它們。

有些失敗類別不會重試，因為重試無法成功：

* 自 v2.1.199 起，TLS 憑證驗證失敗（例如 TLS 檢查代理、遺失的 `NODE_EXTRA_CA_CERTS` 套件或過期的憑證）在第一次嘗試時失敗，因此修復會立即出現，而不是在完整重試預算之後。請參閱 [SSL 憑證錯誤](#ssl-certificate-errors)。暫時性 TLS 條件（例如握手逾時）仍會重試。
* 自 v2.1.199 起，在 Claude 已經串流可見輸出後到達的伺服器錯誤會保留部分回應並附加 [不完整回應通知](#the-response-above-may-be-incomplete)，而不是重試，因為重新執行請求可能會執行相同的工具兩次。較早的版本會捨棄部分輸出並將轉向報告為錯誤。
* [Amazon Bedrock 串流回應具有非預期的內容類型](#bedrock-streaming-response-has-an-unexpected-content-type)在第一次嘗試時失敗，因為重寫回應的閘道或代理會以相同方式重寫重試。需要 Claude Code v2.1.208 或更新版本。

重試時，微調器會在錯誤標籤後顯示 `Retrying in Ns · attempt x/y` 倒數計時。標籤命名第一次嘗試的特定原因，以便您可以立即採取行動的失敗：網路已關閉、TLS 握手失敗或您達到速率限制。對於其他錯誤，它最初讀取 `API error`。自 v2.1.198 起，它會切換到第三次嘗試的特定原因，或在 `CLAUDE_CODE_MAX_RETRIES` 允許少於三次時的最後一次嘗試；較早的版本僅在最後一次嘗試時切換。

自 v2.1.198 起，通常的微調器提示在重試期間被抑制。一旦錯誤原因被揭示，如果失敗是 529 過載，倒數計時下方的行也會命名檢查服務狀態的位置：Anthropic API 上的 `status.claude.com`，或其他配置上提供者或閘道主機命名的位置。

如果在請求仍待處理時，回應串流上 20 秒內沒有資料到達，微調器會在任何重試開始之前顯示 `Waiting for API response · will retry in … · check your network`。請求尚未失敗：倒數計時會執行到 Claude Code 中止停滯連線並重試的位置，因此一旦資料恢復或重試成功，橫幅就會自動清除。自 v2.1.185 起，閾值為 20 秒；較早的版本會在 10 秒後顯示橫幅，措辭不同。如果它在每次嘗試時都重新出現，請將其視為[網路問題](#unable-to-connect-to-api)。

當您看到本頁上的其中一個錯誤時，這些重試已經用盡，除非它屬於不會重試的類別，例如憑證驗證失敗。您可以使用這些環境變數調整行為：

| 變數                                              | 預設值    | 效果                                                                                                                                                                                                         |
| :---------------------------------------------- | :----- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`CLAUDE_CODE_MAX_RETRIES`](/docs/zh-TW/env-vars)    | 10     | 重試次數。自 v2.1.186 起上限為 15；自 v2.1.199 起 `CLAUDE_CODE_RETRY_WATCHDOG` 提高預設值並移除上限。降低它以在指令碼中更快地顯示失敗。                                                                                                             |
| [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/zh-TW/env-vars) | 未設定    | 在 CI 工作等無人值守的工作階段中設定為 `1`，以無限期重試 `429` 和 `529` 容量錯誤，而不是在 `CLAUDE_CODE_MAX_RETRIES` 次嘗試後失敗。自 v2.1.199 起，它也提高了其他暫時性錯誤（例如伺服器錯誤、逾時和中斷的連線）的預設重試計數至 300，大約三小時的退避，並在您明確設定該變數時移除 `CLAUDE_CODE_MAX_RETRIES` 的上限 15。 |
| [`API_TIMEOUT_MS`](/docs/zh-TW/env-vars)             | 600000 | 每個請求的逾時（毫秒）。為慢速網路或代理提高它。                                                                                                                                                                                   |

<h2 id="server-errors">
  伺服器錯誤
</h2>

這些錯誤來自推論提供者，而非您的帳戶或請求。在 Anthropic API 上，這表示 Anthropic 基礎設施。在 Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 或自訂閘道上，這表示該提供者的基礎設施。

<h3 id="api-error-500-internal-server-error">
  API 錯誤：500 內部伺服器錯誤
</h3>

Claude Code 會顯示任何 5xx 回應的狀態碼和 API 的錯誤訊息。下面的範例顯示 Anthropic API 上的 500 回應：

```text theme={null}
API Error: 500 Internal server error. This is a server-side issue, usually temporary — try again in a moment. If it persists, check https://status.claude.com.
```

結尾的句子會指出要檢查服務健康狀態的位置，並因提供者而異。Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 配置會指出該提供者的服務狀態。自訂 `ANTHROPIC_BASE_URL` 會指出閘道主機。

這表示 API 內部發生了意外故障。它不是由您的提示、設定或帳戶造成的。

**該怎麼做：**

* 檢查 [status.claude.com](https://status.claude.com) 或訊息中指名的提供者狀態頁面，查看是否有活躍的事件
* 等待一分鐘，然後再次傳送您的訊息。您的原始訊息仍在對話中，所以對於較長的提示，您可以輸入 `try again` 而不是貼上整個內容。
* 如果錯誤持續出現且沒有發佈的事件，請執行 `/feedback` 以便 Anthropic 可以使用您的請求詳細資訊進行調查。如果您的環境中無法使用 `/feedback`，請參閱[報告錯誤](#report-an-error)。

<h3 id="api-error-repeated-529-overloaded-errors">
  API 錯誤：重複的 529 超載錯誤
</h3>

API 在所有使用者中暫時達到容量上限。Claude Code 在顯示此訊息之前已經重試了多次：

```text theme={null}
API Error: Repeated 529 Overloaded errors. The API is at capacity — this is usually temporary. Try again in a moment. If it persists, check https://status.claude.com.
```

結尾的句子因提供者而異，方式與上面的 500 錯誤相同。

529 不是您的使用限制，也不會計入您的配額。

**該怎麼做：**

* 檢查 [status.claude.com](https://status.claude.com) 或訊息中指名的提供者狀態頁面，查看是否有容量通知
* 幾分鐘後再試一次
* 執行 `/model` 並切換到不同的模型以繼續工作，因為容量是按模型追蹤的。當某個模型負載特別高時，Claude Code 會提示您執行此操作，例如 `Opus is experiencing high load, please use /model to switch to Sonnet`。

<h3 id="request-timed-out">
  請求逾時
</h3>

API 在連線截止期限之前沒有回應。

```text theme={null}
Request timed out
```

這可能在高負載期間或模型生成非常大的回應時發生。預設請求逾時為 10 分鐘。

**該怎麼做：**

* 重試請求
* 對於長時間執行的任務，將工作分解為較小的提示
* 如果是緩慢的網路或代理造成的，請按照[自動重試](#automatic-retries)中的說明提高 `API_TIMEOUT_MS`
* 如果逾時頻繁且您的網路狀況良好，請參閱下面的[網路和連線錯誤](#network-and-connection-errors)

<h3 id="the-response-above-may-be-incomplete">
  上面的回應可能不完整
</h3>

串流回應在 Claude 已經產生可見輸出後失敗。重新傳送請求可能會執行相同的工具呼叫兩次，所以 Claude Code 會保留已經串流的內容，並改為附加此通知，而不是捨棄該輪次。您看到的變體會指出原因：

```text theme={null}
API Error: Server error mid-response. The response above may be incomplete.
API Error: Connection closed mid-response. The response above may be incomplete.
API Error: Response stalled mid-stream. The response above may be incomplete.
```

* }`Server error mid-response`：中途串流超載或 5xx 伺服器錯誤。此變體需要 Claude Code v2.1.199 或更新版本；在此之前，該情況會捨棄部分輸出並將整個輪次報告為錯誤。
* `Connection closed mid-response`：連線中斷。
* `Response stalled mid-stream`：串流停止傳送資料。

**該怎麼做：**

* 閱讀已串流的回應。沒有任何內容遺失，但最後的句子或工具呼叫可能缺失。
* 回覆 `continue` 以讓 Claude 從停止的地方繼續
* 如果相同的錯誤在任何可見輸出之前出現，Claude Code 會重試請求而不是完成它。請參閱[自動重試](#automatic-retries)。

<h3 id="auto-mode-cannot-determine-the-safety-of-an-action">
  自動模式無法判斷動作的安全性
</h3>

[自動模式](/docs/zh-TW/permission-modes#eliminate-prompts-with-auto-mode)用來分類動作的模型無法做出決定，所以自動模式沒有自動批准該動作。您看到的訊息取決於分類器失敗的原因。

在您的工作目錄內的讀取、搜尋和編輯會跳過分類器，所以它們在所有這些情況下都能繼續工作。

當分類器模型超載時：

```text theme={null}
<model> is temporarily unavailable, so auto mode cannot determine the safety of <tool> right now. Wait briefly and then try this action again.
```

**該怎麼做：**

* 幾秒鐘後重試；Claude 會看到相同的訊息，通常會自動重試
* 如果重試持續失敗，請繼續執行唯讀任務，稍後再回到被阻止的動作
* 這是暫時的，與[自動模式資格](/docs/zh-TW/permission-modes#eliminate-prompts-with-auto-mode)無關；您不需要變更設定

當分類器傳回無法解析的回應時：

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — run with --debug for details
```

**該怎麼做：**

* 重試該動作；這通常在下一次嘗試時成功
* 執行 `claude --debug` 並重複該動作以在偵錯日誌中查看基礎分類器回應

當單獨的 API 安全檢查因為較早的對話內容而阻止了分類器請求時：

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — a safety check separate from auto mode blocked this request because of earlier conversation content — it isn't about the action itself — run with --debug for details
```

**該怎麼做：**

* 這不是關於您的動作的決定。您對話中已有的內容在自動模式將對話傳送給分類器時觸發了 API 上的安全篩選器
* 重試無法幫助；相同的對話內容會再次觸發篩選器
* 切換到不同的[權限模式](/docs/zh-TW/permission-modes)，以便在出現提示時批准該動作，或開始一個沒有觸發內容的新對話

當對話大小超過分類器的上下文視窗時：

```text theme={null}
Auto mode classifier transcript exceeded context window — falling back to manual approval (try /compact to reduce conversation size)
```

在互動式工作階段中，自動模式會為該動作回退到正常的權限提示，以便您可以手動批准或拒絕它。在[非互動式模式](/docs/zh-TW/headless)中，執行會中止，因為文字記錄只會增長，重試無法成功。

**該怎麼做：**

* 在出現的提示中批准或拒絕該動作
* 執行 `/compact` 以減少對話大小，以便後續動作再次適應分類器視窗

<h3 id="agent-terminated-early-due-to-an-api-error">
  代理因 API 錯誤而提前終止
</h3>

[子代理](/docs/zh-TW/sub-agents)的 API 請求終止失敗，例如因為達到使用限制或伺服器錯誤的重試用盡，所以子代理在完成其任務之前停止。此訊息需要 Claude Code v2.1.199 或更新版本；在此之前，API 錯誤文字被傳回給 Claude，就像它是子代理的結果一樣。

```text theme={null}
Agent terminated early due to an API error: <error detail>
```

**該怎麼做：**

* 將冒號後的錯誤詳細資訊與此頁面上的自己的部分相符，例如[使用限制](#usage-limits)或[伺服器錯誤](#server-errors)，並遵循該部分的步驟
* 一旦基礎錯誤清除，請要求 Claude 重試任務或[恢復子代理](/docs/zh-TW/sub-agents#resume-subagents)

當速率限制、超載或伺服器錯誤中斷已經產生文字輸出的前景子代理時，Claude 會收到該部分輸出標記為不完整，而不是此錯誤。只有工具呼叫輸出的子代理也會收到此錯誤；在 v2.1.199 中，該形狀改為傳回空的部分結果。請參閱[子代理中的 API 錯誤](/docs/zh-TW/sub-agents#api-errors-in-subagents)。

<h2 id="usage-limits">
  使用限制
</h2>

這些錯誤表示與您的帳戶或方案相關的配額已達到。它們與[伺服器錯誤](#server-errors)不同，伺服器錯誤會影響所有人。

<h3 id="youve-hit-your-session-limit">
  您已達到工作階段限制
</h3>

訂閱方案包括滾動使用額度。當額度用完時，您會看到以下其中一條訊息：

```text theme={null}
You've hit your session limit · resets 3:45pm
You've hit your weekly limit · resets Mon 12:00am
You've hit your Opus limit · resets 3:45pm
```

Claude Code 會阻止進一步的請求，直到訊息中顯示的重設時間。工作階段和每週限制在所有模型中共享，因此切換模型不會恢復存取。Opus 限制僅適用於 Opus 請求，因此使用 `/model` 切換到另一個模型可讓您繼續工作。

使用額度會同時計入工作階段和每週額度。單一次的大量活動突發，例如大型工作流程扇出，可能會在工作階段視窗重設之前耗盡每週額度。

**該怎麼做：**

* 等待錯誤訊息中顯示的重設時間
* 對於 Opus 限制，執行 `/model` 並切換到另一個模型以繼續工作
* 執行 `/usage` 以查看您的方案限制及其重設時間
* 執行 `/usage-credits` 以在 Pro 和 Max 上購買額外使用額度，或在 Team 和 Enterprise 上向您的管理員請求。請參閱[付費方案的使用額度](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)以了解如何計費。
* 若要升級您的方案以獲得更高的基本限制，請參閱 [claude.com/pricing](https://claude.com/pricing)

若要在達到限制之前監控您的剩餘額度，請將 `rate_limits` 欄位新增至[自訂狀態列](/docs/zh-TW/statusline#rate-limit-usage)，或在桌面應用程式中按一下模型選擇器旁的[使用量環](/docs/zh-TW/desktop#check-usage)。

<h3 id="usage-credits-required-for-1m-context">
  1M 上下文需要使用額度
</h3>

選定的模型使用 1M 令牌擴展上下文視窗，而您的方案僅透過使用額度包含它。

```text theme={null}
API Error: Usage credits required for 1M context · run /usage-credits to turn them on, or /model to switch to standard context
```

這是一項權利檢查，而不是配額耗盡。即使您的工作階段和每週額度仍有容量，它也會觸發。請參閱[擴展上下文](/docs/zh-TW/model-config#extended-context)以了解哪些方案直接包含 1M 上下文，哪些需要使用額度。

當此錯誤在對話中途出現，因為上下文增長超過 200K 令牌時，Claude Code 會自動將對話壓縮回標準上下文限制以下，並在之後將工作階段保持在該限制，因此無需採取任何行動。在 v2.1.172 之前的版本上，錯誤會在每個後續請求（包括 `/compact`）上重複出現；在這些版本上執行 `/clear` 以恢復。以下步驟適用於您明確選擇 `[1m]` 模型的情況。

**該怎麼做：**

* 執行 `/model` 並選擇不帶 `[1m]` 後綴的變體以回退到標準上下文視窗
* 執行 `/usage-credits` 以在 Pro 和 Max 上開啟 1M 變體的計量計費，或在 Team 和 Enterprise 上向您的管理員請求
* 如果 `/model` 後錯誤仍然存在，1M 模型 ID 可能在其他地方設定。請參閱[選定的模型有問題](#theres-an-issue-with-the-selected-model)以按優先順序檢查配置位置。
* 若要從模型選擇器中完全移除 1M 變體，請設定 [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/zh-TW/env-vars)

<h3 id="server-is-temporarily-limiting-requests">
  伺服器暫時限制請求
</h3>

API 應用了與您的方案配額無關的短期節流。

```text theme={null}
API Error: Server is temporarily limiting requests (not your usage limit)
```

Claude Code 透過真實限制回應所攜帶的統一配額標頭的缺失來區分這些與您的方案限制。自 v2.1.199 起，無論您如何驗證，這都會[自動重試](#automatic-retries)並進行退避，然後才會顯示。在較早的版本上，使用 claude.ai 訂閱登入的工作階段在第一次出現時失敗；只有 API 金鑰和 Enterprise 登入會重試它。

**該怎麼做：**

* 稍等片刻後重試
* 如果問題持續，請檢查 [status.claude.com](https://status.claude.com)

<h3 id="request-rejected-429">
  請求被拒絕 (429)
</h3>

您已達到為 API 金鑰、Amazon Bedrock 專案或 Google Cloud 專案配置的速率限制。

```text theme={null}
API Error: Request rejected (429) · this may be a temporary capacity issue. If it persists, check https://status.claude.com.
```

尾部句子命名檢查服務健康狀況的位置，並因提供者而異。Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 配置會命名該提供者的服務狀態，而不是 Anthropic 狀態頁面。自訂 `ANTHROPIC_BASE_URL` 會命名閘道主機。

**該怎麼做：**

* 執行 `/status` 並確認作用中的認證是您預期的認證。環境中的流浪 `ANTHROPIC_API_KEY` 可能會透過低階金鑰而不是您的訂閱來路由請求。
* 檢查您的提供者主控台以了解作用中的限制，並在需要時請求更高的層級
* 對於 Anthropic API 金鑰，請參閱[速率限制參考](https://platform.claude.com/docs/en/api/rate-limits)以了解層級如何運作以及如何設定每個工作區的上限
* 降低並行性：降低 [`CLAUDE_CODE_MAX_TOOL_USE_CONCURRENCY`](/docs/zh-TW/env-vars)、避免執行許多平行子代理，或使用 `/model` 切換到較小的模型以進行大量指令碼執行

<h3 id="credit-balance-is-too-low">
  信用額度餘額過低
</h3>

您的 Console 組織已用完預付信用額度。

```text theme={null}
Credit balance is too low
```

**該怎麼做：**

* 在 [platform.claude.com/settings/billing](https://platform.claude.com/settings/billing) 新增信用額度，並考慮在那裡啟用自動重新載入，以便在餘額達到零之前進行補充
* 如果您有 Pro、Max、Team 或 Enterprise 方案，請使用 `/login` 切換到訂閱驗證
* 在 Console 中設定每個工作區的支出上限，以防止單一專案耗盡組織餘額。請參閱[有效管理成本](/docs/zh-TW/costs)。

<h2 id="authentication-errors">
  驗證錯誤
</h2>

這些錯誤表示 Claude Code 無法向 API 證明您的身份。隨時執行 `/status` 以查看目前使用的認證方式。

<h3 id="not-logged-in">
  未登入
</h3>

此工作階段沒有有效的認證方式可用。

```text theme={null}
Not logged in · Please run /login
```

**應該怎麼做：**

* 執行 `/login` 以使用您的 Claude 訂閱或 Console 帳戶進行驗證
* 如果您預期使用環境變數進行驗證，請確認 `ANTHROPIC_API_KEY` 已在啟動 `claude` 的 shell 中設定並匯出
* 對於無法進行互動式登入的 CI 或自動化環境，請設定一個 [`apiKeyHelper`](/docs/zh-TW/settings#available-settings) 指令碼，在啟動時取得金鑰
* 請參閱[驗證優先順序](/docs/zh-TW/authentication#authentication-precedence)以了解當存在多個認證方式時，Claude Code 使用哪一個

如果系統反覆提示您登入，請參閱[未登入或權杖已過期](/docs/zh-TW/troubleshoot-install#not-logged-in-or-token-expired)以取得系統時鐘和 macOS Keychain 的修復方法。

<h3 id="could-not-resolve-authentication-method">
  無法解析驗證方法
</h3>

工作階段到達 API 用戶端時沒有任何認證方式。這會出現在[背景工作階段](/docs/zh-TW/agent-view)、雲端工作階段和 Agent SDK 環境中，其中互動式登入檢查在第一個請求之前不會執行。

```text theme={null}
Could not resolve authentication method. Expected one of apiKey, authToken, credentials, config, or profile to be set. Or for one of the "X-Api-Key" or "Authorization" headers to be explicitly omitted
```

在 v2.1.174 之前，指派給閒置預初始化背景工作程序的背景或雲端工作階段即使已設定有效認證方式也可能以此方式失敗。請升級以恢復。在目前版本中，此錯誤表示背景工作程序沒有可用的認證方式。

**應該怎麼做：**

* 如果此錯誤出現在背景或雲端工作階段中且您的認證方式已設定，請升級至 v2.1.174 或更新版本
* 確認 `ANTHROPIC_API_KEY`、`CLAUDE_CODE_OAUTH_TOKEN` 或您的雲端提供者認證方式已在啟動背景工作程序的環境中設定，而不僅在您的互動式 shell 中
* 對於 Agent SDK，請參閱[驗證設定](/docs/zh-TW/agent-sdk/overview#get-started)
* 在相同環境中的互動式工作階段中執行 `/status` 以確認哪個認證方式來源可以解析

<h3 id="invalid-api-key">
  無效的 API 金鑰
</h3>

`ANTHROPIC_API_KEY` 環境變數或 `apiKeyHelper` 指令碼傳回的金鑰被 API 拒絕。

```text theme={null}
Invalid API key · Fix external API key
```

**應該怎麼做：**

* 檢查是否有拼寫錯誤，並確認該金鑰未在 [Console](https://platform.claude.com/settings/keys) 中被撤銷
* 在相同的 shell 中執行 `env | grep ANTHROPIC`。direnv、dotenv shell 外掛程式和 IDE 終端等工具可能會從您專案中的 `.env` 檔案載入過時的金鑰，而您並未明確設定它
* 取消設定 `ANTHROPIC_API_KEY` 並執行 `/login` 以改用訂閱驗證
* 如果金鑰來自 [`apiKeyHelper`](/docs/zh-TW/settings#available-settings) 指令碼，請直接執行該指令碼以確認它在 stdout 上列印有效的金鑰
* 執行 `/status` 以確認 Claude Code 實際使用的認證方式來源

<h3 id="your-apikeyhelper-script-is-failing">
  您的 apiKeyHelper 指令碼失敗
</h3>

在 [`apiKeyHelper`](/docs/zh-TW/settings#available-settings) 設定中設定的命令已結束並出現錯誤、逾時或未在 stdout 上列印任何內容。如果沒有來自指令碼的金鑰，請求會到達 API 並使用預留位置認證方式，API 會以 `401` 拒絕它。

```text theme={null}
Your apiKeyHelper script is failing · This usually means you need to re-authenticate with your provider · Run /status to see the script's error output
```

Claude Code 會重新執行指令碼並在顯示此訊息之前最多重試兩次請求，因此失敗會在三次嘗試內出現。在 v2.1.208 之前，Claude Code 花費完整的[重試預算](#automatic-retries)使用預留位置認證方式重新傳送請求，然後報告通用的 `401` 驗證錯誤而不是指令碼失敗。

執行 `/login` 在此無法幫助：只要設定存在，協助程式的輸出[優先於](/docs/zh-TW/authentication#authentication-precedence)已儲存的登入。

**應該怎麼做：**

* 在您的 shell 中直接執行在 `apiKeyHelper` 中設定的命令以重現失敗
* 如果命令報告工作階段已過期，請使用您的認證方式提供者重新驗證，例如再次登入您的 SSO 或機密保管庫
* 修復命令以便它將金鑰列印到 stdout 並以代碼 0 結束。請參閱[使用 apiKeyHelper 輪換認證方式](/docs/zh-TW/llm-gateway-connect#rotate-credentials-with-apikeyhelper)以取得有效的設定。
* 執行 `/status` 以確認 `apiKeyHelper` 是使用中的認證方式來源。每次命令失敗時，其結束代碼和錯誤輸出會出現在終端中的 `Cloud authentication` 面板中。

<h3 id="this-organization-has-been-disabled">
  此組織已被停用
</h3>

來自已停用 Console 組織的過時 `ANTHROPIC_API_KEY` 正在覆蓋您的訂閱登入。

```text theme={null}
Your ANTHROPIC_API_KEY belongs to a disabled organization · Unset the environment variable to use your other credentials
API Error: 400 ... This organization has been disabled.
```

環境變數優先於 `/login`，因此即使您有有效的 Pro 或 Max 訂閱，在 shell 設定檔中匯出或從 `.env` 檔案載入的金鑰也會被使用。在非互動模式 (`-p`) 中，當金鑰存在時總是使用該金鑰。

**應該怎麼做：**

* 在目前 shell 中取消設定 `ANTHROPIC_API_KEY` 並從您的 shell 設定檔中移除它，然後重新啟動 `claude`
* 之後執行 `/status` 以確認使用中的認證方式是您的訂閱
* 如果未設定環境變數且錯誤仍然存在，則已停用的組織是與您的 `/login` 相關聯的組織。請聯絡支援或使用不同的帳戶登入。

<h3 id="your-organization-has-disabled-api-key-authentication">
  您的組織已停用 API 金鑰驗證
</h3>

此訊息需要 Claude Code v2.1.169 或更新版本。您的 Console 組織管理員已關閉 API 金鑰驗證，因此 API 拒絕了 Claude Code 正在傳送的金鑰。`·` 之後的恢復提示會根據金鑰的來源而有所不同：

```text theme={null}
Your organization has disabled API key authentication · Run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY to use your claude.ai account instead
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY and run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset the apiKeyHelper setting and run /login to sign in with your claude.ai account
```

環境變數和 `apiKeyHelper` 優先於 `/login`，因此當其中任一個仍在提供金鑰時，單獨執行 `/login` 無法幫助。請參閱[驗證優先順序](/docs/zh-TW/authentication#authentication-precedence)。

**應該怎麼做：**

* 如果訊息提及 `ANTHROPIC_API_KEY`，請在目前 shell 中取消設定它，並從您的 shell 設定檔或 `.env` 檔案中移除它，然後重新啟動 `claude`
* 如果訊息提及 `apiKeyHelper`，請從您的 `settings.json` 中移除 [`apiKeyHelper`](/docs/zh-TW/settings#available-settings) 設定
* 執行 `/login` 以使用您的 claude.ai 帳戶登入
* 之後執行 `/status` 以確認使用中的認證方式是您的訂閱而不是 API 金鑰
* 如果您需要 API 金鑰驗證進行自動化，請要求您的組織管理員在 Console 中重新啟用它

<h3 id="your-organization-has-disabled-claude-subscription-access">
  您的組織已停用 Claude 訂閱存取
</h3>

您的 Claude 組織不允許使用訂閱登入來登入 Claude Code。使用相同帳戶再次執行 `/login` 會傳回相同的錯誤。

```text theme={null}
Your organization has disabled Claude subscription access for Claude Code · Use an Anthropic API key instead, or ask your admin to enable access
```

這是伺服器端組織設定，因此無法從本機設定、環境變數或 CLI 旗標覆蓋。

Agent SDK 和 `-p` 非互動模式將此顯示為 `oauth_org_not_allowed` 錯誤代碼。

**應該怎麼做：**

* 要求您的管理員為您的組織啟用 Claude Code 存取
* 使用 Console API 金鑰而不是您的訂閱進行驗證。請參閱 [Claude Console 驗證](/docs/zh-TW/authentication#claude-console-authentication)以進行設定。
* 如果您是管理員且看不到啟用存取的選項，請聯絡 [Anthropic 支援](https://support.claude.com)

<h3 id="routines-are-disabled-by-your-organizations-policy">
  例行工作已被您的組織政策停用
</h3>

您的 Team 或 Enterprise 組織中的擁有者已在組織層級關閉例行工作。當您嘗試建立或執行例行工作時（包括從 `/schedule` 和 claude.ai/code 上的[例行工作](/docs/zh-TW/routines) UI），會出現此錯誤。

```text theme={null}
Routines are disabled by your organization's policy.
```

這是伺服器端設定，因此無法從本機設定、環境變數或 CLI 旗標覆蓋。

**應該怎麼做：**

* 要求您的組織中的擁有者在 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) 啟用**例行工作**切換
* 對於不需要組織層級例行工作的一次性排程工作，請參閱[排程工作](/docs/zh-TW/scheduled-tasks)

<h3 id="remote-control-requires-the-anthropic-api">
  Remote Control 需要 Anthropic API
</h3>

工作階段未直接與 Anthropic API 通訊，因此沒有 claude.ai 後端供 [Remote Control](/docs/zh-TW/remote-control) 配對。

```text theme={null}
Remote Control is only available when using Claude via api.anthropic.com.
```

這會出現在 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 上。從 v2.1.196 開始，當 [`ANTHROPIC_BASE_URL`](/docs/zh-TW/env-vars) 指向 `api.anthropic.com` 以外的主機（例如 [LLM 閘道](/docs/zh-TW/llm-gateway)或代理）時，即使您使用 claude.ai 登入，也會出現此訊息。

**應該怎麼做：**

* 取消設定 `ANTHROPIC_BASE_URL` 並重新啟動工作階段，或從直接與 Anthropic API 通訊的工作階段啟動 Remote Control
* 對於此訊息和其他 Remote Control 啟動訊息，請參閱[疑難排解 Remote Control](/docs/zh-TW/remote-control#troubleshooting)

<h3 id="oauth-token-revoked-or-expired">
  OAuth 權杖已撤銷或已過期
</h3>

您儲存的登入不再有效。撤銷的權杖表示您已在所有地方登出或管理員移除了存取；過期的權杖表示自動重新整理在工作階段中途失敗。

兩個訊息都報告 API 為 Claude Code 傳送的請求傳回的拒絕。當已儲存的登入在失敗的重新整理後已被清除時，您會看到[登入已過期](#login-expired)。

```text theme={null}
OAuth token revoked · Please run /login
OAuth token has expired · Please run /login
API Error: 401 ... authentication_error
```

**應該怎麼做：**

* 執行 `/login` 以重新登入
* 如果在同一工作階段中重新驗證後錯誤仍然出現，請先執行 `/logout` 以完全清除儲存的權杖，然後執行 `/login`
* 對於跨啟動的重複登入提示，請參閱[疑難排解](/docs/zh-TW/troubleshoot-install#not-logged-in-or-token-expired)中的系統時鐘和 macOS Keychain 檢查
* 對於其他失敗（包括 `403 Forbidden` 和 OAuth 瀏覽器問題），請參閱[登入和驗證](/docs/zh-TW/troubleshoot-install#login-and-authentication)

<h3 id="login-expired">
  登入已過期
</h3>

Claude Code 嘗試更新您儲存的 claude.ai 或 Claude Console 登入，OAuth 服務拒絕了儲存的重新整理權杖，因此 Claude Code 清除了儲存的認證方式。之後，每個請求在到達 API 之前都會在本機停止，因為只有 `/login` 可以建立新的認證方式。在 v2.1.206 之前，Claude Code 無論如何都會傳送請求，並使用環境中剩餘的任何認證方式，然後每個模型都會失敗並出現[所選模型有問題](#theres-an-issue-with-the-selected-model)或 401 而不是登入提示。

```text theme={null}
Login expired · Please run /login
```

在[非互動模式](/docs/zh-TW/headless)(`-p`) 和 [Agent SDK](/docs/zh-TW/agent-sdk/overview) 中，訊息如下所示，結構化錯誤代碼為 `authentication_failed`：

```text theme={null}
Failed to authenticate: OAuth session expired and could not be refreshed
```

這與[OAuth 權杖已撤銷或已過期](#oauth-token-revoked-or-expired)的狀態不同。這些訊息報告 API 傳回的 401。Claude Code 本身為已失敗更新的登入產生 `Login expired`，因此它不傳送任何請求。

使用 API 金鑰、[`CLAUDE_CODE_OAUTH_TOKEN`](/docs/zh-TW/env-vars) 或第三方提供者驗證的工作階段不使用儲存的登入，永遠不會看到此訊息。

**應該怎麼做：**

* 執行 `/login` 以重新登入。在不登入的情況下重試會在每個請求上顯示相同的訊息。
* 在非互動模式中，在相同環境中執行 `claude`，完成 `/login`，然後重新執行您的命令。對於無法互動式登入的自動化，請使用 `ANTHROPIC_API_KEY` 進行驗證或[使用 `claude setup-token` 產生長期權杖](/docs/zh-TW/authentication#generate-a-long-lived-token)。
* 如果登入持續失敗，請參閱[登入和驗證](/docs/zh-TW/troubleshoot-install#login-and-authentication)

<h3 id="oauth-scope-requirement">
  OAuth 範圍要求
</h3>

儲存的權杖早於較新功能所需的權限範圍。您最常從 `/usage` 和狀態列使用量指示器看到此訊息：

```text theme={null}
OAuth token does not meet scope requirement: user:profile
```

**應該怎麼做：**

* 執行 `/login` 以取得具有目前範圍的新權杖。您不需要先登出。

<h3 id="aws-credentials-expired-or-invalid">
  AWS 認證方式已過期或無效
</h3>

此訊息需要 Claude Code v2.1.198 或更新版本，且僅在您的設定檔中設定了 [`awsAuthRefresh`](/docs/zh-TW/amazon-bedrock#advanced-credential-configuration) 時出現。您的 AWS 工作階段權杖已過期或被拒絕，Claude Code 已執行的自動重新整理未產生 API 接受的認證方式。它會出現在來自 [Claude Platform on AWS](/docs/zh-TW/claude-platform-on-aws) 或 [Mantle 端點](/docs/zh-TW/amazon-bedrock#use-the-mantle-endpoint) 的 401 上，這是這些提供者報告過期安全權杖的方式。

中間的動作提示會命名您設定中的 `awsAuthRefresh` 命令，因此會有所不同。穩定的部分是前導的 `AWS credentials expired or invalid`：

```text theme={null}
AWS credentials expired or invalid · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · API Error: 401 ...
```

如果未設定 `awsAuthRefresh`，相同的 401 會改為顯示通用的 `Please run /login` 訊息，該訊息無法重新整理 AWS 認證方式。

**應該怎麼做：**

* 在另一個終端中執行訊息中命名的 `awsAuthRefresh` 命令（例如 `aws sso login --profile myprofile`）並完成瀏覽器登入，然後重試
* 在互動式工作階段中，執行 `/login`，選擇 **3rd-party platform**，然後在 **Using 3rd-party platforms** 下選擇 **Claude Platform on AWS · refresh credentials** 以執行相同的命令而無需重新啟動 Claude Code。請參閱[設定 AWS 認證方式](/docs/zh-TW/claude-platform-on-aws#1-configure-aws-credentials)
* 如果重新整理命令成功後錯誤仍然重複出現，請在相同的 shell 和設定檔中使用 `aws sts get-caller-identity` 確認身份在 Claude Code 外部有效

<h3 id="aws-authentication-failed">
  AWS 驗證失敗
</h3>

此訊息需要 Claude Code v2.1.198 或更新版本，且僅在您的設定檔中設定了 [`awsAuthRefresh`](/docs/zh-TW/amazon-bedrock#advanced-credential-configuration) 時出現。您的 AWS 提供者傳回了 403，或 [Amazon Bedrock](/docs/zh-TW/amazon-bedrock) 傳回了 401。

Claude Code 無法判斷您遇到了哪個原因。Amazon Bedrock 將過期的安全權杖報告為 403，但 403 也是它報告授權拒絕的方式，例如來自遺失 IAM 權限或未為您的帳戶啟用的模型的 `AccessDeniedException`。

來自 Amazon Bedrock 的 401 也會落在此處而不是在 [AWS 認證方式已過期或無效](#aws-credentials-expired-or-invalid) 下，因為 Amazon Bedrock 不會將過期的權杖報告為 401。來自該端點的 401 通常來自請求路徑中的其他內容，例如公司代理。

認證方式重新整理可以修復過期的權杖，無法修復其他原因，因此訊息提供了兩者：

```text theme={null}
AWS authentication failed · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · if credentials are current, check AWS permissions and model access · API Error: 403 ...
```

中間的動作提示會命名您設定中的 `awsAuthRefresh` 命令，因此會有所不同。穩定的部分是前導的 `AWS authentication failed`。

**應該怎麼做：**

* 執行訊息中命名的 `awsAuthRefresh` 命令或 `aws sso login`，以防過期的認證方式是原因
* 如果您的認證方式是最新的，請確認 [IAM 配置](/docs/zh-TW/amazon-bedrock#iam-configuration) 中的 IAM 權限已附加到您使用的身份，且所選模型已為您的帳戶和區域啟用
* 執行 `aws sts get-caller-identity` 以確認您的請求使用哪個身份；過時的 `AWS_PROFILE` 或預設設定檔是權限不匹配的常見原因

<h3 id="aws-default-chain-credential-resolve-timed-out">
  AWS 預設鏈認證方式解析逾時
</h3>

AWS 預設認證方式提供者鏈在 60 秒內未產生認證方式，因此 Claude Code 停止了解析並使請求失敗。失敗是本機認證方式解析：請求永遠未到達 [Amazon Bedrock](/docs/zh-TW/amazon-bedrock)、[Claude Platform on AWS](/docs/zh-TW/claude-platform-on-aws) 或 [Mantle 端點](/docs/zh-TW/amazon-bedrock#use-the-mantle-endpoint)。Claude Code 在此錯誤出現之前會清除其[認證方式快取](/docs/zh-TW/amazon-bedrock#credential-caching-and-resolution-timeout)並在重複嘗試後重試，因此當您看到它時鏈已在重複嘗試上停滯。

```text theme={null}
API Error: AWS default-chain credential resolve timed out
```

常見原因是您的 AWS 設定檔中的 `credential_process` 命令等待它無法接收的輸入，以及容器或 VM 的執行個體中繼資料服務 (IMDS) 永遠不會回答鏈的探測。在 v2.1.207 之前，停滯的鏈會讓請求無限期等待，而不是以此訊息失敗。

**應該怎麼做：**

* 在相同的 shell 中使用相同的 `AWS_PROFILE` 執行 `aws sts get-caller-identity`。如果它也掛起，請修復設定檔；互動式提示的 `credential_process` 命令是常見原因。
* 在啟動 Claude Code 之前完成登入步驟，例如 `aws sso login --profile myprofile`，以便鏈從本機 SSO 快取解析而不是等待瀏覽器流程
* 如果您的鏈執行合法需要超過 60 秒的互動式登入，例如透過 `aws-vault` 等包裝程式的 SSO 搭配 MFA，請使用 [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/zh-TW/env-vars) 以毫秒為單位提高限制

<h2 id="network-and-connection-errors">
  網路和連線錯誤
</h2>

這些錯誤表示來自 Claude Code 的網路請求無法到達其目的地，或 Claude Code 和 API 之間的某些東西在回程中改變了回應。它們通常源自您的本機網路、代理伺服器或防火牆，或雲端環境的網路政策。

<h3 id="unable-to-connect-to-api">
  無法連線到 API
</h3>

與 API 的 TCP 連線失敗或從未完成。

```text theme={null}
Unable to connect to API. Check your internet connection
Unable to connect to API (ECONNREFUSED)
Unable to connect to API (ECONNRESET)
Unable to connect to API (ETIMEDOUT)
fetch failed
Request timed out. Check your internet connection and proxy settings
```

常見原因包括沒有網際網路存取、阻止 `api.anthropic.com` 的 VPN，或未設定的必要公司代理伺服器。

**該怎麼做：**

* 透過在同一個 shell 中執行 `curl -I https://api.anthropic.com` 來確認您可以到達 API 主機。在 Windows PowerShell 上使用 `curl.exe -I https://api.anthropic.com`，以免使用內建的 `Invoke-WebRequest` 別名。
* 如果您在公司代理伺服器後面，請在啟動 Claude Code 前設定 `HTTPS_PROXY`，並參閱[網路設定](/docs/zh-TW/network-config)
* 如果您透過 LLM 閘道或中繼站路由，請將 [`ANTHROPIC_BASE_URL`](/docs/zh-TW/env-vars) 設定為其位址。請參閱[將 Claude Code 連線到 LLM 閘道](/docs/zh-TW/llm-gateway-connect)以取得設定說明。
* 確保您的防火牆允許[網路存取需求](/docs/zh-TW/network-config#network-access-requirements)中列出的主機
* 間歇性故障會[自動重試](#automatic-retries)；持續性故障指向本機網路問題

如果 `curl` 成功但 Claude Code 仍然失敗，原因通常是執行時間和網路之間的某些東西，而不是網路本身：

* 在 Linux 和 WSL 上，檢查 `/etc/resolv.conf` 是否有無法到達的名稱伺服器。特別是 WSL 可能會從主機繼承損壞的解析器。
* 在 macOS 上，已斷開連線或卸載的 VPN 用戶端可能會留下隧道介面或路由規則。檢查 `ifconfig` 是否有過時的 `utun` 介面，並在系統設定中移除 VPN 的網路擴充功能。
* Docker Desktop 和類似的容器執行時間可能會攔截出站流量。結束它們並重試以排除此可能性。

<h3 id="bedrock-streaming-response-has-an-unexpected-content-type">
  Bedrock 串流回應有非預期的 content-type
</h3>

Claude Code 和 [Amazon Bedrock](/docs/zh-TW/amazon-bedrock) 之間的閘道或代理伺服器正在轉換串流回應本體或其 `Content-Type` 標頭。Amazon Bedrock 將回應串流為 `application/vnd.amazon.eventstream`，而 Claude Code 會拒絕報告不同 content-type 的成功串流回應，而不是解碼它無法讀取的本體。該請求不會重試。

```text theme={null}
Bedrock streaming response has content-type "text/event-stream"; expected "application/vnd.amazon.eventstream". A gateway or proxy between Claude Code and Bedrock is likely transforming the response body — Bedrock's binary event-stream format must be passed through unmodified. Set CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1 to suppress this check while the gateway is being fixed.
```

在 v2.1.208 之前，相同的設定錯誤會在整個回應被緩衝後顯示為 `API Error: Truncated event message received`。

**該怎麼做：**

* 設定閘道以不修改地傳遞 `InvokeModelWithResponseStream` 回應本體及其 `Content-Type` 標頭。將串流重新發出為伺服器傳送事件的中介是常見原因。
* 如果閘道只重寫標頭並完整傳遞二進位本體，請設定 [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1`](/docs/zh-TW/env-vars) 以在閘道修復前跳過檢查。請參閱[閘道或代理伺服器後的串流錯誤](/docs/zh-TW/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy)。

<h3 id="ssl-certificate-errors">
  SSL 憑證錯誤
</h3>

您網路上的代理伺服器或安全設備正在用其自己的憑證攔截 TLS 流量，而 Claude Code 不信任它。

```text theme={null}
Unable to connect to API: SSL certificate verification failed. Check your proxy or corporate SSL certificates
Unable to connect to API: Self-signed certificate detected
```

自 v2.1.199 起，憑證驗證失敗不會重試，因此此錯誤會在第一次嘗試時出現，而不是在完整[重試預算](#automatic-retries)後出現。較早的版本在顯示它之前會花費幾分鐘重試。暫時性 TLS 條件（例如握手逾時）仍會重試。

在 `/login` 和啟動連線檢查期間，同樣的失敗會以 OpenSSL 代碼和內聯修復報告：

```text theme={null}
SSL certificate error (UNABLE_TO_GET_ISSUER_CERT_LOCALLY). If you are behind a corporate proxy or TLS-intercepting firewall, set NODE_EXTRA_CA_CERTS to your CA bundle path, or ask IT to allowlist *.anthropic.com. Run `claude doctor` for details.
```

**該怎麼做：**

* 匯出您組織的 CA 套件，並使用 `NODE_EXTRA_CA_CERTS=/path/to/ca-bundle.pem` 將 Claude Code 指向它
* 請參閱[網路設定](/docs/zh-TW/network-config#custom-ca-certificates)以取得完整設定說明
* 不要設定 `NODE_TLS_REJECT_UNAUTHORIZED=0`，這會完全停用憑證驗證

<h3 id="host-not-allowed-in-a-cloud-session">
  雲端工作階段中不允許的主機
</h3>

來自雲端工作階段或例行程序的出站 HTTP 請求被環境的網路政策阻止。

```text theme={null}
HTTP 403
x-deny-reason: host_not_allowed
```

您也可能看到與目的地實際憑證不符的 TLS 憑證。雲端環境透過代理伺服器路由出站流量以強制執行網路政策，因此不符的憑證表示代理伺服器終止了連線，而不是目的地。

這不是用戶端網路問題。雲端工作階段和[例行程序](/docs/zh-TW/routines)在沙箱環境內執行，其出站流量被篩選到環境的允許清單。**預設**環境使用**信任**存取，允許[預設允許清單](/docs/zh-TW/claude-code-on-the-web#default-allowed-domains)的套件登錄、雲端提供者 API、容器登錄和常見開發網域，但阻止其他所有內容。

**該怎麼做：**

* 開啟例行程序進行編輯，或啟動雲端工作階段。選擇顯示您環境名稱（例如**預設**）的雲端圖示以開啟選擇器。將滑鼠懸停在您的環境上，然後按一下設定圖示。
* 在**更新雲端環境**對話方塊中，將**網路存取**從**信任**變更為**自訂**，然後將被阻止的網域新增到**允許的網域**。每行輸入一個網域。勾選**也包含常見套件管理員的預設清單**以在自訂網域旁保留[預設允許清單](/docs/zh-TW/claude-code-on-the-web#default-allowed-domains)。如果您想要不受限制的存取，請改為選擇**完整**。
* 按一下**儲存變更**。下一次執行會使用更新的允許清單。

請參閱[網路存取](/docs/zh-TW/claude-code-on-the-web#network-access)以取得存取層級和預設允許清單。本機 CLI 工作階段不受此政策影響。

<h3 id="couldnt-reconnect-to-your-remote-control-session">
  無法重新連線到您的遠端控制工作階段
</h3>

```text theme={null}
Couldn't reconnect to your Remote Control session. Retry, or start a fresh session without --resume.
```

使用 `claude --resume` 或 `claude --continue` 恢復會重新連線到該對話中記錄的[遠端控制](/docs/zh-TW/remote-control)工作階段。此訊息表示重新連線因可能是暫時性的原因（例如網路中斷或伺服器錯誤）而失敗，因此 Claude Code 無法確認遠端工作階段是否仍然存在。您的本機工作階段會繼續執行，但不使用遠端控制。

**該怎麼做：**

* 執行 `/remote-control` 以重試連線
* 啟動 Claude Code 時不使用 `--resume` 以建立新的遠端控制工作階段
* 如需其他遠端控制啟動訊息，請參閱[遠端控制疑難排解](/docs/zh-TW/remote-control#troubleshooting)

當伺服器確認前一個工作階段不再存在時，您不會看到此訊息；Claude Code 在這種情況下會建立一個新的工作階段。在 v2.1.200 之前，任何重新連線失敗都會建立新的遠端控制工作階段，這在 claude.ai/code 的工作階段清單中留下額外的工作階段。

<h2 id="request-errors">
  請求錯誤
</h2>

這些錯誤與您的請求內容有關。大多數來自 API 在拒絕請求後的回應；少數是由 Claude Code 在發送任何請求之前在本地產生的。

<h3 id="prompt-is-too-long">
  提示詞過長
</h3>

對話加上附加檔案超過了模型的上下文視窗。

```text theme={null}
Prompt is too long
```

**該怎麼做：**

* 執行 `/compact` 來總結早期的回合並釋放空間，或執行 `/clear` 來重新開始
* 執行 `/context` 來查看視窗消耗的詳細分解：系統提示詞、工具、記憶檔案和訊息
* 使用 `/mcp disable <name>` 停用您未使用的 MCP 伺服器，以從上下文中移除其工具定義
* 修剪大型 `CLAUDE.md` 記憶檔案，或將指令移至[路徑範圍規則](/docs/zh-TW/memory#path-specific-rules)，這些規則只在相關時載入
* 子代理繼承父工作階段中的每個 MCP 工具定義，這可能會在第一個回合之前填滿其上下文視窗。在生成子代理之前停用您未使用的 MCP 伺服器。
* 自動壓縮預設為開啟，通常可防止此錯誤。如果您已設定 [`DISABLE_AUTO_COMPACT`](/docs/zh-TW/env-vars)，請重新啟用它或在視窗填滿之前手動執行 `/compact`。

請參閱[探索上下文視窗](/docs/zh-TW/context-window)以取得上下文如何填滿的互動式檢視。

<h3 id="error-during-compaction-conversation-too-long">
  壓縮期間出錯：對話過長
</h3>

`/compact` 本身失敗，因為沒有足夠的可用上下文來保存它產生的摘要。

```text theme={null}
Error during compaction: Conversation too long. Press esc twice to go up a few messages and try again.
```

當視窗在自動壓縮觸發時已滿，或當您在看到 `Prompt is too long` 後執行 `/compact` 時，可能會發生這種情況。

**該怎麼做：**

* 按 Esc 兩次以開啟訊息清單並回溯幾個回合。這會從上下文中移除最近的訊息。然後再次執行 `/compact`。
* 如果回溯沒有釋放足夠的空間，執行 `/clear` 以開始新的工作階段。您之前的對話會被保留，可以使用 `/resume` 重新開啟。

<h3 id="request-too-large">
  請求過大
</h3>

原始請求主體在標記化之前超過了 API 的位元組限制，通常是因為貼上了大型檔案或附件。

```text theme={null}
Request too large (max 30 MB). Double press esc to go back and remove or shrink the attached content.
```

這是 HTTP 請求的大小限制，與[上下文視窗限制](#prompt-is-too-long)分開。

**該怎麼做：**

* 按 Esc 兩次並回溯到添加超大內容的回合之前
* 按路徑參考大型檔案而不是貼上其內容，以便 Claude 可以分塊讀取它們
* 對於影像，請參閱下面的[影像過大](#image-was-too-large)

<h3 id="image-was-too-large">
  影像過大
</h3>

貼上或附加的影像超過了 API 的大小或尺寸限制。

```text theme={null}
Image was too large. Double press esc to go back and try again with a smaller image.
API Error: 400 ... image dimensions exceed max allowed size
```

Claude Code 將無法處理的影像替換為文字佔位符並重試，因此後續訊息會成功。在 2.1.142 之前的版本上，貼上的影像可能會保留在對話中，並在每個後續訊息上重複相同的錯誤。若要在這些版本上恢復，請按 Esc 兩次並回溯到添加影像的回合之前。

**該怎麼做：**

* 在貼上之前調整影像大小。API 接受單個影像最長邊最多 8000 像素的影像，或當許多影像在上下文中時為 2000 像素。
* 拍攝相關區域的更緊密螢幕截圖，而不是整個螢幕

<h3 id="unable-to-resize-image">
  無法調整影像大小
</h3>

Claude Code 無法在將附加影像發送到 API 之前將其縮小。

```text theme={null}
Unable to resize image — image processing is unavailable and dimensions could not be read from the file header. Please convert the image to PNG, JPEG, GIF, or WebP.
Unable to resize image — dimensions exceed the 2000x2000px limit and image processing failed. Please resize the image to reduce its pixel dimensions.
Unable to resize image (… raw, … base64). The image exceeds the … API limit and compression failed. Please resize the image manually or use a smaller image.
Unable to resize image — could not verify image dimensions are within the 2000x2000px API limit.
```

Claude Code 通常會自動調整大型影像的大小。這些錯誤意味著原生影像處理器無法載入或返回錯誤，因此無法調整影像大小以符合 API 限制。

**該怎麼做：**

* 如果訊息要求您轉換影像，請將其轉換為 PNG、JPEG、GIF 或 WebP，然後再次附加。Claude Code 可以驗證這些格式的尺寸，無需影像處理器。
* 如果訊息報告尺寸或大小限制，請在附加之前將影像調整或重新壓縮到該限制以下。

<h3 id="pdf-errors">
  PDF 錯誤
</h3>

您附加的 PDF 無法處理。

```text theme={null}
PDF too large (max 100 pages, 32 MB). Try splitting it or extracting text first.
PDF is password protected. Try removing protection or extracting text first.
The PDF file was not valid. Try converting to a different format first.
```

**該怎麼做：**

* 對於超大 PDF，請要求 Claude 使用 Read 工具讀取頁面範圍，而不是附加整個檔案，或使用 `pdftotext` 之類的工具提取文字並按路徑參考輸出檔案
* 對於受保護或無效的 PDF，移除密碼或從其源應用程式重新匯出檔案，然後重試

<h3 id="extra-inputs-are-not-permitted">
  不允許額外輸入
</h3>

Claude Code 和 API 之間的代理或 LLM 閘道移除了 `anthropic-beta` 請求標頭，因此 API 拒絕了依賴它的欄位。

```text theme={null}
API Error: 400 ... Extra inputs are not permitted ... context_management
API Error: 400 ... Extra inputs are not permitted ... tools.0.custom.input_examples
API Error: 400 ... Unexpected value(s) for the `anthropic-beta` header
```

Claude Code 發送測試版專用欄位，例如 `context_management`、`effort` 和工具 `input_examples`，以及啟用它們的 `anthropic-beta` 標頭。當閘道轉發主體但移除標頭時，API 會看到它不認識的欄位。

**該怎麼做：**

* 配置您的閘道以轉發 `anthropic-beta` 標頭。請參閱[功能傳遞](/docs/zh-TW/llm-gateway-protocol#feature-pass-through)以了解閘道必須轉發的內容。
* 作為備選方案，在啟動前設定 [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/zh-TW/env-vars)。這會停用需要測試版標頭的功能，以便請求通過無法轉發它的閘道成功。

<h3 id="theres-an-issue-with-the-selected-model">
  選定的模型有問題
</h3>

配置的模型名稱未被識別，或您的帳戶缺乏對其的存取權限。從 v2.1.160 開始，尾部提示（此處以其互動形式顯示）因表面而異。

```text theme={null}
There's an issue with the selected model (claude-...). It may not exist or you may not have access to it. Run /model to pick a different model.
```

**該怎麼做：**

* **互動式 CLI**：執行 `/model` 以從您帳戶可用的模型中選擇。
* **非互動模式 (`-p`)**：使用有效的別名或 ID 傳遞 `--model`，或設定 [`ANTHROPIC_MODEL`](/docs/zh-TW/env-vars)。錯誤文字在此表面上顯示 `Run --model`。
* **Agent SDK**：錯誤文字省略提示，因為模型是以程式設計方式設定的。在 TypeScript 中設定 [`Options` 上的 `model`](/docs/zh-TW/agent-sdk/typescript#options)，或在 Python 中設定 [`ClaudeAgentOptions(model=...)`](/docs/zh-TW/agent-sdk/python#claudeagentoptions)，並處理結構化的 `model_not_found` 錯誤以呈現您自己的重試或模型選擇器。
* 使用別名（例如 `sonnet` 或 `opus`）而不是完整的版本化 ID。別名解析為維護的預設值，因此不會過時。請參閱[模型配置](/docs/zh-TW/model-config)。
* 如果 CLI 中一直出現錯誤的模型，則某處設定了過時的 ID。按[優先順序](/docs/zh-TW/model-config#setting-your-model)檢查：`--model` 標誌、`ANTHROPIC_MODEL` 環境變數，然後是 `.claude/settings.local.json` 中的 `model` 欄位、您專案的 `.claude/settings.json` 和 `~/.claude/settings.json`。移除過時的值，Claude Code 會回退到您的帳戶預設值。
* Claude Code 將過期的 claude.ai 登入報告為[登入已過期](#login-expired)，而不是此錯誤。在 v2.1.206 之前，無法再刷新的過期登入在每個模型上都失敗，出現此錯誤；如果您在較舊版本上看到此情況，請執行 `/login`。
* 對於 Google Cloud 的 Agent Platform 部署，請參閱 [Google Cloud 的 Agent Platform 故障排除](/docs/zh-TW/google-vertex-ai#troubleshooting)。

<h3 id="model-is-not-a-recognized-model-id">
  模型不是公認的模型 ID
</h3>

您傳遞給模型切換的模型字串不是模型別名、此 Claude Code 版本知道的模型 ID，也不是以 `claude-` 開頭的 ID。常見原因是 ID 中的拼寫錯誤、顯示名稱（例如 `Sonnet 5`，其中需要 ID `claude-sonnet-5`），或只有較新 Claude Code 版本識別的別名。Claude Code 立即拒絕切換。在 v2.1.200 之前，Claude Code 會儲存字串並在下一個請求時失敗，出現[選定的模型有問題](#theres-an-issue-with-the-selected-model)。

```text theme={null}
Model "claud-sonnet-5" is not a recognized model id. Did you mean 'claude-sonnet-5'?
```

尾部提示命名最接近的匹配別名或模型 ID。當沒有足夠接近的內容時，它會改為讀取 `Run /model to see available models.`。

Claude Code 在請求切換時在本地產生此錯誤，在發出任何 API 請求之前。它適用於通過 [Agent SDK](/docs/zh-TW/agent-sdk/typescript) `setModel()` 方法或為您執行 Claude Code CLI 的應用程式（例如 [Desktop 應用程式](/docs/zh-TW/desktop)）設定模型的情況。

**該怎麼做：**

* 執行不帶引數的 `/model` 以開啟選擇器並從您帳戶可用的模型中選擇，然後傳遞那裡顯示的別名或 ID
* 如果您使用了較新 Claude Code 版本支援的別名，請執行 `claude update`。以 `claude-` 開頭的完整 ID 即使模型比您的 Claude Code 版本更新，也會通過此檢查，因此不需要升級。
* v2.1.200 之前儲存的模型不會被此檢查修復。如果過時的值一直出現，請從[選定的模型有問題](#theres-an-issue-with-the-selected-model)下列出的位置移除它。
* 檢查僅在 Anthropic API 上執行。在 Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry、[AWS 上的 Claude Platform](/docs/zh-TW/claude-platform-on-aws) 和 [LLM 閘道](/docs/zh-TW/llm-gateway)後面或自訂 `ANTHROPIC_BASE_URL`，您的提供者或閘道定義模型名稱，因此 Claude Code 接受任何字串並將其傳遞。

<h3 id="claude-opus-is-not-available-with-the-claude-pro-plan">
  Claude Opus 不適用於 Claude Pro 方案
</h3>

您的有效訂閱方案不包括您選擇的模型。

```text theme={null}
Claude Opus is not available with the Claude Pro plan · Select a different model in /model
```

**該怎麼做：**

* 執行 `/model` 並選擇您的方案包括的模型
* 如果您最近升級了方案但仍然看到此訊息，請執行 `/logout` 然後 `/login`。儲存的令牌反映您登入時的方案，因此在現有工作階段中在網路上升級不會生效，直到您重新驗證。
* 請參閱 [claude.com/pricing](https://claude.com/pricing) 以了解每個方案包括哪些模型

<h3 id="model-is-restricted-by-your-organizations-settings">
  模型受您組織的設定限制
</h3>

您的組織管理員已在 claude.ai 管理控制台中停用此模型，或它被託管設定中的 [`availableModels`](/docs/zh-TW/model-config#restrict-model-selection) 允許清單排除。當使用 `--model`、`ANTHROPIC_MODEL` 或 `model` 設定設定受限制的模型時，Claude Code 會替換為允許的模型並繼續。為受限制的模型鍵入 `/model <name>` 會被拒絕，顯示 `Run /model to choose a different model.`，工作階段保持其目前模型。

```text theme={null}
Model "claude-opus-4-8" is restricted by your organization's settings. Using claude-sonnet-4-6 instead.
```

Claude Code 將模型系列別名（`opus`、`sonnet`、`haiku` 或 `fable` 之一）視為對該系列的請求，而不是對其最新版本的請求。在 Anthropic API 和 [AWS 上的 Claude Platform](/docs/zh-TW/claude-platform-on-aws) 上，受限制的系列別名解析為您的組織和 `availableModels` 允許清單允許的系列的最新版本，替換通知命名該版本。Claude Code 僅在系列的每個版本都受限制時才拒絕 `/model <alias>`。在 v2.1.205 之前，系列別名是根據其最新版本單獨替換或拒絕的，即使同一系列的較舊版本被允許。

**該怎麼做：**

* 執行 `/model` 以從您的組織允許的模型中選擇。受限制的模型在選擇器中隱藏。
* 如果受限制的模型是在 `--model`、`ANTHROPIC_MODEL` 或設定檔案的 `model` 欄位中設定的，請移除或更新該值，以便通知不會在每次啟動時重複出現
* 如果您需要存取受限制的模型，請要求您的組織管理員啟用它。請參閱[組織模型限制](/docs/zh-TW/model-config#organization-model-restrictions)。

<h3 id="thinking-type-enabled-is-not-supported-for-this-model">
  此模型不支援 thinking.type.enabled
</h3>

您的 Claude Code 版本比 Sonnet 5、Opus 4.8 或 Opus 4.7 的最低版本更舊。CLI 發送了模型不再接受的思考配置。

```text theme={null}
API Error: 400 ... "thinking.type.enabled" is not supported for this model. Use "thinking.type.adaptive" and "output_config.effort" to control thinking behavior.
```

**該怎麼做：**

* 執行 `claude update` 並重新啟動 Claude Code。Opus 4.7 需要 v2.1.111 或更新版本。Opus 4.8 需要 v2.1.154 或更新版本。Sonnet 5 需要 v2.1.197 或更新版本
* 如果您無法升級，請執行 `/model` 並改為選擇 Opus 4.6 或 Sonnet 4.6
* 如果您在 [Agent SDK](/docs/zh-TW/agent-sdk/overview) 中遇到此問題，請改為升級 SDK 套件。Opus 4.8 需要 TypeScript SDK v0.3.154 或更新版本和 Python SDK v0.2.88 或更新版本。Sonnet 5 需要 TypeScript SDK v0.3.197 或更新版本

<h3 id="thinking-budget-exceeds-output-limit">
  思考預算超過輸出限制
</h3>

配置的擴展思考預算超過最大回應長度，因此沒有空間留給實際答案。

```text theme={null}
API Error: 400 ... max_tokens must be greater than thinking.budget_tokens
```

Claude Code 在 Anthropic API 上自動調整這些值。當 [`MAX_THINKING_TOKENS`](/docs/zh-TW/env-vars) 設定高於提供者的輸出限制時，或當計畫模式提高思考預算時，您通常會在 Amazon Bedrock 或 Google Cloud 的 Agent Platform 上看到此錯誤。

**該怎麼做：**

* 降低 `MAX_THINKING_TOKENS`，或將 [`CLAUDE_CODE_MAX_OUTPUT_TOKENS`](/docs/zh-TW/env-vars) 提高到思考預算之上
* 請參閱[擴展思考](/docs/zh-TW/model-config#extended-thinking)以了解預算如何與輸出長度互動

<h3 id="tool-use-or-thinking-block-mismatch">
  工具使用或思考區塊不匹配
</h3>

對話歷史以不一致的狀態到達 API，通常是在工具呼叫被中斷或回合在中途被編輯後。

```text theme={null}
API Error: 400 due to tool use concurrency issues. Run /rewind to recover the conversation.
API Error: 400 ... unexpected `tool_use_id` found in `tool_result` blocks
API Error: 400 ... thinking blocks ... cannot be modified
```

所有三個變體都意味著同一件事：歷史中 `tool_use`、`tool_result` 和 `thinking` 區塊的序列不再與 API 期望的相符。

**該怎麼做：**

* 如果您使用的是 Opus 4.7 或 Opus 4.8，請先執行 `claude update`。v2.1.156 之前的版本可能在正常工具使用期間觸發此錯誤，而 `/rewind` 不會清除它。
* 執行 `/rewind` 或按 Esc 兩次，以回溯到損壞回合之前的檢查點並從那裡繼續。請參閱[檢查點](/docs/zh-TW/checkpointing)以了解如何建立和恢復檢查點。

<h3 id="usage-policy-refusal">
  使用政策拒絕
</h3>

API 拒絕回應，因為對話中的內容觸發了[使用政策](https://www.anthropic.com/legal/aup)檢查。訊息包括您可以引用給支援的請求 ID，如果您認為拒絕不正確。

```text theme={null}
API Error: Claude Code is unable to respond to this request, which appears to violate our Usage Policy (https://www.anthropic.com/legal/aup). Please double press esc to edit your last message or start a new session for Claude Code to assist with a different task.
```

檢查評估完整對話，而不僅是您的最新提示，因此在同一工作階段中發送新訊息通常會重新觸發相同的拒絕。在使用 `--continue` 或 `--resume` 退出並重新開啟工作階段後也是如此，因為磁碟上的文字記錄仍然包含觸發內容。在 [Amazon Bedrock](/docs/zh-TW/amazon-bedrock)、[Google Cloud 的 Agent Platform](/docs/zh-TW/google-vertex-ai) 和 [Microsoft Foundry](/docs/zh-TW/microsoft-foundry) 上，此訊息也涵蓋模型的安全措施標記為網路安全主題的請求。請參閱[安全措施標記了網路安全主題](#safety-measures-flagged-a-cybersecurity-topic)。

**該怎麼做：**

* 按 Esc 兩次或執行 `/rewind` 以回溯到觸發拒絕的回合之前的檢查點，然後重新表述或採取不同的方法。請參閱[檢查點](/docs/zh-TW/checkpointing)。
* 如果您無法識別哪個回合導致了它，請執行 `/clear` 以在同一專案中開始新的對話。您之前的對話會保留在磁碟上，並在 `/resume` 中保持可用。
* 在[非互動模式](/docs/zh-TW/headless)(`-p`) 中，其中無法進行倒帶，請在沒有 `--continue` 的新工作階段中使用重新表述的提示重試。政策檢查因模型而異，因此使用 `--model` 切換到不同的模型也可能在某些情況下解決拒絕。

<h3 id="safety-measures-flagged-a-cybersecurity-topic">
  安全措施標記了網路安全主題
</h3>

模型的安全措施將對話中的內容標記為網路安全主題。訊息命名標記請求的模型：

```text theme={null}
API Error: Opus 4.8 has safety measures that flagged this message for a cybersecurity topic. To learn about the Cyber Verification Program and apply for access, visit our help center: https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude.

If you were not engaging in a cybersecurity topic, please send feedback via /feedback.
```

訊息連結到[網路安全驗證計畫](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude)，該計畫為合法網路安全工作授予存取權限。保護措施本身是伺服器端的，早於 v2.1.203；此版本僅更改了訊息的措辭和它連結到的頁面。

您看到的內容取決於您的提供者和模式：

* 在 [Amazon Bedrock](/docs/zh-TW/amazon-bedrock)、[Google Cloud 的 Agent Platform](/docs/zh-TW/google-vertex-ai) 和 [Microsoft Foundry](/docs/zh-TW/microsoft-foundry) 上，網路安全標記會產生[使用政策拒絕](#usage-policy-refusal)訊息。
* [非互動模式](/docs/zh-TW/headless)省略 `/feedback` 句子。

在 v2.1.203 之前，訊息讀取 `<model>'s safeguards flagged this message for a cybersecurity topic. If your work requires this access, you can apply for an exemption:` 後跟豁免表單連結。

**該怎麼做：**

* 如果您的工作需要此內容，請通過[網路安全驗證計畫](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude)申請存取權限
* 如果您的請求不是關於網路安全主題，請執行 `/feedback` 以報告誤報
* 若要在同一工作階段中繼續工作，請按 Esc 兩次或執行 `/rewind` 以回溯到觸發標記的回合之前的檢查點，然後採取不同的方法。請參閱[檢查點](/docs/zh-TW/checkpointing)。

<h2 id="installation-errors">
  安裝錯誤
</h2>

這些錯誤會在安裝或更新 Claude Code 時出現，來自 [安裝指令碼](/docs/zh-TW/setup#install-claude-code)、`claude install` 或 `claude update`。如需 `command not found`、PATH、權限和設定期間的 TLS 問題，請參閱 [疑難排解安裝和登入](/docs/zh-TW/troubleshoot-install)。

<h3 id="installation-was-killed-before-it-could-finish">
  安裝在完成前被中止
</h3>

當 `claude install` 步驟被信號終止時，安裝指令碼會報告。在 Linux 上，結束代碼 137 表示程序收到 SIGKILL，在低記憶體主機上，通常是核心記憶體不足 (OOM) 殺手。指令碼會列印此說明並以代碼 137 結束：

```text theme={null}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

對於任何其他致命信號，以及 macOS 上的結束代碼 137，指令碼會列印 `Installation was killed before it could finish (exit code <N>)`，其中包含實際結束代碼，並省略記憶體不足的說明。該訊息來自 macOS 和 Linux 使用的安裝指令碼，也涵蓋 WSL 內的安裝；原生 Windows 安裝指令碼永遠不會列印它。在 v2.1.200 之前，指令碼只以 shell 的裸 `Killed` 行結束。

**該怎麼做：**

* 停止其他程序以釋放記憶體，然後重新執行安裝程式
* 新增交換空間或移至更大的執行個體。請參閱 [在低記憶體 Linux 伺服器上安裝被中止](/docs/zh-TW/troubleshoot-install#install-killed-on-low-memory-linux-servers) 以取得交換檔案命令。

<h3 id="the-connection-dropped-while-downloading-the-update">
  下載更新時連線中斷
</h3>

當 `claude install`、`claude update` 或 [自動更新程式](/docs/zh-TW/setup#auto-updates) 正在擷取 Claude Code 二進位檔案時，與下載伺服器的連線已關閉，且重試未能恢復。當連線中斷、傳輸停滯或下載的檔案未通過校驗和時，Claude Code 會重試下載，最多嘗試三次。已完成的 HTTP 錯誤（例如 404）不會重試，因為伺服器已經回應。在 v2.1.202 之前，單一連線中斷會立即導致下載失敗，並顯示裸錯誤 `aborted`，而不是重試。

```text theme={null}
The connection dropped while downloading the update (attempt 3/3: aborted). Check your network — proxies sometimes cut off large downloads.
```

括號中的文字命名失敗的嘗試和基礎網路錯誤。`claude update` 在 stderr 上以 `Error: Failed to install native update` 開頭該訊息。

保持連線但在 10 分鐘內未完成的下載會失敗，並顯示 `Download timed out: exceeded the total deadline`。Claude Code 不會重試逾時的下載，因為連線速度太慢而無法在期限內完成，在立即重試時也不會完成。下面的步驟適用於兩個訊息。在 v2.1.205 之前，相同的 10 分鐘期限被報告為 HTTP 用戶端的通用 `timeout of 600000ms exceeded`。

通常的原因是代理或閘道在傳輸完成前關閉長傳輸。Claude Code 二進位檔案是大型下載，因此永遠不會影響正常 API 流量的代理連線限制仍然可能中斷它。

**該怎麼做：**

* 再次執行 `claude update`。在網路狀況良好的情況下，下載通常在下次執行時成功。對於逾時訊息，請從更快或限制較少的網路重新執行。
* 如果您的網路需要代理，請在執行安裝程式或 `claude update` 之前設定 `HTTPS_PROXY`。請參閱 [檢查網路連線](/docs/zh-TW/troubleshoot-install#check-network-connectivity)。
* 如果公司代理持續關閉傳輸，請要求您的網路團隊允許從 `downloads.claude.ai` 進行完整下載。請參閱 [網路存取需求](/docs/zh-TW/network-config#network-access-requirements)。
* 從您的 shell 執行 `claude doctor` 以進行安裝診斷

<h2 id="command-line-errors">
  命令列錯誤
</h2>

這些錯誤來自 `claude` 命令列及其子命令。Claude Code 在執行您的提示或傳送任何 API 請求之前會列印這些錯誤。

<h3 id="conflict-between-bg-and-print">
  \--bg 和 --print 之間的衝突
</h3>

此訊息需要 Claude Code v2.1.198 或更新版本。您在同一個 `claude` 呼叫中結合了 `--bg` 與 `-p` 或 `--print`。`--bg` 啟動一個[背景工作階段](/docs/zh-TW/agent-view#from-your-shell)，您稍後可以使用 `claude agents` 附加到該工作階段，而 `--print` 以[非互動模式](/docs/zh-TW/headless)執行，永遠不會啟動 `claude agents` 附加到的互動工作階段。在 v2.1.198 之前，此組合會無聲地建立一個永遠無法附加的背景工作。

```text theme={null}
--bg and --print conflict: --print never starts the interactive session that `claude agents` attaches to, so the job would be unattachable. The prompt is the positional — drop --print: `claude --bg '<task>'`.
```

**該怎麼做：**

* 移除 `-p` 或 `--print`。`--bg` 將提示作為其位置引數，所以 `claude --bg "<task>"` 是完整的命令。請參閱[從您的 shell 分派新代理](/docs/zh-TW/agent-view#from-your-shell)。
* 若要以非互動模式執行提示並列印結果而不是建立背景工作階段，請移除 `--bg` 並執行 `claude -p "<task>"`

<h3 id="the-json-schema-value-is-not-a-valid-json-schema">
  \--json-schema 值不是有效的 JSON Schema
</h3>

您傳遞給[`--json-schema`](/docs/zh-TW/cli-reference#cli-flags)的結構描述在[非互動模式](/docs/zh-TW/headless#get-structured-output)中未能通過 JSON Schema 編譯，所以 `claude` 以代碼 1 結束而不是執行提示。在 v2.1.205 之前，無效的結構描述會產生無結構的輸出且沒有錯誤，任何使用 `format` 關鍵字的結構描述都被視為無效。

```text theme={null}
Error: --json-schema is not a valid JSON Schema: data/type must be equal to one of the allowed values
```

第二個冒號之後的文字是驗證器的診斷，並命名失敗的關鍵字或位置。使用 `format` 關鍵字的結構描述（例如 `"format": "email"`）是有效的：Claude Code 接受 `format` 作為註解，不強制執行它。

Claude Code 在結構描述編譯之前執行兩項檢查：它拒絕不可解析的 JSON 值並顯示 `Error: --json-schema is not valid JSON`，以及有效但不是物件的 JSON 並顯示 `Error: --json-schema must be a JSON object`。

**該怎麼做：**

* 修復診斷命名的結構描述部分，然後重新執行命令
* 如果診斷是 `schema too large`，請減少結構描述的巢狀和 `$ref` 重複使用
* 請參閱[取得結構化輸出](/docs/zh-TW/headless#get-structured-output)以取得有效的結構描述和命令

<h3 id="could-not-import-a-server-from-claude-desktop">
  無法從 Claude Desktop 匯入伺服器
</h3>

Claude Code 無法新增您在 `claude mcp add-from-claude-desktop` 中選擇的其中一個伺服器。該命令仍會匯入其他選定的伺服器，並為每個無法新增的伺服器列印一行。在 v2.1.205 之前，第一個失敗的伺服器會停止匯入，且沒有選定的伺服器被新增。

```text theme={null}
Could not import my server: Invalid name my server. Names can only contain letters, numbers, hyphens, and underscores.
```

伺服器名稱之後的文字是原因。最常見的是名稱檢查：Claude Desktop 允許伺服器名稱中的字元（例如空格和句號），而 `claude mcp` 限制為字母、數字、連字號和底線。其他原因包括無法通過驗證的伺服器配置，以及被您組織的 [MCP 原則](/docs/zh-TW/managed-mcp)阻止的伺服器。

**該怎麼做：**

* 在 `claude_desktop_config.json` 中重新命名伺服器，僅使用字母、數字、連字號和底線，然後再次執行 `claude mcp add-from-claude-desktop`
* 使用 `claude mcp add` 或 `claude mcp add-json` 在有效名稱下直接新增該伺服器。請參閱[從 Claude Desktop 匯入 MCP 伺服器](/docs/zh-TW/mcp#import-mcp-servers-from-claude-desktop)。

<h3 id="mcp-permission-prompt-tool-not-found">
  找不到 MCP 權限提示工具
</h3>

您傳遞給 [`--permission-prompt-tool`](/docs/zh-TW/cli-reference#cli-flags) 的工具在執行首次需要權限決定時不在連接的 MCP 工具中，原因可能是其伺服器從未連接，或者沒有連接的伺服器公開該名稱的工具。Claude Code 仍會傳送您的提示：[非互動](/docs/zh-TW/headless)執行在第一個需要批准的工具呼叫時以此錯誤和結束代碼 1 結束，因此即使請求已發出也不會產生答案。在第一個提示之前，Claude Code 會等待最多由 [`MCP_TIMEOUT`](/docs/zh-TW/env-vars) 設定的每個伺服器連接逾時 30 秒，以便該伺服器連接。在 v2.1.206 之前，啟動不會等待伺服器完成連接，所以啟動緩慢但健康的伺服器也會產生此錯誤。

```text theme={null}
Error: MCP tool mcp__permissions__approve (passed via --permission-prompt-tool) not found. Available MCP tools: none
```

`Available MCP tools:` 之後的清單命名了在等待結束時連接的 MCP 工具。

**該怎麼做：**

* 檢查伺服器是否啟動並保持連接：在同一目錄中執行 `claude mcp list`，並確認伺服器列為已連接
* 確認工具名稱與伺服器公開的 `mcp__<server>__<tool>` 名稱相符
* 如果伺服器需要超過 30 秒才能啟動，請提高 [`MCP_TIMEOUT`](/docs/zh-TW/env-vars)

<h2 id="plugin-errors">
  外掛程式錯誤
</h2>

這些錯誤來自 [外掛程式](/docs/zh-TW/plugins) 和 [市集](/docs/zh-TW/plugin-marketplaces) 設定。對於不會產生此頁面上其中一則訊息的外掛程式問題，例如無法載入的市集 URL 或已安裝但未出現的外掛程式，請參閱 [外掛程式疑難排解](/docs/zh-TW/discover-plugins#troubleshooting)。

<h3 id="marketplace-is-registered-from-an-untrusted-source">
  市集是從不受信任的來源註冊的
</h3>

市集是以 [為官方 Anthropic 市集保留的名稱](/docs/zh-TW/plugin-marketplaces#marketplace-schema) 註冊的，但其註冊的來源不是 `anthropics` GitHub 儲存庫。Claude Code 每次載入或重新整理市集時都會重新檢查保留的名稱，因此市集和從中安裝的外掛程式會停止載入。在 v2.1.205 之前，只有在新增市集時才會檢查名稱，因此在名稱變成保留名稱之前註冊的項目會繼續載入。

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

**該怎麼做：**

* 執行 `claude plugin marketplace remove <name>`，然後從官方 `github.com/anthropics` 儲存庫重新新增市集
* 如果您發佈了在名稱變成保留名稱之前使用該名稱的第三方市集，請重新命名它並要求使用者從您的來源重新新增它
* 請參閱 [市集結構描述](/docs/zh-TW/plugin-marketplaces#marketplace-schema) 下的保留名稱清單

<h3 id="plugin-command-references-user-config">
  外掛程式命令在 shell 命令中參考 user\_config
</h3>

外掛程式 hook、[monitor](/docs/zh-TW/plugins-reference#monitors) 或 MCP [`headersHelper`](/docs/zh-TW/mcp#use-dynamic-headers-for-custom-authentication) 命令參考 `${user_config.KEY}` [外掛程式選項](/docs/zh-TW/plugins-reference#user-configuration)，而替換後的字串會被傳遞到 shell。設定的值包含 `$(...)` 、反引號或 `;` 會在該處作為程式碼執行，因此 Claude Code 拒絕啟動元件而不是替換該值。檢查在命令範本上執行，因此即使尚未設定任何值，錯誤也會出現。在 v2.1.207 之前，該值被替換到 shell 命令中。

措辭取決於哪個介面參考了該選項。shell 形式的 hook 會報告：

```text theme={null}
Hook from plugin formatter@acme-tools references ${user_config.*} in a shell-form command. The substituted value would be re-parsed by the shell. Use exec form instead — {"command": "<executable>", "args": ["${user_config.KEY}", ...]} — or read $CLAUDE_PLUGIN_OPTION_<KEY> from the hook's environment. Command: ./scripts/notify.sh ${user_config.webhook_url}
```

monitor 會報告：

```text theme={null}
Monitor "deploy-status" from plugin deploy-tools references ${user_config.*} in its command. The substituted value would be passed to a shell. Monitor commands cannot safely reference ${user_config.*}; have the monitor script read the value from a config file or prompt instead.
```

MCP `headersHelper` 會報告：

```text theme={null}
headersHelper for MCP server 'internal-api' references ${user_config.*}. The substituted value would be passed to a shell; read the value inside the helper script instead (e.g. from an env var set in the server's "env" block).
```

**該怎麼做：**

* 對於 hook，新增 `args` 陣列使其以 [exec 形式](/docs/zh-TW/hooks#exec-form-and-shell-form) 執行，其中每個 `${user_config.KEY}` 變成一個引數，中間沒有 shell。或者移除參考並在指令碼內讀取 `$CLAUDE_PLUGIN_OPTION_<KEY>` 環境變數
* 對於 monitor，移除參考並讓 monitor 指令碼從設定檔讀取該值
* 對於 `headersHelper`，將 `${user_config.KEY}` 移到伺服器的 `headers` 欄位（不會進行 shell 解析），或在 helper 指令碼內讀取該值

<h2 id="tool-errors">
  工具錯誤
</h2>

這些錯誤來自 Claude 的內建工具拒絕輸入。Claude 會自動修正大多數工具錯誤；以下兩個需要您進行變更，因為它們來自您控制的子代理定義或權限規則。

<h3 id="agent-would-be-spawned-with-zero-tools">
  Agent would be spawned with zero tools
</h3>

[子代理的 `tools` 清單](/docs/zh-TW/sub-agents#supported-frontmatter-fields)中沒有任何內容解析為工具，因此 Claude Code 拒絕啟動子代理，而不是啟動無法執行操作的代理。該訊息按它們未解析的原因對條目進行分組：未被識別的工具、不適用於子代理的工具，或已識別但與目前工作階段中的任何工具都不匹配。省略 `tools` 欄位永遠不會觸發此拒絕。MCP 伺服器模式（例如 `mcp__github__*`）不在豁免範圍內：當該伺服器沒有連接的工具時，啟動會被拒絕，並在不匹配的群組中顯示該模式。在 v2.1.208 之前，子代理會以零個工具啟動並返回空的或令人困惑的結果。

```text theme={null}
Agent 'code-reviewer' would be spawned with zero tools — refusing. Its tools list resolved to nothing: unrecognized [Grpe]. Fix the agent's tools frontmatter or pass a different subagent_type.
```

**應該怎麼做：**

* 根據[子代理可用的工具](/docs/zh-TW/sub-agents#available-tools)更正錯誤命名的每個條目
* 移除工作階段沒有的工具條目，例如來自未連接伺服器的 MCP 工具
* 若要讓子代理擁有父代理的所有工具，請刪除 `tools` 欄位，而不是列出工具

<h3 id="file-is-covered-by-a-read-deny-rule">
  File is covered by a Read deny rule
</h3>

Edit 工具在與 [`Read` 拒絕規則](/docs/zh-TW/permissions#read-and-edit)相符的路徑上被呼叫，包括在該路徑建立新檔案。編輯會重寫 Claude 必須能夠讀回的內容，因此呼叫在任何檔案存取之前被拒絕。該規則僅阻止 Edit 工具：Write 和 NotebookEdit 不受 `Read` 拒絕規則涵蓋。在 v2.1.208 之前，只有 `Edit` 拒絕規則會阻止編輯，而 `Read` 拒絕規則單獨不會。

```text theme={null}
File is covered by a Read deny rule in your permission settings and cannot be edited.
```

**應該怎麼做：**

* 如果 Claude 應該能夠編輯該檔案，請在 `/permissions` 或[設定](/docs/zh-TW/settings#permission-settings)中移除或縮小 `Read` 拒絕規則
* 如果檔案必須保持未觸及狀態，請保留該規則並為相同路徑新增 `Edit` 拒絕規則，以便 Write 和 NotebookEdit 工具也被阻止

<h2 id="background-session-errors">
  背景工作階段錯誤
</h2>

[背景工作階段](/docs/zh-TW/agent-view)在沒有互動式終端的情況下執行，因此需要終端的命令在那裡的行為會有所不同。這些訊息會出現在背景工作階段的文字記錄中，在代理檢視中或附加後。

<h3 id="commands-refused-in-a-background-session">
  背景工作階段中被拒絕的命令
</h3>

在背景工作階段中，開啟互動式對話框的命令會被拒絕，並顯示一條訊息，說明在該處有效的表單或告訴您從常規終端執行命令。`/install-github-app`、`/mcp` 設定清單和 MCP 伺服器選單中的驗證操作都以這種方式被拒絕。在 v2.1.208 之前，它們在背景工作階段內開啟了對話框。
在 v2.1.208 中，`/model` 選擇器也在背景工作階段中被拒絕，`/upgrade` 列印升級 URL 而不是開啟瀏覽器。

措辭會說明被拒絕的命令。`/mcp` 設定清單報告：

```text theme={null}
Can't open MCP settings in a background session — use `/mcp enable|disable|reconnect <server>` to steer, or run /mcp from an interactive terminal to authenticate.
```

**該怎麼做：**

* 使用訊息所說的表單，例如 `/mcp reconnect <server>`、`/mcp enable` 或 `/mcp disable`
* 對於登入和授權流程，請從終端中的常規 `claude` 工作階段執行命令

<h3 id="claude_code_process_wrapper-launcher-errors">
  CLAUDE\_CODE\_PROCESS\_WRAPPER 啟動器錯誤
</h3>

[`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/zh-TW/corporate-launcher) 已設定，但其值無法使用，因此 Claude Code 拒絕啟動受影響的程序，而不是在沒有啟動器的情況下執行它。配置問題會報告為以變數名稱開頭並說明原因的訊息，例如：

```text theme={null}
CLAUDE_CODE_PROCESS_WRAPPER: launcher `/opt/corp/launcher` is not an executable regular file
```

啟動但退出而不用 Claude Code 替換自身的啟動器會導致它啟動的工作階段失敗，該工作階段在代理檢視中的列會報告啟動器 `must exec, not daemonize`，後面跟著啟動器列印的任何內容。由於啟動器而無法啟動或無法到達背景服務的工作階段會將啟動器問題報告為 `Couldn't reach the background service (...)` 內的原因。

**該怎麼做：**

* 將變數設定為可執行檔的絕對路徑，該路徑以呼叫 `exec "$@"` 結尾。請參閱[啟動器合約](/docs/zh-TW/corporate-launcher#the-launcher-contract)以了解完整合約
* 檢查 `/status`，它在其 Self-exec 項目中顯示已解析的啟動命令，並在執行中的背景服務不符合時發出警告，或從 shell 執行 `claude daemon status`
* 在修復 [settings](/docs/zh-TW/corporate-launcher#set-up-the-launcher) 的 `env` 區塊中的值後，使用 `claude daemon stop --any` 重新啟動背景服務，以便下一次分派啟動包裝的服務

<h2 id="configuration-warnings">
  設定警告
</h2>

Claude Code 在啟動時將這些訊息寫入 stderr，而不是在對話中顯示錯誤。它們報告 Claude Code 讀取但未應用的設定。

<h3 id="workspace-has-not-been-trusted">
  工作區尚未受信任
</h3>

Claude Code 在專案的 `.claude/settings.json` 或 `.claude/settings.local.json` 中找到了 `permissions.allow` 規則或 `permissions.additionalDirectories` 項目，但未應用它們，因為[來自專案設定的允許規則需要工作區信任](/docs/zh-TW/permissions#project-allow-rules-and-workspace-trust)。訊息中的計數、設定名稱和檔案名稱會根據您的設定而變化。`deny` 和 `ask` 規則不受影響。

```text theme={null}
Ignoring 2 permissions.allow entries from .claude/settings.local.json: this workspace has not been trusted. Run Claude Code interactively here once and accept the trust dialog, or set projects["/Users/you/project"].hasTrustDialogAccepted: true in /Users/you/.claude.json.
```

**該怎麼做：**

* 在目錄中執行 `claude` 並接受信任對話框。即使父目錄已經受信任，對話框仍會出現，列出被保留的規則，並讓您可以拒絕並繼續工作而不使用這些規則。在 v2.1.200 之前，在這種情況下不會出現對話框，因此無法在那裡完成此步驟。
* 在[非互動模式](/docs/zh-TW/headless)中使用 `-p` 時不會顯示對話框。使用訊息列印的確切 `projects` 金鑰在 `~/.claude.json` 中設定 `hasTrustDialogAccepted` 項目。
* 如果訊息命名 `.claude/settings.local.json` 且您在 git 儲存庫外或在主目錄中啟動 Claude Code，請更新至 v2.1.200 或更新版本。版本 2.1.196 至 2.1.199 在這些工作區中將您自己的 `.claude/settings.local.json` 視為儲存庫提供的。在 v2.1.207 及更新版本上，如果您尚未信任該資料夾，在 git 儲存庫外更新是不夠的：判斷資料夾是否在儲存庫內會執行 git，而 Claude Code 只在您接受信任對話框後才執行該檢查，因此請使用第一步。您的主目錄和任何其他[設定主目錄](/docs/zh-TW/permissions#project-allow-rules-and-workspace-trust)都被豁免，不需要等待對話框。請參閱[專案允許規則和工作區信任](/docs/zh-TW/permissions#project-allow-rules-and-workspace-trust)。

<h2 id="responses-seem-lower-quality-than-usual">
  回應品質似乎低於預期
</h2>

如果 Claude 的回答似乎不如您預期的那樣有能力，但沒有顯示錯誤，原因通常是對話狀態而非模型本身。Claude Code 不會無聲地更改模型版本。它只能在三種特定情況下切換到備用模型：

* 配置的 [`--fallback-model`](/docs/zh-TW/cli-reference#cli-flags) 在可用性錯誤後接管該輪次，並在文字記錄中顯示通知
* Amazon Bedrock 或 Google Cloud 的 Agent Platform 啟動檢查發現您的預設模型不可用
* [自動模型備用](/docs/zh-TW/model-config#automatic-model-fallback)在 Fable 5 上將工作階段移至預設 Opus 模型，並在文字記錄中顯示通知

下面的模型選擇檢查可捕捉第二和第三種情況；第一種情況顯示為文字記錄通知而非 `/model` 變更。[模型配置](/docs/zh-TW/model-config)說明每個備用何時適用。

首先檢查這些項目：

* **模型選擇**：執行 `/model` 以確認您使用的是預期的模型。先前的 `/model` 選擇或 `ANTHROPIC_MODEL` 環境變數可能使您使用的模型比預期的要小。
* **努力程度**：執行 `/effort` 以檢查目前的推理級別，並針對困難的除錯或設計工作提高它。預設值因模型而異，因此在假設您低於最大值之前請先檢查。請參閱[調整努力程度](/docs/zh-TW/model-config#adjust-effort-level)以了解每個模型的預設值和 `ultrathink` 快捷方式。
* **上下文壓力**：執行 `/context` 以查看視窗的滿度。如果接近容量，請在自然中斷點執行 `/compact` 或執行 `/clear` 以重新開始。請參閱[探索上下文視窗](/docs/zh-TW/context-window)以了解自動壓縮如何影響較早的輪次。
* **過時的指示**：大型或過時的 `CLAUDE.md` 檔案和 MCP 工具定義會消耗上下文，並可能引導回應。`/doctor` 檢查會標記超大記憶體檔案和未使用的擴充功能，而 `/context` 會顯示 MCP 工具令牌使用情況。在 v2.1.205 之前，`/doctor` 開啟診斷畫面，標記超大記憶體檔案和子代理定義。

當回應出錯時，回溯通常比用更正回覆效果更好。按 Esc 兩次或執行 `/rewind` 以回到不良輪次之前，然後用更具體的內容重新表述提示。在執行緒中更正會將錯誤的嘗試保留在上下文中，這可能會將後續答案錨定到它。請參閱[檢查點](/docs/zh-TW/checkpointing)。

如果在檢查上述項目後品質仍然似乎不對，請執行 `/feedback` 並描述您預期的內容與您得到的內容。以這種方式提交的回饋包括對話文字記錄，這是 Anthropic 診斷真實回歸的最快方式。如果 `/feedback` 在您的環境中不可用，請參閱[報告錯誤](#report-an-error)。

如果 Claude 警告懷疑提示注入，或因懷疑注入而拒絕請求，而警告命名的文字是 Claude Code 自動添加到對話中的上下文而非檔案或網路內容，請執行 `claude update` 並重試。如果更新後警告重複出現，請[報告它](#report-an-error)而不是將標記的內容貼回提示中。在 v2.1.201 之前，Sonnet 5 以相同方式拒絕了某些請求。

<h2 id="report-an-error">
  回報錯誤
</h2>

如需了解此頁面未涵蓋的元件錯誤，請參閱相關指南：

* MCP 伺服器連線或驗證失敗：[MCP](/docs/zh-TW/mcp)
* Hook 指令碼失敗或阻止了工具：[Debug hooks](/docs/zh-TW/hooks#debug-hooks)
* 安裝期間權限被拒或檔案系統錯誤：[Troubleshoot installation and login](/docs/zh-TW/troubleshoot-install)

如果此處未列出錯誤或建議的修正方法無法幫助：

* 在 Claude Code 內執行 `/feedback` 以將文字記錄和說明傳送給 Anthropic。該命令也提供開啟預先填入的 GitHub issue 的選項。傳送給 Anthropic 需要[驗證](/docs/zh-TW/authentication)。在 Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 和其他第三方提供者上，或當未設定 Anthropic 認證時，`/feedback` 會儲存本機封存，您可以改為傳送給您的 Anthropic 帳戶代表。
* 從您的 shell 執行 `claude doctor` 以進行安裝的唯讀診斷，或在 Claude Code 內執行 `/doctor` 檢查以尋找並修正設定問題
* 檢查 [status.claude.com](https://status.claude.com) 以了解活躍的事件
* 在 GitHub 上搜尋[現有 issue](https://github.com/anthropics/claude-code/issues)
