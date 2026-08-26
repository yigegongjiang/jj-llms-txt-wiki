> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 플랫폼 및 통합

> Claude Code를 실행할 위치를 선택하고 연결할 항목을 결정합니다. CLI, Desktop, VS Code, JetBrains, 웹, 모바일 및 Chrome, Slack, CI/CD와 같은 통합을 비교합니다.

Claude Code는 모든 곳에서 동일한 기본 엔진을 실행하지만, 각 플랫폼은 다양한 작업 방식에 맞게 조정됩니다. 이 페이지는 워크플로우에 적합한 플랫폼을 선택하고 이미 사용 중인 도구를 연결하는 데 도움을 줍니다.

<h2 id="where-to-run-claude-code">
  Claude Code를 실행할 위치
</h2>

작업 방식과 프로젝트가 있는 위치에 따라 플랫폼을 선택합니다.

| 플랫폼                               | 최적 용도                                                  | 제공 기능                                                                                                                                              |
| :-------------------------------- | :----------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------- |
| [CLI](/docs/ko/quickstart)             | 터미널 워크플로우, 스크립팅, 원격 서버                                 | 전체 기능 세트, [Agent SDK](/docs/ko/headless), macOS의 [컴퓨터 사용](/docs/ko/computer-use) (Pro 및 Max), 타사 제공자                                                         |
| [Desktop](/docs/ko/desktop)            | 시각적 검토, 병렬 세션, 관리형 설정                                  | Diff 뷰어, 앱 미리보기, Pro 및 Max의 [컴퓨터 사용](/docs/ko/desktop#let-claude-use-your-computer) 및 [Dispatch](/docs/ko/desktop#sessions-from-dispatch)                    |
| [VS Code](/docs/ko/vs-code)            | VS Code 내에서 터미널로 전환하지 않고 작업                            | 인라인 Diff, 통합 터미널, 파일 컨텍스트                                                                                                                          |
| [JetBrains](/docs/ko/jetbrains)        | IntelliJ, PyCharm, WebStorm 또는 기타 JetBrains IDE 내에서 작업 | Diff 뷰어, 선택 공유, 터미널 세션                                                                                                                             |
| [Web](/docs/ko/claude-code-on-the-web) | 많은 조작이 필요하지 않은 장기 실행 작업 또는 오프라인 상태에서도 계속되어야 하는 작업      | Anthropic 관리형 클라우드, 연결 해제 후에도 계속 실행                                                                                                                |
| Mobile                            | 컴퓨터에서 멀리 떨어져 있을 때 작업 시작 및 모니터링                         | iOS 및 Android용 Claude 앱의 클라우드 세션, 로컬 세션용 [Remote Control](/docs/ko/remote-control), Pro 및 Max의 Desktop으로 [Dispatch](/docs/ko/desktop#sessions-from-dispatch) |

CLI는 터미널 기반 작업을 위한 가장 완전한 플랫폼입니다. 스크립팅 및 Agent SDK는 CLI 전용입니다. 타사 제공자는 [VS Code](/docs/ko/vs-code#use-third-party-providers)에서도 작동합니다. Enterprise [Desktop](/docs/ko/desktop) 배포는 Google Cloud의 Agent Platform을 지원하며, Desktop은 [게이트웨이 제공자](/docs/ko/llm-gateway-connect#desktop-app)를 지원합니다. Amazon Bedrock 또는 Microsoft Foundry의 경우 CLI 또는 VS Code를 사용하거나, 해당 제공자에서 Code 탭을 실행하는 [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)를 사용합니다. Desktop과 IDE 확장 프로그램은 일부 CLI 전용 기능을 포기하는 대신 시각적 검토와 더 긴밀한 편집기 통합을 제공합니다. 웹은 Anthropic의 클라우드에서 실행되므로 연결을 해제한 후에도 작업이 계속됩니다. Mobile은 동일한 클라우드 세션으로의 씬 클라이언트이거나 Remote Control을 통한 로컬 세션으로의 씬 클라이언트이며, Dispatch를 통해 Desktop으로 작업을 보낼 수 있습니다.

동일한 프로젝트에서 여러 플랫폼을 혼합하여 사용할 수 있습니다. 구성, 프로젝트 메모리 및 MCP 서버는 로컬 플랫폼 간에 공유됩니다.

<h2 id="connect-your-tools">
  도구 연결
</h2>

통합을 통해 Claude는 코드베이스 외부의 서비스와 작업할 수 있습니다.

| 통합                                   | 기능                               | 사용 용도                          |
| :----------------------------------- | :------------------------------- | :----------------------------- |
| [Chrome](/docs/ko/chrome)                 | 로그인된 세션으로 브라우저를 제어합니다            | 웹 앱 테스트, 양식 작성, API 없이 사이트 자동화 |
| [GitHub Actions](/docs/ko/github-actions) | CI 파이프라인에서 Claude를 실행합니다         | 자동화된 PR 검토, 이슈 분류, 예약된 유지보수    |
| [GitLab CI/CD](/docs/ko/gitlab-ci-cd)     | GitHub Actions와 동일하지만 GitLab용입니다 | GitLab의 CI 기반 자동화              |
| [Code Review](/docs/ko/code-review)       | 모든 PR을 자동으로 검토합니다                | 인간 검토 전에 버그 포착                 |
| [Slack](/docs/ko/slack)                   | 채널의 `@Claude` 멘션에 응답합니다          | 팀 채팅에서 버그 보고를 풀 요청으로 변환        |

여기에 나열되지 않은 통합의 경우, [MCP 서버](/docs/ko/mcp) 및 [커넥터](/docs/ko/desktop#connect-external-tools)를 사용하면 거의 모든 것을 연결할 수 있습니다. Linear, Notion, Google Drive 또는 자체 내부 API입니다.

<h2 id="work-when-you-are-away-from-your-terminal">
  터미널에서 멀리 떨어져 있을 때 작업
</h2>

Claude Code offers several ways to work when you're not at your terminal. They differ in what triggers the work, where Claude runs, and how much you need to set up.

|                                                          | Trigger                                                                                        | Claude runs on                                                                               | Setup                                                                                                                                | Best for                                                      |
| :------------------------------------------------------- | :--------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------ |
| [Dispatch](/docs/en/desktop#sessions-from-dispatch)           | Message a task from the Claude mobile app                                                      | Your machine (Desktop)                                                                       | [Pair the mobile app with Desktop](https://support.claude.com/en/articles/13947068)                                                  | Delegating work while you're away, minimal setup              |
| [Remote Control](/docs/en/remote-control)                     | Drive a running session from [claude.ai/code](https://claude.ai/code) or the Claude mobile app | Your machine (CLI or VS Code)                                                                | Run `claude remote-control`                                                                                                          | Steering in-progress work from another device                 |
| [Channels](/docs/en/channels)                                 | Push events from a chat app like Telegram or Discord, or your own server                       | Your machine (CLI)                                                                           | [Install a channel plugin](/docs/en/channels#quickstart) or [build your own](/docs/en/channels-reference)                                      | Reacting to external events like CI failures or chat messages |
| [Slack](/docs/en/slack)                                       | Mention `@Claude` in a team channel                                                            | Anthropic cloud                                                                              | [Install the Slack app](/docs/en/slack#setting-up-claude-code-in-slack) with [Claude Code on the web](/docs/en/claude-code-on-the-web) enabled | PRs and reviews from team chat                                |
| [Self-hosted environments](/docs/en/self-hosted-environments) | Start a [cloud session](/docs/en/claude-code-on-the-web) and pick your organization's environment   | Your organization's infrastructure                                                           | [Deploy runners](/docs/en/self-hosted-environments-quickstart), on Team and Enterprise plans                                              | Cloud sessions that must run inside your network              |
| [Scheduled tasks](/docs/en/scheduled-tasks)                   | Set a schedule                                                                                 | [CLI](/docs/en/scheduled-tasks), [Desktop](/docs/en/desktop-scheduled-tasks), or [cloud](/docs/en/routines) | Pick a frequency                                                                                                                     | Recurring automation like daily reviews                       |

시작할 위치가 확실하지 않으면 [CLI를 설치](/docs/ko/quickstart)하고 프로젝트 디렉토리에서 실행합니다. 터미널을 사용하지 않으려면 [Desktop](/docs/ko/desktop-quickstart)이 그래픽 인터페이스와 함께 동일한 엔진을 제공합니다.

<h2 id="related-resources">
  관련 리소스
</h2>

<h3 id="platforms">
  플랫폼
</h3>

* [CLI 빠른 시작](/docs/ko/quickstart): 터미널에서 첫 번째 명령을 설치하고 실행합니다
* [Desktop](/docs/ko/desktop): 시각적 Diff 검토, 병렬 세션, 컴퓨터 사용 및 Dispatch
* [VS Code](/docs/ko/vs-code): 편집기 내 Claude Code 확장 프로그램
* [JetBrains](/docs/ko/jetbrains): IntelliJ, PyCharm 및 기타 JetBrains IDE용 확장 프로그램
* [웹의 Claude Code](/docs/ko/claude-code-on-the-web): 연결 해제 후에도 계속 실행되는 클라우드 세션
* Mobile: [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) 및 [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude)용 Claude 앱으로 컴퓨터에서 멀리 떨어져 있을 때 작업 시작 및 모니터링

<h3 id="integrations">
  통합
</h3>

* [Chrome](/docs/ko/chrome): 로그인된 세션으로 브라우저 작업 자동화
* [컴퓨터 사용](/docs/ko/computer-use): Claude가 macOS에서 앱을 열고 화면을 제어하도록 허용
* [GitHub Actions](/docs/ko/github-actions): CI 파이프라인에서 Claude 실행
* [GitLab CI/CD](/docs/ko/gitlab-ci-cd): GitLab용 동일한 기능
* [Code Review](/docs/ko/code-review): 모든 풀 요청에 대한 자동 검토
* [Slack](/docs/ko/slack): 팀 채팅에서 작업을 보내고 PR을 받습니다

<h3 id="remote-access">
  원격 액세스
</h3>

* [Dispatch](/docs/ko/desktop#sessions-from-dispatch): 휴대폰에서 작업을 메시지로 보내면 Desktop 세션을 생성할 수 있습니다
* [Remote Control](/docs/ko/remote-control): 휴대폰 또는 브라우저에서 실행 중인 세션을 제어합니다
* [Channels](/docs/ko/channels): 채팅 앱 또는 자체 서버의 이벤트를 세션으로 푸시합니다
* [Scheduled tasks](/docs/ko/scheduled-tasks): 반복 일정에 따라 프롬프트를 실행합니다
