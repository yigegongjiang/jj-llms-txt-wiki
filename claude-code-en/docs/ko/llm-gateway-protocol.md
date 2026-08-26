> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 게이트웨이 프로토콜 참조

> Claude Code와 LLM 게이트웨이 간의 API 계약: 엔드포인트, 전달할 헤더 및 본문 필드, 필드가 제거될 때의 기능 저하, 비용 추적을 위한 속성 헤더, 모델 검색.

이 페이지는 Claude Code가 게이트웨이로 전송하는 요청을 문서화합니다. 여기에는 호출하는 엔드포인트, 게이트웨이가 전달해야 하는 헤더 및 본문 필드, 그리고 게이트웨이가 이를 수행하지 않을 때 작동을 멈추는 기능이 포함됩니다. 이 문서는 Claude Code와 함께 작동하도록 게이트웨이 제품을 구성하는 운영자를 위해 작성되었습니다.

실행 중인 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway)는 `GET /protocol`에서 이 계약의 기계 판독 가능한 버전을 제공하며, 동일한 전달 요구 사항과 SSO 로그인, 관리형 설정 전달, 원격 측정을 위한 Claude 앱 게이트웨이 특정 엔드포인트를 포함합니다. Claude 앱 게이트웨이는 CLI와 동일한 `claude` 바이너리에서 실행되므로, [Claude 앱 게이트웨이 빠른 시작](/docs/ko/claude-apps-gateway#quickstart)이 사양을 가져올 수 있는 실행 중인 인스턴스로 가는 가장 짧은 경로입니다.

<Note>
  * 조직을 위해 기존 또는 타사 게이트웨이를 배포하려면 [LLM 게이트웨이 배포](/docs/ko/llm-gateway-rollout)를 참조하십시오.
  * 제공받은 자격 증명으로 Claude Code를 게이트웨이에 인증하는 개별 개발자인 경우 [Claude Code를 LLM 게이트웨이에 연결](/docs/ko/llm-gateway-connect)을 참조하십시오.
</Note>

이 페이지는 다음을 다룹니다:

* [API 형식](#api-formats) 및 각각에 대해 제공할 엔드포인트
* [요청 헤더](#request-headers): 업스트림에 도달해야 하는 것과 게이트웨이가 사용할 수 있는 것
* [시스템 프롬프트 속성 블록](#system-prompt-attribution-block) 및 프롬프트 캐싱과의 상호작용
* [기능 통과](#feature-pass-through): 헤더 또는 본문 필드가 제거될 때 손상되는 것
* [모델 검색](#model-discovery)

이 페이지는 게이트웨이가 각 헤더 및 본문 필드로 수행하는 작업에 대해 두 가지 용어를 사용합니다:

* **변경 없이 전달**: 바이트 단위로 업스트림으로 전달
* **사용**: 게이트웨이가 라우팅, 속성 또는 추적을 위해 읽을 수 있으며 전달할 필요가 없음

변경 없이 전달로 표시되지 않은 모든 것은 사용하거나 무시할 수 있습니다.

<h2 id="api-formats">
  API 형식
</h2>

게이트웨이는 Claude Code 클라이언트에 다음 API 형식 중 최소 하나를 노출해야 합니다. Claude Code가 사용하는 형식은 클라이언트의 구성에 의해 결정됩니다. 아래 표의 선택 기준 열의 변수는 Claude Code를 해당 형식의 게이트웨이로 지정합니다. Google Cloud의 Agent Platform은 Google Cloud의 Claude 엔드포인트이며, 이전에는 Vertex AI였습니다. 변수 이름은 `VERTEX` 철자를 유지합니다.

| 형식                                      | 선택 기준                                                         | 엔드포인트                                                                 | 변경 없이 전달                                                                     |
| :-------------------------------------- | :------------------------------------------------------------ | :-------------------------------------------------------------------- | :--------------------------------------------------------------------------- |
| Anthropic Messages                      | `ANTHROPIC_BASE_URL`                                          | `/v1/messages`, `/v1/messages/count_tokens` (선택 사항)                   | `anthropic-beta` 및 `anthropic-version` 요청 헤더                                 |
| Amazon Bedrock InvokeModel              | `ANTHROPIC_BEDROCK_BASE_URL` with `CLAUDE_CODE_USE_BEDROCK=1` | `/model/{model}/invoke`, `/model/{model}/invoke-with-response-stream` | `anthropic_beta` 및 `anthropic_version` 요청 본문 필드                              |
| Google Cloud의 Agent Platform rawPredict | `ANTHROPIC_VERTEX_BASE_URL` with `CLAUDE_CODE_USE_VERTEX=1`   | `:rawPredict`, `:streamRawPredict`, `count-tokens:rawPredict` (선택 사항) | `anthropic-beta` 및 `anthropic-version` 요청 헤더, 및 `anthropic_version` 요청 본문 필드 |

<h3 id="foundry-and-claude-platform-on-aws">
  Foundry 및 AWS의 Claude Platform
</h3>

Microsoft Foundry 및 [AWS의 Claude Platform](/docs/ko/claude-platform-on-aws)은 Anthropic Messages 형식을 구현합니다. Claude Code는 자체 변수인 `ANTHROPIC_FOUNDRY_BASE_URL` 및 `ANTHROPIC_AWS_BASE_URL`을 통해 이들로 라우팅하지만, 둘 중 하나를 앞에 두는 게이트웨이는 위의 Anthropic Messages 행을 구현합니다. AWS의 Claude Platform을 앞에 두는 게이트웨이는 또한 `anthropic-workspace-id` 헤더를 전달해야 하며, [해당 플랫폼은 모든 요청에 이를 요구합니다](/docs/ko/claude-platform-on-aws).

<h3 id="optional-endpoints-and-startup-traffic">
  선택 사항 엔드포인트 및 시작 트래픽
</h3>

토큰 계산 엔드포인트는 유일한 선택 사항입니다. 이들이 없을 때 Claude Code는 컨텍스트 사용을 로컬에서 추정합니다. 추론 요청은 `/v1/messages?beta=true`로 게시되므로 전체 URL이 아닌 경로와 일치합니다. Google Cloud의 Agent Platform 메서드 접미사는 게시자 모델 경로에 첨부됩니다. 예: `/projects/{project}/locations/{location}/publishers/anthropic/models/{model}:streamRawPredict`.

게이트웨이는 또한 아무것도 손상시키지 않고 거부할 수 있는 최선의 노력 시작 트래픽을 봅니다: `HEAD /` 연결 프로브, 및 Amazon Bedrock 형식 게이트웨이에서 `GET /inference-profiles?type=SYSTEM_DEFINED` 요청.

<h3 id="streaming">
  스트리밍
</h3>

추론 응답은 스트리밍되어야 합니다. Claude Code는 서버 전송 이벤트를 도착하는 대로 사용하므로, 완전한 응답을 버퍼링한 후 릴레이하는 게이트웨이는 클라이언트를 정지시킵니다.

<h3 id="format-mismatch-with-the-upstream">
  업스트림과의 형식 불일치
</h3>

클라이언트가 사용하는 형식은 게이트웨이가 수신하는 것을 결정합니다. 일반적인 실패 모드는 클라이언트가 게이트웨이로 전송하는 형식과 뒤에 있는 업스트림 제공자가 수락하는 형식 간의 불일치입니다.

* 클라이언트가 Amazon Bedrock 또는 Google Cloud의 Agent Platform 형식을 사용할 때, Claude Code는 해당 제공자가 수락하는 전체 기능 집합의 부분 집합만 전송합니다.
* 클라이언트가 Anthropic Messages 형식을 사용할 때, Claude Code는 게이트웨이가 Amazon Bedrock 또는 Google Cloud의 Agent Platform 업스트림으로 전달하더라도 전체 집합을 전송합니다.

그 차이를 연결하는 것은 게이트웨이의 작업입니다. [기능 통과](#feature-pass-through)는 그렇지 않을 때 손상되는 것을 설명합니다.

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

<h3 id="forward-as-open-lists">
  개방형 목록으로 전달
</h3>

헤더 및 본문 필드를 닫힌 목록이 아닌 개방형 목록으로 취급합니다. Claude Code는 릴리스에 따라 기능을 얻으며, 이들은 새로운 `anthropic-beta` 값, 새로운 요청 본문 필드, 그리고 때때로 새로운 `anthropic-*` 또는 `x-claude-code-*` 헤더로 도착합니다.

Anthropic 형식 업스트림으로 전달할 때, 오늘 보는 것들을 허용 목록에 추가하기보다는 `anthropic-*` 요청 헤더 및 요청 본문 필드를 변경 없이 통과시킵니다. 관찰된 목록에 고정된 게이트웨이는 다음 기능의 헤더 또는 필드를 제거하고 이를 도입하는 릴리스에서 손상시킵니다.

예외는 Amazon Bedrock 또는 Google Cloud의 Agent Platform과 같은 비 Anthropic 업스트림입니다. 여기서 스키마 차이를 연결하는 것은 게이트웨이의 작업입니다. [기능 통과](#feature-pass-through)를 참조하십시오.

<h2 id="system-prompt-attribution-block">
  시스템 프롬프트 속성 블록
</h2>

Claude Code는 클라이언트 버전과 대화에서 파생된 지문을 포함하는 짧은 속성 블록을 시스템 프롬프트 앞에 추가합니다. `api.anthropic.com` 엔드포인트는 변경되지 않은 상태로 첫 번째 시스템 블록으로 도착할 때 처리 전에 블록을 제거하므로 자사 프롬프트 캐싱에 영향을 주지 않습니다. 다른 업스트림은 프롬프트의 일부로 수신합니다.

제거는 위치 기반이므로 게이트웨이가 `system` 배열을 변경되지 않은 상태로 전달할 때만 작동합니다. 다른 시스템 콘텐츠를 잃지 않으면서 블록을 프롬프트에서 제외하려면:

* 받은 `system` 배열을 정확히 전달하고 블록을 먼저 유지합니다: 다른 시스템 블록을 앞에 추가하거나, 배열을 재정렬하거나, 단일 문자열로 변환하면 제거가 실패하고 블록이 모델과 프롬프트 캐시 키에 도달합니다.
* 블록을 자체 배열 항목에 유지합니다: 엔드포인트는 속성 헤더로 시작하는 병합된 블록을 속성 전체로 취급하고 병합된 나머지 시스템 프롬프트를 포함한 모든 것을 삭제합니다.
* 게이트웨이가 시스템 콘텐츠를 재구성해야 하는 경우, [`CLAUDE_CODE_ATTRIBUTION_HEADER=0`](/docs/ko/env-vars)을 설정하여 Claude Code가 블록을 생략하도록 합니다. Anthropic 및 클라우드 제공자의 Claude 엔드포인트는 속성을 위해 블록을 읽으므로, 게이트웨이에서 제거하거나 이동하기보다는 클라이언트에서 생략합니다.

변경되지 않은 상태로 엔드포인트에 도달하는 요청은 영향을 받지 않습니다.

Claude Code v2.1.181부터, 요청이 사용자 정의 기본 URL을 통해 라우팅될 때 블록은 대화의 수명 동안 안정적이므로, 전체 요청 본문을 기반으로 하는 게이트웨이 측 프롬프트 캐시는 이를 비활성화하지 않고도 작동합니다. v2.1.181 이전에는 블록이 요청별 토큰을 포함했습니다. 해당 버전에서 게이트웨이가 이러한 캐시를 구현하면 `CLAUDE_CODE_ATTRIBUTION_HEADER=0`을 설정합니다.

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
| 베타 [도구 필드](https://platform.claude.com/docs/en/agents-and-tools/tool-use/overview)                                                                                                                                        | 도구 관련 베타 헤더는 `strict` 및 `defer_loading`과 같은 도구 스키마 필드와 쌍을 이룹니다.                                                                      | 본문이 헤더 없이 통과할 때 인식되지 않는 도구 스키마 필드의 이름을 지정하는 `400`                                                                         | 둘 다 전달하거나 `CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`                                                   |
| [노력](https://platform.claude.com/docs/en/build-with-claude/effort) 및 [구조화된 출력](https://platform.claude.com/docs/en/build-with-claude/structured-outputs)                                                                  | `output_config` 본문 필드는 노력, 구조화된 출력 형식 및 작업 예산 설정을 전달합니다. 각각은 자체 베타 헤더와 쌍을 이룹니다.                                                      | Amazon Bedrock 및 Google Cloud의 Agent Platform 업스트림에서 `output_config`의 이름을 지정하는 `400`. 종종 `Extra inputs are not permitted` | 필드와 헤더를 함께 전달합니다.                                                                                      |
| [토큰 계산](https://platform.claude.com/docs/en/build-with-claude/token-counting)                                                                                                                                             | 베타 쌍 없음. `count_tokens` 엔드포인트를 사용합니다.                                                                                                | Claude Code는 컨텍스트 사용을 로컬에서 추정하는 것으로 돌아갑니다.                                                                                | 정확한 계산을 원하면 엔드포인트를 노출합니다.                                                                              |

`ANTHROPIC_DEFAULT_*_MODEL_SUPPORTED_CAPABILITIES` [변수](/docs/ko/model-config)는 제공자 구성에서만 모델 기능을 선언합니다: `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX`, `CLAUDE_CODE_USE_FOUNDRY`, 및 [`CLAUDE_CODE_USE_MANTLE`](/docs/ko/amazon-bedrock#use-the-mantle-endpoint). 이들은 `ANTHROPIC_BASE_URL` 게이트웨이 뒤에서 효과가 없습니다.

<h3 id="automatic-retry-and-error-forwarding">
  자동 재시도 및 오류 전달
</h3>

Claude Code는 일부 업스트림 거부 후 자동으로 재시도하고 거부된 기능을 나머지 대화에 대해 비활성화합니다. `thinking` 필드 거부, [사고 서명](https://platform.claude.com/docs/en/build-with-claude/extended-thinking) 거부, 및 중간 대화 시스템 메시지 거부는 모두 이러한 방식으로 복구됩니다. 컨텍스트 관리 및 도구 스키마 필드 거부는 재시도하지 않습니다. 해당 `400` 오류는 개발자에게 도달합니다.

재시도 로직은 업스트림의 오류 표현과 일치하므로 오류 응답 본문을 수정하지 않고 전달합니다. 업스트림 오류를 자체 봉투로 래핑하는 게이트웨이는 상태 코드를 유지하더라도 복구 경로를 손상시킵니다.

<h3 id="disable-pre-release-capabilities">
  사전 릴리스 기능 비활성화
</h3>

`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`은 Claude Code가 모든 제공자 (컨텍스트 관리 및 베타 도구 필드 포함)에서 사전 릴리스 기능 및 해당 본문 필드를 전송하는 것을 중지합니다. 이는 모델에 의해 선택되는 적응형 추론에는 영향을 주지 않으며, 구독 인증이 요구하는 OAuth 기능을 억제하지 않습니다.

Claude Code가 전송하는 기능 집합은 릴리스에 따라 증가합니다. 현재 베타 헤더 문자열은 [베타 헤더 참조](https://platform.claude.com/docs/en/api/beta-headers)를 참조하십시오. 관찰된 목록에 고정하기보다는 새로운 Claude Code 릴리스에 대해 게이트웨이를 테스트합니다.

<h2 id="model-discovery">
  모델 검색
</h2>

`ANTHROPIC_BASE_URL`이 Anthropic Messages 형식을 노출하는 게이트웨이를 가리킬 때, Claude Code는 시작 시 게이트웨이의 `/v1/models` 엔드포인트를 쿼리하고 반환된 모델을 `/model` 선택기에 추가할 수 있습니다.

개발자는 자신의 환경 또는 관리되는 설정을 통해 [`CLAUDE_CODE_ENABLE_GATEWAY_MODEL_DISCOVERY=1`](/docs/ko/env-vars)을 설정하여 이를 활성화합니다. 검색은 기본적으로 꺼져 있으므로 공유 API 키로 지원되는 게이트웨이가 키가 액세스할 수 있는 모든 모델을 모든 사용자에게 표시하지 않습니다. 이를 위해서는 Claude Code v2.1.129 이상이 필요합니다.

<h3 id="when-discovery-runs">
  검색이 실행되는 경우
</h3>

검색은 Anthropic Messages 형식에만 적용됩니다. 다음의 경우 실행되지 않습니다:

* `ANTHROPIC_BASE_URL`도 설정되어 있더라도 `CLAUDE_CODE_USE_*` 제공자 변수가 설정된 경우
* `ANTHROPIC_BASE_URL`이 설정되지 않았거나 `api.anthropic.com`을 가리키는 경우
* 비필수 트래픽이 비활성화된 경우. [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/ko/env-vars) 또는 조직 정책을 통해

<h3 id="request-and-response">
  요청 및 응답
</h3>

요청은 3초 타임아웃을 포함한 `GET /v1/models?limit=1000`이며, 모든 리디렉션은 자격 증명이 리디렉션 대상으로 유출되지 않도록 실패로 취급됩니다. 느리게 응답하거나 `/v1/models`를 리디렉션하는 게이트웨이 (예: `http`에서 `https`로)는 검색을 조용히 실패합니다. 구성된 기본 URL에서 직접 엔드포인트를 제공합니다.

검색 요청은 정확히 하나의 자격 증명 헤더를 전송합니다:

* 설정된 경우 `ANTHROPIC_AUTH_TOKEN`을 베어러 토큰으로
* 그렇지 않으면 [`apiKeyHelper`](/docs/ko/llm-gateway-connect#rotate-credentials-with-apikeyhelper) 값을 포함한 해결된 API 키를 `x-api-key` 헤더에

이는 두 헤더 모두에 도우미 값을 전송하는 추론 요청과 다릅니다. `/v1/models`를 인증하는 게이트웨이는 도우미 배포를 위해 `x-api-key`를 수락해야 합니다. `ANTHROPIC_CUSTOM_HEADERS`의 모든 헤더도 포함됩니다.

Claude Code는 응답의 `data` 배열의 각 항목에서 `id` 및 선택 사항인 `display_name`을 읽고, `id`가 `claude` 또는 `anthropic`으로 시작하지 않는 항목을 무시합니다:

```json theme={null}
{
  "data": [
    { "id": "claude-sonnet-4-6", "display_name": "Claude Sonnet 4.6" },
    { "id": "claude-opus-4-8" }
  ]
}
```

<h3 id="picker-entries-and-caching">
  선택기 항목 및 캐싱
</h3>

선택기는 개발자가 Claude Code에서 `/model`을 실행할 때 열리는 대화형 모델 목록입니다. 각 검색된 항목은 "게이트웨이에서" 레이블이 지정되고 제공된 경우 `display_name`을 사용합니다. [`availableModels` 관리되는 설정](/docs/ko/settings#available-settings)은 검색이 추가할 수 있는 것을 제한합니다.

검색된 ID는 선택기에 이미 있는 행과 정확히 일치하거나 검색된 ID와 기존 ID가 모두 [Fable](/docs/ko/model-config#work-with-fable-5)로 해결될 때만 건너뜁니다. Claude Code v2.1.197부터, 검색된 명시적 ID는 둘 다 동일한 모델로 해결될 때 기본 제공 항목으로 접혀집니다. 기본 제공 행은 `sonnet`과 같은 별칭으로 키가 지정되므로, 별칭이 현재 해결되는 모델의 명시적 ID (예: `claude-sonnet-5`)는 `sonnet` 행으로 축소되는 반면, 별칭이 해결되지 않는 ID (예: `claude-sonnet-4-6`)는 기본 제공 항목 옆에 자체 "게이트웨이에서" 행을 추가합니다.

결과는 `~/.claude/cache/gateway-models.json` 또는 Windows의 `%USERPROFILE%\.claude\cache\gateway-models.json`으로 캐시되고 각 시작 시 새로 고쳐집니다. 요청이 실패하거나 게이트웨이가 `/v1/models`를 구현하지 않으면, 선택기는 이전 시작의 캐시된 목록 또는 기본 제공 모델 목록으로 돌아갑니다. 게이트웨이가 검색 필터와 일치하지 않는 별칭 아래에서 Claude 모델을 제공하면, 개발자는 [모델 구성](/docs/ko/model-config) 변수를 사용하여 해당 별칭을 수동으로 추가할 수 있습니다.

<h2 id="related-resources">
  관련 리소스
</h2>

게이트웨이 문서 집합의 나머지 부분 및 기본 API 참조:

* [게이트웨이 개요](/docs/ko/gateways): 게이트웨이가 무엇이고 Claude 앱 게이트웨이와 다른 제품 중에서 선택하는 방법
* [다른 LLM 게이트웨이](/docs/ko/llm-gateway): 조직이 실행하는 게이트웨이를 배포하는 방법 및 claude.ai 구독과 어떻게 상호작용하는지
* [조직을 위해 LLM 게이트웨이 배포](/docs/ko/llm-gateway-rollout): 이 계약을 사용하는 관리자 체크리스트
* [Claude Code를 LLM 게이트웨이에 연결](/docs/ko/llm-gateway-connect): 개발자별 구성 및 문제 해결 표
* [베타 헤더 참조](https://platform.claude.com/docs/en/api/beta-headers): 현재 `anthropic-beta` 값 집합
* [Messages API](https://platform.claude.com/docs/en/api/messages): Anthropic 형식 게이트웨이가 구현하는 API 형식
