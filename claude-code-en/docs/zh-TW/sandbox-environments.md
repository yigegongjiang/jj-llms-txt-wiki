> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 選擇沙箱環境

> 比較 Claude Code 沙箱選項：內建的沙箱化 Bash 工具、sandbox runtime、dev containers、Docker 和虛擬機。為您的威脅模型選擇適當的隔離。

隔離 Claude Code 會限制一個會話可以讀取、寫入和在網路上存取的內容。當您讓 Claude 在較少的權限提示下工作、無人值守地執行它，或將其指向您不完全信任的程式碼時，這一點最為重要。

Claude Code 可以在多種隔離環境中執行，範圍從輕量級的每個命令沙箱到完全獨立的虛擬機。本頁涵蓋如何比較它們隔離的內容和所需條件，幫助您為威脅模型選擇一個，並展示如何在整個組織中強制執行該選擇。

<Info>
  有關更廣泛的安全模型，請參閱 [Security](/docs/zh-TW/security)。有關 Agent SDK 部署，請參閱 [Secure deployment](/docs/zh-TW/agent-sdk/secure-deployment)。
</Info>

<h2 id="compare-sandboxing-approaches">
  比較沙箱方法
</h2>

下表中的前兩種方法在主機作業系統上執行，不使用容器。其餘的方法將 Claude Code 放在容器或虛擬機內。

| 方法                                                | 隔離的內容                                   | 需要 Docker | 設定工作量                      |
| :------------------------------------------------ | :-------------------------------------- | :-------- | :------------------------- |
| [Sandboxed Bash tool](#sandboxed-bash-tool)       | Bash 命令及其子進程                            | 否         | macOS 上最少；Linux 和 WSL2 上較少 |
| [Sandbox runtime](#sandbox-runtime)               | 整個 Claude Code 進程，包括檔案工具、MCP 伺服器和 hooks | 否         | 較少                         |
| [Dev container](#dev-containers)                  | 完整開發環境                                  | 是         | 中等                         |
| [Custom container](#custom-container)             | 完整開發環境                                  | 是         | 中等到高                       |
| [Virtual machine](#virtual-machine)               | 完整作業系統                                  | 否         | 高                          |
| [Claude Code on the web](#claude-code-on-the-web) | 完整作業系統，由 Anthropic 託管                   | 否         | 無；需要 Claude 訂閱和 GitHub     |

[Sandboxed Bash tool](/docs/zh-TW/sandboxing) 內建於 Claude Code 中，僅限制 Bash 命令。內建檔案工具、MCP 伺服器和 hooks 仍直接在您的主機上執行。表中的所有其他方法都將整個 Claude Code 進程放在隔離邊界內，因此檔案工具、MCP 伺服器和 hooks 也受到限制。

<Warning>
  沙箱隔離可減少違規的影響，但不能消除風險。任何允許網路出站的方法仍然可能洩露代理可以讀取的資料，任何以可寫方式掛載您的專案目錄的方法仍然可以修改該程式碼。在依賴沙箱作為硬控制之前，請查看 [security limitations](/docs/zh-TW/sandboxing#security-limitations)。

  隔離也不會改變發送到模型的內容。您的提示和 Claude 讀取的檔案無論是否使用沙箱，都會傳輸到 Anthropic API 或您配置的提供者。有關 Claude Code 發送的內容以及如何減少它，請參閱 [Data usage](/docs/zh-TW/data-usage)。
</Warning>

<h2 id="choose-an-approach">
  選擇一種方法
</h2>

將您的目標與下面的一行相匹配，然後閱讀隨後的詳細部分。

| 您想要                                                       | 開始使用                                                                                            |
| :-------------------------------------------------------- | :---------------------------------------------------------------------------------------------- |
| 在您自己的機器上減少日常工作中的權限提示                                      | [sandboxed Bash tool](/docs/zh-TW/sandboxing)，使用 `/sandbox` 啟用                                       |
| 讓 Claude 使用 `--dangerously-skip-permissions` 或自動模式無人值守地工作 | 預配置的 [dev container](/docs/zh-TW/devcontainer)、任何容器或虛擬機，或 [sandbox runtime](#sandbox-runtime)        |
| 隔離 MCP 伺服器和 hooks 以及 Bash，不使用 Docker                      | sandbox runtime                                                                                 |
| 在不受信任的儲存庫上工作                                              | 專用虛擬機，或如果您有 Claude 訂閱和連接的 GitHub 帳戶，則使用 [Claude Code on the web](/docs/zh-TW/claude-code-on-the-web) |
| 在整個團隊中標準化沙箱環境                                             | 預配置的 [dev container](/docs/zh-TW/devcontainer)，複製到您的儲存庫中                                             |
| 從沒有本地設定的設備使用 Claude Code                                  | [Claude Code on the web](/docs/zh-TW/claude-code-on-the-web)，需要 Claude 訂閱和連接的 GitHub 帳戶              |
| 要求為組織中的每個開發人員強制執行隔離                                       | [在整個組織中強制執行隔離](#enforce-isolation-across-an-organization)                                       |
| 在原生 Windows 主機上工作                                         | 容器或虛擬機，或在 WSL2 內執行 Bash 沙箱                                                                      |

<h3 id="how-isolation-relates-to-permission-modes">
  隔離與權限模式的關係
</h3>

[Permission modes](/docs/zh-TW/permission-modes) 決定工具呼叫是否執行以及是否首先提示您。隔離限制命令執行後可以存取的內容。兩者協同工作：當權限模式允許操作在不詢問您的情況下執行時，隔離邊界限制這些操作可以到達的內容。

當您傳遞 `--dangerously-skip-permissions` 時，Claude 會在不先詢問您的情況下採取行動；您只會被提示明確的 [ask rules](/docs/zh-TW/permissions#manage-permissions)、連接器工具 [您的組織設定為 `ask`](/docs/zh-TW/mcp#organization-controls-on-connector-tools)、標記為 [`requiresUserInteraction`](/docs/zh-TW/mcp#require-approval-for-a-specific-tool) 的 MCP 工具，以及針對 `/` 或您的主目錄的移除。沒有提示來捕捉錯誤，您選擇的隔離邊界是保護您系統的東西。始終在容器、虛擬機或 [sandbox runtime](#sandbox-runtime) 內執行 `--dangerously-skip-permissions` 工作階段，以便檔案工具、MCP 伺服器和 hooks 也在邊界內。

[Auto mode](/docs/zh-TW/permission-modes#eliminate-prompts-with-auto-mode) 用分類器替換提示，該分類器審查操作並阻止超出請求範圍、針對無法識別的基礎設施或似乎由 Claude 讀取的敵對內容驅動的操作。分類器是每個操作的控制，而不是隔離邊界，因此隔離邊界仍然為無人值守的執行添加深度防禦，並且不像 `--dangerously-skip-permissions` 那樣是必需的。

[sandboxed Bash tool](#sandboxed-bash-tool) 本身只限制 Bash，因此對於任一模式中的完全無人值守執行都不夠。您可以分層方法：在容器或虛擬機內執行沙箱化 Bash 工具可在外部環境邊界之上為您提供作業系統級命令限制。有關 Bash 沙箱本身如何與權限規則和模式交互的信息，請參閱 [How sandboxing relates to permissions and permission modes](/docs/zh-TW/sandboxing#how-sandboxing-relates-to-permissions-and-permission-modes)。

<h2 id="sandboxed-bash-tool">
  Sandboxed Bash tool
</h2>

<Note>
  此選項不支援原生 Windows。在 Windows 主機上，使用 WSL2 或下面的容器或虛擬機方法之一。
</Note>

Sandboxed Bash tool 內建於 Claude Code 中。它使用作業系統原語來限制 Claude 執行的每個 Bash 命令的檔案系統和網路存取：Seatbelt（內建的 macOS 沙箱）和 Linux 和 WSL2 上的 [bubblewrap](https://github.com/containers/bubblewrap)。預設情況下，它允許寫入工作目錄，並在命令首次需要新網路域時提示。

使用 `/sandbox` 命令啟用它。[Sandboxing](/docs/zh-TW/sandboxing) 指南涵蓋批准模式、預設邊界以及如何擴大或縮小它。

每個命令沙箱不涵蓋在會話中執行的所有內容：

* 其他 [built-in tools](/docs/zh-TW/tools-reference)（如 Read、Edit 和 WebFetch）在 Claude Code 進程內執行，不會生成任意程式碼。[Permission rules](/docs/zh-TW/permissions) 用於路徑或域來控制它們。
* [MCP](/docs/zh-TW/mcp) 伺服器和 hooks 是在主機上無約束執行的獨立進程。

要將內建工具、MCP 伺服器和 hooks 全部放在一個作業系統邊界後面，請在 [sandbox runtime](#sandbox-runtime)、[dev container](#dev-containers) 或 [custom container](#custom-container) 內執行整個 Claude Code 進程。

<h2 id="sandbox-runtime">
  Sandbox runtime
</h2>

[`@anthropic-ai/sandbox-runtime`](https://github.com/anthropic-experimental/sandbox-runtime) 套件將整個進程包裝在內建 Bash 沙箱使用的相同 Seatbelt 或 bubblewrap 隔離中。通過它執行 Claude Code 會限制會話中的每個工具、hook 和 MCP 伺服器，而不僅僅是 Bash。該 runtime 是測試版研究預覽，其配置格式可能會隨著套件的發展而改變。

該 runtime 預設拒絕所有寫入和網路存取，因此在通過它啟動 Claude Code 之前配置它。在 `~/.srt-settings.json` 中，或您使用 `--settings` 傳遞的檔案中，至少允許寫入您的專案目錄和 Claude Code 的配置路徑 `~/.claude` 和 `~/.claude.json`。允許您的會話需要的網路域，包括 `api.anthropic.com` 或您配置的提供者的端點。有關完整的配置架構，請參閱套件 [README](https://github.com/anthropic-experimental/sandbox-runtime)。

設定檔就位後，使用 `npx` 啟動 Claude Code 並傳遞 `claude` 作為要包裝的命令：

```bash theme={null}
npx @anthropic-ai/sandbox-runtime claude
```

Claude Code 在沙箱內啟動，具有您配置的檔案系統和網路邊界。相同的命令適用於沙箱化獨立 MCP 伺服器或其他輔助進程。

<h2 id="dev-containers">
  Dev containers
</h2>

Dev container 在 VS Code 或相容編輯器管理的 Docker 容器內執行 Claude Code，您的專案掛載在其中。您可以在您的儲存庫中使用 `.devcontainer/` 目錄定義您自己的。

claude-code 儲存庫發佈了一個 [example dev container](/docs/zh-TW/devcontainer)，其中包含預設拒絕 iptables 防火牆作為起點。將其複製到您的儲存庫中，並調整防火牆允許清單、基礎映像和固定的 Claude Code 版本以適應您的環境。因為防火牆阻止未批准的出站流量，像這樣的配置支援使用 `--dangerously-skip-permissions` 執行 Claude Code 進行無人值守工作。

<h2 id="custom-container">
  Custom container
</h2>

您可以在任何 Docker 或 OCI 容器映像中執行 Claude Code，具有您自己的網路策略、掛載的卷和 seccomp 設定檔。這是具有現有容器基礎設施或 CI 執行器的組織最常見的路徑。

多個託管沙箱和遠端執行服務可以為您託管容器。與您操作的任何容器相同的檢查清單適用：審查掛載為可寫的內容、容器內可到達的認證和令牌，以及網路出站策略允許的內容。

您可以在容器內分層內建 Bash 沙箱以進行每個命令的限制。無特權容器需要 [Sandboxing troubleshooting](/docs/zh-TW/sandboxing#troubleshooting) 中描述的嵌套沙箱設定。

<h2 id="virtual-machine">
  Virtual machine
</h2>

專用虛擬機提供最強的分離，具有自己的核心，在雲或 microVM 部署中，具有自己的虛擬化硬體。選項包括雲實例、本地虛擬機管理程式和 microVM（如 Firecracker）。

當您評估不受信任的程式碼、當您的安全策略要求代理和主機之間的核心級分離，或當沒有主機級方法滿足您的合規要求時，使用此方法。Docker Desktop 的 [sandboxes feature](https://docs.docker.com/ai/sandboxes/) 提供了一個具有自己的 Docker daemon 和工作區同步的 microVM，可以在已經有 Docker Desktop 的主機上執行 Claude Code。

<h2 id="claude-code-on-the-web">
  Claude Code on the web
</h2>

[Claude Code on the web](/docs/zh-TW/claude-code-on-the-web) 在隔離的、由 Anthropic 管理的虛擬機中執行每個會話。網路代理強制執行預設允許清單，單獨的代理在沙箱外保持您的 GitHub 令牌，同時在其內部為儲存庫存取發出範圍限定的認證。

當您想要完整的虛擬機隔離而無需自己配置基礎設施，或當您從沒有本地開發環境的設備委派任務時，使用此方法。它需要 Claude 訂閱和連接的 GitHub 帳戶，會話從 GitHub 複製您的儲存庫。有關計劃可用性和 GitHub 身份驗證選項，請參閱 [Claude Code on the web](/docs/zh-TW/claude-code-on-the-web)。

<h2 id="enforce-isolation-across-an-organization">
  在整個組織中強制執行隔離
</h2>

個別開發人員可以選擇上述任何方法。組織可以強制執行的內容以及使用哪些工具取決於方法：

* **Built-in Bash sandbox**：唯一 Claude Code 本身強制執行的方法。通過 [managed settings](/docs/zh-TW/settings#settings-files) 傳遞 `sandbox` 設定金鑰，可以是由您的 MDM 管理的檔案，也可以通過 Claude.ai 上的 [server-managed settings](/docs/zh-TW/server-managed-settings)。有關要部署的金鑰以及如何防止開發人員擴大策略，請參閱 [Enforce sandboxing with managed settings](/docs/zh-TW/sandboxing#enforce-sandboxing-with-managed-settings)。
* **Dev containers**：將 [example dev container](/docs/zh-TW/devcontainer) 提交到您的儲存庫以標準化整個團隊的環境。這是一個約定而不是強制邊界，因為 Claude Code 不需要容器。如果開發人員不應該能夠在其外部執行 Claude Code，請使用您組織的設備管理或軟體允許清單工具強制執行。
* **Custom containers and VMs**：通過批准的映像分發 Claude Code，並使用您組織的設備管理或軟體允許清單工具防止在其外部安裝。

<h2 id="see-also">
  另請參閱
</h2>

這些頁面涵蓋上述方法的配置和策略詳細資訊。

* [Sandboxing](/docs/zh-TW/sandboxing)：配置內建沙箱化 Bash 工具
* [Dev container](/docs/zh-TW/devcontainer)：預配置的 Docker 開發容器
* [Security](/docs/zh-TW/security)：完整的 Claude Code 安全模型
* [Secure deployment](/docs/zh-TW/agent-sdk/secure-deployment)：Agent SDK 應用程式的隔離指南
* [Settings](/docs/zh-TW/settings#sandbox-settings)：所有沙箱配置金鑰，包括託管設定傳遞
