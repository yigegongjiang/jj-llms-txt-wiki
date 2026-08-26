> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Linux 上的 Claude Desktop（測試版）

> 在 Ubuntu 和 Debian 上安裝和更新 Claude 桌面應用程式

<Note>
  Linux 上的 Claude 桌面應用程式處於測試版。Chat、Cowork 和 Code 標籤都可用。
</Note>

Linux 上的桌面應用程式提供與 macOS 和 Windows 相同的 Chat、Cowork 和 Claude Code 體驗：平行工作階段、視覺化差異檢視、整合式終端機和編輯器，以及即時應用程式預覽。請參閱[使用 Claude Code Desktop](/docs/zh-TW/desktop)以取得完整功能參考。

<h2 id="requirements">
  需求
</h2>

* Ubuntu 22.04 或更新版本，或 Debian 12 或更新版本
* x86\_64 或 arm64

其他符合這些需求的 Debian 衍生發行版本可能可以運作，但未經官方測試。

<h2 id="install">
  安裝
</h2>

從 Anthropic 的 apt 儲存庫安裝，以便更新透過您系統的定期套件更新到達。開啟終端機並執行每個步驟中的命令。

<Steps>
  <Step title="新增 Anthropic 的 apt 儲存庫">
    此步驟使用 `curl` 下載簽署金鑰，新鮮的 Debian 和 Ubuntu 安裝可能不包含此工具。如果下載命令失敗並顯示 `sudo: curl: command not found`，請先安裝 curl：

    ```bash theme={null}
    sudo apt install curl
    ```

    下載 Anthropic 的簽署金鑰：

    ```bash theme={null}
    sudo curl -fsSLo /usr/share/keyrings/claude-desktop-archive-keyring.asc https://downloads.claude.ai/claude-desktop/key.asc
    ```

    註冊儲存庫：

    ```bash theme={null}
    echo "deb [arch=amd64,arm64 signed-by=/usr/share/keyrings/claude-desktop-archive-keyring.asc] https://downloads.claude.ai/claude-desktop/apt/stable stable main" | sudo tee /etc/apt/sources.list.d/claude-desktop.list
    ```
  </Step>

  <Step title="安裝套件">
    ```bash theme={null}
    sudo apt update && sudo apt install claude-desktop
    ```
  </Step>

  <Step title="啟動並登入">
    從您的應用程式啟動器啟動 **Claude**，或從終端機執行 `claude-desktop`，然後使用您的 Anthropic 帳戶登入。

    Linux 應用程式的登入方式與 macOS 和 Windows 上相同：使用 claude.ai 訂閱，或透過您組織的 SSO。Desktop 不直接接受 Claude Console API 金鑰；請使用 [CLI](/docs/zh-TW/quickstart) 進行 API 金鑰驗證。對於將 Desktop 路由到 Google Cloud 的 Agent Platform 或 LLM 閘道的企業部署，請參閱 [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview) 和 [網路配置](/docs/zh-TW/network-config)。
  </Step>
</Steps>

<Accordion title="驗證簽署金鑰">
  您可以確認下載的簽署金鑰屬於 Anthropic：

  ```bash theme={null}
  gpg --show-keys /usr/share/keyrings/claude-desktop-archive-keyring.asc
  ```

  指紋應為 `31DD DE24 DDFA B679 F42D 7BD2 BAA9 29FF 1A7E CACE`。
</Accordion>

<h3 id="install-from-a-downloaded-file">
  從下載的檔案安裝
</h3>

如果您無法透過 apt 儲存庫安裝，請直接從儲存庫的套件池下載 `.deb` 套件。此命令在儲存庫索引中查詢您的架構的最新套件，然後將其下載到目前目錄：

```bash theme={null}
curl -fLO "https://downloads.claude.ai/claude-desktop/apt/stable/$(curl -s "https://downloads.claude.ai/claude-desktop/apt/stable/dists/stable/main/binary-$(dpkg --print-architecture)/Packages" | grep '^Filename: pool/main/c/claude-desktop/claude-desktop_' | sort -V | tail -n 1 | cut -d' ' -f2)"
```

如果命令失敗並顯示 `Remote file name has no length`，表示查詢未傳回套件路徑。這可能表示無法擷取儲存庫索引，例如當您的網路阻止 `downloads.claude.ai` 時，或該架構不存在套件。確認您的網路可以到達 `downloads.claude.ai`，且 `dpkg --print-architecture` 列印 `amd64` 或 `arm64`；儲存庫不會為其他架構發佈套件。

然後使用您的軟體安裝程式（例如 GNOME Software）開啟下載的檔案，或從包含下載檔案的目錄使用 apt 安裝它：

```bash theme={null}
sudo apt install ./claude-desktop_*.deb
```

如果 apt 報告 `E: Unsupported file ./claude-desktop_*.deb given on commandline`，表示該模式與目前目錄中的 `.deb` 檔案不符。確認下載已完成，然後從包含該檔案的目錄再次執行命令。

以這種方式安裝的 `.deb` 不會接收更新。若要透過 apt 取得更新，請從 [新增 Anthropic 的 apt 儲存庫](#install) 步驟註冊儲存庫。該套件也會將註解掉的儲存庫項目寫入 `/etc/apt/sources.list.d/claude-desktop.list`；取消註解其 `deb` 行等同於此。

<h2 id="update">
  更新
</h2>

桌面應用程式在 Linux 上不會自動更新。更新透過您系統的定期套件更新到達：

```bash theme={null}
sudo apt update && sudo apt upgrade
```

您的發行版本的圖形軟體更新程式也會取得新版本。

<h2 id="uninstall">
  解除安裝
</h2>

```bash theme={null}
sudo apt remove claude-desktop
```

這會移除簽署金鑰以及應用程式，因此如果您在安裝期間新增了儲存庫項目，也請移除它：

```bash theme={null}
sudo rm /etc/apt/sources.list.d/claude-desktop.list
```

<h2 id="troubleshoot">
  疑難排解
</h2>

<h3 id="unable-to-locate-package-claude-desktop">
  無法找到 claude-desktop 套件
</h3>

如果 `sudo apt install claude-desktop` 失敗並顯示 `E: Unable to locate package claude-desktop`，表示 apt 找不到您新增的儲存庫。請檢查以下項目：

* 確認儲存庫項目已寫入。`cat /etc/apt/sources.list.d/claude-desktop.list` 應該顯示來自[新增 Anthropic 的 apt 儲存庫](#install)步驟的 `deb` 行。如果檔案為空或遺失，請再次執行該步驟。
* 確認您的架構受支援。`dpkg --print-architecture` 應該列印 `amd64` 或 `arm64`。儲存庫不會為其他架構發佈套件。
* 再次執行 `sudo apt update` 並檢查其輸出中是否有與 `downloads.claude.ai` 相關的錯誤。該處的網路或金鑰錯誤表示儲存庫已新增但無法連線或驗證。

如果儲存庫已就位且可連線，但仍找不到套件，請改為[從下載的檔案安裝](#install-from-a-downloaded-file)。

<h2 id="what’s-not-in-the-linux-beta-yet">
  Linux 測試版中尚未提供的功能
</h2>

* **Computer Use**：[應用程式和螢幕控制](/docs/zh-TW/desktop#let-claude-use-your-computer)在 Linux 上不可用。
* **Dictation**：語音輸入在 Linux 桌面應用程式中不可用。請改用 CLI 中的[語音聽寫](/docs/zh-TW/voice-dictation)。
* **Quick Entry 全域快捷鍵**：在 X11 上運作。在原生 Wayland 上，它需要您的桌面環境的 GlobalShortcuts 入口。
* **Fedora 和 RHEL**：目前僅支援 Debian 衍生發行版本。將來會支援其他發行版本。

對於桌面應用程式中尚未提供的任何功能，[CLI](/docs/zh-TW/quickstart) 執行相同的 Claude Code 引擎並支援更廣泛的 Linux 發行版本；請參閱[系統需求](/docs/zh-TW/setup#system-requirements)。
