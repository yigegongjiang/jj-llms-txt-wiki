> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Slack 中的 Claude Code

> 直接從您的 Slack 工作區委派編碼任務

<Note>
  Slack 中的 Claude Code 正被 [Claude Tag](https://claude.com/product/tag) 取代，適用於 Team 和 Enterprise 工作區。Claude Tag 以 @Claude 身份作為您組織的共享身份運行，具有管理員配置的存取權限，在同一個 Slack 應用程式下，因此無需重新安裝，現有設定在轉換期間繼續運作。若要切換工作區，請參閱[從較早的 Slack 中的 Claude 遷移](https://claude.com/docs/claude-tag/admins/migrate-from-earlier)。
</Note>

Slack 中的 Claude Code 將 Claude Code 的強大功能直接帶入您的 Slack 工作區。當您提及 `@Claude` 並附帶編碼任務時，Claude 會自動檢測意圖並在網路上建立 Claude Code 工作階段，讓您無需離開團隊對話即可委派開發工作。

此整合建立在現有的 Claude for Slack 應用程式基礎上，但為編碼相關請求添加了智能路由到網路上的 Claude Code。每個工作階段在您自己的 Claude 帳戶下運行，使用您連接的儲存庫和您的方案限制。

<h2 id="use-cases">
  使用案例
</h2>

* **錯誤調查和修復**：要求 Claude 在 Slack 頻道中報告錯誤時立即調查和修復。
* **快速代碼審查和修改**：讓 Claude 根據團隊反饋實現小功能或重構代碼。
* **協作調試**：當團隊討論提供關鍵背景資訊（例如錯誤重現或用戶報告）時，Claude 可以使用該資訊來指導其調試方法。
* **並行任務執行**：在 Slack 中啟動編碼任務，同時繼續其他工作，完成時接收通知。

<h2 id="prerequisites">
  先決條件
</h2>

在使用 Slack 中的 Claude Code 之前，請確保您具有以下條件：

| 要求               | 詳情                                                                         |
| :--------------- | :------------------------------------------------------------------------- |
| Claude 計畫        | Pro、Max、Team 或 Enterprise，具有 Claude Code 存取權限（高級席位或 Chat + Claude Code 席位） |
| 網路上的 Claude Code | 必須啟用對[網路上的 Claude Code](/docs/zh-TW/claude-code-on-the-web) 的存取                 |
| GitHub 帳戶        | 連接到網路上的 Claude Code，至少有一個存儲庫已驗證                                            |
| Slack 驗證         | 您的 Slack 帳戶通過 Claude 應用程式連接到您的 Claude 帳戶                                   |

<h2 id="setting-up-claude-code-in-slack">
  在 Slack 中設定 Claude Code
</h2>

<Steps>
  <Step title="在 Slack 中安裝 Claude 應用程式">
    工作區管理員必須從 Slack 應用程式市場安裝 Claude 應用程式。訪問 [Slack 應用程式市場](https://slack.com/marketplace/A08SF47R6P4) 並點擊'Add to Slack'以開始安裝過程。
  </Step>

  <Step title="連接您的 Claude 帳戶">
    應用程式安裝後，驗證您的個人 Claude 帳戶：

    1. 通過點擊您的應用程式部分中的「Claude」在 Slack 中打開 Claude 應用程式
    2. 導航到應用程式首頁標籤
    3. 點擊「Connect」以將您的 Slack 帳戶與您的 Claude 帳戶連接
    4. 在您的瀏覽器中完成驗證流程
  </Step>

  <Step title="在網路上配置 Claude Code">
    確保您在網路上的 Claude Code 已正確配置：

    * 訪問 [claude.ai/code](https://claude.ai/code) 並使用您連接到 Slack 的同一帳戶登入
    * 如果尚未連接，請連接您的 GitHub 帳戶
    * 驗證至少一個您希望 Claude 使用的存儲庫
  </Step>

  <Step title="選擇您的路由模式">
    連接帳戶後，配置 Claude 如何在 Slack 中處理您的訊息。導航到 Slack 中的 Claude 應用程式首頁以找到**路由模式**設定。

    | 模式          | 行為                                                                                                     |
    | :---------- | :----------------------------------------------------------------------------------------------------- |
    | **僅代碼**     | Claude 將所有 @mentions 路由到 Claude Code 工作階段。最適合使用 Claude in Slack 專門用於開發任務的團隊。                           |
    | **代碼 + 聊天** | Claude 分析每條訊息並智能地在 Claude Code（用於編碼任務）和 Claude Chat（用於寫作、分析和一般問題）之間路由。最適合希望為所有類型工作提供單一 @Claude 入口點的團隊。 |

    <Note>
      在代碼 + 聊天模式中，如果 Claude 將訊息路由到聊天但您想要編碼工作階段，您可以點擊「Retry as Code」以改為建立 Claude Code 工作階段。同樣，如果它被路由到代碼但您想要聊天工作階段，您可以在該執行緒中選擇該選項。
    </Note>
  </Step>

  <Step title="將 Claude 添加到頻道">
    Claude 在安裝後不會自動添加到任何頻道。要在頻道中使用 Claude，請通過在該頻道中輸入 `/invite @Claude` 來邀請它。Claude 只能在已添加它的頻道中回應 @mentions。
  </Step>
</Steps>

<h2 id="how-it-works">
  工作原理
</h2>

<h3 id="automatic-detection">
  自動檢測
</h3>

當您在 Slack 頻道或執行緒中提及 @Claude 時，Claude 會自動分析您的訊息以確定它是否是編碼任務。如果 Claude 檢測到編碼意圖，它將把您的請求路由到網路上的 Claude Code，而不是作為常規聊天助手回應。

您也可以明確告訴 Claude 將請求作為編碼任務處理，即使它沒有自動檢測到。

<Note>
  Slack 中的 Claude Code 僅在頻道（公開或私人）中工作。它在直接訊息 (DM) 中不起作用。
</Note>

<h3 id="context-gathering">
  背景資訊收集
</h3>

**來自執行緒**：當您在執行緒中 @mention Claude 時，它會從該執行緒中的所有訊息收集背景資訊以理解完整對話。

**來自頻道**：當直接在頻道中提及時，Claude 會查看最近的頻道訊息以獲取相關背景資訊。

此背景資訊幫助 Claude 理解問題、選擇適當的存儲庫並指導其任務方法。

<Warning>
  當在 Slack 中調用 @Claude 時，Claude 會獲得對對話背景資訊的存取權限以更好地理解您的請求。Claude 可能會遵循背景資訊中其他訊息的指示，因此用戶應確保僅在受信任的 Slack 對話中使用 Claude。
</Warning>

<h3 id="session-flow">
  工作階段流程
</h3>

1. **啟動**：您 @mention Claude 並提出編碼請求
2. **檢測**：Claude 分析您的訊息並檢測編碼意圖
3. **工作階段建立**：在 claude.ai/code 上建立新的 Claude Code 工作階段
4. **進度更新**：Claude 在工作進行時向您的 Slack 執行緒發佈狀態更新
5. **完成**：完成後，Claude @mentions 您並提供摘要和操作按鈕
6. **審查**：點擊'View Session'以查看完整記錄，或點擊'Create PR'以開啟拉取請求

<h2 id="user-interface-elements">
  用戶介面元素
</h2>

<h3 id="app-home">
  應用程式首頁
</h3>

應用程式首頁標籤顯示您的連接狀態，並允許您連接或斷開您的 Claude 帳戶與 Slack 的連接。

<h3 id="message-actions">
  訊息操作
</h3>

* **View Session**：在您的瀏覽器中打開完整的 Claude Code 工作階段，您可以在其中查看所有執行的工作、繼續工作階段或提出其他請求。
* **Create PR**：直接從工作階段的更改建立拉取請求。
* **Retry as Code**：如果 Claude 最初作為聊天助手回應但您想要編碼工作階段，點擊此按鈕以將請求重試為 Claude Code 任務。
* **Change Repo**：允許您選擇不同的存儲庫，如果 Claude 選擇不正確。

<h3 id="repository-selection">
  存儲庫選擇
</h3>

Claude 根據您的 Slack 對話中的背景資訊自動選擇存儲庫。如果多個存儲庫可能適用，Claude 可能會顯示一個下拉菜單，允許您選擇正確的存儲庫。

<h2 id="access-and-permissions">
  存取和權限
</h2>

<h3 id="user-level-access">
  用戶級別存取
</h3>

| 存取類型             | 要求                                            |
| :--------------- | :-------------------------------------------- |
| Claude Code 工作階段 | 每個用戶在其自己的 Claude 帳戶下運行工作階段                    |
| 使用情況和速率限制        | 工作階段計入個人用戶的計畫限制                               |
| 存儲庫存取            | 用戶只能存取他們個人連接的存儲庫                              |
| 工作階段歷史記錄         | 工作階段出現在您的 Claude Code 歷史記錄中，位於 claude.ai/code |

<h3 id="workspace-level-access">
  工作區級別存取
</h3>

Slack 工作區管理員控制 Claude 應用程式是否可在其工作區中使用：

| 控制                 | 描述                                                   |
| :----------------- | :--------------------------------------------------- |
| 應用程式安裝             | 工作區管理員決定是否從 Slack 應用程式市場安裝 Claude 應用程式               |
| Enterprise Grid 分發 | 對於 Enterprise Grid 組織，組織管理員可以控制哪些工作區有權存取 Claude 應用程式 |
| 應用程式移除             | 從工作區移除應用程式會立即撤銷該工作區中所有用戶的存取權限                        |

<h3 id="channel-based-access-control">
  基於頻道的存取控制
</h3>

Claude 在安裝後不會自動添加到任何頻道。用戶必須明確邀請 Claude 到他們想要使用它的頻道：

* **需要邀請**：在任何頻道中輸入 `/invite @Claude` 以將 Claude 添加到該頻道
* **頻道成員資格控制存取**：Claude 只能在已添加它的頻道中回應 @mentions
* **通過頻道進行存取控制**：管理員可以通過管理 Claude 被邀請到哪些頻道以及誰有權存取這些頻道來控制誰使用 Claude Code
* **私人頻道支援**：Claude 在公開和私人頻道中都可以工作，為團隊提供了控制可見性的靈活性

此基於頻道的模型允許團隊將 Claude Code 使用限制在特定頻道，提供了超越工作區級別權限的額外存取控制層。

<h2 id="what’s-accessible-where">
  在何處可以存取什麼
</h2>

**在 Slack 中**：您將看到狀態更新、完成摘要和操作按鈕。完整記錄被保留並始終可存取。

**在網路上**：完整的 Claude Code 工作階段，包含完整對話歷史記錄、所有代碼更改、文件操作以及繼續工作階段或建立拉取請求的能力。

對於 Enterprise 和 Team 帳戶，從 Slack 中的 Claude 建立的工作階段會自動對組織可見。有關更多詳情，請參閱 [Claude Code on the Web 共享](/docs/zh-TW/claude-code-on-the-web#share-sessions)。

<h2 id="best-practices">
  最佳實踐
</h2>

<h3 id="writing-effective-requests">
  撰寫有效的請求
</h3>

* **具體明確**：在相關時包括文件名、函數名或錯誤訊息。
* **提供背景資訊**：如果從對話中不清楚，請提及存儲庫或專案。
* **定義成功**：解釋'完成'的樣子——Claude 應該編寫測試嗎？更新文檔？建立拉取請求？
* **使用執行緒**：在討論錯誤或功能時在執行緒中回覆，以便 Claude 可以收集完整背景資訊。

<h3 id="when-to-use-slack-vs-web">
  何時使用 Slack 與網路
</h3>

**在以下情況下使用 Slack**：背景資訊已存在於 Slack 討論中、您想要非同步啟動任務或與需要可見性的隊友協作。

**直接在網路上使用**：當您需要上傳文件、想要在開發期間進行實時互動或正在處理更長、更複雜的任務時。

<h2 id="troubleshooting">
  故障排除
</h2>

<h3 id="claude-code-is-not-enabled-for-your-account">
  'Claude Code 未為您的帳戶啟用'
</h3>

此錯誤表示您的 Claude 帳戶尚未有雲端環境，而不是管理員需要啟用任何內容。使用您連接到 Slack 的同一帳戶登入 [claude.ai/code](https://claude.ai/code) 一次。首次造訪會建立您的預設雲端環境，錯誤會在您下次提及時清除。每位使用者必須個別執行此操作。

<h3 id="sessions-not-starting">
  工作階段未啟動
</h3>

1. 驗證您的 Claude 帳戶已在 Claude 應用程式首頁中連接
2. 檢查您是否已啟用網路上的 Claude Code 存取
3. 確保您至少有一個 GitHub 存儲庫連接到 Claude Code

<h3 id="repository-not-showing">
  存儲庫未顯示
</h3>

1. 在 [claude.ai/code](https://claude.ai/code) 的網路上的 Claude Code 中連接存儲庫
2. 驗證您對該存儲庫的 GitHub 權限
3. 嘗試斷開並重新連接您的 GitHub 帳戶

<h3 id="wrong-repository-selected">
  選擇了錯誤的存儲庫
</h3>

1. 點擊'Change Repo'按鈕以選擇不同的存儲庫
2. 在您的請求中包括存儲庫名稱以獲得更準確的選擇

<h3 id="authentication-errors">
  驗證錯誤
</h3>

1. 在應用程式首頁中斷開並重新連接您的 Claude 帳戶
2. 確保您在瀏覽器中登入正確的 Claude 帳戶
3. 檢查您的 Claude 計畫是否包括 Claude Code 存取

<h3 id="session-expiration">
  工作階段過期
</h3>

1. 工作階段在網路上的 Claude Code 歷史記錄中保持可存取
2. 您可以從 [claude.ai/code](https://claude.ai/code) 繼續或參考過去的工作階段

<h2 id="current-limitations">
  目前限制
</h2>

* **僅 GitHub**：目前支持 GitHub 上的存儲庫。
* **一次一個拉取請求**：每個工作階段可以建立一個拉取請求。
* **速率限制適用**：工作階段使用您的個人 Claude 計畫的速率限制。
* **需要網路存取**：用戶必須具有網路上的 Claude Code 存取；沒有它的用戶將只獲得標準 Claude 聊天回應。

<h2 id="related-resources">
  相關資源
</h2>

<CardGroup>
  <Card title="網路上的 Claude Code" icon="globe" href="/docs/zh-TW/claude-code-on-the-web">
    了解更多關於網路上的 Claude Code
  </Card>

  <Card title="Claude for Slack" icon="slack" href="https://claude.com/claude-and-slack">
    Claude for Slack 一般文檔
  </Card>

  <Card title="Claude Tag" icon="users" href="https://claude.com/docs/claude-tag/overview">
    組織管理的 Slack 中的 @Claude，具有管理員配置的存取權限
  </Card>

  <Card title="Slack 應用程式市場" icon="store" href="https://slack.com/marketplace/A08SF47R6P4">
    從 Slack 市場安裝 Claude 應用程式
  </Card>

  <Card title="Claude 幫助中心" icon="circle-question" href="https://support.claude.com">
    獲取額外支援
  </Card>
</CardGroup>
