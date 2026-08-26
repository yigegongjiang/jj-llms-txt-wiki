> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude 應用程式閘道設定

> 每個 gateway.yaml 選項的參考資料：監聽器和 TLS、OIDC、工作階段、Postgres 存放區、Amazon Bedrock、Claude Platform on AWS、Google Cloud 的 Agent Platform 和 Microsoft Foundry 上游、模型路由、受管原則和遙測。

Claude 應用程式閘道部署由一個 YAML 檔案設定，按慣例稱為 `gateway.yaml`。該檔案定義閘道執行的所有操作：它在哪裡監聽、開發人員如何登入、推論去往何處，以及哪些原則和遙測適用。本頁是該檔案中每個選項的參考資料。若要撰寫您的第一個，請從[快速入門](/docs/zh-TW/claude-apps-gateway#quickstart)開始，它會建立最小的工作設定並執行它；一旦您有了滿意的設定，[部署指南](/docs/zh-TW/claude-apps-gateway-deploy)涵蓋將其容器化並在 Kubernetes、Cloud Run 或您自己的平台上託管。

閘道在啟動時使用 `claude gateway --config /path/to/gateway.yaml` 讀取該檔案一次。每個選項都在啟動時根據架構進行驗證，因此格式不正確的設定會在啟動時失敗，並出現欄位級錯誤，而不是在首次使用時失敗。

本頁末尾的[完整範例](#complete-example)涵蓋每個部分。

<h2 id="file-structure">
  檔案結構
</h2>

五個部分是[必需的](#required-sections)。所有其他部分都是[選用的](#optional-sections)，省略的部分採用其預設值。未知的鍵會導致啟動失敗，因此打字錯誤會顯示為命名錯誤，而不是被無聲地忽略的設定。

**必需部分：**

* [`listen`](#listen)：繫結位址、公開 URL、TLS 終止
* [`oidc`](#oidc)：您的身分識別提供者 (IdP)，包括簽發者、用戶端、宣告對應和誰可以登入
* [`session`](#session)：閘道鑄造的持有人令牌，包括祕密和生命週期
* [`store`](#store)：PostgreSQL，用於裝置授權和速率限制計數器
* [`upstreams`](#upstreams)：推論去往何處，無論是 Anthropic、Amazon Bedrock、Claude Platform on AWS、Google Cloud 的 Agent Platform 或 Microsoft Foundry

**選用部分：**

* [`admin`](#admin)：Admin API 驗證和支出限制的保留
* [`enforcement`](#enforcement)：支出限制失敗開放或失敗關閉行為
* [`models`](#models) 和 `auto_include_builtin_models`：管理員策劃的模型清單和每個上游 ID
* [`managed`](#managed)：按 IdP 群組的受管設定原則
* [`telemetry`](#telemetry)：OTLP 轉發到您的可觀測性堆疊
* [`access_control`、`limits`、`timeouts`、`rate_limits`](#http-tuning)：IP 允許/拒絕、請求大小上限、上游首位元組時間和每 IP 登入限制

<h2 id="secret-expansion">
  祕密擴展
</h2>

不要直接在 `gateway.yaml` 中寫入祕密，例如 `client_secret`、`jwt_secret` 或 `postgres_url`。使用下列其中一種形式參考它們，閘道會在啟動時從環境變數或檔案解析該值：

| 形式              | 解析為                    | 用於                                     |
| --------------- | ---------------------- | -------------------------------------- |
| `${VAR}`        | 環境變數 `VAR`。如果未定義，啟動失敗。 | 容器環境變數、透過環境注入的 AWS Secrets Manager     |
| `${file:/path}` | 檔案內容，已修剪               | Kubernetes Secret 卷掛載、Vault Agent、SOPS |

<h2 id="required-sections">
  必需部分
</h2>

<h3 id="listen">
  `listen`
</h3>

`listen` 區塊控制閘道服務的位置：繫結位址和連接埠、外部可見的來源，以及選用的 TLS 終止。

| 欄位                     | 必需    | 說明                                                                                                                                                                                                                                           |
| ---------------------- | ----- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `host`                 | 否     | 繫結位址。預設 `0.0.0.0`。                                                                                                                                                                                                                           |
| `port`                 | 否     | 繫結連接埠。預設 `8080`。                                                                                                                                                                                                                             |
| `public_url`           | 在代理後面 | 外部可見的 `https://` 來源，用於建立 IdP `redirect_uri` 和發現中繼資料。在任何 TLS 終止代理（例如 ALB、Ingress 或 Cloud Run）後面時為必需，因為閘道在建立自己的來源時不信任 `X-Forwarded-*` 標頭；它們是用戶端可欺騙的。下面的 `trusted_proxies` 僅控制用戶端 IP 解析。啟用[遙測](#telemetry)時也是必需的，因為閘道從此 URL 建立它推送給用戶端的 OTLP 端點。 |
| `tls.cert` / `tls.key` | 否     | 如果閘道自己終止 TLS，則為 PEM 路徑                                                                                                                                                                                                                       |
| `trusted_proxies`      | 否     | 閘道前面的負載平衡器的 CIDR 或 IP。設定時，閘道僅信任來自這些對等方的 `X-Forwarded-For`，並記錄真實用戶端 IP 以進行每 IP 速率限制和稽核。等同於 nginx `set_real_ip_from`。                                                                                                                          |

<h3 id="oidc">
  `oidc`
</h3>

`oidc` 區塊將閘道連接到您的身分識別提供者，並決定誰可以登入。它命名簽發者和 OAuth 用戶端、對應攜帶電子郵件和群組的宣告，並按電子郵件網域或群組限制登入。

OpenID Connect (OIDC) 是閘道與您的身分識別提供者一起使用的 SSO 協定；請參閱[身分識別提供者設定](/docs/zh-TW/claude-apps-gateway-deploy#identity-provider-setup)以了解在 IdP 端註冊的內容。

| 欄位                              | 必需 | 說明                                                                                                                                                                                                                                                                                                                                                                      |
| ------------------------------- | -- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `issuer`                        | 是  | OIDC 發現基礎。必須在 `/.well-known/openid-configuration` 提供發現。在生產環境中使用 HTTPS；閘道接受 `http://` 簽發者。環回簽發者（例如 `http://localhost:8081`）會被[SSRF 防護](/docs/zh-TW/claude-apps-gateway-deploy#threat-model-summary)拒絕，除非在閘道的環境中設定了 `CLAUDE_GATEWAY_ALLOW_LOOPBACK=1`。                                                                                                                         |
| `client_id` / `client_secret`   | 是  | 來自您的 OAuth 用戶端註冊                                                                                                                                                                                                                                                                                                                                                        |
| `allowed_email_domains`         | 否  | 拒絕 `email` 宣告不在這些網域之一中的 id\_token，不區分大小寫。針對多租戶 IdP 誤設定的深度防禦。獨立於此設定，`email_verified` 宣告明確為 `false` 的 id\_token 始終被拒絕。                                                                                                                                                                                                                                                    |
| `allowed_groups`                | 否  | 限制登入為這些 IdP 群組的成員，與 `groups_claim` 相符。允許電子郵件網域中但不在這些群組中的使用者被拒絕。需要 IdP 發出群組宣告。                                                                                                                                                                                                                                                                                           |
| `groups_claim`                  | 否  | 哪個 id\_token 宣告攜帶群組成員資格。預設 `groups`。Microsoft Entra 在 `roles` 下發出應用程式角色。接受平面鍵或 RFC 6901 JSON 指標，例如 `/resource_access/gateway/roles` 用於嵌套宣告。                                                                                                                                                                                                                             |
| `google_groups`                 | 否  | 透過 Google Workspace Admin SDK Directory API 查詢已登入使用者的群組，因為 Google 的 id\_token 不攜帶群組宣告。將 `service_account_json_path` 設定為具有 `https://www.googleapis.com/auth/admin.directory.group.readonly` 範圍的網域範圍委派的服務帳戶金鑰檔案，並將 `admin_email` 設定為服務帳戶模擬的 Workspace 管理員；Directory API 需要真實的管理員主體。每個使用者的群組電子郵件地址成為其群組宣告，因此 `allowed_groups` 和 `managed.policies.match.groups` 與群組電子郵件相符。 |
| `email_claim`                   | 否  | 哪個 id\_token 宣告攜帶使用者的電子郵件。預設 `email`。某些 IdP（例如 ADFS 和 Entra B2C）改為發出 `upn` 或 `preferred_username`。接受平面鍵、JSON 指標或後備鍵清單，其中使用第一個存在的鍵。                                                                                                                                                                                                                                      |
| `scopes`                        | 否  | 閘道請求的 OIDC 範圍的完整覆蓋。預設 `[openid, profile, email, offline_access]`。當您的 IdP 拒絕它不識別的範圍或需要自訂範圍來發出群組或電子郵件時設定。必須包含 `openid`。刪除 `offline_access` 會停用重新整理令牌，因此開發人員每 `session.ttl_hours` 重新執行瀏覽器登入。請參閱[身分識別提供者設定](/docs/zh-TW/claude-apps-gateway-deploy#identity-provider-setup)以了解每個 IdP 範圍配方，例如 Google 的重新整理令牌流程。                                                                   |
| `extra_auth_params`             | 否  | 附加到 IdP 授權請求的額外查詢參數，逐字。這是 IdP 特定行為的覆蓋機制，例如 Google 重新整理令牌的 `access_type: offline`、某些 Entra 租戶的 `domain_hint` 或逐步提升流程的 `acr_values`。無法覆蓋閘道管理的協定參數：`state`、`nonce`、`redirect_uri`、PKCE、`scope`、`response_type`、`response_mode` 和 `client_id`。                                                                                                                              |
| `userinfo_fallback`             | 否  | 當 id\_token 省略電子郵件或群組時，從 `/userinfo` 擷取它們。Keycloak 輕量級存取令牌、Okta 組織伺服器和 ADFS 最小令牌需要。id\_token 保持權威；userinfo 僅填補空白。預設 `false`。                                                                                                                                                                                                                                            |
| `use_pkce`                      | 否  | 在授權請求上傳送 PKCE (S256) 挑戰。預設 `true`。僅當您的 IdP 為此機密用戶端拒絕 PKCE 時設定 `false`。                                                                                                                                                                                                                                                                                                  |
| `clock_skew_seconds`            | 否  | 驗證 id\_token 時間宣告時容許時鐘漂移。預設 `0`，這是嚴格的。如果您在登入後立即看到「令牌已過期/尚未有效」錯誤，請提高以應對主機/IdP 時鐘偏差。                                                                                                                                                                                                                                                                                      |
| `token_endpoint_auth_method`    | 否  | 覆蓋令牌端點驗證方法。接受 `client_secret_basic` 或 `client_secret_post`。預設自動協商。                                                                                                                                                                                                                                                                                                      |
| `id_token_signed_response_alg`  | 否  | 預期的 id\_token 簽署演算法。預設 `RS256`。為使用 ES256、PS256 或 EdDSA 簽署的 IdP 設定。                                                                                                                                                                                                                                                                                                      |
| `additional_authorized_parties` | 否  | 除了 `client_id` 之外要接受的額外 `azp` 值，用於 Keycloak 代理和令牌交換流程                                                                                                                                                                                                                                                                                                                   |
| `discovery_url`                 | 否  | 從此 URL 擷取發現文件，而不是從 `issuer` 衍生，用於代理後面重寫簽發者主機的 IdP。路徑必須包含 `/.well-known/`。                                                                                                                                                                                                                                                                                               |
| `form_action_origins`           | 否  | `/device` 頁面的 `Content-Security-Policy: form-action` 指令的其他來源。閘道已允許 `'self'` 和發現的 `authorization_endpoint` 來源，但 Chrome 對整個重新導向鏈強制執行 `form-action`。如果您的 IdP 透過第二個主機重新導向，例如 Azure AD 聯合到 ADFS、中樞輪輻 Okta 或公司 SSO 攔截器，列出授權請求可能重新導向的每個來源。                                                                                                                                     |
| `ca_cert_pem`                   | 否  | PEM CA 憑證，用於替換 IdP 請求的系統信任存放區。用於公司 PKI 後面的 Keycloak 或 Dex。                                                                                                                                                                                                                                                                                                              |

<h3 id="session">
  `session`
</h3>

`session` 區塊塑造閘道在登入後鑄造的持有人令牌：簽署它們的祕密和它們的生命週期。

| 欄位           | 必需 | 說明                                                                                                                                                                                  |
| ------------ | -- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `jwt_secret` | 是  | 至少 32 位元組的熵，例如來自 `openssl rand -base64 32`。簽署閘道的 HS256 持有人令牌。接受單一字串或用於輪換的陣列：索引 0 簽署，所有項目驗證。若要輪換，前置新祕密，等待 `ttl_hours`，然後刪除舊祕密。                                                       |
| `ttl_hours`  | 否  | 閘道持有人令牌生命週期。預設 `1`。當 IdP 發出重新整理令牌時，CLI 在過期前無聲地重新整理。較短的生命週期會更快地取消佈建；較長的生命週期會減少 IdP 往返次數。如果您的 IdP 因為 `offline_access` 不可用而無法發出重新整理令牌，則沒有無聲重新整理，因此將其提高到 `8` 或 `12` 以避免每小時將開發人員送回瀏覽器登入。 |

<h3 id="store">
  `store`
</h3>

`store` 區塊將閘道指向其 PostgreSQL 資料庫，該資料庫保存裝置授權和速率限制計數器。

| 欄位                | 必需 | 說明                                                                                                                                                                                                                                                                                                                                 |
| ----------------- | -- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `postgres_url`    | 是  | `postgres://` 或 `postgresql://` URL。必需：裝置授權會合點，瀏覽器回呼寫入且輪詢 CLI 讀取，需要跨副本狀態。閘道在啟動時執行自己的架構遷移，因此角色需要在目標架構上 `CREATE TABLE`。如果您的安全原則禁止應用程式角色的 DDL，請使用管理員角色執行遷移，最初和每當新版本發出遷移時，並授予應用程式角色對閘道表的 `SELECT, INSERT, UPDATE, DELETE`。請參閱[升級](/docs/zh-TW/claude-apps-gateway-deploy#upgrades)和 [Postgres](/docs/zh-TW/claude-apps-gateway-deploy#postgres)。 |
| `username`        | 否  | 覆蓋 `postgres_url` 中的使用者                                                                                                                                                                                                                                                                                                            |
| `password`        | 否  | 資料庫認證。在此設定它而不是在 `postgres_url` 中，以便認證保持在 URL 之外。接受任何字元並優先於 URL 認證。                                                                                                                                                                                                                                                                 |
| `max_connections` | 否  | 每個副本的 Postgres 連線池大小。預設 `5`，這是保守的且對共享資料庫友善。啟用[支出限制](#admin)後，熱路徑每個推論請求執行幾個操作，因此在負載下為專用資料庫提高它，並保持副本 × 此值低於資料庫的 `max_connections`。                                                                                                                                                                                                   |

對於本地開發，將 `postgres_url` 指向一次性 Postgres 容器，例如 `docker run --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust postgres`。

<h3 id="upstreams">
  `upstreams`
</h3>

`upstreams` 是一個有序清單。閘道將推論轉發到解析所請求模型的第一個上游。在 `5xx`、`429`、`401`、`403`、`404` 或逾時時，它會故障轉移到下一個；其他 `4xx` 不會，因為這些錯誤可歸因於請求而不是上游。`401` 或 `403` 表示閘道自己的認證對該上游失敗，`404` 表示該上游不服務所請求的模型，因此清單中稍後的上游仍然可以。

故障轉移於 `404` 需要閘道 v2.1.198 或更新版本。較早的版本即使清單中稍後的上游服務該模型，也會將第一個 `404` 返回給用戶端。

相同提供者的多個上游必須設定不同的 `name:`。

Amazon Bedrock、Claude Platform on AWS、Google Cloud 的 Agent Platform 和 Microsoft Foundry 用戶端在啟動時建立一次，其 SDK 在內部重新整理認證，因此輪換雲認證不需要重新啟動。靜態 Anthropic API 金鑰和持有人在啟動時讀取；請參閱 [Anthropic API](#anthropic-api)。

<h4 id="anthropic-api">
  Anthropic API
</h4>

最小的 Anthropic 上游是來自 [Claude Console](https://platform.claude.com) 的 API 金鑰：

```yaml theme={null}
upstreams:
  - provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}
    # 或 OAuth 持有人（例如工作負載身分識別聯合交換的令牌）：
    #   oauth_token: ${file:/var/run/secrets/anthropic-oauth-token}
    # base_url: https://api.anthropic.com   # 預設；覆蓋以使用轉發代理
```

兩種認證形式在它們傳送的標頭中有所不同：

* **`api_key`**：傳送 `x-api-key`。在 Claude Console 中輪換它並更新環境變數。
* **`oauth_token`**：傳送 `Authorization: Bearer`。當您的組織發出短期令牌而不是長期 API 金鑰時使用持有人形式。持有人在啟動時讀取一次，因此透過重新掛載祕密和重新啟動來重新整理。

除了靜態金鑰或持有人，您可以使用工作負載身分識別聯合。按照[工作負載身分識別聯合指南](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation)建立聯合規則，然後將您的工作負載的 OIDC JWT 掛載為檔案，例如 Kubernetes 投影服務帳戶令牌或 CI 平台的 id-token。閘道將 JWT 交換為短期持有人並自動重新整理它。令牌檔案在每次交換時重新讀取，因此輪換的投影令牌無需重新啟動即可被拾取。

```yaml theme={null}
upstreams:
  - provider: anthropic
    auth:
      federation_rule_id: ${ANTHROPIC_FEDERATION_RULE_ID}
      organization_id: ${ANTHROPIC_ORGANIZATION_ID}
      identity_token_file: /var/run/secrets/anthropic/id-token
      # workspace_id: wrkspc_...       # 如果規則涵蓋 >1 個工作區，則為必需
      # service_account_id: svac_...   # 選用的預期目標檢查
```

<h4 id="amazon-bedrock">
  Amazon Bedrock
</h4>

對於閘道替換或前置的用戶端 Bedrock 部署，請參閱 [Claude Code on Amazon Bedrock](/docs/zh-TW/amazon-bedrock)。閘道端上游：

```yaml theme={null}
upstreams:
  - provider: bedrock
    region: us-east-1
    auth: {}                           # 首選：AWS 預設認證鏈
    # 或明確認證：
    # auth:
    #   aws_access_key_id: ${AWS_AKID}
    #   aws_secret_access_key: ${AWS_SK}
    #   aws_session_token: ${AWS_ST}
    # 或 Bedrock API 持有人令牌：
    # auth:
    #   aws_bearer_token: ${AWS_BEARER_TOKEN}
    # 覆蓋 bedrock-runtime 端點以進行 FIPS 或 VPC 端點部署：
    # base_url: https://bedrock-runtime-fips.us-east-1.amazonaws.com
```

空的 `auth` 區塊使用 AWS SDK 的預設認證鏈：環境變數、`~/.aws/credentials`、ECS 任務角色、EC2 執行個體中繼資料或 EKS 上的 IRSA。在生產環境中，給予閘道 Pod 一個 IAM 角色，而不是在容器映像中嵌入靜態金鑰。

明確認證必須完整：當 `aws_access_key_id` 和 `aws_secret_access_key` 未一起設定時，或當 `aws_session_token` 在沒有它們的情況下設定時，閘道在啟動時失敗。在 v2.1.207 之前，部分 `auth:` 區塊通過驗證。

| 設定         | 方式                                                                                                                                                                                                                                      |
| ---------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| IAM 權限     | 授予閘道的主體 `bedrock:InvokeModel` 和 `bedrock:InvokeModelWithResponseStream` 在推論設定檔 ARN 和基礎基礎模型 ARN 上。對於美國地區的內建目錄：`arn:aws:bedrock:<region>:<account>:inference-profile/us.anthropic.*` 和 `arn:aws:bedrock:*::foundation-model/anthropic.*`。 |
| 模型存取       | 在 Bedrock 主控台中，按地區，請求並啟用您想要的 Claude 模型的模型存取。跨地區推論設定檔 (`us.anthropic.*`) 需要在設定檔跨越的每個地區中進行模型存取。                                                                                                                                           |
| EKS (IRSA) | 建立具有上述原則和針對您叢集的 OIDC 提供者的信任原則的 IAM 角色，範圍限於閘道的服務帳戶。使用 `eks.amazonaws.com/role-arn: arn:aws:iam::<acct>:role/claude-gateway` 註釋服務帳戶。`auth: {}` 會拾取它。                                                                                      |
| ECS / EC2  | 將 IAM 角色附加到任務定義或執行個體設定檔。`auth: {}` 會拾取它。                                                                                                                                                                                                |
| 其他任何地方     | 透過 `AWS_ACCESS_KEY_ID`、`AWS_SECRET_ACCESS_KEY` 和 `AWS_SESSION_TOKEN` 環境變數傳遞認證，或在 `auth:` 中使用 `${VAR}` 擴展明確設定它們                                                                                                                          |
| 地區         | `region:` 是 API 端點地區。跨地區推論設定檔無論您選擇哪一個，都會跨地理位置 (US、EU、APAC) 路由。對於非美國地區或佈建輸送量 ARN，新增具有正確每上游 ID 的 [`models:`](#models) 區塊。                                                                                                                 |

<h4 id="claude-platform-on-aws">
  Claude Platform on AWS
</h4>

Claude Platform on AWS 在 `aws-external-anthropic.<region>.api.aws` 的 AWS 基礎設施上服務第一方 Anthropic API。它使用第一方模型 ID，按原樣接受 `anthropic-beta` 標頭，並服務 `count_tokens`，因此 Bedrock 特定的轉譯都不適用。`anthropicAws` 提供者需要 Claude Code v2.1.198 或更新版本；較早的閘道版本在啟動時拒絕它。

對於相同平台的用戶端部署，請參閱 [Claude Code on Claude Platform on AWS](/docs/zh-TW/claude-platform-on-aws)。閘道端上游：

```yaml theme={null}
upstreams:
  - provider: anthropicAws
    region: us-east-1
    workspace_id: wrkspc_...
    auth:
      api_key: ${ANTHROPIC_AWS_API_KEY}   # 作為 x-api-key 傳送
    # 或透過 AWS 預設認證鏈的 SigV4：
    # auth: {}
    # 或明確的 SigV4 認證：
    # auth:
    #   aws_access_key_id: ${AWS_ACCESS_KEY_ID}
    #   aws_secret_access_key: ${AWS_SECRET_ACCESS_KEY}
    # 覆蓋衍生的端點：
    # base_url: https://aws-external-anthropic.us-east-1.api.aws
```

該平台在與 Amazon Bedrock 不同的 AWS 帳戶中執行，並為其自己的服務名稱 `aws-external-anthropic` 簽署 SigV4 請求，因此 Bedrock 範圍的 IAM 角色不授權它。`auth.api_key` 中的 API 金鑰在同時設定 SigV4 認證時優先。空的 `auth` 區塊使用 AWS SDK 的預設認證鏈，與 [Amazon Bedrock](#amazon-bedrock) 上游使用的相同鏈。

| 欄位                                                      | 必需 | 說明                                                                             |
| ------------------------------------------------------- | -- | ------------------------------------------------------------------------------ |
| `region`                                                | 是  | AWS 地區，小寫字母、數字和連字號。閘道將端點衍生為 `https://aws-external-anthropic.<region>.api.aws`。 |
| `workspace_id`                                          | 是  | 在每個請求上傳送為標頭；平台需要它                                                              |
| `auth.api_key`                                          | 否  | 平台的 API 金鑰，作為 `x-api-key` 傳送。不是持有人令牌：兩種驗證模式是 API 金鑰或 SigV4。                    |
| `auth.aws_access_key_id` / `auth.aws_secret_access_key` | 否  | 明確的 SigV4 認證。設定其中一個而不設定另一個在啟動時失敗。`auth.aws_session_token` 與它們一起被接受。            |
| `base_url`                                              | 否  | 覆蓋衍生的端點                                                                        |

因為平台解析第一方模型 ID，內建目錄無需 [`models:`](#models) 區塊即可路由到它。當您策劃 `models:` 清單時，使用第一方 ID 鍵入 `anthropicAws:` 項目。

<h4 id="google-cloud-agent-platform">
  Google Cloud Agent Platform
</h4>

對於等效的用戶端設定，請參閱 [Claude Code on Google Cloud](/docs/zh-TW/google-vertex-ai)。閘道端上游：

```yaml theme={null}
upstreams:
  - provider: vertex
    region: us-east5
    project_id: example-prod
    auth: {}                           # 首選：應用程式預設認證
    # 或服務帳戶金鑰檔案：
    # auth: { service_account_json: /secrets/sa.json }
    # 覆蓋 aiplatform 端點以進行私人服務連線：
    # base_url: https://us-east5-aiplatform.p.googleapis.com
```

空的 `auth` 區塊使用應用程式預設認證：`GOOGLE_APPLICATION_CREDENTIALS`、GCE 中繼資料或 GKE 工作負載身分識別。支援服務帳戶 JSON 金鑰檔案但不建議；使用工作負載身分識別或將服務帳戶附加到 GCE 或 Cloud Run 執行個體。

設定 `region: global` 以使用 [Agent Platform 的全域端點](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations)而不是區域端點。Google 然後將每個請求路由到可用的地區，因此您不追蹤每個地區的模型可用性。設定特定地區會將每個請求固定到它。

| 設定              | 方式                                                                                                                                           |
| --------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| IAM 權限          | 授予閘道的服務帳戶在專案上的 `roles/aiplatform.user`，或具有 `aiplatform.endpoints.predict` 的自訂角色。啟用 Agent Platform API (`aiplatform.googleapis.com`)。         |
| 模型存取            | 在 Model Garden 中，為您的專案啟用 Claude 模型。它們發佈到特定地區；檢查模型卡以了解支援的地區。                                                                                  |
| GKE (工作負載身分識別)  | 將 GCP 服務帳戶繫結到閘道的 Kubernetes 服務帳戶，並使用 `iam.gke.io/gcp-service-account: claude-gateway@<proj>.iam.gserviceaccount.com` 註釋 KSA。`auth: {}` 會拾取它。 |
| Cloud Run / GCE | 將服務的服務帳戶設定為具有 `roles/aiplatform.user` 的帳戶。`auth: {}` 會拾取它。                                                                                   |
| 其他任何地方          | `auth: { service_account_json: /secrets/sa.json }`，JSON 金鑰檔案的路徑，掛載為祕密。該欄位採用檔案路徑，而不是金鑰內容，因此不涉及 `${file:…}` 擴展。                                |

<h4 id="microsoft-foundry">
  Microsoft Foundry
</h4>

對於用戶端 Foundry 部署，請參閱 [Claude Code on Microsoft Foundry](/docs/zh-TW/microsoft-foundry)。閘道端上游：

```yaml theme={null}
upstreams:
  - provider: foundry
    resource: example-foundry              # https://example-foundry.services.ai.azure.com
    auth: { use_azure_ad: true }        # 首選：DefaultAzureCredential / 受管身分識別
    # 或 API 金鑰：
    # auth:
    #   api_key: ${FOUNDRY_API_KEY}
```

`use_azure_ad: true` 透過 `DefaultAzureCredential` 解析：AKS、ACI 或 App Service 上的受管身分識別；Azure CLI；或環境認證。API 金鑰有效但是專案範圍的，不會自動輪換。Foundry 的端點衍生自 `resource:`；設定選用的 `base_url` 以覆蓋它以進行主權雲，例如 Azure Government。

| 設定                | 方式                                                                                                   |
| ----------------- | ---------------------------------------------------------------------------------------------------- |
| RBAC              | 授予閘道的身分識別在 Foundry 資源上的 `Azure AI User` 或 `Cognitive Services User`                                  |
| 部署                | Foundry 使用管理員選擇的部署名稱，而不是規範模型 ID。新增 [`models:`](#models) 區塊，將每個規範 ID 對應到您的部署名稱。                       |
| AKS (工作負載身分識別)    | 將使用者指派的受管身分識別與叢集的 OIDC 簽發者聯合，並將其繫結到閘道的服務帳戶。`use_azure_ad: true` 透過 `WorkloadIdentityCredential` 拾取它。 |
| ACI / App Service | 在資源上啟用系統指派或使用者指派的受管身分識別。`use_azure_ad: true` 會拾取它。                                                   |
| 其他任何地方            | `auth: { api_key: "${FOUNDRY_API_KEY}" }`。在 `{ }` 內引用 `${…}`。                                        |

<h4 id="multiple-upstreams">
  多個上游
</h4>

相同的提供者可以出現多次，具有不同的 `name:`。這涵蓋不同的地區、透過不同認證鏈的不同帳戶、佈建輸送量與隨需，以及跨提供者故障轉移。

閘道按順序嘗試上游。`5xx`、`429`、`401`、`403`、`404`、逾時和遺漏端點 (`501`) 故障轉移；其他 `4xx` 不會。

`429` 是每個上游容量，因此佈建輸送量 (PT) 耗盡會故障轉移到隨需。`404` 是每個上游模型可用性，因此未啟用模型的上游不會阻止清單中稍後服務它的上游。無法解析所請求模型的上游會被跳過，無需網路往返。

此範例首先路由佈建輸送量 Bedrock 配額，溢出到隨需和第二個帳戶，最後故障轉移到 Anthropic API：

```yaml theme={null}
upstreams:
  # 主要：您主區域中的佈建輸送量。
  - name: bedrock-pt
    provider: bedrock
    region: us-east-1
    auth: {}
  # 溢出：隨需跨地區。
  - name: bedrock-od
    provider: bedrock
    region: us-west-2
    auth: {}
  # 不同帳戶：透過假設角色認證的單獨 Bedrock 配額。
  - name: bedrock-acct2
    provider: bedrock
    region: us-east-1
    auth:
      aws_access_key_id: ${ACCT2_AKID}
      aws_secret_access_key: ${ACCT2_SK}
  # 最後手段：直接 Anthropic API。
  - name: anthropic-fallback
    provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}

# 每個上游模型 ID 以上游的 `name:` 為鍵；沒有 `name:` 的上游預設為其提供者字串（例如 `bedrock`）。任何未為模型列出的上游都會被跳過，這是您將模型路由到佈建輸送量的方式，而所有其他模型保持隨需。
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      bedrock-pt: arn:aws:bedrock:us-east-1:111111111111:provisioned-model/abcdef
      bedrock-od: us.anthropic.claude-opus-4-8
      bedrock-acct2: us.anthropic.claude-opus-4-8
      anthropic-fallback: claude-opus-4-8
```

| 槓桿            | 方式                                                                                                                            |
| ------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| 不同地區          | 每個地區一個 Bedrock 上游，每個都有自己的 `region:`。使用 [`auto_include_builtin_models: true`](#models)，跨地區推論設定檔會自動路由；對於地區固定部署，使用 `models:` 區塊。 |
| 不同帳戶          | 每個帳戶一個 Bedrock 上游，每個在 `auth:` 中都有自己的認證。預設鏈 (`auth: {}`) 使用 Pod 的身分識別；對於第二個帳戶，設定明確認證或持有人令牌。                                    |
| 佈建輸送量         | 將模型對應到該上游名稱的 `models:` 中的佈建輸送量 ARN。其他上游保持隨需 ID，因此 PT 容量在故障轉移前耗盡。                                                              |
| VPC / FIPS 端點 | 在上游上設定 `base_url:` 為您的 VPC 端點或 FIPS 端點 URL                                                                                    |
| 模型範圍路由        | 從模型的 `upstream_model:` 對應中省略上游，該上游會被跳過以進行該模型。例如，將 Opus 路由到佈建輸送量，將 Sonnet 和 Haiku 路由到隨需。                                       |

在雲提供者之間或直接 Anthropic API 之間故障轉移會改變哪些協議、地理位置和其他條款管理請求。

CLI 對閘道應用相同的功能閘控，無論哪個上游服務給定請求，因此故障轉移不會傳送上游會拒絕的正文欄位。

<h2 id="optional-sections">
  選用部分
</h2>

<h3 id="admin">
  `admin`
</h3>

選用。啟用 `/v1/organizations/spend_limits`，它鏡像 Anthropic 的公開 Admin API，以及 `/v1/messages` 上的每個開發人員支出強制執行。請參閱[支出限制](/docs/zh-TW/claude-apps-gateway-spend-limits)以了解如何設定和強制執行上限；本部分涵蓋打開功能和調整它的 `gateway.yaml` 鍵。

```yaml theme={null}
admin:
  # 管理員端點的命名靜態 API 金鑰，作為 x-api-key 傳送。
  # id 在稽核日誌中顯示為 admin-key:<id>，因此每個金鑰都是
  # 可歸因的。用於輪換的陣列：新增新金鑰、滾動用戶端、
  # 移除舊金鑰。
  write_keys:
    - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
    - { id: ci,        key: "${GATEWAY_ADMIN_WRITE_KEY_CI}" }
  read_keys:
    - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
  # 透過正常閘道 JWT（無 API 金鑰）授予完整管理員的 IdP 群組。
  admin_groups: [platform-finops]
  blocked_message: request an increase at https://go.example.com/claude-limits
```

| 欄位                        | 必需 | 說明                                                                                                                                                                                        |
| ------------------------- | -- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `write_keys`              | 否  | `{id, key}` 的陣列。與其中一個相符的 `x-api-key` 可以列出、設定和刪除支出限制。金鑰值必須至少 32 個字元；`id` 必須在 `read_keys` 和 `write_keys` 中唯一。                                                                               |
| `read_keys`               | 否  | `{id, key}` 的陣列。唯讀：每個 `GET` 端點，包括列出上限、按 ID 擷取一個，以及讀取 [`/effective`](/docs/zh-TW/claude-apps-gateway-spend-limits#%2Feffective) 和 [`/audit`](/docs/zh-TW/claude-apps-gateway-spend-limits#%2Faudit)。 |
| `admin_groups`            | 否  | IdP 群組名稱。其 `groups` 宣告包含其中一個的閘道 JWT 具有完整管理員存取權，讀取和寫入，並稽核為 `oidc:<sub>`。將此用於人類管理員；將 API 金鑰用於機器。                                                                                            |
| `blocked_message`         | 否  | 逐字附加到被阻止的開發人員看到的 `429 billing_error`。寫入整個指令，例如 URL 或 Slack 頻道。未設定，錯誤是 `spend limit reached`。                                                                                              |
| `audit_retention_days`    | 否  | 預設 `365`。較舊的 `admin_audit` 列會被掃除。                                                                                                                                                         |
| `spend_retention_months`  | 否  | 預設 `13`。早於此的 `spend` 計數器列會被掃除。預設值保留完整年份加當前部分月份以進行年度比較報告。                                                                                                                                  |
| `identity_retention_days` | 否  | 預設 `90`。`principal_emails` 列的最後一次看到 TTL，其中保存每個開發人員的電子郵件、顯示名稱和群組 (PII)。故意比支出保留更短，因此已取消佈建的身分識別會在其匿名支出計數器保留時老化。                                                                              |
| `group_limit_mode`        | 否  | `min`（預設）或 `max`。當開發人員在具有上限的多個群組中時，`min` 強制執行最限制的，`max` 強制執行最寬鬆的。由強制執行和 `/effective` 使用。                                                                                                  |

<h3 id="enforcement">
  `enforcement`
</h3>

`enforcement` 區塊控制當存放區不可用時支出限制檢查的行為。

| 欄位                     | 必需 | 說明                                                                                                                        |
| ---------------------- | -- | ------------------------------------------------------------------------------------------------------------------------- |
| `fail_closed_on_error` | 否  | 預設 `false`。支出強制執行在 Postgres 中斷時失敗開放，因此推論保持運行。設定 `true` 以失敗關閉：超過上限的開發人員被阻止，但如果存放區無法到達，每個人都被阻止。沒有 [`admin:`](#admin) 區塊時無效。 |

<h3 id="models">
  `models`
</h3>

`models` 區塊是選用的管理員策劃模型清單，在 `/v1/models` 提供並用於轉換每個上游的模型 ID。對於非美國 Amazon Bedrock 地區、Amazon Bedrock 佈建輸送量 ARN 和 Microsoft Foundry 部署名稱是必需的。

```yaml theme={null}
auto_include_builtin_models: true   # false：僅公開下面的清單
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    # description: 在表面它的用戶端中顯示的選用文字
    upstream_model:
      anthropic: claude-opus-4-8
      bedrock: us.anthropic.claude-opus-4-8   # 或推論設定檔 ARN
      foundry: your-opus-deployment-name
```

<h3 id="managed">
  `managed`
</h3>

`managed` 區塊定義基於 IdP 群組或電子郵件網域的角色型存取原則。原則按順序評估；選擇第一個相符項，然後合併到下面描述的 `match: {}` 全部捕捉基礎上。它們按使用者在 `GET /managed/settings` 提供，具有 ETag/304 快取。

```yaml theme={null}
managed:
  policies:
    # 特定群組優先。
    - match: { groups: [eng-contractors] }
      cli:
        availableModels: [claude-sonnet-4-6]
        permissions: { deny: ["WebFetch", "WebSearch"] }
    # 預設全部捕捉最後：符合每個已驗證的人。
    - match: {}
      cli:
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
```

`match: {}` 全部捕捉，按慣例列在最後，被視為基礎層。每個其他原則從全部捕捉繼承它不設定的任何鍵，因此每個角色項目只需要列出與組織預設不同的內容。合併規則取決於鍵類型：

* **允許清單**：`availableModels` 和 `permissions.allow`。特定原則的清單完全替換基礎的。
* **拒絕清單和掛鉤陣列**：`permissions.deny`、`permissions.ask`、`disabledMcpjsonServers`、`deniedMcpServers`、`blockedMarketplaces` 和每個 `hooks` 事件類型陣列。這些採用基礎和原則的聯合，因此組織範圍的拒絕或稽核掛鉤無法被每個角色覆蓋意外刪除。
* **記錄類型鍵**：`env`、`modelOverrides` 和 `skillOverrides`。這些淺合併，因此每個角色 `env` 區塊覆蓋它設定的鍵並從基礎繼承其餘的。

`availableModels` 也在 `/v1/messages` 伺服器端強制執行，因此被拒絕的模型返回 `400`，無論用戶端傳送什麼。

| 匹配器                                                 | 行為                                                         |
| --------------------------------------------------- | ---------------------------------------------------------- |
| `match: {}`                                         | 符合每個已驗證的使用者。從其中一個開始，稍後新增群組範圍的原則。                           |
| `match: { groups: [a, b] }`                         | 如果 JWT 的 `groups` 宣告包含任何列出的群組，則符合。區分大小寫：群組必須符合 IdP 的確切大小寫。 |
| `match: { email_domain: example.com }`              | 符合 JWT 的 `email` 宣告中最後一個 `@` 之後的部分，不區分大小寫。每個原則接受一個網域。      |
| `match: { groups: [a], email_domain: example.com }` | 兩個條件都必須符合                                                  |

未符合任何原則的已驗證使用者獲得閘道的預設值，這意味著目錄中的每個模型和沒有受管設定。如果您想要保證的預設原則，請在最後新增 `match: {}` 全部捕捉。

<Note>
  閘道保持沒有自己的使用者目錄。它從使用者的 IdP 令牌授權每個請求，從令牌的 `groups` 宣告讀取群組成員資格，並根據它評估原則。沒有要列舉的名冊，沒有要預先建立的帳戶，因此沒有 SCIM 端點，因為沒有什麼可供 SCIM 同步到。

  在真實來源（您的 IdP 的原生 SCIM 佈建或專用身分識別治理平台）執行使用者和群組生命週期管理。那裡管理的成員資格和取消佈建透過令牌自動流入閘道。如果您想要 Claude 帳戶本身的 SCIM 佈建，那是 [Claude for Enterprise](/docs/zh-TW/admin-setup) 功能。

  兩個傳播時鐘適用：

  * **原則內容**：編輯原則並重新部署在連接的用戶端的下一個受管設定輪詢時到達，在一小時內
  * **群組成員資格**：變更使用者的群組成員資格會變更哪個原則符合他們。這在下一個工作階段重新鑄造時生效，意味著下一個無聲重新整理，受 `session.ttl_hours` 限制。
</Note>

<h4 id="what-goes-in-cli">
  `cli` 中的內容
</h4>

每個 `cli` 值是完整的 Claude Code `managed-settings.json` 文件，與您透過 MDM 或 `/etc/claude-code/managed-settings.json` 部署的相同架構，在此表示為 YAML。CLI 在受管層應用傳遞的文件，在使用者和專案設定之上。

閘道在啟動時根據 CLI 的設定架構驗證每個文件，因此無法識別的頂層鍵或具有格式不正確值的識別鍵會在啟動時失敗，並出現命名每個違規鍵的錯誤。架構的故意開放部分仍然接受任意值，因為較新的用戶端可能識別閘道的架構不識別的項目。這些開放鍵是 `env`、`pluginConfigs` 和 `permissions` 下嵌套的鍵。

因為驗證使用與閘道的已安裝版本捆綁的架構，將較新 Claude Code 版本引入的頂層設定鍵放入受管設定需要首先升級閘道。在將新原則推出到一個用戶端之前進行煙霧測試。

完整的鍵參考在 [Claude Code 設定](/docs/zh-TW/settings#available-settings) 中。操作員首先尋求的鍵：

```yaml theme={null}
managed:
  policies:
    - match: {}
      cli:
        # 模型存取（也在 /v1/messages 伺服器端強制執行）
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]

        # 權限原則
        permissions:
          deny:
            - "WebFetch"
            - "Read(./.env)"
            - "Read(./secrets/**)"
          disableBypassPermissionsMode: disable   # 阻止 --dangerously-skip-permissions
        allowManagedPermissionRulesOnly: true     # 忽略使用者/專案權限規則

        # 推送到 CLI 程序的環境。DISABLE_UPDATES 阻止
        # 背景和手動更新；DISABLE_AUTOUPDATER 僅停止
        # 背景更新。
        env:
          DISABLE_UPDATES: "1"                    # 透過您自己的發佈固定版本

        # 組織範圍的掛鉤。掛鉤命令在開發人員機器上執行，而不是
        # 閘道，因此路徑必須存在於原則中的每個用戶端 OS 上。
        hooks:
          PostToolUse:
            - matcher: "Edit|Write"
              hooks:
                - { type: command, command: /usr/local/bin/audit-edit.sh }
```

| 鍵                                          | 由以下強制執行  | 效果                                                                                                                                                                |
| ------------------------------------------ | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `availableModels`                          | 閘道 + CLI | 模型允許清單。也在 `/v1/messages` 檢查，因此修補的用戶端無法繞過它。                                                                                                                        |
| `permissions.allow` / `.deny`              | CLI      | 工具和命令規則。請參閱[權限](/docs/zh-TW/permissions)。                                                                                                                              |
| `permissions.disableBypassPermissionsMode` | CLI      | 設定為 `disable` 以阻止 [`bypassPermissions`](/docs/zh-TW/permission-modes#skip-all-checks-with-bypasspermissions-mode)，自動批准每個工具呼叫的模式，以及 `--dangerously-skip-permissions` 旗標 |
| `allowManagedPermissionRulesOnly`          | CLI      | 當 `true` 時，使用者和專案權限規則被忽略；僅此文件中的規則適用                                                                                                                               |
| `env`                                      | CLI      | 合併到 CLI 程序的環境變數。用於遙測、自動更新和模型名稱覆蓋。                                                                                                                                 |
| `hooks`                                    | CLI      | 組織範圍的[掛鉤](/docs/zh-TW/hooks)                                                                                                                                           |

因為這些設定透過網路到達，CLI 在應用任何可以執行 shell 命令或改變流量去往何處的內容之前，向每個開發人員顯示一次性安全批准對話。對話涵蓋：

* `hooks`
* `env` 變數不在 CLI 的內建安全清單上
* shell 執行設定，例如 `apiKeyHelper` 和 `statusLine`
* 受管 CLAUDE.md 內容

安全清單決定哪些 `env` 變數無需批准即可應用：

* **在安全清單上**：自動更新和模型名稱變數
* **不在安全清單上**：代理變數、基礎 URL 變數和 `OTEL_EXPORTER_OTLP_ENDPOINT`

閘道的[遙測](#telemetry)設定推送 `OTEL_EXPORTER_OTLP_ENDPOINT`，因此設定 `telemetry.forward_to` 在每個互動式用戶端上觸發對話。使用 `-p` 旗標的非互動式執行無法顯示對話。它僅為該執行應用推送的設定，不將其記錄為已批准，因此開發人員的下一個互動式工作階段仍然顯示對話。在 v2.1.207 之前，非互動式執行將設定儲存為已批准，沒有後來的互動式工作階段為它們顯示對話。

如果開發人員拒絕，Claude Code 退出而不是應用原則。將新掛鉤或非安全環境變數推送到廣泛原則因此意味著每個符合開發人員下一次啟動時的批准提示。

`cli` 鍵在較早版本中被命名為 `settings`。該拼寫仍然被接受為別名，但新部署應使用 `cli`。

<h4 id="precedence-with-other-managed-sources">
  與其他受管來源的優先順序
</h4>

如果裝置也有本地 `managed-settings.json` 或 MDM 傳遞的原則，受管來源不合併。最高優先順序來源提供所有原則設定，按此順序排列，最高優先順序優先：

1. [原則幫助程式](/docs/zh-TW/settings#compute-managed-settings-with-a-policy-helper)
2. 閘道傳遞的設定
3. MDM，透過 Windows 上的 HKLM 登錄或 macOS 上的 plist
4. `managed-settings.json` 檔案
5. HKCU 登錄，僅在 Windows 上

嵌入主機可以透過 SDK `managedSettings` 選項提供原則。預設情況下它被忽略，僅當受管來源使用 [`parentSettingsBehavior: "merge"`](/docs/zh-TW/settings#available-settings) 選擇加入時適用，經過篩選以便它可以收緊原則但不能放鬆它。

例外是一小組跨來源鍵，在任何管理來源設定它們時被尊重；使用者可寫 HKCU 層被排除：

* `sandbox.network.allowManagedDomainsOnly` 和 `sandbox.filesystem.allowManagedReadPathsOnly`：當鎖定時，對應的允許清單跨來源聯合
* [`allowAllClaudeAiMcps`](/docs/zh-TW/settings#available-settings)：claude.ai MCP 伺服器允許清單的僅允許覆蓋
* `sandbox.bwrapPath` 和 `sandbox.socatPath`：[沙箱](/docs/zh-TW/sandboxing)幫助程式二進位檔的檔案系統路徑
* [`forceRemoteSettingsRefresh`](/docs/zh-TW/server-managed-settings)：阻止啟動直到遠端受管設定被新鮮擷取，因此設定它的 MDM 或檔案原則即使缺少該鍵的最高優先順序來源是快取遠端承載也被尊重

`allowManagedPermissionRulesOnly` 和 `disableBypassPermissionsMode` 不是跨來源的，因此僅獲勝來源的值適用。請參閱[設定優先順序](/docs/zh-TW/settings#settings-precedence)以了解設定頁面上的相同規則。

閘道原則適用於機器上的每個 Claude Code 呼叫，包括非互動式 `claude -p` 執行和由 Agent SDK 生成的工作階段。如果閘道在啟動時無法到達，已登入的工作階段退出並出現錯誤，而不是在沒有其原則的情況下執行。

<Warning>
  原則的 `cli` 區塊內的 `mcpServers` 在閘道啟動時被拒絕。不提供每個群組 MCP 發佈；透過每個裝置上的檔案型 `managed-mcp.json` 部署 MCP 伺服器或讓開發人員在本地新增它們。
</Warning>

<h3 id="telemetry">
  `telemetry`
</h3>

CLI 透過 HTTP 指標、日誌和（啟用時）追蹤傳送 OpenTelemetry Protocol (OTLP) 到閘道，閘道逐字將它們轉發到每個設定的目的地。請參閱[監控使用](/docs/zh-TW/monitoring-usage)以了解 CLI 發出的指標和事件。

CLI 使用從閘道簽發的 JWT 讀取的已驗證使用者的身分識別戳記每個匯出：`user.id`、`user.email` 和 `user.groups` 屬性。每個開發人員的成本和使用歸因因此無需開發人員端設定即可運作。

```yaml theme={null}
telemetry:
  forward_to:
    - url: https://otel-collector.internal.example.com
      headers:
        Authorization: ${OTLP_TOKEN}
      # 每個信號選擇加入。預設：僅指標。
      metrics: true
      logs: false
      traces: false
    - url: https://api.datadoghq.com/api/v2/otlp
      headers:
        DD-API-KEY: ${DD_API_KEY}
```

<Warning>
  每個目的地獨立選擇加入 `metrics`、`logs` 和 `traces`，預設僅指標。信號在敏感性上有所不同：

  * **指標**：聚合計數器，例如令牌計數、請求計數和延遲
  * **日誌和追蹤**：可以攜帶完整的 bash 命令、工具輸入和檔案路徑，涵蓋 Claude Code 在開發人員機器上執行的任何操作

  僅在具有該資料保證的存取控制和保留原則的目的地上啟用日誌和追蹤。
</Warning>

遙測在 CLI 中預設關閉。將 `telemetry.forward_to` 與 `listen.public_url` 一起設定會打開它。閘道透過 `/managed/settings` 推送五個環境變數到每個連接的用戶端：

* `CLAUDE_CODE_ENABLE_TELEMETRY=1`
* `OTEL_METRICS_EXPORTER=otlp`
* `OTEL_LOGS_EXPORTER=otlp`
* `OTEL_TRACES_EXPORTER=otlp`
* `OTEL_EXPORTER_OTLP_ENDPOINT=<public_url>`

推送的端點是從公開 URL 建立的，因此指標和日誌不需要開發人員或原則的 OTEL 設定。推送的設定在受管層應用，覆蓋開發人員在本地設定的 `OTEL_*` 變數。

[追蹤](/docs/zh-TW/monitoring-usage#traces-beta)另外需要每個用戶端上的 `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1`。閘道不推送該變數，因此透過受管原則的 `env` 區塊設定它。它不在 CLI 的安全清單上，因此透過原則傳遞它由推送的 OTLP 端點已經觸發的相同[安全批准對話](#managed)涵蓋。

protobuf 和 JSON OTLP 編碼都被轉發，任何 OpenTelemetry 相容後端都可作為目的地。

<h3 id="http-tuning">
  HTTP 調整
</h3>

四個選用的頂層區塊 `access_control`、`limits`、`timeouts` 和 `rate_limits` 調整 HTTP 表面。預設值適合大多數部署。

| 區塊               | 鍵                                              | 預設       | 說明                                                                                                                                                                            |
| ---------------- | ---------------------------------------------- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `access_control` | `allow_cidrs` / `deny_cidrs`                   | 空        | 按用戶端位址的入站 IP 允許/拒絕，在 `trusted_proxies` 解析後。`deny_cidrs` 首先檢查；符合它的用戶端即使 `allow_cidrs` 也相符也被拒絕。如果 `allow_cidrs` 非空，閘道是預設拒絕。`/healthz` 和 `/readyz` 豁免於 `allow_cidrs`。            |
| `limits`         | `max_request_bytes`                            | 32 MiB   | 最大入站請求正文；超大小請求在正文被緩衝前獲得 `413`。為大型檔案或影像請求提高。                                                                                                                                   |
| `limits`         | `max_request_header_bytes`                     | 未設定      | 設定時，超大小標頭返回 `431`                                                                                                                                                             |
| `limits`         | `max_url_length`                               | 未設定      | 設定時，過長 URL 返回 `414`                                                                                                                                                           |
| `timeouts`       | `upstream_ttfb_ms`                             | 120000   | 等待上游回應標頭（首位元組時間）的最大時間。回應正文然後以無牆鐘上限流。適用於直接 Anthropic 上游路徑；每個其他提供者受其提供者 SDK 自己的逾時限制。                                                                                            |
| `rate_limits`    | `device_authorization.max` / `.window_seconds` | 30 / 600 | 未驗證裝置授權端點上的每 IP 速率限制。為在共享出口 IP 或 NAT 後面的大型組織提高。這些限制僅適用於裝置授權登入流程，不適用於 `/v1/messages` 推論。請參閱[使用者程式碼暴力破解抵抗](/docs/zh-TW/claude-apps-gateway-deploy#user-code-brute-force-resistance)。 |
| `rate_limits`    | `device_verify.max` / `.window_seconds`        | 10 / 600 | 在 `/device` 上 `user_code` 提交的每 IP 速率限制                                                                                                                                        |

<h2 id="complete-example">
  完整範例
</h2>

此完整參考設定涵蓋每個核心部分；[HTTP 調整區塊](#http-tuning)保持其預設值。複製它，刪除您不需要的內容，並填入您的值。[快速入門](/docs/zh-TW/claude-apps-gateway#quickstart)中的設定是此的最小版本。

```yaml gateway.yaml theme={null}
# 執行方式：
#   claude gateway --config gateway.yaml
#
# 操作日誌詳細程度由 CLAUDE_GATEWAY_LOG_LEVEL
# 環境變數控制（info | warn | error；預設 info）。它不
# 影響稽核事件，始終發出。

listen:
  host: 0.0.0.0
  port: 8080
  public_url: https://claude-gateway.internal.example.com
  # 在 TLS 終止入口後面執行時省略 tls 區塊。
  # tls:
  #   cert: /certs/gateway.crt
  #   key: /certs/gateway.key
  # trusted_proxies:
  #   - 10.0.0.0/8

oidc:
  issuer: https://example.okta.com
  client_id: 0oa1example2
  client_secret: ${OIDC_CLIENT_SECRET}
  allowed_email_domains:
    - example.com
  # 當簽發者是 Okta 組織伺服器時需要，其 id_token
  # 可以省略電子郵件和群組；閘道從 /userinfo 填充它們。
  userinfo_fallback: true
  # allowed_groups: [claude-code-users]
  # Okta 僅在請求 `groups` 範圍且
  # 應用程式的群組宣告篩選允許它們時發出群組。下面的承包商原則
  # 符合群組，因此此處請求範圍。
  scopes: [openid, profile, email, offline_access, groups]
  # extra_auth_params: { access_type: offline, prompt: consent }  # Google
  # groups_claim: groups          # Entra 應用程式角色：使用 `roles`
  # email_claim: email

session:
  jwt_secret: ${GATEWAY_JWT_SECRET}   # openssl rand -base64 32
  # ttl_hours: 1

store:
  postgres_url: ${GATEWAY_POSTGRES_URL}
  # max_connections: 5

# 啟用 /v1/organizations/spend_limits（鏡像 Anthropic Admin API）
# 和 /v1/messages 上的每個開發人員支出強制執行。省略以停用。
# 上限本身透過 admin API 設定，而不是此處。
# admin:
#   write_keys:
#     - { id: terraform, key: "${GATEWAY_ADMIN_WRITE_KEY_TF}" }
#   read_keys:
#     - { id: reporting, key: "${GATEWAY_ADMIN_READ_KEY}" }
#   admin_groups: [platform-finops]
#   blocked_message: request an increase at https://go.example.com/claude-limits
#   # audit_retention_days: 365
#   # spend_retention_months: 13
#   # identity_retention_days: 90
#   # group_limit_mode: min

# enforcement:
#   fail_closed_on_error: false

upstreams:
  - provider: anthropic
    auth:
      api_key: ${ANTHROPIC_API_KEY}

  # - provider: bedrock
  #   region: us-east-1
  #   auth: {}

  # - provider: anthropicAws
  #   region: us-east-1
  #   workspace_id: wrkspc_...
  #   auth:
  #     api_key: ${ANTHROPIC_AWS_API_KEY}

  # - provider: vertex
  #   region: us-east5
  #   project_id: example-prod
  #   auth: {}

  # - provider: foundry
  #   resource: example-foundry
  #   auth: { use_azure_ad: true }

auto_include_builtin_models: true
models:
  - id: claude-opus-4-8
    label: Claude Opus 4.8
    upstream_model:
      anthropic: claude-opus-4-8
      # bedrock: us.anthropic.claude-opus-4-8
      # anthropicAws: claude-opus-4-8
      # vertex: claude-opus-4-8
      # foundry: <your-opus-deployment-name>
  - id: claude-sonnet-4-6
    label: Claude Sonnet 4.6
    upstream_model:
      anthropic: claude-sonnet-4-6
  - id: claude-haiku-4-5
    label: Claude Haiku 4.5
    upstream_model:
      anthropic: claude-haiku-4-5

managed:
  policies:
    - match: { groups: [contractors] }
      cli:
        availableModels: [claude-haiku-4-5]
        # 將預設選擇器選項限制為 availableModels 而不是
        # 層級預設，因此承包商不會在預設上獲得 400。
        enforceAvailableModels: true
        # allow 自動批准這些工具；它不阻止其餘的。
        # 新增 deny 規則以限制工具。
        permissions: { allow: [Read, Grep] }
    - match: {}
      cli:
        availableModels: [claude-opus-4-8, claude-sonnet-4-6, claude-haiku-4-5]
        permissions:
          allow: [Read, Grep, Bash, Edit]
          deny: ["WebFetch"]
        env: { HTTP_PROXY: http://proxy.example.com:8080 }

telemetry:
  forward_to:
    - url: https://otel.internal.example.com:4318
      headers:
        Authorization: Bearer ${OTEL_TOKEN}
```

<h2 id="client-side-managed-settings">
  用戶端受管設定
</h2>

上面的所有內容設定閘道伺服器。將開發人員機器指向它在每個裝置上單獨設定，透過 Claude Code 的[受管設定](/docs/zh-TW/settings#settings-files)。閘道無法自己推送這些鍵，因為它們是告訴用戶端閘道在哪裡的內容。

對於 CLI，在每個 OS `managed-settings.json` 中設定兩個鍵：

```json theme={null}
{
  "forceLoginMethod": "gateway",
  "forceLoginGatewayUrl": "https://claude-gateway.internal.example.com"
}
```

將該檔案部署到每個裝置，通常透過您的 MDM 平台。檔案路徑因平台而異：

| 平台          | 路徑                                                                                                    |
| ----------- | ----------------------------------------------------------------------------------------------------- |
| macOS       | `/Library/Application Support/ClaudeCode/managed-settings.json`，或 `com.anthropic.claudecode` 受管偏好設定網域 |
| Linux 和 WSL | `/etc/claude-code/managed-settings.json`                                                              |
| Windows     | `C:\Program Files\ClaudeCode\managed-settings.json`，或透過 HKLM 登錄的群組原則                                  |

`forceLoginGatewayUrl` 和 `forceLoginMethod` 的 `"gateway"` 值僅從管理員控制的受管層被尊重。開發人員在自己的 `~/.claude/settings.json` 中設定它們無效。

<h2 id="related">
  相關
</h2>

* [Claude 應用程式閘道概述](/docs/zh-TW/claude-apps-gateway)：快速入門和開發人員連接
* [部署指南](/docs/zh-TW/claude-apps-gateway-deploy)：IdP 設定、容器映像、Kubernetes 和 Cloud Run，以及操作
* [支出限制](/docs/zh-TW/claude-apps-gateway-spend-limits)：每個開發人員上限和 Admin API
