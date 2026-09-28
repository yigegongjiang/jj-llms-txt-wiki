> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 출력 스타일

> 내장된 Concise 또는 Explanatory와 같은 출력 스타일이나 사용자 정의 스타일로 Claude Code의 역할, 톤, 응답 형식을 변경합니다.

출력 스타일은 Claude의 역할, 톤, 응답 형식을 세션의 모든 응답에 대해 설정하는 일련의 지침입니다. Claude Code는 기본값 외에 4가지 내장 스타일을 포함하고 있으며, 사용자 정의 스타일을 작성할 수 있습니다.

출력 스타일을 사용하여 Claude가 응답하고 전체 세션 동안 사용자와 함께 작업하는 방식을 변경하므로 각 프롬프트에서 요청을 반복할 필요가 없습니다. 예를 들어, 내장 스타일은 응답을 더 짧게 만들거나, 각 변경 사항에 대한 설명을 추가하거나, Claude가 일상적인 질문을 하지 않고 작업을 시작하도록 할 수 있습니다. 사용자 정의 스타일은 Claude를 소프트웨어 엔지니어 이외의 것(예: 작성 어시스턴트 또는 데이터 분석가)으로 변환할 수도 있습니다.

* 내장 스타일을 사용하려면 [내장 출력 스타일](#built-in-output-styles)에서 하나를 선택하고 [스타일로 전환](#change-your-output-style)합니다.
* 자신의 지침을 작성하려면 [사용자 정의 출력 스타일 만들기](#create-a-custom-output-style)를 참조합니다.

<Note>
  출력 스타일은 Claude에게 따를 지침을 제공합니다. 무언가가 항상 발생하거나 절대 발생하지 않는다는 것을 보장하지 않습니다. 일부 요구 사항은 다른 기능에 적합합니다:

  * Claude가 프로젝트에 대해 알아야 할 사항은 [CLAUDE.md](/docs/ko/memory)를 사용합니다.
  * 각 편집 후 형식 지정 또는 명령 차단과 같이 매번 발생해야 하는 경우 [hook](/docs/ko/hooks-guide)을 사용합니다.
  * 스킬, 서브에이전트 및 기타 옵션은 [출력 스타일과 다른 기능 간 선택](#choose-between-an-output-style-and-other-features)을 참조합니다.
</Note>

<h2 id="built-in-output-styles">
  기본 제공 출력 스타일
</h2>

Claude Code는 [**Default**](#default) 스타일로 시작하며, 이는 소프트웨어 엔지니어링 작업을 완료하기 위한 표준 지침입니다. 다른 네 가지 기본 제공 스타일은 이러한 지침을 유지하면서 자신만의 지침을 추가합니다.

이 표는 각 스타일이 세션을 어떻게 변경하는지와 언제 사용하면 좋은지 보여줍니다:

| 스타일                         | 변경 사항                                              | 사용 시기                                                     |
| :-------------------------- | :------------------------------------------------- | :-------------------------------------------------------- |
| [Proactive](#proactive)     | Claude가 즉시 작업을 시작하고 일상적인 결정에 대해 묻는 대신 합리적인 가정을 합니다 | Claude가 일상적인 결정을 계속 처리하도록 하고 싶으며, 가정이 잘못되면 방향을 수정할 수 있습니다 |
| [Concise](#concise)         | 응답이 결과로 시작하고 서문, 설명, 요약을 생략합니다                     | 기본 응답이 원하는 것보다 깁니다                                        |
| [Explanatory](#explanatory) | Claude가 작성한 코드 뒤의 선택을 설명하는 짧은 `Insight` 블록을 추가합니다  | 코드베이스를 배우고 있거나 변경 사항과 함께 추론을 원합니다                         |
| [Learning](#learning)       | Claude가 선택을 설명하고 작성할 코드의 작은 조각을 남깁니다               | 작업이 여전히 완료되는 동안 실습 코딩 연습을 원합니다                            |

<h3 id="default">
  Default
</h3>

Default는 출력 스타일이 선택되지 않음을 의미합니다. Claude Code는 스타일 지침을 추가하지 않으며, Claude는 소프트웨어 엔지니어링 작업을 위해 작성된 Claude Code의 표준 시스템 프롬프트에서 작동합니다.

`default`는 다른 스타일과 함께 `/output-style` 목록에 나타나므로, [같은 방식으로 선택](#change-your-output-style)합니다.

<h3 id="proactive">
  Proactive
</h3>

Proactive 스타일에서 Claude는 작업을 보내자마자 구현을 시작합니다. 일상적인 결정에 대해 합리적인 가정을 하며 묻지 않으며, 계획을 요청하지 않는 한 계획 모드로 전환하지 않습니다. 언제든지 방향을 바꿀 수 있습니다.

스타일의 지침은 또한 Claude에게 데이터를 삭제하거나 공유 또는 프로덕션 시스템을 변경하는 작업 전에 대화에서 사용자와 확인하도록 지시합니다. 이 확인은 Claude가 따르는 지침이며 권한 프롬프트와는 별개입니다.

Proactive 스타일로 전환해도 [권한 모드](/docs/ko/permission-modes)는 변경되지 않습니다. 권한 모드는 여전히 사용자에게 묻지 않고 실행되는 도구 호출을 결정하므로, 권한 프롬프트는 전환 전과 같은 방식으로 나타납니다.

<h3 id="concise">
  Concise
</h3>

Concise 스타일에서 응답의 첫 번째 문장은 무엇이 일어났는지 또는 답이 무엇인지를 나타냅니다. Claude는 도입부, 단계별 설명, 마무리 요약을 생략하고 간단한 질문에 1\~3개 문장으로 답변합니다. Default 스타일과 동일하게 엔지니어링 작업을 철저히 수행합니다. Claude Code v2.1.237 이상이 필요합니다.

Claude는 다음의 경우 전체 길이로 작성합니다:

* **요청하는 모든 것**: 설명이나 더 많은 세부 정보를 요청할 때, Claude는 완전히 답변합니다.
* **안전하게 행동하기 위해 필요한 모든 것**: 오류 보고서, 실패한 테스트 출력, 보안 경고, 파괴적인 작업에 대한 확인은 전체 내용을 유지합니다.

<h3 id="explanatory">
  Explanatory
</h3>

Explanatory 스타일에서 Claude는 Default 스타일과 동일한 방식으로 작업을 수행하고 선택한 이유에 대한 짧은 설명을 추가합니다. 각 설명은 대화에서 관련 코드 앞이나 뒤에 `Insight`라는 레이블이 붙은 블록으로 나타납니다. 설명은 파일에 주석으로 작성되지 않습니다.

`Insight` 블록은 코드베이스 또는 Claude가 작성한 코드에 대한 2\~3개의 포인트를 포함하며, API 엔드포인트를 추가한 후의 다음과 같은 예시가 있습니다:

```text theme={null}
★ Insight ─────────────────────────────────────
- Every route in this repo goes through the withAuth wrapper, so the new endpoint gets session checks without its own middleware.
- Rate limits are set per route in limits.ts, which is why this change adds an entry there rather than a global default.
─────────────────────────────────────────────────
```

<h3 id="learning">
  Learning
</h3>

Learning 스타일에서 Claude는 [Explanatory 스타일](#explanatory)과 동일한 `Insight` 블록을 추가하고 코드의 일부를 작성하도록 요청합니다. Claude는 일상적인 구현을 자체적으로 처리합니다. 오류 처리, 데이터 구조, 또는 유효한 접근 방식이 여러 개인 비즈니스 로직과 같은 실제 설계 결정이 있는 부분에 도달하면, 몇 줄을 남깁니다.

Claude는 파일의 `TODO(human)` 주석으로 위치를 표시한 다음, 이미 구축된 것, 작성할 것, 고려할 것을 설명하는 요청을 보냅니다:

```text theme={null}
● Learn by Doing

Context: The upload form is in place and calls validateFile() before accepting a file. Size and type checks work for images, but the switch statement has no handling for documents yet.

Your Task: In upload.js, implement the case "document" branch inside validateFile(). Look for TODO(human).

Guidance: Decide on a size limit for documents and whether the file extension has to match the MIME type. Return {valid: boolean, error?: string}.
```

Claude는 멈추고 기다립니다. `TODO(human)` 주석에서 코드를 작성하고 완료했을 때 Claude에게 알립니다. Claude는 코드에 대한 하나의 `Insight`로 응답하고 작업을 계속합니다.

<h2 id="change-your-output-style">
  출력 스타일 변경
</h2>

명령, 메뉴 또는 설정 파일로 스타일을 선택합니다. 명령과 두 메뉴 모두 선택 사항을 [로컬 프로젝트 수준](/docs/ko/settings)의 `.claude/settings.local.json`에 저장합니다.

* **`/output-style` 명령**: `/output-style <style>`을 실행하여 전환합니다. 예를 들어 `/output-style concise`입니다. 인수 없이 실행하면 선택할 수 있는 스타일을 나열하고 현재 스타일을 표시합니다.

  이 명령은 [비대화형 모드](/docs/ko/headless) 및 Agent SDK 세션에서도 작동하며, 모바일 앱 또는 웹에서 [원격 제어](/docs/ko/remote-control#limitations)를 통해 사용할 수 있습니다. 여기서는 [기본 제공 스타일](#built-in-output-styles)만 나열하고 선택할 수 있습니다. Claude Code v2.1.269 이상이 필요합니다.
* **Terminal 메뉴**: `/config`를 실행하고 **Output style**을 선택하여 메뉴에서 스타일을 선택합니다.
* **VS Code extension**: `/`로 [명령 메뉴](/docs/ko/vs-code#use-the-prompt-box)를 열고 **Output styles**을 선택하여 사용자 정의 스타일을 포함한 스타일을 선택합니다. Claude Code v2.1.257 이상이 필요합니다.
* **Desktop app**: 설정 파일(예: `.claude/settings.local.json`, 터미널 메뉴가 작성하는 파일)에서 `outputStyle` 필드를 설정합니다. `/config`를 실행하면 Claude Code는 메뉴 대신 [**Settings > Claude Code**](/docs/ko/desktop#what%E2%80%99s-not-available-in-desktop)를 엽니다.

메뉴 없이 스타일을 설정하려면 설정 파일에서 `outputStyle` 필드를 직접 편집합니다:

```json theme={null}
{
  "outputStyle": "Explanatory"
}
```

값은 대소문자를 구분하므로 기본 제공 이름을 `Proactive`, `Concise`, `Explanatory`, `Learning`으로 작성합니다. `explanatory`와 같이 스타일 이름과 정확히 일치하지 않는 값은 기본 스타일을 제공합니다. `/output-style` 명령은 대소문자를 무시합니다.

프로젝트 전체에서 스타일을 기본값으로 설정하려면 `~/.claude/settings.json`에서 `outputStyle`을 설정합니다. 프로젝트의 자체 설정 파일은 해당 값보다 [우선합니다](/docs/ko/settings#settings-precedence).

세션 중에 스타일을 전환하면 Claude는 다음 메시지부터 새로운 스타일을 사용합니다. 첫 번째 메시지의 prompt caching 비용에 대해서는 [출력 스타일 변경](/docs/ko/prompt-caching#changing-output-style)을 참조하십시오. v2.1.251 이전에는 새로운 스타일이 `/clear`를 실행하거나 새 세션을 시작한 후에만 적용되었습니다.

<h2 id="create-a-custom-output-style">
  사용자 정의 출력 스타일 만들기
</h2>

사용자 정의 출력 스타일은 Markdown 파일입니다: frontmatter는 메타데이터용이고, 그 다음에 Claude를 위한 지침이 있습니다.

VS Code 확장 프로그램에서는 직접 작성하는 대신 [**출력 스타일** 메뉴](/docs/ko/vs-code#use-the-prompt-box)에서 파일을 만들 수도 있습니다. 이 기능은 Claude Code v2.1.261 이상이 필요합니다.

<Steps>
  <Step title="Markdown 파일 만들기">
    세 가지 수준 중 하나에 저장합니다. 파일 이름이 스타일 이름이 되며, frontmatter에서 `name`을 설정하지 않는 한 그렇습니다.

    * 사용자: `~/.claude/output-styles`
    * 프로젝트: `.claude/output-styles`
    * 관리형 정책: [관리형 설정 디렉토리](/docs/ko/managed-settings#delivery-mechanisms) 내의 `.claude/output-styles`

    프로젝트 출력 스타일은 작업 디렉토리와 저장소 루트 사이의 모든 `.claude/output-styles/`에서 로드됩니다. 이러한 중첩된 디렉토리 중 하나 이상이 동일한 이름의 스타일을 정의하면 Claude Code는 작업 디렉토리에 가장 가까운 것을 사용합니다.
  </Step>

  <Step title="Frontmatter 및 지침 추가">
    Claude Code의 소프트웨어 엔지니어링 지침을 유지할지 여부를 결정합니다. Claude가 통신 방식을 변경하지만 여전히 동일한 방식으로 코딩하기를 원하면 `keep-coding-instructions: true`를 설정합니다. Claude가 소프트웨어 엔지니어링을 수행하지 않을 경우 제외합니다.

    이 예제는 Claude의 코딩 동작을 유지하면서 모든 설명 앞에 다이어그램을 배치합니다:

    ```markdown theme={null}
    ---
    name: Diagrams first
    description: Lead every explanation with a diagram
    keep-coding-instructions: true
    ---

    When explaining code, architecture, or data flow, start with a Mermaid diagram showing the structure, then explain in prose.

    ## Diagram conventions

    Use `flowchart TD` for control flow and `sequenceDiagram` for request paths. Keep diagrams under 15 nodes.
    ```
  </Step>

  <Step title="스타일로 전환">
    터미널에서 `/output-style <style>`을 실행하거나 `/config`를 실행하고 **출력 스타일** 아래에서 스타일을 선택합니다. Claude는 다음 메시지부터 새 스타일을 사용합니다. 터미널에서 Claude Code는 시작할 때 스타일 파일을 읽으므로, 실행 중인 세션 중에 파일을 만들거나 편집하면 변경 사항을 적용하려면 Claude Code를 다시 시작해야 합니다.
  </Step>
</Steps>

[플러그인](/docs/ko/plugins/manifest-reference)도 `output-styles/` 디렉토리에 출력 스타일을 포함할 수 있습니다.

<h3 id="frontmatter">
  Frontmatter 참조
</h3>

YAML [frontmatter](/docs/ko/glossary#frontmatter)를 사용하여 출력 스타일을 구성합니다. 파일 맨 위의 `---` 마커 사이에 위치합니다. 모든 필드는 선택 사항이며, 필드 이름은 하이픈으로 구분된 소문자 단어를 사용합니다. 오타가 있는 필드는 오류 없이 무시됩니다. YAML이 파싱되지 않으면 스타일은 여전히 파일 이름으로 로드되며 필드가 설정되지 않습니다. `claude --debug`를 실행하여 파싱 오류를 확인합니다.

| 필드                         | 필수  | 설명                                                                                                                                                                               |
| :------------------------- | :-- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                     | 아니요 | 출력 스타일의 이름으로, `/config` 선택기에 표시됩니다. 기본값: 파일 이름                                                                                                                                   |
| `description`              | 아니요 | 출력 스타일의 설명으로, `/config` 선택기에 표시됩니다                                                                                                                                               |
| `keep-coding-instructions` | 아니요 | `true`로 설정하면 Claude Code의 기본 제공 소프트웨어 엔지니어링 지침을 스타일과 함께 유지합니다. 기본값: `false`                                                                                                      |
| `force-for-plugin`         | 아니요 | 플러그인 출력 스타일만 해당합니다. `true`로 설정하면 사용자가 선택하지 않아도 플러그인이 활성화될 때마다 이 스타일을 자동으로 적용합니다. 사용자의 `outputStyle` 설정을 재정의합니다. 여러 활성화된 플러그인이 이를 설정하면 Claude Code는 먼저 로드된 것을 사용합니다. 기본값: `false` |

<span id="comparisons-to-related-features" />

<h2 id="choose-between-an-output-style-and-other-features">
  출력 스타일과 다른 기능 중에서 선택하기
</h2>

출력 스타일은 세션의 모든 응답에 적용됩니다. 이는 Claude가 따르는 지침이므로 이를 강제하는 것은 없습니다. 원하는 것이 모든 응답보다 좁거나 반드시 실패 없이 발생해야 하는 경우, 다른 기능이 더 적합합니다.

이 표는 원하는 것을 이를 수행하는 기능과 일치시킵니다:

| 원하는 것                                        | 사용                                                                | 적합한 이유                                                                   |
| :------------------------------------------- | :---------------------------------------------------------------- | :----------------------------------------------------------------------- |
| 특정 음성, 길이 또는 형식의 모든 응답, 또는 다른 역할의 Claude     | 출력 스타일                                                            | 전체 세션에 적용되며, 한 명령으로 스타일을 전환합니다                                           |
| Claude가 프로젝트의 규칙, 명령 및 구조를 알기를 원함            | [CLAUDE.md](/docs/ko/memory)                                           | 이는 Claude가 코드베이스에 대해 알아야 할 내용을 보유하며, 선택한 스타일에 관계없이 로드된 상태로 유지됩니다         |
| 릴리스 체크리스트 또는 검토 절차와 같은 한 종류의 작업에 대한 지침       | [skill](/docs/ko/skills)                                               | Claude는 이를 호출하거나 작업이 일치할 때만 로드하므로 관련 없는 응답을 형성하지 않습니다                    |
| 각 편집 후 형식 지정 또는 명령 차단과 같이 예외 없이 매번 발생해야 하는 것 | [hook](/docs/ko/hooks-guide)                                           | Claude Code는 라이프사이클 이벤트에서 hook을 자체적으로 실행하므로 Claude가 지침을 따르는 것에 의존하지 않습니다 |
| 집중된 작업을 위해 자체 지침, 모델 및 도구를 가진 도우미            | [subagent](/docs/ko/sub-agents)                                        | 자체 시스템 프롬프트를 사용하여 별도의 컨텍스트에서 실행되고 대화에 요약을 반환합니다                          |
| Claude Code를 시작할 때 전달하는 Claude의 지침에 대한 추가    | [`--append-system-prompt`](/docs/ko/cli-reference#system-prompt-flags) | 아무것도 제거하지 않고 시스템 프롬프트에 추가합니다                                             |

이러한 기능들은 결합됩니다. 예를 들어, Claude가 알아야 할 내용에 CLAUDE.md를 사용하고, 응답 방식에 출력 스타일을 사용하며, 보장되어야 하는 모든 것에 hook을 사용할 수 있습니다. [Claude Code 확장](/docs/ko/features-overview)은 나머지 확장 기능을 비교합니다.

<h2 id="how-output-styles-work">
  출력 스타일의 작동 방식
</h2>

출력 스타일은 Claude Code가 Claude에게 제공하는 지침을 변경합니다.

* Claude Code는 모든 요청과 함께 활성 스타일의 지침을 전송합니다.
* 사용자 정의 출력 스타일은 `keep-coding-instructions`가 `true`로 설정되지 않은 한, 변경 범위 지정 방법, 주석 작성 방법, 작업 검증 방법 등 Claude Code의 기본 제공 소프트웨어 엔지니어링 지침을 제외합니다.

출력 스타일은 주 대화와 부모의 전체 대화 및 시스템 프롬프트를 상속하는 [포크](/docs/ko/sub-agents#fork-the-current-conversation)에 적용됩니다. 다른 [subagent는 자신의 시스템 프롬프트를 실행](/docs/ko/sub-agents#what-loads-at-startup)하므로 스타일은 응답 방식을 변경하지 않습니다.

토큰 사용량은 스타일에 따라 달라집니다. 스타일의 지침은 입력 토큰을 추가하지만, 프롬프트 캐싱은 세션의 첫 번째 요청 이후 이 비용을 줄입니다.

기본 제공 설명 및 학습 스타일은 설계상 기본값보다 더 긴 응답을 생성하므로 출력 토큰이 증가합니다. 간결 스타일은 Claude에게 기본적으로 응답을 짧게 유지하도록 지시하여 반대의 효과를 냅니다. 사용자 정의 스타일의 경우, 출력 토큰 사용량은 지침이 Claude에게 생성하도록 지시하는 내용에 따라 달라집니다.

<h2 id="related-resources">
  관련 리소스
</h2>

* [Settings](/docs/ko/settings): `outputStyle` 필드가 있는 위치 및 설정 우선순위 작동 방식
* [Permission modes](/docs/ko/permission-modes): Proactive 스타일이 자동 모드와 어떻게 비교되는지
* [Plugins](/docs/ko/plugins/overview): skills, hooks, agents와 함께 출력 스타일을 패키징하고 배포합니다
* [Debug your configuration](/docs/ko/debug-your-config): 출력 스타일이 적용되지 않는 이유를 진단합니다
