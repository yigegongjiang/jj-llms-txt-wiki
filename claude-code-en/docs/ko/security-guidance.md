> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude가 코드를 작성할 때 보안 문제 포착

> security-guidance 플러그인을 설치하여 Claude가 자신의 코드 변경 사항을 취약점에 대해 검토하고 동일한 세션에서 수정하도록 합니다.

security-guidance 플러그인은 Claude가 작업하는 동안 자신의 코드 변경 사항을 일반적인 취약점에 대해 검토하고 동일한 세션에서 발견한 문제를 수정하도록 합니다. 플러그인은 주입, 안전하지 않은 역직렬화, 안전하지 않은 DOM API와 같은 문제를 코드가 풀 요청에 도달하기 전에 포착하여 다운스트림의 인간 검토자에게 떨어지는 보안 검토의 양을 줄입니다.

설치되면 플러그인이 자동으로 실행됩니다. 호출할 것도 없고 기억해야 할 별도의 명령도 없습니다.

플러그인은 [Code Review](/docs/ko/code-review)의 세션 내 동반자이며, 이는 풀 요청에서 실행됩니다. 이 플러그인은 PR에 도달하는 것을 줄입니다. Code Review는 도달하는 것을 포착합니다. 플러그인이 온디맨드 검토 및 CI 스캔과 어떻게 계층화되는지에 대해서는 [이것이 다른 보안 도구와 어떻게 맞는지](#how-this-fits-with-other-security-tools)를 참조하십시오.

<h2 id="prerequisites">
  필수 조건
</h2>

* Claude Code CLI 버전 2.1.144 이상
* `PATH`에 Python 3.8 이상. 플러그인은 `python3`, `python`, `py -3` 순서로 시도합니다.
* 작업 중인 디렉터리에 대한 git 저장소. 턴 끝 및 커밋 검토는 git 상태에 대해 diff하고 저장소 외부에서는 자동으로 건너뜁니다. 편집당 패턴 확인은 어디서나 작동합니다.

첫 실행 시 플러그인은 `~/.claude/security/` 아래에 가상 환경을 만들고 Claude Agent SDK를 설치하며, 이는 `pip` 및 네트워크 액세스가 필요합니다. 해당 설치가 실패하면 커밋 검토는 에이전트 검토 대신 단일 샷 검토로 폴백됩니다. Windows에서는 가상 환경 단계를 건너뛰므로 에이전트 커밋 검토는 `claude-agent-sdk`가 이미 가져올 수 있는 경우에만 실행되고 그렇지 않으면 동일한 방식으로 폴백됩니다.

<h2 id="install-the-plugin">
  플러그인 설치
</h2>

Claude Code 세션에서 [공식 Anthropic 마켓플레이스](/docs/ko/discover-plugins#official-anthropic-marketplace)에서 설치합니다:

```text theme={null}
/plugin install security-guidance@claude-plugins-official
```

설치는 범위를 묻습니다. 사용자 범위를 선택하여 플러그인을 사용자 설정에 기록하면 이 머신에서 시작하는 모든 새 로컬 세션에 로드됩니다. Claude Code가 마켓플레이스를 찾을 수 없다고 보고하면 먼저 `/plugin marketplace add anthropics/claude-plugins-official`을 실행한 후 설치를 다시 시도합니다.

그런 다음 현재 세션에서 `/reload-plugins`로 활성화합니다. 이는 재시작 없이 보류 중인 플러그인 변경 사항을 적용합니다:

```text theme={null}
/reload-plugins
```

<h3 id="enable-in-cloud-sessions-and-shared-repositories">
  클라우드 세션 및 공유 저장소에서 활성화
</h3>

사용자 범위 플러그인은 [웹의 Claude Code](/docs/ko/claude-code-on-the-web)로 전달되지 않습니다. 왜냐하면 이러한 세션은 머신이 아닌 Anthropic 인프라에서 실행되기 때문입니다. 거기서 플러그인을 활성화하거나 저장소를 복제하는 모든 사람에 대해 켜려면 프로젝트의 체크인된 설정에서 선언합니다:

```json .claude/settings.json theme={null}
{
  "enabledPlugins": {
    "security-guidance@claude-plugins-official": true
  }
}
```

관리자는 [관리 설정](/docs/ko/admin-setup)에서 [`enabledPlugins`](/docs/ko/settings#plugin-settings)를 설정하여 조직 전체에서 플러그인을 활성화할 수 있습니다.

<h2 id="what-the-plugin-checks">
  플러그인이 확인하는 것
</h2>

플러그인은 Claude의 작업을 세 지점에서 검토하며, 각각 다른 깊이에서 검토합니다:

* [각 파일 편집 시](#on-each-file-edit): 모델 호출 없이 위험한 호출에 대한 빠른 패턴 일치
* [각 턴의 끝에서](#at-the-end-of-each-turn): 해당 턴이 변경한 모든 것에 대한 백그라운드 모델 검토
* [Claude가 수행하는 각 커밋 또는 푸시 시](#on-each-commit-or-push-claude-makes): 주변 코드를 읽는 더 깊은 에이전트 검토

[자신의 규칙을 추가](#add-your-own-rules)하여 각 계층을 확장할 수 있습니다. 기본 제공 확인은 개별적으로 제거할 수 없지만 [각 계층을 독립적으로 비활성화](#disable-or-uninstall)할 수 있습니다.

<h3 id="on-each-file-edit">
  각 파일 편집 시
</h3>

Claude가 파일에 쓸 때 플러그인은 새 콘텐츠를 알려진 위험한 패턴에 대해 스캔합니다. 이는 모델 호출이 없는 패턴 일치이므로 사용 비용이 추가되지 않습니다.

예제 패턴 범주:

* 동적 코드 실행: `eval(`, `new Function`, `os.system`, `child_process.exec`
* 안전하지 않은 역직렬화: `pickle`
* DOM 주입: `dangerouslySetInnerHTML`, `.innerHTML =`, `document.write`
* 워크플로우 파일: `.github/workflows/` 아래의 편집으로, 저장소 수준 권한을 부여할 수 있습니다.

확인은 편집이 적용된 후 실행되고 경고를 Claude의 다음 단계 컨텍스트에 추가합니다. 각 경고는 세션당 파일당 패턴당 한 번 발생하므로 동일한 파일의 반복 일치가 대화를 범람시키지 않습니다.

`security-patterns.yaml` 파일로 이 계층에 [자신의 패턴을 추가](#add-custom-per-edit-patterns)할 수 있습니다.

<h3 id="at-the-end-of-each-turn">
  각 턴의 끝에서
</h3>

턴은 Claude가 응답하는 한 라운드입니다: 메시지를 보내고, Claude가 작업하고 회신하며, 턴이 끝납니다. 각 턴 후에 플러그인은 Claude의 편집 도구, Bash 명령 및 서브에이전트의 변경 사항을 포함하여 턴 중에 작업 트리에서 변경된 모든 것의 git diff를 계산하고 보안에 중점을 둔 별도의 Claude 검토로 보냅니다. 검토는 백그라운드에서 실행되므로 Claude의 회신이 지연되지 않습니다. 검토에서 문제를 발견하면 Claude는 발견 사항으로 다시 프롬프트되고 후속 조치로 해결합니다.

이는 문자열 일치가 포착할 수 없는 문제를 포착합니다. 예를 들어:

* 인증 우회
* 안전하지 않은 직접 객체 참조
* 주입
* 서버 측 요청 위조
* 약한 암호화

세션에서 발견 사항과 Claude의 해결책을 모두 직접 볼 수 있습니다. 검토는 턴당 최대 30개의 변경된 파일을 포함하고 최대 3번 연속으로 발생한 후 다시 사용자에게 양보합니다.

<h3 id="on-each-commit-or-push-claude-makes">
  Claude가 수행하는 각 커밋 또는 푸시 시
</h3>

Claude가 Bash 도구를 통해 `git commit` 또는 `git push`를 실행할 때 플러그인은 백그라운드에서 변경 사항에 대한 더 깊은 에이전트 검토를 실행합니다. 이 검토는 호출자, 새니타이저 및 관련 파일을 포함한 주변 코드를 읽어 발견 사항이 보고되기 전에 실제인지 결정합니다. 추가 컨텍스트는 코드베이스에서 격리되어 보면 위험해 보이지만 안전한 패턴에 대한 거짓 양성을 낮게 유지합니다.

이 계층은 Claude가 Bash 도구를 통해 수행하는 커밋 및 푸시에서만 발생합니다. 자신의 셸에서 실행하는 커밋(세션 내 `!` 셸 이스케이프 포함)은 검토되지 않습니다. 커밋 및 푸시 검토는 롤링 시간당 20개로 제한됩니다. 커밋 검토의 발견 사항이 턴 끝 검토에서 이미 보고한 것과 중복되면 Claude는 다시 프롬프트되지 않으므로 깨끗한 커밋은 이 계층에서 보이는 출력을 생성하지 않습니다.

<h3 id="review-independence-and-limits">
  검토 독립성 및 제한
</h3>

플러그인은 코드를 작성한 동일한 Claude 인스턴스에 자신을 평가하도록 요청하지 않습니다. 편집당 확인은 모델이 관여하지 않는 결정론적 문자열 일치입니다. 턴 끝 및 커밋 검토는 새로운 컨텍스트와 보안 중심 프롬프트를 사용하는 별도의 Claude 호출로 실행됩니다: 검토자는 diff에서 시작하고 원래 접근 방식에 투자하지 않으며 문제를 찾도록만 지시됩니다.

어떤 계층도 쓰기 또는 커밋을 차단하지 않습니다. 발견 사항은 쓰기 Claude에 지시로 도달하고, Claude는 대화에서 해결하며, 검토 모델은 문제를 놓칠 수 있습니다. 플러그인을 완전한 보안 솔루션이 아닌 심층 방어의 한 계층으로 취급합니다. [이것이 다른 보안 도구와 어떻게 맞는지](#how-this-fits-with-other-security-tools)를 참조하십시오.

<h2 id="add-your-own-rules">
  자신의 규칙 추가
</h2>

플러그인에는 두 가지 확장 지점이 있습니다: 모델 지원 검토를 위한 Markdown 지침 파일과 편집당 문자열 일치를 위한 YAML 또는 JSON 패턴 파일입니다. 둘 다 추가적입니다. 확인을 추가할 수 있지만 이러한 파일에서 기본 제공 확인을 비활성화할 수 없습니다.

<h3 id="add-guidance-for-the-model-backed-reviews">
  모델 지원 검토를 위한 지침 추가
</h3>

프로젝트에서 `.claude/claude-security-guidance.md`를 만들고 일반 언어로 위협 모델 및 검토 체크리스트를 설명합니다. 모델 지원 검토는 기본 제공 취약점 체크리스트와 함께 추가 컨텍스트로 로드합니다.

다음 예제는 역할 게이트 관리자 경로 및 고객 데이터 로깅 정책이 있는 웹 서비스용입니다:

```markdown .claude/claude-security-guidance.md theme={null}
# 이 저장소에 대한 보안 지침

- INFO 수준 이상에서 `customer_id` 또는 `account_number`를 로깅하지 마십시오.
- `/admin` 아래의 모든 경로는 데이터베이스 읽기 전에 `require_role("admin")`을 호출해야 합니다.
- `===` 대신 토큰 비교를 위해 `crypto.timingSafeEqual`을 사용합니다.
```

이러한 규칙은 결정론적 보호 장치가 아닌 검토자를 위한 지침입니다. 플러그인은 위반을 Claude가 수정할 발견 사항으로 표시하지만 쓰기를 차단하거나 모든 위반이 포착되도록 보장하지 않습니다. 지침은 추가적일 뿐입니다: 취약점 클래스를 무시하도록 말하는 규칙은 이러한 발견을 억제하지 않습니다. 강력한 적용을 위해 플러그인을 [편집을 차단하는 훅](/docs/ko/hooks-guide#block-edits-to-protected-files) 또는 CI 확인과 쌍으로 만듭니다.

<h3 id="add-custom-per-edit-patterns">
  사용자 정의 편집당 패턴 추가
</h3>

`.claude/security-patterns.yaml`을 만들어 [편집당 패턴 확인](#on-each-file-edit)에 정규식 또는 부분 문자열 규칙을 추가합니다. 이들은 기본 제공 패턴과 함께 결정론적 문자열 일치로 실행됩니다:

```yaml .claude/security-patterns.yaml theme={null}
patterns:
  - rule_name: internal_api_key
    substrings: ["sk_live_", "AKIA"]
    reminder: "하드코딩된 API 키 접두사. 비밀 관리자에서 자격 증명을 로드합니다."
  - rule_name: tenant_unfiltered_query
    regex: "\\.objects\\.all\\(\\)"
    paths: ["**/src/tenants/**"]
    reminder: "다중 테넌트 코드는 org_id로 필터링해야 합니다."
```

| 필드              | 유형  | 설명                                                                                   |
| :-------------- | :-- | :----------------------------------------------------------------------------------- |
| `rule_name`     | 문자열 | 경고에 표시되는 식별자                                                                         |
| `reminder`      | 문자열 | Claude의 컨텍스트에 추가되는 경고 텍스트, 1 KB로 제한됨                                                 |
| `regex`         | 문자열 | 편집된 콘텐츠와 일치하는 Python 정규식                                                             |
| `substrings`    | 목록  | 리터럴 부분 문자열; 이것 또는 `regex` 제공                                                         |
| `paths`         | 목록  | 선택 사항 glob 패턴; 규칙은 일치하는 파일에만 적용됩니다. Glob은 전체 파일 경로와 일치하므로 프로젝트 상대 패턴 앞에 `**/`를 붙입니다. |
| `exclude_paths` | 목록  | 건너뛸 선택 사항 glob 패턴; `paths`와 동일한 일치                                                   |

플러그인은 또한 동일한 스키마를 사용하여 `.claude/security-patterns.yml` 및 `.claude/security-patterns.json`을 읽습니다. JSON은 모든 Python 설치에서 작동합니다. YAML 형식은 PyYAML을 가져올 수 있어야 하며, 플러그인은 이를 설치하지 않습니다. 플러그인은 최대 50개의 사용자 정의 규칙을 로드하고 재앙적 백트래킹에 취약해 보이는 정규식을 건너뜁니다.

<h3 id="rule-file-lookup-locations">
  규칙 파일 조회 위치
</h3>

플러그인은 플러그인이 활성화된 방식과 관계없이 동일한 위치에서 `claude-security-guidance.md` 및 `security-patterns.yaml`을 찾습니다:

| 범위      | 경로                                          | 참고                  |
| :------ | :------------------------------------------ | :------------------ |
| 사용자     | `~/.claude/claude-security-guidance.md`     | 머신의 모든 프로젝트에 적용됩니다. |
| 프로젝트    | `.claude/claude-security-guidance.md`       | 저장소와 함께 체크인됨        |
| 프로젝트 로컬 | `.claude/claude-security-guidance.local.md` | Gitignored, 개인 재정의용 |

플러그인은 존재하는 모든 위치를 로드하고 연결하며, 지침 파일의 결합 상한은 8 KB입니다. 관리자는 장치 관리를 통해 `~/.claude/`에 사용자 범위 파일을 푸시하여 조직 전체 규칙을 배포할 수 있습니다. 동일한 경로가 `security-patterns.yaml`에 적용됩니다.

<h2 id="usage-cost">
  사용 비용
</h2>

[편집당 패턴 확인](#on-each-file-edit)은 모델 호출을 하지 않으며 비용을 추가하지 않습니다. [턴 끝](#at-the-end-of-each-turn) 및 [커밋](#on-each-commit-or-push-claude-makes) 검토는 각각 다른 Claude 요청처럼 [사용](/docs/ko/costs)으로 계산되는 추가 모델 사용을 소비합니다. 커밋 검토는 에이전트이며 커밋당 여러 모델 턴을 걸릴 수 있으며, 롤링 시간당 20개 검토로 제한됩니다. 파일을 변경하는 턴당 대략 하나의 검토 호출과 커밋당 하나의 더 깊은 검토를 예상하며, 둘 다 위의 상한을 따릅니다.

두 모델 지원 검토 모두 기본적으로 Claude Opus 4.7을 사용합니다. `SECURITY_REVIEW_MODEL`을 설정하여 턴 끝 검토를 위해 다른 모델을 선택하고 `SG_AGENTIC_MODEL`을 커밋 검토를 위해 선택합니다.

플러그인은 모든 플랜에서 사용 가능합니다.

<h2 id="disable-or-uninstall">
  비활성화 또는 제거
</h2>

나머지를 유지하면서 개별 계층을 끄려면 일치하는 환경 변수를 설정합니다:

| 변수                              | 효과                                                      |
| :------------------------------ | :------------------------------------------------------ |
| `ENABLE_PATTERN_RULES=0`        | [편집당 패턴 확인](#on-each-file-edit) 비활성화                    |
| `ENABLE_STOP_REVIEW=0`          | [턴 끝 diff 검토](#at-the-end-of-each-turn) 비활성화            |
| `ENABLE_COMMIT_REVIEW=0`        | [커밋 및 푸시 검토](#on-each-commit-or-push-claude-makes) 비활성화 |
| `ENABLE_CODE_SECURITY_REVIEW=0` | 모든 모델 지원 검토를 한 번에 비활성화                                  |
| `SECURITY_GUIDANCE_DISABLE=1`   | 제거하지 않고 플러그인 완전히 비활성화                                   |

사용자 범위에서 플러그인을 일시 중지하려면:

```text theme={null}
/plugin disable security-guidance@claude-plugins-official
```

사용자 범위에서 제거하려면:

```text theme={null}
/plugin uninstall security-guidance@claude-plugins-official
```

플러그인이 프로젝트의 `.claude/settings.json`을 통해 활성화된 경우 `/plugin`에서 비활성화하면 체크인된 파일을 편집하지 않고 `.claude/settings.local.json`에 재정의를 기록하므로 플러그인이 사용자에게는 꺼져 있고 팀원은 영향을 받지 않습니다. 동일한 대화 상자는 공유 `.claude/settings.json`에서 제거하여 모든 사용자를 위해 플러그인을 제거할 수 있는 옵션도 제공합니다. 이 옵션은 Claude Code v2.1.203 이상이 필요합니다. [관리 설정](/docs/ko/admin-setup)을 통해 활성화된 경우 관리자만 비활성화할 수 있습니다.

<h2 id="how-the-plugin-integrates-with-claude-code">
  플러그인이 Claude Code와 통합되는 방식
</h2>

플러그인은 전적으로 [훅](/docs/ko/hooks)에 구축되어 있으며, 이는 Claude의 루프의 특정 지점에서 자신의 코드를 실행하는 메커니즘입니다. 등록:

| 훅 이벤트                                                            | 목적                          |
| :--------------------------------------------------------------- | :-------------------------- |
| `SessionStart`                                                   | 플러그인의 Python 환경 부트스트랩       |
| `UserPromptSubmit`                                               | 턴 끝 검토가 diff하는 작업 트리 기준선 캡처 |
| `PostToolUse` on `Edit`, `Write`, and `NotebookEdit`             | 편집당 패턴 일치                   |
| `Stop`                                                           | 턴 끝 diff 검토, 백그라운드에서 실행     |
| `PostToolUse` on `Bash`, filtered to `git commit` and `git push` | 커밋 및 푸시 검토, 백그라운드에서 실행      |

자신의 훅을 구축하면 [플러그인의 소스](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/security-guidance)는 훅에서 별도의 모델 호출을 실행하고 결과를 세션으로 다시 피드하는 작업 예제입니다.

<h2 id="how-this-fits-with-other-security-tools">
  이것이 다른 보안 도구와 어떻게 맞는지
</h2>

플러그인은 심층 방어 접근 방식의 한 계층입니다. 코드가 여전히 편집기에 있는 동안 가장 빨리 문제를 포착하지만 보장이 아니며 나중의 확인을 대체하지 않습니다. 일반적인 스택:

| 단계     | 도구                                                   | 포함 내용                                  |
| :----- | :--------------------------------------------------- | :------------------------------------- |
| 세션 내   | Security guidance 플러그인                               | Claude가 작성한 코드의 일반적인 취약점, 동일한 세션에서 수정됨 |
| 온디맨드   | [`/security-review`](/docs/ko/commands#all-commands)      | 현재 분기에 대한 일회성 보안 통과, 요청할 때 실행          |
| 풀 요청 시 | [Code Review](/docs/ko/code-review), Team 및 Enterprise 플랜 | 전체 코드베이스 컨텍스트를 사용한 다중 에이전트 정확성 및 보안 검토 |
| CI에서   | 기존 정적 분석 및 종속성 스캐너                                   | 플러그인이 시도하지 않는 언어별 규칙, 공급망 확인 및 정책 적용   |

각 이후 단계는 이전 단계가 놓친 것을 포착합니다. 플러그인의 가치는 도달하는 양을 줄이는 것이지, 필요성을 제거하는 것이 아닙니다.

<h2 id="troubleshooting">
  문제 해결
</h2>

플러그인은 런타임 진단을 `~/.claude/security/log.txt`에 기록합니다. 검토가 나타나지 않으면 먼저 거기를 확인하십시오.

검토 계층이 대화에서 메시지 없이 건너뛰는 일반적인 이유:

* 디렉터리가 git 저장소가 아닙니다: 턴 끝 및 커밋 검토는 git 상태가 필요하고 저장소 외부에서 건너뜁니다
* 세션에 Anthropic 인증이 없습니다: 모델 지원 검토는 건너뛰고 편집당 패턴 확인만 실행됩니다
* `security-patterns.yaml` 파일이 있지만 PyYAML을 가져올 수 없습니다: 파일이 무시됩니다. 대신 `security-patterns.json`을 사용하십시오

<h2 id="related-resources">
  관련 리소스
</h2>

이 페이지가 다루는 부분을 더 깊이 있게 살펴보려면:

* [Code Review](/docs/ko/code-review): PR 시간 다중 에이전트 검토 설정
* [훅으로 워크플로우 자동화](/docs/ko/hooks-guide): 동일한 라이프사이클 지점에서 자신의 확인 구축
* [플러그인 발견 및 설치](/docs/ko/discover-plugins#official-anthropic-marketplace): 다른 공식 플러그인 찾아보기
