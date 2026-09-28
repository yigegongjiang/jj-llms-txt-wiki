> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 마켓플레이스 호스팅 및 유지 관리

> 사용자가 접근할 수 있는 플러그인 마켓플레이스를 게시하고, 비공개 마켓플레이스에 대한 액세스를 부여하며, 설치를 중단하지 않고 업데이트 및 이름 변경을 릴리스합니다.

마켓플레이스를 호스팅한다는 것은 `marketplace.json` 카탈로그를 다른 사람들이 `/plugin marketplace add`로 추가할 수 있고, 플러그인을 설치할 수 있으며, 푸시 후에도 계속해서 변경 사항을 받을 수 있는 위치에 배치하는 것을 의미합니다.

이 페이지는 마켓플레이스를 운영하는 사람을 위한 것입니다.

<Note>
  다음 경우는 다른 페이지에서 다룹니다:

  * **카탈로그 파일을 아직 작성하지 않았습니다**: [마켓플레이스 생성](/docs/ko/plugins/create-marketplace)으로 시작하세요
  * **조직의 머신 전체에서 마켓플레이스를 요구, 제한 또는 사전 설치하는 관리자입니다**: [조직을 위한 플러그인 관리](/docs/ko/plugins/org)를 읽으세요
</Note>

[마켓플레이스 호스팅](#host-your-marketplace)으로 시작하여 호스트와 사용자가 실행할 명령을 선택하세요. 첫 번째 릴리스 전에 [사용자를 최신 상태로 유지](#keep-users-up-to-date)를 읽으세요. 플러그인의 `name`을 변경하기 전에 [플러그인 이름 변경 또는 제거](#rename-or-remove-a-plugin)를 읽으세요.

<h2 id="host-your-marketplace">
  마켓플레이스 호스팅
</h2>

GitHub, 다른 git 호스트, 호스팅된 `marketplace.json` URL 또는 공유 파일 시스템의 디렉터리에서 마켓플레이스를 호스팅할 수 있습니다. 사용자에게 호스트에 대한 추가 명령과 머신에 필요한 것을 알려주세요:

| 호스트                                                       | Claude Code 세션에서 사용자가 실행                                               | 사용자가 필요한 것                                                                                  |
| :-------------------------------------------------------- | :--------------------------------------------------------------------- | :------------------------------------------------------------------------------------------ |
| GitHub                                                    | `/plugin marketplace add your-org/your-marketplace`                    | `git`, 비공개 저장소의 경우 [비공개 마켓플레이스에 대한 액세스 부여](#grant-access-to-a-private-marketplace)에 설명된 액세스 |
| GitLab, Bitbucket, GitHub Enterprise Server 또는 다른 git 호스트 | `/plugin marketplace add https://gitlab.example.com/team/plugins.git`  | `git`, 머신에서 호스트에 대한 액세스. `owner/repo` 약식은 항상 github.com을 의미하므로 전체 URL을 보내세요                 |
| 호스팅된 `marketplace.json` URL                               | `/plugin marketplace add https://plugins.example.com/marketplace.json` | URL에 대한 HTTPS 액세스. 사용자는 카탈로그 자체에 `git`이 필요하지 않습니다                                           |
| 공유 파일 시스템의 디렉터리                                           | `/plugin marketplace add /Volumes/shared/claude-plugins`               | 경로에 대한 읽기 액세스                                                                               |

GitHub 또는 git URL 마켓플레이스의 분기 또는 태그를 고정하려면 사용자에게 `#<ref>`를 추가하도록 지시하세요(예: `your-org/your-marketplace#stable`). [플러그인 명령 참조](/docs/ko/plugins/cli-reference#plugin-marketplace-add)는 명령이 허용하는 모든 형식을 나열합니다.

성공적인 추가는 `Successfully added marketplace: your-marketplace`를 출력합니다. Claude Code는 저장소 이름이 아닌 `marketplace.json`의 `name` 필드에서 해당 이름을 가져옵니다.

그러면 사용자는 마켓플레이스의 `name`과 항목의 `name`으로 플러그인을 설치합니다(예: `/plugin install code-formatter@your-marketplace`).

<h3 id="register-the-marketplace-for-everyone-in-a-repository">
  저장소의 모든 사람을 위해 마켓플레이스 등록
</h3>

마켓플레이스를 한 저장소에서 작업하는 모든 사람과 공유하려면 셸에서 `claude plugin marketplace add your-org/your-marketplace --scope project`를 한 번 실행하고 작성하는 `.claude/settings.json`을 커밋하세요. Claude Code는 [폴더를 신뢰](/docs/ko/plugins/org#require-plugins-per-repository)하는 각 팀원을 위해 마켓플레이스를 등록합니다.

<h3 id="avoid-relative-path-entries-in-a-url-hosted-marketplace">
  URL 호스팅 마켓플레이스에서 상대 경로 항목 피하기
</h3>

사용자가 마켓플레이스를 베어 `marketplace.json` URL로 추가할 때 Claude Code는 해당 파일만 다운로드합니다. `plugins` 배열의 항목 중 `source`가 `./plugins/formatter`와 같은 상대 경로인 경우 설치 시 [`its marketplace entry path does not stay inside the marketplace directory`](/docs/ko/plugins/troubleshooting#plugins-with-relative-paths-fail-in-url-based-marketplaces)로 실패합니다. 모든 항목에 `github` 저장소 또는 `archive` URL과 같이 자체적으로 가져올 수 있는 소스를 제공하거나, Claude Code가 전체 트리를 복제할 수 있도록 마켓플레이스를 git 저장소에서 호스팅하세요.

<h3 id="edit-plugins-in-place-on-a-shared-directory">
  공유 디렉터리에서 플러그인을 제자리에서 편집
</h3>

사용자가 공유 디렉터리에서 마켓플레이스를 추가할 때 Claude Code는 상대 경로 소스를 가진 플러그인을 복사하는 대신 해당 디렉터리에서 직접 읽습니다. 사용자는 다음 세션을 시작하거나 `/reload-plugins`를 실행할 때 업데이트 단계나 버전 범프 없이 편집 내용을 봅니다.

<h3 id="keep-plugin-files-out-of-git-lfs">
  플러그인 파일을 Git LFS에서 제외
</h3>

플러그인이 필요한 파일을 [Git LFS](https://git-lfs.com)에서 제외하세요. 사용자가 git 저장소에서 호스팅되는 마켓플레이스를 추가하거나 나열된 git 기반 플러그인을 설치할 때 Claude Code는 해당 마켓플레이스 또는 플러그인 저장소를 머신에 복제합니다. 복제는 LFS 콘텐츠를 다운로드하지 않으므로 LFS 추적 파일은 포인터 파일로 도착합니다.

<h3 id="share-files-within-a-marketplace-with-symlinks">
  심볼릭 링크로 마켓플레이스 내 파일 공유
</h3>

플러그인과 동일한 마켓플레이스의 다른 부분 간에 파일을 공유하려면 플러그인 디렉터리 내에 심볼릭 링크를 만드세요. Claude Code가 플러그인을 캐시에 복사할 때 대상이 해결되는 위치에 따라 각 심볼릭 링크를 처리합니다:

* **플러그인 자체 디렉터리 내**: 심볼릭 링크는 캐시에서 상대 심볼릭 링크로 유지되므로 런타임에 복사된 대상으로 계속 해결됩니다.
* **동일한 마켓플레이스 내 다른 곳**: 심볼릭 링크가 역참조됩니다. 대상의 콘텐츠가 캐시에 복사됩니다. 이를 통해 메타 플러그인의 `skills/` 디렉터리가 마켓플레이스의 다른 플러그인으로 정의된 스킬에 연결될 수 있습니다.
* **마켓플레이스 외부**: 심볼릭 링크는 보안상 건너뜁니다.

로컬 경로에서 설치된 플러그인 또는 `mode`가 기본값 `copy`인 [`command` 소스](/docs/ko/plugins/marketplace-reference#command-plugin-source)의 경우 Claude Code는 플러그인 자체 디렉터리 내에서 해결되는 심볼릭 링크만 유지하고 다른 모든 것을 건너뜁니다.

다음 명령은 마켓플레이스 플러그인 내부에서 형제 플러그인으로 정의된 공유 스킬에 대한 링크를 만듭니다. Windows에서는 상승된 명령 프롬프트에서 `mklink /D`를 사용하거나 개발자 모드를 활성화하세요:

```bash theme={null}
ln -s ../../shared-plugin/skills/foo ./skills/foo
```

<h2 id="distribute-through-organization-settings">
  조직 설정을 통해 배포
</h2>

Team 또는 Enterprise 플랜에서는 사용자가 직접 추가하는 위치에서 호스팅하는 대신 claude.ai의 [**조직 설정 > 플러그인 및 스킬**](https://claude.ai/admin-settings/skills?tab=inventory)을 통해 마켓플레이스를 배포할 수도 있습니다. 조직 동기화는 claude.ai의 조직의 GitHub 또는 GitLab 연결을 통해 저장소를 읽으므로 사용자의 git 자격 증명이 관련되지 않습니다.

조직 동기화는 `/plugin marketplace add`보다 저장소에 대해 더 엄격합니다:

* **마켓플레이스 저장소**: github.com 및 gitlab.com에서 비공개 또는 내부여야 합니다
* **플러그인 소스**: 각 플러그인 소스는 `github`, `url` 또는 `git-subdir` 유형이거나 `./`로 시작하는 [상대 경로](/docs/ko/plugins/marketplace-reference#relative-path-plugin-source)여야 합니다
* **최상위 `bin/` 디렉터리**: claude.ai는 이를 가진 플러그인을 거부하고 마켓플레이스의 나머지를 동기화합니다. 오류 메시지는 `Plugin contains a top-level bin/ directory`로 시작합니다. 실행 파일을 `scripts/`와 같은 다른 디렉터리에 유지하고 훅 또는 MCP 서버 구성에서 `${CLAUDE_PLUGIN_ROOT}/scripts/<name>`으로 참조하세요

관리자 워크플로우는 [조직을 위한 플러그인 관리](https://support.claude.com/en/articles/13837433)를 참조하세요.

<h2 id="grant-access-to-a-private-marketplace">
  비공개 마켓플레이스에 대한 액세스 부여
</h2>

사용자가 마켓플레이스를 추가, 설치 또는 업데이트할 때 Claude Code는 머신에서 `git`을 실행하며 대화형 프롬프트를 끄고 해당 머신이 이미 보유한 자격 증명에 의존합니다. Claude Code는 자체 git 토큰이 없으며 `marketplace.json`에는 토큰 필드가 없습니다.

보내는 추가 명령의 형식으로 복제가 SSH 또는 HTTPS를 통해 실행되는지 선택합니다:

* **GitHub `owner/repo`**: Claude Code는 `ssh -T git@github.com`을 프로브하고 프로브가 성공하면 SSH를 통해 복제합니다. 프로브가 실패하거나 SSH 복제 자체가 실패하면 HTTPS를 통해 복제합니다. GitHub SSH 키가 없는 머신의 사용자는 `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`을 설정하여 프로브를 건너뛰고 HTTPS를 통해 복제할 수 있습니다.
* **`git@host:path.git`**: SSH.
* **`https://example.com/repo.git`**: HTTPS.

각 프로토콜이 머신에서 필요한 것을 사용자에게 알려주세요:

* **SSH**: 키는 암호 프롬프트 없이 작동해야 합니다(예: `ssh-agent`에 로드됨). 호스트는 이미 `known_hosts`에 있어야 합니다.
* **HTTPS**: Claude Code는 사용자의 git 자격 증명 도우미를 활성화 상태로 두지만 프롬프트를 금지합니다. 도우미가 이미 저장한 자격 증명은 작동합니다. 요청해야 하는 자격 증명은 실패합니다. GitHub에서 `gh auth login` 다음 `gh auth setup-git`을 실행하면 자격 증명이 저장됩니다.

GitHub Enterprise Server 호스트의 경우 사용자는 머신에서 해당 호스트에 대한 git 액세스가 필요합니다. [GHES의 플러그인 마켓플레이스](/docs/ko/github-enterprise-server#plugin-marketplaces-on-ghes)에서 각 Claude Code 표면이 GHES 호스팅 마켓플레이스에 도달하는 데 필요한 것을 참조하세요.

대신 claude.ai의 **조직 설정 > 플러그인 및 스킬**을 통해 배포하는 경우 사용자의 git 자격 증명이 관련되지 않습니다. 비공개일 수 있는 플러그인 소스는 [조직 설정을 통해 배포](#distribute-through-organization-settings)를 참조하세요.

<h3 id="serve-users-who-have-no-git-host-account">
  git 호스트 계정이 없는 사용자 제공
</h3>

git 호스트 계정이 없는 사용자는 `marketplace.json` URL로 또는 공유 디렉터리에서 마켓플레이스를 추가할 수 있지만 항목 소스에도 도달할 수 있는 플러그인만 설치할 수 있습니다. 비공개 `github` 저장소를 가리키는 항목은 Claude Code가 git 호스팅 마켓플레이스에 사용하는 것과 동일한 비대화형 `git`으로 가져오기 때문에 설치 시 여전히 실패합니다.

이러한 항목 소스는 git 계정이 필요하지 않습니다:

* **`archive`**: HTTPS를 통해 다운로드된 zip. 사용자는 `git`이나 계정이 필요하지 않으며 URL에 대한 네트워크 액세스만 필요합니다. Claude Code v2.1.224 이상이 필요합니다. 각 아카이브를 `sha256`으로 고정하여 Claude Code가 변경된 다운로드를 거부하도록 합니다. 다운로드와 함께 자격 증명을 보내려면 [아카이브 다운로드 인증](#authenticate-archive-downloads)을 참조하세요.
* **공개 git 저장소**: Claude Code는 항목이 `https://` URL을 제공할 때 자격 증명 없이 HTTPS를 통해 공개 `url` 또는 `git-subdir` 소스를 복제합니다. `github` 소스 또는 `owner/repo`로 작성된 `git-subdir` 소스의 경우 GitHub SSH 키가 없는 사용자는 `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`을 설정합니다.

한 네트워크의 팀의 경우 공유 파일 시스템의 `directory` 마켓플레이스도 git 계정 없이 작동합니다. 사용자는 경로에 대한 읽기 액세스만 필요합니다.

<h3 id="what-background-auto-update-does-with-credentials">
  백그라운드 자동 업데이트가 자격 증명으로 수행하는 작업
</h3>

백그라운드 자동 업데이트는 세션 시작 후 Claude Code의 마켓플레이스 및 설치된 플러그인의 무인 새로 고침입니다. [사용자를 최신 상태로 유지](#keep-users-up-to-date)에서 다루는 대로 사용자 또는 관리자가 켤 때까지 마켓플레이스에 대해 꺼져 있습니다.

비공개 마켓플레이스에 대해 켜져 있을 때 새 커밋에 대한 백그라운드 확인은 사용자의 구성된 git 자격 증명 도우미를 사용하며 절대 프롬프트하지 않습니다. 각 종류의 원격 및 도우미는 다른 결과를 제공합니다:

* **SSH 원격**: `ssh-agent`에 로드된 키가 확인을 인증합니다.
* **저장된 자격 증명이 있는 HTTPS 원격**: 프롬프트 없이 저장된 자격 증명을 제공할 수 있는 도우미가 확인을 인증합니다. Git Credential Manager, macOS Keychain 도우미 및 `git-credential-store`는 호스트에 대한 자격 증명을 보유한 후 이런 방식으로 작동합니다.
* **프롬프트가 필요한 HTTPS 원격 도우미**: 도우미는 백그라운드에서 응답할 수 없습니다. 업데이트가 조용히 실패하고 기존 체크아웃이 제자리에 남아 있으므로 사용자의 플러그인은 마지막 동기화된 상태에서 계속 작동합니다.

확인 후 Claude Code는 다음 중 하나를 수행합니다:

* **체크아웃이 최신 상태입니다**: Claude Code는 그대로 둡니다.
* **확인이 새 커밋을 찾거나 원격에 도달하거나 인증할 수 없어 실패합니다**: Claude Code는 마켓플레이스를 다시 복제하고 기존 체크아웃을 새 복제로 바꿉니다. 해당 복제가 실패하면 기존 체크아웃이 제자리에 남아 있습니다. 다시 복제는 [큰 저장소에서 시간 초과](/docs/ko/plugins/troubleshooting#git-clone-timed-out-after-120s)될 수 있습니다.

비공개 마켓플레이스를 최신 상태로 유지하려면 사용자는 다음 중 하나를 수행할 수 있습니다:

* **자격 증명 저장**: 먼저 자격 증명 도우미에 로그인하여 호스트에 대한 자격 증명을 보유하도록 합니다. GitHub의 경우 `gh auth login`을 실행한 다음 `gh auth setup-git`을 실행합니다.
* **실패 시 체크아웃 유지**: 사용자가 `CLAUDE_CODE_PLUGIN_KEEP_MARKETPLACE_ON_FAILURE=1`을 설정하면 Claude Code는 백그라운드 확인이 원격에 도달하거나 인증할 수 없을 때 다시 복제를 시도하지 않고 기존 체크아웃을 유지합니다. 플러그인은 마지막 동기화된 상태에서 계속 작동합니다.

사용자가 환경에서 `GITHUB_TOKEN` 또는 다른 공급자 토큰을 설정하면 그것만으로는 백그라운드 확인을 인증하지 않습니다. 토큰은 `GH_TOKEN` 및 `GITHUB_TOKEN`을 읽는 `gh` CLI의 도우미와 같은 자격 증명 도우미를 통해 효과를 발휘합니다.

<h2 id="roll-out-to-a-whole-company">
  전체 회사에 롤아웃
</h2>

플러그인을 회사에 롤아웃하려면 마켓플레이스 소유자, 관리 설정을 제어하는 관리자 및 Claude Code를 사용하는 각 사람이 필요합니다. 관리자 없이 롤아웃을 실행할 수 있으며, 이 경우 각 사람이 마켓플레이스를 추가하고 플러그인을 직접 설치합니다.

| 담당자            | 수행할 작업                                                                                                        | 다루는 위치                                                                                                                      |
| :------------- | :------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------- |
| 마켓플레이스 소유자인 당신 | 회사만 읽을 수 있는 저장소에 카탈로그를 유지하고, 호스트에 대한 추가 명령을 보내며, 각 사람이 머신에서 필요한 것을 말합니다                                       | [마켓플레이스 호스팅](#host-your-marketplace) 및 [비공개 마켓플레이스에 대한 액세스 부여](#grant-access-to-a-private-marketplace)                      |
| 관리자            | 관리 설정에서 `extraKnownMarketplaces` 및 `enabledPlugins`로 마켓플레이스를 등록하고 모든 사람을 위해 플러그인을 켜며, 거기서 `autoUpdate`를 설정합니다 | [마켓플레이스 및 플러그인 요구](/docs/ko/plugins/org#require-a-marketplace-and-its-plugins) 및 [업데이트 정책 설정](/docs/ko/plugins/org#set-update-policy) |
| 각 사람           | 머신에 이미 저장된 자격 증명이 있는 비공개 git 저장소에 대한 읽기 액세스가 필요합니다. 관리자가 없으면 추가 및 설치 명령도 실행합니다                                | [비공개 마켓플레이스 추가](/docs/ko/plugins/install#add-a-private-marketplace)                                                              |

git 호스트 계정이 없는 사람들의 경우 이러한 섹션 각각은 그들에게 도달하는 한 가지 방법을 다룹니다:

* **git 계정이 필요하지 않은 항목 소스**: [git 호스트 계정이 없는 사용자 제공](#serve-users-who-have-no-git-host-account)
* **사전 채워진 플러그인 디렉터리**: [컨테이너 및 CI 시드](/docs/ko/plugins/org#seed-containers-and-ci), 또한 git 호스트 계정이 없는 사용자를 제공합니다
* **claude.ai 조직 설정**: [조직 설정을 통해 배포](#distribute-through-organization-settings), 사용자의 git 자격 증명이 관련되지 않습니다

<h2 id="keep-users-up-to-date">
  사용자를 최신 상태로 유지
</h2>

개발자의 변경 사항은 마켓플레이스에서 백그라운드 자동 업데이트가 켜져 있거나 사용자가 플러그인을 직접 업데이트할 때 사용자에게 전달됩니다. 두 경우 모두 사용자는 플러그인의 계산된 버전이 변경될 때만 플러그인의 새 복사본을 받습니다. 자세한 내용은 [새 버전 출시](#release-a-new-version)를 참조하십시오.

<h3 id="turn-on-auto-update">
  자동 업데이트 켜기
</h3>

백그라운드 자동 업데이트는 기본적으로 마켓플레이스에서 꺼져 있으며, `marketplace.json`에는 이를 켜는 필드가 없습니다. 사용자 또는 관리자가 켜야 합니다:

* **사용자에게 켜도록 지시**: 각 사용자는 `/plugin`에서 **마켓플레이스**로 이동하여 마켓플레이스를 선택한 후 **자동 업데이트 활성화**를 선택합니다.
* **관리자에게 설정하도록 요청**: 관리자가 관리 설정의 마켓플레이스 `extraKnownMarketplaces` 항목에서 `"autoUpdate": true`를 설정하면 해당 설정을 받는 모든 사용자에게 켜집니다. [업데이트 정책 설정](/docs/ko/plugins/org#set-update-policy)을 참조하십시오.

자동 업데이트 없이 사용자는 세션에서 `/plugin marketplace update <name>`을 실행하거나 셸에서 `claude plugin update <plugin>@<name>`을 실행할 때 변경 사항을 받습니다.

업데이트가 사용자에게 도달할 때 사용자가 보는 내용은 [자동 업데이트가 실행될 때](/docs/ko/plugins/loading#when-auto-update-runs)를 참조하십시오.

<h3 id="release-a-new-version">
  새 버전 출시
</h3>

사용자에게 새 버전을 출시하려면 플러그인의 `version`을 변경합니다. 사용자는 플러그인의 계산된 버전이 보유한 버전과 다를 때만 새 복사본을 받습니다. 해당 버전은 [버전 및 업데이트](/docs/ko/plugins/loading#versions-and-updates)에 따라 먼저 `plugin.json`에서 나온 다음 마켓플레이스 항목에서 나옵니다.

사용자가 마켓플레이스에 추가한 로컬 디렉터리에서 [제자리에 로드](/docs/ko/plugins/loading#find-plugins-on-disk)하는 플러그인은 `version`으로 제어되지 않습니다. 버전 문자열이 무엇이든 상관없이 모든 세션 시작 시 현재 파일을 로드합니다.

제자리 로드 또는 `command` 소스의 로드를 제외한 모든 설치의 경우 각 릴리스에서 `version`을 증가시키거나 생략합니다:

* **각 릴리스에서 `version` 증가**: 사용자는 문자열이 변경될 때까지 캐시된 복사본에 유지됩니다. `"version": "1.0.0"`을 설정하고 변경하지 않고 새 커밋을 푸시하면 사용자는 이를 받지 못합니다.
* **`version` 생략**: 사용자는 대신 커밋을 추적합니다. `plugin.json`과 마켓플레이스 항목 모두에서 `version`을 제외합니다.

`plugin.json`과 마켓플레이스 항목 모두에 `version`을 설정하지 마십시오. 설정하면 Claude Code는 경고 없이 `plugin.json` 값을 사용하고, `claude plugin validate`는 불일치를 `Entry declares version "<a>" but <path>/plugin.json says "<b>"`로 보고합니다.

<h3 id="hold-users-on-one-version">
  사용자를 한 버전에 유지
</h3>

하나의 마켓플레이스는 한 번에 각 플러그인의 한 버전만 제공하므로 각 항목이 가리키는 것을 선택하여 사용자를 버전에 유지합니다:

* **플러그인 항목의 `ref` 및 `sha`**: `ref`는 분기 또는 태그의 이름이고 `sha`는 `github`, `url` 또는 `git-subdir` 소스의 커밋 이름입니다. [플러그인 소스](/docs/ko/plugins/marketplace-reference#plugin-sources)를 참조하십시오.
* **추가 명령의 `#<ref>`**: `your-org/your-marketplace#stable`을 추가하는 사용자는 카탈로그의 해당 분기 또는 태그를 받습니다. 동시에 두 개의 릴리스 라인의 경우 [릴리스 채널 실행](#run-release-channels)을 참조하십시오.
* **`<plugin>--v<version>` 태그**: 종속성의 버전 범위는 이러한 태그에 대해 확인됩니다. [다른 사용자가 의존하는 플러그인 출시](/docs/ko/plugins/dependencies#tag-plugin-releases-for-version-resolution)를 참조하십시오.

[새 버전 출시](#release-a-new-version)는 변경된 항목이 사용자에게 도달할 때를 설명합니다.

<h3 id="change-the-command-of-a-command-source">
  명령 소스의 명령 변경
</h3>

[`command` 소스](/docs/ko/plugins/marketplace-reference#command-plugin-source)의 `command`를 변경하거나 `mode`를 전환하면 각 사용자는 Claude Code가 실행하기 전에 새 명령을 수락해야 합니다. Claude Code는 사용자가 플러그인을 설치하거나 마지막으로 업데이트할 때 수락한 정확한 명령만 실행합니다.

사용자의 마켓플레이스 복사본이 변경 사항을 선택한 후 해당 사용자는 다음을 봅니다:

* **더 이상 백그라운드 실행 없음**: 해당 사용자에 대해 명령의 [세션당 한 번 실행](/docs/ko/plugins/loading#when-a-command-source-re-runs)이 중지되므로 도구의 새 출력이 사용자에게 도달하지 않습니다.
* **`/plugin` 오류 탭의 항목**: 항목은 새 명령과 실행할 `claude plugin update` 명령을 표시합니다.

사용자에게 해당 항목이 표시하는 `claude plugin update` 명령을 터미널에서 실행하도록 지시합니다. Claude Code는 사용자에게 새 명령을 표시하고 수락하도록 요청합니다.

<h2 id="run-release-channels">
  릴리스 채널 실행
</h2>

안정적이고 조기 액세스 트랙을 제공하려면 항목이 동일한 플러그인의 다른 ref를 가리키는 두 마켓플레이스를 호스팅하고 각 사용자가 원하는 것을 추가하도록 합니다. Claude Code에는 릴리스 채널 개념이 없으며 한 마켓플레이스는 한 번에 각 플러그인의 한 버전을 제공합니다.

두 `marketplace.json` 파일에 다른 `name` 값을 제공하세요. Claude Code는 마켓플레이스를 `name`으로 식별하므로 사용자는 동일한 이름의 두 마켓플레이스를 한 번에 등록할 수 없습니다.

이러한 두 카탈로그를 사용하면 `stable-tools`를 추가하는 사용자는 `stable` 분기에서 `code-formatter`를 설치하고 `latest-tools`를 추가하는 사용자는 `latest`에서 설치합니다:

```json theme={null}
{
  "name": "stable-tools",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": { "source": "github", "repo": "your-org/code-formatter", "ref": "stable" } }
  ]
}
```

```json theme={null}
{
  "name": "latest-tools",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": { "source": "github", "repo": "your-org/code-formatter", "ref": "latest" } }
  ]
}
```

두 ref에 다른 `plugin.json` 버전을 제공하거나 커밋 SHA가 구별하도록 `version`을 생략하세요. 업데이트는 버전을 비교하여 감지되므로 버전 변경 없이 이동하는 ref는 사용자를 캐시된 복사본에 남깁니다.

채널을 사용자가 선택하도록 하는 대신 사용자 그룹에 할당하려면 관리자가 각 그룹에 일치하는 `extraKnownMarketplaces` 항목을 제공합니다([업데이트 정책 설정](/docs/ko/plugins/org#set-update-policy)에 설명됨).

<h2 id="rename-or-remove-a-plugin">
  플러그인 이름 변경 또는 제거
</h2>

플러그인의 `name`은 해당 플러그인의 식별자입니다. 사용자는 `enabledPlugins` 및 `pluginConfigs` 설정 키와 `/plugin install`에서 이를 참조하므로, 이를 변경하면 기존의 모든 설치가 중단됩니다.

사용자가 `/plugin`에서 보는 레이블을 변경하되 아무것도 중단하지 않으려면, `plugin.json`에서 `displayName`을 설정하고 `name`은 변경하지 않은 상태로 유지하십시오.

<h3 id="migrate-users-with-a-renames-map">
  이름 변경 맵을 사용하여 사용자 마이그레이션
</h3>

`name`을 반드시 변경해야 하는 경우, `marketplace.json`에 최상위 수준의 `renames` 맵을 추가하여 Claude Code가 기존 사용자를 마이그레이션하도록 하고 [`Plugin "<name>" not found in marketplace`](/docs/ko/plugins/troubleshooting#plugin-not-found-in-marketplace)를 보고하지 않도록 합니다. `plugins`에서 항목을 제거할 때도 동일하게 수행하십시오. 자동 마이그레이션에는 Claude Code v2.1.193 이상이 필요합니다.

각 이전 이름을 현재 이름으로 매핑하거나, 플러그인이 없을 때는 `null`로 매핑합니다. 이 마켓플레이스는 `formatter`를 `code-formatter`로 이름을 변경하고 `legacy-linter`가 제거되었음을 기록합니다:

```json theme={null}
{
  "name": "your-marketplace",
  "owner": { "name": "Your Org" },
  "plugins": [
    { "name": "code-formatter", "source": "./plugins/code-formatter" }
  ],
  "renames": {
    "formatter": "code-formatter",
    "legacy-linter": null
  }
}
```

푸시한 후, 여전히 이전 이름이 활성화된 사용자는 다음 결과 중 하나를 봅니다:

* **이름이 변경된 항목**: 플러그인이 새 이름으로 로드됩니다. `claude plugin list`와 `/plugin` 아래의 플러그인 세부 정보에는 한 번 `Renamed to "code-formatter" in the "your-marketplace" marketplace`가 표시되며, Claude Code는 사용자, 프로젝트 및 로컬 설정 범위의 `enabledPlugins` 및 `pluginConfigs`에서 이전 키를 새 키로 다시 작성합니다.
* **`null` 항목**: 이전 키가 해당 범위에서 삭제되고 사용자는 `Removed from the "your-marketplace" marketplace`를 봅니다.
* **관리되는 설정에서 활성화됨**: 플러그인은 여전히 새 이름으로 로드되지만, Claude Code는 관리되는 설정을 다시 작성할 수 없으므로 관리자가 해당 위치의 `enabledPlugins`을 업데이트할 때까지 알림이 반복됩니다.

사용자가 git 저장소 또는 URL에서 추가한 마켓플레이스의 경우, 이름이 변경된 플러그인은 사용자가 세션에서 한 번 `/plugin install code-formatter@your-marketplace`를 실행할 때까지 [`Plugin "<name>" not cached at <path>`](/docs/ko/plugins/troubleshooting#plugin-not-cached-at)를 보고합니다.

`renames`를 추가 전용 기록으로 취급합니다. 모든 사용자가 마이그레이션한 후에도 이전 항목을 유지합니다. 다시 이름을 변경할 때는 Claude Code가 가장 오래된 이름에서 체인을 따르기 때문에 첫 번째 항목을 편집하는 대신 두 번째 항목을 추가합니다.

셸에서 맵을 편집한 후 `claude plugin validate .`를 실행합니다. 순환하거나 `null` 또는 `plugins`의 이름 이외의 다른 곳에서 끝나는 체인을 거부하며, `renames.<name>: chain does not resolve`를 표시합니다.

<h3 id="uninstall-removed-plugins-from-users’-machines">
  사용자 머신에서 제거된 플러그인 제거
</h3>

제거된 플러그인을 사용자 머신에서 제거하되 복사본을 남기지 않으려면, `marketplace.json`의 최상위 수준에서 `"forceRemoveDeletedPlugins": true`를 설정합니다. 이 필드가 없으면 제거된 플러그인이 설치된 상태로 유지되고 세션이 로드할 때 `Plugin "<name>" not found in marketplace`를 보고합니다. 이 필드가 있으면 Claude Code는 각 세션 시작 시 다음을 수행합니다:

1. 사용자가 마켓플레이스에서 설치한 항목을 항목 및 `renames` 맵과 비교하고, 나열되지 않거나 이름이 변경되지 않은 플러그인을 제거된 것으로 취급합니다.
2. 사용자, 프로젝트 및 로컬 범위에서 각 제거된 플러그인을 제거합니다. 관리되는 설정만 설치한 플러그인은 제자리에 유지됩니다.
3. `/plugin`의 **Flagged** 제목 아래에 각 제거된 플러그인을 `Removed from marketplace` 상태로 나열합니다.

<h2 id="authenticate-archive-downloads">
  아카이브 다운로드 인증
</h2>

[`archive`](/docs/ko/plugins/marketplace-reference#archive-plugin-source) 다운로드(예: 비공개 레지스트리에서의 다운로드)를 인증하려면 Claude Code가 함께 보내는 HTTP 헤더를 설정하세요. 다음 위치 중 하나에서 `headers`를 설정할 수 있습니다:

* **마켓플레이스의 `url` 소스**: [`extraKnownMarketplaces`](/docs/ko/settings-reference#extraknownmarketplaces) 항목과 같이 마켓플레이스를 등록한 `url` 소스입니다.
* **플러그인의 항목**: Claude Code v2.1.238 이상에서는 `source` 옆의 플러그인의 `marketplace.json` 항목에 설정할 수 있습니다.

어느 곳이든 값이 단기간인 경우(예: 레지스트리가 요청 시 생성하는 토큰) `headers` 대신 `headersHelper` 명령을 설정하세요. Claude Code는 명령을 실행하고 인쇄하는 JSON 객체를 해당 위치의 헤더로 보냅니다. Claude Code v2.1.238 이상이 필요합니다.

[마켓플레이스 참조](/docs/ko/plugins/marketplace-reference#plugin-entries)는 `headers` 및 `headersHelper` 항목 필드를 나열합니다.

선택한 위치는 어떤 다운로드가 헤더를 받는지와 Claude Code가 명령을 실행할 때를 결정합니다:

| 위치              | 헤더를 받는 다운로드                                     | Claude Code가 거기서 설정된 `headersHelper`를 실행할 때                                                          |
| :-------------- | :---------------------------------------------- | :--------------------------------------------------------------------------------------------------- |
| 마켓플레이스 `url` 소스 | 마켓플레이스 URL의 원점에서의 아카이브 다운로드, 즉 동일한 스킴, 호스트 및 포트 | 마켓플레이스의 `marketplace.json` 각 가져오기 전과 해당 원점의 각 아카이브 다운로드 전. Claude Code는 한 번 실행의 출력을 최대 60초 동안 재사용합니다 |
| 플러그인 항목         | 해당 항목의 다운로드만                                    | 사용자가 해당 플러그인 하나를 직접 설치 또는 업데이트하고 [명령을 수락](#how-users-accept-a-headershelper-command)할 때만             |

두 위치 모두 동일한 이름의 헤더를 설정할 때 Claude Code는 항목의 값을 보냅니다. 한 위치 내에서 명령이 인쇄하는 헤더는 동일한 이름의 `headers`에 나열된 헤더를 재정의합니다.

<h3 id="add-a-headershelper-to-a-plugin-entry">
  플러그인 항목에 headersHelper 추가
</h3>

이 항목은 `source` 옆에 `headersHelper`를 설정합니다. 또한 [`"strict": false`](/docs/ko/plugins/marketplace-reference#strict-mode)를 설정하며, Claude Code는 `headersHelper`를 설정하는 `marketplace.json` 항목이 필요합니다:

```json theme={null}
{
  "name": "my-plugin",
  "description": "Formatting commands for internal services",
  "strict": false,
  "source": {
    "source": "archive",
    "url": "https://registry.example.com/plugins/my-plugin-2.1.0.zip"
  },
  "headersHelper": "/opt/bin/mint-registry-token.sh"
}
```

항목을 확인하려면 셸에서 `claude plugin install my-plugin@your-marketplace`를 실행하세요. Claude Code는 명령과 아카이브 URL을 표시하고 수락 후 zip을 다운로드합니다.

<h3 id="write-the-headershelper-command">
  headersHelper 명령 작성
</h3>

마켓플레이스의 `url` 소스 또는 플러그인 항목에 `headersHelper`를 설정하든 명령을 작성하여 이러한 요구 사항을 충족하세요:

* **명령 텍스트**: 최대 500자의 인쇄 가능한 ASCII, 4개 이상의 공백 실행 없음.
* **출력**: stdout에 헤더 이름 및 문자열 값의 JSON 객체 하나를 인쇄한 후 10초 내에 종료 0.
* **셸 및 작업 디렉터리**: Claude Code는 `sh`를 통해 또는 Windows에서 `cmd.exe`를 통해 명령을 실행합니다. 작업 디렉터리는 구성 디렉터리이며 `~/.claude` 또는 [`CLAUDE_CONFIG_DIR`](/docs/ko/env-vars#variables)입니다. 상대 경로가 사용자의 프로젝트가 아닌 해당 디렉터리에 대해 해결되므로 절대 경로 또는 `PATH`의 명령을 제공하세요.
* **Claude Code가 제거하는 변수**: 명령이 `marketplace.json` 항목 또는 프로젝트의 `.claude/settings.json` 또는 `.claude/settings.local.json`에 설정되면 Claude Code는 환경에서 자격 증명처럼 보이는 이름의 모든 변수를 제거합니다([MCP `headersHelper`에 적용하는 동일한 규칙](/docs/ko/mcp#which-variables-a-helper-can-read)). `ANTHROPIC_API_KEY` 및 `MY_REGISTRY_TOKEN` 모두 제거되므로 명령이 파일 또는 자격 증명 저장소에서 자격 증명을 읽도록 합니다. 이 제거는 사용자 설정, `--settings` 파일 또는 관리 설정에 설정된 명령에는 적용되지 않습니다.
* **Claude Code가 설정하는 변수**: `url` 소스의 명령에 대해 `CLAUDE_CODE_MARKETPLACE_URL` 및 `CLAUDE_CODE_MARKETPLACE_NAME`, 항목의 명령에 대해 `CLAUDE_CODE_PLUGIN_NAME` 및 `CLAUDE_CODE_PLUGIN_ARCHIVE_URL`. `CLAUDE_CODE_MARKETPLACE_NAME`은 사용자가 URL로 마켓플레이스를 추가한 후 첫 번째 가져오기에서 설정되지 않습니다. 해당 가져오기가 이름을 제공하기 때문입니다.

베어러 토큰을 발행하는 명령은 다음과 같은 객체를 인쇄합니다:

```json theme={null}
{"Authorization": "Bearer eyJhbGciOiJSUzI1NiJ9"}
```

<h3 id="when-claude-code-skips-a-headershelper-command-or-drops-its-output">
  Claude Code가 headersHelper 명령을 건너뛰거나 출력을 삭제할 때
</h3>

`headersHelper` 명령이 실행되지 않거나 `headers` 또는 명령의 출력에서 헤더가 삭제되는 경우는 다음 중 하나가 적용될 때입니다:

* **명령 실패**: 명령이 0이 아닌 종료, 10초 이상 실행 또는 JSON 문자열 값 객체 이외의 것을 인쇄하면 명령이 실행된 가져오기 또는 다운로드가 발생하지 않습니다.
* **마켓플레이스 URL이 `https://`로 시작하지 않음**: 해당 `url` 소스의 명령이 실행되지 않으며 요청은 `headers` 필드에 나열된 헤더만 전달합니다.
* **리디렉션이 원점을 떠남**: 다운로드가 아카이브 URL의 원점에서 리디렉션될 때 리디렉션된 요청은 마켓플레이스 `url` 소스 또는 플러그인 항목의 `headers` 값 또는 명령 출력을 전달하지 않습니다.
* **항목이 라우팅 또는 ID 헤더를 설정함**: Claude Code는 항목의 `headers` 및 명령 출력에서 `Host`, `Cookie` 및 `X-Forwarded-*`와 같은 요청 라우팅 및 클라이언트 ID 이름을 삭제하고 `Authorization`과 같은 인증 이름을 유지합니다. 모든 `marketplace.json` 항목이 이런 방식으로 필터링됩니다. 설정의 인라인 플러그인 항목의 경우 [`extraKnownMarketplaces`](/docs/ko/settings-reference#extraknownmarketplaces)를 참조하세요.
* **`--add-dir` 디렉터리의 설정에 설정된 명령**: 명령이 무시되며 `url` 소스 및 [인라인 플러그인 항목](/docs/ko/settings-reference#extraknownmarketplaces) 모두에서 해당 파일의 `headers`만 전송됩니다.
* **관리 설정이 명령을 차단함**: [`disableCommandPluginSources`](/docs/ko/settings-reference#disablecommandpluginsources)를 `true`로 설정하면 `headersHelper` 명령이 차단되고 [`allowManagedHooksOnly`](/docs/ko/settings-reference#allowmanagedhooksonly)도 `disableCommandPluginSources`가 명시적으로 `false`가 아닌 한 차단합니다. 두 차단 중 하나에서 Claude Code는 여전히 관리 설정이 선언하는 마켓플레이스에 대한 명령을 실행합니다.

<h3 id="how-users-accept-a-headershelper-command">
  사용자가 headersHelper 명령을 수락하는 방법
</h3>

사용자는 해당 플러그인 항목을 설치 또는 업데이트할 때마다 명령을 수락합니다. 그들은 `/plugin`의 플러그인 자체 보기에서 또는 `claude plugin install` 또는 `claude plugin update`로 수행합니다. Claude Code는 명령과 아카이브 URL을 표시하고 사용자가 수락한 후에만 명령을 실행합니다.

비대화형 셸에서 [`--yes`](/docs/ko/plugins/cli-reference#plugin-install)를 전달하여 명령을 수락합니다. 이전 `--json` 실행이 표시한 명령만 수락하려면 실행이 보고한 `sha256`과 함께 [`--accept-command`](/docs/ko/plugins/cli-reference#plugin-install)를 전달하세요.

Claude Code는 표시한 명령만 실행하며 표시한 아카이브 URL의 경우입니다. 그 사이에 항목의 명령 또는 아카이브 URL이 변경되면 Claude Code는 설치 또는 업데이트를 거부합니다. 쿼리 문자열만의 변경은 계산되지 않습니다.

<h3 id="installs-and-updates-that-refuse-the-command-instead-of-asking">
  명령을 요청하는 대신 거부하는 설치 및 업데이트
</h3>

단일 플러그인 설치 또는 업데이트 이외의 다른 작업에서 Claude Code는 항목의 명령을 실행하거나 아카이브를 다운로드하지 않습니다. 플러그인은 설치된 버전에 남아 있거나 설치되지 않은 상태로 유지되며 사용자는 다음 중 하나의 결과를 봅니다:

* **여러 플러그인을 한 번에 설치, 플러그인 제안에서 또는 다른 플러그인의 종속성으로**: Claude Code는 명령을 가진 플러그인을 거부하고 사용자를 `/plugin`의 해당 플러그인 자체 보기로 지시합니다. 대량 설치의 다른 플러그인은 여전히 설치됩니다. 거부된 플러그인에 의존하는 플러그인은 사용자가 거부된 플러그인을 직접 설치할 때까지 설치에 실패합니다.
* **백그라운드 자동 업데이트 또는 아카이브가 다운로드되지 않은 플러그인의 세션 시작**: Claude Code는 플러그인을 `/plugin` 오류 탭에 나열하므로 사용자는 직접 설치 또는 업데이트해야 함을 알 수 있습니다.

<h3 id="when-a-marketplace-url-sources-command-runs">
  마켓플레이스 `url` 소스의 명령이 실행될 때
</h3>

마켓플레이스 `url` 소스의 `headersHelper`를 마켓플레이스가 게시하는 카탈로그가 아닌 설정 파일(예: [`extraKnownMarketplaces`](/docs/ko/settings-reference#extraknownmarketplaces) 항목)에 선언합니다. Claude Code는 따라서 각 설치 또는 업데이트에서 사용자에게 수락하도록 요청하지 않습니다. 대신 선언하는 설정 파일이 Claude Code가 실행할 때를 결정합니다:

| 설정 파일                                                          | Claude Code가 명령을 실행할 때                                                                                                                                |
| :------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------- |
| 사용자 설정, `--settings` 파일 또는 머신의 관리 설정 파일                        | 백그라운드 마켓플레이스 새로 고침을 포함하여 요청 없이                                                                                                                        |
| 프로젝트의 `.claude/settings.json` 또는 `.claude/settings.local.json` | 해당 폴더 자체에 대한 [작업 공간 신뢰 대화](/docs/ko/permissions#what-runs-before-you-trust-a-folder)를 사용자가 수락한 후에만. `-p` 또는 SDK 세션은 수락으로 계산되지 않으며 부모 폴더에 부여된 신뢰도 계산되지 않습니다 |
| 서버 관리 설정                                                       | 대화형 세션에서 사용자가 [보안 승인 대화](/docs/ko/server-managed-settings#security-approval-dialogs)에서 전달된 설정을 승인한 후에만                                                     |

이러한 파일 중 하나의 [인라인 플러그인 항목](/docs/ko/settings-reference#extraknownmarketplaces)의 경우 Claude Code는 해당 파일의 마켓플레이스 수준 명령과 동일한 폴더 신뢰 또는 설정 승인을 요구하며 사용자는 각 설치 또는 업데이트에서 항목의 명령을 수락합니다.

<h2 id="depend-on-and-recommend-other-plugins">
  다른 플러그인에 의존 및 권장
</h2>

항목은 다른 플러그인에 대한 종속성을 선언할 수 있습니다.

* **버전 범위**: 종속성은 semver 범위를 전달할 수 있습니다.
* **교차 마켓플레이스 종속성**: 다른 마켓플레이스의 종속성은 마켓플레이스가 `allowCrossMarketplaceDependenciesOn`에 해당 마켓플레이스를 나열할 때만 설치됩니다.

버전 범위, 해결되는 `<plugin>--v<version>` git 태그 규칙 및 교차 마켓플레이스 신뢰의 경우 [플러그인 종속성](/docs/ko/plugins/dependencies)을 참조하세요.

프로젝트가 일치할 때 Claude Code가 플러그인을 제안하도록 하려면 항목에 프로젝트를 식별하는 신호가 있는 `relevance` 블록을 추가하세요. 사용자는 관리자가 `pluginSuggestionMarketplaces`에 나열할 때만 마켓플레이스의 제안을 봅니다. 신호 및 활성화 단계는 [플러그인 관련성](/docs/ko/plugins/relevance)을 참조하세요.

<h2 id="work-around-what-a-marketplace-can’t-do">
  마켓플레이스가 할 수 없는 것 해결
</h2>

일부 소유자가 요청하는 것은 `marketplace.json`에 필드가 없습니다. 각각에 대한 가장 가까운 옵션은 다음과 같습니다:

* **사용자가 설치하는 다른 것 제한**: 마켓플레이스 허용 목록은 관리 설정 `strictKnownMarketplaces`입니다. [사용자가 설치할 수 있는 것 제한](/docs/ko/plugins/org#restrict-what-users-can-install)을 참조하세요.
* **사용자가 요청하지 않고 플러그인 설치 또는 활성화**: 항목 필드가 플러그인을 설치하지 않습니다. 관리 `enabledPlugins`는 플릿에 대해 그렇게 합니다. [플러그인 사전 설치 및 요구](/docs/ko/plugins/org#pre-install-and-require-plugins)를 참조하세요.
* **다른 사용자에게 다른 항목 표시**: 항목은 대상 필드를 전달하지 않으며 마켓플레이스를 추가하는 모든 사용자는 전체 카탈로그를 봅니다. 다른 대상을 위해 별도의 마켓플레이스를 호스팅하세요.
* **플러그인을 더 이상 사용되지 않음으로 표시**: 더 이상 사용되지 않음 상태가 없습니다. 옵션은 항목을 제거하고 `renames`에서 이름을 `null`로 매핑하며 선택적으로 `forceRemoveDeletedPlugins`를 설정하는 것입니다.
* **사용자를 위해 자동 업데이트 켜기**: 각 사용자는 `/plugin`의 **마켓플레이스** 아래에서 켜거나 관리자가 관리 설정에서 `autoUpdate`를 설정합니다. [자동 업데이트 켜기](#turn-on-auto-update)를 참조하세요.
* **git 자격 증명 전달**: 마켓플레이스 필드가 git 토큰을 보유하지 않습니다. git 호스팅 마켓플레이스 또는 플러그인에 대한 액세스는 [비공개 마켓플레이스에 대한 액세스 부여](#grant-access-to-a-private-marketplace)에 따라 사용자의 git 설정을 따릅니다. `archive` 소스의 경우 항목은 대신 [`headers` 또는 `headersHelper`](#authenticate-archive-downloads)를 설정할 수 있습니다.

<h2 id="next-steps">
  다음 단계
</h2>

* [마켓플레이스 참조](/docs/ko/plugins/marketplace-reference): `marketplace.json` 필드, 소스 유형 및 유효성 검사 메시지
* [조직을 위한 플러그인 관리](/docs/ko/plugins/org): 조직의 머신 전체에서 마켓플레이스를 요구, 제한 또는 시드
* [플러그인 종속성](/docs/ko/plugins/dependencies): 플러그인에 의존하는 플러그인이 버전을 해결할 수 있도록 릴리스 태그
* [플러그인 문제 해결](/docs/ko/plugins/troubleshooting): 마켓플레이스에서 추가 또는 업데이트할 때 사용자가 보는 오류
