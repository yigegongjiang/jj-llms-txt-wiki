> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用快速模式加快回應速度

> 在 Claude Code 中切換快速模式，以獲得更快的 Opus 回應。

<Note>
  快速模式處於[研究預覽](#research-preview)階段。該功能、定價和可用性可能會根據反饋而改變。
</Note>

快速模式是 Claude Opus 的高速配置，使模型速度提升最高 2.5 倍，但每個 token 的成本更高。當您需要速度進行互動式工作（如快速迭代或實時調試）時，使用 `/fast` 切換開啟，當成本比延遲更重要時，切換關閉。

快速模式不是不同的模型。它使用 Claude Opus 搭配不同的 API 配置，優先考慮速度而非成本效率。您獲得相同的品質和功能，只是回應速度更快。快速模式在 Opus 4.8 和 Opus 4.7 上受支援。它在 Sonnet、Haiku 或其他模型上不可用。

<Warning>
  Opus 4.7 的快速模式自 2026 年 6 月 25 日起已棄用，將於 2026 年 7 月 24 日移除。移除後，Opus 4.7 上的快速模式請求將返回錯誤，不會回退至標準 Opus 4.7。遷移至 Opus 4.8 以保持加速。
</Warning>

需要了解的事項：

* 使用 `/fast` 在 Claude Code CLI 中切換快速模式。VS Code 擴充功能不支援快速模式。
* 快速模式定價在 Opus 4.8 上為 $10/$50 MTok，在 Opus 4.7 上為 $30/$150 MTok。
* 適用於訂閱方案（Pro/Max/Team/Enterprise）上的所有 Claude Code 使用者和 Claude Console。
* 對於訂閱方案（Pro/Max/Team/Enterprise）上的 Claude Code 使用者，快速模式僅透過使用額度提供，不包含在訂閱速率限制中。

<h2 id="toggle-fast-mode">
  切換快速模式
</h2>

透過以下任一方式切換快速模式：

* 輸入 `/fast` 並按 Tab 鍵切換開啟或關閉
* 在您的[使用者設定檔案](/docs/zh-TW/settings)中設定 `"fastMode": true`

預設情況下，在互動式工作階段中開啟的快速模式會在工作階段之間保持。在[非互動式模式](/docs/zh-TW/headless)中，使用 `-p` 旗標時，`/fast` 僅在使用快速模式在其 [`--settings`](/docs/zh-TW/cli-reference#cli-flags) 值中啟動的工作階段中運作，例如 `claude -p --settings '{"fastMode": true}'`；切換則僅適用於該工作階段，不會儲存為您的預設值，在任何其他非互動式工作階段中，該命令會報告快速模式不可用。您可以配置快速模式在每個工作階段重設。詳見[要求每個工作階段選擇加入](#require-per-session-opt-in)以了解詳情。

為了獲得最佳成本效率，在工作階段開始時啟用快速模式，而不是在對話中途切換。詳見[了解成本權衡](#understand-the-cost-tradeoff)以了解詳情。

當您啟用快速模式時：

* 如果您使用不同的模型，Claude Code 會自動切換到 Opus
* 您會看到確認訊息：「Fast mode ON」
* 快速模式啟用時，提示旁會出現一個小的 `↯` 圖示
* 隨時再次執行 `/fast` 以檢查快速模式是否開啟或關閉

當您再次使用 `/fast` 關閉快速模式時，您仍保持在 Opus 上。模型不會還原到您之前的模型。要切換到不同的模型，請使用 `/model`。

切換到不支援快速模式的模型會關閉快速模式。切換回支援的 Opus 模型時，當您儲存的快速模式偏好設定為開啟時，它會再次開啟，這與新工作階段預設啟動的偏好設定相同。配置了[每個工作階段選擇加入](#require-per-session-opt-in)時，切換回去不會再次開啟快速模式；執行 `/fast` 以重新啟用它。對於儲存偏好設定為關閉的工作階段，快速模式永遠不會開啟，`↯` 圖示和「Fast mode ON」確認會在它啟動時出現。在 v2.1.208 之前，快速模式在您切換回去後會保持關閉，直到您再次執行 `/fast`。

Opus 4.8 是 Claude Code v2.1.154 及更新版本中的快速模式預設值。在 v2.1.142 至 v2.1.153 版本上，快速模式預設為 Opus 4.7。

<h2 id="understand-the-cost-tradeoff">
  了解成本權衡
</h2>

快速模式的每個 token 定價高於標準 Opus，乘數因模型而異：

| 模型       | 輸入 (MTok) | 輸出 (MTok) |
| -------- | --------- | --------- |
| Opus 4.8 | \$10      | \$50      |
| Opus 4.7 | \$30      | \$150     |

快速模式定價在整個 1M token 上下文視窗中是固定的。如需與標準 Opus 費率進行比較，請參閱 [Claude 定價參考](https://platform.claude.com/docs/zh-TW/about-claude/pricing)。

當您在對話中首次啟用快速模式時，您需要為整個對話上下文支付完整的快速模式未快取輸入 token 價格。對話進行得越深入，成本就越高，因此從一開始就啟用快速模式會更便宜。成本每個對話只適用一次，因此稍後關閉並再次開啟快速模式不會重複計費。如需了解機制，請參閱[快速模式如何與 prompt cache 互動](/docs/zh-TW/prompt-caching#turning-on-fast-mode)。

<h2 id="decide-when-to-use-fast-mode">
  決定何時使用快速模式
</h2>

快速模式最適合用於回應延遲比成本更重要的互動式工作：

* 快速迭代程式碼變更
* 實時調試工作階段
* 時間敏感的工作，有緊迫的截止日期

標準模式更適合：

* 速度不那麼重要的長期自主任務
* 批次處理或 CI/CD 管道
* 成本敏感的工作負載

<h3 id="fast-mode-vs-effort-level">
  快速模式與努力等級
</h3>

快速模式和努力等級都會影響回應速度，但方式不同：

| 設定          | 效果                        |
| ----------- | ------------------------- |
| **快速模式**    | 相同的模型品質、更低的延遲、更高的成本       |
| **較低的努力等級** | 較少的思考時間、更快的回應、複雜任務上可能品質較低 |

您可以結合兩者：在直接任務上使用快速模式搭配較低的[努力等級](/docs/zh-TW/model-config#adjust-effort-level)以獲得最大速度。

<h2 id="requirements">
  要求
</h2>

快速模式需要以下所有條件：

* **僅限 Anthropic API 或訂閱**：快速模式可透過 Anthropic Console API 和使用額度的 Claude 訂閱方案取得。它在 Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 或 AWS 上的 Claude Platform 上不可用。
* **啟用使用額度**：您的帳戶必須啟用使用額度，這允許超出您方案包含使用量的計費。對於個人帳戶，在您的 [Console 計費設定](https://platform.claude.com/settings/billing)中啟用此功能。對於 Team 和 Enterprise，管理員必須為組織啟用使用額度。

<Note>
  快速模式使用直接計費到使用額度，即使您的方案上還有剩餘使用量。這意味著快速模式 token 不計入您方案的包含使用量，並從第一個 token 開始按快速模式費率計費。
</Note>

* **Team 和 Enterprise 的管理員啟用**：快速模式預設對 Team 和 Enterprise 組織禁用。管理員必須明確[啟用快速模式](#enable-fast-mode-for-your-organization)，使用者才能存取它。

<Note>
  如果您的管理員尚未為您的組織啟用快速模式，`/fast` 命令將顯示「Fast mode has been disabled by your organization.」如果您組織的 [`availableModels`](/docs/zh-TW/model-config#restrict-model-selection) 允許清單排除了快速模式 Opus 模型，`/fast` 會被拒絕，顯示「is not in your organization's allowed models」。例外情況是已在允許的 Opus 模型上執行的工作階段，該模型支援快速模式：`/fast` 則在您目前的模型上啟用快速模式，而不是切換模型。
</Note>

<h3 id="enable-fast-mode-for-your-organization">
  為您的組織啟用快速模式
</h3>

您啟用快速模式的位置取決於您的組織使用的產品：

* **Console**（API 客戶）：管理員在 [Claude Code 偏好設定](https://platform.claude.com/claude-code/preferences)中啟用它
* **Claude AI**（Team 和 Enterprise）：管理員在 [管理員設定 > Claude Code](https://claude.ai/admin-settings/claude-code)中啟用它

另一個完全禁用快速模式的選項是設定 `CLAUDE_CODE_DISABLE_FAST_MODE=1`。詳見[環境變數](/docs/zh-TW/env-vars)。

<h3 id="require-per-session-opt-in">
  要求每個工作階段選擇加入
</h3>

預設情況下，快速模式在工作階段之間保持：使用者在互動式工作階段中啟用的快速模式會在未來工作階段中保持開啟。若要變更此行為，在任何[設定檔](/docs/zh-TW/settings#settings-files)中將 `fastModePerSessionOptIn` 設定為 `true`，這會導致每個工作階段以快速模式關閉開始，並要求使用者使用 `/fast` 明確啟用它。[Team](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_teams#team-&-enterprise) 或 [Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_enterprise) 方案上的擁有者可以透過[伺服器受管設定](/docs/zh-TW/server-managed-settings)在組織範圍內部署它。

```json theme={null}
{
  "fastModePerSessionOptIn": true
}
```

這對於控制執行多個並行工作階段的使用者的組織成本很有用。使用者在需要速度時仍可以使用 `/fast` 啟用快速模式，但它在每個新工作階段開始時重設。使用者的快速模式偏好設定仍會保存，因此移除此設定會還原預設的持久行為。

<h2 id="handle-rate-limits">
  處理速率限制
</h2>

快速模式與標準 Opus 有不同的速率限制。Opus 4.8 和 Opus 4.7 的快速模式共享相同的速率限制池：任一模型上的使用都會從相同的限制中扣除。當您達到快速模式速率限制或用完使用額度時：

1. 快速模式自動回退到標準速度
2. `↯` 圖示變灰以指示冷卻
3. 您以標準速度和定價繼續工作
4. 冷卻期過期時，快速模式自動重新啟用

要手動禁用快速模式而不是等待冷卻，請再次執行 `/fast`。

<h2 id="research-preview">
  研究預覽
</h2>

快速模式是研究預覽功能。這意味著：

* 該功能可能會根據反饋而改變
* 可用性和定價可能會改變
* 底層 API 配置可能會演變

透過您通常的 Anthropic 支援管道報告問題或反饋。

<h2 id="see-also">
  另請參閱
</h2>

* [模型配置](/docs/zh-TW/model-config)：切換模型和調整努力等級
* [有效管理成本](/docs/zh-TW/costs)：追蹤 token 使用量並降低成本
* [狀態行配置](/docs/zh-TW/statusline)：顯示模型和上下文資訊
