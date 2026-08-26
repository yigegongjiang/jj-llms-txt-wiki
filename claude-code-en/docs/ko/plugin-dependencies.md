> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 플러그인 종속성 버전 제약

> 플러그인 종속성에 대한 버전 제약을 선언하고 선별된 플러그인 세트를 하나의 설치 뒤에 번들로 제공합니다.

플러그인은 `plugin.json` 또는 마켓플레이스 항목에 나열하여 다른 플러그인에 종속될 수 있습니다. 기본적으로 종속성은 최신 사용 가능 버전을 추적하므로 업스트림 릴리스가 경고 없이 플러그인의 종속성을 변경할 수 있습니다. 버전 제약을 사용하면 이동하기로 선택할 때까지 테스트된 버전 범위에서 종속성을 유지할 수 있습니다.

종속성을 선언하는 플러그인을 설치하면 Claude Code가 자동으로 종속성을 해결하고 설치하며 설치 출력의 끝에 추가된 종속성을 나열합니다. 종속성이 나중에 누락되면 `/reload-plugins`와 백그라운드 플러그인 자동 업데이트가 이를 다시 설치합니다. 단, 마켓플레이스가 이미 구성된 마켓플레이스에 있어야 합니다. 종속 플러그인에서 `claude plugin install`을 다시 실행하거나 `claude plugin marketplace add`로 마켓플레이스를 추가하면 누락된 종속성도 해결됩니다. 추가하지 않은 마켓플레이스의 종속성은 해결되지 않은 상태로 유지됩니다.

이 가이드는 `plugin.json`에서 종속성을 선언하는 플러그인 작성자와 릴리스에 태그를 지정하는 마켓플레이스 유지 관리자를 위한 것입니다. 종속성이 있는 플러그인을 설치하려면 [플러그인 검색 및 설치](/docs/ko/discover-plugins)를 참조하세요. 전체 매니페스트 스키마는 [플러그인 참조](/docs/ko/plugins-reference)를 참조하세요.

<h2 id="why-constrain-dependency-versions">
  종속성 버전을 제약하는 이유
</h2>

두 팀이 플러그인을 게시하는 내부 마켓플레이스를 생각해 봅시다. 플랫폼 팀은 비밀 백엔드를 래핑하는 MCP 서버인 `secrets-vault`를 유지 관리합니다. 배포 팀은 배포 중에 자격 증명을 가져오기 위해 `secrets-vault`를 호출하는 `deploy-kit`을 유지 관리합니다.

`deploy-kit`은 `secrets-vault` v2.1.0에 대해 테스트됩니다. 버전 제약이 없으면 플랫폼 팀이 MCP 도구의 이름을 바꾸는 릴리스에 태그를 지정할 때마다 자동 업데이트가 모든 엔지니어의 `secrets-vault`를 새 버전으로 이동하고 `deploy-kit`이 중단됩니다.

버전 제약을 사용하면 `deploy-kit`은 `~2.1.0` 범위에서 `secrets-vault`가 필요함을 선언합니다. `deploy-kit`이 설치된 엔지니어는 가장 높은 일치하는 `2.1.x` 패치에 머물러 있습니다. 배포 팀은 더 넓은 제약이 있는 새로운 `deploy-kit` 버전을 게시하여 자신의 일정에 따라 업그레이드합니다.

<h2 id="declare-a-dependency-with-a-version-constraint">
  버전 제약으로 종속성 선언
</h2>

플러그인의 `.claude-plugin/plugin.json`의 `dependencies` 배열에 종속성을 나열합니다. 각 항목은 플러그인 이름 또는 버전 제약이 있는 객체입니다.

다음 매니페스트는 하나의 버전 없는 종속성과 하나의 제약된 종속성을 선언합니다:

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

항목은 위의 예에서 `"audit-logger"`처럼 플러그인 이름만 있는 문자열일 수 있으며, 이는 해당 플러그인의 마켓플레이스가 제공하는 모든 버전에 종속됩니다. 더 많은 제어를 위해 다음 필드가 있는 객체를 사용합니다:

| 필드            | 유형     | 설명                                                                                                                                                                                          |
| :------------ | :----- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `name`        | string | 플러그인 이름입니다. 선언 플러그인과 동일한 마켓플레이스 내에서 해결됩니다. 필수입니다.                                                                                                                                           |
| `version`     | string | `~2.1.0`, `^2.0`, `>=1.4` 또는 `=2.1.0`과 같은 [semver 범위](https://github.com/npm/node-semver#ranges)입니다. 종속성은 이 범위를 만족하는 가장 높은 태그된 버전에서 가져옵니다.                                                  |
| `marketplace` | string | `name`을 해결할 다른 마켓플레이스입니다. 교차 마켓플레이스 종속성은 대상 마켓플레이스가 루트 마켓플레이스의 `marketplace.json`에서 [`allowCrossMarketplaceDependenciesOn`](#depend-on-a-plugin-from-another-marketplace)에 나열되지 않는 한 차단됩니다. |

`version` 필드는 캐럿, 틸드, 하이픈 및 비교자 범위를 포함하여 Node의 `semver` 패키지에서 지원하는 모든 표현식을 허용합니다. `^2.0.0-0`과 같은 사전 릴리스 접미사로 옵트인하지 않는 한 `2.0.0-beta.1`과 같은 사전 릴리스 버전은 제외됩니다.

<h2 id="bundle-plugins-for-a-team">
  팀을 위한 플러그인 번들
</h2>

필수 `name` 외에도 플러그인 매니페스트는 `dependencies` 배열만으로 구성될 수 있습니다. 이를 설치하면 모든 종속성이 함께 설치되므로, 이는 하나의 설치 뒤에 큐레이션된 플러그인 세트를 패키징하는 방법입니다.

예를 들어, 플랫폼 팀은 내부 마켓플레이스에서 역할별 번들을 게시하여 엔지니어가 각 도구를 별도로 설치하는 대신 하나의 `claude plugin install`을 실행하도록 할 수 있습니다:

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

`backend-standard`를 설치하면 네 가지 종속성이 모두 해결되고 설치됩니다.

나중에 표준 세트에 도구를 추가하려면 추가 종속성과 함께 새로운 `backend-standard` 버전을 게시합니다. 자동 업데이트는 기본적으로 Anthropic이 아닌 마켓플레이스에서는 꺼져 있으므로, 엔지니어는 다음 두 가지 방법 중 하나로 새 버전을 적용합니다:

* `/plugin`에서 마켓플레이스에 대한 자동 업데이트를 활성화합니다. 다음 자동 업데이트는 번들을 새 버전으로 이동하고 추가되는 모든 종속성을 설치합니다.
* `claude plugin update backend-standard`를 실행한 다음 `/reload-plugins`를 실행하여 새로 추가된 종속성을 설치합니다.

조직 전체에 번들을 배포하려면 [관리 설정](/docs/ko/settings#enabledplugins)의 `enabledPlugins`에 번들 플러그인을 추가합니다.

<h2 id="depend-on-a-plugin-from-another-marketplace">
  다른 마켓플레이스의 플러그인에 종속
</h2>

기본적으로 Claude Code는 플러그인을 선언하는 플러그인과 다른 마켓플레이스에 있는 종속성을 자동 설치하기를 거부합니다. 이는 한 마켓플레이스가 검토하지 않은 소스의 플러그인을 자동으로 가져오는 것을 방지합니다.

이를 허용하려면 루트 마켓플레이스의 유지 관리자가 대상 마켓플레이스 이름을 `marketplace.json`의 `allowCrossMarketplaceDependenciesOn`에 추가합니다. 루트 마켓플레이스는 사용자가 설치하는 플러그인을 호스팅하는 마켓플레이스이며, 해당 허용 목록만 참조되므로 신뢰가 중간 마켓플레이스를 통해 연결되지 않습니다.

다음 `marketplace.json`은 `deploy-kit`이 `acme-shared`의 플러그인에 종속되도록 허용합니다:

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "acme-tools",
  "owner": { "name": "Acme" },
  "allowCrossMarketplaceDependenciesOn": ["acme-shared"],
  "plugins": [
    {
      "name": "deploy-kit",
      "source": "./deploy-kit",
      "dependencies": [
        { "name": "audit-logger", "marketplace": "acme-shared" }
      ]
    }
  ]
}
```

필드가 없거나 대상 마켓플레이스를 포함하지 않으면 설정할 필드의 이름을 지정하는 `cross-marketplace` 오류로 설치가 실패합니다. 사용자는 여전히 종속성을 수동으로 먼저 설치할 수 있으며, 이는 허용 목록을 변경하지 않고 제약을 만족합니다.

<h2 id="tag-plugin-releases-for-version-resolution">
  버전 해결을 위한 플러그인 릴리스 태그
</h2>

버전 제약은 마켓플레이스 저장소의 git 태그에 대해 해결됩니다. Claude Code가 종속성의 사용 가능한 버전을 찾으려면 업스트림 플러그인의 릴리스가 특정 명명 규칙을 사용하여 태그되어야 합니다.

각 릴리스를 `{plugin-name}--v{version}`으로 태그하며, 여기서 `{version}`은 해당 커밋의 `plugin.json`의 `version` 필드와 일치합니다. 플러그인 디렉터리에서 다음을 실행합니다:

```bash theme={null}
claude plugin tag --push
```

`claude plugin tag` 명령은 플러그인의 매니페스트와 포함된 마켓플레이스 항목에서 태그 이름을 파생합니다. 태그를 생성하기 전에 플러그인 콘텐츠를 검증하고, `plugin.json`과 마켓플레이스 항목이 버전에 동의하는지 확인하고, 플러그인 디렉터리 아래에 깨끗한 작업 트리를 요구하며, 태그가 이미 존재하면 거부합니다. `--dry-run`을 추가하여 태그를 생성하지 않고 어떤 것이 태그될지 확인합니다. `plugin.json`과 마켓플레이스 항목을 직접 동기화 상태로 유지하면 `git tag secrets-vault--v2.1.0`을 직접 실행하는 것과 동일합니다.

플러그인 이름 접두사를 사용하면 하나의 마켓플레이스 저장소가 독립적인 버전 라인을 가진 여러 플러그인을 호스팅할 수 있습니다. `--v` 구분자는 전체 플러그인 이름에 대한 접두사 일치로 구문 분석되므로 하이픈을 포함하는 플러그인 이름이 올바르게 처리됩니다.

`{ "name": "secrets-vault", "version": "~2.1.0" }`을 선언하는 플러그인을 설치하면 Claude Code는 마켓플레이스의 태그를 나열하고, `secrets-vault--v`로 시작하는 태그로 필터링하고, `~2.1.0`을 만족하는 가장 높은 버전을 가져옵니다. 일치하는 태그가 없으면 종속 플러그인은 사용 가능한 버전을 나열하는 오류로 비활성화됩니다.

로컬 폴더 경로로 추가된 마켓플레이스는 폴더가 git 저장소일 때 동일한 방식으로 태그를 해결합니다. 이는 Claude Code v2.1.196 이상이 필요합니다. 두 가지 경우에 Claude Code는 폴더의 현재 콘텐츠에서 종속성을 설치합니다:

* 이전 버전은 로컬 폴더 마켓플레이스에서 태그를 읽지 않으므로 제약된 종속성은 해당 복사본이 범위를 만족할 때만 로드됩니다.
* git 저장소가 아닌 로컬 폴더는 버전에 관계없이 태그가 없습니다.

해결된 태그의 semver는 `plugin.json`의 `version`과 별도로 기록되므로 제약 확인은 해당 커밋의 `plugin.json`에 오래된 값이 있더라도 실제로 가져온 태그를 사용합니다. 태그 해결 설치를 위한 캐시 디렉터리 이름에는 12자 커밋-SHA 접미사가 포함되므로 유지 관리자가 태그를 다른 커밋으로 강제 이동하면 다음 설치는 오래된 콘텐츠를 재사용하는 대신 새로운 캐시 디렉터리를 가져옵니다.

<Note>
  `npm` 마켓플레이스 소스의 경우 태그 기반 해결이 git 지원 소스에만 적용되므로 제약이 가져온 버전을 제어하지 않습니다. 제약은 여전히 로드 시간에 확인되며, 설치된 버전이 이를 만족하지 않으면 종속 플러그인은 `dependency-version-unsatisfied`로 비활성화됩니다.
</Note>

<h2 id="how-constraints-interact">
  제약이 상호 작용하는 방식
</h2>

여러 설치된 플러그인이 동일한 종속성을 제약하면 Claude Code는 해당 범위를 교차하고 모든 범위를 만족하는 가장 높은 버전으로 종속성을 해결합니다. 아래 표는 일반적인 조합이 어떻게 해결되는지 보여줍니다.

| 플러그인 A 필요 | 플러그인 B 필요 | 결과                                                            |
| :-------- | :-------- | :------------------------------------------------------------ |
| `^2.0`    | `>=2.1`   | `2.1.0` 이상의 가장 높은 `2.x` 태그에서 하나의 설치입니다. 두 플러그인 모두 로드됩니다.      |
| `~2.1`    | `~3.0`    | 플러그인 B 설치가 `range-conflict`로 실패합니다. 플러그인 A와 종속성은 그대로 유지됩니다.   |
| `=2.1.0`  | none      | 종속성은 `2.1.0`에 머물러 있습니다. 플러그인 A가 설치된 동안 자동 업데이트는 최신 버전을 건너뜁니다. |

자동 업데이트는 마켓플레이스의 최신 버전이 아닌 모든 설치된 플러그인의 범위를 만족하는 가장 높은 git 태그에서 제약된 종속성을 가져오므로, 종속성은 허용된 범위 내에서 계속 업데이트를 받습니다. 모든 범위를 만족하는 태그가 없으면 자동 업데이트는 해당 종속성을 건너뛰고 `/plugin` 오류 탭에서 건너뛴 내용을 나열하며 제약 플러그인의 이름을 지정합니다.

종속성을 제약하는 마지막 플러그인을 제거하면 종속성은 더 이상 유지되지 않으며 다음 업데이트에서 마켓플레이스 항목 추적을 재개합니다.

<h2 id="enable-or-disable-a-plugin-with-dependencies">
  플러그인 종속성 활성화 또는 비활성화
</h2>

플러그인을 활성화하면 이에 종속된 플러그인도 활성화되며, 다른 활성화된 플러그인이 여전히 필요로 하면 플러그인을 비활성화할 수 없습니다. 두 동작 모두 Claude Code v2.1.143 이상이 필요합니다. 이전 버전은 명명된 플러그인만 활성화하거나 비활성화하고 다음 로드에서 `dependency-unsatisfied` 오류를 표시합니다.

플러그인을 활성화하면 Claude Code는 동일한 범위에서 해당 종속성도 활성화합니다. 종속성에 자체 종속성이 있으면 Claude Code는 이들도 활성화합니다. 성공 메시지는 명명한 플러그인과 함께 활성화된 다른 항목을 나열합니다. 종속성을 활성화할 수 없으면 명령이 거부되고 무엇이 차단하고 있는지, 어떻게 해결할지 알려줍니다:

| 조건                                       | 결과                                                         |
| :--------------------------------------- | :--------------------------------------------------------- |
| 종속성이 설치되지 않음                             | 활성화가 실패하고 누락된 각 종속성에 대해 `claude plugin install` 명령을 인쇄합니다. |
| 종속성이 조직의 플러그인 정책에 의해 차단됨                 | 활성화가 실패하고 차단된 종속성의 이름을 지정합니다.                              |
| 종속성이 대상 범위보다 우선 순위가 높은 범위에서 `false`로 설정됨 | 활성화가 실패합니다. 해당 범위에서 종속성을 활성화하거나 `--scope`를 전달하여 거기에 쓰기합니다. |
| 모든 종속성이 설치되고 허용됨                         | 활성화가 성공하고 대상 범위에서 아직 활성화되지 않은 플러그인과 각 종속성에 대해 `true`를 씁니다. |

이는 종속성이 매니페스트에서 [`defaultEnabled: false`](/docs/ko/plugins-reference#default-enablement)를 설정하는 경우에도 적용됩니다. Claude Code는 이에 대해 명시적 `true`를 쓰기 때문입니다. 설치 시에도 동일하게 적용됩니다. 활성화된 플러그인을 만족시키기 위해 가져온 종속성은 자체 기본값에 관계없이 `true`로 설치됩니다.

플러그인을 비활성화하면 다른 활성화된 플러그인이 여전히 이에 종속되면 Claude Code가 거부합니다. 오류는 이에 종속된 플러그인의 이름을 지정하고 올바른 순서로 비활성화하는 연쇄 명령을 제공하며, 요청한 것으로 끝납니다.

예를 들어 `deploy-kit`이 `secrets-vault`에 종속되면 `secrets-vault`만 비활성화하면 다음과 유사한 출력으로 실패합니다:

```text theme={null}
secrets-vault is still required by deploy-kit. Disable that plugin first, or
disable everything together: claude plugin disable deploy-kit@acme-tools && claude plugin disable secrets-vault@acme-tools
```

오류에서 연쇄 명령을 복사하여 한 단계에서 전체 집합을 비활성화합니다.

<h2 id="remove-orphaned-auto-installed-dependencies">
  고아 자동 설치 종속성 제거
</h2>

자동 설치된 종속성은 이를 설치한 플러그인이 제거된 후에도 디스크에 남아 있으며, 종속 플러그인을 다시 설치하거나 종속성을 직접 계속 사용하려는 경우를 대비합니다. 이를 정리하려면 `claude plugin prune`을 실행하여 더 이상 설치된 플러그인이 필요로 하지 않는 자동 설치된 종속성을 나열하고 확인 프롬프트 후 제거합니다. 이는 Claude Code v2.1.121 이상이 필요합니다.

```bash theme={null}
claude plugin prune
```

기본적으로 prune은 사용자 범위에서 작동합니다. `--scope project` 또는 `--scope local`을 사용하여 다른 범위를 대상으로 합니다. `--dry-run`을 전달하여 아무것도 변경하지 않고 제거될 항목을 나열합니다. `-y`를 전달하여 확인 프롬프트를 건너뜁니다. stdin 또는 stdout이 터미널이 아닐 때 prune은 고아를 나열하고 `-y`가 전달되지 않는 한 제거하지 않고 종료합니다.

제거의 일부로 prune하려면 `claude plugin uninstall`에 `--prune`을 전달합니다. 명명된 플러그인을 제거한 후 Claude Code는 이제 고아가 된 자동 설치된 종속성을 검사하고 제거합니다. 직접 설치한 플러그인은 절대 prune되지 않으며, 다른 플러그인의 `dependencies` 배열을 통해 자동으로 설치된 플러그인만 prune됩니다.

예를 들어 `deploy-kit`을 제거하고 이를 남기는 종속성을 정리하려면:

```bash theme={null}
claude plugin uninstall deploy-kit --prune
```

<h2 id="resolve-dependency-errors">
  종속성 오류 해결
</h2>

종속성 문제는 `claude plugin list` 및 `/plugin` 인터페이스에 표시됩니다. Claude Code는 오류를 해결할 때까지 영향을 받는 플러그인을 비활성화합니다. 아래 표에는 가장 일반적인 오류와 해결 방법이 나열되어 있습니다.

| 오류                               | 의미                                                                                                                          | 해결 방법                                                                                                                                                               |
| :------------------------------- | :-------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `dependency-unsatisfied`         | 선언된 종속성이 설치되지 않았거나 설치되었지만 비활성화되어 있습니다.                                                                                      | 오류 메시지에 표시된 `claude plugin install` 명령을 실행합니다. 종속성의 마켓플레이스가 아직 구성되지 않은 경우 `claude plugin marketplace add`로 추가하면 Claude Code가 종속성을 자동으로 해결합니다. 종속성이 비활성화된 경우 활성화합니다. |
| `range-conflict`                 | 종속성의 버전 요구 사항을 결합할 수 없습니다. 오류 메시지는 원인의 이름을 지정합니다: 모든 범위를 만족하는 버전이 없거나, 범위가 유효한 semver 구문이 아니거나, 결합된 범위가 너무 복잡하여 교차할 수 없습니다. | 충돌하는 플러그인 중 하나를 제거하거나 업데이트하고, 유효하지 않은 `version` 문자열을 수정하고, 긴 `\|\|` 체인을 단순화하거나, 업스트림 작성자에게 제약을 넓히도록 요청합니다.                                                          |
| `dependency-version-unsatisfied` | 설치된 종속성의 버전이 이 플러그인의 선언된 범위를 벗어났습니다.                                                                                        | `claude plugin install <dependency>@<marketplace>`를 실행하여 모든 현재 제약에 대해 종속성을 다시 해결합니다.                                                                                |
| `no-matching-tag`                | 종속성의 저장소에 범위를 만족하는 `{name}--v*` 태그가 없습니다.                                                                                   | 업스트림이 위의 규칙을 사용하여 릴리스에 태그를 지정했는지 확인하거나 범위를 완화합니다.                                                                                                                   |

이러한 오류를 프로그래밍 방식으로 확인하려면 `claude plugin list --json`을 실행하고 각 플러그인의 `errors` 필드를 읽습니다.

<h2 id="see-also">
  참고 항목
</h2>

* [플러그인 생성](/docs/ko/plugins): 기술, 에이전트 및 훅으로 플러그인 빌드
* [플러그인 마켓플레이스 생성 및 배포](/docs/ko/plugin-marketplaces): 팀을 위한 플러그인 호스팅
* [플러그인 참조](/docs/ko/plugins-reference#plugin-manifest-schema): 전체 `plugin.json` 스키마
* [버전 관리](/docs/ko/plugins-reference#version-management): 플러그인의 자체 버전이 어떻게 해결되고 캐시 키로 사용되는지
