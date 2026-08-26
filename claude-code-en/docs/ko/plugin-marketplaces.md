> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 플러그인 마켓플레이스 생성 및 배포

> Claude Code 확장 프로그램을 팀과 커뮤니티에 배포하기 위한 플러그인 마켓플레이스를 구축하고 호스팅합니다.

**플러그인 마켓플레이스**는 다른 사용자에게 플러그인을 배포할 수 있는 카탈로그입니다. 마켓플레이스는 중앙 집중식 검색, 버전 추적, 자동 업데이트 및 git 저장소와 로컬 경로를 포함한 여러 소스 유형을 지원합니다. 이 가이드에서는 팀이나 커뮤니티와 플러그인을 공유하기 위해 자신의 마켓플레이스를 만드는 방법을 보여줍니다.

기존 마켓플레이스에서 플러그인을 설치하려고 하시나요? [미리 빌드된 플러그인 검색 및 설치](/docs/ko/discover-plugins)를 참조하세요.

<h2 id="overview">
  개요
</h2>

마켓플레이스를 생성하고 배포하는 과정은 다음과 같습니다:

1. **플러그인 생성**: skills, 에이전트, hooks, MCP 서버 또는 LSP 서버를 사용하여 하나 이상의 플러그인을 빌드합니다. 이 가이드에서는 배포할 플러그인이 이미 있다고 가정합니다. 플러그인 생성 방법에 대한 자세한 내용은 [플러그인 생성](/docs/ko/plugins)을 참조하세요.
2. **마켓플레이스 파일 생성**: 플러그인을 나열하고 플러그인을 찾을 위치를 정의하는 `marketplace.json`을 정의합니다. [마켓플레이스 파일 생성](#create-the-marketplace-file)을 참조하세요.
3. **마켓플레이스 호스팅**: GitHub, GitLab 또는 다른 git 호스트에 푸시합니다. [마켓플레이스 호스팅 및 배포](#host-and-distribute-marketplaces)를 참조하세요.
4. **사용자와 공유**: 사용자가 `/plugin marketplace add`로 마켓플레이스를 추가하고 개별 플러그인을 설치합니다. [플러그인 검색 및 설치](/docs/ko/discover-plugins)를 참조하세요.

마켓플레이스가 라이브 상태가 되면 저장소에 변경 사항을 푸시하여 업데이트할 수 있습니다. 사용자는 `/plugin marketplace update`로 로컬 복사본을 새로 고칩니다.

<h2 id="walkthrough-create-a-local-marketplace">
  연습: 로컬 마켓플레이스 생성
</h2>

이 예제에서는 하나의 플러그인으로 마켓플레이스를 생성합니다: 코드 리뷰를 위한 `quality-review` skill입니다. 디렉터리 구조를 생성하고, skill을 추가하고, 플러그인 매니페스트와 마켓플레이스 카탈로그를 생성한 다음, 설치하고 테스트합니다.

<Steps>
  <Step title="디렉터리 구조 생성">
    ```bash theme={null}
    mkdir -p my-marketplace/.claude-plugin
    mkdir -p my-marketplace/plugins/quality-review-plugin/.claude-plugin
    mkdir -p my-marketplace/plugins/quality-review-plugin/skills/quality-review
    ```
  </Step>

  <Step title="skill 생성">
    `quality-review` skill이 수행하는 작업을 정의하는 `SKILL.md` 파일을 생성합니다.

    ```markdown my-marketplace/plugins/quality-review-plugin/skills/quality-review/SKILL.md theme={null}
    ---
    description: Review code for bugs, security, and performance
    ---

    선택한 코드 또는 최근 변경 사항을 다음 항목에 대해 검토합니다:
    - 잠재적 버그 또는 엣지 케이스
    - 보안 문제
    - 성능 문제
    - 가독성 개선

    간결하고 실행 가능한 내용을 제공합니다.
    ```
  </Step>

  <Step title="플러그인 매니페스트 생성">
    플러그인을 설명하는 `plugin.json` 파일을 생성합니다. 매니페스트는 `.claude-plugin/` 디렉터리에 위치합니다.

    ```json my-marketplace/plugins/quality-review-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "quality-review-plugin",
      "description": "Adds a quality-review skill for quick code reviews",
      "version": "1.0.0"
    }
    ```

    <Note>
      `version`을 설정하면 사용자는 이 필드를 변경할 때만 업데이트를 받으므로, 모든 릴리스에서 이를 증가시킵니다. `version`을 생략하고 이 마켓플레이스를 git에서 호스팅하면, 모든 커밋이 자동으로 새 버전으로 계산됩니다. [버전 해석](#version-resolution-and-release-channels)을 참조하여 올바른 접근 방식을 선택합니다.
    </Note>
  </Step>

  <Step title="마켓플레이스 파일 생성">
    플러그인을 나열하는 마켓플레이스 카탈로그를 생성합니다.

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
          "description": "Adds a quality-review skill for quick code reviews"
        }
      ]
    }
    ```
  </Step>

  <Step title="추가 및 설치">
    마켓플레이스를 추가하고 플러그인을 설치합니다.

    ```shell theme={null}
    /plugin marketplace add ./my-marketplace
    /plugin install quality-review-plugin@my-plugins
    ```
  </Step>

  <Step title="시도해보기">
    편집기에서 일부 코드를 선택하고 새 skill을 실행합니다. 플러그인 skill은 플러그인 이름으로 네임스페이스됩니다.

    ```shell theme={null}
    /quality-review-plugin:quality-review
    ```
  </Step>
</Steps>

플러그인이 수행할 수 있는 작업(hooks, 에이전트, MCP 서버 및 LSP 서버 포함)에 대해 자세히 알아보려면 [플러그인](/docs/ko/plugins)을 참조하세요.

<Note>
  **플러그인 설치 방법**: 사용자가 플러그인을 설치하면 Claude Code는 플러그인 디렉터리를 캐시 위치에 복사합니다. 이는 `../shared-utils`와 같은 경로를 사용하여 플러그인 디렉터리 외부의 파일을 참조할 수 없다는 의미입니다. 왜냐하면 해당 파일이 복사되지 않기 때문입니다.

  플러그인 간에 파일을 공유해야 하는 경우 symlink를 사용합니다. 자세한 내용은 [플러그인 캐싱 및 파일 해석](/docs/ko/plugins-reference#plugin-caching-and-file-resolution)을 참조하세요.
</Note>

<h2 id="create-the-marketplace-file">
  마켓플레이스 파일 생성
</h2>

저장소 루트에 `.claude-plugin/marketplace.json`을 생성합니다. 이 파일은 마켓플레이스의 이름, 소유자 정보 및 소스가 있는 플러그인 목록을 정의합니다.

각 플러그인 항목에는 최소한 `name`과 `source`(Claude Code가 가져올 위치를 알려주는)가 필요합니다. 사용 가능한 모든 필드는 아래의 [전체 스키마](#marketplace-schema)를 참조하세요.

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
      "description": "저장 시 자동 코드 포맷팅",
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
      "description": "배포 자동화 도구"
    }
  ]
}
```

<h2 id="marketplace-schema">
  마켓플레이스 스키마
</h2>

<h3 id="required-fields">
  필수 필드
</h3>

| 필드        | 유형     | 설명                                                                                                                                                                                                                                                                                               | 예제             |
| :-------- | :----- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------- |
| `name`    | string | 마켓플레이스 식별자(kebab-case, 공백 없음). 이는 공개 대면입니다: 사용자는 플러그인을 설치할 때 이를 봅니다(예: `/plugin install my-tool@your-marketplace`). 각 사용자는 이름당 하나의 마켓플레이스만 등록할 수 있습니다: 동일한 이름으로 두 번째 마켓플레이스를 추가하면 첫 번째를 대체합니다. 하나의 마켓플레이스 이름 아래에 여러 플러그인을 게시하려면 [단일 `marketplace.json`](#create-the-marketplace-file)에 모두 나열하세요. | `"acme-tools"` |
| `owner`   | object | 마켓플레이스 유지 관리자 정보([아래 필드 참조](#owner-fields))                                                                                                                                                                                                                                                      |                |
| `plugins` | array  | 사용 가능한 플러그인 목록                                                                                                                                                                                                                                                                                   | 아래 참조          |

<Note>
  **예약된 이름**: 다음 마켓플레이스 이름은 공식 Anthropic 사용을 위해 예약되어 있으며 타사 마켓플레이스에서 사용할 수 없습니다: `claude-code-marketplace`, `claude-code-plugins`, `claude-plugins-official`, `claude-plugins-community`, `claude-community`, `anthropic-marketplace`, `anthropic-plugins`, `agent-skills`, `anthropic-agent-skills`, `knowledge-work-plugins`, `life-sciences`, `claude-for-legal`, `claude-for-financial-services`, `financial-services-plugins`, `first-party-plugins`, `healthcare`. 공식 마켓플레이스를 사칭하는 이름(예: `official-claude-plugins` 또는 `anthropic-plugins-v2`)도 차단됩니다. 이러한 이름을 예약하면 타사 마켓플레이스가 자신을 Anthropic 게시 소스로 제시하는 것을 방지합니다.

  Claude Code는 마켓플레이스를 추가할 때뿐만 아니라 마켓플레이스를 로드할 때마다 예약된 이름을 다시 확인합니다. 이름이 예약되기 전에 이러한 이름 중 하나로 등록된 마켓플레이스는 로드를 중지하고 [신뢰할 수 없는 소스에서 등록됨](/docs/ko/errors#marketplace-is-registered-from-an-untrusted-source)을 보고합니다. 해당 마켓플레이스를 제거하고 공식 Anthropic 소스에서 다시 추가하세요. 새로 예약된 이름의 영향을 받는 타사 마켓플레이스는 다른 이름으로 다시 추가하는 즉시 다시 로드됩니다. v2.1.205 이전에는 `first-party-plugins` 및 `healthcare`가 예약되지 않았으며, 예약된 이름으로 이미 등록된 마켓플레이스는 계속 로드되었습니다.
</Note>

<h3 id="owner-fields">
  소유자 필드
</h3>

| 필드      | 유형     | 필수  | 설명              |
| :------ | :----- | :-- | :-------------- |
| `name`  | string | 예   | 유지 관리자 또는 팀의 이름 |
| `email` | string | 아니오 | 유지 관리자의 연락처 이메일 |

<h3 id="optional-fields">
  선택적 필드
</h3>

| 필드                                    | 유형     | 설명                                                                                                                                                                                                          |
| :------------------------------------ | :----- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `$schema`                             | string | 편집기 자동 완성 및 유효성 검사를 위한 JSON Schema URL입니다. Claude Code는 로드 시 이 필드를 무시합니다.                                                                                                                                   |
| `description`                         | string | 간단한 마켓플레이스 설명                                                                                                                                                                                               |
| `version`                             | string | 마켓플레이스 매니페스트 버전                                                                                                                                                                                             |
| `metadata.pluginRoot`                 | string | 상대 플러그인 소스 경로에 앞에 붙는 기본 디렉터리(예: `"./plugins"`를 사용하면 `"source": "./plugins/formatter"` 대신 `"source": "formatter"`를 작성할 수 있습니다)                                                                               |
| `allowCrossMarketplaceDependenciesOn` | array  | 이 마켓플레이스의 플러그인이 의존할 수 있는 다른 마켓플레이스입니다. 여기에 나열되지 않은 마켓플레이스의 종속성은 설치 시 차단됩니다. [다른 마켓플레이스의 플러그인에 의존](/docs/ko/plugin-dependencies#depend-on-a-plugin-from-another-marketplace)을 참조하세요.                              |
| `renames`                             | object | 이전 플러그인 `name`을 현재 이름으로 매핑하거나, 플러그인이 제거된 경우 `null`로 매핑합니다. `plugins`의 항목을 이름 변경하거나 제거할 때 기존 사용자가 자동으로 마이그레이션되도록 합니다. [플러그인 이름 변경 또는 제거](#rename-or-remove-a-plugin)를 참조하세요. Claude Code v2.1.193 이상이 필요합니다. |

`description` 및 `version`은 이전 버전과의 호환성을 위해 `metadata` 아래에서도 허용됩니다.

<h2 id="plugin-entries">
  플러그인 항목
</h2>

`plugins` 배열의 각 플러그인 항목은 플러그인과 플러그인을 찾을 위치를 설명합니다. [플러그인 매니페스트 스키마](/docs/ko/plugins-reference#plugin-manifest-schema)의 모든 필드(예: `description`, `version`, `author`, `commands`, `hooks` 등)와 이러한 마켓플레이스 특정 필드를 포함할 수 있습니다: `source`, `category`, `tags`, `strict`, 및 `relevance`.

<h3 id="required-fields-2">
  필수 필드
</h3>

| 필드       | 유형             | 설명                                                                                                       |
| :------- | :------------- | :------------------------------------------------------------------------------------------------------- |
| `name`   | string         | 플러그인 식별자(kebab-case, 공백 없음). 이는 공개 대면입니다: 사용자는 설치할 때 이를 봅니다(예: `/plugin install my-plugin@marketplace`). |
| `source` | string\|object | 플러그인을 가져올 위치([아래 플러그인 소스](#plugin-sources) 참조)                                                           |

<h3 id="optional-plugin-fields">
  선택적 플러그인 필드
</h3>

**표준 메타데이터 필드:**

| 필드               | 유형      | 설명                                                                                                                                                                                                                      |
| :--------------- | :------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `displayName`    | string  | UI 표면에 표시되는 사람이 읽을 수 있는 이름입니다. 생략하면 `name`으로 돌아갑니다. 공백과 모든 대소문자를 포함할 수 있습니다. 네임스페이싱이나 조회에 사용되지 않습니다. Claude Code v2.1.143 이상이 필요합니다.                                                                                    |
| `description`    | string  | 간단한 플러그인 설명                                                                                                                                                                                                             |
| `version`        | string  | 플러그인 버전. 설정된 경우(여기 또는 `plugin.json`에서), 플러그인은 이 문자열로 고정되며 사용자는 변경될 때만 업데이트를 받습니다. 생략하면 git 커밋 SHA로 돌아갑니다. [버전 해석](#version-resolution-and-release-channels)을 참조하세요.                                                     |
| `author`         | object  | 플러그인 작성자 정보(`name` 필수, `email` 선택)                                                                                                                                                                                      |
| `homepage`       | string  | 플러그인 홈페이지 또는 문서 URL                                                                                                                                                                                                     |
| `repository`     | string  | 소스 코드 저장소 URL                                                                                                                                                                                                           |
| `license`        | string  | SPDX 라이선스 식별자(예: MIT, Apache-2.0)                                                                                                                                                                                       |
| `keywords`       | array   | 플러그인 검색 및 분류를 위한 태그                                                                                                                                                                                                     |
| `category`       | string  | 조직을 위한 플러그인 카테고리                                                                                                                                                                                                        |
| `tags`           | array   | 검색 가능성을 위한 태그                                                                                                                                                                                                           |
| `strict`         | boolean | `plugin.json`이 구성 요소 정의의 권한인지 여부를 제어합니다(기본값: true). 아래의 [Strict 모드](#strict-mode)를 참조하세요.                                                                                                                               |
| `relevance`      | object  | Claude Code가 사용자에게 이 플러그인을 제안할 시기를 알려주는 신호입니다. 관리자가 관리 설정에서 허용 목록에 추가한 마켓플레이스에만 적용됩니다. [조직을 위한 플러그인 권장](/docs/ko/plugin-relevance)을 참조하세요. Claude Code v2.1.152 이상이 필요합니다.                                                   |
| `defaultEnabled` | boolean | 플러그인이 설치 후 활성화되는지 여부(기본값: true). 사용자가 옵트인할 때까지 플러그인을 비활성화된 상태로 설치하려면 `false`로 설정합니다. 플러그인의 `plugin.json`에 있는 동일한 필드보다 우선합니다. [기본 활성화](/docs/ko/plugins-reference#default-enablement)를 참조하세요. Claude Code v2.1.154 이상이 필요합니다. |

**구성 요소 구성 필드:**

| 필드           | 유형             | 설명                                            |
| :----------- | :------------- | :-------------------------------------------- |
| `skills`     | string\|array  | `<name>/SKILL.md`를 포함하는 skill 디렉터리의 사용자 정의 경로 |
| `commands`   | string\|array  | 평면 `.md` skill 파일 또는 디렉터리의 사용자 정의 경로          |
| `agents`     | string\|array  | 에이전트 파일의 사용자 정의 경로                            |
| `hooks`      | string\|object | 사용자 정의 hooks 구성 또는 hooks 파일 경로                |
| `mcpServers` | string\|object | MCP 서버 구성 또는 MCP 구성 경로                        |
| `lspServers` | string\|object | LSP 서버 구성 또는 LSP 구성 경로                        |

<h2 id="plugin-sources">
  플러그인 소스
</h2>

플러그인 소스는 Claude Code에 마켓플레이스에 나열된 각 개별 플러그인을 가져올 위치를 알려줍니다. 이는 `marketplace.json`의 각 플러그인 항목의 `source` 필드에 설정됩니다.

Claude Code가 플러그인을 로컬 머신에 복제하거나 다운로드한 후, 플러그인을 `~/.claude/plugins/cache`의 로컬 버전 관리 플러그인 캐시에 복사합니다.

| 소스           | 유형                            | 필드                                 | 참고                                                                                         |
| ------------ | ----------------------------- | ---------------------------------- | ------------------------------------------------------------------------------------------ |
| 상대 경로        | `string` (예: `"./my-plugin"`) | 없음                                 | 마켓플레이스 저장소 내의 로컬 디렉터리. `./`로 시작해야 합니다. 마켓플레이스 루트에 상대적으로 해석되며, `.claude-plugin/` 디렉터리가 아닙니다 |
| `github`     | object                        | `repo`, `ref?`, `sha?`             |                                                                                            |
| `url`        | object                        | `url`, `ref?`, `sha?`              | Git URL 소스                                                                                 |
| `git-subdir` | object                        | `url`, `path`, `ref?`, `sha?`      | git 저장소 내의 하위 디렉터리. 모노레포의 대역폭을 최소화하기 위해 희소하게 복제합니다                                         |
| `npm`        | object                        | `package`, `version?`, `registry?` | `npm install`을 통해 설치됨                                                                      |

<Note>
  **마켓플레이스 소스 vs 플러그인 소스**: 이는 다양한 것을 제어하는 다양한 개념입니다.

  * **마켓플레이스 소스**: `marketplace.json` 카탈로그 자체를 가져올 위치. 사용자가 `/plugin marketplace add`를 실행하거나 `extraKnownMarketplaces` 설정에서 설정합니다. `ref`(분기/태그)를 지원하지만 `sha`는 지원하지 않습니다.
  * **플러그인 소스**: 마켓플레이스에 나열된 개별 플러그인을 가져올 위치. `marketplace.json` 내의 각 플러그인 항목의 `source` 필드에 설정됩니다. `ref`(분기/태그)와 `sha`(정확한 커밋) 모두를 지원합니다.

  예를 들어, `acme-corp/plugin-catalog`에서 호스팅되는 마켓플레이스(마켓플레이스 소스)는 `acme-corp/code-formatter`에서 가져온 플러그인을 나열할 수 있습니다(플러그인 소스). 마켓플레이스 소스와 플러그인 소스는 다양한 저장소를 가리키며 독립적으로 고정됩니다.
</Note>

아래의 git 기반 소스 유형은 `github`, `url`, 및 `git-subdir`입니다. `ref`와 `sha`가 모두 설정되면 `sha`가 유효한 핀입니다. Claude Code는 고정된 커밋을 직접 가져오고 체크아웃합니다.

GitHub, GitLab, Bitbucket을 포함한 대부분의 git 호스트에서 이는 분기 또는 태그가 업스트림에서 삭제되었더라도 커밋이 저장소에서 여전히 도달 가능한 한 설치가 성공함을 의미합니다. AWS CodeCommit과 같은 일부 서버는 SHA로 커밋을 가져오는 것을 지원하지 않습니다. 이러한 서버에서는 `ref`가 여전히 존재해야 하고 고정된 커밋이 이로부터 도달 가능해야 합니다.

<h3 id="relative-paths">
  상대 경로
</h3>

동일한 저장소의 플러그인의 경우 `./`로 시작하는 경로를 사용합니다:

```json theme={null}
{
  "name": "my-plugin",
  "source": "./plugins/my-plugin"
}
```

경로는 마켓플레이스 루트(`.claude-plugin/`을 포함하는 디렉터리)에 상대적으로 해석됩니다. 위의 예에서 `./plugins/my-plugin`은 `marketplace.json`이 `<repo>/.claude-plugin/marketplace.json`에 있더라도 `<repo>/plugins/my-plugin`을 가리킵니다. 마켓플레이스 루트 외부로 나가기 위해 `../`를 사용하지 마세요.

<Note>
  상대 경로는 git 소스 또는 로컬 디렉터리에서 마켓플레이스를 추가할 때 작동하므로, `marketplace.json` 파일에 대한 직접 URL을 통해 마켓플레이스를 추가하면 상대 경로가 해석되지 않습니다. 왜냐하면 해당 파일만 다운로드되기 때문입니다. URL 기반 배포의 경우 GitHub, npm 또는 git URL 소스를 대신 사용합니다. 자세한 내용은 [문제 해결](#plugins-with-relative-paths-fail-in-url-based-marketplaces)을 참조하세요.
</Note>

<h3 id="github-repositories">
  GitHub 저장소
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

특정 분기, 태그 또는 커밋에 고정할 수 있습니다:

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

| 필드     | 유형     | 설명                                    |
| :----- | :----- | :------------------------------------ |
| `repo` | string | 필수. `owner/repo` 형식의 GitHub 저장소       |
| `ref`  | string | 선택. Git 분기 또는 태그(저장소 기본 분기로 기본값)      |
| `sha`  | string | 선택. 정확한 버전에 고정하기 위한 전체 40자 git 커밋 SHA |

<h3 id="git-repositories">
  Git 저장소
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

특정 분기, 태그 또는 커밋에 고정할 수 있습니다:

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

| 필드    | 유형     | 설명                                                                                                              |
| :---- | :----- | :-------------------------------------------------------------------------------------------------------------- |
| `url` | string | 필수. 전체 git 저장소 URL(`https://` 또는 `git@`). `.git` 접미사는 선택 사항이므로 Azure DevOps 및 AWS CodeCommit URL(접미사 없음)이 작동합니다 |
| `ref` | string | 선택. Git 분기 또는 태그(저장소 기본 분기로 기본값)                                                                                |
| `sha` | string | 선택. 정확한 버전에 고정하기 위한 전체 40자 git 커밋 SHA                                                                           |

<h3 id="git-subdirectories">
  Git 하위 디렉터리
</h3>

`git-subdir`을 사용하여 git 저장소의 하위 디렉터리 내에 있는 플러그인을 가리킵니다. Claude Code는 희소하고 부분적인 복제를 사용하여 하위 디렉터리만 가져오므로 대규모 모노레포의 대역폭을 최소화합니다.

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

특정 분기, 태그 또는 커밋에 고정할 수 있습니다:

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

`url` 필드는 GitHub 단축형(`owner/repo`) 또는 SSH URL(`git@github.com:owner/repo.git`)도 허용합니다.

| 필드     | 유형     | 설명                                                           |
| :----- | :----- | :----------------------------------------------------------- |
| `url`  | string | 필수. Git 저장소 URL, GitHub `owner/repo` 단축형 또는 SSH URL          |
| `path` | string | 필수. 플러그인을 포함하는 저장소 내의 하위 디렉터리 경로(예: `"tools/claude-plugin"`) |
| `ref`  | string | 선택. Git 분기 또는 태그(저장소 기본 분기로 기본값)                             |
| `sha`  | string | 선택. 정확한 버전에 고정하기 위한 전체 40자 git 커밋 SHA                        |

<h3 id="npm-packages">
  npm 패키지
</h3>

npm 패키지로 배포되는 플러그인은 `npm install`을 사용하여 설치됩니다. 이는 공개 npm 레지스트리 또는 팀이 호스팅하는 개인 레지스트리의 모든 패키지에서 작동합니다.

```json theme={null}
{
  "name": "my-npm-plugin",
  "source": {
    "source": "npm",
    "package": "@acme/claude-plugin"
  }
}
```

특정 버전에 고정하려면 `version` 필드를 추가합니다:

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

개인 또는 내부 레지스트리에서 설치하려면 `registry` 필드를 추가합니다:

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

| 필드         | 유형     | 설명                                                            |
| :--------- | :----- | :------------------------------------------------------------ |
| `package`  | string | 필수. 패키지 이름 또는 범위 지정 패키지(예: `@org/plugin`)                     |
| `version`  | string | 선택. 버전 또는 버전 범위(예: `2.1.0`, `^2.0.0`, `~1.5.0`)               |
| `registry` | string | 선택. 사용자 정의 npm 레지스트리 URL. 시스템 npm 레지스트리(일반적으로 npmjs.org)로 기본값 |

<h3 id="advanced-plugin-entries">
  고급 플러그인 항목
</h3>

이 예제는 명령어, 에이전트, hooks 및 MCP 서버의 사용자 정의 경로를 포함하여 많은 선택적 필드를 사용하는 플러그인 항목을 보여줍니다:

```json theme={null}
{
  "name": "enterprise-tools",
  "source": {
    "source": "github",
    "repo": "company/enterprise-plugin"
  },
  "description": "Enterprise workflow automation tools",
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

주목할 주요 사항:

* **`commands` 및 `agents`**: 여러 디렉터리 또는 개별 파일을 지정할 수 있습니다. 경로는 플러그인 루트에 상대적입니다.
* **`${CLAUDE_PLUGIN_ROOT}`**: hooks 및 MCP 서버 구성에서 이 변수를 사용하여 플러그인의 설치 디렉터리 내의 파일을 참조합니다. 플러그인이 설치될 때 캐시 위치에 복사되기 때문에 필요합니다.
  * 서버 유형별로 어느 구성 필드가 이를 대체하는지에 대한 [대체 테이블](/docs/ko/plugins-reference#environment-variables)을 참조하세요
  * 플러그인 업데이트를 통해 유지되어야 하는 종속성 또는 상태의 경우 [`${CLAUDE_PLUGIN_DATA}`](/docs/ko/plugins-reference#persistent-data-directory)를 대신 사용합니다
* **`strict: false`**: 이것이 false로 설정되어 있으므로 플러그인은 자신의 `plugin.json`이 필요하지 않습니다. 마켓플레이스 항목이 모든 것을 정의합니다. 아래의 [Strict 모드](#strict-mode)를 참조하세요.

기본적으로 플러그인의 skills는 해당 `source` 아래의 `skills/` 디렉터리에서 로드됩니다. `skills` 필드에 나열된 경로는 해당 스캔에 추가됩니다:

```json theme={null}
"skills": ["./skills/", "./extra-skills/"]
```

여러 플러그인 항목이 마켓플레이스 루트(`source: "./"`)에서 하나의 `skills/` 폴더를 공유할 때 각 항목이 자신의 skills만 로드하도록 특정 하위 디렉터리를 대신 나열합니다:

```json theme={null}
"source": "./",
"skills": ["./skills/code-review", "./skills/docs"]
```

마켓플레이스 루트 `source`를 사용하면 나열된 경로가 해당 항목의 완전한 집합이 되며, 공유된 `skills/` 폴더의 다른 디렉터리는 로드되지 않습니다. `./skills/` 자체 또는 플러그인 루트를 나열하면 전체 스캔이 유지됩니다. 나열된 경로 중 어느 것도 존재하지 않으면 기본 스캔이 대신 실행됩니다.

<h3 id="strict-mode">
  Strict 모드
</h3>

`strict` 필드는 `plugin.json`이 구성 요소 정의(skills, 에이전트, hooks, MCP 서버, 출력 스타일)의 권한인지 여부를 제어합니다.

| 값           | 동작                                                                                  |
| :---------- | :---------------------------------------------------------------------------------- |
| `true`(기본값) | `plugin.json`이 권한입니다. 마켓플레이스 항목은 추가 구성 요소로 이를 보완할 수 있으며 두 소스가 병합됩니다.                |
| `false`     | 마켓플레이스 항목이 전체 정의입니다. 플러그인에 구성 요소를 선언하는 `plugin.json`도 있으면 충돌이 발생하고 플러그인이 로드되지 않습니다. |

**각 모드를 사용할 때:**

* **`strict: true`**: 플러그인은 자신의 `plugin.json`을 가지고 있으며 자신의 구성 요소를 관리합니다. 마켓플레이스 항목은 맨 위에 추가 skills 또는 hooks를 추가할 수 있습니다. 이것이 기본값이며 대부분의 플러그인에서 작동합니다.
* **`strict: false`**: 마켓플레이스 운영자가 완전한 제어를 원합니다. 플러그인 저장소는 원본 파일을 제공하고 마켓플레이스 항목은 이러한 파일 중 어느 것이 skills, 에이전트, hooks 등으로 노출되는지 정의합니다. 마켓플레이스가 플러그인 작성자의 의도와 다르게 플러그인의 구성 요소를 재구성하거나 큐레이션할 때 유용합니다.

<h2 id="host-and-distribute-marketplaces">
  마켓플레이스 호스팅 및 배포
</h2>

<h3 id="host-on-github-recommended">
  GitHub에서 호스팅(권장)
</h3>

GitHub는 마켓플레이스를 호스팅하고 배포하는 권장 방법입니다:

1. **저장소 생성**: 마켓플레이스를 위한 새 저장소 설정
2. **마켓플레이스 파일 추가**: 플러그인 정의와 함께 `.claude-plugin/marketplace.json` 생성
3. **팀과 공유**: 사용자가 `/plugin marketplace add owner/repo`로 마켓플레이스를 추가합니다

**이점**: 기본 제공 버전 제어, 문제 추적 및 팀 협업 기능.

<h3 id="host-on-other-git-services">
  다른 git 서비스에서 호스팅
</h3>

GitLab, Bitbucket 및 자체 호스팅 서버와 같은 모든 git 호스팅 서비스가 작동합니다. 사용자는 전체 저장소 URL로 추가합니다:

```shell theme={null}
/plugin marketplace add https://gitlab.com/company/plugins.git
```

<h3 id="private-repositories">
  개인 저장소
</h3>

Claude Code는 개인 저장소에서 플러그인 설치를 지원합니다. 수동 설치 및 업데이트의 경우 Claude Code는 기존 git 자격 증명 도우미를 사용합니다. 따라서 HTTPS 액세스는 `gh auth login`, macOS Keychain 또는 `git-credential-store`를 통해 터미널에서와 동일하게 작동합니다. SSH 액세스는 호스트가 이미 `known_hosts` 파일에 있고 키가 `ssh-agent`에 로드되어 있는 한 작동합니다. Claude Code는 호스트 지문 및 키 암호에 대한 대화형 SSH 프롬프트를 억제하기 때문입니다. GitHub `owner/repo` 단축 소스는 기본적으로 SSH를 통해 복제합니다. 대신 HTTPS를 통해 복제하려면 [`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`](/docs/ko/env-vars#variables)을 설정합니다.

백그라운드 자동 업데이트는 다르게 작동합니다. 기본적으로 백그라운드 새로고침은 `git pull`에 대해 git 자격 증명 도우미를 비활성화하므로 도우미가 구성되어 있어도 HTTPS를 통해 개인 저장소에 인증할 수 없습니다. SSH 원격은 영향을 받지 않습니다. `ssh-agent`에 로드된 키는 수동 작업과 동일한 방식으로 백그라운드 풀을 인증합니다. 백그라운드 풀이 실패하면 Claude Code는 마켓플레이스를 처음부터 다시 복제하는 것으로 폴백합니다. 다시 복제는 저장된 git 자격 증명을 사용하지만 [대규모 저장소에서 시간 초과](#git-operations-time-out)될 수 있으므로 개인 마켓플레이스 자동 업데이트가 간헐적으로 실패할 수 있습니다.

두 가지 설정이 개인 마켓플레이스를 예측 가능하게 작동하게 합니다:

* `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1`을 설정하여 백그라운드 풀이 실패할 때 기존 복제를 삭제하고 다시 복제하는 대신 유지합니다. 플러그인은 마지막 동기화된 상태에서 계속 작동하며 `/plugin marketplace update`를 사용한 수동 업데이트는 여전히 자격 증명으로 풀합니다.
* git 자격 증명 도우미를 구성합니다. 예를 들어 GitHub의 경우 `gh auth setup-git`을 사용하여 다시 복제 폴백이 프롬프트 없이 인증할 수 있습니다.

`GITHUB_TOKEN`과 같은 공급자 토큰을 환경에서 설정하는 것만으로는 백그라운드 인증을 활성화하지 않습니다. 토큰은 구성된 자격 증명 도우미(예: `GH_TOKEN` 및 `GITHUB_TOKEN`을 읽는 `gh` CLI의 도우미)를 통해서만 적용됩니다.

백그라운드 풀 자체가 HTTPS를 통해 인증하도록 하려면 전역 git URL 다시 쓰기를 구성합니다. 다시 쓰기는 원격 URL에 토큰을 포함하므로 백그라운드 풀이 자격 증명 도우미를 비활성화하더라도 적용되며 성공적인 풀은 다시 복제 폴백을 건너뜁니다. 다음 예제는 마켓플레이스 저장소의 URL을 다시 쓰어 액세스 토큰을 포함합니다:

```bash theme={null}
git config --global url."https://x-access-token:YOUR_TOKEN@github.com/acme-corp/plugins".insteadOf "https://github.com/acme-corp/plugins"
```

다시 쓰기를 마켓플레이스 저장소 또는 조직 경로로 범위를 지정합니다. 기본이 호스트만인 다시 쓰기는 해당 호스트에 대한 모든 가져오기 및 푸시에 적용되며 자신의 저장소에 대한 푸시를 포함한 일반 자격 증명을 재정의합니다.

각 공급자는 다시 쓴 URL에서 다른 사용자 이름을 예상하며 동일한 경로 범위 지정이 모든 공급자에 적용됩니다. 자체 호스팅 서버의 경우 호스트 이름을 서버의 호스트 이름으로 바꿉니다:

| 공급자       | 다시 쓴 URL 형식                                                       |
| :-------- | :---------------------------------------------------------------- |
| GitHub    | `https://x-access-token:YOUR_TOKEN@github.com/acme-corp/plugins`  |
| GitLab    | `https://oauth2:YOUR_TOKEN@gitlab.com/acme-corp/plugins`          |
| Bitbucket | `https://x-token-auth:YOUR_TOKEN@bitbucket.org/acme-corp/plugins` |

다시 쓰기는 gitconfig에 평문으로 토큰을 저장하므로 마켓플레이스 저장소에 대한 읽기 전용 액세스 권한이 있는 토큰을 사용합니다.

<Note>
  CI/CD 환경에서는 개인 저장소에서 플러그인을 설치하기 전에 git 자격 증명 도우미를 구성합니다. GitHub Actions에서 마켓플레이스 저장소에 대한 읽기 액세스 권한이 있는 토큰을 `GH_TOKEN`으로 내보낸 다음 `gh auth setup-git`을 실행합니다. 기본 워크플로우 토큰은 워크플로우 자신의 저장소에만 액세스할 수 있으므로 다른 저장소의 개인 마켓플레이스는 개인 액세스 토큰 또는 앱 토큰이 필요합니다. 파이프라인에서 구성된 전역 URL 다시 쓰기도 백그라운드 풀을 직접 인증합니다.
</Note>

<h3 id="test-locally-before-distribution">
  배포 전에 로컬에서 테스트
</h3>

공유하기 전에 마켓플레이스를 로컬에서 테스트합니다:

```shell theme={null}
/plugin marketplace add ./my-marketplace
/plugin install quality-review-plugin@my-plugins
```

추가 명령어의 전체 범위(GitHub, Git URL, 로컬 경로, 원격 URL)는 [마켓플레이스 추가](/docs/ko/discover-plugins#add-marketplaces)를 참조하세요.

<h3 id="require-marketplaces-for-your-team">
  팀을 위한 마켓플레이스 필수
</h3>

프로젝트 폴더를 신뢰할 때 팀 구성원이 자동으로 마켓플레이스를 설치하도록 저장소를 구성할 수 있습니다. 마켓플레이스를 `.claude/settings.json`에 추가합니다:

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

기본적으로 활성화해야 하는 플러그인을 지정할 수도 있습니다:

```json theme={null}
{
  "enabledPlugins": {
    "code-formatter@company-tools": true,
    "deployment-tools@company-tools": true
  }
}
```

전체 구성 옵션은 [플러그인 설정](/docs/ko/settings#plugin-settings)을 참조하세요.

<Note>
  로컬 `directory` 또는 `file` 소스를 상대 경로와 함께 사용하는 경우 경로는 저장소의 주 체크아웃에 대해 해석됩니다. git worktree에서 Claude Code를 실행할 때 경로는 여전히 주 체크아웃을 가리키므로 모든 worktree가 동일한 마켓플레이스 위치를 공유합니다. 마켓플레이스 상태는 프로젝트당이 아니라 사용자당 한 번 `~/.claude/plugins/known_marketplaces.json`에 저장됩니다.
</Note>

<h3 id="pre-populate-plugins-for-containers">
  컨테이너에 대한 플러그인 사전 채우기
</h3>

컨테이너 이미지 및 CI 환경의 경우 빌드 시간에 플러그인 디렉터리를 사전 채우므로 Claude Code가 런타임에 아무것도 복제하지 않고도 마켓플레이스 및 플러그인이 이미 사용 가능한 상태로 시작됩니다. `CLAUDE_CODE_PLUGIN_SEED_DIR` 환경 변수를 이 디렉터리를 가리키도록 설정합니다.

여러 시드 디렉터리를 계층화하려면 Unix에서는 `:`로, Windows에서는 `;`로 경로를 구분합니다. Claude Code는 각 디렉터리를 순서대로 검색하고 주어진 마켓플레이스 또는 플러그인 캐시를 포함하는 첫 번째 시드가 우선합니다.

시드 디렉터리는 `~/.claude/plugins`의 구조를 미러링합니다:

```
$CLAUDE_CODE_PLUGIN_SEED_DIR/
  known_marketplaces.json
  marketplaces/<name>/...
  cache/<marketplace>/<plugin>/<version>/...
```

시드 디렉터리를 구축하려면 이미지 빌드 중에 Claude Code를 한 번 실행하고, 필요한 플러그인을 설치한 다음, 결과 `~/.claude/plugins` 디렉터리를 이미지에 복사하고 `CLAUDE_CODE_PLUGIN_SEED_DIR`을 가리킵니다.

복사 단계를 건너뛰려면 빌드 중에 `CLAUDE_CODE_PLUGIN_CACHE_DIR`을 대상 시드 경로로 설정하여 플러그인이 직접 설치되도록 합니다:

```bash theme={null}
CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin marketplace add your-org/plugins
CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin install my-tool@your-plugins
```

그런 다음 런타임 환경에서 `CLAUDE_CODE_PLUGIN_SEED_DIR=/opt/claude-seed`를 설정하여 Claude Code가 시작 시 시드에서 읽도록 합니다.

시작 시 Claude Code는 시드의 `known_marketplaces.json`에서 찾은 마켓플레이스를 기본 구성에 등록하고 `cache/` 아래에서 찾은 플러그인 캐시를 다시 복제하지 않고 사용합니다. 이는 대화형 모드와 `-p` 플래그를 사용한 비대화형 모드 모두에서 작동합니다.

동작 세부 정보:

* **읽기 전용**: 시드 디렉터리는 절대 쓰기되지 않습니다. git pull이 읽기 전용 파일 시스템에서 실패하므로 시드 마켓플레이스에 대해 자동 업데이트가 비활성화됩니다.
* **시드 항목이 우선합니다**: 시드에서 선언된 마켓플레이스는 각 시작 시 사용자 구성의 일치하는 항목을 덮어씁니다. 시드 플러그인을 거부하려면 마켓플레이스를 제거하는 대신 `/plugin disable`을 사용합니다.
* **경로 해석**: Claude Code는 시드의 JSON 내에 저장된 경로를 신뢰하지 않고 런타임에 `$CLAUDE_CODE_PLUGIN_SEED_DIR/marketplaces/<name>/`을 탐색하여 마켓플레이스 콘텐츠를 찾습니다. 이는 시드가 빌드된 위치와 다른 경로에 마운트된 경우에도 시드가 올바르게 작동함을 의미합니다.
* **변경 차단**: 시드 관리 마켓플레이스에 대해 `/plugin marketplace remove` 또는 `/plugin marketplace update`를 실행하면 시드 이미지를 업데이트하도록 관리자에게 문의하라는 지침과 함께 실패합니다.
* **설정과 구성**: `extraKnownMarketplaces` 또는 `enabledPlugins`이 시드에 이미 존재하는 마켓플레이스를 선언하면 Claude Code는 복제하는 대신 시드 복사본을 사용합니다.

<h3 id="managed-marketplace-restrictions">
  관리되는 마켓플레이스 제한
</h3>

플러그인 소스에 대한 엄격한 제어가 필요한 조직의 경우 관리자는 관리되는 설정에서 [`strictKnownMarketplaces`](/docs/ko/settings#strictknownmarketplaces) 설정을 사용하여 사용자가 추가할 수 있는 플러그인 마켓플레이스를 제한할 수 있습니다. CLI 플래그를 거부하여 단일 실행을 위해 플러그인, 에이전트 및 MCP 서버를 사이드로드하려면 [`disableSideloadFlags`](/docs/ko/settings#available-settings)와 쌍을 이룹니다. 컨텍스트 설치 제안으로 나타날 수 있는 마켓플레이스의 플러그인을 허용 목록으로 지정하려면 [`pluginSuggestionMarketplaces`](/docs/ko/settings#available-settings)를 설정합니다.

`strictKnownMarketplaces`가 관리되는 설정에서 구성되면 제한 동작은 값에 따라 달라집니다:

| 값            | 동작                                      |
| ------------ | --------------------------------------- |
| 정의되지 않음(기본값) | 제한 없음. 사용자는 모든 마켓플레이스를 추가할 수 있습니다       |
| 빈 배열 `[]`    | 완전한 잠금. 사용자는 새 마켓플레이스를 추가할 수 없습니다       |
| 소스 목록        | 사용자는 허용 목록과 정확히 일치하는 마켓플레이스만 추가할 수 있습니다 |

<h4 id="common-configurations">
  일반적인 구성
</h4>

모든 마켓플레이스 추가 비활성화:

```json theme={null}
{
  "strictKnownMarketplaces": []
}
```

특정 마켓플레이스만 허용:

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

호스트에 대한 정규식 패턴 일치를 사용하여 내부 git 서버의 모든 마켓플레이스 허용. 이는 [GitHub Enterprise Server](/docs/ko/github-enterprise-server#plugin-marketplaces-on-ghes) 또는 자체 호스팅 GitLab 인스턴스에 권장되는 방법입니다:

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

경로에 대한 정규식 패턴 일치를 사용하여 특정 디렉터리의 파일 시스템 기반 마켓플레이스 허용:

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

`pathPattern`으로 모든 파일 시스템 경로를 허용하면서 `hostPattern`으로 네트워크 소스를 제어하려면 `".*"`를 `pathPattern`으로 사용합니다.

<Note>
  `strictKnownMarketplaces`는 사용자가 추가할 수 있는 것을 제한하지만 자체적으로 마켓플레이스를 등록하지는 않습니다. 허용된 마켓플레이스를 사용자가 `/plugin marketplace add`를 실행하지 않고도 자동으로 사용 가능하게 하려면 동일한 `managed-settings.json`에서 [`extraKnownMarketplaces`](/docs/ko/settings#extraknownmarketplaces)와 쌍을 이룹니다. [둘 다 함께 사용](/docs/ko/settings#strictknownmarketplaces)을 참조하세요.
</Note>

<h4 id="how-restrictions-work">
  제한 작동 방식
</h4>

제한은 네트워크 또는 파일 시스템 작업이 발생하기 전에 확인됩니다. 확인은 마켓플레이스 추가 및 플러그인 설치, 업데이트, 새로고침 및 자동 업데이트 시 실행됩니다. 마켓플레이스가 정책 구성 전에 추가되었고 해당 소스가 더 이상 허용 목록과 일치하지 않으면 Claude Code는 해당 마켓플레이스에서 플러그인을 설치하거나 업데이트하기를 거부합니다. 동일한 적용이 `blockedMarketplaces`에도 적용됩니다.

허용 목록은 대부분의 소스 유형에 대해 정확한 일치를 사용합니다. 마켓플레이스가 허용되려면 지정된 모든 필드가 정확히 일치해야 합니다:

* GitHub 소스의 경우: `repo`는 필수이며 허용 목록에 지정된 경우 `ref` 또는 `path`도 일치해야 합니다
* URL 소스의 경우: 전체 URL이 정확히 일치해야 합니다
* `hostPattern` 소스의 경우: 마켓플레이스 호스트가 정규식 패턴과 일치합니다
* `pathPattern` 소스의 경우: 마켓플레이스의 파일 시스템 경로가 정규식 패턴과 일치합니다

정확한 일치는 URL을 정규화하지 않습니다. 후행 슬래시, `.git` 접미사 또는 `ssh://` 대 `https://` 형식은 다른 값으로 취급됩니다. 조직의 마켓플레이스를 둘 이상의 URL 형식으로 복제할 수 있는 경우 모든 형식이 일치하도록 리터럴 URL보다 `hostPattern` 항목을 선호합니다.

`strictKnownMarketplaces`는 [관리되는 설정](/docs/ko/settings#settings-files)에서 설정되므로 개별 사용자 및 프로젝트 구성은 이러한 제한을 재정의할 수 없습니다.

전체 구성 세부 정보(지원되는 모든 소스 유형 및 `extraKnownMarketplaces`와의 비교 포함)는 [strictKnownMarketplaces 참조](/docs/ko/settings#strictknownmarketplaces)를 참조하세요.

<h3 id="version-resolution-and-release-channels">
  버전 해석 및 릴리스 채널
</h3>

플러그인 버전은 캐시 경로 및 업데이트 감지를 결정합니다. 해석된 버전이 사용자가 이미 가지고 있는 것과 일치하면 `/plugin update` 및 자동 업데이트는 플러그인을 건너뜁니다.

Claude Code는 다음 중 설정된 첫 번째 항목에서 플러그인의 버전을 해석합니다:

1. 플러그인의 `plugin.json`의 `version`
2. 플러그인의 마켓플레이스 항목의 `version`
3. 플러그인 소스의 git 커밋 SHA

git 기반 소스 유형 `github`, `url`, `git-subdir` 및 git 호스팅 마켓플레이스 내의 상대 경로의 경우 `version`을 완전히 생략할 수 있으며 모든 새 커밋은 새 버전으로 취급됩니다. 이는 내부 또는 활발하게 개발 중인 플러그인에 대한 가장 간단한 설정입니다.

<Warning>
  `version`을 설정하면 플러그인이 고정됩니다. `plugin.json`이 `"version": "1.0.0"`을 선언하면 해당 문자열을 변경하지 않고 새 커밋을 푸시해도 기존 사용자에게는 아무것도 하지 않습니다. Claude Code가 동일한 버전을 보고 캐시된 복사본을 유지하기 때문입니다. 모든 릴리스에서 필드를 범프하거나 커밋 SHA를 사용하도록 생략합니다.

  `plugin.json` 및 마켓플레이스 항목 모두에서 `version`을 설정하지 마세요. `plugin.json` 값이 항상 자동으로 우선하므로 오래된 매니페스트 버전이 `marketplace.json`에서 설정한 버전을 숨길 수 있습니다.
</Warning>

<h4 id="set-up-release-channels">
  릴리스 채널 설정
</h4>

플러그인에 대한 "stable" 및 "latest" 릴리스 채널을 지원하려면 동일한 저장소의 다양한 refs 또는 SHA를 가리키는 두 개의 마켓플레이스를 설정할 수 있습니다. 그런 다음 [관리되는 설정](/docs/ko/settings#settings-files)을 통해 두 마켓플레이스를 다양한 사용자 그룹에 할당할 수 있습니다.

<Warning>
  각 채널은 다른 버전으로 해석되어야 합니다. 명시적 버전을 사용하는 경우 `plugin.json`은 각 고정된 ref에서 다른 `version`을 선언해야 합니다. `version`을 생략하면 서로 다른 커밋 SHA가 이미 채널을 구분합니다. 두 refs가 동일한 버전 문자열로 해석되면 Claude Code는 이들을 동일한 것으로 취급하고 업데이트를 건너뜁니다.
</Warning>

<h5 id="example">
  예제
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
  사용자 그룹에 채널 할당
</h5>

관리되는 설정을 통해 각 마켓플레이스를 적절한 사용자 그룹에 할당합니다. 예를 들어 stable 그룹은 다음을 받습니다:

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

early-access 그룹은 대신 `latest-tools`를 받습니다:

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
  의존성 버전 고정
</h4>

플러그인은 의존성에 대한 semver 범위를 제한하여 의존성 업데이트가 종속 플러그인을 손상시키지 않도록 할 수 있습니다. `{plugin-name}--v{version}` git 태그 규칙, 범위 구문 및 동일한 의존성에 대한 여러 제약 조건이 어떻게 결합되는지에 대해서는 [플러그인 의존성 버전 제한](/docs/ko/plugin-dependencies)을 참조하세요.

<h3 id="rename-or-remove-a-plugin">
  플러그인 이름 바꾸기 또는 제거
</h3>

플러그인의 `name`은 안정적인 식별자입니다. 사용자는 `enabledPlugins`, `pluginConfigs` 및 `/plugin install` 명령에서 이를 참조하므로 변경하면 모든 기존 설치가 손상됩니다. UI에 표시되는 레이블을 설치를 손상시키지 않고 변경하려면 [`displayName`](#optional-plugin-fields)을 설정하고 `name`을 변경하지 않은 상태로 유지합니다.

플러그인의 `name`을 변경하거나 `plugins` 배열에서 플러그인을 제거해야 하는 경우 최상위 `renames` 항목을 추가하여 기존 사용자가 `plugin-not-found` 오류를 보는 대신 마이그레이션하도록 합니다. 자동 마이그레이션에는 Claude Code v2.1.193 이상이 필요합니다. 각 이전 이름을 현재 이름으로 매핑하거나 플러그인이 더 이상 존재하지 않으면 `null`로 매핑합니다. 다음 예제는 `formatter`를 `code-formatter`로 이름을 바꾸고 `legacy-linter`가 제거되었음을 기록합니다:

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

사용자가 설정에 여전히 이전 이름이 있는 상태로 Claude Code를 시작하면 Claude Code는 `renames` 맵을 따릅니다:

* 항목이 새 이름을 가리키면 Claude Code는 플러그인을 새 이름으로 로드하고 `"acme-tools" 마켓플레이스에서 "code-formatter"로 이름이 바뀌었습니다`와 같은 한 줄 알림을 표시합니다. 그런 다음 `enabledPlugins` 및 `pluginConfigs` 모두에 대해 사용자, 프로젝트 및 로컬 설정 범위에서 이전 키를 새 키로 다시 작성하므로 알림이 한 번 나타납니다.
* `null` 항목의 경우 Claude Code는 이전 키를 삭제하고 알림은 플러그인이 마켓플레이스에서 제거되었음을 보고합니다.
* 이름이 바뀐 플러그인이 `github` 또는 `npm`과 같은 원격 소스를 사용하면 Claude Code는 이름 바꾸기 후 `plugin-cache-miss`를 보고하고 사용자는 새 이름으로 가져오기 위해 한 번 `/plugin install`을 실행해야 합니다.

`renames`를 추가 전용 기록으로 취급합니다. 모든 사용자가 마이그레이션했을 것으로 예상한 후에도 이전 항목을 제자리에 유지합니다. Claude Code는 체인을 따르므로 나중에 `code-formatter`를 `formatter-pro`로 이름을 바꾸면 첫 번째 항목을 편집하는 대신 두 번째 항목을 추가합니다. 여전히 원본 `formatter`가 활성화된 사용자는 두 항목을 모두 통해 `formatter-pro`로 해석됩니다.

맵을 편집한 후 `claude plugin validate .`를 실행합니다. 체인이 사이클을 형성하거나 `null` 또는 `plugins`에 나열된 이름으로 종료되지 않는 항목을 거부합니다.

<Note>
  관리되는 설정 및 정책 설정은 Claude Code에 대해 읽기 전용이므로 거기에서 활성화된 플러그인은 자동으로 다시 작성될 수 없습니다. 이름이 바뀐 플러그인은 여전히 각 세션에서 로드되지만 관리자가 관리되는 설정 파일의 `enabledPlugins`을 새 이름으로 업데이트할 때까지 이름 바꾸기 알림이 반복됩니다. 동일한 사항이 `--add-dir`과 같은 다른 읽기 전용 소스를 통해 활성화된 플러그인에도 적용됩니다.
</Note>

이전 버전의 Claude Code는 `renames` 필드를 무시하고 이전 이름에 대해 `plugin-not-found`를 보고합니다.

<h2 id="validation-and-testing">
  검증 및 테스트
</h2>

공유하기 전에 마켓플레이스를 테스트합니다.

마켓플레이스 JSON 구문 검증:

```bash theme={null}
claude plugin validate .
```

또는 Claude Code 내에서:

```shell theme={null}
/plugin validate .
```

테스트를 위해 마켓플레이스 추가:

```shell theme={null}
/plugin marketplace add ./path/to/marketplace
```

모든 것이 작동하는지 확인하기 위해 테스트 플러그인 설치:

```shell theme={null}
/plugin install test-plugin@marketplace-name
```

전체 플러그인 테스트 워크플로우는 [플러그인을 로컬에서 테스트](/docs/ko/plugins#test-your-plugins-locally)를 참조하세요. 기술적 문제 해결은 [플러그인 참조](/docs/ko/plugins-reference)를 참조하세요.

<h2 id="manage-marketplaces-from-the-cli">
  CLI에서 마켓플레이스 관리
</h2>

Claude Code는 스크립팅 및 자동화를 위한 비대화형 `claude plugin marketplace` 하위 명령어를 제공합니다. 이는 대화형 세션 내에서 사용 가능한 `/plugin marketplace` 명령어와 동일합니다.

<h3 id="plugin-marketplace-add">
  플러그인 마켓플레이스 추가
</h3>

GitHub 저장소, git URL, 원격 URL 또는 로컬 경로에서 마켓플레이스를 추가합니다.

```bash theme={null}
claude plugin marketplace add <source> [options]
```

**인수:**

* `<source>`: GitHub `owner/repo` 단축형, git URL, `marketplace.json` 파일에 대한 원격 URL 또는 로컬 디렉터리 경로. 분기 또는 태그에 고정하려면 GitHub 단축형에 `@ref`를 추가하거나 git URL에 `#ref`를 추가합니다

URL은 스킴을 포함해야 합니다. Claude Code v2.1.196부터 `gitlab.example.com/team/plugins`와 같이 스킴 없이 입력된 호스트는 잘못된 `owner/repo` 단축형으로 거부되며, 오류 메시지에서 `https://`를 추가하거나 로컬 경로의 경우 `./`를 사용하도록 지시합니다. 이전 버전에서는 이를 GitHub 저장소 경로로 잘못 읽고 GitHub 찾을 수 없음 오류로 클론 시간에 실패합니다.

**옵션:**

| 옵션                    | 설명                                                                                                              | 기본값    |
| :-------------------- | :-------------------------------------------------------------------------------------------------------------- | :----- |
| `--scope <scope>`     | 마켓플레이스를 선언할 위치: `user`, `project` 또는 `local`. [플러그인 설치 범위](/docs/ko/plugins-reference#plugin-installation-scopes) 참조 | `user` |
| `--sparse <paths...>` | git sparse-checkout을 통해 특정 디렉터리로 체크아웃 제한. 모노레포에 유용                                                              |        |

GitHub에서 `owner/repo` 단축형을 사용하여 마켓플레이스 추가:

```bash theme={null}
claude plugin marketplace add acme-corp/claude-plugins
```

`@ref`를 사용하여 특정 분기 또는 태그에 고정:

```bash theme={null}
claude plugin marketplace add acme-corp/claude-plugins@v2.0
```

비 GitHub 호스트의 git URL에서 추가:

```bash theme={null}
claude plugin marketplace add https://gitlab.example.com/team/plugins.git
```

`marketplace.json` 파일을 직접 제공하는 원격 URL에서 추가:

```bash theme={null}
claude plugin marketplace add https://example.com/marketplace.json
```

테스트를 위해 로컬 디렉터리에서 추가:

```bash theme={null}
claude plugin marketplace add ./my-marketplace
```

마켓플레이스를 프로젝트 범위에서 선언하여 `.claude/settings.json`을 통해 팀과 공유:

```bash theme={null}
claude plugin marketplace add acme-corp/claude-plugins --scope project
```

모노레포의 경우 플러그인 콘텐츠를 포함하는 디렉터리로 체크아웃 제한:

```bash theme={null}
claude plugin marketplace add acme-corp/monorepo --sparse .claude-plugin plugins
```

<h3 id="plugin-marketplace-list">
  플러그인 마켓플레이스 목록
</h3>

구성된 모든 마켓플레이스를 나열합니다.

```bash theme={null}
claude plugin marketplace list [options]
```

**옵션:**

| 옵션       | 설명        |
| :------- | :-------- |
| `--json` | JSON으로 출력 |

`--json`을 사용하면 각 항목에는 `name`, `source` 및 소스별 필드가 포함됩니다: GitHub 소스의 경우 `repo`, git 및 URL 소스의 경우 `url`, 로컬 소스의 경우 `path`. GitHub 및 git 소스는 마켓플레이스가 고정된 분기 또는 태그로 추가된 경우 `ref` 필드도 포함합니다.

<h3 id="plugin-marketplace-remove">
  플러그인 마켓플레이스 제거
</h3>

구성된 마켓플레이스를 제거합니다. 별칭 `rm`도 허용됩니다.

```bash theme={null}
claude plugin marketplace remove <name> [options]
```

**인수:**

* `<name>`: `claude plugin marketplace list`에 표시된 마켓플레이스 이름을 제거합니다. 이는 `add`에 전달한 소스가 아니라 `marketplace.json`의 `name`입니다

**옵션:**

| 옵션                | 설명                                                                                                                                                                                                                                 | 기본값     |
| :---------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------ |
| `--scope <scope>` | 제거를 단일 설정 범위로 제한: `user`, `project` 또는 `local`. [플러그인 설치 범위](/docs/ko/plugins-reference#plugin-installation-scopes) 참조. 생략하면 모든 편집 가능한 범위에서 선언이 제거됩니다. 지정하면 해당 범위의 선언만 제거되고, 마켓플레이스가 다른 범위에서 여전히 선언된 경우 공유 상태, 캐시 및 설치된 플러그인 데이터는 유지됩니다 | (모든 범위) |

<Warning>
  마켓플레이스를 마지막 남은 범위에서 제거하면 해당 마켓플레이스에서 설치한 모든 플러그인도 제거됩니다. 설치된 플러그인을 잃지 않고 마켓플레이스를 새로 고치려면 `claude plugin marketplace update`를 대신 사용합니다.
</Warning>

<h3 id="plugin-marketplace-update">
  플러그인 마켓플레이스 업데이트
</h3>

소스에서 마켓플레이스를 새로 고쳐 새 플러그인 및 버전 변경을 검색합니다. 분기 또는 태그 `ref`로 추가된 마켓플레이스는 저장소의 기본 분기가 아니라 해당 ref의 최신 커밋으로 업데이트됩니다.

```bash theme={null}
claude plugin marketplace update [name]
```

**인수:**

* `[name]`: `claude plugin marketplace list`에 표시된 마켓플레이스 이름을 업데이트합니다. 생략하면 모든 마켓플레이스를 업데이트합니다

`remove`와 `update` 모두 시드 관리 마켓플레이스에 대해 실행할 때 실패합니다. 이는 읽기 전용입니다. 모든 마켓플레이스를 업데이트할 때 시드 관리 항목은 건너뛰고 다른 마켓플레이스는 여전히 업데이트됩니다. 시드 제공 플러그인을 변경하려면 관리자에게 시드 이미지를 업데이트하도록 요청합니다. [컨테이너에 대한 플러그인 사전 채우기](#pre-populate-plugins-for-containers)를 참조하세요.

<h2 id="troubleshooting">
  문제 해결
</h2>

<h3 id="marketplace-not-loading">
  마켓플레이스가 로드되지 않음
</h3>

**증상**: 마켓플레이스를 추가할 수 없거나 플러그인을 볼 수 없습니다

**해결책**:

* 마켓플레이스 URL이 액세스 가능한지 확인합니다
* `.claude-plugin/marketplace.json`이 지정된 경로에 있는지 확인합니다
* `claude plugin validate` 또는 `/plugin validate`를 사용하여 JSON 구문이 유효한지 확인합니다. skill, agent 및 command frontmatter를 확인하려면 각 플러그인 디렉터리에 대해 명령을 실행합니다
* 개인 저장소의 경우 액세스 권한이 있는지 확인합니다

<h3 id="marketplace-validation-errors">
  마켓플레이스 검증 오류
</h3>

마켓플레이스 디렉터리에서 `claude plugin validate .` 또는 `/plugin validate .`를 실행하여 문제를 확인합니다. 마켓플레이스 디렉터리를 가리킬 때 검증자는 `marketplace.json`에서 스키마 오류, 중복 플러그인 이름 및 소스 경로 순회를 확인합니다. `source`가 로컬 경로인 각 항목에 대해 해당 플러그인의 `plugin.json`도 검증하고 항목의 `version`이 `plugin.json`의 버전과 일치하지 않을 때 경고합니다. 플러그인의 `plugin.json`에서 발견된 문제는 항목 인덱스 형식인 `plugins[2] plugin.json →`으로 접두사가 붙습니다.

Claude Code v2.1.196부터 항목별 통과는 다음을 포함합니다:

* `source`가 `.`인 플러그인 포함
* `marketplace.json`이 `.claude-plugin` 디렉터리 외부에 있을 때 실행되며, 파일 자체의 디렉터리에 대해 소스를 해석합니다
* 파일의 다른 부분에 스키마 오류가 있을 때도 각 항목의 문제를 보고합니다

이전 버전은 마켓플레이스 루트의 플러그인을 건너뛰고 `.claude-plugin/marketplace.json`에서만 내려갑니다.

개별 플러그인의 `plugin.json` 및 해당 skill, agent, command 및 hook 파일을 검증하려면 플러그인 디렉터리 자체에 대해 명령을 실행합니다(예: `claude plugin validate ./plugins/my-plugin`). 일반적인 오류:

| 오류                                                | 원인                                  | 해결책                                                                                        |
| :------------------------------------------------ | :---------------------------------- | :----------------------------------------------------------------------------------------- |
| `File not found: .claude-plugin/marketplace.json` | 누락된 매니페스트                           | 필수 필드를 사용하여 `.claude-plugin/marketplace.json` 생성                                           |
| `Invalid JSON syntax: Unexpected token...`        | marketplace.json의 JSON 구문 오류        | 누락된 쉼표, 추가 쉼표 또는 인용되지 않은 문자열 확인                                                            |
| `Duplicate plugin name "x" found in marketplace`  | 두 플러그인이 동일한 이름을 공유합니다               | 각 플러그인에 고유한 `name` 값 지정                                                                    |
| `plugins[0].source: Path contains ".."`           | 소스 경로에 `..` 포함                      | 마켓플레이스 루트에 상대적인 경로를 `..` 없이 사용합니다. [상대 경로](#relative-paths) 참조                             |
| `YAML frontmatter failed to parse: ...`           | skill, agent 또는 command 파일의 YAML 무효 | frontmatter 블록의 YAML 구문을 수정합니다. 런타임에 이 파일은 메타데이터 없이 로드됩니다. 플러그인 디렉터리를 검증할 때만 보고됩니다         |
| `Invalid JSON syntax: ...` (hooks.json)           | 형식이 잘못된 `hooks/hooks.json`          | JSON 구문을 수정합니다. 형식이 잘못된 `hooks/hooks.json`은 전체 플러그인이 로드되지 않도록 합니다. 플러그인 디렉터리를 검증할 때만 보고됩니다 |

**경고**(차단하지 않음):

* `Marketplace has no plugins defined`: `plugins` 배열에 최소한 하나의 플러그인 추가
* `No marketplace description provided`: 사용자가 마켓플레이스를 이해하도록 돕기 위해 최상위 `description` 추가
* `Plugin name "x" is not kebab-case`: 플러그인 이름에 대문자, 공백 또는 특수 문자가 포함되어 있습니다. 소문자, 숫자 및 하이픈만 사용하도록 이름을 바꿉니다(예: `my-plugin`). Claude Code는 다른 형식을 허용하지만 claude.ai 마켓플레이스 동기화는 이를 거부합니다.

<h3 id="plugin-installation-failures">
  플러그인 설치 실패
</h3>

**증상**: 마켓플레이스가 나타나지만 플러그인 설치가 실패합니다

**해결책**:

* 플러그인 소스 URL이 액세스 가능한지 확인합니다
* 플러그인 디렉터리에 필수 파일이 포함되어 있는지 확인합니다
* GitHub 소스의 경우 저장소가 공개이거나 액세스 권한이 있는지 확인합니다
* 플러그인 소스를 수동으로 복제/다운로드하여 테스트합니다
* 소스가 `ref`와 `sha`를 모두 고정하는 경우 삭제된 업스트림 분기 또는 태그는 대부분의 git 호스트(GitHub, GitLab 및 Bitbucket 포함)에서 설치를 차단하지 않습니다. AWS CodeCommit과 같이 SHA로 커밋을 가져오기를 지원하지 않는 서버에서는 `ref`가 여전히 존재해야 하고 고정된 커밋이 이로부터 도달 가능해야 합니다. 설치가 계속 실패하면 고정된 커밋이 저장소에 여전히 존재하는지 확인합니다

<h3 id="private-repository-authentication-fails">
  개인 저장소 인증 실패
</h3>

**증상**: 개인 저장소에서 플러그인을 설치할 때 인증 오류

**해결책**:

수동 설치 및 업데이트의 경우:

* git 공급자로 인증되었는지 확인합니다(예: GitHub의 경우 `gh auth status` 실행).
* 자격 증명 도우미가 올바르게 구성되었는지 확인합니다: `git config --global credential.helper`
* 저장소를 수동으로 복제하여 자격 증명이 작동하는지 확인합니다

백그라운드 자동 업데이트의 경우:

* 기본적으로 백그라운드 새로 고침은 pull에 대해 git 자격 증명 도우미를 비활성화하므로 pull이 HTTPS를 통해 인증할 수 없습니다. `ssh-agent`에 로드된 키가 있는 SSH 원격은 여전히 인증합니다. 실패한 pull은 저장된 자격 증명을 사용하지만 대규모 저장소에서 시간 초과될 수 있는 처음부터 다시 복제를 트리거합니다
* `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1`을 설정하여 백그라운드 pull이 실패할 때 기존 복제본을 유지합니다
* git 자격 증명 도우미(예: `gh auth setup-git`)를 구성하여 다시 복제 폴백이 인증할 수 있도록 합니다
* 대규모 저장소에서 다시 복제 시간이 초과되면 [`CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS`](#git-operations-time-out)를 사용하여 제한을 늘립니다
* 백그라운드 pull이 직접 인증하도록 마켓플레이스 저장소로 범위가 지정된 [git URL 재작성](#private-repositories)을 구성합니다
* 또는 자격 증명을 사용하는 `/plugin marketplace update <name>`으로 개인 마켓플레이스를 수동으로 업데이트합니다

<h3 id="marketplace-updates-fail-in-offline-environments">
  마켓플레이스 업데이트가 오프라인 환경에서 실패합니다
</h3>

**증상**: 마켓플레이스 `git pull`이 백그라운드에서 실패하고 Claude Code가 성공할 수 없는 다시 복제를 반복적으로 시도합니다.

**원인**: 기본적으로 `git pull`이 실패하면 Claude Code는 처음부터 다시 복제를 시도합니다. 오프라인 또는 에어갭 환경에서 다시 복제가 동일한 방식으로 실패하고 그 후 이전 캐시의 복원은 최선의 노력입니다. 새로 고침은 시작 후 백그라운드에서 실행되므로 시작을 지연시키지 않지만 각 세션은 실패한 시도를 반복하고 각 git 작업은 [120초 시간 초과](#git-operations-time-out)를 기다릴 수 있습니다.

**해결책**: `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1`을 설정하여 pull이 실패할 때 다시 복제 시도를 건너뛰고 기존 캐시를 계속 사용합니다:

```bash theme={null}
export CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1
```

이 변수가 설정되면 Claude Code는 `git pull` 실패 시 오래된 마켓플레이스 복제본을 유지하고 마지막으로 알려진 좋은 상태를 계속 사용합니다. 저장소에 절대 도달할 수 없는 완전히 오프라인 배포의 경우 대신 [`CLAUDE_CODE_PLUGIN_SEED_DIR`](#pre-populate-plugins-for-containers)을 사용하여 빌드 시간에 플러그인 디렉터리를 사전 채웁니다.

<h3 id="git-operations-time-out">
  Git 작업 시간 초과
</h3>

**증상**: 플러그인 설치 또는 마켓플레이스 업데이트가 "Git clone timed out after 120s" 또는 "Git pull timed out after 120s"와 같은 시간 초과 오류로 실패합니다.

**원인**: Claude Code는 플러그인 저장소 복제 및 마켓플레이스 업데이트 끌어오기를 포함한 모든 git 작업에 120초 시간 초과를 사용합니다. 대규모 저장소 또는 느린 네트워크 연결이 이 제한을 초과할 수 있습니다.

**해결책**: `CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS` 환경 변수를 사용하여 시간 초과를 늘립니다. 값은 밀리초 단위입니다:

```bash theme={null}
export CLAUDE_CODE_PLUGIN_GIT_TIMEOUT_MS=300000  # 5분
```

<h3 id="plugins-with-relative-paths-fail-in-url-based-marketplaces">
  상대 경로가 있는 플러그인이 URL 기반 마켓플레이스에서 실패합니다
</h3>

**증상**: URL을 통해 마켓플레이스를 추가했습니다(예: `https://example.com/marketplace.json`). 하지만 `"./plugins/my-plugin"`과 같은 상대 경로 소스가 있는 플러그인이 "path not found" 오류로 설치되지 않습니다.

**원인**: URL 기반 마켓플레이스는 `marketplace.json` 파일 자체만 다운로드합니다. 서버에서 플러그인 파일을 다운로드하지 않습니다. 마켓플레이스 항목의 상대 경로는 다운로드되지 않은 원격 서버의 파일을 참조합니다.

**해결책**:

* **외부 소스 사용**: 플러그인 항목을 상대 경로 대신 GitHub, npm 또는 git URL 소스를 사용하도록 변경합니다:
  ```json theme={null}
  { "name": "my-plugin", "source": { "source": "github", "repo": "owner/repo" } }
  ```
* **Git 기반 마켓플레이스 사용**: 마켓플레이스를 Git 저장소에서 호스팅하고 git URL로 추가합니다. Git 기반 마켓플레이스는 전체 저장소를 복제하므로 상대 경로가 올바르게 작동합니다.

<h3 id="files-not-found-after-installation">
  설치 후 파일을 찾을 수 없음
</h3>

**증상**: 플러그인이 설치되지만 파일 참조가 실패합니다. 특히 플러그인 디렉터리 외부의 파일

**원인**: 플러그인은 제자리에 사용되지 않고 캐시 디렉터리에 복사됩니다. 플러그인 디렉터리 외부의 파일을 참조하는 경로(예: `../shared-utils`)는 해당 파일이 복사되지 않기 때문에 작동하지 않습니다.

**해결책**: symlink 및 디렉터리 재구성을 포함한 해결 방법은 [플러그인 캐싱 및 파일 해석](/docs/ko/plugins-reference#plugin-caching-and-file-resolution)을 참조하세요.

추가 디버깅 도구 및 일반적인 문제는 [디버깅 및 개발 도구](/docs/ko/plugins-reference#debugging-and-development-tools)를 참조하세요.

<h2 id="see-also">
  참고 항목
</h2>

* [미리 빌드된 플러그인 검색 및 설치](/docs/ko/discover-plugins) - 기존 마켓플레이스에서 플러그인 설치
* [플러그인](/docs/ko/plugins) - 자신의 플러그인 생성
* [플러그인 참조](/docs/ko/plugins-reference) - 완전한 기술 사양 및 스키마
* [플러그인 설정](/docs/ko/settings#plugin-settings) - 플러그인 구성 옵션
* [strictKnownMarketplaces 참조](/docs/ko/settings#strictknownmarketplaces) - 관리되는 마켓플레이스 제한
