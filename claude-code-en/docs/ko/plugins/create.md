> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code 플러그인 만들기

> 빈 디렉토리에서 첫 번째 Claude Code 플러그인을 만들고, 마켓플레이스 없이 테스트하며, 기존 .claude/ 설정을 변환합니다.

플러그인은 스킬, 에이전트, 훅 및 MCP 서버의 디렉토리이며, 플러그인의 이름을 지정하는 `plugin.json` 파일(매니페스트)을 포함합니다. Claude Code는 디렉토리를 하나의 단위로 로드하므로 팀원과 공유하거나, 여러 프로젝트에 설치하거나, 마켓플레이스에 게시할 수 있습니다.

이 페이지는 자신의 플러그인을 작성하는 사람들을 위한 것입니다.

<Note>
  다음 경우는 다른 페이지에서 다룹니다:

  * **다른 사람의 플러그인 설치**: [플러그인 설치](/docs/ko/plugins/install) 참조
  * **플러그인이 필요한지 확실하지 않음**: 개요의 [플러그인이 필요한지 결정](/docs/ko/plugins/overview#decide-whether-you-need-a-plugin) 참조
  * **플러그인의 사용자가 claude.ai 또는 Cowork에 있음**: 동일한 폴더가 다른 구성 요소 부분 집합으로 설치됩니다. [claude.ai 및 Cowork의 플러그인](https://claude.com/docs/plugins/overview) 참조
</Note>

이미 가지고 있는 것과 일치하는 섹션에서 시작하세요:

* **아직 아무것도 없음**: [첫 번째 플러그인 만들기](#create-your-first-plugin)를 따른 다음 [마켓플레이스 없이 개발](#develop-without-a-marketplace) 및 [테스트 및 디버그](#test-and-debug)를 따릅니다.
* **`.claude/` 아래에 파일이 이미 있음**: 레이아웃을 배우기 위해 첫 번째 플러그인 연습을 한 번 수행한 다음 [기존 `.claude/` 설정 변환](#convert-an-existing-claude-setup)을 따릅니다.

<h2 id="decide-when-to-use-a-plugin">
  플러그인을 사용할 시기 결정
</h2>

스킬, 에이전트, 훅 및 MCP 서버는 모두 프로젝트 또는 홈 디렉토리에서 독립적으로 작동합니다. 하나의 프로젝트에만 제공하거나 자신만 사용하는 동안 독립 실행형 설정을 유지하세요. 팀원과 설정을 공유하거나, 여러 프로젝트에 설치하거나, 버전이 지정된 릴리스를 게시하려는 경우 플러그인을 만드세요.

독립 실행형 스킬, 에이전트, 훅 및 MCP 구성을 플러그인으로 이동할 때 해당 위치와 이름이 변경됩니다:

* **파일이 가는 위치**: 플러그인 루트라고 하는 플러그인의 자체 디렉토리 아래에 `skills/`, `agents/`, `hooks/hooks.json` 및 `.mcp.json`으로 저장됩니다.
* **이름 지정 방식**: 플러그인 스킬 및 에이전트는 플러그인 이름을 접두사로 가져옵니다(예: `/my-plugin:hello`). 따라서 두 플러그인이 각각 `hello` 스킬을 제공할 수 있으며 충돌하지 않습니다.

기존 설정을 플러그인으로 이동하려면 [기존 `.claude/` 설정 변환](#convert-an-existing-claude-setup)을 참조하세요.

<h2 id="create-your-first-plugin">
  첫 번째 플러그인 만들기
</h2>

이 연습에서는 유일한 구성 요소가 하나의 스킬(인사말)인 플러그인을 만들고 `--plugin-dir`으로 실행합니다. 이는 설치하지 않고 한 세션 동안 플러그인을 로드합니다. 플러그인은 스킬, 에이전트, 훅 및 MCP 서버와 같은 [구성 요소](/docs/ko/plugins/components)의 모든 조합을 보유할 수 있으며, 어느 것도 필요하지 않습니다. 하나의 스킬은 레이아웃을 보여주는 가장 작은 예제입니다.

Claude Code [설치 및 로그인](/docs/ko/quickstart#step-1-install-claude-code)이 필요합니다.

플러그인을 보관할 디렉토리(예: `~/projects`)에서 터미널을 열고 이 단계의 명령을 실행하세요. 플러그인을 어디든 보관할 수 있습니다. 세션을 시작할 때 Claude Code에 경로를 전달하기 때문입니다.

<Steps>
  <Step title="플러그인 디렉토리 만들기">
    플러그인 디렉토리를 만들고, 매니페스트를 보관할 `.claude-plugin/` 폴더를 그 안에 만듭니다:

    ```bash theme={null}
    mkdir -p my-first-plugin/.claude-plugin
    ```
  </Step>

  <Step title="매니페스트 작성">
    [매니페스트](/docs/ko/plugins/manifest-reference)는 Claude Code에 플러그인의 이름을 알려주고 설명하는 `plugin.json`이라는 JSON 파일입니다. 이것을 `my-first-plugin/.claude-plugin/plugin.json`으로 저장하세요:

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

    네 필드는 다음을 수행합니다:

    * **`name`**: 필수입니다. 플러그인을 식별하고 플러그인이 제공하는 모든 스킬 및 에이전트의 접두사가 됩니다. 공백을 포함하지 마세요.
    * **`description`**: 사용자가 `/plugin`에서 플러그인에 대해 보는 텍스트입니다.
    * **`version`**: 선택 사항입니다. 설정하면 사용자가 변경할 때까지 해당 버전에 유지됩니다. [새 버전 릴리스](/docs/ko/plugins/host-marketplace#release-a-new-version)는 설정하거나 생략할 시기를 설명합니다.
    * **`author`**: 누구에게 크레딧을 줄지입니다. 그 안에 `name`은 필수입니다. `email` 및 `url`은 선택 사항입니다.

    다른 모든 필드는 [매니페스트 참조](/docs/ko/plugins/manifest-reference#fields)에 있습니다.

    `.claude-plugin/` 안에는 `plugin.json`만 들어갑니다. 다음에 추가할 스킬은 `my-first-plugin/` 아래에 직접 들어가며, 해당 폴더 옆에 있습니다.
  </Step>

  <Step title="스킬 추가">
    이 플러그인의 유일한 구성 요소는 스킬입니다. 각 스킬은 `SKILL.md` 파일을 포함하는 `skills/` 아래의 디렉토리입니다. 스킬의 디렉토리를 만듭니다:

    ```bash theme={null}
    mkdir -p my-first-plugin/skills/hello
    ```

    그런 다음 다음 내용으로 `my-first-plugin/skills/hello/SKILL.md`를 만듭니다:

    ```markdown my-first-plugin/skills/hello/SKILL.md theme={null}
    ---
    name: hello
    description: Greet the user with a friendly message
    disable-model-invocation: true
    ---

    Greet the user warmly and ask how you can help them today.
    ```

    `disable-model-invocation: true` 줄은 Claude가 스킬을 자동으로 실행하지 않음을 의미하므로 사용자만 트리거합니다. Claude가 자동으로 실행하려는 스킬에서 해당 줄을 제거하세요. 스킬의 명령은 플러그인 이름과 스킬의 이름을 결합하므로 이것을 `/my-first-plugin:hello`로 실행합니다. 다른 프론트매터 필드는 [스킬 프론트매터 참조](/docs/ko/skills#frontmatter-reference)를 참조하세요.
  </Step>

  <Step title="플러그인 검증">
    아무것도 실행하기 전에 매니페스트와 스킬의 프론트매터를 확인하세요:

    ```bash theme={null}
    claude plugin validate ./my-first-plugin
    ```

    명령은 확인한 매니페스트 경로와 `✔ Validation passed`를 출력합니다. 대신 `✘ Validation failed`를 출력하면, 그 결과 줄 위의 각 줄은 수정할 필드의 이름을 지정합니다. [`claude plugin validate` 보고 오류](/docs/ko/plugins/troubleshooting#claude-plugin-validate-reports-errors) 아래에서 각 메시지를 찾아보세요.
  </Step>

  <Step title="플러그인으로 Claude Code 실행">
    플러그인이 로드된 세션을 시작합니다:

    ```bash theme={null}
    claude --plugin-dir ./my-first-plugin
    ```

    Claude Code가 시작되면 스킬을 실행합니다:

    ```text theme={null}
    /my-first-plugin:hello
    ```

    Claude가 인사말로 응답합니다.
  </Step>
</Steps>

플러그인은 `--plugin-dir`으로 시작하는 세션에서만 로드됩니다. 플래그 없이 계속 작업하거나 `.zip` 빌드를 테스트하려면 [마켓플레이스 없이 개발](#develop-without-a-marketplace)을 참조하세요.

<h3 id="share-the-plugin">
  플러그인 공유
</h3>

[첫 번째 플러그인 만들기](#create-your-first-plugin)로 만든 플러그인은 컴퓨터에만 존재합니다. 다른 사람들이 사용할 준비가 되면 세 가지 방법으로 전달할 수 있습니다:

* **몇 사람에게 직접 보내기**: 플러그인의 디렉토리 또는 `.zip`을 제공하면 아무것도 게시할 필요가 없습니다. [마켓플레이스 없이 플러그인 공유](/docs/ko/plugins/publish#share-a-plugin-without-a-marketplace)를 참조하세요.
* **자신의 마켓플레이스에 나열**: 팀원이 마켓플레이스를 한 번 추가하고 이름으로 플러그인을 설치하면 업데이트를 받습니다. [자신의 마켓플레이스를 통해 게시](/docs/ko/plugins/publish#publish-through-your-own-marketplace)를 참조하세요.
* **Anthropic의 커뮤니티 마켓플레이스에 제출**: 나열되면 해당 마켓플레이스를 추가하는 모든 사람이 설치할 수 있습니다. [커뮤니티 마켓플레이스에 제출](/docs/ko/plugins/publish#submit-to-the-community-marketplace)을 참조하세요.

<h3 id="plugin-layout">
  플러그인 레이아웃
</h3>

스킬, 에이전트, 훅 및 MCP 서버와 같은 각 종류의 [구성 요소](/docs/ko/plugins/components)는 플러그인 루트(즉, `--plugin-dir`에 전달하는 디렉토리) 아래의 고정 디렉토리에 들어갑니다. 사용하는 디렉토리만 추가하세요. 완전한 플러그인 디렉토리를 클릭하고 각 파일이 무엇을 하는지 읽으려면 [플러그인 탐색기](/docs/ko/plugins/components#explore-the-plugin-directory)를 열어보세요.

표는 대부분의 플러그인이 시작하는 디렉토리를 나열하며, [전체 레이아웃](/docs/ko/plugins/manifest-reference#standard-layout)은 나머지를 나열합니다.

| 위치                           | 내용                                                                                      |
| :--------------------------- | :-------------------------------------------------------------------------------------- |
| `.claude-plugin/plugin.json` | 매니페스트입니다. `--plugin-dir`으로 플러그인을 로드하고 매니페스트가 없으면 Claude Code는 디렉토리 이름으로 플러그인의 이름을 지정합니다 |
| `skills/`                    | 스킬당 하나의 `<name>/SKILL.md` 디렉토리                                                          |
| `commands/`                  | 평면 마크다운 파일, 스킬의 이전 형식입니다. 새 플러그인에는 `skills/`를 사용하세요                                     |
| `agents/`                    | 서브에이전트당 하나의 마크다운 파일                                                                     |
| `hooks/hooks.json`           | 훅 구성: 최상위 `"hooks"` 키이며 값은 설정 파일의 `hooks`와 동일한 형태입니다                                    |
| `.mcp.json`                  | MCP 서버 정의                                                                               |

<Warning>
  `.claude-plugin/` 안에는 `plugin.json`만 들어갑니다. 거기에 저장된 구성 요소는 로드되지 않습니다.

  플러그인 루트는 플러그인의 자체 디렉토리이며, `~/.claude/` 자체가 아닙니다. `~/.claude/.mcp.json`에 저장된 `.mcp.json`은 로드되지 않습니다.
</Warning>

<h2 id="develop-without-a-marketplace">
  마켓플레이스 없이 개발
</h2>

작성 중인 플러그인을 실행하기 위해 [마켓플레이스](/docs/ko/plugins/overview#get-plugins-from-a-marketplace)가 필요하지 않습니다. 대신 디스크 또는 URL에서 직접 로드하세요:

* [`--plugin-dir`](#load-a-directory-or-archive-for-one-session): 한 세션 동안 디렉토리 또는 `.zip` 아카이브를 로드합니다.
* [`--plugin-url`](#fetch-an-archive-from-a-url-for-one-session): 한 세션 동안 URL에서 `.zip` 아카이브를 가져옵니다.
* [`claude plugin init`](#scaffold-a-plugin-that-loads-every-session): `~/.claude/skills/` 아래에 모든 세션에서 로드되는 플러그인을 스캐폴드합니다.

다른 방식으로 로드된 두 플러그인이 이름을 공유하면 [이름 충돌](/docs/ko/plugins/loading#name-conflicts)을 참조하여 Claude Code가 어느 것을 유지하는지 확인하세요.

<h3 id="load-a-directory-or-archive-for-one-session">
  한 세션 동안 플러그인 로드
</h3>

세 가지 방법으로 단일 세션 동안 플러그인을 로드할 수 있습니다: `--plugin-dir`으로 디스크의 디렉토리 또는 `.zip` 아카이브에서, `--plugin-url`로 URL에서, 또는 플래그를 추가할 수 없을 때 환경 변수에서. 각 플러그인은 해당 세션에만 로드되며, 설정에 아무것도 기록되지 않습니다. 세션 중에 플러그인의 파일을 편집하면 `/reload-plugins`를 실행하여 변경 사항을 로드합니다.

<h4 id="from-a-directory-or-zip">
  디렉토리 또는 `.zip`에서
</h4>

셸에서 `claude`를 시작할 때 `--plugin-dir`을 플러그인의 루트 디렉토리 또는 그 `.zip` 아카이브와 함께 전달합니다. 여러 플러그인을 로드하려면 플래그를 반복합니다:

```bash theme={null}
claude --plugin-dir ./my-first-plugin --plugin-dir ./other-plugin.zip
```

<h4 id="load-a-folder-of-plugins">
  플러그인 폴더에서
</h4>

한 곳에서 여러 플러그인을 로드하려면 `--plugin-dir ./plugins`와 같이 플러그인을 보관하는 폴더를 전달합니다. 플러그인 폴더를 로드하려면 Claude Code v2.1.265 이상이 필요합니다.

폴더에 `.claude-plugin/` 디렉토리가 없고 최상위 수준에 플러그인 구성 요소가 없으면 Claude Code는 이를 플러그인 폴더로 취급합니다. `.claude-plugin/plugin.json` 매니페스트가 있는 각 직접 하위 폴더는 별도의 플러그인으로 로드됩니다. 폴더의 다른 모든 것은 매니페스트가 없는 하위 폴더를 포함하여 오류 없이 건너뜁니다. 폴더의 플러그인이 로드되지 않으면 하위 폴더에 `.claude-plugin/plugin.json`이 있는지 확인하세요.

대화형 세션에서 시작 후 폴더의 플러그인을 추가 및 제거할 수도 있습니다:

* 추가하는 하위 폴더는 매니페스트가 존재하면 새 플러그인으로 로드됩니다.
* 하위 폴더를 제거하면 해당 플러그인이 언로드됩니다.

이러한 각 변경에 대해 세션에 메시지가 나타납니다. 중간 대화 중에 플러그인을 로드하거나 언로드하면 [프롬프트 캐시](/docs/ko/prompt-caching#enabling-or-disabling-a-plugin)가 무효화되면 변경이 대신 보류되고 메시지는 적용하려면 `/reload-plugins`를 실행하도록 알려줍니다.

<h4 id="fetch-an-archive-from-a-url-for-one-session">
  URL에서
</h4>

셸에서 `claude`를 시작할 때 `--plugin-url`을 `.zip` 아카이브의 주소(예: CI가 게시하는 빌드 아티팩트)와 함께 전달합니다:

```bash theme={null}
claude --plugin-url https://example.com/my-first-plugin.zip
```

Claude Code는 시작 시 아카이브를 다운로드합니다. 여러 개를 로드하려면 플래그를 반복하거나 URL을 공백으로 구분하여 하나의 인용된 인수로 전달합니다.

플래그는 제어하거나 신뢰하는 아카이브에만 가리킵니다.

Claude Code가 아카이브를 가져올 수 없거나 아카이브가 유효하지 않으면 플러그인 없이 시작되고 `/plugin` 관리자의 **Errors** 탭에서 검토할 수 있는 플러그인 로드 오류를 기록합니다.

<h4 id="from-an-environment-variable">
  환경 변수에서
</h4>

`--plugin-dir` 플래그를 추가할 수 없는 세션에서 플러그인을 로드하려면 [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/ko/env-vars#variables) 환경 변수에 절대 경로를 나열하세요. Claude Code는 각 경로를 `--plugin-dir` 경로로 로드합니다. 이 플러그인은 `--plugin-dir`으로 전달하는 모든 플러그인에 추가로 로드됩니다. [프로젝트 및 로컬 설정은 이 변수를 설정할 수 없습니다](/docs/ko/settings-reference#variables-claude-code-ignores-in-env). `CLAUDE_CODE_PLUGIN_DIRS`는 Claude Code v2.1.280 이상이 필요합니다.

관리되는 설정은 `--plugin-dir` 및 `CLAUDE_CODE_PLUGIN_DIRS`를 끌 수 있습니다. [한 세션 동안 플러그인을 로드하는 플래그](/docs/ko/plugins/cli-reference#flags-that-load-a-plugin-for-one-session)를 참조하세요. 플러그인과 그것이 의존하는 플러그인을 함께 테스트하려면 [플러그인 및 해당 종속성을 로컬로 테스트](/docs/ko/plugins/dependencies#test-a-plugin-and-its-dependency-locally)를 참조하세요.

<h3 id="scaffold-a-plugin-that-loads-every-session">
  모든 세션에서 플러그인 로드
</h3>

개인 스킬 디렉토리는 `~/.claude/skills/`입니다. Claude Code는 `.claude-plugin/plugin.json`을 포함하는 모든 폴더를 플래그 없이 설치 단계 없이 모든 세션에서 플러그인으로 로드합니다. `claude plugin init`은 이러한 플러그인 중 하나를 스캐폴드합니다.

<h4 id="scaffold-the-plugin-with-claude-plugin-init">
  `claude plugin init`으로 플러그인 스캐폴드
</h4>

`claude plugin init`은 `~/.claude/skills/` 아래에 스타터 플러그인을 작성합니다. Claude Code v2.1.157 이상이 필요합니다. 셸에서 하나를 스캐폴드합니다:

```bash theme={null}
claude plugin init my-tool
```

명령은 `.claude-plugin/plugin.json` 및 루트 `SKILL.md`와 함께 `~/.claude/skills/my-tool/`을 만듭니다. `✔ Created plugin "my-tool" at ~/.claude/skills/my-tool`을 출력한 다음 `It will auto-load next session as my-tool@skills-dir. Run /reload-plugins to load it now.`를 출력합니다.

`claude plugin init`이 `skills/` 아래에 스킬을 스캐폴드하도록 `--with skills`를 전달합니다. 다른 `--with` 값은 [플러그인 명령 참조](/docs/ko/plugins/cli-reference#plugin-init)에 있습니다.

<h4 id="skill-names-in-a-scaffolded-plugin">
  스캐폴드된 플러그인의 스킬 이름
</h4>

`~/.claude/skills/my-tool/SKILL.md`의 루트 스킬도 개인 스킬이므로 `/my-tool:my-tool`이 아닌 `/my-tool`로 호출합니다. 플러그인 내 `skills/` 아래에 추가하는 스킬은 `/my-tool:example`과 같은 플러그인 이름 접두사를 가집니다.

<h4 id="stop-loading-the-plugin">
  플러그인 로드 중지
</h4>

스캐폴드된 플러그인 로드를 중지하려면 해당 디렉토리를 삭제하거나 셸에서 `claude plugin disable my-tool@skills-dir`을 실행하세요. `my-tool@skills-dir` 이름은 `claude plugin init`이 출력했습니다. ID `my-tool@skills-dir`에서 `skills-dir`은 플러그인이 마켓플레이스가 아닌 스킬 디렉토리에서 로드되기 때문에 마켓플레이스 이름이 있을 위치에 서 있습니다.

<h4 id="load-a-plugin-for-everyone-in-one-repository">
  저장소를 통해 플러그인 공유
</h4>

`claude plugin init`은 플러그인을 개인 스킬 디렉토리 `~/.claude/skills/`에 작성하므로 모든 프로젝트에서 로드됩니다. 한 저장소의 모든 사람이 플러그인을 로드하도록 하려면 `.claude-plugin/plugin.json`을 포함하여 `<project>/.claude/skills/<name>/`에서 동일한 레이아웃을 직접 만드세요. Claude Code가 로드하는 조건은 [저장소를 통해 공유된 플러그인](/docs/ko/plugins/loading#plugins-shared-through-a-repository)을 참조하세요.

<h2 id="test-and-debug">
  테스트 및 디버그
</h2>

플러그인의 변경 사항이 표시되지 않으면 순서대로 이 확인을 진행하세요. 각각은 Claude Code가 플러그인으로 무엇을 했는지 알려줍니다:

1. 셸에서 `claude plugin validate <path>`를 실행합니다. 모든 스킬, 에이전트 및 명령 파일의 매니페스트 및 프론트매터를 확인하고 `Validation passed`에서 종료 코드 `0`으로 종료합니다. 경고에서도 실패하려면 `--strict`를 추가합니다. 종료 코드 및 디렉토리 처리는 [플러그인 명령 참조](/docs/ko/plugins/cli-reference#plugin-validate)에 있습니다.
2. 실행 중인 세션에서 `/reload-plugins`를 실행하여 디스크에서 만든 편집을 적용합니다. 개수가 있는 하나의 `Reloaded:` 줄을 출력합니다. 그런 다음 `/plugin-name:skill` 명령을 입력하거나 `/plugin` **Installed** 탭에서 플러그인을 찾아 스킬이 로드되었는지 확인합니다.
3. 동일한 세션에서 `/plugin`을 실행합니다. **Installed** 탭은 플러그인을 나열하고, 플러그인의 세부 정보에서 Claude Code가 찾은 구성 요소를 나열합니다. **Errors** 탭은 로드되지 않은 것과 이유(예: 매니페스트의 존재하지 않는 경로)를 나열합니다.
4. 셸로 돌아가서 `claude plugin list`를 실행합니다. 세션 전용 및 스킬 디렉토리 플러그인을 자체 섹션에 `Status: ✔ loaded` 또는 로드 오류와 함께 출력합니다. 개발 중인 플러그인을 포함하려면 `plugin list` 전에 `--plugin-dir`을 경로와 함께 전달합니다.

MCP 서버를 확인하려면 세션에서 `/mcp`를 실행하여 서버의 상태를 확인합니다. 서버가 정상이면 `/mcp`는 연결됨으로 나열합니다. 그렇지 않으면 [시작되지 않는 MCP 서버](/docs/ko/plugins/troubleshooting#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start)를 참조하세요.

훅을 확인하려면 일치하는 이벤트를 트리거합니다. 예를 들어 Claude에 파일을 편집하도록 요청하여 `PostToolUse` 훅을 트리거합니다. 그런 다음 [디버그 로그](/docs/ko/hooks#debug-hooks)를 읽으세요. 이는 일치한 훅, 종료 코드 및 출력을 보여줍니다.

다음 섹션은 개발 중에 가장 가능성이 높은 실패를 다루며, [문제 해결 페이지](/docs/ko/plugins/troubleshooting#build-a-plugin)에는 각각에 대한 전체 항목이 있습니다.

<h3 id="a-component-path-isn’t-found">
  구성 요소 경로를 찾을 수 없음
</h3>

`/plugin`의 **Errors** 탭은 `<component> path not found: <path>`를 표시합니다(예: `commands path not found`). 매니페스트의 구성 요소 경로(예: `commands`, `skills`, `agents` 또는 `hooks`)가 아무것도 가리키지 않습니다. 경로를 수정하거나 디렉토리를 만든 다음 세션에서 `/reload-plugins`를 실행합니다. [`commands path not found`](/docs/ko/plugins/troubleshooting#commands-path-not-found)를 참조하세요.

<h3 id="plugin-dir-at-a-marketplace-root-doesn’t-load-the-plugins-under-plugins/">
  `--plugin-dir`이 마켓플레이스 루트에서 `plugins/` 아래의 플러그인을 로드하지 않음
</h3>

`--plugin-dir`은 `.claude-plugin/plugin.json` 및 `skills/`와 같은 구성 요소 디렉토리를 포함하는 플러그인의 루트 디렉토리를 사용합니다. 대신 마켓플레이스 루트를 가리키면 Claude Code는 `marketplace.json`을 읽지 않으므로 `plugins/` 아래의 플러그인이 로드되지 않으며 오류가 표시되지 않습니다. 플래그를 하나의 플러그인 폴더에 가리키거나 마켓플레이스를 추가합니다. [문제 해결 항목](/docs/ko/plugins/troubleshooting#plugin-dir-loads-a-plugin-with-no-components)을 참조하세요.

<h3 id="the-plugin-loads-but-its-skills-are-missing">
  플러그인이 로드되지만 스킬이 누락됨
</h3>

`skills/` 디렉토리가 `.claude-plugin/` 내부에 있거나 매니페스트의 `skills` 항목이 파일을 가리킵니다. `skills/`를 플러그인 루트로 이동하고, 각 `skills` 항목이 `SKILL.md`를 포함하는 디렉토리를 가리키도록 하고, 세션에서 `/reload-plugins`를 실행합니다. [플러그인이 로드되지만 스킬이 누락됨](/docs/ko/plugins/troubleshooting#plugin-loads-but-its-skills-are-missing)을 참조하세요.

<h3 id="the-userconfig-dialog-never-appears">
  `userConfig` 대화 상자가 나타나지 않음
</h3>

플러그인의 [`userConfig`](/docs/ko/plugins/components#user-configuration) 옵션에 대한 대화 상자는 세션에서 `/plugin`을 통해 설치하는 부분입니다. `--plugin-dir`으로 로드하면 표시되지 않으며, `claude plugin install`도 셸에서 표시되지 않습니다. 플러그인이 로드되면 세션에서 `/plugin configure <plugin-name>`을 실행하여 열어보세요. [`userConfig` 대화 상자가 나타나지 않음](/docs/ko/plugins/troubleshooting#the-userconfig-dialog-never-appears)을 참조하세요.

<h3 id="check-that-the-plugin-changes-claude’s-behavior">
  플러그인이 Claude의 동작을 변경하는지 확인
</h3>

오류 없이 로드되는 플러그인도 의도한 방식으로 Claude를 조종하지 못할 수 있습니다. 셸에서 실행하는 `claude plugin eval`은 플러그인 있음과 없음으로 테스트 사례를 실행하고 차이를 점수 매깁니다. [플러그인으로 evals 테스트](/docs/ko/plugin-evals)를 참조하고 [첫 번째 eval 스위트 만들기](/docs/ko/plugin-evals#create-your-first-eval-suite)부터 시작합니다.

<h2 id="convert-an-existing-claude-setup">
  기존 `.claude/` 설정 변환
</h2>

프로젝트의 `.claude/` 디렉토리 아래에 스킬, 에이전트 또는 훅이 이미 있으면 다시 작성하지 않고 플러그인으로 이동할 수 있습니다.

`.claude/`를 포함하는 디렉토리인 프로젝트 루트에서 이 단계의 명령을 실행합니다. `cp` 경로가 상대적이기 때문입니다.

<Steps>
  <Step title="플러그인 구조 만들기">
    플러그인 디렉토리와 `.claude-plugin/` 폴더를 `.claude/` 옆에 만듭니다. 나중에 플러그인을 어디든 이동할 수 있습니다.

    ```bash theme={null}
    mkdir -p my-plugin/.claude-plugin
    ```

    `my-plugin/.claude-plugin/plugin.json`을 만듭니다:

    ```json my-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-plugin",
      "description": "Migrated from standalone configuration",
      "version": "1.0.0"
    }
    ```
  </Step>

  <Step title="기존 파일 복사">
    가지고 있는 각 구성 디렉토리를 플러그인 루트로 복사하고 없는 디렉토리의 명령을 건너뜁니다.

    ```bash theme={null}
    cp -r .claude/commands my-plugin/
    ```

    ```bash theme={null}
    cp -r .claude/agents my-plugin/
    ```

    ```bash theme={null}
    cp -r .claude/skills my-plugin/
    ```

    `ls -a my-plugin`을 실행하여 복사한 각 디렉토리가 `.claude-plugin` 옆에 나타나는지 확인합니다.
  </Step>

  <Step title="훅 이동">
    `.claude/settings.json` 또는 `.claude/settings.local.json`에 훅이 있으면 훅 디렉토리를 만듭니다:

    ```bash theme={null}
    mkdir -p my-plugin/hooks
    ```

    `my-plugin/hooks/hooks.json`을 만들고 설정 파일에서 `hooks` 객체를 복사합니다. 형식은 동일합니다.

    이 예제는 Claude가 작성하거나 편집하는 각 파일에서 린터를 실행하는 하나의 훅이 있는 형태를 보여줍니다. 예제를 자신의 `hooks` 객체로 바꾸세요.

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
    한 세션 동안 플러그인을 로드합니다:

    ```bash theme={null}
    claude --plugin-dir ./my-plugin
    ```

    새 이름으로 각 구성 요소를 확인합니다:

    * **스킬**: `/deploy`였던 스킬에 대해 `/my-plugin:deploy`를 실행합니다.
    * **서브에이전트**: `reviewer`였던 에이전트에 대해 Claude에 `my-plugin:reviewer` 에이전트를 사용하도록 요청합니다.
    * **훅**: 각 훅이 일치하는 이벤트를 트리거합니다.

    뭔가 누락되면 [테스트 및 디버그](#test-and-debug)를 진행합니다.
  </Step>
</Steps>

원본이 여전히 `.claude/` 아래에 있는 동안 플러그인의 복사본과 함께 로드된 상태로 유지됩니다:

* **스킬 및 에이전트**: 두 세트는 충돌하지 않습니다. 플러그인의 스킬 및 에이전트는 `my-plugin:` 접두사를 가지기 때문입니다. `/deploy` 및 `/my-plugin:deploy` 모두 작동하며, Claude는 `reviewer` 및 `my-plugin:reviewer`를 두 개의 서브에이전트로 봅니다.
* **훅**: 훅에는 접두사가 없으므로 설정 파일과 `hooks/hooks.json` 모두에 있는 훅은 이벤트가 발생할 때마다 두 번 실행됩니다.

플러그인이 작동하는지 확인한 후 `.claude/`에서 원본을 삭제하고 설정 파일에서 `hooks` 객체를 제거합니다.

<h2 id="next-steps">
  다음 단계
</h2>

* [플러그인 구성 요소](/docs/ko/plugins/components): 에이전트, 훅, MCP 서버, LSP 서버 및 사용자 구성을 플러그인에 추가합니다
* [플러그인으로 evals 테스트](/docs/ko/plugin-evals): eval 사례를 작성하고 `claude plugin eval`로 실행하여 플러그인이 Claude의 동작을 얼마나 안정적으로 안내하는지 확인합니다
* [플러그인 게시](/docs/ko/plugins/publish): 버전을 지정하고, 마켓플레이스에 넣고, 커뮤니티 마켓플레이스에 제출합니다
* [claude.ai 및 Cowork의 플러그인](https://claude.com/docs/plugins/overview): 동일한 플러그인 폴더가 claude.ai 및 Cowork에 설치됩니다. 일부 구성 요소는 Claude Code 전용입니다
* [플러그인 매니페스트 참조](/docs/ko/plugins/manifest-reference): 모든 `plugin.json` 필드, 경로 규칙 및 디렉토리
* [스킬](/docs/ko/skills): 플러그인이 제공하는 스킬을 작성합니다
* [Anthropic의 claude-code 저장소의 플러그인](https://github.com/anthropics/claude-code/tree/main/plugins): `feature-dev` 및 `code-review`와 같은 이 페이지의 레이아웃의 완전한 작업 예제
