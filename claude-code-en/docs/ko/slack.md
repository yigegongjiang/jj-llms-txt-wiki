> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Slack의 Claude Code

> Slack 워크스페이스에서 직접 코딩 작업 위임

<Note>
  Slack의 Claude Code는 Team 및 Enterprise 워크스페이스를 위해 [Claude Tag](https://claude.com/product/tag)로 대체되고 있습니다. Claude Tag는 @Claude를 조직의 공유 ID로 실행하며 관리자가 구성한 액세스 권한을 가지고 동일한 Slack 앱에서 실행되므로 다시 설치할 필요가 없으며 기존 설정은 전환 중에 계속 작동합니다. 워크스페이스를 전환하려면 [이전 Slack의 Claude에서 마이그레이션](https://claude.com/docs/claude-tag/admins/migrate-from-earlier)을 참조하십시오.
</Note>

Slack의 Claude Code는 Claude Code의 강력한 기능을 Slack 워크스페이스에 직접 가져옵니다. `@Claude`를 언급하여 코딩 작업을 요청하면, Claude는 자동으로 의도를 감지하고 웹에서 Claude Code 세션을 생성하여 팀 대화를 떠나지 않고도 개발 작업을 위임할 수 있습니다.

이 통합은 기존 Slack용 Claude 앱을 기반으로 하지만 코딩 관련 요청에 대해 웹의 Claude Code로 지능형 라우팅을 추가합니다. 각 세션은 사용자의 Claude 계정에서 실행되며, 연결된 저장소와 사용자의 플랜 한도를 사용합니다.

<h2 id="use-cases">
  사용 사례
</h2>

* **버그 조사 및 수정**: Claude에 Slack 채널에서 보고된 버그를 조사하고 수정하도록 요청합니다.
* **빠른 코드 검토 및 수정**: Claude가 팀 피드백을 기반으로 작은 기능을 구현하거나 코드를 리팩토링하도록 합니다.
* **협업 디버깅**: 팀 토론에서 중요한 컨텍스트(예: 오류 재현 또는 사용자 보고)를 제공할 때, Claude는 이 정보를 사용하여 디버깅 접근 방식을 알릴 수 있습니다.
* **병렬 작업 실행**: Slack에서 코딩 작업을 시작하면서 다른 작업을 계속하고, 완료되면 알림을 받습니다.

<h2 id="prerequisites">
  필수 조건
</h2>

Slack에서 Claude Code를 사용하기 전에 다음을 확인하세요:

| 요구 사항          | 세부 정보                                                                              |
| :------------- | :--------------------------------------------------------------------------------- |
| Claude 플랜      | Claude Code 액세스가 있는 Pro, Max, Team 또는 Enterprise(프리미엄 시트 또는 Chat + Claude Code 시트) |
| 웹의 Claude Code | [웹의 Claude Code](/docs/ko/claude-code-on-the-web) 액세스가 활성화되어야 함                         |
| GitHub 계정      | 웹의 Claude Code에 연결되어 있으며 최소 하나의 저장소가 인증됨                                           |
| Slack 인증       | Claude 앱을 통해 Claude 계정에 연결된 Slack 계정                                               |

<h2 id="setting-up-claude-code-in-slack">
  Slack에서 Claude Code 설정
</h2>

<Steps>
  <Step title="Slack에 Claude 앱 설치">
    워크스페이스 관리자는 Slack 앱 마켓플레이스에서 Claude 앱을 설치해야 합니다. [Slack 앱 마켓플레이스](https://slack.com/marketplace/A08SF47R6P4)를 방문하여 "Slack에 추가"를 클릭하여 설치 프로세스를 시작합니다.
  </Step>

  <Step title="Claude 계정 연결">
    앱이 설치된 후 개별 Claude 계정을 인증합니다:

    1. 앱 섹션에서 "Claude"를 클릭하여 Slack에서 Claude 앱을 엽니다
    2. 앱 홈 탭으로 이동합니다
    3. "연결"을 클릭하여 Slack 계정을 Claude 계정과 연결합니다
    4. 브라우저에서 인증 흐름을 완료합니다
  </Step>

  <Step title="웹의 Claude Code 구성">
    웹의 Claude Code가 제대로 구성되어 있는지 확인합니다:

    * [claude.ai/code](https://claude.ai/code)를 방문하여 Slack에 연결한 동일한 계정으로 로그인합니다
    * 아직 연결되지 않은 경우 GitHub 계정을 연결합니다
    * Claude가 작업할 수 있도록 최소 하나의 저장소를 인증합니다
  </Step>

  <Step title="라우팅 모드 선택">
    계정을 연결한 후 Claude가 Slack의 메시지를 처리하는 방식을 구성합니다. Slack의 Claude 앱 홈으로 이동하여 **라우팅 모드** 설정을 찾습니다.

    | 모드          | 동작                                                                                                                                    |
    | :---------- | :------------------------------------------------------------------------------------------------------------------------------------ |
    | **코드만**     | Claude는 모든 @mentions을 Claude Code 세션으로 라우팅합니다. Claude를 Slack에서 개발 작업 전용으로 사용하는 팀에 가장 적합합니다.                                           |
    | **코드 + 채팅** | Claude는 각 메시지를 분석하고 Claude Code(코딩 작업용)와 Claude Chat(작성, 분석 및 일반 질문용) 간에 지능형으로 라우팅합니다. 모든 유형의 작업에 대해 단일 @Claude 진입점을 원하는 팀에 가장 적합합니다. |

    <Note>
      코드 + 채팅 모드에서 Claude가 메시지를 채팅으로 라우팅했지만 코딩 세션을 원했다면 "코드로 다시 시도"를 클릭하여 Claude Code 세션을 대신 생성할 수 있습니다. 마찬가지로 코드로 라우팅되었지만 채팅 세션을 원했다면 해당 스레드에서 해당 옵션을 선택할 수 있습니다.
    </Note>
  </Step>

  <Step title="Claude를 채널에 추가">
    Claude는 설치 후 자동으로 채널에 추가되지 않습니다. 채널에서 Claude를 사용하려면 해당 채널에서 `/invite @Claude`를 입력하여 초대합니다. Claude는 추가된 채널에서만 @mentions에 응답할 수 있습니다.
  </Step>
</Steps>

<h2 id="how-it-works">
  작동 방식
</h2>

<h3 id="automatic-detection">
  자동 감지
</h3>

Slack 채널이나 스레드에서 @Claude를 언급하면, Claude는 자동으로 메시지를 분석하여 코딩 작업인지 여부를 결정합니다. Claude가 코딩 의도를 감지하면 일반 채팅 어시스턴트로 응답하는 대신 요청을 웹의 Claude Code로 라우팅합니다.

자동으로 감지되지 않더라도 Claude에 요청을 코딩 작업으로 처리하도록 명시적으로 지시할 수 있습니다.

<Note>
  Slack의 Claude Code는 채널(공개 또는 비공개)에서만 작동합니다. 직접 메시지(DM)에서는 작동하지 않습니다.
</Note>

<h3 id="context-gathering">
  컨텍스트 수집
</h3>

**스레드에서**: 스레드에서 @Claude를 언급하면 전체 대화를 이해하기 위해 해당 스레드의 모든 메시지에서 컨텍스트를 수집합니다.

**채널에서**: 채널에서 직접 언급되면 Claude는 관련 컨텍스트를 위해 최근 채널 메시지를 살펴봅니다.

이 컨텍스트는 Claude가 문제를 이해하고, 적절한 저장소를 선택하고, 작업에 대한 접근 방식을 알리는 데 도움이 됩니다.

<Warning>
  Slack에서 @Claude가 호출되면 Claude는 요청을 더 잘 이해하기 위해 대화 컨텍스트에 액세스할 수 있습니다. Claude는 컨텍스트의 다른 메시지의 지시를 따를 수 있으므로 사용자는 Claude를 신뢰할 수 있는 Slack 대화에서만 사용해야 합니다.
</Warning>

<h3 id="session-flow">
  세션 흐름
</h3>

1. **시작**: @Claude를 코딩 요청과 함께 언급합니다
2. **감지**: Claude가 메시지를 분석하고 코딩 의도를 감지합니다
3. **세션 생성**: claude.ai/code에서 새로운 Claude Code 세션이 생성됩니다
4. **진행 상황 업데이트**: Claude는 작업이 진행됨에 따라 Slack 스레드에 상태 업데이트를 게시합니다
5. **완료**: 완료되면 Claude는 요약 및 작업 버튼과 함께 @mentions을 합니다
6. **검토**: "세션 보기"를 클릭하여 전체 기록을 보거나 "PR 생성"을 클릭하여 풀 요청을 엽니다

<h2 id="user-interface-elements">
  사용자 인터페이스 요소
</h2>

<h3 id="app-home">
  앱 홈
</h3>

앱 홈 탭은 연결 상태를 표시하고 Claude 계정을 Slack에서 연결하거나 연결 해제할 수 있습니다.

<h3 id="message-actions">
  메시지 작업
</h3>

* **세션 보기**: 수행된 모든 작업을 볼 수 있고, 세션을 계속하거나 추가 요청을 할 수 있는 브라우저에서 전체 Claude Code 세션을 엽니다.
* **PR 생성**: 세션의 변경 사항에서 직접 풀 요청을 생성합니다.
* **코드로 다시 시도**: Claude가 처음에 채팅 어시스턴트로 응답했지만 코딩 세션을 원했다면 이 버튼을 클릭하여 요청을 Claude Code 작업으로 다시 시도합니다.
* **저장소 변경**: Claude가 잘못 선택한 경우 다른 저장소를 선택할 수 있습니다.

<h3 id="repository-selection">
  저장소 선택
</h3>

Claude는 Slack 대화의 컨텍스트를 기반으로 저장소를 자동으로 선택합니다. 여러 저장소가 적용될 수 있는 경우 Claude는 올바른 저장소를 선택할 수 있는 드롭다운을 표시할 수 있습니다.

<h2 id="access-and-permissions">
  액세스 및 권한
</h2>

<h3 id="user-level-access">
  사용자 수준 액세스
</h3>

| 액세스 유형         | 요구 사항                                     |
| :------------- | :---------------------------------------- |
| Claude Code 세션 | 각 사용자는 자신의 Claude 계정에서 세션을 실행합니다          |
| 사용량 및 속도 제한    | 세션은 개별 사용자의 플랜 제한에 대해 계산됩니다               |
| 저장소 액세스        | 사용자는 개인적으로 연결한 저장소에만 액세스할 수 있습니다          |
| 세션 기록          | 세션은 claude.ai/code의 Claude Code 기록에 나타납니다 |

<h3 id="workspace-level-access">
  워크스페이스 수준 액세스
</h3>

Slack 워크스페이스 관리자는 Claude 앱을 워크스페이스에서 사용할 수 있는지 여부를 제어합니다:

| 제어                 | 설명                                                                        |
| :----------------- | :------------------------------------------------------------------------ |
| 앱 설치               | 워크스페이스 관리자는 Slack 앱 마켓플레이스에서 Claude 앱을 설치할지 여부를 결정합니다                     |
| Enterprise Grid 배포 | Enterprise Grid 조직의 경우 조직 관리자는 어떤 워크스페이스가 Claude 앱에 액세스할 수 있는지 제어할 수 있습니다 |
| 앱 제거               | 워크스페이스에서 앱을 제거하면 해당 워크스페이스의 모든 사용자에 대한 액세스가 즉시 취소됩니다                      |

<h3 id="channel-based-access-control">
  채널 기반 액세스 제어
</h3>

Claude는 설치 후 자동으로 채널에 추가되지 않습니다. 사용자는 Claude를 사용하려는 채널에 명시적으로 Claude를 초대해야 합니다:

* **초대 필요**: 채널에서 `/invite @Claude`를 입력하여 Claude를 해당 채널에 추가합니다
* **채널 멤버십이 액세스를 제어합니다**: Claude는 추가된 채널에서만 @mentions에 응답할 수 있습니다
* **채널을 통한 액세스 제어**: 관리자는 Claude가 초대된 채널과 해당 채널에 액세스할 수 있는 사용자를 관리하여 Claude Code 사용을 제어할 수 있습니다
* **비공개 채널 지원**: Claude는 공개 및 비공개 채널 모두에서 작동하여 팀에 가시성 제어의 유연성을 제공합니다

이 채널 기반 모델을 통해 팀은 Claude Code 사용을 특정 채널로 제한하여 워크스페이스 수준 권한 이상의 추가 액세스 제어 계층을 제공할 수 있습니다.

<h2 id="what’s-accessible-where">
  어디서 액세스할 수 있는지
</h2>

**Slack에서**: 상태 업데이트, 완료 요약 및 작업 버튼이 표시됩니다. 전체 기록은 보존되며 항상 액세스할 수 있습니다.

**웹에서**: 전체 대화 기록, 모든 코드 변경, 파일 작업 및 세션을 계속하거나 풀 요청을 생성할 수 있는 기능이 있는 완전한 Claude Code 세션입니다.

Enterprise 및 Team 계정의 경우 Slack의 Claude에서 생성된 세션은 조직에 자동으로 표시됩니다. 자세한 내용은 [웹의 Claude Code 공유](/docs/ko/claude-code-on-the-web#share-sessions)를 참조하세요.

<h2 id="best-practices">
  모범 사례
</h2>

<h3 id="writing-effective-requests">
  효과적인 요청 작성
</h3>

* **구체적으로**: 관련이 있을 때 파일 이름, 함수 이름 또는 오류 메시지를 포함합니다.
* **컨텍스트 제공**: 대화에서 명확하지 않은 경우 저장소 또는 프로젝트를 언급합니다.
* **성공 정의**: "완료"가 무엇인지 설명합니다. Claude가 테스트를 작성해야 합니까? 문서를 업데이트합니까? PR을 생성합니까?
* **스레드 사용**: 버그나 기능을 논의할 때 스레드에서 회신하여 Claude가 전체 컨텍스트를 수집할 수 있도록 합니다.

<h3 id="when-to-use-slack-vs-web">
  Slack과 웹 사용 시기
</h3>

**Slack을 사용할 때**: 컨텍스트가 이미 Slack 토론에 있을 때, 작업을 비동기적으로 시작하려고 할 때, 또는 가시성이 필요한 팀원과 협업할 때입니다.

**웹을 직접 사용할 때**: 파일을 업로드해야 할 때, 개발 중 실시간 상호 작용을 원할 때, 또는 더 길고 복잡한 작업을 수행할 때입니다.

<h2 id="troubleshooting">
  문제 해결
</h2>

<h3 id="claude-code-is-not-enabled-for-your-account">
  "Claude Code가 계정에서 활성화되지 않았습니다"
</h3>

이 오류는 Claude 계정에 아직 클라우드 환경이 없다는 의미이며, 관리자가 무언가를 활성화해야 한다는 뜻이 아닙니다. Slack에 연결한 동일한 계정으로 [claude.ai/code](https://claude.ai/code)에 한 번 로그인합니다. 첫 방문 시 기본 클라우드 환경이 생성되며, 다음 언급 시 오류가 해결됩니다. 각 사용자는 개별적으로 이 작업을 수행해야 합니다.

<h3 id="sessions-not-starting">
  세션이 시작되지 않음
</h3>

1. Claude 앱 홈에서 Claude 계정이 연결되어 있는지 확인합니다
2. 웹의 Claude Code 액세스가 활성화되어 있는지 확인합니다
3. Claude Code에 연결된 GitHub 저장소가 최소 하나 있는지 확인합니다

<h3 id="repository-not-showing">
  저장소가 표시되지 않음
</h3>

1. [claude.ai/code](https://claude.ai/code)에서 웹의 Claude Code에 저장소를 연결합니다
2. 해당 저장소에 대한 GitHub 권한을 확인합니다
3. GitHub 계정을 연결 해제했다가 다시 연결해봅니다

<h3 id="wrong-repository-selected">
  잘못된 저장소 선택됨
</h3>

1. "저장소 변경" 버튼을 클릭하여 다른 저장소를 선택합니다
2. 더 정확한 선택을 위해 요청에 저장소 이름을 포함합니다

<h3 id="authentication-errors">
  인증 오류
</h3>

1. 앱 홈에서 Claude 계정을 연결 해제했다가 다시 연결합니다
2. 브라우저에서 올바른 Claude 계정으로 로그인했는지 확인합니다
3. Claude 플랜에 Claude Code 액세스가 포함되어 있는지 확인합니다

<h3 id="session-expiration">
  세션 만료
</h3>

1. 세션은 웹의 Claude Code 기록에서 액세스할 수 있습니다
2. [claude.ai/code](https://claude.ai/code)에서 과거 세션을 계속하거나 참조할 수 있습니다

<h2 id="current-limitations">
  현재 제한 사항
</h2>

* **GitHub만**: 현재 GitHub의 저장소만 지원합니다.
* **한 번에 하나의 PR**: 각 세션은 하나의 풀 요청을 생성할 수 있습니다.
* **속도 제한 적용**: 세션은 개별 Claude 플랜의 속도 제한을 사용합니다.
* **웹 액세스 필요**: 사용자는 웹의 Claude Code 액세스가 있어야 합니다. 없으면 표준 Claude 채팅 응답만 받습니다.

<h2 id="related-resources">
  관련 리소스
</h2>

<CardGroup>
  <Card title="웹의 Claude Code" icon="globe" href="/docs/ko/claude-code-on-the-web">
    웹의 Claude Code에 대해 자세히 알아보기
  </Card>

  <Card title="Slack용 Claude" icon="slack" href="https://claude.com/claude-and-slack">
    일반 Slack용 Claude 문서
  </Card>

  <Card title="Claude Tag" icon="users" href="https://claude.com/docs/claude-tag/overview">
    관리자가 구성한 액세스 권한이 있는 Slack의 조직 관리 @Claude
  </Card>

  <Card title="Slack 앱 마켓플레이스" icon="store" href="https://slack.com/marketplace/A08SF47R6P4">
    Slack 마켓플레이스에서 Claude 앱 설치
  </Card>

  <Card title="Claude 도움말 센터" icon="circle-question" href="https://support.claude.com">
    추가 지원 받기
  </Card>
</CardGroup>
