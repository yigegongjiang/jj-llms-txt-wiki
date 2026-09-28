> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 플러그인 의존성

> 플러그인이 의존하는 플러그인을 선언하고, ^1.2와 같은 버전 범위를 사용하며, Claude Code가 이를 설치, 해결 및 정리하는 방법을 알아봅니다.

플러그인 의존성은 플러그인이 의존하는 다른 플러그인입니다. 예를 들어 MCP 서버나 스킬을 호출하는 플러그인입니다. 각 의존성은 버전 제약을 선언하지 않는 한 마켓플레이스에서 제공하는 최신 버전을 추적합니다. 버전 제약은 `^2.0` 또는 `~2.1.0`과 같은 의미 있는 버전 범위이며, 이는 테스트한 범위입니다.

이 페이지는 `plugin.json`에서 의존성을 선언하는 플러그인 작성자와 릴리스에 태그를 지정하는 마켓플레이스 유지 관리자를 위한 것입니다.

<Note>
  다음 경우는 다른 페이지에서 다룹니다:

  * **의존성이 있는 플러그인 설치**: [설치된 플러그인 관리](/docs/ko/plugins/install#manage-installed-plugins)를 참조하세요.
  * **의존성 오류 읽기**: [의존성 오류](/docs/ko/plugins/troubleshooting#dependency-errors)를 참조하세요.
  * **플러그인 자체 코드가 필요로 하는 npm 및 Bun 패키지 선언**: [Node.js 패키지 의존성](/docs/ko/plugins/loading#node-js-package-dependencies)을 참조하세요.
</Note>

제약을 추가하려면 [버전 제약으로 의존성 선언](#declare-a-dependency-with-a-version-constraint)에서 시작하세요. 다른 플러그인이 의존하는 플러그인을 유지 관리하는 경우 [릴리스에 태그를 지정](#tag-plugin-releases-for-version-resolution)하여 해당 제약이 해결될 수 있도록 하세요.

<h2 id="declare-dependencies">
  의존성 선언
</h2>

<span id="decide-whether-to-constrain-dependency-versions" />버전 제약이 없으면 의존성은 사용자가 다음에 업데이트할 때마다 마켓플레이스에서 게시하는 각 새 릴리스로 이동합니다. 해당 릴리스가 플러그인이 호출하는 MCP 도구의 이름을 바꾸면 업데이트하는 모든 사용자에 대해 플러그인이 중단됩니다.

git 기반 소스의 의존성에 `~2.1.0`과 같은 제약이 있으면 플러그인이 설치된 사용자는 의존성의 `2.1.x` 패치를 계속 받고 `2.2`로 이동하지 않습니다. 자신의 일정에 따라 업그레이드하려면 최신 릴리스에 대해 테스트한 후 더 넓은 제약으로 플러그인의 새 버전을 게시하세요.

<h3 id="declare-a-dependency-with-a-version-constraint">
  버전 제약으로 의존성 선언
</h3>

플러그인의 `.claude-plugin/plugin.json`의 `dependencies` 배열에 의존성을 나열합니다. 다음 매니페스트는 버전이 지정되지 않은 의존성 하나와 제약이 있는 의존성 하나를 선언합니다:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "deploy-kit",
  "version": "3.1.0",
  "dependencies": [
    "audit-logger",
    { "name": "secrets-vault", "version": "~2.1.0" }
  ]
}
```

항목은 문자열일 수 있습니다: 이 매니페스트의 `"audit-logger"`와 같은 플러그인 이름만 또는 `"name@marketplace"`로 다른 마켓플레이스에서 해결합니다. 단순 문자열을 사용하면 플러그인은 해당 플러그인의 마켓플레이스에서 제공하는 모든 버전에 의존합니다.

버전 제약을 설정하려면 다음 필드가 있는 객체를 사용합니다. 각 필드는 문자열입니다:

| 필드            | 설명                                                                                                                                                                                                                     |
| :------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | 의존성의 플러그인 이름으로, 마켓플레이스 항목에 표시되는 대로입니다. Claude Code는 `marketplace`를 설정하지 않는 한 선언 플러그인과 동일한 마켓플레이스에서 조회합니다. 필수입니다.                                                                                                       |
| `version`     | `~2.1.0`, `^2.0`, `>=1.4` 또는 `=2.1.0`과 같은 [의미 있는 버전 범위](https://github.com/npm/node-semver#ranges)입니다. 의존성은 이 범위를 만족하는 최고 git 태그에 설치되므로 의존성의 유지 관리자는 [릴리스에 태그를 지정](#tag-plugin-releases-for-version-resolution)해야 합니다. |
| `marketplace` | `name`을 해결할 다른 마켓플레이스입니다. 허용 목록은 [다른 마켓플레이스에서 플러그인에 의존](#depend-on-a-plugin-from-another-marketplace)에 설명된 교차 마켓플레이스 의존성을 제어합니다.                                                                                       |

범위는 `^2.0.0-0`과 같은 사전 릴리스 접미사로 옵트인하지 않는 한 `2.0.0-beta.1`과 같은 사전 릴리스 버전과 일치하지 않습니다.

<h3 id="bundle-plugins-for-a-team">
  팀을 위해 플러그인 번들
</h3>

엔지니어가 한 명령으로 선별된 플러그인 세트를 설치하도록 하려면 매니페스트에 `name`과 `dependencies` 배열이 포함된 플러그인을 게시합니다. 플러그인 매니페스트는 `name`만 필요하므로 이는 유효한 플러그인이며, 설치하면 모든 의존성이 설치됩니다.

예를 들어 플랫폼 팀은 내부 마켓플레이스에서 역할별 번들을 게시할 수 있으므로 엔지니어는 각 플러그인을 별도로 설치하는 대신 하나의 `claude plugin install`을 실행합니다:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "backend-standard",
  "version": "1.0.0",
  "description": "Standard plugin set for backend engineers",
  "dependencies": [
    "secrets-vault",
    "deploy-kit",
    { "name": "db-migrate", "version": "^3.0" },
    "oncall-runbook"
  ]
}
```

나중에 표준 세트에 플러그인을 추가하려면 추가 의존성으로 새 `backend-standard` 버전을 게시합니다. 마켓플레이스가 [기본적으로 자동 업데이트하지 않을 때](/docs/ko/plugins/loading#which-marketplaces-and-plugins-auto-update) 엔지니어는 마켓플레이스에 대해 자동 업데이트를 켜거나 수동으로 업데이트합니다:

* **마켓플레이스에 대해 자동 업데이트 켜기**: 다음 자동 업데이트는 번들을 새 버전으로 이동하고 추가하는 모든 의존성을 설치합니다.
* **수동으로 업데이트**: 셸에서 `claude plugin update backend-standard`를 실행한 후 열린 세션에서 `/reload-plugins`를 실행하여 새로 추가된 의존성을 설치합니다.

엔지니어 측 단계는 [플러그인 업데이트 유지](/docs/ko/plugins/install#keep-plugins-updated)를 참조하세요.

번들을 조직의 모든 사람에게 배포하려면 관리자가 관리 설정의 `enabledPlugins`에 추가합니다. [플러그인 사전 설치 및 필수화](/docs/ko/plugins/org#pre-install-and-require-plugins)를 참조하세요.

<h3 id="depend-on-a-plugin-from-another-marketplace">
  다른 마켓플레이스에서 플러그인에 의존
</h3>

기본적으로 Claude Code는 사용자가 이미 해당 의존성을 설치하고 동일한 범위에서 활성화하지 않는 한 선언 플러그인과 다른 마켓플레이스에서 의존성을 설치하지 않습니다. 이 기본값은 한 마켓플레이스가 사용자가 검토하지 않은 소스에서 플러그인을 자동으로 설치하는 것을 방지합니다.

설치를 허용하려면 대상 마켓플레이스의 이름을 루트 마켓플레이스의 `marketplace.json`의 `allowCrossMarketplaceDependenciesOn`에 추가합니다. 루트 마켓플레이스는 사용자가 설치하는 플러그인을 호스팅하는 마켓플레이스입니다. 루트 마켓플레이스의 허용 목록만 적용됩니다.

다음 `marketplace.json`은 `deploy-kit`이 `your-shared-marketplace`에서 플러그인에 의존하도록 허용합니다:

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "allowCrossMarketplaceDependenciesOn": ["your-shared-marketplace"],
  "plugins": [
    {
      "name": "deploy-kit",
      "source": "./deploy-kit",
      "dependencies": [
        { "name": "audit-logger", "marketplace": "your-shared-marketplace" }
      ]
    }
  ]
}
```

`allowCrossMarketplaceDependenciesOn`이 누락되었거나 대상 마켓플레이스를 포함하지 않으면 Claude Code는 의존성을 설치하지 않습니다. 의존성이 마켓플레이스 항목에 선언되면 설치 자체가 `Dependency "audit-logger@your-shared-marketplace" (required by deploy-kit@your-marketplace) is in marketplace "your-shared-marketplace", which is not in the allowlist`로 시작하는 메시지로 거부되고 설정할 필드의 이름을 지정합니다. `plugin.json`에 선언되면 설치는 의존성 없이 완료되고 플러그인은 로드되지 않습니다.

허용 목록 확인은 이미 활성화된 의존성에는 적용되지 않습니다. 사용자가 먼저 `your-shared-marketplace`에서 `audit-logger`를 자신의 범위에서 설치하면 `deploy-kit`은 허용 목록을 변경하지 않고 설치됩니다.

<h3 id="test-a-plugin-and-its-dependency-locally">
  플러그인 및 해당 의존성을 로컬로 테스트
</h3>

플러그인과 의존하는 플러그인을 동시에 개발하는 경우 셸에서 Claude Code를 시작하고 [`--plugin-dir`](/docs/ko/plugins/cli-reference#flags-that-load-a-plugin-for-one-session)로 둘 다 로드합니다:

```bash theme={null}
claude --plugin-dir ./my-dependency --plugin-dir ./my-plugin
```

의존성의 로컬 복사본은 플러그인의 의존성 항목을 만족하므로 마켓플레이스에서 의존성을 설치할 필요가 없습니다.

* **`version` 필요 없음**: 로컬 `plugin.json`은 [버전 제약](#declare-a-dependency-with-a-version-constraint)이 로컬 복사본에 대해 확인되지 않으므로 `version`도 필요하지 않습니다.
* **마켓플레이스의 이름을 지정하는 항목**: 마켓플레이스의 이름을 지정하는 항목도 Claude Code v2.1.242 이상에서 로컬 복사본과 일치합니다.

마켓플레이스에서 의존성을 설치할 때까지 로컬 복사본이 비활성화되거나 없을 때마다 플러그인이 로드되지 않습니다:

* **로컬 복사본을 비활성화했습니다**: 플러그인은 다음 플러그인 로드에서 비활성화되고 오류는 `is disabled — enable it or remove the dependency`로 끝납니다. 오류가 의존성을 `<name>@inline`으로 이름 지으면 해당 식별자는 `--plugin-dir` 복사본을 나타냅니다.
* **의존성의 `--plugin-dir` 플래그 없이 세션을 시작했습니다**: 오류는 의존성이 설치되지 않았다고 보고합니다. 플래그를 다시 전달하거나 마켓플레이스에서 의존성을 설치합니다.

두 플러그인이 하나의 부모 폴더에 있으면 해당 폴더를 `--plugin-dir`에 한 번 전달할 수 있습니다. 폴더 자체가 플러그인이 아니면 Claude Code는 `.claude-plugin/plugin.json`이 있는 각 자식 폴더를 로드합니다. Claude Code v2.1.265 이상이 필요합니다.

<h2 id="tag-plugin-releases-for-version-resolution">
  다른 사용자가 의존하는 플러그인 릴리스하기
</h2>

버전 제약이 있는 다른 플러그인이 의존하는 플러그인을 유지 관리하는 경우, 해당 제약이 해결될 수 있도록 릴리스에 태그를 지정합니다. 제약은 플러그인을 호스팅하는 저장소의 git 태그에 대해 해결됩니다. 플러그인의 [플러그인 소스](/docs/ko/plugins/marketplace-reference#plugin-sources)가 `marketplace.json`에서 가리키는 저장소에 태그를 지정합니다:

* **`github`, `url`, 또는 `git-subdir` 소스**: 플러그인 자체 저장소이므로 플러그인 작성자가 태그를 생성합니다
* **`./plugins/secrets-vault`와 같은 상대 경로**: 마켓플레이스 저장소이므로 마켓플레이스 유지 관리자가 태그를 생성합니다

<h3 id="create-a-release-tag">
  릴리스 태그 생성
</h3>

각 릴리스에 `<plugin-name>--v<version>` 형식으로 태그를 지정합니다. 여기서 `<version>`은 해당 커밋의 `plugin.json`에 있는 `version` 필드와 일치합니다. plugin-name 접두사를 사용하면 하나의 마켓플레이스 저장소에서 독립적인 버전 기록을 가진 여러 플러그인을 호스팅할 수 있습니다.

플러그인 디렉터리에서 `origin` 원격이 푸시된 태그를 수신하도록 구성되어 있을 때, [`claude plugin tag`](/docs/ko/plugins/cli-reference#plugin-tag)를 사용하여 태그를 생성합니다:

```bash theme={null}
claude plugin tag --push
```

이 명령은 플러그인의 매니페스트에서 태그 이름을 빌드합니다. 태그를 생성하기 전에 다음 검사를 실행합니다:

* 플러그인 유효성 검사
* 플러그인 디렉터리가 마켓플레이스 체크아웃 내부에 있을 때 `plugin.json`과 마켓플레이스 항목이 버전에 대해 동의하는지 확인
* 플러그인 디렉터리 아래의 깨끗한 작업 트리 필요
* 태그가 이미 존재하면 거부

성공적인 실행은 `Created tag secrets-vault--v2.1.0`을 출력합니다. `--push`를 사용하면 `Pushed to origin`도 출력합니다. `--push` 없이는 직접 실행할 `git push` 명령을 출력합니다.

`--dry-run`을 전달하여 아무것도 생성하지 않고 계획을 확인합니다.

[`claude plugin tag` 참조](/docs/ko/plugins/cli-reference#plugin-tag)에 나머지 플래그가 나열되어 있습니다.

`plugin.json`의 `version`과 마켓플레이스 항목의 버전을 직접 동기화된 상태로 유지하는 한, `git tag secrets-vault--v2.1.0`을 직접 실행할 수도 있습니다.

<h3 id="constrain-a-dependency-that-has-a-non-git-source">
  비 git 소스를 가진 종속성 제약
</h3>

태그 기반 해결은 git 기반 소스에만 적용됩니다. `npm`, `archive`, 또는 `command` [플러그인 소스](/docs/ko/plugins/marketplace-reference#plugin-sources)를 가진 종속성의 경우, 제약은 어떤 버전을 가져올지 제어하지 않습니다. 플러그인이 로드될 때 여전히 확인되며, 설치된 버전이 제약을 만족하지 않으면 종속 플러그인이 비활성화됩니다.

`npm`, `archive`, 및 `command` 소스의 경우, 확인되는 버전은 종속성의 `plugin.json`에 있는 `version`입니다. 해당 종속성을 제약하기 전에 여기에 버전을 설정합니다. 버전을 설정하지 않는 `plugin.json`은 어떤 제약도 만족하지 않기 때문입니다.

Claude Code는 `command` 소스를 가진 종속성을 직접 설치하지 않으므로 사용자가 [먼저 설치합니다](/docs/ko/plugins/marketplace-reference#command-plugin-source). 또한 종속성의 [`headersHelper`](/docs/ko/plugins/host-marketplace#authenticate-archive-downloads)를 실행하지 않으므로 사용자도 마켓플레이스 항목이 하나를 설정한 종속성을 플러그인을 설치하기 전에 설치합니다.

`claude plugin install` 외에도 이러한 작업은 선언된 모든 누락된 종속성을 설치하며, `command` 및 `headersHelper` 제한이 이들에도 적용됩니다:

* `/reload-plugins`
* 종속 플러그인의 마켓플레이스 자동 업데이트
* 종속 플러그인에서 `claude plugin install` 다시 실행
* `claude plugin marketplace add`

<h2 id="how-dependencies-behave-for-your-users">
  사용자를 위해 의존성이 어떻게 작동하는지
</h2>

이 섹션은 플러그인이 다른 플러그인과 함께 설치되면 Claude Code가 선언한 제약을 해결, 확인 및 결합하는 방법을 설명합니다.

<h3 id="how-a-constraint-resolves-against-tags">
  제약이 태그에 대해 어떻게 해결되는지
</h3>

사용자가 `{ "name": "secrets-vault", "version": "~2.1.0" }`을 선언하는 플러그인을 설치하면 의존성은 `secrets-vault`를 호스팅하는 저장소에서 `~2.1.0`을 만족하는 최고 `secrets-vault--v` 태그에서 설치됩니다. 범위를 만족하는 태그가 없으면 설치는 실패하거나 마켓플레이스의 현재 복사본을 사용합니다:

* **자체 저장소가 있는 플러그인**: 설치는 `Dependency "secrets-vault@your-marketplace" has no git tag satisfying`을 포함하는 메시지로 실패합니다.
* **상대 경로로 참조되는 플러그인**: 설치는 대신 마켓플레이스의 현재 복사본을 사용하고 플러그인이 로드될 때 제약이 확인됩니다. 해당 복사본이 범위를 벗어나면 종속 플러그인은 비활성화된 상태로 유지되고 `claude plugin list`는 `Requires "secrets-vault@your-marketplace" ~2.1.0, installed 3.0.0`을 표시합니다.

마켓플레이스가 상대 경로로 참조하는 플러그인의 경우 로컬 폴더 경로로 추가한 마켓플레이스도 폴더가 git 저장소일 때 해당 폴더의 git 태그에 대해 제약을 해결합니다. Claude Code v2.1.196 이상이 필요합니다. git 저장소가 아닌 로컬 폴더에는 태그가 없으므로 Claude Code는 대신 폴더의 현재 내용에서 의존성을 설치합니다.

<h3 id="confirm-the-resolved-version">
  해결된 버전 확인
</h3>

제약이 해결된 버전을 확인하려면 셸에서 `claude plugin list`를 실행합니다. 태그 해결 의존성은 `2.1.0-8713c5b11005`와 같은 12자 커밋 접미사로 버전을 표시합니다.

제약 확인은 `plugin.json`의 `version`이 뒤처져 있더라도 태그의 버전을 사용합니다.

태그를 다른 커밋으로 강제 이동하면 다음 설치는 오래된 캐시된 복사본을 재사용하는 대신 해당 커밋의 내용을 가져옵니다. 플러그인의 버전이 캐시 키가 되는 방법은 [버전 및 업데이트](/docs/ko/plugins/loading#versions-and-updates)를 참조하세요.

<h3 id="combine-constraints-from-several-plugins">
  여러 플러그인의 제약 결합
</h3>

여러 설치된 플러그인이 동일한 의존성을 제약하면 의존성은 모든 범위를 만족하는 최고 버전으로 해결됩니다. 일반적인 조합은 다음과 같이 해결됩니다:

| 플러그인 A 필요 | 플러그인 B 필요 | 결과                                                                                    |
| :-------- | :-------- | :------------------------------------------------------------------------------------ |
| `^2.0`    | `>=2.1`   | 최고 `2.x` 태그에서 `2.1.0` 이상에서 한 번 설치합니다. 두 플러그인 모두 로드됩니다.                                |
| `~2.1`    | `~3.0`    | 플러그인 B 설치가 `has conflicting version requirements` 메시지로 실패합니다. 플러그인 A와 의존성은 그대로 유지됩니다. |
| `=2.1.0`  | 없음        | 의존성은 `2.1.0`에 유지됩니다. 플러그인 A가 설치된 동안 자동 업데이트는 최신 버전을 건너뜁니다.                            |

자동 업데이트는 마켓플레이스의 최신 버전이 아니라 모든 설치된 플러그인의 범위를 만족하는 최고 git 태그에서 제약된 의존성을 가져옵니다. 설치된 플러그인의 범위가 겹치지 않으면 자동 업데이트는 해당 의존성을 현재 버전에 유지하고 `/plugin` **오류** 탭은 제약 플러그인의 이름을 지정하는 항목을 표시합니다. 범위가 겹치지만 범위에 태그가 없으면 자동 업데이트는 마켓플레이스의 현재 복사본을 가져오고 해당 복사본의 `version`이 설치된 플러그인의 범위를 벗어나면 업데이트를 건너뜁니다.

사용자가 의존성을 제약하는 마지막 플러그인을 제거하면 의존성은 더 이상 버전 범위로 제약되지 않으며 다음 업데이트에서 마켓플레이스 항목 추적을 재개합니다.

<h2 id="see-also">
  참고 항목
</h2>

* [`claude plugin prune`](/docs/ko/plugins/cli-reference#plugin-prune): 플러그인이 더 이상 필요하지 않은 자동 설치 의존성 제거
* [마켓플레이스 호스팅](/docs/ko/plugins/host-marketplace): 릴리스 채널 및 다른 플러그인 권장
