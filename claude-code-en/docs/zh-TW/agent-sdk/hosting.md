> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 託管 Agent SDK

> 在生產環境中部署 Agent SDK：子流程架構、會話持久化、擴展、可觀測性和 Docker、Kubernetes 及沙箱提供商的多租戶隔離。

Agent SDK 會生成並監督一個擁有 shell、工作目錄和磁盤上會話文件的 `claude` CLI 子流程。託管它不像託管無狀態 API 包裝器。每個運行的代理都是與本地狀態綁定的長期進程，這決定了您如何分配資源、持久化會話以及跨租戶進行擴展。

本頁涵蓋在您自己的基礎設施上自託管：了解[子流程模型](#the-subprocess-model)、[選擇會話模式](#choose-a-session-pattern)、[配置容器](#provision-the-container)以及[處理生產問題](#handle-production-concerns)，如持久化、可觀測性、身份驗證和多租戶隔離。有關可部署的 Dockerfile 和 Kubernetes 清單，請參閱[託管指南](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting)。

如果您不需要基礎設施控制、自定義隔離或自己的數據平面，請改為考慮[託管代理](https://platform.claude.com/docs/zh-TW/managed-agents/overview)：一個託管的 REST API，其中 Anthropic 運行代理和沙箱，因此您的應用程序發送事件並流回結果，無需操作任何託管基礎設施。

<Info>
  如需超越基本沙箱的安全強化（包括網路控制、認證管理和隔離選項），請參閱[安全部署](/docs/zh-TW/agent-sdk/secure-deployment)。
</Info>

<h2 id="the-subprocess-model">
  子進程模型
</h2>

此頁面上的每個託管決策都遵循 SDK 如何運行代理的方式。當您的代碼調用 `query()` 時，SDK 會生成一個單獨的 `claude` CLI 進程，並通過 stdio 與其通信。該子進程擁有 shell、工作目錄和本地磁盤上的 JSONL 會話記錄。

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/hosting-subprocess.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=9dac857ca9d3b1410c3734900c386004" alt="Request flow: client to your app, which spawns a claude CLI subprocess over stdio inside the container; the subprocess writes to local disk and calls api.anthropic.com over HTTPS" width="920" height="220" data-path="images/agent-sdk/hosting-subprocess.svg" />

一個代理會話映射到一個子進程。運行 N 個並發會話意味著 N 個子進程，每個都有自己的進程樹和記錄文件。默認情況下，它們都繼承您應用程序的工作目錄，因此當會話需要單獨的文件系統時，在每個 `query()` 調用上傳遞 `cwd`：

<CodeGroup>
  ```typescript TypeScript theme={null}
  query({ prompt, options: { cwd: "/work/session-a" } })
  ```

  ```python Python theme={null}
  query(prompt=prompt, options=ClaudeAgentOptions(cwd="/work/session-a"))
  ```
</CodeGroup>

<h3 id="state-that-lives-on-local-disk">
  存儲在本地磁盤上的狀態
</h3>

三種代理狀態默認存儲在容器的文件系統上。它們都不會在容器重新啟動、縮減或移動到不同節點時存活。

| 狀態               | 默認位置                                                                  |
| ---------------- | --------------------------------------------------------------------- |
| 會話記錄             | `~/.claude/projects/`，或如果設置了 `CLAUDE_CONFIG_DIR`，則為其下的 `projects/` 目錄 |
| `CLAUDE.md` 內存文件 | 用戶層級的 `~/.claude/CLAUDE.md` 和項目層級的會話工作目錄                              |
| 工作目錄工件           | 會話的工作目錄                                                               |

要在主機之間持久化記錄，請配置 [`SessionStore` 適配器](/docs/zh-TW/agent-sdk/session-storage)。內存文件和其他工作目錄工件需要自己的存儲策略，例如掛載卷或對象存儲同步。

有關會話、恢復和分叉在 API 級別如何工作的信息，請參閱 [Sessions](/docs/zh-TW/agent-sdk/sessions)。

<h2 id="choose-a-session-pattern">
  選擇會話模式
</h2>

這四種模式涵蓋會話生命週期：容器相對於其服務的會話存在多長時間。關於容器運行的位置，[託管食譜](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb)提供了[可部署的代碼](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting)，用於本地 Docker、Modal 和 Kubernetes。在此選擇會話模式，並從食譜中選擇部署目標。

<h3 id="ephemeral-sessions">
  臨時會話
</h3>

為每個用戶任務創建一個容器，並在任務完成時銷毀它。最適合一次性任務。用戶可能仍然可以在任務完成時與 AI 互動，但一旦完成，容器就會被銷毀。

示例工作負載包括錯誤調查和修復、發票和收據提取、文檔翻譯和媒體轉換。

容器運行一個一次性入口點，該入口點調用 SDK 並退出。下面的示例顯示了一個最小的 TypeScript 版本。將其保存為 `entrypoint.mts` 或在 `package.json` 中設置 `"type": "module"`，以便頂級 `await` 可用。

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const prompt = process.env.TASK_PROMPT!;
for await (const message of query({ prompt, options: { maxTurns: 20 } })) {
  console.log(message);
}
```

<h3 id="long-running-sessions">
  長時間運行的會話
</h3>

運行持久容器實例，通常在每個容器中託管多個 SDK 進程，以服務持續的工作。最適合採取自主行動、提供內容或處理高容量消息流的代理。

示例工作負載包括對傳入郵件進行分類和回應的電子郵件代理、通過容器端口託管每個用戶可編輯網站的網站構建器，以及處理來自 Slack 等平台的持續流量的聊天機器人。

容器公開 HTTP 或 WebSocket 端點，並將每個活動會話映射到長期查詢及其背後的子進程。在 TypeScript 中，使用 [`streamInput()`](/docs/zh-TW/agent-sdk/typescript#query-object) 向活動會話添加轉換，並使用 [`startup()`](/docs/zh-TW/agent-sdk/typescript#startup) 在傳入流量之前預熱子進程。在 Python 中，使用 [`ClaudeSDKClient`](/docs/zh-TW/agent-sdk/python#claudesdkclient) 在多個轉換中保持會話打開。調整容器大小，使其能夠在內存中保持最大數量的並發會話。

<h3 id="hybrid-sessions">
  混合會話
</h3>

臨時容器在啟動時從 [`SessionStore`](/docs/zh-TW/agent-sdk/session-storage) 進行補充，並將更新持久化回去。最適合跨越許多交互但在交互之間處於空閒狀態的會話。容器在空閒期間關閉，當用戶返回時重新啟動。

示例工作負載包括具有間歇性檢查的個人項目管理器、暫停和恢復數小時的深度研究，以及在交互中加載票證歷史記錄的客戶支持代理。

根據您期望用戶返回的頻率調整提供商的空閒超時。在沒有配置 `SessionStore` 的情況下關閉容器會丟失其轉錄，因此存儲對於此模式是必需的，而不是可選的。

該模式的關鍵在於通過 ID 恢復會話，並附加共享存儲：

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, type SessionStore } from "@anthropic-ai/claude-agent-sdk";

  declare const userInput: string;
  declare const sessionId: string;          // looked up from your database by user
  declare const sessionStore: SessionStore; // S3, Redis, Postgres, or your own adapter

  for await (const message of query({
    prompt: userInput,
    options: { resume: sessionId, sessionStore },
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt=user_input,
      options=ClaudeAgentOptions(
          resume=session_id,            # looked up from your database by user
          session_store=session_store,  # S3, Redis, Postgres, or your own adapter
      ),
  ):
      ...
  ```
</CodeGroup>

有關完整的 `SessionStore` 接口和參考適配器，請參閱[會話存儲](/docs/zh-TW/agent-sdk/session-storage)。

<h3 id="multi-agent-container">
  多代理容器
</h3>

在一個容器內運行多個 SDK 子進程。最適合必須密切協作的代理，例如多代理模擬，其中代理在共享環境中相互交互。

為每個代理提供自己的工作目錄，以便它們不會相互覆蓋文件，並隔離設置加載，以便每個代理的 `CLAUDE.md` 文件不會洩露到其他代理。有關特定選項，請參閱[多租戶隔離](#multi-tenant-isolation)。

<h2 id="provision-the-container">
  配置容器
</h2>

<h3 id="container-based-sandboxing">
  基於容器的沙箱
</h3>

在沙箱容器內運行 SDK，以實現進程隔離、資源限制、網絡控制和臨時文件系統。多個提供商專門提供適合 Agent SDK 模型的沙箱容器環境。

選擇提供商時要回答的問題：

* **誰運行沙箱**：沙箱即服務提供商為您運營基礎設施，而自託管選項則提供您可在自己的基礎設施上運行的軟體。
* **冷啟動延遲**：從「創建沙箱」到「準備好接受第一個請求」需要多長時間。臨時模式需要亞秒級啟動。長期運行模式可以容忍更長的延遲。
* **持久存儲**：提供商是否提供持久卷或僅提供臨時磁盤。混合模式需要在沙箱內或沙箱旁邊的某處進行持久存儲。
* **定價模型**：按秒、按請求或按小時固定計費。按秒計費適合突發性臨時工作負載。按小時計費適合長期運行的會話。
* **網絡**：支持自定義出站規則、出站代理和私有 VPC 對等互連，用於受管制的環境。

要評估的提供商：

* [Modal Sandbox](https://modal.com/docs/guide/sandbox)，附帶[演示實現](https://modal.com/docs/examples/claude-slack-gif-creator)
* [Cloudflare Sandboxes](https://github.com/cloudflare/sandbox-sdk)
* [Daytona](https://www.daytona.io/)
* [E2B](https://e2b.dev/)
* [Fly Machines](https://fly.io/docs/machines/)
* [Vercel Sandbox](https://vercel.com/docs/functions/sandbox)

有關自託管選項（如 Docker、gVisor 和 Firecracker）以及詳細的隔離配置，請參閱[隔離技術](/docs/zh-TW/agent-sdk/secure-deployment#isolation-technologies)。

<h3 id="runtime-dependencies">
  運行時依賴項
</h3>

容器只需要您的 SDK 的語言運行時：

* Python SDK 需要 Python 3.10+，或 TypeScript SDK 需要 Node.js 18+
* 兩個 SDK 套件都為主機平台捆綁了本機 Claude Code 二進制文件，因此不需要為生成的 CLI 單獨安裝 Claude Code 或 Node.js

捆綁的二進制文件被固定到 SDK 套件版本，因此更新 SDK 是更新 CLI 的方式。SDK 遵循 semver：持續採用補丁版本，並在採用次要版本之前查看 [TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md) 或 [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md) 更改日誌。

<h3 id="resources">
  資源
</h3>

每個代理的 1 GiB RAM、5 GiB 磁盤和 1 個 CPU 是新啟動實例的合理起點。內存使用量隨著會話長度和工具活動而增長，因此應根據您實際需要的會話長度和並發性進行調整，而不是根據空閒基線。有關如何計算每個主機的代理數量，請參閱[擴展和並發](#scaling-and-concurrency)。

<h3 id="network">
  網絡
</h3>

SDK 需要對 `api.anthropic.com` 的出站 HTTPS，或在 Amazon Bedrock 或 Google Cloud 的 Agent Platform 上運行時對您提供商的區域端點的出站 HTTPS。如果您的代理使用 [MCP servers](/docs/zh-TW/agent-sdk/mcp) 或外部工具，它們還需要對這些端點的出站訪問。對於生產環境，通過強制執行域名允許列表、注入憑證和記錄請求的出站代理路由出站流量。有關完整模式，請參閱[安全部署](/docs/zh-TW/agent-sdk/secure-deployment)。

對於入站流量，在容器上公開 HTTP 或 WebSocket 端口。您的應用程式在該端口上處理客戶端請求並在內部調用 SDK；子進程本身不在網絡上監聽。

<h2 id="handle-production-concerns">
  處理生產環境問題
</h2>

在部署自託管代理之前，請完成這些決策。

<h3 id="session-and-state-persistence">
  會話和狀態持久化
</h3>

預設本地磁碟在重新啟動、縮減或移至不同節點時會遺失。對於任何使用者期望恢復的會話，使用 [`SessionStore` 適配器](/docs/zh-TW/agent-sdk/session-storage)將文字記錄鏡像到持久儲存。請參閱[參考實現](/docs/zh-TW/agent-sdk/session-storage#reference-implementations)以了解 S3、Redis 和 Postgres 適配器，以及用於您自己實現的一致性測試套件。

關於 `SessionStore` 行為，有三件事需要了解：

* **僅文字記錄**：`SessionStore` 鏡像文字記錄，而不是 `CLAUDE.md` 記憶檔案或其他工作目錄工件。掛載共享磁碟區或分別同步這些檔案。
* **鏡像，而非替換**：子程序首先寫入本地磁碟，然後存儲接收每個批次的副本。本地寫入保持權威性。
* **`mirror_error` 訊息**：存儲拒絕的批次最多會重新發送三次，每次重試前有短暫的退避；逾時的呼叫不會重試。如果批次仍然失敗，SDK 會捨棄它、發出 `{ type: "system", subtype: "mirror_error" }` 訊息，並繼續查詢。如果存儲持久性很重要，請對這些進行警報。

<h3 id="observability">
  可觀測性
</h3>

Agent SDK 代理是長期運行的程序，在許多 API 往返中生成工具呼叫。沒有遙測，您無法看到哪些工具運行、它們花費了多長時間或會話在何處停滯。

SDK 從環境繼承 OpenTelemetry 配置。在容器或編排器級別設定 OTEL 環境變數，以便每個 `query()` 呼叫都將跨度、指標和日誌事件匯出到您的收集器。下面的範例為所有三個信號啟用 OTLP 匯出。`CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` 僅對跟蹤是必需的；如果您僅匯出指標和日誌，請省略它。

```bash title=".env' theme={null}
CLAUDE_CODE_ENABLE_TELEMETRY=1
CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1
OTEL_TRACES_EXPORTER=otlp
OTEL_METRICS_EXPORTER=otlp
OTEL_LOGS_EXPORTER=otlp
OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf
OTEL_EXPORTER_OTLP_ENDPOINT=http://collector.example.com:4318
```

預設情況下，匯出中不包含提示文字和工具輸入。請參閱[控制匯出中的敏感資料](/docs/zh-TW/agent-sdk/observability#control-sensitive-data-in-exports)以了解選擇加入標誌，以及[可觀測性](/docs/zh-TW/agent-sdk/observability)以了解完整的信號目錄。

<h3 id="auth-and-secrets">
  驗證和祕密
</h3>

託管時有三個驗證問題很重要：

* **Anthropic API**：子程序從其環境讀取 `ANTHROPIC_API_KEY`。從您的祕密管理器提供它，或設定 `ANTHROPIC_BASE_URL` 以透過在容器外注入金鑰的代理路由模型呼叫。請參閱[認證管理](/docs/zh-TW/agent-sdk/secure-deployment#credential-management)以了解代理模式，以及[SDK 概述](/docs/zh-TW/agent-sdk/overview#get-started)以了解支援的驗證方法。
* **入站**：在代理容器前面的閘道處放置驗證。代理應接收預先驗證的請求，不應是驗證使用者令牌的元件。
* **出站工具**：將工具認證保留在代理環境之外。透過在請求離開容器後注入 API 金鑰的代理路由出站呼叫。代理進行呼叫；代理添加認證。

<h3 id="scaling-and-concurrency">
  縮放和並行
</h3>

每個會話在其自己的子程序中運行，因此主機上的並行受其 RAM 可以容納多少個子程序的限制。

使用此公式調整每個主機的大小：

```text theme={null}
agents per host = (host RAM - overhead) / (per-session RAM ceiling)
```

透過在您的目標長度下運行代表性會話並在預期的工具負載下記錄峰值 RSS 來測量每個會話的上限。[資源](#resources)中的 1 GiB 起點是下限，而不是上限。

水平縮放路由取決於您的模式。對於長時間運行的會話，其中容器保持許多會話，在負載平衡器後面運行容器池，並使用 `sessionId` 上的一致性雜湊將每個會話固定到一個容器。固定的會話會持續命中同一容器，因此會命中同一運行中的子程序，直到它被驅逐或容器重新啟動。

來自單個會話的大量並行[子代理](/docs/zh-TW/agent-sdk/subagents)可能會達到 API 速率限制。將工作分解為較小的批次，而不是發出一個寬廣的調度。

<h3 id="cost">
  成本
</h3>

Anthropic 令牌成本通常主導容器基礎設施成本一個數量級或更多。最小配置的容器大約每小時運行 \$0.05，而單個長代理會話可能花費數美元的令牌。請參閱[成本追蹤](/docs/zh-TW/agent-sdk/cost-tracking)以了解每個會話的令牌計帳。

<h3 id="multi-tenant-isolation">
  多租戶隔離
</h3>

預設 SDK 行為從檔案系統讀取設定和 `CLAUDE.md` 記憶檔案。在為多個租戶提供服務的共享容器中，這些檔案可能會將一個租戶的上下文洩露到另一個租戶的會話中。

要在共享容器內隔離租戶：

* 在 TypeScript 中傳遞 `settingSources: []` 或在 Python 中傳遞 `setting_sources=[]`，以便不載入檔案系統設定。
* 在 `env` 中設定 `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1`。[自動記憶](/docs/zh-TW/memory#auto-memory)在 `~/.claude/projects/<project>/memory/` 載入系統提示，無論 `settingSources` 如何。請參閱[settingSources 不控制的內容](/docs/zh-TW/agent-sdk/claude-code-features#what-settingsources-does-not-control)以了解無條件載入的其他輸入。
* 將 `CLAUDE_CONFIG_DIR` 指向每個租戶目錄，以便租戶不共享 `~/.claude.json` 全域配置。
* 使用每個租戶的工作目錄。在每個 `query()` 呼叫上明確傳遞 `cwd`。
* 在您的代理處應用每個租戶的出站規則，例如不同的出站 IP、認證或網域允許清單，以便受損的租戶無法透過另一個租戶的出站原則進行資料外洩。

下面的範例將四個 SDK 級別的選項應用在一起。構造 `tenantDir` 和 `configDir`，以便每個租戶獲得其他租戶無法讀取的路徑。在 TypeScript 中，`env` 替換子程序環境，因此展開 `...process.env` 以保留繼承的變數，如 `PATH` 和 `ANTHROPIC_API_KEY`。在 Python 中，`env` 合併在繼承的環境之上。

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  declare const prompt: string;
  declare const tenantDir: string;
  declare const configDir: string;

  for await (const message of query({
    prompt,
    options: {
      cwd: tenantDir,
      settingSources: [],
      env: {
        ...process.env,
        CLAUDE_CONFIG_DIR: configDir,
        CLAUDE_CODE_DISABLE_AUTO_MEMORY: "1",
      },
    },
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions

  async for message in query(
      prompt=prompt,
      options=ClaudeAgentOptions(
          cwd=tenant_dir,
          setting_sources=[],
          env={
              "CLAUDE_CONFIG_DIR": config_dir,
              "CLAUDE_CODE_DISABLE_AUTO_MEMORY": "1",
          },
      ),
  ):
      ...
  ```
</CodeGroup>

如需每個租戶的網路控制，請參閱[安全部署](/docs/zh-TW/agent-sdk/secure-deployment)。

<h2 id="known-limitations">
  已知限制
</h2>

在您的部署設計中規劃這些限制。

| 限制                  | 解決方案                                                                                                                                                                                      |
| ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 沒有頂級會話超時            | 會話不會自動超時。在 `Options` 中設置 `maxTurns` 以限制代理在停止前進行多少次工具使用往返。                                                                                                                                 |
| 長會話期間的記憶體增長         | 限制會話長度或定期回收子流程。請參閱 [Scaling and concurrency](#scaling-and-concurrency)。                                                                                                                   |
| 大規模並行子代理扇出可能會觸發速率限制 | 將工作分解為較小的批次，而不是發出一次寬泛的調度。                                                                                                                                                                 |
| 沒有每個子代理的牆鐘截止時間      | 使用 `AgentDefinition` 中的 `maxTurns` 限制每個 [subagent](/docs/zh-TW/agent-sdk/subagents)。僅對於後台子代理，`CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS` 設置一個停滯監視程序，當 `run_in_background` 子代理停止產生輸出時觸發；它不是總運行時間截止時間。 |

<h2 id="next-steps">
  後續步驟
</h2>

* [Hosting cookbook](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb)：包含 Docker、Modal 和 Kubernetes 的[可部署程式碼](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting)的筆記本逐步解說。
* [Session storage](/docs/zh-TW/agent-sdk/session-storage)：使用 `SessionStore` 配接器在主機間保留文字記錄。
* [Observability](/docs/zh-TW/agent-sdk/observability)：將 OTEL 追蹤、指標和日誌匯出到您的收集器。
* [Secure deployment](/docs/zh-TW/agent-sdk/secure-deployment)：網路控制、認證管理和隔離強化。
* [Cost tracking](/docs/zh-TW/agent-sdk/cost-tracking)：每個會話的權杖和成本計算。
