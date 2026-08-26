> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 使用 Remote Control 從任何裝置繼續本地會話

> 使用 Remote Control 從您的手機、平板電腦或任何瀏覽器繼續本地 Claude Code 會話。適用於 claude.ai/code 和 Claude 行動應用程式。

<Note>
  Remote Control 處於研究預覽階段，在所有方案上都可用。在 Team 和 Enterprise 上，預設為關閉，直到擁有者在 [Claude Code 管理員設定](https://claude.ai/admin-settings/claude-code)中啟用 Remote Control 切換。
</Note>

Remote Control 將 [claude.ai/code](https://claude.ai/code) 或 Claude 應用程式（[iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) 和 [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude)）連接到在您機器上執行的 Claude Code 會話。在您的辦公桌開始一項任務，然後從沙發上的手機或另一台電腦上的瀏覽器繼續。

當您在機器上啟動 Remote Control 會話時，Claude 會在整個過程中在本地執行，因此您的程式碼執行和檔案系統存取保持在您的機器上。使用 Remote Control，您可以：

* **遠端使用您的完整本地環境**：您的檔案系統、[MCP servers](/docs/zh-TW/mcp)、工具和專案配置都保持可用，輸入 `@` 會自動完成來自您本地專案的檔案路徑
* **同時在兩個介面上工作**：對話和 [subagents](/docs/zh-TW/sub-agents) 和 [dynamic workflows](/docs/zh-TW/workflows) 的進度在所有連接的裝置上保持同步，因此您可以從終端機、瀏覽器和手機交替發送訊息。在 v2.1.207 之前，由 [Desktop app](/docs/zh-TW/desktop) 託管的會話不會將 subagent 或工作流程進度發送到連接的裝置。
* **從您的手機或瀏覽器傳送影像和檔案**：當您在 Claude 應用程式或 claude.ai/code 中新增附件時，Claude Code 會將其下載到您的機器，並將其作為 `@` 檔案參考傳遞給 Claude，可以有或沒有標題。在 v2.1.202 之前，Claude Code 可能會在沒有標題的附件到達會話之前將其丟棄。
* **克服中斷**：如果您的筆記型電腦進入睡眠狀態或網路中斷，當您的機器重新上線時，會話會自動重新連接。Claude Code 會在連接重建時將來自 subagents 和工作流程的狀態更新排隊，並在恢復後傳遞它們。在 v2.1.207 之前，在重新連接或認證重新整理期間發送的更新可能會遺失，因此連接的裝置會繼續將已完成的任務顯示為執行中。

與[網頁版 Claude Code](/docs/zh-TW/claude-code-on-the-web)（在雲端基礎設施上執行）不同，Remote Control 會話直接在您的機器上執行並與您的本地檔案系統互動。網頁和行動介面只是該本地會話的一個窗口。

本頁涵蓋設定、如何啟動和連接到會話，以及 Remote Control 與網頁版 Claude Code 的比較。

<h2 id="requirements">
  需求
</h2>

在使用 Remote Control 之前，請確認您的環境符合以下條件：

* **訂閱**：在 Pro、Max、Team 和 Enterprise 方案上可用。不支援 API 金鑰。在 Team 和 Enterprise 上，管理員必須先在 [Claude Code 管理員設定](https://claude.ai/admin-settings/claude-code)中啟用 Remote Control 切換。
* **驗證**：執行 `claude` 並使用 `/login` 透過 claude.ai 登入（如果您還沒有登入）。
* **API 端點**：在 Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 上不可用。自 v2.1.196 起，當 [`ANTHROPIC_BASE_URL`](/docs/zh-TW/env-vars) 指向 `api.anthropic.com` 以外的主機（例如 [LLM 閘道](/docs/zh-TW/llm-gateway)或代理）時，Remote Control 也會被停用。取消設定該變數以使用 Remote Control。
* **工作區信任**：在您的專案目錄中至少執行一次 `claude` 以接受工作區信任對話框。

<h2 id="start-a-remote-control-session">
  啟動 Remote Control 會話
</h2>

您可以從 CLI 或 VS Code 擴充功能啟動 Remote Control 會話。CLI 提供三種調用模式；VS Code 使用 `/remote-control` 命令。

<Tabs>
  <Tab title="伺服器模式">
    導航到您的專案目錄並執行：

    ```bash theme={null}
    claude remote-control
    ```

    該程序在您的終端機中以伺服器模式保持執行，等待遠端連接。它顯示一個會話 URL，您可以使用該 URL 從[另一個裝置連接](#connect-from-another-device)，您可以按空格鍵顯示 QR 碼以從手機快速存取。當遠端會話處於活動狀態時，終端機會顯示連接狀態和工具活動。

    可用的旗標：

    | 旗標                                              | 說明                                                                                                                                                                                                                                            |
    | ----------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | `--name "My Project"`                           | 設定自訂會話標題，在 claude.ai/code 的會話清單中可見。                                                                                                                                                                                                           |
    | `--remote-control-session-name-prefix <prefix>` | 未設定明確名稱時自動生成會話名稱的前綴。預設為您機器的主機名稱，產生類似 `myhost-graceful-unicorn` 的名稱。設定 `CLAUDE_REMOTE_CONTROL_SESSION_NAME_PREFIX` 以獲得相同效果。                                                                                                                    |
    | `-c`, `--continue`                              | 恢復從此目錄啟動的最近 Remote Control 會話，而不是建立新會話。無法與 `--session-id`、`--spawn`、`--capacity` 或 `--create-session-in-dir` 結合。需要 Claude Code v2.1.200 或更新版本；較早版本會將該旗標拒絕為未知引數。                                                                               |
    | `--session-id <id>`                             | 按其 ID 恢復特定的 Remote Control 會話。無法與 `--continue`、`--spawn`、`--capacity` 或 `--create-session-in-dir` 結合。需要 Claude Code v2.1.200 或更新版本；較早版本會將該旗標拒絕為未知引數。                                                                                          |
    | `--spawn <mode>`                                | 伺服器如何建立會話。<br />• `same-dir`（預設）：所有會話共享目前的工作目錄，因此如果編輯相同的檔案可能會衝突。<br />• `worktree`：每個按需會話都會獲得自己的 [git worktree](/docs/zh-TW/worktrees)。需要 git 儲存庫。<br />• `session`：單一會話模式。恰好提供一個會話並拒絕其他連接。僅在啟動時設定。<br />在執行時按 `w` 在 `same-dir` 和 `worktree` 之間切換。 |
    | `--capacity <N>`                                | 並行會話的最大數量。預設值為 32。不能與 `--spawn=session` 一起使用。                                                                                                                                                                                                 |
    | `--[no-]create-session-in-dir`                  | 伺服器啟動時在目前目錄中預先建立一個會話，以便您有地方立即輸入。在 `worktree` 模式中，此會話保留在目前目錄中，而按需會話會獲得隔離的 worktrees。預設為開啟；傳遞 `--no-create-session-in-dir` 以不建立任何會話啟動。                                                                                                          |
    | `--verbose`                                     | 顯示詳細的連接和會話日誌。                                                                                                                                                                                                                                 |
    | `--sandbox` / `--no-sandbox`                    | 啟用或停用[沙箱](/docs/zh-TW/sandboxing)以進行檔案系統和網路隔離。預設為關閉。                                                                                                                                                                                               |
  </Tab>

  <Tab title="互動式會話">
    要啟動啟用了 Remote Control 的一般互動式 Claude Code 會話，請使用 `--remote-control` 旗標（或 `--rc`）：

    ```bash theme={null}
    claude --remote-control
    ```

    可選地為會話傳遞一個名稱：

    ```bash theme={null}
    claude --remote-control "My Project"
    ```

    這為您提供了一個完整的互動式會話在您的終端機中，您也可以從 claude.ai 或 Claude 應用程式控制。與 `claude remote-control`（伺服器模式）不同，您可以在會話也可遠端使用時在本地輸入訊息。
  </Tab>

  <Tab title="從現有會話">
    如果您已經在 Claude Code 會話中並想遠端繼續它，請使用 `/remote-control`（或 `/rc`）命令：

    ```text theme={null}
    /remote-control
    ```

    傳遞一個名稱作為引數以設定自訂會話標題：

    ```text theme={null}
    /remote-control My Project
    ```

    這啟動一個 Remote Control 會話，該會話會延續您目前的對話歷史記錄。

    此命令不提供 `--verbose`、`--sandbox` 和 `--no-sandbox` 旗標。
  </Tab>

  <Tab title="VS Code">
    在 [Claude Code VS Code 擴充功能](/docs/zh-TW/vs-code)中，在提示框中輸入 `/remote-control` 或 `/rc`，或使用 `/` 開啟命令選單並選擇它。

    ```text theme={null}
    /remote-control
    ```

    提示框上方會出現一個橫幅，顯示連接狀態。連接後，點擊橫幅中的**在瀏覽器中開啟**直接進入會話，或在 [claude.ai/code](https://claude.ai/code) 的會話清單中找到它。會話 URL 也會發佈在對話中。

    要斷開連接，點擊橫幅上的關閉圖示或再次執行 `/remote-control`。

    與 CLI 不同，VS Code 命令不接受名稱引數或顯示 QR 碼。會話標題是從您的對話歷史記錄或第一個提示衍生的。
  </Tab>
</Tabs>

<h3 id="check-connection-status">
  檢查連接狀態
</h3>

在互動式終端機會話中，當連接啟動時，`/rc active` 指示器會位於輸入框下方的頁尾中，如果終端機太窄無法容納則會隱藏。指示器文字是 claude.ai 上會話的連結。使用向下箭頭鍵選擇它並按 Enter 鍵，或再次執行 `/remote-control`，以開啟狀態面板，其中包含會話 URL 和 QR 碼，您可以使用它從[另一個裝置連接](#connect-from-another-device)。

如果連接失敗，會出現一個通知，顯示失敗原因，指示器會從頁尾消失。再次執行 `/remote-control` 以重試。

<h3 id="connect-from-another-device">
  從另一個裝置連接
</h3>

一旦 Remote Control 會話處於活動狀態，您有幾種方式從另一個裝置連接：

* **開啟會話 URL** 在任何瀏覽器中直接進入 [claude.ai/code](https://claude.ai/code) 上的會話。
* **掃描 QR 碼** 顯示在會話 URL 旁邊，直接在 Claude 應用程式中開啟它。使用 `claude remote-control` 時，按空格鍵切換 QR 碼顯示。
* **開啟 [claude.ai/code](https://claude.ai/code) 或 Claude 應用程式**，並在會話清單中按名稱找到會話。在 Claude 行動應用程式中，點擊導航中的**程式碼**以到達會話清單。Remote Control 會話在線上時顯示帶有綠色狀態點的電腦圖示。

當您連接時，該裝置會顯示會話已在背景執行的任何子代理和工作流程。在 v2.1.208 之前，連接到在互動式終端機中託管的會話的裝置在其中一個子代理或工作流程啟動或停止之前，不會顯示已在執行的子代理和工作流程。

遠端會話標題按以下順序選擇：

1. 您傳遞給 `--name`、`--remote-control` 或 `/remote-control` 的名稱
2. 您使用 `/rename` 設定的標題
3. 現有對話歷史記錄中最後一條有意義的訊息
4. 類似 `myhost-graceful-unicorn` 的自動生成名稱，其中 `myhost` 是您機器的主機名稱或您使用 `--remote-control-session-name-prefix` 設定的前綴

如果您沒有設定明確名稱，標題會在您發送提示後更新以反映您的提示。自 Claude Code v2.1.176 起，自動生成的標題會符合您對話的語言，或設定的 [`language`](/docs/zh-TW/settings#available-settings) 設定（如果已配置）。從 claude.ai 或 Claude 應用程式重新命名會話也會更新在 `claude --resume` 中顯示的本地標題。

如果環境已經有一個活動會話，您將被詢問是否繼續它或啟動一個新會話。

如果您還沒有 Claude 應用程式，請在 Claude Code 內使用 `/mobile` 命令顯示 [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) 或 [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) 的下載 QR 碼。

<h3 id="enable-remote-control-for-all-sessions">
  為所有會話啟用 Remote Control
</h3>

預設情況下，Remote Control 只在您明確執行 `claude remote-control`、`claude --remote-control` 或 `/remote-control` 時啟動，除非自動連接已開啟。要為每個互動式會話自動啟用它，請在 Claude Code 內執行 `/config` 並將**為所有會話啟用 Remote Control** 設定為 `true`。將其設定為 `false` 以永不自動連接，或保留未設定以遵循您組織的預設值。在桌面應用程式中，您也可以從**設定 → Claude Code → 預設啟用遠端控制**切換此選項。在 [VS Code 擴充功能](/docs/zh-TW/vs-code#use-the-prompt-box)中，相同的切換會在命令選單的設定部分中顯示為**為所有會話啟用 Remote Control**；需要 Claude Code v2.1.203 或更新版本。

啟用此設定後，每個互動式 Claude Code 程序會註冊一個遠端會話。如果您執行多個實例，每個實例都會獲得自己的環境和會話。要從單個程序執行多個並行會話，請改用[伺服器模式](#start-a-remote-control-session)。

<h2 id="connection-and-security">
  連接和安全性
</h2>

您的本地 Claude Code 會話僅發出出站 HTTPS 請求，永遠不會在您的機器上開啟入站連接埠。當您啟動 Remote Control 時，它會向 Anthropic API 註冊並輪詢工作。當您從另一個裝置連接時，伺服器會透過串流連接在網頁或行動用戶端與您的本地會話之間路由訊息。

所有流量都透過 TLS 上的 Anthropic API 傳輸，與任何 Claude Code 會話相同的傳輸安全性。連接使用多個短期認證，每個認證的範圍限定為單一目的並獨立過期。

Remote Control 連接時，會話記錄（包括您的訊息、Claude 的回應和工具活動）會儲存在 Anthropic 伺服器上。儲存的記錄可讓對話在您的裝置間保持同步，並讓會話在網路中斷後重新連接。執行和檔案系統存取保留在您的機器上，儲存的記錄會根據[資料使用](/docs/zh-TW/data-usage)政策保留。

若要完全關閉 Remote Control，請使用 [`disableRemoteControl`](/docs/zh-TW/settings#available-settings) 設定。具有零資料保留等合規要求的組織無法啟用 Remote Control。

<h2 id="trusted-devices">
  受信任的裝置
</h2>

<Note>
  受信任的裝置目前處於測試版。功能和功能可能會隨著體驗的改進而演變。

  受信任的裝置在 Team 和 Enterprise 方案上可用。預設為關閉，直到管理員啟用它。
</Note>

受信任的裝置是一個組織範圍的設定，要求成員在從 claude.ai、Claude 行動應用程式或 Claude Desktop 檢視或控制 Remote Control 會話之前驗證其裝置。它將 Remote Control 存取與已知裝置和最近的驗證相關聯，而不僅僅是已登入的帳戶。

當設定開啟時，與 Remote Control 會話互動需要以下兩項：

* **已註冊的裝置**：成員用於 Remote Control 的每個瀏覽器、手機或桌面應用程式都會註冊自己的認證。註冊僅在完整登入後不久提供，因此裝置作為真實驗證的一部分加入受信任清單，而不是在背景中無聲地進行。
* **最近的登入**：成員的登入不超過 18 小時。成員不需要每天登入一次，而是使用 Face ID、Touch ID、Windows Hello 或通行金鑰確認存在。此生物識別步驟立即重新整理會話。

生物識別檢查透過作業系統或瀏覽器在裝置上執行，與通行金鑰登入相同的機制。Anthropic 永遠不會接收或儲存指紋、臉部資料或任何其他生物識別資訊。只有裝置的公鑰和基本中繼資料（例如顯示名稱、平台和註冊時間）會被儲存。

該設定僅適用於 Remote Control。一般 Claude 聊天、終端機中的 Claude Code 和 API 使用不受影響。

<h3 id="enable-trusted-devices-for-your-organization">
  為您的組織啟用受信任的裝置
</h3>

管理員從 Claude Code 管理員主控台啟用該設定。

<Steps>
  <Step title="開啟 Claude Code 管理員設定">
    前往 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)。**需要受信任的裝置**切換會出現在 Remote Control 設定下方。
  </Step>

  <Step title="開啟需要受信任的裝置">
    該設定適用於組織的每個成員以及在您啟用它後啟動的 Remote Control 會話。在切換開啟之前已經執行的會話不會被追溯保護，並在沒有裝置要求的情況下繼續，直到它們結束。不提供按團隊或按專案的範圍設定。
  </Step>

  <Step title="告知成員預期的情況">
    在啟用該設定後，成員第一次從瀏覽器、手機或桌面應用程式檢視或控制新的 Remote Control 會話時，系統會提示他們註冊該裝置。提前讓他們知道可以避免混淆。
  </Step>
</Steps>

<h3 id="what-members-see">
  成員看到的內容
</h3>

註冊是每個裝置的一次性步驟。之後，唯一可見的變化是偶爾的生物識別提示。

* **首次在每個裝置上使用**：成員被要求註冊。如果他們的登入不是最近的，他們首先透過您的一般流程登入，包括 SSO（如果已配置），然後確認註冊。
* **日常使用**：具有已註冊裝置和最近登入的成員看不到任何提示。當登入超過 18 小時時，下一次 Remote Control 互動會顯示單個 Face ID、Touch ID、Windows Hello 或通行金鑰提示。
* **未註冊的裝置**：在裝置被註冊之前，無法檢視或控制 Remote Control 會話。該裝置上的一般 Claude 聊天不受影響。
* **沒有平台驗證器**：沒有 Face ID、Touch ID 或 Windows Hello 的機器上的成員可以使用硬體安全金鑰，或改為再次登入而不是升級。
* **在終端機中**：執行 Claude Code 的機器在開發人員登入 CLI 時會自動接收自己的認證。終端機中沒有單獨的註冊步驟。

<h3 id="manage-enrolled-devices">
  管理已註冊的裝置
</h3>

成員可以從帳戶設定中檢視和撤銷自己的裝置。

開啟 [claude.ai/settings/account](https://claude.ai/settings/account#trusted-devices) 並找到**受信任的裝置**部分，以查看每個已註冊的裝置及其名稱、平台和註冊日期。移除裝置會立即撤銷其認證，裝置可以在新登入後稍後重新註冊。如果未更新，認證也會自行過期，因此未使用的裝置會自動從受信任清單中刪除。

對於遺失或被盜的裝置，成員從此頁面移除它。如果成員無法登入，管理員可以在管理員主控台中使用**到處登出**來撤銷該成員的每個會話和已註冊的裝置，之後成員重新註冊他們仍然持有的裝置。

<h2 id="remote-control-vs-claude-code-on-the-web">
  Remote Control 與網頁版 Claude Code 的比較
</h2>

Remote Control 和[網頁版 Claude Code](/docs/zh-TW/claude-code-on-the-web)都使用 claude.ai/code 介面。關鍵區別在於會話執行的位置：Remote Control 在您的機器上執行，因此您的本地 MCP servers、工具和專案配置保持可用。網頁版 Claude Code 在 Anthropic 管理的雲端基礎設施中執行。

當您在本地工作中途並想從另一個裝置繼續時，請使用 Remote Control。當您想在沒有任何本地設定的情況下啟動任務、處理您沒有複製的儲存庫或並行執行多個任務時，請使用網頁版 Claude Code。

<h2 id="mobile-push-notifications">
  行動推播通知
</h2>

當 Remote Control 處於活動狀態時，Claude 可以向您的手機發送推播通知。

Claude 決定何時推播。它通常在長時間執行的任務完成或需要您的決定以繼續時發送一個。您也可以在提示中請求推播，例如 `notify me when the tests finish`。除了下面的開啟/關閉切換外，沒有按事件配置。

要設定行動推播通知：

<Steps>
  <Step title="安裝 Claude 行動應用程式">
    下載 Claude 應用程式（[iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) 或 [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude)）。
  </Step>

  <Step title="使用您的 Claude Code 帳戶登入">
    使用您在終端機中用於 Claude Code 的相同帳戶和組織。
  </Step>

  <Step title="允許通知">
    接受來自作業系統的通知權限提示。
  </Step>

  <Step title="在 Claude Code 中啟用推播">
    在您的終端機中，執行 `/config` 並啟用**當 Claude 決定時推播**以取得主動通知、**需要操作時推播**以取得權限提示和問題，或兩者。
  </Step>
</Steps>

如果通知未送達：

* 如果 `/config` 顯示**未註冊行動裝置**，請在手機上開啟 Claude 應用程式，以便它可以重新整理其推播令牌。下次 Remote Control 連接時，警告會清除。
* 在 iOS 上，焦點模式和通知摘要可能會抑制或延遲推播。檢查設定 → 通知 → Claude。
* 在 Android 上，激進的電池優化可能會延遲傳遞。在系統設定中將 Claude 應用程式豁免於電池優化。

Claude Code 在您在連接的終端機中輸入或專注時會跳過行動推播通知。自 v2.1.181 起，您可以將 [`CLAUDE_CLIENT_PRESENCE_FILE`](/docs/zh-TW/env-vars) 設定為標記檔案路徑，以將其擴展到您在機器上的任何時間，即使在另一個視窗中：當檔案存在時，通知會被跳過。配置螢幕鎖定監聽器或類似工具，以在螢幕解鎖時建立檔案，並在螢幕鎖定時刪除檔案。

<h2 id="limitations">
  限制
</h2>

* **每個互動式程序一個遠端會話**：在伺服器模式之外，每個 Claude Code 實例一次支援一個遠端會話。使用[伺服器模式](#start-a-remote-control-session)從單個程序執行多個並行會話。
* **本地程序必須保持執行**：Remote Control 作為本地程序執行。如果您關閉終端機、退出 VS Code 或以其他方式停止 `claude` 程序，會話結束。
* **延長的網路中斷**：如果您的機器處於喚醒狀態但無法在大約 10 分鐘以上的時間內到達網路，會話會逾時並且程序退出。再次執行 `claude remote-control` 以啟動新會話。
* **Ultraplan 斷開 Remote Control**：啟動 [ultraplan](/docs/zh-TW/ultraplan) 會話會斷開任何活動的 Remote Control 會話，因為兩個功能都佔據 claude.ai/code 介面，一次只能連接一個。
* **某些命令僅限本地**：只在終端機介面中執行的命令，例如 `/plugin` 或 `/resume`，無論您是否傳遞引數，都只能從本地 CLI 使用。以下命令可從行動和網頁使用：
  * 文字輸出命令：`/compact`、`/clear`、`/context`、`/usage`、`/exit`、`/usage-credits`（執行文字形式而不是開啟 CLI 內對話框）、`/recap`、`/reload-plugins`
  * `/model`、`/effort`、`/fast`、`/color` 和 `/rename`：將值作為引數傳遞，例如 `/model sonnet` 或 `/effort high`。從行動和網頁，`/model` 和 `/effort` 在終端機選擇器或滑桿的位置接受引數。
  * `/mcp`，自 v2.1.166 起：從行動應用程式，傳回伺服器狀態的文字摘要而不是開啟選擇器。在網頁上，`/mcp` 單獨開啟 [claude.ai 連接器](/docs/zh-TW/mcp#use-mcp-servers-from-claude-ai)的目錄而不是傳回摘要。`reconnect`、`enable` 和 `disable` [子命令](/docs/zh-TW/commands#all-commands)可從兩者使用。與本地 CLI 不同，`/mcp reconnect` 不帶伺服器名稱會重新連接每個已失敗或需要驗證的伺服器。
  * `/config`，自 v2.1.181 起：從行動應用程式，傳遞 `key=value` 以設定設定，或不帶引數執行以列出您可以設定的金鑰。在網頁上，`/config` 改為開啟您設定的 Claude Code 部分，並忽略命令後的文字。

<h2 id="troubleshooting">
  疑難排解
</h2>

<h3 id="remote-control-requires-a-claude-ai-subscription">
  「Remote Control 需要 claude.ai 訂閱」
</h3>

您未使用 claude.ai 帳戶進行驗證。執行 `claude auth login` 並選擇 claude.ai 選項。如果在您的環境中設定了 `ANTHROPIC_API_KEY`，請先取消設定它。

在 v2.1.206 之前，在登出時執行 `/remote-control` 會報告 `Unknown command: /remote-control` 而不是此訊息。

<h3 id="remote-control-requires-a-full-scope-login-token">
  「Remote Control 需要完整範圍登入令牌」
</h3>

您使用來自 `claude setup-token` 或 `CLAUDE_CODE_OAUTH_TOKEN` 環境變數的長期令牌進行驗證。這些令牌僅限於推論，無法建立 Remote Control 會話。執行 `claude auth login` 以改用完整範圍會話令牌進行驗證。

<h3 id="unable-to-determine-your-organization-for-remote-control-eligibility">
  「無法確定您的組織以進行 Remote Control 資格檢查」
</h3>

您的快取帳戶資訊已過期或不完整。執行 `claude auth login` 以重新整理它。

<h3 id="remote-control-is-not-yet-enabled-for-your-account">
  「Remote Control 尚未為您的帳戶啟用」
</h3>

Remote Control 推出尚未到達您的帳戶，或您的快取權利已過期。如果您最近更改了方案，執行 `claude auth logout` 然後 `claude auth login` 以重新整理它們。執行 `claude doctor` 以查看哪個個別資格檢查失敗。環境變數衝突、無法到達的檢查和組織政策各自產生自己的訊息，因此此錯誤表示推出閘門本身。

<h3 id="couldn’t-verify-remote-control-eligibility">
  「無法驗證 Remote Control 資格」
</h3>

Claude Code 無法到達功能旗標服務以檢查您的帳戶是否啟用了 Remote Control，通常是因為您離線或代理阻止了請求。一旦您有網路存取權，請重試，或執行 `claude doctor` 以取得詳細資訊。相關訊息「無法驗證您的組織的 Remote Control 政策」具有相同的原因和相同的修復。兩個訊息都在 v2.1.178 中新增。

<h3 id="remote-control-is-only-available-when-using-claude-via-api-anthropic-com">
  「Remote Control 僅在透過 api.anthropic.com 使用 Claude 時可用」
</h3>

會話未直接與 Anthropic API 通訊，因此沒有 claude.ai 後端可配對。這發生在 Amazon Bedrock、Google Cloud 的 Agent Platform 和 Microsoft Foundry 上。自 v2.1.196 起，當 [`ANTHROPIC_BASE_URL`](/docs/zh-TW/env-vars) 指向 `api.anthropic.com` 以外的主機時，例如 [LLM 閘道](/docs/zh-TW/llm-gateway)或代理，即使您使用 claude.ai 登入，也會發生這種情況。取消設定 `ANTHROPIC_BASE_URL` 並重新啟動會話以使用 Remote Control。

<h3 id="remote-control-is-disabled-by-your-organization’s-policy">
  「Remote Control 已被您的組織政策停用」
</h3>

此錯誤有四個不同的原因。首先執行 `/status` 以查看您使用的登入方法和訂閱。

* **您使用 API 金鑰或 Console 帳戶進行驗證**：Remote Control 需要 claude.ai OAuth。執行 `/login` 並選擇 claude.ai 選項。如果在您的環境中設定了 `ANTHROPIC_API_KEY`，請取消設定它。
* **您的組織管理員尚未啟用它**：Remote Control 在 Team 和 Enterprise 方案上預設為關閉。管理員可以在 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) 透過開啟 **Remote Control** 切換來啟用它。此切換是伺服器端組織設定。
* **管理員切換呈灰色**：您的組織具有與 Remote Control 不相容的資料保留或合規配置。這無法從管理面板更改。請聯絡 Anthropic 支援以討論選項。
* **錯誤提及 `disableRemoteControl`**：您的 IT 管理員已透過[受管設定](/docs/zh-TW/settings#settings-files)在此裝置上停用 Remote Control，獨立於組織範圍的切換。

<h3 id="remote-credentials-fetch-failed">
  「Remote credentials fetch failed」
</h3>

Claude Code 無法從 Anthropic API 獲取短期認證以建立連接。使用 `--verbose` 重新執行以查看完整錯誤：

```bash theme={null}
claude remote-control --verbose
```

常見原因：

* 未登入：執行 `claude` 並使用 `/login` 透過您的 claude.ai 帳戶進行驗證。Remote Control 不支援 API 金鑰驗證。
* 網路或代理問題：防火牆或代理可能阻止出站 HTTPS 請求。Remote Control 需要存取埠 443 上的 Anthropic API。
* 會話建立失敗：如果您也看到 `Session creation failed — see debug log`，失敗發生在設定的早期。檢查您的訂閱是否有效。

<h3 id="couldn’t-reconnect-to-your-remote-control-session">
  「無法重新連接到您的 Remote Control 會話」
</h3>

當您使用 `claude --resume` 或 `claude --continue` 繼續對話時，Claude Code 會重新連接到該對話中記錄的 Remote Control 會話。此訊息表示重新連接因可能是暫時性的原因（例如網路中斷或伺服器錯誤）而失敗，因此 Claude Code 無法確認遠端會話是否仍然存在。當伺服器確認先前的會話不再存在時，Claude Code 會建立新的 Remote Control 會話，而不顯示此訊息。

您的本機會話在沒有 Remote Control 的情況下繼續執行。執行 `/remote-control` 以重試連接，或在不使用 `--resume` 的情況下啟動 Claude Code 以建立新的 Remote Control 會話。

在 v2.1.200 之前，重新連接失敗會建立新的 Remote Control 會話，而不是顯示此訊息，這在 claude.ai/code 的會話清單中留下了額外的會話。

<h3 id="your-organization-requires-trusted-devices-for-remote-control-but-this-device-is-not-enrolled">
  「您的組織需要受信任的裝置進行 Remote Control，但此裝置未註冊」
</h3>

您的組織已[啟用受信任的裝置](#trusted-devices)，此機器尚未註冊。在 Claude Code 中執行 `/login`。註冊作為登入的一部分進行，沒有單獨的註冊命令。

<h3 id="session-expired-for-trusted-device-check">
  「session expired for trusted-device check」
</h3>

您的登入超過 18 小時。在 Claude Code 中執行 `/login`，或當 claude.ai 或行動應用程式提示您時，使用 Face ID、Touch ID、Windows Hello 或通行金鑰確認。請參閱[受信任的裝置](#trusted-devices)。

<h2 id="choose-the-right-approach">
  選擇正確的方法
</h2>

Claude Code offers several ways to work when you're not at your terminal. They differ in what triggers the work, where Claude runs, and how much you need to set up.

|                                                          | Trigger                                                                                        | Claude runs on                                                                               | Setup                                                                                                                                | Best for                                                      |
| :------------------------------------------------------- | :--------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------ |
| [Dispatch](/docs/en/desktop#sessions-from-dispatch)           | Message a task from the Claude mobile app                                                      | Your machine (Desktop)                                                                       | [Pair the mobile app with Desktop](https://support.claude.com/en/articles/13947068)                                                  | Delegating work while you're away, minimal setup              |
| [Remote Control](/docs/en/remote-control)                     | Drive a running session from [claude.ai/code](https://claude.ai/code) or the Claude mobile app | Your machine (CLI or VS Code)                                                                | Run `claude remote-control`                                                                                                          | Steering in-progress work from another device                 |
| [Channels](/docs/en/channels)                                 | Push events from a chat app like Telegram or Discord, or your own server                       | Your machine (CLI)                                                                           | [Install a channel plugin](/docs/en/channels#quickstart) or [build your own](/docs/en/channels-reference)                                      | Reacting to external events like CI failures or chat messages |
| [Slack](/docs/en/slack)                                       | Mention `@Claude` in a team channel                                                            | Anthropic cloud                                                                              | [Install the Slack app](/docs/en/slack#setting-up-claude-code-in-slack) with [Claude Code on the web](/docs/en/claude-code-on-the-web) enabled | PRs and reviews from team chat                                |
| [Self-hosted environments](/docs/en/self-hosted-environments) | Start a [cloud session](/docs/en/claude-code-on-the-web) and pick your organization's environment   | Your organization's infrastructure                                                           | [Deploy runners](/docs/en/self-hosted-environments-quickstart), on Team and Enterprise plans                                              | Cloud sessions that must run inside your network              |
| [Scheduled tasks](/docs/en/scheduled-tasks)                   | Set a schedule                                                                                 | [CLI](/docs/en/scheduled-tasks), [Desktop](/docs/en/desktop-scheduled-tasks), or [cloud](/docs/en/routines) | Pick a frequency                                                                                                                     | Recurring automation like daily reviews                       |

<h2 id="related-resources">
  相關資源
</h2>

* [網頁版 Claude Code](/docs/zh-TW/claude-code-on-the-web)：在 Anthropic 管理的雲端環境中執行會話，而不是在您的機器上
* [Ultraplan](/docs/zh-TW/ultraplan)：從您的終端機啟動雲端規劃會話，並在瀏覽器中檢查計畫
* [Channels](/docs/zh-TW/channels)：將 Telegram、Discord 或 iMessage 轉發到會話中，以便 Claude 在您離開時對訊息做出反應
* [Dispatch](/docs/zh-TW/desktop#sessions-from-dispatch)：從您的手機傳送任務訊息，它可以生成 Desktop 會話來處理它
* [驗證](/docs/zh-TW/authentication)：設定 `/login` 並管理 claude.ai 的認證
* [CLI 參考](/docs/zh-TW/cli-reference)：包括 `claude remote-control` 的旗標和命令的完整清單
* [安全性](/docs/zh-TW/security)：Remote Control 會話如何適應 Claude Code 安全模型
* [資料使用](/docs/zh-TW/data-usage)：在本地和遠端會話期間透過 Anthropic API 流動的資料
