> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 샌드박싱된 Bash 도구 구성

> Claude Code의 샌드박싱된 Bash 도구가 파일시스템 및 네트워크 격리를 제공하여 더 안전하고 자율적인 에이전트 실행을 가능하게 하는 방법을 알아봅니다.

Bash 샌드박스를 사용하면 Claude가 대부분의 셸 명령을 권한을 요청하지 않고 실행할 수 있습니다. 각 명령을 승인하는 대신 명령이 접근할 수 있는 파일과 네트워크 도메인을 정의하면 운영 체제가 모든 Bash 명령과 그 자식 프로세스에 대해 해당 경계를 적용합니다.

<Note>
  dev 컨테이너, 사용자 정의 컨테이너, 가상 머신 등 다른 격리 방식을 비교하려면 [샌드박스 환경](/docs/ko/sandbox-environments)을 참조하세요. Bash 이외의 도구에 대한 권한 프롬프트를 줄이려면 [권한 모드](/docs/ko/permission-modes)를 참조하세요.
</Note>

<h2 id="get-started">
  시작하기
</h2>

샌드박스는 Claude Code에 내장되어 있으며 macOS, Linux 및 WSL2에서 실행됩니다. 기본 Windows는 지원되지 않습니다. Windows에서는 WSL2 배포판 내에서 Claude Code를 실행하세요.

macOS에서는 설치할 것이 없습니다: 샌드박싱은 기본 제공 Seatbelt 프레임워크를 사용합니다. Linux 및 WSL2에서 샌드박스는 두 개의 패키지에 의존하며, [Linux 및 WSL2 설정](#set-up-linux-and-wsl2)에서 다룹니다. 아직 설치하지 않았더라도 `/sandbox`로 시작할 수 있습니다. 해당 패널은 누락된 것이 있는지 보여줍니다.

<Steps>
  <Step title="/sandbox 실행">
    Claude Code 세션을 시작하고 `/sandbox` 명령을 실행합니다:

    ```text theme={null}
    /sandbox
    ```

    이는 세 개의 탭이 있는 샌드박스 패널을 엽니다:

    * **Mode**: 샌드박싱된 명령이 승인되는 방식을 선택합니다. 다음 단계에서 다룹니다
    * **Overrides**: 샌드박스에서 실패한 명령이 샌드박싱되지 않은 상태로 실행되도록 폴백할 수 있는지 선택합니다. 이는 [`allowUnsandboxedCommands`](/docs/ko/settings#sandbox-settings) 설정입니다
    * **Config**: 해석된 샌드박스 설정을 봅니다

    패널에 Dependencies 탭만 표시되면 필수 패키지가 누락된 것입니다. [Linux 및 WSL2 설정](#set-up-linux-and-wsl2)에 설명된 대로 설치하고 Claude Code를 다시 시작한 후 `/sandbox`를 다시 실행합니다.
  </Step>

  <Step title="모드 선택">
    Mode 탭에서 자동 허용 또는 일반 권한을 선택합니다. 자동 허용은 샌드박싱된 명령을 프롬프트 없이 실행하고, 일반 권한은 명령이 샌드박싱되었을 때도 일반 권한 프롬프트를 유지합니다. 자동 허용 모드에서 여전히 프롬프트되는 명령은 [샌드박스 모드](#sandbox-modes)를 참조하세요.
  </Step>

  <Step title="Bash 명령 실행">
    Claude에게 빌드 또는 테스트 스위트와 같은 명령을 실행하도록 요청합니다. 기본적으로 샌드박스 내의 명령은 작업 디렉토리 및 세션 임시 디렉토리에만 쓸 수 있습니다. 명령이 새 네트워크 도메인에 처음 액세스해야 할 때 Claude Code가 승인을 요청합니다.

    샌드박싱할 수 없는 명령은 일반 권한 흐름으로 폴백합니다. 이러한 경계를 확대하거나 축소하려면 [샌드박싱 구성](#configure-sandboxing)을 참조하세요.
  </Step>
</Steps>

패널에서 모드를 선택하면 현재 프로젝트에 적용되고 git에 체크인되지 않는 프로젝트의 로컬 설정 `.claude/settings.local.json`에 기록됩니다. 모든 프로젝트에서 샌드박스를 활성화하려면 사용자 설정 `~/.claude/settings.json`에서 [`sandbox.enabled`](/docs/ko/settings#sandbox-settings)를 `true`로 설정합니다. 조직의 모든 개발자에게 샌드박싱을 적용하려면 [관리 설정](#enforce-sandboxing-with-managed-settings)을 사용합니다.

<Warning>
  기본적으로 종속성이 누락되었거나 플랫폼이 지원되지 않아 샌드박스를 시작할 수 없으면 Claude Code는 경고를 표시하고 샌드박싱 없이 명령을 실행합니다. 대신 하드 실패로 만들려면 [`sandbox.failIfUnavailable`](/docs/ko/settings#sandbox-settings)을 `true`로 설정합니다. 이는 샌드박싱을 보안 게이트로 요구하는 관리 배포를 위한 것입니다.
</Warning>

<h3 id="set-up-linux-and-wsl2">
  Linux 및 WSL2 설정
</h3>

Linux 및 WSL2에서 샌드박스는 두 개의 패키지에 의존합니다:

* [`bubblewrap`](https://github.com/containers/bubblewrap): 파일시스템 격리를 적용하는 권한 없는 샌드박싱 도구
* [`socat`](http://www.dest-unreach.org/socat/): 샌드박스 프록시를 통해 네트워크 트래픽을 라우팅하는 데 사용되는 릴레이

배포판의 패키지 관리자로 설치합니다:

<Tabs>
  <Tab title="Ubuntu/Debian">
    ```bash theme={null}
    sudo apt-get install bubblewrap socat
    ```
  </Tab>

  <Tab title="Fedora">
    ```bash theme={null}
    sudo dnf install bubblewrap socat
    ```
  </Tab>
</Tabs>

설치 후 `/sandbox`의 Dependencies 탭은 `ripgrep`, `bubblewrap`, `socat` 및 seccomp 필터가 플랫폼에서 사용 가능한지 보여줍니다. Ripgrep은 기본 Claude Code 바이너리와 함께 번들됩니다. seccomp 필터는 선택 사항이며 Unix 도메인 소켓 차단을 추가합니다. 누락된 경우 `npm install -g @anthropic-ai/sandbox-runtime`으로 설치합니다.

필수 종속성이 누락되면 설치할 때까지 Dependencies 탭만 표시됩니다. 종속성 확인은 시작 시 실행되므로 패키지 설치 후 Claude Code를 다시 시작하여 `/sandbox`가 이를 감지하도록 합니다.

<AccordionGroup>
  <Accordion title="Ubuntu 24.04 이상: bubblewrap이 사용자 네임스페이스를 생성하도록 허용">
    Ubuntu 24.04 이상에서 기본 AppArmor 정책은 bubblewrap이 격리에 필요한 사용자 네임스페이스를 생성하는 것을 방지합니다.

    WSL2 내부를 포함하여 환경이 이 제한을 적용하는지 확인하려면 `sysctl kernel.apparmor_restrict_unprivileged_userns`를 실행합니다. 키가 존재하지 않거나 `0`을 반환하면 이 단계를 건너뜁니다. `1`을 반환하면 `bwrap`에 이 기능을 부여하는 AppArmor 프로필을 추가합니다:

    ```bash theme={null}
    sudo tee /etc/apparmor.d/bwrap > /dev/null <<'EOF'
    abi <abi/4.0>,
    include <tunables/global>

    profile bwrap /usr/bin/bwrap flags=(unconfined) {
      userns,
      include if exists <local/bwrap>
    }
    EOF
    ```

    프로필은 `bwrap` 자체에만 적용되며 샌드박스 내에서 실행되는 명령에는 적용되지 않습니다. AppArmor를 다시 로드하여 적용합니다:

    ```bash theme={null}
    sudo systemctl reload apparmor
    ```
  </Accordion>

  <Accordion title="WSL2 참고 사항">
    PowerShell에서 `wsl -l -v`로 WSL 버전을 확인합니다. `Sandboxing requires WSL2`가 표시되면 배포판이 WSL1을 실행 중입니다. WSL2로 업그레이드하거나 샌드박싱 없이 Claude Code를 실행합니다.

    WSL2에서 샌드박싱된 명령은 `cmd.exe`, `powershell.exe` 또는 `/mnt/c/` 아래의 모든 항목과 같은 Windows 바이너리를 시작할 수 없습니다. WSL은 이를 Unix 소켓을 통해 Windows 호스트로 전달하며, 샌드박스가 이를 차단합니다. 명령이 Windows 바이너리를 호출해야 하면 [`excludedCommands`](/docs/ko/settings#sandbox-settings)에 추가하여 샌드박스 외부에서 실행되도록 합니다.
  </Accordion>
</AccordionGroup>

<h3 id="sandbox-modes">
  샌드박스 모드
</h3>

Claude Code는 두 가지 샌드박스 모드를 제공합니다:

**자동 허용 모드**: Bash 명령은 샌드박스 내에서 실행을 시도하며 권한 없이 자동으로 허용됩니다. 허용되지 않은 호스트에 대한 네트워크 액세스가 필요한 경우 등 샌드박싱할 수 없는 명령은 일반 권한 흐름으로 폴백합니다. 여기서 Claude Code는 [권한 규칙](/docs/ko/permissions)을 확인하고 해당 규칙이 이미 허용하지 않는 모든 명령에 대해 기본 모드에서 프롬프트하거나 [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)에서 분류기를 사용합니다.

자동 허용 모드에서도 다음이 적용됩니다:

* 명시적 [거부 규칙](/docs/ko/permissions)은 항상 존중됩니다
* `/`, 홈 디렉토리 또는 기타 중요한 시스템 경로를 대상으로 하는 `rm` 또는 `rmdir` 명령은 여전히 권한 프롬프트를 트리거합니다
* `Bash(git push *)`와 같은 콘텐츠 범위 [ask 규칙](/docs/ko/permissions)은 샌드박싱된 명령에 대해서도 프롬프트를 강제합니다
* 단순 `Bash` ask 규칙 또는 동등한 `Bash(*)` 형식은 샌드박싱된 명령에 대해 건너뛰어지며, 일반 권한 흐름으로 폴백되는 명령에는 여전히 적용됩니다

**일반 권한 모드**: 모든 Bash 명령은 샌드박싱되었더라도 일반 권한 흐름을 거칩니다. 이는 더 많은 제어를 제공하지만 더 많은 승인이 필요합니다.

두 모드 모두에서 샌드박스는 동일한 파일시스템 및 네트워크 제한을 적용합니다. 차이점은 샌드박싱된 명령이 자동 승인되는지 또는 명시적 권한이 필요한지 여부뿐입니다.

세션 임시 디렉토리는 기본적으로 작업 디렉토리와 함께 샌드박스 내에서 쓰기 가능합니다. Claude Code는 샌드박싱된 명령에 대해 `$TMPDIR`을 이 디렉토리로 설정하므로 임시 파일을 작성하는 도구는 추가 구성 없이 작동합니다. 샌드박싱되지 않은 명령은 셸의 `$TMPDIR`을 변경되지 않은 상태로 상속하므로, 샌드박싱된 명령과 샌드박싱되지 않은 명령은 `$TMPDIR`을 다른 디렉토리로 해석합니다. 두 명령 간에 임시 파일을 전달하려면 작업 디렉토리 아래에 작성하세요.

일부 명령은 샌드박스 내에서 전혀 실행할 수 없습니다. 예를 들어 호환되지 않는 도구나 허용하지 않은 호스트가 필요한 도구입니다. 작업을 실패하거나 샌드박싱을 끄도록 요구하는 대신 Claude Code는 탈출 해치를 포함합니다: 명령이 샌드박스 제한으로 인해 실패하면 Claude는 실패를 분석하고 `dangerouslyDisableSandbox` 매개변수로 명령을 다시 시도할 수 있습니다. 다시 시도된 명령은 샌드박스 외부에서 실행되므로 일반 권한 흐름을 거칩니다: 기본 모드에서는 확인 프롬프트를 받고, [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)에서는 분류기가 사용자에게 프롬프트하는 대신 기본 명령을 평가합니다. 자동 모드에서도 모든 샌드박싱되지 않은 재시도에 대해 프롬프트를 받으려면 `Bash(dangerouslyDisableSandbox:true)`에 대한 [ask 규칙](/docs/ko/permissions#match-by-input-parameter)을 추가합니다.

[샌드박스 설정](/docs/ko/settings#sandbox-settings)에서 `"allowUnsandboxedCommands": false`를 설정하여 이 탈출 해치를 비활성화할 수 있습니다. 비활성화되면 `/sandbox` Overrides 탭에 **Strict sandbox mode**로 표시되며, `dangerouslyDisableSandbox` 매개변수는 완전히 무시되고 모든 명령은 샌드박싱되거나 `excludedCommands`에 명시적으로 나열되어야 합니다.

<Info>
  자동 허용 모드는 권한 모드 설정과 독립적으로 작동합니다. "편집 수락" 모드에 있지 않더라도 자동 허용이 활성화되면 샌드박싱된 Bash 명령이 자동으로 실행됩니다. 이는 샌드박스 경계 내에서 파일을 수정하는 Bash 명령이 파일 편집 도구가 일반적으로 승인을 요구할 때도 프롬프트 없이 실행됨을 의미합니다.
</Info>

<h2 id="configure-sandboxing">
  샌드박싱 구성
</h2>

`settings.json` 파일을 통해 샌드박스 동작을 사용자 정의합니다. 전체 구성 참조는 [설정](/docs/ko/settings#sandbox-settings)을 참조하세요.

기본적으로 샌드박싱된 명령은 현재 작업 디렉토리와 세션 임시 디렉토리에만 쓸 수 있습니다. `kubectl`, `terraform` 또는 `npm`과 같은 하위 프로세스 명령이 해당 디렉토리 외부에 쓰기해야 하면 `sandbox.filesystem.allowWrite`를 사용하여 특정 경로에 대한 액세스를 부여합니다:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "allowWrite": ["~/.kube", "/tmp/build"]
    }
  }
}
```

이러한 경로는 OS 수준에서 적용되므로 샌드박스 내에서 실행되는 모든 명령(자식 프로세스 포함)이 이를 존중합니다. 이는 도구를 `excludedCommands`로 샌드박스에서 완전히 제외하는 것보다 도구가 특정 위치에 쓰기 액세스가 필요할 때 권장되는 방법입니다.

동일한 파일시스템 배열이 여러 [설정 범위](/docs/ko/settings#settings-precedence)에서 정의되면 배열이 병합됩니다: 모든 범위의 경로가 결합되며 대체되지 않습니다.

경로 접두사는 경로가 해석되는 방식을 제어합니다:

| 접두사            | 의미                                                       | 예시                                                                   |
| :------------- | :------------------------------------------------------- | :------------------------------------------------------------------- |
| `/`            | 파일시스템 루트의 절대 경로                                          | `/tmp/build`는 `/tmp/build`로 유지됩니다                                    |
| `~/`           | 홈 디렉토리에 상대적                                              | `~/.kube`는 `$HOME/.kube`가 됩니다                                        |
| `./` 또는 접두사 없음 | 프로젝트 설정의 경우 프로젝트 루트에 상대적이거나, 사용자 설정의 경우 `~/.claude`에 상대적 | `.claude/settings.json`의 `./output`은 `<project-root>/output`으로 해석됩니다 |

이 구문은 절대 경로에 `//path`를 사용하고 프로젝트 상대에 `/path`를 사용하는 [Read 및 Edit 권한 규칙](/docs/ko/permissions#read-and-edit)과 다릅니다. 샌드박스 파일시스템 경로는 표준 규칙을 사용합니다: `/tmp/build`는 절대 경로입니다.

`sandbox.filesystem.denyWrite` 및 `sandbox.filesystem.denyRead`를 사용하여 쓰기 또는 읽기 액세스를 거부할 수도 있으며, `sandbox.filesystem.allowRead`를 사용하여 거부된 영역 내에서 특정 경로 읽기를 다시 허용할 수 있습니다. 읽기 규칙이 겹칠 때 더 구체적인 경로가 우선합니다:

| 예시 규칙                                               | 결과                                                                                                       |
| :-------------------------------------------------- | :------------------------------------------------------------------------------------------------------- |
| `"denyRead": ["~/"]`와 `"allowRead": ["~/projects"]` | `~/projects`는 읽을 수 있으며 홈 디렉토리의 나머지는 차단된 상태로 유지됩니다. 더 좁은 허용이 거부된 영역의 해당 부분을 다시 엽니다                        |
| `"allowRead": ["~/"]`와 `"denyRead": ["~/.env"]`     | `~/.env`는 차단된 상태로 유지되며 홈 디렉토리의 나머지는 읽을 수 있습니다. 정확한 거부는 더 넓은 허용 내에서 유지되므로 광범위한 허용이 비밀을 자동으로 다시 노출할 수 없습니다 |

아래 예제는 홈 디렉토리 전체에서의 읽기를 차단하면서도 현재 프로젝트에서의 읽기를 허용합니다. 상대 경로 `.`이 프로젝트 설정에 있을 때만 프로젝트 루트로 해석되므로 프로젝트의 `.claude/settings.json`에 배치합니다:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "denyRead": ["~/"],
      "allowRead": ["."]
    }
  }
}
```

`.`의 `allowRead`는 이 구성이 프로젝트 설정에 있으므로 프로젝트 루트로 해석됩니다. 동일한 구성을 `~/.claude/settings.json`에 배치했다면 `.`은 `~/.claude`로 해석되고 프로젝트 파일은 `denyRead` 규칙에 의해 차단된 상태로 유지됩니다.

<h3 id="protect-credentials">
  자격증명 보호
</h3>

`sandbox.credentials` 설정은 샌드박싱된 명령이 액세스하면 안 되는 자격증명 파일 및 환경 변수를 선언합니다. 각 항목은 파일 경로 또는 환경 변수와 `mode`를 지정합니다. 전용 `credentials` 블록은 자격증명 규칙을 함께 그룹화하고 일반 파일시스템 규칙과 분리합니다. Claude Code v2.1.187 이상이 필요합니다.

`"mode": "deny"`를 사용하는 항목의 경우 파일 경로는 샌드박스 내에서 읽기가 거부되며, 이는 `filesystem.denyRead`가 적용하는 것과 동일한 제한이고, 환경 변수는 각 샌드박싱된 명령 실행 전에 설정 해제됩니다.

아래 예제는 AWS 자격증명 파일 및 SSH 디렉토리의 읽기를 차단하고 샌드박싱된 명령의 환경에서 `GITHUB_TOKEN` 및 `NPM_TOKEN`을 제거합니다:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "credentials": {
      "files": [
        { "path": "~/.aws/credentials", "mode": "deny" },
        { "path": "~/.ssh", "mode": "deny" }
      ],
      "envVars": [
        { "name": "GITHUB_TOKEN", "mode": "deny" },
        { "name": "NPM_TOKEN", "mode": "deny" }
      ]
    }
  }
}
```

파일 항목은 `"mode": "deny"`만 지원합니다. 환경 변수 항목은 아래에서 설명하는 `"mode": "mask"`도 허용합니다.

파일 경로는 `sandbox.filesystem.*` 설정과 동일한 [접두사 규칙](/docs/ko/settings#sandbox-path-prefixes)을 따르며, 모든 [설정 범위](/docs/ko/settings#settings-precedence)의 `deny` 항목이 병합됩니다. `deny` 항목은 액세스를 좁히기만 하므로 모든 범위는 항목을 추가할 수 있지만 다른 범위가 추가한 항목을 제거할 수는 없습니다.

기본 제공 자격증명 거부 목록이 없으므로 나열한 파일 및 변수만 제한됩니다. 이 설정은 샌드박싱된 Bash 명령에만 영향을 미칩니다. 샌드박싱 여부와 관계없이 모든 하위 프로세스에서 Anthropic 및 클라우드 공급자 자격증명을 제거하려면 [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/ko/env-vars)를 설정합니다.

<h4 id="mask-environment-variables">
  환경 변수 마스킹
</h4>

`"mode": "mask"`는 자격증명을 보호하면서 이를 사용하여 인증하는 도구가 계속 작동하도록 합니다. `deny`는 변수를 완전히 제거하므로 `gh` 또는 `npm`과 같이 이를 필요로 하는 도구도 중단됩니다. Claude Code v2.1.199 이상이 필요합니다.

`mask`를 사용하면 샌드박싱된 명령은 실제 값 대신 세션별 센티널 값을 봅니다. 요청이 자격증명의 `injectHosts` 중 하나에 대해 샌드박스를 떠날 때 [샌드박스 프록시](#network-isolation)는 센티널을 실제 값으로 바꿉니다. 명령과 이것이 기록하는 모든 것은 실제 자격증명을 보유하지 않지만 요청은 여전히 인증됩니다.

프록시는 요청 내용 내에서 자격증명을 대체하므로 이를 봐야 합니다. [`network.tlsTerminate`](/docs/ko/settings#sandbox-settings)를 설정하여 프록시가 TLS 자체를 종료하도록 합니다. 이 없이는 마스킹이 폐쇄 상태로 실패합니다: 명령은 여전히 센티널만 보지만 센티널이 변경되지 않은 상태로 서버에 도달하고 인증이 실패합니다. Claude Code는 시작 시 이 잘못된 구성을 보고합니다.

아래 예제는 두 개의 토큰을 마스킹합니다. `GH_TOKEN`은 `api.github.com`에 대한 요청에서만 대체되고, `NPM_TOKEN`은 `injectHosts`가 없으며 `network.allowedDomains`의 모든 호스트에 대한 요청에서 대체됩니다. 각 `injectHosts` 항목은 자체적으로 `network.allowedDomains`로 커버되어야 합니다.

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "network": {
      "tlsTerminate": {},
      "allowedDomains": ["*.github.com", "registry.npmjs.org"]
    },
    "credentials": {
      "envVars": [
        { "name": "GH_TOKEN", "mode": "mask", "injectHosts": ["api.github.com"] },
        { "name": "NPM_TOKEN", "mode": "mask" }
      ]
    }
  }
}
```

`deny`와 달리 마스킹은 프록시가 나열된 호스트에 실제 자격증명을 보내도록 승인하므로 사용자 또는 관리자가 제어하는 설정에서만 적용됩니다: 사용자 설정, 관리 설정 및 `--settings` CLI 플래그. 리포지토리의 `.claude/settings.json` 또는 `.claude/settings.local.json`의 `mask` 항목, `network.tlsTerminate` 및 [`credentials.allowPlaintextInject`](/docs/ko/settings#sandbox-settings)는 무시됩니다.

동일한 변수가 모든 범위에서 `deny`로 나열되면 `deny`가 우선합니다.

<h2 id="how-sandboxing-works">
  샌드박싱 작동 방식
</h2>

<h3 id="filesystem-isolation">
  파일시스템 격리
</h3>

샌드박싱된 Bash 도구는 파일 시스템 액세스를 특정 디렉토리로 제한합니다:

* **기본 쓰기 동작**: 현재 작업 디렉토리 및 그 하위 디렉토리에 대한 읽기 및 쓰기 액세스, 그리고 `$TMPDIR`이 가리키는 세션 임시 디렉토리에 대한 액세스
* **기본 읽기 동작**: 특정 거부된 디렉토리를 제외한 전체 컴퓨터에 대한 읽기 액세스. 이 기본값은 여전히 `~/.aws/credentials` 및 `~/.ssh/`와 같은 자격 증명 파일 읽기를 허용합니다. [`sandbox.credentials`](#protect-credentials)를 사용하여 이러한 파일의 읽기를 차단하고 비밀 환경 변수를 설정 해제하거나, 경로를 `denyRead`에 추가합니다.
* **차단된 액세스**: 명시적 권한 없이 현재 작업 디렉토리 및 세션 임시 디렉토리 외부의 파일을 수정할 수 없습니다. `~/.bashrc`와 같은 셸 구성 파일 및 `/bin/`의 시스템 바이너리 포함
* **Git worktrees**: 작업 디렉토리가 [연결된 git worktree](/docs/ko/worktrees)일 때, 샌드박스는 또한 메인 저장소의 공유 `.git` 디렉토리에 대한 쓰기를 허용하므로 `git commit`과 같은 명령이 refs 및 인덱스를 업데이트할 수 있습니다. 해당 디렉토리 내의 `hooks/` 및 `config`에 대한 쓰기는 계속 거부됩니다.
* **구성 가능**: 설정을 통해 사용자 정의 허용 및 거부 경로를 정의합니다

`sandbox.filesystem.allowWrite`를 사용하여 추가 경로에 대한 쓰기 액세스를 부여할 수 있습니다. 이러한 제한은 OS 수준에서 적용되므로 Claude의 파일 도구뿐만 아니라 `kubectl`, `terraform`, `npm`과 같은 도구를 포함한 모든 하위 프로세스 명령에 적용됩니다.

<h3 id="network-isolation">
  네트워크 격리
</h3>

네트워크 액세스는 샌드박스 외부에서 실행되는 프록시 서버를 통해 제어됩니다:

* **도메인 제한**: 사전 허용된 도메인이 없습니다. 명령이 새 도메인에 처음 액세스해야 할 때 Claude Code가 승인을 요청합니다. v2.1.191부터 예를 선택하면 현재 세션의 나머지 기간 동안 호스트가 허용되므로 나중에 동일한 호스트에 연결해도 다시 프롬프트가 표시되지 않습니다. [`allowedDomains`](/docs/ko/settings#sandbox-settings)로 도메인을 사전 허용하여 프롬프트를 피합니다.
* **관리 잠금**: [`allowManagedDomainsOnly`](/docs/ko/settings#sandbox-settings)가 관리 설정에서 설정되면 허용되지 않은 도메인이 프롬프트 대신 자동으로 차단되며 관리 설정의 `allowedDomains`만 존중됩니다.
* **사용자 정의 프록시 지원**: 고급 사용자는 나가는 트래픽에 대한 사용자 정의 규칙을 구현할 수 있습니다
* **포괄적 범위**: 제한은 명령으로 생성된 모든 스크립트, 프로그램 및 하위 프로세스에 적용됩니다

<Note>
  기본 제공 프록시는 요청된 호스트 이름을 기반으로 허용 목록을 적용하며, 기본적으로 TLS 트래픽을 종료하거나 검사하지 않습니다. Claude Code v2.1.199 이상에서 사용 가능한 실험적 [`network.tlsTerminate`](/docs/ko/settings#sandbox-settings) 설정은 기본 제공 프록시가 TLS 자체를 종료하도록 하며, 이는 [`mask` 자격 증명 항목](#protect-credentials)이 필요로 합니다. 기본값의 의미는 [보안 제한 사항](#security-limitations)을 참조하고, 위협 모델이 TLS 검사를 요구하면 [사용자 정의 프록시 구성](#custom-proxy-configuration)을 참조하세요.
</Note>

<h3 id="os-level-enforcement">
  OS 수준 적용
</h3>

샌드박싱된 Bash 도구는 운영 체제 보안 기본 요소를 활용합니다:

* **macOS**: 샌드박스 적용을 위해 Seatbelt를 사용합니다
* **Linux**: 격리를 위해 [bubblewrap](https://github.com/containers/bubblewrap)을 사용합니다
* **WSL2**: Linux와 동일하게 bubblewrap을 사용합니다

WSL1은 bubblewrap이 WSL2에서만 사용 가능한 커널 기능을 필요로 하기 때문에 지원되지 않습니다. 이러한 OS 수준의 제한은 Claude Code의 명령으로 생성된 모든 자식 프로세스가 동일한 보안 경계를 상속하도록 보장합니다.

이러한 동일한 기본 요소는 독립 실행형 [`@anthropic-ai/sandbox-runtime`](https://github.com/anthropic-experimental/sandbox-runtime) 패키지로 사용 가능하며, [샌드박스 환경](/docs/ko/sandbox-environments#sandbox-runtime) 페이지에서 전체 Claude Code 프로세스를 래핑하기 위한 별도의 방식으로 다룹니다.

<h2 id="how-sandboxing-relates-to-permissions-and-permission-modes">
  샌드박싱이 권한 및 권한 모드와 어떻게 관련되는지
</h2>

샌드박싱, [권한 규칙](/docs/ko/permissions) 및 [권한 모드](/docs/ko/permission-modes)는 상호 보완적인 계층입니다. 아래 섹션에서는 샌드박스가 각각과 어떻게 상호 작용하는지 다룹니다.

<h3 id="permission-rules">
  권한 규칙
</h3>

권한 규칙과 샌드박싱은 다른 것을 제어합니다:

* **권한 규칙**은 Claude Code가 사용할 수 있는 도구를 제어하며 도구가 실행되기 전에 평가됩니다. 모든 도구에 적용됩니다: Bash, Read, Edit, WebFetch, MCP 등.
* **샌드박싱**은 Bash 명령이 파일시스템 및 네트워크 수준에서 액세스할 수 있는 것을 제한하는 OS 수준의 적용을 제공합니다. Bash 명령 및 그 자식 프로세스에만 적용됩니다.

두 계층은 또한 적용 방식이 다릅니다. Claude Code는 명령 문자열을 기반으로 명령이 실행되기 전에 권한 결정을 평가하고, 자동 모드에서는 명령이 안전한지 여부에 대한 별도 분류기의 판단을 평가합니다. 운영 체제는 실행 중인 프로세스에 샌드박스 경계를 적용하므로 모델이 실행하도록 선택한 것과 관계없이 그리고 허용된 명령이 이름이 나타내는 것보다 더 많이 하더라도 유지됩니다.

파일시스템 및 네트워크 제한은 샌드박스 설정 및 권한 규칙을 통해 모두 구성됩니다:

| 설정 또는 규칙                                                       | 수행 작업                                                            |
| :------------------------------------------------------------- | :--------------------------------------------------------------- |
| `sandbox.filesystem.allowWrite`                                | 작업 디렉토리 외부의 경로에 대한 하위 프로세스 쓰기 액세스를 부여합니다                         |
| `sandbox.filesystem.denyWrite` 및 `sandbox.filesystem.denyRead` | 특정 경로에 대한 하위 프로세스 액세스를 차단합니다                                     |
| `sandbox.filesystem.allowRead`                                 | `denyRead` 영역 내의 특정 경로 읽기를 다시 허용합니다                              |
| `Edit` 허용 규칙                                                   | 특정 경로에 대한 쓰기 액세스를 부여합니다. `sandbox.filesystem.allowWrite`와 동일한 방식 |
| `Read` 및 `Edit` 거부 규칙                                          | 특정 파일 또는 디렉토리에 대한 액세스를 차단합니다                                     |
| `WebFetch` 허용 및 거부 규칙                                          | 도메인 액세스를 제어합니다                                                   |
| 샌드박스 `allowedDomains`                                          | Bash 명령이 도달할 수 있는 도메인을 제어합니다                                     |
| 샌드박스 `deniedDomains`                                           | 더 광범위한 `allowedDomains` 와일드카드가 허용할 수 있는 경우에도 특정 도메인을 차단합니다       |

`sandbox.filesystem` 설정 및 권한 규칙의 경로는 최종 샌드박스 구성으로 병합됩니다.

[claude-code 저장소의 예제 디렉토리](https://github.com/anthropics/claude-code/tree/main/examples/settings)에는 샌드박스 관련 예제를 포함한 일반적인 배포 시나리오에 대한 시작 설정 구성이 포함되어 있습니다. 이를 시작점으로 사용하고 필요에 맞게 조정합니다.

<h3 id="permission-modes">
  권한 모드
</h3>

`/sandbox`는 [권한 모드](/docs/ko/permission-modes)가 아닙니다. 권한 모드는 도구 호출이 실행되는지 여부와 먼저 프롬프트되는지 여부를 결정하는 반면, 샌드박스는 Bash 명령이 실행되면 액세스할 수 있는 것을 제한합니다. 제어하는 것과 작업별 프롬프트를 대체하는 것이 다릅니다:

|                                                                | 제어하는 것                    | 프롬프트를 대체하는 것                                                                                                                                                                                                                                                                                                        |
| :------------------------------------------------------------- | :------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `/sandbox`                                                     | Bash 명령이 실행되면 액세스할 수 있는 것 | [자동 허용 모드](#sandbox-modes)의 샌드박스 경계 자체                                                                                                                                                                                                                                                                              |
| [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode) | 각 도구 호출이 실행되는지 여부         | 작업을 검토하는 분류기                                                                                                                                                                                                                                                                                                        |
| `--dangerously-skip-permissions`                               | 각 도구 호출이 실행되는지 여부         | 없음. [보호된 경로](/docs/ko/permission-modes#protected-paths) 확인도 건너뜁니다. 명시적 [요청 규칙](/docs/ko/permissions#manage-permissions), 커넥터 도구 [조직에서 `ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools), [`requiresUserInteraction`](/docs/ko/mcp#require-approval-for-a-specific-tool)으로 표시된 MCP 도구, 그리고 `/` 또는 홈 디렉토리 제거만 여전히 프롬프트합니다 |

샌드박스의 [자동 허용 모드](#sandbox-modes)는 [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)와 별개입니다: 자동 허용은 샌드박스 경계가 이를 포함하기 때문에 Bash 명령을 승인하는 반면, 자동 모드는 분류기를 사용하여 작업을 검토합니다. 두 가지는 독립적으로 작동하며 결합할 수 있습니다. 무인 실행을 위한 격리 경계를 선택하려면 [샌드박스 환경](/docs/ko/sandbox-environments#how-isolation-relates-to-permission-modes)을 참조하세요.

<h2 id="configure-the-sandbox-for-your-organization">
  조직을 위해 샌드박스 구성
</h2>

관리자는 모든 사용자에게 샌드박싱을 요구하고, 개발자가 정책을 확대하는 것을 방지하고, 샌드박스 트래픽을 회사 프록시를 통해 라우팅할 수 있습니다.

<h3 id="enforce-sandboxing-with-managed-settings">
  관리 설정으로 샌드박싱 적용
</h3>

모든 개발자에게 샌드박스를 요구하려면 [관리 설정](/docs/ko/settings#settings-files)을 통해 `sandbox` 키를 제공합니다. MDM으로 관리되는 파일 또는 Claude.ai의 [서버 관리 설정](/docs/ko/server-managed-settings)을 통해 제공합니다.

다음 관리 설정 구성은 샌드박스를 활성화하고, 샌드박스를 초기화할 수 없으면 Claude Code 시작을 거부하고, 모델이 샌드박스 외부에서 명령을 다시 시도하는 것을 방지합니다:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "failIfUnavailable": true,
    "allowUnsandboxedCommands": false
  }
}
```

`enabled` 이상의 두 키는 샌드박스가 명령을 실행할 수 없을 때 발생하는 일을 제어합니다:

* **`failIfUnavailable`**: Linux의 bubblewrap과 같은 누락된 종속성이 경고를 표시하고 샌드박싱 없는 실행으로 폴백하는 대신 Claude Code가 시작되는 것을 차단합니다
* **`allowUnsandboxedCommands: false`**: `dangerouslyDisableSandbox` 탈출 해치가 무시되므로 샌드박스에서 실패한 명령을 샌드박스 외부에서 다시 시도할 수 없습니다

함께 고려할 가치가 있는 두 가지 추가 사항이 있습니다. 격리 없이 실행해야 하는 조직 승인 도구에 대해 `excludedCommands`를 추가합니다. `~/.aws` 및 `~/.ssh`와 같은 자격 증명 디렉토리에 대해 [`sandbox.credentials`](#protect-credentials) 항목을 추가합니다. 기본 읽기 정책은 여전히 이를 허용합니다.

샌드박스는 기본 Windows에서 실행되지 않으므로 플릿에 Windows 호스트가 포함되면 이 구성을 macOS 및 Linux로 범위를 지정하거나 해당 사용자가 WSL2 또는 컨테이너 내에서 Claude Code를 실행하도록 합니다.

<h3 id="keep-developers-from-widening-the-policy">
  개발자가 정책을 확대하는 것을 방지
</h3>

`enabled` 및 `failIfUnavailable`과 같은 부울 키의 경우 Claude Code는 관리 값을 사용하고 개발자가 로컬로 설정한 모든 것을 무시합니다. `excludedCommands` 및 `allowRead`와 같은 배열 키의 경우 Claude Code는 모든 범위의 항목을 병합하므로 개발자는 정책을 확대하는 항목을 추가할 수 있습니다.

관리 설정에서 `allowManagedReadPathsOnly`를 `true`로 설정하여 관리 설정의 `allowRead` 항목만 존중되도록 합니다. 사용자, 프로젝트 및 로컬 `allowRead` 항목은 무시됩니다. 이는 개발자가 조직 승인 경로 이상으로 읽기 액세스를 확대하는 것을 방지합니다. 네트워크 도메인을 동일한 방식으로 관리 값으로 잠그려면 [`allowManagedDomainsOnly`](/docs/ko/settings#sandbox-settings)를 설정합니다.

`excludedCommands`는 동등한 관리 전용 잠금이 없으므로 개발자는 항상 샌드박스 외부에서 실행되는 추가 명령을 추가하는 항목을 추가할 수 있습니다. 관리 목록을 좁게 유지합니다.

<h3 id="custom-proxy-configuration">
  사용자 정의 프록시 구성
</h3>

고급 네트워크 보안이 필요한 조직의 경우 사용자 정의 프록시를 구현하여 다음을 수행할 수 있습니다:

* HTTPS 트래픽 복호화 및 검사
* 사용자 정의 필터링 규칙 적용
* 모든 네트워크 요청 로깅
* 기존 보안 인프라와 통합

Claude Code를 프록시로 지정하려면 [샌드박스 설정](/docs/ko/settings#sandbox-settings)에서 프록시 포트를 설정합니다:

```json theme={null}
{
  "sandbox": {
    "network": {
      "httpProxyPort": 8080,
      "socksProxyPort": 8081
    }
  }
}
```

<h2 id="troubleshooting">
  문제 해결
</h2>

일부 명령은 샌드박스 내에서 실패하지만 외부에서는 작동합니다. 아래 수정 사항은 가장 일반적인 경우를 다룹니다.

* **명령이 host-not-allowed 오류로 실패**: 많은 CLI 도구는 특정 호스트에 도달해야 합니다. 프롬프트될 때 권한을 부여하면 호스트가 허용 목록에 추가되므로 도구가 향후 샌드박스 내에서 실행됩니다.
* **`jest`가 중단되거나 실패**: `watchman`은 샌드박스와 호환되지 않습니다. 대신 `jest --no-watchman`을 실행합니다.
* **Go 기반 CLI가 macOS에서 TLS 검증 실패**: `gh`, `gcloud`, `terraform`과 같은 도구는 Seatbelt에서 TLS 검증에 실패할 수 있습니다. 이러한 도구를 `excludedCommands`에 나열하여 샌드박스 외부에서 실행합니다. MITM 프록시 및 사용자 정의 CA와 함께 `httpProxyPort`를 사용하는 경우 대신 [`enableWeakerNetworkIsolation`](/docs/ko/settings#sandbox-settings)을 `true`로 설정합니다.
* **`open`, `osascript`, 또는 브라우저 기반 인증 흐름이 macOS에서 오류 `-600`으로 실패**: 샌드박스는 기본적으로 Apple Events를 차단합니다. 사용자, 관리 또는 CLI 설정에서 [`allowAppleEvents`](/docs/ko/settings#sandbox-settings)를 `true`로 설정하여 이를 허용합니다. 프로젝트 설정은 이 키에 대해 무시됩니다. 이를 활성화하면 샌드박싱된 명령이 사용자 프롬프트 없이 다른 애플리케이션을 비샌드박싱된 상태로 시작할 수 있고 실행 중인 애플리케이션에 AppleScript 명령을 보낼 수 있으므로 코드 실행 격리가 제거됩니다. 이는 macOS 자동화 동의 프롬프트(TCC)의 적용을 받습니다. 또는 명령을 `excludedCommands`에 추가하여 샌드박스 외부에서 실행합니다.
* **`docker` 명령 실패**: `docker`는 샌드박스와 호환되지 않습니다. `docker *`를 `excludedCommands`에 추가하여 샌드박스 외부에서 실행합니다.
* **컨테이너 내에서 Bubblewrap 시작 실패**: 권한 없는 컨테이너에서 bubblewrap은 새로운 `/proc` 파일시스템을 마운트할 수 없습니다. [`enableWeakerNestedSandbox`](/docs/ko/settings#sandbox-settings)를 `true`로 설정하여 내부 샌드박스가 컨테이너의 기존 `/proc`을 바인드 마운트하도록 합니다. 외부 컨테이너가 이미 필요한 격리 경계를 제공할 때만 이 설정을 사용합니다. 새로운 `/proc` 마운트가 숨길 프로세스 정보를 샌드박싱된 명령에 노출하기 때문입니다.
* **Linux의 Seccomp 필터**: seccomp 필터는 Unix 도메인 소켓을 차단하는 데 필요합니다. `/sandbox`의 Dependencies 탭은 사용 가능한지 보여줍니다. 누락된 경우 `npm install -g @anthropic-ai/sandbox-runtime`을 실행하여 도우미를 설치합니다.
* **`--dangerously-skip-permissions`이 root로 실패**: 이 플래그는 Linux 및 macOS에서 root로 또는 sudo를 통해 실행할 때 차단됩니다. root 액세스와 권한 프롬프트 없음이 결합되면 시스템의 모든 파일 또는 서비스를 수정할 수 있기 때문입니다. 확인은 인식된 샌드박스 내에서 자동으로 건너뜁니다. 컨테이너에서 자율적으로 실행하려면 [dev 컨테이너](/docs/ko/devcontainer) 구성을 사용합니다. 이는 Claude Code를 비 root 사용자로 실행합니다.

<h2 id="limitations">
  제한 사항
</h2>

샌드박싱은 위험을 줄이지만 완전한 격리 경계는 아닙니다. 이를 하드 보안 제어로 사용하기 전에 아래 제한 사항을 검토합니다.

<h3 id="security-limitations">
  보안 제한 사항
</h3>

* **네트워크 필터링**: 샌드박스는 프로세스가 연결할 수 있는 도메인을 제한합니다. 기본 제공 프록시는 아웃바운드 트래픽을 종료하거나 TLS를 검사하지 않으므로 암호화된 연결의 내용은 검사되지 않습니다. 실험적인 [`network.tlsTerminate`](/docs/ko/settings#sandbox-settings) 설정은 [`mask` 자격 증명 대체](#protect-credentials)를 위해 프록시에서 TLS를 종료하지만 콘텐츠 필터링을 추가하지 않습니다. 정책에서 신뢰할 수 있는 도메인만 허용하도록 보장하는 것은 사용자의 책임입니다.

<Warning>
  `github.com`과 같은 광범위한 도메인을 허용하면 데이터 유출 경로가 생성될 수 있습니다. 프록시가 TLS를 검사하지 않고 클라이언트 제공 호스트 이름에서 허용 결정을 내리기 때문에 샌드박스 내에서 실행되는 코드는 잠재적으로 [도메인 프론팅](https://en.wikipedia.org/wiki/Domain_fronting) 또는 유사한 기술을 사용하여 허용 목록 외부의 호스트에 도달할 수 있습니다. 위협 모델이 더 강력한 보장을 요구하면 TLS를 종료하고 트래픽을 검사하는 [사용자 정의 프록시](#custom-proxy-configuration)를 구성하고 그 CA 인증서를 샌드박스 내에 설치합니다. 더 강력한 TLS 인식 네트워크 격리는 활발한 개발 영역입니다.
</Warning>

* **Unix 소켓을 통한 권한 상승**: `allowUnixSockets` 구성은 실수로 샌드박스 우회로 이어질 수 있는 강력한 시스템 서비스에 대한 액세스를 부여할 수 있습니다. 예를 들어 `/var/run/docker.sock`에 대한 액세스를 허용하면 Docker 소켓을 통해 호스트 시스템에 대한 액세스를 효과적으로 부여합니다. 샌드박스를 통해 허용하는 모든 Unix 소켓을 신중하게 고려합니다.
* **파일시스템 권한 상승**: 과도하게 광범위한 파일시스템 쓰기 권한은 권한 상승 공격을 가능하게 할 수 있습니다. `$PATH`의 실행 파일을 포함하는 디렉토리, 시스템 구성 디렉토리 또는 `.bashrc` 또는 `.zshrc`와 같은 사용자 셸 구성 파일에 대한 쓰기를 허용하면 다른 사용자 또는 시스템 프로세스가 이러한 파일에 액세스할 때 다른 보안 컨텍스트에서 코드 실행으로 이어질 수 있습니다.
* **Linux 샌드박스 강도**: Linux 구현은 강력한 파일시스템 및 네트워크 격리를 제공하지만 권한 있는 네임스페이스 없이 Docker 환경 내에서 작동할 수 있도록 하는 `enableWeakerNestedSandbox` 모드를 포함합니다. 또는 권한 없는 사용자 네임스페이스가 sysctl에 의해 비활성화된 Linux 호스트에서. 이 옵션은 보안을 상당히 약화시키며 추가 격리가 다른 방식으로 적용되는 경우에만 사용해야 합니다.
* **macOS의 Apple Events**: macOS 샌드박스는 기본적으로 Apple Events를 차단합니다. `allowAppleEvents` 설정은 이 제한을 해제하여 `open` 및 `osascript`와 같은 도구가 작동하지만 코드 실행 격리를 제거합니다. 샌드박싱된 명령은 사용자 프롬프트 없이 다른 애플리케이션을 샌드박싱되지 않은 상태로 시작할 수 있으며 실행 중인 애플리케이션에 AppleScript 명령을 보낼 수 있습니다. 이는 앱별 macOS 자동화 동의 프롬프트(TCC)의 적용을 받습니다. 이는 사용자, 관리 또는 CLI 설정에서만 적용됩니다. 프로젝트 설정은 이를 활성화할 수 없습니다.
* **설정 파일 보호**: 샌드박스는 모든 범위의 Claude Code `settings.json` 파일 및 관리 설정 디렉토리에 대한 쓰기 액세스를 자동으로 거부하므로 샌드박싱된 명령은 자신의 정책을 수정할 수 없습니다.

<h3 id="platform-and-tool-compatibility">
  플랫폼 및 도구 호환성
</h3>

* **플랫폼 지원**: macOS, Linux 및 WSL2를 지원합니다. WSL1 및 기본 Windows는 지원되지 않습니다.
* **성능 오버헤드**: 최소이지만 일부 파일시스템 작업이 약간 더 느릴 수 있습니다.
* **도구 호환성**: 특정 시스템 액세스 패턴이 필요한 일부 도구는 구성 조정이 필요할 수 있으며 샌드박스 외부에서 실행해야 할 수도 있습니다.

<h3 id="scope">
  범위
</h3>

샌드박스는 Bash 하위 프로세스를 격리합니다. 다른 도구는 다른 경계에서 작동합니다:

* **기본 제공 파일 도구**: Read, Edit 및 Write는 샌드박스를 통해 실행되지 않고 권한 시스템을 직접 사용합니다. [권한](/docs/ko/permissions)을 참조하세요.
* **컴퓨터 사용**: Claude가 앱을 열고 화면을 제어할 때 격리된 환경이 아닌 실제 데스크톱에서 실행됩니다. 앱별 권한 프롬프트가 각 애플리케이션을 제어합니다. [CLI의 컴퓨터 사용](/docs/ko/computer-use) 또는 [Desktop의 컴퓨터 사용](/docs/ko/desktop#let-claude-use-your-computer)을 참조하세요.
* **환경 변수**: 샌드박싱된 Bash 명령은 기본적으로 부모 프로세스 환경을 상속합니다. 여기에는 설정된 모든 자격 증명이 포함됩니다. [`sandbox.credentials`](#protect-credentials)를 사용하여 샌드박싱된 명령에 대한 특정 변수를 설정 해제하거나 [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/ko/env-vars)를 설정하여 모든 하위 프로세스에서 Anthropic 및 클라우드 공급자 자격 증명을 제거합니다.
* **하위 에이전트**: [하위 에이전트](/docs/ko/sub-agents)는 부모 세션과 동일한 프로세스에서 실행되며 동일한 샌드박스 구성을 사용합니다. 부모 세션에서 샌드박싱이 활성화되면 하위 에이전트 내의 Bash 명령이 샌드박싱됩니다.

<Warning>
  효과적인 샌드박싱은 파일시스템 및 네트워크 격리 모두를 필요로 합니다. 네트워크 격리가 없으면 손상된 에이전트가 SSH 키와 같은 민감한 파일을 유출할 수 있습니다. 파일시스템 격리가 없으면 손상된 에이전트가 시스템 리소스를 백도어하여 네트워크 액세스를 얻을 수 있습니다. 기본값을 확대할 때 `allowWrite` 경로, 광범위한 `allowedDomains` 항목 또는 `excludedCommands` 예외가 다른 쪽의 제한을 취소하지 않는지 확인합니다.
</Warning>

<h2 id="see-also">
  참고 항목
</h2>

* [샌드박스 환경](/docs/ko/sandbox-environments): 기본 제공 샌드박스를 dev 컨테이너, 컨테이너 및 VM과 비교
* [보안](/docs/ko/security): 포괄적인 보안 기능 및 모범 사례
* [권한](/docs/ko/permissions): 권한 구성 및 액세스 제어
* [설정](/docs/ko/settings): 전체 구성 참조
* [CLI 참조](/docs/ko/cli-reference): 명령줄 옵션
