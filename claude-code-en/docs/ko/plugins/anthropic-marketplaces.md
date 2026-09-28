> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Anthropic의 마켓플레이스

> Claude Code를 위한 Anthropic의 공식, 커뮤니티, 데모 플러그인 마켓플레이스: 이름, 저장소, 각각을 추가하는 방법, 플러그인을 찾아보는 위치.

Anthropic은 Claude Code를 위한 세 가지 범용 플러그인 마켓플레이스를 발행합니다: [공식](https://github.com/anthropics/claude-plugins-official), [커뮤니티](https://github.com/anthropics/claude-plugins-community), [데모](https://github.com/anthropics/claude-code). 각각은 자신의 GitHub 저장소에 있는 플러그인 카탈로그입니다. Claude Code 세션에서 이 중 하나에서 플러그인을 설치할 때, `/plugin install commit-commands@claude-plugins-official`처럼 `@` 뒤에 마켓플레이스 이름을 입력합니다.

이 페이지를 사용하여 세 가지 마켓플레이스를 구분하고 공식 마켓플레이스에 특정 플러그인이 있는지 확인할 수 있는 위치를 찾습니다.

<Note>
  다음 경우는 다른 페이지에서 다룹니다:

  * **플러그인 설치 방법**: [플러그인 설치](/docs/ko/plugins/install) 참조
  * **설치 실패**: [플러그인 문제 해결](/docs/ko/plugins/troubleshooting) 참조
</Note>

필요한 페이지 부분으로 이동합니다:

* 저장소, 마켓플레이스 이름, 각각을 얻는 방법으로 세 가지 마켓플레이스를 구분하려면 [Anthropic의 마켓플레이스](#anthropic%E2%80%99s-marketplaces)를 참조합니다.
* 공식 마켓플레이스에서 플러그인을 찾으려면 [공식 마켓플레이스에서 플러그인 찾기](#find-plugins-in-the-official-marketplace)를 참조합니다.

<h2 id="anthropic’s-marketplaces">
  Anthropic의 마켓플레이스
</h2>

마켓플레이스는 저장소가 `.claude-plugin/marketplace.json` 파일에 정의하는 플러그인 카탈로그입니다. 공식, 커뮤니티, 데모 마켓플레이스는 각각 자신의 GitHub 저장소에서 제공됩니다. Anthropic은 또한 `anthropics/skills` 및 `anthropics/knowledge-work-plugins`와 같은 주제별 마켓플레이스를 발행하며, Claude Code 세션에서 `/plugin marketplace add <owner>/<repo>`로 추가합니다.

이 표는 각 마켓플레이스의 저장소와 마켓플레이스 이름을 제공하며, 이는 해당 마켓플레이스에서 플러그인을 설치할 때 `@` 뒤에 입력하는 것입니다. 커뮤니티 마켓플레이스의 이름은 저장소 이름이 아닌 `claude-community`입니다.

|           | 공식                                                                                                                                                                                                                                                                                                                | 커뮤니티                                                                                            | 데모                                                                                      |
| :-------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------- |
| 저장소       | [`anthropics/claude-plugins-official`](https://github.com/anthropics/claude-plugins-official)                                                                                                                                                                                                                     | [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community) | [`anthropics/claude-code`](https://github.com/anthropics/claude-code/tree/main/plugins) |
| 마켓플레이스 이름 | `claude-plugins-official`                                                                                                                                                                                                                                                                                         | `claude-community`                                                                              | `claude-code-plugins`                                                                   |
| 포함 내용     | Anthropic이 유지 관리하는 플러그인, 파트너 및 기타 작성자의 플러그인                                                                                                                                                                                                                                                                       | 작성자가 Anthropic에 제출한 타사 플러그인                                                                     | 플러그인이 포함할 수 있는 것을 보여주는 작은 예제 플러그인 세트                                                    |
| 얻는 방법     | Claude Code는 [관리 정책](/docs/ko/plugins/org#allow-the-official-marketplace-and-your-own)이나 `CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL`이 차단하지 않는 한 대화형 터미널 세션을 처음 시작할 때 추가합니다. 누락된 경우 [마켓플레이스 `claude-plugins-official` 찾을 수 없음](/docs/ko/plugins/troubleshooting#marketplace-claude-plugins-official-not-found) 참조 | Claude Code 세션에서 `/plugin marketplace add anthropics/claude-plugins-community`로 추가합니다           | Claude Code 세션에서 `/plugin marketplace add anthropics/claude-code`로 추가합니다                |

플러그인을 작성했고 다른 사람들이 설치하도록 하려면 [플러그인 발행](/docs/ko/plugins/publish)을 참조하십시오. 이는 자신의 마켓플레이스와 커뮤니티 마켓플레이스에 제출하는 것을 다룹니다.

<h3 id="the-demo-marketplace-in-anthropics/claude-code">
  `anthropics/claude-code`의 데모 마켓플레이스
</h3>

튜토리얼이나 이전 지침 세트에서 `/plugin marketplace add anthropics/claude-code`를 실행하도록 말하면, 이는 `claude-code-plugins`라는 데모 마켓플레이스를 추가합니다. 이는 Claude Code가 이미 추가한 공식 마켓플레이스가 아닙니다.

데모 마켓플레이스의 플러그인 대부분은 같은 이름으로 공식 마켓플레이스에도 있습니다. 예를 들어, `code-review`, `feature-dev`, `commit-commands`, `security-guidance`는 둘 다에 있습니다. 두 개의 복사본이 설치되지 않도록 `claude-plugins-official`에서 이들을 설치합니다.

<h2 id="find-plugins-in-the-official-marketplace">
  공식 마켓플레이스에서 플러그인 찾기
</h2>

공식 마켓플레이스인 `claude-plugins-official`은 Claude Code가 추가하는 것입니다. 나열된 대부분의 내용은 Anthropic이 아닌 파트너 및 기타 작성자에게서 제공됩니다: 도구 공급업체는 Claude Code를 자신의 서비스에 연결하는 플러그인을 발행하고, Anthropic은 `commit-commands`, `code-review`, `feature-dev` 및 [언어 서버 플러그인](/docs/ko/plugins/code-intelligence)과 같은 더 작은 자신의 세트를 유지 관리합니다. 카탈로그는 자주 변경되므로 이 페이지에는 나열되지 않습니다.

포함된 내용을 보려면 Claude Code 세션에서 `/plugin`의 **발견** 탭을 사용하십시오. 이를 검색하거나 웹에서 [Claude 마켓플레이스](https://claude.com/marketplace/plugins)를 찾아볼 수 있습니다.

<h2 id="browse-and-install-from-anthropic’s-marketplaces">
  Anthropic의 마켓플레이스에서 찾아보기 및 설치
</h2>

Claude Code, 웹 또는 GitHub에서 Anthropic의 마켓플레이스에서 플러그인을 검색할 수 있습니다:

* **Claude Code에서 찾아보기**: 대화형 세션에서 `/plugin`을 실행합니다. **발견** 탭에는 추가한 마켓플레이스의 플러그인이 나열됩니다.
* **Claude Code에서 이름으로**: 세션에서 `/plugin install <name>`을 실행하면, 추가한 마켓플레이스에서 이름을 조회합니다. 플러그인이 그 중 하나에 있으면 세부 정보가 `/plugin` 패널에서 열리고, [설치 범위](/docs/ko/plugins/install#install-a-plugin)를 선택하고 확인할 때까지 아무것도 설치되지 않습니다. 없으면 `Plugin "<name>" not found in any marketplace`가 표시됩니다.
* **웹에서**: [Claude 마켓플레이스](https://claude.com/marketplace/plugins)에서 전체 카탈로그를 검색하면, 설치 수를 표시하고 일부 플러그인을 **Anthropic 검증됨**으로 표시합니다.
* **GitHub에서**: [`anthropics/claude-plugins-official`](https://github.com/anthropics/claude-plugins-official)과 같은 마켓플레이스의 저장소에서 `.claude-plugin/marketplace.json`을 엽니다. 해당 파일이 카탈로그 자체입니다.

데스크톱 앱에서 또는 스크립트에서 설치하거나 클라우드 세션이 로드하는 것을 보려면 [플러그인 설치](/docs/ko/plugins/install)를 참조합니다.

<h3 id="add-the-community-or-demo-marketplace">
  커뮤니티 또는 데모 마켓플레이스 추가
</h3>

커뮤니티 및 데모 마켓플레이스는 Claude Code 세션에서 추가할 때까지 등록되지 않습니다:

* **커뮤니티**: `/plugin marketplace add anthropics/claude-plugins-community`를 실행한 다음 `@claude-community` 접미사로 설치합니다.
* **데모**: `/plugin marketplace add anthropics/claude-code`를 실행한 다음 `@claude-code-plugins` 접미사로 설치합니다.

`claude-plugins-official`이 `/plugin`의 **마켓플레이스** 탭에 없으면 `/plugin marketplace add anthropics/claude-plugins-official`로 같은 방식으로 추가합니다.

`not found` 오류 및 추가되지 않는 마켓플레이스는 [플러그인 문제 해결](/docs/ko/plugins/troubleshooting#install-a-plugin)을 참조합니다.

<h2 id="third-party-marketplaces">
  타사 마켓플레이스
</h2>

많은 인기 있는 플러그인은 Anthropic 마켓플레이스에 없습니다. 이들은 저자의 자체 마켓플레이스에 있으며, 보통 루트에 `.claude-plugin/marketplace.json`이 있는 GitHub 저장소입니다.

Anthropic은 타사 마켓플레이스를 검토하지 않으므로, 마켓플레이스를 추가하기 전에 [플러그인 보안 및 신뢰](/docs/ko/plugins/security)를 읽으십시오.

타사 마켓플레이스를 사용하려면 Claude Code 세션에서 `/plugin marketplace add <owner>/<repo>`를 사용하여 저장소를 추가한 다음, `/plugin install <plugin>@<marketplace-name>`으로 설치하십시오. 마켓플레이스 이름은 해당 `marketplace.json`의 `name` 필드이며, Claude Code는 마켓플레이스를 추가한 후 이를 출력합니다.

마켓플레이스를 추가하는 다른 방법은 [마켓플레이스 추가](/docs/ko/plugins/install#add-a-marketplace)를 참조하십시오.

<h2 id="next-steps">
  다음 단계
</h2>

* [플러그인 설치 및 관리](/docs/ko/plugins/install): 이 마켓플레이스 중 하나에서 플러그인을 설치하고 범위를 선택합니다
* [플러그인 보안 및 신뢰](/docs/ko/plugins/security): 플러그인이 컴퓨터에서 할 수 있는 것과 설치하기 전에 검토하는 방법
* [코드 인텔리전스 플러그인](/docs/ko/plugins/code-intelligence): 공식 마켓플레이스의 언어 서버 플러그인 중 하나를 설치합니다
* [마켓플레이스 만들기](/docs/ko/plugins/create-marketplace): Anthropic의 마켓플레이스와 함께 자신의 마켓플레이스를 실행합니다
