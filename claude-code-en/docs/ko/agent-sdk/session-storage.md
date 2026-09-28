> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 세션을 외부 스토리지에 유지하기

> Agent SDK 세션 트랜스크립트를 자신의 객체 저장소, 키-값 저장소 또는 데이터베이스에 미러링하여 다른 호스트에서 세션을 재개할 수 있습니다.

기본적으로 SDK는 세션 트랜스크립트를 로컬 파일시스템의 `~/.claude/projects/` 아래 JSONL 파일로 작성합니다. `SessionStore` 어댑터를 사용하면 이러한 트랜스크립트를 객체 저장소, 키-값 저장소 또는 데이터베이스와 같은 자신의 백엔드에 미러링할 수 있으므로, 한 호스트에서 생성된 세션을 일치하는 작업 디렉토리에서 실행 중인 다른 호스트에서 재개할 수 있습니다.

세션 저장소를 사용하는 일반적인 이유:

* **다중 호스트 배포.** 서버리스 함수, 자동 확장 워커 및 CI 러너는 파일시스템을 공유하지 않습니다. 공유 저장소를 사용하면 복제본이 서로의 세션을 재개할 수 있습니다.
* **내구성.** 로컬 컨테이너는 임시적입니다. 외부 저장소는 재시작 및 재배포를 견딜 수 있습니다.
* **규정 준수 및 감사.** 이미 관리하고 있는 저장소에 트랜스크립트를 유지하고, 자신의 보존 규칙, 암호화 및 접근 제어를 적용할 수 있습니다.

<h2 id="the-sessionstore-interface">
  `SessionStore` 인터페이스
</h2>

`SessionStore`는 두 개의 필수 메서드 `append`와 `load`, 그리고 네 개의 선택적 메서드를 가진 객체입니다. SDK는 쿼리 중에 기록 항목을 작성하기 위해 `append`를 호출하고 재개를 위해 이를 다시 읽기 위해 `load`를 호출합니다.

<CodeGroup>
  ```typescript TypeScript theme={null}
  // Exported from @anthropic-ai/claude-agent-sdk as
  // SessionStore, SessionKey, SessionStoreEntry, SessionSummaryEntry.

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
    listSessionSummaries?(projectKey: string): Promise<SessionSummaryEntry[]>;
    delete?(key: SessionKey): Promise<void>;
    listSubkeys?(key: {
      projectKey: string;
      sessionId: string;
    }): Promise<string[]>;
  };

  type SessionSummaryEntry = {
    sessionId: string;
    mtime: number;
    data: Record<string, unknown>;
  };
  ```

  ```python Python theme={null}
  # Exported from claude_agent_sdk as
  # SessionStore, SessionKey, SessionStoreEntry, SessionSummaryEntry.

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
      async def list_session_summaries(
          self, project_key: str
      ) -> list[SessionSummaryEntry]: ...
      async def delete(self, key: SessionKey) -> None: ...
      async def list_subkeys(self, key: SessionListSubkeysKey) -> list[str]: ...

  class SessionSummaryEntry(TypedDict):
      session_id: str
      mtime: int
      data: dict[str, Any]
  ```
</CodeGroup>

`SessionKey`는 하나의 기록을 주소 지정합니다. `projectKey`는 작업 디렉토리의 안정적이고 파일 시스템에 안전한 인코딩이고, `sessionId`는 세션 UUID이며, `subpath`는 항목이 하위 에이전트 기록 또는 사이드카 파일이 아닌 주 대화에 속할 때 설정됩니다.

`projectKey`가 작업 디렉토리를 인코딩하므로 원래 실행의 작업 디렉토리와 일치하는 작업 디렉토리에서 저장소를 재개하거나 계속합니다. TypeScript에서 쿼리의 [`env` 옵션](/docs/ko/agent-sdk/typescript#options)에서 `CLAUDE_CONFIG_DIR` 옆에 [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/ko/sessions#name-the-project-directory-yourself)을 설정하면 SDK는 해당 쿼리의 항목과 `resume` 및 `continue` 조회를 대신 그 이름으로 키 지정합니다. `listSessions` 및 `deleteSession`과 같은 독립 실행형 헬퍼는 `env`를 사용하지 않고 프로세스 환경을 읽으므로 호스트 프로세스 환경에서도 `CLAUDE_CONFIG_DIR`과 동일한 이름을 설정합니다. Agent SDK v0.3.234 이상이 필요합니다.

`subpath`를 불투명한 키 접미사로 취급합니다. 예를 들어 `subagents/agent-<id>`와 같이 온디스크 레이아웃을 따릅니다. `subpath`가 정의되지 않으면 키는 주 기록을 참조합니다.

| 메서드                    | 필수  | 호출 시기                                                                                                                                                                                                           |
| :--------------------- | :-- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `append`               | 예   | 기록 항목의 각 배치가 로컬에 작성된 후. 항목은 JSON 안전 객체이며 로컬 JSONL에서 한 줄에 하나씩입니다.                                                                                                                                                |
| `load`                 | 예   | 서브프로세스가 생성되기 전에 `resume`이 설정되었을 때 또는 `continue: true`가 최신 저장소 세션을 해결할 때, 그리고 나열이 `listSessionSummaries`에서 폴백할 때 세션당 한 번. 세션을 알 수 없으면 `null`을 반환합니다.                                                             |
| `listSessions`         | 아니오 | `listSessions({ sessionStore })`에 의해 그리고 `continue: true`를 사용하는 `query()`/`startup()`에 의해. 정의되지 않으면 `continue: true`는 예외를 발생시키고, `listSessionSummaries`가 구현되지 않으면 `listSessions({ sessionStore })`는 예외를 발생시킵니다. |
| `listSessionSummaries` | 아니오 | `listSessions({ sessionStore })`에 의해 한 번의 호출로 모든 세션의 메타데이터를 읽기 위해. `append` 내에서 요약을 유지합니다. 정의되지 않으면 나열은 `listSessions`와 세션당 `load`로 폴백합니다.                                                                      |
| `delete`               | 아니오 | `deleteSession({ sessionStore })`에 의해. 주 키 삭제(`subpath` 없음)는 해당 세션의 모든 하위 키로 계단식으로 전파되어야 하며 세션의 요약 항목도 제거해야 하므로 삭제된 세션은 `listSessionSummaries`에 더 이상 나타나지 않습니다. 정의되지 않으면 삭제는 작동하지 않으며, 이는 추가 전용 백엔드에 적합합니다.     |
| `listSubkeys`          | 아니오 | 재개 중에 하위 에이전트 기록을 발견하기 위해. 정의되지 않으면 주 기록만 복원됩니다.                                                                                                                                                                |

`SessionSummaryEntry`에서 `mtime`은 사이드카의 저장소 쓰기 시간이며 `listSessions`이 반환하는 `mtime` 값과 동일한 클록 소스를 공유해야 합니다. `data`는 불투명한 SDK 소유 상태입니다. 이를 해석하지 않고 그대로 유지합니다.

`append` 내의 각 배치에서 내보낸 `foldSessionSummary` 헬퍼(Python에서는 `fold_session_summary`)를 호출하여 항목을 빌드합니다. `subpath`가 있는 키의 배치는 건너뜁니다. 하위 에이전트 기록은 주 세션의 요약에 기여하면 안 됩니다. 폴드는 `mtime`을 설정하지 않습니다. TypeScript에서는 `options.mtime` 인수를 통해 또는 Python에서 반환된 항목의 필드를 덮어써서 지속 시간에 타임스탬프를 지정합니다. 동일한 세션에 대한 동시 `append` 호출은 사이드카에서 경합할 수 있으므로 트랜잭션, 비교 및 교환 또는 세션당 잠금으로 읽기-폴드-쓰기를 직렬화합니다. 폴드 자체는 순수합니다.

SDK가 기록 `load`가 반환하는 것으로 수행하는 작업에 대해서는 [저장소에서 재개](#resume-from-the-store)를 참조합니다.

<h2 id="quick-start">
  빠른 시작
</h2>

SDK는 개발 및 테스트를 위해 `InMemorySessionStore`를 제공합니다. 아래 예제는 저장소가 연결된 쿼리를 실행하고, 결과 메시지에서 세션 ID를 캡처한 다음, 두 번째 `query()` 호출에서 저장소에서 재개합니다. 두 번째 호출은 동일한 저장소 인스턴스와 `resume`을 전달하므로 SDK는 로컬 파일 시스템 대신 저장소에서 기록을 로드합니다:

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query, InMemorySessionStore } from "@anthropic-ai/claude-agent-sdk";

  const store = new InMemorySessionStore();

  let sessionId: string | undefined;
  try {
    for await (const message of query({
      prompt: "List the TypeScript files under src/",
      options: { sessionStore: store },
    })) {
      if (message.type === "result") {
        sessionId = message.session_id;
      }
    }
  } catch (error) {
    // A single-shot query() throws after yielding an error result. If the
    // failure was an error result, sessionId was already captured by the loop
    // above; connection or process failures yield no result message.
    console.error(`Session ended with an error: ${error}`);
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
      try:
          async for message in query(
              prompt="List the Python files under src/",
              options=ClaudeAgentOptions(session_store=store),
          ):
              if isinstance(message, ResultMessage):
                  session_id = message.session_id
      except Exception as error:
          # A single-shot query() raises after yielding an error result. If the
          # failure was an error result, session_id was already captured by the
          # loop above; connection or process failures yield no result message.
          print(f"Session ended with an error: {error}")

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

백엔드에 대해 `append`와 `load`를 구현합니다. 저장소에 대해 `listSessions()`, 일회성 메타데이터 읽기, `deleteSession()` 및 하위 에이전트 재개가 작동하도록 하려면 `listSessions`, `listSessionSummaries`, `delete` 및 `listSubkeys`를 추가합니다.

`append`에 전달된 항목은 `SessionStoreEntry`로 입력됩니다(`{ type: string; ... }` 객체). 이를 불투명한 JSON 안전 값으로 취급합니다: 순서대로 유지하고 `load`에서 동일한 순서로 반환합니다. `load`는 추가된 항목과 깊이 같은 항목을 반환해야 합니다. 바이트 같은 직렬화는 필요하지 않으므로 객체 키를 재정렬하는 이진 JSON 열 유형과 같은 백엔드는 괜찮습니다.

<h2 id="reference-implementations">
  참조 구현
</h2>

두 SDK 저장소 모두 TypeScript의 [`examples/session-stores/`](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores)와 Python의 [`examples/session_stores/`](https://github.com/anthropics/claude-agent-sdk-python/tree/main/examples/session_stores) 아래에 실행 가능한 참조 어댑터를 포함하고 있습니다. 저장소 유형당 하나의 어댑터가 있으며, 각각은 `append`와 `load`가 해당 종류의 백엔드에 어떻게 매핑되는지 보여줍니다. 이들은 패키지로 게시되지 않습니다. 백엔드에 가장 가까운 유형의 어댑터를 프로젝트에 복사하고, 백엔드의 클라이언트를 설치한 후 이를 조정합니다.

| 저장소 유형               | 저장소 모델                                                        | 예제 어댑터                                                                                                                                                                                                                                                     |
| :------------------- | :------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 객체 저장소               | `append()`당 하나의 부분 파일; `load()`는 부분을 나열하고, 정렬하고, 연결합니다.       | S3 ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/s3), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/s3_session_store.py))                   |
| 키-값 저장소              | `append()`가 푸시하고 `load()`가 범위로 읽는 기록당 하나의 목록, 그리고 정렬된 세션 인덱스. | Redis ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/redis), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/redis_session_store.py))          |
| 관계형 데이터베이스 또는 문서 저장소 | 항목당 하나의 행 또는 문서, JSON으로 저장되고 삽입 시 할당된 키로 정렬됨.                 | Postgres ([TypeScript](https://github.com/anthropics/claude-agent-sdk-typescript/tree/main/examples/session-stores/postgres), [Python](https://github.com/anthropics/claude-agent-sdk-python/blob/main/examples/session_stores/postgres_session_store.py)) |

각 어댑터는 사전 구성된 클라이언트 인스턴스를 사용하므로 자격 증명, TLS, 지역 및 풀링을 제어합니다. 다음 예제는 객체 저장소 어댑터를 `query()`에 연결한 후 다른 호스트에서 이를 재개합니다:

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

TypeScript에서 예제 디렉토리의 [`shared/conformance.ts`](https://github.com/anthropics/claude-agent-sdk-typescript/blob/main/examples/session-stores/shared/conformance.ts)를 테스트 제품군에 복사합니다. Python에서는 제품군이 패키지에 포함됩니다. pytest를 실행하려면 pytest를 먼저 설치합니다(SDK 종속성이 아님):

```bash theme={null}
pip install pytest
```

그런 다음 어댑터를 제품군에 전달합니다. 테스트 파일에서 0개 인수 팩토리로 전달하면 `run_session_store_conformance`가 계약당 한 번씩 호출하여 새로운 저장소를 구축합니다:

```python Python theme={null}
import pytest
from claude_agent_sdk.testing import run_session_store_conformance


@pytest.mark.anyio
async def test_my_store_conformance():
    await run_session_store_conformance(MyRedisStore)
```

이 예제처럼 `MyRedisStore` 클래스 자체를 전달하면 생성자가 인수를 사용하지 않을 때 작동합니다. 사전 구성된 클라이언트를 사용하는 어댑터의 경우 대신 저장소를 구성하는 람다를 전달합니다. 계약이 동일한 세션 키를 재사용하므로 팩토리가 반환하는 각 저장소는 빈 저장소로 시작해야 합니다. 람다가 호출당 격리된 백업 저장소를 프로비저닝하도록 하십시오. 예를 들어 새로운 메모리 내 가짜, 고유한 키 접두사 또는 새로운 테스트 데이터베이스입니다.

<h2 id="behavior-notes">
  동작 참고 사항
</h2>

<h3 id="dual-write-architecture">
  이중 쓰기 아키텍처
</h3>

Claude Code 서브프로세스는 항상 각 배치의 기록 항목을 먼저 로컬 디스크에 쓰고, SDK는 동일한 배치를 저장소의 `append()`로 전달하므로, 저장소는 로컬 기록의 미러이지 대체가 아닙니다. 어느 복사본이 실행을 초과하는지는 실행이 어떻게 시작되었는지에 따라 달라집니다:

* **새 세션 또는 저장소가 세션에 대해 아무것도 없을 때의 재개**: 설정 디렉토리 아래의 로컬 기록이 실행을 초과하고, 저장소는 복사본을 받습니다.
* **[저장소에서 재개된](#resume-from-the-store) 실행**: 로컬 복사본은 실행 끝에 삭제되므로, 저장소가 유일한 내구적 복사본을 보유합니다.

새 세션이 로컬 디스크에 기록을 남기지 않도록 하려면 `options.env`에서 `CLAUDE_CONFIG_DIR`을 임시 디렉토리로 설정합니다. 저장소에서 재개된 실행은 이미 로컬 복사본을 삭제하므로 이러한 설정이 필요하지 않습니다. TypeScript에서는 [`env` 옵션](/docs/ko/agent-sdk/typescript#options)이 서브프로세스 환경을 대체하므로 `process.env`를 `env`로도 전개합니다.

앱이 설정 디렉토리의 파일(예: OAuth 자격 증명 또는 사용자 `settings.json`의 `apiKeyHelper`)을 통해 로그인하는 경우, 먼저 해당 파일을 임시 디렉토리에 복사하거나 대신 `env`에서 `ANTHROPIC_API_KEY`를 설정합니다. 그렇지 않으면 실행이 `Not logged in`으로 실패합니다.

두 옵션이 미러와 충돌하며, 저장소와 함께 둘 중 하나를 결합하면 SDK는 시작 시 예외를 발생시킵니다:

* **TypeScript의 `persistSession: false`**: 미러가 구축되는 로컬 쓰기를 끕니다. Python SDK에는 동등한 옵션이 없습니다.
* **파일 체크포인팅**, TypeScript의 `enableFileCheckpointing` 또는 Python의 `enable_file_checkpointing`: 파일 백업을 로컬 디스크에 직접 쓰고, SDK는 이를 저장소로 미러링하지 않습니다.

<h3 id="resume-from-the-store">
  저장소에서 재개
</h3>

저장소와 함께 `resume`을 전달하거나, TypeScript의 `continue: true` 또는 Python의 `continue_conversation=True`를 전달하면, SDK는 서브프로세스를 생성하기 전에 저장소에 기록을 요청합니다:

* **`resume`**: SDK는 전달한 ID의 세션을 요청합니다.
* **`continue: true`** 또는 **`continue_conversation=True`**: SDK는 저장소의 최신 세션을 요청합니다.

저장소가 기록을 반환하면, SDK는 이를 임시 설정 디렉토리에 쓰고, `CLAUDE_CONFIG_DIR`이 거기를 가리키도록 서브프로세스를 실행하며, 실행이 끝날 때 디렉토리를 삭제합니다. 해당 실행이 쓰는 로컬 기록은 함께 삭제되므로, 저장소가 이 경로에서 유일한 내구적 복사본을 보유하는 이유입니다.

SDK는 또한 실제 설정 디렉토리의 파일로 임시 디렉토리를 시드합니다. 복사되는 내용은 언어에 따라 다릅니다:

* **TypeScript**: 자격 증명, `.claude.json`, 및 사용자 `settings.json`. `settings.json`에서 임시 설정 디렉토리에서 잘못 작동하는 키를 제거합니다: `enabledPlugins`, `extraKnownMarketplaces`, 해당 [`additionalMarketplaces`](/docs/ko/settings-reference#extraknownmarketplaces) 별칭, 및 파일의 `env` 블록의 모든 `CLAUDE_CONFIG_DIR`. Agent SDK v0.3.232 이전에는 SDK가 별칭을 제거하지 않았습니다. 설정에서 구성된 인증(예: [`apiKeyHelper`](/docs/ko/settings-reference#apikeyhelper))은 저장소에서 재개할 때 작동합니다. Agent SDK v0.3.222 이전에는 TypeScript SDK가 자격 증명과 `.claude.json`만 복사했습니다.
* **Python**: 자격 증명과 `.claude.json`만 복사하므로, 사용자 `settings.json`의 `apiKeyHelper`를 통해 인증하는 앱은 저장소에서 재개할 때 `Not logged in`으로 실패합니다. 관리되거나 프로젝트 설정의 `apiKeyHelper`는 여전히 작동합니다. Claude Code가 `CLAUDE_CONFIG_DIR`이 영향을 주지 않는 위치에서 해당 파일을 읽기 때문입니다.

저장소가 세션에 대해 아무것도 없을 때, SDK는 실제 설정 디렉토리에서 실행되고, 결과는 전달한 옵션에 따라 달라집니다:

* **`resume`**: 두 SDK 모두 ID를 서브프로세스로 전달하며, 이는 저장소 없이 `resume`이 하는 것과 정확히 같이 로컬 기록을 재개합니다.
* **TypeScript의 `continue: true`**: SDK는 새 세션을 시작합니다.
* **Python의 `continue_conversation=True`**: SDK는 최신 로컬 세션에서 계속합니다.

<h3 id="mirror-writes-are-best-effort">
  미러 쓰기는 최선의 노력입니다
</h3>

`append()`가 거부하면, SDK는 짧은 백오프를 사용하여 배치를 최대 2회 더 재시도하며, 총 최대 3회 시도합니다. 시간 초과된 호출은 재시도되지 않습니다. 원본 호출이 여전히 도착할 수 있기 때문입니다. 배치가 여전히 실패하면, SDK는 오류를 기록하고, `{ type: "system", subtype: "mirror_error" }` 메시지를 반복자로 내보내며, 배치를 삭제하고 쿼리를 계속합니다. 재시도된 배치는 이미 도착한 항목을 다시 전달할 수 있으므로, `append()` 구현에서 `entry.uuid`로 중복을 제거합니다.

저장소 중단이 에이전트를 중단하지 않습니다. 서브프로세스가 먼저 로컬에 쓰기 때문입니다. 저장소 데이터 손실을 감지해야 하는 경우 `mirror_error`를 모니터링합니다. [저장소에서 재개된](#resume-from-the-store) 실행에서, 삭제된 배치는 실행이 끝나면 생존하는 복사본이 없습니다.

<h3 id="getsessionmessages-returns-the-post-compaction-chain">
  `getSessionMessages`는 압축 후 체인을 반환합니다
</h3>

`getSessionMessages({ sessionStore })`는 에이전트가 재개할 때 볼 수 있는 연결된 메시지 체인을 반환합니다. 자동 압축 후, 이전 턴은 요약으로 대체되므로, 저장소가 503개의 원본 항목을 보유한 세션은 `getSessionMessages`에서 18개의 메시지를 반환할 수 있습니다. 압축 전 턴 및 메타데이터 항목을 포함한 전체 원본 기록의 경우, `store.load(key)`를 직접 호출합니다.

<h3 id="forksession-is-not-a-byte-copy">
  `forkSession`은 바이트 복사가 아닙니다
</h3>

`forkSession({ sessionStore })`는 원본 항목을 읽고, 모든 `sessionId` 필드를 다시 작성하고 메시지 UUID를 재매핑한 다음, 변환된 항목을 새 키 아래에 추가합니다. 어댑터 수준 복사 또는 `CopyObject` 바로 가기는 여전히 이전 세션 ID를 참조하는 기록을 생성하므로, SDK는 이를 사용하지 않습니다.

<h3 id="subagent-transcripts">
  하위 에이전트 기록
</h3>

하위 에이전트 기록은 `subpath: "subagents/agent-<id>"` 아래에 미러링됩니다. `listSubagents({ sessionStore })`는 어댑터가 `listSubkeys`를 구현해야 합니다. `getSubagentMessages({ sessionStore })`는 사용 가능할 때 이를 사용하지만 정의되지 않으면 직접 하위 경로로 폴백합니다. 재개도 `listSubkeys`를 호출하여 하위 에이전트 파일을 복원합니다. 이것이 없으면 주 기록만 구체화됩니다.

<h3 id="retention">
  보존
</h3>

SDK는 자체적으로 저장소에서 삭제하지 않습니다. 보존은 어댑터의 책임입니다: 백엔드의 만료 또는 수명 주기 메커니즘을 사용하거나, 규정 준수 요구 사항에 따라 예약된 정리를 실행합니다.

`CLAUDE_CONFIG_DIR` 아래의 로컬 기록은 `cleanupPeriodDays` 설정에 의해 독립적으로 정리되며, [보존 정리 규칙](/docs/ko/claude-directory#cleaned-up-automatically)을 따릅니다. [저장소에서 재개된](#resume-from-the-store) 실행은 로컬 기록을 남기지 않으므로, 해당 실행의 경우 저장소의 보존이 유일한 보존입니다.

<h2 id="supported-on">
  지원 대상
</h2>

다음 TypeScript SDK 함수는 `sessionStore` 옵션을 수락하고 제공될 때 로컬 파일 시스템 대신 저장소에 대해 작동합니다:

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

Python SDK에서는 [`ClaudeAgentOptions`](/docs/ko/agent-sdk/python#claudeagentoptions)에서 `session_store`를 설정하여 저장소에 대해 `query()`를 실행합니다. 나머지 작업은 각각 저장소를 인수로 사용하는 저장소 기반 Python 함수를 가집니다: `list_sessions_from_store()`, `get_session_info_from_store()`, `get_session_messages_from_store()`, `list_subagents_from_store()`, `get_subagent_messages_from_store()`, `rename_session_via_store()`, `tag_session_via_store()`, `delete_session_via_store()`, 및 `fork_session_via_store()`. `startup()`은 Python 동등물이 없습니다. [Python SDK 참조](/docs/ko/agent-sdk/python#functions)에 문서화된 `list_sessions()`과 같은 독립 실행형 함수는 로컬 세션 파일을 읽습니다.

<h2 id="related-resources">
  관련 리소스
</h2>

* [세션 작업하기](/docs/ko/agent-sdk/sessions): 사용자 정의 저장소 없이 계속, 재개 및 포크
* [SDK 호스팅하기](/docs/ko/agent-sdk/hosting): 다중 호스트 환경을 위한 배포 패턴
* [TypeScript `Options`](/docs/ko/agent-sdk/typescript#options): 전체 옵션 참조
* [참조 구현](#reference-implementations): 객체 저장소, 키-값 저장소 및 데이터베이스용 실행 가능한 예제 어댑터(두 SDK 저장소 모두에 포함)
