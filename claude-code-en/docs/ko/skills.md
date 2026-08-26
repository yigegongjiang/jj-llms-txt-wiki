> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude를 skills로 확장하기

> Claude Code에서 skills를 생성, 관리, 공유하여 Claude의 기능을 확장합니다. 사용자 정의 명령어와 번들 skills를 포함합니다.

Skills는 Claude가 할 수 있는 작업을 확장합니다. `SKILL.md` 파일을 지침과 함께 생성하면 Claude가 이를 자신의 도구 모음에 추가합니다. Claude는 관련이 있을 때 skills를 사용하거나 `/skill-name`으로 직접 호출할 수 있습니다.

같은 지침, 체크리스트 또는 다단계 절차를 계속 채팅에 붙여넣거나, CLAUDE.md의 섹션이 사실이 아닌 절차로 성장했을 때 skill을 생성합니다. CLAUDE.md 콘텐츠와 달리, skill의 본문은 사용할 때만 로드되므로 긴 참조 자료는 필요할 때까지 거의 비용이 들지 않습니다.

<Note>
  `/help` 및 `/compact`와 같은 기본 제공 명령어와 `/debug` 및 `/code-review`와 같은 번들 skills는 [명령어 참조](/docs/ko/commands)를 참조하세요.

  **사용자 정의 명령어가 skills로 병합되었습니다.** `.claude/commands/deploy.md`의 파일과 `.claude/skills/deploy/SKILL.md`의 skill은 모두 `/deploy`를 생성하고 동일하게 작동합니다. 기존 `.claude/commands/` 파일은 계속 작동합니다. Skills는 선택적 기능을 추가합니다: 지원 파일을 위한 디렉토리, [skill을 누가 호출하는지 제어](#control-who-invokes-a-skill)하기 위한 frontmatter, 그리고 Claude가 관련이 있을 때 자동으로 로드할 수 있는 기능입니다.
</Note>

Claude Code skills는 [Agent Skills](https://agentskills.io) 개방형 표준을 따르며, 이는 여러 AI 도구에서 작동합니다. Claude Code는 [호출 제어](#control-who-invokes-a-skill), [subagent 실행](#run-skills-in-a-subagent), [동적 컨텍스트 주입](#inject-dynamic-context)과 같은 추가 기능으로 표준을 확장합니다.

<h2 id="bundled-skills">
  번들 skills
</h2>

Claude Code에는 모든 세션에서 사용 가능한 번들 skills 세트가 포함되어 있으며, [`disableBundledSkills`](/docs/ko/settings#available-settings) 설정으로 비활성화하지 않는 한 `/doctor`, `/code-review`, `/batch`, `/debug`, `/loop`, `/claude-api`를 포함합니다. 고정 로직을 직접 실행하는 대부분의 기본 제공 명령어와 달리, 번들 skills는 프롬프트 기반입니다: Claude에 상세한 지시사항을 제공하고 도구를 사용하여 작업을 조율하도록 합니다. 다른 skill과 동일한 방식으로 호출합니다: `/` 다음에 skill 이름을 입력합니다.

[`/doctor`](/docs/ko/commands#all-commands) 설정 점검은 Claude Code v2.1.205 이상에서 `disableBundledSkills`의 예외입니다: 설정이 켜져 있을 때도 입력 가능합니다. 이를 숨기려면 `DISABLE_DOCTOR_COMMAND` 환경 변수를 설정하거나 [`skillOverrides`](#override-skill-visibility-from-settings) 항목에서 `"doctor": "off"`를 설정합니다. v2.1.205 이전에는 `/doctor`가 번들 skill이 아닌 기본 제공 명령어였습니다.

번들 skills는 [명령어 참조](/docs/ko/commands)에 나열되어 있으며, 목적 열에 **Skill**로 표시됩니다.

<h3 id="run-and-verify-your-app">
  앱 실행 및 확인
</h3>

세 가지 번들 skills는 함께 작동하여 앱을 시작하고 테스트만이 아닌 실행 중인 앱에 대해 변경 사항을 확인합니다:

| Skill                  | 목적                                                           |
| :--------------------- | :----------------------------------------------------------- |
| `/run`                 | 앱을 시작하고 변경 사항이 작동하는지 확인하기 위해 앱을 실행합니다                        |
| `/verify`              | 앱을 빌드하고 실행하여 코드 변경이 의도한 대로 작동하는지 확인하며, 테스트나 타입 체크로 돌아가지 않습니다 |
| `/run-skill-generator` | `/run`과 `/verify`에 프로젝트를 빌드하고 시작하는 방법을 가르칩니다                 |

세 가지 skills 모두 Claude Code v2.1.145 이상이 필요합니다.

`/run`과 `/verify`는 설정 없이 작동합니다. 프로젝트 유형(CLI, 서버, TUI, 브라우저 기반)과 README, `package.json` 또는 `Makefile`의 내용으로부터 시작을 추론합니다. 이 추론은 표준 시작 이상의 것이 필요한 프로젝트(데이터베이스, env 파일, 그래픽 세션, 다단계 빌드)에 대해서는 신뢰할 수 없게 됩니다.

`/run-skill-generator`는 대신 레시피를 기록합니다. 깨끗한 환경에서 앱을 실행하고, 작동한 것(설치 명령어, env 변수, 시작 스크립트)을 캡처하고, 프로젝트별 skill로 `.claude/skills/run-<name>/`에 커밋합니다. 그 후 `/run`, `/verify` 및 리포지토리의 다른 모든 에이전트는 레시피를 다시 발견하는 대신 기록된 레시피를 따릅니다. 프로젝트당 한 번 `/run-skill-generator`를 실행하고, 빌드 또는 시작 프로세스가 변경되면 다시 실행합니다.

<h2 id="getting-started">
  시작하기
</h2>

<h3 id="create-your-first-skill">
  첫 번째 skill 생성
</h3>

이 예제는 git 저장소의 커밋되지 않은 변경 사항을 요약하고 위험한 항목을 표시하는 skill을 생성합니다. 라이브 diff를 Claude가 읽기 전에 프롬프트로 가져오므로, 응답은 Claude가 열린 파일에서 추측할 수 있는 것이 아니라 실제 작업 트리에 기반합니다. Claude는 변경 사항에 대해 물어볼 때 자동으로 skill을 로드하거나 `/summarize-changes`로 직접 호출할 수 있습니다.

<Steps>
  <Step title="skill 디렉토리 생성">
    개인 skills 폴더에 skill을 위한 디렉토리를 생성합니다. 개인 skills는 모든 프로젝트에서 사용 가능합니다.

    ```bash theme={null}
    mkdir -p ~/.claude/skills/summarize-changes
    ```
  </Step>

  <Step title="SKILL.md 작성">
    모든 skill에는 두 부분이 있는 `SKILL.md` 파일이 필요합니다: Claude에게 skill을 언제 사용할지 알려주는 YAML frontmatter (`---` 마커 사이)와 skill이 실행될 때 Claude가 따르는 지침이 있는 markdown 콘텐츠입니다. 디렉토리 이름이 입력하는 명령어가 되고, `description`은 Claude가 자동으로 skill을 로드할 시기를 결정하는 데 도움이 됩니다.

    이를 `~/.claude/skills/summarize-changes/SKILL.md`에 저장합니다:

    ```yaml theme={null}
    ---
    description: Summarizes uncommitted changes and flags anything risky. Use when the user asks what changed, wants a commit message, or asks to review their diff.
    ---

    ## Current changes

    !`git diff HEAD`

    ## Instructions

    Summarize the changes above in two or three bullet points, then list any risks you notice such as missing error handling, hardcoded values, or tests that need updating. If the diff is empty, say there are no uncommitted changes.
    ```

    `` !`git diff HEAD` `` 줄은 [동적 컨텍스트 주입](#inject-dynamic-context)을 사용합니다: Claude Code는 명령어를 실행하고 Claude가 skill 콘텐츠를 보기 전에 줄을 출력으로 바꾸므로, 지침은 현재 diff가 이미 인라인된 상태로 도착합니다.
  </Step>

  <Step title="skill 테스트">
    git 프로젝트를 열고, 파일을 약간 편집한 후, `claude`를 실행하여 Claude Code를 시작합니다. 두 가지 방법으로 skill을 테스트할 수 있습니다.

    **Claude가 자동으로 호출하도록 하기** - 설명과 일치하는 항목을 물어봅니다:

    ```text theme={null}
    What did I change?
    ```

    **또는 skill 이름으로 직접 호출하기**:

    ```text theme={null}
    /summarize-changes
    ```

    어느 쪽이든 Claude는 편집의 짧은 요약과 위험 목록으로 응답해야 합니다.
  </Step>
</Steps>

<h3 id="where-skills-live">
  Skills가 있는 위치
</h3>

skill을 저장하는 위치에 따라 누가 사용할 수 있는지가 결정됩니다:

| 위치         | 경로                                       | 적용 대상         |
| :--------- | :--------------------------------------- | :------------ |
| Enterprise | [관리 설정](/docs/ko/settings#settings-files) 참조  | 조직의 모든 사용자    |
| Personal   | `~/.claude/skills/<skill-name>/SKILL.md` | 모든 프로젝트       |
| Project    | `.claude/skills/<skill-name>/SKILL.md`   | 이 프로젝트만       |
| Plugin     | `<plugin>/skills/<skill-name>/SKILL.md`  | 플러그인이 활성화된 위치 |

skills가 여러 수준에서 같은 이름을 공유할 때, enterprise가 personal을 재정의하고, personal이 project를 재정의합니다. 예를 들어, 프로젝트의 `.claude/skills/`에 있는 `code-review` skill은 번들된 `/code-review`를 대체합니다. Plugin skills는 `plugin-name:skill-name` 네임스페이스를 사용하므로 다른 수준과 충돌할 수 없습니다. `.claude/commands/`에 파일이 있으면 동일한 방식으로 작동하지만, skill과 명령어가 같은 이름을 공유하면 skill이 우선합니다.

Skills는 또한 작업 디렉토리 아래의 중첩된 `.claude/skills/` 디렉토리에서 로드됩니다. Claude가 하위 디렉토리의 파일을 읽거나 편집할 때, 해당 하위 디렉토리의 `.claude/skills/`에 있는 skills가 사용 가능해집니다. 이를 통해 monorepo 패키지가 자신의 skills를 제공할 수 있으며, 세션이 저장소 루트에서 시작되었더라도 해당 패키지에서 작업할 때 적용됩니다.

중첩된 skill이 다른 skill과 같은 이름을 공유하면, 둘 다 사용 가능합니다. 예를 들어, 프로젝트 루트에 `deploy` skill이 있고 `apps/web/.claude/skills/`에 다른 skill이 있는 경우:

* 중첩된 skill은 디렉토리 한정 이름 `apps/web:deploy` 아래에 나타납니다.
* 해당 설명은 어느 디렉토리에 적용되는지 나타냅니다.
* Claude는 작업 중인 파일과 일치하는 변형을 선택합니다.

`/deploy`를 입력하면 프로젝트 루트 skill이 실행됩니다. 중첩된 변형을 명시적으로 실행하려면 한정된 이름 `/apps/web:deploy`를 입력합니다.

한정되지 않은 이름을 호출하거나 Claude가 호출할 때, 프로젝트 루트 skill이 로드되고, Claude Code는 디렉토리 한정 변형 목록을 해당 콘텐츠에 추가하며, Claude가 작업 중인 파일을 보유한 변형도 호출하도록 지시합니다. 따라서 중첩된 skill은 한정되지 않은 이름만 호출할 때도 해당 디렉토리의 작업에 계속 적용됩니다. Claude Code v2.1.203 이상이 필요합니다.

`<skill-name>` 항목이 enterprise, personal 또는 project 위치에 있을 수 있으며, 디스크의 다른 곳에 있는 디렉토리로의 symlink일 수 있습니다. Claude Code는 symlink를 따라가고 대상 디렉토리에서 `SKILL.md`를 읽으며, 같은 대상에 둘 이상의 위치에서 도달할 수 있으면 Claude Code는 skill을 한 번만 로드합니다. Plugin skills는 symlinks를 다르게 처리합니다. [마켓플레이스 내에서 symlinks를 사용하여 파일 공유](/docs/ko/plugins-reference#share-files-within-a-marketplace-with-symlinks)를 참조하세요.

<Note>
  `.claude-plugin/plugin.json`을 skill 폴더에 추가하면 `<name>@skills-dir`이라는 [플러그인](/docs/ko/plugins-reference#skills-directory-plugins)으로 로드되므로, agents, hooks 및 MCP 서버를 번들로 제공할 수 있습니다. 프로젝트의 `.claude/skills/`에서는 먼저 작업 공간 신뢰 대화를 수락해야 합니다.
</Note>

<h4 id="live-change-detection">
  라이브 변경 감지
</h4>

Claude Code는 skill 디렉토리의 파일 변경을 감시합니다. `~/.claude/skills/`, 프로젝트 `.claude/skills/`, 또는 `--add-dir` 디렉토리 내의 `.claude/skills/` 아래에서 skill을 추가, 편집 또는 제거하면 Claude Code를 다시 시작하지 않고도 현재 세션 내에서 적용됩니다. 세션이 시작되었을 때 존재하지 않았던 최상위 skills 디렉토리를 생성하려면 Claude Code를 다시 시작해야 새 디렉토리를 감시할 수 있습니다.

<Note>
  라이브 변경 감지는 `SKILL.md` 텍스트만 포함합니다. skill 폴더가 [플러그인](/docs/ko/plugins-reference#skills-directory-plugins)이기도 한 경우, `hooks/`, `.mcp.json`, `agents/` 및 `output-styles/`의 변경 사항은 `/reload-plugins`를 실행해야 적용됩니다.
</Note>

<h4 id="automatic-discovery-from-parent-and-nested-directories">
  상위 및 중첩된 디렉토리에서 자동 검색
</h4>

프로젝트 skills는 시작 디렉토리의 `.claude/skills/`와 저장소 루트까지의 모든 상위 디렉토리에서 로드되므로, 하위 디렉토리에서 Claude를 시작해도 루트에서 정의된 skills를 선택합니다. 시작 디렉토리 아래의 하위 디렉토리에 있는 파일로 작업할 때, Claude Code는 필요에 따라 중첩된 `.claude/skills/` 디렉토리에서 skills를 검색합니다. 예를 들어, `packages/frontend/`의 파일을 편집하는 경우, Claude Code는 `packages/frontend/.claude/skills/`에서도 skills를 찾습니다. 이는 패키지가 자신의 skills를 가진 monorepo 설정을 지원합니다.

각 skill은 `SKILL.md`를 진입점으로 하는 디렉토리입니다:

```text theme={null}
my-skill/
├── SKILL.md           # 주요 지침 (필수)
├── template.md        # Claude가 채울 템플릿
├── examples/
│   └── sample.md      # 예상 형식을 보여주는 예제 출력
└── scripts/
    └── validate.sh    # Claude가 실행할 수 있는 스크립트
```

`SKILL.md`는 주요 지침을 포함하며 필수입니다. 다른 파일은 선택적이며 더 강력한 skills를 구축할 수 있습니다: Claude가 채울 템플릿, 예상 형식을 보여주는 예제 출력, Claude가 실행할 수 있는 스크립트 또는 상세한 참조 문서. `SKILL.md`에서 이러한 파일을 참조하여 Claude가 각 파일의 내용과 로드 시기를 알 수 있도록 합니다. 자세한 내용은 [지원 파일 추가](#add-supporting-files)를 참조하세요.

<Note>
  `.claude/commands/`의 파일은 계속 작동하며 동일한 [frontmatter](#frontmatter-reference)를 지원합니다. Skills는 지원 파일과 같은 추가 기능을 지원하므로 권장됩니다.
</Note>

<h4 id="skills-from-additional-directories">
  추가 디렉토리의 Skills
</h4>

`--add-dir` 플래그와 `/add-dir` 명령어는 [파일 액세스를 부여](/docs/ko/permissions#additional-directories-grant-file-access-not-configuration)하지만 구성 검색은 하지 않습니다. 그러나 skills는 예외입니다: 추가된 디렉토리 내의 `.claude/skills/`는 자동으로 로드됩니다. 이 예외는 `--add-dir`과 `/add-dir`에만 적용됩니다. `settings.json`의 `permissions.additionalDirectories` 설정은 파일 액세스만 부여하며 skills를 로드하지 않습니다. [라이브 변경 감지](#live-change-detection)를 참조하여 세션 중에 편집이 어떻게 선택되는지 확인하세요.

다른 `.claude/` 구성(예: 명령어 및 출력 스타일)은 추가 디렉토리에서 로드되지 않습니다. 로드되는 항목과 로드되지 않는 항목의 전체 목록과 프로젝트 간 구성을 공유하는 권장 방법은 [예외 표](/docs/ko/permissions#additional-directories-grant-file-access-not-configuration)를 참조하세요.

<Note>
  `--add-dir` 디렉토리의 CLAUDE.md 파일은 기본적으로 로드되지 않습니다. 로드하려면 `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1`을 설정하세요. [추가 디렉토리에서 로드](/docs/ko/memory#load-from-additional-directories)를 참조하세요.
</Note>

<h2 id="configure-skills">
  Skills 구성
</h2>

Skills는 `SKILL.md` 상단의 YAML frontmatter와 그 뒤의 markdown 콘텐츠를 통해 구성됩니다.

<h3 id="types-of-skill-content">
  Skill 콘텐츠 유형
</h3>

Skill 파일은 모든 지침을 포함할 수 있지만, 호출 방식을 생각하면 포함할 내용을 안내하는 데 도움이 됩니다:

**참조 콘텐츠**는 Claude가 현재 작업에 적용하는 지식을 추가합니다. 규칙, 패턴, 스타일 가이드, 도메인 지식. 이 콘텐츠는 인라인으로 실행되므로 Claude가 대화 컨텍스트와 함께 사용할 수 있습니다.

```yaml theme={null}
---
name: api-conventions
description: API design patterns for this codebase
---

When writing API endpoints:
- Use RESTful naming conventions
- Return consistent error formats
- Include request validation
```

**작업 콘텐츠**는 배포, 커밋 또는 코드 생성과 같은 특정 작업에 대한 단계별 지침을 제공합니다. 이는 Claude가 자동으로 실행하도록 하기보다는 `/skill-name`으로 직접 호출하려는 작업입니다. `disable-model-invocation: true`를 추가하여 Claude가 자동으로 트리거하는 것을 방지합니다.

```yaml theme={null}
---
name: deploy
description: Deploy the application to production
context: fork
disable-model-invocation: true
---

Deploy the application:
1. Run the test suite
2. Build the application
3. Push to the deployment target
```

`SKILL.md`는 모든 것을 포함할 수 있지만, skill을 호출하는 방식(사용자, Claude 또는 둘 다)과 실행 위치(인라인 또는 subagent)를 생각하면 포함할 내용을 안내하는 데 도움이 됩니다. 복잡한 skills의 경우, [지원 파일을 추가](#add-supporting-files)하여 주요 skill을 집중적으로 유지할 수도 있습니다.

본문 자체는 간결하게 유지합니다. Skill이 로드되면, 그 콘텐츠는 [턴 전체에 걸쳐 컨텍스트에 유지](#skill-content-lifecycle)되므로, 모든 줄이 반복되는 토큰 비용입니다. 어떻게 또는 왜인지 설명하기보다는 무엇을 할지 명시하고, [CLAUDE.md 콘텐츠](/docs/ko/best-practices#write-an-effective-claude-md)에 적용할 동일한 간결성 테스트를 적용합니다.

<h3 id="frontmatter-reference">
  Frontmatter 참조
</h3>

markdown 콘텐츠 외에도, `SKILL.md` 파일 상단의 `---` 마커 사이의 YAML frontmatter 필드를 사용하여 skill 동작을 구성할 수 있습니다:

```yaml theme={null}
---
name: my-skill
description: What this skill does
disable-model-invocation: true
allowed-tools: Read Grep
---

Your skill instructions here...
```

모든 필드는 선택적입니다. Claude가 skill을 언제 사용할지 알 수 있도록 `description`만 권장됩니다.

| 필드                         | 필수  | 설명                                                                                                                                                                                                                                                                                 |
| :------------------------- | :-- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                     | 아니오 | skill 목록에 표시되는 표시 이름입니다. 디렉토리 이름으로 기본값이 설정됩니다. skill을 호출하기 위해 입력하는 이름과 어떻게 다른지는 [skill이 명령어 이름을 얻는 방법](#how-a-skill-gets-its-command-name)을 참조하세요.                                                                                                                                 |
| `description`              | 권장  | skill이 무엇을 하는지, 언제 사용할지. Claude는 이를 사용하여 skill을 자동으로 적용할 시기를 결정합니다. 생략하면 markdown 콘텐츠의 첫 번째 단락을 사용합니다. 주요 사용 사례를 앞에 배치합니다: 결합된 `description` 및 `when_to_use` 텍스트는 컨텍스트 사용을 줄이기 위해 skill 목록에서 1,536자로 잘립니다.                                                                         |
| `when_to_use`              | 아니오 | Claude가 skill을 호출해야 할 때에 대한 추가 컨텍스트(예: 트리거 구문 또는 예제 요청). skill 목록에서 `description`에 추가되며 1,536자 제한에 포함됩니다.                                                                                                                                                                          |
| `argument-hint`            | 아니오 | 예상 인수를 나타내기 위해 자동 완성 중에 표시되는 힌트. 예: `[issue-number]` 또는 `[filename] [format]`.                                                                                                                                                                                                     |
| `arguments`                | 아니오 | skill 콘텐츠에서 [`$name` 치환](#available-string-substitutions)을 위한 명명된 위치 인수. 공백으로 구분된 문자열 또는 YAML 목록을 허용합니다. 이름은 순서대로 인수 위치에 매핑됩니다.                                                                                                                                                    |
| `disable-model-invocation` | 아니오 | Claude가 이 skill을 자동으로 로드하는 것을 방지하려면 `true`로 설정합니다. `/name`으로 수동으로 트리거하려는 워크플로우에 사용합니다. 또한 skill이 [subagents에 미리 로드되는 것](/docs/ko/sub-agents#preload-skills-into-subagents)을 방지합니다. v2.1.196부터는 [예약된 작업](/docs/ko/scheduled-tasks)이 skill을 프롬프트로 하여 실행될 때 skill이 실행되는 것도 방지합니다. 기본값: `false`. |
| `user-invocable`           | 아니오 | `/` 메뉴에서 숨기려면 `false`로 설정합니다. 사용자가 직접 호출하지 않아야 하는 배경 지식에 사용합니다. 기본값: `true`.                                                                                                                                                                                                       |
| `allowed-tools`            | 아니오 | 이 skill이 활성화되었을 때 Claude가 권한을 요청하지 않고 사용할 수 있는 도구. 공백 또는 쉼표로 구분된 문자열 또는 YAML 목록을 허용합니다.                                                                                                                                                                                            |
| `disallowed-tools`         | 아니오 | 이 skill이 활성화되었을 때 Claude의 사용 가능한 도구 풀에서 제거되는 도구. `AskUserQuestion`과 같이 배경 루프에 대해 특정 도구를 호출하지 않아야 하는 자율 skills에 사용합니다. 공백 또는 쉼표로 구분된 문자열 또는 YAML 목록을 허용합니다. 다음 메시지를 보낼 때 제한이 해제됩니다.                                                                                                 |
| `model`                    | 아니오 | 이 skill이 활성화되었을 때 사용할 모델. 재정의는 현재 턴의 나머지 부분에 적용되며 설정에 저장되지 않습니다. 다음 프롬프트에서 세션 모델이 재개됩니다. [`/model`](/docs/ko/model-config)과 동일한 값을 허용하거나 활성 모델을 유지하려면 `inherit`을 허용합니다. 조직의 [`availableModels`](/docs/ko/model-config#restrict-model-selection) 허용 목록에서 제외된 값은 사용되지 않으며 세션은 현재 모델을 유지합니다.    |
| `effort`                   | 아니오 | [노력 수준](/docs/ko/model-config#adjust-effort-level) - 이 skill이 활성화되었을 때. 세션 노력 수준을 재정의합니다. 기본값: 세션에서 상속. 옵션: `low`, `medium`, `high`, `xhigh`, `max`; 사용 가능한 수준은 모델에 따라 다릅니다.                                                                                                            |
| `context`                  | 아니오 | forked subagent 컨텍스트에서 실행하려면 `fork`로 설정합니다.                                                                                                                                                                                                                                        |
| `agent`                    | 아니오 | `context: fork`가 설정되었을 때 사용할 subagent 유형.                                                                                                                                                                                                                                          |
| `hooks`                    | 아니오 | 이 skill의 라이프사이클에 범위가 지정된 hooks. 구성 형식은 [Skills 및 agents의 Hooks](/docs/ko/hooks#hooks-in-skills-and-agents)를 참조하세요.                                                                                                                                                                      |
| `paths`                    | 아니오 | 이 skill이 활성화되는 시기를 제한하는 Glob 패턴. 쉼표로 구분된 문자열 또는 YAML 목록을 허용합니다. 설정하면 Claude는 패턴과 일치하는 파일로 작업할 때만 자동으로 skill을 로드합니다. [경로별 규칙](/docs/ko/memory#path-specific-rules)과 동일한 형식을 사용합니다.                                                                                                       |
| `shell`                    | 아니오 | 이 skill의 `` !`command` `` 및 ` ```! ` 블록에 사용할 shell. `bash`(기본값) 또는 `powershell`을 허용합니다. `powershell`을 설정하면 Windows에서 PowerShell을 통해 인라인 shell 명령어를 실행합니다. `CLAUDE_CODE_USE_POWERSHELL_TOOL=1`이 필요합니다.                                                                              |

<h4 id="how-a-skill-gets-its-command-name">
  Skill이 명령어 이름을 얻는 방법
</h4>

skill을 호출하기 위해 입력하는 명령어는 skill 파일이 있는 위치에서 나옵니다. frontmatter `name` 필드는 skill 목록에 표시되는 표시 레이블을 설정하며, plugin 루트 `SKILL.md`를 제외하고는 `/` 뒤에 입력하는 내용을 변경하지 않습니다.

아래 표는 각 레이아웃에 대해 명령어 이름이 어디에서 나오는지 보여줍니다:

| Skill 위치                                                              | 명령어 이름 소스                                      | 예제                                                                                                                    |
| :-------------------------------------------------------------------- | :--------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------- |
| `~/.claude/skills/` 또는 `.claude/skills/` 아래의 Skill 디렉토리               | 디렉토리 이름                                        | `.claude/skills/deploy-staging/SKILL.md` → `/deploy-staging`                                                          |
| [중첩된](#where-skills-live) `.claude/skills/` 디렉토리, 다른 skill과 이름이 충돌할 때 | 작업 디렉토리를 기준으로 한 하위 디렉토리 경로, 그 다음 skill 디렉토리 이름 | `apps/web/.claude/skills/deploy/SKILL.md` → `/apps/web:deploy`                                                        |
| `.claude/commands/` 아래의 파일                                            | 확장자 없는 파일 이름                                   | `.claude/commands/deploy.md` → `/deploy`                                                                              |
| Plugin `skills/` 하위 디렉토리                                              | 디렉토리 이름, plugin으로 네임스페이스됨                      | `my-plugin/skills/review/SKILL.md` → `/my-plugin:review`                                                              |
| Plugin 루트 `SKILL.md`                                                  | Frontmatter `name`, plugin 디렉토리 이름을 폴백으로 사용    | `my-plugin/SKILL.md`에서 `name: review` → `/my-plugin:review`. [경로 동작 규칙](/docs/ko/plugins-reference#path-behavior-rules) 참조 |

plugin 루트 경우는 `name`이 명령어 이름을 설정하는 유일한 경우입니다. skill 디렉토리가 없기 때문입니다. frontmatter에서 `name`이 설정되지 않으면 plugin의 디렉토리 이름이 대신 사용됩니다.

<h4 id="available-string-substitutions">
  사용 가능한 문자열 치환
</h4>

Skills는 skill 콘텐츠의 동적 값에 대한 문자열 치환을 지원합니다:

| 변수                      | 설명                                                                                                                                                                                                                           |
| :---------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `$ARGUMENTS`            | skill을 호출할 때 전달된 모든 인수. `$ARGUMENTS`가 콘텐츠에 없으면 인수가 `ARGUMENTS: <value>`로 추가됩니다.                                                                                                                                              |
| `$ARGUMENTS[N]`         | 0 기반 인덱스로 특정 인수에 액세스합니다(예: `$ARGUMENTS[0]`은 첫 번째 인수).                                                                                                                                                                        |
| `$N`                    | `$ARGUMENTS[N]`의 약자(예: `$0`은 첫 번째 인수, `$1`은 두 번째 인수).                                                                                                                                                                        |
| `$name`                 | [`arguments`](#frontmatter-reference) frontmatter 목록에서 선언된 명명된 인수. 이름은 순서대로 위치에 매핑되므로, `arguments: [issue, branch]`를 사용하면 플레이스홀더 `$issue`는 첫 번째 인수로 확장되고 `$branch`는 두 번째 인수로 확장됩니다.                                          |
| `${CLAUDE_SESSION_ID}`  | 현재 세션 ID. 로깅, 세션별 파일 생성 또는 skill 출력을 세션과 연관시키는 데 유용합니다.                                                                                                                                                                      |
| `${CLAUDE_EFFORT}`      | 현재 노력 수준: `low`, `medium`, `high`, `xhigh`, 또는 `max`. Ultracode는 별개의 수준이 아니며 `xhigh`로 보고됩니다. 이를 사용하여 활성 노력 설정에 맞게 skill 지침을 조정합니다.                                                                                           |
| `${CLAUDE_SKILL_DIR}`   | skill의 `SKILL.md` 파일을 포함하는 디렉토리. plugin skills의 경우, 이는 plugin 루트가 아닌 plugin 내의 skill 하위 디렉토리입니다. bash 주입 명령어에서 현재 작업 디렉토리와 관계없이 skill과 함께 번들된 스크립트 또는 파일을 참조하는 데 사용합니다.                                                      |
| `${CLAUDE_PROJECT_DIR}` | 프로젝트 루트 디렉토리. 이는 [hooks](/docs/ko/hooks#reference-scripts-by-path)와 MCP 서버가 `CLAUDE_PROJECT_DIR`로 받는 것과 동일한 경로입니다. 프로젝트 로컬 스크립트 또는 파일(예: `${CLAUDE_PROJECT_DIR}/.claude/hooks/helper.sh`)을 참조하는 데 사용하여 skill이 설치된 위치와 관계없이 사용합니다. |

`${CLAUDE_PROJECT_DIR}` 치환은 Claude Code v2.1.196 이상이 필요합니다. skill 본문과 [`allowed-tools`](#frontmatter-reference) frontmatter 모두에 적용되므로, `Bash(${CLAUDE_PROJECT_DIR}/scripts/lint.sh *)` 같은 권한 규칙은 skill 본문이 사용하는 것과 동일한 경로로 확인됩니다.

인덱싱된 인수는 shell 스타일 인용을 사용하므로 다중 단어 값을 따옴표로 감싸서 단일 인수로 전달합니다. 예를 들어, `/my-skill "hello world" second`는 `$0`을 `hello world`로, `$1`을 `second`로 확장합니다. `$ARGUMENTS` 플레이스홀더는 항상 입력한 전체 인수 문자열로 확장됩니다.

리터럴 `$`를 숫자, `ARGUMENTS` 또는 선언된 인수 이름 앞에 포함하려면(예: 산문에서 `$1.00`), 백슬래시로 이스케이프합니다: `\$1.00`. 다른 `$` 앞의 백슬래시는 변경되지 않습니다. 토큰 바로 앞의 단일 백슬래시만 이스케이프합니다. `\\$1`과 같은 이중 백슬래시는 두 백슬래시를 제자리에 두고, `$1`은 여전히 인수 값으로 확장됩니다.

**치환을 사용한 예제:**

```yaml theme={null}
---
name: session-logger
description: Log activity for this session
---

Log the following to logs/${CLAUDE_SESSION_ID}.log:

$ARGUMENTS
```

<h3 id="add-supporting-files">
  지원 파일 추가
</h3>

Skills는 디렉토리에 여러 파일을 포함할 수 있습니다. 이는 `SKILL.md`를 필수 항목에 집중하게 하면서 Claude가 필요할 때만 상세한 참조 자료에 액세스할 수 있게 합니다. 큰 참조 문서, API 사양 또는 예제 컬렉션은 skill이 실행될 때마다 컨텍스트에 로드될 필요가 없습니다.

```text theme={null}
my-skill/
├── SKILL.md (required - overview and navigation)
├── reference.md (detailed API docs - loaded when needed)
├── examples.md (usage examples - loaded when needed)
└── scripts/
    └── helper.py (utility script - executed, not loaded)
```

`SKILL.md`에서 지원 파일을 참조하여 Claude가 각 파일의 내용과 로드 시기를 알 수 있도록 합니다:

```markdown theme={null}
## Additional resources

- For complete API details, see [reference.md](reference.md)
- For usage examples, see [examples.md](examples.md)
```

<Tip>`SKILL.md`를 500줄 이하로 유지합니다. 상세한 참조 자료를 별도 파일로 이동합니다.</Tip>

<h3 id="control-who-invokes-a-skill">
  Skill을 호출하는 사람 제어
</h3>

기본적으로 사용자와 Claude 모두 모든 skill을 호출할 수 있습니다. `/skill-name`을 입력하여 직접 호출할 수 있고, Claude는 대화와 관련이 있을 때 자동으로 로드할 수 있습니다. 두 frontmatter 필드를 사용하여 이를 제한할 수 있습니다:

* **`disable-model-invocation: true`**: 사용자만 skill을 호출할 수 있습니다. 부작용이 있거나 타이밍을 제어하려는 워크플로우(예: `/commit`, `/deploy` 또는 `/send-slack-message`)에 사용합니다. Claude가 코드가 준비된 것처럼 보인다고 해서 배포하기로 결정하지 않기를 원합니다.

* **`user-invocable: false`**: Claude만 skill을 호출할 수 있습니다. 명령어로 실행할 수 없는 배경 지식에 사용합니다. `legacy-system-context` skill은 오래된 시스템이 어떻게 작동하는지 설명합니다. Claude는 관련이 있을 때 이를 알아야 하지만, `/legacy-system-context`는 사용자가 취할 의미 있는 작업이 아닙니다.

이 예제는 사용자만 트리거할 수 있는 배포 skill을 생성합니다. `disable-model-invocation: true` 필드는 Claude가 자동으로 실행하는 것을 방지합니다:

```yaml theme={null}
---
name: deploy
description: Deploy the application to production
disable-model-invocation: true
---

Deploy $ARGUMENTS to production:

1. Run the test suite
2. Build the application
3. Push to the deployment target
4. Verify the deployment succeeded
```

두 필드가 호출 및 컨텍스트 로딩에 미치는 영향은 다음과 같습니다:

| Frontmatter                      | 사용자가 호출 가능 | Claude가 호출 가능 | 컨텍스트에 로드되는 시기                          |
| :------------------------------- | :--------- | :------------ | :------------------------------------- |
| (기본값)                            | 예          | 예             | 설명은 항상 컨텍스트에 있고, 호출될 때 전체 skill이 로드됨   |
| `disable-model-invocation: true` | 예          | 아니오           | 설명은 컨텍스트에 없고, 사용자가 호출할 때 전체 skill이 로드됨 |
| `user-invocable: false`          | 아니오        | 예             | 설명은 항상 컨텍스트에 있고, 호출될 때 전체 skill이 로드됨   |

<Note>
  일반 세션에서 skill 설명은 Claude가 사용 가능한 항목을 알 수 있도록 컨텍스트에 로드되지만, 전체 skill 콘텐츠는 호출될 때만 로드됩니다. [미리 로드된 skills가 있는 Subagents](/docs/ko/sub-agents#preload-skills-into-subagents)는 다르게 작동합니다: 전체 skill 콘텐츠는 시작 시 주입됩니다.
</Note>

<h3 id="skill-content-lifecycle">
  Skill 콘텐츠 라이프사이클
</h3>

사용자 또는 Claude가 skill을 호출하면, 렌더링된 `SKILL.md` 콘텐츠는 대화에 단일 메시지로 들어가고 세션의 나머지 부분 동안 그대로 유지됩니다. Claude Code는 나중의 턴에서 skill 파일을 다시 읽지 않으므로, 작업 전체에 적용되어야 하는 지침을 일회성 단계가 아닌 상시 지침으로 작성합니다.

렌더링된 콘텐츠가 이미 컨텍스트에 있는 복사본과 동일한 skill을 Claude가 다시 호출할 때, Claude Code는 콘텐츠의 두 번째 복사본 대신 skill이 이미 로드되었다는 짧은 메모를 추가합니다. 렌더링된 콘텐츠가 다를 때(인수가 변경되었거나 [동적 컨텍스트](#inject-dynamic-context) 명령어가 새로운 출력을 생성했기 때문에), Claude Code는 전체 콘텐츠를 다시 추가합니다. v2.1.202 이전에는 모든 재호출이 skill의 지침의 또 다른 전체 복사본을 추가했습니다.

[자동 압축](/docs/ko/how-claude-code-works#when-context-fills-up)은 토큰 예산 내에서 호출된 skills를 전달합니다. 대화가 요약되어 컨텍스트를 확보하면, Claude Code는 요약 후 각 skill의 가장 최근 호출을 다시 첨부하여 처음 5,000토큰을 유지합니다. 다시 첨부된 skills는 25,000토큰의 결합 예산을 공유합니다. Claude Code는 가장 최근에 호출된 skill부터 시작하여 이 예산을 채우므로, 한 세션에서 많은 skills를 호출한 경우 압축 후 이전 skills가 완전히 삭제될 수 있습니다.

skill이 첫 번째 응답 후 동작에 영향을 미치지 않는 것처럼 보이면, 콘텐츠는 일반적으로 여전히 존재하며 모델이 다른 도구나 접근 방식을 선택하고 있습니다. skill의 `description` 및 지침을 강화하여 모델이 계속 선호하도록 하거나, [hooks](/docs/ko/hooks)를 사용하여 동작을 결정론적으로 적용합니다. skill이 크거나 그 후에 다른 여러 skills를 호출한 경우, 압축 후 전체 콘텐츠를 복원하려면 다시 호출합니다.

<h3 id="pre-approve-tools-for-a-skill">
  Skill에 대한 도구 사전 승인
</h3>

`allowed-tools` 필드는 skill이 활성화되었을 때 나열된 도구에 대한 권한을 부여하므로 Claude는 승인을 요청하지 않고 사용할 수 있습니다. 사용 가능한 도구를 제한하지 않습니다: 모든 도구는 호출 가능하게 유지되며, [권한 설정](/docs/ko/permissions)은 나열되지 않은 도구에 대한 도구를 계속 관리합니다.

프로젝트의 `.claude/skills/` 디렉토리에 체크인된 skills의 경우, `allowed-tools`는 해당 폴더에 대한 작업 공간 신뢰 대화를 수락한 후 적용되며, `.claude/settings.json`의 권한 규칙과 동일합니다. 프로젝트 skills를 신뢰하기 전에 검토하세요. skill은 자신에게 광범위한 도구 액세스 권한을 부여할 수 있습니다.

이 skill은 skill을 호출할 때마다 Claude가 승인을 요청하지 않고 git 명령어를 실행할 수 있게 합니다:

```yaml theme={null}
---
name: commit
description: Stage and commit the current changes
disable-model-invocation: true
allowed-tools: Bash(git add *) Bash(git commit *) Bash(git status *)
---
```

skill이 활성화되었을 때 Claude의 사용 가능한 풀에서 도구를 제거하려면, skill의 frontmatter에서 `disallowed-tools`에 나열합니다. 다음 메시지를 보낼 때 제한이 해제됩니다. 모든 skills 및 프롬프트에서 도구를 차단하려면, [권한 설정](/docs/ko/permissions)에 거부 규칙을 추가합니다.

<h3 id="pass-arguments-to-skills">
  Skills에 인수 전달
</h3>

사용자와 Claude 모두 skill을 호출할 때 인수를 전달할 수 있습니다. 인수는 `$ARGUMENTS` 플레이스홀더를 통해 사용 가능합니다.

이 skill은 GitHub 이슈를 번호로 수정합니다. `$ARGUMENTS` 플레이스홀더는 skill 이름 뒤에 오는 모든 것으로 대체됩니다:

```yaml theme={null}
---
name: fix-issue
description: Fix a GitHub issue
disable-model-invocation: true
---

Fix GitHub issue $ARGUMENTS following our coding standards.

1. Read the issue description
2. Understand the requirements
3. Implement the fix
4. Write tests
5. Create a commit
```

`/fix-issue 123`을 실행하면 Claude는 "Fix GitHub issue 123 following our coding standards..."를 받습니다.

인수를 사용하여 skill을 호출하지만 skill에 `$ARGUMENTS`가 포함되지 않으면, Claude Code는 `ARGUMENTS: <your input>`을 skill 콘텐츠의 끝에 추가하므로 Claude는 여전히 입력한 내용을 봅니다.

한 메시지의 시작 부분에 여러 skills를 스택할 수도 있습니다. v2.1.199부터는 `/code-review /fix-issue 123`을 입력하면 두 skill이 모두 로드되고 뒤따르는 텍스트 `123`이 각각에 `$ARGUMENTS`로 전달됩니다. 이전 버전에서는 첫 번째 skill만 로드되고 `/fix-issue 123`을 리터럴 인수 텍스트로 받았습니다.

Claude Code는 첫 번째 skill과 그 뒤에 스택된 최대 5개의 skill을 확장합니다. 확장은 인라인 사용자 호출 가능 skill이 아닌 첫 번째 토큰에서 중지되므로, [forked subagent](#run-skills-in-a-subagent)로 실행되는 skill이나 인수 자체가 `/loop`와 같은 slash 명령어로 시작할 수 있는 skill도 거기서 끝나고, 그 토큰과 그 뒤의 모든 것이 모든 확장된 skill에 대한 인수 텍스트가 됩니다.

위치별로 개별 인수에 액세스하려면 `$ARGUMENTS[N]` 또는 더 짧은 `$N`을 사용합니다:

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one framework to another
---

Migrate the $ARGUMENTS[0] component from $ARGUMENTS[1] to $ARGUMENTS[2].
Preserve all existing behavior and tests.
```

`/migrate-component SearchBar React Vue`를 실행하면 `$ARGUMENTS[0]`을 `SearchBar`로, `$ARGUMENTS[1]`을 `React`로, `$ARGUMENTS[2]`를 `Vue`로 대체합니다. `$N` 약자를 사용하는 동일한 skill:

```yaml theme={null}
---
name: migrate-component
description: Migrate a component from one framework to another
---

Migrate the $0 component from $1 to $2.
Preserve all existing behavior and tests.
```

<h2 id="advanced-patterns">
  고급 패턴
</h2>

<h3 id="inject-dynamic-context">
  동적 컨텍스트 주입
</h3>

`` !`<command>` `` 구문은 skill 콘텐츠가 Claude로 전송되기 전에 shell 명령어를 실행합니다. 명령어 출력이 플레이스홀더를 대체하므로 Claude는 명령어 자체가 아닌 실제 데이터를 받습니다.

이 skill은 GitHub CLI를 사용하여 라이브 PR 데이터를 가져와 pull request를 요약합니다. `` !`gh pr diff` `` 및 기타 명령어가 먼저 실행되고, 출력이 프롬프트에 삽입됩니다:

```yaml theme={null}
---
name: pr-summary
description: Summarize changes in a pull request
context: fork
agent: Explore
allowed-tools: Bash(gh *)
---

## Pull request context
- PR diff: !`gh pr diff`
- PR comments: !`gh pr view --comments`
- Changed files: !`gh pr diff --name-only`

## Your task
Summarize this pull request...
```

이 skill이 실행될 때:

1. 각 `` !`<command>` ``가 즉시 실행됩니다(Claude가 보기 전에).
2. 출력이 skill 콘텐츠의 플레이스홀더를 대체합니다.
3. Claude는 실제 PR 데이터가 있는 완전히 렌더링된 프롬프트를 받습니다.

이는 전처리이며, Claude가 실행하는 것이 아닙니다. Claude는 최종 결과만 봅니다.

대체는 원본 파일에 대해 한 번 실행됩니다. 명령어 출력은 일반 텍스트로 삽입되며 추가 `` !`<command>` `` 플레이스홀더에 대해 다시 스캔되지 않으므로, 명령어는 나중의 패스에서 확장할 플레이스홀더를 내보낼 수 없습니다.

인라인 형식은 `!`이 줄의 시작 또는 공백 직후에 나타날 때만 인식됩니다. `!`이 `` KEY=!`cmd` ``처럼 다른 문자 뒤에 오면, 플레이스홀더는 리터럴 텍스트로 남겨지고 명령어는 실행되지 않습니다.

다중 라인 명령어의 경우, 인라인 형식 대신 ` ```! `로 열린 펜스 코드 블록을 사용합니다:

````markdown theme={null}
## Environment
```!
node --version
npm --version
git status --short
```
````

사용자, 프로젝트, 플러그인 또는 [추가 디렉토리](#skills-from-additional-directories) 소스의 skills 및 사용자 정의 명령어에 대해 이 동작을 비활성화하려면, [설정](/docs/ko/settings)에서 `"disableSkillShellExecution": true`를 설정합니다. 각 명령어는 실행되는 대신 `[shell command execution disabled by policy]`로 대체됩니다. 번들 및 관리 skills는 영향을 받지 않습니다. 이 설정은 사용자가 재정의할 수 없는 [관리 설정](/docs/ko/permissions#managed-settings)에서 가장 유용합니다.

<Tip>
  skill에서 더 깊은 추론을 요청하려면 skill 콘텐츠의 어디든 `ultrathink`를 포함합니다. [일회성 깊은 추론을 위해 ultrathink 사용](/docs/ko/model-config#use-ultrathink-for-one-off-deep-reasoning)을 참조하세요.
</Tip>

<h3 id="run-skills-in-a-subagent">
  Subagent에서 Skills 실행
</h3>

skill을 격리 상태에서 실행하려면 frontmatter에 `context: fork`를 추가합니다. skill 콘텐츠는 subagent를 구동하는 프롬프트가 됩니다. 대화 기록에 액세스할 수 없습니다.

<Warning>
  `context: fork`는 명시적 지침이 있는 skills에만 의미가 있습니다. skill에 작업 없이 "이 API 규칙을 사용하세요"와 같은 지침이 포함되어 있으면, subagent는 지침을 받지만 실행 가능한 프롬프트가 없으므로 의미 있는 출력 없이 반환됩니다.
</Warning>

Skills와 [subagents](/docs/ko/sub-agents)는 두 방향으로 함께 작동합니다:

| 접근 방식                     | 시스템 프롬프트              | 작업             | 또한 로드                                   |
| :------------------------ | :-------------------- | :------------- | :-------------------------------------- |
| `context: fork`가 있는 Skill | 에이전트 유형에서             | SKILL.md 콘텐츠   | CLAUDE.md, 에이전트가 Explore 또는 Plan인 경우 제외 |
| `skills` 필드가 있는 Subagent  | Subagent의 markdown 본문 | Claude의 위임 메시지 | 미리 로드된 skills + CLAUDE.md               |

`context: fork`를 사용하면 skill에 작업을 작성하고 실행할 에이전트 유형을 선택합니다. 기본 제공 Explore 및 Plan 에이전트는 [컨텍스트를 작게 유지하기 위해 CLAUDE.md 및 git status를 건너뜁니다](/docs/ko/sub-agents#what-loads-at-startup). 따라서 `agent: Explore`를 사용하는 forked skill은 SKILL.md 콘텐츠와 에이전트 자체의 시스템 프롬프트만 봅니다. 역방향(skills를 참조 자료로 사용하는 사용자 정의 subagent 정의)은 [Subagents](/docs/ko/sub-agents#preload-skills-into-subagents)를 참조하세요.

<h4 id="example-research-skill-using-explore-agent">
  예제: Explore 에이전트를 사용하는 Research Skill
</h4>

이 skill은 forked Explore 에이전트에서 연구를 실행합니다. skill 콘텐츠는 작업이 되고, 에이전트는 코드베이스 탐색에 최적화된 읽기 전용 도구를 제공합니다:

```yaml theme={null}
---
name: deep-research
description: Research a topic thoroughly
context: fork
agent: Explore
---

Research $ARGUMENTS thoroughly:

1. Find relevant files using Glob and Grep
2. Read and analyze the code
3. Summarize findings with specific file references
```

이 skill이 실행될 때:

1. 새로운 격리된 컨텍스트가 생성됩니다.
2. subagent는 skill 콘텐츠를 프롬프트로 받습니다("Research \$ARGUMENTS thoroughly...").
3. `agent` 필드는 실행 환경(모델, 도구 및 권한)을 결정합니다.
4. 결과는 요약되어 주 대화로 반환됩니다.

`agent` 필드는 사용할 subagent 구성을 지정합니다. 옵션에는 기본 제공 에이전트(`Explore`, `Plan`, `general-purpose`) 또는 `.claude/agents/`의 모든 사용자 정의 subagent가 포함됩니다. 생략하면 `general-purpose`를 사용합니다.

<h3 id="restrict-claude’s-skill-access">
  Claude의 Skill 액세스 제한
</h3>

기본적으로 Claude는 `disable-model-invocation: true`가 설정되지 않은 모든 skill을 호출할 수 있습니다. `allowed-tools`를 정의하는 Skills는 skill이 활성화되었을 때 사용자별 승인 없이 Claude에게 이러한 도구에 대한 액세스를 부여합니다. [권한 설정](/docs/ko/permissions)은 여전히 다른 모든 도구에 대한 기본 승인 동작을 관리합니다. `/init`, `/review`, `/security-review`를 포함한 몇 가지 기본 제공 명령어도 Skill 도구를 통해 사용 가능합니다. `/compact`와 같은 다른 기본 제공 명령어는 그렇지 않습니다.

Claude가 호출할 수 있는 skills를 제어하는 세 가지 방법:

**`/permissions`에서 Skill 도구를 거부하여 모든 skills를 비활성화합니다:**

```text theme={null}
# Add to deny rules:
Skill
```

**[권한 규칙](/docs/ko/permissions)을 사용하여 특정 skills를 허용하거나 거부합니다:**

```text theme={null}
# Allow only specific skills
Skill(commit)
Skill(review-pr *)

# Deny specific skills
Skill(deploy *)
```

권한 구문: 정확한 일치는 `Skill(name)`, 모든 인수를 사용한 접두사 일치는 `Skill(name *)`.

**개별 skills를 숨기기** - frontmatter에 `disable-model-invocation: true`를 추가합니다. 이는 Claude의 컨텍스트에서 skill을 완전히 제거합니다.

<Note>
  `user-invocable` 필드는 메뉴 가시성만 제어하고 Skill 도구 액세스는 제어하지 않습니다. 프로그래밍 방식 호출을 차단하려면 `disable-model-invocation: true`를 사용합니다.
</Note>

<h3 id="override-skill-visibility-from-settings">
  설정에서 Skill 가시성 재정의
</h3>

`skillOverrides` 설정은 skill의 자체 frontmatter 대신 [설정](/docs/ko/settings)에서 skill 가시성을 제어합니다. 공유 프로젝트 리포지토리에 체크인되거나 MCP 서버에서 제공하는 것처럼 SKILL.md를 편집하고 싶지 않은 skills에 사용합니다. `/skills` 메뉴가 이를 작성합니다: skill을 강조하고 `Space`를 눌러 상태를 순환한 다음 `Enter`를 눌러 `.claude/settings.local.json`에 저장합니다.

각 키는 skill 이름이고 각 값은 다음 네 가지 상태 중 하나입니다:

| 값                       | Claude에 나열됨 | `/` 메뉴에서 |
| :---------------------- | :---------- | :------- |
| `"on"`                  | 이름 및 설명     | 예        |
| `"name-only"`           | 이름만         | 예        |
| `"user-invocable-only"` | 숨김          | 예        |
| `"off"`                 | 숨김          | 숨김       |

`skillOverrides`에 없는 skill은 `"on"`으로 처리됩니다. 아래 예제는 한 skill을 이름으로 축소하고 다른 skill을 완전히 끕니다:

```json theme={null}
{
  "skillOverrides": {
    "legacy-context": "name-only",
    "deploy": "off"
  }
}
```

플러그인 skills은 `skillOverrides`의 영향을 받지 않습니다. `/plugin`을 통해 이를 관리합니다.

<h2 id="evaluate-and-iterate-on-a-skill">
  Skills 평가 및 반복
</h2>

skill이 트리거되는 것을 보는 것은 Claude가 이를 찾았다는 뜻이지, 의도한 대로 작동했다는 뜻이 아닙니다. skill이 작동하는지 알기 위해 두 가지를 별도로 측정합니다: Claude가 호출해야 하는 프롬프트에서 호출하는지 여부, 그리고 호출할 때 출력이 예상과 일치하는지 여부입니다.

둘 다에 대한 확인은 기준선 비교입니다. 몇 가지 현실적인 프롬프트를 수집하고, skill을 사용 가능하게 한 새로운 세션에서 각각을 실행한 다음 [비활성화된](#override-skill-visibility-from-settings) 상태에서 다시 실행하고, 결과를 비교합니다. 새로운 세션이 중요합니다. skill 작성의 남은 컨텍스트가 작성된 지침의 간격을 숨기기 때문입니다.

<h3 id="run-evals-with-skill-creator">
  skill-creator로 evals 실행
</h3>

[`skill-creator` 플러그인](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/skill-creator)은 Claude Code 내에서 비교 루프를 자동화합니다. 공식 마켓플레이스에서 설치합니다:

```text theme={null}
/plugin install skill-creator@claude-plugins-official
```

Claude Code가 플러그인을 마켓플레이스에서 찾을 수 없다고 보고하면, 마켓플레이스가 누락되었거나 오래되었습니다. `/plugin marketplace update claude-plugins-official`를 실행하여 새로 고치거나, 아직 추가하지 않았다면 `/plugin marketplace add anthropics/claude-plugins-official`를 실행합니다. 그 다음 설치를 다시 시도합니다.

설치 후 `/reload-plugins`를 실행하여 현재 세션에서 플러그인의 skills를 사용 가능하게 합니다. 그 다음 Claude에게 기존 skill을 평가하도록 요청합니다. 예를 들어 `evaluate my summarize-changes skill with skill-creator`. 플러그인은 테스트 케이스를 작성하고 루프를 실행하도록 안내합니다:

* **테스트 케이스**: skill 디렉토리 내의 `evals/evals.json`에 프롬프트, 입력 파일 및 예상 동작을 저장합니다.
* **격리된 실행**: 각 테스트 케이스당 [subagent](/docs/ko/sub-agents)를 생성하므로 각 실행이 깨끗한 컨텍스트로 시작되고, 토큰 수와 기간을 기록합니다.
* **채점**: 각 어설션을 출력에 대해 확인하고 `grading.json`에 증거와 함께 통과 또는 실패를 작성합니다.
* **벤치마크**: skill 있음 대 skill 없음에 대한 통과율, 시간 및 토큰을 `benchmark.json`에 집계하므로 토큰 및 시간 오버헤드에 대한 통과율 개선을 비교할 수 있습니다.
* **버전 비교**: skill의 두 버전 간에 블라인드 A/B를 실행하므로 커밋하기 전에 편집이 개선인지 확인할 수 있습니다.
* **설명 튜닝**: 트리거해야 하고 트리거하지 않아야 하는 프롬프트를 생성하고, 히트율을 측정하고, skill이 잘못된 요청에서 활성화될 때 설명 편집을 제안합니다.
* **리뷰 뷰어**: 각 출력을 검사하고 다음 반복이 읽을 정성적 피드백을 기록할 수 있는 HTML 보고서를 엽니다.

eval 파일 형식 및 전체 반복 워크플로우는 agentskills.io의 [Evaluating skill output quality](https://agentskills.io/skill-creation/evaluating-skills)를 참조하세요. 벤치마크 및 비교 모드의 배경은 [skill-creator 공지](https://claude.com/blog/improving-skill-creator-test-measure-and-refine-agent-skills)를 참조하세요.

<h2 id="share-skills">
  Skills 공유
</h2>

Skills는 대상에 따라 다양한 범위에서 배포할 수 있습니다:

* **프로젝트 skills**: `.claude/skills/`를 버전 제어에 커밋합니다.
* **플러그인**: [플러그인](/docs/ko/plugins)에서 `skills/` 디렉토리를 생성합니다.
* **관리**: [관리 설정](/docs/ko/settings#settings-files)을 통해 조직 전체에 배포합니다.

<h3 id="generate-visual-output">
  시각적 출력 생성
</h3>

Skills는 모든 언어의 스크립트를 번들하고 실행할 수 있으므로 Claude에게 단일 프롬프트로 가능한 것 이상의 기능을 제공합니다. 강력한 패턴 중 하나는 시각적 출력을 생성하는 것입니다: 브라우저에서 열리는 대화형 HTML 파일로 데이터 탐색, 디버깅 또는 보고서 생성에 사용됩니다.

이 예제는 코드베이스 탐색기를 생성합니다: 디렉토리를 확장 및 축소할 수 있는 대화형 트리 보기로, 한눈에 파일 크기를 보고, 색상으로 파일 유형을 식별할 수 있습니다.

Skill 디렉토리 생성:

```bash theme={null}
mkdir -p ~/.claude/skills/codebase-visualizer/scripts
```

`~/.claude/skills/codebase-visualizer/SKILL.md`에 저장합니다. 설명은 Claude에게 이 Skill을 언제 활성화할지 알려주고, 지침은 Claude에게 번들 스크립트를 실행하도록 알려줍니다. 스크립트 경로는 [`${CLAUDE_SKILL_DIR}`](#available-string-substitutions)를 사용하므로 skill이 개인, 프로젝트 또는 플러그인 수준에서 설치되었는지 여부에 관계없이 올바르게 해석됩니다:

````yaml theme={null}
---
name: codebase-visualizer
description: Generate an interactive collapsible tree visualization of your codebase. Use when exploring a new repo, understanding project structure, or identifying large files.
allowed-tools: Bash(python3 *)
---

# Codebase Visualizer

Generate an interactive HTML tree view that shows your project's file structure with collapsible directories.

## Usage

Run the visualization script from your project root:

```bash
python3 ${CLAUDE_SKILL_DIR}/scripts/visualize.py .
```

This creates `codebase-map.html` in the current directory and opens it in your default browser.

## What the visualization shows

- **Collapsible directories**: Click folders to expand/collapse
- **File sizes**: Displayed next to each file
- **Colors**: Different colors for different file types
- **Directory totals**: Shows aggregate size of each folder
````

`~/.claude/skills/codebase-visualizer/scripts/visualize.py`에 저장합니다. 이 스크립트는 디렉토리 트리를 스캔하고 다음을 포함하는 자체 포함 HTML 파일을 생성합니다:

* 파일 수, 디렉토리 수, 총 크기 및 파일 유형 수를 보여주는 **요약 사이드바**
* 파일 유형별로 코드베이스를 분석하는 **막대 차트**(크기 기준 상위 8개)
* 디렉토리를 확장 및 축소할 수 있는 **축소 가능한 트리**로, 색상으로 코딩된 파일 유형 표시기 포함

스크립트는 Python 3이 필요하지만 기본 제공 라이브러리만 사용하므로 설치할 패키지가 없습니다:

```python expandable theme={null}
#!/usr/bin/env python3
"""Generate an interactive collapsible tree visualization of a codebase."""

import json
import sys
import webbrowser
from html import escape
from pathlib import Path
from collections import Counter

IGNORE = {'.git', 'node_modules', '__pycache__', '.venv', 'venv', 'dist', 'build'}

def scan(path: Path, stats: dict) -> dict:
    result = {"name": path.name, "children": [], "size": 0}
    try:
        for item in sorted(path.iterdir()):
            if item.name in IGNORE or item.name.startswith('.'):
                continue
            if item.is_file():
                size = item.stat().st_size
                ext = item.suffix.lower() or '(no ext)'
                result["children"].append({"name": item.name, "size": size, "ext": ext})
                result["size"] += size
                stats["files"] += 1
                stats["extensions"][ext] += 1
                stats["ext_sizes"][ext] += size
            elif item.is_dir():
                stats["dirs"] += 1
                child = scan(item, stats)
                if child["children"]:
                    result["children"].append(child)
                    result["size"] += child["size"]
    except PermissionError:
        pass
    return result

def generate_html(data: dict, stats: dict, output: Path) -> None:
    ext_sizes = stats["ext_sizes"]
    total_size = sum(ext_sizes.values()) or 1
    sorted_exts = sorted(ext_sizes.items(), key=lambda x: -x[1])[:8]
    colors = {
        '.js': '#f7df1e', '.ts': '#3178c6', '.py': '#3776ab', '.go': '#00add8',
        '.rs': '#dea584', '.rb': '#cc342d', '.css': '#264de4', '.html': '#e34c26',
        '.json': '#6b7280', '.md': '#083fa1', '.yaml': '#cb171e', '.yml': '#cb171e',
        '.mdx': '#083fa1', '.tsx': '#3178c6', '.jsx': '#61dafb', '.sh': '#4eaa25',
    }
    lang_bars = "".join(
        f'<div class="bar-row"><span class="bar-label">{ext}</span>'
        f'<div class="bar" style="width:{(size/total_size)*100}%;background:{colors.get(ext,"#6b7280")}"></div>'
        f'<span class="bar-pct">{(size/total_size)*100:.1f}%</span></div>'
        for ext, size in sorted_exts
    )
    def fmt(b):
        if b < 1024: return f"{b} B"
        if b < 1048576: return f"{b/1024:.1f} KB"
        return f"{b/1048576:.1f} MB"

    html = f'''<!DOCTYPE html>
<html><head>
  <meta charset="utf-8"><title>Codebase Explorer</title>
  <style>
    body {{ font: 14px/1.5 system-ui, sans-serif; margin: 0; background: #1a1a2e; color: #eee; }}
    .container {{ display: flex; height: 100vh; }}
    .sidebar {{ width: 280px; background: #252542; padding: 20px; border-right: 1px solid #3d3d5c; overflow-y: auto; flex-shrink: 0; }}
    .main {{ flex: 1; padding: 20px; overflow-y: auto; }}
    h1 {{ margin: 0 0 10px 0; font-size: 18px; }}
    h2 {{ margin: 20px 0 10px 0; font-size: 14px; color: #888; text-transform: uppercase; }}
    .stat {{ display: flex; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid #3d3d5c; }}
    .stat-value {{ font-weight: bold; }}
    .bar-row {{ display: flex; align-items: center; margin: 6px 0; }}
    .bar-label {{ width: 55px; font-size: 12px; color: #aaa; }}
    .bar {{ height: 18px; border-radius: 3px; }}
    .bar-pct {{ margin-left: 8px; font-size: 12px; color: #666; }}
    .tree {{ list-style: none; padding-left: 20px; }}
    details {{ cursor: pointer; }}
    summary {{ padding: 4px 8px; border-radius: 4px; }}
    summary:hover {{ background: #2d2d44; }}
    .folder {{ color: #ffd700; }}
    .file {{ display: flex; align-items: center; padding: 4px 8px; border-radius: 4px; }}
    .file:hover {{ background: #2d2d44; }}
    .size {{ color: #888; margin-left: auto; font-size: 12px; }}
    .dot {{ width: 8px; height: 8px; border-radius: 50%; margin-right: 8px; }}
  </style>
</head><body>
  <div class="container">
    <div class="sidebar">
      <h1>📊 Summary</h1>
      <div class="stat"><span>Files</span><span class="stat-value">{stats["files"]:,}</span></div>
      <div class="stat"><span>Directories</span><span class="stat-value">{stats["dirs"]:,}</span></div>
      <div class="stat"><span>Total size</span><span class="stat-value">{fmt(data["size"])}</span></div>
      <div class="stat"><span>File types</span><span class="stat-value">{len(stats["extensions"])}</span></div>
      <h2>By file type</h2>
      {lang_bars}
    </div>
    <div class="main">
      <h1>📁 {escape(data["name"])}</h1>
      <ul class="tree" id="root"></ul>
    </div>
  </div>
  <script>
    const data = {json.dumps(data)};
    const colors = {json.dumps(colors)};
    function fmt(b) {{ if (b < 1024) return b + ' B'; if (b < 1048576) return (b/1024).toFixed(1) + ' KB'; return (b/1048576).toFixed(1) + ' MB'; }}
    function esc(s) {{ return s.replace(/[&<>"']/g, c => ({{"&":"&amp;","<":"&lt;",">":"&gt;",'"':"&quot;","'":"&#39;"}}[c])); }}
    function render(node, parent) {{
      if (node.children) {{
        const det = document.createElement('details');
        det.open = parent === document.getElementById('root');
        det.innerHTML = `<summary><span class="folder">📁 ${{esc(node.name)}}</span><span class="size">${{fmt(node.size)}}</span></summary>`;
        const ul = document.createElement('ul'); ul.className = 'tree';
        node.children.sort((a,b) => (b.children?1:0)-(a.children?1:0) || a.name.localeCompare(b.name));
        node.children.forEach(c => render(c, ul));
        det.appendChild(ul);
        const li = document.createElement('li'); li.appendChild(det); parent.appendChild(li);
      }} else {{
        const li = document.createElement('li'); li.className = 'file';
        li.innerHTML = `<span class="dot" style="background:${{colors[node.ext]||'#6b7280'}}"></span>${{esc(node.name)}}<span class="size">${{fmt(node.size)}}</span>`;
        parent.appendChild(li);
      }}
    }}
    data.children.forEach(c => render(c, document.getElementById('root')));
  </script>
</body></html>'''
    output.write_text(html)

if __name__ == '__main__':
    target = Path(sys.argv[1] if len(sys.argv) > 1 else '.').resolve()
    stats = {"files": 0, "dirs": 0, "extensions": Counter(), "ext_sizes": Counter()}
    data = scan(target, stats)
    out = Path('codebase-map.html')
    generate_html(data, stats, out)
    print(f'Generated {out.absolute()}')
    webbrowser.open(f'file://{out.absolute()}')
```

테스트하려면 모든 프로젝트에서 Claude Code를 열고 "Visualize this codebase"를 요청합니다. Claude는 스크립트를 실행하고, `codebase-map.html`을 생성하고, 브라우저에서 엽니다.

이 패턴은 모든 시각적 출력에 작동합니다: 종속성 그래프, 테스트 커버리지 보고서, API 문서 또는 데이터베이스 스키마 시각화. 번들 스크립트가 무거운 작업을 수행하는 동안 Claude는 조율을 처리합니다.

<h2 id="troubleshooting">
  문제 해결
</h2>

<h3 id="skill-not-triggering">
  Skill이 트리거되지 않음
</h3>

Claude가 예상대로 skill을 사용하지 않는 경우:

1. 설명에 사용자가 자연스럽게 말할 키워드가 포함되어 있는지 확인합니다.
2. skill이 `What skills are available?`에 나타나는지 확인합니다.
3. 설명과 더 가깝게 일치하도록 요청을 다시 표현해봅니다.
4. skill이 사용자 호출 가능하면 `/skill-name`으로 직접 호출합니다.

frontmatter YAML이 잘못된 형식이면, Claude Code는 skill 본문을 빈 메타데이터로 로드하므로 `/skill-name`은 여전히 작동하지만 Claude는 일치시킬 `description`이 없습니다. `--debug`로 실행하여 구문 분석 오류를 확인합니다.

<h3 id="skill-triggers-too-often">
  Skill이 너무 자주 트리거됨
</h3>

Claude가 원하지 않을 때 skill을 사용하는 경우:

1. 설명을 더 구체적으로 만듭니다.
2. 수동 호출만 원하면 `disable-model-invocation: true`를 추가합니다.

<h3 id="skill-descriptions-are-cut-short">
  Skill 설명이 잘림
</h3>

Claude Code는 skill 이름과 설명 목록을 컨텍스트에 로드하여 Claude가 사용 가능한 항목을 알 수 있도록 합니다. 목록에는 항상 모든 skill 이름이 포함되지만, skill이 많으면 Claude Code는 설명을 단축하여 목록의 문자 예산에 맞추며, 이는 Claude가 요청과 일치하는 데 필요한 키워드를 제거할 수 있습니다. 예산은 모델의 컨텍스트 윈도우의 1%에서 확장됩니다. 목록이 예산을 초과하면, Claude Code는 가장 적게 호출하는 skill부터 설명을 삭제하므로 가장 자주 사용하는 skill은 전체 텍스트를 유지합니다.

`/doctor`를 실행하여 목록의 컨텍스트 비용 추정치와 가장 큰 기여자를 확인합니다. 목록이 예산을 초과하면, Claude Code는 [`--debug`](/docs/ko/cli-reference#cli-flags)로 볼 수 있는 디버그 로그에 경고를 작성합니다.

`/context`의 Skills 행은 예산이 적용된 후의 목록 크기를 보고하므로 모델이 수신하는 것과 일치합니다. v2.1.196 이전에는 행이 모든 설명의 전체 텍스트를 계산했으므로 구성된 예산보다 몇 배 더 큰 값을 표시할 수 있었습니다.

예산을 높이려면 [`skillListingBudgetFraction`](/docs/ko/settings#available-settings) 설정(예: `0.02` = 2%)을 설정하거나 `SLASH_COMMAND_TOOL_CHAR_BUDGET` 환경 변수를 고정 문자 수로 설정합니다. 다른 skill을 위해 예산을 확보하려면 [`skillOverrides`](#override-skill-visibility-from-settings)에서 낮은 우선순위 항목을 `"name-only"`로 설정하여 설명 없이 나열되도록 합니다. 또한 소스에서 `description` 및 `when_to_use` 텍스트를 자를 수 있습니다: 주요 사용 사례를 먼저 배치합니다. 각 항목의 결합된 텍스트는 예산과 관계없이 1,536자로 제한되기 때문입니다. 이 제한은 [`skillListingMaxDescChars`](/docs/ko/settings#available-settings)로 구성할 수 있습니다.

<h2 id="related-resources">
  관련 리소스
</h2>

* **[구성 디버깅](/docs/ko/debug-your-config)**: skill이 나타나지 않거나 트리거되지 않는 이유 진단
* **[Skill 출력 품질 평가](https://agentskills.io/skill-creation/evaluating-skills)**: agentskills.io의 eval 파일 형식 및 반복 워크플로우
* **[Skill 작성 모범 사례](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices)**: Claude 제품 전체에 적용되는 작성 지침
* **[Subagents](/docs/ko/sub-agents)**: 특화된 에이전트에 작업 위임
* **[플러그인](/docs/ko/plugins)**: 다른 확장과 함께 skills 패키징 및 배포
* **[Hooks](/docs/ko/hooks)**: 도구 이벤트 주변 워크플로우 자동화
* **[메모리](/docs/ko/memory)**: 지속적인 컨텍스트를 위한 CLAUDE.md 파일 관리
* **[명령어](/docs/ko/commands)**: 기본 제공 명령어 및 번들 skills 참조
* **[권한](/docs/ko/permissions)**: 도구 및 skill 액세스 제어
* **[Claude Tag skills](https://claude.com/docs/claude-tag/admins/skills-repo)**: 리포지토리에 커밋된 프로젝트 skills는 해당 리포지토리가 Claude Tag 채널에서 사용될 때도 로드됩니다
