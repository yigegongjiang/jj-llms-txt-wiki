> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用 OpenTelemetry 進行可觀測性

> 使用 OpenTelemetry 將追蹤、指標和事件從 Agent SDK 匯出到您的可觀測性後端。

當您在生產環境中執行代理時，您需要了解它們的行為：

* 它們呼叫了哪些工具
* 每個模型請求花費了多長時間
* 花費了多少個 token
* 失敗發生在哪裡

Agent SDK 可以將此資料作為 OpenTelemetry 追蹤、指標和日誌事件匯出到任何接受 OpenTelemetry Protocol (OTLP) 的後端，例如 Honeycomb、Datadog、Grafana、Langfuse 或自託管收集器。

本指南說明 SDK 如何發出遙測資料、如何配置匯出，以及如何在資料到達您的後端後標記和篩選資料。若要直接從 SDK 回應流讀取 token 使用量和成本，而不是匯出到後端，請參閱[追蹤成本和使用量](/docs/zh-TW/agent-sdk/cost-tracking)。

<h2 id="how-telemetry-flows-from-the-sdk">
  遙測資料如何從 SDK 流動
</h2>

Agent SDK 將 Claude Code CLI 作為子程序執行，並通過本地管道與其通訊。CLI 內建了 OpenTelemetry 檢測：它在每個模型請求和工具執行周圍記錄 span，為 token 和成本計數器發出指標，並為提示和工具結果發出結構化日誌事件。SDK 本身不產生遙測資料。相反，它將配置傳遞給 CLI 程序，CLI 直接匯出到您的收集器。

配置作為環境變數傳遞。預設情況下，子程序繼承您應用程式的環境，因此您可以在以下兩個位置之一配置遙測資料：

* **程序環境：** 在應用程式啟動前在您的 shell、容器或編排器中設定變數。每個 `query()` 呼叫都會自動選擇它們，無需程式碼變更。這是生產部署的推薦方法。
* **按呼叫選項：** 在 `ClaudeAgentOptions.env`（Python）或 `options.env`（TypeScript）中設定變數。當同一程序中的不同代理需要不同的遙測設定時，請使用此方法。在 Python 中，`env` 會合併到繼承的環境之上。在 TypeScript 中，`env` 完全替換繼承的環境，因此請在您傳遞的物件中包含 `...process.env`。

CLI 匯出三個獨立的 OpenTelemetry 訊號。每個都有自己的啟用開關和自己的匯出器，因此您只能開啟需要的訊號。

| 訊號         | 包含內容                             | 啟用方式                                                              |
| ---------- | -------------------------------- | ----------------------------------------------------------------- |
| Metrics    | token、成本、工作階段、程式碼行數和工具決策的計數器     | `OTEL_METRICS_EXPORTER`                                           |
| Log events | 每個提示、API 請求、API 錯誤和工具結果的結構化記錄    | `OTEL_LOGS_EXPORTER`                                              |
| Traces     | 每個互動、模型請求、工具呼叫和 hook 的 span（測試版） | `OTEL_TRACES_EXPORTER` 加上 `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` |

如需完整的指標名稱、事件名稱和屬性清單，請參閱 Claude Code [監控](/docs/zh-TW/monitoring-usage)參考。Agent SDK 發出相同的資料，因為它執行相同的 CLI。Span 名稱列在下面的[讀取代理追蹤](#read-agent-traces)中。

<h2 id="enable-telemetry-export">
  啟用遙測匯出
</h2>

遙測資料預設為關閉，直到您設定 `CLAUDE_CODE_ENABLE_TELEMETRY=1` 並選擇至少一個匯出器。最常見的配置是通過 OTLP HTTP 將所有三個訊號傳送到收集器。

以下範例在字典中設定變數，並通過 `options.env` 傳遞它們。代理執行單個任務，CLI 將 span、指標和事件匯出到位於 `collector.example.com` 的收集器，同時迴圈消費回應流：

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions

  OTEL_ENV = {
      "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
      # 追蹤需要此項，追蹤處於測試版。指標和日誌事件不需要此項。
      "CLAUDE_CODE_ENHANCED_TELEMETRY_BETA": "1",
      # 為每個訊號選擇一個匯出器。SDK 使用 otlp；請參閱下面的注意。
      "OTEL_TRACES_EXPORTER": "otlp",
      "OTEL_METRICS_EXPORTER": "otlp",
      "OTEL_LOGS_EXPORTER": "otlp",
      # 標準 OTLP 傳輸配置。
      "OTEL_EXPORTER_OTLP_PROTOCOL": "http/protobuf",
      "OTEL_EXPORTER_OTLP_ENDPOINT": "http://collector.example.com:4318",
      "OTEL_EXPORTER_OTLP_HEADERS": "Authorization=Bearer your-token",
  }


  async def main():
      options = ClaudeAgentOptions(env=OTEL_ENV)
      async for message in query(
          prompt="List the files in this directory", options=options
      ):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const otelEnv = {
    CLAUDE_CODE_ENABLE_TELEMETRY: "1",
    // 追蹤需要此項，追蹤處於測試版。指標和日誌事件不需要此項。
    CLAUDE_CODE_ENHANCED_TELEMETRY_BETA: "1",
    // 為每個訊號選擇一個匯出器。SDK 使用 otlp；請參閱下面的注意。
    OTEL_TRACES_EXPORTER: "otlp",
    OTEL_METRICS_EXPORTER: "otlp",
    OTEL_LOGS_EXPORTER: "otlp",
    // 標準 OTLP 傳輸配置。
    OTEL_EXPORTER_OTLP_PROTOCOL: "http/protobuf",
    OTEL_EXPORTER_OTLP_ENDPOINT: "http://collector.example.com:4318",
    OTEL_EXPORTER_OTLP_HEADERS: "Authorization=Bearer your-token",
  };

  for await (const message of query({
    prompt: "List the files in this directory",
    // env 在 TypeScript 中完全替換繼承的環境，因此首先展開
    // process.env 以保留 PATH、ANTHROPIC_API_KEY 和其他變數。
    options: { env: { ...process.env, ...otelEnv } },
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

因為子程序預設繼承您應用程式的環境，您可以通過在 Dockerfile、Kubernetes 清單或 shell 設定檔中匯出這些變數並完全省略 `options.env` 來達到相同的結果。

<Note>
  `console` 匯出器將遙測資料寫入標準輸出，SDK 將其用作其訊息通道。在通過 SDK 執行時，不要將 `console` 設定為匯出器值。若要在本地檢查遙測資料，請將 `OTEL_EXPORTER_OTLP_ENDPOINT` 指向本地收集器或一體化 Jaeger 容器。
</Note>

<h3 id="flush-telemetry-from-short-lived-calls">
  從短期呼叫刷新遙測資料
</h3>

CLI 批次處理遙測資料並按間隔匯出。在乾淨的程序退出時，它會嘗試刷新待處理資料，但刷新受短暫超時限制，因此如果收集器回應緩慢，span 仍可能被丟棄。如果程序在 CLI 關閉前被終止，批次緩衝區中的任何內容都會丟失。降低匯出間隔會減少這兩個時間窗口。

預設情況下，指標每 60 秒匯出一次，追蹤和日誌每 5 秒匯出一次。以下範例縮短了所有三個間隔，以便資料在短任務仍在執行時到達收集器：

<CodeGroup>
  ```python Python theme={null}
  OTEL_ENV = {
      # ... 來自前面範例的匯出器配置 ...
      "OTEL_METRIC_EXPORT_INTERVAL": "1000",
      "OTEL_LOGS_EXPORT_INTERVAL": "1000",
      "OTEL_TRACES_EXPORT_INTERVAL": "1000",
  }
  ```

  ```typescript TypeScript theme={null}
  const otelEnv = {
    // ... 來自前面範例的匯出器配置 ...
    OTEL_METRIC_EXPORT_INTERVAL: "1000",
    OTEL_LOGS_EXPORT_INTERVAL: "1000",
    OTEL_TRACES_EXPORT_INTERVAL: "1000",
  };
  ```
</CodeGroup>

<h2 id="read-agent-traces">
  讀取代理追蹤
</h2>

追蹤提供了代理執行的最詳細檢視。設定 `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` 後，代理迴圈的每一步都會變成您可以在追蹤後端檢查的 span：

* **`claude_code.interaction`：** 包裝代理迴圈的單個轉折，從接收提示到產生回應。
* **`claude_code.llm_request`：** 包裝每個 Claude API 呼叫，將模型名稱、延遲和 token 計數作為屬性。
* **`claude_code.tool`：** 包裝每個工具呼叫，具有權限等待的子 span（`claude_code.tool.blocked_on_user`）和執行本身（`claude_code.tool.execution`）。
* **`claude_code.hook`：** 包裝每個 [hook](/docs/zh-TW/agent-sdk/hooks) 執行。除了上述變數外，還需要詳細的測試版追蹤（`ENABLE_BETA_TRACING_DETAILED=1` 和 `BETA_TRACING_ENDPOINT`）。

`llm_request`、`tool` 和 `hook` span 是封閉 `claude_code.interaction` span 的子項。當代理通過 Task 工具生成子代理時，子代理的 `llm_request` 和 `tool` span 嵌套在父代理的 `claude_code.tool` span 下，因此完整的委派鏈顯示為一個追蹤。

Span 預設帶有 `session.id` 屬性。當您對同一[工作階段](/docs/zh-TW/agent-sdk/sessions)進行多個 `query()` 呼叫時，在您的後端篩選 `session.id` 以將它們視為一個時間線。如果 `OTEL_METRICS_INCLUDE_SESSION_ID` 設定為假值，則省略該屬性。

<Note>
  追蹤處於測試版。Span 名稱和屬性可能在版本之間變更。請參閱監控參考中的[追蹤（測試版）](/docs/zh-TW/monitoring-usage#traces-beta)以了解追蹤匯出器配置變數。
</Note>

<h2 id="link-traces-to-your-application">
  將追蹤連結到您的應用程式
</h2>

SDK 自動將 W3C 追蹤上下文傳播到 CLI 子程序。當您在應用程式中有活躍的 OpenTelemetry span 時呼叫 `query()`，SDK 會將 `TRACEPARENT` 和 `TRACESTATE` 注入子程序環境，CLI 讀取它們，使其 `claude_code.interaction` span 成為您的 span 的子項。代理執行隨後出現在您的應用程式追蹤中，而不是作為斷開連接的根。

啟用追蹤上下文傳播時，CLI 還會將 `TRACEPARENT` 轉發到它執行的每個 Bash 和 PowerShell 命令。如果通過 Bash 工具啟動的命令發出自己的 OpenTelemetry span，這些 span 會嵌套在包裝該命令的 `claude_code.tool.execution` span 下。

當您在 `options.env` 中明確設定 `TRACEPARENT` 時，會跳過自動注入，因此您可以在需要時固定特定的父上下文。互動式 CLI 工作階段完全忽略入站 `TRACEPARENT`；只有 Agent SDK 和 `claude -p` 執行會遵守它。請參閱監控參考中的[追蹤（測試版）](/docs/zh-TW/monitoring-usage#traces-beta)以了解完整的 span 和屬性參考。

<h2 id="tag-telemetry-from-your-agent">
  從您的代理標記遙測資料
</h2>

預設情況下，CLI 將 `service.name` 報告為 `claude-code`。如果您執行多個代理，或將 SDK 與匯出到同一收集器的其他服務一起執行，請覆蓋服務名稱並新增資源屬性，以便您可以在後端按代理篩選。

以下範例重新命名服務並附加部署中繼資料。這些值作為 OpenTelemetry 資源屬性應用於代理發出的每個 span、指標和事件：

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      env={
          # ... 匯出器配置 ...
          "OTEL_SERVICE_NAME": "support-triage-agent",
          "OTEL_RESOURCE_ATTRIBUTES": "service.version=1.4.0,deployment.environment=production",
      },
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    env: {
      ...process.env,
      // ... 匯出器配置 ...
      OTEL_SERVICE_NAME: "support-triage-agent",
      OTEL_RESOURCE_ATTRIBUTES":
        "service.version=1.4.0,deployment.environment=production",
    },
  };
  ```
</CodeGroup>

<h2 id="attribute-actions-to-your-end-users">
  將屬性操作歸因於您的終端使用者
</h2>

CLI 根據它用來呼叫 Anthropic 的認證將[身份屬性](/docs/zh-TW/monitoring-usage#standard-attributes)附加到每個事件。當您建立一個從一個部署為許多終端使用者提供服務的應用程式時，這些屬性識別您的服務認證，而不是代理代表其行動的終端使用者。

若要使工具呼叫和 MCP 活動可歸因於您應用程式的終端使用者，請在每個 `query()` 呼叫上將終端使用者身份注入為資源屬性。在插值前對值進行百分比編碼，因為 `OTEL_RESOURCE_ATTRIBUTES` [保留逗號、空格和等號](/docs/zh-TW/monitoring-usage#multi-team-organization-support)。以下範例將請求使用者和租戶附加到來自一個請求的每個 span 和事件。它假設您的網頁框架中有一個 `request` 物件，其中包含使用者和租戶 ID：

<CodeGroup>
  ```python Python theme={null}
  from urllib.parse import quote

  options = ClaudeAgentOptions(
      env={
          # ... 匯出器配置 ...
          "OTEL_RESOURCE_ATTRIBUTES": f"enduser.id={quote(request.user_id)},tenant.id={quote(request.tenant_id)}",
      },
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    env: {
      ...process.env,
      // ... 匯出器配置 ...
      OTEL_RESOURCE_ATTRIBUTES: `enduser.id=${encodeURIComponent(request.userId)},tenant.id=${encodeURIComponent(request.tenantId)}`,
    },
  };
  ```
</CodeGroup>

附加終端使用者身份後，`tool_decision`、`tool_result`、`mcp_server_connection` 和 `permission_mode_changed` 事件（以 `claude_code.` 前綴命名的日誌記錄匯出）會變成您可以轉發到安全資訊和事件管理 (SIEM) 平台的按使用者稽核追蹤。請參閱監控參考中的[稽核安全事件](/docs/zh-TW/monitoring-usage#audit-security-events)以了解完整的安全相關事件清單和每個事件攜帶的屬性。

<h2 id="control-sensitive-data-in-exports">
  控制匯出中的敏感資料
</h2>

遙測資料預設是結構化的。持續時間、模型名稱和工具名稱記錄在每個 span 上；token 計數在基礎 API 請求返回使用資料時記錄，因此失敗或中止請求的 span 可能會省略它們。您的代理讀取和寫入的內容預設不被記錄。這些選擇加入變數將內容新增到匯出的資料：

| 變數                        | 新增內容                                                                                                                                                                                                                                        |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `OTEL_LOG_USER_PROMPTS=1` | `claude_code.user_prompt` 事件和 `claude_code.interaction` span 上的提示文字                                                                                                                                                                         |
| `OTEL_LOG_TOOL_DETAILS=1` | `claude_code.tool_result` 事件上的工具輸入引數（檔案路徑、shell 命令、搜尋模式）                                                                                                                                                                                    |
| `OTEL_LOG_TOOL_CONTENT=1` | `claude_code.tool` 上的完整工具輸入和輸出主體作為 span 事件，截斷為 60 KB。需要啟用[追蹤](#read-agent-traces)                                                                                                                                                           |
| `OTEL_LOG_RAW_API_BODIES` | 完整的 Anthropic Messages API 請求和回應 JSON 作為 `claude_code.api_request_body` 和 `claude_code.api_response_body` 日誌事件。設定為 `1` 以獲得截斷為 60 KB 的內聯主體，或 `file:<dir>` 以在磁碟上獲得未截斷的主體，事件中有 `body_ref` 路徑。主體包括整個對話歷史記錄，並且已編輯了擴展思考內容。啟用此項意味著同意上述三個變數將揭示的所有內容 |

除非您的可觀測性管道已獲准儲存您的代理處理的資料，否則請保持這些未設定。請參閱監控參考中的[安全和隱私](/docs/zh-TW/monitoring-usage#security-and-privacy)以了解完整的屬性清單和編輯行為。

<h2 id="related-documentation">
  相關文件
</h2>

這些指南涵蓋了監控和部署代理的相鄰主題：

* [追蹤成本和使用量](/docs/zh-TW/agent-sdk/cost-tracking)：從訊息流讀取 token 和成本資料，無需外部後端。
* [託管 Agent SDK](/docs/zh-TW/agent-sdk/hosting)：在容器中部署代理，您可以在環境層級設定 OpenTelemetry 變數。
* [監控](/docs/zh-TW/monitoring-usage)：CLI 發出的每個環境變數、指標和事件的完整參考。
