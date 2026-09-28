> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 플러그인 개요

> Claude Code 플러그인이 무엇인지, 독립형 스킬이나 MCP 서버 대신 플러그인이 필요한 경우, 플러그인을 설치하거나 만드는 방법을 알아봅니다.

Claude Code 플러그인은 Claude Code가 하나의 단위로 설치하고 로드하는 스킬, 에이전트, 훅, MCP 서버 또는 기타 구성 요소의 디렉토리입니다. 대부분의 플러그인은 마켓플레이스에서 제공되며, 마켓플레이스는 플러그인을 나열하고 각 플러그인을 가져올 위치를 표시하는 카탈로그입니다. 누군가가 제공한 폴더에서 플러그인을 로드하거나 [자신만의 플러그인을 만들](/docs/ko/plugins/create) 수도 있습니다.

<Note>
  claude.ai 채팅이나 Cowork를 사용하고 Claude Code를 사용하지 않는 경우 [claude.ai 및 Cowork의 플러그인](https://claude.com/docs/plugins/overview)을 참조하세요.
</Note>

지금 플러그인을 시도하려면 Claude Code 터미널 세션에서 `/plugin`을 실행하고 **Discover** 탭에서 플러그인을 설치합니다. **Discover** 탭에는 Anthropic의 공식 마켓플레이스와 추가한 마켓플레이스의 플러그인이 나열됩니다. 여기서:

* [플러그인 설치 및 관리](/docs/ko/plugins/install): 전체 설치 단계, 범위 및 기타 표면
* [플러그인 만들기](/docs/ko/plugins/create): 자신만의 플러그인 만들기
* [플러그인이 필요한지 결정](#decide-whether-you-need-a-plugin): 플러그인이 원하는 작업에 적합한 도구인지 여부

<h2 id="understand-what-a-plugin-is">
  플러그인이 무엇인지 이해하기
</h2>

플러그인은 일반적으로 매니페스트가 있는 구성 요소의 디렉토리입니다. 매니페스트는 `.claude-plugin/plugin.json`의 JSON 파일로, 플러그인에 이름을 지정하고 버전, 설명 및 기타 [메타데이터](/docs/ko/plugins/manifest-reference)를 추가할 수 있습니다. 구성 요소는 플러그인이 Claude Code에 추가하는 것입니다. 예를 들어:

* [**스킬**](/docs/ko/plugins/components#skills): Claude가 관련성이 있을 때 로드하는 `SKILL.md` 지침이며, 명령으로도 실행할 수 있습니다.
* [**에이전트**](/docs/ko/plugins/components#agents): Claude가 위임할 수 있는 서브에이전트 정의
* [**훅**](/docs/ko/plugins/components#hooks): Claude Code가 편집 후와 같은 수명 주기의 특정 지점에서 실행하는 명령
* [**MCP 서버**](/docs/ko/plugins/components#mcp-servers): 플러그인이 활성화되어 있는 동안 Claude Code가 연결하는 도구 서버

이 다이어그램은 각 구성 요소 유형 중 하나씩 보유한 `my-plugin`이라는 플러그인과 플러그인이 로드되면 각 파일에서 얻는 것을 보여줍니다.

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugin-directory.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=f623b64e82713b830e48174f0a922888" className="dark:hidden" alt="Diagram in two columns joined by five straight arrows. Left, the directory of a plugin named my-plugin, holding a manifest at .claude-plugin/plugin.json, skills/review/SKILL.md, agents/reviewer.md, hooks/hooks.json, .mcp.json, and other components. Right, what each file gives you in your session: the manifest sets the plugin name, my-plugin; the skill runs as /my-plugin:review; the agent file is a subagent Claude can delegate to; the hooks file holds hooks that run on lifecycle events; and .mcp.json adds an MCP server that gives Claude tools." width="760" height="336" data-path="images/plugin-directory.svg" />

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugin-directory-dark.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=17ee2bd45b63154fcc148ae1d1f736d8" className="hidden dark:block" alt="Diagram in two columns joined by five straight arrows. Left, the directory of a plugin named my-plugin, holding a manifest at .claude-plugin/plugin.json, skills/review/SKILL.md, agents/reviewer.md, hooks/hooks.json, .mcp.json, and other components. Right, what each file gives you in your session: the manifest sets the plugin name, my-plugin; the skill runs as /my-plugin:review; the agent file is a subagent Claude can delegate to; the hooks file holds hooks that run on lifecycle events; and .mcp.json adds an MCP server that gives Claude tools." width="760" height="336" data-path="images/plugin-directory-dark.svg" />

플러그인이 보유할 수 있는 모든 구성 요소 유형과 각각의 예시는 [플러그인 구성 요소](/docs/ko/plugins/components)를 참조하세요. 플러그인의 디렉토리에서 각 부분이 위치한 곳을 확인하려면 해당 페이지의 [플러그인 탐색기](/docs/ko/plugins/components#explore-the-plugin-directory)를 사용하세요.

<h3 id="decide-whether-you-need-a-plugin">
  플러그인이 필요한지 결정하기
</h3>

스킬, 서브에이전트, 훅 및 MCP 서버는 모두 플러그인 없이도 독립적으로 작동합니다. 예를 들어 `~/.claude/skills/`에 저장한 스킬은 컴퓨터의 모든 프로젝트에서 사용할 수 있습니다. 독립적으로 설정하려면 [스킬](/docs/ko/skills), [서브에이전트](/docs/ko/sub-agents), [훅](/docs/ko/hooks-guide) 또는 [MCP](/docs/ko/mcp)를 참조하세요.

여러 스킬, 서브에이전트, 훅 또는 MCP 서버를 하나의 단위로 패키징하려면 플러그인을 사용합니다. 하나를 설치하여 다른 사람이 만든 설정을 한 명령으로 얻고 마켓플레이스에서 업데이트를 받습니다. 자신의 설정을 팀원에게 제공하거나, 많은 프로젝트에 설치하거나, 버전이 지정된 릴리스를 게시하려면 플러그인을 만듭니다.

<h3 id="what-an-enabled-plugin-adds-to-your-sessions">
  활성화된 플러그인이 세션에 추가하는 것
</h3>

활성화된 플러그인은 사용하는 세션뿐만 아니라 모든 세션의 일부입니다. 이는 플러그인을 설치하기 전에 알아야 할 몇 가지 결과를 초래합니다:

* **컨텍스트 및 사용**: [Claude가 자체적으로 호출할 수 있는](/docs/ko/skills#control-who-invokes-a-skill) 각 스킬, 에이전트 및 명령에 대해 이름과 설명이 모든 턴에서 Claude의 컨텍스트에 있으므로 Claude는 이것이 존재한다는 것을 알 수 있습니다. 이러한 토큰은 사용량에 포함되며 플러그인에서 아무것도 실행되지 않는 세션에서도 [컨텍스트 윈도우](/docs/ko/context-window)에서 공간을 남깁니다. 스킬이나 에이전트의 전체 텍스트는 사용될 때만 로드됩니다. 플러그인의 MCP 서버가 턴당 추가하는 것은 [MCP 도구 검색](/docs/ko/mcp#scale-with-mcp-tool-search)을 따릅니다.
* **프로세스**: 플러그인이 정의하는 MCP 서버는 플러그인이 활성화된 각 세션과 함께 실행되며, 훅은 해당 이벤트에서 실행됩니다.
* **권한**: 플러그인이 실행하는 것은 사용자로서 실행됩니다. 먼저 검토할 사항은 [플러그인 보안 및 신뢰](/docs/ko/plugins/security)를 참조하세요.

각 단계에서 플러그인의 풋프린트를 확인할 수 있습니다:

* **설치 전**: `/plugin`의 **마켓플레이스** 탭에서 플러그인을 엽니다. Anthropic의 공식 마켓플레이스의 플러그인은 **컨텍스트 비용** 추정치를 표시합니다.
* **설치 후**: [플러그인 비용 측정](/docs/ko/plugins/measure#measure-what-a-plugin-costs)은 플러그인의 풋프린트를 읽는 방법을 보여주며, **설치됨** 탭의 **최근에 사용하지 않음** 그룹은 비활성화할 수 있는 플러그인을 나열합니다.
* **제거하지 않고 중지하려면**: `/plugin`으로 플러그인을 비활성화하거나 셸에서 `claude plugin disable`을 사용합니다. [설치된 플러그인 관리](/docs/ko/plugins/install#manage-installed-plugins)를 참조하세요.

<h2 id="get-plugins-from-a-marketplace">
  마켓플레이스에서 플러그인 가져오기
</h2>

마켓플레이스는 플러그인을 나열하고 각 플러그인을 가져올 위치를 표시하는 `.claude-plugin/marketplace.json` 파일이 있는 저장소 또는 디렉토리입니다. 호스팅된 스토어가 아니라 카탈로그입니다. 마켓플레이스를 한 번 추가한 다음 `commit-commands@claude-plugins-official`과 같이 이름으로 플러그인을 설치합니다.

<Note>
  플러그인 마켓플레이스는 [Claude Marketplace](https://claude.com/marketplace)가 아닙니다. Claude Marketplace는 claude.com/marketplace의 웹사이트로, 플러그인, 커넥터, 파트너 제품 및 서비스 파트너를 검색할 수 있습니다. `/plugin marketplace add`로 추가하는 마켓플레이스가 아닙니다.
</Note>

Claude Code는 [관리 정책](/docs/ko/plugins/org#allow-the-official-marketplace-and-your-own)이 차단하지 않는 한 대화형 터미널 세션을 처음 시작할 때 Anthropic의 공식 마켓플레이스를 추가합니다. Claude Code는 Anthropic의 커뮤니티 및 데모 마켓플레이스를 포함하여 자체적으로 다른 마켓플레이스를 추가하지 않습니다. 세 가지 Anthropic 마켓플레이스를 구분하려면 [Anthropic의 마켓플레이스](/docs/ko/plugins/anthropic-marketplaces)를 읽으세요. 공식 마켓플레이스가 나열하는 것을 보려면 세션에서 `/plugin`의 **Discover** 탭을 열거나 [Claude Marketplace](https://claude.com/marketplace/plugins)를 검색하세요.

이 다이어그램은 마켓플레이스에서 세션까지의 경로를 보여줍니다. 마켓플레이스는 플러그인을 나열하고, 플러그인을 설치하면 Claude Code가 구성 요소를 로드합니다.

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugins-model.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=4196344954b7c2e27fc0bd6a9a1113a1" className="dark:hidden" alt="Diagram of the marketplace path in three boxes, left to right. A marketplace, a catalog of plugins, lists a plugin. The plugin is one directory installed as a unit, holding skills, agents, hooks, MCP servers, and other components. You install the plugin into Claude Code, which loads its components." width="760" height="252" data-path="images/plugins-model.svg" />

<img src="https://mintcdn.com/claude-code/2Q_GtOEovg5qaBem/images/plugins-model-dark.svg?fit=max&auto=format&n=2Q_GtOEovg5qaBem&q=85&s=f6cdefe1fc05daf3b253d26e9f3f70f6" className="hidden dark:block" alt="Diagram of the marketplace path in three boxes, left to right. A marketplace, a catalog of plugins, lists a plugin. The plugin is one directory installed as a unit, holding skills, agents, hooks, MCP servers, and other components. You install the plugin into Claude Code, which loads its components." width="760" height="252" data-path="images/plugins-model-dark.svg" />

[플러그인 설치 및 관리](/docs/ko/plugins/install#install-a-plugin)는 Claude Code를 실행하는 각 위치에 대한 설치 단계를 제공합니다. 플러그인을 개발하는 동안 마켓플레이스가 필요하지 않습니다. [마켓플레이스 없이 개발](/docs/ko/plugins/create#develop-without-a-marketplace)에서 보여주는 대로 `--plugin-dir`을 사용하여 폴더에서 직접 로드합니다.

<h3 id="make-an-installed-plugin-available-in-your-session">
  설치된 플러그인을 세션에서 사용 가능하게 만들기
</h3>

설치한 플러그인이 실행할 수 있는 스킬을 제공하기 전에 다음 각 계층에 있어야 합니다:

* **설정**: 설정에 추가한 마켓플레이스와 활성화된 플러그인이 나열됩니다.
* **디스크**: `~/.claude/plugins/`는 Claude Code가 가져오고 설치한 것을 보유합니다.
* **세션**: 플러그인은 시작 시 로드되거나 [플러그인을 다시 로드](/docs/ko/plugins/loading#check-which-stage-a-plugin-reached)할 때 로드됩니다.

[플러그인 로딩 참조](/docs/ko/plugins/loading)에서 각 계층의 규칙, 어떤 설정 파일이 우선하는지, 디스크의 파일 위치를 읽으세요.

<h2 id="tell-anthropic’s-marketplaces-from-third-party-ones">
  Anthropic의 마켓플레이스와 타사 마켓플레이스 구분하기
</h2>

마켓플레이스의 이름은 세 가지 계층 중 하나에 배치합니다. Claude Code는 `github.com/anthropics/` 저장소에서 소싱된 공식 및 커뮤니티 이름만 마켓플레이스에 대해 허용합니다:

* **공식**: `claude-plugins-official` 및 데모 마켓플레이스 `claude-code-plugins`를 포함하여 Anthropic의 [공식 마켓플레이스 이름](/docs/ko/plugins/security#official-marketplace-names) 중 하나를 가진 마켓플레이스입니다.
* **커뮤니티**: `claude-community`와 같은 Anthropic의 커뮤니티 이름을 가진 마켓플레이스입니다. [Anthropic의 마켓플레이스를 이름으로 식별](/docs/ko/plugins/security#marketplace-tiers)하면 이들을 나열합니다.
* **타사**: 다른 모든 마켓플레이스입니다. 동료나 조직이 게시하는 마켓플레이스는 타사입니다.

계층에 관계없이 설치하는 플러그인은 사용자 권한으로 코드를 실행할 수 있습니다. 플러그인을 설치하기 전에 검토하는 방법은 [플러그인 보안 및 신뢰](/docs/ko/plugins/security)를 읽으세요.

[관리 설정](/docs/ko/settings#settings-files)을 통해 조직은 마켓플레이스를 허용 목록에 추가하거나 차단하고, 플러그인을 강제 설치하고, 세션 전용 로딩을 비활성화할 수 있습니다. 이러한 컨트롤은 [조직의 플러그인 관리](/docs/ko/plugins/org)를 읽으세요.

<h2 id="understand-install-scopes">
  설치 범위 이해하기
</h2>

플러그인을 설치할 때 범위를 선택하고, 범위는 플러그인이 활성화되는 대상을 결정합니다:

* **사용자 범위**: 이 컴퓨터의 모든 프로젝트에서 활성화됨
* **프로젝트 범위**: 커밋된 `.claude/settings.json`을 통해 이 저장소에서 작업하는 모든 사람에게 활성화됨. 각 협력자는 여전히 [자신의 컴퓨터에 설치](/docs/ko/plugins/loading#enabled-in-project-settings-but-not-installed)해야 합니다.
* **로컬 범위**: 이 저장소에서만 활성화됨

터미널, 데스크톱 앱의 로컬 세션 또는 VS Code 확장에서 사용자 범위로 설치한 플러그인은 모두 동일한 설정 파일을 읽기 때문에 해당 컴퓨터의 다른 두 개에서 사용할 수 있습니다. 범위를 선택하는 방법은 [설치 범위 선택](/docs/ko/plugins/install#choose-an-install-scope)을 참조하세요.

claude.ai/code의 브라우저를 포함한 클라우드 세션은 로컬 설정의 플러그인을 로드하지 않습니다. 터미널, VS Code 및 데스크톱 앱의 설치 단계와 클라우드 세션이 로드하는 것은 [플러그인 설치](/docs/ko/plugins/install#install-a-plugin)를 참조하세요.

<Note>
  동일한 플러그인 형식은 claude.ai 및 Cowork에도 설치되며, 다른 구성 요소 집합이 로드됩니다. 이러한 표면의 경우 claude.com의 [claude.ai 및 Cowork의 플러그인](https://claude.com/docs/plugins/overview)을 참조하세요.
</Note>

<h2 id="next-steps">
  다음 단계
</h2>

대부분의 사람들은 Anthropic의 공식 마켓플레이스에서 플러그인을 설치하는 것으로 시작합니다. Claude Code는 대화형 터미널 세션을 처음 시작할 때 이를 추가합니다. 터미널 세션에서 `/plugin`을 실행하여 검색하거나 [플러그인 설치 및 관리](/docs/ko/plugins/install)를 따릅니다. 이는 데스크톱 앱 및 VS Code도 다룹니다. Claude Code를 열기 전에 해당 마켓플레이스에 있는 것을 보려면 웹에서 [Claude Marketplace](https://claude.com/marketplace/plugins)를 검색하세요.

자신만의 플러그인을 만들려면 [플러그인 만들기](/docs/ko/plugins/create)는 빈 디렉토리로 시작하여 작동하는 플러그인으로 끝납니다.

플러그인을 설치하거나 만든 후 다음 페이지는 다음에 올 것을 다룹니다:

* **만든 것 공유**: [플러그인 게시 및 배포](/docs/ko/plugins/publish)
* **작동 여부 및 사용 여부 확인**: [평가로 플러그인 테스트](/docs/ko/plugin-evals) 및 [플러그인 비용 및 사용 측정](/docs/ko/plugins/measure)
* **팀을 위한 마켓플레이스 실행**: [마켓플레이스 만들기](/docs/ko/plugins/create-marketplace), 그 다음 [마켓플레이스 호스팅 및 유지 관리](/docs/ko/plugins/host-marketplace)
* **조직의 플러그인 정책 설정**: [조직의 플러그인 관리](/docs/ko/plugins/org)
* **문제 해결**: [플러그인 문제 해결](/docs/ko/plugins/troubleshooting)
