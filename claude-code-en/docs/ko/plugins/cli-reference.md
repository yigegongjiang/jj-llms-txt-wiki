> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 플러그인 명령어 참조

> claude 플러그인 셸 명령어, 세션 내 /plugin 및 /reload-plugins, 그리고 한 세션 동안 플러그인을 로드하는 플래그에 대한 완전한 참조입니다.

플러그인 명령어는 셸이나 스크립트에서 `claude plugin`으로 실행하거나, Claude Code 세션 내에서 `/plugin` 및 `/reload-plugins`로 실행합니다. 이 참조는 각 명령어의 플래그, 기본값, 출력 및 종료 코드와 함께 한 세션 동안 플러그인을 로드하는 두 가지 플래그를 제공합니다.

빌드에서 `claude plugin --help`를 실행하여 버전에 어떤 하위 명령어가 있는지 확인하세요.

<Note>
  다음 경우는 다른 페이지에서 다룹니다:

  * **단계 설치 및 관리, 그리고 `/plugin`이 실행되는 위치**: [플러그인 설치 및 관리](/docs/ko/plugins/install) 참조
  * **명령어가 디스크에서 변경하는 내용 및 어떤 범위가 우선순위를 갖는지**: [플러그인 로딩 참조](/docs/ko/plugins/loading) 참조
  * **오류 메시지의 의미**: [플러그인 문제 해결](/docs/ko/plugins/troubleshooting) 참조
</Note>

<h2 id="claude-plugin-commands">
  claude plugin 명령어
</h2>

셸이나 스크립트에서 Claude Code 세션 외부에서 `claude plugin <subcommand>`를 실행합니다. 이러한 하위 명령어는 [`/plugin`](#plugin-in-a-session) 패널을 열지 않고 플러그인을 설치하고 관리합니다.

`claude plugins`는 `claude plugin`의 별칭입니다.

모든 하위 명령어는 다음 종료 코드, 플러그인 인수 및 범위 값을 공유합니다:

* **종료 코드**: 성공 시 `0`, 실패 시 `1`. `validate`는 예상치 못한 오류에 대해 종료 `2`를 추가하고, `eval`은 [해당 섹션](#plugin-eval)에 나열된 코드를 추가합니다.
* **플러그인 인수**: `<plugin>` 인수는 플러그인 `name` 또는 `name@marketplace`입니다. 두 마켓플레이스가 같은 이름을 제공할 때는 정규화된 형식을 사용하세요.
* **범위**: `--scope`는 `user`, `project` 또는 `local`을 사용하며, 명령어가 쓰는 설정 파일의 이름을 지정합니다. `update`는 `managed`도 사용합니다.

<h3 id="plugin-init">
  plugin init
</h3>

`~/.claude/skills/<name>/`에서 새 플러그인을 스캐폴드합니다. 다음 세션에서 설치 단계 없이 `<name>@skills-dir`로 로드됩니다.

`new`는 `init`의 별칭입니다.

이 명령어로 시작하는 생성, 테스트 및 편집 워크플로우는 [플러그인 생성](/docs/ko/plugins/create)을 참조하세요.

```bash theme={null}
claude plugin init <name> [options]
```

`<name>`은 `~/.claude/skills/` 아래의 디렉토리 이름이 되고 플러그인의 매니페스트에서 `name`이 됩니다.

명령어에는 다른 위치에 대한 플래그가 없습니다. 대신 프로젝트 내에서 스캐폴드하려면 [플러그인 생성](/docs/ko/plugins/create)을 참조하세요.

| 플래그                      | 설명                                                                                         |
| :----------------------- | :----------------------------------------------------------------------------------------- |
| `--description <text>`   | 매니페스트 설명                                                                                   |
| `--author <name>`        | 작성자 이름. 기본값은 `git config user.name`                                                        |
| `--author-email <email>` | 작성자 이메일. 기본값은 `git config user.email`                                                      |
| `--with <components...>` | `skills`, `agents`, `hooks`, `mcp`, `lsp`, `output-style` 또는 `channel`에 대한 스타터 파일도 스캐폴드합니다 |
| `-f, --force`            | 대상의 기존 `.claude-plugin/`을 덮어씁니다                                                            |

스킬 및 훅 파일 스타터를 사용하여 플러그인을 스캐폴드합니다:

```bash theme={null}
claude plugin init my-helper --with skills hooks
```

Claude Code는 작성한 내용을 검증하고 `Created plugin "my-helper" at ~/.claude/skills/my-helper`를 출력한 후 로드되는 id와 이를 끄는 `claude plugin disable` 명령어를 출력합니다.

Claude Code는 안전하게 스캐폴드할 수 없을 때 `1`로 종료하고 메시지는 이유를 명시합니다. 다음은 일반적인 이유입니다:

* 알 수 없는 `--with` 값
* `--force` 없이 대상에 기존 스캐폴드가 있음
* skills-directory 플러그인을 차단하는 관리되는 설정

<h3 id="plugin-install">
  plugin install
</h3>

추가한 마켓플레이스에서 플러그인을 설치합니다. `i`는 `install`의 별칭입니다.

```bash theme={null}
claude plugin install <plugin> [options]
```

대부분의 플러그인은 프롬프트 없이 설치됩니다. 마켓플레이스 항목이 [설치를 위해 명령어를 실행](/docs/ko/plugins/host-marketplace)하거나 [다운로드를 위해 `headersHelper`를 설정](/docs/ko/plugins/host-marketplace#how-users-accept-a-headershelper-command)하는 플러그인의 경우, Claude Code는 먼저 명령어를 출력하고 `Run this command now? [y/N]`을 묻습니다.

| 플래그                         | 설명                                                                                                                                                                                                                   |
| :-------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>`       | 설치 범위: `user`, `project` 또는 `local`. 기본값은 `user`                                                                                                                                                                     |
| `--config <key=value>`      | 플러그인의 매니페스트가 선언하는 [`userConfig`](/docs/ko/plugins/manifest-reference) 옵션을 설정합니다. 각 옵션에 대해 플래그를 반복합니다. Claude Code v2.1.147 이상 필요                                                                                          |
| `-y, --yes`                 | `Run this command now?` 프롬프트 없이 표시된 설치 명령어를 수락합니다. Bash 도구나 훅과 같이 Claude Code 세션 내에서 명령어가 실행될 때는 무시됩니다. Claude Code v2.1.229 이상 필요                                                                                   |
| `--accept-command <sha256>` | 이전 [`--json` 실행](#plugin-json-result)이 `shownCommand`에서 보고한 `sha256`을 가진 표시된 설치 명령어를 수락합니다. `-y` 대신 사용합니다. `-y`와 결합할 수 없습니다. [표시된 설치 명령어 수락](#accept-a-displayed-install-command)을 참조하세요. Claude Code v2.1.271 이상 필요 |
| `--json`                    | 스크립트에서 사용하기 위해 stdout의 마지막 줄에 하나의 JSON 객체로 결과를 출력합니다. [JSON 결과 형식](#plugin-json-result)을 참조하세요. Claude Code v2.1.268 이상 필요                                                                                           |

자신의 터미널에서 `-y`를 전달하여 프롬프트 없이 표시된 명령어를 수락합니다. TTY가 없을 때와 Claude가 명령어를 실행할 때 어떤 일이 발생하는지 다음과 같습니다:

* **stdin 또는 stdout이 TTY가 아니고 `-y` 또는 `--accept-command`를 전달하지 않음**: 설치가 거부됩니다. 출력은 명령어가 표시되었을 뿐이라고 말하고 종료 코드는 `1`입니다
* **Claude가 Bash 도구를 통해 명령어를 실행**: `-y`는 무시됩니다. 대신 자신의 터미널에서 명령어를 실행하세요

프로젝트를 복제하는 모든 사람을 위해 플러그인을 설치합니다:

```bash theme={null}
claude plugin install formatter@my-marketplace --scope project
```

Claude Code는 `Successfully installed plugin: formatter@my-marketplace (scope: project)`를 출력합니다. 새로운 것이 설치되지 않으면 출력은 이유를 설명합니다:

* **해당 범위에 이미 설치됨**: 출력은 `Plugin "formatter@my-marketplace" is already installed (scope: project)`이고 종료 코드는 `0`입니다
* **명령어 소스 프롬프트를 거부함**: 출력은 `Aborted.`이고 종료 코드는 `1`입니다
* **`headersHelper` 프롬프트를 거부하거나 TTY 없이 확인할 수 없음**: 출력은 `Aborted — the command was not run.`이고 종료 코드는 `1`입니다

<h4 id="plugin-json-result">
  JSON 결과 형식
</h4>

`plugin install`에 `--json`을 전달하면 stdout의 마지막 줄은 하나의 JSON 객체입니다. 마켓플레이스가 선언한 명령어가 앞에 출력될 수 있으므로 해당 줄만 파싱하세요.

세 가지 필드는 항상 존재합니다:

* `command`: 실행된 하위 명령어(예: `install`)
* `outcome`: `ok` 또는 `failed`
* `message`: 결과에 대한 사람이 읽을 수 있는 설명

`pluginId`, `scope` 및 `failureCode`와 같은 다른 필드는 적용될 때만 나타납니다.

`plugin uninstall`, `plugin update`, `plugin enable` 및 `plugin disable`의 `--json` 옵션은 해당 하위 명령어의 자체 필드를 포함한 동일한 객체를 출력합니다.

사용 오류(예: 잘못된 `--scope`)는 결과 줄을 출력하지 않고 stderr의 이유와 함께 `1`로 종료합니다.

<h4 id="accept-a-displayed-install-command">
  표시된 설치 명령어 수락
</h4>

`--json` 실행이 마켓플레이스에서 선언한 명령어를 표시하고 실행하지 않으면 `failed` 결과는 `shownCommand` 객체도 포함합니다. 해당 필드에는 표시된 명령어, 속한 플러그인 및 명령어의 `sha256`이 포함됩니다.

정확히 그 명령어를 수락하려면 자신의 터미널에서 해당 `sha256`을 `--accept-command`로 다시 실행하세요. 플래그는 Claude Code 세션 내에서 효과가 없기 때문입니다. Claude Code v2.1.271 이상 필요합니다.

`sha256`은 정확히 그 명령어, 플러그인 및 마켓플레이스 카탈로그에 대한 수락으로 계산됩니다. 명령어가 표시된 이후 이들 중 하나라도 변경되면 Claude Code는 `sha256`을 수락하지 않고 명령어를 다시 표시합니다. 실행 자체의 마켓플레이스 새로고침이 가져오는 변경도 그러한 변경으로 계산됩니다.

`shownCommand.acceptCommandMatched`가 `false`이면 전달한 `sha256`이 현재 표시된 명령어와 일치하지 않습니다. 해당 명령어를 검토한 후 해당 `sha256`으로 다시 실행하세요.

<h3 id="plugin-uninstall">
  plugin uninstall
</h3>

설치된 플러그인을 한 범위에서 제거합니다. `remove` 및 `rm`은 `uninstall`의 별칭입니다.

```bash theme={null}
claude plugin uninstall <plugin> [options]
```

| 플래그                   | 설명                                                                                                                                                  |
| :-------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>` | 범위에서 제거: `user`, `project` 또는 `local`. 기본값은 `user`                                                                                                  |
| `--keep-data`         | 플러그인의 지속적 데이터 디렉토리 `~/.claude/plugins/data/<id>/`를 보존합니다                                                                                            |
| `--prune`             | 남은 플러그인이 필요하지 않은 자동 설치된 [종속성](/docs/ko/plugins/dependencies)도 제거합니다                                                                                      |
| `-y, --yes`           | `--prune` 확인 프롬프트를 건너뜁니다. stdin 또는 stdout이 TTY가 아닐 때 `--prune`과 함께 필요합니다                                                                            |
| `--json`              | stdout의 마지막 줄에 하나의 JSON 객체로 결과를 출력합니다. [`plugin install --json`](#plugin-json-result)과 동일한 형식입니다. `--prune`과 결합할 수 없습니다. Claude Code v2.1.268 이상 필요 |

프로젝트 범위에서 플러그인을 제거합니다:

```bash theme={null}
claude plugin uninstall formatter@my-marketplace --scope project
```

Claude Code는 `Successfully uninstalled plugin: formatter (scope: project)`를 출력합니다. 플러그인이 해당 범위에 설치되지 않으면 명령어는 `Failed to uninstall plugin "formatter@my-marketplace":`로 시작하는 줄을 출력하고 `1`로 종료합니다.

<h3 id="plugin-enable">
  plugin enable
</h3>

비활성화된 플러그인을 활성화합니다. [claude.ai에서 동기화된 플러그인](/docs/ko/plugins/loading#synced-plugins)의 경우 `<name>@synced`를 플러그인으로 전달합니다.

```bash theme={null}
claude plugin enable <plugin> [options]
```

| 플래그                   | 설명                                                                                                                           |
| :-------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>` | 활성화할 범위: `user`, `project` 또는 `local`. 생략하면 자동 감지됩니다                                                                         |
| `--json`              | stdout의 마지막 줄에 하나의 JSON 객체로 결과를 출력합니다. [`plugin install --json`](#plugin-json-result)과 동일한 형식입니다. Claude Code v2.1.268 이상 필요 |

`--scope` 없이 명령어는 local, project, user 순서로 설정 파일을 확인하고 플러그인을 언급하는 첫 번째 범위를 사용합니다.

플러그인이 선언되지 않은 `--scope`를 전달하면 명령어는 재정의를 쓰거나 실패합니다:

* **선언하는 범위보다 [우선순위를 갖는](/docs/ko/plugins/loading) 범위**: Claude Code는 전달한 범위에서 재정의를 씁니다. 예를 들어 `claude plugin disable formatter --scope local`은 프로젝트에서 활성화된 플러그인을 당신만 끕니다
* **다른 범위**: 명령어는 `Plugin "formatter" is installed at project scope, not user. Use --scope project or omit --scope to auto-detect.`로 실패합니다

플러그인이 해결된 범위에서 이미 활성화되어 있으면 명령어는 `Plugin "formatter" is already enabled`를 출력하고 `1`로 종료합니다. `--json`을 사용하면 결과는 `"failureCode": "already_in_goal_state"`와 `"alreadyInGoalState": true`를 가지므로 스크립트는 그 경우를 성공으로 처리할 수 있습니다.

플러그인이 [종속성](/docs/ko/plugins/dependencies)을 선언하면 Claude Code는 이들도 활성화합니다. 명령어는 다음 경우에 실패합니다:

* **종속성이 설치되지 않음**: 활성화가 실패하고 누락된 각 종속성에 대해 `claude plugin install` 명령어를 출력합니다
* **종속성이 조직의 플러그인 정책에 의해 차단됨**: 활성화가 실패하고 차단된 종속성의 이름을 지정합니다
* **종속성이 대상 범위보다 높은 우선순위를 가진 범위에서 `false`로 설정됨**: 활성화가 실패합니다. 해당 범위에서 종속성을 활성화하거나 `--scope`를 전달하여 거기에 쓰세요

선언된 곳 어디든 플러그인을 다시 활성화합니다:

```bash theme={null}
claude plugin enable formatter
```

Claude Code는 `Successfully enabled plugin: formatter (scope: project)`를 출력하고 감지된 범위의 이름을 지정합니다.

<h3 id="plugin-disable">
  plugin disable
</h3>

플러그인을 제거하지 않고 비활성화합니다. [claude.ai에서 동기화된 플러그인](/docs/ko/plugins/loading#synced-plugins)의 경우 `<name>@synced`를 플러그인으로 전달합니다.

```bash theme={null}
claude plugin disable [plugin] [options]
```

| 플래그                   | 설명                                                                                                                           |
| :-------------------- | :--------------------------------------------------------------------------------------------------------------------------- |
| `-a, --all`           | 활성화된 모든 플러그인을 비활성화합니다. 플러그인 이름이나 `--scope`와 결합할 수 없습니다                                                                       |
| `-s, --scope <scope>` | 비활성화할 범위: `user`, `project` 또는 `local`. 생략하면 자동 감지됩니다                                                                        |
| `--json`              | stdout의 마지막 줄에 하나의 JSON 객체로 결과를 출력합니다. [`plugin install --json`](#plugin-json-result)과 동일한 형식입니다. Claude Code v2.1.268 이상 필요 |

`--scope` 없이 범위는 [`plugin enable`](#plugin-enable)과 동일한 local, project, user 순서로 자동 감지됩니다.

플러그인 이름이나 `--all`을 전달하지 않으면 Claude Code는 `Please specify a plugin name or use --all to disable all plugins`를 출력하고 `1`로 종료합니다. 이미 비활성화된 플러그인을 비활성화하면 `Plugin "formatter" is already disabled`를 출력하고 `1`로 종료합니다. [`plugin enable`](#plugin-enable)이 이미 활성화된 플러그인에 대해 하는 것처럼 말입니다.

명령어는 여전히 필요한 플러그인에 대해 실패합니다:

* **다른 활성화된 플러그인이 [이에 종속](/docs/ko/plugins/dependencies)됨**: 명령어는 실패하고 먼저 비활성화할 종속성의 이름을 지정합니다
* **조직이 동기화된 플러그인으로 이를 요구함**: 명령어는 실패하고 아무것도 저장하지 않습니다

한 플러그인을 비활성화합니다:

```bash theme={null}
claude plugin disable formatter
```

Claude Code는 `Successfully disabled plugin: formatter (scope: project)`를 출력합니다.

<h3 id="plugin-update">
  plugin update
</h3>

플러그인을 마켓플레이스가 제공하는 최신 버전으로 업데이트합니다. 새 버전은 다음 세션에 로드되거나 실행 중인 세션에서 `/reload-plugins`를 실행한 후 로드됩니다.

```bash theme={null}
claude plugin update <plugin> [options]
```

| 플래그                         | 설명                                                                                                                                                                         |
| :-------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-s, --scope <scope>`       | 업데이트할 범위: `user`, `project`, `local` 또는 `managed`. 기본값은 플러그인이 설치된 범위입니다                                                                                                    |
| `-y, --yes`                 | [명령어 소스](/docs/ko/plugins/host-marketplace) 플러그인에서 변경된 설치 명령어를 프롬프트 없이 수락합니다. stdin 또는 stdout이 TTY가 아닐 때 필요합니다. `--accept-command`를 전달하지 않는 한 필요합니다. Claude Code v2.1.229 이상 필요 |
| `--accept-command <sha256>` | 이전 [`--json` 실행](#plugin-json-result)이 `shownCommand`에서 보고한 `sha256`을 가진 마켓플레이스에서 선언한 명령어를 수락합니다. `-y` 대신 사용합니다. `-y`와 결합할 수 없습니다. Claude Code v2.1.271 이상 필요              |
| `--json`                    | stdout의 마지막 줄에 하나의 JSON 객체로 결과를 출력합니다. [`plugin install --json`](#plugin-json-result)과 동일한 형식입니다. Claude Code v2.1.268 이상 필요                                               |

`managed`는 업데이트할 수 있지만 설치할 수 없는 유일한 범위입니다. 관리자가 설치한 플러그인의 경우 [조직을 위한 플러그인 관리](/docs/ko/plugins/org)를 참조하세요.

플러그인을 업데이트합니다:

```bash theme={null}
claude plugin update formatter@my-marketplace
```

Claude Code는 `Checking for updates for plugin "formatter@my-marketplace"…`를 출력한 후 결과를 출력합니다. 더 새로운 것이 없으면 `formatter is already at the latest version (1.0.0).`를 출력하고 `0`으로 종료합니다.

설치된 플러그인과 일치하는 베어 플러그인 이름을 전달할 수 있습니다. 다른 마켓플레이스의 설치된 플러그인이 이름을 공유하면 명령어는 업데이트를 거부하고 실행할 정규화된 `plugin-name@marketplace-name` 명령어를 나열합니다. 베어 이름으로 업데이트하려면 Claude Code v2.1.246 이상이 필요합니다.

<h3 id="plugin-list">
  plugin list
</h3>

설치된 플러그인을 버전, 범위 및 상태와 함께 나열합니다.

```bash theme={null}
claude plugin list [options]
```

| 플래그           | 설명                                                      |
| :------------ | :------------------------------------------------------ |
| `--json`      | 목록을 JSON으로 출력합니다                                        |
| `--available` | 설치하지 않은 마켓플레이스가 제공하는 플러그인도 나열합니다. `--json` 없이는 효과가 없습니다 |

Claude Code는 각 플러그인이 로드되는 방식에 따라 사람이 읽을 수 있는 출력을 그룹화합니다:

* **`Installed plugins:`**: 마켓플레이스에서 설치한 플러그인
* **`Session-only plugins (--plugin-dir / --plugin-url):`**: 같은 명령어에서 이러한 플래그로 로드된 플러그인(예: `claude --plugin-dir ./my-plugin plugin list`)
* **`Skills-directory plugins (.claude/skills/*):`**: Claude Code가 skills 디렉토리에서 찾은 플러그인
* **`Synced from claude.ai`**: [claude.ai 계정에서 동기화된 플러그인](/docs/ko/plugins/loading#synced-plugins)

어떤 그룹에도 아무것도 없으면 Claude Code는 ``No plugins installed. Use `claude plugin install` to install a plugin.``를 출력합니다.

<h4 id="json-output">
  JSON 출력
</h4>

`--json`을 사용하면 Claude Code는 설치당 하나의 객체를 포함하는 배열을 출력합니다. 각 객체는 아래 필드를 포함합니다. `id`, `version`, `scope`, `enabled` 및 `installPath`는 항상 존재하고 다른 필드는 적용될 때만 나타납니다.

| 필드             | 유형               | 설명                                                                                                                                                                      |
| :------------- | :--------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`           | string           | 설치의 경우 `name@marketplace`, 세션 전용 플러그인의 경우 `name@inline`, skills-directory 플러그인의 경우 `name@skills-dir`, claude.ai에서 동기화된 플러그인의 경우 `name@synced`                           |
| `version`      | string           | 마켓플레이스 설치의 경우 [Claude Code가 설치 시 계산한](/docs/ko/plugins/loading#versions-and-updates) 버전입니다. 세션 전용, skills-directory 또는 동기화된 플러그인의 경우 매니페스트의 `version` 또는 선언하지 않을 때 `unknown` |
| `scope`        | string           | 설치의 경우 `user`, `project`, `local` 또는 `managed`; skills-directory 플러그인의 경우 `user` 또는 `project`; 세션 전용 플러그인의 경우 `session`; claude.ai에서 동기화된 플러그인의 경우 `synced`             |
| `enabled`      | boolean          | 병합된 설정에서 플러그인이 활성화되어 있는지 여부                                                                                                                                             |
| `installPath`  | string           | 플러그인이 로드되는 디렉토리                                                                                                                                                         |
| `installedAt`  | string           | 설치의 ISO 타임스탬프입니다. 마켓플레이스 설치만 해당                                                                                                                                         |
| `lastUpdated`  | string           | 마지막 업데이트의 ISO 타임스탬프입니다. 마켓플레이스 설치만 해당                                                                                                                                   |
| `projectPath`  | string           | 설치가 속한 프로젝트입니다. `project` 및 `local` 범위만 해당                                                                                                                              |
| `mcpServers`   | object           | 마켓플레이스에서 설치한 플러그인이 있을 때 플러그인의 MCP 서버 정의                                                                                                                                 |
| `errors`       | array of strings | 플러그인이 로드되지 않았을 때 오류를 로드합니다                                                                                                                                              |
| `notes`        | array of strings | 로드되고 작동하는 플러그인에 대한 작성 경고                                                                                                                                                |
| `errorDetails` | array of objects | 각 `errors` 항목당 하나의 객체로 진단 `type`과 플러그인, 마켓플레이스, 서버 또는 파일과 같이 참조하는 이름을 제공합니다. Claude Code v2.1.268 이상 필요                                                                 |
| `noteDetails`  | array of objects | 각 `notes` 항목에 대한 동일한 세부 객체입니다. Claude Code v2.1.268 이상 필요                                                                                                               |

`--json --available`을 사용하면 Claude Code는 배열 대신 하나의 객체를 출력합니다. 해당 `installed` 필드는 설치된 플러그인 객체의 배열을 보유하고 `available` 필드는 아래 필드를 포함하는 설치되지 않은 마켓플레이스 플러그인당 하나의 객체를 보유합니다.

| 필드                | 유형               | 설명                                                                                |
| :---------------- | :--------------- | :-------------------------------------------------------------------------------- |
| `pluginId`        | string           | `name@marketplace`                                                                |
| `name`            | string           | 마켓플레이스의 플러그인 이름                                                                   |
| `marketplaceName` | string           | 이를 제공하는 마켓플레이스                                                                    |
| `source`          | string or object | 마켓플레이스 항목의 [source](/docs/ko/plugins/marketplace-reference): 상대 경로의 경우 문자열, 그 외의 경우 객체 |
| `description`     | string           | 항목의 설명(있을 때)                                                                      |
| `version`         | string           | 항목의 버전(선언할 때)                                                                     |
| `installCount`    | number           | 설치 수(Claude Code가 플러그인에 대해 가지고 있을 때)                                              |

<h3 id="plugin-details">
  plugin details
</h3>

플러그인의 구성 요소 인벤토리 및 예상 토큰 비용을 표시합니다.

플러그인은 로드되어야 합니다: 설치되거나, skills 디렉토리에서 찾거나, 같은 명령어에서 `--plugin-dir` 또는 `--plugin-url`로 전달됩니다. `<name>`은 플러그인 `name` 또는 `name@marketplace`입니다.

```bash theme={null}
claude plugin details <name>
```

명령어는 `--help` 이외의 플래그를 사용하지 않습니다.

설치된 플러그인이 기여하는 것을 표시합니다:

```bash theme={null}
claude plugin details formatter
```

Claude Code는 플러그인의 이름, 버전, 설명 및 소스를 출력한 후 다음 섹션을 출력합니다:

* **`Component inventory`**: 플러그인의 skills, agents, hooks, MCP 서버 및 LSP 서버
* **`Projected token cost`**: 플러그인이 모든 세션에 추가하는 항상 켜진 토큰
* **`Per-component (rounded)`**: 각 skill, agent 및 명령어에 대한 항상 켜진 및 호출 시 추정치입니다. 플러그인에 없을 때 생략됩니다

두 비용 수치가 의미하는 바는 [플러그인 비용 및 사용량 측정](/docs/ko/plugins/measure)을 참조하세요.

로드되지 않은 플러그인의 경우 Claude Code는 ``Plugin "formatter" not found. Run `claude plugin list` to see installed plugins, or pass --plugin-dir <path> to load one from disk.``를 출력하고 `1`로 종료합니다.

<h3 id="plugin-prune">
  plugin prune
</h3>

설치된 플러그인이 더 이상 필요하지 않은 자동 설치된 [종속성](/docs/ko/plugins/dependencies)을 제거합니다. 명령어는 직접 설치한 플러그인을 절대 제거하지 않습니다. `autoremove`는 `prune`의 별칭입니다.

```bash theme={null}
claude plugin prune [options]
```

| 플래그                   | 설명                                                 |
| :-------------------- | :------------------------------------------------- |
| `-s, --scope <scope>` | 범위에서 정리: `user`, `project` 또는 `local`. 기본값은 `user` |
| `--dry-run`           | 제거하지 않고 제거될 항목을 나열합니다                              |
| `-y, --yes`           | 확인 프롬프트를 건너뜁니다. stdin 또는 stdout이 TTY가 아닐 때 필요합니다   |

정리가 제거할 항목을 미리 봅니다:

```bash theme={null}
claude plugin prune --dry-run
```

Claude Code는 고아 종속성을 나열하고 `(dry run — nothing removed)`로 끝냅니다. 제거할 것이 없으면 `Nothing to prune`으로 시작하는 줄을 출력합니다.

`--dry-run` 없이 명령어는 프롬프트에서 확인하거나 `-y`를 전달한 후에만 고아 종속성을 제거합니다.

프롬프트에서 어떻게 답하든 종료 코드는 `0`입니다.

`prune`이 하는 일은 터미널이 연결되어 있는지와 `-y`를 전달하는지에 따라 다릅니다:

| 터미널 및 플래그                        | 어떤 일이 발생하는지                                                                           |
| :------------------------------- | :------------------------------------------------------------------------------------ |
| 대화형 터미널, `-y` 없음                 | 고아 종속성을 나열하고 `Remove? [y/N]`을 묻습니다                                                    |
| 모든 터미널, `-y`                     | 제거하고 `Removed N auto-installed plugins: <names>`를 출력합니다                               |
| TTY가 아닌 stdin 또는 stdout, `-y` 없음 | 목록을 출력하고 ``Not a TTY — run `claude plugin prune -y` to remove.``를 출력하고 아무것도 제거하지 않습니다 |

<h3 id="plugin-eval">
  plugin eval
</h3>

플러그인의 [eval 케이스](/docs/ko/plugin-evals)를 실행하고 점수가 매겨진 결과를 보고합니다. Claude Code v2.1.269 이상 필요합니다.

각 케이스는 프롬프트와 채점자입니다. Claude Code는 대상 플러그인만 로드된 격리된 세션에서 여러 번 실행하고 기본적으로 플러그인 없이도 실행하므로 보고서는 차이를 보여줍니다.

케이스 형식, 채점자, 결과 및 CI 사용법은 [evals로 플러그인 테스트](/docs/ko/plugin-evals)를 참조하세요.

```bash theme={null}
claude plugin eval [target] [options]
```

선택적 `target`은 기본값이 현재 디렉토리이고 다음 형식 중 하나를 사용합니다:

* 플러그인 디렉토리
* 단일 `prompt.md` 또는 `case.yaml` 파일
* `name` 또는 `name@marketplace`로 설치된 플러그인
* `name@skills-dir`

`--tag`, `--allow-tools` 및 `--json` 앞에 대상을 배치합니다. 이러한 각 옵션은 뒤따르는 단어를 값으로 사용하므로 이들 중 하나 뒤에 작성된 대상은 태그, 도구 이름 또는 대상 대신 JSON 출력 경로로 읽힙니다.

이 표는 대부분의 실행이 사용하는 옵션을 나열합니다. `--case`, `--tag`, `--output-dir`, `--report`, `--allow-real-servers`, `--keep-temp` 및 `--verbose`를 포함한 전체 집합에 대해 `claude plugin eval --help`를 실행하세요.

| 옵션                         | 설명                                                                                                                                       | 기본값                                                                           |
| :------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------- |
| `--runs <n>`               | 각 [arm](/docs/ko/plugin-evals#compare-against-a-no-plugin-baseline)의 케이스당 실행                                                                  | 각 케이스의 `runs`, 그렇지 않으면 3                                                      |
| `-j, --concurrency <n>`    | 한 번에 실행할 에이전트 세션, 1\~8. 속도 제한을 공유합니다                                                                                                     | `1`                                                                           |
| `--model <model>`          | 테스트 중인 에이전트의 모델                                                                                                                          | 각 케이스의 `model`, 그렇지 않으면 `ANTHROPIC_MODEL`이 설정되어 있으면, 그렇지 않으면 Claude Code의 기본값 |
| `--judge-model <model>`    | `llm` 및 `baseline` 채점자의 모델                                                                                                               | 작은 빠른 모델                                                                      |
| `--ablation <mode>`        | `none` 또는 `with-without`. [플러그인 없는 기준선과 비교](/docs/ko/plugin-evals#compare-against-a-no-plugin-baseline)를 참조하세요                                | 플러그인이 해결될 때 `with-without`, 그렇지 않으면 `none`                                    |
| `--threshold <0..1>`       | 케이스가 이 아래로 점수를 받으면 1로 종료                                                                                                                 | `1.0`                                                                         |
| `--max-cost-usd <usd>`     | 지출이 이에 도달하면 다음 실행 전에 중지하고 2로 종료하고 부분 결과를 보고합니다                                                                                           | 제한 없음                                                                         |
| `--allow-tools <tools...>` | `Bash`, `Write`, `Edit` 또는 `"mcp__plugin_<plugin>_<server>__*"`와 같은 읽기 전용 집합 이상의 도구를 부여합니다. [도구 부여](/docs/ko/plugin-evals#grant-tools)를 참조하세요 |                                                                               |
| `--scaffold`               | 각 케이스의 [`scaffold_script`](/docs/ko/plugin-evals#add-setup-or-history-with-case-yaml)를 실행합니다                                                  | 꺼짐                                                                            |
| `--trust-plugin`           | 첫 실행 신뢰 프롬프트를 건너뜁니다(CI용). [실행이 액세스할 수 있는 것](/docs/ko/plugin-evals#security)을 참조하세요                                                            | 꺼짐                                                                            |
| `--mocks <mode>`           | `record` 또는 `off`. [Mock MCP 서버](/docs/ko/plugin-evals#mock-mcp-servers)를 참조하세요                                                               | `record`                                                                      |
| `--eval-dir <dir>`         | 케이스를 보유하는 플러그인 아래의 디렉토리                                                                                                                  | 매니페스트의 `experimental.evals`, 그렇지 않으면 `evals`                                  |
| `--json [path]`            | [결과 문서](/docs/ko/plugin-evals#json-result)를 stdout으로 출력하거나 `.json` 경로에 씁니다                                                                    |                                                                               |
| `--no-publish`             | HTML 보고서를 로컬로 유지합니다                                                                                                                      |                                                                               |

종료 코드는 실행이 어떻게 끝났는지 보고합니다. 파이프라인에서 이에 대해 조치하려면 [CI에서 evals 실행](/docs/ko/plugin-evals#run-evals-in-ci)을 참조하세요.

| 종료 코드 | 의미                                   |
| :---- | :----------------------------------- |
| `0`   | 모든 케이스가 임계값을 충족합니다                   |
| `1`   | 실패한 케이스, 로드 오류 또는 신뢰할 수 없는 플러그인 디렉토리 |
| `2`   | 부분 실행                                |
| `130` | 중단됨                                  |
| `143` | 종료됨                                  |

<h3 id="plugin-eval-init">
  plugin eval init
</h3>

현재 디렉토리의 플러그인에 대한 eval 스위트를 만듭니다. Claude Code v2.1.269 이상 필요합니다. [첫 번째 eval 스위트 생성](/docs/ko/plugin-evals#create-your-first-eval-suite)을 참조하세요.

```bash theme={null}
claude plugin eval init [name] [options]
```

터미널에서 명령어는 작성 인터뷰를 위해 대화형 Claude Code 세션을 엽니다. 인터뷰에서 Claude는 다음을 수행합니다:

1. 플러그인을 읽습니다
2. 잘 해야 할 일을 묻습니다
3. 케이스와 채점자를 제안합니다
4. 케이스 파일을 씁니다
5. 케이스를 실행하고 채점자가 당신이 하는 방식으로 점수를 매기는지 확인하기 위해 당신과 함께 등급을 검토합니다

`--bare`를 사용하거나 터미널이 없으면 명령어는 대신 빈 단일 케이스 템플릿을 씁니다. Claude가 Claude Code 세션 내에서 명령어를 실행하면 명령어는 해당 세션이 따를 인터뷰 지침을 출력합니다.

선택적 `name`은 케이스 이름입니다. `--bare`를 사용하거나 터미널이 없을 때 필요합니다. 명령어가 해당 케이스에 대한 빈 템플릿을 쓰기 때문입니다. 인터뷰는 하나가 필요하지 않습니다.

명령어는 다음 옵션을 수락합니다:

| 옵션                  | 설명                                                                   | 기본값                                          |
| :------------------ | :------------------------------------------------------------------- | :------------------------------------------- |
| `--bare`            | 인터뷰를 실행하는 대신 `<name>`에 대한 빈 `prompt.md` 및 `graders/criteria.md`를 씁니다 |                                              |
| `-i, --interactive` | 인터뷰를 요구합니다. 템플릿을 쓰는 대신 터미널이 없으면 실패합니다                                |                                              |
| `--eval-dir <dir>`  | 케이스를 쓸 현재 디렉토리 아래의 디렉토리                                              | 매니페스트의 `experimental.evals`, 그렇지 않으면 `evals` |

<h3 id="plugin-tag">
  plugin tag
</h3>

플러그인 릴리스에 대해 `<name>--v<version>`이라는 주석이 달린 git 태그를 만듭니다. 태그 지정 전에 명령어는 플러그인의 `plugin.json`과 이를 나열하는 마켓플레이스 항목이 버전에 동의하는지 확인합니다.

릴리스를 태그할 때는 [플러그인 게시](/docs/ko/plugins/publish)를 참조하세요.

```bash theme={null}
claude plugin tag [path] [options]
```

`[path]`는 플러그인 디렉토리이며 기본값은 현재 디렉토리입니다. 명령어는 플러그인을 나열하는 `.claude-plugin/marketplace.json`에 대해 해당 디렉토리에서 위로 걸어가서 마켓플레이스 항목을 찾습니다.

| 플래그                   | 설명                                                     |
| :-------------------- | :----------------------------------------------------- |
| `--push`              | 태그를 생성한 후 `--remote`로 푸시합니다                            |
| `--dry-run`           | 태그를 생성하지 않고 태그 지정될 항목을 출력합니다                           |
| `-f, --force`         | 더티 작업 트리 및 태그 이미 존재 확인을 건너뜁니다                          |
| `-m, --message <msg>` | 태그 주석 메시지입니다. `%s`는 버전을 나타냅니다. 기본값은 `<name> <version>` |
| `--remote <name>`     | `--push`로 푸시할 원격입니다. 기본값은 `origin`                     |

마켓플레이스 체크아웃의 플러그인에 대한 태그를 미리 봅니다:

```bash theme={null}
claude plugin tag plugins/formatter --dry-run
```

Claude Code는 계획을 출력합니다:

* 플러그인 이름
* 버전 및 어느 파일에서 왔는지
* 일치하는 마켓플레이스 항목(있을 때)
* 태그 이름
* 실행할 `git tag` 및 `git push` 명령어

`--dry-run` 없이 Claude Code는 `Created tag formatter--v1.0.0`을 출력하고 `Pushed to origin` 또는 직접 실행할 푸시 명령어를 출력합니다. 푸시가 실패하면 태그는 여전히 로컬로 생성되고 명령어는 오류로 종료됩니다.

명령어는 안전하게 태그 지정할 수 없을 때 `1`로 종료하고 이유를 출력합니다. 일반적인 이유는:

* `plugin.json` 또는 마켓플레이스 항목에 `version` 없음
* 태그가 이미 존재함
* 작업 트리가 더티함

<h3 id="plugin-validate">
  plugin validate
</h3>

플러그인 매니페스트, 마켓플레이스 매니페스트 또는 디렉토리의 skills, agents 및 명령어를 검증하고 CI 작업이 조치할 수 있는 코드로 종료합니다. 생성, 테스트 및 편집 워크플로우는 [플러그인 생성](/docs/ko/plugins/create)을 참조하세요. 검증자가 각 매니페스트에서 확인하는 내용은 [플러그인 매니페스트 참조](/docs/ko/plugins/manifest-reference) 및 [마켓플레이스 참조](/docs/ko/plugins/marketplace-reference)를 참조하세요.

```bash theme={null}
claude plugin validate <path> [options]
```

| 플래그        | 설명                                                                                       |
| :--------- | :--------------------------------------------------------------------------------------- |
| `--strict` | 경고를 오류로 취급하므로 런타임이 허용하는 인식되지 않은 필드 및 누락된 메타데이터가 실행을 실패하게 합니다. Claude Code v2.1.145 이상 필요 |
| `--json`   | 검증 보고서를 동일한 종료 코드를 포함하는 하나의 JSON 객체로 출력합니다. Claude Code v2.1.259 이상 필요                   |

커밋하기 전에 플러그인을 검증합니다:

```bash theme={null}
claude plugin validate ./my-plugin --strict
```

<h4 id="validate-a-directory">
  디렉토리 검증
</h4>

`<path>`는 매니페스트 파일 또는 디렉토리입니다. 디렉토리가 주어지면 Claude Code는 거기서 찾은 것으로 검증할 항목을 선택합니다:

* `.claude-plugin/marketplace.json`(존재할 때)
* 그렇지 않으면 `.claude-plugin/plugin.json`
* 그렇지 않으면 구성 요소 파일(디렉토리 이름으로 선택). 매니페스트 없이 구성 요소 파일을 검증하려면 Claude Code v2.1.233 이상이 필요합니다:
  * `skills`, `agents` 또는 `commands`라는 디렉토리: 그 안의 파일
  * `.claude`라는 디렉토리: 그 안의 `skills`, `agents` 및 `commands` 디렉토리
  * 다른 디렉토리: 해당 `.claude` 아래의 세 디렉토리

Claude Code는 이름을 지정한 디렉토리 내의 심볼릭 링크를 따르지 않습니다. 링크가 있는 위치에 따라 어떤 일이 발생하는지:

* **플러그인 또는 `.claude` 루트 아래의 연결된 `skills`, `agents` 또는 `commands` 디렉토리**: Claude Code는 그 안의 아무것도 읽지 않았다고 경고합니다.
* **`skills`, `agents` 또는 `commands` 디렉토리 내의 연결된 항목**: Claude Code는 이를 건너뛰고 경고하며 디렉토리당 세션이 로드할 건너뛴 항목 수를 경고합니다.
* **이름을 지정한 `skills`, `agents` 또는 `commands` 디렉토리 자체가 심볼릭 링크이거나 해당 부모 `.claude` 디렉토리가 심볼릭 링크**: Claude Code는 오류를 보고하고 그 안의 아무것도 확인하지 않습니다. 대신 실제 디렉토리의 이름을 지정하세요.

몇 가지 파일은 검증 실행으로 읽지 않습니다:

* **플러그인 루트의 `SKILL.md`**: 플러그인 디렉토리에 대해 `claude plugin validate`를 실행하면 Claude Code는 플러그인 루트의 `SKILL.md`를 확인하지 않습니다
* **플러그인 루트의 `CLAUDE.md`**: 플러그인 실행에서 Claude Code는 플러그인 루트의 `CLAUDE.md`에 대해서도 경고합니다
* **마켓플레이스 실행의 플러그인 파일**: 마켓플레이스 디렉토리에서 Claude Code는 플러그인의 skill, agent, command 또는 hook 파일을 열지 않습니다. 이러한 파일의 오류를 찾으려면 각 플러그인 디렉토리를 검증하세요

<h4 id="output-and-exit-codes">
  출력 및 종료 코드
</h4>

Claude Code는 검증한 파일, 경로가 있는 오류 및 경고, 그리고 판정 줄을 출력합니다. 종료 코드는 판정을 따릅니다:

| 종료 코드 | 판정 줄                                                                            | 의미                                      |
| :---- | :------------------------------------------------------------------------------ | :-------------------------------------- |
| `0`   | `Validation passed` 또는 `Validation passed with warnings`                        | 매니페스트가 로드됩니다. `--strict`를 사용하면 경고도 없습니다 |
| `1`   | `Validation failed` 또는 `Validation failed (--strict treats warnings as errors)` | 오류 또는 `--strict` 아래의 경고                 |
| `2`   | `Unexpected error during validation: <reason>`                                  | 검증자 자체가 실패했습니다(예: 읽을 수 없는 경로)           |

`--json`을 사용하면 Claude Code는 보고서를 stdout에 다음 최상위 필드를 포함하는 하나의 JSON 객체로 씁니다:

* `success`: 종료 코드가 제공하는 동일한 판정
* `strict`: 실행이 경고를 오류로 취급했는지 여부
* `target`: Claude Code가 검증한 해결된 경로
* `manifest`: 매니페스트의 자체 결과 또는 매니페스트 없는 실행의 경우 `null`
* `contents`: 파일당 결과로 `file`의 이름을 지정하고 `errors`, `warnings` 및 `notes` 배열을 포함합니다

종료 `2`에서 명령어는 stdout에 아무것도 쓰지 않습니다. 오류 메시지는 stderr로 이동합니다.

<h2 id="claude-plugin-marketplace-commands">
  claude plugin marketplace 명령어
</h2>

셸에서 `claude plugin marketplace <subcommand>`를 실행하여 플러그인을 설치하는 마켓플레이스를 추가, 나열, 새로고침 및 제거합니다.

* **종료 코드**: 이러한 하위 명령어는 플러그인 명령어의 [종료 코드 규칙](#claude-plugin-commands)을 따릅니다
* **범위**: 해당 `--scope` 플래그에는 `-s` 짧은 형식이 없습니다

마켓플레이스가 무엇이고 Claude Code가 이를 캐시하는 방법은 [플러그인 로딩 참조](/docs/ko/plugins/loading)를 참조하세요.

<h3 id="plugin-marketplace-add">
  plugin marketplace add
</h3>

GitHub 저장소, git URL, 호스팅된 `marketplace.json` 또는 로컬 경로에서 마켓플레이스를 추가하고 설정 파일에 선언합니다.

추가한 후 Claude Code는 설치된 플러그인이 누락된 [종속성](/docs/ko/plugins/dependencies)을 설치합니다.

```bash theme={null}
claude plugin marketplace add <source> [options]
```

| 플래그                   | 설명                                                                                                                  |
| :-------------------- | :------------------------------------------------------------------------------------------------------------------ |
| `--scope <scope>`     | 마켓플레이스를 선언할 설정 파일: `user`, `project` 또는 `local`. 기본값은 `user`                                                        |
| `--sparse <paths...>` | monorepos의 경우 git 체크아웃을 이러한 디렉토리로 제한합니다. `github` 및 `git` 소스만 해당                                                    |
| `--claudeai`          | 인수를 소스 대신 [claude.ai에서 호스팅되는 마켓플레이스](/docs/ko/plugins/install#add-from-claude-ai)의 이름으로 읽습니다. Claude Code v2.1.273 이상 필요 |

`<source>`는 아래 표의 형식 중 하나를 사용하고 해당 형식은 소스 유형과 Claude Code가 마켓플레이스를 가져오는 방식을 결정합니다. 결과 소스 객체는 [마켓플레이스 참조](/docs/ko/plugins/marketplace-reference)를 참조하세요.

| 입력                                                                        | 소스 유형       | Claude Code가 가져오는 방식                                                  |
| :------------------------------------------------------------------------ | :---------- | :-------------------------------------------------------------------- |
| `owner/repo`, `owner/repo#ref` 또는 `owner/repo@ref`                        | `github`    | GitHub 저장소를 복제하고 주어진 경우 `ref`로 고정합니다. 소유자와 저장소는 GitHub 명명 규칙을 따라야 합니다 |
| `user@host:path[.git][#ref]`                                              | `git`       | SSH를 통해 복제합니다                                                         |
| `https://example.com/repo.git[#ref]` 또는 `/_git/`를 포함하는 URL                | `git`       | Azure DevOps URL을 포함하여 HTTPS를 통해 복제합니다                                |
| `https://github.com/owner/repo` 또는 `https://gitlab.com/namespace/project` | `git`       | `.git`을 추가한 후 HTTPS를 통해 복제합니다                                         |
| `.git`이 없는 자체 호스팅 git 호스트를 포함한 다른 `http://` 또는 `https://` URL             | `url`       | URL을 `marketplace.json`으로 가져옵니다. 대신 저장소를 복제하려면 `.git`을 추가하세요          |
| `./path`, `../path`, `/path` 또는 `~/path`에서 디렉토리로                          | `directory` | 디렉토리를 제자리에서 읽습니다. Windows에서 `.\`, `..\` 및 `C:\` 형식도 작동합니다             |
| 동일한 경로 형식(`.json` 파일로)                                                    | `file`      | 파일을 제자리에서 읽습니다                                                        |

복제 URL이 `.git` 접미사를 포함하지 않는 호스트(예: AWS CodeCommit)의 경우 마켓플레이스를 [`extraKnownMarketplaces`](/docs/ko/settings-reference#extraknownmarketplaces)의 git 항목으로 추가하세요. Claude Code는 URL이 `.git`으로 끝나는지 여부에 관계없이 git 항목을 복제합니다.

Claude Code는 `https://gitlab.com/group/subgroup/project`와 같은 중첩된 하위 그룹이 있는 `gitlab.com` URL도 복제합니다.

마켓플레이스를 추가하고 프로젝트와 공유합니다:

```bash theme={null}
claude plugin marketplace add your-org/your-marketplace --scope project
```

Claude Code는 `Successfully added marketplace: your-marketplace (declared in project settings)`를 출력하고 마켓플레이스의 자체 매니페스트에서 `name`을 사용합니다. 반복 추가 또는 잘못된 소스는 대신 다음 결과 중 하나를 출력합니다:

* **마켓플레이스가 이미 디스크에 있음**: 출력은 `Marketplace 'your-marketplace' already on disk — declared in project settings`이고 종료 코드는 `0`입니다
* **인식되지 않은 소스**: 출력은 `Invalid marketplace source format. Try: owner/repo, https://..., or ./path`이고 종료 코드는 `1`입니다
* **`gitlab.example.com/team/plugins`와 같은 베어 호스트**: 추가는 잘못된 `owner/repo` 약자로 실패하고 메시지는 `https://`를 추가하거나 로컬 경로를 사용하도록 알려줍니다

[claude.ai에서 호스팅되는 마켓플레이스](/docs/ko/plugins/install#add-from-claude-ai)를 `claude plugin marketplace list`의 `From claude.ai:` 섹션에 출력된 이름으로 추가합니다:

```bash theme={null}
claude plugin marketplace add --claudeai claudeai-organization-library
```

`--claudeai`를 사용하면 명령어는 `--scope` 및 `--sparse`를 거부합니다. 마켓플레이스는 계정에 대해 호스팅되고 설정 파일에 선언되지 않으므로 프로젝트의 `.claude/settings.json`을 통해 공유할 수 없습니다.

<h3 id="plugin-marketplace-list">
  plugin marketplace list
</h3>

추가한 모든 마켓플레이스를 소스와 함께 나열합니다.

```bash theme={null}
claude plugin marketplace list [options]
```

| 플래그      | 설명               |
| :------- | :--------------- |
| `--json` | 목록을 JSON으로 출력합니다 |

Claude Code는 `Configured marketplaces:`를 출력하고 마켓플레이스당 하나의 `Source:` 줄을 출력하거나 `No marketplaces configured`를 출력합니다.

`--json`을 사용하면 Claude Code는 마켓플레이스당 하나의 객체를 포함하는 배열을 출력하고 아래 필드를 포함합니다. 모든 필드는 문자열입니다.

| 필드                | 설명                                                        |
| :---------------- | :-------------------------------------------------------- |
| `name`            | 마켓플레이스의 이름                                                |
| `source`          | `github`, `git`, `url`, `directory`, `file` 또는 `claudeai` |
| `repo`            | `owner/repo`. `github` 소스만 해당                             |
| `url`             | 복제 또는 가져오기 URL입니다. `git` 및 `url` 소스만 해당                   |
| `path`            | 로컬 경로입니다. `directory` 및 `file` 소스만 해당                     |
| `ref`             | 고정된 분기 또는 태그입니다. `github` 및 `git` 소스, 고정된 경우만 해당          |
| `installLocation` | Claude Code가 마켓플레이스를 캐시한 위치                               |

추가된 [claude.ai 마켓플레이스](/docs/ko/plugins/install#add-from-claude-ai)에는 로컬 복제본이 없으므로 해당 항목은 `installLocation` 대신 claude.ai 식별자 `marketplaceId` 및 `organizationUuid`를 포함합니다. 또한 기록된 경우 `scope`와 `status`를 포함합니다.

터미널 세션이 [claude.ai 계정에서 플러그인을 동기화](/docs/ko/plugins/loading#synced-plugins)하면 텍스트 목록은 `From claude.ai:` 섹션으로 끝납니다. 해당 섹션은 claude.ai가 추가하지 않은 계정에 대해 나열하는 마켓플레이스의 이름을 지정합니다(git 기반 및 호스팅). Claude Code v2.1.273 이상 필요합니다.

해당 섹션에서 마켓플레이스를 추가하려면 [claude.ai에서 마켓플레이스 추가](/docs/ko/plugins/install#add-from-claude-ai)를 참조하세요.

`--json` 출력은 구성된 마켓플레이스만 다루고 섹션을 제외합니다.

<h3 id="plugin-marketplace-remove">
  plugin marketplace remove
</h3>

설정에서 마켓플레이스의 선언을 제거합니다. `rm`은 `remove`의 별칭입니다.

<Warning>
  마켓플레이스를 선언하는 마지막 범위에서 제거하면 Claude Code는 캐시도 삭제하고 설치한 모든 플러그인을 제거합니다. `--scope` 없이 명령어는 모든 범위에서 선언을 제거합니다. 플러그인을 잃지 않고 마켓플레이스를 새로고침하려면 `plugin marketplace update`를 대신 실행하세요.
</Warning>

```bash theme={null}
claude plugin marketplace remove <name> [options]
```

`<name>`은 전달한 소스가 아니라 `plugin marketplace list`가 표시하는 마켓플레이스 이름입니다.

| 플래그               | 설명                                                                                |
| :---------------- | :-------------------------------------------------------------------------------- |
| `--scope <scope>` | 한 설정 범위에서 선언을 제거합니다: `user`, `project` 또는 `local`. 없으면 Claude Code는 모든 범위에서 제거합니다 |

모든 범위에서 마켓플레이스를 제거합니다:

```bash theme={null}
claude plugin marketplace remove your-marketplace
```

Claude Code는 `Successfully removed marketplace: your-marketplace`를 출력하고 범위를 지정할 때 `(from project settings)`를 추가합니다. 마켓플레이스를 선언하지 않는 설정 파일로 범위를 지정하면 명령어는 `Marketplace 'your-marketplace' is not declared in project settings. Omit --scope to remove it from all scopes.`로 실패합니다.

<h3 id="plugin-marketplace-update">
  plugin marketplace update
</h3>

한 마켓플레이스 또는 모든 마켓플레이스를 소스에서 새로고침하여 새 플러그인 및 버전을 가져옵니다. 분기 또는 태그 `ref`로 추가된 마켓플레이스는 저장소의 기본 분기가 아니라 해당 ref의 최신 커밋으로 업데이트됩니다.

```bash theme={null}
claude plugin marketplace update [name]
```

명령어는 `--help` 이외의 플래그를 사용하지 않습니다.

한 마켓플레이스를 새로고침합니다:

```bash theme={null}
claude plugin marketplace update your-marketplace
```

Claude Code는 `Successfully updated marketplace: your-marketplace`를 출력합니다. 이름을 생략하면 `Successfully updated 2 marketplaces`와 같은 수를 출력합니다. 추가된 마켓플레이스가 없으면 `No marketplaces configured`를 출력하고 `0`으로 종료합니다.

<h2 id="plugin-in-a-session">
  세션의 /plugin
</h2>

대화형 세션 내에서 `/plugin`은 플러그인 패널을 엽니다. 각 하위 명령어는 패널을 탭에서 열고 거기서 작업을 실행하거나 결과를 인라인으로 출력합니다. `/plugins` 및 `/marketplace`는 `/plugin`의 별칭입니다.

이러한 명령어는 대화형 터미널 세션에서만 실행할 수 있습니다. `claude -p`와 같은 비대화형 실행에서 Claude Code는 `/plugin`이 이 환경에서 사용 가능하지 않다고 회신합니다.

어떤 표면에 `/plugin`이 있는지, 이를 설치하지 않고 설치하는 방법, 각 패널 탭이 표시하는 것은 [플러그인 설치 및 관리](/docs/ko/plugins/install)를 참조하세요.

`<plugin>`은 플러그인 `name` 또는 `name@marketplace`입니다.

아래 표는 모든 세션 형식을 나열합니다. 셸 하위 명령어 `init`, `update`, `details`, `prune`, `eval` 및 `eval init`에는 세션 형식이 없습니다.

| 명령어                                                 | 별칭                                             | 어떤 일을 하는지                                                                                                                                                                                           |
| :-------------------------------------------------- | :--------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/plugin`                                           |                                                | **Discover** 탭에서 패널을 엽니다. `/plugin` 뒤의 인식되지 않은 첫 단어는 동일하게 합니다                                                                                                                                       |
| `/plugin help`                                      | `/plugin --help`, `/plugin -h`                 | `/plugin` 하위 명령어의 사용 목록을 표시합니다                                                                                                                                                                      |
| `/plugin list [--enabled\|--disabled]`              | `ls`                                           | 마켓플레이스에서 설치한 플러그인을 버전, 범위 및 상태와 함께 인라인으로 출력합니다. 필터 플래그는 해당 상태만 표시합니다. 활성화 상태가 아직 적용되지 않은 플러그인은 `— run /reload-plugins to apply`로 표시됩니다. Claude Code v2.1.163 이상 필요                                  |
| `/plugin install`                                   | `i`                                            | **Discover** 탭을 엽니다                                                                                                                                                                                 |
| `/plugin install <plugin>`                          | `i`                                            | **Discover** 탭에서 플러그인의 세부 정보를 엽니다. `name@marketplace`를 사용하면 해당 마켓플레이스의 목록에서 엽니다                                                                                                                     |
| `/plugin install <plugin> --marketplace <source>`   | `i`                                            | 아직 추가하지 않은 경우 `<source>`에서 마켓플레이스를 추가하고 먼저 확인을 요청한 후 플러그인의 세부 정보를 엽니다. [마켓플레이스 추가 및 한 명령어로 설치](/docs/ko/plugins/install#add-a-marketplace-and-install-in-one-command)를 참조하세요. Claude Code v2.1.275 이상 필요 |
| `/plugin manage`                                    |                                                | **Installed** 탭을 엽니다                                                                                                                                                                                |
| `/plugin stats`                                     |                                                | [`/skill-doctor`](/docs/ko/skills#find-unused-skills)를 사용할 수 있는 세션에서 **Stats** 탭을 엽니다. 다른 곳에서는 **Discover** 탭에서 패널을 엽니다                                                                                  |
| `/plugin enable <plugin>`                           |                                                | **Installed** 탭에서 플러그인을 열고 활성화합니다                                                                                                                                                                   |
| `/plugin disable <plugin>`                          |                                                | **Installed** 탭에서 플러그인을 열고 비활성화합니다                                                                                                                                                                  |
| `/plugin uninstall <plugin>`                        |                                                | **Installed** 탭에서 플러그인을 열고 제거합니다                                                                                                                                                                    |
| `/plugin configure <plugin>`                        | `config`                                       | 플러그인의 [`userConfig`](/docs/ko/plugins/manifest-reference) 대화 상자를 열거나 플러그인이 선언하지 않음을 보고합니다. Claude Code v2.1.147 이상 필요                                                                                    |
| `/plugin validate <path>`                           |                                                | `claude plugin validate`와 동일한 보고서를 인라인으로 출력합니다                                                                                                                                                      |
| `/plugin tag [path] [--push] [--dry-run] [--force]` |                                                | `claude plugin tag`가 하는 것처럼 릴리스 태그를 만듭니다. `--push`, `--dry-run` 및 `--force` 또는 `-f`를 수락합니다. 다른 플래그나 추가 인수를 사용하면 Claude Code는 대신 사용법을 출력합니다                                                          |
| `/plugin marketplace`                               | `market`                                       | 보이는 것이 없습니다. `add`, `list`, `update` 또는 `remove`를 전달합니다                                                                                                                                             |
| `/plugin marketplace add [source]`                  | `market add`                                   | 소스를 사용하면 추가하고 결과를 보고합니다. 없으면 **Add marketplace** 입력을 엽니다                                                                                                                                            |
| `/plugin marketplace list`                          | `market list`                                  | 마켓플레이스 이름을 인라인으로 출력합니다                                                                                                                                                                              |
| `/plugin marketplace update [name]`                 | `market update`                                | **Marketplaces** 탭을 엽니다. 이름을 사용하면 거기서 해당 마켓플레이스를 새로고침합니다                                                                                                                                            |
| `/plugin marketplace remove [name]`                 | `market remove`, `market rm`, `marketplace rm` | **Marketplaces** 탭을 엽니다. 이름을 사용하면 거기서 해당 마켓플레이스를 제거합니다                                                                                                                                              |

`/plugin enable`, `disable`, `uninstall` 또는 `configure`에서 현재 프로젝트에 설치되지 않은 플러그인의 이름을 지정하면 Claude Code는 조치하는 대신 `Plugin "<plugin>" is not installed in this project`를 출력합니다.

<h2 id="reload-plugins">
  /reload-plugins
</h2>

실행 중인 세션을 다시 시작하지 않고 보류 중인 플러그인 변경 사항을 적용합니다. 보류 중인 변경 사항은 세션이 시작된 이후 설치, 업데이트, 활성화, 비활성화 또는 디스크에서 편집한 플러그인입니다.

보류 중인 변경 사항으로 `/plugin` 패널을 닫으면 Claude Code는 자동으로 `/reload-plugins`를 실행합니다. 다른 터미널에서 실행한 `claude plugin` 명령어와 같이 패널 외부에서 발생하는 플러그인 변경 사항 후에 직접 실행하세요.

```text theme={null}
/reload-plugins [--force]
```

| 플래그       | 설명                                                   |
| :-------- | :--------------------------------------------------- |
| `--force` | 프롬프트 캐시를 무효화할 때에도 다시 로드를 적용합니다. 대시 없이 `force`도 작동합니다 |

<h3 id="reload-summary">
  다시 로드 요약
</h3>

Claude Code는 모든 활성 플러그인을 다시 로드하고 하나의 요약 줄 `Reloaded: N plugins · N skills · N agents · N hooks · N plugin MCP servers · N plugin LSP servers`를 출력하고 대화형 터미널이 없는 세션에서 플러그인 MCP 서버 수를 생략합니다. 플러그인이 실패하면 요약은 `N errors during load. Run /plugin for details.`를 추가합니다.

skills 수는 플러그인이 제공하는 모든 skill을 포함합니다. 즉, `commands/` 항목과 `SKILL.md` skills 모두입니다. agents 수는 플러그인에서 오지 않은 것을 포함하여 세션에 로드된 agents의 수입니다.

다시 로드된 플러그인의 [종속성](/docs/ko/plugins/dependencies)이 누락되면 Claude Code는 이들을 설치하고 다시 로드하고 요약에 `(+ N dependencies: <names>) resolved`를 추가합니다.

<h3 id="reloads-that-change-mcp-tools">
  MCP 도구를 변경하는 다시 로드
</h3>

다시 로드가 플러그인 MCP 서버 또는 `LSP` 도구를 추가하거나 제거할 때 그 변경이 [프롬프트 캐시](/docs/ko/prompt-caching#enabling-or-disabling-a-plugin)를 무효화할 것이면 Claude Code는 다시 로드를 적용하지 않습니다. `This reload changes MCP tools (<server>) — your next message will re-read the whole conversation instead of using the cache. Run /reload-plugins --force to apply.`와 같은 줄을 출력합니다. `--force`를 전달하여 어쨌든 적용하세요.

<h3 id="sessions-without-an-interactive-terminal">
  대화형 터미널이 없는 세션
</h3>

`/reload-plugins`는 데스크톱 앱, Agent SDK 및 [`-p`를 사용한 비대화형 모드](/docs/ko/headless)와 같이 대화형 터미널이 없는 세션에서도 실행됩니다. Claude Code v2.1.260 이상 필요합니다.

이러한 세션에서 명령어는 `-p` 프롬프트 또는 데스크톱 앱의 프롬프트 상자와 같이 세션에 직접 입력할 때만 실행됩니다. [Remote Control](/docs/ko/remote-control) 또는 Slack에서 중계된 메시지와 같이 다른 방식으로 도착하면 명령어는 `/reload-plugins isn't available over a remote connection in this session.`을 회신하고 아무것도 다시 로드하지 않습니다.

이러한 세션의 다시 로드는 플러그인 MCP 서버를 연결하거나 연결 해제하지 않습니다. 이러한 변경 사항은 다음 세션에서 적용됩니다.

<h2 id="flags-that-load-a-plugin-for-one-session">
  한 세션 동안 플러그인을 로드하는 플래그
</h2>

두 `claude` 플래그는 설치하지 않고 한 세션 동안만 플러그인을 로드합니다. 둘 다 반복 가능합니다.

플러그인 작성자는 이들을 사용하여 게시하기 전에 플러그인을 테스트합니다. 로드-편집-다시 로드 워크플로우는 [마켓플레이스 없이 개발](/docs/ko/plugins/create#develop-without-a-marketplace)을 참조하세요.

| 플래그                   | 설명                                                                                                                          | 예                                                                           |
| :-------------------- | :-------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------- |
| `--plugin-dir <path>` | 디렉토리 또는 그 디렉토리의 `.zip` 아카이브에서 플러그인을 로드합니다. 플러그인 폴더는 `.claude-plugin/plugin.json`을 보유하는 각 자식 폴더를 로드합니다. 각 플래그는 하나의 경로를 사용합니다 | `claude --plugin-dir ./my-plugin --plugin-dir ./other.zip`                  |
| `--plugin-url <url>`  | URL에서 플러그인 `.zip` 아카이브를 가져옵니다. 플래그를 반복하거나 하나의 인용된 값에서 여러 URL을 공백으로 구분하여 전달합니다                                               | `claude --plugin-url "https://example.com/a.zip https://example.com/b.zip"` |

이러한 플래그가 로드하는 플러그인은 세션 전용 플러그인입니다. `claude plugin list`는 `<name>@inline`으로 범위 `session`으로 표시하지만 같은 플래그가 하위 명령어 앞에 올 때만 표시합니다. 예를 들어 `claude --plugin-dir ./my-plugin plugin list`를 실행합니다.

세션 전용 플러그인이 설치된 플러그인과 이름을 공유하면 Claude Code는 해당 세션에 대해 세션 전용 복사본을 로드하고 설치된 것을 건너뜁니다. `claude plugin disable <name>@inline`으로 세션 전용 복사본을 비활성화했거나 관리되는 설정이 해당 플러그인 이름을 잠그면 설치된 복사본이 대신 로드됩니다. 우선순위는 [플러그인 로딩 참조](/docs/ko/plugins/loading)를 참조하세요.

관리자는 [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/ko/env-vars#variables) 변수에 명명된 폴더와 함께 두 플래그를 거부할 수 있습니다. 관리되는 [`disableSideloadFlags`](/docs/ko/settings-reference#disablesideloadflags) 설정을 사용합니다. Claude Code는 플래그가 조직의 관리되는 설정에 의해 비활성화되었다고 출력하고 시작하지 않고 `1`로 종료합니다.

Agent SDK에서 [`plugins`](/docs/ko/agent-sdk/plugins) 옵션은 `--plugin-dir`과 동등합니다.

<h2 id="next-steps">
  다음 단계
</h2>

* [플러그인 설치 및 관리](/docs/ko/plugins/install): 단계와 동일한 작업으로 각 단계에서 보는 것
* [플러그인 로딩 참조](/docs/ko/plugins/loading): 각 명령어가 디스크에서 변경하는 것과 어떤 범위가 적용되는지
* [플러그인 문제 해결](/docs/ko/plugins/troubleshooting): 설치, 마켓플레이스, 로드 및 검증 오류 메시지와 해당 수정 사항
* [플러그인 매니페스트 참조](/docs/ko/plugins/manifest-reference): `claude plugin validate`가 확인하는 필드
