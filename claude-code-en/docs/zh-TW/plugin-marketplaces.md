> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 建立並分發 plugin marketplace

> 建立並託管 plugin marketplace，以在團隊和社群中分發 Claude Code 擴充功能。

**plugin marketplace** 是一個目錄，可讓您將 plugin 分發給他人。Marketplace 提供集中式發現、版本追蹤、自動更新，以及對多種來源類型（包括 git 儲存庫和本機路徑）的支援。本指南將向您展示如何建立自己的 marketplace，以與您的團隊或社群分享 plugin。

想要從現有 marketplace 安裝 plugin？請參閱[探索並安裝預先建立的 plugin](/docs/zh-TW/discover-plugins)。

<h2 id="overview">
  概述
</h2>

建立並分發 marketplace 涉及：

1. **建立 plugin**：使用 skills、agents、hooks、MCP servers 或 LSP servers 建立一個或多個 plugin。本指南假設您已經有要分發的 plugin；有關如何建立 plugin 的詳細資訊，請參閱[建立 plugin](/docs/zh-TW/plugins)。
2. **建立 marketplace 檔案**：定義 `marketplace.json`，列出您的 plugin 及其位置。請參閱[建立 marketplace 檔案](#create-the-marketplace-file)。
3. **託管 marketplace**：推送到 GitHub、GitLab 或其他 git 主機。請參閱[託管並分發 marketplace](#host-and-distribute-marketplaces)。
4. **與使用者分享**：使用者使用 `/plugin marketplace add` 新增您的 marketplace 並安裝個別 plugin。請參閱[探索並安裝 plugin](/docs/zh-TW/discover-plugins)。

一旦您的 marketplace 上線，您可以透過推送變更到您的儲存庫來更新它。使用者使用 `/plugin marketplace update` 重新整理其本機副本。

<h2 id="walkthrough-create-a-local-marketplace">
  逐步解說：建立本機 marketplace
</h2>

此範例建立一個包含一個 plugin 的 marketplace：用於程式碼審查的 `quality-review` skill。您將建立目錄結構、新增 skill、建立 plugin manifest 和 marketplace 目錄，然後安裝並測試它。

<Steps>
  <Step title="建立目錄結構">
    ```bash theme={null}
    mkdir -p my-marketplace/.claude-plugin
    mkdir -p my-marketplace/plugins/quality-review-plugin/.claude-plugin
    mkdir -p my-marketplace/plugins/quality-review-plugin/skills/quality-review
    ```
  </Step>

  <Step title="建立 skill">
    建立 `SKILL.md` 檔案，定義 `quality-review` skill 的功能。

    ```markdown my-marketplace/plugins/quality-review-plugin/skills/quality-review/SKILL.md theme={null}
    ---
    description: 檢查程式碼中的錯誤、安全性和效能問題
    ---

    檢查我選擇的程式碼或最近的變更，查找：
    - 潛在的錯誤或邊界情況
    - 安全性問題
    - 效能問題
    - 可讀性改進

    簡潔且可行動。
    ```
  </Step>

  <Step title="建立 plugin manifest">
    建立 `plugin.json` 檔案，描述 plugin。manifest 位於 `.claude-plugin/` 目錄中。

    ```json my-marketplace/plugins/quality-review-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "quality-review-plugin",
      "description": "新增 quality-review skill 以進行快速程式碼審查",
      "version": "1.0.0"
    }
    ```

    <Note>
      設定 `version` 表示使用者只會在您變更此欄位時收到更新，因此在每次發行時都要提升版本。如果您省略 `version` 並在 git 中託管此 marketplace，每次提交都會自動計為新版本。請參閱 [Version resolution](#version-resolution-and-release-channels) 以選擇正確的方法。
    </Note>
  </Step>

  <Step title="建立 marketplace 檔案">
    建立列出您的 plugin 的 marketplace 目錄。

    ```json my-marketplace/.claude-plugin/marketplace.json theme={null}
    {
      "name": "my-plugins",
      "owner": {
        "name": "Your Name"
      },
      "plugins": [
        {
          "name": "quality-review-plugin",
          "source": "./plugins/quality-review-plugin",
          "description": "新增 quality-review skill 以進行快速程式碼審查"
        }
      ]
    }
    ```
  </Step>

  <Step title="新增並安裝">
    新增 marketplace 並安裝 plugin。

    ```shell theme={null}
    /plugin marketplace add ./my-marketplace
    /plugin install quality-review-plugin@my-plugins
    ```
  </Step>

  <Step title="試試看">
    在編輯器中選擇一些程式碼並執行您的新 skill。Plugin skills 使用 plugin 名稱進行命名空間。

    ```shell theme={null}
    /quality-review-plugin:quality-review
    ```
  </Step>
</Steps>

若要深入瞭解 plugin 可以執行的操作，包括 hooks、agents、MCP servers 和 LSP servers，請參閱 [Plugins](/docs/zh-TW/plugins)。

<Note>
  **plugin 如何安裝**：當使用者安裝 plugin 時，Claude Code 會將 plugin 目錄複製到快取位置。這表示 plugin 無法使用 `../shared-utils` 之類的路徑參考其目錄外的檔案，因為這些檔案不會被複製。

  如果您需要在 plugin 之間共享檔案，請使用符號連結。有關詳細資訊，請參閱 [Plugin caching and file resolution](/docs/zh-TW/plugins-reference#plugin-caching-and-file-resolution)。
</Note>

<h2 id="create-the-marketplace-file">
  建立 marketplace 檔案
</h2>

在您的儲存庫根目錄中建立 `.claude-plugin/marketplace.json`。此檔案定義您的 marketplace 名稱、擁有者資訊以及包含其來源的 plugin 清單。

每個 plugin 項目至少需要 `name` 和 `source`（告訴 Claude Code 從何處取得）。有關所有可用欄位，請參閱下面的[完整架構](#marketplace-schema)。

```json theme={null}
{
  "name": "company-tools",
  "owner": {
    "name": "DevTools Team",
    "email": "devtools@example.com"
  },
  "plugins": [
    {
      "name": "code-formatter",
      "source": "./plugins/formatter",
      "description": "在保存時自動格式化程式碼",
      "version": "2.1.0",
      "author": {
        "name": "DevTools Team"
      }
    },
    {
      "name": "deployment-tools",
      "source": {
        "source": "github",
        "repo": "company/deploy-plugin"
      },
      "description": "部署自動化工具"
    }
  ]
}
```

<h2 id="marketplace-schema">
  Marketplace 架構
</h2>

<h3 id="required-fields">
  必需欄位
</h3>

| 欄位        | 類型     | 描述                                                                                                                                                                                                                                                                 | 範例             |
| :-------- | :----- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------- |
| `name`    | string | Marketplace 識別碼（kebab-case，無空格）。這是公開的：使用者在安裝 plugin 時會看到它（例如，`/plugin install my-tool@your-marketplace`）。每個使用者只能為每個名稱註冊一個 marketplace：新增第二個同名 marketplace 會取代第一個。若要在一個 marketplace 名稱下發佈多個 plugin，請在[單一 `marketplace.json`](#create-the-marketplace-file) 中列出它們全部。 | `"acme-tools"` |
| `owner`   | object | Marketplace 維護者資訊（[請參閱下面的欄位](#owner-fields)）                                                                                                                                                                                                                       |                |
| `plugins` | array  | 可用 plugin 的清單                                                                                                                                                                                                                                                      | 請參閱下面          |

<Note>
  **保留名稱**：以下 marketplace 名稱保留供 Anthropic 官方使用，第三方 marketplace 無法使用：`claude-code-marketplace`、`claude-code-plugins`、`claude-plugins-official`、`claude-plugins-community`、`claude-community`、`anthropic-marketplace`、`anthropic-plugins`、`agent-skills`、`anthropic-agent-skills`、`knowledge-work-plugins`、`life-sciences`、`claude-for-legal`、`claude-for-financial-services`、`financial-services-plugins`、`first-party-plugins`、`healthcare`。模仿官方 marketplace 的名稱（如 `official-claude-plugins` 或 `anthropic-plugins-v2`）也被阻止。保留這些名稱可防止第三方 marketplace 將自己冒充為 Anthropic 發佈的來源。

  Claude Code 每次載入 marketplace 時都會重新檢查保留名稱，而不僅在您新增 marketplace 時檢查。在名稱成為保留名稱之前以其中一個名稱註冊的 marketplace 會停止載入，並報告它是[從不受信任的來源註冊](/docs/zh-TW/errors#marketplace-is-registered-from-an-untrusted-source)。移除該 marketplace，並從官方 Anthropic 來源重新新增它。受新保留名稱影響的第三方 marketplace 在您以不同名稱重新新增它後立即再次載入。在 v2.1.205 之前，`first-party-plugins` 和 `healthcare` 未被保留，已在保留名稱下註冊的 marketplace 繼續載入。
</Note>

<h3 id="owner-fields">
  擁有者欄位
</h3>

| 欄位      | 類型     | 必需 | 描述         |
| :------ | :----- | :- | :--------- |
| `name`  | string | 是  | 維護者或團隊的名稱  |
| `email` | string | 否  | 維護者的聯絡電子郵件 |

<h3 id="optional-fields">
  選用欄位
</h3>

| 欄位                                    | 類型     | 描述                                                                                                                                                                                      |
| :------------------------------------ | :----- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `$schema`                             | string | JSON Schema URL，用於編輯器自動完成和驗證。Claude Code 在載入時會忽略此欄位。                                                                                                                                    |
| `description`                         | string | 簡短的 marketplace 描述                                                                                                                                                                      |
| `version`                             | string | Marketplace 版本                                                                                                                                                                          |
| `metadata.pluginRoot`                 | string | 前置於相對 plugin 來源路徑的基本目錄（例如，`"./plugins"` 可讓您寫入 `"source": "formatter"` 而不是 `"source": "./plugins/formatter"`）                                                                            |
| `allowCrossMarketplaceDependenciesOn` | array  | 此 marketplace 中的 plugin 可能依賴的其他 marketplace。來自此處未列出的 marketplace 的相依性在安裝時被阻止。請參閱[依賴來自另一個 marketplace 的 plugin](/docs/zh-TW/plugin-dependencies#depend-on-a-plugin-from-another-marketplace)。 |
| `renames`                             | object | 從前一個 plugin `name` 對應到其目前名稱，或對應到 `null`（如果 plugin 已移除）的對應。當您重新命名或移除 `plugins` 中的項目時，可讓現有使用者自動遷移。請參閱[重新命名或移除 plugin](#rename-or-remove-a-plugin)。需要 Claude Code v2.1.193 或更新版本。          |

`description` 和 `version` 也可在 `metadata` 下接受，以保持向後相容性。

<h2 id="plugin-entries">
  Plugin 項目
</h2>

`plugins` 陣列中的每個 plugin 項目描述一個 plugin 及其位置。您可以包含 [plugin manifest 架構](/docs/zh-TW/plugins-reference#plugin-manifest-schema)中的任何欄位（如 `description`、`version`、`author`、`commands`、`hooks` 等），加上這些 marketplace 特定欄位：`source`、`category`、`tags`、`strict` 和 `relevance`。

<h3 id="required-fields-2">
  必需欄位
</h3>

| 欄位       | 類型             | 描述                                                                                        |
| :------- | :------------- | :---------------------------------------------------------------------------------------- |
| `name`   | string         | Plugin 識別碼（kebab-case，無空格）。這是公開的：使用者在安裝時會看到它（例如，`/plugin install my-plugin@marketplace`）。 |
| `source` | string\|object | 從何處取得 plugin（請參閱下面的 [Plugin 來源](#plugin-sources)）                                         |

<h3 id="optional-plugin-fields">
  選用 plugin 欄位
</h3>

**標準中繼資料欄位：**

| 欄位               | 類型      | 描述                                                                                                                                                                                      |
| :--------------- | :------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `displayName`    | string  | 在 UI 介面中顯示的人類可讀名稱。當省略時回退到 `name`。可以包含空格和任何大小寫。不用於命名空間或查詢。需要 Claude Code v2.1.143 或更新版本。                                                                                                 |
| `description`    | string  | 簡短的 plugin 描述                                                                                                                                                                           |
| `version`        | string  | Plugin 版本。如果設定（在此處或在 `plugin.json` 中），plugin 會固定到此字串，使用者只有在版本變更時才會收到更新。省略以回退到 git commit SHA。請參閱 [版本解析](#version-resolution-and-release-channels)。                                      |
| `author`         | object  | Plugin 作者資訊（`name` 必需，`email` 選用）                                                                                                                                                       |
| `homepage`       | string  | Plugin 首頁或文件 URL                                                                                                                                                                        |
| `repository`     | string  | 原始碼儲存庫 URL                                                                                                                                                                              |
| `license`        | string  | SPDX 授權識別碼（例如，MIT、Apache-2.0）                                                                                                                                                           |
| `keywords`       | array   | 用於 plugin 發現和分類的標籤                                                                                                                                                                      |
| `category`       | string  | Plugin 類別以供組織                                                                                                                                                                           |
| `tags`           | array   | 用於可搜尋性的標籤                                                                                                                                                                               |
| `strict`         | boolean | 控制 `plugin.json` 是否為元件定義的權威（預設值：true）。請參閱下面的 [Strict mode](#strict-mode)。                                                                                                               |
| `relevance`      | object  | 告知 Claude Code 何時向使用者建議此 plugin 的訊號。僅對管理員在受管設定中允許清單的 marketplace 生效。請參閱 [為您的組織推薦 plugin](/docs/zh-TW/plugin-relevance)。需要 Claude Code v2.1.152 或更新版本。                                        |
| `defaultEnabled` | boolean | 安裝後 plugin 是否啟用（預設值：true）。設定為 `false` 以安裝已停用的 plugin，直到使用者選擇加入。優先於 plugin 的 `plugin.json` 中的相同欄位。請參閱 [預設啟用](/docs/zh-TW/plugins-reference#default-enablement)。需要 Claude Code v2.1.154 或更新版本。 |

**元件配置欄位：**

| 欄位           | 類型             | 描述                                   |
| :----------- | :------------- | :----------------------------------- |
| `skills`     | string\|array  | 包含 `<name>/SKILL.md` 的 skill 目錄的自訂路徑 |
| `commands`   | string\|array  | 平面 `.md` skill 檔案或目錄的自訂路徑            |
| `agents`     | string\|array  | agent 檔案的自訂路徑                        |
| `hooks`      | string\|object | 自訂 hooks 配置或 hooks 檔案的路徑             |
| `mcpServers` | string\|object | MCP server 配置或 MCP 配置的路徑             |
| `lspServers` | string\|object | LSP server 配置或 LSP 配置的路徑             |

<h2 id="plugin-sources">
  Plugin 來源
</h2>

Plugin 來源告訴 Claude Code 在您的 marketplace 中列出的每個個別 plugin 從何處取得。這些在 `marketplace.json` 中每個 plugin 項目的 `source` 欄位中設定。

一旦 Claude Code 複製或下載 plugin 到本機，它就會將 plugin 複製到本機版本化 plugin 快取中，位於 `~/.claude/plugins/cache`。

| 來源           | 類型                           | 欄位                               | 備註                                                                               |
| ------------ | ---------------------------- | -------------------------------- | -------------------------------------------------------------------------------- |
| 相對路徑         | `string`（例如 `"./my-plugin"`） | 無                                | marketplace 儲存庫內的本機目錄。必須以 `./` 開頭。相對於 marketplace 根目錄解析，而不是 `.claude-plugin/` 目錄 |
| `github`     | object                       | `repo`、`ref?`、`sha?`             |                                                                                  |
| `url`        | object                       | `url`、`ref?`、`sha?`              | Git URL 來源                                                                       |
| `git-subdir` | object                       | `url`、`path`、`ref?`、`sha?`       | git 儲存庫內的子目錄。稀疏複製以最小化大型 monorepo 的頻寬                                             |
| `npm`        | object                       | `package`、`version?`、`registry?` | 透過 `npm install` 安裝                                                              |

<Note>
  **Marketplace 來源與 plugin 來源**：這些是控制不同事物的不同概念。

  * **Marketplace 來源**：從何處取得 `marketplace.json` 目錄本身。在使用者執行 `/plugin marketplace add` 或在 `extraKnownMarketplaces` 設定中設定。支援 `ref`（分支/標籤）但不支援 `sha`。
  * **Plugin 來源**：從何處取得 marketplace 中列出的個別 plugin。在 `marketplace.json` 內每個 plugin 項目的 `source` 欄位中設定。支援 `ref`（分支/標籤）和 `sha`（確切提交）。

  例如，託管在 `acme-corp/plugin-catalog`（marketplace 來源）的 marketplace 可以列出從 `acme-corp/code-formatter`（plugin 來源）取得的 plugin。marketplace 來源和 plugin 來源指向不同的儲存庫，並獨立固定。
</Note>

基於 git 的來源類型如下為 `github`、`url` 和 `git-subdir`。當任何一個上同時設定 `ref` 和 `sha` 時，`sha` 是有效的固定。Claude Code 直接取得並簽出固定的提交。在大多數 git 主機上，包括 GitHub、GitLab 和 Bitbucket，這表示即使上游的 `ref` 命名的分支或標籤已被刪除，只要提交仍可從儲存庫到達，安裝就會成功。某些伺服器（例如 AWS CodeCommit）不支援透過 SHA 取得提交。在這些伺服器上，`ref` 仍必須存在，且固定的提交必須可從其到達。

<h3 id="relative-paths">
  相對路徑
</h3>

對於同一儲存庫中的 plugin，使用以 `./` 開頭的路徑：

```json theme={null}
{
  "name": "my-plugin",
  "source": "./plugins/my-plugin"
}
```

路徑相對於 marketplace 根目錄解析，即包含 `.claude-plugin/` 的目錄。在上面的範例中，`./plugins/my-plugin` 指向 `<repo>/plugins/my-plugin`，即使 `marketplace.json` 位於 `<repo>/.claude-plugin/marketplace.json`。不要使用 `../` 參考 marketplace 根目錄外的路徑。

<Note>
  相對路徑會針對 marketplace 的本機副本解析，因此當使用者從 git 來源或本機目錄新增您的 marketplace 時可以運作。如果使用者透過直接 URL 新增您的 marketplace 到 `marketplace.json` 檔案，相對路徑將無法解析，因為只會下載該檔案。對於基於 URL 的分發，請改用 GitHub、npm 或 git URL 來源。有關詳細資訊，請參閱[疑難排解](#plugins-with-relative-paths-fail-in-url-based-marketplaces)。
</Note>

<h3 id="github-repositories">
  GitHub 儲存庫
</h3>

```json theme={null}
{
  "name": "github-plugin",
  "source": {
    "source": "github",
    "repo": "owner/plugin-repo"
  }
}
```

您可以固定到特定分支、標籤或提交：

```json theme={null}
{
  "name": "github-plugin",
  "source": {
    "source": "github",
    "repo": "owner/plugin-repo",
    "ref": "v2.0.0",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

| 欄位     | 類型     | 描述                               |
| :----- | :----- | :------------------------------- |
| `repo` | string | 必需。`owner/repo` 格式的 GitHub 儲存庫   |
| `ref`  | string | 選用。Git 分支或標籤（預設為儲存庫預設分支）         |
| `sha`  | string | 選用。完整的 40 字元 git 提交 SHA 以固定到確切版本 |

<h3 id="git-repositories">
  Git 儲存庫
</h3>

```json theme={null}
{
  "name": "git-plugin",
  "source": {
    "source": "url",
    "url": "https://gitlab.com/team/plugin.git"
  }
}
```

您可以固定到特定分支、標籤或提交：

```json theme={null}
{
  "name": "git-plugin",
  "source": {
    "source": "url",
    "url": "https://gitlab.com/team/plugin.git",
    "ref": "main",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

| 欄位    | 類型     | 描述                                                                                                   |
| :---- | :----- | :--------------------------------------------------------------------------------------------------- |
| `url` | string | 必需。完整的 git 儲存庫 URL（`https://` 或 `git@`）。`.git` 後綴是選用的，因此 Azure DevOps 和 AWS CodeCommit URL 不含後綴也可以運作 |
| `ref` | string | 選用。Git 分支或標籤（預設為儲存庫預設分支）                                                                             |
| `sha` | string | 選用。完整的 40 字元 git 提交 SHA 以固定到確切版本                                                                     |

<h3 id="git-subdirectories">
  Git 子目錄
</h3>

使用 `git-subdir` 指向位於 git 儲存庫子目錄內的 plugin。Claude Code 使用稀疏、部分複製來僅取得子目錄，最小化大型 monorepo 的頻寬。

```json theme={null}
{
  "name": "my-plugin",
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/acme-corp/monorepo.git",
    "path": "tools/claude-plugin"
  }
}
```

您可以固定到特定分支、標籤或提交：

```json theme={null}
{
  "name": "my-plugin",
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/acme-corp/monorepo.git",
    "path": "tools/claude-plugin",
    "ref": "v2.0.0",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

`url` 欄位也接受 GitHub 簡寫（`owner/repo`）或 SSH URL（`git@github.com:owner/repo.git`）。

| 欄位     | 類型     | 描述                                                  |
| :----- | :----- | :-------------------------------------------------- |
| `url`  | string | 必需。Git 儲存庫 URL、GitHub `owner/repo` 簡寫或 SSH URL      |
| `path` | string | 必需。儲存庫內包含 plugin 的子目錄路徑（例如，`"tools/claude-plugin"`） |
| `ref`  | string | 選用。Git 分支或標籤（預設為儲存庫預設分支）                            |
| `sha`  | string | 選用。完整的 40 字元 git 提交 SHA 以固定到確切版本                    |

<h3 id="npm-packages">
  npm 套件
</h3>

作為 npm 套件分發的 plugin 使用 `npm install` 安裝。這適用於公開 npm 登錄表或您的團隊託管的任何私人登錄表上的任何套件。

```json theme={null}
{
  "name": "my-npm-plugin",
  "source": {
    "source": "npm",
    "package": "@acme/claude-plugin"
  }
}
```

若要固定到特定版本，請新增 `version` 欄位：

```json theme={null}
{
  "name": "my-npm-plugin",
  "source": {
    "source": "npm",
    "package": "@acme/claude-plugin",
    "version": "2.1.0"
  }
}
```

若要從私人或內部登錄表安裝，請新增 `registry` 欄位：

```json theme={null}
{
  "name": "my-npm-plugin",
  "source": {
    "source": "npm",
    "package": "@acme/claude-plugin",
    "version": "^2.0.0",
    "registry": "https://npm.example.com"
  }
}
```

| 欄位         | 類型     | 描述                                             |
| :--------- | :----- | :--------------------------------------------- |
| `package`  | string | 必需。套件名稱或範圍套件（例如，`@org/plugin`）                 |
| `version`  | string | 選用。版本或版本範圍（例如，`2.1.0`、`^2.0.0`、`~1.5.0`）       |
| `registry` | string | 選用。自訂 npm 登錄表 URL。預設為系統 npm 登錄表（通常為 npmjs.org） |

<h3 id="advanced-plugin-entries">
  進階 plugin 項目
</h3>

此範例顯示使用許多選用欄位的 plugin 項目，包括 commands、agents、hooks 和 MCP servers 的自訂路徑：

```json theme={null}
{
  "name": "enterprise-tools",
  "source": {
    "source": "github",
    "repo": "company/enterprise-plugin"
  },
  "description": "企業工作流程自動化工具",
  "version": "2.1.0",
  "author": {
    "name": "Enterprise Team",
    "email": "enterprise@example.com"
  },
  "homepage": "https://docs.example.com/plugins/enterprise-tools",
  "repository": "https://github.com/company/enterprise-plugin",
  "license": "MIT",
  "keywords": ["enterprise", "workflow", "automation"],
  "category": "productivity",
  "commands": [
    "./commands/core/",
    "./commands/enterprise/",
    "./commands/experimental/preview.md"
  ],
  "agents": ["./agents/security-reviewer.md", "./agents/compliance-checker.md"],
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "${CLAUDE_PLUGIN_ROOT}/scripts/validate.sh"
          }
        ]
      }
    ]
  },
  "mcpServers": {
    "enterprise-db": {
      "command": "${CLAUDE_PLUGIN_ROOT}/servers/db-server",
      "args": ["--config", "${CLAUDE_PLUGIN_ROOT}/config.json"]
    }
  },
  "strict": false
}
```

需要注意的關鍵事項：

* **`commands` 和 `agents`**：您可以指定多個目錄或個別檔案。路徑相對於 plugin 根目錄。
* **`${CLAUDE_PLUGIN_ROOT}`**：在 hooks 和 MCP server 配置中使用此變數來參考 plugin 安裝目錄內的檔案。這是必要的，因為 plugin 在安裝時被複製到快取位置。
  * 請參閱[替換表](/docs/zh-TW/plugins-reference#environment-variables)以了解每個伺服器類型的哪些配置欄位會替換它
  * 對於應在 plugin 更新後保留的相依性或狀態，請改用 [`${CLAUDE_PLUGIN_DATA}`](/docs/zh-TW/plugins-reference#persistent-data-directory)
* **`strict: false`**：由於此設定為 false，plugin 不需要自己的 `plugin.json`。marketplace 項目定義所有內容。請參閱下面的 [Strict mode](#strict-mode)。

根據預設，plugin 的 skills 從其 `source` 下的 `skills/` 目錄載入。`skills` 欄位中列出的路徑會新增到該掃描中：

```json theme={null}
"skills": ["./skills/", "./extra-skills/"]
```

當多個 plugin 項目在 marketplace 根目錄（`source: "./"`) 共享一個 `skills/` 資料夾時，改為列出特定子目錄，以便每個項目只載入自己的 skills：

```json theme={null}
"source": "./",
"skills": ["./skills/code-review", "./skills/docs"]
```

使用 marketplace 根目錄 `source` 時，列出的路徑是該項目的完整集合，共享 `skills/` 資料夾中的其他目錄不會載入。列出 `./skills/` 本身或 plugin 根目錄會保持完整掃描。如果列出的路徑都不存在，則改為執行預設掃描。

<h3 id="strict-mode">
  Strict mode
</h3>

`strict` 欄位控制 `plugin.json` 是否為元件定義（skills、agents、hooks、MCP servers、輸出樣式）的權威。

| 值          | 行為                                                                     |
| :--------- | :--------------------------------------------------------------------- |
| `true`（預設） | `plugin.json` 是權威。marketplace 項目可以用額外的元件補充它，兩個來源都會合併。                  |
| `false`    | marketplace 項目是完整定義。如果 plugin 也有宣告元件的 `plugin.json`，那就是衝突，plugin 無法載入。 |

**何時使用每種模式：**

* **`strict: true`**：plugin 有自己的 `plugin.json` 並管理自己的元件。marketplace 項目可以在頂部新增額外的 skills 或 hooks。這是預設值，適用於大多數 plugin。
* **`strict: false`**：marketplace 運營商想要完全控制。plugin 儲存庫提供原始檔案，marketplace 項目定義這些檔案中的哪些被公開為 skills、agents、hooks 等。當 marketplace 以不同於 plugin 作者預期的方式重組或策劃 plugin 的元件時很有用。

<h2 id="host-and-distribute-marketplaces">
  託管並分發 marketplace
</h2>

<h3 id="host-on-github-recommended">
  在 GitHub 上託管（推薦）
</h3>

GitHub 是託管和分發 marketplace 的推薦方式：

1. **建立儲存庫**：為您的 marketplace 設定新儲存庫
2. **新增 marketplace 檔案**：使用您的 plugin 定義建立 `.claude-plugin/marketplace.json`
3. **與團隊分享**：使用者使用 `/plugin marketplace add owner/repo` 新增您的 marketplace

**優點**：內建版本控制、問題追蹤和團隊協作功能。

<h3 id="host-on-other-git-services">
  在其他 git 服務上託管
</h3>

任何 git 託管服務都可以使用，例如 GitLab、Bitbucket 和自託管伺服器。使用者使用完整儲存庫 URL 新增：

```shell theme={null}
/plugin marketplace add https://gitlab.com/company/plugins.git
```

<h3 id="private-repositories">
  私人儲存庫
</h3>

Claude Code 支援從私人儲存庫安裝 plugin。對於手動安裝和更新，Claude Code 使用您現有的 git 認證助手，因此 HTTPS 存取透過 `gh auth login`、macOS Keychain 或 `git-credential-store` 的方式與在您的終端中相同。只要主機已在您的 `known_hosts` 檔案中且金鑰已載入 `ssh-agent`，SSH 存取就可以運作，因為 Claude Code 會抑制主機指紋和金鑰密碼的互動式 SSH 提示。GitHub `owner/repo` 簡寫來源預設透過 SSH 複製；設定 [`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`](/docs/zh-TW/env-vars#variables) 以改為透過 HTTPS 複製它們。

背景自動更新的運作方式不同。根據預設，背景重新整理會為其 `git pull` 停用 git 認證助手，因此即使已配置助手，pull 也無法對私人儲存庫進行 HTTPS 驗證。SSH 遠端不受影響：載入在 `ssh-agent` 中的金鑰會以與手動操作相同的方式驗證背景 pull。當背景 pull 失敗時，Claude Code 會回退到從頭重新複製 marketplace。重新複製確實會使用您儲存的 git 認證，但它可能會在大型儲存庫上[逾時](#git-operations-time-out)，因此私人 marketplace 自動更新可能會間歇性失敗。

兩個設定使私人 marketplace 的行為可預測：

* 設定 `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1` 以在背景 pull 失敗時保留現有複製，而不是刪除並重新複製。您的 plugin 會從最後同步的狀態繼續運作，使用 `/plugin marketplace update` 的手動更新仍會使用您的認證進行 pull。
* 配置 git 認證助手，例如使用 `gh auth setup-git` 針對 GitHub，以便重新複製回退可以在不提示的情況下進行驗證。

在您的環境中設定提供者令牌（例如 `GITHUB_TOKEN`）本身不會啟用背景驗證。令牌只有透過已配置的認證助手（例如 `gh` CLI 的助手，它讀取 `GH_TOKEN` 和 `GITHUB_TOKEN`）才會生效。

若要使背景 pull 本身透過 HTTPS 進行驗證，請配置全域 git URL 重寫。重寫會在遠端 URL 中嵌入令牌，因此即使背景 pull 停用認證助手，它也會生效，成功的 pull 會跳過重新複製回退。以下範例會重寫 marketplace 儲存庫的 URL 以包含存取令牌：

```bash theme={null}
git config --global url."https://x-access-token:YOUR_TOKEN@github.com/acme-corp/plugins".insteadOf "https://github.com/acme-corp/plugins"
```

將重寫範圍限制在 marketplace 儲存庫或組織路徑。其基礎僅為主機的重寫適用於該機器上對該主機的每次 fetch 和 push，並覆蓋您的正常認證，包括推送到您自己的儲存庫。

每個提供者在重寫的 URL 中預期不同的使用者名稱，相同的路徑範圍適用於每個提供者。對於自託管伺服器，請將主機名稱替換為您的伺服器主機名稱：

| 提供者       | 重寫的 URL 形式                                                        |
| :-------- | :---------------------------------------------------------------- |
| GitHub    | `https://x-access-token:YOUR_TOKEN@github.com/acme-corp/plugins`  |
| GitLab    | `https://oauth2:YOUR_TOKEN@gitlab.com/acme-corp/plugins`          |
| Bitbucket | `https://x-token-auth:YOUR_TOKEN@bitbucket.org/acme-corp/plugins` |

重寫會以純文字形式將令牌儲存在您的 gitconfig 中，因此請使用具有對 marketplace 儲存庫的唯讀存取權的令牌。

<Note>
  在 CI/CD 環境中，在從私人儲存庫安裝 plugin 之前配置 git 認證助手。在 GitHub Actions 上，匯出具有對 marketplace 儲存庫的讀取存取權的令牌作為 `GH_TOKEN`，然後執行 `gh auth setup-git`。預設工作流程令牌只能存取工作流程自己的儲存庫，因此另一個儲存庫中的私人 marketplace 需要個人存取令牌或應用程式令牌。在管道中配置的全域 URL 重寫也會直接驗證背景 pull。
</Note>

<h3 id="test-locally-before-distribution">
  在分發前在本機測試
</h3>

在分享前在本機測試您的 marketplace：

```shell theme={null}
/plugin marketplace add ./my-marketplace
/plugin install quality-review-plugin@my-plugins
```

有關完整的新增命令範圍（GitHub、Git URL、本機路徑、遠端 URL），請參閱[新增 marketplace](/docs/zh-TW/discover-plugins#add-marketplaces)。

<h3 id="require-marketplaces-for-your-team">
  為您的團隊要求 marketplace
</h3>

您可以配置您的儲存庫，以便當團隊成員信任專案資料夾時，他們會自動被提示安裝您的 marketplace。將您的 marketplace 新增到 `.claude/settings.json`：

```json theme={null}
{
  "extraKnownMarketplaces": {
    "company-tools": {
      "source": {
        "source": "github",
        "repo": "your-org/claude-plugins"
      }
    }
  }
}
```

您也可以指定預設應啟用哪些 plugin：

```json theme={null}
{
  "enabledPlugins": {
    "code-formatter@company-tools": true,
    "deployment-tools@company-tools": true
  }
}
```

有關完整的配置選項，請參閱 [Plugin settings](/docs/zh-TW/settings#plugin-settings)。

<Note>
  如果您使用具有相對路徑的本機 `directory` 或 `file` 來源，路徑會針對您的儲存庫的主要簽出進行解析。當您從 git worktree 執行 Claude Code 時，路徑仍然指向主要簽出，因此所有 worktrees 共享相同的 marketplace 位置。Marketplace 狀態每個使用者儲存一次在 `~/.claude/plugins/known_marketplaces.json` 中，而不是每個專案。
</Note>

<h3 id="pre-populate-plugins-for-containers">
  為容器預先填充 plugin
</h3>

對於容器映像和 CI 環境，您可以在建置時預先填充 plugin 目錄，以便 Claude Code 啟動時已有 marketplace 和 plugin 可用，無需在執行時複製任何內容。設定 `CLAUDE_CODE_PLUGIN_SEED_DIR` 環境變數以指向此目錄。

若要分層多個種子目錄，請在 Unix 上使用 `:` 或在 Windows 上使用 `;` 分隔路徑。Claude Code 按順序搜尋每個目錄，第一個包含給定 marketplace 或 plugin 快取的種子獲勝。

種子目錄鏡像 `~/.claude/plugins` 的結構：

```
$CLAUDE_CODE_PLUGIN_SEED_DIR/
  known_marketplaces.json
  marketplaces/<name>/...
  cache/<marketplace>/<plugin>/<version>/...
```

建立種子目錄的最簡單方法是在映像建置期間執行 Claude Code 一次，安裝您需要的 plugin，然後將產生的 `~/.claude/plugins` 目錄複製到您的映像中，並將 `CLAUDE_CODE_PLUGIN_SEED_DIR` 指向它。

若要跳過複製步驟，在建置期間將 `CLAUDE_CODE_PLUGIN_CACHE_DIR` 設定為您的目標種子路徑，以便 plugin 直接安裝到那裡：

```bash theme={null}
CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin marketplace add your-org/plugins
CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin install my-tool@your-plugins
```

然後在您的容器的執行時環境中設定 `CLAUDE_CODE_PLUGIN_SEED_DIR=/opt/claude-seed`，以便 Claude Code 在啟動時從種子讀取。

在啟動時，Claude Code 將種子的 `known_marketplaces.json` 中找到的 marketplace 註冊到主要配置中，並使用在 `cache/` 下找到的 plugin 快取，而無需重新複製。這在互動模式和使用 `-p` 旗標的非互動模式中都有效。

行為詳細資訊：

* **唯讀**：種子目錄永遠不會被寫入。自動更新對種子 marketplace 被停用，因為 git pull 在唯讀檔案系統上會失敗。
* **種子項目優先**：種子中宣告的 marketplace 在每次啟動時覆蓋使用者配置中的任何相符項目。若要選擇退出種子 plugin，請使用 `/plugin disable` 而不是移除 marketplace。
* **路徑解析**：Claude Code 在執行時透過探測 `$CLAUDE_CODE_PLUGIN_SEED_DIR/marketplaces/<name>/` 來定位 marketplace 內容，而不是信任儲存在種子 JSON 內的路徑。這表示即使在與建置位置不同的路徑上掛載，種子也能正確運作。
* **變更被阻止**：針對種子管理的 marketplace 執行 `/plugin marketplace remove` 或 `/plugin marketplace update` 會失敗，並提示您要求管理員更新種子映像。
* **與設定組合**：如果 `extraKnownMarketplaces` 或 `enabledPlugins` 宣告已存在於種子中的 marketplace，Claude Code 使用種子副本而不是複製。

<h3 id="managed-marketplace-restrictions">
  受管 marketplace 限制
</h3>

對於需要對 plugin 來源進行嚴格控制的組織，管理員可以使用受管設定中的 [`strictKnownMarketplaces`](/docs/zh-TW/settings#strictknownmarketplaces) 設定限制使用者允許新增的 plugin marketplace。若要也拒絕為單次執行側載 plugin、agent 和 MCP 伺服器的 CLI 旗標，請將其與 [`disableSideloadFlags`](/docs/zh-TW/settings#available-settings) 配對。若要允許清單化哪些 marketplace 的 plugin 可以顯示為內容相關安裝建議，請設定 [`pluginSuggestionMarketplaces`](/docs/zh-TW/settings#available-settings)。

當在受管設定中配置 `strictKnownMarketplaces` 時，限制行為取決於值：

| 值        | 行為                            |
| -------- | ----------------------------- |
| 未定義（預設）  | 無限制。使用者可以新增任何 marketplace     |
| 空陣列 `[]` | 完全鎖定。使用者無法新增任何新 marketplace   |
| 來源清單     | 使用者只能新增與允許清單完全相符的 marketplace |

<h4 id="common-configurations">
  常見配置
</h4>

停用所有 marketplace 新增：

```json theme={null}
{
  "strictKnownMarketplaces": []
}
```

僅允許特定 marketplace：

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "github",
      "repo": "acme-corp/approved-plugins"
    },
    {
      "source": "github",
      "repo": "acme-corp/security-tools",
      "ref": "v2.0"
    },
    {
      "source": "url",
      "url": "https://plugins.example.com/marketplace.json"
    }
  ]
}
```

使用主機上的正規表達式模式匹配允許來自內部 git 伺服器的所有 marketplace。這是 [GitHub Enterprise Server](/docs/zh-TW/github-enterprise-server#plugin-marketplaces-on-ghes) 或自託管 GitLab 執行個體的推薦方法：

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "hostPattern",
      "hostPattern": "^github\\.example\\.com$"
    }
  ]
}
```

使用路徑上的正規表達式模式匹配允許來自特定目錄的檔案系統型 marketplace：

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "pathPattern",
      "pathPattern": "^/opt/approved/"
    }
  ]
}
```

使用 `".*"` 作為 `pathPattern` 以允許任何檔案系統路徑，同時仍使用 `hostPattern` 控制網路來源。

<Note>
  `strictKnownMarketplaces` 限制使用者可以新增的內容，但不會自行註冊 marketplace。若要在不需要使用者執行 `/plugin marketplace add` 的情況下自動提供允許的 marketplace，請將其與同一 `managed-settings.json` 中的 [`extraKnownMarketplaces`](/docs/zh-TW/settings#extraknownmarketplaces) 配對。請參閱[同時使用兩者](/docs/zh-TW/settings#strictknownmarketplaces)。
</Note>

<h4 id="how-restrictions-work">
  限制如何運作
</h4>

限制在任何網路或檔案系統操作之前進行檢查。檢查在 marketplace 新增以及 plugin 安裝、更新、重新整理和自動更新時執行。如果 marketplace 在配置原則之前被新增，且其來源不再符合允許清單，Claude Code 會拒絕從中安裝或更新 plugin。相同的強制執行也適用於 `blockedMarketplaces`。

允許清單對大多數來源類型使用精確匹配。若要允許 marketplace，所有指定的欄位必須完全相符：

* 對於 GitHub 來源：`repo` 是必需的，如果在允許清單中指定，`ref` 或 `path` 也必須相符
* 對於 URL 來源：完整 URL 必須完全相符
* 對於 `hostPattern` 來源：marketplace 主機與正規表達式模式相符
* 對於 `pathPattern` 來源：marketplace 的檔案系統路徑與正規表達式模式相符

精確匹配不會正規化 URL：尾部斜線、`.git` 後綴或 `ssh://` 與 `https://` 形式被視為不同的值。如果您的組織 marketplace 可以透過多個 URL 形式複製，請優先使用 `hostPattern` 項目而不是字面 URL，以便所有形式都相符。

因為 `strictKnownMarketplaces` 在[受管設定](/docs/zh-TW/settings#settings-files)中設定，個別使用者和專案配置無法覆蓋這些限制。

有關完整的配置詳細資訊，包括所有支援的來源類型和與 `extraKnownMarketplaces` 的比較，請參閱 [strictKnownMarketplaces 參考](/docs/zh-TW/settings#strictknownmarketplaces)。

<h3 id="version-resolution-and-release-channels">
  版本解析和發行通道
</h3>

Plugin 版本決定快取路徑和更新偵測：如果解析的版本與使用者已有的版本相符，`/plugin update` 和自動更新會跳過 plugin。

Claude Code 從以下第一個設定的項目解析 plugin 的版本：

1. plugin 的 `plugin.json` 中的 `version`
2. plugin 的 marketplace 項目中的 `version`
3. plugin 來源的 git 提交 SHA

對於 git 型來源類型 `github`、`url`、`git-subdir` 和 git 託管 marketplace 內的相對路徑，您可以完全省略 `version`，每個新提交都被視為新版本。這是內部或積極開發的 plugin 的最簡單設定。

<Warning>
  設定 `version` 會固定 plugin。如果 `plugin.json` 宣告 `"version": "1.0.0"`，推送新提交而不更改該字串對現有使用者沒有任何作用，因為 Claude Code 看到相同的版本並保留快取副本。在每次發行時提升該欄位，或省略它以使用提交 SHA。

  避免在 `plugin.json` 和 marketplace 項目中同時設定 `version`。Claude Code 總是無聲地使用 `plugin.json` 值，因此過時的 manifest 版本可能會掩蓋您在 `marketplace.json` 中設定的版本。
</Warning>

<h4 id="set-up-release-channels">
  設定發行通道
</h4>

若要為您的 plugin 支援「穩定」和「最新」發行通道，您可以設定兩個指向同一儲存庫的不同 ref 或 SHA 的 marketplace。然後，您可以透過[受管設定](/docs/zh-TW/settings#settings-files)將兩個 marketplace 指派給不同的使用者群組。

<Warning>
  每個通道必須解析為不同的版本。如果您使用明確版本，`plugin.json` 必須在每個固定的 ref 處宣告不同的 `version`。如果您省略 `version`，不同的提交 SHA 已經區分通道。如果兩個 ref 解析為相同的版本字串，Claude Code 會將它們視為相同並跳過更新。
</Warning>

<h5 id="example">
  範例
</h5>

```json theme={null}
{
  "name": "stable-tools",
  "plugins": [
    {
      "name": "code-formatter",
      "source": {
        "source": "github",
        "repo": "acme-corp/code-formatter",
        "ref": "stable"
      }
    }
  ]
}
```

```json theme={null}
{
  "name": "latest-tools",
  "plugins": [
    {
      "name": "code-formatter",
      "source": {
        "source": "github",
        "repo": "acme-corp/code-formatter",
        "ref": "latest"
      }
    }
  ]
}
```

<h5 id="assign-channels-to-user-groups">
  將通道指派給使用者群組
</h5>

透過受管設定將每個 marketplace 指派給適當的使用者群組。例如，穩定群組接收：

```json theme={null}
{
  "extraKnownMarketplaces": {
    "stable-tools": {
      "source": {
        "source": "github",
        "repo": "acme-corp/stable-tools"
      }
    }
  }
}
```

早期存取群組改為接收 `latest-tools`：

```json theme={null}
{
  "extraKnownMarketplaces": {
    "latest-tools": {
      "source": {
        "source": "github",
        "repo": "acme-corp/latest-tools"
      }
    }
  }
}
```

<h4 id="pin-dependency-versions">
  固定依賴版本
</h4>

Plugin 可以將其依賴限制在 semver 範圍內，以便依賴的更新不會破壞依賴 plugin。請參閱[限制 plugin 依賴版本](/docs/zh-TW/plugin-dependencies)以了解 `{plugin-name}--v{version}` git 標籤慣例、範圍語法，以及如何組合對同一依賴的多個限制。

<h3 id="rename-or-remove-a-plugin">
  重新命名或移除 plugin
</h3>

Plugin 的 `name` 是其穩定識別碼。使用者在 `enabledPlugins`、`pluginConfigs` 和 `/plugin install` 命令中參考它，因此更改它會破壞每個現有安裝。若要更改 UI 中顯示的標籤而不破壞安裝，請設定 [`displayName`](#optional-plugin-fields) 並保持 `name` 不變。

如果您必須更改 plugin 的 `name`，或您從 `plugins` 陣列中移除 plugin，請新增頂層 `renames` 項目，以便現有使用者遷移而不是看到 `plugin-not-found` 錯誤。自動遷移需要 Claude Code v2.1.193 或更新版本。將每個前名稱對應到其目前名稱，或如果 plugin 不再存在，則對應到 `null`。以下範例將 `formatter` 重新命名為 `code-formatter`，並記錄 `legacy-linter` 已被移除：

```json theme={null}
{
  "name": "acme-tools",
  "owner": { "name": "Acme" },
  "plugins": [
    { "name": "code-formatter", "source": "./plugins/code-formatter" }
  ],
  "renames": {
    "formatter": "code-formatter",
    "legacy-linter": null
  }
}
```

當使用者啟動 Claude Code 時舊名稱仍在其設定中，Claude Code 會遵循 `renames` 對應：

* 如果項目指向新名稱，Claude Code 會在其新名稱下載入 plugin，並顯示一行通知，例如 `已在 "acme-tools" marketplace 中重新命名為 "code-formatter"`。然後它會在使用者、專案和本機設定範圍中重寫舊金鑰為新金鑰，用於 `enabledPlugins` 和 `pluginConfigs`，因此通知只出現一次。
* 對於 `null` 項目，Claude Code 會刪除舊金鑰，通知報告 plugin 已從 marketplace 中移除。
* 如果重新命名的 plugin 使用遠端來源，例如 `github` 或 `npm`，Claude Code 在重新命名後報告 `plugin-cache-miss`，使用者必須執行 `/plugin install` 一次以在新名稱下取得它。

將 `renames` 視為僅附加歷史記錄：即使在您預期每個使用者都已遷移後，也要保持舊項目就位。Claude Code 遵循鏈，因此如果您稍後將 `code-formatter` 重新命名為 `formatter-pro`，請新增第二項而不是編輯第一項。仍然啟用原始 `formatter` 的使用者然後透過兩項解析到 `formatter-pro`。

在編輯對應後執行 `claude plugin validate .`；它會拒絕任何鏈形成循環或不終止於 `null` 或 `plugins` 中列出的名稱的項目。

<Note>
  受管和原則設定對 Claude Code 是唯讀的，因此在那裡啟用的 plugin 無法自動重寫。重新命名的 plugin 仍在每個工作階段載入，但重新命名通知會重複出現，直到管理員更新受管設定檔案中的 `enabledPlugins` 以使用新名稱。相同的情況也適用於透過其他唯讀來源（例如 `--add-dir`）啟用的 plugin。
</Note>

Claude Code 的早期版本會忽略 `renames` 欄位，並為舊名稱報告 `plugin-not-found`。

<h2 id="validation-and-testing">
  驗證和測試
</h2>

在分享前測試您的 marketplace。

驗證您的 marketplace JSON 語法：

```bash theme={null}
claude plugin validate .
```

或從 Claude Code 內：

```shell theme={null}
/plugin validate .
```

新增 marketplace 進行測試：

```shell theme={null}
/plugin marketplace add ./path/to/marketplace
```

安裝測試 plugin 以驗證一切正常運作：

```shell theme={null}
/plugin install test-plugin@marketplace-name
```

有關完整的 plugin 測試工作流程，請參閱[在本機測試您的 plugin](/docs/zh-TW/plugins#test-your-plugins-locally)。有關技術疑難排解，請參閱 [Plugins reference](/docs/zh-TW/plugins-reference)。

<h2 id="manage-marketplaces-from-the-cli">
  從 CLI 管理 marketplace
</h2>

Claude Code 提供非互動式 `claude plugin marketplace` 子命令用於指令碼和自動化。這些等同於互動式工作階段內可用的 `/plugin marketplace` 命令。

<h3 id="plugin-marketplace-add">
  Plugin marketplace add
</h3>

從 GitHub 儲存庫、git URL、遠端 URL 或本機路徑新增 marketplace。

```bash theme={null}
claude plugin marketplace add <source> [options]
```

**引數：**

* `<source>`：GitHub `owner/repo` 簡寫、git URL、遠端 URL 到 `marketplace.json` 檔案或本機目錄路徑。若要固定到分支或標籤，請將 `@ref` 附加到 GitHub 簡寫或 `#ref` 附加到 git URL

URL 必須包含其配置。自 Claude Code v2.1.196 起，未輸入配置的主機（例如 `gitlab.example.com/team/plugins`）會被拒絕為無效的 `owner/repo` 簡寫，錯誤訊息會告訴您新增 `https://` 或使用 `./` 作為本機路徑。較早的版本會將其誤讀為 GitHub 儲存庫路徑，並在複製時因 GitHub 找不到錯誤而失敗。

**選項：**

| 選項                    | 描述                                                                                                                                  | 預設     |
| :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------- | :----- |
| `--scope <scope>`     | 宣告 marketplace 的位置：`user`、`project` 或 `local`。請參閱 [Plugin installation scopes](/docs/zh-TW/plugins-reference#plugin-installation-scopes) | `user` |
| `--sparse <paths...>` | 透過 git sparse-checkout 限制簽出到特定目錄。對 monorepo 很有用                                                                                     |        |

從 GitHub 使用 `owner/repo` 簡寫新增 marketplace：

```bash theme={null}
claude plugin marketplace add acme-corp/claude-plugins
```

使用 `@ref` 固定到特定分支或標籤：

```bash theme={null}
claude plugin marketplace add acme-corp/claude-plugins@v2.0
```

從非 GitHub 主機上的 git URL 新增：

```bash theme={null}
claude plugin marketplace add https://gitlab.example.com/team/plugins.git
```

從直接提供 `marketplace.json` 檔案的遠端 URL 新增：

```bash theme={null}
claude plugin marketplace add https://example.com/marketplace.json
```

從本機目錄新增以進行測試：

```bash theme={null}
claude plugin marketplace add ./my-marketplace
```

在專案範圍宣告 marketplace，以便透過 `.claude/settings.json` 與您的團隊共享：

```bash theme={null}
claude plugin marketplace add acme-corp/claude-plugins --scope project
```

對於 monorepo，限制簽出到包含 plugin 內容的目錄：

```bash theme={null}
claude plugin marketplace add acme-corp/monorepo --sparse .claude-plugin plugins
```

<h3 id="plugin-marketplace-list">
  Plugin marketplace list
</h3>

列出所有已配置的 marketplace。

```bash theme={null}
claude plugin marketplace list [options]
```

**選項：**

| 選項       | 描述       |
| :------- | :------- |
| `--json` | 輸出為 JSON |

使用 `--json` 時，每個項目包括 `name`、`source` 和來源特定的欄位：GitHub 來源的 `repo`、git 和 URL 來源的 `url`，以及本機來源的 `path`。當 marketplace 使用固定的分支或標籤新增時，GitHub 和 git 來源也包括 `ref` 欄位。

<h3 id="plugin-marketplace-remove">
  Plugin marketplace remove
</h3>

移除已配置的 marketplace。別名 `rm` 也被接受。

```bash theme={null}
claude plugin marketplace remove <name> [options]
```

**引數：**

* `<name>`：marketplace 名稱以移除，如 `claude plugin marketplace list` 所示。這是 `marketplace.json` 中的 `name`，而不是您傳遞給 `add` 的來源

**選項：**

| 選項                | 描述                                                                                                                                                                                                                   | 預設     |
| :---------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----- |
| `--scope <scope>` | 限制移除到單一設定範圍：`user`、`project` 或 `local`。請參閱 [Plugin installation scopes](/docs/zh-TW/plugins-reference#plugin-installation-scopes)。省略時，宣告會從每個可編輯的範圍中移除。指定時，只會移除該範圍的宣告；當 marketplace 仍在另一個範圍中宣告時，共享狀態、快取和已安裝的 plugin 資料會被保留 | (所有範圍) |

<Warning>
  從其最後剩餘的範圍移除 marketplace 也會卸載您從中安裝的任何 plugin。若要重新整理 marketplace 而不失去已安裝的 plugin，請改用 `claude plugin marketplace update`。
</Warning>

<h3 id="plugin-marketplace-update">
  Plugin marketplace update
</h3>

從其來源重新整理 marketplace 以檢索新 plugin 和版本變更。使用分支或標籤 `ref` 新增的 marketplace 會更新到該 ref 的最新提交，而不是儲存庫的預設分支。

```bash theme={null}
claude plugin marketplace update [name]
```

**引數：**

* `[name]`：marketplace 名稱以更新，如 `claude plugin marketplace list` 所示。如果省略，更新所有 marketplace

`remove` 和 `update` 在針對種子管理的 marketplace 執行時都會失敗，該 marketplace 是唯讀的。更新所有 marketplace 時，種子管理的項目被跳過，其他 marketplace 仍然更新。若要變更種子提供的 plugin，請要求您的管理員更新種子映像。請參閱[為容器預先填充 plugin](#pre-populate-plugins-for-containers)。

<h2 id="troubleshooting">
  疑難排解
</h2>

<h3 id="marketplace-not-loading">
  Marketplace 未載入
</h3>

**症狀**：無法新增 marketplace 或看不到其中的 plugin

**解決方案**：

* 驗證 marketplace URL 可存取
* 檢查 `.claude-plugin/marketplace.json` 是否存在於指定路徑
* 使用 `claude plugin validate` 或 `/plugin validate` 確保 JSON 語法有效。若要檢查 skill、agent 和 command frontmatter，請針對每個 plugin 目錄執行該命令
* 對於私人儲存庫，確認您有存取權限

<h3 id="marketplace-validation-errors">
  Marketplace 驗證錯誤
</h3>

從您的 marketplace 目錄執行 `claude plugin validate .` 或 `/plugin validate .` 以檢查問題。當指向 marketplace 目錄時，驗證器檢查 `marketplace.json` 是否有架構錯誤、重複的 plugin 名稱和來源路徑遍歷。對於每個 `source` 為本機路徑的項目，它也會驗證該 plugin 自己的 `plugin.json`，並在項目的 `version` 與 `plugin.json` 中的版本不符時發出警告。在 plugin 的 `plugin.json` 中發現的問題會以項目索引作為前綴，形式為 `plugins[2] plugin.json →`。

自 Claude Code v2.1.196 起，每個項目的檢查也會：

* 包含 `source` 為 `.` 的 plugin
* 在 `marketplace.json` 位於 `.claude-plugin` 目錄外時執行，針對檔案自己的目錄解析來源
* 即使檔案的另一部分有架構錯誤，也會報告每個項目的問題

較早的版本會跳過 marketplace 根目錄中的 plugin，並且只從 `.claude-plugin/marketplace.json` 開始下降。

若要驗證個別 plugin 的 `plugin.json` 及其 skill、agent、command 和 hook 檔案，請針對 plugin 目錄本身執行該命令，例如 `claude plugin validate ./plugins/my-plugin`。常見錯誤：

| 錯誤                                                | 原因                                 | 解決方案                                                                   |
| :------------------------------------------------ | :--------------------------------- | :--------------------------------------------------------------------- |
| `File not found: .claude-plugin/marketplace.json` | 缺少 manifest                        | 使用必需欄位建立 `.claude-plugin/marketplace.json`                             |
| `Invalid JSON syntax: Unexpected token...`        | JSON 語法錯誤在 marketplace.json 中      | 檢查缺少的逗號、多餘的逗號或未引用的字串                                                   |
| `Duplicate plugin name "x" found in marketplace`  | 兩個 plugin 共享相同名稱                   | 為每個 plugin 指定唯一的 `name` 值                                              |
| `plugins[0].source: Path contains ".."`           | 來源路徑包含 `..`                        | 使用相對於 marketplace 根目錄的路徑，不含 `..`。請參閱[相對路徑](#relative-paths)            |
| `YAML frontmatter failed to parse: ...`           | skill、agent 或 command 檔案中的 YAML 無效 | 修正 frontmatter 區塊中的 YAML 語法。在執行時，此檔案載入時不含中繼資料。僅在驗證 plugin 目錄時報告        |
| `Invalid JSON syntax: ...`（hooks.json）            | 格式不正確的 `hooks/hooks.json`          | 修正 JSON 語法。格式不正確的 `hooks/hooks.json` 會防止整個 plugin 載入。僅在驗證 plugin 目錄時報告 |

**警告**（非阻止性）：

* `Marketplace has no plugins defined`：將至少一個 plugin 新增到 `plugins` 陣列
* `No marketplace description provided`：新增頂層 `description` 以幫助使用者瞭解您的 marketplace
* `Plugin name "x" is not kebab-case`：plugin 名稱包含大寫字母、空格或特殊字元。重新命名為僅包含小寫字母、數字和連字號（例如，`my-plugin`）。Claude Code 接受其他形式，但 claude.ai marketplace 同步會拒絕它們。

<h3 id="plugin-installation-failures">
  Plugin 安裝失敗
</h3>

**症狀**：Marketplace 出現但 plugin 安裝失敗

**解決方案**：

* 驗證 plugin 來源 URL 可存取
* 檢查 plugin 目錄是否包含必需的檔案
* 對於 GitHub 來源，確保儲存庫是公開的或您有存取權
* 透過手動複製/下載測試 plugin 來源
* 如果來源同時固定 `ref` 和 `sha`，已刪除的上游分支或標籤不會阻止在大多數 git 主機（包括 GitHub、GitLab 和 Bitbucket）上的安裝。在不支援按 SHA 擷取提交的伺服器上（例如 AWS CodeCommit），`ref` 仍必須存在，且固定的提交必須可從其到達。如果安裝仍然失敗，請確認固定的提交仍然存在於儲存庫中

<h3 id="private-repository-authentication-fails">
  私人儲存庫驗證失敗
</h3>

**症狀**：從私人儲存庫安裝 plugin 時出現驗證錯誤

**解決方案**：

對於手動安裝和更新：

* 驗證您已使用您的 git 提供者進行驗證（例如，為 GitHub 執行 `gh auth status`）
* 檢查您的認證助手是否正確配置：`git config --global credential.helper`
* 嘗試手動複製儲存庫以驗證您的認證有效

對於背景自動更新：

* 根據預設，背景重新整理會停用 git 認證助手以進行拉取，因此拉取無法透過 HTTPS 進行驗證。具有在 `ssh-agent` 中載入的金鑰的 SSH 遠端仍會進行驗證。失敗的拉取會觸發從頭開始的重新複製，這會使用您儲存的認證，但在大型儲存庫上可能會逾時
* 設定 `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1` 以在背景拉取失敗時保留現有複製
* 配置 git 認證助手（例如 `gh auth setup-git`），以便重新複製後備可以進行驗證
* 如果重新複製在大型儲存庫上逾時，請使用 [`CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS`](#git-operations-time-out) 增加限制
* 配置範圍限定於 marketplace 儲存庫的 [git URL 重寫](#private-repositories)，以便背景拉取直接進行驗證
* 或使用 `/plugin marketplace update <name>` 手動更新私人 marketplace，這會使用您的認證

<h3 id="marketplace-updates-fail-in-offline-environments">
  Marketplace 更新在離線環境中失敗
</h3>

**症狀**：Marketplace `git pull` 在背景中失敗，Claude Code 重複嘗試無法成功的重新複製。

**原因**：根據預設，當 `git pull` 失敗時，Claude Code 會嘗試從頭開始重新複製。在離線或隔離環境中，重新複製以相同方式失敗，之後先前快取的還原是盡力而為。重新整理在啟動後在背景中執行，因此不會延遲啟動，但每個工作階段都會重複失敗的嘗試，每個 git 操作都可以等待 [120 秒逾時](#git-operations-time-out)。

**解決方案**：設定 `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1` 以在拉取失敗時跳過重新複製嘗試並繼續使用現有快取：

```bash theme={null}
export CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1
```

設定此變數後，Claude Code 在 `git pull` 失敗時保留過時的 marketplace 複製，並繼續使用最後已知的良好狀態。對於儲存庫永遠無法到達的完全離線部署，請改用 [`CLAUDE_CODE_PLUGIN_SEED_DIR`](#pre-populate-plugins-for-containers) 在建置時預先填充 plugin 目錄。

<h3 id="git-operations-time-out">
  Git 操作逾時
</h3>

**症狀**：Plugin 安裝或 marketplace 更新失敗，出現逾時錯誤，例如「Git clone timed out after 120s」或「Git pull timed out after 120s」。

**原因**：Claude Code 對所有 git 操作（包括複製 plugin 儲存庫和拉取 marketplace 更新）使用 120 秒逾時。大型儲存庫或緩慢的網路連線可能超過此限制。

**解決方案**：使用 `CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS` 環境變數增加逾時。值以毫秒為單位：

```bash theme={null}
export CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS=300000  # 5 分鐘
```

<h3 id="plugins-with-relative-paths-fail-in-url-based-marketplaces">
  相對路徑 plugin 在基於 URL 的 marketplace 中失敗
</h3>

**症狀**：透過 URL（例如 `https://example.com/marketplace.json`）新增 marketplace，但具有相對路徑來源（如 `"./plugins/my-plugin"`）的 plugin 無法安裝，出現「path not found」錯誤。

**原因**：基於 URL 的 marketplace 僅下載 `marketplace.json` 檔案本身。它們不從伺服器下載 plugin 檔案。marketplace 項目中的相對路徑參考未下載的遠端伺服器上的檔案。

**解決方案**：

* **使用外部來源**：將 plugin 項目變更為使用 GitHub、npm 或 git URL 來源，而不是相對路徑：
  ```json theme={null}
  { "name": "my-plugin", "source": { "source": "github", "repo": "owner/repo" } }
  ```
* **使用基於 Git 的 marketplace**：在 Git 儲存庫中託管您的 marketplace 並使用 git URL 新增它。基於 Git 的 marketplace 複製整個儲存庫，使相對路徑正常運作。

<h3 id="files-not-found-after-installation">
  安裝後找不到檔案
</h3>

**症狀**：Plugin 安裝但對檔案的參考失敗，特別是 plugin 目錄外的檔案

**原因**：Plugin 被複製到快取目錄而不是就地使用。參考 plugin 目錄外檔案的路徑（例如 `../shared-utils`）無法運作，因為這些檔案不會被複製。

**解決方案**：有關解決方案（包括符號連結和目錄重組），請參閱 [Plugin caching and file resolution](/docs/zh-TW/plugins-reference#plugin-caching-and-file-resolution)。

有關其他偵錯工具和常見問題，請參閱 [Debugging and development tools](/docs/zh-TW/plugins-reference#debugging-and-development-tools)。

<h2 id="see-also">
  另請參閱
</h2>

* [探索並安裝預先建立的 plugins](/docs/zh-TW/discover-plugins) - 從現有 marketplace 安裝 plugins
* [Plugins](/docs/zh-TW/plugins) - 建立您自己的 plugins
* [Plugins reference](/docs/zh-TW/plugins-reference) - 完整的技術規格和架構
* [Plugin settings](/docs/zh-TW/settings#plugin-settings) - Plugin 配置選項
* [strictKnownMarketplaces reference](/docs/zh-TW/settings#strictknownmarketplaces) - 受管 marketplace 限制
