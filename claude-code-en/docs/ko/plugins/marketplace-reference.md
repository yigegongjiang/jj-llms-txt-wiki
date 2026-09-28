> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 마켓플레이스 참조

> marketplace.json 필드, 플러그인 항목, 플러그인 및 마켓플레이스 소스 객체의 완전한 참조와 각각이 유효한 위치입니다.

`marketplace.json`은 플러그인 마켓플레이스를 정의하는 파일입니다. 마켓플레이스의 이름, 소유자, 그리고 플러그인당 하나의 항목을 포함합니다. 각 항목의 플러그인 소스는 Claude Code가 해당 플러그인을 어디서 가져오는지를 나타냅니다.

마켓플레이스 소스는 Claude Code가 마켓플레이스 파일 자체를 어디서 가져오는지를 나타내는 별도의 객체입니다. 설정에서 작성하거나, `claude plugin marketplace add`를 실행할 때 Claude Code가 빌드합니다.

이 참조는 정확한 필드 이름이나 값이 필요한 마켓플레이스 유지보수자와 [`extraKnownMarketplaces`](/docs/ko/settings-reference#extraknownmarketplaces), [`strictKnownMarketplaces`](/docs/ko/settings-reference#strictknownmarketplaces), [`blockedMarketplaces`](/docs/ko/plugins/org#restrict-what-users-can-install)에서 어떤 `source` 값이 유효한지 알아야 하는 관리자를 위한 것입니다.

<Note>
  다음 경우는 다른 페이지에서 다룹니다:

  * **마켓플레이스 구축 또는 호스팅**: [마켓플레이스 만들기](/docs/ko/plugins/create-marketplace) 및 [마켓플레이스 호스팅 및 유지보수](/docs/ko/plugins/host-marketplace) 참조
  * **허용 목록 및 차단 목록 레시피**: [조직의 플러그인 관리](/docs/ko/plugins/org) 참조
</Note>

작성하거나 읽고 있는 항목에 대한 섹션을 찾으세요:

* **마켓플레이스 파일**: [최상위 필드](#top-level-fields) 및 [플러그인 항목](#plugin-entries)
* **항목의 `source`**: [플러그인 소스](#plugin-sources)
* **설정의 `source` 객체**: [마켓플레이스 소스](#marketplace-sources)
* **[`claude plugin validate <path>`](/docs/ko/plugins/cli-reference)의 출력**: [검증 메시지](#validation-messages) - 각 메시지를 이름이 지정된 필드에 매핑합니다

<h2 id="marketplace-file">
  마켓플레이스 파일
</h2>

마켓플레이스 파일을 마켓플레이스 디렉토리의 `.claude-plugin/marketplace.json`에 저장합니다. 파일을 저장소의 다른 위치에 보관하는 경우, 사용자는 `source`에 `path`를 설정하여 [`extraKnownMarketplaces`](/docs/ko/settings-reference#extraknownmarketplaces)에서 마켓플레이스를 선언해야 합니다. `claude plugin marketplace add`에는 이에 대한 옵션이 없기 때문입니다.

`.claude-plugin/`을 포함하는 디렉토리를 마켓플레이스 루트라고 하며, 모든 상대 플러그인 소스는 `.claude-plugin/`이 아닌 마켓플레이스 루트에서 확인됩니다.

각 사용자는 `name`당 하나의 마켓플레이스를 등록하므로, 사용자는 동시에 같은 이름의 두 마켓플레이스를 등록할 수 없습니다.

Claude Code는 알 수 없는 최상위 키나 플러그인 항목 키를 거부하지 않고 무시하므로, 오타가 조용히 로드됩니다. `claude plugin validate`는 각 알 수 없는 키를 경고로 보고합니다.

<h3 id="reserved-names">
  예약된 이름
</h3>

마켓플레이스에 다음 이름을 지정할 수 없습니다:

* **공식 마켓플레이스 이름**: `claude-code-marketplace`, `claude-code-plugins`, `claude-plugins-official`, `anthropic-marketplace`, `anthropic-plugins`, `agent-skills`, `anthropic-agent-skills`, `life-sciences`, `knowledge-work-plugins`, `claude-for-legal`, `claude-for-financial-services`, `financial-services-plugins`, `first-party-plugins`, `claude-tag-plugins`. `github.com/anthropics/` 아래의 `github` 또는 `git` [마켓플레이스 소스](#marketplace-sources)에서 오는 마켓플레이스가 아닌 한 예약됨.
* **커뮤니티 마켓플레이스 이름**: `claude-community`, `claude-plugins-community`, `healthcare`. 공식 이름과 동일한 규칙 아래 예약됨.
* **플러그인 디렉토리 이름**: `anthropic-plugin-directory`, `claude-plugin-directory`. 공식 이름과 동일한 규칙 아래 예약됨.
* **공식 마켓플레이스를 사칭하는 이름**: `official-claude-plugins` 또는 `claude-plugins-v2`와 같은 이름, 그리고 비ASCII 문자를 포함하는 모든 이름. 오류는 `Marketplace name impersonates an official Anthropic/Claude marketplace`입니다. 이름의 제어 또는 양방향 서식 문자도 `Marketplace name cannot contain control or bidirectional-formatting characters`를 보고합니다.
* <span id="reserved-name-spellings" />**예약된 이름의 다른 철자**: 예약된 이름과 후행 점으로만 다르거나 하이픈 대신 다른 기호를 사용하는 이름이므로 `claude.code.plugins`는 `claude-code-plugins`로 계산됩니다. `claude plugin validate`는 이러한 이름을 수락합니다. 마켓플레이스 추가는 [`is another spelling of "<reserved>", a reserved marketplace name`](/docs/ko/errors#marketplace-name-is-another-spelling-of-a-reserved-name)으로 실패하고, 하나 아래에 등록된 마켓플레이스는 로드를 중지합니다. 이 확인에는 Claude Code v2.1.280 이상이 필요합니다.
* **Claude Code가 마켓플레이스에서 오지 않는 플러그인에 사용하는 이름**: [`--plugin-dir`](/docs/ko/cli-reference)로 로드된 플러그인의 경우 `inline`, 기본 제공 플러그인의 경우 `builtin`, [`.claude/skills/`](/docs/ko/skills)에서 자동 로드되는 플러그인의 경우 `skills-dir`, claude.ai 계정에서 동기화된 플러그인의 경우 `synced`. `claude-plugin-test`도 예약됩니다. `skills-dir`은 `{"source": "skills-dir"}`로도 `strictKnownMarketplaces` 및 `blockedMarketplaces`에 나타나며, [소스 값이 정책 목록에서만 유효함](#source-values-valid-only-in-policy-lists)에서 설명합니다.
* **`npm`, `pip`, `uv`, `cargo`, `github`, `gh`**: 모든 대소문자로 예약됨. 이 확인에는 Claude Code v2.1.275 이상이 필요합니다.
* **`claudeai-`로 시작하는 이름**: claude.ai에서 호스팅되는 마켓플레이스를 위해 예약됨. `claude plugin marketplace add`는 `Cannot add marketplace "<name>": names starting with "claudeai-" are reserved for marketplaces hosted on claude.ai`로 이를 사용하는 다른 마켓플레이스를 거부합니다.

<h2 id="top-level-fields">
  최상위 필드
</h2>

표는 Claude Code가 `marketplace.json`에서 읽는 모든 키를 나열합니다. `name`, `owner`, `plugins`는 필수입니다.

| 필드                                         | 유형               | 설명                                                                                                                                             |
| :----------------------------------------- | :--------------- | :--------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                                     | string           | 마켓플레이스 식별자. 공백, 제어 문자 또는 양방향 서식 문자 없음, `/` 또는 `\` 없음, `..` 없음, `.` 아님. [예약된 이름](#reserved-names) 참조. 사용자는 플러그인을 설치할 때 `@` 뒤에 입력합니다             |
| `owner`                                    | object           | 유지보수자 정보. `name`은 필수이고 `email` 및 `url`은 선택사항입니다                                                                                                |
| `plugins`                                  | array            | [플러그인 항목](#plugin-entries). 각 항목은 독립적으로 검증되므로 하나의 잘못된 항목이 마켓플레이스를 실패하게 하지 않습니다                                                                 |
| `$schema`                                  | string           | 편집기 자동 완성을 위한 JSON Schema URL. 로드 시간에 무시됨                                                                                                      |
| `description`                              | string           | 사용자에게 표시되는 마켓플레이스 설명. `claude plugin validate`는 누락되면 경고합니다                                                                                     |
| `version`                                  | string           | 마켓플레이스 매니페스트 버전                                                                                                                                |
| `metadata.description`, `metadata.version` | string           | `description` 및 `version`의 대체 위치                                                                                                               |
| `metadata.pluginRoot`                      | string           | 베어 플러그인 소스 이름이 확인되는 디렉토리. [상대 경로 플러그인 소스](#relative-path-plugin-source) 참조. Claude Code v2.1.239 이상 필요                                         |
| `forceRemoveDeletedPlugins`                | boolean          | `true`일 때, `plugins`에서 제거한 플러그인이 사용자 머신에서 제거됩니다. [마켓플레이스 호스팅 및 유지보수](/docs/ko/plugins/host-marketplace) 참조                                          |
| `allowCrossMarketplaceDependenciesOn`      | array of strings | 이 마켓플레이스의 플러그인의 종속성으로 설치될 수 있는 플러그인의 마켓플레이스 이름. 플러그인을 설치할 때, 해당 플러그인의 자체 마켓플레이스의 목록만 전체 종속성 체인에 적용됩니다. [플러그인 종속성](/docs/ko/plugins/dependencies) 참조 |
| `renames`                                  | object           | 이전 플러그인 `name`에서 현재 이름으로의 맵, 또는 제거한 플러그인의 경우 `null`. Claude Code v2.1.193 이상 필요. [마켓플레이스 호스팅 및 유지보수](/docs/ko/plugins/host-marketplace) 참조          |

<h2 id="plugin-entries">
  플러그인 항목
</h2>

`marketplace.json`의 최상위 `plugins` 배열의 각 객체는 플러그인의 이름을 지정하고 가져올 위치를 나타냅니다. `name` 및 `source`는 필수입니다.

항목은 또한 `description`, `version`, `author`, `commands`, `hooks`와 같은 모든 [`plugin.json` 필드](/docs/ko/plugins/manifest-reference)를 수락합니다. 이러한 필드가 적용되는 경우는 [항목이 plugin.json과 결합되는 방식](#entry-and-plugin-json)을 참조하세요.

표는 항목의 자체 필드와 항목에서 의미가 변경되는 매니페스트 필드를 나열합니다.

| 필드               | 유형               | 설명                                                                                                                                                                                                                       |
| :--------------- | :--------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`           | string           | 공백, 제어 문자 또는 양방향 서식 문자가 없는 플러그인 식별자. 사용자는 플러그인의 자체 `plugin.json`이 다른 `name`을 설정하더라도 설치할 때 `@` 앞에 입력합니다                                                                                                                   |
| `source`         | string or object | 플러그인을 가져올 위치. [플러그인 소스](#plugin-sources) 참조                                                                                                                                                                              |
| `description`    | string           | [`/plugin`](/docs/ko/plugins/install) 목록 및 세부 정보에 표시됨                                                                                                                                                                         |
| `version`        | string           | 플러그인의 버전 문자열. `plugin.json`도 `version`을 설정할 때, `plugin.json`이 우선이고 `claude plugin validate`가 경고합니다. [플러그인 로딩 참조](/docs/ko/plugins/loading) 참조                                                                                 |
| `category`       | string           | 카탈로그를 구성하기 위한 자유 형식 카테고리                                                                                                                                                                                                 |
| `tags`           | array of strings | 검색을 위한 자유 형식 태그                                                                                                                                                                                                          |
| `strict`         | boolean          | 기본값 `true`. `plugin.json`이 플러그인의 구성 요소에 대한 확정적 소스인지 여부. [엄격 모드](#strict-mode) 참조                                                                                                                                         |
| `relevance`      | object           | Claude Code에 플러그인을 제안할 시기를 알려주는 신호. [조직의 플러그인 권장](/docs/ko/plugins/relevance) 참조                                                                                                                                              |
| `dependencies`   | array            | 이 플러그인이 작동하기 위해 활성화되어야 하는 플러그인. 각 항목은 `"name"`, `"name@marketplace"`, 또는 객체입니다. [플러그인 종속성](/docs/ko/plugins/dependencies) 참조                                                                                                  |
| `defaultEnabled` | boolean          | 기본값 `true`. 사용자가 [`enabledPlugins`](/docs/ko/settings-reference#enabledplugins)에서 설정하지 않았을 때 플러그인이 활성화되어 시작되는지 여부. 항목 값이 `plugin.json`보다 우선합니다                                                                                |
| `displayName`    | string           | UI에 표시되는 사람이 읽을 수 있는 이름. 항목도 플러그인의 `plugin.json`도 설정하지 않으면, 사용자는 플러그인의 `name`을 봅니다                                                                                                                                       |
| `metadata`       | object           | 자신의 필드를 위한 자유 형식 객체. Claude Code는 이를 읽지 않습니다. Claude Code v2.1.222 이상 필요                                                                                                                                                 |
| `headers`        | object           | Claude Code가 이 항목의 [아카이브](#archive-plugin-source)를 다운로드할 때 보내는 HTTP 헤더. 여기에 설정된 헤더는 마켓플레이스 소스의 [`headers`](#fields-by-type)에서 같은 이름의 헤더를 대체합니다. Claude Code v2.1.238 이상 필요                                               |
| `headersHelper`  | string           | 이 항목의 아카이브 다운로드 헤더를 하나의 JSON 객체로 인쇄하는 명령, 만료되는 자격 증명의 경우. 항목은 또한 [`"strict": false`](#strict-mode)를 설정해야 합니다. Claude Code v2.1.238 이상 필요. [아카이브 다운로드 인증](/docs/ko/plugins/host-marketplace#authenticate-archive-downloads) 참조 |

<h3 id="entry-and-plugin-json">
  항목이 plugin.json과 결합되는 방식
</h3>

항목의 필드는 자체 `.claude-plugin/plugin.json`을 가진 가져온 플러그인과 그렇지 않은 플러그인에 다르게 적용됩니다:

* **`plugin.json` 없음**: 항목은 `strict`에 관계없이 매니페스트입니다. [`mcpServers`, `lspServers`, `userConfig`, `channels`](/docs/ko/plugins/manifest-reference)를 포함한 모든 매니페스트 필드가 항목에 적용됩니다.
* **`plugin.json` 있음**: `plugin.json`이 매니페스트입니다. [엄격 모드](#strict-mode)는 항목의 6개 구성 요소 필드인 `commands`, `agents`, `skills`, `hooks`, `outputStyles`, `themes`를 결합할지 아니면 충돌로 거부할지 결정합니다. 항목 `mcpServers`, `lspServers`, `userConfig`, `channels`는 적용되지 않습니다. `plugin.json`에서 선언하세요.

<h4 id="hooks-in-an-entry">
  항목의 훅
</h4>

항목 `hooks`를 훅 이벤트 이름을 매처 배열에 매핑하는 인라인 객체로 작성하세요. 파일 경로나 배열을 작성하면, `claude plugin validate`는 통과합니다. 이러한 훅은 실행되지 않으며, Claude Code는 플러그인에 대해 `not yet supported in a marketplace entry` 오류를 보고합니다. 파일 기반 훅을 플러그인의 자체 [`hooks/hooks.json`](/docs/ko/plugins/components) 또는 `plugin.json`에 넣으세요.

<h4 id="display-fields">
  표시 필드
</h4>

항목과 플러그인의 자체 `plugin.json` 모두 표시 필드 `displayName`, `description`, `author`, `homepage`, `repository`, `license`, `keywords`를 설정할 수 있습니다. 사용자는 설치 전후 플러그인 목록 및 세부 정보에서 이러한 값을 봅니다:

* 항목에 설정한 필드의 경우, 사용자는 `plugin.json`이 다른 값을 설정하더라도 항목의 값을 봅니다.
* 항목이 설정하지 않은 필드의 경우, 사용자는 `plugin.json` 값을 봅니다.

설치 전에, Claude Code는 마켓플레이스 내부에 플러그인 파일이 있는 [상대 경로 소스](#relative-path-plugin-source)를 가진 항목에 대해서만 `plugin.json`을 읽을 수 있습니다. 다른 소스 유형을 가진 항목의 경우, 사용자는 플러그인을 설치할 때까지 항목의 자체 필드만 봅니다.

<h3 id="strict-mode">
  엄격 모드
</h3>

`strict`는 가져온 플러그인이 자체 `plugin.json`을 가지고 있고 항목도 [구성 요소 필드](#entry-and-plugin-json) 중 하나를 선언할 때 어떤 일이 발생하는지 결정합니다: `commands`, `agents`, `skills`, `hooks`, `outputStyles`, `themes`. `strict: true`(기본값)일 때, Claude Code는 항목의 구성 요소 필드를 `plugin.json`에 추가합니다. `hooks` 제외하고, 그 매처는 매니페스트의 이벤트별 매처를 대체합니다. `strict: false`일 때, 구성 요소 필드를 선언하는 항목은 충돌이며, 플러그인이 로드되지 않습니다. 표는 `strict`, `plugin.json`, 항목의 구성 요소 필드의 각 조합을 보여줍니다.

| `strict`    | `plugin.json` | 항목 구성 요소 필드 | 결과                                                                                                                                                                            |
| :---------- | :------------ | :---------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| any         | absent        | any         | 항목이 매니페스트입니다                                                                                                                                                                  |
| `true`, 기본값 | present       | any         | `plugin.json`이 권한입니다. Claude Code는 항목의 구성 요소 필드를 추가합니다. `hooks` 제외하고, 그 매처는 [매니페스트의 이벤트별 매처를 대체합니다](/docs/ko/plugins/manifest-reference#how-entry-fields-combine-with-plugin-json) |
| `false`     | present       | none        | `plugin.json`이 매니페스트입니다. `true`와 동일                                                                                                                                           |
| `false`     | present       | one or more | 충돌. 플러그인이 `Plugin <name> has conflicting manifests: both plugin.json and marketplace entry specify components`로 로드되지 않습니다                                                     |

<h2 id="plugin-sources">
  플러그인 소스
</h2>

플러그인 항목의 `source`는 Claude Code가 해당 플러그인을 어디서 가져오는지를 나타냅니다. 상대 경로 문자열이거나 자체 `source` 키가 유형을 지정하는 객체이므로, 항목은 `"source": { "source": "github", "repo": "your-org/formatter" }`와 같은 형태입니다.

아래 표는 각 플러그인 소스 유형과 해당 필드를 나열합니다.

| 유형           | 필드                               | 참고                                                                                                                                                       |
| :----------- | :------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 상대 경로        | 문자열 자체                           | 마켓플레이스 내의 디렉토리로, 마켓플레이스 루트에서 확인됩니다. `./`로 시작해야 하며, [`metadata.pluginRoot`](#relative-path-plugin-source) 아래에 베어 이름을 작성하지 않는 한 그렇습니다. `"."`는 루트 자체를 의미합니다 |
| `github`     | `repo`, `ref`, `sha`             | `owner/repo` 형식의 GitHub 저장소                                                                                                                              |
| `url`        | `url`, `ref`, `sha`              | URL로 지정된 모든 git 저장소                                                                                                                                      |
| `git-subdir` | `url`, `path`, `ref`, `sha`      | git 저장소의 한 하위 디렉토리로, 스파스 부분 클론으로 가져옵니다                                                                                                                   |
| `npm`        | `package`, `version`, `registry` | npm 패키지로, npm 클라이언트로 가져오고 설치 스크립트를 실행하지 않고 압축을 풉니다                                                                                                       |
| `archive`    | `url`, `sha256`                  | HTTPS를 통한 Zip 아카이브입니다. Claude Code v2.1.224 이상이 필요합니다                                                                                                    |
| `command`    | `command`, `timeout`, `mode`     | Claude Code가 사용자의 머신에서 실행하는 명령으로 출력된 디렉토리입니다. Claude Code v2.1.229 이상이 필요합니다                                                                             |

`url`과 `github`이라는 이름은 또한 [마켓플레이스 소스](#marketplace-sources) 유형이기도 하며, 여기서 `url`은 git 저장소가 아닌 `marketplace.json` 파일로의 직접 링크를 의미합니다. `git`은 마켓플레이스 소스로만 존재하고, `npm`은 둘 다로 존재합니다. `git-subdir`, `archive`, `command`는 플러그인 소스로만 존재합니다.

마켓플레이스 저장소 자체의 하위 디렉토리에 있는 플러그인의 경우 상대 경로를 사용합니다. 다른 저장소의 하위 디렉토리의 경우 `git-subdir`을 사용합니다.

`github`, `url`, `git-subdir` 소스는 `ref`와 `sha` 필드를 공유합니다:

* **`ref`**: 브랜치 또는 태그입니다. 저장소의 기본 브랜치로 기본 설정됩니다.
* **`sha`**: 전체 40자 소문자 커밋 SHA입니다. `ref`와 `sha`를 모두 설정하면 Claude Code는 `sha`를 체크아웃합니다. GitHub, GitLab, Bitbucket을 포함한 대부분의 git 호스트에서 이는 `ref`로 지정된 브랜치 또는 태그가 업스트림에서 삭제되었더라도 커밋이 여전히 저장소에서 도달 가능한 한 설치가 성공함을 의미합니다. AWS CodeCommit과 같은 일부 서버는 SHA로 커밋을 가져오는 것을 지원하지 않습니다. 이러한 서버에서는 `ref`가 여전히 존재해야 하고 고정된 커밋이 이로부터 도달 가능해야 합니다.

각 유형이 어떻게 가져오고, 캐시되고, 버전 관리되는지는 [플러그인 로딩 참조](/docs/ko/plugins/loading)를 참조하세요.

<h3 id="relative-path-plugin-source">
  상대 경로 플러그인 소스
</h3>

경로는 마켓플레이스 루트에서 확인됩니다. `./plugins/formatter`는 마켓플레이스 파일이 `<root>/.claude-plugin/`에 있더라도 `<root>/plugins/formatter`입니다.

`..`를 포함하는 경로는 검증에 실패합니다. macOS와 Linux에서 Claude Code는 선행 `./` 이후에 백슬래시를 포함하는 항목 경로를 거부하므로 경로를 슬래시로 작성합니다.

```json theme={null}
{ "name": "formatter", "source": "./plugins/formatter" }
```

상대 경로는 Claude Code가 마켓플레이스의 파일을 가지고 있을 때만 확인되므로 [마켓플레이스 소스](#marketplace-sources) 유형을 확인합니다:

* **`github`, `git`, `file`, `directory`**: Claude Code가 마켓플레이스의 파일을 가지고 있습니다.
* **`url`**: Claude Code는 `marketplace.json`만 가져오므로 상대 경로를 확인할 수 없습니다. 각 플러그인에 `github` 또는 `git-subdir`과 같은 객체 소스를 제공합니다.
* **`settings`**: 상대 경로는 완전히 거부됩니다.

<h4 id="bare-names-under-pluginroot">
  pluginRoot 아래의 베어 이름
</h4>

베어 이름은 `/`가 없는 단일 디렉토리 이름입니다(예: `"formatter"`). `./` 경로 대신 베어 이름을 작성하려면 [`metadata.pluginRoot`](#top-level-fields)를 이들이 확인되는 디렉토리로 설정합니다. `"pluginRoot": "./plugins"`를 사용하면 `"source": "formatter"`는 `./plugins/formatter`로 확인됩니다. Claude Code v2.1.239 이상이 필요합니다.

`metadata.pluginRoot`에는 다음과 같은 제한이 있습니다:

* 그 자체가 마켓플레이스 내의 상대 경로여야 합니다.
* 이미 `./`로 시작하는 소스에는 영향을 주지 않습니다.
* `team-a/formatter`와 같이 `/`를 포함하는 소스는 베어 이름이 아니며 `metadata.pluginRoot`가 설정되어 있더라도 여전히 `./` 접두사가 필요합니다.

<h3 id="github-plugin-source">
  github 플러그인 소스
</h3>

`repo`는 `owner/repo`를 사용합니다. `ref`와 `sha`는 선택 사항입니다.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "github",
    "repo": "your-org/formatter",
    "ref": "v2.0.0",
    "sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0"
  }
}
```

<h3 id="url-plugin-source">
  url 플러그인 소스
</h3>

`url`은 전체 git URL입니다: `https://`, `http://`, `file://`, 또는 `git@`. `.git` 접미사는 필요하지 않으므로 Azure DevOps 및 AWS CodeCommit URL이 그대로 작동합니다. 이 유형은 `owner/repo` 단축형을 사용하지 않습니다.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "url",
    "url": "https://gitlab.example.com/your-group/formatter.git",
    "ref": "main"
  }
}
```

<h3 id="git-subdir-plugin-source">
  git-subdir 플러그인 소스
</h3>

`url`은 전체 git URL 또는 GitHub `owner/repo` 단축형을 허용합니다. `path`는 플러그인을 보유한 하위 디렉토리이며, Claude Code는 해당 하위 디렉토리만 다운로드합니다.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "git-subdir",
    "url": "https://github.com/your-org/monorepo.git",
    "path": "tools/formatter"
  }
}
```

<h3 id="npm-plugin-source">
  npm 플러그인 소스
</h3>

`npm` 소스는 다음 필드를 사용합니다:

* `package`: 패키지 이름 또는 `@your-org/formatter`와 같은 스코프된 이름
* `version`: 버전 또는 범위
* `registry`: 기본 레지스트리에 없는 패키지의 레지스트리 URL

Claude Code는 npm 클라이언트로 패키지를 가져옵니다. 패키지의 설치 스크립트(예: `preinstall` 또는 `postinstall`)는 절대 실행되지 않으며, 해당 종속성은 가져오기 중에 설치되지 않습니다. 패키지의 `package.json` 옆에 지원되는 lockfile이 있으면 Claude Code는 스크립트도 비활성화된 상태에서 별도의 단계에서 해당 [Node.js 패키지 종속성](/docs/ko/plugins/loading#node-js-package-dependencies)을 설치합니다.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "npm",
    "package": "@your-org/formatter",
    "version": "^2.0.0",
    "registry": "https://npm.example.com"
  }
}
```

<h3 id="archive-plugin-source">
  archive 플러그인 소스
</h3>

`url`은 `https://`를 사용해야 하며 루프백, 링크-로컬 또는 클라우드 메타데이터 호스트를 가리킬 수 없습니다.

플러그인 루트는 zip의 맨 위 또는 한 디렉토리 아래에 있을 수 있습니다.

`sha256`은 아카이브의 다이제스트로 64개의 16진 문자(대문자 또는 소문자)입니다. 이를 설정하면 Claude Code는 일치하지 않는 다운로드를 거부합니다.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "archive",
    "url": "https://artifacts.example.com/formatter-2.0.0.zip",
    "sha256": "6bfa50e3d2e00c052b46abe51fff89346ac803e45771f76dcf6df1ab74cca5e1"
  }
}
```

<h3 id="command-plugin-source">
  command 플러그인 소스
</h3>

사용자의 머신에 설치된 도구가 플러그인 디렉토리를 생성할 때(예: 사용자가 선택한 도구 체인에 대해 플러그인을 렌더링하는 IDE) `command` 소스를 사용합니다. Claude Code는 사용자가 플러그인을 설치하거나 업데이트할 때 명령을 실행하고, [세션당 한 번 다시](/docs/ko/plugins/loading#when-a-command-source-re-runs) 실행하므로 사용자는 재설치 없이 도구의 변경된 출력을 얻습니다.

`command` 소스는 다음 필드를 사용합니다:

* `command`: 플러그인 디렉토리의 절대 경로를 한 줄로 출력하고 0으로 종료하는 셸 명령입니다. Claude Code는 실행하기 전에 사용자에게 전체 문자열을 검토하도록 표시합니다. 인쇄 가능한 ASCII로 작성하고, 최대 500자이며, 4개 이상의 연속 공백이 없어야 합니다.
* `timeout`: 1에서 600 사이의 전체 초 수입니다. 기본값은 60입니다.
* `mode`: `copy`(기본값) 또는 `link`입니다. [복사 모드 및 링크 모드](#copy-mode-and-link-mode)를 참조하세요.

```json theme={null}
{
  "name": "formatter",
  "source": {
    "source": "command",
    "command": "my-tool claude-plugin-path",
    "timeout": 120
  }
}
```

사용자가 명령을 수락하는 방법은 [셸에서 설치](/docs/ko/plugins/install#install-from-your-shell)를 참조하세요. 변경 후 사용자가 보는 내용은 [명령 소스의 명령 변경](/docs/ko/plugins/host-marketplace#change-the-command-of-a-command-source)을 참조하세요. 관리자는 [`disableCommandPluginSources`](/docs/ko/settings-reference#disablecommandpluginsources)로 명령 소스를 비활성화합니다.

<h4 id="what-the-command-must-do">
  명령이 수행해야 할 작업
</h4>

명령이 다음 요구 사항을 충족하도록 작성합니다:

* **셸 및 작업 디렉토리**: Claude Code는 사용자의 홈 디렉토리에서 `sh` 또는 Windows의 `cmd.exe`를 통해 명령을 실행합니다. 절대 경로 또는 `PATH`의 명령을 제공합니다.
* **출력**: stdout에 정확히 한 줄(플러그인 디렉토리의 절대 경로)을 출력하고 `timeout` 초 내에 0으로 종료합니다.
* **디렉토리 내용**: 디렉토리는 명령이 종료될 때까지 완전한 플러그인을 보유합니다. 경로는 실행마다 다를 수 있습니다.

<h4 id="output-that-fails-the-install-or-update">
  설치 또는 업데이트를 실패하게 하는 출력
</h4>

명령이 0이 아닌 값으로 종료되거나, `timeout`보다 오래 실행되거나, 하나의 절대 경로 이외의 것을 출력할 때 설치 또는 업데이트가 실패합니다. 또한 인쇄된 디렉토리가 다음 중 하나일 때도 실패합니다:

* **플러그인 콘텐츠 없음**: 인쇄된 디렉토리의 최상위 수준에 `.claude-plugin/` 디렉토리 또는 `skills/`, `commands/`, `agents/`, `hooks/` 디렉토리와 같은 플러그인 콘텐츠가 없습니다.
* **세션의 자체 디렉토리**: 인쇄된 디렉토리는 Claude Code가 시작된 디렉토리 또는 그 부모 중 하나입니다.
* **네트워크 경로**: Windows에서 인쇄된 경로는 UNC 경로입니다.
* **복사하기에 너무 큼**: 복사 모드에서 디렉토리는 256 MiB보다 크거나 20,000개 이상의 항목을 가집니다.

<h4 id="copy-mode-and-link-mode">
  복사 모드 및 링크 모드
</h4>

`mode`는 Claude Code가 인쇄된 디렉토리를 복사할지 아니면 제자리에서 사용할지를 결정합니다:

* **`copy`**: Claude Code는 디렉토리를 플러그인 캐시에 복사하고 복사된 파일의 해시에서 [플러그인 버전](/docs/ko/plugins/loading#how-claude-code-computes-the-version)을 파생합니다. 도구는 명령이 종료된 후 디렉토리를 삭제하거나 다시 쓸 수 있습니다. 동일한 파일을 생성하는 재실행은 최신 상태로 계산됩니다.
* **`link`**: Claude Code는 인쇄된 디렉토리의 각 최상위 항목에 대한 링크로 플러그인의 캐시 항목을 채우고 파일을 제자리에서 로드합니다. 아무것도 복사되지 않고, 파일 내용이 해시되지 않으며, 크기 제한이 적용되지 않습니다. 렌더링된 SDK 내보내기와 같이 복사하기에 너무 큰 디렉토리에 사용합니다.

링크 모드 플러그인에는 다음과 같은 요구 사항이 있습니다:

* **디렉토리를 제자리에 유지**: Claude Code는 모든 시작 시 링크를 통해 플러그인을 로드하므로 인쇄된 디렉토리는 플러그인이 설치된 상태로 유지되는 동안 그 위치에 남아 있어야 합니다.
* **새 콘텐츠를 신호하기 위해 다른 경로 출력**: 버전은 인쇄된 디렉토리의 실제 경로 및 최상위 항목에서 나오며, 내부의 파일에서는 나오지 않습니다.
* **디렉토리 내에 최상위 심볼릭 링크 유지**: 최상위 항목이 인쇄된 디렉토리 외부를 가리키는 심볼릭 링크인 경우 설치가 실패합니다.
* **`node_modules` 포함**: Claude Code는 링크 모드 플러그인에 대해 [Node.js 패키지 종속성 설치](/docs/ko/plugins/loading#node-js-package-dependencies)를 건너뛰므로 플러그인이 필요한 패키지를 이미 포함하는 디렉토리를 출력합니다.
* **디렉토리 내에서 시작된 세션**: 인쇄된 디렉토리 또는 그 아래 어디서나 시작된 세션은 플러그인을 로드하지 않습니다.
* **Windows에서 아님**: Claude Code는 Windows에서 링크 모드 플러그인 설치를 거부합니다. 거기서 `"mode": "copy"`를 선언합니다.

<h2 id="marketplace-sources">
  마켓플레이스 소스
</h2>

마켓플레이스 소스는 Claude Code가 `marketplace.json`을 어디서 가져오는지를 나타냅니다. CLI는 마켓플레이스를 추가할 때 하나를 빌드하고, 설정에서 직접 작성합니다:

* **[`claude plugin marketplace add`](/docs/ko/plugins/cli-reference)**: Claude Code는 전달한 문자열에서 소스를 빌드합니다.
* **[`extraKnownMarketplaces`](/docs/ko/settings-reference#extraknownmarketplaces)**: `source` 객체로 직접 작성합니다.
* **[`strictKnownMarketplaces`](/docs/ko/settings-reference#strictknownmarketplaces) 및 [`blockedMarketplaces`](/docs/ko/plugins/org#restrict-what-users-can-install)**: 관리자가 이 두 정책 목록에 소스를 작성합니다. `strictKnownMarketplaces`는 허용 목록이고 `blockedMarketplaces`는 차단 목록입니다.

`url`, `git`, `github` 유형 이름은 [플러그인 소스](#plugin-sources)에서와 마켓플레이스 소스에서 다른 의미를 가집니다:

| 유형 이름    | 마켓플레이스 소스로                                                               | 플러그인 소스로                                          |
| :------- | :----------------------------------------------------------------------- | :------------------------------------------------ |
| `url`    | `marketplace.json` 파일에 대한 직접 링크, `url`, `headers`, `headersHelper` 필드 포함 | 복제할 git 저장소, `url`, `ref`, `sha` 필드 포함            |
| `git`    | 복제할 git 저장소, `url`, `ref`, `path`, `sparsePaths` 필드 포함                   | 존재하지 않음                                           |
| `github` | GitHub 저장소, `repo`, `ref`, `path`, `sparsePaths` 필드 포함                   | GitHub 저장소, `repo`, `ref`, `sha` 필드 포함, `path` 없음 |

표는 모든 마켓플레이스 소스 유형을 필드, 생성하는 `claude plugin marketplace add` 입력, 그리고 세 가지 설정 키 각각에서의 작동 방식과 함께 나열합니다.

| 유형            | 필드                                   | `marketplace add` 입력                                                                                                        | `extraKnownMarketplaces`                             | `strictKnownMarketplaces`                                                                                                                                             | `blockedMarketplaces`               |
| :------------ | :----------------------------------- | :-------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------- |
| `url`         | `url`, `headers`, `headersHelper`    | git 형식과 일치하지 않는 `http://` 또는 `https://` URL                                                                                 | 로드                                                   | 동일한 URL 허용                                                                                                                                                            | 동일한 URL 차단                          |
| `github`      | `repo`, `ref`, `path`, `sparsePaths` | `owner/repo`, `owner/repo@ref`, 또는 `owner/repo#ref`                                                                         | 로드                                                   | 동일한 `repo`, `ref`, `path` 허용. `repo`는 `owner/*`일 수 있음                                                                                                                 | 동일한 것 차단, 동일한 저장소에 대한 `git` URL     |
| `git`         | `url`, `ref`, `path`, `sparsePaths`  | `user@host:path` URL, 또는 `.git`로 끝나거나 `/_git/`를 포함하거나 github.com 또는 gitlab.com 저장소를 이름 지정하는 `https://` URL. `#ref`는 ref를 고정 | 로드                                                   | 동일한 URL, `ref`, `path` 허용                                                                                                                                             | 동일한 것 차단, 동일한 github.com 저장소의 다른 철자 |
| `npm`         | `package`                            | 생성되지 않음                                                                                                                     | 로드 실패: `NPM marketplace sources not yet implemented` | 구문 분석되지만 아무것도 등록하지 않으므로 일치하는 것이 없음                                                                                                                                    | 구문 분석되지만 일치하는 것이 없음                 |
| `file`        | `path`                               | `.json` 파일의 경로                                                                                                              | 로드                                                   | 동일한 경로 허용                                                                                                                                                             | 동일한 경로 차단                           |
| `directory`   | `path`                               | 디렉토리의 경로                                                                                                                    | 로드                                                   | 동일한 경로 허용                                                                                                                                                             | 동일한 경로 차단                           |
| `settings`    | `name`, `plugins`, `owner`           | 생성되지 않음                                                                                                                     | 로드                                                   | 동일한 `name` 및 동일한 `plugins`를 가진 항목 허용                                                                                                                                  | 동일한 `name` 차단                       |
| `skills-dir`  | none                                 | 생성되지 않음                                                                                                                     | 로드 실패: `Unsupported marketplace source type`         | 허용 목록이 설정된 동안 [skills-directory 플러그인](/docs/ko/plugins/org#keep-skills-directory-plugins-loading) 로드 유지. [정책 목록에서만 유효한 소스 값](#source-values-valid-only-in-policy-lists) 참조 | skills-directory 플러그인 로드 중지         |
| `hostPattern` | `hostPattern`                        | 생성되지 않음                                                                                                                     | 로드 실패: `Unsupported marketplace source type`         | 호스트가 일치하는 `github`, `git`, `url` 소스 허용                                                                                                                                | 이러한 소스 차단                           |
| `pathPattern` | `pathPattern`                        | 생성되지 않음                                                                                                                     | 로드 실패: `Unsupported marketplace source type`         | `path`가 일치하는 `file` 및 `directory` 소스 허용                                                                                                                               | 이러한 소스 차단                           |

<h3 id="fields-by-type">
  유형별 필드
</h3>

표는 기본값, 제약 또는 유형별 의미를 가진 각 마켓플레이스 소스 필드를 나열합니다.

| 필드              | 유형              | 설명                                                                                                                                                                                                               |
| :-------------- | :-------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `url`           | `url`           | `marketplace.json` 파일에 대한 링크. Claude Code는 해당 파일만 다운로드하므로, 마켓플레이스의 플러그인은 [상대 경로 소스](#relative-path-plugin-source)를 사용할 수 없습니다                                                                                    |
| `url`           | `git`           | 복제할 git 저장소                                                                                                                                                                                                      |
| `headers`       | `url`           | Claude Code가 가져오기와 함께 보내는 HTTP 헤더의 맵, 인증된 호스트의 경우                                                                                                                                                                |
| `headersHelper` | `url`           | 값이 너무 단기간인 헤더를 인쇄하는 명령. Claude Code v2.1.238 이상 필요. [아카이브 다운로드 인증](/docs/ko/plugins/host-marketplace#authenticate-archive-downloads) 참조                                                                               |
| `repo`          | `github`        | `marketplace add` 및 `extraKnownMarketplaces`에서, `repo`는 하나의 저장소를 이름 지정해야 합니다. `marketplace add`는 `owner/*`를 유효한 `owner/repo` 단축형이 아닌 것으로 거부합니다. `extraKnownMarketplaces`에서 Claude Code는 이를 문자 그대로 사용하고 클론이 실패합니다 |
| `ref`           | `github`, `git` | 분기 또는 태그. 저장소의 기본 분기로 기본값 설정됨                                                                                                                                                                                    |
| `path`          | `github`, `git` | 저장소 내부의 마켓플레이스 파일의 경로. `.claude-plugin/marketplace.json`으로 기본값 설정됨                                                                                                                                               |
| `path`          | `file`          | 마켓플레이스 파일 자체. Claude Code는 제자리에서 읽고 두 수준 위의 디렉토리를 마켓플레이스 루트로 사용하므로, 파일을 `<root>/.claude-plugin/marketplace.json`에 유지하세요                                                                                          |
| `path`          | `directory`     | 마켓플레이스 루트, `.claude-plugin/marketplace.json`을 포함하는 디렉토리                                                                                                                                                          |
| `sparsePaths`   | `github`, `git` | 스파스 체크아웃을 위한 디렉토리의 배열. 예: `[".claude-plugin", "plugins"]`. `claude plugin marketplace add --sparse`가 설정합니다                                                                                                       |
| `skipLfs`       | `github`, `git` | 수락되고 효과 없음. [Git LFS에서 플러그인 파일 유지](/docs/ko/plugins/host-marketplace#keep-plugin-files-out-of-git-lfs) 참조                                                                                                             |
| `name`          | `settings`      | `extraKnownMarketplaces` 키와 같아야 하며 [예약된 이름](#reserved-names)이 될 수 없습니다                                                                                                                                           |
| `plugins`       | `settings`      | 호스팅된 파일이 없는 인라인 카탈로그. 각 항목은 `name`, `source`, `description`, `version`, `strict`, `headers`, `headersHelper`를 사용합니다. 상대 경로가 확인될 저장소가 없으므로 각 항목의 `source`를 객체 유형으로 작성하세요                                          |

<h3 id="source-values-valid-only-in-policy-lists">
  정책 목록에서만 유효한 소스 값
</h3>

`hostPattern`, `pathPattern`, `skills-dir`, `repo`의 `owner/*` 형식은 두 정책 목록인 `strictKnownMarketplaces` 및 `blockedMarketplaces`에서만 유효합니다:

* **`hostPattern` 및 `pathPattern`**: Claude Code가 가져오기 전에 소스에 대해 테스트하는 정규 표현식.
* **`skills-dir`**: 소스가 아닙니다. `strictKnownMarketplaces`를 설정하면, [skills-directory 플러그인](/docs/ko/plugins/org#keep-skills-directory-plugins-loading)은 해당 목록에 `{"source": "skills-dir"}`을 추가할 때까지 로드를 중지합니다.
* **`owner/*`**: 마켓플레이스 소스의 `github` `repo` 값으로, 정확히 해당 GitHub 소유자 아래의 모든 저장소와 일치합니다. Claude Code v2.1.223 이상 필요.

일치 순서, 정확한 `ref` 의미론, 레시피는 [조직의 플러그인 관리](/docs/ko/plugins/org)를 참조하세요.

<h3 id="source-objects-in-settings">
  설정의 소스 객체
</h3>

`extraKnownMarketplaces` 값은 마켓플레이스 이름에서 `source`를 가진 객체로의 맵입니다. 이 항목은 `main` 분기의 git 저장소에서 마켓플레이스를 등록합니다:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": {
        "source": "git",
        "url": "https://git.example.com/your-org/your-marketplace.git",
        "ref": "main"
      }
    }
  }
}
```

`strictKnownMarketplaces` 및 `blockedMarketplaces`는 소스 객체의 배열입니다. 이 허용 목록은 하나의 GitHub 소유자와 하나의 내부 호스트를 허용합니다:

```json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "your-org/*" },
    { "source": "hostPattern", "hostPattern": "^git\\.example\\.com$" }
  ]
}
```

<h2 id="validation-messages">
  유효성 검사 메시지
</h2>

`claude plugin validate <path>`는 마켓플레이스 루트 또는 마켓플레이스 파일 자체를 사용합니다. 오류 및 경고를 출력합니다. 종료 코드 및 `--strict`에 대해서는 [plugin validate](/docs/ko/plugins/cli-reference#plugin-validate)를 참조하십시오.

메시지는 플러그인 항목을 인덱스로 이름 지으며, `plugins.1.source` 또는 `plugins[1].source`로 작성됩니다.

항목 인덱스 및 `plugin.json →`으로 시작하는 메시지(예: `plugins[2] plugin.json →`)는 해당 플러그인의 자체 파일에 관한 것입니다. [`claude plugin validate` 오류 보고](/docs/ko/plugins/troubleshooting#claude-plugin-validate-reports-errors)에서 이러한 메시지와 해결 방법을 나열합니다.

Claude Desktop 플래그 이름을 언급하는 경고는 Claude Code가 허용하지만 Claude Desktop이 거부하는 것입니다. Claude Desktop의 이름 규칙이 더 엄격하기 때문입니다.

표는 마켓플레이스 수준의 메시지를 각 메시지가 관련된 필드에 매핑합니다.

| 메시지                                                                                                                                                                                                 | 수준 | 필드                                                                               |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :- | :------------------------------------------------------------------------------- |
| `Marketplace must have a name`                                                                                                                                                                      | 오류 | `name`이 비어 있음                                                                    |
| `Marketplace name cannot contain spaces. Use kebab-case (e.g., "my-marketplace")`                                                                                                                   | 오류 | `name`                                                                           |
| `Marketplace name cannot contain path separators (/ or \), ".." sequences, or be "."`                                                                                                               | 오류 | `name`                                                                           |
| `Marketplace name impersonates an official Anthropic/Claude marketplace`                                                                                                                            | 오류 | `name`. [예약된 이름](#reserved-names) 참조                                             |
| `Marketplace name cannot contain control or bidirectional-formatting characters`                                                                                                                    | 오류 | `name`에 이스케이프 또는 줄 바꿈과 같은 제어 문자 또는 유니코드 양방향 서식 문자 포함                             |
| `Marketplace name "inline" is reserved for --plugin-dir session plugins`, and the `builtin`, `skills-dir`, `synced`, `claude-plugin-test`, `npm`, `pip`, `uv`, `cargo`, `github`, and `gh` variants | 오류 | `name`                                                                           |
| `Author name cannot be empty`                                                                                                                                                                       | 오류 | `owner.name`                                                                     |
| `Plugin name cannot contain spaces. Use kebab-case (e.g., "my-plugin")`                                                                                                                             | 오류 | `plugins[i].name`                                                                |
| `Plugin name cannot contain control or bidirectional-formatting characters`                                                                                                                         | 오류 | `plugins[i].name`                                                                |
| `Duplicate plugin name "x" found in marketplace`                                                                                                                                                    | 오류 | 두 항목이 `name` 공유                                                                  |
| `plugins.i.source: Invalid input`                                                                                                                                                                   | 오류 | 항목의 `source`가 어떤 유형과도 일치하지 않음. [source의 잘못된 입력](#invalid-input-on-a-source) 참조   |
| `plugins[i].source: Path contains "..": <path>`                                                                                                                                                     | 오류 | 마켓플레이스 루트를 벗어나는 상대 `source`                                                      |
| `source.source: 'unsupported' is a parse-time placeholder and cannot be authored`                                                                                                                   | 오류 | `plugins[i].source`                                                              |
| `Plugin "x" sets headersHelper but is not "strict": false`                                                                                                                                          | 오류 | `plugins[i].headersHelper`, `archive` 항목에서                                       |
| `chain does not resolve (<reason>) — target must be a name in plugins[], a key in renames, or null`                                                                                                 | 오류 | `renames.<old>`                                                                  |
| `target "x" is not a valid plugin name (PluginIdSchema)`                                                                                                                                            | 오류 | `renames.<old>`                                                                  |
| `Unknown field 'x'. Claude Code ignores it at load time.`                                                                                                                                           | 경고 | 최상위 수준, `metadata` 아래, 항목 내, 또는 항목의 `relevance` 아래의 명명된 키                        |
| `Marketplace has no plugins defined`                                                                                                                                                                | 경고 | `plugins`이 비어 있음                                                                 |
| `Plugin "x" sets headers/headersHelper, which only apply to "archive" sources; they have no effect on this entry.`                                                                                  | 경고 | `plugins[i].headers` 또는 `plugins[i].headersHelper`, `source`가 `archive`가 아닌 항목에서 |
| `Plugin "x" fetches its archive with a headersHelper but sets no sha256 pin`                                                                                                                        | 경고 | `plugins[i].source.sha256`                                                       |
| `Header "x" is a request-routing/identity header that catalog entries may not set; Claude Code drops it at download time.`                                                                          | 경고 | `plugins[i].headers.<name>`                                                      |
| `Local source "x" is or traverses a symlink, so <path> was not read`                                                                                                                                | 경고 | `plugins[i].source`                                                              |
| `No marketplace description provided. Adding a description helps users understand what this marketplace offers`                                                                                     | 경고 | `description`                                                                    |
| `Entry declares version "x" but <path>/plugin.json says "y". At install time, plugin.json wins`                                                                                                     | 경고 | `plugins[i].version`, 상대 경로 항목에서                                                 |
| `'relevance' must be an object containing topic and signals; got <type>. It will be ignored at load time.`                                                                                          | 경고 | `plugins[i].relevance`                                                           |
| `'metadata' must be a free-form object; got <type>. It will be ignored at load time.`                                                                                                               | 경고 | `plugins[i].metadata`                                                            |
| `'experimental' must be an object containing component declarations; got <type>. It will be ignored at load time.`                                                                                  | 경고 | `plugins[i].experimental`                                                        |
| `Marketplace name "x" is reserved in Claude Desktop`                                                                                                                                                | 경고 | `name`이 `org`, `org-provisioned`, 또는 `unknown`. Claude Desktop이 마켓플레이스 거부        |
| `Marketplace name "x" is not accepted by Claude Desktop (letters, digits, ".", "_", "-"; must start alphanumeric; max 128 chars)`                                                                   | 경고 | `name`. Claude Desktop이 마켓플레이스 거부                                                |
| `Plugin name "x" is not accepted by Claude Desktop (letters, digits, ".", "_", "-"; must start alphanumeric; max 128 chars)`                                                                        | 경고 | `plugins[i].name`. Claude Desktop이 항목 삭제                                         |

<h3 id="invalid-input-on-a-source">
  source의 잘못된 입력
</h3>

`source`의 `Invalid input`은 객체가 어떤 source 유형과도 일치하지 않음을 의미합니다. 다음 원인을 확인하십시오:

* `./`로 시작하지 않는 상대 경로(`"."` 또는 [metadata.pluginRoot 아래의 베어 이름](#relative-path-plugin-source) 제외)
* `..`를 포함하는 `npm` `package`
* [플러그인 source](#plugin-sources) 중 하나가 아닌 `source` 유형
* 필수 필드가 누락되었거나 잘못된 유형의 알려진 유형(예: `repo` 없는 `github`)

<h3 id="failures-that-validation-doesn’t-catch">
  유효성 검사가 포착하지 못하는 오류
</h3>

`claude plugin validate`는 모든 오류를 보고하지 않습니다. 파일 경로 또는 배열로 작성된 항목 `hooks`는 유효성 검사를 통과하며, 오류는 플러그인이 로드될 때만 나타나며, [항목의 Hooks](#hooks-in-an-entry)에서 설명합니다. `source`를 가져오는 오류도 유효성 검사가 아닌 설치 후에만 나타납니다.

[`claude plugin list`](/docs/ko/plugins/cli-reference)는 로드에 실패한 플러그인을 오류와 함께 표시하며, [플러그인 문제 해결](/docs/ko/plugins/troubleshooting)에서 로드 시간 문자열을 다룹니다.

<h2 id="next-steps">
  다음 단계
</h2>

* [마켓플레이스 만들기](/docs/ko/plugins/create-marketplace): 이러한 필드에서 마켓플레이스를 빌드하고 로컬에서 설치
* [마켓플레이스 호스팅 및 유지보수](/docs/ko/plugins/host-marketplace): 파일을 어디에 넣을지, 사용자가 변경 사항을 받는 방식
* [플러그인 매니페스트 참조](/docs/ko/plugins/manifest-reference): 항목이 재정의할 수 있는 `plugin.json` 필드
* [조직의 플러그인 관리](/docs/ko/plugins/org): 이러한 소스 값을 사용하는 허용 목록 및 차단 목록 레시피
