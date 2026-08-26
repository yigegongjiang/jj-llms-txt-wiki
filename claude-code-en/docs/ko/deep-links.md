> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 링크에서 세션 시작하기

> URL에서 Claude Code 터미널 세션을 엽니다. 런북, 알림 및 대시보드에 `claude-cli://` 링크를 포함하여 클릭하면 Claude Code가 올바른 저장소에서 올바른 프롬프트와 함께 열립니다.

딥 링크는 Claude Code를 새 터미널 창에서 여는 `claude-cli://` URL입니다. URL은 작업 디렉터리와 미리 채울 프롬프트를 전달할 수 있습니다.

이를 통해 작업을 위한 원클릭 시작점을 공유할 수 있습니다. Claude Code가 설치된 사용자가 링크를 클릭하면 프롬프트가 이미 입력된 세션이 열립니다. 프롬프트는 채워지지만 Enter를 누를 때까지 전송되지 않습니다.

딥 링크는 URL이므로 링크가 갈 수 있는 모든 곳에 배치할 수 있습니다.

* 영향을 받는 서비스의 저장소를 진단 프롬프트와 함께 여는 인시던트 런북 단계
* 특정 메트릭에 대한 조사 프롬프트로 연결되는 모니터링 알림 또는 대시보드
* 프로젝트를 온보딩 프롬프트와 함께 여는 README 또는 위키 페이지
* 실패한 작업의 이름을 미리 채우는 CI 실패 알림

이 페이지에서는 [링크를 빌드하는 방법](#build-a-link), [런북에 포함하거나 셸에서 트리거하는 방법](#examples), 그리고 [각 플랫폼에서 핸들러 등록을 관리하거나 비활성화하는 방법](#registration-and-supported-platforms)을 다룹니다.

<h2 id="how-it-works">
  작동 방식
</h2>

`claude-cli://` 접두사는 `mailto:` 링크가 이메일 클라이언트를 여는 방식과 유사하게 Claude Code가 운영 체제에 등록하는 사용자 정의 URL 스키마입니다. 링크는 웹 페이지, 위키, Slack 메시지 또는 링크를 렌더링하는 모든 앱에 있을 수 있습니다. 클릭하면:

1. 브라우저 또는 앱이 URL을 운영 체제에 전달합니다.
2. 운영 체제가 `claude-cli://` 접두사를 인식하고 컴퓨터에서 Claude Code를 시작합니다.
3. 새 터미널 창이 열리고 Claude Code가 링크가 지정한 디렉터리에서 실행되며, 링크의 프롬프트 텍스트가 이미 입력 상자에 있습니다.
4. 프롬프트를 읽고 필요하면 편집한 후 Enter를 눌러 전송합니다.

링크 자체는 어디든 호스팅될 수 있지만 세션은 항상 클릭한 컴퓨터에서 로컬로 열립니다. 각 운영 체제에서 어떤 터미널 에뮬레이터가 열리는지는 [등록 및 지원되는 플랫폼](#registration-and-supported-platforms)을 참조하세요.

<Note>
  링크를 표시하는 플랫폼은 사용자 정의 URL 스키마를 허용해야 합니다. GitHub에서 렌더링된 Markdown은 `http` 및 `https`를 허용하지만 README, 이슈, 풀 요청 및 위키에서 `claude-cli://`와 같은 스키마를 제거합니다. 링크 텍스트만 표시되고 뒤에 링크가 없으며 URL이 숨겨집니다. 해결 방법은 [문제 해결](#the-link-renders-as-plain-text-instead-of-being-clickable)을 참조하세요.
</Note>

<h3 id="what-a-launched-session-shows">
  시작된 세션이 표시하는 것
</h3>

딥 링크는 자체적으로 아무것도 실행하지 않습니다. 링크는 디렉터리를 선택하고 프롬프트 상자를 채울 뿐입니다. 신뢰하지 않는 페이지에서 링크를 클릭하면 프롬프트는 여전히 비활성 상태입니다. 채워진 내용을 읽고 Enter를 누를 때까지 모델에 아무것도 도달하지 않습니다.

세션이 열리면 입력 상자 아래의 경고 줄에 `외부 링크의 프롬프트`라고 표시되며 프롬프트를 전송하거나 지울 때까지 계속 표시됩니다. 1,000자를 초과하는 프롬프트의 경우 경고에는 문자 수가 포함되며 Enter를 누르기 전에 전체 텍스트를 스크롤하고 검토하도록 지시합니다. 긴 프롬프트는 지시사항을 화면 밖으로 밀어낼 수 있기 때문입니다. 권한 규칙, `CLAUDE.md` 및 선택한 디렉터리에 대한 신뢰 프롬프트는 다른 세션과 동일한 방식으로 적용됩니다.

<h2 id="build-a-link">
  링크 빌드하기
</h2>

모든 딥 링크는 `claude-cli://open`으로 시작하며, 이는 핸들러가 허용하는 유일한 경로이고 선택적 쿼리 매개변수가 뒤따릅니다. 최소 형식은 Claude Code를 홈 디렉터리에서 빈 프롬프트로 엽니다:

```text theme={null}
claude-cli://open
```

프롬프트 상자에 세션이 시작되는 위치와 포함되는 내용을 제어하는 매개변수를 추가합니다:

| 매개변수   | 설명                                                                                                                                                                                          |
| ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `q`    | 프롬프트 상자에 미리 채울 텍스트입니다. [URL 인코딩](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent)을 수행합니다. 여러 줄 프롬프트의 줄 바꿈에는 `%0A`를 사용합니다. 최대 5,000자입니다. |
| `cwd`  | 작업 디렉터리로 사용할 절대 경로입니다. 네트워크 및 UNC 경로는 거부되며, 보이지 않는 문자나 양방향 제어 문자가 포함된 경로도 거부됩니다.                                                                                                            |
| `repo` | GitHub `owner/name` 슬러그입니다. Claude Code는 이를 이전에 본 로컬 클론으로 확인하고 거기서 시작합니다. 일치하는 클론이 없으면 세션이 홈 디렉터리에서 대신 열립니다.                                                                                |

`cwd`와 `repo`는 [작업 디렉터리를 설정하는 두 가지 방법](#choose-between-cwd-and-repo)입니다. 둘 다 전달하면 `cwd`가 우선하고 `cwd` 경로가 존재하지 않더라도 `repo`는 무시됩니다.

다음 링크는 `acme/payments`라는 저장소를 가리키며 두 줄의 진단 프롬프트가 있습니다. 자신의 링크를 빌드할 때 `acme/payments`를 저장소의 `owner/name` 슬러그로 바꿉니다:

```text theme={null}
claude-cli://open?repo=acme/payments&q=Investigate%20the%20failed%20deploy%20of%20payments-api.%0ACheck%20recent%20commits%20to%20main%20and%20the%20last%20successful%20build.
```

클릭하면 새 터미널 창이 열리고 Claude Code가 `acme/payments`의 로컬 클론에서 시작되며 프롬프트 상자가 디코딩된 텍스트로 채워집니다:

```text theme={null}
Investigate the failed deploy of payments-api.
Check recent commits to main and the last successful build.
```

Enter를 눌러 전송하기 전에 프롬프트를 편집할 수 있습니다. 저장소의 로컬 클론이 없으면 세션이 홈 디렉터리에서 대신 열립니다. 여러 클론 또는 worktree가 있을 때 로컬 경로가 선택되는 방식은 [`cwd`와 `repo` 중 선택하기](#choose-between-cwd-and-repo)를 참조하세요.

<h3 id="choose-between-cwd-and-repo">
  `cwd`와 `repo` 중 선택하기
</h3>

`cwd`를 사용하는 경우는 링크를 클릭하는 모든 사람이 표준화된 devcontainer 또는 VM 이미지와 같은 동일한 절대 경로에 프로젝트를 가지고 있을 때입니다.

링크가 공유되고 각 사람이 다른 위치에 클론할 때 `repo`를 사용합니다. Claude Code는 슬러그를 다음과 같이 로컬 경로로 확인합니다:

* `claude`를 Git 저장소에서 실행할 때마다 해당 디렉터리의 파일 시스템 경로가 저장소의 GitHub `owner/name` 슬러그에 대해 기록됩니다.
* 딥 링크가 도착하면 `repo`는 가장 최근에 사용한 일치하는 경로를 엽니다. 여러 클론과 worktree는 별도로 추적되므로 마지막으로 작업한 경로를 선택합니다.
* 조회는 Claude Code를 최소한 한 번 이상 실행한 경로만 찾습니다.
* 링크는 체크아웃된 분기를 변경하지 않습니다. 세션은 해당 디렉터리가 현재 있는 상태로 열립니다.

시작 헤더는 선택한 경로를 표시하므로 올바른 클론이 열렸는지 확인할 수 있습니다.

<h2 id="examples">
  예제
</h2>

아래 섹션은 딥 링크를 사용하는 두 가지 일반적인 방법을 보여줍니다. 문서의 Markdown 링크와 스크립트 또는 셸 별칭의 명령입니다.

<h3 id="embed-a-link-in-a-runbook">
  런북에 링크 포함하기
</h3>

런북의 딥 링크는 분류 중인 사람에게 올바른 저장소에서 준비된 프롬프트로 조사를 시작하는 원클릭 방법을 제공합니다. 런북을 렌더링하는 플랫폼은 사용자 정의 URL 스키마를 허용해야 합니다. GitHub에서 렌더링된 Markdown은 `claude-cli://`를 허용하지 않으므로 GitHub README, 이슈 또는 위키의 딥 링크는 클릭 가능한 링크 없이 레이블만 표시합니다. 해결 방법은 [문제 해결 참고](#the-link-renders-as-plain-text-instead-of-being-clickable)를 참조하세요.

프롬프트는 URL의 일부이며 URL 인코딩되어야 합니다. 인코딩된 값을 생성하려면 프롬프트 텍스트를 브라우저 콘솔의 `encodeURIComponent` 또는 모든 URL 인코더를 통해 전달합니다.

아래 예제는 `web-gateway`라는 서비스에 대한 인시던트 런북에 조사 진입점을 추가합니다:

```markdown theme={null}
## High 5xx rate on web-gateway

1. Acknowledge the page in PagerDuty.
2. [Open Claude Code in the gateway repo](claude-cli://open?repo=acme/web-gateway&q=5xx%20rate%20is%20elevated%20on%20web-gateway.%20Check%20recent%20deploys%2C%20error%20logs%20from%20the%20last%2030%20minutes%2C%20and%20open%20incidents%20in%20Linear.)
3. Post initial findings in #incident.
```

자신의 런북에서 이를 사용하려면 `acme/web-gateway`를 서비스의 저장소 슬러그로 바꿉니다. 이를 통해 Claude Code가 설치되어 있고 해당 저장소의 로컬 클론을 가진 엔지니어는 2단계를 클릭하고 프롬프트가 전송 준비가 된 상태로 조사를 시작할 수 있습니다.

<h3 id="open-a-link-from-the-shell">
  셸에서 링크 열기
</h3>

클릭하는 대신 셸 스크립트, 별칭 또는 자동화에서 딥 링크를 열 수도 있습니다. 운영 체제의 URL 열기 명령을 링크를 인수로 호출합니다.

<Tabs>
  <Tab title="macOS">
    기본 제공 `open` 명령은 URL을 등록된 `claude-cli://` 핸들러로 전달합니다:

    ```bash theme={null}
    open "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```
  </Tab>

  <Tab title="Linux">
    대부분의 데스크톱 환경은 `xdg-open`을 제공하며, 이는 URL을 등록된 핸들러로 전달합니다:

    ```bash theme={null}
    xdg-open "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```
  </Tab>

  <Tab title="Windows">
    PowerShell에서 `Start-Process`는 URL을 등록된 핸들러로 전달합니다:

    ```powershell theme={null}
    Start-Process "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```

    `cmd.exe`에서 `start`는 첫 번째 따옴표 붙은 인수를 창 제목으로 취급하므로 URL 앞에 빈 제목을 전달합니다:

    ```cmd theme={null}
    start "" "claude-cli://open?repo=acme/payments&q=review%20open%20PRs"
    ```
  </Tab>
</Tabs>

<h2 id="registration-and-supported-platforms">
  등록 및 지원되는 플랫폼
</h2>

Claude Code는 macOS, Linux 및 Windows에서 대화형 세션을 처음 시작할 때 운영 체제에 `claude-cli://` 핸들러를 등록합니다. 별도의 설치 명령을 실행하지 않습니다. 등록은 사용자 수준 위치에만 씁니다:

| 플랫폼     | 핸들러 위치                                                                                                  |
| ------- | ------------------------------------------------------------------------------------------------------- |
| macOS   | `~/Applications/Claude Code URL Handler.app`                                                            |
| Linux   | `$XDG_DATA_HOME/applications` 아래의 `claude-code-url-handler.desktop`, 기본값은 `~/.local/share/applications` |
| Windows | `HKEY_CURRENT_USER\Software\Classes\claude-cli`                                                         |

핸들러는 감지된 터미널 에뮬레이터에서 Claude Code를 시작합니다. macOS에서 Claude Code는 가장 최근의 대화형 세션에서 터미널을 기억하고 재사용하며, iTerm2, Ghostty, kitty, Alacritty, WezTerm 및 Terminal.app을 지원합니다. Linux에서는 `$TERMINAL` 환경 변수를 준수한 다음 `x-terminal-emulator`, 그 다음 일반적인 에뮬레이터 목록을 준수합니다. Windows에서는 Windows Terminal을 선호한 다음 PowerShell, 그 다음 `cmd.exe`를 선호합니다.

등록을 완전히 방지하려면 `settings.json`에서 [`disableDeepLinkRegistration`](/docs/ko/settings)을 `"disable"`로 설정합니다. 조직 전체에 이를 적용하여 사용자가 다시 활성화할 수 없도록 하려면 [관리되는 설정](/docs/ko/server-managed-settings)에서 대신 설정합니다.

<h2 id="open-a-vs-code-tab-instead-of-a-terminal">
  터미널 대신 VS Code 탭 열기
</h2>

VS Code 확장은 `vscode://anthropic.claude-code/open`에서 자체 핸들러를 등록하며, 이는 터미널 창 대신 Claude Code 편집기 탭을 엽니다. 해당 URL의 매개변수는 [다른 도구에서 VS Code 탭 시작하기](/docs/ko/vs-code#launch-a-vs-code-tab-from-other-tools)를 참조하세요.

<h2 id="troubleshooting">
  문제 해결
</h2>

<h3 id="clicking-the-link-does-nothing">
  링크를 클릭해도 아무것도 일어나지 않음
</h3>

핸들러가 아직 등록되지 않았을 가능성이 높습니다. 해당 컴퓨터에서 대화형 `claude` 세션을 한 번 시작하고 종료한 후 링크를 다시 시도합니다. Linux에서 데스크톱 환경이 없으면 `xdg-open`이 디스패치할 것이 없을 수 있습니다.

<h3 id="the-link-renders-as-plain-text-instead-of-being-clickable">
  링크가 일반 텍스트로 렌더링되고 클릭 가능하지 않음
</h3>

일부 Markdown 렌더러는 `http` 및 `https` 링크만 허용하고 다른 URL 스키마를 제거합니다. GitHub는 README, 이슈, 풀 요청 및 위키에서 이를 수행합니다: `[label](claude-cli://...)`는 링크 없이 `label`로만 렌더링되고 URL이 제거됩니다. 이러한 플랫폼에서는 딥 링크를 코드 블록에 넣어 독자가 URL을 보고 브라우저의 주소 표시줄에 붙여넣을 수 있도록 합니다.

<h3 id="the-session-opens-in-my-home-directory-instead-of-the-repo">
  세션이 저장소 대신 홈 디렉터리에서 열림
</h3>

`repo` 매개변수는 Claude Code가 이미 본 클론으로만 확인됩니다. 클론 내에서 `claude`를 한 번 실행하여 경로를 기록하거나 링크를 절대 경로와 함께 `cwd`를 사용하도록 전환합니다.

<h3 id="the-link-opens-the-wrong-terminal">
  링크가 잘못된 터미널을 열음
</h3>

macOS에서 선호하는 터미널에서 `claude`를 시작하면 다음 딥 링크가 이를 사용합니다. Linux에서는 `$TERMINAL` 환경 변수를 선호하는 에뮬레이터의 명령 이름으로 설정합니다. Windows에서는 순서가 고정되어 있습니다. PowerShell 또는 `cmd.exe` 창 대신 링크가 거기서 열리도록 하려면 Windows Terminal을 설치합니다.

<h2 id="learn-more">
  자세히 알아보기
</h2>

이 페이지들은 Claude Code 세션을 시작하거나 확장하는 관련 방법을 다룹니다:

* [Skills](/docs/ko/skills): 긴 런북 프롬프트를 저장소의 `/skill`로 저장하여 딥 링크의 `q` 매개변수가 이름만 지정하면 됩니다
* [Non-interactive mode](/docs/ko/headless): 스크립트에서 Claude를 실행하고 터미널을 열지 않고 출력을 캡처합니다
