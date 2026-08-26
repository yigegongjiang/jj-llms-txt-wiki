> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# TypeScript SDK V2 세션 API (지원 중단됨)

> 다중 턴 대화를 위한 세션 기반 send/stream 패턴을 사용하는 지원 중단된 V2 TypeScript Agent SDK 세션 API 참조입니다.

<Warning>
  V2 세션 API는 더 이상 지원되지 않습니다. TypeScript Agent SDK 0.3.142는 `unstable_v2_createSession`, `unstable_v2_resumeSession`, `unstable_v2_prompt` 및 `SDKSession`과 `SDKSessionOptions` 타입을 제거합니다.

  마이그레이션하려면 [`query()` API](/docs/ko/agent-sdk/typescript)와 이를 허용하는 [세션 옵션](/docs/ko/agent-sdk/sessions)을 사용하십시오. 다중 턴 대화의 경우 `AsyncIterable<SDKUserMessage>`를 전달하거나 저장된 세션을 계속하려면 `options.resume`을 사용하십시오. 이 페이지는 Agent SDK 0.2.x 이전 버전의 코드를 유지 관리하는 경우 참조용으로 유지됩니다.
</Warning>

V2는 비동기 생성기와 yield 조정의 필요성을 제거한 실험적 세션 API였습니다. 턴 전체에서 생성기 상태를 관리하는 대신 각 턴은 별도의 `send()`/`stream()` 사이클이었습니다. API 표면은 세 가지 개념으로 축소되었습니다:

* `createSession()` / `resumeSession()`: 대화 시작 또는 계속
* `session.send()`: 메시지 전송
* `session.stream()`: 응답 받기

<h2 id="installation">
  설치
</h2>

Agent SDK 0.2.x는 V2 인터페이스를 포함하는 마지막 버전입니다. 패키지 버전은 0.2.x에서 0.3.142로 직접 점프했으므로, 위의 제거 버전과 아래의 설치 핀은 동일한 경계를 설명합니다. 마지막 V2 호환 릴리스를 설치하려면 주 버전과 부 버전을 고정하십시오:

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk@0.2
```

<Note>
  SDK는 선택적 종속성으로 플랫폼용 네이티브 Claude Code 바이너리를 번들로 제공하므로 Claude Code를 별도로 설치할 필요가 없습니다.
</Note>

<h2 id="quick-start">
  빠른 시작
</h2>

<h3 id="one-shot-prompt">
  일회성 프롬프트
</h3>

세션을 유지할 필요가 없는 간단한 단일 턴 쿼리의 경우 `unstable_v2_prompt()`를 사용합니다. 이 예제는 수학 질문을 보내고 답변을 기록합니다:

```typescript theme={null}
import { unstable_v2_prompt } from "@anthropic-ai/claude-agent-sdk";

const result = await unstable_v2_prompt("What is 2 + 2?", {
  model: "claude-opus-4-7"
});
if (result.subtype === "success") {
  console.log(result.result);
}
```

<details>
  <summary>V1에서 동일한 작업 보기</summary>

  ```typescript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const q = query({
    prompt: "What is 2 + 2?",
    options: { model: "claude-opus-4-7" }
  });

  for await (const msg of q) {
    if (msg.type === "result" && msg.subtype === "success") {
      console.log(msg.result);
    }
  }
  ```
</details>

<h3 id="basic-session">
  기본 세션
</h3>

단일 프롬프트를 넘어서는 상호작용의 경우 세션을 생성합니다. V2는 전송과 스트리밍을 별개의 단계로 분리합니다:

* `send()`는 메시지를 전달합니다
* `stream()`은 응답을 스트리밍합니다

이러한 명시적 분리를 통해 턴 사이에 로직을 추가하기가 더 쉬워집니다(예: 후속 메시지를 보내기 전에 응답 처리).

아래 예제는 세션을 생성하고, Claude에 "Hello!"를 보내고, 텍스트 응답을 출력합니다. 블록이 종료될 때 세션을 자동으로 닫기 위해 [`await using`](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-2.html#using-declarations-and-explicit-resource-management)(TypeScript 5.2+)을 사용합니다. `session.close()`를 수동으로 호출할 수도 있습니다.

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

await using session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});

await session.send("Hello!");
for await (const msg of session.stream()) {
  // Filter for assistant messages to get human-readable output
  if (msg.type === "assistant") {
    const text = msg.message.content
      .filter((block) => block.type === "text")
      .map((block) => block.text)
      .join("");
    console.log(text);
  }
}
```

<details>
  <summary>V1에서 동일한 작업 보기</summary>

  V1에서는 입력과 출력이 모두 단일 비동기 생성기를 통해 흐릅니다. 기본 프롬프트의 경우 유사해 보이지만 다중 턴 로직을 추가하려면 입력 생성기를 사용하도록 재구성해야 합니다.

  ```typescript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const q = query({
    prompt: "Hello!",
    options: { model: "claude-opus-4-7" }
  });

  for await (const msg of q) {
    if (msg.type === "assistant") {
      const text = msg.message.content
        .filter((block) => block.type === "text")
        .map((block) => block.text)
        .join("");
      console.log(text);
    }
  }
  ```
</details>

<h3 id="multi-turn-conversation">
  다중 턴 대화
</h3>

세션은 여러 교환 전체에서 컨텍스트를 유지합니다. 대화를 계속하려면 동일한 세션에서 `send()`를 다시 호출합니다. Claude는 이전 턴을 기억합니다.

이 예제는 수학 질문을 한 다음 이전 답변을 참조하는 후속 질문을 합니다:

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

await using session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});

// Turn 1
await session.send("What is 5 + 3?");
for await (const msg of session.stream()) {
  // Filter for assistant messages to get human-readable output
  if (msg.type === "assistant") {
    const text = msg.message.content
      .filter((block) => block.type === "text")
      .map((block) => block.text)
      .join("");
    console.log(text);
  }
}

// Turn 2
await session.send("Multiply that by 2");
for await (const msg of session.stream()) {
  if (msg.type === "assistant") {
    const text = msg.message.content
      .filter((block) => block.type === "text")
      .map((block) => block.text)
      .join("");
    console.log(text);
  }
}
```

<details>
  <summary>V1에서 동일한 작업 보기</summary>

  ```typescript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Must create an async iterable to feed messages
  async function* createInputStream() {
    yield {
      type: "user",
      session_id: "",
      message: { role: "user", content: [{ type: "text", text: "What is 5 + 3?" }] },
      parent_tool_use_id: null
    };
    // Must coordinate when to yield next message
    yield {
      type: "user",
      session_id: "",
      message: { role: "user", content: [{ type: "text", text: "Multiply by 2" }] },
      parent_tool_use_id: null
    };
  }

  const q = query({
    prompt: createInputStream(),
    options: { model: "claude-opus-4-7" }
  });

  for await (const msg of q) {
    if (msg.type === "assistant") {
      const text = msg.message.content
        .filter((block) => block.type === "text")
        .map((block) => block.text)
        .join("");
      console.log(text);
    }
  }
  ```
</details>

<h3 id="session-resume">
  세션 재개
</h3>

이전 상호작용에서 세션 ID가 있는 경우 나중에 재개할 수 있습니다. 이는 장기 실행 워크플로우나 애플리케이션 재시작 전체에서 대화를 유지해야 할 때 유용합니다.

이 예제는 세션을 생성하고, ID를 저장하고, 닫은 다음 대화를 재개합니다:

```typescript theme={null}
import {
  unstable_v2_createSession,
  unstable_v2_resumeSession,
  type SDKMessage
} from "@anthropic-ai/claude-agent-sdk";

// Helper to extract text from assistant messages
function getAssistantText(msg: SDKMessage): string | null {
  if (msg.type !== "assistant") return null;
  return msg.message.content
    .filter((block) => block.type === "text")
    .map((block) => block.text)
    .join("");
}

// Create initial session and have a conversation
const session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});

await session.send("Remember this number: 42");

// Get the session ID from any received message
let sessionId: string | undefined;
for await (const msg of session.stream()) {
  sessionId = msg.session_id;
  const text = getAssistantText(msg);
  if (text) console.log("Initial response:", text);
}

console.log("Session ID:", sessionId);
session.close();

// Later: resume the session using the stored ID
await using resumedSession = unstable_v2_resumeSession(sessionId!, {
  model: "claude-opus-4-7"
});

await resumedSession.send("What number did I ask you to remember?");
for await (const msg of resumedSession.stream()) {
  const text = getAssistantText(msg);
  if (text) console.log("Resumed response:", text);
}
```

<details>
  <summary>V1에서 동일한 작업 보기</summary>

  ```typescript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Create initial session
  const initialQuery = query({
    prompt: "Remember this number: 42",
    options: { model: "claude-opus-4-7" }
  });

  // Get session ID from any message
  let sessionId: string | undefined;
  for await (const msg of initialQuery) {
    sessionId = msg.session_id;
    if (msg.type === "assistant") {
      const text = msg.message.content
        .filter((block) => block.type === "text")
        .map((block) => block.text)
        .join("");
      console.log("Initial response:", text);
    }
  }

  console.log("Session ID:", sessionId);

  // Later: resume the session
  const resumedQuery = query({
    prompt: "What number did I ask you to remember?",
    options: {
      model: "claude-opus-4-7",
      resume: sessionId
    }
  });

  for await (const msg of resumedQuery) {
    if (msg.type === "assistant") {
      const text = msg.message.content
        .filter((block) => block.type === "text")
        .map((block) => block.text)
        .join("");
      console.log("Resumed response:", text);
    }
  }
  ```
</details>

<h3 id="cleanup">
  정리
</h3>

세션은 수동으로 닫거나 [`await using`](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-5-2.html#using-declarations-and-explicit-resource-management)을 사용하여 자동으로 닫을 수 있습니다. 이는 자동 리소스 정리를 위한 TypeScript 5.2+ 기능입니다. 이전 TypeScript 버전을 사용 중이거나 호환성 문제가 발생하면 대신 수동 정리를 사용합니다.

**자동 정리 (TypeScript 5.2+):**

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

await using session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});
// Session closes automatically when the block exits
```

**수동 정리:**

```typescript theme={null}
import { unstable_v2_createSession } from "@anthropic-ai/claude-agent-sdk";

const session = unstable_v2_createSession({
  model: "claude-opus-4-7"
});
// ... use the session ...
session.close();
```

<h2 id="api-reference">
  API 참조
</h2>

<h3 id="unstable_v2_createsession">
  `unstable_v2_createSession()`
</h3>

다중 턴 대화를 위한 새 세션을 생성합니다.

```typescript theme={null}
function unstable_v2_createSession(options: {
  model: string;
  // Additional options supported
}): SDKSession;
```

<h3 id="unstable_v2_resumesession">
  `unstable_v2_resumeSession()`
</h3>

ID로 기존 세션을 재개합니다.

```typescript theme={null}
function unstable_v2_resumeSession(
  sessionId: string,
  options: {
    model: string;
    // Additional options supported
  }
): SDKSession;
```

<h3 id="unstable_v2_prompt">
  `unstable_v2_prompt()`
</h3>

단일 턴 쿼리를 위한 일회성 편의 함수입니다.

```typescript theme={null}
function unstable_v2_prompt(
  prompt: string,
  options: {
    model: string;
    // Additional options supported
  }
): Promise<SDKResultMessage>;
```

<h3 id="sdksession-interface">
  SDKSession 인터페이스
</h3>

```typescript theme={null}
interface SDKSession {
  readonly sessionId: string;
  send(message: string | SDKUserMessage): Promise<void>;
  stream(): AsyncGenerator<SDKMessage, void>;
  close(): void;
}
```

<h2 id="feature-availability">
  기능 가용성
</h2>

V2 세션 API는 모든 V1 기능을 지원하지 않습니다. 다음은 [V1 SDK](/docs/ko/agent-sdk/typescript)가 필요합니다:

* 세션 포킹 (`forkSession` 옵션)
* 일부 고급 스트리밍 입력 패턴

<h2 id="see-also">
  참고 항목
</h2>

* [TypeScript SDK 참조 (V1)](/docs/ko/agent-sdk/typescript) - 전체 V1 SDK 문서
* [SDK 개요](/docs/ko/agent-sdk/overview) - 일반 SDK 개념
* [GitHub의 V2 예제](https://github.com/anthropics/claude-agent-sdk-demos/tree/main/hello-world-v2) - 작동하는 코드 예제
