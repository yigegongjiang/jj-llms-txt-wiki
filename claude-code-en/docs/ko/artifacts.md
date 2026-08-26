> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 아티팩트로 세션 출력 공유

> 아티팩트는 Claude Code의 작업을 claude.ai의 비공개 URL에서 라이브 인터랙티브 페이지로 변환하며, 비공개로 유지하거나 조직과 공유하거나 공개 링크로 게시할 수 있습니다.

<Note>
  아티팩트는 Pro, Max, Team, Enterprise 플랜에서 사용 가능하며 [`/login`](/docs/ko/setup#authenticate)으로 로그인한 세션이 필요합니다. 전체 요구사항은 [가용성](#availability)을 참조하십시오.
</Note>

아티팩트는 Claude Code가 세션에서 claude.ai의 비공개 URL로 게시하는 라이브 인터랙티브 웹 페이지입니다. 브라우저에서 열면 세션이 계속되면서 제자리에서 업데이트됩니다. 페이지 헤더에서 공유하여 다른 사람도 볼 수 있도록 할 수 있습니다. 예를 들어, 아티팩트를 사용하여 주석이 달린 diff로 풀 요청을 검토자에게 설명하거나, 세션 데이터에서 대시보드를 구축하거나, Claude가 작업하면서 채워지는 조사 타임라인을 유지할 수 있습니다.

<Frame>
  <img src="https://mintcdn.com/claude-code/kaHIYYMIYMYPxQg9/images/artifacts-viewer.png?fit=max&auto=format&n=kaHIYYMIYMYPxQg9&q=85&s=dbfd671cdb0d15f49f808b9e89778fe1" alt="claude.ai/code/artifact에서 열린 아티팩트입니다. 뷰어 헤더는 아티팩트 제목 acme-funnel-fix, 공유 버튼, 작성자 아바타를 표시합니다. 공유 메뉴는 최신 버전 항상 공유 토글, 버전 2를 읽는 버전 선택기, Acme 대상 선택기, 링크 복사 버튼과 함께 열려 있습니다. 헤더 아래에서 아티팩트 페이지는 두 개의 모바일 목업을 나란히, 깔때기 차트, 메트릭 카드 행을 표시합니다." width="2511" height="1890" data-path="images/artifacts-viewer.png" />
</Frame>

<h2 id="when-to-use-an-artifact">
  아티팩트를 사용할 시기
</h2>

Claude가 생성한 출력이 터미널 텍스트로는 부적절한 경우 아티팩트를 사용하십시오. 즉, 한 줄씩 읽는 것보다 보고 상호작용하기가 더 쉬운 출력입니다. Claude는 코드베이스와 [연결된 도구](/docs/ko/mcp)를 통해 가져오는 데이터를 포함하여 세션이 도달할 수 있는 모든 것에서 페이지를 구축하므로, 페이지는 설명하는 데 여러 문단이 필요한 것들을 표시할 수 있습니다. 예를 들어 Claude에 다음을 요청하십시오:

* 주석이 달린 diff로 풀 요청을 검토자에게 설명
* 세션이 이미 가져온 데이터에서 대시보드 렌더링
* 여러 디자인 또는 구현 옵션을 나란히 배치
* 긴 작업이 실행되는 동안 채워지는 조사 타임라인 유지
* Slack에 출력을 붙여넣는 대신 팀원에게 링크 전송
* [MCP 커넥터를 통해 새로운 데이터를 가져오는](#pull-live-data-with-mcp-connectors) 상태 보드 게시

[빌드할 수 있는 것](#what-you-can-build)에서 이러한 항목과 일치하는 프롬프트를 참조하십시오. [MCP 커넥터를 통해 라이브 데이터 가져오기](#pull-live-data-with-mcp-connectors)에서 커넥터 기반 보드의 프롬프트를 참조하십시오.

<h3 id="what-an-artifact-is-not">
  아티팩트가 아닌 것
</h3>

아티팩트는 작업의 캡처이지 애플리케이션이 아닙니다. 백엔드가 없는 하나의 자체 포함 페이지이므로 양식 입력을 저장하거나 여러 경로를 제공할 수 없으며, 누군가 이를 볼 때 외부 데이터로의 유일한 경로는 [MCP 커넥터 호출](#pull-live-data-with-mcp-connectors)입니다. 백엔드가 있는 호스팅된 내부 도구의 경우 자신의 인프라에 배포하십시오. 전체 제한 사항 목록은 [페이지 제약](#page-constraints)을 참조하십시오.

<h2 id="create-an-artifact">
  아티팩트 생성
</h2>

Claude는 출력이 페이지에 적합할 때 자동으로 아티팩트를 게시하거나 직접 요청할 수 있습니다. 요청하려면 원하는 기능이나 시각적 출력을 일반 언어로 설명하십시오. 좋은 후보는 텍스트로 읽는 것보다 보기가 더 쉬운 모든 것입니다. 예를 들어 주석이 달린 diff, 차트 또는 비교할 옵션 집합입니다. 아래 프롬프트는 두 가지 예입니다. 더 많은 패턴은 [빌드할 수 있는 것](#what-you-can-build)을 참조하십시오.

```text wrap theme={null}
이 PR을 인라인 주석이 달린 diff로 설명하는 아티팩트를 만드십시오.
```

```text wrap theme={null}
지난주 배포 실패를 서비스별로 표시하는 대시보드 아티팩트를 구축하고 조사하면서 업데이트된 상태로 유지하십시오.
```

Claude는 페이지를 프로젝트의 HTML 또는 Markdown 파일에 작성한 다음 게시합니다. 새 아티팩트를 게시하기 전에 Claude Code는 권한을 요청합니다. `Claude wants to publish "Deploy failures by service" (deploy-failures.html) to a private page on claude.ai`와 같은 내용이 표시될 수 있습니다. 이미 승인한 아티팩트를 다시 게시하면 다시 묻지 않습니다.

**예**를 선택하여 게시하십시오. Claude는 URL을 인쇄하고 브라우저가 새 페이지로 열립니다. 터미널에서 언제든지 `Ctrl+]`를 눌러 가장 최근 아티팩트를 다시 열 수 있습니다.

Claude는 아티팩트의 제목과 브라우저 탭 아이콘용 이모지를 선택합니다. 둘 다 claude.ai의 [아티팩트 갤러리](#share-an-artifact)와 공유 링크에 나타나므로, 특정 제목이나 아이콘을 원하면 Claude에 사용하도록 요청하십시오.

새 아티팩트가 게시될 때 브라우저가 자동으로 열리지 않도록 하려면 환경에서 `CLAUDE_CODE_ARTIFACT_AUTO_OPEN=0`을 설정하십시오.

Claude가 게시할 수 없다고 응답하거나 링크 없이 로컬 HTML 파일을 작성하면 도구가 세션에 대해 활성화되지 않은 것입니다. [가용성](#availability) 요구사항을 확인하십시오.

<h2 id="update-an-artifact">
  아티팩트 업데이트
</h2>

Claude에 페이지를 수정하도록 요청하거나 진행 상황을 다시 게시하는 장기 실행 작업을 허용하십시오. Claude는 기본 파일을 편집하고 동일한 URL로 다시 게시합니다.

```text wrap theme={null}
요약 차트 아래에 지역별 분석을 추가하고 다시 게시하십시오.
```

페이지를 열고 있는 모든 사람이 제자리에서 업데이트를 봅니다. 각 게시는 버전이 되며, 페이지 헤더의 **공유** 컨트롤에서 뷰어가 보는 버전을 선택할 수 있습니다.

다른 세션에서 아티팩트를 업데이트하려면 Claude에 아티팩트의 URL을 제공하고 수정하도록 요청하십시오. URL이 없으면 새 세션은 항상 기존 아티팩트를 업데이트하는 대신 새 아티팩트를 생성합니다.

```text wrap theme={null}
https://claude.ai/code/artifact/5fbea6f3-...을 오늘의 숫자로 업데이트하십시오.
```

<h2 id="share-an-artifact">
  아티팩트 공유하기
</h2>

새로운 아티팩트는 처음에는 사용자에게만 표시됩니다. 이를 공유하려면 브라우저에서 아티팩트를 열고 페이지 헤더의 **공유** 컨트롤을 사용하면 됩니다. 헤더에는 사용자가 아티팩트의 작성자로 표시되므로, 공유 대상자는 누가 페이지를 게시했는지 확인할 수 있습니다. 또한 [claude.ai/code/artifacts](https://claude.ai/code/artifacts)의 갤러리로 연결되며, 여기에는 사용자가 생성한 모든 아티팩트가 나열됩니다.

공유 대상은 사용자의 플랜에 따라 달라집니다:

* **조직 내에서**: Team 및 Enterprise 플랜에서는 조직의 특정 사람 또는 조직의 모든 사람에게 액세스 권한을 부여할 수 있습니다. 뷰어는 페이지를 보기 위해 조직의 구성원으로 claude.ai에 로그인합니다.
* **공개적으로**: 인터넷의 누구나 열 수 있는 링크를 공유하며, claude.ai 로그인이 필요하지 않습니다. Pro 및 Max 플랜에서는 공개 링크가 아티팩트를 공유하는 유일한 방법입니다. Team 및 Enterprise 플랜에서는 Owner가 [조직에 대해 공개 공유를 활성화](#control-public-sharing)할 때까지 공개 공유가 비활성화됩니다.

<h3 id="let-someone-edit-with-you">
  다른 사람이 함께 편집하도록 허용하기
</h3>

공유 대상자는 기본적으로 뷰어입니다: 사용자가 게시한 각 버전을 볼 수 있지만 페이지를 변경할 수 없습니다. Team 및 Enterprise 플랜에서는 다른 사람을 편집자로 만들 수도 있습니다. 공유 대화 상자에서 사람을 추가하고 해당 역할을 **뷰어**에서 **편집자**로 전환합니다.

편집자는 [다른 세션에서 아티팩트를 업데이트](#update-an-artifact)하는 것과 동일한 방식으로 새 버전을 게시합니다: 자신의 세션에서 Claude에게 아티팩트의 URL을 제공하면, Claude는 현재 콘텐츠를 가져와 변경 사항을 적용하여 다시 게시합니다. 페이지를 열어 둔 모든 사람이 각 업데이트를 실시간으로 볼 수 있습니다.

<h2 id="pull-live-data-with-mcp-connectors">
  MCP 커넥터로 라이브 데이터 가져오기
</h2>

아티팩트는 누군가 이를 볼 때마다 [MCP 커넥터](/docs/ko/mcp#use-mcp-servers-from-claude-ai)를 호출할 수 있으므로, 페이지는 이를 구축한 세션에서 수집한 스냅샷이 아닌 현재 데이터를 표시합니다. 아티팩트의 커넥터 호출은 Pro, Max, Team 및 Enterprise 플랜에서 사용 가능하며 Claude Code v2.1.209 이상이 필요합니다. 이전 버전에서는 Claude가 세션이 구축하는 동안 수집한 데이터로 페이지를 게시합니다.

커넥터 기반 페이지를 만들려면 프롬프트에서 커넥터와 원하는 데이터의 이름을 지정하십시오:

```text wrap theme={null}
Build a dashboard artifact of our open pull requests that pulls the live list through my GitHub connector when the page loads.
```

Claude는 게시의 일부로 페이지가 호출할 수 있는 커넥터를 선언하며, 페이지는 해당 선언 외의 커넥터를 호출할 수 없습니다. claude.ai 계정의 커넥터만 적격입니다: Claude가 선언에서 이름을 지정하고, 누군가 페이지를 볼 때 각 호출은 [보는 계정의 자체 연결을 통해](#how-connector-calls-work-for-viewers) 해당 커넥터로 실행됩니다. `.mcp.json`과 같은 Claude Code에서 구성하는 로컬 MCP 서버는 Claude가 페이지를 구축하는 동안 데이터를 제공할 수 있지만, 게시된 페이지는 이들을 호출할 수 없습니다.

페이지는 로드될 때 데이터를 가져오며 간격에 따라 새로 고치거나 보는 사람이 페이지의 새로 고침 컨트롤을 사용할 때 새로 고칠 수 있습니다. 응답은 보는 사람의 브라우저에 캐시되므로, 다시 열린 페이지는 캐시된 응답에서 즉시 렌더링된 후 새로운 결과로 업데이트됩니다.

<h3 id="how-connector-calls-work-for-viewers">
  보는 사람을 위한 커넥터 호출 작동 방식
</h3>

게시된 페이지가 커넥터를 호출할 때, 호출은 이를 게시한 사람의 계정이 아닌 페이지를 보는 사람의 계정을 사용합니다:

* **각 보는 사람은 자신의 커넥터를 사용합니다**: 호출은 보는 계정의 연결된 도구를 통해 이루어지므로, 같은 대시보드를 여는 두 사람은 자신의 계정이 액세스할 수 있는 것에 따라 다른 데이터를 볼 수 있습니다. 페이지는 누구의 자격 증명도 보지 않습니다; claude.ai가 페이지를 대신하여 호출을 수행합니다.
* **보는 사람이 먼저 액세스를 승인합니다**: claude.ai는 페이지의 첫 번째 커넥터 호출 전에 각 보는 사람에게 권한을 요청합니다. 거부하거나 페이지가 사용하는 커넥터를 연결하지 않은 보는 사람도 라이브 섹션 없이 페이지를 볼 수 있습니다.
* **작업도 보는 사람의 계정을 사용합니다**: 페이지는 메시지 게시 또는 문제 업데이트와 같은 부작용이 있는 커넥터 도구를 호출하는 컨트롤을 제공할 수 있습니다. 작업은 컨트롤을 선택하는 사람의 계정을 통해 이루어집니다.

커넥터 기반 페이지를 공유할 계획이라면, Claude에게 각 라이브 섹션에 필요한 커넥터의 이름을 지정하는 폴백 메시지를 포함하도록 요청하십시오. 연결이 없는 보는 사람은 빈 섹션 대신 연결할 항목을 봅니다.

커넥터를 호출하는 아티팩트는 어떤 플랜에서도 공개 링크로 공유할 수 없습니다. Team 및 Enterprise 플랜에서는 비공개로 유지하거나 [조직 내에서 공유](#share-an-artifact)할 수 있습니다. Pro 및 Max 플랜에서는 공개 링크가 공유하는 유일한 방법이므로, 커넥터 기반 아티팩트는 비공개로 유지됩니다.

<h3 id="the-page-shows-no-live-data-for-a-viewer">
  페이지가 보는 사람을 위한 라이브 데이터를 표시하지 않음
</h3>

커넥터 기반 페이지가 렌더링되지만 공유한 사람의 라이브 섹션이 비어 있을 때, 다음 원인을 확인하십시오:

* **보는 사람이 커넥터를 연결하지 않았습니다**: 커넥터는 계정별이므로, 각 보는 사람은 페이지가 호출하는 모든 커넥터에 대한 자신의 연결이 필요합니다. claude.ai의 **설정 > 커넥터**에서 추가한 후 페이지를 다시 로드할 수 있습니다.
* **보는 사람이 권한 요청을 거부했습니다**: 거부는 해당 페이지 로드의 나머지 동안 지속됩니다. 페이지를 다시 로드하면 권한 요청이 다시 나타납니다.
* **조직에 대해 커넥터 호출이 꺼져 있습니다**: 소유자는 관리 설정에서 [**아티팩트 커넥터 활성화** 토글](#control-connector-calls-from-artifacts)을 제어합니다.

<h2 id="what-you-can-build">
  빌드할 수 있는 것
</h2>

아티팩트는 단일 HTML 페이지이므로 HTML, CSS 및 인라인 JavaScript로 표현할 수 있는 모든 것이 범위 내입니다. 아래 패턴이 가장 자주 나타납니다.

<h3 id="walk-through-a-change">
  변경 사항 설명
</h3>

diff 또는 디자인 변경을 관련 줄 옆에 주석과 함께 렌더링하는 페이지를 요청하여 검토자가 설명에서 재구성하는 대신 코드 옆에서 추론을 읽을 수 있도록 하십시오.

```text wrap theme={null}
이 PR을 설명하는 아티팩트를 만드십시오. 여백 주석이 있는 diff를 렌더링하고 심각도별로 결과를 색상 코딩하십시오.
```

<h3 id="compare-alternatives">
  대안 비교
</h3>

한 페이지에 여러 변형을 요청하여 서로 평가할 수 있도록 하십시오. 이는 레이아웃, 복사, API 모양 또는 구현 계획에 적합합니다.

```text wrap theme={null}
설정 패널에 대해 뚜렷하게 다른 4가지 레이아웃이 있는 아티팩트를 만드십시오. 밀도와 그룹화를 변경하고 각각 아래에 한 줄의 트레이드오프가 있는 그리드로 배치하십시오.
```

<h3 id="tune-with-interactive-controls">
  인터랙티브 컨트롤로 조정
</h3>

조정 중인 것에 바인딩된 슬라이더, 토글 또는 입력 필드를 요청하여 설명하는 대신 값을 직접 탐색할 수 있도록 하십시오.

```text wrap theme={null}
이 전환에 대한 이징 곡선, 지속 시간 및 지연에 대한 슬라이더가 있는 아티팩트를 구축하십시오. 이동하면서 애니메이션을 라이브로 표시하십시오.
```

<h3 id="bring-the-result-back-to-your-session">
  결과를 세션으로 다시 가져오기
</h3>

아티팩트는 Claude에 다시 전달하는 결정을 위한 경량 편집기로 작동할 수 있습니다. 페이지와 상호작용한 결과가 페이지에 남아 있는 대신 세션으로 흐르도록 터미널에 붙여넣을 수 있는 텍스트를 생성하는 내보내기 컨트롤을 요청하십시오.

```text wrap theme={null}
각 미해결 문제를 Now, Next, Later, Cut 열 전체에서 드래그 가능한 카드로 하는 트리아지 보드 아티팩트를 만드십시오. "프롬프트로 복사" 버튼을 추가하여 여기에 붙여넣을 최종 순서를 제공하십시오.
```

<h3 id="track-work-in-progress">
  진행 중인 작업 추적
</h3>

Claude가 긴 작업이 실행되는 동안 아티팩트를 최신 상태로 유지하도록 요청하여 링크가 있는 모든 사람이 터미널을 읽지 않고도 따라갈 수 있도록 하십시오.

```text wrap theme={null}
이 마이그레이션 계획을 체크리스트 아티팩트로 변환하십시오. 완료하면서 항목을 확인하고 건너뛴 항목에 대한 메모를 추가하십시오.
```

<h2 id="improve-the-visual-design">
  시각적 디자인 개선
</h2>

Claude Code v2.1.183부터 Claude는 아티팩트를 구축할 때 기본 제공 디자인 기술을 적용하므로 페이지는 추가 프롬프팅 없이 의도적인 팔레트, 타이포그래피 및 레이아웃을 얻습니다. 해당 기술은 또한 자신의 기술을 선택하기 전에 프로젝트의 기존 디자인 시스템을 찾습니다. 아티팩트를 제품 브랜딩과 일치시키려면 Claude가 찾을 수 있는 위치(예: 프로젝트의 [CLAUDE.md](/docs/ko/memory) 또는 저장소의 테마 파일)에 디자인 토큰을 기록하십시오:

```markdown theme={null}
## Design system

- Colors: primary #1a4d8f, accent #f59e0b, surface #f8fafc
- Typography: Inter for body, JetBrains Mono for code
- Spacing: 8px scale, 6px border radius
```

Claude는 디자인 시스템을 자신의 선택보다 높은 우선순위로 취급하고 프롬프트를 둘 다보다 높은 우선순위로 취급합니다. 위의 제목과 형식은 예입니다. 색상, 글꼴 및 간격의 명확한 목록이면 됩니다.

<h2 id="page-constraints">
  페이지 제약
</h2>

각 아티팩트는 하나의 자체 포함 페이지입니다. Claude Code는 게시하는 파일을 HTML 문서 셸로 래핑하고 엄격한 콘텐츠 보안 정책(CSP) 아래에서 제공하며, 이는 페이지가 할 수 있는 것을 형성합니다.

| 제약       | 효과                                                                                                                                                                                                                                                                           |
| :------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 외부 요청 없음 | CSP는 다른 호스트에서 로드된 스크립트, 스타일시트, 글꼴 및 이미지와 `fetch`, XHR 및 WebSocket 호출을 차단합니다. Claude는 CSS 및 JavaScript를 인라인하고 이미지를 데이터 URI로 포함하여 페이지가 외부 요청 없이 렌더링되도록 합니다. [Connector 호출](#pull-live-data-with-mcp-connectors)은 예외입니다. 페이지는 이를 claude.ai에 전달하고, claude.ai가 네트워크 호출을 직접 수행합니다. |
| 백엔드 없음   | 아티팩트는 정적 페이지입니다. 양식을 통해 제출된 데이터를 저장하거나 뷰어를 자체 인증할 수 없습니다. 누군가 페이지를 볼 때 데이터를 가져오는 유일한 방법은 [MCP Connector 호출](#pull-live-data-with-mcp-connectors)이며, 자체 API가 아닙니다.                                                                                                            |
| 단일 페이지   | 상대 링크는 페이지와 함께 배포된 것이 없기 때문에 해결되지 않습니다. 다중 섹션 콘텐츠의 경우 Claude는 별도 파일 대신 페이지 내 앵커를 사용합니다.                                                                                                                                                                                      |
| 소스 파일 유형 | 게시된 파일은 `.html`, `.htm` 또는 `.md`여야 합니다. Markdown 파일은 스타일이 지정된 HTML로 렌더링됩니다.                                                                                                                                                                                                  |
| 렌더링된 크기  | 렌더링된 페이지는 16 MiB 이하여야 합니다. 큰 포함된 이미지는 게시가 크기로 인해 실패할 때 일반적인 원인입니다.                                                                                                                                                                                                           |

아티팩트를 생성하면 다른 응답과 마찬가지로 출력 토큰을 사용하며, 스타일이 지정된 페이지는 동일한 콘텐츠를 터미널 텍스트로 사용하는 것보다 더 토큰 집약적입니다. 인라인 CSS, 인터랙티브 컨트롤용 JavaScript, 특히 데이터 URI로 포함된 이미지가 주요 기여자입니다. 아티팩트의 토큰 비용을 줄이려면:

* 포함된 래스터 이미지보다 다이어그램에 SVG 또는 HTML 및 CSS를 선호합니다.
* 필요하지 않은 상호작용을 생략합니다.
* 페이지가 전체를 인라인하는 대신 큰 데이터 세트를 요약하도록 합니다.

<h2 id="availability">
  가용성
</h2>

아티팩트는 아래의 모든 조건이 필요합니다. 하나가 충족되지 않으면 Claude는 로컬 HTML 파일을 작성하거나 게시할 수 없다고 말합니다.

| 요구사항   | 사용 가능한 경우                                                                                                                                                                                                                                                                                                                                  |
| :----- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 플랜     | Pro, Max, Team 또는 Enterprise. Pro 및 Max 플랜에서는 아티팩트가 사용자에게만 비공개이며 관리자 관리가 적용되지 않습니다. Team 플랜에서는 아티팩트가 기본적으로 켜져 있습니다. Enterprise 플랜에서는 Owner가 claude.ai 관리 설정에서 [활성화](#manage-artifacts-for-your-organization)합니다.                                                                                                                           |
| 인증     | 세션이 claude.ai 계정으로 지원됩니다: CLI 또는 데스크톱 앱에서 `/login`으로 로그인합니다. Claude Tag 세션은 에이전트의 신원을 통해 로그인되므로 추가 단계가 필요하지 않습니다. API 키, [게이트웨이 토큰](/docs/ko/llm-gateway) 또는 클라우드 공급자 자격증명을 사용하는 세션은 게시할 수 없습니다.                                                                                                                                                |
| 모델 공급자 | Anthropic API. [Amazon Bedrock](/docs/ko/amazon-bedrock), [Google Cloud의 Agent Platform](/docs/ko/google-vertex-ai) 또는 [Microsoft Foundry](/docs/ko/microsoft-foundry)에서는 사용할 수 없습니다.                                                                                                                                                                     |
| 조직 정책  | 고객 관리 암호화 키(CMEK), HIPAA 및 [Zero Data Retention](/docs/ko/zero-data-retention)이 조직에 대해 활성화되지 않습니다.                                                                                                                                                                                                                                              |
| 표면     | Claude Code CLI 버전 2.1.183 이상 또는 Claude 데스크톱 앱 버전 1.13576.0 이상. [Claude Tag](https://claude.com/docs/claude-tag/overview) 세션도 Claude Tag와 아티팩트가 모두 조직에 대해 활성화된 경우 아티팩트를 게시할 수 있습니다. [Agent SDK](/docs/ko/agent-sdk/overview), GitHub Action 및 MCP 서버 컨텍스트에서 기본적으로 꺼져 있으며 [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/ko/env-vars)이 설정된 경우입니다. |

<h2 id="disable-artifacts">
  아티팩트 비활성화
</h2>

조직의 설정에 관계없이 자신의 세션에 대해 아티팩트를 끄려면 다음 중 하나를 사용하십시오:

| 방법                       | 설정                                |
| :----------------------- | :-------------------------------- |
| [설정 파일](/docs/ko/settings)    | `"disableArtifact": true`         |
| [환경 변수](/docs/ko/env-vars)    | `CLAUDE_CODE_DISABLE_ARTIFACT=1`  |
| [권한 규칙](/docs/ko/permissions) | `permissions.deny`에 `Artifact` 추가 |

<h2 id="manage-artifacts-for-your-organization">
  조직의 아티팩트 관리
</h2>

Team 및 Enterprise 플랜의 관리자는 [claude.ai 관리 설정](https://claude.ai/admin-settings/claude-code)에서 아티팩트를 제어합니다. 아티팩트 콘텐츠는 Anthropic 운영 인프라에 저장되며 게시 조직의 인증된 구성원에게만 표시됩니다. 아티팩트가 [공개적으로 공유](#control-public-sharing)되지 않는 한 그렇습니다.

<h3 id="enable-or-disable-artifacts">
  아티팩트 활성화 또는 비활성화
</h3>

전체 조직에 대해 아티팩트를 활성화하거나 비활성화하려면 **설정 > Claude Code > 기능**으로 이동하여 **아티팩트** 토글을 사용하십시오. 역할 기반 액세스 제어가 있는 Enterprise 플랜에서는 추가로 아티팩트를 특정 역할로 범위 지정할 수 있습니다. **설정 > 역할**로 이동하여 역할을 편집하고 **Claude Code** 그룹 아래에서 **아티팩트** 권한을 설정하십시오.

<h3 id="control-connector-calls-from-artifacts">
  아티팩트에서 커넥터 호출 제어
</h3>

[아티팩트에서의 커넥터 호출](#pull-live-data-with-mcp-connectors)은 아티팩트를 켜거나 끄는 **아티팩트** 토글과 별도의 토글을 가지고 있습니다. [**설정 > 기능**](https://claude.ai/admin-settings/capabilities)으로 이동하여 **아티팩트 커넥터 활성화** 토글을 사용하십시오. 동일한 토글은 claude.ai 대화에서 생성된 아티팩트의 커넥터 호출을 관리하므로, **설정 > Claude Code**가 아닌 **설정 > 기능** 아래에 위치합니다.

<h3 id="control-public-sharing">
  공개 공유 제어
</h3>

공개 공유는 Team 및 Enterprise 플랜에서 기본적으로 꺼져 있으므로 구성원은 관리자가 켤 때까지 조직 내에서만 아티팩트를 공유할 수 있습니다. 구성원이 로그인 없이 누구나 볼 수 있는 공개 링크에 아티팩트를 게시할 수 있도록 하려면 **설정 > Claude Code > 기능**으로 이동하여 **아티팩트** 토글 아래에서 **외부 공유**를 켜십시오. 다시 끄면 각 아티팩트의 대상을 변경하지 않고 기존 공개 링크를 통한 액세스를 차단합니다. 다시 활성화하면 액세스가 재개됩니다.

<h3 id="set-a-retention-policy">
  보존 정책 설정
</h3>

아티팩트가 자동 삭제 전에 유지되는 기간을 설정하려면 **설정 > 데이터 및 개인정보 보호 제어**로 이동하십시오. 작성자에게만 비공개인 아티팩트와 공유된 아티팩트에 대해 별도의 보존 기간을 설정할 수 있습니다.

<h3 id="review-the-audit-log">
  감사 로그 검토
</h3>

아티팩트 게시, 공유 및 삭제는 각각 조직의 감사 로그에 `claude_artifact_*` 이벤트 유형 아래에 나타나며, 이는 claude.ai 대화에서 생성된 아티팩트에 사용되는 동일한 제품군입니다.

<h3 id="allowlist-the-viewer-domain">
  뷰어 도메인 허용 목록
</h3>

claude.ai의 뷰어는 샌드박스된 `*.claudeusercontent.com` 원본에서 각 아티팩트를 로드합니다. 조직이 아웃바운드 네트워크 액세스를 제한하는 경우 `claude.ai`와 함께 해당 도메인을 허용 목록에 추가하십시오. 전체 목록은 [네트워크 액세스 요구사항](/docs/ko/network-config#network-access-requirements)을 참조하십시오.

<h3 id="list-and-delete-artifacts-with-the-compliance-api">
  Compliance API를 사용하여 아티팩트 나열 및 삭제
</h3>

[Compliance API](https://docs.claude.com/en/api/compliance)는 조직의 아티팩트를 나열하고, 특정 버전의 콘텐츠를 검색하고, 아티팩트를 삭제하는 엔드포인트를 제공합니다:

| 메서드      | 엔드포인트                                                               |
| :------- | :------------------------------------------------------------------ |
| `GET`    | `/v1/compliance/code/artifacts`                                     |
| `GET`    | `/v1/compliance/code/artifacts/{artifact_id}/versions/{version_id}` |
| `DELETE` | `/v1/compliance/code/artifacts/{artifact_id}`                       |

요청 및 응답 스키마는 [Compliance API 참조](https://docs.claude.com/en/api/compliance/code/artifacts)를 참조하십시오.

<h2 id="related-resources">
  관련 리소스
</h2>

* 아티팩트와 쌍을 이루는 [프롬프팅 패턴 및 워크플로우](/docs/ko/prompt-library) 찾아보기
* 재사용하는 아티팩트 프롬프트를 [기술](/docs/ko/skills)로 변환하여 명령으로 호출할 수 있도록 하기
* [MCP 서버 연결](/docs/ko/mcp)하여 Claude가 페이지를 구축하는 동안 아티팩트로 데이터를 가져올 수 있도록 하기
