> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用顧問工具升級困難決策

> 將您的主要模型與更強大的顧問模型配對，Claude 在任務期間的關鍵時刻會諮詢該模型。

<Note>
  顧問工具是實驗性功能，需要 Anthropic API。它在 Amazon Bedrock、Claude Platform on AWS、Google Cloud 的 Agent Platform 或 Microsoft Foundry 上不可用。行為、定價和可用性可能會變更。
</Note>

顧問工具讓 Claude 在任務期間的關鍵時刻諮詢第二個通常更強大的模型，例如在提交方案前、遇到重複錯誤時，或在宣佈任務完成前。顧問會收到完整的對話記錄，包括每個工具呼叫和結果，並返回 Claude 在繼續前應用的指導。

顧問在 Anthropic 基礎設施上以伺服器端方式運行，作為[伺服器工具](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool)，可供訂閱和 API 計費帳戶使用。您選擇哪個模型充當顧問，Claude 決定何時呼叫它。

本頁涵蓋如何啟用顧問、接受哪些模型配對、Claude 在諮詢期間顯示什麼，以及顧問使用如何計費。

<h2 id="when-to-use-the-advisor">
  何時使用顧問
</h2>

顧問適合長期、多步驟的任務，其中大多數輪次是例行的，但計畫品質決定結果。範例包括大型重構、錯誤不斷重複的除錯會話，以及您希望在 Claude 宣佈完成前獨立檢查的任務。

在短期任務（幾乎沒有計畫空間）或每個輪次都需要最強模型的工作上，它的價值較少。對於這些情況，[切換主要模型](/docs/zh-TW/model-config#setting-your-model)，或查看[顧問與 opusplan 和子代理的比較](#compare-with-related-features)以了解獲取第二意見的其他方式。

<h2 id="enable-the-advisor">
  啟用顧問
</h2>

您可以透過三種方式設定顧問模型：

* **`/advisor` 命令**：在會話中途設定或變更顧問，並將其儲存為預設值
* **`advisorModel` 設定**：在您的[設定檔](/docs/zh-TW/settings)中配置持久預設值
* **`--advisor` 旗標**：在啟動時為單一會話設定顧問

如果其中任何一個設定了顧問模型，則對於主要模型[支援它](#choose-an-advisor-model)的會話，顧問會被啟用。若要停止使用它，請參閱[關閉顧問](#turn-the-advisor-off)。

<Note>
  若要使用 Fable 5 作為顧問，您需要 Claude Code v2.1.170 或更新版本以及您的組織的 [Fable 5 存取權](/docs/zh-TW/model-config#work-with-fable-5)。
</Note>

<h3 id="use-the-/advisor-command">
  使用 `/advisor` 命令
</h3>

執行 `/advisor` 而不帶引數以開啟列出可用顧問模型的選擇器，或直接傳遞模型：

```
/advisor opus
```

您的選擇會儲存到使用者設定中的 `advisorModel`，並在會話間保持。如果您組織的 [`availableModels`](/docs/zh-TW/model-config#restrict-model-selection) 允許清單排除了已儲存的顧問模型，則顧問不會被叫用，直到您使用 `/advisor` 選擇允許的模型。如果您目前的主要模型不支援顧問，選擇仍會被儲存，並在您使用 [`/model`](/docs/zh-TW/model-config#setting-your-model) 切換到[相容的主要模型](#choose-an-advisor-model)時啟動。

<h3 id="set-advisormodel-in-settings">
  在設定中設定 `advisorModel`
</h3>

若要在不開啟會話的情況下將顧問配置為預設值，請在設定檔中設定它：

```json theme={null}
{
  "advisorModel": "opus"
}
```

<h3 id="use-the-advisor-flag">
  使用 `--advisor` 旗標
</h3>

若要為單一會話設定顧問而不變更已儲存的設定，請使用旗標啟動：

```bash theme={null}
claude --advisor opus
```

該旗標在該會話中優先於 `advisorModel` 設定。如果會話的主要模型不支援顧問，或如果要求的顧問模型被您組織的 [`availableModels`](/docs/zh-TW/model-config#restrict-model-selection) 允許清單排除，它會以錯誤退出。

<h2 id="choose-an-advisor-model">
  選擇顧問模型
</h2>

顧問的能力必須至少與主要模型相同。每個主要模型接受的顧問為：

| 主要模型                | 接受的顧問                   | 備註                                                                                     |
| ------------------- | ----------------------- | -------------------------------------------------------------------------------------- |
| Haiku 4.5           | Fable、Opus、Sonnet       | Haiku 可以呼叫顧問但不能充當顧問                                                                    |
| Sonnet 4.6          | Fable、Opus、Sonnet       |                                                                                        |
| Sonnet 5            | Fable、Opus、Sonnet 5     | Sonnet 4.6 顧問會被拒絕                                                                      |
| Opus 4.6            | Fable、Opus、Sonnet 5     | Sonnet 5 和 Opus 4.6 的能力排名相同，因此 Opus 4.6 主要模型接受 Sonnet 5 顧問                             |
| Opus 4.7 或更新版本      | Fable、Opus 4.7、Opus 4.8 | Opus 4.7 和 Opus 4.8 的能力排名相同，因此任一個都接受另一個作為顧問。Opus 4.7 主要模型搭配 Opus 4.6 或 Sonnet 5 顧問會被拒絕 |
| Fable 5 (v2.1.170+) | Fable                   | Opus 或 Sonnet 顧問會被拒絕                                                                   |

Fable 5 需要 Claude Code v2.1.170 或更新版本以及 Fable 5 存取權，無論它是充當主要模型還是顧問。

將顧問設定為 `opus`、`sonnet` 或 `fable`。這些別名解析為每個模型的最新版本。您也可以傳遞完整的模型 ID，例如 `claude-opus-4-8`。

子代理繼承已設定的顧問，並針對其自身模型應用相同的配對檢查。

Claude Code 在傳送請求前驗證配對：

* 如果顧問的能力低於主要模型，顧問不會附加到主要模型的請求。`/advisor` 命令輸出和通知會顯示此情況。其自身模型滿足配對的子代理仍可使用顧問。
* 如果主要模型或顧問是 Claude Code 無法識別的模型，顧問不會附加。

<h3 id="common-model-pairings">
  常見模型配對
</h3>

任何接受的配對都有效。這些組合以不同方式平衡成本與能力：

| 配對                    | 何時使用                                                                               |
| --------------------- | ---------------------------------------------------------------------------------- |
| Sonnet 主要 + Opus 顧問   | Sonnet 處理例行工作，並將計畫、模糊失敗和完成檢查升級到 Opus                                               |
| Sonnet 主要 + Fable 顧問  | 在決策點獲得 Fable 5 指導，而無需全程執行 Fable 5。需要 v2.1.170 或更新版本以及 Fable 5 存取權                  |
| Haiku 主要 + Opus 顧問    | 具有強大計畫的最低成本主要模型。預期成本高於單獨使用 Haiku，但低於將主要模型切換到 Sonnet 或 Opus                         |
| Opus 主要 + Opus 顧問     | 第二個 Opus 審查第一個。適用於獨立檢查比成本更重要的高風險任務                                                 |
| Fable 主要 + Fable 顧問   | 當 Fable 5 可用時（v2.1.170+）的最高能力配對。Fable 是比 Opus 和 Sonnet 更高的層級，因此是 Fable 主要模型唯一接受的顧問 |
| Sonnet 主要 + Sonnet 顧問 | 用於捕捉例行疏漏的較低成本第二意見                                                                  |

<h2 id="when-claude-consults-the-advisor">
  Claude 何時諮詢顧問
</h2>

Claude 決定何時呼叫顧問。它傾向於在提交方案前、錯誤不斷重複時以及在宣佈任務完成前進行諮詢，但時機是由模型驅動而非基於規則的。

您可以在提示中要求諮詢，就像您會要求任何工具一樣，例如 `consult the advisor before you continue`。沒有設定來限制或強制顧問呼叫；如果您希望 Claude 在任務期間更頻繁或更少地諮詢顧問，請在您的指示中說明。

<h2 id="what-you-see-during-a-session">
  會話期間您看到的內容
</h2>

當 Claude 呼叫顧問時，文字記錄會在呼叫進行中顯示帶有顧問模型名稱的 `Advising` 行。當結果返回時，該行確認顧問已審查對話。按 `Ctrl+O` 展開它並閱讀顧問的完整指導。

Claude 通常遵循顧問的指導，但在其自身證據與特定聲明相矛盾時進行調整：如果建議的步驟在嘗試時失敗，或檔案內容與建議相矛盾，Claude 會表面衝突而不是無條件地遵循指導。

顧問始終收到完整的對話，Claude 控制時機。如需更多控制或不同的配置，請參閱[顧問與子代理和 opusplan 的比較](#compare-with-related-features)。

<h2 id="cost">
  成本
</h2>

每個顧問呼叫都會將對話發送到顧問模型，因此除了主要模型的使用外，還會以顧問模型的費率消耗代幣。使用 API 計費時，顧問代幣按顧問模型的輸入和輸出費率計費。在訂閱計畫上，顧問使用計入您計畫的使用限制。

Claude 在決策點而非每個輪次都呼叫顧問，因此將更快的主要模型與更強大的顧問配對通常比全程執行更強大的模型成本更低。顧問使用計入 [`/usage`](/docs/zh-TW/costs#track-your-costs) 顯示的會話總計。

有關顧問代幣如何在 API 回應中報告的資訊，請參閱 Claude API 文件中的[使用和計費](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool#usage-and-billing)。

<h2 id="impact-on-prompt-caching">
  對提示快取的影響
</h2>

在會話中途啟用或停用顧問不會使主要模型的[提示快取](/docs/zh-TW/prompt-caching)失效。與[變更模型或努力等級](/docs/zh-TW/prompt-caching#actions-that-invalidate-the-cache)不同，切換 `/advisor` 會保持快取的前綴完整，顧問返回的指導會在後續輪次中作為文字記錄的一部分被快取。

顧問模型自身對對話的讀取不會被快取。每個顧問呼叫都會全新處理完整的文字記錄，呼叫之間沒有重複使用。

<h2 id="requirements">
  需求
</h2>

顧問工具需要以下所有條件：

* **僅限 Anthropic API**：顧問是伺服器執行的工具。它在 Amazon Bedrock、Claude Platform on AWS、Google Cloud 的 Agent Platform 或 Microsoft Foundry 上不可用。透過配置有 `ANTHROPIC_BASE_URL` 的 [LLM 閘道](/docs/zh-TW/llm-gateway)，可用性取決於閘道是否將請求完整轉發到 Anthropic API。
* **支援的主要模型**：Opus 4.6 或更新版本、Sonnet 4.6 或更新版本，或 Haiku 4.5。Fable 5 在 Claude Code v2.1.170 或更新版本上也符合條件。

<h2 id="turn-the-advisor-off">
  關閉顧問
</h2>

若要停止使用顧問並清除已儲存的 `advisorModel`，執行 `/advisor off` 或在 `/advisor` 選擇器中選擇 **No advisor**：

```
/advisor off
```

若要完全停用顧問工具，設定 `CLAUDE_CODE_DISABLE_ADVISOR_TOOL=1`。`/advisor` 命令變為無法使用，任何已設定的 `advisorModel` 都會被忽略。`--advisor` 旗標被接受但沒有效果；傳遞它的現有指令碼會繼續運作而不會出現錯誤。請參閱[環境變數](/docs/zh-TW/env-vars)。

<h2 id="compare-with-related-features">
  與相關功能的比較
</h2>

顧問是結合模型優勢的幾種方式之一。根據您希望何時涉及第二個模型來選擇。

| 方法                                                       | 更強大的模型何時執行                                                                                        | 如何啟動              |
| -------------------------------------------------------- | ------------------------------------------------------------------------------------------------- | ----------------- |
| 顧問工具                                                     | 在任務中途的決策點                                                                                         | Claude 在需要指導時呼叫它  |
| [`opusplan`](/docs/zh-TW/model-config#opusplan-model-setting) | 在計畫模式期間（當 [`availableModels`](/docs/zh-TW/model-config#restrict-model-selection) 允許時），然後切換到 Sonnet 以執行 | 您進入計畫模式           |
| [子代理](/docs/zh-TW/sub-agents#choose-a-model)搭配 `model` 設定     | 針對整個委派的子任務                                                                                        | Claude 委派，或您呼叫子代理 |
| [`/model`](/docs/zh-TW/model-config#setting-your-model)       | 針對所有後續輪次                                                                                          | 您切換模型             |

<h2 id="see-also">
  另請參閱
</h2>

* [模型配置](/docs/zh-TW/model-config)：切換模型、設定努力等級並使用 `opusplan`
* [有效管理成本](/docs/zh-TW/costs)：跨模型追蹤代幣使用
* [Claude API 中的顧問工具](https://platform.claude.com/docs/en/agents-and-tools/tool-use/advisor-tool)：了解基礎伺服器工具，或直接從 Messages API 使用它
* [顧問策略](https://claude.com/blog/the-advisor-strategy)：為什麼將快速主要模型與更強大的顧問配對有效
