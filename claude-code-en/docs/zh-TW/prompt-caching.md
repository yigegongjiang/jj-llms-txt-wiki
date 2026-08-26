> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code 如何使用 prompt caching

> Claude Code 自動管理 prompt caching。了解為什麼模型切換會觸發緩慢的未快取轉換、`/compact` 的成本、為什麼 CLAUDE.md 編輯在會話中期不適用，以及如何檢查快取命中率。

Prompt caching 使 Claude Code 更快速且更具成本效益。沒有快取的情況下，API 會在每次轉換時重新處理您的完整歷史記錄。有了快取，它會重複使用已經處理過的內容，只對變更的部分進行新工作。

Claude Code 會為您處理 prompt caching，除非您[禁用它](#disable-prompt-caching)。了解 prompt caching 的工作原理仍然很有用，因為某些操作會使快取失效，使下一個回應變得更慢且更昂貴，同時它重新建立快取。本頁涵蓋哪些操作會這樣做、為什麼某些設定需要等待重新啟動才能應用，以及當使用量看起來很高時如何檢查快取效能。

<h2 id="how-the-cache-is-organized">
  快取的組織方式
</h2>

每次您在 Claude Code 中發送訊息時，它都會發出新的 API 請求。模型在請求之間不記得任何內容，因此 Claude Code 會重新發送完整的上下文：系統提示、您的專案上下文、每個先前的訊息和工具結果，以及您的新訊息。新內容會附加在末尾，這意味著每個請求的大部分內容與前一個請求相同。Prompt caching 是 API 避免重新處理未變更部分的方式。

API 通過將每個請求的開始部分（稱為前綴）與最近處理過的內容進行匹配來進行快取。在正常轉換中，前綴是整個先前的請求，只有最新的交換是新的。匹配是精確的，因此前綴中任何地方的變更都會重新計算其後的所有內容。沒有按檔案或按段的快取。請參閱 API 參考中的[prompt caching 如何工作](https://platform.claude.com/docs/zh-TW/build-with-claude/prompt-caching#how-prompt-caching-works)以了解基礎機制。

<img src="https://mintcdn.com/claude-code/VbDJw--l6T9a9Wvm/images/prompt-caching-prefix.svg?fit=max&auto=format&n=VbDJw--l6T9a9Wvm&q=85&s=f2e8f0b8298a50305fe428ca3f1d1594" className="dark:hidden" alt="四個轉換顯示為不斷增長的水平條。每個轉換的請求包含前一個轉換的所有內容加上附加在末尾的最新交換。在第二和第三個轉換中，未變更的前綴從快取中讀取，只有新的交換被處理。在第四個轉換中，系統提示已變更，因此前綴不再匹配，整個請求被重新處理並寫入。" width="720" height="454" data-path="images/prompt-caching-prefix.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/prompt-caching-prefix-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=297dc1c639f0915cae858d0c4b6f3be5" className="hidden dark:block" alt="四個轉換顯示為不斷增長的水平條。每個轉換的請求包含前一個轉換的所有內容加上附加在末尾的最新交換。在第二和第三個轉換中，未變更的前綴從快取中讀取，只有新的交換被處理。在第四個轉換中，系統提示已變更，因此前綴不再匹配，整個請求被重新處理並寫入。" width="720" height="454" data-path="images/prompt-caching-prefix-dark.svg" />

為了充分利用前綴匹配，Claude Code 會組織每個請求，使轉換之間很少變更的內容首先出現：

| 層     | 內容                   | 變更時機                             |
| ----- | -------------------- | -------------------------------- |
| 系統提示  | 核心指令、工具定義、輸出風格       | 載入的工具定義集合變更，或 Claude Code 升級     |
| 專案上下文 | CLAUDE.md、自動記憶、無範圍規則 | 會話開始，或在 `/clear` 或 `/compact` 之後 |
| 對話    | 您的訊息、Claude 的回應、工具結果 | 每次轉換                             |

對對話層的變更會保留系統提示和專案上下文的快取。對系統提示的變更會使所有內容失效，因為所有後續內容現在位於不同的前綴後面。第三列提供常見觸發器而不是詳盡列表，下面的部分涵蓋完整集合，包括在會話開始時固定的輸出風格等內容。

前綴匹配規則解釋了本頁上的大多數行為。例如，[Plan Mode](/docs/zh-TW/permission-modes#analyze-before-you-edit-with-plan-mode) 和[技能加載](/docs/zh-TW/skills)將其指令附加為對話訊息，因此快取的前綴保持完整。

兩個設定根本不是提示文本的一部分，因此它們不會出現在層表中，但兩者都是快取金鑰的一部分：

* **Model**：每個模型都有自己的快取。切換模型會重新計算整個請求，即使內容相同。請參閱下面的[切換模型](#switching-models)。
* **Effort level**：同一模型的每個努力級別都有自己的快取。在會話中期變更它會重新計算整個請求，Claude Code 會要求您在應用變更前確認。請參閱下面的[變更努力級別](#changing-effort-level)。

<Tip>
  在會話開始時選擇您的模型和努力級別，然後在任務之間的自然中斷處保存 `/compact`。您在任務中期進行的變更越少，快取命中率就越高。
</Tip>

<h3 id="where-the-cache-lives">
  快取位置
</h3>

快取發生在伺服器端，在提供您的模型的任何基礎設施中。位置取決於您如何進行身份驗證：

* **API 金鑰、Claude 訂閱或 [Claude Platform on AWS](/docs/zh-TW/claude-platform-on-aws)**：快取位於 Anthropic 的基礎設施中，通過 [Claude API](https://platform.claude.com/docs) 訪問
* **Amazon Bedrock 或 Google Cloud 的 Agent Platform**：快取位於您的雲端提供商的服務基礎設施中
* **Microsoft Foundry**：請求路由到 Anthropic 的基礎設施
* **自訂 `ANTHROPIC_BASE_URL` 或 [LLM gateway](/docs/zh-TW/llm-gateway)**：快取位於您的請求轉發到的位置，快取是否工作取決於閘道

有關每個提供商存儲和處理的內容，請參閱[資料使用](/docs/zh-TW/data-usage)。無論快取位於何處，條目在一段時間不活動後過期，[下面的快取生命週期](#cache-lifetime)涵蓋 TTL 以及如何延長它。

<h2 id="actions-that-invalidate-the-cache">
  使快取失效的操作
</h2>

這些操作會導致下一個請求錯過部分或全部快取。您會看到一次較慢、更昂貴的轉換，之後新的前綴會被快取。一旦您知道它們有成本，大多數都可以在任務中期避免。模型切換可能看起來是免費的，直到您注意到隨後的較慢轉換。

* [切換模型](#switching-models)
* [變更努力程度](#changing-effort-level)
* [開啟快速模式](#turning-on-fast-mode)
* [連接或斷開 MCP 伺服器](#connecting-or-disconnecting-an-mcp-server)
* [啟用或停用外掛程式](#enabling-or-disabling-a-plugin)
* [拒絕整個工具](#denying-an-entire-tool)
* [壓縮對話](#compacting-the-conversation)
* [升級 Claude Code](#upgrading-claude-code)

<h3 id="switching-models">
  切換模型
</h3>

每個模型都有自己的快取。使用 [`/model`](/docs/zh-TW/model-config#setting-your-model) 切換意味著下一個請求讀取整個對話歷史記錄而沒有快取命中，即使內容相同。

[`opusplan` 模型設定](/docs/zh-TW/model-config#opusplan-model-setting)在 Plan Mode 期間解析為 Opus，在執行期間解析為 Sonnet，因此每個 Plan Mode 切換都是模型切換並啟動新的快取。

[Fable 5 上的自動模型回退](/docs/zh-TW/model-config#automatic-model-fallback)也是模型切換。當安全分類器標記請求時，Claude Code 會在預設 Opus 模型上重新執行它，會話會在那裡繼續。

<h3 id="changing-effort-level">
  變更努力程度
</h3>

快取由[努力程度](/docs/zh-TW/model-config#adjust-effort-level)以及模型作為鍵，因此使用 `/effort` 切換意味著下一個請求讀取整個對話歷史記錄而沒有快取命中。一旦對話開始，Claude Code 會在應用會使快取失效的努力程度變更之前顯示確認對話框。解析為已生效的相同程度的變更（例如明確設定模型的預設值）會跳過對話框並保持快取。

<h3 id="turning-on-fast-mode">
  開啟快速模式
</h3>

啟用[快速模式](/docs/zh-TW/fast-mode)會新增一個請求標頭，該標頭是快取鍵的一部分，因此下一個請求讀取整個對話歷史記錄而沒有快取命中。這些未快取的輸入令牌按[快速模式費率](/docs/zh-TW/fast-mode#understand-the-cost-tradeoff)計費，這就是為什麼在會話開始時開啟它的成本比在長會話深處開啟它的成本要低。從非 Opus 模型啟用快速模式也會[切換您的模型](#switching-models)，這本身會啟動新的快取。

成本每個對話應用一次。在第一個快速模式轉換之後，Claude Code 會繼續發送標頭，並且只改變請求的速度設定，這不是快取鍵的一部分。關閉快速模式、[在速率限制後自動回退到標準速度](/docs/zh-TW/fast-mode#handle-rate-limits)，以及稍後重新開啟它都會保持快取。`/clear` 和 `/compact` 會重設此設定，因為它們無論如何都會在這些點重新建立快取。

<h3 id="connecting-or-disconnecting-an-mcp-server">
  連接或斷開 MCP 伺服器
</h3>

工具定義位於系統提示層中，因此當請求之間的工具定義集合變更時，快取會失效。切換[顧問工具](/docs/zh-TW/advisor)是例外：其定義位於快取中斷點之後，因此啟用或停用 `/advisor` 會保持快取的前綴完整。[MCP 伺服器](/docs/zh-TW/mcp)變更是否執行此操作取決於其工具是否由[工具搜尋](/docs/zh-TW/mcp#scale-with-mcp-tool-search)延遲或載入到前綴中：

* **延遲工具**，在支援的模型上為預設值：伺服器連接、斷開連接或變更其工具列表只會附加新內容，不會擾亂已快取的任何內容。
* **載入到前綴中的工具**：對它們的任何變更都會使快取失效。這發生在[工具搜尋不可用或已停用](/docs/zh-TW/mcp#configure-tool-search)時，例如在 Google Cloud 的 Agent Platform 上或使用自訂 `ANTHROPIC_BASE_URL` 閘道時。它也發生在標記為 [`alwaysLoad`](/docs/zh-TW/mcp#exempt-a-server-from-deferral) 的伺服器或工具上，以及由[基於閾值的載入](/docs/zh-TW/mcp#configure-tool-search)保持在前面的定義上。

當工具載入到前綴中時，失效的最常見原因是伺服器在會話中期連接或斷開連接，這可能在沒有您採取任何操作的情況下發生：stdio 伺服器的進程退出、HTTP 會話過期，或伺服器[在暫時性故障後自動重新連接](/docs/zh-TW/mcp#automatic-reconnection)。連接的伺服器也可以推送[動態工具更新](/docs/zh-TW/mcp#dynamic-tool-updates)來變更其工具列表。

編輯您的 MCP 配置本身不會變更快取。新配置只有在重新啟動後才會生效，這是伺服器連接或斷開連接的時候。

<h3 id="enabling-or-disabling-a-plugin">
  啟用或停用外掛程式
</h3>

[外掛程式](/docs/zh-TW/plugins)捆綁多個元件類型，變更的成本取決於外掛程式提供的元件。Skills、commands、agents、hooks、LSP 伺服器、monitors 和 themes 永遠不會使快取失效：它們添加到請求的任何內容都會附加在現有對話之後，因此下一個請求為新內容付費，但仍然從快取中讀取它之前的所有內容。

例外是提供 [MCP 伺服器](/docs/zh-TW/plugins-reference#mcp-servers)的外掛程式。啟用或停用一個遵循與[連接或斷開 MCP 伺服器](#connecting-or-disconnecting-an-mcp-server)相同的規則：當伺服器的工具被延遲時快取會保留，當它們載入到前綴中時下一個請求會重新讀取整個對話。

外掛程式變更在您運行 [`/reload-plugins`](/docs/zh-TW/discover-plugins#apply-plugin-changes-without-restarting) 或啟動新會話時應用。成本（無論是附加公告還是完整重新讀取）會在重新載入後的第一個轉換時顯示，而不是在您運行 `/plugin install`、`/plugin enable` 或 `/plugin disable` 時。自 v2.1.163 起，當重新載入會觸發完整重新讀取時，`/reload-plugins` 會顯示警告並不應用重新載入。傳遞 `--force` 以強制應用。

停用您在會話中較早啟用的外掛程式會恢復先前的請求形狀。如果該前綴仍在其[快取生命週期](#cache-lifetime)內，下一個請求會讀取較舊的快取項目，而不是重新建立。

<h3 id="denying-an-entire-tool">
  拒絕整個工具
</h3>

添加裸工具名稱（如 `Bash` 或 `WebFetch`）作為[拒絕規則](/docs/zh-TW/permissions#manage-permissions)會將該工具從 Claude 的上下文中完全移除。內建工具定義載入到系統提示層中，因此在會話中期添加或移除其中一個規則會使快取失效。無論您通過 `/permissions` 添加它還是通過[直接編輯設定檔](/docs/zh-TW/settings#when-edits-take-effect)，變更都會在下一個轉換時生效。

只有裸工具名稱、等效的 `Bash(*)` 形式或[工具名稱 glob](/docs/zh-TW/permissions#tool-name-wildcards)（如 `"*"`）才有此效果。匹配只有 MCP 工具的 glob（例如 `"mcp__*"`）會以相同方式移除這些工具，但當匹配的工具被[延遲](#connecting-or-disconnecting-an-mcp-server)時（預設值）會保持快取完整，因為延遲定義從未在快取的前綴中。作用域拒絕規則（如 `Bash(rm *)`）以及所有允許和詢問規則都不會改變 Claude 看到的工具。Claude Code 在 Claude 嘗試呼叫時檢查它們，保持前綴完整。

<h3 id="compacting-the-conversation">
  壓縮對話
</h3>

[壓縮](/docs/zh-TW/context-window#what-survives-compaction)用摘要替換您的訊息歷史記錄。根據設計，這會使對話層失效，因為下一個請求有一個新的、更短的歷史記錄，不與舊的共享前綴。Claude Code 重複使用系統提示層並從磁碟重新載入專案上下文，只有在 CLAUDE.md 和記憶自會話開始以來未變更時才快取命中。

為了生成摘要，Claude Code 發送一個一次性請求，其系統提示、工具和歷史記錄與您的對話相同，加上作為最終使用者訊息附加的摘要指令。因為它共享您的前綴，該請求讀取現有快取而不是重新處理完整歷史記錄。壓縮的大部分時間用於生成摘要，而不是快取未命中。隨後的轉換只為更短的摘要重新建立對話快取，因此壓縮後的轉換不是緩慢的部分。

<Tip>
  當您丟棄的上下文是您不再需要的內容時，壓縮對您有利。為了選擇其開銷何時發生，在工作中的自然中斷處（例如任務之間）運行 `/compact`，而不是等待自動壓縮在任務中期觸發。如果您走上了一條想要完全放棄的路徑，請改為[`/rewind`](#rewinding-the-conversation)到較早的轉換。重新開始會截斷回到已經快取的前綴，而不是像壓縮那樣建立新的。
</Tip>

<h3 id="upgrading-claude-code">
  升級 Claude Code
</h3>

新的 Claude Code 版本通常會更新系統提示或工具定義，因此升級後的第一個請求會從頂部重新建立快取。[自動更新](/docs/zh-TW/setup#auto-updates)在後台下載新版本，但在下次啟動時應用它們，從不在會話中期，因此您會看到這是重新啟動後的未快取第一個轉換，而不是會話期間的驚喜。設定 `DISABLE_AUTOUPDATER=1` 以控制何時應用升級。

<Note>
  在升級後[恢復會話](/docs/zh-TW/sessions#resume-a-session)會重新處理整個對話歷史記錄而沒有快取命中，因為歷史記錄現在位於不同的系統提示後面。成本隨著恢復的對話有多長而擴展，因此回到長會話的第一個轉換可能是您發送的最昂貴的請求。
</Note>

<h2 id="actions-that-keep-the-cache">
  保留快取的操作
</h2>

這些操作要麼附加到對話的末尾，要麼根本不觸及請求。其中一些，例如編輯 CLAUDE.md 或變更輸出風格，也是為什麼設定變更需要等待重新啟動才能應用的原因。

* [編輯您的儲存庫中的檔案](#editing-files-in-your-repository)
* [在會話中期編輯 CLAUDE.md](#editing-claude-md-mid-session)
* [變更輸出風格](#changing-output-style)
* [變更權限模式](#changing-permission-mode)
* [調用技能和命令](#invoking-skills-and-commands)
* [運行 `/recap`](#running-%2Frecap)
* [重新開始對話](#rewinding-the-conversation)
* [生成子代理](#subagents-and-the-cache)

<h3 id="editing-files-in-your-repository">
  編輯您的儲存庫中的檔案
</h3>

檔案內容只有在 Claude 讀取它們時才進入上下文，讀取會附加到對話。編輯 Claude 之前讀過的檔案不會追溯變更歷史記錄中的較早讀取。相反，Claude Code 附加一個 `<system-reminder>` 注意到檔案已變更，如果需要，Claude 會重新讀取它。

<h3 id="editing-claude-md-mid-session">
  在會話中期編輯 CLAUDE.md
</h3>

您的專案根目錄和使用者級別 CLAUDE.md 檔案在會話開始時讀取一次並保存在記憶中。在會話中期編輯它們不會使快取失效，但編輯也不會應用。Claude 繼續使用在會話開始時加載的版本。新內容在下一個 `/clear`、`/compact` 或重新啟動時加載。

[子目錄中的嵌套 CLAUDE.md 檔案](/docs/zh-TW/memory)和[帶有 `paths:` frontmatter 的規則](/docs/zh-TW/memory#path-specific-rules)稍後加載，當 Claude 首次讀取匹配檔案時。在它加載之前編輯一個確實會生效。加載後，內容是對話歷史記錄的一部分，因此會話中期的編輯不會追溯變更它。

<h3 id="changing-output-style">
  變更輸出風格
</h3>

[輸出風格](/docs/zh-TW/output-styles)是系統提示的一部分，Claude Code 在會話開始時讀取一次。通過 `/config` 或 `outputStyle` 設定在會話中期變更它不會使快取失效，但變更也不會應用。Claude 繼續使用在會話開始時加載的風格。新風格在下一個 `/clear` 或重新啟動時加載。

<h3 id="changing-permission-mode">
  變更權限模式
</h3>

在[權限模式](/docs/zh-TW/permission-modes)之間切換，例如從預設切換到接受編輯，不會變更系統提示或工具定義，因此模式變更是快取安全的。例外是使用 [`opusplan`](/docs/zh-TW/model-config#opusplan-model-setting) 模型設定的 Plan Mode，它在進入或離開 Plan Mode 時在 Opus 和 Sonnet 之間切換模型。這使模式切換成為[模型切換](#switching-models)。

<h3 id="invoking-skills-and-commands">
  調用技能和命令
</h3>

[技能](/docs/zh-TW/skills)和[命令](/docs/zh-TW/commands)在調用點將其指令注入為使用者訊息。對話中較早的任何內容都不會變更。

<h3 id="running-/recap">
  運行 `/recap`
</h3>

[`/recap`](/docs/zh-TW/interactive-mode#session-recap) 生成一個摘要以在您的終端中顯示。與 `/compact` 不同，它將摘要附加為命令輸出而不是替換您的訊息歷史記錄，因此快取的前綴保持完整。

<h3 id="rewinding-the-conversation">
  重新開始對話
</h3>

[`/rewind`](/docs/zh-TW/checkpointing) 將您的對話截斷回到較早的轉換。剩餘的歷史記錄是快取在該點建立時的相同內容，系統提示和專案上下文層未變更，因此下一個請求命中較早的快取條目。自那時以來的每個轉換都通過該前綴讀取，即使原始轉換比 TTL 更久遠，也保持條目溫暖。

恢復檔案檢查點與對話一起對快取沒有單獨的影響。檔案內容只有在 Claude 讀取它們時才進入上下文，與[編輯您的儲存庫中的檔案](#editing-files-in-your-repository)相同。

<h2 id="cache-lifetime">
  快取生命週期
</h2>

快取的前綴在一段時間不活動後過期。每個命中快取的請求都會重置計時器，因此只要您繼續工作，快取就會保持溫暖。在足夠長的間隙之後，下一個請求會重新計算完整輸入並重新建立快取，這就是為什麼在步開後回來的第一個轉換可能會明顯變慢。

生存時間 (TTL) 控制快取存活的間隙有多長。API 提供兩個：五分鐘 TTL 和[一小時 TTL](https://platform.claude.com/docs/zh-TW/build-with-claude/prompt-caching#1-hour-cache-duration)，它通過更長的中斷保持快取溫暖，但[以更高的速率計費快取寫入](https://platform.claude.com/docs/zh-TW/build-with-claude/prompt-caching#pricing)。Claude Code 根據您如何進行身份驗證為您選擇 TTL，您可以使用環境變數覆蓋它。

<h3 id="on-a-claude-subscription">
  在 Claude 訂閱上
</h3>

在 Claude 訂閱上，Claude Code 自動請求一小時 TTL。使用量包含在您的計畫中，而不是按令牌計費，因此更長的 TTL 對您沒有額外成本，只會影響快取保持溫暖的時間。

如果您已超過計畫的使用量限制，Claude Code 正在使用[使用額度](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)，您將被計費該使用量，因此 Claude Code 會自動將 TTL 降低到五分鐘。

<h3 id="on-an-api-key-or-third-party-provider">
  在 API 金鑰或第三方提供商上
</h3>

在 API 金鑰、Amazon Bedrock、Google Cloud 的 Agent Platform、Microsoft Foundry 或 Claude Platform on AWS 上，您支付按令牌費率，因此 TTL 預設保持在更便宜的五分鐘。要選擇加入[一小時 TTL](https://platform.claude.com/docs/zh-TW/build-with-claude/prompt-caching#1-hour-cache-duration)，設定 `ENABLE_PROMPT_CACHING_1H=1`。

在 Amazon Bedrock 上，prompt caching 支援、最小可快取前綴長度和一小時 TTL 可用性都因模型而異。如果快取令牌計數保持為零，請檢查 Amazon Bedrock 文件中的[支援的模型、區域和限制](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models)。

<h3 id="override-the-ttl">
  覆蓋 TTL
</h3>

設定 `FORCE_PROMPT_CACHING_5M=1` 以強制五分鐘 TTL，無論身份驗證如何。當您調試快取行為、比較兩個 TTL 或覆蓋在[受管設定](/docs/zh-TW/settings#settings-files)中設定的 `ENABLE_PROMPT_CACHING_1H` 時，這很有用。

<h2 id="cache-scope">
  快取範圍
</h2>

在 Claude Code 中，快取有效地限定在一台機器和目錄。系統提示嵌入工作目錄、平台、shell、OS 版本和自動記憶路徑，因此在不同目錄中的兩個會話建立不同的前綴並錯過彼此的快取。這包括同一儲存庫的 worktrees，因為每個 worktree 都有自己的工作目錄。

您在同一目錄中並行運行的會話建立匹配的前綴並讀取彼此的快取。順序會話只有在啟動時的 git 狀態快照匹配時才共享前綴，因為系統提示也捕獲分支和最近的提交。

基礎 API 快取更廣泛。快取在組織之間隔離，在某些提供商上，[在組織內的工作區之間隔離](https://platform.claude.com/docs/zh-TW/build-with-claude/prompt-caching#cache-storage-and-sharing)。在這些邊界內，任何兩個具有相同模型和前綴的請求讀取相同的快取。對於運行自動化流程艦隊的 Agent SDK 呼叫者，請參閱[改進跨使用者和機器的 prompt caching](/docs/zh-TW/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines)以抑制系統提示的按機器部分並跨機器共享快取。

<h2 id="check-cache-performance">
  檢查快取效能
</h2>

快取效能顯示為 API 在每個回應上報告的兩個令牌計數。最直接的方式是觀看它們實時是[狀態行指令碼](/docs/zh-TW/statusline)，它讀取 `current_usage` 物件：

| 欄位                            | 含義                              |
| ----------------------------- | ------------------------------- |
| `cache_creation_input_tokens` | 在此轉換上寫入快取的令牌，按快取寫入速率計費          |
| `cache_read_input_tokens`     | 在此轉換上從快取提供的令牌，按標準輸入速率的大約 10% 計費 |

高讀取與建立比率意味著快取工作良好。如果建立在轉換之間保持高位，您的前綴中有些東西在變更。[使快取失效的操作](#actions-that-invalidate-the-cache)部分列出了常見原因。

為了在整個組織中獲得可見性，OpenTelemetry 匯出器報告每個使用者和會話的快取讀取和建立令牌。請參閱[監控使用](/docs/zh-TW/monitoring-usage)以了解指標和事件屬性參考。

<h2 id="subagents-and-the-cache">
  子代理和快取
</h2>

[子代理](/docs/zh-TW/sub-agents)開始自己的對話，具有自己的系統提示和工具集，與父代分開。它建立自己的快取，在第一次呼叫時沒有快取命中，並在自己的轉換中預熱。子代理使用五分鐘 TTL，即使在訂閱上，因為自動一小時 TTL 適用於主對話。

父代的快取不受影響。從父代的角度來看，子代理的呼叫和結果附加到對話，保留父代的前綴完整。

[分支](/docs/zh-TW/sub-agents#fork-the-current-conversation)相比之下，完全繼承父代的系統提示、工具和對話歷史記錄，因此其第一個請求讀取父代的快取。[壓縮對話](#compacting-the-conversation)中描述的壓縮摘要呼叫使用相同的前綴共享方法。

<h2 id="disable-prompt-caching">
  禁用 prompt caching
</h2>

禁用快取在使用特定模型或提供商調試快取行為時偶爾很有用。要關閉它，請將以下環境變數之一設定為 `1`：

| 變數                              | 效果           |
| ------------------------------- | ------------ |
| `DISABLE_PROMPT_CACHING`        | 為所有模型禁用      |
| `DISABLE_PROMPT_CACHING_HAIKU`  | 僅為 Haiku 禁用  |
| `DISABLE_PROMPT_CACHING_SONNET` | 僅為 Sonnet 禁用 |
| `DISABLE_PROMPT_CACHING_OPUS`   | 僅為 Opus 禁用   |
| `DISABLE_PROMPT_CACHING_FABLE`  | 僅為 Fable 禁用  |

要在整個組織中設定快取策略，請將這些或[TTL 變數](#cache-lifetime)中的任何一個放在[受管設定](/docs/zh-TW/settings#settings-files)的 `env` 區塊中。為了正常使用，請保持快取啟用。

<h2 id="related-resources">
  相關資源
</h2>

* [從建立 Claude Code 中學到的課程：Prompt caching 就是一切](https://claude.com/blog/lessons-from-building-claude-code-prompt-caching-is-everything)：Plan Mode、延遲工具加載和壓縮的設計基本原理
* [探索上下文視窗](/docs/zh-TW/context-window)：什麼加載到上下文以及何時加載
* [減少令牌使用](/docs/zh-TW/costs#reduce-token-usage)：超越快取的策略，用於管理上下文大小
* [追蹤和減少成本](/docs/zh-TW/agent-sdk/cost-tracking)：Agent SDK 呼叫者的快取令牌追蹤和 TTL 配置
* [Prompt caching](https://platform.claude.com/docs/en/build-with-claude/prompt-caching)：基礎 API 機制、中斷點和定價
