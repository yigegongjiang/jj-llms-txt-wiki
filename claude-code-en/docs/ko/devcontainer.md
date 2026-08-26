> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 개발 컨테이너

> 팀 전체에서 일관되고 격리된 환경을 위해 Claude Code를 개발 컨테이너 내에서 실행합니다.

[개발 컨테이너](https://containers.dev/)(또는 dev container)를 사용하면 팀의 모든 엔지니어가 실행할 수 있는 동일하고 격리된 환경을 정의할 수 있습니다. Claude Code가 해당 컨테이너에 설치되면 Claude가 실행하는 명령은 호스트 머신이 아닌 컨테이너 내에서 실행되며, 프로젝트 파일에 대한 편집 사항은 작업할 때 로컬 저장소에 나타납니다.

이 페이지는 [개발 컨테이너에 Claude Code 추가](#add-claude-code-to-your-dev-container)를 다룬 후, 자체 포함된 구성 항목 집합을 제공합니다: 재구축 시 인증 유지, 조직 정책 적용, 네트워크 송신 제한, 권한 프롬프트 없이 실행. 설정과 일치하는 항목을 읽으세요.

<Warning>
  개발 컨테이너가 상당한 보호를 제공하지만, 모든 공격에 완전히 면역인 시스템은 없습니다.
  `--dangerously-skip-permissions`으로 실행할 때, 개발 컨테이너는 [`~/.claude`](/docs/ko/claude-directory)에 저장된 Claude Code 자격 증명을 포함하여 컨테이너 내에서 접근 가능한 모든 것을 악의적인 프로젝트가 유출하는 것을 방지하지 않습니다.
  신뢰할 수 있는 저장소로 개발할 때만 개발 컨테이너를 사용하고 Claude의 활동을 모니터링하세요.
  호스트 시크릿(예: `~/.ssh` 또는 클라우드 자격 증명 파일)을 컨테이너에 마운트하지 마세요. 대신 저장소 범위 또는 단기 토큰을 사용하세요.
</Warning>

<Accordion title="개발 컨테이너가 편집기와 어떻게 작동하는지">
  <img src="https://mintcdn.com/claude-code/YvJyjZfd9yMihr0i/images/devcontainer-architecture.svg?fit=max&auto=format&n=YvJyjZfd9yMihr0i&q=85&s=9017b1d16a446c6cc37ba562f35b9aae" className="dark:hidden" alt="호스트의 편집기가 Docker 개발 컨테이너에 연결되는 것을 보여주는 다이어그램입니다. Claude Code, 터미널 및 빌드 도구는 컨테이너 내에서 실행됩니다. 호스트 저장소는 컨테이너에 바인드 마운트되어 작업 공간으로 사용됩니다." width="640" height="300" data-path="images/devcontainer-architecture.svg" />

  <img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/devcontainer-architecture-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=a0a340b1f2afc6a590696102c8acaaca" className="hidden dark:block" alt="호스트의 편집기가 Docker 개발 컨테이너에 연결되는 것을 보여주는 다이어그램입니다. Claude Code, 터미널 및 빌드 도구는 컨테이너 내에서 실행됩니다. 호스트 저장소는 컨테이너에 바인드 마운트되어 작업 공간으로 사용됩니다." width="640" height="300" data-path="images/devcontainer-architecture-dark.svg" />

  개발 컨테이너는 Docker 컨테이너로 실행되며, 머신 또는 GitHub Codespaces와 같은 클라우드 호스트에서 실행될 수 있습니다. Dev Containers 사양을 지원하는 편집기(예: VS Code, GitHub Codespaces, JetBrains IDE 또는 Cursor)가 해당 컨테이너에 연결됩니다. 편집기에서 파일을 평소대로 탐색하고 편집하지만, 통합 터미널, 언어 서버 및 빌드 도구는 모두 호스트가 아닌 컨테이너 내에서 실행됩니다. 일반 Vim과 같이 개발 컨테이너를 지원하지 않는 편집기는 이 워크플로우에 포함되지 않습니다.

  Claude Code는 컨테이너 내에서 실행되므로 프로젝트의 나머지 도구 체인과 동일한 파일, 종속성 및 도구를 봅니다. VS Code에서는 [Claude Code 확장 패널](/docs/ko/vs-code)을 사용하거나 통합 터미널에서 `claude`를 실행할 수 있습니다. 둘 다 컨테이너 내에서 실행되며 동일한 `~/.claude` 구성을 공유합니다.
</Accordion>

<h2 id="add-claude-code-to-your-dev-container">
  개발 컨테이너에 Claude Code 추가
</h2>

Claude Code는 [Claude Code Dev Container Feature](https://github.com/anthropics/devcontainer-features/tree/main/src/claude-code)를 통해 모든 개발 컨테이너에 설치됩니다.

설정은 VS Code, GitHub Codespaces 또는 JetBrains IDE와 같이 Dev Containers 사양을 지원하는 모든 도구와 함께 작동합니다. 아래 단계는 VS Code를 예로 사용합니다.

VS Code 또는 Codespaces에서 컨테이너를 열면 기능이 Claude Code VS Code 확장도 추가합니다. 다른 편집기는 해당 부분을 무시합니다.

<Tip>
  개발 컨테이너를 처음 사용하시나요? [VS Code Dev Containers 튜토리얼](https://code.visualstudio.com/docs/devcontainers/tutorial)은 Docker, 확장 및 첫 번째 컨테이너 열기를 안내합니다. 방화벽 및 지속적 볼륨이 있는 더 완전한 강화된 예제는 [참조 컨테이너 시도](#try-the-reference-container)를 참조하세요.
</Tip>

<Steps>
  <Step title="devcontainer.json 생성 또는 업데이트">
    다음을 저장소에 `.devcontainer/devcontainer.json`으로 저장하거나 기존 파일에 `features` 블록을 추가합니다.

    끝의 버전 태그(예: `:1.0`)는 기능의 설치 스크립트를 고정하며, Claude Code 릴리스가 아닙니다. 기능은 최신 Claude Code를 설치하며, Claude Code는 기본적으로 컨테이너 내에서 자동으로 업데이트됩니다.

    CLI 버전을 고정하거나 자동 업데이트를 비활성화하려면 [조직 정책 적용](#enforce-organization-policy)을 참조하세요.

    ```json .devcontainer/devcontainer.json theme={null}
    {
      "image": "mcr.microsoft.com/devcontainers/base:ubuntu",
      "features": {
        "ghcr.io/anthropics/devcontainer-features/claude-code:1.0": {}
      }
    }
    ```

    `image` 줄을 프로젝트의 기본 이미지로 바꾸거나 기존 파일이 Dockerfile을 사용하는 경우 제거합니다.
  </Step>

  <Step title="컨테이너 재구축">
    Mac에서는 `Cmd+Shift+P`를 사용하거나 Windows 및 Linux에서는 `Ctrl+Shift+P`를 사용하여 VS Code 명령 팔레트를 열고 **Dev Containers: Rebuild Container**를 실행합니다.

    다른 도구의 경우 해당 도구의 재구축 작업을 따르세요. [GitHub Codespaces에서 재구축](https://docs.github.com/en/codespaces/developing-in-a-codespace/rebuilding-the-container-in-a-codespace), [Dev Containers CLI](https://github.com/devcontainers/cli) 또는 IDE의 개발 컨테이너 문서를 참조하세요.
  </Step>

  <Step title="Claude Code에 로그인">
    재구축된 컨테이너에서 터미널을 열고 `claude`를 실행한 다음 인증 프롬프트를 따릅니다.
  </Step>
</Steps>

인증 프롬프트에서 보는 내용은 제공자에 따라 다릅니다:

* **Anthropic**: Claude 또는 Anthropic Console 계정으로 브라우저를 통해 로그인
* **[Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry](/docs/ko/third-party-integrations)**: Claude Code는 클라우드 제공자 자격 증명을 사용하며 브라우저 프롬프트가 없습니다.

클라우드 제공자의 경우 호스트에서 자격 증명 파일을 마운트하는 대신 `containerEnv`, Codespaces 시크릿 또는 클라우드의 워크로드 ID를 통해 자격 증명을 컨테이너에 전달합니다. Claude Code가 읽는 자격 증명 체인은 [Amazon Bedrock](/docs/ko/amazon-bedrock), [Google Cloud의 Agent Platform](/docs/ko/google-vertex-ai) 또는 [Microsoft Foundry](/docs/ko/microsoft-foundry)를 참조하세요.

조직에 맞는 경로를 결정하려면 [API 제공자 선택](/docs/ko/admin-setup#choose-your-api-provider)을 참조하세요.

<Note>
  브라우저 로그인이 완료되었지만 콜백이 컨테이너에 도달하지 않으면 브라우저에 표시된 코드를 복사하여 터미널의 `Paste code here if prompted` 프롬프트에 붙여넣습니다. 이는 편집기의 포트 포워딩이 localhost 콜백을 라우팅하지 않을 때 발생할 수 있습니다.
</Note>

<h2 id="persist-authentication-and-settings-across-rebuilds">
  재구축 시 인증 및 설정 유지
</h2>

기본적으로 컨테이너의 홈 디렉토리는 재구축 시 삭제되므로 엔지니어는 매번 다시 로그인해야 합니다. Claude Code는 인증 토큰, 사용자 설정 및 세션 기록을 [`~/.claude`](/docs/ko/claude-directory) 아래에 저장합니다. 해당 경로에 명명된 볼륨을 마운트하여 재구축 시 이 상태를 유지합니다.

다음 예제는 `node` 사용자의 홈 디렉토리에 볼륨을 마운트합니다:

```json devcontainer.json theme={null}
"mounts": [
  "source=claude-code-config,target=/home/node/.claude,type=volume"
]
```

`/home/node`를 컨테이너의 `remoteUser`의 홈 디렉토리로 바꿉니다. 볼륨을 `~/.claude` 이외의 위치에 마운트하는 경우 [`CLAUDE_CONFIG_DIR`](/docs/ko/env-vars)을 마운트 경로로 설정하여 Claude Code가 해당 위치에서 읽고 쓰도록 합니다.

모든 저장소에서 하나의 볼륨을 공유하는 대신 프로젝트별로 상태를 격리하려면 소스 이름에 `${devcontainerId}` 변수를 포함합니다. [참조 구성](https://github.com/anthropics/claude-code/blob/main/.devcontainer/devcontainer.json)은 이 목적을 위해 `source=claude-code-config-${devcontainerId}`를 사용합니다.

GitHub Codespaces에서 `~/.claude`는 codespace를 중지하고 시작할 때 유지되지만 컨테이너를 재구축할 때는 여전히 지워지므로 위의 볼륨 마운트가 여기에도 적용됩니다. codespace 간에 인증을 유지하려면 [`claude setup-token`](/docs/ko/authentication#generate-a-long-lived-token)에서 `ANTHROPIC_API_KEY` 또는 `CLAUDE_CODE_OAUTH_TOKEN`을 [Codespaces 시크릿](https://docs.github.com/en/codespaces/managing-your-codespaces/managing-your-account-specific-secrets-for-github-codespaces)으로 저장합니다. Codespaces는 시크릿을 컨테이너 내에서 자동으로 환경 변수로 사용 가능하게 합니다.

<h2 id="enforce-organization-policy">
  조직 정책 적용
</h2>

개발 컨테이너는 동일한 이미지와 구성이 모든 엔지니어의 머신에서 실행되므로 조직 정책을 적용하기에 편리한 장소입니다.

Claude Code는 Linux에서 `/etc/claude-code/managed-settings.json`을 읽고 [설정 계층](/docs/ko/settings#how-scopes-interact)에서 가장 높은 우선순위로 적용하므로 해당 값은 엔지니어가 `~/.claude` 또는 프로젝트의 `.claude/` 디렉토리에서 설정한 모든 것을 재정의합니다. Dockerfile에서 파일을 제자리에 복사합니다:

```dockerfile Dockerfile theme={null}
RUN mkdir -p /etc/claude-code
COPY managed-settings.json /etc/claude-code/managed-settings.json
```

Dockerfile이 저장소에 있으므로 쓰기 액세스 권한이 있는 모든 사람이 이 단계를 변경하거나 제거할 수 있습니다. 엔지니어가 저장소 파일을 편집하여 우회할 수 없는 정책의 경우 [서버 관리 설정](/docs/ko/server-managed-settings) 또는 MDM을 통해 관리 설정을 제공합니다. 사용 가능한 키 및 기타 전달 경로는 [관리 설정 파일](/docs/ko/settings#settings-files)을 참조하세요.

컨테이너의 모든 Claude Code 세션에 적용되는 [환경 변수](/docs/ko/env-vars)를 설정하려면 `devcontainer.json`의 `containerEnv`에 추가합니다. 다음 예제는 원격 분석 및 오류 보고를 거부하고 Claude Code가 설치 후 자동으로 업데이트되는 것을 방지합니다:

```json devcontainer.json theme={null}
"containerEnv": {
  "CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC": "1",
  "DISABLE_AUTOUPDATER": "1"
}
```

Dev Container Feature는 항상 최신 Claude Code 릴리스를 설치합니다. 재현 가능한 빌드를 위해 특정 Claude Code 버전을 고정하려면 기능을 사용하는 대신 Dockerfile에서 `npm install -g @anthropic-ai/claude-code@X.Y.Z`로 설치하고 위와 같이 `DISABLE_AUTOUPDATER`를 설정합니다.

권한 규칙, 도구 제한 및 MCP 서버 허용 목록을 포함한 정책 제어의 전체 목록은 [조직을 위한 Claude Code 설정](/docs/ko/admin-setup)을 참조하세요.

[MCP 서버](/docs/ko/mcp)를 컨테이너 내에서 사용 가능하게 하려면 저장소 루트의 `.mcp.json` 파일에서 [프로젝트 범위](/docs/ko/mcp#mcp-installation-scopes)로 정의하여 개발 컨테이너 구성과 함께 체크인됩니다. 로컬 stdio 서버가 의존하는 모든 바이너리를 Dockerfile에 설치하고 원격 서버 도메인을 네트워크 허용 목록에 추가합니다.

<h2 id="restrict-network-egress">
  네트워크 송신 제한
</h2>

컨테이너의 아웃바운드 트래픽을 Claude Code가 필요로 하는 도메인으로만 제한할 수 있습니다. 추론 및 인증 도메인은 [네트워크 액세스 요구 사항](/docs/ko/network-config#network-access-requirements)을 참조하고, 선택적 원격 분석 및 오류 보고 연결 및 비활성화 방법은 [원격 분석 서비스](/docs/ko/data-usage#telemetry-services)를 참조하세요.

참조 컨테이너에는 Claude Code 및 개발 도구가 필요로 하는 도메인을 제외한 모든 아웃바운드 트래픽을 차단하는 [`init-firewall.sh`](https://github.com/anthropics/claude-code/blob/main/.devcontainer/init-firewall.sh) 스크립트가 포함되어 있습니다. 컨테이너 내에서 방화벽을 실행하려면 추가 권한이 필요하므로 참조는 `runArgs`를 통해 `NET_ADMIN` 및 `NET_RAW` 기능을 추가합니다. 방화벽 스크립트 및 이러한 기능은 Claude Code 자체에는 필요하지 않습니다. 이를 제외하고 대신 자신의 네트워크 제어에 의존할 수 있습니다.

<h2 id="run-without-permission-prompts">
  권한 프롬프트 없이 실행
</h2>

컨테이너가 Claude Code를 비루트 사용자로 실행하고 명령 실행을 컨테이너로 제한하므로 무인 작동을 위해 `--dangerously-skip-permissions`을 전달할 수 있습니다. CLI는 루트로 시작될 때 이 플래그를 거부하므로 `remoteUser`가 비루트 계정으로 설정되어 있는지 확인합니다.

권한 프롬프트를 건너뛰면 도구 호출을 실행하기 전에 검토할 기회가 제거됩니다. Claude는 여전히 바인드 마운트된 작업 공간의 모든 파일을 수정할 수 있으며, 이는 호스트에 직접 나타나고 컨테이너의 네트워크 정책이 허용하는 모든 것에 도달할 수 있습니다. 이 플래그를 [위의 네트워크 송신 제한](#restrict-network-egress)과 쌍으로 사용하여 우회된 세션이 도달할 수 있는 것을 제한합니다.

안전 검사를 비활성화하지 않고 더 적은 프롬프트를 원하면 대신 [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)를 고려하세요. 이는 분류기가 실행 전에 작업을 검토합니다. 엔지니어가 `--dangerously-skip-permissions`을 전혀 사용하지 못하도록 하려면 [관리 설정](/docs/ko/settings#permission-settings)에서 `permissions.disableBypassPermissionsMode`를 `"disable"`로 설정합니다.

<h2 id="try-the-reference-container">
  참조 컨테이너 시도
</h2>

[`anthropics/claude-code`](https://github.com/anthropics/claude-code/tree/main/.devcontainer) 저장소에는 CLI, 송신 방화벽, 지속적 볼륨 및 Zsh 기반 셸을 결합하는 예제 개발 컨테이너가 포함되어 있습니다. 이는 유지 관리되는 기본 이미지가 아닌 작동 예제로 제공됩니다. 이를 사용하여 자신의 구성에 적용하기 전에 조각들이 어떻게 맞는지 확인하세요.

<Steps>
  <Step title="필수 구성 요소 설치">
    VS Code 및 [Dev Containers 확장](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)을 설치합니다.
  </Step>

  <Step title="참조 복제">
    [Claude Code 저장소](https://github.com/anthropics/claude-code)를 복제하고 VS Code에서 엽니다.
  </Step>

  <Step title="컨테이너에서 다시 열기">
    메시지가 표시되면 **Reopen in Container**를 클릭하거나 명령 팔레트에서 **Dev Containers: Reopen in Container**를 실행합니다.
  </Step>

  <Step title="Claude Code 시작">
    컨테이너 빌드가 완료되면 `` Ctrl+` ``로 터미널을 열고 `claude`를 실행하여 로그인하고 첫 번째 세션을 시작합니다.
  </Step>
</Steps>

이 구성을 자신의 프로젝트와 함께 사용하려면 `.devcontainer/` 디렉토리를 저장소에 복사하고 도구 체인에 맞게 Dockerfile을 조정하거나 [개발 컨테이너에 Claude Code 추가](#add-claude-code-to-your-dev-container)로 돌아가 이미 있는 설정에만 기능을 추가합니다.

참조 구성은 세 개의 파일로 구성됩니다. 기능을 통해 자신의 개발 컨테이너에 Claude Code를 추가할 때 이들 중 어느 것도 필요하지 않지만 조각들을 결합하는 한 가지 방법을 보여줍니다.

| 파일                                                                                                         | 목적                                                |
| ---------------------------------------------------------------------------------------------------------- | ------------------------------------------------- |
| [`devcontainer.json`](https://github.com/anthropics/claude-code/blob/main/.devcontainer/devcontainer.json) | 볼륨 마운트, `runArgs` 기능, VS Code 확장 및 `containerEnv` |
| [`Dockerfile`](https://github.com/anthropics/claude-code/blob/main/.devcontainer/Dockerfile)               | 기본 이미지, 개발 도구 및 Claude Code 설치                    |
| [`init-firewall.sh`](https://github.com/anthropics/claude-code/blob/main/.devcontainer/init-firewall.sh)   | 허용된 도메인을 제외한 모든 아웃바운드 네트워크 트래픽 차단                 |

<h2 id="next-steps">
  다음 단계
</h2>

Claude Code가 개발 컨테이너에서 실행되면 아래 페이지는 조직 롤아웃의 나머지 부분을 다룹니다. 인증 경로 선택, 저장소 외부에서 관리 정책 제공, 사용량 모니터링 및 Claude Code가 저장하고 전송하는 것 이해:

* [조직을 위한 Claude Code 설정](/docs/ko/admin-setup): 인증 제공자 선택, 정책이 장치에 도달하는 방법 결정 및 롤아웃 계획
* [서버 관리 설정](/docs/ko/server-managed-settings): Claude.ai 관리 콘솔에서 관리 정책을 제공하여 엔지니어가 저장소 파일을 편집하여 우회할 수 없도록 합니다.
* [사용량 모니터링 및 활동 감사](/docs/ko/monitoring-usage): OpenTelemetry 메트릭을 내보내고 팀이 실행 중인 것을 검토합니다.
* [네트워크 액세스 요구 사항](/docs/ko/network-config#network-access-requirements): 프록시 및 방화벽을 위한 전체 도메인 허용 목록
* [원격 분석 서비스 및 거부](/docs/ko/data-usage#telemetry-services): Claude Code가 기본적으로 전송하는 것 및 비활성화하는 환경 변수
* [`.claude` 디렉토리 탐색](/docs/ko/claude-directory): 볼륨 마운트가 보유하는 것(자격 증명, 설정 및 세션 기록 포함)
* [보안 모델](/docs/ko/security): Claude Code의 권한 시스템, 샌드박싱 및 프롬프트 주입 보호가 어떻게 맞는지
* [권한 모드](/docs/ko/permission-modes): 계획 모드에서 자동 모드에서 우회까지의 전체 범위 및 각각을 사용할 때
