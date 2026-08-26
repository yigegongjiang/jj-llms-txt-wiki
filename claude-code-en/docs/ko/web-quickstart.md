> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 웹에서 Claude Code 시작하기

> 브라우저나 휴대폰에서 클라우드에서 Claude Code를 실행합니다. GitHub 저장소를 연결하고, 작업을 제출하고, 로컬 설정 없이 PR을 검토합니다.

<Note>
  웹의 Claude Code는 Pro, Max, Team 사용자와 프리미엄 시트 또는 Chat + Claude Code 시트가 있는 Enterprise 사용자를 위한 연구 미리보기 상태입니다.
</Note>

웹의 Claude Code는 사용자의 머신 대신 Anthropic 관리 클라우드 인프라에서 실행됩니다. [claude.ai/code](https://claude.ai/code)에서 브라우저 또는 Claude 모바일 앱을 통해 작업을 제출합니다.

[시작하기](#connect-github-and-create-an-environment)를 위해 GitHub 저장소가 필요합니다. Claude는 이를 격리된 가상 머신으로 복제하고, 변경 사항을 만들고, 검토할 수 있도록 브랜치를 푸시합니다. 세션은 기기 간에 지속되므로, 노트북에서 시작한 작업을 나중에 휴대폰에서 검토할 수 있습니다.

웹의 Claude Code는 다음에 적합합니다:

* **병렬 작업**: 여러 개의 독립적인 작업을 동시에 실행하며, 각각 자신의 세션과 브랜치에서 실행되고, 여러 worktrees를 관리할 필요가 없습니다
* **로컬에 없는 저장소**: Claude는 매 세션마다 저장소를 새로 복제하므로, 체크아웃할 필요가 없습니다
* **자주 조정할 필요가 없는 작업**: 잘 정의된 작업을 제출하고, 다른 작업을 하고, Claude가 완료되면 결과를 검토합니다
* **코드 질문 및 탐색**: 로컬 체크아웃 없이 코드베이스를 이해하거나 기능이 어떻게 구현되는지 추적합니다

로컬 구성, 도구 또는 환경이 필요한 작업의 경우, Claude Code를 로컬에서 실행하거나 [Remote Control](/docs/ko/remote-control)을 사용하는 것이 더 적합합니다.

<h2 id="how-sessions-run">
  세션이 실행되는 방식
</h2>

작업을 제출할 때:

1. **복제 및 준비**: 저장소가 Anthropic 관리 VM으로 복제되고, 구성된 경우 [설정 스크립트](/docs/ko/claude-code-on-the-web#setup-scripts)가 실행됩니다.
2. **네트워크 구성**: 인터넷 접근은 환경의 [접근 수준](/docs/ko/claude-code-on-the-web#access-levels)에 따라 설정됩니다.
3. **작업**: Claude는 코드를 분석하고, 변경 사항을 만들고, 테스트를 실행하고, 작업을 확인합니다. 전체 과정을 지켜보고 조정할 수 있거나, 물러나 있다가 완료되면 돌아올 수 있습니다.
4. **브랜치 푸시**: Claude가 중지점에 도달하면, 브랜치를 GitHub로 푸시합니다. 차이를 검토하고, 인라인 댓글을 남기고, PR을 생성하거나, 계속 진행하도록 다른 메시지를 보냅니다.

브랜치가 푸시될 때 세션이 닫히지 않습니다. PR 생성 및 추가 편집은 모두 동일한 대화 내에서 발생합니다.

<h2 id="compare-ways-to-run-claude-code">
  Claude Code를 실행하는 방법 비교
</h2>

Claude Code는 모든 곳에서 동일하게 작동합니다. 변경되는 것은 코드가 실행되는 위치와 로컬 구성을 사용할 수 있는지 여부입니다. Desktop 앱은 로컬 및 클라우드 세션을 모두 제공하므로, 아래의 답변은 선택한 것에 따라 달라집니다:

|                                   | 웹에서                                                                                                | Remote Control     | Terminal CLI | Desktop 앱              |
| :-------------------------------- | :------------------------------------------------------------------------------------------------- | :----------------- | :----------- | :--------------------- |
| **코드 실행 위치**                      | Anthropic 클라우드 VM                                                                                  | 사용자의 머신            | 사용자의 머신      | 사용자의 머신 또는 클라우드 VM     |
| **채팅 위치**                         | claude.ai 또는 모바일 앱                                                                                 | claude.ai 또는 모바일 앱 | 터미널          | Desktop UI             |
| **로컬 구성 사용**                      | 아니오, 저장소만                                                                                          | 예                  | 예            | 로컬의 경우 예, 클라우드의 경우 아니오 |
| **GitHub 필요**                     | 예, 또는 `--cloud`를 통해 [로컬 저장소 번들](/docs/ko/claude-code-on-the-web#send-local-repositories-without-github) | 아니오                | 아니오          | 클라우드 세션의 경우만           |
| **연결 해제 시 계속 실행**                 | 예                                                                                                  | 터미널이 열려 있는 동안      | 아니오          | 세션 유형에 따라 다름           |
| **[권한 모드](/docs/ko/permission-modes)** | 편집 자동 수락, Plan, Auto                                                                               | 요청, 편집 자동 수락, Plan | 모든 모드        | 세션 유형에 따라 다름           |
| **네트워크 접근**                       | 환경별로 구성 가능                                                                                         | 머신의 네트워크           | 머신의 네트워크     | 세션 유형에 따라 다름           |

[터미널 빠른 시작](/docs/ko/quickstart), [Desktop 앱](/docs/ko/desktop) 또는 [Remote Control](/docs/ko/remote-control) 문서를 참조하여 설정합니다.

<h2 id="connect-github-and-create-an-environment">
  GitHub 연결 및 환경 생성
</h2>

설정은 일회성 프로세스입니다. 이미 GitHub CLI를 사용하는 경우, 브라우저 대신 [터미널에서 이를 수행](#connect-from-your-terminal)할 수 있습니다.

<Steps>
  <Step title="claude.ai/code 방문">
    [claude.ai/code](https://claude.ai/code)로 이동하고 Anthropic 계정으로 로그인합니다.
  </Step>

  <Step title="Claude GitHub 앱 설치">
    로그인 후, claude.ai/code는 GitHub를 연결하도록 요청합니다. 프롬프트를 따라 Claude GitHub 앱을 설치하고 저장소에 대한 접근을 허용합니다. 클라우드 세션은 기존 GitHub 저장소와 함께 작동하므로, 새 프로젝트를 시작하려면 먼저 [GitHub에서 빈 저장소를 생성](https://github.com/new)합니다.
  </Step>

  <Step title="환경 생성">
    GitHub를 연결한 후, 클라우드 환경을 생성하도록 요청받습니다. 환경은 세션 중에 Claude가 가진 네트워크 접근 권한과 새 세션이 생성될 때 실행되는 것을 제어합니다. 구성 없이 사용 가능한 것은 [설치된 도구](/docs/ko/claude-code-on-the-web#installed-tools)를 참조합니다.

    양식에는 다음 필드가 있습니다:

    * **이름**: 표시 레이블입니다. 다양한 프로젝트 또는 접근 수준을 위해 여러 환경이 있을 때 유용합니다.
    * **네트워크 접근**: 세션이 인터넷에서 도달할 수 있는 것을 제어합니다. 기본값인 `Trusted`는 npm, PyPI, RubyGems와 같은 [일반적인 패키지 레지스트리](/docs/ko/claude-code-on-the-web#default-allowed-domains)에 대한 연결을 허용하면서 일반 인터넷 접근을 차단합니다.
    * **환경 변수**: `.env` 형식의 모든 세션에서 사용 가능한 선택적 변수입니다. 따옴표가 값의 일부로 저장되므로 값을 따옴표로 감싸지 마십시오. 이 환경을 편집할 수 있는 모든 사람에게 표시됩니다.
    * **설정 스크립트**: Claude Code가 시작되기 전에 실행되는 선택적 Bash 스크립트입니다. `apt install -y gh`와 같이 클라우드 VM에 포함되지 않은 시스템 도구를 설치하는 데 사용합니다. 결과는 [캐시됨](/docs/ko/claude-code-on-the-web#environment-caching)이므로, 스크립트는 매 세션마다 다시 실행되지 않습니다. 예제 및 디버깅 팁은 [설정 스크립트](/docs/ko/claude-code-on-the-web#setup-scripts)를 참조합니다.

    첫 번째 프로젝트의 경우, 기본값을 유지하고 **환경 생성**을 클릭합니다. 나중에 [편집하거나 다양한 프로젝트를 위해 추가 환경을 생성](/docs/ko/claude-code-on-the-web#configure-your-environment)할 수 있습니다.
  </Step>
</Steps>

<h3 id="connect-from-your-terminal">
  터미널에서 연결
</h3>

이미 GitHub CLI(`gh`)를 사용하는 경우, 브라우저를 열지 않고 웹에서 Claude Code를 설정할 수 있습니다. 이는 [Claude Code CLI](/docs/ko/quickstart)가 필요합니다. `/web-setup`은 로컬 `gh` 토큰을 읽고, Claude 계정에 연결하고, 아직 없는 경우 기본 클라우드 환경을 생성합니다.

<Note>
  [Zero Data Retention](/docs/ko/zero-data-retention)이 활성화된 조직은 `/web-setup` 또는 기타 클라우드 세션 기능을 사용할 수 없습니다. GitHub CLI가 설치되지 않았거나 인증되지 않은 경우, `/web-setup`은 브라우저 온보딩 흐름을 대신 엽니다.
</Note>

<Steps>
  <Step title="GitHub CLI로 인증">
    셸에서, 아직 하지 않은 경우 GitHub CLI를 인증합니다:

    ```bash theme={null}
    gh auth login
    ```
  </Step>

  <Step title="Claude에 로그인">
    Claude Code CLI에서 `/login`을 실행하여 claude.ai 계정으로 로그인합니다. 이미 로그인한 경우 이 단계를 건너뜁니다.
  </Step>

  <Step title="/web-setup 실행">
    Claude Code CLI에서 다음을 실행합니다:

    ```text theme={null}
    /web-setup
    ```

    이는 `gh` 토큰을 Claude 계정과 동기화합니다. 아직 클라우드 환경이 없는 경우, `/web-setup`은 Trusted 네트워크 접근 및 설정 스크립트 없이 환경을 생성합니다. 나중에 [환경을 편집하거나 변수를 추가](/docs/ko/claude-code-on-the-web#configure-your-environment)할 수 있습니다. `/web-setup`이 완료되면, 터미널에서 [`--cloud`](/docs/ko/claude-code-on-the-web#from-terminal-to-web)를 사용하여 클라우드 세션을 시작하거나 [`/schedule`](/docs/ko/routines)을 사용하여 반복 작업을 설정할 수 있습니다.
  </Step>
</Steps>

<h2 id="start-a-task">
  작업 시작
</h2>

GitHub가 연결되고 환경이 생성되면, 작업을 제출할 준비가 되었습니다.

<Steps>
  <Step title="저장소 및 브랜치 선택">
    [claude.ai/code](https://claude.ai/code) 또는 Claude 모바일 앱의 Code 탭에서, 입력 상자 아래의 저장소 선택기를 클릭하고 Claude가 작업할 저장소를 선택합니다. 각 저장소는 브랜치 선택기를 표시합니다. 기본값 대신 기능 브랜치에서 Claude를 시작하도록 변경합니다. 한 세션에서 여러 저장소를 추가하여 작업할 수 있습니다.
  </Step>

  <Step title="권한 모드 선택">
    입력 옆의 모드 드롭다운은 기본값으로 **편집 자동 수락**이며, Claude는 승인을 기다리지 않고 변경 사항을 만들고 브랜치를 푸시합니다. Claude가 접근 방식을 제안하고 파일을 편집하기 전에 승인을 기다리도록 하려면 **Plan Mode**로 전환합니다. 클라우드 세션은 Manual 또는 Bypass 권한을 제공하지 않습니다. 각 권한 모드가 허용하는 사항에 대해서는 [권한 모드의 전체 목록](/docs/ko/permission-modes#available-modes)을 참조합니다.
  </Step>

  <Step title="작업 설명 및 제출">
    원하는 작업에 대한 설명을 입력하고 Enter를 누릅니다. 구체적으로 작성합니다:

    * 파일 또는 함수 이름 지정: "설정 지침이 포함된 README 추가" 또는 "`tests/test_auth.py`에서 실패한 인증 테스트 수정"이 "테스트 수정"보다 낫습니다
    * 오류 출력이 있으면 붙여넣기
    * 증상이 아닌 예상 동작을 설명합니다

    Claude는 저장소를 복제하고, 구성된 경우 설정 스크립트를 실행하고, 작업을 시작합니다. 각 작업은 자신의 세션과 자신의 브랜치를 가지므로, 하나가 완료될 때까지 기다릴 필요가 없습니다.
  </Step>
</Steps>

<h2 id="pre-fill-sessions">
  세션 미리 채우기
</h2>

[claude.ai/code](https://claude.ai/code) URL에 쿼리 매개변수를 추가하여 새 세션의 프롬프트, 저장소 및 환경을 미리 채울 수 있습니다. 이를 사용하여 문제 추적기의 버튼과 같은 통합을 구축하여 문제 설명을 프롬프트로 하여 Claude Code를 엽니다.

| 매개변수           | 설명                                                                                        |
| :------------- | :---------------------------------------------------------------------------------------- |
| `prompt`       | 입력 상자에 미리 채울 프롬프트 텍스트입니다. 별칭 `q`도 허용됩니다.                                                  |
| `prompt_url`   | 쿼리 문자열에 포함하기에 너무 긴 프롬프트 텍스트를 가져올 URL입니다. URL은 교차 출처 요청을 허용해야 합니다. `prompt`도 설정된 경우 무시됩니다. |
| `repositories` | 미리 선택할 `owner/repo` 슬러그의 쉼표로 구분된 목록입니다. 별칭 `repo`도 허용됩니다.                                 |
| `environment`  | 미리 선택할 [환경](#connect-github-and-create-an-environment)의 이름 또는 ID입니다.                      |

각 값을 URL 인코딩합니다. 아래 예제는 프롬프트와 저장소가 이미 선택된 양식을 엽니다:

```text theme={null}
https://claude.ai/code?prompt=Fix%20the%20login%20bug&repositories=acme/webapp
```

<h2 id="review-and-iterate">
  검토 및 반복
</h2>

Claude가 완료되면, 변경 사항을 검토하고, 특정 줄에 피드백을 남기고, 차이가 올바를 때까지 계속합니다.

<Steps>
  <Step title="차이 보기 열기">
    차이 표시기는 세션 전체에서 추가되고 제거된 줄을 표시합니다(예: `+42 -18`). 이를 선택하여 차이 보기를 열고, 왼쪽에 파일 목록이 있고 오른쪽에 변경 사항이 있습니다.
  </Step>

  <Step title="인라인 댓글 남기기">
    차이의 모든 줄을 선택하고, 피드백을 입력하고, Enter를 누릅니다. 댓글은 다음 메시지를 보낼 때까지 대기열에 있다가 함께 번들로 제공됩니다. Claude는 주요 지침과 함께 "`src/auth.ts:47`에서 여기서 오류를 포착하지 마십시오"를 보므로, 문제가 있는 위치를 설명할 필요가 없습니다.
  </Step>

  <Step title="풀 요청 생성">
    차이가 올바르면, 차이 보기 상단에서 **PR 생성**을 선택합니다. 전체 PR로 열거나, 초안으로 열거나, 생성된 제목 및 설명과 함께 GitHub의 작성 페이지로 이동할 수 있습니다.
  </Step>

  <Step title="PR 후 계속 반복">
    PR이 생성된 후 세션이 활성 상태로 유지됩니다. CI 실패 출력 또는 검토자 댓글을 채팅에 붙여넣고 Claude에게 이를 해결하도록 요청합니다. Claude가 PR을 자동으로 모니터링하도록 하려면, [자동 수정 풀 요청](/docs/ko/claude-code-on-the-web#auto-fix-pull-requests)을 참조합니다.
  </Step>
</Steps>

<h2 id="troubleshoot-setup">
  설정 문제 해결
</h2>

<h3 id="no-repositories-appear-after-connecting-github">
  GitHub 연결 후 저장소가 나타나지 않음
</h3>

클라우드 세션은 연결된 GitHub 계정이 볼 수 있는 모든 저장소를 사용할 수 있습니다. Claude GitHub 앱이 설치된 저장소와 관계없이 말입니다. 저장소가 누락된 경우, 연결된 GitHub 계정이 GitHub에서 해당 저장소에 접근할 수 있는지 확인합니다. 저장소에 대해 [자동 수정](/docs/ko/claude-code-on-the-web#auto-fix-pull-requests)을 원하는 경우, 앱을 설치합니다: github.com에서 **Settings → Applications → Claude → Configure**를 열고 저장소가 **Repository access** 아래에 나열되어 있는지 확인합니다. 비공개 저장소는 공개 저장소와 동일한 권한이 필요합니다.

<h3 id="the-page-only-shows-a-github-login-button">
  페이지에 GitHub 로그인 버튼만 표시됨
</h3>

클라우드 세션은 연결된 GitHub 계정이 필요합니다. 위의 브라우저 흐름을 통해 연결하거나, GitHub CLI를 사용하는 경우 터미널에서 `/web-setup`을 실행합니다. GitHub를 연결하지 않으려면, [Remote Control](/docs/ko/remote-control)을 참조하여 자신의 머신에서 Claude Code를 실행하고 웹에서 모니터링합니다.

<h3 id="not-available-for-the-selected-organization">
  "선택한 조직에서 사용할 수 없음"
</h3>

Enterprise 조직은 관리자가 웹에서 Claude Code를 활성화해야 할 수 있습니다. Anthropic 계정 팀에 문의합니다.

<h3 id="/web-setup-shows-no-commands-match-or-unknown-command">
  `/web-setup`이 "No commands match" 또는 "Unknown command"를 표시함
</h3>

`/web-setup`은 셸이 아닌 Claude Code CLI 내부에서 실행됩니다. 먼저 `claude`를 시작한 다음, 프롬프트에서 `/web-setup`을 입력합니다.

Claude Code 내부에 입력했는데 명령 메뉴에 `No commands match "/web-setup"`이 표시되거나 제출하면 `Unknown command: /web-setup`이 반환되면, 요구 사항이 충족되지 않아 명령이 숨겨져 있습니다. 원인은 보통 API 키 또는 타사 공급자 대신 claude.ai 구독으로 인증되었기 때문입니다. `/login`을 실행하여 claude.ai 계정으로 로그인합니다.

<h3 id="could-not-create-a-cloud-environment-or-no-cloud-environment-available-when-using-cloud-or-ultraplan">
  `--cloud` 또는 ultraplan 사용 시 "클라우드 환경을 생성할 수 없음" 또는 "사용 가능한 클라우드 환경 없음"
</h3>

원격 세션 기능은 아직 없는 경우 기본 클라우드 환경을 자동으로 생성합니다. "클라우드 환경을 생성할 수 없음"이 표시되면, 자동 생성이 실패했습니다. "사용 가능한 클라우드 환경 없음"이 표시되면, CLI가 자동 생성보다 앞서 있습니다. 어느 경우든, Claude Code CLI에서 `/web-setup`을 실행하여 수동으로 생성하거나, [claude.ai/code](https://claude.ai/code)를 방문하고 위의 **환경 생성** 단계를 따릅니다.

<h3 id="setup-script-failed">
  설정 스크립트 실패
</h3>

설정 스크립트가 0이 아닌 상태로 종료되어 세션 시작을 차단합니다. 일반적인 원인:

* 레지스트리가 [네트워크 접근 수준](/docs/ko/claude-code-on-the-web#access-levels)에 없어서 패키지 설치가 실패했습니다. `Trusted`는 대부분의 패키지 관리자를 포함합니다. `None`은 모두 차단합니다.
* 스크립트가 신선한 복제에 존재하지 않는 파일 또는 경로를 참조합니다.
* 로컬에서 작동하는 명령이 Ubuntu에서 다른 호출이 필요합니다.

디버깅하려면, 스크립트 상단에 `set -x`를 추가하여 어느 명령이 실패했는지 확인합니다. 중요하지 않은 명령의 경우, `|| true`를 추가하여 세션 시작을 차단하지 않도록 합니다.

<h3 id="new-sessions-hang-or-time-out-during-setup">
  새 세션이 설정 중에 중단되거나 시간 초과됨
</h3>

새 세션이 설정 스크립트 단계에서 정체되거나 스크립트가 완료되기 전에 일반 컨테이너 오류로 실패하면, 스크립트가 [환경 캐시](/docs/ko/claude-code-on-the-web#environment-caching) 구축을 위한 대략 5분의 시간 예산을 초과할 가능성이 높습니다. 큰 Docker 이미지 가져오기, 전체 종속성 트리 동기화 또는 모델 가중치 다운로드와 같은 무거운 단계는 특히 순차적으로 실행될 때 총합을 제한을 초과합니다.

이를 해결하려면, 스크립트를 정리하여 5분 이내에 안정적으로 완료되도록 합니다:

* `&`와 최종 `wait`를 사용하여 독립적인 설치를 병렬로 실행하는 대신 순차적으로 실행합니다.
* 가장 큰 다운로드를 설정 스크립트에서 [SessionStart hook](/docs/ko/claude-code-on-the-web#setup-scripts-vs-sessionstart-hooks)으로 이동하여 백그라운드에서 시작하도록 하여, 세션이 완료되는 동안 사용 가능해집니다.
* 설정 스크립트에서 긴 재시도 대기를 제거합니다. 정체된 재시도 루프는 예산에 포함됩니다.

<h3 id="session-keeps-running-after-closing-the-tab">
  탭을 닫은 후 세션이 계속 실행됨
</h3>

이는 의도된 동작입니다. 탭을 닫거나 다른 곳으로 이동해도 세션이 중지되지 않습니다. Claude가 현재 작업을 완료한 다음 유휴 상태가 될 때까지 백그라운드에서 계속 실행됩니다. 사이드바에서, [세션을 보관](/docs/ko/claude-code-on-the-web#archive-sessions)하여 목록에서 숨기거나, [삭제](/docs/ko/claude-code-on-the-web#delete-sessions)하여 영구적으로 제거할 수 있습니다.

<h2 id="next-steps">
  다음 단계
</h2>

이제 작업을 제출하고 검토할 수 있으므로, 이 페이지들은 다음에 올 것을 다룹니다: 터미널에서 클라우드 세션 시작, 반복 작업 예약, Claude에게 상시 지침 제공.

* [웹에서 Claude Code 사용](/docs/ko/claude-code-on-the-web): 터미널로 세션 텔레포트, 설정 스크립트, 환경 변수, 네트워크 구성을 포함한 전체 참조
* [Routines](/docs/ko/routines): 일정에 따라, API 호출을 통해, 또는 GitHub 이벤트에 응답하여 작업을 자동화합니다
* [CLAUDE.md](/docs/ko/memory): 모든 세션의 시작 시 로드되는 지속적인 지침 및 컨텍스트를 Claude에게 제공합니다
* [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) 또는 [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude)용 Claude 모바일 앱을 설치하여 휴대폰에서 세션을 모니터링합니다. Claude Code CLI에서, `/mobile`은 QR 코드를 표시합니다.
