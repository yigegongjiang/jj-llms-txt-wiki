> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 세션 출력을 아티팩트로 공유하기

> 아티팩트는 Claude Code의 작업을 claude.ai의 라이브 인터랙티브 페이지로 변환하여 비공개로 유지하거나, 조직과 공유하거나, 공개 링크로 게시할 수 있습니다.

<Note>
  아티팩트는 Pro, Max, Team, Enterprise 플랜에서 사용 가능하며 [`/login`](/docs/ko/setup#authenticate)으로 로그인한 세션이 필요합니다. 전체 요구사항은 [가용성](#availability)을 참조하십시오.
</Note>

[아티팩트](https://claude.com/features/artifacts)는 Claude Code가 세션에서 claude.ai의 비공개 URL로 게시하는 라이브 인터랙티브 웹 페이지입니다. 브라우저에서 열 수 있으며, 세션이 계속되면서 제자리에서 업데이트됩니다. 다른 사람이 보기를 원할 때 페이지 헤더에서 공유하십시오.

<Frame>
  <img src="https://mintcdn.com/claude-code/kaHIYYMIYMYPxQg9/images/artifacts-viewer.png?fit=max&auto=format&n=kaHIYYMIYMYPxQg9&q=85&s=dbfd671cdb0d15f49f808b9e89778fe1" alt="claude.ai/code/artifact에서 열린 아티팩트입니다. 뷰어 헤더는 아티팩트 제목 acme-funnel-fix, 공유 버튼, 작성자 아바타를 표시합니다. 공유 메뉴는 최신 버전 항상 공유 토글, 버전 2를 읽는 버전 선택기, Acme 대상 선택기의 모든 사람, 링크 복사 버튼과 함께 열려 있습니다. 헤더 아래에서 아티팩트 페이지는 나란히 두 개의 모바일 목업, 깔때기 차트, 메트릭 카드 행을 표시합니다." width="2511" height="1890" data-path="images/artifacts-viewer.png" />
</Frame>

<h2 id="when-to-use-an-artifact">
  아티팩트를 사용해야 할 때
</h2>

Claude가 생성한 결과물이 터미널 텍스트로는 부적절한 경우 아티팩트를 사용하세요. 즉, 한 줄씩 읽는 것보다 보고 상호작용하기가 더 쉬운 결과물입니다. Claude는 코드베이스와 [연결된 도구](/docs/ko/mcp)를 통해 가져온 데이터를 포함하여 세션이 접근할 수 있는 모든 것으로부터 페이지를 구축하므로, 설명하는 데 여러 문단이 필요할 만한 것들을 페이지에 표시할 수 있습니다. 예를 들어 Claude에 다음을 요청하세요:

* 주석이 달린 diff를 포함하여 검토자에게 풀 요청 설명하기
* 세션이 이미 가져온 데이터로부터 대시보드 렌더링하기
* 여러 설계 또는 구현 옵션을 나란히 배치하기
* 긴 작업이 실행되는 동안 조사 타임라인 유지하기
* Slack에 출력을 붙여넣는 대신 팀원에게 링크 보내기
* [MCP 커넥터를 통해 새로운 데이터를 가져오는](#pull-live-data-with-mcp-connectors) 상태 보드 게시하기

이러한 항목과 일치하는 프롬프트는 [구축할 수 있는 것](#what-you-can-build)을 참조하고, 커넥터 기반 보드의 프롬프트는 [MCP 커넥터를 통해 라이브 데이터 가져오기](#pull-live-data-with-mcp-connectors)를 참조하세요.

<h3 id="what-an-artifact-is-not">
  아티팩트가 아닌 것
</h3>

아티팩트는 작업의 캡처입니다. 백엔드가 없는 자체 포함된 페이지이므로 여러 경로를 제공할 수 없습니다. 호스팅된 백엔드가 있는 내부 도구의 경우 대신 자신의 인프라에 배포하세요. 제한 사항의 전체 집합은 [페이지 제약 사항](#page-constraints)을 참조하세요.

<h2 id="create-an-artifact">
  아티팩트 생성
</h2>

Claude는 출력이 페이지에 적합할 때 자동으로 아티팩트를 게시할 수 있으며, 직접 요청할 수도 있습니다. 요청하려면 기능의 이름을 지정하거나 원하는 시각적 출력을 일반 언어로 설명하면 됩니다. 좋은 후보는 텍스트로 읽는 것보다 보는 것이 더 쉬운 것들입니다. 예를 들어 주석이 달린 diff, 차트 또는 비교할 옵션 집합 등이 있습니다. 아래 프롬프트는 두 가지 예시입니다. 더 많은 패턴은 [빌드할 수 있는 것](#what-you-can-build)을 참조하십시오.

```text wrap theme={null}
Make an artifact that walks through this PR with the diff annotated inline.
```

```text wrap theme={null}
Build a dashboard artifact of last week's deploy failures by service and keep it updated as you investigate.
```

위치를 지정하지 않으면 Claude는 프로젝트 외부의 임시 디렉토리에 있는 HTML 또는 Markdown 파일에 페이지를 작성한 후 게시합니다. 새 아티팩트를 게시하면 세션의 [권한 모드](/docs/ko/permission-modes)를 거칩니다:

* **자동 모드**: 분류기가 프롬프트 대신 게시를 검토하므로 Claude는 프롬프트를 보지 않고도 페이지를 게시할 수 있습니다. 세션이 시작되는 모드는 플랜에 따라 다릅니다. [시작 권한 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)를 참조하십시오.
* **수동 및 편집 수락 모드**: Claude Code가 권한을 요청합니다. `Claude wants to publish deploy-failures.html, uploading it to claude.ai (Anthropic's servers) to host as the page "Deploy failures by service", private to you until you share it`와 같은 메시지가 표시될 수 있습니다. **예**를 선택하여 게시합니다.

한 번 아티팩트를 승인한 후 Claude Code는 다시 묻지 않고 다시 게시하며, 다음을 포함한 일부 경우에 다시 묻습니다:

* Claude가 [커넥터 호출](#pull-live-data-with-mcp-connectors) 또는 [파일 다운로드](#offer-a-file-download)와 같은 페이지에 대한 런타임 기능을 선언하는 경우
* 이후 [공개적으로 공유](#share-an-artifact)한 경우
* 이후 특정 사람 또는 조직과 공유했으며 최신 버전이 뷰어가 보는 버전으로 선택된 경우

첫 번째 게시 후 Claude는 URL을 인쇄하고 브라우저가 새 페이지로 열립니다. [Remote Control](/docs/ko/remote-control)을 통해 claude.ai, Claude Desktop 또는 Claude 모바일 앱에서 프롬프트를 보낸 경우 세션을 실행하는 머신에서 탭이 열리지 않습니다. 터미널에서 입력한 프롬프트로부터 Claude가 아티팩트를 다시 게시할 때 다음에 브라우저가 열립니다. 언제든지 `Ctrl+]`를 눌러 세션의 가장 최근 아티팩트를 다시 열 수 있습니다.

Claude는 아티팩트의 제목과 이모지를 선택하며, 둘 다 claude.ai의 [아티팩트 갤러리](#share-an-artifact)와 공유 링크에 나타납니다. Claude는 또한 차트나 달력과 같이 페이지가 무엇인지 일치하는 브라우저 탭 아이콘을 선택할 수 있습니다. 특정 제목, 이모지 또는 탭 아이콘을 원하면 Claude에게 요청하십시오.

새 아티팩트가 게시될 때 브라우저가 자동으로 열리지 않도록 하려면 환경에서 `CLAUDE_CODE_ARTIFACT_AUTO_OPEN=0`을 설정하십시오.

Claude가 게시할 수 없다고 응답하거나 링크 없이 로컬 HTML 파일을 작성하면 도구가 세션에 대해 활성화되지 않은 것입니다. [가용성](#availability) 요구 사항을 확인하십시오.

<h2 id="update-an-artifact">
  아티팩트 업데이트
</h2>

Claude에 페이지를 수정하도록 요청하거나, 장시간 실행되는 작업이 진행되면서 다시 게시하도록 할 수 있습니다. Claude는 기본 파일을 편집하고 동일한 URL로 다시 게시합니다.

```text wrap theme={null}
요약 차트 아래에 지역별 분석을 추가하고 다시 게시합니다.
```

페이지를 열어 있는 모든 사용자는 업데이트를 즉시 확인할 수 있습니다. 각 게시는 버전이 되며, 페이지 헤더의 **공유** 컨트롤에서 뷰어가 볼 수 있는 버전을 선택할 수 있습니다.

다른 세션에서 아티팩트를 업데이트하려면 Claude에 해당 URL을 제공하거나 [`/artifacts`](#find-an-artifact-again)로 첨부합니다. 둘 다 없으면 새 세션이 기존 아티팩트를 업데이트하는 대신 새 아티팩트를 만듭니다.

```text wrap theme={null}
https://claude.ai/code/artifact/5fbea6f3-...을 오늘의 숫자로 업데이트합니다.
```

<h2 id="find-an-artifact-again">
  아티팩트 다시 찾기
</h2>

Claude Code에서 `/artifacts`를 실행하여 소유한 모든 아티팩트와 공유받은 모든 아티팩트를 나열합니다. 하나를 선택하고 `o`를 눌러 브라우저에서 열거나 `c`를 눌러 링크를 복사합니다. Enter를 눌러 현재 세션에 첨부합니다. v2.1.216 이전에는 Enter를 누르면 브라우저에서 열렸습니다. Claude Code는 claude.ai 계정에서 목록을 읽으므로 새 세션에서 작동하며 링크가 터미널에서 스크롤되어 나간 후 `/clear` 후에도 작동합니다. Claude Code v2.1.208 이상이 필요합니다.

<h2 id="share-an-artifact">
  아티팩트 공유하기
</h2>

새로운 아티팩트는 처음에는 사용자에게만 표시됩니다. 공유하려면 브라우저에서 아티팩트를 열고 페이지 헤더의 **공유** 컨트롤을 사용하십시오. 헤더에는 [claude.ai/code/artifacts](https://claude.ai/code/artifacts)의 갤러리로 연결되는 링크도 있으며, 여기에는 사용자가 생성한 모든 아티팩트가 나열됩니다.

조직의 뷰어는 페이지를 게시한 사람을 볼 수 있습니다. 조직 내에서 공유된 아티팩트의 경우 사용자의 이름이 제목 메뉴에 표시되고, 공개 아티팩트의 경우 조직의 로그인한 뷰어를 위해 페이지 헤더에 표시됩니다. 로그인하지 않고 공개 링크를 열거나 조직 외부에서 열어본 뷰어는 사용자의 이름 대신 `콘텐츠는 사용자가 생성했으며 검증되지 않았습니다.` 레이블을 봅니다.

공유할 수 있는 대상은 사용자의 플랜에 따라 다릅니다.

* **조직 내**: Team 및 Enterprise 플랜에서는 조직의 특정 사람 또는 모든 사람에게 액세스 권한을 부여할 수 있습니다. 뷰어는 페이지를 보기 위해 조직의 구성원으로 claude.ai에 로그인합니다.
* **공개**: 인터넷의 누구나 열 수 있는 링크를 공유하며, claude.ai 로그인이 필요하지 않습니다. Pro 및 Max 플랜에서는 공개 링크가 아티팩트를 공유하는 유일한 방법입니다. Team 및 Enterprise 플랜에서는 Owner가 [조직에 대해 공개 공유를 활성화](#control-public-sharing)할 때까지 공개 공유가 비활성화됩니다.

<h3 id="let-someone-edit-with-you">
  다른 사람이 함께 편집하도록 허용하기
</h3>

공유한 사람들은 기본적으로 뷰어입니다. 즉, 사용자가 게시한 각 버전을 볼 수 있지만 페이지를 변경할 수 없습니다. Team 및 Enterprise 플랜에서는 누군가를 편집자로 만들 수도 있습니다. 공유 대화 상자에서 사람을 추가하고 역할을 **뷰어**에서 **편집자**로 전환합니다.

편집자는 [다른 세션에서 아티팩트를 업데이트](#update-an-artifact)하는 것과 같은 방식으로 새 버전을 게시합니다. 즉, Claude에 아티팩트의 URL을 제공하거나 [`/artifacts`](#find-an-artifact-again)에서 첨부하면 Claude가 현재 콘텐츠를 가져와 변경 사항과 함께 다시 게시합니다. 페이지를 열어본 모든 사람이 각 업데이트를 실시간으로 봅니다.

<h2 id="read-an-artifact-shared-with-you">
  공유받은 아티팩트 읽기
</h2>

누군가 아티팩트를 공유할 때, Claude가 이를 읽도록 할 수 있습니다. Claude에 URL을 제공하거나 [`/artifacts`](#find-an-artifact-again)에서 첨부하면 됩니다.

Claude는 다른 사람이 작성한 페이지를 [WebFetch](/docs/ko/tools-reference#webfetch-tool-behavior)로 웹 페이지를 읽는 방식으로 읽습니다. 원본 페이지 대신 요청한 내용의 요약을 받으며, 요약은 페이지에 작성된 지시사항을 전달하는 대신 보고합니다. Claude Code는 또한 페이지의 전체 소스를 로컬 파일로 저장하므로, Claude는 [편집자](#let-someone-edit-with-you)로 아티팩트를 다시 게시하는 등 정확한 내용이 필요할 때 파일을 열 수 있습니다.

<h2 id="collect-comments-on-an-artifact">
  아티팩트에 대한 댓글 수집
</h2>

조직 내에서 아티팩트를 공유할 때, 공유 대상자들은 페이지에 댓글을 남길 수 있으며, Claude가 해당 댓글을 읽고 답변할 수 있습니다. Claude Code v2.1.221 이상과 Team 또는 Enterprise 플랜이 필요합니다. 왜냐하면 [조직 내에서 공유](#share-an-artifact)하는 아티팩트만 댓글을 받기 때문입니다. Claude는 두 가지 경우에 댓글을 읽습니다:

* **Claude에게 읽도록 요청하는 경우**: Claude에게 아티팩트의 URL을 제공하고 댓글을 요청합니다. Claude는 각 스레드를 나열하고 아티팩트를 편집할 수 있는 사람이 보낸 댓글을 표시합니다.
* **아티팩트를 편집할 수 있는 사람이 Claude에게 댓글을 보내는 경우**: 페이지의 스레드에서 **Send to Claude**로 댓글을 보내거나 `@claude`를 언급합니다. 어느 쪽이든 스레드가 활성화됩니다.

Claude는 활성화된 스레드에만 답변하거나 해결할 수 있습니다. 다른 스레드는 사람이 페이지에서 해결할 때까지 열린 상태로 유지됩니다. 뷰어는 각 답변이 Claude에게 귀속되는 것을 봅니다(당신을 통해).

아티팩트를 공개적으로 공유하면 뷰어는 댓글을 달 수 없습니다: 페이지에 `Comments aren't available while this Artifact is shared publicly.`라고 표시됩니다. 이미 댓글 스레드가 있는 아티팩트를 공개 링크로 전환하려면 먼저 스레드를 삭제하십시오.

직접 댓글을 요청하려면 Claude에게 URL을 제공하십시오:

```text wrap theme={null}
Read the comments on https://claude.ai/code/artifact/5fbea6f3-... and make the changes the commenters ask for.
```

Claude가 댓글을 읽을 수 없다고 말하면 버전, 세션 및 기능 플래그 설정을 확인하십시오:

* Claude Code v2.1.221 이상을 실행 중입니다.
* Claude Code를 설치하거나 v2.1.221 이전 버전에서 업그레이드한 후 첫 번째 세션이 아닙니다. [설치 또는 업그레이드 후 첫 번째 세션](/docs/ko/env-vars#first-session-after-an-install-or-upgrade)에서는 Claude가 아직 댓글을 읽지 못할 수 있습니다. 새 세션을 시작하고 다시 요청하십시오.
* 기능 플래그 가져오기를 끄지 않았습니다.

<h3 id="let-claude-reply-to-comments-on-its-own">
  Claude가 댓글에 자동으로 답변하도록 허용
</h3>

세션이 아티팩트를 게시한 후 Claude Code는 세션이 실행되는 동안 해당 아티팩트의 댓글을 감시합니다. 아티팩트를 편집할 수 있는 사람이 Claude에게 댓글을 보내면 즉시 세션에 도달하고, Claude는 스레드를 읽고 당신이 요청하지 않아도 답변할 수 있습니다.

Claude Code v2.1.228 이상이 필요합니다. [기능 플래그 가져오기](/docs/ko/env-vars#features-that-need-feature-flag-fetching)를 끄면 Claude Code는 댓글을 감시하지 않습니다.

[권한 모드](/docs/ko/permission-modes)는 보낸 댓글이 도착할 때 Claude가 수행하는 작업을 결정합니다:

* **Claude가 자동으로 답변합니다**: 권한 모드가 Claude가 당신에게 묻지 않고 답변을 게시하도록 허용할 때, Claude는 스레드를 읽고 답변하며, 댓글이 변경을 요청할 때 아티팩트를 편집합니다. `Auto-replied to comment thread on Artifact: <name>` 또는 `Auto-edited Artifact: <name> in response to a comment thread`가 표시됩니다.
* **Claude가 당신을 기다립니다**: 계획 모드 외부에서 답변 게시에 승인이 필요할 때, `Comments are waiting on Artifact: <name>`이 표시됩니다. Claude는 스레드를 읽을 승인을 요청하고, 다시 답변을 게시할 승인을 요청합니다.
* **Claude가 계획 모드에서 일시 중지됩니다**: `Comments are waiting on Artifact: <name>`이 표시되고, 계획 모드를 종료하고 스레드를 읽고 답변하도록 요청할 때까지 Claude는 답변하지 않습니다.

Claude는 또한 1시간 내에 해당 아티팩트에서 60개의 보낸 댓글 또는 스레드 활성화를 처리한 후 아티팩트에 자동으로 답변하는 것을 중지합니다. `Comments are waiting on Artifact: <name>`이 한 번 표시되고, Claude는 해당 시간의 댓글이 오래되면서 다시 시작합니다.

`/tasks`를 실행하여 세션이 감시 중인 각 아티팩트를 실시간 업데이트 작업으로 나열된 것을 확인합니다. 다음 방법 중 하나로 Claude가 아티팩트에 자동으로 답변하는 것을 중지할 수 있습니다:

* **유휴 프롬프트에서 Ctrl+C를 한 번 누릅니다**: Claude는 세션이 감시 중인 모든 아티팩트에 답변하는 것을 일시 중지합니다. 다음 메시지를 보낸 후 답변이 다시 시작됩니다.
* **`/tasks`에서 작업을 중지합니다**: Claude는 당신이 해당 아티팩트에서 답변을 재개하도록 요청할 때까지 해당 아티팩트에 답변하는 것을 중지합니다. 아티팩트를 다시 게시해도 답변이 다시 시작되지 않으며, 세션을 재개할 때도 중지가 계속 적용됩니다.
* **3초 내에 `Ctrl+X Ctrl+K`를 두 번 누릅니다**: [모든 실행 중인 백그라운드 서브에이전트를 중지](/docs/ko/interactive-mode#general-controls)하는 코드는 또한 Claude가 세션의 나머지 기간 동안 모든 아티팩트에 답변하는 것을 중지합니다. Claude에게 답변을 재개하도록 요청해도 이 중지는 취소되지 않습니다.

댓글을 전달하는 서비스를 사용할 수 없거나 응답을 중지하면 Claude Code는 한동안 다시 연결을 시도한 후 세션이 감시 중이던 각 아티팩트 감시를 중지합니다.

<h2 id="pull-live-data-with-mcp-connectors">
  MCP 커넥터로 실시간 데이터 가져오기
</h2>

아티팩트는 누군가 이를 볼 때마다 [MCP 커넥터](/docs/ko/mcp#use-mcp-servers-from-claude-ai)를 호출할 수 있으므로, 페이지는 이를 구축한 세션에서 수집한 스냅샷이 아닌 현재 데이터를 표시합니다. 아티팩트의 커넥터 호출은 Pro, Max, Team, Enterprise 플랜에서 사용 가능하며 Claude Code v2.1.209 이상이 필요합니다. 이전 버전에서는 Claude가 세션이 구축하는 동안 수집한 데이터로 페이지를 게시합니다.

커넥터 기반 페이지를 만들려면 프롬프트에서 커넥터와 원하는 데이터의 이름을 지정하세요:

```text wrap theme={null}
Build a dashboard artifact of our open pull requests that pulls the live list through my GitHub connector when the page loads.
```

Claude는 게시의 일부로 페이지가 호출할 수 있는 커넥터를 선언하며, 페이지는 해당 선언 외의 커넥터를 호출할 수 없습니다. claude.ai 계정의 커넥터만 적격입니다: Claude가 선언에서 이름을 지정하고, 누군가 페이지를 볼 때 각 호출은 [보는 계정의 자체 연결을 통해](#how-connector-calls-work-for-viewers) 해당 커넥터로 실행됩니다. `.mcp.json`과 같은 Claude Code에서 구성하는 로컬 MCP 서버는 Claude가 페이지를 구축하는 동안 데이터를 제공할 수 있지만, 게시된 페이지는 이들을 호출할 수 없습니다.

페이지는 로드될 때 데이터를 가져오며 간격에 따라 새로 고치거나 보는 사람이 페이지의 새로 고침 컨트롤을 사용할 때 새로 고칠 수 있습니다. 응답은 보는 사람의 브라우저에 캐시되므로, 다시 열린 페이지는 캐시된 응답에서 즉시 렌더링된 후 새로운 결과로 업데이트됩니다.

<h3 id="how-connector-calls-work-for-viewers">
  보는 사람을 위한 커넥터 호출 작동 방식
</h3>

게시된 페이지가 커넥터를 호출할 때, 호출은 이를 게시한 사람의 계정이 아닌 페이지를 보는 사람의 계정을 사용합니다:

* **각 보는 사람이 자신의 커넥터를 사용합니다**: 호출은 보는 계정의 연결된 도구를 통해 이루어지므로, 같은 대시보드를 여는 두 사람은 자신의 계정이 액세스할 수 있는 것에 따라 다른 데이터를 볼 수 있습니다. 페이지는 누구의 자격 증명도 보지 않습니다; claude.ai가 페이지를 대신하여 호출을 수행합니다.
* **보는 사람이 먼저 액세스를 승인합니다**: claude.ai는 페이지의 첫 번째 커넥터 호출 전에 각 보는 사람에게 권한을 요청합니다. 거부하거나 페이지가 사용하는 커넥터를 연결하지 않은 보는 사람도 실시간 섹션 없이 페이지를 볼 수 있습니다.
* **작업도 보는 사람의 계정을 사용합니다**: 페이지는 메시지 게시 또는 문제 업데이트와 같은 부작용이 있는 커넥터 도구를 호출하는 컨트롤을 제공할 수 있습니다. 작업은 컨트롤을 선택하는 사람의 계정을 통해 이루어집니다.

커넥터 기반 페이지를 공유할 계획이라면, Claude에게 각 실시간 섹션에 필요한 커넥터의 이름을 지정하는 폴백 메시지를 포함하도록 요청하세요. 연결이 없는 보는 사람은 빈 섹션 대신 연결할 항목을 봅니다.

커넥터를 호출하는 아티팩트는 어떤 플랜에서도 공개 링크로 공유할 수 없습니다. Team 및 Enterprise 플랜에서는 비공개로 유지하거나 [조직 내에서 공유](#share-an-artifact)할 수 있습니다. 공개 링크가 공유하는 유일한 방법인 Pro 및 Max 플랜에서는 커넥터 기반 아티팩트가 비공개로 유지됩니다.

<h3 id="the-page-shows-no-live-data-for-a-viewer">
  페이지가 보는 사람을 위한 실시간 데이터를 표시하지 않음
</h3>

커넥터 기반 페이지가 렌더링되지만 공유한 사람의 실시간 섹션이 비어 있을 때, 다음 원인을 확인하세요:

* **보는 사람이 커넥터를 연결하지 않았습니다**: 커넥터는 계정별이므로, 각 보는 사람은 페이지가 호출하는 모든 커넥터에 대한 자신의 연결이 필요합니다. claude.ai의 **설정 > 커넥터**에서 추가한 후 페이지를 다시 로드할 수 있습니다.
* **보는 사람이 권한 요청을 거부했습니다**: 거부는 해당 페이지 로드의 나머지 동안 지속됩니다. 페이지를 다시 로드하면 권한 요청이 다시 나타납니다.
* **조직에 대해 커넥터 호출이 꺼져 있습니다**: 소유자가 관리 설정에서 [**아티팩트 커넥터 활성화** 토글](#control-connector-calls-from-artifacts)을 제어합니다.
* **페이지가 커넥터가 노출하지 않는 도구 이름을 호출합니다**: 영향을 받는 섹션은 당신을 포함한 모든 사람에게 비어 있습니다. 이는 페이지가 자신의 도구만 몇 개 노출하는 게이트웨이 스타일 커넥터 뒤의 개별 도구의 이름을 지정할 때 발생할 수 있습니다. Claude에게 페이지가 호출하는 도구 이름을 수정하고 다시 게시하도록 요청하세요.

  Claude가 페이지를 게시할 때 해당 커넥터의 도구를 세션에서 사용할 수 있으면, Claude Code는 페이지가 선언하는 도구 이름을 이들과 비교하고, 일치하지 않는 이름에 대해 Claude에게 경고하며, 일치하는 이름이 없으면 게시를 거부합니다. v2.1.265 이전에는 이들을 확인하지 않고 페이지를 게시했습니다.

<h2 id="offer-a-file-download">
  파일 다운로드 제공
</h2>

아티팩트는 페이지가 생성하는 파일(예: 테이블의 CSV 내보내기 또는 차트의 PNG)을 뷰어에게 제공할 수 있습니다. 뷰어는 페이지의 다운로드 컨트롤(예: 버튼)을 통해 파일을 저장합니다. 파일 다운로드는 claude.ai가 계정별로 제공하는 런타임 기능이므로, Claude는 컨트롤을 구축하기 전에 계정에 이 기능이 있는지 확인합니다.

뷰어는 일반 다운로드 링크나 페이지의 스크립트에서 파일을 저장할 수 없습니다. claude.ai의 아티팩트 뷰어는 페이지가 시작하는 모든 다운로드(예: `data:` 또는 `blob:` URL로의 링크)를 차단하기 때문입니다. 페이지에 이런 방식으로 구축된 다운로드 버튼이 있다면, Claude에게 다운로드 기능으로 다시 구축해 달라고 요청하세요.

파일을 제공하려면 프롬프트에서 컨트롤과 파일 형식을 요청하세요:

```text wrap theme={null}
Add a button that downloads this table as a CSV file.
```

Claude는 [커넥터를 선언](#pull-live-data-with-mcp-connectors)하는 것과 같은 방식으로 게시의 일부로 다운로드 기능을 선언합니다.

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

Claude는 아티팩트를 빌드할 때 내장된 디자인 스킬을 적용하므로, 추가 프롬프트 없이도 페이지가 의도적인 팔레트, 타이포그래피 및 레이아웃을 얻습니다. 해당 스킬은 자신의 선택을 하기 전에 프로젝트의 기존 디자인 시스템을 찾습니다. 디자인 토큰은 디자인 시스템이 재사용하는 명명된 색상, 타이포그래피 및 간격 값입니다. 아티팩트를 제품의 브랜딩과 일치시키려면, Claude가 찾을 수 있는 위치(예: 프로젝트의 [CLAUDE.md](/docs/ko/memory) 또는 리포지토리의 테마 파일)에 기록하세요:

```markdown theme={null}
## Design system

- Colors: primary #1a4d8f, accent #f59e0b, surface #f8fafc
- Typography: Inter for body, JetBrains Mono for code
- Spacing: 8px scale, 6px border radius
```

Claude는 디자인 시스템을 자신의 선택보다 높은 우선순위로 취급하며, 프롬프트를 둘 다보다 높은 우선순위로 취급합니다. 위의 제목과 형식은 예시이며, 색상, 글꼴 및 간격의 명확한 목록이면 모두 작동합니다.

타이포그래피의 경우, Claude는 Google Fonts에서 타입페이스를 로드할 수 있으며, 이는 아티팩트 페이지가 로드할 수 있는 유일한 외부 글꼴 소스입니다. Claude는 다른 모든 타입페이스를 `@font-face` 데이터 URI로 인라인하고 모든 타입페이스에 폴백 스택을 제공하므로, 글꼴이 로드되지 않아도 페이지가 여전히 렌더링됩니다. 특정 타입페이스를 사용하려면 프롬프트나 디자인 시스템에서 이름을 지정하세요.

<h2 id="draft-a-design-canvas">
  디자인 캔버스 초안 작성
</h2>

UI, 화면 흐름, 랜딩 페이지 또는 포스터를 구축하기보다는 목업하려면 간단한 설명과 함께 `/design`을 실행하세요. Claude는 디자인을 하나의 캔버스에 아트보드로 초안 작성하고 Design 아티팩트로 캔버스를 게시합니다. 설명은 그려야 할 내용을 지정합니다:

```text wrap theme={null}
/design a settings screen for a mobile banking app
```

게시된 아티팩트를 데스크톱 브라우저에서 열어 아트보드를 검토하세요. 아트보드의 요소를 선택하고 변경하면 편집 내용이 자동으로 저장됩니다. 각 아트보드를 PNG 또는 PDF로 내보낼 수 있습니다.

`/design`은 [아티팩트를 사용할 수 있는](#availability) 세션과 Claude Code v2.1.265 이상이 필요합니다.

<h2 id="page-constraints">
  페이지 제약 사항
</h2>

각 아티팩트는 하나의 자체 포함된 페이지입니다. Claude Code는 게시하는 파일을 HTML 문서 셸로 래핑하고 엄격한 콘텐츠 보안 정책(CSP)에 따라 제공하며, 이는 페이지가 수행할 수 있는 작업을 결정합니다.

| 제약 사항    | 효과                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| :------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 외부 요청    | 페이지는 Google Fonts에서 글꼴을 로드할 수 있으며, [5개의 공개 CDN 호스트](#allowlist-the-viewer-domain)에서 스크립트를 로드할 수 있습니다: cdnjs, unpkg, Tailwind 및 jQuery CDN, 그리고 jsDelivr의 선택된 경로(예: `/npm/`). CSP는 모든 외부 이미지와 다른 모든 외부 스크립트, 스타일시트 및 글꼴을 차단하며, `fetch`, XHR 및 WebSocket 호출이 페이지 자체 원본과 Google Fonts 호스트에만 도달하도록 합니다. Claude는 따라서 페이지가 필요로 하는 모든 라이브러리를 이러한 CDN 중 하나에서 로드하고, 다른 모든 CSS 및 JavaScript를 인라인으로 처리하며, 이미지를 데이터 URI로 임베드합니다. [커넥터 호출](#pull-live-data-with-mcp-connectors)은 claude.ai를 통해 진행되며, 이는 네트워크 호출을 자체적으로 수행합니다. |
| 백엔드 없음   | 아티팩트는 정적 페이지입니다. 이는 뷰어를 자체적으로 인증할 수 없습니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| 다운로드     | 페이지는 자체적으로 다운로드를 시작할 수 없습니다. 뷰어가 페이지에서 생성하는 파일을 저장할 수 있도록 하려면 Claude가 다운로드 기능을 선언합니다. [파일 다운로드 제공](#offer-a-file-download)을 참조하십시오.                                                                                                                                                                                                                                                                                                                                                                              |
| 단일 페이지   | 상대 링크는 페이지와 함께 배포된 것이 없기 때문에 해석되지 않습니다. 다중 섹션 콘텐츠의 경우 Claude는 별도의 파일이 아닌 페이지 내 앵커를 사용합니다.                                                                                                                                                                                                                                                                                                                                                                                                                        |
| 소스 파일 유형 | 게시된 파일은 `.html`, `.htm` 또는 `.md`여야 하며, UTF-8로 디코딩되거나 바이트 순서 표시에 의해 little-endian UTF-16으로 디코딩되어야 합니다. Markdown 파일은 구문 강조 표시된 코드가 있는 스타일이 지정된 문서 페이지로 렌더링됩니다. 디코딩되지 않거나 대체 문자 `U+FFFD`를 포함하는 파일은 [수정할 줄과 열과 함께 거부됩니다](/docs/ko/errors#the-source-file-is-not-valid-utf-8-text).                                                                                                                                                                                                                                        |
| 렌더링된 크기  | 렌더링된 페이지는 16 MiB 이하여야 합니다. 게시 실패가 크기로 인한 경우 일반적으로 큰 임베드된 이미지가 원인입니다.                                                                                                                                                                                                                                                                                                                                                                                                                                             |

아티팩트를 생성하면 다른 응답과 마찬가지로 출력 토큰을 사용하며, 스타일이 지정된 페이지는 동일한 콘텐츠를 터미널 텍스트로 표현하는 것보다 더 토큰 집약적입니다. 인라인 CSS, 대화형 컨트롤을 위한 JavaScript, 특히 데이터 URI로 임베드된 이미지가 주요 기여 요소입니다. 아티팩트의 토큰 비용을 줄이려면:

* 임베드된 래스터 이미지보다 다이어그램에 SVG 또는 HTML 및 CSS를 선호합니다
* 필요하지 않은 상호작용을 생략합니다
* 페이지가 전체 인라인 대신 큰 데이터 세트를 요약하도록 합니다

<h2 id="availability">
  가용성
</h2>

Artifacts는 아래의 모든 조건을 충족해야 합니다. 하나라도 충족되지 않으면 Claude는 로컬 HTML 파일을 작성하거나 게시할 수 없다고 말합니다.

| 요구사항   | 사용 가능한 경우                                                                                                                                                                                                                                                                                                                                        |
| :----- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 플랜     | Pro, Max, Team 또는 Enterprise. Pro 및 Max 플랜에서는 artifacts가 공유할 때까지 사용자에게만 비공개이며 관리자 관리가 적용되지 않습니다. Team 플랜에서는 artifacts가 기본적으로 활성화됩니다. Enterprise 플랜에서는 Owner가 claude.ai 관리자 설정에서 [이를 활성화](#manage-artifacts-for-your-organization)합니다.                                                                                                            |
| 인증     | 세션이 claude.ai 계정으로 지원됩니다: CLI 또는 데스크톱 앱에서 `/login`으로 로그인합니다. Claude Tag 세션은 에이전트의 ID를 통해 로그인되므로 추가 단계가 필요하지 않습니다. API 키, [gateway token](/docs/ko/llm-gateway) 또는 클라우드 제공자 자격증명을 사용하는 세션은 게시할 수 없습니다.                                                                                                                                                 |
| 모델 제공자 | Anthropic API. [Amazon Bedrock](/docs/ko/amazon-bedrock), [Google Cloud의 Agent Platform](/docs/ko/google-vertex-ai) 또는 [Microsoft Foundry](/docs/ko/microsoft-foundry)에서는 사용할 수 없습니다.                                                                                                                                                                           |
| 조직 정책  | 고객 관리 암호화 키(CMEK), HIPAA 및 [Zero Data Retention](/docs/ko/zero-data-retention)이 조직에 대해 활성화되지 않았습니다.                                                                                                                                                                                                                                                   |
| 표면     | Claude Code CLI 또는 Claude 데스크톱 앱 버전 1.13576.0 이상. [Claude Tag](https://claude.com/docs/claude-tag/overview) 세션도 Claude Tag와 artifacts가 조직에 대해 활성화된 경우 artifacts를 게시할 수 있습니다. [Agent SDK](/docs/ko/agent-sdk/overview), GitHub Action 및 MCP-server 컨텍스트에서는 기본적으로 비활성화되며, [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/ko/env-vars)이 설정된 경우에도 비활성화됩니다. |

조직에서 artifacts가 허용되는지 여부는 Claude Code가 `api.anthropic.com`에서 로드하는 조직의 정책에서 나옵니다. Claude Code가 정책을 로드할 수 없으면 artifacts를 사용할 수 없습니다. 사용자가 요청하면 Claude가 그 이유를 설명합니다.

프록시, VPN 또는 웹 필터가 관련된 경우 IT 관리자에게 `api.anthropic.com`을 허용하도록 요청하십시오. Claude Code는 백그라운드에서 계속 재시도하며, 정책이 로드되고 이를 허용하면 artifacts를 사용할 수 있게 됩니다.

<h2 id="disable-artifacts">
  아티팩트 비활성화
</h2>

조직의 설정에 관계없이 자신의 세션에 대해 아티팩트를 끄려면 다음 중 하나를 사용하십시오:

| 방법                        | 설정                                                                                               |
| :------------------------ | :----------------------------------------------------------------------------------------------- |
| [`/config`](/docs/ko/commands) | **아티팩트** 행을 끄면 사용자 설정에 [`"enableArtifact": false`](/docs/ko/settings-reference#enableartifact)가 기록됩니다 |
| [설정 파일](/docs/ko/settings)     | `"enableArtifact": false`를 설정합니다. 더 이상 사용되지 않는 `"disableArtifact": true`도 아티팩트를 끕니다              |
| [환경 변수](/docs/ko/env-vars)     | `CLAUDE_CODE_DISABLE_ARTIFACT=1`을 설정합니다                                                          |
| [권한 규칙](/docs/ko/permissions)  | `permissions.deny`에 `Artifact`를 추가합니다                                                            |

[`--settings`](/docs/ko/cli-reference#cli-flags) 파일에서 아티팩트를 끄거나 `CLAUDE_CODE_DISABLE_ARTIFACT`를 사용하거나, 관리자가 [관리되는 설정](/docs/ko/server-managed-settings)에서 아티팩트를 끄면, 어떤 설정 파일도 아티팩트를 다시 켤 수 없습니다. v2.1.242 이전에는 [우선순위 스택](/docs/ko/settings#settings-precedence)에서 더 높은 파일이 낮은 우선순위 파일에서 `"enableArtifact": false`를 설정했을 때도 아티팩트를 다시 켤 수 있었습니다.

프로젝트의 `.claude/settings.json` 또는 `.claude/settings.local.json`에서 `"enableArtifact": false`를 설정하여 해당 프로젝트의 세션에 대해 아티팩트를 끌 수도 있습니다. 두 파일 중 하나에서 `"enableArtifact": true`를 설정해도 아티팩트를 다시 켜지 않습니다. 프로젝트 및 로컬 설정에서 키를 인식하려면 Claude Code v2.1.242 이상이 필요합니다.

`domain:` 부분이 없는 `WebFetch` 거부 또는 요청 규칙을 추가하면 아티팩트를 끄거나 아티팩트 읽기를 차단하지 않습니다. [`deny` 또는 `ask`의 `WebFetch(domain:claude.ai)` 규칙은 아티팩트 읽기에 적용됩니다](/docs/ko/permissions#allow-or-deny-every-fetch).

<h2 id="manage-artifacts-for-your-organization">
  조직의 아티팩트 관리
</h2>

Team 및 Enterprise 플랜의 관리자는 [claude.ai 관리 설정](https://claude.ai/admin-settings/claude-code)에서 아티팩트를 제어합니다. 아티팩트 콘텐츠는 Anthropic 운영 인프라에 저장되며 게시 조직의 인증된 구성원에게만 표시됩니다. 아티팩트가 [공개적으로 공유](#control-public-sharing)되지 않는 한 그렇습니다.

<h3 id="enable-or-disable-artifacts">
  아티팩트 활성화 또는 비활성화
</h3>

전체 조직에 대해 아티팩트를 활성화하거나 비활성화하려면 [**설정 > Claude Code > 기능**](https://claude.ai/admin-settings/claude-code)으로 이동하여 **아티팩트** 토글을 사용하십시오. 역할 기반 액세스 제어가 있는 Enterprise 플랜에서는 추가로 아티팩트를 특정 역할로 범위 지정할 수 있습니다. [**설정 > 역할**](https://claude.ai/admin-settings/roles)로 이동하여 역할을 편집하고 **Claude Code** 그룹 아래에서 **아티팩트** 권한을 설정하십시오.

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

아티팩트가 자동 삭제 전에 유지되는 기간을 설정하려면 [**설정 > 데이터 및 개인정보 보호 제어**](https://claude.ai/admin-settings/data-privacy-controls)로 이동하십시오. 작성자에게만 비공개인 아티팩트와 공유된 아티팩트에 대해 별도의 보존 기간을 설정할 수 있습니다.

<h3 id="review-the-audit-log">
  감사 로그 검토
</h3>

아티팩트 게시, 공유 및 삭제는 각각 조직의 감사 로그에 `claude_artifact_*` 이벤트 유형 아래에 나타나며, 이는 claude.ai 대화에서 생성된 아티팩트에 사용되는 동일한 제품군입니다.

<h3 id="allowlist-the-viewer-domain">
  뷰어 도메인 허용 목록
</h3>

claude.ai의 뷰어는 샌드박스된 `*.claudeusercontent.com` 원본에서 각 아티팩트를 로드합니다. 조직이 아웃바운드 네트워크 액세스를 제한하는 경우 `claude.ai`와 함께 해당 도메인을 허용 목록에 추가하십시오. 전체 목록은 [네트워크 액세스 요구사항](/docs/ko/network-config#network-access-requirements)을 참조하십시오.

[Google Fonts](#improve-the-visual-design)에서 글꼴을 로드하는 아티팩트는 `fonts.googleapis.com` 및 `fonts.gstatic.com`도 요청합니다. 두 호스트 모두 선택 사항입니다. 이들을 차단하면 아티팩트는 대체 글꼴로 렌더링됩니다. 글꼴 요청이 페이지의 첫 렌더링을 지연시키는 대신 즉시 실패하도록 빠른 거부로 차단하십시오.

아티팩트는 또한 React 또는 차트 패키지와 같은 JavaScript 라이브러리를 `cdnjs.cloudflare.com`, `cdn.jsdelivr.net`, `cdn.tailwindcss.com`, `code.jquery.com`, `unpkg.com`에서 로드할 수 있으며 다른 외부 호스트에서는 로드할 수 없습니다. 이러한 호스트를 차단하면 라이브러리에 의존하는 아티팩트의 부분이 작동하지 않으며, 차단된 글꼴과 달리 차단된 라이브러리는 대체 기능이 없습니다. 차단된 라이브러리 요청이 시간 초과될 때까지 중단되는 대신 즉시 실패하도록 여기서도 빠른 거부로 차단하십시오.

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
