> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 채널을 사용하여 실행 중인 세션으로 이벤트 푸시하기

> 채널을 사용하여 MCP 서버에서 실행 중인 Claude Code 세션으로 메시지, 알림 및 웹훅을 푸시합니다. CI 결과, 채팅 메시지 및 모니터링 이벤트를 전달하여 Claude가 자리를 비웠을 때 반응할 수 있도록 합니다.

<Note>
  채널은 [연구 미리보기](#research-preview)에 있으며 Anthropic 인증이 필요합니다(claude.ai 또는 Console API 키를 통해). Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry에서는 사용할 수 없습니다. Team 및 Enterprise 조직은 [명시적으로 활성화](#enterprise-controls)해야 합니다.
</Note>

채널은 실행 중인 Claude Code 세션으로 이벤트를 푸시하는 MCP 서버이므로 Claude는 터미널에 없을 때 발생하는 일에 반응할 수 있습니다. 채널은 양방향일 수 있습니다. Claude는 이벤트를 읽고 동일한 채널을 통해 다시 회신합니다(채팅 브리지처럼). 이벤트는 세션이 열려 있는 동안에만 도착하므로 항상 켜진 설정의 경우 Claude를 백그라운드 프로세스 또는 지속적인 터미널에서 실행합니다.

새로운 클라우드 세션을 생성하거나 폴링될 때까지 기다리는 통합과 달리 이벤트는 이미 열려 있는 세션에 도착합니다. [채널이 어떻게 비교되는지](#how-channels-compare) 참조하세요.

채널을 플러그인으로 설치하고 자신의 자격증명으로 구성합니다. Telegram, Discord 및 iMessage는 연구 미리보기에 포함되어 있습니다.

Claude가 채널을 통해 회신할 때 터미널에서 인바운드 메시지를 볼 수 있지만 회신 텍스트는 볼 수 없습니다. 터미널에는 도구 호출과 확인(예: "전송됨")이 표시되고 실제 회신은 다른 플랫폼에 나타납니다.

Team, Enterprise 또는 Console 조직을 관리하는 경우 [조직에 대해 채널 활성화](#enterprise-controls)를 참조하세요. 자신의 채널을 구축하려면 [채널 참조](/docs/ko/channels-reference)를 참조하세요.

<h2 id="supported-channels">
  지원되는 채널
</h2>

각 지원되는 채널은 [Bun](https://bun.sh)이 필요한 플러그인입니다. 실제 플랫폼을 연결하기 전에 플러그인 흐름의 실습 데모를 보려면 [fakechat 빠른 시작](#quickstart)을 시도하세요.

<Tabs>
  <Tab title="Telegram">
    전체 [Telegram 플러그인 소스](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/telegram)를 확인하세요.

    <Steps>
      <Step title="Telegram 봇 만들기">
        Telegram에서 [BotFather](https://t.me/BotFather)를 열고 `/newbot`을 보냅니다. 표시 이름과 `bot`으로 끝나는 고유한 사용자 이름을 지정합니다. BotFather가 반환하는 토큰을 복사합니다.
      </Step>

      <Step title="플러그인 설치">
        Claude Code에서 다음을 실행합니다:

        ```
        /plugin install telegram@claude-plugins-official
        ```

        Claude Code가 플러그인을 어떤 마켓플레이스에서도 찾을 수 없다고 보고하면 마켓플레이스가 누락되었거나 오래되었습니다. `/plugin marketplace update claude-plugins-official`을 실행하여 새로 고치거나 이전에 추가하지 않았다면 `/plugin marketplace add anthropics/claude-plugins-official`을 실행합니다. 그런 다음 설치를 다시 시도합니다.

        설치 후 `/reload-plugins`을 실행하여 플러그인의 구성 명령을 활성화합니다.
      </Step>

      <Step title="토큰 구성">
        BotFather의 토큰으로 구성 명령을 실행합니다:

        ```
        /telegram:configure <token>
        ```

        이것은 `~/.claude/channels/telegram/.env`에 저장됩니다. Claude Code를 시작하기 전에 셸 환경에서 `TELEGRAM_BOT_TOKEN`을 설정할 수도 있습니다.
      </Step>

      <Step title="채널이 활성화된 상태로 다시 시작">
        Claude Code를 종료하고 채널 플래그로 다시 시작합니다. 이것은 Telegram 플러그인을 시작하여 봇에서 메시지 폴링을 시작합니다:

        ```bash theme={null}
        claude --channels plugin:telegram@claude-plugins-official
        ```
      </Step>

      <Step title="계정 페어링">
        Telegram을 열고 봇에 메시지를 보냅니다. 봇이 페어링 코드로 회신합니다.

        <Note>봇이 응답하지 않으면 Claude Code가 이전 단계에서 `--channels`로 실행 중인지 확인하세요. 봇은 채널이 활성화된 동안에만 회신할 수 있습니다.</Note>

        Claude Code로 돌아가서 다음을 실행합니다:

        ```
        /telegram:access pair <code>
        ```

        그런 다음 계정만 메시지를 보낼 수 있도록 액세스를 잠급니다:

        ```
        /telegram:access policy allowlist
        ```
      </Step>
    </Steps>
  </Tab>

  <Tab title="Discord">
    전체 [Discord 플러그인 소스](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/discord)를 확인하세요.

    <Steps>
      <Step title="Discord 봇 만들기">
        [Discord Developer Portal](https://discord.com/developers/applications)로 이동하여 **New Application**을 클릭하고 이름을 지정합니다. **Bot** 섹션에서 사용자 이름을 만든 다음 **Reset Token**을 클릭하고 토큰을 복사합니다.
      </Step>

      <Step title="Message Content Intent 활성화">
        봇의 설정에서 **Privileged Gateway Intents**로 스크롤하고 **Message Content Intent**를 활성화합니다.
      </Step>

      <Step title="봇을 서버에 초대">
        **OAuth2 > URL Generator**로 이동합니다. `bot` 범위를 선택하고 다음 권한을 활성화합니다:

        * View Channels
        * Send Messages
        * Send Messages in Threads
        * Read Message History
        * Attach Files
        * Add Reactions

        생성된 URL을 열어 봇을 서버에 추가합니다.
      </Step>

      <Step title="플러그인 설치">
        Claude Code에서 다음을 실행합니다:

        ```
        /plugin install discord@claude-plugins-official
        ```

        Claude Code가 플러그인을 어떤 마켓플레이스에서도 찾을 수 없다고 보고하면 마켓플레이스가 누락되었거나 오래되었습니다. `/plugin marketplace update claude-plugins-official`을 실행하여 새로 고치거나 이전에 추가하지 않았다면 `/plugin marketplace add anthropics/claude-plugins-official`을 실행합니다. 그런 다음 설치를 다시 시도합니다.

        설치 후 `/reload-plugins`을 실행하여 플러그인의 구성 명령을 활성화합니다.
      </Step>

      <Step title="토큰 구성">
        복사한 봇 토큰으로 구성 명령을 실행합니다:

        ```
        /discord:configure <token>
        ```

        이것은 `~/.claude/channels/discord/.env`에 저장됩니다. Claude Code를 시작하기 전에 셸 환경에서 `DISCORD_BOT_TOKEN`을 설정할 수도 있습니다.
      </Step>

      <Step title="채널이 활성화된 상태로 다시 시작">
        Claude Code를 종료하고 채널 플래그로 다시 시작합니다. 이것은 Discord 플러그인을 연결하여 봇이 메시지를 수신하고 응답할 수 있도록 합니다:

        ```bash theme={null}
        claude --channels plugin:discord@claude-plugins-official
        ```
      </Step>

      <Step title="계정 페어링">
        Discord에서 봇에 DM을 보냅니다. 봇이 페어링 코드로 회신합니다.

        <Note>봇이 응답하지 않으면 Claude Code가 이전 단계에서 `--channels`로 실행 중인지 확인하세요. 봇은 채널이 활성화된 동안에만 회신할 수 있습니다.</Note>

        Claude Code로 돌아가서 다음을 실행합니다:

        ```
        /discord:access pair <code>
        ```

        그런 다음 계정만 메시지를 보낼 수 있도록 액세스를 잠급니다:

        ```
        /discord:access policy allowlist
        ```
      </Step>
    </Steps>
  </Tab>

  <Tab title="iMessage">
    전체 [iMessage 플러그인 소스](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/imessage)를 확인하세요.

    iMessage 채널은 Messages 데이터베이스를 직접 읽고 AppleScript를 통해 회신을 보냅니다. macOS가 필요하며 봇 토큰이나 외부 서비스가 필요하지 않습니다.

    <Steps>
      <Step title="전체 디스크 액세스 권한 부여">
        `~/Library/Messages/chat.db`의 Messages 데이터베이스는 macOS로 보호됩니다. 서버가 처음 읽을 때 macOS는 액세스를 요청합니다. **Allow**를 클릭합니다. 프롬프트는 Terminal, iTerm 또는 IDE와 같이 Bun을 시작한 앱의 이름을 지정합니다.

        프롬프트가 나타나지 않거나 Don't Allow를 클릭한 경우 **System Settings > Privacy & Security > Full Disk Access**에서 수동으로 액세스를 부여하고 터미널을 추가합니다. 이 없이는 서버가 `authorization denied`로 즉시 종료됩니다.
      </Step>

      <Step title="플러그인 설치">
        Claude Code에서 다음을 실행합니다:

        ```
        /plugin install imessage@claude-plugins-official
        ```

        Claude Code가 플러그인을 어떤 마켓플레이스에서도 찾을 수 없다고 보고하면 마켓플레이스가 누락되었거나 오래되었습니다. `/plugin marketplace update claude-plugins-official`을 실행하여 새로 고치거나 이전에 추가하지 않았다면 `/plugin marketplace add anthropics/claude-plugins-official`을 실행합니다. 그런 다음 설치를 다시 시도합니다.
      </Step>

      <Step title="채널이 활성화된 상태로 다시 시작">
        Claude Code를 종료하고 채널 플래그로 다시 시작합니다:

        ```bash theme={null}
        claude --channels plugin:imessage@claude-plugins-official
        ```
      </Step>

      <Step title="자신에게 문자 보내기">
        Apple ID로 로그인한 모든 기기에서 Messages를 열고 자신에게 메시지를 보냅니다. 즉시 Claude에 도달합니다. 자체 채팅은 설정 없이 액세스 제어를 우회합니다.

        <Note>Claude가 보내는 첫 번째 회신은 터미널이 Messages를 제어할 수 있는지 묻는 macOS Automation 프롬프트를 트리거합니다. **OK**를 클릭합니다.</Note>
      </Step>

      <Step title="다른 발신자 허용">
        기본적으로 자신의 메시지만 통과합니다. 다른 연락처가 Claude에 도달하도록 하려면 해당 핸들을 추가합니다:

        ```
        /imessage:access allow +15551234567
        ```

        핸들은 `+country` 형식의 전화번호 또는 `user@example.com`과 같은 Apple ID 이메일입니다.
      </Step>
    </Steps>
  </Tab>
</Tabs>

아직 플러그인이 없는 시스템의 경우 [자신의 채널을 구축](/docs/ko/channels-reference)할 수도 있습니다.

<h2 id="quickstart">
  빠른 시작
</h2>

Fakechat은 localhost에서 채팅 UI를 실행하는 공식적으로 지원되는 데모 채널이며 인증할 것도 없고 구성할 외부 서비스도 없습니다.

fakechat을 설치하고 활성화한 후 브라우저에서 입력하면 메시지가 Claude Code 세션에 도착합니다. Claude가 회신하면 회신이 브라우저로 돌아옵니다. fakechat 인터페이스를 테스트한 후 [Telegram](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/telegram), [Discord](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/discord) 또는 [iMessage](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins/imessage)를 시도해 보세요.

fakechat 데모를 시도하려면 다음이 필요합니다:

* Claude Code [설치 및 인증](/docs/ko/quickstart#step-1-install-claude-code): claude.ai 계정 또는 Claude Console API 키 사용
* [Bun](https://bun.sh) 설치됨. 사전 구축된 채널 플러그인은 Bun 스크립트입니다. `bun --version`으로 확인하세요. 실패하면 [Bun 설치](https://bun.sh/docs/installation)하세요.
* **Team, Enterprise 또는 관리 Console 조직**: 조직 관리자가 관리 설정에서 [채널을 활성화](#enterprise-controls)해야 합니다.

<Steps>
  <Step title="fakechat 채널 플러그인 설치">
    Claude Code 세션을 시작하고 설치 명령을 실행합니다:

    ```text theme={null}
    /plugin install fakechat@claude-plugins-official
    ```

    Claude Code가 플러그인을 어떤 마켓플레이스에서도 찾을 수 없다고 보고하면 마켓플레이스가 누락되었거나 오래되었습니다. `/plugin marketplace update claude-plugins-official`을 실행하여 새로 고치거나 이전에 추가하지 않았다면 `/plugin marketplace add anthropics/claude-plugins-official`을 실행합니다. 그런 다음 설치를 다시 시도합니다.
  </Step>

  <Step title="채널이 활성화된 상태로 다시 시작">
    Claude Code를 종료한 다음 `--channels`로 다시 시작하고 설치한 fakechat 플러그인을 전달합니다:

    ```bash theme={null}
    claude --channels plugin:fakechat@claude-plugins-official
    ```

    fakechat 서버가 자동으로 시작됩니다.

    <Tip>
      `--channels`에 여러 플러그인을 공백으로 구분하여 전달할 수 있습니다.
    </Tip>
  </Step>

  <Step title="메시지 푸시">
    [http://localhost:8787](http://localhost:8787)에서 fakechat UI를 열고 메시지를 입력합니다:

    ```text theme={null}
    hey, what's in my working directory?
    ```

    메시지는 Claude Code 세션에 `<channel source="fakechat">` 이벤트로 도착합니다. Claude가 읽고 작업을 수행한 다음 fakechat의 `reply` 도구를 호출합니다. 답변이 채팅 UI에 나타납니다.
  </Step>
</Steps>

Claude가 터미널에서 멀리 있을 때 권한 프롬프트에 도달하면 세션이 응답할 때까지 일시 중지됩니다. [권한 릴레이 기능](/docs/ko/channels-reference#relay-permission-prompts)을 선언하는 채널 서버는 이러한 프롬프트를 사용자에게 전달하여 원격으로 승인하거나 거부할 수 있습니다. 무인 사용의 경우 [`--dangerously-skip-permissions`](/docs/ko/permission-modes#skip-all-checks-with-bypasspermissions-mode)는 명시적 요청 규칙 이외의 프롬프트를 우회하지만 신뢰하는 환경에서만 사용하세요. 명시적 요청 규칙, 커넥터 도구 [조직에서 `ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools), 그리고 [`requiresUserInteraction`](/docs/ko/mcp#require-approval-for-a-specific-tool)으로 표시된 MCP 도구는 여전히 프롬프트를 표시합니다.

비대화형 모드에서 `-p`로 채널을 실행할 때 여러 선택지 질문 및 계획 모드 승인과 같이 터미널 입력이 필요한 도구는 비활성화되므로 세션이 입력을 기다리며 멈추지 않습니다.

<h2 id="security">
  보안
</h2>

승인된 모든 채널 플러그인은 발신자 허용 목록을 유지합니다. 추가한 ID만 메시지를 푸시할 수 있으며 다른 모든 것은 자동으로 삭제됩니다.

Telegram 및 Discord는 페어링으로 목록을 부트스트랩합니다:

1. Telegram 또는 Discord에서 봇을 찾고 메시지를 보냅니다.
2. 봇이 페어링 코드로 회신합니다.
3. Claude Code 세션에서 코드를 승인합니다.
4. 발신자 ID가 허용 목록에 추가됩니다.

iMessage는 다르게 작동합니다. 자신에게 문자를 보내면 자동으로 게이트를 우회하고 `/imessage:access allow`로 다른 연락처를 핸들로 추가합니다.

그 위에 `--channels`로 각 세션에서 활성화된 서버를 제어하고 조직은 [`channelsEnabled`](#enterprise-controls)로 가용성을 제어합니다. claude.ai Team 및 Enterprise 계획과 관리 설정을 배포하는 Console 조직에서 가능합니다.

`.mcp.json`에 있는 것만으로는 메시지를 푸시하기에 충분하지 않습니다. 서버도 `--channels`에서 명명되어야 합니다.

허용 목록은 채널이 선언하는 경우 [권한 릴레이](/docs/ko/channels-reference#relay-permission-prompts)도 게이트합니다. 채널을 통해 회신할 수 있는 모든 사람이 세션에서 도구 사용을 승인하거나 거부할 수 있으므로 해당 권한을 신뢰하는 발신자만 허용 목록에 추가하세요.

<h2 id="enterprise-controls">
  Enterprise 제어
</h2>

관리자는 사용자가 재정의할 수 없는 두 가지 [관리 설정](/docs/ko/settings)을 통해 가용성을 제어합니다. 기본값은 인증 방식에 따라 다릅니다:

* **claude.ai Team 및 Enterprise**: 소유자가 활성화할 때까지 채널이 차단됩니다.
* **Anthropic Console with API key authentication**: 채널이 기본적으로 허용됩니다. 조직이 관리 설정을 배포하는 경우에만 이 설정이 필요합니다.

모든 경우에 사용자가 `--channels`로 세션에 옵트인할 때까지 채널이 실행되지 않습니다.

| 설정                      | 목적                                                                                                                                                                     | 구성되지 않은 경우                                                                                          |
| :---------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------- |
| `channelsEnabled`       | 마스터 스위치. 채널이 메시지를 전달하려면 `true`여야 합니다. [claude.ai Admin 콘솔](https://claude.ai/admin-settings/claude-code) 토글을 통해 또는 관리 설정에서 직접 설정합니다. 꺼져 있을 때 개발 플래그를 포함한 모든 채널을 차단합니다. | claude.ai Team 및 Enterprise: 채널 차단됨. Console: 조직이 관리 설정을 배포하지 않는 한 채널 허용됨. 이 경우 이 키가 설정될 때까지 채널 차단됨 |
| `allowedChannelPlugins` | 채널이 활성화되면 등록할 수 있는 플러그인. 설정되면 Anthropic 유지 관리 목록을 대체합니다. `channelsEnabled`가 `true`일 때만 적용됩니다.                                                                          | Anthropic 기본 목록 적용                                                                                  |

조직이 없는 Pro 및 Max 사용자는 이러한 검사를 완전히 건너뜁니다. 채널을 사용할 수 있으며 사용자는 `--channels`로 세션당 옵트인합니다.

<h3 id="enable-channels-for-your-organization">
  조직에 대해 채널 활성화
</h3>

조직에 대해 채널을 활성화하려면 소유자 역할이 필요한 [**claude.ai → Admin settings → Claude Code → Channels**](https://claude.ai/admin-settings/claude-code)에서 활성화하거나 관리 설정에서 `channelsEnabled`를 `true`로 설정합니다.

활성화되면 조직의 사용자는 `--channels`를 사용하여 개별 세션에 채널 서버를 옵트인할 수 있습니다. 설정이 비활성화되었거나 설정되지 않은 경우 MCP 서버는 여전히 연결되고 해당 도구가 작동하지만 채널 메시지는 도착하지 않습니다. 시작 경고는 사용자에게 관리자가 설정을 활성화하도록 합니다.

<h3 id="restrict-which-channel-plugins-can-run">
  실행할 수 있는 채널 플러그인 제한
</h3>

기본적으로 Anthropic 유지 관리 허용 목록의 모든 플러그인이 채널로 등록할 수 있습니다. Team 및 Enterprise 계획의 관리자는 관리 설정에서 `allowedChannelPlugins`을 설정하여 해당 허용 목록을 자신의 목록으로 바꿀 수 있습니다. 이를 사용하여 허용되는 공식 플러그인을 제한하거나 자신의 내부 마켓플레이스에서 채널을 승인하거나 둘 다 수행합니다. 각 항목은 플러그인과 그것이 나오는 마켓플레이스의 이름을 지정합니다:

```json theme={null}
{
  "channelsEnabled": true,
  "allowedChannelPlugins": [
    { "marketplace": "claude-plugins-official", "plugin": "telegram" },
    { "marketplace": "claude-plugins-official", "plugin": "discord" },
    { "marketplace": "acme-corp-plugins", "plugin": "internal-alerts" }
  ]
}
```

`allowedChannelPlugins`이 설정되면 Anthropic 허용 목록을 완전히 대체합니다. 나열된 플러그인만 등록할 수 있습니다. 기본 Anthropic 허용 목록으로 돌아가려면 설정하지 않은 상태로 두세요. 빈 배열은 허용 목록의 모든 채널 플러그인을 차단하지만 `--dangerously-load-development-channels`는 여전히 로컬 테스트를 위해 이를 우회할 수 있습니다. 개발 플래그를 포함한 채널을 완전히 차단하려면 대신 `channelsEnabled`를 설정하지 않은 상태로 두세요.

이 설정에는 `channelsEnabled: true`가 필요합니다. 사용자가 `--channels`에 조직 목록에 없는 플러그인을 전달하면 Claude Code가 정상적으로 시작되지만 채널이 등록되지 않으며 시작 알림이 플러그인이 조직의 승인된 목록에 없음을 설명합니다.

<h2 id="research-preview">
  연구 미리보기
</h2>

채널은 연구 미리보기 기능입니다. 가용성은 점진적으로 출시되고 있으며 `--channels` 플래그 구문 및 프로토콜 계약은 피드백에 따라 변경될 수 있습니다.

미리보기 중에 `--channels`는 Anthropic 유지 관리 허용 목록의 플러그인만 허용하거나 관리자가 [`allowedChannelPlugins`](#restrict-which-channel-plugins-can-run)을 설정한 경우 조직의 허용 목록에서만 허용합니다. [claude-plugins-official](https://github.com/anthropics/claude-plugins-official/tree/main/external_plugins)의 채널 플러그인은 기본 승인된 집합입니다. 유효한 허용 목록에 없는 것을 전달하면 Claude Code가 정상적으로 시작되지만 채널이 등록되지 않으며 시작 알림이 이유를 알려줍니다.

구축 중인 채널을 테스트하려면 `--dangerously-load-development-channels`를 사용합니다. 구축하는 사용자 정의 채널 테스트에 대한 정보는 [연구 미리보기 중 테스트](/docs/ko/channels-reference#test-during-the-research-preview)를 참조하세요.

[Claude Code GitHub 저장소](https://github.com/anthropics/claude-code/issues)에서 문제 또는 피드백을 보고합니다.

<h2 id="how-channels-compare">
  채널이 어떻게 비교되는지
</h2>

여러 Claude Code 기능이 터미널 외부의 시스템에 연결되며 각각 다른 종류의 작업에 적합합니다:

| 기능                                           | 수행하는 작업                                   | 좋은 점                                  |
| -------------------------------------------- | ----------------------------------------- | ------------------------------------- |
| [웹의 Claude Code](/docs/ko/claude-code-on-the-web) | GitHub에서 복제된 새로운 클라우드 샌드박스에서 작업 실행        | 나중에 확인하는 자체 포함된 비동기 작업 위임             |
| [Slack의 Claude](/docs/ko/slack)                   | 채널 또는 스레드의 `@Claude` 언급에서 웹 세션 생성         | 팀 대화 컨텍스트에서 직접 작업 시작                  |
| 표준 [MCP 서버](/docs/ko/mcp)                         | Claude는 작업 중에 쿼리합니다. 세션으로 푸시되는 것은 없습니다.   | Claude에게 시스템을 읽거나 쿼리하기 위한 온디맨드 액세스 제공 |
| [Remote Control](/docs/ko/remote-control)         | claude.ai 또는 Claude 모바일 앱에서 로컬 세션을 운전합니다. | 책상에서 멀리 있을 때 진행 중인 세션 조종              |

채널은 Claude가 아닌 소스의 이벤트를 이미 실행 중인 로컬 세션으로 푸시하여 해당 목록의 간격을 채웁니다.

* **채팅 브리지**: Telegram, Discord 또는 iMessage를 통해 휴대폰에서 Claude에 무언가를 물어보고 답변이 같은 채팅으로 돌아오는 동안 작업이 기계에서 실제 파일에 대해 실행됩니다.
* **[웹훅 수신기](/docs/ko/channels-reference#example-build-a-webhook-receiver)**: CI, 오류 추적기, 배포 파이프라인 또는 기타 외부 서비스의 웹훅이 Claude가 이미 파일을 열고 있고 디버깅 중인 것을 기억하는 곳에 도착합니다.

<h2 id="next-steps">
  다음 단계
</h2>

채널이 실행 중이면 다음 관련 기능을 살펴보세요:

* [자신의 채널 구축](/docs/ko/channels-reference) - 아직 플러그인이 없는 시스템의 경우
* [Remote Control](/docs/ko/remote-control) - 이벤트를 전달하는 대신 휴대폰에서 로컬 세션을 운전하기
* [예약된 작업](/docs/ko/scheduled-tasks) - 푸시된 이벤트에 반응하는 대신 타이머에서 폴링하기
