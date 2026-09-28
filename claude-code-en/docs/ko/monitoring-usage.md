> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 모니터링

> Claude Code에 대한 OpenTelemetry를 활성화하고 구성하는 방법을 알아봅니다.

OpenTelemetry(OTel)를 통해 원격 측정 데이터를 내보내 조직 전체에서 Claude Code 사용, 비용 및 도구 활동을 추적합니다. Claude Code는 표준 메트릭 프로토콜을 통해 메트릭을 시계열 데이터로 내보내고, 로그/이벤트 프로토콜을 통해 이벤트를 내보내며, 선택적으로 [추적 프로토콜](#traces-beta)을 통해 분산 추적을 내보냅니다.

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

# 5. 디버깅용: 내보내기 간격 단축, 프로덕션 사용을 위해 재설정
export OTEL_METRIC_EXPORT_INTERVAL=10000  # 10초 (기본값: 60000ms)
export OTEL_LOGS_EXPORT_INTERVAL=5000     # 5초 (기본값: 5000ms)

# 6. Claude Code 실행
claude
```

메트릭을 내보내는 설정을 확인하려면 백엔드에서 `claude_code.session.count` 메트릭을 확인하세요. Claude Code는 세션이 시작될 때 이 메트릭을 내보냅니다. 로그 전용 설정을 확인하려면 프롬프트를 제출하고 `claude_code.user_prompt` 이벤트를 확인하세요.

아무것도 도착하지 않으면 `claude --debug`를 실행하고 디버그 로그를 확인하세요. Claude Code는 구성한 내보내기에서의 실패를 `[3P telemetry]` 오류로 보고합니다. 여기서 3P는 타사를 의미합니다. `[Anthropic telemetry]`로 시작하는 줄은 [Anthropic의 별도 운영 원격 측정](/docs/ko/data-usage#telemetry-services)을 설명하며 설정 문제를 나타내지 않습니다.

전체 구성 옵션은 [OpenTelemetry 사양](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/protocol/exporter.md#configuration-options)을 참조하세요.

<h2 id="administrator-configuration">
  관리자 구성
</h2>

관리자는 [관리 설정 파일](/docs/ko/managed-settings#delivery-mechanisms)을 통해 모든 사용자에 대한 OpenTelemetry 설정을 구성할 수 있습니다. 설정이 적용되는 방식에 대한 자세한 내용은 [설정 우선순위](/docs/ko/settings#settings-precedence)를 참조하세요.

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

Claude Code는 저장소의 `.claude/settings.json` 및 `.claude/settings.local.json`에서 [OpenTelemetry 내보내기 변수](/docs/ko/settings-reference#variables-claude-code-ignores-in-env)를 무시하므로 저장소는 이를 사용하여 원격 측정을 켜거나, 이동 위치를 선택하거나, 콘텐츠를 캡처할 수 없습니다. 관리 설정에서 설정하거나 각 개발자가 자신의 셸 또는 `~/.claude/settings.json`에서 설정하세요. 저장소는 `OTEL_LOGS_EXPORTER`와 같은 내보내기 선택기를 `none`으로 설정하여 신호를 끌 수 있습니다. 단, 관리 설정, `--settings` 파일 또는 Claude Code를 시작하는 환경이 해당 변수를 설정하지 않는 경우에만 가능합니다.

Claude Code는 `OTEL_*` 환경 변수를 Bash 도구, 훅, MCP 서버 및 언어 서버를 포함하여 생성하는 하위 프로세스에 전달하지 않습니다. Bash 도구를 통해 실행하는 OpenTelemetry 계측 애플리케이션은 Claude Code의 내보내기 엔드포인트 또는 헤더를 상속하지 않으므로 해당 애플리케이션이 자신의 원격 측정을 내보내야 하는 경우 명령에서 직접 이러한 변수를 설정합니다.

<h3 id="how-managed-settings-lock-the-otlp-destination">
  관리 설정이 OTLP 대상을 잠그는 방식
</h3>

관리 설정에서 `OTEL_EXPORTER_OTLP_*` 변수를 설정하면 Claude Code는 시작 시 충돌하는 개발자 설정 변수를 제거하고 `claude --debug`로 볼 수 있는 경고를 기록합니다. 제거되는 항목은 설정하는 변수에 따라 달라집니다:

* **엔드포인트**: `OTEL_EXPORTER_OTLP_ENDPOINT`를 설정하면 Claude Code는 개발자가 설정한 모든 신호별 엔드포인트를 제거합니다. 개발자가 한 신호를 다른 수집기로 지정할 수 없으므로 관리 설정에서 신호별 엔드포인트 변수를 설정할 필요가 없습니다.
* **프로토콜**: `OTEL_EXPORTER_OTLP_PROTOCOL`을 설정하면 Claude Code는 개발자가 설정한 모든 신호별 프로토콜을 제거합니다.
* **자격증명**: `OTEL_EXPORTER_OTLP_HEADERS`, `OTEL_EXPORTER_OTLP_CLIENT_KEY` 또는 `OTEL_EXPORTER_OTLP_CLIENT_CERTIFICATE`를 설정하면 Claude Code는 해당 변수의 개발자 설정 신호별 버전과 모든 개발자 설정 엔드포인트 변수(일반 또는 신호별)를 제거합니다. 이러한 자격증명이 관리 설정이 선택하지 않은 수집기에 도달할 수 있기 때문입니다.
* **내보내기 선택기**: `OTEL_METRICS_EXPORTER`, `OTEL_LOGS_EXPORTER` 및 베타 `OTEL_TRACES_EXPORTER`는 일반적인 키별 우선순위를 따릅니다. 개발자의 설정이 여전히 신호를 비활성화하거나 콘솔 내보내기로 전환할 수 있으므로 필요한 경우 관리 설정에서도 선택기를 설정하세요. [관리 소스](/docs/ko/managed-settings#precedence-within-the-managed-tier)에서 `OTEL_LOGS_EXPORTER`는 [원격 측정 단위](/docs/ko/server-managed-settings#per-key-exceptions-across-managed-sources)를 따르는 반면 다른 두 선택기는 키별로 병합됩니다. Claude Code v2.1.223 이상이 필요합니다.
* **베타 추적 엔드포인트**: [상세 베타 추적](#traces-beta)이 활성화되면 Claude Code는 로그 및 추적 내보내기를 통해서가 아니라 `BETA_TRACING_ENDPOINT`로 내보냅니다. 따라서 Claude Code는 다음 관리 설정 중 하나가 신호의 대상을 결정할 때마다 개발자 설정 `BETA_TRACING_ENDPOINT`를 제거합니다:

  * 일반 또는 로그/추적 엔드포인트 또는 자격증명
  * [`otelHeadersHelper`](/docs/ko/settings-reference#otelheadershelper)
  * `none`, `console` 또는 비어있음으로 설정된 로그 또는 추적 내보내기 선택기(신호를 수집기에서 유지하는 값)
  * `CLAUDE_CODE_ENABLE_TELEMETRY` 비활성화

  메트릭 전용 엔드포인트 또는 자격증명은 제거하지 않습니다. v2.1.251 이전에는 개발자 설정 `BETA_TRACING_ENDPOINT`가 관리 설정이 수집기를 고정했을 때도 상세 베타 추적이 내보내는 로그 및 추적을 리디렉션했습니다.

Claude Code는 관리 설정 자체에서 설정한 신호별 변수를 제거하지 않으므로 해당 변수를 설정하여 한 신호를 다른 수집기로 라우팅할 수 있습니다([SIEM 예](#send-events-to-a-siem)에서 수행하는 것처럼). 신호별 자격증명을 설정하면 Claude Code는 해당 신호에 대한 개발자 설정 엔드포인트를 제거합니다.

이 제거 동작은 원격 측정이 전달되는 위치를 변경하며 Claude Code가 수집하는 항목은 변경하지 않습니다.

v2.1.217 이전에는 모든 변수가 키별 설정 우선순위를 독립적으로 따랐으므로 사용자 설정 또는 셸에서 설정한 신호별 엔드포인트가 해당 신호를 관리 수집기에서 리디렉션했습니다.

데스크톱 앱 또는 [자체 호스팅 환경](/docs/ko/self-hosted-environments) 실행기가 Claude Code를 시작하고 제공하는 환경에서 OTLP 엔드포인트를 지정하면 Claude Code는 동일한 방식으로 대상을 고정합니다: 실행기의 원격 측정 변수는 관리 설정과 정확히 동일하게 개발자 설정 변수를 제거합니다. Claude Code는 실행기 자체가 설정한 변수를 제거하지 않습니다. Claude Code v2.1.251 이상이 필요합니다.

<h2 id="configuration-details">
  구성 세부 정보
</h2>

<h3 id="common-configuration-variables">
  일반적인 구성 변수
</h3>

이러한 변수는 모든 배포에 대한 내보내기, 엔드포인트 및 내보내기 동작을 구성합니다.

`OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`와 같은 신호별 엔드포인트 또는 프로토콜 변수를 설정하면 Claude Code는 해당 신호에 대해 일반 변수 대신 이를 사용합니다. `OTEL_EXPORTER_OTLP_METRICS_HEADERS`와 같은 신호별 헤더 변수를 설정하면 Claude Code는 해당 신호에 대해 일반 `OTEL_EXPORTER_OTLP_HEADERS`와 병합합니다.

관리되는 설정이 있는 머신에서는 [관리되는 설정이 OTLP 대상을 잠그는 방법](#how-managed-settings-lock-the-otlp-destination)을 참조하여 Claude Code가 제거하는 항목을 확인합니다.

| 환경 변수                                               | 설명                                                                                                                                                                                                                                                                                                                                                                                             | 예제 값                                                                                     |
| --------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| `CLAUDE_CODE_ENABLE_TELEMETRY`                      | 원격 측정 수집 활성화 (필수)                                                                                                                                                                                                                                                                                                                                                                              | `1`                                                                                      |
| `OTEL_METRICS_EXPORTER`                             | 메트릭 내보내기 유형 (쉼표로 구분). `none`을 사용하여 비활성화                                                                                                                                                                                                                                                                                                                                                        | `console`, `otlp`, `prometheus`, `none`                                                  |
| `OTEL_LOGS_EXPORTER`                                | 로그/이벤트 내보내기 유형 (쉼표로 구분). `none`을 사용하여 비활성화                                                                                                                                                                                                                                                                                                                                                     | `console`, `otlp`, `none`                                                                |
| `OTEL_EXPORTER_OTLP_PROTOCOL`                       | OTLP 내보내기 프로토콜 (모든 신호에 적용). Claude Code는 기본 프로토콜이 없으므로 활성화하는 각 `otlp` 내보내기에 대해 이 또는 신호별 프로토콜 변수를 설정합니다                                                                                                                                                                                                                                                                                         | `grpc`, `http/json`, `http/protobuf`                                                     |
| `OTEL_EXPORTER_OTLP_ENDPOINT`                       | 모든 신호에 대한 OTLP 수집기 엔드포인트                                                                                                                                                                                                                                                                                                                                                                       | `http://localhost:4317`                                                                  |
| `OTEL_EXPORTER_OTLP_METRICS_PROTOCOL`               | 메트릭 프로토콜 (일반 설정 재정의)                                                                                                                                                                                                                                                                                                                                                                           | `grpc`, `http/json`, `http/protobuf`                                                     |
| `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`               | OTLP 메트릭 엔드포인트 (일반 설정 재정의)                                                                                                                                                                                                                                                                                                                                                                     | `http://localhost:4318/v1/metrics`                                                       |
| `OTEL_EXPORTER_OTLP_LOGS_PROTOCOL`                  | 로그 프로토콜 (일반 설정 재정의)                                                                                                                                                                                                                                                                                                                                                                            | `grpc`, `http/json`, `http/protobuf`                                                     |
| `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT`                  | OTLP 로그 엔드포인트 (일반 설정 재정의)                                                                                                                                                                                                                                                                                                                                                                      | `http://localhost:4318/v1/logs`                                                          |
| `OTEL_EXPORTER_OTLP_HEADERS`                        | OTLP용 인증 헤더                                                                                                                                                                                                                                                                                                                                                                                    | `Authorization=Bearer token`                                                             |
| `OTEL_EXPORTER_OTLP_METRICS_HEADERS`                | 메트릭용 인증 헤더 (일반 헤더와 병합)                                                                                                                                                                                                                                                                                                                                                                         | `Authorization=Bearer token`                                                             |
| `OTEL_EXPORTER_OTLP_LOGS_HEADERS`                   | 로그용 인증 헤더 (일반 헤더와 병합)                                                                                                                                                                                                                                                                                                                                                                          | `Authorization=Bearer token`                                                             |
| `OTEL_METRIC_EXPORT_INTERVAL`                       | 내보내기 간격 (밀리초 단위, 기본값: 60000)                                                                                                                                                                                                                                                                                                                                                                   | `5000`, `60000`                                                                          |
| `OTEL_LOGS_EXPORT_INTERVAL`                         | 로그 내보내기 간격 (밀리초 단위, 기본값: 5000)                                                                                                                                                                                                                                                                                                                                                                 | `1000`, `10000`                                                                          |
| `OTEL_LOG_USER_PROMPTS`                             | 사용자 프롬프트 콘텐츠 로깅 활성화 (기본값: 비활성화)                                                                                                                                                                                                                                                                                                                                                                | `1`로 활성화                                                                                 |
| `OTEL_LOG_ASSISTANT_RESPONSES`                      | `assistant_response` 이벤트에서 어시스턴트 응답 텍스트 로깅 활성화 (기본값: 비활성화). 설정되지 않으면 `OTEL_LOG_USER_PROMPTS`의 값으로 폴백됩니다. Claude Code v2.1.193 이상 필요                                                                                                                                                                                                                                                            | `1`로 활성화, `0`으로 수정된 상태 유지                                                                |
| `OTEL_LOG_TOOL_DETAILS`                             | 도구 이벤트 및 추적 스팬 속성에서 도구 매개변수 및 입력 인수 로깅 활성화: Bash 명령, MCP 서버 및 도구 이름, 스킬 이름, 사용자 작성 워크플로우 이름 및 도구 입력. 또한 `user_prompt` 이벤트에서 사용자 정의, 플러그인 및 MCP 명령 이름을 활성화합니다 (기본값: 비활성화). Claude Desktop이 소유한 세션에서 Claude Desktop의 기본 제공 서버의 경우 플래그가 꺼져 있어도 `tool_decision`/`tool_result`에서 `mcp_server_name`/`mcp_tool_name`이 내보내집니다. 예외는 Claude Code v2.1.214 이상 필요                                          | `1`로 활성화                                                                                 |
| `OTEL_LOG_TOOL_CONTENT`                             | [`tool.output` 스팬 이벤트](#tool-output-span-event)에서 도구 콘텐츠 로깅 활성화 (기본값: 비활성화). 스팬 속성은 [자신의 게이트](#new-context-gates) 아래에서 도구 콘텐츠를 전달합니다. [추적](#traces-beta)이 필요합니다. 콘텐츠는 콘텐츠 제한 (기본값 60KB)에서 잘립니다                                                                                                                                                                                                 | `1`로 활성화                                                                                 |
| `OTEL_LOG_MANAGED_SETTINGS`                         | 수정된 관리 설정 및 수정 전 설정의 SHA-256 다이제스트를 [관리 설정 해결됨](#managed-settings-resolved-event) 이벤트에 추가합니다 (기본값: 비활성화). 프로젝트 또는 로컬 설정의 값은 이를 켜지 않습니다. Claude Code v2.1.274 이상 필요                                                                                                                                                                                                                             | `1`로 활성화                                                                                 |
| `OTEL_LOG_RAW_API_BODIES`                           | 전체 Anthropic Messages API 요청 및 응답 JSON을 `api_request_body` / `api_response_body` 로그 이벤트로 내보냅니다 (기본값: 비활성화). 본문에는 전체 대화 기록이 포함됩니다. 이를 활성화하면 `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_TOOL_DETAILS` 및 `OTEL_LOG_TOOL_CONTENT`가 공개할 모든 것에 동의하는 것을 의미합니다                                                                                                                                                 | `1`로 콘텐츠 제한 (기본값 60KB)에서 잘린 인라인 본문, 또는 `file:<dir>`로 디스크의 잘리지 않은 본문과 이벤트의 `body_ref` 포인터 |
| `CLAUDE_CODE_OTEL_CONTENT_MAX_LENGTH`               | 콘텐츠 제한: 모델 응답, 도구 콘텐츠, 시스템 프롬프트 및 원본 API 본문과 같은 콘텐츠 포함 속성의 최대 길이 (UTF-16 코드 단위, 기본값: 61440, 즉 60KB). 기본값은 속성 값을 64KB로 제한하는 백엔드용으로 크기가 조정되었습니다. 백엔드가 더 큰 값을 허용하는 경우에만 증가시키거나 원격 측정 볼륨을 줄이기 위해 감소시킵니다. OpenTelemetry SDK 속성 제한 `OTEL_ATTRIBUTE_VALUE_LENGTH_LIMIT` 또는 해당 로그레코드 및 스팬 변형이 더 낮게 설정된 경우 Claude Code는 `[TRUNCATED ...]` 마커가 SDK 제한 내에 유지되도록 더 작은 값에서 잘립니다. Claude Code v2.1.214 이상 필요 | `262144`                                                                                 |
| `OTEL_EXPORTER_OTLP_METRICS_TEMPORALITY_PREFERENCE` | 메트릭 시간성 선호도 (기본값: `delta`). 백엔드가 누적 시간성을 예상하는 경우 `cumulative`로 설정                                                                                                                                                                                                                                                                                                                              | `delta`, `cumulative`                                                                    |
| `CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`       | 동적 헤더 새로 고침 간격 (기본값: 1740000ms / 29분)                                                                                                                                                                                                                                                                                                                                                          | `900000`                                                                                 |

`http/protobuf` 및 `http/json` 프로토콜의 경우 Claude Code는 각 내보내기 요청을 `Content-Length` 헤더와 함께 전송합니다. v2.1.212 이전에는 v2.1.191 이상의 Claude Code 버전이 청크 전송 인코딩을 사용하여 이러한 요청을 전송했습니다. Azure Monitor 및 기타 선언된 길이가 필요한 엔드포인트는 `411 Length Required` 또는 `400` 오류로 거부했습니다.

<h3 id="mtls-authentication">
  mTLS 인증
</h3>

OTLP 내보내기를 위한 클라이언트 인증서를 구성하는 방법은 해당 신호에 사용되는 OTLP 프로토콜에 따라 다르며, `OTEL_EXPORTER_OTLP_PROTOCOL` 또는 신호별 재정의를 통해 설정됩니다. 동일한 구성이 메트릭, 로그 및 추적에 적용됩니다.

| 프로토콜                         | 클라이언트 인증서 변수                                                                                                                                          | 수집기의 CA 신뢰                       |
| :--------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------- |
| `http/protobuf`, `http/json` | `CLAUDE_CODE_CLIENT_CERT`, `CLAUDE_CODE_CLIENT_KEY` 및 선택적으로 `CLAUDE_CODE_CLIENT_KEY_PASSPHRASE`. [네트워크 구성](/docs/ko/network-config#mtls-authentication) 참조 | `NODE_EXTRA_CA_CERTS`            |
| `grpc`                       | `OTEL_EXPORTER_OTLP_CLIENT_KEY` 및 `OTEL_EXPORTER_OTLP_CLIENT_CERTIFICATE`, 또는 신호별 인증서를 사용하기 위한 `OTEL_EXPORTER_OTLP_METRICS_CLIENT_KEY`와 같은 신호별 변형     | `OTEL_EXPORTER_OTLP_CERTIFICATE` |

`grpc`의 경우 OpenTelemetry SDK는 표준 OTLP 변수를 직접 읽으므로 신호별 메트릭 변수를 설정하는 기존 구성은 계속 작동합니다. 관리되는 설정이 있는 머신에서는 Claude Code가 시작 시 [개발자가 설정한 신호별 자격 증명 및 엔드포인트를 제거](#how-managed-settings-lock-the-otlp-destination)할 수 있습니다.

<h3 id="metrics-cardinality-control">
  메트릭 카디널리티 제어
</h3>

다음 환경 변수는 카디널리티를 관리하기 위해 메트릭에 포함되는 속성을 제어합니다:

| 환경 변수                                      | 설명                                                                                        | 기본값     | 비활성화 예  |
| ------------------------------------------ | ----------------------------------------------------------------------------------------- | ------- | ------- |
| `OTEL_METRICS_INCLUDE_SESSION_ID`          | 메트릭에 session.id 속성 포함                                                                     | `true`  | `false` |
| `OTEL_METRICS_INCLUDE_VERSION`             | 메트릭에 app.version 속성 포함                                                                    | `false` | `true`  |
| `OTEL_METRICS_INCLUDE_ACCOUNT_UUID`        | 메트릭에 user.account\_uuid 및 user.account\_id 속성 포함                                          | `true`  | `false` |
| `OTEL_METRICS_INCLUDE_ENTRYPOINT`          | 메트릭에 app.entrypoint 속성 포함                                                                 | `false` | `true`  |
| `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` | `OTEL_RESOURCE_ATTRIBUTES`의 키를 메트릭 데이터포인트의 속성으로 포함                                        | `true`  | `false` |
| `OTEL_METRICS_INCLUDE_REPOSITORY`          | 메트릭 및 이벤트에 `vcs.*` [저장소 식별 속성](#repository-attributes)을 포함합니다. Claude Code v2.1.269 이상 필요 | `false` | `true`  |

낮은 카디널리티는 일반적으로 더 나은 성능과 낮은 저장소 비용을 의미하지만 분석을 위한 세분화된 데이터는 적습니다.

<h3 id="traces-beta">
  추적 (베타)
</h3>

분산 추적은 각 사용자 프롬프트를 해당 프롬프트가 트리거하는 API 요청 및 도구 실행에 연결하는 스팬을 내보내므로 추적 백엔드에서 전체 요청을 단일 추적으로 볼 수 있습니다.

추적은 기본적으로 꺼져 있습니다. 활성화하려면 `CLAUDE_CODE_ENABLE_TELEMETRY=1` 및 `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1`을 모두 설정한 다음 `OTEL_TRACES_EXPORTER`를 설정하여 스팬을 보낼 위치를 선택합니다. 추적은 엔드포인트, 프로토콜, 헤더 및 [mTLS](#mtls-authentication)에 대해 [일반적인 OTLP 구성](#common-configuration-variables)을 재사용합니다. 관리되는 설정이 있는 머신에서는 Claude Code가 시작 시 [개발자가 설정한 신호별 자격 증명 및 엔드포인트를 제거](#how-managed-settings-lock-the-otlp-destination)할 수 있습니다.

| 환경 변수                                 | 설명                                                    | 예제 값                                 |
| ------------------------------------- | ----------------------------------------------------- | ------------------------------------ |
| `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` | 스팬 추적 활성화 (필수). `ENABLE_ENHANCED_TELEMETRY_BETA`도 허용됨 | `1`                                  |
| `OTEL_TRACES_EXPORTER`                | 추적 내보내기 유형 (쉼표로 구분). `none`을 사용하여 비활성화                | `console`, `otlp`, `none`            |
| `OTEL_EXPORTER_OTLP_TRACES_PROTOCOL`  | 추적 프로토콜 (`OTEL_EXPORTER_OTLP_PROTOCOL` 재정의)           | `grpc`, `http/json`, `http/protobuf` |
| `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`  | OTLP 추적 엔드포인트 (`OTEL_EXPORTER_OTLP_ENDPOINT` 재정의)     | `http://localhost:4318/v1/traces`    |
| `OTEL_EXPORTER_OTLP_TRACES_HEADERS`   | 추적용 인증 헤더 (`OTEL_EXPORTER_OTLP_HEADERS`와 병합)          | `Authorization=Bearer token`         |
| `OTEL_TRACES_EXPORT_INTERVAL`         | 스팬 배치 내보내기 간격 (밀리초 단위, 기본값: 5000)                     | `1000`, `10000`                      |

스팬은 기본적으로 사용자 프롬프트 텍스트, 도구 입력 세부 정보 및 도구 콘텐츠를 수정합니다. `OTEL_LOG_USER_PROMPTS=1`, `OTEL_LOG_TOOL_DETAILS=1` 및 `OTEL_LOG_TOOL_CONTENT=1`을 설정하여 포함합니다.

추적이 활성화되면 Bash 및 PowerShell 하위 프로세스는 활성 도구 실행 스팬의 W3C 추적 컨텍스트를 포함하는 `TRACEPARENT` 환경 변수를 자동으로 상속합니다. 이를 통해 `TRACEPARENT`를 읽는 모든 하위 프로세스가 자신의 스팬을 동일한 추적 아래에 부모로 지정할 수 있으므로 Claude가 실행하는 스크립트 및 명령을 통한 엔드투엔드 분산 추적이 가능합니다.

추적이 활성화되고 Claude Code가 Anthropic API에 직접 연결되어 있으면 각 모델 요청은 `claude_code.llm_request` 스팬의 컨텍스트로 설정된 W3C `traceparent` 헤더를 전달하고, API의 `traceresponse` 헤더는 스팬 링크로 기록됩니다. 이들은 함께 Claude Code의 클라이언트 측 스팬을 모든 호환 중간 계층을 통해 서버 측 추적에 연결합니다. 아웃바운드 HTTP MCP 요청은 동일한 방식으로 `traceparent`를 전달합니다. 헤더는 타사 제공자에게 전송되지 않습니다.

기본적으로 모델 및 HTTP MCP 요청의 `traceparent` 헤더는 `ANTHROPIC_BASE_URL`이 설정되지 않았거나 Anthropic API를 가리킬 때만 전송됩니다. 일부 프록시는 인식되지 않는 헤더를 거부하기 때문입니다. 하위 프로세스 `TRACEPARENT` 변수는 일관성을 위해 동일한 스위치로 제어됩니다. 사용자 정의 `ANTHROPIC_BASE_URL` 프록시를 통해 Claude Code를 실행하고 추적 컨텍스트를 전파하려면 `CLAUDE_CODE_PROPAGATE_TRACEPARENT=1`을 설정합니다.

Agent SDK 및 `-p`로 시작된 비대화형 세션에서 Claude Code는 각 상호 작용 스팬을 시작할 때 자신의 환경에서 `TRACEPARENT` 및 `TRACESTATE`를 읽습니다. 이를 통해 임베딩 프로세스가 활성 W3C 추적 컨텍스트를 하위 프로세스에 전달할 수 있으므로 Claude Code의 스팬이 호출자의 분산 추적의 자식으로 나타납니다. 대화형 세션은 CI 또는 컨테이너 환경의 주변 값을 실수로 상속하는 것을 피하기 위해 인바운드 `TRACEPARENT`를 무시합니다.

인바운드 추적 컨텍스트는 [이벤트](#events)에도 적용됩니다. `TRACEPARENT`가 설정된 Agent SDK 및 `-p` 세션에서 각 OTLP 이벤트 로그 레코드는 추적 내보내기가 구성되지 않은 경우에도 로깅 백엔드가 이벤트를 추적의 나머지 부분과 상관시킬 수 있도록 애플리케이션의 추적에 조인하는 `trace_id` 및 `span_id` 값을 전달합니다.

활성 상호 작용 중에 내보낸 레코드는 권한 프롬프트 콜백이나 시작 중에 버퍼링되고 나중에 내보낸 레코드와 같이 스팬의 비동기 컨텍스트 외부에서 Claude Code가 내보낸 경우에도 상호 작용 스팬의 ID를 전달합니다. 활성 상호 작용 스팬이 없는 상태에서 내보낸 레코드는 인바운드 `TRACEPARENT` ID를 직접 전달합니다. v2.1.214 이전에는 스팬의 비동기 컨텍스트 외부에서 내보낸 레코드가 스팬의 ID 대신 인바운드 `TRACEPARENT` ID를 전달했습니다. v2.1.212 이전에는 활성 스팬 외부에서 내보낸 이벤트 레코드가 `trace_id` 또는 `span_id`를 전달하지 않았습니다.

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

`PreToolUse` 훅이 [도구 호출을 나중으로 연기](/docs/ko/hooks#defer-a-tool-call-for-later)하면 Claude Code는 이를 연기한 턴의 추적 컨텍스트를 저장합니다. 세션을 재개하고 도구가 다시 실행되면 도구의 스팬은 턴의 `claude_code.interaction` 스팬의 자식으로 이전 턴의 추적에 조인됩니다.

<h4 id="span-attributes">
  스팬 속성
</h4>

모든 스팬은 [표준 속성](#standard-attributes)과 이름과 일치하는 `span.type` 속성을 전달합니다. 아래 표는 각 스팬에 설정된 추가 속성을 나열합니다. `llm_request`, `tool.execution` 및 `hook` 스팬은 실패를 기록할 때 OpenTelemetry 상태 `ERROR`를 설정합니다. 다른 스팬은 항상 상태 `UNSET`으로 끝납니다.

**`claude_code.interaction`**

| 속성                        | 설명                                                                                                        | 게이트 대상                  |
| ------------------------- | --------------------------------------------------------------------------------------------------------- | ----------------------- |
| `user_prompt`             | 프롬프트 텍스트. 게이트가 설정되지 않으면 값은 `<REDACTED>`입니다                                                                | `OTEL_LOG_USER_PROMPTS` |
| `user_prompt_length`      | 프롬프트 길이 (문자 단위)                                                                                           |                         |
| `interaction.sequence`    | 상호 작용 1 기반 카운터 ([`event.sequence`](#event-correlation-attributes)에 설명된 대로 세션당이 아닌 Claude Code 프로세스당 계산됨)  |                         |
| `parent.source`           | 스팬이 추적 부모를 얻은 방법: 인바운드 `TRACEPARENT`에서 부모가 되었을 때 `env`, 자신의 추적을 시작했을 때 `none`. Claude Code v2.1.268 이상 필요 |                         |
| `interaction.duration_ms` | 턴의 벽시계 지속 시간                                                                                              |                         |

**`claude_code.llm_request`**

| 속성                               | 설명                                                                                                                                                                                                  | 게이트 대상                         |
| -------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------ |
| `model`                          | 모델 식별자                                                                                                                                                                                              |                                |
| `gen_ai.system`                  | 항상 `anthropic`. OpenTelemetry GenAI 의미론적 규칙                                                                                                                                                         |                                |
| `gen_ai.request.model`           | `model`과 동일한 값. OpenTelemetry GenAI 의미론적 규칙                                                                                                                                                         |                                |
| `query_source`                   | 요청을 발급한 하위 시스템 (예: `repl_main_thread` 또는 하위 에이전트 이름)                                                                                                                                                | `ENABLE_BETA_TRACING_DETAILED` |
| `query_source_safe`              | `query_source`의 제한된 형식 (상세 베타 추적이 활성화되었는지 여부와 관계없이 내보내짐). `repl_main_thread` 또는 `agent.builtin.general-purpose`와 같은 값. `:`는 `.`이 되고 사용자 명명 에이전트는 `agent.custom`으로 나타납니다. Claude Code v2.1.268 이상 필요 |                                |
| `agent_id`                       | 요청을 발급한 하위 에이전트 또는 팀원의 식별자. 주 세션에는 없음                                                                                                                                                               |                                |
| `parent_agent_id`                | 이 에이전트를 생성한 에이전트의 식별자. 주 세션 및 직접 생성된 에이전트에는 없음                                                                                                                                                      |                                |
| `workflow.run_id`                | 이 에이전트를 생성한 [Workflow](/docs/ko/workflows) 도구 실행의 실행 식별자 (접두사 `wf_`). 워크플로우에 의해 생성되지 않은 에이전트의 경우 없음                                                                                                      |                                |
| `workflow.name`                  | 이 에이전트를 생성한 워크플로우의 이름. 사용자 작성 이름은 게이트가 설정되지 않으면 `custom`으로 대체됩니다                                                                                                                                    | `OTEL_LOG_TOOL_DETAILS`        |
| `speed`                          | `fast` 또는 `normal`                                                                                                                                                                                  |                                |
| `effort`                         | [노력 수준](/docs/ko/model-config#adjust-effort-level) (요청에 적용됨): `low`, `medium`, `high`, `xhigh` 또는 `max`. Claude Code가 노력 수준을 보내지 않을 때 (예: 노력을 지원하지 않는 모델)는 없음. Claude Code v2.1.274 이상 필요                |                                |
| `llm_request.context`            | 부모 스팬에 따라 `interaction`, `tool` 또는 `standalone`                                                                                                                                                     |                                |
| `duration_ms`                    | 재시도를 포함한 벽시계 지속 시간                                                                                                                                                                                  |                                |
| `ttft_ms`                        | 첫 번째 토큰까지의 시간 (밀리초)                                                                                                                                                                                 |                                |
| `first_content_ms`               | 요청 시작부터 성공한 시도의 첫 번째 콘텐츠 블록까지의 시간 (밀리초). 스트리밍되지 않은 경로로 폴백된 요청에는 없음. Claude Code v2.1.268 이상 필요                                                                                                      |                                |
| `input_tokens`                   | API 사용 블록의 입력 토큰 수                                                                                                                                                                                  |                                |
| `output_tokens`                  | 출력 토큰 수                                                                                                                                                                                             |                                |
| `cache_read_tokens`              | 프롬프트 캐시에서 읽은 토큰                                                                                                                                                                                     |                                |
| `cache_creation_tokens`          | 프롬프트 캐시에 기록된 토큰                                                                                                                                                                                     |                                |
| `request_id`                     | `request-id` 응답 헤더의 Anthropic API 요청 ID                                                                                                                                                             |                                |
| `gen_ai.response.id`             | `request_id`와 동일한 값. OpenTelemetry GenAI 의미론적 규칙                                                                                                                                                    |                                |
| `client_request_id`              | 최종 시도의 클라이언트 생성 `x-client-request-id`                                                                                                                                                               |                                |
| `attempt`                        | 이 요청에 대해 수행된 총 시도                                                                                                                                                                                   |                                |
| `success`                        | `true` 또는 `false`                                                                                                                                                                                   |                                |
| `status_code`                    | 요청이 실패했을 때 HTTP 상태 코드                                                                                                                                                                               |                                |
| `error`                          | 요청이 실패했을 때 오류 메시지                                                                                                                                                                                   |                                |
| `error_class`                    | 요청이 실패했을 때 짧은 오류 클래스 토큰 (예: `api_timeout` 또는 `server_overload`). Claude Code v2.1.268 이상 필요                                                                                                         |                                |
| `response.has_tool_call`         | 응답에 도구 사용 블록이 포함되었을 때 `true`                                                                                                                                                                        |                                |
| `stop_reason`                    | API 응답 `stop_reason` (예: `end_turn`, `tool_use`, `max_tokens`, `stop_sequence`, `pause_turn` 또는 `refusal`)                                                                                          |                                |
| `gen_ai.response.finish_reasons` | `stop_reason`과 동일한 값 (문자열 배열로 래핑됨). OpenTelemetry GenAI 의미론적 규칙                                                                                                                                     |                                |

각 재시도 시도는 `attempt` 및 `client_request_id` 속성이 있는 `gen_ai.request.attempt` 스팬 이벤트로도 기록됩니다.

**`claude_code.tool`**

| 속성                    | 설명                                                                                                                                                                                  | 게이트 대상                  |
| --------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `tool_name`           | 도구 이름                                                                                                                                                                               |                         |
| `tool_name_safe`      | 사용자 선택 이름을 포함하지 않는 `tool_name`의 형식. 기본 제공 도구 이름은 그대로 전달됩니다. MCP 도구 이름은 `mcp_other`로 나타나며, `playwright` 도구 `browser_*`와 같이 고정된 형태와 일치하는 도구 이름은 그대로 전달됩니다. Claude Code v2.1.268 이상 필요 |                         |
| `bash_command_class`  | Bash 도구의 경우: 고정 목록의 명령 첫 프로그램 범주 (예: `vcs` 또는 `package_manager`). 목록 외의 프로그램의 경우 `other`, 줄을 구문 분석할 수 없을 때 `unparsed`. Claude Code v2.1.268 이상 필요                                   |                         |
| `bash_argv0`          | Bash 도구의 경우: 동일한 고정 목록에 있을 때 명령의 첫 프로그램 (예: `git` 또는 `npm`). 목록 외의 모든 프로그램의 경우 `other`. Claude Code v2.1.268 이상 필요                                                                  |                         |
| `duration_ms`         | 권한 대기 및 실행을 포함한 벽시계 지속 시간                                                                                                                                                           |                         |
| `result_tokens`       | 도구 결과의 대략적인 토큰 크기                                                                                                                                                                   |                         |
| `agent_id`            | 도구를 실행한 하위 에이전트 또는 팀원의 식별자. 주 세션에는 없음                                                                                                                                               |                         |
| `parent_agent_id`     | 이 에이전트를 생성한 에이전트의 식별자. 주 세션 및 직접 생성된 에이전트에는 없음                                                                                                                                      |                         |
| `workflow.run_id`     | 이 에이전트를 생성한 Workflow 도구 실행의 실행 식별자 (접두사 `wf_`). 워크플로우에 의해 생성되지 않은 에이전트의 경우 없음                                                                                                       |                         |
| `workflow.name`       | 이 에이전트를 생성한 워크플로우의 이름. 사용자 작성 이름은 게이트가 설정되지 않으면 `custom`으로 대체됩니다                                                                                                                    | `OTEL_LOG_TOOL_DETAILS` |
| `tool_use_id`         | 이 호출에 대한 모델의 `tool_use` 블록 ID. [tool\_result](#tool-result-event) 및 [tool\_decision](#tool-decision-event) 이벤트의 `tool_use_id`와 훅 페이로드의 `tool_use_id`와 일치하므로 스팬을 해당 레코드에 조인할 수 있습니다  |                         |
| `gen_ai.tool.call.id` | `tool_use_id`와 동일한 값. OpenTelemetry GenAI 의미론적 규칙                                                                                                                                   |                         |
| `file_path`           | Read, Edit 및 Write 도구의 대상 파일 경로                                                                                                                                                     | `OTEL_LOG_TOOL_DETAILS` |
| `full_command`        | Bash 도구의 명령 문자열                                                                                                                                                                     | `OTEL_LOG_TOOL_DETAILS` |
| `skill_name`          | Skill 도구의 스킬 이름                                                                                                                                                                     | `OTEL_LOG_TOOL_DETAILS` |
| `subagent_type`       | Agent 도구 또는 레거시 Task 도구의 하위 에이전트 유형                                                                                                                                                 | `OTEL_LOG_TOOL_DETAILS` |

<span id="tool-output-span-event" />**`tool.output` span event on `claude_code.tool`**

`OTEL_LOG_TOOL_CONTENT=1`을 설정하면 Read 및 Bash 호출은 `claude_code.tool` 스팬에 `tool.output` 스팬 이벤트를 기록할 수 있습니다. Edit 및 Write 호출은 `OTEL_LOG_TOOL_DETAILS=1`도 설정할 때만 기록합니다. 해당 변수는 해당 두 도구로 범위가 지정되지 않으므로 구성 테이블의 [행](#common-configuration-variables)을 확인하여 다른 곳에서 추가하는 인수를 확인합니다.

Claude Code는 도구 호출의 성공적인 반환에서 이 이벤트를 작성하므로 오류를 발생시키는 호출은 도구에 관계없이 아무것도 기록하지 않습니다. 반환하는 호출 중에서 다음에 대해 `tool.output` 이벤트를 기록하지 않습니다:

* Read, Edit, Write 및 Bash 이외의 도구 (MCP 도구 및 WebFetch 포함)에 대한 호출
* 이미지, PDF 또는 콘텐츠가 변경되지 않은 파일의 재읽기와 같이 파일 텍스트 이외의 것을 반환하는 Read
* `OTEL_LOG_TOOL_DETAILS=1`도 설정하지 않으면 Edit 또는 Write 호출

이벤트는 이러한 속성을 전달하며, 각각은 콘텐츠 제한 (기본값 60KB)에서 잘립니다. `Gated by`는 속성이 `OTEL_LOG_TOOL_CONTENT=1` 위에 필요한 변수를 이름 지으며, Edit 및 Write의 경우 해당 변수는 속성이 아닌 이벤트 자체를 게이트합니다.

| 속성             | 설명                                                 | 게이트 대상                                 |
| -------------- | -------------------------------------------------- | -------------------------------------- |
| `content`      | Read 도구가 반환한 텍스트 또는 Write 호출이 작성하도록 요청한 텍스트        | `OTEL_LOG_TOOL_DETAILS` (Write 도구의 경우) |
| `output`       | Bash 명령의 결합된 출력 (stderr가 stdout에 인터리브됨)            |                                        |
| `diff`         | Edit 도구가 적용한 구조화된 패치                               | `OTEL_LOG_TOOL_DETAILS`                |
| `file_path`    | Read, Edit 및 Write 도구의 대상 파일 경로 (동일한 이름의 스팬 속성 반복) | `OTEL_LOG_TOOL_DETAILS`                |
| `bash_command` | Bash 도구의 명령 문자열                                    | `OTEL_LOG_TOOL_DETAILS`                |

부모 스팬의 `tool_name` 속성은 이벤트가 어느 도구에서 왔는지 알려줍니다. 콘텐츠 제한에서 잘린 속성에는 `<attribute>_truncated` 및 `<attribute>_original_length`가 함께 제공됩니다.

**`claude_code.tool.blocked_on_user`**

| 속성            | 설명                                                      | 게이트 대상 |
| ------------- | ------------------------------------------------------- | ------ |
| `duration_ms` | 권한 결정 대기 시간                                             |        |
| `decision`    | `accept` 또는 `reject`                                    |        |
| `source`      | 결정 출처 ([Tool decision event](#tool-decision-event)와 일치) |        |

**`claude_code.tool.execution`**

| 속성                    | 설명                                                                                                                                                       | 게이트 대상                  |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| `duration_ms`         | 도구 본문 실행 시간                                                                                                                                              |                         |
| `tool_use_id`         | 부모 `claude_code.tool` 스팬과 동일한 값                                                                                                                          |                         |
| `gen_ai.tool.call.id` | `tool_use_id`와 동일한 값. OpenTelemetry GenAI 의미론적 규칙                                                                                                        |                         |
| `success`             | `true` 또는 `false`                                                                                                                                        |                         |
| `error`               | 실행이 실패했을 때 오류 범주 문자열 (예: `Error:ENOENT` 또는 `ShellError`). 게이트가 설정되면 전체 오류 메시지를 포함합니다                                                                     | `OTEL_LOG_TOOL_DETAILS` |
| `error_class`         | 오류 범주를 식별자 형식으로 표현한 것 (문자, 숫자 및 언더스코어 외의 문자는 `_`로 대체됨). 예: `Error_ENOENT` 또는 `ShellError`. `error`가 전체 메시지를 포함할 때도 범주를 전달합니다. Claude Code v2.1.268 이상 필요 |                         |

**`claude_code.hook`**

이 스팬은 상세 베타 추적이 활성화되어 있을 때만 나타나며, 이는 `ENABLE_BETA_TRACING_DETAILED=1` 및 `BETA_TRACING_ENDPOINT`가 필요합니다. 이 쌍은 또한 [로그 및 추적이 이동하는 위치를 변경](/docs/ko/env-vars#variables)합니다. 셸, 사용자 설정 또는 관리되는 설정에서 쌍을 설정합니다. 두 변수 모두 [프로젝트 및 로컬 설정](/docs/ko/settings-reference#variables-claude-code-ignores-in-env)에서 무시됩니다. `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA`만으로는 생성되지 않습니다.

대화형 CLI 세션에서 상세 베타 추적은 또한 조직이 이 기능에 대해 허용 목록에 있어야 합니다. Agent SDK 및 비대화형 `-p` 세션은 허용 목록이 필요하지 않습니다.

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

<span id="new-context-gates" />

<Note>
  `new_context`, `system_prompt_preview`, `user_system_prompt`, `tool_input` 및 `response.model_output`과 같은 추가 콘텐츠 포함 속성은 상세 베타 추적이 활성화되어 있을 때만 내보내집니다. 이들은 안정적인 스팬 스키마의 일부가 아닙니다.

  `new_context`의 게이트는 해당 스팬을 전달하는 것에 따라 다르며, 각 복사본은 콘텐츠 제한 (기본값 60KB)에서 잘립니다. `claude_code.tool` 스팬에서 도구에 관계없이 해당 도구 호출의 결과를 전달하며 `OTEL_LOG_TOOL_CONTENT=1`이 필요합니다. `claude_code.interaction` 스팬에서 사용자 프롬프트를 전달하고, `claude_code.llm_request` 스팬에서 해당 요청의 새 사용자 메시지 및 도구 결과를 전달합니다. 둘 다 `OTEL_LOG_USER_PROMPTS=1`이 필요합니다.

  `user_system_prompt`는 추가로 `OTEL_LOG_USER_PROMPTS=1`이 필요합니다. 이는 `systemPrompt` SDK 옵션 또는 `--system-prompt` 및 `--append-system-prompt` 플래그를 통해 제공하는 시스템 프롬프트 텍스트만 포함하며 (콘텐츠 제한 (기본값 60KB)에서 잘림), 요청당이 아닌 세션당 한 번 내보내집니다.
</Note>

<h3 id="dynamic-headers">
  동적 헤더
</h3>

동적 인증이 필요한 엔터프라이즈 환경의 경우 스크립트를 구성하여 헤더를 동적으로 생성할 수 있습니다. 동적 헤더는 `http/protobuf` 및 `http/json` 프로토콜에만 적용됩니다. `grpc` 프로토콜의 경우 Claude Code는 정적 헤더 변수 `OTEL_EXPORTER_OTLP_HEADERS` 및 신호별 변형만 사용합니다.

<h4 id="settings-configuration">
  설정 구성
</h4>

`.claude/settings.json`에 추가합니다 (경로를 자신의 스크립트로 바꿉니다):

```json theme={null}
{
  "otelHeadersHelper": "/path/to/generate-otel-headers.sh"
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

도우미가 실패하거나 이러한 요구 사항을 충족하지 않는 출력을 인쇄하면 내보내기가 실패하고 도우미가 다시 작동할 때까지 세션에서 원격 측정 백엔드가 아무것도 받지 못합니다. Claude Code는 다음에서 오류를 보고합니다:

* 대화형 세션의 경고 알림 (도우미가 처음 실패할 때 세션당 한 번 표시되는 [`otelHeadersHelper failed; telemetry is not being exported`](/docs/ko/errors#otelheadershelper-failed))
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

Claude Code는 이러한 값을 모든 메트릭 데이터포인트 및 이벤트 레코드의 속성으로 첨부하고, OTLP 리소스 블록에서도 전송합니다. 대부분의 메트릭 백엔드는 데이터포인트 속성을 쿼리 가능한 레이블로 노출하므로 사용자 정의 키로 직접 메트릭을 그룹화하고 필터링할 수 있습니다. `vcs.*` [저장소 속성](#repository-attributes)을 제외하고 사용자 정의 키는 `user.id` 또는 `session.id`와 같은 [표준 속성](#standard-attributes)을 재정의하지 않습니다. 키가 충돌하면 Claude Code는 기본 제공 값을 유지합니다.

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

`claude`를 실행하기 전에 이러한 환경 변수를 설정합니다. 각 시나리오는 완전한 구성을 보여주며, 각 변수는 [일반적인 구성 변수](#common-configuration-variables)에서 설명됩니다. 구성이 적용되었는지 확인하려면 세션을 시작한 후 백엔드에서 `claude_code.session.count` 메트릭을 확인합니다. [빠른 시작](#quick-start)은 로그 전용 확인 및 아무것도 도착하지 않을 때 확인할 사항을 다룹니다.

콘솔 디버깅 (1초 간격):

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console
export OTEL_METRIC_EXPORT_INTERVAL=1000
```

OTLP over gRPC:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

Prometheus ([http://localhost:9464/metrics에서](http://localhost:9464/metrics에서) 스크래핑):

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=prometheus
```

[자체 호스팅 환경](/docs/ko/self-hosted-environments-reference#pass-through-session-child-metrics)에서 세션은 러너의 기본 용량 1에서만 포트 9464를 바인딩합니다. 더 높은 용량에서 러너는 대신 자신의 `/metrics` 엔드포인트에서 세션 카운터 및 게이지를 다시 노출합니다.

여러 내보내기로 메트릭을 보내려면:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=console,otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=http/json
```

메트릭 및 로그를 다양한 엔드포인트 또는 백엔드로 보내려면:

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_LOGS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_METRICS_PROTOCOL=http/protobuf
export OTEL_EXPORTER_OTLP_METRICS_ENDPOINT=http://metrics.example.com:4318
export OTEL_EXPORTER_OTLP_LOGS_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_LOGS_ENDPOINT=http://logs.example.com:4317
```

메트릭만 내보내려면 (이벤트/로그 없음):

```bash theme={null}
export CLAUDE_CODE_ENABLE_TELEMETRY=1
export OTEL_METRICS_EXPORTER=otlp
export OTEL_EXPORTER_OTLP_PROTOCOL=grpc
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
```

이벤트/로그만 내보내려면 (메트릭 없음):

```bash theme={null}
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

모든 메트릭과 이벤트는 다음과 같은 표준 속성을 공유합니다:

| 속성                                                                                      | 설명                                                                                                                            | 제어 대상                                                                      |
| --------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| `session.id`                                                                            | 고유한 세션 식별자                                                                                                                    | `OTEL_METRICS_INCLUDE_SESSION_ID` (기본값: true)                              |
| `app.version`                                                                           | 현재 Claude Code 버전                                                                                                             | `OTEL_METRICS_INCLUDE_VERSION` (기본값: false)                                |
| `app.entrypoint`                                                                        | 세션이 시작된 방식(예: `cli`, `sdk-cli`, `sdk-ts`, `sdk-py`, 또는 `claude-vscode`)                                                       | `OTEL_METRICS_INCLUDE_ENTRYPOINT` (기본값: false)                             |
| `organization.id`                                                                       | 조직 UUID (인증된 경우)                                                                                                              | 사용 가능할 때 항상 포함됨                                                            |
| `user.account_uuid`                                                                     | 계정 UUID (인증된 경우)                                                                                                              | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (기본값: true)                            |
| `user.account_id`                                                                       | Anthropic 관리자 API와 일치하는 태그 형식의 계정 ID (인증된 경우)(예: `user_01BWBeN28...`)                                                         | `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` (기본값: true)                            |
| `user.id`                                                                               | 첫 실행 시 생성되고 `~/.claude.json`에 유지되는 무작위 익명 식별자입니다. 개인 정보를 포함하지 않으며 Claude 계정에서 파생되지 않습니다. 파일을 삭제하면 다음 실행 시 새로운 관련 없는 값이 생성됩니다. | 항상 포함됨                                                                     |
| `user.email`                                                                            | 사용자 이메일 주소(로그인 시 또는 [클라우드 세션](/docs/ko/claude-code-on-the-web)에서 세션 자체의 자격 증명에서)                                                   | 사용 가능할 때 항상 포함됨                                                            |
| `terminal.type`                                                                         | 터미널 유형(예: `iTerm.app`, `vscode`, `cursor`, 또는 `tmux`)                                                                         | 감지될 때 항상 포함됨                                                               |
| `OTEL_RESOURCE_ATTRIBUTES`의 키                                                           | 설정한 사용자 정의 속성(예: `department` 또는 `team.id`). [다중 팀 조직 지원](#multi-team-organization-support) 참조                                | `OTEL_METRICS_INCLUDE_RESOURCE_ATTRIBUTES` (기본값: true)                     |
| `vcs.repository.url.full`, `vcs.owner.name`, `vcs.repository.name`, `vcs.provider.name` | 세션 저장소의 ID(해당 `origin` 원격에서 파생됨). [저장소 속성](#repository-attributes) 참조                                                         | `OTEL_METRICS_INCLUDE_REPOSITORY` (기본값: false). Claude Code v2.1.269 이상 필요 |

Claude Code가 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway)에 로그인되어 있으면 CLI는 게이트웨이 세션의 인증된 ID로 내보내기를 스탬프합니다: `user.id`는 익명 설치 식별자가 아닌 IdP 주체이고, `user.email`은 로그인한 이메일이며, `user.groups`는 쉼표로 구분된 문자열로 IdP 그룹 멤버십을 전달합니다. 각 내보내기는 또한 `identity.source: gateway-oidc`를 전달합니다. 게이트웨이 ID가 마지막에 적용되므로 `OTEL_RESOURCE_ATTRIBUTES`를 통해 설정된 `user.*` 및 `identity.*` 키는 게이트웨이 세션에서 무시됩니다.

이벤트는 추가로 다음 속성을 포함합니다. 이들은 무한한 카디널리티를 야기할 수 있으므로 메트릭에는 절대 첨부되지 않습니다:

* `prompt.id`: 사용자 프롬프트를 다음 프롬프트까지의 모든 후속 이벤트와 연관시키는 UUID입니다. [이벤트 상관 속성](#event-correlation-attributes) 참조.
* `workspace.host_paths`: 데스크톱 앱에서 선택한 호스트 작업 공간 디렉토리(문자열 배열)
* `workflow.run_id`: [Workflow](/docs/ko/workflows) 도구 실행에 속하는 에이전트가 내보낸 API 및 도구 이벤트의 실행 식별자(접두사 `wf_`). 하나의 `workflow.run_id`로 이벤트를 필터링하면 해당 실행의 API 요청 및 도구 결과를 재구성합니다. 식별자는 워크플로우 스크립트가 생성하는 에이전트와 그 에이전트가 차례로 생성하는 모든 에이전트(예: 스킬 호출)를 포함합니다. Workflow 도구 결과에서 보고된 실행 식별자와 일치합니다. 다른 모든 이벤트에는 없습니다. Claude Code v2.1.202 이상 필요
* `workflow.name`: 워크플로우의 이름(스크립트의 `meta.name`), `workflow.run_id`와 함께 내보냅니다. 기본 제공 워크플로우 이름은 실행이 수정되지 않은 기본 제공 스크립트를 실행할 때 그대로 나타납니다. 기본 제공 스크립트의 편집된 복사본을 포함한 사용자 작성 이름은 `OTEL_LOG_TOOL_DETAILS=1`이 설정되지 않은 경우 `custom`으로 대체됩니다. Claude Code v2.1.202 이상 필요

<h4 id="repository-attributes">
  저장소 속성
</h4>

`OTEL_METRICS_INCLUDE_REPOSITORY=true`를 설정하여 메트릭 및 이벤트에 세션의 저장소 ID를 태그하면 공유 수집기가 저장소별 사용량을 속성화할 수 있습니다. Claude Code v2.1.269 이상 필요합니다.

Claude Code는 저장소의 `origin` 원격에서 이러한 속성을 세션당 한 번 파생합니다. 한 저장소의 HTTPS 및 SSH 원격은 동일한 값을 생성합니다:

| 속성                        | 값                                                                                                  |
| ------------------------- | -------------------------------------------------------------------------------------------------- |
| `vcs.repository.url.full` | 저장소의 브라우저 URL(`.git` 제외)(예: `https://github.com/example-org/example-repo`)                         |
| `vcs.owner.name`          | 소유자 또는 그룹 경로(예: `example-org`); 원격 경로에 단일 세그먼트가 있을 때 생략됨                                           |
| `vcs.repository.name`     | 기본 저장소 이름(예: `example-repo`)                                                                       |
| `vcs.provider.name`       | Claude Code가 원격의 호스트 또는 URL 형태를 `github`, `gitlab`, `bitbucket`, 또는 `gitea` 중 하나로 인식할 때; 그 외에는 생략됨 |

값은 소문자로 변환되며, 원격 URL의 자격 증명, 쿼리 문자열 및 조각은 절대 나타나지 않습니다. 세션에 `origin` 원격이 없을 때, 원격이 URL 형태가 아닐 때, 또는 유일한 포함 저장소가 홈 디렉토리일 때 속성이 생략됩니다.

[`OTEL_RESOURCE_ATTRIBUTES`](#multi-team-organization-support)에서 선언한 `vcs.*` 키는 해당 키의 파생된 값을 대체합니다. `vcs.repository.url.full`을 선언하면 Claude Code는 절대 원격을 읽지 않고 선언한 키만 보고합니다.

속성은 자신의 내보내기로만 흐릅니다; Anthropic의 원격 측정은 모든 `vcs.*` 키를 삭제합니다.

<h3 id="metrics">
  메트릭
</h3>

Claude Code는 다음 메트릭을 내보냅니다. Unit 열은 각 메트릭에 첨부된 OpenTelemetry 단위 문자열을 보여줍니다; 카운트 메트릭은 없습니다.

| 메트릭 이름                                | 설명                 | 단위     |
| ------------------------------------- | ------------------ | ------ |
| `claude_code.session.count`           | 시작된 CLI 세션 수       | 없음     |
| `claude_code.lines_of_code.count`     | 수정된 코드 라인 수        | 없음     |
| `claude_code.pull_request.count`      | 생성된 풀 요청 수         | 없음     |
| `claude_code.commit.count`            | 생성된 git 커밋 수       | 없음     |
| `claude_code.cost.usage`              | Claude Code 세션의 비용 | USD    |
| `claude_code.token.usage`             | 사용된 토큰 수           | tokens |
| `claude_code.code_edit_tool.decision` | 코드 편집 도구 권한 결정 수   | 없음     |
| `claude_code.active_time.total`       | 총 활성 시간            | s      |

`prometheus`가 `OTEL_METRICS_EXPORTER`에 나열된 유일한 내보내기일 때 Claude Code는 스크래이프가 유효한 Prometheus 텍스트 형식으로 유지되도록 내보낸 메트릭에서 `USD`, `tokens`, 및 `s` 단위를 생략합니다. 메트릭 이름은 변경되지 않으며, `otlp,prometheus`와 같이 내보내기를 결합하는 구성은 단위를 유지합니다. v2.1.216 이전에는 Prometheus 스크래이프에 일부 스크래퍼가 거부한 OpenMetrics 전용 `# UNIT` 라인이 포함되었습니다.

<h3 id="metric-details">
  메트릭 세부 정보
</h3>

각 메트릭은 위에 나열된 표준 속성을 포함합니다. 추가 컨텍스트별 속성이 있는 메트릭은 아래에 표시됩니다.

<h4 id="session-counter">
  세션 카운터
</h4>

각 세션의 시작 시 증가합니다.

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `start_type`: 세션이 시작된 방식. `"fresh"`, `"resume"`, `"continue"`, 또는 `"agents_view"` 중 하나입니다. `"agents_view"` 값은 `claude agents` 대시보드 프로세스(대화형 세션이 아닌 사용자 시작 로컬 UI)를 식별합니다. 대시보드에서 UI 프로세스 시작을 대화형 세션과 분리하려면 이 값을 필터링합니다.

<h4 id="lines-of-code-counter">
  코드 라인 카운터
</h4>

코드가 추가되거나 제거될 때 증가합니다.

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `type`: (`"added"`, `"removed"`)
* `model`: 변경을 수행한 모델의 모델 식별자(예: "claude-sonnet-5")

<h4 id="pull-request-counter">
  풀 요청 카운터
</h4>

Claude Code가 셸 명령 또는 MCP 도구를 통해 풀 요청 또는 병합 요청을 생성할 때 증가합니다.

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
* `model`: 모델 식별자(예: "claude-sonnet-5")
* `query_source`: 요청을 발급한 하위 시스템의 범주. `"main"`, `"subagent"`, 또는 `"auxiliary"` 중 하나입니다.
* `speed`: 요청이 빠른 모드를 사용했을 때 `"fast"`. 그 외에는 없습니다.
* `effort`: 요청에 적용된 [노력 수준](/docs/ko/model-config#adjust-effort-level): `"low"`, `"medium"`, `"high"`, `"xhigh"`, 또는 `"max"`. Claude Code가 노력 수준을 보내지 않을 때(예: 노력을 지원하지 않는 모델)는 없습니다.
* `agent.name`: 요청을 발급한 하위 에이전트 유형. 기본 제공 에이전트 이름과 공식 마켓플레이스 플러그인의 에이전트는 그대로 나타납니다. 다른 사용자 정의 에이전트 이름은 `OTEL_LOG_TOOL_DETAILS=1`이 설정되지 않은 경우 `"custom"`으로 대체됩니다. 요청이 명명된 하위 에이전트 유형에 의해 발급되지 않았을 때는 없습니다.
* `skill.name`: 요청에 대해 활성화된 스킬(Skill 도구, `/` 명령, 또는 생성된 하위 에이전트에 의해 상속됨으로 설정됨). 기본 제공, 번들, 사용자 정의, 및 공식 마켓플레이스 플러그인 스킬 이름은 그대로 나타납니다. 타사 플러그인 스킬 이름은 `OTEL_LOG_TOOL_DETAILS=1`이 설정되지 않은 경우 `"third-party"`로 대체됩니다. 활성 스킬이 없을 때는 없습니다.
* `plugin.name`: 활성 스킬 또는 하위 에이전트를 제공하는 플러그인의 소유자. 공식 마켓플레이스 플러그인 이름은 그대로 나타납니다. 타사 플러그인 이름은 `OTEL_LOG_TOOL_DETAILS=1`이 설정되지 않은 경우 `"third-party"`로 대체됩니다. 스킬과 하위 에이전트 모두 소유 플러그인이 없을 때는 없습니다.
* `marketplace.name`: 소유 플러그인이 설치된 마켓플레이스. 공식 마켓플레이스 플러그인에만 내보냅니다. 그 외에는 없습니다.
* `mcp_server.name`: 이 요청이 소비한 도구 결과의 MCP 서버. 기본 제공, claude.ai 프록시, 및 공식 레지스트리 서버 이름은 그대로 나타납니다. 사용자 구성 서버 이름은 `OTEL_LOG_TOOL_DETAILS=1`이 설정되지 않은 경우 `"custom"`으로 대체됩니다. 요청이 MCP 도구 결과를 소비하지 않았을 때는 없습니다. v2.1.222 이전에는 Claude Code가 MCP 도구 호출 후 모든 요청에 이 속성을 설정했으며, 도구 결과를 소비한 요청에만 설정하지 않았으므로 이를 집계하는 대시보드는 업그레이드 후 단계 감소를 보여줍니다.
* `mcp_tool.name`: 이 요청이 소비한 도구 결과의 MCP 도구(삭제 및 버전 동작은 `mcp_server.name`과 동일). 요청이 MCP 도구 결과를 소비하지 않았을 때는 없습니다.

<h4 id="token-counter">
  토큰 카운터
</h4>

각 API 요청 후 증가합니다.

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `type`: (`"input"`, `"output"`, `"cacheRead"`, `"cacheCreation"`)
* `model`: 모델 식별자(예: "claude-sonnet-5")
* `query_source`: 요청을 발급한 하위 시스템의 범주. `"main"`, `"subagent"`, 또는 `"auxiliary"` 중 하나입니다.
* `speed`: 요청이 빠른 모드를 사용했을 때 `"fast"`. 그 외에는 없습니다.
* `effort`: 요청에 적용된 [노력 수준](/docs/ko/model-config#adjust-effort-level). 세부 정보는 [비용 카운터](#cost-counter)를 참조하세요.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: 요청에 대한 스킬, 플러그인, 에이전트, 및 MCP 속성. 정의 및 삭제 동작은 [비용 카운터](#cost-counter)를 참조하세요.

<h4 id="code-edit-tool-decision-counter">
  코드 편집 도구 결정 카운터
</h4>

사용자가 Edit, Write, 또는 NotebookEdit 도구 사용을 수락하거나 거부할 때 증가합니다.

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `tool_name`: 도구 이름 (`"Edit"`, `"Write"`, `"NotebookEdit"`)
* `decision`: 사용자 결정 (`"accept"`, `"reject"`)
* `source`: 결정이 나온 위치. `"config"`, `"hook"`, `"user_permanent"`, `"user_temporary"`, `"user_abort"`, 또는 `"user_reject"` 중 하나입니다. 각 값의 의미는 [도구 결정 이벤트](#tool-decision-event)를 참조하세요.
* `language`: 편집된 파일의 프로그래밍 언어(예: `"TypeScript"`, `"Python"`, `"JavaScript"`, 또는 `"Markdown"`). 인식되지 않은 파일 확장자의 경우 `"unknown"`을 반환합니다.

<h4 id="active-time-counter">
  활성 시간 카운터
</h4>

유휴 시간을 제외하고 Claude Code를 적극적으로 사용하는 실제 시간을 추적합니다. 이 메트릭은 입력 및 응답 읽기와 같은 사용자 상호 작용 중, 그리고 도구 실행 및 AI 응답 생성과 같은 CLI 처리 중에 증가합니다.

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `type`: 키보드 상호 작용의 경우 `"user"`, 도구 실행 및 AI 응답의 경우 `"cli"`

<h3 id="events">
  이벤트
</h3>

Claude Code는 OpenTelemetry 로그/이벤트를 통해 다음 이벤트를 내보냅니다(`OTEL_LOGS_EXPORTER`가 구성된 경우):

<h4 id="event-correlation-attributes">
  이벤트 상관 속성
</h4>

사용자가 프롬프트를 제출하면 Claude Code는 여러 API 호출을 수행하고 여러 도구를 실행할 수 있습니다. `prompt.id` 속성을 사용하면 이러한 모든 이벤트를 이를 트리거한 단일 프롬프트에 연결할 수 있습니다.

| 속성                  | 설명                                                                                                                                                                                                                                                                                                                             |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `prompt.id`         | 단일 사용자 프롬프트 처리 중에 생성된 모든 이벤트를 연결하는 UUID v4 식별자                                                                                                                                                                                                                                                                                 |
| `event.sequence`    | 이벤트 순서 지정을 위한 0 기반 카운터(세션당이 아닌 Claude Code 프로세스당 계산됨)                                                                                                                                                                                                                                                                          |
| `message.uuid`      | 세션 기록에 유지되는 메시지의 UUID(\~/.claude/projects/*/*.jsonl 파일). `assistant_response`, `api_response_body`, 및 명령 디스패치를 제외한 `user_prompt`에 있습니다(0개 이상의 메시지를 생성할 수 있음). `assistant_response` 및 `api_response_body`에서 이는 응답의 최종 기록 항목이며, 다음 턴의 `parentUuid`가 이로부터 체인됩니다. Claude Code v2.1.214 이상 필요, `api_response_body`에서 v2.1.274 이상 필요 |
| `client_request_id` | `x-client-request-id` 요청 헤더로 전송된 클라이언트 생성 UUID. 첫 번째 당사자 API 연결에서 `api_request` 및 `api_error`에 있습니다; 타사 공급자 백엔드 및 요청이 비스트리밍 폴백을 통해 재시도되었을 때는 없습니다. 요청을 응답과 쌍으로 만들고 타임아웃과 같이 서버 `request_id`를 생성하지 않은 실패에 대해 사용 가능합니다. `llm_request` 추적 스팬의 동일한 속성과 일치합니다. Claude Code v2.1.214 이상 필요                                           |

단일 프롬프트로 트리거된 모든 활동을 추적하려면 특정 `prompt.id` 값으로 이벤트를 필터링합니다. 이는 user\_prompt 이벤트, 모든 api\_request 이벤트, 및 해당 프롬프트 처리 중에 발생한 모든 tool\_result 이벤트를 반환합니다.

`event.sequence`는 Claude Code 프로세스가 시작될 때마다 0에서 시작하고 해당 프로세스의 수명 동안 증가합니다. `/clear`를 통해 계속 계산되며, 이는 새로운 `session.id`를 할당합니다. [세션을 포크하지 않고 재개](/docs/ko/how-claude-code-works#resume-or-fork-sessions)하면 세션은 `session.id`를 유지하지만 이를 재개한 프로세스에서 `event.sequence` 값을 가져오므로 한 세션 내에서 나중 이벤트가 이전 이벤트보다 낮은 값을 전달하거나 반복할 수 있습니다. 세션의 이벤트를 순서대로 정렬하려면 `event.timestamp`로 정렬하고 `event.sequence`를 사용하여 타임스탬프를 공유하는 이벤트를 순서대로 정렬합니다.

메시지 수준 재구성의 경우 각 이벤트 클래스는 세션 기록의 필드와 일치하는 키를 전달합니다. 기록 항목 형식은 [Claude Code 내부](/docs/ko/sessions#where-transcripts-are-stored)이며 버전 간에 변경되므로 이러한 필드에 조인하는 파이프라인은 모든 릴리스에서 중단될 수 있습니다; 조인을 안정적인 계약이 아닌 버전별 조인으로 취급합니다:

* `user_prompt`, `assistant_response`, 및 `api_response_body`의 `message.uuid`
* API 이벤트의 `request_id`(기록의 어시스턴트 항목에 `requestId`로 유지됨)
* `tool_result` 및 `tool_decision` 이벤트의 `tool_use_id`

<h4 id="user-prompt-event">
  사용자 프롬프트 이벤트
</h4>

사용자가 프롬프트를 제출할 때 기록됩니다.

**이벤트 이름**: `claude_code.user_prompt`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"user_prompt"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `prompt_length`: 프롬프트의 길이
* `prompt`: 프롬프트 내용. 기본적으로 삭제됩니다. `OTEL_LOG_USER_PROMPTS=1`을 설정하여 포함합니다.
* `message.uuid`: 결과 사용자 메시지의 UUID(유지된 기록 항목과 일치). 명령 디스패치에는 없습니다(0개 이상의 메시지를 생성할 수 있음). Claude Code v2.1.214 이상 필요
* `command_name`: 프롬프트가 명령을 호출할 때의 명령 이름. `compact` 또는 `debug`와 같은 기본 제공 및 번들 명령 이름은 그대로 내보냅니다; `reset`과 같은 별칭은 정규 이름이 아닌 입력한 대로 내보냅니다. 사용자 정의, 플러그인, 및 MCP 명령 이름은 `OTEL_LOG_TOOL_DETAILS=1`이 설정되지 않은 경우 `custom` 또는 `mcp`로 축소됩니다.
* `command_source`: 명령이 있을 때의 명령 출처: `builtin`, `custom`, 또는 `mcp`. 플러그인 제공 명령은 `custom`으로 보고합니다.

<h4 id="assistant-response-event">
  어시스턴트 응답 이벤트
</h4>

모델에서 텍스트 콘텐츠를 반환하는 각 API 요청 후 기록됩니다. 응답의 텍스트 블록만 포함됩니다; 생각 블록 및 도구 사용 블록은 제외됩니다. Claude Code v2.1.193 이상 필요.

**이벤트 이름**: `claude_code.assistant_response`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"assistant_response"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `response_length`: 응답 텍스트의 문자 길이
* `response`: 응답 텍스트(콘텐츠 제한(기본값 60KB)에서 잘림). 기본적으로 `<REDACTED>`로 삭제됩니다. `OTEL_LOG_ASSISTANT_RESPONSES=1`을 설정하여 포함합니다. `OTEL_LOG_ASSISTANT_RESPONSES`가 설정되지 않으면 `OTEL_LOG_USER_PROMPTS`가 대신 제어하므로 프롬프트 로깅이 켜져 있는 동안 응답을 삭제된 상태로 유지하려면 `OTEL_LOG_ASSISTANT_RESPONSES=0`을 설정합니다.
* `model`: 모델 식별자(예: "claude-sonnet-5")
* `request_id`: 응답의 `request-id` 헤더에서 Anthropic API 요청 ID. API가 반환할 때만 있습니다.
* `message.uuid`: 응답의 최종 기록 항목의 UUID. API 응답은 콘텐츠 블록당 하나의 기록 항목으로 유지됩니다; 이는 마지막 항목이며, 다음 턴의 `parentUuid`가 이로부터 체인됩니다. Claude Code v2.1.214 이상 필요
* `query_source`: 요청을 발급한 하위 시스템(예: `"repl_main_thread"`, `"compact"`, 또는 하위 에이전트 이름)

<h4 id="tool-result-event">
  도구 결과 이벤트
</h4>

도구가 실행을 완료할 때 기록됩니다. 도구 호출이 거부된 경우 내보내지 않습니다; 거부에 대해서는 [도구 결정 이벤트](#tool-decision-event)를 참조하세요.

**이벤트 이름**: `claude_code.tool_result`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"tool_result"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `tool_name`: 도구의 이름
* `tool_use_id`: 이 도구 호출의 고유 식별자. 훅에 전달된 `tool_use_id`와 일치하여 OTel 이벤트와 훅 캡처 데이터 간의 상관 관계를 허용합니다.
* `success`: `"true"` 또는 `"false"`
* `duration_ms`: 밀리초 단위의 실행 시간
* `error_type`: 도구가 실패했을 때의 오류 범주 문자열(예: `"Error:ENOENT"` 또는 `"ShellError"`)
* `error` (`OTEL_LOG_TOOL_DETAILS=1`일 때): 도구가 실패했을 때의 전체 오류 메시지
* `decision_type`: 항상 `"accept"`(이 이벤트는 도구 실행 후에만 내보내짐). 거부된 호출은 도구 결과를 생성하지 않습니다.
* `decision_source`: 권한 결정이 나온 위치. `"config"`, `"hook"`, `"user_permanent"`, 또는 `"user_temporary"` 중 하나입니다. 각 값의 의미는 [도구 결정 이벤트](#tool-decision-event)를 참조하세요. 거부 전용 소스 `"user_abort"` 및 `"user_reject"`는 이 이벤트에 나타나지 않습니다.
* `tool_input_size_bytes`: JSON 직렬화된 도구 입력의 바이트 크기
* `tool_result_size_bytes`: 도구 결과의 바이트 크기
* `mcp_server_scope`: MCP 서버 범위 식별자(MCP 도구의 경우)
*

`vcs.ref.head.revision`, `vcs.ref.head.name`, `vcs.ref.head.type` (`OTEL_LOG_TOOL_DETAILS=1`일 때): Bash 또는 PowerShell 도구에서 실행한 성공적인 `git commit`의 커밋 ID. `vcs.ref.head.revision`은 커밋 SHA이고, `vcs.ref.head.name`은 커밋된 브랜치이며, `vcs.ref.head.type`은 `branch`입니다. 커밋이 분리된 HEAD에서 이루어진 경우 이름과 유형은 생략됩니다. Claude Code v2.1.269 이상 필요

* `tool_parameters` (`OTEL_LOG_TOOL_DETAILS=1`일 때): 도구별 매개변수를 포함하는 JSON 문자열. Claude Desktop의 기본 제공 서버의 경우 Claude Desktop이 소유한 세션에서 플래그가 꺼져 있어도 `mcp_server_name`/`mcp_tool_name` 쌍이 포함됩니다([도구 결정 이벤트](#tool-decision-event)와 동일한 호스트 작성 예외). Claude Code v2.1.214 이상 필요. 매개변수는 도구에 따라 다릅니다:
  * Bash 도구의 경우: `bash_command`, `full_command`, `timeout`, `description`, 및 `dangerouslyDisableSandbox`를 포함하며, `git commit` 명령이 성공할 때 `git_commit_id` 및 `git_branch`를 포함합니다. `git_commit_id`는 커밋이 세션의 작업 디렉토리의 HEAD일 때 전체 커밋 SHA이고, 그 외에는 git의 축약된 SHA입니다. `git_branch`는 커밋된 브랜치이며, 분리된 HEAD에서는 생략됩니다.
  * 데스크톱 앱의 작업 공간 Bash 도구(또한 `tool_name`을 `Bash`로 보고함): `bash_command`, `full_command`, 및 `timeout`만 포함합니다.
  * MCP 도구의 경우: `mcp_server_name`, `mcp_tool_name`을 포함합니다.
  * Skill 도구의 경우: `skill_name`을 포함합니다.
  * Agent 도구 또는 레거시 Task 도구의 경우: `subagent_type`을 포함합니다.
* `tool_input` (`OTEL_LOG_TOOL_DETAILS=1`일 때): JSON 직렬화된 도구 인수. 512자를 초과하는 개별 값은 잘리고, 전체 페이로드는 약 4K 문자로 제한됩니다. MCP 도구를 포함한 모든 도구에 적용됩니다.

<h4 id="api-request-event">
  API 요청 이벤트
</h4>

Claude에 대한 각 API 요청에 대해 기록됩니다.

**이벤트 이름**: `claude_code.api_request`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"api_request"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `model`: 사용된 모델(예: "claude-sonnet-5")
* `cost_usd`: USD 단위의 예상 비용
* `cost_usd_micros`: 미국 달러의 백만분의 일 단위의 예상 비용(정수로 내보냄)
* `duration_ms`: 밀리초 단위의 요청 기간
* `input_tokens`: 입력 토큰 수
* `output_tokens`: 출력 토큰 수
* `cache_read_tokens`: 캐시에서 읽은 토큰 수
* `cache_creation_tokens`: 캐시 생성에 사용된 토큰 수
* `request_id`: 응답의 `request-id` 헤더에서 Anthropic API 요청 ID(예: `"req_011..."`). API가 반환할 때만 있습니다.
* `client_request_id`: `x-client-request-id` 요청 헤더로 전송된 클라이언트 생성 UUID; 있을 때에 대해서는 [이벤트 상관 속성](#event-correlation-attributes) 표를 참조하세요. Claude Code v2.1.214 이상 필요
* `speed`: 빠른 모드가 활성화되었는지 여부를 나타내는 `"fast"` 또는 `"normal"`
* `query_source`: 요청을 발급한 하위 시스템(예: `"repl_main_thread"`, `"compact"`, 또는 하위 에이전트 이름)
* `effort`: 요청에 적용된 [노력 수준](/docs/ko/model-config#adjust-effort-level): `"low"`, `"medium"`, `"high"`, `"xhigh"`, 또는 `"max"`. Claude Code가 노력 수준을 보내지 않을 때(예: 노력을 지원하지 않는 모델)는 없습니다.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: 요청에 대한 스킬, 플러그인, 에이전트, 및 MCP 속성. 정의 및 삭제 동작은 [비용 카운터](#cost-counter)를 참조하세요.

<h4 id="api-error-event">
  API 오류 이벤트
</h4>

Claude에 대한 API 요청이 실패할 때 기록됩니다.

**이벤트 이름**: `claude_code.api_error`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"api_error"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `model`: 사용된 모델(예: "claude-sonnet-5")
* `error`: 오류 메시지
* `status_code`: HTTP 상태 코드(숫자). 연결 실패와 같은 비 HTTP 오류의 경우 없습니다.
* `duration_ms`: 밀리초 단위의 요청 기간
* `attempt`: 초기 요청을 포함한 총 시도 횟수(`1`은 재시도가 발생하지 않았음을 의미)
* `request_id`: 응답의 `request-id` 헤더에서 Anthropic API 요청 ID(예: `"req_011..."`). API가 반환할 때만 있습니다.
* `client_request_id`: `x-client-request-id` 요청 헤더로 전송된 클라이언트 생성 UUID. 타임아웃 또는 연결 오류와 같은 실패가 서버 `request_id`를 생성하지 않았을 때도 사용 가능합니다; 있을 때에 대해서는 [이벤트 상관 속성](#event-correlation-attributes) 표를 참조하세요. Claude Code v2.1.214 이상 필요
* `speed`: 빠른 모드가 활성화되었는지 여부를 나타내는 `"fast"` 또는 `"normal"`
* `query_source`: 요청을 발급한 하위 시스템(예: `"repl_main_thread"`, `"compact"`, 또는 하위 에이전트 이름)
* `effort`: 요청에 적용된 [노력 수준](/docs/ko/model-config#adjust-effort-level). Claude Code가 노력 수준을 보내지 않을 때(예: 노력을 지원하지 않는 모델)는 없습니다.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: 요청에 대한 스킬, 플러그인, 에이전트, 및 MCP 속성. 정의 및 삭제 동작은 [비용 카운터](#cost-counter)를 참조하세요.

<h4 id="api-refusal-event">
  API 거부 이벤트
</h4>

API 요청이 `stop_reason: "refusal"`을 반환할 때 기록됩니다. 거부는 HTTP 오류가 아닌 성공적인 응답 스트림에 도착하므로 `api_error` 이벤트는 이에 대해 발생하지 않습니다. 이 이벤트를 사용하면 거부 빈도를 추적하고 거부를 `api_request` 및 `api_error`와 동일한 속성으로 그룹화할 수 있습니다.

**이벤트 이름**: `claude_code.api_refusal`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"api_refusal"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `model`: 요청의 모델 식별자
* `request_id`: 응답의 `request-id` 헤더에서 Anthropic API 요청 ID(예: `"req_011..."`). API가 반환할 때만 있습니다.
* `query_source`: 요청을 발급한 하위 시스템(예: `"repl_main_thread"`, `"compact"`, 또는 하위 에이전트 이름). 정의는 [`api_request`](#api-request-event)를 참조하세요.
* `speed`: [빠른 모드](/docs/ko/fast-mode)가 활성화되었을 때 `"fast"`, 또는 `"normal"`
* `attempt`: 재시도 시도 번호. 첫 번째 시도는 `1`입니다.
* `effort`: 요청에 적용된 [노력 수준](/docs/ko/model-config#adjust-effort-level). Claude Code가 노력 수준을 보내지 않을 때(예: 노력을 지원하지 않는 모델)는 없습니다.
* `server_fallback_hop`: API의 서버 측 모델 폴백이 이미 이 거부를 다른 모델에서 재시도했으므로 사용자가 이 특정 거부를 보지 못했을 때 `true`. 요청이 거부로 끝났을 때 `false`. 단일 턴은 폴백 모델도 거부할 때 `true` 홉 이벤트와 나중의 `false` 최종 이벤트를 모두 내보낼 수 있습니다.
* `has_category`: API 응답이 `"cyber"`, `"bio"`, `"frontier_llm"`, 또는 `"reasoning_extraction"`의 `stop_details.category`를 전달했을 때 `true`. 응답이 범주를 전달하지 않았거나 해당 집합 외의 값을 전달했을 때 `false`. `server_fallback_hop`이 `true`일 때는 없습니다(홉 블록은 `stop_details`를 전달하지 않음).
* `has_explanation`: API 응답이 `stop_details.explanation`을 전달했을 때 `true`, 그 외에는 `false`. `server_fallback_hop`이 `true`일 때는 없습니다.
* `category`: API 응답의 `stop_details.category` 값. `"cyber"`, `"bio"`, `"frontier_llm"`, 또는 `"reasoning_extraction"` 중 하나입니다. `OTEL_LOG_TOOL_DETAILS=1`이 설정되고 `has_category`가 `true`일 때만 있습니다.
* `agent.name`, `skill.name`, `plugin.name`, `marketplace.name`, `mcp_server.name`, `mcp_tool.name`: 요청에 대한 스킬, 플러그인, 에이전트, 및 MCP 속성. 정의 및 삭제 동작은 [비용 카운터](#cost-counter)를 참조하세요.

<h4 id="api-request-body-event">
  API 요청 본문 이벤트
</h4>

`OTEL_LOG_RAW_API_BODIES`가 설정되었을 때 각 API 요청 시도에 대해 기록됩니다. 시도당 하나의 이벤트가 내보내지므로 조정된 매개변수를 사용한 재시도는 각각 자신의 이벤트를 생성합니다.

**이벤트 이름**: `claude_code.api_request_body`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"api_request_body"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `body`: JSON 직렬화된 Messages API 요청 매개변수(시스템 프롬프트, 메시지, 및 도구 등)(콘텐츠 제한(기본값 60KB)에서 잘림). 이전 어시스턴트 턴의 확장 생각 콘텐츠는 삭제됩니다. 인라인 모드(`OTEL_LOG_RAW_API_BODIES=1`)에서만 내보냅니다.
* `body_ref`: 잘리지 않은 본문을 포함하는 `<dir>/<uuid>.request.json` 파일의 절대 경로. 파일 모드(`OTEL_LOG_RAW_API_BODIES=file:<dir>`)에서만 내보냅니다.
* `body_length`: 잘리지 않은 본문 길이. `OTEL_LOG_RAW_API_BODIES=file:<dir>`일 때 UTF-8 바이트, 또는 `=1`일 때 UTF-16 코드 단위
* `body_truncated`: 인라인 잘림이 발생했을 때 `"true"`. 파일 모드 및 잘림이 발생하지 않았을 때는 없습니다.
* `model`: 요청 매개변수의 모델 식별자
* `query_source`: 요청을 발급한 하위 시스템(예: `"compact"`)
* `request_body_id`: 이 시도의 요청 본문을 식별하는 UUID. 성공한 시도의 [`api_response_body` 이벤트](#api-response-body-event)는 동일한 값을 전달하므로 응답을 생성한 정확한 요청과 쌍으로 만들 수 있습니다. Claude Code v2.1.274 이상 필요

<h4 id="api-response-body-event">
  API 응답 본문 이벤트
</h4>

`OTEL_LOG_RAW_API_BODIES`가 설정되었을 때 각 성공적인 API 응답에 대해 기록됩니다.

파일 모드(`OTEL_LOG_RAW_API_BODIES=file:<dir>`)에서 Claude Code는 또한 각 성공적인 응답에 대해 `<dir>/index.jsonl`에 하나의 JSON 라인을 추가하며, `timestamp`, `session_id`, `query_source`, `model`, `request_id`, `message_id`, `message_uuid`, `request_file`, 및 `response_file` 필드를 포함합니다. 이를 읽어 원격 측정 백엔드를 쿼리하지 않고 주어진 기록 메시지 뒤의 요청 및 응답 파일을 찾습니다. 인덱스 파일은 Claude Code v2.1.274 이상 필요합니다.

**이벤트 이름**: `claude_code.api_response_body`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"api_response_body"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `body`: JSON 직렬화된 Messages API 응답(ID, 콘텐츠 블록, 사용량, 및 중지 이유 포함)(콘텐츠 제한(기본값 60KB)에서 잘림). 확장 생각 콘텐츠는 삭제됩니다. 인라인 모드(`OTEL_LOG_RAW_API_BODIES=1`)에서만 내보냅니다.
* `body_ref`: 잘리지 않은 본문을 포함하는 `<dir>/<request_id>.response.json` 파일의 절대 경로. 파일 모드(`OTEL_LOG_RAW_API_BODIES=file:<dir>`)에서만 내보냅니다.
* `body_length`: 잘리지 않은 본문 길이. `OTEL_LOG_RAW_API_BODIES=file:<dir>`일 때 UTF-8 바이트, 또는 `=1`일 때 UTF-16 코드 단위
* `body_truncated`: 인라인 잘림이 발생했을 때 `"true"`. 파일 모드 및 잘림이 발생하지 않았을 때는 없습니다.
* `model`: 모델 식별자
* `query_source`: 요청을 발급한 하위 시스템
* `request_id`: 응답의 `request-id` 헤더에서 Anthropic API 요청 ID(예: `"req_011..."`). API가 반환할 때만 있습니다.
* `request_body_id`: 이 응답이 답변하는 [`api_request_body` 이벤트](#api-request-body-event)의 `request_body_id`. Claude Code v2.1.274 이상 필요
* `message.id`: API가 응답에 할당한 메시지 ID(응답 본문의 `id` 필드). Claude Code v2.1.274 이상 필요
* `message.uuid`: 응답의 최종 기록 항목의 UUID. `request_body_id`와 함께 기록 메시지를 뒤의 요청 및 응답 본문에 연결합니다. Claude Code v2.1.274 이상 필요

<h4 id="tool-decision-event">
  도구 결정 이벤트
</h4>

도구 권한 결정이 이루어질 때(수락/거부) 기록됩니다.

**이벤트 이름**: `claude_code.tool_decision`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"tool_decision"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `tool_name`: 도구의 이름(예: "Read", "Edit", "Write", "NotebookEdit")
* `tool_use_id`: 이 도구 호출의 고유 식별자. 훅에 전달된 `tool_use_id`와 일치하여 OTel 이벤트와 훅 캡처 데이터 간의 상관 관계를 허용합니다.
* `decision`: `"accept"` 또는 `"reject"`
* `tool_source`: 항상 있습니다. 도구의 출처(CLI 작성 값의 폐쇄 집합). Claude Code v2.1.214 이상 필요
  * `"builtin"`: CLI 자체의 도구
  * `"mcp"`: 일반적으로 MCP 서버
  * `"sdk_host_builtin_mcp"`: Claude Desktop 자체에 내장된 프로세스 내 서버(Claude Desktop이 소유한 세션). Claude Desktop은 자신의 엔드포인트 중 하나(`claude-desktop`, `claude-desktop-3p`, 또는 `local-agent`)에서 시작한 세션을 소유합니다(해당 세션이 중첩된 자식이 아닐 때); 중첩된 세션(Claude Code 자체가 생성하는 세션 포함)은 이러한 서버를 `"mcp"`로 보고합니다.
* `source`: 결정이 나온 위치:
  * `"config"`: 프롬프트 없이 자동으로 결정됨(프로젝트 설정, 사용자의 개인 설정의 허용 또는 거부 규칙, 엔터프라이즈 관리 정책, `--allowedTools` 또는 `--disallowedTools` 플래그, 활성 권한 모드, 동일한 대화형 CLI 세션의 이전 프롬프트에서 세션 범위 부여, 또는 도구가 본질적으로 안전하기 때문). 이벤트는 이러한 소스 중 어느 것이 일치했는지 나타내지 않습니다. Claude Code는 또한 권한 프롬프트 요청 자체가 실패할 때(예: Agent SDK의 [`canUseTool`](/docs/ko/agent-sdk/typescript#canusetool) 콜백 또는 [`--permission-prompt-tool`](/docs/ko/cli-reference#cli-flags) 도구가 잘못된 결과를 반환하거나 입력 스트림이 요청 대기 중에 닫힐 때) `"config"`을 보고합니다. v2.1.216 이전에는 Claude Code가 이러한 실패를 `"user_reject"`로 보고했습니다.
  * `"hook"`: `PreToolUse` 또는 `PermissionRequest` 훅이 결정을 반환했습니다.
  * `"user_permanent"`: 사용자가 권한 프롬프트에서 "Yes, and don't ask again for ..."을 선택했을 때 내보내집니다(개인 설정에 허용 규칙을 저장함). 대화형 CLI에서는 해당 선택 자체에만 내보내집니다; 나중의 호출이 저장된 규칙과 일치하면 `"config"`을 내보냅니다. Agent SDK 또는 비대화형 `-p` 세션에서는 초기 선택과 나중의 규칙 일치 모두 `"user_permanent"`를 내보냅니다. 수락으로 취급됩니다.
  * `"user_temporary"`: 사용자가 권한 프롬프트에서 "Yes"를 선택했거나 파일 편집 또는 읽기 프롬프트에서 세션의 나머지 부분에 대한 액세스를 부여하는 옵션을 선택했을 때 내보내집니다. 대화형 CLI에서는 해당 선택 자체에만 내보내집니다; 나중의 호출이 해당 세션 범위 부여와 일치하면 `"config"`을 내보냅니다. Agent SDK 또는 비대화형 `-p` 세션에서는 선택과 나중의 일치 모두 `"user_temporary"`를 내보냅니다. 수락으로 취급됩니다.
  * `"user_abort"`: 사용자가 권한 프롬프트를 답변 없이 해제했을 때 내보내집니다. Agent SDK 및 비대화형 `-p` 세션에서는 `canUseTool` 또는 `--permission-prompt-tool` 권한 요청이 대기 중인 동안 턴을 중단하는 것을 포함합니다; v2.1.216 이전에는 Claude Code가 해당 중단을 `"user_reject"`로 보고했습니다. 거부로 취급됩니다.
  * `"user_reject"`: 사용자가 프롬프트에서 "No"를 선택했을 때 내보내집니다. 대화형 CLI에서는 해당 선택 자체에만 내보내집니다; 사용자의 개인 설정의 거부 규칙과 일치하는 호출은 `"config"`을 내보냅니다. Agent SDK 또는 비대화형 `-p` 세션에서는 개인 설정의 거부 규칙과 일치하는 호출이 `"user_reject"`를 내보냅니다. 거부로 취급됩니다.
* `tool_parameters` (`OTEL_LOG_TOOL_DETAILS=1`일 때): 도구별 매개변수를 포함하는 JSON 문자열. [도구 결과 이벤트](#tool-result-event)와 동일한 형태이지만 `git_commit_id`와 같은 실행 후 필드는 제외됩니다. 수락된 호출의 경우 `tool_result`와 다를 수 있습니다(권한 결정이 `updatedInput`을 통해 도구 입력을 다시 쓸 경우). 이 속성을 사용하여 `decision`이 `"reject"`일 때 어느 명령이 거부되었는지 확인합니다.
  * `"sdk_host_builtin_mcp"` 도구의 경우: `OTEL_LOG_TOOL_DETAILS`가 꺼져 있어도 `mcp_server_name` 및 `mcp_tool_name`이 포함됩니다(호스트 애플리케이션이 이러한 이름을 정의하기 때문); 이들이 없으면 이러한 기본 제공 서버 중 하나에 대한 거부된 호출은 기본 스트림에서 속성화할 수 없습니다. 사용자 구성 MCP 서버의 경우 이벤트의 `tool_name`은 항상 리터럴 `"mcp_tool"`이고, 서버 및 도구 이름은 플래그가 켜져 있을 때만 `tool_parameters`에 나타납니다; 인수 콘텐츠는 모든 곳에서 플래그가 필요합니다. Claude Code v2.1.214 이상 필요
  * Bash 도구의 경우: `bash_command`, `full_command`, `timeout`, `description`, `dangerouslyDisableSandbox`를 포함합니다. 데스크톱 앱의 작업 공간 bash 도구도 `tool_name`을 `Bash`로 보고하지만 `bash_command`, `full_command`, 및 `timeout`만 포함합니다.
  * MCP 도구의 경우: `mcp_server_name`, `mcp_tool_name`을 포함합니다.
  * Skill 도구의 경우: `skill_name`을 포함합니다.
  * Agent 도구 또는 레거시 Task 도구의 경우: `subagent_type`을 포함합니다.

<h4 id="permission-mode-changed-event">
  권한 모드 변경 이벤트
</h4>

권한 모드가 변경될 때(예: `Shift+Tab` 순환, 계획 모드 종료, 또는 자동 모드 게이트 확인) 기록됩니다.

**이벤트 이름**: `claude_code.permission_mode_changed`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"permission_mode_changed"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `from_mode`: 이전 권한 모드(예: `"default"`, `"plan"`, `"acceptEdits"`, `"auto"`, 또는 `"bypassPermissions"`)
* `to_mode`: 새로운 권한 모드
* `trigger`: 변경을 야기한 것. `"shift_tab"`, `"exit_plan_mode"`, `"auto_gate_denied"`, 또는 `"auto_opt_in"` 중 하나입니다. SDK 또는 브리지에서 전환이 시작될 때는 없습니다.

<h4 id="auth-event">
  인증 이벤트
</h4>

`/login` 또는 `/logout`이 완료될 때 기록됩니다.

**이벤트 이름**: `claude_code.auth`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"auth"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `action`: `"login"` 또는 `"logout"`
* `success`: `"true"` 또는 `"false"`
* `auth_method`: 인증 방법(예: `"oauth"`)
* `error_category`: 작업이 실패했을 때의 범주별 오류 종류. 원시 오류 메시지는 절대 포함되지 않습니다.
* `status_code`: 작업이 HTTP 오류로 실패했을 때의 HTTP 상태 코드(문자열)

<h4 id="mcp-server-connection-event">
  MCP 서버 연결 이벤트
</h4>

MCP 서버가 연결, 연결 해제, 또는 연결 실패할 때 기록됩니다.

**이벤트 이름**: `claude_code.mcp_server_connection`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"mcp_server_connection"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `status`: `"connected"`, `"failed"`, 또는 `"disconnected"`
* `transport_type`: 서버 전송(예: `"stdio"`, `"sse"`, 또는 `"http"`)
* `server_scope`: 서버가 구성된 범위(예: `"user"`, `"project"`, 또는 `"local"`)
* `duration_ms`: 밀리초 단위의 연결 시도 기간
* `error_code`: 연결이 실패했을 때의 오류 코드
* `is_plugin`: 서버가 플러그인에 의해 제공될 때 `true`, 그 외에는 `false`
* `plugin_id_hash` (`is_plugin`이 `true`일 때): 플러그인 이름과 마켓플레이스의 안정적인 해시(이름을 노출하지 않고 플러그인별로 이벤트를 그룹화하기 위해). Claude Code는 [플러그인 로드 이벤트](#plugin-loaded-event)에서 설명한 대로 계산합니다.
* `plugin.name` (`is_plugin`이 `true`일 때): 서버를 제공하는 플러그인의 이름. 타사 플러그인의 경우 `OTEL_LOG_TOOL_DETAILS=1`이 아닌 경우 리터럴 문자열 `"third-party"`입니다; 이는 기본적으로 로그에 타사 플러그인 이름이 나타나는 것을 방지합니다. 공식 Anthropic 소스의 플러그인은 항상 이름으로 식별됩니다. `plugin_id_hash` 및 `plugin.name` 속성은 자신의 모니터링 백엔드로 흐르며 Anthropic으로 전송되지 않습니다.
* `server_name` (`OTEL_LOG_TOOL_DETAILS=1`일 때): 구성된 서버 이름
* `error` (`OTEL_LOG_TOOL_DETAILS=1`일 때): 연결이 실패했을 때의 전체 오류 메시지

<h4 id="internal-error-event">
  내부 오류 이벤트
</h4>

Claude Code가 예상치 못한 내부 오류를 포착할 때 기록됩니다. 오류 클래스 이름과 errno 스타일 코드만 기록됩니다. 오류 메시지와 스택 추적은 절대 포함되지 않습니다. 이 이벤트는 Amazon Bedrock, Google Cloud의 Agent Platform, 또는 Microsoft Foundry에 대해 실행하거나 `DISABLE_ERROR_REPORTING`이 설정되었을 때 내보내지 않습니다.

**이벤트 이름**: `claude_code.internal_error`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"internal_error"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `error_name`: 오류 클래스 이름(예: `"TypeError"` 또는 `"SyntaxError"`)
* `error_code`: 오류에 있을 때 Node.js errno 코드(예: `"ENOENT"`)

<h4 id="plugin-installed-event">
  플러그인 설치 이벤트
</h4>

플러그인이 설치를 완료할 때 기록됩니다(`claude plugin install` CLI 명령 및 대화형 `/plugin` UI 모두).

**이벤트 이름**: `claude_code.plugin_installed`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"plugin_installed"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `marketplace.is_official`: 마켓플레이스가 공식 Anthropic 마켓플레이스일 때 `"true"`, 그 외에는 `"false"`
* `install.trigger`: `"cli"` 또는 `"ui"`
* `plugin.name`: 설치된 플러그인의 이름. 타사 마켓플레이스의 경우 `OTEL_LOG_TOOL_DETAILS=1`일 때만 포함됩니다.
* `plugin.version`: 마켓플레이스 항목에서 선언된 플러그인 버전. 타사 마켓플레이스의 경우 `OTEL_LOG_TOOL_DETAILS=1`일 때만 포함됩니다.
* `marketplace.name`: 플러그인이 설치된 마켓플레이스. 타사 마켓플레이스의 경우 `OTEL_LOG_TOOL_DETAILS=1`일 때만 포함됩니다.

<h4 id="plugin-loaded-event">
  플러그인 로드 이벤트
</h4>

세션 시작 시 활성화된 플러그인당 한 번 기록됩니다. 이 이벤트를 사용하여 플릿 전체에서 활성화된 플러그인을 인벤토리하세요(설치 작업 자체를 기록하는 `plugin_installed`의 보완).

**이벤트 이름**: `claude_code.plugin_loaded`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"plugin_loaded"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `plugin.name`: 플러그인의 이름. 공식 마켓플레이스 및 기본 제공 번들 외부의 플러그인의 경우 `OTEL_LOG_TOOL_DETAILS=1`이 아닌 경우 `"third-party"`입니다.
* `marketplace.name`: 플러그인이 설치된 마켓플레이스(알려진 경우). `plugin.name`과 동일한 조건에서 `"third-party"`로 삭제됩니다.
* `plugin.version`: 플러그인 매니페스트의 버전. 이름이 삭제되지 않고 매니페스트가 버전을 선언할 때만 포함됩니다.
* `plugin.scope`: 플러그인의 출처 범주: `"official"`, `"community"`, `"org"`, `"user-local"`, 또는 `"default-bundle"`
* `enabled_via`: 플러그인이 활성화되게 된 방식: `"default-enable"`, `"org-policy"`, `"admin-install"`, `"seed-mount"`, 또는 `"user-install"`.&#x20;
  `"admin-install"` 값은 플러그인이 [**조직 설정 > 플러그인 & 스킬**](https://claude.ai/admin-settings/skills?tab=inventory)에서 조직에 필수 또는 자동 설치로 설정되어 있음을 의미합니다. v2.1.246 이전에는 Claude Code가 이러한 플러그인을 `"user-install"` 또는 `"seed-mount"`로 보고했습니다.
* `plugin_id_hash`: 플러그인 이름과 마켓플레이스의 결정적 해시(구성된 내보내기로만 전송됨). 이름을 기록하지 않고 플릿 전체에서 로드된 서로 다른 타사 플러그인을 계산할 수 있습니다. [claude.ai에서 동기화된 플러그인](/docs/ko/plugins/loading#synced-plugins)의 경우 Claude Code는 플러그인 이름을 claude.ai가 플러그인에 대해 보고하는 마켓플레이스 이름 또는 그 외의 경우 `synced`와 함께 해시합니다. v2.1.246 이전에는 Claude Code가 해시에서 claude.ai가 보고하는 마켓플레이스 이름을 사용하지 않았습니다.
* `has_hooks`: 플러그인이 훅을 제공하는지 여부
* `has_mcp`: 플러그인이 MCP 서버를 제공하는지 여부
* `host_owned_mcp`: SDK 호스트가 이 플러그인의 MCP 연결을 관리하고 Claude Code가 플러그인의 MCP 서버 구성 읽기를 건너뛸 때 `true`, 그 외에는 `false`. Claude Code v2.1.172 이상 필요
* `skill_path_count`: 플러그인이 선언하는 스킬 디렉토리 수
* `command_path_count`: 플러그인이 선언하는 명령 디렉토리 수
* `agent_path_count`: 플러그인이 선언하는 에이전트 디렉토리 수
* `safe_mode`: 세션이 [`--safe-mode`](/docs/ko/cli-reference)로 시작되었을 때 `"true"`, 그 외에는 `"false"`. 안전 모드에서 이 이벤트는 구성된 인벤토리만 보고합니다; 플러그인의 명령, 스킬, 훅, 및 MCP 서버는 로드되지 않습니다. Claude Code v2.1.169 이상 필요

<h4 id="skill-activated-event">
  스킬 활성화 이벤트
</h4>

스킬이 호출될 때 기록됩니다(Claude가 Skill 도구를 통해 호출하거나 `/` 명령으로 실행할 때).

**이벤트 이름**: `claude_code.skill_activated`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"skill_activated"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `skill.name`: 스킬의 이름. 사용자 정의 및 타사 플러그인 스킬의 경우 `OTEL_LOG_TOOL_DETAILS=1`이 아닌 경우 플레이스홀더 `"custom_skill"`입니다.
* `invocation_trigger`: 스킬이 트리거된 방식(`"user-slash"`, `"claude-proactive"`, 또는 `"nested-skill"`)
* `skill.source`: 스킬이 로드된 위치(예: `"bundled"`, `"userSettings"`, `"projectSettings"`, `"plugin"`)
* `skill.kind`: 스킬이 워크플로우 스킬일 때 `"workflow"`. 그 외에는 없습니다.
* `plugin.name` (`OTEL_LOG_TOOL_DETAILS=1`이거나 플러그인이 공식 마켓플레이스에서 온 경우): 스킬이 플러그인에 의해 제공될 때 소유 플러그인의 이름
* `marketplace.name` (`OTEL_LOG_TOOL_DETAILS=1`이거나 플러그인이 공식 마켓플레이스에서 온 경우): 스킬이 플러그인에 의해 제공될 때 소유 플러그인이 설치된 마켓플레이스

<h4 id="at-mention-event">
  @ 멘션 이벤트
</h4>

Claude Code가 프롬프트의 `@` 멘션을 해결할 때 기록됩니다. 모든 멘션이 이벤트를 내보내는 것은 아닙니다: 권한 거부, 과도한 파일, PDF 참조 첨부, 및 디렉토리 목록 실패와 같은 조기 종료 경로는 로깅 없이 반환됩니다.

**이벤트 이름**: `claude_code.at_mention`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"at_mention"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `mention_type`: 멘션의 유형(`"file"`, `"directory"`, `"agent"`, `"mcp_resource"`, `"peer"`).&#x20;
  `"peer"` 값은 [다른 Claude Code 세션](/docs/ko/cross-session-messaging) 중 하나를 멘션했음을 의미합니다. Claude Code v2.1.232 이상 필요
* `success`: 멘션이 성공적으로 해결되었는지 여부(`"true"` 또는 `"false"`)

<h4 id="api-retries-exhausted-event">
  API 재시도 소진 이벤트
</h4>

API 요청이 두 번 이상 시도 후 실패할 때 한 번 기록됩니다. 최종 `api_error` 이벤트와 함께 내보내집니다.

**이벤트 이름**: `claude_code.api_retries_exhausted`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"api_retries_exhausted"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `model`: 사용된 모델
* `error`: 최종 오류 메시지
* `status_code`: HTTP 상태 코드(숫자). 비 HTTP 오류의 경우 없습니다.
* `total_attempts`: 수행된 총 시도 횟수
* `total_retry_duration_ms`: 모든 시도에 걸친 총 벽시계 시간
* `speed`: `"fast"` 또는 `"normal"`

<h4 id="hook-registered-event">
  훅 등록 이벤트
</h4>

세션 시작 시 구성된 훅당 한 번 기록됩니다. 이 이벤트를 사용하여 플릿 전체에서 활성화된 훅을 인벤토리하세요(실행별 `hook_execution_start` 및 `hook_execution_complete` 이벤트의 보완).

**이벤트 이름**: `claude_code.hook_registered`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"hook_registered"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `hook_event`: 훅 이벤트 유형(예: `"PreToolUse"` 또는 `"PostToolUse"`)
* `hook_type`: 훅 구현 유형: `"command"`, `"prompt"`, `"mcp_tool"`, `"http"`, 또는 `"agent"`
* `hook_source`: 훅이 정의된 위치: `"userSettings"`, `"projectSettings"`, `"localSettings"`, `"flagSettings"`, `"policySettings"`, 또는 `"pluginHook"`
* `safe_mode`: 세션이 [`--safe-mode`](/docs/ko/cli-reference)로 시작되었을 때 `"true"`, 그 외에는 `"false"`. Claude Code v2.1.169 이상 필요
* `hook_matcher` (`OTEL_LOG_TOOL_DETAILS=1`일 때): 훅 구성에서 설정된 경우 훅 구성의 매처 문자열
* `plugin.name` (`hook_source`가 `"pluginHook"`일 때): 기여하는 플러그인의 이름. 공식 마켓플레이스 및 기본 제공 번들 외부의 플러그인의 경우 `OTEL_LOG_TOOL_DETAILS=1`이 아닌 경우 `"third-party"`입니다.
* `plugin_id_hash` (`hook_source`가 `"pluginHook"`일 때): 플러그인 이름과 마켓플레이스의 결정적 해시(구성된 내보내기로만 전송됨). 이름을 기록하지 않고 기여하는 서로 다른 플러그인을 계산할 수 있습니다. Claude Code는 [플러그인 로드 이벤트](#plugin-loaded-event)에서 설명한 대로 계산합니다.

<h4 id="hook-execution-start-event">
  훅 실행 시작 이벤트
</h4>

하나 이상의 훅이 훅 이벤트에 대해 실행을 시작할 때 기록됩니다.

**이벤트 이름**: `claude_code.hook_execution_start`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"hook_execution_start"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `hook_event`: 훅 이벤트 유형(예: `"PreToolUse"` 또는 `"PostToolUse"`)
* `hook_name`: 매처를 포함한 전체 훅 이름(예: `"PreToolUse:Write"`)
* `num_hooks`: 일치하는 훅 명령 수
* `managed_only`: 관리 정책 훅만 허용될 때 `"true"`
* `hook_source`: `"policySettings"` 또는 `"merged"`
* `safe_mode`: 세션이 [`--safe-mode`](/docs/ko/cli-reference)로 시작되었을 때 `"true"`, 그 외에는 `"false"`. Claude Code v2.1.169 이상 필요
* `hook_definitions`: JSON 직렬화된 훅 구성. 상세 베타 추적과 `OTEL_LOG_TOOL_DETAILS=1`이 모두 활성화되었을 때만 포함됩니다.

<h4 id="hook-execution-complete-event">
  훅 실행 완료 이벤트
</h4>

훅 이벤트의 모든 훅이 완료되었을 때 기록됩니다.

**이벤트 이름**: `claude_code.hook_execution_complete`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"hook_execution_complete"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `hook_event`: 훅 이벤트 유형
* `hook_name`: 매처를 포함한 전체 훅 이름
* `num_hooks`: 일치하는 훅 명령 수
* `num_success`: 성공적으로 완료된 수
* `num_blocking`: 차단 결정을 반환한 수
* `num_non_blocking_error`: 차단 없이 실패한 수
* `num_cancelled`: 완료 전에 취소된 수
* `total_duration_ms`: 모든 일치하는 훅의 벽시계 기간
* `stdout_chars`: 성공한 일치하는 훅 전체의 stdout 총 문자 수. Claude Code v2.1.280 이상 필요
* `additional_context_chars`: 일치하는 훅이 반환한 `additionalContext`의 총 문자 수. Claude Code v2.1.280 이상 필요
* `system_message_chars`: 일치하는 훅이 반환한 `systemMessage`의 총 문자 수. Claude Code v2.1.280 이상 필요
* `initial_user_message_chars`: 일치하는 훅이 반환한 `initialUserMessage`의 총 문자 수. Claude Code v2.1.280 이상 필요
* `num_outputs_persisted`: [10,000자 상한](/docs/ko/hooks#json-output)을 초과한 훅 출력 수(Claude Code가 파일에 저장함). Claude Code v2.1.280 이상 필요
* `managed_only`: 관리 정책 훅만 허용될 때 `"true"`
* `hook_source`: `"policySettings"` 또는 `"merged"`
* `safe_mode`: 세션이 [`--safe-mode`](/docs/ko/cli-reference)로 시작되었을 때 `"true"`, 그 외에는 `"false"`. Claude Code v2.1.169 이상 필요
* `hook_definitions`: JSON 직렬화된 훅 구성. 상세 베타 추적과 `OTEL_LOG_TOOL_DETAILS=1`이 모두 활성화되었을 때만 포함됩니다.

<h4 id="hook-plugin-metrics-event">
  훅 플러그인 메트릭 이벤트
</h4>

공식 마켓플레이스 플러그인 훅이 호출별 메트릭을 내보낼 때 기록됩니다. 공식 Anthropic 마켓플레이스에서 설치된 플러그인만 이를 내보낼 수 있습니다. 타사 마켓플레이스 플러그인 및 사용자 구성 훅은 이 이벤트로 내보내지 않습니다. 이 이벤트를 사용하여 자신의 관찰성 스택에서 찾기 비율, 비용, 및 플러그인 동작의 기간과 같은 것을 모니터링합니다.

**이벤트 이름**: `claude_code.hook_plugin_metrics`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"hook_plugin_metrics"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
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
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `trigger`: `"auto"` 또는 `"manual"`
* `success`: `"true"` 또는 `"false"`
* `duration_ms`: 압축 기간
* `pre_tokens`: 압축 전 대략적인 토큰 수
* `post_tokens`: 압축 후 대략적인 토큰 수
* `error`: 압축이 실패했을 때의 오류 메시지
* `precompute_reuse`: `trigger`가 `"manual"`일 때만 설정됩니다. 자동 압축은 컨텍스트 윈도우가 채워지기 전에 백그라운드에서 요약을 준비할 수 있으며, 이 속성은 `/compact`가 해당 준비된 요약을 재사용했는지 기록합니다. `"hit"`는 재사용되었음을 의미합니다; `"miss_custom_instructions"`, `"miss_hook"`, 및 `"miss_not_ready"`는 대신 새로운 요약이 계산된 이유를 제공합니다. Claude Code v2.1.153 이상 필요

<h4 id="subagent-completed-event">
  하위 에이전트 완료 이벤트
</h4>

[하위 에이전트](/docs/ko/sub-agents)가 완료되고 결과를 시작한 대화에 반환할 때 기록됩니다. 하위 에이전트 유형별로 도구 사용 및 실행 시간을 롤업하는 데 사용합니다; 토큰 또는 비용 롤업의 경우 [토큰 카운터](#token-counter) 및 [비용 카운터](#cost-counter)를 `query_source` `"subagent"`로 필터링하여 사용합니다(이 이벤트의 `total_tokens`은 최종 요청만 포함). `"subagent"` 범주는 또한 하위 에이전트 이벤트를 내보내지 않는 에이전트 기반 훅의 요청도 계산합니다.

**이벤트 이름**: `claude_code.subagent_completed`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"subagent_completed"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `agent_type`: 하위 에이전트 유형. 기본 제공 에이전트 이름과 공식 마켓플레이스 플러그인의 에이전트는 그대로 나타납니다; 다른 에이전트 이름은 `OTEL_LOG_TOOL_DETAILS=1`이 설정되지 않은 경우 `"custom"`으로 대체됩니다.
* `agent.source`: 에이전트 정의가 나온 위치: `built-in`, `plugin`, 또는 `userSettings` 또는 `projectSettings`와 같은 사용자 정의 에이전트를 정의한 설정 소스
* `is_built_in`: 하위 에이전트가 기본 제공 에이전트 유형인지 여부
* `is_async`: 하위 에이전트가 [백그라운드](/docs/ko/sub-agents#run-subagents-in-foreground-or-background)에서 실행되었는지 여부
* `total_tokens`: 하위 에이전트의 최종 API 요청의 토큰 풋프린트: 해당 하나의 요청의 입력, 캐시 생성, 캐시 읽기, 및 출력 토큰(대략 완료 시 하위 에이전트의 컨텍스트 크기). 실행 전체에 걸친 합계가 아닙니다.
* `total_tool_uses`: 하위 에이전트가 전체 실행에 걸쳐 수행한 도구 호출 수
* `duration_ms`: 밀리초 단위의 실행 시간
* `model`: 하위 에이전트가 실행하도록 해결된 모델
* `final_model`: 하위 에이전트의 최종 응답을 생성한 모델(폴백과 같은 중간 실행 전환 후 `model`과 다름). Claude Code v2.1.212 이상 필요
* `model_swapped`: 둘 이상의 모델이 하위 에이전트의 요청을 제공했는지 여부. Claude Code v2.1.212 이상 필요
* `plugin_id_hash`, `plugin.name`: 플러그인 제공 에이전트에 대해 있습니다. 공식 마켓플레이스 플러그인 이름은 그대로 나타납니다; 다른 플러그인 이름은 `OTEL_LOG_TOOL_DETAILS=1`이 설정되지 않은 경우 `"third-party"`로 대체됩니다.

<h4 id="feedback-survey-event">
  피드백 설문 이벤트
</h4>

세션 품질 설문이 표시되거나 답변될 때 기록됩니다. 설문이 수집하는 것과 제어 방법에 대해서는 [세션 품질 설문](/docs/ko/data-usage#session-quality-surveys)을 참조하세요.

**이벤트 이름**: `claude_code.feedback_survey`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"feedback_survey"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `event_type`: 설문 수명 주기 이벤트(예: `"appeared"`, `"responded"`, 또는 `"transcript_prompt_appeared"`)
* `appearance_id`: 하나의 설문 인스턴스에 대해 내보낸 이벤트를 연결하는 고유 ID
* `survey_type`: 이벤트를 생성한 설문. `"session"`은 "Claude가 어떻게 하고 있나요?" 평가 프롬프트입니다.
* `response`: `responded` 이벤트의 사용자 선택
* `enabled_via_override`: [`CLAUDE_CODE_ENABLE_FEEDBACK_SURVEY_FOR_OTEL`](/docs/ko/env-vars)이 설정되었을 때 `true`. 부울로 내보내집니다(문자열이 아님). `session` 설문 이벤트에 있습니다. 이 속성을 필터링하여 플릿 전체에서 재정의가 적용되었는지 확인합니다.

<h4 id="retention-sweep-event">
  보존 스윕 이벤트
</h4>

보존 정리 스윕 실행당 한 번 기록됩니다(이는 [`cleanupPeriodDays`](/docs/ko/settings-reference#cleanupperioddays) 설정보다 오래된 [세션 기록 및 기타 애플리케이션 데이터](/docs/ko/claude-directory#cleaned-up-automatically)를 삭제합니다). Claude Code는 백그라운드에서 세션당 최대 한 번 스윕을 실행하며, 아무것도 삭제하지 않는 실행도 이벤트를 내보냅니다. Claude Code가 지난 24시간 동안 동일한 머신의 모든 세션에서 스윕을 실행했으면 이 세션의 스윕을 최소 10분 이상 지연하므로 더 빨리 종료되는 세션은 아무것도 내보내지 않습니다. `claude -p`를 `--bare`로 실행하면 Claude Code는 스윕을 실행하지 않고 아무것도 내보내지 않습니다.

이 페이지의 모든 OTel 이벤트처럼 구성한 원격 측정 백엔드로만 이동합니다. Claude Code v2.1.227 이상 필요.

Claude Code가 보존 기간을 안전하게 결정할 수 없으면 스윕을 일시 중지하고 `result`를 `"skipped"`로 설정하고 `skip_reason`을 포함하는 이벤트를 내보냅니다. [관리 설정](/docs/ko/server-managed-settings)이 `cleanupPeriodDays`를 설정하면 관리 값이 보존 기간을 고정하고 낮은 우선순위 범위의 설정 파일이 손상되거나 유효하지 않아도 스윕이 실행됩니다. `managed-settings.json` 자체를 읽을 수 없으면 Claude Code는 [관리 계층](/docs/ko/managed-settings#how-claude-code-combines-managed-sources)이 서버 관리 설정과 같은 다른 곳에서 `cleanupPeriodDays`를 제공하지 않는 한 스윕을 일시 중지합니다. 또는 손상된 파일 옆의 `managed-settings.d/` 드롭인입니다. 삭제 카운터 속성은 `result`가 `"complete"`일 때만 있습니다.

**이벤트 이름**: `claude_code.retention_sweep`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"retention_sweep"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `result`: 스윕이 실행되었을 때 `"complete"`, Claude Code가 일시 중지했을 때 `"skipped"`
* `period_days`: 병합된 설정의 `cleanupPeriodDays` 값(일 단위) 또는 소스가 설정하지 않을 때 `30`. 건너뛴 이벤트에서 스윕이 사용했을 값(Claude Code가 읽을 수 있는 설정 소스에서 계산됨)
* `used_default`: 읽을 수 있는 설정 소스가 `cleanupPeriodDays`를 설정하지 않을 때 `"true"`, 그 외에는 `"false"`. 완료 이벤트에서 `"true"`는 30일 기본값이 적용되었음을 의미합니다.
* `skip_reason`: Claude Code가 스윕을 일시 중지한 이유. `result`가 `"skipped"`일 때만 있습니다:
  * `"user_source_disabled"`: 사용자 설정이 제외됨(예: [`--setting-sources`](/docs/ko/cli-reference#cli-flags) 플래그 또는 SDK의 [`settingSources`](/docs/ko/agent-sdk/typescript#options) 옵션에 의해) 그리고 활성화된 소스가 `cleanupPeriodDays`를 제공하지 않습니다.
  * `"settings_unknowable"`: 설정 파일을 읽거나 구문 분석할 수 없어 `cleanupPeriodDays` 또는 `desktopSessionCleanupPeriodDays`가 Claude Code가 볼 수 없는 값으로 설정될 수 있습니다.
  * `"settings_invalid_key_set"`: 설정에 유효성 검사 오류가 있고 `cleanupPeriodDays` 또는 `desktopSessionCleanupPeriodDays`가 명시적으로 설정되어 있어 기본값으로 폴백하면 해당 설정에 대해 파일을 삭제하거나 유지할 수 있습니다.
* `transcripts_deleted`: 스윕이 삭제한 세션 기록(최상위 `~/.claude/projects/*/*.jsonl` 파일) 수
*

`transcripts_exempted_desktop`: 보존 기간을 지났지만 스윕이 [Claude Desktop 및 Cowork 규칙](/docs/ko/claude-directory#cleaned-up-automatically)에 따라 유지한 기록 수. 이들은 `files_past_cutoff`에 계산되지 않습니다. Claude Code v2.1.248 이상 필요

* `session_files_deleted`: 세션 파일 스윕이 삭제한 항목 수: 기록 및 사이드카, 녹음, 및 도구 결과와 같은 세션별 동반 파일
* `artifacts_deleted`: 데이터 디렉토리 전체에서 스윕이 삭제한 총 항목(세션 파일 포함). 일부 스윕은 제거된 전체 디렉토리 트리를 하나의 항목으로 계산하고 몇 가지 정리 통과는 카운터에 기여하지 않으므로 값을 정확한 파일 수보다는 하한으로 취급합니다.
* `files_retained_fresh`: 검사되었으며 보존 기간 내에 있기 때문에 제자리에 남겨진 파일. 파일별 스윕만 이들을 계산하므로 값은 하한입니다; 0이 아닌 값은 정상적인 정상 상태입니다.
* `files_past_cutoff`: 보존 기간보다 오래되었지만 스윕이 삭제하지 못한 파일(예: 권한 오류 또는 열린 파일). 0보다 큰 값은 파일이 구성된 보존 기간을 초과했음을 의미합니다; 0은 없었다는 증거가 아닙니다(전체 디렉토리 제거 실패는 대신 `error_count`에 계산되기 때문).
* `error_count`: 스윕이 파일을 나열하거나 삭제하는 동안 발생한 오류 수

<h4 id="managed-settings-resolved-event">
  관리 설정 해결 이벤트
</h4>

세션이 해결한 [관리 설정](/docs/ko/managed-settings)으로 기록됩니다: 세션 시작 시 한 번, 관리 설정 또는 [정책 도우미](/docs/ko/managed-settings#compute-the-policy-with-a-helper-program)의 상태가 세션 중에 변경될 때 다시, 그리고 Claude Code가 `error.type` 속성이 나열하는 이유 중 하나로 시작을 거부하거나 세션을 종료할 때.
이 이벤트를 사용하여 예상치 못한 관리 소스에서 실행 중인 머신, 정책 도우미가 실패하는 머신, 및 머신이 시작을 거부한 이유를 찾습니다.
Claude Code v2.1.274 이상 필요.

기본적으로 이벤트는 관리 소스 및 정책 도우미의 상태를 전달하지만 설정 자체는 전달하지 않습니다. 삭제된 `managed_settings.settings` 속성 및 `managed_settings.resolved_sha256` 다이제스트를 추가하려면 `OTEL_LOG_MANAGED_SETTINGS=1`을 설정합니다:

* 관리 설정, 사용자 설정, 또는 `--settings`의 `env` 블록에서 또는 Claude Code를 시작하는 환경에서 설정합니다. 프로젝트 또는 로컬 설정의 값은 복제된 저장소가 이들을 쓸 수 있기 때문에 켜지 않습니다.
* 서버 관리 설정은 변수가 조직이 이미 받는 이벤트에 조직의 자체 삭제된 정책만 추가하기 때문에 [보안 승인 대화](/docs/ko/server-managed-settings#security-approval-dialogs)를 표시하지 않고 설정할 수 있습니다.

신뢰하지 않은 폴더에서 대화형 세션에서 Claude Code는 거부 이벤트를 내보내지 않습니다([신뢰](/docs/ko/permissions#what-runs-before-you-trust-a-folder)하지 않은 폴더). 프로젝트 및 로컬 설정이 내보내기를 다른 수집기로 가리킬 수 있기 때문입니다.

**이벤트 이름**: `claude_code.managed_settings_resolved`

**속성**:

* 모든 [표준 속성](#standard-attributes)
* `event.name`: `"managed_settings_resolved"`
* `event.timestamp`: ISO 8601 타임스탬프
* `event.sequence`: 이벤트 순서 지정을 위한 프로세스별 카운터([이벤트 상관 속성](#event-correlation-attributes)에서 설명)
* `managed_settings.trigger`: 세션 시작 이벤트의 경우 `"startup"`, 관리 설정 또는 정책 도우미의 상태가 세션 중에 변경되었을 때 `"change"`, 또는 관리 설정 정책이 세션을 중지했을 때 `"refused"`. Claude Code는 마지막 이벤트와 다른 속성이 있을 때만 `change` 이벤트를 보내며, 변경된 설정 값은 `OTEL_LOG_MANAGED_SETTINGS`가 꺼져 있어도 계산됩니다.
* `error.type`: Claude Code가 세션을 중지한 이유. `refused` 이벤트에만 있습니다:
  * `"helper_failed"`: [정책 도우미 실행이 실패했습니다](/docs/ko/settings-reference#helper-failures).
  * `"policy_invalid"`: 관리 설정에 Claude Code가 시작하지 못하게 하는 오류가 있거나 관리 소스가 로드되지 못해 Claude Code가 조직 로그인 적용을 확인할 수 없습니다.
  * `"consent_rejected"`: 사용자가 서버 관리 설정의 [보안 승인 대화](/docs/ko/server-managed-settings#security-approval-dialogs)를 거부했습니다.
  * `"force_refresh_failed"`: [`forceRemoteSettingsRefresh`](/docs/ko/settings-reference#forceremotesettingsrefresh)가 필요로 하는 설정 가져오기가 실패했습니다.
  * `"gateway_rejected"`: [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway)가 관리 설정 로드에 HTTP 403으로 응답했습니다.
  * `"version_below_minimum"`: 이 Claude Code 버전이 [`requiredMinimumVersion`](/docs/ko/settings-reference#requiredminimumversion) 아래이거나 [`requiredMaximumVersion`](/docs/ko/settings-reference#requiredmaximumversion) 위입니다.
  * `"_OTHER"`: Claude 앱 게이트웨이 관리 설정 로드가 다른 이유로 실패했습니다.
* `managed_settings.sources`: 최소한 하나의 [정책 키](/docs/ko/managed-settings#how-claude-code-combines-managed-sources)를 전달하는 모든 관리 소스(우선순위 순서대로, `first-wins`에서 효과를 갖지 않는 소스 포함). 값은 `"remote"`, MDM 또는 OS 수준 정책의 경우 `"plist"` 또는 `"hklm"`, 관리 설정 파일 및 드롭인의 경우 `"file"`, [포함 호스트](/docs/ko/managed-settings#let-an-embedding-host-add-policy)가 설정을 제공할 때 `"parent"`, 그리고 Claude Code가 [읽을 때](/docs/ko/managed-settings#how-claude-code-combines-managed-sources) [Windows HKCU 레지스트리 값](/docs/ko/managed-settings#where-each-mechanism-stores-the-policy)의 경우 `"hkcu"`입니다. 제어 키만 전달하거나 Claude Code가 읽을 수 없는 소스는 나열되지 않습니다. 문자열 배열로 내보내집니다. 관리 소스가 정책 키를 전달하지 않을 때 비어 있습니다.
* `managed_settings.source_behavior`: Claude Code가 읽은 [`managedSourcesBehavior`](/docs/ko/settings-reference#managedsourcesbehavior) 값(`"first-wins"` 또는 `"merge"`). 소스가 키를 설정하지 않을 때 `"first-wins"`
* `managed_settings.helper.state`: 선택한 MDM 또는 파일 소스가 구성하는 정책 도우미의 상태:
  * `"ok"`: 도우미의 출력이 관리 설정으로 제공됩니다.
  * `"bad_path"`, `"not_a_file"`, `"exit_nonzero"`, `"timed_out"`, `"oversize"`, `"parse_failed"`, `"envelope_invalid"`, 또는 `"schema_rejected"`: 도우미의 마지막 실행이 실패했습니다. [도우미 실패](/docs/ko/settings-reference#helper-failures)가 경우를 설명합니다.
  * `"none"`: 도우미가 구성되지 않았거나 이를 구성하는 소스가 MDM 정책 또는 관리 설정 파일이 아닙니다.
* `managed_settings.helper.applied`: 도우미의 자체 출력이 관리 설정으로 제공될 때 `"output"`, 그렇지 않을 때 `"none"`
* `managed_settings.helper.entry`: Claude Code가 [`policyHelper`](/docs/ko/settings-reference#policyhelper)를 선택했을 때 `"policyHelper"`. 도우미를 선택하지 않았을 때는 없습니다.
* `managed_settings.helper.path`: 도우미의 구성된 [`path`](/docs/ko/settings-reference#policyhelper-path). Claude Code가 도우미를 선택했을 때마다 있습니다(도우미가 작동하는지 여부와 관계없이).
* `managed_settings.resolved_sha256` (`OTEL_LOG_MANAGED_SETTINGS=1`일 때): 삭제 전 해결된 관리 설정의 SHA-256(JSON으로 직렬화되고 키가 재귀적으로 정렬되고 공백이 없음). 동일한 다이제스트를 가진 머신은 동일한 정책을 실행합니다. Claude Code는 짧은 정책을 추측 해싱으로 복구할 수 있기 때문에 옵트인으로만 다이제스트를 보냅니다. 관리 설정이 해결되지 않았을 때는 없으며, `refused` 이벤트에는 없습니다.
* `managed_settings.settings` (`OTEL_LOG_MANAGED_SETTINGS=1`일 때): 해결된 관리 설정의 이름 및 형태(JSON 문자열)이며 값은 삭제됩니다. `refused` 이벤트에는 없습니다. Claude Code는 설정 스키마에서 이를 구축합니다:

  * 스키마가 내보내기를 선언하는 설정 이름이고, 스키마가 선언하지 않는 키는 생략됩니다.
  * 부울, 숫자, 및 스키마가 `permissions.defaultMode`와 같은 고정 옵션 집합으로 제한하는 문자열 값은 그대로 내보내집니다. `sandbox.network.httpProxyPort` 및 `sandbox.network.socksProxyPort`는 `"[REDACTED]"`로 내보내집니다.
  * 모든 다른 문자열(예: `model`, `apiKeyHelper`, 모든 `env` 값, 모든 URL, 및 모든 명령)은 `"[REDACTED]"`로 내보내집니다.
  * 맵의 항목 이름(예: `env` 변수 이름 및 플러그인 ID)은 그대로 내보내집니다. 스키마가 항목을 입력하지 않는 설정(예: `vimInsertModeRemaps`)은 단일 `"[REDACTED]"`로 내보내지며, `sandbox.ignoreViolations`는 명령 패턴 없이 경로 목록 목록으로 내보내집니다.
  * 목록은 길이를 유지하며 각 항목은 동일한 규칙으로 삭제됩니다.
  * `permissions.allow`, `permissions.deny`, 또는 `permissions.ask` 규칙은 도구 이름(이 Claude Code 버전에 기본 제공되거나 `mcp__jira__create_issue`와 같은 `mcp__` 참조인 경우)으로 내보내지며 콘텐츠는 삭제됩니다(예: `Read([REDACTED])`). 다른 규칙은 `"[REDACTED]"`로 내보내집니다.
  * 훅은 동일한 규칙을 따르므로 `type` 및 `timeout`과 같은 고정 옵션 및 숫자 필드는 표시되는 반면 각 명령, URL, `matcher`, 및 `if` 조건은 `"[REDACTED]"`로 내보내집니다.

  예를 들어 `apiKeyHelper`, 두 개의 `env` 변수, 및 거부 규칙이 있는 관리 설정은 `{"apiKeyHelper":"[REDACTED]","env":{"HTTPS_PROXY":"[REDACTED]","CLAUDE_CODE_ENABLE_TELEMETRY":"[REDACTED]"},"permissions":{"deny":["Read([REDACTED])"]}}`.

  Claude Code는 값을 UTF-8의 8KB에서 자르며, 자른 값은 유효한 JSON이 아닙니다.
* `managed_settings.settings_truncated` (`managed_settings.settings`가 있을 때): Claude Code가 `managed_settings.settings`를 8KB에서 자를 때 `true`, 그 외에는 `false`. 부울로 내보내집니다(문자열이 아님).

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

Claude Code는 `ANTHROPIC_BASE_URL` 뒤의 게이트웨이 또는 프록시가 여러 프레임에 걸쳐 사용량을 점진적으로 스트리밍할 때를 포함하여 각 스트리밍 응답을 비용 및 토큰 메트릭에 정확히 한 번 계산합니다. v2.1.214 이전에는 둘 이상의 프레임에서 사용량을 전달한 스트림이 추가 프레임당 대략 하나의 추가 전체 요청으로 `claude_code.cost.usage` 및 `claude_code.token.usage`를 부풀렸습니다.

<h3 id="alerting-and-segmentation">
  경고 및 세분화
</h3>

일반적인 경고 고려 사항:

* 비용 급증
* 비정상적인 토큰 소비
* 특정 사용자의 높은 세션 볼륨

모든 메트릭은 [표준 속성](#standard-attributes)으로 세분화할 수 있습니다. `model` 속성은 `claude_code.token.usage`, `claude_code.cost.usage`에서 사용 가능하며, v2.1.172부터 `claude_code.lines_of_code.count`에서도 사용 가능합니다.

커밋의 모델별 분류는 한 세션이 여러 모델에 걸쳐 있을 수 있으므로 `session.id`에서 토큰 또는 비용 메트릭에 대해 조인하여만 근사할 수 있습니다. 토큰 또는 비용 측면을 `query_source`가 `"main"`인 행으로 필터링하여 보조 및 서브에이전트 요청이 세션의 커밋을 해당 요청을 수행하지 않은 모델에 귀속시키지 않도록 합니다.

<h3 id="detect-retry-exhaustion">
  재시도 소진 감지
</h3>

Claude Code는 실패한 API 요청을 내부적으로 재시도하고 포기한 후에만 단일 `claude_code.api_error` 이벤트를 내보내므로 이벤트 자체가 해당 요청의 최종 신호입니다. 중간 재시도 시도는 별도의 이벤트로 기록되지 않습니다.

이벤트의 `attempt` 속성은 총 시도 횟수를 기록합니다. `CLAUDE_CODE_MAX_RETRIES`는 기본값이 10이고 최대 15입니다. v2.1.199 이상에서는 `CLAUDE_CODE_RETRY_WATCHDOG`을 설정하여 기본값을 높이고 상한을 제거할 수 있습니다.

요청이 일시적 오류에 대한 모든 재시도를 소진하면 `attempt`는 해당 유효 제한보다 하나 많습니다: 기본값으로는 11이고 감시 기능이 설정되지 않은 경우 16을 초과하지 않습니다. 더 낮은 값은 `400` 응답과 같은 재시도 불가능한 오류를 나타내거나 자체 더 작은 재시도 예산이 있는 원인을 나타냅니다. 예를 들어 Claude Code는 AWS 또는 Google Cloud 자격 증명 로드 실패를 최대 두 번 재시도합니다.

복구된 세션과 정체된 세션을 구분하려면 `session.id`로 이벤트를 그룹화하고 오류 후 나중에 `api_request` 이벤트가 존재하는지 확인합니다.

<h3 id="event-analysis">
  이벤트 분석
</h3>

이벤트 데이터는 Claude Code 상호 작용에 대한 자세한 정보를 제공합니다:

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

각 이벤트의 [표준 속성](#standard-attributes)에는 인증된 사용자의 ID가 포함됩니다: Claude 계정으로 로그인할 때 `user.email`, `user.account_uuid`, `user.account_id` 및 `organization.id`, [클라우드 세션](/docs/ko/claude-code-on-the-web)에서 세션 자체의 자격 증명이 이들을 전달할 때, 그리고 설치 범위 `user.id` 및 세션별 `session.id`. `user.id`는 설치 범위 식별자이며, [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway) 세션에서는 게이트웨이 발급 토큰의 IdP 주체입니다.

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

* `tool_result`: `mcp_server_scope`를 유지하고 사용자 구성 서버의 경우 `tool_name`을 리터럴 `"mcp_tool"`로 수정하며, 인수 내용을 생략합니다. Claude Desktop의 기본 제공 서버의 경우, Claude Desktop이 소유한 세션에서는 `tool_parameters` 내부의 `mcp_server_name`/`mcp_tool_name` 쌍도 유지하며, `tool_decision`과 동일한 호스트 작성 예외입니다. Claude Code v2.1.214 이상 필요
* `tool_decision`: `tool_source`를 유지하고 사용자 구성 서버의 경우 `tool_name`을 리터럴 `"mcp_tool"`로 수정하며, 인수 내용을 생략합니다. Claude Desktop의 기본 제공 서버의 경우, Claude Desktop이 소유한 세션에서는 `tool_parameters` 내부의 `mcp_server_name`/`mcp_tool_name` 쌍도 유지합니다. `tool_source` 및 이름 쌍 모두 Claude Code v2.1.214 이상 필요
* `mcp_server_connection`: `server_name` 및 오류 메시지를 생략하지만, `is_plugin`, `plugin_id_hash` 및 `plugin.name`을 유지하며, Anthropic이 아닌 플러그인 이름은 리터럴 `"third-party"`로 수정되므로 플러그인 제공 서버는 상세 로깅 없이도 구별 가능합니다

<h3 id="map-security-questions-to-events">
  보안 질문을 이벤트에 매핑
</h3>

감지 규칙을 구축할 때 모니터링하려는 신호를 찾고 해당 이벤트 및 속성에 대해 백엔드를 쿼리합니다:

| 신호                                             | 이벤트                                                                         | 주요 속성                                                                                                                                                                                                                                          |
| ---------------------------------------------- | --------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 도구 호출 허용 또는 거부, 그리고 어떻게                        | `tool_decision`                                                             | `decision`, `source`, `tool_name`, `tool_parameters`                                                                                                                                                                                           |
| 권한 모드 에스컬레이션                                   | `permission_mode_changed`                                                   | `from_mode`, `to_mode`, `trigger`                                                                                                                                                                                                              |
| 정책 훅이 작업을 차단함                                  | `hook_execution_complete`                                                   | `hook_event`, `num_blocking`                                                                                                                                                                                                                   |
| 로그인, 로그아웃 및 인증 실패                              | `auth`                                                                      | `action`, `success`, `error_category`                                                                                                                                                                                                          |
| MCP 서버 연결 또는 실패                                | `mcp_server_connection`                                                     | `status`, `server_name`, `is_plugin`, `error_code`                                                                                                                                                                                             |
| 플러그인 설치 및 출처                                   | `plugin_installed`                                                          | `plugin.name`, `marketplace.name`, `marketplace.is_official`                                                                                                                                                                                   |
| 실행된 명령 및 터치된 파일                                | `tool_result` (실행됨) 또는 `tool_decision` (거부됨) (`OTEL_LOG_TOOL_DETAILS=1` 포함) | `tool_parameters`; `tool_input` (`tool_result`만 해당)                                                                                                                                                                                            |
| 머신이 실행되는 관리 설정 소스, 정책 도우미의 상태 및 머신이 시작을 거부한 이유 | `managed_settings_resolved`                                                 | `managed_settings.trigger`, `managed_settings.sources`, `managed_settings.source_behavior`, `managed_settings.helper.state`, `error.type`; `managed_settings.settings` 및 `managed_settings.resolved_sha256` (`OTEL_LOG_MANAGED_SETTINGS=1` 포함) |

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

이벤트가 도착하는지 확인하려면 이 구성에서 실행 중인 세션에서 프롬프트를 제출하고 `claude_code.user_prompt` 이벤트에 대해 SIEM을 확인합니다. 아무것도 도착하지 않으면 `claude --debug`를 실행하고 `[3P telemetry]` 내보내기 오류에 대해 디버그 로그를 확인합니다.

<h2 id="backend-considerations">
  백엔드 고려 사항
</h2>

메트릭, 로그 및 추적 백엔드 선택은 수행할 수 있는 분석 유형을 결정합니다:

<h3 id="for-metrics">
  메트릭의 경우
</h3>

* **시계열 데이터베이스**: 비율 계산, 집계된 메트릭
* **컬럼형 저장소**: 복잡한 쿼리, 고유 사용자 분석
* **완전한 기능의 관찰성 플랫폼**: 고급 쿼리, 시각화, 경고

<h3 id="for-events/logs">
  이벤트/로그의 경우
</h3>

* **로그 집계 시스템**: 전체 텍스트 검색, 로그 분석
* **컬럼형 저장소**: 구조화된 이벤트 분석
* **완전한 기능의 관찰성 플랫폼**: 메트릭과 이벤트 간의 상관 관계

<h3 id="for-traces">
  추적의 경우
</h3>

분산 추적 저장소 및 스팬 상관 관계를 지원하는 백엔드를 선택합니다:

* **분산 추적 시스템**: 스팬 시각화, 요청 워터폴, 지연 시간 분석
* **완전한 기능의 관찰성 플랫폼**: 추적 검색 및 메트릭과 로그와의 상관 관계

일일/주간/월간 활성 사용자 (DAU/WAU/MAU) 메트릭이 필요한 조직의 경우 효율적인 고유 값 쿼리를 지원하는 백엔드를 고려하세요.

<h2 id="service-information">
  서비스 정보
</h2>

모든 메트릭 및 이벤트는 다음 리소스 속성과 함께 내보내집니다:

* `service.name`: 터미널 세션의 경우 `claude-code`, [Claude Desktop 앱](/docs/ko/desktop)의 Code 탭에서 시작된 세션의 경우 `claude-code-desktop`
* `service.version`: 현재 Claude Code 버전, 또는 Code 탭 세션의 경우 Desktop 앱 버전
* `os.type`: 운영 체제 유형 (예: `linux`, `darwin`, `windows`)
* `os.version`: 운영 체제 버전 문자열
* `host.arch`: 호스트 아키텍처 (예: `amd64`, `arm64`)
* `wsl.version`: WSL 버전 번호 (Windows Subsystem for Linux에서 실행할 때만 표시)
* 미터 이름: `com.anthropic.claude_code`

수집기 파이프라인 또는 대시보드에서 `service.name = claude-code`로 필터링하는 경우, Code 탭 세션의 원격 분석도 캡처하기 위해 필터에 `claude-code-desktop`을 추가하십시오.

<h2 id="roi-measurement-resources">
  ROI 측정 리소스
</h2>

Claude Code의 투자 수익률 측정에 대한 포괄적인 가이드(원격 측정 설정, 비용 분석, 생산성 메트릭 및 자동화된 보고 포함)는 [Claude Code ROI 측정 가이드](https://github.com/anthropics/claude-code-monitoring-guide)를 참조하세요. 이 저장소는 즉시 사용 가능한 Docker Compose 구성, Prometheus 및 OpenTelemetry 설정, Linear와 같은 도구와 통합된 생산성 보고서 생성 템플릿을 제공합니다.

<h2 id="security-and-privacy">
  보안 및 개인 정보 보호
</h2>

* OpenTelemetry 내보내기는 선택 사항이며 명시적 구성이 필요합니다. Anthropic의 별도 운영 원격 측정 및 이를 비활성화하는 방법에 대해서는 [데이터 사용](/docs/ko/data-usage#telemetry-services)을 참조하세요
* 원본 파일 콘텐츠 및 코드 스니펫은 메트릭 또는 이벤트에 포함되지 않습니다. 추적 스팬은 별도의 데이터 경로입니다: 아래의 `OTEL_LOG_TOOL_CONTENT` 항목을 참조하세요
* OAuth를 통해 인증된 경우 `user.email`이 원격 측정 속성에 포함되며, 구성한 OTel 엔드포인트로만 전송되고 Anthropic으로는 절대 전송되지 않습니다. 조직에서 이것이 우려 사항인 경우 원격 측정 백엔드와 함께 작업하여 이 필드를 필터링하거나 수정하세요
* 사용자 프롬프트 콘텐츠는 기본적으로 수집되지 않습니다. 프롬프트 길이만 기록됩니다. 프롬프트 콘텐츠를 포함하려면 `OTEL_LOG_USER_PROMPTS=1`을 설정하세요. 상세 베타 추적에서 이 변수는 프롬프트 텍스트보다 더 멀리 도달합니다: 또한 `claude_code.llm_request` 스팬의 도구 결과를 전달하는 [`new_context` 스팬 속성](#new-context-gates)을 제어합니다
* 어시스턴트 응답 텍스트는 기본적으로 수집되지 않습니다. 응답 길이만 기록됩니다. 응답 텍스트를 포함하려면 `OTEL_LOG_ASSISTANT_RESPONSES=1`을 설정하세요. Claude Code의 모든 OpenTelemetry 데이터와 마찬가지로 응답 텍스트는 구성한 OTel 엔드포인트로만 전송되며 Anthropic으로는 전송되지 않습니다. 이 변수가 설정되지 않으면 `OTEL_LOG_USER_PROMPTS`가 폴백으로 사용되므로 프롬프트 콘텐츠는 원하지만 응답 콘텐츠는 원하지 않는 경우 `OTEL_LOG_ASSISTANT_RESPONSES=0`을 설정하세요
* 도구 입력 인수 및 매개변수는 기본적으로 기록되지 않습니다. 이를 포함하려면 `OTEL_LOG_TOOL_DETAILS=1`을 설정하세요. Claude Desktop의 기본 제공 서버의 경우, Claude Desktop이 소유한 세션에서 `tool_decision` 및 `tool_result`는 인수 콘텐츠가 아닌 호스트 작성 이름인 `mcp_server_name`/`mcp_tool_name` 쌍을 전달하며, 플래그가 꺼져 있어도 그렇습니다. 이 예외는 Claude Code v2.1.214 이상이 필요합니다. 이 데이터는 구성한 OTEL 엔드포인트로만 전송되며 Anthropic으로는 절대 전송되지 않습니다. 인수에는 여전히 민감한 값이 포함될 수 있으므로 필요에 따라 이러한 속성을 필터링하거나 수정하도록 원격 측정 백엔드를 구성하세요. 활성화되면:
  * `tool_result` 및 `tool_decision` 이벤트는 Bash 명령, MCP 서버 및 도구 이름, 스킬 이름이 포함된 `tool_parameters` 속성을 포함합니다. `full_command`와 같은 필드는 잘리지 않은 상태로 내보내집니다
  * `tool_result` 이벤트는 추가로 파일 경로, URL, 검색 패턴 및 기타 인수가 포함된 `tool_input` 속성을 포함합니다. 512자를 초과하는 개별 값은 잘리고 전체는 약 4K 문자로 제한됩니다
  * `user_prompt` 이벤트는 사용자 정의, 플러그인 및 MCP 명령의 축자 `command_name`을 포함합니다
  * 추적 스팬은 동일한 `tool_input` 속성 및 `file_path`와 같은 입력 파생 속성을 포함하며, `tool_input`과 동일한 잘림이 적용됩니다
* 도구 콘텐츠는 기본적으로 추적 스팬에 기록되지 않습니다. 이를 포함하려면 `OTEL_LOG_TOOL_CONTENT=1`을 설정하세요. 그러면 `claude_code.tool` 스팬은 원본 파일 콘텐츠 및 Bash 명령 출력이 포함된 [`tool.output` 스팬 이벤트](#tool-output-span-event)를 전달하며, 속성당 콘텐츠 제한(기본값 60KB)에서 잘립니다. 도구 콘텐츠는 또한 [`new_context`를 통해 스팬에 도달하며, 그 제어는 스팬마다 다릅니다](#new-context-gates). 필요에 따라 이러한 속성을 필터링하거나 수정하도록 원격 측정 백엔드를 구성하세요
* 원본 Anthropic Messages API 요청 및 응답 본문은 기본적으로 기록되지 않습니다. 이를 포함하려면 셸, 사용자 설정 또는 관리 설정에서 `OTEL_LOG_RAW_API_BODIES`를 설정하세요. [프로젝트 및 로컬 설정](/docs/ko/settings-reference#variables-claude-code-ignores-in-env)에서는 무시됩니다. 본문에는 전체 대화 기록(시스템 프롬프트, 모든 이전 사용자 및 어시스턴트 턴, 도구 결과)이 포함되므로 이를 활성화하면 다른 `OTEL_LOG_*` 콘텐츠 플래그가 공개할 모든 것에 동의하는 것을 의미합니다. Claude Code는 다른 설정에 관계없이 항상 이러한 본문에서 Claude의 확장 사고 콘텐츠를 수정합니다. 설정한 값은 Claude Code가 본문을 전달하는 방식을 결정합니다:
  * `=1`일 때 Claude Code는 각 API 호출에 대해 `api_request_body` 및 `api_response_body` 로그 이벤트를 내보냅니다. 이벤트의 `body` 속성은 JSON 직렬화된 페이로드를 전달하며, 콘텐츠 제한(기본값 60KB)에서 잘립니다
  * `=file:<dir>`일 때 Claude Code는 잘리지 않은 본문을 해당 디렉토리 아래의 `.request.json` 및 `.response.json` 파일에 기록하고, 이벤트는 인라인 본문 대신 `body_ref` 경로를 전달합니다. 로그 수집기 또는 사이드카와 함께 디렉토리를 배포하되 원격 측정 스트림을 통해서는 배포하지 마세요.

    각 성공적인 응답에 대해 Claude Code는 또한 해당 디렉토리의 `index.jsonl`에 한 줄을 추가하여 응답 파일을 이를 생성한 요청 파일 및 이것이 된 트랜스크립트 메시지에 연결합니다. 각 줄은 메시지 콘텐츠를 포함하지 않으며, [API 응답 본문 이벤트](#api-response-body-event) 섹션에 해당 필드가 나열됩니다. 인덱스 파일은 Claude Code v2.1.274 이상이 필요합니다

<h2 id="monitor-claude-code-on-amazon-bedrock">
  Amazon Bedrock에서 Claude Code 모니터링
</h2>

Amazon Bedrock의 Claude Code 사용 모니터링에 대한 자세한 지침은 [Claude Code 모니터링 구현 (Amazon Bedrock)](https://github.com/aws-solutions-library-samples/guidance-for-claude-code-with-amazon-bedrock/blob/main/assets/docs/MONITORING.md)을 참조하세요.
