> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 進階設定

> Claude Code 的系統需求、平台特定安裝、版本管理和卸載。

本頁涵蓋系統需求、平台特定安裝詳情、更新和卸載。如需首次會話的引導式逐步說明，請參閱[快速入門](/docs/zh-TW/quickstart)。如果您從未使用過終端機，請參閱[終端機指南](/docs/zh-TW/terminal-guide)。

<h2 id="system-requirements">
  系統需求
</h2>

Claude Code 在以下平台和配置上運行：

* **作業系統**：
  * macOS 13.0+
  * Windows 10 1809+ 或 Windows Server 2019+
  * Ubuntu 20.04+
  * Debian 10+
  * Alpine Linux 3.19+
* **硬體**：4 GB+ RAM、x64 或 ARM64 處理器
* **網路**：需要網際網路連線。請參閱[網路配置](/docs/zh-TW/network-config#network-access-requirements)。
* **Shell**：Bash、Zsh、PowerShell 或 CMD。
* **位置**：[Anthropic 支援的國家](https://www.anthropic.com/supported-countries)

<h3 id="additional-dependencies">
  其他依賴項
</h3>

* **ripgrep**：通常包含在 Claude Code 中。如果搜尋失敗，請參閱[搜尋疑難排解](/docs/zh-TW/troubleshooting#search-and-discovery-issues)。

<h2 id="install-claude-code">
  安裝 Claude Code
</h2>

<Tip>
  偏好圖形介面？[桌面應用程式](/docs/zh-TW/desktop-quickstart)讓您無需終端機即可使用 Claude Code。下載適用於 [macOS](https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code\&utm_medium=docs)、[Windows](https://claude.com/download?utm_source=claude_code\&utm_medium=docs) 或 [Linux](/docs/zh-TW/desktop-linux) 的版本。

  初次使用終端機？請參閱[終端機指南](/docs/zh-TW/terminal-guide)以取得逐步說明。
</Tip>

To install Claude Code, use one of the following methods:

<Tabs>
  <Tab title="Native Install (Recommended)">
    **macOS, Linux, WSL:**

    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash
    ```

    **Windows PowerShell:**

    ```powershell theme={null}
    irm https://claude.ai/install.ps1 | iex
    ```

    **Windows CMD:**

    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
    ```

    If you see `The token '&&' is not a valid statement separator`, you're in PowerShell, not CMD. If you see `'irm' is not recognized as an internal or external command`, you're in CMD, not PowerShell. Your prompt shows `PS C:\` when you're in PowerShell and `C:\` without the `PS` when you're in CMD.

    If the install command fails with `syntax error near unexpected token '<'`, a `403`, or another curl error, see [Troubleshoot installation](/docs/en/troubleshoot-install#find-your-error) to match the error to a fix and for alternative install methods.

    [Git for Windows](https://git-scm.com/downloads/win) is recommended on native Windows so Claude Code can use the Bash tool. If Git for Windows is not installed, Claude Code uses PowerShell as the shell tool instead. WSL setups do not need Git for Windows.

    <Info>
      Native installations automatically update in the background to keep you on the latest version.
    </Info>
  </Tab>

  <Tab title="Homebrew">
    ```bash theme={null}
    brew install --cask claude-code
    ```

    Homebrew offers two casks. `claude-code` tracks the stable release channel, which is typically about a week behind and skips releases with major regressions. `claude-code@latest` tracks the latest channel and receives new versions as soon as they ship.

    <Info>
      Homebrew installations do not auto-update. Run `brew upgrade claude-code` or `brew upgrade claude-code@latest`, depending on which cask you installed, to get the latest features and security fixes.
    </Info>
  </Tab>

  <Tab title="WinGet">
    ```powershell theme={null}
    winget install Anthropic.ClaudeCode
    ```

    <Info>
      WinGet installations do not auto-update. Run `winget upgrade Anthropic.ClaudeCode` periodically to get the latest features and security fixes.
    </Info>
  </Tab>
</Tabs>

You can also install with [apt, dnf, or apk](/docs/en/setup#install-with-linux-package-managers) on Debian, Fedora, RHEL, and Alpine.

安裝完成後，在您要使用的專案中開啟終端機並啟動 Claude Code：

```bash theme={null}
claude
```

如果您在安裝期間遇到任何問題，請參閱[疑難排解安裝和登入](/docs/zh-TW/troubleshoot-install)。

<h3 id="set-up-on-windows">
  在 Windows 上設定
</h3>

您可以在 Windows 上原生執行 Claude Code 或在 WSL 內執行。根據您的專案位置和所需功能進行選擇：

| 選項         | 需要                                                         | [沙箱](/docs/zh-TW/sandboxing) | 何時使用              |
| ---------- | ---------------------------------------------------------- | ----------------------- | ----------------- |
| 原生 Windows | 無；[Git for Windows](https://git-scm.com/downloads/win) 為選用 | 不支援                     | Windows 原生專案和工具   |
| WSL 2      | WSL 2 已啟用                                                  | 支援                      | Linux 工具鏈或沙箱化命令執行 |
| WSL 1      | WSL 1 已啟用                                                  | 不支援                     | 如果 WSL 2 無法使用     |

**選項 1：原生 Windows**

從 PowerShell 或 CMD 執行安裝命令。您不需要以系統管理員身分執行。安裝 [Git for Windows](https://git-scm.com/downloads/win) 為選用。它透過提供 Git Bash 來啟用 [Bash 工具](/docs/zh-TW/tools-reference#bash-tool-behavior)。

無論您從 PowerShell 還是 CMD 安裝，只會影響您執行的安裝命令。您的提示在 PowerShell 中顯示 `PS C:\Users\YourName>`，在 CMD 中顯示 `C:\Users\YourName>`（沒有 `PS`）。如果您是終端機新手，[終端機指南](/docs/zh-TW/terminal-guide#windows)會逐步說明每個步驟。

安裝後，從任何終端機啟動 `claude`。

* **沒有 Git for Windows**，Claude Code 透過 [PowerShell 工具](/docs/zh-TW/tools-reference#powershell-tool)執行 shell 命令。
* **有 Git for Windows**，Claude Code 使用 Git Bash 來執行 [Bash 工具](/docs/zh-TW/tools-reference#bash-tool-behavior)。如果 Claude Code 找不到 Git Bash，請在您的 [settings.json 檔案](/docs/zh-TW/settings)中設定路徑：

  ```json theme={null}
  {
    "env": {
      "CLAUDE_CODE_GIT_BASH_PATH": "C:\\Program Files\\Git\\bin\\bash.exe"
    }
  }
  ```

安裝 Git for Windows 時，PowerShell 工具正在逐步推出作為 Bash 的額外選項。設定 `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` 以選擇加入或 `0` 以選擇退出。請參閱 [PowerShell tool](/docs/zh-TW/tools-reference#powershell-tool) 以了解設定和限制。

**選項 2：WSL**

開啟您的 WSL 發行版本並從上面的[安裝說明](#install-claude-code)執行 Linux 安裝程式。您在 WSL 終端機內安裝和啟動 `claude`，而不是從 PowerShell 或 CMD。

<h3 id="alpine-linux-and-musl-based-distributions">
  Alpine Linux 和 musl 型發行版
</h3>

Alpine 和其他 musl/uClibc 型發行版上的原生安裝程式需要 `libgcc`、`libstdc++` 和 `ripgrep`。使用您的發行版套件管理員安裝這些，然後設定 `USE_BUILTIN_RIPGREP=0`。

此範例在 Alpine 上安裝所需的套件：

```bash theme={null}
apk add libgcc libstdc++ ripgrep
```

然後在您的 [`settings.json`](/docs/zh-TW/settings#available-settings) 檔案中將 `USE_BUILTIN_RIPGREP` 設定為 `0`：

```json theme={null}
{
  "env": {
    "USE_BUILTIN_RIPGREP": "0"
  }
}
```

<h2 id="verify-your-installation">
  驗證您的安裝
</h2>

安裝後，確認 Claude Code 正常運作：

```bash theme={null}
claude --version
```

如果此命令失敗並出現 `command not found` 或其他錯誤，請參閱[疑難排解安裝和登入](/docs/zh-TW/troubleshoot-install)。

如需更詳細的安裝和配置檢查，請執行 [`claude doctor`](/docs/zh-TW/troubleshooting#get-more-help)：

```bash theme={null}
claude doctor
```

<h2 id="authenticate">
  驗證身份
</h2>

Claude Code 需要 Pro、Max、Team、Enterprise 或 Console 帳戶。免費的 Claude.ai 方案不包括 Claude Code 存取權。您也可以透過第三方 API 提供者（如 [Amazon Bedrock](/docs/zh-TW/amazon-bedrock)、[Google Cloud's Agent Platform](/docs/zh-TW/google-vertex-ai) 或 [Microsoft Foundry](/docs/zh-TW/microsoft-foundry)）使用 Claude Code。

安裝後，執行 `claude` 並按照瀏覽器提示登入。請參閱[驗證](/docs/zh-TW/authentication)以了解所有帳戶類型和團隊設定選項。

<h2 id="update-claude-code">
  更新 Claude Code
</h2>

原生安裝會在背景自動更新。您可以[配置發行版本通道](#configure-release-channel)來控制您是立即接收更新還是按延遲穩定時間表接收，或[完全停用自動更新](#disable-auto-updates)。Homebrew、WinGet 和[Linux 套件管理員](#install-with-linux-package-managers)安裝預設需要手動更新。

<h3 id="auto-updates">
  自動更新
</h3>

Claude Code 在啟動時和執行期間定期檢查更新。更新會在背景下載和安裝，然後在您下次啟動 Claude Code 時生效。

執行 `claude doctor` 以查看最近更新嘗試的結果。

在 macOS 和 Linux 上，原生安裝程式會將啟動器管理為 `~/.local/bin/claude` 的符號連結，指向 `~/.local/share/claude/versions/`。如果您將該啟動器替換為自己的指令碼或符號連結，自動更新和 `claude update` 會將其保留在原位：新版本仍會安裝在 `versions/` 目錄下，而您的啟動器決定執行哪個版本。在 v2.1.207 之前，自動更新程式會在每次更新時將該路徑上的自訂啟動器替換為自己的符號連結。

使用自訂啟動器時，Claude Code 也會將每個已安裝的版本保留在磁碟上，因為它無法判斷啟動器需要哪個版本。`claude doctor` 會報告原生安裝程式未建立的啟動器。

若要讓 Claude Code 再次管理啟動器，請移除 `~/.local/bin/claude` 並執行 `claude update`。

如果 npm 全域安裝因為 npm 全域目錄不可寫而無法自動更新，Claude Code 會在啟動時顯示一次性通知，而 `claude doctor` 會列出可用的修復。詳見[安裝期間的權限錯誤](/docs/zh-TW/troubleshoot-install#permission-errors-during-installation)。

<Note>
  Homebrew、WinGet、apt、dnf 和 apk 安裝預設不會自動更新；請參閱下方以選擇加入 Homebrew 和 WinGet。若要手動升級 Homebrew，請執行 `brew upgrade claude-code` 或 `brew upgrade claude-code@latest`，取決於您安裝的 cask。對於 WinGet，請執行 `winget upgrade Anthropic.ClaudeCode`。對於 Linux 套件管理員，請參閱[使用 Linux 套件管理員安裝](#install-with-linux-package-managers)中的升級命令。

  若要讓 Claude Code 在 Homebrew 或 WinGet 上為您執行升級命令，請將 [`CLAUDE_CODE_PACKAGE_MANAGER_AUTO_UPDATE`](/docs/zh-TW/env-vars) 設定為 `1`。Claude Code 會在有新版本可用時在背景執行升級，並在成功時顯示重新啟動提示。升級僅針對 Claude Code 套件，不會影響您已安裝的其他軟體。

  在 WinGet 上，當 Claude Code 執行時升級可能會失敗，因為 Windows 會鎖定可執行檔。在這種情況下，Claude Code 會改為顯示手動命令。apt、dnf 和 apk 繼續需要手動升級，因為這些命令需要提升的權限。

  **已知問題**：Claude Code 可能會在新版本在這些套件管理員中可用之前通知您有更新。如果升級失敗，請稍候並稍後重試。

  Homebrew 在升級後會將舊版本保留在磁碟上。定期執行 `brew cleanup` 以回收磁碟空間。
</Note>

<h3 id="configure-release-channel">
  配置發行版本通道
</h3>

使用 `autoUpdatesChannel` 設定控制 Claude Code 為自動更新和 `claude update` 遵循的發行版本通道：

* `"latest"`，預設值：在新功能發佈時立即接收
* `"stable"`：使用通常約一週舊的版本，跳過有重大迴歸的發佈

透過 `/config` → **自動更新通道**配置此項，或將其新增到您的 [settings.json 檔案](/docs/zh-TW/settings)：

```json theme={null}
{
  "autoUpdatesChannel": "stable"
}
```

對於企業部署，您可以使用[受管設定](/docs/zh-TW/permissions#managed-settings)在整個組織中強制執行一致的發行版本通道。

Homebrew 安裝根據 cask 名稱而不是此設定選擇通道：`claude-code` 追蹤穩定版本，`claude-code@latest` 追蹤最新版本。

<h3 id="pin-a-minimum-version">
  固定最低版本
</h3>

`minimumVersion` 設定建立一個下限。背景自動更新和 `claude update` 拒絕安裝低於此值的任何版本，因此如果您已經在較新的 `"latest"` 組建上，移至 `"stable"` 通道不會降級您。

透過 `/config` 從 `"latest"` 切換到 `"stable"` 會提示您保留目前版本或允許降級。選擇保留會將 `minimumVersion` 設定為該版本。切換回 `"latest"` 會清除它。

將其新增到您的 [settings.json 檔案](/docs/zh-TW/settings)以明確固定下限：

```json theme={null}
{
  "autoUpdatesChannel": "stable",
  "minimumVersion": "2.1.100"
}
```

在[受管設定](/docs/zh-TW/permissions#managed-settings)中，這會強制執行使用者和專案設定無法覆蓋的組織範圍最低版本。

`minimumVersion` 固定只會限制更新。若要讓 Claude Code 拒絕在版本範圍外啟動，請改為使用受管設定 `requiredMinimumVersion` 和 `requiredMaximumVersion`。更新也會遵守 `requiredMaximumVersion` 上限。請參閱[可用設定](/docs/zh-TW/settings#available-settings)。

<h3 id="disable-auto-updates">
  停用自動更新
</h3>

在您的 [`settings.json`](/docs/zh-TW/settings#available-settings) 檔案的 `env` 鍵中將 `DISABLE_AUTOUPDATER` 設定為 `"1"`：

```json theme={null}
{
  "env": {
    "DISABLE_AUTOUPDATER": "1"
  }
}
```

`DISABLE_AUTOUPDATER` 只會停止背景檢查；`claude update` 和 `claude install` 仍然有效。若要阻止所有更新路徑（包括手動更新），請改為設定 [`DISABLE_UPDATES`](/docs/zh-TW/env-vars)。當您透過自己的通道發佈 Claude Code 並需要使用者保持在您提供的版本上時，請使用此選項。

<h3 id="update-manually">
  手動更新
</h3>

若要立即套用更新而不等待下一次背景檢查，請執行：

```bash theme={null}
claude update
```

<h2 id="advanced-installation-options">
  進階安裝選項
</h2>

這些選項適用於版本固定、Linux 套件管理員、npm 和驗證二進位檔案完整性。

<h3 id="install-a-specific-version">
  安裝特定版本
</h3>

原生安裝程式接受特定版本號或發行版本通道（`latest` 或 `stable`）。您在安裝時選擇的通道將成為自動更新的預設值。請參閱[配置發行版本通道](#configure-release-channel)以取得更多資訊。

若要安裝最新版本（預設）：

<Tabs>
  <Tab title="macOS、Linux、WSL">
    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    irm https://claude.ai/install.ps1 | iex
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
    ```
  </Tab>
</Tabs>

若要安裝穩定版本：

<Tabs>
  <Tab title="macOS、Linux、WSL">
    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash -s stable
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    & ([scriptblock]::Create((irm https://claude.ai/install.ps1))) stable
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd stable && del install.cmd
    ```
  </Tab>
</Tabs>

若要安裝特定版本號：

<Tabs>
  <Tab title="macOS、Linux、WSL">
    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash -s 2.1.89
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    & ([scriptblock]::Create((irm https://claude.ai/install.ps1))) 2.1.89
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd 2.1.89 && del install.cmd
    ```
  </Tab>
</Tabs>

<h3 id="install-with-linux-package-managers">
  使用 Linux 套件管理員安裝
</h3>

Claude Code 發佈已簽署的 apt、dnf 和 apk 儲存庫。每個儲存庫提供兩個通道：`stable` 提供通常約一週前的版本，跳過有重大迴歸的發佈，而 `latest` 在每個發佈發行時立即提供。下面的命令配置 `stable` 通道，適合大多數使用者；每個標籤也顯示 `latest` 儲存庫 URL。套件管理員安裝不會透過 Claude Code 自動更新；更新會透過您的正常系統升級工作流程進行。

所有儲存庫都使用 [Claude Code 發佈簽署金鑰](#binary-integrity-and-code-signing)簽署。在信任金鑰之前，請按照每個標籤中的說明驗證它。

<Tabs>
  <Tab title="apt">
    適用於 Debian 和 Ubuntu。下列安裝命令使用 `curl` 下載簽署金鑰，新鮮的 Debian 和 Ubuntu 安裝可能不包含此命令。如果下載失敗並出現 `sudo: curl: command not found`，請先安裝 curl：

    ```bash theme={null}
    sudo apt install curl
    ```

    下列命令配置 `stable` 通道：

    ```bash theme={null}
    sudo install -d -m 0755 /etc/apt/keyrings
    sudo curl -fsSL https://downloads.claude.ai/keys/claude-code.asc \
      -o /etc/apt/keyrings/claude-code.asc
    echo "deb [signed-by=/etc/apt/keyrings/claude-code.asc] https://downloads.claude.ai/claude-code/apt/stable stable main" \
      | sudo tee /etc/apt/sources.list.d/claude-code.list
    sudo apt update
    sudo apt install claude-code
    ```

    若要改用 `latest` 通道，URL 路徑和套件組合名稱都會變更。使用此 `deb` 行：

    ```bash theme={null}
    echo "deb [signed-by=/etc/apt/keyrings/claude-code.asc] https://downloads.claude.ai/claude-code/apt/latest latest main" \
      | sudo tee /etc/apt/sources.list.d/claude-code.list
    ```

    在信任之前驗證 GPG 金鑰指紋：`gpg --show-keys /etc/apt/keyrings/claude-code.asc` 應該報告 `31DD DE24 DDFA B679 F42D 7BD2 BAA9 29FF 1A7E CACE`。

    若要稍後升級，請執行 `sudo apt update && sudo apt upgrade claude-code`。
  </Tab>

  <Tab title="dnf">
    適用於 Fedora 和 RHEL。下列命令配置 `stable` 通道：

    ```bash theme={null}
    sudo tee /etc/yum.repos.d/claude-code.repo <<'EOF'
    [claude-code]
    name=Claude Code
    baseurl=https://downloads.claude.ai/claude-code/rpm/stable
    enabled=1
    gpgcheck=1
    gpgkey=https://downloads.claude.ai/keys/claude-code.asc
    EOF
    sudo dnf install claude-code
    ```

    若要改用 `latest` 通道，將 `baseurl` 設定為 `latest` 儲存庫：

    ```ini theme={null}
    baseurl=https://downloads.claude.ai/claude-code/rpm/latest
    ```

    dnf 在首次安裝時下載金鑰，並提示您確認指紋。在接受之前驗證它是否與 `31DD DE24 DDFA B679 F42D 7BD2 BAA9 29FF 1A7E CACE` 相符。

    若要稍後升級，請執行 `sudo dnf upgrade claude-code`。
  </Tab>

  <Tab title="apk">
    適用於 Alpine Linux。下列命令配置 `stable` 通道：

    ```sh theme={null}
    wget -O /etc/apk/keys/claude-code.rsa.pub \
      https://downloads.claude.ai/keys/claude-code.rsa.pub
    echo "https://downloads.claude.ai/claude-code/apk/stable" >> /etc/apk/repositories
    apk add claude-code
    ```

    若要切換至 `latest` 通道，移除 `stable` 儲存庫行並新增 `latest` 儲存庫：

    ```sh theme={null}
    sed -i '\|downloads.claude.ai/claude-code/apk/stable|d' /etc/apk/repositories
    echo "https://downloads.claude.ai/claude-code/apk/latest" >> /etc/apk/repositories
    ```

    使用 `sha256sum /etc/apk/keys/claude-code.rsa.pub` 驗證下載的金鑰，應該報告 `395759c1f7449ef4cdef305a42e820f3c766d6090d142634ebdb049f113168b6`。

    若要稍後升級，請執行 `apk update && apk upgrade claude-code`。
  </Tab>
</Tabs>

<h3 id="install-with-npm">
  使用 npm 安裝
</h3>

您也可以將 Claude Code 安裝為全域 npm 套件。自 v2.1.198 起，npm 套件需要 [Node.js 22 或更新版本](https://nodejs.org/en/download)。在較舊的 Node.js 版本上，npm 在安裝期間列印 `EBADENGINE` 警告而不是失敗；安裝完成且 `claude` 仍然執行，因為該套件下載不使用您的 Node.js 的原生二進位檔案。

```bash theme={null}
npm install -g @anthropic-ai/claude-code
```

npm 套件安裝與獨立安裝程式相同的原生二進位檔案。npm 透過每個平台的選擇性依賴項（例如 `@anthropic-ai/claude-code-darwin-arm64`）提取二進位檔案，並透過 postinstall 步驟將其連結到位。已安裝的 `claude` 二進位檔案本身不會呼叫 Node。

支援的 npm 安裝平台為 `darwin-arm64`、`darwin-x64`、`linux-x64`、`linux-arm64`、`linux-x64-musl`、`linux-arm64-musl`、`win32-x64` 和 `win32-arm64`。您的套件管理員必須允許選擇性依賴項。如果安裝後二進位檔案遺失，請參閱[疑難排解](/docs/zh-TW/troubleshoot-install#native-binary-not-found-after-npm-install)。

若要升級 npm 安裝，請執行 `npm install -g @anthropic-ai/claude-code@latest`。避免使用 `npm update -g`，因為它會遵守原始安裝的 semver 範圍，可能無法將您移至最新版本。

<Warning>
  請勿使用 `sudo npm install -g`，因為這可能導致權限問題和安全風險。如果您遇到權限錯誤，請參閱[疑難排解權限錯誤](/docs/zh-TW/troubleshoot-install#permission-errors-during-installation)。
</Warning>

<h3 id="binary-integrity-and-code-signing">
  二進位檔案完整性和程式碼簽署
</h3>

每個發佈都會發佈一個 `manifest.json`，其中包含每個平台二進位檔案的 SHA256 校驗和。該資訊清單使用 Anthropic GPG 金鑰簽署，因此驗證資訊清單上的簽名可以傳遞地驗證它列出的每個二進位檔案。

<h4 id="verify-the-manifest-signature">
  驗證資訊清單簽名
</h4>

步驟 1-3 需要具有 `gpg` 和 `curl` 的 POSIX shell。在 Windows 上，在 Git Bash 或 WSL 中執行它們。步驟 4 包括 PowerShell 選項。

<Steps>
  <Step title="下載並匯入公開金鑰">
    發佈簽署金鑰發佈在固定 URL。

    ```bash theme={null}
    curl -fsSL https://downloads.claude.ai/keys/claude-code.asc | gpg --import
    ```

    顯示匯入金鑰的指紋。

    ```bash theme={null}
    gpg --fingerprint security@anthropic.com
    ```

    確認輸出包含此指紋：

    ```text theme={null}
    31DD DE24 DDFA B679 F42D  7BD2 BAA9 29FF 1A7E CACE
    ```
  </Step>

  <Step title="下載資訊清單和簽名">
    將 `VERSION` 設定為您要驗證的發佈。

    ```bash theme={null}
    REPO=https://downloads.claude.ai/claude-code-releases
    VERSION=2.1.89
    curl -fsSLO "$REPO/$VERSION/manifest.json"
    curl -fsSLO "$REPO/$VERSION/manifest.json.sig"
    ```
  </Step>

  <Step title="驗證簽名">
    驗證分離的簽名對比資訊清單。

    ```bash theme={null}
    gpg --verify manifest.json.sig manifest.json
    ```

    有效的結果報告 `Good signature from "Anthropic Claude Code Release Signing <security@anthropic.com>"`。

    `gpg` 也會為任何新匯入的金鑰列印 `WARNING: This key is not certified with a trusted signature!`。這是預期的。`Good signature` 行確認密碼檢查已通過。第 1 步中的指紋比較確認金鑰本身是真實的。
  </Step>

  <Step title="根據資訊清單檢查二進位檔案">
    將您下載的二進位檔案的 SHA256 校驗和與 `manifest.json` 中 `platforms.<platform>.checksum` 下列出的值進行比較。下列命令假設目前目錄中有 `claude` 二進位檔案。若要改為驗證已安裝的原生二進位檔案，請針對 `~/.local/share/claude/versions/VERSION` 執行命令，將 VERSION 替換為您在步驟 2 中設定的發佈。

    <Tabs>
      <Tab title="Linux">
        ```bash theme={null}
        sha256sum claude
        ```
      </Tab>

      <Tab title="macOS">
        ```bash theme={null}
        shasum -a 256 claude
        ```
      </Tab>

      <Tab title="Windows PowerShell">
        ```powershell theme={null}
        (Get-FileHash claude.exe -Algorithm SHA256).Hash.ToLower()
        ```
      </Tab>
    </Tabs>
  </Step>
</Steps>

<Note>
  資訊清單簽名適用於 `2.1.89` 及以後的發佈。較早的發佈在 `manifest.json` 中發佈校驗和，但沒有分離的簽名。
</Note>

<h4 id="platform-code-signatures">
  平台程式碼簽名
</h4>

除了簽署的資訊清單外，個別二進位檔案在支援的地方還帶有平台原生程式碼簽名。

* **macOS**：由「Anthropic PBC」簽署並由 Apple 公證。使用 `codesign --verify --verbose ./claude` 驗證。
* **Windows**：由「Anthropic, PBC」簽署。使用 `Get-AuthenticodeSignature .\claude.exe` 驗證。
* **Linux**：二進位檔案不是單獨程式碼簽署的。如果您直接從 `claude-code-releases` 儲存庫下載或使用原生安裝程式，請使用上面的資訊清單簽名驗證完整性。如果您使用 [apt、dnf 或 apk](#install-with-linux-package-managers) 安裝，您的套件管理員會使用儲存庫簽署金鑰自動驗證簽名。

<h2 id="uninstall-claude-code">
  卸載 Claude Code
</h2>

若要移除 Claude Code，請按照您的安裝方法的說明進行。如果之後 `claude` 仍然執行，您可能有第二個安裝或來自舊版安裝程式的遺留 shell 別名。請參閱[檢查衝突的安裝](/docs/zh-TW/troubleshoot-install#check-for-conflicting-installations)以找到並移除它。

<h3 id="native-installation">
  原生安裝
</h3>

移除 Claude Code 二進位檔案和版本檔案：

<Tabs>
  <Tab title="macOS、Linux、WSL">
    ```bash theme={null}
    rm -f ~/.local/bin/claude
    rm -rf ~/.local/share/claude
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    Remove-Item -Path "$env:USERPROFILE\.local\bin\claude.exe" -Force
    Remove-Item -Path "$env:USERPROFILE\.local\share\claude" -Recurse -Force
    ```
  </Tab>
</Tabs>

<h3 id="homebrew-installation">
  Homebrew 安裝
</h3>

移除您安裝的 Homebrew cask。如果您安裝了穩定版 cask：

```bash theme={null}
brew uninstall --cask claude-code
```

如果您安裝了最新版 cask：

```bash theme={null}
brew uninstall --cask claude-code@latest
```

<h3 id="winget-installation">
  WinGet 安裝
</h3>

移除 WinGet 套件：

```powershell theme={null}
winget uninstall Anthropic.ClaudeCode
```

<h3 id="apt-/-dnf-/-apk">
  apt / dnf / apk
</h3>

移除套件和儲存庫配置：

<Tabs>
  <Tab title="apt">
    ```bash theme={null}
    sudo apt remove claude-code
    sudo rm /etc/apt/sources.list.d/claude-code.list /etc/apt/keyrings/claude-code.asc
    ```
  </Tab>

  <Tab title="dnf">
    ```bash theme={null}
    sudo dnf remove claude-code
    sudo rm /etc/yum.repos.d/claude-code.repo
    ```
  </Tab>

  <Tab title="apk">
    ```sh theme={null}
    apk del claude-code
    sed -i '\|downloads.claude.ai/claude-code/apk|d' /etc/apk/repositories
    rm /etc/apk/keys/claude-code.rsa.pub
    ```
  </Tab>
</Tabs>

<h3 id="npm">
  npm
</h3>

移除全域 npm 套件：

```bash theme={null}
npm uninstall -g @anthropic-ai/claude-code
```

<h3 id="remove-configuration-files">
  移除配置檔案
</h3>

<Warning>
  移除配置檔案將刪除您的所有設定、允許的工具、MCP 伺服器配置和會話歷史記錄。
</Warning>

VS Code 擴充功能、JetBrains 外掛程式和桌面應用程式也會寫入 `~/.claude/`。如果其中任何一個仍然安裝，下次執行時目錄會被重新建立。若要完全移除 Claude Code，請在刪除這些檔案之前卸載 [VS Code 擴充功能](/docs/zh-TW/vs-code#uninstall-the-extension)、JetBrains 外掛程式和桌面應用程式。

若要移除 Claude Code 設定和快取資料：

<Tabs>
  <Tab title="macOS、Linux、WSL">
    ```bash theme={null}
    # 移除使用者設定和狀態
    rm -rf ~/.claude
    rm ~/.claude.json

    # 移除專案特定設定（從您的專案目錄執行）
    rm -rf .claude
    rm -f .mcp.json
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    # 移除使用者設定和狀態
    Remove-Item -Path "$env:USERPROFILE\.claude" -Recurse -Force
    Remove-Item -Path "$env:USERPROFILE\.claude.json" -Force

    # 移除專案特定設定（從您的專案目錄執行）
    Remove-Item -Path ".claude" -Recurse -Force
    Remove-Item -Path ".mcp.json" -Force
    ```
  </Tab>
</Tabs>
