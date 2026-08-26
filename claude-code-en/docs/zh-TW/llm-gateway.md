> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 其他 LLM gateway

> 透過您的組織已運行的 LLM gateway 路由 Claude Code。涵蓋將 Claude Code 連接到 gateway、為您的組織推出 gateway，以及 Claude Code 發送到 gateway 的內容。

本節涵蓋使用您的組織已運行的 gateway 產品，而不是 [Claude apps gateway](/docs/zh-TW/claude-apps-gateway)。有關 gateway 是什麼、它如何位於 Claude Code 和您的提供商之間，以及如何在 Claude apps gateway 和其他產品之間進行選擇，請參閱 [gateway 概述](/docs/zh-TW/gateways)。

<Note>
  * 如果您是連接到現有 gateway 的開發人員：[將 Claude Code 連接到您的 gateway](/docs/zh-TW/llm-gateway-connect)
  * 如果您是為組織推出 gateway 的管理員：[部署和分發 gateway](/docs/zh-TW/llm-gateway-rollout)
  * 如果您正在配置 gateway 產品：[gateway 協議參考](/docs/zh-TW/llm-gateway-protocol)
</Note>

任何公開[支持的 API 格式](/docs/zh-TW/llm-gateway-protocol#api-formats)的 gateway 都可以運作。Anthropic 不認可、維護或審計第三方 gateway 產品，也不支持透過任何 gateway 將 Claude Code 路由到非 Claude 模型。按照 gateway 自己的文檔部署它，然後使用下面的[推出步驟](#roll-out-a-gateway)完成 Claude Code 端的配置。

<h2 id="what-a-gateway-provides">
  gateway 提供的功能
</h2>

gateway 為您的組織提供一個地方來管理：

* **憑證**：提供商金鑰保留在伺服器端；開發人員改為持有 gateway 憑證
* **使用情況追蹤**：按開發人員或團隊歸屬使用情況，無論哪個提供商處理請求
* **成本控制**：在一個地方強制執行預算和速率限制
* **審計日誌**：記錄每個模型請求以進行合規性檢查
* **提供商切換**：在 gateway 配置中更改提供商，無需觸及開發人員機器

除了提供商切換外，所有這些都適用於上游是 Anthropic API 還是[雲提供商](/docs/zh-TW/third-party-integrations)。提供商切換而無需重新配置開發人員機器也取決於 gateway 公開單一 [Anthropic 格式端點](/docs/zh-TW/llm-gateway-protocol#api-formats)，無論上游如何；公開提供商自己格式的 gateway 將客戶端配置與該提供商綁定。

權衡是 gateway 成為您的組織運營的基礎設施。Claude Code 在每個版本中添加功能，不轉發這些功能的 gateway 會破壞相應的功能，因此 gateway 產品需要隨著 Claude Code 的發展而保持更新。[gateway 協議參考](/docs/zh-TW/llm-gateway-protocol)涵蓋要轉發的內容。

<h2 id="roll-out-a-gateway">
  推出 gateway
</h2>

當您準備好為組織推出 LLM gateway 時，無論您選擇哪個 gateway 產品，順序都是相同的：

1. 部署 gateway 並給予它您的提供商憑證，以便它可以對它轉發的請求進行身份驗證。
2. 為每個開發人員發行 gateway 憑證，以便使用情況歸屬於開發人員，離職時撤銷一個憑證。
3. 透過[受管設定檔](/docs/zh-TW/settings#settings-files)和您的機密工具分發配置，以便每台機器都接收基本 URL 和憑證。當兩者都分發時，開發人員無需配置任何內容。如果您沒有設定分發，開發人員按照[連接頁面](/docs/zh-TW/llm-gateway-connect)自己設置變數。
4. 讓每個開發人員[檢查 Claude Code 中的配置](/docs/zh-TW/llm-gateway-connect#check-for-an-existing-configuration)，以便分發問題在他們依賴 gateway 之前浮出水面。

[為您的組織推出 LLM gateway](/docs/zh-TW/llm-gateway-rollout)逐步介紹每個步驟，並顯示在每個步驟分發的配置檔案。gateway 是組織設置的一部分；有關政策強制執行、使用情況可見性和資料處理決策，請參閱[為您的組織設置 Claude Code](/docs/zh-TW/admin-setup)。

<h2 id="subscriptions-and-gateways">
  訂閱和 gateway
</h2>

當[gateway 憑證變數](/docs/zh-TW/llm-gateway-connect#set-the-credential-variable)或 `apiKeyHelper` 處於活動狀態時，開發人員的 claude.ai 訂閱不被使用：憑證替換該會話的訂閱登錄，訂閱的使用限制不適用。該流量按令牌計費給擁有 gateway 轉發的憑證的人，例如您的組織的 Anthropic Console 帳戶，或當 gateway 路由到那裡時您的 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 帳戶。

[`ANTHROPIC_BASE_URL`](/docs/zh-TW/llm-gateway-connect#set-the-base-url-and-credential)是指向 Claude Code 指向 gateway 的變數。僅設置該變數而不設置 gateway 憑證不會替換訂閱。請求仍然透過 gateway 路由，但保存的 claude.ai 登錄保持活動憑證，因此其使用限制和計費適用。將此流量轉發給 Anthropic 的 gateway 必須轉發 `anthropic-beta` 中的 OAuth 功能；請參閱[請求標頭參考](/docs/zh-TW/llm-gateway-protocol#request-headers)。

<h2 id="related-pages">
  相關頁面
</h2>

* [Gateway 概述](/docs/zh-TW/gateways)：gateway 如何運作以及如何在 Claude apps gateway 和其他產品之間進行選擇
* [Claude apps gateway](/docs/zh-TW/claude-apps-gateway)：Anthropic 的自託管 gateway，具有 SSO 登錄和 OTLP 遙測
* [將 Claude Code 連接到 LLM gateway](/docs/zh-TW/llm-gateway-connect)：在您自己的機器上設置基本 URL 和憑證，具有每個表面的配置和故障排除表
* [為您的組織推出 LLM gateway](/docs/zh-TW/llm-gateway-rollout)：部署 gateway、發行開發人員憑證和分發受管設定的管理員檢查清單
* [Gateway 協議參考](/docs/zh-TW/llm-gateway-protocol)：Claude Code 發送到 gateway 的內容，供配置 gateway 的操作人員使用，涵蓋端點、要轉發的標頭和功能傳遞
