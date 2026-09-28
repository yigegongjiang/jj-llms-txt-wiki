> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 플러그인 로딩 참조

> Claude Code가 각 플러그인을 어디에서 로드하는지, 어떤 설정 파일이 로드 여부를 결정하는지, 그리고 업데이트가 아무것도 변경하지 않은 이유를 추적합니다.

플러그인이 로드되지 않았거나, 예상과 다른 복사본이 로드되었거나, 업데이트를 적용하지 않았을 때 어떤 소스, 설정 범위 또는 디스크의 파일이 그 결정을 내렸는지 확인하려면 이 페이지를 사용합니다. 세션이 시작될 때와 `/reload-plugins`를 실행할 때마다 Claude Code가 적용하는 규칙을 제공합니다. Claude에게 이 페이지를 읽고 설정을 진단하도록 요청할 수도 있습니다.

<Note>
  다음 경우는 다른 페이지에서 다룹니다:

  * **설치, 활성화, 비활성화 및 업데이트 단계**: [플러그인 설치 및 관리](/docs/ko/plugins/install) 참조
  * **특정 오류 메시지가 있는 경우**: [플러그인 문제 해결](/docs/ko/plugins/troubleshooting) 참조
</Note>

설치된 플러그인이 통과하는 세 단계에 대해 [플러그인이 도달한 단계 확인](#check-which-stage-a-plugin-reached)부터 시작하거나, 보고 있는 상황과 일치하는 섹션으로 이동합니다:

* 끈 플러그인이 여전히 로드됨: [플러그인이 활성화된 위치 찾기](#find-where-a-plugin-is-enabled)
* 업데이트가 아무것도 변경하지 않음: [버전 및 업데이트](#versions-and-updates)
* `~/.claude/plugins/` 아래의 파일을 보고 있음: [디스크에서 플러그인 찾기](#find-plugins-on-disk)
* `--plugin-dir` 플러그인이 로드되지 않았거나 같은 이름의 플러그인이 대신 로드됨: [이름 충돌](#name-conflicts)

<h2 id="check-which-stage-a-plugin-reached">
  플러그인이 도달한 단계 확인
</h2>

`enabledPlugins` 항목은 플러그인을 사용할 수 있는 단계를 거칩니다: 설정이 선언하고, Claude Code가 디스크에 가져오고, 실행 중인 세션이 로드합니다. 플러그인이 설정 파일이 제안하는 대로 작동하지 않을 때 어떤 단계에 도달했는지 확인합니다:

* **선언됨, 설정에서**: `enabledPlugins`는 어떤 플러그인이 켜져 있어야 하는지 말하고, `extraKnownMarketplaces`는 어떤 마켓플레이스가 존재해야 하는지 말합니다. `claude plugin marketplace add`를 실행하면 Claude Code는 마켓플레이스를 사용자 설정의 `extraKnownMarketplaces`에 쓰고 디스크에도 씁니다
* **가져옴, `~/.claude/plugins/` 아래 디스크에**: Claude Code가 가져온 것의 기록과 가져온 파일 자체:
  * `known_marketplaces.json`은 Claude Code가 가져온 각 마켓플레이스를 `source`, `installLocation`, `lastUpdated`, `autoUpdate`와 함께 기록합니다. 사용자당 하나의 `known_marketplaces.json`이 있으므로 한 프로젝트에서 추가한 마켓플레이스는 모든 프로젝트에서 사용 가능합니다
  * `installed_plugins.json`은 각 설치를 `scope`, `installPath`, `version`과 함께 기록합니다
  * `cache/`는 플러그인 파일을 보유합니다
* **로드됨, 실행 중인 세션에서**: Claude Code가 시작 시 또는 마지막 `/reload-plugins`에서 로드한 플러그인 세트입니다. 설정 또는 디스크의 변경 사항은 `/reload-plugins`를 실행하거나 새 세션을 시작할 때까지 이 계층에 도달하지 않습니다. 이것이 `claude plugin update`가 `Restart to apply changes.`로 끝나고 백그라운드 업데이트가 `Run /reload-plugins to apply`로 프롬프트하는 이유입니다

<h3 id="plugins-and-marketplaces-that-aren’t-on-disk-at-session-start">
  세션 시작 시 디스크에 없는 플러그인 및 마켓플레이스
</h3>

플러그인은 세션 시작 시 `installed_plugins.json`과 캐시에서 네트워크를 사용하지 않고 로드됩니다. 세션이 시작된 후 Claude Code는 백그라운드에서 선언된 마켓플레이스를 확인합니다:

* **설정이 선언하지만 `known_marketplaces.json`이 부족한 마켓플레이스**: Claude Code는 이를 복제한 다음 플러그인을 다시 로드하고 아직 캐시되지 않은 활성화된 플러그인을 다운로드합니다
* **선언된 마켓플레이스의 소스가 설정에서 변경됨**: Claude Code는 새 소스에서 다시 가져오고 `Plugins changed. Run /reload-plugins to activate.`를 표시합니다

활성화된 플러그인이 어느 경로도 가져오지 않았고 사용 가능한 캐시 디렉토리가 없으면 `/plugin` **Errors** 탭에 `Plugin "<name>" not cached at <path>`가 표시되고, `claude plugin list`는 같은 줄에 `— run /plugin to refresh`를 추가합니다. 수정 방법은 [`Plugin "<name>" not cached at <path>`](/docs/ko/plugins/troubleshooting#plugin-not-cached-at)를 참조합니다.

<h2 id="find-where-a-plugin-came-from">
  플러그인이 어디에서 왔는지 찾기
</h2>

모든 플러그인에는 `<name>@<origin>` 형식의 id가 있으며, 이는 설정 파일과 `claude plugin list --json`에서 볼 수 있습니다. `@` 뒤의 부분은 Claude Code가 플러그인을 찾은 위치를 알려줍니다:

| ID 끝             | 플러그인이 어떻게 도착했는지                                                                                                                                                    | 켜거나 끄는 방법                                                                                                         |
| :--------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------- |
| `@<marketplace>` | 추가한 마켓플레이스에서 설치함                                                                                                                                                   | 설정 파일의 `enabledPlugins` 아래에서 `"<name>@<marketplace>": true` 또는 `false`                                            |
| `@inline`        | `--plugin-dir` 또는 `--plugin-url`로 Claude Code를 시작했거나, [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/ko/env-vars#variables)를 설정했거나, Agent SDK 앱이 `plugins` 옵션을 전달했습니다. 해당 세션에만 로드됩니다 | 매니페스트가 `defaultEnabled: false`를 설정하거나 설정 파일이 `"<name>@inline": false`를 설정하지 않는 한 세션에 대해 켜짐                        |
| `@skills-dir`    | `.claude-plugin/plugin.json`이 있는 플러그인 디렉토리를 `~/.claude/skills/` 또는 프로젝트의 `.claude/skills/` 아래에 저장했습니다                                                              | 매니페스트의 `defaultEnabled`, 설정 파일이 `"<name>@skills-dir"`을 `true` 또는 `false`로 설정하지 않는 한                               |
| `@synced`        | 사용자 또는 조직이 claude.ai 계정에 대해 켜고 Claude Code가 [다운로드했습니다](#synced-plugins)                                                                                            | 매니페스트가 `defaultEnabled: false`를 설정하거나 설정 파일이 `"<name>@synced": false`를 설정하지 않는 한 켜짐. 조직이 필수로 표시한 플러그인은 관계없이 로드됩니다 |

마켓플레이스 플러그인의 경우 `<name>`은 `marketplace.json`의 항목 이름입니다. `@inline` 및 `@skills-dir`의 경우 플러그인의 매니페스트에서 `name`입니다.

이 표의 원본 이름은 예약되어 있으므로 마켓플레이스는 `inline`, `skills-dir` 또는 `synced`로 명명될 수 없습니다.

<h3 id="entry-name-and-manifest-name">
  항목 이름 및 매니페스트 이름
</h3>

마켓플레이스 플러그인에는 두 개의 이름이 있으며 다를 수 있습니다:

* **`marketplace.json`의 항목 이름**: 설치 및 활성화 키입니다. `enabledPlugins`에 작성하는 것, 캐시 디렉토리의 이름이 지정되는 것, `claude plugin list`가 표시하는 것입니다
* **매니페스트의 `name`**: 플러그인의 구성 요소가 네임스페이스되는 것, [이름 충돌](#name-conflicts)이 비교하는 것입니다

<h3 id="plugins-shared-through-a-repository">
  저장소를 통해 공유된 플러그인
</h3>

저장소를 통해 플러그인을 공유하려면 `.claude/settings.json`의 `enabledPlugins` 아래에 나열하거나 `.claude/skills/` 아래에 배치합니다. Claude Code는 프로젝트의 `.claude/plugins/` 디렉토리를 스캔하지 않습니다.

클라우드 세션은 저장소가 [`extraKnownMarketplaces`](/docs/ko/settings-reference#extraknownmarketplaces) 아래에 나열하는 마켓플레이스를 추가하지 않습니다. 이는 작업 영역 신뢰 대화 상자가 필요하기 때문이며, 클라우드 세션은 절대 표시하지 않습니다.

프로젝트 범위 기술 디렉토리 플러그인은 세션의 [기본 작업 디렉토리](/docs/ko/permissions#working-directories)의 `.claude/skills/`에서만 로드되며, 해당 폴더에 대한 [작업 영역 신뢰 대화 상자](/docs/ko/permissions#what-runs-before-you-trust-a-folder)를 수락한 후에만 로드됩니다. 일반 기술 및 명령이 하는 방식으로 [저장소 루트까지 부모 디렉토리를 검색](/docs/ko/skills#discovery-from-parent-and-nested-directories)하지 않습니다. 하위 디렉토리에서 시작하면 저장소 루트의 플러그인이 로드되지 않습니다. 대신 저장소 루트에서 시작하거나, [v2.1.246 이상에서 `/cd`로 세션을 이동](/docs/ko/permissions#move-the-session-to-another-directory)합니다.

프로젝트 범위 플러그인은 저장소에 체크인되고 이를 복제하는 모든 협력자에게 도달합니다. 해당 콘텐츠는 사용자가 아닌 저장소에서 오기 때문에 `.claude/settings.json`의 프로젝트 허용 규칙에 적용되는 것과 동일한 신뢰 확인 후에만 로드됩니다. 부모 폴더를 신뢰하거나 `-p`로 실행하는 것으로는 충분하지 않습니다. 코드를 실행하는 구성 요소는 추가로 제한됩니다:

* 선언하는 MCP 서버는 프로젝트 `.mcp.json`과 동일한 [서버별 승인](/docs/ko/mcp)을 거칩니다
* [MCP 번들](/docs/ko/plugins/manifest-reference#mcpservers)로 선언하는 MCP 서버, `.mcpb` 또는 `.dxt` 파일, 또는 플러그인 디렉토리 외부의 파일에서 선언하는 MCP 서버는 건너뜁니다. 인라인으로 선언하거나 플러그인 디렉토리 내의 `.mcp.json`에서 선언합니다
* [백그라운드 모니터](/docs/ko/plugins/components#monitors)는 로드되지 않습니다

개인 범위 플러그인에는 이러한 제한이 없습니다.

`--plugin-dir` 및 기술 디렉토리 플러그인을 작성하는 방법은 [플러그인 생성](/docs/ko/plugins/create)을 참조합니다.

<h3 id="synced-plugins">
  claude.ai에서 동기화된 플러그인
</h3>

claude.ai 계정에 대해 켜는 플러그인도 마켓플레이스에서 설치한 플러그인과 함께 Claude Code에 로드됩니다. 여기에는 조직이 구성원에 대해 켜는 플러그인이 포함됩니다. 이러한 각 플러그인은 마켓플레이스 없이 `<name>@synced`로 로드되며 [설치 기록](#check-which-stage-a-plugin-reached)이 없습니다.

터미널 세션에서 동기화된 플러그인의 기술, 에이전트, 훅, MCP 서버 및 LSP 서버는 모두 마켓플레이스 플러그인을 설치한 것과 동일한 신뢰로 로드됩니다.

Cowork가 로드하는 구성 요소는 claude.com의 [claude.ai 및 Cowork의 플러그인](https://claude.com/docs/plugins/overview)을 참조합니다.

동기화된 플러그인은 Cowork 세션 및 claude.ai 계정으로 로그인하는 터미널 세션에 로드됩니다:

* **[Cowork](https://claude.com/product/cowork)**: Claude Code는 세션이 시작될 때 세션의 자체 환경으로 다운로드합니다
* **터미널 세션**: Claude Code를 시작할 때마다 백그라운드에서 한 번 동기화되어 새로운 플러그인과 업데이트된 플러그인을 다운로드하고 사용자 또는 조직이 끈 플러그인을 제거합니다. 터미널 세션에서 동기화하려면 Claude Code v2.1.273 이상이 필요합니다

<h4 id="sync-timing-in-terminal-sessions">
  터미널 세션의 동기화 타이밍
</h4>

터미널 동기화가 백그라운드에서 실행되기 때문에 세션이 시작된 후에 완료될 수 있습니다. 대화형 세션에서 동기화된 플러그인을 추가, 업데이트 또는 제거할 때 `Plugins changed. Run /reload-plugins to activate.`가 표시됩니다. `/reload-plugins`를 실행하여 해당 세션에서 변경 사항을 로드하거나 다음 번 Claude Code를 시작할 때까지 기다립니다.

claude.ai에서 세션이 실행 중인 동안 플러그인을 활성화하면 다음 번 Claude Code를 시작할 때 플러그인이 다운로드됩니다.

<h4 id="sign-in-requirements-for-terminal-sync">
  터미널 동기화를 위한 로그인 요구 사항
</h4>

터미널에서 플러그인은 claude.ai 계정으로 로그인하는 세션에서만 동기화됩니다.

이전 버전의 Claude Code에 로그인한 경우 해당 로그인은 Claude Code가 백그라운드에서 갱신할 때까지 플러그인을 포함하지 않습니다. 더 빨리 액세스하려면 `/login`을 다시 실행합니다. 플러그인 동기화는 다음 번 Claude Code를 시작할 때 시작됩니다.

<h4 id="control-which-synced-plugins-load">
  로드되는 동기화된 플러그인 제어
</h4>

조직이 필수로 요구하는 플러그인을 제외하고 동기화된 플러그인을 한 번에 하나씩 끄거나 머신의 모든 동기화된 플러그인을 끌 수 있습니다:

* **한 플러그인**: 셸에서 `claude plugin disable <name>@synced`를 실행하고 세션의 `/plugin` **Installed** 탭은 모두 사용자 수준 [`enabledPlugins`](/docs/ko/settings-reference#enabledplugins)에 `"<name>@synced": false`를 저장합니다. 모든 환경의 프로젝트에서 플러그인을 유지하려면 프로젝트의 커밋된 `.claude/settings.json`에서 동일한 키를 설정합니다
* **머신의 모든 동기화된 플러그인**: 사용자 설정에서 [`syncClaudeAiPlugins`](/docs/ko/settings-reference#syncclaudeaiplugins)를 `false`로 설정하거나 조직이 [관리 설정](/docs/ko/managed-settings)에서 설정합니다. Claude Code는 다운로드를 중지하고 다음 번 시작할 때 이미 동기화한 플러그인을 `~/.claude/plugins/.trash/`로 이동하고 더 이상 로드하지 않습니다. 조직이 claude.ai에서 기술을 끄면 플러그인도 동기화를 중지합니다
* **조직이 필수로 요구하는 플러그인**: 조직이 claude.ai에서 필수로 표시한 플러그인은 이전에 비활성화했더라도 로드됩니다. `claude plugin disable`은 `Plugin "<name>@synced" is required by your organization and can't be disabled here. Contact your admin to change it.`로 거부하고, `claude plugin list`는 이를 `required by your org`로 표시합니다

claude.ai에서 플러그인을 제거하는 방법은 [설치된 플러그인 관리](/docs/ko/plugins/install#manage-installed-plugins)를 참조합니다.

<h2 id="find-where-a-plugin-is-enabled">
  플러그인이 활성화된 위치 찾기
</h2>

6개의 소스 중 어느 곳에서나 `enabledPlugins` 항목을 설정할 수 있습니다. 표는 가장 낮은 우선 순위에서 가장 높은 우선 순위로 나열하고 각각이 누구에게 적용되는지 나열합니다. 설정 파일 자체는 [설정 파일 및 영향을 미치는 사람](/docs/ko/settings#where-settings-live)을 참조합니다.

| 소스          | 설정 위치                                                                            | 도달 범위                                                                |
| :---------- | :------------------------------------------------------------------------------- | :------------------------------------------------------------------- |
| `--add-dir` | `--add-dir`로 전달하는 디렉토리의 `.claude/settings.json` 또는 `.claude/settings.local.json` | 이 세션만. `true` 값만 효과가 있으며 다른 모든 소스가 이를 재정의합니다                         |
| `user`      | `~/.claude/settings.json`                                                        | 모든 프로젝트에서 사용자                                                        |
| `project`   | `.claude/settings.json`                                                          | 저장소를 복제하는 모든 사람                                                      |
| `local`     | `.claude/settings.local.json`                                                    | 이 저장소에서만 사용자                                                         |
| `flag`      | 시작 시 전달하는 `--settings` 값                                                         | 이 세션만                                                                |
| `managed`   | [관리 설정](/docs/ko/managed-settings)                                                    | 정책이 적용되는 모든 사용자. `true`는 강제 활성화하고 `false`는 차단하며 다른 소스는 이를 재정의하지 않습니다 |

이러한 소스는 키별로 병합됩니다. 각 플러그인 id에 대해 적용되는 값은 id를 언급하는 가장 높은 우선 순위 소스의 값입니다. id를 언급하지 않는 소스는 낮은 우선 순위 소스의 값을 유효하게 유지합니다.

<h3 id="disabled-in-user-settings-but-still-loads">
  사용자 설정에서 비활성화되었지만 여전히 로드됨
</h3>

`~/.claude/settings.json`에서 플러그인을 `false`로 설정했는데 여전히 로드되면 더 높은 우선 순위 소스의 `true`가 이를 재정의하고 있습니다. `claude plugin list` 및 `/plugin`의 플러그인 행은 `Disabled in ~/.claude/settings.json but still loads — project settings enable it, which overrides your user setting`을 표시합니다. 메시지는 사용자를 재정의한 소스의 이름을 지정합니다: `project`, `project, gitignored` (`.claude/settings.local.json`의 경우), `cli flag`, 또는 `managed`.

프로젝트 활성화 플러그인을 머신에서 거부하려면 프로젝트 파일보다 우선 순위가 높은 `.claude/settings.local.json`에서 id를 `false`로 설정합니다.

<h3 id="enabled-in-project-settings-but-not-installed">
  프로젝트 설정에서 활성화되었지만 설치되지 않음
</h3>

플러그인의 유일한 `true`가 프로젝트의 `.claude/settings.json`에 있을 때 Claude Code는 [상대 경로 소스](/docs/ko/plugins/marketplace-reference#plugin-sources)가 있거나 [시드 디렉토리](/docs/ko/plugins/org#seed-containers-and-ci)가 이미 보유하지 않는 한 설치되지 않은 머신에 플러그인을 가져오지 않습니다. 대신 `/plugin` **Errors** 탭은 `Plugin "<name>" is enabled in project settings but isn't installed here`를 표시합니다.

상대 경로 플러그인은 마켓플레이스 자체에서 로드되기 때문에 설치 기록이 필요하지 않습니다.

Claude Code는 다음 소스 중 하나가 이를 `true`로 설정할 때만 외부 소스가 있는 플러그인을 가져옵니다:

* 사용자 설정
* git이 추적하지 않는 `.claude/settings.local.json`
* `--settings` 플래그
* 관리 설정

<h2 id="find-plugins-on-disk">
  디스크에서 플러그인 찾기
</h2>

Claude Code는 플러그인 파일과 상태 기록을 하나의 플러그인 루트 아래에 유지하며, 이는 [`CLAUDE_CODE_PLUGIN_CACHE_DIR`](/docs/ko/env-vars)을 설정하지 않는 한 `~/.claude/plugins`입니다. 표의 모든 경로는 해당 루트에 상대적입니다.

| 경로                                                   | 보유 내용                                                                                                                                                                                                                                                                               |
| :--------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cache/<marketplace>/<plugin>/<version>/`            | 마켓플레이스 플러그인의 설치된 각 버전당 하나의 디렉토리. `<plugin>`은 마켓플레이스 항목 이름이고 `<version>`은 [해결된 버전](#versions-and-updates)입니다. `${CLAUDE_PLUGIN_ROOT}`는 이 디렉토리를 가리킵니다                                                                                                                                 |
| `data/<plugin-id>/`                                  | 플러그인의 영구 디렉토리, `${CLAUDE_PLUGIN_DATA}`로 노출됩니다. `<plugin-id>`가 형성되는 방식은 [경로 변수 및 영구 데이터](/docs/ko/plugins/components#path-variables-and-persistent-data)를 참조합니다. Claude Code는 플러그인 구성 요소가 처음 사용할 때 생성하고 업데이트 전체에서 유지합니다. Claude Code는 마지막 범위에서 플러그인을 제거할 때 삭제하며, `--keep-data`를 전달하지 않는 한 |
| `marketplaces/<name>/`                               | GitHub, 다른 Git 호스트 또는 URL에서 추가한 마켓플레이스의 복제 또는 다운로드. 로컬 `file` 또는 `directory` 소스에서 추가한 마켓플레이스는 여기에 복사본이 없으며, `known_marketplaces.json`의 `installLocation`은 제공한 경로입니다                                                                                                                 |
| `synced/`                                            | Claude Code가 [claude.ai 계정에서 동기화한](#synced-plugins) 플러그인                                                                                                                                                                                                                            |
| `.trash/`                                            | claude.ai 동기화가 제거한 플러그인, 예를 들어 claude.ai에서 하나를 끈 후 또는 동기화를 중지한 후                                                                                                                                                                                                                    |
| `installed_plugins.json` 및 `known_marketplaces.json` | Claude Code가 설치한 것과 가져온 마켓플레이스의 기록, [플러그인이 도달한 단계 확인](#check-which-stage-a-plugin-reached) 아래에 설명됨. [claude.ai에서 호스팅되는 마켓플레이스](/docs/ko/plugins/install#add-from-claude-ai)는 대신 `known_marketplaces_claudeai.json`에 기록됩니다                                                                |
| `flagged-plugins.json`                               | Claude Code가 마켓플레이스가 목록에서 제거했기 때문에 제거한 플러그인. `/plugin`의 **Flagged** 섹션에 나타나며, [마켓플레이스 호스팅](/docs/ko/plugins/host-marketplace)을 참조합니다                                                                                                                                                     |

`${CLAUDE_PLUGIN_ROOT}`는 버전 디렉토리를 가리키기 때문에 플러그인의 루트 경로는 모든 버전과 함께 변경됩니다. 플러그인의 내구성 있는 파일을 `${CLAUDE_PLUGIN_DATA}` 대신에 유지합니다.

<h3 id="in-place-and-copied-plugins">
  제자리 및 복사된 플러그인
</h3>

Claude Code는 일부 플러그인을 보관 위치에서 제자리로 로드하고 나머지는 원본에 따라 캐시에 복사합니다:

* **`--plugin-dir` 및 기술 디렉토리 플러그인**: 디렉토리는 제자리로 로드되며 절대 복사되지 않습니다. `--plugin-url` 아카이브 또는 `--plugin-dir` `.zip`은 먼저 세션 임시 디렉토리로 추출됩니다
* **로컬 디렉토리에서 추가한 마켓플레이스의 상대 경로 플러그인**: 플러그인은 마켓플레이스 폴더 내의 경로에서 제자리로 로드됩니다. 소스 디렉토리에 대한 편집은 다음 세션 시작 또는 `/reload-plugins`에서 적용되며 버전을 증가시킬 필요가 없습니다. 플러그인의 훅 프로세스 및 MCP 및 LSP 서버는 소스 디렉토리를 가리키는 `CLAUDE_PLUGIN_ROOT`를 수신합니다. Node.js 패키지 종속성은 [종속성 설치가 실행되는 경우](#when-the-dependency-install-runs)를 참조합니다
* **[링크 모드](/docs/ko/plugins/marketplace-reference#command-plugin-source)의 `command` 소스 플러그인**: 명령이 인쇄한 디렉토리는 캐시 항목의 링크를 통해 제자리로 로드됩니다
* **다른 모든 마켓플레이스 플러그인**: Claude Code는 플러그인을 설치 시 `cache/<marketplace>/<plugin>/<version>/`에 복사하고 해당 복사본을 로드합니다. 플러그인 디렉토리 외부의 파일은 복사되지 않으므로 복사된 플러그인 내의 스크립트가 플러그인 루트 위의 경로를 읽을 때 (예: `../shared`), 찾지 못합니다

<h3 id="paths-that-escape-the-plugin-directory">
  플러그인 디렉토리를 벗어나는 경로
</h3>

플러그인이 제자리에서 로드되든 캐시된 복사본에서 로드되든 Claude Code는 자신의 디렉토리 외부에 구성 요소를 선언하도록 허용하지 않습니다. 플러그인 루트 외부로 해결되는 구성 요소 경로를 거부합니다. 경로가 `plugin.json`에 선언되든 마켓플레이스 항목에 선언되든:

* **작성된 대로 플러그인 외부를 가리키는 경로**, 예: `../shared-utils`
* **플러그인 외부로 이어지는 심볼릭 링크**, [하나의 마켓플레이스 내 플러그인 간 링크](/docs/ko/plugins/host-marketplace#share-files-within-a-marketplace-with-symlinks) 제외
* **macOS 및 Linux에서 경로에 백슬래시가 포함된 경로**, 경로가 플러그인 내에 머물더라도. 백슬래시 경로로 선언된 구성 요소는 Windows에서만 로드되므로 `./commands/deploy.md`와 같이 정방향 슬래시로 구성 요소 경로를 작성합니다

거부된 경로는 [`path escapes plugin directory`](/docs/ko/errors#path-escapes-plugin-directory) 오류로 나타나고 플러그인은 해당 구성 요소 없이 로드됩니다.

<h3 id="cleanup-of-previous-versions">
  이전 버전의 정리
</h3>

플러그인을 업데이트하거나 제거할 때 Claude Code는 이전 버전 디렉토리에 `.orphaned_at` 마커를 씁니다. 14일 후 백그라운드 정리에서 해당 디렉토리를 제거하므로 이미 이전 버전을 로드한 세션은 계속 실행됩니다.

`installed_plugins.json`이 최소한 하나의 설치를 기록하는 동안에만 스윕이 실행됩니다. 마지막 플러그인을 제거한 후 고아 디렉토리는 다른 플러그인을 설치할 때까지 유지됩니다.

<h3 id="node-js-package-dependencies">
  Node.js 패키지 종속성
</h3>

Claude Code가 플러그인을 캐시에 복사할 때 플러그인의 Node.js 패키지 종속성도 거기에 설치하므로 플러그인의 훅과 MCP 서버가 로드할 수 있습니다.

이 섹션은 플러그인이 자신의 `package.json`에 선언하는 npm 및 Bun 패키지를 다룹니다. 다른 플러그인에 의존하는 플러그인은 [플러그인 종속성 버전](/docs/ko/plugins/dependencies)을 참조합니다.

<h4 id="when-the-dependency-install-runs">
  종속성 설치가 실행되는 경우
</h4>

Claude Code는 생성할 때마다 복사된 버전 디렉토리 내에서 설치를 실행합니다:

* 플러그인을 설치할 때
* Claude Code가 플러그인을 새 버전으로 업데이트할 때
* 새 머신과 같이 활성화된 플러그인이 아직 캐시되지 않았을 때 세션 시작 시

로컬 디렉토리 마켓플레이스에서 [제자리로 로드된](#in-place-and-copied-plugins) 상대 경로 플러그인의 경우 Claude Code는 소스 디렉토리에 종속성을 설치하지 않습니다. 거기에 설치하거나 훅에서 [`${CLAUDE_PLUGIN_DATA}`](/docs/ko/plugins/components#path-variables-and-persistent-data)로 설치합니다.

설치는 플러그인의 루트 디렉토리에 `package.json`과 지원되는 잠금 파일이 모두 포함될 때만 실행됩니다. 잠금 파일은 Claude Code가 실행하는 명령을 결정합니다:

| 잠금 파일                                        | 명령                                               |
| :------------------------------------------- | :----------------------------------------------- |
| `bun.lock` 또는 `bun.lockb`                    | `bun install --frozen-lockfile --ignore-scripts` |
| `npm-shrinkwrap.json` 또는 `package-lock.json` | `npm ci --ignore-scripts`                        |

플러그인에 이러한 잠금 파일이 두 개 이상 포함되어 있으면 Claude Code는 첫 번째 일치를 사용하며 순서대로 확인합니다: `bun.lock`, `bun.lockb`, `npm-shrinkwrap.json`, `package-lock.json`.

Claude Code는 Yarn 및 pnpm 잠금 파일과 Bun 잠금 파일 옆의 `bunfig.toml`에 대한 설치를 건너뜁니다:

* 플러그인에 `yarn.lock` 또는 `pnpm-lock.yaml`만 있으면 npm 잠금 파일로 바꿉니다
* `bunfig.toml`이 Bun 잠금 파일과 같은 디렉토리에 있으면 `bunfig.toml`을 제거하거나 Bun 잠금 파일을 npm 잠금 파일로 바꿉니다

npm 잠금 파일을 포함하여 가장 많은 사용자에게 도달합니다. Claude Code는 사용자의 PATH에서 일치하는 잠금 파일의 패키지 관리자를 실행하고 해당 패키지 관리자가 누락된 경우 다른 잠금 파일을 시도하지 않습니다.

npm 소스를 통해 배포된 플러그인의 경우 npm이 게시된 패키지에서 `package-lock.json`을 제외하기 때문에 `npm-shrinkwrap.json`을 사용합니다.

<h4 id="limits-on-the-dependency-install">
  종속성 설치의 제한
</h4>

Claude Code는 이 종속성 설치를 제한하여 설치 중에 플러그인 또는 패키지의 코드가 실행되지 않도록 하고 실행 시간을 제한합니다:

* **고정 해결**: Bun 및 npm은 잠금 파일이 고정한 것을 정확히 설치하고 `package.json`과 잠금 파일이 불일치할 때 버전을 다시 해결하는 대신 실패합니다
* **라이프사이클 스크립트 없음**: `--ignore-scripts`는 `preinstall`, `install`, `postinstall` 스크립트가 실행되지 않도록 하므로 해당 스크립트에서 네이티브 모듈을 빌드하는 종속성은 다운로드되지만 이 설치 중에 컴파일되지 않습니다
* **60초 타임아웃**: Claude Code는 더 오래 실행되는 설치를 중지하고 실패로 처리합니다

Claude Code는 이 종속성 설치 전에 npm 소스 플러그인을 가져오고 패키지의 자체 설치 스크립트는 가져오는 중에 실행되지 않습니다. [npm 플러그인 소스](/docs/ko/plugins/marketplace-reference#npm-plugin-source)를 참조합니다.

자동 설치를 끌 수 없습니다. 설정이나 환경 변수가 이를 비활성화하지 않습니다.

제한된 네트워크에서는 [네트워크 액세스 요구 사항](/docs/ko/network-config#network-access-requirements)을 참조하여 허용할 호스트를 확인합니다.

<h4 id="when-the-dependency-install-fails-or-is-skipped">
  종속성 설치가 실패하거나 건너뛰어질 때
</h4>

실패하거나 건너뛴 설치는 플러그인을 절대 차단하지 않으며 각 경우는 다른 신호를 남깁니다:

* 실패한 설치 또는 Yarn 또는 pnpm 잠금 파일 또는 `bunfig.toml` 때문에 건너뛴 설치는 `claude --debug` 출력에 경고로 나타납니다
* `package.json`이 있고 잠금 파일이 없는 플러그인은 로그 항목 없이 건너뜁니다
* 시간 초과된 설치는 캐시된 복사본에 부분 `node_modules` 트리를 남길 수 있습니다

자동 설치가 종속성을 제공할 수 없을 때 [영구 데이터 디렉토리](/docs/ko/plugins/components#path-variables-and-persistent-data)로 훅에서 설치합니다. 여기에는 라이프사이클 스크립트를 빌드해야 하는 패키지, Python 종속성, Yarn 또는 pnpm으로 잠긴 플러그인이 포함됩니다.

<h2 id="versions-and-updates">
  버전 및 업데이트
</h2>

플러그인의 작성자가 새 커밋을 푸시했고 `claude plugin update`가 `<name> is already at the latest version (<version>).`를 인쇄하면 Claude Code가 플러그인에 대해 계산하는 버전은 변경되지 않으므로 디스크에서 아무것도 변경되지 않습니다.

Claude Code는 설치하는 모든 플러그인에 대해 버전을 계산하고 그 버전이 업데이트를 감지하는 방법입니다. `claude plugin update` 및 백그라운드 자동 업데이트는 버전을 다시 계산하고 `installed_plugins.json`이 기록한 것과 일치할 때 플러그인을 건너뜁니다.

버전은 플러그인의 캐시 디렉토리의 이름도 지정합니다.

`"version"`을 고정하는 매니페스트는 계산된 버전이 커밋 전체에서 동일하게 유지되는 한 가지 방법입니다. [Claude Code가 버전을 계산하는 방법](#how-claude-code-computes-the-version)을 참조합니다.

로컬 디렉토리 마켓플레이스에서 [제자리로 로드된](#in-place-and-copied-plugins) 플러그인은 버전 문자열이 무엇이든 모든 세션 시작에서 현재 소스 파일을 로드합니다. [claude.ai에서 호스팅되는 마켓플레이스](/docs/ko/plugins/install#add-from-claude-ai)의 플러그인의 경우 claude.ai가 플러그인에 대해 기록하는 버전이 버전이고 매니페스트의 `version`은 읽지 않습니다.

<h3 id="how-claude-code-computes-the-version">
  Claude Code가 버전을 계산하는 방법
</h3>

추가한 소스의 마켓플레이스의 경우 Claude Code는 플러그인의 마켓플레이스 항목의 `source` 유형으로 규칙을 선택합니다. [마켓플레이스 참조](/docs/ko/plugins/marketplace-reference#plugin-sources)는 소스 유형을 나열합니다. 해당 목록의 `command` 제외 모든 소스 유형에 대해:

1. 플러그인의 매니페스트의 `version` 필드가 먼저 옵니다
2. 그 다음 플러그인의 마켓플레이스 항목의 `version` 필드
3. 둘 다 설정되지 않으면 버전은 소스 유형에서 옵니다:

| 소스 유형                                     | `version` 필드가 설정되지 않을 때 버전                                                |
| :---------------------------------------- | :------------------------------------------------------------------------ |
| `github`, `url`, 또는 `git-subdir`          | 소스의 커밋 SHA, 12자로 단축됨. `git-subdir` 버전은 또한 하위 디렉토리 경로의 해시를 전달합니다           |
| `archive`                                 | SHA-256 다이제스트, 12자로 단축됨: 마켓플레이스 항목의 `sha256` 핀 또는 핀이 없을 때 다운로드된 파일의 다이제스트 |
| Git 호스팅 마켓플레이스 내 상대 경로                    | 설치된 디렉토리의 커밋 SHA                                                          |
| 로컬 디렉토리, 플러그인 디렉토리도 마켓플레이스도 git 저장소가 아닐 때 | `unknown`                                                                 |
| `npm`                                     | `unknown`                                                                 |

Claude Code는 설치 경로를 둘러싼 저장소 (예: git 관리 `~/.claude`)에서 버전을 가져오지 않습니다.

`command` 소스의 경우 Claude Code는 항상 명령이 생성한 것에서 버전을 파생합니다: 자체적으로 12자 해시 또는 매니페스트가 하나를 설정할 때 `<manifest version>-<hash>`. 마켓플레이스 항목의 `version`은 명령 소스에 대해 무시됩니다. 해시가 포함하는 것은 [복사 모드 및 링크 모드](/docs/ko/plugins/marketplace-reference#copy-mode-and-link-mode)를 참조합니다.

매니페스트가 먼저 오기 때문에 `"version": "1.0.0"`을 고정하는 매니페스트는 작성자가 문자열을 변경할 때까지 모든 사용자를 캐시된 복사본에 유지하며, 푸시하는 커밋이 몇 개이든 상관없습니다. 사용자가 커밋을 추적하도록 하려면 매니페스트와 항목 모두에서 `version`을 생략합니다. [마켓플레이스 호스팅](/docs/ko/plugins/host-marketplace)은 어떤 선택이 어떤 릴리스 설정에 맞는지 다룹니다.

<h3 id="when-claude-code-refreshes-a-marketplace-before-an-install">
  Claude Code가 설치 전에 마켓플레이스를 새로 고칠 때
</h3>

플러그인을 설치할 때 Claude Code는 마켓플레이스 카탈로그의 로컬 복사본에서 조회합니다. 세션에서 `/plugin install`을 실행하거나 셸에서 `claude plugin install`을 실행하고 마켓플레이스 없이 또는 마켓플레이스와 함께 플러그인의 이름을 지정할 수 있습니다. 표는 이러한 조합 중 어느 것이 로컬 복사본을 새로 고치는지 보여줍니다.

| 플러그인 이름            | 명령                                           | Claude Code가 새로 고치는 것            |
| :----------------- | :------------------------------------------- | :------------------------------- |
| `name@marketplace` | `/plugin install` 또는 `claude plugin install` | 조회 전에 명명된 마켓플레이스                 |
| `name` 단독          | `/plugin install`                            | 자동 업데이트가 켜진 마켓플레이스만, 조회가 누락된 후에만 |
| `name` 단독          | `claude plugin install`                      | 없음. 새로 고침 없이 캐시된 카탈로그를 읽습니다      |

`name@marketplace` 설치 전의 새로 고침은 마켓플레이스의 자동 업데이트 설정이나 `DISABLE_AUTOUPDATER`에 의존하지 않습니다.

새로 고침이 실패하면 설치는 캐시된 카탈로그에서 진행되고 `claude plugin install`은 `marketplace not refreshed`를 보고합니다.

Claude Code는 다음 경우에 `name@marketplace` 설치 전의 새로 고침을 건너뜁니다:

* 마켓플레이스가 로컬 `file` 또는 `directory` 소스에서 추가되었거나 [`settings` 소스](/docs/ko/settings-reference#extraknownmarketplaces)를 사용하여 설정에서 인라인으로 정의됨
* [시드 디렉토리](/docs/ko/env-vars)가 마켓플레이스를 제공함
* Claude Code가 지난 30초 내에 마켓플레이스를 새로 고침
* `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`을 설정함
* [관리 설정](/docs/ko/plugins/org#restrict-what-users-can-install)이 마켓플레이스를 차단하며, 이 경우 Claude Code도 설치를 거부합니다

<h3 id="when-auto-update-runs">
  자동 업데이트가 실행될 때
</h3>

대화형 세션에서 첫 번째 메시지를 보낸 후 Claude Code는 최대 10분의 무작위 지연을 기다립니다. 그 다음 자동 업데이트가 켜진 모든 마켓플레이스를 새로 고치고 디스크에서 설치된 플러그인을 업데이트합니다.

실행 중인 세션은 로드한 버전을 유지하고 `Plugin updated: <name> · Run /reload-plugins to apply`가 표시됩니다. 다시 로드하든 안 하든 새 버전은 다음 시작 시 로드됩니다.

<h4 id="which-marketplaces-and-plugins-auto-update">
  어떤 마켓플레이스 및 플러그인이 자동 업데이트되는지
</h4>

마켓플레이스가 자동 업데이트되는지는 설정된 첫 번째를 따릅니다:

1. **설정 파일의 `extraKnownMarketplaces` 항목의 `autoUpdate`**
2. **`known_marketplaces.json` 항목의 `autoUpdate`**, `/plugin` **Marketplaces** 아래의 **Enable auto-update** 토글이 씁니다. 설정 파일이 `extraKnownMarketplaces` 아래에서 마켓플레이스를 선언할 때 토글은 해당 설정 항목에도 `autoUpdate`를 씁니다
3. **기본값**: Anthropic의 공식 마켓플레이스 (예: `claude-plugins-official`)의 경우 켜짐, `knowledge-work-plugins` 및 `first-party-plugins`의 경우 꺼짐, [claude.ai에서 추가한 마켓플레이스](/docs/ko/plugins/install#add-from-claude-ai)의 경우 켜짐, 다른 모든 마켓플레이스의 경우 꺼짐

`DISABLE_UPDATES=1`, `DISABLE_AUTOUPDATER=1`, 또는 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1`을 설정하면 전체 패스가 꺼지고 `FORCE_AUTOUPDATE_PLUGINS=1`도 설정하지 않는 한 **Enable auto-update** 토글이 숨겨집니다. [환경 변수 참조](/docs/ko/env-vars)는 각 변수의 더 넓은 효과를 다룹니다.

자동 업데이트는 또한 마켓플레이스 항목이 `headersHelper`를 선언하는 플러그인을 건너뜁니다. [명령 대신 거부하는 설치 및 업데이트](/docs/ko/plugins/host-marketplace#installs-and-updates-that-refuse-the-command-instead-of-asking)는 이러한 플러그인이 `/plugin` **Errors** 탭에 나타나는 경우와 거기에서 업데이트하는 방법을 설명합니다.

복사된 플러그인이 세션 중에 업데이트될 때 훅 명령, 모니터, MCP 서버 및 LSP 서버는 이전 버전의 경로를 계속 사용합니다. `/reload-plugins`를 실행하여 훅, MCP 서버 및 LSP 서버를 새 경로로 전환합니다. 모니터는 세션 재시작이 필요합니다.

<h3 id="when-a-command-source-re-runs">
  명령 소스가 다시 실행될 때
</h3>

`command` 소스가 있는 플러그인은 [자동 업데이트 패스](#when-auto-update-runs)를 기다리지 않습니다. 인쇄된 디렉토리는 명령이 실행된 시점의 도구 상태를 반영하므로 Claude Code는 [수락한 명령](/docs/ko/plugins/host-marketplace#change-the-command-of-a-command-source)을 다시 실행합니다:

* 플러그인을 설치하거나 업데이트할 때마다
* 세션당 한 번씩 활성화된 각 명령 소스 플러그인에 대해 백그라운드에서 세션이 시작된 직후. 이 실행은 마켓플레이스의 자동 업데이트 설정이나 `DISABLE_AUTOUPDATER`에 의존하지 않습니다
* 시작 시 또는 `/reload-plugins`에서 활성화된 플러그인의 설치된 버전이 플러그인 캐시에서 누락되었을 때

[`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/ko/env-vars)을 설정할 때 Claude Code는 두 백그라운드 실행을 건너뜁니다. 명시적 설치 및 업데이트는 해당 변수가 설정되어 있어도 명령을 실행합니다.

명령의 해시된 출력이 변경되면 Claude Code는 결과를 새 버전으로 설치하고 실행 중인 대화형 세션에서 다시 로드하며 [`/reload-plugins`가 전환하는 동일한 구성 요소](/docs/ko/plugins/cli-reference#reload-plugins)를 전환합니다. 플러그인이 다시 로드되었다는 알림이 표시됩니다.

제자리에서 다시 로드하면 세션의 프롬프트 캐시가 무효화되면 Claude Code는 대신 `/reload-plugins`를 실행하도록 프롬프트하며, [캐시 비용에 대해 경고하고 `--force`로 다시 실행할 때 적용](/docs/ko/prompt-caching#enabling-or-disabling-a-plugin)합니다.

<h2 id="name-conflicts">
  이름 충돌
</h2>

다른 원본의 활성화된 플러그인이 매니페스트 이름을 공유할 때 이 순서는 어느 것이 로드되는지 결정하며, 가장 높은 우선 순위에서 가장 낮은 우선 순위로:

1. id가 관리 설정 `enabledPlugins`에 나타나는 플러그인, `true` 또는 `false`로. 매니페스트 이름이 id의 이름 부분과 일치하는 `--plugin-dir` 복사본은 로드되지 않으며 `--plugin-dir copy of "<name>" ignored: plugin is locked by managed settings`가 표시됩니다
2. 활성화된 `--plugin-dir`, `--plugin-url`, 또는 `CLAUDE_CODE_PLUGIN_DIRS` 플러그인. 같은 이름의 설치된 마켓플레이스 플러그인 또는 기술 디렉토리 플러그인을 대체합니다:
   * **설치된 마켓플레이스 플러그인**: 조용히 대체됨. `claude plugin list`는 여전히 마켓플레이스 행을 활성화된 것으로 표시합니다. 설정을 반영하기 때문입니다. `--debug`로 시작할 때 Claude Code가 `~/.claude/debug/` 아래에 쓰는 로그만 `Plugin "<name>" from --plugin-dir overrides installed version`을 기록합니다
   * **기술 디렉토리 플러그인**: `/plugin` **Errors** 탭 행으로 대체되며 `Not loaded — the name "<name>" is already taken by a session-only plugin (--plugin-dir / --plugin-url), which takes precedence`를 읽습니다
3. 설치된 마켓플레이스 플러그인. 같은 이름의 기술 디렉토리 플러그인은 동일한 `Not loaded` 행을 가지며 설치된 플러그인의 이름을 지정합니다
4. 기술 디렉토리 플러그인. 이 둘 사이에서 `~/.claude/skills/` 아래의 복사본이 로드되고 프로젝트의 `.claude/skills/` 복사본이 삭제되며 어떤 경로가 이를 섀도우했는지 말하는 행이 있습니다
5. [claude.ai에서 동기화된](#synced-plugins) 플러그인. 다른 원본의 활성화된 플러그인이 이름과 일치할 때 Claude Code는 해당 플러그인을 로드하고 동기화된 복사본을 로드되지 않은 것으로 보고합니다. claude.ai 복사본을 대신 사용하려면 자신의 복사본을 비활성화합니다

순서가 매니페스트 이름을 비교하기 때문에 `hello-plugin`이라는 `--plugin-dir` 플러그인은 해당 플러그인의 매니페스트도 `"name": "hello-plugin"`을 말할 때 `hello@example-marketplace`를 대체합니다.

<h3 id="keep-a-session-only-plugin-from-loading">
  세션 전용 플러그인이 로드되지 않도록 유지
</h3>

`--plugin-dir` 플러그인이 아무것도 섀도우하지 않도록 하거나 부모 프로세스가 플래그를 전달할 때 하나를 끄려면 설정 파일에서 id를 `false`로 설정합니다. 매니페스트 이름이 `hello-plugin`인 플러그인의 경우 항목은 `"enabledPlugins": {"hello-plugin@inline": false}`입니다. 비활성화된 세션 전용 플러그인은 섀도우하지 않으므로 마켓플레이스 또는 기술 디렉토리 복사본이 대신 로드됩니다.

<h2 id="next-steps">
  다음 단계
</h2>

* [플러그인 설치 및 관리](/docs/ko/plugins/install): 설치, 활성화, 비활성화 및 업데이트 단계 자체
* [플러그인 문제 해결](/docs/ko/plugins/troubleshooting): 이를 생성하는 단계별 오류 메시지
* [플러그인 명령 참조](/docs/ko/plugins/cli-reference): 이 페이지에서 명명된 플래그 및 명령
* [조직의 플러그인 관리](/docs/ko/plugins/org): 플러그인을 강제 활성화하거나 차단하는 관리 설정
