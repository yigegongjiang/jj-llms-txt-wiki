> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 웹에서 Claude Code 사용하기

> 클라우드 환경, 설정 스크립트, 네트워크 액세스 및 Docker를 Anthropic의 샌드박스에서 구성합니다. `--cloud` 및 `--teleport`를 사용하여 웹과 터미널 간에 세션을 이동합니다.

<Note>
  웹에서 Claude Code는 Pro, Max 및 Team 사용자, 그리고 프리미엄 시트 또는 Chat + Claude Code 시트가 있는 Enterprise 사용자를 위한 연구 미리보기 상태입니다.
</Note>

웹에서 Claude Code는 [claude.ai/code](https://claude.ai/code)의 Anthropic 관리 클라우드 인프라에서 작업을 실행합니다. 세션은 브라우저를 닫아도 유지되며, Claude 모바일 앱에서 모니터링할 수 있습니다.

<Tip>
  웹에서 Claude Code를 처음 사용하시나요? [시작하기](/docs/ko/web-quickstart)에서 GitHub 계정을 연결하고 첫 번째 작업을 제출하세요.
</Tip>

이 페이지에서 다루는 내용:

* [GitHub 인증 옵션](#github-authentication-options): GitHub를 연결하는 두 가지 방법
* [클라우드 환경](#the-cloud-environment): 어떤 구성이 이월되는지, 어떤 도구가 설치되어 있는지, 환경을 구성하는 방법
* [설정 스크립트](#setup-scripts) 및 종속성 관리
* [네트워크 액세스](#network-access): 수준, 프록시 및 기본 허용 목록
* [`--cloud` 및 `--teleport`를 사용하여 웹과 터미널 간에 작업 이동](#move-tasks-between-web-and-terminal)
* [세션 작업](#work-with-sessions): 검토, 공유, 보관, 삭제
* [Pull request 자동 수정](#auto-fix-pull-requests): CI 실패 및 검토 주석에 자동으로 응답
* [보안 및 격리](#security-and-isolation): 세션이 어떻게 격리되는지
* [제한 사항](#limitations): 속도 제한 및 플랫폼 제한

<h2 id="github-authentication-options">
  GitHub 인증 옵션
</h2>

클라우드 세션은 코드를 복제하고 분기를 푸시하기 위해 GitHub 저장소에 액세스해야 합니다. 두 가지 방법으로 액세스 권한을 부여할 수 있습니다:

| 방법               | 작동 방식                                                       | 최적 대상                                             |
| :--------------- | :---------------------------------------------------------- | :------------------------------------------------ |
| **GitHub App**   | [웹 온보딩](/docs/ko/web-quickstart) 중에 Claude GitHub App을 승인합니다.    | 브라우저 온보딩; [자동 수정](#auto-fix-pull-requests)을 원하는 팀 |
| **`/web-setup`** | 터미널에서 `/web-setup`을 실행하여 로컬 `gh` CLI 토큰을 Claude 계정과 동기화합니다. | 이미 `gh`를 사용하는 개별 개발자                              |

<Note>
  두 방법 모두에서 클라우드 세션은 Claude GitHub App이 설치된 저장소뿐만 아니라 연결된 GitHub 계정이 볼 수 있는 모든 저장소에 액세스할 수 있습니다. App 설치는 [자동 수정](#auto-fix-pull-requests)을 위한 PR 웹훅을 활성화합니다. 이는 세션 수준의 액세스 제어가 아닙니다. 클라우드 세션에서 팀이 도달할 수 있는 저장소를 제한하려면 GitHub 자체에서 액세스를 제한하세요. 예를 들어 연결된 GitHub 계정의 팀 또는 저장소 멤버십을 제한하면 됩니다.
</Note>

두 방법 모두 작동합니다. [`/schedule`](/docs/ko/routines)은 두 형태의 액세스를 확인하고 구성되지 않은 경우 `/web-setup`을 실행하라는 메시지를 표시합니다. `/web-setup` 안내는 [터미널에서 연결](/docs/ko/web-quickstart#connect-from-your-terminal)을 참조하세요.

GitHub App은 App을 사용하여 PR 웹훅을 수신하는 [자동 수정](#auto-fix-pull-requests)에 필요합니다. `/web-setup`으로 연결했다가 나중에 자동 수정을 원하면 해당 저장소에 App을 설치하세요.

Team 및 Enterprise 관리자는 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)의 Quick web setup 토글로 `/web-setup`을 비활성화할 수 있습니다.

<Note>
  [Zero Data Retention](/docs/ko/zero-data-retention)이 활성화된 조직은 `/web-setup` 또는 기타 클라우드 세션 기능을 사용할 수 없습니다.
</Note>

<h2 id="the-cloud-environment">
  클라우드 환경
</h2>

각 세션은 저장소가 복제된 새로운 Anthropic 관리 VM에서 실행됩니다. 이 섹션에서는 세션이 시작될 때 사용 가능한 것과 이를 사용자 정의하는 방법을 다룹니다.

<h3 id="what’s-available-in-cloud-sessions">
  클라우드 세션에서 사용 가능한 것
</h3>

클라우드 세션은 저장소의 새로운 복제본에서 시작됩니다. 저장소에 커밋된 모든 것이 사용 가능합니다. 자신의 머신에만 설치하거나 구성한 것은 사용할 수 없습니다. 조직의 정책은 [서버 관리 설정](/docs/ko/server-managed-settings)을 통해 별도로 도착합니다.

|                                                                     | 클라우드 세션에서 사용 가능 | 이유                                                                                                                                                                                                |
| :------------------------------------------------------------------ | :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 저장소의 `CLAUDE.md`                                                    | 예               | 복제본의 일부                                                                                                                                                                                           |
| 저장소의 `.claude/settings.json` hooks                                  | 예               | 복제본의 일부                                                                                                                                                                                           |
| 저장소의 `.mcp.json` MCP 서버                                             | 예               | 복제본의 일부                                                                                                                                                                                           |
| 저장소의 `.claude/rules/`                                               | 예               | 복제본의 일부                                                                                                                                                                                           |
| 저장소의 `.claude/skills/`, `.claude/agents/`, `.claude/commands/`      | 예               | 복제본의 일부                                                                                                                                                                                           |
| `.claude/settings.json`에 선언된 플러그인                                   | 예               | 선언한 [마켓플레이스](/docs/ko/plugin-marketplaces)에서 세션 시작 시 설치됩니다. 마켓플레이스 소스에 도달하려면 네트워크 액세스가 필요합니다                                                                                                           |
| 조직의 [서버 관리 설정](/docs/ko/server-managed-settings)                         | 예               | 세션이 시작될 때 Anthropic의 서버에서 가져옵니다. 클라우드 세션에서 `availableModels`이 어떻게 적용되는지는 [표면 범위](/docs/ko/model-config#surface-coverage)를 참조하세요. MDM 또는 관리 설정 파일을 통해 장치에 배포된 설정은 세션이 Anthropic 관리 VM에서 실행되므로 적용되지 않습니다 |
| 사용자 `~/.claude/CLAUDE.md`                                           | 아니오             | 저장소가 아닌 머신에 있습니다                                                                                                                                                                                  |
| 사용자 `~/.claude/skills/`, `~/.claude/agents/`, `~/.claude/commands/` | 아니오             | 저장소가 아닌 머신에 있습니다. 대신 저장소의 `.claude/` 디렉토리에 커밋하세요. claude.ai에서 활성화한 Skills는 클라우드 세션에 자동으로 로드됩니다                                                                                                    |
| 사용자 설정에서만 활성화된 플러그인                                                 | 아니오             | 사용자 범위 `enabledPlugins`는 `~/.claude/settings.json`에 있습니다. 저장소의 `.claude/settings.json`에 선언하세요                                                                                                     |
| `claude mcp add`로 추가한 MCP 서버                                        | 아니오             | 저장소가 아닌 로컬 사용자 구성에 씁니다. [`.mcp.json`](/docs/ko/mcp#project-scope)에 서버를 선언하세요                                                                                                                           |
| 정적 API 토큰 및 자격 증명                                                   | 아니오             | 아직 전용 비밀 저장소가 없습니다. 아래를 참조하세요                                                                                                                                                                     |
| AWS SSO와 같은 대화형 인증                                                  | 아니오             | 지원되지 않습니다. SSO는 클라우드 세션에서 실행할 수 없는 브라우저 기반 로그인이 필요합니다                                                                                                                                             |

클라우드 세션에서 구성을 사용 가능하게 하려면 저장소에 커밋하세요. 조직 정책은 [서버 관리 설정](/docs/ko/server-managed-settings)을 통해 별도로 도착합니다.

전용 비밀 저장소는 아직 사용할 수 없습니다. 환경 변수와 설정 스크립트 모두 환경 구성에 저장되며, 해당 환경을 편집할 수 있는 모든 사람이 볼 수 있습니다. 클라우드 세션에서 비밀이 필요하면 이러한 가시성을 염두에 두고 환경 변수로 추가하세요.

<h3 id="installed-tools">
  설치된 도구
</h3>

클라우드 세션에는 일반적인 언어 런타임, 빌드 도구 및 데이터베이스가 사전 설치되어 있습니다. 아래 표는 카테고리별로 포함된 것을 요약합니다.

| 카테고리        | 포함됨                                                                          |
| :---------- | :--------------------------------------------------------------------------- |
| **Python**  | pip, poetry, uv, black, mypy, pytest, ruff가 포함된 Python 3.x                   |
| **Node.js** | nvm을 통한 20, 21, 22, npm, yarn, pnpm, bun¹, eslint, prettier, chromedriver 포함 |
| **Ruby**    | gem, bundler, rbenv가 포함된 3.1, 3.2, 3.3                                       |
| **PHP**     | Composer가 포함된 8.4                                                            |
| **Java**    | Maven 및 Gradle이 포함된 OpenJDK 21                                               |
| **Go**      | 모듈 지원이 포함된 최신 안정 버전                                                          |
| **Rust**    | rustc 및 cargo                                                                |
| **C/C++**   | GCC, Clang, cmake, ninja, conan                                              |
| **Docker**  | docker, dockerd, docker compose                                              |
| **데이터베이스**  | PostgreSQL 16, Redis 7.0                                                     |
| **유틸리티**    | git, jq, yq, ripgrep, tmux, vim, nano                                        |

¹ Bun이 설치되어 있지만 패키지 가져오기에 대해 알려진 [프록시 호환성 문제](#install-dependencies-with-a-sessionstart-hook)가 있습니다.

정확한 버전은 클라우드 세션에서 Claude에 `check-tools`를 실행하도록 요청하세요. 이 명령은 클라우드 세션에만 존재합니다.

<h3 id="work-with-github-issues-and-pull-requests">
  GitHub 이슈 및 pull request 작업
</h3>

클라우드 세션에는 Claude가 이슈를 읽고, pull request를 나열하고, diff를 가져오고, 설정 없이 주석을 게시할 수 있는 기본 제공 GitHub 도구가 포함되어 있습니다. 이러한 도구는 [GitHub 프록시](#github-proxy)를 통해 인증되며, [GitHub 인증 옵션](#github-authentication-options)에서 구성한 방법을 사용하므로 토큰이 컨테이너에 들어가지 않습니다.

[환경 설정](#configure-your-environment)에서 `GH_TOKEN` 또는 `GITHUB_TOKEN`을 직접 설정하거나, 둘 다 설정하지 않고 [GitHub 프록시](#github-proxy)가 자동으로 인증하도록 할 수 있습니다:

* 토큰을 설정하면 컨테이너에 변경되지 않은 상태로 전달되므로 `gh`와 스크립트가 직접 사용합니다.
* 둘 다 설정하지 않으면 컨테이너는 두 변수를 자리 표시자 문자열 `proxy-injected`로 설정하고 프록시는 아웃바운드 GitHub 요청에서 실제 자격 증명을 대체합니다. `gh`는 자신의 토큰 없이 작동하지만, `GITHUB_TOKEN`을 직접 읽는 스크립트는 자리 표시자를 받으며, 사용 가능한 토큰을 받지 않습니다.

세션에 어느 경우가 적용되는지 확인하려면 Claude에 `echo $GH_TOKEN`을 실행하도록 요청하세요.

`gh` CLI는 사전 설치되지 않습니다. 기본 제공 도구가 다루지 않는 `gh release` 또는 `gh workflow run`과 같은 `gh` 명령이 필요하면 직접 설치하고 인증하세요:

<Steps>
  <Step title="설정 스크립트에 gh 설치">
    [설정 스크립트](#setup-scripts)에 `apt update && apt install -y gh`를 추가하세요.
  </Step>

  <Step title="프록시가 인증을 처리하지 않는 경우 토큰 제공">
    `echo $GH_TOKEN`이 `proxy-injected`를 출력하면 [GitHub 프록시](#github-proxy)가 `gh`를 인증하므로 이 단계는 불필요합니다. 그렇지 않으면 GitHub 개인 액세스 토큰으로 [환경 설정](#configure-your-environment)에 `GH_TOKEN` 환경 변수를 추가하세요. `gh`는 `GH_TOKEN`을 자동으로 읽으므로 `gh auth login` 단계가 필요하지 않습니다.
  </Step>
</Steps>

<h3 id="link-output-back-to-the-session">
  세션으로 출력 다시 연결
</h3>

각 클라우드 세션에는 claude.ai의 트랜스크립트 URL이 있으며, 세션은 `CLAUDE_CODE_REMOTE_SESSION_ID` 환경 변수에서 자신의 ID를 읽을 수 있습니다. 이를 사용하여 PR 본문, 커밋 메시지, Slack 게시물 또는 생성된 보고서에 추적 가능한 링크를 넣어서 검토자가 이를 생성한 실행을 열 수 있습니다.

v2.1.179부터 Claude가 웹 세션에서 생성하는 커밋에는 `Claude-Session: <url>` git 트레일러가 포함되며, PR 본문에는 세션 URL이 자체 줄에 포함됩니다. v2.1.182부터 [`attribution.sessionUrl`](/docs/ko/settings#attribution-settings)을 `false`로 설정하여 트레일러와 PR 본문 링크를 생략할 수 있습니다.

커밋이나 PR이 아닌 다른 것(예: Claude가 게시하는 Slack 메시지 또는 작성하는 보고서 파일)에 세션 링크를 포함하려면 Claude에 다음 명령을 실행하도록 하고 출력을 사용하세요. 이 명령은 환경 변수의 값에서 `cse_` 접두사를 트랜스크립트 URL이 예상하는 `session_` 접두사로 변환합니다:

```bash theme={null}
echo "https://claude.ai/code/${CLAUDE_CODE_REMOTE_SESSION_ID/#cse_/session_}"
```

<h3 id="run-tests-start-services-and-add-packages">
  테스트 실행, 서비스 시작 및 패키지 추가
</h3>

Claude는 작업을 수행하는 과정에서 테스트를 실행합니다. 프롬프트에서 요청하세요. 예: "tests/의 실패한 테스트 수정" 또는 "각 변경 후 pytest 실행". pytest, jest, cargo test와 같은 테스트 러너는 사전 설치되어 있으므로 추가 설정 없이 작동합니다.

PostgreSQL 및 Redis는 사전 설치되어 있지만 기본적으로 실행되지 않습니다. 세션 중에 Claude에 각각을 시작하도록 요청하세요:

```bash theme={null}
service postgresql start
```

```bash theme={null}
service redis-server start
```

Docker는 컨테이너화된 서비스를 실행하는 데 사용 가능합니다. Claude에 `docker compose up`을 실행하여 프로젝트의 서비스를 시작하도록 요청하세요. 이미지를 가져오기 위한 네트워크 액세스는 환경의 [액세스 수준](#access-levels)을 따르며, [신뢰할 수 있는 기본값](#default-allowed-domains)에는 Docker Hub 및 기타 일반적인 레지스트리가 포함됩니다.

이미지가 크거나 가져오기가 느린 경우 [설정 스크립트](#setup-scripts)에 `docker compose pull` 또는 `docker compose build`를 추가하세요. 가져온 이미지는 [캐시된 환경](#environment-caching)에 저장되므로 각 새 세션에는 디스크에 이미지가 있습니다. 캐시는 파일만 저장하고 실행 중인 프로세스는 저장하지 않으므로 Claude는 여전히 각 세션에서 컨테이너를 시작합니다.

사전 설치되지 않은 패키지를 추가하려면 [설정 스크립트](#setup-scripts)를 사용하세요. 스크립트의 출력은 [캐시됨](#environment-caching)이므로 여기에 설치한 패키지는 매번 다시 설치하지 않고도 모든 세션의 시작 시 사용 가능합니다. 세션 중에 Claude에 패키지를 설치하도록 요청할 수도 있지만, 이러한 설치는 다른 세션으로 이월되지 않습니다.

<h3 id="resource-limits">
  리소스 제한
</h3>

클라우드 세션은 시간이 지남에 따라 변할 수 있는 대략적인 리소스 상한선으로 실행됩니다:

* 4 vCPU
* 16 GB RAM
* 30 GB 디스크

대규모 빌드 작업이나 메모리 집약적 테스트와 같이 훨씬 더 많은 메모리가 필요한 작업은 실패하거나 종료될 수 있습니다. 이러한 제한을 초과하는 워크로드의 경우 [Remote Control](/docs/ko/remote-control)을 사용하여 자신의 하드웨어에서 Claude Code를 실행하세요.

<h3 id="configure-your-environment">
  환경 구성
</h3>

환경은 [네트워크 액세스](#network-access), 환경 변수 및 세션이 시작되기 전에 실행되는 [설정 스크립트](#setup-scripts)를 제어합니다. 구성 없이 사용 가능한 것은 [설치된 도구](#installed-tools)를 참조하세요. 웹 인터페이스 또는 터미널에서 환경을 관리할 수 있습니다:

| 작업                    | 방법                                                                                                                  |
| :-------------------- | :------------------------------------------------------------------------------------------------------------------ |
| 환경 추가                 | 현재 환경을 선택하여 선택기를 열고 **환경 추가**를 선택합니다. 대화 상자에는 이름, 네트워크 액세스 수준, 환경 변수 및 설정 스크립트가 포함됩니다.                              |
| 환경 편집                 | 클라우드 아이콘을 선택하여 현재 환경의 이름을 표시하고 선택기를 열고, 환경 위에 마우스를 올리고, 오른쪽에 나타나는 설정 아이콘을 클릭합니다.                                    |
| 환경 보관                 | 환경을 편집하기 위해 열고 **보관**을 선택합니다. 보관된 환경은 선택기에서 숨겨지지만 기존 세션은 계속 실행됩니다.                                                  |
| CLI 클라우드 세션의 기본 환경 설정 | 터미널에서 `/remote-env`를 실행합니다. 단일 환경이 있으면 이 명령은 현재 구성을 표시합니다. `/remote-env`는 기본값만 선택합니다. 웹 인터페이스에서 환경을 추가, 편집 및 보관합니다. |

환경 변수는 `.env` 형식을 사용하며 한 줄에 하나의 `KEY=value` 쌍입니다. 따옴표는 값의 일부로 저장되므로 값을 따옴표로 감싸지 마세요. 이 예제는 세 개의 변수를 정의합니다:

```text theme={null}
NODE_ENV=development
LOG_LEVEL=debug
DATABASE_URL=postgres://localhost:5432/myapp
```

<h3 id="organization-shared-environments">
  조직 공유 환경
</h3>

Team 및 Enterprise 플랜의 소유자와 관리자는 조직의 모든 구성원과 공유되는 클라우드 환경을 만들 수 있습니다. 공유 환경은 각 구성원의 환경 선택기에 개인 환경과 함께 나타나므로 팀이 각 구성원이 다시 만드는 대신 하나의 구성으로 표준화할 수 있습니다.

[관리 설정](https://claude.ai/admin-settings)의 **클라우드 환경** 페이지에서 공유 환경을 관리합니다. 여기에서 다음을 수행할 수 있습니다:

* 공유 환경을 만들고, 편집하고, 보관합니다. 각각은 개인 환경과 동일한 필드를 가집니다: 이름, [네트워크 액세스 수준](#access-levels), `.env` 형식의 [환경 변수](#configure-your-environment) 및 [설정 스크립트](#setup-scripts).
* 조직의 기본 환경을 설정합니다.

공유 환경의 값은 해당 환경의 모든 구성원의 세션에 도달합니다. 개인 환경과 마찬가지로 공유 환경에는 전용 비밀 저장소가 없으므로 비밀을 포함하지 마세요.

<h2 id="setup-scripts">
  설정 스크립트
</h2>

설정 스크립트는 새 클라우드 세션이 시작될 때, Claude Code가 시작되기 전에 실행되는 Bash 스크립트입니다. 설정 스크립트를 사용하여 종속성을 설치하고, 도구를 구성하거나, 세션이 필요하지만 사전 설치되지 않은 것을 가져오세요.

스크립트는 Ubuntu 24.04에서 root로 실행되므로 `apt install` 및 대부분의 언어 패키지 관리자가 작동합니다.

설정 스크립트를 추가하려면 환경 설정 대화 상자를 열고 **Setup script** 필드에 스크립트를 입력하세요.

이 예제는 사전 설치되지 않은 `gh` CLI를 설치합니다:

```bash theme={null}
#!/bin/bash
apt update && apt install -y gh
```

스크립트가 0이 아닌 값으로 종료되면 세션이 시작되지 않습니다. 간헐적인 설치 실패로 세션을 차단하지 않으려면 중요하지 않은 명령에 `|| true`를 추가하세요.

스크립트의 총 런타임을 대략 5분 이내로 유지하여 [환경 캐시](#environment-caching)를 구성할 수 있습니다. `&` 및 `wait`를 사용하여 독립적인 설치를 병렬로 실행하세요. 단일 다운로드가 5분 제한에 맞지 않으면 [SessionStart hook](#setup-scripts-vs-sessionstart-hooks)으로 이동하여 백그라운드에서 시작하세요.

<Note>
  패키지를 설치하는 설정 스크립트는 레지스트리에 도달하기 위해 네트워크 액세스가 필요합니다. 기본 **Trusted** 네트워크 액세스는 npm, PyPI, RubyGems 및 crates.io를 포함한 [일반적인 패키지 레지스트리](#default-allowed-domains)에 대한 연결을 허용합니다. 환경이 **None** 네트워크 액세스를 사용하면 스크립트가 패키지 설치에 실패합니다.
</Note>

<h3 id="environment-caching">
  환경 캐싱
</h3>

설정 스크립트는 환경에서 새 세션을 시작할 때 처음 실행됩니다. 완료되면 Anthropic이 파일 시스템을 스냅샷하고 해당 스냅샷을 나중 세션의 시작점으로 재사용합니다. 새 세션은 이미 디스크에 있는 종속성, 도구 및 Docker 이미지로 시작되며, 설정 스크립트 단계는 건너뜁니다. 이렇게 하면 스크립트가 큰 도구 체인을 설치하거나 컨테이너 이미지를 가져올 때도 시작이 빠릅니다.

캐시는 파일을 캡처하고 실행 중인 프로세스는 캡처하지 않습니다. 설정 스크립트가 디스크에 쓰는 모든 것이 이월됩니다. 시작하는 서비스 또는 컨테이너는 그렇지 않으므로 Claude에 요청하거나 [SessionStart hook](#setup-scripts-vs-sessionstart-hooks)으로 세션당 이를 시작하세요.

환경의 설정 스크립트 또는 허용된 네트워크 호스트를 변경할 때, 그리고 캐시가 대략 7일 후 만료에 도달할 때 캐시를 다시 빌드하기 위해 설정 스크립트가 다시 실행됩니다. 기존 세션을 재개하면 설정 스크립트가 다시 실행되지 않습니다.

캐싱을 활성화하거나 스냅샷을 직접 관리할 필요가 없습니다.

<h3 id="setup-scripts-vs-sessionstart-hooks">
  설정 스크립트 vs. SessionStart hooks
</h3>

클라우드가 필요하지만 노트북에 이미 있는 것(예: 언어 런타임 또는 CLI 도구)을 설치하려면 설정 스크립트를 사용합니다. 클라우드 및 로컬 모두에서 실행되어야 하는 프로젝트 설정(예: `npm install`)의 경우 [SessionStart hook](/docs/ko/hooks#sessionstart)을 사용합니다.

둘 다 세션 시작 시 실행되지만 다른 위치에 속합니다:

|       | 설정 스크립트                                                 | SessionStart hooks                    |
| ----- | ------------------------------------------------------- | ------------------------------------- |
| 첨부 대상 | 클라우드 환경                                                 | 저장소                                   |
| 구성 위치 | 클라우드 환경 UI                                              | 저장소의 `.claude/settings.json`          |
| 실행    | Claude Code 시작 전, [캐시된 환경](#environment-caching)이 없을 때만 | Claude Code 시작 후, 재개된 세션을 포함한 모든 세션에서 |
| 범위    | 클라우드 환경만                                                | 로컬 및 클라우드 모두                          |

SessionStart hooks는 로컬의 사용자 수준 `~/.claude/settings.json`에서도 정의할 수 있지만 사용자 수준 설정은 클라우드 세션으로 이월되지 않습니다. 클라우드에서는 저장소에서 그리고 조직의 [서버 관리 설정](/docs/ko/server-managed-settings)에서 hooks가 제공됩니다.

<h3 id="install-dependencies-with-a-sessionstart-hook">
  SessionStart hook로 종속성 설치
</h3>

클라우드 세션에서만 종속성을 설치하려면 저장소의 `.claude/settings.json`에 SessionStart hook을 추가하세요:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "startup|resume",
        "hooks": [
          {
            "type": "command",
            "command": "\"$CLAUDE_PROJECT_DIR\"/scripts/install_pkgs.sh"
          }
        ]
      }
    ]
  }
}
```

`scripts/install_pkgs.sh`에 스크립트를 생성하고 `chmod +x`로 실행 가능하게 만드세요. `CLAUDE_CODE_REMOTE` 환경 변수는 클라우드 세션에서 `true`로 설정되므로 이를 사용하여 로컬 실행을 건너뛸 수 있습니다:

```bash theme={null}
#!/bin/bash

if [ "$CLAUDE_CODE_REMOTE" != "true" ]; then
  exit 0
fi

npm install
pip install -r requirements.txt
exit 0
```

SessionStart hooks는 클라우드 세션에서 몇 가지 제한 사항이 있습니다:

* **클라우드 전용 범위 없음**: hooks는 로컬 및 클라우드 세션 모두에서 실행됩니다. 로컬 실행을 건너뛰려면 위에 표시된 대로 `CLAUDE_CODE_REMOTE` 환경 변수를 확인하세요.
* **네트워크 액세스 필요**: 설치 명령은 패키지 레지스트리에 도달해야 합니다. 환경이 **None** 네트워크 액세스를 사용하면 이러한 hooks가 실패합니다. **Trusted** 아래의 [기본 허용 목록](#default-allowed-domains)은 npm, PyPI, RubyGems 및 crates.io를 포함합니다.
* **프록시 호환성**: 모든 아웃바운드 트래픽은 [보안 프록시](#security-proxy)를 통과합니다. 일부 패키지 관리자는 이 프록시에서 제대로 작동하지 않습니다. Bun은 알려진 예입니다.
* **시작 지연 추가**: hooks는 세션이 시작되거나 재개될 때마다 실행됩니다. 설정 스크립트와 달리 [환경 캐싱](#environment-caching)의 이점을 얻지 못합니다. 재설치하기 전에 종속성이 이미 있는지 확인하여 설치 스크립트를 빠르게 유지하세요.

후속 Bash 명령에 대한 환경 변수를 유지하려면 `$CLAUDE_ENV_FILE`의 파일에 쓰세요. 자세한 내용은 [SessionStart hooks](/docs/ko/hooks#sessionstart)를 참조하세요.

기본 이미지를 자신의 Docker 이미지로 바꾸는 것은 아직 지원되지 않습니다. [제공된 이미지](#installed-tools) 위에 필요한 것을 설치하려면 설정 스크립트를 사용하거나, `docker compose`로 Claude와 함께 이미지를 컨테이너로 실행하세요.

<h2 id="network-access">
  네트워크 액세스
</h2>

네트워크 액세스는 클라우드 환경에서 아웃바운드 연결을 제어합니다. 각 환경은 하나의 액세스 수준을 지정하며, 사용자 정의 허용 도메인으로 확장할 수 있습니다. 기본값은 **Trusted**이며, 패키지 레지스트리 및 기타 [허용 목록 도메인](#default-allowed-domains)을 허용합니다.

환경의 네트워크 액세스를 변경하려면 [편집을 위해 열고](#configure-your-environment) 대화 상자에서 **Network access** 선택기를 사용하세요. 별도의 Environments 페이지가 없습니다. 클라우드 세션을 시작하거나 [routine](/docs/ko/routines#environments-and-network-access)을 구성할 때마다 클라우드 아이콘이 나타납니다.

<Note>
  MCP 커넥터 트래픽은 Anthropic의 서버를 통해 라우팅되므로 세션 또는 routine에서 활성화한 커넥터는 **Allowed domains**에 해당 호스트를 추가하지 않고도 작동합니다. 커넥터는 세션별 또는 routine별로 구성됩니다. 필요하지 않은 것을 제거하여 Claude가 도달할 수 있는 도구를 제한하세요. 이는 [보안 및 격리](#security-and-isolation)에서 언급한 동일한 Anthropic 바운드 채널에 의존합니다.
</Note>

<h3 id="access-levels">
  액세스 수준
</h3>

환경을 생성하거나 편집할 때 액세스 수준을 선택하세요:

| 수준          | 아웃바운드 연결                                                            |
| :---------- | :------------------------------------------------------------------ |
| **None**    | 아웃바운드 네트워크 액세스 없음                                                   |
| **Trusted** | [허용 목록 도메인](#default-allowed-domains)만: 패키지 레지스트리, GitHub, 클라우드 SDK |
| **Full**    | 모든 도메인                                                              |
| **Custom**  | 기본값을 선택적으로 포함하는 자신의 허용 목록                                           |

GitHub 작업은 이 설정과 독립적인 [별도 프록시](#github-proxy)를 사용합니다.

<h3 id="allow-specific-domains">
  특정 도메인 허용
</h3>

Trusted 목록에 없는 도메인을 허용하려면 환경의 네트워크 액세스 설정에서 **Custom**을 선택하세요. **Allowed domains** 필드가 나타납니다. 한 줄에 하나의 도메인을 입력하세요:

```text theme={null}
api.example.com
*.internal.example.com
registry.example.com
```

와일드카드 하위 도메인 일치를 위해 `*.`를 사용하세요. **Also include default list of common package managers**를 확인하여 [Trusted 도메인](#default-allowed-domains)을 사용자 정의 항목과 함께 유지하거나, 나열한 것만 허용하려면 선택 해제하세요.

허용 도메인은 환경별로 구성됩니다. 소유자가 모든 사용자의 환경에 푸시할 수 있는 조직 수준의 허용 목록은 없습니다. [서버 관리 설정](/docs/ko/server-managed-settings)은 클라우드 세션을 제한할 수 있지만 허용 도메인을 추가할 수 없습니다.

<h3 id="github-proxy">
  GitHub 프록시
</h3>

보안을 위해 모든 GitHub 작업은 실제 GitHub 자격 증명을 샌드박스 외부에 유지하는 전용 프록시 서비스를 통해 진행됩니다. 프록시는 두 가지 유형의 트래픽을 인증합니다:

* Git 상호 작용: 샌드박스 내의 git 클라이언트는 사용자 정의 빌드 범위 자격 증명을 사용하며, 프록시가 이를 확인하고 실제 GitHub 인증 토큰으로 변환합니다
* GitHub API 요청: 프록시는 기본 제공 GitHub 도구의 요청과 [GitHub 이슈 및 풀 요청 작업](#work-with-github-issues-and-pull-requests)에서 설명한 `proxy-injected` 플레이스홀더를 설정하는 세션의 `gh`에서 실제 자격 증명을 대체합니다

프록시는 또한 안전을 위해 git push 작업을 현재 작업 분기로 제한하며, 보안 경계를 유지하면서 복제, 가져오기 및 PR 작업을 활성화합니다.

프록시는 환경의 [네트워크 액세스 수준](#access-levels)과 관계없이 GitHub API 및 릴리스 자산 요청을 세션에 연결된 저장소로 제한합니다. 연결되지 않은 저장소에서 릴리스 자산을 다운로드하는 설정 스크립트는 403을 반환합니다. 공개 저장소의 커밋된 파일은 `raw.githubusercontent.com`을 통해 가져오며, [보안 프록시](#security-proxy)가 처리합니다. 해당 도메인은 기본 [Trusted 목록](#default-allowed-domains)에 있으므로 환경의 [액세스 수준](#access-levels)이 이를 제외하지 않는 한 파일은 계속 도달 가능합니다.

<h3 id="security-proxy">
  보안 프록시
</h3>

환경은 보안 및 남용 방지를 위해 HTTP/HTTPS 네트워크 프록시 뒤에서 실행됩니다. 모든 아웃바운드 인터넷 트래픽은 다음을 제공하는 이 프록시를 통과합니다:

* 악의적인 요청으로부터의 보호
* 속도 제한 및 남용 방지
* 향상된 보안을 위한 콘텐츠 필터링
* 요청된 호스트명의 DNS 수준 감사 추적

<h3 id="default-allowed-domains">
  기본 허용 도메인
</h3>

**Trusted** 네트워크 액세스를 사용할 때 다음 도메인이 기본적으로 허용됩니다. `*`로 표시된 도메인은 와일드카드 하위 도메인 일치를 나타내므로 `*.gcr.io`는 `gcr.io`의 모든 하위 도메인을 허용합니다.

<AccordionGroup>
  <Accordion title="Anthropic 서비스">
    * api.anthropic.com
    * statsig.anthropic.com
    * docs.claude.com
    * platform.claude.com
    * code.claude.com
    * claude.ai
  </Accordion>

  <Accordion title="버전 제어">
    * github.com
    * [www.github.com](http://www.github.com)
    * api.github.com
    * npm.pkg.github.com
    * raw\.githubusercontent.com
    * pkg-npm.githubusercontent.com
    * objects.githubusercontent.com
    * release-assets.githubusercontent.com
    * codeload.github.com
    * avatars.githubusercontent.com
    * camo.githubusercontent.com
    * gist.github.com
    * gitlab.com
    * [www.gitlab.com](http://www.gitlab.com)
    * registry.gitlab.com
    * bitbucket.org
    * [www.bitbucket.org](http://www.bitbucket.org)
    * api.bitbucket.org
  </Accordion>

  <Accordion title="컨테이너 레지스트리">
    * registry-1.docker.io
    * auth.docker.io
    * index.docker.io
    * hub.docker.com
    * [www.docker.com](http://www.docker.com)
    * production.cloudflare.docker.com
    * download.docker.com
    * gcr.io
    * \*.gcr.io
    * ghcr.io
    * mcr.microsoft.com
    * \*.data.mcr.microsoft.com
    * public.ecr.aws
  </Accordion>

  <Accordion title="클라우드 플랫폼">
    * cloud.google.com
    * accounts.google.com
    * gcloud.google.com
    * \*.googleapis.com
    * storage.googleapis.com
    * compute.googleapis.com
    * container.googleapis.com
    * azure.com
    * portal.azure.com
    * microsoft.com
    * [www.microsoft.com](http://www.microsoft.com)
    * \*.microsoftonline.com
    * packages.microsoft.com
    * dotnet.microsoft.com
    * dot.net
    * visualstudio.com
    * dev.azure.com
    * \*.amazonaws.com
    * \*.api.aws
    * oracle.com
    * [www.oracle.com](http://www.oracle.com)
    * java.com
    * [www.java.com](http://www.java.com)
    * java.net
    * [www.java.net](http://www.java.net)
    * download.oracle.com
    * yum.oracle.com
  </Accordion>

  <Accordion title="JavaScript 및 Node 패키지 관리자">
    * registry.npmjs.org
    * [www.npmjs.com](http://www.npmjs.com)
    * [www.npmjs.org](http://www.npmjs.org)
    * npmjs.com
    * npmjs.org
    * yarnpkg.com
    * registry.yarnpkg.com
  </Accordion>

  <Accordion title="Python 패키지 관리자">
    * pypi.org
    * [www.pypi.org](http://www.pypi.org)
    * files.pythonhosted.org
    * pythonhosted.org
    * test.pypi.org
    * pypi.python.org
    * pypa.io
    * [www.pypa.io](http://www.pypa.io)
  </Accordion>

  <Accordion title="Ruby 패키지 관리자">
    * rubygems.org
    * [www.rubygems.org](http://www.rubygems.org)
    * api.rubygems.org
    * index.rubygems.org
    * ruby-lang.org
    * [www.ruby-lang.org](http://www.ruby-lang.org)
    * rubyforge.org
    * [www.rubyforge.org](http://www.rubyforge.org)
    * rubyonrails.org
    * [www.rubyonrails.org](http://www.rubyonrails.org)
    * rvm.io
    * get.rvm.io
  </Accordion>

  <Accordion title="Rust 패키지 관리자">
    * crates.io
    * [www.crates.io](http://www.crates.io)
    * index.crates.io
    * static.crates.io
    * rustup.rs
    * static.rust-lang.org
    * [www.rust-lang.org](http://www.rust-lang.org)
  </Accordion>

  <Accordion title="Go 패키지 관리자">
    * proxy.golang.org
    * sum.golang.org
    * index.golang.org
    * golang.org
    * [www.golang.org](http://www.golang.org)
    * goproxy.io
    * pkg.go.dev
  </Accordion>

  <Accordion title="JVM 패키지 관리자">
    * maven.org
    * repo.maven.org
    * central.maven.org
    * repo1.maven.org
    * repo.maven.apache.org
    * jcenter.bintray.com
    * gradle.org
    * [www.gradle.org](http://www.gradle.org)
    * services.gradle.org
    * plugins.gradle.org
    * kotlinlang.org
    * [www.kotlinlang.org](http://www.kotlinlang.org)
    * spring.io
    * repo.spring.io
  </Accordion>

  <Accordion title="기타 패키지 관리자">
    * packagist.org (PHP Composer)
    * [www.packagist.org](http://www.packagist.org)
    * repo.packagist.org
    * nuget.org (.NET NuGet)
    * [www.nuget.org](http://www.nuget.org)
    * api.nuget.org
    * pub.dev (Dart/Flutter)
    * api.pub.dev
    * hex.pm (Elixir/Erlang)
    * [www.hex.pm](http://www.hex.pm)
    * cpan.org (Perl CPAN)
    * [www.cpan.org](http://www.cpan.org)
    * metacpan.org
    * [www.metacpan.org](http://www.metacpan.org)
    * api.metacpan.org
    * cocoapods.org (iOS/macOS)
    * [www.cocoapods.org](http://www.cocoapods.org)
    * cdn.cocoapods.org
    * haskell.org
    * [www.haskell.org](http://www.haskell.org)
    * hackage.haskell.org
    * swift.org
    * [www.swift.org](http://www.swift.org)
  </Accordion>

  <Accordion title="Linux 배포판">
    * archive.ubuntu.com
    * security.ubuntu.com
    * ubuntu.com
    * [www.ubuntu.com](http://www.ubuntu.com)
    * \*.ubuntu.com
    * ppa.launchpad.net
    * launchpad.net
    * [www.launchpad.net](http://www.launchpad.net)
    * \*.nixos.org
  </Accordion>

  <Accordion title="개발 도구 및 플랫폼">
    * dl.k8s.io (Kubernetes)
    * pkgs.k8s.io
    * k8s.io
    * [www.k8s.io](http://www.k8s.io)
    * releases.hashicorp.com (HashiCorp)
    * apt.releases.hashicorp.com
    * rpm.releases.hashicorp.com
    * archive.releases.hashicorp.com
    * hashicorp.com
    * [www.hashicorp.com](http://www.hashicorp.com)
    * repo.anaconda.com (Anaconda/Conda)
    * conda.anaconda.org
    * anaconda.org
    * [www.anaconda.com](http://www.anaconda.com)
    * anaconda.com
    * continuum.io
    * apache.org (Apache)
    * [www.apache.org](http://www.apache.org)
    * archive.apache.org
    * downloads.apache.org
    * eclipse.org (Eclipse)
    * [www.eclipse.org](http://www.eclipse.org)
    * download.eclipse.org
    * nodejs.org (Node.js)
    * [www.nodejs.org](http://www.nodejs.org)
    * developer.apple.com
    * developer.android.com
    * pkg.stainless.com
    * binaries.prisma.sh
  </Accordion>

  <Accordion title="클라우드 서비스 및 모니터링">
    * statsig.com
    * [www.statsig.com](http://www.statsig.com)
    * api.statsig.com
    * sentry.io
    * \*.sentry.io
    * downloads.sentry-cdn.com
    * http-intake.logs.datadoghq.com
    * browser-intake-us5-datadoghq.com
    * \*.datadoghq.com
    * \*.datadoghq.eu
    * api.honeycomb.io
  </Accordion>

  <Accordion title="콘텐츠 전달 및 미러">
    * sourceforge.net
    * \*.sourceforge.net
    * packagecloud.io
    * \*.packagecloud.io
    * fonts.googleapis.com
    * fonts.gstatic.com
  </Accordion>

  <Accordion title="스키마 및 구성">
    * json-schema.org
    * [www.json-schema.org](http://www.json-schema.org)
    * json.schemastore.org
    * [www.schemastore.org](http://www.schemastore.org)
  </Accordion>

  <Accordion title="Model Context Protocol">
    * \*.modelcontextprotocol.io
  </Accordion>
</AccordionGroup>

<h2 id="move-tasks-between-web-and-terminal">
  웹과 터미널 간에 작업 이동
</h2>

이러한 워크플로우는 동일한 claude.ai 계정에 로그인한 [Claude Code CLI](/docs/ko/quickstart)가 필요합니다. 터미널에서 새 클라우드 세션을 시작하거나 클라우드 세션을 터미널로 가져와 로컬에서 계속할 수 있습니다. 클라우드 세션은 노트북을 닫아도 유지되며, Claude 모바일 앱을 포함한 어디서나 모니터링할 수 있습니다.

<Note>
  CLI에서 세션 핸드오프는 일방향입니다: `--teleport`로 클라우드 세션을 터미널로 가져올 수 있지만 기존 터미널 세션을 웹으로 푸시할 수 없습니다. `--cloud` 플래그는 현재 저장소에 대한 새로운 클라우드 세션을 생성합니다. [Desktop 앱](/docs/ko/desktop#continue-in-another-surface)은 로컬 세션을 웹으로 보낼 수 있는 Continue in 메뉴를 제공합니다.
</Note>

<h3 id="from-terminal-to-web">
  터미널에서 웹으로
</h3>

`--cloud` 플래그로 명령줄에서 클라우드 세션을 시작하세요:

```bash theme={null}
claude --cloud "Fix the authentication bug in src/auth/login.ts"
```

이렇게 하면 claude.ai에서 새 클라우드 세션이 생성됩니다. 세션은 현재 디렉토리의 GitHub 원격을 현재 분기에서 복제하므로, VM이 머신이 아닌 GitHub에서 복제하기 때문에 로컬 커밋이 있으면 먼저 푸시하세요. `--cloud`는 한 번에 하나의 저장소에서 작동합니다. 작업은 클라우드에서 실행되는 동안 로컬에서 계속 작업할 수 있습니다. 더 이상 사용되지 않는 `--remote` 표기법은 여전히 `--cloud`의 더 이상 사용되지 않는 별칭으로 작동합니다.

v2.1.195부터 CLI는 저장소 복제 및 [설정 스크립트](#setup-scripts) 실행과 같은 설정 단계의 라이브 체크리스트를 표시하며, 클라우드 컨테이너가 시작됩니다. 컨테이너가 프로비저닝되는 동안 입력한 메시지는 큐에 저장되었다가 세션이 준비되면 전송됩니다.

<Note>
  `--cloud`는 클라우드 세션을 생성합니다. `--remote-control`은 관련이 없습니다: 로컬 CLI 세션을 노출하여 웹에서 모니터링할 수 있습니다. [Remote Control](/docs/ko/remote-control)을 참조하세요.
</Note>

Claude Code CLI에서 `/tasks`를 사용하여 진행 상황을 확인하거나 claude.ai 또는 Claude 모바일 앱에서 세션을 열어 직접 상호 작용하세요. 여기서 Claude를 조종하고, 피드백을 제공하거나, 다른 대화처럼 질문에 답변할 수 있습니다.

<h4 id="tips-for-cloud-tasks">
  클라우드 작업 팁
</h4>

**로컬에서 계획하고 원격으로 실행**: 복잡한 작업의 경우 Claude를 Plan Mode에서 시작하여 접근 방식을 협력한 다음 작업을 클라우드로 보내세요:

```bash theme={null}
claude --permission-mode plan
```

Plan Mode에서 Claude는 파일을 읽고, 명령을 실행하여 탐색하고, 소스 코드를 편집하지 않고 계획을 제안합니다. 계획에 만족하면 저장소에 저장하고, 커밋하고, 푸시하여 클라우드 VM이 복제할 수 있도록 한 다음 자율 실행을 위해 클라우드 세션을 시작하세요:

```bash theme={null}
claude --cloud "Execute the migration plan in docs/migration-plan.md"
```

이 패턴은 Claude가 클라우드에서 자율적으로 실행되도록 하면서 전략에 대한 제어를 제공합니다.

**ultraplan으로 클라우드에서 계획**: 웹 세션 자체에서 계획을 작성하고 검토하려면 [ultraplan](/docs/ko/ultraplan)을 사용하세요. Claude는 웹에서 Claude Code on the web에서 계획을 생성하는 동안 계속 작업하고, 브라우저에서 섹션에 주석을 달고 원격으로 실행하거나 계획을 터미널로 다시 보낼 수 있습니다.

**작업을 병렬로 실행**: 각 `--cloud` 명령은 독립적으로 실행되는 자체 클라우드 세션을 생성합니다. 여러 작업을 시작할 수 있으며 모두 별도의 세션에서 동시에 실행됩니다:

```bash theme={null}
claude --cloud "Fix the flaky test in auth.spec.ts"
claude --cloud "Update the API documentation"
claude --cloud "Refactor the logger to use structured output"
```

Claude Code CLI에서 `/tasks`로 모든 세션을 모니터링하세요. 세션이 완료되면 웹 인터페이스에서 PR을 생성하거나 [세션을 텔레포트](#from-web-to-terminal)하여 터미널에서 계속 작업할 수 있습니다.

<h4 id="send-local-repositories-without-github">
  GitHub 없이 로컬 저장소 보내기
</h4>

GitHub에 연결되지 않은 저장소에서 `claude --cloud`를 실행하면 Claude Code가 로컬 저장소를 번들로 만들어 클라우드 세션에 직접 업로드합니다. 번들에는 모든 분기의 전체 저장소 기록과 추적된 파일에 대한 커밋되지 않은 변경 사항이 포함됩니다.

GitHub 액세스를 사용할 수 없을 때 이 폴백이 자동으로 활성화됩니다. GitHub가 연결되어 있어도 강제하려면 `CCR_FORCE_BUNDLE=1`을 설정하세요:

```bash theme={null}
CCR_FORCE_BUNDLE=1 claude --cloud "Run the test suite and fix any failures"
```

번들된 저장소는 이러한 제한을 충족해야 합니다:

* 디렉토리는 최소 하나의 커밋이 있는 git 저장소여야 합니다
* 번들된 저장소는 100 MB 미만이어야 합니다. 더 큰 저장소는 현재 분기만 번들로 만들기로 폴백한 다음 작업 트리의 단일 스쿼시 스냅샷으로 폴백하고, 스냅샷이 여전히 너무 크면 실패합니다
* 추적되지 않은 파일은 포함되지 않습니다. 클라우드 세션이 보기를 원하는 파일에 대해 `git add`를 실행하세요
* 번들에서 생성된 세션은 [GitHub 인증](#github-authentication-options)도 구성되어 있지 않으면 원격으로 다시 푸시할 수 없습니다

<h3 id="from-web-to-terminal">
  웹에서 터미널로
</h3>

다음 중 하나를 사용하여 클라우드 세션을 터미널로 가져오세요:

* **`--teleport` 사용**: 명령줄에서 `claude --teleport`를 실행하여 대화형 세션 선택기를 사용하거나 `claude --teleport <session-id>`를 실행하여 특정 세션을 직접 재개합니다. 커밋되지 않은 변경 사항이 있으면 먼저 stash하라는 메시지가 표시됩니다.
* **`/teleport` 사용**: 기존 CLI 세션 내에서 `/teleport`(또는 `/tp`)를 실행하여 Claude Code를 다시 시작하지 않고 동일한 세션 선택기를 엽니다.
* **`/tasks`에서**: `/tasks`를 실행하여 백그라운드 세션을 보고 `t`를 눌러 하나로 텔레포트합니다.
* **웹 인터페이스에서**: **Open in CLI**를 선택하여 터미널에 붙여넣을 수 있는 명령을 복사합니다.

세션을 텔레포트하면 Claude가 올바른 저장소에 있는지 확인하고, 클라우드 세션에서 분기를 가져와 체크아웃하고, 전체 대화 기록을 터미널에 로드합니다.

`--teleport`는 `--resume`과 다릅니다. `--resume`은 이 머신의 로컬 기록에서 대화를 다시 열고 클라우드 세션을 나열하지 않습니다. `--teleport`는 클라우드 세션과 해당 분기를 가져옵니다.

<h4 id="teleport-requirements">
  텔레포트 요구 사항
</h4>

텔레포트는 세션을 재개하기 전에 이러한 요구 사항을 확인합니다. 요구 사항이 충족되지 않으면 오류가 표시되거나 문제를 해결하라는 메시지가 표시됩니다.

| 요구 사항           | 세부 정보                                                                                                                                                                                                                                                 |
| --------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Clean git state | 작업 디렉토리에 커밋되지 않은 변경 사항이 없어야 합니다. 텔레포트가 필요한 경우 변경 사항을 stash하라는 메시지를 표시합니다.                                                                                                                                                                             |
| 올바른 저장소         | fork가 아닌 동일한 저장소의 체크아웃에서 `--teleport`를 실행해야 합니다. v2.1.199부터 Claude Code는 `git@work:owner/repo.git`과 같은 SSH 호스트 별칭이나 `insteadOf`로 다시 쓴 짧은 형식과 같이 원격을 호스트 이름으로 구문 분석할 수 없는 경우에도 체크아웃을 수락합니다. 먼저 확인 프롬프트를 표시하고, 원격의 소유자 및 저장소 이름이 세션의 저장소와 일치할 때만 표시됩니다. |
| 분기 사용 가능        | 클라우드 세션의 분기가 원격으로 푸시되어야 합니다. 텔레포트가 자동으로 가져와 체크아웃합니다.                                                                                                                                                                                                  |
| 동일한 계정          | 클라우드 세션에서 사용한 동일한 claude.ai 계정으로 인증되어야 합니다.                                                                                                                                                                                                           |

<h4 id="teleport-is-unavailable">
  `--teleport`를 사용할 수 없음
</h4>

텔레포트는 claude.ai 구독 인증이 필요합니다. API 키, Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry를 통해 인증된 경우 `/login`을 실행하여 대신 claude.ai 계정으로 로그인하세요. 이미 claude.ai를 통해 로그인했는데 `--teleport`를 여전히 사용할 수 없으면 조직이 클라우드 세션을 비활성화했을 수 있습니다.

<h2 id="work-with-sessions">
  세션 작업
</h2>

세션은 claude.ai/code의 사이드바에 나타납니다. 여기서 변경 사항을 검토하고, 팀원과 공유하고, 완료된 작업을 보관하거나, 세션을 영구적으로 삭제할 수 있습니다.

<h3 id="manage-context">
  컨텍스트 관리
</h3>

클라우드 세션은 텍스트 출력을 생성하는 [기본 제공 명령](/docs/ko/commands)을 지원합니다. `/plugin` 또는 `/resume`과 같이 터미널 인터페이스에서만 실행되는 명령은 사용할 수 없습니다. 터미널에서 선택기 또는 패널을 열어야 하는 명령은 클라우드 세션에서 다르게 작동합니다:

* **`/model`, `/effort`, `/fast`, `/color`, `/rename`**: 터미널 선택기 또는 슬라이더를 열지 않고 대신 인수로 값을 전달합니다(예: `/model sonnet`). 인수 형식은 세션의 환경에서 Claude Code v2.1.205 이상이 필요하며 각 명령의 [가용성 참고 사항](/docs/ko/commands#all-commands)을 따릅니다: `/effort`는 모델의 [launch-default effort hold](/docs/ko/model-config#adjust-effort-level)가 적용 중일 때 `Not applied`를 보고하고, `/fast`는 빠른 모드가 켜진 상태로 시작된 세션에서만 작동합니다.
* **`/config`**: 웹에서는 값을 설정하는 대신 Claude Code 설정 섹션을 열며, `key=value`를 포함한 명령 뒤의 텍스트는 무시됩니다. 클라우드 세션의 설정을 변경하려면 [환경 변수](#configure-your-environment)를 사용하거나 [설정 파일](/docs/ko/settings)을 저장소에 커밋하세요.

컨텍스트 관리 특히:

| 명령         | 클라우드 세션에서 작동 | 참고                                                                          |
| :--------- | :----------- | :-------------------------------------------------------------------------- |
| `/compact` | 예            | 대화를 요약하여 컨텍스트를 확보합니다. `/compact keep the test output`과 같은 선택적 포커스 지침을 허용합니다 |
| `/context` | 예            | 현재 컨텍스트 윈도우에 있는 것을 표시합니다                                                    |
| `/clear`   | 아니오          | 사이드바에서 새 세션을 시작하세요                                                          |

자동 압축은 컨텍스트 윈도우가 용량에 접근할 때 자동으로 실행됩니다. 더 일찍 트리거하려면 [환경 변수](#configure-your-environment)에서 [`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`](/docs/ko/env-vars)를 설정하세요. 예를 들어 `CLAUDE_AUTOCOMPACT_PCT_OVERRIDE=70`은 윈도우가 거의 가득 찰 때까지 기다리는 대신 70% 용량에서 압축합니다. 압축 계산을 위한 유효 윈도우 크기를 변경하려면 [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/ko/env-vars)를 사용하세요.

[Subagents](/docs/ko/sub-agents)는 로컬과 동일한 방식으로 작동합니다. Claude는 Task 도구로 이들을 생성하여 연구 또는 병렬 작업을 별도의 컨텍스트 윈도우로 오프로드하여 주 대화를 더 가볍게 유지할 수 있습니다. 저장소의 `.claude/agents/`에 정의된 Subagents는 자동으로 선택됩니다.

[Agent teams](/docs/ko/agent-teams)는 기본적으로 꺼져 있지만 [환경 변수](#configure-your-environment)에 `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`을 추가하여 활성화할 수 있습니다.

<h3 id="review-changes">
  변경 사항 검토
</h3>

각 세션은 추가 및 제거된 줄을 표시하는 diff 표시기를 표시합니다(예: `+42 -18`). 이를 선택하여 diff 보기를 열고, 특정 줄에 인라인 주석을 남기고, 다음 메시지로 Claude에 보내세요. 전체 안내는 PR 생성을 포함하여 [검토 및 반복](/docs/ko/web-quickstart#review-and-iterate)을 참조하세요. Claude가 PR을 모니터링하여 CI 실패 및 검토 주석에 자동으로 응답하도록 하려면 [Pull request 자동 수정](#auto-fix-pull-requests)을 참조하세요.

<h3 id="share-sessions">
  세션 공유
</h3>

세션을 공유하려면 아래 계정 유형에 따라 가시성을 전환하세요. 그 후 세션 링크를 그대로 공유합니다. 수신자는 링크를 열 때 최신 상태를 보지만 보기가 실시간으로 업데이트되지 않습니다.

<h4 id="share-from-an-enterprise-or-team-account">
  Enterprise 또는 Team 계정에서 공유
</h4>

Enterprise 및 Team 계정의 경우 두 가지 가시성 옵션은 **Private** 및 **Team**입니다. Team 가시성은 claude.ai 조직의 다른 구성원에게 세션을 표시합니다. [Claude in Slack](/docs/ko/slack) 세션은 자동으로 Team 가시성으로 공유됩니다.

저장소 액세스 확인은 기본적으로 수신자의 계정에 연결된 GitHub 계정을 기반으로 활성화됩니다. 계정의 표시 이름은 액세스 권한이 있는 모든 수신자에게 표시됩니다.

<h4 id="share-from-a-max-or-pro-account">
  Max 또는 Pro 계정에서 공유
</h4>

Max 및 Pro 계정의 경우 두 가지 가시성 옵션은 **Private** 및 **Public**입니다. Public 가시성은 claude.ai에 로그인한 모든 사용자에게 세션을 표시합니다.

공유하기 전에 민감한 내용이 있는지 세션을 확인하세요. 세션에는 개인 GitHub 저장소의 코드 및 자격 증명이 포함될 수 있습니다. 저장소 액세스 확인은 기본적으로 활성화되지 않습니다.

저장소 액세스를 요구하거나 공유 세션에서 이름을 숨기려면 Settings > Claude Code > Sharing settings로 이동하세요.

<h3 id="archive-sessions">
  세션 보관
</h3>

세션을 보관하여 세션 목록을 정리할 수 있습니다. 보관된 세션은 기본 세션 목록에서 숨겨지지만 보관된 세션을 필터링하여 볼 수 있습니다.

세션을 보관하려면 사이드바의 세션 위에 마우스를 올리고 보관 아이콘을 선택합니다.

<h3 id="delete-sessions">
  세션 삭제
</h3>

세션을 삭제하면 세션과 해당 데이터가 영구적으로 제거됩니다. 이 작업은 실행 취소할 수 없습니다. 두 가지 방법으로 세션을 삭제할 수 있습니다:

* **사이드바에서**: 보관된 세션을 필터링한 다음 삭제할 세션 위에 마우스를 올리고 삭제 아이콘을 선택합니다
* **세션 메뉴에서**: 세션을 열고 세션 제목 옆의 드롭다운을 선택한 다음 **Delete**를 선택합니다

세션이 삭제되기 전에 확인하라는 메시지가 표시됩니다.

<h2 id="auto-fix-pull-requests">
  Pull request 자동 수정
</h2>

Claude는 pull request를 감시하고 CI 실패 및 검토 주석에 자동으로 응답할 수 있습니다. Claude는 PR의 GitHub 활동을 구독하고, 검사가 실패하거나 검토자가 주석을 남기면 Claude가 조사하고 명확한 수정이 있으면 푸시합니다.

<Note>
  자동 수정을 위해서는 Claude GitHub App이 저장소에 설치되어야 합니다. 아직 설치하지 않았으면 [GitHub App 페이지](https://github.com/apps/claude)에서 설치하거나 [설정](/docs/ko/web-quickstart#connect-github-and-create-an-environment) 중에 메시지가 표시될 때 설치합니다.
</Note>

PR이 어디에서 왔는지와 어떤 기기를 사용하는지에 따라 자동 수정을 켜는 방법은 몇 가지가 있습니다:

* **웹에서 Claude Code로 생성된 PR**: CI 상태 표시줄을 열고 **Auto-fix**를 선택합니다
* **터미널에서**: PR의 분기에 있는 동안 [`/autofix-pr`](/docs/ko/commands)을 실행합니다. Claude Code가 `gh`로 열린 PR을 감지하고, 웹 세션을 생성하고, 한 단계에서 자동 수정을 켭니다
* **모바일 앱에서**: Claude에 PR을 자동 수정하도록 지시합니다. 예를 들어 "watch this PR and fix any CI failures or review comments"
* **기존 PR**: PR URL을 세션에 붙여넣고 Claude에 자동 수정하도록 지시합니다

자동 수정은 PR별 토글입니다. 모니터링을 중지하려면 웹 세션에서 CI 상태 표시줄을 열고 **Auto-fix** 토글을 해제하거나, Claude에 PR 감시를 중지하도록 지시합니다.

<h3 id="how-claude-responds-to-pr-activity">
  Claude가 PR 활동에 응답하는 방식
</h3>

자동 수정이 활성화되면 Claude는 새 검토 주석 및 CI 검사 실패를 포함한 PR의 GitHub 이벤트를 수신합니다. 각 이벤트에 대해 Claude는 조사하고 진행 방식을 결정합니다:

* **명확한 수정**: Claude가 수정에 확신하고 이전 지침과 충돌하지 않으면 Claude가 변경을 수행하고, 푸시하고, 세션에서 수행한 작업을 설명합니다
* **모호한 요청**: 검토자의 주석을 여러 방식으로 해석할 수 있거나 아키텍처적으로 중요한 사항이 포함되면 Claude가 행동하기 전에 확인합니다
* **중복 또는 조치 불필요 이벤트**: 이벤트가 중복이거나 변경이 필요 없으면 Claude가 세션에서 이를 기록하고 계속합니다

GitHub는 기본 분기가 진행되고 병합 충돌이 생성될 때 웹훅을 내보내지 않으므로 자동 수정은 충돌에 자체적으로 반응할 수 없습니다. 충돌을 해결하려면 세션을 열고 Claude에 리베이스하도록 요청합니다.

Claude는 GitHub의 검토 주석 스레드에 회신할 수 있습니다. 이러한 회신은 GitHub 계정을 사용하여 게시되므로 사용자 이름 아래에 나타나지만 각 회신은 Claude Code에서 온 것으로 표시되어 검토자가 에이전트에 의해 작성되었으며 직접 작성되지 않았음을 알 수 있습니다.

<Warning>
  저장소가 Atlantis, Terraform Cloud 또는 `issue_comment` 이벤트에서 실행되는 사용자 정의 GitHub Actions와 같은 주석 트리거 자동화를 사용하는 경우 Claude의 회신이 해당 워크플로우를 트리거할 수 있음을 알아두세요. 자동 수정을 활성화하기 전에 저장소의 자동화를 검토하고 PR 주석이 인프라를 배포하거나 권한 있는 작업을 실행할 수 있는 저장소에서는 자동 수정을 비활성화하는 것을 고려하세요.
</Warning>

<h2 id="security-and-isolation">
  보안 및 격리
</h2>

각 클라우드 세션은 여러 계층을 통해 머신과 다른 세션으로부터 분리됩니다:

* **격리된 가상 머신**: 각 세션은 격리된 Anthropic 관리 VM에서 실행됩니다
* **네트워크 액세스 제어**: 네트워크 액세스는 기본적으로 제한되며 비활성화할 수 있습니다. 네트워크 액세스가 비활성화된 상태에서 실행할 때 Claude Code는 여전히 Anthropic API와 통신할 수 있으며, 이는 VM에서 데이터가 나갈 수 있습니다.
* **자격 증명 보호**: git 자격 증명 또는 서명 키와 같은 민감한 자격 증명은 Claude Code가 있는 샌드박스 내부에 없습니다. 인증은 범위 자격 증명을 사용하는 보안 프록시를 통해 처리됩니다.
* **안전한 분석**: 코드는 PR을 생성하기 전에 격리된 VM 내에서 분석 및 수정됩니다

<h2 id="troubleshooting">
  문제 해결
</h2>

런타임 API 오류(예: `API Error: 500`, `529 Overloaded`, `429` 또는 `Prompt is too long`)가 대화에 나타나면 [오류 참조](/docs/ko/errors)를 참조하세요. 이러한 오류와 해결 방법은 CLI 및 Desktop 앱과 공유됩니다. 아래 섹션에서는 클라우드 세션에 특정한 문제를 다룹니다.

<h3 id="session-creation-failed">
  세션 생성 실패
</h3>

새 세션이 `Session creation failed`로 시작되지 않거나 프로비저닝에서 정지되면 Claude Code가 클라우드 환경을 할당할 수 없습니다.

* [status.claude.com](https://status.claude.com)에서 클라우드 세션 인시던트를 확인하세요
* 용량이 온디맨드로 프로비저닝되므로 1분 후 다시 시도하세요
* 저장소에 도달할 수 있는지 확인하세요. 연결하는 GitHub 계정은 Claude GitHub App 인증 또는 `/web-setup`을 통해 동기화된 `gh` 토큰을 통해 GitHub의 저장소에 액세스할 수 있어야 합니다. 저장소에 App을 설치할 필요는 없습니다. [GitHub 인증 옵션](#github-authentication-options)을 참조하세요.

<h3 id="remote-control-session-expired-or-access-denied">
  Remote Control 세션 만료 또는 액세스 거부
</h3>

`--teleport`는 클라우드 세션이 사용하는 동일한 Remote Control 세션 인프라를 통해 연결되므로 인증 및 세션 만료 오류는 Remote Control 용어로 표시됩니다. `Remote Control session expired` 또는 `Access denied`가 표시될 수 있습니다. 연결 토큰은 단기이며 계정으로 범위가 지정됩니다.

* 로컬에서 `/login`을 실행하여 자격 증명을 새로 고친 다음 다시 연결하세요
* 세션을 소유한 동일한 계정으로 로그인했는지 확인하세요
* `Remote Control may not be available for this organization`이 표시되면 관리자가 클라우드 세션을 조직에 대해 활성화하지 않았습니다

<h3 id="environment-expired">
  환경 만료
</h3>

클라우드 세션은 비활성 기간 후 중지되고 기본 환경이 회수됩니다. 로컬 터미널에서 이는 `Could not resume session ... its environment has expired. Creating a fresh session instead.`로 표시됩니다. 웹에서 세션은 세션 목록에서 만료된 것으로 표시됩니다.

[claude.ai/code](https://claude.ai/code)에서 세션을 다시 열어 대화 기록이 복원된 새로운 환경을 프로비저닝하세요.

<h2 id="limitations">
  제한 사항
</h2>

클라우드 세션을 워크플로우에 사용하기 전에 이러한 제약을 고려하세요:

* **속도 제한**: Claude Code on the web은 계정 내의 다른 모든 Claude 및 Claude Code 사용과 속도 제한을 공유합니다. 여러 작업을 병렬로 실행하면 비례적으로 더 많은 속도 제한을 소비합니다. 클라우드 VM에 대한 별도의 컴퓨팅 요금은 없습니다.
* **저장소 인증**: 웹에서 로컬로 세션을 이동할 때 동일한 계정으로 인증된 경우에만 가능합니다
* **플랫폼 제한**: 저장소 복제 및 pull request 생성에는 GitHub가 필요합니다. 자체 호스팅 [GitHub Enterprise Server](/docs/ko/github-enterprise-server) 인스턴스는 Team 및 Enterprise 플랜에서 지원됩니다. GitLab, Bitbucket 및 기타 비 GitHub 저장소는 [로컬 번들](#send-local-repositories-without-github)로 클라우드 세션에 보낼 수 있지만 세션이 원격으로 결과를 다시 푸시할 수 없습니다
* **조직 IP 허용 목록**: 클라우드 세션은 네트워크에서가 아닌 Anthropic 관리 인프라에서 Anthropic API를 호출합니다. 조직에 [IP 허용 목록](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting)이 활성화되어 있으면 모든 클라우드 세션이 인증 오류로 실패합니다. 동일하게 [Code Review](/docs/ko/code-review) 및 [Routines](/docs/ko/routines)에 적용됩니다. [Anthropic 지원](https://support.claude.com/)에 문의하여 조직의 IP 허용 목록에서 Anthropic 호스팅 서비스를 제외하세요.

<h2 id="related-resources">
  관련 리소스
</h2>

* [Ultraplan](/docs/ko/ultraplan): 클라우드 세션에서 계획을 작성하고 브라우저에서 검토합니다
* [Ultrareview](/docs/ko/ultrareview): 클라우드 샌드박스에서 심층 다중 에이전트 코드 검토를 실행합니다
* [Routines](/docs/ko/routines): 일정에 따라, API 호출을 통해 또는 GitHub 이벤트에 응답하여 작업을 자동화합니다
* [Hooks 구성](/docs/ko/hooks): 세션 수명 주기 이벤트에서 스크립트를 실행합니다
* [설정 참조](/docs/ko/settings): 모든 구성 옵션
* [보안](/docs/ko/security): 격리 보장 및 데이터 처리
* [데이터 사용](/docs/ko/data-usage): Anthropic이 클라우드 세션에서 보유하는 것
* [Claude Tag](https://claude.com/docs/claude-tag/overview): 동일한 클라우드 환경에서 실행되는 조직 관리형 Slack의 @Claude
