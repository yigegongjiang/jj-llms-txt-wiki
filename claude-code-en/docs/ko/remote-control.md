> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 모든 기기에서 로컬 세션 계속하기 (Remote Control)

> Remote Control을 사용하여 휴대폰, 태블릿 또는 모든 브라우저에서 로컬 Claude Code 세션을 계속할 수 있습니다. claude.ai/code 및 Claude 모바일 앱과 함께 작동합니다.

<Note>
  Remote Control은 모든 요금제에서 사용할 수 있습니다. Team 및 Enterprise의 경우 소유자가 [Claude Code 관리자 설정](https://claude.ai/admin-settings/claude-code)에서 Remote Control 토글을 활성화할 때까지 기본적으로 꺼져 있습니다.
</Note>

Remote Control은 [claude.ai/code](https://claude.ai/code) 또는 [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) 및 [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude)용 Claude 앱을 컴퓨터에서 실행 중인 Claude Code 세션에 연결합니다. 책상에서 작업을 시작한 다음 소파의 휴대폰이나 다른 컴퓨터의 브라우저에서 계속할 수 있습니다.

컴퓨터에서 Remote Control 세션을 시작하면 Claude는 전체 시간 동안 로컬에서 실행되므로 코드 실행 및 파일 시스템 접근이 컴퓨터에 유지됩니다. Remote Control을 사용하면 다음을 수행할 수 있습니다:

* **전체 로컬 환경을 원격으로 사용**: 파일 시스템, [MCP servers](/docs/ko/mcp), 도구 및 프로젝트 구성이 모두 사용 가능하게 유지되며, `@`를 입력하면 로컬 프로젝트의 파일 경로가 자동 완성됩니다.
* **두 표면에서 동시에 작업**: 대화 및 [subagents](/docs/ko/sub-agents) 및 [dynamic workflows](/docs/ko/workflows)의 진행 상황이 모든 연결된 기기에서 동기화되므로 터미널, 브라우저 및 휴대폰에서 메시지를 교대로 보낼 수 있습니다.
* **휴대폰 또는 브라우저에서 이미지 및 파일 전송**: Claude 앱 또는 claude.ai/code에서 사진 또는 파일을 첨부하면 Claude는 첨부된 사진을 메시지의 일부로 직접 봅니다. Claude Code는 다른 파일을 컴퓨터에 다운로드하고 `@` 파일 참조로 Claude에 전달합니다.
* **중단 극복**: 노트북이 절전 모드로 전환되거나 네트워크가 끊어지면 컴퓨터가 다시 온라인 상태가 될 때 Claude Code가 자동으로 다시 연결됩니다. 연결이 재구축되는 동안 Claude Code는 메시지, 권한 프롬프트 및 subagents 및 workflows의 상태 업데이트를 대기열에 넣고 연결이 복구되면 전달합니다.

클라우드 인프라에서 실행되는 [웹의 Claude Code](/docs/ko/claude-code-on-the-web)와 달리 Remote Control 세션은 컴퓨터에서 직접 실행되며 로컬 파일 시스템과 상호 작용합니다. 웹 및 모바일 인터페이스는 단지 해당 로컬 세션의 창일 뿐입니다.

이 페이지에서는 설정, 세션을 시작하고 연결하는 방법, Remote Control과 웹의 Claude Code를 비교하는 방법을 다룹니다.

<h2 id="requirements">
  요구 사항
</h2>

Remote Control을 사용하기 전에 환경이 다음 조건을 충족하는지 확인하세요:

* **구독**: Pro, Max, Team 및 Enterprise 요금제에서 사용 가능합니다. API 키는 지원되지 않습니다. Team 및 Enterprise의 경우 Owner가 먼저 [Claude Code 관리자 설정](https://claude.ai/admin-settings/claude-code)에서 Remote Control 토글을 활성화해야 합니다.
* **인증**: `claude`를 실행하고 아직 로그인하지 않았다면 `/login`을 사용하여 claude.ai를 통해 로그인하세요. 적격 로그인이 없으면 `claude remote-control`은 오류로 종료되고, `claude --remote-control`은 여전히 대화형 세션을 시작하며 시작 직후 Remote Control 실패 알림을 표시합니다.
* **API 엔드포인트**: 다음 구성 중 어느 것도 사용할 수 없습니다:
  * Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry를 사용합니다.
  * [`ANTHROPIC_BASE_URL`](/docs/ko/env-vars)이 `api.anthropic.com` 이외의 호스트(예: [LLM gateway](/docs/ko/llm-gateway) 또는 프록시)를 가리킵니다. Remote Control을 사용하려면 변수를 설정 해제하세요. v2.1.196 이전에는 Claude Code가 사용자 정의 `ANTHROPIC_BASE_URL`로 Remote Control을 허용했습니다.
  * enterprise [Claude apps gateway](/docs/ko/claude-apps-gateway)를 통해 로그인합니다.
* **기능 플래그 평가**: [`DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` 및 `DISABLE_GROWTHBOOK`](/docs/ko/env-vars)은 각각 Remote Control 가용성이 의존하는 기능 플래그 평가를 비활성화합니다. Remote Control을 사용하려면 셸 환경이나 [`settings.json` 파일](/docs/ko/settings-reference#all-settings)의 `env` 블록에서 변수가 설정된 곳 어디든 설정 해제하세요.
* **작업 공간 신뢰**: 작업 공간 신뢰 대화를 수락하려면 프로젝트 디렉토리에서 최소한 한 번 `claude`를 실행하세요. 시작 신뢰 대화는 홈 디렉토리에 대한 신뢰를 저장하지 않으므로 프로젝트 디렉토리에서 Remote Control을 시작하세요.

<h2 id="start-a-remote-control-session">
  Remote Control 세션 시작
</h2>

CLI 또는 VS Code 확장에서 Remote Control 세션을 시작할 수 있습니다. CLI는 세 가지 호출 모드를 제공하며, VS Code는 `/remote-control` 명령을 사용합니다.

<Tabs>
  <Tab title="서버 모드">
    프로젝트 디렉토리에서 다음을 실행하세요:

    ```bash theme={null}
    claude remote-control
    ```

    Remote Control의 일회성 확인을 수락할 때까지 `claude remote-control`은 수행하는 작업을 설명하고 시작하기 전에 `Enable Remote Control? (y/n)`을 묻습니다. `y`를 입력하여 수락하고 서버를 시작하세요. 거절하면 Claude Code는 서버를 시작하지 않고 종료되며 다음에 명령을 실행할 때 다시 묻습니다.

    프로세스는 터미널에서 서버 모드로 계속 실행되어 원격 연결을 기다립니다. [다른 기기에서 연결](#connect-from-another-device)하는 데 사용할 수 있는 세션 URL을 표시하며, 스페이스바를 눌러 휴대폰에서 빠르게 액세스할 수 있는 QR 코드를 표시할 수 있습니다. 원격 세션이 활성화되어 있는 동안 터미널은 연결 상태 및 도구 활동을 표시합니다.

    사용 가능한 플래그:

    | 플래그                                             | 설명                                                                                                                                                                                                                                                                                                                     |
    | ----------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | `--name "My Project"`                           | claude.ai/code의 세션 목록에 표시되는 사용자 정의 세션 제목을 설정합니다.                                                                                                                                                                                                                                                                       |
    | `--remote-control-session-name-prefix <prefix>` | 명시적 이름이 설정되지 않았을 때 자동 생성된 세션 이름의 접두사입니다. 기본값은 컴퓨터의 호스트 이름이며, `myhost-graceful-unicorn`과 같은 이름을 생성합니다. 동일한 효과를 위해 `CLAUDE_REMOTE_CONTROL_SESSION_NAME_PREFIX`를 설정하세요.                                                                                                                                                   |
    | `-c`, `--continue`                              | 이 디렉토리에서 마지막 서버가 시작한 세션을 다시 가져오며, 새로운 세션을 만드는 대신 사용합니다. [서버를 중지한 후 세션 재개](#resume-sessions-after-stopping-the-server)를 참조하세요. `--session-id`, `--spawn`, `--capacity` 또는 `--create-session-in-dir`과 함께 사용할 수 없습니다. Claude Code v2.1.200 이상이 필요하며, 이전 버전은 이 플래그를 알 수 없는 인수로 거부합니다.                                      |
    | `--session-id <id>`                             | ID로 특정 세션을 다시 가져옵니다. [서버를 중지한 후 세션 재개](#resume-sessions-after-stopping-the-server)를 참조하세요. `--continue`, `--spawn`, `--capacity` 또는 `--create-session-in-dir`과 함께 사용할 수 없습니다. Claude Code v2.1.200 이상이 필요하며, 이전 버전은 이 플래그를 알 수 없는 인수로 거부합니다.                                                                           |
    | `--spawn <mode>`                                | 서버가 세션을 생성하는 방식입니다.<br />• `same-dir` (기본값): 모든 세션이 현재 작업 디렉토리를 공유하므로 동일한 파일을 편집할 때 충돌할 수 있습니다.<br />• `worktree`: 각 온디맨드 세션은 자체 [git worktree](/docs/ko/worktrees)를 가져옵니다. git 저장소가 필요합니다.<br />• `session`: 단일 세션 모드입니다. 정확히 하나의 세션을 제공하고 추가 연결을 거부합니다. 시작 시에만 설정합니다.<br />런타임에 `w`를 눌러 `same-dir`과 `worktree` 사이를 전환하세요. |
    | `--capacity <N>`                                | 최대 동시 세션 수입니다. 기본값은 32입니다. `--spawn=session`과 함께 사용할 수 없습니다.                                                                                                                                                                                                                                                           |
    | `--[no-]create-session-in-dir`                  | 서버가 시작할 때 현재 디렉토리에 하나의 세션을 미리 생성하여 즉시 입력할 수 있는 위치를 제공합니다. `worktree` 모드에서 이 세션은 현재 디렉토리에 유지되고 온디맨드 세션은 격리된 worktree를 가져옵니다. 기본적으로 켜져 있습니다. `--no-create-session-in-dir`을 전달하여 아무것도 없이 시작하면 Claude Code는 서버의 세션을 보관하므로 [재개](#resume-sessions-after-stopping-the-server)할 것이 없습니다.                                       |
    | `--permission-mode <mode>`                      | 서버의 세션에 대한 시작 [권한 모드](/docs/ko/permission-modes)를 설정합니다(예: `acceptEdits`). `manual`을 `default`의 별칭으로 허용합니다. 인식되지 않는 모드는 시작 시 서버를 중지하고 유효한 모드를 나열합니다.                                                                                                                                                                        |
    | `--debug-file <path>`                           | 주어진 파일에 디버그 로그를 작성합니다.                                                                                                                                                                                                                                                                                                 |
    | `--verbose`                                     | 자세한 연결 및 세션 로그를 표시합니다.                                                                                                                                                                                                                                                                                                 |
    | `--sandbox` / `--no-sandbox`                    | 파일 시스템 및 네트워크 격리를 위해 [샌드박싱](/docs/ko/sandboxing)을 활성화하거나 비활성화합니다. 기본적으로 꺼져 있습니다.                                                                                                                                                                                                                                            |

    `remote-control` 뒤에 이 플래그들을 입력하세요.

    `remote-control` 앞에 전역 `claude` 플래그를 전달하거나 래퍼 스크립트가 하나를 추가하면 Claude Code는 서버가 생성하는 세션으로 플래그를 이월하지 않습니다. Claude Code는 `--verbose` 또는 `--model`과 같이 플래그를 삭제해도 해당 세션이 수행할 수 있는 작업이 변경되지 않는 경우에만 플래그를 통과시킵니다. 다른 플래그(예: `--settings`)의 경우 Claude Code는 [시작을 거부](/docs/ko/errors#not-carried-over-to-the-sessions-remote-control-starts)하고 제거할 플래그를 이름으로 지정합니다. v2.1.248 이전에는 `remote-control` 앞의 모든 옵션이 Claude Code가 그 뒤의 플래그를 `unknown option` 오류로 거부하게 했습니다.

    Claude Code는 도움말을 인쇄하기 전에 Remote Control 적격성을 확인하므로 적격 계정으로 로그인하지 않았을 때 `claude remote-control --help`는 이 플래그 목록 대신 오류를 반환합니다.
  </Tab>

  <Tab title="대화형 세션">
    Remote Control이 활성화된 일반 대화형 Claude Code 세션을 시작하려면 `--remote-control` 플래그(또는 `--rc`)를 사용하세요:

    ```bash theme={null}
    claude --remote-control
    ```

    선택적으로 세션의 이름을 전달하세요:

    ```bash theme={null}
    claude --remote-control "My Project"
    ```

    이렇게 하면 터미널에서 전체 대화형 세션을 얻을 수 있으며, claude.ai 또는 Claude 앱에서도 제어할 수 있습니다. `claude remote-control`(서버 모드)과 달리 세션이 원격으로도 사용 가능한 동안 로컬에서 메시지를 입력할 수 있습니다.
  </Tab>

  <Tab title="기존 세션에서">
    이미 Claude Code 세션에 있고 원격으로 계속하려면 `/remote-control`(또는 `/rc`) 명령을 사용하세요:

    ```text theme={null}
    /remote-control
    ```

    인수로 이름을 전달하여 사용자 정의 세션 제목을 설정하세요:

    ```text theme={null}
    /remote-control My Project
    ```

    이렇게 하면 현재 대화 기록을 이어받는 Remote Control 세션이 시작됩니다.

    Remote Control의 일회성 확인을 수락할 때까지 `/remote-control`이 연결되기 전에 대화 상자가 나타납니다. **Enable Remote Control**을 선택하여 수락하고 연결하세요. **Never mind**를 선택하거나 Esc를 누르면 Claude Code는 연결하지 않으며 다음에 `/remote-control`을 실행할 때 다시 묻습니다.

    `--verbose`, `--sandbox` 및 `--no-sandbox` 플래그는 이 명령에서 사용할 수 없습니다.
  </Tab>

  <Tab title="VS Code">
    [Claude Code VS Code 확장](/docs/ko/vs-code)에서 프롬프트 상자에 `/remote-control` 또는 `/rc`를 입력하세요.

    ```text theme={null}
    /remote-control
    ```

    Remote Control이 켜져 있는 동안 Claude Code는 프롬프트 상자 바닥글에 **Remote Control** 표시기를 표시합니다. 세션이 연결되면 표시기를 클릭하여 세션으로 직접 이동하거나 [claude.ai/code](https://claude.ai/code)의 세션 목록에서 찾으세요. Claude Code는 또한 대화에 세션 URL을 게시합니다. 연결을 끊으려면 `/remote-control`을 다시 실행하세요.

    CLI와 달리 VS Code 명령은 이름 인수를 허용하지 않으며 QR 코드를 표시하지 않습니다. 세션 제목은 대화 기록 또는 첫 번째 프롬프트에서 파생됩니다.
  </Tab>
</Tabs>

<h3 id="check-connection-status">
  연결 상태 확인
</h3>

대화형 세션에서 Remote Control이 연결되어 있는 동안 터미널은 `/rc active` 표시기를 표시하며, 이는 claude.ai의 세션으로 연결됩니다. 터미널이 너무 좁으면 표시기가 숨겨집니다. [다른 기기에서 연결](#connect-from-another-device)하는 데 사용할 수 있는 세션 URL과 QR 코드를 보려면 `/remote-control`을 다시 실행하여 상태 패널을 엽니다. 패널은 또한 Remote Control을 연결 해제할 수 있게 하며, 로컬 세션은 계속 실행됩니다.

<span id="session-ended-elsewhere" />대화형 세션에서 연결이 실패하면 표시기가 실패를 표시하도록 변경되고, Claude Code는 이유를 알림으로 표시하고 대화에 추가합니다. 다시 연결하려면 `/remote-control`을 실행하세요. 단, 이유가 다음 중 하나를 말하는 경우는 제외합니다:

* **다른 연결이 이 세션을 인수했습니다**: 다른 기기 또는 Claude Code 세션이 이제 이를 가지고 있습니다. 세션을 다시 가져오려는 경우에만 `/remote-control`을 실행하세요.
* **이 세션이 다른 기기 또는 앱에서 종료되거나 보관되었습니다**: 세션을 다시 원하는 경우에만 `/remote-control`을 실행하세요. Claude Code는 보관된 세션을 다시 엽니다.
* **서버가 더 이상 이 세션을 보고하지 않습니다**: 다른 기기 또는 앱에서 삭제되었을 수 있습니다.

<h3 id="session-url-reminders">
  세션 URL 미리 알림
</h3>

Remote Control이 연결되어 있는 동안 Claude Code는 휴대폰 또는 브라우저로 전환하는 것이 가장 도움이 될 때 세션 URL을 상기시켜 주므로 `/remote-control`에서 링크를 찾을 필요가 없습니다. 다음 중 하나의 순간에 프롬프트 상자 위에 미리 알림이 나타납니다:

* **긴 턴**: 턴이 서버 조정 임계값보다 오래 실행될 때 Claude Code는 **Still working** 알림과 **Check in from your phone** 링크를 표시하므로 터미널에서 기다리는 대신 휴대폰 또는 브라우저에서 턴을 따를 수 있습니다. Claude Code는 턴이 끝나면 이를 제거합니다.
* **반복된 권한 프롬프트**: 세션에서 여러 [권한 프롬프트](/docs/ko/permissions)에 답한 후 **Approve tool calls from your phone** 알림이 세션 URL을 표시합니다. Claude Code는 다음 턴이 시작되면 이를 제거합니다.

미리 알림은 Remote Control이 [자동으로 연결](#enable-remote-control-for-all-sessions)되는 세션을 포함하여 연결된 모든 세션에 나타날 수 있습니다. 이러한 조건이 발생할 때마다 나타나지는 않으며 각각은 세션 전체에서 몇 번만 나타납니다. 이를 구성하거나 끌 수 없습니다. 각각은 자체적으로 지워집니다.

<h3 id="connect-from-another-device">
  다른 기기에서 연결
</h3>

Remote Control 세션이 활성화되면 다른 기기에서 연결하는 몇 가지 방법이 있습니다:

* **세션 URL 열기**: 모든 브라우저에서 URL을 열어 [claude.ai/code](https://claude.ai/code)의 세션으로 직접 이동합니다.
* **QR 코드 스캔**: 세션 URL 옆에 표시된 QR 코드를 스캔하여 Claude 앱에서 직접 열 수 있습니다. `claude remote-control`을 사용하면 스페이스바를 눌러 QR 코드 표시를 전환할 수 있습니다.
* **[claude.ai/code](https://claude.ai/code) 또는 Claude 앱 열기**: 세션 목록에서 이름으로 세션을 찾습니다. Claude 모바일 앱에서 네비게이션의 **코드**를 탭하여 세션 목록에 도달하세요. Remote Control 세션은 온라인 상태일 때 녹색 상태 점이 있는 컴퓨터 아이콘을 표시합니다.

연결하면 기기에 세션이 이미 백그라운드에서 실행 중인 모든 서브에이전트 및 워크플로우가 표시됩니다. 기기에서 그 중 하나를 중지하면 Claude Code는 컴퓨터에서 해당 작업을 중지합니다.

원격 세션 제목은 다음 순서로 선택됩니다:

1. `--name`, `--remote-control` 또는 `/remote-control`에 전달한 이름
2. `/rename`으로 설정한 제목
3. 기존 대화 기록의 마지막 의미 있는 메시지
4. `myhost-graceful-unicorn`과 같은 자동 생성된 이름입니다. 여기서 `myhost`는 컴퓨터의 호스트 이름 또는 `--remote-control-session-name-prefix`로 설정한 접두사입니다.

명시적 이름을 설정하지 않았다면 메시지를 보낸 후 제목이 프롬프트를 반영하도록 업데이트됩니다. Claude Code는 자동 생성된 제목을 대화의 언어 또는 구성된 [`language`](/docs/ko/settings-reference#language) 설정과 일치시킵니다.

claude.ai 또는 Claude 앱에서 세션의 이름을 바꾸면 `claude --resume`에 표시되는 로컬 제목도 업데이트됩니다. Claude Code는 동일한 이름 바꾸기를 프롬프트 바에 표시된 세션 이름과 세션이 [백그라운드에서 실행](/docs/ko/agent-view)될 때 `claude agents` 목록에 적용합니다. v2.1.221 이전에는 claude.ai의 세션 목록 또는 Claude 앱에서 이름을 바꾸면 제목만 업데이트되었고 CLI는 이전 세션 이름을 유지했습니다. CLI 자체에서 실행되는 `/rename`은 모든 버전에서 이름을 설정합니다.

Claude 앱이 아직 없으면 Claude Code 내에서 `/mobile`을 실행하여 [claude.ai/mobile](https://claude.ai/mobile)에 대한 QR 코드를 표시하세요. 이는 휴대폰에 맞는 앱 스토어를 엽니다.

<h3 id="what-connected-devices-see">
  연결된 기기가 보는 것
</h3>

연결된 기기는 터미널의 대화를 실시간으로 표시합니다. 이러한 경우는 일반 메시지를 넘어갑니다:

* **압축 및 `/clear`**: Claude Code가 [대화를 압축](/docs/ko/context-window#what-survives-compaction)하는 동안 연결된 기기는 진행 상황을 표시한 다음 대화가 압축된 위치를 표시합니다. `/clear`를 실행하면 연결된 기기에서도 대화가 재설정됩니다.
* **`/resume`으로 대화 전환**: 연결된 기기는 전환된 대화의 제목 또는 이전 기록을 받지 않지만 양방향의 새 메시지는 터미널에서 열려 있는 대화로 이동합니다. 기기에서 원래 대화로 다시 작업하려면 터미널에서 `/resume`을 실행하고 다시 전환하세요.
* **`/teleport`로 세션 끌어오기**: [Claude Code on the web 세션](/docs/ko/claude-code-on-the-web#from-cloud-to-terminal)을 `/teleport`로 터미널로 끌어올 때 연결된 기기는 끌어온 대화의 이전 기록을 받지 않습니다. 양방향의 새 메시지는 이제 터미널에서 열려 있는 끌어온 대화로 이동합니다.
* **다른 세션의 메시지**: [교차 세션 메시징](/docs/ko/cross-session-messaging)을 사용하면 동일한 연결이 다른 컴퓨터의 자신의 세션 간 메시지와 [Claude Code on the web](/docs/ko/claude-code-on-the-web) 세션에서 Anthropic 서버를 통해 Remote Control 트래픽의 나머지와 같이 메시지를 전달합니다. [다른 컴퓨터의 메시지 세션](/docs/ko/cross-session-messaging#message-sessions-on-other-machines)은 전달 규칙을 다루고 [인바운드 메시지 제어](/docs/ko/cross-session-messaging#control-inbound-messages)는 인바운드 제어를 다룹니다. Claude Code v2.1.224 이상이 필요합니다.
* **턴 중간에 보낸 프롬프트**: 연결된 기기에서 현재 턴이 끝나기 전에 프롬프트를 보내면 Claude Code는 이를 큐에 넣고 해당 턴이 끝난 후 기기의 기록에 유지합니다.
* **변경 사항의 diff**: 세션의 디렉토리가 git 저장소에 있을 때 연결된 기기의 diff 창은 변경 사항을 표시합니다. 기기는 연결을 통해 diff를 요청하고 Claude Code는 컴퓨터에서 이를 계산합니다. 저장소의 기본 분기보다 앞에 있는 커밋이 있는 분기에서 창은 분기가 분기한 이후의 분기 변경 사항을 표시하며, 커밋되지 않은 편집을 포함합니다. 기본 분기 자체에서 또는 앞에 있지 않은 분기에서 창은 커밋되지 않은 변경 사항만 표시합니다. v2.1.247 이전에는 Claude Code가 `claude remote-control`로 제공되는 세션의 연결된 기기에만 diff를 보고했습니다.
* **모델**: 연결된 기기에서 [모델](/docs/ko/model-config)을 선택하면 Claude Code는 해당 모델에서 세션을 실행합니다. 터미널의 `/model` 선택기, `/status` 및 `/config`는 해당 모델을 표시합니다. Claude Code v2.1.238 이상이 필요합니다.
  * 기기의 모델 제어에서 선택한 모델은 현재 세션에만 적용됩니다. 기기에서 대화형 세션으로 `/model <name>`을 보내면 Claude Code는 새 세션의 기본값도 설정합니다.
  * 모델 ID가 필요한 곳에 표시 이름과 같이 Claude Code가 인식하지 못하는 이름을 보내면 Claude Code는 [선택을 거부](/docs/ko/errors#model-is-not-a-recognized-model-id)하고 세션은 현재 모델을 유지합니다. v2.1.260 이전에는 Claude Code가 기기의 모델 제어에서 인식되지 않는 선택을 저장했고 다음 메시지가 실패했습니다.
* **노력 수준**: 연결된 기기에서 `/effort` 또는 기기의 노력 제어로 [노력 수준](/docs/ko/model-config#adjust-effort-level)을 설정하면 Claude Code는 이를 컴퓨터의 세션에 적용하고 claude.ai/code는 세션이 사용 중인 수준을 표시합니다. `CLAUDE_CODE_EFFORT_LEVEL`로 수준을 고정했으면 세션은 해당 수준을 유지하고 Claude Code는 노력 제어에서 다른 선택을 거부합니다. 노력 제어에서 수준을 선택하려면 컴퓨터에 Claude Code v2.1.234 이상이 필요합니다.
* **연결 실패 후 다시 연결**: 다시 연결하려면 `/remote-control`을 실행하세요. 압축이 대화를 다시 작성했거나 그 사이에 `/resume`으로 대화를 전환했으면 Claude Code는 사용 중이던 서버 세션을 세션 목록에 남겨두는 대신 보관합니다. [보관된 세션 필터링](/docs/ko/claude-code-on-the-web#archive-sessions)으로 여전히 찾을 수 있습니다. 기기가 여전히 연결되어 있는 동안 대화를 전환해도 세션이 보관되지 않습니다.

<h3 id="enable-remote-control-for-all-sessions">
  모든 세션에 대해 Remote Control 활성화
</h3>

Remote Control은 `claude remote-control`, `claude --remote-control` 또는 `/remote-control`을 명시적으로 실행할 때만 활성화되며, 자동 연결이 켜져 있지 않으면 활성화되지 않습니다. 모든 대화형 세션에 대해 자동으로 활성화하려면 Claude Code 내에서 `/config`를 실행하고 **모든 세션에 대해 Remote Control 활성화**를 설정하세요. 토글은 세 가지 값을 가집니다:

* **`true`**: 대화형 세션이 시작될 때 자동으로 연결합니다.
* **`false`**: 자동 연결을 끕니다. 단, [관리 설정](/docs/ko/managed-settings)의 `true`가 이를 무시합니다. Claude Code는 선택을 사용자 설정에 저장하기 때문입니다. 프로젝트 또는 로컬 설정(`.claude/settings.json`, `.claude/settings.local.json`)의 `false`는 관리 `true`보다도 자동 연결을 끕니다.
* **`default`**: 선택을 지우고 설정된 경우 조직의 관리자 기본값을 따르거나, 그렇지 않으면 Claude Code의 현재 기본값을 따릅니다.

동일한 토글은 CLI 외부에도 나타납니다:

* **Desktop 앱**: **설정 > Claude Code > 기본적으로 원격 제어 활성화**.
* **VS Code 확장**: [명령 메뉴](/docs/ko/vs-code#use-the-prompt-box)의 설정 섹션에서 **모든 세션에 대해 Remote Control 활성화**. Claude Code v2.1.203 이상이 필요합니다.

설정 파일에서 자동 연결을 켜려면 사용자 `~/.claude/settings.json` 또는 [관리 설정](/docs/ko/managed-settings)에서 [`remoteControlAtStartup`](/docs/ko/settings-reference#remotecontrolatstartup)을 `true`로 설정하세요. 프로젝트 또는 로컬 설정(`.claude/settings.json`, `.claude/settings.local.json`)에서 Claude Code는 `false`를 준수하고 해당 저장소에 대해 자동 연결을 끕니다. 하지만 `true`는 무시하므로 체크인된 파일이 저장소를 여는 모든 사람에 대해 Remote Control을 켤 수 없습니다.

자동 연결은 자신의 claude.ai 계정으로 로그인하므로 시작하는 세션은 자신의 계정의 Claude 앱에만 나타나고 다른 사람에게 액세스 권한을 부여하지 않습니다.

이 설정이 켜져 있으면 각 대화형 Claude Code 프로세스는 하나의 원격 세션을 등록합니다. 여러 인스턴스를 실행하면 각각 자체 원격 세션을 가져옵니다. 단일 프로세스에서 여러 동시 세션을 실행하려면 [서버 모드](#start-a-remote-control-session)를 대신 사용하세요.

<h3 id="resume-sessions-after-stopping-the-server">
  서버를 중지한 후 세션 재개
</h3>

Ctrl+C로 `claude remote-control`을 중지하면 제공하던 세션이 휴대폰 또는 브라우저에서 응답하지 않습니다. 동일한 디렉토리에서 다른 `claude remote-control`을 실행하지 않았고 `--no-create-session-in-dir`으로 이를 시작하지 않았으면 Claude Code는 이들을 보관하지 않습니다. 이들을 다시 가져오려면 동일한 디렉토리에서 다음 명령 중 하나를 실행하세요:

* **`claude remote-control`**: 서버가 제공하던 모든 세션을 다시 가져옵니다.
* **`claude remote-control --continue`**: 서버가 시작한 세션만 다시 가져오고 해당 세션이 끝나면 종료합니다. 이 디렉토리에 기록이 없으면 Claude Code는 이 저장소의 다른 git worktree에서 가장 최신 것을 사용합니다.
* **`claude remote-control --session-id <id>`**: 전달한 ID의 세션만 다시 가져오고 해당 세션이 끝나면 종료합니다. ID는 claude.ai/code의 세션 URL에서 `/code/`와 모든 `?` 사이의 부분입니다.

이 명령들은 서버가 중지된 후 약 4시간 동안 작동합니다. 그 후에는 `claude remote-control`을 실행하여 새 세션을 시작하세요. 그 사이에 세션을 보관했으면 `--continue` 및 `--session-id`는 Claude Code v2.1.228 이상에서 이를 보관 해제합니다.

`claude --remote-control` 또는 `/remote-control`로 시작한 세션을 다시 가져오려면 `claude --continue` 또는 `claude --resume`으로 대화를 재개하세요. Claude Code가 다시 연결하는지 여부와 어느 세션으로 연결하는지는 대화의 [재개 결과](#resume-outcomes)에 따라 달라집니다.

첫 번째 터미널이 여전히 Remote Control이 켜져 있는 동안 두 번째 터미널에서 대화를 재개하면 Claude Code는 두 번째 터미널에 알림을 인쇄하고 세션을 첫 번째에서 가져가는 대신 Remote Control을 끕니다. Remote Control이 거기서 꺼져 있는 동안 해당 터미널의 Claude는 [다른 컴퓨터의 세션](/docs/ko/cross-session-messaging#see-which-sessions-claude-can-reach)을 보지 못하고 이들은 이에 도달할 수 없습니다. 두 번째 터미널에서 `/remote-control`을 실행하여 Remote Control을 이동하세요.

Remote Control이 켜져 있던 Claude Desktop 또는 IDE 확장에서 대화를 재개하면 Claude Code는 세션 목록에 새로운 것을 추가하는 대신 기존 claude.ai 세션에 다시 연결합니다.

<h2 id="connection-and-security">
  연결 및 보안
</h2>

로컬 Claude Code 세션은 아웃바운드 HTTPS 요청만 수행하며 컴퓨터에서 인바운드 포트를 열지 않습니다. Remote Control을 시작하면 Anthropic API에 등록되고 작업을 폴링합니다. 다른 기기에서 연결하면 서버는 웹 또는 모바일 클라이언트와 로컬 세션 간의 메시지를 스트리밍 연결을 통해 라우팅합니다.

모든 트래픽은 TLS를 통해 Anthropic API를 통해 이동하며, 이는 모든 Claude Code 세션과 동일한 전송 보안입니다. 연결은 각각 단일 목적으로 범위가 지정되고 독립적으로 만료되는 여러 단기 자격 증명을 사용합니다. `claude remote-control` 서버의 등록 자격 증명이 만료되면 서버는 Anthropic API에 다시 등록하고 세션을 계속 제공합니다.

Remote Control이 연결되어 있는 동안 메시지, Claude의 응답 및 도구 활동을 포함한 세션 기록이 Anthropic 서버에 저장됩니다. 저장된 기록은 기기 간에 대화를 동기화 상태로 유지하고 네트워크 중단 후 세션을 다시 연결할 수 있게 합니다. 실행 및 파일 시스템 액세스는 컴퓨터에 유지되며, 저장된 기록은 [데이터 사용](/docs/ko/data-usage) 정책에 따라 보관됩니다.

Remote Control을 완전히 끄려면 [`disableRemoteControl`](/docs/ko/settings-reference#disableremotecontrol) 설정을 사용합니다. Zero Data Retention과 같은 규정 준수 요구 사항이 있는 조직은 Remote Control을 활성화할 수 없습니다.

<h2 id="trusted-devices">
  신뢰할 수 있는 기기
</h2>

<Note>
  신뢰할 수 있는 기기는 현재 베타 단계입니다. 경험이 개선됨에 따라 기능이 변할 수 있습니다.

  신뢰할 수 있는 기기는 Pro, Max, Team 및 Enterprise 요금제에서 사용할 수 있으며 기본적으로 꺼져 있습니다. Team 및 Enterprise 요금제에서는 소유자가 조직에 대해 이를 켭니다. Pro 및 Max 요금제에서는 설정의 Cowork 또는 Account 페이지에서 **신뢰할 수 있는 기기 필요**를 직접 켭니다.
</Note>

신뢰할 수 있는 기기는 조직의 각 구성원 또는 Pro 또는 Max 요금제의 경우 사용자 혼자서 claude.ai, Claude 모바일 앱 또는 Claude Desktop에서 Remote Control 세션을 보거나 제어하기 전에 기기를 확인해야 합니다. Remote Control 액세스를 서명된 계정이 아닌 알려진 기기 및 최근 인증에 연결합니다.

설정이 켜져 있으면 Remote Control 세션과 상호 작용하려면 다음 두 가지가 모두 필요합니다:

* **등록된 기기**: 구성원이 Remote Control에 사용하는 각 브라우저, 휴대폰 또는 데스크톱 앱은 자체 자격 증명을 등록합니다. 등록은 전체 로그인 직후에만 제공되므로 기기는 백그라운드에서 자동으로 신뢰 목록에 추가되지 않고 실제 인증의 일부로 참여합니다.
* **최근 로그인**: 구성원의 로그인은 18시간 이상 되지 않아야 합니다. 매일 다시 로그인하는 대신 구성원은 Face ID, Touch ID, Windows Hello 또는 passkey로 존재를 확인합니다. 이 생체 인식 단계는 세션을 즉시 새로 고칩니다.

생체 인식 확인은 passkey 로그인과 동일한 메커니즘인 운영 체제 또는 브라우저를 통해 기기에서 실행됩니다. Anthropic은 지문, 얼굴 데이터 또는 기타 생체 인식 정보를 받거나 저장하지 않습니다. 기기의 공개 키 및 표시 이름, 플랫폼, 등록 시간 등의 기본 메타데이터만 저장됩니다.

설정은 Remote Control에만 적용됩니다. 일반 Claude 채팅, 터미널의 Claude Code 및 API 사용은 영향을 받지 않습니다.

<h3 id="enable-trusted-devices-for-your-organization">
  Team 또는 Enterprise 조직에 대해 신뢰할 수 있는 기기 활성화
</h3>

소유자는 claude.ai 조직 설정에서 설정을 활성화합니다.

<Steps>
  <Step title="Capabilities 페이지로 이동">
    [**조직 설정 > Capabilities > Remote sessions**](https://claude.ai/admin-settings/capabilities)로 이동합니다. **신뢰할 수 있는 기기 필요** 토글이 해당 섹션에 나타납니다.
  </Step>

  <Step title="신뢰할 수 있는 기기 필요 켜기">
    설정은 조직의 모든 구성원과 토글을 활성화한 후 시작된 Remote Control 세션에 적용됩니다. 토글이 켜지기 전에 이미 실행 중이던 세션은 소급 적용되지 않으며 기기 요구 사항 없이 종료될 때까지 계속됩니다. 팀별 또는 프로젝트별 범위 지정은 사용할 수 없습니다.
  </Step>

  <Step title="구성원에게 예상되는 사항 알리기">
    설정이 활성화된 후 구성원이 브라우저, 휴대폰 또는 데스크톱 앱에서 새 Remote Control 세션을 처음 보거나 제어할 때 해당 기기를 등록하라는 메시지가 표시됩니다. 미리 알려주면 혼동을 피할 수 있습니다.
  </Step>
</Steps>

<h3 id="what-members-see">
  구성원이 보는 것
</h3>

등록은 기기당 일회성 단계입니다. 그 후 유일한 눈에 띄는 변화는 가끔 생체 인식 프롬프트입니다.

* **각 기기에서 처음 사용**: 구성원에게 등록하라는 메시지가 표시됩니다. 로그인이 최근이 아니면 SSO가 구성된 경우를 포함하여 일반적인 흐름을 통해 먼저 로그인한 다음 등록을 확인합니다.
* **일상적으로**: 등록된 기기와 최근 로그인이 있는 구성원은 프롬프트를 보지 않습니다. 로그인이 18시간을 초과하면 다음 Remote Control 상호 작용에서 단일 Face ID, Touch ID, Windows Hello 또는 passkey 프롬프트가 표시됩니다.
* **등록되지 않은 기기**: 기기가 등록될 때까지 Remote Control 세션을 보거나 제어할 수 없습니다. 해당 기기의 일반 Claude 채팅은 영향을 받지 않습니다.
* **플랫폼 인증자 없음**: Face ID, Touch ID 또는 Windows Hello가 없는 기계의 구성원은 하드웨어 보안 키를 사용하거나 단계를 올리는 대신 다시 로그인할 수 있습니다.
* **터미널에서**: Claude Code를 실행하는 기계는 개발자가 CLI에 로그인할 때 자동으로 자체 자격 증명을 받습니다. 터미널에는 별도의 등록 단계가 없습니다.

<h3 id="manage-enrolled-devices">
  등록된 기기 관리
</h3>

구성원은 계정 설정에서 자신의 기기를 검토하고 취소할 수 있습니다.

[claude.ai/settings/account](https://claude.ai/settings/account#trusted-devices)를 열고 **신뢰할 수 있는 기기** 섹션을 찾아 이름, 플랫폼 및 등록 날짜가 있는 모든 등록된 기기를 확인하세요. 기기를 제거하면 자격 증명이 즉시 취소되며, 기기는 새로운 로그인 후 나중에 다시 등록할 수 있습니다. 자격 증명은 갱신되지 않으면 자동으로 만료되므로 사용하지 않는 기기는 신뢰 목록에서 자동으로 제거됩니다.

분실하거나 도난당한 기기의 경우 구성원이 이 페이지에서 제거합니다. 구성원이 로그인할 수 없으면 관리자는 관리자 콘솔에서 **모든 곳에서 로그아웃**을 사용하여 해당 구성원의 모든 세션 및 등록된 기기를 취소한 후 구성원이 여전히 보유한 기기를 다시 등록합니다.

<h2 id="remote-control-vs-cloud-sessions">
  Remote Control과 클라우드 세션 비교
</h2>

Remote Control과 [클라우드 세션](/docs/ko/claude-code-on-the-web)은 모두 claude.ai/code 인터페이스를 사용합니다. 주요 차이점은 세션이 실행되는 위치입니다: Remote Control은 사용자의 컴퓨터에서 실행되므로 로컬 MCP 서버, 도구 및 프로젝트 구성이 사용 가능하게 유지됩니다. 클라우드 세션은 클라우드 인프라에서 실행되며, 기본적으로 Anthropic에서 관리합니다.

로컬 작업 중간에 있고 다른 기기에서 계속하려고 할 때 Remote Control을 사용하세요. 로컬 설정 없이 작업을 시작하거나, 복제하지 않은 저장소에서 작업하거나, 여러 작업을 병렬로 실행하려고 할 때 클라우드 세션을 사용하세요.

<h2 id="mobile-push-notifications">
  모바일 푸시 알림
</h2>

Remote Control이 활성화되면 Claude는 휴대폰으로 푸시 알림을 보낼 수 있습니다.

Claude는 언제 푸시할지 결정합니다. 일반적으로 오래 실행되는 작업이 완료되거나 계속하기 위해 사용자의 결정이 필요할 때 하나를 보냅니다. 프롬프트에서 푸시를 요청할 수도 있습니다. 예를 들어 `테스트가 완료되면 알려주세요`. 아래의 켜기/끄기 토글 외에는 이벤트별 구성이 없습니다.

모바일 푸시 알림을 설정하려면:

<Steps>
  <Step title="Claude 모바일 앱 설치">
    [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) 또는 [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude)용 Claude 앱을 다운로드하세요.
  </Step>

  <Step title="Claude Code 계정으로 로그인">
    터미널에서 Claude Code에 사용하는 동일한 계정 및 조직을 사용하세요.
  </Step>

  <Step title="알림 허용">
    운영 체제의 알림 권한 프롬프트를 수락하세요.
  </Step>

  <Step title="Claude Code에서 푸시 활성화">
    터미널에서 `/config`를 실행하고 사전 알림을 위해 **Claude가 결정할 때 푸시**를 활성화하거나, 권한 프롬프트 및 질문을 위해 **작업이 필요할 때 푸시**를 활성화하거나, 둘 다 활성화하세요.
  </Step>
</Steps>

알림이 도착하지 않으면:

* `/config`에 **등록된 모바일 없음**이 표시되면 휴대폰에서 Claude 앱을 열어 푸시 토큰을 새로 고칠 수 있습니다. Remote Control이 다음에 연결할 때 경고가 지워집니다.
* iOS에서 포커스 모드 및 알림 요약이 푸시를 억제하거나 지연시킬 수 있습니다. 설정 → 알림 → Claude를 확인하세요.
* Android에서 적극적인 배터리 최적화가 전달을 지연시킬 수 있습니다. 시스템 설정에서 Claude 앱을 배터리 최적화에서 제외하세요.

Claude Code는 터미널에 입력하거나 연결된 터미널에 집중하는 동안 모바일 푸시 알림을 건너뜁니다. v2.1.181부터 [`CLAUDE_CLIENT_PRESENCE_FILE`](/docs/ko/env-vars)을 마커 파일 경로로 설정하여 다른 창에 있더라도 기계에 있는 모든 시간으로 확장할 수 있습니다. 파일이 존재하는 동안 알림이 건너뛰어집니다. 화면 잠금 해제 시 파일을 생성하고 화면이 잠길 때 파일을 삭제하도록 화면 잠금 리스너 또는 유사한 도구를 구성하세요.

<h2 id="limitations">
  제한 사항
</h2>

* **대화형 프로세스당 하나의 원격 세션**: 서버 모드 외부에서 각 Claude Code 인스턴스는 한 번에 하나의 원격 세션을 지원합니다. 단일 프로세스에서 여러 동시 세션을 실행하려면 [서버 모드](#start-a-remote-control-session)를 사용하세요.
* **로컬 프로세스는 계속 실행되어야 함**: Remote Control은 로컬 프로세스로 실행됩니다. 터미널을 닫거나, VS Code를 종료하거나, 다른 방식으로 `claude` 프로세스를 중지하면 세션이 오프라인 상태가 됩니다([다시 시작](#resume-sessions-after-stopping-the-server)할 때까지). Claude가 작업 중이 아닌 경우, claude.ai 및 Claude 앱은 프로세스가 종료된 후 몇 초 내에 세션을 오프라인으로 표시합니다. SSH에서 연결을 해제한 후 원격 머신에서 세션을 계속 실행하려면 `tmux` 또는 `screen` 내에서 시작하세요.
* **서버 모드에서 충돌한 세션**: `claude remote-control`로 제공되는 세션이 충돌하면 연결된 디바이스에서 메시지를 보내세요. Claude Code가 다시 제공합니다. 서버를 다시 시작할 필요가 없습니다. Claude Code v2.1.238 이상이 필요합니다.
* **연결된 세션에서 HTTP 403 거부**: 대화형 세션이 연결되면, VPN 또는 네트워크 변경 후 발생할 수 있는 것처럼 컴퓨터와 Anthropic 서버 사이의 무언가가 HTTP 403으로 응답할 때 Claude Code는 최대 3분 동안 재시도합니다. 거부가 더 오래 지속되면 Claude Code는 연결을 해제하고 거부한 대상을 표시합니다: 네트워크 엣지 또는 자신의 네트워크의 프록시, VPN 또는 방화벽.
* **장시간 네트워크 중단**: 컴퓨터가 켜져 있지만 네트워크에 도달할 수 없으면 다음 단계는 모드에 따라 달라집니다:
  * **서버 모드**: Claude Code는 약 10분 후에 포기하고 `claude remote-control` 프로세스가 종료됩니다. `claude remote-control`을 다시 실행하여 새 세션을 시작하세요.
  * **대화형 세션**: 로컬에서 계속 작업하세요. Claude Code는 중단이 지속되는 동안 재시도하고 네트워크가 복구되면 자동으로 다시 연결됩니다.
* **현재 상태 하트비트 실패**: 대화형 세션이 `could not reach the Remote Control server for about 30 minutes`로 연결 해제되면 `/remote-control`을 실행하여 다시 연결하세요. Claude Code는 세션의 현재 상태 하트비트가 실패했지만 나머지 연결은 유지되었을 때만 이 메시지를 표시합니다. 약 30분 동안 세션을 다시 등록한 후 연결을 해제합니다.
* **전달된 대화 상자 만료**: Claude Code는 권한 프롬프트와 `AskUserQuestion` 질문을 답변할 때까지 열어 둡니다. Claude Code가 다른 종류의 대화 상자를 원격 세션으로 전달할 때(예: 안전 거부 후 표시되는 모델 선택 프롬프트), 기본적으로 5분을 기다린 후 대화 상자를 닫고 대화 상자의 작업 없음 기본값으로 계속합니다. [`dialogExpiry`](/docs/ko/settings-reference#dialogexpiry)를 설정하여 기한을 조정하거나 비활성화하세요. Claude Code v2.1.224 이상이 필요합니다.
* **Fable 사용 크레딧 동의 프롬프트는 전달되지 않음**: Claude Code는 중간 세션 [Fable 사용 크레딧 동의 프롬프트](/docs/ko/model-config#fable-and-usage-credits)를 세션이 실행되는 위치에만 표시하고 사용자의 디바이스에는 표시하지 않습니다. 세션이 터미널에서 실행되고 Claude Code가 프롬프트를 닫기 전에 아무도 답변하지 않으면 턴이 요청을 보내지 않고 종료됩니다. [프롬프트 확인이 답변되지 않음](/docs/ko/errors#the-prompt-to-confirm-went-unanswered)을 참조하세요.
* **일부 명령은 로컬 전용**: `/plugin` 또는 `/resume`과 같이 터미널 인터페이스에서만 실행되는 명령은 인수를 전달하는지 여부와 관계없이 로컬 CLI에서만 작동합니다. 다음은 모바일 및 웹에서 작동합니다:
  * 텍스트 출력 명령: `/compact`, `/clear`, `/context`, `/usage`, `/exit`, `/usage-credits`, `/recap`, `/reload-plugins`. `/usage-credits`는 브라우저를 열지 않고 청구 URL을 인쇄합니다. `/reload-plugins`는 세션이 대화형 터미널에서 실행될 때만 작동합니다. 터미널이 없는 세션은 거부합니다.
  * `/model`, `/effort`, `/fast`, `/color`, `/rename`: 값을 인수로 전달합니다. 예를 들어 `/model sonnet` 또는 `/effort high`입니다. 모바일 및 웹에서 `/model`과 `/effort`는 터미널 선택기 또는 슬라이더 대신 인수를 사용합니다.
  * `/mcp`: 모바일 앱에서는 선택기를 열지 않고 서버 상태의 텍스트 요약을 반환합니다. 웹에서는 `/mcp`만으로 요약을 반환하는 대신 [claude.ai 커넥터](/docs/ko/mcp#use-mcp-servers-from-claude-ai)의 디렉토리를 엽니다. `reconnect`, `enable`, `disable` [하위 명령](/docs/ko/commands#all-commands)은 둘 다에서 작동합니다. 로컬 CLI와 달리, 서버 이름 없이 `/mcp reconnect`를 실행하면 실패했거나 인증이 필요한 모든 서버를 다시 연결합니다.
  * `/config`: 모바일 앱에서는 `key=value`를 전달하여 설정을 지정하거나, 인수 없이 실행하여 설정할 수 있는 키를 나열합니다. 웹에서는 `/config`가 설정의 Claude Code 섹션을 열고 명령 뒤의 텍스트는 무시합니다.
  * Team 및 Enterprise에서 모바일 또는 웹의 `/usage-credits`는 [관리자에게 사용 크레딧 요청을 보내지](/docs/ko/costs#add-usage-credits-to-your-subscription) 않습니다. 전송하려면 대화형 CLI에만 나타나는 확인이 필요하므로 명령은 대신 거기서 실행하도록 지시합니다. v2.1.211 이전에는 텍스트 형식이 확인 없이 요청을 보냈습니다.
  * `/autocompact`, v2.1.221부터: 창 크기를 인수로 전달합니다. 예를 들어 `/autocompact 500k`입니다. 인수 없이 실행하면 터미널 세션에서 명령이 표시하는 대화 상자를 여는 대신 현재 창 크기를 텍스트로 인쇄합니다.
  * `/advisor`, v2.1.260부터: 모델을 인수로 전달합니다. 예를 들어 `/advisor opus`이거나 advisor를 끄려면 `off`를 전달합니다. 두 형식 모두 현재 세션에만 적용되며 저장된 기본값은 변경하지 않습니다. 인수 없이 실행하면 선택기를 여는 대신 현재 advisor를 텍스트로 인쇄합니다.
  * `/output-style`, v2.1.269부터: 스타일 이름을 인수로 전달합니다. 예를 들어 `/output-style concise`이거나, 인수 없이 실행하여 스타일을 나열합니다. 모바일 및 웹에서는 [기본 제공 스타일](/docs/ko/output-styles#built-in-output-styles)만 나열하고 선택할 수 있습니다. [사용자 정의 스타일](/docs/ko/output-styles#create-a-custom-output-style)을 사용하려면 세션 자체에서 선택하세요.

<h2 id="troubleshooting">
  문제 해결
</h2>

<h3 id="remote-control-requires-a-claude-ai-subscription">
  "Remote Control에는 claude.ai 구독이 필요합니다"
</h3>

claude.ai 계정으로 인증되지 않았거나 다른 자격 증명이 로그인보다 우선합니다. 메시지는 다음 중 하나의 형태를 취합니다:

* 로그아웃 상태, `/remote-control` 또는 `--remote-control`에서: `Remote Control requires a claude.ai subscription.` 또는 `/remote-control requires a claude.ai subscription.`
* 로그아웃 상태, `claude remote-control`에서: `You must be logged in to use Remote Control. Remote Control is only available with claude.ai subscriptions.`
* 로그인 상태이지만 API 키 또는 토큰이 사용 중: `Remote Control requires claude.ai subscription auth.` 다음에 사용 중인 자격 증명(예: `ANTHROPIC_API_KEY is set, so this session is using API-key auth`)이 표시됩니다. `apiKeyHelper` 설정과 `ANTHROPIC_AUTH_TOKEN`도 동일한 방식으로 명명됩니다.

`claude auth login`을 실행하고 claude.ai 옵션을 선택하세요. 메시지에 `ANTHROPIC_API_KEY` 또는 `ANTHROPIC_AUTH_TOKEN`이 명시되어 있으면 설정된 위치(셸 환경 또는 [설정 파일](/docs/ko/settings-reference#env)의 `env` 블록)에서 제거하세요. `apiKeyHelper`가 명시되어 있으면 해당 설정을 제거하세요.

v2.1.206 이전에는 로그아웃 상태에서 `/remote-control`을 실행하면 이 메시지 대신 `Unknown command: /remote-control`을 보고했습니다.

<h3 id="remote-control-requires-a-full-scope-login-token">
  "Remote Control에는 전체 범위 로그인 토큰이 필요합니다"
</h3>

`claude setup-token` 또는 `CLAUDE_CODE_OAUTH_TOKEN` 환경 변수의 장기 토큰으로 인증되었습니다. 이러한 토큰은 모델 요청만 수행할 수 있으므로 Remote Control 세션을 설정할 수 없습니다. 대신 `claude auth login`을 실행하여 전체 범위 세션 토큰으로 인증하세요.

<h3 id="unable-to-determine-your-organization-for-remote-control-eligibility">
  "Remote Control 적격성을 위해 조직을 결정할 수 없습니다"
</h3>

캐시된 계정 정보가 오래되었거나 불완전합니다. `claude auth login`을 실행하여 새로 고치세요.

<h3 id="remote-control-isn’t-enabled-for-this-account">
  "Remote Control이 이 계정에 대해 활성화되지 않았습니다"
</h3>

Claude Code가 로그인한 계정에 대한 Remote Control 가용성을 확인했으며 확인 결과가 꺼져 있습니다. 일반적인 원인은 요금제 변경 후 최신이 아닌 캐시된 자격입니다. `claude auth logout`을 실행한 다음 `claude auth login`을 실행하여 새로 고치고, 이전 버전을 사용 중인 경우 Claude Code를 업데이트하세요.

`claude doctor`를 실행하여 어떤 개별 적격성 확인이 실패했는지 확인하세요. 환경 변수 충돌, 도달할 수 없는 확인, 조직의 Remote Control 설정은 각각 자신의 메시지를 생성하므로 이 오류는 계정 수준 확인 자체를 의미합니다.

v2.1.239 이전에는 이 메시지가 "Remote Control is not yet enabled for your account"로 표시되었습니다. v2.1.154 이전에는 `DISABLE_TELEMETRY` 또는 `DO_NOT_TRACK`과 같이 기능 플래그 평가를 비활성화하는 변수도 이 메시지를 생성했습니다. 아래의 "Remote Control requires feature-flag evaluation" 항목이 해당 구성을 다룹니다.

<h3 id="couldn’t-verify-remote-control-eligibility">
  "Remote Control 적격성을 확인할 수 없습니다"
</h3>

Claude Code가 Remote Control이 계정에 대해 활성화되어 있는지 확인하기 위해 기능 플래그 서비스에 도달할 수 없습니다. 일반적으로 오프라인 상태이거나 프록시가 요청을 차단하고 있기 때문입니다. 네트워크 액세스가 있으면 다시 시도하거나 `claude doctor`를 실행하여 세부 정보를 확인하세요. 관련 메시지인 "조직의 Remote Control 정책을 확인할 수 없습니다"는 동일한 원인과 동일한 해결책을 가집니다. 두 메시지 모두 v2.1.178에서 추가되었습니다.

<h3 id="remote-control-requires-feature-flag-evaluation">
  "Remote Control은 기능 플래그 평가가 필요합니다"
</h3>

다음 변수 중 하나가 설정되어 있습니다: [`DISABLE_TELEMETRY`, `DO_NOT_TRACK`, `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`, 또는 `DISABLE_GROWTHBOOK`](/docs/ko/env-vars). 이들 각각은 Remote Control 가용성이 의존하는 기능 플래그 평가를 비활성화하며, 전체 메시지는 Claude Code가 찾은 변수의 이름을 지정합니다. 설정된 위치(셸 환경 또는 [`settings.json` 파일](/docs/ko/settings-reference#all-settings)의 `env` 블록)에서 해당 변수를 설정 해제하세요. 2.1.154 이전 버전에서는 동일한 구성이 대신 "Remote Control is not yet enabled for your account"를 생성합니다.

<h3 id="remote-control-is-only-available-when-using-claude-via-api-anthropic-com">
  "Remote Control은 api.anthropic.com을 통해 Claude를 사용할 때만 사용 가능합니다"
</h3>

세션이 Anthropic API와 직접 통신하지 않으므로 페어링할 claude.ai 백엔드가 없습니다. 이는 Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry에서 발생합니다. 또한 [`ANTHROPIC_BASE_URL`](/docs/ko/env-vars)이 `api.anthropic.com` 이외의 호스트(예: [LLM 게이트웨이](/docs/ko/llm-gateway) 또는 프록시)를 가리킬 때도 발생하며, claude.ai로 로그인한 경우에도 마찬가지입니다. v2.1.196 이전에는 Claude Code가 사용자 정의 `ANTHROPIC_BASE_URL`에 대해 이 메시지를 표시하지 않았습니다. 전체 원인 목록은 [오류 참조](/docs/ko/errors#remote-control-requires-the-anthropic-api)를 참조하세요.

메시지는 세션을 Anthropic API에서 멀어지게 한 것(예: `CLAUDE_CODE_USE_BEDROCK` 또는 사용자 정의 `ANTHROPIC_BASE_URL`)을 명시합니다. 적격 claude.ai 로그인이 있으면 명시된 변수를 설정 해제하고, [설정](/docs/ko/settings)의 `env` 키에서 제거한 경우 제거하고, 세션을 다시 시작하세요. v2.1.219 이전에는 메시지가 이 섹션의 헤더에 있는 문장만 포함했으므로 이전 버전에서는 `CLAUDE_CODE_USE_BEDROCK` 및 `CLAUDE_CODE_USE_VERTEX`와 같은 공급자 변수와 `ANTHROPIC_BASE_URL`에 대해 환경을 직접 확인하세요.

<h3 id="remote-control-is-disabled-by-your-organization’s-policy">
  "Remote Control은 조직의 정책에 의해 비활성화되었습니다"
</h3>

정책이 Remote Control을 차단하거나 Claude Code가 이 기계에서 조직의 정책을 로드할 수 없어 그 동안 Remote Control을 꺼둡니다. 다음 원인을 순서대로 확인하세요:

* **오류에 `disableRemoteControl`이 언급됨**: IT 관리자가 조직 전체 토글 및 로그인 방식과 무관하게 [관리 설정](/docs/ko/managed-settings)을 통해 이 장치에서 Remote Control을 비활성화했습니다.
* **claude.ai 요금제가 Pro 또는 Max**: Claude Code가 여전히 이전 로그인의 Team 또는 Enterprise 조직으로 로그인되어 있으므로 해당 조직의 Remote Control 정책을 확인합니다. `/status`를 실행하여 로그인이 사용하는 요금제와 조직을 확인하세요. `claude auth logout`을 실행한 다음 `claude auth login`을 실행하여 현재 요금제로 다시 로그인하세요.
* **조직 정책이 이 기계에 로드되지 않음**: `claude doctor`를 실행하고 `Organization policy` 줄을 읽으세요. 줄에 정책이 로드되지 않음을 표시하면 Remote Control을 꺼두는 것입니다. v2.1.261 이전에는 `claude doctor`가 이 줄을 인쇄하지 않았습니다.
* **메시지에 조직 관리자에게 문의하라고 표시되지 않음**: 조직에 Remote Control과 호환되지 않는 HIPAA 구성이 있으며, `/status`는 `Compliance` 행에 `HIPAA`를 나열합니다. 이 상태에서 관리 패널의 Remote Control 토글은 회색으로 표시되므로 Owner가 변경할 수 없습니다. Anthropic 지원팀에 문의하여 옵션을 논의하세요. v2.1.267 이전에는 이 경우에 대신 "Remote Control isn't available for your organization due to its compliance policy"가 표시되었습니다.
* **그 외의 경우, Owner가 조직에 대해 활성화하지 않음**: Remote Control은 Team 및 Enterprise 요금제에서 기본적으로 꺼져 있습니다. Owner는 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)에서 **Remote Control** 토글을 켜서 활성화할 수 있습니다. 이 토글은 서버 측 조직 설정입니다.

<h3 id="remote-credentials-fetch-failed">
  "원격 자격 증명 가져오기 실패"
</h3>

Claude Code가 Anthropic API에서 연결을 설정하기 위한 단기 자격 증명을 얻을 수 없습니다. `--verbose`로 다시 실행하여 전체 오류를 확인하세요:

```bash theme={null}
claude remote-control --verbose
```

일반적인 원인:

* 로그인하지 않음: `claude`를 실행하고 `/login`을 사용하여 claude.ai 계정으로 인증하세요. API 키 인증은 Remote Control에서 지원되지 않습니다.
* 네트워크 또는 프록시 문제: 방화벽 또는 프록시가 아웃바운드 HTTPS 요청을 차단할 수 있습니다. Remote Control은 포트 443의 Anthropic API에 대한 액세스가 필요합니다.
* 세션 생성 실패: `Session creation failed — see debug log`도 표시되면 설정 초기에 실패가 발생했습니다. 구독이 활성 상태인지 확인하세요.

오래된 로그인 토큰은 이 오류를 발생시키지 않습니다. Anthropic API가 저장된 토큰을 거부할 때(예: 다른 Claude Code 프로세스가 이미 새로 고쳤기 때문에) Claude Code는 토큰을 새로 고치고 자동으로 다시 시도합니다. v2.1.224 이전에는 오래된 토큰이 이 메시지로 Remote Control 시작을 실패하게 했으므로 [자동으로 연결하도록 설정된](#enable-remote-control-for-all-sessions) 세션이 시작 시 간헐적으로 실패할 수 있었습니다.

<h3 id="couldn’t-reconnect-to-your-remote-control-session">
  "Remote Control 세션에 다시 연결할 수 없습니다"
</h3>

`claude --resume` 또는 `claude --continue`로 대화를 재개할 때 Claude Code는 해당 대화에 기록된 Remote Control 세션에 다시 연결합니다. 이 메시지는 네트워크 중단 또는 서버 오류와 같이 일시적일 수 있는 이유로 재연결이 실패했음을 의미하므로 Claude Code는 원격 세션이 여전히 존재하는지 확인할 수 없습니다.

`/remote-control`을 실행하여 연결을 다시 시도하거나 `claude --remote-control`로 새 Remote Control 세션을 생성하세요. 그 동안 로컬 세션은 Remote Control 없이 계속 실행됩니다.

<span id="resume-outcomes" />재개할 때 이 메시지 대신 다음 결과 중 하나를 얻을 수도 있습니다:

* **서버가 기록된 세션이 없다고 보고하거나 재연결 기록이 다른 계정을 명시함**: Claude Code는 대화의 재연결 기록이 말하는 것을 따릅니다:
  * **기록이 로그인한 계정을 명시함**: Claude Code는 자동 생성된 이름으로 대체 세션을 시작하고 대화의 이전 메시지를 제외합니다. 예를 들어 claude.ai 또는 Claude 앱에서 세션을 삭제한 후 이를 얻습니다.
  * **기록이 다른 계정을 명시함**: Claude Code는 대화의 이전 메시지 없이 메시지를 표시하지 않고 새 세션을 시작합니다. 기록된 세션이 여전히 존재하는지 여부와 관계없이 마찬가지입니다.
  * **기록이 세션을 소유한 계정을 명시하지 않거나 Claude Code가 저장된 로그인을 읽을 수 없음**: Claude Code는 이 메시지 대신 [`Previous session is unavailable — run /remote-control to start a new one`](#previous-session-is-unavailable)을 표시하고 아무것도 시작하지 않으며 대화에서 기록을 제거합니다.
* **재개 전에 Remote Control을 꺼짐**: Claude Code를 호스팅하는 앱이 앱이 claude.ai 세션을 소유한다고 알려주지 않은 한, Claude Code는 CLI의 [상태 패널](#check-connection-status), VS Code 확장 또는 [Agent SDK](/docs/ko/agent-sdk/overview)를 기반으로 구축된 호스트에서 Remote Control을 꺼질 때 재연결 기록을 제거했으므로 다시 연결하지 않습니다. 소유 앱이 꺼질 때 Claude Code는 기록을 유지하고 다시 연결합니다.
* **이 기계의 다른 Claude Code가 여전히 세션을 가지고 있음**: `Remote Control not started here`로 시작하는 알림이 표시되고 Claude Code는 [재개된 세션에서 Remote Control을 꺼둡니다](#resume-sessions-after-stopping-the-server). 거기서 `/remote-control`을 실행하여 이동하세요.

<span id="reconnect-history" />v2.1.232 이전에는 Claude Code가 서버가 기록된 세션이 없다고 보고할 때 다르게 응답했습니다. v2.1.227부터 v2.1.231까지 Claude Code는 기록이 계정과 일치할 때도 대체를 시작하기를 거부했습니다. v2.1.226까지 Claude Code는 기록이 계정과 일치하는지 여부와 관계없이 대체를 시작했으며, v2.1.224부터 v2.1.226까지 해당 기계에 로그인한 계정으로 생성했으며 다른 계정의 계정으로 생성하지 않았으며 대화의 이전 메시지를 업로드하지 않았습니다. v2.1.200 이전에는 Claude Code가 재연결 실패 후 새 세션을 생성했습니다.

<h3 id="previous-session-is-unavailable">
  "이전 세션을 사용할 수 없습니다 — 새 세션을 시작하려면 /remote-control을 실행하세요"
</h3>

Claude Code가 이전 Remote Control 세션을 복구할 수 없어 자동으로 새 세션을 시작하는 대신 중지했습니다. `claude --resume` 또는 `claude --continue`로 대화를 재개한 후 또는 Claude Code가 [연결 해제 후 자동으로 다시 연결](/docs/ko/errors#remote-control-couldnt-refresh-your-login)한 후 이 메시지를 볼 수 있습니다.

`/remote-control`을 실행하여 현재 로그인으로 새 Remote Control 세션을 시작하세요. 로컬 세션은 그 동안 Remote Control 없이 계속 실행됩니다. 관련 메시지인 `Remote Control could not verify the signed-in account — run /remote-control to reconnect`는 동일한 해결책을 가집니다. Claude Code는 로그인한 계정이 변경되었거나 검증과 재연결 사이에 읽을 수 없을 때 이를 표시합니다. `Previous session is unavailable` 후에 먼저 Claude Code를 다시 시작하지 않고 `/remote-control`을 실행하면 Claude Code는 대화의 이전 메시지를 새 세션에서 제외합니다.

재개 시 Claude Code는 대화의 재연결 기록이 세션을 소유한 계정을 명시할 때만 [그 자리에 새 세션을 시작합니다](#resume-outcomes). 서버가 삭제한 세션과 다른 계정이 소유한 세션을 동일한 방식으로 보고하기 때문입니다. v2.1.227 이전의 Claude Code는 해당 계정을 기록하지 않았으며 Claude Code는 저장된 로그인을 읽을 수 없을 때 기록을 확인할 수 없습니다. v2.1.232 이전의 Claude Code는 [다른 경우 집합](#reconnect-history)에서 `Remote Control could not resume the previous session under the current login — run /remote-control to start fresh` 대신 표시했습니다.

<h3 id="remote-control-got-an-unexpected-server-response">
  "Remote Control이 예기치 않은 서버 응답을 받았습니다"
</h3>

Remote Control 서버가 요청을 수락했지만 원격 세션을 생성하거나 자격 증명을 가져올 때 이 버전의 Claude Code가 읽을 수 없는 형태로 응답했습니다. 동일한 버전에서 다시 시도하면 동일한 방식으로 실패합니다. `claude update`를 실행한 다음 `/remote-control`을 실행하여 다시 연결하세요. 이 메시지는 v2.1.225에서 추가되었습니다.

<h3 id="your-organization-requires-trusted-devices-for-remote-control-but-this-device-is-not-enrolled">
  "조직에서 Remote Control에 신뢰할 수 있는 기기를 요구하지만 이 기기는 등록되지 않았습니다"
</h3>

조직에 [신뢰할 수 있는 기기](#trusted-devices)가 활성화되어 있고 이 기계가 아직 등록되지 않았습니다. Claude Code에서 `/login`을 실행하세요. 등록은 로그인의 일부로 발생하며 별도의 등록 명령이 없습니다.

<h3 id="session-expired-for-trusted-device-check">
  "신뢰할 수 있는 기기 확인을 위해 세션이 만료되었습니다"
</h3>

로그인이 18시간 이상 되었습니다. Claude Code에서 `/login`을 실행하거나, claude.ai 또는 모바일 앱에서 Face ID, Touch ID, Windows Hello 또는 passkey로 확인하세요. [신뢰할 수 있는 기기](#trusted-devices)를 참조하세요.

<h2 id="choose-the-right-approach">
  올바른 접근 방식 선택
</h2>

Claude Code는 터미널에 있지 않을 때 작업할 수 있는 여러 방법을 제공합니다. 이들은 작업을 트리거하는 것, Claude가 실행되는 위치, 그리고 설정해야 할 양이 다릅니다.

|                                                          | 트리거                                                                    | Claude 실행 위치                                                                                | 설정                                                                                                                      | 최적 용도                          |
| :------------------------------------------------------- | :--------------------------------------------------------------------- | :------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------- | :----------------------------- |
| [Dispatch](/docs/ko/desktop#sessions-from-dispatch)           | Claude 모바일 앱에서 작업 메시지 전송                                               | 사용자 머신 (Desktop)                                                                            | [모바일 앱을 Desktop과 페어링](https://support.claude.com/en/articles/13947068)                                                  | 외출 중 작업 위임, 최소 설정              |
| [Remote Control](/docs/ko/remote-control)                     | [claude.ai/code](https://claude.ai/code) 또는 Claude 모바일 앱에서 실행 중인 세션 제어 | 사용자 머신 (CLI 또는 VS Code)                                                                     | `claude remote-control` 실행                                                                                              | 다른 기기에서 진행 중인 작업 조종            |
| [Channels](/docs/ko/channels)                                 | Telegram 또는 Discord와 같은 채팅 앱이나 자체 서버에서 이벤트 푸시                          | 사용자 머신 (CLI)                                                                                | [채널 플러그인 설치](/docs/ko/channels#quickstart) 또는 [직접 구축](/docs/ko/channels-reference)                                                | CI 실패 또는 채팅 메시지와 같은 외부 이벤트에 반응 |
| [Slack](/docs/ko/slack)                                       | 팀 채널에서 `@Claude` 언급                                                    | Anthropic 클라우드                                                                              | [Claude Code on the web](/docs/ko/claude-code-on-the-web)이 활성화된 상태에서 [Slack 앱 설치](/docs/ko/slack#setting-up-claude-code-in-slack) | 팀 채팅에서 PR 및 리뷰                 |
| [Self-hosted environments](/docs/ko/self-hosted-environments) | [클라우드 세션](/docs/ko/claude-code-on-the-web)을 시작하고 조직의 환경 선택                  | 조직의 인프라                                                                                     | [러너 배포](/docs/ko/self-hosted-environments-quickstart), Team 및 Enterprise 플랜                                                  | 네트워크 내에서 실행해야 하는 클라우드 세션       |
| [Scheduled tasks](/docs/ko/scheduled-tasks)                   | 일정 설정                                                                  | [CLI](/docs/ko/scheduled-tasks), [Desktop](/docs/ko/desktop-scheduled-tasks), 또는 [클라우드](/docs/ko/routines) | 빈도 선택                                                                                                                   | 일일 검토와 같은 반복 자동화               |

<h2 id="related-resources">
  관련 리소스
</h2>

* [웹의 Claude Code](/docs/ko/claude-code-on-the-web): 머신 대신 클라우드에서 세션을 실행하며, [클라우드 환경](/docs/ko/cloud-environments)을 통해 구성합니다
* [크로스 세션 메시징](/docs/ko/cross-session-messaging): Claude가 다른 머신이나 [클라우드 세션](/docs/ko/claude-code-on-the-web)으로 메시지를 보낼 수 있습니다
* [채널](/docs/ko/channels): Telegram, Discord 또는 iMessage를 세션으로 전달하여 Claude가 자리를 비운 동안 메시지에 반응하도록 합니다
* [Dispatch](/docs/ko/desktop#sessions-from-dispatch): 휴대폰에서 작업을 메시지로 보내면 Desktop 세션을 생성하여 처리할 수 있습니다
* [인증](/docs/ko/authentication): `/login` 설정 및 claude.ai 자격 증명 관리
* [CLI 참조](/docs/ko/cli-reference): `claude remote-control`을 포함한 플래그 및 명령의 전체 목록
* [보안](/docs/ko/security): Remote Control 세션이 Claude Code 보안 모델에 어떻게 적합한지
* [데이터 사용](/docs/ko/data-usage): 로컬, Remote Control 및 클라우드 세션 중에 Anthropic API를 통해 흐르는 데이터
