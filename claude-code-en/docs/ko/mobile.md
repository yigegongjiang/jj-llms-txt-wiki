> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 모바일에서 Claude Code

> Claude 앱(iOS 및 Android)을 통해 휴대폰에서 Claude Code 작업을 시작, 모니터링 및 조종합니다.

Claude 앱([iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) 및 [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude))은 코드가 실행되는 장소가 아니라 Claude Code 세션의 클라이언트입니다. 휴대폰에서 [클라우드 세션](#start-and-monitor-cloud-sessions) 및 클라우드의 [프로젝트](/docs/ko/claude-projects)에 접근하거나, [Remote Control](#continue-a-local-session-with-remote-control)을 통해 자신의 머신에서 실행 중인 세션에 접근하거나, [Dispatch](/docs/ko/desktop#sessions-from-dispatch)를 통해 Desktop 앱에 접근할 수 있습니다.

<Note>
  Claude Code는 별도의 모바일 앱이 없습니다. 클라우드 세션과 Remote Control은 모두 Claude 앱의 **Code** 탭에 있으며, Dispatch는 앱에서 메시지를 보내 요청하는 작업입니다.
</Note>

<h2 id="get-the-app">
  앱 다운로드
</h2>

<Steps>
  <Step title="Claude 앱 다운로드">
    [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) 또는 [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude)용 Claude 앱을 설치합니다. iPad에서는 동일한 iOS 앱을 설치합니다.

    <Tip>
      Claude Code 세션에서 `/mobile`을 실행하여 [claude.ai/mobile](https://claude.ai/mobile)에 대한 QR 코드를 표시합니다. 이 코드는 휴대폰에 맞는 앱 스토어를 엽니다. `/ios` 및 `/android`도 동일한 작업을 수행합니다.
    </Tip>
  </Step>

  <Step title="로그인">
    Claude Code에 사용하는 동일한 claude.ai 계정 및 조직으로 로그인합니다. 클라우드 세션과 Remote Control은 claude.ai 계정이 필요하므로 Anthropic Console API 키나 Amazon Bedrock과 같은 타사 제공자로는 접근할 수 없습니다.
  </Step>

  <Step title="Code 탭 열기">
    앱의 네비게이션에서 **Code**를 탭하여 세션에 접근하거나, 휴대폰에서 [claude.ai/code/new](https://claude.ai/code/new)를 열어 앱에서 새 Code 세션을 시작합니다. Code 탭이 보이지 않으면 사용자의 플랜이나 조직에 이러한 기능이 포함되지 않을 수 있습니다. [구독 플랜별 가용성](/docs/ko/feature-availability#availability-by-subscription-plan)을 참조하세요.
  </Step>
</Steps>

<h2 id="work-from-your-phone">
  휴대폰에서 작업하기
</h2>

앱에서 클라우드 세션을 시작하거나, 프로젝트를 열거나, 컴퓨터에서 실행 중인 Claude Code 세션을 조종하거나, Dispatch에 작업을 메시지로 보낼 수 있습니다. 앱은 각각에 대해 동일하지만 작업이 발생하는 위치가 다릅니다.

| 기능                                             | 연결 대상                            | 사용 시기                                                                                        |
| :--------------------------------------------- | :------------------------------- | :------------------------------------------------------------------------------------------- |
| [클라우드 세션](/docs/ko/claude-code-on-the-web)          | Anthropic 관리 클라우드 인프라의 세션        | 저장소가 GitHub에 있고 휴대폰을 치워도 작업이 계속 실행되어야 할 때입니다. 설정하려면 [클라우드 빠른 시작](/docs/ko/web-quickstart)을 참조하세요. |
| [프로젝트](/docs/ko/claude-projects)                    | Claude가 병렬 클라우드 세션을 스레드로 조정하는 대화 | 단일 작업이 아닌 관련 작업의 흐름이 있고 어떤 스레드가 완료되었거나 사용자가 필요한지 확인하려고 할 때입니다.                               |
| [Remote Control](/docs/ko/remote-control)           | 컴퓨터에서 실행 중인 Claude Code 세션       | 작업에 로컬 파일 시스템, 도구 또는 MCP 서버가 필요할 때입니다.                                                       |
| [Dispatch](/docs/ko/desktop#sessions-from-dispatch) | 컴퓨터의 Desktop 앱                   | 작업을 메시지로 보내고 Dispatch가 실행 방법을 결정하도록 하려고 할 때입니다. Pro 또는 Max 플랜이 필요합니다.                        |

컴퓨터가 꺼져 있을 경우 클라우드 세션 또는 프로젝트를 사용하세요. 이들은 클라우드에서 실행되며 노트북을 닫은 후에도 계속됩니다. Remote Control과 Dispatch는 자신의 머신을 조종하므로 Claude Code 또는 Desktop 앱이 실행 중인 상태로 켜져 있어야 합니다. Remote Control 세션 중에 머신이 절전 모드로 전환되면 다시 온라인 상태가 될 때 Claude Code가 다시 연결됩니다.

더 자세한 비교는 [터미널에서 멀리 떨어져 있을 때 작업하기](/docs/ko/platforms#work-when-you-are-away-from-your-terminal)를 참조하세요.

클라우드 세션과 Remote Control은 **Code** 탭에서 실행됩니다. 앱에서 작업으로 메시지를 보내는 Dispatch의 경우 [Dispatch의 세션](/docs/ko/desktop#sessions-from-dispatch)을 참조하세요.

<h3 id="start-and-monitor-cloud-sessions">
  클라우드 세션 시작 및 모니터링
</h3>

클라우드 세션은 Anthropic 관리 클라우드 인프라에서 작업을 실행하므로 휴대폰을 치운 후에도 세션이 계속됩니다. Code 탭에서 저장소와 분기를 선택하고, 작업을 설명한 후 제출합니다. 세션은 기기 간에 유지됩니다. 노트북에서 시작한 작업은 휴대폰에서 검토할 준비가 되어 있고, 휴대폰에서 시작한 작업은 책상으로 돌아올 때 대기 중입니다.

앱에서 세션을 열어 진행 상황을 확인하거나, Claude의 질문에 답하거나, 새로운 방향으로 조종할 수 있습니다. Claude에게 [풀 요청을 감시](/docs/ko/claude-code-on-the-web#auto-fix-pull-requests)하고 CI 실패 또는 검토 의견을 도착하는 대로 수정하도록 할 수도 있습니다. GitHub를 연결하고 환경을 설정하려면 [클라우드 빠른 시작](/docs/ko/web-quickstart)을 따르고, 클라우드 세션이 할 수 있는 모든 것은 [웹의 Claude Code](/docs/ko/claude-code-on-the-web)를 참조하세요.

<h3 id="continue-a-local-session-with-remote-control">
  Remote Control로 로컬 세션 계속하기
</h3>

Remote Control은 Claude 앱을 머신에서 실행 중인 Claude Code 세션에 연결하므로 코드 실행 및 파일 시스템 접근은 로컬로 유지되면서 휴대폰에서 세션을 조종합니다. 컴퓨터에서 `claude remote-control`로 세션을 시작하거나, 이미 열려 있는 세션에서 `/remote-control`을 실행합니다. 그런 다음 터미널이 표시할 수 있는 QR 코드를 스캔하거나, Claude 앱을 열고 **Code**를 탭한 후 목록에서 세션을 선택합니다. 각 옵션에 대해 [다른 기기에서 연결](/docs/ko/remote-control#connect-from-another-device)을 참조하세요.

Claude 앱에서 첨부 파일을 추가하면 로컬 세션에도 도달합니다.

* **사진**: Claude는 첨부된 사진을 메시지의 일부로 직접 봅니다. Claude Code는 또한 각 사진을 `~/.claude/uploads/` 아래에 저장하고 저장된 파일 경로를 Claude에 알려주므로 Claude는 생성하는 파일에 이미지를 복사할 수 있습니다.
* **기타 파일**: Claude Code는 이들을 머신에 다운로드하고 `@` 파일 참조로 Claude에 전달합니다.

요구 사항, 호출 모드 및 문제 해결은 [Remote Control 개요](/docs/ko/remote-control)를 참조하세요.

<h3 id="get-push-notifications">
  푸시 알림 받기
</h3>

Remote Control이 활성화되면 Claude는 휴대폰에 푸시 알림을 보낼 수 있습니다. 일반적으로 오래 실행되는 작업이 완료되거나 사용자의 결정이 필요할 때입니다. 프롬프트에서 `notify me when the tests finish`와 같이 요청할 수도 있습니다. 두 개의 `/config` 토글과 전달 문제 해결은 [모바일 푸시 알림](/docs/ko/remote-control#mobile-push-notifications)을 참조하세요.

Dispatch는 생성한 Code 세션이 완료되거나 승인이 필요할 때 자체 알림을 보냅니다. 이는 [Dispatch의 세션](/docs/ko/desktop#sessions-from-dispatch)에서 설명합니다.

<h2 id="limitations">
  제한 사항
</h2>

모바일 클라이언트는 세션이 필요한 대부분의 것을 다루지만 몇 가지 제한 사항이 있습니다.

* **로컬 전용 명령**: `/plugin` 및 `/resume`과 같이 터미널 인터페이스에서만 실행되는 명령은 앱에서 작동하지 않습니다. [Remote Control 제한 사항](/docs/ko/remote-control#limitations)은 모바일에서 작동하는 명령과 동작이 어떻게 다른지 나열합니다.
* **권한 모드**: 클라우드 세션은 모드 드롭다운에서 편집 수락, Plan 및 Auto를 제공하고, Remote Control 세션은 Manual, 편집 수락 및 Plan을 제공합니다. 두 경우 모두 앱에서 Bypass 권한을 선택할 수 없으며, Remote Control 세션에 대해 Auto를 선택할 수 없습니다. [권한 모드 전환](/docs/ko/permission-modes#switch-permission-modes)을 참조하세요.
* **Dispatch 플랜**: Dispatch는 Pro 또는 Max 플랜이 필요하며 Team 또는 Enterprise에서는 사용할 수 없습니다.

<h2 id="related-resources">
  관련 리소스
</h2>

* [플랫폼 및 통합](/docs/ko/platforms): Claude Code가 실행되는 모든 표면 비교
* [웹의 Claude Code](/docs/ko/claude-code-on-the-web): 클라우드 세션이 실행되는 방식 및 터미널과 작업을 주고받는 방법
* [클라우드 환경 구성](/docs/ko/cloud-environments): 클라우드 세션의 네트워크 접근 수준, 환경 변수 및 설정 스크립트
* [Remote Control](/docs/ko/remote-control): 모든 기기에서 로컬 세션 계속하기
* [Dispatch의 세션](/docs/ko/desktop#sessions-from-dispatch): Dispatch 작업이 Desktop 앱에서 Code 세션이 되는 방식
* [Channels](/docs/ko/channels): 작업이 머신에서 실행되는 동안 Telegram, Discord 또는 iMessage를 통해 휴대폰에서 Claude에 질문하기
* [Slack의 Claude Code](/docs/ko/slack): `@Claude`를 언급하여 Slack 워크스페이스에서 코딩 작업 위임하기
