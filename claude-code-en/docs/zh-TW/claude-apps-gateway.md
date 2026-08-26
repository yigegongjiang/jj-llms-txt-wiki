> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Amazon Bedrock、Claude Platform on AWS、Google Cloud 和 Microsoft Foundry 的 Claude 應用程式閘道

> 透過自託管閘道在 Amazon Bedrock、Claude Platform on AWS、Google Cloud 或 Microsoft Foundry 上執行 Claude Code，具備 SSO 登入、按群組模型存取和 OTLP 遙測功能。

<Note>
  Claude 應用程式閘道是為必須或偏好透過自己的雲端提供商路由推論的組織而設計的，例如為了滿足[資料駐留](/docs/zh-TW/claude-apps-gateway-deploy#compliance-posture)要求。如果您沒有此要求，並且想要存取其他功能，例如 SCIM 佈建或 Claude Code 網頁和行動版本，Claude Enterprise 可能更適合。請參閱[功能可用性](/docs/zh-TW/feature-availability)頁面，以取得所有部署方法的完整比較。
</Note>

Claude 應用程式閘道是一個自託管服務，位於開發人員的 Claude Code 用戶端和模型提供商之間。開發人員使用您的企業身份提供商 (IdP) 登入，而不是持有 API 金鑰或雲端認證。閘道持有上游認證，按 IdP 群組強制執行模型存取和[受管設定](/docs/zh-TW/permissions#managed-settings)，並將使用情況遙測轉發到您自己的可觀測性堆疊。

它包含在 `claude` 二進位檔中，因此在筆記型電腦上執行 Claude Code 的相同可執行檔可以使用 `claude gateway --config gateway.yaml` 執行閘道伺服器。

本頁涵蓋：

* [為什麼使用 Claude 應用程式閘道](#why-claude-apps-gateway)、它相比自行執行增加了什麼，以及何時其他解決方案更合適
* 一個[快速入門](#quickstart)，包含[先決條件](#prerequisites)，可將閘道從零開始設定為已登入的開發人員
* [連接開發人員](#connect-developers)，包括透過受管設定設定閘道 URL
* [可用性和限制](#availability-and-limitations)，涵蓋哪些 Claude Code 功能可透過閘道運作，以及伺服器支援什麼

相關頁面會更深入地介紹。[配置參考](/docs/zh-TW/claude-apps-gateway-config)涵蓋快速入門寫入的 YAML 檔案中的每個選項，[部署指南](/docs/zh-TW/claude-apps-gateway-deploy)涵蓋每個 IdP 的設定、Kubernetes 和 Cloud Run 部署，以及操作。

<h2 id="why-claude-apps-gateway">
  為什麼使用 Claude 應用程式閘道
</h2>

[閘道概述](/docs/zh-TW/gateways)涵蓋閘道的功能以及為什麼要執行一個。Claude 應用程式閘道是 Anthropic 自己的閘道，內建於 `claude` 二進位檔中，並與每個 Claude Code 版本一起測試，因此它轉發 Claude Code 發送的標頭和請求欄位，無需操作員維護單獨的允許清單。部署後，它為您提供：

* **認證**：上游 API 金鑰或雲端認證僅存在於您的基礎設施中。開發人員使用公司 SSO 進行身份驗證並接收短期的持有人令牌，因此離職發生在您的 IdP 中。取消佈建使用者，其閘道存取在會話生命週期內過期，預設為一小時。
* **存取控制**：您的 IdP 群組對應到模型允許清單和[受管設定](/docs/zh-TW/permissions#managed-settings)原則。閘道在伺服器端強制執行模型存取，拒絕非授予模型的請求，並選擇每個群組的受管設定原則，CLI 在[受管設定層級](/docs/zh-TW/settings#settings-precedence)應用該原則。不同的團隊獲得不同的模型、工具和權限，開發人員無法覆蓋其原則鎖定的內容。
* **設定傳遞**：閘道本身將受管設定傳遞給已登入的用戶端，取代來自 claude.ai 管理員主控台的[伺服器管理設定](/docs/zh-TW/server-managed-settings)。
* **遙測**：每個配置的目的地（例如 Datadog、Splunk 或 ClickHouse）接收[OpenTelemetry Protocol (OTLP) 指標](/docs/zh-TW/monitoring-usage)，預設包含令牌計數、模型、使用者身份和延遲，日誌和追蹤作為按目的地的選擇加入。
* **上游路由**：用戶端向閘道說 Anthropic Messages API，閘道為每個上游進行轉換，無論是 Amazon Bedrock、[AWS 上的 Claude Platform](/docs/zh-TW/claude-platform-on-aws)、Google Cloud 的 Agent Platform、Microsoft Foundry 或 Anthropic API，並在它們之間進行故障轉移。您可以更改區域、提供商或故障轉移順序，而開發人員無需注意或重新配置。

<Frame>
  <img src="https://mintcdn.com/claude-code/VbyXug8hBU9UK6oT/images/claude-gateway-architecture.svg?fit=max&auto=format&n=VbyXug8hBU9UK6oT&q=85&s=9e4f1190fc56718144190a3db61c63af" alt="圖表顯示 Claude Code 用戶端透過 HTTPS 和持有人令牌連接到您基礎設施內的自託管 Claude 應用程式閘道，該閘道針對您的 IdP 簽署使用者，在 PostgreSQL 中儲存身份驗證狀態，將遙測轉發到您的 OTLP 收集器，並將推理轉發到 Amazon Bedrock、AWS 上的 Claude Platform、Google Cloud、Microsoft Foundry 或 Anthropic API" width="760" height="320" data-path="images/claude-gateway-architecture.svg" />
</Frame>

<Note>
  閘道自己的資料平面不會向 Anthropic 基礎設施發送任何內容，除非 Anthropic API 是配置的上游。您控制遙測、稽核日誌、受管設定和開發人員的 IdP 身份去向，閘道不會將它們中的任何一個發送給 Anthropic。對於其餘流量，CLI 程序可以發送什麼以及如何關閉它，請參閱[合規性態勢](/docs/zh-TW/claude-apps-gateway-deploy#compliance-posture)。
</Note>

有關哪些 Claude Code 功能可透過閘道運作以及伺服器本身支援什麼，請參閱下面的[可用性和限制](#availability-and-limitations)。有關成本、繞過、執行多個閘道和無伺服器平台等決策，請參閱[部署指南](/docs/zh-TW/claude-apps-gateway-deploy#deployment)。

<h3 id="other-gateway-implementations">
  其他閘道實現
</h3>

如果您已經執行滿足您需求的 LLM 閘道或 API 閘道，請繼續使用它；[其他 LLM 閘道](/docs/zh-TW/llm-gateway)涵蓋針對它配置 Claude Code。

[閘道協議參考](/docs/zh-TW/llm-gateway-protocol)記錄了 Claude Code 期望從任何閘道的合約：它呼叫的端點、要轉發的標頭和正文欄位，以及當它們被剝離時停止運作的內容。執行中的 Claude 應用程式閘道在 `GET /protocol` 提供該合約的超集，添加 Claude 應用程式閘道特定的端點用於 SSO 登入、受管設定傳遞和遙測。使用 `curl https://claude-gateway.internal.example.com/protocol` 從任何部署的閘道（例如下面[快速入門](#quickstart)產生的閘道）獲取它。協議的重大變更會提前宣佈，但不保證無限期的向後相容性。

<h2 id="quickstart">
  快速入門
</h2>

此快速入門走最小路徑：在您的 IdP 中註冊 OAuth 用戶端，寫入 `gateway.yaml`，使用 Docker Compose 與 Postgres 一起執行閘道，並驗證端到端登入。它使用 Amazon Bedrock 上游；Claude Platform on AWS、Google Cloud 的 Agent Platform、Microsoft Foundry 和 Anthropic API 同樣受支援，只需如[配置參考](/docs/zh-TW/claude-apps-gateway-config#upstreams)所示交換 `upstreams` 區塊。最後，您有一個開發人員可以 `/login` 的閘道。

<Note>
  **在您的私有網路上部署。** Claude Code 只連接到地址為私有的閘道。這是一個安全防護，因為受信任的閘道可以推送在開發人員機器上執行命令的設定。將閘道放在內部負載平衡器或 VPN 後面，並給它一個只解析為私有 IP 的主機名。

  Anthropic 營運的公開閘道端點是例外：`/login` 透過 `https://` 接受它們。這些是 Anthropic 本身營運的一小組固定閘道；它們不是您可以選擇或配置的部署選項。清單編譯到 Claude Code 中，因此沒有配置可以將主機名新增到其中，您託管的任何閘道都不符合豁免資格。在 v2.1.206 之前，`/login` 像任何其他公開地址一樣拒絕這些端點。
</Note>

<h3 id="prerequisites">
  先決條件
</h3>

在開始之前，請準備好以下內容：

| 您需要                         | 詳細資訊                                                                                                                                                                                                                                                                                                                                                                             |
| --------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude Code v2.1.195 或更新版本  | `claude gateway` 子命令和閘道登入流程在 v2.1.195 中發布。較早的公開版本不包含它們。執行閘道伺服器的機器和每個開發人員的機器都必須是 v2.1.195 或更新版本；執行 `claude update` 以取得最新版本。[Claude Platform on AWS 上游](/docs/zh-TW/claude-apps-gateway-config#claude-platform-on-aws)在閘道伺服器上需要 Claude Code v2.1.198 或更新版本。                                                                                                                             |
| OpenID Connect (OIDC) 身份提供商 | Okta、Microsoft Entra ID、Google Workspace、Keycloak 或 Dex，或任何其他符合 OIDC 的 IdP，例如 PingFederate。閘道針對它執行標準 OIDC 發現和授權碼流程。不支援 SAML 和 LDAP。                                                                                                                                                                                                                                              |
| PostgreSQL 14 或更新版本         | 支援裝置登入流程，其中瀏覽器回呼寫入，輪詢 CLI 讀取，加上速率限制計數器。任何受管 Postgres 都可以，包括最小層級。在未配置支出限制的情況下，閘道儲存幾 KB 的短期身份驗證狀態；使用[支出限制](/docs/zh-TW/claude-apps-gateway-spend-limits)，它還持有應備份的耐久支出、稽核和身份表。建議透過 `?sslmode=require` 使用 TLS。                                                                                                                                                                            |
| 模型上游                        | Amazon Bedrock 認證、Claude Platform on AWS 認證、Google Cloud 認證、Microsoft Foundry 資源或 Anthropic API 金鑰。支援多個上游和故障轉移。                                                                                                                                                                                                                                                                  |
| HTTPS                       | 閘道必須可從開發人員筆記型電腦和用於登入的任何瀏覽器透過 `https://` 到達；閘道在同一監聽器上提供裝置驗證頁面。透過 `listen.tls` 提供 TLS 憑證，或在 TLS 終止入口後執行並設定 `listen.public_url`。純 `http://` 來源僅在本地開發的環回上接受。                                                                                                                                                                                                                         |
| 私有網路地址                      | 在 `/login` 處，Claude Code 要求閘道的主機名或 IP 地址僅解析為私有地址：RFC 1918、CGNAT `100.64.0.0/10`、IPv6 ULA `fc00::/7` 或本地開發的環回。檢查在每個解析的 IP 上執行，因此如果名稱解析到的任何地址是公開的，`/login` 會拒絕該 URL。如果開發人員機器透過公司代理路由 HTTPS，登入還要求代理主機解析為私有地址；如果不是，將閘道主機新增到 `NO_PROXY`，以便 CLI 直接連接。Anthropic 營運的公開閘道端點豁免於私有地址和代理檢查：`/login` 透過精確主機名符合接受它們，因此私有網路要求僅適用於您自己託管的閘道。在 v2.1.206 之前，`/login` 像任何其他公開地址一樣拒絕 Anthropic 營運的端點。 |
| Linux 執行時                   | 閘道伺服器僅在原生 Linux 二進位檔上執行。macOS 適用於本地開發。Windows 不支援作為伺服器平台。                                                                                                                                                                                                                                                                                                                        |

閘道伺服器需要原生 `claude` 二進位檔；如[安裝 Claude Code](/docs/zh-TW/setup) 中所述下載固定版本。伺服器使用在 Claude Code 在 Node 下執行時不可用的執行時功能。如果您在啟動時看到 `requires the native binary`，請切換到其中一個獨立安裝方法。

<h3 id="steps">
  步驟
</h3>

<Steps>
  <Step title="在您的 IdP 中註冊 OAuth 用戶端">
    首先決定閘道的主機名，因為重定向 URI 必須與其匹配。建立新的 OIDC Web 應用程式，並將重定向 URI 設定為 `https://claude-gateway.<your-domain>/oauth/callback`，其中主機是您在步驟 3 中設定為 [`listen.public_url`](/docs/zh-TW/claude-apps-gateway-config#listen) 的相同值。記下 `client_id` 和 `client_secret`。每個 IdP 的說明在[身份提供商設定](/docs/zh-TW/claude-apps-gateway-deploy#identity-provider-setup)中。
  </Step>

  <Step title="佈建 PostgreSQL 資料庫">
    任何 Postgres 14 或更新版本都可以，包括最小受管層級。閘道在啟動時執行自己的架構遷移，因此資料庫使用者需要 `CREATE TABLE` 權限。如果您的安全原則禁止應用程式角色的 DDL，請改為預先建立架構；請參閱 [`store`](/docs/zh-TW/claude-apps-gateway-config#store)。
  </Step>

  <Step title="寫入 gateway.yaml">
    機密透過 `${ENV_VAR}` 擴展讀取，因此檔案本身可以存在於版本控制中。使用在您的網路上解析為私有 IP 的 `public_url` 主機名，因為 `/login` 拒絕公開地址。最小配置有五個部分，其他每個欄位都有預設值：

    ```yaml gateway.yaml theme={null}
    listen:
      host: 0.0.0.0
      port: 8080
      # 在任何 TLS 終止代理後面需要。用於 IdP
      # redirect_uri 和發現文件。
      public_url: https://claude-gateway.internal.example.com

    oidc:
      issuer: https://login.example.com        # 必須提供 /.well-known/openid-configuration
      client_id: 0oa1example2
      client_secret: ${OIDC_CLIENT_SECRET}
      allowed_email_domains: [example.com]        # 拒絕組織外的 id_tokens
      userinfo_fallback: true                  # 對於 id_token 省略電子郵件/群組的 IdP；否則無害

    session:
      jwt_secret: ${GATEWAY_JWT_SECRET}        # openssl rand -base64 32
      ttl_hours: 1                             # 也限制 IdP 取消佈建時的撤銷延遲

    store:
      postgres_url: ${GATEWAY_POSTGRES_URL}    # 為受管 Postgres 新增 ?sslmode=require

    upstreams:
      - provider: bedrock
        region: us-east-1
        auth: {} # 空：AWS 預設認證鏈
    # (IRSA、EC2/ECS 任務角色、環境變數、~/.aws)

    # 模型會自動按上游轉換。內建目錄
    # 將 claude-opus-4-8 對應到 us.anthropic.claude-opus-4-8 等，適用於每個
    # Bedrock 支援的 Claude 模型。設定為 false 並新增 `models:` 清單以
    # 僅公開特定模型。
    auto_include_builtin_models: true
    ```

    此配置足以使用預設 Amazon Bedrock 模型目錄進行有效的登入迴圈。執行後，透過 [`managed.policies`](/docs/zh-TW/claude-apps-gateway-config#managed) 新增按群組 RBAC 和受管設定、透過 [`telemetry`](/docs/zh-TW/claude-apps-gateway-config#telemetry) 的遙測扇出，以及多上游故障轉移、佈建輸送量 ARN 或非美國區域，透過 [`models`](/docs/zh-TW/claude-apps-gateway-config#models)。

    <Note>
      Bedrock 上游需要一個 AWS 主體，具有 `bedrock:InvokeModel` 和 `bedrock:InvokeModelWithResponseStream` 在 `inference-profile/us.anthropic.*` ARN 和基礎 `foundation-model/anthropic.*` ARN 上，以及在 Bedrock 主控台中為您想要的 Claude 模型啟用的模型存取。使用 EKS 上的 IRSA、ECS 任務角色或 EC2 執行個體設定檔提供認證，而不是靜態金鑰。[`upstreams` 參考](/docs/zh-TW/claude-apps-gateway-config#upstreams)具有完整的 IAM 詳細資訊、跨雲認證矩陣和其他提供商的 `auth` 區塊。
    </Note>
  </Step>

  <Step title="執行它">
    圍繞滿足[映像要求](/docs/zh-TW/claude-apps-gateway-deploy#container-image)的 `claude` 二進位檔建立容器映像，然後與 Postgres 一起執行它：

    ```yaml docker-compose.yaml theme={null}
    services:
      gateway:
        image: <your-registry>/claude-gateway:<version>
        ports: ["8080:8080"]
        volumes: ["./gateway.yaml:/etc/claude/gateway.yaml:ro"]
        environment:
          OIDC_CLIENT_SECRET: ${OIDC_CLIENT_SECRET}
          GATEWAY_JWT_SECRET: ${GATEWAY_JWT_SECRET}
          GATEWAY_POSTGRES_URL: postgres://gw:pw@postgres/gateway
          # AWS 認證：在生產中，省略這些並使用執行個體
          # 角色。對於本地 Compose 測試，傳遞您自己的：
          AWS_ACCESS_KEY_ID: ${AWS_ACCESS_KEY_ID}
          AWS_SECRET_ACCESS_KEY: ${AWS_SECRET_ACCESS_KEY}
          AWS_SESSION_TOKEN: ${AWS_SESSION_TOKEN}
        depends_on:
          postgres:
            condition: service_healthy
      postgres:
        image: postgres:16-alpine
        environment: { POSTGRES_USER: gw, POSTGRES_PASSWORD: pw, POSTGRES_DB: gateway }
        healthcheck:
          test: ["CMD-SHELL", "pg_isready -U gw"]
          interval: 5s
        volumes: ["pgdata:/var/lib/postgresql/data"]
    volumes: { pgdata: }
    ```

    閘道是一個單一 Linux 二進位檔，讀取配置，針對您的 IdP 執行 OIDC 發現，應用其 Postgres 架構遷移，建立上游用戶端，並開始監聽。啟動對配置、Postgres 連接（5 秒超時）、OIDC 發現和上游用戶端構造是失敗關閉的。如果其中任何一個無法到達或配置錯誤，閘道會以錯誤退出，而不是以降級狀態提供流量。

    成功啟動不驗證推理路徑，因為 Bedrock 和 Google Cloud 的 Agent Platform 執行個體認證在第一個請求時解析，而不是在啟動時。

    監視 stderr 以了解啟動序列。日誌行使用格式 `[gateway] <timestamp> <level> <message>`，稽核事件是帶有 `evt` 欄位的單行 JSON，啟動橫幅（下面省略）在遷移和監聽行之間列印。您應該按順序看到：

    ```text theme={null}
    {"ts":"2026-06-10T17:03:21.114Z","evt":"config.load","path":"/etc/claude/gateway.yaml","sha256":"…"}
    [gateway] 2026-06-10T17:03:21.408Z info migration 1 applied
    [gateway] 2026-06-10T17:03:21.512Z info claude gateway listening on http://0.0.0.0:8080
    ```

    如果啟動在 `claude gateway listening on` 行之前退出，stderr 的最後一行命名問題：

    * 無法到達的 Postgres
    * 沒有 DDL 權限的 Postgres 角色
    * 無法到達或無效的 OIDC 發現文件
    * 配置架構違規，帶有違規欄位路徑

    修復它並重新啟動。

    如果您已經有 TLS 終止入口，請跳過 Compose 並直接使用 `claude gateway --config gateway.yaml` 執行二進位檔。將 `public_url` 設定為入口來源，並將 `listen` 綁定到環回或叢集內部地址。
  </Step>

  <Step title="驗證身份驗證表面">
    三個檢查確認閘道可以在將其交給開發人員之前驗證真實使用者。

    示例使用閘道的公開 URL；對於沒有入口的本地 Compose 設定，在前兩個檢查中替換 `http://localhost:8080`。第三個檢查開啟 `verification_uri_complete`，它從 `public_url` 建立，因此對於本地 Compose，在 `gateway.yaml` 中設定 `public_url: http://localhost:8080`，並在步驟 1 的 OAuth 用戶端上新增 `http://localhost:8080/oauth/callback` 作為第二個重定向 URI，因為閘道從 `public_url` 建立 IdP `redirect_uri`。驗證連結然後在您的本地瀏覽器中開啟。

    在 Windows PowerShell 中，執行 `curl.exe`；裸 `curl` 是 `Invoke-WebRequest` 的別名，拒絕這些標誌。

    首先，獲取發現文件，確認閘道已啟動、配置有效且所有啟動檢查已通過：

    ```bash theme={null}
    curl -s https://claude-gateway.internal.example.com/.well-known/oauth-authorization-server | jq
    ```

    ```json theme={null}
    {
      "issuer": "https://claude-gateway.internal.example.com",
      "device_authorization_endpoint": "…/oauth/device_authorization",
      "token_endpoint": "…/oauth/token",
      "grant_types_supported": ["urn:ietf:params:oauth:grant-type:device_code", "refresh_token"]
    }
    ```

    回應包括其他欄位，例如 `response_types_supported` 和 `scopes_supported`。

    其次，請求裝置授權，確認裝置登入流程有效且 Postgres 可到達且可寫：

    ```bash theme={null}
    curl -s -X POST https://claude-gateway.internal.example.com/oauth/device_authorization | jq
    ```

    ```json theme={null}
    {
      "device_code": "…",
      "user_code": "WDJB-MJHT",
      "verification_uri": "https://claude-gateway.internal.example.com/device",
      "verification_uri_complete": "https://claude-gateway.internal.example.com/device?user_code=WDJB-MJHT",
      "expires_in": 600,
      "interval": 5
    }
    ```

    第三，透過在瀏覽器中開啟 `verification_uri_complete` 並確認代碼來測試瀏覽器部分。您應該被重定向到您的 IdP 的登入頁面，登入後，返回閘道並顯示已登入確認。

    使用第一個失敗的檢查來定位問題：

    * **第一個檢查失敗**：啟動未完成；檢查 stderr
    * **第二個檢查失敗**：Postgres 無法從閘道到達或角色無法寫入；檢查連接字串和授予
    * **第三個檢查無法到達 IdP**：檢查 IdP 的重定向 URI 是否完全符合 `https://<gateway>/oauth/callback`
    * **第三個檢查到達 IdP 但以錯誤反彈**：讀取閘道的稽核日誌，它記錄每個身份驗證拒絕及其原因，例如 `email domain not allowed`
  </Step>

  <Step title="登入開發人員">
    最後一步發生在開發人員機器上，而不是伺服器上。在該機器的[受管設定檔](/docs/zh-TW/settings#settings-files)中將 `forceLoginMethod` 設定為 `"gateway"` 並將 `forceLoginGatewayUrl` 設定為您的閘道的 `public_url`，然後執行 `/login`，在**雲端閘道**螢幕上按 Enter，並完成瀏覽器登入。下面的[設定閘道 URL](#set-the-gateway-url) 涵蓋大規模分發兩個金鑰。
  </Step>
</Steps>

<h2 id="connect-developers">
  連接開發人員
</h2>

開發人員從自己的筆記型電腦使用一次瀏覽器登入進行連接，使用他們的公司工作帳戶。他們不需要 claude.ai 帳戶、API 金鑰或訂閱，因為對模型的請求透過使用組織上游認證的閘道進行。連接由您透過 MDM 推送的[用戶端側受管設定](/docs/zh-TW/claude-apps-gateway-config#client-side-managed-settings)驅動，因此開發人員端沒有手動設定；本節涵蓋管理員配置的內容。

CLI 在首次連接時對閘道的 TLS 葉憑證進行指紋識別，並按主機名固定它。發佈預期的 SHA-256 指紋以及閘道 URL，以便開發人員有可比較的內容。使用 `openssl x509 -noout -fingerprint -sha256 -in cert.pem` 從憑證檔案取得指紋；`/login` 提示顯示摘要的前 16 個字元作為小寫十六進位，無分隔符。

當憑證輪換時，每個開發人員都會再次看到信任提示，因此將輪換視為計劃事件並重新發佈指紋。

登入後，[模型選擇器](/docs/zh-TW/model-config)顯示開發人員 `availableModels` 允許清單中的模型、受管設定在啟動時應用並每小時刷新一次，遙測路由到您的收集器。會話在 `ttl_hours` 過期前無聲刷新，IdP 取消佈建後的失敗刷新會提示重新登入。

<h3 id="set-the-gateway-url">
  設定閘道 URL
</h3>

在您透過 MDM 或直接在磁碟上部署的每個 OS [受管設定檔](/docs/zh-TW/settings#settings-files)中設定兩個金鑰，`/login` 直接在**雲端閘道**螢幕上開啟，URL 已填入：

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com"
}
```

開發人員按 Enter 進行連接。首次連接 TLS 指紋提示仍然出現。

登入選擇器中沒有閘道選項供開發人員手動選擇，`forceLoginGatewayUrl` 在開發人員自己的設定檔中被忽略。`forceLoginMethod` 單獨，沒有 URL，將開發人員留在「聯絡您的 IT 管理員」訊息處。兩個金鑰都應該在您推送到機器的檔案中，而不是在閘道的 `managed.policies[].cli` 區塊中，該區塊僅到達已連接的用戶端。

<h3 id="ci-pipelines-and-remote-machines">
  CI 管道和遠端機器
</h3>

沒有無人值守管道的服務令牌流程。閘道登入始終執行瀏覽器裝置流程，因此沒有開發人員批准登入的 CI 作業無法進行身份驗證；針對您的提供商直接配置這些。

開發人員登入後，該機器上的每個 Claude Code 呼叫都使用閘道會話，包括非互動式 `claude -p` 執行和由 Agent SDK 啟動的會話，[閘道原則適用於所有這些](/docs/zh-TW/claude-apps-gateway-config#managed)。

裝置流程將輪詢 CLI 與批准瀏覽器分開，因此沒有顯示的遠端開發框仍然有效：開發人員透過 SSH 在遠端機器上執行 `/login`，並在其筆記型電腦上的瀏覽器中開啟驗證連結。

<h3 id="what’s-enforced-on-developers">
  在開發人員上強制執行的內容
</h3>

這些保證適用於每個已登入的閘道會話。

* **模型存取**：對於原則不授予的模型的請求返回 400，`/model` 選擇器被篩選為原則的 `availableModels` 允許清單。在原則中設定 [`enforceAvailableModels: true`](/docs/zh-TW/model-config#default-model-behavior)，以便預設選項解析為 `availableModels` 內的模型，而不是 Claude Code 的內建預設值；沒有它，預設保持可選擇，如果該模型未被授予，則在請求時被拒絕。
* **遙測目的地**：當配置[遙測轉發](/docs/zh-TW/claude-apps-gateway-config#telemetry)時，OTLP 匯出端點被固定到閘道，閘道推送的配置覆蓋本地設定的 `OTEL_*` 變數。
* **認證**：閘道令牌是會話的唯一認證。`ANTHROPIC_AUTH_TOKEN`、`ANTHROPIC_API_KEY`、`apiKeyHelper` 和任何較早的 claude.ai 登入在登入時被忽略，因此開發人員不需要先登出 claude.ai。
* **受管設定**：鎖定的金鑰無法在本地覆蓋。CLI 在啟動時和每個每小時輪詢時應用原則。
* **啟動**：當閘道無法到達時，已登入的會話在啟動時約 10 秒後以錯誤退出，而不是在沒有其設定的情況下啟動。
* **取消佈建**：其使用者在 IdP 中被禁用的會話在下一次刷新失敗時在 `ttl_hours` 內過期。

<h3 id="what-the-organization-can-see">
  組織可以看到什麼
</h3>

使用情況遙測攜帶開發人員的身份、令牌計數、模型和延遲到組織的收集器。閘道不記錄或儲存提示或完成內容。是否收集更豐富的遙測（例如日誌和追蹤），可能包括命令和檔案路徑，是組織的[按目的地選擇](/docs/zh-TW/claude-apps-gateway-config#telemetry)。

<h2 id="availability-and-limitations">
  可用性和限制
</h2>

該表涵蓋當開發人員透過閘道連接時哪些 Claude Code 功能有效，以及閘道伺服器本身支援什麼。如果不支援某些內容，「備註」欄提供替代方案。

閘道傳遞 CLI 發送到每個上游的 [`anthropic-beta`](https://platform.claude.com/docs/en/api/beta-headers) 值，因此操作員不維護測試版允許清單。對於 Amazon Bedrock（忽略標頭），閘道將值移到請求正文的 `anthropic_beta` 欄位；其他上游接收按發送方式發送的標頭。CLI 的閘道會話測試版集合省略了僅限第一方的測試版和擴展快取 TTL 測試版，這就是為什麼下面這些行顯示為不可用。

| 功能                                                                                                     | 狀態  | 備註                                                                                                                                                                                                                                                                             |
| ------------------------------------------------------------------------------------------------------ | --- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 推理轉發 (Amazon Bedrock、Claude Platform on AWS、Google Cloud 的 Agent Platform、Microsoft Foundry、Anthropic) | 可用  | 具有按上游模型轉換和故障轉移。Amazon Bedrock 上游使用 `bedrock-runtime` 端點和 AWS 預設認證鏈；Amazon Bedrock [Mantle 端點](/docs/zh-TW/amazon-bedrock#use-the-mantle-endpoint)不是支援的上游。[Claude Platform on AWS 上游](/docs/zh-TW/claude-apps-gateway-config#claude-platform-on-aws)需要閘道伺服器上的 Claude Code v2.1.198 或更新版本。 |
| 按 IdP 群組的模型存取和受管設定                                                                                     | 可用  | 模型存取在伺服器端強制執行；受管設定按 IdP 群組傳遞，由 CLI 在[受管設定層級](/docs/zh-TW/settings#settings-precedence)應用                                                                                                                                                                                            |
| 遙測扇出 (OTLP/HTTP)                                                                                       | 可用  | 按匯出標識戳記；protobuf 和 JSON 編碼                                                                                                                                                                                                                                                     |
| OIDC 身份提供者                                                                                             | 可用  | 任何符合 OIDC 的 IdP；閘道執行標準 OIDC 探索和授權碼流程。請參閱[身份提供者設定](/docs/zh-TW/claude-apps-gateway-deploy#identity-provider-setup)以了解各 IdP 的配置                                                                                                                                                       |
| 按使用者和按群組支出限制                                                                                           | 可用  | 請參閱[支出限制](/docs/zh-TW/claude-apps-gateway-spend-limits)                                                                                                                                                                                                                             |
| 伺服器端網路搜尋                                                                                               | 不可用 | CLI 無法看到閘道路由到的上游提供商，因此無法驗證網路搜尋支援並在閘道會話上禁用 WebSearch                                                                                                                                                                                                                            |
| 標準提示快取                                                                                                 | 可用  | `cache_control` 斷點被轉發到每個上游                                                                                                                                                                                                                                                     |
| 1 小時快取 TTL                                                                                             | 不可用 | CLI 在閘道會話上省略擴展快取 TTL 測試版，因為並非閘道可以路由到的每個上游都支援 1 小時 TTL，因此透過閘道的提示快取使用 5 分鐘 TTL；請參閱上面的測試版標頭備註                                                                                                                                                                                     |
| 自動模式                                                                                                   | 可用  | 遵循[第三方提供商規則](/docs/zh-TW/permission-modes#enable-auto-mode-on-bedrock-agent-platform-or-foundry)：只有第三方提供商上符合條件的模型可以使用它。在 v2.1.207 之前，閘道會話上的自動模式需要設定 `CLAUDE_CODE_ENABLE_AUTO_MODE=1`，可透過受管原則 `env` 區塊傳遞                                                                             |
| 僅限第一方的最佳化，例如全域快取範圍和令牌高效工具                                                                              | 不可用 | CLI 在閘道會話上不啟用它們；請參閱上面的測試版標頭備註                                                                                                                                                                                                                                                  |
| OTLP/gRPC                                                                                              | 不支援 | 僅 OTLP over HTTP                                                                                                                                                                                                                                                               |
| SAML、LDAP 和其他非 OIDC 身份驗證                                                                               | 不支援 | 僅 OIDC。如果需要，使用 OIDC 橋接                                                                                                                                                                                                                                                         |
| 多租戶（多個 OIDC 發行者）                                                                                       | 不支援 | 每個閘道一個發行者。執行單獨的執行個體                                                                                                                                                                                                                                                            |
| Windows 伺服器                                                                                            | 不支援 | 在 Linux 上部署。僅本地開發的 macOS                                                                                                                                                                                                                                                       |
| Helm 圖表                                                                                                | 不可用 | 閘道作為標準無狀態部署執行；請參閱[部署指南](/docs/zh-TW/claude-apps-gateway-deploy#kubernetes)                                                                                                                                                                                                          |
| 管理員 UI                                                                                                 | 不可用 | 配置是 YAML 檔案；重新部署以更改它                                                                                                                                                                                                                                                           |

<h2 id="next-steps">
  後續步驟
</h2>

快速入門讓您在 Docker Compose 下執行最小配置。要進一步進行：

* 擴展 `gateway.yaml` 超越最小配置，例如新增按群組 RBAC、多上游故障轉移或遙測目的地。[配置參考](/docs/zh-TW/claude-apps-gateway-config)涵蓋每個選項。
* 從 Compose 移動到 Kubernetes 或 Cloud Run 上的生產部署，正確設定您的 IdP，並檢查安全模型。[部署和操作指南](/docs/zh-TW/claude-apps-gateway-deploy)涵蓋每個 IdP 的設定、容器映像要求、健康探針和故障排除。
* 對個別開發人員或群組設定支出上限，以便失控的工作負載無法消耗您的整個承諾。[支出限制](/docs/zh-TW/claude-apps-gateway-spend-limits)涵蓋管理員 API 以及強制執行的工作方式。
* 有關 Google Cloud 上的完整實踐示例，包括 Cloud Run、Cloud SQL 和 Secret Manager，請參閱[在 Google Cloud 上部署](/docs/zh-TW/claude-apps-gateway-on-gcp)。
