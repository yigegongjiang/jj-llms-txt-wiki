> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code 게이트웨이 호환성 가이드

> Claude Code와 호환되는 LLM 게이트웨이 유지: 호출하는 엔드포인트, 전달해야 할 헤더 및 본문 필드, 그리고 제거될 때 중단되는 기능.

이 페이지는 Claude Code가 게이트웨이로 전송하는 요청을 문서화하며, 호출하는 엔드포인트, 게이트웨이가 전달해야 할 헤더 및 본문 필드, 그리고 전달하지 않을 때 작동을 멈추는 기능을 포함합니다. 이 문서는 Claude Code와 함께 작동하도록 게이트웨이 제품을 구성하는 운영자를 위해 작성되었습니다.

[Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway)는 Anthropic의 자체 호스팅 게이트웨이이며, `GET /protocol`에서 자체 엔드포인트 참조를 제공하며, 해당 게이트웨이의 로그인, 추론, 관리 설정, 모델 검색 및 원격 분석 엔드포인트를 다룹니다. 이는 이 가이드와는 별개의 문서입니다.

<Note>
  * 조직을 위해 기존 또는 타사 게이트웨이를 배포하려면 [LLM 게이트웨이 배포](/docs/ko/llm-gateway-rollout)를 참조하세요.
  * Claude Code를 제공받은 자격 증명으로 게이트웨이에 인증하는 개별 개발자인 경우 [Claude Code를 LLM 게이트웨이에 연결](/docs/ko/llm-gateway-connect)을 참조하세요.
</Note>

이 페이지는 다음을 다룹니다:

* [API 형식](#api-formats) 및 각 형식에 대해 제공할 엔드포인트
* [연결 방법별 클라이언트 동작](#how-the-connection-method-changes-client-behavior): 모델 ID, `anthropic-beta` 값, 요청 필드 및 기본값이 형식과 Claude 앱 게이트웨이 로그인 간에 어떻게 다른지
* [요청 헤더](#request-headers): 업스트림에 도달해야 하는 것과 게이트웨이가 사용할 수 있는 것
* [응답 헤더](#response-headers): 정체 감지, 재시도 및 사용량 제한 표시가 작동하도록 반환할 내용
* [시스템 프롬프트 속성 블록](#system-prompt-attribution-block) 및 프롬프트 캐싱과의 상호 작용
* [기능 통과](#feature-pass-through): 헤더 또는 본문 필드가 제거될 때 중단되는 것
* [모델 검색](#model-discovery)

이 페이지는 게이트웨이가 각 헤더 및 본문 필드로 수행하는 작업에 대해 두 가지 용어를 사용합니다:

* **변경 없이 전달**: 바이트 단위로 업스트림에 전달
* **사용**: 게이트웨이가 라우팅, 속성 또는 추적을 위해 읽을 수 있으며 전달할 필요가 없음

변경 없이 전달로 표시되지 않은 모든 것은 사용하거나 무시할 수 있습니다.

<h2 id="api-formats">
  API 형식
</h2>

게이트웨이는 Claude Code 클라이언트에 다음 API 형식 중 최소 하나 이상을 노출해야 합니다. 클라이언트는 형식을 선택하고 아래 표의 선택됨 열에 있는 변수를 사용하여 Claude Code를 게이트웨이로 지정합니다.

Google Cloud의 Agent Platform은 Google Cloud의 Claude 엔드포인트이며, 이전의 Vertex AI입니다. 변수 이름은 `VERTEX` 표기법을 유지합니다.

| 형식                                      | 선택됨                                                       | 엔드포인트                                                                                                       | 변경 없이 전달                                                                       |
| :-------------------------------------- | :-------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------- |
| Anthropic Messages                      | `ANTHROPIC_BASE_URL`                                      | `/v1/messages`, `/v1/messages/count_tokens` (선택사항)                                                          | `anthropic-beta` 및 `anthropic-version` 요청 헤더                                   |
| Amazon Bedrock InvokeModel              | `ANTHROPIC_BEDROCK_BASE_URL`과 `CLAUDE_CODE_USE_BEDROCK=1` | `/model/{model}/invoke`, `/model/{model}/invoke-with-response-stream`, `/model/{model}/count-tokens` (선택사항) | `anthropic_beta` 및 `anthropic_version` 요청 본문 필드                                |
| Google Cloud의 Agent Platform rawPredict | `ANTHROPIC_VERTEX_BASE_URL`과 `CLAUDE_CODE_USE_VERTEX=1`   | `:rawPredict`, `:streamRawPredict`, `count-tokens:rawPredict` (선택사항)                                        | `anthropic-beta` 및 `anthropic-version` 요청 헤더, 그리고 `anthropic_version` 요청 본문 필드 |

<h3 id="foundry-and-claude-platform-on-aws">
  Foundry 및 AWS의 Claude Platform
</h3>

Microsoft Foundry 및 [AWS의 Claude Platform](/docs/ko/claude-platform-on-aws)은 Anthropic Messages 형식을 구현합니다. Claude Code는 자체 변수인 `ANTHROPIC_FOUNDRY_BASE_URL` 및 `ANTHROPIC_AWS_BASE_URL`을 통해 이들로 라우팅하지만, 둘 중 하나를 앞에 두는 게이트웨이는 위의 Anthropic Messages 행을 구현합니다. AWS의 Claude Platform을 앞에 두는 게이트웨이는 또한 `anthropic-workspace-id` 헤더를 전달해야 하며, [해당 플랫폼은 모든 요청에서 이를 요구합니다](/docs/ko/claude-platform-on-aws).

<h3 id="optional-endpoints-and-startup-traffic">
  선택사항 엔드포인트 및 시작 트래픽
</h3>

토큰 계산 엔드포인트는 유일한 선택사항입니다. 이들이 없을 때 Claude Code는 컨텍스트 사용량의 문자 기반 추정으로 폴백합니다.

전체 URL이 아닌 경로로 일치시킵니다:

* 추론 요청은 `/v1/messages?beta=true`로 게시됩니다
* Google Cloud의 Agent Platform 메서드 접미사는 게시자 모델 경로에 첨부됩니다(예: `/projects/{project}/locations/{location}/publishers/anthropic/models/{model}:streamRawPredict`).

게이트웨이는 또한 아무것도 깨지 않고 거부할 수 있는 최선의 노력 시작 트래픽을 봅니다. Anthropic Messages 형식 게이트웨이는 `HEAD /api/hello` 연결 워밍 프로브를 수신하며, Claude Code는 HTTP 프록시 또는 클라이언트 인증서가 구성되어 있을 때 이를 건너뜁니다. Amazon Bedrock 형식 게이트웨이는 `GET /inference-profiles?type=SYSTEM_DEFINED` 요청을 수신하고, 구성된 모델이 추론 프로필일 때 `GET /inference-profiles/{profile}` 조회를 수신합니다.

[빠른 모드](/docs/ko/fast-mode) 가용성 확인은 게이트웨이 로그에 나타나지 않습니다. `ANTHROPIC_BASE_URL`을 따르지 않고 `api.anthropic.com`을 직접 호출하므로, `api.anthropic.com`으로의 직접 송신을 차단하는 네트워크에서 빠른 모드는 연결 오류를 보고할 수 있지만 게이트웨이를 통한 추론은 계속 작동합니다. [WebFetch 도메인 안전 확인](/docs/ko/data-usage#webfetch-domain-safety-check)도 `api.anthropic.com`을 직접 호출합니다. [프록시 및 LLM 게이트웨이 뒤에서 빠른 모드 사용](/docs/ko/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways)은 이를 복원하는 변수를 다룹니다.

<h3 id="streaming">
  스트리밍
</h3>

추론 응답을 스트리밍합니다. Claude Code는 도착하는 대로 스트림을 읽으므로, 게이트웨이가 완전한 응답을 버퍼링한 후 릴레이하면 Claude Code가 정지됩니다.

클라이언트가 Amazon Bedrock 형식을 사용할 때, `InvokeModelWithResponseStream` 응답 본문과 `Content-Type: application/vnd.amazon.eventstream` 헤더를 수정하지 않고 릴레이하고, 스트림을 서버 전송 이벤트로 변환하지 마십시오. [게이트웨이 또는 프록시 뒤의 스트리밍 오류](/docs/ko/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy)를 참조하십시오.

keep-alive 핑도 전달합니다. `ANTHROPIC_BASE_URL` 또는 `ANTHROPIC_AWS_BASE_URL`을 통한 연결에서 Claude Code는 게이트웨이가 릴레이하는 모든 바이트(SSE `ping` 이벤트 및 주석 줄 포함)를 계산하고, 기본적으로 300초 동안 침묵하는 스트림을 중단합니다. 업스트림의 핑은 긴 사고 일시 중지 중 유일한 트래픽이므로, 게이트웨이가 이를 제거하거나 버퍼링하면 Claude Code는 해당 일시 중지 중에 스트림을 중단합니다. [자동 재시도](/docs/ko/errors#automatic-retries)는 응답이 진행된 정도에 따라 중단된 스트림이 보고하는 내용을 다룹니다. Amazon Bedrock의 이진 이벤트 스트림과 같이 핑을 전혀 보내지 않는 업스트림은 해당 일시 중지를 전달할 것이 없습니다. 이러한 업스트림에서 변환할 때, 침묵한 간격 동안 자신의 `ping` 이벤트를 내보냅니다. `ANTHROPIC_BEDROCK_BASE_URL`, `ANTHROPIC_VERTEX_BASE_URL` 또는 `ANTHROPIC_FOUNDRY_BASE_URL`을 통해 도달한 게이트웨이는 Anthropic Messages 형식을 릴레이할 때도 이 바이트 수준 감시견으로 래핑되지 않습니다. 거기서는 [5분 유휴 타임아웃](/docs/ko/env-vars)이 침묵한 스트림을 중단하고, `ANTHROPIC_BEDROCK_BASE_URL` 연결에서 [`CLAUDE_ENABLE_BYTE_WATCHDOG_BEDROCK`](/docs/ko/env-vars)으로 바이트 감시견을 추가할 수 있습니다.

<h3 id="format-mismatch-with-the-upstream">
  업스트림과의 형식 불일치
</h3>

클라이언트가 사용하는 형식은 게이트웨이가 수신하는 것을 결정합니다. 일반적인 실패 모드는 클라이언트가 게이트웨이로 보내는 형식과 뒤의 업스트림 제공자가 허용하는 형식 간의 불일치입니다.

* 클라이언트가 Amazon Bedrock 또는 Google Cloud의 Agent Platform 형식을 사용할 때, Claude Code는 해당 제공자가 허용하는 전체 기능 집합의 부분 집합만 보냅니다
* 클라이언트가 Anthropic Messages 형식을 사용할 때, 게이트웨이가 Amazon Bedrock 또는 Google Cloud의 Agent Platform 업스트림으로 전달하더라도 Claude Code는 전체 집합을 보냅니다

그 차이를 연결하는 것은 게이트웨이의 작업입니다. [기능 통과](#feature-pass-through)는 그렇지 않을 때 무엇이 깨지는지 설명합니다.

업스트림이 Amazon Bedrock 또는 Google Cloud의 Agent Platform인 경우, 대신 해당 제공자의 형식을 노출하여 연결을 피할 수 있습니다. [게이트웨이를 통해 클라우드 제공자로 라우팅](/docs/ko/llm-gateway-connect#route-to-a-cloud-provider-through-a-gateway)은 해당 형식의 클라이언트 구성을 보여줍니다.

<h2 id="how-the-connection-method-changes-client-behavior">
  연결 방법이 클라이언트 동작을 변경하는 방식
</h2>

개발자가 게이트웨이에 연결하는 방식에 따라 Claude Code가 전송하는 모델 ID, `anthropic-beta` 값, 요청 필드가 결정되며, 적용되는 기본값도 달라집니다. 게이트웨이는 다음 세 가지 클라이언트 동작 중 하나를 확인합니다:

* **Amazon Bedrock 또는 Agent Platform 형식**: 개발자가 `CLAUDE_CODE_USE_BEDROCK=1`을 `ANTHROPIC_BEDROCK_BASE_URL`과 함께 설정하거나, `CLAUDE_CODE_USE_VERTEX=1`을 `ANTHROPIC_VERTEX_BASE_URL`과 함께 설정하여 게이트웨이를 가리킵니다. Claude Code는 해당 제공자의 모델 ID, 요청 필드 및 기본값을 사용합니다.
* **Anthropic Messages 형식**: 개발자가 `ANTHROPIC_BASE_URL`을 게이트웨이로 설정합니다. Claude Code는 게이트웨이를 Claude API로 취급하며 어느 업스트림으로 전달하는지 알 수 없습니다.
* **Claude 앱 게이트웨이 로그인**: 개발자가 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway)에 로그인합니다. 해당 게이트웨이는 Anthropic Messages 형식을 사용하지만 모든 업스트림으로 라우팅할 수 있으므로, Claude Code는 Amazon Bedrock 및 Agent Platform도 허용하는 `anthropic-beta` 값과 모델 기능 가정만 전송합니다.

<h3 id="requests-and-defaults-by-connection-method">
  연결 방법별 요청 및 기본값
</h3>

아래 표는 세 가지 연결 방법을 비교하며, 행마다 하나의 동작을 나타냅니다. Microsoft Foundry 및 Claude Platform on AWS는 Anthropic Messages 형식을 사용하지만 Claude Code가 자체 변수를 통해 도달하므로 제외되었습니다. 이들의 경우 [Microsoft Foundry](/docs/ko/microsoft-foundry) 및 [Claude Platform on AWS](/docs/ko/claude-platform-on-aws) 페이지를 참조하십시오.

| 동작                                                                                              | Amazon Bedrock 또는 Agent Platform 형식                                                                                                                                   | Anthropic Messages 형식                                                                                                                     | Claude 앱 게이트웨이 로그인                                                                     |
| :---------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------- |
| 기본적으로 요청의 모델 ID                                                                                 | Amazon Bedrock의 `us.anthropic.claude-opus-4-8`과 같은 제공자 형식                                                                                                             | `claude-opus-4-8`과 같은 Anthropic ID                                                                                                        | Anthropic ID                                                                           |
| 전송되는 `anthropic-beta` 값                                                                         | Amazon Bedrock 및 Agent Platform이 허용하는 부분 집합                                                                                                                           | [기능 통과](#feature-pass-through)에 설명된 전체 집합, 개발자가 [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](#disable-pre-release-capabilities)를 설정하지 않은 경우 | Amazon Bedrock 및 Agent Platform이 허용하는 부분 집합                                            |
| Claude Code가 인식하지 못하는 모델 ID(예: 게이트웨이 별칭)에 대한 요청 필드                                              | 적응형 추론이 아닌 고정 예산의 사고, 그리고 노력 또는 컨텍스트 관리 필드 없음                                                                                                                         | Amazon Bedrock 또는 Agent Platform 업스트림이 거부할 수 있는 적응형 추론, 노력 및 컨텍스트 관리를 포함하여 현재 Claude 모델이 Claude API에서 허용하는 모든 것                           | Amazon Bedrock 또는 Agent Platform 형식과 동일                                                |
| 개발자가 옵트인할 때 1시간 [프롬프트 캐시 TTL](/docs/ko/prompt-caching#choose-the-ttl-yourself)                       | `cache_control`의 `ttl` 필드를 통해 요청되며, 베타 값 없음                                                                                                                           | `ttl` 필드와 `anthropic-beta`의 `extended-cache-ttl` 값을 통해 요청되며, 이를 전달해야 함                                                                    | Claude 앱 게이트웨이 [가용성 및 제한사항](/docs/ko/claude-apps-gateway#availability-and-limitations) 표 참조 |
| `ANTHROPIC_DEFAULT_HAIKU_MODEL`이 하나를 고정하지 않는 한 [백그라운드 작업](/docs/ko/costs#background-token-usage)용 모델 | 기본 Sonnet 모델, 또는 하나가 선택되면 주 모델, [Amazon Bedrock](/docs/ko/amazon-bedrock#4-pin-model-versions) 및 [Agent Platform](/docs/ko/google-vertex-ai#5-pin-model-versions) 페이지에서 설명하는 대로 | 주 모델, 또는 `ANTHROPIC_API_KEY` 또는 `apiKeyHelper`가 Anthropic Console 키를 제공하고 `ANTHROPIC_AUTH_TOKEN`이 설정되지 않은 경우 기본 Haiku 모델                  | 주 모델                                                                                   |

각 연결이 지원하는 기능과 기본적으로 Anthropic에 전송하는 원격 측정에 대해서는 [기능 가용성](/docs/ko/feature-availability#availability-by-model-provider) 및 [API 제공자별 기본 동작](/docs/ko/data-usage#default-behaviors-by-api-provider)을 참조하십시오.

<h3 id="settings-for-unrecognized-model-ids">
  인식되지 않는 모델 ID에 대한 설정
</h3>

두 가지 클라이언트 측 설정은 개발자가 사용하는 연결 방법에 관계없이 Claude Code가 인식하지 못하는 모델 ID에 대해 가정하는 것을 변경합니다:

* **컨텍스트 윈도우**: Claude Code는 200K를 가정하거나, ID에 `[1m]`이 포함되면 1M을 가정합니다. 실제 윈도우를 선언하려면 [게이트웨이 또는 사용자 정의 모델 ID에 대한 윈도우 수정](/docs/ko/model-config#correct-the-window-for-a-gateway-or-custom-model-id)을 참조하십시오.
* **기능**: 게이트웨이 별칭에 뒤에 있는 모델의 기능을 제공하려면, 해당 모델의 Anthropic ID를 배포하는 설정의 [`modelOverrides`](/docs/ko/errors#unrecognized-model-id-on-a-request) 항목으로 별칭에 매핑하십시오. `ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES` 변수가 적용되는 위치에 대해서는 [기능 통과](#feature-pass-through)를 참조하십시오.

<h2 id="request-headers">
  요청 헤더
</h2>

Claude Code는 API 요청에 이러한 헤더를 포함합니다. 헤더 이름은 전송 중에 대소문자를 구분하지 않습니다. `anthropic-version` 및 `anthropic-beta`를 변경 없이 전달하고, 업스트림이 [AWS의 Claude Platform](/docs/ko/claude-platform-on-aws)일 때 `anthropic-workspace-id`를 전달합니다. 나머지는 게이트웨이가 라우팅, 속성 및 추적을 위해 사용할 수 있으며 전달할 필요가 없습니다.

| 헤더                              | 설명                                                                                                                                                                                                                                                  |
| :------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Authorization`, `x-api-key`    | 개발자의 게이트웨이 자격 증명. 설정한 [자격 증명 변수](/docs/ko/llm-gateway-connect#set-the-credential-variable)에 따라 하나 또는 두 헤더 모두에 포함됨                                                                                                                                        |
| `anthropic-version`             | API 버전. 현재 `2023-06-01`. Amazon Bedrock 및 Google Cloud의 Agent Platform 형식 요청은 또한 `anthropic_version` 본문 필드를 전달하며, 그 값은 이 헤더의 값이 아닌 제공자 방언 문자열입니다.                                                                                                   |
| `anthropic-beta`                | 요청에 대한 쉼표로 구분된 기능 값. 헤더를 그대로 전달합니다. 개별 값을 허용 목록에 추가하지 마십시오. Claude Code 릴리스에 따라 집합이 변경되기 때문입니다. 개발자가 claude.ai 로그인으로 인증할 때 (이는 `ANTHROPIC_BASE_URL`이 게이트웨이 자격 증명 변수 없이 설정될 때 가능함), 이 헤더는 또한 업스트림이 요구하는 OAuth 기능을 전달하며, 이를 제거하면 해당 요청이 `401`로 실패합니다. |
| `x-claude-code-session-id`      | 현재 Claude Code 세션의 고유 식별자. 요청 본문을 구문 분석하지 않고 한 세션의 모든 요청을 집계하는 데 사용합니다.                                                                                                                                                                             |
| `x-claude-code-agent-id`        | 요청을 발급한 [서브에이전트](/docs/ko/sub-agents)의 식별자. 세션 내에서 Claude Code가 생성한 에이전트의 요청에만 존재합니다. 세션 ID와 함께 사용하여 병렬 에이전트에 비용을 속성화합니다.                                                                                                                                |
| `x-claude-code-parent-agent-id` | 요청하는 에이전트를 생성한 에이전트의 식별자. 중첩된 에이전트에만 존재합니다.                                                                                                                                                                                                         |

서브에이전트 ID는 각 생성 시마다 새로 생성됩니다. 팀 에이전트 (즉, [에이전트 팀](/docs/ko/agent-teams)의 명명된 멤버)는 재연결 시 안정적인 이름 기반 ID를 재사용합니다. 두 경우 모두 ID는 사람이나 장치가 아닌 에이전트를 식별하므로, 에이전트 ID 헤더를 사용자 식별자로 취급하지 마십시오.

개발자가 `ANTHROPIC_CUSTOM_HEADERS`를 설정하면, 해당 헤더도 요청에 나타납니다.

<h3 id="gateway-hint-headers">
  게이트웨이 힌트 헤더
</h3>

Claude Code는 또한 라우팅 힌트를 전송할 수 있습니다. 게이트웨이 또는 라우터가 요청을 스케줄링, 캐싱 또는 속성화하는 데 사용할 수 있는 요청별 팩트입니다. Claude Code v2.1.273 이상이 필요합니다.

요청이 이를 전달하는지 여부는 Claude Code가 이를 전송하는 위치에 따라 다릅니다.

* Anthropic API에 직접 연결: 기본적으로 전송됨
* 사용자 정의 기본 URL: 기본적으로 꺼짐. 알 수 없는 헤더를 거부하는 프록시가 요청을 실패시킬 수 있기 때문입니다. 이를 수신하려면 개발자를 위해 [`CLAUDE_CODE_GATEWAY_HINT_HEADERS=1`](/docs/ko/env-vars)을 설정합니다. 예를 들어 [관리 설정](/docs/ko/managed-settings)의 `env` 블록에서 설정합니다.
* Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry, AWS의 Claude Platform을 포함한 다른 모든 백엔드: `CLAUDE_CODE_GATEWAY_HINT_HEADERS=1`이 설정되었을 때만 전송됨

`CLAUDE_CODE_GATEWAY_HINT_HEADERS`를 `0`으로 설정하면 모든 연결에서 헤더가 중지됩니다.

헤더는 아래 행에 나열된 것만 전달합니다. 고정된 어휘, 도구 이름 및 기간이며, 프롬프트 텍스트나 파일 내용은 절대 아닙니다. 모든 값은 인쇄 가능한 ASCII입니다.

| 헤더                                  | 설명                                                                                                                                                                                                                                                                                                                                                 |
| :---------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `x-claude-code-request-class`       | 이 요청의 종류: 주 대화의 턴에 대해 `main`, [서브에이전트](/docs/ko/sub-agents)의 턴에 대해 `subagent`, 워크플로우 내에서 실행되는 에이전트에 대해 `workflow`, 대화를 압축하는 요약 요청에 대해 `compaction`, 세션 제목, 분류기 및 요약과 같은 측면 요청에 대해 `auxiliary`. 모든 요청에서 전송됨                                                                                                                                              |
| `x-claude-code-agent-type`          | 요청을 발급한 서브에이전트의 종류: `Explore`, `Plan` 또는 `general-purpose`와 같은 기본 제공 에이전트 타입 이름, 사용자 정의 에이전트에 대해 `custom`, [에이전트 팀](/docs/ko/agent-teams) 멤버가 리드의 프로세스에서 실행 중일 때 `teammate`, 또는 [포크](/docs/ko/sub-agents#fork-the-current-conversation)에 대해 `fork`. 서브에이전트 자신의 턴에만 존재합니다. 서브에이전트의 압축 또는 측면 요청은 에이전트 ID를 유지하지만 타입을 전달하지 않습니다. 사용자가 선택한 에이전트 이름은 절대 전송되지 않습니다. |
| `x-claude-code-compaction`          | [압축](/docs/ko/prompt-caching#compacting-the-conversation) 중에 대화를 요약하는 요청에 존재합니다. 값은 트리거된 것을 나타냅니다: 컨텍스트 윈도우가 용량에 접근할 때 `auto`, `/compact`에 대해 `manual`, API가 요청을 너무 길다고 거부했을 때 `reactive`. 다른 모든 요청에서는 없습니다.                                                                                                                                            |
| `x-claude-code-context-compacted`   | 압축 후 첫 번째 주 대화 요청에 한 번 존재하며, `x-claude-code-compaction`과 동일한 값을 가집니다. 이 요청 전의 대화 접두사는 더 이상 사용되지 않으므로, 이를 기반으로 키가 지정된 캐시를 삭제할 수 있습니다.                                                                                                                                                                                                               |
| `x-claude-code-prev-tool-durations` | 이 요청이 전달하는 결과의 도구 호출의 측정된 실행 시간. `<name>=<ms>;<name>=<ms>` 형식입니다. 예를 들어 `Bash=742;Read=9`. 도구 호출 배치 후 동일한 대화의 다음 요청에서 전송됩니다. 주 세션 또는 서브에이전트에서 전송됩니다.                                                                                                                                                                                               |

`x-claude-code-prev-tool-durations`를 구문 분석하기 전에 Claude Code가 값을 구성하는 방법과 생략하는 것을 확인합니다.

* 항목: 실행된 도구 호출당 하나씩, 결과가 수집된 순서대로, 전체 밀리초 단위
* 상한: Claude Code는 최대 32개 항목과 4 KB를 전송하며, 첫 번째 항목을 유지합니다.
* 인코딩: 도구 이름은 퍼센트 인코딩되며, `%`, `;`, `=`, 쉼표, 공백 및 인쇄 가능한 ASCII 외의 모든 문자를 포함합니다.
* 구문 분석: `;`로 분할한 다음 `=`로 분할하고 각 이름을 디코딩합니다.
* 부재: 압축 호출, 측면 요청 및 새 프롬프트의 첫 번째 요청은 이를 전달하지 않습니다. 누락된 헤더를 도구를 실행하지 않은 턴으로 읽지 마십시오.
* 시간: 각각은 권한 프롬프트 및 훅을 제외하며, 병렬 도구 호출은 각각 자신의 시간을 보고하므로, 항목이 요청 간의 간격에 합산되지 않습니다.

<h3 id="forward-as-open-lists">
  개방형 목록으로 전달
</h3>

헤더 및 본문 필드를 닫힌 목록이 아닌 개방형 목록으로 취급합니다. Claude Code는 릴리스에 따라 기능을 얻으며, 이들은 새로운 `anthropic-beta` 값, 새로운 요청 본문 필드, 그리고 때때로 새로운 `anthropic-*` 또는 `x-claude-code-*` 헤더로 도착합니다.

Anthropic 형식 업스트림으로 전달할 때, 오늘 보는 것들을 허용 목록에 추가하기보다는 `anthropic-*` 요청 헤더 및 요청 본문 필드를 변경 없이 통과시킵니다. 관찰된 목록에 고정된 게이트웨이는 다음 기능의 헤더 또는 필드를 제거하고 이를 도입하는 릴리스에서 손상시킵니다.

예외는 Amazon Bedrock 또는 Google Cloud의 Agent Platform과 같은 비 Anthropic 업스트림입니다. 여기서 스키마 차이를 연결하는 것은 게이트웨이의 작업입니다. [기능 통과](#feature-pass-through)를 참조하십시오.

<h2 id="response-headers">
  응답 헤더
</h2>

Claude Code는 이러한 응답 헤더를 읽어 정지된 스트림을 감지하고, 재시도 여부 및 시기를 결정하며, 사용량 제한을 표시합니다. 다음 표는 각 헤더에 대해 반환할 내용을 나열합니다. 또한 오류 응답 본문을 수정하지 않고 전달하여 Claude Code의 [기능 거부 복구](#automatic-retry-and-error-forwarding)가 업스트림의 오류 표현과 일치할 수 있도록 합니다.

| 헤더                              | 반환할 내용 및 이유                                                                                                                                                                                                                                                                           |
| :------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `content-type`                  | 스트리밍된 Anthropic Messages 형식 응답에서는 `text/event-stream`을 반환하고, Amazon Bedrock 형식 응답에서는 `application/vnd.amazon.eventstream`을 수정하지 않고 반환합니다. 여기서 [다른 유형은 요청을 실패시킵니다](/docs/ko/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy). [스트리밍](#streaming)은 이러한 스트림에서 정지 감지를 실행하는 연결을 나열합니다 |
| `retry-after`                   | HTTP 날짜가 아닌 정수 초를 반환합니다. Claude Code는 다음 [자동 재시도](/docs/ko/errors#automatic-retries) 전에 최소한 그 시간만큼 대기하며, [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/ko/env-vars) 세션 외부에서 60 이상의 값은 재시도를 중지하고 오류를 즉시 표시합니다                                                                                             |
| `x-should-retry`                | 업스트림의 값을 수정하지 않고 그대로 전달합니다. Claude Code는 실패한 요청을 재시도할지 결정할 때 이 헤더를 하나의 입력으로 읽습니다: `true`는 응답을 재시도 가능으로 표시하고 `false`는 재시도 불가능으로 표시합니다. 재시도 횟수, 백오프 및 Claude Code가 재시도하는 실패에 대해서는 [자동 재시도](/docs/ko/errors#automatic-retries)를 참조하세요                                                         |
| `anthropic-ratelimit-unified-*` | 모든 응답에서 업스트림의 값을 수정하지 않고 전달합니다. Claude Code는 성공한 응답에서 이를 읽어 claude.ai로 로그인한 개발자에게 계획 제한에 대한 사용량을 표시하고, `429`에서 계획 제한 또는 지출 한도를 임시 제한과 구분합니다. [사용량 제한](/docs/ko/errors#usage-limits)을 참조하세요                                                                                                 |

<h2 id="system-prompt-attribution-block">
  시스템 프롬프트 속성 블록
</h2>

Claude Code는 클라이언트 버전과 대화에서 파생된 지문을 포함하는 짧은 속성 블록을 시스템 프롬프트 앞에 추가합니다. `api.anthropic.com` 엔드포인트는 변경되지 않은 상태로 첫 번째 시스템 블록으로 도착할 때 처리 전에 블록을 제거하므로 자사 프롬프트 캐싱에 영향을 주지 않습니다. 다른 업스트림은 프롬프트의 일부로 수신합니다.

제거는 위치 기반이므로 게이트웨이가 `system` 배열을 변경되지 않은 상태로 전달할 때만 작동합니다. 다른 시스템 콘텐츠를 잃지 않으면서 블록을 프롬프트에서 제외하려면:

* 받은 `system` 배열을 정확히 전달하고 블록을 먼저 유지합니다: 다른 시스템 블록을 앞에 추가하거나, 배열을 재정렬하거나, 단일 문자열로 변환하면 제거가 실패하고 블록이 모델과 프롬프트 캐시 키에 도달합니다.
* 블록을 자체 배열 항목에 유지합니다: 엔드포인트는 속성 헤더로 시작하는 병합된 블록을 속성 전체로 취급하고 병합된 나머지 시스템 프롬프트를 포함한 모든 것을 삭제합니다.
* 게이트웨이가 시스템 콘텐츠를 재구성해야 하는 경우, [`CLAUDE_CODE_ATTRIBUTION_HEADER=0`](/docs/ko/env-vars)을 설정하여 Claude Code가 블록을 생략하도록 합니다. Anthropic 및 클라우드 제공자의 Claude 엔드포인트는 속성을 위해 블록을 읽으므로, 게이트웨이에서 제거하거나 이동하기보다는 클라이언트에서 생략합니다.

이 변수는 게이트웨이 및 타사 캐싱 호환성을 위해 존재하며, 개인정보 보호 제어로서가 아닙니다: 직접 연결에서 전체 요청은 어느 쪽이든 Anthropic API로 이미 전달됩니다. 이 두 조건이 모두 충족될 때, Claude Code는 변수를 `0`으로 설정한 경우에도 [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode) 분류기 요청에서 블록을 유지합니다:

* 요청이 `api.anthropic.com`으로 이동하고, `ANTHROPIC_BASE_URL`이 설정되지 않았거나 해당 호스트를 지정하며 타사 제공자가 선택되지 않았습니다.
* 활성 자격증명이 [Anthropic 프로필 또는 페더레이션 자격증명](/docs/ko/authentication#anthropic-profiles-and-federation-credentials)이 아닙니다.

분류기 요청은 Claude Code의 나머지 시스템 프롬프트를 건너뛰므로, 해당 요청에서 블록은 요청 본문에서 이를 Claude Code 트래픽으로 식별하는 유일한 마커입니다. 조건 중 하나라도 실패하면, LLM 게이트웨이를 통해, 타사 제공자에서, 또는 프로필이나 페더레이션 자격증명이 활성화된 경우, `0`을 설정하면 분류기 요청에서도 블록을 제거합니다. v2.1.229 이전에는 이 예외가 존재하지 않았습니다: `0`을 설정하면 해당 분류기 요청에서 블록을 제거했고, API가 식별되지 않은 요청을 거부했을 때, 자동 모드는 분류기로 전송한 모든 작업에서 실패했습니다.

Claude Code v2.1.181부터, 요청이 사용자 정의 기본 URL을 통해 라우팅될 때 블록은 대화의 수명 동안 안정적이므로, 전체 요청 본문을 기반으로 하는 게이트웨이 측 프롬프트 캐시는 이를 비활성화하지 않고도 작동하며, 게이트웨이가 전달하는 모든 제공자는 안정적인 프롬프트 접두사를 수신합니다. v2.1.181 이전에는 블록이 요청별 토큰을 포함했으므로 모든 요청에서 시스템 프롬프트의 시작이 변경되었습니다. 해당 버전에서 게이트웨이가 다음 중 하나를 수행하는 경우 `CLAUDE_CODE_ATTRIBUTION_HEADER=0`을 설정합니다:

* 요청 본문을 기반으로 하는 프롬프트 캐시를 구현합니다.
* Amazon Bedrock, Microsoft Foundry 또는 Google Cloud의 Agent Platform과 같은 타사 제공자로 요청을 전달하며, Anthropic Messages 형식 또는 제공자 자체 형식으로, 변경되는 접두사가 해당 제공자에서 프롬프트 캐시 재사용을 감소시킵니다.

<h2 id="feature-pass-through">
  기능 통과
</h2>

Claude Code는 `ANTHROPIC_BASE_URL` 게이트웨이를 Anthropic 형식 엔드포인트로 취급하고 `api.anthropic.com`으로 전송하는 베타 헤더 및 요청 본문 필드를 전송합니다. 단, 직접 연결을 위해 예약된 작은 진단 및 기본값 집합은 제외합니다. 이 집합은 릴리스에 따라 다르므로 그 내용에 의존하지 마십시오.

기능을 추가하는 본문 필드는 베타 헤더와 쌍을 이루며, 쌍은 함께 이동합니다. 헤더를 제거하면서 본문을 통과시키거나, Anthropic 형식 본문을 다른 스키마의 업스트림으로 전달하는 게이트웨이는 하드 `400` 오류를 생성합니다. 두 절반이 함께 없을 때만 기능이 조용히 꺼집니다. 콘텐츠 검사를 위해 요청 본문을 다시 쓰거나 수정하는 게이트웨이는 제거하는 것과 같은 방식으로 쌍을 손상시키므로, 수정하지 않고 검사합니다. 표는 기능이 쌍에서 벗어나는 경우를 기록합니다.

세분화된 도구 스트리밍은 직접 연결 기본값 중 하나입니다. 요청이 사용자 정의 기본 URL을 통해 라우팅될 때마다 기본적으로 꺼져 있으며, 개발자가 [`CLAUDE_CODE_ENABLE_FINE_GRAINED_TOOL_STREAMING=1`](/docs/ko/env-vars)을 설정할 때 게이트웨이가 이를 수신합니다.

| 기능                                                                                                                                                                                                                        | 헤더 및 본문 쌍                                                                                                                            | 손상될 때의 증상                                                                                                                 | 해결 방법                                                                                                  |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------- |
| [적응형 추론](/docs/ko/model-config#adjust-effort-level)                                                                                                                                                                            | 베타 헤더 없음. Claude Code는 Claude 4.6 이상에 대해 `thinking: {"type": "adaptive"}`를 전송하고, 게이트웨이 별칭과 같이 인식하지 못하는 모델 이름을 현재 모델로 취급하여 필드를 수신합니다. | 업스트림 모델 빌드가 이를 수락하지 않을 때 `thinking` 필드 또는 `adaptive` 태그의 이름을 지정하는 `400`                                                   | 업스트림을 업그레이드합니다. Opus 4.6 및 Sonnet 4.6에서 개발자는 대신 `CLAUDE_CODE_DISABLE_ADAPTIVE_THINKING=1`을 설정할 수 있습니다. |
| [컨텍스트 관리](https://platform.claude.com/docs/en/build-with-claude/context-editing)                                                                                                                                          | 컨텍스트 관리 베타 헤더는 `context_management` 본문 필드와 쌍을 이룹니다.                                                                                  | `Extra inputs are not permitted`를 포함한 `400`. 게이트웨이가 Anthropic 형식 요청을 수락하지만 Amazon Bedrock으로 전달할 때 일반적입니다.                 | 둘 다 전달하거나 [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/ko/env-vars)                                   |
| [확장 컨텍스트](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) 및 [인터리브된 사고](https://platform.claude.com/docs/en/build-with-claude/extended-thinking#interleaved-thinking) | 베타 헤더만, 본문 필드 없음                                                                                                                     | 헤더가 제거될 때 조용히 사용 불가능. 업스트림은 기능 요청을 보지 못합니다.                                                                               | `anthropic-beta`를 그대로 전달합니다.                                                                           |
| 베타 [도구 필드](https://platform.claude.com/docs/en/agents-and-tools/tool-use/overview)                                                                                                                                        | 도구 관련 베타 헤더는 `strict` 및 `defer_loading`과 같은 도구 스키마 필드와 쌍을 이룹니다.                                                                      | 본문이 헤더 없이 통과할 때 인식되지 않는 도구 스키마 필드의 이름을 지정하는 `400`                                                                         | 둘 다 전달하거나 [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](#disable-pre-release-capabilities)              |
| [노력](https://platform.claude.com/docs/en/build-with-claude/effort) 및 [구조화된 출력](https://platform.claude.com/docs/en/build-with-claude/structured-outputs)                                                                  | `output_config` 본문 필드는 노력, 구조화된 출력 형식 및 작업 예산 설정을 전달합니다. 각각은 자체 베타 헤더와 쌍을 이룹니다.                                                      | Amazon Bedrock 및 Google Cloud의 Agent Platform 업스트림에서 `output_config`의 이름을 지정하는 `400`. 종종 `Extra inputs are not permitted` | 필드와 헤더를 함께 전달합니다.                                                                                      |
| [프롬프트 캐싱](/docs/ko/prompt-caching)                                                                                                                                                                                             | 베타 쌍 없음. Claude Code는 `cache_control` 마커를 `system` 블록 및 `messages` 항목(대화 중간에 추가된 `role: "system"` 항목 포함)에 첨부합니다.                     | 오류 없음: 대화는 매 턴마다 캐시되지 않은 입력으로 청구되며, `usage`에서 높은 `input_tokens`과 거의 또는 전혀 캐시 활동이 없는 것으로 표시됩니다.                            | `cache_control`을 나타나는 모든 곳에서 그대로 전달하고, 블록 형식 `system` 또는 메시지 콘텐츠를 일반 문자열로 변환하지 마십시오.                   |
| [토큰 계산](https://platform.claude.com/docs/en/build-with-claude/token-counting)                                                                                                                                             | 베타 쌍 없음. `count_tokens` 엔드포인트를 사용합니다.                                                                                                | 오류 없음: Claude Code는 문자 기반 추정으로 돌아가므로 `/context`는 대략적인 계산을 표시합니다.                                                          | 정확한 토큰 계산을 위해 엔드포인트를 노출합니다.                                                                            |

`ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES` [변수](/docs/ko/model-config)는 제공자 구성에서만 모델 기능을 선언합니다: `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, 및 [`CLAUDE_CODE_USE_MANTLE`](/docs/ko/amazon-bedrock#use-the-mantle-endpoint). 이들은 `ANTHROPIC_BASE_URL` 게이트웨이 뒤에서 효과가 없습니다.

<h3 id="automatic-retry-and-error-forwarding">
  자동 재시도 및 오류 전달
</h3>

업스트림이 거부하는 것에 따라 Claude Code가 수행하는 작업이 달라집니다:

* 업스트림이 `thinking` 필드, 중간 대화 시스템 메시지, 또는 해당 메시지의 `cache_control` 마커를 거부할 때, Claude Code는 요청을 재시도하고 거부된 기능을 나머지 대화에 대해 비활성화합니다.
* 업스트림이 [사고 서명](https://platform.claude.com/docs/en/build-with-claude/extended-thinking)을 거부할 때, 블록이 `bound to a different conversation`이라는 `400`을 포함하여, Claude Code는 요청에서 이전 사고 블록을 제거하고, 재시도하며, 이후의 모든 요청에서 이들을 제외합니다. 새로운 응답은 여전히 사고를 포함합니다.
* 게이트웨이 또는 그 업스트림이 `tools`의 [어드바이저 도구](/docs/ko/advisor) 항목을 인식되지 않는 도구 유형으로 거부할 때, Claude Code는 해당 항목과 그 `anthropic-beta` 값 없이 요청을 한 번 재시도합니다. 이후 해당 기본 URL에 대한 요청은 Claude Code가 종료될 때까지 어드바이저를 제외하고, `/advisor`는 그 시간 동안 개발자에게 사용 불가능합니다. Claude Code는 `Input tag` 뒤에 도구 유형의 이름을 지정하는 `400` 또는 `422` 응답으로 이 거부를 인식합니다. 예를 들어 `Input tag 'advisor_20260301'`입니다. v2.1.280 이전에는 Claude Code가 이 거부를 재시도하지 않았습니다.
* Claude Code는 컨텍스트 관리 또는 도구 스키마 필드 거부를 재시도하지 않으므로, 해당 `400` 오류는 개발자에게 도달합니다.

`bound to a different conversation` 거부는 API의 [보존된 사고](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking) 확인에서 나오며, `system`, `tools`, 또는 이전 `messages` 콘텐츠가 사고를 생성한 요청과 다를 때 실패합니다. 해당 콘텐츠를 다시 쓰는 게이트웨이는 거부 자체를 야기할 수 있습니다. [라이브러리, 프록시, 및 게이트웨이](https://platform.claude.com/docs/en/build-with-claude/preserved-thinking#libraries-proxies-gateways)는 변경하지 않고 통과시켜야 할 것을 다룹니다.

재시도 로직은 업스트림의 오류 표현과 일치하므로 오류 응답 본문을 수정하지 않고 전달합니다. 업스트림 오류를 자체 봉투로 래핑하는 게이트웨이는 상태 코드를 유지하더라도 복구 경로를 손상시킵니다. 단, 봉투의 메시지가 안정적인 `capability_rejected:` 토큰을 포함하는 경우는 예외입니다. [Claude 앱 게이트웨이는 클라우드 제공자의 오류 표현을 위해 해당 토큰으로 대체합니다](/docs/ko/claude-apps-gateway-config#upstream-error-messages). 예를 들어 `capability_rejected: prompt_too_long`입니다.

<h3 id="disable-pre-release-capabilities">
  사전 릴리스 기능 비활성화
</h3>

`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`은 Claude Code가 모든 제공자(컨텍스트 관리 및 베타 도구 필드 포함)에서 사전 릴리스 기능 및 해당 본문 필드를 전송하는 것을 중지합니다. 이 변수는 모델에 의해 선택되는 적응형 추론에는 영향을 주지 않으며, 구독 인증이 요구하는 OAuth 기능을 억제하지 않습니다.

Claude Code v2.1.227 이상에서 조직은 [관리 설정](/docs/ko/managed-settings)을 통해 이 변수 아래에서 [MCP 도구 검색](/docs/ko/mcp#scale-with-mcp-tool-search)을 유지할 수 있습니다. Claude Code가 해당 재정의를 적용하여 전송하는 것은 연결 방식에 따라 다릅니다:

* 직접 연결 또는 `ANTHROPIC_BASE_URL`로 설정된 게이트웨이를 통해, Claude Code는 도구 검색 베타 헤더, `defer_loading` 도구 필드, 및 `tool_reference` 블록을 계속 전송하고 나머지는 제거합니다.
* 클라우드 제공자 또는 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway)를 통해 로그인한 경우, 재정의는 효과가 없습니다.

Claude Code가 전송하는 기능 집합은 릴리스에 따라 증가합니다. 현재 베타 헤더 문자열은 [베타 헤더 참조](https://platform.claude.com/docs/en/api/beta-headers)를 참조하십시오. 관찰된 목록에 고정하기보다는 새로운 Claude Code 릴리스에 대해 게이트웨이를 테스트합니다.

<h2 id="model-discovery">
  모델 검색
</h2>

`ANTHROPIC_BASE_URL`이 Anthropic Messages 형식을 노출하는 게이트웨이를 가리킬 때, Claude Code는 시작 시 게이트웨이의 `/v1/models` 엔드포인트를 쿼리하고 반환된 모델을 `/model` 선택기에 추가할 수 있습니다. 사용자 또는 관리자가 [`modelPicker`](/docs/ko/settings-reference#modelpicker) 라인업에서 `replaceBuiltInOptions`을 설정하면, Claude Code는 검색된 모델을 선택기에서 숨깁니다.

개발자는 자신의 환경 또는 관리되는 설정을 통해 [`CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1`](/docs/ko/env-vars)을 설정하여 이를 활성화합니다. 검색은 기본적으로 꺼져 있으므로 공유 API 키로 지원되는 게이트웨이가 키가 액세스할 수 있는 모든 모델을 모든 사용자에게 표시하지 않습니다.

<h3 id="when-discovery-runs">
  검색이 실행되는 경우
</h3>

검색은 Anthropic Messages 형식에만 적용됩니다. 다음의 경우 실행되지 않습니다:

* `ANTHROPIC_BASE_URL`도 설정되어 있더라도 `CLAUDE_CODE_USE_*` 제공자 변수가 설정된 경우
* `ANTHROPIC_BASE_URL`이 설정되지 않았거나 `api.anthropic.com`을 가리키는 경우

[비필수 트래픽이 꺼져 있을 때](/docs/ko/llm-gateway-connect#turn-off-traffic-outside-the-gateway-path) 검색이 계속 실행됩니다. 요청이 게이트웨이로만 이동하기 때문입니다. v2.1.257 이전에는 비필수 트래픽이 꺼져 있는 동안 검색이 실행되지 않았습니다.

<h3 id="request-and-response">
  요청 및 응답
</h3>

요청은 3초 타임아웃을 포함한 `GET /v1/models?limit=1000`이며, 모든 리디렉션은 자격 증명이 리디렉션 대상으로 유출되지 않도록 실패로 취급됩니다. 느리게 응답하거나 `/v1/models`를 리디렉션하는 게이트웨이 (예: `http`에서 `https`로)는 검색을 조용히 실패합니다. 구성된 기본 URL에서 직접 엔드포인트를 제공합니다.

느린 게이트웨이에 더 오래 걸리도록 하려면 [`CLAUDE_CODE_GATEWAY_MODEL_DISCOVERY_TIMEOUT_MS`](/docs/ko/env-vars#variables)를 설정합니다. 이 변수는 Claude Code v2.1.269 이상이 필요합니다.

Claude Code는 아래의 두 자격 증명 헤더로 검색 요청을 전송하고 값이 해결되지 않는 헤더는 생략합니다. 두 헤더를 모두 전송하려면 Claude Code v2.1.248 이상이 필요합니다. 이전 버전은 `ANTHROPIC_AUTH_TOKEN`이 설정되었을 때만 `Authorization`을 전송하고, 그렇지 않으면 `x-api-key`만 전송합니다.

* `Authorization`: `ANTHROPIC_AUTH_TOKEN`을 베어러 토큰으로, 그렇지 않으면 [`apiKeyHelper`](/docs/ko/llm-gateway-connect#rotate-credentials-with-apikeyhelper) 값을 베어러 토큰으로. 이 경우 Claude Code는 요청을 전송하기 전에 도우미가 반환될 때까지 기다립니다.
* `x-api-key`: Claude Code가 해결한 API 키 (예: `ANTHROPIC_API_KEY`). 도우미 값이 유일한 자격 증명일 때, 이 헤더도 이를 전달하므로 값이 두 헤더 모두에 도착합니다.

Claude Code는 또한 `ANTHROPIC_CUSTOM_HEADERS`의 모든 헤더를 전송합니다. 사용자 정의 헤더에 비어 있지 않은 값이 있으면, Claude Code는 같은 이름의 기본 제공 헤더 대신 이를 전송하며, 이름은 대소문자를 구분하지 않게 일치합니다.

어느 자격 증명 헤더의 값도 해결되지 않으면, Claude Code는 검색을 건너뛰고 `claude --debug` 세션의 디버그 로그에 `[gatewayDiscovery] skipped` 줄을 씁니다. `ANTHROPIC_CUSTOM_HEADERS`를 통해서만 자격 증명을 제공하면, Claude Code는 여전히 검색을 건너뜁니다.

Claude Code는 응답의 `data` 배열의 각 항목에서 `id`, 선택 사항인 `display_name`, 그리고 선택 사항인 `description`을 읽습니다:

```json theme={null}
{
  "data": [
    {
      "id": "claude-sonnet-4-6",
      "display_name": "Claude Sonnet 4.6",
      "description": "Default model for everyday coding tasks"
    },
    { "id": "claude-opus-4-8" }
  ]
}
```

Claude Code는 `id`에 `claude` 또는 `anthropic`이 문자열의 어디든 포함되어 있으면 항목을 유지하며, 대소문자를 구분하지 않게 일치하고 나머지는 무시합니다. `vertex_ai/claude-sonnet-4-6` 또는 `bedrock/anthropic.claude-sonnet-4-5`와 같은 제공자 접두사가 있는 ID는 필터를 통과합니다. 두 부분 문자열을 포함하지 않는 ID는 통과하지 않습니다. v2.1.223 이전에는 Claude Code가 `id`가 `claude` 또는 `anthropic`으로 시작할 때만 항목을 유지했으며, 이는 제공자 접두사가 있는 ID를 숨겼습니다.

<h3 id="picker-entries-and-caching">
  선택기 항목 및 캐싱
</h3>

선택기는 개발자가 Claude Code에서 `/model`을 실행할 때 열리는 대화형 모델 목록입니다. 각 검색된 항목은 게이트웨이가 `id`와 다른 항목을 전송할 때 `display_name`을 이름으로 사용합니다. 그렇지 않으면 항목은 Claude Code가 [`id`를 인식할 때](/docs/ko/model-config#customize-pinned-model-display-and-capabilities) 모델의 이름을 표시하고, 인식하지 못할 때 `id`를 표시합니다. 예를 들어, `id`가 `my-gateway-claude-sonnet-4-6`이고 `display_name`이 없는 항목은 `Sonnet 4.6`으로 나타납니다.

검색은 [`availableModels` 관리되는 설정](/docs/ko/settings-reference#availablemodels)이 허용하는 모델만 추가합니다.

각 항목은 또한 모델의 `description`을 한 줄로 축소하여 표시합니다. `description`이 없는 항목은 대신 "게이트웨이에서"를 읽습니다. v2.1.257 이전에는 모든 검색된 항목이 "게이트웨이에서"를 읽었습니다.

검색된 ID는 선택기에 이미 있는 행과 일치할 때 자신의 행을 얻지 않습니다:

* 같은 ID: 검색된 ID가 기존 행의 ID와 정확히 일치하거나, 두 ID가 같은 [Fable](/docs/ko/model-config#work-with-fable) 버전의 철자입니다.
* 기본 제공 별칭과 같은 모델: 검색된 명시적 ID가 기본 제공 별칭이 현재 해결되는 모델의 이름을 지을 때, 선택기는 별칭 행만 표시합니다. 예를 들어, `sonnet`이 `claude-sonnet-5`로 해결되는 동안, 검색된 `claude-sonnet-5`는 `sonnet` 행으로 축소되고, 검색된 `claude-sonnet-4-6`은 여전히 자신의 행을 얻습니다. v2.1.197 이전에는 Claude Code가 이러한 ID를 기본 제공 행으로 접지 않았으므로 `claude-sonnet-5`도 자신의 "게이트웨이에서" 행을 얻었습니다.

결과는 `~/.claude/cache/gateway-models.json` 또는 Windows의 `%USERPROFILE%\.claude\cache\gateway-models.json`으로 캐시되고 각 시작 시 새로 고쳐집니다. [`CLAUDE_CONFIG_DIR`](/docs/ko/env-vars)을 설정하면, 캐시는 대신 해당 디렉토리 아래에 있습니다. 요청이 실패하거나 게이트웨이가 `/v1/models`를 구현하지 않으면, 선택기는 이전 시작의 캐시된 목록 또는 기본 제공 모델 목록으로 돌아갑니다. 게이트웨이가 검색 필터와 일치하지 않는 별칭 아래에서 Claude 모델을 제공하면, 개발자는 [모델 구성](/docs/ko/model-config) 변수를 사용하여 해당 별칭을 수동으로 추가할 수 있습니다.

<h2 id="related-resources">
  관련 리소스
</h2>

게이트웨이 문서 집합의 나머지 부분 및 기본 API 참조:

* [게이트웨이 개요](/docs/ko/gateways): 게이트웨이가 무엇이고 Claude 앱 게이트웨이와 다른 제품 중에서 선택하는 방법
* [다른 LLM 게이트웨이](/docs/ko/llm-gateway): 조직이 실행하는 게이트웨이를 배포하는 방법 및 claude.ai 구독과 어떻게 상호작용하는지
* [조직을 위해 LLM 게이트웨이 배포](/docs/ko/llm-gateway-rollout): 이 가이드를 사용하는 관리자 체크리스트
* [Claude Code를 LLM 게이트웨이에 연결](/docs/ko/llm-gateway-connect): 개발자별 구성 및 문제 해결 표
* [베타 헤더 참조](https://platform.claude.com/docs/en/api/beta-headers): 현재 `anthropic-beta` 값 집합
* [Messages API](https://platform.claude.com/docs/en/api/messages): Anthropic 형식 게이트웨이가 구현하는 API 형식
