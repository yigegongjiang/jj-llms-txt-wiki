> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 루틴으로 작업 자동화하기

> Claude Code를 자동 조종 장치에 올려놓으세요. Anthropic 관리 클라우드 인프라에서 일정에 따라 실행되거나 API 호출로 트리거되거나 GitHub 이벤트에 반응하는 루틴을 정의하세요.

<Note>
  루틴은 연구 미리보기 상태입니다. 동작, 제한 사항 및 API 표면이 변경될 수 있습니다.
</Note>

루틴은 저장된 Claude Code 구성입니다. 프롬프트, 하나 이상의 저장소, 그리고 [커넥터](/docs/ko/mcp) 세트를 한 번에 패키징하여 자동으로 실행합니다. 루틴은 Anthropic 관리 클라우드 인프라에서 실행되므로 노트북을 닫아도 계속 작동합니다.

각 루틴에는 하나 이상의 트리거를 연결할 수 있습니다.

* **예약됨**: 시간별, 야간, 또는 주간과 같은 반복 주기로 실행되거나 특정 미래 시간에 한 번 실행
* **API**: HTTP POST를 루틴별 엔드포인트로 보내 베어러 토큰으로 요청 시 트리거
* **GitHub**: 풀 요청 또는 릴리스와 같은 저장소 이벤트에 자동으로 반응하여 실행

단일 루틴은 트리거를 결합할 수 있습니다. 예를 들어 PR 검토 루틴은 야간에 실행되고, 배포 스크립트에서 트리거되며, 모든 새로운 PR에도 반응할 수 있습니다.

루틴은 [웹에서 Claude Code](/docs/ko/claude-code-on-the-web)가 활성화된 Pro, Max, Team 및 Enterprise 플랜에서 사용할 수 있습니다. [claude.ai/code/routines](https://claude.ai/code/routines)에서 생성 및 관리하거나 CLI에서 `/schedule`로 관리하세요.

Team 및 Enterprise 관리자는 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)의 루틴 토글로 모든 구성원에 대해 루틴을 비활성화할 수 있습니다. 비활성화되면 기존 루틴이 실행을 중지하고 구성원은 새로운 루틴을 생성할 수 없습니다.

이 페이지에서는 루틴 생성, 각 트리거 유형 구성, 실행 관리 및 사용 제한 적용 방법을 다룹니다.

<h2 id="example-use-cases">
  사용 사례 예시
</h2>

각 예시는 트리거 유형을 루틴이 적합한 작업 종류와 짝지어줍니다. 무인 상태에서 반복 가능하며 명확한 결과와 연결된 작업입니다.

**백로그 유지 관리.** 일정 트리거가 커넥터를 통해 이슈 추적기에 대해 매주 평일 밤에 실행됩니다. 루틴은 마지막 실행 이후 열린 이슈를 읽고, 레이블을 적용하고, 참조된 코드 영역을 기반으로 소유자를 할당하고, Slack에 요약을 게시하여 팀이 정리된 큐로 하루를 시작할 수 있도록 합니다.

**경고 분류.** 모니터링 도구가 오류 임계값을 초과할 때 루틴의 API 엔드포인트를 호출하고 경고 본문을 `text`로 전달합니다. 루틴은 스택 추적을 가져오고, 저장소의 최근 커밋과 상관관계를 지으며, 제안된 수정 사항과 경고로 돌아가는 링크가 있는 초안 풀 요청을 엽니다. 온콜 담당자는 빈 터미널에서 시작하는 대신 PR을 검토합니다.

**맞춤형 코드 검토.** GitHub 트리거가 `pull_request.opened`에서 실행됩니다. 루틴은 팀의 자체 검토 체크리스트를 적용하고, 보안, 성능 및 스타일 문제에 대해 인라인 댓글을 남기고, 요약 댓글을 추가하여 인간 검토자가 기계적 검사 대신 설계에 집중할 수 있도록 합니다.

**배포 검증.** CD 파이프라인이 각 프로덕션 배포 후 루틴의 API 엔드포인트를 호출합니다. 루틴은 새 빌드에 대해 스모크 테스트를 실행하고, 오류 로그에서 회귀를 스캔하고, 배포 윈도우가 닫히기 전에 릴리스 채널에 진행 또는 진행 불가를 게시합니다.

**문서 드리프트.** 일정 트리거가 매주 실행됩니다. 루틴은 마지막 실행 이후 병합된 PR을 스캔하고, 변경된 API를 참조하는 문서에 플래그를 지정하고, 편집자가 검토할 수 있도록 문서 저장소에 대해 업데이트 PR을 엽니다.

**라이브러리 포트.** GitHub 트리거가 한 SDK 저장소의 병합된 PR로 필터링된 `pull_request.closed`에서 실행됩니다. 루틴은 변경 사항을 다른 언어의 병렬 SDK로 포트하고 일치하는 PR을 열어 두 라이브러리를 동기화 상태로 유지하며 인간이 각 변경 사항을 다시 구현할 필요가 없습니다.

아래 섹션에서는 루틴을 생성하고 이러한 각 트리거 유형을 구성하는 방법을 설명합니다.

<h2 id="create-a-routine">
  루틴 생성
</h2>

웹의 [claude.ai/code/routines](https://claude.ai/code/routines), 데스크톱 앱 또는 CLI에서 루틴을 생성합니다. 세 가지 표면 모두 동일한 클라우드 계정에 쓰므로 한 곳에서 생성한 루틴이 즉시 다른 곳에 표시됩니다. 데스크톱 앱에서 사이드바의 **루틴**을 클릭한 다음 **새 루틴**을 클릭하고 **원격**을 선택합니다. 대신 **로컬**을 선택하면 머신에서 실행되는 [데스크톱 예약 작업](/docs/ko/desktop-scheduled-tasks)이 생성되며, 클라우드에서 실행되지 않습니다.

생성 양식은 루틴의 프롬프트, 저장소, 환경, 커넥터 및 트리거를 설정합니다.

루틴은 완전한 Claude Code 클라우드 세션으로 자율적으로 실행됩니다. 권한 모드 선택기나 실행 중 승인 프롬프트가 없습니다. 세션은 셸 명령을 실행하고, 복제된 저장소에 커밋된 [스킬](/docs/ko/skills)을 사용하고, 포함된 모든 커넥터를 호출할 수 있습니다. 루틴이 도달할 수 있는 것은 선택한 저장소와 해당 브랜치 푸시 설정, [환경의](/docs/ko/claude-code-on-the-web#the-cloud-environment) 네트워크 액세스 및 변수, 그리고 포함된 커넥터에 의해 결정됩니다. 루틴이 실제로 필요한 것으로 각각을 범위 지정합니다.

루틴은 개별 claude.ai 계정에 속합니다. 팀원과 공유되지 않으며 계정의 일일 실행 허용량에 대해 계산됩니다. 루틴이 연결된 GitHub 신원 또는 커넥터를 통해 수행하는 모든 작업은 사용자로 표시됩니다. 커밋 및 풀 요청은 GitHub 사용자를 전달하고, Slack 메시지, Linear 티켓 또는 기타 커넥터 작업은 해당 서비스에 대해 연결된 계정을 사용합니다.

<h3 id="create-from-the-web">
  웹에서 생성
</h3>

<Steps>
  <Step title="생성 양식 열기">
    [claude.ai/code/routines](https://claude.ai/code/routines)를 방문하고 **새 루틴**을 클릭합니다.
  </Step>

  <Step title="루틴 이름 지정 및 프롬프트 작성">
    루틴에 설명적인 이름을 지정하고 Claude가 매번 실행할 프롬프트를 작성합니다. 프롬프트가 가장 중요한 부분입니다. 루틴이 자율적으로 실행되므로 프롬프트는 자체 포함되어야 하며 수행할 작업과 성공이 무엇인지에 대해 명시적이어야 합니다.

    프롬프트 입력에는 모델 선택기가 포함됩니다. Claude는 모든 실행에서 선택된 모델을 사용합니다.
  </Step>

  <Step title="저장소 선택">
    Claude가 작업할 하나 이상의 GitHub 저장소를 추가합니다. 각 저장소는 실행 시작 시 기본 브랜치에서 시작하여 복제됩니다. Claude는 변경 사항에 대해 `claude/` 접두사가 붙은 브랜치를 생성합니다.
  </Step>

  <Step title="환경 선택">
    루틴에 대해 [클라우드 환경](/docs/ko/claude-code-on-the-web#the-cloud-environment)을 선택합니다. 환경은 클라우드 세션이 액세스할 수 있는 것을 제어합니다.

    * **네트워크 액세스**: 각 실행 중에 사용 가능한 인터넷 액세스 수준 설정
    * **환경 변수**: Claude가 사용할 수 있는 API 키, 토큰 또는 기타 비밀 제공
    * **설정 스크립트**: 루틴이 필요한 종속성 및 도구를 설치합니다. 결과는 [캐시됩니다](/docs/ko/claude-code-on-the-web#environment-caching). 따라서 스크립트는 모든 세션에서 다시 실행되지 않습니다.

    **기본** 환경이 제공되며 **신뢰할 수 있는** 네트워크 액세스가 있습니다. 이는 [기본 설정](/docs/ko/claude-code-on-the-web#default-allowed-domains) 패키지 레지스트리, 클라우드 공급자 API, 컨테이너 레지스트리 및 일반적인 개발 도메인을 허용하지만 다른 모든 것을 차단합니다. 루틴이 자신의 서비스나 해당 목록 외의 도메인에 도달해야 하는 경우 실행하기 전에 환경의 [네트워크 액세스](/docs/ko/claude-code-on-the-web#network-access)를 편집합니다. 별도의 환경을 사용하려면 먼저 [하나를 생성](/docs/ko/claude-code-on-the-web#configure-your-environment)합니다.
  </Step>

  <Step title="트리거 선택">
    **트리거 선택** 아래에서 루틴이 시작되는 방식을 선택합니다. 하나의 트리거 유형을 선택하거나 여러 개를 결합할 수 있습니다.

    <Tabs>
      <Tab title="일정">
        반복 실행을 위해 사전 설정된 빈도를 선택하거나 특정 타임스탬프에서 일회성 실행을 예약합니다. 시간대 처리, 엇갈림, 사용자 정의 cron 간격 및 일회성 실행은 [일정 트리거 추가](#add-a-schedule-trigger)를 참조합니다.
      </Tab>

      <Tab title="GitHub 이벤트">
        저장소, 반응할 이벤트 및 선택적 필터를 선택합니다. 지원되는 이벤트 및 필터 필드의 전체 목록은 [GitHub 트리거 추가](#add-a-github-trigger)를 참조합니다.
      </Tab>

      <Tab title="API">
        여기서 **API**를 선택한 다음 루틴을 저장합니다. URL과 토큰은 루틴 ID에 따라 달라지므로 루틴이 저장된 후 생성됩니다. URL을 복사하고 토큰을 생성하려면 [API 트리거 추가](#add-an-api-trigger)를 참조합니다.
      </Tab>
    </Tabs>
  </Step>

  <Step title="커넥터 및 권한 검토">
    양식 하단의 **커넥터** 및 **권한** 탭은 루틴이 도달할 수 있는 것을 제어합니다.

    커넥터 아래에서 연결된 모든 [MCP 커넥터](/docs/ko/mcp)는 기본적으로 포함됩니다. 루틴이 필요하지 않은 것을 제거합니다. Claude는 실행 중에 권한을 요청하지 않고 포함된 커넥터의 모든 도구(쓰기 포함)를 사용할 수 있습니다.

    권한 아래에서 Claude가 `claude/` 접두사가 붙은 브랜치만 푸시하는 대신 기존 브랜치로 푸시할 수 있어야 하는 모든 저장소에 대해 **제한 없는 브랜치 푸시 허용**을 활성화합니다.
  </Step>

  <Step title="루틴 생성">
    **생성**을 클릭합니다. 루틴이 목록에 나타나고 다음 번에 트리거 중 하나가 일치할 때 실행됩니다. 즉시 실행을 시작하려면 루틴의 세부 정보 페이지에서 **지금 실행**을 클릭합니다.

    각 실행은 다른 세션과 함께 새 세션을 생성하므로 Claude가 수행한 작업을 확인하고, 변경 사항을 검토하고, 풀 요청을 생성할 수 있습니다.
  </Step>
</Steps>

<h3 id="create-from-the-cli">
  CLI에서 생성
</h3>

모든 세션에서 `/schedule`을 실행하여 예약된 루틴을 대화식으로 생성합니다. `/schedule daily PR review at 9am`과 같은 반복 루틴이나 `/schedule clean up feature flag in one week`과 같은 일회성 루틴에 대해 설명을 직접 전달할 수도 있습니다. Claude는 웹 양식이 수집하는 동일한 정보를 안내한 다음 루틴을 계정에 저장합니다.

성공적인 시작은 대화처럼 보입니다. Claude는 저장하기 전에 일정, 저장소 및 프롬프트에 대한 후속 질문을 합니다. Claude가 대신 인증이 필요하거나 원격 claude.ai 계정에 연결할 수 없다고 회신하면 루틴이 생성되지 않았습니다. [문제 해결](#troubleshooting)을 참조합니다.

CLI의 `/schedule`은 예약된 루틴만 생성합니다. API 또는 GitHub 트리거를 추가하려면 [claude.ai/code/routines](https://claude.ai/code/routines)의 웹에서 루틴을 편집합니다.

CLI는 기존 루틴 관리도 지원합니다. `/schedule list`를 실행하여 모든 루틴을 보거나, `/schedule update`를 실행하여 하나를 변경하거나, `/schedule run`을 실행하여 즉시 트리거합니다.

<h2 id="configure-triggers">
  트리거 구성
</h2>

루틴은 트리거 중 하나가 일치할 때 시작됩니다. 동일한 루틴에 일정, API 및 GitHub 트리거의 모든 조합을 연결할 수 있으며, 루틴의 편집 양식의 **트리거 선택** 섹션에서 언제든지 추가하거나 제거할 수 있습니다.

<h3 id="add-a-schedule-trigger">
  일정 트리거 추가
</h3>

일정 트리거는 반복 주기에 따라 루틴을 실행하거나 특정 미래 시간에 한 번 실행합니다. **트리거 선택** 섹션에서 사전 설정된 빈도를 선택하세요: 시간별, 일일, 평일 또는 주간. 시간은 로컬 시간대에 입력되고 자동으로 변환되므로 클라우드 인프라가 어디에 있든 루틴이 해당 벽시계 시간에 실행됩니다.

실행은 엇갈림으로 인해 예약된 시간 몇 분 후에 시작될 수 있습니다. 오프셋은 각 루틴에 대해 일관됩니다.

2시간마다 또는 매월 1일과 같은 사용자 정의 간격의 경우 양식에서 가장 가까운 사전 설정을 선택한 다음 CLI에서 `/schedule update`를 실행하여 특정 cron 표현식을 설정하세요. 최소 간격은 1시간입니다. 더 자주 실행되는 표현식은 거부됩니다.

<h4 id="schedule-a-one-off-run">
  일회성 실행 예약
</h4>

일회성 일정은 특정 타임스탬프에서 루틴을 한 번만 실행합니다. 이를 사용하여 주 후반에 자신에게 상기시키거나, 롤아웃이 완료된 후 정리 PR을 열거나, 업스트림 변경이 도착할 때 후속 작업을 시작하세요. 루틴이 실행된 후 자동으로 비활성화되고 웹 UI는 이를 **실행됨**으로 표시합니다. 다시 실행하려면 루틴을 편집하고 새로운 일회성 시간을 설정하세요.

<Note>
  CLI에서의 일회성 스케줄링은 점진적으로 출시 중이며 아직 계정에서 사용할 수 없을 수 있습니다. `/schedule`이 반복 일정만 제공하는 경우 [claude.ai/code/routines](https://claude.ai/code/routines)의 웹에서 일회성 실행을 만드세요.
</Note>

CLI에서 자연어로 시간을 설명하여 일회성 실행을 만드세요. Claude는 현재 시간에 대해 구문을 해석하고 저장하기 전에 절대 타임스탬프를 확인합니다.

```text theme={null}
/schedule tomorrow at 9am, summarize yesterday's merged PRs
```

```text theme={null}
/schedule in 2 weeks, open a cleanup PR that removes the feature flag
```

반복 일정과 동일한 로컬-UTC 변환이 일회성 타임스탬프에 적용됩니다.

일회성 실행은 일일 루틴 실행 상한선에 포함되지 않습니다. 다른 세션과 마찬가지로 플랜의 정기 구독 사용량을 소비합니다. 자세한 내용은 [사용량 및 제한](#usage-and-limits)을 참조하세요.

<h3 id="add-an-api-trigger">
  API 트리거 추가
</h3>

API 트리거는 루틴에 전용 HTTP 엔드포인트를 제공합니다. 루틴의 베어러 토큰으로 엔드포인트에 POST하면 새 세션이 시작되고 세션 URL이 반환됩니다. 이를 사용하여 Claude Code를 경고 시스템, 배포 파이프라인, 내부 도구 또는 인증된 HTTP 요청을 할 수 있는 곳에 연결하세요.

API 트리거는 웹에서 기존 루틴에 추가됩니다. CLI는 현재 토큰을 생성하거나 취소할 수 없습니다.

<Steps>
  <Step title="편집을 위해 루틴 열기">
    [claude.ai/code/routines](https://claude.ai/code/routines)로 이동하고, API를 통해 트리거하려는 루틴을 클릭한 다음, 연필 아이콘을 클릭하여 **루틴 편집**을 엽니다.
  </Step>

  <Step title="API 트리거 추가">
    **지침** 상자 아래의 **트리거 선택** 섹션으로 스크롤하고, **다른 트리거 추가**를 클릭한 다음, **API**를 선택하세요.
  </Step>

  <Step title="URL 복사 및 토큰 생성">
    모달은 이 루틴의 URL과 샘플 curl 명령을 표시합니다. URL을 복사한 다음 **토큰 생성**을 클릭하고 토큰을 즉시 복사하세요. 토큰은 한 번만 표시되며 나중에 검색할 수 없으므로 경고 도구의 비밀 저장소와 같은 안전한 곳에 저장하세요.
  </Step>

  <Step title="엔드포인트 호출">
    URL에 POST할 때 `Authorization: Bearer` 헤더에 토큰을 보내세요. 아래의 [루틴 트리거](#trigger-a-routine) 섹션에서 완전한 예시를 보여줍니다.
  </Step>
</Steps>

각 루틴에는 자체 토큰이 있으며, 해당 루틴 트리거로만 범위가 지정됩니다. 회전하거나 취소하려면 동일한 모달로 돌아가 **재생성** 또는 **취소**를 클릭하세요.

<h4 id="trigger-a-routine">
  루틴 트리거
</h4>

`/fire` 엔드포인트에 `Authorization` 헤더의 베어러 토큰으로 POST 요청을 보내세요. 요청 본문은 경고 본문 또는 실패한 로그와 같은 실행별 컨텍스트에 대한 선택적 `text` 필드를 수락하며, 저장된 프롬프트와 함께 루틴에 전달됩니다. 값은 자유 형식 텍스트이며 구문 분석되지 않습니다. JSON 또는 다른 구조화된 페이로드를 보내면 루틴은 이를 리터럴 문자열로 받습니다.

아래 예시는 셸에서 루틴을 트리거합니다. 표시된 루틴 ID와 토큰은 자리 표시자입니다. [API 트리거 추가](#add-an-api-trigger) 시 복사한 URL과 토큰으로 바꾸거나, 요청이 `401` 인증 오류로 실패합니다.

```bash theme={null}
curl -X POST https://api.anthropic.com/v1/claude_code/routines/trig_01ABCDEFGHJKLMNOPQRSTUVW/fire \
  -H "Authorization: Bearer sk-ant-oat01-xxxxx" \
  -H "anthropic-beta: experimental-cc-routine-2026-04-01" \
  -H "anthropic-version: 2023-06-01" \
  -H "Content-Type: application/json" \
  -d '{"text": "Sentry alert SEN-4521 fired in prod. Stack trace attached."}'
```

성공한 요청은 새 세션 ID와 URL이 있는 JSON 본문을 반환합니다.

```json theme={null}
{
  "type": "routine_fire",
  "claude_code_session_id": "session_01HJKLMNOPQRSTUVWXYZ",
  "claude_code_session_url": "https://claude.ai/code/session_01HJKLMNOPQRSTUVWXYZ"
}
```

브라우저에서 세션 URL을 열어 실시간으로 실행을 보거나, 변경 사항을 검토하거나, 대화를 수동으로 계속하세요.

<Warning>
  `/fire` 엔드포인트는 `experimental-cc-routine-2026-04-01` 베타 헤더 아래에서 제공됩니다. 요청 및 응답 형태, 속도 제한 및 토큰 의미론은 기능이 연구 미리보기 상태인 동안 변경될 수 있습니다. 주요 변경 사항은 새로운 날짜 지정 베타 헤더 버전 뒤에 제공되며, 가장 최근의 이전 헤더 버전 두 개는 계속 작동하므로 호출자가 마이그레이션할 시간이 있습니다.
</Warning>

<h4 id="api-reference">
  API 참조
</h4>

모든 오류 응답, 검증 규칙 및 필드 제한을 포함한 전체 API 참조는 Claude 플랫폼 설명서의 [API를 통해 루틴 트리거](https://platform.claude.com/docs/ko/api/claude-code/routines-fire)를 참조하세요.

`/fire` 엔드포인트는 claude.ai 사용자만 사용할 수 있으며 Claude 플랫폼 API 표면의 일부가 아닙니다.

<h3 id="add-a-github-trigger">
  GitHub 트리거 추가
</h3>

GitHub 트리거는 연결된 저장소에서 일치하는 이벤트가 발생할 때 자동으로 새 세션을 시작합니다. 각 일치하는 이벤트는 자체 세션을 시작합니다.

<Note>
  연구 미리보기 중에 GitHub 웹훅 이벤트는 루틴별 및 계정별 시간당 상한선이 있습니다. 제한을 초과하는 이벤트는 윈도우가 재설정될 때까지 삭제됩니다. [claude.ai/code/routines](https://claude.ai/code/routines)에서 현재 제한을 확인하세요.
</Note>

GitHub 트리거는 웹 UI에서만 구성됩니다.

<Steps>
  <Step title="편집을 위해 루틴 열기">
    [claude.ai/code/routines](https://claude.ai/code/routines)로 이동하고, 루틴을 클릭한 다음, 연필 아이콘을 클릭하여 **루틴 편집**을 엽니다.
  </Step>

  <Step title="GitHub 이벤트 트리거 추가">
    **트리거 선택** 섹션으로 스크롤하고, **다른 트리거 추가**를 클릭한 다음, **GitHub 이벤트**를 선택하세요.
  </Step>

  <Step title="Claude GitHub 앱 설치">
    Claude GitHub 앱을 구독하려는 저장소에 설치해야 합니다. 트리거 설정은 아직 설치되지 않은 경우 설치하도록 요청합니다.

    <Note>
      CLI에서 `/web-setup`을 실행하면 복제를 위한 저장소 액세스 권한이 부여되지만 Claude GitHub 앱을 설치하지 않으며 웹훅 전달을 활성화하지 않습니다. GitHub 트리거는 Claude GitHub 앱을 설치해야 하며, 트리거 설정이 이를 수행하도록 요청합니다.
    </Note>
  </Step>

  <Step title="트리거 구성">
    저장소를 선택하고, [지원되는 이벤트](#supported-events) 목록에서 이벤트를 선택하고, 선택적으로 필터를 추가하세요. 트리거를 저장하세요.
  </Step>
</Steps>

<h4 id="supported-events">
  지원되는 이벤트
</h4>

GitHub 트리거는 다음 이벤트 범주 중 하나를 구독할 수 있습니다. 각 범주 내에서 `pull_request.opened`와 같은 특정 작업을 선택하거나 범주의 모든 작업에 반응할 수 있습니다.

| 이벤트  | 트리거 시점                                                   |
| :--- | :------------------------------------------------------- |
| 풀 요청 | PR이 열리거나, 닫히거나, 할당되거나, 레이블이 지정되거나, 동기화되거나, 기타 방식으로 업데이트됨 |
| 릴리스  | 릴리스가 생성되거나, 게시되거나, 편집되거나, 삭제됨                            |

<h4 id="filter-pull-requests">
  풀 요청 필터링
</h4>

필터를 사용하여 새 세션을 시작하는 풀 요청을 좁히세요. 루틴이 트리거되려면 모든 필터 조건이 일치해야 합니다. 사용 가능한 필터 필드는 다음과 같습니다.

| 필터     | 일치                  |
| :----- | :------------------ |
| 작성자    | PR 작성자의 GitHub 사용자명 |
| 제목     | PR 제목 텍스트           |
| 본문     | PR 설명 텍스트           |
| 기본 브랜치 | PR이 대상으로 하는 브랜치     |
| 헤드 브랜치 | PR이 나오는 브랜치         |
| 레이블    | PR에 적용된 레이블         |
| 초안 여부  | PR이 초안 상태인지 여부      |
| 병합 여부  | PR이 병합되었는지 여부       |

각 필터는 필드를 연산자와 쌍으로 지정합니다: 같음, 포함, 시작, 하나, 하나 아님 또는 정규식 일치.

`matches regex` 연산자는 전체 필드 값을 테스트하며, 그 내의 부분 문자열이 아닙니다. `hotfix`를 포함하는 제목과 일치하려면 `.*hotfix.*`를 작성하세요. 주변 `.*` 없이는 필터가 정확히 `hotfix`인 제목과만 일치하며 앞이나 뒤에 아무것도 없습니다. 정규식 구문 없이 리터럴 부분 문자열 일치의 경우 대신 `contains` 연산자를 사용하세요.

몇 가지 예시 필터 조합:

* **인증 모듈 검토**: 기본 브랜치 `main`, 헤드 브랜치 포함 `auth-provider`. 인증을 건드리는 모든 PR을 집중된 검토자에게 보냅니다.
* **검토 준비 완료만**: 초안 여부 `false`. 초안을 건너뛰므로 루틴은 PR이 검토 준비가 되었을 때만 실행됩니다.
* **레이블 게이트 백포트**: 레이블 포함 `needs-backport`. 유지 관리자가 PR에 태그를 지정할 때만 다른 브랜치로의 포트 루틴을 트리거합니다.

<h4 id="how-sessions-map-to-events">
  세션이 이벤트에 매핑되는 방식
</h4>

각 일치하는 GitHub 이벤트는 새 세션을 시작합니다. GitHub 트리거 루틴의 경우 이벤트 간 세션 재사용을 사용할 수 없으므로 두 PR 업데이트는 두 개의 독립적인 세션을 생성합니다.

<h2 id="manage-routines">
  루틴 관리
</h2>

목록에서 루틴을 클릭하여 세부 정보 페이지를 엽니다. 세부 정보 페이지는 루틴의 저장소, 커넥터, 프롬프트, 일정, API 토큰, GitHub 트리거 및 과거 실행 목록을 표시합니다.

<h3 id="view-and-interact-with-runs">
  실행 보기 및 상호 작용
</h3>

모든 실행을 클릭하여 완전한 세션으로 엽니다. 거기서 Claude가 수행한 작업을 확인하고, 변경 사항을 검토하고, 풀 요청을 생성하거나, 대화를 계속할 수 있습니다. 각 실행 세션은 다른 세션처럼 작동합니다. 세션 제목 옆의 드롭다운 메뉴를 사용하여 이름을 바꾸거나, 보관하거나, 삭제하세요.

<Note>
  실행 목록의 녹색 상태는 세션이 시작되어 인프라 오류 없이 종료되었음을 의미합니다. 프롬프트의 작업이 성공했음을 의미하지는 않습니다. 실행을 열어 기록을 읽고 Claude가 실제로 수행한 작업을 확인하세요. 차단된 네트워크 요청, 누락된 커넥터 도구 및 작업 수준 오류는 모두 상태 표시기가 아닌 여기에 표시됩니다.
</Note>

<h3 id="edit-and-control-routines">
  루틴 편집 및 제어
</h3>

루틴 세부 정보 페이지에서 다음을 수행할 수 있습니다.

* **지금 실행**을 클릭하여 다음 예약된 시간을 기다리지 않고 즉시 실행을 시작하세요.
* **반복** 섹션의 토글을 사용하여 일정을 일시 중지하거나 재개하세요. 일시 중지된 루틴은 구성을 유지하지만 다시 활성화할 때까지 실행되지 않습니다.
* 연필 아이콘을 클릭하여 **루틴 편집**을 열고 이름, 프롬프트, 저장소, 환경, 커넥터 또는 루틴의 트리거를 변경하세요. **트리거 선택** 섹션은 일정, API 토큰 및 GitHub 이벤트 트리거를 추가하거나 제거하는 곳입니다.
* 삭제 아이콘을 클릭하여 루틴을 제거하세요. 루틴에서 생성한 과거 세션은 세션 목록에 남아 있습니다.

<h3 id="repositories-and-branch-permissions">
  저장소 및 브랜치 권한
</h3>

루틴은 저장소를 복제하기 위해 GitHub 액세스가 필요합니다. CLI에서 `/schedule`로 루틴을 생성할 때 Claude는 계정에 GitHub이 연결되어 있는지 확인하고 연결되지 않은 경우 `/web-setup`을 실행하도록 요청합니다. [GitHub 인증 옵션](/docs/ko/claude-code-on-the-web#github-authentication-options)을 참조하여 액세스 권한을 부여하는 두 가지 방법을 확인하세요.

추가하는 각 저장소는 모든 실행에서 복제됩니다. Claude는 프롬프트에서 달리 지정하지 않는 한 저장소의 기본 브랜치에서 시작합니다.

기본적으로 Claude는 `claude/` 접두사가 붙은 브랜치로만 푸시할 수 있습니다. 이는 루틴이 실수로 보호되거나 장기 브랜치를 수정하는 것을 방지합니다. 특정 저장소에 대해 이 제한을 제거하려면 루틴을 생성하거나 편집할 때 해당 저장소에 대해 **제한 없는 브랜치 푸시 허용**을 활성화하세요.

<h3 id="connectors">
  커넥터
</h3>

루틴은 연결된 MCP 커넥터를 사용하여 각 실행 중에 외부 서비스에서 읽고 쓸 수 있습니다. 예를 들어 지원 요청을 분류하는 루틴은 Slack 채널에서 읽고 Linear에서 이슈를 생성할 수 있습니다.

커넥터는 계정의 [claude.ai 통합](/docs/ko/mcp#use-mcp-servers-from-claude-ai)입니다. CLI에서 `claude mcp add`로 로컬에 추가한 MCP 서버는 claude.ai 계정이 아닌 컴퓨터에 저장되므로 커넥터 목록에 나타나지 않습니다. 루틴에서 이러한 서버 중 하나를 사용하려면 [claude.ai/customize/connectors](https://claude.ai/customize/connectors)에서 커넥터로 추가하거나 커밋된 [`.mcp.json`](/docs/ko/mcp#project-scope)에서 선언하여 복제된 저장소의 일부가 되도록 하세요.

루틴을 생성할 때 현재 연결된 모든 커넥터가 기본적으로 포함됩니다. 실행 중에 Claude가 액세스할 수 있는 도구를 제한하려면 필요하지 않은 것을 제거하세요. 루틴 양식에서 직접 커넥터를 추가할 수도 있습니다.

루틴 양식 외부에서 커넥터를 관리하거나 추가하려면 claude.ai의 **설정 > 커넥터**를 방문하거나 CLI에서 `/schedule update`를 사용하세요.

<h3 id="environments-and-network-access">
  환경 및 네트워크 액세스
</h3>

각 루틴은 네트워크 액세스, 환경 변수 및 설정 스크립트를 제어하는 [클라우드 환경](/docs/ko/claude-code-on-the-web#the-cloud-environment)에서 실행됩니다. 루틴은 모든 실행에서 환경의 네트워크 정책을 상속합니다.

**기본** 환경은 **신뢰할 수 있는** 네트워크 액세스를 사용합니다. [기본 허용 목록](/docs/ko/claude-code-on-the-web#default-allowed-domains)의 패키지 레지스트리, 클라우드 공급자 API, 컨테이너 레지스트리 및 일반적인 개발 도메인에 도달할 수 있지만 임의의 도메인에는 도달할 수 없습니다. 다른 호스트로의 아웃바운드 요청은 `403` 및 `x-deny-reason: host_not_allowed`로 실패합니다. MCP 커넥터 트래픽은 Anthropic의 서버를 통해 라우팅되므로 루틴에 추가하는 커넥터는 **허용된 도메인**에 호스트를 추가하지 않고도 작동합니다. [커넥터](#connectors) 아래에서 필요하지 않은 커넥터를 제거하세요.

추가 도메인을 허용하려면:

<Steps>
  <Step title="루틴을 편집하기 위해 열기">
    루틴의 세부 정보 페이지에서 연필 아이콘을 클릭하여 **루틴 편집**을 엽니다.
  </Step>

  <Step title="환경 선택기 열기">
    **지침** 상자 아래에서 **기본**과 같은 환경의 이름을 표시하는 클라우드 아이콘을 선택합니다.
  </Step>

  <Step title="환경 설정 열기">
    목록의 환경 위에 마우스를 올리고 오른쪽에 나타나는 설정 아이콘을 클릭합니다.
  </Step>

  <Step title="네트워크 액세스 수준 변경">
    **클라우드 환경 업데이트** 대화 상자에서 **네트워크 액세스**를 **사용자 정의**로 변경하고 **허용된 도메인**에 도메인을 입력합니다. **기본 패키지 관리자 목록도 포함**을 확인하여 [기본 허용 목록](/docs/ko/claude-code-on-the-web#default-allowed-domains)을 사용자 정의 도메인과 함께 유지합니다. 제한 없는 액세스를 위해 **전체**를 대신 선택합니다.
  </Step>

  <Step title="저장">
    **변경 사항 저장**을 클릭합니다. 새 정책은 다음 실행부터 적용됩니다.
  </Step>
</Steps>

액세스 수준 및 기본 허용 목록에 대한 자세한 내용은 [네트워크 액세스](/docs/ko/claude-code-on-the-web#network-access)를 참조하세요.

<h2 id="usage-and-limits">
  사용 및 제한
</h2>

루틴은 대화형 세션과 동일한 방식으로 구독 사용을 소비합니다. 표준 구독 제한 외에도 루틴은 계정당 시작할 수 있는 실행 수에 대한 일일 상한선이 있습니다. [claude.ai/code/routines](https://claude.ai/code/routines) 또는 [claude.ai/settings/usage](https://claude.ai/settings/usage)에서 현재 소비 및 남은 일일 루틴 실행을 확인하세요.

루틴이 일일 상한선 또는 구독 사용 제한에 도달할 때 추가 사용이 활성화된 조직은 계량된 초과 요금으로 루틴을 계속 실행할 수 있습니다. 추가 사용이 없으면 윈도우가 재설정될 때까지 추가 실행이 거부됩니다. claude.ai의 **설정 > 청구**에서 추가 사용을 활성화하세요.

일회성 실행은 일일 루틴 실행 상한선에 포함되지 않습니다. 다른 세션과 마찬가지로 정기 구독 사용을 소비하지만 계정당 일일 루틴 실행 허용량에서 제외됩니다.

<h2 id="troubleshooting">
  문제 해결
</h2>

<h3 id="/schedule-returns-unknown-command">
  `/schedule` "알 수 없는 명령" 반환
</h3>

CLI는 요구 사항 중 하나가 충족되지 않으면 `/schedule`을 숨깁니다. 입력 중에 명령 메뉴는 `"/schedule"과 일치하는 명령이 없습니다`를 표시하고, 제출하면 `알 수 없는 명령: /schedule`을 반환합니다. 원인은 일반적으로 다음 중 하나입니다.

* Console API 키 또는 Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry와 같은 클라우드 공급자로 인증되어 있습니다. `/schedule`은 claude.ai 구독 로그인이 필요합니다. 셸에서 `ANTHROPIC_API_KEY` 또는 `ANTHROPIC_AUTH_TOKEN`이 설정되어 있거나 `settings.json`에서 `apiKeyHelper`가 설정되어 있으면 먼저 제거하세요. 이들이 claude.ai 로그인보다 우선하기 때문입니다.
* `DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` 또는 `DISABLE_GROWTHBOOK`이 셸 환경 또는 [`settings.json` 파일](/docs/ko/settings#available-settings)의 `env` 블록에 설정되어 있습니다. 이들은 `/schedule`이 의존하는 기능 플래그 가져오기를 비활성화합니다.
* Claude Code 웹 세션 내부에 있습니다. 대신 [웹 UI](https://claude.ai/code/routines)에서 루틴을 관리하세요.

CLI가 어떻게 구성되어 있든 관계없이 [claude.ai/code/routines](https://claude.ai/code/routines)에서 언제든지 루틴을 생성하고 관리할 수 있습니다.

<h3 id="/schedule-asks-you-to-authenticate">
  `/schedule`이 인증을 요청합니다.
</h3>

`/schedule`이 실행되지만 Claude가 먼저 claude.ai 계정으로 인증해야 한다고 응답하면, CLI에 저장된 claude.ai 로그인이 없습니다. API 계정은 루틴에 지원되지 않습니다. `/login`을 실행하고, claude.ai 계정으로 로그인한 다음, `/schedule`을 다시 실행하세요.

<h3 id="routines-are-disabled-by-your-organization’s-policy">
  "루틴이 조직의 정책에 의해 비활성화되었습니다"
</h3>

Team 또는 Enterprise 조직의 Owner가 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)의 **루틴** 토글을 꺼놨을 가능성이 높습니다. 이는 서버 측 조직 설정이므로 로컬 구성에서 재정의할 수 없습니다. 조직에 대해 루틴을 활성화하도록 Owner에게 요청하세요.

<h2 id="related-resources">
  관련 리소스
</h2>

* [`/loop` 및 세션 내 예약](/docs/ko/scheduled-tasks): 열린 CLI 세션 내에서 로컬 작업 예약
* [데스크톱 예약 작업](/docs/ko/desktop-scheduled-tasks): 로컬 파일에 액세스할 수 있는 머신에서 실행되는 로컬 예약 작업
* [클라우드 환경](/docs/ko/claude-code-on-the-web#the-cloud-environment): 클라우드 세션의 런타임 환경 구성
* [MCP 커넥터](/docs/ko/mcp): Slack, Linear 및 Google Drive와 같은 외부 서비스 연결
* [GitHub Actions](/docs/ko/github-actions): 저장소 이벤트에서 CI 파이프라인에서 Claude 실행
