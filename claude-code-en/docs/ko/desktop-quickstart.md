> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 데스크톱 앱 시작하기

> 데스크톱에 Claude Code를 설치하고 첫 번째 코딩 세션을 시작합니다

데스크톱 앱은 여러 세션을 나란히 실행하도록 구축된 그래픽 인터페이스를 갖춘 Claude Code를 제공합니다: 병렬 작업을 관리하기 위한 사이드바, 통합 터미널 및 파일 편집기가 있는 드래그 앤 드롭 레이아웃, 시각적 diff 검토, 라이브 앱 미리보기, GitHub PR 모니터링 및 자동 병합, 그리고 예약된 작업입니다. 터미널이 필요하지 않습니다.

<CardGroup cols={3}>
  <Card title="macOS용 다운로드" icon="apple" href="https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code&utm_medium=docs">
    Intel 및 Apple Silicon용 범용 빌드
  </Card>

  <Card title="Windows용 다운로드" icon="windows" href="https://claude.ai/api/desktop/win32/x64/setup/latest/redirect?utm_source=claude_code&utm_medium=docs">
    x64 프로세서용
  </Card>

  <Card title="Linux용 Claude 다운로드(베타)" icon="linux" href="/docs/ko/desktop-linux">
    Ubuntu 및 Debian용 apt 또는 .deb
  </Card>
</CardGroup>

Windows ARM64의 경우 [ARM64 설치 프로그램](https://claude.ai/api/desktop/win32/arm64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs)을 다운로드하십시오. Linux에서는 apt로 설치하십시오. [Linux의 Claude Desktop](/docs/ko/desktop-linux)을 참조하십시오.

<Note>
  Claude Code는 [Pro, Max, Team, 또는 Enterprise 구독](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=desktop_quickstart_pricing)이 필요합니다.
</Note>

이 페이지는 앱 설치 및 첫 번째 세션 시작을 안내합니다. 이미 설정되어 있다면 전체 참조는 [Claude Code Desktop 사용](/docs/ko/desktop)을 참조하세요.

데스크톱 앱에는 세 개의 탭이 있습니다:

* **Chat**: 파일 접근이 없는 일반 대화로, claude.ai와 유사합니다.
* **Cowork**: 자신의 환경을 가진 샌드박스 가상 머신에서 작업을 수행하는 자율 백그라운드 에이전트입니다. 사용자가 다른 작업을 하는 동안 독립적으로 실행됩니다. 온디바이스 Cowork 세션은 컴퓨터에서 VM을 실행하고, 원격 Cowork 세션은 대신 Anthropic 관리 VM에서 실행됩니다.
* **Code**: 로컬 파일에 직접 접근할 수 있는 대화형 코딩 어시스턴트입니다. 실시간으로 각 변경 사항을 검토하고 승인합니다.

Chat과 Cowork는 [Claude 도움말 센터](https://support.claude.com/)에서 다룹니다. 데스크톱 앱 설치 및 배포는 [Claude Desktop 지원 문서](https://support.claude.com/en/collections/16163169-claude-desktop)에서 다룹니다. 이 페이지는 **Code** 탭에 중점을 둡니다.

<h2 id="install">
  설치
</h2>

<Steps>
  <Step title="설치 및 로그인">
    macOS와 Windows에서는 위의 링크에서 설치 프로그램을 다운로드하고 실행합니다. Linux에서는 [Linux의 Claude Desktop](/docs/ko/desktop-linux)에서 설치 단계를 따릅니다. macOS의 Applications 폴더, Windows의 Start 메뉴 또는 Linux의 애플리케이션 런처에서 Claude를 실행한 다음 Anthropic 계정으로 로그인합니다.
  </Step>

  <Step title="Code 탭 열기">
    상단 중앙의 **Code** 탭을 클릭합니다. Code를 클릭할 때 업그레이드를 요청하는 메시지가 나타나면 먼저 [유료 요금제를 구독](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=desktop_quickstart_upgrade)해야 합니다. 온라인 로그인을 요청하는 메시지가 나타나면 로그인을 완료하고 앱을 다시 시작합니다. 403 오류가 표시되면 [인증 문제 해결](/docs/ko/desktop#403-or-authentication-errors-in-the-code-tab)을 참조하세요.
  </Step>
</Steps>

데스크톱 앱에는 Claude Code가 포함되어 있습니다. Node.js나 CLI를 별도로 설치할 필요가 없습니다. 터미널에서 `claude`를 사용하려면 CLI를 별도로 설치하세요. [CLI 시작하기](/docs/ko/quickstart)를 참조하세요.

<h2 id="start-your-first-session">
  첫 번째 세션 시작하기
</h2>

Code 탭을 열고 프로젝트를 선택한 후 Claude에게 작업을 지시합니다.

<Steps>
  <Step title="환경 및 폴더 선택">
    **Local**을 선택하여 파일을 직접 사용하여 머신에서 Claude를 실행합니다. **Select folder**를 클릭하고 프로젝트 디렉토리를 선택합니다.

    <Tip>
      잘 알고 있는 작은 프로젝트부터 시작하십시오. Claude Code가 무엇을 할 수 있는지 가장 빠르게 확인할 수 있는 방법입니다.
    </Tip>

    다음을 선택할 수도 있습니다:

    * **Cloud**: 앱을 닫아도 계속되는 클라우드에서 세션을 실행합니다. 클라우드 세션이 어떻게 작동하는지는 [웹의 Claude Code](/docs/ko/claude-code-on-the-web)를 참조하십시오.
    * **SSH**: SSH를 통해 원격 머신(예: 자신의 서버, 클라우드 VM 또는 개발 컨테이너)에 연결합니다. Desktop은 처음 연결할 때 원격 머신에 Claude Code를 자동으로 설치합니다.
    * **WSL** (Windows): [WSL 2 배포판](/docs/ko/desktop-wsl) 내에서 세션을 실행합니다. Claude Code, 도구 및 git은 Linux 측에서 기본 경로로 실행됩니다.
  </Step>

  <Step title="모델 선택">
    전송 버튼 옆의 드롭다운에서 모델을 선택합니다. 사용 가능한 모델의 비교는 [models](/docs/ko/model-config#available-models)를 참조하십시오. 나중에 동일한 드롭다운에서 모델을 변경할 수 있습니다.
  </Step>

  <Step title="Claude에게 작업 지시">
    Claude가 수행할 작업을 입력합니다:

    * `Find a TODO comment and fix it`
    * `Add tests for the main function`
    * `Create a CLAUDE.md with instructions for this codebase`

    [session](/docs/ko/desktop#work-in-parallel-with-sessions)은 코드에 대한 Claude와의 대화입니다. 각 세션은 자신의 컨텍스트와 변경 사항을 추적합니다.
  </Step>

  <Step title="변경 사항 검토 및 수락">
    다음에 일어나는 일은 전송 버튼 옆의 선택기에 표시된 [permission mode](/docs/ko/desktop#choose-a-permission-mode)에 따라 달라집니다:

    * **Auto or Accept edits**: Claude가 파일 변경 사항을 적용하고 `+12 -1`과 같은 표시기가 나타나므로 diff 보기에서 검토할 수 있습니다.
    * **Manual**: Claude는 각 변경 사항을 제안하고 적용하기 전에 승인을 기다립니다. 파일은 수락할 때까지 수정되지 않으며, 변경 사항을 거부하면 Claude는 대신 어떻게 진행할지 묻습니다.

    Manual 모드에서는 다음을 볼 수 있습니다:

    1. 각 파일에서 정확히 무엇이 변경될지 보여주는 [diff view](/docs/ko/desktop#review-changes-with-diff-view)
    2. 각 변경 사항을 승인하거나 거부하는 Accept/Reject 버튼
    3. Claude가 요청을 처리하는 동안 실시간 업데이트
  </Step>
</Steps>

<h2 id="now-what">
  이제 어떻게 할까요?
</h2>

첫 번째 편집을 완료했습니다. Claude Code Desktop이 할 수 있는 모든 기능에 대한 전체 참고 자료는 [Claude Code Desktop 사용](/docs/ko/desktop)을 참조하세요. 다음에 시도해 볼 수 있는 몇 가지 사항입니다.

**중단 및 방향 조정.** 언제든지 Claude를 리디렉션할 수 있습니다. 중지 버튼을 클릭하여 즉시 중단하거나, 수정 사항을 입력하고 **Enter**를 눌러 실행 중인 작업을 중지하지 않고 전송할 수 있습니다. 어느 쪽이든 완료될 때까지 기다리거나 다시 시작할 필요가 없습니다.

**Claude에 더 많은 컨텍스트 제공.** 프롬프트 상자에 `@filename`을 입력하여 특정 파일을 대화에 가져오거나, 첨부 버튼을 사용하여 이미지 및 PDF를 첨부하거나, 파일을 프롬프트에 직접 드래그 앤 드롭할 수 있습니다. Claude가 더 많은 컨텍스트를 가질수록 결과가 더 좋습니다. [파일 및 컨텍스트 추가](/docs/ko/desktop#add-files-and-context-to-prompts)를 참조하세요.

**반복 가능한 작업에 스킬 사용.** `/`를 입력하거나 **+** → **Slash commands**를 클릭하여 [기본 제공 명령어](/docs/ko/commands), [사용자 정의 스킬](/docs/ko/skills) 및 플러그인 스킬을 찾아봅니다. 스킬은 코드 검토 체크리스트 또는 배포 단계와 같이 필요할 때마다 호출할 수 있는 재사용 가능한 프롬프트입니다.

**커밋하기 전에 변경 사항 검토.** Claude가 파일을 편집한 후 `+12 -1` 표시기가 나타납니다. 이를 클릭하여 [diff 보기](/docs/ko/desktop#review-changes-with-diff-view)를 열고, 파일별로 수정 사항을 검토하고, 특정 줄에 대해 댓글을 달 수 있습니다. Claude는 사용자의 댓글을 읽고 수정합니다. **Review code**를 클릭하여 Claude가 diff를 직접 평가하고 인라인 제안을 남기도록 할 수 있습니다.

**제어 수준 조정.** [권한 모드](/docs/ko/desktop#choose-a-permission-mode)는 Claude가 승인을 요청하지 않고 수행할 수 있는 작업의 양을 설정합니다:

* **Auto**: 분류기가 백그라운드에서 작업을 검토하고 사용자에게 묻는 대신 위험한 작업을 차단합니다.
* **Manual**: Claude는 파일을 편집하거나 명령어를 실행하기 전에 요청합니다.
* **Accept edits**: Claude는 더 빠른 반복을 위해 파일 편집을 자동으로 수락합니다.
* **Plan**: Claude는 파일을 편집하지 않고 접근 방식을 제안하며, 이는 대규모 리팩토링 전에 유용합니다.

**더 많은 기능을 위해 플러그인 추가.** 프롬프트 상자 옆의 **+** 버튼을 클릭하고 **Plugins**를 선택하여 스킬, 에이전트, MCP 서버 등을 추가하는 [플러그인](/docs/ko/desktop#install-plugins)을 찾아보고 설치합니다.

**작업 공간 정렬.** 채팅, diff, 터미널, 파일 및 브라우저 창을 원하는 레이아웃으로 드래그합니다. \*\*Ctrl+\`\*\*를 사용하여 터미널을 열어 세션과 함께 명령어를 실행하거나, 파일 경로를 클릭하여 파일 창에서 열 수 있습니다. [작업 공간 정렬](/docs/ko/desktop#arrange-your-workspace)을 참조하세요.

**앱 미리보기.** 데스크톱에서 개발 서버를 실행하면 앱이 브라우저 창에서 열리며, 이는 [외부 사이트를 열 수도](/docs/ko/desktop#browse-external-sites) 있습니다. Claude는 실행 중인 앱을 보고, 엔드포인트를 테스트하고, 로그를 검사하고, 보는 것에 대해 반복할 수 있습니다. [앱 미리보기](/docs/ko/desktop#preview-your-app)를 참조하세요.

**풀 요청 추적.** PR을 연 후 Claude Code는 CI 확인 결과를 모니터링하고 실패를 자동으로 수정하거나 모든 확인이 통과되면 PR을 병합할 수 있습니다. [풀 요청 상태 모니터링](/docs/ko/desktop#monitor-pull-request-status)을 참조하세요.

**Claude를 일정에 따라 실행.** [예약된 작업](/docs/ko/desktop-scheduled-tasks)을 설정하여 Claude를 정기적으로 자동으로 실행합니다: 매일 아침 일일 코드 검토, 주간 종속성 감사 또는 연결된 도구에서 정보를 가져오는 브리핑입니다.

**준비가 되면 확장.** 사이드바에서 [병렬 세션](/docs/ko/desktop#work-in-parallel-with-sessions)을 열어 여러 작업을 동시에 수행하고, 각각 자체 Git worktree에서 실행하고, [작업 창](/docs/ko/desktop#watch-background-tasks)을 열어 세션이 실행 중인 서브에이전트 및 백그라운드 명령어를 봅니다. [사이드 채팅](/docs/ko/desktop#ask-a-side-question-without-derailing-the-session)을 열어 메인 스레드를 방해하지 않고 질문을 할 수 있습니다. [장기 실행 작업을 클라우드로 전송](/docs/ko/desktop#run-long-running-tasks-in-the-cloud)하여 앱을 닫아도 계속 실행되도록 하거나, 작업이 예상보다 오래 걸리면 [웹 또는 IDE에서 세션을 계속](/docs/ko/desktop#continue-in-another-surface)할 수 있습니다. [GitHub, Slack 및 Linear와 같은 외부 도구를 연결](/docs/ko/desktop#extend-claude-code)하여 워크플로우를 통합합니다.

<h2 id="what’s-next">
  다음 단계
</h2>

* [Claude Code Desktop 사용하기](/docs/ko/desktop): 권한 모드, 병렬 세션, diff 보기, 커넥터 및 엔터프라이즈 구성
* [CLI에서 전환하시나요?](/docs/ko/desktop#coming-from-the-cli): Desktop과 CLI를 동일한 프로젝트에서 실행하고, 기능, 플래그 동등물 및 Desktop에서 사용할 수 없는 기능을 비교합니다
* [문제 해결](/docs/ko/desktop#troubleshooting): 일반적인 오류 및 설정 문제에 대한 해결책
* [모범 사례](/docs/ko/best-practices): 효과적인 프롬프트 작성 및 Claude Code를 최대한 활용하기 위한 팁
* [일반적인 워크플로우](/docs/ko/common-workflows): 디버깅, 리팩토링, 테스트 등에 대한 튜토리얼
