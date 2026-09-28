> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 코드베이스에서 취약점 스캔하기

> Claude Security 플러그인을 설치하여 Claude Code 세션에서 코드베이스의 취약점을 스캔하고 발견 사항을 검토 및 적용할 수 있는 패치로 변환합니다.

Claude Security 플러그인은 Claude Code 세션 내에서 코드베이스의 다중 에이전트 취약점 스캔을 실행합니다. Claude 에이전트 팀이 아키텍처를 매핑하고, 위협 모델을 구축하고, 취약점을 찾고, 보고서를 작성하기 전에 모든 발견 사항을 독립적으로 검토합니다. 플러그인을 사용하여 전체 저장소를 스캔하거나 [변경 사항만 스캔](#scan-only-your-changes)할 수 있습니다. 예를 들어 브랜치의 diff, 풀 요청의 diff 또는 단일 커밋과 같은 변경 사항 집합을 스캔한 후, 선택한 발견 사항을 검토하고 직접 적용할 수 있는 패치로 변환합니다.

플러그인은 세션에서 로컬로 실행되며, Claude Code에서 액세스할 수 있는 모든 모델을 사용하고, 각 스캔은 플랜의 사용량 제한에 포함됩니다. 저장소를 모니터링하는 관리형 서비스를 원하거나 [Claude Mythos 5](https://platform.claude.com/docs/en/about-claude/models/introducing-claude-fable-5-and-claude-mythos-5)에서 스캔을 실행하려면 Enterprise 플랜에서 사용 가능한 [Claude Security](https://claude.com/product/claude-security) 제품을 참조하세요. 플러그인은 관리형 제품이 도달할 수 없는 코드에 접근할 수 있습니다. 예를 들어 GitLab 또는 Bitbucket에서 호스팅되는 저장소나 인바운드 연결을 허용하지 않는 네트워크의 저장소입니다.

플러그인은 Claude Code에 이미 있는 검토 도구와도 다릅니다. [보안 지침 플러그인](/docs/ko/security-guidance)은 Claude가 코드를 작성할 때 검토하고, [`/security-review`](/docs/ko/commands#all-commands)는 브랜치에 대해 한 번의 패스를 실행하며, [Code Review](/docs/ko/code-review)는 풀 요청을 검토합니다. 계층이 어떻게 쌓이는지는 [플러그인이 다른 보안 도구와 어떻게 맞는지](#how-the-plugin-fits-with-other-security-tools)를 참조하세요.

<h2 id="prerequisites">
  필수 조건
</h2>

플러그인을 실행하려면 다음이 필요합니다.

* 유료 플랜. 스캔이 에이전트를 조율하는 데 사용하는 [동적 워크플로우](/docs/ko/workflows)를 위해서입니다. Pro에서는 `/config`의 Dynamic workflows 행에서 켜세요.
* Python 3.9 이상이 `PATH`에서 `python3`로 사용 가능해야 합니다. `python3 --version`으로 확인하세요. 플러그인의 도구는 Python 표준 라이브러리만 사용하므로 아무것도 설치되지 않습니다.
* Linux, macOS 또는 Windows.
* Git. 변경 사항 스캔 및 발견 사항을 패치로 변환하기 위해 필요합니다. 이러한 작업은 다른 버전 관리 시스템을 지원하지 않습니다. 전체 스캔은 버전 관리 여부와 관계없이 모든 디렉토리에서 작동합니다.

<h2 id="install-the-plugin">
  플러그인 설치
</h2>

Claude Code 세션에서 [공식 Anthropic 마켓플레이스](/docs/ko/plugins/anthropic-marketplaces)에서 설치합니다:

```text theme={null}
/plugin install claude-security@claude-plugins-official
```

명령은 플러그인의 세부 정보를 열며, 여기서 [설치 범위](/docs/ko/plugins/install#install-a-plugin)를 선택하여 설치를 시작합니다.

설치가 실패하면 Claude Code가 보고하는 메시지에 따라 수정 방법이 달라집니다:

* `Marketplace "claude-plugins-official" not found`를 보고하면 `/plugin marketplace add anthropics/claude-plugins-official`로 마켓플레이스를 추가한 후 설치를 다시 시도하세요.
* [마켓플레이스에서 플러그인을 찾을 수 없다](/docs/ko/plugins/install#install-a-plugin)고 보고하면 플러그인 이름에 오타가 없는지 확인하세요.

설치 요약을 확인하세요. `Run /reload-plugins to activate.`를 보고하면 [재시작 없이 플러그인 변경 사항 적용](/docs/ko/plugins/cli-reference#reload-plugins)을 참조하여 현재 세션에서 플러그인을 활성화하세요.

플러그인이 활성화되면 [코드베이스 스캔 및 수정](#scan-and-fix-your-codebase)을 시작할 준비가 되었습니다.

<h3 id="uninstall-the-plugin">
  플러그인 제거
</h3>

플러그인을 제거하려면 `/plugin` 메뉴에서 제거하거나 터미널에서 `claude plugin uninstall claude-security`를 실행하세요.

<h2 id="scan-and-fix-your-codebase">
  코드베이스 스캔 및 수정
</h2>

플러그인은 `/claude-security` 명령 하나를 추가합니다. 이 명령은 세 가지 작업의 메뉴를 엽니다. 코드베이스 스캔, 변경 사항 집합 스캔, 패치 제안입니다. 일반적인 경로는 전체 스캔을 실행한 후 발견 사항을 패치로 변환합니다.

<Steps>
  <Step title="Claude Security 메뉴 열기">
    `/claude-security`를 실행하고 **Scan codebase**를 선택합니다.
  </Step>

  <Step title="스캔할 항목 선택">
    플러그인이 먼저 저장소를 읽은 후 전체 저장소 또는 집중된 영역을 제공합니다. 각 옵션의 파일 수와 상대 비용이 명시됩니다. 전체 저장소를 선택하거나 "I don't know"라고 답하면 플러그인이 저장소 크기에 맞는 합리적인 기본값을 선택합니다.
  </Step>

  <Step title="실행 확인">
    스캔은 시간이 걸릴 수 있으며, 상당한 수의 토큰을 사용할 수 있으며, 완료될 때까지 Claude Code를 열어 두어야 합니다. 확인할 때까지 아무것도 실행되지 않습니다.
  </Step>

  <Step title="보고서 읽기">
    스캔이 실행되는 동안 각 단계가 시작될 때 보고합니다. 자세한 내용은 [`/workflows`](/docs/ko/workflows)에서 확인할 수 있습니다. 결과는 저장소의 타임스탬프가 지정된 디렉토리에 저장됩니다. [스캔 결과 읽기](#read-the-scan-results)에서 설명합니다.
  </Step>

  <Step title="발견 사항을 패치로 변환">
    `/claude-security`를 다시 실행하고 **Suggest patches**를 선택한 후 해결할 발견 사항을 선택합니다. 검토된 패치는 보고서의 `patches/` 폴더에 저장됩니다. [발견 사항 수정](#fix-findings)에서 각 패치가 어떻게 구축되고 검토되는지 설명합니다.
  </Step>

  <Step title="수락한 패치 적용">
    셸에서 `git apply`로 각 패치를 자신의 풀 요청에 적용합니다. 패치는 자동으로 적용되지 않습니다.
  </Step>
</Steps>

메뉴에서 시작할 필요가 없습니다. 명령에 대한 인수로 작업을 직접 요청할 수 있습니다. 예를 들어 `/claude-security scan my branch` 또는 일반 언어로 "scan commit abc1234"와 같이 요청할 수 있습니다. 플러그인은 [자동 모드](/docs/ko/permission-modes)에서 가장 잘 작동합니다. 이 모드는 스캔의 에이전트가 각 단계에서 권한 프롬프트 없이 진행할 수 있게 합니다.

<h3 id="scan-only-your-changes">
  변경 사항만 스캔
</h3>

브랜치에 기본이 없는 커밋이 있으면 `/claude-security` 메뉴는 해당 diff만 스캔하도록 제공합니다. 병합하기 전에 브랜치를 확인할 수 있습니다. 열린 풀 요청 중 하나를 스캔하거나 "scan commit abc1234"와 같이 요청하여 단일 커밋을 스캔할 수도 있습니다. 커밋된 변경 사항만 스캔됩니다. 진행 중인 편집을 먼저 커밋하거나 stash하거나 작업 트리를 읽는 전체 스캔을 실행하세요.

변경 사항 스캔에는 git 저장소가 필요합니다. 버전 관리되지 않은 디렉토리의 전체 스캔은 여전히 작동합니다. 열린 풀 요청을 찾는 것은 네트워크에 도달하는 유일한 단계이며, 세션이 이미 GitHub CLI를 실행할 권한이 있고 `gh`가 로그인되어 있을 때만 제공됩니다.

<h3 id="scope-large-repositories">
  대규모 저장소 범위 지정
</h3>

대규모 저장소에서는 전체 트리 대신 한 번에 한 영역씩 스캔합니다. 플러그인이 제공하는 집중된 범위 중 하나를 선택합니다. 예를 들어 API 계층 또는 인증 코드를 선택하면 실행이 선택한 항목에 맞게 크기가 조정됩니다. 보고서의 범위 섹션에서 검사된 항목과 검사되지 않은 항목을 명시합니다. 언제든지 다른 영역에서 다른 스캔을 실행하세요.

<h3 id="read-the-scan-results">
  스캔 결과 읽기
</h3>

모든 스캔은 저장소의 타임스탬프가 지정된 `CLAUDE-SECURITY-<timestamp>/` 디렉토리에 결과를 작성합니다.

* **`CLAUDE-SECURITY-RESULTS.md`**: 보고서. 각 발견 사항의 ID(예: `F1`), 영향, 악용 시나리오, 심각도, 신뢰도 및 권장 사항 포함
* **`CLAUDE-SECURITY-RESULTS.jsonl`**: 동일한 발견 사항을 기계 판독 가능한 형식으로 표현. 줄당 하나의 JSON 객체
* **`CLAUDE-SECURITY-RESULTS.sarif`**: 동일한 발견 사항을 [SARIF 2.1.0](https://docs.oasis-open.org/sarif/sarif/v2.1.0/sarif-v2.1.0.html) 로그로 표현. GitHub 코드 스캔 및 표준을 읽는 다른 도구용입니다. 스캔은 발견 사항을 [CWE](https://cwe.mitre.org/) 약점 범주로 분류합니다.
* **`CLAUDE-SECURITY-REVISION-<commit>.json`**: 수정 스탬프. 스캔된 커밋, 노력 수준, 커밋되지 않은 변경 사항이 스캔된 트리의 일부인지 여부, 실행이 얼마나 철저히 검증되었는지를 기록합니다. 이렇게 하면 보고서가 항상 설명하는 코드에 연결됩니다. 버전 관리 외부의 스캔은 커밋 대신 `UNVERSIONED`를 스탬프합니다.

해당 디렉토리는 스캔이 체크아웃에 수행하는 유일한 변경 사항이며, 자체 `.gitignore`를 포함하므로 실수로 `git add`를 하더라도 보고서가 커밋으로 스윕되지 않습니다. 감사 추적을 위해 보고서를 기록에 유지하려면 해당 `.gitignore` 파일 하나를 삭제하고 다른 디렉토리처럼 디렉토리를 커밋하세요.

발견 사항은 독립적인 검증자 에이전트가 분석한 후에만 보고서에 나타나므로 보고서가 짧고 읽을 가치가 있습니다. 스캔은 비결정적입니다. 동일한 코드의 두 스캔은 다른 발견 사항을 표시할 수 있습니다. 정기적으로 스캔을 실행하고 수정 스탬프를 사용하여 각 보고서를 다룬 정확한 코드 및 설정에 귀속시키세요.

<h2 id="fix-findings">
  발견 사항 수정
</h2>

`/claude-security` 메뉴에서 **Suggest patches**를 선택하거나 "fix finding F3"과 같이 일반 언어로 요청하여 수정 흐름을 시작합니다. 그런 다음 보고서에서 해결할 발견 사항을 선택합니다. 패치는 커밋된 코드에 대해 구축되며, 보고서는 여전히 현재 코드를 설명해야 합니다. 코드가 변경된 발견 사항은 메모와 함께 건너뛰어지며, 플러그인은 오래된 보고서에서 패치하는 대신 새로운 스캔을 제공합니다. 각 패치는 저장소의 스크래치 복사본에서 작성되므로 패치를 직접 적용할 때까지 소스 파일은 손상되지 않습니다.

전달 전에 각 패치는 작성한 에이전트와 독립적인 에이전트에 의해 검토됩니다. 이 에이전트는 코드에 테스트가 있을 때 변경에 대해 프로젝트의 테스트를 실행하고 자체 용어로 diff를 읽어 도입할 수 있는 새로운 항목을 찾습니다. 패치는 해당 검토가 변경이 하나의 발견 사항을 해결하고, 새로운 취약점을 도입하지 않으며, 그 외에는 동작을 변경하지 않음을 보증할 수 있을 때만 작성됩니다. 세 가지를 모두 보증할 수 없으면 패치 대신 이유를 설명하는 짧은 메모를 받습니다.

<h3 id="patches-are-never-applied-automatically">
  패치는 자동으로 적용되지 않습니다
</h3>

패치 적용은 항상 사용자의 결정입니다. 패치는 보고서의 `patches/` 폴더에 저장됩니다. 발견 사항당 하나의 `F<n>.patch`와 변경 사항을 설명하는 메모가 있습니다. 셸에서 하나를 적용하거나 Claude에 적용하고 풀 요청을 열도록 요청하세요.

```bash theme={null}
git apply CLAUDE-SECURITY-<timestamp>/patches/F1.patch
```

패치된 코드에 테스트가 없으면 패치의 메모에 그렇게 표시되므로 검토가 테스트 통과 없이 실행되었음을 알 수 있습니다. 각 패치를 자신의 풀 요청에 적용하여 자체적으로 검토하고 테스트할 수 있도록 하세요.

<h2 id="how-the-plugin-fits-with-other-security-tools">
  플러그인이 다른 보안 도구와 어떻게 맞는지
</h2>

Claude Security 플러그인은 [보안 지침 플러그인](/docs/ko/security-guidance), [`/security-review`](/docs/ko/commands#all-commands), [Code Review](/docs/ko/code-review), 관리형 [Claude Security](https://claude.com/product/claude-security) 제품 및 기존 스캐너와 함께 심층 방어 스택의 온디맨드 심층 스캔 계층입니다.

| 단계          | 도구                                                                           | 범위                                            |
| :---------- | :--------------------------------------------------------------------------- | :-------------------------------------------- |
| 세션 내        | [보안 지침 플러그인](/docs/ko/security-guidance)                                          | Claude가 작성하는 코드의 일반적인 취약점. 동일한 세션에서 수정됨       |
| 온디맨드, 단일 패스 | [`/security-review`](/docs/ko/commands#all-commands)                              | 현재 브랜치에 대한 일회성 보안 패스                          |
| 온디맨드, 심층 스캔 | Claude Security 플러그인                                                         | 저장소 또는 diff의 다중 에이전트 스캔. 독립적으로 검토된 발견 사항 및 패치 |
| 풀 요청        | [Code Review](/docs/ko/code-review), Team 및 Enterprise 플랜                         | 전체 코드베이스 컨텍스트를 포함한 다중 에이전트 정확성 및 보안 검토        |
| 관리형         | [Claude Security](https://claude.com/product/claude-security), Enterprise 플랜 | 연결된 저장소를 모니터링하는 호스팅된 스캔                       |
| CI          | 기존 정적 분석 및 종속성 스캐너                                                           | 언어별 규칙, 공급망 확인 및 정책 적용                        |

플러그인은 기존 소스 코드 보안 도구를 대체하지 않습니다. 정적 분석, 종속성 스캔 및 코드 검토와 함께 실행하세요. 인간 보안 연구원이 하는 방식으로 코드를 추론합니다. 이는 이러한 도구가 제공하는 결정론적 검사를 보완합니다.

<h2 id="troubleshooting">
  문제 해결
</h2>

**`/claude-security` 메뉴가 Python 경고와 함께 열립니다.** 플러그인에는 `PATH`에서 `python3` 3.9 이상이 필요합니다. 전혀 `python3`를 찾을 수 없으면 메뉴는 Claude Security가 설치될 때까지 작동하지 않는다고 경고합니다. `PATH`의 첫 번째 `python3`가 더 오래되면 경고는 찾은 버전을 이름으로 지정합니다. Python 3을 설치하거나 더 새로운 `python3`를 `PATH`의 첫 번째에 놓은 후 새 세션을 시작하세요.

**Fable 모델에서 스캔할 때 "safeguards flagged this message" 알림이 표시될 수 있습니다.** 메시지는 모델을 이름으로 지정합니다. 예를 들어 "Fable 5.1's safeguards flagged this message"입니다. Fable의 사이버 보안 안전 분류자는 특정 요청을 플래그하며, Claude Code는 [자동 모델 폴백](/docs/ko/model-config#automatic-model-fallback)을 통해 플래그된 요청을 Opus 모델에서 다시 실행합니다. 이는 예상된 동작이며, 스캔은 여전히 성공적으로 완료되어야 합니다.

<h2 id="related-resources">
  관련 리소스
</h2>

이 페이지가 다루는 부분을 더 깊이 있게 알아보려면:

* [보안 지침 플러그인](/docs/ko/security-guidance): Claude가 코드를 작성할 때 동일한 세션에서 문제를 포착합니다.
* [Code Review](/docs/ko/code-review): PR 시간 다중 에이전트 검토를 설정합니다.
* [Claude Security](https://claude.com/product/claude-security): 연결된 저장소를 모니터링하는 관리형 서비스
* [Claude Code 보안](/docs/ko/security): Claude Code가 신뢰, 권한 및 보안 조치에 어떻게 접근하는지
* [플러그인 설치 및 관리](/docs/ko/plugins/install): 공식 마켓플레이스에서 다른 플러그인을 찾아 설치합니다.
