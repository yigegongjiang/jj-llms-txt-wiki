> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Anthropic 的 marketplace

> Anthropic 官方、社群和示範 plugin marketplace for Claude Code：它們的名稱、儲存庫、如何新增每個，以及在哪裡瀏覽它們的 plugin。

Anthropic 為 Claude Code 發佈三個通用 plugin marketplace：[官方](https://github.com/anthropics/claude-plugins-official)、[社群](https://github.com/anthropics/claude-plugins-community)和[示範](https://github.com/anthropics/claude-code)。每個都是其自身 GitHub 儲存庫中的 plugin 目錄。當您在 Claude Code 工作階段中從其中一個安裝 plugin 時，您在 `@` 後輸入 marketplace 的名稱，如 `/plugin install commit-commands@claude-plugins-official`。

使用此頁面來區分三個 marketplace，並找到檢查官方 marketplace 是否包含給定 plugin 的位置。

<Note>
  這些情況在其他頁面上涵蓋：

  * **如何安裝 plugin**：請參閱[安裝 plugin](/docs/zh-TW/plugins/install)
  * **安裝失敗**：請參閱[plugin 疑難排解](/docs/zh-TW/plugins/troubleshooting)
</Note>

前往您需要的頁面部分：

* 若要按儲存庫、marketplace 名稱和取得方式區分三個 marketplace，請參閱 [Anthropic 的 marketplace](#anthropic%E2%80%99s-marketplaces)。
* 若要在官方 marketplace 中尋找 plugin，請參閱[在官方 marketplace 中尋找 plugin](#find-plugins-in-the-official-marketplace)。

<h2 id="anthropic’s-marketplaces">
  Anthropic 的 marketplace
</h2>

Marketplace 是儲存庫在其 `.claude-plugin/marketplace.json` 檔案中定義的 plugin 目錄。官方、社群和示範 marketplace 各自來自其自身的 GitHub 儲存庫。Anthropic 也發佈主題特定的 marketplace，例如 `anthropics/skills` 和 `anthropics/knowledge-work-plugins`，您可以在 Claude Code 工作階段中使用 `/plugin marketplace add <owner>/<repo>` 新增。

此表格提供每個 marketplace 的儲存庫和 marketplace 名稱，這是您從該 marketplace 安裝 plugin 時在 `@` 後輸入的內容。社群 marketplace 的名稱是 `claude-community`，而不是其儲存庫名稱。

|                | 官方                                                                                                                                                                                                                                                                                                       | 社群                                                                                              | 示範                                                                                      |
| :------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------- |
| 儲存庫            | [`anthropics/claude-plugins-official`](https://github.com/anthropics/claude-plugins-official)                                                                                                                                                                                                            | [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community) | [`anthropics/claude-code`](https://github.com/anthropics/claude-code/tree/main/plugins) |
| Marketplace 名稱 | `claude-plugins-official`                                                                                                                                                                                                                                                                                | `claude-community`                                                                              | `claude-code-plugins`                                                                   |
| 其中包含的內容        | Anthropic 維護的 plugin，加上來自合作夥伴和其他作者的 plugin                                                                                                                                                                                                                                                               | 第三方 plugin，由其作者提交給 Anthropic                                                                    | 一小組示範 plugin，展示 plugin 可以包含的內容                                                          |
| 取得方式           | Claude Code 在您第一次啟動互動式終端工作階段時新增它，除非[受管原則](/docs/zh-TW/plugins/org#allow-the-official-marketplace-and-your-own)或 `CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL` 阻止它。如果遺失，請參閱 [Marketplace `claude-plugins-official` 找不到](/docs/zh-TW/plugins/troubleshooting#marketplace-claude-plugins-official-not-found) | 您在 Claude Code 工作階段中使用 `/plugin marketplace add anthropics/claude-plugins-community` 新增它        | 您在 Claude Code 工作階段中使用 `/plugin marketplace add anthropics/claude-code` 新增它             |

如果您編寫了 plugin 並希望其他人安裝它，請參閱[發佈 plugin](/docs/zh-TW/plugins/publish)，其涵蓋您自己的 marketplace 和提交到社群 marketplace。

<h3 id="the-demo-marketplace-in-anthropics/claude-code">
  `anthropics/claude-code` 中的示範 marketplace
</h3>

如果教學課程或較舊的指示集告訴您執行 `/plugin marketplace add anthropics/claude-code`，這會新增示範 marketplace，名為 `claude-code-plugins`。這不是官方 marketplace，Claude Code 已經為您新增了。

示範 marketplace 的大多數 plugin 也在官方 marketplace 中以相同名稱存在。例如，`code-review`、`feature-dev`、`commit-commands` 和 `security-guidance` 在兩者中都有。從 `claude-plugins-official` 安裝這些，以免安裝了兩份副本。

<h2 id="find-plugins-in-the-official-marketplace">
  在官方 marketplace 中尋找 plugin
</h2>

官方 marketplace `claude-plugins-official` 是 Claude Code 為您新增的。它列出的大部分內容來自合作夥伴和其他作者，而不是來自 Anthropic：工具供應商發佈將 Claude Code 連接到其服務的 plugin，Anthropic 維護一個較小的自有集合，例如 `commit-commands`、`code-review`、`feature-dev` 和[語言伺服器 plugin](/docs/zh-TW/plugins/code-intelligence)。目錄經常變化，所以此頁面不列出它。

若要查看其中的內容，請在 Claude Code 工作階段中使用 `/plugin` 的 **Discover** 標籤（您可以搜尋），或在網路上瀏覽 [Claude Marketplace](https://claude.com/marketplace/plugins)。

<h2 id="browse-and-install-from-anthropic’s-marketplaces">
  從 Anthropic 的 marketplace 瀏覽和安裝
</h2>

您可以在 Claude Code、網路或 GitHub 上搜尋 Anthropic 的 marketplace 中的 plugin：

* **在 Claude Code 中，透過瀏覽**：在互動式工作階段中執行 `/plugin`。其 **Discover** 標籤列出您已新增的 marketplace 中的 plugin。
* **在 Claude Code 中，按名稱**：在工作階段中執行 `/plugin install <name>`，它會在您已新增的 marketplace 中查詢名稱。如果 plugin 在其中之一，其詳細資訊會在 `/plugin` 面板中開啟，在您選擇[安裝範圍](/docs/zh-TW/plugins/install#install-a-plugin)並確認之前，不會安裝任何內容。如果不在，您會看到 `Plugin "<name>" not found in any marketplace`。
* **在網路上**：在 [Claude Marketplace](https://claude.com/marketplace/plugins) 上搜尋完整目錄，其顯示安裝計數並標記某些 plugin 為 **Anthropic verified**。
* **在 GitHub 上**：在 marketplace 的儲存庫中開啟 `.claude-plugin/marketplace.json`，例如 [`anthropics/claude-plugins-official`](https://github.com/anthropics/claude-plugins-official)。該檔案就是目錄本身。

若要從桌面應用程式或指令碼安裝，或查看雲端工作階段載入的內容，請參閱[安裝 plugin](/docs/zh-TW/plugins/install)。

<h3 id="add-the-community-or-demo-marketplace">
  新增社群或示範 marketplace
</h3>

社群和示範 marketplace 在您在 Claude Code 工作階段中新增它們之前不會註冊：

* **社群**：執行 `/plugin marketplace add anthropics/claude-plugins-community`，然後使用 `@claude-community` 尾碼安裝。
* **示範**：執行 `/plugin marketplace add anthropics/claude-code`，然後使用 `@claude-code-plugins` 尾碼安裝。

如果 `claude-plugins-official` 不在 `/plugin` 的 **Marketplaces** 標籤上，使用 `/plugin marketplace add anthropics/claude-plugins-official` 以相同方式新增它。

如需 `not found` 錯誤和無法新增的 marketplace，請參閱[plugin 疑難排解](/docs/zh-TW/plugins/troubleshooting#install-a-plugin)。

<h2 id="third-party-marketplaces">
  第三方 marketplace
</h2>

許多熱門 plugin 不在任何 Anthropic marketplace 中。它們在其作者自己的 marketplace 中，通常是在其根目錄中具有 `.claude-plugin/marketplace.json` 的 GitHub 儲存庫。

Anthropic 不審查第三方 marketplace，所以在新增一個之前，請閱讀 [Plugin 安全性和信任](/docs/zh-TW/plugins/security)。

若要使用第三方 marketplace，在 Claude Code 工作階段中使用 `/plugin marketplace add <owner>/<repo>` 新增其儲存庫，然後使用 `/plugin install <plugin>@<marketplace-name>` 安裝。marketplace 名稱是該 `marketplace.json` 的 `name` 欄位，Claude Code 在新增 marketplace 後會列印它。

如需新增 marketplace 的其他方式，請參閱[新增 marketplace](/docs/zh-TW/plugins/install#add-a-marketplace)。

<h2 id="next-steps">
  後續步驟
</h2>

* [安裝和管理 plugin](/docs/zh-TW/plugins/install)：從其中一個 marketplace 安裝 plugin 並選擇範圍
* [Plugin 安全性和信任](/docs/zh-TW/plugins/security)：plugin 可以在您的機器上執行的操作，以及在安裝前如何檢查一個
* [程式碼智慧 plugin](/docs/zh-TW/plugins/code-intelligence)：安裝官方 marketplace 的語言伺服器 plugin 之一
* [建立 marketplace](/docs/zh-TW/plugins/create-marketplace)：在 Anthropic 的旁邊執行您自己的 marketplace
