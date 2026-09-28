> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 설치 및 로그인 문제 해결

> Claude Code 설치 또는 로그인 시 command not found, PATH, 권한, 네트워크 및 인증 오류를 수정합니다.

설치가 실패하거나 로그인할 수 없는 경우 아래에서 오류를 찾으세요. Claude Code가 작동한 후 런타임 문제의 경우 [문제 해결](/docs/ko/troubleshooting)을 참조하세요. 설정이 적용되지 않거나 hooks가 실행되지 않는 등의 구성 문제의 경우 [구성 디버깅](/docs/ko/debug-your-config)을 참조하세요.

<h2 id="find-your-error">
  오류 찾기
</h2>

표시되는 오류 메시지 또는 증상을 수정 사항과 일치시키세요:

| 표시되는 내용                                                                                         | 해결책                                                                                                                                  |
| :---------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------- |
| `command not found: claude` 또는 `'claude' is not recognized`                                     | [PATH 수정](#command-not-found-claude-after-installation)                                                                              |
| `syntax error near unexpected token '<'`                                                        | [설치 스크립트가 HTML 반환](#install-script-returns-html-instead-of-a-shell-script)                                                           |
| `curl: (22) The requested URL returned error: 403`                                              | [설치 스크립트가 403 반환](#install-script-returns-html-instead-of-a-shell-script)                                                            |
| `curl: (23)` 또는 `curl: (56) Failure writing output to destination`                              | [연결성 확인 또는 대체 설치 프로그램 사용](#curl-56-failure-writing-output-to-destination)                                                            |
| Linux에서 설치 중 `Killed` 또는 `Installation was killed before it could finish (exit code 137)`       | [메모리 확보 또는 스왑 공간 추가](#install-killed-on-low-memory-linux-servers)                                                                    |
| 설치 중 `Raw mode is not supported`                                                                | [설치 프로그램 다시 실행](#raw-mode-is-not-supported-during-install)                                                                           |
| `TLS connect error` 또는 `SSL/TLS secure channel`                                                 | [CA 인증서 업데이트](#tls-or-ssl-connection-errors)                                                                                         |
| `Failed to fetch version` 또는 다운로드 서버에 도달할 수 없음                                                  | [네트워크 및 프록시 설정 확인](#check-network-connectivity)                                                                                      |
| `irm is not recognized` 또는 `The token '&&' is not a valid statement separator`                  | [셸에 맞는 명령 사용](#wrong-install-command-on-windows)                                                                                     |
| `Cask 'claude-code' is unavailable: No Cask with this name exists`                              | [Homebrew 업데이트](#homebrew-cask-unavailable-or-outdated)                                                                              |
| `'bash' is not recognized as the name of a cmdlet`                                              | [Windows 설치 프로그램 명령 사용](#wrong-install-command-on-windows)                                                                           |
| `A parameter cannot be found that matches parameter name 'fsSL'`                                | [Windows 설치 프로그램 명령 사용](#wrong-install-command-on-windows)                                                                           |
| `Claude Code on Windows requires either Git for Windows (for bash) or PowerShell`               | [셸 설치](#claude-code-on-windows-requires-either-git-for-windows-for-bash-or-powershell)                                               |
| `Claude Code does not support 32-bit Windows`                                                   | [Windows PowerShell 열기, x86 항목 아님](#claude-code-does-not-support-32-bit-windows)                                                     |
| `The process cannot access the file ... because it is being used by another process`            | [다운로드 폴더 지우기 및 다시 시도](#the-process-cannot-access-the-file-during-windows-install)                                                    |
| `Error loading shared library`                                                                  | [시스템에 맞는 잘못된 바이너리 변형](#linux-musl-or-glibc-binary-mismatch)                                                                          |
| `Illegal instruction`                                                                           | [아키텍처 또는 CPU 명령어 세트 불일치](#illegal-instruction)                                                                                       |
| WSL에서 `cannot execute binary file: Exec format error`                                           | [WSL1 네이티브 바이너리 회귀](#exec-format-error-on-wsl1)                                                                                      |
| PowerShell 설치 프로그램이 완료되지만 `claude`를 찾을 수 없거나 이전 버전 표시                                           | [설치 디렉터리를 PATH에 추가](#verify-your-path), 그런 다음 새 터미널 열기                                                                               |
| macOS에서 `dyld: Symbol not found`, `dyld: cannot load` 또는 `Abort trap`                           | [바이너리 비호환성](#dyld-cannot-load-on-macos)                                                                                              |
| `claude update`가 `Checking for updates` 후 중단되거나 `claude doctor`가 출력 없이 중단됨                      | [셸 구성 경로의 디렉터리 이동](#claude-update-or-claude-doctor-hangs)                                                                            |
| `Invoke-Expression` 또는 `iex` 구문 분석 오류로 HTML 태그 또는 CSS 인용, 또는 `ParseException`이 있는 `ParserError` | [설치 스크립트가 HTML 반환](#install-script-returns-html-instead-of-a-shell-script)                                                           |
| `running scripts is disabled on this system` 또는 `PSSecurityException`                           | [npm shim이 실행되도록 허용](#running-scripts-is-disabled-on-this-system)                                                                    |
| `Error: claude native binary not installed`                                                     | [npm 설치 완료](#native-binary-not-found-after-npm-install)                                                                              |
| 업데이트 또는 재설치 중 `npm error code ENOTEMPTY`                                                        | [남은 패키지 디렉터리 제거](#npm-enotempty-during-update-or-reinstall)                                                                          |
| Windows에서 설치 명령이 스크립트 텍스트를 인쇄하고 아무것도 설치되지 않음                                                    | [전체 설치 명령 실행](#wrong-install-command-on-windows)                                                                                     |
| `App unavailable in region`                                                                     | Claude Code는 귀국에서 사용할 수 없습니다. [지원되는 국가](https://www.anthropic.com/supported-countries)를 참조하세요.                                       |
| `unable to get local issuer certificate`                                                        | [회사 CA 인증서 구성](#tls-or-ssl-connection-errors)                                                                                        |
| `OAuth error` 또는 `403 Forbidden`                                                                | [인증 수정](#login-and-authentication)                                                                                                   |
| 설정 중 `Unable to connect to Anthropic services`                                                  | 오류 참조에서 [Anthropic 서비스에 연결할 수 없음](/docs/ko/errors#unable-to-connect-to-anthropic-services)을 참조하세요                                         |
| `Could not load the default credentials` 또는 `Could not load credentials from any providers`     | [Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry 자격증명](#bedrock-agent-platform-or-foundry-credentials-not-loading) |
| `ChainedTokenCredential authentication failed` 또는 `CredentialUnavailableError`                  | [Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry 자격증명](#bedrock-agent-platform-or-foundry-credentials-not-loading) |
| `API Error: 500`, `529 Overloaded`, `429` 또는 위에 나열되지 않은 기타 4xx 및 5xx 오류                         | [오류 참조](/docs/ko/errors)를 참조하세요                                                                                                           |

문제가 나열되지 않은 경우 아래의 진단 검사를 수행하여 원인을 좁혀보세요.

<Tip>
  터미널을 완전히 건너뛰고 싶다면 [Claude Code Desktop 앱](/docs/ko/desktop-quickstart)을 사용하여 그래픽 인터페이스를 통해 Claude Code를 설치하고 사용할 수 있습니다. [macOS](https://claude.ai/api/desktop/darwin/universal/dmg/latest/redirect?utm_source=claude_code\&utm_medium=docs) 또는 [Windows](https://claude.com/download?utm_source=claude_code\&utm_medium=docs)용으로 다운로드하고 명령줄 설정 없이 코딩을 시작하세요. Linux에서는 [Linux 설치 지침](/docs/ko/desktop-linux)을 따라 apt로 앱을 설치하세요.
</Tip>

<h2 id="run-diagnostic-checks">
  진단 검사 실행
</h2>

<h3 id="check-network-connectivity">
  네트워크 연결성 확인
</h3>

설치 프로그램은 `downloads.claude.ai`에서 다운로드합니다. 도달할 수 있는지 확인하세요:

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    curl -sI https://downloads.claude.ai/claude-code-releases/latest
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    curl.exe -sI https://downloads.claude.ai/claude-code-releases/latest
    ```

    PowerShell은 `curl`을 `Invoke-WebRequest`로 별칭 지정하며, 이는 `-sI` 플래그를 거부합니다. 따라서 `curl.exe`를 명시적으로 호출하세요.
  </Tab>
</Tabs>

첫 번째 줄에 `200` 상태가 표시되면 서버에 도달했습니다. macOS 및 Linux에서는 `HTTP/2 200`이 표시되고, Windows에 포함된 `curl.exe`에서는 `HTTP/1.1 200 OK`가 표시됩니다. 다른 결과는 원인을 나타냅니다:

* `403`: 일반적으로 호스트를 차단하는 프록시 또는 네트워크 필터이거나, Claude Code가 [해당 지역에서 사용 불가능](https://www.anthropic.com/supported-countries)합니다.
* `5xx`: 일반적으로 임시 서비스 문제입니다. 몇 분 기다렸다가 다시 시도하세요.

출력이 없거나 `Could not resolve host` 또는 연결 시간 초과가 표시되면 네트워크가 연결을 차단하고 있습니다. 일반적인 원인:

* `downloads.claude.ai`를 차단하는 회사 방화벽 또는 프록시
* 지역 네트워크 제한: VPN 또는 대체 네트워크 시도
* TLS/SSL 문제: 시스템의 CA 인증서를 업데이트하거나 `HTTPS_PROXY`가 구성되어 있는지 확인

회사 프록시 뒤에 있는 경우 설치하기 전에 `HTTPS_PROXY` 및 `HTTP_PROXY`를 프록시 주소로 설정하세요. 프록시 URL을 모르는 경우 IT 팀에 문의하거나 브라우저의 프록시 설정을 확인하세요.

이 예제는 두 프록시 변수를 설정한 다음 프록시를 통해 설치 프로그램을 실행합니다:

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    export HTTP_PROXY=http://proxy.example.com:8080
    export HTTPS_PROXY=http://proxy.example.com:8080
    curl -fsSL https://claude.ai/install.sh | bash
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    $env:HTTP_PROXY = 'http://proxy.example.com:8080'
    $env:HTTPS_PROXY = 'http://proxy.example.com:8080'
    irm https://claude.ai/install.ps1 | iex
    ```
  </Tab>
</Tabs>

<h3 id="verify-your-path">
  PATH 확인
</h3>

설치가 성공했지만 `claude`를 실행할 때 `command not found` 또는 `not recognized` 오류가 발생하면 설치 디렉토리가 PATH에 없습니다. 셸은 PATH에 나열된 디렉토리에서 프로그램을 검색하고 설치 프로그램은 macOS/Linux에서 `~/.local/bin/claude`에 또는 Windows에서 `%USERPROFILE%\.local\bin\claude.exe`에 `claude`를 배치합니다.

<Note>
  [VS Code 확장](/docs/ko/vs-code)은 `claude`를 이 위치에 배치하지 않습니다. 확장 디렉토리 내에 CLI의 개인 복사본을 번들로 제공하며 자체 채팅 패널용으로 사용하고 PATH에 추가하지 않습니다. 확장만 설치한 경우 `~/.local/bin/claude`가 존재하지 않습니다. [독립 실행형 설치](/docs/ko/setup)를 실행하여 터미널에서 `claude`를 사용한 다음 아래를 계속하세요.
</Note>

PATH 항목을 나열하고 `local/bin`을 필터링하여 설치 디렉토리가 PATH에 있는지 확인하세요:

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    echo $PATH | tr ':' '\n' | grep -Fx "$HOME/.local/bin"
    ```

    이것이 `/Users/you/.local/bin` 또는 `/home/you/.local/bin`을 인쇄하면 디렉토리가 PATH에 있으므로 [충돌하는 설치 확인](#check-for-conflicting-installations)으로 건너뛸 수 있습니다. 출력이 없으면 셸 구성에 추가하세요.

    macOS의 기본값인 Zsh의 경우:

    ```bash theme={null}
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
    source ~/.zshrc
    ```

    대부분의 Linux 배포판의 기본값인 Bash의 경우:

    ```bash theme={null}
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
    source ~/.bashrc
    ```

    또는 터미널을 닫았다가 다시 여세요.

    fish 또는 Nushell과 같은 다른 셸의 경우 셸의 자체 구성 구문을 사용하여 `~/.local/bin`을 PATH에 추가한 다음 터미널을 다시 시작하세요.

    수정이 작동했는지 확인하세요:

    ```bash theme={null}
    claude --version
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    $env:PATH -split ';' | Select-String '\.local\\bin'
    ```

    출력이 없으면 설치 디렉토리를 사용자 PATH에 추가하세요:

    ```powershell theme={null}
    $currentPath = [Environment]::GetEnvironmentVariable('PATH', 'User')
    [Environment]::SetEnvironmentVariable('PATH', "$currentPath;$env:USERPROFILE\.local\bin", 'User')
    ```

    변경 사항이 적용되려면 터미널을 다시 시작하세요.

    수정이 작동했는지 확인하세요:

    ```powershell theme={null}
    claude --version
    ```
  </Tab>

  <Tab title="Windows CMD">
    ```batch theme={null}
    echo %PATH% | findstr /i "local\bin"
    ```

    출력이 없으면 시스템 설정을 열고 환경 변수로 이동한 다음 `%USERPROFILE%\.local\bin`을 사용자 PATH 변수에 추가하세요. 터미널을 다시 시작하세요.

    수정이 작동했는지 확인하세요:

    ```batch theme={null}
    claude --version
    ```
  </Tab>
</Tabs>

<h3 id="check-for-conflicting-installations">
  충돌하는 설치 확인
</h3>

여러 Claude Code 설치로 인해 버전 불일치 또는 예기치 않은 동작이 발생할 수 있습니다. 설치된 항목을 확인하세요:

<Tabs>
  <Tab title="macOS/Linux">
    PATH에서 찾은 모든 `claude` 바이너리를 나열하세요:

    ```bash theme={null}
    which -a claude
    ```

    이것이 아무것도 인쇄하지 않으면 아직 PATH에 `claude`가 없습니다. [PATH 확인](#verify-your-path)으로 돌아가세요.

    `claude` 바이너리가 올 수 있는 세 위치를 확인하세요. `~/.local/bin/claude`는 네이티브 설치 프로그램이고 `~/.claude/local/`은 Claude Code의 이전 버전에서 생성한 레거시 로컬 npm 설치이며 npm 글로벌 목록은 `-g` 설치를 표시합니다:

    ```bash theme={null}
    ls -la ~/.local/bin/claude
    ```

    네이티브 설치는 `~/.local/share/claude/versions/`로의 심볼릭 링크를 표시합니다. 이 경로에서 직접 만든 스크립트 또는 심볼릭 링크는 사용자 정의 런처이며, [자동 업데이트는 제자리에 남겨둡니다](/docs/ko/setup#auto-updates).

    `ls` 명령이 `No such file or directory`를 인쇄하면 오류가 아닙니다. 이는 해당 위치에 아무것도 설치되지 않았음을 의미하므로 다음 검사로 이동하세요.

    ```bash theme={null}
    ls -la ~/.claude/local/
    ```

    ```bash theme={null}
    npm -g ls @anthropic-ai/claude-code 2>/dev/null
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    PATH에서 찾은 모든 `claude` 바이너리를 나열하세요:

    ```powershell theme={null}
    where.exe claude
    ```

    네이티브 설치 프로그램이 바이너리를 배치했는지 확인하세요:

    ```powershell theme={null}
    Test-Path "$env:USERPROFILE\.local\bin\claude.exe"
    ```
  </Tab>
</Tabs>

여러 설치를 찾으면 하나만 유지하세요. macOS/Linux의 `~/.local/bin/claude` 또는 Windows의 `%USERPROFILE%\.local\bin\claude.exe`에서의 네이티브 설치가 권장됩니다. 추가 항목을 제거하세요:

npm 글로벌 설치 제거:

```bash theme={null}
npm uninstall -g @anthropic-ai/claude-code
```

레거시 로컬 npm 설치 제거:

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    rm -rf ~/.claude/local
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    Remove-Item -Recurse -Force "$env:USERPROFILE\.claude\local"
    ```
  </Tab>
</Tabs>

macOS에서 Homebrew 설치 제거. `claude-code@latest` cask를 설치한 경우 해당 이름으로 대체하세요:

```bash theme={null}
brew uninstall --cask claude-code
```

Windows에서 WinGet 설치 제거:

```powershell theme={null}
winget uninstall Anthropic.ClaudeCode
```

<h3 id="check-directory-permissions">
  디렉토리 권한 확인
</h3>

설치 프로그램은 macOS 및 Linux의 `~/.local/bin/` 및 `~/.claude/`에 대한 쓰기 액세스가 필요합니다. Windows에서 설치 위치는 `%USERPROFILE%` 아래에 있으며 기본적으로 사용자가 쓸 수 있으므로 이 섹션은 거의 적용되지 않습니다.

디렉토리가 쓸 수 있는지 확인하세요:

```bash theme={null}
test -w ~/.local/bin && echo "writable" || echo "not writable"
test -w ~/.claude && echo "writable" || echo "not writable"
```

디렉토리를 쓸 수 없으면 설치 디렉토리를 만들고 사용자를 소유자로 설정하세요:

```bash theme={null}
sudo mkdir -p ~/.local/bin
sudo chown -R $(whoami) ~/.local
```

<h3 id="verify-the-binary-works">
  바이너리 작동 확인
</h3>

`claude --version`이 버전을 인쇄하지만 `claude`가 시작 시 충돌하거나 중단되면 이 검사를 실행하여 원인을 좁혀보세요. `claude --version`이 command not found를 표시하면 먼저 [PATH 확인](#verify-your-path)으로 이동하세요. 아래 명령은 `claude`가 PATH에 있다고 가정합니다.

바이너리가 존재하고 실행 가능한지 확인하세요:

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    ls -la "$(command -v claude)"
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    Get-Command claude | Select-Object Source
    ```
  </Tab>
</Tabs>

Linux에서 누락된 공유 라이브러리를 확인하세요. `ldd`가 누락된 라이브러리를 표시하면 시스템 패키지를 설치해야 할 수 있습니다. Alpine Linux 및 기타 musl 기반 배포판의 경우 [Alpine Linux 설정](/docs/ko/setup#alpine-linux-and-musl-based-distributions)을 참조하세요.

```bash theme={null}
ldd "$(command -v claude)" | grep "not found"
```

바이너리가 실행될 수 있는지 확인하세요:

```bash theme={null}
claude --version
```

<h2 id="common-installation-issues">
  일반적인 설치 문제
</h2>

이는 가장 자주 발생하는 설치 문제와 해결책입니다.

<h3 id="install-script-returns-html-instead-of-a-shell-script">
  설치 스크립트가 셸 스크립트 대신 HTML 반환
</h3>

설치 명령을 실행할 때 다음 오류 중 하나가 표시될 수 있습니다:

```text theme={null}
bash: line 1: syntax error near unexpected token `<'
bash: line 1: `<!DOCTYPE html>'
```

PowerShell에서 동일한 문제는 반환된 페이지로 파싱 오류로 나타나며, `iex`가 HTML과 CSS를 PowerShell로 실행하려고 시도합니다:

```text theme={null}
iex : At line:1 char:2310
+ ... igin="anonymous"/><script type="text/javascript">!function(o,c){var n ...
Missing argument in parameter list.
...
```

표현은 PowerShell 버전 및 시스템 언어에 따라 다릅니다: `Missing expression after unary operator '--'` 또는 `ParserError`와 함께 `ParseException`이 표시될 수 있습니다. 인용된 텍스트의 HTML 태그 또는 CSS는 이 실패를 식별합니다. 대신 `-OutFile install.ps1`로 다운로드하면 저장된 파일은 동일한 웹 페이지이므로 도움이 되지 않습니다.

요청이 라우팅된 방식에 따라 HTML 본문이 없는 403이 대신 표시될 수 있습니다:

```text theme={null}
curl: (22) The requested URL returned error: 403
```

이 모든 것은 설치 URL이 설치 스크립트 대신 HTML 페이지 또는 오류 상태를 반환했음을 의미합니다. HTML 페이지에 "App unavailable in region"이 표시되면 Claude Code는 귀국에서 사용할 수 없습니다. [지원되는 국가](https://www.anthropic.com/supported-countries)를 참조하세요.

본문이 없는 단순 403은 종종 동일한 원인을 가지지만 회사 프록시 또는 방화벽이 다운로드를 차단하는 것에서 비롯될 수도 있습니다. 지원되는 국가에 있는데도 여전히 403이 표시되면 아래의 대체 설치 프로그램을 시도하기 전에 [네트워크 연결 확인](#check-network-connectivity)을 진행하세요. 대체 설치 프로그램도 동일한 호스트에 도달하기 때문입니다.

그렇지 않으면 네트워크 문제, 지역 라우팅 또는 일시적인 서비스 중단으로 인해 발생할 수 있습니다.

**해결책:**

1. **대체 설치 방법 사용**:

   macOS에서 Homebrew를 통해 설치:

   ```bash theme={null}
   brew install --cask claude-code
   ```

   Windows에서 WinGet을 통해 설치:

   ```powershell theme={null}
   winget install Anthropic.ClaudeCode
   ```

   그런 다음 `claude --version`을 실행하여 확인하세요: 명령은 `2.1.211 (Claude Code)`와 같은 버전 번호를 인쇄합니다. 셸이 `claude`를 찾을 수 없다고 보고하면 새 터미널 창을 열고 다시 시도하세요: 설치한 세션은 이전 `PATH`를 유지합니다.

2. **몇 분 후 다시 시도**: 문제는 종종 일시적입니다. 기다렸다가 원래 명령을 다시 시도하세요.

<h3 id="command-not-found-claude-after-installation">
  설치 후 `command not found: claude`
</h3>

설치가 완료되었지만 `claude`가 작동하지 않습니다. 정확한 오류는 플랫폼에 따라 다릅니다:

| 플랫폼         | 오류 메시지                                                                 |
| :---------- | :--------------------------------------------------------------------- |
| macOS       | `zsh: command not found: claude`                                       |
| Linux       | `bash: claude: command not found`                                      |
| Windows CMD | `'claude' is not recognized as an internal or external command`        |
| PowerShell  | `claude : The term 'claude' is not recognized as the name of a cmdlet` |

이는 설치 디렉토리가 셸의 검색 경로에 없음을 의미합니다. 각 플랫폼의 수정 사항은 [PATH 확인](#verify-your-path)을 참조하세요.

<h3 id="curl-56-failure-writing-output-to-destination">
  `curl: (56) Failure writing output to destination`
</h3>

`curl ... | bash` 명령은 스크립트를 다운로드하고 Bash에 파이프하여 실행합니다. 이 오류와 관련된 `curl: (23) Failure writing output to destination`은 Bash가 완전한 스크립트를 받지 못했음을 의미합니다. 종료 코드 56은 다운로드 자체가 중단되었음을 나타내고 종료 코드 23은 curl이 받은 것을 파이프에 쓸 수 없었음을 나타내며, 일반적으로 Bash가 조기에 종료되었기 때문입니다.

[네트워크 연결 확인](#check-network-connectivity)의 확인으로 `downloads.claude.ai`에 도달할 수 있는지 테스트하세요. 서버에 도달했으면 원래 실패는 일시적이었을 가능성이 높습니다. 설치 명령을 다시 시도하세요. [대체 설치 방법](/docs/ko/setup#install-claude-code)을 시도할 수도 있습니다.

<h3 id="homebrew-cask-unavailable-or-outdated">
  Homebrew cask를 사용할 수 없거나 오래됨
</h3>

Homebrew가 `Error: Cask 'claude-code' is unavailable: No Cask with this name exists`를 보고하면 Homebrew cask 인덱스의 로컬 복사본이 cask의 게시 이전입니다. 인덱스를 새로 고치고 다시 시도하세요:

```bash theme={null}
brew update
brew install --cask claude-code
```

Homebrew가 예상보다 이전 Claude Code 버전을 설치하면 동일한 오래된 인덱스가 일반적으로 원인입니다. `claude-code` cask는 안정적인 채널을 추적하며 일반적으로 최신 릴리스보다 약 1주일 뒤떨어져 있습니다. 최신 버전의 경우 대신 `brew install --cask claude-code@latest`를 실행하세요. 두 cask의 차이점은 [릴리스 채널 구성](/docs/ko/setup#configure-release-channel)을 참조하세요.

<h3 id="tls-or-ssl-connection-errors">
  TLS 또는 SSL 연결 오류
</h3>

`curl: (35) TLS connect error`, `schannel: next InitializeSecurityContext failed` 또는 PowerShell의 `Could not establish trust relationship for the SSL/TLS secure channel`과 같은 오류는 TLS 핸드셰이크 실패를 나타냅니다.

**해결책:**

1. **시스템 CA 인증서 업데이트**:

   Ubuntu/Debian에서:

   ```bash theme={null}
   sudo apt-get update && sudo apt-get install ca-certificates
   ```

   macOS에서 시스템 curl은 Keychain 신뢰 저장소를 사용합니다. macOS 자체를 업데이트하면 루트 인증서가 업데이트됩니다.

2. **Windows에서 설치 프로그램을 실행하기 전에 PowerShell에서 TLS 1.2 활성화**:
   ```powershell theme={null}
   [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
   irm https://claude.ai/install.ps1 | iex
   ```

3. **프록시 또는 방화벽 간섭 확인**: TLS 검사를 수행하는 회사 프록시는 `unable to get local issuer certificate` 및 `SELF_SIGNED_CERT_IN_CHAIN`을 포함한 이러한 오류를 유발할 수 있습니다. 설치 단계의 경우 설치 다운로드가 회사 프록시의 CA를 신뢰하도록 하세요:

   <Tabs>
     <Tab title="macOS/Linux">
       ```bash theme={null}
       curl --cacert /path/to/corporate-ca.pem -fsSL https://claude.ai/install.sh | bash
       ```
     </Tab>

     <Tab title="Windows PowerShell">
       PowerShell 설치 프로그램은 .NET을 통해 다운로드하며, 이는 Windows 인증서 저장소에 대해 TLS를 검증합니다. IT 팀에 프록시의 CA 인증서를 Windows 저장소에 추가하도록 요청하세요(아직 없는 경우). 그런 다음 설치 프로그램을 실행하세요:

       ```powershell theme={null}
       irm https://claude.ai/install.ps1 | iex
       ```
     </Tab>
   </Tabs>

   설치된 Claude Code 자체의 경우 `NODE_EXTRA_CA_CERTS`를 설정하여 API 요청이 동일한 번들을 신뢰하도록 하세요:

   <Tabs>
     <Tab title="macOS/Linux">
       ```bash theme={null}
       export NODE_EXTRA_CA_CERTS=/path/to/corporate-ca.pem
       ```
     </Tab>

     <Tab title="Windows PowerShell">
       ```powershell theme={null}
       $env:NODE_EXTRA_CA_CERTS = 'C:\path\to\corporate-ca.pem'
       ```
     </Tab>
   </Tabs>

   인증서 파일이 없으면 IT 팀에 문의하세요. 프록시가 원인인지 확인하기 위해 직접 연결에서 시도할 수도 있습니다.

4. **Windows에서 차단된 해지 확인 해결**. `CRYPT_E_NO_REVOCATION_CHECK (0x80092012)` 및 `CRYPT_E_REVOCATION_OFFLINE (0x80092013)` 오류는 curl이 서버에 도달했지만 네트워크가 인증서 해지 조회를 차단함을 의미하며, 이는 회사 방화벽 뒤에서 일반적입니다. 실패한 명령이 `install.cmd`를 다운로드하는 `curl`인 경우 `--ssl-revoke-best-effort`를 추가하여 명령 프롬프트에서 다시 실행하세요:
   ```batch theme={null}
   curl --ssl-revoke-best-effort -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
   ```
   스크립트 자체의 다운로드가 동일한 오류를 겪으면 자동으로 최선의 노력 해지 확인으로 다시 시도하므로 플래그는 직접 실행하는 명령에만 필요합니다. 최선의 노력 확인은 도달할 수 없는 해지 서버를 허용하지만 알려진 해지된 인증서는 거부하며, 브라우저가 해지를 처리하는 방식과 일치합니다. PowerShell에서 PowerShell 설치 프로그램을 실행하여 curl의 해지 확인을 완전히 피할 수도 있습니다. 이는 .NET을 통해 다운로드하며 해지 서버에 도달할 수 없을 때 실패하지 않습니다:
   ```powershell theme={null}
   irm https://claude.ai/install.ps1 | iex
   ```
   `winget install Anthropic.ClaudeCode`로 설치할 수도 있으며, 이는 curl을 완전히 피합니다.

<h3 id="failed-to-fetch-version-from-downloads-claude-ai">
  `Failed to fetch version from downloads.claude.ai`
</h3>

설치 프로그램이 다운로드 서버에 도달할 수 없습니다. 이는 일반적으로 `downloads.claude.ai`가 네트워크에서 차단됨을 의미합니다. [네트워크 연결 확인](#check-network-connectivity)을 참조하세요.

<h3 id="wrong-install-command-on-windows">
  Windows에서 잘못된 설치 명령
</h3>

`'irm' is not recognized`, `The token '&&' is not valid`, `A parameter cannot be found that matches parameter name 'fsSL'` 또는 `'bash' is not recognized as the name of a cmdlet`이 표시되면 다른 셸 또는 운영 체제의 설치 명령을 복사했습니다. 명령이 스크립트의 텍스트를 인쇄하면 부분만 실행했습니다.

* **`irm` 인식 안 됨**: CMD에 있고 PowerShell이 아닙니다. 두 가지 옵션이 있습니다:

  시작 메뉴에서 "PowerShell"을 검색하여 PowerShell을 열고 원래 설치 명령을 실행하세요:

  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

  또는 CMD에 머물러 있고 CMD 설치 프로그램을 대신 사용하세요:

  ```batch theme={null}
  curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
  ```

* **`&&` 유효하지 않음**: PowerShell에 있지만 CMD 설치 프로그램 명령을 실행했습니다. PowerShell 설치 프로그램을 사용하십시오:
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

* **`A parameter cannot be found that matches parameter name 'fsSL'`**: Windows PowerShell에서 macOS/Linux `curl -fsSL ... | bash` 설치 프로그램을 실행했습니다. 여기서 `curl`은 `Invoke-WebRequest`의 별칭이며 `-fsSL` 플래그를 거부합니다. 대신 PowerShell 설치 프로그램을 사용하세요:
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

* **`bash` 인식 안 됨**: Windows에서 macOS/Linux 설치 프로그램을 실행했습니다. 대신 PowerShell 설치 프로그램을 사용하세요:
  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

* **명령이 스크립트 텍스트를 인쇄함**: 다운로드 절반을 실행하는 부분 없이 실행했습니다. `irm https://claude.ai/install.ps1`만으로는 다운로드된 스크립트를 터미널에 인쇄합니다. 이를 실행하려면 `iex`에 파이프하세요:

  ```powershell theme={null}
  irm https://claude.ai/install.ps1 | iex
  ```

  CMD에서 `-o` 없이 `curl -fsSL https://claude.ai/install.cmd`는 배치 스크립트를 저장하는 대신 인쇄합니다. 완전한 명령을 실행하세요:

  ```batch theme={null}
  curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
  ```

어느 설치 프로그램을 사용하든 작동했는지 확인하세요: 새 터미널을 열고 `claude --version`을 실행하세요. 이는 `2.1.211 (Claude Code)`와 같은 버전 번호를 인쇄합니다.

<h3 id="running-scripts-is-disabled-on-this-system">
  `running scripts is disabled on this system`
</h3>

Windows에서 npm을 통해 Claude Code를 설치하거나 실행하면 `SecurityError`로 실패할 수 있습니다:

```text theme={null}
npm : File C:\Program Files\nodejs\npm.ps1 cannot be loaded because running scripts is disabled on this system. For more information, see about_Execution_Policies at https:/go.microsoft.com/fwlink/?LinkID=135170.
...
    + CategoryInfo          : SecurityError: (:) [], PSSecurityException
```

npm 설치 후 `claude`를 실행할 때 동일한 오류가 `claude.ps1`의 이름을 지정합니다. PowerShell의 실행 정책은 npm이 명령에 대해 생성하는 `.ps1` 런처 스크립트를 차단하고 있습니다. 정책은 스크립트 파일에 적용되므로 다운로드된 텍스트를 직접 실행하는 PowerShell 설치 프로그램 `irm https://claude.ai/install.ps1 | iex`에는 영향을 주지 않습니다.

**해결책:**

1. **사용자에 대해 로컬로 생성된 스크립트 허용**한 다음 다시 시도하세요:
   ```powershell theme={null}
   Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
   ```
2. **`.cmd` 런처 대신 호출**: `npm.cmd` 및 `claude.cmd`는 동일한 작업을 수행하며 정책은 이들을 다루지 않습니다.
3. **npm 대신 [PowerShell 설치 프로그램](/docs/ko/setup#install-claude-code) 사용**. 이는 `.ps1` 스크립트가 아닌 바이너리를 설치합니다.

<h3 id="the-process-cannot-access-the-file-during-windows-install">
  Windows 설치 중 `The process cannot access the file`
</h3>

PowerShell 설치 프로그램이 `Failed to download binary: The process cannot access the file ... because it is being used by another process`로 실패하면 설치 프로그램이 `%USERPROFILE%\.claude\downloads`에 쓸 수 없습니다. 이는 일반적으로 이전 설치 시도가 여전히 실행 중이거나 바이러스 백신 소프트웨어가 해당 폴더의 부분적으로 다운로드된 바이너리를 스캔하고 있음을 의미합니다.

설치 프로그램을 실행하는 다른 PowerShell 창을 닫고 바이러스 백신 스캔이 파일을 해제할 때까지 기다리세요. 그런 다음 다운로드 폴더를 삭제하고 설치 프로그램을 다시 실행하세요:

```powershell theme={null}
Remove-Item -Recurse -Force "$env:USERPROFILE\.claude\downloads"
irm https://claude.ai/install.ps1 | iex
```

<h3 id="install-killed-on-low-memory-linux-servers">
  저메모리 Linux 서버에서 설치 중단
</h3>

설치 중에 `Killed` 메시지가 표시되면 일반적으로 Linux OOM(메모리 부족) killer가 시스템이 메모리 부족으로 인해 `claude install` 단계를 종료했음을 의미합니다. 이는 작은 VPS 및 클라우드 인스턴스에서 일반적입니다. 설치 스크립트는 원인을 보고하고 종료 코드 137로 종료됩니다. 이 예에서 줄 번호와 프로세스 ID는 릴리스 및 실행에 따라 다릅니다:

```text theme={null}
Setting up Claude Code...
bash: line 183: 34803 Killed    "$binary_path" install ${TARGET:+"$TARGET"}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

설치에는 대략 512MB의 여유 메모리가 필요하며 Claude Code를 실행하려면 더 많은 메모리가 필요합니다. [시스템 요구사항](/docs/ko/setup#system-requirements)을 참조하세요.

**해결책:**

1. **서버의 RAM이 제한된 경우 스왑 공간 추가**. 스왑은 디스크 공간을 오버플로우 메모리로 사용하여 낮은 물리적 RAM으로도 설치를 완료할 수 있게 합니다.

   2GB 스왑 파일을 만들고 활성화하세요:

   ```bash theme={null}
   sudo fallocate -l 2G /swapfile
   sudo chmod 600 /swapfile
   sudo mkswap /swapfile
   sudo swapon /swapfile
   ```

   그런 다음 설치를 다시 시도하세요:

   ```bash theme={null}
   curl -fsSL https://claude.ai/install.sh | bash
   ```

2. **설치하기 전에 다른 프로세스를 닫아** 메모리를 확보하세요.

3. **가능하면 더 큰 인스턴스 사용**. Claude Code는 최소 4GB의 RAM이 필요합니다.

<h3 id="install-hangs-in-docker">
  Docker에서 설치 중단
</h3>

Docker 컨테이너에서 Claude Code를 설치할 때 root로 `/`에 설치하면 중단될 수 있습니다.

**해결책:**

1. **설치 프로그램을 실행하기 전에 작업 디렉토리 설정**. `/`에서 실행하면 설치 프로그램이 전체 파일 시스템을 스캔하여 과도한 메모리 사용을 유발합니다. `WORKDIR`을 설정하면 스캔이 작은 디렉토리로 제한됩니다:
   ```dockerfile theme={null}
   WORKDIR /tmp
   RUN curl -fsSL https://claude.ai/install.sh | bash
   ```

2. **Docker에 더 많은 메모리 제공** Docker Desktop을 사용하는 경우. **Settings > Resources**를 열고 메모리 제한을 높이고 빌드를 다시 실행하세요.

<h3 id="raw-mode-is-not-supported-during-install">
  설치 중 `Raw mode is not supported`
</h3>

조직의 [서버 관리 설정](/docs/ko/server-managed-settings)에 [보안 승인](/docs/ko/server-managed-settings#security-approval-dialogs)이 필요한 변경 사항이 포함되어 있으면 Claude Code 버전 2.1.246 이전에는 `claude install` 중에 승인 대화 상자를 표시하려고 시도합니다. 대화 상자는 stdin의 터미널이 필요합니다. 설치 프로그램이 `curl -fsSL https://claude.ai/install.sh | bash`처럼 파이프에서 `claude install`을 실행하면 stdin은 터미널이 아닌 파이프이므로 설치가 `Raw mode is not supported`를 포함하는 오류로 실패합니다.

Claude Code v2.1.246 이상은 `claude install` 또는 `claude update` 중에 대화 상자를 표시하지 않습니다. 명령은 마지막으로 승인한 설정으로 실행되며 Claude Code는 다음 대화형 세션에서 대화 상자를 표시합니다. 조직의 시작 구성이 [설정 가져오기를 기다리는 경우](/docs/ko/server-managed-settings#enforce-fail-closed-startup)(예: `forceRemoteSettingsRefresh`를 설정할 때) 대화 상자는 여전히 이러한 명령 중에 나타나며 파이프에서 실행되는 설치는 여전히 실패합니다.

다른 모든 구성에서 설치 프로그램을 다시 실행하면 이 오류를 지나갑니다. 스크립트는 이전 버전을 설치하도록 요청할 때도 최신 릴리스의 `install` 명령을 실행하기 때문입니다. 플랫폼에 대한 명령을 다시 실행하세요:

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    irm https://claude.ai/install.ps1 | iex
    ```
  </Tab>
</Tabs>

`claude --version`은 다시 실행이 설치한 버전을 인쇄합니다.

<h3 id="claude-update-or-claude-doctor-hangs">
  `claude update` 또는 `claude doctor` 중단
</h3>

`claude update` 및 `claude doctor`는 셸 구성 파일에서 오래된 `claude` 별칭을 스캔합니다: `~/.zshrc`, `~/.bashrc` 및 `~/.config/fish/config.fish`. macOS에서는 존재하는 `~/.bash_profile`, `~/.bash_login` 또는 `~/.profile` 중 첫 번째입니다. `ZDOTDIR`을 설정하면 Zsh 파일은 `$ZDOTDIR/.zshrc`입니다. 이러한 경로 중 하나가 디렉토리인 경우 Claude Code는 이를 건너뛰고 두 명령 모두 정상적으로 완료됩니다. v2.1.214 이전에는 이러한 경로의 디렉토리로 인해 두 명령 모두 중단되었으며 `/status`의 System diagnostics 섹션이 비어 있었습니다. `claude doctor`는 출력 없이 중단되었습니다. `claude update`는 `Checking for updates`를 인쇄한 직후 중단되었습니다.

이전 버전에서 중단을 겪으면 디렉토리를 찾으세요. 이 명령의 출력에서 `d`로 시작하는 줄은 해당 경로를 디렉토리로 표시합니다. `No such file or directory` 줄은 해당 경로에 아무것도 없으며 원인이 아님을 의미합니다:

```bash theme={null}
ls -ld ~/.zshrc ~/.bashrc ~/.bash_profile ~/.bash_login ~/.profile ~/.config/fish/config.fish
```

디렉토리를 옆으로 이동하거나 v2.1.214 이상으로 업데이트하세요. `claude update`는 영향을 받는 버전에서 중단되므로 [설치 스크립트](/docs/ko/setup#install-claude-code)를 다시 실행하여 업데이트하세요.

<h3 id="claude-desktop-overrides-the-claude-command-on-windows">
  Claude Desktop이 Windows에서 `claude` 명령 무시
</h3>

Claude Desktop의 이전 버전을 설치한 경우 `WindowsApps` 디렉토리에 `Claude.exe`를 등록할 수 있으며, 이는 Claude Code CLI보다 PATH 우선순위를 가집니다. `claude`를 실행하면 CLI 대신 Desktop 앱이 열립니다.

Claude Desktop을 최신 버전으로 업데이트하여 이 문제를 해결하세요.

<h3 id="claude-code-on-windows-requires-either-git-for-windows-for-bash-or-powershell">
  Windows에서 Claude Code는 Git for Windows(Bash용) 또는 PowerShell 필요
</h3>

Git for Windows는 선택 사항입니다. Claude Code는 Git Bash가 없을 때 [PowerShell 도구](/docs/ko/tools-reference#powershell-tool)를 사용하므로 이 오류는 어느 셸도 찾을 수 없음을 의미합니다.

**PowerShell이 PATH에서 누락된 경우** 기본 위치는 `C:\Windows\System32\WindowsPowerShell\v1.0\`입니다. 해당 디렉토리를 `PATH`에 추가하거나 `pwsh`를 제공하는 [PowerShell 7](https://aka.ms/powershell)을 설치하세요.

**Git for Windows를 설치하려면** [git-scm.com/downloads/win](https://git-scm.com/downloads/win)에서 다운로드하세요. 설정 중에 "Add to PATH"를 선택하세요. 설치 후 터미널을 다시 시작하세요. 설치하면 Bash 도구가 활성화되어 Bash 기반 스크립트 및 도구로 작업할 때 유용합니다.

**Git이 이미 설치되어 있지만** Claude Code가 찾을 수 없으면 위치를 Claude Code가 확인하는 위치와 비교하세요. `CLAUDE_CODE_GIT_BASH_PATH`가 설정되지 않으면 Claude Code는 다음 순서로 `bash.exe`를 찾습니다:

1. 기본 설치 위치 `C:\Program Files\Git` 및 `C:\Program Files (x86)\Git`.
2. `PATH`의 `git`. 해당 Git 설치에서 `bin\bash.exe`를 사용합니다.

2단계에서 Claude Code는 Claude Code를 시작한 폴더에 있거나 `node_modules` 또는 `.venv` 또는 `env`와 같은 가상 환경 폴더를 포함하는 경로 아래에 있는 `git`을 건너뜁니다. 예를 들어 `C:\dev\env\myproject`에서 시작했을 때 `C:\dev\env\myproject\Git`. 이는 Claude Code가 프로젝트가 거기에 배치한 실행 파일을 실행하지 않도록 합니다. Git이 그런 위치에 있으면 `CLAUDE_CODE_GIT_BASH_PATH`를 가리키세요.

**Claude Code를 특정 Git 설치로 가리키려면** PowerShell에서 `where.exe git`을 실행하여 찾고 해당 설치에서 `bin\bash.exe` 경로를 [settings.json 파일](/docs/ko/settings)에서 `CLAUDE_CODE_GIT_BASH_PATH`로 설정하세요:

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_GIT_BASH_PATH": "C:\\Program Files\\Git\\bin\\bash.exe"
  }
}
```

**`CLAUDE_CODE_GIT_BASH_PATH`가 올바른 경로로 설정되고 파일이 존재하지만** Claude Code가 여전히 사용하지 않으면 파일의 이름을 먼저 확인하세요. Claude Code는 `bash.exe`, `sh.exe`, `bash` 또는 `sh`라는 파일만 허용합니다. Git for Windows의 `git-bash.exe` 런처와 같은 다른 이름이면 변수를 무시하고 설정되지 않은 것처럼 자동 감지하며 `--debug`로 볼 수 있는 경고를 기록합니다. 존재하지 않는 경로는 동일한 폴백과 경고를 받습니다. v2.1.219 이전에는 Claude Code가 이름을 확인하지 않고 모든 기존 파일을 셸로 사용했으며 경로가 존재하지 않으면 `Claude Code was unable to find CLAUDE_CODE_GIT_BASH_PATH path`로 시작 시 종료되었습니다.

파일의 이름이 맞으면 AppLocker, 그룹 정책 소프트웨어 제한 정책 또는 EDR 에이전트와 같은 엔드포인트 보안 소프트웨어가 간섭할 수 있습니다. IT 팀에 `claude.exe` 및 `cmd.exe` 및 `bash.exe`를 포함한 생성하는 프로세스를 엔드포인트 보호 정책에서 허용 목록에 추가하도록 요청하세요.

<h3 id="claude-code-does-not-support-32-bit-windows">
  Claude Code는 32비트 Windows를 지원하지 않음
</h3>

Windows는 시작 메뉴에 두 개의 PowerShell 항목을 포함합니다: `Windows PowerShell` 및 `Windows PowerShell (x86)`. x86 항목은 32비트 프로세스로 실행되며 64비트 머신에서도 이 오류를 트리거합니다. 어느 경우인지 확인하려면 오류를 생성한 동일한 창에서 이를 실행하세요:

```powershell theme={null}
[Environment]::Is64BitOperatingSystem
```

이것이 `True`를 인쇄하면 운영 체제는 정상입니다. 창을 닫고 x86 접미사 없이 `Windows PowerShell`을 열고 설치 명령을 다시 실행하세요.

이것이 `False`를 인쇄하면 32비트 Windows 버전을 사용 중입니다. Claude Code는 64비트 운영 체제가 필요합니다. [시스템 요구사항](/docs/ko/setup#system-requirements)을 참조하세요.

<h3 id="linux-musl-or-glibc-binary-mismatch">
  Linux musl 또는 glibc 바이너리 불일치
</h3>

설치 후 `libstdc++.so.6` 또는 `libgcc_s.so.1`과 같은 누락된 공유 라이브러리에 대한 오류가 표시되면 설치 프로그램이 시스템에 맞는 잘못된 바이너리 변형을 다운로드했을 수 있습니다.

```text theme={null}
Error loading shared library libstdc++.so.6: No such file or directory
```

이는 musl 크로스 컴파일 패키지가 설치된 glibc 기반 시스템에서 발생할 수 있으며, 설치 프로그램이 시스템을 musl로 잘못 감지하게 합니다.

**해결책:**

1. **시스템이 어느 libc를 사용하는지 확인**:
   ```bash theme={null}
   ldd --version 2>&1 | head -1
   ```
   `GNU libc` 또는 `GLIBC`를 언급하는 출력은 glibc를 의미합니다. `musl`을 언급하는 출력은 musl을 의미합니다.

2. **glibc에 있지만 musl 바이너리를 받은 경우** 설치를 제거하고 다시 설치하세요. `https://downloads.claude.ai/claude-code-releases/{VERSION}/manifest.json`의 매니페스트를 사용하여 올바른 바이너리를 수동으로 다운로드할 수도 있습니다. `ldd --version` 및 `ls /lib/libc.musl*`의 출력과 함께 [GitHub 이슈](https://github.com/anthropics/claude-code/issues)를 제출하세요.

3. **실제로 musl에 있는 경우** Alpine Linux와 같이 필요한 패키지를 설치하세요:
   ```bash theme={null}
   apk add libgcc libstdc++ ripgrep
   ```
   Alpine에서 `ripgrep`은 커뮤니티 저장소에 있습니다. `apk`가 패키지가 누락되었다고 보고하면 [Alpine Linux 설정](/docs/ko/setup#alpine-linux-and-musl-based-distributions)을 참조하세요.

<h3 id="illegal-instruction">
  `Illegal instruction`
</h3>

`claude`를 실행하거나 설치 프로그램이 `Illegal instruction`을 인쇄하면 네이티브 바이너리는 프로세서가 지원하지 않는 CPU 명령어를 사용합니다. 두 가지 서로 다른 원인이 있습니다.

**아키텍처 불일치.** 설치 프로그램이 잘못된 바이너리를 다운로드했습니다. 예를 들어 ARM 서버의 x86. macOS 또는 Linux에서 `uname -m`으로 확인하거나 PowerShell에서 `$env:PROCESSOR_ARCHITECTURE`로 확인하세요. 결과가 받은 바이너리와 일치하지 않으면 출력과 함께 [GitHub 이슈](https://github.com/anthropics/claude-code/issues)를 제출하세요.

**누락된 AVX 명령어 세트.** 아키텍처는 올바르지만 여전히 `Illegal instruction`이 표시되면 CPU에 바이너리가 필요로 하는 AVX 또는 다른 명령어가 없을 가능성이 높습니다. 이는 대략 2013년 이전의 Intel 및 AMD 프로세서에 영향을 미치며, 하이퍼바이저가 게스트에게 AVX를 전달하지 않는 가상 머신에도 영향을 미칩니다.

VPS 또는 VM에서 `grep -m1 -ow avx /proc/cpuinfo`를 실행하세요. 빈 결과는 AVX를 게스트에서 사용할 수 없음을 의미합니다.

네이티브 바이너리 해결 방법이 없습니다. [이슈 #50384](https://github.com/anthropics/claude-code/issues/50384)를 추적하고 Linux에서 `grep -m1 "model name" /proc/cpuinfo`의 CPU 모델 또는 macOS에서 `sysctl -n machdep.cpu.brand_string`을 보고할 때 포함하세요.

대체 설치 방법은 동일한 네이티브 바이너리를 다운로드하며 어느 원인도 해결하지 않습니다.

<h3 id="dyld-cannot-load-on-macos">
  macOS에서 `dyld: cannot load`
</h3>

설치 중에 `dyld: Symbol not found`, `dyld: cannot load` 또는 `Abort trap: 6`이 표시되면 바이너리는 macOS 버전 또는 하드웨어와 호환되지 않습니다.

`libicucore`를 참조하는 `Symbol not found` 오류는 macOS 버전이 바이너리가 지원하는 것보다 오래되었음을 의미합니다:

```text theme={null}
dyld: Symbol not found: _ubrk_clone
  Referenced from: claude-darwin-x64 (which was built for Mac OS X 13.0)
  Expected in: /usr/lib/libicucore.A.dylib
```

로더는 대신 바이너리의 로드 명령을 거부할 수 있으며, 이는 macOS 버전이 너무 오래되었음을 의미합니다:

```text theme={null}
dyld: cannot load 'claude-2.1.42-darwin-x64' (load command 0x80000034 is unknown)
Abort trap: 6
```

**해결책:**

1. **macOS 버전 확인**: Claude Code는 macOS 13.0 이상이 필요합니다. Apple 메뉴를 열고 이 Mac에 관하여를 선택하여 버전을 확인하세요.

2. **이전 버전을 사용 중인 경우 macOS 업데이트**. 바이너리는 이전 macOS 버전이 지원하지 않는 로드 명령 및 시스템 라이브러리를 사용합니다. Homebrew와 같은 대체 설치 방법은 동일한 바이너리를 다운로드하며 이 오류를 해결하지 않습니다.

<h3 id="exec-format-error-on-wsl1">
  WSL1에서 `Exec format error`
</h3>

WSL에서 `claude`를 실행하면 `cannot execute binary file: Exec format error`가 인쇄되면 WSL1에 있으며 [이슈 #38788](https://github.com/anthropics/claude-code/issues/38788)에서 추적되는 알려진 네이티브 바이너리 회귀를 겪고 있습니다. 바이너리의 프로그램 헤더가 WSL1의 로더가 처리할 수 없는 방식으로 변경되었습니다.

가장 깔끔한 수정은 PowerShell에서 배포판을 WSL2로 변환하는 것입니다:

```powershell theme={null}
wsl --set-version <DistroName> 2
```

WSL1에 머물러야 하는 경우 동적 링커를 통해 바이너리를 호출하세요. WSL 내 `~/.bashrc`에 이 함수를 추가하고 홈 디렉토리가 다르면 경로를 바꾸세요:

```bash theme={null}
claude() {
  /lib64/ld-linux-x86-64.so.2 "$(readlink -f "$HOME/.local/bin/claude")" "$@"
}
```

그런 다음 `source ~/.bashrc`를 실행하고 `claude`를 다시 시도하세요.

<h3 id="npm-install-errors-in-wsl">
  WSL에서 npm 설치 오류
</h3>

이 문제는 WSL 내에서 `npm install -g`로 Claude Code를 설치한 경우 적용됩니다. [네이티브 설치 프로그램](/docs/ko/setup)을 사용한 경우 이 섹션을 건너뛰세요.

**OS 또는 플랫폼 감지 문제.** npm이 설치 중에 플랫폼 불일치를 보고하면 WSL이 Windows `npm`을 선택하고 있을 가능성이 높습니다. 먼저 `npm config set os linux`를 실행한 다음 `npm install -g @anthropic-ai/claude-code --force`로 설치하세요. `sudo`를 사용하지 마세요.

**`claude` 실행 시 `exec: node: not found`.** WSL 환경이 Node.js의 Windows 설치를 사용하고 있을 가능성이 높습니다. `which npm` 및 `which node`로 확인하세요: `/mnt/c/`로 시작하는 경로는 Windows 바이너리이고 Linux 경로는 `/usr/`로 시작합니다. 이를 수정하려면 Linux 배포판의 패키지 관리자 또는 [`nvm`](https://github.com/nvm-sh/nvm)을 통해 Node를 설치하세요.

**nvm 버전 충돌.** WSL과 Windows 모두에 nvm이 설치되어 있으면 WSL에서 Node 버전을 전환하면 WSL이 기본적으로 Windows PATH를 가져오고 Windows nvm이 우선순위를 가지기 때문에 중단될 수 있습니다. 가장 일반적인 원인은 nvm이 셸에 로드되지 않는 것입니다. nvm 로더를 `~/.bashrc` 또는 `~/.zshrc`에 추가하세요:

```bash theme={null}
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"
```

또는 현재 세션에서 로드하세요:

```bash theme={null}
source ~/.nvm/nvm.sh
```

nvm이 로드되었지만 Windows 경로가 여전히 우선순위를 가지면 Linux Node 경로를 명시적으로 앞에 추가하세요:

```bash theme={null}
export PATH="$HOME/.nvm/versions/node/$(node -v)/bin:$PATH"
```

<Warning>
  `appendWindowsPath = false`를 통해 Windows PATH 가져오기를 비활성화하지 마세요. WSL에서 Windows 실행 파일을 호출하는 기능이 중단됩니다. 마찬가지로 Windows 개발에 사용하는 경우 Windows에서 Node.js를 제거하지 마세요.
</Warning>

<h3 id="permission-errors-during-installation">
  설치 중 권한 오류
</h3>

네이티브 설치 프로그램이 권한 오류로 실패하면 대상 디렉토리를 쓸 수 없을 수 있습니다. [디렉토리 권한 확인](#check-directory-permissions)을 참조하세요.

이전에 npm으로 설치했고 npm 특정 권한 오류를 겪고 있으면 네이티브 설치 프로그램으로 전환하세요:

```bash theme={null}
curl -fsSL https://claude.ai/install.sh | bash
```

<h3 id="native-binary-not-found-after-npm-install">
  npm 설치 후 네이티브 바이너리를 찾을 수 없음
</h3>

`@anthropic-ai/claude-code` npm 패키지는 `@anthropic-ai/claude-code-darwin-arm64`와 같은 플랫폼별 선택적 종속성을 통해 네이티브 바이너리를 다운로드합니다. npm은 패키지의 postinstall 스크립트를 실행하여 해당 바이너리를 `claude` 명령으로 제자리에 복사합니다. 실행될 때까지 `claude`는 자리 표시자 스크립트입니다. 다운로드 또는 postinstall 단계가 건너뛰어지면 자리 표시자가 제자리에 남아 있으며 macOS 및 Linux에서 `claude`를 실행하면 다음이 인쇄됩니다:

```text theme={null}
Error: claude native binary not installed.

Either postinstall did not run (--ignore-scripts, some pnpm configs)
or the platform-native optional dependency was not downloaded
(--omit=optional).

Run the postinstall manually (adjust path for local vs global install):
  node node_modules/@anthropic-ai/claude-code/install.cjs

Or reinstall without --ignore-scripts / --omit=optional.
```

Windows에서 `bin/claude.exe`는 동일한 셸 스크립트 자리 표시자이므로 PowerShell 및 CMD는 이 메시지를 인쇄하는 대신 파일을 실행할 수 없다고 보고합니다.

다음 원인을 확인하세요:

* **선택적 종속성이 비활성화됨.** npm 설치 명령에서 `--omit=optional`을 제거하고 pnpm에서 `--no-optional`을 제거하고 yarn에서 `--ignore-optional`을 제거하고 `.npmrc`가 `optional=false`를 설정하지 않는지 확인하세요. 그런 다음 다시 설치하세요. 네이티브 바이너리는 선택적 종속성으로만 제공되므로 건너뛰면 JavaScript 폴백이 없으며 `install.cjs`를 다시 실행해도 다운로드되지 않은 바이너리를 배치할 수 없습니다.
* **설치 스크립트가 비활성화됨.** `--ignore-scripts` 및 일부 pnpm 구성은 postinstall 단계를 건너뛰지만 여전히 플랫폼 패키지를 다운로드합니다. 메시지가 제안하는 대로 `node node_modules/@anthropic-ai/claude-code/install.cjs`를 실행하거나 플래그 없이 다시 설치하세요. postinstall이 환경에서 실행될 수 없으면 `node node_modules/@anthropic-ai/claude-code/cli-wrapper.cjs`가 다운로드된 패키지를 찾아 시작하며, 각 시작 시 추가 Node 프로세스의 비용이 발생합니다. 래퍼가 `Could not find native binary package` 대신 인쇄하면 플랫폼 패키지가 다운로드되지 않았으므로 먼저 위의 선택적 종속성 원인을 수정하세요.
* **지원되지 않는 플랫폼.** 미리 빌드된 바이너리는 `darwin-arm64`, `darwin-x64`, `linux-x64`, `linux-arm64`, `linux-x64-musl`, `linux-arm64-musl`, `win32-x64` 및 `win32-arm64`에 대해 게시됩니다. Claude Code는 다른 플랫폼에 대한 바이너리를 제공하지 않습니다. [시스템 요구사항](/docs/ko/setup#system-requirements)을 참조하세요. FreeBSD에서 설치 프로그램은 플랫폼을 지원되지 않음으로 보고합니다. v2.1.205 이전에는 FreeBSD를 Linux로 취급하고 실행할 수 없는 바이너리를 다운로드했습니다.
* **회사 npm 미러가 플랫폼 패키지를 누락함.** 레지스트리가 메타 패키지 외에도 8개의 `@anthropic-ai/claude-code-*` 플랫폼 패키지를 모두 미러링하는지 확인하세요.

<h3 id="npm-enotempty-during-update-or-reinstall">
  npm `ENOTEMPTY` 오류 업데이트 또는 재설치 중
</h3>

기존 설치에 대해 `npm install -g @anthropic-ai/claude-code`를 실행하면 npm이 이전 패키지 디렉토리를 옆으로 이동하는 동안 실패할 수 있습니다:

```text theme={null}
npm error code ENOTEMPTY
npm error syscall rename
npm error path /home/you/.nvm/versions/node/v22.13.1/lib/node_modules/@anthropic-ai/claude-code
npm error dest /home/you/.nvm/versions/node/v22.13.1/lib/node_modules/@anthropic-ai/.claude-code-tVWAnUUt
npm error errno -39
npm error ENOTEMPTY: directory not empty, rename '...'
```

`npm error path` 줄은 npm이 이동할 수 없는 디렉토리의 이름을 지정합니다. 해당 디렉토리와 옆에 있는 모든 남은 `.claude-code-*` 디렉토리를 삭제하세요. 이전 중단된 실행은 뒤에 남길 수 있습니다. 아래 명령은 `npm root -g`로 전역 패키지 디렉토리를 찾습니다. `npm error path` 줄이 이름을 지정하는 디렉토리가 `npm root -g`가 인쇄하는 디렉토리 아래에 없으면(예: nvm으로 Node 버전을 전환했기 때문에) 오류가 이름을 지정하는 디렉토리를 대신 삭제하세요:

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    rm -rf "$(npm root -g)/@anthropic-ai/claude-code"
    ```

    그런 다음 남은 임시 디렉토리를 제거하세요. zsh가 `no matches found`를 인쇄하면 제거할 것이 없었습니다:

    ```bash theme={null}
    rm -rf "$(npm root -g)/@anthropic-ai/.claude-code-"*
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    Remove-Item -Recurse -Force "$(npm root -g)/@anthropic-ai/claude-code", "$(npm root -g)/@anthropic-ai/.claude-code-*"
    ```
  </Tab>
</Tabs>

그런 다음 다시 설치하세요:

```bash theme={null}
npm install -g @anthropic-ai/claude-code
```

`claude --version`으로 확인하세요. 이는 `2.1.211 (Claude Code)`와 같은 버전 번호를 인쇄합니다.

<h2 id="login-and-authentication">
  로그인 및 인증
</h2>

이 섹션은 로그인 실패, OAuth 오류 및 토큰 문제를 다룹니다.

<h3 id="reset-your-login">
  로그인 재설정
</h3>

로그인이 실패하고 원인이 명확하지 않으면 깨끗한 재인증이 대부분의 경우를 해결합니다:

1. `/logout`을 실행하여 완전히 로그아웃
2. Claude Code 닫기
3. `claude`로 다시 시작하고 인증 프로세스 완료

브라우저가 로그인 중에 자동으로 열리지 않으면 `c`를 눌러 OAuth URL을 클립보드에 복사한 다음 수동으로 브라우저에 붙여넣으세요. 이는 URL이 좁은 또는 SSH 터미널에서 줄을 넘어 래핑되고 직접 클릭할 수 없을 때도 작동합니다.

<h3 id="oauth-error-invalid-code">
  OAuth 오류: 유효하지 않은 코드
</h3>

`OAuth error: Invalid code. Please make sure the full code was copied`가 표시되면 로그인 코드가 만료되었거나 복사-붙여넣기 중에 잘렸습니다.

**해결책:**

* Enter를 눌러 다시 시도하고 브라우저가 열린 후 빠르게 로그인 완료
* 브라우저가 자동으로 열리지 않으면 `c`를 입력하여 전체 URL 복사
* 원격/SSH 세션을 사용하는 경우 브라우저가 잘못된 머신에서 열릴 수 있습니다. 터미널에 표시된 URL을 복사하고 로컬 브라우저에서 대신 열어보세요.

<h3 id="403-forbidden-after-login">
  로그인 후 403 Forbidden
</h3>

로그인 후 `API Error: 403 {"error":{"type":"forbidden","message":"Request not allowed"}}`가 표시되면:

* **Claude Pro/Max 사용자**: [claude.ai/settings](https://claude.ai/settings)에서 구독이 활성화되어 있는지 확인
* **Anthropic Console 사용자**: 계정에 "Claude Code" 또는 "Developer" 역할이 있는지 확인. 관리자는 Anthropic Console의 설정 → 멤버에서 이를 할당합니다.
* **프록시 뒤에 있음**: 회사 프록시가 API 요청을 방해할 수 있습니다. 프록시 설정은 [네트워크 구성](/docs/ko/network-config)을 참조하세요.

<h3 id="this-organization-has-been-disabled-with-an-active-subscription">
  이 조직은 활성 구독으로 비활성화되었습니다
</h3>

활성 Claude 구독이 있음에도 불구하고 `API Error: 400 ... "This organization has been disabled"`가 표시되면 `ANTHROPIC_API_KEY` 환경 변수가 구독을 무시하고 있습니다. 이는 이전 고용주 또는 프로젝트의 이전 API 키가 여전히 셸 프로필에 설정되어 있을 때 일반적으로 발생합니다.

`ANTHROPIC_API_KEY`가 있고 승인한 경우 Claude Code는 구독의 OAuth 자격증명 대신 해당 키를 사용합니다. `-p` 플래그를 사용한 비대화형 모드에서는 키가 있을 때 항상 사용됩니다. 전체 해결 순서는 [인증 우선순위](/docs/ko/authentication#authentication-precedence)를 참조하세요.

대신 구독을 사용하려면 환경 변수를 설정 해제하고 셸 프로필에서 제거하세요:

<Tabs>
  <Tab title="macOS/Linux">
    ```bash theme={null}
    unset ANTHROPIC_API_KEY
    claude
    ```
  </Tab>

  <Tab title="Windows PowerShell">
    ```powershell theme={null}
    Remove-Item Env:ANTHROPIC_API_KEY
    claude
    ```
  </Tab>
</Tabs>

`~/.zshrc`, `~/.bashrc` 또는 `~/.profile`에서 `export ANTHROPIC_API_KEY=...` 줄을 확인하고 변경을 영구적으로 만들려면 제거하세요. Windows에서 `$PROFILE`의 PowerShell 프로필과 `ANTHROPIC_API_KEY`의 사용자 환경 변수를 확인하세요. Claude Code 내에서 `/status`를 실행하여 어느 인증 방법이 활성화되어 있는지 확인하세요.

<h3 id="oauth-login-fails-in-wsl2-ssh-or-containers">
  WSL2, SSH 또는 컨테이너에서 OAuth 로그인 실패
</h3>

Claude Code가 WSL2에서 실행되거나 SSH를 통해 원격 머신에서 실행되거나 컨테이너 내부에서 실행될 때 브라우저는 보통 다른 호스트에서 열리고 그 리다이렉트는 Claude Code의 로컬 콜백 서버에 도달할 수 없습니다. 로그인한 후 브라우저는 자동으로 다시 리다이렉트되지 않고 대신 로그인 코드를 표시합니다. 터미널의 `Paste code here if prompted` 프롬프트에 해당 코드를 붙여넣어 로그인을 완료하세요.

WSL2에서 브라우저가 전혀 열리지 않으면 `BROWSER` 환경 변수를 Windows 브라우저 경로로 설정하세요:

```bash theme={null}
export BROWSER="/mnt/c/Program Files/Google/Chrome/Application/chrome.exe"
claude
```

또는 대화형 로그인 프롬프트에서 `c`를 눌러 OAuth URL을 복사하거나 `claude auth login`이 출력하는 URL을 복사하고 로컬 머신의 브라우저에서 열어보세요.

대화형 프롬프트에 코드를 붙여넣어도 아무것도 하지 않으면 터미널의 붙여넣기 바인딩이 입력 필드에 도달하지 않을 가능성이 높습니다. 터미널의 대체 붙여넣기 바로 가기를 시도해보세요. 종종 Windows Terminal에서 우클릭 또는 Shift+Insert이거나 대신 `claude auth login`을 사용하세요. 이는 표준 입력에서 붙여넣은 코드를 읽습니다:

```bash theme={null}
claude auth login
```

이 폴백은 네이티브 Windows 또는 대화형 프롬프트에 붙여넣기가 실패하는 모든 터미널에도 적용됩니다.

<h3 id="not-logged-in-or-token-expired">
  로그인하지 않음 또는 토큰 만료
</h3>

Claude Code가 세션 후 다시 로그인하도록 요청하면 OAuth 토큰이 만료되었을 수 있습니다.

`/login`을 실행하여 다시 인증하세요. 이것이 자주 발생하면 토큰 검증이 올바른 타임스탬프에 따라 달라지므로 시스템 시계가 정확한지 확인하세요.

한 머신의 병렬 세션은 저장된 로그인을 공유하고 그 갱신을 조정하여 한 번에 하나의 프로세스만 토큰을 새로고침합니다. v2.1.211 이전에는 머신을 절전 모드에서 깨우면 두 세션이 같은 토큰으로 갱신될 수 있어서 저장된 로그인이 취소되고 모든 열린 세션이 한 번에 다시 로그인하도록 요청받았습니다.

macOS에서 Claude Code는 자격증명을 로그인 Keychain에 저장합니다. Keychain이 쓰기를 거부할 때(예: SSH 세션에서 잠겨 있거나 암호가 계정 암호와 동기화되지 않은 경우) Claude Code는 대신 자격증명을 일반 텍스트 `~/.claude/.credentials.json` 파일에 저장합니다. API 키를 생성하는 Console 로그인은 Keychain이 다시 쓰기 가능해질 때까지 실패합니다.

Keychain을 다시 쓰기 가능하게 만들고 로그인을 암호화된 Keychain으로 다시 이동하려면:

<Steps>
  <Step title="Keychain 액세스 확인">
    `claude doctor`를 실행하여 Keychain 액세스를 확인하세요. Keychain이 쓰기를 거부할 때 보고서는 `macOS Keychain is not writable`로 시작하는 경고를 나열하고 그 뒤에 제안된 수정 사항이 있습니다. 보고서에 Keychain 경고가 없으면 Keychain은 쓰기 가능하며 마지막 단계로 건너뛸 수 있습니다.
  </Step>

  <Step title="Keychain 잠금 해제">
    ```bash theme={null}
    security unlock-keychain ~/Library/Keychains/login.keychain-db
    ```

    명령이 요청할 때 Keychain 암호를 입력한 다음 `claude doctor`를 다시 실행하세요. 잠금 해제가 작동했으면 보고서는 더 이상 Keychain 경고를 나열하지 않습니다.
  </Step>

  <Step title="잠금 해제가 도움이 되지 않으면 Keychain 암호 재동기화">
    Keychain Access를 열고 `login` keychain을 선택한 다음 **편집 > Keychain "login"의 암호 변경**을 선택하여 계정 암호와 재동기화하세요. 그런 다음 `claude doctor`를 다시 실행하세요. 보고서에 더 이상 Keychain 경고가 없으면 다음 단계로 진행하세요.
  </Step>

  <Step title="로그아웃 후 다시 로그인">
    Keychain이 다시 쓰기 가능해지면 Claude Code는 다음 번에 자격증명을 쓸 때 자격증명을 다시 Keychain으로 이동합니다. 지금 강제하려면 `/logout`을 실행한 다음 `/login`을 실행하세요. 로그아웃하면 일반 텍스트 파일의 내용, 저장된 MCP 서버 로그인 및 플러그인 민감한 값을 포함한 모든 저장된 자격증명이 제거되므로 그 후에 MCP 서버를 다시 인증하고 플러그인 비밀을 다시 입력해야 합니다. 다시 로그인하면 Keychain에 로그인이 저장됩니다.
  </Step>
</Steps>

<h3 id="bedrock-agent-platform-or-foundry-credentials-not-loading">
  Bedrock, Agent Platform 또는 Foundry 자격증명이 로드되지 않음
</h3>

Claude Code를 클라우드 공급자를 사용하도록 구성했고 Amazon Bedrock에서 `Could not load credentials from any providers`, Google Cloud의 Agent Platform에서 `Could not load the default credentials` 또는 Microsoft Foundry에서 `ChainedTokenCredential authentication failed`가 표시되면 클라우드 공급자 CLI가 현재 셸에서 인증되지 않았을 가능성이 높습니다.

Amazon Bedrock의 경우 AWS 자격증명이 유효한지 확인하세요:

```bash theme={null}
aws sts get-caller-identity
```

Google Cloud의 Agent Platform의 경우 `ANTHROPIC_VERTEX_PROJECT_ID` 및 `CLOUD_ML_REGION`이 셸에 설정되어 있는지 확인한 다음 애플리케이션 기본 자격증명을 설정하세요:

```bash theme={null}
gcloud auth application-default login
```

Microsoft Foundry의 경우 `ANTHROPIC_FOUNDRY_API_KEY`가 설정되어 있는지 확인하거나 Azure CLI로 로그인하여 기본 자격증명 체인이 계정을 찾을 수 있도록 하세요:

```bash theme={null}
az login
```

자격증명이 터미널에서 작동하지만 VS Code 또는 JetBrains 확장에서 작동하지 않으면 IDE 프로세스가 셸 환경을 상속하지 않았을 가능성이 높습니다. IDE의 자체 설정에서 공급자 환경 변수를 설정하거나 이미 내보낸 터미널에서 IDE를 시작하세요.

전체 공급자 설정은 [Amazon Bedrock](/docs/ko/amazon-bedrock), [Google Cloud의 Agent Platform](/docs/ko/google-vertex-ai) 또는 [Microsoft Foundry](/docs/ko/microsoft-foundry)를 참조하세요.

<h2 id="still-stuck">
  여전히 막혔습니다
</h2>

위의 어느 것도 문제를 해결하지 못하면:

1. [GitHub 저장소](https://github.com/anthropics/claude-code/issues)에서 알려진 문제를 확인하거나 운영 체제, 실행한 설치 명령 및 전체 오류 출력과 함께 새 문제를 열어보세요
2. `claude --version`이 작동하지만 다른 것이 잘못되면 `claude doctor`를 실행하여 자동화된 진단 보고서를 받으세요
3. 세션을 시작할 수 있으면 Claude Code 내에서 `/feedback`을 사용하여 문제를 보고하세요
4. 설치 문제가 아니라 계정 문제인 경우(예: 로그인 루프, 인식되지 않는 구독 또는 비활성화된 조직)에는 Anthropic 지원팀에 문의하세요. [claude.ai](https://claude.ai)에 로그인하고(Console 사용자: [platform.claude.com](https://platform.claude.com)), 왼쪽 아래의 이니셜을 클릭한 후 **도움말 받기**를 선택하세요. 전체 절차는 [지원을 받는 방법](https://support.claude.com/en/articles/9015913-how-to-get-support)을 참조하세요.
