> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 플러그인 매니페스트 참조

> plugin.json의 완전한 참조: 모든 필드의 타입과 기본값, 허용되는 경로 형식, userConfig 및 환경 변수 스키마.

플러그인 매니페스트는 플러그인의 `.claude-plugin/` 디렉토리에 있는 `plugin.json` 파일입니다. 플러그인의 메타데이터와 Claude Code가 사용자에게 요청하는 [`userConfig`](#user-configuration) 값을 포함합니다. 또한 인라인으로 정의하거나 [기본 위치](#standard-layout) 외부에 유지하는 모든 컴포넌트를 선언합니다.

이 참조는 플러그인 작성자와 플러그인 마켓플레이스 항목에 컴포넌트 필드를 추가하는 마켓플레이스 소유자를 위한 것입니다.

<Note>
  다음 경우는 다른 페이지에서 다룹니다:

  * **플러그인 빌드 학습**: [플러그인 생성](/docs/ko/plugins/create)부터 시작하세요
  * **각 컴포넌트가 런타임에 수행하는 작업**: [플러그인 컴포넌트](/docs/ko/plugins/components) 참조
</Note>

찾고 있는 내용과 일치하는 섹션부터 시작하세요:

* 필드: [필드 테이블](#fields)에서 각 필드의 타입, 필수 여부, 기본값 및 허용되는 항목을 제공합니다. [경로 규칙](#path-rules)은 모든 컴포넌트 경로에 대한 `./` 접두사 및 포함을 다룹니다
* `userConfig` 옵션 또는 `channels` 항목: [사용자 구성](#user-configuration) 및 [채널](#channels) 스키마
* `${CLAUDE_PLUGIN_ROOT}` 또는 플러그인이 참조할 수 있는 다른 변수: [환경 변수](#environment-variables)
* 각 컴포넌트의 파일이 위치하는 곳: [표준 레이아웃](#standard-layout)
* `claude plugin validate`의 메시지: [문제 해결 페이지](/docs/ko/plugins/troubleshooting)에서 각 메시지와 해결 방법, 이 페이지의 관련 섹션으로의 링크를 나열합니다

<h2 id="manifest-file">
  매니페스트 파일
</h2>

매니페스트는 선택 사항입니다. 없으면 Claude Code는 [표준 레이아웃](#standard-layout)에서 찾은 컴포넌트를 로드합니다. 그러면 플러그인 이름은 마켓플레이스 항목에서 오거나 `--plugin-dir`으로 플러그인을 로드할 때 디렉토리 이름에서 옵니다.

메타데이터, 기본 디렉토리 외부의 컴포넌트, `userConfig` 또는 인라인 컴포넌트 정의를 원할 때 매니페스트를 작성하세요.

매니페스트를 플러그인 루트 아래 `.claude-plugin/plugin.json`에 저장하세요. 다른 모든 플러그인 파일을 플러그인 루트에 배치하고 `.claude-plugin/` 내부에는 배치하지 마세요. 여기에는 `skills/`, `commands/` 및 `hooks/`가 포함됩니다.

다음 예제는 [필드 테이블](#fields)의 대부분의 키를 설정합니다. 각 참조된 경로를 포함하는 플러그인 디렉토리에서 유효성 검사를 통과합니다.

```json theme={null}
{
  "name": "deploy-tools",
  "displayName": "Deploy Tools",
  "version": "1.2.0",
  "description": "Deployment commands, a review agent, and a status monitor",
  "author": {
    "name": "Example Team",
    "email": "dev@example.com",
    "url": "https://example.com"
  },
  "homepage": "https://example.com/docs/deploy-tools",
  "repository": "https://github.com/example/deploy-tools",
  "license": "MIT",
  "keywords": ["deployment", "ci"],
  "defaultEnabled": true,
  "dependencies": ["secrets-vault"],
  "metadata": { "catalogId": "cat-123" },
  "skills": ["./extra-skills/"],
  "commands": {
    "status": {
      "source": "./commands/status.md",
      "description": "Show the current deployment status"
    },
    "about": {
      "content": "Explain what the deploy-tools plugin provides.",
      "description": "Describe this plugin"
    }
  },
  "agents": ["./agents/reviewer.md"],
  "hooks": "./config/extra-hooks.json",
  "mcpServers": {
    "deploy-api": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"]
    }
  },
  "lspServers": "./.lsp.json",
  "outputStyles": "./styles/",
  "experimental": {
    "themes": "./themes/",
    "monitors": "./config/monitors.json"
  },
  "userConfig": {
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "Token for the deployment API",
      "sensitive": true
    }
  }
}
```

<h3 id="unrecognized-fields">
  인식되지 않는 필드
</h3>

인식되지 않는 최상위 키는 제거되고, `userConfig` 옵션, `channels` 항목, `lspServers` 구성 또는 `monitors` 항목 내의 인식되지 않는 키는 거부됩니다:

* **최상위 필드**: 필드가 제거되고 플러그인이 로드됩니다. `claude plugin validate`는 각 인식되지 않는 최상위 필드를 경고로 보고합니다
* **엄격한 객체**: `userConfig` 옵션, `channels` 항목, `lspServers` 구성 및 `monitors` 항목은 엄격합니다. 내부의 알 수 없는 키는 오류이며 플러그인이 로드되지 않습니다

<h3 id="validate-the-manifest">
  매니페스트 유효성 검사
</h3>

`claude plugin validate`는 매니페스트에 대한 권위 있는 검사입니다. 셸에서 플러그인 디렉토리에 대해 실행하세요:

```bash theme={null}
claude plugin validate ./my-plugin
```

명령은 다음 결과 중 하나를 보고합니다:

* **`Validation passed`**: 매니페스트가 로드됩니다
* **`Validation passed with warnings`**: 매니페스트가 로드되지만 유효성 검사기가 수정할 사항을 발견했습니다. 예를 들어 Claude Code가 제거하는 알 수 없는 최상위 필드, kebab-case가 아닌 `name`, 누락된 `version`, `description` 또는 `author`입니다. CI에서 경고를 실패로 바꾸려면 `--strict`를 전달하세요
* **`Validation failed`**: 매니페스트에 타입 불일치, 누락되었거나 플러그인 루트를 벗어나는 경로, 또는 `userConfig` 옵션, `channels` 항목, `lspServers` 구성 또는 `monitors` 항목 내의 알 수 없는 키가 있습니다. Claude Code는 플러그인을 로드할 때 동일한 문제를 보고합니다

<h2 id="fields">
  필드
</h2>

테이블은 `plugin.json`의 최상위 키를 나열합니다. `name`은 유일한 필수 키입니다. 필드 이름이 링크인 경우 링크된 섹션에 전체 규칙이 있습니다.

`commands` 및 `hooks`와 같은 컴포넌트 키의 경우 [컴포넌트 경로 형식](#component-path-forms)은 예제와 함께 허용되는 각 형식을 보여주며, 모든 경로는 `./` 접두사, 확장자 및 포함에 대한 [경로 규칙](#path-rules)을 따릅니다.

| 필드                                   | 타입                               | 설명                                                                                                                                                                                                                                    |
| :----------------------------------- | :------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `$schema`                            | String                           | 편집기 자동 완성을 위한 JSON Schema URL. Claude Code는 로드 시 이를 무시합니다                                                                                                                                                                             |
| [`name`](#name)                      | String                           | 플러그인 식별자, 필수. kebab-case를 사용하세요. 모든 컴포넌트는 이 아래에 네임스페이스됩니다                                                                                                                                                                             |
| [`displayName`](#displayname)        | String                           | `name` 대신 UI에 표시되는 이름                                                                                                                                                                                                                 |
| [`version`](#version)                | String                           | 버전 문자열. 설정하면 변경할 때까지 사용자를 해당 버전에 유지합니다                                                                                                                                                                                                |
| `description`                        | String                           | 플러그인이 제공하는 것에 대한 간단한 설명                                                                                                                                                                                                               |
| `author`                             | Object                           | 필수인 `name`, 선택적인 `email` 및 `url`                                                                                                                                                                                                      |
| `homepage`                           | String                           | 문서 URL. URL로 구문 분석되어야 하거나 플러그인이 로드되지 않습니다                                                                                                                                                                                             |
| `repository`                         | String                           | 소스 저장소 URL. 유효성이 검사되지 않습니다                                                                                                                                                                                                            |
| `license`                            | String                           | `MIT` 또는 `Apache-2.0`과 같은 SPDX 식별자                                                                                                                                                                                                    |
| `keywords`                           | Array of strings                 | 검색 태그                                                                                                                                                                                                                                 |
| [`metadata`](#metadata)              | Object                           | 자신의 데이터를 위한 자유 형식 객체. Claude Code는 이를 읽지 않습니다                                                                                                                                                                                         |
| [`defaultEnabled`](#defaultenabled)  | Boolean                          | 사용자가 설정하지 않았을 때 플러그인이 활성화된 상태로 시작되는지 여부. 기본값은 `true`입니다                                                                                                                                                                               |
| [`dependencies`](#dependencies)      | Array of strings or objects      | 이 플러그인이 작동하기 위해 활성화되어야 하는 플러그인                                                                                                                                                                                                        |
| [`settings`](#settings)              | Object                           | 플러그인이 활성화된 동안 Claude Code가 적용하는 설정. `agent` 및 `subagentStatusLine`만 적용됩니다                                                                                                                                                             |
| [`userConfig`](#user-configuration)  | Object                           | 플러그인이 활성화될 때 Claude Code가 사용자에게 요청하는 값                                                                                                                                                                                                |
| [`channels`](#channels)              | Array of objects                 | 플러그인이 제공하는 메시지 채널, 각각 MCP 서버 중 하나에 바인딩됨                                                                                                                                                                                               |
| `skills`                             | Path, or array of paths          | 스킬을 스캔할 디렉토리, 각각 `<name>/SKILL.md` 폴더의 디렉토리 또는 `SKILL.md`를 직접 보유하는 하나의 폴더. `"."`는 플러그인 루트를 지정합니다. 기본 `skills/` 스캔에 추가됩니다                                                                                                              |
| [`commands`](#commands)              | Path, array of paths, or object  | 평면 `.md` 명령 파일, 이들의 디렉토리, 또는 명령 이름을 `source` 또는 `content`에 매핑하는 객체. 기본 `commands/` 스캔을 대체합니다                                                                                                                                          |
| `agents`                             | Path, or array of paths          | 에이전트 `.md` 파일. 디렉토리는 허용되지 않습니다. 기본 `agents/` 스캔을 대체합니다                                                                                                                                                                                |
| [`hooks`](#hooks)                    | Path, object, or array of either | `.json` 훅 파일 또는 인라인 훅 구성. `hooks/hooks.json`과 함께 로드됨                                                                                                                                                                                  |
| [`mcpServers`](#mcpservers)          | Path, object, or array of either | `.json` MCP 구성 파일, `.mcpb` 또는 `.dxt` 번들, 또는 이름으로 키가 지정된 인라인 서버 구성. `.mcp.json`과 함께 로드됨; 나중에 선언된 서버 이름은 이전 이름을 대체합니다                                                                                                                   |
| [`lspServers`](#lspservers)          | Path, object, or array of either | `.json` LSP 구성 파일 또는 이름으로 키가 지정된 인라인 서버 구성. `.lsp.json`과 함께 로드됨                                                                                                                                                                       |
| `outputStyles`                       | Path, or array of paths          | 출력 스타일 파일 또는 디렉토리. 기본 `output-styles/` 스캔을 대체합니다                                                                                                                                                                                      |
| `workflows`                          | Path, or array of paths          | [워크플로우](/docs/ko/workflows#distribute-a-workflow-in-a-plugin) `.js` 파일 또는 디렉토리. 기본 `workflows/` 스캔을 대체합니다                                                                                                                                  |
| `experimental`                       | Object                           | `themes`, `monitors` 및 `evals`의 컨테이너, 매니페스트 형식이 여전히 변경될 수 있음                                                                                                                                                                          |
| `experimental.themes`                | Path, or array of paths          | 테마 파일 또는 디렉토리. 기본 `themes/` 스캔을 대체합니다. 최상위 `themes` 키는 여전히 로드되며 `claude plugin validate` 경고가 표시됩니다                                                                                                                                    |
| [`experimental.monitors`](#monitors) | Path, or inline array            | 모니터 배열을 보유하는 `.json` 파일 또는 배열 자체. 기본값은 `monitors/monitors.json`입니다. 최상위 `monitors` 키는 여전히 로드되며 `claude plugin validate` 경고가 표시됩니다. 모니터는 대화형 세션에서만 실행되며 Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry에서는 실행되지 않습니다 |
| `experimental.evals`                 | Path, or array of paths          | 기본값이 `evals/`가 아닐 때 플러그인의 [평가 사례](/docs/ko/plugin-evals#use-a-different-eval-directory)를 보유하는 디렉토리. `claude plugin eval --eval-dir`이 이를 재정의합니다                                                                                             |

Type 열에서 경로는 `"./custom/commands"`와 같이 플러그인 루트에 상대적인 문자열입니다.

<h3 id="name">
  `name`
</h3>

플러그인 식별자. 공백, `@`, `:`, 경로 구분자, 제어 문자 또는 양방향 형식 문자가 없는 비어있지 않은 상태여야 합니다. kebab-case를 사용하세요.

Claude Code는 모든 컴포넌트를 이 아래에 네임스페이스하므로 플러그인 `deploy-tools`의 에이전트 `reviewer`는 `deploy-tools:reviewer`로 나타납니다.

<h3 id="displayname">
  `displayName`
</h3>

`name` 대신 UI에 표시되는 이름. 공백과 모든 대소문자를 포함할 수 있으며 네임스페이싱이나 조회에 사용되지 않습니다.

마켓플레이스에서 설치된 플러그인의 경우 [마켓플레이스 항목](/docs/ko/plugins/marketplace-reference#plugin-entries)의 `displayName`이 이 값보다 우선합니다.

<h3 id="version">
  `version`
</h3>

semver에 대해 검사되지 않는 버전 문자열. 설정하면 변경할 때까지 플러그인을 해당 버전에 고정합니다. [버전 및 업데이트](/docs/ko/plugins/loading#versions-and-updates)를 참조하세요. [`command` 소스](/docs/ko/plugins/marketplace-reference)가 있는 플러그인, [claude.ai에서 호스팅되는 마켓플레이스](/docs/ko/plugins/install#add-from-claude-ai)의 플러그인, 그리고 로컬 디렉토리로 추가된 마켓플레이스에서 [제자리에 로드](/docs/ko/plugins/loading#find-plugins-on-disk)된 플러그인은 이 필드로 고정되지 않습니다.

<h3 id="metadata">
  `metadata`
</h3>

카탈로그 또는 자격 필드와 같은 자신의 데이터를 위한 자유 형식 객체. Claude Code는 이를 읽지 않습니다. Claude Code v2.1.222 이상이 필요합니다.

<h3 id="defaultenabled">
  `defaultEnabled`
</h3>

사용자가 [`enabledPlugins`](/docs/ko/settings-reference#enabledplugins)에서 설정하지 않았을 때 플러그인이 활성화된 상태로 시작되는지 여부. 기본값은 `true`입니다. 활성화된 플러그인이 의존하는 플러그인은 관계없이 활성화된 상태로 시작됩니다. 마켓플레이스 항목의 동일한 필드가 이 필드를 재정의합니다.

사용자의 `enabledPlugins` 항목이 작성되면 플러그인 업데이트 전체에서 유지되므로 나중 릴리스에서 `defaultEnabled`를 변경해도 기존 사용자의 설정은 변경되지 않습니다.

<h3 id="dependencies">
  `dependencies`
</h3>

이 플러그인이 작동하기 위해 활성화되어야 하는 플러그인. 각 항목은 `"name"`, `"name@marketplace"` 또는 `{ "name": "...", "marketplace": "...", "version": "..." }`입니다. 베어 이름은 이 플러그인 자신의 마켓플레이스에 대해 해결됩니다. [의존성 제약](/docs/ko/plugins/dependencies)을 참조하세요.

<h3 id="settings">
  `settings`
</h3>

플러그인이 활성화된 동안 Claude Code가 적용하는 설정. `agent` 및 `subagentStatusLine`만 적용됩니다. 다른 키는 로드 시 제거됩니다. 플러그인 루트의 `settings.json`이 이 키보다 우선합니다. [기본 설정](/docs/ko/plugins/components#default-settings)을 참조하세요.

<h2 id="component-path-forms">
  컴포넌트 경로 형식
</h2>

모든 컴포넌트 키는 플러그인 루트에 상대적인 경로를 허용합니다. `hooks`, `mcpServers`, `lspServers` 및 `experimental.monitors`는 또한 인라인 구성을 허용하고, `commands`는 또한 객체 맵을 허용하며, `mcpServers`는 또한 MCP 번들 경로 및 URL을 허용합니다. 다음 예제는 허용되는 각 형식을 한 번씩 보여줍니다. 각 컴포넌트가 런타임에 수행하는 작업은 [플러그인 컴포넌트](/docs/ko/plugins/components)를 참조하세요.

<h3 id="path-only-fields">
  경로 전용 필드
</h3>

`agents`, `skills`, `outputStyles`, `workflows` 및 `experimental.themes`는 하나의 경로 또는 경로 배열을 사용합니다. `agents` 항목은 `.md` 파일이어야 하고 `skills` 항목은 디렉토리여야 합니다. 다른 세 개는 디렉토리 또는 파일을 허용합니다.

```json theme={null}
{
  "agents": ["./custom-agents/reviewer.md", "./custom-agents/tester.md"],
  "skills": ["./extra-skills/", "."],
  "outputStyles": "./styles/"
}
```

<h3 id="commands">
  `commands`
</h3>

`commands`는 경로, 경로 배열 또는 객체 맵을 사용합니다. 경로는 평면 `.md` 명령 파일 또는 디렉토리를 지정합니다. 객체 맵에서 각 키는 플러그인 접두사 후 명령 이름이 됩니다. 예를 들어 플러그인 `deploy-tools`의 `"about"`은 `/deploy-tools:about`으로 실행됩니다.

각 값은 정확히 `source` 또는 `content` 중 하나를 설정하며, 둘 다 설정하거나 둘 다 설정하지 않는 항목은 유효성 검사에 실패합니다. 이 테이블의 다른 필드는 선택 사항입니다:

| 필드             | 타입               | 설명                               |
| :------------- | :--------------- | :------------------------------- |
| `source`       | string           | 명령의 Markdown 파일 경로, 플러그인 루트에 상대적 |
| `content`      | string           | `source` 대신 명령 본문의 인라인 Markdown  |
| `description`  | string           | 명령에 대해 표시되는 설명                   |
| `argumentHint` | string           | 명령 이름 뒤에 표시되는 인수 힌트, 예: `[file]` |
| `model`        | string           | 명령의 기본 모델                        |
| `allowedTools` | array of strings | 명령이 프롬프트 없이 사용할 수 있는 도구          |

이 맵은 파일의 한 명령과 인라인 콘텐츠의 한 명령을 선언합니다:

```json theme={null}
{
  "commands": {
    "status": { "source": "./commands/status.md", "argumentHint": "[env]" },
    "about": { "content": "Explain what this plugin provides." }
  }
}
```

<h3 id="hooks">
  `hooks`
</h3>

`hooks`는 `.json` 파일 경로, [`settings.json`의 `hooks`](/docs/ko/hooks#configuration)와 동일한 형식의 인라인 훅 객체, 또는 둘을 혼합하는 배열을 사용합니다. 훅 이벤트 및 핸들러 필드는 [훅 참조](/docs/ko/hooks#hook-events)를 참조하세요.

Claude Code는 해당 파일이 존재할 때 `hooks/hooks.json`과 함께 선언한 모든 것을 병합합니다.

```json theme={null}
{
  "hooks": [
    "./config/extra-hooks.json",
    {
      "PostToolUse": [
        {
          "matcher": "Write|Edit",
          "hooks": [
            { "type": "command", "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/format.sh" }
          ]
        }
      ]
    }
  ]
}
```

<h3 id="mcpservers">
  `mcpServers`
</h3>

`mcpServers`는 `.json` 파일 경로, MCP 번들 경로 또는 URL, 인라인 맵, 또는 이들을 혼합하는 배열을 사용합니다. 서버 구성 필드는 [플러그인 제공 MCP 서버](/docs/ko/mcp#plugin-provided-mcp-servers)를 참조하세요.

Claude Code는 먼저 플러그인 루트의 `.mcp.json`을 로드한 다음 선언된 각 형식을 순서대로 로드합니다. 나중에 선언된 서버 이름은 이전 이름을 대체합니다.

`mcpServers` 값은 다음 형식 중 하나를 사용합니다:

| 형식            | 예제 값                                                                                   | Claude Code가 수행하는 작업                                            |
| :------------ | :------------------------------------------------------------------------------------- | :-------------------------------------------------------------- |
| `.json` 파일 경로 | `"./mcp/servers.json"`                                                                 | 파일을 `mcpServers` 맵으로 읽음                                         |
| MCP 번들 경로     | `"./bundle.mcpb"`                                                                      | `.mcpb` 또는 `.dxt` 번들을 플러그인 루트 아래 `.mcpb-cache/`로 추출하고 서버 구성을 읽음 |
| MCP 번들 URL    | `"https://example.com/server.mcpb"`                                                    | 번들을 `.mcpb-cache/`로 다운로드한 다음 읽음                                 |
| 인라인 맵         | `{ "deploy-api": { "command": "node", "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"] } }` | 맵을 이름으로 키가 지정된 서버 구성으로 사용                                       |

번들 경로 또는 URL은 `.mcpb` 또는 `.dxt`로 끝나야 합니다. 다른 확장자는 유효성 검사에 실패합니다.

<h3 id="lspservers">
  `lspServers`
</h3>

`lspServers`는 `.json` 파일 경로, 서버 이름을 구성에 매핑하는 인라인 맵, 또는 둘의 배열을 사용합니다.

Claude Code는 먼저 플러그인 루트의 `.lsp.json`을 로드한 다음 선언된 각 구성을 순서대로 로드합니다. 나중에 선언된 서버 이름은 이전 이름을 대체합니다.

각 서버 구성은 다음 필드가 있는 엄격한 객체입니다. 알 수 없는 키는 유효성 검사에 실패합니다.

| 필드                      | 필수  | 설명                                                                                                                |
| :---------------------- | :-- | :---------------------------------------------------------------------------------------------------------------- |
| `command`               | Yes | 언어 서버 바이너리. 값이 `/`로 시작하지 않으면 공백 없음; 인수를 `args`에 배치                                                                |
| `extensionToLanguage`   | Yes | 파일 확장자를 LSP 언어 ID에 매핑, 최소 하나의 항목. 키는 `.go`와 같이 점으로 시작                                                             |
| `args`                  | No  | 서버에 전달되는 인수                                                                                                       |
| `transport`             | No  | 통신 전송: `stdio`(기본값) 또는 `socket`. Claude Code는 `socket`을 허용하지만 모든 서버를 stdio를 통해 실행하므로 stdout 프로토콜 규칙이 모든 서버에 적용됩니다 |
| `env`                   | No  | 서버 프로세스의 환경 변수                                                                                                    |
| `initializationOptions` | No  | initialize 요청에서 전송되는 옵션                                                                                           |
| `settings`              | No  | `workspace/didChangeConfiguration`으로 전송되는 설정                                                                      |
| `workspaceFolder`       | No  | 서버의 작업 공간 폴더 경로                                                                                                   |
| `startupTimeout`        | No  | 시작을 기다릴 밀리초, 양의 정수                                                                                                |
| `shutdownTimeout`       | No  | 정상 종료를 기다릴 밀리초, 양의 정수. 시간 초과가 경과하면 Claude Code가 서버 프로세스를 종료합니다. 설정하지 않으면 시간 초과가 적용되지 않습니다                         |
| `restartOnCrash`        | No  | 서버가 충돌한 후 다시 시작할지 여부. 기본값은 `true`입니다. 충돌한 서버를 다시 시작하지 않고 중지된 상태로 두려면 `false`로 설정                                  |
| `maxRestarts`           | No  | 포기하기 전 재시작 시도, 0 이상                                                                                               |
| `diagnostics`           | No  | 편집 후 진단을 컨텍스트에 푸시할지 여부. 기본값은 `true`입니다                                                                            |

이 인라인 구성은 `.go` 파일에 대해 `gopls`를 실행합니다:

```json theme={null}
{
  "lspServers": {
    "go": {
      "command": "gopls",
      "args": ["serve"],
      "extensionToLanguage": { ".go": "go" }
    }
  }
}
```

Anthropic이 플러그인으로 게시하는 언어 서버 및 서버가 런타임에 동작하는 방식은 [코드 인텔리전스](/docs/ko/plugins/code-intelligence)를 참조하세요.

<h3 id="monitors">
  `monitors`
</h3>

`experimental.monitors`는 `.json` 파일 경로 또는 인라인 배열을 사용합니다. 키를 생략하면 Claude Code는 존재하는 경우 `monitors/monitors.json`을 로드합니다.

각 항목은 다음 필드가 있는 엄격한 객체입니다.

| 필드            | 필수  | 설명                                                                                                         |
| :------------ | :-- | :--------------------------------------------------------------------------------------------------------- |
| `name`        | Yes | 플러그인 내에서 고유한 식별자                                                                                           |
| `command`     | Yes | Claude Code가 세션 작업 디렉토리에서 지속적인 백그라운드 프로세스로 실행하는 셸 명령                                                       |
| `description` | Yes | 작업 패널 및 알림 요약에 표시되는 간단한 요약                                                                                 |
| `when`        | No  | `"always"`(기본값)인 경우 모니터는 세션 시작 및 플러그인 다시 로드 시 시작됩니다. `"on-skill-invoke:<skill>"`인 경우 해당 스킬이 처음 실행될 때 시작됩니다 |

이 인라인 배열은 `deploy` 스킬이 처음 실행될 때 시작되는 하나의 모니터를 선언합니다:

```json theme={null}
{
  "experimental": {
    "monitors": [
      {
        "name": "deploy-status",
        "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/poll-deploy.sh",
        "description": "Deployment status changes",
        "when": "on-skill-invoke:deploy"
      }
    ]
  }
}
```

모니터 `command`는 `${user_config.*}`를 참조할 수 없습니다. [셸을 통해 실행되는 필드](#fields-that-run-through-a-shell)를 참조하세요.

<h2 id="path-rules">
  경로 규칙
</h2>

매니페스트의 모든 컴포넌트 경로는 플러그인 루트에 상대적이며 `./`로 시작해야 합니다. `commands/foo.md`와 같은 경로는 유효성 검사에 실패합니다. `skills` 및 `mcpServers`는 각각 해당 규칙 외부의 한 형식을 허용합니다:

* **`skills`**: 또한 `"."`를 허용합니다. `"."`와 `"./"`는 모두 플러그인 루트를 나타냅니다. v2.1.221 이전에는 `"."`가 매니페스트 유효성 검사에 실패했으므로 플러그인이 이전 버전에서 로드되어야 할 때 `"./"`를 사용하세요
* **`mcpServers`**: 또한 `https://` 번들 URL을 허용합니다

<h3 id="containment-and-existence">
  포함 및 존재
</h3>

모든 컴포넌트 경로는 플러그인 루트 내부로 해결되어야 하며 존재해야 합니다. `claude plugin validate`는 `outputStyles`, `lspServers`, `monitors` 또는 `themes` 경로를 검사하지 않으므로 이러한 필드의 잘못된 경로는 플러그인이 로드될 때만 실패합니다:

* **포함**: 플러그인 루트 외부로 해결되는 경로는 로드되지 않으며 `/plugin` **Errors** 탭에 `<component> path escapes plugin directory: <path>`가 표시됩니다. `..`를 포함하는 경로가 일반적인 경우이며 `claude plugin validate`는 이를 `Path contains ".." which could be a path traversal attempt`로 보고합니다
* **존재**: 존재하지 않는 경로는 로드되지 않으며 `/plugin` **Errors** 탭에 `<component> path not found: <path>`가 표시됩니다. `claude plugin validate`는 이를 `Path not found`로 보고합니다

<h3 id="how-each-key-combines-with-its-default-location">
  각 키가 기본 위치와 결합되는 방식
</h3>

각 컴포넌트 키는 기본 위치를 대체하거나, 추가하거나, 병합합니다:

* **기본값 대체**: `commands`, `agents`, `outputStyles`, `workflows`, `experimental.themes`, `experimental.monitors`. `commands`를 설정하면 기본 `commands/` 디렉토리가 스캔되지 않습니다. 기본값을 유지하고 더 추가하려면 명시적으로 나열하세요: `"commands": ["./commands/", "./extras/"]`
* **기본값에 추가**: `skills`. `skills/` 디렉토리는 여전히 스캔되며 나열된 디렉토리는 함께 로드됩니다
* **병합**: `hooks`, `mcpServers`, `lspServers`. 기본 파일이 먼저 로드되고 매니페스트가 선언한 것이 병합됩니다. [컴포넌트 경로 형식](#component-path-forms)에서 설명한 대로

플러그인에 `commands/`와 같은 기본 폴더가 있고 이를 대체하는 매니페스트 키도 설정하면 Claude Code는 매니페스트 경로를 로드하고 폴더는 로드하지 않습니다. `claude plugin list` 및 `/plugin` 인터페이스는 경고 `Default <folder>/ folder is ignored because the manifest sets "<key>"`를 표시합니다.

경고를 피하려면 키를 해당 폴더 내의 경로로 설정하세요: `"commands": ["./commands/deploy.md"]`는 기본 폴더의 파일을 지정하고 경고를 생성하지 않습니다.

<h2 id="user-configuration">
  사용자 구성
</h2>

`userConfig`는 플러그인이 활성화될 때 Claude Code가 사용자에게 요청하는 값을 선언하므로 사용자는 `settings.json`을 직접 편집하지 않습니다.

키는 문자, 숫자 및 밑줄로 만든 식별자이며 숫자로 시작할 수 없습니다.

각 값은 다음 필드가 있는 엄격한 객체입니다. 알 수 없는 키는 유효성 검사에 실패합니다.

| 필드            | 필수  | 설명                                                                                                                                        |
| :------------ | :-- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| `type`        | Yes | `string`, `number`, `boolean`, `directory` 또는 `file` 중 하나                                                                                 |
| `title`       | Yes | 구성 대화 상자에 표시되는 레이블                                                                                                                        |
| `description` | Yes | 필드 아래에 표시되는 도움말 텍스트                                                                                                                       |
| `required`    | No  | `true`인 경우 구성 대화 상자는 빈 값을 허용하지 않습니다                                                                                                       |
| `default`     | No  | 사용자가 아무것도 제공하지 않을 때 사용되는 값: 문자열, 숫자, 부울 또는 문자열 배열                                                                                         |
| `options`     | No  | `string`의 경우 필드가 허용하는 값, `/config`에서 선택기로 표시됩니다. [필드를 고정 옵션으로 제한](#limit-a-field-to-fixed-options)을 참조하세요. Claude Code v2.1.271 이상이 필요합니다 |
| `multiple`    | No  | `string`의 경우 문자열 배열을 허용합니다                                                                                                                |
| `sensitive`   | No  | `true`인 경우 입력을 마스크하고 `settings.json` 대신 보안 저장소에 값을 저장합니다                                                                                  |
| `min` / `max` | No  | `number`의 범위                                                                                                                              |

각 활성화된 플러그인의 각 옵션은 `/config` 패널의 행으로도 나타나며, `sensitive` 옵션 및 `multiple` 목록은 제외됩니다. `/config` 행에는 Claude Code v2.1.269 이상이 필요합니다.

이 `userConfig`는 엔드포인트와 마스크된 토큰을 선언합니다:

```json theme={null}
{
  "userConfig": {
    "api_endpoint": {
      "type": "string",
      "title": "API endpoint",
      "description": "Your team's API endpoint"
    },
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "API authentication token",
      "sensitive": true
    }
  }
}
```

<h3 id="limit-a-field-to-fixed-options">
  필드를 고정 옵션으로 제한
</h3>

`userConfig` 필드에 `options`를 설정하여 사용자가 고정 목록에서 값을 선택하도록 합니다.

`tone` 필드를 세 가지 옵션으로 제한하려면 `options`에 나열하고 `default`를 그 중 하나로 설정하세요:

```json theme={null}
{
  "userConfig": {
    "tone": {
      "type": "string",
      "title": "Tone",
      "description": "Voice for generated replies",
      "options": ["neutral", "warm", "formal"],
      "default": "neutral"
    }
  }
}
```

모든 필드에 `options`를 선언하면 Claude Code v2.1.271 이전 버전의 사용자는 플러그인을 로드할 수 없습니다.

`options`는 `multiple` 또는 `sensitive`가 아닌 `string` 필드에 적용됩니다. `default`를 나열된 값 중 하나로 설정하거나 사용자가 하나를 선택하도록 `required: true`를 설정하세요. 각 옵션은 1\~64자의 일반 레이블이며 셸에서 실행하는 `claude plugin validate`는 거부하는 다른 항목을 보고합니다. `options`가 이러한 규칙을 위반하는 플러그인은 로드되지 않습니다.

<h3 id="where-values-are-stored">
  값이 저장되는 위치
</h3>

민감하지 않은 값은 사용자의 `settings.json`의 [`pluginConfigs`](/docs/ko/settings-reference#pluginconfigs) 아래에 저장됩니다. 민감한 값은 플랫폼의 보안 자격 증명 저장소로 이동합니다. [설정 페이지](/docs/ko/settings-reference#pluginconfigs)는 `pluginConfigs`가 읽혀지는 설정 파일을 나열합니다.

<h3 id="reference-a-saved-value">
  저장된 값 참조
</h3>

플러그인이 필요한 곳에서 저장된 값을 참조하세요. 두 가지 형식 중 하나:

* **`${user_config.KEY}`**: MCP 서버 구성, LSP 서버 구성, [exec-form](/docs/ko/hooks#exec-form-and-shell-form) 훅 `args` 및 스킬과 에이전트 콘텐츠에서 대체됩니다. 스킬과 에이전트 콘텐츠에서는 민감하지 않은 값만 대체되며 민감한 값은 자리 표시자가 됩니다
* **`CLAUDE_PLUGIN_OPTION_<KEY>`**: 모든 옵션에 대해 훅 프로세스로 내보내집니다. `<KEY>`는 대문자입니다. 셸 형식 훅은 `api_token`에 대해 `$CLAUDE_PLUGIN_OPTION_API_TOKEN`을 읽습니다

<h3 id="fields-that-run-through-a-shell">
  셸을 통해 실행되는 필드
</h3>

셸 형식 훅 명령, 모니터 명령 및 MCP [`headersHelper`](/docs/ko/mcp#use-dynamic-headers-for-custom-authentication)는 `${user_config.*}`를 거부합니다. 이러한 필드 중 하나에서 이를 참조하는 컴포넌트는 실행되지 않고 [오류](/docs/ko/errors#plugin-command-references-user-config)로 실패합니다. 필드의 값이 대체된 값을 다시 구문 분석할 셸에 전달되기 때문입니다.

테이블은 값이 이러한 각 필드에 도달할 수 있는 방법을 보여줍니다.

| 필드                  | 값이 도달할 수 있는 방법                                                                                                                                             |
| :------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 셸 형식 훅 명령           | [exec form](/docs/ko/hooks#exec-form-and-shell-form)을 `args`와 함께 사용하거나 훅의 환경에서 `CLAUDE_PLUGIN_OPTION_<KEY>`를 읽습니다                                               |
| 모니터 명령              | Claude Code를 통하지 않습니다. 모니터 프로세스는 `CLAUDE_PLUGIN_OPTION_<KEY>`를 받지 않으므로 모니터 스크립트가 값을 직접 얻어야 합니다                                                             |
| MCP `headersHelper` | Claude Code를 통하지 않습니다. 헬퍼의 환경은 `CLAUDE_PLUGIN_ROOT`, `CLAUDE_CODE_MCP_SERVER_NAME` 및 `CLAUDE_CODE_MCP_SERVER_URL`을 전달하지만 옵션 값은 없으므로 헬퍼 스크립트가 값을 직접 얻어야 합니다 |

<h2 id="channels">
  채널
</h2>

`channels`는 플러그인이 제공하는 메시지 채널(예: 채팅 앱으로의 브리지)을 선언합니다. 하나를 선언하면 Claude Code는 플러그인이 활성화될 때 채널의 구성을 요청할 수 있습니다. 서버가 메시지를 주입하는 방식은 [채널 참조](/docs/ko/channels-reference#package-as-a-plugin)를 참조하세요.

각 항목은 플러그인의 MCP 서버 중 하나에 바인딩된 엄격한 객체이며 다음 필드가 있습니다:

| 필드            | 필수  | 설명                                                                                                        |
| :------------ | :-- | :-------------------------------------------------------------------------------------------------------- |
| `server`      | Yes | 채널이 바인딩되는 이 플러그인의 `mcpServers`의 MCP 서버 키                                                                  |
| `displayName` | No  | 구성 대화 상자 제목에 표시되는 이름. 기본값은 서버 이름입니다                                                                       |
| `userConfig`  | No  | 요청할 옵션, [최상위 `userConfig`](#user-configuration)와 동일한 형식. 저장된 값은 서버의 `env`의 `${user_config.KEY}` 참조로 대체됩니다 |

이 매니페스트는 채널을 플러그인의 `telegram` MCP 서버에 바인딩하고 서버의 `env`로 대체되는 봇 토큰을 요청합니다:

```json theme={null}
{
  "mcpServers": {
    "telegram": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"],
      "env": { "BOT_TOKEN": "${user_config.bot_token}" }
    }
  },
  "channels": [
    {
      "server": "telegram",
      "displayName": "Telegram",
      "userConfig": {
        "bot_token": {
          "type": "string",
          "title": "Bot token",
          "description": "Telegram bot token",
          "sensitive": true
        }
      }
    }
  ]
}
```

<h2 id="environment-variables">
  환경 변수
</h2>

Claude Code는 플러그인 컴포넌트에 세 가지 경로 변수를 제공합니다. [각 변수가 해결되는 위치](#where-each-variable-resolves)에 나열된 필드에서 `${NAME}`으로 참조하고 이를 받는 프로세스에서 환경 변수로 읽으세요.

| 변수                      | 해결되는 대상                                                                                                                 | 사용 목적                                   |
| :---------------------- | :---------------------------------------------------------------------------------------------------------------------- | :-------------------------------------- |
| `${CLAUDE_PLUGIN_ROOT}` | 플러그인의 설치된 버전의 절대 경로                                                                                                     | 플러그인과 함께 번들된 스크립트, 바이너리 및 구성 파일         |
| `${CLAUDE_PLUGIN_DATA}` | `~/.claude/plugins/data/<id>/`, 첫 참조 시 생성되고 플러그인 업데이트 전체에서 유지됨. `<id>`는 문자, 숫자, `_` 또는 `-` 이외의 모든 문자가 `-`로 대체된 플러그인 식별자 | `node_modules`와 같은 설치된 의존성, 생성된 코드 및 캐시 |
| `${CLAUDE_PROJECT_DIR}` | 프로젝트 루트                                                                                                                 | 프로젝트 로컬 스크립트 및 구성 파일                    |

`${CLAUDE_PLUGIN_ROOT}`는 플러그인이 업데이트될 때 변경되므로 상태를 거기에 쓰지 마세요. 루트가 이동하는 위치와 이전 디렉토리가 정리되는 시기는 [로딩 페이지](/docs/ko/plugins/loading)를 참조하세요.

마지막으로 플러그인을 설치한 곳에서 플러그인을 제거하면 [`--keep-data`](/docs/ko/plugins/cli-reference)를 전달하지 않는 한 `${CLAUDE_PLUGIN_DATA}` 디렉토리가 삭제됩니다.

<h3 id="where-each-variable-resolves">
  각 변수가 해결되는 위치
</h3>

각 플러그인 컴포넌트에서 `${...}` 참조는 특정 필드에서 인라인으로 해결되며 일부 컴포넌트는 또한 프로세스 환경에서 변수를 받습니다:

| 플러그인 컴포넌트                  | `${...}`이 해결되는 필드                           | 프로세스로 내보내짐                                                                                      |
| :------------------------- | :------------------------------------------ | :---------------------------------------------------------------------------------------------- |
| 훅 명령                       | `command` 및 `args`의 어디든지                    | `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`, `CLAUDE_PROJECT_DIR` 및 `CLAUDE_PLUGIN_OPTION_<KEY>` |
| 모니터 명령                     | `command`의 어디든지                             | 내보내지지 않음                                                                                        |
| MCP `stdio` 서버             | `command`, `args`, `env`                    | `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`                                                      |
| MCP `http`, `sse`, `ws` 서버 | `url`, `headers`, `headersHelper`           | 해당 없음                                                                                           |
| LSP 서버                     | `command`, `args`, `env`, `workspaceFolder` | `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`, `CLAUDE_PROJECT_DIR`                                |
| 스킬, 명령 및 에이전트 콘텐츠          | Markdown 본문의 어디든지                           | 해당 없음                                                                                           |

변수는 Bash 도구를 통해 Claude가 실행하는 명령의 환경에 없으며, 주 세션이나 서브에이전트에도 없습니다. 스킬, 명령 및 에이전트 콘텐츠에서 Markdown 본문에 `${...}` 참조를 작성하고 Claude Code는 콘텐츠를 로드할 때 경로를 인라인으로 대체합니다.

<h3 id="quoting-and-path-separators">
  인용 및 경로 구분자
</h3>

각 대체된 경로를 단일 인수로 유지하세요:

* **훅 명령**: [exec form](/docs/ko/hooks#exec-form-and-shell-form)을 `args`와 함께 사용하여 각 경로가 인용 없이 하나의 인수가 되도록 합니다
* **셸 형식 훅 및 모니터 명령**: 변수를 큰따옴표로 감싸서 공백이 있는 경로가 한 단어로 유지되도록 합니다

이 셸 형식 훅은 플러그인과 함께 번들된 스크립트를 실행합니다:

```json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}\"/scripts/process.sh"
          }
        ]
      }
    ]
  }
}
```

Windows에서 대체된 경로는 앞으로 슬래시를 사용하므로 셸이 백슬래시를 이스케이프로 읽지 않습니다.

<h2 id="standard-layout">
  표준 레이아웃
</h2>

각 컴포넌트 타입은 매니페스트가 다른 곳을 가리키지 않을 때 사용되는 플러그인 루트 아래의 기본 위치를 가집니다.

| 컴포넌트   | 기본 위치                        | 내용                                                                                                                                                                                                                                        |
| :----- | :--------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 매니페스트  | `.claude-plugin/plugin.json` | 플러그인 메타데이터 및 구성. 선택 사항                                                                                                                                                                                                                    |
| 스킬     | `skills/`                    | 스킬당 하나의 `<name>/SKILL.md`. 루트에 `SKILL.md`가 있고 `skills/`가 없으며 `skills` 키가 없는 플러그인은 단일 스킬로 로드됩니다                                                                                                                                            |
| 명령     | `commands/`                  | 평면 Markdown 명령 파일. 새 플러그인의 경우 `skills/`를 선호합니다                                                                                                                                                                                            |
| 에이전트   | `agents/`                    | 에이전트 Markdown 파일. 하위 폴더는 [에이전트 이름](/docs/ko/plugins/components#agents)의 일부입니다                                                                                                                                                                  |
| 훅      | `hooks/hooks.json`           | 훅 구성                                                                                                                                                                                                                                      |
| MCP 서버 | `.mcp.json`                  | MCP 서버 정의                                                                                                                                                                                                                                 |
| LSP 서버 | `.lsp.json`                  | LSP 서버 구성                                                                                                                                                                                                                                 |
| 출력 스타일 | `output-styles/`             | 출력 스타일 Markdown 파일                                                                                                                                                                                                                        |
| 워크플로우  | `workflows/`                 | 워크플로우 `.js` 파일                                                                                                                                                                                                                            |
| 테마     | `themes/`                    | 테마 JSON 파일                                                                                                                                                                                                                                |
| 모니터    | `monitors/monitors.json`     | 모니터 배열                                                                                                                                                                                                                                    |
| 실행 파일  | `bin/`                       | 여기의 파일은 플러그인이 활성화된 동안 Bash 도구의 `PATH`에 있으므로 Claude는 이들을 베어 명령으로 실행합니다. claude.ai 및 Cowork는 이 디렉토리가 있는 플러그인을 설치하지 않습니다. 여기에는 [claude.ai 조직 설정을 통해 배포](/docs/ko/plugins/host-marketplace#distribute-through-organization-settings)하는 플러그인도 포함됩니다 |
| 설정     | `settings.json`              | 플러그인이 활성화된 동안 적용되는 `agent` 및 `subagentStatusLine` 기본값                                                                                                                                                                                     |

모든 기본 위치를 사용하는 플러그인과 훅이 호출하는 `scripts/` 폴더는 다음과 같이 배치됩니다:

```text theme={null}
deploy-tools/
├── .claude-plugin/
│   └── plugin.json
├── skills/
│   └── deploy/
│       └── SKILL.md
├── commands/
│   └── status.md
├── agents/
│   └── reviewer.md
├── hooks/
│   └── hooks.json
├── monitors/
│   └── monitors.json
├── output-styles/
│   └── terse.md
├── themes/
│   └── dracula.json
├── workflows/
│   └── release-audit.js
├── bin/
│   └── deploy-tool
├── scripts/
│   └── format.sh
├── settings.json
├── .mcp.json
└── .lsp.json
```

이 레이아웃을 클릭하고 각 파일이 수행하는 작업을 읽으려면 [플러그인 탐색기](/docs/ko/plugins/components#explore-the-plugin-directory)를 열으세요.

플러그인 루트의 `CLAUDE.md`는 컨텍스트로 로드되지 않으며 `claude plugin validate`는 하나를 찾으면 경고합니다. Claude의 컨텍스트에 로드되는 지침을 포함하려면 스킬에 배치하세요.

<h2 id="marketplace-entries-and-the-manifest">
  마켓플레이스 항목 및 매니페스트
</h2>

[마켓플레이스 항목](/docs/ko/plugins/marketplace-reference)은 [자신의 필드](/docs/ko/plugins/marketplace-reference#plugin-entries)와 함께 이 페이지의 모든 필드를 허용합니다. `strict` 포함.

`strict` 필드는 항목이 자신의 `plugin.json`을 가진 플러그인에 컴포넌트를 추가할 수 있는지 여부를 결정합니다. 기본값은 `true`입니다.

<h3 id="how-entry-fields-combine-with-plugin-json">
  항목 필드가 `plugin.json`과 결합되는 방식
</h3>

항목은 매니페스트로 제공되거나 컴포넌트를 추가하거나 충돌합니다:

* **`plugin.json` 없음**: 항목은 `strict`에 관계없이 매니페스트입니다. 항목 `hooks`는 인라인 객체 형식으로만 로드됩니다. 파일 경로 또는 배열의 경우 `/plugin` **Errors** 탭에 `not yet supported in a marketplace entry` 오류가 표시됩니다
* **`plugin.json` 있음, `strict` 설정 안 됨 또는 `true`**: Claude Code는 매니페스트를 로드하고 항목의 `commands`, `agents`, `skills`, `outputStyles` 및 `themes`를 추가합니다. `hooks`의 경우 항목의 이벤트 매처는 매니페스트의 동일한 이벤트 매처를 대체하고 매니페스트만 선언하는 이벤트는 자신의 것을 유지합니다
* **`plugin.json` 있음, `strict: false`**: `commands`, `agents`, `skills`, `hooks`, `outputStyles` 또는 `themes` 중 하나를 선언하는 항목은 충돌이며 플러그인은 `Plugin <name> has conflicting manifests`로 로드되지 않습니다

[`source`가 마켓플레이스 루트인 마켓플레이스 항목](/docs/ko/plugins/marketplace-reference)이 특정 `skills` 하위 디렉토리를 나열하면 해당 하위 디렉토리만 로드되고 플러그인의 기본 `skills/` 디렉토리는 스캔되지 않습니다. 매니페스트의 `skills` 키는 대신 [기본값에 추가](#how-each-key-combines-with-its-default-location)합니다.

<h3 id="metadata-precedence">
  메타데이터 우선순위
</h3>

일부 메타데이터 필드는 `strict`에 관계없이 고정 우선순위를 가집니다:

* **`defaultEnabled` 및 표시 필드**: 항목의 `defaultEnabled` 및 [표시 필드](/docs/ko/plugins/marketplace-reference#entry-and-plugin-json)(`displayName` 등)는 매니페스트의 것을 재정의합니다
* **`version`**: 매니페스트의 `version`은 항목의 것을 재정의합니다
* **`name`**: 항목이 플러그인을 매니페스트와 다른 `name`으로 나열할 때 `enabledPlugins`는 항목 이름을 사용하고 컴포넌트는 매니페스트 이름 아래에 네임스페이스됩니다

전체 우선순위 테이블은 [엄격한 모드](/docs/ko/plugins/marketplace-reference)를 참조하세요.

<h2 id="next-steps">
  다음 단계
</h2>

* [플러그인에 컴포넌트 추가](/docs/ko/plugins/components): 각 컴포넌트가 런타임에 수행하는 작업, 유효성 검사 예제 포함
* [마켓플레이스 참조](/docs/ko/plugins/marketplace-reference): 마켓플레이스가 플러그인에 대해 설정할 수 있는 항목 필드
* [플러그인 명령 참조](/docs/ko/plugins/cli-reference#plugin-validate): `claude plugin validate` 플래그 및 출력
* [플러그인 문제 해결](/docs/ko/plugins/troubleshooting#claude-plugin-validate-reports-errors): 각 유효성 검사 메시지와 해결 방법
