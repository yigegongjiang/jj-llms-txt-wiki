> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent SDK 호스팅

> 프로덕션에서 Agent SDK 배포: 서브프로세스 아키텍처, 세션 지속성, 확장성, 관찰성, Docker, Kubernetes 및 샌드박스 제공자를 위한 멀티테넌트 격리.

Agent SDK는 셸, 작업 디렉토리 및 디스크의 세션 파일을 소유하는 `claude` CLI 서브프로세스를 생성하고 감독합니다. 이를 호스팅하는 것은 상태 비저장 API 래퍼를 호스팅하는 것과 같지 않습니다. 실행 중인 모든 에이전트는 로컬 상태에 연결된 장기 실행 프로세스이며, 이는 리소스 할당, 세션 지속성 및 테넌트 간 확장 방식을 결정합니다.

이 페이지는 자체 인프라에서의 자체 호스팅을 다룹니다. 배포 가능한 Dockerfile 및 Kubernetes 매니페스트는 [호스팅 쿡북](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting)을 참조하십시오.

에이전트 루프를 자체 인프라에서 실행할 필요가 없는 경우 [Managed Agents](https://platform.claude.com/docs/en/managed-agents/overview)를 대신 고려하십시오. Anthropic이 에이전트 루프를 호스팅하며, 애플리케이션은 클라이언트 SDK 또는 REST API를 통해 이벤트를 보내고 스트리밍된 결과를 수신합니다. 도구 실행은 Anthropic 관리 클라우드 샌드박스 또는 자체 인프라의 [자체 호스팅 샌드박스](https://platform.claude.com/docs/en/managed-agents/self-hosted-sandboxes)에서 실행됩니다.

<h2 id="the-subprocess-model">
  서브프로세스 모델
</h2>

이 페이지의 모든 호스팅 결정은 SDK가 에이전트를 실행하는 방식에서 비롯됩니다. 코드에서 `query()`를 호출하면 SDK는 별도의 `claude` CLI 프로세스를 생성하고 stdio를 통해 통신합니다. 해당 서브프로세스는 셸, 작업 디렉토리, 로컬 디스크의 JSONL 세션 트랜스크립트를 소유합니다.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/agent-sdk/hosting-subprocess.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=9dac857ca9d3b1410c3734900c386004" className="dark:hidden" alt="요청 흐름: 클라이언트에서 앱으로, 컨테이너 내 stdio를 통해 claude CLI 서브프로세스를 생성하고, 서브프로세스는 로컬 디스크에 쓰고 HTTPS를 통해 api.anthropic.com을 호출합니다" width="920" height="220" data-path="images/agent-sdk/hosting-subprocess.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/agent-sdk/hosting-subprocess-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=3fdeff3d7f44b2b67762668acfbb25f5" className="hidden dark:block" alt="요청 흐름: 클라이언트에서 앱으로, 컨테이너 내 stdio를 통해 claude CLI 서브프로세스를 생성하고, 서브프로세스는 로컬 디스크에 쓰고 HTTPS를 통해 api.anthropic.com을 호출합니다" width="920" height="220" data-path="images/agent-sdk/hosting-subprocess-dark.svg" />

하나의 에이전트 세션은 하나의 서브프로세스에 매핑됩니다. N개의 동시 세션을 실행하면 N개의 서브프로세스가 생성되며, 각각은 자체 프로세스 트리와 트랜스크립트 파일을 가집니다. 기본적으로 모두 애플리케이션의 작업 디렉토리를 상속합니다. 세션이 별도의 파일시스템이 필요한 경우 각 세션의 `query()` 호출 옵션에서 고유한 `cwd`를 전달합니다:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "Summarize the files in this directory",
    options: { cwd: "/work/session-a" },
  })) {
    console.log(message);
  }
  ```

  ```python Python theme={null}
  import asyncio

  from claude_agent_sdk import ClaudeAgentOptions, query


  async def main():
      async for message in query(
          prompt="Summarize the files in this directory",
          options=ClaudeAgentOptions(cwd="/work/session-a"),
      ):
          print(message)


  asyncio.run(main())
  ```
</CodeGroup>

이 페이지의 TypeScript 예제는 최상위 `await`를 사용하므로 `.mts` 파일로 저장하거나 `package.json`에서 `"type": "module"`을 설정합니다.

<h3 id="state-that-lives-on-local-disk">
  로컬 디스크에 저장되는 상태
</h3>

세 가지 종류의 에이전트 상태가 기본적으로 컨테이너의 파일시스템에 저장됩니다. 이들 중 어느 것도 컨테이너 재시작, 축소, 또는 다른 노드로의 이동을 견디지 못합니다.

| 상태                 | 기본 위치                                                                     |
| ------------------ | ------------------------------------------------------------------------- |
| 세션 트랜스크립트          | `~/.claude/projects/`, 또는 설정된 경우 `CLAUDE_CONFIG_DIR` 아래의 `projects/` 디렉토리 |
| `CLAUDE.md` 메모리 파일 | 사용자 계층의 경우 `~/.claude/CLAUDE.md`, 프로젝트 계층의 경우 세션의 작업 디렉토리                 |
| 작업 디렉토리 아티팩트       | 세션의 작업 디렉토리                                                               |

호스트 간에 트랜스크립트를 유지하려면 [`SessionStore` 어댑터](/docs/ko/agent-sdk/session-storage)를 구성합니다. 메모리 파일 및 기타 작업 디렉토리 아티팩트는 마운트된 볼륨 또는 객체 저장소 동기화와 같은 자체 저장소 전략이 필요합니다.

세션, 재개, 포킹이 API 수준에서 어떻게 작동하는지에 대해서는 [세션](/docs/ko/agent-sdk/sessions)을 참조합니다.

<h2 id="choose-a-session-pattern">
  세션 패턴 선택
</h2>

이 네 가지 패턴은 세션 라이프사이클을 다룹니다: 컨테이너가 제공하는 세션에 상대적으로 얼마나 오래 존재하는지입니다. 컨테이너가 실행되는 위치에 대해서는 [호스팅 쿡북](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb)에 로컬 Docker, Modal, Kubernetes용 [배포 가능한 코드](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting)가 있습니다. 여기서 세션 패턴을 선택하고 쿡북에서 배포 대상을 선택하십시오.

<h3 id="ephemeral-sessions">
  임시 세션
</h3>

각 사용자 작업에 대해 컨테이너를 생성하고 작업이 완료되면 삭제합니다. 일회성 작업에 최적입니다. 사용자는 작업이 완료되는 동안 AI와 상호작용할 수 있지만, 완료되면 컨테이너가 삭제됩니다.

예제 워크로드에는 버그 조사 및 수정, 송장 및 영수증 추출, 문서 번역, 미디어 변환이 포함됩니다.

컨테이너는 `TASK_PROMPT` 환경 변수에서 작업을 읽고, SDK를 호출하고, 종료하는 일회성 진입점을 실행합니다.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const prompt = process.env.TASK_PROMPT!;
  for await (const message of query({ prompt, options: { maxTurns: 20 } })) {
    console.log(message);
  }
  ```

  ```python Python theme={null}
  import asyncio
  import os

  from claude_agent_sdk import ClaudeAgentOptions, query


  async def main():
      async for message in query(
          prompt=os.environ["TASK_PROMPT"],
          options=ClaudeAgentOptions(max_turns=20),
      ):
          print(message)


  asyncio.run(main())
  ```
</CodeGroup>

스크립트는 각 메시지가 도착할 때 인쇄하며, 작업이 턴 제한 내에서 완료되면 `subtype`이 `success`인 결과 메시지를 포함합니다. 작업이 20턴 제한에 도달하면 결과 메시지의 `subtype`은 `error_max_turns`이고 `query()` 호출은 이를 생성한 후 오류를 발생시키므로, 컨테이너가 깔끔하게 종료되어야 하면 루프를 try 블록으로 감싸십시오. 오류 서브타입에 대해서는 [결과 처리](/docs/ko/agent-sdk/agent-loop#handle-the-result)를 참조하십시오.

<h3 id="long-running-sessions">
  장기 실행 세션
</h3>

지속적인 컨테이너 인스턴스를 실행하며, 종종 컨테이너당 여러 SDK 프로세스를 호스팅하여 진행 중인 작업을 제공합니다. 자율적으로 조치를 취하거나, 콘텐츠를 제공하거나, 대량의 메시지 스트림을 처리하는 에이전트에 최적입니다.

예제 워크로드에는 들어오는 메일을 분류하고 응답하는 이메일 에이전트, 컨테이너 포트를 통해 사용자별 편집 가능한 사이트를 호스팅하는 사이트 빌더, Slack과 같은 플랫폼에서 지속적인 트래픽을 처리하는 채팅봇이 포함됩니다.

컨테이너는 HTTP 또는 WebSocket 엔드포인트를 노출하고 각 활성 세션을 장기 실행 쿼리 및 그 뒤의 서브프로세스에 매핑합니다. TypeScript에서는 [`streamInput()`](/docs/ko/agent-sdk/typescript#query-object)을 사용하여 활성 세션에 턴을 추가하고 [`startup()`](/docs/ko/agent-sdk/typescript#startup)을 사용하여 들어오는 트래픽 전에 서브프로세스를 미리 준비합니다. Python에서는 [`ClaudeSDKClient`](/docs/ko/agent-sdk/python#claudesdkclient)를 사용하여 여러 턴에 걸쳐 세션을 열린 상태로 유지합니다. 컨테이너 크기를 메모리에 보유할 수 있는 최대 동시 세션 수에 맞게 조정하십시오.

<h3 id="hybrid-sessions">
  하이브리드 세션
</h3>

시작 시 [`SessionStore`](/docs/ko/agent-sdk/session-storage)에서 수화되고 업데이트를 다시 유지하는 임시 컨테이너입니다. 많은 상호작용에 걸쳐 있지만 그 사이에 유휴 상태인 세션에 최적입니다. 컨테이너는 유휴 기간 동안 종료되고 사용자가 돌아올 때 다시 시작됩니다.

예제 워크로드에는 간헐적인 체크인이 있는 개인 프로젝트 관리자, 몇 시간에 걸쳐 일시 중지되고 재개되는 심층 연구, 상호작용 전반에 걸쳐 티켓 기록을 로드하는 고객 지원 에이전트가 포함됩니다.

사용자가 돌아올 것으로 예상되는 빈도에 맞게 제공자의 유휴 시간 초과를 조정하십시오. `SessionStore`가 구성되지 않은 상태에서 컨테이너를 종료하면 트랜스크립트가 손실되므로 저장소는 이 패턴에 필수이며 선택 사항이 아닙니다.

패턴은 공유 저장소가 연결된 ID로 세션을 재개하는 것에 달려 있습니다:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, type SessionStore } from "@anthropic-ai/claude-agent-sdk";

  declare const userInput: string;
  declare const sessionId: string;          // looked up from your database by user
  declare const sessionStore: SessionStore; // an object store, key-value store, database, or your own adapter

  for await (const message of query({
    prompt: userInput,
    options: { resume: sessionId, sessionStore },
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions, SessionStore
  import asyncio

  user_input: str = ...
  session_id: str = ...              # looked up from your database by user
  session_store: SessionStore = ...  # an object store, key-value store, database, or your own adapter


  async def main():
      async for message in query(
          prompt=user_input,
          options=ClaudeAgentOptions(
              resume=session_id,
              session_store=session_store,
          ),
      ):
          ...


  asyncio.run(main())
  ```
</CodeGroup>

<h3 id="multi-agent-container">
  다중 에이전트 컨테이너
</h3>

하나의 컨테이너 내에서 여러 SDK 서브프로세스를 실행합니다. 에이전트가 긴밀하게 협력해야 하는 경우, 예를 들어 에이전트가 공유 환경에서 서로 상호작용하는 다중 에이전트 시뮬레이션에 최적입니다.

각 에이전트에 자신의 작업 디렉토리를 제공하여 서로의 파일을 덮어쓰지 않도록 하고, 설정 로딩을 격리하여 에이전트별 `CLAUDE.md` 파일이 에이전트 간에 유출되지 않도록 하십시오. 특정 옵션에 대해서는 [다중 테넌트 격리](#multi-tenant-isolation)를 참조하십시오.

<h2 id="provision-the-container">
  컨테이너 프로비저닝
</h2>

<h3 id="container-based-sandboxing">
  컨테이너 기반 샌드박싱
</h3>

프로세스 격리, 리소스 제한, 네트워크 제어 및 임시 파일 시스템을 위해 샌드박스 컨테이너 내에서 SDK를 실행합니다.

공급자를 선택할 때 답변해야 할 질문:

* **샌드박스를 실행하는 주체**: 샌드박스형 서비스 공급자는 사용자를 위해 인프라를 운영하며, 자체 호스팅 옵션은 자신의 환경에서 실행할 소프트웨어를 제공합니다.
* **콜드 스타트 지연 시간**: "샌드박스 생성"부터 "첫 번째 요청을 수락할 준비 완료"까지의 시간입니다. 임시 패턴은 1초 미만의 시작이 필요합니다. 장기 실행 패턴은 더 많은 시간을 허용합니다.
* **지속적 스토리지**: 공급자가 내구성 있는 볼륨을 제공하는지 또는 임시 디스크만 제공하는지 여부입니다. 하이브리드 패턴은 샌드박스 내부 또는 외부의 어딘가에 내구성 있는 스토리지가 필요합니다.
* **가격 책정 모델**: 초당, 요청당 또는 시간당 정액 청구입니다. 초당 가격 책정은 버스트 임시 워크로드에 적합합니다. 시간당 청구는 장기 실행 세션에 적합합니다.
* **네트워킹**: 사용자 정의 송신 규칙, 아웃바운드 프록시 및 규제 환경을 위한 프라이빗 VPC 피어링 지원입니다.

Docker, gVisor 및 Firecracker와 같은 자체 호스팅 옵션 및 자세한 격리 구성은 [격리 기술](/docs/ko/agent-sdk/secure-deployment#isolation-technologies)을 참조하십시오.

<h3 id="runtime-dependencies">
  런타임 종속성
</h3>

컨테이너에는 SDK의 언어 런타임이 필요합니다:

* Python SDK의 경우 Python 3.10+ 또는 TypeScript SDK의 경우 Node.js 18+
* TypeScript 및 Python SDK 모두 대부분의 설치에 대해 네이티브 Claude Code 바이너리를 번들로 제공하며, 생성된 CLI는 별도의 Node.js 설치가 필요하지 않습니다. 별도의 네이티브 Claude Code 설치가 필요한 설치는 [빠른 시작의 설치 참고](/docs/ko/agent-sdk/quickstart)를 참조하십시오.

번들된 바이너리는 SDK 패키지 버전에 고정되므로 SDK를 업데이트하는 것이 CLI를 업데이트하는 방법입니다. SDK는 semver를 따릅니다: 패치 릴리스를 지속적으로 적용하고 마이너 버전을 적용하기 전에 [TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/CHANGELOG.md) 또는 [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/CHANGELOG.md) 변경 로그를 검토하십시오.

<h3 id="resources">
  리소스
</h3>

새로 시작된 인스턴스의 경우 에이전트당 1GiB RAM, 5GiB 디스크 및 1 CPU가 합리적인 시작점입니다. 메모리 사용량은 세션 길이 및 도구 활동에 따라 증가하므로 유휴 기준선이 아닌 실제로 필요한 세션 길이 및 동시성에 맞게 크기를 조정하십시오. 호스트당 에이전트 수를 계산하는 방법은 [확장 및 동시성](#scaling-and-concurrency)을 참조하십시오.

<h3 id="network">
  네트워크
</h3>

SDK는 `api.anthropic.com`으로 또는 Amazon Bedrock 또는 Google Cloud의 Agent Platform에서 실행할 때 공급자의 지역 엔드포인트로 아웃바운드 HTTPS가 필요합니다. 에이전트가 [MCP 서버](/docs/ko/agent-sdk/mcp)를 사용하거나 외부 도구를 사용하는 경우 해당 엔드포인트에 대한 아웃바운드 액세스도 필요합니다. 프로덕션의 경우 도메인 허용 목록을 적용하고, 자격 증명을 주입하고, 요청을 기록하는 송신 프록시를 통해 아웃바운드 트래픽을 라우팅하십시오. 전체 패턴은 [보안 배포](/docs/ko/agent-sdk/secure-deployment)를 참조하십시오.

인바운드 트래픽의 경우 컨테이너에서 HTTP 또는 WebSocket 포트를 노출합니다. 애플리케이션은 해당 포트에서 클라이언트 요청을 처리하고 내부적으로 SDK를 호출합니다. 서브프로세스 자체는 네트워크에서 수신 대기하지 않습니다.

<h2 id="handle-production-concerns">
  프로덕션 문제 처리
</h2>

자체 호스팅 에이전트를 배포하기 전에 이러한 결정 사항을 검토하십시오.

<h3 id="session-and-state-persistence">
  세션 및 상태 지속성
</h3>

기본 로컬 디스크는 재시작, 축소 또는 다른 노드로의 이동 시 손실됩니다. 사용자가 재개할 것으로 예상하는 모든 세션에 대해 [`SessionStore` 어댑터](/docs/ko/agent-sdk/session-storage)를 사용하여 트랜스크립트를 지속 가능한 스토리지에 미러링하십시오. [참조 구현](/docs/ko/agent-sdk/session-storage#reference-implementations)에서 객체 저장소, 키-값 저장소, 데이터베이스용 예제 어댑터와 자신의 어댑터를 위한 적합성 테스트 모음을 참조하십시오.

`SessionStore` 동작에 대해 알아야 할 세 가지 사항:

* **트랜스크립트만**: `SessionStore`는 트랜스크립트를 미러링하며, `CLAUDE.md` 메모리 파일이나 다른 작업 디렉토리 아티팩트는 미러링하지 않습니다. 공유 볼륨을 마운트하거나 이들을 별도로 동기화하십시오.
* **미러링, 대체 아님**: 서브프로세스는 먼저 로컬 디스크에 쓰고, SDK는 각 배치의 복사본을 저장소로 전달합니다. 새로운 세션의 로컬 트랜스크립트는 실행보다 오래 지속되며, 저장소에서 재개된 실행은 끝에서 로컬 복사본을 삭제하므로 저장소가 유일한 지속 가능한 복사본을 보유합니다. [이중 쓰기 아키텍처](/docs/ko/agent-sdk/session-storage#dual-write-architecture)를 참조하십시오.
* **`mirror_error` 메시지**: SDK가 배치를 저장소에 전달할 수 없을 때, 배치를 삭제하고, `{ type: "system", subtype: "mirror_error" }` 메시지를 내보내며, 쿼리를 계속합니다. 저장소 지속성이 중요한 경우 이에 대해 경고하십시오. [미러 쓰기는 최선의 노력](/docs/ko/agent-sdk/session-storage#mirror-writes-are-best-effort)에서 재시도 및 타임아웃 동작을 참조하십시오.

<h3 id="observability">
  관찰성
</h3>

Agent SDK 에이전트는 많은 API 왕복에 걸쳐 도구 호출을 생성하는 장기 실행 프로세스입니다. 텔레메트리 없이는 어떤 도구가 실행되었는지, 얼마나 오래 걸렸는지, 또는 세션이 어디서 정체되었는지 볼 수 없습니다.

SDK는 환경에서 OpenTelemetry 구성을 상속합니다. 모든 `query()` 호출이 스팬, 메트릭 및 로그 이벤트를 수집기로 내보내도록 컨테이너 또는 오케스트레이터 수준에서 OTEL 환경 변수를 설정하십시오. 아래 예제는 세 신호 모두에 대해 OTLP 내보내기를 활성화합니다. `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA`는 트레이스에만 필요하며, 메트릭과 로그만 내보내는 경우 생략하십시오.

```bash title=".env" theme={null}
CLAUDE_CODE_ENABLE_TELEMETRY=1
CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1
OTEL_TRACES_EXPORTER=otlp
OTEL_METRICS_EXPORTER=otlp
OTEL_LOGS_EXPORTER=otlp
OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf
OTEL_EXPORTER_OTLP_ENDPOINT=http://collector.example.com:4318
```

프롬프트 텍스트 및 도구 입력은 기본적으로 내보내기에 포함되지 않습니다. [내보내기에서 민감한 데이터 제어](/docs/ko/agent-sdk/observability#control-sensitive-data-in-exports)에서 옵트인 플래그를 참조하고, [관찰성](/docs/ko/agent-sdk/observability)에서 전체 신호 카탈로그를 참조하십시오.

<h3 id="auth-and-secrets">
  인증 및 비밀
</h3>

호스팅 시점에 세 가지 인증 문제가 중요합니다:

* **Anthropic API**: 서브프로세스는 환경에서 `ANTHROPIC_API_KEY`를 읽습니다. 비밀 관리자에서 제공하거나, `ANTHROPIC_BASE_URL`을 설정하여 모델 호출을 컨테이너 외부에서 키를 주입하는 프록시를 통해 라우팅하십시오. [자격증명 관리](/docs/ko/agent-sdk/secure-deployment#credential-management)에서 프록시 패턴을 참조하고, [SDK 빠른 시작의 설정](/docs/ko/agent-sdk/quickstart#setup)에서 지원되는 인증 방법을 참조하십시오.
* **인바운드**: 에이전트 컨테이너 앞의 게이트웨이에 인증을 배치하십시오. 에이전트는 사전 인증된 요청을 수신해야 하며 사용자 토큰을 검증하는 구성 요소가 아니어야 합니다.
* **아웃바운드 도구**: 도구 자격증명을 에이전트 환경 밖에 유지하십시오. 아웃바운드 호출을 요청이 컨테이너를 떠난 후 API 키를 주입하는 프록시를 통해 라우팅하십시오. 에이전트가 호출을 수행하고, 프록시가 자격증명을 추가합니다.

<h3 id="scaling-and-concurrency">
  확장 및 동시성
</h3>

각 세션은 자체 서브프로세스에서 실행되므로 호스트의 동시성은 RAM이 보유할 수 있는 서브프로세스 수로 제한됩니다.

이 공식으로 각 호스트를 크기 조정하십시오:

```text theme={null}
호스트당 에이전트 = (호스트 RAM - 오버헤드) / (세션당 RAM 상한)
```

대표적인 세션을 목표 길이까지 예상 도구 로드 하에서 실행하고 피크 RSS를 기록하여 세션당 상한을 측정하십시오. [리소스](#resources)의 1 GiB 시작점은 상한이 아닌 하한입니다.

수평 확장 라우팅은 패턴에 따라 다릅니다. 컨테이너가 많은 세션을 보유하는 장기 실행 세션의 경우, 로드 밸런서 뒤에 컨테이너 풀을 실행하고 `sessionId`에 대한 일관된 해싱을 사용하여 각 세션을 한 컨테이너에 고정하십시오. 고정된 세션은 제거되거나 컨테이너가 재시작될 때까지 같은 컨테이너, 따라서 같은 실행 중인 서브프로세스를 계속 사용합니다.

<h3 id="cost">
  비용
</h3>

Anthropic 토큰 비용은 일반적으로 컨테이너 인프라 비용을 한 자릿수 이상으로 지배합니다. 최소한으로 프로비저닝된 컨테이너는 시간당 약 \$0.05를 실행하는 반면, 단일 장기 에이전트 세션은 토큰에서 수 달러를 소비할 수 있습니다. [비용 추적](/docs/ko/agent-sdk/cost-tracking)에서 세션당 토큰 회계를 참조하십시오.

<h3 id="multi-tenant-isolation">
  다중 테넌트 격리
</h3>

기본 SDK 동작은 파일 시스템에서 설정 및 `CLAUDE.md` 메모리 파일을 읽습니다. 여러 테넌트를 제공하는 공유 컨테이너에서 이러한 파일은 한 테넌트의 컨텍스트를 다른 테넌트의 세션으로 유출할 수 있습니다.

공유 컨테이너 내에서 테넌트를 격리하려면:

* TypeScript에서 `settingSources: []`를 전달하거나 Python에서 `setting_sources=[]`를 전달하여 사용자, 프로젝트 및 로컬 설정을 건너뛰십시오.
* `env`에서 `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1`을 설정하십시오. [자동 메모리](/docs/ko/memory#auto-memory)는 `~/.claude/projects/<project>/memory/`에서 `settingSources`에 관계없이 시스템 프롬프트로 로드됩니다. [settingSources가 제어하지 않는 것](/docs/ko/agent-sdk/claude-code-features#what-settingsources-does-not-control)에서 무조건 로드되는 다른 입력을 참조하십시오.
* `CLAUDE_CONFIG_DIR`을 테넌트별 디렉토리로 지정하여 테넌트가 `~/.claude.json` 전역 구성을 공유하지 않도록 하십시오. 각 구성 디렉토리가 하나의 작업 디렉토리를 제공하고 테넌트 간에 [`SessionStore`](/docs/ko/agent-sdk/session-storage)를 공유하지 않을 때, `env`에서 [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/ko/sessions#name-the-project-directory-yourself)을 설정하여 그 아래의 트랜스크립트 경로를 짧게 유지할 수도 있습니다. TypeScript Agent SDK v0.3.234 이상 또는 Python Agent SDK v0.2.140 이상이 필요합니다.
* 테넌트별 작업 디렉토리를 사용하십시오. 모든 `query()` 호출에서 `cwd`를 명시적으로 전달하십시오.
* 프록시에서 테넌트별 이그레스 규칙을 적용하십시오(예: 서로 다른 아웃바운드 IP, 자격증명 또는 도메인 허용 목록). 이렇게 하면 손상된 테넌트가 다른 테넌트의 아웃바운드 정책을 통해 데이터를 유출할 수 없습니다.

아래 예제는 설정, 자동 메모리, 구성 디렉토리 및 작업 디렉토리 옵션을 함께 적용합니다. `tenantDir` 및 `configDir`을 구성하여 각 테넌트가 다른 테넌트가 읽을 수 없는 경로를 얻도록 하십시오. TypeScript에서 `env`는 서브프로세스 환경을 대체하므로 `...process.env`를 전개하여 `PATH` 및 `ANTHROPIC_API_KEY`와 같은 상속된 변수를 유지하십시오. Python에서 `env`는 상속된 환경 위에 병합됩니다.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  declare const prompt: string;
  declare const tenantDir: string;
  declare const configDir: string;

  for await (const message of query({
    prompt,
    options: {
      cwd: tenantDir,
      settingSources: [],
      env: {
        ...process.env,
        CLAUDE_CONFIG_DIR: configDir,
        CLAUDE_CODE_DISABLE_AUTO_MEMORY: "1",
      },
    },
  })) {
    // ...
  }
  ```

  ```python Python theme={null}
  from claude_agent_sdk import query, ClaudeAgentOptions
  import asyncio

  prompt: str = ...
  tenant_dir: str = ...
  config_dir: str = ...


  async def main():
      async for message in query(
          prompt=prompt,
          options=ClaudeAgentOptions(
              cwd=tenant_dir,
              setting_sources=[],
              env={
                  "CLAUDE_CONFIG_DIR": config_dir,
                  "CLAUDE_CODE_DISABLE_AUTO_MEMORY": "1",
              },
          ),
      ):
          ...


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="known-limitations">
  알려진 제한 사항
</h2>

배포 설계에서 이러한 사항들을 고려하십시오.

| 제한 사항                              | 해결 방법                                                                                                                                                                              |
| ---------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 최상위 세션 타임아웃 없음                     | 세션은 자동으로 타임아웃되지 않습니다. TypeScript에서 `maxTurns`를 설정하거나 Python에서 `max_turns`를 설정하여 에이전트가 중지되기 전에 수행하는 도구 사용 왕복 횟수를 제한하십시오.                                                            |
| 장시간 세션에서의 메모리 증가                   | 세션 길이를 제한하거나 주기적으로 서브프로세스를 재활용하십시오. [확장성 및 동시성](#scaling-and-concurrency)을 참조하십시오.                                                                                                 |
| 대규모 병렬 서브에이전트 팬아웃이 속도 제한에 도달할 수 있음 | 광범위한 디스패치를 한 번에 발행하는 대신 작업을 더 작은 배치로 나누십시오.                                                                                                                                        |
| 서브에이전트별 벽시계 데드라인 없음                | 각 [서브에이전트](/docs/ko/agent-sdk/subagents)를 `AgentDefinition`의 `maxTurns`로 제한하십시오. `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`는 서브에이전트가 출력 생성을 중지할 때 발동하는 정지 감시 장치를 설정합니다. 이는 총 런타임 데드라인이 아닙니다. |

<h2 id="troubleshoot-deployment-failures">
  배포 실패 문제 해결
</h2>

기계에서 작동하는 에이전트가 배포된 서비스에서 실패할 때 이 섹션을 사용하십시오. 아래의 각 항목은 실패를 명명하고 이를 다루는 항목으로 연결합니다:

* **서비스 시작 시 CLI를 찾을 수 없음**: Python에서는 컨테이너 또는 서비스 관리자가 셸과 다른 `PATH`로 애플리케이션을 실행하므로 로컬에서 작동하는 설치가 프로세스에 표시되지 않습니다. TypeScript에서는 이미지 빌드가 SDK의 선택적 종속성을 건너뛰었거나 `pathToClaudeCodeExecutable`이 이미지에 존재하지 않는 파일을 가리킵니다. [Claude Code를 찾을 수 없음](/docs/ko/agent-sdk/troubleshooting#clinotfounderror-claude-code-not-found)을 참조하십시오.
* **이미지에 CLI가 있지만 시작되지 않음**: Claude Code는 컨테이너의 아키텍처 또는 libc와 일치하지 않는 바이너리에서 시작할 수 없거나 이미지 빌드에서 실행 권한을 잃은 파일에서 시작할 수 없습니다. [Claude Code 시작 실패](/docs/ko/agent-sdk/troubleshooting#cliconnectionerror-failed-to-start-claude-code)를 참조하십시오.
* **Claude Code 프로세스가 실행 중 종료됨**: 애플리케이션이 수신하는 오류는 SDK 언어와 CLI가 먼저 오류 결과를 보고했는지 여부에 따라 달라집니다. [CLI 프로세스 종료](/docs/ko/agent-sdk/troubleshooting#cli-process-exit) 아래의 항목들이 각 메시지를 다룹니다.

<h2 id="next-steps">
  다음 단계
</h2>

* [호스팅 쿡북](https://github.com/anthropics/claude-cookbooks/blob/main/claude_agent_sdk/07_Hosting_the_agent.ipynb): Docker, Modal, Kubernetes용 [배포 가능한 코드](https://github.com/anthropics/claude-cookbooks/tree/main/claude_agent_sdk/hosting)가 포함된 노트북 안내입니다.
* [세션 저장소](/docs/ko/agent-sdk/session-storage): `SessionStore` 어댑터를 사용하여 호스트 간에 트랜스크립트를 유지합니다.
* [관찰성](/docs/ko/agent-sdk/observability): OTEL 추적, 메트릭 및 로그를 수집기로 내보냅니다.
* [보안 배포](/docs/ko/agent-sdk/secure-deployment): 네트워크 제어, 자격증명 관리 및 격리 강화입니다.
* [비용 추적](/docs/ko/agent-sdk/cost-tracking): 세션별 토큰 및 비용 회계입니다.
