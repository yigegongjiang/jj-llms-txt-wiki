> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 클라우드에서 Claude Code 시작하기

> 브라우저나 휴대폰에서 클라우드에서 Claude Code를 실행합니다. GitHub 저장소를 연결하고, 작업을 제출하고, 로컬 설정 없이 PR을 검토합니다.

<Note>
  클라우드 세션은 Pro, Max, Team 플랜과 프리미엄 시트 또는 Chat + Claude Code 시트가 있는 Enterprise 사용자를 위해 제공됩니다.
</Note>

클라우드 세션은 사용자의 머신 대신 기본적으로 Anthropic 관리 클라우드 인프라에서 Claude Code를 실행합니다. 이 빠른 시작은 브라우저에서 [claude.ai/code](https://claude.ai/code)에서 시작합니다. Claude 모바일 앱, Desktop 앱, 또는 터미널에서 `claude --cloud`를 사용하여 시작할 수도 있습니다.

[시작하기](#connect-github)를 위해 GitHub 저장소가 필요합니다. Claude는 이를 격리된 가상 머신으로 복제하고, 변경 사항을 만들고, 검토할 수 있도록 브랜치를 푸시합니다. 세션은 기기 간에 지속되므로, 노트북에서 시작한 작업을 나중에 휴대폰에서 검토할 수 있습니다.

클라우드 세션은 다음에 적합합니다:

* **병렬 작업**: 여러 개의 독립적인 작업을 동시에 실행하며, 각각 자신의 세션과 브랜치에서 실행되고, 여러 worktrees를 관리할 필요가 없습니다
* **로컬에 없는 저장소**: Claude는 매 세션마다 저장소를 새로 복제하므로, 체크아웃할 필요가 없습니다
* **자주 조정할 필요가 없는 작업**: 잘 정의된 작업을 제출하고, 다른 작업을 하고, Claude가 완료되면 결과를 검토합니다
* **코드 질문 및 탐색**: 로컬 체크아웃 없이 코드베이스를 이해하거나 기능이 어떻게 구현되는지 추적합니다

로컬 구성, 도구 또는 환경이 필요한 작업의 경우, Claude Code를 로컬에서 실행하거나 [Remote Control](/docs/ko/remote-control)을 사용하는 것이 더 적합합니다.

<h2 id="how-sessions-run">
  세션이 실행되는 방식
</h2>

아래 단계는 Anthropic 호스팅 세션을 설명합니다. [자체 호스팅 환경](/docs/ko/self-hosted-environments)에서는 복제 및 그 이후의 모든 작업이 조직의 자체 러너에서 실행되며, 네트워크 경계, 설정 및 푸시 동작은 운영자가 구성합니다. 작업을 제출할 때:

1. **복제 및 준비**: 저장소가 Anthropic 관리 VM으로 복제되고, 구성된 경우 [설정 스크립트](/docs/ko/cloud-environments#setup-scripts)가 실행됩니다.
2. **네트워크 구성**: 인터넷 접근은 환경의 [접근 수준](/docs/ko/cloud-environments#access-levels)에 따라 설정됩니다.
3. **작업**: Claude는 코드를 분석하고, 변경 사항을 만들고, 테스트를 실행하고, 작업을 확인합니다. 전체 과정을 지켜보고 조정할 수 있거나, 물러나 있다가 완료되면 돌아올 수 있습니다.
4. **브랜치 푸시**: Claude가 중지점에 도달하면, 브랜치를 GitHub로 푸시합니다. 차이를 검토하고, 인라인 댓글을 남기고, PR을 생성하거나, 계속 진행하도록 다른 메시지를 보냅니다.

브랜치가 푸시될 때 세션이 닫히지 않습니다. PR 생성 및 추가 편집은 모두 동일한 대화 내에서 발생합니다.

<h2 id="compare-ways-to-run-claude-code">
  Claude Code를 실행하는 방법 비교
</h2>

Claude Code는 모든 곳에서 동일하게 작동합니다. 변경되는 것은 코드가 실행되는 위치와 로컬 구성을 사용할 수 있는지 여부입니다:

|                                   | 클라우드 세션                                                                                            | 로컬 세션                                                                                       | [Remote Control](/docs/ko/remote-control)을 사용한 로컬 세션 |
| :-------------------------------- | :------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------ | :---------------------------------------------- |
| **코드 실행 위치**                      | 기본적으로 Anthropic에서 관리하는 클라우드 VM                                                                     | 사용자의 머신                                                                                     | 사용자의 머신                                         |
| **시작 위치**                         | claude.ai/code, Claude 모바일 앱, **Cloud**가 선택된 Desktop 앱, 또는 `claude --cloud`                        | 터미널, IDE, 또는 **Local**이 선택된 Desktop 앱                                                       | 터미널, VS Code 확장 프로그램, 또는 Desktop 앱              |
| **채팅 위치**                         | claude.ai, 모바일 앱, 또는 Desktop 앱                                                                     | 시작한 위치                                                                                      | claude.ai 또는 모바일 앱, 그리고 시작한 위치                  |
| **로컬 구성 사용**                      | 아니오, 저장소만                                                                                          | 예                                                                                           | 예                                               |
| **GitHub 필요**                     | 예, 또는 `--cloud`를 통해 [로컬 저장소 번들](/docs/ko/claude-code-on-the-web#send-local-repositories-without-github) | 아니오                                                                                         | 아니오                                             |
| **연결 해제 시 계속 실행**                 | 예                                                                                                  | 아니오                                                                                         | 머신에서 세션이 열려 있는 동안                               |
| **[권한 모드](/docs/ko/permission-modes)** | 편집 자동 수락, Plan, Auto                                                                               | 터미널의 모든 모드; IDE 및 Desktop 앱의 경우 [권한 모드 전환](/docs/ko/permission-modes#switch-permission-modes) 참조 | claude.ai 및 모바일 앱에서 수동, 편집 자동 수락, 또는 Plan       |
| **네트워크 접근**                       | 환경별로 구성 가능                                                                                         | 머신의 네트워크                                                                                    | 머신의 네트워크                                        |

로컬 세션을 설정하려면 [터미널 빠른 시작](/docs/ko/quickstart), [Desktop 앱](/docs/ko/desktop), 또는 [Remote Control](/docs/ko/remote-control) 문서를 참조하십시오.

<h2 id="connect-github">
  GitHub 연결
</h2>

GitHub 연결은 일회성 단계입니다. 이미 GitHub CLI를 사용하는 경우, 브라우저 대신 [터미널에서 이를 수행](#connect-from-your-terminal)할 수 있습니다.

<Note>
  Team 및 Enterprise 플랜에서 **GitHub로 로그인** 단계는 Claude 조직의 [Owner](/docs/ko/server-managed-settings#access-control)가 [**Admin settings > Connectors**](https://claude.ai/admin-settings/connectors)에서 GitHub 커넥터를 켠 후에만 작동합니다. 그때까지 해당 단계는 로그인 버튼 대신 "GitHub access is required for Claude Code on the web"을 표시합니다. 커넥터가 켜진 후 [claude.ai/code](https://claude.ai/code)를 다시 로드하고 첫 번째 단계부터 다시 시작합니다. [**Admin settings > Claude Code**](https://claude.ai/admin-settings/claude-code)의 [Quick web setup](/docs/ko/claude-code-on-the-web#github-authentication-options)이라는 두 번째 토글은 선택 사항입니다. 이를 켜면 `/web-setup`이 작동하고 온보딩이 멤버를 위한 환경을 생성합니다.
</Note>

<Steps>
  <Step title="Visit claude.ai/code">
    [claude.ai/code](https://claude.ai/code)로 이동하고 claude.ai 계정으로 로그인합니다.
  </Step>

  <Step title="Sign in with GitHub">
    로그인한 후 claude.ai/code는 GitHub를 연결하도록 요청합니다. 프롬프트를 따르면 claude.ai/code가 GitHub의 인증 페이지로 이동합니다. 인증 요청을 승인하면 GitHub가 claude.ai/code로 돌아갑니다. 클라우드 세션은 기존 GitHub 저장소와 함께 작동합니다. 새 프로젝트를 시작하려면 먼저 [GitHub에서 빈 저장소를 생성](https://github.com/new)합니다.

    이 연결을 통해 세션은 모든 공개 저장소를 복제할 수 있지만, Claude GitHub 앱이 설치된 경우에만 비공개 저장소에서 작동할 수 있습니다. 사용하려는 비공개 저장소가 있는 각 GitHub 계정 또는 조직에 [Claude GitHub 앱을 설치](https://github.com/apps/claude/installations/new)합니다. GitHub 조직에서는 조직 소유자가 설치를 승인해야 할 수 있습니다. 앱을 설치하면 [Auto-fix](/docs/ko/claude-code-on-the-web#auto-fix-pull-requests)도 활성화되어 Claude가 CI 실패에 응답하고 해당 저장소의 pull request에 대한 검토 의견에 응답할 수 있습니다.

    온보딩이 이 시점에서 Claude GitHub 앱을 설치하도록 요청하지만 나중에 설치하려면 **Skip**을 클릭합니다.
  </Step>

  <Step title="Set up your Default environment">
    [cloud environment](/docs/ko/cloud-environments)는 세션 중에 Claude가 가진 네트워크 접근 권한과 세션이 시작될 때 실행되는 것을 제어하는 저장된 구성입니다. GitHub를 연결한 후 발생하는 일은 플랜에 따라 다릅니다:

    * **Pro and Max**: 온보딩이 **Default**라는 환경을 생성합니다.
    * **Team and Enterprise**: 온보딩이 **Create your first cloud environment** 양식을 표시합니다. 미리 채워진 이름과 네트워크 접근을 변경하지 않고 **Create & finish**를 클릭하여 **Default** 환경을 생성합니다. Owner가 [Quick web setup](/docs/ko/claude-code-on-the-web#github-authentication-options)을 켠 경우 온보딩이 대신 **Default**를 생성합니다.

    **Default**는 [`Trusted` network access](/docs/ko/cloud-environments#access-levels)를 사용합니다: 세션은 [common package registries](/docs/ko/cloud-environments#default-allowed-domains) 및 기타 허용 목록에 있는 도메인에 도달하고 세션의 네트워크를 통해 다른 것에는 도달하지 않습니다. 구성 없이 사용 가능한 것은 [Installed tools](/docs/ko/cloud-environments#installed-tools)를 참조합니다.

    첫 번째 프로젝트의 경우 **Default** 환경이 그대로 작동합니다. 네트워크 접근을 변경하거나, 환경 변수를 추가하거나, 세션이 시작되기 전에 [setup script](/docs/ko/cloud-environments#setup-scripts)를 실행하려면 [이를 편집하거나 추가 환경을 생성](/docs/ko/cloud-environments#configure-your-environment)합니다.
  </Step>
</Steps>

<h3 id="connect-from-your-terminal">
  터미널에서 연결
</h3>

이미 GitHub CLI(`gh`)를 사용하는 경우, 터미널에서 클라우드 세션을 위해 GitHub를 연결할 수 있습니다. 이는 [Claude Code CLI](/docs/ko/quickstart)가 필요합니다. Team 및 Enterprise 플랜에서 `/web-setup`은 Owner가 [Quick web setup](/docs/ko/claude-code-on-the-web#github-authentication-options)을 켠 후에만 사용 가능합니다.

`/web-setup`을 실행하면 Claude Code는 `gh auth token`이 출력하는 토큰을 읽고, 확인을 요청하고, 토큰을 Anthropic으로 보냅니다. Anthropic은 claude.ai 계정으로 암호화하여 저장하고, 클라우드 세션은 [토큰을 제거](#remove-the-web-setup-token)할 때까지 GitHub 접근을 위해 이를 사용합니다. 직접 시작한 클라우드 세션은 그 후 해당 토큰이 접근할 수 있는 모든 저장소에 접근할 수 있으며, Claude GitHub 앱 설치가 필요하지 않습니다. [project](/docs/ko/claude-projects#set-up-github-access)의 스레드는 여전히 Claude GitHub 앱이 필요합니다.

이미 브라우저에서 GitHub를 연결한 경우, `/web-setup`은 클라우드 세션에 대한 해당 연결을 대체한다는 경고를 표시합니다.

<Note>
  [Zero Data Retention](/docs/ko/zero-data-retention)이 활성화된 조직은 `/web-setup` 또는 기타 클라우드 세션 기능을 사용할 수 없습니다. GitHub CLI가 설치되지 않았거나 인증되지 않은 경우, Claude Code는 대신 브라우저 온보딩 흐름을 엽니다.
</Note>

<Steps>
  <Step title="GitHub CLI로 인증">
    셸에서, 아직 하지 않은 경우 GitHub CLI를 인증합니다:

    ```bash theme={null}
    gh auth login
    ```
  </Step>

  <Step title="Claude에 로그인">
    Claude Code CLI에서 `/login`을 실행하여 claude.ai 계정으로 로그인합니다. 이미 claude.ai 계정으로 로그인한 경우 이 단계를 건너뜁니다. API 키로 인증하는 것은 계산되지 않습니다. 확인하려면 `/status`를 실행하고 **Login method** 행이 claude.ai 계정을 표시하는지 확인합니다.
  </Step>

  <Step title="/web-setup 실행">
    Claude Code CLI에서 다음을 실행합니다:

    ```text theme={null}
    /web-setup
    ```

    프롬프트를 확인하여 `gh` 토큰을 Claude 계정으로 보냅니다. 성공하면 Claude Code는 `Connected as <your-github-username>`을 출력하고 브라우저에서 [claude.ai/code](https://claude.ai/code)를 엽니다. 아직 클라우드 환경이 없는 경우, `/web-setup`은 Trusted 네트워크 접근 및 설정 스크립트 없이 환경을 생성합니다. 나중에 [환경을 편집하거나 변수를 추가](/docs/ko/cloud-environments#configure-your-environment)할 수 있습니다. `/web-setup`이 완료되면 터미널에서 [`--cloud`](/docs/ko/claude-code-on-the-web#from-terminal-to-cloud)를 사용하여 클라우드 세션을 시작하거나 [`/schedule`](/docs/ko/routines)을 사용하여 반복 작업을 설정할 수 있습니다.
  </Step>
</Steps>

<h4 id="remove-the-web-setup-token">
  `/web-setup` 토큰 제거
</h4>

Claude 계정에서 토큰을 제거하려면 [claude.ai/customize/connectors](https://claude.ai/customize/connectors)에서 GitHub를 연결 해제합니다. 연결 해제하면 클라우드 세션이 사용하는 GitHub 자격 증명이 삭제되며, 이는 브라우저에서 오든 `/web-setup`에서 오든 상관없이 클라우드 세션이 다시 연결할 때까지 GitHub 접근을 잃습니다. 로컬 `gh`는 로그인 상태로 유지되고, 토큰은 GitHub에서 유효한 상태로 유지됩니다.

토큰 자체를 무효화하려면 GitHub에서 이를 취소합니다. 브라우저를 통해 `gh`에 로그인한 경우, 토큰은 GitHub의 [**Settings > Applications > Authorized OAuth Apps**](https://github.com/settings/applications)의 **GitHub CLI** 항목에 속하며, 해당 항목을 취소하면 머신의 GitHub CLI도 로그아웃됩니다. 클라우드 세션은 `gh auth login` 및 `/web-setup`을 다시 실행할 때까지 GitHub 접근을 잃습니다.

<h2 id="start-a-task">
  작업 시작
</h2>

GitHub가 연결되고 환경이 생성되면, 작업을 제출할 준비가 되었습니다.

<Steps>
  <Step title="저장소 및 브랜치 선택">
    [claude.ai/code](https://claude.ai/code) 또는 Claude 모바일 앱의 Code 탭에서, 입력 상자 아래의 저장소 선택기를 클릭하고 Claude가 작업할 저장소를 선택합니다. 각 저장소는 브랜치 선택기를 표시합니다. 기본값 대신 기능 브랜치에서 Claude를 시작하도록 변경합니다. 한 세션에서 여러 저장소를 추가하여 작업할 수 있습니다.
  </Step>

  <Step title="권한 모드 선택">
    입력 옆의 모드 드롭다운은 세션이 실행될 모드를 표시합니다:

    * **Auto**: 분류기가 사용자에게 묻는 대신 Claude의 작업을 검토합니다. 조직에서 자동 모드를 허용하고 선택한 모델이 이를 지원할 때 나타납니다
    * **Accept edits**: Claude는 승인을 기다리지 않고 변경 사항을 만들고 브랜치를 푸시합니다
    * **Plan**: Claude는 접근 방식을 제안하고 파일을 편집하기 전에 승인을 기다립니다

    클라우드 세션은 Manual 또는 Bypass 권한을 제공하지 않습니다. 각 권한 모드가 허용하는 사항에 대해서는 [권한 모드의 전체 목록](/docs/ko/permission-modes#available-modes)을 참조합니다.
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
| `environment`  | 미리 선택할 [환경](#connect-github)의 이름 또는 ID입니다.                                                |

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

    차이 보기는 기본적으로 세션의 변경 사항을 기본 분기와 비교합니다. 다른 분기와 비교하려면 **비교 대상**을 선택하고 하나를 선택합니다.
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

브라우저에서 GitHub를 연결한 경우, 세션은 모든 공개 저장소를 복제할 수 있지만, 비공개 저장소는 Claude GitHub App이 해당 저장소를 소유한 계정 또는 조직에 설치되어 있고 설치의 저장소 액세스에 포함된 경우에만 나타납니다. [Claude GitHub App을 설치](https://github.com/apps/claude/installations/new)하거나, 조직 소유자에게 설치 또는 승인을 요청하세요.

`/web-setup`으로 연결한 경우, 세션은 `gh` 토큰이 액세스할 수 있는 모든 저장소에 도달합니다. 셸에서 `gh repo view OWNER/REPO`를 실행하여 GitHub CLI 로그인이 저장소를 볼 수 있는지 확인하고, 연결한 이후 `gh` 계정을 전환했다면 `/web-setup`을 다시 실행하세요.

<h3 id="the-page-only-shows-a-github-login-button">
  페이지에 GitHub 로그인 버튼만 표시됨
</h3>

클라우드 세션에는 연결된 GitHub 계정이 필요합니다. 위의 브라우저 흐름을 통해 연결하거나, GitHub CLI를 사용하는 경우 터미널에서 `/web-setup`을 실행하세요. GitHub를 연결하지 않으려면 [Remote Control](/docs/ko/remote-control)을 참조하여 자신의 머신에서 Claude Code를 실행하고 웹에서 모니터링하세요.

<h3 id="not-available-for-the-selected-organization">
  "선택한 조직에서 사용할 수 없음"
</h3>

엔터프라이즈 조직의 경우 소유자가 클라우드 세션을 활성화해야 할 수 있습니다. Anthropic 계정 팀에 문의하세요.

<h3 id="/web-setup-says-not-signed-in-to-claude">
  `/web-setup`에서 "Claude에 로그인되지 않음"이라고 표시됨
</h3>

`/web-setup`이 "Claude에 로그인되지 않았습니다. 먼저 /login을 실행하세요."라고 응답하면, CLI에 유효한 claude.ai 로그인이 없습니다. 이는 이전 로그인이 만료된 경우에도 발생할 수 있습니다. `/login`을 실행하고, claude.ai 계정으로 로그인한 후, `/web-setup`을 다시 실행하세요.

<h3 id="/web-setup-warns-that-your-token-doesn’t-have-the-workflow-scope">
  `/web-setup`에서 GitHub CLI 토큰에 `workflow` 범위가 없다는 경고가 표시됨
</h3>

`/web-setup`에서 GitHub CLI 토큰에 `workflow` 범위가 없다고 표시되면 계속 진행할 수 있지만, GitHub은 해당 토큰으로 수행된 일부 푸시(예: GitHub Actions 워크플로우 파일을 변경하는 푸시)를 거부할 수 있습니다. 범위를 추가하려면 셸에서 `gh auth refresh -s workflow`를 실행한 후 `/web-setup`을 다시 실행하세요.

<h3 id="web-setup-shows-no-commands-match-or-unknown-command">
  `/web-setup`에서 "명령과 일치하는 항목 없음" 또는 "알 수 없는 명령"이 표시됨
</h3>

`/web-setup`은 Claude Code CLI 내에서 실행되며, 셸에서는 실행되지 않습니다. 먼저 `claude`를 시작한 후 프롬프트에서 `/web-setup`을 입력하세요.

Claude Code 내에서 입력했는데 명령 메뉴에 `"/web-setup"과 일치하는 명령 없음`이 표시되거나 제출하면 `알 수 없는 명령: /web-setup`이 반환되면, 요구 사항이 충족되지 않아 명령이 숨겨져 있습니다. 일반적으로 API 키 또는 타사 제공자로 인증되었으며 claude.ai 구독이 아닌 경우입니다. `/login`을 실행하여 claude.ai 계정으로 로그인하세요.

Team 및 Enterprise 플랜에서는 명령이 기본적으로 숨겨져 있습니다. [빠른 웹 설정 토글](/docs/ko/claude-code-on-the-web#github-authentication-options)은 소유자가 켤 때까지 꺼져 있습니다. 꺼져 있는 동안 [브라우저에서 GitHub 연결](#connect-github)하세요.

명령은 다른 두 가지 경우에도 숨겨집니다:

* 관리자가 조직에 대해 클라우드 세션을 비활성화했습니다. 이 경우 `/web-setup`을 제출하면 [`클라우드 세션이 조직의 정책에 의해 비활성화됨`](/docs/ko/errors#cloud-sessions-are-disabled-by-your-organizations-policy)이 반환됩니다. v2.1.268 이전에는 이 경우도 `알 수 없는 명령: /web-setup`을 반환했습니다.
* Enterprise 조직에 [Zero Data Retention](/docs/ko/zero-data-retention)이 활성화되어 있어 클라우드 세션을 사용할 수 없습니다.

<h3 id="could-not-create-a-cloud-environment-or-no-cloud-environment-available-when-using-cloud">
  `--cloud` 사용 시 "클라우드 환경을 만들 수 없음" 또는 "사용 가능한 클라우드 환경 없음"
</h3>

클라우드 세션 기능은 클라우드 환경이 없으면 자동으로 기본 클라우드 환경을 만듭니다. "클라우드 환경을 만들 수 없음"이 표시되면 자동 생성이 실패한 것입니다. "사용 가능한 클라우드 환경 없음"이 표시되면 CLI가 자동 생성보다 이전 버전입니다. 어느 경우든 Claude Code CLI에서 `/web-setup`을 실행하거나, [claude.ai/code](https://claude.ai/code)의 [환경 선택기](/docs/ko/cloud-environments#configure-your-environment)에서 환경을 추가하세요.

<h3 id="setup-script-failed">
  설정 스크립트 실패
</h3>

설정 스크립트가 0이 아닌 상태로 종료되어 세션 시작이 차단되었습니다. 일반적인 원인:

* 레지스트리가 [네트워크 액세스 수준](/docs/ko/cloud-environments#access-levels)에 없어서 패키지 설치가 실패했습니다. `Trusted`는 대부분의 패키지 관리자를 포함합니다. `None`은 모두 차단합니다.
* 스크립트가 새로운 복제본에 존재하지 않는 파일 또는 경로를 참조합니다.
* 로컬에서 작동하는 명령이 Ubuntu에서 다른 호출이 필요합니다.

디버깅하려면 스크립트 맨 위에 `set -x`를 추가하여 어느 명령이 실패했는지 확인하세요. 중요하지 않은 명령의 경우 `|| true`를 추가하여 세션 시작을 차단하지 않도록 하세요.

<h3 id="new-sessions-hang-or-time-out-during-setup">
  새 세션이 설정 중에 중단되거나 시간 초과됨
</h3>

새 세션이 설정 스크립트 단계에서 정지되거나 스크립트가 완료되기 전에 일반적인 컨테이너 오류로 실패하면, 스크립트가 [환경 캐시](/docs/ko/cloud-environments#environment-caching) 구축을 위한 대략 5분의 시간 예산을 초과하고 있을 가능성이 높습니다. 큰 Docker 이미지 가져오기, 전체 종속성 트리 동기화 또는 모델 가중치 다운로드와 같은 무거운 단계는 특히 순차적으로 실행될 때 총합을 제한을 초과하는 경우가 많습니다.

이를 해결하려면 스크립트를 정리하여 5분 이내에 안정적으로 완료되도록 하세요:

* 독립적인 설치를 `&`와 최종 `wait`를 사용하여 병렬로 실행하고, 순차적으로 실행하지 마세요.
* 가장 큰 다운로드를 설정 스크립트에서 [SessionStart 훅](/docs/ko/cloud-environments#setup-scripts-vs-sessionstart-hooks)으로 이동하여 백그라운드에서 시작하도록 하면, 세션이 완료되는 동안 사용 가능해집니다.
* 설정 스크립트에서 긴 재시도 대기를 제거하세요. 정지된 재시도 루프는 예산에 포함됩니다.

<h3 id="session-keeps-running-after-closing-the-tab">
  탭을 닫은 후 세션이 계속 실행됨
</h3>

이는 의도된 동작입니다. 탭을 닫거나 다른 페이지로 이동해도 세션이 중지되지 않습니다. Claude가 현재 작업을 완료할 때까지 백그라운드에서 계속 실행된 후 유휴 상태가 됩니다. 사이드바에서 [세션을 보관](/docs/ko/claude-code-on-the-web#archive-sessions)하여 목록에서 숨기거나, [삭제](/docs/ko/claude-code-on-the-web#delete-sessions)하여 영구적으로 제거할 수 있습니다.

<h2 id="next-steps">
  다음 단계
</h2>

이제 작업을 제출하고 검토할 수 있으므로, 이 페이지들은 다음에 올 것을 다룹니다: 터미널에서 클라우드 세션 시작, 반복 작업 예약, Claude에게 상시 지침 제공.

* [웹에서 Claude Code 사용](/docs/ko/claude-code-on-the-web): 터미널로 세션 텔레포트, 세션 공유, 자동 풀 요청 수정을 포함한 전체 참조
* [클라우드 환경 구성](/docs/ko/cloud-environments): 네트워크 액세스 수준, 환경 변수, 클라우드 세션용 설정 스크립트
* [Routines](/docs/ko/routines): 일정에 따라, API 호출을 통해, 또는 GitHub 이벤트에 응답하여 작업을 자동화합니다
* [CLAUDE.md](/docs/ko/memory): 모든 세션의 시작 시 로드되는 지속적인 지침 및 컨텍스트를 Claude에게 제공합니다
* [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) 또는 [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude)용 Claude 모바일 앱을 설치하여 휴대폰에서 세션을 모니터링합니다. Claude Code CLI에서, `/mobile`은 [claude.ai/mobile](https://claude.ai/mobile)에 대한 QR 코드를 표시하며, 이는 휴대폰에 맞는 올바른 앱 스토어를 엽니다.
