> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 샌드박싱된 Bash 도구 구성

> Claude Code의 샌드박싱된 Bash 도구가 파일시스템 및 네트워크 격리를 제공하여 더 안전하고 자율적인 에이전트 실행을 가능하게 하는 방법을 알아봅니다.

Bash 샌드박스를 사용하면 Claude가 대부분의 셸 명령을 권한을 요청하지 않고 실행할 수 있습니다. 각 명령을 승인하는 대신 명령이 접근할 수 있는 파일과 네트워크 도메인을 정의하면 운영 체제가 모든 Bash, PowerShell 또는 Monitor 명령과 그 자식 프로세스에 대해 해당 경계를 적용합니다.

<Note>
  dev 컨테이너, 사용자 정의 컨테이너, 가상 머신 등 다른 격리 방식을 비교하려면 [샌드박스 환경](/docs/ko/sandbox-environments)을 참조하세요. Bash 이외의 도구에 대한 권한 프롬프트를 줄이려면 [권한 모드](/docs/ko/permission-modes)를 참조하세요.
</Note>

<h2 id="get-started">
  시작하기
</h2>

샌드박스는 Claude Code에 내장되어 있으며 macOS, Linux 및 WSL2에서 실행됩니다. 기본 Windows는 지원되지 않습니다. Windows에서는 WSL2 배포판 내에서 Claude Code를 실행하세요.

macOS에서는 설치할 것이 없습니다. 샌드박싱은 기본 제공되는 Seatbelt 프레임워크를 사용합니다. Linux 및 WSL2에서 샌드박스는 두 개의 패키지에 의존하며, 이는 [Linux 및 WSL2 설정](#set-up-linux-and-wsl2)에서 다룹니다. 아직 설치하지 않았더라도 `/sandbox`로 시작할 수 있습니다. 패널에 누락된 항목이 있는지 표시되기 때문입니다.

<Steps>
  <Step title="/sandbox 실행">
    Claude Code 세션을 시작하고 `/sandbox` 명령을 실행하세요:

    ```text theme={null}
    /sandbox
    ```

    이렇게 하면 세 개의 탭이 있는 샌드박스 패널이 열리며, Linux에서 선택적 seccomp 필터가 누락된 경우 Dependencies 탭이 추가됩니다:

    * **Mode**: 샌드박스된 명령이 승인되는 방식을 선택합니다. 다음 단계에서 다룹니다.
    * **Overrides**: 샌드박스에서 실패한 명령이 샌드박스 없이 실행되도록 폴백할 수 있는지 선택합니다. 이것은 [`allowUnsandboxedCommands`](/docs/ko/settings-reference#sandbox-allowunsandboxedcommands) 설정입니다.
    * **Config**: 해결된 샌드박스 설정을 봅니다.

    패널에 Dependencies 탭만 표시되면 필수 패키지가 누락된 것입니다. [Linux 및 WSL2 설정](#set-up-linux-and-wsl2)에 설명된 대로 설치하고, Claude Code를 다시 시작한 후 `/sandbox`를 다시 실행하세요.
  </Step>

  <Step title="모드 선택">
    Mode 탭에서 자동 허용 또는 일반 권한을 선택하세요. 자동 허용은 샌드박스된 명령을 프롬프트 없이 실행하고, 일반 권한은 명령이 샌드박스되었을 때도 일반 권한 프롬프트를 유지합니다. 자동 허용 모드에서 어떤 명령이 여전히 프롬프트되는지는 [샌드박스 모드](#sandbox-modes)를 참조하세요.
  </Step>

  <Step title="Bash 명령 실행">
    Claude에게 빌드 또는 테스트 스위트와 같은 명령을 실행하도록 요청하세요. 기본적으로 샌드박스 내의 명령은 작업 디렉토리, 세션 임시 디렉토리 및 `--add-dir`, `/add-dir` 또는 `permissions.additionalDirectories`로 [추가한 모든 디렉토리](/docs/ko/permissions#additional-directories-grant-file-access-not-configuration)에 쓸 수 있습니다.

    명령이 처음으로 새 네트워크 도메인이 필요할 때 Claude Code는 승인을 요청합니다. [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)에서는 Claude가 대신 명령이 필요한 호스트를 [명령 자체에](#per-command-allowed-domains-in-auto-mode) 이름 지어 분류기가 함께 검토하도록 합니다.

    샌드박스에서 실행할 수 없는 명령은 일반 권한 흐름으로 폴백합니다. Claude Code는 권한 프롬프트를 "Bash command"가 아닌 "Bash command (unsandboxed)"로 제목을 지정하므로 어떤 명령이 샌드박스 외부에서 실행되었는지 알 수 있습니다. 샌드박스가 허용하는 범위를 넓히거나 좁히려면 [샌드박싱 구성](#configure-sandboxing)을 참조하세요.

    샌드박스된 명령이 컨테이너 내에서 `Operation not permitted`로 실패하면 [문제 해결](#troubleshooting)의 Bubblewrap 항목을 참조하세요.
  </Step>
</Steps>

패널에서 모드를 선택하면 Claude Code는 이를 프로젝트의 로컬 설정인 `.claude/settings.local.json`에 저장하며, 이는 현재 프로젝트에 적용됩니다. Claude Code는 설정을 저장할 때 해당 파일을 전역 gitignore에 추가합니다. 모든 프로젝트에서 샌드박스를 활성화하려면 사용자 설정인 `~/.claude/settings.json`에서 [`sandbox.enabled`](/docs/ko/settings-reference#sandbox-enabled)를 `true`로 설정하세요. 조직의 모든 개발자에게 샌드박싱을 적용하려면 [관리 설정](#enforce-sandboxing-with-managed-settings)을 사용하세요.

한 세션 동안 설정 파일에 쓰지 않고 샌드박스를 변경하려면 [`--settings`](/docs/ko/settings#change-a-setting-for-one-session)로 Claude Code를 시작하세요. 예를 들어 이 명령은 Claude가 차단된 명령을 샌드박스 외부에서 재시도할 수 없는 샌드박스된 세션을 시작합니다:

```bash theme={null}
claude --settings '{"sandbox": {"enabled": true, "allowUnsandboxedCommands": false}}'
```

<Warning>
  기본적으로 종속성이 누락되었거나 플랫폼이 지원되지 않아 샌드박스를 시작할 수 없으면 Claude Code는 경고를 표시하고 샌드박싱 없이 명령을 실행합니다. 이를 하드 실패로 만들려면 [`sandbox.failIfUnavailable`](/docs/ko/settings-reference#sandbox-failifunavailable)을 `true`로 설정하세요. 이는 샌드박싱을 보안 게이트로 요구하는 관리 배포를 위한 것입니다.
</Warning>

<h3 id="set-up-linux-and-wsl2">
  Linux 및 WSL2 설정
</h3>

Linux 및 WSL2에서 샌드박스는 두 개의 패키지에 의존합니다:

* [`bubblewrap`](https://github.com/containers/bubblewrap): 파일 시스템 격리를 적용하는 권한 없는 샌드박싱 도구
* [`socat`](http://www.dest-unreach.org/socat/): 샌드박스 프록시를 통해 네트워크 트래픽을 라우팅하는 데 사용되는 릴레이

배포판의 패키지 관리자로 설치하세요:

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

종속성이 누락되면 `/sandbox`의 Dependencies 탭에 플랫폼이 부족한 `ripgrep`, `bubblewrap`, `socat` 및 seccomp 필터가 나열됩니다. Claude Code를 설치하고 다시 시작한 후 탭이 보이지 않으면 모든 종속성이 있는 것입니다.

Ripgrep은 기본 Claude Code 바이너리와 함께 번들로 제공됩니다. seccomp 필터는 선택 사항이며 Unix 도메인 소켓 차단을 추가합니다. 누락된 경우 `npm install -g @anthropic-ai/sandbox-runtime`으로 설치하세요.

필수 종속성이 누락되면 Dependencies 탭이 설치될 때까지 유일한 탭으로 표시됩니다. 선택적 seccomp 필터만 누락되면 Dependencies 탭이 다른 탭과 함께 나타납니다. 종속성 확인은 시작 시 실행되므로 패키지를 설치한 후 Claude Code를 다시 시작하여 `/sandbox`가 이를 감지하도록 하세요.

<AccordionGroup>
  <Accordion title="Ubuntu 24.04 이상: bubblewrap이 사용자 네임스페이스를 생성하도록 허용">
    Ubuntu 24.04 이상에서는 기본 AppArmor 정책이 bubblewrap이 격리에 필요한 사용자 네임스페이스를 생성하는 것을 방지합니다.

    WSL2 내부를 포함하여 환경이 이 제한을 적용하는지 확인하려면 `sysctl kernel.apparmor_restrict_unprivileged_userns`를 실행하세요. 명령이 `0`을 반환하면 이 단계를 건너뛰세요. `No such file or directory` 오류를 출력하면 키가 없으므로 이 단계를 건너뛸 수 있습니다. `1`을 반환하면 `bwrap`에 이 기능을 부여하는 AppArmor 프로필을 추가하세요:

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

    프로필은 `bwrap` 자체에만 적용되며 샌드박스 내에서 실행되는 명령에는 적용되지 않습니다. AppArmor를 다시 로드하여 적용하세요:

    ```bash theme={null}
    sudo systemctl reload apparmor
    ```
  </Accordion>

  <Accordion title="WSL2 참고">
    PowerShell에서 `wsl -l -v`로 WSL 버전을 확인하세요. `Sandboxing requires WSL2`가 표시되면 배포판이 WSL1을 실행 중입니다. WSL2로 업그레이드하거나 샌드박싱 없이 Claude Code를 실행하세요.

    WSL2에서 WSL은 `cmd.exe`, `powershell.exe` 또는 `/mnt/c/` 아래의 모든 항목과 같은 Windows 바이너리 실행을 Unix 소켓을 통해 Windows 호스트에 전달하므로 샌드박스된 명령이 하나를 실행할 수 있는지 여부는 샌드박스의 [Unix 소켓 설정](/docs/ko/settings-reference#sandbox-network-allowunixsockets)을 따릅니다. 선택적 seccomp 필터를 설치하여 먼저 소켓을 차단해야 합니다. 이러한 실행을 허용하려면 `allowAllUnixSockets`를 설정하세요. 샌드박스에서 완전히 제외하려면 명령을 [`excludedCommands`](/docs/ko/settings-reference#sandbox-excludedcommands)에 추가하세요.
  </Accordion>
</AccordionGroup>

<h3 id="sandbox-modes">
  샌드박스 모드
</h3>

Claude Code는 두 가지 샌드박스 모드를 제공합니다. 둘 다에서 샌드박스는 동일한 파일 시스템 및 네트워크 제한을 적용합니다. 차이점은 샌드박스된 명령이 자동으로 승인되는지 또는 명시적 권한이 필요한지 여부뿐입니다.

<h4 id="auto-allow-mode">
  자동 허용 모드
</h4>

명령을 샌드박스할 수 있으면 Claude Code는 이를 샌드박스 내에서 실행하고 권한을 요청하지 않고 자동으로 승인합니다. 허용되지 않은 호스트에 대한 네트워크 액세스가 필요한 명령과 같이 샌드박스할 수 없는 명령은 일반 권한 흐름으로 폴백하며, Claude Code는 [권한 규칙](/docs/ko/permissions)을 확인하고 해당 규칙이 이미 허용하지 않는 모든 명령을 수동 모드에서 프롬프트로 제어합니다.

자동 허용 모드에서도 다음이 적용됩니다:

* 명시적 [거부 규칙](/docs/ko/permissions)은 항상 존중됩니다.
* [중요 경로](/docs/ko/permission-modes#critical-paths)를 대상으로 하는 `rm` 또는 `rmdir` 명령은 여전히 일반 권한 흐름을 거칩니다.
* `Bash(git push *)`와 같은 콘텐츠 범위 [요청 규칙](/docs/ko/permissions)은 샌드박스된 명령에 대해서도 프롬프트를 강제합니다.
* 기본 `Bash` 요청 규칙 또는 동등한 `Bash(*)` 형식은 샌드박스에서 실행되는 명령에 대해 건너뜁니다. 일반 권한 흐름으로 폴백하는 명령에는 여전히 적용됩니다. [계획 모드](/docs/ko/permission-modes#analyze-before-you-edit-with-plan-mode)에서는 규칙이 건너뛰어지지 않습니다. 읽기 전용 명령을 포함하여 샌드박스된 명령에 대해 프롬프트됩니다. v2.1.212 이전에는 건너뛰기가 계획 모드에도 적용되었습니다.

<Info>
  자동 허용 모드는 [계획 모드](/docs/ko/permission-modes#analyze-before-you-edit-with-plan-mode)를 제외하고 권한 모드 설정과 독립적으로 작동하며, 자동 모드에서는 [명령별 허용 도메인](#per-command-allowed-domains-in-auto-mode)을 수행하는 명령에 대해 작동하고, [서버 측 분류기 검토](/docs/ko/permission-modes#how-the-classifier-evaluates-actions)는 자동 모드에서 샌드박스된 명령에 대해 작동합니다. "수정 사항 수락" 모드가 아니더라도 자동 허용이 활성화되면 샌드박스된 Bash 명령이 자동으로 실행됩니다. 이는 샌드박스 경계 내에서 파일을 수정하는 Bash 명령이 수동 모드에서도 프롬프트 없이 실행됨을 의미합니다. 파일 편집 도구는 프롬프트됩니다.

  계획 모드에서는 자동 허용이 승인을 넓히지 않습니다. Claude Code가 계획하는 동안 명령을 제어하는 방법은 [계획 모드](/docs/ko/permission-modes#analyze-before-you-edit-with-plan-mode)를 참조하세요. v2.1.212 이전에는 자동 허용이 계획 모드에서도 프롬프트 없이 샌드박스된 명령을 실행했습니다.
</Info>

<h4 id="regular-permissions-mode">
  일반 권한 모드
</h4>

모든 Bash 명령은 샌드박스되었을 때도 일반 권한 흐름을 거칩니다. 이는 더 많은 제어를 제공하지만 더 많은 승인이 필요합니다.

<h4 id="the-unsandboxed-retry-escape-hatch">
  샌드박스 해제 재시도 이스케이프 해치
</h4>

일부 명령은 샌드박스 내에서 전혀 실행할 수 없습니다. 예를 들어 호환되지 않는 도구나 허용하지 않은 호스트가 필요한 도구입니다. Claude Code는 차단된 명령의 결과에서 샌드박스 위반을 보고하며, 샌드박스가 거부한 경로 또는 호스트의 이름을 지정하므로 Claude는 샌드박스가 차단한 것을 봅니다. 작업을 실패하거나 샌드박싱을 끄도록 요구하는 대신 Claude Code는 이스케이프 해치를 포함합니다. Claude는 위반을 분석하고 `dangerouslyDisableSandbox` 매개변수로 명령을 재시도할 수 있습니다.

재시도된 명령은 샌드박스 외부에서 실행되므로 일반 권한 흐름을 거칩니다. 수동 모드에서는 확인 프롬프트를 받습니다. [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)에서는 분류기가 기본 명령을 평가합니다. [`permissions.blockReadsOutsideWorkingDirectories`](/docs/ko/settings-reference#permissions-blockreadsoutsideworkingdirectories)가 켜져 있는 동안 샌드박스 외부에서 실행하기 위해 승인이 필요한 재시도는 대신 프롬프트됩니다. 자동 모드에서도 샌드박스 해제된 모든 재시도에 대해 프롬프트되도록 하려면 `Bash(dangerouslyDisableSandbox:true)`에 대한 [요청 규칙](/docs/ko/permissions#match-by-input-parameter)을 추가하세요.

[샌드박스 설정](/docs/ko/settings-reference#sandbox-settings)에서 `"allowUnsandboxedCommands": false`를 설정하여 이 이스케이프 해치를 비활성화할 수 있습니다. 이스케이프 해치가 비활성화되면 Claude Code는 `dangerouslyDisableSandbox` 매개변수를 무시하고 Claude가 실행하는 모든 명령은 `excludedCommands`에 나열하지 않는 한 샌드박스에서 실행되어야 합니다. `/sandbox` **Overrides** 탭은 이 설정을 **Strict sandbox mode**로 표시합니다.

엄격한 샌드박스 모드는 Claude가 실행하는 명령에 적용됩니다. [`!` 셸 모드 프롬프트](/docs/ko/interactive-mode#shell-mode-with-prefix)에서 직접 입력하는 명령은 세션이 다음 중 하나가 아닌 한 샌드박스 외부에서 실행됩니다:

* **[백그라운드 세션](/docs/ko/agent-view)**: 엄격한 샌드박스 모드는 셸 모드 명령도 포함합니다.
* **[`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/ko/env-vars#variables)가 설정된 Linux 세션**: 모든 명령이 샌드박스되며, 셸 모드 명령 포함

v2.1.260 이전에는 엄격한 샌드박스 모드가 모든 세션에서 셸 모드 명령을 샌드박스했습니다.

<h4 id="temporary-directories">
  임시 디렉토리
</h4>

세션 임시 디렉토리는 기본적으로 작업 디렉토리와 함께 샌드박스 내에서 쓰기 가능합니다. [파일 시스템 격리를 비활성화](#disable-filesystem-isolation)하지 않는 한 Claude Code는 샌드박스된 명령에 대해 `$TMPDIR`을 이 디렉토리로 설정하므로 임시 파일을 쓰는 도구는 추가 구성 없이 작동합니다.

샌드박스 해제된 명령은 셸의 `$TMPDIR`을 변경하지 않고 상속하므로 파일 시스템 격리가 켜져 있는 동안 샌드박스된 명령과 샌드박스 해제된 명령은 `$TMPDIR`을 다른 디렉토리로 해결합니다. 셸이 `$TMPDIR`을 설정하지 않거나 비워두면 `$TMPDIR`을 참조하는 샌드박스 해제된 명령은 [`CLAUDE_CODE_TMPDIR`](/docs/ko/env-vars) 재정의를 받거나, 설정하지 않았거나 재정의가 긴 경로인 경우 운영 체제의 임시 디렉토리를 받으므로 변수가 빈 문자열로 확장되지 않습니다. 두 명령 간에 임시 파일을 전달하려면 대신 작업 디렉토리 아래에 작성하세요.

<h2 id="configure-sandboxing">
  샌드박싱 구성
</h2>

`settings.json` 파일을 통해 샌드박스 동작을 사용자 정의합니다. 전체 구성 참조는 [설정](/docs/ko/settings-reference#sandbox-settings)을 참조하세요.

기본적으로 샌드박싱된 명령은 현재 작업 디렉토리, 세션 임시 디렉토리, 그리고 `--add-dir`, `/add-dir` 또는 `permissions.additionalDirectories`로 [추가한 디렉토리](/docs/ko/permissions#additional-directories-grant-file-access-not-configuration)에 쓸 수 있습니다. `kubectl`, `terraform` 또는 `npm`과 같은 하위 프로세스 명령이 해당 디렉토리 외부에 쓰기해야 하면 `sandbox.filesystem.allowWrite`를 사용하여 특정 경로에 대한 액세스를 부여합니다:

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

동일한 파일시스템 배열이 여러 [설정 범위](/docs/ko/settings#settings-precedence)에서 정의되면 Claude Code는 이를 병합하여 모든 범위의 경로를 결합합니다.

CLI에서 [`--setting-sources`](/docs/ko/cli-reference)를 사용하거나 Agent SDK에서 [`settingSources`](/docs/ko/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources)를 사용하여 소스를 제외하면 Claude Code는 샌드박스 구성을 구축할 때 해당 소스의 `sandbox.filesystem` 항목, `Edit` 권한 규칙 및 `Read` 거부 규칙을 무시합니다. Claude Code v2.1.246 이상이 필요합니다.

세션 중에 이러한 파일시스템 목록을 편집하면 Claude Code는 [실행 중인 세션에 변경 사항을 적용](/docs/ko/settings#when-edits-take-effect)하므로 다음 샌드박싱된 명령은 새로운 경로에서 실행됩니다.

경로 접두사는 경로가 해석되는 방식을 제어합니다:

| 접두사            | 의미                                                       | 예시                                                                   |
| :------------- | :------------------------------------------------------- | :------------------------------------------------------------------- |
| `/`            | 파일시스템 루트의 절대 경로                                          | `/tmp/build`는 `/tmp/build`로 유지됩니다                                    |
| `~/`           | 홈 디렉토리에 상대적                                              | `~/.kube`는 `$HOME/.kube`가 됩니다                                        |
| `./` 또는 접두사 없음 | 프로젝트 설정의 경우 프로젝트 루트에 상대적이거나, 사용자 설정의 경우 `~/.claude`에 상대적 | `.claude/settings.json`의 `./output`은 `<project-root>/output`으로 해석됩니다 |

이 구문은 절대 경로에 `//path`를 사용하고 프로젝트 상대에 `/path`를 사용하는 [Read 및 Edit 권한 규칙](/docs/ko/permissions#read-and-edit)과 다릅니다. 샌드박스 파일시스템 경로는 표준 규칙을 사용합니다: `/tmp/build`는 절대 경로입니다. Claude Code가 이러한 경로의 후행 슬래시 또는 와일드카드를 어떻게 처리하는지는 [샌드박스 경로 접두사](/docs/ko/settings-reference#sandbox-path-prefixes)를 참조하세요.

`sandbox.filesystem.denyWrite` 및 `sandbox.filesystem.denyRead`를 사용하여 쓰기 또는 읽기 액세스를 거부할 수도 있으며, `sandbox.filesystem.allowRead`를 사용하여 거부된 영역 내에서 특정 경로 읽기를 다시 허용할 수 있습니다. 읽기 규칙이 겹칠 때 더 구체적인 경로가 우선합니다:

| 예시 규칙                                               | 결과                                                                                                                                              |
| :-------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------- |
| `"denyRead": ["~/"]`와 `"allowRead": ["~/projects"]` | `~/projects`는 읽을 수 있으며 홈 디렉토리의 나머지는 차단된 상태로 유지됩니다. 더 좁은 허용이 거부된 영역의 해당 부분을 다시 엽니다                                                               |
| `"allowRead": ["~/"]`와 `"denyRead": ["~/.env"]`     | `~/.env`는 차단된 상태로 유지되며 홈 디렉토리의 나머지는 읽을 수 있습니다. 거부는 더 넓은 허용 내에서 유지되므로 광범위한 허용이 비밀을 자동으로 다시 노출할 수 없습니다                                            |
| `"allowRead": ["~/"]`와 `"denyRead": ["~/**/.env"]`  | 홈 디렉토리 아래의 모든 `.env`는 차단된 상태로 유지되며 나머지는 읽을 수 있습니다. [와일드카드 거부](/docs/ko/settings-reference#sandbox-path-prefixes)는 더 넓은 허용 내에서 정확한 경로와 동일한 방식으로 유지됩니다 |

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

동일한 구성을 `~/.claude/settings.json`에 배치했다면 `.`은 `~/.claude`로 해석되고 프로젝트 파일은 `denyRead` 규칙에 의해 차단된 상태로 유지됩니다.

샌드박싱된 명령이 작업 디렉토리 외부의 홈 디렉토리 및 마운트된 볼륨에서 읽기 액세스를 거부하면서 작업 디렉토리는 읽을 수 있도록 유지하려면 경로 규칙을 작성하는 대신 [`permissions.blockReadsOutsideWorkingDirectories`](/docs/ko/settings-reference#permissions-blockreadsoutsideworkingdirectories)를 설정합니다.

<h3 id="disable-filesystem-isolation">
  파일시스템 격리 비활성화
</h3>

`sandbox.filesystem.disabled`를 `true`로 설정하여 네트워크 격리를 유지하면서 파일시스템 격리를 건너뜁니다. 아래 예제는 파일시스템 격리를 끄면서 네트워크 도메인의 허용 목록을 유지합니다:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "disabled": true
    },
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org"]
    }
  }
}
```

샌드박스에는 두 개의 독립적인 계층이 있습니다: [파일시스템 격리](#filesystem-isolation)는 샌드박싱된 명령이 읽고 쓸 수 있는 경로를 제어하고, [네트워크 격리](#network-isolation)는 도달할 수 있는 도메인을 제어합니다. 파일시스템 계층이 꺼져 있으면 샌드박싱된 명령은 호스트 파일시스템에 대한 무제한 읽기 및 쓰기 액세스를 얻지만 네트워크 송신은 허용된 도메인으로 제한됩니다. 명령이 쓰는 것이 아니라 연결하는 위치를 제어하기 위해 샌드박싱할 때 계층을 끕니다.

설정은 기본적으로 꺼져 있으며 샌드박스가 실행되는 플랫폼(macOS, Linux 및 WSL2)에 적용됩니다. Claude Code v2.1.216 이상이 필요합니다.

<Warning>
  파일시스템 격리가 꺼져 있고 명령이 자동으로 허용되면 샌드박싱된 명령은 나중에 명령이 실행하거나 읽는 파일(예: 셸 시작 파일, `$PATH`의 실행 파일 또는 `~/.claude/settings.json`)을 쓸 수 있으며 다음 실행에서 자신의 액세스를 확대하는 데 사용할 수 있습니다. `filesystem.disabled`를 `true`로 설정하는 것은 자신의 액세스를 확대하지 않도록 신뢰하는 워크로드에만 사용합니다. [`allowManagedDomainsOnly`](#keep-developers-from-widening-the-policy)로 네트워크 도메인을 잠그면 위험을 좁히지만 제거하지는 않습니다. 해당 잠금은 샌드박스 내에서 실행되는 명령에만 적용되기 때문입니다.
</Warning>

<h4 id="which-settings-can-disable-it">
  어떤 설정이 이를 비활성화할 수 있는지
</h4>

파일시스템 격리를 끄면 샌드박싱된 명령이 할 수 있는 작업이 확대되므로 Claude Code는 `filesystem.disabled`를 다음 설정 소스에서만 인정합니다:

* 사용자 설정, 관리 설정 및 `--settings` CLI 플래그는 이를 설정할 수 있습니다. `.claude/settings.json` 및 `.claude/settings.local.json`의 프로젝트 설정은 할 수 없으므로 체크아웃된 프로젝트는 파일시스템 격리를 끌 수 없습니다.
* 관리 설정이 `sandbox.filesystem`을 전혀 구성하거나 `"mode": "deny"`를 사용하는 `sandbox.credentials.files` 항목을 나열하면 관리 설정만 키를 설정할 수 있습니다. 이는 관리자가 배포한 파일시스템 제한을 유지합니다. 이러한 배포를 완화하려면 관리 설정에서 `"disabled": true`를 설정합니다.
* [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/ko/env-vars)가 설정되면 Claude Code는 관리 설정을 포함한 모든 소스에서 `filesystem.disabled`를 무시하고 파일시스템 격리를 유지합니다.

관리 `credentials.files` 항목이 `filesystem.disabled`를 고정하여 개발자가 파일시스템 격리를 끌 수 없도록 잠그는지 여부는 항목의 `mode`와 샌드박스가 시작될 때 항목에 어떤 일이 발생하는지에 따라 달라집니다:

| 관리 항목                                                                                          | `filesystem.disabled` 고정 | 격리가 꺼져 있을 때 파일을 보호하는 것                                                             |
| ---------------------------------------------------------------------------------------------- | ------------------------ | ---------------------------------------------------------------------------------- |
| `"mode": "deny"`                                                                               | 예                        | 없음: 읽기 블록은 파일시스템 계층의 일부입니다                                                         |
| `"mode": "mask"`, 마스크로 적용됨                                                                     | 아니요                      | 마스킹 자체: Linux 및 WSL2의 [센티널 복사 및 프록시](#mask-credential-files), macOS의 샌드박스 자체 읽기 규칙 |
| `"mode": "mask"`, [설정 시 `deny`로 폴백됨](#mask-credential-files)                                   | 아니요                      | 없음, `deny`와 동일합니다. 디렉토리와 같이 마스킹할 수 없는 경로를 명시적 `deny` 항목으로 나열합니다. 이는 키를 고정합니다       |
| `"mode": "mask"`, [검증으로 `deny`로 저하됨](/docs/ko/managed-settings#invalid-entries-in-managed-settings) | 예, 명시적 `deny`처럼          | 없음, `deny`와 동일합니다                                                                  |

폴백은 Claude Code가 이미 설정을 읽은 후 샌드박스가 시작될 때 발생하므로 폴백된 항목은 절대 고정하지 않습니다. 검증은 설정이 로드되는 동안 잘못된 항목을 `deny`로 다시 쓰므로 저하된 항목은 `deny`로 작성한 것처럼 고정합니다.

<h4 id="what-changes-when-filesystem-isolation-is-off">
  파일시스템 격리가 꺼져 있을 때 변경되는 것
</h4>

`filesystem.disabled`를 설정하면 파일시스템 계층 자체가 적용하는 보호가 해제됩니다. 다른 계층이 적용하는 보호는 계속 적용됩니다:

| 보호                                                                               | 파일시스템 격리가 꺼져 있을 때                                                                      |
| -------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| `filesystem.denyRead` 및 [`credentials.files`](#protect-credentials) `deny` 읽기 블록 | 적용되지 않음. 파일시스템 계층이 둘 다 적용합니다                                                           |
| `credentials.envVars` `deny` 및 `mask` 항목                                         | 적용됨. 환경 변수 스크러빙은 파일시스템 계층과 독립적입니다                                                      |
| [`credentials.files` `mask` 항목](#mask-credential-files) 마스크로 적용됨                 | 적용됨: 마스킹은 파일시스템 계층과 독립적입니다. [폴백된](#mask-credential-files) 항목은 모든 `deny` 항목처럼 적용되지 않습니다 |

두 가지 다른 것이 변경됩니다:

* 샌드박싱된 명령은 세션 임시 디렉토리 대신 셸의 `$TMPDIR`을 상속합니다. 모든 임시 디렉토리는 쓸 수 있고 Claude Code는 더 이상 명령을 세션 임시 디렉토리로 리디렉션하지 않기 때문입니다.

  Linux에서 변수는 종종 부모 셸에서 설정되지 않으므로 샌드박싱된 명령 내에서 비어 있을 수 있습니다. Claude Code는 `$TMPDIR`에 의존하는 대신 `mktemp -d`로 스크래치 디렉토리를 만들도록 Claude에 Bash 도구 지침을 통해 알립니다.
* [`autoAllowBashIfSandboxed`](/docs/ko/settings-reference#sandbox-autoallowbashifsandboxed)는 여전히 기본값이 `true`이므로 샌드박싱된 명령은 프롬프트 없이 계속 실행됩니다. 샌드박싱된 명령에 대해 프롬프트하려면 `false`로 설정합니다.

<h3 id="protect-credentials">
  자격증명 보호
</h3>

`sandbox.credentials` 설정은 샌드박싱된 명령이 액세스하면 안 되는 자격증명 파일 및 환경 변수를 선언합니다. 각 항목은 파일 경로 또는 환경 변수와 `mode`를 지정합니다. 전용 `credentials` 블록은 자격증명 규칙을 함께 그룹화하고 일반 파일시스템 규칙과 분리합니다.

`"mode": "deny"`를 사용하는 항목의 경우 파일 경로는 샌드박스 내에서 읽기가 거부되며, 이는 `filesystem.denyRead`가 적용하는 것과 동일한 제한이고, 환경 변수는 각 샌드박싱된 명령 실행 전에 설정 해제됩니다. 파일 보호는 파일시스템 계층의 일부이므로 [파일시스템 격리를 비활성화](#disable-filesystem-isolation)하면 적용되지 않습니다. 환경 변수 보호는 여전히 적용됩니다.

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

환경 변수 항목 및 파일 항목은 [자격증명 마스킹](#mask-credentials)에서 설명하는 `"mode": "mask"`도 허용합니다.

파일 경로는 `sandbox.filesystem.*` 설정과 동일한 [접두사 규칙](/docs/ko/settings-reference#sandbox-path-prefixes)을 따릅니다.

Claude Code는 세션이 로드하는 모든 [설정 범위](/docs/ko/settings#settings-precedence)의 `deny` 항목을 병합합니다. `deny` 항목은 액세스를 좁히기만 하므로 모든 범위는 항목을 추가할 수 있지만 다른 범위가 추가한 항목을 제거할 수는 없습니다.

[설정 소스를 제외](#configure-sandboxing)할 때:

* **프로젝트 또는 로컬 설정**: Claude Code는 해당 `credentials` 항목을 적용하지 않습니다. Claude Code v2.1.246 이상이 필요합니다.
* **사용자 설정**: Claude Code는 여전히 `~/.claude/settings.json`의 `deny` 항목을 적용하고 [파일 `mask` 항목](#mask-credential-files)을 제한으로 유지하지만 [환경 변수 `mask` 항목](#mask-environment-variables)은 삭제합니다.

기본 제공 자격증명 거부 목록이 없으므로 나열한 파일 및 변수만 제한됩니다.

`sandbox.credentials`는 샌드박싱된 Bash 명령에만 영향을 미칩니다. 샌드박싱 여부와 관계없이 모든 하위 프로세스에서 자격증명을 제거하려면 [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/ko/env-vars)를 설정합니다.

<h3 id="mask-credentials">
  자격증명 마스킹
</h3>

마스킹은 [자격증명 보호](#protect-credentials)의 `deny` 항목보다 더 나아갑니다. 자격증명을 차단하는 대신 Claude Code는 샌드박싱된 명령에 플레이스홀더인 센티널을 표시하고 [샌드박스 프록시](#network-isolation)는 허용하는 호스트에 대한 아웃바운드 요청에서 실제 값으로 바꿉니다. 파일의 경우 대체는 Linux 및 WSL2 동작입니다. [macOS는 대신 파일을 차단합니다](#mask-credential-files).

<h4 id="mask-environment-variables">
  환경 변수 마스킹
</h4>

`"mode": "mask"`는 자격증명을 보호하면서 이를 사용하여 인증하는 도구가 계속 작동하도록 합니다. `deny`는 변수를 완전히 제거하므로 `gh` 또는 `npm`과 같이 이를 필요로 하는 도구도 중단됩니다. Claude Code v2.1.199 이상이 필요합니다.

`mask`를 사용하면 샌드박싱된 명령은 실제 값 대신 세션별 센티널 값을 봅니다. 각 `mask` 항목은 `injectHosts`를 나열할 수 있으며, 이는 실제 값이 도달할 수 있는 호스트입니다. 요청이 이 중 하나에 대해 샌드박스를 떠날 때 [샌드박스 프록시](#network-isolation)는 센티널을 실제 값으로 바꿉니다. 명령과 이것이 기록하는 모든 것은 실제 자격증명을 보유하지 않지만 요청은 여전히 인증됩니다.

프록시는 요청 내용 내에서 자격증명을 대체하므로 이를 봐야 합니다. [`network.tlsTerminate`](/docs/ko/settings-reference#sandbox-network-tlsterminate)를 설정하여 프록시가 TLS 자체를 종료하도록 합니다.

이 없이는 마스킹이 폐쇄 상태로 실패합니다: 명령은 여전히 센티널만 보지만 센티널이 변경되지 않은 상태로 서버에 도달하고 인증이 실패합니다. Claude Code는 시작 시 이 잘못된 구성을 보고합니다.

대체는 헤더 및 요청 본문을 포함합니다. 자격증명 자체가 아니라 자격증명에서 파생된 서명으로 인증하는 요청은 프록시에서 다시 서명해야 합니다. [AWS 요청 다시 서명](#re-sign-aws-requests)은 AWS에 대해 이것이 어떻게 작동하는지 다룹니다.

프록시는 [도메인 허용 목록](#network-isolation)이 허용하는 연결에만 주입하므로 각 `injectHosts` 대상도 `network.allowedDomains`를 통해 도달 가능해야 합니다.

아래 예제는 두 개의 토큰을 마스킹합니다. `GH_TOKEN`은 `api.github.com`에 대한 요청에서만 대체되고, `NPM_TOKEN`은 `injectHosts`가 없으며 `network.allowedDomains`의 모든 호스트에 대한 요청에서 대체됩니다.

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

<span id="ipv6-destinations-in-injecthosts" />IPv6 대상을 두 목록에서 다르게 표기합니다. 각 목록에는 자체 매처가 있기 때문입니다:

* **`network.allowedDomains`**: [도메인 목록이 사용하는 괄호 형식](#ipv6-addresses-in-domain-lists), 예: `"[::1]"`. 프록시는 이 목록을 확인하여 연결을 허용합니다.
* **`injectHosts`**: 정규 압축 형식의 베어 주소, 예: `"::1"` 또는 `"2001:db8::1"`. 프록시는 각 항목을 연결의 베어 대상 주소와 비교하며 포트를 무시하므로 괄호, 영역 ID 또는 다르게 압축된 표기는 절대 일치하지 않으며 프록시는 거기에 자격증명을 주입하지 않습니다.

`claude doctor`는 대상과 절대 일치할 수 없는 `injectHosts` 항목에 `Sandbox credential injectHosts entries can never match their destination` 경고로 플래그를 지정합니다. 이 확인에는 Claude Code v2.1.229 이상이 필요합니다.

`deny`와 달리 마스킹은 프록시가 나열된 호스트에 실제 자격증명을 보내도록 승인하므로 Claude Code는 사용자 또는 관리자가 제어하는 설정에서만 인정합니다: 사용자 설정, 관리 설정 및 `--settings` CLI 플래그. Claude Code는 리포지토리의 `.claude/settings.json` 또는 `.claude/settings.local.json`의 `mask` 항목을 무시합니다. 이러한 파일에서 `network.tlsTerminate` 및 [`credentials.allowPlaintextInject`](/docs/ko/settings-reference#sandbox-credentials-allowplaintextinject)도 무시합니다. 이는 프록시가 암호화되지 않은 요청에 자격증명을 주입할 수 있도록 하는 설정입니다. [사용자 설정을 제외](#configure-sandboxing)하면 Claude Code는 `~/.claude/settings.json`의 환경 변수 `mask` 항목도 삭제합니다.

관리자가 서버 관리 설정을 통해 `mask` 항목, `network.tlsTerminate` 또는 `credentials.allowPlaintextInject`를 제공할 때 이들은 [승인이 필요한 설정](/docs/ko/server-managed-settings#security-approval-dialogs)으로 계산됩니다.

동일한 변수가 모든 범위에서 `deny`로 나열되면 `deny`가 우선합니다.

마스킹은 기본적으로 변수의 전체 값을 바꾸며, 이는 베어 토큰에 적합합니다. Claude Code v2.1.224 이상이 필요한 선택적 항목 필드는 구조가 있는 값을 처리합니다:

* `extract`: Claude Code가 값 전체에 적용하는 정규 표현식으로, 각 일치의 그룹 1로 캡처된 텍스트만 바꾸므로 값을 파싱하는 도구(예: `DATABASE_URL` 연결 문자열)는 샌드박스 내에서 계속 작동합니다. 패턴은 최소한 하나의 캡처 그룹을 포함해야 합니다.
* `onExtractNoMatch`는 패턴이 아무것도 일치하지 않을 때 발생하는 일을 제어합니다:
  * `warn`, 기본값, 경고하고 변수를 마스킹되지 않은 상태로 전달합니다
  * `deny`는 샌드박스 내에서 변수를 설정 해제합니다
  * `error`는 구성을 수정할 때까지 샌드박스 설정을 중지합니다
* `decode: "jwt"`: JSON Web Token (JWT)을 보유하는 변수의 경우. Claude Code는 값이 JWT인지 확인하고 이를 구조적으로 유효한 가짜 토큰으로 바꾸므로 샌드박스 내의 토큰을 디코딩하는 코드는 계속 작동합니다. `maskClaims`를 추가하여 전체 토큰을 바꾸는 대신 개별적으로 마스킹할 최상위 페이로드 클레임을 나열합니다. 다른 클레임은 읽을 수 있는 상태로 유지됩니다. 값이 JWT로 확인되지 않거나 나열된 클레임이 일치하지 않으면 Claude Code는 경고와 함께 변수를 마스킹되지 않은 상태로 전달합니다. `decode`는 `extract`와 결합할 수 없습니다.

설정 참조의 [`credentials.envVars[]` 행](/docs/ko/settings-reference#sandbox-settings)에서 전체 필드 목록을 참조하세요.

<h4 id="re-sign-aws-requests">
  AWS 요청 다시 서명
</h4>

AWS 요청은 요청 내용에 대한 SigV4 서명을 전달하므로 `AWS_ACCESS_KEY_ID` 및 `AWS_SECRET_ACCESS_KEY`를 함께 마스킹합니다. 프록시는 액세스 키의 센티널로 SigV4 요청을 감지하고 실제 값을 대체한 후 다시 서명합니다. 비밀만 마스킹하면 플레이스홀더로 서명된 요청이 남으며, 프록시는 이를 감지할 수 없으므로 AWS에서 실패합니다. Claude Code는 시작 시 이 경우에 대해 경고하지만 액세스 키 ID만 마스킹될 때는 경고하지 않습니다. 프록시가 다시 서명할 수 없는 감지된 요청(예: `x-amz-date` 헤더가 누락된 요청)은 손상된 서명으로 서버에 도달하는 대신 프록시 오류로 실패합니다.

Claude Code는 전체 값을 마스킹할 때 기존 `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` 및 `AWS_SESSION_TOKEN` 변수를 자동으로 하나의 자격증명으로 연결합니다. AWS 자격증명이 다른 이름의 변수에 있으면 [`credentials.awsPairs`](/docs/ko/settings-reference#sandbox-credentials-awspairs)로 직접 그룹화합니다. Claude Code v2.1.224 이상이 필요합니다. 이 예제는 이미 [위의 마스킹 구성](#mask-environment-variables)처럼 전체 값을 마스킹하는 `MY_KEY_ID`, `MY_SECRET_KEY` 및 `MY_SESSION_TOKEN`을 마스킹하는 구성에 페어링을 추가합니다:

```json theme={null}
{
  "sandbox": {
    "credentials": {
      "awsPairs": [
        {
          "accessKeyIdVar": "MY_KEY_ID",
          "secretAccessKeyVar": "MY_SECRET_KEY",
          "sessionTokenVar": "MY_SESSION_TOKEN"
        }
      ]
    }
  }
}
```

각 항목은 다음 규칙을 따릅니다:

* `accessKeyIdVar` 및 `secretAccessKeyVar`는 액세스 키 ID 및 비밀 키를 보유하는 마스킹된 `envVars` 항목의 이름을 지정합니다. 선택적 `sessionTokenVar`는 임시 자격증명의 세션 토큰을 보유하는 항목의 이름을 지정합니다. 설정되면 프록시는 다시 서명된 요청에서 실제 토큰을 `x-amz-security-token`으로 보냅니다.
* 각 명명된 변수는 `extract` 또는 `decode` 없이 전체 값을 마스킹하는 `mask` 항목이어야 합니다.
* 프록시는 액세스 키 ID 항목의 `injectHosts`에 나열된 호스트에서 요청을 다시 서명합니다.
* 페어에서 기존 변수를 명명하면 자동 페어링을 바꿉니다.

`mask` 항목과 마찬가지로 `awsPairs`는 사용자 설정, 관리 설정 및 `--settings` CLI 플래그에서만 인정됩니다.

세 가지 AWS 요청 형식은 프록시가 다시 계산할 수 없는 서명을 전달합니다. 이러한 요청이 마스킹된 페어의 플레이스홀더로 서명되면 프록시는 손상된 서명을 전달하는 대신 실패합니다. 마스킹되지 않은 자격증명으로 서명된 요청은 절대 영향을 받지 않습니다. [`credentials.sigv4`](/docs/ko/settings-reference#sandbox-credentials-sigv4) 설정은 Claude Code v2.1.224 이상이 필요하며 형식당 이를 완화합니다: 형식의 키를 `passthrough`로 설정하면 플레이스홀더 파생 서명으로 요청을 전달하므로 호출 도구는 프록시 오류 대신 AWS 자체의 거부 응답을 받습니다. `awsPairs`와 마찬가지로 `sigv4`는 사용자 설정, 관리 설정 및 `--settings` CLI 플래그에서만 인정됩니다.

| 요청 형식                | `sigv4` 키   | 프록시가 다시 서명할 수 없는 이유                            |
| :------------------- | :---------- | :--------------------------------------------- |
| aws-chunked 스트리밍 업로드 | `streaming` | 청크별 서명이 시드 서명에서 체인되므로 다시 서명하려면 본문을 다시 작성해야 합니다 |
| 사전 서명된 URL           | `presigned` | 서명은 `Authorization` 헤더 없이 URL 자체에 있습니다         |
| SigV4A 비대칭 서명        | `sigv4a`    | 다시 계산할 공유 키 HMAC이 없습니다                         |

<h4 id="mask-credential-files">
  자격증명 파일 마스킹
</h4>

파일 항목도 `"mode": "mask"`를 허용하며, Claude Code v2.1.221 이상이 필요합니다. 샌드박싱된 명령이 보는 것은 플랫폼에 따라 다릅니다:

* **Linux 및 WSL2**: 샌드박싱된 명령은 파일의 센티널 복사본을 읽으며, 비밀이 플레이스홀더 값으로 바뀐 스탠드인이고, [샌드박스 프록시](#network-isolation)는 송신에서 실제 값으로 대체합니다.
* **macOS**: 샌드박싱된 명령은 나열된 파일을 읽을 수 없습니다. Claude Code는 센티널 복사본을 구축하지 않으며 송신에서 아무것도 대체하지 않으므로 파일로 인증하는 도구는 샌드박스 내에서 작동하지 않으며, `deny`와 동일한 효과입니다. `deny` 항목과 달리 [파일시스템 격리를 비활성화](#disable-filesystem-isolation)할 때도 읽기 블록이 유지됩니다.

모든 플랫폼에서 Claude Code는 [`network.tlsTerminate`](/docs/ko/settings-reference#sandbox-network-tlsterminate) 요구 사항 및 `injectHosts`를 [마스킹된 환경 변수](#mask-environment-variables)와 동일한 방식으로 적용하고 리포지토리 설정을 동일한 방식으로 무시합니다. [사용자 설정을 제외](#configure-sandboxing)하면 Claude Code는 `~/.claude/settings.json`의 파일 `mask` 항목을 제한으로 유지하지만 항목은 더 이상 프록시가 실제 값을 대체하도록 승인하지 않습니다.

아래 예제는 `~/.config/gh/hosts.yml`에 저장된 GitHub 토큰을 마스킹합니다. 아래에서 다루는 `extract` 패턴은 Claude Code에 파일의 어느 부분이 비밀인지 알려줍니다. Linux 및 WSL2에서 파일을 읽는 샌드박싱된 명령은 토큰 대신 센티널을 얻으며 프록시는 `api.github.com`에 대한 요청에서 실제 토큰을 대체합니다:

```json theme={null}
{
  "sandbox": {
    "enabled": true,
    "network": {
      "tlsTerminate": {},
      "allowedDomains": ["*.github.com"]
    },
    "credentials": {
      "files": [
        {
          "path": "~/.config/gh/hosts.yml",
          "mode": "mask",
          "extract": "oauth_token:\\s*(\\S+)",
          "injectHosts": ["api.github.com"]
        }
      ]
    }
  }
}
```

마스크가 활성화되었는지 확인하려면 Claude에 샌드박싱된 명령에서 `cat ~/.config/gh/hosts.yml`을 실행하도록 요청합니다: Linux 및 WSL2에서 출력은 토큰 대신 센티널 값을 표시하고 macOS에서는 읽기가 실패합니다.

Linux 및 WSL2에서 `extract` 패턴은 `hosts.yml`의 나머지를 읽을 수 있게 유지하는 것입니다. Claude Code는 정규 표현식을 전체 파일에 적용하고 각 일치의 그룹 1로 캡처된 텍스트만 바꾸므로 `gh`는 여전히 구성을 파싱하고 토큰만 플레이스홀더입니다. `.netrc`, JSON 또는 YAML과 같이 도구가 파싱하는 구조화된 파일에 `extract`를 사용합니다. 패턴은 최소한 하나의 캡처 그룹을 포함해야 합니다. `extract` 없이 Claude Code는 전체 파일 내용을 하나의 센티널 값으로 바꾸며, 이는 단일 베어 비밀을 보유하고 다른 것은 없는 파일에 적합합니다.

JWT를 보유하는 파일의 경우 `extract` 대신 또는 함께 `decode: "jwt"`를 설정합니다. `decode`에는 Claude Code v2.1.224 이상이 필요합니다. Claude Code는 기본 제공 패턴 또는 설정된 `extract` 패턴으로 JWT 후보를 찾고, 각 후보가 JWT인지 확인하고, 이를 구조적으로 유효한 가짜 토큰으로 바꾸므로 샌드박스 내에서 토큰을 디코딩하는 코드는 계속 작동합니다. `maskClaims`를 추가하여 각 확인된 토큰 내에서 명명된 최상위 페이로드 클레임만 마스킹하고 다른 클레임은 읽을 수 있게 유지합니다. 후보가 확인되지 않거나 명명된 클레임이 일치하지 않으면 아래의 `onExtractNoMatch` 필드가 결과를 제어합니다. 패턴이 아무것도 일치하지 않을 때와 동일합니다.

두 개의 선택적 필드는 일치 동작을 개선합니다. 둘 다 `mode`가 `mask`이고 `extract` 또는 `decode`가 설정된 경우에만 적용됩니다. macOS에서 Claude Code는 파일시스템 격리가 켜져 있을 때마다 `mask` 항목을 `deny`로 적용하므로 이러한 필드 및 아래의 일치 없음 결과는 [파일시스템 격리가 꺼져 있을 때](#disable-filesystem-isolation)만 적용됩니다:

* `onExtractNoMatch`는 일치가 파일에서 마스킹할 것을 찾지 못할 때 발생하는 일을 제어합니다:

  * `warn`, 기본값, 경고하고 항목을 건너뜁니다. 샌드박싱된 명령은 실제 파일을 마스킹되지 않은 상태로 읽을 수 있습니다. 기본값은 자격증명이 합법적으로 없을 수 있는 경우에 적합합니다. 비밀이 있을 수 있지만 패턴이 놓칠 수 있으면 `deny`를 사용합니다
  * `deny`는 파일을 대신 읽을 수 없게 만듭니다
  * `error`는 구성을 수정할 때까지 샌드박스 설정을 중지합니다

  Claude Code는 [파일시스템 격리를 비활성화](#disable-filesystem-isolation)할 때 `deny`를 `error`로 처리하고, 모든 설정 소스의 `filesystem.allowRead` 항목이 파일의 경로를 다시 열 때도 마찬가지입니다.
* `maskDuplicates`는 또한 각 마스킹된 자격증명 값의 축자 복사본을 바꾸며, `extract` 캡처 또는 `decode` 확인된 토큰은 일치한 범위 외부에서 발견되며, 비밀이 일치가 도달하지 않는 곳에서 반복됩니다. 원시 부분 문자열을 일치시키므로 짧거나 일반적인 값은 나타나는 모든 곳에서 바뀝니다. 긴 고엔트로피 비밀을 위해 예약합니다. 기본값: false.

`mask`는 단일 파일에 적용되므로 각 자격증명 파일을 개별적으로 나열합니다. Claude Code는 안전하게 마스킹할 수 없는 `mask` 항목에 대해서는 `deny`로 폴백합니다: 디렉토리 경로, 글로브 패턴, 8 MiB보다 큰 파일 또는 UTF-8 텍스트가 아닌 파일. 디렉토리를 명시적 `deny` 항목으로 대신 작성합니다. [어떤 설정이 이를 비활성화할 수 있는지](#which-settings-can-disable-it) 아래의 표는 각 형식이 `filesystem.disabled`를 고정하는지 여부와 파일시스템 격리가 꺼져 있을 때 동작하는 방식을 다룹니다.

<h2 id="how-sandboxing-works">
  샌드박싱 작동 방식
</h2>

<h3 id="filesystem-isolation">
  파일 시스템 격리
</h3>

샌드박스된 Bash 도구는 파일 시스템 접근을 특정 디렉터리로 제한합니다:

* **기본 쓰기 동작**: 현재 작업 디렉터리 및 그 하위 디렉터리, `--add-dir`, `/add-dir` 또는 [`permissions.additionalDirectories`](/docs/ko/settings-reference#permissions-additionaldirectories)로 추가한 모든 디렉터리, 그리고 `$TMPDIR`이 가리키는 세션 임시 디렉터리에 대한 읽기 및 쓰기 접근
* **기본 읽기 동작**: 특정 거부된 디렉터리를 제외한 전체 컴퓨터에 대한 읽기 접근. 이 기본 설정은 여전히 `~/.aws/credentials` 및 `~/.ssh/`와 같은 자격 증명 파일 읽기를 허용합니다. [`sandbox.credentials`](#protect-credentials)를 사용하여 이러한 파일의 읽기를 차단하고 비밀 환경 변수를 설정 해제하거나, 경로를 `denyRead`에 추가합니다.
* **차단된 접근**: 명시적 권한 없이 작업 디렉터리, 추가된 디렉터리, 세션 임시 디렉터리 외부의 파일을 수정할 수 없습니다. 여기에는 `~/.bashrc`와 같은 셸 구성 파일 및 `/bin/`의 시스템 바이너리가 포함됩니다.
* **Git worktrees**: 작업 디렉터리가 [연결된 git worktree](/docs/ko/worktrees)인 경우, 샌드박스는 `git commit`과 같은 명령이 refs 및 인덱스를 업데이트할 수 있도록 주 저장소의 공유 `.git` 디렉터리에 대한 쓰기도 허용합니다. 해당 디렉터리 내의 `hooks/` 및 `config`에 대한 쓰기는 계속 거부됩니다.
* **구성 가능**: 설정을 통해 사용자 정의 허용 및 거부 경로 정의

파일 시스템 격리를 완전히 건너뛰면서 네트워크 격리를 유지하려면 [`sandbox.filesystem.disabled`](#disable-filesystem-isolation)를 설정합니다.

<h3 id="protected-paths">
  보호된 경로
</h3>

샌드박스된 명령이 쓸 수 있는 디렉터리 내에서, 샌드박스는 여전히 Claude Code가 구성 및 코드를 로드하는 파일에 대한 쓰기를 거부합니다. 이러한 파일을 편집할 수 있는 명령은 자신에게 권한을 부여하거나, Claude Code가 샌드박스 외부에서 실행하는 hook 또는 MCP 서버를 추가할 수 있습니다. 권한 시스템에는 자체 [보호된 경로](/docs/ko/permission-modes#protected-paths)가 있으며, 이는 도구가 실행되기 전에 Claude Code가 승인하는 것을 제어합니다. 샌드박스의 목록은 이미 실행 중인 명령에 적용됩니다. 네 가지 경로 그룹을 다룹니다:

* **작업 디렉터리 및 그 위의 디렉터리**: `.claude` 설정 파일, `.claude/skills`, `.claude/agents`, `.claude/commands`, `.claude/hooks` 디렉터리, `.mcp.json`, 그리고 Claude Code가 자체적으로 실행하는 파일(예: `.claude/workflows` 및 `.claude/scheduled_tasks.json`)
* **작업 디렉터리에만**: `.bashrc` 및 `.zshrc`와 같은 셸 시작 파일, `.gitconfig`, `.vscode` 및 `.idea` 디렉터리, 그리고 `.git` 내의 `hooks` 및 `config`
* **작업 디렉터리를 베어 git 저장소로 변환할 파일**: 최상위 수준의 `HEAD`, `objects`, `refs`, 그리고 `HEAD`가 옆에 있을 때의 `config` 및 `hooks`. `HEAD`가 없어도 `config`라는 파일은 거부됩니다. Linux 및 WSL2에서, 샌드박스된 명령이 실행되는 동안 나타나는 최상위 수준의 `HEAD` 파일 또는 `objects` 또는 `refs` 디렉터리는 샌드박스에서 삭제합니다.
* **`~/.claude` 또는 `CLAUDE_CONFIG_DIR`이 가리키는 디렉터리**: 대부분의 내용, 그리고 `~/.claude.json` 및 `.credentials.json` 자격 증명 저장소

보호된 설정 파일의 경로에 심볼릭 링크가 세션 중에 나타나면, 샌드박스는 다음 명령부터 그것이 가리키는 파일에 대한 쓰기도 거부합니다.

이러한 경로 중 하나를 면제할 방법은 없습니다. `allowWrite` 항목 또는 경로를 포함하는 `Edit` 허용 규칙은 보호를 해제하지 않습니다. 보호를 끄는 유일한 방법은 [`filesystem.disabled`](#disable-filesystem-isolation)이며, 이는 모든 경로에 대해 파일 시스템 격리를 끕니다. 이러한 경로 대부분이 머신에서 어떻게 해석되는지 보려면 `/sandbox`를 실행하고 **Config** 탭을 열면, **Denied within allowed** 아래에 나열되어 있으며 자신의 `denyWrite` 항목과 함께 섞여 있습니다.

`git merge` 또는 `git checkout`이 이러한 경로 중 하나에서 `unable to unlink old`로 실패하면, [문제 해결](#troubleshooting)을 참조합니다.

<h3 id="network-isolation">
  네트워크 격리
</h3>

네트워크 접근은 샌드박스 외부에서 실행되는 프록시 서버를 통해 제어됩니다:

* **도메인 제한**: Claude Code는 기본적으로 도메인을 사전 허용하지 않습니다. 명령이 새 도메인이 필요한 첫 번째 시간에, Claude Code는 승인을 요청합니다. [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)에서는 Claude가 대신 명령이 필요한 호스트를 명령 자체에 [명령별 허용 도메인](#per-command-allowed-domains-in-auto-mode)에 따라 이름을 지정합니다.
* **승인 선택**: 프롬프트에서 예를 선택하면, Claude Code는 현재 세션의 나머지 동안 호스트를 허용하고 나중에 동일한 호스트에 대한 연결에 대해 다시 프롬프트하지 않습니다. "예, 다시 묻지 마세요"를 선택하면, Claude Code는 `WebFetch(domain:...)` 허용 규칙을 [로컬 설정](/docs/ko/permissions#permission-system)에 저장하므로, 호스트는 향후 세션에서도 허용된 상태로 유지됩니다.
* **사전 허용 도메인**: [`allowedDomains`](/docs/ko/settings-reference#sandbox-network-alloweddomains)로 도메인을 사전 허용하여 프롬프트를 완전히 피합니다. Claude Code는 또한 [권한 규칙](#permission-rules)에 설명된 대로 `WebFetch(domain:...)` 허용 규칙의 도메인을 사전 허용합니다.
* **엄격한 허용 목록**: 사용자, 관리 또는 CLI `--settings` 설정에서 [`strictAllowlist`](/docs/ko/settings-reference#sandbox-network-strictallowlist)를 `true`로 설정하면, Claude Code는 프롬프트 대신 허용 목록 외부의 모든 호스트에 대한 샌드박스된 명령 접근을 거부합니다. 허용 목록은 샌드박스가 프롬프트하는 것과 동일합니다: `allowedDomains` 더하기 `WebFetch(domain:...)` 허용 규칙의 도메인, 또는 `allowManagedDomainsOnly`가 설정되었을 때 관리 설정 항목만. Claude Code는 샌드박스된 명령에만 이를 적용합니다. `WebFetch`와 같은 인프로세스 도구는 여전히 [권한 규칙](#permission-rules)을 따릅니다. 저장소의 `.claude/settings.json` 또는 `.claude/settings.local.json`에서 설정하면 효과가 없습니다. Claude Code v2.1.219 이상이 필요합니다.
* **관리 잠금**: [`allowManagedDomainsOnly`](/docs/ko/settings-reference#sandbox-network-allowmanageddomainsonly)가 관리 설정에서 설정되면, 허용되지 않은 도메인은 프롬프트 대신 자동으로 차단되며, 관리 설정의 `allowedDomains` 및 `WebFetch(domain:...)` 허용 규칙만 인정됩니다.
* **기업 프록시**: 네트워크가 아웃바운드 트래픽을 기업 프록시를 통해 가도록 요구하는 경우, `HTTPS_PROXY`, `HTTP_PROXY`, `NO_PROXY`를 [프록시 구성](/docs/ko/network-config#proxy-configuration)이 설명하는 대로 설정합니다. 설정의 `env` 블록에서 [백그라운드 에이전트](/docs/ko/network-config#set-network-variables-in-settings-not-the-shell)도 가져오거나, Claude Code를 시작하는 환경에서. Claude Code는 도메인 허용 목록을 적용한 다음 허용된 연결을 해당 업스트림 프록시를 통해 터널링합니다.
* **사용자 정의 프록시 지원**: 고급 사용자는 아웃바운드 트래픽에 대한 사용자 정의 규칙을 구현할 수 있습니다.
* **포괄적 범위**: 제한은 명령으로 생성된 모든 스크립트, 프로그램, 서브프로세스에 적용됩니다.

`WebFetch(domain:...)` 규칙에서, 샌드박스는 두 가지 와일드카드 형식을 인정합니다: `*.example.com`과 같은 선행 `*.`, 그리고 베어 `*`. 베어 `*` 형식은 Claude Code v2.1.186 이상이 필요합니다. `WebFetch(domain:example.*)`와 같이 다른 위치의 와일드카드는 여전히 페치와 일치하지만 샌드박스된 명령에는 영향을 주지 않습니다.

<Note>
  내장 프록시는 요청된 호스트명을 기반으로 허용 목록을 적용하며, 기본적으로 TLS 트래픽을 종료하거나 검사하지 않습니다. Claude Code v2.1.199 이상에서 사용 가능한 실험적 [`network.tlsTerminate`](/docs/ko/settings-reference#sandbox-network-tlsterminate) 설정은 내장 프록시가 TLS 자체를 종료하도록 하며, 이는 [`mask` 자격 증명 항목](#mask-credentials)에 필요합니다. 기본값의 의미는 [보안 제한](#security-limitations)을 참조하고, 위협 모델이 TLS 검사를 요구하는 경우 [사용자 정의 프록시 구성](#custom-proxy-configuration)을 참조합니다.
</Note>

<h4 id="per-command-allowed-domains-in-auto-mode">
  자동 모드에서 명령별 허용 도메인
</h4>

샌드박싱이 켜진 [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)에서, Claude는 각 연결에 대해 네트워크 승인을 트리거하는 대신 명령이 필요한 호스트를 명령 자체에 이름을 지정합니다. 샌드박스에서 실행되는 각 Bash, PowerShell 또는 [Monitor](/docs/ko/tools-reference#monitor-tool) 명령은 샌드박스의 허용 목록을 넘어서는 호스트 목록을 가질 수 있습니다: `registry.npmjs.org`와 같은 도메인, `*.pythonhosted.org`와 같은 와일드카드, 또는 IP 주소, 각각 선택적 `:port`. 분류기는 호스트를 명령과 함께 검토합니다. Claude Code v2.1.271 이상이 필요합니다.

승인된 목록은 해당 명령 하나에만 실행되는 동안 이러한 호스트를 엽니다. 세션의 허용된 호스트 또는 설정에 아무것도 추가되지 않습니다. 다음 명령은 자신의 호스트를 이름 지정합니다.

호스트를 가진 명령은 권한 규칙 또는 샌드박스의 [자동 허용 모드](#sandbox-modes)에 의해 승인되는 대신 분류기로 이동합니다. [ask 규칙](/docs/ko/permissions#manage-permissions)이 명령에 대한 프롬프트를 강제하면, 터미널의 권한 대화는 호스트를 옆에 나열하고, 거기서 승인하면 둘 다 포함됩니다.

명령별 목록은 샌드박스가 기본적으로 거부하는 것만 넓힙니다. [`deniedDomains`](/docs/ko/settings-reference#sandbox-network-denieddomains) 항목은 여전히 차단합니다. [`strictAllowlist`](/docs/ko/settings-reference#sandbox-network-strictallowlist) 또는 [`allowManagedDomainsOnly`](/docs/ko/settings-reference#sandbox-network-allowmanageddomainsonly)가 허용 목록을 잠그면, Claude Code는 명령별 목록을 거부합니다.

명령별 목록이 적용되는 동안, Claude Code는 승인된 명령이 나열하지 않은 호스트에 대한 연결을 프롬프트나 분류기 확인 없이 거부합니다. 거부는 명령의 결과에서 호스트를 이름 지정하고, Claude는 호스트가 추가된 명령을 다시 실행합니다.

<h4 id="ipv6-addresses-in-domain-lists">
  도메인 목록의 IPv6 주소
</h4>

샌드박스의 도메인 목록은 `allowedDomains`, `deniedDomains`, 그리고 이들을 공급하는 `WebFetch(domain:...)` 규칙입니다. 이들 중 하나에서 IPv6 주소와 일치하려면, 리터럴을 괄호로 작성합니다: `"[::1]"`은 모든 포트에서 해당 주소와 일치하고, `"[::1]:443"`은 포트 443에서만 일치합니다. 포트를 1에서 65535 사이의 숫자로 선행 0 없이 작성합니다. 괄호로 묶인 형식은 Claude Code v2.1.229 이상이 필요합니다. v2.1.229 이전에는, 괄호로 묶이지 않은 항목의 마지막 콜론 뒤의 텍스트가 포트 번호였을 때, Claude Code는 이를 포트로 읽었으므로 `::1:443`은 포트 443의 주소 `::1`을 이름 지정했습니다.

IPv6 주소에 대한 네트워크 승인 프롬프트에서 "예, 다시 묻지 마세요"를 선택하면, Claude Code는 `WebFetch(domain:...)` 규칙을 주소가 괄호로 묶인 상태로 저장하므로, 규칙은 향후 세션에서 주소와 일치하는 상태로 유지됩니다.

괄호로 묶이지 않은 항목이 두 개 이상의 콜론을 가지면 모호합니다: `::1:443`은 완전한 IPv6 주소이자 주소 뒤에 포트가 있는 것입니다. Claude Code는 어느 읽기를 의도했는지 추측하는 대신 모호한 철자를 보수적으로 적용합니다:

* **거부 목록**: Claude Code는 항목이 파싱하는 모든 읽기를 거부하므로, 어느 읽기를 의도했든 차단됩니다. 파싱 가능한 읽기가 없는 항목의 경우, Claude Code는 아무것도 차단하지 않습니다.
* **허용 목록**: Claude Code는 작성한 것보다 더 많이 허용하지 않습니다. 해당 읽기가 깔끔하게 파싱될 때 모호한 항목을 호스트 및 포트 읽기로 다시 작성하고, 허용 목록을 넓히기보다는 항목을 완전히 삭제할 수 있습니다.

터미널에서 `claude doctor`를 실행하여 영향을 받는 항목을 찾습니다: `Sandbox network domain entries have unreliable spellings` 경고는 최대 3개를 이름 지정하고 나머지를 계산합니다. 각각을 괄호로 묶인 형식으로 다시 작성하여 경고를 지웁니다. 경고는 또한 `@`, 경로 또는 쿼리 문자, 또는 괄호 내 와일드카드와 같은 다른 이유로 철자가 신뢰할 수 없는 항목을 이름 지정합니다.

<h3 id="os-level-enforcement">
  OS 수준 적용
</h3>

샌드박스된 Bash 도구는 운영 체제 보안 기본 요소를 사용합니다:

* **macOS**: 샌드박스 적용을 위해 Seatbelt를 사용합니다.
* **Linux**: 격리를 위해 [bubblewrap](https://github.com/containers/bubblewrap)을 사용합니다.
* **WSL2**: Linux와 동일하게 bubblewrap을 사용합니다.

WSL1은 bubblewrap이 WSL2에서만 사용 가능한 커널 기능을 필요로 하기 때문에 지원되지 않습니다.

이러한 동일한 기본 요소는 독립 실행형 [`@anthropic-ai/sandbox-runtime`](https://github.com/anthropic-experimental/sandbox-runtime) 패키지로 사용 가능하며, [샌드박스 환경](/docs/ko/sandbox-environments#sandbox-runtime) 페이지는 전체 Claude Code 프로세스를 래핑하기 위한 별도의 접근 방식으로 다룹니다.

<h2 id="how-sandboxing-relates-to-permissions-and-permission-modes">
  샌드박싱이 권한 및 권한 모드와 어떻게 관련되는지
</h2>

샌드박싱, [권한 규칙](/docs/ko/permissions), 및 [권한 모드](/docs/ko/permission-modes)는 상호 보완적인 계층입니다. 아래 섹션에서는 샌드박스가 각각과 어떻게 상호작용하는지 다룹니다.

<h3 id="permission-rules">
  권한 규칙
</h3>

권한 규칙과 샌드박싱은 서로 다른 것들을 제어합니다:

* **권한 규칙**은 Claude Code가 사용할 수 있는 도구를 제어하며 도구가 실행되기 전에 평가됩니다. 이들은 모든 도구(Bash, Read, Edit, WebFetch, MCP 및 기타)에 적용되지만, deny 또는 ask 규칙은 다른 도구가 남아 있는 동안 [`EndConversation`](/docs/ko/tools-reference#endconversation-tool-behavior)을 차단할 수 없습니다.
* **샌드박싱**은 OS 수준의 강제 실행을 제공하여 셸 명령이 파일 시스템 및 네트워크 수준에서 접근할 수 있는 것을 제한합니다. 이는 Bash, PowerShell 및 [Monitor](/docs/ko/tools-reference#monitor-tool) 명령과 그 자식 프로세스에만 적용됩니다.

두 계층은 또한 강제 실행 방식이 다릅니다. Claude Code는 명령 문자열을 기반으로 명령이 실행되기 전에 권한 결정을 평가하며, 자동 모드에서는 명령이 안전한지 여부에 대한 별도 분류기의 판단을 기반으로 합니다. 운영 체제는 실행 중인 프로세스에 샌드박스 경계를 강제 실행하므로, 모델이 실행하도록 선택한 것과 관계없이 그리고 허용된 명령이 이름이 시사하는 것보다 더 많은 작업을 수행하더라도 유지됩니다.

파일 시스템 및 네트워크 제한은 샌드박스 설정과 권한 규칙을 통해 모두 구성됩니다:

| 설정 또는 규칙                                                       | 수행하는 작업                                                         |
| :------------------------------------------------------------- | :-------------------------------------------------------------- |
| `sandbox.filesystem.allowWrite`                                | 작업 디렉토리 외부의 경로에 대한 부프로세스 쓰기 액세스 권한 부여                           |
| `sandbox.filesystem.denyWrite` 및 `sandbox.filesystem.denyRead` | 특정 경로에 대한 부프로세스 액세스 차단                                          |
| `sandbox.filesystem.allowRead`                                 | `denyRead` 영역 내에서 특정 경로 읽기를 다시 허용                               |
| [`sandbox.filesystem.disabled`](#disable-filesystem-isolation) | 네트워크 격리를 유지하면서 파일 시스템 계층을 완전히 끔                                 |
| `Edit` allow 규칙                                                | 특정 경로에 대한 쓰기 액세스 권한 부여, `sandbox.filesystem.allowWrite`와 동일한 방식 |
| `Read` 및 `Edit` deny 규칙                                        | 특정 파일 또는 디렉토리에 대한 액세스 차단                                        |
| `WebFetch(domain:...)` allow 및 deny 규칙                         | 도메인 액세스 제어                                                      |
| 샌드박스 `allowedDomains`                                          | Bash 명령이 도달할 수 있는 도메인 제어                                        |
| 샌드박스 `deniedDomains`                                           | 더 광범위한 `allowedDomains` 와일드카드가 그렇지 않으면 허용할 특정 도메인 차단            |

샌드박스 설정과 권한 규칙의 경로 및 도메인은 최종 샌드박스 구성으로 병합됩니다.

[claude-code 저장소의 examples 디렉토리](https://github.com/anthropics/claude-code/tree/main/examples/settings)에는 샌드박스 관련 예제를 포함한 일반적인 배포 시나리오에 대한 시작 설정 구성이 포함되어 있습니다. 이들을 시작점으로 사용하고 필요에 맞게 조정하십시오.

<h3 id="permission-modes">
  권한 모드
</h3>

`/sandbox`는 [권한 모드](/docs/ko/permission-modes)가 아닙니다. 권한 모드는 도구 호출이 실행되는지 여부와 먼저 프롬프트를 받는지 여부를 결정하는 반면, 샌드박스는 Bash 명령이 실행되면 접근할 수 있는 것을 제한합니다. 이들은 제어하는 것과 작업별 프롬프트를 대체하는 것이 다릅니다:

|                                                                | 제어하는 것                   | 프롬프트를 대체하는 것                                                                                                                                    |
| :------------------------------------------------------------- | :----------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------- |
| `/sandbox`                                                     | Bash 명령이 실행되면 접근할 수 있는 것 | [자동 허용 모드](#sandbox-modes)의 샌드박스 경계 자체                                                                                                          |
| [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode) | 각 도구 호출이 실행되는지 여부        | 작업을 검토하는 분류기                                                                                                                                    |
| `--dangerously-skip-permissions`                               | 각 도구 호출이 실행되는지 여부        | 없음. [보호된 경로](/docs/ko/permission-modes#protected-paths) 검사도 건너뜀; [어떤 모드도 자동 승인하지 않는 작업](/docs/ko/permission-modes#actions-no-mode-auto-approves)은 여전히 적용됨 |

샌드박스의 [자동 허용 모드](#sandbox-modes)는 [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)와 별개입니다: 자동 허용은 샌드박스 경계가 이들을 포함하기 때문에 Bash 명령을 승인하는 반면, 자동 모드는 분류기를 사용하여 작업을 검토합니다. 이 둘은 독립적으로 작동하며 [샌드박스 모드](#sandbox-modes)에 나열된 예외를 제외하고 결합될 수 있습니다. 무인 실행을 위한 격리 경계를 선택하려면 [샌드박스 환경](/docs/ko/sandbox-environments#how-isolation-relates-to-permission-modes)을 참조하십시오. 일반적인 권한 모드 및 샌드박스 쌍과 각각을 시작하는 플래그의 표는 [일반적인 설정](/docs/ko/permission-modes#common-setups)을 참조하십시오.

<h2 id="configure-the-sandbox-for-your-organization">
  조직을 위해 샌드박스 구성
</h2>

관리자는 모든 사용자에게 샌드박싱을 요구하고, 개발자가 정책을 확대하는 것을 방지하고, 샌드박스 트래픽을 회사 프록시를 통해 라우팅할 수 있습니다.

<h3 id="enforce-sandboxing-with-managed-settings">
  관리 설정으로 샌드박싱 적용
</h3>

모든 개발자에게 샌드박스를 요구하려면 [관리 설정](/docs/ko/managed-settings#delivery-mechanisms)을 통해 `sandbox` 키를 제공합니다. MDM으로 관리되는 파일 또는 claude.ai의 [서버 관리 설정](/docs/ko/server-managed-settings)을 통해 제공합니다.

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
* **`allowUnsandboxedCommands: false`**: Claude Code가 `dangerouslyDisableSandbox` 탈출 해치를 무시하므로 샌드박스에서 명령이 실패할 때 Claude가 샌드박스 외부에서 다시 시도할 수 없습니다

함께 고려할 가치가 있는 두 가지 추가 사항이 있습니다. 격리 없이 실행해야 하는 조직 승인 도구에 대해 `excludedCommands`를 추가합니다. `~/.aws` 및 `~/.ssh`와 같은 자격 증명 디렉토리와 비밀 환경 변수에 대해 [`sandbox.credentials`](#protect-credentials) 항목을 추가합니다. 기본 읽기 정책은 여전히 이를 허용하기 때문입니다.

이 구성은 Claude가 실행하는 명령을 샌드박싱합니다. 개발자는 여전히 [`!` 셸 모드 프롬프트](/docs/ko/interactive-mode#shell-mode-with-prefix)에서 명령을 입력하고 Claude Code 외부의 모든 터미널에서 이미 가지고 있는 것과 동일한 액세스 권한으로 샌드박스 외부에서 실행할 수 있습니다. 입력된 명령이 샌드박싱되는 세션에 대해서는 [샌드박스 없는 재시도 탈출 해치](#the-unsandboxed-retry-escape-hatch)를 참조하십시오.

샌드박스는 기본 Windows에서 실행되지 않으므로 플릿에 Windows 호스트가 포함되면 이 구성을 macOS 및 Linux로 범위를 지정하거나 해당 사용자가 WSL2 또는 컨테이너 내에서 Claude Code를 실행하도록 합니다.

<h3 id="keep-developers-from-widening-the-policy">
  개발자가 정책을 확대하는 것을 방지
</h3>

`enabled` 및 `failIfUnavailable`과 같은 부울 키의 경우 Claude Code는 관리 값을 사용하고 개발자가 로컬로 설정한 모든 것을 무시합니다. `excludedCommands` 및 `allowRead`와 같은 배열 키의 경우 Claude Code는 세션이 로드하는 모든 범위의 항목을 병합하므로 개발자는 정책을 확대하는 항목을 추가할 수 있습니다.

관리 설정에서 `allowManagedReadPathsOnly`를 `true`로 설정하여 관리 설정의 `allowRead` 항목만 존중되도록 합니다. 이는 개발자가 조직 승인 경로 이상으로 읽기 액세스를 확대하는 것을 방지합니다. 네트워크 도메인을 동일한 방식으로 관리 값으로 잠그려면 [`allowManagedDomainsOnly`](/docs/ko/settings-reference#sandbox-network-allowmanageddomainsonly)를 설정합니다.

관리 설정이 `sandbox.filesystem`을 구성하거나 `"mode": "deny"`를 사용하여 `sandbox.credentials.files` 항목을 나열할 때 관리 설정만 [`filesystem.disabled`](#disable-filesystem-isolation)를 설정할 수 있으므로 개발자는 관리자가 배포한 파일 시스템 제한을 끌 수 없습니다. `mask` 항목이 키를 고정하는지 여부는 해결 방식에 따라 다릅니다. [어떤 설정이 이를 비활성화할 수 있는지](#which-settings-can-disable-it) 아래의 표는 네 가지 경우를 다룹니다.

`excludedCommands`는 동등한 관리 전용 잠금이 없으므로 개발자는 항상 샌드박스 외부에서 실행되는 추가 명령을 추가하는 항목을 추가할 수 있습니다. 관리 목록을 좁게 유지합니다.

<h3 id="custom-proxy-configuration">
  사용자 정의 프록시 구성
</h3>

고급 네트워크 보안이 필요한 조직의 경우 사용자 정의 프록시를 구현하여 다음을 수행할 수 있습니다:

* HTTPS 트래픽 복호화 및 검사
* 사용자 정의 필터링 규칙 적용
* 모든 네트워크 요청 로깅
* 기존 보안 인프라와 통합

Claude Code를 프록시로 지정하려면 [샌드박스 설정](/docs/ko/settings-reference#sandbox-settings)에서 프록시 포트를 설정합니다:

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
* **Go 기반 CLI가 macOS에서 TLS 검증 실패**: `gh`, `gcloud`, `terraform`과 같은 도구는 Seatbelt에서 TLS 검증에 실패할 수 있습니다. 이러한 도구를 [`excludedCommands`](/docs/ko/settings-reference#sandbox-excludedcommands)에 나열합니다. MITM 프록시 및 사용자 정의 CA와 함께 `httpProxyPort`를 사용하는 경우 대신 [`enableWeakerNetworkIsolation`](/docs/ko/settings-reference#sandbox-enableweakernetworkisolation)을 `true`로 설정합니다.
* **`open`, `osascript`, 또는 브라우저 기반 인증 흐름이 macOS에서 오류 `-600`으로 실패**: 샌드박스는 기본적으로 Apple Events를 차단합니다. 사용자, 관리 또는 CLI 설정에서 [`allowAppleEvents`](/docs/ko/settings-reference#sandbox-allowappleevents)를 `true`로 설정하여 이를 허용합니다. 프로젝트 설정은 이 키에 대해 무시됩니다. 이를 활성화하면 샌드박싱된 명령이 사용자 프롬프트 없이 다른 애플리케이션을 비샌드박싱된 상태로 시작할 수 있고 실행 중인 애플리케이션에 AppleScript 명령을 보낼 수 있으므로 코드 실행 격리가 제거됩니다. 이는 macOS 자동화 동의 프롬프트(TCC)의 적용을 받습니다. 또는 명령을 [`excludedCommands`](/docs/ko/settings-reference#sandbox-excludedcommands)에 추가합니다.
* **`docker` 명령 실패**: `docker`는 샌드박스와 호환되지 않습니다. `docker *`를 [`excludedCommands`](/docs/ko/settings-reference#sandbox-excludedcommands)에 추가합니다.
* **`pbcopy`, `xclip`, 또는 `wl-copy`가 클립보드를 업데이트하지 않음**: 이러한 클립보드 유틸리티는 샌드박스 내에서 시스템 클립보드에 도달하지 못할 수 있으며, 이 경우 이들에게 파이프된 텍스트가 도착하지 않습니다.

  Claude의 출력을 클립보드에 넣으려면 Claude에게 응답에서 인쇄하도록 요청한 다음 [`/copy`](/docs/ko/commands)를 실행합니다. `/copy`는 샌드박싱된 명령이 아닌 Claude Code 프로세스에서 클립보드에 씁니다.

  Claude가 텍스트를 이러한 도구 중 하나로 파이프할 때, 도구를 [`excludedCommands`](/docs/ko/settings-reference#sandbox-excludedcommands)에 추가해도 그 자체로는 그 호출을 샌드박스 외부로 꺼내지 않습니다.
* **git 명령이 `unable to unlink old`로 실패**: `git merge`, `git checkout` 및 유사한 명령은 샌드박스가 쓰기를 거부하는 파일을 교체해야 할 때 이런 방식으로 실패합니다. 해당 파일이 `.claude/skills`와 같은 [보호된 경로](#protected-paths) 아래에 있거나, `denyWrite` 항목 중 하나 아래에 있거나, 샌드박스가 명령을 쓸 수 있도록 허용하는 디렉토리 외부에 있을 수 있습니다. Linux 및 WSL2에서 오류는 `Read-only file system`으로 끝납니다.

  실패 후 Claude는 [명령을 샌드박스 외부에서 다시 실행하도록 제안](#the-unsandboxed-retry-escape-hatch)할 수 있습니다. 해당 재시도를 승인하거나 다른 터미널에서 git 명령을 직접 실행합니다. `allowUnsandboxedCommands`를 `false`로 설정한 경우 Claude는 재시도를 제안할 수 없으므로 명령을 직접 실행합니다. 동일한 git 명령이 자주 실패하면 [`excludedCommands`](/docs/ko/settings-reference#sandbox-excludedcommands)에 추가합니다.
* **Bubblewrap이 컨테이너 내에서 시작 실패**: 권한 없는 컨테이너에서 bubblewrap은 새로운 `/proc` 파일시스템을 마운트할 수 없으므로 샌드박싱된 명령은 `bwrap` 오류(예: `Can't mount proc on /newroot/proc: Operation not permitted`)로 실패합니다. [`enableWeakerNestedSandbox`](/docs/ko/settings-reference#sandbox-enableweakernestedsandbox)를 `true`로 설정하여 내부 샌드박스가 컨테이너의 기존 `/proc`을 바인드 마운트하도록 합니다. 외부 컨테이너가 이미 필요한 격리 경계를 제공할 때만 이 설정을 사용합니다. 새로운 `/proc` 마운트가 숨길 프로세스 정보를 샌드박싱된 명령에 노출하기 때문입니다.
* **0바이트 읽기 전용 파일이 `.claude` 설정 경로에 나타나고 "예, 다시 묻지 않기"가 저장되지 않음**: Linux 및 WSL2에서 샌드박스는 샌드박싱된 명령이 실행되는 동안 아직 존재하지 않는 파일에 대한 쓰기 거부를 0바이트 읽기 전용 자리 표시자를 만들어 유지합니다. 샌드박스는 그 후 자리 표시자를 제거합니다. 예를 들어 SIGKILL에 의해 세션이 정리 실행 전에 종료되면 자리 표시자가 남아 있습니다. 이후 세션은 매번 시작할 때마다 이들을 읽기 전용으로 바인드하므로 권한 선택 저장과 같은 설정 쓰기가 하나가 있는 곳에서 실패합니다.

  `claude doctor`를 실행하여 남은 자리 표시자 파일을 나열합니다. [`Stale sandbox mask files left by a killed session`](/docs/ko/errors#stale-sandbox-mask-files-left-by-a-killed-session) 경고는 그중 최대 3개의 이름을 표시하고 나머지는 개수로 표시합니다. 해당 프로젝트에서 다른 Claude Code 세션이 실행되지 않는 동안 `rm`으로 각 파일을 삭제합니다. v2.1.257 이전에는 Claude Code가 동일한 자리 표시자를 남겨두면서도 이를 알리지 않았습니다.
* **`--dangerously-skip-permissions`이 root로 실패**: 이 플래그는 Linux 및 macOS에서 root로 또는 sudo를 통해 실행할 때 차단됩니다. root 액세스와 권한 프롬프트 없음이 결합되면 시스템의 모든 파일 또는 서비스를 수정할 수 있기 때문입니다. 확인은 인식된 샌드박스 내에서 자동으로 건너뜁니다. 컨테이너에서 자율적으로 실행하려면 [dev 컨테이너](/docs/ko/devcontainer) 구성을 사용합니다. 이는 Claude Code를 비 root 사용자로 실행합니다.

<h2 id="limitations">
  제한 사항
</h2>

샌드박싱은 위험을 줄이지만 완전한 격리 경계는 아닙니다. 이를 하드 보안 제어로 사용하기 전에 아래 제한 사항을 검토합니다.

<h3 id="security-limitations">
  보안 제한 사항
</h3>

* **네트워크 필터링**: 샌드박스는 프로세스가 연결할 수 있는 도메인을 제한합니다. 기본 제공 프록시는 아웃바운드 트래픽을 종료하거나 TLS를 검사하지 않으므로 암호화된 연결의 내용은 검사되지 않습니다. 실험적인 [`network.tlsTerminate`](/docs/ko/settings-reference#sandbox-network-tlsterminate) 설정은 [`mask` 자격 증명 대체](#mask-credentials)를 위해 프록시에서 TLS를 종료하지만 콘텐츠 필터링을 추가하지 않습니다. 정책에서 신뢰할 수 있는 도메인만 허용하도록 보장하는 것은 사용자의 책임입니다.

<Warning>
  `github.com`과 같은 광범위한 도메인을 허용하면 데이터 유출 경로가 생성될 수 있습니다. 프록시가 TLS를 검사하지 않고 클라이언트 제공 호스트 이름에서 허용 결정을 내리기 때문에 샌드박스 내에서 실행되는 코드는 잠재적으로 [도메인 프론팅](https://en.wikipedia.org/wiki/Domain_fronting) 또는 유사한 기술을 사용하여 허용 목록 외부의 호스트에 도달할 수 있습니다. 위협 모델이 더 강력한 보장을 요구하면 TLS를 종료하고 트래픽을 검사하는 [사용자 정의 프록시](#custom-proxy-configuration)를 구성하고 그 CA 인증서를 샌드박스 내에 설치합니다. 더 강력한 TLS 인식 네트워크 격리는 활발한 개발 영역입니다.
</Warning>

* **Unix 소켓을 통한 권한 상승**: `allowUnixSockets` 구성은 실수로 샌드박스 우회로 이어질 수 있는 시스템 서비스에 대한 액세스를 부여할 수 있습니다. 예를 들어 `/var/run/docker.sock`에 대한 액세스를 허용하면 Docker 소켓을 통해 호스트 시스템에 대한 액세스를 효과적으로 부여합니다. 샌드박스를 통해 허용하는 모든 Unix 소켓을 신중하게 고려합니다.
* **파일시스템 권한 상승**: 과도하게 광범위한 파일시스템 쓰기 권한은 권한 상승 공격을 가능하게 할 수 있습니다. `$PATH`의 실행 파일을 포함하는 디렉토리, 시스템 구성 디렉토리 또는 `.bashrc` 또는 `.zshrc`와 같은 사용자 셸 구성 파일에 대한 쓰기를 허용하면 다른 사용자 또는 시스템 프로세스가 이러한 파일에 액세스할 때 다른 보안 컨텍스트에서 코드 실행으로 이어질 수 있습니다.
* **Linux 샌드박스 강도**: Linux 구현은 강력한 파일시스템 및 네트워크 격리를 제공하지만 권한 있는 네임스페이스 없이 Docker 환경 내에서 작동할 수 있도록 하는 `enableWeakerNestedSandbox` 모드를 포함합니다. 또는 권한 없는 사용자 네임스페이스가 sysctl에 의해 비활성화된 Linux 호스트에서. 이 옵션은 보안을 상당히 약화시키며 추가 격리가 다른 방식으로 적용되는 경우에만 사용해야 합니다.
* **macOS의 Apple Events**: macOS 샌드박스는 기본적으로 Apple Events를 차단합니다. `allowAppleEvents` 설정은 이 제한을 해제하여 `open` 및 `osascript`와 같은 도구가 작동하지만 코드 실행 격리를 제거합니다. 샌드박싱된 명령은 사용자 프롬프트 없이 다른 애플리케이션을 샌드박싱되지 않은 상태로 시작할 수 있으며 실행 중인 애플리케이션에 AppleScript 명령을 보낼 수 있습니다. 이는 앱별 macOS 자동화 동의 프롬프트(TCC)의 적용을 받습니다. 이는 사용자, 관리 또는 CLI 설정에서만 적용됩니다. 프로젝트 설정은 이를 활성화할 수 없습니다.

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

* **기본 제공 파일 도구**: Read, Edit 및 Write는 권한 시스템을 직접 사용하며 샌드박스를 통해 실행되지 않습니다. [권한](/docs/ko/permissions)을 참조합니다.
* **컴퓨터 사용**: Claude가 앱을 열고 화면을 제어할 때 격리된 환경이 아닌 실제 데스크톱에서 실행됩니다. 앱별 권한 프롬프트가 각 애플리케이션을 제어합니다. [CLI의 컴퓨터 사용](/docs/ko/computer-use) 또는 [Desktop의 컴퓨터 사용](/docs/ko/desktop#let-claude-use-your-computer)을 참조합니다.
* **환경 변수**: 샌드박싱된 Bash 명령은 기본적으로 부모 프로세스 환경을 상속합니다. 여기에는 설정된 모든 자격 증명이 포함됩니다. [`sandbox.credentials`](#protect-credentials)를 사용하여 샌드박싱된 명령에 대한 특정 변수를 설정 해제하거나 마스크하거나 [`CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`](/docs/ko/env-vars)를 설정하여 모든 하위 프로세스에서 자격 증명을 제거합니다.
* **하위 에이전트**: [하위 에이전트](/docs/ko/sub-agents)는 부모 세션과 동일한 프로세스에서 실행되며 동일한 샌드박스 구성을 사용합니다. 부모 세션에서 샌드박싱이 활성화되면 하위 에이전트 내의 Bash 명령이 샌드박싱됩니다.

<Warning>
  효과적인 샌드박싱은 파일시스템 및 네트워크 격리 모두를 필요로 합니다. 네트워크 격리가 없으면 손상된 에이전트가 SSH 키와 같은 민감한 파일을 유출할 수 있습니다. 파일시스템 격리가 없으면 권한 있는 정책이나 [파일시스템 레이어 비활성화](#disable-filesystem-isolation)로 인해 손상된 에이전트가 시스템 리소스를 백도어하여 네트워크 액세스를 얻을 수 있습니다. 기본값을 확대할 때 `allowWrite` 경로, 광범위한 `allowedDomains` 항목 또는 `excludedCommands` 예외가 다른 쪽의 제한을 취소하지 않는지 확인합니다.
</Warning>

<h2 id="see-also">
  참고 항목
</h2>

* [샌드박스 환경](/docs/ko/sandbox-environments): 기본 제공 샌드박스를 dev 컨테이너, 컨테이너 및 VM과 비교
* [보안](/docs/ko/security): 포괄적인 보안 기능 및 모범 사례
* [권한](/docs/ko/permissions): 권한 구성 및 액세스 제어
* [모든 설정](/docs/ko/settings-reference): 모든 설정 키
* [CLI 참조](/docs/ko/cli-reference): 명령줄 옵션
