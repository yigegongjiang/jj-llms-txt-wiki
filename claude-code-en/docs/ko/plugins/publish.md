> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 플러그인 게시 및 배포

> Claude Code 플러그인을 자신의 마켓플레이스 또는 Anthropic의 커뮤니티 마켓플레이스를 통해 게시하고, 사전 릴리스 체크리스트 및 사용자가 업데이트를 받는 방법을 알아봅니다.

Claude Code 플러그인을 게시한다는 것은 플러그인을 나열하는 마켓플레이스(플러그인을 나열하고 각 플러그인을 가져올 위치를 지정하는 JSON 카탈로그)에 플러그인을 등록하는 것을 의미하므로, 다른 사람들이 이름으로 플러그인을 설치하고 업데이트를 받을 수 있습니다. 자신의 마켓플레이스를 운영하거나 플러그인을 Anthropic의 커뮤니티 마켓플레이스에 제출할 수 있습니다. 플러그인을 게시하지 않고 공유하려면 플러그인의 디렉터리 또는 `.zip` 파일을 사람들에게 보내서 직접 로드하도록 하면 됩니다.

이 페이지는 작동하는 플러그인을 작성한 저자가 이를 공유할 준비가 되었을 때를 위한 것입니다.

<Note>
  다음 경우는 다른 페이지에서 다룹니다:

  * **플러그인이 아직 완성되지 않았습니다**: [플러그인 만들기](/docs/ko/plugins/create)로 시작하세요
  * **공식 마켓플레이스에 플러그인이 있는 CLI 또는 SDK를 유지 관리합니다**: [CLI에서 플러그인 권장](/docs/ko/plugins/cli-hints)을 참조하세요
</Note>

[배포 방법 선택](#choose-how-to-distribute)으로 시작하여 배포 옵션을 비교하세요. 이미 경로를 알고 있다면 [릴리스를 위해 플러그인 준비](#prepare-your-plugin-for-release)로 이동한 후 사용자에게 알려야 할 사항과 사용자가 업데이트를 받는 방법에 대해 경로의 섹션을 따르세요.

<h2 id="choose-how-to-distribute">
  배포 방법 선택
</h2>

플러그인을 설치해야 하는 사람에 따라 배포 옵션을 선택하세요:

| 경로                                                             | 설치할 수 있는 사람                                       | 필요한 것                                                              | 사용자가 자동으로 업데이트를 받나요? |
| :------------------------------------------------------------- | :------------------------------------------------ | :----------------------------------------------------------------- | :------------------- |
| [마켓플레이스 없음](#share-a-plugin-without-a-marketplace)             | 플러그인 폴더 또는 `.zip` 파일을 받은 사람                       | 플러그인의 폴더                                                           | 없음. 보낸 복사본을 로드합니다    |
| [자신의 마켓플레이스](#publish-through-your-own-marketplace)            | 저장소에 접근할 수 있는 모든 사람(팀이 복제할 수 있는 비공개 저장소일 수 있음)    | `.claude-plugin/marketplace.json`이 있는 git 저장소 또는 기타 호스트(플러그인을 나열함) | 꺼짐                   |
| [Anthropic의 커뮤니티 마켓플레이스](#submit-to-the-community-marketplace) | `anthropics/claude-plugins-community`를 추가하는 모든 사람 | 플러그인 디렉터리 제출 양식을 통한 제출                                             | 꺼짐                   |

자동 업데이트는 사용자 측의 마켓플레이스별 설정으로, 백그라운드에서 새 버전을 가져옵니다.

<h2 id="prepare-your-plugin-for-release">
  릴리스를 위해 플러그인 준비
</h2>

이름, 버전, 유효성 검사 및 마켓플레이스에서의 설치는 릴리스가 설치하는 사람들을 위해 작동하는지 여부를 결정합니다. 첫 번째 릴리스 전에 그리고 이후 각 릴리스 전에 확인하세요.

<Steps>
  <Step title="영구적인 이름 선택">
    사용자는 `name@marketplace`로 플러그인을 설치, 활성화 및 구성하므로, 이름이 바뀐 플러그인은 기존의 모든 설치에 대해 다른 플러그인입니다. `deploy-helper`와 같은 kebab-case 이름을 선택하세요. `claude plugin validate`는 다른 형식에 대해 경고하고 이를 영구적으로 취급하기 때문입니다. 사용자가 보는 레이블을 위해 `plugin.json`에서 `displayName`을 설정하세요.
  </Step>

  <Step title="버전 관리 방법 결정">
    `plugin.json`에서 `version`을 설정하고 나중에 변경하지 않고 커밋을 푸시하면, `claude plugin update`는 `<name> is already at the latest version (1.0.0).`을 출력하고 사용자는 이전 복사본을 유지합니다. 모든 릴리스에서 `version`을 증가시키거나, git 호스팅 마켓플레이스에서 생략하여 Claude Code가 커밋 SHA를 대신 사용하도록 하세요. [버전 및 업데이트](/docs/ko/plugins/loading#versions-and-updates)를 참조하세요.
  </Step>

  <Step title="유효성 검사">
    셸에서 `claude plugin validate --strict ./your-plugin`을 실행하세요. 깨끗한 실행은 `✔ Validation passed`를 출력합니다.

    * **CI에서**: `--strict`를 유지하세요. 이는 또한 알 수 없는 매니페스트 필드 또는 누락된 `version`과 같은 경고에서 종료 코드 1로 실행을 실패합니다. 이전 단계에서 `version`을 생략하기로 선택한 경우 `--strict`를 제거하세요.
    * **경로**: 유효성 검사는 `./`로 시작하지 않는 구성 요소 경로를 보고합니다. hook 명령 및 MCP 서버 구성 내에서 파일을 `${CLAUDE_PLUGIN_ROOT}/...`로 참조하세요. [경로 규칙](/docs/ko/plugins/manifest-reference#path-rules)을 참조하세요.
  </Step>

  <Step title="로컬 마켓플레이스에서 설치">
    셸에서 `claude plugin marketplace add ./path-to-marketplace`로 플러그인을 나열하는 로컬 마켓플레이스를 추가하고, 플러그인을 설치한 후 세션을 시작하여 로드되는지 확인하세요.

    * 작동하는 가장 작은 마켓플레이스는 [마켓플레이스 만들기](/docs/ko/plugins/create-marketplace)를 참조하세요.
    * 설치가 소스 디렉터리를 로드하는지 아니면 캐시된 복사본을 로드하는지 알아보려면 [제자리 및 복사된 플러그인](/docs/ko/plugins/loading#in-place-and-copied-plugins)을 참조하세요.
  </Step>

  <Step title="사용자가 보는 메타데이터 채우기">
    `plugin.json`에서 `description`, `author`, `homepage` 및 `repository`를 설정하고, 플러그인 루트에 `README.md`를 추가하세요. `homepage`는 URL로 구문 분석되어야 합니다. [매니페스트 참조](/docs/ko/plugins/manifest-reference#fields)는 모든 필드를 나열합니다.
  </Step>

  <Step title="eval 스위트 실행">
    eval 스위트가 있으면 셸에서 `claude plugin eval`을 실행하세요. 플러그인의 테스트 케이스를 실행하고 결과를 점수 매기므로, 플러그인을 변경할 때 회귀를 포착합니다. [eval로 플러그인 테스트](/docs/ko/plugin-evals)를 참조하세요.
  </Step>
</Steps>

<h2 id="share-a-plugin-without-a-marketplace">
  마켓플레이스 없이 플러그인 공유
</h2>

플러그인이 git 저장소에 있으면 사람들이 복제하고 체크아웃을 로드하거나, 릴리스에 첨부한 `.zip`을 가리키는 `--plugin-url`로 셸에서 Claude Code를 시작할 수 있습니다. 다음 버전을 얻으려면 풀하거나 다시 다운로드하면 됩니다. 저장소에 없으면 디렉터리 또는 `.zip` 파일을 보내세요. 다음 두 가지 방법 중 하나로 로드합니다:

* **한 세션의 경우**: 셸에서 `claude --plugin-dir ./deploy-helper`로 Claude Code를 시작합니다. 여기서 경로는 복제, 압축 해제된 폴더 또는 `.zip` 파일 자체입니다. [한 세션 동안 플러그인을 로드하는 플래그](/docs/ko/plugins/cli-reference#flags-that-load-a-plugin-for-one-session)를 참조하세요.
* **모든 세션의 경우**: 플러그인 디렉터리를 `.claude-plugin/plugin.json`과 함께 `~/.claude/skills/` 아래로 이동하여 Claude Code가 [모든 세션에서 로드](/docs/ko/plugins/loading#find-where-a-plugin-came-from)하도록 합니다.

동일한 저장소에 `.claude-plugin/marketplace.json`을 추가하면 사람들이 이름으로 설치하고 명령으로 업데이트할 수 있습니다. [자신의 마켓플레이스를 통해 게시](#publish-through-your-own-marketplace)를 참조하세요.

<h3 id="ship-a-plugin-with-your-own-tool">
  자신의 도구와 함께 플러그인 배포
</h3>

CLI 또는 SDK를 유지 관리하는 경우 마켓플레이스에 플러그인을 게시하고 설치 관리자 또는 설치 후 메시지가 사용자가 필요로 하는 두 명령을 실행하거나 출력하도록 합니다: `claude plugin marketplace add <source>`, 그 다음 `claude plugin install <name>@<marketplace>`. 누군가 도구를 사용할 때 세션 내 검색의 경우 [CLI에서 플러그인 권장](/docs/ko/plugins/cli-hints)을 참조하세요.

<h2 id="publish-through-your-own-marketplace">
  자신의 마켓플레이스를 통해 게시
</h2>

자신의 마켓플레이스는 플러그인을 나열하는 `.claude-plugin/marketplace.json` 파일로, git 저장소에 추가됩니다. 파일이 저장소에 있으면 플러그인이 게시되며, 제출 양식이 없습니다. 파일을 플러그인의 자체 저장소 또는 별도의 저장소에 보관할 수 있습니다.

<h3 id="add-the-marketplace-file-to-your-repository">
  저장소에 마켓플레이스 파일 추가
</h3>

플러그인의 자체 저장소에서 게시하려면 마켓플레이스 파일을 `.claude-plugin/`의 `plugin.json` 옆에 저장하고, `source`가 `"./"`(저장소 루트)인 항목 하나를 포함합니다. 항목에 `plugin.json`과 동일한 `name`을 지정하세요. [항목 이름과 매니페스트 이름을 동일하게 유지](/docs/ko/plugins/create-marketplace#keep-the-entry-name-and-the-manifest-name-the-same)를 참조하세요:

```json .claude-plugin/marketplace.json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Name" },
  "plugins": [
    { "name": "deploy-helper", "source": "./" }
  ]
}
```

셸에서 저장소의 `claude plugin validate .`을 실행하여 푸시하기 전에 파일을 확인하세요.

[마켓플레이스 만들기](/docs/ko/plugins/create-marketplace)는 한 저장소에 여러 플러그인이 있는 레이아웃을 다룹니다.

<h3 id="control-who-can-install">
  설치할 수 있는 사람 제어
</h3>

저장소를 복제할 수 있는 모든 사람이 설치할 수 있으므로, 저장소가 비공개이면 마켓플레이스도 비공개입니다. git 저장소 이외의 호스트의 경우 [마켓플레이스 호스팅](/docs/ko/plugins/host-marketplace)을 참조하세요. 회사의 모든 사람(git을 사용하지 않는 사람 포함)에게 도달하려면 [전체 회사에 배포](/docs/ko/plugins/host-marketplace#roll-out-to-a-whole-company)를 참조하세요.

<h3 id="tell-users-how-to-install">
  사용자에게 설치 방법 알리기
</h3>

사용자에게 마켓플레이스를 추가한 후 셸에서 플러그인을 설치하도록 알리고, 소스 및 이름을 자신의 것으로 바꾸세요:

* 마켓플레이스를 한 번 추가: `claude plugin marketplace add your-org/your-marketplace`. 인수는 GitHub `owner/repo` 약자, URL 또는 경로입니다
* 플러그인 설치: `claude plugin install deploy-helper@your-marketplace`
* 또는 세션 내에서 둘 다 수행: `/plugin install deploy-helper --marketplace your-org/your-marketplace`. Claude Code v2.1.275 이상이 필요합니다. [한 명령으로 마켓플레이스 추가 및 설치](/docs/ko/plugins/install#add-a-marketplace-and-install-in-one-command)를 참조하세요

<h3 id="ship-updates-to-users">
  사용자에게 업데이트 배포
</h3>

사용자는 요청할 때 또는 마켓플레이스에 대해 자동 업데이트가 켜져 있을 때 릴리스를 받습니다:

* **요청 시**: 사용자의 셸에서 `claude plugin update deploy-helper@your-marketplace`는 마켓플레이스를 새로 고치고 플러그인의 버전이 변경되었을 때 새 복사본을 설치합니다
* **자동 업데이트**: 마켓플레이스의 경우 기본적으로 꺼져 있습니다. [자동 업데이트 켜기](/docs/ko/plugins/host-marketplace#turn-on-auto-update)를 참조하세요. 켜지면 세션 시작 후 지연 후 `claude plugin update`와 동일한 작업을 수행합니다

[플러그인 설치](/docs/ko/plugins/install)는 사용자 측 명령을 다루고, [자동 업데이트가 실행되는 시기](/docs/ko/plugins/loading#when-auto-update-runs)는 타이밍을 다룹니다.

<h2 id="submit-to-the-community-marketplace">
  커뮤니티 마켓플레이스에 제출
</h2>

Anthropic의 커뮤니티 마켓플레이스인 `claude-community`는 플러그인 디렉터리 제출 양식을 통해 제출된 플러그인을 나열하는 공개 마켓플레이스입니다.

사용자는 Claude Code 세션에서 `/plugin marketplace add anthropics/claude-plugins-community`로 커뮤니티 마켓플레이스를 추가하고 `@claude-community`로 설치합니다.

커뮤니티 마켓플레이스가 공식 마켓플레이스와 어떻게 다른지는 [Anthropic의 마켓플레이스](/docs/ko/plugins/anthropic-marketplaces)를 참조하세요.

플러그인을 커뮤니티 마켓플레이스에 제출하려면 다음 앱 내 양식 중 하나를 사용하세요:

* **claude.ai**: [claude.ai/admin-settings/directory/submissions/plugins/new](https://claude.ai/admin-settings/directory/submissions/plugins/new)
* **Console**: [platform.claude.com/plugins/submit](https://platform.claude.com/plugins/submit)

claude.ai 양식에는 Team 또는 Enterprise 조직과 Directory 권한(기본적으로 소유자가 보유)이 필요합니다. Team 또는 Enterprise 조직에 속하지 않은 개별 저자는 Console 양식을 대신 사용할 수 있습니다.

셸에서 제출하기 전에 `claude plugin validate ./your-plugin`을 로컬로 실행하고, `./your-plugin`을 플러그인 디렉터리의 경로로 바꾸세요. 유효성 검사가 통과하면 Claude Code는 `✔ Validation passed` 또는 경고가 있으면 `✔ Validation passed with warnings`를 출력합니다. 경고는 유효성 검사를 실패하지 않습니다. `--strict`를 추가하여 경고를 오류로 취급하세요.

나열된 플러그인은 [`anthropics/claude-plugins-community`](https://github.com/anthropics/claude-plugins-community) 카탈로그에 나타나며, 거의 모든 경우 특정 커밋 SHA에 고정됩니다.

제출과 플러그인이 `marketplace.json`에 나타나는 사이에 지연이 있을 수 있습니다. 플러그인이 아직 설치 가능한지 확인하려면 [커뮤니티 카탈로그](https://github.com/anthropics/claude-plugins-community/blob/main/.claude-plugin/marketplace.json)에서 이름을 검색하세요.

공식 마켓플레이스인 `claude-plugins-official`은 이러한 양식을 통한 제출을 받지 않습니다. Anthropic 파트너 담당자와 함께 일하면 공식 마켓플레이스 목록에 대해 물어보세요.

<h2 id="ship-updates-renames-and-removals">
  업데이트, 이름 바꾸기 및 제거 배포
</h2>

<h3 id="release-a-new-version">
  새 버전 릴리스
</h3>

자신의 마켓플레이스를 통해 게시하고 `plugin.json`이 `version`을 설정하면 증가시키고 푸시하세요. `claude plugin update`를 실행하거나 자동 업데이트가 켜져 있는 사용자는 [사용자에게 업데이트 배포](#ship-updates-to-users)에서 설명한 대로 새 버전을 받습니다.

<h3 id="tag-a-release">
  릴리스 태그 지정
</h3>

다른 플러그인이 버전 범위를 선언할 때 git에서 릴리스를 태그하세요. 이러한 범위는 태그에 대해 해결되기 때문입니다. 그렇지 않으면 태그가 필요하지 않습니다.

태그를 지정하려면 플러그인 디렉터리에서 셸의 `claude plugin tag`를 실행하세요. `{name}--v{version}` 태그를 만듭니다. `--push`를 추가하여 태그를 `origin`으로 보내세요. [`plugin tag` 참조](/docs/ko/plugins/cli-reference#plugin-tag)는 플래그를 나열합니다.

<h3 id="rename-or-remove-a-plugin">
  플러그인 이름 바꾸기 또는 제거
</h3>

게시된 플러그인의 `name`을 절대 변경하지 마세요. 이름을 바꾼 후 이미 설치한 사용자는 플러그인을 잃습니다. 설치가 이전 이름으로 기록되기 때문입니다. 마켓플레이스 파일의 `renames` 항목이 대신 마이그레이션합니다. 다른 레이블을 원할 때 `displayName`을 변경하세요.

이름 바꾸기가 불가피한 경우 마켓플레이스 파일의 `renames` 맵을 사용하여 기존 설치가 [`Plugin "<name>" not found in marketplace`](/docs/ko/plugins/troubleshooting#plugin-not-found-in-marketplace)로 실패하는 대신 마이그레이션하도록 합니다. 마켓플레이스에서 플러그인을 제거하거나 전체 `renames` 세부 정보는 호스팅 페이지의 [플러그인 이름 바꾸기 또는 제거](/docs/ko/plugins/host-marketplace#rename-or-remove-a-plugin)를 참조하세요. [마켓플레이스 참조](/docs/ko/plugins/marketplace-reference#top-level-fields)에 필드가 있습니다.

<h2 id="declare-dependencies">
  종속성 선언
</h2>

플러그인이 동일한 마켓플레이스의 다른 플러그인을 활성화해야 하는 경우 `plugin.json`의 `dependencies` 배열에 나열하세요. 각 항목은 베어 이름 또는 semver `version` 범위가 있는 객체입니다. 사용자가 플러그인을 설치하면 Claude Code가 종속성도 설치하고 활성화합니다.

[플러그인 종속성](/docs/ko/plugins/dependencies)은 범위 구문, 교차 마켓플레이스 종속성 및 사용자가 더 이상 필요하지 않은 종속성을 정리하는 방법을 다룹니다.

<h2 id="next-steps">
  다음 단계
</h2>

* [마켓플레이스 호스팅 및 유지 관리](/docs/ko/plugins/host-marketplace): 새 버전을 릴리스하고 사용자를 최신 상태로 유지합니다
* [플러그인 종속성](/docs/ko/plugins/dependencies): 플러그인이 의존하는 플러그인을 선언하고 버전 관리합니다
* [CLI에서 플러그인 권장](/docs/ko/plugins/cli-hints): CLI 사용자에게 플러그인을 설치하도록 Claude Code 사용자에게 메시지를 표시합니다
* [플러그인 비용 및 사용량 측정](/docs/ko/plugins/measure): 플러그인이 컨텍스트에서 비용이 얼마나 드는지 확인하고 사람들이 사용하는지 확인합니다
