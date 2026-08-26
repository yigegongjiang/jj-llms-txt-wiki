> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 플러그인 만들기

> skills, agents, hooks, MCP servers를 사용하여 Claude Code를 확장하는 사용자 정의 플러그인을 만듭니다.

플러그인을 사용하면 프로젝트와 팀 전체에서 공유할 수 있는 사용자 정의 기능으로 Claude Code를 확장할 수 있습니다. 이 가이드에서는 skills, agents, hooks, MCP servers를 사용하여 자신의 플러그인을 만드는 방법을 다룹니다.

기존 플러그인을 설치하려고 하시나요? [플러그인 발견 및 설치](/docs/ko/discover-plugins)를 참조하세요. 완전한 기술 사양은 [플러그인 참조](/docs/ko/plugins-reference)를 참조하세요.

<h2 id="when-to-use-plugins-vs-standalone-configuration">
  플러그인 대 독립 실행형 구성 사용 시기
</h2>

Claude Code는 사용자 정의 skills, agents, hooks를 추가하는 두 가지 방법을 지원합니다:

| 접근 방식                                                                                     | Skill 이름             | 최적 용도                                   |
| :---------------------------------------------------------------------------------------- | :------------------- | :-------------------------------------- |
| **독립 실행형** (`.claude/` 디렉토리)                                                              | `/hello`             | 개인 워크플로우, 프로젝트별 사용자 정의, 빠른 실험           |
| **플러그인** (skills, agents, hooks를 포함하거나 `.claude-plugin/plugin.json` 매니페스트가 있는 자체 포함 디렉토리) | `/plugin-name:hello` | 팀원과 공유, 커뮤니티에 배포, 버전 관리 릴리스, 프로젝트 간 재사용 |

**다음의 경우 독립 실행형 구성을 사용하세요**:

* 단일 프로젝트에 대해 Claude Code를 사용자 정의하는 경우
* 구성이 개인적이며 공유할 필요가 없는 경우
* skills 또는 hooks를 패키징하기 전에 실험하는 경우
* `/hello` 또는 `/deploy`와 같은 짧은 skill 이름을 원하는 경우

**다음의 경우 플러그인을 사용하세요**:

* 팀 또는 커뮤니티와 기능을 공유하려는 경우
* 여러 프로젝트에서 동일한 skills/agents가 필요한 경우
* 확장 기능에 대한 버전 제어 및 쉬운 업데이트를 원하는 경우
* 마켓플레이스를 통해 배포하는 경우
* `/my-plugin:hello`와 같은 네임스페이스 skills를 사용해도 괜찮은 경우 (네임스페이싱은 플러그인 간 충돌을 방지합니다)

<Tip>
  빠른 반복을 위해 `.claude/`의 독립 실행형 구성으로 시작한 다음, 공유할 준비가 되면 [기존 구성을 플러그인으로 변환](#convert-existing-configurations-to-plugins)하세요.
</Tip>

<h2 id="quickstart">
  빠른 시작
</h2>

이 빠른 시작은 사용자 정의 skill을 사용하여 플러그인을 만드는 과정을 안내합니다. 매니페스트(플러그인을 정의하는 구성 파일)를 만들고, skill을 추가하고, `--plugin-dir` 플래그를 사용하여 로컬에서 테스트합니다.

<h3 id="prerequisites">
  필수 조건
</h3>

* Claude Code [설치 및 인증](/docs/ko/quickstart#step-1-install-claude-code)

<Note>
  `/plugin` 명령이 보이지 않으면 Claude Code를 최신 버전으로 업데이트하세요. 업그레이드 지침은 [문제 해결](/docs/ko/troubleshooting)을 참조하세요.
</Note>

<h3 id="create-your-first-plugin">
  첫 번째 플러그인 만들기
</h3>

<Steps>
  <Step title="플러그인 디렉토리 만들기">
    모든 플러그인은 skills, agents 또는 hooks를 포함하는 자체 디렉토리에 있으며, 선택적으로 `.claude-plugin/plugin.json` 매니페스트와 함께 있습니다. 이 빠른 시작에서는 `--plugin-dir`을 사용하여 테스트 단계에서 Claude Code가 디렉토리를 가리키기 때문에 위치는 중요하지 않습니다. 스크래치 폴더나 프로젝트 디렉토리와 같이 편리한 곳 어디든 만들 수 있습니다:

    ```bash theme={null}
    mkdir my-first-plugin
    ```

    나머지 단계는 상위 디렉토리에서 실행되며 `my-first-plugin/...`과 같은 경로를 상대 경로로 참조합니다.
  </Step>

  <Step title="플러그인 매니페스트 만들기">
    `.claude-plugin/plugin.json`의 매니페스트 파일은 플러그인의 정체성을 정의합니다: 이름, 설명, 버전. Claude Code는 이 메타데이터를 사용하여 플러그인 관리자에서 플러그인을 표시합니다.

    플러그인 폴더 내에 `.claude-plugin` 디렉토리를 만듭니다:

    ```bash theme={null}
    mkdir my-first-plugin/.claude-plugin
    ```

    그런 다음 다음 내용으로 `my-first-plugin/.claude-plugin/plugin.json`을 만듭니다:

    ```json my-first-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-first-plugin",
      "description": "A greeting plugin to learn the basics",
      "version": "1.0.0",
      "author": {
        "name": "Your Name"
      }
    }
    ```

    | 필드            | 목적                                                                                                                                                                 |
    | :------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | `name`        | 고유 식별자 및 skill 네임스페이스. Skills는 이것으로 접두사가 붙습니다 (예: `/my-first-plugin:hello`).                                                                                       |
    | `description` | 플러그인을 검색하거나 설치할 때 플러그인 관리자에 표시됩니다.                                                                                                                                 |
    | `version`     | 선택 사항. 설정된 경우 사용자는 이 필드를 변경할 때만 업데이트를 받습니다. 생략되고 플러그인이 git을 통해 배포되는 경우 커밋 SHA가 사용되며 모든 커밋이 새 버전으로 계산됩니다. [버전 관리](/docs/ko/plugins-reference#version-management)를 참조하세요. |
    | `author`      | 선택 사항. 속성에 유용합니다.                                                                                                                                                  |

    `homepage`, `repository`, `license`와 같은 추가 필드는 [전체 매니페스트 스키마](/docs/ko/plugins-reference#plugin-manifest-schema)를 참조하세요.
  </Step>

  <Step title="Skill 추가">
    Skills는 `skills/` 디렉토리에 있습니다. 각 skill은 `SKILL.md` 파일을 포함하는 폴더입니다. 폴더 이름은 skill 이름이 되며, 플러그인의 네임스페이스가 접두사로 붙습니다 (`my-first-plugin`이라는 플러그인의 `hello/`는 `/my-first-plugin:hello`를 만듭니다).

    플러그인 폴더에 skill 디렉토리를 만듭니다:

    ```bash theme={null}
    mkdir -p my-first-plugin/skills/hello
    ```

    그런 다음 다음 내용으로 `my-first-plugin/skills/hello/SKILL.md`를 만듭니다:

    ```markdown my-first-plugin/skills/hello/SKILL.md theme={null}
    ---
    description: Greet the user with a friendly message
    disable-model-invocation: true
    ---

    Greet the user warmly and ask how you can help them today.
    ```
  </Step>

  <Step title="플러그인 테스트">
    `--plugin-dir` 플래그를 사용하여 Claude Code를 실행하여 플러그인을 로드합니다:

    ```bash theme={null}
    claude --plugin-dir ./my-first-plugin
    ```

    Claude Code가 시작되면 새 skill을 시도해보세요:

    ```shell theme={null}
    /my-first-plugin:hello
    ```

    Claude가 인사말로 응답하는 것을 볼 수 있습니다. `/help`를 실행하여 플러그인 네임스페이스 아래에 나열된 skill을 확인하세요.

    <Note>
      **네임스페이싱이 필요한 이유?** 플러그인 skills는 항상 네임스페이스가 지정됩니다 (예: `/my-first-plugin:hello`). 여러 플러그인이 동일한 이름의 skills를 가질 때 충돌을 방지합니다.

      네임스페이스 접두사를 변경하려면 `plugin.json`의 `name` 필드를 업데이트하세요.
    </Note>
  </Step>

  <Step title="Skill 인수 추가">
    사용자 입력을 수락하여 skill을 동적으로 만듭니다. `$ARGUMENTS` 자리 표시자는 사용자가 skill 이름 뒤에 제공하는 모든 텍스트를 캡처합니다.

    `SKILL.md` 파일을 업데이트합니다:

    ```markdown my-first-plugin/skills/hello/SKILL.md theme={null}
    ---
    description: Greet the user with a personalized message
    ---

    # Hello Skill

    Greet the user named "$ARGUMENTS" warmly and ask how you can help them today. Make the greeting personal and encouraging.
    ```

    `/reload-plugins`를 실행하여 변경 사항을 적용한 다음 이름으로 skill을 시도해보세요:

    ```shell theme={null}
    /my-first-plugin:hello Alex
    ```

    Claude가 이름으로 인사할 것입니다. skills에 인수를 전달하는 방법에 대한 자세한 내용은 [Skills](/docs/ko/skills#pass-arguments-to-skills)를 참조하세요.
  </Step>
</Steps>

다음과 같은 주요 구성 요소로 플러그인을 성공적으로 만들고 테스트했습니다:

* **플러그인 매니페스트** (`.claude-plugin/plugin.json`): 플러그인의 메타데이터를 설명합니다
* **Skills 디렉토리** (`skills/`): 사용자 정의 skills를 포함합니다
* **Skill 인수** (`$ARGUMENTS`): 동적 동작을 위해 사용자 입력을 캡처합니다

<Tip>
  `--plugin-dir` 플래그는 개발 및 테스트에 유용합니다. 플러그인을 다른 사람과 공유할 준비가 되면 [플러그인 마켓플레이스 만들기 및 배포](/docs/ko/plugin-marketplaces)를 참조하세요.
</Tip>

<h2 id="develop-a-plugin-in-your-skills-directory">
  기술 디렉토리에서 플러그인 개발
</h2>

매번 시작할 때 `--plugin-dir`을 전달하는 대신 기술 디렉토리에 플러그인을 유지하고 Claude Code가 자동으로 로드하도록 할 수 있습니다. `claude plugin init`이 스캐폴딩합니다:

```bash theme={null}
claude plugin init my-tool
```

이는 `.claude-plugin/plugin.json` 매니페스트와 시작 `SKILL.md`를 포함하는 `~/.claude/skills/my-tool/`을 만듭니다. 다음 세션에서는 마켓플레이스나 설치 단계 없이 `my-tool@skills-dir`로 로드됩니다.

자동 로드 규칙, 개인 대 프로젝트 범위, 작업 공간 신뢰 요구 사항, 업데이트 또는 제거 방법은 [기술 디렉토리 플러그인](/docs/ko/plugins-reference#skills-directory-plugins)을 참조하세요.

<h2 id="plugin-structure-overview">
  플러그인 구조 개요
</h2>

skill을 사용하여 플러그인을 만들었지만, 플러그인에는 훨씬 더 많은 것이 포함될 수 있습니다: 사용자 정의 agents, hooks, MCP servers, LSP servers, 백그라운드 모니터.

<Warning>
  **일반적인 실수**: `commands/`, `agents/`, `skills/`, `hooks/`를 `.claude-plugin/` 디렉토리 내에 넣지 마세요. `.claude-plugin/` 내에는 `plugin.json`만 들어갑니다. 다른 모든 디렉토리는 플러그인 루트 수준에 있어야 합니다.

  플러그인 루트는 개별 플러그인의 자체 디렉토리입니다: `.claude-plugin/plugin.json`을 포함하는 디렉토리입니다. 절대 `~/.claude/`가 아닙니다. 예를 들어, Claude Code는 `~/.claude/.mcp.json`에 배치된 `.mcp.json`을 읽지 않습니다.
</Warning>

| 디렉토리              | 위치      | 목적                                                       |
| :---------------- | :------ | :------------------------------------------------------- |
| `.claude-plugin/` | 플러그인 루트 | `plugin.json` 매니페스트를 포함합니다 (구성 요소가 기본 위치를 사용하는 경우 선택 사항) |
| `skills/`         | 플러그인 루트 | `<name>/SKILL.md` 디렉토리로서의 Skills                         |
| `commands/`       | 플러그인 루트 | Markdown 파일로서의 Skills. 새 플러그인의 경우 `skills/`를 사용하세요       |
| `agents/`         | 플러그인 루트 | 사용자 정의 agent 정의                                          |
| `hooks/`          | 플러그인 루트 | `hooks.json`의 이벤트 핸들러                                    |
| `.mcp.json`       | 플러그인 루트 | MCP server 구성                                            |
| `.lsp.json`       | 플러그인 루트 | 코드 인텔리전스를 위한 LSP server 구성                               |
| `monitors/`       | 플러그인 루트 | `monitors.json`의 백그라운드 모니터 구성                            |
| `bin/`            | 플러그인 루트 | 플러그인이 활성화된 동안 Bash tool의 `PATH`에 추가되는 실행 파일              |
| `settings.json`   | 플러그인 루트 | 플러그인이 활성화될 때 적용되는 기본 [설정](/docs/ko/settings)                  |

정확히 하나의 skill을 제공하는 플러그인은 `skills/` 디렉토리를 만드는 대신 `SKILL.md`를 플러그인 루트에 직접 배치할 수 있습니다. Claude Code는 이를 단일 skill로 로드하고 frontmatter `name` 필드를 호출 이름으로 사용합니다. 플러그인이 하나 이상의 skill로 성장할 수 있는 경우 `skills/` 레이아웃을 사용하세요.

<Note>
  **다음 단계**: 더 많은 기능을 추가할 준비가 되셨나요? [더 복잡한 플러그인 개발](#develop-more-complex-plugins)로 이동하여 agents, hooks, MCP servers, LSP servers를 추가하세요. 모든 플러그인 구성 요소의 완전한 기술 사양은 [플러그인 참조](/docs/ko/plugins-reference)를 참조하세요.
</Note>

<h2 id="develop-more-complex-plugins">
  더 복잡한 플러그인 개발
</h2>

기본 플러그인에 익숙해지면 더 정교한 확장 기능을 만들 수 있습니다.

<h3 id="add-skills-to-your-plugin">
  플러그인에 Skills 추가
</h3>

플러그인은 Claude의 기능을 확장하기 위해 [Agent Skills](/docs/ko/skills)를 포함할 수 있습니다. Skills는 모델 호출입니다: Claude는 작업 컨텍스트에 따라 자동으로 사용합니다.

플러그인 루트에 `SKILL.md` 파일을 포함하는 Skill 폴더가 있는 `skills/` 디렉토리를 추가합니다:

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json
└── skills/
    └── code-review/
        └── SKILL.md
```

각 `SKILL.md`는 YAML 프론트매터와 지침을 포함합니다. Claude가 skill을 언제 사용할지 알 수 있도록 `description`을 포함하세요:

```yaml theme={null}
---
description: Reviews code for best practices and potential issues. Use when reviewing code, checking PRs, or analyzing code quality.
---

When reviewing code, check for:
1. Code organization and structure
2. Error handling
3. Security concerns
4. Test coverage
```

플러그인을 설치한 후 `/reload-plugins`를 실행하여 Skills를 로드합니다. 점진적 공개 및 도구 제한을 포함한 완전한 Skill 작성 지침은 [Agent Skills](/docs/ko/skills)를 참조하세요.

<h3 id="add-lsp-servers-to-your-plugin">
  플러그인에 LSP servers 추가
</h3>

<Tip>
  TypeScript, Python, Rust와 같은 일반적인 언어의 경우 공식 마켓플레이스에서 미리 빌드된 LSP 플러그인을 설치하세요. 이미 다루어진 언어가 아닌 언어에 대한 지원이 필요한 경우에만 사용자 정의 LSP 플러그인을 만드세요.
</Tip>

LSP (Language Server Protocol) 플러그인은 Claude에 실시간 코드 인텔리전스를 제공합니다. 아직 공식 LSP 플러그인이 없는 언어를 지원해야 하는 경우 플러그인에 `.lsp.json` 파일을 추가하여 자신의 플러그인을 만들 수 있습니다:

```json .lsp.json theme={null}
{
  "go": {
    "command": "gopls",
    "args": ["serve"],
    "extensionToLanguage": {
      ".go": "go"
    }
  }
}
```

플러그인을 설치하는 사용자는 자신의 머신에 언어 server 바이너리를 설치해야 합니다.

완전한 LSP 구성 옵션은 [LSP servers](/docs/ko/plugins-reference#lsp-servers)를 참조하세요.

<h3 id="add-background-monitors-to-your-plugin">
  플러그인에 백그라운드 모니터 추가
</h3>

백그라운드 모니터를 사용하면 플러그인이 로그, 파일 또는 외부 상태를 백그라운드에서 감시하고 이벤트가 도착할 때 Claude에 알릴 수 있습니다. Claude Code는 플러그인이 활성화될 때 각 모니터를 자동으로 시작하므로 Claude에 감시를 시작하도록 지시할 필요가 없습니다.

플러그인 루트에 `monitors/monitors.json` 파일을 추가하고 모니터 항목의 배열을 포함합니다:

```json monitors/monitors.json theme={null}
[
  {
    "name": "error-log",
    "command": "tail -F ./logs/error.log",
    "description": "Application error log"
  }
]
```

`command`의 각 stdout 줄은 세션 중에 Claude에 알림으로 전달됩니다. `when` 트리거 및 변수 대체를 포함한 전체 스키마는 [Monitors](/docs/ko/plugins-reference#monitors)를 참조하세요.

<h3 id="ship-default-settings-with-your-plugin">
  플러그인과 함께 기본 설정 제공
</h3>

플러그인은 플러그인 루트에 `settings.json` 파일을 포함하여 플러그인이 활성화될 때 기본 구성을 적용할 수 있습니다. 현재 `agent` 및 `subagentStatusLine` 키만 지원됩니다.

`agent`를 설정하면 플러그인의 [사용자 정의 agents](/docs/ko/sub-agents) 중 하나를 주 스레드로 활성화하여 시스템 프롬프트, 도구 제한, 모델을 적용합니다. 이를 통해 플러그인은 활성화될 때 Claude Code의 동작 방식을 기본적으로 변경할 수 있습니다.

```json settings.json theme={null}
{
  "agent": "security-reviewer"
}
```

이 예제는 플러그인의 `agents/` 디렉토리에 정의된 `security-reviewer` agent를 활성화합니다. `settings.json`의 설정은 `plugin.json`에 선언된 `settings`보다 우선합니다. 알 수 없는 키는 자동으로 무시됩니다.

<h3 id="organize-complex-plugins">
  복잡한 플러그인 구성
</h3>

많은 구성 요소가 있는 플러그인의 경우 기능별로 디렉토리 구조를 구성합니다. 완전한 디렉토리 레이아웃 및 구성 패턴은 [플러그인 디렉토리 구조](/docs/ko/plugins-reference#plugin-directory-structure)를 참조하세요.

<h3 id="test-your-plugins-locally">
  플러그인을 로컬에서 테스트
</h3>

개발 중에 플러그인을 테스트하려면 `--plugin-dir` 플래그를 사용합니다. 이는 설치를 요구하지 않고 플러그인을 직접 로드합니다.

```bash theme={null}
claude --plugin-dir ./my-plugin
```

플래그는 플러그인 디렉토리의 `.zip` 아카이브도 허용하며, 이는 Claude Code v2.1.128 이상이 필요합니다.

```bash theme={null}
claude --plugin-dir ./my-plugin.zip
```

`--plugin-dir` 플러그인이 설치된 마켓플레이스 플러그인과 동일한 이름을 가진 경우 로컬 복사본이 해당 세션에 우선합니다. 이를 통해 먼저 제거하지 않고도 이미 설치한 플러그인의 변경 사항을 테스트할 수 있습니다. 관리 설정에 의해 강제로 활성화되거나 비활성화된 플러그인은 유일한 예외이며 `--plugin-dir`로 재정의할 수 없습니다.

플러그인을 변경할 때 `/reload-plugins`를 실행하여 다시 시작하지 않고 업데이트를 적용합니다. 이는 플러그인, skills, agents, hooks, 플러그인 MCP servers, 플러그인 LSP servers를 다시 로드합니다. 플러그인 구성 요소를 테스트합니다:

* `/plugin-name:skill-name`으로 skills를 시도해보세요
* agents가 `/context`의 Custom Agents 아래에 나타나는지 확인하거나 범위가 지정된 이름으로 @-mention하세요
* hooks가 예상대로 작동하는지 확인하세요

<Tip>
  플래그를 여러 번 지정하여 한 번에 여러 플러그인을 로드할 수 있습니다:

  ```bash theme={null}
  claude --plugin-dir ./plugin-one --plugin-dir ./plugin-two
  ```
</Tip>

URL에서 호스팅되는 `.zip` 아카이브로 이미 패키징된 플러그인을 테스트하려면(예: CI 빌드 아티팩트) 대신 `--plugin-url`을 사용하세요. Claude Code는 시작 시 아카이브를 가져오고 해당 세션에만 로드합니다. 가져오기가 실패하거나 아카이브가 유효하지 않으면 Claude Code는 플러그인 로드 오류를 보고하고 이를 제외하고 시작합니다. 모든 플러그인 소스에 대해 동일한 [신뢰 고려 사항](/docs/ko/discover-plugins#security)이 적용됩니다: 이 플래그를 제어하거나 신뢰하는 아카이브에만 지정하세요.

여러 플러그인을 로드하려면 각 URL에 대해 플래그를 반복합니다:

```bash theme={null}
claude --plugin-url https://example.com/my-plugin.zip --plugin-url https://example.com/other.zip
```

또는 공백으로 구분된 URL을 하나의 따옴표로 묶인 인수로 전달합니다:

```bash theme={null}
claude --plugin-url "https://example.com/my-plugin.zip https://example.com/other.zip"
```

<h3 id="debug-plugin-issues">
  플러그인 문제 디버깅
</h3>

플러그인이 예상대로 작동하지 않는 경우:

1. **구조 확인**: 디렉토리가 `.claude-plugin/` 내부가 아닌 플러그인 루트에 있는지 확인하세요
2. **구성 요소를 개별적으로 테스트**: 각 skill, agent, hook을 별도로 확인하세요
3. **검증 및 디버깅 도구 사용**: CLI 명령 및 문제 해결 기법은 [디버깅 및 개발 도구](/docs/ko/plugins-reference#debugging-and-development-tools)를 참조하세요

<h3 id="share-your-plugins">
  플러그인 공유
</h3>

플러그인을 공유할 준비가 되면:

1. **문서 추가**: 설치 및 사용 지침이 포함된 `README.md`를 포함하세요
2. **버전 관리 전략 선택**: 명시적 `version`을 설정할지 또는 git 커밋 SHA에 의존할지 결정하세요. [버전 관리](/docs/ko/plugins-reference#version-management)를 참조하세요
3. **마켓플레이스 만들기 또는 사용**: [플러그인 마켓플레이스](/docs/ko/plugin-marketplaces)를 통해 배포하여 설치하세요
4. **다른 사람과 테스트**: 더 광범위한 배포 전에 팀원이 플러그인을 테스트하도록 하세요

플러그인이 마켓플레이스에 있으면 다른 사람들이 [플러그인 발견 및 설치](/docs/ko/discover-plugins)의 지침을 사용하여 설치할 수 있습니다. 플러그인을 팀 내부로만 유지하려면 [비공개 저장소](/docs/ko/plugin-marketplaces#private-repositories)에서 마켓플레이스를 호스팅하세요.

<h3 id="submit-your-plugin-to-the-community-marketplace">
  플러그인을 커뮤니티 마켓플레이스에 제출
</h3>

Anthropic은 Claude Code 플러그인을 위한 두 개의 공개 마켓플레이스를 유지합니다:

* **`claude-plugins-official`**: Anthropic에서 유지 관리하는 엄선된 플러그인 세트입니다. Claude Code를 처음 대화형으로 시작할 때 자동으로 등록됩니다. 첫 번째 시작 전에 실행되는 비대화형 스크립트는 `claude plugin marketplace add anthropics/claude-plugins-official`로 명시적으로 추가해야 합니다.
* **`claude-community`**: 검토 후 타사 제출이 도착하는 공개 커뮤니티 마켓플레이스입니다. 사용자는 `/plugin marketplace add anthropics/claude-plugins-community`로 추가하고 `@claude-community`로 설치합니다.

커뮤니티 마켓플레이스 검토를 위해 플러그인을 제출하려면 다음 앱 내 양식 중 하나를 사용하세요:

* **claude.ai**: [claude.ai/admin-settings/directory/submissions/plugins/new](https://claude.ai/admin-settings/directory/submissions/plugins/new)
* **Console**: [platform.claude.com/plugins/submit](https://platform.claude.com/plugins/submit)

claude.ai 양식은 Team 또는 Enterprise 조직과 디렉토리 관리 액세스가 필요합니다. 조직 소유자는 기본적으로 이 액세스 권한을 가집니다. Team 또는 Enterprise 조직에 속하지 않은 개별 작성자는 대신 Console 양식을 사용할 수 있습니다.

제출하기 전에 로컬에서 `claude plugin validate`를 실행하세요. 검토 파이프라인은 모든 제출에 대해 동일한 검사를 실행하며, 자동화된 안전 검사도 함께 수행합니다.

승인된 플러그인은 [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community) 카탈로그의 특정 커밋 SHA에 고정되며, CI는 저장소에 새 커밋을 푸시할 때 자동으로 핀을 업데이트합니다. 공개 카탈로그는 검토 파이프라인에서 매일 밤 동기화되므로 승인과 플러그인이 `marketplace.json`에 나타나는 사이에 지연이 있을 수 있습니다. 플러그인이 설치 가능한지 확인하려면 [커뮤니티 카탈로그](https://github.com/anthropics/claude-plugins-community/blob/main/.claude-plugin/marketplace.json)에서 이름을 검색하세요.

공식 마켓플레이스인 `claude-plugins-official`은 별도로 엄선됩니다. Anthropic은 자신의 재량에 따라 포함할 플러그인을 결정합니다. 신청 절차가 없으며, 제출 양식은 플러그인을 공식 마켓플레이스에 추가하지 않습니다.

Anthropic이 플러그인을 공식 마켓플레이스에 나열하면 CLI에서 Claude Code 사용자에게 설치를 권장할 수 있습니다. [CLI에서 플러그인 권장](/docs/ko/plugin-hints)을 참조하세요.

<Note>
  완전한 기술 사양, 디버깅 기법, 배포 전략은 [플러그인 참조](/docs/ko/plugins-reference)를 참조하세요.
</Note>

<h2 id="convert-existing-configurations-to-plugins">
  기존 구성을 플러그인으로 변환
</h2>

`.claude/` 디렉토리에 이미 skills 또는 hooks가 있는 경우 더 쉬운 공유 및 배포를 위해 플러그인으로 변환할 수 있습니다.

<h3 id="migration-steps">
  마이그레이션 단계
</h3>

<Steps>
  <Step title="플러그인 구조 만들기">
    프로젝트 루트에 새 플러그인 디렉토리를 만듭니다. 기존 `.claude/` 폴더와 함께 배치하여 다음 단계의 상대 `cp` 경로가 올바르게 해석되도록 합니다:

    ```bash theme={null}
    mkdir -p my-plugin/.claude-plugin
    ```

    `my-plugin/.claude-plugin/plugin.json`에 매니페스트 파일을 만듭니다:

    ```json my-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-plugin",
      "description": "Migrated from standalone configuration",
      "version": "1.0.0"
    }
    ```
  </Step>

  <Step title="기존 파일 복사">
    기존 구성을 플러그인 디렉토리에 복사합니다:

    ```bash theme={null}
    # Copy commands
    cp -r .claude/commands my-plugin/

    # Copy agents (if any)
    cp -r .claude/agents my-plugin/

    # Copy skills (if any)
    cp -r .claude/skills my-plugin/
    ```
  </Step>

  <Step title="Hooks 마이그레이션">
    설정에 hooks가 있는 경우 hooks 디렉토리를 만듭니다:

    ```bash theme={null}
    mkdir my-plugin/hooks
    ```

    `my-plugin/hooks/hooks.json`을 hooks 구성으로 만듭니다. `.claude/settings.json` 또는 `settings.local.json`에서 `hooks` 객체를 복사합니다. 형식이 동일하기 때문입니다. 명령은 stdin에서 JSON으로 hook 입력을 받으므로 `jq`를 사용하여 파일 경로를 추출합니다:

    ```json my-plugin/hooks/hooks.json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [{ "type": "command", "command": "jq -r '.tool_input.file_path' | xargs npm run lint:fix" }]
          }
        ]
      }
    }
    ```
  </Step>

  <Step title="마이그레이션된 플러그인 테스트">
    플러그인을 로드하여 모든 것이 작동하는지 확인합니다:

    ```bash theme={null}
    claude --plugin-dir ./my-plugin
    ```

    각 구성 요소를 테스트합니다: 명령을 실행하고, agents가 `/context`에 나타나는지 확인하고, hooks가 올바르게 트리거되는지 확인합니다.
  </Step>
</Steps>

<h3 id="what-changes-when-migrating">
  마이그레이션 시 변경되는 사항
</h3>

| 독립 실행형 (`.claude/`)     | 플러그인                        |
| :---------------------- | :-------------------------- |
| 한 프로젝트에서만 사용 가능         | 마켓플레이스를 통해 공유 가능            |
| `.claude/commands/`의 파일 | `plugin-name/commands/`의 파일 |
| `settings.json`의 Hooks  | `hooks/hooks.json`의 Hooks   |
| 공유하려면 수동으로 복사해야 함       | `/plugin install`로 설치       |

<Note>
  마이그레이션 후 중복을 피하기 위해 `.claude/`에서 원본 파일을 제거합니다. 프로젝트 및 사용자 `.claude/agents/` 정의는 같은 이름의 플러그인 agents를 재정의하므로, 원본이 제거되면 플러그인 버전만 적용됩니다. 플러그인 skills는 `/plugin-name:skill-name`으로 네임스페이스되므로, 원본 `/skill-name`과 플러그인 복사본이 모두 사용 가능하게 유지되며 하나가 다른 하나를 재정의하지 않습니다.
</Note>

<h2 id="next-steps">
  다음 단계
</h2>

이제 Claude Code의 플러그인 시스템을 이해했으므로 다양한 목표에 대한 제안된 경로는 다음과 같습니다:

<h3 id="for-plugin-users">
  플러그인 사용자의 경우
</h3>

* [플러그인 발견 및 설치](/docs/ko/discover-plugins): 마켓플레이스를 검색하고 플러그인을 설치합니다
* [팀 마켓플레이스 구성](/docs/ko/discover-plugins#configure-team-marketplaces): 팀을 위한 저장소 수준 플러그인을 설정합니다

<h3 id="for-plugin-developers">
  플러그인 개발자의 경우
</h3>

* [마켓플레이스 만들기 및 배포](/docs/ko/plugin-marketplaces): 플러그인을 패키징하고 공유합니다
* [플러그인 참조](/docs/ko/plugins-reference): 완전한 기술 사양
* 특정 플러그인 구성 요소에 대해 더 깊이 있게 살펴보세요:
  * [Skills](/docs/ko/skills): skill 개발 세부 사항
  * [Subagents](/docs/ko/sub-agents): agent 구성 및 기능
  * [Hooks](/docs/ko/hooks): 이벤트 처리 및 자동화
  * [MCP](/docs/ko/mcp): 외부 도구 통합
