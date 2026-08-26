> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 在網頁上使用 Claude Code

> 配置雲端環境、設定指令碼、網路存取和 Docker 在 Anthropic 的沙箱中。使用 `--cloud` 和 `--teleport` 在網頁和終端之間移動工作階段。

<Note>
  Claude Code 網頁版目前處於研究預覽階段，適用於 Pro、Max 和 Team 使用者，以及具有高級席位或 Chat + Claude Code 席位的 Enterprise 使用者。
</Note>

Claude Code 網頁版在 [claude.ai/code](https://claude.ai/code) 上的 Anthropic 管理的雲端基礎設施上執行任務。工作階段即使在您關閉瀏覽器後仍會保留，您可以從 Claude 行動應用程式監控它們。

<Tip>
  初次使用 Claude Code 網頁版？從[開始使用](/docs/zh-TW/web-quickstart)開始，連接您的 GitHub 帳戶並提交您的第一個任務。
</Tip>

本頁涵蓋：

* [GitHub 驗證選項](#github-authentication-options)：連接 GitHub 的兩種方式
* [雲端環境](#the-cloud-environment)：哪些配置會保留、安裝了哪些工具以及如何配置環境
* [設定指令碼](#setup-scripts)和依賴管理
* [網路存取](#network-access)：級別、代理和預設允許清單
* [在網頁和終端之間移動任務](#move-tasks-between-web-and-terminal)，使用 `--cloud` 和 `--teleport`
* [使用工作階段](#work-with-sessions)：檢查、共享、封存、刪除
* [自動修復拉取請求](#auto-fix-pull-requests)：自動回應 CI 失敗和審查評論
* [安全性和隔離](#security-and-isolation)：工作階段如何隔離
* [限制](#limitations)：速率限制和平台限制

<h2 id="github-authentication-options">
  GitHub 驗證選項
</h2>

雲端工作階段需要存取您的 GitHub 儲存庫以複製程式碼和推送分支。您可以通過兩種方式授予存取權限：

| 方法               | 運作方式                                                     | 最適合                                        |
| :--------------- | :------------------------------------------------------- | :----------------------------------------- |
| **GitHub App**   | 在[網頁上線](/docs/zh-TW/web-quickstart)期間授權 Claude GitHub App。    | 瀏覽器上線；想要[自動修復](#auto-fix-pull-requests)的團隊 |
| **`/web-setup`** | 在您的終端中執行 `/web-setup` 以將您的本機 `gh` CLI 令牌同步到您的 Claude 帳戶。 | 已經使用 `gh` 的個人開發者                           |

<Note>
  使用任一方法，雲端工作階段都可以存取連接的 GitHub 帳戶可以看到的任何儲存庫，而不僅僅是安裝了 Claude GitHub App 的儲存庫。App 安裝啟用 PR webhooks 以進行[自動修復](#auto-fix-pull-requests)；它不是工作階段級別的存取控制。若要限制您的團隊可以從雲端工作階段存取的儲存庫，請在 GitHub 本身上限制存取，例如通過限制連接的 GitHub 帳戶的團隊或儲存庫成員資格。
</Note>

任一方法都可以。[`/schedule`](/docs/zh-TW/routines)檢查任一形式的存取，如果都未配置，會提示您執行 `/web-setup`。有關 `/web-setup` 的逐步說明，請參閱[從您的終端連接](/docs/zh-TW/web-quickstart#connect-from-your-terminal)。

GitHub App 是[自動修復](#auto-fix-pull-requests)所必需的，它使用該應用程式接收 PR webhooks。如果您使用 `/web-setup` 連接，稍後想要自動修復，請在這些儲存庫上安裝該應用程式。

Team 和 Enterprise 管理員可以在 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) 使用快速網頁設定切換來禁用 `/web-setup`。

<Note>
  啟用[零資料保留](/docs/zh-TW/zero-data-retention)的組織無法使用 `/web-setup` 或其他雲端工作階段功能。
</Note>

<h2 id="the-cloud-environment">
  雲端環境
</h2>

每個工作階段在一個新的 Anthropic 管理的 VM 中執行，其中您的儲存庫已複製。本節涵蓋工作階段啟動時可用的內容以及如何自訂它。

<h3 id="what’s-available-in-cloud-sessions">
  雲端工作階段中可用的內容
</h3>

雲端工作階段從您的儲存庫的新複製開始。任何提交到儲存庫的內容都可用。您只在自己的機器上安裝或配置的任何內容都不可用。您的組織的政策通過[伺服器管理的設定](/docs/zh-TW/server-managed-settings)單獨到達。

|                                                                     | 在雲端工作階段中可用 | 原因                                                                                                                                                                        |
| :------------------------------------------------------------------ | :--------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 您的儲存庫的 `CLAUDE.md`                                                  | 是          | 複製的一部分                                                                                                                                                                    |
| 您的儲存庫的 `.claude/settings.json` hooks                                | 是          | 複製的一部分                                                                                                                                                                    |
| 您的儲存庫的 `.mcp.json` MCP 伺服器                                          | 是          | 複製的一部分                                                                                                                                                                    |
| 您的儲存庫的 `.claude/rules/`                                             | 是          | 複製的一部分                                                                                                                                                                    |
| 您的儲存庫的 `.claude/skills/`、`.claude/agents/`、`.claude/commands/`      | 是          | 複製的一部分                                                                                                                                                                    |
| 在 `.claude/settings.json` 中聲明的 Plugins                              | 是          | 在工作階段啟動時從您聲明的[市場](/docs/zh-TW/plugin-marketplaces)安裝。需要網路存取才能到達市場來源                                                                                                            |
| 您的組織的[伺服器管理的設定](/docs/zh-TW/server-managed-settings)                     | 是          | 在工作階段啟動時從 Anthropic 的伺服器取得。請參閱[表面涵蓋範圍](/docs/zh-TW/model-config#surface-coverage)以了解 `availableModels` 如何在雲端工作階段中強制執行。通過 MDM 或管理設定檔案部署到您的裝置的設定不適用，因為工作階段在 Anthropic 管理的 VM 上執行 |
| 您的使用者 `~/.claude/CLAUDE.md`                                         | 否          | 位於您的機器上，不在儲存庫中                                                                                                                                                            |
| 您的使用者 `~/.claude/skills/`、`~/.claude/agents/`、`~/.claude/commands/` | 否          | 位於您的機器上，不在儲存庫中。改為將它們提交到儲存庫的 `.claude/` 目錄。您在 claude.ai 上啟用的技能會自動載入到雲端工作階段中                                                                                                |
| 僅在您的使用者設定中啟用的 Plugins                                               | 否          | 使用者範圍的 `enabledPlugins` 位於 `~/.claude/settings.json`。改為在儲存庫的 `.claude/settings.json` 中聲明它們                                                                                |
| 您使用 `claude mcp add` 新增的 MCP 伺服器                                    | 否          | 這些寫入您的本機使用者配置，不是儲存庫。改為在[`.mcp.json`](/docs/zh-TW/mcp#project-scope)中聲明伺服器                                                                                                      |
| 靜態 API 令牌和認證                                                        | 否          | 尚不存在專用的秘密存儲。請參閱下文                                                                                                                                                         |
| 互動式驗證，如 AWS SSO                                                     | 否          | 不支援。SSO 需要無法在雲端工作階段中執行的基於瀏覽器的登入                                                                                                                                           |

若要在雲端工作階段中提供您自己的配置，請將其提交到儲存庫；組織政策通過[伺服器管理的設定](/docs/zh-TW/server-managed-settings)單獨到達。

尚不可用專用的秘密存儲。環境變數和設定指令碼都存儲在環境配置中，對任何可以編輯該環境的人可見。如果您需要雲端工作階段中的秘密，請將它們新增為環境變數，並考慮該可見性。

<h3 id="installed-tools">
  已安裝的工具
</h3>

雲端工作階段預先安裝了常見的語言執行時、建置工具和資料庫。下表按類別總結了包含的內容。

| 類別            | 包含                                                                    |
| :------------ | :-------------------------------------------------------------------- |
| **Python**    | Python 3.x，包含 pip、poetry、uv、black、mypy、pytest、ruff                    |
| **Node.js**   | 20、21 和 22（通過 nvm），包含 npm、yarn、pnpm、bun¹、eslint、prettier、chromedriver |
| **Ruby**      | 3.1、3.2、3.3，包含 gem、bundler、rbenv                                      |
| **PHP**       | 8.4，包含 Composer                                                       |
| **Java**      | OpenJDK 21，包含 Maven 和 Gradle                                          |
| **Go**        | 最新穩定版本，包含模組支援                                                         |
| **Rust**      | rustc 和 cargo                                                         |
| **C/C++**     | GCC、Clang、cmake、ninja、conan                                           |
| **Docker**    | docker、dockerd、docker compose                                         |
| **Databases** | PostgreSQL 16、Redis 7.0                                               |
| **Utilities** | git、jq、yq、ripgrep、tmux、vim、nano                                       |

¹ Bun 已安裝，但在套件取得時有已知的[代理相容性問題](#install-dependencies-with-a-sessionstart-hook)。

如需確切版本，請要求 Claude 在雲端工作階段中執行 `check-tools`。此命令僅存在於雲端工作階段中。

<h3 id="work-with-github-issues-and-pull-requests">
  使用 GitHub 問題和拉取請求
</h3>

雲端工作階段包含內建的 GitHub 工具，讓 Claude 可以讀取問題、列出拉取請求、取得差異和發佈評論，無需任何設定。這些工具通過 [GitHub 代理](#github-proxy)進行驗證，使用您在 [GitHub 驗證選項](#github-authentication-options)下配置的任何方法，因此您的令牌永遠不會進入容器。

您可以在[環境設定](#configure-your-environment)中自行設定 `GH_TOKEN` 或 `GITHUB_TOKEN`，或者兩者都不設定，讓 [GitHub 代理](#github-proxy)為您驗證：

* 如果您設定了令牌，它會原封不動地傳遞到容器，因此 `gh` 和您的指令碼直接使用它。
* 如果您都不設定，容器會將兩個變數都設定為佔位符字串 `proxy-injected`，代理會在出站 GitHub 請求上替換您的真實認證。`gh` 無需您自己的令牌即可工作，但直接讀取 `GITHUB_TOKEN` 的指令碼會獲得佔位符，而不是可用的令牌。

若要檢查哪種情況適用於您的工作階段，請要求 Claude 執行 `echo $GH_TOKEN`。

`gh` CLI 未預先安裝。如果您需要內建工具不涵蓋的 `gh` 命令，例如 `gh release` 或 `gh workflow run`，請自行安裝和驗證：

<Steps>
  <Step title="在您的設定指令碼中安裝 gh">
    將 `apt update && apt install -y gh` 新增到您的[設定指令碼](#setup-scripts)。
  </Step>

  <Step title="如果代理未處理驗證，請提供令牌">
    如果 `echo $GH_TOKEN` 列印 `proxy-injected`，[GitHub 代理](#github-proxy)為您驗證 `gh`，此步驟不必要。否則，將 `GH_TOKEN` 環境變數新增到您的[環境設定](#configure-your-environment)，其中包含 GitHub 個人存取令牌。`gh` 會自動讀取 `GH_TOKEN`，因此不需要 `gh auth login` 步驟。
  </Step>
</Steps>

<h3 id="link-output-back-to-the-session">
  將輸出連結回工作階段
</h3>

每個雲端工作階段在 claude.ai 上都有一個成績單 URL，工作階段可以從 `CLAUDE_CODE_REMOTE_SESSION_ID` 環境變數讀取自己的 ID。使用此在 PR 正文、提交訊息、Slack 貼文或生成的報告中放置可追蹤的連結，以便審查者可以開啟產生它們的執行。

自 v2.1.179 起，Claude 在網頁工作階段中建立的提交包括 `Claude-Session: <url>` git 預告片，PR 正文包括工作階段 URL 在其自己的行上。從 v2.1.182 起，設定 [`attribution.sessionUrl`](/docs/zh-TW/settings#attribution-settings) 為 `false` 以省略預告片和 PR 正文連結。

若要在提交或 PR 以外的其他內容中包括工作階段連結，例如 Claude 發佈的 Slack 訊息或它寫入的報告檔案，請讓 Claude 執行以下命令並使用其輸出。該命令將環境變數值中的 `cse_` 前綴轉換為成績單 URL 期望的 `session_` 前綴：

```bash theme={null}
echo "https://claude.ai/code/${CLAUDE_CODE_REMOTE_SESSION_ID/#cse_/session_}"
```

<h3 id="run-tests-start-services-and-add-packages">
  執行測試、啟動服務和新增套件
</h3>

Claude 執行測試作為處理任務的一部分。在您的提示中要求它，例如「修復 `tests/` 中的失敗測試」或「在每次變更後執行 pytest」。測試執行器（如 pytest、jest 和 cargo test）開箱即用，因為它們已預先安裝。

PostgreSQL 和 Redis 已預先安裝，但預設不執行。在工作階段期間要求 Claude 啟動每一個：

```bash theme={null}
service postgresql start
```

```bash theme={null}
service redis-server start
```

Docker 可用於執行容器化服務。要求 Claude 執行 `docker compose up` 以啟動您的專案服務。拉取映像的網路存取遵循您的環境的[存取級別](#access-levels)，[信任預設值](#default-allowed-domains)包括 Docker Hub 和其他常見登錄。

如果您的映像很大或拉取速度很慢，請將 `docker compose pull` 或 `docker compose build` 新增到您的[設定指令碼](#setup-scripts)。拉取的映像會保存在[快取環境](#environment-caching)中，因此每個新工作階段都已在磁碟上有它們。快取僅存儲檔案，不存儲執行中的程序，因此 Claude 仍然在每個工作階段啟動容器。

若要新增未預先安裝的套件，請使用[設定指令碼](#setup-scripts)。指令碼的輸出會被[快取](#environment-caching)，因此您在那裡安裝的套件在每個工作階段開始時都可用，無需每次重新安裝。您也可以要求 Claude 在工作階段期間安裝套件，但這些安裝不會在工作階段之間保留。

<h3 id="resource-limits">
  資源限制
</h3>

雲端工作階段執行時具有可能隨時間變化的近似資源上限：

* 4 vCPU
* 16 GB RAM
* 30 GB 磁碟

需要明顯更多記憶體的任務，例如大型建置工作或記憶體密集型測試，可能會失敗或被終止。對於超出這些限制的工作負載，請使用[遠端控制](/docs/zh-TW/remote-control)在您自己的硬體上執行 Claude Code。

<h3 id="configure-your-environment">
  配置您的環境
</h3>

環境控制[網路存取](#network-access)、環境變數和在工作階段啟動前執行的[設定指令碼](#setup-scripts)。有關不需要任何配置即可使用的內容，請參閱[已安裝的工具](#installed-tools)。您可以從網頁介面或終端管理環境：

| 操作                 | 方式                                                                                |
| :----------------- | :-------------------------------------------------------------------------------- |
| 新增環境               | 選擇目前環境以開啟選擇器，然後選擇**新增環境**。對話框包括名稱、網路存取級別、環境變數和設定指令碼。                              |
| 編輯環境               | 選擇顯示目前環境名稱的雲端圖示以開啟選擇器，將滑鼠懸停在環境上，然後按一下右側出現的設定圖示。                                   |
| 封存環境               | 開啟環境進行編輯並選擇**封存**。封存的環境隱藏在選擇器中，但現有工作階段繼續執行。                                       |
| 為 CLI 雲端工作階段設定預設環境 | 在您的終端中執行 `/remote-env`。如果您有單一環境，此命令顯示您目前的配置。`/remote-env` 僅選擇預設值；從網頁介面新增、編輯和封存環境。 |

環境變數使用 `.env` 格式，每行一個 `KEY=value` 對。不要用引號包裝值，因為引號會存儲為值的一部分。此範例定義三個變數：

```text theme={null}
NODE_ENV=development
LOG_LEVEL=debug
DATABASE_URL=postgres://localhost:5432/myapp
```

<h3 id="organization-shared-environments">
  組織共享環境
</h3>

Team 和 Enterprise 計畫上的擁有者和管理員可以建立與組織的每個成員共享的雲端環境。共享環境在每個成員的環境選擇器中與他們的個人環境一起出現，因此團隊可以標準化一個配置，而不是每個成員重新建立它。

從[管理設定](https://claude.ai/admin-settings)中的**雲端環境**頁面管理共享環境。從那裡您可以：

* 建立、編輯和封存共享環境。每個環境都有與個人環境相同的欄位：名稱、[網路存取級別](#access-levels)、`.env` 格式的[環境變數](#configure-your-environment)和[設定指令碼](#setup-scripts)。
* 為組織設定預設環境。

共享環境中的值到達該環境中每個成員的工作階段。與個人環境一樣，共享環境沒有專用的秘密存儲，因此不要包括秘密。

<h2 id="setup-scripts">
  設定指令碼
</h2>

設定指令碼是一個 Bash 指令碼，在新的雲端工作階段啟動時執行，在 Claude Code 啟動之前。使用設定指令碼來安裝依賴項、配置工具或取得工作階段需要但未預先安裝的任何內容。

指令碼在 Ubuntu 24.04 上以 root 身份執行，因此 `apt install` 和大多數語言套件管理器都可以工作。

若要新增設定指令碼，請開啟環境設定對話框並在**設定指令碼**欄位中輸入您的指令碼。

此範例安裝 `gh` CLI，它未預先安裝：

```bash theme={null}
#!/bin/bash
apt update && apt install -y gh
```

如果指令碼以非零值退出，工作階段將無法啟動。將 `|| true` 附加到非關鍵命令以避免在不穩定的安裝失敗時阻止工作階段。

保持指令碼的總執行時間在大約五分鐘以下，以便[環境快取](#environment-caching)可以建置。使用 `&` 和 `wait` 並行執行獨立安裝。如果單一下載無法在五分鐘限制內完成，請將其移動到在背景啟動它的 [SessionStart hook](#setup-scripts-vs-sessionstart-hooks)。

<Note>
  安裝套件的設定指令碼需要網路存取才能到達登錄。預設**信任**網路存取允許連接到[常見套件登錄](#default-allowed-domains)，包括 npm、PyPI、RubyGems 和 crates.io。如果您的環境使用**無**網路存取，指令碼將無法安裝套件。
</Note>

<h3 id="environment-caching">
  環境快取
</h3>

設定指令碼在您第一次在環境中啟動工作階段時執行。完成後，Anthropic 會快照檔案系統並將該快照重新用作後續工作階段的起點。新工作階段以您的依賴項、工具和 Docker 映像已在磁碟上開始，設定指令碼步驟被跳過。這即使在指令碼安裝大型工具鏈或拉取容器映像時也能保持啟動速度快。

快取捕獲檔案，不捕獲執行中的程序。設定指令碼寫入磁碟的任何內容都會保留。它啟動的服務或容器不會，因此通過要求 Claude 或使用 [SessionStart hook](#setup-scripts-vs-sessionstart-hooks) 按工作階段啟動這些。

當您更改環境的設定指令碼或允許的網路主機時，以及當快取在大約七天後達到其過期時間時，設定指令碼會再次執行以重建快取。恢復現有工作階段永遠不會重新執行設定指令碼。

您不需要自行啟用快取或管理快照。

<h3 id="setup-scripts-vs-sessionstart-hooks">
  設定指令碼與 SessionStart hooks
</h3>

使用設定指令碼來安裝雲端需要但您的筆記型電腦已有的東西，例如語言執行時或 CLI 工具。使用 [SessionStart hook](/docs/zh-TW/hooks#sessionstart) 進行應在任何地方執行的專案設定，雲端和本機，例如 `npm install`。

兩者都在工作階段開始時執行，但它們屬於不同的位置：

|     | 設定指令碼                                               | SessionStart hooks                 |
| --- | --------------------------------------------------- | ---------------------------------- |
| 附加到 | 雲端環境                                                | 您的儲存庫                              |
| 配置在 | 雲端環境 UI                                             | 您的儲存庫中的 `.claude/settings.json`    |
| 執行  | 在 Claude Code 啟動之前，當沒有[快取環境](#environment-caching)時 | 在 Claude Code 啟動之後，在每個工作階段上，包括已恢復的 |
| 範圍  | 僅雲端環境                                               | 本機和雲端                              |

SessionStart hooks 也可以在本機使用者級別 `~/.claude/settings.json` 中定義，但使用者級別設定不會轉移到雲端工作階段。在雲端中，hooks 來自儲存庫和您組織的[伺服器管理設定](/docs/zh-TW/server-managed-settings)。

<h3 id="install-dependencies-with-a-sessionstart-hook">
  使用 SessionStart hook 安裝依賴項
</h3>

若要僅在雲端工作階段中安裝依賴項，請將 SessionStart hook 新增到您的儲存庫的 `.claude/settings.json`：

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "startup|resume",
        "hooks": [
          {
            "type": "command",
            "command": "\"$CLAUDE_PROJECT_DIR\"/scripts/install_pkgs.sh"
          }
        ]
      }
    ]
  }
}
```

在 `scripts/install_pkgs.sh` 建立指令碼並使用 `chmod +x` 使其可執行。`CLAUDE_CODE_REMOTE` 環境變數在雲端工作階段中設定為 `true`，因此您可以使用它來跳過本機執行：

```bash theme={null}
#!/bin/bash

if [ "$CLAUDE_CODE_REMOTE" != "true" ]; then
  exit 0
fi

npm install
pip install -r requirements.txt
exit 0
```

SessionStart hooks 在雲端工作階段中有一些限制：

* **無雲端專用範圍**：hooks 在本機和雲端工作階段中執行。若要跳過本機執行，請檢查上面所示的 `CLAUDE_CODE_REMOTE` 環境變數。
* **需要網路存取**：安裝命令需要到達套件登錄。如果您的環境使用**無**網路存取，這些 hooks 會失敗。[**信任**下的預設允許清單](#default-allowed-domains)涵蓋 npm、PyPI、RubyGems 和 crates.io。
* **代理相容性**：所有出站流量都通過[安全代理](#security-proxy)。某些套件管理器無法與此代理正確配合使用。Bun 是一個已知的例子。
* **新增啟動延遲**：hooks 在每次工作階段啟動或恢復時執行，不像設定指令碼受益於[環境快取](#environment-caching)。通過在重新安裝之前檢查依賴項是否已存在來保持安裝指令碼快速。

若要為後續 Bash 命令保留環境變數，請寫入 `$CLAUDE_ENV_FILE` 處的檔案。有關詳細資訊，請參閱 [SessionStart hooks](/docs/zh-TW/hooks#sessionstart)。

尚不支援使用您自己的 Docker 映像替換基礎映像。使用設定指令碼在[提供的映像](#installed-tools)上安裝您需要的內容，或使用 `docker compose` 將您的映像作為容器與 Claude 一起執行。

<h2 id="network-access">
  網路存取
</h2>

網路存取控制來自雲端環境的出站連接。每個環境指定一個存取級別，您可以使用自訂允許的域擴展它。預設值為**信任**，允許套件登錄和其他[允許清單域](#default-allowed-domains)。

若要更改環境的網路存取，請[開啟它進行編輯](#configure-your-environment)並在對話框中使用**網路存取**選擇器。沒有單獨的環境頁面。雲端圖示出現在您啟動雲端工作階段或配置[例行工作](/docs/zh-TW/routines#environments-and-network-access)的任何地方。

<Note>
  MCP 連接器流量通過 Anthropic 的伺服器路由，因此您在工作階段或例行工作上啟用的連接器無需將其主機新增到**允許的域**即可工作。連接器按工作階段或按例行工作配置；移除您不需要的任何連接器以限制 Claude 可以到達的工具。這依賴於[安全性和隔離](#security-and-isolation)下提到的相同 Anthropic 綁定通道。
</Note>

<h3 id="access-levels">
  存取級別
</h3>

在建立或編輯環境時選擇存取級別：

| 級別     | 出站連接                                                  |
| :----- | :---------------------------------------------------- |
| **無**  | 無出站網路存取                                               |
| **信任** | [允許清單域](#default-allowed-domains)僅：套件登錄、GitHub、雲端 SDK |
| **完全** | 任何域                                                   |
| **自訂** | 您自己的允許清單，可選地包括預設值                                     |

GitHub 操作使用[單獨的代理](#github-proxy)，獨立於此設定。

<h3 id="allow-specific-domains">
  允許特定域
</h3>

若要允許不在信任清單中的域，請在環境的網路存取設定中選擇**自訂**。出現**允許的域**欄位。每行輸入一個域：

```text theme={null}
api.example.com
*.internal.example.com
registry.example.com
```

使用 `*.` 進行萬用字元子域匹配。檢查**也包括常見套件管理器的預設清單**以將[信任域](#default-allowed-domains)與您的自訂項目一起保留，或將其取消選中以僅允許您列出的內容。

允許的域按環境配置。沒有組織級別的允許清單，擁有者可以推送給所有使用者的環境；[伺服器管理的設定](/docs/zh-TW/server-managed-settings)可以限制雲端工作階段，但無法新增允許的域。

<h3 id="github-proxy">
  GitHub 代理
</h3>

為了安全起見，所有 GitHub 操作都通過專用代理服務進行，該服務將您的真實 GitHub 認證保留在沙箱外。代理驗證兩種流量：

* Git 互動：沙箱內的 git 用戶端使用自訂建置的限定認證，代理驗證並將其轉換為您的實際 GitHub 驗證令牌
* GitHub API 請求：代理在來自內建 GitHub 工具的請求上替換您的真實認證，以及來自 `gh` 的請求（當您的工作階段設定[使用 GitHub 問題和提取請求](#work-with-github-issues-and-pull-requests)中描述的 `proxy-injected` 預留位置時）

代理還限制 git push 操作到目前工作分支以確保安全，並啟用複製、取得和 PR 操作，同時維護安全邊界。

代理限制 GitHub API 和發行資產請求到附加到工作階段的儲存庫，無論環境的[存取級別](#access-levels)如何。下載來自未附加儲存庫的發行資產的設定指令碼會傳回 403。來自公開儲存庫的已提交檔案通過 `raw.githubusercontent.com` 取得，[安全代理](#security-proxy)會處理該檔案。該域在預設[信任清單](#default-allowed-domains)中，因此除非環境的[存取級別](#access-levels)排除它，否則檔案保持可達。

<h3 id="security-proxy">
  安全代理
</h3>

環境在 HTTP/HTTPS 網路代理後面執行，用於安全和濫用防止目的。所有出站網際網路流量都通過此代理，該代理提供：

* 防止惡意請求
* 速率限制和濫用防止
* 增強安全性的內容篩選
* 所請求主機名稱的 DNS 級別稽核軌跡

<h3 id="default-allowed-domains">
  預設允許的域
</h3>

使用**信任**網路存取時，預設允許以下域。標記為 `*` 的域表示萬用字元子域匹配，因此 `*.gcr.io` 允許 `gcr.io` 的任何子域。

<AccordionGroup>
  <Accordion title="Anthropic 服務">
    * api.anthropic.com
    * statsig.anthropic.com
    * docs.claude.com
    * platform.claude.com
    * code.claude.com
    * claude.ai
  </Accordion>

  <Accordion title="版本控制">
    * github.com
    * [www.github.com](http://www.github.com)
    * api.github.com
    * npm.pkg.github.com
    * raw\.githubusercontent.com
    * pkg-npm.githubusercontent.com
    * objects.githubusercontent.com
    * release-assets.githubusercontent.com
    * codeload.github.com
    * avatars.githubusercontent.com
    * camo.githubusercontent.com
    * gist.github.com
    * gitlab.com
    * [www.gitlab.com](http://www.gitlab.com)
    * registry.gitlab.com
    * bitbucket.org
    * [www.bitbucket.org](http://www.bitbucket.org)
    * api.bitbucket.org
  </Accordion>

  <Accordion title="容器登錄">
    * registry-1.docker.io
    * auth.docker.io
    * index.docker.io
    * hub.docker.com
    * [www.docker.com](http://www.docker.com)
    * production.cloudflare.docker.com
    * download.docker.com
    * gcr.io
    * \*.gcr.io
    * ghcr.io
    * mcr.microsoft.com
    * \*.data.mcr.microsoft.com
    * public.ecr.aws
  </Accordion>

  <Accordion title="雲端平台">
    * cloud.google.com
    * accounts.google.com
    * gcloud.google.com
    * \*.googleapis.com
    * storage.googleapis.com
    * compute.googleapis.com
    * container.googleapis.com
    * azure.com
    * portal.azure.com
    * microsoft.com
    * [www.microsoft.com](http://www.microsoft.com)
    * \*.microsoftonline.com
    * packages.microsoft.com
    * dotnet.microsoft.com
    * dot.net
    * visualstudio.com
    * dev.azure.com
    * \*.amazonaws.com
    * \*.api.aws
    * oracle.com
    * [www.oracle.com](http://www.oracle.com)
    * java.com
    * [www.java.com](http://www.java.com)
    * java.net
    * [www.java.net](http://www.java.net)
    * download.oracle.com
    * yum.oracle.com
  </Accordion>

  <Accordion title="JavaScript 和 Node 套件管理器">
    * registry.npmjs.org
    * [www.npmjs.com](http://www.npmjs.com)
    * [www.npmjs.org](http://www.npmjs.org)
    * npmjs.com
    * npmjs.org
    * yarnpkg.com
    * registry.yarnpkg.com
  </Accordion>

  <Accordion title="Python 套件管理器">
    * pypi.org
    * [www.pypi.org](http://www.pypi.org)
    * files.pythonhosted.org
    * pythonhosted.org
    * test.pypi.org
    * pypi.python.org
    * pypa.io
    * [www.pypa.io](http://www.pypa.io)
  </Accordion>

  <Accordion title="Ruby 套件管理器">
    * rubygems.org
    * [www.rubygems.org](http://www.rubygems.org)
    * api.rubygems.org
    * index.rubygems.org
    * ruby-lang.org
    * [www.ruby-lang.org](http://www.ruby-lang.org)
    * rubyforge.org
    * [www.rubyforge.org](http://www.rubyforge.org)
    * rubyonrails.org
    * [www.rubyonrails.org](http://www.rubyonrails.org)
    * rvm.io
    * get.rvm.io
  </Accordion>

  <Accordion title="Rust 套件管理器">
    * crates.io
    * [www.crates.io](http://www.crates.io)
    * index.crates.io
    * static.crates.io
    * rustup.rs
    * static.rust-lang.org
    * [www.rust-lang.org](http://www.rust-lang.org)
  </Accordion>

  <Accordion title="Go 套件管理器">
    * proxy.golang.org
    * sum.golang.org
    * index.golang.org
    * golang.org
    * [www.golang.org](http://www.golang.org)
    * goproxy.io
    * pkg.go.dev
  </Accordion>

  <Accordion title="JVM 套件管理器">
    * maven.org
    * repo.maven.org
    * central.maven.org
    * repo1.maven.org
    * repo.maven.apache.org
    * jcenter.bintray.com
    * gradle.org
    * [www.gradle.org](http://www.gradle.org)
    * services.gradle.org
    * plugins.gradle.org
    * kotlinlang.org
    * [www.kotlinlang.org](http://www.kotlinlang.org)
    * spring.io
    * repo.spring.io
  </Accordion>

  <Accordion title="其他套件管理器">
    * packagist.org (PHP Composer)
    * [www.packagist.org](http://www.packagist.org)
    * repo.packagist.org
    * nuget.org (.NET NuGet)
    * [www.nuget.org](http://www.nuget.org)
    * api.nuget.org
    * pub.dev (Dart/Flutter)
    * api.pub.dev
    * hex.pm (Elixir/Erlang)
    * [www.hex.pm](http://www.hex.pm)
    * cpan.org (Perl CPAN)
    * [www.cpan.org](http://www.cpan.org)
    * metacpan.org
    * [www.metacpan.org](http://www.metacpan.org)
    * api.metacpan.org
    * cocoapods.org (iOS/macOS)
    * [www.cocoapods.org](http://www.cocoapods.org)
    * cdn.cocoapods.org
    * haskell.org
    * [www.haskell.org](http://www.haskell.org)
    * hackage.haskell.org
    * swift.org
    * [www.swift.org](http://www.swift.org)
  </Accordion>

  <Accordion title="Linux 發行版">
    * archive.ubuntu.com
    * security.ubuntu.com
    * ubuntu.com
    * [www.ubuntu.com](http://www.ubuntu.com)
    * \*.ubuntu.com
    * ppa.launchpad.net
    * launchpad.net
    * [www.launchpad.net](http://www.launchpad.net)
    * \*.nixos.org
  </Accordion>

  <Accordion title="開發工具和平台">
    * dl.k8s.io (Kubernetes)
    * pkgs.k8s.io
    * k8s.io
    * [www.k8s.io](http://www.k8s.io)
    * releases.hashicorp.com (HashiCorp)
    * apt.releases.hashicorp.com
    * rpm.releases.hashicorp.com
    * archive.releases.hashicorp.com
    * hashicorp.com
    * [www.hashicorp.com](http://www.hashicorp.com)
    * repo.anaconda.com (Anaconda/Conda)
    * conda.anaconda.org
    * anaconda.org
    * [www.anaconda.com](http://www.anaconda.com)
    * anaconda.com
    * continuum.io
    * apache.org (Apache)
    * [www.apache.org](http://www.apache.org)
    * archive.apache.org
    * downloads.apache.org
    * eclipse.org (Eclipse)
    * [www.eclipse.org](http://www.eclipse.org)
    * download.eclipse.org
    * nodejs.org (Node.js)
    * [www.nodejs.org](http://www.nodejs.org)
    * developer.apple.com
    * developer.android.com
    * pkg.stainless.com
    * binaries.prisma.sh
  </Accordion>

  <Accordion title="雲端服務和監控">
    * statsig.com
    * [www.statsig.com](http://www.statsig.com)
    * api.statsig.com
    * sentry.io
    * \*.sentry.io
    * downloads.sentry-cdn.com
    * http-intake.logs.datadoghq.com
    * browser-intake-us5-datadoghq.com
    * \*.datadoghq.com
    * \*.datadoghq.eu
    * api.honeycomb.io
  </Accordion>

  <Accordion title="內容傳遞和鏡像">
    * sourceforge.net
    * \*.sourceforge.net
    * packagecloud.io
    * \*.packagecloud.io
    * fonts.googleapis.com
    * fonts.gstatic.com
  </Accordion>

  <Accordion title="架構和配置">
    * json-schema.org
    * [www.json-schema.org](http://www.json-schema.org)
    * json.schemastore.org
    * [www.schemastore.org](http://www.schemastore.org)
  </Accordion>

  <Accordion title="Model Context Protocol">
    * \*.modelcontextprotocol.io
  </Accordion>
</AccordionGroup>

<h2 id="move-tasks-between-web-and-terminal">
  在網頁和終端之間移動任務
</h2>

這些工作流程需要[Claude Code CLI](/docs/zh-TW/quickstart)登入到相同的 claude.ai 帳戶。您可以從終端啟動新的雲端工作階段，或將雲端工作階段拉入終端以在本機繼續。雲端工作階段即使在您關閉筆記型電腦後仍會保留，您可以從任何地方（包括 Claude 行動應用程式）監控它們。

<Note>
  從 CLI，工作階段交接是單向的：您可以使用 `--teleport` 將雲端工作階段拉入終端，但無法將現有終端工作階段推送到網頁。`--cloud` 旗標為您目前的儲存庫建立新的雲端工作階段。[Desktop 應用程式](/docs/zh-TW/desktop#continue-in-another-surface)提供可將本機工作階段發送到網頁的「在另一個表面繼續」功能表。
</Note>

<h3 id="from-terminal-to-web">
  從終端到網頁
</h3>

使用 `--cloud` 旗標從命令列啟動雲端工作階段：

```bash theme={null}
claude --cloud "Fix the authentication bug in src/auth/login.ts"
```

這會在 claude.ai 上建立新的雲端工作階段。工作階段複製您目前目錄的 GitHub 遠端，位於您目前的分支，因此如果您有本機提交，請先推送，因為 VM 從 GitHub 而不是您的機器複製。`--cloud` 一次適用於單一儲存庫。任務在雲端執行，而您繼續在本機工作。較舊的 `--remote` 拼寫仍然可作為 `--cloud` 的已棄用別名。

自 v2.1.195 起，CLI 會顯示設定步驟的即時檢查清單，例如複製儲存庫和執行您的[設定指令碼](#setup-scripts)，同時雲端容器啟動。您在容器佈建時輸入的訊息會排隊，並在工作階段準備好後發送。

<Note>
  `--cloud` 建立雲端工作階段。`--remote-control` 無關：它公開本機 CLI 工作階段以從網頁進行監控。請參閱[遠端控制](/docs/zh-TW/remote-control)。
</Note>

在 Claude Code CLI 中使用 `/tasks` 檢查進度，或在 claude.ai 或 Claude 行動應用程式上開啟工作階段以直接互動。從那裡，您可以引導 Claude、提供反饋或回答問題，就像任何其他對話一樣。

<h4 id="tips-for-cloud-tasks">
  雲端任務的提示
</h4>

**在本機規劃，在遠端執行**：對於複雜任務，在規劃模式下啟動 Claude 以協作制定方法，然後將工作發送到雲端：

```bash theme={null}
claude --permission-mode plan
```

在規劃模式下，Claude 讀取檔案、執行命令以探索並提出計畫，而不編輯原始程式碼。一旦您對計畫感到滿意，將計畫保存到儲存庫、提交和推送，以便雲端 VM 可以複製它。然後為自主執行啟動雲端工作階段：

```bash theme={null}
claude --cloud "Execute the migration plan in docs/migration-plan.md"
```

此模式讓您可以控制策略，同時讓 Claude 在雲端自主執行。

**使用 ultraplan 在雲端規劃**：若要在網頁工作階段中起草和檢查計畫本身，請使用 [ultraplan](/docs/zh-TW/ultraplan)。Claude 在 Claude Code 網頁版上生成計畫，同時您繼續工作，然後您在瀏覽器中對部分進行評論並選擇遠端執行或將計畫發送回終端。

**並行執行任務**：每個 `--cloud` 命令建立自己的雲端工作階段，獨立執行。您可以啟動多個任務，它們都會在單獨的工作階段中同時執行：

```bash theme={null}
claude --cloud "Fix the flaky test in auth.spec.ts"
claude --cloud "Update the API documentation"
claude --cloud "Refactor the logger to use structured output"
```

使用 Claude Code CLI 中的 `/tasks` 監控所有工作階段。當工作階段完成時，您可以從網頁介面建立 PR，或[傳送](#from-web-to-terminal)工作階段到終端以繼續工作。

<h4 id="send-local-repositories-without-github">
  發送沒有 GitHub 的本機儲存庫
</h4>

當您從未連接到 GitHub 的儲存庫執行 `claude --cloud` 時，Claude Code 會捆綁您的本機儲存庫並直接上傳到雲端工作階段。捆綁包括您的完整儲存庫歷史記錄，跨所有分支，加上任何未提交的對追蹤檔案的變更。

當 GitHub 存取不可用時，此回退會自動啟動。若要即使在 GitHub 已連接時也強制它，請設定 `CCR_FORCE_BUNDLE=1`：

```bash theme={null}
CCR_FORCE_BUNDLE=1 claude --cloud "Run the test suite and fix any failures"
```

捆綁的儲存庫必須符合這些限制：

* 目錄必須是至少有一個提交的 git 儲存庫
* 捆綁的儲存庫必須在 100 MB 以下。較大的儲存庫回退到僅捆綁目前分支，然後回退到工作樹的單一壓縮快照，並且僅在快照仍然太大時失敗
* 未追蹤的檔案不包括；在您希望雲端工作階段看到的檔案上執行 `git add`
* 從捆綁建立的工作階段無法推送回遠端，除非您也配置了 [GitHub 驗證](#github-authentication-options)

<h3 id="from-web-to-terminal">
  從網頁到終端
</h3>

使用以下任何方式將雲端工作階段拉入終端：

* **使用 `--teleport`**：從命令列，執行 `claude --teleport` 以進行互動式工作階段選擇器，或執行 `claude --teleport <session-id>` 以直接恢復特定工作階段。如果您有未提交的變更，系統會提示您先隱藏它們。
* **使用 `/teleport`**：在現有 CLI 工作階段內，執行 `/teleport` 或 `/tp` 以開啟相同的工作階段選擇器，而無需重新啟動 Claude Code。
* **從 `/tasks`**：執行 `/tasks` 以查看您的背景工作階段，然後按 `t` 傳送到其中一個。
* **從網頁介面**：選擇**在 CLI 中開啟**以複製可貼到終端的命令。

當您傳送工作階段時，Claude 驗證您在正確的儲存庫中，從雲端工作階段取得並簽出分支，並將完整的對話歷史記錄載入到終端。

`--teleport` 與 `--resume` 不同。`--resume` 從此機器的本機歷史記錄重新開啟對話，不列出雲端工作階段；`--teleport` 拉取雲端工作階段及其分支。

<h4 id="teleport-requirements">
  傳送要求
</h4>

傳送在恢復工作階段之前檢查這些要求。如果任何要求未滿足，您會看到錯誤或被提示解決問題。

| 要求         | 詳細資訊                                                                                                                                                                                 |
| ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 乾淨的 git 狀態 | 您的工作目錄必須沒有未提交的變更。如果需要，傳送會提示您隱藏變更。                                                                                                                                                    |
| 正確的儲存庫     | 您必須從同一儲存庫的簽出執行 `--teleport`，而不是分支。自 v2.1.199 起，Claude Code 接受簽出，即使它無法將遠端解析為主機名稱，例如 SSH 主機別名（如 `git@work:owner/repo.git`）或 `insteadOf` 重寫的短形式。它首先顯示確認提示，並且僅當遠端的擁有者和儲存庫名稱與工作階段的儲存庫相符時。 |
| 分支可用       | 雲端工作階段中的分支必須已推送到遠端。傳送會自動取得並簽出它。                                                                                                                                                      |
| 相同帳戶       | 您必須驗證到雲端工作階段中使用的相同 claude.ai 帳戶。                                                                                                                                                     |

<h4 id="teleport-is-unavailable">
  `--teleport` 不可用
</h4>

傳送需要 claude.ai 訂閱驗證。如果您通過 API 金鑰、Amazon Bedrock、Google Cloud 的 Agent Platform 或 Microsoft Foundry 進行驗證，請執行 `/login` 以改為使用您的 claude.ai 帳戶登入。如果您已通過 claude.ai 登入，`--teleport` 仍不可用，您的組織可能已禁用雲端工作階段。

<h2 id="work-with-sessions">
  使用工作階段
</h2>

工作階段出現在 claude.ai/code 的側邊欄中。從那裡，您可以檢查變更、與隊友共享、封存完成的工作或永久刪除工作階段。

<h3 id="manage-context">
  管理上下文
</h3>

雲端工作階段支援產生文字輸出的[內建命令](/docs/zh-TW/commands)。只在終端介面中執行的命令，例如 `/plugin` 或 `/resume`，無法使用。在雲端工作階段中開啟選擇器或面板的命令行為不同：

* **`/model`、`/effort`、`/fast`、`/color` 和 `/rename`**：將值作為引數傳遞，例如 `/model sonnet`，而不是開啟終端選擇器或滑塊。引數形式需要工作階段環境中的 Claude Code v2.1.205 或更新版本，並遵循每個命令的[可用性說明](/docs/zh-TW/commands#all-commands)：當模型的[啟動預設努力保持](/docs/zh-TW/model-config#adjust-effort-level)生效時，`/effort` 會報告 `Not applied`，而 `/fast` 僅在以快速模式啟動的工作階段中有效。
* **`/config`**：在網路上，開啟您設定的 Claude Code 部分，而不是設定值，命令後的文字（包括 `key=value`）會被忽略。若要變更雲端工作階段的設定，請使用[環境變數](#configure-your-environment)或將[設定檔案](/docs/zh-TW/settings)提交到儲存庫。

對於上下文管理特別：

| 命令         | 在雲端工作階段中有效 | 備註                                                     |
| :--------- | :--------- | :----------------------------------------------------- |
| `/compact` | 是          | 總結對話以釋放上下文。接受可選的焦點指示，如 `/compact keep the test output` |
| `/context` | 是          | 顯示目前在上下文視窗中的內容                                         |
| `/clear`   | 否          | 改為從側邊欄啟動新工作階段                                          |

自動壓縮在上下文視窗接近容量時自動執行。若要更早觸發它，請在您的[環境變數](#configure-your-environment)中設定 [`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`](/docs/zh-TW/env-vars)。例如，`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=70` 在 70% 容量而不是等待視窗幾乎滿時壓縮。若要更改壓縮計算的有效視窗大小，請使用 [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/zh-TW/env-vars)。

[Subagents](/docs/zh-TW/sub-agents) 的運作方式與本機相同。Claude 可以使用 Task 工具生成它們，以將研究或並行工作卸載到單獨的上下文視窗中，保持主對話更輕。在您的儲存庫的 `.claude/agents/` 中定義的 Subagents 會自動選擇。

[Agent teams](/docs/zh-TW/agent-teams) 預設關閉，但可以通過將 `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1` 新增到您的[環境變數](#configure-your-environment)來啟用。

<h3 id="review-changes">
  檢查變更
</h3>

每個工作階段顯示一個差異指示器，其中包含新增和移除的行數，例如 `+42 -18`。選擇它以開啟差異檢視，在特定行上留下內聯評論，並使用您的下一條訊息將它們發送給 Claude。有關完整逐步說明（包括 PR 建立），請參閱[檢查和迭代](/docs/zh-TW/web-quickstart#review-and-iterate)。若要讓 Claude 自動監控 PR 以查找 CI 失敗和審查評論，請參閱[自動修復拉取請求](#auto-fix-pull-requests)。

<h3 id="share-sessions">
  共享工作階段
</h3>

若要共享工作階段，請根據下面的帳戶類型切換其可見性。之後，按原樣共享工作階段連結。收件者在開啟連結時看到最新狀態，但他們的檢視不會即時更新。

<h4 id="share-from-an-enterprise-or-team-account">
  從 Enterprise 或 Team 帳戶共享
</h4>

對於 Enterprise 和 Team 帳戶，兩個可見性選項是**私人**和**Team**。Team 可見性使工作階段對您的 claude.ai 組織的其他成員可見。[Slack 中的 Claude](/docs/zh-TW/slack)工作階段會自動以 Team 可見性共享。

儲存庫存取驗證預設啟用，基於連接到收件者帳戶的 GitHub 帳戶。您帳戶的顯示名稱對所有有存取權限的收件者可見。

<h4 id="share-from-a-max-or-pro-account">
  從 Max 或 Pro 帳戶共享
</h4>

對於 Max 和 Pro 帳戶，兩個可見性選項是**私人**和**公開**。公開可見性使工作階段對任何登入 claude.ai 的使用者可見。

在共享之前檢查您的工作階段是否包含敏感內容。工作階段可能包含來自私人 GitHub 儲存庫的程式碼和認證。儲存庫存取驗證預設未啟用。

若要要求收件者具有儲存庫存取權限，或從共享工作階段中隱藏您的名稱，請前往「設定」>「Claude Code」>「共享設定」。

<h3 id="archive-sessions">
  封存工作階段
</h3>

您可以封存工作階段以保持工作階段清單的組織。封存的工作階段隱藏在預設工作階段清單中，但可以通過篩選封存的工作階段來檢視。

若要封存工作階段，請在側邊欄中將滑鼠懸停在工作階段上，然後選擇封存圖示。

<h3 id="delete-sessions">
  刪除工作階段
</h3>

刪除工作階段會永久移除工作階段及其資料。此操作無法撤銷。您可以通過兩種方式刪除工作階段：

* **從側邊欄**：篩選封存的工作階段，然後將滑鼠懸停在您要刪除的工作階段上，並選擇刪除圖示
* **從工作階段功能表**：開啟工作階段，選擇工作階段標題旁的下拉式功能表，然後選擇**刪除**

在刪除工作階段之前，系統會要求您確認。

<h2 id="auto-fix-pull-requests">
  自動修復拉取請求
</h2>

Claude 可以監視拉取請求並自動回應 CI 失敗和審查評論。Claude 訂閱 PR 上的 GitHub 活動，當檢查失敗或審查者留下評論時，Claude 會調查並推送修復（如果有明確的修復）。

<Note>
  自動修復需要在您的儲存庫上安裝 Claude GitHub App。如果您還沒有，請從 [GitHub App 頁面](https://github.com/apps/claude)安裝它，或在[設定](/docs/zh-TW/web-quickstart#connect-github-and-create-an-environment)期間出現提示時安裝。
</Note>

根據 PR 來自何處以及您使用的設備，有幾種方式可以開啟自動修復：

* **在 Claude Code 網頁版中建立的 PR**：開啟 CI 狀態欄並選擇**自動修復**
* **從您的終端**：在 PR 的分支上執行 [`/autofix-pr`](/docs/zh-TW/commands)。Claude Code 使用 `gh` 偵測開啟的 PR，生成網頁工作階段，並在一個步驟中開啟自動修復
* **從行動應用程式**：告訴 Claude 自動修復 PR，例如「監視此 PR 並修復任何 CI 失敗或審查評論」
* **任何現有 PR**：將 PR URL 貼到工作階段中並告訴 Claude 自動修復它

自動修復是每個 PR 的切換開關。若要停止監視，請在網頁工作階段中開啟 CI 狀態欄並清除**自動修復**切換，或告訴 Claude 停止監視 PR。

<h3 id="how-claude-responds-to-pr-activity">
  Claude 如何回應 PR 活動
</h3>

當自動修復處於活動狀態時，Claude 會收到 PR 的 GitHub 事件，包括新的審查評論和 CI 檢查失敗。對於每個事件，Claude 會調查並決定如何進行：

* **明確的修復**：如果 Claude 對修復有信心且不與早期指示衝突，Claude 會進行變更、推送它，並在工作階段中解釋所做的工作
* **模糊的請求**：如果審查者的評論可以以多種方式解釋或涉及架構上重要的內容，Claude 會在採取行動前詢問您
* **重複或無操作事件**：如果事件是重複的或不需要變更，Claude 會在工作階段中記錄它並繼續

GitHub 不會在基礎分支推進並建立合併衝突時發出 webhook，因此自動修復無法自行對衝突做出反應。若要解決衝突，請開啟工作階段並要求 Claude 進行變基。

Claude 可能會在 GitHub 上回覆審查評論執行緒作為解決它們的一部分。這些回覆使用您的 GitHub 帳戶發佈，因此它們會出現在您的使用者名稱下，但每個回覆都標記為來自 Claude Code，以便審查者知道它是由代理編寫的，而不是由您直接編寫的。

<Warning>
  如果您的儲存庫使用評論觸發的自動化，例如 Atlantis、Terraform Cloud 或在 `issue_comment` 事件上執行的自訂 GitHub Actions，請注意 Claude 可以代表您回覆，這可能會觸發這些工作流程。在啟用自動修復之前檢查您的儲存庫自動化，並考慮為 PR 評論可以部署基礎設施或執行特權操作的儲存庫禁用自動修復。
</Warning>

<h2 id="security-and-isolation">
  安全性和隔離
</h2>

每個雲端工作階段通過多個層與您的機器和其他工作階段分離：

* **隔離的虛擬機器**：每個工作階段在隔離的 Anthropic 管理的 VM 中執行
* **網路存取控制**：網路存取預設受限，可以禁用。在禁用網路存取的情況下執行時，Claude Code 仍然可以與 Anthropic API 通訊，這可能允許資料離開 VM。
* **認證保護**：敏感認證（如 git 認證或簽署金鑰）永遠不在沙箱內與 Claude Code 一起。驗證通過使用限定認證的安全代理進行處理。
* **安全分析**：程式碼在隔離的 VM 內進行分析和修改，然後建立 PR

<h2 id="troubleshooting">
  故障排除
</h2>

對於出現在對話中的執行時 API 錯誤，例如 `API Error: 500`、`529 Overloaded`、`429` 或 `Prompt is too long`，請參閱[錯誤參考](/docs/zh-TW/errors)。這些錯誤及其修復與 CLI 和 Desktop 應用程式共享。下面的部分涵蓋特定於雲端工作階段的問題。

<h3 id="session-creation-failed">
  工作階段建立失敗
</h3>

如果新工作階段無法啟動，出現 `Session creation failed` 或在佈建時停滯，Claude Code 無法分配雲端環境。

* 檢查 [status.claude.com](https://status.claude.com) 以查找雲端工作階段事件
* 一分鐘後重試，因為容量是按需佈建的
* 確認您的儲存庫可到達。連接的 GitHub 帳戶必須能夠存取 GitHub 上的儲存庫，可以透過 Claude GitHub App 授權或透過 `/web-setup` 同步的 `gh` 令牌進行存取。不需要在儲存庫上安裝 App。請參閱 [GitHub 驗證選項](#github-authentication-options)。

<h3 id="remote-control-session-expired-or-access-denied">
  遠端控制工作階段已過期或存取被拒絕
</h3>

`--teleport` 通過與雲端工作階段使用的相同遠端控制工作階段基礎設施連接，因此驗證和工作階段過期錯誤會以遠端控制措辭出現。您可能會看到 `Remote Control session expired` 或 `Access denied`。連接令牌是短期的，並限定於您的帳戶。

* 在本機執行 `/login` 以刷新您的認證，然後重新連接
* 確認您登入到擁有工作階段的相同帳戶
* 如果您看到 `Remote Control may not be available for this organization`，組織的擁有者尚未為您的組織啟用雲端工作階段

<h3 id="environment-expired">
  環境已過期
</h3>

雲端工作階段在不活動一段時間後停止，基礎環境被回收。從本機終端，這會顯示為 `Could not resume session ... its environment has expired. Creating a fresh session instead.` 在網頁上，工作階段在工作階段清單中標記為已過期。

從 [claude.ai/code](https://claude.ai/code) 重新開啟工作階段以佈建新環境，並恢復您的對話歷史記錄。

<h2 id="limitations">
  限制
</h2>

在依賴雲端工作階段進行工作流程之前，請考慮這些限制：

* **速率限制**：Claude Code 網頁版與您帳戶內所有其他 Claude 和 Claude Code 使用共享速率限制。並行執行多個任務會按比例消耗更多速率限制。雲端 VM 沒有單獨的計算費用。
* **儲存庫驗證**：您只能在驗證到相同帳戶時將工作階段從網頁移動到本機
* **平台限制**：儲存庫複製和拉取請求建立需要 GitHub。自託管[GitHub Enterprise Server](/docs/zh-TW/github-enterprise-server) 執行個體支援 Team 和 Enterprise 計畫。GitLab、Bitbucket 和其他非 GitHub 儲存庫可以作為[本機捆綁](#send-local-repositories-without-github)發送到雲端工作階段，但工作階段無法將結果推送回遠端
* **組織 IP 允許清單**：雲端工作階段從 Anthropic 管理的基礎設施而不是您的網路呼叫 Anthropic API。如果您的組織啟用了 [IP 允許清單](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting)，每個雲端工作階段都會失敗，出現驗證錯誤。這同樣適用於[程式碼審查](/docs/zh-TW/code-review)和[例行工作](/docs/zh-TW/routines)。聯絡 [Anthropic 支援](https://support.claude.com/)以從您的組織的 IP 允許清單中豁免 Anthropic 託管的服務。

<h2 id="related-resources">
  相關資源
</h2>

* [Ultraplan](/docs/zh-TW/ultraplan)：在雲端工作階段中起草計畫並在瀏覽器中檢查它
* [Ultrareview](/docs/zh-TW/ultrareview)：在雲端沙箱中執行深度多代理程式碼審查
* [Routines](/docs/zh-TW/routines)：自動化按排程、通過 API 呼叫或回應 GitHub 事件的工作
* [Hooks 配置](/docs/zh-TW/hooks)：在工作階段生命週期事件執行指令碼
* [設定參考](/docs/zh-TW/settings)：所有配置選項
* [安全性](/docs/zh-TW/security)：隔離保證和資料處理
* [資料使用](/docs/zh-TW/data-usage)：Anthropic 從雲端工作階段保留的內容
* [Claude Tag](https://claude.com/docs/claude-tag/overview)：在 Slack 中由組織管理的 @Claude，在相同的雲端環境中執行
