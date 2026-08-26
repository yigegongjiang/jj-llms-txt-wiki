> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# OpenTelemetry를 통한 관찰성

> Agent SDK에서 OpenTelemetry를 사용하여 추적, 메트릭 및 이벤트를 관찰성 백엔드로 내보냅니다.

프로덕션 환경에서 에이전트를 실행할 때는 에이전트가 수행한 작업을 파악해야 합니다:

* 호출한 도구
* 각 모델 요청에 소요된 시간
* 사용한 토큰 수
* 오류 발생 위치

Agent SDK는 이 데이터를 OpenTelemetry 추적, 메트릭 및 로그 이벤트로 OpenTelemetry Protocol(OTLP)을 지원하는 모든 백엔드(예: Honeycomb, Datadog, Grafana, Langfuse 또는 자체 호스팅 수집기)로 내보낼 수 있습니다.

이 가이드에서는 SDK가 텔레메트리를 내보내는 방식, 내보내기를 구성하는 방법, 데이터가 백엔드에 도달한 후 태그를 지정하고 필터링하는 방법을 설명합니다. 백엔드로 내보내는 대신 SDK 응답 스트림에서 직접 토큰 사용량 및 비용을 읽으려면 [비용 및 사용량 추적](/docs/ko/agent-sdk/cost-tracking)을 참조하세요.

<h2 id="how-telemetry-flows-from-the-sdk">
  SDK에서 텔레메트리가 흐르는 방식
</h2>

Agent SDK는 Claude Code CLI를 자식 프로세스로 실행하고 로컬 파이프를 통해 통신합니다. CLI에는 OpenTelemetry 계측이 내장되어 있습니다. 각 모델 요청과 도구 실행 주위에 스팬을 기록하고, 토큰 및 비용 카운터에 대한 메트릭을 내보내며, 프롬프트 및 도구 결과에 대한 구조화된 로그 이벤트를 내보냅니다. SDK는 자체 텔레메트리를 생성하지 않습니다. 대신 구성을 CLI 프로세스로 전달하고, CLI가 수집기로 직접 내보냅니다.

구성은 환경 변수로 전달됩니다. 기본적으로 자식 프로세스는 애플리케이션의 환경을 상속하므로 다음 두 위치 중 하나에서 텔레메트리를 구성할 수 있습니다:

* **프로세스 환경:** 애플리케이션이 시작되기 전에 셸, 컨테이너 또는 오케스트레이터에서 변수를 설정합니다. 모든 `query()` 호출이 코드 변경 없이 자동으로 이를 선택합니다. 이것이 프로덕션 배포에 권장되는 방식입니다.
* **호출별 옵션:** `ClaudeAgentOptions.env`(Python) 또는 `options.env`(TypeScript)에서 변수를 설정합니다. 같은 프로세스의 다른 에이전트가 다른 텔레메트리 설정이 필요할 때 사용합니다. Python에서 `env`는 상속된 환경 위에 병합됩니다. TypeScript에서 `env`는 상속된 환경을 완전히 대체하므로 전달하는 객체에 `...process.env`를 포함하세요.

CLI는 세 가지 독립적인 OpenTelemetry 신호를 내보냅니다. 각각 자체 활성화 스위치와 자체 내보내기 도구가 있으므로 필요한 것만 켤 수 있습니다.

| 신호     | 포함 내용                                       | 활성화 방법                                                           |
| ------ | ------------------------------------------- | ---------------------------------------------------------------- |
| 메트릭    | 토큰, 비용, 세션, 코드 라인 및 도구 결정에 대한 카운터           | `OTEL_METRICS_EXPORTER`                                          |
| 로그 이벤트 | 각 프롬프트, API 요청, API 오류 및 도구 결과에 대한 구조화된 레코드 | `OTEL_LOGS_EXPORTER`                                             |
| 추적     | 각 상호작용, 모델 요청, 도구 호출 및 훅(베타)에 대한 스팬         | `OTEL_TRACES_EXPORTER` 및 `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1` |

메트릭 이름, 이벤트 이름 및 속성의 전체 목록은 Claude Code [모니터링](/docs/ko/monitoring-usage) 참조를 참조하세요. Agent SDK는 동일한 CLI를 실행하기 때문에 동일한 데이터를 내보냅니다. 스팬 이름은 아래 [에이전트 추적 읽기](#read-agent-traces)에 나열되어 있습니다.

<h2 id="enable-telemetry-export">
  텔레메트리 내보내기 활성화
</h2>

`CLAUDE_CODE_ENABLE_TELEMETRY=1`을 설정하고 최소 하나의 내보내기 도구를 선택할 때까지 텔레메트리는 꺼져 있습니다. 가장 일반적인 구성은 세 신호 모두를 OTLP HTTP를 통해 수집기로 보냅니다.

다음 예제는 변수를 딕셔너리에 설정하고 `options.env`를 통해 전달합니다. 에이전트는 단일 작업을 실행하고, CLI는 루프가 응답 스트림을 소비하는 동안 `collector.example.com`의 수집기로 스팬, 메트릭 및 이벤트를 내보냅니다:

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions

  OTEL_ENV = {
      "CLAUDE_CODE_ENABLE_TELEMETRY": "1",
      # 베타 상태인 추적에 필요합니다. 메트릭 및 로그 이벤트는 이것이 필요하지 않습니다.
      "CLAUDE_CODE_ENHANCED_TELEMETRY_BETA": "1",
      # 신호별로 내보내기 도구를 선택합니다. SDK의 경우 otlp를 사용하세요. 아래 참고를 참조하세요.
      "OTEL_TRACES_EXPORTER": "otlp",
      "OTEL_METRICS_EXPORTER": "otlp",
      "OTEL_LOGS_EXPORTER": "otlp",
      # 표준 OTLP 전송 구성입니다.
      "OTEL_EXPORTER_OTLP_PROTOCOL": "http/protobuf",
      "OTEL_EXPORTER_OTLP_ENDPOINT": "http://collector.example.com:4318",
      "OTEL_EXPORTER_OTLP_HEADERS": "Authorization=Bearer your-token",
  }


  async def main():
      options = ClaudeAgentOptions(env=OTEL_ENV)
      async for message in query(
          prompt="List the files in this directory", options=options
      ):
          print(message)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const otelEnv = {
    CLAUDE_CODE_ENABLE_TELEMETRY: "1",
    // 베타 상태인 추적에 필요합니다. 메트릭 및 로그 이벤트는 이것이 필요하지 않습니다.
    CLAUDE_CODE_ENHANCED_TELEMETRY_BETA: "1",
    // 신호별로 내보내기 도구를 선택합니다. SDK의 경우 otlp를 사용하세요. 아래 참고를 참조하세요.
    OTEL_TRACES_EXPORTER: "otlp",
    OTEL_METRICS_EXPORTER: "otlp",
    OTEL_LOGS_EXPORTER: "otlp",
    // 표준 OTLP 전송 구성입니다.
    OTEL_EXPORTER_OTLP_PROTOCOL: "http/protobuf",
    OTEL_EXPORTER_OTLP_ENDPOINT: "http://collector.example.com:4318",
    OTEL_EXPORTER_OTLP_HEADERS: "Authorization=Bearer your-token",
  };

  for await (const message of query({
    prompt: "List the files in this directory",
    // TypeScript에서 env는 상속된 환경을 대체하므로 먼저
    // process.env를 전개하여 PATH, ANTHROPIC_API_KEY 및 기타 변수를 유지합니다.
    options: { env: { ...process.env, ...otelEnv } },
  })) {
    console.log(message);
  }
  ```
</CodeGroup>

자식 프로세스는 기본적으로 애플리케이션의 환경을 상속하므로 Dockerfile, Kubernetes 매니페스트 또는 셸 프로필에서 이러한 변수를 내보내고 `options.env`를 완전히 생략하여 동일한 결과를 얻을 수 있습니다.

<Note>
  `console` 내보내기 도구는 텔레메트리를 표준 출력에 기록하며, SDK는 이를 메시지 채널로 사용합니다. SDK를 통해 실행할 때 내보내기 도구 값으로 `console`을 설정하지 마세요. 로컬에서 텔레메트리를 검사하려면 `OTEL_EXPORTER_OTLP_ENDPOINT`를 로컬 수집기 또는 올인원 Jaeger 컨테이너로 지정하세요.
</Note>

<h3 id="flush-telemetry-from-short-lived-calls">
  단기 호출에서 텔레메트리 플러시
</h3>

CLI는 텔레메트리를 배치하고 간격에 따라 내보냅니다. 깨끗한 프로세스 종료 시 보류 중인 데이터를 플러시하려고 시도하지만, 플러시는 짧은 타임아웃으로 제한되므로 수집기가 느리게 응답하면 스팬이 여전히 삭제될 수 있습니다. 프로세스가 CLI가 종료되기 전에 종료되면 배치 버퍼에 있는 모든 것이 손실됩니다. 내보내기 간격을 줄이면 두 창을 모두 줄입니다.

기본적으로 메트릭은 60초마다 내보내고 추적 및 로그는 5초마다 내보냅니다. 다음 예제는 세 간격을 모두 단축하여 데이터가 짧은 작업이 실행되는 동안 수집기에 도달하도록 합니다:

<CodeGroup>
  ```python Python theme={null}
  OTEL_ENV = {
      # ... 이전 예제의 내보내기 도구 구성 ...
      "OTEL_METRIC_EXPORT_INTERVAL": "1000",
      "OTEL_LOGS_EXPORT_INTERVAL": "1000",
      "OTEL_TRACES_EXPORT_INTERVAL": "1000",
  }
  ```

  ```typescript TypeScript theme={null}
  const otelEnv = {
    // ... 이전 예제의 내보내기 도구 구성 ...
    OTEL_METRIC_EXPORT_INTERVAL: "1000",
    OTEL_LOGS_EXPORT_INTERVAL: "1000",
    OTEL_TRACES_EXPORT_INTERVAL: "1000",
  };
  ```
</CodeGroup>

<h2 id="read-agent-traces">
  에이전트 추적 읽기
</h2>

추적은 에이전트 실행의 가장 상세한 보기를 제공합니다. `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1`이 설정되면 에이전트 루프의 각 단계가 추적 백엔드에서 검사할 수 있는 스팬이 됩니다:

* **`claude_code.interaction`:** 프롬프트 수신에서 응답 생성까지 에이전트 루프의 단일 턴을 래핑합니다.
* **`claude_code.llm_request`:** Claude API에 대한 각 호출을 래핑하며, 모델 이름, 지연 시간 및 토큰 수를 속성으로 포함합니다.
* **`claude_code.tool`:** 각 도구 호출을 래핑하며, 권한 대기(`claude_code.tool.blocked_on_user`)와 실행 자체(`claude_code.tool.execution`)에 대한 자식 스팬을 포함합니다.
* **`claude_code.hook`:** 각 [훅](/docs/ko/agent-sdk/hooks) 실행을 래핑합니다. 위의 변수 외에 상세 베타 추적(`ENABLE_BETA_TRACING_DETAILED=1` 및 `BETA_TRACING_ENDPOINT`)이 필요합니다.

`llm_request`, `tool` 및 `hook` 스팬은 포함하는 `claude_code.interaction` 스팬의 자식입니다. 에이전트가 Task 도구를 통해 하위 에이전트를 생성할 때, 하위 에이전트의 `llm_request` 및 `tool` 스팬은 상위 에이전트의 `claude_code.tool` 스팬 아래에 중첩되므로 전체 위임 체인이 하나의 추적으로 나타납니다.

스팬은 기본적으로 `session.id` 속성을 포함합니다. 동일한 [세션](/docs/ko/agent-sdk/sessions)에 대해 여러 `query()` 호출을 수행할 때 백엔드에서 `session.id`로 필터링하여 이를 하나의 타임라인으로 봅니다. `OTEL_METRICS_INCLUDE_SESSION_ID`가 거짓 값으로 설정되면 속성이 생략됩니다.

<Note>
  추적은 베타 상태입니다. 스팬 이름과 속성은 릴리스 간에 변경될 수 있습니다. 모니터링 참조의 [추적(베타)](/docs/ko/monitoring-usage#traces-beta)을 참조하여 추적 내보내기 구성 변수를 확인하세요.
</Note>

<h2 id="link-traces-to-your-application">
  추적을 애플리케이션에 연결
</h2>

SDK는 W3C 추적 컨텍스트를 CLI 하위 프로세스로 자동으로 전파합니다. 애플리케이션에서 OpenTelemetry 스팬이 활성화된 상태에서 `query()`를 호출하면 SDK는 `TRACEPARENT` 및 `TRACESTATE`를 자식 프로세스 환경에 주입하고, CLI는 이를 읽어 `claude_code.interaction` 스팬이 사용자의 스팬의 자식이 되도록 합니다. 그러면 에이전트 실행이 연결되지 않은 루트가 아닌 애플리케이션의 추적 내에 나타납니다.

추적 컨텍스트 전파가 활성화되면 CLI는 실행하는 모든 Bash 및 PowerShell 명령에 `TRACEPARENT`를 전달합니다. Bash 도구를 통해 시작된 명령이 자체 OpenTelemetry 스팬을 내보내면 해당 스팬은 명령을 래핑하는 `claude_code.tool.execution` 스팬 아래에 중첩됩니다.

`options.env`에서 `TRACEPARENT`를 명시적으로 설정하면 자동 주입이 건너뛰어지므로 필요한 경우 특정 상위 컨텍스트를 고정할 수 있습니다. 대화형 CLI 세션은 인바운드 `TRACEPARENT`를 완전히 무시합니다. Agent SDK 및 `claude -p` 실행만 이를 인정합니다. 모니터링 참조의 [추적(베타)](/docs/ko/monitoring-usage#traces-beta)에서 전체 스팬 및 속성 참조를 확인하세요.

<h2 id="tag-telemetry-from-your-agent">
  에이전트에서 텔레메트리 태그 지정
</h2>

기본적으로 CLI는 `service.name`을 `claude-code`로 보고합니다. 여러 에이전트를 실행하거나 동일한 수집기로 내보내는 다른 서비스와 함께 SDK를 실행하는 경우 서비스 이름을 재정의하고 리소스 속성을 추가하여 백엔드에서 에이전트로 필터링할 수 있습니다.

다음 예제는 서비스의 이름을 바꾸고 배포 메타데이터를 첨부합니다. 이러한 값은 에이전트가 내보내는 모든 스팬, 메트릭 및 이벤트에 OpenTelemetry 리소스 속성으로 적용됩니다:

<CodeGroup>
  ```python Python theme={null}
  options = ClaudeAgentOptions(
      env={
          # ... exporter configuration ...
          "OTEL_SERVICE_NAME": "support-triage-agent",
          "OTEL_RESOURCE_ATTRIBUTES": "service.version=1.4.0,deployment.environment=production",
      },
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    env: {
      ...process.env,
      // ... exporter configuration ...
      OTEL_SERVICE_NAME: "support-triage-agent",
      OTEL_RESOURCE_ATTRIBUTES":
        "service.version=1.4.0,deployment.environment=production",
    },
  };
  ```
</CodeGroup>

<h2 id="attribute-actions-to-your-end-users">
  최종 사용자에게 속성 작업 할당
</h2>

CLI는 Anthropic을 호출하는 데 사용하는 자격 증명을 기반으로 모든 이벤트에 [ID 속성](/docs/ko/monitoring-usage#standard-attributes)을 첨부합니다. 하나의 배포에서 많은 최종 사용자를 제공하는 애플리케이션을 구축할 때 이러한 속성은 에이전트가 대신 작동한 최종 사용자가 아닌 서비스의 자격 증명을 식별합니다.

도구 호출 및 MCP 활동을 애플리케이션의 최종 사용자에게 귀속시키려면 각 `query()` 호출에서 최종 사용자 ID를 리소스 속성으로 주입합니다. `OTEL_RESOURCE_ATTRIBUTES`가 [쉼표, 공백 및 등호 기호를 예약](/docs/ko/monitoring-usage#multi-team-organization-support)하므로 값을 퍼센트 인코딩하세요. 다음 예제는 요청하는 사용자 및 테넌트를 한 요청의 모든 스팬 및 이벤트에 첨부합니다. 웹 프레임워크의 `request` 객체가 사용자 및 테넌트 ID를 전달한다고 가정합니다:

<CodeGroup>
  ```python Python theme={null}
  from urllib.parse import quote

  options = ClaudeAgentOptions(
      env={
          # ... 내보내기 도구 구성 ...
          "OTEL_RESOURCE_ATTRIBUTES": f"enduser.id={quote(request.user_id)},tenant.id={quote(request.tenant_id)}",
      },
  )
  ```

  ```typescript TypeScript theme={null}
  const options = {
    env: {
      ...process.env,
      // ... 내보내기 도구 구성 ...
      OTEL_RESOURCE_ATTRIBUTES: `enduser.id=${encodeURIComponent(request.userId)},tenant.id=${encodeURIComponent(request.tenantId)}`,
    },
  };
  ```
</CodeGroup>

최종 사용자 ID가 첨부되면 `tool_decision`, `tool_result`, `mcp_server_connection` 및 `permission_mode_changed` 이벤트는 `claude_code.` 접두사가 있는 로그 레코드로 내보내지며, Security Information and Event Management(SIEM) 플랫폼으로 전달할 수 있는 사용자별 감사 추적이 됩니다. 모니터링 참조의 [보안 이벤트 감사](/docs/ko/monitoring-usage#audit-security-events)에서 보안 관련 이벤트의 전체 목록과 각 이벤트가 포함하는 속성을 확인하세요.

<h2 id="control-sensitive-data-in-exports">
  내보내기에서 민감한 데이터 제어
</h2>

텔레메트리는 기본적으로 구조적입니다. 지속 시간, 모델 이름 및 도구 이름은 모든 스팬에 기록됩니다. 토큰 수는 기본 API 요청이 사용 데이터를 반환할 때 기록되므로 실패하거나 중단된 요청에 대한 스팬은 이를 생략할 수 있습니다. 에이전트가 읽고 쓰는 콘텐츠는 기본적으로 기록되지 않습니다. 이러한 옵트인 변수는 내보낸 데이터에 콘텐츠를 추가합니다:

| 변수                        | 추가 내용                                                                                                                                                                                                                                                                                            |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `OTEL_LOG_USER_PROMPTS=1` | `claude_code.user_prompt` 이벤트 및 `claude_code.interaction` 스팬의 프롬프트 텍스트                                                                                                                                                                                                                           |
| `OTEL_LOG_TOOL_DETAILS=1` | `claude_code.tool_result` 이벤트의 도구 입력 인수(파일 경로, 셸 명령, 검색 패턴)                                                                                                                                                                                                                                      |
| `OTEL_LOG_TOOL_CONTENT=1` | `claude_code.tool`의 스팬 이벤트로 전체 도구 입력 및 출력 본문, 60KB에서 잘림. [추적](#read-agent-traces)이 활성화되어야 함                                                                                                                                                                                                      |
| `OTEL_LOG_RAW_API_BODIES` | `claude_code.api_request_body` 및 `claude_code.api_response_body` 로그 이벤트로 전체 Anthropic Messages API 요청 및 응답 JSON. 인라인 본문의 경우 `1`로 설정하여 60KB에서 잘리거나, 이벤트의 `body_ref` 경로가 있는 디스크의 잘리지 않은 본문의 경우 `file:<dir>`로 설정합니다. 본문에는 전체 대화 기록이 포함되며 확장 사고 콘텐츠가 수정됩니다. 이를 활성화하면 위의 세 변수가 드러낼 모든 것에 대한 동의를 의미합니다 |

에이전트가 처리하는 데이터를 저장하도록 관찰성 파이프라인이 승인되지 않은 경우 이를 설정하지 마세요. 모니터링 참조의 [보안 및 개인정보](/docs/ko/monitoring-usage#security-and-privacy)에서 모든 속성 및 수정 동작의 전체 목록을 확인하세요.

<h2 id="related-documentation">
  관련 문서
</h2>

이 가이드는 에이전트 모니터링 및 배포를 위한 인접 주제를 다룹니다:

* [비용 및 사용량 추적](/docs/ko/agent-sdk/cost-tracking): 외부 백엔드 없이 메시지 스트림에서 토큰 및 비용 데이터를 읽습니다.
* [Agent SDK 호스팅](/docs/ko/agent-sdk/hosting): 환경 수준에서 OpenTelemetry 변수를 설정할 수 있는 컨테이너에 에이전트를 배포합니다.
* [모니터링](/docs/ko/monitoring-usage): CLI가 내보내는 모든 환경 변수, 메트릭 및 이벤트에 대한 완전한 참조입니다.
