> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 다른 Claude Code 세션에 메시지 보내기

> Claude가 이 머신의 다른 Claude Code 세션을 나열하고 메시지를 보낼 수 있도록 하며, 다른 머신이나 클라우드의 세션에 도달합니다.

<Note>
  세션 간 메시징은 WSL 2 내부의 Linux를 포함하여 macOS 및 Linux에서 Claude Code v2.1.224 이상이 필요합니다. 기본 Windows에서는 Claude Code v2.1.234 이상이 필요합니다. 세션이 요구 사항을 충족하면 메시징이 활성화할 것이 없이 켜집니다. 공급자 요구 사항 및 세션이 이를 가지고 있는지 확인하는 방법은 [가용성](#availability)을 참조하십시오.
</Note>

세션 간 메시징을 통해 Claude는 Claude Code 세션 중 하나에서 다른 세션으로 메시지를 전달할 수 있습니다. 한 세션의 변경이 다른 세션이 구축하고 있는 것을 깨뜨릴 때, Claude는 사용자가 알아차리기 전에 해당 세션에 경고할 수 있습니다. 한 세션이 다른 세션이 막혀 있는 질문을 해결할 때, Claude는 답변을 전달할 수 있습니다.

메시지는 한 Claude가 다른 Claude에게 작성한 텍스트 조각이며, 발신자의 대화 기록이나 파일은 절대 아닙니다. 전체 대화 또는 해당 컨텍스트를 이동하려면 [세션 재개](/docs/ko/sessions#resume-a-session)를 대신 사용하십시오.

Claude는 이를 위해 두 가지 도구를 사용합니다: 도달할 수 있는 에이전트를 발견하기 위한 `ListAgents`와 이름으로 메시지를 전달하기 위한 `SendMessage`입니다. 동일한 `SendMessage` 도구를 사용하여 Claude는 단일 세션 또는 팀 내에서 [서브에이전트](/docs/ko/sub-agents#resume-subagents) 및 [에이전트 팀](/docs/ko/agent-teams) 팀원에게도 메시지를 보낼 수 있습니다. 이 페이지는 독립적인 세션 간의 메시지를 다룹니다.

<h2 id="when-to-use-cross-session-messaging">
  교차 세션 메시징을 사용할 때
</h2>

한 세션이 다른 세션이 작업 중에 필요한 것을 가지고 있을 때 메시징을 사용하십시오. Claude는 필요를 볼 때 자동으로 메시지를 보낼 수 있습니다. 예를 들어 다른 세션이 수행 중인 작업에 영향을 미치는 변경을 한 후, 또는 사용자가 메시지를 보내도록 요청할 수 있습니다. 일반적인 경우:

* **발견 사항 전달**: 한 세션이 주요 변경 사항을 발견하거나 결정을 내릴 때, Claude는 영향을 받는 영역에서 작업 중인 세션에 요약을 제공하므로 사용자가 거기서 다시 설명할 필요가 없습니다.
* **병렬 worktree 조정**: 세션이 별도의 [worktrees](/docs/ko/worktrees)에서 동일한 저장소에서 작업할 때, Claude는 다른 세션에 무엇이 병합되었는지 알릴 수 있습니다.
* **장기 실행 작업에서 상태 가져오기**: 마이그레이션 또는 테스트 실행이 보고 있는 세션으로 다시 보고하도록 하거나, 거기서 직접 요청하십시오. 해당 세션이 이 머신에 있으면, Claude는 [다음 유휴 상태가 되거나 종료될 때 한 번의 알림을 요청](#get-a-notice-when-another-session-goes-idle)할 수도 있습니다.
* **머신 간 메시지**: 다른 머신이나 클라우드의 세션 중 하나에 도달합니다.

사용자가 직접 시작하고 조종하는 독립적인 세션 간에 메시징을 사용하십시오. Claude Code는 여러 세션을 실행하거나 도달하는 다른 각 방법에 대해 전용 기능을 가지고 있으므로, 수행 중인 작업에 맞게 구축된 기능을 사용하십시오:

* 다른 터미널에서 한 대화를 계속하거나 새 세션과 컨텍스트를 공유하려면 [세션을 재개](/docs/ko/sessions#resume-a-session)하십시오.
* Claude가 생성하고 감독하는 조정된 세션 팀의 경우 [에이전트 팀](/docs/ko/agent-teams)을 사용하십시오.
* 한 곳에서 많은 세션을 보고 조종하려면 [에이전트 보기](/docs/ko/agent-view)를 사용하십시오.
* 세션이 서로 메시지를 보내는 대신 휴대폰이나 다른 장치에서 세션을 직접 조종하려면 [원격 제어](/docs/ko/remote-control)를 사용하십시오.
* CI 결과나 채팅 메시지와 같은 외부 이벤트를 세션으로 푸시하려면 [채널](/docs/ko/channels)을 사용하십시오.

<h2 id="message-another-session">
  다른 세션에 메시지 보내기
</h2>

한 세션이 다른 세션이 필요한 것(예: 발견 사항, 상태 또는 결정)을 배우면, Claude는 사용자가 터미널 간에 복사-붙여넣기하는 대신 이를 전달합니다. Claude는 `ListAgents`로 대상을 발견하고 `SendMessage`로 보내므로, 사용자는 도구를 직접 호출할 필요가 없습니다. Claude는 요청받지 않고도 메시지를 보내기로 결정할 수 있으며, 사용자도 메시지를 보내도록 요청할 수 있습니다.

직접 요청하려면 다른 세션이 알아야 할 사항이나 수행해야 할 작업을 Claude에게 알려주십시오. 이 예는 사용자가 입력하는 프롬프트이며, Claude가 보내는 메시지가 아닙니다:

```text wrap theme={null}
다른 터미널에서 실행 중인 세션에 마이그레이션이 완료되었는지 물어보십시오.
```

Claude는 실제 메시지 자체를 작성하므로, 프롬프트는 내용을 Claude에게 맡길 수 있습니다. 이 프롬프트는 표현을 지정하지 않고 요약을 요청하며, Claude가 보내는 내용은 다양합니다:

```text wrap theme={null}
결제 API에서 작업 중인 세션에 방금 수행한 작업을 설명하십시오.
```

대상을 직접 이름 지으려면 프롬프트에서 세션을 언급하십시오: `@`를 입력한 후 세션 이름의 첫 글자를 입력하고 자동 완성에서 세션을 선택하십시오. 이는 [@-mention a subagent](/docs/ko/sub-agents#invoke-subagents-explicitly)와 동일한 방식입니다. Claude Code v2.1.232 이상이 필요합니다. Claude Code는 `@api-worker`와 같은 언급을 삽입하고 Claude에게 어떤 세션을 이름 지었는지 알려주므로, Claude는 먼저 세션을 나열하지 않고도 해당 세션에 메시지를 보낼 수 있습니다. 이 프롬프트는 언급으로 대상을 이름 짓습니다:

```text wrap theme={null}
@api-worker에 스키마 마이그레이션이 완료되었음을 알려주십시오.
```

자동 완성은 이 머신의 다른 라이브 세션을 나열합니다. 두 가지 경우에는 이름의 첫 글자보다 더 필요합니다:

* **이 머신을 넘어선 세션**: 클라우드 또는 원격 제어 세션은 Claude가 이 머신을 넘어선 세션을 나열하거나 메시지를 보낸 후에만 자동 완성에 나타나므로, Claude에게 먼저 나열하도록 요청하십시오.
* **공백이나 문자, 숫자, 하이픈, 밑줄 이외의 문자가 있는 이름**: 큰따옴표로 입력하십시오. 예: `@"release notes"`. 자동 완성에서 세션을 선택하면, Claude Code가 따옴표를 삽입합니다.

선택기 없이 언급을 입력할 수도 있습니다. 둘 이상의 라이브 세션이 언급된 이름에 응할 때, Claude는 메시지를 보내기 전에 어떤 것을 의미하는지 묻습니다.

메시지가 도착할 때 어떻게 보이는지(예시 포함)는 [메시지가 어떻게 보이는지](#what-a-message-looks-like)를 참조하십시오.

<h3 id="message-delivery">
  메시지 전달
</h3>

수신 Claude는 활성 턴 중에 도구 호출 사이에서 메시지를 읽으므로, 실행 중인 도구는 절대 중단되지 않습니다. 수신 세션이 유휴 상태일 때, Claude Code는 메시지로 새 턴을 시작합니다.

다른 세션에서 온 메시지는 일반 텍스트로 도착합니다. `@`로 파일이나 [MCP 리소스](/docs/ko/mcp#use-mcp-resources)를 언급하면, Claude는 언급을 작성된 대로 보고 Claude Code는 메시지가 새 턴을 시작하든 하나 중에 도착하든 아무것도 첨부하지 않습니다. Claude는 여전히 수신 머신의 언급된 경로를 자신의 도구로 열 수 있으며, 해당 세션의 권한에 따릅니다. v2.1.251 이전에는 새 턴을 시작한 메시지의 `@` 언급이 수신 측에서 파일이나 MCP 리소스를 첨부했습니다.

Claude Code는 다음 경우에 메시지를 거부합니다:

* 메시지가 [크기 제한을 초과](#limitations)합니다. Claude Code는 발신 세션에서 거부하며, 메시지가 떠나기 전입니다.
* 이 머신의 세션으로의 빠른 버스트가 [해당 세션의 받은편지함이 수용하는 것](#limitations)에 도달했습니다. Claude Code는 해당 세션으로의 추가 메시지를 거부합니다.
* 이 머신의 회신 대상이 심볼릭 링크된 대상이나 예상 프로세스가 아닌 엔드포인트와 같은 안전 검사에 실패합니다. [교차 세션 메시지 전송 거부](/docs/ko/errors#refusing-to-send-a-cross-session-message)는 이러한 검사를 나열합니다.
* Claude가 메시지를 이 세션의 자신의 이름으로 주소 지정합니다. [Claude가 도달할 수 있는 세션 확인](#see-which-sessions-claude-can-reach)에서 설명합니다.

수신 세션은 도착하는 각 메시지를 자신의 [인바운드 제어](#control-inbound-messages)에 대해 확인하며, 검사는 다음 세 가지 결과 중 하나로 끝납니다:

* **전달됨**: Claude Code는 메시지를 수신 Claude에게 전달합니다.
* **보류됨**: Claude Code는 메시지를 전달하지 않고 따로 설정합니다. 나중에 `accept`가 적용되거나 모드 또는 설정 변경이 허용하면, Claude Code는 보류된 메시지를 해제합니다.
* **거부됨**: Claude Code는 메시지를 전달하지 않고 삭제합니다.

전달되면, 메시지는 사용자가 입력한 프롬프트처럼 [사용량](/docs/ko/costs)에 포함되며, 수신 Claude는 [일방향 교차 머신 경우](#message-sessions-on-other-machines)를 제외하고 발신자에게 같은 방식으로 회신할 수 있습니다.

권한 경계는 세션별로 유지됩니다. Claude는 자신의 세션에서 거부되거나 차단된 작업을 다른 세션에 요청하지 않도록, 또는 자신의 권한 설정이 차단할 작업을 다른 세션에 요청하지 않도록 지시받으며, 해당 작업을 사용자에게 다시 라우팅합니다. 수신 측에서, [수신 세션의 자신의 권한 프롬프트 및 규칙](#how-a-session-treats-an-incoming-message)은 메시지가 요청하는 모든 것에 여전히 적용됩니다.

<h3 id="get-a-notice-when-another-session-goes-idle">
  다른 세션이 유휴 상태가 될 때 알림 받기
</h3>

Claude는 이 머신의 세션 중 하나에 해당 세션이 다음에 유휴 상태가 되거나 종료될 때 한 번의 알림을 보내도록 요청할 수 있습니다. 여기서 유휴는 세션이 대기 중인 것이 없는 턴을 완료했음을 의미합니다. 다른 세션의 장기 작업을 기다리고 있으며 확인하는 대신 완료되었을 때 듣고 싶을 때 사용하십시오. 두 세션 모두에서 Claude Code v2.1.236 이상이 필요합니다.

<h4 id="ask-for-a-notice">
  알림 요청
</h4>

Claude에게 기다리고 있는 것을 알려주십시오. 이 프롬프트는 마이그레이션 세션에서 알림을 요청합니다:

```text wrap theme={null}
마이그레이션 세션이 작업 중인 것을 완료할 때 알려주십시오.
```

Claude는 `SendMessage` 도구의 `notify_when_idle` 입력으로 구독하며, 어차피 보내고 있는 메시지에 첨부되거나 자체적으로 구독합니다. 자체적으로, Claude Code는 보고 있는 세션에서 턴을 시작하거나 토큰을 소비하지 않고 구독하며, 해당 세션이 이미 유휴 상태이면 즉시 알림을 보냅니다. 메시지에 첨부되면, Claude Code는 먼저 메시지를 전달하고 나중에 알림을 보냅니다.

<h4 id="what-each-session-shows">
  각 세션이 표시하는 것
</h4>

보고 있는 세션은 다른 프로세스가 세션이 다음에 유휴 상태가 될 때 알려달라고 요청했다는 줄을 표시합니다. 요청하는 세션은 알림을 보고 있는 세션의 이름을 지정하는 줄로 표시합니다. 줄에는 해당 세션의 턴이 완료된 시간과 해당 턴의 한 줄 상태가 포함될 수 있습니다. 요청하는 세션이 유휴 상태이면, Claude Code는 알림으로 새 턴을 시작합니다.

<h4 id="limits">
  제한
</h4>

알림은 일회성입니다: Claude Code는 보고 있는 세션에서 한 번 보내며, 어느 세션도 다른 세션을 폴링하지 않습니다. 12시간 내에 알림이 도착하지 않으면, Claude Code는 구독을 삭제하고 Claude에게 알려주므로, 계속 기다리지 않습니다.

각 측의 [인바운드 제어](#control-inbound-messages)는 메시지처럼 알림에 적용됩니다:

* **양쪽에서 `refuse`**: 아무것도 도착하지 않습니다. 보고 있는 세션은 요청을 기록하거나 답변하지 않고 삭제하므로, 구독은 12시간 후 답변 없이 만료되며, `refuse`가 있는 요청하는 세션은 절대 구독하지 않습니다.
* **양쪽에서 `hold`**: 알림이 더 적게 도착합니다. 보고 있는 세션은 한 줄 상태를 생략하며, 요청하는 세션은 Claude에게 전달하지 않고 대화 기록에 알림을 표시합니다.

주 대화의 Claude만 구독할 수 있으며, 이 머신의 세션에만 구독할 수 있습니다. 서브에이전트나 에이전트 팀 팀원이 `notify_when_idle`을 설정하면, Claude Code는 구독하지 않고 이를 알려줍니다. Claude가 팀원, 서브에이전트 또는 이 머신을 넘어선 세션과 같은 다른 에이전트에게 알림을 요청하면, Claude Code는 첨부된 메시지를 포함한 전체 호출을 거부하고 거부를 Claude에게 보고하므로 요청 없이 메시지를 다시 보낼 수 있습니다.

<h3 id="see-which-sessions-claude-can-reach">
  Claude가 도달할 수 있는 세션 확인
</h3>

Claude는 자신의 메시지 대상을 찾으므로, 메시지를 보내도록 요청하기 전에 아무것도 실행할 필요가 없습니다. 직접 Claude가 도달할 수 있는 세션을 확인하려면 `/list-agents` 명령을 실행하십시오. 첫 번째 줄(있을 때)은 이 세션의 자신의 이름이며, 다른 세션이 이를 사용하여 메시지를 보냅니다. 아래 행은 Claude가 도달할 수 있는 세션입니다:

* **서브에이전트**: 현재 세션 내에서 실행 중인 에이전트.
* **팀원**: 이 세션의 자신의 [에이전트 팀](/docs/ko/agent-teams) 팀원. v2.1.239 이전에는 팀원이 나열에 나타나지 않았지만, Claude는 이미 이름으로 메시지를 보낼 수 있었습니다.
* **다른 로컬 세션**: 동일한 머신에서 실행 중인 Claude Code 세션([배경 세션](/docs/ko/agent-view) 포함). 세션은 [받은편지함 소켓](#the-sessions-inbox-socket)을 바인딩할 때만 나타납니다.
* **클라우드 세션**: [원격 제어](/docs/ko/remote-control)에 연결되어 있는 동안 표시되는 [Claude Code on the web](/docs/ko/claude-code-on-the-web) 세션. Claude Code는 나열에서 `cloud`로 레이블을 지정합니다.
* **다른 머신의 원격 제어 세션**: [원격 제어](/docs/ko/remote-control)에 연결되어 있는 동안 표시되며, `Remote Control`로 레이블이 지정됩니다. Claude Code는 원격 제어 연결이 끊어진 세션의 상태를 `offline`으로 표시합니다.

이 세션은 행 중 하나가 아닙니다. Claude가 이 세션의 자신의 이름으로 메시지를 주소 지정하면, Claude Code는 이를 거부하고 대상이 현재 세션임을 Claude에게 알려줍니다. v2.1.239 이전에는 나열이 이 세션의 이름을 표시하지 않았으며, Claude Code는 이를 찾을 수 없는 에이전트로 보낸 메시지를 보고했습니다.

이 세션이 [원격 제어](/docs/ko/remote-control)에 연결되어 있는 동안, Claude Code는 `/list-agents` 출력에서 로컬 세션의 일부 세부 정보를 보류하며, Claude 자체가 메시지를 보낼 세션을 찾을 때 보는 것은 변경하지 않습니다:

* **작업 디렉토리**: 각 로컬 세션의 작업 디렉토리를 생략합니다.
* **세션 이름**: 사람에게 귀속될 수 없는 세션 이름을 생략하므로, 이름이 없는 행은 `(unnamed session)`으로 읽습니다.
* **첫 번째 줄**: 이 터미널에서 해당 이름을 입력하지 않은 한 이 세션의 자신의 이름이 있는 줄을 생략합니다. `--name` 또는 `/rename`과 이름을 사용하여, 세션을 시작하거나 마지막으로 재개한 이후입니다.

출력이 아무것이나 나열할 때, 세부 정보가 보류되었다는 참고로 끝납니다. 세션의 자신의 키보드에서 `/rename` 다음에 사용하지 않은 이름을 실행하면 해당 세션에 출력에 나타나는 이름을 제공합니다.

Claude Code는 클라우드 및 원격 제어 세션 목록을 최신 순서로 읽고 각각에 대해 제한된 페이지 수 후에 중지합니다. 계정에 맞는 것보다 더 많은 세션이 있으면, Claude Code는 오래된 것을 나열하지 않으며, Claude는 이름으로 메시지를 보낼 수 없습니다. 이 경우 Claude Code는 나열에서 이를 알려주며, Claude는 메시지를 보낼 때 동일한 참고를 봅니다.

Claude는 이 머신을 넘어선 세션을 이름으로 주소 지정하며, 로컬 세션과 동일합니다. [다른 머신의 세션에 메시지 보내기](#message-sessions-on-other-machines)에서 이러한 메시지가 어떻게 이동하는지 확인하십시오.

세션은 [`/rename`](/docs/ko/commands) 명령이나 [`--name`](/docs/ko/cli-reference#cli-flags) 플래그로 설정한 이름에 응합니다. 설정하지 않으면, Claude Code가 세션을 이름 짓습니다. 대화형 세션의 경우, 이는 [실행 중인 세션 목록](/docs/ko/sessions#name-your-sessions)에 표시되는 이름입니다.

세션을 이름 바꾸면, Claude Code는 다른 세션이 세션의 이름을 조회하는 데 사용하는 공유 레코드도 업데이트합니다. 해당 레코드를 업데이트할 수 없으면, `/rename` 출력에서 다른 세션이 여전히 이전 이름을 표시할 수 있음을 경고합니다. [`--debug`](/docs/ko/cli-reference#cli-flags)로 세션을 실행하면, Claude Code는 실패한 업데이트의 원인을 기록합니다.

세션을 이름 바꾸거나, 이 머신의 다른 라이브 세션이 이미 사용 중인 이름으로 대화형 세션을 시작하거나 재개하면, Claude Code는 이미 이름을 가진 세션에 이름을 남기고 [변형으로 이름을 바꿉니다](/docs/ko/sessions#name-your-sessions). 세션은 여전히 이름을 공유할 수 있습니다. 예를 들어 하나가 이전 버전의 Claude Code를 실행하거나 공유 이름이 Claude Code가 생성한 것일 때입니다. 이 세션이 원격 제어에 연결되지 않은 한, Claude Code는 `/list-agents` 출력에서 각 로컬 세션의 작업 디렉토리를 표시하므로, 다른 디렉토리에서 실행할 때 같은 이름의 세션을 구분할 수 있습니다. Claude는 이름에 응하는 라이브 세션의 수에 따라 두 가지 방식 중 하나로 메시지를 주소 지정합니다:

* **한 세션이 이름에 응함**: Claude Code는 이름만으로 메시지를 전달합니다.
* **여러 세션이 이름을 공유하거나 Claude Code가 세션이 실행되는 모든 곳을 확인할 수 없음**: Claude는 나열의 각 행에 짧은 식별자를 추가하고 주소에서 식별자를 사용합니다.

<h3 id="message-sessions-on-other-machines">
  다른 머신의 세션에 메시지 보내기
</h3>

메시지가 이동하는 방식과 Anthropic 서버를 통과하는지 여부는 대상 세션이 실행되는 위치에 따라 다릅니다:

| 다른 세션이 실행되는 위치                                         | 메시지가 이동하는 방식                                                               |
| :----------------------------------------------------- | :------------------------------------------------------------------------- |
| 이 머신에서                                                 | macOS 및 Linux의 세션별 소켓 또는 기본 Windows의 세션별 명명된 파이프를 통해, Anthropic 서버를 통하지 않음 |
| 다른 머신 중 하나에서                                           | Anthropic 서버를 통해, 해당 머신의 [원격 제어](/docs/ko/remote-control) 연결을 통해 도착             |
| [Claude Code on the web](/docs/ko/claude-code-on-the-web)에서 | Anthropic 서버를 통해, 클라우드 세션으로 직접                                             |

다른 머신의 세션과 대화를 시작하려면 Claude Code v2.1.225 이상이 필요하며, [나열에 나타나는](#see-which-sessions-claude-can-reach) 대상이 필요합니다. v2.1.225 이전에는 Claude가 도착한 메시지에만 회신할 수 있었습니다.

[나열](#see-which-sessions-claude-can-reach)에서 `offline`으로 표시되는 세션(원격 제어 연결이 끊어진 세션)에 메시지를 보낼 수 있습니다. 전송은 진행되지만, 메시지는 해당 세션의 머신이 다시 연결된 후에만 도착합니다. Claude는 메시지를 보낼 때 이를 알려줍니다.

같은 머신 전달은 기능이 활성화된 모든 곳에서 작동합니다. 각 세션은 디스크의 파일에 자신을 등록합니다. Claude가 로컬 세션을 나열하거나 메시지를 보낼 때, Claude Code는 이러한 파일을 읽어 세션을 찾으므로, 두 세션은 동일한 파일을 볼 수 있을 때만 서로 도달할 수 있습니다.

컨테이너는 자신의 파일 시스템을 가지므로, 컨테이너 내부의 세션과 호스트의 세션은 서로 도달할 수 없습니다. 동일한 컨테이너 내의 두 세션은 여전히 서로 메시지를 보낼 수 있으며, [자체 호스팅 러너](/docs/ko/self-hosted-environments)를 포함합니다. WSL 2 내부의 세션과 동일한 컴퓨터의 기본 Windows 세션도 서로 도달할 수 없습니다. 다른 홈 디렉토리에 등록하고 다른 소켓 유형을 수신하기 때문입니다.

이 세션이 원격 제어에 연결되어 있는 동안, 다른 머신의 세션에 메시지를 보낼 때, Claude Code는 해당 세션의 대화에서 이 세션의 원격 제어 이름 아래에 메시지를 표시합니다. 해당 머신의 Claude는 해당 이름에 회신할 수 있습니다. 예를 들어, 이 세션이 `laptop-graceful-unicorn`으로 원격 제어에 연결되어 있고 데스크톱에 메시지를 보내면, 데스크톱 세션에서 `laptop-graceful-unicorn` 아래에 메시지가 표시됩니다.

Claude가 이 머신을 넘어선 세션으로 보낼 때 이 세션이 원격 제어에 연결되지 않으면, 메시지는 여전히 진행되지만 [회신 주소](#what-a-message-looks-like) 없이, 수신 Claude가 답변할 수 없습니다. Claude는 메시지를 보낼 때 이를 알려줍니다.

이 머신을 넘어선 메시지가 나가기 전에 승인을 요구하려면 [`isolatePeerMachines`](#require-approval-for-cross-machine-messages)를 설정하십시오.

<h2 id="how-a-session-treats-an-incoming-message">
  세션이 들어오는 메시지를 처리하는 방식
</h2>

세션 A가 세션 B에 메시지를 보낼 때, Claude Code는 B의 Claude에게 메시지가 사용자가 아닌 다른 세션에서 왔음을 알리고 메시지가 할 수 있는 것을 제한합니다:

* **아무것도 승인할 수 없음**: 다른 세션의 메시지는 사용자의 동의로 절대 계산되지 않으므로, 사용자를 대신하여 보류 중인 권한 프롬프트에 답변할 수 없습니다.
* **구성을 변경할 수 없음**: Claude Code는 수신 Claude에게 다른 세션이 요청했기 때문에 권한 설정, `CLAUDE.md` 또는 기타 구성을 변경하지 않도록 지시합니다.
* **명령이 실행되지 않음**: `/compact`와 같은 메시지 텍스트의 명령은 일반 텍스트로 도착합니다. Claude Code는 절대 실행하지 않습니다.
* **권한 프롬프트는 여전히 발생함**: 메시지에 따라 행동하려면 수신 세션이 갖지 않은 권한이 필요하면, 다른 작업에 대해 표시되는 동일한 프롬프트가 표시됩니다.

<h3 id="what-a-message-looks-like">
  메시지가 어떻게 보이는지
</h3>

메시지가 도착하면, Claude Code는 대화에서 희미한 한 줄 미리보기로 표시하며, 미리보기 줄은 나중에 대화에 남아 있습니다. 미리보기는 발신자의 이름과 메시지의 첫 줄을 전달하며, 길 때 `…`로 자르며, 예: `› Message from @api-worker: Schema migration finished (ctrl+o to expand)`. v2.1.247 이전에는 Claude Code가 도착하는 메시지를 미리보기 대신 전체로 표시했습니다.

다음 중 하나가 전체 텍스트를 표시합니다:

* `Ctrl+O`를 눌러 [대화 기록 뷰어](/docs/ko/interactive-mode#transcript-viewer)를 열고 발신자의 세션 이름 아래에서 전체 텍스트를 읽습니다.
* [`--verbose`](/docs/ko/cli-reference#cli-flags)로 시작한 세션에서, Claude Code는 미리보기 대신 전체 텍스트를 표시합니다.

미리보기는 표시되는 것만 단축합니다. 확장하든 안 하든, Claude는 전체 메시지를 읽습니다.

Claude는 발신자의 이름과 회신 주소를 포함한 메시지를 받으며, [일방향 교차 머신 메시지](#message-sessions-on-other-machines)는 회신 주소를 전달하지 않습니다. 이름과 회신 주소를 넘어, 수신 Claude는 메시지의 텍스트를 받으며, 발신자의 대화 기록이나 파일은 절대 아닙니다. [메시지 전달](#message-delivery)은 텍스트의 `@` 언급을 다룹니다.

[서브에이전트](/docs/ko/sub-agents)가 작성한 메시지는 발신 세션의 이름 아래에 도착하며, 서브에이전트가 메시지 텍스트에서 식별됩니다. 이에 대한 회신은 해당 세션의 주 대화에 도달하며, 서브에이전트에는 도달하지 않습니다.

이 예는 한 Claude가 다른 Claude에게 작성한 메시지이며, 확장할 때 전체 텍스트로 읽습니다:

```text wrap theme={null}
스키마 마이그레이션 완료됨
새 열은 tenant_id이며, main에 리베이싱하는 것이 안전합니다.
```

<h3 id="control-inbound-messages">
  들어오는 메시지 제어
</h3>

[`crossSessionInbound`](/docs/ko/settings-reference#crosssessioninbound)를 설정하여 세션이 다른 세션에서 도착하는 메시지로 무엇을 하는지 선택하십시오:

| 값        | 동작                                                                                                                                                     |
| :------- | :----------------------------------------------------------------------------------------------------------------------------------------------------- |
| `accept` | Claude Code는 각 메시지를 Claude에게 전달합니다.                                                                                                                    |
| `hold`   | Claude Code는 각 메시지에 대한 알림을 표시하고 전달하지 않습니다. 나중에 `accept`가 적용되면, [우선순위 규칙](/docs/ko/settings-reference#crosssessioninbound)에 따라, Claude Code는 보류된 메시지를 해제합니다. |
| `refuse` | Claude Code는 각 메시지를 전달하지 않고 삭제합니다.                                                                                                                     |

설정 파일을 편집하는 것 외에도, `/config` 행 **다른 세션의 메시지**에서 값을 선택할 수 있습니다. Claude Code는 선택한 값을 사용자 설정에 씁니다. 행은 Claude Code v2.1.232 이상이 필요하며, 관리 설정이나 `--settings` 플래그가 키를 설정할 때는 나타나지 않습니다. 사용자 설정 값이 적용되지 않기 때문입니다. Claude Code는 이 키에 대해 `/config crossSessionInbound=value` 단축을 거부합니다.

어떤 값이 적용되는지 확인하려면, [설정 참조](/docs/ko/settings-reference#crosssessioninbound)에서 `crossSessionInbound` 우선순위 규칙을 따르십시오.

값이 적용되지 않으면, Claude Code는 두 세션의 권한 모드에 따라 메시지별로 결정합니다. [권한 프롬프트를 우회하는](/docs/ko/permission-modes#skip-all-checks-with-bypasspermissions-mode) 세션을 한 클래스로 그룹화하고, 다른 모든 세션을 다른 클래스로 그룹화합니다. Plan 모드는 우회 권한이 있는 대화형 터미널 세션에서 우회로 계산되며, [auto](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode), `acceptEdits`, `dontAsk`는 프롬프트로 계산됩니다:

* **수신 세션이 권한을 프롬프트함**: Claude Code는 각 메시지를 전달합니다. 발신 세션이 권한 프롬프트를 우회하는 것으로 식별될 때만 하나를 승인을 위해 보류합니다.
* **수신 세션이 권한 프롬프트를 우회함**: Claude Code는 각 메시지를 승인을 위해 보류합니다. 발신 세션이 또한 우회하는 것으로 식별될 때만 하나를 전달합니다.

기본값이 메시지를 보류할 때, Claude Code는 수신 세션에서 승인 대화를 엽니다. 대화는 발신자와 미리보기를 표시합니다:

* **승인**은 해당 메시지를 Claude에게 전달합니다.
* **거부** 또는 대화를 닫으면 삭제합니다.
* 대화가 [`dialogExpiry`](/docs/ko/settings-reference#dialogexpiry) 기한을 지나 답변되지 않으면, Claude Code는 이를 닫고 메시지를 삭제합니다. 기한은 기본적으로 5분입니다.
* 터미널이 [배경 세션](/docs/ko/agent-view)에 첨부되지 않은 동안, Claude Code는 대화를 기한을 지나 열어 둡니다. 첨부한 후, 대화가 전체 기한 기간 동안 답변되지 않으면, Claude Code는 이를 닫고 메시지를 삭제합니다.
* 메시지가 보류되는 동안 이 세션의 권한 모드 클래스가 변경되면, Claude Code는 인바운드 규칙을 다시 적용하고, 이제 수용하는 메시지를 전달하며, 알림을 표시합니다.
* 메시지가 보류되는 동안 설정 변경이 `refuse`를 적용하면, Claude Code는 모든 보류된 메시지를 삭제하고 도달할 수 있는 각 발신자에게 거부를 보고합니다.

발신자가 동일한 머신의 세션일 때, Claude Code는 수신자가 메시지를 보류할 때 거기에 알림을 표시하고, 수신자가 나중에 전달, 거부 또는 만료할 때 후속 조치를 표시합니다. 알림은 발신 Claude에 도달하므로, 다른 세션이 읽지 않은 메시지에서 계속 기다리지 않아야 함을 알 수 있습니다.

대화형 발신 세션에서, 알림은 대화 기록에 나타납니다. [`claude -p`](/docs/ko/headless) 발신자는 [스트리밍된 출력](/docs/ko/headless#stream-responses)에서 [정보 `system` 메시지](/docs/ko/agent-sdk/typescript#sdkinformationalmessage)로 받습니다. `claude -p` 발신자에 대한 알림은 Claude Code v2.1.271 이상이 필요합니다.

수신자가 메시지를 거부하면, 발신자의 알림은 수신자가 교차 세션 메시지를 수용하지 않음을 말하고 발신자의 Claude에게 기다리거나 다시 보내지 않도록 알려줍니다.

Claude Code는 최대 100개의 메시지를 보류하며, 전달 큐와 별도로, 그 이상은 가장 오래된 것을 삭제합니다.

<h3 id="non-interactive-sessions">
  비대화형 세션
</h3>

Claude Code는 대화형 세션처럼 [`claude -p`](/docs/ko/headless) 세션에 대해 받은편지함 소켓을 바인딩하므로, 장기 실행 `-p` 워커는 메시지를 받을 수 있으며 나열에 나타납니다. [베어 모드](/docs/ko/headless#start-faster-with-bare-mode)에서 세션을 시작하면, Claude Code는 소켓을 바인딩하지 않으므로, 해당 세션은 메시지를 받을 수 없으며 에이전트 목록에 나타나지 않습니다.

`-p` 세션은 승인 대화를 표시할 수 없습니다. [인바운드 기본값](#control-inbound-messages)이 거기에 메시지를 보류할 때, Claude Code는 대화가 사용하는 동일한 [`dialogExpiry`](/docs/ko/settings-reference#dialogexpiry) 기한(기본적으로 5분)에 대해 이를 유지합니다:

* **기한 전**: 모드 또는 설정 변경이 메시지를 허용하면, Claude Code는 이를 전달합니다.
* **기한 후**: Claude Code는 메시지를 삭제하고 도달할 수 있는 발신자에게 만료로 보고합니다.

`dialogExpiry`를 `"never"`로 설정하여 기본값으로 보류된 메시지를 세션이 끝날 때까지 유지하십시오. 명시적 `hold` 설정으로 보류된 메시지는 만료되지 않습니다. Claude Code는 나중에 `accept`가 적용될 때만 전달합니다.

세션이 여전히 보류된 메시지로 끝나면, Claude Code는 도달할 수 있는 각 발신자에게 만료로 보고합니다. v2.1.225 이전에는 `-p` 세션에 기한이 적용되지 않았습니다: 보류된 메시지는 실행 중 권한 모드 변경이 전달하지 않는 한 보류 상태로 유지되었으며, 보류된 메시지로 끝나는 세션은 발신자에게 아무것도 보고하지 않았습니다.

`-p` 워커가 무인으로 메시지를 받도록 하려면, `--settings` 값에서 `crossSessionInbound`를 `accept`로 설정하여 시작하십시오. 사용자 설정의 `accept`도 작동하지만 실행하는 모든 세션에 적용됩니다.

<h3 id="the-sessions-inbox-socket">
  세션의 받은편지함 소켓
</h3>

세션이 에이전트 목록에 없을 때, 스크립트나 훅이 세션에 게시하기를 원할 때, 또는 샌드박스된 명령이 소켓에 도달할 수 없을 때 이 섹션을 읽으십시오.

Claude Code는 교차 세션 메시징이 활성화된 각 세션에 대해 받은편지함 소켓을 바인딩하며, 이 머신의 다른 세션이 이 소켓으로 메시지를 전달합니다. 소켓은 macOS 및 Linux(WSL 2 내부의 Linux 포함)의 Unix 도메인 소켓이며, 기본 Windows의 명명된 파이프입니다. 어떤 세션 종류가 하나를 바인딩하는지는 [비대화형 세션](#non-interactive-sessions)을 참조하십시오.

소켓의 경로는 두 곳에서 찾을 수 있습니다:

* `/status`는 `Peer address` 행에 표시합니다. 경로는 `uds:`로 접두사가 붙습니다.
* Claude Code는 [훅](/docs/ko/hooks) 및 Bash 명령에 [`CLAUDE_CODE_MESSAGING_SOCKET`](/docs/ko/env-vars#variables) 환경 변수로 내보냅니다:
  * 메시징이 켜진 상태로 시작하는 세션에서, Claude Code는 `SessionStart`를 포함한 모든 훅이 실행되기 전에 변수를 내보냅니다.
  * 각 세션은 자신의 소켓을 내보내며, 부모 세션에서 상속된 것은 절대 아닙니다.

macOS 및 Linux에서, Claude Code는 소켓을 운영 체제 사용자로 제한합니다. 기본 Windows에서는 각 연결이 먼저 운영 체제 사용자만 읽을 수 있는 키로 인증해야 합니다. 어느 쪽이든, 공유 머신에서 다른 사용자의 세션은 이를 전달할 수 없습니다.

macOS 및 Linux에서, Claude Code는 또한 수용할 수 없는 디렉토리(예: 다른 사용자가 소유한 디렉토리)에서 소켓을 만드는 것을 거부하고, 대신 개인 사용자별 디렉토리 `/tmp/cc-socks-<uid>`를 사용합니다. 어떤 디렉토리도 수용할 수 없으면, 세션은 받은편지함 없이 실행됩니다: Claude Code는 알림을 표시하고, `/status`는 `unavailable`과 `Peer address` 행의 이유를 표시하며, [`--debug`](/docs/ko/cli-reference#cli-flags) 로그는 전체 거부를 기록합니다.

소켓의 경로와 함께, Claude Code는 [`CLAUDE_CODE_MESSAGING_TOKEN`](/docs/ko/env-vars#variables)으로 세션별 토큰을 내보냅니다. 자신의 세션의 소켓에 게시하는 스크립트는 `{"type":"auth","token":"<token>"}` 연결의 첫 줄로 보낼 수 있으며, `<token>`은 `CLAUDE_CODE_MESSAGING_TOKEN`의 값입니다. Claude Code가 줄을 요구하는지 여부는 플랫폼에 따라 다릅니다:

* **macOS 및 Linux(WSL 2 포함)**: 줄은 선택 사항입니다. Claude Code는 이를 포함하거나 포함하지 않고 연결을 수용합니다.
* **기본 Windows**: 줄은 필수입니다. Claude Code는 첫 줄이 유효한 인증 줄이 아닌 모든 연결을 닫고 해당 연결에서 아무것도 전달하지 않습니다.

게시할 메시지가 준비되었을 때만 연결을 엽니다. Claude Code는 30초 내에 완전한 줄을 보내지 않은 연결을 닫으므로, 느린 명령의 출력을 먼저 캡처한 다음 연결을 열어 보내십시오.

[자신의 자식 규칙](#own-child-messages) 아래는 Claude Code가 토큰을 참조할 때와 확인할 수 없는 메시지를 처리하는 방식을 설명합니다.

<span id="own-child-messages" />Claude Code는 소켓에 도착하는 메시지를 다른 [인바운드 제어](#control-inbound-messages)와 동일한 피어 메시지를 통해 실행하며, 한 가지 예외와 한 가지 전제 조건이 있습니다:

* **자신의 자식 메시지**: `crossSessionInbound` 값이 적용되지 않을 때, Claude Code는 훅이나 Bash 명령과 같은 세션의 자신의 자식 프로세스에서 온 것으로 확인된 메시지를 전달합니다.
  * Linux(WSL 2 내부 포함)에서, Claude Code는 게시하는 프로세스가 이미 종료된 경우에도 프로세스 증거로 확인할 수 있습니다. macOS에서는 게시하는 프로세스가 여전히 실행 중일 때만 확인할 수 있으며, Claude Code가 프로세스 ID 1로 실행되는 컨테이너에서는 프로세스 증거가 없습니다. 기본 Windows에서도 없습니다.
  * 게시하는 프로세스가 종료된 후 macOS에서 그리고 Claude Code가 프로세스 ID 1로 실행되는 컨테이너에서, 프로세스 증거가 누락되며, Claude Code는 대신 연결을 연 인증 줄에서 세션의 내보낸 [`CLAUDE_CODE_MESSAGING_TOKEN`](/docs/ko/env-vars#variables)을 보낸 자식을 확인합니다. 기본 Windows에서는 해당 토큰이 Claude Code가 자신의 자식 메시지를 확인하는 유일한 방법입니다.
  * Claude Code가 어느 쪽도 확인할 수 없을 때, 권한 클래스를 주장하지 않는 다른 메시지처럼 처리하므로, 권한 프롬프트를 우회하는 세션은 승인을 위해 이를 보류합니다.
* **샌드박스된 세션**: Bash 명령이 [샌드박스](/docs/ko/sandboxing) 내부의 소켓에 도달할 수 있는지 제어하며, 샌드박스의 Unix 소켓 설정 [`sandbox.network.allowAllUnixSockets` 및 `sandbox.network.allowUnixSockets`](/docs/ko/settings-reference#sandbox-settings)를 사용합니다.

<h2 id="restrict-cross-session-messaging">
  교차 세션 메시징 제한
</h2>

메시지별 기본값을 넘어, 두 가지 방식으로 메시징을 좁힐 수 있습니다. 메시지가 머신을 떠나기 전에 승인을 요구하거나, 세션이나 조직에 대해 메시징을 끕니다.

<h3 id="require-approval-for-cross-machine-messages">
  교차 머신 메시지에 대한 승인 요구
</h3>

[`isolatePeerMachines`](/docs/ko/settings-reference#isolatepeermachines)를 `true`로 설정하여 `SendMessage`가 이 머신을 넘어선 세션에 도달하기 전에 명시적 승인을 요구하십시오:

```json theme={null}
{
  "isolatePeerMachines": true
}
```

이를 설정하면, Claude Code는 `bypassPermissions` 모드에서도 Claude의 메시지가 이 머신을 넘어선 세션으로 나가기 전에 승인을 요청합니다. 일반 권한 프롬프트를 건너뜁니다. 모든 설정 범위의 `true`가 적용되므로, 체크인된 프로젝트 파일은 요구사항을 켤 수 있지만 끌 수 없습니다. Claude Code는 동일한 머신의 세션 간 메시지에 대해 프롬프트하지 않습니다.

<h3 id="turn-off-cross-session-messaging">
  교차 세션 메시징 끄기
</h3>

수신 및 전송은 별도의 제어이므로, 필요한 방향을 끄거나 둘 다 끕니다. 도착하는 메시지에는 `crossSessionInbound`를 사용하고, Claude가 여기서 보내거나 나열할 수 있는 것에는 권한 규칙을 사용하십시오:

* **수신 중지**: `crossSessionInbound`를 `refuse`로 설정하면, Claude Code는 인바운드 피어 메시지를 전달하지 않고 삭제합니다. 프로젝트 또는 로컬 설정에서, `refuse`는 다른 모든 소스를 통해 적용되며, 사용자 설정에서는 관리 설정이나 `--settings` 플래그가 값을 설정하지 않는 한 적용됩니다.
* **전송 및 나열 중지**: `SendMessage` 및 `ListAgents`를 이름 지정하는 [권한 거부 규칙](/docs/ko/permissions#tool-specific-permission-rules)을 추가하십시오. 둘 다 지정자 없이 도구 이름을 사용합니다.

관리자는 [관리 설정](/docs/ko/managed-settings)에서 조직에 대해 양쪽을 끌 수 있으며, 거부 규칙과 `refuse`를 결합합니다:

```json theme={null}
{
  "permissions": {
    "deny": ["SendMessage", "ListAgents"]
  },
  "crossSessionInbound": "refuse"
}
```

이를 설정하면, Claude Code는 여전히 각 세션의 받은편지함 소켓을 바인딩하지만, 도착하는 모든 메시지를 Claude에게 전달하지 않고 삭제합니다. `SendMessage`를 거부하면 동일한 도구가 둘 다 제공하므로 서브에이전트 및 에이전트 팀 팀원에 대한 메시징도 제거됩니다. 거부하는 세션은 자신의 `/status` 또는 동일한 머신의 다른 세션 나열에서 눈에 띄는 변화를 표시하지 않으므로, 확인하려면 해당 세션의 상태보다는 해당 세션에 적용되는 설정 파일을 확인하십시오.

<h2 id="availability">
  가용성
</h2>

교차 세션 메시징은 macOS, Linux 및 WSL 2에서 Claude Code v2.1.224 이상이 필요하며, 기본 Windows에서는 v2.1.234 이상이 필요합니다. 가용성 및 Claude가 메시지를 보낼 수 있는 세션은 운영 체제, 공급자 및 구성에 따라 다릅니다:

* **운영 체제**: macOS, Windows 및 Linux(WSL 2 내부의 Linux 포함)에서 사용 가능합니다.

* **이 머신의 세션**: Amazon Bedrock, Claude Platform on AWS, Google Cloud의 Agent Platform 및 Microsoft Foundry를 포함한 모든 공급자에서 사용 가능하며, [기능 플래그 가져오기](/docs/ko/env-vars#features-that-need-feature-flag-fetching)가 꺼진 세션에서 사용 가능합니다. 이러한 공급자에서 그리고 플래그 가져오기가 꺼진 상태에서, 동일한 머신 메시징은 Claude Code v2.1.248 이상이 필요합니다. Claude Code는 이러한 메시지를 [머신의 세션별 소켓](#the-sessions-inbox-socket)을 통해 전달하며, Anthropic 서버를 통하지 않습니다.

  세션이 이를 받는 것을 중지하려면, [`crossSessionInbound`](#turn-off-cross-session-messaging)를 `refuse`로 설정하십시오.

* **이 머신을 넘어선 세션**: Claude는 [클라우드 세션](/docs/ko/claude-code-on-the-web)과 원격 제어에 연결된 세션에서 다른 머신의 세션을 찾으며, 이는 이 세션의 활성 인증으로 claude.ai 로그인이 필요하며 다른 [원격 제어 요구사항](/docs/ko/remote-control#requirements)이 필요합니다. Claude는 API 키 또는 Amazon Bedrock, Claude Platform on AWS, Google Cloud의 Agent Platform 및 Microsoft Foundry에서 이러한 세션을 찾을 수 없습니다.

세션을 확인하려면 `/list-agents`를 입력하십시오. `/peers`로도 사용 가능합니다. 결과는 기능이 없는 세션을 메시지 전송이 실패한 것과 같은 더 좁은 것으로 인한 세션과 분리합니다. 예: 누락된 `SendMessage` 도구 또는 거부된 전송:

* **`/list-agents`가 인식되지 않음**: 세션에 교차 세션 메시징이 없습니다. 위의 요구사항을 통해 작업하십시오. `claude --version`으로 버전 요구사항을 시작하십시오.
* **`/list-agents`는 작동하지만 전송이 도착하지 않음**: 메시징이 켜져 있으며, 더 좁은 것이 적용됩니다:
  * **거부 규칙**: [권한 거부 규칙](#turn-off-cross-session-messaging)은 `SendMessage` 및 `ListAgents` 도구를 제거합니다.
  * **인바운드 제어**: [수신 세션의 인바운드 제어](#control-inbound-messages)는 보내는 것을 보류하거나 삭제할 수 있습니다.
  * **클라우드 세션 누락**: 클라우드 세션은 이 세션이 [원격 제어](/docs/ko/remote-control)에 연결되어 있는 동안만 나타납니다.
  * **다른 머신 세션 누락**: 다른 머신의 세션은 [원격 제어](/docs/ko/remote-control)로 실행되고 이 세션도 연결되어 있을 때만 나타납니다.
  * **다른 머신 세션 `offline`**: `offline`으로 나열된 세션으로의 메시지는 진행되지만, [해당 세션의 머신이 다시 연결된 후에만 도착](#message-sessions-on-other-machines)합니다.
  * **오래된 클라우드 또는 다른 머신 세션 누락**: Claude Code는 [이러한 세션 목록을 최신 순서로 읽고 제한된 페이지 수 후에 중지](#see-which-sessions-claude-can-reach)하므로, Claude는 이름으로 지난 세션에 메시지를 보낼 수 없습니다.
  * **대화 시작**: [다른 머신의 세션에 메시지 보내기](#message-sessions-on-other-machines)는 이 머신을 넘어선 세션과 대화를 시작하는 것을 다룹니다.

메시징이 있는 세션에서, `/status`는 또한 세션의 자신의 받은편지함 주소를 포함한 `Peer address` 행을 표시하거나, Claude Code가 [받은편지함을 설정할 수 없을 때](#the-sessions-inbox-socket) `unavailable`과 이유를 표시합니다.

<h2 id="limitations">
  제한
</h2>

여기의 제한은 메시징 채널 자체의 속성이며 기능이 실행되는 모든 곳에 적용됩니다. 플랫폼 및 공급자 격차는 [가용성](#availability)을 참조하십시오.

* **일반 텍스트만**: Claude는 세션 간에 일반 텍스트만 보냅니다. 구조화된 [에이전트 팀](/docs/ko/agent-teams) 프로토콜 메시지는 팀 내에 유지됩니다.
* **동일한 머신 메시지 크기는 제한됨**: Claude Code는 직렬화된 형식이 약 백만 문자를 통과하면 이 머신의 세션으로의 메시지를 거부합니다. 거부는 [정확한 크기를 이름 지음](/docs/ko/errors#message-too-large-for-cross-session-delivery)합니다. 아무것도 수신 세션에 도달하지 않습니다.
* **한 세션으로의 빠른 버스트는 발신자에서 거부됨**: 한 세션으로의 빠른 메시지 버스트가 해당 세션의 받은편지함이 수용하는 것에 도달하면, Claude Code는 발신 세션에서 추가 전송을 거부합니다. [거부는 버스트를 이름 지음](/docs/ko/errors#too-many-messages-to-this-session-just-now)하고 Claude에게 나머지를 하나의 메시지로 일괄 처리하거나 기다리도록 알려줍니다. v2.1.236 이전에는 Claude Code가 이러한 전송을 보낸 것으로 보고했으며 수신 세션이 이를 삭제했습니다.
* **메시지 루프는 제한됨**: 수신 세션에서, Claude Code는 발신자별로 반복된 메시지의 속도를 제한하고, 짧은 창 내에 도착하는 동일한 반복을 삭제하며, Claude가 읽을 수 있도록 최대 50개의 수용된 메시지를 큐에 넣습니다. 따라서 두 세션 간의 메시지 루프는 자체적으로 중지됩니다. 속도 제한, 반복 검사 또는 큐 제한이 이 머신의 대화형 세션에서 메시지를 삭제할 때, Claude Code는 어떤 것이 삭제했는지 알려주고 Claude에게 즉시 다시 보내지 않도록 알려줍니다.

<h2 id="related-resources">
  관련 리소스
</h2>

* [서브에이전트](/docs/ko/sub-agents#resume-subagents) 및 [에이전트 팀](/docs/ko/agent-teams#messages-between-agents): 단일 세션이나 팀 내의 메시징
* [배경 에이전트](/docs/ko/agent-view): 메시지를 보낼 수 있는 병렬 세션 디스패치 및 모니터링
* [원격 제어](/docs/ko/remote-control): 이 세션을 연결하여 다른 머신의 세션에 도달
* [설정](/docs/ko/settings-reference#all-settings): `crossSessionInbound`, `isolatePeerMachines` 및 `dialogExpiry`
* [권한 모드](/docs/ko/permission-modes): 인바운드 기본값의 두 클래스 뒤의 모드
* [도구 참조](/docs/ko/tools-reference): 도구 테이블의 `ListAgents` 및 `SendMessage` 행
* [에이전트를 병렬로 실행](/docs/ko/agents): Claude Code가 여러 에이전트를 실행하는 방식 비교
