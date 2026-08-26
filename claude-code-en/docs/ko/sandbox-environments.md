> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 샌드박스 환경 선택

> Claude Code 샌드박스 옵션 비교: 기본 제공 샌드박스 Bash 도구, 샌드박스 런타임, 개발 컨테이너, Docker, VM. 위협 모델에 맞는 적절한 격리를 선택합니다.

Claude Code를 격리하면 세션이 호스트에서 읽고, 쓰고, 네트워크에 접근할 수 있는 범위를 제한합니다. 이는 Claude가 더 적은 권한 프롬프트로 작동하도록 하거나, 무인 상태로 실행하거나, 완전히 신뢰하지 않는 코드를 가리킬 때 가장 중요합니다.

Claude Code는 경량 명령별 샌드박스부터 완전히 별도의 가상 머신까지 여러 종류의 격리된 환경에서 실행될 수 있습니다. 이 페이지에서는 격리 방식을 비교하고, 위협 모델에 맞는 방식을 선택하는 데 도움을 주며, 조직 전체에서 해당 선택을 적용하는 방법을 보여줍니다.

<Info>
  더 광범위한 보안 모델은 [보안](/docs/ko/security)을 참조하세요. Agent SDK 배포의 경우 [안전한 배포](/docs/ko/agent-sdk/secure-deployment)를 참조하세요.
</Info>

<h2 id="compare-sandboxing-approaches">
  샌드박스 방식 비교
</h2>

아래 표의 처음 두 가지 방식은 컨테이너 없이 호스트 운영 체제에서 실행됩니다. 나머지는 Claude Code를 컨테이너 또는 가상 머신 내에 배치합니다.

| 방식                                        | 격리되는 항목                                  | Docker 필요 | 설정 노력                         |
| :---------------------------------------- | :--------------------------------------- | :-------- | :---------------------------- |
| [샌드박스 Bash 도구](#sandboxed-bash-tool)      | Bash 명령 및 자식 프로세스                        | 아니오       | macOS에서 최소; Linux 및 WSL2에서 낮음 |
| [샌드박스 런타임](#sandbox-runtime)              | 전체 Claude Code 프로세스(파일 도구, MCP 서버, 훅 포함) | 아니오       | 낮음                            |
| [개발 컨테이너](#dev-containers)                | 전체 개발 환경                                 | 예         | 중간                            |
| [사용자 정의 컨테이너](#custom-container)          | 전체 개발 환경                                 | 예         | 중간\~높음                        |
| [가상 머신](#virtual-machine)                 | 전체 운영 체제                                 | 아니오       | 높음                            |
| [웹의 Claude Code](#claude-code-on-the-web) | 전체 운영 체제(Anthropic에서 호스팅)                | 아니오       | 없음; Claude 구독 및 GitHub 필요     |

[샌드박스 Bash 도구](/docs/ko/sandboxing)는 Claude Code에 기본 제공되며 Bash 명령만 제한합니다. 기본 제공 파일 도구, MCP 서버, 훅은 여전히 호스트에서 직접 실행됩니다. 표의 다른 모든 방식은 전체 Claude Code 프로세스를 격리 경계 내에 배치하므로 파일 도구, MCP 서버, 훅도 제한됩니다.

<Warning>
  샌드박스 격리는 침해의 영향을 줄이지만 위험을 완전히 제거하지는 않습니다. 네트워크 송신을 허용하는 모든 방식은 여전히 에이전트가 읽을 수 있는 데이터를 유출할 수 있으며, 프로젝트 디렉토리를 쓰기 가능하게 마운트하는 모든 방식은 여전히 해당 코드를 수정할 수 있습니다. 샌드박스를 하드 제어로 사용하기 전에 [보안 제한 사항](/docs/ko/sandboxing#security-limitations)을 검토하세요.

  격리는 또한 모델로 전송되는 항목을 변경하지 않습니다. 프롬프트와 Claude가 읽는 파일은 샌드박스 여부와 관계없이 Anthropic API 또는 구성된 제공자에게 전송됩니다. Claude Code가 전송하는 항목과 이를 줄이는 방법은 [데이터 사용](/docs/ko/data-usage)을 참조하세요.
</Warning>

<h2 id="choose-an-approach">
  방식 선택
</h2>

아래 행과 목표를 일치시킨 다음 뒤따르는 세부 정보 섹션을 읽으세요.

| 원하는 작업                                                   | 시작 위치                                                                                     |
| :------------------------------------------------------- | :---------------------------------------------------------------------------------------- |
| 자신의 머신에서 일상 작업 중 권한 프롬프트 감소                              | [샌드박스 Bash 도구](/docs/ko/sandboxing), `/sandbox`로 활성화                                           |
| Claude가 `--dangerously-skip-permissions` 또는 자동 모드로 무인 작동 | 사전 구성된 [개발 컨테이너](/docs/ko/devcontainer), 모든 컨테이너 또는 VM, 또는 [샌드박스 런타임](#sandbox-runtime)        |
| Docker 없이 MCP 서버 및 훅과 Bash 격리                            | 샌드박스 런타임                                                                                  |
| 신뢰할 수 없는 저장소에서 작업                                        | 전용 가상 머신 또는 Claude 구독 및 연결된 GitHub 계정이 있는 경우 [웹의 Claude Code](/docs/ko/claude-code-on-the-web) |
| 팀 전체에서 샌드박스 환경 표준화                                       | 사전 구성된 [개발 컨테이너](/docs/ko/devcontainer), 저장소에 복사됨                                              |
| 로컬 설정이 없는 장치에서 Claude Code 사용                            | [웹의 Claude Code](/docs/ko/claude-code-on-the-web), Claude 구독 및 연결된 GitHub 계정 필요                |
| 조직의 모든 개발자를 위한 격리 필요                                     | [조직 전체 격리 적용](#enforce-isolation-across-an-organization)                                  |
| 기본 Windows 호스트에서 작업                                      | 컨테이너 또는 VM, 또는 WSL2 내에서 Bash 샌드박스 실행                                                      |

<h3 id="how-isolation-relates-to-permission-modes">
  격리가 권한 모드와 어떻게 관련되는지
</h3>

[권한 모드](/docs/ko/permission-modes)는 도구 호출이 실행되는지 여부와 먼저 프롬프트를 받는지 여부를 결정합니다. 격리는 명령이 실행되면 접근할 수 있는 항목을 제한합니다. 둘은 함께 작동합니다: 권한 모드가 작업을 묻지 않고 실행하도록 하면 격리 경계는 해당 작업이 도달할 수 있는 항목을 제한합니다.

`--dangerously-skip-permissions`를 전달하면 Claude는 먼저 묻지 않고 작동합니다. 명시적 [요청 규칙](/docs/ko/permissions#manage-permissions), 커넥터 도구 [조직이 `ask`로 설정한](/docs/ko/mcp#organization-controls-on-connector-tools), [`requiresUserInteraction`으로 표시된](/docs/ko/mcp#require-approval-for-a-specific-tool) MCP 도구, 그리고 `/` 또는 홈 디렉토리를 대상으로 하는 제거에 대해서만 프롬프트를 받습니다. 실수를 잡을 프롬프트가 없으면 선택한 격리 경계가 시스템을 보호하는 것입니다. 항상 컨테이너, VM 또는 [샌드박스 런타임](#sandbox-runtime) 내에서 `--dangerously-skip-permissions` 세션을 실행하여 파일 도구, MCP 서버, 훅도 경계 내에 있도록 하세요.

[자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)는 프롬프트를 요청을 초과하여 확대되거나 인식되지 않은 인프라를 대상으로 하거나 Claude가 읽은 적대적 콘텐츠에 의해 주도되는 것으로 보이는 작업을 검토하고 차단하는 분류기로 바꿉니다. 분류기는 작업별 제어이지 격리 경계가 아니므로 격리 경계는 여전히 무인 실행을 위한 심층 방어를 추가하며 `--dangerously-skip-permissions`의 경우처럼 필수는 아닙니다.

[샌드박스 Bash 도구](#sandboxed-bash-tool) 자체는 Bash만 제한하므로 두 모드 모두에서 완전히 무인 실행에는 충분하지 않습니다. 방식을 계층화할 수 있습니다: 컨테이너 또는 VM 내에서 샌드박스 Bash 도구를 실행하면 외부 환경 경계 위에 OS 수준 명령 제한이 제공됩니다. Bash 샌드박스 자체가 권한 규칙 및 권한 모드와 상호 작용하는 방식은 [샌드박싱이 권한 및 권한 모드와 어떻게 관련되는지](/docs/ko/sandboxing#how-sandboxing-relates-to-permissions-and-permission-modes)를 참조하세요.

<h2 id="sandboxed-bash-tool">
  샌드박스 Bash 도구
</h2>

<Note>
  이 옵션은 기본 Windows를 지원하지 않습니다. Windows 호스트에서는 WSL2 또는 아래의 컨테이너 또는 VM 방식 중 하나를 사용하세요.
</Note>

샌드박스 Bash 도구는 Claude Code에 기본 제공됩니다. 운영 체제 기본 요소를 사용하여 Claude가 실행하는 모든 Bash 명령의 파일 시스템 및 네트워크 접근을 제한합니다: Seatbelt(기본 제공 macOS 샌드박스) 및 Linux와 WSL2의 [bubblewrap](https://github.com/containers/bubblewrap). 기본적으로 작업 디렉토리에 대한 쓰기를 허용하고 명령이 새 네트워크 도메인이 필요할 때 처음으로 프롬프트합니다.

`/sandbox` 명령으로 활성화하세요. [샌드박싱](/docs/ko/sandboxing) 가이드는 승인 모드, 기본 경계, 이를 확대하거나 좁히는 방법을 다룹니다.

명령별 샌드박스는 세션에서 실행되는 모든 것을 다루지 않습니다:

* Read, Edit, WebFetch와 같은 다른 [기본 제공 도구](/docs/ko/tools-reference)는 Claude Code 프로세스 내에서 실행되며 임의의 코드를 생성하지 않습니다. [권한 규칙](/docs/ko/permissions)이 경로 또는 도메인으로 이들을 제어합니다.
* [MCP](/docs/ko/mcp) 서버 및 훅은 호스트에서 제약 없이 실행되는 별도의 프로세스입니다.

기본 제공 도구, MCP 서버, 훅을 모두 하나의 OS 경계 뒤에 배치하려면 전체 Claude Code 프로세스를 [샌드박스 런타임](#sandbox-runtime), [개발 컨테이너](#dev-containers) 또는 [사용자 정의 컨테이너](#custom-container) 내에서 실행하세요.

<h2 id="sandbox-runtime">
  샌드박스 런타임
</h2>

[`@anthropic-ai/sandbox-runtime`](https://github.com/anthropic-experimental/sandbox-runtime) 패키지는 전체 프로세스를 기본 제공 Bash 샌드박스가 사용하는 동일한 Seatbelt 또는 bubblewrap 격리로 래핑합니다. Claude Code를 통해 실행하면 Bash뿐만 아니라 세션의 모든 도구, 훅, MCP 서버를 제한합니다. 런타임은 베타 연구 미리보기이며 패키지가 발전함에 따라 구성 형식이 변경될 수 있습니다.

런타임은 기본적으로 모든 쓰기 및 네트워크 접근을 거부하므로 Claude Code를 통해 실행하기 전에 구성하세요. `~/.srt-settings.json` 또는 `--settings`로 전달하는 파일에서 최소한 프로젝트 디렉토리 및 Claude Code의 구성 경로 `~/.claude` 및 `~/.claude.json`에 대한 쓰기 접근을 허용하세요. `api.anthropic.com` 또는 구성된 제공자의 엔드포인트를 포함하여 세션이 필요한 네트워크 도메인을 허용하세요. 전체 구성 스키마는 패키지 [README](https://github.com/anthropic-experimental/sandbox-runtime)를 참조하세요.

설정 파일이 준비되면 `npx`로 Claude Code를 시작하고 래핑할 명령으로 `claude`를 전달하세요:

```bash theme={null}
npx @anthropic-ai/sandbox-runtime claude
```

Claude Code는 구성한 파일 시스템 및 네트워크 경계를 사용하여 샌드박스 내에서 시작됩니다. 동일한 명령은 독립 실행형 MCP 서버 또는 다른 도우미 프로세스를 샌드박싱하는 데 작동합니다.

<h2 id="dev-containers">
  개발 컨테이너
</h2>

개발 컨테이너는 VS Code 또는 호환 가능한 편집기가 관리하는 Docker 컨테이너 내에서 Claude Code를 실행하며 프로젝트가 마운트됩니다. 저장소의 `.devcontainer/` 디렉토리로 자신의 것을 정의할 수 있습니다.

claude-code 저장소는 시작점으로 기본 거부 iptables 방화벽이 있는 [예제 개발 컨테이너](/docs/ko/devcontainer)를 게시합니다. 저장소에 복사하고 방화벽 허용 목록, 기본 이미지, 고정된 Claude Code 버전을 환경에 맞게 조정하세요. 방화벽이 승인되지 않은 송신을 차단하므로 이와 같은 구성은 무인 작업을 위해 `--dangerously-skip-permissions`로 Claude Code를 실행하는 것을 지원합니다.

<h2 id="custom-container">
  사용자 정의 컨테이너
</h2>

자신의 네트워크 정책, 마운트된 볼륨, seccomp 프로필을 사용하여 모든 Docker 또는 OCI 컨테이너 이미지에서 Claude Code를 실행할 수 있습니다. 이는 기존 컨테이너 인프라 또는 CI 러너가 있는 조직의 가장 일반적인 경로입니다.

여러 관리형 샌드박스 및 원격 실행 서비스가 컨테이너를 호스팅할 수 있습니다. 운영하는 모든 컨테이너에 동일한 체크리스트가 적용됩니다: 쓰기 가능하게 마운트된 항목, 내부에서 도달 가능한 자격 증명 및 토큰, 네트워크 송신 정책이 허용하는 항목을 검토하세요.

명령별 제한을 위해 컨테이너 내에서 기본 제공 Bash 샌드박스를 계층화할 수 있습니다. 권한 없는 컨테이너는 [샌드박싱 문제 해결](/docs/ko/sandboxing#troubleshooting)에 설명된 중첩 샌드박스 설정이 필요합니다.

<h2 id="virtual-machine">
  가상 머신
</h2>

전용 가상 머신은 자체 커널과 클라우드 또는 microVM 배포에서 자체 가상화된 하드웨어를 사용하여 가장 강력한 분리를 제공합니다. 옵션에는 클라우드 인스턴스, 로컬 하이퍼바이저, Firecracker와 같은 microVM이 포함됩니다.

신뢰할 수 없는 코드를 평가할 때, 보안 정책이 에이전트와 호스트 간의 커널 수준 분리를 요구할 때, 또는 호스트 수준 방식이 규정 준수 요구 사항을 충족하지 않을 때 이 방식을 사용하세요. Docker Desktop의 [샌드박스 기능](https://docs.docker.com/ai/sandboxes/)은 자체 Docker 데몬 및 작업 공간 동기화가 있는 microVM을 제공하며, 이미 Docker Desktop이 있는 호스트에서 Claude Code를 실행할 수 있습니다.

<h2 id="claude-code-on-the-web">
  웹의 Claude Code
</h2>

[웹의 Claude Code](/docs/ko/claude-code-on-the-web)는 각 세션을 격리된 Anthropic 관리 가상 머신에서 실행합니다. 네트워크 프록시는 기본 허용 목록을 적용하고 별도의 프록시는 GitHub 토큰을 샌드박스 외부에 보관하면서 내부에서 저장소 접근을 위한 범위가 지정된 자격 증명을 발급합니다.

인프라를 직접 프로비저닝하지 않고 전체 VM 격리를 원하거나 로컬 개발 환경이 없는 장치에서 작업을 위임할 때 이 방식을 사용하세요. Claude 구독 및 연결된 GitHub 계정이 필요하며 세션은 GitHub에서 저장소를 복제합니다. 계획 가용성 및 GitHub 인증 옵션은 [웹의 Claude Code](/docs/ko/claude-code-on-the-web)를 참조하세요.

<h2 id="enforce-isolation-across-an-organization">
  조직 전체 격리 적용
</h2>

개별 개발자는 위의 모든 방식을 선택할 수 있습니다. 조직이 적용할 수 있는 항목과 어떤 도구로 적용할 수 있는지는 방식에 따라 다릅니다:

* **기본 제공 Bash 샌드박스**: Claude Code가 자체적으로 적용하는 유일한 방식입니다. [관리형 설정](/docs/ko/settings#settings-files)을 통해 `sandbox` 설정 키를 전달하세요. MDM에서 관리하는 파일 또는 Claude.ai의 [서버 관리 설정](/docs/ko/server-managed-settings)을 통해 전달하세요. 배포할 키 및 개발자가 정책을 확대하지 못하도록 하는 방법은 [관리형 설정으로 샌드박싱 적용](/docs/ko/sandboxing#enforce-sandboxing-with-managed-settings)을 참조하세요.
* **개발 컨테이너**: [예제 개발 컨테이너](/docs/ko/devcontainer)를 저장소에 커밋하여 팀 전체에서 환경을 표준화하세요. Claude Code가 컨테이너를 요구하지 않으므로 이는 적용 경계가 아닌 규칙입니다. 개발자가 Claude Code를 외부에서 실행할 수 없어야 하는 경우 조직의 장치 관리 또는 소프트웨어 허용 목록 도구로 적용하세요.
* **사용자 정의 컨테이너 및 VM**: 승인된 이미지를 통해 Claude Code를 배포하고 조직의 장치 관리 또는 소프트웨어 허용 목록 도구를 사용하여 외부 설치를 방지하세요.

<h2 id="see-also">
  참고 항목
</h2>

이 페이지들은 위의 방식에 대한 구성 및 정책 세부 정보를 다룹니다.

* [샌드박싱](/docs/ko/sandboxing): 기본 제공 샌드박스 Bash 도구 구성
* [개발 컨테이너](/docs/ko/devcontainer): 사전 구성된 Docker 개발 컨테이너
* [보안](/docs/ko/security): 전체 Claude Code 보안 모델
* [안전한 배포](/docs/ko/agent-sdk/secure-deployment): Agent SDK 애플리케이션을 위한 격리 지침
* [설정](/docs/ko/settings#sandbox-settings): 관리형 설정 전달을 포함한 모든 샌드박스 구성 키
