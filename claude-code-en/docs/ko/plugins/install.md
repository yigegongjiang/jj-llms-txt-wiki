> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 플러그인 설치 및 관리

> 마켓플레이스에서 Claude Code 플러그인을 설치하고, 설치 범위를 선택하며, 나중에 업데이트하거나 제거합니다.

플러그인을 설치하면 해당 플러그인의 skills, agents, hooks 및 MCP servers가 사용자의 머신에 있는 Claude Code에 추가됩니다.

이 페이지는 터미널, 데스크톱 앱, IDE 또는 클라우드 세션에서 자신의 머신이나 계정에서 플러그인을 사용하는 모든 사용자를 위한 것입니다. 플러그인 설치, 범위 선택, 마켓플레이스 추가 및 플러그인 업데이트 유지에 대해 다룹니다.

<Note>
  다음 경우는 다른 페이지에서 다룹니다:

  * **claude.ai 채팅 또는 Cowork를 사용하며, Claude Code는 사용하지 않음**: [claude.ai 및 Cowork의 플러그인](https://claude.com/docs/plugins/overview) 참조
  * **Claude Code에서 오류가 발생함**: [플러그인 문제 해결](/docs/ko/plugins/troubleshooting)에서 찾기
</Note>

[플러그인 설치](#install-a-plugin)부터 시작하세요. 누군가가 보낸 설치 명령의 `@` 이름이 `claude-plugins-official`이 아닌 경우, 먼저 [마켓플레이스 추가](#add-a-marketplace)를 하세요.

<h2 id="install-a-plugin">
  플러그인 설치
</h2>

예시로, 이 섹션에서는 [Anthropic의 공식 마켓플레이스](/docs/ko/plugins/anthropic-marketplaces)에서 [`commit-commands`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/commit-commands)를 설치합니다. 이 플러그인은 커밋, 푸시 및 풀 요청 열기를 위한 명령을 추가합니다.

동일한 단계로 다른 플러그인을 설치합니다. `commit-commands`와 `claude-plugins-official`이 나타나는 곳마다 해당 플러그인의 이름과 마켓플레이스 이름으로 대체하세요. 해당 플러그인이 다른 마켓플레이스에서 제공되는 경우, 먼저 [마켓플레이스를 추가](#add-a-marketplace)하세요.

Claude Code를 실행하는 위치에 해당하는 탭을 선택하세요.

<Tabs>
  <Tab title="Terminal">
    프로젝트에서 `claude`로 Claude Code를 시작한 후:

    <Steps>
      <Step title="설치 명령으로 플러그인의 세부 정보 열기">
        플러그인의 이름과 마켓플레이스와 함께 `/plugin install`을 실행합니다. 세션에서 이 명령은 즉시 설치하지 않습니다. 해당 플러그인의 세부 정보를 보여주는 `/plugin` 패널을 열어서 검토하고 먼저 범위를 선택할 수 있습니다.

        ```text theme={null}
        /plugin install commit-commands@claude-plugins-official
        ```

        대신 검색하려면 플러그인 이름 없이 `/plugin`을 실행합니다. 패널이 **Discover** 탭에서 열리며, 추가한 모든 마켓플레이스의 플러그인을 나열하고, 입력하여 검색한 후 플러그인에서 **Enter**를 눌러 세부 정보를 열 수 있습니다.
      </Step>

      <Step title="플러그인이 추가하는 항목 검토">
        세부 정보 창에는 플러그인의 설명이 표시됩니다. 다음도 표시할 수 있습니다:

        * **Will install**: 플러그인이 추가하는 명령, agents, skills, hooks 및 MCP와 LSP servers.
        * **Last updated**: Anthropic의 공식 마켓플레이스에 있는 플러그인에 대해 표시됩니다.
        * **Context cost**: Anthropic의 공식 마켓플레이스에 있는 플러그인의 경우, 두 가지 토큰 추정치입니다. **Every turn**은 플러그인이 보내는 각 메시지에 추가하는 것이고, **When invoked**는 Claude가 skills과 agents를 로드한 후 추가하는 것입니다. 추정치는 1단계 명령처럼 마켓플레이스 이름을 지정하여 플러그인을 열거나 **Marketplaces** 탭에서 나타납니다. **Discover** 목록에서 도달한 세부 정보 창에는 표시되지 않습니다.

        로컬 또는 사용자 정의 마켓플레이스의 플러그인은 대신 `Components will be discovered at installation`을 표시할 수 있습니다.

        플러그인은 hooks와 MCP servers를 실행할 수 있으므로 설치하기 전에 창을 읽으세요. [플러그인 보안 및 신뢰](/docs/ko/plugins/security)를 참조하세요.
      </Step>

      <Step title="범위 선택">
        세 가지 설치 옵션 중 하나를 선택합니다:

        * **Install for you (user scope)**: 이 머신의 모든 프로젝트에서 플러그인을 얻습니다
        * **Install for all collaborators on this repository (project scope)**: 이 저장소에서 작업하는 모든 사람에게 활성화됩니다
        * **Install for you, in this repo only (local scope)**: 이 저장소에서만 플러그인을 얻습니다

        [설치 범위 선택](#choose-an-install-scope)에서는 각 범위가 어느 설정 파일에 기록되는지, 동일한 플러그인이 둘 이상의 범위에서 설정된 경우 어느 것이 적용되는지 설명합니다.

        범위를 선택한 후, Claude Code는 플러그인과 선언한 모든 종속성을 설치한 후 설치 요약을 인쇄합니다.
      </Step>

      <Step title="설치 요약 읽기">
        요약의 마지막 문장은 플러그인이 이 세션에서 사용 가능한지 여부를 알려줍니다:

        * **Active now**: `Plugin is now active.` 다시 로드할 필요가 없습니다.
        * **Reload needed**: `Run /reload-plugins to activate.` 패널이 닫히고 Claude Code가 해당 다시 로드를 실행합니다. 다시 로드가 [프롬프트 캐시를 무효화](/docs/ko/prompt-caching#enabling-or-disabling-a-plugin)하면 경고하고 대신 플러그인을 보류 상태로 둡니다. `/reload-plugins --force`를 실행하여 어쨌든 활성화하면 캐시되지 않은 요청 하나가 소요됩니다.
        * **Load failed**: `The plugin couldn't be loaded`. `/plugin`의 **Errors** 탭을 열어 이유를 확인한 후 [설치 후: 플러그인이 작동하지 않음](/docs/ko/plugins/troubleshooting#plugin-installed-but-not-working)을 참조하세요.
      </Step>

      <Step title="플러그인이 작동하는지 확인">
        `/`를 입력하고 플러그인의 skills를 `/<plugin>:<skill>` 형식으로 플러그인 이름 아래에서 찾습니다. `commit-commands`의 경우 `/commit-commands:commit`이 나타납니다. 플러그인을 나열하는 다른 두 곳이 있습니다:

        * `/plugin`의 **Installed** 탭을 열면 플러그인과 해당 범위가 나열됩니다.
        * 셸에서 `claude plugin list`를 실행하면 `Version`, `Scope` 및 `Status` 줄과 함께 동일한 목록을 인쇄합니다.

        `/commit-commands:commit`이 나타나지 않으면 [설치 후: 플러그인이 작동하지 않음](/docs/ko/plugins/troubleshooting#plugin-installed-but-not-working)을 참조하세요.
      </Step>
    </Steps>

    다른 마켓플레이스에서 설치하려면 먼저 한 가지 추가 단계가 필요합니다: [마켓플레이스 추가](#add-a-marketplace). Claude Code는 대화형 터미널 세션을 처음 시작할 때 Anthropic의 공식 마켓플레이스를 추가하므로 예시에서는 해당 단계를 건너뜁니다. [claude.com/marketplace](https://claude.com/marketplace)에서 플러그인을 찾은 경우, 해당 **Claude Code** 버튼은 [셸 형식](#install-from-your-shell)인 `claude plugin install <name>@claude-plugins-official`의 설치 명령을 복사합니다.
  </Tab>

  <Tab title="Desktop app">
    데스크톱 앱의 **Code** 탭에서 로컬 또는 SSH 세션:

    <Steps>
      <Step title="플러그인 브라우저 열기">
        프롬프트 상자 옆의 **+** 버튼을 클릭하고 **Plugins**를 선택한 후 **Add plugin**을 선택합니다. 플러그인 브라우저가 마켓플레이스의 플러그인과 함께 열립니다.
      </Step>

      <Step title="플러그인 선택">
        `commit-commands`를 찾아 선택합니다.
      </Step>

      <Step title="범위 선택">
        [범위](#choose-an-install-scope)를 선택합니다: 사용자 계정, 이 프로젝트 또는 로컬 전용.
      </Step>
    </Steps>

    나중에 활성화, 비활성화 또는 제거하려면 **+ > Plugins > Manage plugins**를 사용합니다. 플러그인 브라우저는 데스크톱 앱의 클라우드 세션에서 사용할 수 없습니다. [데스크톱 앱에서 플러그인 설치](/docs/ko/desktop#install-plugins)를 참조하세요.
  </Tab>

  <Tab title="VS Code">
    VS Code의 Claude Code 패널에서:

    <Steps>
      <Step title="플러그인 관리 열기">
        프롬프트 상자에 `/plugins`를 입력하여 **Manage plugins**를 엽니다.
      </Step>

      <Step title="플러그인 설치">
        **Plugins** 탭에서 `commit-commands`를 검색하고 **Install**을 클릭합니다. 탭에 플러그인이 나열되지 않으면 먼저 **Marketplaces** 탭에서 `anthropics/claude-plugins-official`을 추가하세요.
      </Step>

      <Step title="범위 선택">
        [범위](#choose-an-install-scope)를 선택합니다: **Install for you**, **Install for this project** 또는 **Install locally**.
      </Step>
    </Steps>

    변경 사항은 다시 시작 없이 열린 세션에 적용됩니다. [VS Code에서 플러그인 관리](/docs/ko/vs-code#manage-plugins)를 참조하세요.
  </Tab>

  <Tab title="Cloud session">
    [클라우드 세션](/docs/ko/cloud-environments) (예: [claude.ai/code의 브라우저](/docs/ko/claude-code-on-the-web))에는 플러그인 브라우저가 없으며 자신의 머신에 설치한 플러그인이나 저장소의 `.claude/settings.json`이 켜는 플러그인을 로드하지 않습니다. 조직이 관리 설정을 통해 배포하는 플러그인의 경우 [조직을 위한 플러그인 관리](/docs/ko/plugins/org)를 참조하세요.

    [클라우드 세션에서도 사용 가능한 설정의 어느 부분](/docs/ko/cloud-environments#what-carries-over-from-your-setup)을 참조하여 나머지 설정을 확인하세요.
  </Tab>
</Tabs>

<h3 id="choose-an-install-scope">
  설치 범위 선택
</h3>

플러그인의 설치 범위는 누가 플러그인을 얻는지, 어느 설정 파일에 활성화된 것으로 기록되는지를 결정합니다:

* **User scope**: 플러그인이 이 머신의 모든 프로젝트에서 사용자에게 활성화됩니다. 항목은 `~/.claude/settings.json`의 `enabledPlugins`에 들어갑니다.
* **Project scope**: 플러그인이 이 저장소에서 작업하는 모든 사람에게 활성화됩니다. 항목은 커밋하는 `.claude/settings.json`에 들어갑니다.
* **Local scope**: 플러그인이 이 저장소에서만 사용자에게 활성화됩니다. 항목은 `.claude/settings.local.json`에 들어갑니다.

일부 플러그인은 [`defaultEnabled`](/docs/ko/plugins/manifest-reference#defaultenabled) 필드를 통해 작성자가 꺼진 상태로 시작하도록 설정합니다. 이러한 플러그인은 설치되지만 셸에서 `claude plugin enable <name>`을 사용하거나 세션의 `/plugin`의 **Installed** 탭에서 켤 때까지 꺼진 상태로 유지됩니다.

동일한 플러그인이 여러 범위에서 설정된 경우, 로컬 설정이 프로젝트 설정을 재정의하고, 프로젝트 설정이 사용자 설정을 재정의합니다. [플러그인이 활성화된 위치 찾기](/docs/ko/plugins/loading#find-where-a-plugin-is-enabled)에서 전체 규칙을 참조하세요.

터미널, 데스크톱 앱의 로컬 세션 및 한 컴퓨터의 VS Code 확장은 동일한 설정 파일을 읽으므로, 이들 중 하나에서 사용자 범위로 설치한 플러그인은 다른 두 개에서도 사용 가능합니다.

<h3 id="other-places-you-run-claude-code">
  JetBrains, 비대화형 실행 및 Agent SDK
</h3>

Claude Code를 실행하는 일부 위치에는 자체 플러그인 브라우저가 없습니다:

* **JetBrains IDEs**: JetBrains 플러그인은 IDE의 터미널에서 Claude Code를 실행하므로 **Terminal** 탭의 단계를 사용하세요.
* **`claude -p` 및 기타 비대화형 실행**: `/plugin`이 실행되지 않으며, Claude는 `/plugin isn't available in this environment.`로 응답합니다. 이미 설치한 플러그인은 로드됩니다. 셸에서 [`claude plugin` 명령](#install-from-your-shell)으로 설치하고 관리합니다.
* **Agent SDK**: SDK의 플러그인 옵션을 통해 플러그인을 로드합니다. [Agent SDK에서 플러그인 로드](/docs/ko/agent-sdk/plugins)를 참조하세요.

Claude Code가 저장소의 `.claude/settings.json`에서 활성화된 플러그인이 설치되지 않았다고 보고하면 [프로젝트 설정에서 활성화되었지만 설치되지 않음](/docs/ko/plugins/loading#enabled-in-project-settings-but-not-installed)을 참조하세요.

<Tip>
  플러그인 작성자이고 디스크에 있는 플러그인 복사본을 테스트하는 경우, 셸에서 `--plugin-dir`로 Claude Code를 시작하여 설치하는 대신 한 세션 동안 로드합니다. [한 세션 동안 플러그인을 로드하는 플래그](/docs/ko/plugins/cli-reference#flags-that-load-a-plugin-for-one-session)를 참조하세요.
</Tip>

<h3 id="plugins-from-your-claude-ai-account">
  claude.ai 계정의 플러그인
</h3>

claude.ai 계정은 마켓플레이스에서 설치하는 플러그인과 함께 플러그인의 별도 소스입니다:

* **도착하는 것**: claude.ai 계정에서 켜는 모든 플러그인, 그리고 조직이 구성원을 위해 켜는 모든 플러그인. 터미널 세션에서는 해당 계정으로 로그인한 상태에서 Claude Code를 시작할 때마다 백그라운드에서 동기화되고, Cowork 세션에서는 세션이 시작될 때 다운로드됩니다.
* **어디서 보는지**: `/plugin` 및 `claude plugin list`에서 ID `<name>@synced` 아래. 조직이 요구하지 않는 한 자신의 범위에서 하나를 끌 수 있습니다.
* **다른 방향으로 가지 않는 것**: `/plugin` 또는 `claude plugin install`로 설치한 플러그인은 이 머신에 남아 있으며 claude.ai 계정에 추가되지 않습니다.

동기화 타이밍, 로그인 요구 사항 및 동기화 끄기에 대해서는 [claude.ai에서 동기화된 플러그인](/docs/ko/plugins/loading#synced-plugins)을 참조하세요.

<h3 id="install-from-your-shell">
  셸에서 설치
</h3>

셸에서 `claude plugin install`을 실행하여 Claude Code 세션을 시작하지 않고 플러그인을 설치합니다. 예를 들어 설정 스크립트에서.

* **범위**: 기본적으로 사용자 범위. `--scope project` 또는 `--scope local`을 전달하여 변경합니다.
* **플러그인이 로드되는 시기**: 설치한 플러그인은 다음 번에 Claude Code를 시작할 때 또는 이미 열려 있는 세션에서 `/reload-plugins`을 실행할 때 로드됩니다.
* **마켓플레이스를 먼저 추가해야 함**: 아직 아무도 대화형 Claude Code 세션을 열지 않은 머신에서는 공식 마켓플레이스가 등록되지 않으므로, 이를 설치하는 스크립트는 설치 전에 `claude plugin marketplace add anthropics/claude-plugins-official`을 실행합니다.

```bash theme={null}
claude plugin install formatter@your-org --scope project
```

명령이 완료되면 `Successfully installed plugin: formatter@your-org (scope: project)`를 인쇄합니다.

일부 플러그인은 마켓플레이스가 이름을 지정한 명령을 실행하여 설치되며, 이를 [`command` source](/docs/ko/plugins/marketplace-reference#command-plugin-source)라고 합니다. Claude Code는 해당 명령을 표시하고 실행하기 전에 수락하도록 요청합니다. 스크립트에는 해당 프롬프트에 답할 사람이 없으므로 거기에 `--yes`를 전달하여 수락합니다.

모든 `claude plugin install` 플래그에 대해서는 [plugin install](/docs/ko/plugins/cli-reference#plugin-install)을 참조하세요.

<h2 id="add-a-marketplace">
  마켓플레이스 추가
</h2>

Anthropic의 공식 마켓플레이스에 없는 플러그인을 원할 때만 이 섹션이 필요합니다. 예를 들어 동료가 게시한 플러그인이나 Anthropic의 커뮤니티 마켓플레이스의 플러그인.

마켓플레이스는 플러그인의 카탈로그이며, Claude Code는 마켓플레이스에서 설치하기 전에 마켓플레이스에 대해 알아야 합니다. 마켓플레이스를 한 번 추가합니다. 그 후, 해당 플러그인은 **Discover** 탭에 나타나고 세션에서 `/plugin install <plugin>@<marketplace>` 또는 셸에서 `claude plugin install <plugin>@<marketplace>`로 설치합니다. 여기서 `<marketplace>`는 마켓플레이스가 등록한 이름입니다. 한 단계로 둘 다 수행하려면 [마켓플레이스 추가 및 한 명령으로 설치](#add-a-marketplace-and-install-in-one-command)를 참조하세요.

Claude Code 세션에서 `/plugin marketplace add`를 실행한 후 마켓플레이스의 소스를 입력합니다: GitHub 저장소, 모든 호스트의 git 저장소, 로컬 디렉토리 또는 파일, 또는 호스팅된 `marketplace.json`.

| 소스                      | 입력할 내용                                                                                                                                                       | 예시                                                                                                                        |
| :---------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------ |
| GitHub 저장소              | `owner/repo`. 분기 또는 태그를 고정하려면 `#ref`를 추가합니다.                                                                                                                 | `/plugin marketplace add anthropics/claude-code`, 또는 `v1.2.0` 태그를 고정하려면 `/plugin marketplace add your-org/plugins#v1.2.0` |
| 모든 호스트의 Git 저장소         | 전체 클론 URL. 분기 또는 태그를 고정하려면 `#ref`를 추가합니다.                                                                                                                    | `/plugin marketplace add https://gitlab.example.com/your-group/your-marketplace.git#v1.0.0`                               |
| 로컬 디렉토리 또는 파일           | `.claude-plugin/marketplace.json`을 보유한 디렉토리 또는 JSON 파일 자체에 대한 상대 또는 절대 경로. 상대 경로를 `./` 또는 `../`로 시작합니다. Claude Code는 bare `name/name`을 GitHub 저장소로 읽기 때문입니다. | `/plugin marketplace add ./my-marketplace`                                                                                |
| 호스팅된 `marketplace.json` | 해당 `https://` URL                                                                                                                                            | `/plugin marketplace add https://example.com/marketplace.json`                                                            |

셸에서 `claude plugin marketplace add`는 동일한 소스를 사용합니다.

<Tip>
  `/plugin market`도 `/plugin marketplace`의 더 짧은 형식으로 작동합니다.
</Tip>

모든 URL에 `https://` 접두사를 포함하거나 SSH의 경우 `git@host:path` 형식을 사용합니다. bare `gitlab.example.com/your-group/your-marketplace.git`을 입력하면 Claude Code는 이를 GitHub `owner/repo` 약자로 읽고 거부합니다.

명령이 성공하면 `Successfully added marketplace: <name>`을 인쇄하고, 마켓플레이스의 플러그인은 다음 번에 `/plugin`을 열 때 **Discover** 탭에 나타나며, 다시 로드할 필요가 없습니다. 실패하면 [플러그인 문제 해결](/docs/ko/plugins/troubleshooting#add-a-marketplace)에서 오류 메시지를 일치시킵니다.

<h3 id="add-a-marketplace-and-install-in-one-command">
  마켓플레이스 추가 및 한 명령으로 설치
</h3>

아직 추가하지 않은 마켓플레이스에서 플러그인을 설치하려면 Claude Code 세션에서 `/plugin install`을 실행하고 `--marketplace`로 마켓플레이스 소스를 이름 지정합니다. Claude Code v2.1.275 이상이 필요합니다.

```text theme={null}
/plugin install deploy-helper --marketplace your-org/plugins
```

소스는 GitHub `owner/repo`, git URL 또는 로컬 경로와 같이 [/plugin marketplace add와 동일한 형식](#add-a-marketplace)을 사용합니다. 단, 공백을 포함할 수 없습니다. 플러그인 이름을 `@marketplace` 접미사 없이 지정합니다.

아직 해당 마켓플레이스를 추가하지 않았으면 Claude Code는 해결한 소스를 표시하고 추가하기 전에 확인하도록 요청합니다. 마켓플레이스가 추가되면 플러그인의 세부 정보가 열리고 [설치 범위](#install-a-plugin)를 선택합니다. 소스가 이미 추가한 마켓플레이스와 일치하면 Claude Code는 확인을 건너뛰고 해당 마켓플레이스에서 플러그인의 세부 정보를 엽니다.

<h3 id="add-a-private-marketplace">
  비공개 마켓플레이스 추가
</h3>

비공개 마켓플레이스는 GitHub 또는 다른 git 호스트의 저장소에 있으며, 복제하려면 자격 증명이 필요합니다. 공개 마켓플레이스와 동일한 `/plugin marketplace add` 또는 `claude plugin marketplace add` 명령으로 추가합니다. Claude Code는 머신에 이미 있는 git 자격 증명으로 복제하고 절대 프롬프트하지 않으므로, 각 연결 방식에는 요구 사항이 있습니다:

* **HTTPS**: git 자격 증명 도우미가 적용되므로 `gh auth login`, macOS Keychain 또는 `git-credential-store`로 설정한 액세스가 작동합니다. 대화형 프롬프트가 억제되므로 인증한 적이 없는 호스트는 암호를 요청하는 대신 실패합니다.
* **SSH**: 호스트가 이미 `known_hosts` 파일에 있어야 하고 키가 암호 프롬프트 없이 작동해야 합니다. 호스트 지문 및 암호 프롬프트도 억제되기 때문입니다.
* **GitHub `owner/repo` 약자**: Claude Code는 SSH 키가 `github.com`에 인증되는지 확인한 후, 인증되면 SSH를 통해 복제하고 인증되지 않으면 HTTPS를 통해 복제합니다. [`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`](/docs/ko/env-vars#variables)을 설정하여 해당 확인을 건너뛰고 항상 HTTPS를 통해 복제합니다.

동일한 자격 증명이 `/plugin install`, `/plugin marketplace update` 및 `claude plugin update`를 실행할 때 적용됩니다.

GitHub Enterprise Server 호스트에서는 [GHES의 플러그인 마켓플레이스](/docs/ko/github-enterprise-server#plugin-marketplaces-on-ghes)를 참조하여 각 작업에 필요한 자격 증명을 확인하세요.

조직이 관리 설정을 통해 마켓플레이스를 등록하면 직접 추가할 필요가 없습니다. [플러그인 사전 설치 및 요구](/docs/ko/plugins/org#pre-install-and-require-plugins)를 참조하세요.

<h3 id="add-from-claude-ai">
  claude.ai에서 마켓플레이스 추가
</h3>

[claude.ai 계정에서 플러그인이 동기화되는](/docs/ko/plugins/loading#synced-plugins) 터미널 세션에서, claude.ai는 조직의 플러그인 라이브러리 및 자신의 claude.ai 업로드와 같은 플러그인 마켓플레이스를 나열할 수도 있습니다. 소스가 아닌 이름으로 이들 중 하나를 추가합니다. claude.ai에서 마켓플레이스를 추가하려면 Claude Code v2.1.273 이상이 필요합니다.

`/plugin` 패널 또는 셸에서 claude.ai 마켓플레이스를 추가합니다:

* **세션 내**: `/plugin`을 실행하고 **Marketplaces** 탭으로 이동합니다. 여기에는 claude.ai의 마켓플레이스가 나열됩니다. 거기서 하나를 선택하여 추가합니다.
* **셸에서**: `claude plugin marketplace list`를 실행합니다. 이는 `From claude.ai:` 섹션에 인쇄합니다. 그런 다음 `claude plugin marketplace add`를 `--claudeai` 플래그 및 목록에 표시된 이름으로 실행합니다.

예를 들어, 이 명령은 `claudeai-organization-library`라는 마켓플레이스를 추가합니다:

```bash theme={null}
claude plugin marketplace add --claudeai claudeai-organization-library
```

Claude Code는 마켓플레이스를 `claudeai-`로 시작하는 로컬 이름으로 등록합니다. 이는 claude.ai가 나열한 이름에서 파생됩니다. 예를 들어, "Organization library"로 나열된 마켓플레이스는 `claudeai-organization-library`가 됩니다. 예를 들어 `claude plugin install <plugin>@claudeai-organization-library`로 해당 이름으로 플러그인을 설치합니다.

로그아웃하거나 다른 claude.ai 조직에 로그인하면 마켓플레이스는 구성된 상태로 유지되지만 플러그인을 표시하지 않으며, 이미 설치한 플러그인은 계속 로드됩니다.

`From claude.ai:` 섹션은 또한 claude.ai를 통해 공유되는 git 기반 마켓플레이스를 나열할 수 있으며, 각각에 대한 소스를 인쇄합니다. `--claudeai`가 아닌 [마켓플레이스 추가](#add-a-marketplace)에서와 같이 해당 소스로 추가합니다.

<h2 id="manage-installed-plugins">
  설치된 플러그인 관리
</h2>

**Installed** 탭의 `/plugin`에는 플러그인이 나열되며, 각 플러그인을 활성화, 비활성화, 업데이트 또는 제거할 수 있는 작업이 있습니다. Claude Code 세션에서 `/plugin`을 실행하고 **Tab**을 눌러 도달하거나, `/plugin enable`, `/plugin disable` 또는 `/plugin uninstall`을 실행하여 패널을 열고 변경할 수 있습니다. 비활성화된 플러그인은 목록 하단의 축소된 헤더 아래에 그룹화됩니다. 목록에서 다음 키를 사용합니다:

* 입력하여 이름 또는 설명으로 필터링합니다.
* **Space**를 눌러 선택한 플러그인을 활성화 또는 비활성화하고, **f**를 눌러 즐겨찾기에 추가합니다.
* **Enter**를 눌러 플러그인의 세부 정보를 엽니다. 여기의 메뉴는 **Disable plugin** 또는 **Enable plugin**, **Update now**, **Uninstall**을 제공합니다. 설정을 사용하는 플러그인은 **Configure options**도 제공합니다.

탭은 **Managed** 범위의 플러그인도 표시할 수 있습니다. 조직이 [관리 설정](/docs/ko/settings#settings-files)을 통해 설치한 플러그인이며, 여기서 활성화, 비활성화 또는 제거할 수 없습니다.

조직이 claude.ai에서 요구하는 동기화된 플러그인의 경우 [claude.ai에서 동기화된 플러그인 관리](#manage-plugins-synced-from-claude-ai)를 참조하세요.

`/plugin` 패널을 닫을 때 패널에서 변경한 보류 중인 변경 사항이 있으면 Claude Code가 `/reload-plugins`을 실행하여 적용합니다. 다시 로드하면 [프롬프트 캐시가 무효화](/docs/ko/prompt-caching#enabling-or-disabling-a-plugin)되는 경우 경고하고 대신 변경 사항을 보류 상태로 둡니다. `/reload-plugins --force`를 실행하여 어쨌든 적용합니다.

<h3 id="manage-plugins-synced-from-claude-ai">
  claude.ai에서 동기화된 플러그인 관리
</h3>

`/plugin`의 **Installed** 탭은 [claude.ai 계정에서 동기화된 플러그인](/docs/ko/plugins/loading#synced-plugins)도 나열하며, 소스로 `synced`를 표시합니다. 동기화된 플러그인은 Claude Code v2.1.273 이상의 터미널 세션에 나타납니다.

* **활성화 또는 비활성화**: 조직이 플러그인을 필수로 표시하지 않은 경우 **Installed** 탭을 사용합니다.
* **제거**: claude.ai에서 플러그인을 끕니다.

Claude Code가 추가, 업데이트 또는 제거된 플러그인을 대화형 세션으로 동기화하면 `Plugins changed. Run /reload-plugins to activate.`가 표시됩니다. `/reload-plugins`를 실행하여 해당 세션에서 변경 사항을 로드하거나, Claude Code를 다음에 시작할 때까지 기다립니다.

<h3 id="uninstall-a-plugin-the-project-enables">
  프로젝트가 활성화하는 플러그인 제거
</h3>

이 저장소의 `.claude/settings.json`이 활성화하는 플러그인에 대해 **Uninstall**을 선택할 때, **Installed** 탭에서든 `/plugin uninstall`로든 Claude Code는 비활성화할지 아니면 모두를 위해 제거할지 묻습니다:

* **내 계정에서 비활성화**: **y**를 누릅니다. Claude Code는 `.claude/settings.local.json`에서 플러그인에 대해 `false`를 작성하고 프로젝트에 설치된 상태로 둡니다.
* **모두를 위해 제거**: **u**를 누릅니다. Claude Code는 공유 `.claude/settings.json`에서 플러그인을 제거합니다.

<h3 id="see-what-an-installed-plugin-adds-to-your-sessions">
  설치된 플러그인이 세션에 추가하는 항목 확인
</h3>

셸에서 설치된 플러그인에 대해 `claude plugin details <name>`을 실행합니다. `Always-on` 줄은 플러그인이 활성화된 모든 세션에 추가하는 토큰 수이며, 구성 요소별 행은 어느 스킬 또는 에이전트가 가장 많이 기여하는지 보여줍니다. 전체 출력 및 각 수치의 의미에 대해서는 [플러그인 비용 측정](/docs/ko/plugins/measure#measure-what-a-plugin-costs)을 참조하세요.

<h3 id="find-plugins-you-no-longer-use">
  더 이상 사용하지 않는 플러그인 찾기
</h3>

`/plugin`의 **Installed** 탭에서 직접 설치했으며 최근에 사용하지 않은 플러그인은 **Not used recently** 헤더 아래에 나타나며, 각 플러그인의 세부 정보는 **Last used** 줄을 표시합니다. 해당 헤더와 줄을 사용하여 여전히 시작 및 컨텍스트 비용을 추가하는 플러그인을 찾은 다음 비활성화하거나 제거합니다.

<h3 id="plugins-with-dependencies">
  종속성이 있는 플러그인
</h3>

플러그인은 종속된 다른 플러그인을 선언할 수 있습니다. 마켓플레이스에서 이러한 플러그인을 설치, 비활성화 또는 제거할 때 Claude Code는 해당 종속성에도 작용합니다:

* **설치**: Claude Code는 플러그인의 선언된 종속성도 동일한 범위에서 설치하고 활성화합니다. 성공 메시지에 나열됩니다.
* **활성화**: Claude Code는 설치되었지만 비활성화된 플러그인의 종속성도 활성화합니다. 선언된 종속성이 설치되지 않은 경우 활성화가 실패하고 메시지는 먼저 설치하도록 알려줍니다.
* **비활성화**: 다른 활성화된 플러그인이 여전히 명명한 플러그인이 필요한 경우 Claude Code는 거부하고 올바른 순서로 둘 다 비활성화하는 연결된 명령을 인쇄합니다.
* **제거**: 자동 설치된 종속성은 셸에서 `claude plugin prune`을 실행할 때까지 유지됩니다. [plugin prune](/docs/ko/plugins/cli-reference#plugin-prune)을 참조하세요.

`--plugin-dir`로 플러그인을 로드한 경우 [플러그인 및 해당 종속성을 로컬로 테스트](/docs/ko/plugins/dependencies#test-a-plugin-and-its-dependency-locally)를 참조하세요.

<h3 id="manage-plugins-from-your-shell">
  셸에서 플러그인 관리
</h3>

Claude Code 세션을 시작하지 않고도 플러그인을 관리할 수 있습니다. 셸에서 `claude plugin install`, `enable`, `disable` 또는 `uninstall`을 일반 터미널 명령으로 실행합니다. 이들은 `/plugin` 패널이 하는 것과 동일한 설정을 변경합니다. 각각은 `--scope`를 사용하여 한 범위를 대상으로 하며, 생략할 때 기본 범위를 사용합니다:

* `enable` 및 `disable`은 설정이 이미 플러그인을 나열하는 가장 구체적인 범위에 작용합니다.
* `install` 및 `uninstall`은 사용자 범위에 작용합니다.

예를 들어, 이 명령은 플러그인을 비활성화한 후 다시 활성화한 다음 프로젝트 범위에서 제거합니다:

```bash theme={null}
claude plugin disable formatter@your-org
claude plugin enable formatter@your-org
claude plugin uninstall formatter@your-org --scope project
```

<h2 id="keep-plugins-updated">
  플러그인 업데이트 유지
</h2>

플러그인은 플러그인이 제공되는 마켓플레이스에서 자동 업데이트가 켜져 있을 때 자동으로 업데이트됩니다. 세션이 시작된 후 Claude Code는 해당 마켓플레이스를 새로고침하고 설치한 플러그인의 디스크 복사본을 업데이트합니다.

실행 중인 세션은 이미 로드한 버전을 유지합니다. 업데이트 후 `Plugin updated: <name> · Run /reload-plugins to apply`가 표시되며, 다음 세션은 새 버전을 자동으로 로드합니다.

각 마켓플레이스 종류별 자동 업데이트 기본값은 다음과 같습니다:

* **기본값으로 켜짐**: `claude-plugins-official` 및 `knowledge-work-plugins`과 `first-party-plugins`을 제외한 [공식 마켓플레이스 이름](/docs/ko/plugins/security#official-marketplace-names), 그리고 [claude.ai에서 추가된 마켓플레이스](#add-from-claude-ai).
* **기본값으로 꺼짐**: 커뮤니티 마켓플레이스, 타사 마켓플레이스, 로컬 개발 마켓플레이스를 포함한 다른 모든 마켓플레이스.

자동 업데이트가 실행되는 시기, 건너뛰는 플러그인, 자동 업데이트를 끄는 환경 변수에 대해서는 [자동 업데이트가 실행되는 시기](/docs/ko/plugins/loading#when-auto-update-runs)를 참조하세요.

<h3 id="turn-auto-update-on-or-off-for-a-marketplace">
  마켓플레이스의 자동 업데이트 켜기 또는 끄기
</h3>

Claude Code 세션에서 `/plugin`을 실행하고 **Marketplaces** 탭으로 이동합니다. 마켓플레이스를 선택한 후 **Enable auto-update** 또는 **Disable auto-update**를 선택합니다.

<h3 id="update-one-plugin-now">
  지금 한 플러그인 업데이트
</h3>

세션에서 `/plugin`의 **Installed** 탭에서 플러그인을 열고 **Update now**를 선택하거나, 셸에서 `claude plugin update <plugin>@<marketplace>`를 실행합니다.

<h3 id="auto-update-from-a-private-marketplace">
  비공개 마켓플레이스에서 자동 업데이트
</h3>

비공개 마켓플레이스의 경우, 백그라운드 자동 업데이트가 SSH 및 HTTPS를 통해 인증하는 방법에 대해 [백그라운드 자동 업데이트가 자격 증명으로 수행하는 작업](/docs/ko/plugins/host-marketplace#what-background-auto-update-does-with-credentials)을 참조하고, 실패할 때 표시되는 메시지에 대해 [플러그인 문제 해결](/docs/ko/plugins/troubleshooting#add-a-marketplace)을 참조하세요.

<h2 id="manage-marketplaces">
  마켓플레이스 관리
</h2>

`/plugin`의 **Marketplaces** 탭은 등록한 모든 마켓플레이스를 해당 소스와 함께 나열합니다. 하나를 선택하여 플러그인을 검색하고, 목록을 업데이트하고, 자동 업데이트를 켜거나 끄거나, 제거합니다.

또한 셸 또는 세션 내에서 명령으로 마켓플레이스를 나열, 업데이트 및 제거할 수 있습니다:

| 작업             | 셸에서                                       | 세션 내                                |
| :------------- | :---------------------------------------- | :---------------------------------- |
| 마켓플레이스 나열      | `claude plugin marketplace list`          | `/plugin marketplace list`          |
| 마켓플레이스 목록 업데이트 | `claude plugin marketplace update <name>` | `/plugin marketplace update <name>` |
| 마켓플레이스 제거      | `claude plugin marketplace remove <name>` | `/plugin marketplace remove <name>` |

마켓플레이스를 제거할 때, Claude Code는 설치한 모든 플러그인을 제거하고 설정 파일에서 `enabledPlugins` 항목을 제거합니다. **Marketplaces** 탭은 확인하도록 요청하기 전에 해당 플러그인의 이름을 지정합니다.

<h2 id="next-steps">
  다음 단계
</h2>

* [Anthropic의 마켓플레이스](/docs/ko/plugins/anthropic-marketplaces): 공식, 커뮤니티 및 데모 마켓플레이스가 어떻게 다르고 각각을 어디서 검색할 수 있는지
* [플러그인 로딩 참조](/docs/ko/plugins/loading): 플러그인이 로드되었거나, 로드되지 않았거나, 업데이트 후 변경되지 않은 이유
* [플러그인 보안 및 신뢰](/docs/ko/plugins/security): 알 수 없는 마켓플레이스에서 플러그인을 설치하기 전에 검토할 사항
* [플러그인 문제 해결](/docs/ko/plugins/troubleshooting): 설치 및 마켓플레이스 오류 메시지 및 해결 방법
* [플러그인 만들기](/docs/ko/plugins/create): 자신의 플러그인 빌드
