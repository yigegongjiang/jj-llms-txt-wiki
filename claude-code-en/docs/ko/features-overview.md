> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code 확장하기

> CLAUDE.md, Skills, subagents, hooks, MCP, 플러그인을 언제 사용할지 이해합니다.

Claude Code는 코드를 추론하는 모델과 파일 작업, 검색, 실행 및 웹 접근을 위한 [내장 도구](/docs/ko/how-claude-code-works#tools)를 결합합니다. 내장 도구는 대부분의 코딩 작업을 다룹니다. 이 가이드는 확장 계층을 다룹니다. Claude가 알아야 할 내용을 사용자 정의하고, 외부 서비스에 연결하고, 워크플로우를 자동화하기 위해 추가하는 기능입니다.

<Note>
  핵심 에이전트 루프가 어떻게 작동하는지 알아보려면 [Claude Code 작동 방식](/docs/ko/how-claude-code-works)을 참조하세요.
</Note>

**Claude Code를 처음 사용하시나요?** 프로젝트 규칙을 위해 [CLAUDE.md](/docs/ko/memory)로 시작하세요. 그런 다음 [특정 트리거가 발생할 때](#build-your-setup-over-time) 다른 확장을 추가하세요.

<h2 id="overview">
  개요
</h2>

확장은 에이전트 루프의 다양한 부분에 연결됩니다:

* \*\*[CLAUDE.md](/docs/ko/memory)\*\*는 Claude가 모든 세션에서 보는 지속적인 컨텍스트를 추가합니다.
* \*\*[출력 스타일](/docs/ko/output-styles)\*\*은 세션의 모든 응답에 대해 Claude의 역할, 톤 및 응답 형식을 설정합니다.
* \*\*[Skills](/docs/ko/skills)\*\*는 재사용 가능한 지식과 호출 가능한 워크플로우를 추가합니다.
* \*\*[Code intelligence](/docs/ko/tools-reference#lsp-tool-behavior)\*\*는 Claude를 언어 서버에 연결하여 기호 수준의 네비게이션 및 실시간 타입 오류를 제공합니다.
* \*\*[MCP](/docs/ko/mcp)\*\*는 Claude를 외부 서비스 및 도구에 연결합니다.
* \*\*[Subagents](/docs/ko/sub-agents)\*\*는 격리된 컨텍스트에서 자신의 루프를 실행하고 요약을 반환합니다.
* \*\*[Dynamic workflows](/docs/ko/workflows)\*\*는 Claude가 작성한 스크립트에서 많은 subagents를 실행하고 하나의 결과를 반환합니다.
* \*\*[Cross-session messaging](/docs/ko/cross-session-messaging)\*\*을 통해 Claude는 한 세션에서 다른 세션으로 메시지를 전달할 수 있습니다.
* \*\*[Hooks](/docs/ko/hooks-guide)\*\*는 Claude Code가 라이프사이클 이벤트에 도달할 때 스크립트, HTTP 요청, MCP 도구 호출, 프롬프트 또는 subagent를 실행합니다.
* **[Plugins](/docs/ko/plugins/overview)** 및 \*\*[marketplaces](/docs/ko/plugins/overview)\*\*는 이러한 기능을 패키징하고 배포합니다.

[Skills](/docs/ko/skills)는 가장 유연한 확장입니다. Skill은 지식, 워크플로우 또는 지침을 포함하는 마크다운 파일입니다. `/deploy`와 같은 명령으로 skill을 호출하거나, Claude가 관련이 있을 때 자동으로 로드할 수 있습니다. Skill은 현재 대화에서 실행되거나 subagents를 통해 격리된 컨텍스트에서 실행될 수 있습니다.

<h2 id="match-features-to-your-goal">
  목표에 맞는 기능 선택하기
</h2>

기능은 Claude가 매 세션마다 보는 항상 활성화된 컨텍스트부터, 사용자나 Claude가 필요할 때 호출할 수 있는 온디맨드 기능, 특정 이벤트에서 실행되는 백그라운드 자동화까지 다양합니다. 아래 표는 사용 가능한 기능과 각 기능을 사용해야 할 시점을 보여줍니다.

| 기능                                                             | 역할                                                    | 사용 시점                                                      | 예시                                                          |
| -------------------------------------------------------------- | ----------------------------------------------------- | ---------------------------------------------------------- | ----------------------------------------------------------- |
| **CLAUDE.md**                                                  | 매 대화마다 로드되는 지속적인 컨텍스트                                 | 프로젝트 규칙, "항상 X를 하기" 규칙                                     | "npm이 아닌 pnpm을 사용하세요. 커밋하기 전에 테스트를 실행하세요."                  |
| **[출력 스타일](/docs/ko/output-styles)**                                | Claude의 역할, 톤, 전체 세션의 응답 형식을 설정하는 지침                  | 모든 응답에서 원하는 음성, 길이 또는 형식, 또는 Claude가 소프트웨어 엔지니어 이외의 역할로 작동 | 더 짧은 응답을 위한 기본 제공 간결 스타일; 모든 질문에 다이어그램으로 먼저 답변하는 사용자 정의 스타일 |
| **Skill**                                                      | Claude가 사용할 수 있는 지침, 지식, 워크플로우                        | 재사용 가능한 콘텐츠, 참고 문서, 반복 가능한 작업                              | `/deploy`는 배포 체크리스트를 실행합니다. 엔드포인트 패턴이 있는 API 문서 skill       |
| **Subagent**                                                   | 요약된 결과를 반환하는 격리된 실행 컨텍스트                              | 컨텍스트 격리, 병렬 작업, 전문화된 워커                                    | 많은 파일을 읽지만 주요 결과만 반환하는 연구 작업                                |
| **[Dynamic workflow](/docs/ko/workflows)**                          | Claude가 작성하여 백그라운드에서 많은 subagent를 실행하는 스크립트           | 소수의 subagent를 초과하는 작업, 또는 교차 검증하려는 결과                      | 전체 코드베이스를 감사하고, 두 번째 에이전트 세트가 각 결과를 검증                      |
| **[Cross-session messaging](/docs/ko/cross-session-messaging)**     | Claude가 한 세션의 메시지를 다른 세션으로 전달                         | 작업 중간에 서로의 결과가 필요한 직접 실행하는 세션                              | 한 세션이 다른 세션에 자신이 만든 변경 사항이 다른 세션이 구축 중인 것을 깨뜨린다고 경고         |
| **[Code intelligence](/docs/ko/tools-reference#lsp-tool-behavior)** | 언어 서버 네비게이션 및 진단                                      | 타입이 지정된 언어, grep이 느리거나 부정확한 대규모 코드베이스                      | 전체 파일을 읽는 대신 기호의 정의로 이동                                     |
| **MCP**                                                        | 외부 서비스에 연결                                            | 외부 데이터 또는 작업                                               | 데이터베이스 쿼리, Slack에 게시, 브라우저 제어                               |
| **Hook**                                                       | 이벤트로 트리거되는 스크립트, HTTP 요청, MCP 도구 호출, 프롬프트 또는 subagent | 일치하는 모든 이벤트에서 실행되어야 하는 자동화                                 | 모든 파일 편집 후 ESLint 실행                                        |
| **[Artifact](/docs/ko/artifacts)**                                  | 세션 출력을 비공개 대화형 웹 페이지로 게시                              | 터미널 텍스트가 아닌 시각적으로 보거나 공유하려는 출력                             | Claude가 조사할 때 업데이트되는 인시던트 타임라인                              |

\*\*[Plugins](/docs/ko/plugins/overview)\*\*는 패키징 계층입니다. 플러그인은 skill, hook, subagent, MCP 서버를 단일 설치 가능한 단위로 번들합니다. 플러그인 skill은 네임스페이스됩니다(`/my-plugin:review` 같은). 여러 플러그인이 공존할 수 있습니다. 여러 저장소에서 동일한 설정을 재사용하거나 \*\*[마켓플레이스](/docs/ko/plugins/overview)\*\*를 통해 다른 사용자에게 배포하려면 플러그인을 사용하세요.

<h3 id="build-your-setup-over-time">
  시간에 따라 설정 구축하기
</h3>

모든 것을 미리 구성할 필요는 없습니다. 각 기능에는 인식 가능한 트리거가 있으며, 대부분의 팀은 대략 이 순서로 추가합니다:

| 트리거                                               | 추가                                                     |
| :------------------------------------------------ | :----------------------------------------------------- |
| Claude가 규칙이나 명령을 두 번 잘못 실행                        | [CLAUDE.md](/docs/ko/memory)에 추가                            |
| 계속해서 Claude에게 더 짧게, 더 많이 설명하거나, 동일한 형식으로 답변하도록 요청 | [출력 스타일](/docs/ko/output-styles) 설정                         |
| 작업을 시작하기 위해 동일한 프롬프트를 계속 입력                       | 사용자가 호출 가능한 [skill](/docs/ko/skills)로 저장                    |
| 동일한 플레이북이나 다단계 절차를 세 번째로 채팅에 붙여넣기                 | [skill](/docs/ko/skills)로 캡처                                |
| Claude가 볼 수 없는 브라우저 탭에서 계속 데이터 복사                 | 해당 시스템을 [MCP 서버](/docs/ko/mcp)로 연결                          |
| Claude가 기호가 정의되거나 사용되는 위치를 찾기 위해 많은 파일 읽기         | 언어용 [코드 인텔리전스 플러그인](/docs/ko/plugins/code-intelligence)을 설치 |
| 부작용 작업이 다시 참조하지 않을 출력으로 대화를 채우기                   | [subagent](/docs/ko/sub-agents)를 통해 라우팅                     |
| 요청하지 않고 매번 무언가가 발생하기를 원함                          | [hook](/docs/ko/hooks-guide) 작성                             |
| 두 번째 저장소에 동일한 설정이 필요                              | [플러그인](/docs/ko/plugins/overview)으로 패키징                     |

동일한 트리거는 이미 있는 것을 업데이트할 시점을 알려줍니다. 반복된 실수나 반복되는 검토 의견은 채팅의 일회성 수정이 아닌 CLAUDE.md 편집입니다. 손으로 계속 조정하는 워크플로우는 다른 수정이 필요한 skill입니다.

<h3 id="compare-similar-features">
  유사한 기능 비교하기
</h3>

일부 기능은 유사해 보일 수 있습니다. 기능 선택에 대한 더 깊은 설명은 블로그의 [Steering Claude Code: when to use CLAUDE.md, skills, hooks, and subagents](https://claude.com/blog/steering-claude-code-skills-hooks-rules-subagents-and-more)를 참조하세요. 이들을 구분하는 방법은 다음과 같습니다.

<Tabs>
  <Tab title="Skill vs Subagent">
    Skill과 subagent는 다른 문제를 해결합니다:

    * **Skill**은 모든 컨텍스트에 로드할 수 있는 재사용 가능한 콘텐츠입니다
    * **Subagent**는 주 대화와 별도로 실행되는 격리된 워커입니다

    | 측면                                          | Skill                   | Subagent                     |
    | ------------------------------------------- | ----------------------- | ---------------------------- |
    | **정의**                                      | 재사용 가능한 지침, 지식 또는 워크플로우 | 자체 컨텍스트가 있는 격리된 워커           |
    | **주요 이점**                                   | 컨텍스트 간 콘텐츠 공유           | 컨텍스트 격리. 작업은 별도로 실행되고 요약만 반환 |
    | **[Context window](/docs/ko/context-window) 영향** | 주 윈도우에 추가               | 자체 입력 및 출력 토큰이 있는 별도 윈도우 사용  |
    | **최적 용도**                                   | 참고 자료, 호출 가능한 워크플로우     | 많은 파일을 읽는 작업, 병렬 작업, 전문화된 워커 |

    **Skill은 참고 또는 작업일 수 있습니다.** 참고 skill은 세션 전체에서 Claude가 사용하는 지식을 제공합니다(API 스타일 가이드 같은). 작업 skill은 Claude에게 특정 작업을 수행하도록 지시합니다(`/deploy`는 배포 워크플로우를 실행).

    **컨텍스트 격리가 필요하거나 컨텍스트 윈도우가 가득 찰 때 subagent를 사용하세요.** Subagent는 수십 개의 파일을 읽거나 광범위한 검색을 실행할 수 있지만, 주 대화는 요약만 받습니다. Subagent 작업이 주 컨텍스트를 소비하지 않으므로, 중간 작업이 표시된 상태로 유지될 필요가 없을 때도 유용합니다. 사용자 정의 subagent는 자체 지침을 가질 수 있고 skill을 미리 로드할 수 있습니다.

    **이들은 결합할 수 있습니다.** Subagent는 특정 skill을 미리 로드할 수 있습니다(`skills:` 필드). Skill은 `context: fork`를 사용하여 격리된 컨텍스트에서 실행할 수 있습니다. 자세한 내용은 [Skills](/docs/ko/skills)를 참조하세요.
  </Tab>

  <Tab title="CLAUDE.md vs Skill">
    둘 다 지침을 저장하지만, 로드 방식과 목적이 다릅니다.

    | 측면               | CLAUDE.md         | Skill               |
    | ---------------- | ----------------- | ------------------- |
    | **로드**           | 매 세션마다 자동으로       | 온디맨드                |
    | **파일 포함 가능**     | 예, `@path` 임포트 사용 | 예, `@path` 임포트 사용   |
    | **워크플로우 트리거 가능** | 아니오               | 예, `/<name>` 사용     |
    | **최적 용도**        | "항상 X를 하기" 규칙     | 참고 자료, 호출 가능한 워크플로우 |

    **Claude가 항상 알아야 할 경우 CLAUDE.md에 넣으세요:** 코딩 규칙, 빌드 명령, 프로젝트 구조, "X를 하지 마세요" 규칙.

    **Claude가 때때로 필요한 참고 자료이거나 `/<name>`으로 트리거하는 워크플로우인 경우 skill에 넣으세요**(배포, 검토, 릴리스).

    **경험 법칙:** CLAUDE.md를 200줄 이하로 유지하세요. 증가하면 참고 콘텐츠를 skill로 이동하거나 [`.claude/rules/`](/docs/ko/memory#organize-rules-with-claude/rules/) 파일로 분할하세요.
  </Tab>

  <Tab title="CLAUDE.md vs 출력 스타일">
    둘 다 Claude에게 상시 지침을 제공합니다. CLAUDE.md는 Claude가 알아야 할 내용을 담고, 출력 스타일은 Claude가 응답하는 방식을 설정합니다.

    | 측면        | CLAUDE.md                 | 출력 스타일                                                                    |
    | --------- | ------------------------- | ------------------------------------------------------------------------- |
    | **보유**    | 프로젝트에 대한 사실 및 규칙          | 역할, 톤, 응답 형식                                                              |
    | **전환**    | 항상 로드됨                    | 한 번에 하나 활성화; [스타일 전환](/docs/ko/output-styles#change-your-output-style) 언제든지 가능 |
    | **최적 용도** | 빌드 명령, 규칙, "X를 하지 마세요" 규칙 | 더 짧은 응답, 코드와 함께 설명, 쓰기 보조자 같은 비엔지니어링 역할                                   |

    **프로젝트에 대해 어떤 스타일이든 참인 경우 CLAUDE.md에 넣으세요:** 코딩 규칙, 빌드 명령, 프로젝트 구조.

    **응답 자체에 관한 것이고 다시 끄고 싶을 수 있는 경우 출력 스타일을 사용하세요:** 길이, 형식, Claude가 설명하는 정도, 또는 쓰기 보조자 같은 다른 역할. Claude Code는 [기본 제공 스타일](/docs/ko/output-styles#built-in-output-styles)을 포함하며, 자신만의 스타일을 작성할 수 있습니다.

    **이들은 결합됩니다.** CLAUDE.md는 선택한 스타일과 관계없이 로드된 상태로 유지됩니다. Claude는 둘 다 지침으로 따르므로 어느 것도 강제되지 않습니다. 매번 발생해야 하는 모든 것에 대해 [hook](/docs/ko/hooks-guide)을 사용하세요.
  </Tab>

  <Tab title="CLAUDE.md vs 규칙 vs Skill">
    세 가지 모두 지침을 저장하지만, 로드 방식이 다릅니다:

    | 측면        | CLAUDE.md     | `.claude/rules/`         | Skill                |
    | --------- | ------------- | ------------------------ | -------------------- |
    | **로드**    | 매 세션마다        | 매 세션마다, 또는 일치하는 파일이 열릴 때 | 온디맨드, 호출되거나 관련이 있을 때 |
    | **범위**    | 전체 프로젝트       | 파일 경로로 범위 지정 가능          | 작업별                  |
    | **최적 용도** | 핵심 규칙 및 빌드 명령 | 언어별 또는 디렉토리별 가이드라인       | 참고 자료, 반복 가능한 워크플로우  |

    **모든 세션이 필요한 지침에 CLAUDE.md를 사용하세요:** 빌드 명령, 테스트 규칙, 프로젝트 아키텍처.

    **CLAUDE.md를 집중하게 유지하려면 규칙을 사용하세요.** [`paths` frontmatter](/docs/ko/memory#path-specific-rules)가 있는 규칙은 Claude가 일치하는 파일로 작업할 때만 로드되어 컨텍스트를 절약합니다.

    **Claude가 때때로만 필요한 콘텐츠에 skill을 사용하세요**, API 문서나 `/<name>`으로 트리거하는 배포 체크리스트 같은.
  </Tab>

  <Tab title="Subagent vs Dynamic workflow">
    둘 다 주 대화 외부에서 작업을 수행합니다. Subagent에서는 Claude가 차례대로 다음에 실행할 것을 결정합니다. 워크플로우에서는 스크립트가 결정합니다:

    * **Subagent**는 Claude가 생성하는 워커이며, 각각 대화를 생성한 대화에 요약을 반환합니다
    * \*\*[Dynamic workflow](/docs/ko/workflows)\*\*는 Claude가 작성하는 스크립트로 백그라운드에서 많은 subagent를 실행하고 하나의 결과를 반환합니다

    **빠르고 집중된 워커가 필요할 때 subagent를 사용하세요:** 질문 조사, 주장 검증, 파일 검토. Subagent는 작업을 수행하고 요약을 반환하므로 주 대화가 깔끔하게 유지됩니다. Claude가 생성할 때 이름을 지정한 Subagent는 [서로 메시지를 보낼](/docs/ko/sub-agents#what-loads-at-startup) 수도 있습니다.

    **작업이 [소수의 subagent를 초과](/docs/ko/workflows#when-to-use-a-workflow)하거나 결과를 보기 전에 교차 검증하려는 경우 dynamic workflow를 사용하세요**, 코드베이스 전체 감사, 대규모 마이그레이션, 여러 각도에서 작성된 계획 같은. 시작하려면 [프롬프트에서 워크플로우를 요청하세요](/docs/ko/workflows#ask-for-a-workflow-in-your-prompt).

    **한 세션의 결과를 다른 세션으로 전달하려면**, 첫 번째 세션의 Claude에게 전송하도록 요청하세요. Claude는 [cross-session messaging](/docs/ko/cross-session-messaging)으로 전달합니다. [Run agents in parallel](/docs/ko/agents)은 한 번에 여러 Claude를 실행하는 다른 방법을 비교합니다. 나중에 확인하기 위해 넘기는 세션 포함.
  </Tab>

  <Tab title="MCP vs Skill">
    MCP는 Claude를 외부 서비스에 연결합니다. Skill은 Claude가 알고 있는 것을 확장하며, 이러한 서비스를 효과적으로 사용하는 방법도 포함합니다.

    | 측면     | MCP                          | Skill                              |
    | ------ | ---------------------------- | ---------------------------------- |
    | **정의** | 외부 서비스에 연결하기 위한 프로토콜         | 지식, 워크플로우, 참고 자료                   |
    | **제공** | 도구 및 데이터 액세스                 | 지식, 워크플로우, 참고 자료                   |
    | **예시** | Slack 통합, 데이터베이스 쿼리, 브라우저 제어 | 코드 검토 체크리스트, 배포 워크플로우, API 스타일 가이드 |

    이들은 다른 문제를 해결하며 함께 잘 작동합니다:

    **MCP**는 Claude에게 외부 시스템을 위한 목적 구축 도구를 제공하며, 연결과 인증은 서버에서 처리합니다.

    **Skill**은 Claude에게 이러한 도구를 효과적으로 사용하는 방법에 대한 지식을 제공하며, `/<name>`으로 트리거할 수 있는 워크플로우도 포함합니다. Skill은 팀의 데이터베이스 스키마 및 쿼리 패턴, 또는 팀의 메시지 형식 규칙이 있는 `/post-to-slack` 워크플로우를 포함할 수 있습니다.
  </Tab>

  <Tab title="Hook vs Skill">
    Claude Code는 라이프사이클 이벤트에서 hook을 실행합니다. Skill을 컨텍스트에 로드하여 Claude가 적용하도록 합니다.

    | 측면          | Hook                                                                   | Skill                                    |
    | ----------- | ---------------------------------------------------------------------- | ---------------------------------------- |
    | **실행**      | 셸 명령, HTTP 요청, MCP 도구 호출, LLM 프롬프트 또는 subagent                         | Claude가 읽고 따르는 지침                        |
    | **트리거**     | [라이프사이클 이벤트](/docs/ko/hooks#hook-events) 예: `PostToolUse` 또는 `SessionStart` | `/<name>`을 입력하거나, Claude가 설명을 작업과 일치시킬 때 |
    | **결정론**     | 항상 이벤트에서 실행됩니다. 트리거가 보장됩니다                                             | Claude가 지침을 해석합니다. 결과는 다를 수 있습니다         |
    | **컨텍스트 비용** | Hook이 출력을 반환하지 않으면 0                                                   | 설명은 매 세션마다 로드됩니다. 전체 콘텐츠는 사용할 때 로드됩니다    |
    | **최적 용도**   | 편집 후 린팅, 안전하지 않은 명령 차단, 로깅, 알림                                         | 추론이 필요한 워크플로우, 참고 자료, 다단계 작업             |

    **작업이 항상 동일한 방식으로 발생해야 하고 Claude가 생각할 필요가 없을 때 hook을 사용하세요.** 예: 저장 시 형식 지정, `rm -rf /` 거부, 세션이 끝날 때 Slack 메시지 게시.

    **Claude가 단계를 적용하는 방법을 결정해야 하거나 콘텐츠가 스크립트가 아닌 지식일 때 skill을 사용하세요.** 예: `/release` 체크리스트, API 스타일 가이드, 디버깅 플레이북.

    **Hook에 가드레일을 넣으세요.** CLAUDE.md나 skill의 "`.env`를 편집하지 마세요"와 같은 지침은 요청이지 보장이 아닙니다. 편집을 차단하는 `PreToolUse` hook은 강제입니다. 규칙이 매번 유지되어야 하면, 프롬프트 지침이 아닌 hook으로 만드세요.

    **Hook 출력은 컨텍스트에 들어갑니다.** 린터를 실행하는 `PostToolUse` hook은 결과를 Claude가 읽는 텍스트로 피드백합니다. `/fix-lint` skill은 Claude에게 이를 해결하는 방법을 알려줍니다.
  </Tab>
</Tabs>

<h3 id="understand-how-features-layer">
  기능이 어떻게 계층화되는지 이해하기
</h3>

기능은 여러 수준에서 정의할 수 있습니다: 사용자 전체, 프로젝트별, 플러그인을 통해, 또는 관리 정책을 통해. CLAUDE.md 파일을 하위 디렉토리에 중첩하거나 monorepo의 특정 패키지에 skill을 배치할 수도 있습니다. 동일한 기능이 여러 수준에 존재할 때, 다음과 같이 계층화됩니다:

* **CLAUDE.md 파일**은 가산적입니다: 모든 수준의 콘텐츠가 동시에 Claude의 컨텍스트에 기여합니다. 작업 디렉토리 및 위의 파일은 시작 시 로드됩니다. 하위 디렉토리는 작업할 때 로드됩니다. 지침이 충돌할 때, Claude는 판단을 사용하여 조정합니다. [CLAUDE.md 파일이 로드되는 방식](/docs/ko/memory#how-claude-md-files-load)을 참조하세요.
* **Skill과 subagent**는 이름으로 재정의됩니다: 동일한 이름이 여러 수준에 존재할 때, 우선순위에 따라 하나의 정의가 승리합니다(skill의 경우 관리 > 사용자 > 프로젝트; subagent의 경우 관리 > CLI 플래그 > 프로젝트 > 사용자 > 플러그인). 플러그인 skill은 [네임스페이스됩니다](/docs/ko/plugins/components#skills) 충돌을 피하기 위해. [Skill 발견](/docs/ko/skills#resolve-skills-that-share-a-name) 및 [Subagent 범위](/docs/ko/sub-agents#choose-the-subagent-scope)를 참조하세요.
* **MCP 서버**는 이름으로 재정의됩니다: 로컬 > 프로젝트 > 사용자. [MCP 범위](/docs/ko/mcp#scope-hierarchy-and-precedence)를 참조하세요.
* **Hook**은 병합됩니다: 등록된 모든 hook은 소스와 관계없이 일치하는 이벤트에 대해 실행됩니다. [Hook](/docs/ko/hooks)을 참조하세요.

<h3 id="combine-features">
  기능 결합하기
</h3>

각 확장은 다른 문제를 해결합니다: CLAUDE.md는 항상 활성화된 컨텍스트를 처리하고, skill은 온디맨드 지식과 워크플로우를 처리하고, MCP는 외부 연결을 처리하고, subagent는 격리를 처리하고, hook은 자동화를 처리합니다. 실제 설정은 워크플로우에 따라 이들을 결합합니다.

예를 들어, CLAUDE.md를 프로젝트 규칙에 사용하고, skill을 배포 워크플로우에 사용하고, MCP를 데이터베이스에 연결하는 데 사용하고, hook을 모든 편집 후 린팅을 실행하는 데 사용할 수 있습니다. 각 기능은 최고의 역할을 합니다.

| 패턴                    | 작동 방식                                                        | 예시                                                             |
| --------------------- | ------------------------------------------------------------ | -------------------------------------------------------------- |
| **Skill + MCP**       | MCP가 연결을 제공합니다. Skill은 Claude에게 이를 잘 사용하는 방법을 가르칩니다          | MCP가 데이터베이스에 연결합니다. Skill은 스키마 및 쿼리 패턴을 문서화합니다                 |
| **Skill + Subagent**  | Skill이 병렬 작업을 위해 subagent를 생성합니다                             | `/audit` skill은 격리된 컨텍스트에서 작동하는 보안, 성능, 스타일 subagent를 시작합니다    |
| **CLAUDE.md + Skill** | CLAUDE.md는 항상 활성화된 규칙을 보유합니다. Skill은 온디맨드로 로드되는 참고 자료를 보유합니다 | CLAUDE.md는 "API 규칙을 따르세요"라고 말합니다. Skill은 전체 API 스타일 가이드를 포함합니다 |
| **Hook + MCP**        | Hook은 MCP를 통해 외부 작업을 트리거합니다                                  | 편집 후 hook은 Claude가 중요한 파일을 수정할 때 Slack 알림을 게시합니다               |

<h2 id="understand-context-costs">
  컨텍스트 비용 이해하기
</h2>

추가하는 모든 기능은 Claude의 컨텍스트를 소비합니다. 너무 많으면 컨텍스트 윈도우를 채울 수 있지만, 노이즈를 추가하여 Claude를 덜 효과적으로 만들 수도 있습니다. Skill이 올바르게 트리거되지 않거나 Claude가 규칙을 잃을 수 있습니다. 이러한 트레이드오프를 이해하면 효과적인 설정을 구축하는 데 도움이 됩니다. 실행 중인 세션에서 이러한 기능이 어떻게 결합되는지 대화형으로 보려면 [컨텍스트 윈도우 탐색](/docs/ko/context-window)을 참조하세요.

<h3 id="context-cost-by-feature">
  기능별 컨텍스트 비용
</h3>

각 기능은 다양한 로딩 전략과 컨텍스트 비용을 가집니다.

| 기능                    | 로드 시기                    | 로드되는 내용                                                                              | 컨텍스트 비용                     |
| --------------------- | ------------------------ | ------------------------------------------------------------------------------------ | --------------------------- |
| **CLAUDE.md**         | 세션 시작                    | 전체 콘텐츠                                                                               | 모든 요청                       |
| **출력 스타일**            | 세션 시작, 그리고 스타일을 전환할 때 다시 | 활성 스타일의 전체 지침; 기본 스타일의 경우 없음                                                         | 모든 요청                       |
| **Skills**            | 세션 시작 + 사용 시             | 시작 시 설명, 사용 시 전체 콘텐츠                                                                 | 낮음(모든 요청마다 설명)\*            |
| **MCP 서버**            | 세션 시작                    | 도구 이름; 필요 시 전체 스키마                                                                   | 도구 사용 시까지 낮음                |
| **Code intelligence** | 파일 편집 후 및 온디맨드           | 각 파일 편집 후 진단; 기호 조회 시 정의, 참조 및 유형 정보                                                 | 낮음; 다른 곳에서 파일 읽기 감소         |
| **Subagents**         | 생성 시                     | 지정된 skill이 있는 신선한 컨텍스트, 또는 [포크](/docs/ko/sub-agents#fork-the-current-conversation)의 부모 대화 | 주 세션에서 격리됨                  |
| **Hooks**             | 트리거 시                    | 없음(외부에서 실행)                                                                          | 0, hook이 추가 컨텍스트를 반환하지 않는 한 |

\*기본적으로 skill 설명은 세션 시작 시 로드되므로 Claude가 사용할 시기를 결정할 수 있습니다. Skill의 frontmatter에서 `disable-model-invocation: true`를 설정하여 수동으로 호출할 때까지 Claude에서 완전히 숨깁니다. 작성하지 않은 skill의 경우, 파일을 편집하지 않고도 동일한 작업을 수행하도록 설정에서 [`skillOverrides`](/docs/ko/skills#override-skill-visibility-from-settings)를 설정하세요.

<h3 id="understand-how-features-load">
  기능이 어떻게 로드되는지 이해하기
</h3>

각 기능은 세션의 다양한 지점에서 로드됩니다. 아래 탭은 각 기능이 언제 로드되고 무엇이 컨텍스트에 들어가는지 설명합니다.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/context-loading.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=aab139e750494a237ae2e0c8f9139b0a" className="dark:hidden" alt="컨텍스트 로딩: CLAUDE.md는 세션 시작 시 로드되고 모든 요청에 유지됩니다. MCP 도구 이름은 시작 시 로드되고 전체 스키마는 사용 시까지 연기됩니다. Skill은 시작 시 설명을 로드하고 호출 시 전체 콘텐츠를 로드합니다. Subagent는 격리된 컨텍스트를 받습니다. Hook은 외부에서 실행됩니다." width="720" height="382" data-path="images/context-loading.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/context-loading-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=b274089ef9612d9c760bca9838557626" className="hidden dark:block" alt="컨텍스트 로딩: CLAUDE.md는 세션 시작 시 로드되고 모든 요청에 유지됩니다. MCP 도구 이름은 시작 시 로드되고 전체 스키마는 사용 시까지 연기됩니다. Skill은 시작 시 설명을 로드하고 호출 시 전체 콘텐츠를 로드합니다. Subagent는 격리된 컨텍스트를 받습니다. Hook은 외부에서 실행됩니다." width="720" height="382" data-path="images/context-loading-dark.svg" />

<Tabs>
  <Tab title="CLAUDE.md">
    **시기:** 세션 시작

    **로드되는 내용:** 모든 CLAUDE.md 파일의 전체 콘텐츠(관리, 사용자 및 프로젝트 수준).

    **상속:** Claude는 작업 디렉토리에서 루트까지 CLAUDE.md 파일을 읽고, 해당 파일에 접근할 때 하위 디렉토리에서 중첩된 파일을 검색합니다. 자세한 내용은 [CLAUDE.md 파일이 로드되는 방식](/docs/ko/memory#how-claude-md-files-load)을 참조하세요.

    <Tip>CLAUDE.md를 200줄 이하로 유지하세요. 참조 자료를 skill로 이동하면 온디맨드로 로드됩니다. [체크인된 CLAUDE.md에 대한 트림 제안을 받으려면](/docs/ko/memory#my-claude-md-is-too-large) `/doctor`를 실행하세요.</Tip>
  </Tab>

  <Tab title="Skills">
    Skill은 Claude의 도구 키트에 있는 추가 기능입니다. 참조 자료(API 스타일 가이드처럼) 또는 `/<name>`으로 트리거하는 호출 가능한 워크플로우(배포처럼)일 수 있습니다. Claude Code는 기본적으로 작동하는 `/code-review`, `/batch`, `/debug`와 같은 [번들 skill](/docs/ko/commands)과 함께 제공됩니다. 자신의 것을 만들 수도 있습니다.

    **시기:** Skill의 구성에 따라 다릅니다. 기본적으로 설명은 세션 시작 시 로드되고 전체 콘텐츠는 사용 시 로드됩니다. 사용자 전용 skill(`disable-model-invocation: true`)의 경우, 호출할 때까지 아무것도 로드되지 않습니다.

    **로드되는 내용:** 모델 호출 가능 skill의 경우, Claude는 모든 요청에서 이름과 설명을 봅니다. `/<name>`으로 skill을 호출하거나 Claude가 자동으로 로드할 때, 전체 콘텐츠가 대화에 로드됩니다.

    **Claude가 skill을 선택하는 방식:** Claude는 작업을 skill 설명과 비교하여 관련성이 있는지 결정합니다. 설명이 모호하거나 겹치면, Claude가 잘못된 skill을 로드하거나 도움이 될 skill을 놓칠 수 있습니다. Claude에게 특정 skill을 사용하도록 지시하려면 `/<name>`으로 호출하세요. `disable-model-invocation: true`가 있는 Skill은 호출할 때까지 Claude에게 보이지 않습니다.

    **컨텍스트 비용:** 사용할 때까지 낮음. 사용자 전용 skill은 호출할 때까지 0 비용입니다.

    **Subagent에서:** Skill은 subagent에서 다르게 작동합니다. 온디맨드 로딩 대신, subagent의 `skills` 필드에 나열된 skill은 시작 시 컨텍스트에 완전히 미리 로드됩니다. Subagent는 여전히 Skill 도구를 통해 나열되지 않은 프로젝트, 사용자 및 플러그인 skill을 검색하고 호출할 수 있습니다.

    <Tip>부작용이 있는 skill에 `disable-model-invocation: true`를 사용하세요. 이는 컨텍스트를 절약하고 오직 사용자만 트리거하도록 보장합니다.</Tip>
  </Tab>

  <Tab title="MCP servers">
    **시기:** 세션 시작.

    **로드되는 내용:** 연결된 서버의 도구 이름 및 서버 지침. 전체 JSON 스키마는 Claude가 특정 도구가 필요할 때까지 연기됩니다.

    **컨텍스트 비용:** [도구 검색](/docs/ko/mcp#scale-with-mcp-tool-search)은 기본적으로 활성화되므로, 유휴 MCP 도구는 최소한의 컨텍스트를 소비합니다.

    <Tip>`/mcp`를 실행하여 각 서버의 연결 상태를 확인하세요. `/context all`을 실행하여 각 로드된 MCP 도구가 사용하는 토큰 수를 확인하세요. Claude Code는 서버가 끊어지면 [원격 서버에 자동으로 다시 연결](/docs/ko/mcp#automatic-reconnection)되며, 적극적으로 사용하지 않는 서버를 연결 해제할 수 있습니다.</Tip>
  </Tab>

  <Tab title="Code intelligence">
    **시기:** 파일 편집 후, 그리고 Claude가 코드를 탐색할 때 온디맨드.

    **로드되는 내용:** 각 파일 편집 후 유형 오류 및 경고. Claude가 기호를 조회할 때 정의, 참조 및 유형 정보.

    **컨텍스트 비용:** 낮음. 기호 조회는 종종 광범위한 파일 읽기를 대체하므로, 순 컨텍스트 사용이 감소할 수 있습니다.

    <Tip>LSP 도구는 언어에 대한 [code intelligence 플러그인](/docs/ko/plugins/code-intelligence)을 설치할 때까지 비활성화됩니다.</Tip>
  </Tab>

  <Tab title="Subagents">
    **시기:** 온디맨드, 작업을 위해 사용자나 Claude가 생성할 때.

    **로드되는 내용:** 신선한, 격리된 컨텍스트 포함:

    * 에이전트 자신의 시스템 프롬프트, Claude Code 시스템 프롬프트가 아님
    * 에이전트의 `skills:` 필드에 나열된 skill의 전체 콘텐츠
    * CLAUDE.md 및 git 상태, 내장 Explore 및 Plan 에이전트 [둘 다 생략](/docs/ko/sub-agents#what-loads-at-startup) 제외, 그리고 정의가 [`omitClaudeMd`](/docs/ko/sub-agents#supported-frontmatter-fields)를 설정하는 에이전트는 사용자, 프로젝트 및 로컬 CLAUDE.md 파일을 건너뜁니다
    * 리드 에이전트가 프롬프트에서 전달하는 모든 컨텍스트

    [포크](/docs/ko/sub-agents#fork-the-current-conversation)의 경우, Claude Code는 부모의 대화 지금까지, 시스템 프롬프트 및 도구를 로드합니다.

    **컨텍스트 비용:** 주 세션에서 격리됨.

    <Tip>전체 대화 컨텍스트가 필요하지 않은 작업에 subagent를 사용하세요. 격리는 주 세션이 부풀어지는 것을 방지합니다.</Tip>
  </Tab>

  <Tab title="Hooks">
    **시기:** 트리거 시. Claude Code는 도구 실행, 세션 경계, 프롬프트 제출, 권한 요청 및 압축과 같은 특정 라이프사이클 이벤트에서 hook을 실행합니다. 전체 목록은 [Hooks](/docs/ko/hooks)를 참조하세요.

    **로드되는 내용:** 기본적으로 없음. Hook은 외부에서 실행됩니다.

    **컨텍스트 비용:** 0, hook이 대화에 메시지로 추가되는 출력을 반환하지 않는 한.

    <Tip>Hook은 Claude의 컨텍스트에 영향을 주지 않아야 하는 부작용(린팅, 로깅)에 이상적입니다.</Tip>
  </Tab>
</Tabs>

<h2 id="learn-more">
  더 알아보기
</h2>

각 기능에는 설정 지침, 예시 및 구성 옵션이 있는 자신의 가이드가 있습니다.

<CardGroup cols={2}>
  <Card title="CLAUDE.md" icon="file-lines" href="/docs/ko/memory">
    프로젝트 컨텍스트, 규칙 및 지침 저장
  </Card>

  <Card title="Skills" icon="brain" href="/docs/ko/skills">
    Claude에게 도메인 전문성 및 재사용 가능한 워크플로우 제공
  </Card>

  <Card title="Subagents" icon="users" href="/docs/ko/sub-agents">
    격리된 컨텍스트로 작업 오프로드
  </Card>

  <Card title="Dynamic workflows" icon="network" href="/docs/ko/workflows">
    한 스크립트에서 많은 서브에이전트 실행
  </Card>

  <Card title="Cross-session messaging" icon="terminal" href="/docs/ko/cross-session-messaging">
    Claude가 다른 세션에 메시지를 보낼 수 있도록 허용
  </Card>

  <Card title="MCP" icon="plug" href="/docs/ko/mcp">
    Claude를 외부 서비스에 연결
  </Card>

  <Card title="Hooks" icon="bolt" href="/docs/ko/hooks-guide">
    Hook으로 작업 자동화
  </Card>

  <Card title="Plugins" icon="puzzle-piece" href="/docs/ko/plugins/overview">
    기능 세트 번들 및 공유
  </Card>

  <Card title="Marketplaces" icon="store" href="/docs/ko/plugins/create-marketplace">
    플러그인 컬렉션 호스트 및 배포
  </Card>
</CardGroup>
