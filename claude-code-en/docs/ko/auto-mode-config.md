> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 자동 모드 구성

> 자동 모드 분류기에 조직이 신뢰하는 저장소, 버킷 및 도메인을 알려줍니다. 환경 컨텍스트를 설정하고, 기본 차단 및 허용 규칙을 재정의하며, 자동 모드 CLI 하위 명령으로 유효한 구성을 검사합니다.

[자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)를 사용하면 Claude Code가 도구 호출을 분류기를 통해 라우팅하여 비가역적이거나 파괴적이거나 환경 외부를 대상으로 하는 모든 것을 차단함으로써 일상적인 권한 프롬프트 없이 실행될 수 있습니다. 거부 및 명시적 요청 규칙은 분류기 전에 평가되며 여전히 차단하거나 프롬프트합니다. `autoMode` 설정 블록을 사용하여 분류기에 조직이 신뢰하는 저장소, 버킷 및 도메인을 알려주면 일상적인 내부 작업 차단을 중지합니다.

<Note>
  자동 모드는 Anthropic API, [AWS의 Claude Platform](/docs/ko/claude-platform-on-aws), Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry 및 로그인된 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway) 세션을 포함한 모든 제공자의 모든 사용자가 사용할 수 있습니다. Claude Code가 계정에 대해 자동 모드를 사용할 수 없다고 보고하는 경우 지원되는 모델 및 Team 및 Enterprise 플랜의 조직 수준 제어도 다루는 [전체 요구사항](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)을 확인하십시오. v2.1.158부터 v2.1.206까지 Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry 및 Claude 앱 게이트웨이 세션의 자동 모드는 `CLAUDE_CODE_ENABLE_AUTO_MODE=1` 설정이 필요했습니다. v2.1.207은 이 요구사항을 제거했습니다.
</Note>

기본적으로 분류기는 작업 디렉토리와 현재 저장소의 구성된 원격만 신뢰합니다. 회사의 소스 제어 조직으로 푸시하거나 팀 클라우드 버킷에 쓰기와 같은 작업은 `autoMode.environment`에 추가할 때까지 차단됩니다.

자동 모드에 세션이 어떻게 진입하고 분류기가 기본적으로 무엇을 차단하는지에 대해서는 [권한 모드 페이지의 자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)를 참조하십시오. 이 페이지는 구성 참조입니다.

이 페이지에서는 다음을 다룹니다:

* [`permissions.ask`를 사용하여 푸시 및 풀 요청에 대한 사람 체크포인트 추가](#add-a-human-checkpoint)
* [CLAUDE.md, 사용자 설정 및 관리 설정 전체에서 규칙을 설정할 위치 선택](#where-the-classifier-reads-configuration)
* [`autoMode.environment`를 사용하여 신뢰할 수 있는 인프라 정의](#define-trusted-infrastructure)
* [`/auto-mode-setup`을 사용하여 환경 항목 생성](#generate-environment-entries)
* [기본값이 파이프라인에 맞지 않을 때 차단 및 허용 규칙 재정의](#override-the-block-and-allow-rules)
* [`/permissions`에서 설정 파일을 열지 않고 규칙 편집](#edit-rules-from-permissions)
* [`autoMode.classifyAllShell`을 사용하여 모든 셸 명령을 분류기를 통해 라우팅](#route-all-shell-commands-through-the-classifier)
* [`claude auto-mode` 하위 명령으로 기본값 및 유효한 구성 검사](#inspect-the-defaults-and-your-effective-config)
* [거부 사항 검토](#review-denials)하여 다음에 추가할 항목을 파악합니다.

<h2 id="common-boundaries">
  일반적인 경계
</h2>

자동 모드는 작업 중인 저장소의 모든 브랜치(기본 브랜치 포함)로의 푸시와 기본적으로 풀 요청 생성을 허용합니다. `production`, `release`, 또는 `gh-pages`와 같이 배포 또는 게시 대상으로 표시되는 기본이 아닌 브랜치는 해당 기본값에 포함되지 않습니다. 분류기는 프로덕션 배포를 포함하여 자체 조건에 따라 해당 푸시를 판단합니다. 푸시의 콘텐츠도 여전히 확인되므로 강제 푸시, 커밋에 진입하는 시크릿, 또는 CI나 배포 파이프라인이 실행할 때 시크릿을 저장소 외부로 보낼 변경 사항은 차단된 상태로 유지됩니다.

<Info>v2.1.211 이전에는 분류기가 작업 브랜치, Claude가 생성한 브랜치, 그리고 기본 브랜치로의 일상적인 푸시만 허용했습니다.</Info>

Claude의 푸시 및 풀 요청 명령 전에 사람의 체크포인트를 원하신다면 권한 규칙을 추가하십시오. [아래의 레시피](#add-a-human-checkpoint)는 다른 모든 것에 대해 자동 모드를 유지합니다.

<h3 id="add-a-human-checkpoint">
  사람의 체크포인트 추가
</h3>

가장 직접적인 메커니즘은 [`permissions.ask`](/docs/ko/permissions#permission-rule-syntax)입니다. 아래와 같은 콘텐츠 범위 ask 규칙은 분류기 전에 평가되며, 명시적 ask 규칙이 해당 작업에 대한 프롬프트를 받으려는 의도를 나타내기 때문에 자동 모드에서도 항상 권한 프롬프트를 강제합니다. [설정](/docs/ko/settings#where-settings-live)에 규칙을 추가하십시오:

```json theme={null}
{
  "permissions": {
    "ask": [
      "Bash(git push *)",
      "Bash(gh pr create *)"
    ]
  }
}
```

이 규칙은 `git push` 또는 `gh pr create`로 시작하는 명령과 일치합니다. Claude가 `git -C <dir> push` 또는 `git -c <key>=<value> push`와 같이 다른 방식으로 작성한 푸시는 [규칙과 일치하지 않으므로](/docs/ko/permissions#bash-rule-limits) 체크포인트되지 않습니다. 전체 명령 텍스트를 검사하는 체크포인트의 경우 [PreToolUse 훅](/docs/ko/hooks#pretooluse)을 추가하십시오.

경계가 얼마나 견고해야 하는지에 맞는 메커니즘을 선택하십시오:

| 경계              | 메커니즘                           | 자동 모드에서의 동작                                                                                                                        |
| :-------------- | :----------------------------- | :--------------------------------------------------------------------------------------------------------------------------------- |
| 작업 전에 프롬프트      | `permissions.ask`              | 위의 레시피와 같은 콘텐츠 범위 규칙에 일치하는 명령에 대해 항상 프롬프트합니다. 분류기는 일치하는 작업을 자동으로 승인할 수 없습니다.                                                       |
| 작업을 절대 실행하지 않음  | `permissions.deny`             | 분류기가 참고되기 전에 차단합니다. 분류기도 사용자 의도도 이를 무시할 수 없습니다.                                                                                    |
| 이 세션에 대한 일회성 경계 | "검토할 때까지 푸시하지 마세요"와 같이 대화에서 명시 | 분류기는 일치하는 작업을 차단하지만, [컨텍스트 압축](/docs/ko/costs#reduce-token-usage)이 이를 명시한 메시지를 제거하면 경계가 손실될 수 있습니다. 지속적인 보장을 위해 ask 또는 deny 규칙을 사용하십시오. |

<h2 id="where-the-classifier-reads-configuration">
  분류기가 구성을 읽는 위치
</h2>

분류기는 Claude 자체가 로드하는 것과 동일한 [CLAUDE.md](/docs/ko/memory) 콘텐츠를 읽으므로, 프로젝트의 CLAUDE.md에 있는 "절대 강제 푸시하지 않기"와 같은 지시사항은 Claude와 분류기를 동시에 제어합니다. 프로젝트 규칙 및 동작 규칙을 시작하는 위치입니다.

여러 프로젝트에 걸쳐 적용되는 규칙(예: 신뢰할 수 있는 인프라 또는 조직 전체 거부 규칙)의 경우 `autoMode` 설정 블록을 사용합니다. 분류기는 다음 범위에서 `autoMode`를 읽습니다:

| 범위                            | 파일                                     | 용도                         |
| :---------------------------- | :------------------------------------- | :------------------------- |
| 한 명의 개발자                      | `~/.claude/settings.json`              | 개인 신뢰할 수 있는 인프라            |
| 조직 전체                         | [관리되는 설정](/docs/ko/server-managed-settings) | 모든 개발자에게 배포되는 신뢰할 수 있는 인프라 |
| `--settings` 플래그 또는 Agent SDK | 인라인 JSON                               | 자동화를 위한 호출별 재정의            |

분류기는 `.claude/settings.json` 또는 `.claude/settings.local.json`의 프로젝트 설정에서 `autoMode`를 읽지 않습니다. 두 파일 모두 저장소 디렉토리에 있으므로 체크인된 저장소 또는 빌드 단계가 자체 허용 규칙을 주입할 수 있습니다. v2.1.207 이전에는 분류기도 `.claude/settings.local.json`을 읽었습니다. 해당 파일의 `autoMode` 블록을 `~/.claude/settings.json`으로 이동합니다. `.claude/settings.local.json`을 제외하면 저장소가 파일을 커밋하거나 로컬 도구 또는 빌드 단계가 파일을 작성하는 경우도 해결됩니다.

각 범위의 항목이 결합됩니다. 개발자는 `environment`, `allow`, `soft_deny` 및 `hard_deny`를 개인 항목으로 확장할 수 있지만 관리되는 설정이 제공하는 항목을 제거할 수 없습니다. 허용 규칙이 분류기 내의 소프트 블록 규칙에 대한 예외로 작동하므로 개발자가 추가한 `allow` 항목은 조직의 `soft_deny` 항목을 재정의할 수 있습니다. 조합은 가산적이며 하드 정책 경계가 아닙니다.

<Note>
  분류기는 [권한 시스템](/docs/ko/permissions) 이후에 실행되는 두 번째 게이트입니다. 사용자 의도나 분류기 구성에 관계없이 절대 실행되어서는 안 되는 작업의 경우 관리되는 설정에서 `permissions.deny`를 사용합니다. 이는 분류기를 참조하기 전에 작업을 차단하며 재정의될 수 없습니다.
</Note>

<h2 id="define-trusted-infrastructure">
  신뢰할 수 있는 인프라 정의
</h2>

대부분의 조직에서 `autoMode.environment`는 설정해야 할 유일한 필드입니다. 이 필드는 분류기에 어떤 저장소, 버킷, 도메인이 신뢰할 수 있는지 알려줍니다. 분류기는 이를 사용하여 "외부"가 무엇인지 결정하므로, 나열되지 않은 모든 대상은 잠재적 데이터 유출 대상입니다.

Claude Code v2.1.198부터 `claude auto-mode defaults`는 세 가지 종류의 환경 항목을 출력합니다. v2.1.195 이전 버전은 처음 다섯 개의 신뢰 슬롯만 출력합니다.

* **컨텍스트 슬롯**: 조직, 스택, 보안 태세를 설명하여 분류기가 컨텍스트의 다른 규칙을 읽을 수 있도록 합니다. 각각은 `None configured`로 기본 설정되거나 옆에 명시된 보수적 가정으로 기본 설정됩니다:
  * **조직**
  * **Claude Code의 주요 용도**: 소프트웨어 개발으로 기본 설정됨
  * **클라우드 제공자**
  * **저장소 가시성**: 저장소는 원격 호스트와 이름이 달리 나타내지 않는 한 비공개로 간주되며, 분류기가 읽는 대화에서 이전의 가시성 확인이 공개임을 보여주는 경우도 예외입니다.

    분류기는 Claude Code 자체가 보낸 분류기 요청에서 사용자의 메시지와 Claude가 실행하는 명령을 읽으며 그 출력은 읽지 않습니다. 증거는 저장소를 공개로 명시하는 사용자 자신의 메시지와 같이 분류기가 읽을 수 있는 것이어야 합니다. `gh repo view`의 출력만으로는 분류기에 도달하지 않습니다. 트랜스크립트 증거 확인에는 Claude Code v2.1.200 이상이 필요합니다
  * **내부 공유 / 스니펫 호스팅**: 공개 붙여넣기 및 gist 서비스는 사용자가 명시할 때까지 신뢰 경계 외부로 취급됩니다
  * **조직별 CLI**
  * **비밀 관리**
  * **CI/CD 배포 대상**
  * **네트워크 태세**
  * **호스트 격리**: 개방형 인터넷이 있는 일반 개발자 머신 또는 CI 러너로 기본 설정됩니다. Claude Code가 송신 허용 목록이 있거나 접근하면 안 되는 이웃이 있는 컨테이너, VM 또는 Pod에서 실행되는 경우, 허용된 호스트, 클라우드 메타데이터 엔드포인트에 도달 가능 여부, 작업이 사용하는 클라우드 프로젝트, 클러스터 또는 레지스트리, 그리고 어떤 신원으로 사용하는지 명시합니다. 이 항목이 해당 신원을 명시할 때까지 분류기는 [호스트 자신의 자격 증명에 대한 요청을 차단합니다](/docs/ko/permission-modes#what-the-classifier-blocks-by-default). Claude Code v2.1.257 이상이 필요합니다
  * **보호된 배포 네임스페이스 / 환경**: 사용자가 명시할 때까지 민감한 원격 대상 휴리스틱으로 폴백됩니다
  * **데이터 보존 / 기밀 해제**
* **신뢰 슬롯**: 분류기가 경계 내부로 취급하는 것을 명시합니다. 슬롯은 신뢰할 수 있는 저장소, 소스 제어, 신뢰할 수 있는 내부 도메인, 신뢰할 수 있는 클라우드 버킷, 주요 내부 서비스, 내부 패키지 레지스트리입니다. 저장소 및 소스 제어 항목은 작업 저장소 및 구성된 원격으로 기본 설정됩니다. 다른 모든 신뢰 슬롯은 `None configured`로 기본 설정되므로, 추가할 때까지 다른 것은 신뢰되지 않습니다. 저장소의 가시성은 기밀 자료만 범위를 지정합니다: 비공개 저장소는 기밀 자료의 허용 가능한 대상이지만, 저장소를 비공개로 만든다고 해서 비밀이나 개인 데이터 또는 위탁받은 데이터를 그 안에 넣어도 되는 것은 아니며, 분류기는 작업 저장소 외부에서 이식되거나 재지정되거나 처음 읽은 콘텐츠를 해당 저장소 자신의 작업으로 취급하지 않습니다. 이 범위 지정에는 Claude Code v2.1.203 이상이 필요합니다.
* **민감도 슬롯**: 보호 규칙이 고위험으로 취급하는 것을 명시합니다. 슬롯은 민감한 데이터 위치 및 대상, 민감한 원격 대상, 보호된 IaC 범위입니다. 각각은 `prod` 또는 `production`을 포함하는 이름의 모든 호스트 또는 네임스페이스를 민감한 원격 대상으로 취급하는 것과 같은 광범위한 휴리스틱으로 기본 설정되므로, 보호 규칙은 아무것도 구성하기 전에 활성화됩니다. 민감도 슬롯에 구체적인 대상을 명시하면 보호 규칙이 휴리스틱 대신 명시된 대상에 적용됩니다.

<Info>v2.1.211 이전에는 컨텍스트 슬롯에 `main` 및 `master`를 보호된 것으로 취급하는 기본 / 보호된 분기 항목도 포함되었습니다. v2.1.211에서 제거되었습니다: [작업 중인 저장소의 모든 분기로의 푸시](#common-boundaries)는 기본적으로 허용되므로 구성할 보호된 분기 기본값이 없습니다.</Info>

기본값과 함께 자신의 항목을 추가하려면 배열에 리터럴 문자열 `"$defaults"`를 포함합니다. 기본 항목은 해당 위치에 삽입되므로 사용자 정의 항목은 앞이나 뒤에 올 수 있습니다.

다음 예제는 기본 항목을 유지하고 조직의 저장소, 버킷, 도메인, 서비스를 추가합니다.

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Source control: github.example.com/acme-corp and all repos under it",
      "Trusted cloud buckets: s3://acme-build-artifacts, gs://acme-ml-datasets",
      "Trusted internal domains: *.corp.example.com, api.internal.example.com",
      "Key internal services: Jenkins at ci.example.com, Artifactory at artifacts.example.com"
    ]
  }
}
```

설정을 저장한 후 `claude auto-mode config`를 실행하여 [유효한 규칙이 항목을 포함하는지 확인합니다](#inspect-the-defaults-and-your-effective-config).

항목은 정규식이나 도구 패턴이 아닌 산문입니다. 분류기는 이를 자연어 규칙으로 읽습니다. 새로운 엔지니어에게 인프라를 설명하는 방식으로 작성합니다. 철저한 환경 섹션은 다음을 포함합니다:

* **조직**: 회사 이름과 Claude Code가 주로 사용되는 용도(예: 소프트웨어 개발, 인프라 자동화, 데이터 엔지니어링)
* **소스 제어**: 개발자가 푸시하는 모든 GitHub, GitLab 또는 Bitbucket 조직
* **클라우드 제공자 및 신뢰할 수 있는 버킷**: Claude가 읽고 쓸 수 있어야 하는 버킷 이름 또는 접두사
* **신뢰할 수 있는 내부 도메인**: `*.internal.example.com`과 같은 네트워크 내부의 API, 대시보드, 서비스에 대한 호스트명
* **주요 내부 서비스**: CI, 아티팩트 레지스트리, 내부 패키지 인덱스, 인시던트 도구
* **내부 패키지 레지스트리**: 설치가 라우팅되어야 하는 비공개 npm, PyPI 또는 기타 레지스트리이므로, 공개 레지스트리를 위해 이를 우회하는 설치가 차단됩니다
* **민감한 데이터 위치 및 대상**: 개인 데이터, 기밀 비즈니스 데이터, 자격 증명, 규제 데이터 또는 유사하게 민감한 자료를 보유하는 버킷, 데이터베이스 또는 경로, 그리고 각 위치의 데이터가 공유될 수 있는 대상이므로 분류기가 콘텐츠에서 추측하는 대신 해당 위치를 보호합니다. Claude Code v2.1.195부터 v2.1.197은 이 항목을 PII / 규제 데이터 위치로 명시하고 대상 차원 없이 개인 또는 규제 데이터를 보유하는 위치만 포함합니다
* **민감한 원격 대상**: 프로덕션으로 계산되는 네임스페이스, 호스트 또는 컨테이너이므로 원격 셸 및 포트 포워드에는 명시적 승인이 필요합니다
* **보호된 IaC 범위**: 적용 또는 삭제가 항상 변경을 명시하도록 요구해야 하는 인프라 리소스
* **추가 컨텍스트**: 규제 산업 제약, 다중 테넌트 인프라 또는 분류기가 위험으로 취급해야 할 영향을 미치는 규정 준수 요구 사항

내부 패키지 레지스트리, 민감한 데이터 위치 및 대상, 민감한 원격 대상, 보호된 IaC 범위 항목에는 Claude Code v2.1.195 이상이 필요합니다. 이전 버전은 여전히 이를 일반 컨텍스트로 읽지만 이를 대상으로 하는 기본 제공 규칙이 없습니다.

유용한 시작 템플릿: 괄호로 묶인 필드를 채우고 적용되지 않는 줄을 제거합니다.

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Organization: {COMPANY_NAME}. Primary use: {PRIMARY_USE_CASE, e.g. software development, infrastructure automation}",
      "Source control: {SOURCE_CONTROL, e.g. GitHub org github.example.com/acme-corp}",
      "Cloud provider(s): {CLOUD_PROVIDERS, e.g. AWS, GCP, Azure}",
      "Trusted cloud buckets: {TRUSTED_BUCKETS, e.g. s3://acme-builds, gs://acme-datasets}",
      "Trusted internal domains: {TRUSTED_DOMAINS, e.g. *.internal.example.com, api.example.com}",
      "Key internal services: {SERVICES, e.g. Jenkins at ci.example.com, Artifactory at artifacts.example.com}",
      "Additional context: {EXTRA, e.g. regulated industry, multi-tenant infrastructure, compliance requirements}"
    ]
  }
}
```

더 구체적인 컨텍스트를 제공할수록 분류기가 일상적인 내부 작업과 데이터 유출 시도를 더 잘 구분할 수 있습니다.

모든 것을 한 번에 채울 필요는 없습니다. 합리적인 롤아웃: 기본값으로 시작하여 소스 제어 조직과 주요 내부 서비스를 추가합니다. 이는 자신의 저장소로 푸시하는 것과 같은 가장 일반적인 거짓 양성을 해결합니다. 다음으로 신뢰할 수 있는 도메인과 클라우드 버킷을 추가합니다. 블록이 발생할 때 나머지를 채웁니다.

<h2 id="generate-environment-entries">
  `/auto-mode-setup`으로 환경 항목 생성
</h2>

`/auto-mode-setup`을 실행하여 Claude Code가 프로젝트와 최근 세션에서 `autoMode.environment` 항목을 작성하도록 하고, 때로는 [규칙 항목](#override-the-block-and-allow-rules)도 작성하도록 할 수 있습니다. 초안을 수락하면 Claude Code가 이를 `~/.claude/settings.json`에 작성합니다.

<Note>
  `/auto-mode-setup`은 Pro, Max 또는 Team 플랜이 필요하며 Claude Code v2.1.228 이상이어야 합니다. 기본 Windows에서는 v2.1.233 이상이 필요합니다. [웹의 Claude Code](/docs/ko/claude-code-on-the-web)에서는 실행할 수 없습니다. 또한 [기능 플래그 가져오기](/docs/ko/env-vars#features-that-need-feature-flag-fetching)가 필요하므로 플래그 가져오기를 비활성화한 세션에서는 실행할 수 없습니다.
</Note>

<h3 id="what-auto-mode-setup-reads">
  `/auto-mode-setup`이 읽는 항목
</h3>

`~/.claude/settings.json`에 이미 `autoMode` 항목이 있으면 Claude Code는 먼저 환경 목록에 추가할지 아니면 바꿀지 묻고, 어느 쪽이든 작성한 규칙을 유지합니다. 그 후 Claude Code는 이 프로젝트를 어떻게 사용하는지 묻고 스캔하기 전에 두 가지 선택적 스캔을 제공합니다. 스캔에서 Claude Code는 항상 다음 소스를 읽습니다:

* 이 프로젝트의 `CLAUDE.md`, `README.md`, 설정 파일 및 git 원격
* `autoMode` 및 `permissions.allow` 설정
* 이 프로젝트의 최근 세션에서 Claude가 실행한 명령의 호스트, 버킷 및 명령 이름(메시지는 절대 아님)

두 가지 선택적 스캔은 각각 하나의 소스를 추가합니다:

* 셸 히스토리의 각 명령의 첫 번째 단어
* 홈 디렉토리 아래의 원격 호스트 및 저장소 이름

<h3 id="review-and-save-the-draft">
  초안 검토 및 저장
</h3>

Claude Code는 백그라운드에서 스캔한 후 초안을 표시합니다. 전체적으로 수락하거나 버릴 수 있으므로 나중에 `~/.claude/settings.json`을 편집하여 개별 항목을 조정합니다. 수락하면 Claude Code는 초안을 작성하고 이미 있는 설정과 조정합니다:

* Claude Code는 초안이 변경하지 않은 기본 제공 항목을 명시하므로 `"$defaults"` 없이 `environment` 목록을 작성합니다
* Claude Code는 초안이 항목을 추가하는 `allow`, `soft_deny` 및 `hard_deny` 목록 각각에 `"$defaults"`를 포함합니다. 단, 이미 `allow` 목록을 `"$defaults"` 없이 작성한 경우는 제외되므로 [기본 제공 규칙](#override-the-block-and-allow-rules)을 바꾸지 않은 경우 계속 적용됩니다
* 저장 후 Claude Code는 `~/.claude/settings.json`의 `permissions.allow` 규칙 중 자동 모드가 무시하는 규칙(예: `Bash(*)`) 또는 파괴적인 명령을 자동으로 승인하는 규칙을 제거할 것을 제안합니다

그 후 `claude auto-mode config`를 실행하여 [유효한 결과를 확인](#inspect-the-defaults-and-your-effective-config)합니다.

<h3 id="turn-off-auto-mode-setup">
  `/auto-mode-setup` 비활성화
</h3>

자동 모드가 여러 작업을 차단했는데도 여전히 `autoMode.environment` 항목이 없으면 Claude Code는 턴 끝에 "자동 모드에 환경을 알려주시겠습니까?"라는 제목의 대화 상자를 표시하고 `/auto-mode-setup`을 실행할 것을 제안합니다. 명령은 유지하면서 제안을 중지하려면 해당 대화 상자에서 **다시 표시 안 함**을 선택합니다.

명령과 제안을 모두 비활성화하려면 이 [`skillOverrides`](/docs/ko/skills#override-skill-visibility-from-settings) 항목을 `~/.claude/settings.json`에 추가합니다:

```json theme={null}
{
  "skillOverrides": {
    "auto-mode-setup": "off"
  }
}
```

`/auto-mode-setup`은 [번들 스킬](/docs/ko/skills#bundled-skills)이 아닌 기본 제공 명령이므로 이 `skillOverrides` 항목은 여전히 적용되지만 [`disableBundledSkills`](/docs/ko/settings-reference#disablebundledskills)는 이를 비활성화하지 않습니다.

<h2 id="override-the-block-and-allow-rules">
  차단 및 허용 규칙 재정의
</h2>

세 개의 추가 필드를 사용하여 분류기의 기본 제공 규칙 목록을 바꿀 수 있습니다:

* `autoMode.hard_deny`: 무조건적인 보안 경계
* `autoMode.soft_deny`: 사용자 의도로 해제할 수 있는 파괴적인 작업
* `autoMode.allow`: 소프트 블록 규칙의 예외

각각은 자연어 규칙으로 읽히는 산문 설명의 배열입니다. 분류기 이전에 실행되는 도구 패턴 기반의 하드 블록의 경우 [`permissions.deny`](/docs/ko/permissions)를 사용하세요.

분류기 내에서 우선순위는 네 가지 계층으로 작동합니다:

* `hard_deny` 규칙은 무조건적으로 차단합니다. 사용자 의도와 `allow` 예외는 적용되지 않습니다.
* `soft_deny` 규칙이 다음으로 차단합니다. 사용자 의도와 `allow` 예외가 이를 재정의할 수 있습니다.
* `allow` 규칙이 일치하는 `soft_deny` 규칙을 예외로 재정의합니다.
* 명시적 사용자 의도가 나머지 소프트 블록을 재정의합니다: 사용자의 메시지가 Claude가 수행하려는 정확한 작업을 직접적이고 구체적으로 설명하면, `soft_deny` 규칙이 일치하더라도 분류기가 이를 허용합니다.

일반적인 요청은 명시적 의도로 계산되지 않습니다. Claude에게 "저장소를 정리해 달라"고 요청하는 것은 강제 푸시를 승인하지 않지만, "이 브랜치를 강제 푸시해 달라"고 요청하는 것은 승인합니다.

느슨하게 하려면, 분류기가 기본 예외가 다루지 않는 일상적인 패턴을 반복적으로 플래그할 때 `allow`에 추가하세요. 더 엄격하게 하려면, 기본값이 놓친 환경에 특정한 파괴적 위험에 대해 `soft_deny`에 추가하거나, 절대 넘어서는 안 되는 보안 경계에 대해 `hard_deny`에 추가하세요.

기본 제공 규칙을 유지하면서 자신의 규칙을 추가하려면 배열에 리터럴 문자열 `"$defaults"`를 포함하세요. 기본 규칙이 해당 위치에 삽입되므로, 사용자 정의 규칙이 앞이나 뒤에 올 수 있으며, 릴리스 전반에 걸쳐 기본 제공 목록이 변경되면서 업데이트를 계속 상속받습니다.

다음 예제는 네 개의 목록 모두에서 기본값을 유지하고 각각에 조직별 규칙을 추가합니다.

```json theme={null}
{
  "autoMode": {
    "environment": [
      "$defaults",
      "Source control: github.example.com/acme-corp and all repos under it"
    ],
    "allow": [
      "$defaults",
      "Deploying to the staging namespace is allowed: staging is isolated from production and resets nightly",
      "Writing to s3://acme-scratch/ is allowed: ephemeral bucket with a 7-day lifecycle policy"
    ],
    "soft_deny": [
      "$defaults",
      "Never run database migrations outside the migrations CLI, even against dev databases",
      "Never modify files under infra/terraform/prod/: production infrastructure changes go through the review workflow"
    ],
    "hard_deny": [
      "$defaults",
      "Never send repository contents to third-party code-review APIs"
    ]
  }
}
```

<Danger>
  `environment`, `allow`, `soft_deny` 또는 `hard_deny` 중 하나를 `"$defaults"` 없이 설정하면 해당 섹션의 전체 기본 목록이 바뀝니다. `"$defaults"`가 없는 배열을 설정하면 해당 섹션의 기본 제공 규칙을 버립니다:

  * `soft_deny`: 강제 푸시, `curl | bash`, 프로덕션 배포 및 자동 모드 우회를 포함한 모든 기본 제공 소프트 블록 규칙
  * `hard_deny`: 기본 제공 데이터 유출 규칙
</Danger>

각 섹션은 독립적으로 평가되므로, `environment`만 설정하면 기본 `allow`, `soft_deny` 및 `hard_deny` 목록은 그대로 유지됩니다.

`"$defaults"`를 생략하는 것은 목록의 전체 소유권을 가질 의도가 있을 때만 하세요. 이 경우 `claude auto-mode defaults`를 실행하여 기본 제공 규칙을 인쇄하고, 설정 파일에 복사한 다음, 각 규칙을 자신의 파이프라인 및 위험 허용도와 비교하여 검토하세요.

<h2 id="edit-rules-from-permissions">
  `/permissions`에서 규칙 편집
</h2>

설정 파일을 열지 않고 분류기 규칙을 보고 편집하려면 [`/permissions`](/docs/ko/permissions#manage-permissions)를 실행하고 **Auto mode** 탭을 선택합니다. 이 탭은 Claude Code v2.1.246 이상이 필요하며, [자동 모드를 사용할 수 있을 때](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)만 세션에 나타납니다.

이 탭은 [분류기가 구성을 읽는 각 범위](#where-the-classifier-reads-configuration)의 `allow`, `soft_deny`, `hard_deny`, `environment` 항목을 나열하고, 각 섹션에 대해 기본 제공 규칙이 적용되는지 여부를 표시합니다. Claude Code는 [관리되는 설정](/docs/ko/server-managed-settings) 또는 `--settings` 플래그의 항목을 읽기 전용으로 표시하고, 탭에서 수행한 모든 변경 사항을 `~/.claude/settings.json`에 저장합니다. 탭에서 다음을 수행할 수 있습니다:

* `allow`, `soft_deny`, `hard_deny` 섹션의 규칙을 추가, 편집 또는 삭제합니다. 섹션에 첫 번째 규칙을 추가하면 Claude Code도 `"$defaults"`를 삽입하여 [기본 제공 규칙](#override-the-block-and-allow-rules)이 계속 적용되도록 합니다.
* `allow`, `soft_deny` 또는 `hard_deny`의 기본 제공 규칙을 끄거나 다시 켭니다. Claude Code는 해당 섹션의 목록에서 `"$defaults"`를 추가하거나 제거하여 선택을 기록하므로, 기본 제공 규칙을 끄려면 섹션에 자신의 규칙이 최소 하나 필요합니다.
* `environment` 항목을 편집기에서 하나의 문서로 편집합니다. 아직 `environment` 항목을 구성하지 않았다면 Claude Code는 먼저 기본 제공 환경을 바꿀지 여부를 묻고, 전체 기본 제공 텍스트에서 편집기를 엽니다. 저장하면 Claude Code는 `autoMode.environment` 배열을 문서로 바꿉니다. [기본 제공 항목을 유지](#define-trusted-infrastructure)하려면 `"$defaults"` 줄을 포함합니다.

<h2 id="route-all-shell-commands-through-the-classifier">
  모든 셸 명령을 분류기를 통해 라우팅합니다
</h2>

기본적으로 좁은 Bash 및 PowerShell 허용 규칙(예: `Bash(npm test)`)은 자동 모드에서 계속 적용됩니다. Claude Code는 명령이 [명령별 허용 도메인](/docs/ko/sandboxing#per-command-allowed-domains-in-auto-mode)을 포함하지 않는 한 분류기가 실행되기 전에 이를 해결합니다. Claude Code는 `Bash(*)` 또는 와일드카드 인터프리터와 같은 임의의 코드 실행을 허용하는 광범위한 규칙만 일시 중단하며, [`Monitor`](/docs/ko/tools-reference#monitor-tool)를 명명하는 모든 규칙과 함께 일시 중단합니다. Monitor 명령은 셸을 통해 실행되기 때문입니다. 이는 좁은 규칙이 여전히 분류기가 보지 못하는 파괴적인 인수(예: 스크립트 경로 또는 규칙의 접두사가 예상하지 못한 플래그)를 통과시킬 수 있음을 의미합니다.

`autoMode.classifyAllShell`을 `true`로 설정하여 자동 모드가 활성화되어 있는 동안 모든 Bash 및 PowerShell 허용 규칙을 일시 중단하면, 분류기가 허용 목록에 관계없이 모든 셸 명령을 평가합니다.

```json theme={null}
{
  "autoMode": {
    "classifyAllShell": true
  }
}
```

이는 지연 시간을 범위로 교환합니다. 허용 규칙이 즉시 승인했을 명령은 이제 분류기 결정을 기다리며, 각 셸 명령은 분류기 호출로 계산됩니다.

이 설정은 자동 모드가 활성화되어 있는 동안에만 적용되며, 다른 권한 모드에서는 허용 규칙이 정상적으로 작동합니다.

<Note>
  `autoMode.classifyAllShell`은 Claude Code v2.1.193 이상이 필요합니다. 이전 버전은 키를 무시하고 좁은 셸 허용 규칙을 자동 모드로 계속 전달합니다.
</Note>

<h2 id="inspect-the-defaults-and-your-effective-config">
  기본값 및 유효한 구성 검사
</h2>

`claude auto-mode` 하위 명령어는 구성을 검사, 검증 및 재설정하는 데 도움이 됩니다.

기본 제공 `environment`, `allow`, `soft_deny` 및 `hard_deny` 규칙을 JSON으로 출력합니다:

```bash theme={null}
claude auto-mode defaults
```

`jq`를 통해 파이핑하지 않고 한 규칙의 전체 표현을 읽으려면 `--label`을 규칙 레이블의 시작 부분(예: `claude auto-mode defaults --label 'Git Destructive'`)과 함께 전달합니다. 일치는 각 규칙의 레이블에 대한 대소문자를 구분하지 않는 접두사이며, 일치하는 항목이 없는 섹션은 빈 목록으로 출력됩니다. Claude Code v2.1.208 이상이 필요합니다.

분류기가 실제로 사용하는 것을 JSON으로 출력합니다. 설정된 곳에서는 설정을 적용하고 그 외에는 기본값을 사용합니다:

```bash theme={null}
claude auto-mode config
```

`defaults`와 `config` 모두 네 개의 규칙 목록을 단일 JSON 객체로 출력하며, 각 규칙은 산문 문자열입니다. 다음은 축약된 예입니다:

```json theme={null}
{
  "allow": [
    ...
    "Test Artifacts: Hardcoded test API keys, placeholder credentials in examples, or hardcoding test cases. Placeholder means authored as a placeholder — a file or value copied from a real secret or sensitive path is never a test artifact (see Sensitive-Source Provenance).",
    ...
  ],
  "soft_deny": [
    "Git Destructive [named+specifics — **must name:** the destructive operation and its target]: Force pushing (`git push --force`), deleting remote branches, tags, or releases, or rewriting remote history. Also `git commit --amend` when the commit being rewritten is not the agent's own unpushed work: either no prior `git commit` is visible (HEAD pre-dates the session), or a `git push` of the current branch is visible after the most recent commit (it has been pushed). Clears when the user asked to amend/reword/fixup, or when it is a message-only reword (`--amend -m …`, nothing newly staged) of a commit the agent visibly created this session.",
    ...
  ],
  "hard_deny": [...],
  "environment": [
    ...
    "**Trusted repo**: The git repository the agent started in (its working directory) and its configured remote(s). When the repo's public/private visibility is given — by the Repository visibility entry or the user's own message — use it to scope what is OK to commit or push there: confidential material is fine in a private repo; in a public one, only that repo's own work is — and content ported, repointed, or first read from outside this session's repo is not its own work, whoever directed the port. Visibility scopes confidential material only: secrets and sensitive data (personal & entrusted) are never cleared into any repo by its visibility (see Definitions).",
    ...
  ]
}
```

사용자 정의 `allow`, `soft_deny` 및 `hard_deny` 규칙에 대한 AI 피드백을 받습니다:

```bash theme={null}
claude auto-mode critique
```

설정을 저장한 후 `claude auto-mode config`를 실행하여 유효한 규칙이 예상한 것인지 확인합니다. `"$defaults"`는 제자리에 확장됩니다. 사용자 정의 규칙을 작성한 경우 `claude auto-mode critique`는 이를 검토하고 모호하거나, 중복되거나, 거짓 양성을 유발할 가능성이 있는 항목에 플래그를 지정합니다.

사용자 정의를 버리고 기본 제공 기본값으로 돌아가려면 reset 하위 명령어를 실행합니다. Claude Code v2.1.212 이상이 필요하며 사용자 설정 파일에서 `autoMode` 섹션을 제거합니다:

```bash theme={null}
claude auto-mode reset
```

이 명령어는 제거할 내용을 요약하고 쓰기 전에 `Reset auto mode configuration to defaults?`를 묻습니다. `--yes`를 전달하여 확인을 건너뜁니다. Reset은 `~/.claude/settings.json`만 변경합니다: [관리되는 설정](/docs/ko/server-managed-settings)이나 `--settings` 플래그의 `autoMode` 규칙은 여전히 적용됩니다.

<h2 id="review-denials">
  거부 검토
</h2>

자동 모드 분류기가 거부한 작업을 검토하고 재시도하려면 `/permissions`를 열고 **최근 거부됨** 탭을 선택합니다. 여기서 Claude Code는 각 거부를 기록합니다. 거부된 작업에서 `r`을 누르면 재시도 대상으로 표시됩니다. 대화 상자를 종료하면 Claude Code는 모델에 해당 도구 호출을 재시도할 수 있음을 알리는 메시지를 보내고 대화를 재개합니다.

분류기가 [작업의 안전성에 대해 판단을 내릴 수 없을 때](/docs/ko/errors#auto-mode-cannot-determine-the-safety-of-an-action), 자동 모드와 별개의 안전 검사가 분류기의 요청을 거부했거나 응답이 파싱되지 않았기 때문에 Claude Code는 **최근 거부됨** 아래에 기록하지 않고 작업을 거부합니다. 연결된 오류 항목은 Claude에게 전달되는 내용과 필요한 경우 작업을 실행하는 방법을 다룹니다.

<h3 id="fix-a-denial-with-an-allow-rule-an-environment-entry-or-a-retry">
  허용 규칙, 환경 항목 또는 재시도로 거부 해결
</h3>

분류기가 차단한 내용을 확인하려면 대화에서 도구 호출을 찾습니다. 호출이 단축되거나 `Ran 3 shell commands`와 같은 요약 줄로 접혀 있으면 `Ctrl+O`를 눌러 [트랜스크립트 뷰어](/docs/ko/interactive-mode#transcript-viewer)를 열어 확장합니다.

화면의 거부를 보고하는 다른 두 위치는 명령 또는 URL을 생략합니다. 입력 상자 근처의 알림(예: `bash denied by auto mode · [Data Exfiltration] · /permissions`)은 도구와 이유를 제공하고, **최근 거부됨** 탭은 Claude가 작성한 설명으로 셸 명령을 나열합니다. 이러한 거부의 정확한 입력을 프로그래밍 방식으로 캡처하려면 [`PermissionDenied` 훅](/docs/ko/hooks#permissiondenied)을 추가합니다. 이 훅은 `tool_input`으로 입력을 받습니다.

호출 아래의 텍스트는 수정할 사항이 있는지 알려줍니다. 분류기 자체의 문제를 보고하는 텍스트(예: `is temporarily unavailable`인 모델 또는 분류기 오류)는 Claude Code가 분류기로부터 최종 판단 없이 호출을 차단했음을 의미합니다. [자동 모드가 작업의 안전성을 판단할 수 없음](/docs/ko/errors#auto-mode-cannot-determine-the-safety-of-an-action)을 참조하여 수행할 작업을 확인합니다. 그렇지 않으면 `Denied by auto mode classifier`로 읽히는 줄과 `[Production Deploy]` 또는 `Blocked by classifier`와 같은 이유는 분류기가 호출을 안전하지 않다고 판단했음을 의미하므로, 호출이 도달하거나 수행하려던 내용에서 수정 사항을 선택합니다.

* Claude가 작업 전체에서 필요로 하는 대상(예: 패키지 레지스트리, 내부 도메인 또는 저장소 호스트): `autoMode.environment`에 추가합니다.
* 앞으로 검토 없이 실행하려는 명령: `allow` 규칙을 추가합니다.
* 의도한 일회성 작업: 다음 메시지에서 해당 의도를 명시하고 Claude가 재시도하도록 합니다.

`/permissions` 대화 상자의 [**자동 모드** 탭](#edit-rules-from-permissions)에서 환경 항목 또는 `allow` 규칙을 추가할 수 있습니다.

대부분의 세션에서 이유는 분류기가 일치한 규칙을 `[Data Exfiltration]` 또는 `[Production Deploy]`와 같이 대괄호로 표시하며, 일부 세션은 짧은 설명을 추가하는 분류기 모델을 실행합니다. Claude Code가 분류기 모델을 선택하므로 표시되는 이유의 형식은 구성할 수 있는 것이 아닙니다.

<h3 id="fix-repeated-denials">
  반복된 거부 해결
</h3>

동일한 대상에 대한 반복된 거부는 일반적으로 분류기가 컨텍스트를 누락했음을 의미합니다. 해당 대상을 `autoMode.environment`에 추가하거나 [/auto-mode-setup 실행](#generate-environment-entries)하여 Claude Code가 항목을 작성하도록 한 다음 `claude auto-mode config`를 실행하여 변경 사항이 적용되었는지 확인합니다.

거부에 프로그래밍 방식으로 반응하려면 [`PermissionDenied` 훅](/docs/ko/hooks#permissiondenied)을 사용합니다.

<h2 id="see-also">
  참고 항목
</h2>

* [권한 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode): 자동 모드가 무엇인지, 기본적으로 차단하는 항목, 그리고 어떤 세션이 자동 모드로 시작되는지에 대한 설명
* [관리되는 설정](/docs/ko/server-managed-settings): 조직 전체에 `autoMode` 구성 배포
* [권한](/docs/ko/permissions): 분류기가 실행되기 전에 적용되는 허용, 요청 및 거부 규칙
* [모든 설정](/docs/ko/settings-reference#automode): `autoMode`를 포함한 모든 설정 키
