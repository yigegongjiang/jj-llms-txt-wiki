> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 零數據保留

> 了解 Claude for Enterprise 上 Claude Code 的零數據保留 (ZDR)，包括範圍、禁用功能以及如何請求啟用。

零數據保留 (ZDR) 在通過 Claude for Enterprise 使用 Claude Code 時可用。啟用 ZDR 後，Claude Code 會話期間生成的提示和模型回應會實時處理，並在返回回應後不會由 Anthropic 存儲，除非需要遵守法律或防止濫用。

<Note>
  ZDR 不包含在標準 Claude for Enterprise 方案中，也無法從您的管理員設定中啟用。它僅適用於符合條件的帳戶，需要由 Anthropic 進行單獨啟用。如果您的組織需要 ZDR，請[聯絡銷售](https://www.anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=zero_data_retention_request)或您的 Anthropic 帳戶團隊以確認符合條件。
</Note>

Claude for Enterprise 上的 ZDR 使企業客戶能夠使用 Claude Code 並實現零數據保留，同時獲得管理功能：

* 每個用戶的成本控制
* [分析](/docs/zh-TW/analytics)儀表板
* [服務器管理的設置](/docs/zh-TW/server-managed-settings)
* 審計日誌

Claude for Enterprise 上 Claude Code 的 ZDR 僅適用於 Anthropic 的直接平台。對於在 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 上的 Claude 部署，請參考這些平台的數據保留政策。

<h2 id="zdr-scope">
  ZDR 範圍
</h2>

ZDR 涵蓋 Claude for Enterprise 上的 Claude Code 推理。

<Warning>
  ZDR 在每個組織的基礎上啟用。每個新組織都需要由您的 Anthropic 帳戶團隊單獨啟用 ZDR。ZDR 不會自動應用於在同一帳戶下創建的新組織。請聯繫您的帳戶團隊為任何新組織啟用 ZDR。
</Warning>

<h3 id="what-zdr-covers">
  ZDR 涵蓋的內容
</h3>

ZDR 涵蓋通過 Claude for Enterprise 上的 Claude Code 進行的模型推理調用。當您在終端中使用 Claude Code 時，您發送的提示和 Claude 生成的回應不會由 Anthropic 保留。這適用於 ZDR 組織可用的每個模型。某些模型需要資料保留，在 ZDR 下不可用；請參閱 [ZDR 下的模型可用性](#model-availability-under-zdr)。

<h3 id="what-zdr-does-not-cover">
  ZDR 不涵蓋的內容
</h3>

即使對於啟用了 ZDR 的組織，ZDR 也不適用於以下內容。這些功能遵循[標準資料保留政策](/docs/zh-TW/data-usage#data-retention)：

| 功能             | 詳情                                                                                        |
| -------------- | ----------------------------------------------------------------------------------------- |
| claude.ai 上的聊天 | 通過 Claude for Enterprise 網頁介面的聊天對話不受 ZDR 保護。                                              |
| Cowork         | Cowork 會話不受 ZDR 保護。                                                                       |
| Claude Code 分析 | 不存儲提示或模型回應，但收集生產力中繼資料，例如帳戶電子郵件和使用統計資訊。對於 ZDR 組織，貢獻指標不可用；[分析儀表板](/docs/zh-TW/analytics)僅顯示使用指標。 |
| 使用者和座位管理       | 管理資料（例如帳戶電子郵件和座位分配）根據標準政策保留。                                                              |
| 第三方整合          | 由第三方工具、MCP servers 或其他外部整合處理的資料不受 ZDR 保護。請獨立審查這些服務的資料處理實踐。                                |

<h2 id="features-disabled-under-zdr">
  ZDR 下禁用的功能
</h2>

當為 Claude for Enterprise 上的 Claude Code 組織啟用 ZDR 時，某些需要存儲提示或完成的功能會在後端級別自動禁用：

| 功能                                                  | 原因                                |
| --------------------------------------------------- | --------------------------------- |
| [Web 上的 Claude Code](/docs/zh-TW/claude-code-on-the-web) | 需要服務器端存儲對話歷史記錄。                   |
| Desktop 應用程式的[雲端會話](/docs/zh-TW/desktop#cloud-sessions)  | 需要包含提示和完成的持久會話資料。                 |
| [Artifacts](/docs/zh-TW/artifacts)                       | 需要在 Anthropic 營運的基礎設施上存儲已發佈的頁面內容。 |
| 反饋提交 (`/feedback`)                                  | 提交反饋會將對話資料發送給 Anthropic。          |
| [遠端控制](/docs/zh-TW/remote-control)                       | 在 Anthropic 伺服器上存儲會話記錄，以跨裝置同步對話。  |

這些功能在後端被阻止，無論客戶端顯示如何。如果您在啟動期間在 Claude Code 終端中看到禁用的功能，嘗試使用它會返回一個錯誤，指示組織的政策不允許該操作。

未來的功能如果需要存儲提示或完成，也可能被禁用。

<h3 id="model-availability-under-zdr">
  ZDR 下的模型可用性
</h3>

Claude Fable 5 不適用於啟用零資料保留的組織。此模型類別[需要資料保留](https://platform.claude.com/docs/en/manage-claude/api-and-data-retention#model-specific-data-retention-requirements)，因此來自 ZDR 組織的請求無法由其提供服務。該模型在 ZDR 組織的 `/model` 選擇器中不存在，或顯示為禁用並附帶需要禁用 ZDR 的通知，且無論客戶端配置如何，伺服器都會拒絕對其的請求。

其他模型在 ZDR 下仍然可用。Fable 5 不是預設模型，`best` 別名在可用的地方解析為 Fable 5，在不可用的地方（包括 ZDR 組織）解析為 Opus。

<h2 id="data-retention-for-policy-violations">
  政策違規的數據保留
</h2>

即使啟用了 ZDR，Anthropic 也可能在法律要求或解決使用政策違規時保留數據。如果會話因政策違規而被標記，Anthropic 可能會保留相關的輸入和輸出長達 2 年，與 Anthropic 的標準 ZDR 政策一致。

<h2 id="request-zdr">
  請求 ZDR
</h2>

要為 Claude for Enterprise 上的 Claude Code 請求 ZDR，請[聯繫銷售](https://www.anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=zero_data_retention_request)或您的 Anthropic 帳戶團隊。您的帳戶團隊將在內部提交請求，Anthropic 將在確認符合條件後在您的組織上審查並啟用 ZDR。所有啟用操作都會被審計記錄。

如果您目前通過按使用量付費的 API 密鑰使用 Claude Code 的 ZDR，您可以過渡到 Claude for Enterprise 以獲得管理功能的訪問權限，同時為 Claude Code 保持 ZDR。請聯繫您的帳戶團隊以協調遷移。
