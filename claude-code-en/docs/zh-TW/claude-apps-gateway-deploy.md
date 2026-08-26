> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude 應用程式閘道部署和運營

> 向您的身份提供者註冊閘道、建置容器、在 Kubernetes 或 Cloud Run 上部署，並運營它：健康檢查、祕密輪換、升級和安全性。

本頁涵蓋執行 [Claude 應用程式閘道](/docs/zh-TW/claude-apps-gateway) 的運營方面：在您的身份提供者 (IdP) 中註冊 OAuth 用戶端、將閘道部署為容器，以及日常運營。關於閘道在啟動時讀取的 `gateway.yaml` 檔案中的每個選項，請參閱 [設定參考](/docs/zh-TW/claude-apps-gateway-config)。

生產部署按順序遵循四個步驟，下面的章節與之相符。前兩個是您做出選擇的地方；後兩個是一旦運行時要查閱的參考資料。

1. [設定您的身份提供者](#identity-provider-setup)：註冊 OAuth 用戶端並檢查 Okta、Entra 和 Google 的各個 IdP 說明
2. [部署閘道](#deployment)：建置固定版本的容器映像並在 Kubernetes、Cloud Run 或您自己的平台上執行。本章節也涵蓋成本、繞過、多閘道和無伺服器決策
3. [設定運營](#operations)：日誌、健康探針、中斷行為、祕密輪換和升級。當您連接監控和運行手冊時的參考資料
4. [檢查安全態勢](#security)：資料流向何處、威脅模型和合規性答案。用於安全審查的參考資料

如果沿途簽入或啟動失敗，請直接前往 [故障排除](#troubleshooting)，該部分根據您看到的錯誤進行索引。

<Note>
  **在您的私有網路上部署。** Claude Code 只連接到地址為私有的閘道。這是一個安全防護，因為受信任的閘道可以推送在開發人員機器上執行命令的設定。將閘道放在內部負載平衡器或 VPN 後面，並給它一個只解析為私有 IP 的主機名。

  Anthropic 運營的公開閘道端點是例外：`/login` 透過 `https://` 接受它們。這些是 Anthropic 本身運營的一小組固定閘道；它們不是您可以選擇或配置的部署選項。該清單已編譯到 Claude Code 中，因此沒有配置可以將主機名新增到其中，您託管的任何閘道都不符合豁免條件。在 v2.1.206 之前，`/login` 像拒絕任何其他公開地址一樣拒絕這些端點。
</Note>

<h2 id="identity-provider-setup">
  身份提供者設定
</h2>

向任何 OIDC 相容的身份提供者註冊機密 OAuth/OpenID Connect (OIDC) 網路應用程式，使用單一重新導向 URI `https://<gateway>/oauth/callback`，並將其分配給應該有閘道存取權限的使用者或群組。

任何 OIDC 相容的 IdP 都可以使用：Okta、Microsoft Entra ID、Google Workspace、Keycloak、Dex、PingFederate 等。IdP 必須滿足三個要求：

* 在生產環境中透過 HTTPS 提供 `/.well-known/openid-configuration`；閘道接受 [`http://` 發行者](/docs/zh-TW/claude-apps-gateway-config#oidc)，環回發行者另外需要 `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1`
* 支援授權碼流程。PKCE（代碼交換的證明金鑰）預設開啟；對於不支援它的 IdP，使用 `oidc.use_pkce: false` 停用它
* 在 id\_token 中傳回 `email` 和可選的 `groups`，或使用 `oidc.userinfo_fallback: true` 從 userinfo 端點提供它們

對於私有 PKI，設定 `oidc.ca_cert_pem`。

一些提供者以不同方式處理電子郵件和群組聲明：

* **Okta**：位於 `https://example.okta.com` 的組織授權伺服器傳回省略 `email` 和 `groups` 的簡化 id\_token，因此當您將其用作 `issuer` 時設定 `oidc.userinfo_fallback: true`。包含 id\_token 中 `email` 和可選 `groups` 的自訂授權伺服器（例如 `https://example.okta.com/oauth2/default`）直接發出它們，不需要回退。Okta 只在 `oidc.scopes` 中請求 `groups` 範圍且應用程式的群組聲明篩選器允許時才發出 `groups`；`userinfo_fallback` 無法填充 IdP 未被要求的聲明。
* **Microsoft Entra ID**：`issuer` = `https://login.microsoftonline.com/<tenant-id>/v2.0`。Entra 發出群組物件 ID 而不是名稱，因此在 `managed.policies.match.groups` 中使用 GUID，或使用應用程式角色以獲得人類可讀的名稱。如果您的租戶在 `roles` 而不是 `groups` 下發出角色，設定 `oidc.groups_claim: roles`。
* **Google Workspace**：`issuer` = `https://accounts.google.com`。Google 的 id\_token 不包含群組。要將基於群組的 `allowed_groups` 或 `managed.policies` 與 Google 作為 IdP 一起使用，請配置 [`oidc.google_groups`](/docs/zh-TW/claude-apps-gateway-config#oidc)，它使用具有網域範圍委派的服務帳戶透過 Admin SDK Directory API 查詢每個使用者的群組。沒有它，使用 `oidc.allowed_email_domains` 進行成員資格閘道和 `managed.policies.match.email_domain` 進行原則分配。Google 也忽略標準 `offline_access` 範圍。對於重新整理令牌，設定 `oidc.scopes: [openid, profile, email]` 和 `oidc.extra_auth_params: { access_type: offline, prompt: consent }`。

如需有關上述未涵蓋的身份提供者的支援，請參閱[疑難排解](#troubleshooting)。

<Warning>
  重新整理令牌讓閘道無聲地更新開發人員的會話，無需將開發人員送回瀏覽器。它們也驅動取消配置，因為當 IdP 停用使用者時，下一次重新整理失敗，會話在 `ttl_hours` 內結束。閘道預設請求 `offline_access` 以獲取重新整理令牌。如果您的 IdP 需要明確同意離線存取，請配置 OAuth 用戶端以允許它。

  如果您的 IdP 根本無法發出重新整理令牌，閘道仍然可以工作，但沒有無聲更新，因此開發人員在會話過期時重新執行瀏覽器登入。為了防止每小時發生一次，將 [`session.ttl_hours`](/docs/zh-TW/claude-apps-gateway-config#session) 提高到 `8` 或 `12`。權衡是取消配置延遲，因為沒有重新整理令牌，被停用的使用者在更長的 TTL 經過之前保持存取權限。
</Warning>

<h2 id="deployment">
  部署
</h2>

閘道是單一 Linux 二進位檔案。它水平擴展，因為副本是無狀態的，Postgres 是共享協調層。以您在環境中執行無狀態服務的任何方式執行它。本章節的其餘部分說明映像需要什麼，並為 Kubernetes 和 Cloud Run 提供簡短說明。

閘道設計為在您的網路內執行，因為它持有您的上游認證並充當推理的單一出口點。它可以在您的開發人員和您的 IdP 可以透過 HTTPS 到達的任何地方執行；將其視為任何其他持有生產認證的服務。

除了它執行的位置之外，還有一些決策塑造部署：

* **成本**：閘道沒有單獨的許可證或按座位費用；它是 `claude` 二進位檔案的一部分。您透過現有的雲端或 Anthropic 承諾為推理付費，加上容器的計算和您的遙測收集器。
* **繞過**：閘道不強制執行通往模型的唯一路由必須通過它。具有自己認證的開發人員仍然可以直接呼叫提供者，因此關閉該路徑是網路原則決策，例如阻止到 `api.anthropic.com` 的出口，除了來自閘道。阻止該出口也會破壞 [WebFetch 網域安全檢查](/docs/zh-TW/data-usage#webfetch-domain-safety-check)，它從每個開發人員的機器呼叫 `api.anthropic.com`；在受管原則中設定 `skipWebFetchPreflight: true` 以停用它。
* **多個閘道**：每個閘道是一個單獨的部署，具有自己的配置。CLI 按閘道主機名儲存其信任指紋和認證，因此不同的團隊可以連接到不同的閘道而不會衝突。要提供多個 OIDC 發行者，請執行單獨的實例。
* **無伺服器**：Cloud Run 可以工作；設定 `min-instances: 1` 以避免冷 OIDC 發現。Lambda 和 Cloud Functions 不行，因為閘道是長時間執行的 HTTP 伺服器。

此處的每個生產拓撲都在純 HTTP 副本前面放置 L7 代理，例如 Ingress、Cloud Run 的前端或 ALB。設定 [`listen.trusted_proxies`](/docs/zh-TW/claude-apps-gateway-config#listen) 為代理的來源範圍，以便閘道從 `X-Forwarded-For` 讀取用戶端 IP。閘道只在 TCP 對等體受信任時才遵守標頭；[Google Cloud 實際工作範例](/docs/zh-TW/claude-apps-gateway-on-gcp) 為每個拓撲提供具體值。沒有受信任的代理，每個請求似乎都來自代理的 IP，這會將按 IP 速率限制摺疊為一個共享桶，並在審計事件中記錄代理的 IP。

<h3 id="container-image">
  容器映像
</h3>

圍繞標準 Claude Code 版本中的原生 `claude` 二進位檔案建置您自己的映像：

1. 從固定版本下載您的映像架構的 Linux 建置；請參閱 [安裝特定版本](/docs/zh-TW/setup#install-a-specific-version) 以取得下載 URL。
2. 根據 [二進位完整性和代碼簽名](/docs/zh-TW/setup#binary-integrity-and-code-signing) 中所述，根據版本的 GPG 簽名 `manifest.json` 驗證它。
3. 將其複製到建置上下文中。

如果您的建置無法到達版本主機，請將版本鏡像到您的內部登錄中，並固定您的機隊執行的版本。

除了二進位檔案，映像還需要：

* **基於 glibc 的映像**：glibc 建置的唯一動態依賴項是 glibc 庫。基於 Musl 的映像需要 `linux-x64-musl` 或 `linux-arm64-musl` 建置加上額外套件；請參閱 [Alpine Linux 設定](/docs/zh-TW/setup#alpine-linux-and-musl-based-distributions)。
* **可寫入的狀態目錄**：閘道以任何使用者身份執行，但最小映像沒有可寫入的主目錄。將 `CLAUDE_CONFIG_DIR` 設定為可寫入的路徑，例如 `/tmp/.claude`。
* **容器命令**：`claude gateway --config /etc/claude/gateway.yaml`，配置檔案以唯讀方式掛載，祕密作為環境變數提供；閘道在 `listen.port` 上監聽，預設為 `8080`。

<h3 id="kubernetes">
  Kubernetes
</h3>

將閘道作為 Deployment 執行，就像任何無狀態服務一樣：

* 從 ConfigMap 掛載配置，從 Secret 掛載祕密；透過 `${file:/path/to/secret}` 或作為環境變數在 YAML 中參考祕密
* 在 Ingress 處終止 TLS 並將 `listen.public_url` 設定為 Ingress 主機名
* 將就緒探針指向 `GET /readyz`，將活躍探針指向 `GET /healthz`

<Note>
  **工作負載身份**

  優先選擇平台的工作負載身份而不是靜態金鑰：EKS 上的 IRSA 用於 Bedrock 和 AWS 上的 Claude Platform，GKE 上的工作負載身份用於 Agent Platform，以及 AKS 上的工作負載身份用於 Foundry。在上游區塊中設定 `auth: {}`，或對 Foundry 設定 `use_azure_ad: true`，閘道透過該提供者的預設認證鏈拾取 Pod 的身份。對於跨雲配對，例如 GKE 上的 Bedrock 上游，在上游的 `auth` 區塊中設定明確認證。[`upstreams` 參考](/docs/zh-TW/claude-apps-gateway-config#upstreams) 有各平台設定詳情。
</Note>

<h3 id="cloud-run">
  Cloud Run
</h3>

按如下方式配置服務：

* 將 `listen.port` 保留在預設值 `8080`，這與 Cloud Run 的預設 `PORT` 相符，或設定 `port: ${PORT}`
* 將 `public_url` 設定為外部可到達的來源。對於生產，這通常是內部負載平衡器的主機名，因為 `/login` [拒絕公開地址](/docs/zh-TW/claude-apps-gateway#prerequisites)，而 `*.run.app` URL 解析為一個，所以單獨的 Cloud Run URL 僅適用於 `curl` 或瀏覽器煙霧測試。例外是一個網路，其中 `*.run.app` 透過 Private Service Connect 和 Cloud DNS 私有區域私下解析；在該拓撲中，Cloud Run URL 是有效的 `public_url`。[Google Cloud 實際工作範例](/docs/zh-TW/claude-apps-gateway-on-gcp#deploy-the-gateway) 涵蓋兩者。
* 將配置掛載為祕密卷
* 設定 `min-instances: 1` 以避免首次請求時的冷 OIDC 發現

<Note>
  如需 Google Cloud 上的完整實際工作範例，涵蓋 Cloud Run 或 GKE、Cloud SQL 和 Secret Manager，請參閱 [在 Google Cloud 上部署](/docs/zh-TW/claude-apps-gateway-on-gcp)。
</Note>

<h3 id="push-the-gateway-url-to-developer-machines">
  將閘道 URL 推送到開發人員機器
</h3>

一旦閘道開始提供服務，透過受管設定、MDM 或直接寫入各個 OS `managed-settings.json` 將 `forceLoginMethod` 和 `forceLoginGatewayUrl` 推送到每個開發人員的機器。沒有這個，`/login` 顯示標準帳戶選擇器，沒有閘道選項。請參閱 [用戶端受管設定](/docs/zh-TW/claude-apps-gateway-config#client-side-managed-settings) 以取得檔案路徑。

<h2 id="operations">
  運營
</h2>

一旦閘道開始提供流量，日常運營就是讀取其日誌、探測其健康狀況，以及按您的時間表輪換其祕密。下面的小節涵蓋每一個，加上 Postgres 持有的內容以及升級和回滾的行為方式。

<h3 id="logs">
  日誌
</h3>

閘道向 stderr 寫入兩個流，都是 JSON 友好的：

* **審計事件**：每個安全相關事件的單行 JSON。將 stderr 管道傳輸到您的日誌聚合器。發出的事件包括 `config.load`、`session.mint`、`session.refresh`、`device.authorize`、`device.verify`、`auth.denied`、`access.denied`、`inference`、`managed.serve`、`spend.blocked` 和 `admin.denied`。欄位因事件而異：
  * 成功的 mint 和 refresh 事件攜帶 `sub`、`email`、`client_ip` 和結果
  * 拒絕事件攜帶原因、路徑和用戶端 IP，因為拒絕時不存在身份
  * `inference` 記錄哪個上游提供了請求以及回應狀態
  * `admin.denied` 記錄被拒絕的管理員 API 驗證嘗試，包括原因（`invalid_key` 或 `no_credentials`）、用戶端 IP、方法和路徑，不包括呈現的金鑰材料
* **運營日誌**：人類可讀的 `[gateway]` 前綴行，用於啟動、警告和上游錯誤。`CLAUDE_GATEWAY_LOG_LEVEL` 環境變數控制詳細程度，接受 `info`、`warn` 或 `error`，預設為 `info`。它不影響審計事件，這些事件始終被發出。

<h3 id="health">
  健康
</h3>

閘道提供 `GET /healthz` 作為活躍探針，`GET /readyz` 作為就緒探針；`/readyz` 驗證存儲是否可到達。兩者都豁免於 `access_control.allow_cidrs`，因此探針在鎖定的監聽器上保持工作。

OAuth 發現文件位於 `/.well-known/oauth-authorization-server` 也只在配置載入、OIDC 發現、上游用戶端構造和 Postgres 遷移全部成功後才傳回 `200`，因此它也充當端到端啟動檢查。

執行中的閘道也在 `<public_url>/protocol` 提供它接受的路徑和請求形狀的描述，與您執行的版本相符。內容在版本之間不穩定。

<h3 id="outage-behavior">
  中斷行為
</h3>

如果 Postgres 宕機，閘道本身繼續為已簽入的開發人員提供服務，新的簽入失敗。開發人員是否實際保持工作取決於您的協調器如何處理就緒：

* **現有會話**：持有人令牌使用 JWT 祕密在本地驗證，會話重新整理不接觸存儲，閘道程序仍然可以提供推理
* **新簽入**：失敗直到 Postgres 恢復，因為設備流及其速率限制計數器存在於 Postgres 中
* **[支出限制執行](/docs/zh-TW/claude-apps-gateway-spend-limits#postgres-availability)**：在中斷期間預設失敗開放，因此推理仍然流動；如果您寧願阻止也不願無計量執行，請將其翻轉為失敗關閉
* **就緒**：`/readyz` 在中斷期間報告未就緒，因此在就緒上閘門流量的協調器一次從輪換中移除每個副本。在該拓撲中，所有流量，包括閘道仍然可以提供的推理，在負載平衡器處失敗，直到 Postgres 恢復。`/healthz` 上的活躍探針保持通過，因此副本不會重新啟動。如果您寧願已簽入的開發人員在存儲中斷期間保持工作，請將就緒探針指向 `/healthz`；成本是新簽入失敗對仍然報告就緒的副本。

如果您的 IdP 宕機，現有會話工作直到 `ttl_hours`，新登入和重新整理失敗。如果您的 IdP 有頻繁的維護窗口，設定更長的 `ttl_hours`。

<h3 id="jwt-secret-rotation">
  JWT 祕密輪換
</h3>

分三個步驟輪換簽名祕密，以便現有會話保持有效：

1. 生成新祕密。將其前置到 `session.jwt_secret` 陣列。
2. 滾動部署。新令牌使用新祕密簽名；舊令牌仍然驗證。
3. 在 `ttl_hours` 加上邊距後，移除舊祕密並再次滾動。

輪換也是在它們過期之前強制會話退出的唯一方法：持有人令牌根據 JWT 祕密在本地驗證，因此沒有按會話撤銷。直接替換祕密，不在陣列中保留舊祕密，一次使每個未完成的會話無效。對於個別離職，在您的 IdP 中取消配置使用者；他們的會話在 `ttl_hours` 內結束。

<h3 id="postgres">
  Postgres
</h3>

閘道持有五個表，全部由其啟動時遷移建立：

| 表                  | 內容                                  | 保留                                            |
| ------------------ | ----------------------------------- | --------------------------------------------- |
| `kv`               | 設備授予（10 分鐘 TTL）和速率限制計數器             | 每行 TTL                                        |
| `spend`            | 按主體期間至今支出計數器，以美分計                   | `admin.spend_retention_months`，預設 13          |
| `spend_limits`     | 配置的支出上限                             | 直到透過 API 刪除                                   |
| `admin_audit`      | 管理員 API 變更軌跡                        | `admin.audit_retention_days`，預設 365           |
| `principal_emails` | 每個主體的最後看到的電子郵件、顯示名稱和 IdP 群組。包含 PII。 | `admin.identity_retention_days` 自上次活動以來，預設 90 |

30 秒迴圈過期 `kv` 行超過其 TTL，每小時掃描在支出表上強制保留窗口，因此沒有任何東西無限增長。沒有 [支出限制](/docs/zh-TW/claude-apps-gateway-spend-limits) 配置，只有 `kv` 被寫入。如果您的安全原則禁止應用程式角色的 DDL，預先建立這些表和 `_migrations`，使用管理員角色，並授予應用程式角色 `SELECT, INSERT, UPDATE, DELETE` 在每個上。

使用支出限制，丟失的資料庫意味著丟失支出追蹤和上限，不僅僅是開發人員重新登入，因此執行定期備份。要立即清除一個已離職的開發人員，而不是等待保留，直接執行 `DELETE FROM principal_emails WHERE principal = '<sub>'`；這移除唯一持有其電子郵件、名稱和群組的表。`spend` 和 `admin_audit` 行僅參考偽匿名 OIDC `sub`。

<h3 id="upgrades">
  升級
</h3>

副本是無狀態的，因此滾動重新啟動在任何時間都是安全的。閘道在啟動時執行架構遷移，這意味著部署新二進位檔案自我遷移資料庫。如果資料庫角色無法執行 DDL，預先建立架構，包括 `_migrations` 表，種子為當前版本；否則啟動失敗，嘗試 `CREATE TABLE`。

遷移是僅附加的，因此回滾到知道較少遷移的先前二進位檔案是安全的；它忽略額外的行。回滾也根據較舊二進位檔案的架構重新驗證 YAML，因此採用較新版本引入的金鑰的配置在較舊版本上啟動失敗。在回滾之前移除新金鑰。

因為您在自己的映像中固定閘道的版本，新 Claude Code 版本中的修復，包括安全修復，只有在您更新固定並重新部署時才會到達您的部署。將閘道包含在您用於持有生產認證的其他服務的相同修補週期中。

<h2 id="security">
  安全
</h2>

本章節回答安全審查提出的問題：什麼資料流經閘道以及它流向何處、設計防禦的攻擊，以及哪些答案屬於合規性問卷。

<h3 id="data-flow">
  資料流
</h3>

| 資料                                                                      | 路徑                                     | 由閘道發送給 Anthropic          |
| ----------------------------------------------------------------------- | -------------------------------------- | ------------------------- |
| 推理（提示、完成）                                                               | CLI → 閘道 → 您的上游                        | 只有在 Anthropic API 是配置的上游時 |
| 遙測（OTLP 指標，加上 [選擇加入日誌和追蹤](/docs/zh-TW/claude-apps-gateway-config#telemetry)） | CLI → 閘道 → 您的收集器                       | 從不                        |
| 身份（電子郵件、群組、sub）                                                         | IdP → 閘道 → JWT → CLI；CLI 在 OTLP 匯出上標記它 | 從不                        |
| 受管設定                                                                    | 您的閘道 YAML → CLI                        | 從不                        |
| 審計日誌                                                                    | 閘道 stderr → 您的聚合器                      | 從不                        |

<h3 id="threat-model-summary">
  威脅模型摘要
</h3>

閘道位於您的網路周邊內，但個別開發人員筆記型電腦不被視為受信任。設計以三種方式考慮這一點：

* 開發人員持有短期 JWT 而不是原始上游金鑰。CLI 到閘道的腿使用 RFC 8628 設備授予，閘道與 IdP 的授權碼交換在預設配置中執行 PKCE，因此攔截的 IdP 授權碼是無用的。
* 設備驗證頁面強制執行同源 POST 和根據 RFC 8628 §5.1 的每 IP 速率限制。請參閱 [使用者代碼暴力破解抵抗](#user-code-brute-force-resistance)。
* 出站請求通過伺服器端請求偽造 (SSRF) 防護，解析 DNS、阻止連結本地和雲端中繼資料地址加上預設環回，並將連接固定到解析的 IP，因此操作員影響的 URL（例如 IdP 和 OTLP 目的地）無法重新導向到雲端中繼資料端點。RFC 1918 私有範圍被刻意允許，因為 IdP 和 OTLP 收集器通常存在於私有 IP 上。對於針對環回 IdP 或收集器的本地開發，在閘道的環境中設定 `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1`；在生產中保持未設定。

如果您新增自己的出口控制，閘道必須在使用工作負載身份等實例中繼資料認證時到達中繼資料伺服器。

兩個威脅超出範圍，因為它們是您的基礎設施要保護的：

* **受損的閘道主機**：主機既持有上游認證，又向每個連接的開發人員分發 [受管設定](/docs/zh-TW/claude-apps-gateway-config#managed)，因此對閘道配置的控制與對您的 MDM 的控制相當。CLI 的一次性批准對話框用於殼層功能設定限制無聲變更，但不替代主機安全。
* **惡意 OIDC 提供者**：提供者簽署閘道信任的 id\_token，因此它可以聲稱任何身份。審查和保護您的 IdP 是您的責任。

<h3 id="user-code-brute-force-resistance">
  使用者代碼暴力破解抵抗
</h3>

開發人員在 `/device` 驗證頁面中輸入的 `user_code` 是從 20 字元字母表中抽取的 8 個字元，產生 20⁸ 或約 2.56×10¹⁰ 個組合，並在 10 分鐘後過期。

閘道在設備授予端點上應用按 IP 速率限制，可透過 [`rate_limits`](/docs/zh-TW/claude-apps-gateway-config#http-tuning) 配置。如果許多開發人員從單一共享公司 NAT 地址簽入，請提高限制。限制僅適用於簽入流程，不適用於推理。

<h3 id="compliance-posture">
  合規性態勢
</h3>

* **資料駐留**：閘道自己的資料平面不向 Anthropic 發送任何東西，除非 Anthropic API 是配置的上游；當它是時，您現有的資料處理協議適用於推理路徑。遙測、審計、身份和設定只流向您配置的目的地。
* **主機程序流量**：主機程序是 Claude Code CLI，可以向 Anthropic 發送啟動分析和更新檢查。對於嚴格出口部署，在閘道的容器環境中設定 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1`。
* **用戶端分析**：CLI 在簽入到閘道時停用自己的使用分析，錯誤報告在第三方 API 表面上預設關閉。
* **用戶端機器**：開發人員的 CLI 仍然向 Anthropic 發送 WebFetch 主機名檢查和版本檢查，除非設定 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1` 和 `skipWebFetchPreflight: true`。請參閱 [資料使用](/docs/zh-TW/data-usage)。
* **調查評分**：閘道認證停用 Anthropic 綁定評分接收器，因此評分不發送給 Anthropic。
* **記錄單共享**：在調查的記錄單共享提示上選擇「是」會在 `~/.claude/feedback-bundles/` 下寫入本地檔案，而不是上傳到 Anthropic。
* **用戶端更新**：更新檢查與閘道流量分開。透過您自己的分發固定版本，如果筆記型電腦不得提取版本，設定 `DISABLE_UPDATES`。`DISABLE_AUTOUPDATER` 只停止背景更新，而 `claude update` 仍然有效。
* **TLS**：在生產中透過 HTTPS 提供 `public_url`，要麼從閘道自己的監聽器透過 `listen.tls`，要麼從 TLS 終止 ingress 在純 HTTP 副本前面，設定 `listen.public_url`。閘道不拒絕純 HTTP。IdP 必須在生產中提供 HTTPS，Postgres 支援 `?sslmode=require`。在您的 ingress 設定 `Strict-Transport-Security`。
* **漏洞披露**：遵循 [報告安全問題](/docs/zh-TW/security#reporting-security-issues)

<h2 id="troubleshooting">
  故障排除
</h2>

如有問題和反饋，請使用 [Claude Code 支援](https://support.claude.com/en/collections/14445694-claude-code)，或在 [Claude Code GitHub 儲存庫](https://github.com/anthropics/claude-code/issues) 上開啟問題。報告問題時，請包括：

* **閘道問題**：相關窗口的閘道 stderr、您的 `gateway.yaml`（祕密已編輯）、閘道版本（顯示在 `/` 的登陸頁面和 `/managed/settings` 上的 `x-cc-gateway-version` 回應標頭中），以及最近變更的內容
* **登入問題**：開發人員執行 `claude --debug-file ./claude-debug.txt`、重現，並發送該檔案加上相同窗口的閘道審計日誌
* **推理問題**：請求的模型、配置的上游，以及請求的閘道審計日誌，記錄哪個上游提供了它以及回應狀態

閘道的 stderr 包含審計事件串流，審計日誌記錄開發人員身分，除錯檔案記錄來自開發人員機器的 hook 和 MCP 伺服器輸出。在發佈到公開議題之前，請檢查並隱蔽這些資訊。

| 症狀                                                                                                                                        | 原因                                                                                                                                                                                                              | 修復                                                                                                                                                                                                                                                    |
| ----------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 開發人員的 `/login` 顯示標準帳戶選擇器而不是 **Cloud 閘道** 螢幕                                                                                               | `forceLoginMethod` 或 `forceLoginGatewayUrl` 未在該機器上的受管設定中設定                                                                                                                                                      | 將 [受管設定檔案](/docs/zh-TW/claude-apps-gateway#set-the-gateway-url) 部署到設備；`/login` 從那裡讀取閘道 URL                                                                                                                                                                 |
| 啟動顯示 `Gateway login is configured in managed settings, but this Claude Code build does not include Cloud gateway support.`                | 已安裝的 Claude Code 建置早於閘道支援                                                                                                                                                                                       | 讓開發人員更新 Claude Code 到包含 Cloud 閘道支援的版本                                                                                                                                                                                                                 |
| CLI `/login`：`Gateway hosts must be on your organization's private network; <host> resolves to the public (or unrecognized) address <ip>` | 閘道主機名解析為至少一個公開 IP 地址。Claude Code 檢查每個解析的地址，並要求每個都是私有的。常見原因是雙堆棧名稱，其中一個系列解析為公開地址，包括 AWS 內部雙堆棧負載平衡器，它們傳回公開範圍 AAAA 地址。Anthropic 營運的公開閘道端點豁免於檢查，`/login` 透過 `https://` 接受它們。在 v2.1.206 之前，`/login` 拒絕它們，就像任何其他公開地址一樣 | 讓閘道名稱在開發人員機器上只解析為私有地址。對於雙堆棧名稱，刪除公開範圍記錄或提供單獨的僅內部 DNS 名稱。請參閱 [私有網路先決條件](/docs/zh-TW/claude-apps-gateway#prerequisites)。                                                                                                                                      |
| CLI `/login`：`Gateway login requires a direct connection and does not support connecting through an HTTP proxy`                           | `HTTPS_PROXY` 或 `HTTP_PROXY` 適用於閘道主機，代理的主機名解析為公開地址。代理的主機解析為僅私有地址是允許的，不會觸發此錯誤                                                                                                                                    | 在開發人員的機器上將閘道主機新增到 `NO_PROXY`，以便連接是直接的，或使用主機名解析為私有地址的代理                                                                                                                                                                                                |
| CLI `/login`：`Could not resolve gateway host <host>`                                                                                      | 機器無法解析閘道的內部 DNS 名稱，通常是因為它不在公司網路上                                                                                                                                                                                | 讓開發人員連接到您的網路或 VPN，然後重試 `/login`                                                                                                                                                                                                                       |
| 啟動退出，配置驗證錯誤命名 `store.postgres_url`                                                                                                        | 未配置 Postgres；閘道需要 Postgres                                                                                                                                                                                      | 設定 `store.postgres_url`。對於本地開發，使用一次性容器：`docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`。                                                                                                                                    |
| 啟動退出：`requires the native binary`                                                                                                         | 在 Node 下執行而不是原生二進位檔案                                                                                                                                                                                            | 使用其中一種 [獨立安裝方法](/docs/zh-TW/setup) 安裝 Claude Code                                                                                                                                                                                                          |
| 啟動退出，OIDC 發現錯誤在 `config.load` 之後                                                                                                          | `oidc.issuer` 無法到達，或 TLS 鏈不受信任                                                                                                                                                                                  | 檢查發行者是否可從 Pod 到達並提供 `/.well-known/openid-configuration`。為私有 PKI 設定 `ca_cert_pem`。                                                                                                                                                                     |
| 啟動退出，Postgres 權限錯誤                                                                                                                        | 應用程式角色缺少 `CREATE TABLE`                                                                                                                                                                                         | 使用管理員角色預先建立架構，並授予應用程式角色 DML，或臨時授予 DDL 以進行應用新遷移的啟動                                                                                                                                                                                                     |
| `/oauth/callback` 顯示「Sign-in could not be completed」                                                                                      | 電子郵件網域被拒絕、id\_token 驗證失敗，或 `email_verified` 明確為 `false`，閘道始終拒絕，沒有覆蓋                                                                                                                                             | 檢查 `allowed_email_domains` 以及 IdP 是否傳回驗證的 `email` 聲明。對於 `email_verified: false`，修復 IdP 端驗證。如果您的 IdP 在不同的聲明名稱下發出電子郵件，設定 `oidc.email_claim`。                                                                                                            |
| 日誌：`token exchange failed: id_token missing email claim`                                                                                  | IdP 預設不在 id\_token 中包含 `email`。此拒絕僅在設定 `allowed_email_domains` 時觸發；沒有它，缺失的電子郵件鑄造沒有電子郵件的會話                                                                                                                       | 配置 IdP 在 id\_token 中發出 `email`。Okta：將 `email` 新增到自訂授權伺服器的 ID 令牌聲明。Entra：在應用程式註冊上新增 `email` 作為可選聲明。PingFederate：啟用發出 `email` 的 OpenID Connect 原則。如果 IdP 從 userinfo 端點提供 `email` 但不會在 id\_token 中包含它，例如 Okta 組織授權伺服器，設定 `oidc.userinfo_fallback: true`。 |
| 每個 Amazon Bedrock 請求傳回 502；日誌顯示 `Could not load credentials from any providers`                                                           | 在 EC2 上，IMDSv2 的預設躍點限制 1 阻止來自容器內的實例中繼資料請求。啟動和 `/readyz` 仍然通過，因為 AWS SDK 在第一個請求時解析實例認證，而不是在用戶端構造時                                                                                                                | 使用 `aws ec2 modify-instance-metadata-options --instance-id <id> --http-put-response-hop-limit 2` 提高躍點限制，或在啟動範本中設定它。變更適用於實例上的每個容器。優先選擇 ECS 任務角色（如果可用），它從 ECS 容器認證端點讀取認證，完全避免變更，或在專用閘道實例上應用變更以限制暴露。                                                     |
| IdP 錯誤：unknown or unsupported scope                                                                                                       | IdP 拒絕它不識別的範圍                                                                                                                                                                                                   | 將 `oidc.scopes` 設定為您的 IdP 接受的確切清單；它必須包含 `openid`。預設為 `openid profile email offline_access`。                                                                                                                                                           |
| 設定 `oidc.scopes` 後會話不無聲更新                                                                                                                 | `offline_access` 從覆蓋中被刪除                                                                                                                                                                                        | 如果您的 IdP 支援，新增 `offline_access` 回來。沒有重新整理令牌，開發人員每 `session.ttl_hours` 重新執行瀏覽器登入。                                                                                                                                                                      |
| 瀏覽器顯示「This request came from another site and was blocked」                                                                                | 跨網站表單 POST，被阻止作為 CSRF 保護。嵌入或代理頁面的預期                                                                                                                                                                             | 直接開啟驗證連結                                                                                                                                                                                                                                              |
| Chrome 使用「Refused to send form data … violates … Content Security Policy directive: form-action」阻止「批准」按鈕，但相同頁面在 Safari 或 Firefox 中有效      | Chrome 對整個重新導向鏈強制執行 `form-action`。您的 IdP 重新導向到不在允許清單中的第二個主機。                                                                                                                                                    | 在 `oidc.form_action_origins` 中新增重新導向鏈中的每個額外來源。在「批准」頁面上開啟 Chrome DevTools → 主控台以查看哪個來源被阻止。                                                                                                                                                             |
| 簽入在 IdP 完成但回調失敗，Chrome 中出現 CSP 錯誤或 Safari 中出現「this sign-in link has expired」                                                              | IdP 透過 `response_mode=form_post` 傳回代碼，它透過 POST 自動提交到 `/oauth/callback`。Chrome 在嚴格 CSP 下阻止它；Safari 允許提交但回調只讀取查詢字串。                                                                                               | 確保您的 IdP 遵守 `response_mode=query`，閘道明確請求它，以便回調是純重新導向                                                                                                                                                                                                  |
| 登入在本地有效但在 ALB 後面失敗                                                                                                                        | `public_url` 未設定，因此 IdP 獲得內部 `http://` 來源作為 `redirect_uri`                                                                                                                                                      | 將 `listen.public_url` 設定為外部 `https://` 來源                                                                                                                                                                                                             |
| 開發人員重複看到信任提示                                                                                                                              | TLS 憑證按副本或按請求輪換                                                                                                                                                                                                 | 在 ingress 使用穩定憑證，或終止 TLS 一次並在內部透過純 HTTP 執行副本                                                                                                                                                                                                          |
| CLI `/login`：「Could not verify the gateway's TLS certificate」或 `SELF_SIGNED_CERT_IN_CHAIN`                                                | 閘道的 TLS 鏈由 CLI 主機的信任存儲中不存在的私有 CA 簽署                                                                                                                                                                             | Claude Code 預設在原生二進位檔案上讀取 OS 信任存儲，在 Node 22.15 或更新版本上；[`CLAUDE_CODE_CERT_STORE`](/docs/zh-TW/network-config#ca-certificate-store) 控制此行為。如果 CA 安裝在 OS 信任存儲中，確保開發人員在當前執行時上。否則在啟動前將 `NODE_EXTRA_CA_CERTS` 設定為 CA 憑證 PEM。首次連接指紋提示仍然適用。                         |

<h2 id="related">
  相關
</h2>

* [Claude 應用程式閘道概述](/docs/zh-TW/claude-apps-gateway)：快速入門和開發人員連接
* [設定參考](/docs/zh-TW/claude-apps-gateway-config)：每個 `gateway.yaml` 選項
