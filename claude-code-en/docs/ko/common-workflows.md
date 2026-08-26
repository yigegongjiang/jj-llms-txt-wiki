> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 일반적인 워크플로우

> Claude Code를 사용하여 코드베이스 탐색, 버그 수정, 리팩토링, 테스트 및 기타 일상적인 작업을 위한 단계별 가이드입니다.

이 페이지는 일상적인 개발을 위한 짧은 레시피를 모아놓았습니다. 프롬프팅 및 컨텍스트 관리에 대한 더 높은 수준의 지침은 [모범 사례](/docs/ko/best-practices)를 참조하십시오.

이 페이지는 다음을 다룹니다:

* [프롬프트 레시피](#prompt-recipes) - 코드 탐색, 버그 수정, 리팩토링, 테스트, PR 및 문서화
* [이전 대화 재개](#resume-previous-conversations) - 작업이 여러 세션에 걸쳐 진행될 수 있도록
* [worktree를 사용하여 병렬 세션 실행](#run-parallel-sessions-with-worktrees) - 동시 편집이 충돌하지 않도록
* [편집 전에 계획](#plan-before-editing) - 변경사항이 디스크에 닿기 전에 검토
* [subagent에게 연구 위임](#delegate-research-to-subagents) - 주 컨텍스트를 깨끗하게 유지
* [Claude를 스크립트로 파이프](#pipe-claude-into-scripts) - CI 및 배치 처리용

<h2 id="prompt-recipes">
  프롬프트 레시피
</h2>

이는 낯선 코드 탐색, 디버깅, 리팩토링, 테스트 작성, PR 생성과 같은 일상적인 작업을 위한 프롬프트 패턴입니다. 각각은 모든 Claude Code 표면에서 작동하며, 프로젝트에 맞게 표현을 조정하십시오.

<h3 id="understand-new-codebases">
  새로운 코드베이스 이해하기
</h3>

모노레포 또는 대규모 코드베이스에서 Claude Code를 구성하는 방법은 [모노레포 및 대규모 저장소](/docs/ko/large-codebases)를 참조하십시오.

<h4 id="get-a-quick-codebase-overview">
  코드베이스의 빠른 개요 얻기
</h4>

새로운 프로젝트에 방금 참여했고 그 구조를 빠르게 이해해야 한다고 가정해봅시다.

<Steps>
  <Step title="프로젝트 루트 디렉토리로 이동">
    ```bash theme={null}
    cd /path/to/project 
    ```
  </Step>

  <Step title="Claude Code 시작">
    ```bash theme={null}
    claude 
    ```
  </Step>

  <Step title="높은 수준의 개요 요청">
    ```text theme={null}
    give me an overview of this codebase
    ```
  </Step>

  <Step title="특정 구성 요소에 대해 더 깊이 있게 살펴보기">
    ```text theme={null}
    explain the main architecture patterns used here
    ```

    ```text theme={null}
    what are the key data models?
    ```

    ```text theme={null}
    how is authentication handled?
    ```
  </Step>
</Steps>

<Tip>
  팁:

  * 광범위한 질문으로 시작한 다음 특정 영역으로 좁혀나가기
  * 프로젝트에서 사용되는 코딩 규칙과 패턴에 대해 질문하기
  * 프로젝트별 용어의 용어집 요청하기
</Tip>

<h4 id="find-relevant-code">
  관련 코드 찾기
</h4>

특정 기능이나 기능과 관련된 코드를 찾아야 한다고 가정해봅시다.

<Steps>
  <Step title="Claude에게 관련 파일을 찾도록 요청">
    ```text theme={null}
    find the files that handle user authentication
    ```
  </Step>

  <Step title="구성 요소가 어떻게 상호작용하는지에 대한 컨텍스트 얻기">
    ```text theme={null}
    how do these authentication files work together?
    ```
  </Step>

  <Step title="실행 흐름 이해하기">
    ```text theme={null}
    trace the login process from front-end to database
    ```
  </Step>
</Steps>

<Tip>
  팁:

  * 찾고 있는 것에 대해 구체적으로 설명하기
  * 프로젝트의 도메인 언어 사용하기
  * 언어에 대한 [코드 인텔리전스 플러그인](/docs/ko/discover-plugins#code-intelligence)을 설치하여 Claude에게 정확한 "정의로 이동" 및 "참조 찾기" 네비게이션 제공하기
</Tip>

***

<h3 id="fix-bugs-efficiently">
  효율적으로 버그 수정하기
</h3>

오류 메시지가 나타났고 그 원인을 찾아 수정해야 한다고 가정해봅시다.

<Steps>
  <Step title="Claude와 오류 공유하기">
    ```text theme={null}
    I'm seeing an error when I run npm test
    ```
  </Step>

  <Step title="수정 권장사항 요청하기">
    ```text theme={null}
    suggest a few ways to fix the @ts-ignore in user.ts
    ```
  </Step>

  <Step title="수정 적용하기">
    ```text theme={null}
    update user.ts to add the null check you suggested
    ```
  </Step>
</Steps>

<Tip>
  팁:

  * Claude에게 문제를 재현하는 명령과 스택 추적을 알려주기
  * 오류를 재현하는 단계 언급하기
  * 오류가 간헐적인지 일관적인지 Claude에게 알려주기
</Tip>

***

<h3 id="refactor-code">
  코드 리팩토링
</h3>

오래된 코드를 최신 패턴과 관행을 사용하도록 업데이트해야 한다고 가정해봅시다.

<Steps>
  <Step title="리팩토링할 레거시 코드 식별">
    ```text theme={null}
    find deprecated API usage in our codebase
    ```
  </Step>

  <Step title="리팩토링 권장사항 얻기">
    ```text theme={null}
    suggest how to refactor utils.js to use modern JavaScript features
    ```
  </Step>

  <Step title="안전하게 변경사항 적용하기">
    ```text theme={null}
    refactor utils.js to use ES2024 features while maintaining the same behavior
    ```
  </Step>

  <Step title="리팩토링 검증하기">
    ```text theme={null}
    run tests for the refactored code
    ```
  </Step>
</Steps>

<Tip>
  팁:

  * Claude에게 최신 접근 방식의 이점을 설명하도록 요청하기
  * 필요할 때 변경사항이 하위 호환성을 유지하도록 요청하기
  * 작고 테스트 가능한 증분으로 리팩토링 수행하기
</Tip>

***

<h3 id="work-with-tests">
  테스트 작업하기
</h3>

적용되지 않은 코드에 대한 테스트를 추가해야 한다고 가정해봅시다.

<Steps>
  <Step title="테스트되지 않은 코드 식별">
    ```text theme={null}
    find functions in NotificationsService.swift that are not covered by tests
    ```
  </Step>

  <Step title="테스트 스캐폴딩 생성">
    ```text theme={null}
    add tests for the notification service
    ```
  </Step>

  <Step title="의미 있는 테스트 케이스 추가">
    ```text theme={null}
    add test cases for edge conditions in the notification service
    ```
  </Step>

  <Step title="테스트 실행 및 검증">
    ```text theme={null}
    run the new tests and fix any failures
    ```
  </Step>
</Steps>

Claude는 프로젝트의 기존 패턴과 규칙을 따르는 테스트를 생성할 수 있습니다. 테스트를 요청할 때 검증하려는 동작에 대해 구체적으로 설명하십시오. Claude는 기존 테스트 파일을 검토하여 이미 사용 중인 스타일, 프레임워크 및 어설션 패턴을 일치시킵니다.

포괄적인 적용 범위를 위해 Claude에게 놓쳤을 수 있는 엣지 케이스를 식별하도록 요청하십시오. Claude는 코드 경로를 분석하고 오류 조건, 경계값 및 쉽게 간과할 수 있는 예상치 못한 입력에 대한 테스트를 제안할 수 있습니다.

***

<h3 id="create-pull-requests">
  풀 요청 만들기
</h3>

Claude에게 직접 풀 요청을 만들도록 요청하거나 ("create a pr for my changes"), 단계별로 Claude를 안내할 수 있습니다:

<Steps>
  <Step title="변경사항 요약하기">
    ```text theme={null}
    summarize the changes I've made to the authentication module
    ```
  </Step>

  <Step title="풀 요청 생성하기">
    ```text theme={null}
    create a pr
    ```
  </Step>

  <Step title="검토 및 정제하기">
    ```text theme={null}
    enhance the PR description with more context about the security improvements
    ```
  </Step>
</Steps>

`gh pr create`를 사용하여 PR을 만들면 세션이 자동으로 해당 PR에 연결됩니다. 나중에 `claude --from-pr 123`으로 재개하거나 (123을 PR 번호로 바꾸기), [`/resume` 선택기](/docs/ko/sessions#use-the-session-picker)에 PR URL을 붙여넣어 재개할 수 있습니다.

<Tip>
  Claude가 생성한 PR을 제출하기 전에 검토하고 Claude에게 잠재적 위험이나 고려사항을 강조하도록 요청하십시오.
</Tip>

<h3 id="handle-documentation">
  문서 처리하기
</h3>

코드에 대한 문서를 추가하거나 업데이트해야 한다고 가정해봅시다.

<Steps>
  <Step title="문서화되지 않은 코드 식별">
    ```text theme={null}
    find functions without proper JSDoc comments in the auth module
    ```
  </Step>

  <Step title="문서 생성하기">
    ```text theme={null}
    add JSDoc comments to the undocumented functions in auth.js
    ```
  </Step>

  <Step title="검토 및 개선하기">
    ```text theme={null}
    improve the generated documentation with more context and examples
    ```
  </Step>

  <Step title="문서 검증하기">
    ```text theme={null}
    check if the documentation follows our project standards
    ```
  </Step>
</Steps>

<Tip>
  팁:

  * 원하는 문서 스타일 지정하기 (JSDoc, docstring 등)
  * 문서에 예제 요청하기
  * 공개 API, 인터페이스 및 복잡한 로직에 대한 문서 요청하기
</Tip>

***

<h3 id="work-in-notes-and-non-code-folders">
  노트 및 비코드 폴더에서 작업하기
</h3>

Claude Code는 모든 디렉토리에서 작동합니다. 노트 저장소, 문서 폴더 또는 마크다운 파일의 모든 컬렉션 내에서 실행하여 코드처럼 콘텐츠를 검색, 편집 및 재구성합니다.

`.claude/` 디렉토리와 `CLAUDE.md`는 다른 도구의 구성 디렉토리와 충돌 없이 나란히 있습니다. Claude는 각 도구 호출에서 파일을 새로 읽으므로 다른 애플리케이션에서 만든 편집을 다음에 파일을 읽을 때 봅니다.

***

<h3 id="work-with-images">
  이미지 작업하기
</h3>

코드베이스에서 이미지를 작업해야 하고 Claude의 이미지 콘텐츠 분석 도움을 원한다고 가정해봅시다.

<Steps>
  <Step title="대화에 이미지 추가하기">
    다음 방법 중 하나를 사용할 수 있습니다:

    1. Claude Code 창으로 이미지를 드래그 앤 드롭하기
    2. 이미지를 복사하고 Ctrl+V로 CLI에 붙여넣기. macOS에서는 iTerm2에서도 Cmd+V가 작동합니다.
    3. Claude에 이미지 경로 제공하기. 예: "Analyze this image: /path/to/your/image.png"
  </Step>

  <Step title="Claude에게 이미지 분석 요청하기">
    ```text theme={null}
    What does this image show?
    ```

    ```text theme={null}
    Describe the UI elements in this screenshot
    ```

    ```text theme={null}
    Are there any problematic elements in this diagram?
    ```
  </Step>

  <Step title="컨텍스트를 위해 이미지 사용하기">
    ```text theme={null}
    Here's a screenshot of the error. What's causing it?
    ```

    ```text theme={null}
    This is our current database schema. How should we modify it for the new feature?
    ```
  </Step>

  <Step title="시각적 콘텐츠에서 코드 제안 얻기">
    ```text theme={null}
    Generate CSS to match this design mockup
    ```

    ```text theme={null}
    What HTML structure would recreate this component?
    ```
  </Step>
</Steps>

<Tip>
  팁:

  * 텍스트 설명이 불명확하거나 번거로울 때 이미지 사용하기
  * 더 나은 컨텍스트를 위해 오류, UI 디자인 또는 다이어그램의 스크린샷 포함하기
  * 대화에서 여러 이미지를 작업할 수 있습니다
  * 이미지 분석은 다이어그램, 스크린샷, 목업 등과 함께 작동합니다
  * Claude가 이미지를 참조할 때 (예: `[Image #1]`), `Cmd+Click` (Mac) 또는 `Ctrl+Click` (Windows/Linux)을 클릭하여 기본 뷰어에서 이미지를 엽니다
</Tip>

***

<h3 id="reference-files-and-directories">
  파일 및 디렉토리 참조하기
</h3>

@를 사용하여 Claude가 읽을 때까지 기다리지 않고 파일이나 디렉토리를 빠르게 포함합니다.

<Steps>
  <Step title="단일 파일 참조하기">
    ```text theme={null}
    Explain the logic in @src/utils/auth.js
    ```

    이것은 대화에 파일의 전체 내용을 포함합니다.
  </Step>

  <Step title="디렉토리 참조하기">
    ```text theme={null}
    What's the structure of @src/components?
    ```

    이것은 파일 정보가 있는 디렉토리 목록을 제공합니다.
  </Step>

  <Step title="MCP 리소스 참조하기">
    ```text theme={null}
    Show me the data from @github:repos/owner/repo/issues
    ```

    이것은 @server:resource 형식을 사용하여 연결된 MCP 서버에서 데이터를 가져옵니다. 자세한 내용은 [MCP 리소스](/docs/ko/mcp#use-mcp-resources)를 참조하십시오.
  </Step>
</Steps>

<Tip>
  팁:

  * 파일 경로는 상대 또는 절대 경로일 수 있습니다
  * @ 파일 참조는 파일의 디렉토리 및 상위 디렉토리에 `CLAUDE.md`를 추가합니다
  * 디렉토리 참조는 내용이 아닌 파일 목록을 표시합니다
  * 단일 메시지에서 여러 파일을 참조할 수 있습니다 (예: "@file1.js and @file2.js")
</Tip>

***

<h3 id="run-claude-on-a-schedule">
  일정에 따라 Claude 실행하기
</h3>

Claude가 정기적으로 작업을 자동으로 처리하도록 하고 싶다고 가정해봅시다. 예를 들어 매일 아침 열린 PR을 검토하거나, 매주 종속성을 감사하거나, 밤새 CI 실패를 확인합니다.

작업을 실행할 위치에 따라 일정 옵션을 선택합니다:

| 옵션                                        | 실행 위치            | 최적 사용                                                                                                                                     |
| :---------------------------------------- | :--------------- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| [Routines](/docs/ko/routines)                  | Anthropic 관리 인프라 | 컴퓨터가 꺼져 있어도 실행되어야 하는 작업. API 호출 또는 GitHub 이벤트 외에도 일정에 따라 트리거될 수 있습니다. [claude.ai/code/routines](https://claude.ai/code/routines)에서 구성합니다. |
| [데스크톱 예약 작업](/docs/ko/desktop-scheduled-tasks) | 데스크톱 앱을 통한 컴퓨터   | 로컬 파일, 도구 또는 커밋되지 않은 변경사항에 직접 액세스해야 하는 작업.                                                                                                |
| [GitHub Actions](/docs/ko/github-actions)      | CI 파이프라인         | 열린 PR 또는 cron 일정과 같은 저장소 이벤트와 연결된 작업. 워크플로우 구성과 함께 있어야 합니다.                                                                               |
| [`/loop`](/docs/ko/scheduled-tasks)            | 현재 CLI 세션        | 세션이 열려 있는 동안 빠른 폴링. 새 대화를 시작하면 작업이 중지됩니다. `--resume` 및 `--continue`는 만료되지 않은 것을 복원합니다.                                                    |

<Tip>
  예약된 작업을 위한 프롬프트를 작성할 때 성공이 무엇인지, 결과를 어떻게 처리할지 명시적으로 설명하십시오. 작업이 자동으로 실행되므로 명확한 질문을 할 수 없습니다. 예를 들어: "Review open PRs labeled `needs-review`, leave inline comments on any issues, and post a summary in the `#eng-reviews` Slack channel."
</Tip>

***

<h3 id="ask-claude-about-its-capabilities">
  Claude의 기능에 대해 Claude에게 물어보기
</h3>

Claude는 자신의 문서에 대한 기본 제공 액세스 권한을 가지고 있으며 자신의 기능과 제한사항에 대한 질문에 답할 수 있습니다.

<h4 id="example-questions">
  예제 질문
</h4>

```text theme={null}
can Claude Code create pull requests?
```

```text theme={null}
how does Claude Code handle permissions?
```

```text theme={null}
what skills are available?
```

```text theme={null}
how do I use MCP with Claude Code?
```

```text theme={null}
how do I configure Claude Code for Amazon Bedrock?
```

```text theme={null}
what are the limitations of Claude Code?
```

<Note>
  Claude는 이러한 질문에 대해 문서 기반 답변을 제공합니다. 실행 가능한 예제 및 실습 시연을 위해 `/powerup`을 실행하여 애니메이션 데모가 있는 대화형 수업을 받거나 위의 특정 워크플로우 섹션을 참조하십시오.
</Note>

<Tip>
  팁:

  * Claude는 사용 중인 버전에 관계없이 항상 최신 Claude Code 문서에 액세스할 수 있습니다
  * 자세한 답변을 얻으려면 구체적인 질문을 하십시오
  * Claude는 MCP 통합, 엔터프라이즈 구성 및 고급 워크플로우와 같은 복잡한 기능을 설명할 수 있습니다
</Tip>

***

<h2 id="resume-previous-conversations">
  이전 대화 재개하기
</h2>

작업이 여러 세션에 걸쳐 진행될 때 컨텍스트를 다시 설명하는 대신 중단한 부분부터 계속합니다. Claude Code는 모든 대화를 로컬에 저장합니다.

```bash theme={null}
claude --continue
```

이것은 현재 디렉토리에서 가장 최근 세션을 재개합니다. 아직 없으면 `No conversation found to continue`를 출력하고 종료합니다. `claude --resume`을 사용하여 목록에서 선택하거나 실행 중인 세션 내에서 `/resume`을 사용합니다. [세션 관리](/docs/ko/sessions)에서 이름 지정, 분기 및 전체 선택기 참조를 참조하십시오.

<h2 id="run-parallel-sessions-with-worktrees">
  worktree를 사용하여 병렬 세션 실행하기
</h2>

한 터미널에서 기능을 작업하는 동안 Claude가 다른 터미널에서 버그를 수정하며, 편집이 충돌하지 않습니다. 각 worktree는 자체 분기의 별도 체크아웃입니다.

```bash theme={null}
claude --worktree feature-auth
```

다른 이름으로 두 번째 터미널에서 동일한 명령을 실행하여 격리된 병렬 세션을 시작합니다. [Worktrees](/docs/ko/worktrees)에서 정리, `.worktreeinclude` 및 비git VCS 지원을 참조하십시오. 별도의 터미널 대신 한 화면에서 병렬 세션을 모니터링하려면 [백그라운드 에이전트](/docs/ko/agent-view)를 참조하십시오.

<h2 id="plan-before-editing">
  편집 전에 계획하기
</h2>

변경사항이 디스크에 닿기 전에 검토하려는 경우 계획 모드로 전환합니다. Claude는 파일을 읽고 계획을 제안하지만 승인할 때까지 편집하지 않습니다.

```bash theme={null}
claude --permission-mode plan
```

세션 중에 `Shift+Tab`을 눌러 계획 모드로 전환할 수도 있습니다. [계획 모드](/docs/ko/permission-modes#analyze-before-you-edit-with-plan-mode)에서 승인 흐름 및 텍스트 편집기에서 계획 편집을 참조하십시오.

<h2 id="delegate-research-to-subagents">
  subagent에게 연구 위임하기
</h2>

큰 코드베이스를 탐색하면 컨텍스트가 파일 읽기로 채워집니다. 탐색을 위임하여 결과만 돌아오도록 합니다.

```text theme={null}
use a subagent to investigate how our auth system handles token refresh
```

subagent는 자체 컨텍스트 윈도우에서 파일을 읽고 요약을 보고합니다. [Subagents](/docs/ko/sub-agents)에서 자체 도구 및 프롬프트가 있는 사용자 정의 에이전트 정의를 참조하십시오.

<h2 id="pipe-claude-into-scripts">
  Claude를 스크립트로 파이프하기
</h2>

CI, 사전 커밋 훅 또는 배치 처리를 위해 Claude를 비대화형으로 실행합니다. stdin 및 stdout은 모든 Unix 도구처럼 작동합니다.

```bash theme={null}
git log --oneline -20 | claude -p "summarize these recent commits"
```

[비대화형 모드](/docs/ko/headless)에서 출력 형식, 권한 플래그 및 팬아웃 패턴을 참조하십시오.

<h2 id="next-steps">
  다음 단계
</h2>

<CardGroup cols={2}>
  <Card title="모범 사례" icon="lightbulb" href="/docs/ko/best-practices">
    Claude Code에서 최대한 활용하기 위한 패턴
  </Card>

  <Card title="세션 관리" icon="rotate-left" href="/docs/ko/sessions">
    대화 재개, 이름 지정 및 분기
  </Card>

  <Card title="Worktrees" icon="code-branch" href="/docs/ko/worktrees">
    격리된 병렬 세션 실행
  </Card>

  <Card title="Claude Code 확장하기" icon="puzzle-piece" href="/docs/ko/features-overview">
    skill, hook, MCP, subagent 및 플러그인 추가
  </Card>
</CardGroup>
