> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 設定沙箱化 Bash 工具

> 了解 Claude Code 的沙箱化 Bash 工具如何提供檔案系統和網路隔離，以實現更安全、更自主的代理執行。

Bash 沙箱讓 Claude 執行大多數 shell 命令，而無需停下來請求權限。與其批准每個命令，您可以定義命令可以接觸哪些檔案和網路域，作業系統會為每個 Bash 命令及其子流程強制執行該邊界。

<Note>
  若要比較其他隔離方法，例如開發容器、自訂容器和虛擬機，請參閱 [Sandbox environments](/docs/zh-TW/sandbox-environments)。若要減少 Bash 以外工具的權限提示，請參閱 [permission modes](/docs/zh-TW/permission-modes)。
</Note>

<h2 id="get-started">
  開始使用
</h2>

沙箱內建於 Claude Code 中，在 macOS、Linux 和 WSL2 上執行。不支援原生 Windows。在 Windows 上，在 WSL2 發行版內執行 Claude Code。

在 macOS 上，無需安裝任何內容：沙箱化使用內建的 Seatbelt 框架。在 Linux 和 WSL2 上，沙箱依賴於兩個套件，詳見 [Set up Linux and WSL2](#set-up-linux-and-wsl2)。即使您還沒有安裝它們，您也可以從 `/sandbox` 開始，因為其面板會顯示是否缺少任何內容。

<Steps>
  <Step title="執行 /sandbox">
    啟動 Claude Code 工作階段並執行 `/sandbox` 命令：

    ```text theme={null}
    /sandbox
    ```

    這會開啟沙箱面板，有三個標籤：

    * **Mode**：選擇沙箱化命令的批准方式，詳見下一步
    * **Overrides**：選擇在沙箱下失敗的命令是否可以回退到執行未沙箱化。這是 [`allowUnsandboxedCommands`](/docs/zh-TW/settings#sandbox-settings) 設定
    * **Config**：檢視已解析的沙箱設定

    如果面板只顯示 Dependencies 標籤，則缺少必需的套件。按照 [Set up Linux and WSL2](#set-up-linux-and-wsl2) 中的說明安裝它，重新啟動 Claude Code，然後再次執行 `/sandbox`。
  </Step>

  <Step title="選擇一個模式">
    在 Mode 標籤上，選擇自動允許或常規權限。自動允許在不提示的情況下執行沙箱化命令，常規權限即使命令沙箱化也保持常規權限提示。請參閱 [Sandbox modes](#sandbox-modes) 了解在自動允許模式中仍然提示的命令。
  </Step>

  <Step title="執行 Bash 命令">
    要求 Claude 執行命令，例如構建或測試套件。預設情況下，沙箱內的命令只能寫入工作目錄和工作階段暫存目錄。命令首次需要新的網路域時，Claude Code 會提示批准。

    無法沙箱化執行的命令會回退到常規權限流程。若要擴大或縮小這些邊界，請參閱 [Configure sandboxing](#configure-sandboxing)。
  </Step>
</Steps>

在面板中選擇模式會寫入您專案的本地設定，位於 `.claude/settings.local.json`，這適用於目前專案，不會簽入 git。若要在所有專案中啟用沙箱，請在 `~/.claude/settings.json` 的使用者設定中將 [`sandbox.enabled`](/docs/zh-TW/settings#sandbox-settings) 設定為 `true`。若要為組織中的每個開發人員強制執行沙箱化，請使用 [managed settings](#enforce-sandboxing-with-managed-settings)。

<Warning>
  預設情況下，如果沙箱因缺少依賴項或不支援的平台而無法啟動，Claude Code 會顯示警告並在沒有沙箱化的情況下執行命令。若要改為將其設為硬失敗，請將 [`sandbox.failIfUnavailable`](/docs/zh-TW/settings#sandbox-settings) 設定為 `true`。這適用於需要沙箱化作為安全閘道的受管部署。
</Warning>

<h3 id="set-up-linux-and-wsl2">
  設定 Linux 和 WSL2
</h3>

在 Linux 和 WSL2 上，沙箱依賴於兩個套件：

* [`bubblewrap`](https://github.com/containers/bubblewrap)：無特權沙箱化工具，強制執行檔案系統隔離
* [`socat`](http://www.dest-unreach.org/socat/)：用於通過沙箱代理路由網路流量的中繼

使用您發行版的套件管理器安裝它們：

<Tabs>
  <Tab title="Ubuntu/Debian">
    ```bash theme={null}
    sudo apt-get install bubblewrap socat
    ```
  </Tab>

  <Tab title="Fedora">
    ```bash theme={null}
    sudo dnf install bubblewrap socat
    ```
  </Tab>
</Tabs>

安裝後，`/sandbox` 中的 Dependencies 標籤會顯示 `ripgrep`、`bubblewrap`、`socat` 和 seccomp 過濾器是否在您的平台上可用。Ripgrep 與原生 Claude Code 二進位檔案一起打包。seccomp 過濾器是可選的，增加 Unix 域套接字阻止。如果缺少，請使用 `npm install -g @anthropic-ai/sandbox-runtime` 安裝它。

當缺少必需的依賴項時，Dependencies 標籤是唯一顯示的標籤，直到您安裝它。依賴項檢查在啟動時執行，因此在安裝套件後重新啟動 Claude Code，以便 `/sandbox` 檢測到它們。

<AccordionGroup>
  <Accordion title="Ubuntu 24.04 及更新版本：允許 bubblewrap 建立使用者命名空間">
    在 Ubuntu 24.04 及更新版本上，預設 AppArmor 策略防止 bubblewrap 建立隔離所需的使用者命名空間。

    若要檢查您的環境（包括 WSL2 內）是否強制執行此限制，請執行 `sysctl kernel.apparmor_restrict_unprivileged_userns`。如果金鑰不存在或傳回 `0`，請跳過此步驟。如果傳回 `1`，請新增授予 `bwrap` 此功能的 AppArmor 設定檔：

    ```bash theme={null}
    sudo tee /etc/apparmor.d/bwrap > /dev/null <<'EOF'
    abi <abi/4.0>,
    include <tunables/global>

    profile bwrap /usr/bin/bwrap flags=(unconfined) {
      userns,
      include if exists <local/bwrap>
    }
    EOF
    ```

    該設定檔僅適用於 `bwrap` 本身，不適用於在沙箱內執行的命令。重新載入 AppArmor 以應用它：

    ```bash theme={null}
    sudo systemctl reload apparmor
    ```
  </Accordion>

  <Accordion title="WSL2 注意事項">
    使用 PowerShell 中的 `wsl -l -v` 檢查您的 WSL 版本。如果您看到 `Sandboxing requires WSL2`，您的發行版執行的是 WSL1。將其升級到 WSL2 或在沒有沙箱化的情況下執行 Claude Code。

    在 WSL2 上，沙箱化命令無法啟動 Windows 二進位檔案，例如 `cmd.exe`、`powershell.exe` 或 `/mnt/c/` 下的任何內容。WSL 通過 Unix 套接字將這些交給 Windows 主機，沙箱會阻止此操作。如果命令需要呼叫 Windows 二進位檔案，請將其新增到 [`excludedCommands`](/docs/zh-TW/settings#sandbox-settings)，以便它在沙箱外執行。
  </Accordion>
</AccordionGroup>

<h3 id="sandbox-modes">
  沙箱模式
</h3>

Claude Code 提供兩種沙箱模式：

**自動允許模式**：Bash 命令將嘗試在沙箱內執行，並自動允許而無需權限。無法沙箱化的命令（例如需要存取非允許主機的網路存取的命令）會回退到常規權限流程，其中 Claude Code 檢查您的 [permission rules](/docs/zh-TW/permissions) 並提示您進行這些規則不允許的任何命令，在預設模式中提示或在 [auto mode](/docs/zh-TW/permission-modes#eliminate-prompts-with-auto-mode) 中使用分類器。

即使在自動允許模式中，以下仍然適用：

* 明確的 [deny rules](/docs/zh-TW/permissions) 始終被尊重
* 針對 `/`、您的主目錄或其他關鍵系統路徑的 `rm` 或 `rmdir` 命令仍然會觸發權限提示
* 內容範圍的 [ask rules](/docs/zh-TW/permissions)（例如 `Bash(git push *)`）仍然會強制提示，即使是沙箱化命令
* 裸 `Bash` ask 規則，或等效的 `Bash(*)` 形式，對於執行沙箱化的命令會被跳過；它仍然適用於回退到常規權限流程的命令

**常規權限模式**：所有 Bash 命令都通過常規權限流程進行，即使沙箱化也是如此。這提供了更多控制，但需要更多批准。

在兩種模式中，沙箱強制執行相同的檔案系統和網路限制。區別僅在於沙箱化命令是自動批准還是需要明確權限。

工作階段暫存目錄在沙箱內預設可寫，與工作目錄一起。Claude Code 為沙箱化命令設定 `$TMPDIR` 為此目錄，因此寫入暫存檔案的工具無需額外配置即可工作。未沙箱化命令繼承您的 shell 的 `$TMPDIR` 不變，這意味著沙箱化和未沙箱化命令將 `$TMPDIR` 解析為不同的目錄。若要在兩者之間傳遞暫存檔案，請改為在工作目錄下寫入它們。

某些命令根本無法在沙箱內執行，例如與其不相容的工具或需要您未允許的主機的工具。與其讓任務失敗或要求您關閉沙箱化，Claude Code 包含一個逃生艙：當命令因沙箱限制而失敗時，Claude 分析失敗，可能使用 `dangerouslyDisableSandbox` 參數重試命令。重試的命令在沙箱外執行，因此通過常規權限流程進行：在預設模式中您會獲得確認提示；在 [auto mode](/docs/zh-TW/permission-modes#eliminate-prompts-with-auto-mode) 中分類器評估基礎命令而不是提示您。若要在 auto mode 中即使在每次未沙箱化重試時也被提示，請新增 [ask rule](/docs/zh-TW/permissions#match-by-input-parameter) 用於 `Bash(dangerouslyDisableSandbox:true)`。

您可以通過在 [sandbox settings](/docs/zh-TW/settings#sandbox-settings) 中設定 `"allowUnsandboxedCommands": false` 來禁用此逃生艙。禁用時，`/sandbox` Overrides 標籤顯示為 **Strict sandbox mode**，`dangerouslyDisableSandbox` 參數被完全忽略，所有命令必須沙箱化執行或在 `excludedCommands` 中明確列出。

<Info>
  自動允許模式獨立於您的權限模式設定工作。即使您不在「接受編輯」模式中，當啟用自動允許時，沙箱化 Bash 命令也會自動執行。這意味著在沙箱邊界內修改檔案的 Bash 命令將執行而不提示，即使檔案編輯工具通常需要批准。
</Info>

<h2 id="configure-sandboxing">
  設定沙箱化
</h2>

通過您的 `settings.json` 檔案自訂沙箱行為。請參閱 [Settings](/docs/zh-TW/settings#sandbox-settings) 以了解完整的配置參考。

預設情況下，沙箱化命令只能寫入目前工作目錄和工作階段暫存目錄。如果子流程命令（如 `kubectl`、`terraform` 或 `npm`）需要寫入這些目錄外，請使用 `sandbox.filesystem.allowWrite` 授予對特定路徑的存取：

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "allowWrite": ["~/.kube", "/tmp/build"]
    }
  }
}
```

這些路徑在作業系統級別強制執行，因此在沙箱內執行的所有命令（包括其子流程）都尊重它們。當工具需要對特定位置的寫入存取時，這是推薦的方法，而不是使用 `excludedCommands` 將工具排除在沙箱外。

當在多個 [settings scopes](/docs/zh-TW/settings#settings-precedence) 中定義相同的檔案系統陣列時，陣列被合併：來自每個範圍的路徑被組合，而不是被替換。

路徑前綴控制路徑的解析方式：

| 前綴        | 含義                                    | 範例                                                                |
| :-------- | :------------------------------------ | :---------------------------------------------------------------- |
| `/`       | 從檔案系統根目錄的絕對路徑                         | `/tmp/build` 保持 `/tmp/build`                                      |
| `~/`      | 相對於主目錄                                | `~/.kube` 變成 `$HOME/.kube`                                        |
| `./` 或無前綴 | 相對於專案設定的專案根目錄，或相對於 `~/.claude` 的使用者設定 | `.claude/settings.json` 中的 `./output` 解析為 `<project-root>/output` |

此語法與 [Read and Edit permission rules](/docs/zh-TW/permissions#read-and-edit) 不同，後者使用 `//path` 表示絕對路徑，`/path` 表示專案相對路徑。沙箱檔案系統路徑使用標準慣例：`/tmp/build` 是絕對路徑。

您也可以使用 `sandbox.filesystem.denyWrite` 和 `sandbox.filesystem.denyRead` 拒絕寫入或讀取存取，並使用 `sandbox.filesystem.allowRead` 重新允許讀取被拒絕區域內的特定路徑。當讀取規則重疊時，更具體的路徑優先：

| 範例規則                                                  | 結果                                                            |
| :---------------------------------------------------- | :------------------------------------------------------------ |
| `"denyRead": ["~/"]` 搭配 `"allowRead": ["~/projects"]` | `~/projects` 可讀，主目錄的其餘部分保持被阻止。較窄的允許重新開啟被拒絕區域的該部分              |
| `"allowRead": ["~/"]` 搭配 `"denyRead": ["~/.env"]`     | `~/.env` 保持被阻止，主目錄的其餘部分可讀。精確的拒絕在更寬的允許內保持有效，因此廣泛的允許無法無聲地重新暴露機密 |

下面的範例阻止從整個主目錄讀取，同時仍允許從目前專案讀取。將其放在您的專案的 `.claude/settings.json` 中，因為相對路徑 `.` 僅在配置位於專案設定中時才解析為專案根目錄：

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "denyRead": ["~/"],
      "allowRead": ["."]
    }
  }
}
```

`allowRead` 中的 `.` 解析為專案根目錄，因為此配置位於專案設定中。如果您將相同的配置放在 `~/.claude/settings.json` 中，`.` 將解析為 `~/.claude`，專案檔案將保持被 `denyRead` 規則阻止。

<h3 id="protect-credentials">
  保護認證
</h3>

`sandbox.credentials` 設定聲明認證檔案和環境變數以保護沙箱化命令。每個項目命名一個檔案路徑或環境變數和一個 `mode`。專用的 `credentials` 區塊將認證規則分組在一起，並與一般檔案系統規則分開。需要 Claude Code v2.1.187 或更新版本。

對於 `"mode": "deny"` 的項目，檔案路徑在沙箱內被拒絕讀取，與 `filesystem.denyRead` 應用的限制相同，環境變數在每個沙箱化命令執行前被取消設定。

下面的範例阻止讀取 AWS 認證檔案和 SSH 目錄，並從沙箱化命令的環境中移除 `GITHUB_TOKEN` 和 `NPM_TOKEN`：

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "credentials": {
      "files": [
        { "path": "~/.aws/credentials", "mode": "deny" },
        { "path": "~/.ssh", "mode": "deny" }
      ],
      "envVars": [
        { "name": "GITHUB_TOKEN", "mode": "deny" },
        { "name": "NPM_TOKEN", "mode": "deny" }
      ]
    }
  }
}
```

檔案項目僅支援 `"mode": "deny"`。環境變數項目也接受 `"mode": "mask"`，如下所述。

檔案路徑遵循與 `sandbox.filesystem.*` 設定相同的 [prefix rules](/docs/zh-TW/settings#sandbox-path-prefixes)，來自每個 [settings scope](/docs/zh-TW/settings#settings-precedence) 的 `deny` 項目被合併。`deny` 項目只會縮小存取，因此任何範圍都可以新增一個，但沒有任何範圍可以移除另一個範圍新增的項目。

沒有內建的認證拒絕清單，因此只有您列出的檔案和變數被限制。此設定僅影響沙箱化 Bash 命令。若要從所有子流程中移除 Anthropic 和雲端提供者認證，無論沙箱化如何，請設定 [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/zh-TW/env-vars)。

<h4 id="mask-environment-variables">
  遮罩環境變數
</h4>

`"mode": "mask"` 保護認證同時保持使用它進行身份驗證的工具正常工作。`deny` 完全移除變數，這也會破壞需要它的工具，例如 `gh` 或 `npm`。需要 Claude Code v2.1.199 或更新版本。

使用 `mask`，沙箱化命令看到的是每個工作階段的哨兵值而不是真實值。當請求離開沙箱前往認證的 `injectHosts` 之一時，[sandbox proxy](#network-isolation) 將哨兵值替換為真實值。命令和它記錄的任何內容都不會持有真實認證，但其請求仍然進行身份驗證。

代理在請求內容中替換認證，因此它必須看到它們。設定 [`network.tlsTerminate`](/docs/zh-TW/settings#sandbox-settings) 以便代理自己終止 TLS。沒有它，遮罩會失敗關閉：命令仍然只看到哨兵值，但哨兵值不變地到達伺服器，身份驗證失敗。Claude Code 在啟動時報告此配置錯誤。

下面的範例遮罩兩個令牌。`GH_TOKEN` 僅在對 `api.github.com` 的請求上被替換，而 `NPM_TOKEN` 沒有 `injectHosts` 並在對 `network.allowedDomains` 中每個主機的請求上被替換。每個 `injectHosts` 項目本身必須被 `network.allowedDomains` 覆蓋。

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "network": {
      "tlsTerminate": {},
      "allowedDomains": ["*.github.com", "registry.npmjs.org"]
    },
    "credentials": {
      "envVars": [
        { "name": "GH_TOKEN", "mode": "mask", "injectHosts": ["api.github.com"] },
        { "name": "NPM_TOKEN", "mode": "mask" }
      ]
    }
  }
}
```

與 `deny` 不同，遮罩授權代理將您的真實認證發送到列出的主機，因此它僅從您或您的管理員控制的設定中被尊重：使用者設定、受管設定和 `--settings` CLI 旗標。`mask` 項目、`network.tlsTerminate` 和 [`credentials.allowPlaintextInject`](/docs/zh-TW/settings#sandbox-settings) 在儲存庫的 `.claude/settings.json` 或 `.claude/settings.local.json` 中被忽略。

當相同的變數在任何範圍中以 `deny` 列出時，`deny` 優先。

<h2 id="how-sandboxing-works">
  沙箱化如何運作
</h2>

<h3 id="filesystem-isolation">
  檔案系統隔離
</h3>

沙箱化 Bash 工具將檔案系統存取限制在特定目錄：

* **預設寫入行為**：對目前工作目錄及其子目錄的讀取和寫入存取，加上 `$TMPDIR` 指向的工作階段暫存目錄
* **預設讀取行為**：對整個電腦的讀取存取，除了某些被拒絕的目錄。請注意，此預設仍允許讀取認證檔案，例如 `~/.aws/credentials` 和 `~/.ssh/`。使用 [`sandbox.credentials`](#protect-credentials) 來阻止讀取這些檔案並取消設定祕密環境變數，或將路徑新增到 `denyRead`。
* **被阻止的存取**：無法在沒有明確權限的情況下修改目前工作目錄和工作階段暫存目錄外的檔案，包括 shell 配置檔案（例如 `~/.bashrc`）和 `/bin/` 中的系統二進位檔案
* **Git worktrees**：當工作目錄是[連結的 git worktree](/docs/zh-TW/worktrees)時，沙箱也允許寫入主儲存庫的共享 `.git` 目錄，以便 `git commit` 等命令可以更新 refs 和索引。對該目錄內的 `hooks/` 和 `config` 的寫入仍然被拒絕。
* **可配置**：通過設定定義自訂允許和拒絕的路徑

您可以使用設定中的 `sandbox.filesystem.allowWrite` 授予對其他路徑的寫入存取。這些限制在作業系統級別強制執行，因此它們適用於所有子流程命令，包括 `kubectl`、`terraform` 和 `npm` 等工具，而不僅僅是 Claude 的檔案工具。

<h3 id="network-isolation">
  網路隔離
</h3>

網路存取通過在沙箱外執行的代理伺服器進行控制：

* **域名限制**：沒有預先允許的域名。命令首次需要新的域名時，Claude Code 會提示批准。自 v2.1.191 起，選擇「是」允許該主機在目前工作階段的其餘時間內使用，因此稍後連線到同一主機時不會再次提示。使用 [`allowedDomains`](/docs/zh-TW/settings#sandbox-settings) 預先允許域名以避免提示。
* **受管鎖定**：如果在受管設定中設定了 [`allowManagedDomainsOnly`](/docs/zh-TW/settings#sandbox-settings)，非允許的域名會自動被阻止而不是提示，只有來自受管設定的 `allowedDomains` 被尊重。
* **自訂代理支援**：進階使用者可以在出站流量上實施自訂規則
* **全面覆蓋**：限制適用於所有指令碼、程式和由命令產生的子流程

<Note>
  內建代理根據請求的主機名強制執行允許清單，預設不終止或檢查 TLS 流量。實驗性的 [`network.tlsTerminate`](/docs/zh-TW/settings#sandbox-settings) 設定在 Claude Code v2.1.199 及更新版本中可用，使內建代理自行終止 TLS，這是 [`mask` 認證項目](#protect-credentials)所需的。請參閱 [Security limitations](#security-limitations) 了解預設設計的含義，以及 [Custom proxy configuration](#custom-proxy-configuration) 如果您的威脅模型需要 TLS 檢查。
</Note>

<h3 id="os-level-enforcement">
  作業系統級別的強制執行
</h3>

沙箱化 Bash 工具利用作業系統安全原語：

* **macOS**：使用 Seatbelt 進行沙箱強制執行
* **Linux**：使用 [bubblewrap](https://github.com/containers/bubblewrap) 進行隔離
* **WSL2**：使用 bubblewrap，與 Linux 相同

不支援 WSL1，因為 bubblewrap 需要僅在 WSL2 中可用的核心功能。這些作業系統級別的限制確保由 Claude Code 命令產生的所有子流程都繼承相同的安全邊界。

這些相同的原語可作為獨立的 [`@anthropic-ai/sandbox-runtime`](https://github.com/anthropic-experimental/sandbox-runtime) 套件使用，[Sandbox environments](/docs/zh-TW/sandbox-environments#sandbox-runtime) 頁面涵蓋作為包裝整個 Claude Code 流程的單獨方法。

<h2 id="how-sandboxing-relates-to-permissions-and-permission-modes">
  沙箱化與權限和權限模式的關係
</h2>

沙箱化、[permission rules](/docs/zh-TW/permissions) 和 [permission modes](/docs/zh-TW/permission-modes) 是互補的層。下面的部分涵蓋沙箱如何與每個互動。

<h3 id="permission-rules">
  權限規則
</h3>

權限規則和沙箱化控制不同的事物：

* **權限規則**控制 Claude Code 可以使用哪些工具，並在任何工具執行之前進行評估。它們適用於所有工具：Bash、Read、Edit、WebFetch、MCP 和其他工具。
* **沙箱化**提供作業系統級別的強制執行，限制 Bash 命令在檔案系統和網路級別可以存取的內容。它僅適用於 Bash 命令及其子流程。

這兩層在強制執行方式上也有所不同。Claude Code 在命令執行之前根據命令字串評估權限決定，在自動模式中，還根據單獨分類器對命令是否安全的判斷。作業系統在執行流程上強制執行沙箱邊界，因此無論模型選擇執行什麼，它都成立，即使允許的命令執行的操作超出其名稱所示。

檔案系統和網路限制通過沙箱設定和權限規則進行配置：

| 設定或規則                                                          | 它的作用                                              |
| :------------------------------------------------------------- | :------------------------------------------------ |
| `sandbox.filesystem.allowWrite`                                | 授予子流程對工作目錄外路徑的寫入存取                                |
| `sandbox.filesystem.denyWrite` 和 `sandbox.filesystem.denyRead` | 阻止子流程對特定路徑的存取                                     |
| `sandbox.filesystem.allowRead`                                 | 重新允許讀取 `denyRead` 區域內的特定路徑                        |
| `Edit` 允許規則                                                    | 授予對特定路徑的寫入存取，與 `sandbox.filesystem.allowWrite` 相同 |
| `Read` 和 `Edit` 拒絕規則                                           | 阻止對特定檔案或目錄的存取                                     |
| `WebFetch` 允許和拒絕規則                                             | 控制域名存取                                            |
| 沙箱 `allowedDomains`                                            | 控制 Bash 命令可以到達的域名                                 |
| 沙箱 `deniedDomains`                                             | 阻止特定域名，即使更廣泛的 `allowedDomains` 萬用字元會允許它們          |

來自 `sandbox.filesystem` 設定和權限規則的路徑被合併到最終沙箱配置中。

[claude-code repository 的 examples 目錄](https://github.com/anthropics/claude-code/tree/main/examples/settings)包含常見部署場景的入門設定配置，包括沙箱特定的範例。使用這些作為起點，並根據您的需求進行調整。

<h3 id="permission-modes">
  權限模式
</h3>

`/sandbox` 不是 [permission mode](/docs/zh-TW/permission-modes)。權限模式決定工具呼叫是否執行以及您是否首先被提示，而沙箱限制 Bash 命令執行後可以存取的內容。它們在控制的內容和替換每個操作提示的內容上有所不同：

|                                                                       | 它控制什麼             | 替換提示的內容                                                                                                                                                                                                                                                                                                              |
| :-------------------------------------------------------------------- | :---------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/sandbox`                                                            | Bash 命令執行後可以存取的內容 | 沙箱邊界本身，在 [auto-allow mode](#sandbox-modes) 中                                                                                                                                                                                                                                                                         |
| [Auto mode](/docs/zh-TW/permission-modes#eliminate-prompts-with-auto-mode) | 每個工具呼叫是否執行        | 檢查操作的分類器                                                                                                                                                                                                                                                                                                             |
| `--dangerously-skip-permissions`                                      | 每個工具呼叫是否執行        | 無。[Protected path](/docs/zh-TW/permission-modes#protected-paths) 檢查也被跳過；只有明確的 [ask rules](/docs/zh-TW/permissions#manage-permissions)、連接器工具 [您的組織設定為 `ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools)、標記為 [`requiresUserInteraction`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 的 MCP 工具，以及移除 `/` 或您的主目錄仍然提示 |

沙箱的 [auto-allow mode](#sandbox-modes) 與 [auto mode](/docs/zh-TW/permission-modes#eliminate-prompts-with-auto-mode) 分開：自動允許批准 Bash 命令，因為沙箱邊界包含它們，而自動模式使用分類器檢查操作。這兩個獨立工作，可以結合。若要為無人值守執行選擇隔離邊界，請參閱 [Sandbox environments](/docs/zh-TW/sandbox-environments#how-isolation-relates-to-permission-modes)。

<h2 id="configure-the-sandbox-for-your-organization">
  為您的組織設定沙箱
</h2>

管理員可以為每個使用者要求沙箱化，防止開發人員擴大策略，並通過公司代理路由沙箱流量。

<h3 id="enforce-sandboxing-with-managed-settings">
  使用受管設定強制執行沙箱化
</h3>

若要為每個開發人員要求沙箱，通過 [managed settings](/docs/zh-TW/settings#settings-files) 傳遞 `sandbox` 金鑰，可以是由您的 MDM 管理的檔案，也可以是通過 Claude.ai 上的 [server-managed settings](/docs/zh-TW/server-managed-settings)。

以下受管設定配置啟用沙箱，如果沙箱無法初始化則拒絕啟動 Claude Code，並防止模型在沙箱外重試命令：

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "failIfUnavailable": true,
    "allowUnsandboxedCommands": false
  }
}
```

超過 `enabled` 的兩個金鑰控制沙箱無法執行命令時會發生什麼：

* **`failIfUnavailable`**：缺少的依賴項（例如 Linux 上的 bubblewrap）會阻止 Claude Code 啟動，而不是顯示警告並回退到未沙箱化執行
* **`allowUnsandboxedCommands: false`**：`dangerouslyDisableSandbox` 逃生艙被忽略，因此在沙箱下失敗的命令無法在其外重試

值得考慮與它們一起的兩個補充。為任何必須在沒有隔離的情況下執行的組織批准的工具新增 `excludedCommands`。為認證目錄（例如 `~/.aws` 和 `~/.ssh`）和祕密環境變數新增 [`sandbox.credentials`](#protect-credentials) 項目，因為預設讀取策略仍允許這些。

沙箱不在原生 Windows 上執行，因此如果您的機隊包括 Windows 主機，請將此配置限制在 macOS 和 Linux，或讓這些使用者在 WSL2 或容器內執行 Claude Code。

<h3 id="keep-developers-from-widening-the-policy">
  防止開發人員擴大策略
</h3>

對於布林金鑰（例如 `enabled` 和 `failIfUnavailable`），Claude Code 使用受管值並忽略開發人員在本地設定的任何內容。對於陣列金鑰（例如 `excludedCommands` 和 `allowRead`），Claude Code 合併來自每個範圍的項目，因此開發人員可以附加擴大策略的項目。

在受管設定中將 `allowManagedReadPathsOnly` 設定為 `true`，以便只有來自受管設定的 `allowRead` 項目被尊重。使用者、專案和本地 `allowRead` 項目被忽略。這防止開發人員擴大讀取存取超過組織批准的路徑。若要以相同方式將網路域鎖定到受管值，請設定 [`allowManagedDomainsOnly`](/docs/zh-TW/settings#sandbox-settings)。

`excludedCommands` 沒有等效的受管專用鎖定，因此開發人員總是可以附加在沙箱外執行其他命令的項目。保持受管清單狹窄。

<h3 id="custom-proxy-configuration">
  自訂代理配置
</h3>

對於需要進階網路安全的組織，您可以實施自訂代理以：

* 解密和檢查 HTTPS 流量
* 應用自訂過濾規則
* 記錄所有網路請求
* 與現有安全基礎設施整合

若要將 Claude Code 指向您的代理，請在 [sandbox settings](/docs/zh-TW/settings#sandbox-settings) 中設定代理連接埠：

```json theme={null}
{
  "sandbox": {
    "network": {
      "httpProxyPort": 8080,
      "socksProxyPort": 8081
    }
  }
}
```

<h2 id="troubleshooting">
  故障排除
</h2>

某些命令在沙箱內失敗，即使它們在沙箱外工作。下面的修復涵蓋最常見的情況。

* **命令因主機不允許錯誤而失敗**：許多 CLI 工具需要到達特定主機。在提示時授予權限會將主機新增到您的允許清單，以便工具在將來在沙箱內執行。
* **`jest` 掛起或失敗**：`watchman` 與沙箱不相容。改為執行 `jest --no-watchman`。
* **Go 型 CLI 在 macOS 上 TLS 驗證失敗**：`gh`、`gcloud` 和 `terraform` 等工具在 Seatbelt 下可能無法進行 TLS 驗證。在 `excludedCommands` 中列出這些工具以在沙箱外執行它們。如果您使用 `httpProxyPort` 與 MITM 代理和自訂 CA，請改為將 [`enableWeakerNetworkIsolation`](/docs/zh-TW/settings#sandbox-settings) 設定為 `true`。
* **`open`、`osascript` 或瀏覽器型驗證流程在 macOS 上因錯誤 `-600` 而失敗**：沙箱預設會阻止 Apple Events。在您的使用者、受管理或 CLI 設定中將 [`allowAppleEvents`](/docs/zh-TW/settings#sandbox-settings) 設定為 `true` 以允許它們。專案設定會被忽略此金鑰。啟用它會移除程式碼執行隔離，因為沙箱化命令之後可以啟動其他應用程式而不進行沙箱化，無需使用者提示，並向執行中的應用程式傳送 AppleScript 命令，受限於 macOS 自動化同意提示 (TCC)。或者，將命令新增到 `excludedCommands` 以在沙箱外執行它。
* **`docker` 命令失敗**：`docker` 與沙箱不相容。將 `docker *` 新增到 `excludedCommands` 以在沙箱外執行它。
* **Bubblewrap 在容器內啟動失敗**：在無特權容器中，bubblewrap 無法掛載新的 `/proc` 檔案系統。將 [`enableWeakerNestedSandbox`](/docs/zh-TW/settings#sandbox-settings) 設定為 `true`，以便內部沙箱綁定掛載容器的現有 `/proc`。僅在外部容器已提供您需要的隔離邊界時使用此設定，因為它向沙箱化命令公開流程資訊，新的 `/proc` 掛載會隱藏。
* **Linux 上的 Seccomp 過濾器**：seccomp 過濾器是阻止 Unix 域套接字所必需的。`/sandbox` 中的 Dependencies 標籤顯示它是否可用。如果缺少，請執行 `npm install -g @anthropic-ai/sandbox-runtime` 安裝幫助程式。
* **`--dangerously-skip-permissions` 以 root 身份失敗**：在 Linux 和 macOS 上以 root 身份或通過 sudo 執行時，此旗標被阻止，因為 root 存取加上沒有權限提示可以修改系統上的任何檔案或服務。檢查在識別的沙箱內自動跳過。若要在容器中自主執行，請使用 [dev container](/docs/zh-TW/devcontainer) 配置，它以非 root 使用者身份執行 Claude Code。

<h2 id="limitations">
  限制
</h2>

沙箱化減少風險，但不是完整的隔離邊界。在依賴它作為硬安全控制之前，請檢查下面的限制。

<h3 id="security-limitations">
  安全限制
</h3>

* **網路過濾**：沙箱限制流程可以連接的域名。預設情況下，內建代理不終止或檢查出站流量上的 TLS，因此加密連接的內容不被檢查。實驗性的 [`network.tlsTerminate`](/docs/zh-TW/settings#sandbox-settings) 設定在代理處終止 TLS 以進行 [`mask` 認證替換](#protect-credentials)，但不添加內容過濾。您負責確保只有受信任的域名在您的策略中被允許。

<Warning>
  允許廣泛域名（例如 `github.com`）可能會為資料洩露建立路徑。因為代理根據用戶端提供的主機名進行允許決定而不檢查 TLS，在沙箱內執行的程式碼可能可以使用 [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting) 或類似技術到達允許清單外的主機。如果您的威脅模型需要更強的保證，請配置 [custom proxy](#custom-proxy-configuration)，它終止 TLS 並檢查流量，並在沙箱內安裝其 CA 憑證。更強的 TLS 感知網路隔離是一個活躍的開發領域。
</Warning>

* **通過 Unix 套接字的特權提升**：`allowUnixSockets` 配置可能會無意中授予對強大系統服務的存取，這可能導致沙箱繞過。例如，允許存取 `/var/run/docker.sock` 有效地通過 Docker 套接字授予對主機系統的存取。仔細考慮您通過沙箱允許的任何 Unix 套接字。
* **檔案系統權限提升**：過於寬泛的檔案系統寫入權限可能導致特權提升攻擊。允許寫入包含 `$PATH` 中可執行檔案的目錄、系統配置目錄或使用者 shell 配置檔案（例如 `.bashrc` 或 `.zshrc`）可能導致當其他使用者或系統流程存取這些檔案時在不同安全上下文中執行程式碼。
* **Linux 沙箱強度**：Linux 實現提供強大的檔案系統和網路隔離，但包含一個 `enableWeakerNestedSandbox` 模式，使其能夠在 Docker 環境中工作而無需特權命名空間，或在 Linux 主機上禁用無特權使用者命名空間的情況下。此選項大大削弱了安全性，應僅在其他隔離被強制執行時使用。
* **macOS 上的 Apple Events**：macOS 沙箱預設阻止 Apple Events。`allowAppleEvents` 設定解除此限制，使 `open` 和 `osascript` 等工具能夠運作，但它移除了程式碼執行隔離：沙箱化命令可以啟動其他應用程式而不進行沙箱化，無需使用者提示，並可以向執行中的應用程式傳送 AppleScript 命令，受限於每個應用程式的 macOS 自動化同意提示 (TCC)。它僅從使用者、受管或 CLI 設定中被接受。專案設定無法啟用它。
* **設定檔案受保護**：沙箱自動拒絕對 Claude Code 的 `settings.json` 檔案在每個範圍和受管設定目錄的寫入存取，因此沙箱化命令無法修改其自己的策略。

<h3 id="platform-and-tool-compatibility">
  平台和工具相容性
</h3>

* **平台支援**：支援 macOS、Linux 和 WSL2。不支援 WSL1 和原生 Windows。
* **效能開銷**：最小，但某些檔案系統操作可能稍慢。
* **工具相容性**：某些需要特定系統存取模式的工具可能需要配置調整，或可能需要在沙箱外執行。

<h3 id="scope">
  範圍
</h3>

沙箱隔離 Bash 子流程。其他工具在不同的邊界下運作：

* **內建檔案工具**：Read、Edit 和 Write 直接使用權限系統，而不是通過沙箱執行。請參閱 [permissions](/docs/zh-TW/permissions)。
* **電腦使用**：當 Claude 打開應用程式並控制您的螢幕時，它在您的實際桌面上執行，而不是在隔離環境中。每個應用程式的權限提示控制每個應用程式。請參閱 [CLI 中的電腦使用](/docs/zh-TW/computer-use) 或 [Desktop 中的電腦使用](/docs/zh-TW/desktop#let-claude-use-your-computer)。
* **環境變數**：沙箱化 Bash 命令預設繼承父流程環境，包括在那裡設定的任何認證。使用 [`sandbox.credentials`](#protect-credentials) 為沙箱化命令取消設定或遮罩特定變數，或設定 [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/zh-TW/env-vars) 以從所有子流程中去除 Anthropic 和雲端提供商認證。
* **子代理**：[subagents](/docs/zh-TW/sub-agents) 在與父工作階段相同的流程中執行，並使用相同的沙箱配置。當在父工作階段中啟用沙箱化時，子代理內的 Bash 命令被沙箱化。

<Warning>
  有效的沙箱化需要同時進行檔案系統和網路隔離。沒有網路隔離，受損的代理可能會洩露敏感檔案，如 SSH 金鑰。沒有檔案系統隔離，受損的代理可能會後門系統資源以獲得網路存取。當您擴大預設值時，檢查 `allowWrite` 路徑、廣泛的 `allowedDomains` 項目或 `excludedCommands` 例外是否不會撤銷另一側的限制。
</Warning>

<h2 id="see-also">
  另請參閱
</h2>

* [Sandbox environments](/docs/zh-TW/sandbox-environments)：比較內建沙箱與開發容器、容器和虛擬機
* [Security](/docs/zh-TW/security)：全面的安全功能和最佳實踐
* [Permissions](/docs/zh-TW/permissions)：權限配置和存取控制
* [Settings](/docs/zh-TW/settings)：完整配置參考
* [CLI reference](/docs/zh-TW/cli-reference)：命令列選項
