> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 조직을 위한 플러그인 추천

> 마켓플레이스 플러그인 항목에 관련성 블록을 추가하여 사용자의 작업이 일치할 때 Claude Code가 플러그인을 제안하도록 합니다.

조직의 플러그인 마켓플레이스를 운영하는 경우, 사용자가 작업 중인 내용을 기반으로 Claude Code가 특정 플러그인을 제안하도록 할 수 있습니다. `marketplace.json`의 플러그인 항목에 `relevance` 블록을 추가한 다음 관리 설정에서 마켓플레이스를 허용 목록에 추가합니다. 사용자의 세션이 선언된 신호 중 하나와 일치하면 Claude Code가 해당 플러그인에 대한 설치 제안을 표시합니다.

마켓플레이스에서 선언한 제안은 [관리 설정](/docs/ko/settings#settings-files)을 통해 마켓플레이스별로 선택 사항입니다. 관리자가 공식 Anthropic 마켓플레이스를 포함하여 허용 목록에 추가할 때까지 마켓플레이스의 `relevance` 선언이 제안을 생성하지 않습니다. Claude Code에는 이 허용 목록과 무관한 기본 제안 하나가 포함되어 있습니다. 이 팁과 모든 마켓플레이스에서 선언한 팁은 [`spinnerTipsEnabled`](/docs/ko/settings#available-settings)가 `false`로 설정되면 비활성화됩니다.

이 기능을 사용하려면 Claude Code v2.1.152 이상이 필요합니다. 이전 클라이언트는 `relevance` 필드를 무시합니다.

이 페이지는 마켓플레이스 운영자 및 엔터프라이즈 관리자를 위한 것입니다. 플러그인을 설치하려는 경우 [플러그인 발견 및 설치](/docs/ko/discover-plugins)를 참조하세요.

<h2 id="how-it-works">
  작동 방식
</h2>

`marketplace.json`의 각 플러그인 항목은 `relevance` 객체를 포함할 수 있습니다. 이 객체는 주제와 하나 이상의 신호를 지정합니다. 신호는 Claude Code가 현재 세션에 대해 테스트하는 패턴입니다. 예를 들어 작업 디렉터리 또는 Claude가 읽은 파일입니다.

신호 일치는 사용자의 컴퓨터에서 로컬로 발생합니다. 일치는 네트워크 트래픽을 추가하지 않으며 어떤 신호가 일치했는지 또는 해당 값을 Anthropic이나 마켓플레이스 운영자에게 보고하지 않습니다.

신호가 일치하고 플러그인이 아직 설치되지 않은 경우 Claude Code는 플러그인을 세 곳에 표시합니다.

* **스피너 팁**: Claude가 응답하는 동안 스피너 아래에 `/plugin install` 명령과 함께 "Working with *topic*? Install the *plugin* plugin" 메시지가 나타납니다.
* **세션 시작 제안**: `cwd` 신호가 작업 디렉터리와 일치하면 첫 번째 턴 전에 한 줄의 `plugin suggestion: <name>@<marketplace> · /plugin` 알림이 나타납니다. 이 표면은 Claude Code v2.1.153 이상이 필요합니다.
* **`/plugin` 발견 탭**: 플러그인이 발견 목록의 맨 위에 고정되며 "suggested for this directory" 또는 "suggested for stripe commands"와 같은 주석이 표시됩니다. 이 표면은 Claude Code v2.1.154 이상이 필요합니다.

스피너 팁과 세션 시작 알림은 스피너 팁 시스템의 일부입니다. 사용자 또는 프로젝트가 `spinnerTipsEnabled`를 `false`로 설정하거나 `excludeDefault`를 사용하여 사용자 정의 `spinnerTipsOverride`가 구성된 경우 둘 다 비활성화됩니다. 발견 탭 핀은 팁 설정과 무관합니다.

Claude Code는 플러그인을 자동으로 설치하지 않습니다. 사용자가 항상 확인합니다.

<h2 id="add-relevance-to-a-plugin-entry">
  플러그인 항목에 관련성 추가
</h2>

`marketplace.json`의 플러그인 항목에 `relevance` 객체를 추가합니다. 다음 예제는 Claude가 `.tf` 파일을 읽거나 Claude가 `terraform`을 실행할 때 `terraform-helpers` 플러그인이 관련이 있음을 선언합니다.

```json theme={null}
{
  "name": "acme-corp-plugins",
  "owner": { "name": "Acme Platform Team" },
  "plugins": [
    {
      "name": "terraform-helpers",
      "source": "./plugins/terraform-helpers",
      "description": "Acme conventions and helpers for Terraform",
      "relevance": {
        "topic": "Terraform",
        "signals": {
          "cli": ["terraform"],
          "filesRead": ["**/*.tf"]
        }
      }
    }
  ]
}
```

`relevance` 블록이 있지만 일치하는 신호가 없는 플러그인은 다른 마켓플레이스 항목처럼 작동합니다. 발견 목록에 정상 위치에 나타나며 스피너 팁으로 표시되지 않습니다.

<h2 id="field-reference">
  필드 참조
</h2>

<h3 id="relevance">
  `relevance`
</h3>

| 필드        | 유형     | 설명                                                                                                                                                                                                                    |
| :-------- | :----- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `topic`   | string | 선택 사항입니다. 스피너 팁에서 "Working with *topic*?"를 채우는 구문입니다. 종종 제품 이름입니다. 예를 들어 `Stripe`입니다. 플러그인 이름이 주제로 자연스럽게 읽히지 않을 때 `design`과 같은 도메인을 사용합니다. 기본값은 각 하이픈 세그먼트가 대문자로 표기된 플러그인 이름입니다. 세션 시작 알림은 이 값을 사용하지 않습니다. 최대 64자입니다. |
| `signals` | object | 플러그인이 관련이 있는 시기를 결정하는 매처입니다. 플러그인이 제안 가능하려면 최소 하나의 신호가 필요합니다. 아래 표를 참조하세요.                                                                                                                                            |

<h3 id="relevance-signals">
  `relevance.signals`
</h3>

| 필드             | 유형               | 설명                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :------------- | :--------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cwd`          | array of strings | 세션의 작업 디렉터리에 대해 일치하는 Glob 패턴입니다. 절대 경로로 일치하며, git 저장소 내에 있을 때 저장소 루트에 상대적인 경로로 일치합니다. 정방향 슬래시로 정규화되고 대소문자를 구분하지 않습니다. 모든 패턴은 디렉터리 자체 및 그 아래의 모든 항목과 일치하므로 `infra`, `infra/`, 및 `infra/**`는 동일하게 작동합니다. 이것은 첫 번째 턴 전에 세션 시작 시 일치할 수 있는 유일한 신호입니다. 최대 10개 패턴, 각각 256자입니다.                                                                                                                                                                                                                          |
| `cli`          | array of strings | Claude가 이 세션에서 실행한 셸 명령의 명령 이름입니다. 예를 들어 `["stripe"]`입니다. 모든 플랫폼에 적용됩니다. Windows에서 PowerShell 또는 Git Bash를 통해 실행된 명령은 동일한 방식으로 기록됩니다. Claude Code는 셸 도구 호출당 하나의 명령 이름을 기록합니다. 선행 환경 변수 할당 및 `sudo` 이후의 첫 번째 토큰입니다. 복합 명령은 선행 명령만 기여하므로 `cd infra && terraform plan`은 `cd`를 기록하며 `terraform`은 기록하지 않습니다. 정확한 일치입니다. 최대 10개 항목, 각각 64자입니다.                                                                                                                                                         |
| `hosts`        | array of strings | 이 세션의 Bash 명령에서 `http://` 또는 `https://` URL에서 본 호스트 이름입니다. 예를 들어 `["api.stripe.com"]`입니다. 스키마, 포트 또는 경로 없이 베어 소문자 호스트 이름만 해당합니다. 정확한 대소문자를 구분하지 않는 일치입니다. 최대 20개 항목, 각각 128자입니다.                                                                                                                                                                                                                                                                                                                 |
| `filesRead`    | array of strings | Claude가 이 세션에서 읽은 파일의 경로에 대해 일치하는 Glob 패턴입니다. 예를 들어 `["**/*.tf"]`입니다. 정방향 슬래시로 정규화되고 대소문자를 구분하지 않습니다. 최대 10개 패턴, 각각 256자입니다.                                                                                                                                                                                                                                                                                                                                                                     |
| `manifestDeps` | array of objects | Claude가 이 세션에서 읽은 패키지 매니페스트에 선언된 종속성입니다. 각 항목은 `{ "file": "...", "pattern": "..." }`이며, 여기서 `file`은 세션 상태에 기록된 매니페스트 파일의 경로(일반적으로 절대 경로)에 대해 일치하는 정규식이고 `pattern`은 해당 파일의 내용에 대해 일치하는 정규식입니다. 절대 경로와 일치하지 않는 시작 앵커 패턴이므로 `file`을 끝에 앵커합니다. 예를 들어 JSON 이스케이프 형식의 `[/\\\\]package\\.json$`입니다. 경로는 이 신호에 대해 구분자로 정규화되지 않으므로 Windows 경로는 백슬래시를 사용합니다. 512 KB보다 큰 매니페스트 파일은 건너뜁니다. 두 값 모두 최대 256자의 JavaScript `RegExp` 소스 문자열입니다. `file`은 대소문자를 구분하지 않게 일치합니다. `pattern`은 대소문자를 구분합니다. 최대 10개 항목입니다. |

`cli`, `hosts`, `filesRead`, 및 `manifestDeps` 신호는 세션 기록이 필요하므로 스피너 팁과 발견 탭에서만 일치할 수 있습니다. 세션 시작 시에는 `cwd`만 일치할 수 있습니다. `filesRead` 및 `manifestDeps` 신호는 세션의 기록된 파일 상태를 테스트합니다. 여기에는 Claude가 작성하거나 편집한 파일과 자동 로드된 `CLAUDE.md` 메모리 파일도 포함됩니다.

다음 예제는 `manifestDeps`를 사용하여 Claude가 `stripe`에 의존하는 `package.json`을 읽은 후 Stripe 플러그인을 제안합니다. `file` 패턴은 `[/\\\\]`를 사용하므로 정방향 슬래시와 백슬래시 경로 구분자 모두와 일치하며 `\\.`는 점이 리터럴임을 의미합니다. JSON에서 정규식의 각 백슬래시는 두 번 작성됩니다.

```json theme={null}
{
  "name": "stripe-helpers",
  "source": "./plugins/stripe-helpers",
  "relevance": {
    "topic": "Stripe",
    "signals": {
      "manifestDeps": [
        {
          "file": "[/\\\\]package\\.json$",
          "pattern": "\"stripe\"\\s*:"
        }
      ]
    }
  }
}
```

<Note>
  `relevance` 및 `relevance.signals` 아래의 알 수 없는 필드는 로드 시 무시되므로 이전 Claude Code 클라이언트는 마켓플레이스를 계속 로드합니다. `claude plugin validate`를 실행하여 경고로 표시합니다.
</Note>

<h2 id="enable-suggestions-in-managed-settings">
  관리 설정에서 제안 활성화
</h2>

`marketplace.json`에서 `relevance`를 선언하는 것만으로는 충분하지 않습니다. 관리자는 제안이 사용자에게 나타나기 전에 [관리 설정](/docs/ko/settings#settings-files)에서 마켓플레이스를 허용 목록에 추가해야 합니다.

마켓플레이스 이름을 `pluginSuggestionMarketplaces`에 추가합니다. 공식 Anthropic 마켓플레이스 이외의 마켓플레이스의 경우 동일한 관리 설정에서 마켓플레이스 소스를 선언합니다. 이는 해당 이름의 `extraKnownMarketplaces` 항목 또는 `strictKnownMarketplaces`의 항목으로 선언합니다. 허용 목록에 추가된 이름은 마켓플레이스가 다른 소스에서 등록된 경우 무시됩니다. 이는 관련 없는 소스가 허용 목록에 추가된 이름으로 등록되어 조직 전체에서 플러그인이 제안되는 것을 방지합니다.

다음 `managed-settings.json`은 GitHub 저장소에서 조직 마켓플레이스를 등록하고 제안을 활성화합니다.

```json theme={null}
{
  "extraKnownMarketplaces": {
    "acme-corp-plugins": {
      "source": {
        "source": "github",
        "repo": "acme-corp/claude-plugins"
      }
    }
  },
  "pluginSuggestionMarketplaces": ["acme-corp-plugins"]
}
```

공식 마켓플레이스는 소스 선언 요구 사항에서 제외됩니다. 이름은 공식 Anthropic 소스에서만 등록할 수 있기 때문입니다. 이름만 허용 목록에 추가하는 것으로 충분합니다.

```json theme={null}
{
  "pluginSuggestionMarketplaces": ["claude-plugins-official"]
}
```

전체 구성 세부 정보는 [설정 참조](/docs/ko/settings)에서 `pluginSuggestionMarketplaces` 및 [`extraKnownMarketplaces`](/docs/ko/settings#extraknownmarketplaces)를 참조하세요.

<h2 id="what-the-user-sees">
  사용자가 보는 것
</h2>

신호가 세션 중에 일치하면 스피너 팁은 다음과 같이 읽습니다.

```text theme={null}
Working with Terraform? Install the terraform-helpers plugin:
/plugin install terraform-helpers@acme-corp-plugins
```

세션 시작 시 일치하는 `cwd` 신호는 한 줄의 알림을 표시합니다.

```text theme={null}
plugin suggestion: terraform-helpers@acme-corp-plugins · /plugin
```

주어진 플러그인의 제안은 스피너 팁과 세션 시작 알림을 합쳐서 최대 3개 세션마다 한 번씩 나타나며, 플러그인이 설치되면 둘 다 반복되지 않습니다. 세션 시작 알림은 제안이 두 번 표시된 후 나타나지 않습니다.

`/plugin` 발견 탭에서 플러그인은 다른 결과 위에 고정되며 `suggested for this directory` 또는 `suggested for terraform commands`와 같이 일치하는 신호의 이름을 지정하는 주석이 표시됩니다. 발견 탭은 주어진 플러그인을 한 번 고정합니다. 이후 방문은 정상 순서로 나열합니다. 발견 탭 핀은 Claude Code v2.1.154 이상이 필요합니다. v2.1.152에서는 스피너 팁만 나타나며 세션 시작 알림은 v2.1.153에서 추가됩니다.

<h2 id="validate-your-marketplace">
  마켓플레이스 검증
</h2>

마켓플레이스 디렉터리에 대해 `claude plugin validate`를 실행하여 게시하기 전에 `relevance` 블록을 확인합니다.

```
claude plugin validate ./my-marketplace
```

검증자는 `relevance` 및 `relevance.signals` 아래의 알 수 없는 키를 경고로 보고하고, 객체가 아닌 `relevance` 값을 플래그하며, 스키마, 포트 또는 경로를 포함하는 `signals.hosts` 항목을 거부합니다.

<h2 id="see-also">
  참고 항목
</h2>

* [플러그인 마켓플레이스 생성 및 배포](/docs/ko/plugin-marketplaces): 플러그인을 호스팅하는 마켓플레이스를 구축합니다.
* [CLI에서 플러그인 추천](/docs/ko/plugin-hints): Claude Code의 세션 신호 대신 자신의 CLI에서 사용자에게 메시지를 표시합니다.
* [설정](/docs/ko/settings): `pluginSuggestionMarketplaces` 및 `extraKnownMarketplaces`에 대한 전체 참조입니다.
