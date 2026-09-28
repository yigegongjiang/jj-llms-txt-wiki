> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent SDK 문제 해결

> Claude Code CLI가 시작되지 않거나, CLI 프로세스가 종료되거나, 구조화된 출력 없이 성공적인 결과가 도착할 때 Agent SDK 오류를 수정합니다.

이 페이지는 CLI 시작, CLI 프로세스 종료 및 구조화된 출력의 Agent SDK 오류를 다룹니다. 이 페이지의 항목들은 표시되는 오류를 기준으로 정렬되어 있습니다. 각 항목은 원인과 해결 방법을 설명합니다.

기능과 관련된 증상(예: hook이 실행되지 않거나 skill이 사용되지 않음)은 해당 기능의 페이지에 문제 해결 섹션이 있습니다. 표는 각 증상을 다루는 섹션 또는 페이지를 나열합니다:

| 증상                                                                                                                                                                                                 | 이동                                                                                               |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------- |
| Skills를 찾을 수 없음, skill이 사용되지 않음, `Invalid skill name` 오류                                                                                                                                           | [Skills 문제 해결](/docs/ko/agent-sdk/skills#troubleshooting)                                             |
| MCP 서버가 `failed` 상태를 표시, 도구가 호출되지 않음, 연결 시간 초과, 최대 허용 토큰을 초과하는 도구 출력                                                                                                                               | [MCP 문제 해결](/docs/ko/agent-sdk/mcp#troubleshooting)                                                   |
| Plugin이 로드되지 않음, plugin skills이 나타나지 않음                                                                                                                                                            | [Plugins 문제 해결](/docs/ko/agent-sdk/plugins#troubleshooting)                                           |
| Claude가 subagents로 위임하지 않음, 파일 시스템 기반 agents가 로드되지 않음                                                                                                                                              | [Subagents 문제 해결](/docs/ko/agent-sdk/subagents#troubleshooting)                                       |
| Checkpointing 옵션이 인식되지 않음, UUID 없는 사용자 메시지, `No file checkpoint found`, `File rewinding is not enabled`, `ProcessTransport is not ready for writing`                                               | [File checkpointing 문제 해결](/docs/ko/agent-sdk/file-checkpointing#troubleshooting)                     |
| Hook이 실행되지 않음, matcher가 예상대로 필터링하지 않음, hook 시간 초과, 도구가 예기치 않게 차단됨, 수정된 입력이 적용되지 않음, Python에서 session hooks를 사용할 수 없음, subagent 권한 프롬프트가 증가함, subagents와의 재귀적 hook 루프, `systemMessage`가 출력에 나타나지 않음 | [일반적인 문제 해결](/docs/ko/agent-sdk/hooks#fix-common-issues) (hooks 페이지)                                  |
| 사용자의 머신에서 작동하는 agent가 배포된 서비스 또는 컨테이너에서 실패함                                                                                                                                                        | [배포 실패 문제 해결](/docs/ko/agent-sdk/hosting#troubleshoot-deployment-failures)                            |
| `Not logged in`, `Invalid API key`, `API Error`, `429`, `There's an issue with the selected model`                                                                                                 | [오류 참조](/docs/ko/errors#find-your-error)                                                              |
| `CLINotFoundError`, `CLIConnectionError`, `ProcessError`, `Claude Code process exited with code N`, `Claude Code returned an error result`, `structured_output`이 `None`임                           | 이 페이지의 [CLI 시작](#cli-startup), [CLI 프로세스 종료](#cli-process-exit) 및 [구조화된 출력](#structured-outputs) |

<h2 id="cli-startup">
  CLI 시작
</h2>

<h3 id="clinotfounderror-claude-code-not-found">
  CLINotFoundError: Claude Code not found
</h3>

Python SDK는 Claude Code CLI를 서브프로세스로 실행합니다. `claude` 실행 파일을 찾을 수 없으면 연결이 `CLINotFoundError`로 실패합니다:

```
Claude Code not found at: /your/configured/path
```

메시지에는 `ClaudeAgentOptions(cli_path=...)`를 설정했을 때 구성된 경로가 포함되며, 누락된 파일을 가리킵니다. `cli_path`가 없으면 SDK는 `PATH`와 일반적인 설치 위치를 검색하고, 메시지에는 플랫폼에 대한 설치 지침이 포함됩니다.

해결 방법:

* Claude Code가 설치되지 않았으면 설치합니다. 플랫폼의 명령어는 [Claude Code 설치](/docs/ko/setup#install-claude-code)를 참조하세요.
* `cli_path`를 설정했으면 파일이 존재하고 `claude` 실행 파일인지 확인합니다.
* `PATH` 해석에 의존하면 애플리케이션이 실행되는 동일한 환경에서 `claude --version`이 작동하는지 확인합니다. IDE나 서비스 관리자에서 실행하는 프로세스와 같이 셸 외부에서 실행하는 프로세스는 종종 다른 `PATH`로 실행됩니다.

TypeScript SDK는 번들된 플랫폼 패키지와 `pathToClaudeCodeExecutable`에 설정한 경로에서 CLI를 찾습니다. 표시되는 메시지와 일치시킵니다:

* `Native CLI binary for <platform>-<arch> not found`: 번들된 플랫폼 패키지가 누락되었으며, 대부분 설치 시 선택적 종속성을 건너뛰었기 때문입니다. 선택적 종속성을 건너뛰지 않고 `@anthropic-ai/claude-agent-sdk`를 다시 설치하거나, `pathToClaudeCodeExecutable`을 [네이티브 설치](/docs/ko/setup#install-claude-code)로 지정합니다. `bun build --compile`로 빌드한 단일 파일 실행 파일에서는 동일한 메시지가 다른 원인과 해결 방법을 가집니다. [단일 실행 파일로 컴파일](/docs/ko/agent-sdk/typescript#compile-to-a-single-executable)을 참조하세요.
* `Claude Code native binary not found at <path>` 또는 `Claude Code executable not found at <path>. Is options.pathToClaudeCodeExecutable set?`: 해석된 경로의 파일이 누락되었거나 프로세스가 접근할 수 없습니다. 파일이 해당 경로에 존재하고 프로세스가 접근할 수 있는지 확인합니다.

<h3 id="cliconnectionerror-refusing-to-execute-batch-script">
  CLIConnectionError: Refusing to execute batch script
</h3>

Windows에서 Python SDK가 사용하는 CLI 경로가 `.bat` 또는 `.cmd` 배치 스크립트(npm 설치가 생성하는 `claude.cmd` shim 포함)일 때 연결이 `CLIConnectionError`로 실패합니다:

```
Refusing to execute batch script 'C:\\Users\\you\\AppData\\Roaming\\npm\\claude.cmd': Windows runs .bat/.cmd files via cmd.exe, which can execute commands injected through CLI arguments, and no reliable escaping for cmd.exe exists. Use a native claude executable instead: install Claude Code natively (irm https://claude.ai/install.ps1 | iex), point ClaudeAgentOptions(cli_path=...) at a claude.exe, or install the claude-agent-sdk wheel for a platform that bundles claude.exe (e.g. Windows x64).
```

거부는 의도적인 보안 강화이며, 손상된 설치가 아닙니다. Windows는 배치 스크립트를 `cmd.exe /c` 호출로 다시 작성하여 실행하고, `cmd.exe`는 실행 시 전체 명령줄을 다시 구문 분석하므로 인수 값이 주입된 명령을 실행할 수 있습니다.

대부분의 Windows 설치는 이 오류에 도달하지 않습니다. Windows x64 `claude-agent-sdk` 휠은 `claude.exe`를 번들로 포함하고, SDK는 번들된 CLI를 선호한 다음 발견할 수 있는 모든 네이티브 `claude.exe`를 선호하고, 배치 shim으로 폴백하기 전에 선호합니다. 두 가지 경우에 거부가 표시됩니다:

* `ClaudeAgentOptions(cli_path=...)`를 npm의 `claude.cmd` shim과 같은 `.bat` 또는 `.cmd` 파일로 설정했습니다.
* 설치에 번들된 또는 네이티브 `claude.exe`가 없습니다. 예를 들어 `PATH`의 유일한 `claude`가 npm shim인 ARM64 Windows의 소스 설치입니다.

해결 방법은 배치 스크립트 대신 네이티브 실행 파일을 SDK에 제공하는 것입니다:

* `ClaudeAgentOptions(cli_path=...)`를 설정했으면 `claude.exe`로 지정하거나 옵션을 제거합니다. SDK는 `cli_path`가 설정된 동안 검색을 건너뛰므로 네이티브 설치만으로는 효과가 없습니다.
* PowerShell에서 Claude Code를 네이티브로 설치합니다: `irm https://claude.ai/install.ps1 | iex`
* x64 Windows에서 `claude.exe`를 번들로 포함하는 `claude-agent-sdk` 휠을 설치합니다.

`claude-agent-sdk` 0.2.124 이전에는 Python SDK가 이 확인 없이 `cmd.exe`를 통해 배치 스크립트를 생성했습니다.

<h3 id="cliconnectionerror-failed-to-start-claude-code">
  CLIConnectionError: Failed to start Claude Code
</h3>

SDK는 해석된 경로에서 파일을 찾았지만 실행할 수 없었습니다. Python은 이러한 실패를 `CLIConnectionError`로 발생시킵니다. TypeScript는 SDK 클래스가 없는 오류로 메시지 반복을 거부합니다. 아래 표는 각 메시지를 그것이 알려주는 것으로 매핑합니다. 표시되는 메시지와 일치시킵니다:

| 메시지                                                               | SDK        | 알려주는 것                               |
| ----------------------------------------------------------------- | ---------- | ------------------------------------ |
| `Failed to start Claude Code: <detail>`                           | Python     | 메시지의 나머지는 운영 체제 자체의 오류입니다            |
| `Claude Code executable at <path> exists but failed to launch`    | TypeScript | 구성된 경로의 스크립트를 실행할 수 없습니다             |
| `Claude Code native binary at <path> exists but failed to launch` | TypeScript | 바이너리를 실행할 수 없으며, libc 제안이 메시지에 추가됩니다 |
| `Failed to spawn Claude Code process: <detail>`                   | TypeScript | 다른 모든 실행 실패                          |

두 SDK 모두에서 일반적인 원인은 텍스트 파일, 디렉토리 또는 실행 권한이 없는 파일과 같이 실행할 수 없는 것을 가리키는 해석된 경로입니다. 네이티브 바이너리 메시지의 libc 제안을 가능한 원인 중 하나로 읽습니다.

두 SDK 모두에서 해결하려면:

* 구성된 경로가 `claude` 실행 파일 자체를 가리키고 파일에 실행 권한이 있는지 확인합니다.
* 사용자 정의 경로가 필요하지 않으면 Python에서 `cli_path`를 제거하거나 TypeScript에서 `pathToClaudeCodeExecutable`을 제거하여 SDK가 자체적으로 CLI를 찾도록 하고, 번들된 복사본을 선호합니다.
* 실패한 바이너리가 컨테이너 이미지의 SDK 번들 복사본일 때 이미지 빌드 중에 SDK를 다시 설치하여 번들된 바이너리가 컨테이너의 플랫폼과 일치하도록 하거나, 실행되는 아키텍처에 대해 이미지를 다시 빌드합니다. 일반적인 원인은 컨테이너의 아키텍처 또는 libc와 일치하지 않는 바이너리이거나 이미지 빌드에서 실행 권한을 잃은 바이너리입니다.

<h3 id="cliconnectionerror-not-connected">
  CLIConnectionError: Not connected
</h3>

Python에서 클라이언트가 연결되기 전에 또는 연결이 끊긴 후에 `ClaudeSDKClient` 메서드를 호출하면 다음 메시지와 함께 `CLIConnectionError`가 발생합니다:

```
Not connected. Call connect() first.
```

메시지가 말하는 대로 하세요. 다른 클라이언트 메서드 전에 `await client.connect()`를 호출하거나 `async with ClaudeSDKClient() as client:`로 클라이언트를 열어서 진입 시 연결합니다.

<h2 id="cli-process-exit">
  CLI 프로세스 종료
</h2>

이 섹션의 항목들은 애플리케이션이 사용 중일 때 Claude Code 프로세스가 종료되었음을 의미합니다. 표시되는 오류는 SDK 언어와 CLI가 종료 전에 오류 결과를 보고했는지 여부에 따라 달라집니다.

<h3 id="processerror-command-failed-with-exit-code">
  ProcessError: Command failed with exit code
</h3>

Python SDK는 Claude Code 프로세스가 0이 아닌 코드로 종료될 때 `ProcessError`를 발생시킵니다:

```
Command failed with exit code 1 (exit code: 1)
Error output: Check stderr output for details
```

메시지는 종료 코드를 두 번 나타내고, `Error output` 줄은 프로세스의 실제 오류 출력이 아닌 고정 텍스트입니다. 동일한 고정 텍스트가 예외의 `stderr` 속성을 채웁니다. 예외의 `exit_code` 속성은 코드를 전달합니다. CLI가 실제로 stderr에 작성한 내용을 캡처하려면 `ClaudeAgentOptions`에서 `stderr` 콜백을 전달하고 수신한 내용을 기록합니다.

단순한 `ProcessError`는 CLI가 오류 결과를 보고하지 않고 종료되었음을 의미합니다. CLI가 보고했을 때 SDK는 대신 [`ResultError`](/docs/ko/agent-sdk/python#resulterror)를 발생시키며, [Claude Code returned an error result](#claude-code-returned-an-error-result)에서 다룹니다. `ResultError`는 `ProcessError`를 서브클래싱하므로 `except ProcessError`는 둘 다 캐치합니다. 다르게 처리하려면 `except ResultError` 절을 먼저 배치합니다.

`claude-agent-sdk` 0.2.140 이전에는 Python SDK가 오류 결과 종료를 `ResultError` 대신 일반 `Exception`으로 발생시켰습니다.

<h3 id="claude-code-process-exited-with-code-n">
  Claude Code process exited with code N
</h3>

IDE 래퍼도 이 메시지를 인쇄하고, [오류 참조](/docs/ko/errors#claude-code-process-exited-with-code-n)는 VS Code 및 기타 런처에 대해 다룹니다. 이 항목은 TypeScript SDK 코드가 수신하는 것을 다룹니다. SDK는 0이 아닌 CLI 종료를 `query()`의 메시지에 대한 `for await` 루프를 거부하는 일반 `Error`로 표시합니다. 캐치할 SDK 오류 클래스가 없으므로 루프를 `try`/`catch`로 래핑하고 메시지와 일치시킵니다:

```
Claude Code process exited with code 1. stderr: <tail of the CLI's stderr>
```

CLI가 stderr에 작성했을 때 메시지는 그 끝으로 끝납니다. 전체 스트림을 캡처하려면 쿼리 옵션에서 `stderr` 콜백을 전달합니다. 신호로 종료된 프로세스는 동일한 형식으로 `Claude Code process terminated by signal <name>`을 보고합니다.

<h3 id="claude-code-returned-an-error-result">
  Claude Code returned an error result
</h3>

두 SDK 모두 CLI가 종료 전에 오류 결과를 보고했을 때 프로세스 종료 오류를 이 메시지로 바꿉니다:

```
Claude Code returned an error result: <the CLI's own error report>
```

콜론 뒤의 텍스트는 무엇이 잘못되었는지에 대한 CLI의 보고이므로 종료 자체보다는 여기서 시작합니다. Python은 이를 [`ResultError`](/docs/ko/agent-sdk/python#resulterror)로 발생시키며, 그 `data` 속성은 전체 오류 결과를 전달합니다. TypeScript는 동일한 메시지 형태를 전달하는 일반 `Error`로 메시지 루프를 거부합니다.

<h2 id="structured-outputs">
  구조화된 출력
</h2>

<h3 id="structured_output-is-none-but-the-result-says-success">
  structured\_output is None but the result says success
</h3>

결과 메시지는 `subtype: "success"`로 끝날 수 있지만 Python에서 `structured_output`은 `None`이거나 TypeScript에서 `undefined`입니다. 실행이 완료되지만 검증된 출력이 없습니다. 이를 발생시키는 한 가지 방법은 충돌하는 길이 제약과 같이 출력이 만족할 수 없는 스키마입니다. 실행은 검증 오류 없이 끝나고 유일한 신호는 누락된 `structured_output`입니다.

애플리케이션 코드에서 이 결과를 실패로 취급합니다. `structured_output`을 사용하기 전에 `subtype`이 `success`이고 `structured_output`이 존재하는지 확인합니다. [오류 처리](/docs/ko/agent-sdk/structured-outputs#error-handling) 섹션은 두 SDK 모두에 대해 이 패턴을 보여줍니다.

올바르다고 생각하는 스키마에서 반복적으로 발생하면 스키마가 만족 가능한지 확인한 다음 출력이 검증될 때까지 단순화하고 제약을 한 번에 하나씩 다시 도입합니다.

<h2 id="report-a-new-issue">
  새 문제 보고
</h2>

오류가 여기에 포함되지 않으면 SDK 저장소에서 열린 문제를 확인하거나 새 문제를 제출합니다: [claude-agent-sdk-typescript](https://github.com/anthropics/claude-agent-sdk-typescript/issues) 또는 [claude-agent-sdk-python](https://github.com/anthropics/claude-agent-sdk-python/issues). 전체 오류 텍스트와 SDK 버전을 포함합니다.
