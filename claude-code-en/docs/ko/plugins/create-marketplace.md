> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 마켓플레이스 만들기

> marketplace.json 파일에서 플러그인 마켓플레이스를 구축하고 호스팅하기 전에 로컬에서 테스트합니다.

플러그인 마켓플레이스는 플러그인을 나열하고 각 플러그인을 가져올 위치를 지정하는 `.claude-plugin/marketplace.json` 파일이 있는 디렉터리 또는 저장소입니다. 디렉터리를 git 호스트에 푸시하면 액세스 권한이 있는 모든 사용자가 한 명령으로 Claude Code에 등록하고 카탈로그에서 플러그인을 설치할 수 있습니다.

팀이나 조직과 같이 선택한 그룹이 플러그인을 설치하고 제어하는 카탈로그에서 계속 업데이트를 받도록 하려면 자신의 마켓플레이스를 만듭니다. 저장소는 비공개일 수 있으며, 원하는 만큼 많은 플러그인을 나열할 수 있으며, 관리자는 [모든 머신에서 이를 요구](/docs/ko/plugins/org)할 수 있습니다.

<Note>
  다음 경우는 다른 페이지에서 다룹니다:

  * **한 플러그인을 몇 명과 공유**: 플러그인의 디렉터리 또는 `.zip` 파일을 보냅니다. [마켓플레이스 없이 플러그인 공유](/docs/ko/plugins/publish#share-a-plugin-without-a-marketplace)를 참조하세요.
  * **모든 사람에게 플러그인 제공**: Anthropic의 커뮤니티 마켓플레이스에 제출합니다. [커뮤니티 마켓플레이스에 제출](/docs/ko/plugins/publish#submit-to-the-community-marketplace)을 참조하세요.
  * **플러그인을 직접 사용**: `--plugin-dir`로 로드하거나 skills 디렉터리에 저장합니다. [마켓플레이스 없이 개발](/docs/ko/plugins/create#develop-without-a-marketplace)을 참조하세요.
</Note>

[마켓플레이스 만들기](#create-a-marketplace)로 시작하여 자신의 머신에서 마켓플레이스를 구축하고 플러그인을 설치한 다음, [더 많은 플러그인 항목을 추가](#add-plugin-entries)합니다.

<h2 id="create-a-marketplace">
  마켓플레이스 만들기
</h2>

다음 단계는 머신에서 마켓플레이스를 만들고, 플러그인을 추가하고, Claude Code에 등록하고, 마켓플레이스에서 플러그인을 설치합니다. 이것이 전체 루프이며, 마켓플레이스를 호스팅한 후 사용자가 거치는 루프와 동일합니다. `my-marketplace/`를 만들려는 디렉터리에서 셸의 모든 명령을 실행합니다.

나열할 플러그인이 필요합니다. 예제는 [첫 번째 플러그인 만들기](/docs/ko/plugins/create#create-your-first-plugin)의 `my-first-plugin`을 사용합니다. 이는 `/my-first-plugin:hello`로 실행하는 하나의 skill이 있는 플러그인입니다. 아직 플러그인이 없으면 먼저 빌드합니다. 대신 자신의 플러그인을 사용하려면 단계에서 `my-first-plugin`이라고 하는 곳마다 해당 디렉터리와 `name`을 대체합니다. 플러그인 디렉터리에 포함될 수 있는 내용은 [플러그인 디렉터리 탐색기](/docs/ko/plugins/components#explore-the-plugin-directory)를 참조하세요.

<Steps>
  <Step title="마켓플레이스 디렉터리 설정">
    마켓플레이스는 `.claude-plugin/marketplace.json` 파일이 있는 디렉터리이며, 나열하는 플러그인도 포함합니다. 마켓플레이스 디렉터리와 `.claude-plugin/` 폴더를 만든 다음 플러그인을 `plugins/` 아래에 복사합니다:

    ```bash theme={null}
    mkdir -p my-marketplace/.claude-plugin my-marketplace/plugins
    cp -r my-first-plugin my-marketplace/plugins/
    ```

    플러그인이 현재 위치에서 유효한지 확인하여 나중의 오류가 마켓플레이스가 아닌 플러그인에 대한 것이 아닌지 확인합니다:

    ```bash theme={null}
    claude plugin validate ./my-marketplace/plugins/my-first-plugin
    ```

    출력의 마지막 줄은 `✔ Validation passed`입니다.
  </Step>

  <Step title="마켓플레이스 파일 만들기">
    `marketplace.json`을 `my-marketplace/.claude-plugin/marketplace.json`에 저장합니다. 파일에는 `name`, `owner`, `plugins` 배열이 필요합니다.

    `plugins`의 각 객체는 플러그인 항목이며 `name`과 `source`가 필요합니다. 항목의 `source`를 마켓플레이스 루트에서의 경로로 작성합니다. 루트는 `.claude-plugin/`을 포함하는 디렉터리인 `my-marketplace/`입니다.

    ```json my-marketplace/.claude-plugin/marketplace.json theme={null}
    {
      "name": "my-marketplace",
      "description": "Plugins for my team",
      "owner": {
        "name": "Your Name"
      },
      "plugins": [
        {
          "name": "my-first-plugin",
          "source": "./plugins/my-first-plugin",
          "description": "A greeting plugin to learn the basics"
        }
      ]
    }
    ```
  </Step>

  <Step title="마켓플레이스 검증">
    마켓플레이스 디렉터리에서 `claude plugin validate`를 실행하여 JSON 구문, 필수 필드, `.claude-plugin/marketplace.json`의 각 플러그인 항목을 확인합니다.

    ```bash theme={null}
    claude plugin validate ./my-marketplace
    ```

    2단계에서 작성한 파일의 경우 출력의 마지막 줄은 `✔ Validation passed`입니다.
  </Step>

  <Step title="마켓플레이스 추가 및 플러그인 설치">
    디렉터리를 마켓플레이스로 등록합니다.

    ```bash theme={null}
    claude plugin marketplace add ./my-marketplace
    ```

    명령은 `✔ Successfully added marketplace: my-marketplace (declared in user settings)`를 출력합니다. 이는 마켓플레이스가 사용자 설정 파일에 기록되었음을 의미합니다.

    플러그인을 설치합니다. 설치 ID는 항목의 `name`, `@`, 마켓플레이스 `name`입니다.

    ```bash theme={null}
    claude plugin install my-first-plugin@my-marketplace
    ```

    명령은 `✔ Successfully installed plugin: my-first-plugin@my-marketplace (scope: user)`를 출력합니다.

    세션 내에서 `/plugin marketplace add ./my-marketplace`는 마켓플레이스를 동일한 방식으로 등록합니다. `/plugin install my-first-plugin@my-marketplace`는 플러그인의 세부 정보를 `/plugin` 패널에서 열며, 여기서 플러그인을 설치합니다. 해당 흐름은 [플러그인 설치 및 관리](/docs/ko/plugins/install)를 참조하세요.
  </Step>

  <Step title="플러그인이 로드되었는지 확인">
    설치된 플러그인을 나열합니다.

    ```bash theme={null}
    claude plugin list
    ```

    출력은 `Status: ✔ enabled`와 함께 `my-first-plugin@my-marketplace`를 나열합니다.

    플러그인이 로드한 내용을 보려면 세부 정보를 표시합니다.

    ```bash theme={null}
    claude plugin details my-first-plugin
    ```

    `Component inventory` 섹션은 `Skills (1)  hello`를 읽습니다.

    skill을 실행하려면 세션을 시작하고 `/my-first-plugin:hello`를 입력합니다. Claude가 인사합니다. 명령은 플러그인의 이름을 접두사로 가지며, 모든 플러그인 skill의 이름도 마찬가지입니다.
  </Step>
</Steps>

<h2 id="add-plugin-entries">
  플러그인 항목 추가
</h2>

배포하는 모든 플러그인은 `marketplace.json`의 `plugins` 배열의 하나의 객체입니다. 두 번째 플러그인을 추가하려면 두 번째 객체를 추가합니다. 이 필드는 대부분의 항목을 다룹니다:

* `name`: 설치할 때 `@` 앞에 입력하는 식별자입니다. 공백을 포함할 수 없습니다.
* `source`: Claude Code가 플러그인을 가져오는 위치입니다. [연습](#create-a-marketplace)에서처럼 마켓플레이스 디렉터리 내의 플러그인에 대해 상대 경로 문자열을 작성하거나, 외부의 플러그인에 대해 소스 객체를 작성합니다. [플러그인 소스 선택](#choose-a-plugin-source)을 참조하세요.
* `description`: 사용자가 `/plugin`에서 마켓플레이스를 탐색할 때 플러그인 옆에 표시되는 줄입니다.

전체 필드 목록은 [플러그인 항목](/docs/ko/plugins/marketplace-reference#plugin-entries)을 참조하세요.

항목은 또한 모든 [`plugin.json`](/docs/ko/plugins/manifest-reference) 필드를 설정할 수 있습니다. 항목의 `plugin.json` 필드가 자신의 `plugin.json`을 가진 플러그인에 적용되는 경우는 [항목 및 plugin.json](/docs/ko/plugins/marketplace-reference#entry-and-plugin-json)을 참조하세요.

<h2 id="rules-for-plugin-entries">
  플러그인 항목 규칙
</h2>

새 마켓플레이스에서 대부분의 설치 실패는 잘못된 디렉터리에서 작성된 상대 경로 또는 플러그인의 `plugin.json`의 `name`과 다른 항목 이름으로 인해 발생합니다.

<h3 id="write-relative-paths-from-the-marketplace-root">
  마켓플레이스 루트에서 상대 경로 작성
</h3>

마켓플레이스 루트는 `.claude-plugin/`을 포함하는 디렉터리입니다. [연습](#create-a-marketplace)에서는 `my-marketplace/`이므로 항목의 `source`는 `"./plugins/my-first-plugin"`입니다. 경로는 `.claude-plugin/` 내부에서 시작하지 않으므로 `..`를 사용하여 나가지 마세요.

`..`가 있는 경로와 누락된 디렉터리에 대한 경로는 다른 명령에서 실패합니다:

* **`..`가 있는 경로**: `claude plugin validate`는 항목을 유효하지 않은 것으로 보고합니다. 메시지는 `Path contains "..": ./../plugins/my-first-plugin`으로 시작합니다.
* **존재하지 않는 디렉터리에 대한 경로**: `claude plugin validate`는 통과합니다. `claude plugin install`은 `Source path does not exist: <path>`로 실패하며, `<path>`는 Claude Code가 확인한 절대 위치입니다.

<h3 id="keep-the-entry-name-and-the-manifest-name-the-same">
  항목 이름과 매니페스트 이름을 동일하게 유지
</h3>

마켓플레이스 플러그인은 `marketplace.json`의 항목 `name`과 자신의 `plugin.json`의 `name`을 가지며, 이를 매니페스트 이름이라고 합니다. 각 이름은 다른 위치에 나타납니다:

* **항목 이름**: 설치 ID인 `<entry-name>@<marketplace>`입니다. 사용자가 설치하기 위해 입력하는 것, `claude plugin list`가 표시하는 것, Claude Code가 설정 파일의 [`enabledPlugins`](/docs/ko/settings-reference#enabledplugins) 아래에 작성하는 키입니다.
* **매니페스트 이름**: 플러그인의 skill의 접두사이며, `claude plugin details`가 사용하는 이름입니다.

두 이름이 다르고 누군가 매니페스트 이름으로 설치할 때 Claude Code는 `Plugin "<manifest-name>" not found in marketplace "<marketplace>"`를 보고합니다. 두 이름을 동일하게 유지합니다. Claude Code가 두 이름을 사용하는 방법에 대한 자세한 내용은 [플러그인 로딩 참조](/docs/ko/plugins/loading#find-where-a-plugin-came-from)를 참조하세요.

<h2 id="choose-a-plugin-source">
  플러그인 소스 선택
</h2>

`marketplace.json`의 각 플러그인 항목에는 Claude Code에 해당 플러그인을 가져올 위치를 알려주는 `source`가 있습니다. 플러그인의 파일이 저장된 위치에 따라 소스를 선택합니다. 표는 대부분의 마켓플레이스 소유자가 사용하는 소스를 나열합니다.

| 소스           | 사용 시기                           | 최소 `source` 값                                                                             |
| :----------- | :------------------------------ | :---------------------------------------------------------------------------------------- |
| 상대 경로        | 플러그인의 파일이 마켓플레이스 디렉터리 내부에 있음    | `"./plugins/my-first-plugin"`                                                             |
| `github`     | 플러그인이 자신의 GitHub 저장소임           | `{ "source": "github", "repo": "your-org/my-first-plugin" }`                              |
| `git-subdir` | 플러그인이 모노레포와 같은 다른 저장소의 하위 디렉터리임 | `{ "source": "git-subdir", "url": "your-org/monorepo", "path": "tools/my-first-plugin" }` |

`git-subdir` 소스에서 `url`은 git URL 또는 `owner/repo` GitHub 약식을 사용합니다.

플러그인은 또한 다음 소스 유형 중 하나에서 올 수 있습니다:

* `url`: 모든 호스트의 git 저장소 URL
* `archive`: HTTPS를 통해 다운로드한 zip 파일
* `npm`: npm 패키지
* `command`: 플러그인이 설치된 머신에서 명령을 실행하여 생성된 디렉터리

모든 소스 유형의 필드, 그리고 git 기반 소스를 `ref` 또는 `sha`에 고정하는 방법은 [플러그인 소스](/docs/ko/plugins/marketplace-reference#plugin-sources)를 참조하세요.

<h2 id="validate-and-test">
  검증 및 테스트
</h2>

플러그인을 추가할 때마다 편집 후 셸에서 `claude plugin validate ./my-marketplace`를 실행하고, 공유하기 전에 자신의 머신에서 마켓플레이스에서 설치합니다. 검증과 설치는 다른 문제를 포착합니다.

<h3 id="problems-that-validation-reports">
  검증이 보고하는 문제
</h3>

`claude plugin validate`는 마켓플레이스 디렉터리 내의 파일만 읽습니다. 다음을 보고합니다:

* JSON 구문 오류, `json: Invalid JSON syntax: <reason>`로
* `owner: Invalid input`과 같은 필수 필드 누락
* 공백, 비ASCII 문자, 또는 `claude-official`과 같은 공식 Anthropic 마켓플레이스를 모방하는 형식의 마켓플레이스 이름
* `..`를 포함하는 상대 `source`
* 최상위 또는 플러그인 항목의 알 수 없는 필드, 경고로
* 상대 경로 플러그인의 `plugin.json`의 문제, `plugins[N] plugin.json → <field>: <message>`로

`validate`가 인쇄할 수 있는 모든 메시지는 [검증 메시지](/docs/ko/plugins/marketplace-reference#validation-messages)를 참조하세요. 플래그 및 종료 코드는 [`plugin validate`](/docs/ko/plugins/cli-reference#plugin-validate)를 참조하세요.

<h3 id="problems-that-surface-when-you-add-or-install">
  마켓플레이스를 추가하거나 설치할 때 나타나는 문제
</h3>

`claude plugin validate`가 보고하지 않는 문제는 마켓플레이스를 추가하거나 설치할 때 나타납니다:

* **마켓플레이스를 추가할 때**: 정확한 [공식 마켓플레이스 이름](/docs/ko/plugins/marketplace-reference#reserved-names), 예를 들어 `claude-plugins-official`은 검증을 통과합니다. 이러한 이름 중 하나로 마켓플레이스를 추가할 때 Claude Code는 `The name '<name>' is reserved for official Anthropic marketplaces`로 시작하는 메시지로 거부합니다.
* **플러그인을 설치할 때**:
  * Claude Code는 플러그인을 설치할 때 먼저 `github`, `git-subdir` 또는 다른 원격 소스를 가져오므로 잘못된 `repo` 또는 `path`가 그때 나타납니다.
  * 존재하지 않는 디렉터리의 상대 `source`도 `Source path does not exist: <path>`로 설치에서 실패합니다.

<h3 id="test-an-edit-to-a-plugin">
  플러그인 편집 테스트
</h3>

[연습](#create-a-marketplace)에서 상대 경로 `source`가 있는 로컬 디렉터리에서 `my-marketplace`를 추가했습니다. 이 설정으로 Claude Code는 `my-marketplace/plugins/`에서 플러그인의 파일을 직접 읽습니다. 편집은 다음 세션 시작 또는 세션에서 `/reload-plugins`를 실행할 때 적용되며, 플러그인의 `version`에는 변경이 없습니다.

호스팅된 마켓플레이스에서 설치하는 사람들은 대신 플러그인 캐시에 복사본을 받습니다. 새 버전을 받는 방법은 [사용자를 최신 상태로 유지](/docs/ko/plugins/host-marketplace#keep-users-up-to-date)를 참조하세요.

<h3 id="remove-the-marketplace-to-start-over">
  마켓플레이스를 제거하여 다시 시작
</h3>

모든 것을 제거하고 다시 시작하려면 셸에서 `claude plugin marketplace remove my-marketplace`를 실행합니다. 명령은 마켓플레이스와 해당 플러그인을 제거합니다.

<h2 id="host-your-marketplace">
  마켓플레이스 호스팅
</h2>

[마켓플레이스 만들기](#create-a-marketplace)에서처럼 자신의 머신에서 마켓플레이스에서 플러그인을 설치할 수 있으면 마켓플레이스 디렉터리를 git 호스트에 푸시합니다.

팀원들은 GitHub 저장소의 경우 셸에서 `claude plugin marketplace add <owner>/<repo>`를 실행하거나 저장소 URL과 함께 동일한 명령을 실행합니다. 그런 다음 [연습](#create-a-marketplace)에서처럼 이름으로 플러그인을 설치합니다.

비공개 저장소 액세스, 업데이트, 버전 관리, 항목 이름 변경 또는 제거는 [마켓플레이스 호스팅 및 유지](/docs/ko/plugins/host-marketplace)를 참조하세요.

<h2 id="next-steps">
  다음 단계
</h2>

* [마켓플레이스 호스팅 및 유지](/docs/ko/plugins/host-marketplace): 호스트를 선택하고, 사용자를 최신 상태로 유지하고, 플러그인을 안전하게 이름 변경 또는 제거합니다
* [마켓플레이스 참조](/docs/ko/plugins/marketplace-reference): `marketplace.json` 필드 및 소스 유형
* [조직을 위한 플러그인 관리](/docs/ko/plugins/org): 모든 머신에서 마켓플레이스 및 해당 플러그인을 요구합니다
* [관련성별로 플러그인 제안](/docs/ko/plugins/relevance): 세션이 일치할 때 Claude Code가 마켓플레이스에서 플러그인을 제안하도록 합니다
