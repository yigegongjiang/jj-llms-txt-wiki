> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 조직을 위한 플러그인 추천

> 마켓플레이스 플러그인 항목에 관련성 블록을 추가하여 사용자의 작업이 일치할 때 Claude Code가 플러그인을 제안하도록 하고, 관리 설정에서 마켓플레이스를 허용 목록에 추가합니다.

Claude Code는 사용자의 세션이 해당 플러그인에 대해 정의한 신호와 일치할 때 조직의 마켓플레이스에서 플러그인 설치를 제안할 수 있습니다. 신호에는 작업 디렉토리, Claude가 읽은 파일, Claude가 실행한 명령이 포함됩니다. 플러그인의 `marketplace.json` 항목에 `relevance` 블록을 추가하여 신호를 정의합니다.

마켓플레이스 운영자가 `relevance` 항목을 작성합니다. 그 다음 관리자가 관리 설정에서 마켓플레이스를 허용 목록에 추가합니다. 마켓플레이스가 허용 목록에 추가될 때까지 사용자는 마켓플레이스에서 제안을 볼 수 없습니다.

<Note>
  다음 페이지에서 다루는 경우:

  * **플러그인을 설치하려는 경우**: [플러그인 설치 및 관리](/docs/ko/plugins/install) 참조
  * **제안을 끄려는 경우**: [플러그인 관련성 작동 방식 이해](#understand-how-plugin-relevance-works) 참조
</Note>

역할에 해당하는 섹션부터 시작하세요:

* **마켓플레이스 운영자**: [제안 작동 방식](#understand-how-plugin-relevance-works) 읽기, [플러그인 항목에 관련성 추가](#add-relevance-to-a-plugin-entry) 및 [마켓플레이스 검증](#validate-your-marketplace)
* **관리자**: [관리 설정에서 제안 활성화](#enable-suggestions-in-managed-settings)

<h2 id="understand-how-plugin-relevance-works">
  플러그인 관련성 작동 방식 이해
</h2>

`marketplace.json`의 각 플러그인 항목은 `relevance` 객체를 포함할 수 있습니다. 이 객체는 주제와 하나 이상의 신호를 이름 지정합니다. 신호는 Claude Code가 현재 세션에 대해 테스트하는 패턴입니다(예: 작업 디렉토리 또는 Claude가 읽은 파일).

신호 일치는 사용자의 머신에서 로컬로 발생하며 네트워크 트래픽을 추가하지 않습니다. Claude Code는 어떤 신호가 일치했는지 또는 해당 값을 Anthropic이나 마켓플레이스 운영자에게 보고하지 않습니다.

신호가 일치하고 플러그인이 아직 설치되지 않았을 때, Claude Code는 다음 위치에서 플러그인을 제안합니다:

* **스피너 팁**: Claude가 응답하는 동안 스피너 아래에 `/plugin install` 명령이 포함된 메시지가 나타납니다.
* **세션 시작 알림**: `cwd` 신호가 작업 디렉토리와 일치하면 사용자가 첫 번째 메시지를 보내기 전에 한 줄 알림이 나타납니다.
* **`/plugin` Discover 탭**: 플러그인이 Discover 목록의 맨 위에 고정됩니다.

[사용자가 보는 내용 미리보기](#preview-what-the-user-sees)는 각각의 정확한 텍스트와 반복 빈도를 보여줍니다.

Claude Code는 플러그인을 자동으로 설치하지 않습니다. 사용자가 항상 확인합니다.

스피너 팁과 세션 시작 알림은 모두 사용자 또는 프로젝트가 [`spinnerTipsEnabled`](/docs/ko/settings-reference#spinnertipsenabled)를 `false`로 설정하거나 [`spinnerTipsOverride`](/docs/ko/settings-reference#spinnertipsoverride)가 기본 제공 팁을 대체할 때 나타나지 않습니다. Discover 탭 핀은 두 설정 모두의 영향을 받지 않습니다.

<h2 id="add-relevance-to-a-plugin-entry">
  플러그인 항목에 관련성 추가
</h2>

`marketplace.json`의 플러그인 항목에 `relevance` 객체를 추가합니다. 다음 예제는 Claude가 `.tf` 파일을 읽거나 `terraform`을 실행할 때 `terraform-helpers` 플러그인이 관련성이 있음을 선언합니다:

```json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "plugins": [
    {
      "name": "terraform-helpers",
      "source": "./plugins/terraform-helpers",
      "description": "Your organization's Terraform conventions and helpers",
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

신호가 일치하지 않는 동안 플러그인은 Discover 목록에서 정상 위치를 유지하며 스피너 팁으로 나타나지 않습니다.

게시하기 전에 블록을 확인하려면 [마켓플레이스 검증](#validate-your-marketplace)을 참조하세요.

<h2 id="field-reference">
  필드 참조
</h2>

`relevance` 객체와 중첩된 `signals` 객체는 다음 표의 필드를 허용합니다.

이전 클라이언트는 여전히 인식하지 못하는 `relevance` 필드를 사용하는 마켓플레이스를 로드합니다. `relevance` 및 `relevance.signals` 아래의 알 수 없는 필드는 로드 시 무시되기 때문입니다. 인식된 필드의 값이 [필드 참조](#field-reference)의 제한을 초과하면 전체 플러그인 항목이 무효화되고 사용자는 수정할 때까지 마켓플레이스에서 해당 플러그인을 설치할 수 없습니다. `claude plugin validate`는 동일한 제한을 보고합니다.

<h3 id="relevance">
  `relevance`
</h3>

| 필드        | 유형     | 설명                                                                                                                             |
| :-------- | :----- | :----------------------------------------------------------------------------------------------------------------------------- |
| `topic`   | string | 선택 사항입니다. 스피너 팁에서 "\_topic\_으로 작업 중입니까?"를 채우는 구문입니다. 기본값은 각 하이픈 세그먼트가 대문자로 표기된 플러그인 이름입니다. 최대 64자입니다.                          |
| `signals` | object | 플러그인이 관련성이 있는 시기를 결정하는 매처입니다. Claude Code는 최소한 하나의 신호가 설정된 경우에만 플러그인을 제안합니다. [`relevance.signals`](#relevance-signals)를 참조하세요. |

`topic`은 종종 제품 이름입니다(예: `Terraform`). 플러그인 이름이 주제로 자연스럽게 들리지 않을 때 `design`과 같은 도메인을 사용합니다.

<h3 id="relevance-signals">
  `relevance.signals`
</h3>

`signals` 객체는 다음 필드를 허용합니다.

| 필드             | 유형               | 설명                                                                                                                                                           | 제한                                                  |
| :------------- | :--------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------- |
| `cwd`          | array of strings | 세션의 작업 디렉토리와 일치하는 Glob 패턴입니다. [작업 디렉토리 일치](#working-directory-matching)를 참조하세요.                                                                              | 각 256자의 10개 패턴                                      |
| `cli`          | array of strings | 이 세션에서 Claude가 실행한 셸 명령의 명령 이름입니다(예: `["terraform"]`). 정확한 일치입니다. [명령 이름 일치](#command-name-matching)를 참조하세요.                                                 | 각 64자의 10개 항목                                       |
| `hosts`        | array of strings | 이 세션의 Bash 명령에서 `http://` 또는 `https://` URL에 표시되는 호스트 이름입니다(예: `["registry.terraform.io"]`). 베어 소문자 호스트 이름만: 스키마, 포트 또는 경로 없음. 정확한 대소문자 구분 없는 일치입니다.         | 각 128자의 20개 항목                                      |
| `filesRead`    | array of strings | 이 세션에서 Claude가 읽은 파일의 경로와 일치하는 Glob 패턴입니다(예: `["**/*.tf"]`). 정방향 슬래시 정규화 및 대소문자 구분 없음입니다.                                                                    | 각 256자의 10개 패턴                                      |
| `manifestDeps` | array of objects | Claude가 이 세션에서 읽은 패키지 매니페스트에 선언된 종속성입니다. 각 항목은 `{ "file": "...", "pattern": "..." }`이며, 두 값 모두 정규식입니다. [매니페스트 종속성 일치](#manifest-dependency-matching)를 참조하세요. | 10개 항목, 각 값은 최대 256자입니다. 512KB보다 큰 매니페스트 파일은 건너뜁니다. |

`filesRead` 및 `manifestDeps` 신호는 또한 Claude가 이 세션에서 작성하거나 편집한 파일과 프로젝트의 자동 로드된 `CLAUDE.md` 메모리 파일에 대해 일치합니다.

<h4 id="working-directory-matching">
  작업 디렉토리 일치
</h4>

`cwd`는 세션 시작 시, 사용자가 첫 번째 메시지를 보내기 전에 일치할 수 있는 유일한 신호입니다.

Claude Code는 다음과 같이 각 `cwd` 패턴을 일치시킵니다:

* 패턴은 절대 경로로 작업 디렉토리와 일치합니다. 세션이 git 저장소 내에 있을 때, 저장소 루트에 상대적인 작업 디렉토리의 경로와도 일치합니다.
* 일치는 정방향 슬래시 정규화 및 대소문자 구분 없음입니다.
* 모든 패턴은 디렉토리 자체 및 그 아래의 모든 항목과 일치하므로 `infra`, `infra/`, `infra/**`는 동일하게 작동합니다.

<h4 id="command-name-matching">
  명령 이름 일치
</h4>

Claude Code는 Claude가 실행하는 각 셸 명령에 대해 하나의 명령 이름을 기록합니다: 선행 환경 변수 할당 및 `sudo` 이후의 첫 번째 토큰입니다. 복합 명령은 선행 명령만 기여하므로 `cd infra && terraform plan`은 `terraform`이 아닌 `cd`를 기록합니다.

<h4 id="manifest-dependency-matching">
  매니페스트 종속성 일치
</h4>

각 `manifestDeps` 항목은 두 개의 JavaScript `RegExp` 소스 문자열을 쌍으로 만듭니다:

* `file`: 매니페스트 파일의 경로와 대소문자 구분 없이 일치합니다. 경로는 일반적으로 절대 경로이므로 시작이 아닌 끝에 패턴을 고정합니다. 경로는 이 신호에 대해 구분자 정규화되지 않으므로 Windows 경로는 백슬래시를 사용합니다.
* `pattern`: 해당 파일의 내용과 대소문자 구분하여 일치합니다.

다음 예제는 `manifestDeps`를 사용하여 Claude가 SDK의 npm 패키지(여기서는 `your-sdk`라고 함)에 의존하는 `package.json`을 읽은 후 플러그인을 제안합니다.

```json theme={null}
{
  "name": "your-plugin",
  "source": "./plugins/your-plugin",
  "relevance": {
    "signals": {
      "manifestDeps": [
        {
          "file": "[/\\\\]package\\.json$",
          "pattern": "\"your-sdk\"\\s*:"
        }
      ]
    }
  }
}
```

이 예제에서 `file` 패턴은 `[/\\\\]`를 사용하여 정방향 슬래시와 백슬래시 경로 구분자 모두와 일치하고, `\\.`를 사용하여 점이 리터럴입니다. JSON에서 정규식의 각 백슬래시는 두 번 작성됩니다.

<h2 id="validate-your-marketplace">
  마켓플레이스 검증
</h2>

셸에서 마켓플레이스 디렉토리에 대해 `claude plugin validate`를 실행하여 게시하기 전에 `relevance` 블록을 확인합니다:

```bash theme={null}
claude plugin validate ./my-marketplace
```

검증자는 `relevance` 블록에 대한 오류 및 경고를 보고합니다:

* `relevance` 및 `relevance.signals` 아래의 알 수 없는 키를 경고로 보고합니다.
* `relevance` 값이 객체가 아닌 경우를 플래그합니다.
* 스키마, 포트 또는 경로를 포함하는 `signals.hosts` 항목을 거부합니다.

각 발견은 관련된 필드의 경로와 함께 인쇄되며, 출력은 `Validation passed`, `Validation passed with warnings` 또는 `Validation failed`로 끝납니다.

<h2 id="enable-suggestions-in-managed-settings">
  관리 설정에서 제안 활성화
</h2>

사용자는 관리자가 [관리 설정](/docs/ko/plugins/org)에서 마켓플레이스를 허용 목록에 추가할 때까지 마켓플레이스에서 제안을 볼 수 없습니다. 해당 `marketplace.json`이 `relevance`를 선언하더라도 마찬가지입니다.

마켓플레이스를 허용 목록에 추가하려면 관리 설정을 다음과 같이 편집합니다:

* 마켓플레이스 이름을 `pluginSuggestionMarketplaces`에 추가합니다.
* 공식 Anthropic 마켓플레이스 이외의 마켓플레이스의 경우, 마켓플레이스 소스를 [`extraKnownMarketplaces`](/docs/ko/plugins/org#require-a-marketplace-and-its-plugins)의 해당 이름 항목으로 또는 [`strictKnownMarketplaces`](/docs/ko/plugins/org#allowlist-with-strictknownmarketplaces)의 항목으로 선언합니다.

마켓플레이스가 등록되지 않은 머신이나 허용 목록에 추가된 이름에서 다른 소스로 등록된 머신에서는 해당 제안이 나타나지 않습니다. 소스 확인은 관련 없는 소스가 허용 목록에 추가된 이름으로 등록되어 조직 전체에서 플러그인을 제안받는 것을 방지합니다.

다음 `managed-settings.json`은 GitHub 저장소에서 조직 마켓플레이스를 등록하고 해당 제안을 활성화합니다:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": {
        "source": "github",
        "repo": "your-org/your-marketplace"
      }
    }
  },
  "pluginSuggestionMarketplaces": ["your-marketplace"]
}
```

공식 마켓플레이스의 이름은 공식 Anthropic 소스에서만 등록할 수 있으므로 소스 선언이 필요하지 않습니다. 공식 마켓플레이스의 경우 이름만 허용 목록에 추가합니다:

```json theme={null}
{
  "pluginSuggestionMarketplaces": ["claude-plugins-official"]
}
```

<h2 id="preview-what-the-user-sees">
  사용자가 보는 내용 미리보기
</h2>

플러그인의 `relevance` 신호가 세션 중에 일치할 때, 스피너 아래의 팁은 다음과 같이 읽습니다:

```text theme={null}
Working with Terraform? Install the terraform-helpers plugin:
/plugin install terraform-helpers@your-marketplace
```

`cwd` 신호가 세션 시작 시 일치할 때, 한 줄 알림은 다음과 같이 읽습니다:

```text theme={null}
plugin suggestion: terraform-helpers@your-marketplace · /plugin
```

`/plugin` Discover 탭에서 플러그인은 다른 결과 위에 고정되며 일치하는 신호를 이름 지정하는 주석이 있습니다(예: `suggested for this directory` 또는 `suggested for terraform commands`).

Claude Code는 주어진 플러그인을 제안하는 빈도를 제한합니다:

* 제안은 스피너 팁과 세션 시작 알림을 합쳐서 최대 3개 세션마다 한 번 나타납니다.
* 세션 시작 알림은 스피너 팁과 알림이 플러그인을 합쳐서 2번 표시한 후 나타나지 않습니다.
* 스피너 팁과 세션 시작 알림은 플러그인이 설치되면 반복되지 않습니다.
* Discover 탭은 사용자가 플러그인의 신호가 일치하는 동안 탭을 처음 열 때 플러그인을 고정합니다. Claude Code는 이를 `~/.claude.json`에 기록하므로 사용자가 나중에 해당 머신에서 `/plugin`을 열 때마다 플러그인은 정상 순서로 나타납니다.

<h2 id="see-also">
  참고 항목
</h2>

* [마켓플레이스 호스팅](/docs/ko/plugins/host-marketplace): 플러그인을 호스팅하는 마켓플레이스 실행
* [마켓플레이스 참조](/docs/ko/plugins/marketplace-reference#plugin-entries): 플러그인 항목이 허용하는 모든 필드
* [CLI에서 플러그인 추천](/docs/ko/plugins/cli-hints): Claude Code의 세션 신호 대신 자신의 CLI에서 사용자에게 메시지 표시
* [조직을 위한 플러그인 관리](/docs/ko/plugins/org): `extraKnownMarketplaces`, `strictKnownMarketplaces` 및 나머지 플러그인 정책 키
