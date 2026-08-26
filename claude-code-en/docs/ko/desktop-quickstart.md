> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 데스크톱 앱 시작하기

> 데스크톱에 Claude Code를 설치하고 첫 번째 코딩 세션을 시작합니다

데스크톱 앱은 여러 세션을 나란히 실행하도록 구축된 그래픽 인터페이스를 갖춘 Claude Code를 제공합니다: 병렬 작업을 관리하기 위한 사이드바, 통합 터미널 및 파일 편집기가 있는 드래그 앤 드롭 레이아웃, 시각적 diff 검토, 라이브 앱 미리보기, GitHub PR 모니터링 및 자동 병합, 그리고 예약된 작업입니다. 터미널이 필요하지 않습니다.

<CardGroup cols={3}>
  <Card title="Download for macOS" icon="apple" href="https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code&utm_medium=docs">
    Universal build for Intel and Apple Silicon
  </Card>

  <Card title="Download for Windows" icon="windows" href="https://claude.ai/api/desktop/win32/x64/setup/latest/redirect?utm_source=claude_code&utm_medium=docs">
    For x64 processors
  </Card>

  <Card title="Get Claude for Linux (beta)" icon="linux" href="/docs/en/desktop-linux">
    apt or .deb for Ubuntu and Debian
  </Card>
</CardGroup>

For Windows ARM64, download the [ARM64 installer](https://claude.ai/api/desktop/win32/arm64/setup/latest/redirect?utm_source=claude_code\&utm_medium=docs). On Linux, install with apt; see [Claude Desktop on Linux](/docs/en/desktop-linux).

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
  첫 번째 세션 시작
</h2>

Code 탭이 열려 있으면 프로젝트를 선택하고 Claude에게 할 일을 지시합니다.

<Steps>
  <Step title="환경 및 폴더 선택">
    **Local**을 선택하여 파일을 직접 사용하여 머신에서 Claude를 실행합니다. **Select folder**를 클릭하고 프로젝트 디렉토리를 선택합니다.

    <Tip>
      잘 알고 있는 작은 프로젝트부터 시작하세요. Claude Code가 할 수 있는 일을 보는 가장 빠른 방법입니다. Windows에서는 로컬 세션이 작동하려면 [Git](https://git-scm.com/downloads/win)이 설치되어 있어야 합니다. 대부분의 Mac에는 기본적으로 Git이 포함되어 있습니다.
    </Tip>

    다음을 선택할 수도 있습니다:

    * **Remote**: Anthropic의 클라우드 인프라에서 세션을 실행하며, 앱을 닫아도 계속됩니다. 클라우드 세션은 [웹의 Claude Code](/docs/ko/claude-code-on-the-web)와 동일한 인프라를 사용합니다.
    * **SSH**: SSH를 통해 원격 머신(자신의 서버, 클라우드 VM 또는 dev 컨테이너)에 연결합니다. Desktop은 처음 연결할 때 원격 머신에 Claude Code를 자동으로 설치합니다.
    * **WSL** (Windows): [WSL 2 배포판](/docs/ko/desktop-wsl) 내에서 세션을 실행합니다. Claude Code, 도구 및 git은 Linux 측에서 기본 경로로 실행됩니다.
  </Step>

  <Step title="모델 선택">
    전송 버튼 옆의 드롭다운에서 모델을 선택합니다. 사용 가능한 모델의 비교는 [모델](/docs/ko/model-config#available-models)을 참조하세요. 나중에 동일한 드롭다운에서 모델을 변경할 수 있습니다.
  </Step>

  <Step title="Claude에게 할 일 지시">
    Claude가 할 일을 입력합니다:

    * `Find a TODO comment and fix it`
    * `Add tests for the main function`
    * `Create a CLAUDE.md with instructions for this codebase`

    [세션](/docs/ko/desktop#work-in-parallel-with-sessions)은 코드에 대한 Claude와의 대화입니다. 각 세션은 자신의 컨텍스트와 변경 사항을 추적하므로 여러 작업을 동시에 수행할 수 있으며 서로 간섭하지 않습니다.
  </Step>

  <Step title="변경 사항 검토 및 수락">
    기본적으로 Code 탭은 [Manual mode](/docs/ko/desktop#choose-a-permission-mode)에서 시작되며, Claude는 변경 사항을 제안하고 적용하기 전에 승인을 기다립니다. 다음을 볼 수 있습니다:

    1. 각 파일에서 정확히 무엇이 변경될지 보여주는 [diff 보기](/docs/ko/desktop#review-changes-with-diff-view)
    2. 각 변경 사항을 승인하거나 거부하는 Accept/Reject 버튼
    3. Claude가 요청을 처리하면서 실시간 업데이트

    변경 사항을 거부하면 Claude는 다르게 진행하고 싶은 방법을 묻습니다. 승인할 때까지 파일이 수정되지 않습니다.
  </Step>
</Steps>

<h2 id="now-what">
  이제 뭘 할까요?
</h2>

첫 번째 편집을 완료했습니다. Desktop이 할 수 있는 모든 것에 대한 전체 참조는 [Claude Code Desktop 사용](/docs/ko/desktop)을 참조하세요. 다음으로 시도할 수 있는 몇 가지 사항입니다.

**중단 및 조정.** 언제든지 Claude를 중단할 수 있습니다. 중지 버튼을 클릭하여 즉시 중단하거나, 수정 사항을 입력하고 **Enter**를 눌러 실행 중인 작업을 중단하지 않고 전송합니다. 어느 쪽이든 완료될 때까지 기다리거나 다시 시작할 필요가 없습니다.

**Claude에게 더 많은 컨텍스트 제공.** 프롬프트 상자에 `@filename`을 입력하여 특정 파일을 대화에 가져오거나, 첨부 버튼을 사용하여 이미지 및 PDF를 첨부하거나, 파일을 프롬프트에 직접 드래그 앤 드롭합니다. Claude가 더 많은 컨텍스트를 가질수록 결과가 더 좋습니다. [파일 및 컨텍스트 추가](/docs/ko/desktop#add-files-and-context-to-prompts)를 참조하세요.

**반복 가능한 작업에 skills 사용.** `/`를 입력하거나 **+** → **Slash commands**를 클릭하여 [내장 명령](/docs/ko/commands), [사용자 정의 skills](/docs/ko/skills), 플러그인 skills를 찾아봅니다. Skills는 코드 검토 체크리스트나 배포 단계와 같이 필요할 때마다 호출할 수 있는 재사용 가능한 프롬프트입니다.

**커밋하기 전에 변경 사항 검토.** Claude가 파일을 편집한 후 `+12 -1` 표시기가 나타납니다. 이를 클릭하여 [diff 보기](/docs/ko/desktop#review-changes-with-diff-view)를 열고, 파일별로 수정 사항을 검토하고, 특정 줄에 대해 댓글을 달 수 있습니다. Claude는 댓글을 읽고 수정합니다. **Review code**를 클릭하여 Claude가 diff를 평가하고 인라인 제안을 남기도록 합니다.

**제어 수준 조정.** [권한 모드](/docs/ko/desktop#choose-a-permission-mode)는 Claude가 승인을 요청하지 않고 할 수 있는 작업의 양을 설정합니다:

* **Manual**: 기본값입니다. Claude는 파일을 편집하거나 명령을 실행하기 전에 승인을 요청합니다.
* **Accept edits**: Claude는 파일 편집을 자동으로 수락하여 더 빠른 반복을 가능하게 합니다.
* **Plan**: Claude는 파일을 편집하지 않고 접근 방식을 제안하며, 이는 큰 리팩토링 전에 유용합니다.

**더 많은 기능을 위해 플러그인 추가.** 프롬프트 상자 옆의 **+** 버튼을 클릭하고 **Plugins**를 선택하여 skills, 에이전트, MCP servers 등을 추가하는 [플러그인](/docs/ko/desktop#install-plugins)을 찾아보고 설치합니다.

**워크스페이스 정렬.** 채팅, diff, 터미널, 파일, 브라우저 창을 원하는 레이아웃으로 드래그합니다. \*\*Ctrl+\`\*\*로 터미널을 열어 세션과 함께 명령을 실행하거나, 파일 경로를 클릭하여 파일 창에서 엽니다. [워크스페이스 정렬](/docs/ko/desktop#arrange-your-workspace)을 참조하세요.

**앱 미리보기.** Desktop에서 dev 서버를 실행하면 앱이 Browser 창에서 열리며, 이는 [외부 사이트를 열 수도](/docs/ko/desktop#browse-external-sites) 있습니다. Claude는 실행 중인 앱을 보고, 엔드포인트를 테스트하고, 로그를 검사하고, 보는 것에 대해 반복할 수 있습니다. [앱 미리보기](/docs/ko/desktop#preview-your-app)를 참조하세요.

**pull request 추적.** PR을 연 후 Claude Code는 CI 확인 결과를 모니터링하고 실패를 자동으로 수정하거나 모든 확인이 통과되면 PR을 자동으로 병합할 수 있습니다. [pull request 상태 모니터링](/docs/ko/desktop#monitor-pull-request-status)을 참조하세요.

**Claude를 일정에 따라 실행.** [예약된 작업](/docs/ko/desktop-scheduled-tasks)을 설정하여 Claude를 정기적으로 자동으로 실행합니다: 매일 아침 일일 코드 검토, 주간 종속성 감사, 또는 연결된 도구에서 정보를 가져오는 브리핑입니다.

**준비가 되면 확장.** 사이드바에서 [병렬 세션](/docs/ko/desktop#work-in-parallel-with-sessions)을 열어 여러 작업을 동시에 수행하며, 각각 자신의 Git worktree에서 실행하고, [작업 창](/docs/ko/desktop#watch-background-tasks)을 열어 세션이 실행 중인 subagents 및 백그라운드 명령을 봅니다. [side chat](/docs/ko/desktop#ask-a-side-question-without-derailing-the-session)을 열어 메인 스레드를 방해하지 않고 질문을 합니다. [장기 실행 작업을 클라우드로 보내](/docs/ko/desktop#run-long-running-tasks-remotely) 앱을 닫아도 계속되도록 하거나, 작업이 예상보다 오래 걸리면 [웹 또는 IDE에서 세션을 계속](/docs/ko/desktop#continue-in-another-surface)합니다. [GitHub, Slack, Linear와 같은 외부 도구를 연결](/docs/ko/desktop#extend-claude-code)하여 워크플로우를 통합합니다.

<h2 id="coming-from-the-cli">
  CLI에서 오셨나요?
</h2>

Desktop은 그래픽 인터페이스를 갖춘 CLI와 동일한 엔진을 실행합니다. 동일한 프로젝트에서 둘 다 동시에 실행할 수 있으며, 구성(CLAUDE.md 파일, MCP servers, hooks, skills, 설정)을 공유합니다. 기능, 플래그 동등물, Desktop에서 사용할 수 없는 것의 전체 비교는 [CLI 비교](/docs/ko/desktop#coming-from-the-cli)를 참조하세요.

<h2 id="what’s-next">
  다음 단계
</h2>

* [Claude Code Desktop 사용](/docs/ko/desktop): 권한 모드, 병렬 세션, diff 보기, 커넥터, 엔터프라이즈 구성
* [문제 해결](/docs/ko/desktop#troubleshooting): 일반적인 오류 및 설정 문제에 대한 해결책
* [모범 사례](/docs/ko/best-practices): 효과적인 프롬프트 작성 및 Claude Code 활용을 위한 팁
* [일반적인 워크플로우](/docs/ko/common-workflows): 디버깅, 리팩토링, 테스트 등에 대한 튜토리얼
