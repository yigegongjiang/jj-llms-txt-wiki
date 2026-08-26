> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 출력 스타일

> 소프트웨어 엔지니어링 이상의 용도로 Claude Code 적응시키기

출력 스타일은 Claude가 응답하는 방식을 변경하며, Claude가 알고 있는 내용을 변경하지 않습니다. 이들은 시스템 프롬프트를 수정하여 역할, 톤, 출력 형식을 설정합니다. 매 턴마다 동일한 음성이나 형식을 다시 요청하거나 Claude가 소프트웨어 엔지니어 이외의 역할을 하기를 원할 때 사용합니다.

사용자 정의 출력 스타일은 지침을 시스템 프롬프트에 추가하고 Claude Code의 기본 제공 소프트웨어 엔지니어링 지침을 유지할지 여부를 선택할 수 있게 합니다. Claude가 여전히 코딩하고 있지만 통신 방식을 변경할 때(예: 항상 다이어그램으로 답변)는 유지합니다. Claude가 쓰기 어시스턴트나 데이터 분석가와 같이 소프트웨어 엔지니어링을 수행하지 않을 때는 제외합니다.

프로젝트, 규칙 또는 코드베이스에 대한 지침은 대신 [CLAUDE.md](/docs/ko/memory)를 사용합니다.

<h2 id="built-in-output-styles">
  기본 제공 출력 스타일
</h2>

Claude Code의 **Default** 출력 스타일은 기존 시스템 프롬프트이며, 소프트웨어 엔지니어링 작업을 효율적으로 완료하도록 설계되었습니다.

세 가지 추가 기본 제공 출력 스타일이 있습니다:

* **Proactive**: Claude가 즉시 실행하고, 일상적인 결정을 위해 일시 중지하는 대신 합리적인 가정을 하며, 계획보다 행동을 선호합니다. 이는 [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)가 적용하는 것보다 더 강력한 자율 실행 지침이며, 권한 모드를 변경하지 않고도 작동하므로, 도구가 실행되기 전에 여전히 권한 프롬프트를 볼 수 있습니다.

* **Explanatory**: 소프트웨어 엔지니어링 작업을 완료하는 동안 교육용 "Insights"를 제공합니다. 구현 선택 사항과 코드베이스 패턴을 이해하는 데 도움이 됩니다.

* **Learning**: 협업 방식의 학습 모드로, Claude는 코딩하면서 "Insights"를 공유할 뿐만 아니라 사용자가 작은 전략적 코드 조각을 직접 작성하도록 요청합니다. Claude Code는 구현할 코드에 `TODO(human)` 마커를 추가합니다.

<h2 id="change-your-output-style">
  출력 스타일 변경
</h2>

`/config`를 실행하고 **Output style**을 선택하여 메뉴에서 스타일을 선택합니다. 선택 사항은 [로컬 프로젝트 수준](/docs/ko/settings)의 `.claude/settings.local.json`에 저장됩니다.

<Note>독립 실행형 `/output-style` 명령은 v2.1.73에서 더 이상 사용되지 않으며 v2.1.91에서 제거되었습니다. `/config`를 사용하거나 `outputStyle` 설정을 직접 편집하십시오.</Note>

메뉴 없이 스타일을 설정하려면 설정 파일에서 `outputStyle` 필드를 직접 편집합니다:

```json theme={null}
{
  "outputStyle": "Explanatory"
}
```

출력 스타일은 시스템 프롬프트의 일부이며, Claude Code는 세션 시작 시 이를 한 번 읽습니다. 변경 사항은 `/clear` 후 또는 새 세션 후에 적용됩니다. 출력 스타일 변경이 캐시에 미치는 영향에 대해서는 [Claude Code가 prompt caching을 사용하는 방법](/docs/ko/prompt-caching#changing-output-style)을 참조하십시오.

<h2 id="create-a-custom-output-style">
  사용자 정의 출력 스타일 만들기
</h2>

사용자 정의 출력 스타일은 Markdown 파일입니다: frontmatter는 메타데이터용이고, 그 다음에 시스템 프롬프트에 추가할 지침이 있습니다.

<Steps>
  <Step title="Markdown 파일 만들기">
    세 가지 수준 중 하나에 저장합니다. 파일 이름이 스타일 이름이 되며, frontmatter에서 `name`을 설정하지 않는 한 그렇습니다.

    * 사용자: `~/.claude/output-styles`
    * 프로젝트: `.claude/output-styles`
    * 관리형 정책: [관리형 설정 디렉토리](/docs/ko/settings#settings-files) 내의 `.claude/output-styles`

    프로젝트 출력 스타일은 작업 디렉토리와 저장소 루트 사이의 모든 `.claude/output-styles/`에서 로드됩니다. v2.1.178부터 이러한 중첩된 디렉토리 중 하나 이상이 동일한 이름의 스타일을 정의하면 Claude Code는 작업 디렉토리에 가장 가까운 것을 사용합니다.
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
    `/config`를 실행하고 **출력 스타일** 아래에서 스타일을 선택합니다. `/clear` 후 또는 다음 세션을 시작할 때 적용됩니다.
  </Step>
</Steps>

[플러그인](/docs/ko/plugins-reference)도 `output-styles/` 디렉토리에 출력 스타일을 포함할 수 있습니다.

<h3 id="frontmatter">
  Frontmatter
</h3>

출력 스타일 파일은 다음 frontmatter 필드를 지원합니다:

| Frontmatter                | 목적                                                                                                                                     | 기본값        |
| :------------------------- | :------------------------------------------------------------------------------------------------------------------------------------- | :--------- |
| `name`                     | 파일 이름이 아닌 경우 출력 스타일의 이름                                                                                                                | 파일 이름에서 상속 |
| `description`              | `/config` 선택기에 표시되는 출력 스타일의 설명                                                                                                         | 없음         |
| `keep-coding-instructions` | Claude Code의 기본 제공 소프트웨어 엔지니어링 지침 유지                                                                                                   | `false`    |
| `force-for-plugin`         | 플러그인 출력 스타일만 해당: 사용자가 선택하지 않아도 플러그인이 활성화될 때마다 이 스타일을 자동으로 적용합니다. 사용자의 `outputStyle` 설정을 재정의합니다. 여러 활성화된 플러그인이 이를 설정하면 먼저 로드된 것이 우선합니다. | `false`    |

<h2 id="how-output-styles-work">
  출력 스타일의 작동 방식
</h2>

출력 스타일은 Claude Code의 시스템 프롬프트를 직접 수정합니다.

* 모든 출력 스타일은 시스템 프롬프트 끝에 추가된 자체 사용자 정의 지침을 가집니다.
* 모든 출력 스타일은 대화 중에 Claude가 출력 스타일 지침을 준수하도록 상기시키는 알림을 트리거합니다.
* 사용자 정의 출력 스타일은 `keep-coding-instructions`가 `true`로 설정되지 않는 한 범위 지정, 주석 작성, 작업 검증 방법과 같은 Claude Code의 기본 제공 소프트웨어 엔지니어링 지침을 제외합니다.

토큰 사용량은 스타일에 따라 다릅니다. 시스템 프롬프트에 지침을 추가하면 입력 토큰이 증가하지만, prompt caching은 세션의 첫 번째 요청 이후 이 비용을 줄입니다. 기본 제공 Explanatory 및 Learning 스타일은 설계상 Default보다 더 긴 응답을 생성하므로 출력 토큰이 증가합니다. 사용자 정의 스타일의 경우, 출력 토큰 사용량은 지침이 Claude에게 생성하도록 지시하는 내용에 따라 달라집니다.

<h2 id="comparisons-to-related-features">
  관련 기능과의 비교
</h2>

여러 기능이 Claude Code의 동작을 사용자 정의합니다. 출력 스타일은 시스템 프롬프트를 직접 수정하고 모든 응답에 적용됩니다. 다른 기능들은 기본 시스템 프롬프트를 변경하지 않고 지침을 추가하거나 특정 작업으로 범위를 지정합니다.

| 기능                       | 작동 방식                                    | 사용 시기                                   |
| :----------------------- | :--------------------------------------- | :-------------------------------------- |
| 출력 스타일                   | 시스템 프롬프트를 수정합니다                          | 매 턴마다 다른 역할, 톤, 또는 기본 응답 형식을 원할 때       |
| [CLAUDE.md](/docs/ko/memory)  | 시스템 프롬프트 이후에 사용자 메시지를 추가합니다              | Claude가 항상 프로젝트 규칙과 코드베이스 컨텍스트를 알아야 할 때 |
| `--append-system-prompt` | 아무것도 제거하지 않고 시스템 프롬프트에 추가합니다             | 단일 호출을 위한 일회성 추가를 원할 때                  |
| [Agents](/docs/ko/sub-agents) | 자신의 시스템 프롬프트, 모델, 도구를 가진 subagent를 실행합니다 | 초점이 맞춰진 작업을 위해 별도로 범위가 지정된 도우미를 원할 때    |
| [Skills](/docs/ko/skills)     | 호출되거나 관련성이 있을 때 작업별 지침을 로드합니다            | 재사용 가능한 워크플로우가 있을 때                     |

<h2 id="related-resources">
  관련 리소스
</h2>

* [Settings](/docs/ko/settings): `outputStyle` 필드가 있는 위치 및 설정 우선순위 작동 방식
* [Permission modes](/docs/ko/permission-modes): Proactive 스타일이 자동 모드와 어떻게 비교되는지
* [Plugins](/docs/ko/plugins): skills, hooks, agents와 함께 출력 스타일을 패키징하고 배포합니다
* [Debug your configuration](/docs/ko/debug-your-config): 출력 스타일이 적용되지 않는 이유를 진단합니다
