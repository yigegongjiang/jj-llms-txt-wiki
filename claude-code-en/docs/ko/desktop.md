> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Desktop 애플리케이션

> Claude Code Desktop을 더 활용하기: Git 격리를 통한 병렬 세션, 드래그 앤 드롭 패널 레이아웃, 통합 터미널 및 파일 편집기, 사이드 채팅, 컴퓨터 사용, 휴대폰에서 Dispatch 세션 전송, 시각적 diff 검토, 앱 미리보기, PR 모니터링, 커넥터, 엔터프라이즈 구성.

Claude Desktop 앱에는 세 개의 탭이 있습니다: 대화를 위한 **Chat**, [Dispatch 및 더 긴 에이전트 작업](https://claude.com/product/cowork)을 위한 **Cowork**, 소프트웨어 개발을 위한 **Code**입니다. 이 페이지는 Code 탭에 대한 참고 자료입니다.

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

설치 후 Claude를 실행하고, 로그인한 다음 **Code** 탭을 클릭합니다. Windows에서 처음 열 때는 [Git for Windows](https://git-scm.com/downloads/win)가 설치되어 있어야 하며, 설치 후 앱을 다시 시작합니다. 첫 번째 세션의 전체 안내는 [시작하기 가이드](/docs/ko/desktop-quickstart)를 참조하세요.

Code 탭에서 각 대화는 **세션**입니다: 자신의 채팅 기록, 프로젝트 폴더, 코드 변경 사항을 가지고 있으며, 다른 세션과는 독립적입니다. 사이드바에는 세션이 나열되어 있으며 여러 세션을 병렬로 실행할 수 있습니다. 세션 내에서 다음을 수행할 수 있습니다:

* [diff를 검토하고 댓글 달기](#review-changes-with-diff-view), 그 다음 [CI를 통해 결과 PR 모니터링](#monitor-pull-request-status)
* [실행 중인 앱 미리보기](#preview-your-app) (Claude가 자신의 변경 사항을 확인하는 동안 내장 브라우저에서)
* [패널 정렬](#arrange-your-workspace) (채팅, diff, 미리보기, 터미널, 파일 편집기를 나란히 배치)
* [세션을 벗어나지 않고 부가 질문 하기](#ask-a-side-question-without-derailing-the-session) (세션의 컨텍스트를 사용하되 벗어나지 않음)
* [외부 도구 연결](#connect-external-tools) (GitHub, Slack, Linear 등)
* Claude가 [앱을 열고 화면을 제어](#let-claude-use-your-computer)하도록 허용
* 머신에서, [클라우드](#run-long-running-tasks-remotely)에서, 또는 [SSH](#ssh-sessions)를 통해 실행

[예약된 반복 작업](/docs/ko/desktop-scheduled-tasks), [키보드 단축키](#keyboard-shortcuts), 또는 [휴대폰에서 작업 전송](#sessions-from-dispatch)에 대해서는 연결된 페이지 및 섹션을 참조하세요. 이미 터미널 기반 CLI를 사용 중이라면, [CLI 비교](#coming-from-the-cli)에서 어떤 것이 이월되는지 확인하세요.

<h2 id="start-a-session">
  세션 시작하기
</h2>

첫 번째 메시지를 보내기 전에 프롬프트 영역에서 네 가지를 구성하세요:

* **환경**: Claude가 실행되는 위치를 선택합니다. 자신의 머신의 경우 **Local**, Anthropic 호스팅 클라우드 세션의 경우 **Remote**, 관리하는 원격 머신의 경우 [**SSH 연결**](#ssh-sessions)을 선택하거나, Windows의 경우 [**WSL 배포판**](/docs/ko/desktop-wsl)을 선택합니다. [환경 구성](#environment-configuration)을 참조하세요.
* **프로젝트 폴더**: Claude가 작업할 폴더 또는 저장소를 선택합니다. 원격 세션의 경우 [여러 저장소](#run-long-running-tasks-remotely)를 추가할 수 있습니다.
* **모델**: 전송 버튼 옆의 드롭다운에서 [모델](/docs/ko/model-config#available-models)을 선택합니다. 세션 중에 이를 변경할 수 있습니다.
* **권한 모드**: [모드 선택기](#choose-a-permission-mode)에서 Claude가 가질 자율성을 선택합니다. 세션 중에 이를 변경할 수 있습니다.

작업을 입력하고 **Enter**를 눌러 시작합니다. 각 세션은 자신의 컨텍스트와 변경 사항을 독립적으로 추적합니다.

<h2 id="work-with-code">
  코드 작업하기
</h2>

Claude에게 올바른 컨텍스트를 제공하고, 자동으로 수행할 작업의 양을 제어하고, 변경 사항을 검토합니다.

<h3 id="use-the-prompt-box">
  프롬프트 상자 사용하기
</h3>

Claude가 수행할 작업을 입력하고 **Enter**를 눌러 보냅니다. Claude는 프로젝트 파일을 읽고, 변경 사항을 만들고, [권한 모드](#choose-a-permission-mode)에 따라 명령을 실행합니다. 언제든지 Claude를 중단할 수 있습니다: 중지 버튼을 클릭하여 즉시 중단하거나, 수정 사항을 입력하고 **Enter**를 눌러 실행 중인 작업을 중지하지 않고 보냅니다. Claude는 현재 작업이 완료되면 수정 사항을 읽고 다음 단계 전에 조정합니다.

프롬프트 상자 옆의 **+** 버튼을 클릭하면 파일 첨부, [skills](#use-skills), [connectors](#connect-external-tools), [plugins](#install-plugins)에 액세스할 수 있습니다.

<h3 id="add-files-and-context-to-prompts">
  프롬프트에 파일 및 컨텍스트 추가하기
</h3>

프롬프트 상자는 외부 컨텍스트를 가져오는 두 가지 방법을 지원합니다:

* **@mention 파일**: `@` 다음에 파일 이름을 입력하여 파일을 대화 컨텍스트에 추가합니다. Claude는 그 파일을 읽고 참조할 수 있습니다. @mention은 클라우드 또는 WSL 세션에서 사용할 수 없습니다.
* **파일 첨부**: 첨부 버튼을 사용하여 이미지, PDF 및 기타 파일을 프롬프트에 첨부하거나, 파일을 프롬프트에 직접 드래그 앤 드롭합니다. 이는 버그 스크린샷, 디자인 목업 또는 참고 문서를 공유하는 데 유용합니다.

<h3 id="choose-a-permission-mode">
  권한 모드 선택하기
</h3>

권한 모드는 세션 중에 Claude가 가질 자율성을 제어합니다: 파일 편집, 명령 실행 또는 둘 다 전에 묻는지 여부입니다. 전송 버튼 옆의 모드 선택기를 사용하여 언제든지 모드를 전환할 수 있습니다. Claude가 수행하는 작업을 정확히 보기 위해 Manual로 시작한 다음, 편하면 Accept edits 또는 Plan으로 이동합니다.

새 로컬 세션의 기본 모드를 설정하려면 [설정 파일](/docs/ko/settings#settings-files)에 `permissions.defaultMode`를 추가합니다. 데스크톱 앱은 CLI와 동일한 설정 파일을 읽습니다. 선택기에서 선택한 모드는 폴더별로 기억되며 해당 폴더의 `defaultMode`보다 우선하지만, Plan은 현재 세션에만 적용됩니다.

| 모드                     | 설정 키                | 동작                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| ---------------------- | ------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Manual**             | `default`           | Claude는 파일을 편집하거나 명령을 실행하기 전에 요청합니다. diff를 보고 각 변경 사항을 수락하거나 거부할 수 있습니다. 새 사용자에게 권장됩니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| **Accept edits**       | `acceptEdits`       | Claude는 파일 편집을 자동으로 수락하고 `mkdir`, `touch`, `mv`와 같은 일반적인 파일시스템 명령을 자동으로 수락하지만 다른 터미널 명령 실행 전에는 여전히 요청합니다. 파일 변경을 신뢰하고 더 빠른 반복을 원할 때 사용합니다.                                                                                                                                                                                                                                                                                                                                                                                                                  |
| **Plan**               | `plan`              | Claude는 파일을 읽고 명령을 실행하여 탐색한 다음 소스 코드를 편집하지 않고 계획을 제안합니다. 먼저 접근 방식을 검토하려는 복잡한 작업에 좋습니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| **Auto**               | `auto`              | Claude는 요청과의 정렬을 확인하는 백그라운드 안전 검사를 통해 모든 작업을 실행합니다. 감시를 유지하면서 권한 프롬프트를 줄입니다. 계정이 아래의 [가용성 요구 사항](#auto-mode-availability)을 충족할 때 나타나며, 별도의 Settings 토글이 없습니다.                                                                                                                                                                                                                                                                                                                                                                                               |
| **Bypass permissions** | `bypassPermissions` | Claude는 명시적 [ask rules](/docs/ko/permissions#manage-permissions), connector 도구 [조직이 `ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools), [`requiresUserInteraction`](/docs/ko/mcp#require-approval-for-a-specific-tool)으로 표시된 MCP 도구, 또는 Claude가 [외부 사이트에서 작동](#browse-external-sites)할 때 안전 분류기에 의해 강제된 권한 프롬프트를 제외하고 권한 프롬프트 없이 실행됩니다. CLI의 `--dangerously-skip-permissions`와 동일합니다. Pro 및 Max 플랜에서는 Settings → Claude Code의 "Allow bypass permissions mode"에서 활성화합니다. Team 및 Enterprise 플랜에서는 Settings 토글이 없으며 조직 정책이 대신 제어합니다. 샌드박스 컨테이너 또는 VM에서만 사용합니다. |

이전 버전의 Code 탭은 이러한 모드를 Ask permissions, Auto accept edits, Plan mode로 표시했습니다.

`dontAsk` 권한 모드는 [CLI](/docs/ko/permission-modes#allow-only-pre-approved-tools-with-dontask-mode)에서만 사용 가능합니다.

<span id="auto-mode-availability" />

Auto mode는 Anthropic API의 모든 사용자에게 제공되며 Claude Opus 4.6 이상 또는 Sonnet 4.6 이상이 필요합니다. 조직 관리자는 [관리 설정](#managed-settings)에서 `disableAutoMode` 키를 사용하여 auto mode를 끌 수 있습니다.

Google Cloud의 Agent Platform으로 라우팅하는 Enterprise 배포에서는 auto mode가 [기본적으로 사용 가능](/docs/ko/permission-modes#enable-auto-mode-on-bedrock-agent-platform-or-foundry)하며, 거기서는 Claude Sonnet 5, Opus 4.7, Opus 4.8만 지원됩니다. Claude Code v2.1.207 이전에는 Google Cloud의 Agent Platform의 Enterprise 배포에서 auto mode를 활성화하기 위해 `CLAUDE_CODE_ENABLE_AUTO_MODE`를 설정해야 했습니다.

<Tip title="모범 사례">
  복잡한 작업을 Plan에서 시작하여 Claude가 변경하기 전에 접근 방식을 매핑하도록 합니다. 계획을 승인한 후 Accept edits 또는 Manual로 전환하여 실행합니다. 이 워크플로우에 대한 자세한 내용은 [먼저 탐색, 그 다음 계획, 그 다음 코드](/docs/ko/best-practices#explore-first-then-plan-then-code)를 참조하세요.
</Tip>

클라우드 세션은 Accept edits, Plan, Auto를 지원합니다. Accept edits는 `default` 모드에 해당합니다: 클라우드 세션은 파일 편집을 미리 승인하므로 선택기는 Manual 대신 Accept edits를 표시합니다. Bypass permissions는 클라우드 환경이 이미 샌드박스되어 있기 때문에 사용할 수 없습니다.

엔터프라이즈 관리자는 사용 가능한 권한 모드를 제한할 수 있습니다. 자세한 내용은 [엔터프라이즈 구성](#enterprise-configuration)을 참조하세요.

<h3 id="preview-your-app">
  앱 미리보기
</h3>

Claude는 개발 서버를 시작하고 Browser 패널에서 열어 변경 사항을 확인할 수 있습니다. 이는 프론트엔드 웹 앱뿐만 아니라 백엔드 서버에도 작동합니다: Claude는 API 엔드포인트를 테스트하고, 서버 로그를 보고, 발견한 문제를 반복할 수 있습니다. 대부분의 경우 Claude는 프로젝트 파일을 편집한 후 자동으로 서버를 시작합니다. 언제든지 Claude에게 미리보기를 요청할 수도 있습니다. 기본적으로 Claude는 모든 편집 후 [자동으로 변경 사항을 확인](#auto-verify-changes)합니다.

Browser 패널은 프로젝트의 정적 HTML 파일, PDF, 이미지 및 비디오도 열 수 있습니다. 채팅에서 HTML, PDF, 이미지 또는 비디오 경로를 클릭하여 Browser 패널에서 엽니다.

Browser 패널에서 다음을 수행할 수 있습니다:

* Browser 패널에서 실행 중인 앱과 직접 상호작용합니다
* Claude가 자동으로 자신의 변경 사항을 확인하는 것을 봅니다: 스크린샷을 찍고, DOM을 검사하고, 요소를 클릭하고, 양식을 채우고, 발견한 문제를 수정합니다
* 세션 도구 모음의 서버 드롭다운에서 서버를 시작하거나 중지합니다
* 드롭다운에서 **Persist sessions**을 선택하여 서버 재시작 시 쿠키 및 로컬 스토리지를 유지하므로 개발 중에 다시 로그인할 필요가 없습니다
* 서버 구성을 편집하거나 모든 서버를 한 번에 중지합니다

Claude는 프로젝트를 기반으로 초기 서버 구성을 만듭니다. 앱이 사용자 정의 개발 명령을 사용하는 경우 `.claude/launch.json`을 편집하여 설정과 일치시킵니다. 전체 참조는 [미리보기 서버 구성](#configure-preview-servers)을 참조하세요.

저장된 세션 데이터를 지우거나 Browser를 완전히 비활성화하려면 Settings → Claude Code에서 토글을 사용합니다.

<h3 id="browse-external-sites">
  외부 사이트 탐색하기
</h3>

Browser 패널은 탭 브라우저이므로 실행 중인 앱 옆에 문서, 이슈 추적기 또는 다른 사이트를 열 수 있습니다. Browser를 열려면 macOS에서 **Cmd+Shift+B**를 누르거나 Windows에서 **Ctrl+Shift+B**를 누르거나 **Views** 메뉴에서 선택합니다. 채팅에서 외부 링크를 클릭하면 선택기가 Browser 패널을 사용하려면 **Open in app**을 또는 자신의 브라우저를 사용하려면 **Default browser**를 제공합니다; macOS에서 **Cmd**-클릭 또는 Windows에서 **Ctrl**-클릭하면 시스템 브라우저에서 직접 링크를 엽니다. Google OAuth와 같은 팝업 로그인 흐름을 포함하여 패널의 사이트에 로그인할 수 있습니다.

Claude는 [앱을 확인](#preview-your-app)하는 데 사용하는 동일한 도구를 사용하여 외부 페이지를 읽고 상호작용할 수 있으며, 두 가지 추가 안전 검사가 있습니다:

* 안전 분류기는 모든 권한 모드에서 클릭 및 입력과 같은 외부 페이지에 대한 Claude의 쓰기 작업을 검토합니다. 이는 [auto mode](#choose-a-permission-mode)가 사용하는 동일한 분류기이며, 작업에 플래그를 지정하면 모드에 관계없이 권한 프롬프트를 받습니다.
* Auto 및 Bypass permissions 이외의 권한 모드에서는 Claude가 새 사이트로 이동하기 전에 도메인 허용 목록 확인도 적용됩니다.

<h4 id="approve-claude’s-actions-on-a-site">
  사이트에서 Claude의 작업 승인하기
</h4>

Claude가 처음으로 외부 사이트에서 작동할 때 권한 카드가 나타나고 Claude는 선택을 기다립니다: **Allow once**, **Always allow**, 또는 **Deny**. **Allow once**는 아무것도 저장하지 않고 작업을 승인합니다. **Always allow**는 장치에 해당 사이트에 대한 승인을 저장하며, Settings에서 취소할 수 있습니다. 하위 도메인을 포함하여 각 사이트는 자체 승인이 필요합니다. 로컬 개발 서버 및 프로젝트 파일은 승인이 필요하지 않으므로 [auto-verify](#auto-verify-changes)는 프롬프트 없이 계속 작동합니다.

승인된 사이트에서도 Claude는 입력 없이 항목을 구매하거나, 계정을 만들거나, CAPTCHA를 우회하지 않습니다. Browser 패널에서 탐색하면 [Claude in Chrome extension](/docs/ko/chrome)과 동일한 안전 모델을 사용합니다. Claude가 민감한 사이트 및 위험한 작업을 처리하는 방법은 [Using Claude in Chrome safely](https://support.claude.com/en/articles/12902428-using-claude-in-chrome-safely)를 참조하세요.

<h4 id="choose-between-the-browser-and-the-chrome-extension">
  Browser와 Chrome 확장 프로그램 중 선택하기
</h4>

Browser 패널은 개인 브라우저와 별도의 깨끗한 브라우저 프로필을 사용하며 저장된 로그인이나 기록이 없습니다. 앱을 빌드 및 테스트하고 신원이 필요하지 않은 사이트에 사용합니다. Claude가 로그인한 세션에서 사용자로 작동하기를 원할 때는 대신 [Claude in Chrome extension](/docs/ko/chrome)을 사용하세요. 이는 브라우저의 로그인 상태를 공유합니다.

<h4 id="restrict-external-browsing-for-your-organization">
  조직의 외부 탐색 제한하기
</h4>

Browser는 Claude in Chrome 확장 프로그램과 동일한 [site allowlist and blocklist controls](https://support.claude.com/en/articles/13065128-claude-in-chrome-admin-controls)를 따릅니다. 조직이 이미 확장 프로그램에 대해 해당 목록을 구성한 경우 Browser는 자동으로 이를 준수합니다. 관리자는 [`browserExternalPageTools` managed setting](#managed-settings)으로 외부 페이지에서 Claude의 도구를 끌 수도 있습니다. 도구가 비활성화되면 사용자는 여전히 외부 사이트로 이동할 수 있습니다; Claude의 도구는 이를 읽거나 작동할 수 없습니다.

외부 탐색을 완전히 끄려면 [`disableBrowserExternalNavigation` managed setting](#managed-settings)을 `true`로 설정합니다. 이는 조직의 허용 목록에 있는 사이트를 포함하여 Browser의 모든 외부 탐색을 차단합니다; localhost 개발 서버 및 파일 미리보기는 계속 작동합니다. `browserExternalPageTools`를 사용하여 사용자가 Claude의 도구 없이 외부 사이트를 계속 탐색하도록 하고, `disableBrowserExternalNavigation`을 사용하여 사용자와 Claude 모두에 대해 외부 사이트를 차단합니다.

<h3 id="review-changes-with-diff-view">
  diff 보기로 변경 사항 검토하기
</h3>

Claude가 코드를 변경한 후 diff 보기를 사용하면 pull request를 만들기 전에 파일별로 수정 사항을 검토할 수 있습니다.

Claude가 파일을 변경하면 `+12 -1`과 같이 추가 및 제거된 줄 수를 표시하는 diff 통계 표시기가 나타납니다. 이 표시기를 클릭하여 diff 뷰어를 열면 왼쪽에 파일 목록이 표시되고 오른쪽에 각 파일의 변경 사항이 표시됩니다.

특정 줄에 댓글을 달려면 diff의 모든 줄을 클릭하여 댓글 상자를 엽니다. 피드백을 입력하고 **Enter**를 눌러 댓글을 추가합니다. 여러 줄에 댓글을 추가한 후 모든 댓글을 한 번에 제출합니다:

* **macOS**: **Cmd+Enter** 누르기
* **Windows**: **Ctrl+Enter** 누르기

Claude는 댓글을 읽고 요청된 변경 사항을 만들며, 이는 검토할 수 있는 새로운 diff로 나타납니다.

<h3 id="review-your-code">
  코드 검토하기
</h3>

diff 보기에서 오른쪽 상단 도구 모음의 **Review code**를 클릭하여 Claude에게 커밋하기 전에 변경 사항을 평가하도록 요청합니다. Claude는 현재 diff를 검토하고 diff 보기에 직접 댓글을 남깁니다. 모든 댓글에 응답하거나 Claude에게 수정을 요청할 수 있습니다.

검토는 높은 신호 문제에 중점을 둡니다: 컴파일 오류, 명확한 논리 오류, 보안 취약점, 명백한 버그입니다. 스타일, 형식, 기존 문제 또는 linter가 포착할 수 있는 것은 플래그하지 않습니다.

<h3 id="monitor-pull-request-status">
  pull request 상태 모니터링하기
</h3>

pull request를 연 후 CI 상태 표시줄이 세션에 나타납니다. Claude Code는 GitHub CLI를 사용하여 확인 결과를 폴링하고 실패를 표시합니다.

* **Auto-fix**: 활성화되면 Claude는 실패 출력을 읽고 반복하여 실패한 CI 확인을 자동으로 수정하려고 시도합니다.
* **Auto-merge**: 활성화되면 모든 확인이 통과하면 Claude가 PR을 병합합니다. 병합 방법은 squash입니다. Auto-merge는 이 작업을 수행하기 위해 [GitHub 저장소 설정에서 활성화](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/managing-auto-merge-for-pull-requests-in-your-repository)되어야 합니다.

CI 상태 표시줄의 **Auto-fix** 및 **Auto-merge** 토글을 사용하여 옵션을 활성화합니다. Claude Code는 CI가 완료되면 데스크톱 알림도 보냅니다. 세션이 PR을 병합하거나 닫은 후 자동으로 아카이브되도록 하려면 Settings → Claude Code에서 [auto-archive](#work-in-parallel-with-sessions)를 켭니다.

<Note>
  PR 모니터링에는 [GitHub CLI (`gh`)](https://cli.github.com/)가 머신에 설치되고 인증되어야 합니다. `gh`가 설치되지 않은 경우 Desktop은 처음으로 PR을 만들려고 할 때 설치하도록 요청합니다.
</Note>

<h2 id="arrange-your-workspace">
  워크스페이스 정렬하기
</h2>

Code 탭은 모든 레이아웃으로 정렬할 수 있는 패널을 중심으로 구축되어 있습니다: 채팅, diff, 브라우저, 터미널, 파일, 계획, 작업, 서브에이전트. 패널을 헤더로 드래그하여 위치를 변경하거나 패널 가장자리를 드래그하여 크기를 조정합니다. macOS에서 \*\*Cmd+\\\*\*를 누르거나 Windows에서 \*\*Ctrl+\\\*\*를 눌러 포커스된 패널을 닫습니다. 세션 도구 모음의 **Views** 메뉴에서 추가 패널을 엽니다.

<Note>
  이 섹션의 패널 레이아웃, 터미널, 파일 편집기, 보기 모드는 Claude Desktop v1.2581.0 이상이 필요합니다. macOS에서 **Claude → Check for Updates**를 열거나 Windows에서 **Help → Check for Updates**를 열어 업데이트합니다.
</Note>

<h3 id="run-commands-in-the-terminal">
  터미널에서 명령 실행하기
</h3>

통합 터미널을 사용하면 다른 앱으로 전환하지 않고 세션과 함께 명령을 실행할 수 있습니다. **Views** 메뉴에서 열거나 macOS 또는 Windows에서 \*\*Ctrl+\`\*\*를 누릅니다. 터미널은 세션의 작업 디렉토리에서 열리고 Claude와 동일한 환경을 공유하므로 `npm test` 또는 `git status`와 같은 명령은 Claude가 편집하는 것과 동일한 파일을 봅니다. 두 번째 터미널 탭을 열려면 터미널 패널 헤더의 \*\*+\*\*를 클릭하거나 채팅의 폴더를 마우스 오른쪽 버튼으로 클릭하여 **Open in terminal**을 선택합니다. 터미널은 로컬 세션에서만 사용 가능합니다.

<h3 id="open-and-edit-files">
  파일 열기 및 편집하기
</h3>

채팅 또는 diff 뷰어의 파일 경로를 클릭하여 파일 패널에서 엽니다. HTML, PDF, 이미지, 비디오 경로는 대신 [브라우저 패널](#preview-your-app)에서 열립니다. 스팟 편집을 하고 **Save**를 클릭하여 다시 작성합니다. 파일이 열린 이후 디스크에서 변경되었으면 패널이 경고하고 재정의하거나 버립니다. **Discard**를 클릭하여 편집을 되돌리거나 패널 헤더의 경로를 클릭하여 절대 경로를 복사합니다.

파일 패널은 로컬 및 SSH 세션에서 사용 가능합니다. 클라우드 세션의 경우 Claude에게 변경을 요청합니다.

<h3 id="open-files-in-other-apps">
  다른 앱에서 파일 열기
</h3>

채팅, diff 뷰어 또는 파일 패널의 모든 파일 경로를 마우스 오른쪽 버튼으로 클릭하여 컨텍스트 메뉴를 엽니다:

* **Attach as context**: 파일을 다음 프롬프트에 추가
* **Open in**: VS Code, Cursor, Zed와 같은 설치된 편집기에서 파일 열기
* **Show in Finder** (macOS), **Show in Explorer** (Windows): 포함된 폴더 열기
* **Copy path**: 절대 경로를 클립보드에 복사

<h3 id="switch-view-modes">
  보기 모드 전환하기
</h3>

보기 모드는 채팅 기록에 나타나는 세부 정보의 양을 제어합니다. 전송 버튼 옆의 **Transcript view** 드롭다운에서 모드를 전환하거나 macOS 또는 Windows에서 **Ctrl+O**를 눌러 모드를 순환합니다.

| 모드          | 표시되는 것                              |
| ----------- | ----------------------------------- |
| **Normal**  | 도구 호출이 요약으로 축소되고 전체 텍스트 응답          |
| **Verbose** | 모든 도구 호출, 파일 읽기, Claude가 수행하는 중간 단계 |
| **Summary** | Claude의 최종 응답과 변경 사항만               |

Claude가 특정 작업을 수행한 이유를 디버깅할 때 Verbose를 사용합니다. 여러 세션을 실행하고 결과를 빠르게 스캔하려고 할 때 Summary를 사용합니다.

<h3 id="keyboard-shortcuts">
  키보드 단축키
</h3>

macOS에서 \*\*Cmd+/\*\*를 누르거나 Windows에서 \*\*Ctrl+/\*\*를 눌러 Code 탭에서 사용 가능한 모든 단축키를 봅니다. Windows에서는 아래 단축키에 대해 **Cmd** 대신 **Ctrl**을 사용합니다. 세션 순환, 터미널 토글, 보기 모드 토글은 모든 플랫폼에서 **Ctrl**을 사용합니다.

| 단축키                                   | 작업            |
| ------------------------------------- | ------------- |
| `Cmd` `/`                             | 키보드 단축키 표시    |
| `Cmd` `N`                             | 새 세션          |
| `Cmd` `W`                             | 세션 닫기         |
| `Ctrl` `Tab` / `Ctrl` `Shift` `Tab`   | 다음 또는 이전 세션   |
| `Cmd` `Shift` `]` / `Cmd` `Shift` `[` | 다음 또는 이전 세션   |
| `Esc`                                 | Claude의 응답 중지 |
| `Cmd` `Shift` `D`                     | diff 패널 토글    |
| `Cmd` `Shift` `B`                     | 브라우저 패널 토글    |
| `Cmd` `Shift` `S`                     | 브라우저에서 요소 선택  |
| `Ctrl` `` ` ``                        | 터미널 패널 토글     |
| `Cmd` `\`                             | 포커스된 패널 닫기    |
| `Cmd` `;`                             | 사이드 채팅 열기     |
| `Ctrl` `O`                            | 보기 모드 순환      |
| `Cmd` `Shift` `M`                     | 권한 모드 메뉴 열기   |
| `Cmd` `Shift` `I`                     | 모델 메뉴 열기      |
| `Cmd` `Shift` `E`                     | 노력 메뉴 열기      |
| `1`–`9`                               | 열린 메뉴에서 항목 선택 |

이러한 단축키는 Code 탭에만 적용됩니다. 터미널 기반 [대화형 모드 단축키](/docs/ko/interactive-mode#keyboard-shortcuts) (예: 모드를 순환하는 Shift+Tab)는 Desktop에 적용되지 않습니다.

<h3 id="check-usage">
  사용량 확인하기
</h3>

모델 선택기 옆의 사용량 링을 클릭하여 현재 컨텍스트 윈도우 사용량과 기간에 대한 계획 사용량을 봅니다. 컨텍스트 사용량은 세션별입니다. 계획 사용량은 모든 Claude Code 표면에서 공유됩니다.

<h2 id="let-claude-use-your-computer">
  Claude가 컴퓨터를 사용하도록 하기
</h2>

컴퓨터 사용을 통해 Claude는 앱을 열고, 화면을 제어하고, 사용자가 하는 방식으로 머신에서 직접 작업할 수 있습니다. Claude에게 모바일 시뮬레이터에서 네이티브 앱을 테스트하거나, CLI가 없는 데스크톱 도구와 상호작용하거나, GUI를 통해서만 작동하는 것을 자동화하도록 요청합니다.

<Note>
  컴퓨터 사용은 Pro 또는 Max 계획이 필요한 macOS 및 Windows의 연구 미리보기입니다. Team 또는 Enterprise 계획에서는 사용할 수 없습니다. Claude Desktop 앱이 실행 중이어야 합니다.
</Note>

컴퓨터 사용은 기본적으로 꺼져 있습니다. [Settings에서 활성화](#enable-computer-use)하기 전에 Claude가 화면을 제어할 수 있습니다. macOS에서는 Accessibility 및 Screen Recording 권한도 부여해야 합니다.

<Warning>
  [샌드박스 Bash 도구](/docs/ko/sandboxing)와 달리 컴퓨터 사용은 승인한 모든 것에 액세스할 수 있는 실제 데스크톱에서 실행됩니다. Claude는 각 작업을 확인하고 화면 콘텐츠에서 잠재적 프롬프트 주입을 플래그하지만 신뢰 경계가 다릅니다. 모범 사례는 [컴퓨터 사용 안전 가이드](https://support.claude.com/en/articles/14128542)를 참조하세요.
</Warning>

<h3 id="when-computer-use-applies">
  컴퓨터 사용이 적용되는 경우
</h3>

Claude는 앱 또는 서비스와 상호작용하는 여러 방법을 가지고 있으며 컴퓨터 사용이 가장 광범위하고 느립니다. 가장 정확한 도구를 먼저 시도합니다:

* 서비스에 대한 [커넥터](#connect-external-tools)가 있으면 Claude는 커넥터를 사용합니다.
* 작업이 셸 명령이면 Claude는 Bash를 사용합니다.
* 작업이 브라우저 작업이고 [Claude in Chrome](/docs/ko/chrome)이 설정되어 있으면 Claude는 그것을 사용합니다.
* 위의 어느 것도 적용되지 않으면 Claude는 컴퓨터 사용을 사용합니다.

[앱별 액세스 계층](#app-permissions)은 이를 강화합니다: 브라우저는 보기 전용으로 제한되고, 터미널 및 IDE는 클릭 전용으로 제한되어 컴퓨터 사용이 활성화되어 있어도 Claude를 전용 도구로 유도합니다. 화면 제어는 네이티브 앱, 하드웨어 제어판, 모바일 시뮬레이터 또는 API가 없는 독점 도구와 같이 다른 것이 도달할 수 없는 것을 위해 예약되어 있습니다.

<h3 id="enable-computer-use">
  컴퓨터 사용 활성화하기
</h3>

컴퓨터 사용은 기본적으로 꺼져 있습니다. Claude가 필요한 작업을 하도록 요청하는데 꺼져 있으면 Claude는 Settings에서 컴퓨터 사용을 활성화하면 작업을 수행할 수 있다고 알려줍니다.

<Steps>
  <Step title="데스크톱 앱 업데이트">
    최신 버전의 Claude Desktop이 있는지 확인합니다. macOS 및 Windows에서는 [claude.com/download](https://claude.com/download)에서 다운로드하거나 업데이트하고, Linux에서는 패키지 관리자를 통해 업데이트합니다([지침](/docs/ko/desktop-linux)). 그런 다음 앱을 다시 시작합니다.
  </Step>

  <Step title="토글 켜기">
    데스크톱 앱에서 **Settings > General** (**Desktop app** 아래)로 이동합니다. **Computer use** 토글을 찾아 켭니다. Windows에서는 토글이 즉시 적용되고 설정이 완료됩니다. macOS에서는 다음 단계로 계속합니다.

    토글이 보이지 않으면 macOS 또는 Windows에서 Pro 또는 Max 계획을 사용하고 있는지 확인한 다음 업데이트하고 앱을 다시 시작합니다.
  </Step>

  <Step title="macOS 권한 부여">
    macOS에서는 토글이 적용되기 전에 두 가지 시스템 권한을 부여합니다:

    * **Accessibility**: Claude가 클릭, 입력, 스크롤할 수 있게 합니다
    * **Screen Recording**: Claude가 화면에 있는 것을 볼 수 있게 합니다

    Settings 페이지는 각 권한의 현재 상태를 표시합니다. 둘 중 하나가 거부되면 배지를 클릭하여 관련 System Settings 창을 엽니다.
  </Step>
</Steps>

<h3 id="app-permissions">
  앱 권한
</h3>

Claude가 처음 앱을 사용해야 할 때 세션에 프롬프트가 나타납니다. **Allow for this session** 또는 **Deny**를 클릭합니다. 승인은 현재 세션 또는 [Dispatch 생성 세션](#sessions-from-dispatch)에서 30분 동안 지속됩니다.

프롬프트는 또한 Claude가 해당 앱에 대해 얻는 제어 수준을 표시합니다. 이러한 계층은 앱 카테고리별로 고정되며 변경할 수 없습니다:

| 계층    | Claude가 할 수 있는 것                | 적용 대상        |
| :---- | :------------------------------ | :----------- |
| 보기 전용 | 스크린샷에서 앱 보기                     | 브라우저, 거래 플랫폼 |
| 클릭 전용 | 클릭 및 스크롤하지만 입력 또는 키보드 단축키 사용 불가 | 터미널, IDE     |
| 전체 제어 | 클릭, 입력, 드래그, 키보드 단축키 사용         | 기타 모든 것      |

Terminal, Finder 또는 File Explorer, System Settings 또는 Settings와 같이 광범위한 영향을 미치는 앱은 승인이 부여하는 것을 알 수 있도록 프롬프트에 추가 경고를 표시합니다.

**Settings > General** (**Desktop app** 아래)에서 두 가지 설정을 구성할 수 있습니다:

* **Denied apps**: 프롬프트 없이 거부하려면 여기에 앱을 추가합니다. Claude는 허용된 앱의 작업을 통해 거부된 앱에 간접적으로 영향을 미칠 수 있지만 거부된 앱과 직접 상호작용할 수 없습니다.
* **Unhide apps when Claude finishes**: Claude가 작업하는 동안 다른 창이 숨겨져 승인된 앱하고만 상호작용합니다. Claude가 완료되면 이 설정을 끄지 않는 한 숨겨진 창이 복원됩니다.

<h2 id="manage-sessions">
  세션 관리하기
</h2>

각 세션은 자신의 컨텍스트와 변경 사항을 가진 독립적인 대화입니다. 여러 세션을 병렬로 실행하거나, 사이드 채팅을 분기하거나, 작업을 클라우드로 보내거나, Dispatch가 휴대폰에서 세션을 시작하도록 할 수 있습니다.

<h3 id="work-in-parallel-with-sessions">
  세션으로 병렬 작업하기
</h3>

사이드바에서 **+ New session**을 클릭하거나 macOS에서 **Cmd+N**을 누르거나 Windows에서 **Ctrl+N**을 눌러 여러 작업을 병렬로 작업합니다. **Ctrl+Tab** 및 **Ctrl+Shift+Tab**을 눌러 사이드바의 세션을 순환합니다. Git 저장소의 경우 각 세션은 [Git worktrees](/docs/ko/worktrees)를 사용하여 프로젝트의 자신의 격리된 복사본을 가져오므로 한 세션의 변경 사항이 커밋할 때까지 다른 세션에 영향을 주지 않습니다.

두 세션을 동시에 보려면 macOS에서 **Cmd**를 누르거나 Windows에서 **Ctrl**을 누르고 사이드바의 세션을 클릭합니다. 세션이 이미 열려 있는 창 옆에 두 번째 창에서 열립니다. 분할이 활성화되어 있는 동안 다른 사이드바 세션을 클릭하면 포커스가 있는 창을 바꿉니다. macOS에서 \*\*Cmd+\\\*\*를 누르거나 Windows에서 \*\*Ctrl+\\\*\*를 눌러 포커스된 창을 닫고 단일 세션으로 돌아갑니다.

Worktrees는 기본적으로 `<project-root>/.claude/worktrees/`에 저장됩니다. Settings → Claude Code의 "Worktree location"에서 사용자 정의 디렉토리로 변경할 수 있습니다. 또한 모든 worktree 브랜치 이름 앞에 추가되는 브랜치 접두사를 설정할 수 있으며, 이는 Claude가 만든 브랜치를 정리하는 데 유용합니다. 완료되면 사이드바의 세션 위에 마우스를 올리고 아카이브 아이콘을 클릭하여 worktree를 제거합니다. PR이 병합되거나 닫힌 후 세션이 자동으로 아카이브되도록 하려면 Settings → Claude Code에서 **Auto-archive after PR merge or close**를 켭니다. 자동 아카이브는 실행을 완료한 로컬 세션에만 적용됩니다.

gitignored 파일 (예: `.env`)을 새 worktrees에 포함하려면 프로젝트 루트에 [`.worktreeinclude` 파일](/docs/ko/worktrees#copy-gitignored-files-into-worktrees)을 만듭니다.

<Note>
  세션 격리에는 [Git](https://git-scm.com/downloads)이 필요합니다. 대부분의 Mac에는 기본적으로 Git이 포함되어 있습니다. Terminal에서 `git --version`을 실행하여 확인합니다. Windows에서는 Code 탭이 작동하려면 Git이 필요합니다: [Windows용 Git 다운로드](https://git-scm.com/downloads/win), 설치 및 앱 재시작. Git 오류가 발생하면 [Cowork 탭](https://claude.com/product/cowork)에서 Claude에게 설정을 문제 해결하도록 요청하세요.
</Note>

사이드바 상단의 컨트롤을 사용하여 상태, 프로젝트 또는 환경별로 세션을 필터링하고 프로젝트별로 세션을 그룹화합니다. 세션 이름을 바꾸려면 활성 세션 상단의 도구 모음에서 세션 제목을 클릭합니다. 컨텍스트 사용량을 확인하려면 [사용량 확인](#check-usage)을 참조하세요. 컨텍스트가 가득 차면 Claude는 자동으로 대화를 요약하고 계속 작업합니다. `/compact`를 입력하여 요약을 더 일찍 트리거하고 컨텍스트 공간을 확보할 수도 있습니다. [컨텍스트 윈도우](/docs/ko/how-claude-code-works#the-context-window)에서 압축이 작동하는 방식에 대한 자세한 내용을 참조하세요.

데스크톱 앱은 Code 세션이 작업을 완료하고 현재 해당 세션을 보고 있지 않을 때 OS 알림을 보냅니다.

<h3 id="ask-a-side-question-without-derailing-the-session">
  세션을 벗어나지 않고 옆 질문 하기
</h3>

사이드 채팅을 사용하면 세션의 컨텍스트를 사용하지만 메인 대화에 아무것도 추가하지 않고 Claude에게 질문할 수 있습니다. 코드 조각을 이해하거나, 가정을 확인하거나, 세션을 벗어나지 않고 아이디어를 탐색하려고 할 때 사용합니다.

macOS에서 \*\*Cmd+;\*\*를 누르거나 Windows에서 \*\*Ctrl+;\*\*를 누르거나 프롬프트 상자에 `/btw`를 입력하여 사이드 채팅을 엽니다. 사이드 채팅은 그 지점까지 메인 스레드의 모든 것을 읽을 수 있습니다. 완료되면 사이드 채팅을 닫고 중단한 곳에서 메인 세션을 계속합니다. 사이드 채팅은 로컬, SSH 및 WSL 세션에서 사용 가능합니다.

<h3 id="watch-background-tasks">
  백그라운드 작업 보기
</h3>

작업 패널은 현재 세션 내에서 실행 중인 백그라운드 작업을 표시합니다: 서브에이전트, 백그라운드 셸 명령, [동적 워크플로우](/docs/ko/workflows). **Views** 메뉴에서 열거나 레이아웃으로 드래그합니다.

모든 항목을 클릭하여 서브에이전트 패널에서 출력을 보거나 중지합니다. 다른 세션이 수행하는 작업을 보려면 [사이드바](#work-in-parallel-with-sessions)를 사용합니다.

<h3 id="run-long-running-tasks-remotely">
  원격으로 장기 실행 작업 실행하기
</h3>

대규모 리팩토링, 테스트 스위트, 마이그레이션 또는 기타 장기 실행 작업의 경우 세션을 시작할 때 **Local** 대신 **Remote**를 선택합니다. 원격 세션은 Anthropic의 클라우드 인프라에서 실행되며 앱을 닫거나 컴퓨터를 종료해도 계속됩니다. 언제든지 돌아와서 진행 상황을 보거나 Claude를 다른 방향으로 조종할 수 있습니다. [claude.ai/code](https://claude.ai/code)에서 또는 Claude iOS 앱에서 원격 세션을 모니터링할 수도 있습니다.

원격 세션은 또한 여러 저장소를 지원합니다. 클라우드 환경을 선택한 후 저장소 pill 옆의 **+** 버튼을 클릭하여 세션에 추가 저장소를 추가합니다. 각 저장소는 자신의 브랜치 선택기를 가집니다. 이는 공유 라이브러리와 그 소비자를 업데이트하는 것과 같이 여러 코드베이스에 걸친 작업에 유용합니다.

원격 세션이 작동하는 방식에 대한 자세한 내용은 [웹의 Claude Code](/docs/ko/claude-code-on-the-web)를 참조하세요.

<h3 id="continue-in-another-surface">
  다른 표면에서 계속하기
</h3>

세션 도구 모음의 오른쪽 아래에 있는 VS Code 아이콘에서 액세스할 수 있는 **Continue in** 메뉴를 사용하면 세션을 다른 표면으로 이동할 수 있습니다:

* **Claude Code on the Web**: 로컬 세션을 원격으로 계속 실행하도록 보냅니다. Desktop은 브랜치를 푸시하고, 대화 요약을 생성하고, 전체 컨텍스트를 사용하여 새 원격 세션을 만듭니다. 그 후 로컬 세션을 아카이브하거나 유지하도록 선택할 수 있습니다. 이는 깨끗한 작업 트리가 필요하며 SSH 세션에는 사용할 수 없습니다.
* **Your IDE**: 현재 작업 디렉토리에서 지원되는 IDE에서 프로젝트를 엽니다.

<h3 id="sessions-from-dispatch">
  Dispatch에서 세션
</h3>

[Dispatch](https://support.claude.com/en/articles/13947068)는 [Cowork](https://claude.com/product/cowork) 탭에 있는 Claude와의 지속적인 대화입니다. Dispatch에 작업을 메시지하면 처리 방법을 결정합니다.

작업은 두 가지 방법으로 Code 세션이 될 수 있습니다: 직접 요청하는 경우 (예: "Claude Code 세션을 열고 로그인 버그를 수정하세요") 또는 Dispatch가 작업이 개발 작업이라고 결정하고 자동으로 하나를 생성하는 경우입니다. 일반적으로 Code로 라우팅되는 작업에는 버그 수정, 종속성 업데이트, 테스트 실행 또는 pull request 열기가 포함됩니다. 연구, 문서 편집, 스프레드시트 작업은 Cowork에 남아 있습니다.

어느 쪽이든 Code 세션은 **Dispatch** 배지가 있는 Code 탭의 사이드바에 나타납니다. 완료되거나 승인이 필요할 때 휴대폰에서 푸시 알림을 받습니다.

[컴퓨터 사용](#let-claude-use-your-computer)이 활성화되어 있으면 Dispatch 생성 Code 세션도 사용할 수 있습니다. 이러한 세션의 앱 승인은 30분 후 만료되고 다시 프롬프트하며, 일반 Code 세션처럼 전체 세션 동안 지속되지 않습니다.

설정, 페어링, Dispatch 설정은 [Dispatch 도움말 문서](https://support.claude.com/en/articles/13947068)를 참조하세요. Dispatch는 Pro 또는 Max 계획이 필요하며 Team 또는 Enterprise 계획에서는 사용할 수 없습니다.

Dispatch는 터미널에서 멀리 떨어져 있을 때 Claude와 작업하는 여러 방법 중 하나입니다. [플랫폼 및 통합](/docs/ko/platforms#work-when-you-are-away-from-your-terminal)을 참조하여 Remote Control, Channels, Slack, 예약된 작업과 비교하세요.

<h2 id="extend-claude-code">
  Claude Code 확장하기
</h2>

외부 서비스를 연결하고, 재사용 가능한 워크플로우를 추가하고, Claude의 동작을 사용자 정의하고, 미리보기 서버를 구성합니다. 한 곳에서 커넥터, skills, 플러그인을 관리하려면 사이드바에서 **Customize**를 클릭합니다.

<h3 id="connect-external-tools">
  외부 도구 연결하기
</h3>

로컬 및 [SSH](#ssh-sessions) 세션의 경우 프롬프트 상자 옆의 **+** 버튼을 클릭하고 **Connectors**를 선택하여 Google Calendar, Slack, GitHub, Linear, Notion 등과 같은 통합을 추가합니다. 세션 전이나 중에 커넥터를 추가할 수 있습니다. **+** 버튼은 클라우드 또는 WSL 세션에서 사용할 수 없지만 [루틴](/docs/ko/routines)은 루틴 생성 시 커넥터를 구성합니다.

커넥터를 관리하거나 연결을 해제하려면 데스크톱 앱의 Settings → Connectors로 이동하거나 프롬프트 상자의 Connectors 메뉴에서 **Manage connectors**를 선택합니다.

연결되면 Claude는 캘린더를 읽고, 메시지를 보내고, 문제를 만들고, 도구와 직접 상호작용할 수 있습니다. Claude에게 세션에 구성된 커넥터가 무엇인지 물어볼 수 있습니다.

커넥터는 그래픽 설정 흐름이 있는 [MCP servers](/docs/ko/mcp)입니다. 지원되는 서비스와의 빠른 통합을 위해 사용합니다. Connectors에 나열되지 않은 통합의 경우 [설정 파일](/docs/ko/mcp#installing-mcp-servers)을 통해 MCP 서버를 수동으로 추가합니다. [사용자 정의 커넥터를 만들](https://support.claude.com/en/articles/11175166-getting-started-with-custom-connectors-using-remote-mcp) 수도 있습니다.

<h3 id="use-skills">
  skills 사용하기
</h3>

[Skills](/docs/ko/skills)는 Claude가 할 수 있는 것을 확장합니다. Claude는 관련이 있을 때 자동으로 로드하거나 직접 호출할 수 있습니다: 프롬프트 상자에서 `/`를 입력하거나 **+** 버튼을 클릭하고 **Slash commands**를 선택하여 사용 가능한 것을 찾아봅니다. 여기에는 [내장 명령](/docs/ko/commands), [사용자 정의 skills](/docs/ko/skills#create-your-first-skill), 코드베이스의 프로젝트 skills, [설치된 플러그인](/docs/ko/plugins)의 skills가 포함됩니다. 하나를 선택하면 입력 필드에 강조 표시됩니다. 그 후 작업을 입력하고 평소대로 보냅니다.

Claude가 작업 중일 때 다른 메시지와 동일하게 명령을 보낼 수 있으며, 턴이 완료되면 세션이 유휴 상태로 돌아갑니다. v2.1.206 이전에는 턴 중에 보낸 명령이 세션을 실행 중으로 표시된 상태로 남길 수 있었고 그 후에 보낸 메시지는 전달되지 않았습니다.

<h3 id="install-plugins">
  플러그인 설치하기
</h3>

[Plugins](/docs/ko/plugins)는 Claude Code에 skills, agents, hooks, MCP servers, LSP 구성을 추가하는 재사용 가능한 패키지입니다. 터미널을 사용하지 않고 데스크톱 앱에서 플러그인을 설치할 수 있습니다.

로컬 및 [SSH](#ssh-sessions) 세션의 경우 프롬프트 상자 옆의 **+** 버튼을 클릭하고 **Plugins**를 선택하여 설치된 플러그인과 해당 skills를 봅니다. 플러그인을 추가하려면 서브메뉴에서 **Add plugin**을 선택하여 플러그인 브라우저를 열면 공식 Anthropic marketplace를 포함한 구성된 [marketplaces](/docs/ko/plugin-marketplaces)의 사용 가능한 플러그인이 표시됩니다. **Manage plugins**를 선택하여 플러그인을 활성화, 비활성화 또는 제거합니다.

플러그인은 사용자 계정, 특정 프로젝트 또는 로컬 전용으로 범위를 지정할 수 있습니다. 조직이 플러그인을 중앙에서 관리하는 경우 해당 플러그인은 CLI에서와 동일한 방식으로 데스크톱 세션에서 사용 가능합니다. 플러그인은 클라우드 또는 WSL 세션에는 사용할 수 없습니다. 자신의 플러그인을 만드는 것을 포함한 전체 플러그인 참조는 [plugins](/docs/ko/plugins)를 참조하세요.

<h3 id="configure-preview-servers">
  미리보기 서버 구성하기
</h3>

Claude는 개발 서버 설정을 자동으로 감지하고 세션을 시작할 때 선택한 폴더의 루트에 있는 `.claude/launch.json`에 구성을 저장합니다. Preview는 이 폴더를 작업 디렉토리로 사용하므로 부모 폴더를 선택한 경우 자신의 개발 서버가 있는 하위 폴더는 자동으로 감지되지 않습니다. 하위 폴더의 서버로 작업하려면 해당 폴더에서 직접 세션을 시작하거나 구성을 수동으로 추가합니다.

예를 들어 `npm run dev` 대신 `yarn dev`를 사용하거나 포트를 변경하도록 서버가 시작되는 방식을 사용자 정의하려면 파일을 수동으로 편집하거나 서버 드롭다운에서 **Edit configuration**을 클릭하여 코드 편집기에서 엽니다. 파일은 주석이 있는 JSON을 지원합니다.

```json theme={null}
{
  "version": "0.0.1",
  "configurations": [
    {
      "name": "my-app",
      "runtimeExecutable": "npm",
      "runtimeArgs": ["run", "dev"],
      "port": 3000
    }
  ]
}
```

동일한 프로젝트에서 프론트엔드 및 API와 같은 다양한 서버를 실행하도록 여러 구성을 정의할 수 있습니다. 아래의 [예제](#examples)를 참조하세요.

<h4 id="auto-verify-changes">
  자동 변경 사항 확인
</h4>

`autoVerify`가 활성화되면 Claude는 파일을 편집한 후 자동으로 코드 변경 사항을 확인합니다. 스크린샷을 찍고, 오류를 확인하고, 응답을 완료하기 전에 변경 사항이 작동하는지 확인합니다.

자동 확인은 기본적으로 켜져 있습니다. `.claude/launch.json`에 `"autoVerify": false`를 추가하여 프로젝트별로 비활성화하거나 서버 드롭다운 메뉴에서 토글합니다.

```json theme={null}
{
  "version": "0.0.1",
  "autoVerify": false,
  "configurations": [...]
}
```

비활성화되면 미리보기 도구는 여전히 사용 가능하며 언제든지 Claude에게 확인을 요청할 수 있습니다. 자동 확인은 모든 편집 후 자동으로 만듭니다.

<h4 id="configuration-fields">
  구성 필드
</h4>

`configurations` 배열의 각 항목은 다음 필드를 허용합니다:

| 필드                  | 유형        | 설명                                                                                                                                            |
| ------------------- | --------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`              | string    | 이 서버의 고유 식별자                                                                                                                                  |
| `runtimeExecutable` | string    | 실행할 명령 (예: `npm`, `yarn`, `node`)                                                                                                             |
| `runtimeArgs`       | string\[] | `runtimeExecutable`에 전달되는 인수 (예: `["run", "dev"]`)                                                                                            |
| `port`              | number    | 서버가 수신하는 포트. 기본값은 3000                                                                                                                        |
| `cwd`               | string    | 프로젝트 루트에 상대적인 작업 디렉토리. 기본값은 프로젝트 루트입니다. 프로젝트 루트를 명시적으로 참조하려면 `${workspaceFolder}`를 사용합니다                                                      |
| `env`               | object    | `{ "NODE_ENV": "development" }`와 같은 키-값 쌍으로 추가 환경 변수. 이 파일이 저장소에 커밋되므로 여기에 비밀을 넣지 마세요. 비밀을 개발 서버에 전달하려면 [로컬 환경 편집기](#local-sessions)에서 설정합니다. |
| `autoPort`          | boolean   | 포트 충돌을 처리하는 방법. 아래를 참조하세요                                                                                                                     |
| `program`           | string    | `node`로 실행할 스크립트. [언제 `program` vs `runtimeExecutable`을 사용할지](#when-to-use-program-vs-runtimeexecutable) 참조                                   |
| `args`              | string\[] | `program`에 전달되는 인수. `program`이 설정된 경우에만 사용됨                                                                                                   |

<a id="when-to-use-program-vs-runtimeexecutable" />

<h5 id="when-to-use-program-vs-runtimeexecutable">
  `program` vs `runtimeExecutable` 사용 시기
</h5>

패키지 관리자를 통해 개발 서버를 시작하려면 `runtimeExecutable`을 `runtimeArgs`와 함께 사용합니다. 예를 들어 `"runtimeExecutable": "npm"`과 `"runtimeArgs": ["run", "dev"]`는 `npm run dev`를 실행합니다.

`node`로 직접 실행하려는 독립 실행형 스크립트가 있을 때 `program`을 사용합니다. 예를 들어 `"program": "server.js"`는 `node server.js`를 실행합니다. `args`로 추가 플래그를 전달합니다.

<h4 id="port-conflicts">
  포트 충돌
</h4>

`autoPort` 필드는 선호하는 포트가 이미 사용 중일 때 발생하는 상황을 제어합니다:

* **`true`**: Claude는 자동으로 사용 가능한 포트를 찾아 사용합니다. 대부분의 개발 서버에 적합합니다.
* **`false`**: Claude는 오류로 실패합니다. OAuth 콜백 또는 CORS allowlists와 같이 서버가 특정 포트를 사용해야 할 때 사용합니다.
* **설정되지 않음 (기본값)**: Claude는 서버가 정확한 포트가 필요한지 묻고 답변을 저장합니다.

Claude가 다른 포트를 선택하면 할당된 포트를 `PORT` 환경 변수를 통해 서버에 전달합니다.

<h4 id="examples">
  예제
</h4>

이러한 구성은 다양한 프로젝트 유형에 대한 일반적인 설정을 보여줍니다:

<Tabs>
  <Tab title="Next.js">
    이 구성은 Yarn을 사용하여 포트 3000에서 Next.js 앱을 실행합니다:

    ```json theme={null}
    {
      "version": "0.0.1",
      "configurations": [
        {
          "name": "web",
          "runtimeExecutable": "yarn",
          "runtimeArgs": ["dev"],
          "port": 3000
        }
      ]
    }
    ```
  </Tab>

  <Tab title="Multiple servers">
    프론트엔드 및 API 서버가 있는 monorepo의 경우 여러 구성을 정의합니다. 프론트엔드는 `autoPort: true`를 사용하므로 3000이 사용 중이면 사용 가능한 포트를 선택하고, API 서버는 포트 8080을 정확히 요구합니다:

    ```json theme={null}
    {
      "version": "0.0.1",
      "configurations": [
        {
          "name": "frontend",
          "runtimeExecutable": "npm",
          "runtimeArgs": ["run", "dev"],
          "cwd": "apps/web",
          "port": 3000,
          "autoPort": true
        },
        {
          "name": "api",
          "runtimeExecutable": "npm",
          "runtimeArgs": ["run", "start"],
          "cwd": "server",
          "port": 8080,
          "env": { "NODE_ENV": "development" },
          "autoPort": false
        }
      ]
    }
    ```
  </Tab>

  <Tab title="Node.js script">
    패키지 관리자 명령 대신 Node.js 스크립트를 직접 실행하려면 `program` 필드를 사용합니다:

    ```json theme={null}
    {
      "version": "0.0.1",
      "configurations": [
        {
          "name": "server",
          "program": "server.js",
          "args": ["--verbose"],
          "port": 4000
        }
      ]
    }
    ```
  </Tab>
</Tabs>

<h2 id="environment-configuration">
  환경 구성
</h2>

[세션을 시작](#start-a-session)할 때 선택하는 환경은 Claude가 실행되는 위치와 연결 방식을 결정합니다:

* **Local**: 머신에서 실행되며 파일에 직접 액세스합니다
* **Remote**: Anthropic의 클라우드 인프라에서 실행됩니다. 앱을 닫아도 세션이 계속됩니다.
* **SSH**: SSH를 통해 연결하는 원격 머신(예: 자신의 서버, 클라우드 VM 또는 개발 컨테이너)에서 실행됩니다
* **WSL** (Windows): 머신의 [WSL 2 배포판](/docs/ko/desktop-wsl)에서 실행되며, Linux 도구 모음 및 기본 경로를 사용합니다

<h3 id="local-sessions">
  로컬 세션
</h3>

데스크톱 앱이 항상 전체 셸 환경을 상속하지는 않습니다. macOS에서 Dock 또는 Finder에서 앱을 실행하면 `~/.zshrc` 또는 `~/.bashrc`와 같은 셸 프로필을 읽어 `PATH` 및 고정된 Claude Code 변수 집합을 추출하지만, 거기에 내보낸 다른 변수는 선택되지 않습니다. Windows에서 앱은 사용자 및 시스템 환경 변수를 상속하지만 PowerShell 프로필을 읽지 않습니다.

로컬 세션 및 개발 서버에 대한 환경 변수를 설정하려면 프롬프트 상자의 환경 드롭다운을 열고 **Local** 위에 마우스를 올린 다음 기어 아이콘을 클릭하여 로컬 환경 편집기를 엽니다. 여기에 저장한 변수는 머신에 암호화되어 저장되며 시작하는 모든 로컬 세션 및 미리보기 서버에 적용됩니다. `~/.claude/settings.json` 파일의 `env` 키에 변수를 추가할 수도 있습니다. 단, 이는 Claude 세션에만 도달하고 개발 서버에는 도달하지 않습니다. 지원되는 변수의 전체 목록은 [환경 변수](/docs/ko/env-vars)를 참조하세요.

[Extended thinking](/docs/ko/model-config#extended-thinking)은 기본적으로 활성화되어 있으며, 복잡한 추론 작업의 성능을 향상시키지만 추가 토큰을 사용합니다. 생각을 완전히 비활성화하려면 로컬 환경 편집기에서 `MAX_THINKING_TOKENS`을 `0`으로 설정합니다. [third-party providers](/docs/ko/third-party-integrations)에서 `0`은 `thinking` 매개변수를 대신 생략하며, 적응형 추론 모델은 여전히 생각할 수 있습니다. [적응형 추론](/docs/ko/model-config#adjust-effort-level)이 있는 모델에서는 적응형 추론이 생각 깊이를 제어하기 때문에 다른 `MAX_THINKING_TOKENS` 값은 무시됩니다. Opus 4.6 및 Sonnet 4.6에서는 `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING`을 `1`로 설정하여 고정 생각 예산을 사용합니다. Fable 5, Sonnet 5, Opus 4.7 이상은 항상 적응형 추론을 사용하며 고정 예산 모드가 없습니다.

<h3 id="cloud-sessions">
  클라우드 세션
</h3>

클라우드 세션은 앱을 닫아도 백그라운드에서 계속됩니다. 사용량은 별도의 컴퓨팅 요금 없이 [구독 계획 한도](/docs/ko/costs)에 포함됩니다.

다양한 네트워크 액세스 수준 및 환경 변수를 가진 사용자 정의 클라우드 환경을 만들 수 있습니다. 클라우드 세션을 시작할 때 환경 드롭다운을 선택하고 **환경 추가**를 선택합니다. 네트워크 액세스 및 환경 변수 구성에 대한 자세한 내용은 [클라우드 환경](/docs/ko/claude-code-on-the-web#the-cloud-environment)을 참조하세요.

<h3 id="ssh-sessions">
  SSH 세션
</h3>

SSH 세션을 사용하면 데스크톱 앱을 인터페이스로 사용하면서 원격 머신에서 Claude Code를 실행할 수 있습니다. 이는 클라우드 VM, 개발 컨테이너 또는 특정 하드웨어 또는 종속성이 있는 서버에 있는 코드베이스로 작업할 때 유용합니다.

SSH 연결을 추가하려면 세션을 시작하기 전에 환경 드롭다운을 클릭하고 **+ SSH 연결 추가**를 선택합니다. 대화 상자는 다음을 요청합니다:

* **Name**: 이 연결의 친화적인 레이블
* **SSH Host**: `user@hostname` 또는 `~/.ssh/config`에 정의된 호스트
* **SSH Port**: 비워두면 기본값은 22이거나 SSH 구성의 포트를 사용합니다
* **Identity File**: `~/.ssh/id_rsa`와 같은 개인 키의 경로. 기본 키 또는 SSH 구성을 사용하려면 비워둡니다.

추가되면 연결이 환경 드롭다운에 나타납니다. 이를 선택하여 해당 머신에서 세션을 시작합니다. Claude는 원격 머신에서 파일 및 도구에 액세스하여 실행됩니다.

원격 머신은 Linux 또는 macOS를 실행해야 합니다. 데스크톱은 처음 연결할 때 원격 머신에 Claude Code를 자동으로 설치합니다. 연결되면 SSH 세션은 권한 모드, 커넥터, 플러그인 및 MCP 서버를 지원합니다.

<h4 id="pre-configure-ssh-connections-for-your-team">
  팀을 위해 SSH 연결을 미리 구성합니다
</h4>

관리자는 [관리되는 설정](/docs/ko/settings#settings-precedence) 파일에 `sshConfigs`를 추가하여 팀 멤버에게 SSH 연결을 배포할 수 있습니다. 이러한 방식으로 정의된 연결은 각 사용자의 환경 드롭다운에 자동으로 나타나며 관리되는 것으로 표시되므로 사용자는 이를 선택할 수 있지만 앱에서 편집하거나 삭제할 수 없습니다.

다음 예제는 원격 호스트의 `~/projects`에서 열리는 단일 연결을 미리 구성합니다:

```json theme={null}
{
  "sshConfigs": [
    {
      "id": "shared-dev-vm",
      "name": "Shared Dev VM",
      "sshHost": "user@dev.example.com",
      "sshPort": 22,
      "sshIdentityFile": "~/.ssh/id_ed25519",
      "startDirectory": "~/projects"
    }
  ]
}
```

각 항목에는 `id`, `name`, `sshHost`가 필요합니다. `sshPort`, `sshIdentityFile`, `startDirectory` 필드는 선택 사항입니다. 사용자는 자신의 `~/.claude/settings.json`에 `sshConfigs`를 추가할 수도 있습니다. 이는 대화 상자를 통해 추가된 연결이 저장되는 위치입니다.

<h4 id="restrict-which-ssh-hosts-users-can-connect-to">
  SSH 호스트 연결을 제한하여 사용자가 연결할 수 있는 호스트를 제한합니다
</h4>

관리자는 [관리되는 설정](/docs/ko/settings#settings-precedence) 파일에 `sshHostAllowlist`를 추가하여 Desktop의 SSH 세션을 승인된 호스트 집합으로 제한할 수 있습니다. 설정되면 사용자는 확인된 호스트명이 패턴 중 하나와 일치하는 호스트에만 연결할 수 있습니다. SSH 세션을 완전히 비활성화하려면 빈 배열로 설정합니다.

다음 예제는 `devboxes.example.com` 아래의 모든 호스트 및 단일 명명된 bastion 호스트에 대한 연결을 허용합니다:

```json theme={null}
{
  "sshHostAllowlist": ["*.devboxes.example.com", "bastion.example.com"]
}
```

패턴은 대소문자를 구분하지 않습니다. `*`는 모든 호스트와 일치하고, `*.example.com`은 `example.com` 및 모든 하위 도메인과 일치합니다. 다른 모든 것은 정확한 일치입니다. 검사는 `ssh -G`를 통한 `~/.ssh/config` 확인 후 호스트명에 대해 실행되므로 `Host` 별칭 및 `ProxyCommand`/`ProxyJump` 항목은 확인된 `HostName`이 일치하는 한 허용됩니다.

`sshHostAllowlist`는 관리되는 설정에서만 읽혀집니다. 사용자 또는 프로젝트 설정의 값은 무시됩니다. Claude Desktop 앱만 이 설정을 인식합니다. Claude Code CLI 및 IDE 확장은 이를 읽지 않으며, Bash 도구를 통해 실행되는 `ssh` 명령을 제한하지 않습니다. 이는 Desktop 앱이 연결하는 호스트를 제어하며, 네트워크 송신을 제어하지 않으므로 하드 경계가 필요한 경우 조직의 네트워크 또는 제로 트러스트 제어와 함께 사용합니다.

<h2 id="enterprise-configuration">
  엔터프라이즈 구성
</h2>

Team 또는 Enterprise 계획의 조직은 관리 콘솔 컨트롤, 관리 설정 파일, 장치 관리 정책을 통해 데스크톱 앱 동작을 관리할 수 있습니다.

<h3 id="admin-console-controls">
  관리 콘솔 컨트롤
</h3>

이러한 설정은 [관리 설정 콘솔](https://claude.ai/admin-settings/claude-code)을 통해 구성됩니다:

* **데스크톱의 Code**: 조직의 사용자가 데스크톱 앱에서 Claude Code에 액세스할 수 있는지 제어합니다
* **웹의 Code**: 조직의 [웹 세션](/docs/ko/claude-code-on-the-web)을 활성화 또는 비활성화합니다
* **Remote Control**: 조직의 [Remote Control](/docs/ko/remote-control)을 활성화 또는 비활성화합니다
* **권한 무시 모드 비활성화**: 조직의 사용자가 권한 무시 모드를 활성화하지 못하도록 방지합니다

<h3 id="managed-settings">
  관리 설정
</h3>

관리 설정은 프로젝트 및 사용자 설정을 재정의하고 Desktop의 Claude Code 세션에 적용됩니다. 조직의 [관리 설정](/docs/ko/settings#settings-precedence) 파일에서 이러한 키를 설정하거나 관리 콘솔을 통해 원격으로 푸시할 수 있습니다.

| 키                                          | 설명                                                                                                                                                                                                                                                 |
| ------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `permissions.disableBypassPermissionsMode` | 사용자가 권한 무시 모드를 활성화하지 못하도록 하려면 `"disable"`로 설정합니다.                                                                                                                                                                                                  |
| `disableAutoMode`                          | 사용자가 [Auto](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode) 모드를 활성화하지 못하도록 하려면 `"disable"`로 설정합니다. 모드 선택기에서 Auto를 제거합니다. `permissions` 아래에서도 허용됩니다.                                                                                         |
| `autoMode`                                 | 조직 전체에서 auto mode 분류기가 신뢰하고 차단하는 것을 사용자 정의합니다. [auto mode 구성](/docs/ko/auto-mode-config)을 참조하세요.                                                                                                                                                        |
| `browserExternalPageTools`                 | Claude가 [Browser 창](#browse-external-sites)에서 외부 페이지를 읽거나 작동하기 위해 도구를 사용하지 못하도록 하려면 `"disabled"`로 설정합니다. 사용자는 여전히 외부 사이트로 직접 이동할 수 있으며, 로컬 개발 서버 미리보기는 영향을 받지 않습니다.                                                                                |
| `disableBrowserExternalNavigation`         | [Browser 창](#browse-external-sites)에서 외부 브라우징을 완전히 끄려면 `true`로 설정합니다. 사용자와 Claude 모두 외부 사이트로 이동할 수 없으며, localhost 개발 서버 미리보기는 영향을 받지 않습니다. 값은 JSON 부울 `true`여야 합니다. 문자열 `"true"`는 무시됩니다.                                                           |
| `sshConfigs`                               | 환경 드롭다운에 나타나는 [SSH 연결](#pre-configure-ssh-connections-for-your-team)을 사전 구성합니다. 사용자는 관리 연결을 편집하거나 삭제할 수 없습니다.                                                                                                                                      |
| `sshHostAllowlist`                         | [SSH 세션](#restrict-which-ssh-hosts-users-can-connect-to)을 확인된 호스트명이 이러한 패턴 중 하나와 일치하는 호스트로 제한합니다. 빈 배열은 SSH 세션을 비활성화합니다. 관리 설정에서만 읽습니다.                                                                                                            |
| `managedMcpServers`                        | 타사 배포에서 모든 사용자에게 MCP 서버 구성을 푸시합니다. 각 항목은 `"http"`, `"sse"`, 또는 `"stdio"`의 전송, 연결 세부 정보, 그리고 선택적으로 해당 서버의 어떤 도구를 사용자가 호출할 수 있는지 제한하는 `toolPolicy` 맵을 지정합니다. 타사(3P) Desktop 배포에서만 사용 가능합니다. 타사 배포는 관리 콘솔 설정을 받지 않으므로 관리 설정 파일 또는 MDM을 통해 이 키를 전달합니다. |

Desktop 세션이 어느 위치에서 실행되는지에 따라 어떤 관리 설정이 Desktop 세션에 도달하는지가 결정됩니다. [`availableModels`](/docs/ko/model-config#restrict-model-selection)과 같은 모델 제한은 터미널 CLI와 동일한 방식으로 Desktop의 Claude Code 세션에서 적용됩니다. [표면 범위](/docs/ko/model-config#surface-coverage)를 참조하세요.

* **이 머신의 로컬 세션**: 디스크에 배포된 관리 설정 파일이 적용됩니다. 관리 콘솔을 통해 원격으로 푸시된 관리 설정도 세션이 조직 로그인 또는 직접 구성된 API 키로 인증할 때 Anthropic의 API에서 이러한 세션에 도달하며, 터미널 CLI와 동일한 [설정 우선순위](/docs/ko/settings#settings-precedence)를 따릅니다.
* **[클라우드 세션](#cloud-sessions)**: Anthropic이 관리하는 VM에서 실행되며 [서버 관리 설정](/docs/ko/server-managed-settings)만 수신합니다.
* **[SSH 세션](#ssh-sessions)**: 세션은 원격 호스트에서 관리 설정 파일을 읽습니다. Desktop 자체는 연결을 생성할 때 로컬 머신의 관리 설정에서 `sshConfigs` 및 `sshHostAllowlist`를 읽습니다.

`permissions.disableBypassPermissionsMode` 및 `disableAutoMode`는 사용자 및 프로젝트 설정에서도 작동하지만 관리 설정에 배치하면 사용자가 재정의하지 못하도록 방지합니다.

Claude Code는 사용자 설정, `--settings` 플래그, 관리 설정에서 `autoMode`를 읽지만 `.claude/settings.json` 또는 `.claude/settings.local.json`에서는 읽지 않습니다: 두 파일 모두 저장소 디렉토리에 있으므로 복제된 저장소 또는 빌드 단계가 자신의 분류기 규칙을 주입할 수 없습니다. v2.1.207 이전에는 Claude Code도 `.claude/settings.local.json`을 읽었습니다.

`allowManagedPermissionRulesOnly` 및 `allowManagedHooksOnly`를 포함한 관리 전용 설정의 전체 목록은 [관리 전용 설정](/docs/ko/permissions#managed-only-settings)을 참조하세요.

<h3 id="device-management-policies">
  장치 관리 정책
</h3>

IT 팀은 macOS의 MDM 또는 Windows의 그룹 정책을 통해 데스크톱 앱을 관리할 수 있습니다. 사용 가능한 정책에는 Claude Code 기능 활성화 또는 비활성화, 자동 업데이트 제어, 사용자 정의 배포 URL 설정이 포함됩니다.

* **macOS**: Jamf 또는 Kandji와 같은 도구를 사용하여 `com.anthropic.claudefordesktop` 기본 설정 도메인을 통해 구성합니다
* **Windows**: `SOFTWARE\Policies\Claude`의 레지스트리를 통해 구성합니다

<h3 id="network-access-requirements">
  네트워크 액세스 요구 사항
</h3>

Desktop은 Anthropic CDN 호스트에서 애플리케이션 코드 및 사용자 콘텐츠를 로드합니다.

```text theme={null}
anthropic.com
*.anthropic.com
claude.ai
*.claude.ai
claude.com
*.claude.com
claude.app
*.claude.app
*.claudeusercontent.com
*.claudemcpcontent.com
```

[OTLP](/docs/ko/monitoring-usage), LLM 게이트웨이 또는 MCP 서버에 대한 사용자 정의 포트를 구성하지 않는 한 트래픽은 포트 443의 HTTPS입니다.

프록시 서버, 사용자 정의 인증 기관, mTLS 및 독립 실행형 CLI가 필요한 도메인에 대해서는 [네트워크 구성](/docs/ko/network-config)을 참조하세요.

방화벽 와일드카드 수를 줄이려면 대신 이러한 Anthropic 호스트를 허용합니다. 특정 하위 도메인은 동적으로 생성되며 와일드카드로 유지되어야 합니다.

```text theme={null}
anthropic.com
api.anthropic.com
a-api.anthropic.com
a-cdn.anthropic.com
s-cdn.anthropic.com
assets-proxy.anthropic.com
claude.ai
a.claude.ai
a-cdn.claude.ai
assets.claude.ai
downloads.claude.ai
*.livepreview.claude.ai
claude.com
platform.claude.com
*.livepreview.claude.app
*.claudeusercontent.com
*.claudemcpcontent.com
```

<h3 id="authentication-and-sso">
  인증 및 SSO
</h3>

엔터프라이즈 조직은 모든 사용자에게 SSO를 요구할 수 있습니다. 계획 수준 세부 정보는 [인증](/docs/ko/authentication)을 참조하고 [SSO 설정](https://support.claude.com/en/articles/13132885-setting-up-single-sign-on-sso)에서 SAML 구성을 참조하세요. OIDC 설정은 [Claude Enterprise Administrator Guide](https://claude.com/resources/tutorials/claude-enterprise-administrator-guide)에서 다룹니다.

<h3 id="data-handling">
  데이터 처리
</h3>

Claude Code는 로컬 세션에서 코드를 로컬로 처리하거나 클라우드 세션에서 Anthropic의 클라우드 인프라에서 처리합니다. 대화 및 코드 컨텍스트는 처리를 위해 Anthropic의 API로 전송됩니다. 데이터 보존, 개인 정보 보호, 규정 준수에 대한 자세한 내용은 [데이터 처리](/docs/ko/data-usage)를 참조하세요.

<h3 id="deployment">
  배포
</h3>

Desktop은 엔터프라이즈 배포 도구를 통해 배포할 수 있습니다:

* **macOS**: Jamf 또는 Kandji와 같은 MDM을 통해 `.dmg` 설치 프로그램을 사용하여 배포합니다
* **Windows**: MSIX 패키지를 통해 배포합니다. 자동 설치를 포함한 엔터프라이즈 배포 옵션은 [Windows용 Claude Desktop 배포](https://support.claude.com/en/articles/12622703-deploy-claude-desktop-for-windows)를 참조하세요

방화벽에서 허용 목록에 추가할 도메인은 위의 [네트워크 액세스 요구 사항](#network-access-requirements)을 참조하세요. 프록시 설정, 사용자 정의 인증 기관, LLM 게이트웨이는 [네트워크 구성](/docs/ko/network-config)을 참조하세요.

전체 엔터프라이즈 구성 참조는 [엔터프라이즈 구성 가이드](https://support.claude.com/en/articles/12622667-enterprise-configuration)를 참조하세요.

<h2 id="coming-from-the-cli">
  CLI에서 오셨나요?
</h2>

이미 Claude Code CLI를 사용하는 경우 Desktop은 그래픽 인터페이스를 사용하여 동일한 기본 엔진을 실행합니다. 동일한 머신에서 동일한 프로젝트에서도 동시에 둘 다 실행할 수 있습니다. 각각은 별도의 세션 기록을 유지하지만 CLAUDE.md 파일을 통해 구성 및 프로젝트 메모리를 공유합니다.

CLI 세션을 Desktop으로 이동하려면 터미널에서 `/desktop`을 실행합니다. Claude는 세션을 저장하고 데스크톱 앱에서 열고 CLI를 종료합니다. 이 명령은 Claude 구독으로 로그인했을 때 macOS 및 Windows에서 사용 가능합니다. API 키 인증이나 Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry에서는 사용할 수 없습니다.

<Tip>
  Desktop vs CLI를 사용할 때: 병렬 세션을 한 창에서 관리하거나, 패널을 나란히 정렬하거나, 변경 사항을 시각적으로 검토하려고 할 때 Desktop을 사용합니다. 스크립팅, 자동화, 터미널 워크플로우를 선호할 때 CLI를 사용합니다.
</Tip>

<h3 id="cli-flag-equivalents">
  CLI 플래그 동등물
</h3>

이 표는 일반적인 CLI 플래그에 대한 데스크톱 앱 동등물을 보여줍니다. 나열되지 않은 플래그는 스크립팅 또는 자동화를 위해 설계되었기 때문에 데스크톱 동등물이 없습니다.

| CLI                                   | Desktop 동등물                                                                                                       |
| ------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| `--model sonnet`                      | 전송 버튼 옆의 모델 드롭다운                                                                                                  |
| `--resume`, `--continue`              | 사이드바의 세션을 클릭합니다                                                                                                   |
| `--permission-mode`                   | 전송 버튼 옆의 모드 선택기                                                                                                   |
| `--dangerously-skip-permissions`      | 권한 무시 모드. Pro 및 Max 플랜에서 Settings → Claude Code → "권한 무시 모드 허용"에서 활성화합니다. Team 및 Enterprise 플랜에서는 조직 정책이 이를 제어합니다 |
| `--add-dir`                           | 클라우드 세션에서 **+** 버튼으로 여러 저장소 추가                                                                                    |
| `--allowedTools`, `--disallowedTools` | 세션별 동등물이 없습니다. [설정 파일](/docs/ko/settings)의 권한 규칙이 여전히 적용됩니다.                                                           |
| `--verbose`                           | [Verbose 보기 모드](#switch-view-modes) (Transcript 보기 드롭다운)                                                          |
| `--print`, `--output-format`          | 사용할 수 없습니다. Desktop은 대화형만 가능합니다.                                                                                  |
| `ANTHROPIC_MODEL` env var             | 전송 버튼 옆의 모델 드롭다운                                                                                                  |
| `MAX_THINKING_TOKENS` env var         | 로컬 환경 편집기에서 설정합니다. [환경 구성](#environment-configuration)을 참조하세요.                                                    |

<h3 id="shared-configuration">
  공유 구성
</h3>

Desktop과 CLI는 동일한 구성 파일을 읽으므로 설정이 이월됩니다:

* **[CLAUDE.md](/docs/ko/memory)** 및 `CLAUDE.local.md` 파일 (프로젝트)은 둘 다에서 사용됩니다
* **[MCP servers](/docs/ko/mcp)** `~/.claude.json` 또는 `.mcp.json`에 구성된 것은 둘 다에서 작동합니다
* **[Hooks](/docs/ko/hooks)** 및 **[skills](/docs/ko/skills)** 설정에 정의된 것은 둘 다에 적용됩니다
* **[Settings](/docs/ko/settings)** `~/.claude.json` 및 `~/.claude/settings.json`에서 공유됩니다. `settings.json`의 권한 규칙, 허용된 도구 및 기타 설정은 Desktop 세션에 적용됩니다.
* **Models**: 동일한 [모델](/docs/ko/model-config#available-models)은 둘 다에서 사용 가능합니다. Desktop에서 전송 버튼 옆의 드롭다운에서 모델을 선택합니다. 세션 중에 동일한 드롭다운에서 모델을 변경할 수 있습니다.

<Note>
  **Claude Desktop 채팅 앱의 MCP 서버**: Desktop 앱은 `claude_desktop_config.json`에서 MCP 서버를 Code 탭 세션으로 로드하며, `~/.claude.json` 및 `.mcp.json`의 서버와 함께 작동합니다. `claude_desktop_config.json`에 정의된 서버는 Desktop 채팅 표면과 Code 탭 모두에서 사용 가능합니다.

  독립 실행형 CLI는 `claude_desktop_config.json`을 읽지 않습니다. macOS 및 WSL에서 `claude mcp add-from-claude-desktop`을 실행하여 해당 서버를 `~/.claude.json`으로 복사합니다. 가져오기 흐름 및 범위 옵션은 [Claude Desktop에서 MCP 서버 가져오기](/docs/ko/mcp#import-mcp-servers-from-claude-desktop)를 참조하세요.
</Note>

<h3 id="feature-comparison">
  기능 비교
</h3>

이 표는 CLI와 Desktop 간의 핵심 기능을 비교합니다. CLI 플래그의 전체 목록은 [CLI 참조](/docs/ko/cli-reference)를 참조하세요.

| 기능                                                    | CLI                                                             | Desktop                                                                                                                                                                                                                                                                                              |
| ----------------------------------------------------- | --------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 권한 모드                                                 | `dontAsk`를 포함한 모든 모드                                            | Manual, Accept edits, Plan, Auto. Bypass permissions는 모드 선택기에서 활성화된 후 나타납니다: Pro 및 Max 플랜에서는 Settings 토글을 통해, Team 및 Enterprise 플랜에서는 조직 정책을 통해                                                                                                                                                      |
| `--dangerously-skip-permissions`                      | CLI 플래그                                                         | 권한 무시 모드. Pro 및 Max 플랜에서 Settings → Claude Code → "권한 무시 모드 허용"에서 활성화합니다. Team 및 Enterprise 플랜에서는 조직 정책이 이를 제어합니다                                                                                                                                                                                    |
| [Third-party providers](/docs/ko/third-party-integrations) | Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry | Anthropic의 API (기본값). 게이트웨이 라우팅의 경우 [데스크톱 앱을 게이트웨이에 연결](/docs/ko/llm-gateway-connect#desktop-app)을 참조하세요. Code 탭을 Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry 또는 자체 호스팅 LLM 게이트웨이에서 실행하려면 [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)를 참조하세요. |
| [MCP servers](/docs/ko/mcp)                                | 설정 파일에 구성                                                       | 로컬 및 SSH 세션의 Connectors UI 또는 설정 파일                                                                                                                                                                                                                                                                  |
| [Plugins](/docs/ko/plugins)                                | `/plugin` 명령                                                    | 플러그인 관리자 UI                                                                                                                                                                                                                                                                                          |
| @mention 파일                                           | 텍스트 기반                                                          | 자동 완성 포함; 로컬 및 SSH 세션만                                                                                                                                                                                                                                                                               |
| 파일 첨부                                                 | 사용할 수 없음                                                        | 이미지, PDF                                                                                                                                                                                                                                                                                             |
| 세션 격리                                                 | [`--worktree`](/docs/ko/cli-reference) 플래그                           | 자동 worktrees                                                                                                                                                                                                                                                                                         |
| 여러 세션                                                 | 별도 터미널                                                          | 사이드바 탭                                                                                                                                                                                                                                                                                               |
| 반복 작업                                                 | Cron 작업, CI 파이프라인                                               | [예약된 작업](/docs/ko/desktop-scheduled-tasks)                                                                                                                                                                                                                                                                |
| 컴퓨터 사용                                                | [macOS에서 `/mcp`를 통해 활성화](/docs/ko/computer-use)                      | [macOS 및 Windows에서 앱 및 화면 제어](#let-claude-use-your-computer)                                                                                                                                                                                                                                         |
| Dispatch 통합                                           | 사용할 수 없음                                                        | [사이드바의 Dispatch 세션](#sessions-from-dispatch)                                                                                                                                                                                                                                                         |
| 스크립팅 및 자동화                                            | [`--print`](/docs/ko/cli-reference), [Agent SDK](/docs/ko/headless)       | 사용할 수 없음                                                                                                                                                                                                                                                                                             |

<h3 id="what’s-not-available-in-desktop">
  Desktop에서 사용할 수 없는 것
</h3>

다음 기능은 CLI 또는 VS Code 확장에서만 사용 가능하며, 명시된 경우를 제외하고는 그렇습니다:

* **Third-party providers**: Desktop은 Anthropic의 API에 기본적으로 연결됩니다. Desktop을 게이트웨이를 통해 라우팅하려면 [데스크톱 앱을 게이트웨이에 연결](/docs/ko/llm-gateway-connect#desktop-app)을 참조하세요. Enterprise 배포는 [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)를 통해 Google Cloud의 Agent Platform 및 게이트웨이 공급자를 구성할 수 있습니다. Amazon Bedrock 또는 Microsoft Foundry의 경우 CLI에서 [빠른 시작](/docs/ko/quickstart)을 참조하세요. 위 섹션의 예외로, [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)는 Code 탭을 Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry 또는 자체 호스팅 LLM 게이트웨이에서 실행합니다.
* **Linux (베타)**: 컴퓨터 사용은 아직 Linux 데스크톱 앱에서 사용할 수 없습니다. [Linux에서 Claude Desktop](/docs/ko/desktop-linux)을 참조하세요.
* **Inline code suggestions**: Desktop은 자동 완성 스타일 제안을 제공하지 않습니다. 대화형 프롬프트 및 명시적 코드 변경을 통해 작동합니다.
* **Agent teams**: 서로 메시지를 주고받는 병렬 Claude Code 세션은 [CLI](/docs/ko/agent-teams)에서 사용 가능하며 Desktop에서는 사용할 수 없습니다. 한 세션 내에서 다중 에이전트 작업의 경우 [동적 워크플로우](/docs/ko/workflows)를 사용합니다. 이는 Desktop에서 실행됩니다.
* **Terminal-dialog commands**: 터미널에서 대화형 패널을 여는 기본 제공 명령은 Code 탭에서 다르게 작동합니다. [설정 파일](/docs/ko/settings)을 직접 편집하여 권한 규칙 및 구성을 관리하거나 독립 실행형 CLI에서 명령을 실행합니다.
  * 인수 형식이 없는 명령 (예: `/permissions`)은 `isn't available in this environment`로 응답합니다.
  * `/config`는 Settings → Claude Code를 엽니다. 명령 뒤의 텍스트는 무시되므로 `/config theme=dark`는 테마를 설정하지 않습니다.

<h2 id="troubleshooting">
  문제 해결
</h2>

아래 섹션은 데스크톱 앱에 특정한 문제를 다룹니다. `API Error: 500`, `529 Overloaded`, `429`, `Prompt is too long`과 같이 채팅에 나타나는 런타임 API 오류의 경우 [오류 참조](/docs/ko/errors)를 참조하세요. 이러한 오류와 수정 사항은 CLI, 데스크톱, 웹에서 동일합니다.

<h3 id="check-your-version">
  버전 확인하기
</h3>

실행 중인 데스크톱 앱의 버전을 보려면:

* **macOS**: 메뉴 모음에서 **Claude**를 클릭한 다음 **About Claude**를 클릭합니다
* **Windows**: **Help**를 클릭한 다음 **About**을 클릭합니다

버전 번호를 클릭하여 클립보드에 복사합니다.

<h3 id="403-or-authentication-errors-in-the-code-tab">
  Code 탭의 403 또는 인증 오류
</h3>

Code 탭을 사용할 때 `Error 403: Forbidden` 또는 기타 인증 실패가 표시되면:

1. 앱 메뉴에서 로그아웃했다가 다시 로그인합니다. 이것이 가장 일반적인 수정입니다.
2. 활성 유료 구독이 있는지 확인합니다: Pro, Max, Team 또는 Enterprise.
3. CLI는 작동하지만 Desktop은 작동하지 않으면 데스크톱 앱을 완전히 종료하고 (창만 닫지 말고) 다시 열고 로그인합니다.
4. 인터넷 연결 및 프록시 설정을 확인합니다.

<h3 id="blank-or-stuck-screen-on-launch">
  시작 시 빈 화면 또는 정지된 화면
</h3>

앱이 열리지만 빈 화면이나 응답하지 않는 화면이 표시되면:

1. 앱을 다시 시작합니다.
2. 보류 중인 업데이트를 확인합니다. macOS 및 Windows에서 앱은 시작 시 자동으로 업데이트됩니다. Linux에서는 [Claude Desktop on Linux](/docs/ko/desktop-linux)에 설명된 대로 apt를 통해 업데이트합니다.
3. 관리되는 네트워크에서 방화벽이 [네트워크 액세스 요구 사항](#network-access-requirements)의 CDN 호스트를 허용하는지 확인합니다.
4. Windows에서 Event Viewer의 **Windows Logs → Application** 아래에서 충돌 로그를 확인합니다.

<h3 id="failed-to-load-session">
  "Failed to load session"
</h3>

`Failed to load session`이 표시되면 선택한 폴더가 더 이상 존재하지 않거나, Git 저장소에 설치되지 않은 Git LFS가 필요하거나, 파일 권한이 액세스를 방지할 수 있습니다. 다른 폴더를 선택하거나 앱을 다시 시작해 보세요.

<h3 id="session-not-finding-installed-tools">
  세션이 설치된 도구를 찾지 못함
</h3>

Claude가 `npm`, `node` 또는 기타 CLI 명령과 같은 도구를 찾을 수 없으면 도구가 일반 터미널에서 작동하는지 확인하고, 셸 프로필이 PATH를 올바르게 설정하는지 확인하고, 데스크톱 앱을 다시 시작하여 환경 변수를 다시 로드합니다.

<h3 id="git-and-git-lfs-errors">
  Git 및 Git LFS 오류
</h3>

Windows에서 Git은 로컬 세션을 시작하기 위해 Code 탭에 필요합니다. "Git is required"가 표시되면 [Windows용 Git](https://git-scm.com/downloads/win)을 설치하고 앱을 다시 시작합니다.

"Git LFS is required by this repository but is not installed"가 표시되면 [git-lfs.com](https://git-lfs.com/)에서 Git LFS를 설치하고 `git lfs install`을 실행한 다음 앱을 다시 시작합니다.

<h3 id="mcp-servers-not-working-on-windows">
  Windows에서 MCP 서버가 작동하지 않음
</h3>

MCP 서버 토글이 응답하지 않거나 Windows에서 서버가 연결되지 않으면 서버가 설정에 올바르게 구성되었는지 확인하고, 앱을 다시 시작하고, Task Manager에서 서버 프로세스가 실행 중인지 확인하고, 연결 오류에 대한 서버 로그를 검토합니다.

<h3 id="app-won’t-quit">
  앱이 종료되지 않음
</h3>

* **macOS**: Cmd+Q를 누릅니다. 앱이 응답하지 않으면 Cmd+Option+Esc로 강제 종료를 사용하고 Claude를 선택한 다음 Force Quit를 클릭합니다.
* **Windows**: Ctrl+Shift+Esc로 Task Manager를 사용하여 Claude 프로세스를 종료합니다.

<h3 id="windows-specific-issues">
  Windows 특정 문제
</h3>

* **설치 후 PATH가 업데이트되지 않음**: 새 터미널 창을 엽니다. PATH 업데이트는 새 터미널 세션에만 적용됩니다.
* **동시 설치 오류**: 진행 중인 다른 설치에 대한 오류가 표시되지만 없으면 관리자로 설치 프로그램을 실행해 보세요.

<h3 id="branch-doesn’t-exist-yet-when-opening-in-cli">
  CLI에서 열 때 "Branch doesn't exist yet"
</h3>

원격 세션은 로컬 머신에 존재하지 않는 브랜치를 만들 수 있습니다. 세션 도구 모음의 브랜치 이름을 클릭하여 복사한 다음 로컬로 가져옵니다:

```bash theme={null}
git fetch origin <branch-name>
git checkout <branch-name>
```

<h3 id="still-stuck">
  여전히 막혔나요?
</h3>

* 데스크톱 앱에서 Help → Get Support를 열거나 [Claude 지원 센터](https://support.claude.com/)를 직접 방문합니다
* 독립 실행형 `claude` CLI에서도 재현되는 문제의 경우 [GitHub Issues](https://github.com/anthropics/claude-code/issues)에서 검색하거나 버그를 제출합니다

문제를 보고할 때 데스크톱 앱 버전, 운영 체제, 정확한 오류 메시지, 관련 로그를 포함합니다. macOS에서는 Console.app을 확인합니다. Windows에서는 Event Viewer → Windows Logs → Application을 확인합니다.
