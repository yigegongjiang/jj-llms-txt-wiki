> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 將工作階段輸出分享為成品

> 成品將 Claude Code 的工作轉變為可在 claude.ai 上的即時互動頁面，您可以保持私人、與您的組織分享，或發佈到公開連結。

<Note>
  成品適用於 Pro、Max、Team 和 Enterprise 方案，並需要使用 [`/login`](/docs/zh-TW/setup#authenticate) 登入的工作階段。請參閱[可用性](#availability)以了解完整的需求集合。
</Note>

成品是一個即時互動網頁，Claude Code 從您的工作階段發佈到 claude.ai 上的私人 URL。您在瀏覽器中開啟它，當工作階段繼續進行時，它會就地更新。當您想讓其他人看到它時，可以從頁面標題中分享它。例如，使用成品來引導審查者查看帶有註解差異的拉取請求、從工作階段資料建立儀表板，或保持調查時間軸，隨著 Claude 工作而填入。

<Frame>
  <img src="https://mintcdn.com/claude-code/kaHIYYMIYMYPxQg9/images/artifacts-viewer.png?fit=max&auto=format&n=kaHIYYMIYMYPxQg9&q=85&s=dbfd671cdb0d15f49f808b9e89778fe1" alt="在 claude.ai/code/artifact 的瀏覽器中開啟的成品。檢視器標題顯示成品標題 acme-funnel-fix、分享按鈕和作者頭像。分享選單已開啟，顯示'始終分享最新版本'切換、版本選擇器顯示'分享版本 2'、'Acme 的所有人'受眾選擇器和複製連結按鈕。在標題下方，成品頁面顯示兩個並排的行動裝置模型、漏斗圖表和一行指標卡片。" width="2511" height="1890" data-path="images/artifacts-viewer.png" />
</Frame>

<h2 id="when-to-use-an-artifact">
  何時使用成品
</h2>

當終端文字不是 Claude 產生的內容的正確媒介時，請使用成品：輸出更容易查看和互動，而不是逐行閱讀。Claude 從您的工作階段可以到達的任何內容建立頁面，包括您的程式碼庫和它通過您的[連接工具](/docs/zh-TW/mcp)提取的資料，因此頁面可以顯示需要段落才能描述的內容。例如，要求 Claude：

* 引導審查者查看帶有註解差異的拉取請求
* 從工作階段已經提取的資料呈現儀表板
* 並排排列多個設計或實現選項
* 保持調查時間軸，在長任務執行時填入
* 向隊友發送連結，而不是將輸出貼到 Slack
* 發佈一個狀態板，[通過 MCP 連接器提取新鮮資料](#pull-live-data-with-mcp-connectors)，每次有人打開它時

請參閱[您可以建立的內容](#what-you-can-build)以了解與這些相符的提示，以及[通過 MCP 連接器提取即時資料](#pull-live-data-with-mcp-connectors)以了解連接器支持的板的提示。

<h3 id="what-an-artifact-is-not">
  成品不是什麼
</h3>

成品是工作的捕捉，不是應用程式。它是一個自包含的頁面，沒有後端，因此無法儲存表單輸入或提供多個路由，當有人查看它時，它通往外部資料的唯一途徑是[呼叫 MCP 連接器](#pull-live-data-with-mcp-connectors)。對於具有後端的託管內部工具，請改為在您自己的基礎設施上部署它。請參閱[頁面限制](#page-constraints)以了解完整的限制集合。

<h2 id="create-an-artifact">
  建立成品
</h2>

當輸出適合頁面時，Claude 可能會自動發佈成品，或者您可以直接要求一個。要要求，請用純語言命名功能或描述您想要的視覺輸出。任何比作為文字閱讀更容易看到的內容都是很好的候選，例如註解差異、圖表或一組要比較的選項。下面的提示是兩個示例；請參閱[您可以建立的內容](#what-you-can-build)以了解更多模式。

```text wrap theme={null}
Make an artifact that walks through this PR with the diff annotated inline.
```

```text wrap theme={null}
Build a dashboard artifact of last week's deploy failures by service and keep it updated as you investigate.
```

Claude 將頁面寫入您的專案中的 HTML 或 Markdown 檔案，然後發佈它。在發佈新成品之前，Claude Code 會要求許可；它可能會說類似 `Claude wants to publish "Deploy failures by service" (deploy-failures.html) to a private page on claude.ai` 的內容。重新發佈您已經批准的成品不會再次提示。

選擇**是**以發佈。Claude 列印 URL，您的瀏覽器會開啟到新頁面。隨時按 `Ctrl+]` 從終端重新開啟最近的成品。

Claude 為成品選擇標題和瀏覽器標籤圖示的表情符號。兩者都出現在您在 claude.ai 上的[成品庫](#share-an-artifact)和共享連結中，因此如果您想要特定的標題或圖示，請要求 Claude 使用一個。

要停止瀏覽器在發佈新成品時自動開啟，請在您的環境中設定 `CLAUDE_CODE_ARTIFACT_AUTO_OPEN=0`。

如果 Claude 回應它無法發佈，或寫入沒有連結的本地 HTML 檔案，則該工具未為您的工作階段啟用。檢查[可用性](#availability)要求。

<h2 id="update-an-artifact">
  更新成品
</h2>

要求 Claude 修訂頁面，或讓長時間執行的任務在進行時重新發佈。Claude 編輯基礎檔案並重新發佈到相同的 URL。

```text wrap theme={null}
Add a per-region breakdown below the summary chart and republish.
```

任何開啟頁面的人都會看到就地更新。每次發佈都會成為一個版本，從頁面標題中的**分享**控制項，您可以選擇檢視者看到哪個版本。

要從不同的工作階段更新成品，請給 Claude 成品的 URL 並要求它修訂。沒有 URL，新工作階段總是建立新成品，而不是更新現有的。

```text wrap theme={null}
Update https://claude.ai/code/artifact/5fbea6f3-... with today's numbers.
```

<h2 id="share-an-artifact">
  分享成品
</h2>

新的成品只有你能看到。若要分享，請在瀏覽器中開啟成品，並使用頁面標題中的**分享**控制項。標題會將你列為成品的作者，因此任何你分享給的人都能看到誰發佈了該頁面。它也會連結到你的作品集，網址為 [claude.ai/code/artifacts](https://claude.ai/code/artifacts)，其中列出你建立的每個成品。

你可以分享給誰取決於你的方案：

* **在你的組織內**：在 Team 和 Enterprise 方案上，授予組織內特定人員或所有人的存取權。檢視者以組織成員身分登入 claude.ai 以查看該頁面。
* **公開**：分享一個連結，網際網路上的任何人都可以開啟，無需登入 claude.ai。在 Pro 和 Max 方案上，公開連結是分享成品的唯一方式。在 Team 和 Enterprise 方案上，公開分享處於關閉狀態，直到擁有者[為組織啟用它](#control-public-sharing)。

<h3 id="let-someone-edit-with-you">
  讓某人與你一起編輯
</h3>

你分享給的人預設為檢視者：他們可以看到你發佈的每個版本，但無法變更頁面。在 Team 和 Enterprise 方案上，你也可以讓某人成為編輯者。在分享對話方塊中，新增一個人並將其角色從**檢視者**切換為**編輯者**。

編輯者發佈新版本的方式與你[從另一個工作階段更新成品](#update-an-artifact)相同：他們在自己的工作階段中提供成品的 URL 給 Claude，Claude 會提取目前的內容並使用他們的變更重新發佈。所有開啟該頁面的人都會即時看到每個更新。

<h2 id="pull-live-data-with-mcp-connectors">
  使用 MCP 連接器拉取即時資料
</h2>

每次有人查看 artifact 時，它都可以呼叫 [MCP 連接器](/docs/zh-TW/mcp#use-mcp-servers-from-claude-ai)，因此頁面會顯示目前資料，而不是建立該頁面的工作階段所收集的快照。來自 artifact 的連接器呼叫適用於 Pro、Max、Team 和 Enterprise 方案，並需要 Claude Code v2.1.209 或更新版本。在較早的版本上，Claude 會使用工作階段在建立頁面時收集的任何資料來發佈頁面。

若要建立由連接器支援的頁面，請在提示中命名連接器和您想要的資料：

```text wrap theme={null}
Build a dashboard artifact of our open pull requests that pulls the live list through my GitHub connector when the page loads.
```

Claude 會在發佈時宣告頁面可能呼叫的連接器，而頁面無法呼叫該宣告之外的連接器。只有來自您 claude.ai 帳戶的連接器符合條件：Claude 在宣告中命名它們，當有人查看頁面時，每個呼叫都會[透過檢視帳戶自己的連接](#how-connector-calls-work-for-viewers)執行到該連接器。您在 Claude Code 中設定的本機 MCP 伺服器（例如來自 `.mcp.json` 的伺服器）可以在 Claude 建立頁面時提供資料，但已發佈的頁面無法呼叫它們。

頁面在載入時會擷取資料，並可以按間隔重新整理，或當檢視者在頁面上使用重新整理控制項時重新整理。回應會快取在檢視者的瀏覽器中，因此重新開啟的頁面會立即從快取的回應呈現，然後使用新的結果進行更新。

<h3 id="how-connector-calls-work-for-viewers">
  連接器呼叫如何為檢視者工作
</h3>

當已發佈的頁面呼叫連接器時，呼叫會使用檢視頁面的人的帳戶，而不是發佈該頁面的人的帳戶：

* **每個檢視者使用自己的連接器**：呼叫會透過檢視帳戶的已連接工具進行，因此兩個人開啟相同的儀表板可能會看到不同的資料，取決於他們的帳戶可以存取的內容。頁面永遠不會看到任何人的認證；claude.ai 代表頁面進行呼叫。
* **檢視者先核准存取**：claude.ai 在頁面的第一個連接器呼叫之前會要求每個檢視者的許可。拒絕的檢視者，或尚未連接頁面使用的連接器的檢視者，仍然可以看到頁面，但沒有其即時區段。
* **動作也使用檢視者的帳戶**：頁面可以提供控制項，這些控制項會叫用具有副作用的連接器工具，例如發佈訊息或更新問題。動作會透過選擇控制項的任何人的帳戶進行。

當您計畫共享由連接器支援的頁面時，請要求 Claude 在每個即時區段中包含一個後備訊息，該訊息命名它所需的連接器。缺少連接的檢視者會看到要連接的內容，而不是空白區段。

呼叫連接器的 artifact 無法在任何方案上共享到公開連結。在 Team 和 Enterprise 方案上，您可以將其保持為私密或[在您的組織內共享](#share-an-artifact)。在 Pro 和 Max 方案上（其中公開連結是唯一的共享方式），由連接器支援的 artifact 對您保持私密。

<h3 id="the-page-shows-no-live-data-for-a-viewer">
  頁面對檢視者不顯示即時資料
</h3>

當由連接器支援的頁面呈現，但其即時區段對您共享的某人保持空白時，請檢查這些原因：

* **檢視者尚未連接連接器**：連接器是按帳戶的，因此每個檢視者都需要自己連接到頁面呼叫的每個連接器。他們可以在 claude.ai 上的 **Settings > Connectors** 下新增一個，然後重新載入頁面。
* **檢視者拒絕了許可要求**：拒絕在該頁面載入的其餘時間內持續。重新載入頁面會再次帶回許可要求。
* **組織已關閉連接器呼叫**：擁有者控制管理設定中的[**Enable artifact connectors** 切換](#control-connector-calls-from-artifacts)。

<h2 id="what-you-can-build">
  您可以建立的內容
</h2>

成品是單個 HTML 頁面，因此您可以用 HTML、CSS 和內聯 JavaScript 表達的任何內容都在範圍內。下面的模式最常出現。

<h3 id="walk-through-a-change">
  逐步查看變更
</h3>

要求一個頁面，在相關行旁邊呈現差異或設計變更並帶有註解，以便審查者可以在程式碼旁邊閱讀您的推理，而不是從描述中重建它。

```text wrap theme={null}
Make an artifact that walks through this PR. Render the diff with margin annotations and color-code findings by severity.
```

<h3 id="compare-alternatives">
  比較替代方案
</h3>

要求在一個頁面上有多個變體，以便您可以相互評估它們。這適用於佈局、複製、API 形狀或實現計劃。

```text wrap theme={null}
Make an artifact with four distinctly different layouts for the settings panel. Vary density and grouping, and lay them out as a grid with a one-line tradeoff under each.
```

<h3 id="tune-with-interactive-controls">
  使用互動控制項進行調整
</h3>

要求滑塊、切換或輸入欄位綁定到您正在調整的任何內容，以便您可以直接探索值，而不是描述它們。

```text wrap theme={null}
Build an artifact with sliders for the easing curve, duration, and delay so I can try values on this transition. Show the animation live as I move them.
```

<h3 id="bring-the-result-back-to-your-session">
  將結果帶回您的工作階段
</h3>

成品可以充當輕量級編輯器，用於您隨後交給 Claude 的決定。要求匯出控制項，產生您可以貼到終端的文字，以便與頁面互動的結果流回工作階段，而不是停留在頁面上。

```text wrap theme={null}
Make a triage board artifact with each open issue as a draggable card across Now, Next, Later, and Cut columns. Add a "Copy as prompt" button that gives me the final ordering to paste back here.
```

<h3 id="track-work-in-progress">
  追蹤進行中的工作
</h3>

要求 Claude 在長任務執行時保持成品最新，以便任何有連結的人都可以跟進，而無需閱讀終端。

```text wrap theme={null}
Turn this migration plan into a checklist artifact. Check items off as you complete them and add a note for anything you skip.
```

<h2 id="improve-the-visual-design">
  改進視覺設計
</h2>

自 Claude Code v2.1.183 起，Claude 在建立成品時應用內建設計技能，因此頁面無需額外提示即可獲得刻意的調色板、排版和佈局。該技能還會在選擇自己的設計之前在您的專案中尋找現有的設計系統。要保持成品與您產品品牌的一致性，請在 Claude 可以找到的地方記錄您的設計令牌，例如專案的 [CLAUDE.md](/docs/zh-TW/memory) 或您的儲存庫中的主題檔案：

```markdown theme={null}
## Design system

- Colors: primary #1a4d8f, accent #f59e0b, surface #f8fafc
- Typography: Inter for body, JetBrains Mono for code
- Spacing: 8px scale, 6px border radius
```

Claude 將您的設計系統視為比其自己選擇更高的優先級，並將您的提示視為比兩者都更高的優先級。上面的標題和格式是一個示例；任何清晰的顏色、字體和間距列表都有效。

<h2 id="page-constraints">
  頁面限制
</h2>

每個成品是一個自包含的頁面。Claude Code 將您發佈的檔案包裝在 HTML 文件殼中，並在嚴格的內容安全政策 (CSP) 下提供它，這形成了頁面可以做什麼。

| 限制     | 效果                                                                                                                                                                                                         |
| :----- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 無外部請求  | CSP 阻止從任何其他主機載入的指令碼、樣式表、字體和影像，以及 `fetch`、XHR 和 WebSocket 呼叫。Claude 內聯 CSS 和 JavaScript，並將影像嵌入為資料 URI，以便頁面無需任何外部請求即可呈現。[Connector 呼叫](#pull-live-data-with-mcp-connectors)是例外：頁面將它們交給 claude.ai，由它自己進行網路呼叫。 |
| 無後端    | 成品是靜態頁面。它無法儲存通過表單提交的資料或自行驗證檢視者。它在有人檢視時取得資料的唯一方式是[呼叫 MCP connectors](#pull-live-data-with-mcp-connectors)，而不是它自己的 API。                                                                                      |
| 單頁     | 相對連結無法解析，因為沒有任何內容與頁面一起部署。對於多部分內容，Claude 使用頁面內錨點而不是單獨的檔案。                                                                                                                                                   |
| 來源檔案類型 | 發佈的檔案必須是 `.html`、`.htm` 或 `.md`。Markdown 檔案呈現為樣式化的 HTML。                                                                                                                                                   |
| 呈現大小   | 呈現的頁面必須為 16 MiB 或更小。大型嵌入影像是發佈因大小而失敗的常見原因。                                                                                                                                                                  |

生成成品使用輸出令牌，就像任何其他回應一樣，樣式化頁面比相同內容作為終端文字更耗令牌。內聯 CSS、用於互動控制項的 JavaScript，尤其是嵌入為資料 URI 的影像是主要貢獻者。要減少成品的令牌成本：

* 對於圖表，優先使用 SVG 或 HTML 和 CSS，而不是嵌入的光柵影像
* 省略您不需要的互動性
* 讓頁面總結大型資料集，而不是完整內聯它們

<h2 id="availability">
  可用性
</h2>

成品需要下面的每個條件。當不滿足其中一個時，Claude 寫入本地 HTML 檔案或說它無法發佈。

| 要求    | 可用時間                                                                                                                                                                                                                                                                                                            |
| :---- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 方案    | Pro、Max、Team 或 Enterprise。在 Pro 和 Max 方案上，成品僅供您私人使用，不適用管理員管理。在 Team 方案上，成品預設開啟。在 Enterprise 方案上，Owner 在 claude.ai 管理設定中[啟用它們](#manage-artifacts-for-your-organization)。                                                                                                                                         |
| 驗證    | 工作階段由 claude.ai 帳戶支援：在 CLI 或桌面應用程式中使用 `/login` 登入。Claude Tag 工作階段透過代理程式的身分登入，因此不需要任何步驟。使用 API 金鑰、[閘道令牌](/docs/zh-TW/llm-gateway)或雲端提供者認證的工作階段無法發佈。                                                                                                                                                                   |
| 模型提供者 | Anthropic API。在 [Amazon Bedrock](/docs/zh-TW/amazon-bedrock)、[Google Cloud 的 Agent Platform](/docs/zh-TW/google-vertex-ai) 或 [Microsoft Foundry](/docs/zh-TW/microsoft-foundry) 上不可用。                                                                                                                                          |
| 組織政策  | 客戶管理的加密金鑰 (CMEK)、HIPAA 和[零資料保留](/docs/zh-TW/zero-data-retention)未為組織啟用。                                                                                                                                                                                                                                              |
| 表面    | Claude Code CLI 版本 2.1.183 或更新版本，或 Claude 桌面應用程式版本 1.13576.0 或更新版本。[Claude Tag](https://claude.com/docs/claude-tag/overview) 工作階段在 Claude Tag 和成品都為組織啟用時也可以發佈成品。在 [Agent SDK](/docs/zh-TW/agent-sdk/overview)、GitHub Action 和 MCP 伺服器上下文中預設關閉，以及當設定 [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/zh-TW/env-vars) 時。 |

<h2 id="disable-artifacts">
  停用成品
</h2>

要根據您組織的設定為您自己的工作階段關閉成品，請使用以下任何一個：

| 方法                         | 設定                                  |
| :------------------------- | :---------------------------------- |
| [設定檔](/docs/zh-TW/settings)     | `"disableArtifact": true`           |
| [環境變數](/docs/zh-TW/env-vars)    | `CLAUDE_CODE_DISABLE_ARTIFACT=1`    |
| [許可規則](/docs/zh-TW/permissions) | 將 `Artifact` 新增到 `permissions.deny` |

<h2 id="manage-artifacts-for-your-organization">
  為您的組織管理成品
</h2>

Team 和 Enterprise 方案上的擁有者從 [claude.ai 管理設定](https://claude.ai/admin-settings/claude-code)控制成品。成品內容儲存在 Anthropic 營運的基礎設施上，僅對發佈組織的已驗證成員可見，除非成品是[公開分享](#control-public-sharing)。

<h3 id="enable-or-disable-artifacts">
  啟用或停用成品
</h3>

要為整個組織啟用或停用成品，請前往**設定 > Claude Code > 功能**並使用**成品**切換。在具有角色型存取控制的 Enterprise 方案上，您還可以將成品範圍限制為特定角色：前往**設定 > 角色**、編輯角色，並在 **Claude Code** 群組下設定**成品**許可。

<h3 id="control-connector-calls-from-artifacts">
  控制來自成品的連接器呼叫
</h3>

[來自成品的連接器呼叫](#pull-live-data-with-mcp-connectors)有自己的切換，與開啟或關閉成品的**成品**切換分開。前往 [**設定 > 功能**](https://claude.ai/admin-settings/capabilities)並使用**啟用成品連接器**切換。同一個切換控制在 claude.ai 對話中建立的成品的連接器呼叫，這就是為什麼它位於**設定 > 功能**而不是**設定 > Claude Code**。

<h3 id="control-public-sharing">
  控制公開分享
</h3>

在 Team 和 Enterprise 方案上，公開分享預設為關閉，因此成員只能在組織內分享成品，直到擁有者開啟它。要讓成員將成品發佈到任何人都可以檢視而無需登入的公開連結，請前往**設定 > Claude Code > 功能**並在**成品**切換下開啟**外部分享**。將其關閉會阻止透過現有公開連結的存取，而不會變更每個成品的對象；如果您重新啟用它，存取將恢復。

<h3 id="set-a-retention-policy">
  設定保留政策
</h3>

要設定在自動刪除之前保留成品的時間，請前往**設定 > 資料和隱私控制**。您可以為仍然是其作者私人的成品和已共享的成品設定單獨的保留期。

<h3 id="review-the-audit-log">
  檢查稽核日誌
</h3>

發佈、分享和刪除成品各自出現在您組織的稽核日誌中，位於 `claude_artifact_*` 事件類型下，與在 claude.ai 對話中建立的成品使用的相同系列。

<h3 id="allowlist-the-viewer-domain">
  允許列表檢視器網域
</h3>

claude.ai 上的檢視器從沙箱化的 `*.claudeusercontent.com` 來源載入每個成品。如果您的組織限制出站網路存取，請將該網域新增到您的允許列表中，以及 `claude.ai`。請參閱[網路存取要求](/docs/zh-TW/network-config#network-access-requirements)以了解完整清單。

<h3 id="list-and-delete-artifacts-with-the-compliance-api">
  使用 Compliance API 列出和刪除成品
</h3>

[Compliance API](https://docs.claude.com/en/api/compliance) 提供端點來列出組織的成品、檢索特定版本的內容和刪除成品：

| 方法       | 端點                                                                  |
| :------- | :------------------------------------------------------------------ |
| `GET`    | `/v1/compliance/code/artifacts`                                     |
| `GET`    | `/v1/compliance/code/artifacts/{artifact_id}/versions/{version_id}` |
| `DELETE` | `/v1/compliance/code/artifacts/{artifact_id}`                       |

有關請求和回應架構，請參閱 [Compliance API 參考](https://docs.claude.com/en/api/compliance/code/artifacts)。

<h2 id="related-resources">
  相關資源
</h2>

* 瀏覽與成品配對的[提示模式和工作流程](/docs/zh-TW/prompt-library)
* 將您重複使用的成品提示轉變為[技能](/docs/zh-TW/skills)，以便您可以將其作為命令呼叫
* [連接 MCP 伺服器](/docs/zh-TW/mcp)，以便 Claude 可以將資料提取到成品中，同時建置頁面
