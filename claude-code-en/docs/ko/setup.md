> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 고급 설정

> Claude Code의 시스템 요구사항, 플랫폼별 설치, 버전 관리 및 제거.

이 페이지에서는 시스템 요구사항, 플랫폼별 설치 세부사항, 업데이트 및 제거에 대해 다룹니다. 첫 번째 세션의 단계별 안내는 [빠른 시작](/docs/ko/quickstart)을 참조하세요. 터미널을 처음 사용하는 경우 [터미널 가이드](/docs/ko/terminal-guide)를 참조하세요.

<h2 id="system-requirements">
  시스템 요구사항
</h2>

Claude Code는 다음 플랫폼 및 구성에서 실행됩니다:

* **운영 체제**:
  * macOS 13.0+
  * Windows 10 1809+ 또는 Windows Server 2019+
  * Ubuntu 20.04+
  * Debian 10+
  * Alpine Linux 3.19+
* **하드웨어**: 4GB 이상 RAM, x64 또는 ARM64 프로세서
* **네트워크**: 인터넷 연결 필수. [네트워크 구성](/docs/ko/network-config#network-access-requirements)을 참조하세요.
* **셸**: Bash, Zsh, PowerShell 또는 CMD.
* **위치**: [Anthropic 지원 국가](https://www.anthropic.com/supported-countries)

<h3 id="additional-dependencies">
  추가 종속성
</h3>

* **ripgrep**: 일반적으로 Claude Code에 포함됩니다. 검색이 실패하면 [검색 문제 해결](/docs/ko/troubleshooting#search-and-discovery-issues)을 참조하세요.

<h2 id="install-claude-code">
  Claude Code 설치
</h2>

<Tip>
  그래픽 인터페이스를 선호하시나요? [Desktop 앱](/docs/ko/desktop-quickstart)을 사용하면 터미널 없이 Claude Code를 사용할 수 있습니다. [macOS](https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code\&utm_medium=docs), [Windows](https://claude.com/download?utm_source=claude_code\&utm_medium=docs) 또는 [Linux](/docs/ko/desktop-linux)용으로 다운로드하세요.

  터미널이 처음이신가요? 단계별 지침은 [터미널 가이드](/docs/ko/terminal-guide)를 참조하세요.
</Tip>

To install Claude Code, use one of the following methods:

<Tabs>
  <Tab title="Native Install (Recommended)">
    **macOS, Linux, WSL:**

    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash
    ```

    **Windows PowerShell:**

    ```powershell theme={null}
    irm https://claude.ai/install.ps1 | iex
    ```

    **Windows CMD:**

    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
    ```

    If you see `The token '&&' is not a valid statement separator`, you're in PowerShell, not CMD. If you see `'irm' is not recognized as an internal or external command`, you're in CMD, not PowerShell. Your prompt shows `PS C:\` when you're in PowerShell and `C:\` without the `PS` when you're in CMD.

    If the install command fails with `syntax error near unexpected token '<'`, a `403`, or another curl error, see [Troubleshoot installation](/docs/en/troubleshoot-install#find-your-error) to match the error to a fix and for alternative install methods.

    [Git for Windows](https://git-scm.com/downloads/win) is recommended on native Windows so Claude Code can use the Bash tool. If Git for Windows is not installed, Claude Code uses PowerShell as the shell tool instead. WSL setups do not need Git for Windows.

    <Info>
      Native installations automatically update in the background to keep you on the latest version.
    </Info>
  </Tab>

  <Tab title="Homebrew">
    ```bash theme={null}
    brew install --cask claude-code
    ```

    Homebrew offers two casks. `claude-code` tracks the stable release channel, which is typically about a week behind and skips releases with major regressions. `claude-code@latest` tracks the latest channel and receives new versions as soon as they ship.

    <Info>
      Homebrew installations do not auto-update. Run `brew upgrade claude-code` or `brew upgrade claude-code@latest`, depending on which cask you installed, to get the latest features and security fixes.
    </Info>
  </Tab>

  <Tab title="WinGet">
    ```powershell theme={null}
    winget install Anthropic.ClaudeCode
    ```

    <Info>
      WinGet installations do not auto-update. Run `winget upgrade Anthropic.ClaudeCode` periodically to get the latest features and security fixes.
    </Info>
  </Tab>
</Tabs>

You can also install with [apt, dnf, or apk](/docs/en/setup#install-with-linux-package-managers) on Debian, Fedora, RHEL, and Alpine.

설치가 완료된 후 작업하려는 프로젝트에서 터미널을 열고 Claude Code를 시작하세요:

```bash theme={null}
claude
```

설치 중에 문제가 발생하면 [설치 및 로그인 문제 해결](/docs/ko/troubleshoot-install)을 참조하세요.

<h3 id="set-up-on-windows">
  Windows에서 설정
</h3>

Claude Code를 Windows에서 기본적으로 실행하거나 WSL 내에서 실행할 수 있습니다. 프로젝트가 위치한 곳과 필요한 기능을 기반으로 선택하세요:

| 옵션           | 필요 사항                                                           | [샌드박싱](/docs/ko/sandboxing) | 사용 시기                      |
| ------------ | --------------------------------------------------------------- | ---------------------- | -------------------------- |
| 네이티브 Windows | 없음; [Git for Windows](https://git-scm.com/downloads/win)는 선택 사항 | 지원되지 않음                | Windows 기본 프로젝트 및 도구       |
| WSL 2        | WSL 2 활성화                                                       | 지원됨                    | Linux 도구 체인 또는 샌드박싱된 명령 실행 |
| WSL 1        | WSL 1 활성화                                                       | 지원되지 않음                | WSL 2를 사용할 수 없는 경우         |

**옵션 1: 네이티브 Windows**

PowerShell 또는 CMD에서 설치 명령을 실행하세요. 관리자로 실행할 필요가 없습니다. [Git for Windows](https://git-scm.com/downloads/win) 설치는 선택 사항입니다. 이는 Git Bash를 제공하여 [Bash 도구](/docs/ko/tools-reference#bash-tool-behavior)를 활성화합니다.

PowerShell 또는 CMD에서 설치하는지 여부는 실행하는 설치 명령에만 영향을 줍니다. 프롬프트는 PowerShell에서 `PS C:\Users\YourName>`을 표시하고 CMD에서는 `PS` 없이 `C:\Users\YourName>`을 표시합니다. 터미널이 처음이라면 [터미널 가이드](/docs/ko/terminal-guide#windows)에서 각 단계를 안내합니다.

설치 후 모든 터미널에서 `claude`를 실행하세요.

* **Git for Windows 없음**, Claude Code는 [PowerShell 도구](/docs/ko/tools-reference#powershell-tool)를 통해 셸 명령을 실행합니다.
* **Git for Windows 포함**, Claude Code는 [Bash 도구](/docs/ko/tools-reference#bash-tool-behavior)에 Git Bash를 사용합니다. Claude Code가 Git Bash를 찾을 수 없으면 [settings.json 파일](/docs/ko/settings)에서 경로를 설정하세요:

  ```json theme={null}
  {
    "env": {
      "CLAUDE_CODE_GIT_BASH_PATH": "C:\\Program Files\\Git\\bin\\bash.exe"
    }
  }
  ```

Git for Windows가 설치되면 PowerShell 도구는 Bash와 함께 추가 옵션으로 점진적으로 출시되고 있습니다. 옵트인하려면 `CLAUDE_CODE_USE_POWERSHELL_TOOL=1`을 설정하거나 옵트아웃하려면 `0`을 설정하세요. 설정 및 제한사항은 [PowerShell 도구](/docs/ko/tools-reference#powershell-tool)를 참조하세요.

**옵션 2: WSL**

WSL 배포판을 열고 위의 [설치 지침](#install-claude-code)에서 Linux 설치 프로그램을 실행하세요. PowerShell 또는 CMD가 아닌 WSL 터미널 내에서 `claude`를 설치하고 실행합니다.

<h3 id="alpine-linux-and-musl-based-distributions">
  Alpine Linux 및 musl 기반 배포판
</h3>

Alpine 및 기타 musl/uClibc 기반 배포판의 네이티브 설치 프로그램에는 `libgcc`, `libstdc++` 및 `ripgrep`이 필요합니다. 배포판의 패키지 관리자를 사용하여 이들을 설치한 후 `USE_BUILTIN_RIPGREP=0`을 설정하세요.

이 예제는 Alpine에 필요한 패키지를 설치합니다:

```bash theme={null}
apk add libgcc libstdc++ ripgrep
```

그런 다음 [`settings.json`](/docs/ko/settings#available-settings) 파일에서 `USE_BUILTIN_RIPGREP`을 `0`으로 설정하세요:

```json theme={null}
{
  "env": {
    "USE_BUILTIN_RIPGREP": "0"
  }
}
```

<h2 id="verify-your-installation">
  설치 확인
</h2>

설치 후 Claude Code가 작동하는지 확인하세요:

```bash theme={null}
claude --version
```

이 명령이 `command not found` 또는 다른 오류로 실패하면 [설치 및 로그인 문제 해결](/docs/ko/troubleshoot-install)을 참조하세요.

설치 및 구성을 더 자세히 확인하려면 [`claude doctor`](/docs/ko/troubleshooting#get-more-help)를 실행하세요:

```bash theme={null}
claude doctor
```

<h2 id="authenticate">
  인증
</h2>

Claude Code는 Pro, Max, Team, Enterprise 또는 Console 계정이 필요합니다. 무료 Claude.ai 플랜에는 Claude Code 액세스가 포함되지 않습니다. [Amazon Bedrock](/docs/ko/amazon-bedrock), [Google Cloud의 Agent Platform](/docs/ko/google-vertex-ai) 또는 [Microsoft Foundry](/docs/ko/microsoft-foundry)와 같은 타사 API 제공자와 함께 Claude Code를 사용할 수도 있습니다.

설치 후 `claude`를 실행하고 브라우저 프롬프트를 따라 로그인하세요. 모든 계정 유형 및 팀 설정 옵션은 [인증](/docs/ko/authentication)을 참조하세요.

<h2 id="update-claude-code">
  Claude Code 업데이트
</h2>

네이티브 설치는 백그라운드에서 자동으로 업데이트됩니다. [릴리스 채널을 구성](#configure-release-channel)하여 즉시 업데이트를 받을지 또는 지연된 안정적인 일정으로 받을지 제어하거나, [자동 업데이트를 비활성화](#disable-auto-updates)할 수 있습니다. Homebrew, WinGet 및 [Linux 패키지 관리자](#install-with-linux-package-managers) 설치는 기본적으로 수동 업데이트가 필요합니다.

<h3 id="auto-updates">
  자동 업데이트
</h3>

Claude Code는 시작 시 및 실행 중에 주기적으로 업데이트를 확인합니다. 업데이트는 백그라운드에서 다운로드 및 설치되며 다음 번에 Claude Code를 시작할 때 적용됩니다.

`claude doctor`를 실행하여 가장 최근 업데이트 시도의 결과를 확인하세요.

macOS 및 Linux에서 네이티브 설치 프로그램은 `~/.local/bin/claude`의 런처를 `~/.local/share/claude/versions/`로의 심볼릭 링크로 관리합니다. 해당 런처를 자신의 스크립트 또는 심볼릭 링크로 바꾸면 자동 업데이트 및 `claude update`는 이를 제자리에 두고 있습니다. 새 버전은 여전히 `versions/` 디렉터리 아래에 설치되며 런처가 실행할 버전을 결정합니다. v2.1.207 이전에는 자동 업데이터가 모든 업데이트에서 해당 경로의 사용자 정의 런처를 자신의 심볼릭 링크로 바꿨습니다.

사용자 정의 런처를 사용하면 Claude Code는 런처가 필요한 버전을 알 수 없기 때문에 설치된 모든 버전을 디스크에 유지합니다. `claude doctor`는 네이티브 설치 프로그램이 만들지 않은 런처를 보고합니다.

Claude Code가 런처를 다시 관리하도록 하려면 `~/.local/bin/claude`를 제거하고 `claude update`를 실행하세요.

npm 전역 설치가 npm 전역 디렉터리를 쓸 수 없기 때문에 자동 업데이트할 수 없는 경우, Claude Code는 시작 시 일회성 알림을 표시하고 `claude doctor`는 사용 가능한 수정 사항을 나열합니다. 자세한 내용은 [설치 중 권한 오류](/docs/ko/troubleshoot-install#permission-errors-during-installation)를 참조하세요.

<Note>
  Homebrew, WinGet, apt, dnf 및 apk 설치는 기본적으로 자동으로 업데이트되지 않습니다. Homebrew 및 WinGet에 대해 옵트인하려면 아래를 참조하세요. Homebrew를 수동으로 업그레이드하려면 설치한 cask에 따라 `brew upgrade claude-code` 또는 `brew upgrade claude-code@latest`를 실행하세요. WinGet의 경우 `winget upgrade Anthropic.ClaudeCode`를 실행하세요. Linux 패키지 관리자의 경우 [Linux 패키지 관리자로 설치](#install-with-linux-package-managers)의 업그레이드 명령을 참조하세요.

  Claude Code가 Homebrew 또는 WinGet에서 업그레이드 명령을 실행하도록 하려면 [`CLAUDE_CODE_PACKAGE_MANAGER_AUTO_UPDATE`](/docs/ko/env-vars)를 `1`로 설정하세요. Claude Code는 새 버전을 사용할 수 있을 때 백그라운드에서 업그레이드를 실행하고 성공 시 재시작 프롬프트를 표시합니다. 업그레이드는 Claude Code 패키지만 대상으로 하며 설치한 다른 소프트웨어에는 영향을 주지 않습니다.

  WinGet에서는 Claude Code가 실행 중일 때 Windows가 실행 파일을 잠그기 때문에 업그레이드가 실패할 수 있습니다. 이 경우 Claude Code는 수동 명령을 대신 표시합니다. apt, dnf 및 apk는 이러한 명령이 상승된 권한이 필요하기 때문에 계속 수동 업그레이드가 필요합니다.

  **알려진 문제:** Claude Code는 새 버전이 이러한 패키지 관리자에서 사용 가능하기 전에 업데이트를 알릴 수 있습니다. 업그레이드가 실패하면 잠시 기다렸다가 나중에 다시 시도하세요.

  Homebrew는 업그레이드 후 디스크에 이전 버전을 유지합니다. 디스크 공간을 확보하려면 주기적으로 `brew cleanup`을 실행하세요.
</Note>

<h3 id="configure-release-channel">
  릴리스 채널 구성
</h3>

`autoUpdatesChannel` 설정으로 Claude Code가 자동 업데이트 및 `claude update`에 대해 따르는 릴리스 채널을 제어하세요:

* `"latest"`, 기본값: 새 기능이 릴리스되는 즉시 받기
* `"stable"`: 일반적으로 약 1주일 된 버전을 사용하여 주요 회귀가 있는 릴리스 건너뛰기

이를 `/config` → **자동 업데이트 채널**을 통해 구성하거나 [settings.json 파일](/docs/ko/settings)에 추가하세요:

```json theme={null}
{
  "autoUpdatesChannel": "stable"
}
```

엔터프라이즈 배포의 경우 [관리 설정](/docs/ko/permissions#managed-settings)을 사용하여 조직 전체에서 일관된 릴리스 채널을 적용할 수 있습니다.

Homebrew 설치는 이 설정 대신 cask 이름으로 채널을 선택합니다: `claude-code`는 안정적인 버전을 추적하고 `claude-code@latest`는 최신 버전을 추적합니다.

<h3 id="pin-a-minimum-version">
  최소 버전 고정
</h3>

`minimumVersion` 설정은 하한을 설정합니다. 백그라운드 자동 업데이트 및 `claude update`는 이 값 이하의 버전 설치를 거부하므로 `"stable"` 채널로 이동해도 이미 최신 `"latest"` 빌드에 있는 경우 다운그레이드되지 않습니다.

`/config`를 통해 `"latest"`에서 `"stable"`로 전환하면 현재 버전을 유지하거나 다운그레이드를 허용하라는 메시지가 표시됩니다. 현재 버전을 유지하도록 선택하면 `minimumVersion`이 해당 버전으로 설정됩니다. `"latest"`로 다시 전환하면 이를 지웁니다.

[settings.json 파일](/docs/ko/settings)에 추가하여 하한을 명시적으로 고정하세요:

```json theme={null}
{
  "autoUpdatesChannel": "stable",
  "minimumVersion": "2.1.100"
}
```

[관리 설정](/docs/ko/permissions#managed-settings)에서 이는 사용자 및 프로젝트 설정이 재정의할 수 없는 조직 전체 최소값을 적용합니다.

`minimumVersion` 핀은 업데이트만 제한합니다. Claude Code가 버전 범위 외에서 시작되지 않도록 하려면 관리 설정 `requiredMinimumVersion` 및 `requiredMaximumVersion`을 대신 사용하세요. 업데이트는 또한 `requiredMaximumVersion` 상한을 준수합니다. [사용 가능한 설정](/docs/ko/settings#available-settings)을 참조하세요.

<h3 id="disable-auto-updates">
  자동 업데이트 비활성화
</h3>

[`settings.json`](/docs/ko/settings#available-settings) 파일의 `env` 키에서 `DISABLE_AUTOUPDATER`를 `"1"`로 설정하세요:

```json theme={null}
{
  "env": {
    "DISABLE_AUTOUPDATER": "1"
  }
}
```

`DISABLE_AUTOUPDATER`는 백그라운드 확인만 중지합니다. `claude update` 및 `claude install`은 계속 작동합니다. 수동 업데이트를 포함한 모든 업데이트 경로를 차단하려면 [`DISABLE_UPDATES`](/docs/ko/env-vars)를 대신 설정하세요. Claude Code를 자신의 채널을 통해 배포하고 사용자가 제공하는 버전에 머물러야 할 때 이를 사용하세요.

<h3 id="update-manually">
  수동으로 업데이트
</h3>

다음 백그라운드 확인을 기다리지 않고 즉시 업데이트를 적용하려면 다음을 실행하세요:

```bash theme={null}
claude update
```

<h2 id="advanced-installation-options">
  고급 설치 옵션
</h2>

이러한 옵션은 버전 고정, Linux 패키지 관리자, npm 및 바이너리 무결성 확인을 위한 것입니다.

<h3 id="install-a-specific-version">
  특정 버전 설치
</h3>

네이티브 설치 프로그램은 특정 버전 번호 또는 릴리스 채널(`latest` 또는 `stable`)을 허용합니다. 설치 시 선택한 채널이 자동 업데이트의 기본값이 됩니다. 자세한 내용은 [릴리스 채널 구성](#configure-release-channel)을 참조하세요.

최신 버전을 설치하려면(기본값):

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    irm https://claude.ai/install.ps1 | iex
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
    ```
  </Tab>
</Tabs>

안정적인 버전을 설치하려면:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash -s stable
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    & ([scriptblock]::Create((irm https://claude.ai/install.ps1))) stable
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd stable && del install.cmd
    ```
  </Tab>
</Tabs>

특정 버전 번호를 설치하려면:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash -s 2.1.89
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    & ([scriptblock]::Create((irm https://claude.ai/install.ps1))) 2.1.89
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd 2.1.89 && del install.cmd
    ```
  </Tab>
</Tabs>

<h3 id="install-with-linux-package-managers">
  Linux 패키지 관리자로 설치
</h3>

Claude Code는 서명된 apt, dnf 및 apk 저장소를 게시합니다. 각 저장소는 두 가지 채널을 제공합니다. `stable`은 일반적으로 약 1주일 된 버전을 제공하며 주요 회귀가 있는 릴리스를 건너뛰고, `latest`는 모든 릴리스를 출시되는 즉시 제공합니다. 아래 명령은 대부분의 사용자에게 적합한 `stable` 채널을 구성합니다. 각 탭에는 `latest` 저장소 URL도 표시됩니다. 패키지 관리자 설치는 Claude Code를 통해 자동 업데이트되지 않습니다. 업데이트는 일반적인 시스템 업그레이드 워크플로우를 통해 제공됩니다.

모든 저장소는 [Claude Code 릴리스 서명 키](#binary-integrity-and-code-signing)로 서명됩니다. 키를 신뢰하기 전에 각 탭에 설명된 대로 확인하세요.

<Tabs>
  <Tab title="apt">
    Debian 및 Ubuntu용입니다. 설치 명령은 아래에서 `curl`로 서명 키를 다운로드합니다. 새로운 Debian 및 Ubuntu 설치에는 `curl`이 포함되지 않을 수 있습니다. 다운로드가 `sudo: curl: command not found`로 실패하면 먼저 curl을 설치하세요:

    ```bash theme={null}
    sudo apt install curl
    ```

    다음 명령은 `stable` 채널을 구성합니다:

    ```bash theme={null}
    sudo install -d -m 0755 /etc/apt/keyrings
    sudo curl -fsSL https://downloads.claude.ai/keys/claude-code.asc \
      -o /etc/apt/keyrings/claude-code.asc
    echo "deb [signed-by=/etc/apt/keyrings/claude-code.asc] https://downloads.claude.ai/claude-code/apt/stable stable main" \
      | sudo tee /etc/apt/sources.list.d/claude-code.list
    sudo apt update
    sudo apt install claude-code
    ```

    대신 `latest` 채널을 사용하려면 URL 경로와 제품군 이름이 모두 변경됩니다. 이 `deb` 줄을 사용하세요:

    ```bash theme={null}
    echo "deb [signed-by=/etc/apt/keyrings/claude-code.asc] https://downloads.claude.ai/claude-code/apt/latest latest main" \
      | sudo tee /etc/apt/sources.list.d/claude-code.list
    ```

    신뢰하기 전에 GPG 키 지문을 확인하세요: `gpg --show-keys /etc/apt/keyrings/claude-code.asc`는 `31DD DE24 DDFA B679 F42D 7BD2 BAA9 29FF 1A7E CACE`를 보고해야 합니다.

    나중에 업그레이드하려면 `sudo apt update && sudo apt upgrade claude-code`를 실행하세요.
  </Tab>

  <Tab title="dnf">
    Fedora 및 RHEL용입니다. 다음 명령은 `stable` 채널을 구성합니다:

    ```bash theme={null}
    sudo tee /etc/yum.repos.d/claude-code.repo <<'EOF'
    [claude-code]
    name=Claude Code
    baseurl=https://downloads.claude.ai/claude-code/rpm/stable
    enabled=1
    gpgcheck=1
    gpgkey=https://downloads.claude.ai/keys/claude-code.asc
    EOF
    sudo dnf install claude-code
    ```

    대신 `latest` 채널을 사용하려면 `baseurl`을 `latest` 저장소로 설정하세요:

    ```ini theme={null}
    baseurl=https://downloads.claude.ai/claude-code/rpm/latest
    ```

    dnf는 첫 설치 시 키를 다운로드하고 지문을 확인하도록 요청합니다. `31DD DE24 DDFA B679 F42D 7BD2 BAA9 29FF 1A7E CACE`와 일치하는지 확인한 후 수락하세요.

    나중에 업그레이드하려면 `sudo dnf upgrade claude-code`를 실행하세요.
  </Tab>

  <Tab title="apk">
    Alpine Linux용입니다. 다음 명령은 `stable` 채널을 구성합니다:

    ```sh theme={null}
    wget -O /etc/apk/keys/claude-code.rsa.pub \
      https://downloads.claude.ai/keys/claude-code.rsa.pub
    echo "https://downloads.claude.ai/claude-code/apk/stable" >> /etc/apk/repositories
    apk add claude-code
    ```

    `latest` 채널로 전환하려면 `stable` 저장소 줄을 제거하고 `latest` 저장소를 추가하세요:

    ```sh theme={null}
    sed -i '\|downloads.claude.ai/claude-code/apk/stable|d' /etc/apk/repositories
    echo "https://downloads.claude.ai/claude-code/apk/latest" >> /etc/apk/repositories
    ```

    `sha256sum /etc/apk/keys/claude-code.rsa.pub`로 다운로드한 키를 확인하세요. 이는 `395759c1f7449ef4cdef305a42e820f3c766d6090d142634ebdb049f113168b6`을 보고해야 합니다.

    나중에 업그레이드하려면 `apk update && apk upgrade claude-code`를 실행하세요.
  </Tab>
</Tabs>

<h3 id="install-with-npm">
  npm으로 설치
</h3>

Claude Code를 전역 npm 패키지로 설치할 수도 있습니다. v2.1.198부터 npm 패키지에는 [Node.js 22 이상](https://nodejs.org/en/download)이 필요합니다. 이전 Node.js 버전에서는 npm이 설치 중에 실패하는 대신 `EBADENGINE` 경고를 인쇄합니다. 설치가 완료되고 `claude`는 여전히 실행됩니다. 패키지가 런타임에 Node.js를 사용하지 않는 네이티브 바이너리를 다운로드하기 때문입니다.

```bash theme={null}
npm install -g @anthropic-ai/claude-code
```

npm 패키지는 독립 실행형 설치 프로그램과 동일한 네이티브 바이너리를 설치합니다. npm은 `@anthropic-ai/claude-code-darwin-arm64`와 같은 플랫폼별 선택적 종속성을 통해 바이너리를 가져오고 설치 후 단계가 이를 제자리에 연결합니다. 설치된 `claude` 바이너리는 자체적으로 Node를 호출하지 않습니다.

지원되는 npm 설치 플랫폼은 `darwin-arm64`, `darwin-x64`, `linux-x64`, `linux-arm64`, `linux-x64-musl`, `linux-arm64-musl`, `win32-x64` 및 `win32-arm64`입니다. 패키지 관리자는 선택적 종속성을 허용해야 합니다. 설치 후 바이너리가 누락된 경우 [문제 해결](/docs/ko/troubleshoot-install#native-binary-not-found-after-npm-install)을 참조하세요.

npm 설치를 업그레이드하려면 `npm install -g @anthropic-ai/claude-code@latest`를 실행하세요. `npm update -g`는 원래 설치의 semver 범위를 존중하며 최신 릴리스로 이동하지 않을 수 있으므로 피하세요.

<Warning>
  `sudo npm install -g`를 사용하지 마세요. 이는 권한 문제 및 보안 위험으로 이어질 수 있습니다. 권한 오류가 발생하면 [설치 중 권한 오류 문제 해결](/docs/ko/troubleshoot-install#permission-errors-during-installation)을 참조하세요.
</Warning>

<h3 id="binary-integrity-and-code-signing">
  바이너리 무결성 및 코드 서명
</h3>

각 릴리스는 모든 플랫폼 바이너리에 대한 SHA256 체크섬을 포함하는 `manifest.json`을 게시합니다. 매니페스트는 Anthropic GPG 키로 서명되므로 매니페스트의 서명을 확인하면 이것이 나열하는 모든 바이너리를 전이적으로 확인합니다.

<h4 id="verify-the-manifest-signature">
  매니페스트 서명 확인
</h4>

단계 1-3에는 `gpg` 및 `curl`이 있는 POSIX 셸이 필요합니다. Windows에서는 Git Bash 또는 WSL에서 실행하세요. 단계 4에는 PowerShell 옵션이 포함됩니다.

<Steps>
  <Step title="공개 키 다운로드 및 가져오기">
    릴리스 서명 키는 고정 URL에 게시됩니다.

    ```bash theme={null}
    curl -fsSL https://downloads.claude.ai/keys/claude-code.asc | gpg --import
    ```

    가져온 키의 지문을 표시합니다.

    ```bash theme={null}
    gpg --fingerprint security@anthropic.com
    ```

    출력에 이 지문이 포함되어 있는지 확인하세요:

    ```text theme={null}
    31DD DE24 DDFA B679 F42D  7BD2 BAA9 29FF 1A7E CACE
    ```
  </Step>

  <Step title="매니페스트 및 서명 다운로드">
    `VERSION`을 확인하려는 릴리스로 설정하세요.

    ```bash theme={null}
    REPO=https://downloads.claude.ai/claude-code-releases
    VERSION=2.1.89
    curl -fsSLO "$REPO/$VERSION/manifest.json"
    curl -fsSLO "$REPO/$VERSION/manifest.json.sig"
    ```
  </Step>

  <Step title="서명 확인">
    매니페스트에 대해 분리된 서명을 확인합니다.

    ```bash theme={null}
    gpg --verify manifest.json.sig manifest.json
    ```

    유효한 결과는 `Good signature from "Anthropic Claude Code Release Signing <security@anthropic.com>"`을 보고합니다.

    `gpg`는 또한 새로 가져온 키에 대해 `WARNING: This key is not certified with a trusted signature!`을 인쇄합니다. 이는 예상된 것입니다. `Good signature` 줄은 암호화 확인이 통과했음을 확인합니다. 단계 1의 지문 비교는 키 자체가 진정함을 확인합니다.
  </Step>

  <Step title="바이너리를 매니페스트와 비교">
    현재 디렉터리의 `claude` 바이너리의 SHA256 체크섬을 `manifest.json`의 `platforms.<platform>.checksum` 아래에 나열된 값과 비교합니다. 설치된 네이티브 바이너리를 대신 확인하려면 `~/.local/share/claude/versions/VERSION`에 대해 명령을 실행하고 VERSION을 단계 2에서 설정한 릴리스로 바꾸세요.

    <Tabs>
      <Tab title="Linux">
        ```bash theme={null}
        sha256sum claude
        ```
      </Tab>

      <Tab title="macOS">
        ```bash theme={null}
        shasum -a 256 claude
        ```
      </Tab>

      <Tab title="Windows PowerShell">
        ```powershell theme={null}
        (Get-FileHash claude.exe -Algorithm SHA256).Hash.ToLower()
        ```
      </Tab>
    </Tabs>
  </Step>
</Steps>

<Note>
  매니페스트 서명은 `2.1.89` 이상의 릴리스에 사용 가능합니다. 이전 릴리스는 분리된 서명 없이 `manifest.json`에 체크섬을 게시합니다.
</Note>

<h4 id="platform-code-signatures">
  플랫폼 코드 서명
</h4>

서명된 매니페스트 외에도 개별 바이너리는 지원되는 플랫폼에서 플랫폼 기본 코드 서명을 수행합니다.

* **macOS**: "Anthropic PBC"에서 서명하고 Apple에서 공증합니다. `codesign --verify --verbose ./claude`로 확인하세요.
* **Windows**: "Anthropic, PBC"에서 서명합니다. `Get-AuthenticodeSignature .\claude.exe`로 확인하세요.
* **Linux**: 바이너리는 개별적으로 코드 서명되지 않습니다. `claude-code-releases` 버킷에서 직접 다운로드하거나 네이티브 설치 프로그램을 사용하는 경우 위의 매니페스트 서명으로 무결성을 확인하세요. [apt, dnf 또는 apk](#install-with-linux-package-managers)로 설치하는 경우 패키지 관리자가 저장소 서명 키를 사용하여 서명을 자동으로 확인합니다.

<h2 id="uninstall-claude-code">
  Claude Code 제거
</h2>

Claude Code를 제거하려면 설치 방법에 따른 지침을 따르세요. 제거 후에도 `claude`가 계속 실행되면 두 번째 설치 또는 이전 설치 프로그램의 남은 셸 별칭이 있을 가능성이 높습니다. [충돌하는 설치 확인](/docs/ko/troubleshoot-install#check-for-conflicting-installations)을 참조하여 이를 찾아 제거하세요.

<h3 id="native-installation">
  네이티브 설치
</h3>

Claude Code 바이너리 및 버전 파일을 제거하세요:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    rm -f ~/.local/bin/claude
    rm -rf ~/.local/share/claude
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    Remove-Item -Path "$env:USERPROFILE\.local\bin\claude.exe" -Force
    Remove-Item -Path "$env:USERPROFILE\.local\share\claude" -Recurse -Force
    ```
  </Tab>
</Tabs>

<h3 id="homebrew-installation">
  Homebrew 설치
</h3>

설치한 Homebrew cask를 제거하세요. 안정적인 cask를 설치한 경우:

```bash theme={null}
brew uninstall --cask claude-code
```

최신 cask를 설치한 경우:

```bash theme={null}
brew uninstall --cask claude-code@latest
```

<h3 id="winget-installation">
  WinGet 설치
</h3>

WinGet 패키지를 제거하세요:

```powershell theme={null}
winget uninstall Anthropic.ClaudeCode
```

<h3 id="apt-/-dnf-/-apk">
  apt / dnf / apk
</h3>

패키지 및 저장소 구성을 제거하세요:

<Tabs>
  <Tab title="apt">
    ```bash theme={null}
    sudo apt remove claude-code
    sudo rm /etc/apt/sources.list.d/claude-code.list /etc/apt/keyrings/claude-code.asc
    ```
  </Tab>

  <Tab title="dnf">
    ```bash theme={null}
    sudo dnf remove claude-code
    sudo rm /etc/yum.repos.d/claude-code.repo
    ```
  </Tab>

  <Tab title="apk">
    ```sh theme={null}
    apk del claude-code
    sed -i '\|downloads.claude.ai/claude-code/apk|d' /etc/apk/repositories
    rm /etc/apk/keys/claude-code.rsa.pub
    ```
  </Tab>
</Tabs>

<h3 id="npm">
  npm
</h3>

전역 npm 패키지를 제거하세요:

```bash theme={null}
npm uninstall -g @anthropic-ai/claude-code
```

<h3 id="remove-configuration-files">
  구성 파일 제거
</h3>

<Warning>
  구성 파일을 제거하면 모든 설정, 허용된 도구, MCP 서버 구성 및 세션 기록이 삭제됩니다.
</Warning>

VS Code 확장 프로그램, JetBrains 플러그인 및 Desktop 앱도 `~/.claude/`에 씁니다. 이들 중 하나라도 여전히 설치되어 있으면 다음 실행 시 디렉토리가 다시 생성됩니다. Claude Code를 완전히 제거하려면 이 파일을 삭제하기 전에 [VS Code 확장 프로그램](/docs/ko/vs-code#uninstall-the-extension), JetBrains 플러그인 및 Desktop 앱을 제거하세요.

Claude Code 설정 및 캐시된 데이터를 제거하려면:

<Tabs>
  <Tab title="macOS, Linux, WSL">
    ```bash theme={null}
    # 사용자 설정 및 상태 제거
    rm -rf ~/.claude
    rm ~/.claude.json

    # 프로젝트별 설정 제거(프로젝트 디렉토리에서 실행)
    rm -rf .claude
    rm -f .mcp.json
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    # 사용자 설정 및 상태 제거
    Remove-Item -Path "$env:USERPROFILE\.claude" -Recurse -Force
    Remove-Item -Path "$env:USERPROFILE\.claude.json" -Force

    # 프로젝트별 설정 제거(프로젝트 디렉토리에서 실행)
    Remove-Item -Path ".claude" -Recurse -Force
    Remove-Item -Path ".mcp.json" -Force
    ```
  </Tab>
</Tabs>
