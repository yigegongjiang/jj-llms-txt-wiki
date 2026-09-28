> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# VS Code에서 Claude Code 사용하기

> VS Code용 Claude Code 확장 프로그램을 설치하고 구성합니다. 인라인 diff, @-멘션, 계획 검토 및 키보드 단축키를 통해 AI 코딩 지원을 받습니다.

<img src="https://mintcdn.com/claude-code/-YhHHmtSxwr7W8gy/images/vs-code-extension-interface.jpg?fit=max&auto=format&n=-YhHHmtSxwr7W8gy&q=85&s=300652d5678c63905e6b0ea9e50835f8" alt="VS Code 편집기와 오른쪽에 열린 Claude Code 확장 프로그램 패널, Claude와의 대화를 표시" width="2500" height="1155" data-path="images/vs-code-extension-interface.jpg" />

VS Code 확장 프로그램은 Claude Code를 위한 기본 그래픽 인터페이스를 제공하며, IDE에 직접 통합됩니다. 이것이 VS Code에서 Claude Code를 사용하는 권장 방법입니다.

확장 프로그램을 사용하면 Claude의 계획을 수락하기 전에 검토하고 편집할 수 있으며, 편집이 이루어질 때 자동으로 수락하고, 선택 항목에서 특정 줄 범위가 있는 파일을 @-멘션으로 표시하고, 대화 기록에 액세스하고, 별도의 탭이나 창에서 여러 대화를 열 수 있습니다.

<h2 id="prerequisites">
  필수 조건
</h2>

설치하기 전에 다음을 확인하십시오:

* VS Code 1.94.0 이상
* Anthropic 계정: 모든 유료 Claude 구독(Pro, Max, Team 또는 Enterprise) 또는 Claude Console 계정이 작동하며, API 키가 필요하지 않습니다. 확장 프로그램을 처음 열 때 이 계정으로 [로그인](/docs/ko/authentication#log-in-to-claude-code)합니다. Amazon Bedrock이나 Google Cloud의 Agent Platform과 같은 타사 공급자를 통해 Claude에 액세스하는 경우 설정 지침은 [타사 공급자 사용](#use-third-party-providers)을 참조하십시오.

<Tip>
  확장 프로그램에는 채팅 패널용 CLI(명령줄 인터페이스)의 자체 복사본이 포함되어 있습니다. VS Code의 통합 터미널에서 `claude`를 실행하려면 [독립 실행형 CLI 설치](/docs/ko/setup)도 필요합니다. 자세한 내용은 [VS Code 확장 프로그램 vs. Claude Code CLI](#vs-code-extension-vs-claude-code-cli)를 참조하십시오.
</Tip>

<h2 id="install-the-extension">
  확장 프로그램 설치
</h2>

IDE에 대한 링크를 클릭하여 직접 설치합니다:

* [VS Code용 설치](vscode:extension/anthropic.claude-code)
* [Cursor용 설치](cursor:extension/anthropic.claude-code)

또는 VS Code에서 `Cmd+Shift+X`(Mac) 또는 `Ctrl+Shift+X`(Windows/Linux)를 눌러 확장 프로그램 보기를 열고, "Claude Code"를 검색한 후 **설치**를 클릭합니다.

확장 프로그램은 Devin Desktop 또는 Kiro와 같은 다른 VS Code 포크에도 설치됩니다. 편집기의 확장 프로그램 보기에서 "Claude Code"를 검색하거나 [Open VSX 레지스트리](https://open-vsx.org/extension/Anthropic/claude-code)에서 설치합니다. 편집기에서 확장 프로그램을 설치할 수 없는 경우 [CLI](/docs/ko/quickstart)를 설치하고 통합 터미널에서 `claude`를 실행합니다. CLI는 모든 터미널에서 작동합니다.

<Note>설치 후 확장 프로그램이 나타나지 않으면 VS Code를 다시 시작하거나 명령 팔레트에서 "Developer: Reload Window"를 실행합니다.</Note>

<h2 id="get-started">
  시작하기
</h2>

설치 후 VS Code 인터페이스를 통해 Claude Code를 사용할 수 있습니다.

<Steps>
  <Step title="Claude Code 패널 열기">
    VS Code 전체에서 Spark 아이콘은 Claude Code를 나타냅니다. <img src="https://mintcdn.com/claude-code/c5r9_6tjPMzFdDDT/images/vs-code-spark-icon.svg?fit=max&auto=format&n=c5r9_6tjPMzFdDDT&q=85&s=3ca45e00deadec8c8f4b4f807da94505" alt="Spark icon" style={{display: "inline", height: "0.85em", verticalAlign: "middle"}} width="16" height="16" data-path="images/vs-code-spark-icon.svg" />

    Claude를 여는 가장 빠른 방법은 **편집기 도구 모음**(편집기의 오른쪽 상단 모서리)에서 Spark 아이콘을 클릭하는 것입니다. 파일을 열었을 때만 아이콘이 나타납니다.

    <img src="https://mintcdn.com/claude-code/mfM-EyoZGnQv8JTc/images/vs-code-editor-icon.png?fit=max&auto=format&n=mfM-EyoZGnQv8JTc&q=85&s=eb4540325d94664c51776dbbfec4cf02" alt="편집기 도구 모음에서 Spark 아이콘을 보여주는 VS Code 편집기" width="2796" height="734" data-path="images/vs-code-editor-icon.png" />

    Claude Code를 여는 다른 방법:

    * **활동 표시줄**: 왼쪽 사이드바의 Spark 아이콘을 클릭하여 세션 목록을 엽니다. 세션을 클릭하여 [선호하는 위치](#extension-settings)에서 열거나 새 세션을 시작합니다. 이 아이콘은 항상 활동 표시줄에 표시됩니다.
    * **명령 팔레트**: `Cmd+Shift+P`(Mac) 또는 `Ctrl+Shift+P`(Windows/Linux)를 누르고 "Claude Code"를 입력한 후 "새 탭에서 열기"와 같은 옵션을 선택합니다.
    * **상태 표시줄**: [`preferredLocation`](#extension-settings)을 `sidebar`로 설정했거나 **Claude Code: 사이드 바에서 열기**로 Claude를 열었다면 창의 오른쪽 아래 모서리에서 **✻ Claude Code**를 클릭합니다. 파일을 열지 않았을 때도 작동합니다.

    Claude 패널을 드래그하여 VS Code의 어느 곳으로든 재배치할 수 있습니다. 자세한 내용은 [워크플로우 사용자 정의](#customize-your-workflow)를 참조하십시오.
  </Step>

  <Step title="로그인">
    처음 패널을 열면 로그인 화면이 나타납니다. **로그인**을 클릭하고 브라우저에서 인증을 완료합니다.

    나중에 **로그인하지 않음 · /login을 실행하십시오**가 표시되면 확장 프로그램이 자동으로 로그인 화면을 다시 엽니다. 표시되지 않으면 명령 팔레트에서 **개발자: 창 다시 로드**를 사용하여 창을 다시 로드합니다.

    셸에 `ANTHROPIC_API_KEY`가 설정되어 있지만 여전히 로그인 프롬프트가 표시되면 VS Code가 셸 환경을 상속하지 못했을 수 있습니다. 터미널에서 `code .`로 VS Code를 시작하여 환경 변수를 상속하거나 대신 Claude 계정으로 로그인합니다.

    로그인한 후 **Claude Code 배우기** 체크리스트가 나타납니다. **보여주기**를 클릭하여 각 항목을 진행하거나 X로 닫습니다. 나중에 다시 열려면 VS Code 설정의 확장 프로그램 → Claude Code에서 **온보딩 숨기기**를 선택 해제합니다.
  </Step>

  <Step title="프롬프트 전송">
    코드나 파일을 도와달라고 Claude에 요청합니다. 작동 방식 설명, 문제 디버깅 또는 변경 사항 만들기 등이 있습니다.

    <Tip>Claude는 자동으로 선택한 텍스트를 봅니다. `Option+K`(Mac) / `Alt+K`(Windows/Linux)를 눌러 프롬프트에 @-멘션 참조(예: `@file.ts#5-10`)를 삽입할 수도 있습니다.</Tip>

    파일의 특정 줄에 대해 묻는 예시입니다.

    <img src="https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-send-prompt.png?fit=max&auto=format&n=FVYz38sRY-VuoGHA&q=85&s=ede3ed8d8d5f940e01c5de636d009cfd" alt="Python 파일에서 2-3줄이 선택되고 Claude Code 패널에 @-멘션 참조가 있는 해당 줄에 대한 질문이 표시되는 VS Code 편집기" width="3288" height="1876" data-path="images/vs-code-send-prompt.png" />
  </Step>

  <Step title="변경 사항 검토">
    표시되는 내용은 프롬프트 상자 아래에 표시된 [권한 모드](/docs/ko/permission-modes#which-mode-a-session-starts-in)에 따라 다릅니다.

    * 자동 또는 자동 편집 모드에서 Claude는 묻지 않고 작업 공간의 대부분 파일을 편집합니다.
    * 수동 모드에서 Claude가 파일을 편집하려고 하면 원본과 제안된 변경 사항의 나란히 비교를 표시한 후 권한을 요청합니다. 수락, 거부 또는 Claude에 대신 수행할 작업을 알릴 수 있습니다. 수락하기 전에 diff 보기에서 제안된 콘텐츠를 직접 편집하면 Claude는 사용자가 수정했다는 것을 알려지므로 파일이 원래 제안과 일치한다고 가정하지 않습니다.

          <img src="https://mintcdn.com/claude-code/FVYz38sRY-VuoGHA/images/vs-code-edits.png?fit=max&auto=format&n=FVYz38sRY-VuoGHA&q=85&s=e005f9b41c541c5c7c59c082f7c4841c" alt="Claude의 제안된 변경 사항의 diff를 보여주고 편집을 수행할지 여부를 묻는 권한 프롬프트가 있는 VS Code" width="3292" height="1876" data-path="images/vs-code-edits.png" />

    제안된 편집을 한 번에 하나씩 검토하려면 diff의 각 변경 사항 아래에 있는 **이 변경 사항 수락** 및 **이 변경 사항 거부** 버튼을 사용합니다. 변경 사항을 거부하면 제안된 콘텐츠에서 되돌려집니다. 수락하면 검토됨으로 표시됩니다. 전체 파일을 수락하거나 거부하면 여전히 검토가 완료됩니다. 100개 이상의 변경 사항이 있는 diff는 변경별 버튼 없이 열리므로 전체 파일로 검토합니다. 변경별 검토에는 Claude Code v2.1.275 이상이 필요합니다.

    동일한 작업은 편집기의 컨텍스트 메뉴와 명령 팔레트에서 **Claude Code: 커서에서 변경 사항 수락** 및 **Claude Code: 커서에서 변경 사항 거부**로 사용할 수 있습니다.
  </Step>
</Steps>

Claude Code로 할 수 있는 작업에 대한 더 많은 아이디어는 [일반적인 워크플로우](/docs/ko/common-workflows)를 참조하십시오.

<Tip>
  명령 팔레트에서 "Claude Code: 열기 둘러보기"를 실행하여 기본 사항에 대한 안내 투어를 받습니다.
</Tip>

<h2 id="use-the-prompt-box">
  프롬프트 상자 사용
</h2>

프롬프트 상자는 여러 기능을 지원합니다:

* **권한 모드**: 프롬프트 상자 하단의 모드 표시기를 클릭하여 권한 모드를 전환합니다. Pro, Max, Team 플랜에서는 Auto가 기본 시작 권한 모드입니다. [확장 프로그램이 시작 권한 모드를 선택하는 방법](/docs/ko/permission-modes#switch-permission-modes)과 표시기가 제공하는 모든 권한 모드를 참조하세요.
  * **Auto**: 분류기가 사용자에게 묻는 대신 대부분의 작업을 검토합니다. [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)에서 검토하고 차단하는 항목을 참조하세요.
  * **Manual**: Claude가 파일 편집 및 대부분의 셸 명령 전에 권한을 요청합니다.
  * **Plan**: Claude가 수행할 작업을 설명하고 변경 작업을 시작하기 전에 승인을 기다립니다. VS Code는 자동으로 계획을 전체 Markdown 문서로 열어서 Claude가 시작하기 전에 피드백을 제공하기 위해 인라인 주석을 추가할 수 있습니다.

    프롬프트 상자에 `/plan`을 입력할 수도 있습니다. Claude Code v2.1.280 이상이 필요합니다.

    * `/plan`: 계획 모드로 전환합니다. 이미 계획 모드에 있으면 현재 계획을 대신 표시합니다.
    * `/plan`과 함께 작업(예: `/plan fix the auth bug`)을 입력합니다: 계획 모드로 전환하고 해당 작업 계획을 시작합니다.
    * `/plan open`: 이미 계획 모드에 있을 때 편집기에서 계획 파일을 엽니다.
  * **Edit automatically**: Claude가 묻지 않고 편집합니다.
* **모델**: 명령 메뉴에서 \*\*Switch model…\*\*을 선택하여 세션 중간에 모델을 변경합니다. 프롬프트 상자 하단의 모델 이름을 클릭하여 동일한 선택기를 열 수도 있습니다.

  현재 모델이 [노력 수준](/docs/ko/model-config#adjust-effort-level)을 지원하는 경우 선택기에 **Effort** 행도 표시되며 모델 이름 버튼은 선택된 수준을 표시합니다. `max` 이외의 수준을 선택하면 Claude Code는 사용자 설정의 [`modelSettings`](/docs/ko/settings-reference#modelsettings) 아래에서 현재 모델에 대한 기본값으로 저장합니다. `max`는 현재 세션에만 적용됩니다. 모델 이름 버튼과 **Effort** 행에는 Claude Code v2.1.257 이상이 필요합니다.
* **명령 메뉴**: `/`를 클릭하거나 `/`를 입력하여 명령 메뉴를 엽니다. 옵션에는 파일 첨부, 모델 전환, 확장 사고 토글이 포함됩니다.

  Customize 섹션은 MCP 서버, 명령, 출력 스타일, hooks, 메모리, 지시사항, 권한 및 플러그인과 같은 항목을 포함합니다. 터미널 아이콘이 있는 항목은 통합 터미널에서 열립니다.

  * slash 명령 `/usage` 또는 [`/remote-control`](/docs/ko/remote-control)과 같은 명령을 찾아보려면 Customize 섹션에서 **Slash commands**를 선택합니다. 대화 상자에 필터 상자가 있는 목록이 표시됩니다. 하나를 선택하여 실행합니다. 프롬프트 상자에서 `/`를 입력하면 여전히 명령을 인라인으로 제안합니다. Claude Code v2.1.257 이상이 필요합니다.

    또한 `/skills`를 입력하면 이 대화 상자가 열립니다. 각 [skill](/docs/ko/skills) 행은 **On** 또는 **Name only**와 같은 [가시성](/docs/ko/skills#override-skill-visibility-from-settings)을 표시합니다. 플러그인 skills와 같이 **locked**로 표시된 행을 제외하고 가시성을 클릭하여 변경합니다. `/skills` 바로가기 및 가시성 컨트롤에는 Claude Code v2.1.280 이상이 필요합니다.
  * Customize 섹션에서 **Output styles**를 선택하여 사용자 정의 스타일을 포함한 [출력 스타일](/docs/ko/output-styles)을 선택합니다. Claude Code v2.1.257 이상이 필요합니다.

    대신 사용자 정의 스타일을 만들려면 **Output styles** 메뉴에서 **Build a custom style**을 선택합니다. Claude Code는 프로젝트 또는 사용자 수준에서 [스타일 파일](/docs/ko/output-styles#create-a-custom-output-style)을 작성합니다. Claude Code v2.1.261 이상이 필요합니다.
  * Customize 섹션에서 **Hooks**를 선택하여 세션에 로드된 [hooks](/docs/ko/hooks)을 이벤트별로 그룹화하여 봅니다. 사용자, 프로젝트 및 로컬 설정 파일에 저장된 hooks를 추가, 편집 또는 제거할 수 있습니다. 관리되는 설정 또는 플러그인과 같은 다른 소스의 hooks는 읽기 전용입니다. Claude Code v2.1.269 이상이 필요합니다.
  * Customize 섹션에서 **Permissions**를 선택하여 세션의 [권한 규칙](/docs/ko/permissions)을 Allow, Ask, Deny로 그룹화하여 봅니다. 사용자, 프로젝트 또는 로컬 설정에 규칙을 추가하고 거기에 저장된 규칙을 제거할 수 있습니다. 관리되는 설정 또는 이 세션에만 적용되는 승인과 같은 다른 소스의 규칙은 읽기 전용입니다. Claude Code v2.1.269 이상이 필요합니다.
  * Customize 섹션에서 **Memory**를 선택하여 [auto memory](/docs/ko/memory#auto-memory)를 켜거나 끕니다. 켜져 있는 동안 Claude가 저장한 메모리를 찾아보고 파일 관리자에서 이들을 저장하는 폴더를 표시할 수도 있습니다. Claude Code v2.1.274 이상이 필요합니다.

    저장된 메모리를 클릭하여 대화 상자에서 읽고, 텍스트를 편집하거나, 메모리를 삭제하거나, 편집기에서 파일을 열 수 있습니다. 대화 상자에서 메모리를 보거나, 편집하거나, 삭제하려면 Claude Code v2.1.275 이상이 필요합니다.
  * Customize 섹션에서 **Instructions**를 선택하여 Claude가 읽는 [CLAUDE.md 파일](/docs/ko/memory#claude-md-files)을 편집합니다. 파일을 선택하여 편집기에서 엽니다. 파일이 아직 없으면 Claude Code가 먼저 생성합니다. Claude Code v2.1.274 이상이 필요합니다.
  * Customize 섹션에서 **Status**를 선택하거나 `/status`를 입력하여 세션의 Claude Code 버전, 계정, 모델 및 MCP 서버 세부 정보를 확인합니다. Claude Code v2.1.280 이상이 필요합니다.
  * Customize 섹션에서 **Sandbox**를 선택하거나 `/sandbox`를 입력하여 Claude의 Bash 명령이 [샌드박스](/docs/ko/sandboxing)에서 실행되는지 확인합니다. 샌드박스 모드를 전환하고 [제외된 명령](/docs/ko/settings-reference#sandbox-excludedcommands)을 추가할 수 있습니다. Claude Code v2.1.280 이상이 필요합니다.
  * Customize 섹션에서 **Claude in Chrome**을 선택하거나 `/chrome`을 입력하여 [Claude in Chrome](/docs/ko/chrome) 연결을 확인하고 관리합니다. 둘 다 claude.ai 계정으로 로그인해야 합니다. Claude Code v2.1.280 이상이 필요합니다.
  * Context 섹션에서 **Export conversation**을 선택하거나 `/export`를 입력하여 대화를 일반 텍스트로 복사하거나 파일에 저장합니다. `/export notes.txt`와 같은 파일 이름을 추가하여 대화 상자를 건너뛰고 파일을 저장할 위치를 선택합니다. Claude Code v2.1.280 이상이 필요합니다.
  * Settings 섹션에는 **Enable Remote Control for all sessions**이 포함되어 있으며, 이는 [`remoteControlAtStartup`](/docs/ko/settings-reference#remotecontrolatstartup)을 설정하여 [새 대화형 세션이 Remote Control에 자동으로 연결되는지](/docs/ko/remote-control#enable-remote-control-for-all-sessions) 제어합니다. Claude Code v2.1.203 이상이 필요합니다.

    VS Code 창에서 토글을 켜거나 끌 때 변경 사항은 그 이후에 시작하는 세션뿐만 아니라 해당 VS Code 창에서 이미 열려 있는 세션에 적용됩니다. 토글을 끄면 열려 있는 세션이 연결 해제됩니다. Claude Code v2.1.261 이상에서는 변경 사항이 다른 VS Code 창에서 열려 있는 세션에도 도달합니다.
  * Settings 섹션에는 또한 **Focus view**가 포함되어 있으며, 이는 도구 호출, 도구 결과 및 사고를 확장 가능한 행 뒤에 숨기고 프롬프트와 Claude의 응답을 남깁니다. 여기서 토글하거나, `Ctrl+Option+F` (Mac) / `Ctrl+Alt+F` (Windows/Linux)를 사용하거나, Command Palette에서 **Claude Code: Toggle Focus view**를 사용하여 토글합니다. 변경 사항은 모든 열려 있는 세션에 적용되고 세션 전체에서 유지됩니다. Claude Code v2.1.221 이상이 필요합니다.

    Claude의 최신 할 일 목록은 계속 표시되며, Claude가 묻는 보류 중인 질문의 텍스트도 표시됩니다. 이는 Claude Code v2.1.225 이상이 필요합니다. Claude가 [subagents](/docs/ko/sub-agents)를 실행하는 동안 최신 활동이 있는 라이브 진행 행이 이들을 시작한 도구 호출 그룹 아래에 나타납니다. Claude Code v2.1.269 이상이 필요합니다.
  * 계정에서 로그아웃하려면 Settings 섹션에서 **Sign out**을 선택하거나 `/logout`을 입력합니다. [타사 공급자](#use-third-party-providers)에서는 메뉴가 둘 다 제공하지 않습니다. Claude Code v2.1.277 이상이 필요합니다.
  * 버그를 보고하려면 메뉴 하단의 **Report a problem**을 클릭하거나 `/bug` 또는 `/feedback`을 입력하고 선택적으로 보고서를 미리 채우는 설명을 입력합니다. 보고서를 제출하고 1차 연결에서 Anthropic에 로그인한 경우 Claude Code는 이를 Anthropic에 보냅니다. 타사 공급자에서 또는 Anthropic 자격 증명 없이 대화 상자가 여전히 열리지만 제출하면 오류가 표시되고 아무것도 전송되지 않습니다. CLI의 `/bug`와 달리 확장 프로그램은 로컬 아카이브를 작성하지 않습니다. Claude Code v2.1.229 이상이 필요합니다.

    조직의 정책이 제품 피드백을 끄면 **Report a problem**이 메뉴에 나타나지 않으며, `/bug` 및 `/feedback`은 보고서를 열지 않고 `Feedback is turned off by your organization's policy or this environment's settings.` 공지를 표시합니다.
* **Side questions**: `/btw` 다음에 질문을 입력하여 [대화에 추가하지 않고](/docs/ko/interactive-mode#side-questions-with-%2Fbtw) 세션에 대해 질문합니다. 답변은 채팅 옆의 패널에서 열리며, 여기서 후속 질문을 할 수 있습니다. 스레드는 창 다시 로드를 유지합니다. Claude Code는 최신 20개 교환을 유지하고 Claude Code가 [안전하게 보존 기간을 결정](/docs/ko/claude-directory#cleaned-up-automatically)할 수 있는 한 [`cleanupPeriodDays`](/docs/ko/settings-reference#cleanupperioddays) 일정에 따라 저장된 스레드를 만료합니다. 스레드를 지우려면 패널의 휴지통 아이콘을 클릭합니다. Claude Code v2.1.227 이상이 필요합니다.
* **응답 복사**: 응답 위에 마우스를 올리고 **Copy response**를 클릭하여 클립보드에 복사하거나 `/copy`를 입력하여 최신 응답을 복사합니다. `/copy 2`는 두 번째 마지막 응답을 복사합니다. Claude Code v2.1.277 이상이 필요합니다.
* **Context indicator**: 프롬프트 상자는 Claude의 컨텍스트 윈도우를 얼마나 사용하고 있는지 보여줍니다. Claude는 필요할 때 자동으로 압축하거나 `/compact`를 수동으로 실행할 수 있습니다.
* **Prompt cache clock**: 컨텍스트 표시기 옆의 시계 아이콘은 대화의 [prompt cache](/docs/ko/prompt-caching)가 만료되기 전에 남은 시간을 추정합니다. 캐시의 5분 또는 1시간 [수명](/docs/ko/prompt-caching#cache-lifetime)에서 카운트다운되며, 캐시를 사용하는 각 응답은 카운트다운을 다시 시작합니다. 압축과 별개로 [캐시를 무효화하는 작업](/docs/ko/prompt-caching#actions-that-invalidate-the-cache)은 시계를 재설정하지 않으므로 모델을 전환한 후에도 남은 시간을 표시할 수 있습니다.
  * 카운트다운이 끝날 때까지 아이콘은 **12m**과 같이 남은 분을 표시합니다.
  * 카운트다운이 끝나면 분이 사라지고 아이콘이 빨간색 또는 테마의 오류 색상으로 변하며 다음 응답까지 유지됩니다. 캐시가 만료되었을 가능성이 높으므로 캐시가 다시 빌드되는 동안 다음 메시지에 대한 응답이 더 느리고 비용이 더 많이 들 것으로 예상합니다. 5분 수명이 메시지 사이에 계속 끝나면 [TTL을 직접 선택](/docs/ko/prompt-caching#choose-the-ttl-yourself)을 참조하세요.
  * 대화가 [압축](/docs/ko/prompt-caching#compacting-the-conversation)된 직후 아이콘도 다음 응답까지 분 없이 빨간색으로 변합니다. 캐시가 아직 압축된 대화를 포함하지 않기 때문입니다.
* **Agent map**: 대화에 [subagents](/docs/ko/sub-agents)가 포함되면 프롬프트 상자 하단에 **2 agents**와 같은 에이전트 수가 나타납니다. 점은 서브에이전트가 작동 중인지 또는 권한을 기다리고 있는지 여부를 보여줍니다.

  에이전트 맵을 열려면 에이전트 수를 클릭합니다. 에이전트 맵은 대화의 서브에이전트를 주 에이전트 아래의 트리로 그리며, 각각 상태, 경과 시간 및 토큰 수를 표시합니다. 서브에이전트를 클릭하여 프롬프트 및 도구 호출을 보거나, 읽기 전용 기록을 열거나, 실행 중일 때 중지합니다. Claude Code v2.1.269 이상이 필요합니다.

  맵은 또한 세션의 다른 [background tasks](/docs/ko/tools-reference#background-commands)(예: background shell 명령 및 [monitors](/docs/ko/tools-reference#monitor-tool))를 에이전트 아래에 나열합니다. 행을 클릭하여 작업의 카드를 열고 거기서 중지합니다.

  에이전트 수가 표시되지 않는 경우(예: Claude가 background shell을 시작했지만 subagents가 없는 경우) 에이전트 맵을 열려면 프롬프트 상자에 `/tasks`를 입력합니다. 맵의 background tasks 및 입력된 `/tasks`에는 Claude Code v2.1.277 이상이 필요합니다.
* **Extended thinking**: Claude가 복잡한 문제를 추론하는 데 더 많은 시간을 소비할 수 있게 합니다. 명령 메뉴(`/`)를 통해 토글합니다. Claude의 추론은 대화에 축소된 블록으로 나타납니다. 블록을 클릭하여 읽거나 `Ctrl+O`를 눌러 세션의 모든 사고 블록을 확장하거나 축소합니다. 자세한 내용은 [Extended thinking](/docs/ko/model-config#extended-thinking)을 참조하세요.
* **Multi-line input**: `Shift+Enter`를 눌러 보내지 않고 새 줄을 추가합니다. 이는 질문 대화의 "Other" 자유 텍스트 입력에서도 작동합니다.

<h3 id="reference-files-and-folders">
  참조 파일 및 폴더
</h3>

@-멘션을 사용하여 특정 파일 또는 폴더에 대한 컨텍스트를 Claude에 제공합니다. `@` 다음에 파일 또는 폴더 이름을 입력하면 Claude가 해당 콘텐츠를 읽고 이에 대해 질문하거나 변경할 수 있습니다. Claude Code는 퍼지 매칭을 지원하므로 부분 이름을 입력하여 필요한 항목을 찾을 수 있습니다:

```text wrap theme={null}
Explain the logic in @auth (fuzzy matches auth.js, AuthService.ts, etc.)
What's in @src/components/ (include a trailing slash for folders)
```

큰 PDF의 경우 전체 파일 대신 특정 페이지를 읽도록 Claude에 요청할 수 있습니다. 단일 페이지, 1-10페이지와 같은 범위 또는 3페이지 이상과 같은 개방형 범위입니다.

편집기에서 텍스트를 선택하면 Claude가 강조 표시된 코드를 자동으로 볼 수 있습니다. 프롬프트 상자 바닥글은 선택된 줄 수를 표시합니다. `Option+K` (Mac) / `Alt+K` (Windows/Linux)를 눌러 파일 경로 및 줄 번호가 있는 @-멘션을 삽입합니다 (예: `@app.ts#5-10`). 선택 표시기의 **X**를 클릭하여 Claude가 선택 항목을 받지 않도록 제거합니다. 다른 텍스트를 선택하면 표시기가 다시 나타납니다.

확장 프로그램은 일부 파일에서 선택된 텍스트를 보류합니다. 파일이 작업 영역 내에 있고 `files.exclude` 또는 `search.exclude` 설정과 일치하는 경우 Claude는 최대 파일의 경로를 받고 선택한 텍스트는 받지 않습니다. VS Code의 `search.useIgnoreFiles` 설정과 확장 프로그램의 [`respectGitIgnore` 설정](#extension-settings)이 모두 켜져 있는 한 git이 무시하는 파일에도 동일하게 적용되며, 이는 기본값입니다. 이 필터는 채팅 패널만 포함합니다. Claude Code가 통합 터미널에서 실행될 때 CLI는 파일이 무엇이든 선택한 텍스트를 보내므로 [`Read` deny 규칙](#the-built-in-ide-mcp-server)을 추가하여 파일의 내용이 Claude에 도달하지 않도록 합니다.

Claude도 편집기에서 열려 있는 파일을 보며, 아무것도 선택되지 않은 경우에도 프롬프트 상자에 이름이 표시됩니다. 선택한 텍스트만 추가하려면 [Attach Open File 설정](vscode://settings/claudeCode.attachOpenFile)을 끕니다. 설정에는 Claude Code v2.1.271 이상이 필요합니다.

메시지에 이미지 및 파일을 첨부할 수도 있습니다:

* 이미지를 첨부하려면 클립보드에서 프롬프트 상자에 붙여넣습니다.
* 파일을 첨부하려면 프롬프트 상자로 드래그할 때 `Shift`를 누르고 있습니다.
* 컨텍스트에서 첨부 파일을 제거하려면 X를 클릭합니다.

<h3 id="paste-text">
  텍스트 붙여넣기
</h3>

붙여넣은 텍스트는 프롬프트 상자에 표시된 상태로 유지되며, [터미널](/docs/ko/terminal-config#paste-large-content)에서처럼 자리 표시자로 축소되지 않습니다. Claude Code가 [붙여넣은 텍스트를 표시](/docs/ko/terminal-config#how-claude-treats-pasted-text)하는 세션에서 Claude는 여전히 큰 붙여넣기를 입력한 텍스트가 아닌 붙여넣은 텍스트로 봅니다.

Claude Code는 또한 프롬프트 상자에 붙여넣은 텍스트 및 보내는 다른 모든 항목에서 [보이지 않는 Unicode 문자](/docs/ko/interactive-mode#invisible-characters-in-prompts)를 제거합니다:

* 붙여넣을 때 `Removed 3 invisible characters from the pasted text`와 같은 공지가 나타나면 텍스트가 해당 문자 없이 들어갔습니다.
* 보낼 때 제거된 문자에 대한 공지가 나타나면 아무것도 전송되지 않았습니다. 정리된 텍스트가 프롬프트 상자에 다시 있습니다. 다시 보내서 표시된 대로 텍스트를 보냅니다.

<h3 id="resume-past-conversations">
  과거 대화 재개
</h3>

Claude Code 패널 상단의 **Session history** 버튼을 클릭하여 대화 기록에 액세스합니다. 키워드로 검색하거나 시간별로 찾아볼 수 있습니다.

모든 대화를 클릭하여 전체 메시지 기록으로 재개합니다. 세션이 현재 창의 다른 탭에서 이미 열려 있으면 클릭하면 해당 탭으로 전환됩니다. 세션 재개에 대한 자세한 내용은 [Manage sessions](/docs/ko/sessions)를 참조하세요.

* **Session titles**: 새 세션은 첫 번째 메시지를 기반으로 AI가 생성한 제목을 받습니다.
* **Rename and archive**: 세션 위에 마우스를 올려 이러한 작업을 표시합니다. 설명적인 제목을 지정하도록 이름을 바꾸거나 목록 하단의 **Archived sessions** 그룹으로 이동하도록 보관합니다.

기본적으로 14일 동안 활동이 없는 세션은 자동으로 **Archived sessions**으로 이동합니다. 단, 열려 있거나, 읽지 않은 상태이거나, [그룹](#organize-sessions-into-groups)에 속한 세션은 제외됩니다. 자동 보관에는 Claude Code v2.1.265 이상이 필요합니다. 기간을 변경하거나 끄려면 [Archive Inactive Sessions 설정](vscode://settings/claudeCode.archiveInactiveSessions)을 열고 일 수 또는 **Never**를 선택합니다.

보관된 세션을 복원하려면 **Archived sessions**을 확장하고 **Unarchive session**을 클릭합니다. 보관된 모든 세션을 한 번에 복원하려면 Activity Bar의 세션 목록에서 **Archived sessions** 헤더 위에 마우스를 올리고 해당 보관 해제 아이콘을 클릭합니다. Claude Code v2.1.277 이상이 필요합니다. v2.1.257 이전에는 작업이 **Delete session**이었으며, 이는 복원할 방법이 없는 세션을 숨겼습니다. 그 후 삭제한 세션은 업그레이드 후 **Archived sessions** 아래에 나타납니다.

재개한 대화가 계획 모드에서 끝난 경우 Claude Code는 계획 모드를 복원합니다. Claude Code v2.1.246 이상이 필요합니다. Claude Code는 두 가지 경우에 복원하지 않습니다:

* 확장 프로그램이 `claudeCode.initialPermissionMode`에서 또는 이전 대화에서 이월되는 선택에서 [시작 권한 모드를 선택](/docs/ko/permission-modes#switch-permission-modes)합니다
* `claudeCode.claudeProcessWrapper`가 구성되어 있습니다

<h3 id="resume-cloud-sessions-from-claude-ai">
  Claude.ai에서 클라우드 세션 재개
</h3>

[웹에서 Claude Code](/docs/ko/claude-code-on-the-web)를 사용하는 경우 VS Code에서 직접 해당 클라우드 세션을 재개할 수 있습니다. 이는 Anthropic Console이 아닌 **Claude.ai Subscription**으로 로그인해야 합니다.

<Steps>
  <Step title="Open session history">
    Claude Code 패널 상단의 **Session history** 버튼을 클릭합니다.
  </Step>

  <Step title="Select the Web tab">
    대화 상자에 두 개의 탭이 표시됩니다: Local과 Web. **Web**을 클릭하여 claude.ai의 세션을 봅니다.
  </Step>

  <Step title="Select a session to resume">
    클라우드 세션을 찾아보거나 검색합니다. 모든 세션을 클릭하여 다운로드하고 로컬에서 대화를 계속합니다.
  </Step>
</Steps>

<Note>
  GitHub 저장소로 시작한 웹 세션만 Web 탭에 나타납니다. 재개하면 대화 기록이 로컬로 로드되며, 변경 사항은 claude.ai로 다시 동기화되지 않습니다.
</Note>

<h3 id="check-account-and-usage">
  계정 및 사용량 확인
</h3>

`/usage`를 실행하여 Account & usage 대화를 엽니다. 로그인한 계정을 표시하며, 보고하는 사용량은 로그인에 따라 다릅니다:

* **claude.ai plan**: 현재 세션 및 주간과 같은 플랜의 제한에 대한 사용량 막대입니다. 각 막대는 제한이 재설정될 때까지의 시간을 표시합니다.

  대화는 또한 플랜 제한에 기여하는 항목을 분류합니다. 캐시 미스, 긴 컨텍스트, 서브에이전트 집약적 또는 고도로 병렬 세션과 같이 최근 사용량의 10% 이상을 차지하는 동작에 플래그를 지정하며, 각각 이를 줄이기 위한 팁이 있습니다. Attribution 테이블은 각 skill, subagent, 플러그인 및 MCP 서버에서 얼마나 많은 사용량이 나왔는지 보여줍니다.

  Day와 Week 토글을 사용하여 지난 24시간과 지난 7일 사이를 전환합니다. 수치는 대략적이며 이 컴퓨터의 로컬 세션에서 계산되므로 다른 장치 또는 claude.ai의 사용량은 포함되지 않습니다.
* **Other sign-ins**: [타사 공급자](#use-third-party-providers)에서 또는 API 키를 사용하는 경우와 같이 플랜 제한이 로그인에 적용되지 않는 경우 Usage 섹션은 세션 자체의 비용 및 토큰 사용량을 대신 표시합니다. CLI의 `/usage`는 [Session block](/docs/ko/costs#track-your-costs)에서 동일한 합계를 표시합니다. Activity Bar의 세션 목록은 또한 활성 세션의 합계를 **Account & usage** 헤더 아래에 표시합니다. Claude Code v2.1.277 이상이 필요합니다.

사용량 추적 및 감소에 대한 자세한 내용은 [Track your costs](/docs/ko/costs#track-your-costs)를 참조하세요.

<h2 id="customize-your-workflow">
  워크플로우 사용자 정의
</h2>

Claude 패널의 위치를 변경하고, 여러 대화를 실행하며, 세션 목록을 그룹으로 정렬하거나 터미널 모드로 전환할 수 있습니다.

<h3 id="choose-where-claude-lives">
  Claude가 위치할 곳 선택
</h3>

Claude 패널을 드래그하여 VS Code의 어느 곳이든 위치를 변경할 수 있습니다. 패널의 탭이나 제목 표시줄을 잡고 다음 위치로 드래그합니다:

* **보조 사이드바**: 창의 오른쪽입니다. 코드를 작성하는 동안 Claude를 계속 볼 수 있습니다.
* **주 사이드바**: 탐색기, 검색 등의 아이콘이 있는 왼쪽 사이드바입니다.
* **편집기 영역**: Claude를 파일과 함께 탭으로 엽니다. 부수적인 작업에 유용합니다.

Claude가 새 편집기 그룹에서 탭을 열 때, 확장 프로그램은 해당 그룹을 잠금으로써 Claude 탭이 포커스되어 있는 동안 열린 파일이 그 옆이 아닌 다른 그룹으로 이동합니다.

확장 프로그램이 그룹을 잠금으로써 방지하려면 [편집기 그룹 잠금 설정](vscode://settings/claudeCode.lockEditorGroups)을 끕니다. 이미 잠긴 그룹은 잠금을 해제할 때까지 잠긴 상태로 유지됩니다. 이 설정은 Claude Code v2.1.274 이상이 필요합니다.

<Tip>
  주 Claude 세션에는 사이드바를 사용하고 부수적인 작업을 위해 추가 탭을 엽니다. Claude는 선호하는 위치를 기억합니다. 활동 표시줄 세션 목록 아이콘은 Claude 패널과 별개입니다: 세션 목록은 항상 활동 표시줄에 표시되지만, Claude 패널 아이콘은 패널이 왼쪽 사이드바에 도킹되어 있을 때만 표시됩니다.
</Tip>

**Developer: Reload Window**를 실행하거나 VS Code를 다시 시작한 후, 채팅이 대화와 함께 돌아오는지 여부는 채팅이 열려 있던 위치에 따라 달라집니다:

* **편집기 탭**: 대화가 탭과 함께 돌아옵니다.
* **사이드바**: 지난 10분 이내에 메시지를 보냈거나 Claude가 응답한 경우 대화가 돌아옵니다. 돌아오지 않으면 [세션 기록](#resume-past-conversations)에서 대화를 재개합니다.

다시 로드가 Claude의 단계 중간에 중단된 경우, 대화가 돌아올 때 Claude는 해당 단계를 계속하며, 채팅의 공지사항이 계속을 표시합니다. Claude Code v2.1.274 이상이 필요합니다. 단계가 1시간 이상 전에 중단되었거나 세션이 다른 곳에서 열려 있으면, 대화가 유휴 상태로 돌아옵니다.

계속을 끄려면 [다시 로드 후 계속 설정](vscode://settings/claudeCode.continueAfterReload)을 열고 선택을 해제합니다.

<h3 id="run-multiple-conversations">
  여러 대화 실행
</h3>

명령 팔레트에서 **새 탭에서 열기** 또는 **새 창에서 열기**를 사용하여 추가 대화를 시작합니다. 각 대화는 자체 기록과 컨텍스트를 유지하므로 여러 작업을 병렬로 진행할 수 있습니다.

탭을 사용할 때, 스파크 아이콘의 작은 색상 점은 상태를 나타냅니다: 파란색은 권한 요청이 대기 중임을 의미하고, 주황색은 탭이 숨겨진 동안 Claude가 완료되었음을 의미합니다.

<h3 id="organize-sessions-into-groups">
  세션을 그룹으로 정렬
</h3>

활동 표시줄의 세션 목록에서 관련 세션을 명명된 축소 가능한 그룹으로 수집할 수 있습니다. Claude Code v2.1.229 이상이 필요합니다.

* **세션 그룹화 또는 그룹 해제**: 세션을 마우스 오른쪽 버튼으로 클릭하여 그룹을 만들거나, 기존 그룹으로 이동하거나, 그룹에서 제거합니다. 각 세션은 한 번에 하나의 그룹에만 속하므로, 다른 그룹으로 이동하면 첫 번째 그룹에서 제거됩니다.
* **여러 세션을 한 번에 이동**: `Cmd`-클릭(Mac) / `Ctrl`-클릭(Windows/Linux)으로 각 세션을 선택하거나, `Shift`-클릭으로 범위를 선택한 후 선택 항목을 마우스 오른쪽 버튼으로 클릭합니다.
* **탭에서 세션 그룹화**: 명령 팔레트에서 **Claude Code: Add Session Tab to Group**을 실행한 후 그룹을 선택하거나 만듭니다. Claude Code v2.1.257 이상이 필요합니다.
* **그룹 이름 바꾸기 또는 삭제**: 그룹 헤더를 마우스 오른쪽 버튼으로 클릭합니다. 그룹을 삭제하면 그룹만 제거되고 해당 세션은 그룹화되지 않은 목록으로 돌아갑니다.

확장 프로그램은 작업 영역 폴더별로 그룹을 저장하므로, 창을 다시 로드해도 유지되며 같은 폴더를 여는 모든 창에 표시됩니다. 목록을 검색할 때 확장 프로그램은 모든 그룹에서 일치하는 항목을 하나의 평면 목록으로 표시합니다.

<h3 id="switch-to-terminal-mode">
  터미널 모드로 전환
</h3>

기본적으로 확장 프로그램은 그래픽 채팅 패널을 엽니다. CLI 스타일 인터페이스를 선호하는 경우 [터미널 사용 설정](vscode://settings/claudeCode.useTerminal)을 열고 확인란을 선택합니다.

VS Code 설정(`Mac에서 Cmd+,` 또는 `Windows/Linux에서 Ctrl+,`)을 열고, 확장 프로그램 → Claude Code로 이동한 후 **터미널 사용**을 선택할 수도 있습니다.

<h2 id="manage-plugins">
  플러그인 관리
</h2>

VS Code 확장 프로그램에는 [플러그인](/docs/ko/plugins/overview)을 설치하고 관리하기 위한 그래픽 인터페이스가 포함되어 있습니다. 프롬프트 상자에 `/plugins`를 입력하여 **플러그인 관리** 인터페이스를 엽니다.

<h3 id="install-plugins">
  플러그인 설치
</h3>

플러그인 대화 상자에는 **플러그인** 및 **마켓플레이스** 두 개의 탭이 표시됩니다.

플러그인 탭에서:

* **설치된 플러그인**은 상단에 표시되며 토글 스위치로 활성화 또는 비활성화할 수 있습니다
* **사용 가능한 플러그인**은 구성된 마켓플레이스에서 아래에 표시됩니다
* 이름 또는 설명으로 플러그인을 필터링하려면 검색을 사용합니다
* 사용 가능한 플러그인에서 **설치**를 클릭합니다

플러그인을 설치할 때 설치 범위를 선택합니다:

* **사용자용으로 설치**: 모든 프로젝트에서 사용 가능(사용자 범위)
* **이 프로젝트용으로 설치**: 프로젝트 협력자와 공유(프로젝트 범위)
* **로컬로 설치**: 이 저장소에서만 사용자용(로컬 범위)

<h3 id="share-a-plugin-install-link">
  플러그인 설치 링크 공유
</h3>

특정 플러그인을 설치하도록 누군가를 직접 보내려면 확장 프로그램의 `install-plugin` URL을 제공합니다. 이를 열면 VS Code를 시작하거나 포커스하고, Claude Code 패널을 열고, **플러그인 관리** 대화 상자를 해당 플러그인의 범위 선택에서 엽니다. 사용자가 범위를 선택할 때까지 아무것도 설치되지 않습니다. 플러그인의 마켓플레이스가 Claude Code에서 아직 구성되지 않은 경우 대화 상자는 먼저 이를 추가하도록 요청합니다.

```text theme={null}
vscode://anthropic.claude-code/install-plugin?plugin=code-review&marketplace=anthropics/claude-plugins-official
```

URL은 두 개의 쿼리 매개변수를 사용합니다:

| 매개변수          | 설명                                                                                                                                                            |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plugin`      | 마켓플레이스에 나열된 플러그인의 이름입니다. 필수입니다.                                                                                                                               |
| `marketplace` | 플러그인의 출처이며, GitHub `owner/repo`, `https://` URL 또는 `git@github.com:owner/repo.git`과 같은 git SSH URL입니다. 생략하면 `anthropics/claude-plugins-official`로 기본값이 설정됩니다. |

[마켓플레이스 탭](#manage-marketplaces)이 허용하는 일부 값(예: 로컬 경로 또는 `http://` 주소)은 링크에서 작동하지 않습니다. 이러한 경우 VS Code에서 오류 메시지를 표시하고 대화 상자가 열리지 않습니다.

두 가지 경우는 범위 선택 대신 대화 상자의 메시지에서 끝납니다:

* **마켓플레이스에 해당 이름의 플러그인이 없음**: 대화 상자에서 플러그인을 찾을 수 없다고 보고합니다. 마켓플레이스 목록에 대해 `plugin` 값을 확인합니다.
* **플러그인이 이미 설치됨**: 대화 상자에서 이를 표시하며 아무것도 변경되지 않습니다.

GitHub README, 이슈 및 일부 다른 Markdown 호스트는 스키마가 `http` 또는 `https`가 아닌 링크를 제거하므로 `vscode://` 링크는 일반 텍스트로 렌더링됩니다. [링크가 클릭 가능한 대신 일반 텍스트로 렌더링됨](/docs/ko/deep-links#the-link-renders-as-plain-text-instead-of-being-clickable)에서 `claude-cli://` 링크에 대해 설명하는 대로 이러한 호스트에서 URL을 코드 블록에 넣습니다.

<h3 id="manage-marketplaces">
  마켓플레이스 관리
</h3>

**마켓플레이스** 탭으로 전환하여 플러그인 소스를 추가하거나 제거합니다:

* GitHub 저장소, URL 또는 로컬 경로를 입력하여 새 마켓플레이스를 추가합니다
* 새로고침 아이콘을 클릭하여 마켓플레이스의 플러그인 목록을 업데이트합니다
* 휴지통 아이콘을 클릭하여 마켓플레이스를 제거합니다

대화 상자에서 변경한 플러그인 변경 사항은 해당 VS Code 창에서 열려 있는 Claude Code 세션에 즉시 적용됩니다. 대화 상자를 연 세션이 플러그인을 다시 로드할 수 없는 경우 대화 상자에서 다시 시도하거나 해당 세션에서 Claude를 다시 시작하도록 제안합니다.

<Note>
  VS Code의 플러그인 관리는 내부적으로 동일한 CLI 명령을 사용합니다. 확장 프로그램에서 구성한 플러그인 및 마켓플레이스는 CLI에서도 사용 가능하며, 그 반대도 마찬가지입니다.
</Note>

플러그인 시스템에 대한 자세한 내용은 [플러그인](/docs/ko/plugins/overview) 및 [플러그인 마켓플레이스](/docs/ko/plugins/overview)를 참조하십시오.

<h2 id="automate-browser-tasks-with-chrome">
  Chrome으로 브라우저 작업 자동화
</h2>

Claude를 Chrome 브라우저에 연결하여 웹 앱을 테스트하고, 콘솔 로그로 디버깅하며, VS Code를 떠나지 않고 브라우저 워크플로우를 자동화합니다. 이를 위해서는 [Claude in Chrome 확장 프로그램](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) 버전 1.0.36 이상이 필요합니다.

프롬프트 상자에 `@browser`를 입력한 후 Claude가 수행할 작업을 입력합니다:

```text wrap theme={null}
@browser go to localhost:3000 and check the console for errors
```

첨부 메뉴를 열어 새 탭 열기 또는 페이지 콘텐츠 읽기와 같은 특정 브라우저 도구를 선택할 수도 있습니다.

Claude는 브라우저 작업을 위해 새 탭을 열고 브라우저의 로그인 상태를 공유하므로, 이미 로그인한 모든 사이트에 액세스할 수 있습니다.

설정 지침, 전체 기능 목록 및 문제 해결에 대해서는 [Claude Code를 Chrome과 함께 사용](/docs/ko/chrome)을 참조합니다.

<h2 id="vs-code-commands-and-shortcuts">
  VS Code 명령어 및 단축키
</h2>

명령 팔레트를 열고(`Mac에서 Cmd+Shift+P` 또는 `Windows/Linux에서 Ctrl+Shift+P`) "Claude Code"를 입력하면 Claude Code 확장 프로그램의 모든 사용 가능한 VS Code 명령어를 볼 수 있습니다.

일부 단축키는 어느 패널이 "포커스"되어 있는지(키보드 입력을 받고 있는지)에 따라 달라집니다. 커서가 코드 파일에 있으면 편집기가 포커스됩니다. 커서가 Claude의 프롬프트 상자에 있으면 Claude가 포커스됩니다. `Cmd+Esc` / `Ctrl+Esc`를 사용하여 둘 사이를 전환합니다.

<Note>
  이는 확장 프로그램을 제어하기 위한 VS Code 명령어입니다. 모든 기본 제공 Claude Code 명령어가 확장 프로그램에서 사용 가능한 것은 아닙니다. 자세한 내용은 [VS Code 확장 프로그램 vs. Claude Code CLI](#vs-code-extension-vs-claude-code-cli)를 참조하십시오.
</Note>

| 명령어                        | 단축키                                                      | 설명                                                                                                                                                        |
| -------------------------- | -------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Focus Input                | `Cmd+Esc` (Mac) / `Ctrl+Esc` (Windows/Linux)             | 편집기와 Claude 사이의 포커스 전환                                                                                                                                    |
| Focus last message         | -                                                        | 대화에서 가장 최신 메시지로 키보드 포커스를 이동하거나 대기 중인 권한 프롬프트로 이동하여 키보드 또는 화면 판독기로 읽을 수 있습니다. [터미널 모드](#switch-to-terminal-mode)에서는 사용할 수 없습니다. Claude Code v2.1.268 이상 필요 |
| Open in Side Bar           | -                                                        | Claude를 사이드바에서 열기                                                                                                                                         |
| Open in Terminal           | -                                                        | Claude를 터미널 모드에서 열기                                                                                                                                       |
| Open in New Tab            | `Cmd+Shift+Esc` (Mac) / `Ctrl+Shift+Esc` (Windows/Linux) | 새 대화를 편집기 탭으로 열기                                                                                                                                          |
| Open in New Window         | -                                                        | 새 대화를 별도 창에서 열기                                                                                                                                           |
| New Conversation           | `Cmd+N` (Mac) / `Ctrl+N` (Windows/Linux)                 | 새 대화 시작. Claude가 포커스되어 있고 `enableNewConversationShortcut`이 `true`로 설정되어 있어야 함                                                                             |
| Reopen Closed Session      | `Cmd+Shift+T` (Mac) / `Ctrl+Shift+T` (Windows/Linux)     | 가장 최근에 닫은 Claude 세션 탭을 다시 열기. 마지막으로 닫은 탭이 Claude 세션이 아닌 경우 VS Code의 일반 닫힌 편집기 다시 열기로 폴스루됨. `enableReopenClosedSessionShortcut`으로 비활성화                     |
| Insert @-Mention Reference | `Option+K` (Mac) / `Alt+K` (Windows/Linux)               | 현재 파일 및 선택 항목에 대한 참조 삽입(편집기가 포커스되어 있어야 함)                                                                                                                 |
| Accept Change at Cursor    | -                                                        | [제안된 편집 검토](#get-started) 중 한 번에 하나씩 변경 사항을 수락합니다. Claude Code v2.1.275 이상 필요                                                                             |
| Reject Change at Cursor    | -                                                        | 제안된 편집을 검토하는 동안 한 번에 하나씩 변경 사항을 되돌립니다. Claude Code v2.1.275 이상 필요                                                                                         |
| Toggle Focus view          | `Ctrl+Option+F` (Mac) / `Ctrl+Alt+F` (Windows/Linux)     | 대화에서 도구 활동 숨기기 또는 표시. Claude 패널 또는 사이드바가 표시되는 동안 작동. Claude Code v2.1.221 이상 필요                                                                           |
| Rename Session Tab         | -                                                        | 활성 Claude 탭의 세션 이름 바꾸기. Claude Code v2.1.257 이상 필요                                                                                                        |
| Add Session Tab to Group   | -                                                        | 활성 Claude 탭의 세션을 선택하거나 만드는 [세션 그룹](#organize-sessions-into-groups)에 추가. Claude Code v2.1.257 이상 필요                                                        |
| Mark Session as Unread     | -                                                        | 활성 Claude 탭의 세션을 세션 목록에서 읽지 않음으로 표시. Claude Code v2.1.257 이상 필요                                                                                           |
| Show Logs                  | -                                                        | 확장 프로그램 디버그 로그 보기                                                                                                                                         |
| Logout                     | -                                                        | Anthropic 계정에서 로그아웃                                                                                                                                       |

<h3 id="launch-a-vs-code-tab-from-other-tools">
  다른 도구에서 VS Code 탭 시작
</h3>

확장 프로그램은 `vscode://anthropic.claude-code/open`에서 URI 핸들러를 등록합니다. 이를 사용하여 자신의 도구(셸 별칭, 브라우저 북마클릿 또는 URL을 열 수 있는 모든 스크립트)에서 새 Claude Code 탭을 열 수 있습니다. VS Code가 아직 실행 중이 아니면 URL을 열면 먼저 실행됩니다. VS Code가 이미 실행 중이면 URL은 현재 포커스된 창에서 열립니다.

운영 체제의 URL 오프너로 핸들러를 호출합니다.

<Tabs>
  <Tab title="macOS">
    ```bash theme={null}
    open "vscode://anthropic.claude-code/open"
    ```
  </Tab>

  <Tab title="Linux">
    ```bash theme={null}
    xdg-open "vscode://anthropic.claude-code/open"
    ```

    `xdg-open` 명령어는 `xdg-utils` 패키지에서 제공됩니다. 셸에서 찾을 수 없다고 보고하면 [xdg-open is not found on Linux](/docs/ko/deep-links#xdg-open-is-not-found-on-linux)를 참조하십시오.
  </Tab>

  <Tab title="Windows">
    PowerShell에서:

    ```powershell theme={null}
    Start-Process "vscode://anthropic.claude-code/open"
    ```

    `cmd.exe`에서 `start`는 첫 번째 따옴표로 묶인 인수를 창 제목으로 취급하므로 URL 앞에 빈 제목을 전달합니다:

    ```cmd theme={null}
    start "" "vscode://anthropic.claude-code/open"
    ```
  </Tab>
</Tabs>

핸들러는 두 개의 선택적 쿼리 매개변수를 허용합니다:

| 매개변수      | 설명                                                                                                                                                                                                        |
| --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prompt`  | 프롬프트 상자에 미리 채울 텍스트. URL 인코딩되어야 합니다. 프롬프트는 미리 채워지지만 자동으로 제출되지 않습니다.                                                                                                                                        |
| `session` | 새 대화를 시작하는 대신 재개할 세션 ID. 세션은 VS Code에서 현재 열려 있는 작업 영역에 속해야 합니다. 세션을 찾을 수 없으면 새 대화가 시작됩니다. 세션이 이미 탭에서 열려 있으면 해당 탭이 포커스됩니다. 프로그래밍 방식으로 세션 ID를 캡처하려면 [대화 계속하기](/docs/ko/headless#continue-conversations)를 참조하십시오. |

예를 들어 "review my changes"로 미리 채워진 탭을 열려면:

```text theme={null}
vscode://anthropic.claude-code/open?prompt=review%20my%20changes
```

확장 프로그램은 또한 `vscode://anthropic.claude-code/install-plugin`을 처리하며, 이는 [한 플러그인에서 플러그인 대화를 엽니다](#share-a-plugin-install-link). VS Code 탭 대신 터미널 세션을 시작하려면 CLI의 `claude-cli://` 핸들러를 사용합니다. [링크에서 세션 시작하기](/docs/ko/deep-links)를 참조하십시오.

<h2 id="configure-settings">
  설정 구성
</h2>

확장 프로그램에는 두 가지 유형의 설정이 있습니다:

* **VS Code의 확장 프로그램 설정**: VS Code 내에서 확장 프로그램의 동작을 제어합니다. `Cmd+,`(Mac) 또는 `Ctrl+,`(Windows/Linux)로 열고, Extensions → Claude Code로 이동합니다. `/`를 입력하고 \*\*General config…\*\*를 선택하여 설정을 열 수도 있습니다.
* **`~/.claude/settings.json`의 Claude Code 설정**: 확장 프로그램과 CLI 간에 공유됩니다. 허용된 명령, 환경 변수, hooks 및 MCP 서버에 사용합니다. Pro, Max 및 Team 플랜에서는 권한 모드 대화가 시작되는 입력 중 하나이기도 합니다. [권한 모드 전환](/docs/ko/permission-modes#switch-permission-modes)에서 순서를 나열합니다. 자세한 내용은 [설정](/docs/ko/settings)을 참조하세요.

<Tip>
  `settings.json`에 `"$schema": "https://json.schemastore.org/claude-code-settings.json"`을 추가하여 VS Code에서 직접 사용 가능한 모든 설정에 대한 자동 완성 및 인라인 유효성 검사를 받습니다.
</Tip>

<h3 id="extension-settings">
  확장 프로그램 설정
</h3>

VS Code는 사용자 설정에서 `initialPermissionMode`를 읽고 작업 영역 값을 무시합니다. v2.1.225 이전에는 VS Code가 설정을 기본값 `default`로 설정하고 작업 영역 값을 적용했습니다.

| 설정                                  | 기본값     | 설명                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| ----------------------------------- | ------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `useTerminal`                       | `false` | 그래픽 패널 대신 터미널 모드에서 Claude를 실행합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `initialPermissionMode`             | -       | 새 대화에 대한 승인 프롬프트를 제어합니다: `default`, `plan`, `acceptEdits` 또는 `bypassPermissions`. `manual`은 `default`의 별칭이며 모드 표시기에서 **Manual**로 표시된 모드를 선택합니다. 설정을 해제하면 확장 프로그램이 [권한 모드 전환](/docs/ko/permission-modes#switch-permission-modes)에 설명된 대로 시작 권한 모드를 선택합니다.                                                                                                                                                                                                                                                                        |
| `preferredLocation`                 | `panel` | Claude가 열리는 위치: `sidebar`(오른쪽) 또는 `panel`(새 탭)                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `lockEditorGroups`                  | `true`  | [Claude가 탭을 위해 시작하는 편집기 그룹을 잠금](#choose-where-claude-lives)하여 Claude 탭이 포커스되어 있는 동안 열려 있는 파일이 다른 그룹으로 이동합니다. 꺼져 있으면 확장 프로그램이 편집기 그룹을 잠금하지 않습니다. Claude Code v2.1.274 이상 필요                                                                                                                                                                                                                                                                                                                                                 |
| `autosave`                          | `true`  | Claude가 파일을 읽거나 쓰기 전에 파일을 자동 저장합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `attachOpenFile`                    | `true`  | 편집기에서 열려 있는 파일을 메시지에 추가하고 프롬프트 상자에 표시합니다. 꺼져 있으면 선택한 텍스트만 추가됩니다. Claude Code v2.1.271 이상 필요                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `useCtrlEnterToSend`                | `false` | Enter 대신 Ctrl/Cmd+Enter를 사용하여 프롬프트를 보냅니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `scrollToBottomOnSend`              | `true`  | 메시지를 보낼 때 대화를 맨 아래로 스크롤합니다. 꺼져 있으면 대화가 남겨진 위치에 머물러 있습니다. Claude Code v2.1.275 이상 필요                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `enableNewConversationShortcut`     | `false` | Cmd/Ctrl+N을 활성화하여 새 대화를 시작합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `enableReopenClosedSessionShortcut` | `true`  | Cmd/Ctrl+Shift+T를 사용하여 가장 최근에 닫은 Claude 세션 탭을 다시 엽니다. 마지막으로 닫은 탭이 Claude 세션이 아닌 경우 바로 가기는 VS Code의 일반 reopen-closed-editor 명령을 대신 실행합니다.                                                                                                                                                                                                                                                                                                                                                                                   |
| `archiveInactiveSessions`           | `14`    | [세션을 자동으로 보관](#resume-past-conversations)합니다. 이 많은 일 동안 활동이 없으면: `1`, `2`, `7` 또는 `14`. `0`으로 설정하여 끕니다. Claude Code v2.1.265 이상 필요                                                                                                                                                                                                                                                                                                                                                                                         |
| `continueAfterReload`               | `true`  | 창 다시 로드 후 Claude는 [복원된 세션에서 중단된 단계를 계속](#choose-where-claude-lives)합니다. Claude Code v2.1.274 이상 필요                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `hideOnboarding`                    | `false` | 온보딩 체크리스트(졸업 모자 아이콘) 숨기기                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `focusView`                         | `false` | 도구 호출, 도구 결과 및 생각을 확장 가능한 행 뒤에 숨기고 프롬프트와 Claude의 응답을 남깁니다. Claude의 최신 할 일 목록은 계속 표시됩니다. 이는 Claude Code v2.1.225 이상이 필요합니다. 명령 메뉴에서 포커스 보기를 전환할 수도 있습니다. Claude Code v2.1.221 이상 필요                                                                                                                                                                                                                                                                                                                                         |
| `respectGitIgnore`                  | `true`  | 파일 검색 및 [선택 컨텍스트](#reference-files-and-folders)에서 .gitignore 패턴 제외                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `usePythonEnvironment`              | `true`  | Claude를 실행할 때 작업 영역의 Python 환경을 활성화합니다. Python 확장 프로그램이 필요합니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `environmentVariables`              | `[]`    | Claude 프로세스에 대한 환경 변수를 설정합니다. 공유 구성의 경우 Claude Code 설정을 대신 사용합니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `disableLoginPrompt`                | `false` | 인증 프롬프트 건너뛰기(타사 공급자 설정의 경우)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `allowDangerouslySkipPermissions`   | `false` | 모드 선택기에 권한 무시를 추가합니다. 인터넷 접근이 없는 샌드박스에서만 사용합니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `claudeProcessWrapper`              | -       | Claude 프로세스를 실행하는 데 사용되는 실행 파일입니다. 번들된 바이너리 경로는 존재할 때 인수로 전달됩니다. 확장 프로그램 빌드가 플랫폼에 포함되지 않은 경우 별도로 설치된 `claude` 바이너리로 설정합니다. 래핑된 설정에서는 `initialPermissionMode`를 설정하거나 이전 대화에서 Manual, Edit automatically 또는 Auto를 선택하지 않는 한 대화가 Manual 모드에서 시작됩니다. 확장 프로그램이 설정 및 기본 제공 기본값 단계를 건너뛰기 때문입니다. [권한 모드 전환](/docs/ko/permission-modes#switch-permission-modes)을 참조하세요. 활성화 시 "Unsupported platform" 오류는 플랫폼에 번들된 바이너리가 없음을 의미합니다. [어떤 플랫폼에 미리 빌드된 바이너리가 있는지](/docs/ko/troubleshoot-install#native-binary-not-found-after-npm-install) 참조하세요. |

<h2 id="use-a-screen-reader">
  화면 읽기 프로그램 사용
</h2>

확장 프로그램의 채팅 패널은 화면 읽기 프로그램과 호환됩니다. 아무것도 켤 필요가 없습니다. 확장 프로그램은 시각적 변화 없이 모든 사용자에 대해 대화 활동을 알립니다. 이는 터미널 인터페이스를 조정하는 CLI의 선택적 [화면 읽기 모드](/docs/ko/accessibility)와는 별개입니다.

채팅 패널의 화면 읽기 프로그램 지원은 Claude Code v2.1.236 이상이 필요합니다.

대화 중에 확장 프로그램은 다음을 알립니다:

* **Claude의 응답**: 확장 프로그램은 각 응답을 완료되었을 때 한 번 알리고, 텍스트가 스트리밍되는 동안 침묵을 유지합니다. 화면 읽기 프로그램은 코드 블록을 줄 수 요약으로 읽고, 링크를 레이블로 읽으며, 표를 셀 단위로 읽습니다. 전체 응답은 기록에서 읽을 수 있는 상태로 유지됩니다.
* **권한 요청 및 질문**: 확장 프로그램은 권한 프롬프트가 나타날 때 요청을 알리고, Claude가 사용하려는 도구의 이름을 지정합니다. Claude가 질문을 할 때와 Claude가 계획을 완료하고 검토를 기다릴 때도 같은 방식으로 알립니다.
* **상태 변경**: 확장 프로그램은 Claude가 작업을 시작할 때, Claude가 입력을 기다릴 준비가 되었을 때, Claude Code가 대화를 압축하기 시작할 때를 알립니다.
* **오류 및 모델 프롬프트**: 확장 프로그램은 대화의 오류를 알리고, [사용량-크레딧 동의 프롬프트](/docs/ko/model-config#fable-and-usage-credits) 또는 [플래그된 요청 프롬프트](/docs/ko/model-config#ask-before-switching)가 나타날 때를 알립니다.

Claude가 작업하는 동안, 화면 읽기 프로그램은 진행률 스피너의 애니메이션 대신 텍스트 레이블을 읽습니다.

세션을 다시 열거나 다른 세션으로 전환할 때, 확장 프로그램은 아무것도 알리지 않습니다. 복원된 기록, 보류 중인 권한 프롬프트, 진행 중인 상태는 새로운 일이 발생할 때까지 침묵을 유지합니다.

<h3 id="use-the-chat-panel-from-the-keyboard">
  키보드에서 채팅 패널 사용
</h3>

기록의 각 턴은 턴을 시작한 프롬프트로 레이블이 지정된 시각적으로 숨겨진 제목으로 시작되므로, 화면 읽기 프로그램의 제목 탐색을 사용하여 턴 사이를 이동할 수 있습니다.

턴 내에서, 화면 읽기 프로그램은 메시지를 통해 이동할 때 어느 메시지에 있는지 알립니다:

* **사용자 메시지**: "You"
* **Claude의 메시지**: "Claude"
* **도구 단계**: "Claude" 및 도구 이름(예: "Claude, Bash")
* **생각 블록**: "Claude, thinking"

확장 프로그램이 기록을 레이블이 지정된 영역으로 노출하기 때문에, `Tab`을 사용하여 기록 자체에 포커스를 이동하고 자신의 속도로 읽을 수도 있습니다. 최신 메시지 또는 대기 중인 권한 프롬프트로 포커스를 이동하려면, [명령 팔레트](#vs-code-commands-and-shortcuts)에서 **Claude Code: Focus last message**를 실행합니다.

권한 프롬프트의 옵션이 권한 규칙 또는 디렉터리 액세스를 저장할 때, 해당 레이블은 "모든 프로젝트" 또는 "이 세션"과 같이 승인이 저장되는 위치의 이름을 지정하여 끝납니다. 이 옵션에 포커스가 있을 때, `Left` 또는 `Right` 화살표 키를 눌러 대상을 변경할 수 있으며, 확장 프로그램은 각 대상에 도달할 때마다 알립니다. 레이블에서 대상을 클릭할 수도 있습니다. 화살표 키는 Claude Code v2.1.268 이상이 필요합니다.

<h2 id="vs-code-extension-vs-claude-code-cli">
  VS Code 확장 프로그램 vs. Claude Code CLI
</h2>

Claude Code는 VS Code 확장 프로그램(그래픽 패널)과 CLI(터미널의 명령줄 인터페이스) 모두로 사용할 수 있습니다. 일부 기능은 CLI에서만 사용할 수 있습니다. CLI 전용 기능이 필요한 경우 VS Code의 통합 터미널에서 `claude`를 실행하십시오. 이를 위해서는 [독립 실행형 CLI 설치](/docs/ko/setup)가 필요합니다. 확장 프로그램은 `claude`를 PATH에 추가하지 않습니다. [VS Code에서 CLI 실행](#run-cli-in-vs-code)을 참조하십시오.

| 기능             | CLI                | VS Code 확장 프로그램                                                            |
| -------------- | ------------------ | -------------------------------------------------------------------------- |
| 명령 및 기술        | [모두](/docs/ko/commands) | 부분 집합 (채팅 패널에서 `/`를 입력하여 사용 가능한 항목 확인)                                     |
| MCP 서버 구성      | 예                  | 예 (채팅 패널에서 `/mcp`를 사용하여 [서버 추가 및 관리](#connect-to-external-tools-with-mcp)) |
| Checkpoints    | 예                  | 예                                                                          |
| `!` Bash 바로 가기 | 예                  | 아니요                                                                        |
| Tab 완성         | 예                  | 아니요                                                                        |

<h3 id="rewind-with-checkpoints">
  Checkpoints를 사용한 되감기
</h3>

VS Code 확장 프로그램은 Claude의 파일 편집을 추적하고 이전 상태로 되돌릴 수 있는 checkpoints를 지원합니다. 메시지 위에 마우스를 올려 되감기 버튼을 표시한 다음 세 가지 옵션 중에서 선택합니다.

* **여기서 대화 분기**: 모든 코드 변경 사항을 유지하면서 이 메시지에서 새 대화 분기를 시작합니다.
* **여기까지 코드 되감기**: 전체 대화 기록을 유지하면서 파일 변경 사항을 이 지점으로 되돌립니다.
* **대화 분기 및 코드 되감기**: 새 대화 분기를 시작하고 파일 변경 사항을 이 지점으로 되돌립니다.

Checkpoints의 작동 방식 및 제한 사항에 대한 자세한 내용은 [Checkpointing](/docs/ko/checkpointing)을 참조하십시오.

<h3 id="run-cli-in-vs-code">
  VS Code에서 CLI 실행
</h3>

VS Code에 머물면서 CLI를 사용하려면 통합 터미널을 열고 (Windows/Linux에서는 `` Ctrl+` ``, Mac에서는 `` Cmd+` ``) `claude`를 실행합니다. CLI는 diff 보기 및 진단 공유와 같은 기능을 위해 IDE와 자동으로 통합됩니다.

확장 프로그램을 설치해도 `claude`가 셸 PATH에 추가되지 않습니다. 확장 프로그램은 채팅 패널을 위해 CLI의 개인 복사본을 번들로 제공하지만, 터미널에서 `claude`를 입력하려면 [독립 실행형 CLI 설치](/docs/ko/setup)가 필요합니다. 설치를 한 번 실행하면 이 페이지의 명령(예: `claude mcp add` 및 `claude --resume`)이 모든 터미널에서 작동합니다. 설치 후에도 `claude`를 찾을 수 없으면 [PATH 확인](/docs/ko/troubleshoot-install#verify-your-path)을 참조하십시오.

외부 터미널을 사용하는 경우 Claude Code 내에서 `/ide`를 실행하여 VS Code에 연결합니다.

<h3 id="switch-between-extension-and-cli">
  확장 프로그램과 CLI 간 전환
</h3>

확장 프로그램과 CLI는 동일한 대화 기록을 공유합니다. 확장 프로그램 대화를 CLI에서 계속하려면 터미널에서 `claude --resume`을 실행합니다. 이렇게 하면 대화를 검색하고 선택할 수 있는 대화형 선택기가 열립니다.

<h3 id="include-terminal-output-in-prompts">
  프롬프트에 터미널 출력 포함
</h3>

`@terminal:name`을 사용하여 프롬프트에서 터미널 출력을 참조합니다. 여기서 `name`은 터미널의 제목입니다. 이를 통해 Claude는 복사하여 붙여넣기 없이 명령 출력, 오류 메시지 또는 로그를 볼 수 있습니다.

<h3 id="monitor-background-processes">
  백그라운드 프로세스 모니터링
</h3>

프롬프트 상자에서 `/tasks`를 입력하여 [에이전트 맵](#use-the-prompt-box)을 열면, 세션의 백그라운드 작업(예: Claude가 백그라운드 셸 명령으로 실행 중인 개발 서버)이 나열됩니다. 작업을 클릭하여 카드를 열고 거기서 중지할 수 있습니다. Claude Code v2.1.277 이상이 필요합니다.

<h3 id="connect-to-external-tools-with-mcp">
  MCP를 사용하여 외부 도구에 연결
</h3>

MCP(Model Context Protocol) 서버는 Claude에게 외부 도구, 데이터베이스 및 API에 대한 액세스를 제공합니다.

VS Code를 떠나지 않고 MCP 서버를 관리하려면 채팅 패널에서 `/mcp`를 입력합니다. 열리는 대화 상자에서 서버를 추가하고, 로컬, 사용자 또는 프로젝트 [범위](/docs/ko/mcp#mcp-installation-scopes)에 저장된 서버를 제거하고, 서버를 활성화 또는 비활성화하고, 서버에 다시 연결하고, OAuth 인증을 관리할 수 있습니다. 대화 상자에서 서버를 추가하고 제거하려면 Claude Code v2.1.261 이상이 필요합니다.

VS Code의 통합 터미널에서 `claude mcp add`를 실행할 수도 있습니다 (`` Ctrl+` `` 또는 `` Cmd+` ``). 대화 상자와 터미널 명령은 동일한 MCP 구성에 저장되며, 둘 중 하나의 변경 사항은 이후에 시작하는 대화에 적용됩니다. 아래 예제는 [개인 액세스 토큰](https://github.com/settings/personal-access-tokens)으로 인증하는 GitHub의 원격 MCP 서버를 추가합니다. 이 토큰은 헤더로 전달됩니다.

```bash theme={null}
claude mcp add --transport http github https://api.githubcopilot.com/mcp/ \
  --header "Authorization: Bearer YOUR_GITHUB_PAT"
```

`YOUR_GITHUB_PAT`을 개인 액세스 토큰으로 바꿉니다. `claude mcp add` 명령은 자격 증명을 검증하지 않고 구성을 저장하므로 여기에 자리 표시자 값이 허용되지만 서버가 나중에 연결하지 못합니다. 연결을 확인하려면 새 대화를 시작하고 `/mcp`를 입력한 다음 서버가 **Connected**를 표시하는지 확인합니다. 잘못된 자격 증명이 있는 서버는 **Failed**를 표시합니다.

구성되면 Claude에게 도구를 사용하도록 요청합니다 (예: "Review PR #456").

연결할 서버를 찾으려면 [MCP 서버 찾기 및 빌드](/docs/ko/mcp#find-and-build-mcp-servers)를 참조하십시오.

<h2 id="work-with-git">
  git으로 작업하기
</h2>

Claude Code는 git과 통합되어 VS Code에서 직접 버전 제어 워크플로우를 지원합니다. Claude에게 변경 사항을 커밋하거나, 풀 리퀘스트를 생성하거나, 브랜치 간에 작업하도록 요청할 수 있습니다. 자체 파일과 브랜치가 있는 격리된 worktree에서 Claude를 시작하려면 [worktree를 사용하여 병렬 세션 실행](/docs/ko/worktrees)을 참조하십시오.

<h3 id="create-commits-and-pull-requests">
  커밋 및 풀 리퀘스트 생성
</h3>

Claude는 변경 사항을 스테이징하고, 커밋 메시지를 작성하며, 작업을 기반으로 풀 리퀘스트를 생성할 수 있습니다:

```text wrap theme={null}
commit my changes with a descriptive message
create a pr for this feature
summarize the changes I've made to the auth module
```

풀 리퀘스트를 생성할 때, Claude는 실제 코드 변경 사항을 기반으로 설명을 생성하며 테스트 또는 구현 결정에 대한 컨텍스트를 추가할 수 있습니다.

<h2 id="use-third-party-providers">
  타사 제공자 사용
</h2>

기본적으로 Claude Code는 Anthropic의 API에 직접 연결됩니다. 조직에서 Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry를 사용하여 Claude에 액세스하는 경우, 대신 제공자를 사용하도록 확장 프로그램을 구성하세요:

<Steps>
  <Step title="로그인 프롬프트 비활성화">
    [로그인 프롬프트 비활성화 설정](vscode://settings/claudeCode.disableLoginPrompt)을 열고 확인란을 선택하세요.

    VS Code 설정(`Mac에서 Cmd+,` 또는 `Windows/Linux에서 Ctrl+,`)을 열고 "Claude Code login"을 검색한 후 **로그인 프롬프트 비활성화**를 선택할 수도 있습니다.
  </Step>

  <Step title="제공자 구성">
    제공자에 대한 설정 가이드를 따르세요:

    * [Amazon Bedrock의 Claude Code](/docs/ko/amazon-bedrock)
    * [Google Cloud의 Agent Platform의 Claude Code](/docs/ko/google-vertex-ai)
    * [Microsoft Foundry의 Claude Code](/docs/ko/microsoft-foundry)

    이 가이드는 `~/.claude/settings.json`에서 제공자를 구성하는 방법을 다루며, 이를 통해 VS Code 확장 프로그램과 CLI 간에 설정이 공유됩니다.
  </Step>
</Steps>

타사 제공자에서 확장 프로그램은 사용 추적, [음성 받아쓰기](/docs/ko/voice-dictation) 및 [클라우드 세션](#resume-cloud-sessions-from-claude-ai)을 위한 웹 탭과 같이 claude.ai 계정이 필요한 기능을 제공하지 않습니다. 이러한 로그인에서 계정 및 사용 현황 대화 상자가 표시하는 내용은 [계정 및 사용 현황 확인](#check-account-and-usage)을 참조하세요. 이전 `/login`에서 남겨진 claude.ai 로그인은 사용되지 않습니다: 확장 프로그램은 이를 어떤 요청과도 함께 보내지 않습니다.

<h2 id="security-and-privacy">
  보안 및 개인정보 보호
</h2>

사용자의 코드는 비공개로 유지됩니다. Claude Code는 코드를 처리하여 지원을 제공하지만 모델 학습에 사용하지 않습니다. 데이터 처리 및 로깅 거부 방법에 대한 자세한 내용은 [데이터 및 개인정보 보호](/docs/ko/data-usage)를 참조하십시오.

자동 편집 권한이 활성화되면 Claude Code는 VS Code가 자동으로 실행할 수 있는 VS Code 구성 파일(예: `settings.json` 또는 `tasks.json`)을 수정할 수 있습니다. 신뢰할 수 없는 코드로 작업할 때 위험을 줄이려면:

* 신뢰할 수 없는 작업 공간에 대해 [VS Code 제한 모드](https://code.visualstudio.com/docs/editor/workspace-trust#_restricted-mode)를 활성화하십시오
* 자동 편집 또는 자동 편집 대신 수동 모드를 사용하십시오
* 변경 사항을 수락하기 전에 신중하게 검토하십시오

<h3 id="the-built-in-ide-mcp-server">
  기본 제공 IDE MCP 서버
</h3>

확장 프로그램이 활성화되면 CLI가 자동으로 연결하는 로컬 MCP 서버를 실행합니다. 이것이 CLI가 VS Code의 기본 diff 뷰어에서 diff를 열고, `@`-mentions에 대한 현재 선택 항목을 읽으며, Jupyter 노트북에서 작업할 때 VS Code에 셀 실행을 요청하는 방식입니다.

서버의 이름은 `ide`이며 구성할 항목이 없으므로 `/mcp`에서 숨겨집니다. 그러나 조직에서 `PreToolUse` 훅을 사용하여 MCP 도구를 허용 목록에 추가하는 경우 이 서버가 존재한다는 것을 알아야 합니다.

**선택 및 열린 파일 컨텍스트.** 연결되어 있는 동안 CLI는 현재 편집기 선택 항목과 활성 파일의 경로를 각 프롬프트에 컨텍스트로 포함합니다. 이 경우 트랜스크립트에 `⧉ Selected N lines from <file>` 줄이 표시됩니다.

`.env`와 같은 민감한 파일을 제외하려면 해당 경로에 대한 [`Read` 거부 규칙](/docs/ko/permissions#read-and-edit)을 추가하십시오. 일치하는 거부 규칙은 선택된 텍스트와 해당 파일에 대한 열린 파일 공지가 Claude에 도달하는 것을 모두 방지합니다.

[Attach Open File 설정](#extension-settings)을 끄면 CLI는 해당 파일에서 텍스트를 선택한 경우에만 활성 파일의 경로를 받습니다.

**전송 및 인증.** 서버는 10000–65535 범위의 임의 포트에서 `127.0.0.1`에 바인딩되며 포트는 구성할 수 없습니다. 전송은 암호화되지 않은 `ws://`입니다. 소켓이 루프백 전용이므로 트래픽을 캡처할 수 있는 모든 프로세스는 잠금 파일에서 토큰을 읽을 수도 있으므로 TLS는 보호를 추가하지 않습니다. 각 확장 프로그램 활성화는 새로운 임의 인증 토큰을 생성하고 `~/.claude/ide/<port>.lock`의 잠금 파일에 기록하며 CLI는 이를 `X-Claude-Code-Ide-Authorization` 헤더로 제시하여 연결해야 합니다. 잠금 파일은 `0700` 디렉터리에서 `0600` 권한을 가지므로 VS Code를 실행하는 사용자만 읽을 수 있습니다. `CLAUDE_CONFIG_DIR`이 설정되면 잠금 파일은 대신 `$CLAUDE_CONFIG_DIR/ide/`에 기록됩니다.

**모델에 노출된 도구.** 서버는 약 12개의 도구를 호스팅하지만 모델에는 2개만 표시됩니다. 나머지는 CLI가 자체 UI(diff 열기, 선택 항목 읽기, 파일 저장)에 사용하는 내부 RPC이며 도구 목록이 Claude에 도달하기 전에 필터링됩니다.

| 도구 이름(훅에서 보이는 대로)          | 기능                                                                    | 읽기 전용 |
| -------------------------- | --------------------------------------------------------------------- | ----- |
| `mcp__ide__getDiagnostics` | 언어 서버 진단(VS Code의 문제 패널의 오류 및 경고)을 반환합니다. 선택적으로 한 파일로 범위를 지정할 수 있습니다. | 예     |
| `mcp__ide__executeCode`    | 활성 Jupyter 노트북의 커널에서 Python 코드를 실행합니다. 아래의 확인 흐름을 참조하십시오.             | 아니요   |

**Jupyter 실행은 항상 먼저 묻습니다.** `mcp__ide__executeCode`는 아무것도 자동으로 실행할 수 없습니다. 각 호출 시 코드는 활성 노트북의 끝에 새 셀로 삽입되고 VS Code는 이를 보기로 스크롤하며 기본 Quick Pick에서 **실행** 또는 **취소**를 요청합니다. 취소하거나 `Esc`로 선택기를 닫으면 Claude에 오류가 반환되고 아무것도 실행되지 않습니다. 이 도구는 활성 노트북이 없을 때, Jupyter 확장 프로그램(`ms-toolsai.jupyter`)이 설치되지 않았을 때 또는 커널이 Python이 아닐 때 완전히 거부합니다.

<Note>
  Quick Pick 확인은 `PreToolUse` 훅과 별개입니다. `mcp__ide__executeCode`에 대한 허용 목록 항목을 사용하면 Claude가 셀 실행을 *제안*할 수 있습니다. VS Code 내의 Quick Pick은 실제로 *실행*할 수 있게 해주는 것입니다.
</Note>

<a id="troubleshooting" />

<h2 id="fix-common-issues">
  일반적인 문제 해결
</h2>

<h3 id="extension-won’t-install">
  확장 프로그램이 설치되지 않음
</h3>

* VS Code의 호환 버전(1.94.0 이상)이 있는지 확인하세요
* VS Code에 확장 프로그램을 설치할 권한이 있는지 확인하세요
* [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=anthropic.claude-code)에서 직접 설치를 시도하세요

<h3 id="spark-icon-not-visible">
  Spark 아이콘이 보이지 않음
</h3>

Spark 아이콘은 파일이 열려 있을 때 **편집기 도구 모음**(편집기 우측 상단)에 나타납니다. 아이콘이 보이지 않으면:

1. **파일 열기**: 아이콘이 작동하려면 파일이 열려 있어야 합니다. 폴더만 열려 있으면 충분하지 않습니다.
2. **VS Code 버전 확인**: 1.94.0 이상 필요(도움말 → 정보)
3. **VS Code 다시 시작**: 명령 팔레트에서 "Developer: Reload Window" 실행
4. **충돌하는 확장 프로그램 비활성화**: 다른 AI 확장 프로그램(Cline, Continue 등) 임시 비활성화
5. **작업 영역 신뢰 확인**: 확장 프로그램은 제한된 모드에서 작동하지 않습니다

또는 [`preferredLocation`](#extension-settings)을 `sidebar`로 설정했거나 **Claude Code: Open in Side Bar**로 Claude를 열었다면, **상태 표시줄**(우측 하단 모서리)의 "✻ Claude Code"를 클릭하세요. 이는 파일이 열려 있지 않아도 작동합니다. **명령 팔레트**(`Cmd+Shift+P` / `Ctrl+Shift+P`)를 사용하고 "Claude Code"를 입력할 수도 있습니다.

<h3 id="cmd-esc-does-nothing-on-macos">
  macOS에서 Cmd+Esc가 작동하지 않음
</h3>

macOS Tahoe 이상에서는 시스템 Game Overlay 단축키가 기본적으로 `Cmd+Esc`에 바인딩되어 있으며 VS Code에 도달하기 전에 키 입력을 가로챕니다. 단축키를 해제하려면:

1. 시스템 설정 열기
2. 키보드로 이동한 후 키보드 단축키, 그 다음 게임 컨트롤러로 이동
3. Game Overlay 체크박스 선택 해제

또는 확장 프로그램을 다른 키로 다시 바인딩하세요: VS Code [키보드 단축키 편집기](https://code.visualstudio.com/docs/configure/keybindings)(`Cmd+K Cmd+S`)를 열고 `Claude Code: Focus input`을 검색한 후 새 바인딩을 할당하세요.

<h3 id="claude-code-never-responds">
  Claude Code가 응답하지 않음
</h3>

Claude Code가 프롬프트에 응답하지 않으면:

1. **인터넷 연결 확인**: 안정적인 인터넷 연결이 있는지 확인하세요
2. **새 대화 시작**: 새로운 대화를 시작하여 문제가 지속되는지 확인하세요
3. **CLI 시도**: 터미널에서 `claude`를 실행하여 더 자세한 오류 메시지를 확인하세요

문제가 지속되면 오류에 대한 세부 정보와 함께 [GitHub에 이슈를 제출하세요](https://github.com/anthropics/claude-code/issues).

<h2 id="uninstall-the-extension">
  확장 프로그램 제거
</h2>

Claude Code 확장 프로그램을 제거하려면:

1. 확장 프로그램 보기를 엽니다 (Mac에서는 `Cmd+Shift+X`, Windows/Linux에서는 `Ctrl+Shift+X`)
2. "Claude Code"를 검색합니다
3. **제거**를 클릭합니다

VS Code 통합 터미널에서 `claude`를 실행하면 Claude Code가 확장 프로그램을 자동으로 다시 설치합니다. 제거된 상태로 유지하려면 `/config`에서 **Auto-install IDE extension**을 끄거나, [`autoInstallIdeExtension`](/docs/ko/settings-reference#autoinstallideextension)을 `false`로 설정합니다. [`CLAUDE_CODE_IDE_SKIP_AUTO_INSTALL`](/docs/ko/env-vars) 환경 변수를 `1`로 설정할 수도 있습니다.

확장 프로그램 데이터를 제거하고 모든 설정을 초기화하려면 플랫폼에 해당하는 확장 프로그램의 저장소 디렉터리를 삭제합니다.

macOS에서:

```bash theme={null}
rm -rf ~/Library/"Application Support"/Code/User/globalStorage/anthropic.claude-code
```

Linux에서:

```bash theme={null}
rm -rf ~/.config/Code/User/globalStorage/anthropic.claude-code
```

Windows에서 PowerShell에서:

```powershell theme={null}
Remove-Item -Recurse -Force "$env:APPDATA\Code\User\globalStorage\anthropic.claude-code"
```

추가 도움말은 [문제 해결 가이드](/docs/ko/troubleshooting)를 참조하십시오.

<h2 id="next-steps">
  다음 단계
</h2>

이제 VS Code에서 Claude Code를 설정했습니다:

* [일반적인 워크플로우 살펴보기](/docs/ko/common-workflows)를 통해 Claude Code를 최대한 활용하세요
* [MCP 서버 설정](/docs/ko/mcp)을 통해 외부 도구로 Claude의 기능을 확장하세요. 채팅 패널에서 `/mcp`를 사용하여 추가하고 관리할 수 있습니다.
* [Claude Code 설정 구성](/docs/ko/settings)을 통해 허용된 명령어, 훅 등을 사용자 정의하세요. 이러한 설정은 확장 프로그램과 CLI 간에 공유됩니다.
