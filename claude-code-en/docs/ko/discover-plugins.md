> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 마켓플레이스를 통해 미리 빌드된 플러그인 발견 및 설치

> 마켓플레이스에서 플러그인을 찾아 설치하여 Claude Code를 새로운 skills, agents 및 기능으로 확장합니다.

플러그인은 Claude Code를 skills, agents, hooks 및 MCP servers로 확장합니다. 플러그인 마켓플레이스는 직접 빌드하지 않고도 이러한 확장 기능을 발견하고 설치할 수 있도록 도와주는 카탈로그입니다.

자신의 마켓플레이스를 만들고 배포하려고 하시나요? [플러그인 마켓플레이스 만들기 및 배포](/docs/ko/plugin-marketplaces)를 참조하세요.

<h2 id="how-marketplaces-work">
  마켓플레이스 작동 방식
</h2>

마켓플레이스는 다른 사람이 만들어 공유한 플러그인의 카탈로그입니다. 마켓플레이스를 사용하는 것은 두 단계의 프로세스입니다:

<Steps>
  <Step title="마켓플레이스 추가">
    이는 카탈로그를 Claude Code에 등록하여 사용 가능한 항목을 검색할 수 있도록 합니다. 아직 플러그인이 설치되지 않습니다.
  </Step>

  <Step title="개별 플러그인 설치">
    카탈로그를 검색하고 원하는 플러그인을 설치합니다.
  </Step>
</Steps>

앱 스토어를 추가하는 것과 같다고 생각하면 됩니다. 스토어를 추가하면 해당 컬렉션을 검색할 수 있지만, 여전히 개별적으로 다운로드할 앱을 선택합니다.

<h2 id="official-anthropic-marketplace">
  공식 Anthropic 마켓플레이스
</h2>

공식 Anthropic 마켓플레이스(`claude-plugins-official`)는 Claude Code를 시작할 때 자동으로 사용 가능합니다. `/plugin`을 실행하고 **Discover** 탭으로 이동하여 사용 가능한 항목을 검색하거나 [claude.com/plugins](https://claude.com/plugins)에서 카탈로그를 확인합니다.

공식 마켓플레이스에서 플러그인을 설치하려면 `/plugin install <name>@claude-plugins-official`을 사용합니다. 예를 들어 GitHub 통합을 설치하려면:

```shell theme={null}
/plugin install github@claude-plugins-official
```

Claude Code가 플러그인을 어떤 마켓플레이스에서도 찾을 수 없다고 보고하면 마켓플레이스가 누락되었거나 오래되었을 수 있습니다. `/plugin marketplace update claude-plugins-official`을 실행하여 새로 고치거나, 이전에 추가하지 않았다면 `/plugin marketplace add anthropics/claude-plugins-official`을 실행합니다. 그런 다음 설치를 다시 시도합니다.

<Note>
  공식 마켓플레이스는 Anthropic에서 큐레이션하며, 포함 여부는 Anthropic의 재량입니다. 앱 내 제출 양식은 플러그인을 [커뮤니티 마켓플레이스](#community-marketplace)에 추가하며, 공식 마켓플레이스에는 추가하지 않습니다. 플러그인을 독립적으로 배포하려면 [자신의 마켓플레이스를 만들고](/docs/ko/plugin-marketplaces) 사용자와 공유하세요.
</Note>

공식 마켓플레이스에는 여러 카테고리의 플러그인이 포함되어 있습니다:

<h3 id="code-intelligence">
  코드 인텔리전스
</h3>

코드 인텔리전스 플러그인은 Claude Code의 기본 제공 LSP 도구를 활성화하여 Claude가 정의로 이동하고, 참조를 찾으며, 편집 직후 타입 오류를 볼 수 있도록 합니다. 이러한 플러그인은 [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) 연결을 구성하며, 이는 VS Code의 코드 인텔리전스를 지원하는 동일한 기술입니다.

이러한 플러그인은 언어 서버 바이너리가 시스템에 설치되어 있어야 합니다. 이미 언어 서버가 설치되어 있으면 프로젝트를 열 때 Claude가 해당 플러그인을 설치하도록 요청할 수 있습니다.

| 언어         | 플러그인                | 필요한 바이너리                     |
| :--------- | :------------------ | :--------------------------- |
| C/C++      | `clangd-lsp`        | `clangd`                     |
| C#         | `csharp-lsp`        | `csharp-ls`                  |
| Go         | `gopls-lsp`         | `gopls`                      |
| Java       | `jdtls-lsp`         | `jdtls`                      |
| Kotlin     | `kotlin-lsp`        | `kotlin-language-server`     |
| Lua        | `lua-lsp`           | `lua-language-server`        |
| PHP        | `php-lsp`           | `intelephense`               |
| Python     | `pyright-lsp`       | `pyright-langserver`         |
| Rust       | `rust-analyzer-lsp` | `rust-analyzer`              |
| Swift      | `swift-lsp`         | `sourcekit-lsp`              |
| TypeScript | `typescript-lsp`    | `typescript-language-server` |

[다른 언어를 위한 자신의 LSP 플러그인을 만들](/docs/ko/plugins-reference#lsp-servers) 수도 있습니다.

<Note>
  플러그인을 설치한 후 `/plugin` Errors 탭에서 `Executable not found in $PATH`를 보면 위 표에서 필요한 바이너리를 설치하세요.
</Note>

<h4 id="what-claude-gains-from-code-intelligence-plugins">
  코드 인텔리전스 플러그인이 Claude에 제공하는 것
</h4>

코드 인텔리전스 플러그인이 설치되고 해당 언어 서버 바이너리를 사용할 수 있으면 Claude는 두 가지 기능을 얻습니다:

* **자동 진단**: Claude가 파일을 편집할 때마다 언어 서버는 변경 사항을 분석하고 오류 및 경고를 자동으로 보고합니다. Claude는 컴파일러나 린터를 실행할 필요 없이 타입 오류, 누락된 import 및 구문 문제를 봅니다. Claude가 오류를 도입하면 같은 턴에서 문제를 알아차리고 수정합니다. 이는 플러그인 설치 이상의 구성이 필요하지 않습니다. "진단 발견됨" 표시기가 나타날 때 **Ctrl+O**를 눌러 진단을 인라인으로 볼 수 있습니다.
* **코드 네비게이션**: Claude는 언어 서버를 사용하여 정의로 이동하고, 참조를 찾으며, 호버 시 타입 정보를 얻고, 기호를 나열하고, 구현을 찾으며, 호출 계층을 추적할 수 있습니다. 이러한 작업은 Claude에게 grep 기반 검색보다 더 정확한 네비게이션을 제공하지만, 가용성은 언어 및 환경에 따라 다를 수 있습니다.

문제가 발생하면 [코드 인텔리전스 문제 해결](#code-intelligence-issues)을 참조하세요.

<h3 id="external-integrations">
  외부 통합
</h3>

이러한 플러그인은 미리 구성된 [MCP servers](/docs/ko/mcp)를 번들로 제공하므로 수동 설정 없이 Claude를 외부 서비스에 연결할 수 있습니다:

* **소스 제어**: `github`, `gitlab`
* **프로젝트 관리**: `atlassian` (Jira/Confluence), `asana`, `linear`, `notion`
* **디자인**: `figma`
* **인프라**: `vercel`, `firebase`, `supabase`
* **커뮤니케이션**: `slack`
* **모니터링**: `sentry`

<h3 id="automatic-security-review">
  자동 보안 검토
</h3>

`security-guidance` 플러그인은 Claude가 만드는 각 변경 사항을 일반적인 취약점에 대해 검토하고 Claude에게 같은 세션에서 발견한 내용을 수정하도록 지시합니다. [Claude가 코드를 작성할 때 보안 문제 포착](/docs/ko/security-guidance)에서 검사하는 내용과 프로젝트별 규칙을 추가하는 방법을 참조하세요.

<h3 id="development-workflows">
  개발 워크플로우
</h3>

일반적인 개발 작업을 위한 skills 및 agents를 추가하는 플러그인:

* **commit-commands**: commit, push 및 PR 생성을 포함한 Git commit 워크플로우
* **pr-review-toolkit**: pull request 검토를 위한 특화된 agents
* **agent-sdk-dev**: Claude Agent SDK로 빌드하기 위한 도구
* **plugin-dev**: 자신의 플러그인을 만들기 위한 도구 모음

<h3 id="output-styles">
  출력 스타일
</h3>

Claude가 응답하는 방식을 사용자 정의합니다:

* **explanatory-output-style**: 구현 선택에 대한 교육적 통찰력
* **learning-output-style**: 기술 습득을 위한 대화형 학습 모드

<h2 id="community-marketplace">
  커뮤니티 마켓플레이스
</h2>

[`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community)의 커뮤니티 마켓플레이스는 Anthropic의 자동화된 검증 및 안전 심사를 통과한 타사 플러그인을 호스팅합니다. 각 플러그인은 카탈로그의 특정 커밋 SHA에 고정됩니다. 공식 마켓플레이스와 달리 수동으로 추가해야 합니다:

```shell theme={null}
/plugin marketplace add anthropics/claude-plugins-community
```

그런 다음 `claude-community` 마켓플레이스 이름을 사용하여 플러그인을 설치합니다:

```shell theme={null}
/plugin install <plugin-name>@claude-community
```

자신의 플러그인을 커뮤니티 마켓플레이스에 제출하려면 플러그인 생성 가이드의 [Submit your plugin to the community marketplace](/docs/ko/plugins#submit-your-plugin-to-the-community-marketplace)를 참조하세요.

<h2 id="try-it-add-the-demo-marketplace">
  시도해보기: 데모 마켓플레이스 추가
</h2>

Anthropic은 플러그인 시스템으로 가능한 것들을 보여주는 예제 플러그인이 포함된 [데모 플러그인 마켓플레이스](https://github.com/anthropics/claude-code/tree/main/plugins)(`claude-code-plugins`)를 유지 관리합니다. 공식 마켓플레이스와 달리 이 마켓플레이스는 수동으로 추가해야 합니다.

<Steps>
  <Step title="마켓플레이스 추가">
    Claude Code 내에서 `anthropics/claude-code` 마켓플레이스에 대해 `plugin marketplace add` 명령어를 실행합니다:

    ```shell theme={null}
    /plugin marketplace add anthropics/claude-code
    ```

    이는 마켓플레이스 카탈로그를 다운로드하고 해당 플러그인을 사용 가능하게 만듭니다.
  </Step>

  <Step title="사용 가능한 플러그인 찾아보기">
    `/plugin`을 실행하여 플러그인 관리자를 엽니다. 이는 **Tab** (또는 뒤로 가려면 **Shift+Tab**)을 사용하여 순환할 수 있는 4개의 탭이 있는 탭 인터페이스를 엽니다:

    * **Discover**: 모든 마켓플레이스에서 사용 가능한 플러그인 찾아보기
    * **Installed**: 설치된 플러그인 보기 및 관리
    * **Marketplaces**: 추가된 마켓플레이스 추가, 제거 또는 업데이트
    * **Errors**: 플러그인 로딩 오류 보기

    방금 추가한 마켓플레이스의 플러그인을 보려면 **Discover** 탭으로 이동합니다. 관리자가 [`pluginSuggestionMarketplaces`](/docs/ko/settings#available-settings) 관리 설정을 통해 마켓플레이스를 허용 목록에 추가한 경우, 현재 작업 디렉토리와 관련이 있는 것으로 표시된 플러그인은 **suggested for this directory** 레이블과 함께 맨 위에 고정됩니다.
  </Step>

  <Step title="플러그인 설치">
    플러그인을 선택하여 세부 정보를 봅니다. 세부 정보 창에는 플러그인에 포함된 내용과 비용이 표시됩니다:

    * 플러그인이 매 턴마다 [컨텍스트 윈도우](/docs/ko/features-overview#understand-context-costs)에 추가할 토큰 수를 볼 수 있는 **Context cost** 예상치 (Claude Code v2.1.143 이상)
    * 플러그인의 **Last updated** 날짜 (v2.1.144 이상)
    * 플러그인의 명령어, 에이전트, 스킬, 훅 및 MCP와 LSP 서버를 나열하는 **Will install** 섹션으로, 설치 전에 정확히 무엇이 추가되는지 검토할 수 있습니다 (v2.1.145 이상)

    설치 범위를 선택합니다:

    * **User scope**: 모든 프로젝트에서 자신을 위해 설치
    * **Project scope**: 이 저장소의 모든 협력자를 위해 설치
    * **Local scope**: 이 저장소에서만 자신을 위해 설치

    예를 들어 **commit-commands** (git 워크플로우 스킬을 추가하는 플러그인)를 선택하고 사용자 범위에 설치합니다.

    명령줄에서 직접 설치할 수도 있습니다:

    ```shell theme={null}
    /plugin install commit-commands@claude-code-plugins
    ```

    범위에 대해 자세히 알아보려면 [Configuration scopes](/docs/ko/settings#configuration-scopes)를 참조하세요.
  </Step>

  <Step title="새 플러그인 사용">
    설치 후 `/reload-plugins`를 실행하여 플러그인을 활성화합니다. 플러그인 스킬은 플러그인 이름으로 네임스페이스되므로 **commit-commands**는 `/commit-commands:commit`과 같은 스킬을 제공합니다.

    파일을 변경하고 다음을 실행하여 시도해봅니다:

    ```shell theme={null}
    /commit-commands:commit
    ```

    이는 변경 사항을 스테이징하고, 커밋 메시지를 생성하며, 커밋을 만듭니다.

    각 플러그인은 다르게 작동합니다. **Discover** 탭에서 플러그인의 세부 정보를 확인하여 제공하는 명령어와 스킬을 보거나, 사용 지침을 위해 해당 홈페이지를 방문하세요.
  </Step>
</Steps>

이 가이드의 나머지 부분에서는 마켓플레이스를 추가하고, 플러그인을 설치하며, 구성을 관리하는 모든 방법을 다룹니다.

<h2 id="add-marketplaces">
  마켓플레이스 추가
</h2>

`/plugin marketplace add` 명령을 사용하여 다양한 소스에서 마켓플레이스를 추가합니다.

<Tip>
  **단축키**: `/plugin marketplace` 대신 `/plugin market`을 사용할 수 있으며, `remove` 대신 `rm`을 사용할 수 있습니다.
</Tip>

* **GitHub 저장소**: `owner/repo` 형식(예: `anthropics/claude-code`)
* **Git URL**: 모든 git 저장소 URL(GitLab, Bitbucket, 자체 호스팅)
* **로컬 경로**: 디렉터리 또는 `marketplace.json` 파일의 직접 경로
* **원격 URL**: 호스팅된 `marketplace.json` 파일의 직접 URL

<h3 id="add-from-github">
  GitHub에서 추가
</h3>

`.claude-plugin/marketplace.json` 파일을 포함하는 GitHub 저장소를 `owner/repo` 형식으로 추가합니다. 여기서 `owner`는 GitHub 사용자명 또는 조직이고 `repo`는 저장소 이름입니다.

예를 들어, `anthropics/claude-code`는 `anthropics`가 소유한 `claude-code` 저장소를 나타냅니다:

```shell theme={null}
/plugin marketplace add anthropics/claude-code
```

<h3 id="add-from-other-git-hosts">
  다른 Git 호스트에서 추가
</h3>

전체 URL을 제공하여 모든 git 저장소를 추가합니다. 이는 GitLab, Bitbucket 및 자체 호스팅 서버를 포함한 모든 Git 호스트에서 작동합니다. `.git` 접미사를 포함하여 Claude Code가 저장소를 복제하도록 하고 URL을 호스팅된 `marketplace.json` 파일의 직접 링크로 처리하지 않도록 합니다.

`https://` 접두사도 포함합니다. Claude Code v2.1.196 이상은 `gitlab.com/company/plugins.git`과 같이 접두사 없이 입력된 호스트를 잘못된 GitHub `owner/repo` 단축형으로 거부하며, 오류 메시지에서 접두사를 추가하도록 알려줍니다. 이전 버전은 이를 GitHub 저장소 경로로 잘못 읽고 복제 시간에 실패합니다.

HTTPS 사용:

```shell theme={null}
/plugin marketplace add https://gitlab.com/company/plugins.git
```

SSH 사용:

```shell theme={null}
/plugin marketplace add git@gitlab.com:company/plugins.git
```

특정 분기 또는 태그를 추가하려면 `#` 뒤에 ref를 추가합니다:

```shell theme={null}
/plugin marketplace add https://gitlab.com/company/plugins.git#v1.0.0
```

<h3 id="add-from-local-paths">
  로컬 경로에서 추가
</h3>

`.claude-plugin/marketplace.json` 파일을 포함하는 로컬 디렉터리를 추가합니다:

```shell theme={null}
/plugin marketplace add ./my-marketplace
```

`marketplace.json` 파일의 직접 경로를 추가할 수도 있습니다:

```shell theme={null}
/plugin marketplace add ./path/to/marketplace.json
```

<h3 id="add-from-remote-urls">
  원격 URL에서 추가
</h3>

URL을 통해 원격 `marketplace.json` 파일을 추가합니다:

```shell theme={null}
/plugin marketplace add https://example.com/marketplace.json
```

<Note>
  URL 기반 마켓플레이스는 Git 기반 마켓플레이스에 비해 몇 가지 제한 사항이 있습니다. 플러그인을 설치할 때 "경로를 찾을 수 없음" 오류가 발생하면 [문제 해결](/docs/ko/plugin-marketplaces#plugins-with-relative-paths-fail-in-url-based-marketplaces)을 참조하세요.
</Note>

<h2 id="install-plugins">
  플러그인 설치
</h2>

마켓플레이스를 추가한 후 플러그인을 직접 설치할 수 있습니다:

```shell theme={null}
/plugin install plugin-name@marketplace-name
```

이 명령어는 플러그인의 세부 정보를 열며, 여기서 [설치 범위](/docs/ko/settings#configuration-scopes)를 선택합니다. `/plugin`을 실행하고 **Discover** 탭으로 이동한 후 플러그인에서 **Enter**를 누를 때도 동일한 선택지가 표시됩니다:

* **사용자 범위**(기본값): 모든 프로젝트에서 자신을 위해 설치
* **프로젝트 범위**: 이 저장소의 모든 협력자를 위해 설치(`.claude/settings.json`에 추가)
* **로컬 범위**: 이 저장소에서만 자신을 위해 설치(협력자와 공유되지 않음)

대화형 단계 없이 설치하려면 [`claude plugin install`](/docs/ko/plugins-reference#plugin-install) 셸 명령어를 사용하세요. 이 명령어는 `--scope`를 전달하지 않으면 사용자 범위로 설치됩니다.

**관리됨** 범위의 플러그인도 볼 수 있습니다. 이러한 플러그인은 관리자가 [관리 설정](/docs/ko/settings#settings-files)을 통해 설치하며 수정할 수 없습니다.

<Warning>
  플러그인을 설치하기 전에 신뢰할 수 있는지 확인하세요. Anthropic은 플러그인에 포함된 MCP 서버, 파일 또는 기타 소프트웨어를 제어하지 않으며 의도한 대로 작동하는지 확인할 수 없습니다. 자세한 내용은 각 플러그인의 홈페이지를 확인하세요.
</Warning>

<h2 id="manage-installed-plugins">
  설치된 플러그인 관리
</h2>

`/plugin`을 실행하고 **Installed** 탭으로 이동하여 플러그인을 보고, 활성화하고, 비활성화하거나, 제거합니다. 목록은 범위별로 그룹화되고 문제가 먼저 표시되도록 정렬됩니다: 로드 오류 또는 해결되지 않은 종속성이 있는 플러그인이 맨 위에 나타나고, 그 다음 즐겨찾기가 나타나며, 비활성화된 플러그인은 맨 아래의 축소된 헤더 뒤에 접혀 있습니다.

목록에서 다음을 수행할 수 있습니다:

* `f`를 눌러 선택한 플러그인을 즐겨찾기에 추가하거나 제거
* 입력하여 플러그인 이름 또는 설명으로 필터링
* Enter를 눌러 플러그인의 세부 정보 보기를 열고 활성화, 비활성화 또는 제거

프로젝트의 `.claude/settings.json`이 활성화하는 플러그인을 제거하면 어느 범위를 의도했는지 묻습니다: 자신만 비활성화하면 `.claude/settings.local.json`에 재정의를 작성하고 프로젝트에 플러그인을 설치된 상태로 두거나, 모두를 위해 제거하면 공유 `.claude/settings.json`에서 제거합니다. Claude Code v2.1.203 이상이 필요합니다. v2.1.203 이전에는 대화 상자에서 로컬 비활성화만 제공했습니다.

세부 정보 보기는 플러그인이 제공하는 구성 요소를 표시합니다: commands, skills, agents, hooks, MCP servers 및 LSP servers. 동일한 인벤토리는 `claude plugin details` 명령어로 명령줄에서도 사용할 수 있습니다.

**Installed** 탭은 또한 마켓플레이스에서 직접 설치했지만 최소 2주 동안 그리고 최소 10개 세션에 걸쳐 사용하지 않은 플러그인을 **Not used recently** 헤더 아래에 수집합니다. 세부 정보 보기는 각 플러그인에 대한 **Last used** 줄을 표시합니다. 이를 사용하여 더 이상 사용하지 않지만 여전히 시작 및 컨텍스트 비용을 추가하는 플러그인을 찾은 다음 비활성화하거나 제거합니다. Claude Code v2.1.187 이상이 필요합니다.

두 가지 종류의 플러그인은 사용하지 않는 것으로 나열되지 않습니다:

* 조직에서 관리하거나 `--plugin-dir`로 로드하는 플러그인
* theme, output style, monitor 또는 workflow를 제공하는 플러그인. 이들은 추적할 호출 없이 값을 제공하기 때문입니다

**Not used recently** 헤더와 **Last used** 줄은 조직이 [`strictKnownMarketplaces`](/docs/ko/settings#strictknownmarketplaces)로 마켓플레이스를 제한할 때 모두 숨겨집니다.

플러그인의 [language server](/docs/ko/plugins#add-lsp-servers-to-your-plugin)는 진단을 제공하거나 코드 네비게이션 요청에 응답할 때 사용된 것으로 계산되므로, 서버가 세션에서 활성화된 LSP 플러그인은 사용하지 않는 것으로 나열되지 않습니다. v2.1.203 이전에는 language server 활동을 사용으로 계산할 수 없었으므로, LSP server를 제공하는 플러그인은 theme 및 output style 플러그인과 동일한 방식으로 그룹에서 제외되었습니다.

language server 활동을 계산하는 버전의 첫 번째 세션은 또한 아직 사용 기록을 기록하지 않은 각 LSP 플러그인의 사용 기록을 재설정하므로, Claude Code는 서버 활동이 추적되기 전에 기록된 데이터를 기반으로 이전에 설치한 플러그인을 사용하지 않는 것으로 판단하지 않습니다. v2.1.206 이전에는 해당 첫 번째 세션이 활발히 사용 중인 LSP 플러그인을 **Not used recently** 아래에 나열하고 검토를 제안할 수 있었습니다.

종속성을 선언하는 플러그인을 설치하면 설치 출력에 함께 자동 설치된 종속성이 나열됩니다.

직접 명령어로 플러그인을 관리할 수도 있습니다.

메뉴를 열지 않고 설치된 플러그인을 나열합니다:

```shell theme={null}
/plugin list
```

`--enabled` 또는 `--disabled`를 전달하여 해당 상태의 플러그인만 표시합니다.

플러그인을 제거하지 않고 비활성화합니다:

```shell theme={null}
/plugin disable plugin-name@marketplace-name
```

비활성화된 플러그인을 다시 활성화합니다:

```shell theme={null}
/plugin enable plugin-name@marketplace-name
```

이 식별자에서 `plugin-name`은 [마켓플레이스 항목](/docs/ko/plugin-marketplaces#plugin-entries)의 플러그인 `name`이며, 플러그인 자체의 `plugin.json`의 `name`과 다를 수 있습니다.

Claude Code v2.1.195부터 `/plugin` 인터페이스의 **Enable** 및 **Disable**은 두 이름이 다른 플러그인에 대해 작동하며, `/plugin enable` 및 `/plugin disable`은 두 이름 중 하나를 허용합니다. 이전 버전에서 이러한 플러그인을 비활성화하면 Claude Code는 `already disabled`를 보고하고 활성화된 상태로 둡니다.

플러그인을 완전히 제거합니다:

```shell theme={null}
/plugin uninstall plugin-name@marketplace-name
```

`--scope` 옵션을 사용하면 CLI 명령어로 특정 범위를 대상으로 할 수 있습니다:

```shell theme={null}
claude plugin install formatter@your-org --scope project
claude plugin uninstall formatter@your-org --scope project
```

<h3 id="apply-plugin-changes-without-restarting">
  재시작 없이 플러그인 변경 사항 적용
</h3>

세션 중에 플러그인을 설치, 활성화 또는 비활성화할 때 `/reload-plugins`를 실행하여 재시작 없이 모든 변경 사항을 선택합니다:

```shell theme={null}
/reload-plugins
```

Claude Code는 모든 활성 플러그인을 다시 로드하고 플러그인, skills, agents, hooks, 플러그인 MCP servers 및 플러그인 LSP servers의 개수를 표시합니다.

재로드는 다음 요청에서 토큰 비용이 발생합니다: 새로 로드된 구성 요소는 대화에 추가된 콘텐츠에서 자신을 알리고, 기존 기록은 여전히 프롬프트 캐시에서 읽습니다. MCP servers를 제공하는 플러그인은 [tool search](/docs/ko/mcp#scale-with-mcp-tool-search)에 의해 도구가 지연되지 않을 때 더 많은 비용이 발생합니다: 변경으로 인해 캐시가 무효화되고 다음 요청이 전체 대화를 다시 읽습니다. 이 경우 `/reload-plugins`는 경고를 표시하고 재로드를 적용하지 않습니다. `--force`를 전달하여 어쨌든 적용합니다. 자세한 내용은 [플러그인 활성화 또는 비활성화](/docs/ko/prompt-caching#enabling-or-disabling-a-plugin)를 참조하세요.

<h2 id="manage-marketplaces">
  마켓플레이스 관리
</h2>

대화형 `/plugin` 인터페이스 또는 CLI 명령어를 통해 마켓플레이스를 관리할 수 있습니다.

<h3 id="use-the-interactive-interface">
  대화형 인터페이스 사용
</h3>

`/plugin`을 실행하고 **Marketplaces** 탭으로 이동하여:

* 소스 및 상태와 함께 추가된 모든 마켓플레이스 보기
* 새 마켓플레이스 추가
* 마켓플레이스 목록을 업데이트하여 최신 플러그인 가져오기
* 더 이상 필요하지 않은 마켓플레이스 제거

<h3 id="use-cli-commands">
  CLI 명령어 사용
</h3>

직접 명령어로 마켓플레이스를 관리할 수도 있습니다.

구성된 모든 마켓플레이스 나열:

```shell theme={null}
/plugin marketplace list
```

마켓플레이스에서 플러그인 목록 새로 고침:

```shell theme={null}
/plugin marketplace update marketplace-name
```

마켓플레이스 제거:

```shell theme={null}
/plugin marketplace remove marketplace-name
```

<Warning>
  마켓플레이스를 제거하면 해당 마켓플레이스에서 설치한 모든 플러그인이 제거됩니다.
</Warning>

<h3 id="configure-auto-updates">
  자동 업데이트 구성
</h3>

Claude Code는 시작 후 백그라운드에서 마켓플레이스 및 설치된 플러그인을 자동으로 업데이트할 수 있습니다. 마켓플레이스에 대해 자동 업데이트가 활성화되면 Claude Code는 마켓플레이스 데이터를 새로 고치고 설치된 플러그인을 디스크의 최신 버전으로 업데이트합니다.

Claude Code는 세션 시작 후 마켓플레이스 및 플러그인 업데이트를 확인하며, 최대 10분의 무작위 지연이 있으므로 실행 중인 세션은 시작 시 로드된 버전을 계속 사용합니다. 플러그인이 업데이트된 경우 `/reload-plugins`를 실행하도록 요청하는 알림이 표시되거나 다음 시작 시 새 버전이 로드됩니다.

UI를 통해 개별 마켓플레이스에 대한 자동 업데이트를 전환합니다:

1. `/plugin`을 실행하여 플러그인 관리자 열기
2. **Marketplaces** 선택
3. 목록에서 마켓플레이스 선택
4. **자동 업데이트 활성화** 또는 **자동 업데이트 비활성화** 선택

공식 Anthropic 마켓플레이스는 기본적으로 자동 업데이트가 활성화되어 있습니다. 타사 및 로컬 개발 마켓플레이스는 기본적으로 자동 업데이트가 비활성화되어 있습니다.

관리자는 관리되는 설정에서 각 [`extraKnownMarketplaces`](/docs/ko/settings#extraknownmarketplaces) 항목에 `"autoUpdate": true`를 설정하여 각 사용자가 전환하도록 요구하지 않고 조직 마켓플레이스에 대한 자동 업데이트를 활성화할 수 있습니다.

Claude Code 및 모든 플러그인에 대해 모든 자동 업데이트를 완전히 비활성화하려면 `DISABLE_AUTOUPDATER` 환경 변수를 설정합니다. 자세한 내용은 [자동 업데이트](/docs/ko/setup#auto-updates)를 참조하세요.

Claude Code 자동 업데이트를 비활성화하면서 플러그인 자동 업데이트를 활성화된 상태로 유지하려면 `DISABLE_AUTOUPDATER`와 함께 `FORCE_AUTOUPDATE_PLUGINS=1`을 설정합니다:

```bash theme={null}
export DISABLE_AUTOUPDATER=1
export FORCE_AUTOUPDATE_PLUGINS=1
```

Claude Code 업데이트를 수동으로 관리하지만 여전히 자동 플러그인 업데이트를 받으려는 경우에 유용합니다.

<h2 id="configure-team-marketplaces">
  팀 마켓플레이스 구성
</h2>

팀 관리자는 `.claude/settings.json`에 마켓플레이스 구성을 추가하여 프로젝트에 대한 자동 마켓플레이스 설치를 설정할 수 있습니다. 팀 멤버가 저장소 폴더를 신뢰하면 Claude Code는 이러한 마켓플레이스 및 플러그인을 설치하도록 요청합니다.

Claude Code v2.1.195부터 이 설치 단계는 플러그인을 로드하는 모든 경로에 적용됩니다. 프로젝트의 `.claude/settings.json`만으로 활성화되고 GitHub 저장소 또는 npm 패키지와 같은 외부 소스에서 제공되는 플러그인은 팀 멤버가 설치할 때까지 로드되지 않습니다. 그때까지 Claude Code는 플러그인이 설치되지 않았다고 보고하고 실행할 `claude plugin install` 명령을 표시합니다.

프로젝트의 `.claude/settings.json`에 `extraKnownMarketplaces`를 추가합니다:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "my-team-tools": {
      "source": {
        "source": "github",
        "repo": "your-org/claude-plugins"
      }
    }
  }
}
```

`extraKnownMarketplaces` 및 `enabledPlugins`를 포함한 전체 구성 옵션은 [플러그인 설정](/docs/ko/settings#plugin-settings)을 참조하세요.

<h2 id="security">
  보안
</h2>

플러그인 및 마켓플레이스는 사용자 권한으로 머신에서 임의의 코드를 실행할 수 있는 매우 신뢰할 수 있는 구성 요소입니다. 신뢰할 수 있는 소스에서만 플러그인을 설치하고 마켓플레이스를 추가합니다. 조직은 [관리되는 마켓플레이스 제한](/docs/ko/plugin-marketplaces#managed-marketplace-restrictions)을 사용하여 사용자가 추가할 수 있는 마켓플레이스를 제한할 수 있습니다.

<h2 id="troubleshooting">
  문제 해결
</h2>

<h3 id="/plugin-command-not-recognized">
  /plugin 명령어를 인식하지 못함
</h3>

"알 수 없는 명령어" 또는 `/plugin` 명령어가 나타나지 않으면:

1. **버전 확인**: `claude --version`을 실행하여 설치된 항목을 확인합니다.
2. **Claude Code 업데이트**:
   * **Homebrew**: `brew upgrade claude-code`, 또는 해당 cask를 설치한 경우 `brew upgrade claude-code@latest`
   * **npm**: `npm install -g @anthropic-ai/claude-code@latest`
   * **네이티브 설치 프로그램**: [설정](/docs/ko/setup)에서 설치 명령어를 다시 실행합니다.
3. **Claude Code 재시작**: 업데이트 후 터미널을 재시작하고 `claude`를 다시 실행합니다.

<h3 id="common-issues">
  일반적인 문제
</h3>

* **마켓플레이스가 로드되지 않음**: URL에 액세스할 수 있고 `.claude-plugin/marketplace.json`이 경로에 있는지 확인합니다.
* **플러그인 설치 실패**: 플러그인 소스 URL에 액세스할 수 있고 저장소가 공개되어 있거나 액세스 권한이 있는지 확인합니다.
* **설치 후 파일을 찾을 수 없음**: 플러그인은 캐시에 복사되므로 플러그인 디렉토리 외부의 파일을 참조하는 경로는 작동하지 않습니다.
* **플러그인 skills가 나타나지 않음**: `rm -rf ~/.claude/plugins/cache`로 캐시를 지우고, Claude Code를 재시작한 후 플러그인을 다시 설치합니다.

자세한 문제 해결 및 솔루션은 마켓플레이스 가이드의 [문제 해결](/docs/ko/plugin-marketplaces#troubleshooting)을 참조하세요. 디버깅 도구는 [디버깅 및 개발 도구](/docs/ko/plugins-reference#debugging-and-development-tools)를 참조하세요.

<h3 id="code-intelligence-issues">
  코드 인텔리전스 문제
</h3>

* **언어 서버가 시작되지 않음**: 바이너리가 설치되어 있고 `$PATH`에서 사용 가능한지 확인합니다. `/plugin` Errors 탭에서 세부 정보를 확인합니다.
* **높은 메모리 사용량**: `rust-analyzer` 및 `pyright`와 같은 언어 서버는 대규모 프로젝트에서 상당한 메모리를 소비할 수 있습니다. 메모리 문제가 발생하면 `/plugin disable <plugin-name>`으로 플러그인을 비활성화하고 대신 Claude의 기본 제공 검색 도구를 사용합니다.
* **모노레포에서 거짓 양성 진단**: 작업 공간이 올바르게 구성되지 않으면 언어 서버가 내부 패키지에 대해 해결되지 않은 import 오류를 보고할 수 있습니다. 이는 Claude의 코드 편집 능력에 영향을 주지 않습니다.

<h2 id="next-steps">
  다음 단계
</h2>

* **자신의 플러그인 빌드**: [플러그인](/docs/ko/plugins)을 참조하여 skills, agents 및 hooks를 만듭니다.
* **마켓플레이스 만들기**: [플러그인 마켓플레이스 만들기](/docs/ko/plugin-marketplaces)를 참조하여 팀 또는 커뮤니티에 플러그인을 배포합니다.
* **기술 참조**: [플러그인 참조](/docs/ko/plugins-reference)를 참조하여 완전한 사양을 확인합니다.
