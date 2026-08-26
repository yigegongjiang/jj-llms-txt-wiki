> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 모니터링

> Claude Code에 대한 OpenTelemetry를 활성화하고 구성하는 방법을 알아봅니다.

OpenTelemetry(OTel)를 통해 원격 측정 데이터를 내보내 조직 전체에서 Claude Code 사용, 비용 및 도구 활동을 추적합니다. Claude Code는 표준 메트릭 프로토콜을 통해 메트릭을 시계열 데이터로 내보내고, 로그/이벤트 프로토콜을 통해 이벤트를 내보내며, 선택적으로 [추적 프로토콜](#traces-beta)을 통해 분산 추적을 내보냅니다. 메트릭, 로그 및 추적 백엔드를 구성하여 모니터링 요구 사항과 일치하도록 합니다.

<h2 id="quick-start">
  빠른 시작
</h2>

환경 변수를 사용하여 OpenTelemetry를 구성합니다:

```bash theme={null}
# 1. 원격 측정 활성화
export CLAUDE_CODE_ENABLE_TELEMETRY=1

# 2. 내보내기 선택 (둘 다 선택 사항 - 필요한 것만 구성)
export OTEL_METRICS_EXPORTER=otlp       # 옵션: otlp, prometheus, console, none
export OTEL_LOGS_EXPORTER=otlp          # 옵션: otlp, console, none

# 3. OTLP 엔드포인트 구성 (OTLP 내보내기용)
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# 4. 인증 설정 (필요한 경우)
export OTEL_EXPORTER_OTLP_HEADERS="Authorization=Bearer your-token"

# 5. 디버깅용: 내보내기 간격 단축
export OTEL_METRIC_EXPORT_INTERVAL=10000  # 10초 (기본값: 60000ms)
export OTEL_LOGS_EXPORT_INTERVAL=5000     # 5초 (기본값: 5000ms)

# 6. Claude Code 실행
claude
```

<Note>
  기본 내보내기 간격은 메트릭의 경우 60초, 로그의 경우 5초입니다. 설정 중에 디버깅 목적으로 더 짧은 간격을 사용할 수 있습니다. 프로덕션 사용을 위해 이를 재설정하는 것을 잊지 마세요.
</Note>

전체 구성 옵션은 [OpenTelemetry 사양](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/protocol/exporter.md#configuration-options)을 참조하세요.

<h2 id="administrator-configuration">
  관리자 구성
</h2>

관리자는 [관리 설정 파일](/docs/ko/settings#settings-files)을 통해 모든 사용자에 대한 OpenTelemetry 설정을 구성할 수 있습니다. 이를 통해 조직 전체에서 원격 측정 설정을 중앙에서 제어할 수 있습니다. 설정이 적용되는 방식에 대한 자세한 내용은 [설정 우선순위](/docs/ko/settings#settings-precedence)를 참조하세요.

관리 설정 구성 예:

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
    "OTEL_METRICS_EXPORTER": "otlp",
    "OTEL_LOGS_EXPORTER": "otlp",
    "OTEL_EXPORTER_OTLP_PROTOCOL": "grpc",
    "OTEL_EXPORTER_OTLP_ENDPOINT": "http://collector.example.com:4317",
    "OTEL_EXPORTER_OTLP_HEADERS": "Authorization=Bearer example-token"
  }
}
```

<Note>
  관리 설정은 MDM(Mobile Device Management) 또는 기타 장치 관리 솔루션을 통해 배포할 수 있습니다. 관리 설정 파일에 정의된 환경 변수는 높은 우선순위를 가지며 사용자가 재정의할 수 없습니다.
</Note>

Claude Code는 `OTEL_*` 환경 변수를 Bash 도구, 훅, MCP 서버 및 언어 서버를 포함하여 생성하는 하위 프로세스에 전달하지 않습니다. Bash 도구를 통해 실행하는 OpenTelemetry 계측 애플리케이션은 Claude Code의 내보내기 엔드포인트 또는 헤더를 상속하지 않으므로 해당 애플리케이션이 자신의 원격 측정을 내보내야 하는 경우 명령에서 직접 이러한 변수를 설정합니다.

<h2 id="configuration-details">
  구성 세부 정보
</h2>

<h3 id="common-configuration-variables">
  일반적인 구성 변수
</h3>

| 환경 변수                                               | 설명                                                                                                                                                                                                                                             | 예제 값                                                                        |
| --------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| `CLAUDE_CODE_ENABLE_TELEMETRY`                      | 원격 측정 수집 활성화 (필수)                                                                                                                                                                                                                              | `1`                                                                         |
| `OTEL_METRICS_EXPORTER`                             | 메트릭 내보내기 유형 (쉼표로 구분). `none`을 사용하여 비활성화                                                                                                                                                                                                        | `console`, `otlp`, `prometheus`, `none`                                     |
| `OTEL_LOGS_EXPORTER`                                | 로그/이벤트 내보내기 유형 (쉼표로 구분). `none`을 사용하여 비활성화                                                                                                                                                                                                     | `console`, `otlp`, `none`                                                   |
| `OTEL_EXPORTER_OTLP_PROTOCOL`                       | OTLP 내보내기 프로토콜 (모든 신호에 적용)                                                                                                                                                                                                                     | `grpc`, `http/json`, `http/protobuf`                                        |
| `OTEL_EXPORTER_OTLP_ENDPOINT`                       | 모든 신호에 대한 OTLP 수집기 엔드포인트                                                                                                                                                                                                                       | `http://localhost:4317`                                                     |
| `OTEL_EXPORTER_OTLP_METRICS_PROTOCOL`               | 메트릭 프로토콜 (일반 설정 재정의)                                                                                                                                                                                                                           | `grpc`, `http/json`, `http/protobuf`                                        |
| `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`               | OTLP 메트릭 엔드포인트 (일반 설정 재정의)                                                                                                                                                                                                                     | `http://localhost:4318/v1/metrics`                                          |
| `OTEL_EXPORTER_OTLP_LOGS_PROTOCOL`                  | 로그 프로토콜 (일반 설정 재정의)                                                                                                                                                                                                                            | `grpc`, `http/json`, `http/protobuf`                                        |
| `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT`                  | OTLP 로그 엔드포인트 (일반 설정 재정의)                                                                                                                                                                                                                      | `http://localhost:4318/v1/logs`                                             |
| `OTEL_EXPORTER_OTLP_HEADERS`                        | OTLP용 인증 헤더                                                                                                                                                                                                                                    | `Authorization=Bearer token`                                                |
| `OTEL_METRIC_EXPORT_INTERVAL`                       | 내보내기 간격 (밀리초 단위, 기본값: 60000)                                                                                                                                                                                                                   | `5000`, `60000`                                                             |
| `OTEL_LOGS_EXPORT_INTERVAL`                         | 로그 내보내기 간격 (밀리초 단위, 기본값: 5000)                                                                                                                                                                                                                 | `1000`, `10000`                                                             |
| `OTEL_LOG_USER_PROMPTS`                             | 사용자 프롬프트 콘텐츠 로깅 활성화 (기본값: 비활성화)                                                                                                                                                                                                                | `1`로 활성화                                                                    |
| `OTEL_LOG_ASSISTANT_RESPONSES`                      | `assistant_response` 이벤트에서 어시스턴트 응답 텍스트 로깅 활성화 (기본값: 비활성화). 설정되지 않으면 `OTEL_LOG_USER_PROMPTS`의 값으로 폴백됩니다. Claude Code v2.1.193 이상 필요                                                                                                            | `1`로 활성화, `0`으로 수정된 상태 유지                                                   |
| `OTEL_LOG_TOOL_DETAILS`                             | 도구 이벤트 및 추적 스팬 속성에서 도구 매개변수 및 입력 인수 로깅 활성화: Bash 명령, MCP 서버 및 도구 이름, 스킬 이름 및 도구 입력. 또한 `user_prompt` 이벤트에서 사용자 정의, 플러그인 및 MCP 명령 이름을 활성화합니다 (기본값: 비활성화)                                                                                        | `1`로 활성화                                                                    |
| `OTEL_LOG_TOOL_CONTENT`                             | 스팬 이벤트에서 도구 입력 및 출력 콘텐츠 로깅 활성화 (기본값: 비활성화). [추적](#traces-beta)이 필요합니다. 콘텐츠는 60KB에서 잘립니다                                                                                                                                                        | `1`로 활성화                                                                    |
| `OTEL_LOG_RAW_API_BODIES`                           | 전체 Anthropic Messages API 요청 및 응답 JSON을 `api_request_body` / `api_response_body` 로그 이벤트로 내보냅니다 (기본값: 비활성화). 본문에는 전체 대화 기록이 포함됩니다. 이를 활성화하면 `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_TOOL_DETAILS` 및 `OTEL_LOG_TOOL_CONTENT`가 공개할 모든 것에 동의하는 것을 의미합니다 | `1`로 60KB에서 잘린 인라인 본문, 또는 `file:<dir>`로 디스크의 잘리지 않은 본문과 이벤트의 `body_ref` 포인터 |
| `OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE` | 메트릭 시간성 선호도 (기본값: `delta`). 백엔드가 누적 시간성을 예상하는 경우 `cumulative`로 설정                                                                                                                                                                              | `delta`, `cumulative`                                                       |
| `CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`       | 동적 헤더 새로 고침 간격 (기본값: 1740000ms / 29분)                                                                                                                                                                                                          | `900000`                                                                    |

<h3 id="mtls-authentication">
  mTLS 인증
</h3>

OTLP 내보내기를 위한 클라이언트 인증서를 구성하는 방법은 해당 신호에 사용되는 OTLP 프로토콜에 따라 다르며, `OTEL_EXPORTER_OTLP_PROTOCOL` 또는 신호별 재정의를 통해 설정됩니다. 동일한 구성이 메트릭, 로그 및 추적에 적용됩니다.

| 프로토콜                         | 클라이언트 인증서 변수                                                                                                                                          | 수집기의 CA 신뢰                       |
| :--------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------- |
| `http/protobuf`, `http/json` | `CLAUDE_CODE_CLIENT_CERT`, `CLAUDE_CODE_CLIENT_KEY` 및 선택적으로 `CLAUDE_CODE_CLIENT_KEY_PASSPHRASE`. [네트워크 구성](/docs/ko/network-config#mtls-authentication) 참조 | `NODE_EXTRA_CA_CERTS`            |
| `grpc`                       | `OTEL_EXPORTER_OTLP_CLIENT_KEY` 및 `OTEL_EXPORTER_OTLP_CLIENT_CERTIFICATE`, 또는 신호별 인증서를 사용하기 위한 `OTEL_EXPORTER_OTLP_METRICS_CLIENT_KEY`와 같은 신호별 변형     | `OTEL_EXPORTER_OTLP_CERTIFICATE` |

`grpc`의 경우 OpenTelemetry SDK는 표준 OTLP 변수를 직접 읽으므로 신호별 메트릭 변수를 설정하는 기존 구성은 계속 작동합니다.

<h3 id="metrics-cardinality-control">
  메트릭 카디널리티 제어
</h3>

다음 환경 변수는 카디널리티를 관리하기 위해 메트릭에 포함되는 속성을 제어합니다:

| 환경 변수                                      | 설명                                                 | 기본값     | 비활성화 예  |
| ------------------------------------------ | -------------------------------------------------- | ------- | ------- |
| `OTEL_METRICS_INCLUDE_SESSION_ID`          | 메트릭에 session.id 속성 포함                              | `true`  | `false` |
| `OTEL_METRICS_INCLUDE_VERSION`             | 메트릭에 app.version 속성 포함                             | `false` | `true`  |
| `OTEL_METRICS_INCLUDE_ACCOUNT_UUID`        | 메트릭에 user.account\_uuid 및 user.account\_id 속성 포함   | `true`  | `false` |
| `OTEL_METRICS_INCLUDE_ENTRYPOINT`          | 메트릭에 app.entrypoint 속성 포함                          | `false` | `true`  |
| `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` | `OTEL_RESOURCE_ATTRIBUTES`의 키를 메트릭 데이터포인트의 속성으로 포함 | `true`  | `false` |

이러한 변수는 메트릭의 카디널리티를 제어하는 데 도움이 되며, 이는 메트릭 백엔드의 저장소 요구 사항 및 쿼리 성능에 영향을 미칩니다. 낮은 카디널리티는 일반적으로 더 나은 성능과 낮은 저장소 비용을 의미하지만 분석을 위한 세분화된 데이터는 적습니다.

<h3 id="traces-beta">
  추적 (베타)
</h3>

분산 추적은 각 사용자 프롬프트를 해당 프롬프트가 트리거하는 API 요청 및 도구 실행에 연결하는 스팬을 내보내므로 추적 백엔드에서 전체 요청을 단일 추적으로 볼 수 있습니다.

추적은 기본적으로 꺼져 있습니다. 활성화하려면 `CLAUDE_CODE_ENABLE_TELEMETRY=1` 및 `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1`을 모두 설정한 다음 `OTEL_TRACES_EXPORTER`를 설정하여 스팬을 보낼 위치를 선택합니다. 추적은 엔드포인트, 프로토콜, 헤더 및 [mTLS](#mtls-authentication)에 대해 [일반적인 OTLP 구성](#common-configuration-variables)을 재사용합니다.

| 환경 변수                                 | 설명                                                    | 예제 값                                 |
| ------------------------------------- | ----------------------------------------------------- | ------------------------------------ |
| `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` | 스팬 추적 활성화 (필수). `ENABLE_ENHANCED_TELEMETRY_BETA`도 허용됨 | `1`                                  |
| `OTEL_TRACES_EXPORTER`                | 추적 내보내기 유형 (쉼표로 구분). `none`을 사용하여 비활성화                | `console`, `otlp`, `none`            |
| `OTEL_EXPORTER_OTLP_TRACES_PROTOCOL`  | 추적 프로토콜 (`OTEL_EXPORTER_OTLP_PROTOCOL` 재정의)           | `grpc`, `http/json`, `http/protobuf` |
| `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`  | OTLP 추적 엔드포인트 (`OTEL_EXPORTER_OTLP_ENDPOINT` 재정의)     | `http://localhost:4318/v1/traces`    |
| `OTEL_TRACES_EXPORT_INTERVAL`         | 스팬 배치 내보내기 간격 (밀리초 단위, 기본값: 5000)                     | `1000`, `10000`                      |

스팬은 기본적으로 사용자 프롬프트 텍스트, 도구 입력 세부 정보 및 도구 콘텐츠를 수정합니다. `OTEL_LOG_USER_PROMPTS=1`, `OTEL_LOG_TOOL_DETAILS=1` 및 `OTEL_LOG_TOOL_CONTENT=1`을 설정하여 포함합니다.

추적이 활성화되면 Bash 및 PowerShell 하위 프로세스는 활성 도구 실행 스팬의 W3C 추적 컨텍스트를 포함하는 `TRACEPARENT` 환경 변수를 자동으로 상속합니다. 이를 통해 `TRACEPARENT`를 읽는 모든 하위 프로세스가 자신의 스팬을 동일한 추적 아래에 부모로 지정할 수 있으므로 Claude가 실행하는 스크립트 및 명령을 통한 엔드투엔드 분산 추적이 가능합니다.

추적이 활성화되고 Claude Code가 Anthropic API에 직접 연결되어 있으면 각 모델 요청은 `claude_code.llm_request` 스팬의 컨텍스트로 설정된 W3C `traceparent` 헤더를 전달하고, API의 `traceresponse` 헤더는 스팬 링크로 기록됩니다. 이들은 함께 Claude Code의 클라이언트 측 스팬을 모든 호환 중간 계층을 통해 서버 측 추적에 연결합니다. 아웃바운드 HTTP MCP 요청은 동일한 방식으로 `traceparent`를 전달합니다. 헤더는 타사 제공자에게 전송되지 않습니다.

기본적으로 모델 및 HTTP MCP 요청의 `traceparent` 헤더는 `ANTHROPIC_BASE_URL`이 설정되지 않았거나 Anthropic API를 가리킬 때만 전송됩니다. 일부 프록시는 인식되지 않는 헤더를 거부하기 때문입니다. 하위 프로세스 `TRACEPARENT` 변수는 일관성을 위해 동일한 스위치로 제어됩니다. 사용자 정의 `ANTHROPIC_BASE_URL` 프록시를 통해 Claude Code를 실행하고 추적 컨텍스트를 전파하려면 `CLAUDE_CODE_PROPAGATE_TRACEPARENT=1`을 설정합니다.

Agent SDK 및 `-p`로 시작된 비대화형 세션에서 Claude Code는 각 상호 작용 스팬을 시작할 때 자신의 환경에서 `TRACEPARENT` 및 `TRACESTATE`를 읽습니다. 이를 통해 임베딩 프로세스가 활성 W3C 추적 컨텍스트를 하위 프로세스에 전달할 수 있으므로 Claude Code의 스팬이 호출자의 분산 추적의 자식으로 나타납니다. 대화형 세션은 CI 또는 컨테이너 환경의 주변 값을 실수로 상속하는 것을 피하기 위해 인바운드 `TRACEPARENT`를 무시합니다.

<h4 id="span-hierarchy">
  스팬 계층 구조
</h4>

각 사용자 프롬프트는 `claude_code.interaction` 루트 스팬을 시작합니다. API 호출, 도구 호출 및 훅 실행은 자식으로 기록됩니다. 도구 스팬에는 권한 결정 대기 시간과 실행 자체에 대한 두 개의 자식 스팬이 있습니다. Agent 도구 또는 레거시 Task 도구가 하위 에이전트를 생성하면 하위 에이전트의 API 및 도구 스팬은 부모의 `claude_code.tool` 스팬 아래에 중첩됩니다.

```text theme={null}
claude_code.interaction
├── claude_code.llm_request
├── claude_code.hook                    (상세 베타 추적 필요)
└── claude_code.tool
    ├── claude_code.tool.blocked_on_user
    ├── claude_code.tool.execution
    └── (Agent 도구) 하위 에이전트 claude_code.llm_request / claude_code.tool 스팬
```

Agent SDK 및 `claude -p` 세션에서 `TRACEPARENT`가 환경에 설정되면 `claude_code.interaction` 자체가 호출자의 스팬의 자식이 됩니다.

<h4 id="span-attributes">
  스팬 속성
</h4>

모든 스팬은 [표준 속성](#standard-attributes)과 이름과 일치하는 `span.type` 속성을 전달합니다. 아래 표는 각 스팬에 설정된 추가 속성을 나열합니다. `llm_request`, `tool.execution` 및 `hook` 스팬은 실패를 기록할 때 OpenTelemetry 상태 `ERROR`를 설정합니다. 다른 스팬은 항상 상태 `UNSET`으로 끝납니다.

**`claude_code.interaction`**

| 속성                        | 설명                                         | 게이트 대상                  |
| ------------------------- | ------------------------------------------ | ----------------------- |
| `user_prompt`             | 프롬프트 텍스트. 게이트가 설정되지 않으면 값은 `<REDACTED>`입니다 | `OTEL_LOG_USER_PROMPTS` |
| `user_prompt_length`      | 프롬프트 길이 (문자 단위)                            |                         |
| `interaction.sequence`    | 이 세션의 상호 작용 1 기반 카운터                       |                         |
| `interaction.duration_ms` | 턴의 벽시계 지속 시간                               |                         |

**`claude_code.llm_request`**

| 속성                               | 설명                                                                                                         | 게이트 대상                  |
| -------------------------------- | ---------------------------------------------------------------------------------------------------------- | ----------------------- |
| `model`                          | 모델 식별자                                                                                                     |                         |
| `gen_ai.system`                  | 항상 `anthropic`. OpenTelemetry GenAI 의미론적 규칙                                                                |                         |
| `gen_ai.request.model`           | `model`과 동일한 값. OpenTelemetry GenAI 의미론적 규칙                                                                |                         |
| `query_source`                   | 요청을 발급한 하위 시스템 (예: `repl_main_thread` 또는 하위 에이전트 이름)                                                       |                         |
| `agent_id`                       | 요청을 발급한 하위 에이전트 또는 팀원의 식별자. 주 세션에는 없음                                                                      |                         |
| `parent_agent_id`                | 이 에이전트를 생성한 에이전트의 식별자. 주 세션 및 직접 생성된 에이전트에는 없음                                                             |                         |
| `workflow.run_id`                | 이 에이전트를 생성한 [Workflow](/docs/ko/workflows) 도구 실행의 실행 식별자 (접두사 `wf_`). 워크플로우에 의해 생성되지 않은 에이전트의 경우 없음             |                         |
| `workflow.name`                  | 이 에이전트를 생성한 워크플로우의 이름. 사용자 작성 이름은 게이트가 설정되지 않으면 `custom`으로 대체됩니다                                           | `OTEL_LOG_TOOL_DETAILS` |
| `speed`                          | `fast` 또는 `normal`                                                                                         |                         |
| `llm_request.context`            | 부모 스팬에 따라 `interaction`, `tool` 또는 `standalone`                                                            |                         |
| `duration_ms`                    | 재시도를 포함한 벽시계 지속 시간                                                                                         |                         |
| `ttft_ms`                        | 첫 번째 토큰까지의 시간 (밀리초)                                                                                        |                         |
| `input_tokens`                   | API 사용 블록의 입력 토큰 수                                                                                         |                         |
| `output_tokens`                  | 출력 토큰 수                                                                                                    |                         |
| `cache_read_tokens`              | 프롬프트 캐시에서 읽은 토큰                                                                                            |                         |
| `cache_creation_tokens`          | 프롬프트 캐시에 기록된 토큰                                                                                            |                         |
| `request_id`                     | `request-id` 응답 헤더의 Anthropic API 요청 ID                                                                    |                         |
| `gen_ai.response.id`             | `request_id`와 동일한 값. OpenTelemetry GenAI 의미론적 규칙                                                           |                         |
| `client_request_id`              | 최종 시도의 클라이언트 생성 `x-client-request-id`                                                                      |                         |
| `attempt`                        | 이 요청에 대해 수행된 총 시도                                                                                          |                         |
| `success`                        | `true` 또는 `false`                                                                                          |                         |
| `status_code`                    | 요청이 실패했을 때 HTTP 상태 코드                                                                                      |                         |
| `error`                          | 요청이 실패했을 때 오류 메시지                                                                                          |                         |
| `response.has_tool_call`         | 응답에 도구 사용 블록이 포함되었을 때 `true`                                                                               |                         |
| `stop_reason`                    | API 응답 `stop_reason` (예: `end_turn`, `tool_use`, `max_tokens`, `stop_sequence`, `pause_turn` 또는 `refusal`) |                         |
| `gen_ai.response.finish_reasons` | `stop_reason`과 동일한 값 (문자열 배열로 래핑됨). OpenTelemetry GenAI 의미론적 규칙                                            |                         |

각 재시도 시도는 `attempt` 및 `client_request_id` 속성이 있는 `gen_ai.request.attempt` 스팬 이벤트로도 기록됩니다.

**`claude_code.tool`**

| 속성                    | 설명                                                                                                                                                                                 | 게이트 대상                  |
| --------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `tool_name`           | 도구 이름                                                                                                                                                                              |                         |
| `duration_ms`         | 권한 대기 및 실행을 포함한 벽시계 지속 시간                                                                                                                                                          |                         |
| `result_tokens`       | 도구 결과의 대략적인 토큰 크기                                                                                                                                                                  |                         |
| `agent_id`            | 도구를 실행한 하위 에이전트 또는 팀원의 식별자. 주 세션에는 없음                                                                                                                                              |                         |
| `parent_agent_id`     | 이 에이전트를 생성한 에이전트의 식별자. 주 세션 및 직접 생성된 에이전트에는 없음                                                                                                                                     |                         |
| `workflow.run_id`     | 이 에이전트를 생성한 Workflow 도구 실행의 실행 식별자 (접두사 `wf_`). 워크플로우에 의해 생성되지 않은 에이전트의 경우 없음                                                                                                      |                         |
| `workflow.name`       | 이 에이전트를 생성한 워크플로우의 이름. 사용자 작성 이름은 게이트가 설정되지 않으면 `custom`으로 대체됩니다                                                                                                                   | `OTEL_LOG_TOOL_DETAILS` |
| `tool_use_id`         | 이 호출에 대한 모델의 `tool_use` 블록 ID. [tool\_result](#tool-result-event) 및 [tool\_decision](#tool-decision-event) 이벤트의 `tool_use_id`와 훅 페이로드의 `tool_use_id`와 일치하므로 스팬을 해당 레코드에 조인할 수 있습니다 |                         |
| `gen_ai.tool.call.id` | `tool_use_id`와 동일한 값. OpenTelemetry GenAI 의미론적 규칙                                                                                                                                  |                         |
| `file_path`           | Read, Edit 및 Write 도구의 대상 파일 경로                                                                                                                                                    | `OTEL_LOG_TOOL_DETAILS` |
| `full_command`        | Bash 도구의 명령 문자열                                                                                                                                                                    | `OTEL_LOG_TOOL_DETAILS` |
| `skill_name`          | Skill 도구의 스킬 이름                                                                                                                                                                    | `OTEL_LOG_TOOL_DETAILS` |
| `subagent_type`       | Agent 도구 또는 레거시 Task 도구의 하위 에이전트 유형                                                                                                                                                | `OTEL_LOG_TOOL_DETAILS` |

`OTEL_LOG_TOOL_CONTENT=1`일 때 이 스팬은 속성에 도구의 입력 및 출력 본문을 포함하는 `tool.output` 스팬 이벤트도 기록합니다 (속성당 60KB에서 잘림).

**`claude_code.tool.blocked_on_user`**

| 속성            | 설명                                                      | 게이트 대상 |
| ------------- | ------------------------------------------------------- | ------ |
| `duration_ms` | 권한 결정 대기 시간                                             |        |
| `decision`    | `accept` 또는 `reject`                                    |        |
| `source`      | 결정 출처 ([Tool decision event](#tool-decision-event)와 일치) |        |

**`claude_code.tool.execution`**

| 속성                    | 설명                                                                                   | 게이트 대상                  |
| --------------------- | ------------------------------------------------------------------------------------ | ----------------------- |
| `duration_ms`         | 도구 본문 실행 시간                                                                          |                         |
| `tool_use_id`         | 부모 `claude_code.tool` 스팬과 동일한 값                                                      |                         |
| `gen_ai.tool.call.id` | `tool_use_id`와 동일한 값. OpenTelemetry GenAI 의미론적 규칙                                    |                         |
| `success`             | `true` 또는 `false`                                                                    |                         |
| `error`               | 실행이 실패했을 때 오류 범주 문자열 (예: `Error:ENOENT` 또는 `ShellError`). 게이트가 설정되면 전체 오류 메시지를 포함합니다 | `OTEL_LOG_TOOL_DETAILS` |

**`claude_code.hook`**

이 스팬은 상세 베타 추적이 활성화되어 있을 때만 내보내지며, 이는 위의 추적 내보내기 구성 외에 `ENABLE_BETA_TRACING_DETAILED=1` 및 `BETA_TRACING_ENDPOINT`가 필요합니다. 대화형 CLI 세션에서는 조직이 이 기능에 대해 허용 목록에 있어야 합니다. Agent SDK 및 비대화형 `-p` 세션은 게이트되지 않습니다. `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA`만 설정되어 있을 때는 내보내지지 않습니다.

| 속성                       | 설명                              | 게이트 대상                  |
| ------------------------ | ------------------------------- | ----------------------- |
| `hook_event`             | 훅 이벤트 유형 (예: `PreToolUse`)      |                         |
| `hook_name`              | 전체 훅 이름 (예: `PreToolUse:Write`) |                         |
| `num_hooks`              | 실행된 일치하는 훅 명령 수                 |                         |
| `hook_definitions`       | JSON 직렬화된 훅 구성                  | `OTEL_LOG_TOOL_DETAILS` |
| `duration_ms`            | 모든 일치하는 훅의 벽시계 지속 시간            |                         |
| `num_success`            | 성공적으로 완료된 훅 수                   |                         |
| `num_blocking`           | 차단 결정을 반환한 훅 수                  |                         |
| `num_non_blocking_error` | 차단 없이 실패한 훅 수                   |                         |
| `num_cancelled`          | 완료 전에 취소된 훅 수                   |                         |

<Note>
  `new_context`, `system_prompt_preview`, `user_system_prompt`, `tool_input` 및 `response.model_output`과 같은 추가 콘텐츠 포함 속성은 상세 베타 추적이 활성화되어 있을 때만 내보내집니다. 이들은 안정적인 스팬 스키마의 일부가 아닙니다. `user_system_prompt`는 추가로 `OTEL_LOG_USER_PROMPTS=1`이 필요합니다. 이는 `systemPrompt` SDK 옵션 또는 `--system-prompt` 및 `--append-system-prompt` 플래그를 통해 제공하는 시스템 프롬프트 텍스트만 포함하며 60KB에서 잘리고 요청당이 아닌 세션당 한 번 내보내집니다.
</Note>

<h3 id="dynamic-headers">
  동적 헤더
</h3>

동적 인증이 필요한 엔터프라이즈 환경의 경우 스크립트를 구성하여 헤더를 동적으로 생성할 수 있습니다. 동적 헤더는 `http/protobuf` 및 `http/json` 프로토콜에만 적용됩니다. `grpc` 내보내기는 정적 `OTEL_EXPORTER_OTLP_HEADERS` 값만 사용합니다.

<h4 id="settings-configuration">
  설정 구성
</h4>

`.claude/settings.json`에 추가합니다:

```json theme={null}
{
  "otelHeadersHelper": "/bin/generate_opentelemetry_headers.sh"
}
```

값은 공백을 포함한 경로를 포함하는 실행 파일의 경로이거나 인수가 있는 셸 명령줄일 수 있습니다. Windows에서 값은 항상 셸을 통해 실행되므로 공백을 포함하는 경로를 JSON 값 내에 따옴표로 묶습니다.

<h4 id="script-requirements">
  스크립트 요구 사항
</h4>

스크립트는 HTTP 헤더를 나타내는 문자열 키-값 쌍이 있는 유효한 JSON을 출력해야 합니다:

```bash theme={null}
#!/bin/bash
# 예: 여러 헤더
echo "{\"Authorization\": \"Bearer $(get-token.sh)\", \"X-API-Key\": \"$(get-api-key.sh)\"}"
```

도우미가 실패하거나 이러한 요구 사항을 충족하지 않는 출력을 인쇄하면 Claude Code는 다음에서 오류를 보고합니다:

* `/status` 출력
* [`--debug`](/docs/ko/cli-reference#cli-flags)로 실행하거나 세션에서 `/debug`를 실행한 후의 디버그 로그
* `-p`로 시작된 비대화형 세션의 stderr

<h4 id="refresh-behavior">
  새로 고침 동작
</h4>

헤더 도우미 스크립트는 시작 시 그리고 그 이후 주기적으로 실행되어 토큰 새로 고침을 지원합니다. 기본적으로 스크립트는 29분마다 실행됩니다. `CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS` 환경 변수로 간격을 사용자 정의합니다.

<h3 id="multi-team-organization-support">
  다중 팀 조직 지원
</h3>

여러 팀 또는 부서가 있는 조직은 `OTEL_RESOURCE_ATTRIBUTES` 환경 변수를 사용하여 다양한 그룹을 구분하기 위한 사용자 정의 속성을 추가할 수 있습니다:

```bash theme={null}
# 팀 식별을 위한 사용자 정의 속성 추가
export OTEL_RESOURCE_ATTRIBUTES="department=engineering,team.id=platform,cost_center=eng-123"
```

이러한 사용자 정의 속성은 모든 메트릭 및 이벤트에 포함되어 다음을 수행할 수 있습니다:

* 팀 또는 부서별로 메트릭 필터링
* 비용 센터별 비용 추적
* 팀별 대시보드 생성
* 특정 팀에 대한 경고 설정

Claude Code는 이러한 값을 모든 메트릭 데이터포인트 및 이벤트 레코드의 속성으로 첨부하고, OTLP 리소스 블록에서도 전송합니다. 대부분의 메트릭 백엔드는 데이터포인트 속성을 쿼리 가능한 레이블로 노출하므로 사용자 정의 키로 직접 메트릭을 그룹화하고 필터링할 수 있습니다. 사용자 정의 키는 `user.id` 또는 `session.id`와 같은 [표준 속성](#standard-attributes)을 재정의하지 않습니다. 키가 충돌하면 Claude Code는 기본 제공 값을 유지합니다.

각 사용자 정의 키는 모든 메트릭 시리즈의 레이블이 되므로 높은 카디널리티 값은 메트릭 백엔드의 저장소 비용을 증가시킵니다. 사용자 정의 속성을 리소스 블록에만 보내고 데이터포인트 레이블에서 생략하려면 `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES=false`를 설정합니다. [메트릭 카디널리티 제어](#metrics-cardinality-control)를 참조합니다.

<Warning>
  `OTEL_RESOURCE_ATTRIBUTES` 환경 변수는 쉼표로 구분된 key=value 쌍을 사용하며 엄격한 형식 요구 사항이 있습니다:

  * **공백 허용 안 함**: 값에 공백이 포함될 수 없습니다. 예를 들어 `user.organizationName=My Company`는 유효하지 않습니다
  * **형식**: 쉼표로 구분된 키=값 쌍이어야 합니다: `key1=value1,key2=value2`
  * **허용된 문자**: 제어 문자, 공백, 큰따옴표, 쉼표, 세미콜론 및 백슬래시를 제외한 US-ASCII 문자만 허용됩니다
  * **특수 문자**: 허용된 범위 외의 문자는 퍼센트 인코딩되어야 합니다

  공백이 필요한 값의 경우 언더스코어 또는 camelCase를 대신 사용합니다. 다음 예제는 각 형식으로 `org.name`을 설정합니다:

  ```bash theme={null}
  export OTEL_RESOURCE_ATTRIBUTES="org.name=Johns_Organization"
  export OTEL_RESOURCE_ATTRIBUTES="org.name=JohnsOrganization"
  ```

  제외된 문자뿐만 아니라 모든 문자를 퍼센트 인코딩할 수 있습니다. 이 예제는 공백과 아포스트로피를 모두 인코딩합니다:

  ```bash theme={null}
  export OTEL_RESOURCE_ATTRIBUTES="org.name=John%27s%20Organization"
  ```

  값을 따옴표로 감싸도 공백이 이스케이프되지 않습니다. 예를 들어 `org.name="My Company"`는 `My Company`가 아닌 리터럴 값 `"My Company"` (따옴표 포함)를 생성합니다.
</Warning>

<h3 id="example-configurations">
  예제 구성
</h3>

`claude`를 실행하기 전에 이러한 환경 변수를 설정합니다. 각 블록은 다양한 내보내기 또는 배포 시나리오에 대한 완전한 구성을 보여줍니다:

```bash theme={null}
# 콘솔 디버깅 (1초 간격)
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console
export OTEL_METRIC_EXPORT_INTERVAL=1000

# OTLP/gRPC
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# Prometheus
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=prometheus

# 여러 내보내기
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console,otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=http/json

# 메트릭 및 로그에 대한 다양한 엔드포인트/백엔드
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_METRICS_PROTOCOL=http/protobuf
export OTEL_EXPORTER_OTLP_METRICS_ENDPOINT=http://metrics.example.com:4318
export OTEL_EXPORTER_OTLP_LOGS_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_LOGS_ENDPOINT=http://logs.example.com:4317

# 메트릭만 (이벤트/로그 없음)
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317

# 이벤트/로그만 (메트릭 없음)
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

<h2 id="available-metrics-and-events">
  사용 가능한 메트릭 및 이벤트
</h2>

<h3 id="standard-attributes">
  표준 속성
</h3>

모든 메트릭 및 이벤트는 다음 표준 속성을 공유합니다:

| 속성                            | 설명                                                                                                                            | 제어 대상                                                  |
| ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------ |
| `session.id`                  | 고유 세션 식별자                                                                                                                     | `OTEL_METRICS_INCLUDE_SESSION_ID` (기본값: true)          |
| `app.version`                 | 현재 Claude Code 버전                                                                                                             | `OTEL_METRICS_INCLUDE_VERSION` (기본값: false)            |
| `app.entrypoint`              | 세션이 시작된 방식, 예: `cli`, `sdk-cli`, `sdk-ts`, `sdk-py` 또는 `claude-vscode`                                                        | `OTEL_METRICS_INCLUDE_ENTRYPOINT` (기본값: false)         |
| `organization.id`             | 조직 UUID (인증된 경우)                                                                                                              | 사용 가능할 때 항상 포함됨                                        |
| `user.account_uuid`           | 계정 UUID (인증된 경우)                                                                                                              | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (기본값: true)        |
| `user.account_id`             | Anthropic 관리 API와 일치하는 태그 형식의 계정 ID (인증된 경우), 예: `user_01BWBeN28...`                                                          | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (기본값: true)        |
| `user.id`                     | 첫 실행 시 생성되고 `~/.claude.json`에 유지되는 무작위 익명 식별자입니다. 개인 정보를 포함하지 않으며 Claude 계정에서 파생되지 않습니다. 파일을 삭제하면 다음 실행 시 새로운 관련 없는 값이 생성됩니다. | 항상 포함됨                                                 |
| `user.email`                  | 사용자 이메일 주소 (OAuth를 통해 인증된 경우)                                                                                                 | 사용 가능할 때 항상 포함됨                                        |
| `terminal.type`               | 터미널 유형, 예: `iTerm.app`, `vscode`, `cursor` 또는 `tmux`                                                                          | 감지될 때 항상 포함됨                                           |
| `OTEL_RESOURCE_ATTRIBUTES`의 키 | 설정한 사용자 정의 속성, 예: `department` 또는 `team.id`. [다중 팀 조직 지원](#multi-team-organization-support)을 참조하세요                            | `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` (기본값: true) |

Claude Code가 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway)에 로그인되어 있으면 CLI는 게이트웨이 세션의 인증된 ID로 내보내기를 스탬프합니다: `user.id`는 익명 설치 식별자가 아닌 IdP 주체이고, `user.email`은 로그인한 이메일이며, `user.groups`는 쉼표로 구분된 문자열로 IdP 그룹 멤버십을 전달합니다. 각 내보내기는 또한 `identity.source: gateway-oidc`를 전달합니다. 게이트웨이 ID는 마지막에 적용되므로 `OTEL_RESOURCE_ATTRIBUTES`를 통해 설정된 `user.*` 및 `identity.*` 키는 게이트웨이 세션에서 무시됩니다.

이벤트는 추가로 다음 속성을 포함합니다. 이들은 무한 카디널리티를 야기할 수 있으므로 메트릭에 절대 첨부되지 않습니다:

* `prompt.id`: 사용자 프롬프트를 다음 프롬프트까지의 모든 후속 이벤트와 상관시키는 UUID입니다. [이벤트 상관 속성](#event-correlation-attributes)을 참조하세요.
* `workspace.host_paths`: 데스크톱 앱에서 선택한 호스트 작업 공간 디렉토리 (문자열 배열)
* `workflow.run_id`: API 및 [Workflow](/docs/ko/workflows) 도구 실행에 속하는 에이전트가 내보낸 도구 이벤트의 실행 식별자 (접두사 `wf_`). 하나의 `workflow.run_id`로 이벤트를 필터링하면 해당 실행의 API 요청 및 도구 결과를 재구성합니다. 식별자는 워크플로우 스크립트가 생성하는 에이전트 및 이들이 차례로 생성하는 모든 에이전트 (예: 스킬 호출)를 포함합니다. Workflow 도구 결과에서 보고된 실행 식별자와 일치합니다. 다른 모든 이벤트에는 없습니다. Claude Code v2.1.202 이상 필요
* `workflow.name`: 워크플로우의 이름 (스크립트의 `meta.name`), `workflow.run_id`와 함께 내보내집니다. 기본 제공 워크플로우 이름은 수정되지 않은 기본 제공 스크립트를 실행할 때 그대로 나타납니다. 기본 제공 스크립트의 편집된 복사본을 포함한 사용자 작성 이름은 `OTEL_LOG_TOOL_DETAILS=1`이 설정되지 않으면 `custom`으로 대체됩니다. Claude Code v2.1.202 이상 필요

<h3 id="metrics">
  메트릭
</h3>

Claude Code는 다음 메트릭을 내보냅니다:

| 메트릭 이름                                | 설명                 | 단위     |
| ------------------------------------- | ------------------ | ------ |
| `claude_code.session.count`           | 시작된 CLI 세션 수       | count  |
| `claude_code.lines_of_code.count`     | 수정된 코드 라인 수        | count  |
| `claude_code.pull_request.count`      | 생성된 풀 요청 수         | count  |
| `claude_code.commit.count`            | 생성된 git 커밋 수       | count  |
| `claude_code.cost.usage`              | Claude Code 세션의 비용 | USD    |
| `claude_code.token.usage`             | 사용된 토큰 수           | tokens |
| `claude_code.code_edit_tool.decision` | 코드 편집 도구 권한 결정 수   | count  |
| `claude_code.active_time.total`       | 총 활성 시간 (초)        | s      |

<h3 id="metric-details">
  메트릭 세부 정보
</h3>

각 메트릭은 위에 나열된 표준 속성을 포함합니다. 추가 컨텍스트별 속성이 있는 메트릭은 아래에 표시됩니다.

<h4 id="session-counter">
  세션 카운터
</h4>

각 세션 시작 시 증가합니다.

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `start_type`: 세션이 시작된 방식. `"fresh"`, `"resume"`, `"continue"` 또는 `"agents_view"` 중 하나입니다. `"agents_view"` 값은 `claude agents` 대시보드 프로세스 (대화형 세션이 아닌 사용자가 시작한 로컬 UI)를 식별합니다. 대시보드에서 UI 프로세스 시작을 대화형 세션과 분리하려면 이 값으로 필터링합니다.

<h4 id="lines-of-code-counter">
  코드 라인 카운터
</h4>

코드가 추가되거나 제거될 때 증가합니다.

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `type`: (`"added"`, `"removed"`)
* `model`: 변경을 수행한 모델의 모델 식별자 (예: "claude-sonnet-5")

<h4 id="pull-request-counter">
  풀 요청 카운터
</h4>

Claude Code를 통해 셸 명령 또는 MCP 도구를 통해 풀 요청 또는 병합 요청을 생성할 때 증가합니다.

**속성**:

* 모든 [표준 속성](#standard-attributes)

<h4 id="commit-counter">
  커밋 카운터
</h4>

Claude Code를 통해 git 커밋을 생성할 때 증가합니다.

**속성**:

* 모든 [표준 속성](#standard-attributes)

<h4 id="cost-counter">
  비용 카운터
</h4>

각 API 요청 후 증가합니다.

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `model`: 모델 식별자 (예: "claude-sonnet-5")
* `query_source`: 요청을 발급한 하위 시스템의 범주. `"main"`, `"subagent"` 또는 `"auxiliary"` 중 하나
* `speed`: 요청이 빠른 모드를 사용했을 때 `"fast"`. 그 외에는 없음
* `effort`: 요청에 적용된 [노력 수준](/docs/ko/model-config#adjust-effort-level): `"low"`, `"medium"`, `"high"`, `"xhigh"` 또는 `"max"`. 모델이 노력을 지원하지 않을 때는 없음
* `agent.name`: 요청을 발급한 하위 에이전트 유형. 기본 제공 에이전트 이름 및 공식 마켓플레이스 플러그인의 에이전트는 그대로 나타납니다. 다른 사용자 정의 에이전트 이름은 `"custom"`으로 대체됩니다. 요청이 명명된 하위 에이전트 유형에서 발급되지 않았을 때는 없음
* `skill.name`: 요청에 대해 활성화된 스킬 (Skill 도구, `/` 명령으로 설정되거나 생성된 하위 에이전트에 의해 상속됨). 기본 제공, 번들, 사용자 정의 및 공식 마켓플레이스 플러그인 스킬 이름은 그대로 나타납니다. 타사 플러그인 스킬 이름은 `"third-party"`로 대체됩니다. 활성 스킬이 없을 때는 없음
* `plugin.name`: 활성 스킬 또는 하위 에이전트가 플러그인에서 제공될 때 소유 플러그인. 공식 마켓플레이스 플러그인 이름은 그대로 나타납니다. 타사 플러그인 이름은 `"third-party"`로 대체됩니다. 스킬 및 하위 에이전트 모두 소유 플러그인이 없을 때는 없음
* `marketplace.name`: 소유 플러그인이 설치된 마켓플레이스. 공식 마켓플레이스 플러그인에만 내보내집니다. 그 외에는 없음
* `mcp_server.name`: 이 요청을 생성한 턴에서 도구가 실행된 MCP 서버. 기본 제공, claude.ai 프록시 및 공식 레지스트리 서버 이름은 그대로 나타납니다. 사용자 구성 서버 이름은 `"custom"`으로 대체됩니다. MCP 도구가 실행되지 않았을 때는 없음
* `mcp_tool.name`: 이 요청을 생성한 턴에서 실행된 MCP 도구 (`mcp_server.name`과 동일한 수정 처리). MCP 도구가 실행되지 않았을 때는 없음

<h4 id="token-counter">
  토큰 카운터
</h4>

각 API 요청 후 증가합니다.

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `type`: (`"input"`, `"output"`, `"cacheRead"`, `"cacheCreation"`)
* `model`: 모델 식별자 (예: "claude-sonnet-5")
* `query_source`: 요청을 발급한 하위 시스템의 범주. `"main"`, `"subagent"` 또는 `"auxiliary"` 중 하나
* `speed`: 요청이 빠른 모드를 사용했을 때 `"fast"`. 그 외에는 없음
* `effort`: 요청에 적용된 [노력 수준](/docs/ko/model-config#adjust-effort-level). [비용 카운터](#cost-counter)의 세부 정보를 참조하세요.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: 요청에 대한 스킬, 플러그인, 에이전트 및 MCP 속성. [비용 카운터](#cost-counter)의 정의 및 수정 동작을 참조하세요.

<h4 id="code-edit-tool-decision-counter">
  코드 편집 도구 결정 카운터
</h4>

사용자가 Edit, Write 또는 NotebookEdit 도구 사용을 수락하거나 거부할 때 증가합니다.

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `tool_name`: 도구 이름 (`"Edit"`, `"Write"`, `"NotebookEdit"`)
* `decision`: 사용자 결정 (`"accept"`, `"reject"`)
* `source`: 결정 출처. `"config"`, `"hook"`, `"user_permanent"`, `"user_temporary"`, `"user_abort"` 또는 `"user_reject"` 중 하나. [도구 결정 이벤트](#tool-decision-event)를 참조하여 각 값의 의미를 확인하세요.
* `language`: 편집된 파일의 프로그래밍 언어, 예: `"TypeScript"`, `"Python"`, `"JavaScript"` 또는 `"Markdown"`. 인식되지 않는 파일 확장자의 경우 `"unknown"`을 반환합니다.

<h4 id="active-time-counter">
  활성 시간 카운터
</h4>

Claude Code를 적극적으로 사용하는 실제 시간을 추적합니다 (유휴 시간 제외). 이 메트릭은 사용자 상호 작용 (입력, 응답 읽기) 중 및 CLI 처리 (도구 실행, AI 응답 생성) 중에 증가합니다.

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `type`: 키보드 상호 작용의 경우 `"user"`, 도구 실행 및 AI 응답의 경우 `"cli"`

<h3 id="events">
  이벤트
</h3>

Claude Code는 OpenTelemetry 로그/이벤트를 통해 다음 이벤트를 내보냅니다 (`OTEL_LOGS_EXPORTER`가 구성된 경우):

<h4 id="event-correlation-attributes">
  이벤트 상관 속성
</h4>

사용자가 프롬프트를 제출하면 Claude Code는 여러 API 호출을 수행하고 여러 도구를 실행할 수 있습니다. `prompt.id` 속성을 사용하면 이러한 모든 이벤트를 해당 이벤트를 트리거한 단일 프롬프트에 연결할 수 있습니다.

| 속성          | 설명                                             |
| ----------- | ---------------------------------------------- |
| `prompt.id` | 단일 사용자 프롬프트 처리 중에 생성된 모든 이벤트를 연결하는 UUID v4 식별자 |

단일 프롬프트로 트리거된 모든 활동을 추적하려면 특정 `prompt.id` 값으로 이벤트를 필터링합니다. 이는 user\_prompt 이벤트, 모든 api\_request 이벤트 및 해당 프롬프트 처리 중에 발생한 모든 tool\_result 이벤트를 반환합니다.

<Note>
  `prompt.id`는 각 프롬프트가 고유 ID를 생성하여 계속 증가하는 시계열 수를 만들기 때문에 의도적으로 메트릭에서 제외됩니다. 이벤트 수준 분석 및 감사 추적에만 사용합니다.
</Note>

<h4 id="user-prompt-event">
  사용자 프롬프트 이벤트
</h4>

사용자가 프롬프트를 제출할 때 기록됩니다.

**이벤트 이름**: `claude_code.user_prompt`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"user_prompt"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 세션 내 이벤트 순서 지정을 위한 단조 증가 카운터
* `prompt_length`: 프롬프트의 길이
* `prompt`: 프롬프트 콘텐츠. 기본적으로 수정됨. `OTEL_LOG_USER_PROMPTS=1`로 설정하여 포함
* `command_name`: 프롬프트가 명령을 호출할 때 명령 이름. `compact` 또는 `debug`와 같은 기본 제공 및 번들 명령 이름은 그대로 내보내집니다. `reset`과 같은 별칭은 정규 이름이 아닌 입력한 대로 내보냅니다. 사용자 정의, 플러그인 및 MCP 명령 이름은 `OTEL_LOG_TOOL_DETAILS=1`이 설정되지 않으면 `custom` 또는 `mcp`로 축소됩니다
* `command_source`: 명령이 있을 때 명령의 출처: `builtin`, `custom` 또는 `mcp`. 플러그인 제공 명령은 `custom`으로 보고합니다

<h4 id="assistant-response-event">
  어시스턴트 응답 이벤트
</h4>

각 API 요청이 모델의 텍스트 콘텐츠를 반환한 후 기록됩니다. 응답의 텍스트 블록만 포함됩니다. 사고 블록 및 도구 사용 블록은 제외됩니다. Claude Code v2.1.193 이상 필요.

**이벤트 이름**: `claude_code.assistant_response`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"assistant_response"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 세션 내 이벤트 순서 지정을 위한 단조 증가 카운터
* `response_length`: 응답 텍스트의 길이 (문자)
* `response`: 응답 텍스트 (60KB에서 잘림). 기본적으로 `<REDACTED>`로 수정됨. `OTEL_LOG_ASSISTANT_RESPONSES=1`로 설정하여 포함. `OTEL_LOG_ASSISTANT_RESPONSES`가 설정되지 않으면 `OTEL_LOG_USER_PROMPTS`가 대신 제어하므로 프롬프트 로깅이 켜져 있는 동안 응답을 수정된 상태로 유지하려면 `OTEL_LOG_ASSISTANT_RESPONSES=0`으로 설정합니다
* `model`: 모델 식별자 (예: "claude-sonnet-5")
* `request_id`: 응답의 `request-id` 헤더의 Anthropic API 요청 ID. API가 반환할 때만 표시됩니다
* `query_source`: 요청을 발급한 하위 시스템, 예: `"repl_main_thread"`, `"compact"` 또는 하위 에이전트 이름

<h4 id="tool-result-event">
  도구 결과 이벤트
</h4>

도구가 실행을 완료할 때 기록됩니다. 도구 호출이 거부된 경우 내보내지지 않습니다. [도구 결정 이벤트](#tool-decision-event)를 참조하세요.

**이벤트 이름**: `claude_code.tool_result`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"tool_result"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 세션 내 이벤트 순서 지정을 위한 단조 증가 카운터
* `tool_name`: 도구의 이름
* `tool_use_id`: 이 도구 호출의 고유 식별자. 훅에 전달된 `tool_use_id`와 일치하여 OTel 이벤트와 훅 캡처 데이터 간의 상관관계를 허용합니다.
* `success`: `"true"` 또는 `"false"`
* `duration_ms`: 실행 시간 (밀리초)
* `error_type`: 도구가 실패했을 때 오류 범주 문자열, 예: `"Error:ENOENT"` 또는 `"ShellError"`
* `error` (`OTEL_LOG_TOOL_DETAILS=1`일 때): 도구가 실패했을 때 전체 오류 메시지
* `decision_type`: 항상 `"accept"`입니다. 이 이벤트는 도구가 실행된 후에만 내보내집니다 (거부된 호출은 도구 결과를 생성하지 않음)
* `decision_source`: 권한 결정이 나온 위치. `"config"`, `"hook"`, `"user_permanent"` 또는 `"user_temporary"` 중 하나. [도구 결정 이벤트](#tool-decision-event)를 참조하여 각 값의 의미를 확인하세요. 거부 전용 출처인 `"user_abort"` 및 `"user_reject"`는 이 이벤트에 나타나지 않습니다.
* `tool_input_size_bytes`: JSON 직렬화된 도구 입력의 크기 (바이트)
* `tool_result_size_bytes`: 도구 결과의 크기 (바이트)
* `mcp_server_scope`: MCP 서버 범위 식별자 (MCP 도구의 경우)
* `tool_parameters` (`OTEL_LOG_TOOL_DETAILS=1`일 때): 도구별 매개변수를 포함하는 JSON 문자열:
  * Bash 도구의 경우: `bash_command`, `full_command`, `timeout`, `description`, `dangerouslyDisableSandbox` 및 `git_commit_id` (git commit 명령이 성공할 때 커밋 SHA) 포함
  * WorkspaceBash 도구의 경우: `bash_command`, `full_command`, `timeout` 포함
  * MCP 도구의 경우: `mcp_server_name`, `mcp_tool_name` 포함
  * Skill 도구의 경우: `skill_name` 포함
  * Agent 도구 또는 레거시 Task 도구의 경우: `subagent_type` 포함
* `tool_input` (`OTEL_LOG_TOOL_DETAILS=1`일 때): JSON 직렬화된 도구 인수입니다. 512자를 초과하는 개별 값은 잘리고, 전체 페이로드는 약 4K 문자로 제한됩니다. MCP 도구를 포함한 모든 도구에 적용됩니다.

<h4 id="api-request-event">
  API 요청 이벤트
</h4>

Claude에 대한 각 API 요청에 대해 기록됩니다.

**이벤트 이름**: `claude_code.api_request`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"api_request"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 세션 내 이벤트 순서 지정을 위한 단조 증가 카운터
* `model`: 사용된 모델 (예: "claude-sonnet-5")
* `cost_usd`: USD 단위의 예상 비용
* `duration_ms`: 요청 지속 시간 (밀리초)
* `input_tokens`: 입력 토큰 수
* `output_tokens`: 출력 토큰 수
* `cache_read_tokens`: 캐시에서 읽은 토큰 수
* `cache_creation_tokens`: 캐시 생성에 사용된 토큰 수
* `request_id`: 응답의 `request-id` 헤더의 Anthropic API 요청 ID, 예: `"req_011..."`. API가 반환할 때만 표시됩니다.
* `speed`: 빠른 모드가 활성화되었는지 여부를 나타내는 `"fast"` 또는 `"normal"`
* `query_source`: 요청을 발급한 하위 시스템, 예: `"repl_main_thread"`, `"compact"` 또는 하위 에이전트 이름
* `effort`: 요청에 적용된 [노력 수준](/docs/ko/model-config#adjust-effort-level): `"low"`, `"medium"`, `"high"`, `"xhigh"` 또는 `"max"`. 모델이 노력을 지원하지 않을 때는 없음
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: 요청에 대한 스킬, 플러그인, 에이전트 및 MCP 속성. [비용 카운터](#cost-counter)의 정의 및 수정 동작을 참조하세요.

<h4 id="api-error-event">
  API 오류 이벤트
</h4>

Claude에 대한 API 요청이 실패할 때 기록됩니다.

**이벤트 이름**: `claude_code.api_error`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"api_error"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 세션 내 이벤트 순서 지정을 위한 단조 증가 카운터
* `model`: 사용된 모델 (예: "claude-sonnet-5")
* `error`: 오류 메시지
* `status_code`: HTTP 상태 코드 (숫자). HTTP가 아닌 오류 (예: 연결 실패)의 경우 없음
* `duration_ms`: 요청 지속 시간 (밀리초)
* `attempt`: 초기 요청을 포함한 총 시도 횟수 (`1`은 재시도가 발생하지 않았음을 의미)
* `request_id`: 응답의 `request-id` 헤더의 Anthropic API 요청 ID, 예: `"req_011..."`. API가 반환할 때만 표시됩니다.
* `speed`: 빠른 모드가 활성화되었는지 여부를 나타내는 `"fast"` 또는 `"normal"`
* `query_source`: 요청을 발급한 하위 시스템, 예: `"repl_main_thread"`, `"compact"` 또는 하위 에이전트 이름
* `effort`: 요청에 적용된 [노력 수준](/docs/ko/model-config#adjust-effort-level). 모델이 노력을 지원하지 않을 때는 없음
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: 요청에 대한 스킬, 플러그인, 에이전트 및 MCP 속성. [비용 카운터](#cost-counter)의 정의 및 수정 동작을 참조하세요.

<h4 id="api-refusal-event">
  API 거부 이벤트
</h4>

API 요청이 `stop_reason: "refusal"`을 반환할 때 기록됩니다. 거부는 HTTP 오류가 아닌 성공적인 응답 스트림에 도착하므로 `api_error` 이벤트는 이에 대해 발생하지 않습니다. 이 이벤트를 사용하면 거부 빈도를 추적하고 거부를 `api_request` 및 `api_error`와 동일한 속성으로 그룹화할 수 있습니다.

**이벤트 이름**: `claude_code.api_refusal`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"api_refusal"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 세션 내 이벤트 순서 지정을 위한 단조 증가 카운터
* `model`: 요청의 모델 식별자
* `request_id`: 응답의 `request-id` 헤더의 Anthropic API 요청 ID, 예: `"req_011..."`. API가 반환할 때만 표시됩니다.
* `query_source`: 요청을 발급한 하위 시스템, 예: `"repl_main_thread"`, `"compact"` 또는 하위 에이전트 이름. [`api_request`](#api-request-event)의 정의를 참조하세요.
* `speed`: [빠른 모드](/docs/ko/fast-mode)가 활성화되었을 때 `"fast"`, 또는 `"normal"`
* `attempt`: 재시도 시도 번호. 첫 번째 시도는 `1`입니다.
* `effort`: 요청에 적용된 [노력 수준](/docs/ko/model-config#adjust-effort-level). 모델이 노력을 지원하지 않을 때는 없음
* `server_fallback_hop`: API의 서버 측 모델 폴백이 이미 이 거부를 다른 모델에서 재시도했으므로 사용자가 이 특정 거부를 보지 못했을 때 `true`. 요청이 거부로 끝났을 때 `false`. 단일 턴은 나중에 `false` 최종 이벤트가 있는 `true` 홉 이벤트를 모두 내보낼 수 있습니다.
* `has_category`: API 응답이 `"cyber"`, `"bio"`, `"frontier_llm"` 또는 `"reasoning_extraction"`의 `stop_details.category`를 전달했을 때 `true`. 응답이 카테고리를 전달하지 않았거나 해당 집합 외부의 값을 전달했을 때 `false`. `server_fallback_hop`이 `true`일 때는 없음 (홉 블록은 `stop_details`를 전달하지 않음).
* `has_explanation`: API 응답이 `stop_details.explanation`을 전달했을 때 `true`, 그 외에는 `false`. `server_fallback_hop`이 `true`일 때는 없음.
* `category`: API 응답의 `stop_details.category` 값. `"cyber"`, `"bio"`, `"frontier_llm"` 또는 `"reasoning_extraction"` 중 하나. `OTEL_LOG_TOOL_DETAILS=1`이 설정되어 있고 `has_category`가 `true`일 때만 표시됩니다.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: 요청에 대한 스킬, 플러그인, 에이전트 및 MCP 속성. [비용 카운터](#cost-counter)의 정의 및 수정 동작을 참조하세요.

<h4 id="api-request-body-event">
  API 요청 본문 이벤트
</h4>

`OTEL_LOG_RAW_API_BODIES`가 설정되어 있을 때 각 API 요청 시도에 대해 기록됩니다. 조정된 매개변수를 사용한 재시도는 각각 자신의 이벤트를 생성하므로 시도당 하나의 이벤트가 내보내집니다.

**이벤트 이름**: `claude_code.api_request_body`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"api_request_body"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 세션 내 이벤트 순서 지정을 위한 단조 증가 카운터
* `body`: JSON 직렬화된 Messages API 요청 매개변수 (시스템 프롬프트, 메시지, 도구 등) (60KB에서 잘림). 이전 어시스턴트 턴의 확장 사고 콘텐츠는 수정됩니다. 인라인 모드 (`OTEL_LOG_RAW_API_BODIES=1`)에서만 내보내집니다.
* `body_ref`: 잘리지 않은 본문을 포함하는 `<dir>/<uuid>.request.json` 파일의 절대 경로. 파일 모드 (`OTEL_LOG_RAW_API_BODIES=file:<dir>`)에서만 내보내집니다.
* `body_length`: 잘리지 않은 본문 길이. `OTEL_LOG_RAW_API_BODIES=file:<dir>`일 때 UTF-8 바이트, `=1`일 때 UTF-16 코드 단위
* `body_truncated`: 인라인 잘림이 발생했을 때 `"true"`. 파일 모드 및 잘림이 발생하지 않았을 때는 없음
* `model`: 요청 매개변수의 모델 식별자
* `query_source`: 요청을 발급한 하위 시스템 (예: `"compact"`)

<h4 id="api-response-body-event">
  API 응답 본문 이벤트
</h4>

`OTEL_LOG_RAW_API_BODIES`가 설정되어 있을 때 각 성공적인 API 응답에 대해 기록됩니다.

**이벤트 이름**: `claude_code.api_response_body`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"api_response_body"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 세션 내 이벤트 순서 지정을 위한 단조 증가 카운터
* `body`: JSON 직렬화된 Messages API 응답 (id, 콘텐츠 블록, 사용, 중지 이유) (60KB에서 잘림). 확장 사고 콘텐츠는 수정됩니다. 인라인 모드 (`OTEL_LOG_RAW_API_BODIES=1`)에서만 내보내집니다.
* `body_ref`: 잘리지 않은 본문을 포함하는 `<dir>/<request_id>.response.json` 파일의 절대 경로. 파일 모드 (`OTEL_LOG_RAW_API_BODIES=file:<dir>`)에서만 내보내집니다.
* `body_length`: 잘리지 않은 본문 길이. `OTEL_LOG_RAW_API_BODIES=file:<dir>`일 때 UTF-8 바이트, `=1`일 때 UTF-16 코드 단위
* `body_truncated`: 인라인 잘림이 발생했을 때 `"true"`. 파일 모드 및 잘림이 발생하지 않았을 때는 없음
* `model`: 모델 식별자
* `query_source`: 요청을 발급한 하위 시스템
* `request_id`: 응답의 `request-id` 헤더의 Anthropic API 요청 ID, 예: `"req_011..."`. API가 반환할 때만 표시됩니다.

<h4 id="tool-decision-event">
  도구 결정 이벤트
</h4>

도구 권한 결정이 내려질 때 기록됩니다 (수락/거부).

**이벤트 이름**: `claude_code.tool_decision`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"tool_decision"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 세션 내 이벤트 순서 지정을 위한 단조 증가 카운터
* `tool_name`: 도구의 이름 (예: "Read", "Edit", "Write", "NotebookEdit")
* `tool_use_id`: 이 도구 호출의 고유 식별자. 훅에 전달된 `tool_use_id`와 일치하여 OTel 이벤트와 훅 캡처 데이터 간의 상관관계를 허용합니다.
* `decision`: `"accept"` 또는 `"reject"`
* `source`: 결정 출처:
  * `"config"`: 프로젝트 설정, 사용자의 개인 설정의 허용 규칙, 엔터프라이즈 관리 정책, `--allowedTools` 또는 `--disallowedTools` 플래그, 활성 권한 모드, 같은 대화형 CLI 세션의 이전 프롬프트에서의 세션 범위 부여 또는 도구가 본질적으로 안전하기 때문에 프롬프트 없이 자동으로 결정됨. 이벤트는 이러한 출처 중 어느 것이 일치했는지 나타내지 않습니다.
  * `"hook"`: `PreToolUse` 또는 `PermissionRequest` 훅이 결정을 반환함.
  * `"user_permanent"`: 사용자가 권한 프롬프트에서 "예, 그리고 ... 다시 묻지 마세요"를 선택하여 개인 설정에 허용 규칙을 저장했을 때 내보내집니다. 대화형 CLI에서는 해당 선택 자체에 대해서만 내보내집니다. 나중에 저장된 규칙과 일치하는 호출은 대신 `"config"`을 내보냅니다. Agent SDK 또는 비대화형 `-p` 세션에서는 초기 선택과 나중의 규칙 일치 모두 `"user_permanent"`를 내보냅니다. 수락으로 처리됨.
  * `"user_temporary"`: 사용자가 권한 프롬프트에서 "예"를 선택했거나 파일 편집 또는 읽기 프롬프트에서 "이 세션 중" 옵션 중 하나를 선택했을 때 내보내집니다. 대화형 CLI에서는 선택 자체에 대해서만 내보내집니다. 나중에 해당 세션 범위 부여와 일치하는 호출은 대신 `"config"`을 내보냅니다. Agent SDK 또는 비대화형 `-p` 세션에서는 선택과 나중의 일치 모두 `"user_temporary"`를 내보냅니다. 수락으로 처리됨.
  * `"user_abort"`: 사용자가 답변 없이 권한 프롬프트를 닫았을 때 내보내집니다. 거부로 처리됨.
  * `"user_reject"`: 사용자가 프롬프트될 때 "아니오"를 선택했을 때 내보내집니다. 대화형 CLI에서는 해당 선택 자체에 대해서만 내보내집니다. 사용자의 개인 설정의 거부 규칙과 일치하는 호출은 대신 `"config"`을 내보냅니다. Agent SDK 또는 비대화형 `-p` 세션에서는 개인 설정의 거부 규칙과 일치하는 호출이 `"user_reject"`를 내보냅니다. 거부로 처리됨.
* `tool_parameters` (`OTEL_LOG_TOOL_DETAILS=1`일 때): 도구별 매개변수를 포함하는 JSON 문자열. [도구 결과 이벤트](#tool-result-event)와 동일한 형태이지만 `git_commit_id`와 같은 실행 후 필드는 제외됩니다. 권한 결정이 `updatedInput`을 통해 도구 입력을 다시 쓸 경우 수락된 호출의 `tool_result`와 값이 다를 수 있습니다. 이 속성을 사용하여 `decision`이 `"reject"`일 때 어떤 명령이 거부되었는지 확인합니다.
  * Bash 도구의 경우: `bash_command`, `full_command`, `timeout`, `description`, `dangerouslyDisableSandbox` 포함
  * WorkspaceBash 도구의 경우: `bash_command`, `full_command`, `timeout` 포함
  * MCP 도구의 경우: `mcp_server_name`, `mcp_tool_name` 포함
  * Skill 도구의 경우: `skill_name` 포함
  * Agent 도구 또는 레거시 Task 도구의 경우: `subagent_type` 포함

<h4 id="permission-mode-changed-event">
  권한 모드 변경 이벤트
</h4>

권한 모드가 변경될 때 기록됩니다 (예: `Shift+Tab` 순환, 계획 모드 종료 또는 자동 모드 게이트 확인).

**이벤트 이름**: `claude_code.permission_mode_changed`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"permission_mode_changed"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 세션 내 이벤트 순서 지정을 위한 단조 증가 카운터
* `from_mode`: 이전 권한 모드, 예: `"default"`, `"plan"`, `"acceptEdits"`, `"auto"` 또는 `"bypassPermissions"`
* `to_mode`: 새 권한 모드
* `trigger`: 변경을 야기한 것. `"shift_tab"`, `"exit_plan_mode"`, `"auto_gate_denied"` 또는 `"auto_opt_in"` 중 하나. SDK 또는 브리지에서 전환이 시작될 때는 없음

<h4 id="auth-event">
  인증 이벤트
</h4>

`/login` 또는 `/logout`이 완료될 때 기록됩니다.

**이벤트 이름**: `claude_code.auth`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"auth"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 세션 내 이벤트 순서 지정을 위한 단조 증가 카운터
* `action`: `"login"` 또는 `"logout"`
* `success`: `"true"` 또는 `"false"`
* `auth_method`: 인증 방법, 예: `"oauth"`
* `error_category`: 작업이 실패했을 때 범주별 오류 종류. 원본 오류 메시지는 포함되지 않습니다
* `status_code`: 작업이 HTTP 오류로 실패했을 때 HTTP 상태 코드 (문자열)

<h4 id="mcp-server-connection-event">
  MCP 서버 연결 이벤트
</h4>

MCP 서버가 연결, 연결 해제 또는 연결 실패할 때 기록됩니다.

**이벤트 이름**: `claude_code.mcp_server_connection`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"mcp_server_connection"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 세션 내 이벤트 순서 지정을 위한 단조 증가 카운터
* `status`: `"connected"`, `"failed"` 또는 `"disconnected"`
* `transport_type`: 서버 전송, 예: `"stdio"`, `"sse"` 또는 `"http"`
* `server_scope`: 서버가 구성된 범위, 예: `"user"`, `"project"` 또는 `"local"`
* `duration_ms`: 연결 시도 지속 시간 (밀리초)
* `error_code`: 연결이 실패했을 때 오류 코드
* `is_plugin`: 서버가 플러그인에서 제공될 때 `true`, 그 외에는 `false`
* `plugin_id_hash` (`is_plugin`이 `true`일 때): 플러그인 이름 및 마켓플레이스의 안정적인 해시 (이름을 노출하지 않고 플러그인별로 이벤트를 그룹화하기 위함)
* `plugin.name` (`is_plugin`이 `true`일 때): 서버를 제공하는 플러그인의 이름. 타사 플러그인의 경우 `OTEL_LOG_TOOL_DETAILS=1`이 아니면 리터럴 문자열 `"third-party"`입니다. 공식 Anthropic 소스의 플러그인은 항상 이름으로 식별됩니다. `plugin_id_hash` 및 `plugin.name` 속성은 자신의 모니터링 백엔드로 흐르며 Anthropic으로 전송되지 않습니다
* `server_name` (`OTEL_LOG_TOOL_DETAILS=1`일 때): 구성된 서버 이름
* `error` (`OTEL_LOG_TOOL_DETAILS=1`일 때): 연결이 실패했을 때 전체 오류 메시지

<h4 id="internal-error-event">
  내부 오류 이벤트
</h4>

Claude Code가 예상치 못한 내부 오류를 포착할 때 기록됩니다. 오류 클래스 이름과 errno 스타일 코드만 기록됩니다. 오류 메시지 및 스택 추적은 포함되지 않습니다. 이 이벤트는 Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry에 대해 실행하거나 `DISABLE_ERROR_REPORTING`이 설정되어 있을 때는 내보내지지 않습니다.

**이벤트 이름**: `claude_code.internal_error`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"internal_error"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 세션 내 이벤트 순서 지정을 위한 단조 증가 카운터
* `error_name`: 오류 클래스 이름, 예: `"TypeError"` 또는 `"SyntaxError"`
* `error_code`: 오류에 있을 때 Node.js errno 코드, 예: `"ENOENT"`

<h4 id="plugin-installed-event">
  플러그인 설치됨 이벤트
</h4>

플러그인이 설치를 완료할 때 기록됩니다 (`claude plugin install` CLI 명령 및 대화형 `/plugin` UI 모두).

**이벤트 이름**: `claude_code.plugin_installed`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"plugin_installed"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 세션 내 이벤트 순서 지정을 위한 단조 증가 카운터
* `marketplace.is_official`: 마켓플레이스가 공식 Anthropic 마켓플레이스인 경우 `"true"`, 그 외에는 `"false"`
* `install.trigger`: `"cli"` 또는 `"ui"`
* `plugin.name`: 설치된 플러그인의 이름. 타사 마켓플레이스의 경우 `OTEL_LOG_TOOL_DETAILS=1`일 때만 포함됩니다
* `plugin.version`: 마켓플레이스 항목에 선언된 경우 플러그인 버전. 타사 마켓플레이스의 경우 `OTEL_LOG_TOOL_DETAILS=1`일 때만 포함됩니다
* `marketplace.name`: 플러그인이 설치된 마켓플레이스. 타사 마켓플레이스의 경우 `OTEL_LOG_TOOL_DETAILS=1`일 때만 포함됩니다

<h4 id="plugin-loaded-event">
  플러그인 로드됨 이벤트
</h4>

세션 시작 시 활성화된 플러그인당 한 번 기록됩니다. 이 이벤트를 사용하여 플릿 전체에서 활성화된 플러그인을 인벤토리화합니다. 설치 작업 자체를 기록하는 `plugin_installed`를 보완합니다.

**이벤트 이름**: `claude_code.plugin_loaded`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"plugin_loaded"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 세션 내 이벤트 순서 지정을 위한 단조 증가 카운터
* `plugin.name`: 플러그인의 이름. 공식 마켓플레이스 및 기본 제공 번들 외부의 플러그인의 경우 `OTEL_LOG_TOOL_DETAILS=1`이 아니면 값은 `"third-party"`입니다
* `marketplace.name`: 플러그인이 설치된 마켓플레이스 (알려진 경우). `plugin.name`과 동일한 조건에서 `"third-party"`로 수정됩니다
* `plugin.version`: 플러그인 매니페스트의 버전. 이름이 수정되지 않고 매니페스트가 버전을 선언할 때만 포함됩니다
* `plugin.scope`: 플러그인의 출처 범주: `"official"`, `"org"`, `"user-local"` 또는 `"default-bundle"`
* `enabled_via`: 플러그인이 활성화된 방식: `"default-enable"`, `"org-policy"`, `"seed-mount"` 또는 `"user-install"`
* `plugin_id_hash`: 플러그인 이름 및 마켓플레이스의 결정론적 해시 (구성된 내보내기로만 전송됨). 플릿 전체에서 로드된 서로 다른 타사 플러그인 수를 세는 것을 허용합니다 (이름 기록 없이)
* `has_hooks`: 플러그인이 훅을 제공하는지 여부
* `has_mcp`: 플러그인이 MCP 서버를 제공하는지 여부
* `host_owned_mcp`: SDK 호스트가 이 플러그인의 MCP 연결을 관리하고 Claude Code가 플러그인의 MCP 서버 구성 읽기를 건너뛸 때 `true`, 그 외에는 `false`. Claude Code v2.1.172 이상 필요
* `skill_path_count`: 플러그인이 선언하는 스킬 디렉토리 수
* `command_path_count`: 플러그인이 선언하는 명령 디렉토리 수
* `agent_path_count`: 플러그인이 선언하는 에이전트 디렉토리 수
* `safe_mode`: 세션이 [`--safe-mode`](/docs/ko/cli-reference)로 시작되었을 때 `"true"`, 그 외에는 `"false"`. 안전 모드에서 이 이벤트는 구성된 인벤토리만 보고합니다. 플러그인의 명령, 스킬, 훅 및 MCP 서버는 로드되지 않습니다. Claude Code v2.1.169 이상 필요

<h4 id="skill-activated-event">
  스킬 활성화됨 이벤트
</h4>

스킬이 호출될 때 기록됩니다. Claude가 Skill 도구를 통해 호출하든 `/` 명령으로 실행하든 상관없습니다.

**이벤트 이름**: `claude_code.skill_activated`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"skill_activated"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 세션 내 이벤트 순서 지정을 위한 단조 증가 카운터
* `skill.name`: 스킬의 이름. 사용자 정의 및 타사 플러그인 스킬의 경우 `OTEL_LOG_TOOL_DETAILS=1`이 아니면 값은 자리 표시자 `"custom_skill"`입니다
* `invocation_trigger`: 스킬이 트리거된 방식 (`"user-slash"`, `"claude-proactive"` 또는 `"nested-skill"`)
* `skill.source`: 스킬이 로드된 위치 (예: `"bundled"`, `"userSettings"`, `"projectSettings"`, `"plugin"`)
* `skill.kind`: 스킬이 워크플로우 스킬일 때 `"workflow"`. 그 외에는 없음
* `plugin.name` (`OTEL_LOG_TOOL_DETAILS=1`이거나 플러그인이 공식 마켓플레이스에서 온 경우): 스킬이 플러그인에서 제공될 때 소유 플러그인의 이름
* `marketplace.name` (`OTEL_LOG_TOOL_DETAILS=1`이거나 플러그인이 공식 마켓플레이스에서 온 경우): 스킬이 플러그인에서 제공될 때 소유 플러그인이 설치된 마켓플레이스

<h4 id="at-mention-event">
  @ 멘션 이벤트
</h4>

Claude Code가 프롬프트에서 `@`-멘션을 해석할 때 기록됩니다. 모든 멘션이 이벤트를 내보내는 것은 아닙니다: 권한 거부, 파일 크기 초과, PDF 참조 첨부, 디렉토리 목록 실패와 같은 조기 종료 경로는 로깅 없이 반환됩니다.

**이벤트 이름**: `claude_code.at_mention`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"at_mention"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 세션 내 이벤트 순서 지정을 위한 단조 증가 카운터
* `mention_type`: 멘션의 유형 (`"file"`, `"directory"`, `"agent"`, `"mcp_resource"`)
* `success`: 멘션이 성공적으로 해석되었는지 여부 (`"true"` 또는 `"false"`)

<h4 id="api-retries-exhausted-event">
  API 재시도 소진됨 이벤트
</h4>

API 요청이 두 번 이상 시도 후 실패할 때 한 번 기록됩니다. 최종 `api_error` 이벤트와 함께 내보내집니다.

**이벤트 이름**: `claude_code.api_retries_exhausted`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"api_retries_exhausted"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 세션 내 이벤트 순서 지정을 위한 단조 증가 카운터
* `model`: 사용된 모델
* `error`: 최종 오류 메시지
* `status_code`: HTTP 상태 코드 (숫자). HTTP가 아닌 오류의 경우 없음
* `total_attempts`: 수행된 총 시도 횟수
* `total_retry_duration_ms`: 모든 시도에 걸친 총 벽시계 시간
* `speed`: `"fast"` 또는 `"normal"`

<h4 id="hook-registered-event">
  훅 등록됨 이벤트
</h4>

세션 시작 시 구성된 훅당 한 번 기록됩니다. 이 이벤트를 사용하여 플릿 전체에서 활성화된 훅을 인벤토리화합니다. 실행별 `hook_execution_start` 및 `hook_execution_complete` 이벤트를 보완합니다.

**이벤트 이름**: `claude_code.hook_registered`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"hook_registered"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 세션 내 이벤트 순서 지정을 위한 단조 증가 카운터
* `hook_event`: 훅 이벤트 유형, 예: `"PreToolUse"` 또는 `"PostToolUse"`
* `hook_type`: 훅 구현 유형: `"command"`, `"prompt"`, `"mcp_tool"`, `"http"` 또는 `"agent"`
* `hook_source`: 훅이 정의된 위치: `"userSettings"`, `"projectSettings"`, `"localSettings"`, `"flagSettings"`, `"policySettings"` 또는 `"pluginHook"`
* `safe_mode`: 세션이 [`--safe-mode`](/docs/ko/cli-reference)로 시작되었을 때 `"true"`, 그 외에는 `"false"`. Claude Code v2.1.169 이상 필요
* `hook_matcher` (`OTEL_LOG_TOOL_DETAILS=1`일 때): 설정된 경우 훅 구성의 매처 문자열
* `plugin.name` (`hook_source`가 `"pluginHook"`일 때): 기여하는 플러그인의 이름. 공식 마켓플레이스 및 기본 제공 번들 외부의 플러그인의 경우 `OTEL_LOG_TOOL_DETAILS=1`이 아니면 값은 `"third-party"`입니다
* `plugin_id_hash` (`hook_source`가 `"pluginHook"`일 때): 플러그인 이름 및 마켓플레이스의 결정론적 해시 (구성된 내보내기로만 전송됨). 이름을 기록하지 않고 기여하는 서로 다른 플러그인을 세는 것을 허용합니다

<h4 id="hook-execution-start-event">
  훅 실행 시작 이벤트
</h4>

하나 이상의 훅이 훅 이벤트에 대해 실행을 시작할 때 기록됩니다.

**이벤트 이름**: `claude_code.hook_execution_start`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"hook_execution_start"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 세션 내 이벤트 순서 지정을 위한 단조 증가 카운터
* `hook_event`: 훅 이벤트 유형, 예: `"PreToolUse"` 또는 `"PostToolUse"`
* `hook_name`: 매처를 포함한 전체 훅 이름, 예: `"PreToolUse:Write"`
* `num_hooks`: 일치하는 훅 명령 수
* `managed_only`: 관리 정책 훅만 허용될 때 `"true"`
* `hook_source`: `"policySettings"` 또는 `"merged"`
* `safe_mode`: 세션이 [`--safe-mode`](/docs/ko/cli-reference)로 시작되었을 때 `"true"`, 그 외에는 `"false"`. Claude Code v2.1.169 이상 필요
* `hook_definitions`: JSON 직렬화된 훅 구성. 상세 베타 추적과 `OTEL_LOG_TOOL_DETAILS=1`이 모두 활성화되어 있을 때만 포함됨

<h4 id="hook-execution-complete-event">
  훅 실행 완료 이벤트
</h4>

훅 이벤트의 모든 훅이 완료되었을 때 기록됩니다.

**이벤트 이름**: `claude_code.hook_execution_complete`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"hook_execution_complete"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 세션 내 이벤트 순서 지정을 위한 단조 증가 카운터
* `hook_event`: 훅 이벤트 유형
* `hook_name`: 매처를 포함한 전체 훅 이름
* `num_hooks`: 일치하는 훅 명령 수
* `num_success`: 성공적으로 완료된 수
* `num_blocking`: 차단 결정을 반환한 수
* `num_non_blocking_error`: 차단 없이 실패한 수
* `num_cancelled`: 완료 전에 취소된 수
* `total_duration_ms`: 모든 일치하는 훅의 벽시계 지속 시간
* `managed_only`: 관리 정책 훅만 허용될 때 `"true"`
* `hook_source`: `"policySettings"` 또는 `"merged"`
* `safe_mode`: 세션이 [`--safe-mode`](/docs/ko/cli-reference)로 시작되었을 때 `"true"`, 그 외에는 `"false"`. Claude Code v2.1.169 이상 필요
* `hook_definitions`: JSON 직렬화된 훅 구성. 상세 베타 추적과 `OTEL_LOG_TOOL_DETAILS=1`이 모두 활성화되어 있을 때만 포함됨

<h4 id="hook-plugin-metrics-event">
  훅 플러그인 메트릭 이벤트
</h4>

공식 마켓플레이스 플러그인 훅이 호출별 메트릭을 내보낼 때 기록됩니다. 공식 Anthropic 마켓플레이스에서 설치된 플러그인만 이를 내보낼 수 있습니다. 타사 마켓플레이스 플러그인 및 사용자 구성 훅은 이 이벤트로 내보내지 않습니다. 이 이벤트를 사용하여 플러그인 동작 (예: 찾기 비율, 비용, 지속 시간)을 자신의 관찰성 스택에서 모니터링합니다.

**이벤트 이름**: `claude_code.hook_plugin_metrics`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"hook_plugin_metrics"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 세션 내 이벤트 순서 지정을 위한 단조 증가 카운터
* `plugin_id`: `<name>@<marketplace>` 형식의 플러그인 식별자
* `hook_event`: 메트릭을 내보낸 훅 이벤트 유형
* 최대 20개의 플러그인 내보낸 메트릭 키. 이름은 `^[a-z][a-z0-9_]{0,39}$`와 일치합니다. 값은 부울 또는 숫자입니다.

<h4 id="compaction-event">
  압축 이벤트
</h4>

대화 압축이 완료될 때 기록됩니다.

**이벤트 이름**: `claude_code.compaction`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"compaction"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 세션 내 이벤트 순서 지정을 위한 단조 증가 카운터
* `trigger`: `"auto"` 또는 `"manual"`
* `success`: `"true"` 또는 `"false"`
* `duration_ms`: 압축 지속 시간
* `pre_tokens`: 압축 전 대략적인 토큰 수
* `post_tokens`: 압축 후 대략적인 토큰 수
* `error`: 압축이 실패했을 때 오류 메시지
* `precompute_reuse`: `trigger`가 `"manual"`일 때만 설정됩니다. 자동 압축은 컨텍스트 윈도우가 가득 차기 전에 백그라운드에서 요약을 준비할 수 있으며, 이 속성은 `/compact`가 해당 준비된 요약을 재사용했는지 기록합니다. `"hit"`은 재사용되었음을 의미합니다. `"miss_custom_instructions"`, `"miss_hook"` 및 `"miss_not_ready"`는 대신 새로운 요약이 계산된 이유를 제공합니다. Claude Code v2.1.153 이상 필요

<h4 id="feedback-survey-event">
  피드백 설문 이벤트
</h4>

세션 품질 설문이 표시되거나 답변될 때 기록됩니다. [세션 품질 설문](/docs/ko/data-usage#session-quality-surveys)을 참조하여 설문이 수집하는 내용과 제어 방법을 확인하세요.

**이벤트 이름**: `claude_code.feedback_survey`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"feedback_survey"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 세션 내 이벤트 순서 지정을 위한 단조 증가 카운터
* `event_type`: 설문 수명 주기 이벤트, 예: `"appeared"`, `"responded"` 또는 `"transcript_prompt_appeared"`
* `appearance_id`: 하나의 설문 인스턴스에 대해 내보내진 이벤트를 연결하는 고유 ID
* `survey_type`: 이벤트를 생성한 설문. `"session"`은 "Claude가 어떻게 하고 있나요?" 평가 프롬프트입니다
* `response`: `responded` 이벤트에서 사용자의 선택
* `enabled_via_override`: [`CLAUDE_CODE_ENABLE_FEEDBACK_SURVEY_FOR_OTEL`](/docs/ko/env-vars)이 설정되어 있을 때 `true`. 문자열이 아닌 부울로 내보내집니다. `session` 설문 이벤트에 표시됩니다. 이 속성을 필터링하여 플릿 전체에서 재정의가 적용되었는지 확인합니다

<h2 id="interpret-metrics-and-events-data">
  메트릭 및 이벤트 데이터 해석
</h2>

내보낸 메트릭 및 이벤트는 다양한 분석을 지원합니다:

<h3 id="usage-monitoring">
  사용 모니터링
</h3>

| 메트릭                                                           | 분석 기회                                                                        |
| ------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| `claude_code.token.usage`                                     | `type` (입력/출력), 사용자, 팀, 모델, `skill.name`, `plugin.name` 또는 `agent.name`별로 분류 |
| `claude_code.session.count`                                   | 시간 경과에 따른 채택 및 참여 추적                                                         |
| `claude_code.lines_of_code.count`                             | 코드 추가 및 제거를 추적하여 생산성 측정, 모델별로 분류                                             |
| `claude_code.commit.count` & `claude_code.pull_request.count` | 개발 워크플로우에 미치는 영향 이해                                                          |

<h3 id="cost-monitoring">
  비용 모니터링
</h3>

`claude_code.cost.usage` 메트릭은 다음에 도움이 됩니다:

* 팀 또는 개인 전체의 사용 추세 추적
* 최적화를 위한 높은 사용 세션 식별
* `skill.name`, `plugin.name` 및 `agent.name` 속성을 통해 특정 스킬, 플러그인 또는 서브에이전트 유형에 지출 귀속

<Note>
  비용 메트릭은 근사값입니다. 공식 청구 데이터는 API 제공자(Claude Console, Amazon Bedrock 또는 Google Cloud의 Agent Platform)를 참조하세요.
</Note>

<h3 id="alerting-and-segmentation">
  경고 및 세분화
</h3>

일반적인 경고 고려 사항:

* 비용 급증
* 비정상적인 토큰 소비
* 특정 사용자의 높은 세션 볼륨

모든 메트릭은 [표준 속성](#standard-attributes)으로 세분화할 수 있습니다. `model` 속성은 `claude_code.token.usage`, `claude_code.cost.usage`에서 사용 가능하며, v2.1.172부터 `claude_code.lines_of_code.count`에서도 사용 가능합니다. 커밋의 모델별 분류는 한 세션이 여러 모델에 걸쳐 있을 수 있으므로 `session.id`에서 토큰 또는 비용 메트릭에 대해 조인하여만 근사할 수 있습니다. 토큰 또는 비용 측면을 `query_source`가 `"main"`인 행으로 필터링하여 보조 및 서브에이전트 요청이 세션의 커밋을 해당 요청을 수행하지 않은 모델에 귀속시키지 않도록 합니다.

<h3 id="detect-retry-exhaustion">
  재시도 소진 감지
</h3>

Claude Code는 실패한 API 요청을 내부적으로 재시도하고 포기한 후에만 단일 `claude_code.api_error` 이벤트를 내보내므로 이벤트 자체가 해당 요청의 최종 신호입니다. 중간 재시도 시도는 별도의 이벤트로 기록되지 않습니다.

이벤트의 `attempt` 속성은 총 시도 횟수를 기록합니다. `CLAUDE_CODE_MAX_RETRIES`는 기본값이 10이고 최대 15입니다. v2.1.199부터 `CLAUDE_CODE_RETRY_WATCHDOG`은 기본값을 높이고 상한을 제거합니다. 요청이 일시적 오류에 대한 모든 재시도를 소진하면 `attempt`는 해당 유효 제한보다 하나 많습니다: 기본값으로는 11이고 감시 기능이 설정되지 않은 경우 16을 초과하지 않습니다. 더 낮은 값은 `400` 응답과 같은 재시도 불가능한 오류를 나타냅니다.

복구된 세션과 정체된 세션을 구분하려면 `session.id`로 이벤트를 그룹화하고 오류 후 나중에 `api_request` 이벤트가 존재하는지 확인합니다.

<h3 id="event-analysis">
  이벤트 분석
</h3>

이벤트 데이터는 Claude Code 상호 작용에 대한 자세한 통찰력을 제공합니다:

**도구 사용 패턴**: 도구 결과 이벤트를 분석하여 다음을 식별합니다:

* 가장 자주 사용되는 도구
* 도구 성공률
* 평균 도구 실행 시간
* 도구 유형별 오류 패턴

**성능 모니터링**: API 요청 지속 시간 및 도구 실행 시간을 추적하여 성능 병목 현상을 식별합니다.

<h2 id="audit-security-events">
  감사 보안 이벤트
</h2>

OpenTelemetry 이벤트는 Claude Code 활동의 감사 데이터 소스입니다. 모든 이벤트는 도구 호출, MCP 활동 및 권한 결정을 해당 이벤트를 트리거한 사용자에게 연결하는 ID 속성을 전달하며, OTLP 로그 내보내기는 이러한 이벤트를 OTLP 수신기가 있는 모든 SIEM(Security Information and Event Management) 플랫폼 또는 SIEM으로 전달하는 OpenTelemetry Collector에 전달할 수 있습니다.

<h3 id="attribute-actions-to-users">
  속성 작업을 사용자에게 연결
</h3>

각 이벤트의 [표준 속성](#standard-attributes)에는 인증된 사용자의 ID가 포함됩니다: Claude 계정으로 로그인할 때 `user.email`, `user.account_uuid`, `user.account_id` 및 `organization.id`, 그리고 설치 범위 `user.id` 및 세션별 `session.id`. `user.id`는 설치 범위 식별자이며, [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway) 세션에서는 게이트웨이 발급 토큰의 IdP 주체입니다.

MCP 도구 호출, Bash 명령 및 파일 편집은 따라서 세션을 시작한 개발자에게 귀속됩니다. Claude Code는 별도의 서비스 계정으로 작동하지 않습니다. 각 이벤트에 기록된 ID는 개발자 자신의 Claude 계정이거나 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway) 세션의 개발자 IdP 신원입니다.

Claude Code가 직접 API 키로 인증하거나 Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry에 대해 인증할 때 세션에 Claude 계정이 없으며 `user.id` 및 `session.id`만 채워집니다. 이러한 배포에서는 `OTEL_RESOURCE_ATTRIBUTES`를 사용하여 사용자 ID를 직접 첨부하고, [관리 설정](#administrator-configuration) 파일 또는 시작 래퍼를 통해 사용자별로 설정합니다. Claude 앱 게이트웨이 세션은 이 중 어느 것도 필요하지 않습니다: CLI는 [표준 속성](#standard-attributes)에 설명된 대로 IdP 신원을 자동으로 스탬프합니다.

```bash theme={null}
export OTEL_RESOURCE_ATTRIBUTES="enduser.id=jdoe@example.com,enduser.directory_id=S-1-5-21-..."
```

<h3 id="audit-mcp-activity">
  MCP 활동 감사
</h3>

전체 호출 세부 정보로 MCP 서버 활동을 캡처하려면 로그 내보내기를 활성화하고 `OTEL_LOG_TOOL_DETAILS=1`을 설정합니다. 각 MCP 작업은 표준 ID 속성과 함께 서버 이름, 도구 이름 및 호출 인수를 전달하는 구조화된 이벤트를 생성합니다:

| 이벤트                     | MCP에 대해 기록하는 것                                                                                                                                     |
| ----------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| `mcp_server_connection` | 서버 연결, 연결 해제 및 연결 실패 (`server_name`, `transport_type`, `server_scope` 및 오류 세부 정보 포함)                                                               |
| `tool_result`           | 각 MCP 도구 호출 (`tool_name` 및 `mcp_server_scope` 포함, `mcp_server_name` 및 `mcp_tool_name`을 포함하는 `tool_parameters` 페이로드, 호출 인수를 포함하는 `tool_input` 페이로드) |
| `tool_decision`         | 호출이 허용되었는지 거부되었는지, 그리고 결정이 구성, 훅 또는 사용자에서 나왔는지 여부, 그리고 `mcp_server_name` 및 `mcp_tool_name`을 포함하는 `tool_parameters` 페이로드                            |

`OTEL_LOG_TOOL_DETAILS` 없이 이러한 이벤트는 식별 세부 정보를 삭제합니다:

* `tool_result`: `tool_name` 및 `mcp_server_scope`를 유지하고, `mcp_server_name`, `mcp_tool_name` 및 인수를 생략합니다
* `tool_decision`: `tool_name`을 유지하고, `tool_parameters`를 생략합니다
* `mcp_server_connection`: `server_name` 및 오류 메시지를 생략하지만, `is_plugin`, `plugin_id_hash` 및 `plugin.name`을 유지하며, Anthropic이 아닌 플러그인 이름은 리터럴 `"third-party"`로 수정되므로 플러그인 제공 서버는 상세 로깅 없이도 구별 가능합니다

<h3 id="map-security-questions-to-events">
  보안 질문을 이벤트에 매핑
</h3>

감지 규칙을 구축할 때 모니터링하려는 신호를 찾고 해당 이벤트 및 속성에 대해 백엔드를 쿼리합니다:

| 신호                      | 이벤트                                                                         | 주요 속성                                                        |
| ----------------------- | --------------------------------------------------------------------------- | ------------------------------------------------------------ |
| 도구 호출 허용 또는 거부, 그리고 어떻게 | `tool_decision`                                                             | `decision`, `source`, `tool_name`, `tool_parameters`         |
| 권한 모드 에스컬레이션            | `permission_mode_changed`                                                   | `from_mode`, `to_mode`, `trigger`                            |
| 정책 훅이 작업을 차단함           | `hook_execution_complete`                                                   | `hook_event`, `num_blocking`                                 |
| 로그인, 로그아웃 및 인증 실패       | `auth`                                                                      | `action`, `success`, `error_category`                        |
| MCP 서버 연결 또는 실패         | `mcp_server_connection`                                                     | `status`, `server_name`, `is_plugin`, `error_code`           |
| 플러그인 설치 및 출처            | `plugin_installed`                                                          | `plugin.name`, `marketplace.name`, `marketplace.is_official` |
| 실행된 명령 및 터치된 파일         | `tool_result` (실행됨) 또는 `tool_decision` (거부됨) (`OTEL_LOG_TOOL_DETAILS=1` 포함) | `tool_parameters`; `tool_input` (`tool_result`만 해당)          |

Claude Code는 원본 이벤트 스트림만 내보냅니다. 이상 감지, 기준선 설정, 세션 간 상관 관계 및 경고는 SIEM 또는 관찰성 백엔드의 책임입니다.

<h3 id="send-events-to-a-siem">
  SIEM에 이벤트 전송
</h3>

`OTEL_EXPORTER_OTLP_LOGS_ENDPOINT`를 SIEM의 OTLP 수신기 또는 SIEM의 기본 수집 API로 전달하는 OpenTelemetry Collector로 지정합니다. 다음 관리 설정 예는 이벤트만 내보내고 MCP 및 Bash 감사를 위해 전체 도구 세부 정보를 활성화합니다:

```json theme={null}
{
  "env": {
    "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
    "OTEL_LOGS_EXPORTER": "otlp",
    "OTEL_LOG_TOOL_DETAILS": "1",
    "OTEL_EXPORTER_OTLP_LOGS_PROTOCOL": "http/protobuf",
    "OTEL_EXPORTER_OTLP_LOGS_ENDPOINT": "https://siem.example.com:4318/v1/logs",
    "OTEL_EXPORTER_OTLP_HEADERS": "Authorization=Bearer your-siem-token"
  }
}
```

<h2 id="backend-considerations">
  백엔드 고려 사항
</h2>

메트릭, 로그 및 추적 백엔드 선택은 수행할 수 있는 분석 유형을 결정합니다:

<h3 id="for-metrics">
  메트릭의 경우
</h3>

* **시계열 데이터베이스 (예: Prometheus)**: 비율 계산, 집계된 메트릭
* **컬럼형 저장소 (예: ClickHouse)**: 복잡한 쿼리, 고유 사용자 분석
* **완전한 기능의 관찰성 플랫폼 (예: Honeycomb, Datadog, Grafana Cloud)**: 고급 쿼리, 시각화, 경고

<h3 id="for-events/logs">
  이벤트/로그의 경우
</h3>

* **로그 집계 시스템 (예: Elasticsearch, Loki)**: 전체 텍스트 검색, 로그 분석
* **컬럼형 저장소 (예: ClickHouse)**: 구조화된 이벤트 분석
* **완전한 기능의 관찰성 플랫폼 (예: Honeycomb, Datadog, Grafana Cloud)**: 메트릭과 이벤트 간의 상관 관계

<h3 id="for-traces">
  추적의 경우
</h3>

분산 추적 저장소 및 스팬 상관 관계를 지원하는 백엔드를 선택합니다:

* **분산 추적 시스템 (예: Jaeger, Zipkin, Grafana Tempo)**: 스팬 시각화, 요청 워터폴, 지연 시간 분석
* **완전한 기능의 관찰성 플랫폼 (예: Honeycomb, Datadog, Grafana Cloud)**: 추적 검색 및 메트릭과 로그와의 상관 관계

일일/주간/월간 활성 사용자 (DAU/WAU/MAU) 메트릭이 필요한 조직의 경우 효율적인 고유 값 쿼리를 지원하는 백엔드를 고려하세요.

<h2 id="service-information">
  서비스 정보
</h2>

모든 메트릭 및 이벤트는 다음 리소스 속성과 함께 내보내집니다:

* `service.name`: `claude-code`
* `service.version`: 현재 Claude Code 버전
* `os.type`: 운영 체제 유형 (예: `linux`, `darwin`, `windows`)
* `os.version`: 운영 체제 버전 문자열
* `host.arch`: 호스트 아키텍처 (예: `amd64`, `arm64`)
* `wsl.version`: WSL 버전 번호 (Windows Subsystem for Linux에서 실행할 때만 표시)
* 미터 이름: `com.anthropic.claude_code`

<h2 id="roi-measurement-resources">
  ROI 측정 리소스
</h2>

Claude Code의 투자 수익률 측정에 대한 포괄적인 가이드(원격 측정 설정, 비용 분석, 생산성 메트릭 및 자동화된 보고 포함)는 [Claude Code ROI 측정 가이드](https://github.com/anthropics/claude-code-monitoring-guide)를 참조하세요. 이 저장소는 즉시 사용 가능한 Docker Compose 구성, Prometheus 및 OpenTelemetry 설정, Linear와 같은 도구와 통합된 생산성 보고서 생성 템플릿을 제공합니다.

<h2 id="security-and-privacy">
  보안 및 개인 정보 보호
</h2>

* OpenTelemetry 내보내기는 선택 사항이며 명시적 구성이 필요합니다. Anthropic의 별도 운영 원격 측정 및 이를 비활성화하는 방법에 대해서는 [데이터 사용](/docs/ko/data-usage#telemetry-services)을 참조하세요
* 원본 파일 콘텐츠 및 코드 스니펫은 메트릭 또는 이벤트에 포함되지 않습니다. 추적 스팬은 별도의 데이터 경로입니다: 아래의 `OTEL_LOG_TOOL_CONTENT` 항목을 참조하세요
* OAuth를 통해 인증된 경우 `user.email`이 원격 측정 속성에 포함됩니다. 조직에서 이것이 우려 사항인 경우 원격 측정 백엔드와 함께 작업하여 이 필드를 필터링하거나 수정하세요
* 사용자 프롬프트 콘텐츠는 기본적으로 수집되지 않습니다. 프롬프트 길이만 기록됩니다. 프롬프트 콘텐츠를 포함하려면 `OTEL_LOG_USER_PROMPTS=1`을 설정하세요
* 어시스턴트 응답 텍스트는 기본적으로 수집되지 않습니다. 응답 길이만 기록됩니다. 응답 텍스트를 포함하려면 `OTEL_LOG_ASSISTANT_RESPONSES=1`을 설정하세요. Claude Code의 모든 OpenTelemetry 데이터와 마찬가지로 응답 텍스트는 구성한 OTel 엔드포인트로만 전송되며 Anthropic으로는 전송되지 않습니다. 이 변수가 설정되지 않으면 `OTEL_LOG_USER_PROMPTS`가 폴백으로 사용되므로 프롬프트 콘텐츠는 원하지만 응답 콘텐츠는 원하지 않는 경우 `OTEL_LOG_ASSISTANT_RESPONSES=0`을 설정하세요
* 도구 입력 인수 및 매개변수는 기본적으로 기록되지 않습니다. 이를 포함하려면 `OTEL_LOG_TOOL_DETAILS=1`을 설정하세요. 이 데이터는 구성한 OTEL 엔드포인트로만 전송되며 Anthropic으로는 전송되지 않습니다. 인수에는 여전히 민감한 값이 포함될 수 있으므로 필요에 따라 이러한 속성을 필터링하거나 수정하도록 원격 측정 백엔드를 구성하세요. 활성화되면:
  * `tool_result` 및 `tool_decision` 이벤트는 Bash 명령, MCP 서버 및 도구 이름, 스킬 이름이 포함된 `tool_parameters` 속성을 포함합니다. `full_command`와 같은 필드는 잘리지 않은 상태로 내보내집니다
  * `tool_result` 이벤트는 추가로 파일 경로, URL, 검색 패턴 및 기타 인수가 포함된 `tool_input` 속성을 포함합니다. 512자를 초과하는 개별 값은 잘리고 전체는 약 4K 문자로 제한됩니다
  * `user_prompt` 이벤트는 사용자 정의, 플러그인 및 MCP 명령의 축자 `command_name`을 포함합니다
  * 추적 스팬은 동일한 `tool_input` 속성 및 `file_path`와 같은 입력 파생 속성을 포함하며, `tool_input`과 동일한 잘림이 적용됩니다
* 도구 입력 및 출력 콘텐츠는 기본적으로 추적 스팬에 기록되지 않습니다. 이를 포함하려면 `OTEL_LOG_TOOL_CONTENT=1`을 설정하세요. 활성화되면 스팬 이벤트는 스팬당 60KB에서 잘린 전체 도구 입력 및 출력 콘텐츠를 포함합니다. 여기에는 Read 도구 결과의 원본 파일 콘텐츠 및 Bash 명령 출력이 포함될 수 있습니다. 필요에 따라 이러한 속성을 필터링하거나 수정하도록 원격 측정 백엔드를 구성하세요
* 원본 Anthropic Messages API 요청 및 응답 본문은 기본적으로 기록되지 않습니다. 이를 포함하려면 `OTEL_LOG_RAW_API_BODIES`를 설정하세요. `=1`일 때 각 API 호출은 `body` 속성이 JSON 직렬화된 페이로드(60KB에서 잘림)인 `api_request_body` 및 `api_response_body` 로그 이벤트를 내보냅니다. `=file:<dir>`일 때 잘리지 않은 본문은 해당 디렉토리 아래의 `.request.json` 및 `.response.json` 파일에 기록되고 이벤트는 인라인 본문 대신 `body_ref` 경로를 전달합니다. 로그 수집기 또는 사이드카와 함께 디렉토리를 배포하되 원격 측정 스트림을 통해서는 배포하지 마세요. 두 모드 모두에서 본문에는 전체 대화 기록(시스템 프롬프트, 모든 이전 사용자 및 어시스턴트 턴, 도구 결과)이 포함되므로 이를 활성화하면 다른 `OTEL_LOG_*` 콘텐츠 플래그가 공개할 모든 것에 동의하는 것을 의미합니다. Claude의 확장 사고 콘텐츠는 다른 설정에 관계없이 항상 이러한 본문에서 수정됩니다

<h2 id="monitor-claude-code-on-amazon-bedrock">
  Amazon Bedrock에서 Claude Code 모니터링
</h2>

Amazon Bedrock의 Claude Code 사용 모니터링에 대한 자세한 지침은 [Claude Code 모니터링 구현 (Amazon Bedrock)](https://github.com/aws-solutions-library-samples/guidance-for-claude-code-with-amazon-bedrock/blob/main/assets/docs/MONITORING.md)을 참조하세요.
