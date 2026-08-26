> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 조직을 위한 LLM 게이트웨이 배포

> Claude Code용 게이트웨이 제품 배포: Claude Code가 전송하는 내용을 전달하도록 구성하고, 개발자 자격증명을 발급하며, 관리되는 설정을 통해 구성을 배포하고, 롤아웃을 확인합니다.

이 페이지는 관리자가 Claude Code용 LLM 게이트웨이를 롤아웃하는 과정을 안내합니다. [게이트웨이 요구사항](#gateway-requirements)을 충족하는 게이트웨이 제품이 배포되어 있다고 가정합니다. 특정 제품을 배포하거나 운영하는 방법은 여기서 다루지 않습니다. 공급업체의 설명서에 따라 귀사의 제품을 배포하십시오.

<Note>
  * 자신의 머신에서 Claude Code를 기존 게이트웨이에 연결하려면 [Claude Code를 LLM 게이트웨이에 연결](/docs/ko/llm-gateway-connect)을 참조하십시오.
  * Claude Code가 게이트웨이에 전송하는 내용과 전달할 내용을 알아보려면 [게이트웨이 프로토콜 참조](/docs/ko/llm-gateway-protocol)를 참조하십시오.
</Note>

<h2 id="prerequisites">
  필수 조건
</h2>

롤아웃을 완료하려면 다음이 필요합니다.

* 인프라에 배포된 게이트웨이로, 개발자에게 배포할 정확한 주소에서 HTTPS를 제공하며, 리디렉션되는 주소가 아니고, Claude 모델 이름을 공급자에게 라우팅하도록 구성됨
* 게이트웨이가 전달할 공급자 자격증명:
  * Anthropic API의 경우: [Claude 콘솔](https://platform.claude.com/settings/keys)의 API 키
  * 클라우드 공급자의 경우: 모델 액세스 권한이 있는 클라우드 자격증명. [Amazon Bedrock](/docs/ko/amazon-bedrock#prerequisites), [Google Cloud의 Agent Platform](/docs/ko/google-vertex-ai#prerequisites) 또는 [Microsoft Foundry](/docs/ko/microsoft-foundry#prerequisites) 페이지의 필수 조건을 참조하십시오.
* MDM 또는 구성 관리와 같이 개발자 머신에 설정 파일을 전달하는 방법
  * 아직 없는 경우 [설정이 장치에 도달하는 방법](/docs/ko/admin-setup#decide-how-settings-reach-devices)에서 옵션을 비교합니다.

<h3 id="gateway-requirements">
  게이트웨이 요구사항
</h3>

게이트웨이를 제공하는 제품이 무엇이든 다음을 충족해야 합니다.

* **지원되는 API 형식 수락**: [API 형식 표](/docs/ko/llm-gateway-protocol#api-formats)의 형식 중 하나. 아래의 롤아웃 단계는 `POST /v1/messages`의 Anthropic Messages API를 가정하며, 대부분의 게이트웨이가 이를 제공합니다.
* **응답 스트리밍**: 전체 응답을 버퍼링하는 대신 서버 전송 이벤트를 도착하는 대로 전달합니다.
* **Claude 모델 이름 라우팅**: 개발자가 사용하는 각 이름을 업스트림 모델에 매핑합니다. Claude Code는 각 요청에서 `claude-sonnet-4-6`과 같은 모델 이름을 전송합니다. 대부분의 게이트웨이 제품에서 매핑은 게이트웨이 자체 구성의 모델 목록 또는 라우팅 테이블입니다.
* **헤더 및 본문 변경 없이 전달**: `anthropic-beta`, `anthropic-version` 및 요청 본문을 양방향으로 전달합니다. [기능 통과 표](/docs/ko/llm-gateway-protocol#feature-pass-through)는 각각을 이를 없이는 손상되는 기능에 매핑합니다.
* **업스트림 오류 수정되지 않은 상태로 반환**: Claude Code의 자동 복구는 오류 표현에 일치하므로 게이트웨이 자체 봉투에 오류를 래핑하면 이를 손상시킵니다.
* **요청 본문 WAF 검사에서 경로 제외**: Claude Code 프롬프트는 소스 코드와 교차 사이트 스크립팅 본문 규칙과 일치하는 XML 스타일 태그를 포함합니다. 게이트웨이 앞의 WAF는 실제 세션에서 `403`을 반환하지만 짧은 테스트 요청은 통과합니다.

선택적으로 `GET /v1/models`를 제공하여 Claude Code가 [모델 검색](/docs/ko/llm-gateway-protocol#model-discovery)을 통해 게이트웨이에서 모델 선택기를 채울 수 있도록 합니다.&#x20;

<h2 id="rollout-steps">
  롤아웃 단계
</h2>

롤아웃은 5단계로 진행되며, 각 단계마다 체크포인트가 있습니다.

1. [게이트웨이가 모델을 라우팅하는지 확인](#confirm-the-gateway-routes-your-models)
2. [각 개발자에게 자격증명 발급](#issue-developer-credentials)
3. [게이트웨이에 대해 Claude Code 테스트](#test-claude-code-against-the-gateway)
4. [기본 URL 및 자격증명 배포](#distribute-the-configuration)
5. [개발자 머신에서 확인](#verify-the-rollout)

단계에는 3가지 다른 자격증명이 포함되며, 체크포인트는 무언가 실패할 때 어느 것이 잘못되었는지 알 수 있도록 자리 표시자로 이름을 지정합니다.

| 자격증명          | 보유자                                                                 | 체크포인트의 자리 표시자                  |
| :------------ | :------------------------------------------------------------------ | :----------------------------- |
| 공급자 자격증명      | 게이트웨이로, 업스트림 공급자에게 전달합니다.                                           | 게이트웨이에서 구성됨; 클라이언트 명령에 나타나지 않음 |
| 게이트웨이 관리 자격증명 | 귀사로, 게이트웨이 제품이 관리자 또는 테스트 인터페이스용으로 발급하는 경우                          | `<gateway-key>`                |
| 개발자 키         | 각 개발자로, [개발자 자격증명 발급](#issue-developer-credentials)에서 게이트웨이가 발급합니다. | `<developer-key>`              |

<h3 id="confirm-the-gateway-routes-your-models">
  게이트웨이가 모델을 라우팅하는지 확인
</h3>

게이트웨이는 이미 공급자 자격증명으로 구성되어 있고, 기본 URL에서 수신 대기하며, 공급자의 API로 요청을 전달해야 합니다. 배포에서 두 값을 대체하여 최소 요청으로 경로가 종단 간 작동하는지 테스트합니다.

* `<gateway-key>`는 현재 게이트웨이를 호출할 수 있는 모든 자격증명입니다. 관리 키, 테스트 키 또는 이미 발급한 자신의 개발자 키입니다. 모든 게이트웨이 제품이 별도의 관리 자격증명을 가지고 있지는 않습니다. 귀사의 제품이 없는 경우 먼저 [개발자 자격증명 발급](#issue-developer-credentials)에서 자신에게 개발자 키를 발급하십시오.
* `model`은 게이트웨이가 라우팅하도록 구성된 Claude 모델 이름입니다. 예제는 `claude-sonnet-4-6`을 사용합니다. 구성한 이름으로 대체하십시오.

<Tabs>
  <Tab title="Bash 또는 Zsh">
    ```bash theme={null}
    curl -X POST "https://llm-gateway.example.com/v1/messages" \
      -H "Authorization: Bearer <gateway-key>" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    Invoke-RestMethod -Method Post -Uri "https://llm-gateway.example.com/v1/messages" `
      -Headers @{ "Authorization" = "Bearer <gateway-key>"; "anthropic-version" = "2023-06-01" } `
      -ContentType "application/json" `
      -Body '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>
</Tabs>

**체크포인트**: `content` 필드가 있는 `200`은 게이트웨이가 해당 모델 이름으로 공급자에 도달했음을 의미합니다. `404`는 해당 이름이 게이트웨이에서 라우팅되지 않음을 의미합니다. 공급자의 `401`은 게이트웨이의 공급자 자격증명이 잘못되었음을 의미합니다.

게이트웨이의 라우팅 구성에서 Claude 모델 이름당 한 번씩 요청을 반복합니다. 게이트웨이가 라우팅하지 않는 이름은 해당 이름을 선택하는 모든 개발자에게 `404`를 반환하므로 롤아웃 전에 모든 이름을 테스트하십시오.

<Note>
  리디렉션 뒤에 게이트웨이를 제공하지 마십시오. 리디렉션은 요청 본문을 삭제하거나 추론 요청의 자격증명 헤더를 제거할 수 있으며, [모델 검색](/docs/ko/llm-gateway-protocol#model-discovery)은 모든 리디렉션을 실패로 취급하므로 자격증명이 리디렉션 대상으로 유출될 수 없습니다.
</Note>

<h3 id="issue-developer-credentials">
  개발자 자격증명 발급
</h3>

각 개발자는 인증을 위해 자신의 게이트웨이 키가 필요합니다. 제품의 자격증명 관리 설명서에 따라 게이트웨이에서 개발자당 자격증명을 만듭니다.

새로 발급된 키가 [게이트웨이가 모델을 라우팅하는지 확인](#confirm-the-gateway-routes-your-models)과 동일한 요청으로 게이트웨이에 대해 작동하는지 확인하고, `<gateway-key>`를 새 `<developer-key>`로 바꿉니다.

<Tabs>
  <Tab title="Bash 또는 Zsh">
    ```bash theme={null}
    curl -X POST "https://llm-gateway.example.com/v1/messages" \
      -H "Authorization: Bearer <developer-key>" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    Invoke-RestMethod -Method Post -Uri "https://llm-gateway.example.com/v1/messages" `
      -Headers @{ "Authorization" = "Bearer <developer-key>"; "anthropic-version" = "2023-06-01" } `
      -ContentType "application/json" `
      -Body '{"model": "claude-sonnet-4-6", "max_tokens": 1, "messages": [{"role": "user", "content": "."}]}'
    ```
  </Tab>
</Tabs>

**체크포인트**: `content` 필드가 있는 `200`은 개발자 키가 게이트웨이에 도달하고 게이트웨이가 이를 전달함을 의미합니다. [이전 단계](#confirm-the-gateway-routes-your-models)가 성공했을 때 여기서 `401`이 나타나면 개발자 키가 잘못되었거나 게이트웨이에서 아직 적용되지 않았음을 의미합니다.

공유 키가 아닌 개발자당 하나의 키를 발급하는 것이 개발자별 사용 속성 및 개별 오프보딩을 작동하게 합니다. 키를 보유하는 환경 변수는 게이트웨이가 읽는 헤더에 따라 다릅니다. `Authorization: Bearer` 헤더에서 자격증명을 확인하는 게이트웨이의 경우 개발자는 `ANTHROPIC_AUTH_TOKEN`에서 키를 설정합니다. `x-api-key` 헤더에서 키를 읽는 게이트웨이의 경우 개발자는 대신 `ANTHROPIC_API_KEY`를 설정합니다. [자격증명 표](/docs/ko/llm-gateway-connect#set-the-credential-variable)는 매핑을 다룹니다.

<h3 id="test-claude-code-against-the-gateway">
  게이트웨이에 대해 Claude Code 테스트
</h3>

롤아웃이 배포할 동일한 구성을 사용하여 게이트웨이를 통해 Claude Code를 직접 실행한 후 배포하십시오. 이를 `.env` 또는 설정 파일이 아닌 터미널에 직접 입력하십시오. 이 터미널 세션에만 지속되므로 닫으면 머신이 정상 구성으로 돌아갑니다. 게이트웨이가 `x-api-key` 헤더를 읽는 경우 `ANTHROPIC_AUTH_TOKEN` 대신 `ANTHROPIC_API_KEY`를 사용하십시오.

<Tabs>
  <Tab title="Bash 또는 Zsh">
    ```bash theme={null}
    export ANTHROPIC_BASE_URL=https://llm-gateway.example.com
    export ANTHROPIC_AUTH_TOKEN="<developer-key>"
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $env:ANTHROPIC_BASE_URL = "https://llm-gateway.example.com"
    $env:ANTHROPIC_AUTH_TOKEN = "<developer-key>"
    ```
  </Tab>
</Tabs>

그런 다음 게이트웨이를 통해 일회성 프롬프트를 전송합니다.

```bash theme={null}
claude -p "Reply with one word: connected"
```

**체크포인트**: 프롬프트가 응답을 반환하고 요청이 게이트웨이 로그에 `/v1/messages` 경로에 대한 `POST`로 상태 `200`으로 나타납니다. Claude Code는 `?beta=true`와 같은 쿼리 문자열을 추가하므로 전체 URL이 아닌 경로와 일치합니다. 두 가지 실패 메시지는 다른 방향을 가리킵니다.

* `Not logged in`: 게이트웨이 로그를 확인하여 두 가지 원인을 구분합니다. 비어 있으면 자격증명이 세션에 도달하지 않았고 요청이 머신을 떠나지 않았습니다. 테스트 중인 셸에서 내보내기를 다시 실행하십시오. `401` 본문에 `x-api-key`가 표시되면 게이트웨이가 대신 해당 헤더에서 키를 예상합니다. `ANTHROPIC_API_KEY`로 전환하십시오.
* `Failed to authenticate. API Error: 401`은 자격증명이 전송되고 거부되었음을 의미하며, 게이트웨이 로그는 위치를 나타냅니다. `api.anthropic.com` 또는 공급자의 엔드포인트를 명명하는 `401`은 게이트웨이가 업스트림에 도달했지만 보유한 공급자 자격증명이 거부되었음을 의미하므로 개발자 키가 작동했고 게이트웨이가 보유한 공급자 자격증명이 잘못되었거나 자리 표시자입니다.

잘못되었거나 도달할 수 없는 기본 URL은 다른 증상을 생성합니다. Claude Code는 [백오프를 사용하여 연결을 재시도](/docs/ko/errors#automatic-retries)하며 오류를 보고하기 전에 몇 분 동안 출력 없이 앉아 있을 수 있습니다. 명령이 중단된 것으로 보이면 대기하는 대신 게이트웨이 로그를 확인하십시오. 도착하는 요청이 없으면 `ANTHROPIC_BASE_URL`이 게이트웨이를 가리키지 않습니다.

<h3 id="distribute-the-configuration">
  구성 배포
</h3>

모든 개발자 머신에는 게이트웨이 주소와 자격증명이 필요합니다. [관리되는 설정](/docs/ko/settings#settings-files)을 통해 중앙에서 배포할 수 있으므로 개발자가 아무것도 구성하지 않거나 개발자에게 값을 직접 설정하도록 할 수 있습니다.

<h4 id="what-to-distribute">
  배포할 항목
</h4>

어느 경로를 선택하든 동일한 변수 집합이 적용됩니다. 대부분의 롤아웃은 `ANTHROPIC_BASE_URL`과 자격증명만 필요합니다. 게이트웨이 설정에서 필요한 경우 조건부 행을 포함하십시오.

| 변수 또는 설정                                                                                                                                                                                               | 수행 작업                                                                                                                | 포함 시기                                                                                                                                                                                                                                                        |
| :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_BASE_URL`                                                                                                                                                                                   | Claude Code의 API 요청을 `api.anthropic.com` 대신 게이트웨이로 전송합니다.                                                            | 항상                                                                                                                                                                                                                                                           |
| `apiKeyHelper` 또는 `ANTHROPIC_AUTH_TOKEN` 또는 `ANTHROPIC_API_KEY`의 자격증명                                                                                                                                  | 게이트웨이에 대한 각 요청을 인증합니다. 도우미는 키를 가져오는 명령을 실행합니다. 변수는 각각 `Authorization: Bearer` 및 `x-api-key`로 전송되는 정적 키를 보유합니다.       | 항상; 3개 중 1개                                                                                                                                                                                                                                                  |
| `ANTHROPIC_CUSTOM_HEADERS`                                                                                                                                                                             | 모든 API 요청에 추가 HTTP 헤더를 추가합니다.                                                                                        | 게이트웨이가 모든 요청에 테넌트 또는 라우팅 헤더를 요구하는 경우                                                                                                                                                                                                                         |
| `CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY`                                                                                                                                                           | 시작 시 게이트웨이의 `/v1/models`을 쿼리하고 반환된 이름을 `/model` 선택기에 추가합니다.                                                          | 게이트웨이가 `/v1/models`을 제공하고 개발자의 선택기를 게이트웨이에서 채우려는 경우                                                                                                                                                                                                          |
| `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`                                                                                                                                                               | Claude Code가 사전 릴리스 기능 헤더 및 본문 필드를 전송하지 않도록 중지합니다.                                                                   | 게이트웨이가 베타 필드를 거부하는 Amazon Bedrock 또는 Google Cloud의 Agent Platform 업스트림으로 전달하는 경우. [게이트웨이 요구사항](#gateway-requirements) 참조                                                                                                                                     |
| `ANTHROPIC_MODEL` 또는 [`ANTHROPIC_DEFAULT_HAIKU_MODEL`](/docs/ko/model-config)                                                                                                                               | Claude Code가 주 세션 및 백그라운드 트래픽에 대해 요청하는 모델 이름을 설정합니다.                                                                 | 게이트웨이가 Claude Code의 기본값과 일치하지 않는 모델 이름을 라우팅하거나 [백그라운드 기능](/docs/ko/costs#background-token-usage)을 다른 모델로 라우팅하는 경우. 게이트웨이에서 재정의 이름과 Claude Code의 기본 이름을 모두 라우팅하십시오. 일부 하위 호출은 재정의와 관계없이 기본 이름을 요청할 수 있기 때문입니다. [모델 구성](/docs/ko/model-config)은 세션의 각 부분이 사용하는 모델을 다룹니다. |
| `ANTHROPIC_BEDROCK_BASE_URL`, `ANTHROPIC_VERTEX_BASE_URL`, `ANTHROPIC_FOUNDRY_BASE_URL` 또는 `ANTHROPIC_AWS_BASE_URL`과 [해당 공급자의 변수](/docs/ko/llm-gateway-connect#route-to-a-cloud-provider-through-a-gateway) | Claude Code를 게이트웨이를 통해 공급자별 기본 URL로 가리킵니다. Amazon Bedrock 및 Google Cloud의 Agent Platform은 해당 공급자의 기본 요청 형식으로도 전환합니다. | 게이트웨이가 Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry 또는 AWS의 Claude Platform을 앞에 두는 경우. [API 형식](/docs/ko/llm-gateway-protocol#api-formats) 참조                                                                                                  |

<h4 id="distribute-through-managed-settings">
  관리되는 설정을 통해 배포
</h4>

[관리되는 설정 파일](/docs/ko/settings#settings-files)의 `env` 블록을 통해 변수를 배포하고, MDM, 레지스트리 정책 또는 구성 관리로 푸시합니다.

```json theme={null}
{
  "env": {
    "ANTHROPIC_BASE_URL": "https://llm-gateway.example.com"
  },
  "apiKeyHelper": "/usr/local/bin/get-gateway-key"
}
```

표의 조건부 변수를 동일한 `env` 블록에 추가합니다. 관리되는 `ANTHROPIC_BASE_URL`은 적용되며 개발자의 셸 내보내기로 재정의할 수 없습니다. Claude Code는 프로세스 환경 및 낮은 우선순위 설정보다 이를 적용하기 때문입니다.

관리되는 설정에서 게이트웨이 자격증명과 함께 `forceLoginMethod` 또는 `forceLoginOrgUUID`를 포함하지 마십시오. Claude Code v2.1.146 이상에서 두 키 중 하나는 시작 시 `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` 및 `apiKeyHelper`를 차단하므로 개발자는 `This machine's managed settings require a first-party login`을 보고 진행할 수 없습니다.&#x20;

[서버 관리 설정](/docs/ko/server-managed-settings#platform-availability) 배포는 `api.anthropic.com`에 대한 직접 연결이 필요하므로 게이트웨이 라우팅 세션에 도달하지 않습니다. 게이트웨이 배포는 동일한 키를 적용하는 이 파일 기반 관리 설정 경로를 사용합니다.

자격증명의 경우 위에 표시된 대로 관리되는 설정 파일에서 하나의 [`apiKeyHelper`](/docs/ko/llm-gateway-connect#rotate-credentials-with-apikeyhelper) 명령을 배포합니다. 명령은 로컬 개발자로 비밀 저장소에 인증하므로 각 머신이 자신의 키를 받습니다. 또는 기존 비밀 프로세스를 통해 각 개발자에게 키를 배포하고 자신이 `ANTHROPIC_AUTH_TOKEN`을 설정하도록 합니다.

일부 환경에는 별도의 배포가 필요합니다.

* 데스크톱 앱은 관리되는 설정이 아닌 타사 추론 구성에서 게이트웨이 라우팅을 읽습니다. 데스크톱 세션도 게이트웨이를 통해 라우팅되도록 관리되는 설정과 함께 해당 파일을 MDM을 통해 배포하십시오. [데스크톱 타사 구성 문서](https://claude.com/docs/third-party/claude-desktop/configuration) 및 [데스크톱 게이트웨이 문서](https://claude.com/docs/third-party/claude-desktop/gateway)를 참조하십시오.
* CI 러너는 [러너의 환경](/docs/ko/llm-gateway-connect#configure-each-surface)에서 `ANTHROPIC_BASE_URL` 및 자격증명을 설정해야 합니다.
* 관리되는 Windows 머신의 WSL은 [`wslInheritsWindowsSettings`](/docs/ko/settings#available-settings)가 `true`일 때만 Windows 관리 설정을 읽습니다.

<h4 id="hand-developers-the-values-to-set-themselves">
  개발자에게 값을 직접 설정하도록 합니다.
</h4>

관리되는 설정 배포가 없는 경우 각 개발자에게 [연결 페이지](/docs/ko/llm-gateway-connect#configure-claude-code-yourself)를 따르는 데 필요한 것을 보냅니다.

* 게이트웨이 URL
* 개인 자격증명
* **자격증명을 넣을 변수**: 베어러 토큰 게이트웨이의 경우 `ANTHROPIC_AUTH_TOKEN` 또는 `x-api-key` 게이트웨이의 경우 `ANTHROPIC_API_KEY`. 개발자에게 어느 것을 알려주면 [연결 페이지](/docs/ko/llm-gateway-connect#set-the-credential-variable)에 설명된 시행착오를 절약할 수 있습니다.
* [배포할 항목 표](#what-to-distribute)의 모든 조건부 변수와 해당 값

[연결 페이지](/docs/ko/llm-gateway-connect#configure-claude-code-yourself)는 각각을 설정하는 과정을 개발자에게 안내합니다.

**체크포인트**: 개발자 머신에서 `claude`는 배포된 자격증명이 인증을 만족하므로 로그인 화면을 표시하지 않고 세션을 시작합니다. 그런 다음 `/status`를 실행하고 **Status** 탭을 엽니다. `Anthropic base URL` 줄은 게이트웨이 주소를 표시하고, 관리되는 배포의 경우 `Setting sources` 줄에 관리되는 설정이 포함됩니다. 로그인 화면 또는 누락된 `Anthropic base URL` 줄은 구성이 머신에 도달하지 않았음을 의미합니다.

<h3 id="verify-the-rollout">
  롤아웃 확인
</h3>

게이트웨이 호스트가 아닌 개발자 머신에서 모든 것이 작동하는지 확인하므로 테스트는 개발자가 사용하는 네트워크 경로를 다룹니다. 스트리밍 요청을 전송하여 엔드포인트, 스트리밍 통과 및 모델 라우팅을 한 번에 확인합니다.

<Tabs>
  <Tab title="Bash 또는 Zsh">
    ```bash theme={null}
    curl -N -X POST "https://llm-gateway.example.com/v1/messages" \
      -H "Authorization: Bearer <developer-key>" \
      -H "anthropic-version: 2023-06-01" \
      -H "content-type: application/json" \
      -d '{"model": "claude-sonnet-4-6", "max_tokens": 16, "stream": true, "messages": [{"role": "user", "content": "count to 3"}]}'
    ```
  </Tab>

  <Tab title="PowerShell">
    ```powershell theme={null}
    $body = '{"model": "claude-sonnet-4-6", "max_tokens": 16, "stream": true, "messages": [{"role": "user", "content": "count to 3"}]}'
    $body | curl.exe -N -X POST "https://llm-gateway.example.com/v1/messages" `
      -H "Authorization: Bearer <developer-key>" `
      -H "anthropic-version: 2023-06-01" `
      -H "content-type: application/json" `
      --data-binary '@-'
    ```
  </Tab>
</Tabs>

`data:` 줄이 증분으로 도착하는 것을 볼 수 있습니다. 전체 응답이 일시 중지 후 한 번에 도착하면 게이트웨이가 버퍼링되고 있으며, 이는 Claude Code를 정체시킵니다. `404`는 모델 이름이 라우팅되지 않음을 의미합니다. 모델 이름당 반복합니다.

그런 다음 `claude`를 시작하고 메시지를 보냅니다. 이 단계의 각 증상에는 하나의 원인이 있습니다.

* 로그인 프롬프트는 자격증명 간격을 의미합니다. `/status`를 실행하고 **Status** 탭을 엽니다. `Setting sources` 줄에 관리되는 설정이 포함되지 않으면 배포가 머신에 도달하지 않았습니다. 포함되면 개발자 자격증명이 배포되지 않았으므로 `ANTHROPIC_AUTH_TOKEN` 또는 `apiKeyHelper`를 설정하십시오.
* `Failed to authenticate` 오류는 게이트웨이가 요청을 거부함을 의미합니다. 로그는 어느 자격증명이 실패했는지 나타냅니다. 게이트웨이가 자체 로그하는 거부는 개발자 키를 명명하는 반면, `api.anthropic.com` 또는 공급자의 엔드포인트의 `401`은 게이트웨이가 보유한 공급자 자격증명이 거부되었음을 의미합니다.
* 게이트웨이가 `x-api-key` 헤더에서 키를 예상할 때 `ANTHROPIC_API_KEY`로 설정된 키에 대한 일회성 승인 프롬프트는 예상됩니다. `ANTHROPIC_AUTH_TOKEN`을 사용하면 프롬프트가 나타나지 않고 변수가 자동으로 인수합니다. 이전에 저장된 claude.ai 로그인은 해당 세션에 대해 비활성입니다.

마지막으로 전송한 메시지에 대한 게이트웨이의 로그를 확인합니다. 자격증명은 개발자를 식별하고, [`x-claude-code-session-id` 헤더](/docs/ko/llm-gateway-protocol#request-headers)는 요청을 세션별로 그룹화합니다. 기능이 [문제 해결 증상](/docs/ko/llm-gateway-connect#troubleshoot-gateway-errors)으로 실패하면 게이트웨이가 헤더를 제거하거나 오류를 다시 작성하고 있습니다. 위의 [게이트웨이 요구사항](#gateway-requirements)을 참조하십시오.

<h2 id="maintain-the-gateway">
  게이트웨이 유지 관리
</h2>

롤아웃 후 시간이 지남에 따라 3가지 변경 사항이 게이트웨이에 도달합니다. 각각에는 주의할 증상과 취할 조치가 있습니다.

| 변경                                                         | 게이트웨이가 따라가지 못했을 때의 증상                                                                                           | 조치                                                                                                                                                                  |
| :--------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 새로운 Claude Code 릴리스는 `anthropic-beta` 값 및 요청 본문 필드를 추가합니다. | 개발자가 Claude Code를 업데이트한 후 새 필드를 명명하는 `400` 오류를 보고합니다. [기능 통과](/docs/ko/llm-gateway-protocol#feature-pass-through) 참조 | 허용 목록을 작성하는 대신 `anthropic-*` 헤더 및 요청 본문을 그대로 전달합니다. 개발자에게 도달하기 전에 새 Claude Code 릴리스를 게이트웨이에 대해 테스트합니다.                                                              |
| 새로운 Claude 모델을 사용할 수 있게 됩니다.                               | 개발자가 새 모델 이름을 선택하면 `404`를 받습니다. `/model` 선택기에 나열되지 않습니다.                                                        | 게이트웨이의 라우팅 구성에 모델 이름을 추가한 다음 [라우팅 확인](#confirm-the-gateway-routes-your-models)을 다시 실행합니다. `ANTHROPIC_MODEL` 또는 기본 모델 변수를 배포하는 경우 관리되는 설정을 업데이트합니다.                  |
| 자격증명이 만료되거나 회전이 필요합니다.                                     | 모든 개발자 요청이 업스트림에서 `401`로 실패하기 시작합니다.                                                                            | 게이트웨이의 공급자 자격증명을 자체 일정에 따라 회전합니다. 개발자 키는 게이트웨이에서 회전하고, [`apiKeyHelper`](/docs/ko/llm-gateway-connect#rotate-credentials-with-apikeyhelper)는 설정을 재배포하지 않고 개발자별 회전을 처리합니다. |

키당 속도 제한을 크기 조정할 때 클라이언트가 `429` 응답을 포함하여 일시적 실패를 [재시도](/docs/ko/errors#automatic-retries)하는 것을 고려하십시오. 최대 10회 백오프를 사용하여 `Retry-After`를 준수합니다. [프로토콜 참조](/docs/ko/llm-gateway-protocol)를 각 Claude Code 릴리스가 전송하는 내용의 계약으로 유지합니다.

<h2 id="related-resources">
  관련 리소스
</h2>

* [Claude Code를 LLM 게이트웨이에 연결](/docs/ko/llm-gateway-connect): 개발자 대면 설정 단계로, 표면별 구성 및 개발자에게 제공할 수 있는 문제 해결 표 포함
* [게이트웨이 프로토콜 참조](/docs/ko/llm-gateway-protocol): 게이트웨이 운영자를 위한 와이어 계약으로, 엔드포인트, 전달할 헤더 및 기능 통과 표 포함
* [설정 파일 및 우선순위](/docs/ko/settings#settings-files): 관리되는 설정, 프로젝트 및 사용자 설정이 결합되는 방식 및 각 플랫폼에서 관리되는 파일이 위치하는 곳
* [조직을 위한 Claude Code 설정](/docs/ko/admin-setup): 이 게이트웨이가 일부인 더 넓은 롤아웃로, 정책 적용, 사용 가시성 및 데이터 처리 포함
