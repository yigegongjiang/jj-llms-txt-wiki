> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude 應用程式閘道支出限制

> 透過 Claude 應用程式閘道限制每位開發人員的每日、每週或每月支出。使用管理員 API 設定限制，閘道會在每個請求上即時執行這些限制。

支出限制會限制每位開發人員在指定的一天、一週或一個月內透過您的 [Claude 應用程式閘道](/docs/zh-TW/claude-apps-gateway) 可以花費的金額。當開發人員超過其限制時，閘道會在他們的下一個請求上返回 `429`，並在該期間重置或管理員提高限制之前阻止他們。使用支出限制為每位開發人員、群組或整個組織設定共享認證的上限。

Claude 應用程式閘道會透過一個共享的上游認證轉發所有推論，因此您的提供商的帳單會將所有內容歸屬於該認證，而不是個別開發人員。沒有按開發人員的限制，一個失控的代理程式群隊可能會花費組織的整個承諾。支出限制是閘道在該共享帳單之上的按開發人員檢視和斷路器。

<h2 id="set-a-cap">
  設定上限
</h2>

配置了 [`admin:`](/docs/zh-TW/claude-apps-gateway-config#admin) 區塊在 `gateway.yaml` 中後，閘道會在 `/v1/organizations/spend_limits` 提供管理員 API，並在每個推論請求上即時執行上限。上限本身是透過該 API 設定的，而不是在 `gateway.yaml` 中；每個 `POST /v1/organizations/spend_limits` 請求會從 `{scope, amount, period}` 建立或替換一個上限。該 API 鏡像了 Anthropic 公開 [Admin API](https://platform.claude.com/docs/en/manage-claude/admin-api) 支出限制端點的線路形狀，因此針對該合約編寫的 HTTP 用戶端可以透過變更其基礎 URL 來針對閘道。

此請求為每位開發人員設定組織範圍的預設值，每月 \$500：

```bash theme={null}
curl -sS https://claude-gateway.internal.example.com/v1/organizations/spend_limits \
  -H "x-api-key: $GATEWAY_ADMIN_WRITE_KEY" \
  -H "Content-Type: application/json" \
  -d '{"scope": {"type": "organization"}, "amount": "50000", "period": "monthly"}'
```

此請求在 `contractors` 群組的每位成員上分層設定更嚴格的每日 \$100 上限：

```bash theme={null}
curl -sS https://claude-gateway.internal.example.com/v1/organizations/spend_limits \
  -H "x-api-key: $GATEWAY_ADMIN_WRITE_KEY" \
  -H "Content-Type: application/json" \
  -d '{"scope": {"type": "rbac_group", "rbac_group_id": "contractors"}, "amount": "10000", "period": "daily"}'
```

| 欄位           | 值                                  | 說明                                                                                                                                                                                                                                                               |
| ------------ | ---------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `scope.type` | `user`、`rbac_group`、`organization` | `user` 透過其 OpenID Connect (OIDC) `sub`（您的身份提供商指派的穩定使用者 ID）針對一位開發人員；將其作為 `scope.user_id` 傳遞。`rbac_group` 透過名稱針對 [IdP 群組](/docs/zh-TW/claude-apps-gateway-config#managed)；將其作為 `scope.rbac_group_id` 傳遞。`organization` 是組織範圍的預設值。閘道接受全部三種；Anthropic 的公開 `POST` 目前僅限使用者。 |
| `amount`     | USD 美分的整數字串，或 `null`               | `null` 表示無限制。`"0"` 是零上限，會阻止每個請求。                                                                                                                                                                                                                                 |
| `period`     | `daily`、`weekly`、`monthly`         | 一個範圍可以為每個期間保持一個上限，每個都獨立執行：如果開發人員超過其中任何一個，就會被阻止。                                                                                                                                                                                                                  |

群組或組織上限是每位成員繼承的按座位預設值，而不是共享池。每個期間，開發人員的有效上限按此順序解析：按使用者覆蓋、其群組上限中最嚴格的、組織預設值，然後無限制。[`admin.group_limit_mode: max`](/docs/zh-TW/claude-apps-gateway-config#admin) 會將多群組平局決勝改為最寬鬆的。

<h3 id="authenticate-to-the-admin-api">
  向管理員 API 進行身份驗證
</h3>

傳送以下其中之一：

* 與 [`admin.write_keys`](/docs/zh-TW/claude-apps-gateway-config#admin) 中的金鑰相符的 `x-api-key` 標頭以獲得完全存取權限，或 `admin.read_keys` 以獲得僅 `GET` 存取權限。每個金鑰都帶有一個 `id`，在稽核日誌中顯示為 `admin-key:<id>`，因此為 Terraform、CI 和每個自動化提供各自的金鑰。
* 其 `groups` 聲明包含 [`admin.admin_groups`](/docs/zh-TW/claude-apps-gateway-config#admin) 之一的閘道持有人令牌。這是完全存取權限，稽核為 `oidc:<sub>`，因此對人類管理員更好。

<h2 id="how-enforcement-works">
  執行方式
</h2>

在每個 `/v1/messages` 請求上，閘道在一個 Postgres 查詢中解析開發人員的上限和期間至今的支出。如果他們超過任何上限，請求會返回 `429`，其中 `error.type: billing_error` 和標頭 `x-should-retry: false`。訊息是 `spend limit reached`，後面跟著您的 [`admin.blocked_message`](/docs/zh-TW/claude-apps-gateway-config#admin)（如果已設定）。

`/v1/messages/count_tokens` 被豁免。令牌計數是免費的，因此無論上限狀態如何都會執行。

在每個回應之後，使用量計量器從回應中讀取令牌計數，當其流向用戶端時，以 USD 列表價格對其進行定價，並為所有三個期間儲存桶增加 Postgres 計數器。計量器是流上的單一讀取器，因此用戶端的位元組不受影響，計量失敗不會破壞回應。

支出限制根據 USD 列表價格的令牌計數估計支出；它們是斷路器，而不是發票。如需權威計費，請根據您提供商自己的使用情況報告進行協調，例如 Anthropic 使用情況與成本管理員 API、Amazon Bedrock 上的呼叫日誌或 Google Cloud 上的雲端監控。

定價使用與 Claude Code CLI 用於自己的成本顯示相同的表格，在 Anthropic、Amazon Bedrock (`us.anthropic.…-v1:0`)、Google Cloud 的 Agent Platform (`claude-…@date`) 和 Microsoft Foundry ID 形式中具有相同的模型 ID 規範化。表格無法識別的模型 ID（例如 Microsoft Foundry 部署名稱或推論設定檔 ARN）以未知模型預設層級 \$5/\$25 每百萬輸入/輸出令牌定價，而不是零，因此無法識別的 ID 無法透過不計量來繞過上限。閘道在啟動時和執行時每個 ID 一次警告當模型透過後備定價時。

用戶端中止也會被計費。上游僅在流的終端框架中報告輸出令牌，因此中止的流不會攜帶它們。計量器從流式內容大小保持保守的下限估計，約每令牌四個字元，並在終端使用量框架遺失時計費。完整流始終計費上游報告的計數。沒有這個，被限制的開發人員可以流式傳輸輸出並在結束前立即中止每個請求，花費而不被計數。

<h3 id="postgres-availability">
  Postgres 可用性
</h3>

預檢查使用兩秒超時查詢 Postgres。如果存儲無法到達或超時，執行預設會開放失敗：請求繼續進行，閘道記錄警告。設定 [`enforcement.fail_closed_on_error: true`](/docs/zh-TW/claude-apps-gateway-config#enforcement) 改為關閉失敗，這會返回相同的 `429 billing_error`，訊息為 `spend limit unavailable`。開放失敗可防止存儲中斷成為推論中斷；關閉失敗保證沒有無計量支出。

<h2 id="admin-api-reference">
  管理員 API 參考
</h2>

以下端點在 `/v1/organizations/spend_limits` 下提供。

| 方法和路徑                                          | 說明                                             |
| ---------------------------------------------- | ---------------------------------------------- |
| `GET /v1/organizations/spend_limits`           | 列出已配置的上限。查詢：`?limit=&after_id=&before_id=`。    |
| `POST /v1/organizations/spend_limits`          | 為 `{scope, period}` 建立或替換上限。                   |
| `GET /v1/organizations/spend_limits/{id}`      | 透過其 `spl_` 前綴 ID 擷取一個上限。                       |
| `DELETE /v1/organizations/spend_limits/{id}`   | 刪除一個上限。返回 `{type: "spend_limit_deleted", id}`。 |
| `GET /v1/organizations/spend_limits/effective` | 每個主體每個期間的已解析上限和至今支出。                           |
| `GET /v1/organizations/spend_limits/audit`     | 管理員變更軌跡，最新優先。查詢：`?limit=`。                     |

慣例鏡像 Anthropic 的管理員 API：

* 每個物件上的 `type`
* `spl_` 前綴 ID
* 金額為 USD 美分的整數字串；`POST` 拒絕任何其他 `currency`，返回 `400`
* `{type: "error", error: {type, message}, request_id}` 錯誤信封
* 每個管理員回應上的 `request-id` 回應標頭，成功或錯誤，與正文的 `request_id` 相符

每個變更在同一交易中向 `admin_audit` 寫入前/後行，歸屬於 `admin-key:<id>` 或 `oidc:<sub>`。

閘道僅提供支出限制端點。其他管理員 API 表面（例如 `spend_limit_increase_requests` 佇列）不是閘道管理員 API 的一部分。

<h3 id="/effective">
  `/effective`
</h3>

`GET /v1/organizations/spend_limits/effective` 返回 Anthropic 的 `SpendSummary` 架構：每行是一個主體的一個期間，具有已解析的上限、期間至今的支出和一個 `actor` 物件。閘道特定的差異：

* `user_id` 是 OIDC `sub`。
* `actor.name` 和 `actor.email_address` 為 `null`，直到主體透過閘道的第一個推論請求。閘道沒有使用者目錄；它從每個使用者自己的工作階段 JWT 記錄最後看到的值。
* 每行也帶有一個 `groups` 陣列，主體最後看到的 IdP 群組。這是一個閘道擴充，因此管理員 UI 可以顯示適用的每個上限層級；Anthropic 形狀的用戶端會忽略它。
* 沒有 `user_ids[]` 篩選，它列出有記錄支出的主體，因為閘道無法列舉所有組織成員。

群組來源的上限根據那些最後看到的群組解析，使用與執行相同的 `group_limit_mode` 平局決勝，因此檢視器顯示實際適用的上限。

| 查詢參數             | 說明                                              |
| ---------------- | ----------------------------------------------- |
| `user_ids[]`     | 可重複。按 OIDC `sub` 篩選到特定主體。                       |
| `period[]`       | 可重複。篩選到 `daily`、`weekly` 或 `monthly` 行。         |
| `sort`           | `spend_desc` 首先列出最高支出者。需要恰好一個 `period[]`。       |
| `q`              | 不區分大小寫的子字串篩選，涵蓋 OIDC `sub`、最後看到的電子郵件和最後看到的顯示名稱。 |
| `limit` / `page` | 頁面大小（1–1000，預設 20）和前一個回應的 `next_page` 中的不透明游標。  |

<Warning>
  `q=` 和 `user_ids[]=` 乘坐 GET 查詢字串，因此任何前置代理或負載平衡器都會在其存取日誌中擷取它們。如果您的 PII 日誌原則很嚴格，請在那裡清理這些參數。
</Warning>

<h3 id="/audit">
  `/audit`
</h3>

返回支出限制變更軌跡：誰變更了哪個上限、前/後快照和選擇性原因，最新優先。`has_more` 是精確的。此端點遵循本地管理員 API 慣例，而不是第一方線路形狀。

<h3 id="pagination">
  分頁
</h3>

原始列表按 `after_id` 和 `before_id` 分頁，它們是互斥的 `spl_…` ID；結果按建立排序，`has_more` 反映遍歷方向。`/effective` 按傳回的不透明 `next_page` 令牌分頁，作為 `?page=` 傳遞，主體按升序排序，因此在記錄支出時頁面保持穩定。`limit` 在兩者上都是 1–1000，預設 20。

<h2 id="data-lifecycle">
  資料生命週期
</h2>

閘道保持四個支出相關表；每小時掃描執行保留視窗：

| 表格                 | 內容                                 | 保留                                                                                      |
| ------------------ | ---------------------------------- | --------------------------------------------------------------------------------------- |
| `spend`            | 按主體期間至今計數器（美分）                     | [`admin.spend_retention_months`](/docs/zh-TW/claude-apps-gateway-config#admin)，預設 13         |
| `spend_limits`     | 已配置的上限                             | 直到透過 API 刪除                                                                             |
| `admin_audit`      | 變更軌跡                               | [`admin.audit_retention_days`](/docs/zh-TW/claude-apps-gateway-config#admin)，預設 365          |
| `principal_emails` | 每個主體最後看到的電子郵件、顯示名稱和 IdP 群組。包含 PII。 | [`admin.identity_retention_days`](/docs/zh-TW/claude-apps-gateway-config#admin) 自上次活動起，預設 90 |

`identity_retention_days` 刻意短於 `spend_retention_months`：已取消佈建的身份停止重新整理並老化，而其匿名支出計數器保留用於年度比較報告。

當開發人員離開時，透過 `DELETE /v1/organizations/spend_limits/{id}` 刪除任何按使用者上限；其支出和身份行按上述保留視窗老化。若要立即清除一個人，用於離職或資料主體存取請求 (DSAR)，直接針對閘道資料庫執行 `DELETE FROM principal_emails WHERE principal = '<sub>'`。這會移除唯一保持其電子郵件、名稱和群組的表格。`spend` 和 `admin_audit` 行僅參考偽匿名 OIDC `sub`，並按其自己的視窗老化。

<h2 id="related">
  相關
</h2>

* [`admin` 和 `enforcement` 配置](/docs/zh-TW/claude-apps-gateway-config#admin)：啟用管理員 API 和調整保留
* [部署指南](/docs/zh-TW/claude-apps-gateway-deploy#postgres)：Postgres 架構和備份指導
