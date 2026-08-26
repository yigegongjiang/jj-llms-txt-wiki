> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 세션을 외부 저장소에 유지하기

> 세션 기록을 S3, Redis 또는 자신의 백엔드로 미러링하여 모든 호스트에서 세션을 재개할 수 있습니다.

기본적으로 SDK는 세션 기록을 로컬 파일 시스템의 `~/.claude/projects/` 아래 JSONL 파일로 작성합니다. `SessionStore` 어댑터를 사용하면 이러한 기록을 S3, Redis 또는 데이터베이스와 같은 자신의 백엔드로 미러링할 수 있으므로 한 호스트에서 생성된 세션을 다른 호스트에서 재개할 수 있습니다.

세션 저장소를 사용하는 일반적인 이유:

* **다중 호스트 배포.** 서버리스 함수, 자동 확장 워커 및 CI 러너는 파일 시스템을 공유하지 않습니다. 공유 저장소를 사용하면 모든 복제본이 모든 세션을 재개할 수 있습니다.
* **내구성.** 로컬 컨테이너는 임시적입니다. S3 또는 데이터베이스로 지원되는 저장소는 재시작 및 재배포를 견딥니다.
* **규정 준수 및 감사.** 이미 관리하는 저장소에 기록을 유지하고, 자신의 보존 규칙, 암호화 및 접근 제어를 적용합니다.

<h2 id="the-sessionstore-interface">
  `SessionStore` 인터페이스
</h2>

`SessionStore`는 두 개의 필수 메서드 `append`와 `load`, 그리고 세 개의 선택적 메서드를 가진 객체입니다. SDK는 쿼리 중에 기록 항목을 작성하기 위해 `append`를 호출하고 재개를 위해 이를 다시 읽기 위해 `load`를 호출합니다.

<CodeGroup>
  ```typescript TypeScript theme={null}
  // Exported from @anthropic-ai/claude-agent-sdk as
  // SessionStore, SessionKey, SessionStoreEntry.

  type SessionKey = {
    projectKey: string;
    sessionId: string;
    subpath?: string;
  };

  type SessionStore = {
    // Required
    append(key: SessionKey, entries: SessionStoreEntry[]): Promise<void>;
    load(key: SessionKey): Promise<SessionStoreEntry[] | null>;

    // Optional
    listSessions?(
      projectKey: string,
    ): Promise<Array<{ sessionId: string; mtime: number }>>;
    delete?(key: SessionKey): Promise<void>;
    listSubkeys?(key: {
      projectKey: string;
      sessionId: string;
    }): Promise<string[]>;
  };
  ```

  ```python Python theme={null}
  # Exported from claude_agent_sdk as
  # SessionStore, SessionKey, SessionStoreEntry.

  class SessionKey(TypedDict):
      project_key: str
      session_id: str
      subpath: NotRequired[str]

  class SessionStore(Protocol):
      # Required
      async def append(
          self, key: SessionKey, entries: list[SessionStoreEntry]
      ) -> None: ...
      async def load(self, key: SessionKey) -> list[SessionStoreEntry] | None: ...

      # Optional — omit or raise NotImplementedError
      async def list_sessions(
          self, project_key: str
      ) -> list[SessionStoreListEntry]: ...
      async def delete(self, key: SessionKey) -> None: ...
      async def list_subkeys(self, key: SessionListSubkeysKey) -> list[str]: ...
  ```
</CodeGroup>

`SessionKey`는 하나의 기록을 주소 지정합니다. `projectKey`는 작업 디렉토리의 안정적이고 파일 시스템에 안전한 인코딩이고, `sessionId`는 세션 UUID이며, `subpath`는 항목이 하위 에이전트 기록 또는 사이드카 파일이 아닌 주 대화에 속할 때 설정됩니다. `subpath`를 불투명한 키 접미사로 취급합니다. 예를 들어 `subagents/agent-<id>`와 같이 온디스크 레이아웃을 따릅니다. `subpath`가 정의되지 않으면 키는 주 기록을 참조합니다.

| 메서드            | 필수  | 호출 시기                                                                                                                                     |
| :------------- | :-- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| `append`       | 예   | 기록 항목의 각 배치가 로컬에 작성된 후. 항목은 JSON 안전 객체이며 로컬 JSONL에서 한 줄에 하나씩입니다.                                                                          |
| `load`         | 예   | 서브프로세스가 생성되기 전에 한 번, `resume`이 설정되었을 때. 세션을 알 수 없으면 `null`을 반환합니다.                                                                        |
| `listSessions` | 아니오 | `listSessions({ sessionStore })`에 의해 그리고 `continue: true`를 사용하는 `query()`/`startup()`에 의해. 정의되지 않으면 이러한 호출은 예외를 발생시킵니다.                   |
| `delete`       | 아니오 | `deleteSession({ sessionStore })`에 의해. 주 키 삭제(no `subpath`)는 해당 세션의 모든 하위 키로 계단식으로 전파되어야 합니다. 정의되지 않으면 삭제는 작동하지 않으며, 이는 추가 전용 백엔드에 적합합니다. |
| `listSubkeys`  | 아니오 | 재개 중에 하위 에이전트 기록을 발견하기 위해. 정의되지 않으면 주 기록만 복원됩니다.                                                                                          |

<h2 id="quick-start">
  빠른 시작
</h2>

SDK는 개발 및 테스트를 위해 `InMemorySessionStore`를 제공합니다. 아래 예제는 저장소가 연결된 쿼리를 실행하고, 결과 메시지에서 세션 ID를 캡처한 다음, 두 번째 `query()` 호출에서 저장소에서 재개합니다. 두 번째 호출은 동일한 저장소 인스턴스와 `resume`을 전달하므로 SDK는 로컬 파일 시스템 대신 저장소에서 기록을 로드합니다:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, InMemorySessionStore } from "@anthropic-ai/claude-agent-sdk";

  const store = new InMemorySessionStore();

  let sessionId: string | undefined;
  for await (const message of query({
    prompt: "List the TypeScript files under src/",
    options: { sessionStore: store },
  })) {
    if (message.type === "result") {
      sessionId = message.session_id;
    }
  }

  // Resume from the store. The agent has full context from the first call.
  for await (const message of query({
    prompt: "Summarize what those files do",
    options: { sessionStore: store, resume: sessionId },
  })) {
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import (
      ClaudeAgentOptions,
      InMemorySessionStore,
      ResultMessage,
      query,
  )

  store = InMemorySessionStore()


  async def main():
      session_id = None
      async for message in query(
          prompt="List the Python files under src/",
          options=ClaudeAgentOptions(session_store=store),
      ):
          if isinstance(message, ResultMessage):
              session_id = message.session_id

      # Resume from the store. The agent has full context from the first call.
      async for message in query(
          prompt="Summarize what those files do",
          options=ClaudeAgentOptions(session_store=store, resume=session_id),
      ):
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```
</CodeGroup>

두 번째 쿼리는 첫 번째 쿼리의 파일 요약을 출력하며, 이는 에이전트가 저장소에서 전체 컨텍스트와 함께 재개되었음을 보여줍니다.

<h2 id="write-your-own-adapter">
  자신의 어댑터 작성하기
</h2>

백엔드에 대해 `append`와 `load`를 구현합니다. 저장소에 대해 `listSessions()`, `deleteSession()` 및 하위 에이전트 재개가 작동하도록 하려면 `listSessions`, `delete` 및 `listSubkeys`를 추가합니다.

`append`에 전달된 항목은 `SessionStoreEntry`로 입력됩니다(`{ type: string; ... }` 객체). 이를 불투명한 JSON 안전 값으로 취급합니다: 순서대로 유지하고 `load`에서 동일한 순서로 반환합니다. `load`는 추가된 항목과 깊이 같은 항목을 반환해야 합니다. 바이트 같은 직렬화는 필요하지 않으므로 Postgres `jsonb`와 같이 객체 키를 재정렬하는 백엔드는 괜찮습니다.

<h2 id="reference-implementations">
  참조 구현
</h2>

TypeScript SDK 저장소에는 [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores) 아래 S3, Redis 및 Postgres용 실행 가능한 참조 어댑터가 포함되어 있습니다. 이들은 npm에 게시되지 않습니다. 필요한 `src/` 파일을 프로젝트에 복사하고 해당 백엔드 클라이언트를 설치합니다.

| 어댑터                                                                                                                            | 백엔드 클라이언트            | 저장소 모델                                                 |
| :----------------------------------------------------------------------------------------------------------------------------- | :------------------- | :----------------------------------------------------- |
| [`S3SessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/s3)             | `@aws-sdk/client-s3` | `append()`당 하나의 JSONL 부분 파일; `load()`는 나열, 정렬 및 연결합니다. |
| [`RedisSessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/redis)       | `ioredis`            | 기록당 `RPUSH`/`LRANGE` 목록, 그리고 정렬된 세트 세션 인덱스.            |
| [`PostgresSessionStore`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/postgres) | `pg`                 | `jsonb` 테이블의 항목당 하나의 행, `BIGSERIAL`로 정렬됨.              |

각 어댑터는 사전 구성된 클라이언트 인스턴스를 사용하므로 자격 증명, TLS, 지역 및 풀링을 제어합니다. 예를 들어 S3의 경우:

```typescript TypeScript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";
import { S3Client } from "@aws-sdk/client-s3";
import { S3SessionStore } from "./S3SessionStore"; // copied from examples/session-stores/s3

const store = new S3SessionStore({
  bucket: "my-claude-sessions",
  prefix: "transcripts",
  client: new S3Client({ region: "us-east-1" }),
});

for await (const message of query({
  prompt: "Hello!",
  options: { sessionStore: store },
})) {
  if (message.type === "result" && message.subtype === "success") {
    console.log(message.result);
  }
}

// Later, possibly on a different host:
for await (const message of query({
  prompt: "Continue where we left off",
  options: { sessionStore: store, resume: "previous-session-id" },
})) {
  // ...
}
```

<h3 id="validate-your-adapter">
  어댑터 검증하기
</h3>

두 SDK 모두 `append`, `load` 및 선택적 메서드가 만족해야 하는 동작 계약을 주장하는 적합성 제품군을 제공합니다. 선택적 메서드에 대한 테스트는 해당 메서드가 구현되지 않으면 자동으로 건너뜁니다.

TypeScript에서 예제 디렉토리의 [`shared/conformance.ts`](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/examples/session-stores/shared/conformance.ts)를 테스트 제품군에 복사합니다. Python에서는 제품군이 패키지에 포함됩니다:

```python Python theme={null}
import pytest
from claude_agent_sdk.testing import run_session_store_conformance


@pytest.mark.asyncio
async def test_my_store_conformance():
    await run_session_store_conformance(MyRedisStore)
```

<h2 id="behavior-notes">
  동작 참고 사항
</h2>

<h3 id="dual-write-architecture">
  이중 쓰기 아키텍처
</h3>

저장소는 미러이지 대체가 아닙니다. Claude Code 서브프로세스는 항상 로컬 디스크에 먼저 쓰고, SDK는 각 배치를 `append()`로 전달합니다. 로컬 복사본을 임시적으로 만들려면 `options.env`의 임시 디렉토리에 `CLAUDE_CONFIG_DIR`을 가리킵니다. 미러는 로컬 쓰기에 따라 달라지므로 `sessionStore`는 `persistSession: false`와 결합할 수 없습니다. 둘 다 설정하면 SDK는 예외를 발생시킵니다. 또한 `enableFileCheckpointing`과 결합할 수 없습니다. 파일 기록 백업 블롭은 로컬 디스크에 직접 작성되고 저장소로 미러링되지 않기 때문입니다.

<h3 id="mirror-writes-are-best-effort">
  미러 쓰기는 최선의 노력입니다
</h3>

`append()`가 거부하면 SDK는 짧은 백오프를 사용하여 배치를 최대 2회 더 재시도하며, 총 최대 3회 시도합니다. 시간 초과된 호출은 재시도되지 않습니다. 원본 호출이 여전히 도착할 수 있기 때문입니다. 배치가 여전히 실패하면 오류가 기록되고, `{ type: "system", subtype: "mirror_error" }` 메시지가 반복자로 내보내지며, 배치는 삭제되고 쿼리가 계속됩니다. 로컬 기록은 이미 디스크에 내구적이므로 저장소 중단은 에이전트를 중단하거나 로컬에서 데이터를 손실하지 않습니다. 저장소 데이터 손실을 감지해야 하는 경우 `mirror_error`를 모니터링합니다. 재시도된 배치는 이미 도착한 항목을 다시 전달할 수 있으므로 `append()` 구현에서 `entry.uuid`로 중복을 제거합니다.

<h3 id="getsessionmessages-returns-the-post-compaction-chain">
  `getSessionMessages`는 압축 후 체인을 반환합니다
</h3>

`getSessionMessages({ sessionStore })`는 에이전트가 재개할 때 볼 수 있는 연결된 메시지 체인을 반환합니다. 자동 압축 후 이전 턴은 요약으로 대체되므로 저장소에 503개의 원본 항목을 보유한 세션은 `getSessionMessages`에서 18개의 메시지를 반환할 수 있습니다. 압축 전 턴 및 메타데이터 항목을 포함한 전체 원본 기록의 경우 `store.load(key)`를 직접 호출합니다.

<h3 id="forksession-is-not-a-byte-copy">
  `forkSession`은 바이트 복사가 아닙니다
</h3>

`forkSession({ sessionStore })`는 원본 항목을 읽고, 모든 `sessionId` 필드를 다시 작성하고 메시지 UUID를 재매핑한 다음, 변환된 항목을 새 키 아래에 추가합니다. 어댑터 수준 복사 또는 `CopyObject` 바로 가기는 여전히 이전 세션 ID를 참조하는 기록을 생성하므로 SDK는 이를 사용하지 않습니다.

<h3 id="subagent-transcripts">
  하위 에이전트 기록
</h3>

하위 에이전트 기록은 `subpath: "subagents/agent-<id>"` 아래에 미러링됩니다. `listSubagents({ sessionStore })`는 어댑터가 `listSubkeys`를 구현해야 합니다. `getSubagentMessages({ sessionStore })`는 사용 가능할 때 이를 사용하지만 정의되지 않으면 직접 하위 경로로 폴백합니다. 재개도 `listSubkeys`를 호출하여 하위 에이전트 파일을 복원합니다. 이것이 없으면 주 기록만 구체화됩니다.

<h3 id="retention">
  보존
</h3>

SDK는 자체적으로 저장소에서 삭제하지 않습니다. 보존은 어댑터의 책임입니다: TTL, S3 수명 주기 정책 또는 규정 준수 요구 사항에 따른 예약된 정리를 구현합니다. `CLAUDE_CONFIG_DIR` 아래의 로컬 기록은 `cleanupPeriodDays` 설정에 의해 독립적으로 정리됩니다.

<h2 id="supported-on">
  지원 대상
</h2>

다음 SDK 함수는 `sessionStore` 옵션을 수락하고 제공될 때 로컬 파일 시스템 대신 저장소에 대해 작동합니다:

* [`query()`](/docs/ko/agent-sdk/typescript#query)
* [`startup()`](/docs/ko/agent-sdk/typescript#startup)
* [`listSessions()`](/docs/ko/agent-sdk/typescript#listsessions)
* [`getSessionInfo()`](/docs/ko/agent-sdk/typescript#getsessioninfo)
* [`getSessionMessages()`](/docs/ko/agent-sdk/typescript#getsessionmessages)
* [`renameSession()`](/docs/ko/agent-sdk/typescript#renamesession)
* [`tagSession()`](/docs/ko/agent-sdk/typescript#tagsession)
* [`deleteSession()`](/docs/ko/agent-sdk/typescript)
* [`forkSession()`](/docs/ko/agent-sdk/typescript)
* [`listSubagents()`](/docs/ko/agent-sdk/typescript)
* [`getSubagentMessages()`](/docs/ko/agent-sdk/typescript)

<h2 id="related-resources">
  관련 리소스
</h2>

* [세션 작업하기](/docs/ko/agent-sdk/sessions): 사용자 정의 저장소 없이 계속, 재개 및 포크
* [SDK 호스팅하기](/docs/ko/agent-sdk/hosting): 다중 호스트 환경을 위한 배포 패턴
* [TypeScript `Options`](/docs/ko/agent-sdk/typescript#options): 전체 옵션 참조
* [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores): 실행 가능한 S3, Redis 및 Postgres 참조 어댑터
