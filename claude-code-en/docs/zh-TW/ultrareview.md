> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用 Ultrareview 尋找錯誤

> 使用 /code-review ultra 在雲端執行深度多代理程式碼審查，在合併前尋找並驗證錯誤。

<Note>
  Ultrareview 是研究預覽功能。該功能、定價和可用性可能會根據反饋而變更。該命令現在以 `/code-review ultra` 的方式調用，而 `/ultrareview` 仍保留為別名。
</Note>

Ultrareview 是在 Claude Code 網路基礎設施上執行的深度程式碼審查。當您執行 `/code-review ultra` 時，Claude Code 會在遠端沙箱中啟動一群審查代理程式，以尋找您分支或拉取請求中的錯誤。

與本地 `/code-review` 或 `/review` 相比，ultrareview 提供：

* **更高的信號品質**：每個報告的發現都經過獨立重現和驗證，因此結果專注於真實錯誤而非風格建議
* **更廣泛的覆蓋範圍**：許多審查代理程式並行探索變更，這會發現本地審查可能遺漏的問題
* **無本地資源使用**：審查完全在遠端沙箱中執行，因此您的終端在執行期間保持空閒，可用於其他工作

Ultrareview 需要使用 Claude.ai 帳戶進行身份驗證，因為它在 Claude Code 網路基礎設施上執行。如果您僅使用 API 金鑰登入，請先執行 `/login` 並使用 Claude.ai 進行身份驗證。使用 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 的 Claude Code 時，Ultrareview 不可用，對於已啟用零資料保留的組織也不可用。

<h2 id="run-ultrareview-from-the-cli">
  從 CLI 執行 ultrareview
</h2>

從 Claude Code CLI 中的任何 git 儲存庫啟動審查。

```text theme={null}
/code-review ultra
```

不帶引數時，ultrareview 審查您目前分支與預設分支之間的差異，包括工作樹中任何未提交和暫存的變更。Claude Code 會將儲存庫狀態打包並上傳到遠端沙箱進行審查。

若要改為審查 GitHub 拉取請求，請傳遞 PR 編號。

```text theme={null}
/code-review ultra 1234
```

在 PR 模式中，遠端沙箱直接從主機複製拉取請求，而不是打包您的本地工作樹。PR 模式適用於 `github.com` 上的儲存庫以及管理員已連接到 Claude Code 的 [GitHub Enterprise Server](/docs/zh-TW/github-enterprise-server) 實例。

<Tip>
  如果您的儲存庫太大而無法打包，Claude Code 會提示您改用 PR 模式。推送您的分支並開啟草稿 PR，然後執行 `/code-review ultra <PR-number>`。

  如果拉取請求的差異太大，Claude Code 會在任何審查工作執行前以範圍提示拒絕審查。
</Tip>

啟動前，Claude Code 會顯示確認對話框，其中包含審查範圍（審查分支時包括檔案和行數）、您剩餘的免費執行次數和估計成本。確認後，審查會在背景中繼續進行，您可以繼續使用您的工作階段。該命令僅在您使用 `/code-review ultra` 叫用時執行；Claude 不會自動啟動 ultrareview。

<h2 id="pricing-and-free-runs">
  定價和免費執行次數
</h2>

Ultrareview 是一項高級功能，按額外使用量而非您計畫的包含使用量計費。

| 計畫                | 包含的免費執行次數 | 免費執行次數後                                                                                            |
| ----------------- | --------- | -------------------------------------------------------------------------------------------------- |
| Pro               | 3 次免費執行   | 按 [額外使用量](https://support.claude.com/zh-TW/articles/12429409-extra-usage-for-paid-claude-plans) 計費 |
| Max               | 3 次免費執行   | 按 [額外使用量](https://support.claude.com/zh-TW/articles/12429409-extra-usage-for-paid-claude-plans) 計費 |
| Team 和 Enterprise | 無         | 按 [額外使用量](https://support.claude.com/zh-TW/articles/12429409-extra-usage-for-paid-claude-plans) 計費 |

Pro 和 Max 訂閱者獲得三次免費 ultrareview 執行以試用該功能。這三次執行是每個帳戶的一次性配額，不會刷新。使用完全部三次後，或在免費執行期結束後，每次審查都會計入額外使用量，通常成本為 $5 至 $20，具體取決於變更的大小。一次執行在遠端工作階段開始時計算，因此您提前停止或未能完成的審查仍會使用一次免費執行。對於付費審查，額外使用量僅針對執行的部分計費。

由於 ultrareview 在免費執行次數外始終按額外使用量計費，您的帳戶或組織必須在啟動付費審查前啟用額外使用量。如果未啟用額外使用量，Claude Code 會阻止啟動並將您連結到計費設定，您可以在那裡開啟它。您也可以執行 `/usage-credits` 來檢查或變更您目前的設定。

<h2 id="track-a-running-review">
  追蹤執行中的審查
</h2>

審查通常需要 5 到 10 分鐘。審查作為背景工作執行，因此您可以繼續在工作階段中工作、啟動其他命令或完全關閉終端。

使用 `/tasks` 查看執行中和已完成的審查、開啟審查的詳細檢視，或停止進行中的審查。停止審查會封存雲端工作階段，部分發現不會返回。審查完成後，已驗證的發現會在您的工作階段中顯示為通知。每個發現都包括檔案位置和問題說明，以便您可以直接要求 Claude 修復它。

<h2 id="run-ultrareview-non-interactively">
  非互動方式執行 ultrareview
</h2>

使用 `claude ultrareview` 子命令從 CI 或指令碼啟動 ultrareview，無需互動工作階段。該子命令啟動與 `/code-review ultra` 相同的審查，阻止直到遠端審查完成，將發現列印到 stdout，並在成功時以代碼 0 或失敗時以代碼 1 退出。

```bash theme={null}
claude ultrareview
claude ultrareview 1234
claude ultrareview origin/main
```

不帶引數時，該子命令審查您目前分支與預設分支之間的差異。傳遞 PR 編號以審查拉取請求，或傳遞基礎分支以改為審查與該分支的差異。叫用該子命令視為同意互動命令顯示的計費和條款提示。

進度訊息和即時工作階段 URL 會進入 stderr，以便 stdout 保持可解析。使用這些旗標來控制輸出和逾時：

| 旗標                    | 說明                           |
| --------------------- | ---------------------------- |
| `--json`              | 列印原始 `bugs.json` 承載而不是格式化的發現 |
| `--timeout <minutes>` | 等待審查完成的最大分鐘數。預設為 30          |

執行 `claude ultrareview` 需要與 `/code-review ultra` 相同的身份驗證和使用量配額配置。當審查完成時（無論是否有發現），該子命令以代碼 0 退出；當審查無法啟動、遠端工作階段出錯或逾時時，以代碼 1 退出；當使用 Ctrl-C 中斷時，以代碼 130 退出。如果您中斷該子命令，遠端審查會繼續執行；請按照列印到 stderr 的工作階段 URL 在瀏覽器中觀看它。

如需在 GitHub 拉取請求上進行自動審查，[Code Review](/docs/zh-TW/code-review) 直接與您的儲存庫整合，並將發現作為內嵌 PR 評論發佈，無需 CLI 步驟。

<h2 id="how-ultrareview-compares-to-/code-review-and-/review">
  ultrareview 與 `/code-review` 和 `/review` 的比較
</h2>

這三個命令都審查程式碼，但它們針對工作流程的不同階段。

|      | `/code-review` | `/review <pr>` | `/code-review ultra`            |
| ---- | -------------- | -------------- | ------------------------------- |
| 目標   | 您的工作差異         | GitHub 拉取請求    | 您的工作差異或拉取請求                     |
| 執行位置 | 在您的工作階段中本地執行   | 在您的工作階段中本地執行   | 在雲端沙箱中遠端執行                      |
| 深度   | 隨著努力引數調整       | 工作階段努力程度的單次審查  | 具有獨立驗證的多代理程式艦隊                  |
| 持續時間 | 幾秒到幾分鐘         | 幾秒到幾分鐘         | 大約 5 到 10 分鐘                    |
| 成本   | 計入正常使用量        | 計入正常使用量        | 免費執行次數，然後大約 $5 至 $20 每次審查作為使用額度 |
| 最適合  | 迭代時的快速反饋       | 在批准前審查隊友的拉取請求  | 在合併實質性變更前時的信心                   |

使用 `/code-review` 在工作時獲得快速反饋。使用 `/review <pr>` 審查拉取請求，就像您在批准前會做的那樣。在合併實質性變更前使用 `/code-review ultra`，當您想要更深入的審查以捕捉本地審查可能遺漏的問題時。

<h2 id="related-resources">
  相關資源
</h2>

* [Claude Code 網路版](/docs/zh-TW/claude-code-on-the-web)：了解遠端工作階段和雲端沙箱的工作原理
* [使用 ultraplan 規劃複雜變更](/docs/zh-TW/ultraplan)：ultrareview 的規劃對應項，用於前期設計工作
* [有效管理成本](/docs/zh-TW/costs)：追蹤使用量並設定支出限制
