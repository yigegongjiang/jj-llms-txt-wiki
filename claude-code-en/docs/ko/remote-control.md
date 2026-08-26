> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 모든 기기에서 로컬 세션 계속하기 (Remote Control)

> Remote Control을 사용하여 휴대폰, 태블릿 또는 모든 브라우저에서 로컬 Claude Code 세션을 계속할 수 있습니다. claude.ai/code 및 Claude 모바일 앱과 함께 작동합니다.

<Note>
  Remote Control은 연구 미리보기 단계이며 모든 요금제에서 사용할 수 있습니다. Team 및 Enterprise의 경우 관리자가 [Claude Code 관리자 설정](https://claude.ai/admin-settings/claude-code)에서 Remote Control 토글을 활성화할 때까지 기본적으로 꺼져 있습니다.
</Note>

Remote Control은 [claude.ai/code](https://claude.ai/code) 또는 [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) 및 [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude)용 Claude 앱을 컴퓨터에서 실행 중인 Claude Code 세션에 연결합니다. 책상에서 작업을 시작한 다음 소파의 휴대폰이나 다른 컴퓨터의 브라우저에서 계속할 수 있습니다.

컴퓨터에서 Remote Control 세션을 시작하면 Claude는 전체 시간 동안 로컬에서 실행되므로 코드 실행 및 파일 시스템 접근이 컴퓨터에 유지됩니다. Remote Control을 사용하면 다음을 수행할 수 있습니다:

* **전체 로컬 환경을 원격으로 사용**: 파일 시스템, [MCP servers](/docs/ko/mcp), 도구 및 프로젝트 구성이 모두 사용 가능하게 유지되며, `@`를 입력하면 로컬 프로젝트의 파일 경로가 자동 완성됩니다
* **두 표면에서 동시에 작업**: 대화 및 [subagents](/docs/ko/sub-agents) 및 [dynamic workflows](/docs/ko/workflows)의 진행 상황이 모든 연결된 기기에서 동기화되므로 터미널, 브라우저 및 휴대폰에서 메시지를 교대로 보낼 수 있습니다. v2.1.207 이전에는 [Desktop app](/docs/ko/desktop)에서 호스팅하는 세션이 연결된 기기에 subagent 또는 workflow 진행 상황을 보내지 않았습니다.
* **휴대폰 또는 브라우저에서 이미지 및 파일 전송**: Claude 앱 또는 claude.ai/code에서 첨부 파일을 추가하면 Claude Code가 이를 컴퓨터에 다운로드하고 캡션 유무와 관계없이 `@` 파일 참조로 Claude에 전달합니다. v2.1.202 이전에는 Claude Code가 캡션 없이 전송된 첨부 파일을 세션에 도달하기 전에 삭제할 수 있었습니다.
* **중단 극복**: 노트북이 절전 모드로 전환되거나 네트워크가 끊어지면 컴퓨터가 다시 온라인 상태가 될 때 세션이 자동으로 다시 연결됩니다. Claude Code는 연결이 재구축되는 동안 subagents 및 workflows의 상태 업데이트를 대기열에 넣고 복구되면 전달합니다. v2.1.207 이전에는 재연결 또는 자격 증명 새로 고침 중에 전송된 업데이트가 손실될 수 있으므로 연결된 기기가 완료된 작업을 계속 실행 중으로 표시했습니다.

클라우드 인프라에서 실행되는 [웹의 Claude Code](/docs/ko/claude-code-on-the-web)와 달리 Remote Control 세션은 컴퓨터에서 직접 실행되며 로컬 파일 시스템과 상호 작용합니다. 웹 및 모바일 인터페이스는 단지 해당 로컬 세션의 창일 뿐입니다.

이 페이지에서는 설정, 세션을 시작하고 연결하는 방법, Remote Control과 웹의 Claude Code를 비교하는 방법을 다룹니다.

<h2 id="requirements">
  요구 사항
</h2>

Remote Control을 사용하기 전에 환경이 다음 조건을 충족하는지 확인하세요:

* **구독**: Pro, Max, Team 및 Enterprise 요금제에서 사용 가능합니다. API 키는 지원되지 않습니다. Team 및 Enterprise의 경우 Owner가 먼저 [Claude Code 관리자 설정](https://claude.ai/admin-settings/claude-code)에서 Remote Control 토글을 활성화해야 합니다.
* **인증**: `claude`를 실행하고 아직 로그인하지 않았다면 `/login`을 사용하여 claude.ai를 통해 로그인하세요.
* **API 엔드포인트**: Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry에서는 사용할 수 없습니다. v2.1.196부터 [`ANTHROPIC_BASE_URL`](/docs/ko/env-vars)이 `api.anthropic.com` 이외의 호스트(예: [LLM gateway](/docs/ko/llm-gateway) 또는 프록시)를 가리킬 때도 Remote Control이 비활성화됩니다. Remote Control을 사용하려면 변수를 설정 해제하세요.
* **작업 공간 신뢰**: 작업 공간 신뢰 대화를 수락하려면 프로젝트 디렉토리에서 최소한 한 번 `claude`를 실행하세요.

<h2 id="start-a-remote-control-session">
  Remote Control 세션 시작
</h2>

CLI 또는 VS Code 확장에서 Remote Control 세션을 시작할 수 있습니다. CLI는 세 가지 호출 모드를 제공하며, VS Code는 `/remote-control` 명령을 사용합니다.

<Tabs>
  <Tab title="서버 모드">
    프로젝트 디렉토리로 이동하여 다음을 실행하세요:

    ```bash theme={null}
    claude remote-control
    ```

    프로세스는 터미널에서 서버 모드로 계속 실행되어 원격 연결을 기다립니다. [다른 기기에서 연결](#connect-from-another-device)하는 데 사용할 수 있는 세션 URL을 표시하며, 스페이스바를 눌러 휴대폰에서 빠르게 액세스할 수 있는 QR 코드를 표시할 수 있습니다. 원격 세션이 활성화되어 있는 동안 터미널은 연결 상태 및 도구 활동을 표시합니다.

    사용 가능한 플래그:

    | 플래그                                             | 설명                                                                                                                                                                                                                                                                                                                     |
    | ----------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
    | `--name "My Project"`                           | claude.ai/code의 세션 목록에 표시되는 사용자 정의 세션 제목을 설정합니다.                                                                                                                                                                                                                                                                       |
    | `--remote-control-session-name-prefix <prefix>` | 명시적 이름이 설정되지 않았을 때 자동 생성된 세션 이름의 접두사입니다. 기본값은 컴퓨터의 호스트 이름이며, `myhost-graceful-unicorn`과 같은 이름을 생성합니다. 동일한 효과를 위해 `CLAUDE_REMOTE_CONTROL_SESSION_NAME_PREFIX`를 설정하세요.                                                                                                                                                   |
    | `-c`, `--continue`                              | 이 디렉토리에서 시작한 가장 최근의 Remote Control 세션을 재개하여 새로운 세션을 만드는 대신 사용합니다. `--session-id`, `--spawn`, `--capacity` 또는 `--create-session-in-dir`과 함께 사용할 수 없습니다. Claude Code v2.1.200 이상이 필요하며, 이전 버전은 이 플래그를 알 수 없는 인수로 거부합니다.                                                                                                  |
    | `--session-id <id>`                             | ID로 특정 Remote Control 세션을 재개합니다. `--continue`, `--spawn`, `--capacity` 또는 `--create-session-in-dir`과 함께 사용할 수 없습니다. Claude Code v2.1.200 이상이 필요하며, 이전 버전은 이 플래그를 알 수 없는 인수로 거부합니다.                                                                                                                                     |
    | `--spawn <mode>`                                | 서버가 세션을 생성하는 방식입니다.<br />• `same-dir` (기본값): 모든 세션이 현재 작업 디렉토리를 공유하므로 동일한 파일을 편집할 때 충돌할 수 있습니다.<br />• `worktree`: 각 온디맨드 세션은 자체 [git worktree](/docs/ko/worktrees)를 가져옵니다. git 저장소가 필요합니다.<br />• `session`: 단일 세션 모드입니다. 정확히 하나의 세션을 제공하고 추가 연결을 거부합니다. 시작 시에만 설정합니다.<br />런타임에 `w`를 눌러 `same-dir`과 `worktree` 사이를 전환하세요. |
    | `--capacity <N>`                                | 최대 동시 세션 수입니다. 기본값은 32입니다. `--spawn=session`과 함께 사용할 수 없습니다.                                                                                                                                                                                                                                                           |
    | `--[no-]create-session-in-dir`                  | 서버가 시작할 때 현재 디렉토리에 하나의 세션을 미리 생성하여 즉시 입력할 수 있는 위치를 제공합니다. `worktree` 모드에서 이 세션은 현재 디렉토리에 유지되고 온디맨드 세션은 격리된 worktree를 가져옵니다. 기본적으로 켜져 있으며, `--no-create-session-in-dir`을 전달하여 아무것도 없이 시작할 수 있습니다.                                                                                                                       |
    | `--verbose`                                     | 자세한 연결 및 세션 로그를 표시합니다.                                                                                                                                                                                                                                                                                                 |
    | `--sandbox` / `--no-sandbox`                    | 파일 시스템 및 네트워크 격리를 위해 [샌드박싱](/docs/ko/sandboxing)을 활성화하거나 비활성화합니다. 기본적으로 꺼져 있습니다.                                                                                                                                                                                                                                            |
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

    `--verbose`, `--sandbox` 및 `--no-sandbox` 플래그는 이 명령에서 사용할 수 없습니다.
  </Tab>

  <Tab title="VS Code">
    [Claude Code VS Code 확장](/docs/ko/vs-code)에서 프롬프트 상자에 `/remote-control` 또는 `/rc`를 입력하거나 `/`로 명령 메뉴를 열고 선택하세요.

    ```text theme={null}
    /remote-control
    ```

    프롬프트 상자 위에 연결 상태를 표시하는 배너가 나타납니다. 연결되면 배너의 **브라우저에서 열기**를 클릭하여 세션으로 직접 이동하거나 [claude.ai/code](https://claude.ai/code)의 세션 목록에서 찾으세요. 세션 URL도 대화에 게시됩니다.

    연결을 끊으려면 배너의 닫기 아이콘을 클릭하거나 `/remote-control`을 다시 실행하세요.

    CLI와 달리 VS Code 명령은 이름 인수를 허용하지 않으며 QR 코드를 표시하지 않습니다. 세션 제목은 대화 기록 또는 첫 번째 프롬프트에서 파생됩니다.
  </Tab>
</Tabs>

<h3 id="check-connection-status">
  연결 상태 확인
</h3>

대화형 터미널 세션에서 `/rc active` 표시기는 연결이 유지되는 동안 입력 상자 아래 바닥글에 있으며, 터미널이 너무 좁으면 숨겨집니다. 표시기 텍스트는 claude.ai의 세션으로 연결되는 링크입니다. 아래쪽 화살표 키로 선택하고 Enter를 눌러 세션 URL과 [다른 기기에서 연결](#connect-from-another-device)하는 데 사용할 수 있는 QR 코드가 있는 상태 패널을 열거나, `/remote-control`을 다시 실행하여 다시 시도하세요.

연결이 실패하면 알림이 나타나고 실패 이유가 표시되며 표시기가 바닥글에서 사라집니다. `/remote-control`을 다시 실행하여 다시 시도하세요.

<h3 id="connect-from-another-device">
  다른 기기에서 연결
</h3>

Remote Control 세션이 활성화되면 다른 기기에서 연결하는 몇 가지 방법이 있습니다:

* **세션 URL 열기**: 모든 브라우저에서 URL을 열어 [claude.ai/code](https://claude.ai/code)의 세션으로 직접 이동합니다.
* **QR 코드 스캔**: 세션 URL 옆에 표시된 QR 코드를 스캔하여 Claude 앱에서 직접 열 수 있습니다. `claude remote-control`을 사용하면 스페이스바를 눌러 QR 코드 표시를 전환할 수 있습니다.
* **[claude.ai/code](https://claude.ai/code) 또는 Claude 앱 열기**: 세션 목록에서 이름으로 세션을 찾습니다. Claude 모바일 앱에서 네비게이션의 **코드**를 탭하여 세션 목록에 도달하세요. Remote Control 세션은 온라인 상태일 때 녹색 상태 점이 있는 컴퓨터 아이콘을 표시합니다.

연결하면 기기에 세션이 이미 백그라운드에서 실행 중인 모든 서브에이전트 및 워크플로우가 표시됩니다. v2.1.208 이전에는 대화형 터미널에서 호스팅되는 세션에 연결하는 기기가 이미 실행 중인 서브에이전트 및 워크플로우를 표시하지 않았으며, 그 중 하나가 시작되거나 중지될 때까지 표시되지 않았습니다.

원격 세션 제목은 다음 순서로 선택됩니다:

1. `--name`, `--remote-control` 또는 `/remote-control`에 전달한 이름
2. `/rename`으로 설정한 제목
3. 기존 대화 기록의 마지막 의미 있는 메시지
4. `myhost-graceful-unicorn`과 같은 자동 생성된 이름입니다. 여기서 `myhost`는 컴퓨터의 호스트 이름 또는 `--remote-control-session-name-prefix`로 설정한 접두사입니다.

명시적 이름을 설정하지 않았다면 메시지를 보낸 후 제목이 프롬프트를 반영하도록 업데이트됩니다. Claude Code v2.1.176부터 자동 생성된 제목은 대화의 언어 또는 구성된 [`language`](/docs/ko/settings#available-settings) 설정과 일치합니다. claude.ai 또는 Claude 앱에서 세션의 이름을 바꾸면 `claude --resume`에 표시되는 로컬 제목도 업데이트됩니다.

환경에 이미 활성 세션이 있으면 계속할지 새로 시작할지 묻는 메시지가 표시됩니다.

Claude 앱이 아직 없으면 Claude Code 내에서 `/mobile` 명령을 사용하여 [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) 또는 [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude)용 다운로드 QR 코드를 표시하세요.

<h3 id="enable-remote-control-for-all-sessions">
  모든 세션에 대해 Remote Control 활성화
</h3>

Remote Control은 `claude remote-control`, `claude --remote-control` 또는 `/remote-control`을 명시적으로 실행할 때만 활성화되며, 자동 연결이 켜져 있지 않으면 활성화되지 않습니다. 모든 대화형 세션에 대해 자동으로 활성화하려면 Claude Code 내에서 `/config`를 실행하고 **모든 세션에 대해 Remote Control 활성화**를 `true`로 설정하세요. 비활성화하려면 `false`로 설정하거나, 조직의 기본값을 따르려면 설정하지 않은 상태로 두세요. Desktop 앱에서는 **설정 → Claude Code → 기본적으로 원격 제어 활성화**에서도 전환할 수 있습니다. [VS Code 확장](/docs/ko/vs-code#use-the-prompt-box)에서는 동일한 토글이 명령 메뉴의 설정 섹션에 **모든 세션에 대해 Remote Control 활성화**로 나타나며, Claude Code v2.1.203 이상이 필요합니다.

이 설정이 켜져 있으면 각 대화형 Claude Code 프로세스는 하나의 원격 세션을 등록합니다. 여러 인스턴스를 실행하면 각각 자체 환경 및 세션을 가져옵니다. 단일 프로세스에서 여러 동시 세션을 실행하려면 [서버 모드](#start-a-remote-control-session)를 대신 사용하세요.

<h2 id="connection-and-security">
  연결 및 보안
</h2>

로컬 Claude Code 세션은 아웃바운드 HTTPS 요청만 수행하며 컴퓨터에서 인바운드 포트를 열지 않습니다. Remote Control을 시작하면 Anthropic API에 등록되고 작업을 폴링합니다. 다른 기기에서 연결하면 서버는 웹 또는 모바일 클라이언트와 로컬 세션 간의 메시지를 스트리밍 연결을 통해 라우팅합니다.

모든 트래픽은 TLS를 통해 Anthropic API를 통해 이동하며, 이는 모든 Claude Code 세션과 동일한 전송 보안입니다. 연결은 각각 단일 목적으로 범위가 지정되고 독립적으로 만료되는 여러 단기 자격 증명을 사용합니다.

Remote Control이 연결되어 있는 동안 메시지, Claude의 응답 및 도구 활동을 포함한 세션 기록이 Anthropic 서버에 저장됩니다. 저장된 기록은 기기 간에 대화를 동기화 상태로 유지하고 네트워크 중단 후 세션을 다시 연결할 수 있게 합니다. 실행 및 파일 시스템 액세스는 컴퓨터에 유지되며, 저장된 기록은 [데이터 사용](/docs/ko/data-usage) 정책에 따라 보관됩니다.

Remote Control을 완전히 끄려면 [`disableRemoteControl`](/docs/ko/settings#available-settings) 설정을 사용합니다. Zero Data Retention과 같은 규정 준수 요구 사항이 있는 조직은 Remote Control을 활성화할 수 없습니다.

<h2 id="trusted-devices">
  신뢰할 수 있는 기기
</h2>

<Note>
  신뢰할 수 있는 기기는 현재 베타 단계입니다. 경험이 개선됨에 따라 기능이 변할 수 있습니다.

  신뢰할 수 있는 기기는 Team 및 Enterprise 요금제에서 사용할 수 있습니다. 기본적으로 꺼져 있으며 관리자가 활성화해야 합니다.
</Note>

신뢰할 수 있는 기기는 조직 전체 설정으로, 구성원이 claude.ai, Claude 모바일 앱 또는 Claude Desktop에서 Remote Control 세션을 보거나 제어하기 전에 기기를 확인해야 합니다. Remote Control 액세스를 서명된 계정이 아닌 알려진 기기 및 최근 인증에 연결합니다.

설정이 켜져 있으면 Remote Control 세션과 상호 작용하려면 다음 두 가지가 모두 필요합니다:

* **등록된 기기**: 구성원이 Remote Control에 사용하는 각 브라우저, 휴대폰 또는 데스크톱 앱은 자체 자격 증명을 등록합니다. 등록은 전체 로그인 직후에만 제공되므로 기기는 백그라운드에서 자동으로 신뢰 목록에 추가되지 않고 실제 인증의 일부로 참여합니다.
* **최근 로그인**: 구성원의 로그인은 18시간 이상 되지 않아야 합니다. 매일 다시 로그인하는 대신 구성원은 Face ID, Touch ID, Windows Hello 또는 passkey로 존재를 확인합니다. 이 생체 인식 단계는 세션을 즉시 새로 고칩니다.

생체 인식 확인은 passkey 로그인과 동일한 메커니즘인 운영 체제 또는 브라우저를 통해 기기에서 실행됩니다. Anthropic은 지문, 얼굴 데이터 또는 기타 생체 인식 정보를 받거나 저장하지 않습니다. 기기의 공개 키 및 표시 이름, 플랫폼, 등록 시간 등의 기본 메타데이터만 저장됩니다.

설정은 Remote Control에만 적용됩니다. 일반 Claude 채팅, 터미널의 Claude Code 및 API 사용은 영향을 받지 않습니다.

<h3 id="enable-trusted-devices-for-your-organization">
  조직에 대해 신뢰할 수 있는 기기 활성화
</h3>

관리자는 Claude Code 관리자 콘솔에서 설정을 활성화합니다.

<Steps>
  <Step title="Claude Code 관리자 설정 열기">
    [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)로 이동합니다. **신뢰할 수 있는 기기 필요** 토글이 Remote Control 설정 아래에 나타납니다.
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

<h2 id="remote-control-vs-claude-code-on-the-web">
  Remote Control과 웹의 Claude Code 비교
</h2>

Remote Control과 [웹의 Claude Code](/docs/ko/claude-code-on-the-web)는 모두 claude.ai/code 인터페이스를 사용합니다. 주요 차이점은 세션이 실행되는 위치입니다: Remote Control은 컴퓨터에서 실행되므로 로컬 MCP servers, 도구 및 프로젝트 구성이 사용 가능하게 유지됩니다. 웹의 Claude Code는 Anthropic 관리 클라우드 인프라에서 실행됩니다.

로컬 작업 중간에 있고 다른 기기에서 계속하려고 할 때 Remote Control을 사용하세요. 로컬 설정 없이 작업을 시작하거나, 복제하지 않은 저장소에서 작업하거나, 여러 작업을 병렬로 실행하려고 할 때 웹의 Claude Code를 사용하세요.

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
* **로컬 프로세스는 계속 실행되어야 함**: Remote Control은 로컬 프로세스로 실행됩니다. 터미널을 닫거나, VS Code를 종료하거나, 다른 방식으로 `claude` 프로세스를 중지하면 세션이 종료됩니다.
* **장시간 네트워크 중단**: 컴퓨터가 켜져 있지만 약 10분 이상 네트워크에 도달할 수 없으면 세션이 시간 초과되고 프로세스가 종료됩니다. `claude remote-control`을 다시 실행하여 새 세션을 시작하세요.
* **Ultraplan이 Remote Control 연결 해제**: [ultraplan](/docs/ko/ultraplan) 세션을 시작하면 활성 Remote Control 세션이 연결 해제됩니다. 두 기능 모두 claude.ai/code 인터페이스를 차지하고 한 번에 하나만 연결될 수 있기 때문입니다.
* **일부 명령은 로컬 전용**: `/plugin` 또는 `/resume`과 같이 터미널 인터페이스에서만 실행되는 명령은 인수를 전달하는지 여부와 관계없이 로컬 CLI에서만 작동합니다. 다음은 모바일 및 웹에서 작동합니다:
  * 텍스트 출력 명령: `/compact`, `/clear`, `/context`, `/usage`, `/exit`, `/usage-credits` (CLI 내 대화 상자를 열지 않고 텍스트 형식으로 실행), `/recap`, `/reload-plugins`
  * `/model`, `/effort`, `/fast`, `/color`, `/rename`: 값을 인수로 전달합니다. 예를 들어 `/model sonnet` 또는 `/effort high`입니다. 모바일 및 웹에서 `/model`과 `/effort`는 터미널 선택기 또는 슬라이더 대신 인수를 사용합니다.
  * `/mcp`, v2.1.166부터: 모바일 앱에서는 선택기를 열지 않고 서버 상태의 텍스트 요약을 반환합니다. 웹에서는 `/mcp`만으로 요약을 반환하는 대신 [claude.ai 커넥터](/docs/ko/mcp#use-mcp-servers-from-claude-ai)의 디렉토리를 엽니다. `reconnect`, `enable`, `disable` [하위 명령](/docs/ko/commands#all-commands)은 둘 다에서 작동합니다. 로컬 CLI와 달리, 서버 이름 없이 `/mcp reconnect`를 실행하면 실패했거나 인증이 필요한 모든 서버를 다시 연결합니다.
  * `/config`, v2.1.181부터: 모바일 앱에서는 `key=value`를 전달하여 설정을 지정하거나, 인수 없이 실행하여 설정할 수 있는 키를 나열합니다. 웹에서는 `/config`가 설정의 Claude Code 섹션을 열고 명령 뒤의 텍스트는 무시합니다.

<h2 id="troubleshooting">
  문제 해결
</h2>

<h3 id="remote-control-requires-a-claude-ai-subscription">
  "Remote Control에는 claude.ai 구독이 필요합니다"
</h3>

claude.ai 계정으로 인증되지 않았습니다. `claude auth login`을 실행하고 claude.ai 옵션을 선택하세요. `ANTHROPIC_API_KEY`가 환경에 설정되어 있으면 먼저 설정을 해제하세요.

v2.1.206 이전에는 로그아웃 상태에서 `/remote-control`을 실행하면 이 메시지 대신 `Unknown command: /remote-control`을 보고했습니다.

<h3 id="remote-control-requires-a-full-scope-login-token">
  "Remote Control에는 전체 범위 로그인 토큰이 필요합니다"
</h3>

`claude setup-token` 또는 `CLAUDE_CODE_OAUTH_TOKEN` 환경 변수의 장기 토큰으로 인증되었습니다. 이러한 토큰은 추론 전용으로 제한되며 Remote Control 세션을 설정할 수 없습니다. 대신 `claude auth login`을 실행하여 전체 범위 세션 토큰으로 인증하세요.

<h3 id="unable-to-determine-your-organization-for-remote-control-eligibility">
  "Remote Control 적격성을 위해 조직을 결정할 수 없습니다"
</h3>

캐시된 계정 정보가 오래되었거나 불완전합니다. `claude auth login`을 실행하여 새로 고치세요.

<h3 id="remote-control-is-not-yet-enabled-for-your-account">
  "Remote Control이 아직 계정에 대해 활성화되지 않았습니다"
</h3>

Remote Control 롤아웃이 계정에 도달하지 않았거나 캐시된 자격이 최신이 아닙니다. 최근에 요금제를 변경한 경우 `claude auth logout`을 실행한 다음 `claude auth login`을 실행하여 새로 고치세요. `claude doctor`를 실행하여 어떤 개별 적격성 확인이 실패했는지 확인하세요. 환경 변수 충돌, 도달할 수 없는 확인, 조직 정책은 각각 자신의 메시지를 생성하므로 이 오류는 롤아웃 게이트 자체를 의미합니다.

<h3 id="couldn’t-verify-remote-control-eligibility">
  "Remote Control 적격성을 확인할 수 없습니다"
</h3>

Claude Code가 Remote Control이 계정에 대해 활성화되어 있는지 확인하기 위해 기능 플래그 서비스에 도달할 수 없습니다. 일반적으로 오프라인 상태이거나 프록시가 요청을 차단하고 있기 때문입니다. 네트워크 액세스가 있으면 다시 시도하거나 `claude doctor`를 실행하여 세부 정보를 확인하세요. 관련 메시지인 "조직의 Remote Control 정책을 확인할 수 없습니다"는 동일한 원인과 동일한 해결책을 가집니다. 두 메시지 모두 v2.1.178에서 추가되었습니다.

<h3 id="remote-control-is-only-available-when-using-claude-via-api-anthropic-com">
  "Remote Control은 api.anthropic.com을 통해 Claude를 사용할 때만 사용 가능합니다"
</h3>

세션이 Anthropic API와 직접 통신하지 않으므로 페어링할 claude.ai 백엔드가 없습니다. 이는 Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry에서 발생합니다. v2.1.196부터는 [`ANTHROPIC_BASE_URL`](/docs/ko/env-vars)이 `api.anthropic.com` 이외의 호스트(예: [LLM 게이트웨이](/docs/ko/llm-gateway) 또는 프록시)를 가리킬 때도 발생하며, claude.ai로 로그인한 경우에도 마찬가지입니다. `ANTHROPIC_BASE_URL`을 설정 해제하고 세션을 다시 시작하여 Remote Control을 사용하세요.

<h3 id="remote-control-is-disabled-by-your-organization’s-policy">
  "Remote Control은 조직의 정책에 의해 비활성화되었습니다"
</h3>

이 오류에는 네 가지 서로 다른 원인이 있습니다. 먼저 `/status`를 실행하여 사용 중인 로그인 방법과 구독을 확인하세요.

* **API 키 또는 Console 계정으로 인증됨**: Remote Control은 claude.ai OAuth가 필요합니다. `/login`을 실행하고 claude.ai 옵션을 선택하세요. `ANTHROPIC_API_KEY`가 환경에 설정되어 있으면 설정을 해제하세요.
* **Team 또는 Enterprise 관리자가 활성화하지 않음**: Remote Control은 이러한 요금제에서 기본적으로 꺼져 있습니다. 관리자는 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)에서 **Remote Control** 토글을 켜서 활성화할 수 있습니다. 이 토글은 서버 측 조직 설정입니다.
* **관리자 토글이 회색으로 표시됨**: 조직에 Remote Control과 호환되지 않는 데이터 보존 또는 규정 준수 구성이 있습니다. 이는 관리자 패널에서 변경할 수 없습니다. Anthropic 지원팀에 문의하여 옵션을 논의하세요.
* **오류에 `disableRemoteControl`이 언급됨**: IT 관리자가 조직 전체 토글과 무관하게 [관리 설정](/docs/ko/settings#settings-files)을 통해 이 장치에서 Remote Control을 비활성화했습니다.

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

<h3 id="couldn’t-reconnect-to-your-remote-control-session">
  "Remote Control 세션에 다시 연결할 수 없습니다"
</h3>

`claude --resume` 또는 `claude --continue`로 대화를 재개할 때 Claude Code는 해당 대화에 기록된 Remote Control 세션에 다시 연결합니다. 이 메시지는 네트워크 중단 또는 서버 오류와 같이 일시적일 수 있는 이유로 재연결이 실패했음을 의미하므로 Claude Code는 원격 세션이 여전히 존재하는지 확인할 수 없습니다. 서버가 이전 세션이 더 이상 존재하지 않음을 확인하면 Claude Code는 이 메시지를 표시하지 않고 새 Remote Control 세션을 생성합니다.

로컬 세션은 Remote Control 없이 계속 실행됩니다. `/remote-control`을 실행하여 연결을 다시 시도하거나 `--resume` 없이 Claude Code를 시작하여 새 Remote Control 세션을 생성하세요.

v2.1.200 이전에는 재연결 실패로 인해 이 메시지를 표시하는 대신 새 Remote Control 세션이 생성되었으며, 이로 인해 claude.ai/code의 세션 목록에 추가 세션이 남겨졌습니다.

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

Claude Code offers several ways to work when you're not at your terminal. They differ in what triggers the work, where Claude runs, and how much you need to set up.

|                                                          | Trigger                                                                                        | Claude runs on                                                                               | Setup                                                                                                                                | Best for                                                      |
| :------------------------------------------------------- | :--------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------ |
| [Dispatch](/docs/en/desktop#sessions-from-dispatch)           | Message a task from the Claude mobile app                                                      | Your machine (Desktop)                                                                       | [Pair the mobile app with Desktop](https://support.claude.com/en/articles/13947068)                                                  | Delegating work while you're away, minimal setup              |
| [Remote Control](/docs/en/remote-control)                     | Drive a running session from [claude.ai/code](https://claude.ai/code) or the Claude mobile app | Your machine (CLI or VS Code)                                                                | Run `claude remote-control`                                                                                                          | Steering in-progress work from another device                 |
| [Channels](/docs/en/channels)                                 | Push events from a chat app like Telegram or Discord, or your own server                       | Your machine (CLI)                                                                           | [Install a channel plugin](/docs/en/channels#quickstart) or [build your own](/docs/en/channels-reference)                                      | Reacting to external events like CI failures or chat messages |
| [Slack](/docs/en/slack)                                       | Mention `@Claude` in a team channel                                                            | Anthropic cloud                                                                              | [Install the Slack app](/docs/en/slack#setting-up-claude-code-in-slack) with [Claude Code on the web](/docs/en/claude-code-on-the-web) enabled | PRs and reviews from team chat                                |
| [Self-hosted environments](/docs/en/self-hosted-environments) | Start a [cloud session](/docs/en/claude-code-on-the-web) and pick your organization's environment   | Your organization's infrastructure                                                           | [Deploy runners](/docs/en/self-hosted-environments-quickstart), on Team and Enterprise plans                                              | Cloud sessions that must run inside your network              |
| [Scheduled tasks](/docs/en/scheduled-tasks)                   | Set a schedule                                                                                 | [CLI](/docs/en/scheduled-tasks), [Desktop](/docs/en/desktop-scheduled-tasks), or [cloud](/docs/en/routines) | Pick a frequency                                                                                                                     | Recurring automation like daily reviews                       |

<h2 id="related-resources">
  관련 리소스
</h2>

* [웹의 Claude Code](/docs/ko/claude-code-on-the-web): 컴퓨터 대신 Anthropic 관리 클라우드 환경에서 세션 실행
* [Ultraplan](/docs/ko/ultraplan): 터미널에서 클라우드 계획 세션을 시작하고 브라우저에서 계획을 검토합니다
* [채널](/docs/ko/channels): Telegram, Discord 또는 iMessage를 세션으로 전달하여 Claude가 자리를 비운 동안 메시지에 반응하도록 합니다
* [Dispatch](/docs/ko/desktop#sessions-from-dispatch): 휴대폰에서 작업을 메시지로 보내면 Desktop 세션을 생성하여 처리할 수 있습니다
* [인증](/docs/ko/authentication): `/login` 설정 및 claude.ai 자격 증명 관리
* [CLI 참조](/docs/ko/cli-reference): `claude remote-control`을 포함한 플래그 및 명령의 전체 목록
* [보안](/docs/ko/security): Remote Control 세션이 Claude Code 보안 모델에 어떻게 적합한지
* [데이터 사용](/docs/ko/data-usage): 로컬 및 원격 세션 중에 Anthropic API를 통해 흐르는 데이터
