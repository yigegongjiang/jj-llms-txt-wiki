> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 코드 인텔리전스 플러그인

> 언어 서버 플러그인을 설치하여 Claude가 편집 후 타입 오류를 확인하고 기호로 코드를 탐색하며, LSP 플러그인 권장 대화상자에 응답합니다.

코드 인텔리전스 플러그인은 Claude에게 편집기가 가진 라이브 진단 및 정의로 이동 기능을 제공하므로, Claude가 자신의 편집으로 인한 타입 오류 및 누락된 임포트를 빌드를 실행하기 전에 포착하고, 텍스트 검색 대신 기호로 정의 및 참조를 찾을 수 있습니다.

각 플러그인은 Language Server Protocol(LSP)을 통해 Claude Code를 한 언어의 언어 서버에 연결합니다. 플러그인은 Anthropic의 공식 마켓플레이스에서 설치하고 언어 서버 바이너리는 컴퓨터에 설치합니다.

<Note>
  코드 인텔리전스 플러그인은 터미널 세션에서 작동합니다. [클라우드 세션](/docs/ko/claude-code-on-the-web)에서는 Claude Code가 플러그인 언어 서버를 시작하지 않으므로 Claude는 진단 또는 코드 탐색을 받지 않습니다. 자신의 언어 서버 플러그인을 작성하거나 플러그인이 없는 언어 서버를 연결하려면 [플러그인 컴포넌트의 LSP 서버](/docs/ko/plugins/components#lsp-servers)를 참조하세요.
</Note>

시작하려면 [코드 인텔리전스 플러그인 설치](#install-a-code-intelligence-plugin) 아래의 표에서 언어를 찾으세요. 해당 표의 플러그인은 Anthropic의 [공식 플러그인 마켓플레이스](/docs/ko/plugins/anthropic-marketplaces)에서 제공됩니다.

이미 **LSP 플러그인 권장** 대화상자를 본 경우, [권장 대화상자 수락 또는 거부](#accept-or-dismiss-the-recommendation-dialog)에서 각 선택이 무엇을 하는지 확인하세요.

<h2 id="install-a-code-intelligence-plugin">
  코드 인텔리전스 플러그인 설치
</h2>

코드 인텔리전스 플러그인은 Claude Code에 언어 서버를 시작하는 명령과 처리하는 파일 확장자를 알려줍니다. 언어 서버는 포함하지 않습니다. 먼저 언어 서버 바이너리를 설치한 다음 플러그인을 설치하고 서버가 시작되는지 확인하세요.

<Steps>
  <Step title="언어 서버 바이너리 설치">
    아래 표에서 언어를 찾아 해당 행의 바이너리를 설치하세요. 언어가 나열되지 않은 경우 [공식 플러그인 없이 언어 추가](#add-a-language-without-an-official-plugin)를 참조하세요.

    | 언어                      | 플러그인                                                                                                             | 바이너리                         |
    | :---------------------- | :--------------------------------------------------------------------------------------------------------------- | :--------------------------- |
    | C/C++                   | [`clangd-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/clangd-lsp)               | `clangd`                     |
    | C#                      | [`csharp-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/csharp-lsp)               | `csharp-ls`                  |
    | Go                      | [`gopls-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/gopls-lsp)                 | `gopls`                      |
    | Java                    | [`jdtls-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/jdtls-lsp)                 | `jdtls`                      |
    | Kotlin                  | [`kotlin-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/kotlin-lsp)               | `kotlin-lsp`                 |
    | Liquid                  | [`liquid-lsp`](https://github.com/Shopify/liquid-skills/tree/main/plugins/liquid-lsp)                            | `shopify`, Shopify CLI에서     |
    | Lua                     | [`lua-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/lua-lsp)                     | `lua-language-server`        |
    | PHP                     | [`php-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/php-lsp)                     | `intelephense`               |
    | Python                  | [`pyright-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/pyright-lsp)             | `pyright-langserver`         |
    | Ruby                    | [`ruby-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/ruby-lsp)                   | `ruby-lsp`                   |
    | Rust                    | [`rust-analyzer-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/rust-analyzer-lsp) | `rust-analyzer`              |
    | Swift                   | [`swift-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/swift-lsp)                 | `sourcekit-lsp`              |
    | TypeScript 및 JavaScript | [`typescript-lsp`](https://github.com/anthropics/claude-plugins-official/tree/main/plugins/typescript-lsp)       | `typescript-language-server` |

    Anthropic은 `liquid-lsp`를 제외한 표의 모든 플러그인을 유지 관리하며, `liquid-lsp`는 Shopify가 유지 관리하고 공식 마켓플레이스에 나열합니다.

    바이너리를 설치하는 명령을 찾으려면 표의 플러그인 링크를 따라 README로 이동하세요. TypeScript의 경우 해당 명령은 `npm install -g typescript-language-server typescript`입니다.

    바이너리를 설치한 후 `claude`를 시작하는 셸의 `PATH`에 있는지 확인하세요. 예를 들어 `which typescript-language-server` 또는 PowerShell에서 `Get-Command typescript-language-server`를 사용합니다.
  </Step>

  <Step title="플러그인 설치">
    1단계 표에서 언어에 대해 나열된 플러그인을 설치하려면 Claude Code 세션에서 `/plugin install`을 실행하고 `typescript-lsp`를 해당 플러그인의 이름으로 바꾸세요:

    ```
    /plugin install typescript-lsp@claude-plugins-official
    ```

    확인 메시지는 플러그인이 지금 활성화되었는지 또는 `/reload-plugins`가 필요한지 여부를 나타냅니다. 설치가 `Marketplace "claude-plugins-official" not found`로 실패하면 [해당 오류에 대한 문제 해결 항목](/docs/ko/plugins/troubleshooting#marketplace-claude-plugins-official-not-found)을 참조하세요. 플러그인이 설치되는 위치를 제어하거나 Claude Code 내부 대신 셸에서 설치를 실행하려면 [플러그인 설치](/docs/ko/plugins/install)를 참조하세요.
  </Step>

  <Step title="서버 시작 확인">
    언어 서버는 Claude가 플러그인의 확장자 중 하나를 가진 파일을 편집할 때 처음 시작됩니다. 작동하는 것을 보려면 Claude에게 해당 언어의 파일에 타입 오류를 도입한 다음 수정하도록 요청하세요. 그런 다음 대화에서 진단 라인을 확인하세요:

    * **진단 라인이 나타남**: 오류를 도입한 편집 아래에 `Found N new diagnostic issues in M files (ctrl+o to expand)`는 서버가 시작되었음을 의미합니다.
    * **진단 라인이 나타나지 않음**: `/plugin`을 실행하고 **Errors** 탭을 엽니다. `Executable not found in $PATH: "<binary>"`를 읽는 행은 설치할 바이너리의 이름을 지정합니다. 탭에 그러한 행이 없으면 [코드 인텔리전스 문제 해결](#troubleshoot-code-intelligence)을 참조하세요.

    누락된 바이너리를 설치한 후 Claude Code는 Claude가 다음에 일치하는 파일을 편집할 때 다시 시도합니다. 바이너리를 `claude`를 시작한 셸의 `PATH`에 없는 디렉토리에 설치한 경우 해당 디렉토리가 있는 셸에서 새 세션을 시작하세요.
  </Step>
</Steps>

<h2 id="see-what-claude-gains">
  Claude가 얻는 것 확인
</h2>

언어 서버가 실행 중이면 Claude는 진단 및 코드 탐색을 얻습니다:

* **편집 후 진단**: Claude가 서버가 처리하는 파일을 편집하거나 쓸 때마다 Claude는 서버가 보고하는 오류 및 경고를 받습니다. 컴파일러를 실행하지 않고도 도입한 타입 오류, 누락된 임포트 또는 구문 오류를 봅니다.
* **코드 탐색**: Claude는 텍스트를 검색하는 대신 서버를 통해 기호를 조회하는 `LSP` 도구를 얻습니다. 도구는 읽기 전용입니다. Claude가 도구로 조회할 수 있는 것과 권한이 어떻게 적용되는지는 [LSP 도구 동작](/docs/ko/tools-reference#lsp-tool-behavior)을 참조하세요.

<h3 id="read-the-diagnostics-yourself">
  진단을 직접 읽기
</h3>

Claude가 서버가 처리하는 파일을 편집한 후 대화는 `Found N new diagnostic issues` 요약만 표시합니다. 문제 자체를 읽으려면 **Ctrl+O**를 누르세요.

<h2 id="accept-or-dismiss-the-recommendation-dialog">
  권장 대화상자 수락 또는 거부
</h2>

언어 서버 바이너리가 이미 `PATH`에 있고 이를 사용하는 플러그인이 설치되지 않은 경우 Claude Code는 **LSP 플러그인 권장** 대화상자에서 플러그인을 설치하도록 제안합니다.

<h3 id="when-the-recommendation-dialog-appears">
  권장 대화상자가 나타나는 경우
</h3>

**LSP 플러그인 권장** 대화상자는 Claude가 파일을 편집한 후 나타날 수 있습니다. 이러한 조건은 나타나는지 여부와 제공하는 플러그인을 결정합니다:

* **플러그인이 파일과 일치**: 추가한 마켓플레이스 중 하나 또는 Claude Code가 등록한 공식 마켓플레이스가 해당 파일의 확장자에 대한 코드 인텔리전스 플러그인을 나열하고 플러그인의 바이너리가 설치됩니다.
* **공식 우선**: 둘 이상의 마켓플레이스가 확장자에 대한 플러그인을 제공할 때 대화상자는 공식 마켓플레이스의 플러그인을 제공합니다.
* **세션당 한 번**: 대화상자는 세션에서 최대 한 번 나타나며 Claude가 편집하는 첫 번째 일치하는 파일에 대해 나타납니다.
* **클라우드 세션의 경우 아님**: [`claude --cloud`](/docs/ko/claude-code-on-the-web#from-terminal-to-cloud)로 시작한 것과 같은 클라우드 세션에 터미널이 연결되어 있을 때 대화상자는 나타나지 않습니다.

<h3 id="respond-to-the-recommendation-dialog">
  권장 대화상자에 응답
</h3>

**LSP 플러그인 권장** 대화상자는 플러그인의 이름을 지정하고 다음 선택을 제공합니다:

* **예, 설치**: Claude Code는 사용자 계정에 플러그인을 설치하고 `<plugin> installed · restart to apply`를 인쇄합니다. 새 세션을 시작하여 서버를 로드하세요.
* **아니요, 나중에**: 대화상자가 닫히고 나중 세션에서 플러그인을 다시 제공할 수 있습니다. **Esc**를 누르면 동일합니다.
* **이 플러그인은 절대 안 됨**: 대화상자는 해당 플러그인에 대해 나타나지 않으며 다른 플러그인에 대해서는 계속 나타납니다.
* **모든 LSP 권장 사항 비활성화**: 대화상자는 모든 언어에 대해 나타나지 않습니다.

옵션을 선택하지 않으면 Claude Code는 30초 후 닫고 무시된 것으로 계산합니다. 개수는 세션 전체에서 유지됩니다. 5개의 무시된 대화상자 후 Claude Code는 플러그인 권장을 중지하며, 이는 **모든 LSP 권장 사항 비활성화**를 선택한 것과 동일합니다.

<h3 id="turn-recommendations-back-on">
  권장 사항 다시 켜기
</h3>

**LSP 플러그인 권장** 대화상자는 **모든 LSP 권장 사항 비활성화**를 선택하거나 5번 무시한 후 나타나지 않습니다.

* **비활성화 또는 5번 무시됨**: 어느 경우든 다시 켜려면 Claude Code의 자체 구성 파일인 `~/.claude.json`에서 `lspRecommendationDisabled` 및 `lspRecommendationIgnoredCount` 키를 제거하세요.
* **이 플러그인은 절대 안 됨**: **이 플러그인은 절대 안 됨**을 선택했고 해당 플러그인을 다시 제공받으려면 같은 파일의 `lspRecommendationNeverPlugins` 목록에서 해당 `name@marketplace` id를 제거하세요.

<h2 id="troubleshoot-code-intelligence">
  코드 인텔리전스 문제 해결
</h2>

플러그인 문제 해결 페이지는 [언어 서버가 시작되지 않음, 메모리를 너무 많이 사용하거나 잘못된 진단을 보고함](/docs/ko/plugins/troubleshooting#language-server-doesnt-start) 아래의 코드 인텔리전스 플러그인에 특정한 증상을 다룹니다:

* **언어 서버가 시작되지 않음**: `/plugin`의 **Errors** 탭에서 `Executable not found in $PATH`를 보거나 Claude가 해당 언어에 대한 진단을 보고하지 않습니다.
* **높은 메모리 사용**: 서버가 프로젝트를 인덱싱하는 동안 메모리 사용이 증가합니다.
* **모노레포의 거짓 양성 진단**: 진단은 임포트를 해결되지 않은 것으로 보고하지만 실제로는 해결됩니다.

<h2 id="add-a-language-without-an-official-plugin">
  공식 플러그인 없이 언어 추가
</h2>

언어가 [공식 플러그인 표](#install-a-code-intelligence-plugin)에 없으면 여전히 언어 서버를 연결할 수 있습니다.

1. 서버 명령과 처리하는 파일 확장자의 이름을 지정하는 `.lsp.json` 파일로 플러그인을 작성하세요.
2. 그런 다음 [`--plugin-dir`](/docs/ko/plugins/cli-reference#flags-that-load-a-plugin-for-one-session)을 사용하여 플러그인을 로드하거나 마켓플레이스에 게시하세요.

파일의 필드 및 작동 예제는 [플러그인 컴포넌트의 LSP 서버](/docs/ko/plugins/components#lsp-servers)를 참조하세요.

<h2 id="next-steps">
  다음 단계
</h2>

* [플러그인 컴포넌트의 LSP 서버](/docs/ko/plugins/components#lsp-servers): 공식 플러그인이 없는 언어 서버의 `.lsp.json` 작성
* [플러그인 설치 및 관리](/docs/ko/plugins/install): 범위, 업데이트 및 제거
* [플러그인 문제 해결](/docs/ko/plugins/troubleshooting): 이 페이지의 언어 서버 관련 항목을 넘어선 로드 오류
* [공식 마켓플레이스에서 플러그인 찾기](/docs/ko/plugins/anthropic-marketplaces#find-plugins-in-the-official-marketplace): 공식 마켓플레이스의 나머지를 찾아볼 수 있는 위치
