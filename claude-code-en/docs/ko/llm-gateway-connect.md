> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code를 LLM 게이트웨이에 연결

> 조직의 LLM 게이트웨이에 Claude Code를 연결합니다. 관리자가 이미 구성했는지 확인하거나, 기본 URL과 자격 증명을 직접 설정한 후 연결을 확인하고 게이트웨이 오류를 해결합니다.

[LLM 게이트웨이](/docs/ko/llm-gateway)는 Claude Code와 모델 제공자 사이에서 조직이 운영하는 프록시입니다. 조직에서 게이트웨이를 사용할 때, Claude Code는 개인 claude.ai 로그인 대신 조직이 발급한 자격 증명으로 게이트웨이에 인증합니다.

이 페이지는 조직이 운영하는 게이트웨이를 통해 Claude Code를 실행하는 개발자를 위한 것입니다. 두 가지 경로를 다룹니다: [관리자가 이미 구성했는지 확인](#check-for-an-existing-configuration)하기와 [관리자가 구성하지 않았을 때 직접 구성](#configure-claude-code-yourself)하기입니다.

<Note>
  * 조직을 위해 게이트웨이를 배포하려면 [LLM 게이트웨이 배포](/docs/ko/llm-gateway-rollout)를 참조하세요.
  * Claude Code가 게이트웨이에 전송하는 내용은 [게이트웨이 프로토콜 참조](/docs/ko/llm-gateway-protocol)를 참조하세요.
</Note>

<h2 id="check-for-an-existing-configuration">
  기존 구성 확인
</h2>

관리자는 [관리되는 설정](/docs/ko/settings#settings-files), 디바이스 관리 또는 [`apiKeyHelper`](#rotate-credentials-with-apikeyhelper)를 통해 게이트웨이 주소와 자격 증명을 배포할 수 있으므로, Claude Code는 시작 시 설정할 것 없이 이를 선택합니다. 조직이 이미 이를 수행했는지 확인하려면:

<Steps>
  <Step title="Claude Code 시작">
    `claude`를 실행합니다. 로그인 화면 대신 세션으로 열리면 게이트웨이 자격 증명이 배포되지 않은 것입니다. 아래에서 [직접 구성](#configure-claude-code-yourself)하세요.
  </Step>

  <Step title="상태 탭 확인">
    Claude Code가 로그인 화면을 표시하지 않고 세션을 시작했다면, `/status`를 실행하고, **상태** 탭을 열고, 두 줄을 확인합니다:

    * `Anthropic base URL`: 이 줄은 게이트웨이 주소가 설정되었을 때만 나타납니다. 없으면 Claude Code가 게이트웨이를 가리키지 않습니다. 아래에서 [직접 구성](#configure-claude-code-yourself)하세요.
    * `Auth token` 또는 `API key`: `ANTHROPIC_AUTH_TOKEN`, `ANTHROPIC_API_KEY` 또는 `apiKeyHelper`를 명시하는 줄은 게이트웨이 자격 증명이 활성화되어 있음을 확인합니다. 대신 claude.ai 계정을 명시하는 `Login method` 줄은 자격 증명이 배포되지 않았음을 의미합니다. [직접 설정](#set-the-credential-variable)하세요.
  </Step>

  <Step title="테스트 메시지 전송">
    `/status` 메뉴를 닫고 Claude Code에서 프롬프트를 전송합니다. Claude의 정상적인 응답으로 오류가 없으면 게이트웨이 연결이 작동함을 확인합니다.
  </Step>
</Steps>

`/status` 메뉴의 두 줄이 모두 올바르지만 Claude로의 메시지가 실패하면 [문제 해결 표](#troubleshoot-gateway-errors)를 참조하세요.

<h2 id="configure-claude-code-yourself">
  Claude Code 직접 구성
</h2>

게이트웨이에 대해 Claude Code를 직접 구성하려면 게이트웨이 팀에서 다음이 필요합니다:

* 게이트웨이의 기본 URL
* 자격 증명: 키 또는 토큰 문자열, 또는 하나를 가져오는 명령
  * 게이트웨이 팀이 어떤 종류의 자격 증명인지 말하지 않았다면, 아래의 [자격 증명 변수 섹션](#set-the-credential-variable)에서 시도할 것을 다룹니다.

아래 섹션은 순서대로 구성을 다룹니다:

* [자격 증명 변수 설정](#set-the-credential-variable) 및 [기본 URL 설정](#set-the-base-url-and-credential): 모든 게이트웨이 연결에 필요한 두 변수
* [연결 확인](#verify-the-connection): 무엇이든 유지하기 전에 작동하는지 확인
* [각 표면 구성](#configure-each-surface): Claude Code CLI 외에 VS Code와 같은 다른 표면을 사용하는 경우, 게이트웨이 자격 증명으로 구성하는 방법을 참조하세요.
* [추가 구성](#additional-configuration): 기본 URL과 자격 증명 외에 일부 게이트웨이가 필요로 하는 변수(예: 사용자 정의 헤더, 자격 증명 도우미, 모델 검색, 제공자 형식 기본 URL 또는 게이트웨이 경로 외부의 트래픽 끄기). 관리자가 명시한 경우에만 이를 설정하거나 네트워크가 송신을 제한합니다.

<h3 id="set-the-credential-variable">
  자격 증명 변수 설정
</h3>

Claude Code를 게이트웨이에 인증하려면 환경 변수에 자격 증명을 설정합니다. 어떤 변수를 사용할지는 게이트웨이 팀이 말한 것에 따라 다릅니다:

| 자격 증명 설정 위치                                             | 사용 시기                                                     |
| :------------------------------------------------------ | :-------------------------------------------------------- |
| `ANTHROPIC_AUTH_TOKEN`                                  | 게이트웨이 팀이 "bearer token" 또는 "Authorization header"라고 말했을 때 |
| `ANTHROPIC_API_KEY`                                     | 게이트웨이 팀이 "API key" 또는 "x-api-key"라고 말했을 때                 |
| [`apiKeyHelper`](#rotate-credentials-with-apikeyhelper) | 자격 증명이 회전하거나 자격 증명 모음에서 나올 때                              |

어떤 종류인지 말하지 않았다면 `ANTHROPIC_AUTH_TOKEN`을 사용하세요. 아래의 [확인 요청](#verify-the-connection)은 전환이 필요한지 확인하는 방법을 보여줍니다.

<h3 id="set-the-base-url-and-credential">
  기본 URL과 자격 증명 설정
</h3>

게이트웨이의 기본 URL과 위에서 선택한 자격 증명 변수를 환경 변수로 설정합니다. 예제는 `ANTHROPIC_AUTH_TOKEN`을 사용합니다. [선택한 변수](#set-the-credential-variable)가 `ANTHROPIC_API_KEY`라면 이를 바꾸세요. [셸에서](#set-as-shell-environment-variables) 설정할 수 있으며, 이는 한 터미널 세션 동안 지속되거나, [Claude Code 설정 파일에서](#set-in-a-settings-file) 설정할 수 있으며, 이는 Claude Code가 실행되는 모든 곳에서 지속됩니다.

첫 번째 연결의 경우, 셸 내보내기로 시작하고 값을 설정 파일로 이동하기 전에 [확인 요청](#verify-the-connection)을 실행합니다.

<h4 id="set-as-shell-environment-variables">
  셸 환경 변수로 설정
</h4>

값을 게이트웨이 팀이 제공한 값으로 바꾸세요:

<Tabs>
  <Tab title="Bash 또는 Zsh">
    ```bash theme={null}
    export ANTHROPIC_BASE_URL=https://llm-gateway.example.com
    export ANTHROPIC_AUTH_TOKEN=sk-gateway-key
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_BASE_URL = "https://llm-gateway.example.com"
    $env:ANTHROPIC_AUTH_TOKEN = "sk-gateway-key"
    ```
  </Tab>
</Tabs>

셸 내보내기는 해당 터미널 세션과 그로부터 시작된 프로그램에만 적용됩니다. 도크 또는 시작 메뉴에서 시작된 편집기는 이를 볼 수 없습니다. 새 터미널에서 지속되도록 하려면 `~/.zshrc`, `~/.bashrc` 또는 PowerShell `$PROFILE`과 같은 셸 프로필에 동일한 줄을 추가하거나 대신 설정 파일을 사용합니다.

<h4 id="set-in-a-settings-file">
  설정 파일에서 설정
</h4>

Claude Code가 실행되는 모든 곳에서 구성이 적용되도록 하고 셸에 의존하지 않으려면, [설정 파일](/docs/ko/settings)의 `env` 블록에서 변수를 설정합니다. 설정 파일은 다양한 범위를 가집니다:

* `~/.claude/settings.json`은 모든 프로젝트에 적용됩니다. Windows에서 경로는 `%USERPROFILE%\.claude\settings.json`입니다.
* `.claude/settings.local.json`은 한 프로젝트에 적용됩니다. Claude Code는 파일을 생성할 때 이를 gitignore에 추가합니다. 직접 생성하는 경우, 자격 증명을 실수로 커밋하지 않도록 먼저 gitignore에 수동으로 추가합니다.

<Warning>
  프로젝트의 `.claude/settings.json`에 자격 증명을 넣지 마세요. 이 파일은 커밋되고 저장소를 복제하는 모든 사람과 공유됩니다.
</Warning>

`env` 블록은 두 파일 모두에서 동일하게 보입니다:

```json theme={null}
{
  "env": {
    "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com",
    "ANTHROPIC_AUTH_TOKEN": "sk-gateway-key"
  }
}
```

셸 내보내기와 설정 파일 `env` 블록이 동일한 변수를 설정할 때, 설정 파일 값이 적용됩니다. `/status`를 실행하여 Claude Code가 사용 중인 기본 URL과 자격 증명 소스를 확인합니다.

<h3 id="verify-the-connection">
  연결 확인
</h3>

셸에서 변수를 내보낸 상태에서, 게이트웨이에 한 토큰 요청을 직접 전송합니다. 이는 Claude Code를 열기 전에 URL과 자격 증명이 작동하는지 확인하므로, 실패는 구성이 아닌 게이트웨이를 가리킵니다. 아래 명령은 셸 변수를 읽으므로, 설정 파일에도 값을 넣었더라도 [셸 내보내기](#set-as-shell-environment-variables)가 필요합니다.

<Tabs>
  <Tab title="Bash 또는 Zsh">
    ```bash theme={null}
    curl -X POST "$ANTHROPIC_BASE_URL/v1/messages" \
      -H "Authorization: Bearer $ANTHROPIC_AUTH_TOKEN" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    Invoke-RestMethod -Method Post -Uri "$env:ANTHROPIC_BASE_URL/v1/messages" `
      -Headers @{ "Authorization" = "Bearer $env:ANTHROPIC_AUTH_TOKEN"; "anthropic-version" = "2023-06-01" } `
      -ContentType "application/json" `
      -Body '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>
</Tabs>

게이트웨이가 `x-api-key` 헤더의 키를 예상하면, Bash 명령에서 `Authorization` 헤더를 `x-api-key: $ANTHROPIC_API_KEY`로 바꾸거나, PowerShell 명령에서 `"Authorization"` 해시테이블 항목을 `"x-api-key" = "$env:ANTHROPIC_API_KEY"`로 바꾸세요.

`{"id":"msg_`로 시작하고 `"content":[...]` 필드를 포함하는 JSON 응답은 게이트웨이에 도달할 수 있고 자격 증명이 작동함을 의미합니다. 알 수 없는 모델을 명시하는 오류는 게이트웨이가 모델 이름을 거부하기 전에 요청을 인증했으므로 URL과 자격 증명이 작동함을 증명합니다. 게이트웨이가 제공하는 모델을 찾을 필요가 없습니다. `401`은 자격 증명이 거부되었음을 의미합니다. 변수를 추측했다면 다른 변수로 전환하고 다시 내보내세요.

<h4 id="confirm-in-claude-code">
  Claude Code에서 확인
</h4>

같은 셸에서 `claude`를 시작하여 내보내기를 상속받고, 메시지를 전송하고, `/status`를 실행합니다.

**상태** 탭에서 `Anthropic base URL` 줄은 게이트웨이 주소를 표시해야 하며, 이는 요청이 그곳으로 라우팅되고 있음을 확인합니다. 줄이 없으면 변수가 세션에 도달하지 않았습니다. `Auth token` 또는 `API key` 줄이 설정한 변수를 명시하면 저장된 claude.ai 로그인이 아닌 게이트웨이 자격 증명이 활성화되어 있음을 확인합니다.

메시지가 실패하거나 `/status`가 게이트웨이 URL을 표시하지 않으면, 아래의 [문제 해결 표](#troubleshoot-gateway-errors)를 참조하세요.

<h3 id="how-the-credential-variable-maps-to-a-header">
  자격 증명 변수가 헤더에 매핑되는 방식
</h3>

각 변수는 자격 증명을 다른 HTTP 헤더로 전송합니다: `ANTHROPIC_AUTH_TOKEN`은 `Authorization: Bearer`로, `ANTHROPIC_API_KEY`는 `x-api-key`로, `apiKeyHelper`는 둘 다로 전송합니다. 잘못된 변수의 자격 증명은 게이트웨이가 읽지 않는 헤더에 도달하고, 요청은 `401`로 실패합니다. 확인 요청이 `401`을 반환했다면 다른 변수로 전환하고 다시 시도하세요.

<h3 id="conflicts-with-an-existing-login">
  기존 로그인과의 충돌
</h3>

게이트웨이 자격 증명 변수는 저장된 claude.ai 로그인 또는 Console 키보다 우선합니다. claude.ai 로그인은 변수가 설정된 동안 저장되고 사용되지 않습니다. 변수를 설정 해제하면 Claude Code는 이로 돌아갑니다. `ANTHROPIC_AUTH_TOKEN`을 사용하면 변수가 즉시 우선합니다. `ANTHROPIC_API_KEY`를 사용하면 대화형 모드에서 키가 인수하기 전에 한 번 승인하도록 요청받습니다.

`/status`를 실행하여 어떤 자격 증명 소스가 활성화되어 있는지 확인합니다. 시작 시 두 소스를 명시하는 인증 충돌 경고가 표시되면, [문제 해결 표](#troubleshoot-gateway-errors)의 첫 번째 행을 참조하여 어느 것을 제거할지 확인합니다. 저장된 로그인을 지워 게이트웨이 자격 증명만 남기려면 `/logout`을 실행합니다.

<h2 id="configure-each-surface">
  각 표면 구성
</h2>

CLI는 위의 환경 변수와 설정 파일을 읽습니다. 다른 표면은 VS Code 확장, 데스크톱 앱, GitHub Actions, Agent SDK 및 Slack과 웹과 같은 클라우드 표면입니다. 아래 섹션은 이러한 설정이 각 표면에 도달하는지 여부를 다룹니다.

<h3 id="vs-code-extension">
  VS Code 확장
</h3>

[VS Code 확장](/docs/ko/vs-code)에 대해 게이트웨이 변수를 VS Code의 **기본 설정: 사용자 설정 열기(JSON)** 명령으로 열린 VS Code 자체 사용자 설정의 `claudeCode.environmentVariables`에서 설정합니다. 확장은 시작 전에 이 설정에서 자격 증명을 확인하므로, 게이트웨이 자격 증명을 위한 신뢰할 수 있는 위치입니다. `~/.claude/settings.json`의 값은 생성된 프로세스에 도달하지만 확장의 자체 로그인 확인에는 도달하지 않습니다.

```json theme={null}
{
  "claudeCode.environmentVariables": [
    { "name": "ANTHROPIC_BASE_URL", "value": "https://llm-gateway.example.com" },
    { "name": "ANTHROPIC_AUTH_TOKEN", "value": "sk-gateway-key" }
  ]
}
```

<h3 id="desktop-app">
  데스크톱 앱
</h3>

데스크톱 앱은 `ANTHROPIC_BASE_URL` 또는 `settings.json`이 아닌 [타사 추론 구성](https://claude.com/docs/third-party/claude-desktop/gateway)에서 게이트웨이 라우팅을 읽습니다. 해당 구성은 조직에서 제공하거나 앱 자체의 양식에서 제공될 수 있습니다:

* **관리자가 배포함**: 조직이 [구성을 배포](/docs/ko/llm-gateway-rollout#distribute-through-managed-settings)한 경우, 데스크톱 앱은 설정 없이 게이트웨이를 통해 라우팅합니다
* **로컬로 구성됨**: 관리자가 배포한 구성이 없는 기기의 경우, 도움말 → 문제 해결 → 개발자 모드 활성화를 열면 앱이 개발자 메뉴와 함께 다시 시작됩니다. 그런 다음 개발자 → 타사 추론 구성 및 게이트웨이 기본 URL을 입력합니다. 관리자가 배포한 구성이 우선하며 이 양식을 읽기 전용으로 만듭니다

게이트웨이 구성이 활성화되면, 데스크톱 앱은 로컬 머신에서만 세션을 실행합니다: 환경 선택기는 SSH 세션이나 Anthropic 호스팅 클라우드 환경을 제공하지 않으며, [Remote Control](/docs/ko/remote-control)은 사용할 수 없습니다. 게이트웨이를 통해 원격 호스트에서 Claude Code를 사용하려면, [`ANTHROPIC_BASE_URL` 및 게이트웨이 자격 증명](#set-the-base-url-and-credential)이 설정된 해당 호스트에서 CLI를 실행합니다.

데스크톱 앱이 `Gateway was unreachable`을 표시하면, 앱이 시작 시 구성된 기본 URL에 도달할 수 없었습니다. 위의 [curl 테스트](#verify-the-connection)로 URL과 네트워크 경로를 확인합니다.

<h3 id="github-actions">
  GitHub Actions
</h3>

[Claude Code GitHub Actions](/docs/ko/github-actions)는 워크플로우의 `env` 블록에서 `ANTHROPIC_BASE_URL`과 `ANTHROPIC_CUSTOM_HEADERS`를 읽습니다. 자격 증명을 작업의 `anthropic_api_key` 입력으로 전달합니다. 작업은 이를 `ANTHROPIC_API_KEY`로 설정하므로, `x-api-key` 헤더의 게이트웨이에 도달합니다.

`x-api-key` 게이트웨이의 경우, `env`에서 기본 URL을 설정하고 게이트웨이 키를 입력으로 전달합니다:

```yaml theme={null}
env:
  ANTHROPIC_BASE_URL: https://llm-gateway.example.com

steps:
  - uses: anthropics/claude-code-action@v1
    with:
      anthropic_api_key: ${{ secrets.GATEWAY_API_KEY }}
```

bearer 토큰 게이트웨이의 경우, 동일한 시크릿을 `anthropic_api_key` 입력과 워크플로우 `env` 블록의 `ANTHROPIC_AUTH_TOKEN` 모두로 전달합니다. 작업은 Claude Code를 시작하기 전에 `anthropic_api_key`, `CLAUDE_CODE_OAUTH_TOKEN` 또는 워크로드 ID 페더레이션이 필요하고, `ANTHROPIC_AUTH_TOKEN`을 읽지 않으므로, 입력은 해당 시작 확인을 만족하는 동안 env 변수는 게이트웨이가 읽는 `Authorization` 헤더에 키를 넣습니다. `x-api-key`의 복사본은 무시됩니다:

```yaml theme={null}
env:
  ANTHROPIC_BASE_URL: https://llm-gateway.example.com
  ANTHROPIC_AUTH_TOKEN: ${{ secrets.GATEWAY_API_KEY }}

steps:
  - uses: anthropics/claude-code-action@v1
    with:
      anthropic_api_key: ${{ secrets.GATEWAY_API_KEY }}
```

작업의 다른 인증 옵션(예: `CLAUDE_CODE_OAUTH_TOKEN` 및 워크로드 ID 페더레이션)은 [Claude Code GitHub Actions](/docs/ko/github-actions) 및 작업의 [README](https://github.com/anthropics/claude-code-action#readme)를 참조하세요.

<h3 id="agent-sdk">
  Agent SDK
</h3>

[Agent SDK](/docs/ko/agent-sdk/overview)는 게이트웨이 관련 옵션이 없습니다. 생성하는 Claude Code 프로세스에 환경 변수를 전달합니다. 각 SDK는 생성된 프로세스의 환경을 설정하는 `env` 옵션을 허용하고, TypeScript 및 Python SDK는 이를 다르게 처리합니다:

* TypeScript: 생성된 프로세스는 기본적으로 부모 환경을 상속하지만, `options.env`를 설정하면 환경을 완전히 바꿉니다. 게이트웨이 변수를 유지하려면 `process.env`를 이에 펼치세요.
* Python: `ClaudeAgentOptions(env=...)`는 상속된 환경 위에 병합되므로, 부모 프로세스에서 설정된 게이트웨이 변수는 펼칠 필요 없이 통과합니다.

<CodeGroup>
  ```ts TypeScript theme={null}
  const result = query({
    prompt: "...",
    options: {
      env: {
        ...process.env,
        ANTHROPIC_BASE_URL: "https://llm-gateway.example.com",
        ANTHROPIC_AUTH_TOKEN: process.env.GATEWAY_KEY,
      },
    },
  })
  ```

  ```python Python theme={null}
  options = ClaudeAgentOptions(
      env={
          "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com",
          "ANTHROPIC_AUTH_TOKEN": os.environ["GATEWAY_KEY"],
      }
  )
  ```
</CodeGroup>

<h3 id="slack-web-and-remote-control">
  Slack, 웹 및 Remote Control
</h3>

[Slack의 Claude Code](/docs/ko/slack) 및 [웹의 Claude Code](/docs/ko/claude-code-on-the-web)는 항상 Anthropic의 API를 사용하는 Anthropic 호스팅 제품입니다. 게이트웨이 배포의 일부가 아닙니다. 클라우드 세션의 환경 구성에서 설정된 게이트웨이 변수는 적용되지 않습니다. 트래픽이 게이트웨이에 남아 있어야 한다면, 이러한 사용자에 대해 이러한 표면을 활성화하지 마세요.

[Remote Control](/docs/ko/remote-control) 및 [음성 받아쓰기](/docs/ko/voice-dictation)는 모두 claude.ai 신원에 의존합니다: Remote Control은 라이브 세션을 계정과 쌍으로 만들고, 음성 받아쓰기는 claude.ai 전사 엔드포인트에 도달합니다. `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` 또는 `apiKeyHelper`가 활성화되어 있는 동안은 사용할 수 없습니다. v2.1.196부터 Remote Control은 `ANTHROPIC_BASE_URL`이 Anthropic이 아닌 호스트를 가리킬 때도 비활성화되므로, claude.ai로 로그인하는 것만으로는 충분하지 않습니다.

기능을 복원하려면 claude.ai로 로그인하고 확인하는 게이트웨이 변수를 설정 해제합니다. Remote Control 섹션의 `claude doctor`는 설정 해제할 자격 증명 변수를 명시합니다.

* 음성 받아쓰기: 게이트웨이 자격 증명 설정 해제
* Remote Control: 게이트웨이 자격 증명 및 `ANTHROPIC_BASE_URL` 설정 해제

<h2 id="additional-configuration">
  추가 구성
</h2>

이러한 설정은 기본 URL과 자격 증명 이상의 경우를 다룹니다. 관리자의 지시 사항, 네트워크의 송신 규칙 또는 [문제 해결 표](#troubleshoot-gateway-errors)가 하나를 요청할 때만 설정합니다.

<h3 id="send-additional-headers">
  추가 헤더 전송
</h3>

일부 게이트웨이는 자격 증명 외에 사용자 정의 헤더(예: 테넌트 식별자 또는 라우팅 키)를 사용하여 요청을 라우팅하거나 태그합니다. 하나를 전송하려면 [`ANTHROPIC_CUSTOM_HEADERS`](/docs/ko/env-vars)를 한 줄에 한 `Name: Value` 쌍으로 설정합니다. 아래 예제는 `X-Org-Route`라는 라우팅 헤더를 추가합니다:

<Tabs>
  <Tab title="Bash 또는 Zsh">
    ```bash theme={null}
    export ANTHROPIC_CUSTOM_HEADERS="X-Org-Route: prod"
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_CUSTOM_HEADERS = "X-Org-Route: prod"
    ```
  </Tab>
</Tabs>

설정 파일의 `env` 블록에서도 `ANTHROPIC_CUSTOM_HEADERS`를 설정할 수 있습니다. JSON 문자열이 여러 줄에 걸칠 수 없으므로 쌍 사이에 `\n`을 사용합니다:

```json theme={null}
{
  "env": {
    "ANTHROPIC_CUSTOM_HEADERS": "X-Org-Route: prod\nX-Tenant: example"
  }
}
```

<h3 id="add-gateway-models-to-the-model-picker">
  게이트웨이 모델을 모델 선택기에 추가
</h3>

모델 검색은 시작 시 게이트웨이에 모델 목록을 쿼리하고 이러한 이름을 기본 제공 항목과 함께 `/model` 선택기에 추가합니다.

게이트웨이가 Claude Code의 기본 제공 목록에 없는 모델 이름을 제공하고 선택기에서 선택하려면 활성화합니다. 기본 제공 모델이 사용하는 것이라면 검색이 필요하지 않습니다. 관리자가 관리되는 설정을 통해 이미 활성화했을 수도 있습니다.

활성화하려면 셸에서 또는 `~/.claude/settings.json`의 `env` 블록에서 `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1`을 설정합니다. 검색에는 Claude Code v2.1.129 이상이 필요합니다.&#x20;

검색된 모델은 `From gateway`로 표시된 추가 `/model` 항목으로 나타납니다. 검색이 실행되었는지 확인하려면 `claude --debug`를 시작하고 `[gatewayDiscovery]` 줄을 찾습니다. 성공은 캐시된 모델 수를 기록하고, `404`, 시간 초과 또는 리디렉션은 거기에도 기록됩니다. 검색이 실행되는 시기, 필터링하는 것, 게이트웨이가 제공하는 응답 형식은 [모델 검색 참조](/docs/ko/llm-gateway-protocol#model-discovery)를 참조하세요.

<h3 id="rotate-credentials-with-apikeyhelper">
  apiKeyHelper로 자격 증명 회전
</h3>

`apiKeyHelper`는 정적 환경 변수에서 읽는 대신 게이트웨이 자격 증명을 가져오기 위해 Claude Code가 실행하는 명령입니다.

자격 증명이 일정에 따라 만료되거나, 자격 증명 모음 또는 SSO 명령에서 나오거나, 관리자가 하나를 구성하도록 말했을 때 도우미를 사용합니다. 자격 증명이 한 번 설정하는 고정 문자열이라면, [자격 증명 변수](#set-the-credential-variable)만 필요하고 이 섹션을 건너뛸 수 있습니다.

도우미는 현재 자격 증명을 stdout으로 인쇄하는 모든 셸 명령입니다. Claude Code는 이를 시스템 셸을 통해 실행하므로, Windows에서는 실행 파일 또는 PowerShell 호출이 될 수 있습니다. 스크립트를 작성하고, 실행 가능하게 만들고, [설정 파일](/docs/ko/settings)의 `apiKeyHelper`에서 참조합니다:

<Tabs>
  <Tab title="Bash 또는 Zsh">
    예를 들어, 자격 증명 모음에서 읽는 스크립트:

    ```bash theme={null}
    #!/bin/bash
    vault kv get -field=api_key secret/llm-gateway/claude-code
    ```

    `~/.claude/settings.json`에서 경로를 참조합니다:

    ```json theme={null}
    {
      "apiKeyHelper": "~/bin/get-gateway-key.sh"
    }
    ```
  </Tab>

  <Tab title="PowerShell">
    예를 들어, 자격 증명 모음에서 읽는 스크립트:

    ```powershell theme={null}
    vault kv get -field=api_key secret/llm-gateway/claude-code
    ```

    `%USERPROFILE%\.claude\settings.json`에서 PowerShell 호출을 참조하고, JSON 문자열의 백슬래시를 이스케이프합니다:

    ```json theme={null}
    {
      "apiKeyHelper": "powershell -NoProfile -File C:\\scripts\\get-gateway-key.ps1"
    }
    ```
  </Tab>
</Tabs>

Claude Code는 기본적으로 도우미의 출력을 5분 동안 캐시하고 요청이 HTTP 401을 반환할 때 다시 실행합니다. 캐시 수명을 변경하려면 `CLAUDE_CODE_API_KEY_HELPER_TTL_MS`를 밀리초 단위로 설정합니다. 예를 들어 15분의 경우 `CLAUDE_CODE_API_KEY_HELPER_TTL_MS=900000`입니다.

도우미의 값은 `Authorization` 및 `x-api-key` 헤더 모두에 전송되므로, 게이트웨이가 어느 헤더를 읽든 작동합니다.

<h3 id="turn-off-traffic-outside-the-gateway-path">
  게이트웨이 경로 외부의 트래픽 끄기
</h3>

게이트웨이는 모델 요청을 전달하지만, Claude Code는 또한 게이트웨이 경로 외부로 Anthropic 및 GitHub와 같은 타사 서비스로 불필수 백그라운드 트래픽을 전송합니다: 버전 확인, 원격 분석, 오류 보고, 릴리스 노트 및 유사한 요청입니다. 게이트웨이로의 송신만 허용하는 네트워크에서 이러한 요청은 실패하고 송신 모니터링에서 차단된 연결로 나타날 수 있습니다.

해당 트래픽을 끄려면 게이트웨이 변수와 함께 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1`을 설정합니다. 동일한 셸 내보내기 또는 설정 파일 `env` 블록에서:

<Tabs>
  <Tab title="Bash 또는 Zsh">
    ```bash theme={null}
    export CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC = "1"
    ```
  </Tab>
</Tabs>

변수를 설정하면 다음과 같은 효과와 제한이 있습니다:

* 자동 업데이트를 비활성화하므로 패키지 관리자 또는 관리되는 배포와 같은 다른 업데이트 경로를 계획합니다.
* [빠른 모드](/docs/ko/fast-mode) 가용성 확인을 억제합니다. 이전 확인이 이미 머신에서 빠른 모드를 활성화하지 않은 한, `/fast`는 빠른 모드를 사용할 수 없다고 보고합니다.
* [게이트웨이 모델 검색](#add-gateway-models-to-the-model-picker)을 끕니다. 검색이 게이트웨이 자체를 쿼리하더라도 말입니다. 이전에 검색된 모델은 로컬 캐시에서 사용 가능하게 유지되지만 목록은 새로 고쳐지지 않습니다.
* WebFetch 도구의 [도메인 안전 확인](/docs/ko/data-usage#webfetch-domain-safety-check)은 영향을 받지 않으며 여전히 `api.anthropic.com`을 호출합니다. 네트워크가 해당 호스트를 차단하는 경우 [설정](/docs/ko/settings)에서 `skipWebFetchPreflight: true`로 별도로 끕니다.
* 각 원격 분석 스트림 및 이를 제어하는 변수는 [원격 분석 서비스](/docs/ko/data-usage#telemetry-services)를 참조하세요.

<h3 id="route-to-a-cloud-provider-through-a-gateway">
  게이트웨이를 통해 클라우드 제공자로 라우팅
</h3>

이러한 구성은 Claude Code를 `ANTHROPIC_BASE_URL` 대신 제공자 관련 기본 URL 변수를 통해 게이트웨이로 가리킵니다. Amazon Bedrock 및 Google Cloud의 Agent Platform 게이트웨이는 이러한 제공자의 기본 요청 형식을 허용합니다. Microsoft Foundry 및 AWS의 Claude Platform 게이트웨이는 Anthropic Messages 형식을 허용하고 어느 기본 URL 변수가 이에 도달하는지에서만 다릅니다.

게이트웨이 팀이 Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry 또는 AWS의 Claude Platform을 구체적으로 명시한 경우에만 하나를 사용합니다. 위의 [확인 요청](#verify-the-connection)이 JSON을 반환했다면 이 섹션을 건너뛸 수 있습니다.

게이트웨이 팀이 명시한 제공자에 대한 블록을 설정합니다. skip-auth 변수는 게이트웨이가 이를 보유하므로 Claude Code가 제공자 자격 증명으로 요청에 서명하지 않도록 합니다. 게이트웨이가 자체 토큰이 필요하면, 블록 후에 `ANTHROPIC_AUTH_TOKEN`을 추가합니다. Microsoft Foundry는 표시된 대로 `ANTHROPIC_FOUNDRY_API_KEY`를 사용합니다. Bearer 토큰을 예상하는 Microsoft Foundry 게이트웨이는 [`ANTHROPIC_FOUNDRY_AUTH_TOKEN`](/docs/ko/env-vars)을 대신 사용할 수 있습니다. 둘 다 설정된 경우 `ANTHROPIC_FOUNDRY_AUTH_TOKEN`이 `ANTHROPIC_FOUNDRY_API_KEY`보다 우선합니다. `ANTHROPIC_FOUNDRY_AUTH_TOKEN`에는 Claude Code v2.1.203 이상이 필요합니다.

<h4 id="amazon-bedrock">
  Amazon Bedrock
</h4>

<Tabs>
  <Tab title="Bash 또는 Zsh">
    ```bash theme={null}
    export ANTHROPIC_BEDROCK_BASE_URL=https://llm-gateway.example.com/bedrock
    export CLAUDE_CODE_SKIP_BEDROCK_AUTH=1
    export CLAUDE_CODE_USE_BEDROCK=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_BEDROCK_BASE_URL = "https://llm-gateway.example.com/bedrock"
    $env:CLAUDE_CODE_SKIP_BEDROCK_AUTH = "1"
    $env:CLAUDE_CODE_USE_BEDROCK = "1"
    ```
  </Tab>
</Tabs>

<h4 id="google-cloud’s-agent-platform">
  Google Cloud의 Agent Platform
</h4>

<Tabs>
  <Tab title="Bash 또는 Zsh">
    ```bash theme={null}
    export ANTHROPIC_VERTEX_BASE_URL=https://llm-gateway.example.com/vertex
    export ANTHROPIC_VERTEX_PROJECT_ID=your-gcp-project-id
    export CLAUDE_CODE_SKIP_VERTEX_AUTH=1
    export CLAUDE_CODE_USE_VERTEX=1
    export CLOUD_ML_REGION=us-east5
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_VERTEX_BASE_URL = "https://llm-gateway.example.com/vertex"
    $env:ANTHROPIC_VERTEX_PROJECT_ID = "your-gcp-project-id"
    $env:CLAUDE_CODE_SKIP_VERTEX_AUTH = "1"
    $env:CLAUDE_CODE_USE_VERTEX = "1"
    $env:CLOUD_ML_REGION = "us-east5"
    ```
  </Tab>
</Tabs>

<h4 id="microsoft-foundry">
  Microsoft Foundry
</h4>

게이트웨이의 자격 증명을 `ANTHROPIC_FOUNDRY_API_KEY`에 넣습니다. `x-api-key` 헤더로 게이트웨이에 전송됩니다. Bearer 토큰을 예상하는 게이트웨이는 [`ANTHROPIC_FOUNDRY_AUTH_TOKEN`](/docs/ko/env-vars)을 대신 사용할 수 있습니다. Claude Code는 해당 값을 `Authorization: Bearer` 헤더로 전송하며, 둘 다 설정된 경우 `ANTHROPIC_FOUNDRY_API_KEY`보다 우선합니다. Claude Code v2.1.203 이상이 필요합니다.

자체 `Authorization` 헤더를 주입하는 게이트웨이의 경우, `CLAUDE_CODE_SKIP_FOUNDRY_AUTH=1`을 설정하고 두 자격 증명 변수를 모두 설정하지 않은 상태로 둡니다. Claude Code는 Azure 자격 증명 없이 요청을 전송하고 `ANTHROPIC_CUSTOM_HEADERS`를 통해 제공하는 것과 같이 사용자가 제공하는 `Authorization` 헤더를 보존합니다. v2.1.203 이전에는 API 키 없이 `CLAUDE_CODE_SKIP_FOUNDRY_AUTH`를 사용하면 Microsoft Foundry 클라이언트가 요청을 전송할 수 없었습니다.

<Tabs>
  <Tab title="Bash 또는 Zsh">
    ```bash theme={null}
    export ANTHROPIC_FOUNDRY_BASE_URL=https://llm-gateway.example.com/foundry
    export ANTHROPIC_FOUNDRY_API_KEY=sk-gateway-key
    export CLAUDE_CODE_USE_FOUNDRY=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_FOUNDRY_BASE_URL = "https://llm-gateway.example.com/foundry"
    $env:ANTHROPIC_FOUNDRY_API_KEY = "sk-gateway-key"
    $env:CLAUDE_CODE_USE_FOUNDRY = "1"
    ```
  </Tab>
</Tabs>

<h4 id="claude-platform-on-aws">
  AWS의 Claude Platform
</h4>

워크스페이스 ID는 [AWS의 Claude Platform](/docs/ko/claude-platform-on-aws)을 참조하세요.

<Tabs>
  <Tab title="Bash 또는 Zsh">
    ```bash theme={null}
    export ANTHROPIC_AWS_BASE_URL=https://llm-gateway.example.com/anthropic-aws
    export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
    export CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1
    export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_AWS_BASE_URL = "https://llm-gateway.example.com/anthropic-aws"
    $env:ANTHROPIC_AWS_WORKSPACE_ID = "wrkspc_01ABCDEFGHIJKLMN"
    $env:CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH = "1"
    $env:CLAUDE_CODE_USE_ANTHROPIC_AWS = "1"
    ```
  </Tab>
</Tabs>

<h2 id="troubleshoot-gateway-errors">
  게이트웨이 오류 문제 해결
</h2>

Claude Code를 게이트웨이를 통해 실행할 때 가장 일반적인 오류와 게이트웨이 측 원인 및 해결 방법입니다:

| 오류                                                                                                                                                     | 원인                                                                                                                                                                            | 해결 방법                                                                                                                                                                                                                                                               |
| :----------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 두 자격 증명 소스를 명시하고 `auth may not work as expected`로 끝나는 시작 경고. 이전 버전은 `Auth conflict: Both a token (SOURCE) and an API key (SOURCE) are set` 대신 표시합니다.   | 게이트웨이 자격 증명과 저장된 로그인이 모두 활성화되어 있습니다. 변수는 요청에 사용되지만, 오래된 로그인은 예상치 못한 인증 동작을 유발할 수 있습니다.                                                                                        | 변수를 설정 해제하여 저장된 로그인을 사용하거나, `/logout`을 실행하여 게이트웨이 자격 증명을 사용합니다.                                                                                                                                                                                                     |
| 잘못되거나 인식되지 않는 토큰을 명시하는 `401` 오류                                                                                                                        | 자격 증명이 게이트웨이가 발급한 것이 아니거나, 게이트웨이가 읽지 않는 헤더에 있습니다.                                                                                                                             | 변수가 [자격 증명 표](#set-the-credential-variable)의 자격 증명 종류와 일치하는지 확인하고, 게이트웨이에서 키가 취소된 경우 다시 생성합니다.                                                                                                                                                                      |
| `Your apiKeyHelper script is failing`                                                                                                                  | [`apiKeyHelper`](/docs/ko/settings#available-settings) 설정의 명령이 오류로 종료되었거나, 시간 초과되었거나, 아무것도 출력하지 않아서 요청이 자리 표시자 키를 전달합니다.                                                           | 명령을 직접 실행하여 실패 이유를 확인하고, 만료된 세션을 보고하면 자격 증명 제공자로 다시 인증합니다. [오류 참조](/docs/ko/errors#your-apikeyhelper-script-is-failing)를 참조하세요.                                                                                                                                          |
| `Unable to connect to API (ConnectionRefused)` 또는 npm 설치에서 `(ECONNREFUSED)`, 종종 Claude Code가 [백오프로 재시도](/docs/ko/errors#automatic-retries)하는 동안 조용한 일시 중지 후 | 기본 URL에서 아무것도 응답하지 않았습니다. 주소가 잘못되었거나 VPN 또는 방화벽이 게이트웨이로의 경로를 차단합니다.                                                                                                           | 위의 [curl 테스트](#verify-the-connection)를 실행합니다. 동일한 원인으로 즉시 실패하고, 게이트웨이 팀과 URL 및 네트워크 경로를 확인합니다.                                                                                                                                                                      |
| `API returned an empty or malformed response (HTTP 200)`                                                                                               | 게이트웨이 또는 중간 프록시가 비 API 응답(종종 HTML 오류 또는 로그인 페이지)을 반환했습니다.                                                                                                                     | 위의 [curl 요청](#verify-the-connection)으로 테스트합니다. 비 JSON을 반환하는 게이트웨이 경로를 수정합니다.                                                                                                                                                                                        |
| `context_management`, `Extra inputs are not permitted` 또는 기타 인식되지 않는 필드를 명시하는 `400` 오류                                                                 | 게이트웨이는 요청을 Anthropic 형식 엔드포인트로 전달하는 업스트림으로 전달하고, Claude Code가 Anthropic에 전송하는 필드를 거부합니다.                                                                                      | `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`을 설정합니다. 이는 대부분의 사전 릴리스 필드를 억제합니다. [기능 통과](/docs/ko/llm-gateway-protocol#feature-pass-through)를 참조하세요. 일부 베타는 이 플래그로 제어되지 않습니다. 이들의 경우, 일치하는 `CLAUDE_CODE_USE_*` 제공자 변수를 설정하여 Claude Code가 해당 제공자가 허용하는 것만 전송하도록 합니다.          |
| `Input tag 'adaptive' found`와 같은 `thinking` 또는 `adaptive`를 명시하는 `400` 오류                                                                               | 업스트림 모델 빌드는 Claude Code가 Claude 4.6 이상 모델에 대해 요청하는 적응형 추론을 허용하지 않습니다.                                                                                                         | 게이트웨이의 업스트림을 업그레이드합니다. Opus 4.6 및 Sonnet 4.6에서는 `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1`이 대신 작동합니다. [모델 구성](/docs/ko/model-config) 기능 변수는 `CLAUDE_CODE_USE_BEDROCK` 및 `CLAUDE_CODE_USE_VERTEX`와 같은 제공자 구성에만 적용되며, `ANTHROPIC_BASE_URL` 게이트웨이 뒤에는 적용되지 않습니다.             |
| 게이트웨이 자체 단어로 컨텍스트 또는 토큰 제한을 명시하는 `400` 오류(예: `ContextWindowExceededError` 또는 `prompt token count of N exceeds the limit of M`)                         | 게이트웨이는 모델의 기본 윈도우보다 작은 컨텍스트를 적용하고 업스트림 오류를 다시 작성하므로, Anthropic의 `prompt is too long` 단어와 일치하는 자동 compact-and-retry가 실행되지 않습니다.                                                | `/compact`를 실행하여 세션을 복구합니다. 이를 방지하려면 `CLAUDE_CODE_AUTO_COMPACT_WINDOW`를 게이트웨이의 제한으로 설정합니다. 값은 최소 100,000 토큰 이상, 최대 모델의 컨텍스트 윈도우 이하로 제한되므로, 100,000 미만의 게이트웨이 제한은 일치할 수 없고 `/compact`는 거기에서 복구로 남습니다. 또한 `CLAUDE_CODE_MAX_OUTPUT_TOKENS`를 게이트웨이 모델의 출력 제한 아래로 설정합니다. |
| `/model` 선택기에서 누락된 모델                                                                                                                                  | 게이트웨이 모델 이름이 Claude Code의 기본 제공 목록에 없습니다.                                                                                                                                     | [게이트웨이 모델 검색](#add-gateway-models-to-the-model-picker)을 활성화하거나 [모델 구성](/docs/ko/model-config) 변수로 이름을 추가합니다.                                                                                                                                                             |
| Claude Code가 [curl 테스트](#verify-the-connection)가 성공하더라도 로그인하도록 요청합니다.                                                                                  | CLI에는 자체 자격 증명이 없습니다. 도달 가능한 기본 URL은 하나가 아니며, 프로젝트의 `.claude/settings.json` 또는 `.claude/settings.local.json`의 `env` 블록은 첫 실행 마법사 및 신뢰 프롬프트 후에만 적용됩니다.                         | `ANTHROPIC_AUTH_TOKEN`을 Claude Code가 첫 실행 설정 전에 읽는 곳에 설정합니다: 셸 내보내기, `~/.claude/settings.json`의 `env` 블록 또는 관리되는 설정.                                                                                                                                                |
| `ANTHROPIC_API_KEY`가 설정되었지만 무시되고, 프롬프트가 없습니다.                                                                                                          | 키는 대화형 세션에서 일회성 승인이 필요하고, 이전에 거부된 키는 다시 묻지 않고 무시됩니다.                                                                                                                          | `/config`에서 `Use custom API key` 옵션으로 활성화합니다.                                                                                                                                                                                                                       |
| `This machine's managed settings require a first-party login`                                                                                          | 관리되는 설정에 `forceLoginMethod` 또는 `forceLoginOrgUUID`가 포함되어 있으며, Claude Code v2.1.146 이상에서는 `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` 또는 `apiKeyHelper`와 공존할 수 없습니다.           | 관리자는 게이트웨이 자격 증명을 사용하려면 관리되는 설정에서 `forceLoginMethod` 및 `forceLoginOrgUUID`를 제거하거나, 첫 번째 당사자 로그인을 사용하려면 게이트웨이 자격 증명을 제거해야 합니다. 둘을 결합할 수 없습니다.                                                                                                                        |
| `403 Forbidden`과 같은 HTML 본문이 있는 `403`이며, 게이트웨이의 자체 로그에 수신된 요청이 없습니다.                                                                                   | 게이트웨이 앞의 웹 애플리케이션 방화벽 또는 역방향 프록시가 게이트웨이에 도달하기 전에 요청 본문을 차단했습니다. Claude Code 프롬프트에는 XML 스타일 태그와 교차 사이트 스크립팅 본문 규칙과 일치하는 소스 코드가 포함되어 있으므로, 짧은 curl 테스트는 통과하지만 실제 세션은 통과하지 않습니다. | 게이트웨이의 `/v1/messages` 경로를 요청 본문 검사에서 제외합니다. AWS WAF에서는 `CrossSiteScripting_Body` 관리 규칙입니다. nginx와 ModSecurity에서는 동등한 OWASP CRS 본문 규칙입니다.                                                                                                                            |
| `SSL certificate verification failed` 또는 `Self-signed certificate detected`와 같은 인증서 또는 TLS 오류이며, [curl 테스트](#verify-the-connection)는 성공합니다.            | Claude Code의 런타임이 `curl`이 사용하는 것과 동일한 인증 기관을 신뢰하지 않습니다. 일반적으로 기업 TLS 검사 프록시 뒤에 있습니다.                                                                                          | `NODE_EXTRA_CA_CERTS`를 CA 번들 경로로 설정합니다. [CA 인증서 저장소](/docs/ko/network-config#ca-certificate-store)를 참조하세요.                                                                                                                                                               |

게이트웨이 구성을 제거한 후 Claude Code가 반복적으로 로그인하도록 요청하면, 원인은 일반적으로 게이트웨이가 아닌 자격 증명 저장소입니다. [인증 오류](/docs/ko/errors#authentication-errors)를 참조하세요.

<h2 id="related-resources">
  관련 리소스
</h2>

* [LLM 게이트웨이 개요](/docs/ko/llm-gateway): 게이트웨이가 무엇이고 claude.ai 구독과 어떻게 상호 작용하는지
* [조직을 위해 LLM 게이트웨이 배포](/docs/ko/llm-gateway-rollout): 게이트웨이 배포 및 배포 구성을 위한 관리자 대면 체크리스트
* [게이트웨이 프로토콜 참조](/docs/ko/llm-gateway-protocol): Claude Code가 게이트웨이에 전송하는 것(게이트웨이가 전달해야 하는 헤더 및 필드 포함)
* [설정](/docs/ko/settings): 설정 파일이 있는 위치 및 `env` 블록이 읽히는 방식
* [인증](/docs/ko/authentication): 자격 증명 변수, `apiKeyHelper` 및 OAuth 로그인이 상호 작용하는 방식
