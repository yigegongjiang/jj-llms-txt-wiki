> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 조직을 위한 Claude Code 플러그인 관리

> 관리되는 설정을 통해 조직의 모든 머신에 Claude Code가 설치하고 허용하는 플러그인을 제어합니다.

관리되는 설정을 통해 조직의 모든 머신에 Claude Code가 설치하고 허용하는 플러그인을 결정할 수 있습니다. 사용자는 이를 재정의할 수 없습니다. [서버 관리 설정](/docs/ko/server-managed-settings)을 claude.ai 관리자 콘솔에서 제공하거나 MDM 또는 `managed-settings.json` 파일을 통해 엔드포인트 관리 설정으로 제공할 수 있습니다. 이 페이지의 대부분의 제어는 관리되는 설정에서만 적용됩니다.

이 페이지는 관리자용이며, 여기의 설정은 Claude Code를 관리합니다.

<Note>
  다음 경우는 다른 페이지에서 다룹니다:

  * **자신을 위한 플러그인 설치**: [플러그인 설치](/docs/ko/plugins/install)에서 시작하세요
  * **claude.ai 및 Cowork에서 멤버가 사용할 수 있는 플러그인 제어**: 도움말 센터의 [조직을 위한 플러그인 관리](https://support.claude.com/en/articles/13837433)를 참조하세요
  * **claude.ai의 관리자 설정에 있는 플러그인 페이지**: [**조직 설정 > 플러그인 및 스킬**](https://claude.ai/admin-settings/skills?tab=inventory)은 멤버의 claude.ai 계정에 대해 플러그인을 켜고, 이는 Claude Code에 [동기화된 플러그인](/docs/ko/plugins/loading#synced-plugins)으로 도달합니다. 이 페이지의 키를 설정하지 않습니다
</Note>

섹션은 대부분의 롤아웃이 따르는 순서를 따릅니다: 모든 사람 또는 저장소별로 [플러그인 필요](#pre-install-and-require-plugins), [컨테이너 및 CI 시드](#seed-containers-and-ci), 사용자가 추가할 수 있는 것 [제한](#restrict-what-users-can-install), [업데이트 정책 설정](#set-update-policy), 그 다음 [설치된 것 감사](#audit-and-review). 모든 정책 키를 한 곳에서 검토하려면 [제어 매트릭스](#control-matrix)를 참조하세요.

<h2 id="pre-install-and-require-plugins">
  플러그인 사전 설치 및 필수 설정
</h2>

마켓플레이스는 Claude Code가 git 저장소, URL 또는 로컬 경로에서 가져오는 플러그인 카탈로그입니다. 머신에 마켓플레이스를 등록하면 Claude Code는 해당 마켓플레이스에서 플러그인을 설치할 수 있습니다.

플릿에 플러그인을 설치하려면 조직의 모든 머신이 읽는 정책 파일 또는 서버 전달 정책인 [관리 설정](/docs/ko/managed-settings)에서 두 개의 키를 함께 설정합니다. `extraKnownMarketplaces`는 각 머신에 마켓플레이스를 등록하고, `enabledPlugins`는 설치 및 활성화할 플러그인의 이름을 지정합니다. [전달 메커니즘 선택](#choose-a-delivery-mechanism)에서는 관리 설정이 각 머신에 도달하는 방법을 설명합니다.

<h3 id="choose-a-delivery-mechanism">
  전달 메커니즘 선택
</h3>

관리 설정은 다음 세 가지 전달 메커니즘 중 하나를 통해 머신에 도달합니다.

* **서버 관리 설정**: [**조직 설정 > Claude Code > 관리 설정**](https://claude.ai/admin-settings/claude-code)에서 플러그인 키를 JSON으로 설정합니다. Claude 조직에서 [소유자 역할](/docs/ko/server-managed-settings#access-control)이 필요합니다. 클라우드 세션은 플러그인을 설치하기 전에 이러한 설정을 가져옵니다.
* **MDM 정책**: macOS에서는 최상위 키가 설정 키인 plist를 전달합니다. Windows에서는 전체 JSON 문서를 레지스트리 값의 문자열로 저장합니다. plist 도메인과 레지스트리 키는 [각 메커니즘이 정책을 저장하는 위치](/docs/ko/managed-settings#where-each-mechanism-stores-the-policy)에 있습니다.
* **관리 설정 파일**: 플랫폼의 시스템 경로에 `managed-settings.json`을 배치합니다. 옆의 `managed-settings.d/` 드롭인 디렉터리에 파일을 추가할 수도 있습니다. 플랫폼별 파일 경로는 [각 메커니즘이 정책을 저장하는 위치](/docs/ko/managed-settings#where-each-mechanism-stores-the-policy)에 있으며, 드롭인 병합 규칙은 [파일 기반 정책을 팀 간에 분할](/docs/ko/managed-settings#split-a-file-based-policy-across-teams)에 있습니다.

Claude for Teams 또는 Enterprise 조직이 claude.ai에 있고 모든 디바이스가 MDM 관리 대상이 아닌 경우 서버 관리 설정을 사용합니다. 그렇지 않으면 MDM 정책 또는 관리 설정 파일을 사용합니다. 트레이드오프는 [서버 관리 설정과 엔드포인트 관리 설정 간 선택](/docs/ko/server-managed-settings#choose-between-server-managed-and-endpoint-managed-settings)을 참조합니다.

<h4 id="which-managed-source-applies-on-a-machine">
  머신에 적용되는 관리 소스
</h4>

기본적으로 이 세 가지 소스 중 하나만 머신에 적용됩니다. Claude Code는 정책 키를 전달하는 첫 번째 소스를 사용하며, 서버 관리 설정을 먼저 확인한 다음 MDM 정책, 마지막으로 관리 설정 파일을 확인합니다. 서버 관리 설정이 관련 없는 정책 키를 하나라도 전달하면 Claude Code는 해당 머신의 MDM 정책 또는 관리 설정 파일의 플러그인 키를 무시합니다. 단, [모든 소스에서 읽는 키](/docs/ko/managed-settings#keys-read-from-every-admin-source)는 제외됩니다.

모든 소스를 대신 적용하려면 [`managedSourcesBehavior`](/docs/ko/managed-settings#compose-every-managed-source)를 `"merge"`로 설정합니다.

[Claude Code가 관리 소스를 결합하는 방법](/docs/ko/managed-settings#how-claude-code-combines-managed-sources)에서는 두 모드 모두에서 Claude Code가 모든 소스에서 읽는 키도 나열합니다.

<h3 id="require-a-marketplace-and-its-plugins">
  마켓플레이스 및 해당 플러그인 필수 설정
</h3>

`extraKnownMarketplaces` 아래에 마켓플레이스를 추가합니다. 마켓플레이스의 `marketplace.json`에서 자신의 `name`을 키로 사용합니다. 그런 다음 각 플러그인을 `enabledPlugins` 아래에 `plugin-name@marketplace-name`으로 추가합니다. 각 마켓플레이스 항목은 `source` 필드가 있는 `source` 객체를 포함하며, 이 필드는 `github`와 같은 유형의 이름을 지정합니다. 이 관리 설정 예제는 조직 마켓플레이스를 등록하고 해당 마켓플레이스에서 두 개의 플러그인을 강제로 활성화합니다.

```json theme={null}
{
  "extraKnownMarketplaces": {
    "your-marketplace": {
      "source": { "source": "github", "repo": "your-org/your-marketplace" },
      "autoUpdate": true
    }
  },
  "enabledPlugins": {
    "code-formatter@your-marketplace": true,
    "deploy-helper@your-marketplace": true
  }
}
```

설정이 머신에 도달한 후 Claude Code는 사용자의 다음 세션 시작 시 마켓플레이스를 등록하고 두 플러그인을 설치합니다. 사용자는 `/plugin`에서 이들을 볼 수 있으며, 자신의 범위에서 하나를 비활성화해도 관리 설정이 다른 모든 범위보다 우선하기 때문에 로드되는 것을 중단하지 않습니다.

모든 범위에서 플러그인을 차단하고 마켓플레이스 목록에서 숨기려면 대신 관리 `enabledPlugins`에서 `false`로 설정합니다.

마켓플레이스에 맞게 `autoUpdate` 및 `source` 필드를 조정합니다.

* **`autoUpdate`**: `true`는 마켓플레이스와 해당 플러그인을 백그라운드에서 새로 고침 상태로 유지하고, `false`는 이를 끕니다. [업데이트 정책 설정](#set-update-policy)을 참조합니다.
* **`source`**: `github`는 여러 소스 유형 중 하나입니다. `git` 소스는 GitLab 또는 내부 호스트의 `url`을 사용하고, `url` 소스는 호스팅된 `marketplace.json`의 주소를 사용합니다. 모든 소스 형태는 [마켓플레이스 참조](/docs/ko/plugins/marketplace-reference)에 있습니다.

마켓플레이스가 비공개 git 저장소인 경우 각 사용자는 읽기 액세스 권한이 필요합니다. git 기반 마켓플레이스의 클론은 사용자의 머신에서 git을 사용하여 실행되며, 저장된 자격 증명을 사용하고 프롬프트가 없습니다. git 호스트 계정이 없는 사용자의 경우 [시드](#seed-containers-and-ci)를 대신 사용합니다.

관리 항목은 또한 같은 이름의 마켓플레이스 항목 또는 다른 소스의 `--plugin-dir` 복사본을 재정의합니다.

* **마켓플레이스**: 관리 마켓플레이스 항목은 같은 이름의 낮은 우선순위 항목을 대체하며, 두 항목의 필드는 병합되지 않습니다.
* **`--plugin-dir` 복사본**: `--plugin-dir`은 한 세션 동안 로컬 디렉터리에서 플러그인을 로드합니다. 해당 복사본의 이름이 관리 `enabledPlugins`가 지정하는 플러그인과 일치할 때 발생하는 상황은 [이름 충돌](/docs/ko/plugins/loading#name-conflicts)을 참조합니다.

Anthropic의 공식 마켓플레이스 `claude-plugins-official`은 `enabledPlugins`가 해당 플러그인 중 하나를 `true`로 설정할 때 `extraKnownMarketplaces` 항목이 필요하지 않습니다. 해당 `name@claude-plugins-official` 항목은 이러한 키가 적용되는 모든 곳에서 마켓플레이스를 선언합니다. 해당 플러그인을 활성화하지 않으면서도 모든 머신에 등록하려면 [공식 마켓플레이스 및 자신의 마켓플레이스 허용](#allow-the-official-marketplace-and-your-own)에서 하는 것처럼 명시적 항목을 제공합니다.

<h3 id="require-plugins-per-repository">
  저장소별 플러그인 필수 설정
</h3>

전체 플릿 대신 하나의 저장소의 기여자를 대상으로 하려면 해당 저장소의 `.claude/settings.json`에서 `extraKnownMarketplaces` 및 `enabledPlugins`를 설정합니다. `extraKnownMarketplaces` 항목은 기여자가 신뢰한 폴더에만 적용되며, 신뢰하지 않는 폴더에서는 Claude Code가 메시지 없이 이들을 무시합니다.

* **대화형 세션**: Claude Code는 기여자가 해당 폴더에 대한 [작업 영역 신뢰 대화](/docs/ko/permissions#what-runs-before-you-trust-a-folder)를 수락한 후에만 마켓플레이스를 등록합니다.
* **[비대화형 `-p` 실행](/docs/ko/headless)**: 항목은 사용자가 이미 대화형으로 신뢰를 수락한 폴더 또는 `~/.claude.json`에서 `hasTrustDialogAccepted` 플래그를 설정한 폴더에만 적용됩니다.

마켓플레이스가 상대 경로로 나열하는 플러그인은 저장소의 `extraKnownMarketplaces` 항목이 적용되면 마켓플레이스 복사본에서 로드됩니다. 마켓플레이스 항목이 플러그인의 자체 GitHub 저장소와 같은 외부 소스를 대신 가리키는 플러그인은 저장소의 설정만으로는 설치되지 않습니다. 각 기여자는 [플러그인 설치](/docs/ko/plugins/install)에서 설명하는 대로 `claude plugin install <name>@<marketplace> --scope project`를 실행할 때까지 `Plugin "<name>" is enabled in project settings but isn't installed`를 봅니다.

상대 경로가 있는 로컬 `directory` 또는 `file` 소스를 사용하는 경우 경로는 저장소의 주 체크아웃에 대해 확인됩니다. git worktree에서 Claude Code를 실행할 때 경로는 여전히 주 체크아웃을 가리키므로 모든 worktree는 같은 마켓플레이스 위치를 공유합니다.

종속성이 있는 플러그인 번들을 배포하려면 [플러그인 종속성](/docs/ko/plugins/dependencies)에서 설명하는 대로 번들 플러그인을 `enabledPlugins`에 넣습니다.

<h3 id="when-each-surface-applies-the-plugin-keys">
  각 표면이 플러그인 키를 적용하는 시기
</h3>

표는 관리 설정 및 저장소의 `.claude/settings.json`에서 각 종류의 Claude Code 세션이 `extraKnownMarketplaces` 및 `enabledPlugins`를 적용하는 시기를 보여줍니다. Desktop 앱 및 IDE 확장의 경우 [플러그인 설치](/docs/ko/plugins/install#install-a-plugin)를 참조합니다.

| 표면        | 관리 `extraKnownMarketplaces` 및 `enabledPlugins`                                                                                                                                                   | 저장소 `.claude/settings.json`                                               |
| :-------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------ |
| 터미널, 대화형  | 설정을 받는 모든 머신에서 세션 시작 시 적용됨                                                                                                                                                                       | `extraKnownMarketplaces`는 신뢰 후 적용됨; `enabledPlugins`는 세션 시작 시 적용됨         |
| `-p` 및 CI | 세션 시작 시 적용되며, 설치는 백그라운드에서 실행됨                                                                                                                                                                    | 신뢰하는 폴더에서만 `extraKnownMarketplaces`; `enabledPlugins` 적용됨                 |
| 클라우드 세션   | Anthropic 호스팅 환경에서는 서버 관리 설정만 세션에 도달하며, 플러그인을 설치하기 전에 대기합니다. MDM 정책 및 관리 설정 파일은 사용자의 머신에 남아 있습니다. 자체 호스팅 환경의 경우 [정책이 적용되는 위치 및 시기](/docs/ko/managed-settings#where-and-when-a-policy-applies)를 참조합니다. | [플러그인 설치](/docs/ko/plugins/install#install-a-plugin) 아래의 **클라우드 세션** 탭을 참조합니다. |

`-p` 또는 CI 실행에서 마켓플레이스와 플러그인은 백그라운드에서 설치되므로 플러그인이 첫 번째 턴에서 누락될 수 있습니다. `CLAUDE_CODE_SYNC_PLUGIN_INSTALL=1`을 설정하여 첫 번째 쿼리 전에 설치를 기다리도록 실행합니다.

<h3 id="confirm-the-rollout">
  배포 확인
</h3>

마켓플레이스와 플러그인이 머신 또는 CI 실행에 도착했는지 확인합니다.

* **한 머신에서**: Claude Code를 시작하고 `/plugin`을 실행합니다. 마켓플레이스와 플러그인이 나열됩니다.
* **CI에서**: `--output-format stream-json --verbose`와 함께 `claude -p`를 실행합니다. `init` 이벤트는 `plugins` 아래에 로드된 플러그인을 나열합니다.

<h2 id="seed-containers-and-ci">
  컨테이너 및 CI 시드
</h2>

런타임에 복제할 수 없는 컨테이너 이미지 및 CI 러너의 경우 빌드 시간에 플러그인 디렉토리를 미리 채우고 `CLAUDE_CODE_PLUGIN_SEED_DIR`을 가리키세요. Claude Code는 시작 시 시드의 마켓플레이스를 등록하고 복제 없이 시드에서 플러그인 캐시를 로드합니다.

시드는 또한 git 호스트 계정이 없는 사용자를 제공합니다.

<Note>
  CI/CD 환경에서는 비공개 저장소에서 플러그인을 설치하기 전에 git 자격 증명 도우미를 구성하세요. GitHub Actions에서는 마켓플레이스 저장소에 대한 읽기 액세스 권한이 있는 토큰을 `GH_TOKEN`으로 내보낸 다음 `gh auth setup-git`을 실행하세요. 기본 워크플로우 토큰은 워크플로우의 자신의 저장소에만 액세스할 수 있으므로 다른 저장소의 비공개 마켓플레이스는 개인 액세스 토큰 또는 앱 토큰이 필요합니다.
</Note>

<Steps>
  <Step title="빌드 시간에 시드에 설치">
    `CLAUDE_CODE_PLUGIN_CACHE_DIR`을 시드 경로로 설정하여 마켓플레이스 및 플러그인이 `~/.claude/plugins` 대신 그곳에 설치되도록 하세요:

    ```bash theme={null}
    CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin marketplace add your-org/your-marketplace
    CLAUDE_CODE_PLUGIN_CACHE_DIR=/opt/claude-seed claude plugin install code-formatter@your-marketplace
    ```

    시드는 `~/.claude/plugins`와 같은 레이아웃을 가집니다: `known_marketplaces.json`, `marketplaces/<name>/`, 및 `cache/<marketplace>/<plugin>/<version>/`. 시드를 빌드한 경로와 다른 경로에 마운트할 수 있습니다.
  </Step>

  <Step title="런타임을 시드로 가리키기">
    컨테이너의 환경에서 `CLAUDE_CODE_PLUGIN_SEED_DIR=/opt/claude-seed`를 설정하세요. 여러 시드를 사용하려면 Unix에서는 `:`로, Windows에서는 `;`로 경로를 분리하세요. Claude Code는 주어진 마켓플레이스 또는 플러그인 캐시를 포함하는 첫 번째 시드를 사용합니다.
  </Step>

  <Step title="플러그인 활성화">
    시드의 플러그인은 자동으로 활성화되지 않습니다. 관리되는 설정 또는 저장소의 `.claude/settings.json`에서 로드하려는 각 시드 플러그인에 대해 `enabledPlugins`을 설정하세요.
  </Step>
</Steps>

시드를 확인하려면 이미지에서 `--output-format stream-json --verbose`를 사용하여 `claude -p`를 실행하세요. `init` 이벤트의 `plugins` 목록에서 각 로드된 플러그인의 `path`는 시드 아래에 있습니다(예: `/opt/claude-seed/cache/your-marketplace/code-formatter/1.0.0`).

시드 마켓플레이스는 다음 규칙을 따릅니다:

* **읽기 전용**: Claude Code는 시드에 절대 쓰지 않으며 시드 마켓플레이스에 대해 `autoUpdate`를 강제로 끕니다.
* **시드 항목이 우선**: 각 시작 시 시드에서 선언된 마켓플레이스는 같은 이름의 사용자 항목을 덮어씁니다. 사용자는 마켓플레이스를 제거하는 대신 `claude plugin disable`로 시드 플러그인을 거부합니다.
* **업데이트 및 제거 실패**: `claude plugin marketplace update <name>` 및 시드 마켓플레이스에서 `--scope` 없이 `remove`하면 시드 디렉토리의 이름을 지정하는 메시지와 함께 실패합니다.
* **정책이 여전히 적용됨**: [허용 목록 및 차단 목록](#restrict-what-users-can-install)은 시드 마켓플레이스의 기록된 소스도 확인합니다. 시드를 빌드한 소스를 허용하세요.

아웃바운드 git 액세스가 없는 플릿의 경우 시드를 공유 마운트의 `directory` 또는 `file` 마켓플레이스 소스와 결합하세요. `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1`도 설정하세요. 이는 또한 [플러그인 자동 업데이트](/docs/ko/plugins/loading#when-auto-update-runs)를 끕니다. 프록시를 사용할 수 있으면 [프록시 구성](/docs/ko/network-config#proxy-configuration)에서 설정할 변수를 참조하세요.

<h2 id="restrict-what-users-can-install">
  사용자가 설치할 수 있는 것 제한
</h2>

관리되는 `strictKnownMarketplaces` 허용 목록 및 `blockedMarketplaces` 차단 목록은 플러그인이 올 수 있는 마켓플레이스 소스를 결정합니다. 마켓플레이스의 소스는 Claude Code가 가져오는 git 저장소, URL 또는 로컬 경로입니다. 두 목록 모두 플러그인이 오는 마켓플레이스의 소스와 일치하며, 해당 마켓플레이스 내의 플러그인 자신의 항목과는 일치하지 않습니다.

공식 마켓플레이스 및 자신의 마켓플레이스를 허용하는 일반적인 잠금의 경우 [공식 마켓플레이스 및 자신의 마켓플레이스 허용](#allow-the-official-marketplace-and-your-own)을 참조하세요. 사용자가 로컬 디렉토리 또는 URL에서 플러그인을 로드할 수 없도록 [`disableSideloadFlags`](#control-matrix)와 쌍을 이루세요.

두 목록 모두 다운로드 전과 세션 시작 시 적용됩니다:

* **다운로드 전**: 목록은 사용자가 마켓플레이스를 추가할 때 및 모든 설치, 업데이트, 새로 고침 및 자동 업데이트 시 적용됩니다.
* **세션 시작 시**: 목록은 이미 설치된 플러그인에 다시 적용되므로 마켓플레이스 소스가 더 이상 일치하지 않는 설치된 플러그인은 로드되지 않습니다. `/plugin`은 `Marketplace "<name>" is not in the allowed marketplace list` 또는 `Marketplace "<name>" is blocked by enterprise policy`로 나열합니다.

두 목록이 적용되는 위치는 설정하는 위치에 따라 다릅니다:

* **claude.ai 관리자 콘솔**: Claude Code는 [서버 관리 설정을 읽는](/docs/ko/managed-settings#where-and-when-a-policy-applies) 세션에서 두 목록을 적용합니다. claude.ai는 또한 조직의 누군가가 claude.ai에서 git 저장소에서 새 마켓플레이스를 추가하거나 Claude Desktop 앱의 Code 탭 외부에서 **사용자 정의**에서 추가할 때 확인합니다. 이는 멤버가 자신의 계정에 대해 추가하는 마켓플레이스 및 [**조직 설정 > 플러그인**](https://claude.ai/admin-settings/plugins) 아래에서 전체 조직에 대해 추가되는 마켓플레이스를 다룹니다. claude.ai는 허용 목록이 허용하지 않거나 차단 목록이 지정하는 저장소를 거부합니다. 목록을 설정하기 전에 어느 곳에서든 추가된 마켓플레이스를 다시 확인하지 않으며, 업로드된 플러그인을 확인하지 않습니다.
* **관리되는 설정 파일, OS 수준 정책 또는 기타 관리되는 소스**: Claude Code는 해당 소스를 읽는 곳에서 두 목록을 적용합니다. claude.ai는 이를 읽지 않습니다.

허용 목록이 설정되어 있거나 차단 목록이 [`skills-dir`](#blocklist-with-blockedmarketplaces) 이외의 소스를 지정하는 동안 Claude Code가 찾을 수 없는 마켓플레이스의 플러그인은 로드되지 않습니다. `/plugin`은 찾을 수 없음 오류 대신 정책 오류를 표시합니다. 일반적인 경우는 아무도 등록하지 않은 마켓플레이스에 대한 오래된 `enabledPlugins` 항목입니다.

<h3 id="control-matrix">
  제어 매트릭스
</h3>

표는 각 플러그인 정책 키, 적용하는 것 및 할 수 없는 것을 나열합니다.

| 키                                                                        | 적용하는 것                                                                                                                                                                                      | 할 수 없는 것                                                                                                                    |
| :----------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------- |
| `strictKnownMarketplaces`                                                | 마켓플레이스 소스의 허용 목록. `[]`는 공식 마켓플레이스를 포함한 모든 소스를 차단합니다. 별칭: `allowedMarketplaces`                                                                                                              | 마켓플레이스를 등록하거나, 허용된 마켓플레이스 내의 항목을 제한하거나, `--plugin-dir`을 차단하지 않습니다                                                           |
| `blockedMarketplaces`                                                    | 마켓플레이스 소스의 차단 목록, 허용 목록 전에 확인됨                                                                                                                                                              | 이미 일치하지 않는 소스에서 등록된 마켓플레이스를 차단하지 않습니다                                                                                       |
| `syncClaudeAiPlugins`                                                    | 각 사용자의 계정에서 [동기화된](/docs/ko/plugins/loading#synced-plugins) 플러그인을 Claude Code가 다운로드하고 로드하는 것을 중지하려면 `false`로 설정하세요. Claude Code v2.1.273 이상 필요                                                   | 하나의 동기화된 플러그인을 끄지 않습니다. 그렇게 하려면 [`enabledPlugins`](/docs/ko/settings-reference#enabledplugins)에서 `"<name>@synced": false`를 설정하세요 |
| `enabledPlugins`                                                         | `true`는 강제 활성화하고, `false`는 모든 범위에서 차단하고 플러그인을 숨깁니다                                                                                                                                          | 마켓플레이스가 등록되거나 허용되지 않은 플러그인을 설치하지 않습니다                                                                                       |
| `disableSideloadFlags`                                                   | `--plugin-dir`, `--plugin-url`, `--agents`, Agent SDK `plugins` 옵션 및 비 SDK `--mcp-config`를 시작 시 거부하고, [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/ko/env-vars#variables) 변수에 지정된 폴더를 같은 방식으로 거부합니다         | `.mcp.json`, `claude mcp add` 또는 SDK 제공 서버를 제한하지 않습니다. [`allowedMcpServers`](/docs/ko/managed-mcp)와 쌍을 이루세요                      |
| `disableCommandPluginSources`                                            | `command` 소스가 있는 플러그인이 설치, 업데이트 또는 로드되는 것을 차단합니다. `command` 소스는 플러그인 디렉토리가 머신에서 명령을 실행하여 생성되는 것입니다. 설정되지 않으면 `allowManagedHooksOnly`의 값을 사용합니다                                              | 다른 소스 유형에 영향을 주지 않습니다                                                                                                       |
| `allowManagedHooksOnly`                                                  | 실행할 수 있는 훅을 제한합니다. [`allowManagedHooksOnly`](/docs/ko/settings-reference#allowmanagedhooksonly)를 참조하세요                                                                                           | 사용자가 자신이 활성화한 플러그인의 훅을 신뢰하지 않습니다                                                                                            |
| `strictPluginOnlyCustomization`                                          | 플러그인, 관리되는 설정 또는 Claude Code의 기본 제공에서 오지 않는 스킬, 에이전트, 훅 및 MCP 서버를 차단합니다. 모든 네 가지 유형을 다루려면 `true`로 설정하거나 일부를 다루려면 `skills`, `agents`, `hooks` 및 `mcp` 값의 배열(예: `["skills", "hooks"]`)로 설정하세요 | 사용자가 설치하는 플러그인을 제한하지 않습니다. `strictKnownMarketplaces`와 쌍을 이루세요                                                               |
| `pluginSuggestionMarketplaces`                                           | 플러그인이 설치 제안으로 나타날 수 있는 마켓플레이스. [플러그인 권장](#recommend-plugins)을 참조하세요                                                                                                                         | 기본 제공 팁에 영향을 주지 않습니다                                                                                                        |
| `pluginTrustMessage`                                                     | 플러그인이 설치되기 전에 `/plugin`이 표시하는 신뢰 경고에 텍스트를 추가합니다                                                                                                                                             | 경고 자신의 텍스트를 변경하지 않습니다                                                                                                       |
| `allowedChannelPlugins`                                                  | 채널 메시지를 푸시할 수 있는 플러그인의 기본 목록을 대체합니다. `channelsEnabled: true` 필요                                                                                                                             | [채널 플러그인이 실행할 수 있는 것 제한](/docs/ko/channels#restrict-which-channel-plugins-can-run)을 참조하세요                                        |
| [`CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL=1`](/docs/ko/env-vars) | 대화형 터미널 세션이 공식 마켓플레이스를 자동 등록하는 것을 중지합니다                                                                                                                                                     | 이미 등록된 마켓플레이스를 제거하지 않습니다. 허용 목록 및 차단 목록은 이 없이도 같은 자동 등록을 제어합니다. 이를 설정하여 시작한 머신은 설정을 해제한 후 자동 등록을 재개하지 않습니다                  |

표의 모든 키는 `enabledPlugins`, `syncClaudeAiPlugins` 및 `CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL` 제외하고 관리되는 설정입니다:

* **`enabledPlugins`**: 모든 범위에서 설정할 수 있으며 관리되는 설정이 이를 잠급니다.
* **`syncClaudeAiPlugins`**: 각 사용자는 자신의 사용자 또는 로컬 설정에서도 설정할 수 있습니다. [설정 참조에서 해당 범위](/docs/ko/settings-reference#syncclaudeaiplugins)를 참조하세요.
* **`CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL`**: 이는 [전체 플릿에 대해 업데이트 끄기](#turn-updates-off-for-the-whole-fleet) 아래에 표시된 관리되는 `env` 블록을 통해 제공하는 환경 변수입니다.

여기의 각 설정 키는 [설정 참조](/docs/ko/settings-reference)에 항목이 있습니다.

<h4 id="aliases-for-the-marketplace-keys">
  마켓플레이스 키의 별칭
</h4>

`strictKnownMarketplaces`는 `allowedMarketplaces`로도 철자할 수 있고, `extraKnownMarketplaces`는 `additionalMarketplaces`로도 철자할 수 있습니다.

* **버전**: 별칭은 Claude Code v2.1.232 이상이 필요하며, 이전 클라이언트는 이를 무시합니다. 혼합 플릿이 읽는 파일에서 정규 이름을 유지하세요.
* **두 철자 모두 설정**: 파일이 두 철자를 모두 설정할 때 정규 키의 값이 적용됩니다.

<h3 id="allowlist-with-strictknownmarketplaces">
  `strictKnownMarketplaces`를 사용한 허용 목록
</h3>

허용 목록을 이 소스 객체의 목록으로 설정하세요. 대부분의 항목은 정확히 일치하고, `hostPattern` 및 `pathPattern` 항목은 정규식으로 일치하며, `github` 소유자 와일드카드는 소유자별로 일치합니다:

* **`github`**: `{ "source": "github", "repo": "your-org/approved-plugins" }`, 선택적 `ref` 및 `path` 포함.
* **`github` 소유자 와일드카드**: `{ "source": "github", "repo": "your-org/*" }`는 해당 소유자 아래의 모든 저장소와 일치합니다. `*`는 전체 저장소 이름을 나타내야 합니다. Claude Code는 `*/plugins` 및 `your-org/tools-*`와 같은 항목을 유효하지 않은 것으로 무시하므로 아무것도 일치하지 않습니다. Claude Code v2.1.223 이상 필요.
* **`git`**: `{ "source": "git", "url": "https://gitlab.example.com/tools/plugins.git" }`, 선택적 `ref` 및 `path` 포함.
* **`url`**: `{ "source": "url", "url": "https://plugins.example.com/marketplace.json" }`, 선택적 `headers` 포함.
* **`file` 및 `directory`**: `{ "source": "file", "path": "/opt/marketplace/marketplace.json" }` 또는 `{ "source": "directory", "path": "/opt/marketplace/plugins" }`, 절대 경로 포함.
* **`hostPattern`**: `{ "source": "hostPattern", "hostPattern": "^github\\.example\\.com$" }`, `github`, `git` 및 `url` 소스의 호스트에 대해 일치합니다. 패턴은 호스트명의 어디든 일치하므로 전체 호스트를 일치시키려면 표시된 대로 `^` 및 `$`로 고정하세요. `github` 소스는 항상 `github.com`으로 계산됩니다. 개발자가 자신의 마켓플레이스를 만드는 GitHub Enterprise Server 또는 GitLab 호스트에 `hostPattern` 항목을 사용하세요. [GHES 페이지](/docs/ko/github-enterprise-server#allowlist-ghes-marketplaces-in-managed-settings)에 작동하는 예제가 있습니다.
* **`pathPattern`**: `{ "source": "pathPattern", "pathPattern": "^/opt/approved/" }`, `file` 및 `directory` 소스의 `path`에 대해 일치합니다. 패턴은 경로의 어디든 일치하므로 디렉토리 접두사를 고정하려면 `^`로 시작하세요. `".*"`는 모든 로컬 경로를 허용합니다.
* **`skills-dir`**: `{ "source": "skills-dir" }` 허용 목록이 설정되어 있는 동안 [스킬 디렉토리 플러그인](#keep-skills-directory-plugins-loading)이 로드되도록 유지하고 마켓플레이스와 일치하지 않습니다.

<h4 id="how-entries-match">
  항목이 일치하는 방법
</h4>

`url` 항목은 `url` 값에 대해 일치합니다; `headers`는 비교되지 않습니다. `github` 및 `git` 항목의 경우 `repo` 또는 `url`, `ref` 및 `path`는 모두 일치하거나 양쪽 모두 없어야 합니다:

* `ref` 없는 항목은 `ref: "main"`이 있는 소스를 다루지 않습니다.
* `your-org/your-marketplace`에 대한 항목은 같은 저장소를 복제하는 `git` URL을 다루지 않습니다.
* 후행 슬래시, `.git` 접미사 또는 `https://` 대신 `ssh://`는 다른 값입니다. 마켓플레이스를 둘 이상의 URL로 복제할 수 있으면 `hostPattern` 항목을 선호하세요.

소유자 와일드카드 항목은 `ref`에 대한 정확한 규칙을 따르고 항목이 하나를 고정하지 않으면 저장소 내의 모든 `path`와 일치합니다. 와일드카드 일치는 허용 목록에서 대소문자를 구분합니다.

<h4 id="keep-skills-directory-plugins-loading">
  스킬 디렉토리 플러그인 로드 유지
</h4>

스킬 디렉토리 플러그인은 사용자가 `~/.claude/skills/` 또는 프로젝트의 `.claude/skills/` 아래에 `.claude-plugin/plugin.json`을 포함하는 폴더에 보관하는 플러그인입니다. `{ "source": "skills-dir" }` 항목 없이 허용 목록을 설정하면 로드되지 않습니다. 해당 매니페스트 없는 일반 [스킬](/docs/ko/skills)(즉, `SKILL.md`)은 계속 로드됩니다.

<h4 id="marketplaces-hosted-on-claude-ai">
  claude.ai에서 호스팅되는 마켓플레이스
</h4>

허용 목록 및 차단 목록은 [claude.ai에서 호스팅되는](/docs/ko/plugins/install#add-from-claude-ai) 마켓플레이스를 해당 호스트별로 일치시킵니다. 하나를 허용하거나 차단하려면 `claude.ai`와 일치하는 `hostPattern` 항목을 `strictKnownMarketplaces` 또는 `blockedMarketplaces`에 추가하세요. 허용 목록에서 이러한 항목은 조직의 claude.ai 마켓플레이스 및 claude.ai 기본 마켓플레이스를 허용하지만 멤버의 자신의 claude.ai 업로드로 만든 마켓플레이스 또는 범위를 claude.ai가 명시하지 않은 마켓플레이스는 허용하지 않습니다. Claude Code v2.1.273 이상 필요.

<h4 id="lock-every-source-out">
  모든 소스 잠금
</h4>

빈 허용 목록 `[]`는 공식 마켓플레이스를 포함한 모든 마켓플레이스 소스를 잠급니다.

이 잠금은 Claude Code가 마켓플레이스가 아닌 각 사용자의 계정에서 다운로드하는 [claude.ai에서 동기화된](/docs/ko/plugins/loading#synced-plugins) 플러그인을 다루지 않습니다. 이를 중지하려면 관리되는 설정에서 [`syncClaudeAiPlugins`](/docs/ko/settings-reference#syncclaudeaiplugins)를 `false`로 설정하거나 claude.ai에서 조직에 대해 스킬을 끄세요.

<h3 id="blocklist-with-blockedmarketplaces">
  `blockedMarketplaces`를 사용한 차단 목록
</h3>

`blockedMarketplaces`는 [`strictKnownMarketplaces`](#allowlist-with-strictknownmarketplaces)와 같은 소스 객체를 사용하며 먼저 확인되므로 두 목록 모두에 있는 소스는 차단됩니다. 차단 목록 일치는 허용 목록 일치보다 더 넓습니다:

* Git URL은 정규화되므로 하나의 `github.com` 저장소의 `git@` 및 `https://` 형식, `.git` 접미사 및 후행 슬래시는 모두 같은 항목과 일치합니다.
* `github` 항목은 또한 동등한 `git` URL을 차단하고, 그 반대도 마찬가지입니다.
* `owner/*` 항목의 경우 소유자 비교는 대소문자를 구분하지 않습니다.
* `ref` 또는 `path` 없는 항목은 일치하는 저장소의 모든 ref 및 경로를 차단합니다.

이 항목은 하나의 GitHub 소유자 아래의 모든 저장소를 차단합니다:

```json theme={null}
{
  "blockedMarketplaces": [
    { "source": "github", "repo": "untrusted-org/*" }
  ]
}
```

`blockedMarketplaces`의 `url` 항목은 또한 사용자가 Claude Code가 [가져오는 대신 복제하는](/docs/ko/plugins/cli-reference#plugin-marketplace-add) `https://` 저장소 URL(예: 일반 `github.com` 또는 `gitlab.com` 저장소 URL)을 추가할 때 적용됩니다. 사용자는 항목이 이를 지정하면 해당 URL을 추가할 수 없습니다. 일치는 `.git` 접미사 및 사용자가 `#` 뒤에 추가하는 모든 ref를 무시합니다. Claude Code v2.1.232 이상 필요.

`{ "source": "skills-dir" }` 항목은 `~/.claude/skills/` 및 프로젝트의 `.claude/skills/` 모두에서 [스킬 디렉토리 플러그인](#keep-skills-directory-plugins-loading)이 로드되는 것을 중지합니다.

해당 항목만 지정하는 차단 목록은 활성 제한으로 계산되지 않으므로 Claude Code가 찾을 수 없는 마켓플레이스의 [플러그인이 로드되는 것을 중지하지 않습니다](#restrict-what-users-can-install).

<h3 id="allow-the-official-marketplace-and-your-own">
  공식 마켓플레이스 및 자신의 마켓플레이스 허용
</h3>

대부분의 조직은 공식 마켓플레이스 및 자신의 마켓플레이스를 허용하고 모든 머신이 이를 가지도록 둘 다 등록합니다. 이 관리되는 설정 정책은 두 마켓플레이스를 허용하고, 둘 다 등록하고, 두 개의 플러그인을 강제 활성화하고, `--plugin-dir`을 거부합니다:

```json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "anthropics/claude-plugins-official" },
    { "source": "github", "repo": "your-org/*" },
    { "source": "skills-dir" }
  ],
  "extraKnownMarketplaces": {
    "claude-plugins-official": {
      "source": { "source": "github", "repo": "anthropics/claude-plugins-official" }
    },
    "your-marketplace": {
      "source": { "source": "github", "repo": "your-org/your-marketplace" }
    }
  },
  "enabledPlugins": {
    "code-formatter@your-marketplace": true,
    "deploy-helper@your-marketplace": true
  },
  "disableSideloadFlags": true
}
```

이 정책이 있는 머신에서 목록 외부의 소스를 추가하려고 시도하면(예: `/plugin marketplace add https://example.com/other-marketplace.git`) `is blocked by enterprise policy` 다음에 허용된 소스를 포함하는 메시지와 함께 실패합니다. `claude --plugin-dir ./x`는 `disableSideloadFlags`의 이름을 지정하는 메시지와 함께 종료됩니다.

`{ "source": "skills-dir" }` 항목은 이 허용 목록 아래에서 [스킬 디렉토리 플러그인](#keep-skills-directory-plugins-loading)이 로드되도록 유지합니다. 해당 항목을 제거하면 로드되지 않습니다.

이 정책이 하는 것처럼 명시적 `extraKnownMarketplaces` 항목으로 두 마켓플레이스를 등록하세요. 허용 목록 또는 공식 마켓플레이스가 자신을 등록하는 것에 의존하지 마세요:

* **허용 목록은 아무것도 등록하지 않습니다**: `extraKnownMarketplaces` 항목이 등록하고, 자신이 허용 목록을 통과해야 합니다. Claude Code는 소스가 허용 목록과 일치하지 않는 관리되는 마켓플레이스를 등록하기를 거부합니다.
* **공식 마켓플레이스는 대화형 터미널 세션에서만 자신을 등록합니다**: 거기서도 허용 목록이 이를 허용할 때만 등록합니다. `-p` 실행 또는 클라우드 세션에 연결된 터미널은 절대 등록하지 않습니다.
* **차단된 시도는 기억됩니다**: 머신이 공식 마켓플레이스를 차단한 정책 아래에서 실행된 적이 있으면 Claude Code는 차단된 시도를 기록하고 정책이 변경된 후 재시도하지 않습니다. `[]` 잠금은 그러한 정책 중 하나입니다. 해당 머신은 이 정책의 항목과 같은 `extraKnownMarketplaces` 항목, 해당 플러그인 중 하나에 대한 `enabledPlugins` 항목 또는 수동 `/plugin marketplace add`를 통해서만 다시 등록합니다.

<h2 id="set-update-policy">
  업데이트 정책 설정
</h2>

마켓플레이스별, 전체 플릿 또는 릴리스 채널을 통한 사용자 그룹별로 업데이트 정책을 설정할 수 있습니다.

<h3 id="turn-auto-update-on-or-off-per-marketplace">
  마켓플레이스별 자동 업데이트 켜기 또는 끄기
</h3>

플러그인 자동 업데이트는 시작 후 켜진 마켓플레이스에 대해 백그라운드에서 실행됩니다. 기본적으로 어떤 마켓플레이스가 켜져 있는지는 [자동 업데이트가 실행되는 시기](/docs/ko/plugins/loading#when-auto-update-runs)를 참조하세요. 플릿에 대해 결정하려면 관리되는 `extraKnownMarketplaces` 항목에서 `"autoUpdate": true` 또는 `false`를 설정하세요:

* 관리되는 항목이 필드를 설정하면 Claude Code는 사용자의 `/plugin` 토글을 `Auto-update for '<name>' is set by`로 시작하는 오류로 거부합니다.
* 관리되는 항목이 필드를 설정하지 않으면 사용자의 토글이 유지됩니다.

<h3 id="turn-updates-off-for-the-whole-fleet">
  전체 플릿에 대해 업데이트 끄기
</h3>

모든 마켓플레이스에 대해 플러그인 자동 업데이트를 끄려면 이 예제처럼 관리되는 `env` 블록에서 `DISABLE_AUTOUPDATER`를 설정하세요. 같은 변수는 또한 Claude Code 자신의 업데이트를 중지합니다:

```json theme={null}
{
  "env": {
    "DISABLE_AUTOUPDATER": "1"
  }
}
```

Claude Code 자신의 업데이트는 중지하지만 플러그인 자동 업데이트는 유지하려면 같은 블록에 `"FORCE_AUTOUPDATE_PLUGINS": "1"`을 추가하세요. [플러그인 자동 업데이트를 중지하는 다른 환경 변수](/docs/ko/plugins/loading#when-auto-update-runs)는 같은 방식으로 작동합니다.

`DISABLE_AUTOUPDATER`는 [`command` 소스](/docs/ko/plugins/marketplace-reference#command-plugin-source)가 있는 플러그인을 다루지 않습니다. Claude Code는 매 세션마다 활성화된 각 플러그인의 명령을 다시 실행하고 변경되었을 때 출력을 설치합니다. 이를 중지하는 것은 [명령 소스가 다시 실행되는 시기](/docs/ko/plugins/loading#when-a-command-source-re-runs)를 참조하세요.

<h3 id="assign-release-channels-to-user-groups">
  사용자 그룹에 릴리스 채널 할당
</h3>

안정적 및 조기 액세스 채널을 실행하려면 같은 플러그인의 다른 ref를 가리키는 두 개의 마켓플레이스를 호스팅하세요. 그 다음 각 사용자 그룹에 별도의 엔드포인트 관리 설정 또는 게이트웨이 정책을 통해 자신의 마켓플레이스를 제공하세요. 관리자 콘솔의 서버 관리 설정은 [조직의 모든 사용자에게 적용되므로](/docs/ko/server-managed-settings#current-limitations) 다른 그룹에 다른 설정을 할당할 수 없습니다.

* 각 그룹의 장치에 별도의 [엔드포인트 관리 설정](/docs/ko/managed-settings#delivery-mechanisms)(예: 관리되는 설정 파일 또는 MDM 프로필)을 배포하세요. 조직 전체 소스도 있는 장치에 그룹별 파일 또는 프로필이 적용되는지 확인하려면 [Claude Code가 관리되는 소스를 결합하는 방법](/docs/ko/managed-settings#precedence-within-the-managed-tier)을 참조하세요.
* 각 그룹에 대해 하나의 [Claude 앱 게이트웨이 정책](/docs/ko/claude-apps-gateway-config#managed)을 정의하세요. 게이트웨이는 일치 규칙이 사용자에게 맞는 첫 번째 정책을 적용하므로 각 사용자가 자신의 그룹의 정책에 도달하도록 정책을 정렬하세요. 해당 정책의 `extraKnownMarketplaces` 맵은 다른 정책의 것과 병합되지 않으므로 채널 마켓플레이스만이 아닌 그룹이 필요한 모든 마켓플레이스를 나열하세요.

어느 메커니즘이든 안정적 그룹은 이 구성을 받습니다:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "stable-tools": {
      "source": { "source": "github", "repo": "your-org/stable-tools" }
    }
  }
}
```

조기 액세스 그룹은 대신 `latest-tools`를 받습니다. 두 마켓플레이스를 설정하려면 [릴리스 채널 실행](/docs/ko/plugins/host-marketplace#run-release-channels)을 참조하세요.

<h2 id="recommend-plugins">
  플러그인 권장
</h2>

마켓플레이스 소유자는 항목에 `relevance` 신호를 첨부하여 Claude Code가 프로젝트가 일치할 때 플러그인을 제안하도록 할 수 있습니다.

마켓플레이스의 제안은 사용자의 머신에 등록되어 있고, 관리되는 설정에서 `pluginSuggestionMarketplaces`에 해당 이름을 나열하고, 같은 정책에서 해당 소스를 선언할 때만 나타납니다. 소스를 마켓플레이스의 `extraKnownMarketplaces` 항목 또는 허용 목록 항목으로 선언하세요. 공식 마켓플레이스는 이름만 필요합니다. [관리되는 설정에서 제안 활성화](/docs/ko/plugins/relevance#enable-suggestions-in-managed-settings)를 참조하세요.

<h2 id="audit-and-review">
  감사 및 검토
</h2>

OpenTelemetry 이벤트 및 Analytics API는 플릿이 설치하고 실행하는 것을 알려줍니다.

플러그인이 머신에서 실행할 수 있는 것 및 각 신뢰 계층이 허용하는 것은 마켓플레이스를 승인하기 전에 [플러그인 보안](/docs/ko/plugins/security)을 읽으세요.

<h3 id="opentelemetry-events">
  OpenTelemetry 이벤트
</h3>

`claude_code.plugin_installed`는 각 설치를 기록하고, `claude_code.plugin_loaded`는 세션 시작 시 각 활성화된 플러그인을 기록합니다. 두 이벤트 모두 `OTEL_LOG_TOOL_DETAILS=1`을 설정하지 않으면 타사 플러그인 및 마켓플레이스 이름을 수정하거나 생략합니다([백엔드의 수정된 플러그인 이름](/docs/ko/plugins/measure#redacted-plugin-names-in-your-backend) 참조). 필드 목록은 [플러그인 설치 이벤트](/docs/ko/monitoring-usage#plugin-installed-event) 및 [플러그인 로드 이벤트](/docs/ko/monitoring-usage#plugin-loaded-event) 아래에 있습니다.

<h3 id="analytics-api">
  Analytics API
</h3>

Enterprise 플랜에서 `GET /v1/organizations/analytics/plugins`는 Claude Code 및 Cowork 전체에서 플러그인별, 일별 설치 및 호출 수를 반환합니다. 사용자 또는 RBAC 그룹별로 수를 그룹화할 수 있습니다. Anthropic에 도달하는 플러그인 이름 없는 플러그인 활동은 하나의 집계 `third-party` 행에 나타납니다. [엔드포인트 참조](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list) 및 [프로그래밍 방식으로 데이터 액세스](/docs/ko/analytics#access-data-programmatically)에서 필요한 키를 참조하세요.

<h2 id="plan-for-what-managed-settings-can’t-enforce">
  관리되는 설정이 적용할 수 없는 것에 대해 계획
</h2>

보안 검토의 이 요청은 현재 설정 스키마에 전용 키가 없습니다. 가장 가까운 기존 제어는:

* **사용자별 또는 그룹별 타겟팅**: 모든 플러그인 키는 설정을 받는 모든 사용자에게 적용됩니다. 서버 관리 설정은 조직별로 하나의 구성을 제공합니다. 그룹별 정책의 경우 [사용자 그룹에 릴리스 채널 할당](#assign-release-channels-to-user-groups) 아래처럼 별도의 엔드포인트 관리 설정 또는 게이트웨이 정책을 사용하세요.
* **허용된 마켓플레이스 내의 항목 제한**: 허용 목록은 마켓플레이스 소스와 일치합니다. 허용된 마켓플레이스에서 하나의 플러그인을 차단하려면 관리되는 `enabledPlugins`에서 `false`로 설정하세요.
* **`/plugin` 숨기기**: 명령을 비활성화하는 키가 없습니다. 가장 가까운 동등은 자신의 마켓플레이스만 지정하는 허용 목록, 제공하는 플러그인에 대한 관리되는 `enabledPlugins` 항목 및 `disableSideloadFlags`를 결합합니다.
* **허용 목록을 통해 `--plugin-dir` 제어**: 허용 목록은 `--plugin-dir`을 다루지 않습니다. `disableSideloadFlags`는 다룹니다.
* **이 키를 통해 claude.ai 플러그인 토글 적용**: [**조직 설정 > 플러그인 및 스킬**](https://claude.ai/admin-settings/skills?tab=inventory)은 이 페이지의 키를 설정하지 않습니다. 멤버 및 조직이 거기서 켜는 것은 CLI에 [동기화된 플러그인](/docs/ko/plugins/loading#synced-plugins)으로 도달하며, 자신의 제어가 있습니다.

<h2 id="troubleshoot-policy">
  정책 문제 해결
</h2>

플러그인 정책이 머신에서 예상대로 작동하지 않으면 먼저 이 증상을 확인하세요:

* **관리되는 파일이 파싱되지 않음**: `managed-settings.json`이 유효한 JSON이 아니면 Claude Code는 시작을 거부하고 [파일의 이름을 지정하는 오류](/docs/ko/errors#managed-settings-document-could-not-be-parsed)를 인쇄합니다. 파싱되지만 하나의 유효하지 않은 항목이 있는 파일은 나머지 정책을 유지합니다. [관리되는 설정의 유효하지 않은 항목](/docs/ko/managed-settings#invalid-entries-in-managed-settings)을 참조하세요.
* **관리되는 소스가 로드되지 않음**: `/status`를 실행하고 `Setting sources` 행에서 `Enterprise managed settings`를 찾으세요. 누락되면 소스가 로드되지 않았습니다.
* **사용자가 `blocked by enterprise policy` 보고**: 메시지는 마켓플레이스 또는 해당 소스의 이름을 지정합니다. 허용 목록의 경우 허용된 소스도 나열합니다. 사용자 대면 항목은 [플러그인 문제 해결](/docs/ko/plugins/troubleshooting)에 있습니다.
* **사용자가 `~/.claude/settings.json`에서 비활성화한 플러그인이 여전히 로드됨**: 다른 설정 소스가 다시 활성화했습니다(예: 강제 활성화하는 관리되는 `enabledPlugins` 항목). `/plugin` 및 `claude plugin list`는 `Disabled in ~/.claude/settings.json but still loads`를 해당 설정 소스와 함께 표시합니다.

<h2 id="next-steps">
  다음 단계
</h2>

* [마켓플레이스 참조](/docs/ko/plugins/marketplace-reference#marketplace-sources): `extraKnownMarketplaces`, `strictKnownMarketplaces` 및 `blockedMarketplaces`가 수락하는 `source` 값
* [마켓플레이스 호스팅 및 유지](/docs/ko/plugins/host-marketplace): 정책이 가리키는 마켓플레이스 실행
* [플러그인 보안 및 신뢰](/docs/ko/plugins/security): 플러그인이 머신에서 할 수 있는 것 및 설치 전에 하나를 검토하는 방법
* [서버 관리 설정](/docs/ko/server-managed-settings): claude.ai 관리자 콘솔에서 이 키를 제공
* [플러그인 문제 해결](/docs/ko/plugins/troubleshooting#blocked-by-your-organization): 정책이 사용자를 차단할 때 사용자가 보는 메시지
