> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Agent SDK 참조 - TypeScript

> TypeScript Agent SDK의 완전한 API 참조로, 모든 함수, 타입 및 인터페이스를 포함합니다.

<script src="/docs/components/typescript-sdk-type-links.js" defer />

<h2 id="installation">
  설치
</h2>

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk
```

<Note>
  SDK는 선택적 종속성으로 플랫폼용 네이티브 Claude Code 바이너리를 번들로 제공합니다(예: `@anthropic-ai/claude-agent-sdk-darwin-arm64`). 대부분의 설치에서는 Claude Code를 별도로 설치할 필요가 없습니다. SDK 버전은 번들된 Claude Code 버전을 추적합니다. SDK v0.3.191은 Claude Code v2.1.191을 번들로 제공하므로, 이 페이지의 특정 Claude Code 버전이 필요한 기능은 동일한 패치 번호 이상의 SDK 릴리스가 필요합니다. 패키지 관리자가 선택적 종속성을 건너뛰면 SDK는 `Native CLI binary for <platform>-<arch> not found` 오류를 발생시킵니다. 이 경우 [`pathToClaudeCodeExecutable`](#options)을 별도로 설치된 `claude` 바이너리로 설정하세요.

  패키지 관리자가 npm의 `libc` 필드를 적용하지 않으면(Yarn 1.x의 경우처럼), Linux에서 glibc 및 musl 플랫폼 패키지를 모두 받게 되어 설치 크기가 대략 두 배로 증가합니다. Agent SDK v0.2.141 이상에서는 SDK가 여전히 올바른 변형을 실행합니다. 컨테이너 이미지에서 공간을 확보하려면 앱이 실행되는 libc와 일치하지 않는 플랫폼 패키지를 삭제하세요. x64의 glibc 런타임의 경우 `rm -rf node_modules/@anthropic-ai/claude-agent-sdk-linux-x64-musl`입니다. 개발 머신에서는 Yarn이 다음 종속성 변경 시 패키지를 다시 설치하므로 삭제가 임시적입니다.
</Note>

<h3 id="compile-to-a-single-executable">
  단일 실행 파일로 컴파일
</h3>

`bun build --compile`을 사용하여 애플리케이션을 단일 파일 실행 파일로 컴파일하면 SDK는 런타임에 번들된 CLI 바이너리를 확인할 수 없습니다. `require.resolve`는 컴파일된 실행 파일의 `$bunfs` 가상 파일 시스템 내에서 작동하지 않으므로 SDK는 `Native CLI binary for <platform>-<arch> not found` 오류를 발생시킵니다.

이를 해결하려면 플랫폼 바이너리를 파일 자산으로 포함하고, 시작 시 `extractFromBunfs()`를 사용하여 실제 경로로 추출한 다음, 해당 경로를 [`pathToClaudeCodeExecutable`](#options)에 전달하세요.

`extractFromBunfs()` 헬퍼는 `@anthropic-ai/claude-agent-sdk` v0.3.144 이상이 필요합니다. 아래 예제는 Apple Silicon의 macOS용으로 빌드합니다:

```typescript theme={null}
import binPath from "@anthropic-ai/claude-agent-sdk-darwin-arm64/claude" with { type: "file" };
import { extractFromBunfs } from "@anthropic-ai/claude-agent-sdk/extract";
import { query } from "@anthropic-ai/claude-agent-sdk";

const cliPath = extractFromBunfs(binPath);

for await (const message of query({
  prompt: "Hello",
  options: { pathToClaudeCodeExecutable: cliPath },
})) {
  console.log(message);
}
```

`extractFromBunfs()`는 컴파일된 실행 파일의 가상 파일 시스템에서 포함된 바이너리를 사용자별 임시 디렉터리로 복사하고 실제 경로를 반환합니다. 컴파일된 실행 파일 외부에서는 입력 경로를 변경하지 않고 반환하므로 동일한 코드가 수정 없이 개발 환경에서 실행됩니다.

각 컴파일된 실행 파일은 단일 플랫폼의 바이너리를 포함합니다. 가져오기의 플랫폼 패키지를 `--target`과 일치시키세요:

* 크로스 컴파일하려면 일치하지 않는 플랫폼 패키지를 설치하세요. 예를 들어 `npm install @anthropic-ai/claude-agent-sdk-linux-x64 --force`입니다.
* Windows에서 바이너리 하위 경로는 `claude.exe`입니다. 예를 들어 `@anthropic-ai/claude-agent-sdk-win32-x64/claude.exe`입니다.

<h2 id="functions">
  함수
</h2>

<h3 id="query">
  `query()`
</h3>

Claude Code와 상호작용하기 위한 주요 함수입니다. 메시지가 도착하는 대로 스트리밍하는 비동기 생성기를 생성합니다.

```typescript theme={null}
function query({
  prompt,
  options
}: {
  prompt: string | AsyncIterable<SDKUserMessage>;
  options?: Options;
}): Query;
```

<h4 id="parameters">
  매개변수
</h4>

| 매개변수      | 유형                                                               | 설명                                    |
| :-------- | :--------------------------------------------------------------- | :------------------------------------ |
| `prompt`  | `string \| AsyncIterable<`[`SDKUserMessage`](#sdkusermessage)`>` | 문자열 또는 스트리밍 모드용 비동기 반복 가능 객체로 입력 프롬프트 |
| `options` | [`Options`](#options)                                            | 선택적 구성 객체 (아래 Options 유형 참조)          |

<h4 id="returns">
  반환값
</h4>

추가 메서드가 포함된 `AsyncGenerator<`[`SDKMessage`](#sdkmessage)`, void>`를 확장하는 [`Query`](#query-object) 객체를 반환합니다.

<h3 id="startup">
  `startup()`
</h3>

프롬프트를 사용할 수 있기 전에 CLI 부프로세스를 생성하고 초기화 핸드셰이크를 완료하여 미리 준비합니다. 반환된 [`WarmQuery`](#warmquery) 핸들은 나중에 프롬프트를 수락하고 이미 준비된 프로세스에 작성하므로 첫 번째 `query()` 호출이 부프로세스 생성 및 초기화 비용을 인라인으로 지불하지 않고 해결됩니다.

```typescript theme={null}
function startup(params?: {
  options?: Options;
  initializeTimeoutMs?: number;
}): Promise<WarmQuery>;
```

<h4 id="parameters-2">
  매개변수
</h4>

| 매개변수                  | 유형                    | 설명                                                                                      |
| :-------------------- | :-------------------- | :-------------------------------------------------------------------------------------- |
| `options`             | [`Options`](#options) | 선택적 구성 객체입니다. `query()`의 `options` 매개변수와 동일합니다                                          |
| `initializeTimeoutMs` | `number`              | 부프로세스 초기화를 기다릴 최대 시간(밀리초)입니다. 기본값은 `60000`입니다. 초기화가 시간 내에 완료되지 않으면 프로미스가 타임아웃 오류로 거부됩니다 |

<h4 id="returns-2">
  반환값
</h4>

부프로세스가 생성되고 초기화 핸드셰이크를 완료한 후 해결되는 `Promise<`[`WarmQuery`](#warmquery)`>`를 반환합니다.

<h4 id="example">
  예제
</h4>

`startup()`을 조기에 호출합니다(예: 애플리케이션 부팅 시). 그런 다음 프롬프트가 준비되면 반환된 핸들에서 `.query()`를 호출합니다. 이렇게 하면 부프로세스 생성 및 초기화가 중요 경로에서 제거됩니다.

```typescript theme={null}
import { startup } from "@anthropic-ai/claude-agent-sdk";

// 시작 비용을 미리 지불합니다
const warm = await startup({ options: { maxTurns: 3 } });

// 나중에 프롬프트가 준비되면 즉시 실행됩니다
for await (const message of warm.query("What files are here?")) {
  console.log(message);
}
```

<h3 id="tool">
  `tool()`
</h3>

SDK MCP 서버와 함께 사용할 유형 안전 MCP 도구 정의를 생성합니다.

```typescript theme={null}
function tool<Schema extends AnyZodRawShape>(
  name: string,
  description: string,
  inputSchema: Schema,
  handler: (args: InferShape<Schema>, extra: unknown) => Promise<CallToolResult>,
  extras?: { annotations?: ToolAnnotations; searchHint?: string; alwaysLoad?: boolean }
): SdkMcpToolDefinition<Schema>;
```

<h4 id="parameters-3">
  매개변수
</h4>

| 매개변수          | 유형                                                                                                     | 설명                                                                                                                                                                                                          |
| :------------ | :----------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | `string`                                                                                               | 도구의 이름                                                                                                                                                                                                      |
| `description` | `string`                                                                                               | 도구가 수행하는 작업에 대한 설명                                                                                                                                                                                          |
| `inputSchema` | `Schema extends AnyZodRawShape`                                                                        | 도구의 입력 매개변수를 정의하는 Zod 스키마 (Zod 3 및 Zod 4 모두 지원)                                                                                                                                                             |
| `handler`     | `(args, extra) => Promise<`[`CallToolResult`](#calltoolresult)`>`                                      | 도구 로직을 실행하는 비동기 함수                                                                                                                                                                                          |
| `extras`      | `{ annotations?: `[`ToolAnnotations`](#toolannotations)`; searchHint?: string; alwaysLoad?: boolean }` | 선택적 추가 항목입니다. `annotations`는 클라이언트에 MCP 동작 힌트를 제공합니다. `searchHint`는 [도구 검색](/docs/ko/agent-sdk/tool-search)이 활성화되어 있을 때 지연된 도구 목록에 표시되는 한 줄의 기능 구문입니다. `alwaysLoad: true`는 이 도구의 전체 스키마를 초기 프롬프트에 유지하고 지연하지 않습니다 |

<h4 id="toolannotations">
  `ToolAnnotations`
</h4>

`@modelcontextprotocol/sdk/types.js`에서 다시 내보냅니다. 모든 필드는 선택적 힌트이며 클라이언트는 보안 결정을 위해 이를 신뢰해서는 안 됩니다.

| 필드                | 유형        | 기본값         | 설명                                                                                |
| :---------------- | :-------- | :---------- | :-------------------------------------------------------------------------------- |
| `title`           | `string`  | `undefined` | 도구의 사람이 읽을 수 있는 제목                                                                |
| `readOnlyHint`    | `boolean` | `false`     | `true`인 경우 도구는 환경을 수정하지 않습니다                                                      |
| `destructiveHint` | `boolean` | `true`      | `true`인 경우 도구는 파괴적인 업데이트를 수행할 수 있습니다 (`readOnlyHint`가 `false`일 때만 의미 있음)          |
| `idempotentHint`  | `boolean` | `false`     | `true`인 경우 동일한 인수로 반복 호출해도 추가 효과가 없습니다 (`readOnlyHint`가 `false`일 때만 의미 있음)        |
| `openWorldHint`   | `boolean` | `true`      | `true`인 경우 도구는 외부 엔티티와 상호작용합니다 (예: 웹 검색). `false`인 경우 도구의 도메인은 폐쇄적입니다 (예: 메모리 도구) |

```typescript theme={null}
import { tool } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";

const searchTool = tool(
  "search",
  "Search the web",
  { query: z.string() },
  async ({ query }) => {
    return { content: [{ type: "text", text: `Results for: ${query}` }] };
  },
  { annotations: { readOnlyHint: true, openWorldHint: true } }
);
```

<h3 id="createsdkmcpserver">
  `createSdkMcpServer()`
</h3>

애플리케이션과 동일한 프로세스에서 실행되는 MCP 서버 인스턴스를 생성합니다.

```typescript theme={null}
function createSdkMcpServer(options: {
  name: string;
  version?: string;
  instructions?: string;
  tools?: Array<SdkMcpToolDefinition<any>>;
  alwaysLoad?: boolean;
  timeout?: number;
}): McpSdkServerConfigWithInstance;
```

<h4 id="parameters-4">
  매개변수
</h4>

| 매개변수                   | 유형                            | 설명                                                                                                                                                                                     |
| :--------------------- | :---------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options.name`         | `string`                      | MCP 서버의 이름                                                                                                                                                                             |
| `options.version`      | `string`                      | 선택적 버전 문자열                                                                                                                                                                             |
| `options.instructions` | `string`                      | 선택적 서버 지침이며, `initialize`에서 반환되고 MCP 지침 블록으로 모델에 표시됩니다                                                                                                                                 |
| `options.tools`        | `Array<SdkMcpToolDefinition>` | [`tool()`](#tool)로 생성된 도구 정의 배열                                                                                                                                                        |
| `options.alwaysLoad`   | `boolean`                     | `true`인 경우 이 서버의 모든 도구는 초기 프롬프트에 유지되며 [도구 검색](/docs/ko/agent-sdk/tool-search) 뒤에서 지연되지 않습니다. [`tool()`](#tool)의 도구별 `alwaysLoad`와 결합됩니다                                                     |
| `options.timeout`      | `number`                      | 이 서버의 도구 호출에 대한 타임아웃(밀리초)입니다. Claude Code는 [`MCP_TOOL_TIMEOUT`](/docs/ko/env-vars)을 대신하여 이 서버에 적용합니다. 최소 1000의 정수를 전달합니다. Claude Code는 다른 값을 무시합니다. TypeScript Agent SDK v0.3.248 이상이 필요합니다 |

<h3 id="listsessions">
  `listSessions()`
</h3>

가벼운 메타데이터를 포함한 과거 세션을 발견하고 나열합니다. 프로젝트 디렉토리별로 필터링하거나 모든 프로젝트에서 세션을 나열합니다.

```typescript theme={null}
function listSessions(options?: ListSessionsOptions): Promise<SDKSessionInfo[]>;
```

<h4 id="parameters-5">
  매개변수
</h4>

| 매개변수                       | 유형        | 기본값         | 설명                                                |
| :------------------------- | :-------- | :---------- | :------------------------------------------------ |
| `options.dir`              | `string`  | `undefined` | 세션을 나열할 디렉토리입니다. 생략하면 모든 프로젝트에서 세션을 반환합니다         |
| `options.limit`            | `number`  | `undefined` | 반환할 최대 세션 수                                       |
| `options.includeWorktrees` | `boolean` | `true`      | `dir`이 git 저장소 내에 있을 때 모든 worktree 경로에서 세션을 포함합니다 |

<h4 id="return-type-sdksessioninfo">
  반환 유형: `SDKSessionInfo`
</h4>

| 속성             | 유형                    | 설명                                              |
| :------------- | :-------------------- | :---------------------------------------------- |
| `sessionId`    | `string`              | 고유 세션 식별자 (UUID)                                |
| `summary`      | `string`              | 표시 제목: 사용자 정의 제목, 자동 생성된 요약 또는 첫 번째 프롬프트        |
| `lastModified` | `number`              | 마지막 수정 시간(에포크 이후 밀리초)                           |
| `fileSize`     | `number \| undefined` | 세션 파일 크기(바이트)입니다. 로컬 JSONL 저장소에만 채워집니다          |
| `customTitle`  | `string \| undefined` | 사용자 설정 세션 제목 (`/rename`을 통해)                    |
| `firstPrompt`  | `string \| undefined` | 세션의 첫 번째 의미 있는 사용자 프롬프트                         |
| `gitBranch`    | `string \| undefined` | 세션 끝의 Git 분기                                    |
| `cwd`          | `string \| undefined` | 세션의 작업 디렉토리                                     |
| `tag`          | `string \| undefined` | 사용자 설정 세션 태그 ([`tagSession()`](#tagsession) 참조) |
| `createdAt`    | `number \| undefined` | 생성 시간(에포크 이후 밀리초)이며, 첫 번째 항목의 타임스탬프에서 가져옵니다     |

<h4 id="example-2">
  예제
</h4>

프로젝트의 10개 최신 세션을 인쇄합니다. 결과는 `lastModified` 내림차순으로 정렬되므로 첫 번째 항목이 가장 최신입니다. `dir`을 생략하여 모든 프로젝트에서 검색합니다.

```typescript theme={null}
import { listSessions } from "@anthropic-ai/claude-agent-sdk";

const sessions = await listSessions({ dir: "/path/to/project", limit: 10 });

for (const session of sessions) {
  console.log(`${session.summary} (${session.sessionId})`);
}
```

<h3 id="getsessionmessages">
  `getSessionMessages()`
</h3>

과거 세션 기록에서 사용자 및 어시스턴트 메시지를 읽습니다.

```typescript theme={null}
function getSessionMessages(
  sessionId: string,
  options?: GetSessionMessagesOptions
): Promise<SessionMessage[]>;
```

<h4 id="parameters-6">
  매개변수
</h4>

| 매개변수             | 유형       | 기본값         | 설명                                       |
| :--------------- | :------- | :---------- | :--------------------------------------- |
| `sessionId`      | `string` | 필수          | 읽을 세션 UUID (`listSessions()` 참조)         |
| `options.dir`    | `string` | `undefined` | 세션을 찾을 프로젝트 디렉토리입니다. 생략하면 모든 프로젝트를 검색합니다 |
| `options.limit`  | `number` | `undefined` | 반환할 최대 메시지 수                             |
| `options.offset` | `number` | `undefined` | 시작 부분에서 건너뛸 메시지 수                        |

<h4 id="return-type-sessionmessage">
  반환 유형: `SessionMessage`
</h4>

| 속성                   | 유형                      | 설명                                                                                                                                                                                      |
| :------------------- | :---------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`               | `"user" \| "assistant"` | 메시지 역할                                                                                                                                                                                  |
| `uuid`               | `string`                | 고유 메시지 식별자                                                                                                                                                                              |
| `session_id`         | `string`                | 이 메시지가 속한 세션                                                                                                                                                                            |
| `message`            | `unknown`               | 기록에서 원본 메시지 페이로드                                                                                                                                                                        |
| `parent_tool_use_id` | `string \| null`        | 부에이전트 메시지의 경우 부에이전트를 시작한 `Agent` 또는 `Skill` 도구 호출의 `tool_use_id`입니다. 주 세션 메시지 및 이전 세션의 경우 `null`                                                                                        |
| `parent_agent_id`    | `string \| null`        | [중첩된 부에이전트](/docs/ko/sub-agents#let-subagents-spawn-their-own-subagents)의 메시지의 경우 이를 생성한 부에이전트의 `agentId`입니다. 주 세션 메시지, 최상위 부에이전트의 메시지 및 이전 세션의 경우 `null`입니다. Claude Code v2.1.202 이상이 필요합니다 |

<h4 id="example-3">
  예제
</h4>

```typescript theme={null}
import { listSessions, getSessionMessages } from "@anthropic-ai/claude-agent-sdk";

const [latest] = await listSessions({ dir: "/path/to/project", limit: 1 });

if (latest) {
  const messages = await getSessionMessages(latest.sessionId, {
    dir: "/path/to/project",
    limit: 20
  });

  for (const msg of messages) {
    console.log(`[${msg.type}] ${msg.uuid}`);
  }
}
```

<h3 id="getsessioninfo">
  `getSessionInfo()`
</h3>

전체 프로젝트 디렉토리를 스캔하지 않고 ID별로 단일 세션의 메타데이터를 읽습니다.

```typescript theme={null}
function getSessionInfo(
  sessionId: string,
  options?: GetSessionInfoOptions
): Promise<SDKSessionInfo | undefined>;
```

<h4 id="parameters-7">
  매개변수
</h4>

| 매개변수          | 유형       | 기본값         | 설명                                        |
| :------------ | :------- | :---------- | :---------------------------------------- |
| `sessionId`   | `string` | 필수          | 조회할 세션의 UUID                              |
| `options.dir` | `string` | `undefined` | 프로젝트 디렉토리 경로입니다. 생략하면 모든 프로젝트 디렉토리를 검색합니다 |

[`SDKSessionInfo`](#return-type-sdksessioninfo)를 반환하거나, 세션을 찾을 수 없으면 `undefined`를 반환합니다.

<h3 id="renamesession">
  `renameSession()`
</h3>

사용자 정의 제목 항목을 추가하여 세션의 이름을 바꿉니다. 반복 호출은 안전하며 가장 최근 제목이 우선합니다.

```typescript theme={null}
function renameSession(
  sessionId: string,
  title: string,
  options?: SessionMutationOptions
): Promise<void>;
```

<h4 id="parameters-8">
  매개변수
</h4>

| 매개변수          | 유형       | 기본값         | 설명                                        |
| :------------ | :------- | :---------- | :---------------------------------------- |
| `sessionId`   | `string` | 필수          | 이름을 바꿀 세션의 UUID                           |
| `title`       | `string` | 필수          | 새 제목입니다. 공백을 제거한 후 비어 있지 않아야 합니다          |
| `options.dir` | `string` | `undefined` | 프로젝트 디렉토리 경로입니다. 생략하면 모든 프로젝트 디렉토리를 검색합니다 |

<h3 id="tagsession">
  `tagSession()`
</h3>

세션에 태그를 지정합니다. 태그를 지우려면 `null`을 전달합니다. 반복 호출은 안전하며 가장 최근 태그가 우선합니다.

```typescript theme={null}
function tagSession(
  sessionId: string,
  tag: string | null,
  options?: SessionMutationOptions
): Promise<void>;
```

<h4 id="parameters-9">
  매개변수
</h4>

| 매개변수          | 유형               | 기본값         | 설명                                        |
| :------------ | :--------------- | :---------- | :---------------------------------------- |
| `sessionId`   | `string`         | 필수          | 태그를 지정할 세션의 UUID                          |
| `tag`         | `string \| null` | 필수          | 태그 문자열 또는 지우려면 `null`                     |
| `options.dir` | `string`         | `undefined` | 프로젝트 디렉토리 경로입니다. 생략하면 모든 프로젝트 디렉토리를 검색합니다 |

<h3 id="resolvesettings">
  `resolveSettings()`
</h3>

CLI를 생성하지 않고 CLI와 동일한 병합 엔진을 사용하여 주어진 디렉토리에 대한 효과적인 Claude Code 설정을 해결합니다. `query()` 호출을 호출하기 전에 `query()` 호출이 볼 구성을 검사하는 데 사용합니다.

<Note>
  이 함수는 알파 버전이며 안정화 전에 API가 변경될 수 있습니다.
</Note>

스냅샷은 라이브 `query()` 세션이 적용하는 것과 다릅니다:

* **`policyHelper`**: `resolveSettings()`는 macOS plist 및 Windows HKLM/HKCU를 포함한 MDM 소스를 읽지만 관리자 구성 `policyHelper` 부프로세스를 실행하지 않습니다.
* **서버 관리 설정**: `resolveSettings()`는 [서버 관리 설정](/docs/ko/server-managed-settings#fetch-and-caching-behavior)을 가져오지 않습니다. 포함하려면 `options.serverManagedSettings`으로 전달합니다.
* **`defaultMode`**: 스냅샷은 모든 계층에서 `permissions.defaultMode`를 그대로 반환하므로 프로젝트 및 로컬 설정의 `'auto'` 및 `'bypassPermissions'` 값을 포함할 수 있으며, [라이브 세션은 무시합니다](/docs/ko/permission-modes#which-mode-a-session-starts-in).

```typescript theme={null}
function resolveSettings(
  options?: ResolveSettingsOptions
): Promise<ResolvedSettings>;
```

<h4 id="parameters-10">
  매개변수
</h4>

`resolveSettings()`는 단일 옵션 객체를 수락합니다. 모든 필드는 선택적입니다.

| 매개변수                            | 유형                                    | 기본값             | 설명                                                                                                                                                                                                                  |
| :------------------------------ | :------------------------------------ | :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `options.cwd`                   | `string`                              | `process.cwd()` | 프로젝트 및 로컬 설정을 상대적으로 해결할 디렉토리                                                                                                                                                                                        |
| `options.settingSources`        | [`SettingSource`](#settingsource)`[]` | 모든 소스           | 로드할 파일 시스템 소스입니다. 사용자, 프로젝트 및 로컬 설정을 건너뛰려면 `[]`을 전달합니다. [엔드포인트 관리 정책](/docs/ko/managed-settings#delivery-mechanisms)은 모든 경우에 로드됩니다. `resolveSettings()`는 `options.serverManagedSettings`을 전달할 때만 서버 관리 설정을 포함합니다         |
| `options.managedSettings`       | `Settings`                            | `undefined`     | 임베딩 호스트에서 제공하는 정책 계층 설정입니다. [`managedSettings` in `Options`](#options)와 동일한 규칙을 따릅니다. 단, `resolveSettings()`는 구성된 [`policyHelper`](/docs/ko/settings-reference#policyhelper)를 실행하지 않으므로 스냅샷은 라이브 세션이 삭제하는 설정을 포함할 수 있습니다 |
| `options.serverManagedSettings` | `Settings`                            | `undefined`     | `/api/claude_code/settings`의 서버 관리 설정 페이로드입니다. 제한이 없는 키는 필터링되지 않고 통과합니다                                                                                                                                             |

<h4 id="return-type-resolvedsettings">
  반환 유형: `ResolvedSettings`
</h4>

`resolveSettings()`는 병합된 설정 및 각 키에 기여한 소스를 설명하는 객체를 반환합니다.

| 속성           | 유형                                                  | 설명                                             |
| :----------- | :-------------------------------------------------- | :--------------------------------------------- |
| `effective`  | `Settings`                                          | 모든 활성화된 소스를 우선순위 순서로 적용한 후 병합된 설정              |
| `provenance` | `Partial<Record<keyof Settings, ProvenanceEntry>>`  | `effective`의 각 최상위 키에 대해 값을 제공한 소스             |
| `sources`    | `Array<{ source, settings, path?, policyOrigin? }>` | 소스별 원본 설정이며, 가장 낮은 우선순위에서 가장 높은 우선순위 순서로 정렬됩니다 |

<h4 id="example-4">
  예제
</h4>

아래 예제는 프로젝트 디렉토리에 대한 설정을 해결하고 정리 기간을 제어하는 소스를 인쇄합니다. 설정 파일이 `cleanupPeriodDays`를 설정하지 않는 머신에서는 두 인쇄된 줄 모두 값에 대해 `undefined`를 표시하며, 이는 오류가 아닌 예상된 출력입니다.

```typescript theme={null}
import { resolveSettings } from "@anthropic-ai/claude-agent-sdk";

const { effective, provenance } = await resolveSettings({
  cwd: "/path/to/project",
  settingSources: ["user", "project", "local"],
});

console.log(`Cleanup period: ${effective.cleanupPeriodDays} days`);
console.log(`Set by: ${provenance.cleanupPeriodDays?.source}`);
```

<h2 id="types">
  유형
</h2>

<h3 id="options">
  `Options`
</h3>

`query()` 함수의 구성 객체입니다.

| 속성                                | 유형                                                                                                                                                                                                             | 기본값                                | 설명                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `abortController`                 | `AbortController`                                                                                                                                                                                              | `new AbortController()`            | 작업 취소를 위한 컨트롤러                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `additionalDirectories`           | `string[]`                                                                                                                                                                                                     | `[]`                               | Claude가 접근할 수 있는 추가 디렉토리입니다. SDK는 각 항목을 Claude Code에 `--add-dir`로 전달하므로, `project` 설정이 있는 Claude Code 소스도 [디렉토리의 스킬, 명령어 및 서브에이전트를 로드합니다](/docs/ko/permissions#additional-directories-grant-file-access-not-configuration)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `agent`                           | `string`                                                                                                                                                                                                       | `undefined`                        | 메인 스레드의 에이전트 이름입니다. 에이전트는 `agents` 옵션 또는 설정에서 정의되어야 합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `agents`                          | `Record<string, [`AgentDefinition`](#agentdefinition)>`                                                                                                                                                        | `undefined`                        | 프로그래밍 방식으로 서브에이전트를 정의합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `agentProgressSummaries`          | `boolean`                                                                                                                                                                                                      | `false`                            | `true`일 때, 서브에이전트에 대한 한 줄 진행 상황 요약을 생성하고 [`task_progress`](#sdktaskprogressmessage) 이벤트의 `summary` 필드를 통해 전달합니다. 포그라운드 및 백그라운드 서브에이전트에 적용됩니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `allowDangerouslySkipPermissions` | `boolean`                                                                                                                                                                                                      | `false`                            | 권한 우회를 활성화합니다. `permissionMode: 'bypassPermissions'`를 사용할 때 필요하며, 시작 시 또는 나중에 `setPermissionMode()`를 통해 설정할 수 있습니다. [플랜 모드](/docs/ko/agent-sdk/permissions#plan-mode-plan)에서 `permissionMode: 'plan'`과 상호작용하는 방식을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `allowedTools`                    | `string[]`                                                                                                                                                                                                     | `[]`                               | 프롬프트 없이 자동 승인할 도구입니다. 이는 Claude를 이 도구들로만 제한하지 않습니다. [작업 추적 도구](/docs/ko/agent-sdk/todo-tracking#model-availability) 중 하나를 여기에 명시하면, Claude Code도 세션을 옵트인합니다. 나열되지 않은 다른 도구는 `permissionMode` 및 `canUseTool`로 전달됩니다. 도구를 차단하려면 `disallowedTools`를 사용하세요. [권한](/docs/ko/agent-sdk/permissions#allow-and-deny-rules)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `betas`                           | [`SdkBeta`](#sdkbeta)`[]`                                                                                                                                                                                      | `[]`                               | 베타 기능을 활성화합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `canUseTool`                      | [`CanUseTool`](#canusetool)                                                                                                                                                                                    | `undefined`                        | 사용자 정의 권한 함수로, [권한 흐름](/docs/ko/agent-sdk/permissions#how-permissions-are-evaluated)이 프롬프트로 전달될 때만 호출됩니다. `allowedTools`, 허용 규칙 또는 `permissionMode`에 의해 자동 승인된 호출에 대해서는 호출되지 않습니다. 허용 규칙은 [모든 모드가 자동 승인하지 않는 작업](/docs/ko/permission-modes#actions-no-mode-auto-approves)을 사전 승인하지 않습니다. [`CanUseTool`](#canusetool)의 세부 정보를 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `continue`                        | `boolean`                                                                                                                                                                                                      | `false`                            | 가장 최근 대화를 계속합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `cwd`                             | `string`                                                                                                                                                                                                       | `process.cwd()`                    | 현재 작업 디렉토리                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `debug`                           | `boolean`                                                                                                                                                                                                      | `false`                            | Claude Code 프로세스에 대한 디버그 모드를 활성화합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `debugFile`                       | `string`                                                                                                                                                                                                       | `undefined`                        | 디버그 로그를 특정 파일 경로에 작성합니다. 암묵적으로 디버그 모드를 활성화합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `disallowedTools`                 | `string[]`                                                                                                                                                                                                     | `[]`                               | 거부할 도구입니다. `"Bash"`와 같은 단순 이름은 Claude의 컨텍스트에서 도구를 제거합니다. `"Bash(rm *)"` 같은 범위 지정 규칙은 도구를 사용 가능하게 두고 [작성된 대로](/docs/ko/permissions#bash-rule-limits) 모든 권한 모드(예: `bypassPermissions`)에서 일치하는 호출을 거부합니다. [권한](/docs/ko/agent-sdk/permissions#allow-and-deny-rules)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `effort`                          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max'`                                                                                                                                                              | `undefined`                        | Claude가 응답에 투입하는 노력의 정도를 제어합니다. 적응형 사고와 함께 작동하여 사고 깊이를 안내합니다. [노력 수준 조정](/docs/ko/model-config#adjust-effort-level)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `enableFileCheckpointing`         | `boolean`                                                                                                                                                                                                      | `false`                            | 되감기를 위한 파일 변경 추적을 활성화합니다. [파일 체크포인팅](/docs/ko/agent-sdk/file-checkpointing)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `env`                             | `Record<string, string \| undefined>`                                                                                                                                                                          | `process.env`                      | 환경 변수입니다. 설정하면 `process.env`와 병합하는 대신 서브프로세스 환경을 대체하므로, `{ ...process.env, YOUR_VAR: 'value' }`를 전달하여 `PATH`와 같은 상속된 변수를 유지하세요. 이 패턴의 예는 [느리거나 정지된 API 응답 처리](#handle-slow-or-stalled-api-responses)를 참조하고, 기본 CLI가 읽는 변수는 [환경 변수](/docs/ko/env-vars)를 참조하세요. User-Agent 헤더에서 앱을 식별하려면 `CLAUDE_AGENT_SDK_CLIENT_APP`을 설정하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `executable`                      | `'bun' \| 'deno' \| 'node'`                                                                                                                                                                                    | 자동 감지                              | 사용할 JavaScript 런타임입니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `executableArgs`                  | `string[]`                                                                                                                                                                                                     | `[]`                               | 실행 파일에 전달할 인수입니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `extraArgs`                       | `Record<string, string \| null>`                                                                                                                                                                               | `{}`                               | 추가 인수입니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `fallbackModel`                   | `string`                                                                                                                                                                                                       | `undefined`                        | 기본 모델이 실패할 경우 사용할 모델입니다. 쉼표로 구분된 목록을 허용합니다. 순서 및 상한에 대해서는 [폴백 모델 체인](/docs/ko/model-config#fallback-model-chains)을 참조하세요. 지침은 [모델 선택](/docs/ko/agent-sdk/configuration#choose-a-model)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `forkSession`                     | `boolean`                                                                                                                                                                                                      | `false`                            | `resume`으로 재개할 때 원본 세션을 계속하는 대신 새 세션 ID로 포크합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `forwardSubagentText`             | `boolean`                                                                                                                                                                                                      | `false`                            | 서브에이전트 텍스트 및 사고 블록을 `parent_tool_use_id`가 설정된 어시스턴트 및 사용자 메시지로 전달하여 소비자가 중첩된 대화록을 렌더링할 수 있도록 합니다. 이 옵션이 없으면 Claude Code는 서브에이전트 `tool_use` 및 `tool_result` 블록을 내보내지만 텍스트나 사고는 내보내지 않습니다. 모든 중첩 깊이의 서브에이전트 메시지는 Claude Code v2.1.219 이상에서 전달됩니다. v2.1.219 이전에는 깊이 1의 서브에이전트 메시지만 나타났습니다. 포크된 스킬이 생성하는 서브에이전트의 메시지와 중첩된 포크된 스킬의 메시지는 v2.1.275 이상이 필요합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `hooks`                           | `Partial<Record<`[`HookEvent`](#hookevent)`, `[`HookCallbackMatcher`](#hookcallbackmatcher)`[]>>`                                                                                                              | `{}`                               | 이벤트에 대한 훅 콜백입니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `includeHookEvents`               | `boolean`                                                                                                                                                                                                      | `false`                            | 훅 라이프사이클 이벤트를 메시지 스트림에 [`SDKHookStartedMessage`](#sdkhookstartedmessage), [`SDKHookProgressMessage`](#sdkhookprogressmessage) 및 [`SDKHookResponseMessage`](#sdkhookresponsemessage)로 포함합니다. `SessionStart` 및 `Setup` 훅의 라이프사이클 이벤트는 항상 포함되며 이 옵션이 필요하지 않습니다. `Notification`, `SessionEnd`, `PreCompact` 및 `PostCompact` 같은 일부 훅 이벤트는 이 옵션이 있어도 `SDKHookStartedMessage`를 생성하지 않습니다. 이러한 이벤트의 경우 Claude Code는 여전히 `SDKHookProgressMessage`를 내보내고, 1초 이상 실행되는 명령어 훅은 출력을 내보내며, [백그라운드에서 실행되는](/docs/ko/hooks#run-hooks-in-the-background) 훅이 완료될 때만 `SDKHookResponseMessage`를 내보냅니다                                                                                                                                                                                                                                                                                                                |
| `includePartialMessages`          | `boolean`                                                                                                                                                                                                      | `false`                            | 부분 메시지 이벤트를 포함합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `loadTimeoutMs`                   | `number`                                                                                                                                                                                                       | `60000`                            | *알파.* 재개 구체화 중 각 `sessionStore.load()` 및 `sessionStore.listSubkeys()` 호출에 대한 밀리초 단위 시간 초과입니다. 어댑터가 이 창 내에서 정착하지 않으면 쿼리가 중단되는 대신 실패합니다. `sessionStore`가 설정되지 않으면 무시됩니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `managedSettings`                 | `Settings`                                                                                                                                                                                                     | `undefined`                        | 호스트 프로세스가 생성된 세션에 제공하는 정책 계층 설정입니다. 관리자가 배포한 관리 설정이 있는 머신에서 Claude Code는 관리자의 최우선 관리 소스가 `parentSettingsBehavior: 'merge'`를 설정하지 않으면 이를 무시하고, [`policyHelper`](/docs/ko/settings-reference#policyhelper)가 관리 설정을 제공하는 동안 절대 병합하지 않습니다. 병합된 값은 제한적 필터를 통과합니다. [부모 설정 제한](/docs/ko/claude-apps-gateway#restrict-parent-settings)은 필터가 허용하는 것과 `allowManaged*Only` 잠금을 다룹니다. [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/ko/env-vars)를 설정한 호스트는 이 페이로드에서 직접 읽은 세 가지 키를 가집니다. Claude Code v2.1.222 이상의 [모델 구성](/docs/ko/model-config#restrict-model-selection), 관리 소스가 v2.1.246 이상에서 설정하지 않을 때 [`modelPricing`](/docs/ko/settings-reference#modelpricing), v2.1.247 이상에서 `ENABLE_TOOL_SEARCH` env 항목입니다                                                                                                                                                                                                              |
| `maxBudgetUsd`                    | `number`                                                                                                                                                                                                       | `undefined`                        | 클라이언트 측 비용 추정이 이 USD 값에 도달하면 쿼리를 중지합니다. 호출 자체의 지출만 계산합니다. 재개된 세션에서 복원된 합계는 계산되지 않습니다. 정확도 주의 사항 및 재설정 동작은 [비용 및 사용량 추적](/docs/ko/agent-sdk/cost-tracking)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `maxThinkingTokens`               | `number`                                                                                                                                                                                                       | `undefined`                        | *더 이상 사용되지 않음:* 대신 `thinking`을 사용하세요. 사고 프로세스의 최대 토큰                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `maxTurns`                        | `number`                                                                                                                                                                                                       | `undefined`                        | 최대 에이전트 턴(도구 사용 왕복)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `mcpServers`                      | `Record<string, [`McpServerConfig`](#mcpserverconfig)>`                                                                                                                                                        | `{}`                               | MCP 서버 구성입니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `model`                           | `string`                                                                                                                                                                                                       | CLI의 기본값                           | Claude 모델 별칭 또는 전체 모델 이름입니다. [허용되는 값 및 공급자별 ID](/docs/ko/model-config#available-models)를 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `onElicitation`                   | `(request: ElicitationRequest, options: { signal: AbortSignal }) => Promise<ElicitationResult>`                                                                                                                | `undefined`                        | MCP 유도 요청 처리를 위한 콜백입니다. MCP 서버가 사용자 입력을 요청하고 훅이 먼저 처리하지 않을 때 호출됩니다. 제공되지 않으면 처리되지 않은 유도 요청이 자동으로 거부됩니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `outputFormat`                    | `{ type: 'json_schema', schema: JSONSchema }`                                                                                                                                                                  | `undefined`                        | 에이전트 결과의 출력 형식을 정의합니다. [구조화된 출력](/docs/ko/agent-sdk/structured-outputs)의 세부 정보를 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `outputStyle`                     | `string`                                                                                                                                                                                                       | `undefined`                        | `Options` 필드가 아닙니다. 인라인 [`settings`](/docs/ko/settings) 객체 또는 설정 파일에서 `outputStyle`을 설정하세요. [출력 스타일 활성화](/docs/ko/agent-sdk/modifying-system-prompts#activate-an-output-style)를 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `pathToClaudeCodeExecutable`      | `string`                                                                                                                                                                                                       | 번들된 네이티브 바이너리에서 자동 해결됨             | Claude Code 실행 파일의 경로입니다. 설치 중 선택적 종속성을 건너뛰었거나 플랫폼이 지원되는 집합에 없을 때만 필요합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `permissionMode`                  | [`PermissionMode`](#permissionmode)                                                                                                                                                                            | `'default'`                        | 세션의 권한 모드입니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `permissionPromptToolName`        | `string`                                                                                                                                                                                                       | `undefined`                        | 권한 프롬프트의 MCP 도구 이름입니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `permissionPrompts`               | `'host' \| 'none'`                                                                                                                                                                                             | `'host'`                           | 권한 프롬프트에 응답하는 사람입니다. `'host'`는 [`canUseTool`](#canusetool) 콜백 또는 `permissionPromptToolName` 도구로 라우팅하고, `'none'`은 [프롬프트가 표시되었을 호출을 거부합니다](/docs/ko/agent-sdk/permissions#how-permissions-are-evaluated). Claude Code v2.1.259 이상이 필요합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `persistSession`                  | `boolean`                                                                                                                                                                                                      | `true`                             | `false`일 때, 디스크에 대한 세션 지속성을 비활성화합니다. 세션을 나중에 재개할 수 없습니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `planModeInstructions`            | `string`                                                                                                                                                                                                       | `undefined`                        | 플랜 모드의 사용자 정의 워크플로우 지침입니다. `permissionMode`가 `'plan'`일 때, 이 문자열은 기본 플랜 모드 워크플로우 본문을 대체합니다. CLI는 여전히 읽기 전용 적용 프리앰블과 ExitPlanMode 프로토콜 바닥글로 래핑합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `plugins`                         | [`SdkPluginConfig`](#sdkpluginconfig)`[]`                                                                                                                                                                      | `[]`                               | 로컬 경로에서 사용자 정의 플러그인을 로드합니다. [플러그인](/docs/ko/agent-sdk/plugins)의 세부 정보를 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `projectConfigRoot`               | `string`                                                                                                                                                                                                       | `undefined`                        | `cwd`가 worktree인 신뢰할 수 있는 체크아웃의 절대 경로입니다. Claude Code는 프로젝트 설정, `.mcp.json` 및 프로젝트의 `.claude/` 명령어, 에이전트, 스킬, 워크플로우, 루틴 및 출력 스타일을 `cwd` 대신 이 디렉토리에서 읽고 `CLAUDE_PROJECT_DIR`을 설정합니다. `CLAUDE.md` 파일 및 `.claude/rules/`는 여전히 `cwd`에서 로드됩니다. `apiKeyHelper` 같은 훅, 도우미 스크립트 및 stdio MCP 서버는 이 디렉토리를 작업 디렉토리로 시작합니다. Claude Code v2.1.275 이상이 필요합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `promptSuggestions`               | `boolean`                                                                                                                                                                                                      | `false`                            | 프롬프트 제안을 활성화합니다. 턴 후 Claude Code는 예측된 다음 사용자 프롬프트를 전달하는 `prompt_suggestion` 메시지를 내보냅니다. Claude Code는 계정이 사용량 한계에 가깝거나 도달했을 때와 같은 일부 턴에 대해 제안을 생성하지 않습니다. [Claude Code가 제안을 건너뛸 때](/docs/ko/interactive-mode#when-claude-code-skips-suggestions)를 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `resume`                          | `string`                                                                                                                                                                                                       | `undefined`                        | 재개할 세션 ID입니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `resumeDropsTurn`                 | `string`                                                                                                                                                                                                       | `undefined`                        | `resumeSessionAt`과 함께: 자르기 재개가 버릴 의도가 있는 턴의 프롬프트 UUID입니다. Claude Code는 버려진 범위에 해당 턴에 귀속될 수 없는 것(예: 흡수된 대기 메시지 또는 작업 알림)이 포함되면 재개를 거부하고 거부 메시지에서 `--resume-drops-turn` 플래그를 명시합니다. 에이전트 SDK 및 인쇄 모드 재개만 쌍을 읽습니다. Claude Code v2.1.223 이상이 필요합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `resumeSessionAt`                 | `string`                                                                                                                                                                                                       | `undefined`                        | 특정 메시지 UUID에서 세션을 재개합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `sandbox`                         | [`SandboxSettings`](#sandboxsettings)                                                                                                                                                                          | `undefined`                        | 프로그래밍 방식으로 샌드박스 동작을 구성합니다. [샌드박스 설정](#sandboxsettings)의 세부 정보를 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `sessionId`                       | `string`                                                                                                                                                                                                       | 자동 생성됨                             | 자동 생성하는 대신 세션에 특정 UUID를 사용합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `sessionStore`                    | [`SessionStore`](/docs/ko/agent-sdk/session-storage#the-sessionstore-interface)                                                                                                                                     | `undefined`                        | 세션 대화록을 외부 백엔드로 미러링하여 다른 호스트가 재개할 수 있도록 합니다. [외부 저장소에 세션 지속](/docs/ko/agent-sdk/session-storage)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `sessionStoreFlush`               | `'batched' \| 'eager'`                                                                                                                                                                                         | `'batched'`                        | *알파.* `sessionStore`의 플러시 모드입니다. `sessionStore`가 설정되지 않으면 무시됩니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `settings`                        | `string \| Settings`                                                                                                                                                                                           | `undefined`                        | 인라인 [설정](/docs/ko/settings) 객체, 설정 파일 경로 또는 인라인 JSON 문자열입니다. [우선 순위 순서](/docs/ko/settings#settings-precedence)에서 플래그 설정 계층을 채웁니다. [`applyFlagSettings()`](#applyflagsettings)로 런타임에 변경하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `settingSources`                  | [`SettingSource`](#settingsource)`[]`                                                                                                                                                                          | CLI 기본값(모든 소스)                     | 로드할 파일 시스템 설정을 제어합니다. 사용자, 프로젝트 및 로컬 설정을 비활성화하려면 `[]`를 전달하세요. [엔드포인트 관리 정책](/docs/ko/managed-settings#delivery-mechanisms)은 관계없이 로드됩니다. 서버 관리 설정은 세션이 [적격 구성](/docs/ko/server-managed-settings#platform-availability)에서 조직 자격증명으로 인증할 때 가져옵니다. [Claude Code 기능 사용](/docs/ko/agent-sdk/claude-code-features#what-settingsources-does-not-control)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `skills`                          | `string[] \| 'all'`                                                                                                                                                                                            | `undefined`                        | 세션에서 사용 가능한 스킬입니다. 모든 발견된 스킬을 활성화하려면 `'all'`을 전달하거나 스킬 이름 목록을 전달하세요. 정확한 이름만 전달하세요. 에이전트 SDK v0.3.221 이상에서 SDK는 Claude Code 프로세스를 시작하기 전에 잘못된 형식 및 와일드카드 형식 이름을 오류로 거부합니다. 설정하면 SDK는 Skill 도구를 `allowedTools`에 자동으로 추가합니다. `tools`도 전달하면 해당 목록에 `'Skill'`을 포함하세요. [스킬](/docs/ko/agent-sdk/skills)을 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `spawnClaudeCodeProcess`          | `(options: SpawnOptions) => SpawnedProcess`                                                                                                                                                                    | `undefined`                        | Claude Code 프로세스를 생성하는 사용자 정의 함수입니다. VM, 컨테이너 또는 원격 환경에서 Claude Code를 실행하는 데 사용합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `stderr`                          | `(data: string) => void`                                                                                                                                                                                       | `undefined`                        | stderr 출력에 대한 콜백입니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `strictMcpConfig`                 | `boolean`                                                                                                                                                                                                      | `false`                            | `mcpServers`에 전달된 서버만 사용하고 프로젝트 `.mcp.json`, 사용자 설정, 플러그인 제공 MCP 서버 및 [claude.ai 커넥터](/docs/ko/mcp#use-mcp-servers-from-claude-ai)를 무시합니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `systemPrompt`                    | `string \| string[] \| { type: 'custom'; prompt: string \| string[]; snapshot?: boolean } \| { type: 'preset'; preset: 'claude_code'; append?: string; excludeDynamicSections?: boolean; snapshot?: boolean }` | `undefined` (최소 프롬프트)              | 시스템 프롬프트 구성입니다. 사용자 정의 프롬프트의 경우 문자열을 전달하거나, Claude Code의 시스템 프롬프트를 사용하려면 `{ type: 'preset', preset: 'claude_code' }`를 전달하세요. 내보낸 `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` 상수를 정적 및 요청별 부분 사이에 포함하여 [사용자 정의 프롬프트의 정적 부분을 캐시](/docs/ko/agent-sdk/modifying-system-prompts#cache-the-static-part-of-a-custom-prompt)하는 문자열 배열을 전달하세요. 프리셋 객체 형식을 사용할 때 추가 지침으로 확장하려면 `append`를 추가하고, [머신 간 더 나은 프롬프트 캐시 재사용](/docs/ko/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines)을 위해 세션별 컨텍스트를 첫 번째 사용자 메시지로 이동하려면 `excludeDynamicSections: true`를 설정하세요. 세션이 첫 번째 요청에서 기록한 프롬프트를 재사용하는 대신 [모든 요청에서 프롬프트를 다시 빌드](/docs/ko/agent-sdk/modifying-system-prompts#change-the-prompt-of-an-existing-session)하려면 `snapshot: false`를 설정하세요. 사용자 정의 프롬프트에서 `snapshot`을 설정하려면 `{ type: 'custom', prompt }` 형식을 전달하세요. `{ type: 'custom' }` 형식과 `snapshot` 필드는 TypeScript 에이전트 SDK v0.3.257 이상이 필요합니다 |
| `taskBudget`                      | `{ total: number }`                                                                                                                                                                                            | `undefined`                        | *알파.* API 측 작업 예산(토큰 단위)입니다. 설정하면 모델에 남은 토큰 예산이 알려져 도구 사용을 조절하고 한계 전에 마무리할 수 있습니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `thinking`                        | [`ThinkingConfig`](#thinkingconfig)                                                                                                                                                                            | 지원되는 모델의 경우 `{ type: 'adaptive' }` | Claude의 사고/추론 동작을 제어합니다. 옵션은 [`ThinkingConfig`](#thinkingconfig)를 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `title`                           | `string`                                                                                                                                                                                                       | `undefined`                        | 세션의 표시 제목입니다. `resume` 또는 `continue`를 통해 재개할 때 재개된 세션의 지속된 제목이 우선합니다. 기존 세션의 제목을 변경하려면 [`renameSession()`](#renamesession)을 사용하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `toolAliases`                     | `Record<string, string>`                                                                                                                                                                                       | `undefined`                        | 기본 제공 도구 이름을 MCP 도구 이름으로 매핑하여 Claude가 기본 제공 대신 MCP 구현을 호출하도록 합니다. 예를 들어 `{ Bash: 'mcp__workspace__bash' }`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `toolConfig`                      | [`ToolConfig`](#toolconfig)                                                                                                                                                                                    | `undefined`                        | 기본 제공 도구 동작의 구성입니다. 세부 정보는 [`ToolConfig`](#toolconfig)를 참조하세요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `tools`                           | `string[] \| { type: 'preset'; preset: 'claude_code' }`                                                                                                                                                        | `undefined`                        | 도구 구성입니다. 도구 이름 배열을 전달하거나 프리셋을 사용하여 Claude Code의 기본 도구를 가져옵니다                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |

<h4 id="handle-slow-or-stalled-api-responses">
  느리거나 정지된 API 응답 처리
</h4>

CLI 서브프로세스는 API 시간 초과 및 정지 감지를 제어하는 여러 환경 변수를 읽습니다. `env` 옵션을 통해 전달하세요:

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const result = query({
  prompt: "Analyze this code",
  options: {
    env: {
      ...process.env,
      API_TIMEOUT_MS: "120000",
      CLAUDE_CODE_MAX_RETRIES: "2",
      CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS: "120000",
    },
  },
});
```

* `API_TIMEOUT_MS`: Anthropic 클라이언트의 요청별 시간 초과(밀리초 단위)입니다. 기본값 `600000`입니다. 메인 루프 및 모든 서브에이전트에 적용됩니다.
* `CLAUDE_CODE_MAX_RETRIES`: 최대 API 재시도입니다. 기본값 `10`, 상한 `15`입니다. 각 재시도는 자체 `API_TIMEOUT_MS` 창을 가지므로 최악의 경우 벽시간은 대략 `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` 더하기 백오프입니다. 더 긴 중단을 기다려야 하는 무인 실행의 경우 [`CLAUDE_CODE_RETRY_WATCHDOG=1`](/docs/ko/errors#tune-retry-behavior)을 설정하세요. 일시적 용량 오류를 무한정 재시도하고, Claude Code v2.1.199 이상에서 다른 일시적 오류의 기본값을 `300`으로 올리고 이 변수의 상한을 제거합니다.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: 서브에이전트의 정지 감시입니다. 스트림 감시가 켜져 있는 동안 기본값은 `CLAUDE_STREAM_IDLE_TIMEOUT_MS` 더하기 5분이며, 이는 해당 변수를 올리지 않으면 `600000`입니다. 스트림 감시가 꺼져 있으면 기본값은 `600000`입니다. v2.1.257 이전에는 기본값이 항상 `600000`이었습니다.

  타이머는 각 스트림 이벤트에서 재설정됩니다. 정지 시 Claude Code는 서브에이전트를 중단하고 정지를 부모에 보고합니다. 백그라운드 서브에이전트의 경우 작업도 실패로 표시하고 부분 결과를 첨부합니다.
* `CLAUDE_ENABLE_STREAM_WATCHDOG`과 `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: 헤더가 도착했지만 응답 본문이 스트리밍을 중지할 때 요청을 중단하는 스트림 감시입니다. 감시는 모든 공급자에 대해 기본적으로 켜져 있습니다. 비활성화하려면 `CLAUDE_ENABLE_STREAM_WATCHDOG=0`을 설정하세요. `CLAUDE_STREAM_IDLE_TIMEOUT_MS`는 기본값 `300000`이고 해당 최소값으로 고정됩니다. 중단 후 [자동 재시도](/docs/ko/errors#automatic-retries)는 응답이 진행된 정도에 따라 Claude Code가 수행하는 작업을 다룹니다.

  감시가 `ANTHROPIC_BASE_URL` 뒤의 게이트웨이가 keep-alive 핑으로 열어 두는 응답을 기다리는 동안, `includePartialMessages`를 설정한 호스트는 계속 `ping` [스트림 이벤트](#sdkpartialassistantmessage)를 수신하므로 이 프레임을 생존성으로 읽고 마지막 실제 스트림 이벤트 후 5분 동안 세션 시간 초과를 하지 마세요. v2.1.257 이전에는 프레임이 5분 후에 중지되었습니다.

<h3 id="query-object">
  `Query` 객체
</h3>

`query()` 함수에서 반환된 인터페이스입니다.

```typescript theme={null}
interface Query extends AsyncGenerator<SDKMessage, void> {
  interrupt(): Promise<SDKControlInterruptResponse | undefined>;
  rewindFiles(
    userMessageId: string,
    options?: { dryRun?: boolean }
  ): Promise<RewindFilesResult>;
  setPermissionMode(mode: PermissionMode): Promise<void>;
  setModel(model?: string): Promise<void>;
  setMaxThinkingTokens(maxThinkingTokens: number | null): Promise<void>;
  applyFlagSettings(settings: {
    [K in keyof Settings]?: K extends 'effortLevel'
      ? 'low' | 'medium' | 'high' | 'xhigh' | 'max' | null
      : Settings[K] | null;
  }): Promise<void>;
  updateSettings(
    source: 'localSettings' | 'userSettings',
    settings: Record<string, unknown>,
  ): Promise<void>;
  initializationResult(): Promise<SDKControlInitializeResponse>;
  reinitialize(): Promise<SDKControlInitializeResponse>;
  supportedCommands(): Promise<SlashCommand[]>;
  supportedModels(): Promise<ModelInfo[]>;
  supportedAgents(): Promise<AgentInfo[]>;
  mcpServerStatus(): Promise<McpServerStatus[]>;
  getContextUsage(opts?: {
    detail?: 'summary' | 'full';
  }): Promise<SDKControlGetContextUsageResponse>;
  readFile(
    path: string,
    options?: { maxBytes?: number; encoding?: 'utf-8' | 'base64' }
  ): Promise<SDKControlReadFileResponse | null>;
  reloadSkills(): Promise<SDKControlReloadSkillsResponse>;
  accountInfo(): Promise<AccountInfo>;
  reconnectMcpServer(serverName: string): Promise<void>;
  toggleMcpServer(serverName: string, enabled: boolean): Promise<void>;
  setMcpServers(servers: Record<string, McpServerConfig>): Promise<McpSetServersResult>;
  readMcpResource(serverName: string, uri: string): Promise<SDKControlMcpReadResourceResponse>;
  streamInput(stream: AsyncIterable<SDKUserMessage>): Promise<void>;
  stopTask(taskId: string): Promise<void>;
  close(): void;
}
```

<h4 id="methods">
  메서드
</h4>

| 메서드                                    | 설명                                                                                                                                                                                                                                                                                                                                                                              |
| :------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `interrupt()`                          | 쿼리를 중단합니다. 스트리밍 입력 모드에서만 사용 가능합니다. CLI가 [`SDKSystemMessage.capabilities`](#sdksystemmessage)에서 `interrupt_receipt_v1` 기능을 광고할 때 중단이 도착했을 때 대기 중이던 메시지를 나열하는 [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse)로 해결됩니다. v2.1.205 이전의 CLI에서는 `undefined`로 해결됩니다                                                                                                        |
| `rewindFiles(userMessageId, options?)` | 파일을 지정된 사용자 메시지의 상태로 복원합니다. 변경 사항을 미리 보려면 `{ dryRun: true }`를 전달하세요. `enableFileCheckpointing: true`가 필요합니다. [파일 체크포인팅](/docs/ko/agent-sdk/file-checkpointing)을 참조하세요                                                                                                                                                                                                                |
| `setPermissionMode()`                  | 권한 모드를 변경합니다(스트리밍 입력 모드에서만 사용 가능)                                                                                                                                                                                                                                                                                                                                               |
| `setModel()`                           | 모델을 변경합니다(스트리밍 입력 모드에서만 사용 가능). `undefined` 또는 문자열 `"default"`를 전달하면 [Claude Code의 기본 모델](/docs/ko/model-config)로 재설정됩니다                                                                                                                                                                                                                                                             |
| `setMaxThinkingTokens()`               | *더 이상 사용되지 않음:* 대신 `thinking` 옵션을 사용하세요. 최대 사고 토큰을 변경합니다. `null`을 전달하면 사고를 세션 기본값으로 재설정합니다. 중간 세션 재정의가 지워지고, 사고가 비활성화된 세션의 경우 사고는 꺼진 상태로 유지됩니다                                                                                                                                                                                                                                  |
| `applyFlagSettings(settings)`          | 런타임에 설정을 세션의 플래그 설정 계층으로 병합합니다(스트리밍 입력 모드에서만 사용 가능). [`applyFlagSettings()`](#applyflagsettings)를 참조하세요                                                                                                                                                                                                                                                                         |
| `updateSettings(source, settings)`     | 프로젝트의 로컬 설정 파일 또는 사용자 설정 파일에 허용 목록 키를 작성하여 값이 나중 세션에 지속되도록 합니다. [`updateSettings()`](#updatesettings)를 참조하세요. TypeScript SDK v0.3.257 이상이 필요하며, Claude Code v2.1.257을 번들합니다                                                                                                                                                                                                     |
| `initializationResult()`               | 지원되는 명령어, 모델, 계정 정보 및 출력 스타일 구성을 포함한 전체 초기화 결과를 반환합니다                                                                                                                                                                                                                                                                                                                           |
| `reinitialize()`                       | 실행 중인 CLI에 `initialize` 제어 요청을 다시 보내고 캐시된 첫 연결 결과 대신 새로운 결과를 반환합니다. 연결 해제 후 세션에 다시 연결하는 것과 같은 전송 간격 후 사용하여 대기 중인 권한 요청이 `canUseTool` 콜백에 다시 도달하도록 합니다. 응답이 손실된 요청이 다시 전달되므로 요청 ID당 콜백을 멱등성으로 만드세요. Claude Code v2.1.195 이상이 필요합니다                                                                                                                                               |
| `supportedCommands()`                  | 사용 가능한 명령어를 반환합니다. 에이전트 SDK v0.3.216부터 목록은 중간 세션 명령어 변경을 반영합니다. [`SDKCommandsChangedMessage`](#sdkcommandschangedmessage)를 참조하세요                                                                                                                                                                                                                                                |
| `supportedModels()`                    | 표시 정보가 있는 사용 가능한 모델을 반환합니다                                                                                                                                                                                                                                                                                                                                                      |
| `supportedAgents()`                    | [`AgentInfo`](#agentinfo)`[]`로 사용 가능한 서브에이전트를 반환합니다                                                                                                                                                                                                                                                                                                                             |
| `mcpServerStatus()`                    | 연결된 MCP 서버의 상태를 [`McpServerStatus`](#mcpserverstatus)`[]`로 반환합니다                                                                                                                                                                                                                                                                                                                |
| `getContextUsage(opts?)`               | 세션의 컨텍스트 창 사용량을 카테고리, 스킬 및 도구별로 분류하는 [`SDKControlGetContextUsageResponse`](#sdkcontrolgetcontextusageresponse)를 반환합니다. 기본 `detail`을 사용하면 대화형 세션에서 `/context`가 표시하는 것과 동일한 데이터이므로 토큰 수와 함께 Claude Code가 `/context` 사용량 그리드를 그리는 데 사용하는 `color` 및 `gridRows` 같은 표시 필드를 전달합니다. [`detail` 옵션](#sdkcontrolgetcontextusageresponse)은 에이전트 SDK v0.3.257 이상이 필요합니다                      |
| `readFile(path, options?)`             | 세션의 파일 시스템에서 파일을 읽습니다. Claude Code는 경로를 `cwd`에 대해 해결합니다. [readFile()이 읽을 수 있는 것](#what-readfile-can-read)은 제공하는 파일을 나열합니다. 읽기 상한을 변경하려면 `{ maxBytes }`를 전달하세요(기본값 1 MB, 상한 10 MB). 이미지 같은 바이너리 파일의 경우 `{ encoding: 'base64' }`를 전달하세요. 권한 거부, 누락된 파일 또는 전송 오류 시 [`SDKControlReadFileResponse`](#sdkcontrolreadfileresponse) 또는 `null`로 해결됩니다. TypeScript SDK v0.2.121 이상이 필요합니다 |
| `reloadSkills()`                       | 디스크에서 스킬을 다시 로드하여 중간 세션에서 추가하거나 편집한 스킬을 실행 중인 세션에서 사용할 수 있도록 합니다. 다시 로드 후 사용 가능한 스킬을 나열하는 [`SDKControlReloadSkillsResponse`](#sdkcontrolreloadskillsresponse)로 해결됩니다. 에이전트 SDK v0.3.163 이상이 필요합니다                                                                                                                                                                               |
| `accountInfo()`                        | 계정 정보를 반환합니다                                                                                                                                                                                                                                                                                                                                                                    |
| `reconnectMcpServer(serverName)`       | 이름으로 MCP 서버를 다시 연결합니다. 이름이 `.mcp.json` 또는 `~/.claude.json` 같은 설정 파일의 항목과도 일치하면 Claude Code는 설정 파일 항목이 아닌 [`mcpServers`](#options) 또는 `setMcpServers()`를 통해 구성한 서버를 다시 연결합니다. 해당 해결 순서는 Claude Code v2.1.257 이상이 필요합니다                                                                                                                                                           |
| `toggleMcpServer(serverName, enabled)` | `reconnectMcpServer()`와 동일한 이름 해결을 사용하여 이름으로 MCP 서버를 활성화 또는 비활성화합니다. 비활성화하면 서버를 연결 해제합니다                                                                                                                                                                                                                                                                                        |
| `setMcpServers(servers)`               | 이 세션의 MCP 서버 집합을 동적으로 대체합니다. 추가되고 제거된 서버 및 오류를 명시하는 [`McpSetServersResult`](#mcpsetserversresult)로 해결됩니다                                                                                                                                                                                                                                                                        |
| `readMcpResource(serverName, uri)`     | *알파.* 연결된 MCP 서버에서 하나의 MCP Apps `ui://` 리소스를 읽어 애플리케이션이 도구의 위젯을 렌더링할 수 있도록 합니다. [`SDKControlMcpReadResourceResponse`](#sdkcontrolmcpreadresourceresponse)로 해결됩니다. TypeScript 에이전트 SDK v0.3.280 이상이 필요합니다                                                                                                                                                                        |
| `streamInput(stream)`                  | 다중 턴 대화를 위해 쿼리에 입력 메시지를 스트리밍합니다                                                                                                                                                                                                                                                                                                                                                 |
| `stopTask(taskId)`                     | ID로 실행 중인 백그라운드 작업을 중지합니다                                                                                                                                                                                                                                                                                                                                                       |
| `close()`                              | 쿼리를 닫고 기본 프로세스를 종료합니다. 쿼리를 강제로 종료하고 모든 리소스를 정리합니다                                                                                                                                                                                                                                                                                                                               |

<h4 id="applyflagsettings">
  `applyFlagSettings()`
</h4>

실행 중인 세션에서 [설정](/docs/ko/settings)을 변경하고 쿼리를 다시 시작하지 않습니다. 신뢰할 수 없는 입력을 읽은 후 `permissions`를 강화하는 것처럼 전용 설정자가 없는 설정이 중간 세션에서 변경되어야 할 때 사용합니다. `setModel()` 및 `setPermissionMode()`는 이 두 키에 대한 전용 설정자입니다. `applyFlagSettings()`는 설정 키의 모든 부분 집합을 허용하는 일반 형식이며, 여기에 `model`을 전달하는 것은 `setModel()`과 동일하게 동작합니다.

일부 키만 중간 세션에서 적용됩니다:

* **다음 턴에 적용됨**: `effortLevel`, `ultracode`, `permissions`, `hooks`, `skillOverrides`, `fastMode`, `agent`. `agent`를 전환하면 해당 에이전트의 모델 재정의 및 훅도 다음 턴에 적용됩니다. 시스템 프롬프트는 다음 턴에 적용되거나, [기록된 시스템 프롬프트를 재사용](/docs/ko/agent-sdk/modifying-system-prompts#change-the-prompt-of-an-existing-session)하는 세션에서 세션이 압축되면 적용됩니다.
* **현재 턴 중에 적용됨**: `model`. Claude가 턴에서 작업 중일 때 `model`을 전환하면 Claude가 이미 생성 중인 응답은 이전 모델에서 완료되고 Claude Code가 모델에 수행하는 다음 호출부터 시작하는 턴의 나머지는 새 모델을 사용합니다. 서브에이전트는 자신의 모델을 유지합니다. v2.1.212 이전에는 중간 턴 전환이 다음 턴을 기다렸습니다.
* **중간 세션에 영향 없음**: 시스템 프롬프트 옵션입니다. 이는 시작 시 한 번 해결되므로 실행 중인 세션은 호출이 성공하더라도 원본 값을 유지합니다. 변경하려면 새 세션을 시작하세요.

`effortLevel`은 [노력 수준](/docs/ko/model-config#adjust-effort-level) 이름을 허용합니다. 또한 `"ultracode"`를 허용하며, 이는 [ultracode](/docs/ko/workflows#let-claude-decide-with-ultracode)가 켜진 `xhigh` 노력을 요청합니다. `applyFlagSettings()`는 해당 값 없이 `effortLevel`을 선언하므로 TypeScript에서 동등한 `{ ultracode: true }`를 전달하세요. `ultracode` 값은 Claude Code v2.1.203 이상이 필요하며 설정 파일의 `effortLevel` 키가 아닌 `applyFlagSettings()`에서만 허용됩니다.

값은 플래그 설정 계층에 작성되며, 이는 시작 시 `query()`의 인라인 `settings` 옵션이 채우는 계층과 동일합니다. 이는 [온페이지 우선 순위 섹션](#settings-precedence)이 프로그래밍 옵션이라고 부르는 계층과 동일합니다.

연속 호출은 최상위 키를 얕게 병합합니다. `{ permissions: {...} }` 포함 두 번째 호출은 이전 호출의 전체 `permissions` 객체를 깊게 병합하는 대신 대체합니다.

플래그 계층에서 키를 지우려면 해당 키에 `null`을 전달하세요. 그러면 대부분의 키가 먼저 시작 시 `query()`의 `settings` 옵션이 설정한 값으로 폴백한 다음 낮은 우선 순위 소스로 폴백합니다. 지워진 `model`은 설정 파일이 `model`을 설정하더라도 [Claude Code의 기본 모델](/docs/ko/model-config)로 재설정됩니다. `undefined`를 전달하면 JSON 직렬화가 이를 삭제하므로 효과가 없습니다.

`model` 외에 세 가지 키는 폴백하는 대신 세션 상태를 재설정합니다:

* `effortLevel: null`은 설정 파일의 `effortLevel`이 아닌 모델의 기본 노력 수준으로 세션을 반환합니다. `query()`의 `effort` 옵션도 복원하지 않습니다.
* `agent: null`은 다음 턴부터 메인 스레드를 에이전트 없이 실행합니다. 설정 파일의 `agent`도 복원하지 않습니다. 지워진 에이전트가 자신의 모델을 적용했으면 세션은 시작 시 해결한 모델로 돌아갑니다.
* `ultracode: null`은 `false`처럼 ultracode를 끕니다. 설정 파일의 `ultracode` 값을 복원하지 않습니다. 세션은 현재 노력 수준을 유지하므로 같은 호출에서 `effortLevel`을 전달하여 변경하세요.

스트리밍 입력 모드에서만 사용 가능하며, 이는 `setModel()` 및 `setPermissionMode()`와 동일한 제약입니다.

아래 예제는 중간 세션에서 활성 모델을 전환한 다음 재정의를 지워 모델이 [Claude Code의 기본 모델](/docs/ko/model-config)로 재설정되도록 합니다.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const q = query({ prompt: messageStream });

// 세션의 나머지 부분에 대해 모델을 재정의합니다
await q.applyFlagSettings({ model: "claude-opus-4-6" });

// 나중에: 재정의를 지웁니다. 모델이 Claude Code의 기본값으로 재설정됩니다
await q.applyFlagSettings({ model: null });
```

<Note>
  `applyFlagSettings()`는 TypeScript 전용입니다. Python SDK는 동등한 메서드를 노출하지 않습니다.
</Note>

<h4 id="updatesettings">
  `updateSettings()`
</h4>

설정을 디스크의 설정 파일에 작성하여 값이 나중 세션에 지속되도록 합니다. 각 소스는 하나의 키를 허용하며, 문자열 값을 가집니다:

* **`"localSettings"`**: `outputStyle`을 허용하고 프로젝트의 로컬 설정 파일 `.claude/settings.local.json`에 병합합니다. 새 스타일은 세션의 다음 요청에서 적용됩니다.
* **`"userSettings"`**: `effortLevel`을 허용하고 세션의 현재 모델에 대한 기본 [노력 수준](/docs/ko/model-config#adjust-effort-level)으로 사용자 설정 파일의 [`modelSettings`](/docs/ko/settings-reference#modelsettings) 아래에 저장합니다. `max`를 전달하면 아무것도 작성되지 않습니다. `max`는 세션 전용이기 때문입니다. 실행 중인 세션은 어느 쪽이든 현재 노력 수준을 유지하므로 변경하려면 [`applyFlagSettings()`](#applyflagsettings)를 호출하세요. 이 소스는 TypeScript SDK v0.3.277 이상이 필요하며, Claude Code v2.1.277을 번들합니다.

호출은 요청이 다른 키를 전달할 때, 세션이 원격 전송을 통해 실행될 때, 세션의 [`settingSources`](#options)가 명시한 소스를 제외할 때 거부됩니다. 키 삭제는 지원되지 않습니다.

<h3 id="warmquery">
  `WarmQuery`
</h3>

[`startup()`](#startup)에서 반환된 핸들입니다. 서브프로세스가 이미 생성되고 초기화되었으므로 이 핸들에서 `query()`를 호출하면 시작 지연 없이 준비된 프로세스에 프롬프트를 직접 작성합니다.

```typescript theme={null}
interface WarmQuery extends AsyncDisposable {
  query(prompt: string | AsyncIterable<SDKUserMessage>): Query;
  close(): void;
}
```

<h4 id="methods-2">
  메서드
</h4>

| 메서드             | 설명                                                                                     |
| :-------------- | :------------------------------------------------------------------------------------- |
| `query(prompt)` | 사전 준비된 서브프로세스에 프롬프트를 보내고 [`Query`](#query-object)를 반환합니다. `WarmQuery`당 한 번만 호출할 수 있습니다 |
| `close()`       | 프롬프트를 보내지 않고 서브프로세스를 닫습니다. 더 이상 필요하지 않은 준비된 쿼리를 버릴 때 사용합니다                             |

`WarmQuery`는 `AsyncDisposable`을 구현하므로 자동 정리를 위해 `await using`과 함께 사용할 수 있습니다.

<h3 id="sdkcontrolinitializeresponse">
  `SDKControlInitializeResponse`
</h3>

`initializationResult()`의 반환 유형입니다. 세션 초기화 데이터를 포함합니다.

```typescript theme={null}
type SDKControlInitializeResponse = {
  commands: SlashCommand[];
  agents: AgentInfo[];
  output_style: string;
  available_output_styles: string[];
  models: ModelInfo[];
  account: AccountInfo;
  fast_mode_state?: "off" | "cooldown" | "on";
  fast_mode_disabled_reason?: FastModeDisabledReason;
  hooks_applied?: boolean;
};
```

`hooks_applied`는 Claude Code가 `initialize` 요청이 전달한 `hooks`를 등록했는지 보고합니다. SDK는 세션이 시작될 때 한 번 해당 요청을 보내고 각 [`reinitialize()`](#query-object) 호출에서 다시 보냅니다. 필드는 에이전트 SDK v0.3.238 이상이 필요합니다.

요청이 훅을 전달하지 않으면 Claude Code는 필드를 생략합니다. 요청이 훅을 전달하면 값은 요청이 세션의 첫 번째 초기화인지 여부와 반복된 요청의 경우 세션에 도달한 방식에 따라 달라집니다:

* `true`: Claude Code가 훅을 등록했습니다. 세션의 첫 번째 초기화는 이 값을 반환합니다. CLI의 stdin을 통해 전송된 반복 초기화도 `true`를 반환합니다. 이 경우 새 요청의 훅이 이전에 등록된 훅을 대체합니다.
* `false`: Claude Code가 훅을 무시했습니다. 원격 세션으로 전송된 반복 초기화는 이 값을 반환하므로 세션에 참여하는 두 번째 클라이언트는 첫 번째 클라이언트가 등록한 훅을 대체할 수 없습니다.

에이전트 SDK v0.3.238 이전에는 응답이 필드를 전달하지 않았고 Claude Code는 모든 반복 초기화에서 `hooks`를 무시했습니다.

응답은 항상 `fast_mode_state`를 보고하고, [빠른 모드](/docs/ko/fast-mode)를 차단하는 것이 있으면 `fast_mode_disabled_reason`은 이유 코드를 함께 전달하므로 가용성을 다시 파생시키는 대신 차단된 상태를 설명할 수 있습니다. 두 동작 모두 Claude Code v2.1.219 이상이 필요합니다. v2.1.219 이전에는 빠른 모드를 사용할 수 없을 때 응답이 `fast_mode_state`를 생략했고 절대 이유를 전달하지 않았습니다. 이유 코드 및 의미는 결과 메시지의 [`fast_mode_disabled_reason`](#sdkresultmessage)을 참조하세요.

성공적인 `initialize`에 대한 제어 응답 래퍼는 `pending_permission_requests` 배열도 전달합니다. 필드는 위의 `SDKControlInitializeResponse` 페이로드가 아닌 응답 래퍼 자체에 있습니다. 각 항목은 세션이 실행 중일 때 권한 요청에 대해 스트리밍하는 것과 동일한 `{ type: "control_request", request_id, request }` 형태의 완전한 `control_request` 메시지입니다.

배열은 이 Claude Code 프로세스가 발행했고 아직 해결하지 않은 권한 요청을 나열합니다. SDK는 배열을 읽고 각 항목을 [`canUseTool`](#canusetool) 콜백으로 전달하며, 이는 전송 간격 후 [`reinitialize()`](#query-object)가 트리거하는 동일한 재전달입니다. 연결이 끊어지기 전에 콜백이 이미 수신한 요청을 반복할 수 있으므로 반복된 요청 ID를 멱등성으로 처리하세요.

배열은 성공적인 `initialize` 응답에서 항상 존재하며 이 프로세스에 미해결 권한 요청이 없으면 비어 있습니다. Claude Code v2.1.268 이상이 필요합니다. 이전 버전은 필드를 생략할 수 있으므로 와이어 프로토콜을 직접 구문 분석하면 누락된 필드를 이전 CLI로 취급하고 아무것도 대기 중이 아니라는 증거로 취급하지 마세요.

<h3 id="sdkcontrolinterruptresponse">
  `SDKControlInterruptResponse`
</h3>

중단 수신: [`interrupt()`](#query-object)가 [`SDKSystemMessage.capabilities`](#sdksystemmessage)에서 `interrupt_receipt_v1` 기능을 광고하는 CLI에서 해결되는 값입니다. Claude Code v2.1.205 이상이 필요합니다. 이전 CLI는 빈 성공 페이로드로 중단에 응답하므로 `interrupt()`는 `undefined`로 해결됩니다.

```typescript theme={null}
type SDKControlInterruptResponse = {
  still_queued: string[];
  cancelled?: string[];
};
```

`still_queued`는 중단이 도착했을 때 대기 중이던 사용자 메시지의 UUID를 나열합니다. 대기열에 여전히 있는 메시지와 Claude Code가 이미 다음 턴을 위해 대기열에서 꺼낸 메시지입니다. 세션의 첫 턴이 시작된 후 Claude Code는 중단하지 않으면 나열된 메시지를 처리하고 여러 메시지를 한 턴으로 병합할 수 있습니다. 첫 턴이 시작되기 전에 중단하면 Claude Code는 시작되는 즉시 해당 턴을 중단하고 해당 턴의 나열된 메시지는 응답을 받지 않습니다.

수신을 사용하여 다시 보낼 것을 결정하세요. 나열된 메시지를 취소하지 않으면 응답을 받는지 여부에 관계없이 대화에 들어가므로 다시 보내면 Claude에 두 번 전달됩니다.

이 주의 사항으로 목록을 해석하세요:

* UUID가 있는 메시지만 나타납니다. 빈 배열은 다른 것도 실행되지 않는다는 의미가 아닙니다.
* 메인 스레드 메시지만 나열됩니다. 서브에이전트로 주소 지정된 메시지는 범위를 벗어납니다.
* 목록에는 클라이언트가 보낸 적 없는 UUID(예: [예약된 작업](/docs/ko/scheduled-tasks) 트리거)가 포함될 수 있습니다. 오류로 취급하는 대신 인식하지 못하는 UUID를 무시하세요.

제어 프로토콜을 `interrupt()` 대신 직접 구동하는 클라이언트는 `interrupt` 제어 요청에서 `cancel_queued: true`를 설정할 수 있습니다. Claude Code v2.1.219 이상은 [`SDKSystemMessage.capabilities`](#sdksystemmessage)에서 `interrupt_cancel_queued_v1` 기능으로 지원을 광고합니다. 이전 CLI는 필드를 무시하고 대기 중인 메시지를 평소대로 실행합니다. 이러한 중단은 `still_queued` 아래에 나열될 모든 메시지를 취소합니다. 수신은 `cancelled` 아래에 나열하고, `still_queued`는 비어 있으며, 아무것도 실행되지 않습니다.

`cancelled` 목록은 `still_queued`와 동일한 주의 사항을 전달합니다. `interrupt()` 메서드는 절대 `cancel_queued`를 보내지 않으므로 해결되는 수신은 `cancelled`를 전달하지 않습니다.

수신은 중단이 처리되는 순간의 스냅샷이며 깨끗한 중단에서 중단된 턴의 [`SDKResultMessage`](#sdkresultmessage) 전에 도착합니다. 해당 결과 후 수신을 읽으세요. 루프는 즉시 다음 대기 중인 턴을 시작하므로 결과 후 검사하는 대기열이 이미 변경되었습니다.

<h3 id="sdkcontrolgetcontextusageresponse">
  `SDKControlGetContextUsageResponse`
</h3>

[`getContextUsage()`](#query-object)의 반환 유형입니다. 기본 `detail`을 사용하면 이는 Claude Code가 대화형 세션에서 `/context` 명령어에 대해 렌더링하는 것과 동일한 페이로드이므로 토큰 수와 함께 Claude Code가 `/context` 사용량 그리드를 그리는 데 사용하는 `color` 및 `gridRows` 같은 표시 필드를 전달합니다.

메서드의 선택적 `detail` 인수는 Claude Code가 각 카테고리를 계산하는 방식을 선택합니다. 기본값 `'full'`을 사용하면 Claude Code는 토큰 계산 API 요청으로 각 카테고리를 계산합니다. 대신 `{ detail: 'summary' }`를 전달하여 마지막 응답의 사용량 및 로컬 추정에서 답변을 가져옵니다. 토큰 계산 요청이 나가지 않으며 카테고리별 숫자는 대략적입니다. `detail` 인수는 에이전트 SDK v0.3.257 이상이 필요합니다.

프롬프트 대신 `/context`를 메서드로 보내면 Claude Code는 결과를 전달하는 어시스턴트 메시지의 `context_usage` 필드에 [`SDKContextUsage`](#sdkcontextusage) 페이로드를 첨부합니다. 해당 필드는 에이전트 SDK v0.3.232 이상이 필요합니다.

```typescript theme={null}
type SDKControlGetContextUsageResponse = {
  categories: {
    name: string;
    tokens: number;
    color: string;
    isDeferred?: boolean;
  }[];
  totalTokens: number;
  maxTokens: number;
  rawMaxTokens: number;
  percentage: number;
  gridRows: {
    color: string;
    isFilled: boolean;
    categoryName: string;
    tokens: number;
    percentage: number;
    squareFullness: number;
  }[][];
  model: string;
  memoryFiles: {
    path: string;
    type: string;
    tokens: number;
  }[];
  mcpTools: {
    name: string;
    serverName: string;
    tokens: number;
    isLoaded?: boolean;
  }[];
  deferredBuiltinTools?: {
    name: string;
    tokens: number;
    isLoaded: boolean;
  }[];
  systemTools?: {
    name: string;
    tokens: number;
  }[];
  systemPromptSections?: {
    name: string;
    tokens: number;
  }[];
  agents: {
    agentType: string;
    source: string;
    tokens: number;
  }[];
  slashCommands?: {
    totalCommands: number;
    includedCommands: number;
    tokens: number;
  };
  skills?: {
    totalSkills: number;
    includedSkills: number;
    tokens: number;
    skillFrontmatter: {
      name: string;
      source: string;
      tokens: number;
    }[];
  };
  autoCompactThreshold?: number;
  isAutoCompactEnabled: boolean;
  messageBreakdown?: {
    toolCallTokens: number;
    toolResultTokens: number;
    attachmentTokens: number;
    assistantMessageTokens: number;
    userMessageTokens: number;
    redirectedContextTokens: number;
    unattributedTokens: number;
    toolCallsByType: {
      name: string;
      callTokens: number;
      resultTokens: number;
    }[];
    attachmentsByType: {
      name: string;
      tokens: number;
    }[];
  };
  apiUsage: {
    input_tokens: number;
    output_tokens: number;
    cache_creation_input_tokens: number;
    cache_read_input_tokens: number;
  } | null;
};
```

컬렉션 필드에서 토큰 귀속을 읽으세요:

* `categories`는 카테고리별 합계를 보유합니다.
* `mcpTools` 및 `agents`는 개별 MCP 도구 및 서브에이전트에 토큰을 귀속합니다.
* `memoryFiles`는 각 로드된 메모리 파일을 비용과 함께 나열합니다.
* `skills.skillFrontmatter`는 포함된 각 스킬에 스킬 목록의 토큰을 귀속합니다. 스킬별 수는 Claude Code가 실제로 보내는 각 스킬의 목록 항목을 측정하며, 스킬의 전체 프론트매터보다 짧을 수 있습니다. `skills.totalSkills`를 `skills.includedSkills`와 비교하여 발견된 모든 스킬이 목록에 포함되었는지 확인하세요.

`totalTokens`는 세션의 현재 컨텍스트 사용량이고 `maxTokens`는 사용량이 측정되는 창입니다. 해당 창은 모델의 컨텍스트 창이거나 적용되는 낮은 자동 압축 창입니다. `rawMaxTokens`는 `maxTokens`와 동일한 값을 전달하고 `percentage`는 해당 창의 반올림된 백분율로 `totalTokens`입니다.

Claude Code는 선택적 `deferredBuiltinTools`, `systemTools` 및 `systemPromptSections` 진단을 설정하지 않으므로 유형이 선언하더라도 없을 것으로 예상하세요.

<h3 id="sdkcontrolreadfileresponse">
  `SDKControlReadFileResponse`
</h3>

[`readFile()`](#query-object)의 반환 유형입니다.

```typescript theme={null}
type SDKControlReadFileResponse = {
  contents: string;
  absPath: string;
  truncated?: boolean;
  encoding?: 'base64';
};
```

`contents`는 파일 텍스트 또는 `encoding: 'base64'`를 요청했을 때 base64 데이터를 보유합니다. 응답의 `encoding` 필드는 해당 경우 `'base64'`로 설정됩니다. `absPath`는 해결된 절대 경로입니다. `truncated`는 파일이 `maxBytes` 상한보다 길고 내용이 해당 한계에서 잘렸을 때 설정됩니다.

<h4 id="what-readfile-can-read">
  readFile()이 읽을 수 있는 것
</h4>

`readFile()`은 Read 도구보다 더 좁은 파일 집합을 제공합니다:

* `cwd` 및 `additionalDirectories` 같은 세션의 작업 디렉토리 중 하나 내의 일반 파일
* 세션의 도구 결과 같은 Claude Code 자체 파일 중 일부

Read 거부 및 요청 규칙은 여전히 일치하는 경로를 차단하고, 광범위한 Read 허용 규칙은 `readFile()`에 나머지 파일 시스템을 열지 않습니다. 다른 것의 경우 호출은 `null`로 해결됩니다.

<h3 id="sdkcontrolreloadskillsresponse">
  `SDKControlReloadSkillsResponse`
</h3>

[`reloadSkills()`](#query-object)의 반환 유형입니다.

```typescript theme={null}
type SDKControlReloadSkillsResponse = {
  skills: SlashCommand[];
};
```

`skills`는 다시 로드 후 사용 가능한 스킬을 나열하며, `supportedCommands()`가 반환하는 것과 동일한 [`SlashCommand`](#slashcommand) 형태입니다.

<h3 id="sdkcontrolmcpreadresourceresponse">
  `SDKControlMcpReadResourceResponse`
</h3>

[`readMcpResource()`](#query-object)의 반환 유형으로, MCP 서버의 `resources/read` 결과를 전달합니다. TypeScript 에이전트 SDK v0.3.280 이상이 필요합니다.

```typescript theme={null}
type SDKControlMcpReadResourceResponse = {
  contents: {
    uri: string;
    mimeType?: string;
    text?: string;
    blob?: string;
    _meta?: Record<string, unknown>;
  }[];
};
```

`readMcpResource()`에 `mcpServerStatus()`가 보고하는 서버 이름과 `ui://` URI(예: 도구가 [`_meta`](#mcpserverstatus)에서 선언하는 `ui.resourceUri`)를 전달하세요. 호출은 다른 URI 스킴, 애플리케이션이 자체 호스팅하는 [SDK MCP 서버](#createsdkmcpserver), 연결되지 않은 서버에 대해 거부됩니다. init 메시지의 [`capabilities`](#sdksystemmessage)에 `mcp_read_resource_v1`이 포함될 때 사용 가능합니다.

각 `contents` 항목은 서버가 보낸 하나의 콘텐츠 항목입니다. `blob`은 바이너리 항목의 base64 데이터를 보유하고, `_meta`는 항목 자체의 `_meta`이며, MCP Apps 서버는 리소스의 `ui.csp` 및 `ui.permissions`를 여기에 넣습니다. 콘텐츠는 신뢰할 수 없는 제3자 HTML이므로 샌드박스에서 렌더링하세요.

<h3 id="agentdefinition">
  `AgentDefinition`
</h3>

프로그래밍 방식으로 정의된 서브에이전트의 구성입니다.

```typescript theme={null}
type AgentDefinition = {
  description: string;
  tools?: string[];
  disallowedTools?: string[];
  prompt: string;
  model?: string;
  mcpServers?: AgentMcpServerSpec[];
  skills?: string[];
  initialPrompt?: string;
  maxTurns?: number;
  background?: boolean;
  omitClaudeMd?: boolean;
  memory?: "user" | "project" | "local";
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | number;
  permissionMode?: PermissionMode;
  criticalSystemReminder_EXPERIMENTAL?: string;
};
```

| 필드                                    | 필수  | 설명                                                                                                                                                                                                          |
| :------------------------------------ | :-- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`                         | 예   | 이 에이전트를 사용할 때를 설명하는 자연어 설명                                                                                                                                                                                  |
| `tools`                               | 아니오 | 허용된 도구 이름의 배열입니다. 생략하면 [서브에이전트에서 사용 가능한 모든 도구](/docs/ko/sub-agents#available-tools)를 상속합니다. 스킬을 에이전트의 컨텍스트에 미리 로드하려면 여기에 `'Skill'`을 나열하는 대신 `skills` 필드를 사용하세요                                                   |
| `disallowedTools`                     | 아니오 | 이 에이전트에 대해 명시적으로 거부할 도구 이름의 배열입니다. MCP 서버 수준 패턴도 허용됩니다. `mcp__server` 또는 `mcp__server__*`는 해당 서버의 모든 도구를 제거하고 `mcp__*`는 모든 서버의 모든 MCP 도구를 제거합니다                                                             |
| `prompt`                              | 예   | 에이전트의 시스템 프롬프트                                                                                                                                                                                              |
| `model`                               | 아니오 | 이 에이전트의 모델 재정의입니다. `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'` 같은 별칭 또는 전체 모델 ID를 허용합니다. `'inherit'`는 메인 모델을 사용합니다. 생략하면 Claude Code는 [서브에이전트 모델 순서](/docs/ko/sub-agents#choose-a-model)에서 모델을 선택합니다 |
| `mcpServers`                          | 아니오 | 이 에이전트의 MCP 서버 사양입니다                                                                                                                                                                                        |
| `skills`                              | 아니오 | 에이전트 컨텍스트에 미리 로드할 스킬 이름의 배열                                                                                                                                                                                 |
| `initialPrompt`                       | 아니오 | 이 에이전트가 메인 스레드 에이전트로 실행될 때 첫 번째 사용자 턴으로 자동 제출됩니다                                                                                                                                                            |
| `maxTurns`                            | 아니오 | 중지하기 전 최대 에이전트 턴(API 왕복) 수                                                                                                                                                                                  |
| `background`                          | 아니오 | 호출될 때 이 에이전트를 비차단 백그라운드 작업으로 실행합니다                                                                                                                                                                          |
| `omitClaudeMd`                        | 아니오 | 이 에이전트가 서브에이전트로 실행될 때 사용자, 프로젝트 및 로컬 CLAUDE.md 파일 없이 실행합니다. 관리 정책 파일은 여전히 로드됩니다. 에이전트 도구 프롬프트에서 필요한 모든 것을 가져오는 에이전트에 사용합니다. 이 에이전트가 메인 스레드 에이전트로 실행될 때는 무시됩니다. TypeScript 에이전트 SDK v0.3.271 이상이 필요합니다       |
| `memory`                              | 아니오 | 이 에이전트의 메모리 소스: `'user'`, `'project'` 또는 `'local'`                                                                                                                                                          |
| `effort`                              | 아니오 | 이 에이전트의 추론 노력 수준입니다. 명명된 수준 또는 정수를 허용합니다                                                                                                                                                                    |
| `permissionMode`                      | 아니오 | 이 에이전트 내 도구 실행의 권한 모드입니다. [서브에이전트 상속 규칙](/docs/ko/agent-sdk/permissions#available-modes)은 적용 시기를 결정합니다. [`PermissionMode`](#permissionmode)를 참조하세요                                                               |
| `criticalSystemReminder_EXPERIMENTAL` | 아니오 | 실험적: 시스템 프롬프트에 추가된 중요 알림                                                                                                                                                                                    |

<h3 id="agentmcpserverspec">
  `AgentMcpServerSpec`
</h3>

서브에이전트에서 사용 가능한 MCP 서버를 지정합니다. 서버 이름(부모의 `mcpServers` 구성에서 서버를 참조하는 문자열) 또는 서버 이름을 구성으로 매핑하는 인라인 서버 구성 레코드일 수 있습니다.

```typescript theme={null}
type AgentMcpServerSpec = string | Record<string, McpServerConfigForProcessTransport>;
```

여기서 `McpServerConfigForProcessTransport`는 `McpStdioServerConfig | McpSSEServerConfig | McpHttpServerConfig | McpSdkServerConfig`입니다.

<h3 id="settingsource">
  `SettingSource`
</h3>

SDK가 설정을 로드할 파일 시스템 기반 구성 소스를 제어합니다.

```typescript theme={null}
type SettingSource = "user" | "project" | "local";
```

| 값           | 설명                                            | 위치                            |
| :---------- | :-------------------------------------------- | :---------------------------- |
| `'user'`    | 전역 사용자 설정                                     | `~/.claude/settings.json`     |
| `'project'` | 공유 프로젝트 설정(버전 제어됨)                            | `.claude/settings.json`       |
| `'local'`   | 로컬 프로젝트 설정, Claude Code가 설정을 저장할 때 gitignored | `.claude/settings.local.json` |

<h4 id="default-behavior">
  기본 동작
</h4>

`settingSources`가 생략되거나 `undefined`일 때 `query()`는 Claude Code CLI와 동일한 파일 시스템 설정을 로드합니다. 사용자, 프로젝트 및 로컬입니다. [settingSources가 제어하지 않는 것](/docs/ko/agent-sdk/claude-code-features#what-settingsources-does-not-control)을 참조하여 이 옵션에 관계없이 읽는 입력과 비활성화 방법을 확인하세요.

<h4 id="why-use-settingsources">
  settingSources를 사용하는 이유
</h4>

**파일 시스템 설정 비활성화:**

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// 디스크에서 사용자, 프로젝트 또는 로컬 설정을 로드하지 마세요
const result = query({
  prompt: "Analyze this code",
  options: { settingSources: [] }
});
```

**특정 설정 소스만 로드:**

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// 프로젝트 설정만 로드, 사용자 및 로컬 무시
const result = query({
  prompt: "Run CI checks",
  options: {
    settingSources: ["project"] // .claude/settings.json만
  }
});
```

CLAUDE.md 프로젝트 지침을 로드하려면 `settingSources`에 `"project"`를 포함하세요. CLAUDE.md 로드가 시스템 프롬프트 옵션과 상호작용하는 방식은 [시스템 프롬프트 수정](/docs/ko/agent-sdk/modifying-system-prompts#claude-md-files-for-project-level-instructions)을 참조하세요.

<h4 id="settings-precedence">
  설정 우선 순위
</h4>

여러 소스가 로드될 때 설정은 이 우선 순위(높음에서 낮음)로 병합됩니다:

1. 로컬 설정 (`.claude/settings.local.json`)
2. 프로젝트 설정 (`.claude/settings.json`)
3. 사용자 설정 (`~/.claude/settings.json`)

`agents`, `allowedTools` 및 `settings` 같은 프로그래밍 옵션은 사용자, 프로젝트 및 로컬 파일 시스템 설정을 재정의합니다. 관리 정책 설정은 프로그래밍 옵션보다 우선합니다.

<h3 id="permissionmode">
  `PermissionMode`
</h3>

```typescript theme={null}
type PermissionMode =
  | "default" // 표준 권한 동작
  | "acceptEdits" // 파일 편집 자동 수락
  | "bypassPermissions" // 권한 검사 우회; 명시적 요청 규칙은 여전히 프롬프트
  | "plan" // 계획 모드 - 편집 없이 탐색
  | "dontAsk" // 권한에 대해 프롬프트하지 마세요, 사전 승인되지 않으면 거부
  | "auto"; // 모델 분류기가 권한 프롬프트를 승인 또는 거부
```

<h3 id="canusetool">
  `CanUseTool`
</h3>

도구 사용을 제어하기 위한 사용자 정의 권한 함수 유형입니다.

함수는 대화형 권한 프롬프트의 SDK 대체입니다. [권한 평가 흐름](/docs/ko/agent-sdk/permissions#how-permissions-are-evaluated)이 프롬프트로 해결될 때만 호출됩니다. `allowedTools` 항목, 설정 허용 규칙 또는 `acceptEdits` 또는 `bypassPermissions` 같은 권한 모드에 의해 이미 승인된 도구 호출은 절대 호출하지 않습니다. 모든 도구 호출을 게이트하려면 [`PreToolUse` 훅](/docs/ko/agent-sdk/hooks)을 대신 사용하세요.

허용 규칙은 [모든 모드가 자동 승인하지 않는 작업](/docs/ko/permission-modes#actions-no-mode-auto-approves)을 사전 승인하지 않습니다. [권한이 평가되는 방식](/docs/ko/agent-sdk/permissions#how-permissions-are-evaluated)을 참조하여 콜백에 도달하는 것과 `dontAsk` 및 `auto` 모드에서 발생하는 것을 확인하세요.

```typescript theme={null}
type CanUseTool = (
  toolName: string,
  input: Record<string, unknown>,
  options: {
    signal: AbortSignal;
    suggestions?: PermissionUpdate[];
    blockedPath?: string;
    mcpServer?: { name: string; source: string };
    decisionReason?: string;
    toolUseID: string;
    agentID?: string;
    requestId: string;
  }
) => Promise<PermissionResult | null>;
```

| 옵션               | 유형                                          | 설명                                                                                                                                                                                                       |
| :--------------- | :------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `signal`         | `AbortSignal`                               | 작업을 중단해야 하면 신호됩니다                                                                                                                                                                                        |
| `suggestions`    | [`PermissionUpdate`](#permissionupdate)`[]` | 사용자가 이 도구에 대해 다시 프롬프트되지 않도록 제안된 권한 업데이트입니다. Bash 프롬프트는 `localSettings` [대상](#permissionupdatedestination)이 있는 제안을 포함하므로 `updatedPermissions`에서 반환하면 규칙을 `.claude/settings.local.json`에 작성하고 세션 간에 지속됩니다. |
| `blockedPath`    | `string`                                    | 해당하는 경우 권한 요청을 트리거한 파일 경로입니다                                                                                                                                                                             |
| `mcpServer`      | `{ name: string; source: string }`          | `mcp__*` 도구의 경우 해당 도구를 제공하는 MCP 서버 및 해당 서버의 정의가 나온 위치([`McpServerProvenance`](#mcpserverprovenance)의 필드 포함). 다른 도구의 경우 없습니다. 에이전트 SDK v0.3.274 이상이 필요합니다                                                 |
| `decisionReason` | `string`                                    | 이 권한 요청이 트리거된 이유를 설명합니다                                                                                                                                                                                  |
| `toolUseID`      | `string`                                    | 어시스턴트 메시지 내 이 특정 도구 호출의 고유 식별자                                                                                                                                                                           |
| `agentID`        | `string`                                    | 서브에이전트 내에서 실행 중인 경우 서브에이전트의 ID                                                                                                                                                                           |
| `requestId`      | `string`                                    | `control_request` 봉투의 `request_id`입니다. 애플리케이션이 SDK 외부의 자체 채널(예: 서명된 HTTP POST)을 통해 보내는 `control_response`는 이 값을 에코해야 Claude Code 프로세스가 응답을 요청과 일치시킬 수 있습니다                                               |

콜백은 일반적으로 [`PermissionResult`](#permissionresult)를 반환하여 요청을 해결하며, SDK는 이를 전송을 통해 `control_response`로 다시 작성합니다. 애플리케이션이 이미 이 요청에 대해 자체 채널을 통해 `control_response`를 보낸 경우에만 `null`을 반환하고 `requestId`를 에코합니다. SDK는 전송에 응답을 작성하는 것을 건너뜁니다. 다른 경우에 `null`을 반환하면 `control_response`가 절대 보내지지 않고 권한 프롬프트가 시간 초과되지 않으므로 도구 호출이 무한정 차단됩니다.

`requestId` 옵션과 `null` 반환 값은 Claude Code v2.1.199 이상이 필요합니다.

<h3 id="permissionresult">
  `PermissionResult`
</h3>

권한 검사의 결과입니다.

```typescript theme={null}
type PermissionResult =
  | {
      behavior: "allow";
      updatedInput?: Record<string, unknown>;
      updatedPermissions?: PermissionUpdate[];
      toolUseID?: string;
    }
  | {
      behavior: "deny";
      message: string;
      interrupt?: boolean;
      toolUseID?: string;
    };
```

<h3 id="toolconfig">
  `ToolConfig`
</h3>

기본 제공 도구 동작의 구성입니다.

```typescript theme={null}
type ToolConfig = {
  askUserQuestion?: {
    previewFormat?: "markdown" | "html";
  };
};
```

| 필드                              | 유형                     | 설명                                                                                                                                     |
| :------------------------------ | :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------- |
| `askUserQuestion.previewFormat` | `'markdown' \| 'html'` | [`AskUserQuestion`](/docs/ko/agent-sdk/user-input#question-format) 옵션의 `preview` 필드를 옵트인하고 콘텐츠 형식을 설정합니다. 설정하지 않으면 Claude는 미리 보기를 내보내지 않습니다 |

<h3 id="mcpserverconfig">
  `McpServerConfig`
</h3>

MCP 서버의 구성입니다.

```typescript theme={null}
type McpServerConfig =
  | McpStdioServerConfig
  | McpSSEServerConfig
  | McpHttpServerConfig
  | McpSdkServerConfigWithInstance;
```

<h4 id="mcpstdioserverconfig">
  `McpStdioServerConfig`
</h4>

```typescript theme={null}
type McpStdioServerConfig = {
  type?: "stdio";
  command: string;
  args?: string[];
  env?: Record<string, string>;
};
```

<h4 id="mcpsseserverconfig">
  `McpSSEServerConfig`
</h4>

```typescript theme={null}
type McpSSEServerConfig = {
  type: "sse";
  url: string;
  headers?: Record<string, string>;
};
```

<h4 id="mcphttpserverconfig">
  `McpHttpServerConfig`
</h4>

```typescript theme={null}
type McpHttpServerConfig = {
  type: "http";
  url: string;
  headers?: Record<string, string>;
};
```

<h4 id="mcpsdkserverconfigwithinstance">
  `McpSdkServerConfigWithInstance`
</h4>

```typescript theme={null}
type McpSdkServerConfigWithInstance = {
  type: "sdk";
  name: string;
  timeout?: number;
  instance: McpServer;
};
```

<h4 id="mcpclaudeaiproxyserverconfig">
  `McpClaudeAIProxyServerConfig`
</h4>

```typescript theme={null}
type McpClaudeAIProxyServerConfig = {
  type: "claudeai-proxy";
  url: string;
  id: string;
};
```

<h3 id="sdkpluginconfig">
  `SdkPluginConfig`
</h3>

SDK에서 플러그인을 로드하기 위한 구성입니다.

```typescript theme={null}
type SdkPluginConfig = {
  type: "local";
  path: string;
  skipMcpDiscovery?: boolean;
};
```

| 필드                 | 유형        | 설명                                                                                                                              |
| :----------------- | :-------- | :------------------------------------------------------------------------------------------------------------------------------ |
| `type`             | `'local'` | `'local'`이어야 합니다(현재 로컬 플러그인만 지원됨)                                                                                               |
| `path`             | `string`  | 플러그인 디렉토리의 절대 또는 상대 경로                                                                                                          |
| `skipMcpDiscovery` | `boolean` | `true`일 때 SDK는 이 플러그인에서 스킬, 훅, 에이전트 및 명령어를 로드하지만 `.mcp.json` 또는 매니페스트 `mcpServers`를 읽지 않습니다. 애플리케이션이 플러그인의 MCP 연결을 소유할 때 설정하세요. |

**예제:**

```typescript theme={null}
plugins: [
  { type: "local", path: "./my-plugin" },
  { type: "local", path: "/absolute/path/to/plugin" }
];
```

플러그인 생성 및 사용에 대한 완전한 정보는 [플러그인](/docs/ko/agent-sdk/plugins)을 참조하세요.

<h2 id="message-types">
  메시지 타입
</h2>

<h3 id="sdkmessage">
  `SDKMessage`
</h3>

쿼리에서 반환되는 모든 가능한 메시지의 합집합 타입입니다.

```typescript theme={null}
type SDKMessage =
  | SDKAssistantMessage
  | SDKUserMessage
  | SDKUserMessageReplay
  | SDKResultMessage
  | SDKSystemMessage
  | SDKPartialAssistantMessage
  | SDKCompactBoundaryMessage
  | SDKStatusMessage
  | SDKLocalCommandOutputMessage
  | SDKHookStartedMessage
  | SDKHookProgressMessage
  | SDKHookResponseMessage
  | SDKPluginInstallMessage
  | SDKToolProgressMessage
  | SDKAuthStatusMessage
  | SDKTaskNotificationMessage
  | SDKTaskStartedMessage
  | SDKTaskProgressMessage
  | SDKTaskUpdatedMessage
  | SDKBackgroundTasksChangedMessage
  | SDKThinkingTokensMessage
  | SDKSessionStateChangedMessage
  | SDKWorkerShuttingDownMessage
  | SDKCommandsChangedMessage
  | SDKNotificationMessage
  | SDKFilesPersistedEvent
  | SDKToolUseSummaryMessage
  | SDKMemoryRecallMessage
  | SDKRateLimitEvent
  | SDKElicitationCompleteMessage
  | SDKPermissionDeniedMessage
  | SDKPromptSuggestionMessage
  | SDKAPIRetryMessage
  | SDKMirrorErrorMessage
  | SDKInformationalMessage
  | SDKConversationResetMessage;
```

<h3 id="sdkassistantmessage">
  `SDKAssistantMessage`
</h3>

어시스턴트 응답 메시지입니다.

```typescript theme={null}
type SDKAssistantMessage = {
  type: "assistant";
  uuid: UUID;
  session_id: string;
  message: BetaMessage; // From Anthropic SDK
  parent_tool_use_id: string | null;
  error?: SDKAssistantMessageError;
  aborted?: true;
  timestamp?: string;
  context_usage?: SDKContextUsage;
  user_message_uuid?: string;
  user_message_uuids?: string[];
};
```

`message` 필드는 Anthropic SDK의 [`BetaMessage`](https://platform.claude.com/docs/en/api/messages/create)입니다. `id`, `content`, `model`, `stop_reason`, `usage` 같은 필드를 포함합니다.

`SDKAssistantMessageError`는 다음 중 하나입니다: `'authentication_failed'`, `'oauth_org_not_allowed'`, `'account_on_hold'`, `'billing_error'`, `'rate_limit'`, `'overloaded'`, `'invalid_request'`, `'model_not_found'`, `'server_error'`, `'max_output_tokens'`, `'cloud_credential_error'`, 또는 `'unknown'`. 이 중 네 개의 값은 이름이 나타내는 것보다 더 많은 의미를 가집니다:

* `'model_not_found'`: 선택한 모델이 존재하지 않거나 계정이나 배포에서 사용할 수 없음
* `'overloaded'`: API가 서버가 용량에 도달했기 때문에 529를 반환했으며, 할당량에 대한 429인 `'rate_limit'`과는 다름
* `'account_on_hold'`: [계정이 보류 중](/docs/ko/errors#your-account-is-on-hold)
* `'cloud_credential_error'`: Claude Code가 실행되는 머신에서 사용 가능한 AWS 또는 Google Cloud 자격증명을 얻을 수 없어서 클라우드 제공자에게 요청이 도달하지 않았습니다. 일반적인 원인은 해당 머신에서 만료되었거나 완료되지 않은 클라우드 로그인이지만, 일시적으로 도달할 수 없는 자격증명 서비스도 동일한 값을 보고합니다. [AWS 또는 Google Cloud 자격증명을 로드할 수 없음](/docs/ko/errors#could-not-load-aws-or-google-cloud-credentials)을 참조하세요. TypeScript Agent SDK v0.3.267 이상 필요하며, Claude Code v2.1.267을 번들로 포함합니다.

`aborted`는 인터럽트 또는 중단이 스트림이 완료되기 전에 어시스턴트 메시지를 잘랐을 때 `true`입니다: 메시지에는 `stop_reason`이 없고 콘텐츠가 단어 중간에 끝날 수 있습니다. 이 필드는 정상적으로 완료된 메시지에는 없습니다. Agent SDK v0.3.214 이상이 필요합니다.

Claude Code는 [`user_message_uuid`](#user_message_uuid)의 조건에 따라 턴의 첫 번째 어시스턴트 메시지에 `user_message_uuid`와 `user_message_uuids`를 설정합니다.

`timestamp`는 메시지의 콘텐츠가 생성을 완료한 ISO 8601 시간입니다. 값은 해당 머신의 시계에서 나오므로 표시 목적으로만 사용하고 메시지를 순서대로 정렬하지 마세요. 하나의 API 턴은 동일한 `message.id`를 공유하지만 각각 고유한 `timestamp`를 가진 여러 어시스턴트 메시지를 생성할 수 있습니다. 필드가 없으면 메시지를 받은 시간으로 돌아가세요.

`context_usage`는 `/context` 보고서의 구조화된 복사본이며, [`SDKContextUsage`](#sdkcontextusage)로 입력되고 Agent SDK v0.3.232 이상이 필요합니다. 프롬프트로 `/context`를 보낼 때, Claude Code는 보고서를 어시스턴트 메시지로 전달하며, 그 `message.content`는 마크다운 테이블을 보유하고, 동일한 메시지에 `context_usage`를 첨부합니다. Claude Code는 다른 어시스턴트 메시지에는 필드를 설정하지 않으며, 이전 버전은 `/context` 테이블을 없이 전달하므로, 필드가 있을 때 분석을 읽고 없을 때 마크다운 텍스트로 돌아가세요.

<h3 id="sdkusermessage">
  `SDKUserMessage`
</h3>

사용자 입력 메시지입니다.

```typescript theme={null}
type SDKUserMessage = {
  type: "user";
  uuid?: UUID;
  session_id?: string;
  message: MessageParam; // From Anthropic SDK
  pasted_content?: MessageParam["content"][];
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  shouldQuery?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
  inline_pastes?: string[];
};
```

사용자가 입력 UI에 붙여넣은 콘텐츠를 입력한 것이 아니라 보내도록 `pasted_content`를 설정하고, 붙여넣기당 하나의 항목을 설정하며, 각각은 문자열 또는 콘텐츠 블록의 배열입니다. Claude Code는 각 항목의 텍스트를 입력된 텍스트 뒤에 순서대로 추가하고, 각 붙여넣기를 `<pasted_content>` 태그로 감쌀 수 있습니다. 텍스트 이외의 블록은 무시되므로 이미지와 문서는 `message.content`에서 보내세요. Agent SDK v0.3.277 이상이 필요합니다.

`shouldQuery`를 `false`로 설정하여 어시스턴트 턴을 트리거하지 않고 메시지를 트랜스크립트에 추가합니다. 메시지는 보류되고 턴을 트리거하는 다음 사용자 메시지로 병합됩니다. 이를 사용하여 모델 호출을 소비하지 않고 대역 외에서 실행한 명령의 출력과 같은 컨텍스트를 주입합니다.

`tool_result` 블록을 전달하는 메시지에서 `tool_use_result`는 모델로 전송된 텍스트가 아니라 도구의 구조화된 출력 객체입니다. 해당 형태는 일치하는 `tool_use` 블록으로 명명된 도구에 따라 다르므로 필드는 `unknown`으로 입력됩니다. 기본 제공 형태는 [도구 출력 타입](#tool-output-types) 아래에 나열되어 있습니다.

`Agent` 도구의 경우 `tool_use_result`는 [`AgentOutput`](#agent-2)입니다. `completed` 결과에서 `content`는 Claude Code가 `tool_result` 텍스트에 추가하는 에이전트 ID 및 사용량 트레일러 없이 서브에이전트의 보고서를 보유하므로 해당 텍스트를 구문 분석하는 대신 `tool_use_result`에서 렌더링합니다.

결과에 `resource_link` 블록이 포함된 MCP 도구의 경우 `tool_use_result`는 [`SDKMcpResourceLink`](#sdkmcpresourcelink) 항목의 `resourceLinks` 배열을 가진 객체입니다. Claude는 각 링크를 `tool_result` 블록의 텍스트 줄로 받으므로 해당 텍스트를 구문 분석하는 대신 `resourceLinks`를 읽어 서버가 반환한 파일을 렌더링합니다. Claude Code는 결과에 링크가 없을 때 `resourceLinks`를 생략하고 서브에이전트의 결과에서 생략하며, 결과당 최대 50개의 링크를 유지하고, 배열이 64 KiB의 직렬화된 JSON에 도달하면 링크 추가를 중지합니다. `resourceLinks`는 Agent SDK v0.3.257 이상이 필요합니다.

사용자가 입력한 것이 아니라 붙여넣은 `message.content`의 부분을 알려주도록 `inline_pastes`를 설정하고, 붙여넣기당 하나의 문자열을 설정합니다. 프롬프트 텍스트는 사용자가 배치한 위치에 유지됩니다. Claude Code는 각 나열된 붙여넣기를 `<pasted_content>` 태그로 감쌀 수 있으므로 Claude는 붙여넣은 자료를 사용자 자신의 단어와 구별할 수 있습니다. 프롬프트의 마지막 텍스트 블록의 붙여넣기만 래핑됩니다. TypeScript Agent SDK v0.3.280 이상이 필요합니다.

<h3 id="sdkusermessagereplay">
  `SDKUserMessageReplay`
</h3>

필수 UUID가 있는 재생된 사용자 메시지입니다.

```typescript theme={null}
type SDKUserMessageReplay = {
  type: "user";
  uuid: UUID;
  session_id: string;
  message: MessageParam;
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
  isReplay: true;
};
```

세션 외부에서 주입된 사용자 턴으로, [`origin`](#sdkmessageorigin) 종류가 `peer` 또는 `channel`인 턴은 활성 턴 중에 전달되었는지 또는 세션이 유휴 상태일 때 새 턴을 시작했는지 여부에 관계없이 재생으로 스트림에 도달합니다. v2.1.207 이전에는 세션이 유휴 상태일 때 전달된 주입된 턴이 스트림에 메시지를 생성하지 않았고 트랜스크립트를 다시 읽을 때만 나타났습니다.

<h3 id="sdkresultmessage">
  `SDKResultMessage`
</h3>

최종 결과 메시지입니다.

```typescript theme={null}
type SDKResultMessage =
  | {
      type: "result";
      subtype: "success";
      uuid: UUID;
      session_id: string;
      duration_ms: number;
      duration_api_ms: number;
      is_error: boolean;
      api_error_status?: number | null;
      num_turns: number;
      result: string;
      stop_reason: string | null;
      ttft_ms?: number;
      ttft_stream_ms?: number;
      user_message_uuid?: string;
      user_message_uuids?: string[];
      request_sent_wall_ms?: number;
      first_content_frame_ms?: number;
      first_stream_post_ms?: number;
      first_stream_post_ack_ms?: number;
      first_stream_post_wall_ms?: number;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      queued_turn_count?: number;
      structured_output?: unknown;
      deferred_tool_use?: { id: string; name: string; input: Record<string, unknown> };
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      fast_mode_disabled_reason?: FastModeDisabledReason;
      origin?: SDKMessageOrigin;
    }
  | {
      type: "result";
      subtype:
        | "error_max_turns"
        | "error_during_execution"
        | "error_max_budget_usd"
        | "error_max_structured_output_retries";
      uuid: UUID;
      session_id: string;
      duration_ms: number;
      duration_api_ms: number;
      is_error: boolean;
      num_turns: number;
      stop_reason: string | null;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      queued_turn_count?: number;
      errors: string[];
      startup_failure_reason?: SDKStartupFailureReason;
      user_message_uuid?: string;
      user_message_uuids?: string[];
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      fast_mode_disabled_reason?: FastModeDisabledReason;
      origin?: SDKMessageOrigin;
    };
```

결과의 여러 필드는 `subtype` 이상의 진단 세부 정보를 전달합니다:

* `api_error_status`: 대화를 종료한 API 오류의 HTTP 상태 코드입니다. 턴이 API 오류 없이 끝났을 때 없거나 `null`입니다.
* `ttft_ms`: 첫 번째 완전한 어시스턴트 메시지가 도착할 때 측정된 밀리초 단위의 첫 번째 토큰까지의 시간입니다. 성공 분기에만 있습니다.
* `ttft_stream_ms`: 응답 스트림이 열릴 때 첫 번째 `message_start` 스트림 이벤트까지의 밀리초 단위 시간입니다. `ttft_ms`보다 낮습니다. 두 사이의 간격은 첫 번째 메시지를 스트리밍하는 데 소요된 시간입니다. 성공 분기에만 있습니다.
* `user_message_uuid`: 이 턴이 답변한 메시지의 `uuid`입니다. 어떤 결과가 이를 전달하는지는 [`user_message_uuid`](#user_message_uuid)를 참조하세요.
* `user_message_uuids`: Claude Code가 이 턴에서 답변한 모든 메시지의 `uuid`입니다. [`user_message_uuids`](#user_message_uuids)를 참조하세요.
* `request_sent_wall_ms`: Claude Code가 API 요청을 발송한 에포크 밀리초로, 서버 측 타임스탬프와 조인하기 위한 것입니다. [`user_message_uuid`](#user_message_uuid)와 함께만 있으며, `is_error` false인 성공 결과에서 턴이 API 요청을 보냈을 때만 있습니다.
* `first_content_frame_ms`: 첫 번째 `content_block_start` 또는 `content_block_delta` 스트림 이벤트까지의 밀리초 단위 시간으로, 생각 블록을 콘텐츠로 계산합니다. 성공 분기에만 있으며, `is_error`가 false일 때만 있습니다. Agent SDK v0.3.260 이상이 필요합니다.
* `first_stream_post_ms`, `first_stream_post_ack_ms`, `first_stream_post_wall_ms`: 턴의 첫 번째 스트림 이벤트를 업로드하기 위한 타이밍입니다. Claude Code는 [클라우드 세션](/docs/ko/claude-code-on-the-web)과 같이 claude.ai로 스트리밍하는 세션에서만 기록하며, `query()`가 생성하는 결과는 이를 전달하지 않습니다. Agent SDK v0.3.260 이상이 필요합니다.
* `usage`: 메인 에이전트 루프만 해당합니다. 서브에이전트 및 보조 모델 호출을 제외하며, 스트리밍 입력 세션에서는 턴당입니다. 토큰/비용 회계를 위해 `modelUsage`를 선호합니다.
* `modelUsage`: 이 `query()` 호출 중에 쿼리 파이프라인을 통해 수행된 모든 모델 호출에 대한 모델별 합계로, 메인 루프, 서브에이전트, 압축 및 Workflow 에이전트와 같은 내부 호출을 포함합니다. 권한 분류자 및 토큰 계산 요청과 같은 해당 파이프라인 외부의 도우미 호출은 제외됩니다. 세션을 재개하는 호출은 [세션의 이전 호출에서 복원된 모델별 합계](/docs/ko/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls)도 계산합니다. 스트리밍 입력 세션에서 합계는 턴 전체에 누적되므로 결과 전체에서 합산하는 대신 최신 결과를 읽으세요. 재설정에 대해서는 [스트리밍 입력 모드에서 비용 추적](/docs/ko/agent-sdk/cost-tracking#track-costs-in-streaming-input-mode)을 참조하고 0으로 설정된 결과에 대해서는 [세션 충돌 후 합계 복구](/docs/ko/agent-sdk/cost-tracking#recover-totals-after-a-session-crash)를 참조하세요.
* `total_cost_usd`: USD의 누적 예상 비용으로, `modelUsage`와 동일한 호출을 포함하고 동일한 지점에서 재설정됩니다. 세션을 재개하는 호출은 [세션의 이전 호출에서 복원된 합계](/docs/ko/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls)도 계산합니다. 이는 청구 명세서가 아닌 추정치입니다. 정확도 주의 사항은 [비용 및 사용량 추적](/docs/ko/agent-sdk/cost-tracking)을 참조하세요.
* `queued_turn_count`: Claude Code가 결과를 생성했을 때 여전히 대기 중인 `origin: { kind: "human" }`으로 보낸 메시지의 수입니다. `0`과 없는 필드가 무엇을 의미하는지는 [`queued_turn_count`](#queued_turn_count)를 참조하세요.
* `startup_failure_reason`: Claude Code가 시작을 거부한 이유로, 알려진 시작 실패 전에 작성하는 `error_during_execution` 결과에서 확인할 수 있습니다. 값과 어떤 실패가 이를 전달하는지는 [`startup_failure_reason`](#startup_failure_reason)을 참조하세요. Agent SDK v0.3.274 이상이 필요합니다.
* `terminal_reason`: 루프가 끝난 이유입니다. `"completed"`, `"max_turns"`, `"tool_deferred"`, `"aborted_streaming"`, `"aborted_tools"`, `"hook_stopped"`, `"stop_hook_prevented"`, `"background_requested"`, `"blocking_limit"`, `"rapid_refill_breaker"`, `"prompt_too_long"`, `"image_error"`, `"model_error"`, `"api_error"`, `"malformed_tool_use_exhausted"`, `"budget_exhausted"`, `"structured_output_retry_exhausted"`, `"tool_deferred_unavailable"`, 또는 `"turn_setup_failed"` 중 하나입니다.
* `fast_mode_state`: `"on"`, `"off"`, 또는 `"cooldown"` 중 하나입니다.
* `fast_mode_disabled_reason`: [빠른 모드](/docs/ko/fast-mode)를 지금 사용할 수 없는 이유입니다. 빠른 모드를 차단하는 것이 없을 때는 없지만, 요청이 여전히 표준 속도로 실행될 수 있습니다. 빠른 모드 속도 제한 후 쿨다운 중에 Claude Code는 이유 코드 없이 `fast_mode_state: "cooldown"`을 보고하고 쿨다운이 만료되면 빠른 모드를 다시 활성화합니다. Claude Code v2.1.219 이상이 필요합니다.

이유 코드를 사용하여 자신의 UI에서 빠른 모드가 꺼진 이유를 설명하는 대신 가용성을 다시 도출합니다. 각 코드는 빠른 모드를 차단한 검사의 이름을 지정합니다:

| 이유 코드                  | 의미                                                                                                                        |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `free`                 | 계정에 빠른 모드가 필요로 하는 유료 구독 또는 사용 크레딧이 없음                                                                                     |
| `preference`           | 조직이 빠른 모드를 비활성화함                                                                                                          |
| `extra_usage_disabled` | 계정에 대해 사용 크레딧이 꺼짐                                                                                                         |
| `network_error`        | [가용성 검사](/docs/ko/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways)가 `api.anthropic.com`에 도달할 수 없음                      |
| `unknown`              | Claude Code가 가용성을 결정할 수 없음                                                                                                |
| `not_first_party`      | 세션이 Anthropic API 이외의 제공자를 사용함                                                                                            |
| `disabled_by_env`      | [`CLAUDE_CODE_DISABLE_FAST_MODE`](/docs/ko/env-vars)이 설정됨                                                                      |
| `model_not_allowed`    | 빠른 모드 Opus 모델이 조직의 [`availableModels`](/docs/ko/model-config#restrict-model-selection) 허용 목록에 없음                               |
| `sdk_opt_in_required`  | 세션이 빠른 모드에 옵트인하지 않음: [`settings`](#options) 옵션 또는 [`applyFlagSettings()`](#applyflagsettings)를 통해 `fastMode: true`를 전달합니다 |
| `pending`              | 가용성 검사가 아직 완료되지 않음                                                                                                        |

동일한 필드 쌍이 [`SDKSystemMessage`](#sdksystemmessage)와 [`SDKControlInitializeResponse`](#sdkcontrolinitializeresponse)에 나타나므로 첫 번째 턴 전에 빠른 모드 상태를 읽을 수 있습니다.

`origin` 필드는 이 결과를 트리거한 사용자 메시지의 [`SDKMessageOrigin`](#sdkmessageorigin)을 전달합니다. SDK가 완료된 백그라운드 작업과 같은 합성 후속 턴을 주입할 때, 결과 `SDKResultMessage`는 `origin: { kind: "task-notification" }`을 전달합니다. 트리거가 발생하고 서버 검증 메시지가 다른 세션에서 도착하는 루틴은 [작업 알림 서브종류](#task-notification-subkinds)에 설명된 `subkind`와 함께 이 종류도 도착합니다. `kind`를 확인하여 프롬프트에 답변하는 결과를 라우팅하거나 억제하기 전에 주입된 후속 작업과 구별합니다. 애플리케이션이 [예약된 실행을 선언](#declare-a-scheduled-run)하면 해당 결과도 `kind: "task-notification"`을 전달하므로 `kind`만으로 억제하지 마세요.

여러 백그라운드 작업 완료가 함께 대기 중일 때, Claude Code는 각각 하나의 턴이 아니라 하나의 턴에서 답변할 수 있습니다. 각 완료는 여전히 이 원점으로 자신의 결과를 생성합니다. Claude Code가 함께 답변하는 완료 중 마지막을 제외한 모든 것은 순서대로 `num_turns: 0`인 빈 결과를 생성하고, 마지막 것의 결과는 모두에 답변하는 턴을 전달합니다.

필드는 시작 오류와 같이 사용자 턴 전에 내보낸 결과에는 없습니다.

`PreToolUse` 훅이 `permissionDecision: "defer"`를 반환할 때, 결과는 `stop_reason: "tool_deferred"`를 가지며 `deferred_tool_use`는 보류 중인 도구의 `id`, `name`, `input`을 전달합니다. 이 필드를 읽어 자신의 UI에서 요청을 표시한 다음 동일한 `session_id`로 재개하여 계속합니다. 전체 왕복은 [나중을 위해 도구 호출 연기](/docs/ko/hooks#defer-a-tool-call-for-later)를 참조하세요.

<h4 id="user_message_uuid">
  `user_message_uuid`
</h4>

턴이 답변하는 [`SDKUserMessage`](#sdkusermessage)의 `uuid`로, Claude Code의 회신을 보낸 메시지와 일치시킬 수 있도록 에코됩니다. Claude Code는 메시지에 하나를 설정한 경우에만 `uuid`를 에코합니다. 필드는 `SDKUserMessage`에서 선택 사항이며, `query()`에 전달된 문자열 프롬프트는 없습니다.

턴이 답변하는 메시지는 턴이 시작된 방식에 따라 다릅니다:

* **보낸 일반 메시지**, 즉 `isSynthetic: true` 없음: 턴은 전체 실행 동안 해당 메시지에 답변합니다. 여러 메시지를 가깝게 보내면 Claude Code는 이를 하나의 턴으로 병합할 수 있으며, 필드는 마지막 메시지의 `uuid`만 전달합니다. 병합된 메시지 중 하나에 회신을 일치시키려면 [`user_message_uuids`](#user_message_uuids)를 사용합니다.
* **`isSynthetic: true`로 보낸 메시지**: 턴은 처음에 해당 메시지에 답변합니다. Claude Code가 도구 호출 사이에 일반 메시지를 선택하면 턴은 그 이후로 선택된 메시지에 답변합니다. 합성 메시지의 `uuid` 에코는 Agent SDK v0.3.265 이상이 필요합니다. 이전 버전은 합성 턴에서 아무것도 에코하지 않습니다.
* **Claude Code가 자체 생성한 프롬프트**, 예를 들어 세션이 재시작된 후 중단된 작업을 계속하는 턴: 턴은 처음에 메시지에 답변하지 않으며 프레임은 에코를 전달하지 않습니다. Claude Code가 도구 호출 사이에 일반 메시지를 선택하면 턴은 그 이후로 해당 메시지에 답변합니다. 픽업 에코는 Agent SDK v0.3.265 이상이 필요합니다. 이전 버전은 이러한 턴에서 아무것도 에코하지 않습니다.

Claude Code는 세 가지 종류의 프레임에서 답변된 메시지의 `uuid`를 에코합니다:

* **결과**: 메시지에 답변한 턴의 모든 결과입니다. Agent SDK v0.3.265 이상에서 모든 그러한 결과가 이를 전달합니다. v0.3.265 이전에는 일반 메시지가 시작한 턴의 성공 결과가 턴이 API 요청을 보내지 않았거나 연기된 도구 호출로 끝났을 때 이를 전달하지 않았습니다. v0.3.246 이전에는 오류 결과도 이를 전달하지 않았으며, v0.3.216 이전에는 모든 결과가 이를 전달하지 않았습니다.
* **턴의 첫 번째 회신**: 첫 번째 [어시스턴트 메시지](#sdkassistantmessage) 또는 `includePartialMessages`를 사용하면 `event.type`이 `ping`이 아닌 첫 번째 [스트림 이벤트](#sdkpartialassistantmessage)로, 결과가 도착하기 전에 회신을 바인드할 수 있습니다. 턴이 아무것도 스트리밍하지 않으면 Claude Code는 대신 첫 번째 어시스턴트 메시지에 설정합니다. 첫 번째 회신 에코는 Agent SDK v0.3.246 이상이 필요합니다. 턴이 답변하는 메시지가 중간에 변경되면 변경 후 첫 번째 회신이 필드를 전달하며, Agent SDK v0.3.265 이상에서 필요합니다. 이전 버전은 턴당 하나의 회신 프레임에 설정합니다.
* **턴의 모든 [`thinking_tokens`](#sdkthinkingtokensmessage) 프레임**: 턴의 첫 번째 회신을 기다리지 않고 보낸 메시지에 생각 진행을 귀속시킬 수 있습니다. Agent SDK v0.3.260 이상이 필요합니다.

Claude Code는 다음 경우에 필드를 생략합니다:

* 첫 번째 회신 이외의 회신 프레임
* 서브에이전트 프레임
* `uuid`가 있는 메시지에 답변하지 않는 턴: 턴이 하나 없이 보낸 메시지에 답변했거나 Claude Code가 턴을 시작했고 하나가 있는 일반 메시지를 선택하지 않았습니다
* 보낸 메시지에 답변하지 않는 결과로, 충돌한 워커 프로세스 후 0으로 설정된 결과와 같습니다

<h4 id="user_message_uuids">
  `user_message_uuids`
</h4>

Claude Code가 이 턴에서 답변한 모든 메시지의 `uuid`입니다. 여러 메시지를 가깝게 보내면 Claude Code는 이를 하나의 턴으로 병합할 수 있으며, `user_message_uuid`는 마지막 메시지만 명명합니다. 병합된 메시지 중 하나에 회신을 일치시키려면 이 목록의 어디든지 해당 메시지의 `uuid`를 찾으세요. Agent SDK v0.3.259 이상이 필요합니다.

Claude Code는 해당 필드를 전달하는 각 회신 프레임과 결과에서 `user_message_uuid`와 함께 목록을 설정합니다. `user_message_uuid`를 전달하는 전체 프레임 세트와 각각이 필요로 하는 버전은 [`user_message_uuid`](#user_message_uuid)를 참조하세요. 목록은 항상 `user_message_uuid`를 포함하고 최대 64개 항목을 보유합니다.

Claude Code가 턴이 실행되는 동안 보낸 일반 메시지를 선택하면 해당 메시지의 `uuid`를 결과의 목록에 추가합니다.

첫 번째 회신 또는 결과가 목록 없이 `user_message_uuid`를 전달하면 이전 Claude Code 버전에서 나온 것이므로 단일 필드로 돌아가세요.

<h4 id="queued_turn_count">
  `queued_turn_count`
</h4>

Claude Code가 결과를 생성했을 때 [`origin: { kind: "human" }`](#sdkmessageorigin)으로 보낸 메시지 중 여전히 명령 큐에서 대기 중인 메시지의 수입니다. Agent SDK v0.3.242 이상이 필요합니다.

`0`과 없는 필드가 무엇을 의미하는지:

* **`0`**: Claude Code는 해당 `origin` 없이 보낸 메시지를 계산하지 않으며, 작업 알림을 계산하지 않으므로 턴이 여전히 따를 수 있습니다.
* **없음**: Claude Code가 충돌 또는 치명적 시작 오류 후 내보내는 최종 결과는 필드를 생략하며, [0으로 설정된 합계를 전달할 수 있습니다](/docs/ko/agent-sdk/cost-tracking#recover-totals-after-a-session-crash).

<h4 id="startup_failure_reason">
  `startup_failure_reason`
</h4>

Claude Code가 시작을 거부한 이유로, 애플리케이션이 재시도 대신 수정을 제공할 수 있습니다. Claude Code는 알려진 시작 실패 전에 종료하기 전에 작성하는 `error_during_execution` 결과에 설정합니다. 해당 결과는 0으로 설정된 합계를 전달하며, 해당 `errors` 배열은 stderr와 동일한 텍스트를 전달합니다. 필드는 다른 모든 결과에는 없습니다. Agent SDK v0.3.274 이상이 필요합니다.

모든 `SDKStartupFailureReason` 값에 대해 이 결과를 받으려면 [`env`](#options)에서 `CLAUDE_CODE_STARTUP_FAILURE_RESULTS`를 `1`로 설정합니다. 해당 변수 없이 Claude Code는 이러한 실패에 대해서만 결과를 작성하고 나머지는 stderr 출력, 0이 아닌 종료, 결과 메시지 없음으로 끝납니다:

* Claude Code가 [세션을 워크트리로 반환할 수 없기](/docs/ko/worktrees#the-session-resumes-outside-its-worktree) 때문에 중지하는 재개로, `worktree_unverified` 또는 `worktree_resume_refused`입니다. 해당 섹션은 어떤 오류가 어떤 값을 전달하는지 말합니다.
* 백그라운드 세션이 보유하는 대화의 거부된 [`continue`](#options)로, `session_held_by_background`입니다. 그러한 대화의 거부된 [`resume`](#options)의 경우 Claude Code는 변수가 설정되었을 때만 결과를 작성합니다.

```typescript theme={null}
type SDKStartupFailureReason =
  | "org_pin_api_key_conflict"
  | "org_verify_failed"
  | "org_pin_mismatch"
  | "managed_settings_invalid"
  | "remote_settings_required_unavailable"
  | "gateway_signin_required"
  | "gateway_access_denied"
  | "proxy_invalid"
  | "temp_dir_unusable"
  | "cwd_unavailable"
  | "shell_tool_missing"
  | "session_held_by_background"
  | "worktree_resume_refused"
  | "worktree_unverified"
  | "cli_version_too_old"
  | "bypass_root";
```

각 값은 하나의 거부를 명명합니다:

| 값                                      | 세션을 중지한 것                                                                                                                                            |
| :------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------- |
| `org_pin_api_key_conflict`             | 관리 설정이 [첫 번째 당사자 또는 Cloud 게이트웨이 로그인](/docs/ko/authentication#restrict-login-to-your-organization)을 요구하며, Anthropic API 키, 인증 토큰 또는 `apiKeyHelper`가 대신 구성됨 |
| `org_verify_failed`                    | 로그인의 조직을 핀에 대해 확인할 수 없음(예: 네트워크 실패 또는 취소된 토큰)                                                                                                        |
| `org_pin_mismatch`                     | 로그인이 핀이 허용하지 않는 조직에 속함                                                                                                                               |
| `managed_settings_invalid`             | 관리 정책 설정을 읽을 수 없거나 핀이 조직을 명명하지 않음                                                                                                                    |
| `remote_settings_required_unavailable` | 조직이 요구하는 관리 설정을 로드할 수 없음                                                                                                                             |
| `gateway_signin_required`              | [Cloud 게이트웨이](/docs/ko/claude-apps-gateway)가 이 로그인을 종료함                                                                                                   |
| `gateway_access_denied`                | Cloud 게이트웨이에 대한 관리 설정 요청이 403으로 돌아왔으며, 게이트웨이의 [문제 해결 테이블](/docs/ko/claude-apps-gateway-deploy#troubleshooting)이 이를 다룹니다                                   |
| `proxy_invalid`                        | 프록시 설정이 완전한 URL이 아님                                                                                                                                  |
| `temp_dir_unusable`                    | 사용자별 임시 디렉토리가 안전하지 않거나 생성할 수 없음                                                                                                                      |
| `cwd_unavailable`                      | 작업 디렉토리가 삭제되었거나 이동되었거나 읽을 수 없음                                                                                                                       |
| `shell_tool_missing`                   | Windows에서 사용 가능한 셸 도구가 없음: Git Bash가 없고 PowerShell이 없거나 `CLAUDE_CODE_USE_POWERSHELL_TOOL`로 꺼짐                                                        |
| `session_held_by_background`           | 재개하거나 계속할 대화가 [백그라운드 세션](/docs/ko/agent-view)으로 실행 중                                                                                                      |
| `worktree_resume_refused`              | 세션의 워크트리가 안전 검사에 실패했거나 재개가 내부에서 시작됨. `errors`는 동일한 재개를 다시 실행하면 워크트리 없이 계속되는지 여부를 말합니다                                                                |
| `worktree_unverified`                  | 세션의 워크트리를 지금 확인할 수 없으며 재시도하면 성공할 수 있음                                                                                                                |
| `cli_version_too_old`                  | 이 Claude Code 버전이 Anthropic이 요구하는 최소값 아래                                                                                                             |
| `bypass_root`                          | 루트로 실행하는 동안 바이패스 권한 모드가 요청됨                                                                                                                          |

<h3 id="sdksystemmessage">
  `SDKSystemMessage`
</h3>

시스템 초기화 메시지입니다.

```typescript theme={null}
type SDKSystemMessage = {
  type: "system";
  subtype: "init";
  uuid: UUID;
  session_id: string;
  agents?: string[];
  apiKeySource: ApiKeySource;
  betas?: string[];
  claude_code_version: string;
  cwd: string;
  tools: string[];
  mcp_servers: {
    name: string;
    status: string;
    source?: string;
  }[];
  model: string;
  permissionMode: PermissionMode;
  slash_commands: string[];
  terminal_slash_commands?: string[];
  output_style: string;
  skills: string[];
  plugins: { name: string; path: string }[];
  fast_mode_state?: FastModeState;
  fast_mode_disabled_reason?: FastModeDisabledReason;
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | null;
  capabilities?: string[];
};
```

`fast_mode_state`는 세션의 [빠른 모드](/docs/ko/fast-mode) 상태를 보고합니다. 무언가가 빠른 모드를 차단할 때 `fast_mode_disabled_reason`은 이를 차단한 검사의 이름을 지정합니다. 필드는 Claude Code v2.1.219 이상이 필요합니다. 이유 코드와 그 의미는 결과 메시지의 [`fast_mode_disabled_reason`](#sdkresultmessage)을 참조하세요.

`terminal_slash_commands`는 `exit`와 같은 로컬 터미널에 바인드된 인터페이스를 가진 `slash_commands`의 항목을 명명합니다. 다른 `slash_commands` 항목처럼 보낼 수 있습니다. 필드는 원격 또는 모바일 클라이언트가 명령 메뉴에서 이를 숨길 수 있도록 존재합니다. 필드는 비어 있지 않을 때만 있으며 Agent SDK v0.3.229 이상이 필요합니다.

* 각 `mcp_servers` 항목의 `source`: 서버 정의가 어디에서 나왔는지로, [`McpServerStatus`](#mcpserverstatus)의 `source`와 동일한 값입니다. Agent SDK v0.3.274 이상이 필요합니다.
*

`effort`: [노력 수준](/docs/ko/model-config#adjust-effort-level) Claude Code가 세션의 다음 요청에서 보내거나 보내지 않을 때 `null`입니다. Claude Code는 [Remote Control](/docs/ko/remote-control) 클라이언트로 보내는 초기화 메시지에만 필드를 설정하고 애플리케이션이 읽는 초기화 메시지에서 생략합니다. Agent SDK v0.3.234 이상이 필요합니다.

`capabilities` 배열은 이 CLI가 구현하는 프로토콜 동작의 이름을 지정하므로 `claude_code_version` 문자열을 비교하는 대신 기능을 감지할 수 있습니다. 이는 열린 집합입니다: 인식하지 못하는 값을 무시하고 동작이 의존하는 특정 기능을 확인합니다. 필드는 Claude Code v2.1.205 이상이 필요하며 이전 CLI에는 없습니다.

| 기능                                                                                                                                                                                                                  | 의미                                                                                                                                     |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt_receipt_v1`                                                                                                                                                                                              | [`interrupt()`](#query-object)는 인터럽트가 도착했을 때 보류 중인 메시지를 나열하는 [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) 영수증으로 해결됩니다 |
| `interrupt_cancel_queued_v1`                                                                                                                                                                                        |                                                                                                                                        |
| `interrupt` 제어 요청이 `cancel_queued: true`를 준수하여 영수증이 `still_queued` 아래에 나열할 메시지를 취소하고 대신 `cancelled` 아래에 나열합니다. [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse)를 참조하세요. Claude Code v2.1.219 이상이 필요합니다 |                                                                                                                                        |

<h3 id="sdkpartialassistantmessage">
  `SDKPartialAssistantMessage`
</h3>

스트리밍 부분 메시지(`includePartialMessages`가 true일 때만). `parent_tool_use_id` 필드는 항상 `null`입니다: 스트림 이벤트는 메인 세션에만 내보내집니다. 서브에이전트 귀속의 경우 완전한 메시지를 사용하거나 [`forwardSubagentText`](#options)를 활성화하여 서브에이전트 텍스트 및 생각을 완전한 메시지로 받습니다.

```typescript theme={null}
type SDKPartialAssistantMessage = {
  type: "stream_event";
  event: BetaRawMessageStreamEvent; // From Anthropic SDK
  parent_tool_use_id: string | null;
  uuid: UUID;
  session_id: string;
  ttft_ms?: number; // Time to first token in ms, present only on message_start events
  user_message_uuid?: string;
  user_message_uuids?: string[];
};
```

Claude Code는 턴의 첫 번째 비핑 스트림 이벤트에 `user_message_uuid`와 `user_message_uuids`를 설정하고, 턴이 답변하는 메시지가 변경될 때 [`user_message_uuid`](#user_message_uuid)의 조건에 따라 다시 설정합니다.

<h3 id="sdkcompactboundarymessage">
  `SDKCompactBoundaryMessage`
</h3>

대화 압축 경계를 나타내는 메시지입니다.

```typescript theme={null}
type SDKCompactBoundaryMessage = {
  type: "system";
  subtype: "compact_boundary";
  uuid: UUID;
  session_id: string;
  compact_metadata: {
    trigger: "manual" | "auto";
    pre_tokens: number;
  };
};
```

<h3 id="sdkinformationalmessage">
  `SDKInformationalMessage`
</h3>

루프에서 내보낸 일반 텍스트 배너입니다. 비오류 상태 줄, `UserPromptSubmit` 훅의 블록 이유와 같은 훅 피드백, 명령 출력을 전달합니다. Claude Code v2.1.227 이상에서 훅의 [`systemMessage`](/docs/ko/hooks#json-output)는 이 메시지로 도착할 수 있으며, 각 줄은 `PostToolUse:Bash says:`와 같은 훅의 이름으로 접두사가 붙습니다. 훅의 `systemMessage`가 이 메시지로 도착하는지 여부는 이벤트에 따라 다릅니다. 각 [이벤트의 섹션](/docs/ko/hooks#hook-events)은 훅 페이지에서 출력이 어떻게 표시되는지 말합니다. `content`를 주어진 `level`에서 일반 텍스트로 렌더링합니다.

```typescript theme={null}
type SDKInformationalMessage = {
  type: "system";
  subtype: "informational";
  content: string;
  level: "info" | "notice" | "suggestion" | "warning";
  tool_use_id?: string;
  prevent_continuation?: boolean;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkworkershuttingdownmessage">
  `SDKWorkerShuttingDownMessage`
</h3>

정상적인 워커 해제 시 내보내져서 원격 클라이언트가 하트비트 타임아웃을 기다리는 대신 워커가 종료된 이유를 표시할 수 있습니다. `reason`은 호스트 CLI에서 설정한 짧은 snake\_case 문자열로, `"host_exit"` 또는 `"remote_control_disabled"`와 같습니다. 라이브 스트리밍할 때만 이에 대해 조치합니다. 재개된 세션은 이 메시지의 과거 인스턴스를 재생하므로 그 경우 무시합니다.

```typescript theme={null}
type SDKWorkerShuttingDownMessage = {
  type: "system";
  subtype: "worker_shutting_down";
  reason: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkplugininstallmessage">
  `SDKPluginInstallMessage`
</h3>

플러그인 설치 진행 이벤트입니다. [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/ko/env-vars)이 설정되었을 때 내보내져서 Agent SDK 애플리케이션이 첫 번째 턴 전에 마켓플레이스 플러그인 설치를 추적할 수 있습니다. `started`와 `completed` 상태는 전체 설치를 괄호로 묶습니다. `installed`와 `failed` 상태는 개별 마켓플레이스를 보고하고 `name`을 포함합니다.

```typescript theme={null}
type SDKPluginInstallMessage = {
  type: "system";
  subtype: "plugin_install";
  status: "started" | "installed" | "failed" | "completed";
  name?: string;
  error?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkpermissiondeniedmessage">
  `SDKPermissionDeniedMessage`
</h3>

권한 시스템이 대화형 프롬프트 없이 도구 호출을 거부할 때 내보낸 스트림 이벤트입니다. 뒤따르는 `is_error` 도구 결과만 관찰하는 대신 거부를 실시간으로 UI에 렌더링하는 데 사용합니다. 어떤 거부를 보고하는지는 실행이 권한 프롬프트를 처리하는 방식에 따라 다릅니다:

* **[`canUseTool`](#canusetool) 콜백과 기본 [`permissionPrompts: 'host'`](#options)**: 권한 프롬프트는 콜백으로 이동하고, 이 이벤트는 Claude Code가 이를 호출하지 않고 자체적으로 결정한 거부를 보고합니다.
* **둘 다 없음**: 베어 `-p` 실행 또는 `canUseTool`도 `permissionPromptToolName`도 설정하지 않는 `query()`는 프롬프트했을 모든 도구 호출을 거부하고, 이 이벤트는 그러한 거부와 Claude Code가 자체적으로 결정한 거부를 보고합니다. v2.1.223 이전에는 Claude Code가 콜백 없는 실행에서 이 이벤트를 내보내지 않았습니다.
* **MCP 프롬프트 도구**, `permissionPromptToolName` 또는 [`--permission-prompt-tool`](/docs/ko/cli-reference#cli-flags) 플래그로 설정하고 기본 `permissionPrompts: 'host'`: Claude Code는 이 이벤트를 전혀 내보내지 않으며, 규칙 거부를 포함하여 자체적으로 결정한 거부도 포함하지 않습니다.
* **[`permissionPrompts: 'none'`](#options)**: Claude Code는 프롬프트했을 호출을 거부하며, `canUseTool` 또는 MCP 프롬프트 도구도 설정되어 있을 때도 거부하고, 이 이벤트는 그러한 거부와 Claude Code가 자체적으로 결정한 거부를 보고합니다. Claude Code v2.1.259 이상이 필요합니다.

모든 구성에서 이 이벤트는 `PreToolUse` 훅 경로에서 결정된 거부를 건너뜁니다. 훅이 호출을 거부했는지 또는 거부 규칙이 훅의 허용 또는 요청 결정을 재정의했는지 여부입니다. 이벤트는 또한 최선의 노력입니다: 때때로 Claude Code는 이 이벤트를 내보내지 않고 거부를 기록하므로 [결과 메시지](#sdkresultmessage)의 `permission_denials`이 권위 있는 기록입니다.

```typescript theme={null}
type SDKPermissionDeniedMessage = {
  type: "system";
  subtype: "permission_denied";
  tool_name: string;
  tool_use_id: string;
  agent_id?: string;
  decision_reason_type?: string;
  decision_reason?: string;
  message: string;
  uuid: UUID;
  session_id: string;
};
```

| 필드                     | 타입       | 설명                                                                                |
| ---------------------- | -------- | --------------------------------------------------------------------------------- |
| `tool_name`            | `string` | 거부된 도구의 이름                                                                        |
| `tool_use_id`          | `string` | 이 거부가 답변하는 `tool_use` 블록의 ID                                                      |
| `agent_id`             | `string` | 거부된 호출이 서브에이전트 내부에서 발생했을 때 서브에이전트 ID입니다. 호스트 측 라우팅을 위해 `can_use_tool`의 필드를 미러링합니다 |
| `decision_reason_type` | `string` | 결정한 구성 요소의 판별자로, `"rule"`, `"mode"`, `"classifier"`, 또는 `"asyncAgent"`와 같습니다      |
| `decision_reason`      | `string` | 사용 가능할 때 결정 구성 요소의 인간이 읽을 수 있는 이유                                                 |
| `message`              | `string` | `tool_result`에서 모델로 반환된 거부 메시지                                                    |

<h3 id="sdkpermissiondenial">
  `SDKPermissionDenial`
</h3>

거부된 도구 사용에 대한 정보입니다.

```typescript theme={null}
type SDKPermissionDenial = {
  tool_name: string;
  tool_use_id: string;
  tool_input: Record<string, unknown>;
};
```

<h3 id="sdkcontextusage">
  `SDKContextUsage`
</h3>

`/context` 보고서의 구조화된 형태로, [`SDKAssistantMessage`](#sdkassistantmessage)에서 `/context` 결과를 전달하는 `context_usage`로 전달됩니다. Agent SDK v0.3.232 이상은 타입을 내보냅니다. [`SDKControlGetContextUsageResponse`](#sdkcontrolgetcontextusageresponse)와 달리 `color`와 `gridRows`와 같은 표시 필드 없이 사용량 분석을 렌더링하는 데 필요한 데이터만 전달합니다.

```typescript theme={null}
type SDKContextUsage = {
  model: string;
  total_tokens: number;
  raw_max_tokens: number;
  percentage: number;
  over_limit?: {
    tokens_over: number;
    kind: "hard_limit" | "compaction_window";
  };
  categories: SDKContextUsageCategory[];
  mcp_tools: {
    name: string;
    server_name: string;
    tokens: number;
  }[];
  memory_files: {
    path: string;
    type: string;
    tokens: number;
  }[];
  agents: {
    agent_type: string;
    source: string;
    tokens: number;
  }[];
  skills?: {
    name: string;
    source: string;
    plugin_name?: string;
    tokens: number;
  }[];
};
```

테이블은 Claude Code가 각 필드에 넣는 것을 나열합니다. `model`에서 `over_limit`까지의 필드는 세션 전체를 설명하고, 수집 필드는 개별 항목에 토큰을 귀속시킵니다.

| 필드               | 타입                                                        | 설명                                                                                                                                                                                                     |
| ---------------- | --------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `model`          | `string`                                                  | Claude Code가 사용량을 계산한 메인 루프의 모델로, 서브에이전트의 모델이 아님                                                                                                                                                       |
| `total_tokens`   | `number`                                                  | Claude Code의 사용 중인 토큰 추정치입니다. 윈도우에 고정되지 않으므로 세션이 제한을 초과할 때 `raw_max_tokens`를 초과할 수 있습니다                                                                                                                |
| `raw_max_tokens` | `number`                                                  | 모델의 컨텍스트 윈도우 또는 적용되는 낮은 [자동 압축 윈도우](/docs/ko/model-config#context-window-and-auto-compaction)로, 설정한 것 또는 1M 토큰 윈도우를 가진 일부 모델에 Claude Code가 적용하는 200K 경계와 같습니다. Claude Code는 이 윈도우에 대해 `total_tokens`를 측정합니다 |
| `percentage`     | `number`                                                  | `total_tokens`를 `raw_max_tokens`의 반올림된 백분율로, 세션이 제한을 초과할 때 100을 초과할 수 있습니다                                                                                                                             |
| `over_limit`     | `object`                                                  | `total_tokens`가 `raw_max_tokens`를 초과할 때만 있습니다. `tokens_over`는 초과 금액이고 `kind`는 Claude Code가 윈도우를 해결한 방식을 말합니다                                                                                           |
| `categories`     | [`SDKContextUsageCategory`](#sdkcontextusagecategory)`[]` | 사용량별 카테고리 분석의 각 행당 하나의 항목                                                                                                                                                                              |
| `mcp_tools`      | `object[]`                                                | 각 MCP 도구에 귀속된 토큰으로, `mcp__linear__create_issue`와 같은 와이어 이름과 `server_name`                                                                                                                              |
| `memory_files`   | `object[]`                                                | 각 로드된 메모리 파일에 귀속된 토큰으로, `path`와 `Project` 또는 `User`와 같은 소스 레이블이 `type`에 있습니다                                                                                                                           |
| `agents`         | `object[]`                                                | 각 사용자 정의 서브에이전트 정의에 귀속된 토큰으로, `projectSettings`, `userSettings`, 또는 `plugin`과 같은 소스 식별자입니다. 기본 제공 서브에이전트는 나열되지 않습니다                                                                                    |
| `skills`         | `object[]`                                                | 기술 목록의 각 기술에 귀속된 토큰으로, 소스 식별자와 플러그인 기술의 경우 `plugin_name`의 플러그인 이름입니다. 기술이 토큰에 기여하지 않을 때 없습니다                                                                                                           |

`over_limit.kind`는 Claude Code가 윈도우를 해결한 방식을 기록하며, 다음 요청을 API가 수락하는지 여부가 아닙니다:

* `hard_limit`: 윈도우는 Claude Code가 API가 요청을 거부하는 모델 자체의 제한이라고 믿는 것입니다
* `compaction_window`: 윈도우는 압축 정책 윈도우로, 모델의 제한과 일치할 수도 있고 아닐 수도 있습니다

Claude Code는 타입을 추가적으로 진화시켜 기존 타입을 재구성하는 대신 새로운 데이터를 선택적 필드로 추가합니다. 알고 있는 필드를 읽고 인식하지 못하는 필드는 무시합니다.

<h3 id="sdkcontextusagecategory">
  `SDKContextUsageCategory`
</h3>

`/context` 사용량별 카테고리 분석의 한 행입니다.

```typescript theme={null}
type SDKContextUsageCategory = {
  name: string;
  tokens: number;
  kind: "used" | "free" | "buffer" | "deferred";
};
```

테이블은 Claude Code가 행의 각 필드에 넣는 것을 나열합니다.

| 필드       | 타입       | 설명                                                                           |
| -------- | -------- | ---------------------------------------------------------------------------- |
| `name`   | `string` | `/context`가 인쇄하는 행의 표시 이름으로, `Messages`와 같습니다. 이름으로 행을 분류하지 말고 `kind`로 분류합니다 |
| `tokens` | `number` | 행의 토큰 수입니다. 행은 0개의 토큰을 전달할 수 있습니다                                            |
| `kind`   | `string` | 행이 나타내는 것: `used`, `free`, `buffer`, 또는 `deferred`                           |

각 `kind` 값은 행의 토큰이 무엇인지 말합니다:

* `used`: 컨텍스트 윈도우를 차지하는 콘텐츠
* `free`: 남은 윈도우
* `buffer`: 압축 예약
* `deferred`: Claude Code가 윈도우 밖에 보유하고 사용량 계산에서 제외하지만 인식을 위해 나열하는 도구 스키마

<h3 id="sdkmessageorigin">
  `SDKMessageOrigin`
</h3>

사용자 역할 메시지의 출처입니다. 이는 [`SDKUserMessage`](#sdkusermessage)에서 `origin`으로 나타나며 해당 [`SDKResultMessage`](#sdkresultmessage)로 전달되어 주어진 턴을 트리거한 것을 알 수 있습니다.

```typescript theme={null}
type SDKMessageOrigin =
  | { kind: "human" }
  | { kind: "channel"; server: string }
  | {
      kind: "peer";
      from: string;
      fromMode?: "bypass" | "prompting";
      name?: string;
      fromSession?: string;
      senderTaskId?: string;
      body?: string;
      verifiedPeerPid?: number;
    }
  | {
      kind: "task-notification";
      subkind?: "scheduled-trigger" | "peer-send-message";
      fireReason?: string;
    }
  | { kind: "coordinator" }
  | { kind: "auto-continuation" }
  | { kind: "unclassified" };
```

| `kind`              | 의미                                                                                                                                                                                                                                                                                                                                |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `human`             | 최종 사용자의 직접 입력입니다. 애플리케이션이 사용자가 입력한 것을 사용자 메시지로 전달하면 명시적으로 `origin`을 `{ kind: "human" }`으로 설정합니다: Claude Code는 `origin` 없는 사용자 메시지를 미귀속으로 취급하고, [`ultracode` 워크플로우 키워드](/docs/ko/workflows#ask-for-a-workflow-in-your-prompt)와 같이 인간이 입력한 프롬프트를 요구하는 검사는 이를 수락하지 않습니다. v2.1.210 이전에는 Claude Code가 사용자 메시지의 없는 `origin`을 인간 입력으로 취급했습니다. |
| `channel`           | [채널](/docs/ko/channels)에 도착하는 메시지입니다. `server`는 소스 MCP 서버 이름입니다.                                                                                                                                                                                                                                                                       |
| `peer`              | 다른 에이전트의 메시지: 프로세스 내 [팀원](/docs/ko/agent-teams) 또는 [교차 세션 피어](/docs/ko/cross-session-messaging), 다른 Claude Code 세션입니다. [피어 원점 필드](#peer-origin-fields)에서 필드별 의미와 신뢰 모델을 참조하세요.                                                                                                                                                              |
| `task-notification` | 완료된 백그라운드 작업과 같이 신선한 사용자 프롬프트 없이 도착하는 전달을 위해 주입된 합성 턴입니다. [`SDKTaskNotificationMessage`](#sdktasknotificationmessage)를 참조하세요. 애플리케이션이 [예약된 실행으로 선언](#declare-a-scheduled-run)하는 프롬프트도 이 종류를 전달합니다. 선택적 `subkind`는 알림을 발생시킨 것을 표시합니다. [작업 알림 서브종류](#task-notification-subkinds)를 참조하세요.                                            |
| `coordinator`       | [에이전트 팀](/docs/ko/agent-teams)의 팀 코디네이터의 메시지입니다.                                                                                                                                                                                                                                                                                       |
| `auto-continuation` | 명령 결과가 후속 프롬프트를 트리거하는 것과 같이 신선한 사용자 입력 없이 세션이 계속될 때 주입된 합성 턴입니다.                                                                                                                                                                                                                                                                  |
| `unclassified`      | 출처를 결정할 수 없는 주입된 턴입니다. Claude Code가 [`SDKUserMessage`](#sdkusermessage)를 `isSynthetic: true`로 받고 다른 `kind`로 분류할 수 없으면 메시지가 도착할 때 이 종류를 설정하고 턴을 모델에 인간 입력이 아닌 비사용자 소스로 프레임합니다. 애플리케이션은 이 값을 설정하지 않아야 합니다.                                                                                                                          |

<h3 id="task-notification-subkinds">
  작업 알림 서브종류
</h3>

Claude Code가 작업 알림을 세션에 전달할 때, Anthropic 서버가 해당 알림이 어디에서 나왔는지 확인했으면 알림의 `origin`에 `subkind`를 설정합니다. 또한 애플리케이션이 [메시지를 예약된 실행으로 선언](#declare-a-scheduled-run)할 때 `subkind`를 설정하며, TypeScript Agent SDK v0.3.280 이상이 필요합니다. `subkind`는 Claude Code v2.1.213 이상이 필요하며 두 가지 값 중 하나를 취합니다:

* `scheduled-trigger`: 알림은 [루틴](/docs/ko/routines)의 저장된 프롬프트로, 루틴의 트리거 중 하나가 발생했기 때문에 전달됩니다: 일정, [API 트리거](/docs/ko/routines#add-an-api-trigger), [GitHub 트리거](/docs/ko/routines#add-a-github-trigger), 또는 **지금 실행**. 애플리케이션이 [예약된 실행으로 선언](#declare-a-scheduled-run)하는 프롬프트도 이 값을 전달합니다. Claude Code는 이를 세션의 할당된 작업으로 모델에 프레임하며, [다른 작업 알림이 전달하는 알림](#sdktasknotificationmessage)과 다른 알림입니다.
*

`peer-send-message`: 알림은 [교차 세션 `SendMessage` 도구](/docs/ko/cross-session-messaging)가 아니라 [클라우드 세션](/docs/ko/claude-code-on-the-web)이 서로 메시지를 보내는 데 사용하는 서버 측 `send_message` 도구로 다른 세션이 보낸 메시지이며, Anthropic 서버가 두 세션이 동일한 비공개 세션 그룹에 속한다고 확인했습니다. Claude Code v2.1.224 이상이 필요합니다. 서버가 그런 식으로 확인하지 않은 `send_message` 전달은 `subkind`를 얻지 못합니다.

다른 모든 작업 알림에는 `subkind`가 없습니다. 여기에는 [PR 활동](/docs/ko/claude-code-on-the-web#how-claude-responds-to-pr-activity)이 세션에 전달되고 완료된 작업과 같은 백그라운드 이벤트가 포함됩니다. [교차 세션 `SendMessage` 도구](/docs/ko/cross-session-messaging)의 메시지는 작업 알림이 아닙니다: 동일한 머신의 세션에서 오든 다른 머신의 Anthropic 서버를 통해 오든 Claude Code는 `kind: "peer"`를 제공하고 [피어 원점 필드](#peer-origin-fields)를 제공합니다.

`fireReason`은 `scheduled-trigger` 알림이 발생한 이유를 `scheduled`, `manual`, `retry`, `catch_up`, 또는 `api`와 같은 짧은 소문자 토큰으로 말합니다. Anthropic 서버는 [루틴](/docs/ko/routines)의 전달에 설정하고 애플리케이션은 예약된 실행을 선언할 때 설정합니다. 어느 쪽도 보내지 않으면 없습니다. TypeScript Agent SDK v0.3.280 이상이 필요합니다.

<h4 id="declare-a-scheduled-run">
  예약된 실행 선언
</h4>

애플리케이션이 자신의 일정에 따라 프롬프트를 실행하면 각 실행을 선언하여 Claude Code가 턴을 라이브 사용자 입력이 아니라 예약된 작업으로 모델에 프레임하도록 합니다. [`env`](#options)에서 `CLAUDE_CODE_HOST_SCHEDULED_RUN`을 `1`로 설정하여 세션을 시작한 다음 실행의 [`SDKUserMessage`](#sdkusermessage)를 `origin: { kind: "task-notification", subkind: "scheduled-trigger", fireReason: "scheduled" }`로 보내고 `isSynthetic` 없이 보냅니다. Claude Code는 해당 변수 없이 시작된 프로세스에서 선언을 무시합니다. 또한 환경이 [`CLAUDECODE`](/docs/ko/env-vars) 또는 `CLAUDE_CODE_CHILD_SESSION`을 전달하는 프로세스에서도 무시합니다. Claude Code는 값이 1\~32개의 소문자 또는 밑줄일 때만 `fireReason`을 유지합니다. TypeScript Agent SDK v0.3.280 이상이 필요합니다.

<h3 id="peer-origin-fields">
  피어 원점 필드
</h3>

`peer` 원점은 메시지를 보낸 에이전트를 식별합니다: `SendMessage`로 `main`에 보내는 프로세스 내 [팀원](/docs/ko/agent-teams) 또는 [교차 세션 피어](/docs/ko/cross-session-messaging), 다른 Claude Code 세션입니다. 교차 세션 피어는 macOS 및 Linux에서 Claude Code v2.1.224 이상이 필요합니다. [교차 세션 메시징 가용성](/docs/ko/cross-session-messaging#availability)에서 네이티브 Windows 요구 사항을 참조하세요. 교차 세션 피어는 동일한 머신에서 실행되거나 [다른 머신](/docs/ko/cross-session-messaging#message-sessions-on-other-machines)에서 또는 [클라우드](/docs/ko/claude-code-on-the-web)에서 Remote Control을 통해 메시지가 도착할 때 실행될 수 있습니다. 두 종류의 발신자는 필드를 다르게 채웁니다:

* `from`: 팀원의 이름 또는 교차 세션 피어의 발신자 주소입니다. [일방향 교차 머신 메시지](/docs/ko/cross-session-messaging#message-sessions-on-other-machines)의 경우 발신자는 회신 주소가 없고 `from`은 `"unknown"`입니다. 값은 발신자가 작성한 것입니다. `verifiedPeerPid`는 확인된 신원입니다.
*

`fromMode`: 발신 세션의 권한 클래스로, `bypass` 또는 `prompting`이며, [데스크톱 앱](/docs/ko/desktop#work-across-sessions)과 같이 세션 간에 피어 메시지를 중계하는 호스트에서 선언합니다. Claude Code는 [인바운드 제어](/docs/ko/cross-session-messaging#control-inbound-messages)를 적용할 때 수신 세션에서 읽습니다. Agent SDK v0.3.234 이상이 필요합니다.

* `senderTaskId`: 팀원의 작업 ID입니다. 교차 세션 피어의 경우 없습니다.
*

`name`: 발신자의 표시 이름으로, Claude Code에서 정규화됩니다: Unicode 제어, 형식, 대리, 줄 또는 단락 구분 기호 코드 포인트를 제거한 다음 결과를 자르고 64개 코드 포인트로 제한하고 줄임표를 추가합니다. Claude Code v2.1.205 이상이 필요합니다.

*

`body`: 피어 봉투가 제거된 디코딩된 메시지 본문로, 모델이 보는 것과 바이트 정확합니다. 팀원 메시지의 경우 항상 있습니다. 교차 세션 피어의 경우 턴이 정확히 Claude Code에서 형성한 하나의 피어 봉투일 때만 있습니다. 메시지 텍스트를 다시 구문 분석하는 대신 `name`과 `body`를 렌더링합니다. Claude Code v2.1.205 이상이 필요합니다.

*

`fromSession`: 발신자의 호스트 열기 가능 세션 ID로, 발신자의 호스트에서 설정하여 UI가 발신 세션으로 다시 링크할 수 있습니다. `from`과 마찬가지로 발신자가 주장한 것입니다: 네비게이션 대상으로만 사용하고 발신자의 신원 증명으로 취급하지 마세요. Claude Code v2.1.216 이상이 필요합니다.

*

`verifiedPeerPid`: 이 세션의 교차 세션 메시징 소켓에 연결된 프로세스의 프로세스 ID로, 커널에서 확인하고 페이로드가 아닌 연결 자체에서 읽습니다. 발신자를 식별하려면 `from`이 아니라 이를 사용합니다: `from`은 동일한 사용자 프로세스에서 위조 가능합니다. 필드는 Claude Code가 확인할 수 없을 때 없으며, 예를 들어 Windows 또는 비소켓 수신에서 없으므로 없는 값은 발신자가 확인되지 않음을 의미합니다. 중계된 트래픽의 경우 메시지의 작성자가 아니라 중계를 식별하고 프로세스 ID는 재활용 가능하므로 인증 토큰이 아니라 출처로 취급합니다. Claude Code v2.1.216 이상이 필요합니다.

<h2 id="hook-types">
  훅 타입
</h2>

훅 사용에 대한 포괄적인 가이드, 예제 및 일반적인 패턴은 [훅 가이드](/docs/ko/agent-sdk/hooks)를 참조하세요.

<h3 id="hookevent">
  `HookEvent`
</h3>

사용 가능한 훅 이벤트입니다.

```typescript theme={null}
type HookEvent =
  | "PreToolUse"
  | "PostToolUse"
  | "PostToolUseFailure"
  | "PostToolBatch"
  | "Notification"
  | "UserPromptSubmit"
  | "UserPromptExpansion"
  | "SessionStart"
  | "SessionEnd"
  | "Stop"
  | "StopFailure"
  | "SubagentStart"
  | "SubagentStop"
  | "PreCompact"
  | "PostCompact"
  | "PreModelSwitch"
  | "PostModelSwitch"
  | "PermissionRequest"
  | "PermissionDenied"
  | "Setup"
  | "TeammateIdle"
  | "TaskCreated"
  | "TaskCompleted"
  | "Elicitation"
  | "ElicitationResult"
  | "ConfigChange"
  | "DirectoryAdded"
  | "WorktreeCreate"
  | "WorktreeRemove"
  | "InstructionsLoaded"
  | "CwdChanged"
  | "FileChanged"
  | "MessageDisplay";
```

<h3 id="hookcallback">
  `HookCallback`
</h3>

훅 콜백 함수 타입입니다.

```typescript theme={null}
type HookCallback = (
  input: HookInput, // 모든 훅 입력 타입의 합집합
  toolUseID: string | undefined,
  options: { signal: AbortSignal }
) => Promise<HookJSONOutput>;
```

<h3 id="hookcallbackmatcher">
  `HookCallbackMatcher`
</h3>

선택적 매처를 포함한 훅 구성입니다.

```typescript theme={null}
interface HookCallbackMatcher {
  matcher?: string;
  hooks: HookCallback[];
  timeout?: number; // 이 매처의 모든 훅에 대한 타임아웃 (초)
}
```

<h3 id="hookinput">
  `HookInput`
</h3>

모든 훅 입력 타입의 합집합 타입입니다.

```typescript theme={null}
type HookInput =
  | PreToolUseHookInput
  | PostToolUseHookInput
  | PostToolUseFailureHookInput
  | PostToolBatchHookInput
  | PermissionDeniedHookInput
  | NotificationHookInput
  | UserPromptSubmitHookInput
  | UserPromptExpansionHookInput
  | SessionStartHookInput
  | SessionEndHookInput
  | StopHookInput
  | StopFailureHookInput
  | SubagentStartHookInput
  | SubagentStopHookInput
  | PreCompactHookInput
  | PostCompactHookInput
  | PreModelSwitchHookInput
  | PostModelSwitchHookInput
  | PermissionRequestHookInput
  | SetupHookInput
  | TeammateIdleHookInput
  | TaskCreatedHookInput
  | TaskCompletedHookInput
  | ElicitationHookInput
  | ElicitationResultHookInput
  | ConfigChangeHookInput
  | InstructionsLoadedHookInput
  | DirectoryAddedHookInput
  | WorktreeCreateHookInput
  | WorktreeRemoveHookInput
  | CwdChangedHookInput
  | FileChangedHookInput
  | MessageDisplayHookInput;
```

<h3 id="basehookinput">
  `BaseHookInput`
</h3>

모든 훅 입력 타입이 확장하는 기본 인터페이스입니다.

```typescript theme={null}
type BaseHookInput = {
  session_id: string;
  transcript_path: string;
  cwd: string;
  prompt_id?: string;
  permission_mode?: string;
  effort?: { level: string };
  agent_id?: string;
  agent_type?: string;
};
```

`prompt_id` 필드는 현재 처리 중인 사용자 프롬프트를 식별하는 UUID입니다. [OpenTelemetry 이벤트의 `prompt.id` 속성](/docs/ko/monitoring-usage#event-correlation-attributes)과 일치하며 첫 번째 사용자 입력까지는 없습니다. Claude Code v2.1.196 이상이 필요합니다.

<h4 id="pretoolusehookinput">
  `PreToolUseHookInput`
</h4>

```typescript theme={null}
type PreToolUseHookInput = BaseHookInput & {
  hook_event_name: "PreToolUse";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  mcp_server?: McpServerProvenance;
};
```

`mcp_server`는 도구가 MCP 서버에서 올 때 나타납니다. [`McpServerProvenance`](#mcpserverprovenance)를 참조하세요. `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` 및 `PermissionDenied` 입력은 동일한 필드를 전달합니다. 이 필드는 Agent SDK v0.3.274 이상이 필요합니다.

<h4 id="posttoolusehookinput">
  `PostToolUseHookInput`
</h4>

```typescript theme={null}
type PostToolUseHookInput = BaseHookInput & {
  hook_event_name: "PostToolUse";
  tool_name: string;
  tool_input: unknown;
  tool_response: unknown;
  tool_use_id: string;
  duration_ms?: number;
  mcp_server?: McpServerProvenance;
};
```

<h4 id="posttoolusefailurehookinput">
  `PostToolUseFailureHookInput`
</h4>

```typescript theme={null}
type PostToolUseFailureHookInput = BaseHookInput & {
  hook_event_name: "PostToolUseFailure";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  error: string;
  is_interrupt?: boolean;
  duration_ms?: number;
  mcp_server?: McpServerProvenance;
};
```

<h4 id="posttoolbatchhookinput">
  `PostToolBatchHookInput`
</h4>

배치의 모든 도구 호출이 해결된 후, 다음 모델 요청 전에 한 번 실행됩니다. `tool_response`는 모델이 보는 직렬화된 `tool_result` 콘텐츠를 전달합니다. 형태는 `PostToolUseHookInput`의 구조화된 `Output` 객체와 다릅니다.

```typescript theme={null}
type PostToolBatchHookInput = BaseHookInput & {
  hook_event_name: "PostToolBatch";
  tool_calls: PostToolBatchToolCall[];
};

type PostToolBatchToolCall = {
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  tool_response?: unknown;
};
```

<h4 id="permissiondeniedhookinput">
  `PermissionDeniedHookInput`
</h4>

```typescript theme={null}
type PermissionDeniedHookInput = BaseHookInput & {
  hook_event_name: "PermissionDenied";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  reason: string;
  mcp_server?: McpServerProvenance;
};
```

<h4 id="notificationhookinput">
  `NotificationHookInput`
</h4>

```typescript theme={null}
type NotificationHookInput = BaseHookInput & {
  hook_event_name: "Notification";
  message: string;
  title?: string;
  notification_type: string;
};
```

<h4 id="userpromptsubmithookinput">
  `UserPromptSubmitHookInput`
</h4>

```typescript theme={null}
type UserPromptSubmitHookInput = BaseHookInput & {
  hook_event_name: "UserPromptSubmit";
  prompt: string;
  session_title?: string;
};
```

<h4 id="userpromptexpansionhookinput">
  `UserPromptExpansionHookInput`
</h4>

```typescript theme={null}
type UserPromptExpansionHookInput = BaseHookInput & {
  hook_event_name: "UserPromptExpansion";
  expansion_type: "slash_command" | "mcp_prompt";
  command_name: string;
  command_args: string;
  command_source?: string;
  prompt: string;
};
```

<h4 id="sessionstarthookinput">
  `SessionStartHookInput`
</h4>

```typescript theme={null}
type SessionStartHookInput = BaseHookInput & {
  hook_event_name: "SessionStart";
  source: "startup" | "resume" | "clear" | "compact" | "fork";
  agent_type?: string;
  model?: string;
  session_title?: string;
};
```

<h4 id="sessionendhookinput">
  `SessionEndHookInput`
</h4>

```typescript theme={null}
type SessionEndHookInput = BaseHookInput & {
  hook_event_name: "SessionEnd";
  reason: ExitReason; // EXIT_REASONS 배열의 문자열
};
```

<h4 id="stophookinput">
  `StopHookInput`
</h4>

```typescript theme={null}
type StopHookInput = BaseHookInput & {
  hook_event_name: "Stop";
  stop_hook_active: boolean;
  last_assistant_message?: string;
  background_tasks?: BackgroundTaskSummary[];
  session_crons?: SessionCronSummary[];
};
```

<h4 id="stopfailurehookinput">
  `StopFailureHookInput`
</h4>

```typescript theme={null}
type StopFailureHookInput = BaseHookInput & {
  hook_event_name: "StopFailure";
  error: SDKAssistantMessageError;
  error_details?: string;
  last_assistant_message?: string;
};
```

<h4 id="subagentstarthookinput">
  `SubagentStartHookInput`
</h4>

```typescript theme={null}
type SubagentStartHookInput = BaseHookInput & {
  hook_event_name: "SubagentStart";
  agent_id: string;
  agent_type: string;
};
```

<h4 id="subagentstophookinput">
  `SubagentStopHookInput`
</h4>

```typescript theme={null}
type SubagentStopHookInput = BaseHookInput & {
  hook_event_name: "SubagentStop";
  stop_hook_active: boolean;
  agent_id: string;
  agent_transcript_path: string;
  agent_type: string;
  last_assistant_message?: string;
  background_tasks?: BackgroundTaskSummary[];
  session_crons?: SessionCronSummary[];
};

type BackgroundTaskSummary = {
  id: string;
  type: string;
  status: string;
  description: string;
  command?: string;
  agent_type?: string;
  server?: string;
  tool?: string;
  name?: string;
};

type SessionCronSummary = {
  id: string;
  schedule: string;
  recurring: boolean;
  prompt: string;
};
```

<h4 id="precompacthookinput">
  `PreCompactHookInput`
</h4>

```typescript theme={null}
type PreCompactHookInput = BaseHookInput & {
  hook_event_name: "PreCompact";
  trigger: "manual" | "auto";
  custom_instructions: string | null;
};
```

<h4 id="postcompacthookinput">
  `PostCompactHookInput`
</h4>

```typescript theme={null}
type PostCompactHookInput = BaseHookInput & {
  hook_event_name: "PostCompact";
  trigger: "manual" | "auto";
  compact_summary: string;
};
```

<h4 id="premodelswitchhookinput">
  `PreModelSwitchHookInput`
</h4>

요청된 모델 전환이 적용되기 전에 실행됩니다. `context_tokens` 및 그 이후의 필드는 새 모델로 대화를 다시 전송하는 비용을 추정합니다. 전체 필드 설명 및 차단 의미론은 [PreModelSwitch](/docs/ko/hooks#premodelswitch)를 참조하세요.

```typescript theme={null}
type PreModelSwitchHookInput = BaseHookInput & {
  hook_event_name: "PreModelSwitch";
  from_model: string;
  to_model: string;
  requested_model: string | null;
  source: "command" | "picker" | "sdk";
  context_tokens: number;
  prompt_cache_warm: boolean;
  cache_ttl: "5m" | "1h";
  estimated_cache_write_usd: number;
  pricing: "configured" | "catalog" | "default";
};
```

<h4 id="postmodelswitchhookinput">
  `PostModelSwitchHookInput`
</h4>

세션의 모델이 변경된 후 실행됩니다. `PreModelSwitchHookInput`과 동일한 필드를 전달하며, 두 개의 추가 `source` 값이 있습니다. [PostModelSwitch](/docs/ko/hooks#postmodelswitch)를 참조하세요.

```typescript theme={null}
type PostModelSwitchHookInput = BaseHookInput & {
  hook_event_name: "PostModelSwitch";
  from_model: string;
  to_model: string;
  requested_model: string | null;
  source: "command" | "picker" | "sdk" | "auto" | "resume";
  context_tokens: number;
  prompt_cache_warm: boolean;
  cache_ttl: "5m" | "1h";
  estimated_cache_write_usd: number;
  pricing: "configured" | "catalog" | "default";
};
```

<h4 id="permissionrequesthookinput">
  `PermissionRequestHookInput`
</h4>

```typescript theme={null}
type PermissionRequestHookInput = BaseHookInput & {
  hook_event_name: "PermissionRequest";
  tool_name: string;
  tool_input: unknown;
  permission_suggestions?: PermissionUpdate[];
  mcp_server?: McpServerProvenance;
};
```

<h4 id="setuphookinput">
  `SetupHookInput`
</h4>

```typescript theme={null}
type SetupHookInput = BaseHookInput & {
  hook_event_name: "Setup";
  trigger: "init" | "maintenance";
};
```

<h4 id="teammateidlehookinput">
  `TeammateIdleHookInput`
</h4>

```typescript theme={null}
type TeammateIdleHookInput = BaseHookInput & {
  hook_event_name: "TeammateIdle";
  teammate_name: string;
  /** @deprecated v2.1.178 이후로 사용되지 않음. 세션에서 파생된 팀 이름을 전달합니다. 제거될 예정입니다. */
  team_name: string;
};
```

<h4 id="taskcreatedhookinput">
  `TaskCreatedHookInput`
</h4>

```typescript theme={null}
type TaskCreatedHookInput = BaseHookInput & {
  hook_event_name: "TaskCreated";
  task_id: string;
  task_subject: string;
  task_description?: string;
  teammate_name?: string;
  /** @deprecated v2.1.178 이후로 사용되지 않음. 세션에서 파생된 팀 이름을 전달합니다. 제거될 예정입니다. */
  team_name?: string;
};
```

<h4 id="taskcompletedhookinput">
  `TaskCompletedHookInput`
</h4>

```typescript theme={null}
type TaskCompletedHookInput = BaseHookInput & {
  hook_event_name: "TaskCompleted";
  task_id: string;
  task_subject: string;
  task_description?: string;
  teammate_name?: string;
  /** @deprecated v2.1.178 이후로 사용되지 않음. 세션에서 파생된 팀 이름을 전달합니다. 제거될 예정입니다. */
  team_name?: string;
};
```

<h4 id="elicitationhookinput">
  `ElicitationHookInput`
</h4>

```typescript theme={null}
type ElicitationHookInput = BaseHookInput & {
  hook_event_name: "Elicitation";
  mcp_server_name: string;
  message: string;
  mode?: "form" | "url";
  url?: string;
  elicitation_id?: string;
  requested_schema?: Record<string, unknown>;
};
```

<h4 id="elicitationresulthookinput">
  `ElicitationResultHookInput`
</h4>

```typescript theme={null}
type ElicitationResultHookInput = BaseHookInput & {
  hook_event_name: "ElicitationResult";
  mcp_server_name: string;
  elicitation_id?: string;
  mode?: "form" | "url";
  action: "accept" | "decline" | "cancel";
  content?: Record<string, unknown>;
};
```

<h4 id="configchangehookinput">
  `ConfigChangeHookInput`
</h4>

```typescript theme={null}
type ConfigChangeHookInput = BaseHookInput & {
  hook_event_name: "ConfigChange";
  source:
    | "user_settings"
    | "project_settings"
    | "local_settings"
    | "policy_settings"
    | "skills";
  file_path?: string;
};
```

<h4 id="instructionsloadedhookinput">
  `InstructionsLoadedHookInput`
</h4>

```typescript theme={null}
type InstructionsLoadedHookInput = BaseHookInput & {
  hook_event_name: "InstructionsLoaded";
  file_path: string;
  memory_type: "User" | "Project" | "Local" | "Managed";
  load_reason:
    | "session_start"
    | "nested_traversal"
    | "path_glob_match"
    | "include"
    | "compact";
  globs?: string[];
  trigger_file_path?: string;
  parent_file_path?: string;
};
```

<h4 id="directoryaddedhookinput">
  `DirectoryAddedHookInput`
</h4>

```typescript theme={null}
type DirectoryAddedHookInput = BaseHookInput & {
  hook_event_name: "DirectoryAdded";
  directory: string;
  source: "slash_command" | "register_repo_root";
};
```

`directory`는 추가된 디렉토리의 절대 경로입니다. `source`는 `/add-dir`이 추가했을 때 `"slash_command"`이고 SDK 제어 요청이 했을 때 `"register_repo_root"`입니다.

<h4 id="worktreecreatehookinput">
  `WorktreeCreateHookInput`
</h4>

```typescript theme={null}
type WorktreeCreateHookInput = BaseHookInput & {
  hook_event_name: "WorktreeCreate";
  name: string;
};
```

<h4 id="worktreeremovehookinput">
  `WorktreeRemoveHookInput`
</h4>

```typescript theme={null}
type WorktreeRemoveHookInput = BaseHookInput & {
  hook_event_name: "WorktreeRemove";
  worktree_path: string;
};
```

<h4 id="cwdchangedhookinput">
  `CwdChangedHookInput`
</h4>

```typescript theme={null}
type CwdChangedHookInput = BaseHookInput & {
  hook_event_name: "CwdChanged";
  old_cwd: string;
  new_cwd: string;
};
```

<h4 id="filechangedhookinput">
  `FileChangedHookInput`
</h4>

```typescript theme={null}
type FileChangedHookInput = BaseHookInput & {
  hook_event_name: "FileChanged";
  file_path: string;
  event: "change" | "add" | "unlink";
};
```

<h4 id="messagedisplayhookinput">
  `MessageDisplayHookInput`
</h4>

```typescript theme={null}
type MessageDisplayHookInput = BaseHookInput & {
  hook_event_name: "MessageDisplay";
  turn_id: string;
  message_id: string;
  index: number;
  final: boolean;
  delta: string;
};
```

<h3 id="hookjsonoutput">
  `HookJSONOutput`
</h3>

훅 반환값입니다.

```typescript theme={null}
type HookJSONOutput = AsyncHookJSONOutput | SyncHookJSONOutput;
```

<h4 id="asynchookjsonoutput">
  `AsyncHookJSONOutput`
</h4>

```typescript theme={null}
type AsyncHookJSONOutput = {
  async: true;
  asyncTimeout?: number;
};
```

<h4 id="synchookjsonoutput">
  `SyncHookJSONOutput`
</h4>

```typescript theme={null}
type SyncHookJSONOutput = {
  continue?: boolean;
  suppressOutput?: boolean;
  stopReason?: string;
  decision?: "approve" | "block";
  systemMessage?: string;
  /**
   * Claude Code가 사용자를 대신하여 내보낼 터미널 이스케이프 시퀀스 (예: OSC 9 / OSC 777 데스크톱 알림)입니다.
   * 알림/제목 OSC (0, 1, 2, 9, 99, 777) 및 BEL만 허용됩니다. 다른 것을 포함하는 값은 전체적으로 무시됩니다.
   * 대화형 CLI만 내보냅니다. SDK는 이 필드를 무시합니다.
   */
  terminalSequence?: string;
  reason?: string;
  hookSpecificOutput?:
    | {
        hookEventName: "PreToolUse";
        permissionDecision?: "allow" | "deny" | "ask" | "defer";
        permissionDecisionReason?: string;
        updatedInput?: Record<string, unknown>;
        additionalContext?: string;
      }
    | {
        hookEventName: "UserPromptSubmit";
        additionalContext?: string;
        sessionTitle?: string;
        /** decision이 "block"일 때, 블록 메시지에서 원본 프롬프트를 생략합니다. */
        suppressOriginalPrompt?: boolean;
      }
    | {
        hookEventName: "UserPromptExpansion";
        additionalContext?: string;
      }
    | {
        hookEventName: "SessionStart";
        additionalContext?: string;
        initialUserMessage?: string;
        sessionTitle?: string;
        watchPaths?: string[];
        /**
         * SessionStart 훅이 완료된 후 스킬 및 명령 디렉토리를 다시 스캔하므로
         * 훅에 의해 설치된 스킬을 동일한 세션에서 사용할 수 있습니다.
         */
        reloadSkills?: boolean;
      }
    | {
        hookEventName: "Setup";
        additionalContext?: string;
      }
    | {
        hookEventName: "PreModelSwitch";
        /**
         * PreToolUse와 동일한 계약: "allow"는 진행, "deny"는 전환 취소,
         * "ask"는 사용자에게 확인을 요청합니다. 대화형 세션의 /model만
         * 해당 프롬프트를 표시합니다. 다른 모든 표면 (set_model 요청 포함)은
         * "ask"를 거부로 취급합니다.
         */
        permissionDecision?: "allow" | "deny" | "ask";
        permissionDecisionReason?: string;
      }
    | {
        hookEventName: "PostModelSwitch";
        /** 새 모델이 제공하는 다음 요청과 함께 모델에 도달합니다. */
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStart";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolUse";
        additionalContext?: string;
        /**
         * 자동 모드 권한 분류기를 위한 이 도구 호출 결과에 대한 짧은 참고입니다.
         * 2000자로 제한되며, 동일한 호출에 응답하는 모든 훅에서 공유됩니다.
         * 동기 훅 응답에서만 적용됩니다. 신뢰할 수 없는 도구 출력을 복사하지 마세요.
         */
        classifierContext?: string;
        updatedToolOutput?: unknown;
        /** @deprecated `updatedToolOutput`를 사용하세요. 모든 도구에 대해 작동합니다. */
        updatedMCPToolOutput?: unknown;
      }
    | {
        hookEventName: "PostToolUseFailure";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolBatch";
        additionalContext?: string;
      }
    | {
        hookEventName: "Stop";
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStop";
        additionalContext?: string;
      }
    | {
        hookEventName: "PermissionDenied";
        retry?: boolean;
      }
    | {
        hookEventName: "Notification";
        additionalContext?: string;
      }
    | {
        hookEventName: "PermissionRequest";
        decision:
          | {
              behavior: "allow";
              updatedInput?: Record<string, unknown>;
              updatedPermissions?: PermissionUpdate[];
            }
          | {
              behavior: "deny";
              message?: string;
              interrupt?: boolean;
            };
      }
    | {
        hookEventName: "Elicitation";
        action?: "accept" | "decline" | "cancel";
        content?: Record<string, unknown>;
      }
    | {
        hookEventName: "ElicitationResult";
        action?: "accept" | "decline" | "cancel";
        content?: Record<string, unknown>;
      }
    | {
        hookEventName: "CwdChanged";
        watchPaths?: string[];
      }
    | {
        hookEventName: "FileChanged";
        watchPaths?: string[];
      }
    | {
        hookEventName: "WorktreeCreate";
        worktreePath: string;
      }
    | {
        hookEventName: "MessageDisplay";
        /** 델타 대신 표시되는 텍스트입니다. 원본을 표시하려면 생략하거나 델타를 변경하지 않은 상태로 반환합니다. */
        displayContent?: string;
      };
};
```

<h2 id="tool-input-types">
  도구 입력 타입
</h2>

모든 기본 제공 Claude Code 도구의 입력 스키마 문서입니다. 이 타입은 `@anthropic-ai/claude-agent-sdk`에서 내보내지며 타입 안전 도구 상호작용에 사용할 수 있습니다.

<h3 id="toolinputschemas">
  `ToolInputSchemas`
</h3>

`@anthropic-ai/claude-agent-sdk`에서 내보낸 도구 입력 타입의 합집합입니다. 멤버는 다음을 포함합니다:

```typescript theme={null}
type ToolInputSchemas =
  | AgentInput
  | ArtifactInput
  | AskUserQuestionInput
  | BashInput
  | CronCreateInput
  | CronDeleteInput
  | CronListInput
  | EnterPlanModeInput
  | EnterWorktreeInput
  | ExitPlanModeInput
  | ExitWorktreeInput
  | FileEditInput
  | FileReadInput
  | FileWriteInput
  | GlobInput
  | GrepInput
  | ListMcpResourcesInput
  | McpInput
  | MonitorInput
  | NotebookEditInput
  | ProjectsInput
  | PushNotificationInput
  | ReadMcpResourceDirInput
  | ReadMcpResourceInput
  | RefreshMcpToolsInput
  | RemoteTriggerInput
  | ReportFindingsInput
  | ScheduleWakeupInput
  | ShowOnboardingRolePickerInput
  | TaskCreateInput
  | TaskGetInput
  | TaskListInput
  | TaskStopInput
  | TaskUpdateInput
  | TodoWriteInput
  | WebFetchInput
  | WebSearchInput
  | WorkflowInput;
```

<h3 id="agent">
  Agent
</h3>

**도구 이름:** `Agent`. 이전 이름인 `Task`는 여전히 별칭으로 수락되며, [`SDKSystemMessage`](#sdksystemmessage) 초기화 메시지의 `tools` 배열은 현재 이 도구를 하위 호환성을 위해 `Task`로 나열합니다.

<Note>
  `mode` 필드는 Claude Code v2.1.212 이상에서 더 이상 사용되지 않으며 무시됩니다. 서브에이전트는 부모 세션의 권한 모드 또는 해당 정의의 [`permissionMode`](#agentdefinition)에서 실행되며, [서브에이전트 상속 규칙](/docs/ko/agent-sdk/permissions#available-modes)이 어느 것인지 결정합니다.
</Note>

```typescript theme={null}
type AgentInput = {
  description: string;
  prompt: string;
  subagent_type?: string;
  model?: "sonnet" | "opus" | "haiku" | "fable";
  run_in_background?: boolean;
  name?: string;
  team_name?: string; // 더 이상 사용되지 않음; 무시됨
  mode?: "acceptEdits" | "auto" | "bypassPermissions" | "default" | "dontAsk" | "plan"; // 더 이상 사용되지 않음; 무시됨. 서브에이전트 상속 규칙이 서브에이전트의 권한 모드를 결정합니다
  isolation?: "worktree" | "remote";
};
```

복잡한 다단계 작업을 자율적으로 처리할 새로운 에이전트를 시작합니다.

<h3 id="askuserquestion">
  AskUserQuestion
</h3>

**도구 이름:** `AskUserQuestion`

```typescript theme={null}
type AskUserQuestionInput = {
  questions: Array<{
    question: string;
    header: string;
    options: Array<{ label: string; description: string; preview?: string }>;
    multiSelect: boolean;
  }>;
  answers?: Record<string, string>;
  annotations?: Record<string, { preview?: string; notes?: string }>;
  metadata?: { source?: string };
};
```

실행 중에 사용자에게 명확히 하는 질문을 합니다. 사용 세부 정보는 [승인 및 사용자 입력 처리](/docs/ko/agent-sdk/user-input#handle-clarifying-questions)를 참조하세요.

<h3 id="bash">
  Bash
</h3>

**도구 이름:** `Bash`

```typescript theme={null}
type BashInput = {
  command: string;
  timeout?: number; // milliseconds, max 600000; higher values are clamped to the max
  description?: string;
  run_in_background?: boolean;
  dangerouslyDisableSandbox?: boolean;
};
```

선택적 타임아웃 및 백그라운드 실행을 포함한 Bash 명령을 실행합니다. 작업 디렉터리는 다중 턴 세션의 이후 턴에서 실행되는 명령을 포함하여 명령 간에 유지됩니다. 내보낸 환경 변수와 같은 셸 상태는 유지되지 않습니다. 어느 디렉터리 변경이 유지되는지에 대한 제한 사항은 [명령 간에 유지되는 것](/docs/ko/tools-reference#what-persists-between-commands)을 참조하세요.

<h3 id="monitor">
  Monitor
</h3>

**도구 이름:** `Monitor`

```typescript theme={null}
type MonitorInput = {
  description: string;
  timeout_ms: number;
  command?: string;
  ws?: {
    url: string;
    protocols?: string[];
  };
};
```

백그라운드 소스를 실행하고 각 이벤트를 Claude에 전달하므로 폴링 없이 반응할 수 있습니다: `command`는 스크립트를 실행하고 stdout 라인당 하나의 이벤트를 내보내며, `ws`는 WebSocket을 열고 텍스트 프레임당 하나의 이벤트를 내보냅니다. `command` 또는 `ws` 중 정확히 하나를 제공합니다. `ws` 소스는 Claude Code v2.1.195 이상이 필요합니다.

`timeout_ms`는 감시의 마감 시간(밀리초)입니다. 기본값은 300000이며 최대 3600000까지의 값을 허용합니다. 유효한 마감 시간은 최대 1800000(30분)이므로, 더 큰 허용 값은 그 값으로 단축됩니다. 마감 시간에 감시가 종료되고 Claude는 필요한 경우 새 감시를 시작할 수 있도록 하나의 알림을 받습니다.

내보낸 타입은 스키마가 기본값을 채우기 때문에 `timeout_ms`를 필수로 표시합니다. 이를 생략하는 호출은 유효성을 검사합니다.

Monitor가 명령을 실행할 때, Bash와 동일한 권한 규칙을 따릅니다. WebSocket 감시는 별도로 승인을 요청합니다. 동작 및 공급자 가용성은 [Monitor 도구 참조](/docs/ko/tools-reference#monitor-tool)를 참조하세요.

<h3 id="taskoutput">
  TaskOutput
</h3>

Claude Code v2.1.277에서 제거되었으며, 해당 `TaskOutputInput` 타입도 함께 제거되었습니다. 이전에는 실행 중이거나 완료된 백그라운드 작업에서 출력을 검색했습니다. Claude는 `Read`를 사용하여 백그라운드 작업의 출력 파일을 읽습니다.

`disallowedTools` 항목 또는 여전히 `TaskOutput`의 이름을 지정하는 거부 규칙은 경고 없이 무시됩니다.

<h3 id="edit">
  Edit
</h3>

**도구 이름:** `Edit`

```typescript theme={null}
type FileEditInput = {
  file_path: string;
  old_string: string;
  new_string: string;
  replace_all?: boolean;
};
```

파일에서 정확한 문자열 교체를 수행합니다.

<h3 id="read">
  Read
</h3>

**도구 이름:** `Read`

```typescript theme={null}
type FileReadInput = {
  file_path: string;
  offset?: number;
  limit?: number;
  pages?: string;
};
```

텍스트, 이미지, PDF 및 Jupyter 노트북을 포함한 로컬 파일 시스템에서 파일을 읽습니다. PDF 페이지 범위의 경우 `pages`를 사용합니다(예: `"1-5"`).

PDF의 경우 Claude는 Read 호출의 `tool_result` 콘텐츠 내에서 파일의 내용을 받습니다. `pdf` [출력](#tool-output-types)을 반환하는 읽기는 요약 `text` 블록 다음에 `document` 블록을 전달합니다. `parts` 출력을 반환하는 읽기는 요약 `text` 블록 다음에 추출된 페이지당 하나의 블록을 전달합니다: `image` 블록 또는 Claude Code가 이를 이미지로 렌더링할 수 없을 때 페이지의 이름을 지정하는 `text` 블록입니다. Agent SDK v0.3.242 이전에는 Claude Code가 도구 결과 후 별도의 `user` 메시지로 파일의 내용을 전달했습니다.

<h3 id="write">
  Write
</h3>

**도구 이름:** `Write`

```typescript theme={null}
type FileWriteInput = {
  file_path: string;
  content: string;
};
```

로컬 파일 시스템에 파일을 작성하고 존재하면 덮어씁니다.

<h3 id="glob">
  Glob
</h3>

**도구 이름:** `Glob`

```typescript theme={null}
type GlobInput = {
  pattern: string;
  path?: string;
};
```

모든 코드베이스 크기에서 작동하는 빠른 파일 패턴 매칭입니다.

<h3 id="grep">
  Grep
</h3>

**도구 이름:** `Grep`

```typescript theme={null}
type GrepInput = {
  pattern: string;
  path?: string;
  glob?: string;
  type?: string;
  output_mode?: "content" | "files_with_matches" | "count";
  "-i"?: boolean;
  "-o"?: boolean; // print only the matched parts of each line; requires output_mode: "content"
  "-n"?: boolean;
  "-B"?: number;
  "-A"?: number;
  "-C"?: number;
  context?: number;
  head_limit?: number;
  offset?: number;
  multiline?: boolean;
};
```

정규식 지원을 포함한 ripgrep 기반의 강력한 검색 도구입니다.

<h3 id="taskstop">
  TaskStop
</h3>

**도구 이름:** `TaskStop`

```typescript theme={null}
type TaskStopInput = {
  task_id?: string;
  shell_id?: string; // 더 이상 사용되지 않음: task_id 사용
};
```

ID로 실행 중인 백그라운드 작업 또는 셸을 중지합니다. v2.1.198부터 `task_id`는 에이전트 팀 팀원 또는 에이전트 ID 또는 이름으로 명명된 백그라운드 에이전트도 수락합니다.

<h3 id="notebookedit">
  NotebookEdit
</h3>

**도구 이름:** `NotebookEdit`

```typescript theme={null}
type NotebookEditInput = {
  notebook_path: string;
  cell_id?: string;
  new_source: string;
  cell_type?: "code" | "markdown";
  edit_mode?: "replace" | "insert" | "delete";
};
```

Jupyter 노트북 파일의 셀을 편집합니다.

<h3 id="webfetch">
  WebFetch
</h3>

**도구 이름:** `WebFetch`

```typescript theme={null}
type WebFetchInput = {
  url: string;
  prompt: string;
};
```

URL에서 콘텐츠를 가져오고 AI 모델로 처리합니다.

<h3 id="websearch">
  WebSearch
</h3>

**도구 이름:** `WebSearch`

```typescript theme={null}
type WebSearchInput = {
  query: string;
  allowed_domains?: string[];
  blocked_domains?: string[];
};
```

웹을 검색하고 형식화된 결과를 반환합니다.

<h3 id="workflow">
  Workflow
</h3>

**도구 이름:** `Workflow`

```typescript theme={null}
type WorkflowInput = {
  script?: string;
  name?: string;
  scriptPath?: string;
  args?: unknown; // any JSON value; the published typings render this as an object map
  resumeFromRunId?: string;
  title?: string; // ignored; the script's meta block sets the title
  description?: string; // ignored; the script's meta block sets the description
};
```

[동적 워크플로우](/docs/ko/workflows)를 실행합니다: 백그라운드에서 많은 서브에이전트를 조율하고 하나의 통합된 결과를 반환하는 스크립트입니다. `Workflow` 도구는 Agent SDK v0.3.149 이상에서 사용 가능합니다. `script`, `name` 또는 `scriptPath` 중 최소 하나가 필요합니다.

| 필드                | 타입        | 설명                                                                                                                                                                                                                    |
| ----------------- | --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `script`          | `string`  | 인라인 워크플로우 스크립트입니다. `export const meta = { name, description }`을 리터럴로 시작해야 하며, 그 뒤에 `agent()`, `parallel()`, `pipeline()` 및 `phase()`를 사용하는 스크립트 본문이 따릅니다. `meta`의 선택적 `phases` 배열은 진행 상황 보기에서 에이전트를 명명된 단계 아래에 그룹화합니다 |
| `name`            | `string`  | 기본 제공 워크플로우의 이름 또는 `.claude/workflows/`에 저장된 워크플로우입니다. 스크립트로 확인됩니다                                                                                                                                                    |
| `scriptPath`      | `string`  | 디스크의 워크플로우 스크립트 파일 경로입니다. `script` 및 `name`보다 우선합니다. Claude Code는 모든 호출의 스크립트를 유지하고 결과에서 경로를 반환하므로, 해당 파일을 편집하고 동일한 `scriptPath`로 다시 호출하여 반복할 수 있습니다                                                                  |
| `args`            | `unknown` | 스크립트에 전역 `args`로 노출되는 입력 값으로, 연구 질문이나 파일 경로 목록과 같은 매개변수화된 명명된 워크플로우용입니다. 배열과 객체를 JSON 인코딩된 문자열이 아닌 실제 JSON 값으로 전달합니다                                                                                                  |
| `resumeFromRunId` | `string`  | 재개할 이전 `Workflow` 호출의 실행 ID입니다. 입력이 변경되지 않은 완료된 `agent()` 호출은 일반적으로 캐시된 결과를 반환합니다. 나머지는 실시간으로 실행됩니다. [일시 중지 후 재개](/docs/ko/workflows#resume-after-a-pause)는 어느 완료된 호출이 다시 실행되는지 다룹니다. 동일한 세션만 해당됩니다                        |
| `title`           | `string`  | 무시됨; 스크립트의 `meta` 블록이 제목을 설정합니다                                                                                                                                                                                       |
| `description`     | `string`  | 무시됨; 스크립트의 `meta` 블록이 설명을 설정합니다                                                                                                                                                                                       |

<h3 id="todowrite">
  TodoWrite
</h3>

**도구 이름:** `TodoWrite`

```typescript theme={null}
type TodoWriteInput = {
  todos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
};
```

진행 상황을 추적하기 위한 구조화된 작업 목록을 만들고 관리합니다.

<Note>
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.

  [모델 가용성](/docs/ko/agent-sdk/todo-tracking#model-availability)을 참조하여 옵트인하세요.
</Note>

<h3 id="taskcreate">
  TaskCreate
</h3>

**도구 이름:** `TaskCreate`

```typescript theme={null}
type TaskCreateInput = {
  subject: string;
  description: string;
  activeForm?: string;
  metadata?: Record<string, unknown>;
};
```

단일 작업을 만들고 할당된 ID를 반환합니다.

<h3 id="taskupdate">
  TaskUpdate
</h3>

**도구 이름:** `TaskUpdate`

```typescript theme={null}
type TaskUpdateInput = {
  taskId: string;
  status?: "pending" | "in_progress" | "completed" | "deleted";
  subject?: string;
  description?: string;
  activeForm?: string;
  addBlocks?: string[];
  addBlockedBy?: string[];
  owner?: string;
  metadata?: Record<string, unknown>;
};
```

ID로 하나의 작업을 패치합니다. `status`를 `"deleted"`로 설정하여 제거합니다.

<h3 id="taskget">
  TaskGet
</h3>

**도구 이름:** `TaskGet`

```typescript theme={null}
type TaskGetInput = {
  taskId: string;
};
```

하나의 작업에 대한 전체 세부 정보를 반환하거나, ID를 찾을 수 없을 때 `null`을 반환합니다.

<h3 id="tasklist">
  TaskList
</h3>

**도구 이름:** `TaskList`

```typescript theme={null}
type TaskListInput = {};
```

현재 목록의 모든 작업의 스냅샷을 반환합니다.

<h3 id="exitplanmode">
  ExitPlanMode
</h3>

**도구 이름:** `ExitPlanMode`

```typescript theme={null}
type ExitPlanModeInput = {
  /** 더 이상 사용되지 않음: 더 이상 사용되지 않습니다. */
  allowedPrompts?: Array<{
    tool: "Bash";
    prompt: string;
  }>;
  [k: string]: unknown;
};
```

계획 모드를 종료합니다. `allowedPrompts` 필드는 더 이상 사용되지 않으며 무시됩니다. Claude Code는 기존 호출자 및 트랜스크립트가 유효성을 검사하도록 여전히 수락합니다. v2.1.205 이전에는 계획을 구현하기 위한 프롬프트 기반 Bash 권한을 요청했습니다.

<h3 id="listmcpresources">
  ListMcpResources
</h3>

**도구 이름:** `ListMcpResourcesTool`

```typescript theme={null}
type ListMcpResourcesInput = {
  server?: string;
};
```

연결된 서버에서 사용 가능한 MCP 리소스를 나열합니다.

<h3 id="readmcpresource">
  ReadMcpResource
</h3>

**도구 이름:** `ReadMcpResourceTool`

```typescript theme={null}
type ReadMcpResourceInput = {
  server: string;
  uri: string;
};
```

서버에서 특정 MCP 리소스를 읽습니다.

<h3 id="enterworktree">
  EnterWorktree
</h3>

**도구 이름:** `EnterWorktree`

```typescript theme={null}
type EnterWorktreeInput = {
  name?: string;
  path?: string;
};
```

격리된 작업을 위한 임시 git worktree를 만들고 입력합니다. 새 worktree를 만드는 대신 기존 worktree로 전환하려면 `path`를 전달합니다. 첫 번째 입력 시 대상은 현재 저장소의 등록된 worktree이거나, 다중 저장소 작업 공간에서 그 안에 중첩된 저장소여야 합니다. worktree 세션 내에서는 세션의 저장소의 `.claude/worktrees/` 아래에 있어야 합니다. `name` 및 `path`는 상호 배타적입니다.

<h3 id="exitworktree">
  ExitWorktree
</h3>

**도구 이름:** `ExitWorktree`

```typescript theme={null}
type ExitWorktreeInput = {
  action: "keep" | "remove";
  discard_changes?: boolean;
};
```

현재 git worktree를 종료하고 원래 작업 디렉터리로 돌아갑니다. `keep` 작업은 worktree와 분기를 디스크에 남기고, `remove`는 둘 다 삭제합니다. `discard_changes`는 커밋되지 않은 파일이나 병합되지 않은 커밋이 있는 worktree를 제거할 때 `true`여야 합니다.

<h3 id="enterplanmode">
  EnterPlanMode
</h3>

**도구 이름:** `EnterPlanMode`

```typescript theme={null}
type EnterPlanModeInput = {};
```

계획 모드에 입력합니다. 여기서 Claude는 변경을 수행하기 전에 계획을 연구하고 제시합니다.

<h3 id="croncreate">
  CronCreate
</h3>

**도구 이름:** `CronCreate`

```typescript theme={null}
type CronCreateInput = {
  cron: string;
  prompt: string;
  recurring?: boolean;
  durable?: boolean;
};
```

로컬 시간의 5필드 cron 일정에서 실행할 프롬프트를 예약합니다. `recurring`을 `false`로 설정하여 다음 일치 시 한 번 실행합니다. 작업은 기본적으로 세션 범위입니다: 새 대화를 시작하면 작업이 지워지고, `--resume` 또는 `--continue`로 재개하면 만료되지 않은 작업이 복원됩니다. [예약된 작업](/docs/ko/scheduled-tasks)을 참조하세요.

`durable`을 `true`로 설정하면 `.claude/scheduled_tasks.json`에 지속성을 요청하여 작업이 재시작을 견딜 수 있습니다. 지속적인 예약은 모든 세션에서 사용 가능하지 않습니다: 사용 불가능할 때 Claude Code는 `durable: true`를 수락하지만 작업을 세션 전용으로 만듭니다. 출력의 `durable` 필드를 읽어 작업이 지속되었는지 확인하세요.

<h3 id="crondelete">
  CronDelete
</h3>

**도구 이름:** `CronDelete`

```typescript theme={null}
type CronDeleteInput = {
  id: string;
};
```

`CronCreate`에서 반환된 ID로 예약된 cron 작업을 삭제합니다.

<h3 id="cronlist">
  CronList
</h3>

**도구 이름:** `CronList`

```typescript theme={null}
type CronListInput = {};
```

예약된 cron 작업을 나열합니다: `.claude/scheduled_tasks.json`의 지속적인 작업 및 현재 세션의 세션 전용 작업입니다.

<h3 id="schedulewakeup">
  ScheduleWakeup
</h3>

**도구 이름:** `ScheduleWakeup`

```typescript theme={null}
type ScheduleWakeupInput = {
  delaySeconds?: number;
  reason?: string;
  prompt?: string;
  noop?: boolean;
  stop?: boolean;
};
```

지연 후 주어진 프롬프트를 실행하는 일회성 웨이크업을 예약합니다. 이 도구는 자체 속도 `/loop` 명령을 지원합니다. 런타임은 `delaySeconds`를 60\~3600초 사이로 고정합니다. `stop`이 true가 아닌 한 `delaySeconds`, `reason`, `prompt` 및 `noop` 필드가 필요합니다. `noop: true`는 아무것도 변경되지 않은 웨이크업을 보고합니다. `stop: true`를 설정하면 보류 중인 웨이크업을 취소하고 자체 속도 `/loop`를 종료합니다. `stop` 필드는 Claude Code v2.1.202 이상이 필요합니다. [도구 참조의 ScheduleWakeup 행](/docs/ko/tools-reference)을 참조하세요.

<h3 id="remotetrigger">
  RemoteTrigger
</h3>

**도구 이름:** `RemoteTrigger`

```typescript theme={null}
type RemoteTriggerInput = {
  action:
    | "list"
    | "get"
    | "create"
    | "update"
    | "run"
    | "create_webhook_trigger"
    | "list_runs"
    | "get_run_log";
  trigger_id?: string;
  session_id?: string;
  cursor?: string;
  body?: {
    [k: string]: unknown;
  };
};
```

클라우드에서 호스팅되는 예약된 및 트리거된 Claude Code 실행인 [루틴](/docs/ko/routines)을 관리합니다. 이 도구는 `/schedule` 명령을 지원합니다. `trigger_id`는 `get`, `update`, `run` 및 `list_runs` 작업에 필요합니다. `body`는 `create`, `update` 및 `create_webhook_trigger`에 필요하며 `run`에는 선택 사항입니다.

`create_webhook_trigger`는 [GitHub 이벤트](/docs/ko/routines#add-a-github-trigger)와 같은 기존 루틴에 이벤트 소스를 연결하여 이를 실행합니다. `body`는 소스, 이벤트 및 실행할 루틴의 이름을 지정합니다. Claude Code v2.1.225 이상이 필요합니다.

`list_runs`는 루틴의 최근 실행을 나열하고, `get_run_log`는 하나의 실행 로그를 읽습니다. `session_id`는 `list_runs` 결과에서 읽을 실행의 이름을 지정하고, `cursor`는 두 작업의 결과를 페이징합니다. 두 작업 모두 Claude Code v2.1.227 이상이 필요합니다.

이 도구는 세션이 루틴이 활성화된 계획으로 claude.ai 계정으로 인증되었을 때만 사용 가능하며, 조직의 정책이 [클라우드 세션](/docs/ko/claude-code-on-the-web)을 비활성화할 때는 없습니다. Claude Code v2.1.227 이상에서 도구는 소유자가 [조직의 루틴을 비활성화](/docs/ko/routines#routines-are-disabled-by-your-organizations-policy)했을 때도 없습니다. v2.1.227 이전에는 루틴 토글만 비활성화된 세션이 여전히 도구를 표시했고 서버가 호출을 거부했습니다.

<h3 id="pushnotification">
  PushNotification
</h3>

**도구 이름:** `PushNotification`

```typescript theme={null}
type PushNotificationInput = {
  message: string;
  status: "proactive";
};
```

사용자에게 사전 예방적 푸시 알림을 보냅니다. 모바일 운영 체제가 더 긴 텍스트를 자르기 때문에 `message`를 200자 이하로 유지하세요. [도구 참조의 PushNotification 행](/docs/ko/tools-reference)에서 공급자 가용성을 참조하세요. 푸시 전달은 Amazon Bedrock, Claude Platform on AWS, Google Cloud의 Agent Platform 또는 Microsoft Foundry에서 액세스할 수 없는 Anthropic 호스팅 인프라를 통해 실행됩니다.

<h3 id="repl">
  REPL
</h3>

v2.1.275에서 제거되었습니다. v2.1.274까지 실험적 `REPL` 도구는 [`env` 옵션](#options)에서 `CLAUDE_CODE_REPL=1`을 설정하여 켤 수 있었습니다.

<h3 id="reportfindings">
  ReportFindings
</h3>

**도구 이름:** `ReportFindings`

```typescript theme={null}
type ReportFindingsInput = {
  level?: "low" | "medium" | "high" | "xhigh" | "max";
  findings: Array<{
    file: string;
    line?: number;
    summary: string;
    failure_scenario: string;
    short_summary?: string;
    category?: string;
    verdict?: "CONFIRMED" | "PLAUSIBLE";
    outcome?: "fixed" | "skipped" | "no_change_needed";
  }>;
};
```

코드 검토 결과를 구조화된 목록으로 보고하여 Claude Code가 텍스트로 인쇄하는 대신 렌더링할 수 있습니다. `level`은 검토가 실행된 노력 수준입니다. 결과는 가장 심각한 것부터 정렬되며 호출당 최대 32개이고, 배열은 생존한 것이 없을 때 비어 있습니다. Claude Code v2.1.196 이상이 필요합니다.

각 결과는 다음 필드를 전달합니다:

* `file`: 결과가 있는 저장소 상대 경로입니다. 선택적 `line`은 이를 고정하는 1 인덱싱된 라인입니다.
* `summary`: 결함의 한 문장 설명입니다. `failure_scenario`는 잘못된 출력 또는 충돌로 이어지는 구체적인 입력 및 상태를 설명합니다.
* `short_summary`: 컴팩트 디스플레이를 위한 최대 60자의 선택적 압축 레이블입니다. Claude Code v2.1.212 이상이 필요합니다.
* `category`: `correctness` 또는 `test-coverage`와 같은 결과 타입의 선택적 짧은 kebab-case 슬러그입니다. Claude Code v2.1.199 이상이 필요합니다.
* `verdict`: 검증 패스가 실행되었을 때 설정됩니다. 인라인 전용 검토에는 없습니다.
* `outcome`: 수정을 적용한 후 다시 보고할 때만 설정됩니다.

<h3 id="artifact">
  Artifact
</h3>

**도구 이름:** `Artifact`

```typescript theme={null}
type ArtifactInput = {
  action?: "publish" | "list";
  file_path?: string;
  favicon?: string;
  icon?: string;
  limit?: number;
  scope?: "mine" | "shared" | "all";
  title?: string;
  description?: string;
  label?: string;
  url?: string;
  force?: boolean;
  capabilities?: Record<string, unknown>;
  contract?: "latest" | string;
};
```

로컬 `.html` 또는 `.md` 파일을 호스팅된 아티팩트 페이지로 게시하거나 사용자의 게시된 아티팩트를 나열합니다. `action`을 생략하거나 `"publish"`를 전달하여 `file_path`를 게시합니다. 이는 게시 작업에 필요합니다. 아래의 각 필드는 게시에 적용됩니다:

* `icon`: 아티팩트의 브라우저 탭 아이콘에 대한 하나의 짧은 일반 단어(예: `chart` 또는 `map`). Claude는 첫 게시에 포함하고 업데이트에서 생략하여 아티팩트의 저장된 아이콘을 유지합니다.
* `favicon`: 더 이상 사용되지 않으며 Claude는 생략합니다.
* `title`: HTML 파일에 `<title>` 태그가 없을 때 브라우저 탭 및 갤러리에서 게시된 페이지의 이름을 지정합니다.
* `url`: 새 페이지를 만드는 대신 기존 아티팩트를 제자리에서 업데이트하도록 대상을 지정합니다.

`force`는 다른 세션이 게시한 최신 버전을 버리는 최후의 수단 덮어쓰기입니다. 충돌 시 실패한 게시는 최신 콘텐츠를 반환합니다. Claude는 해당 콘텐츠에 변경 사항을 병합하거나 아티팩트를 다시 읽고 다시 게시합니다. 사용자가 명시적으로 해당 버전을 버리도록 요청할 때만 `force`를 전달하세요.

`"list"`를 전달하여 사용자의 게시된 아티팩트를 열거합니다. `limit` 및 `scope`만 이를 동반할 수 있습니다. `scope`는 기본값이 `"mine"`이며, 사용자가 소유한 아티팩트를 나열합니다. `"shared"`는 다른 사람이 사용자와 공유한 아티팩트를 나열하고, `"all"`은 둘 다 나열합니다.

* `capabilities`: 게시된 페이지가 사용하는 런타임 기능으로, 기능 이름으로 키 지정됩니다(예: [페이지가 호출할 수 있는 커넥터](/docs/ko/artifacts#pull-live-data-with-mcp-connectors)). 아티팩트 서비스는 선언을 검증하고 계정이 사용할 수 없거나 잘못된 구성을 제공하는 기능의 이름을 지정하는 게시를 거부합니다. `{}`를 전달하여 저장된 선언을 지우고, 재배포 시 필드를 생략하여 유지하세요. Agent SDK v0.3.235 이상이 필요합니다.
* `contract`: 게시된 페이지가 실행되는 런타임 버전입니다. 생략하여 아티팩트의 현재 버전을 유지하고, `"latest"`를 전달하여 업그레이드하거나, 특정 버전을 전달하여 고정하거나 롤백하세요. Agent SDK v0.3.235 이상이 필요합니다.

타입은 내보내지지만 도구는 Agent SDK 세션에서 기본적으로 꺼집니다. 게시는 또한 [아티팩트 가용성 표](/docs/ko/artifacts#availability)의 모든 조건이 필요하며, API 키로 인증된 세션은 이를 충족하지 않습니다.

<h3 id="projects">
  Projects
</h3>

**도구 이름:** `Projects`

```typescript theme={null}
type ProjectsInput = {
  method:
    | "project_info"
    | "project_read"
    | "project_search"
    | "project_write"
    | "project_delete";
  path?: string;
  content?: string;
  local_path?: string;
  present_to_user?: boolean;
  query?: string;
  n?: number;
};
```

세션에 연결된 claude.ai 프로젝트를 읽고 씁니다. `method`에서 디스패치합니다:

* `project_info`: 프로젝트 메타데이터 및 문서 목록을 반환합니다.
* `project_read`: `path`로 하나의 문서를 읽습니다.
* `project_search`: `query`로 프로젝트의 지식 기반을 쿼리합니다. `n`은 히트를 제한하고 기본값은 `5`입니다.
* `project_write`: `content`(인라인 텍스트 전달) 또는 `local_path`(작업 디렉터리 내의 파일 이름) 중 정확히 하나에서 `path`에 문서를 만들거나 바꿉니다. `present_to_user: true`는 작성된 문서를 사용자가 봐야 할 결과물로 표시합니다.
* `project_delete`: `path`로 문서를 삭제합니다.

<h3 id="readmcpresourcedir">
  ReadMcpResourceDir
</h3>

**도구 이름:** `ReadMcpResourceDirTool`

```typescript theme={null}
type ReadMcpResourceDirInput = {
  server: string;
  uri: string;
};
```

MCP 서버의 디렉터리 리소스의 직접 자식을 나열합니다. 디렉터리 나열을 지원한다고 선언한 서버에 대해서만 사용 가능합니다. 나열은 재귀적이지 않습니다. 디렉터리 나열은 모든 세션에서 활성화되지 않습니다: 비활성화되면 호출은 빈 `resources` 목록을 반환하고 `error` 필드는 디렉터리 나열이 활성화되지 않았음을 보고합니다.

<h3 id="refreshmcptools">
  RefreshMcpTools
</h3>

**도구 이름:** `RefreshMcpTools`

```typescript theme={null}
type RefreshMcpToolsInput = {
  server?: string; // refresh only this server; omit to refresh all connected servers
};
```

연결된 MCP 서버의 도구 목록을 다시 쿼리하고 변경 사항을 적용합니다. 타입은 내보내지지만 [`env` 옵션](#options)에서 `CLAUDE_CODE_ENABLE_REFRESH_MCP_TOOLS=1`을 설정할 때만 Claude Code가 도구를 등록하며, 최소 하나의 MCP 서버가 있는 세션에서만 등록합니다. Claude Code v2.1.211 이상이 필요합니다.

<h3 id="showonboardingrolepicker">
  ShowOnboardingRolePicker
</h3>

**도구 이름:** `ShowOnboardingRolePicker`

```typescript theme={null}
type ShowOnboardingRolePickerInput = {};
```

Cowork 온보딩 중에 클릭 가능한 역할 선택기 칩 행을 렌더링하여 사용자가 역할을 선택하고 일치하는 플러그인을 설치할 수 있습니다. 인수를 사용하지 않습니다. 역할 목록은 클라이언트에서 정의됩니다. 호출은 사용자가 응답할 때까지 차단됩니다.

<h3 id="mcpinput">
  McpInput
</h3>

**도구 이름:** `mcp__<server>__<tool>` 형식의 동적 MCP 도구 이름

```typescript theme={null}
type McpInput = {
  [k: string]: unknown;
};
```

MCP 도구 인수는 개방형 객체입니다: 각 서버는 자체 매개변수를 정의하므로 타입은 필드 이름이나 값에 제약을 두지 않습니다. 특정 도구가 수락하는 필드에 대해 서버의 자체 도구 스키마를 참조하세요.

<h2 id="tool-output-types">
  도구 출력 타입
</h2>

모든 기본 제공 Claude Code 도구의 출력 스키마 문서입니다. 이 타입은 `@anthropic-ai/claude-agent-sdk`에서 내보내지며 각 도구에서 반환된 실제 응답 데이터를 나타냅니다.

<h3 id="tooloutputschemas">
  `ToolOutputSchemas`
</h3>

`@anthropic-ai/claude-agent-sdk`에서 내보낸 도구 출력 타입의 합집합입니다. 멤버는 다음을 포함합니다:

```typescript theme={null}
type ToolOutputSchemas =
  | AgentOutput
  | ArtifactOutput
  | AskUserQuestionOutput
  | BashOutput
  | CronCreateOutput
  | CronDeleteOutput
  | CronListOutput
  | EnterPlanModeOutput
  | EnterWorktreeOutput
  | ExitPlanModeOutput
  | ExitWorktreeOutput
  | FileEditOutput
  | FileReadOutput
  | FileWriteOutput
  | GlobOutput
  | GrepOutput
  | ListMcpResourcesOutput
  | McpOutput
  | MonitorOutput
  | NotebookEditOutput
  | ProjectsOutput
  | PushNotificationOutput
  | ReadMcpResourceDirOutput
  | ReadMcpResourceOutput
  | RefreshMcpToolsOutput
  | RemoteTriggerOutput
  | ReportFindingsOutput
  | ScheduleWakeupOutput
  | ShowOnboardingRolePickerOutput
  | TaskCreateOutput
  | TaskGetOutput
  | TaskListOutput
  | TaskStopOutput
  | TaskUpdateOutput
  | TodoWriteOutput
  | WebFetchOutput
  | WebSearchOutput
  | WorkflowOutput;
```

<h3 id="agent-2">
  Agent
</h3>

**도구 이름:** `Agent`. 이전 이름 `Task`는 여전히 별칭으로 수락되며, [`SDKSystemMessage`](#sdksystemmessage) 초기화 메시지의 `tools` 배열은 현재 이 도구를 하위 호환성을 위해 `Task`로 나열합니다.

```typescript theme={null}
type AgentOutput =
  | {
      status: "completed";
      agentId: string;
      agentType?: string;
      content: Array<{ type: "text"; text: string; citations?: unknown[] | null }>;
      resolvedModel?: string;
      modelsUsed?: string[];
      totalToolUseCount: number;
      totalDurationMs: number;
      totalTokens: number;
      usage: {
        input_tokens: number;
        output_tokens: number;
        cache_creation_input_tokens: number | null;
        cache_read_input_tokens: number | null;
        server_tool_use: {
          web_search_requests: number;
          web_fetch_requests: number;
        } | null;
        service_tier: string | null;
        cache_creation: {
          ephemeral_1h_input_tokens: number;
          ephemeral_5m_input_tokens: number;
        } | null;
        inference_geo?: string | null;
        speed?: string | null;
        iterations?: unknown;
        output_tokens_details?: {
          thinking_tokens?: number | null;
        } | null;
      };
      toolStats?: {
        readCount: number;
        searchCount: number;
        bashCount: number;
        editFileCount: number;
        linesAdded: number;
        linesRemoved: number;
        otherToolCount: number;
        frameCount?: number;
      };
      prompt: string;
      worktreePath?: string;
      worktreeBranch?: string;
    }
  | {
      status: "async_launched";
      isAsync?: true;
      agentId: string;
      description: string;
      resolvedModel?: string;
      modelsUsed?: string[];
      prompt: string;
      outputFile: string;
      canReadOutputFile?: boolean;
    }
  | {
      status: "remote_launched";
      taskId: string;
      sessionUrl: string;
      description: string;
      prompt: string;
      outputFile: string;
    };
```

서브에이전트의 결과를 반환합니다. `status` 필드에서 구분됩니다: 완료된 작업의 경우 `"completed"`, 백그라운드 작업의 경우 `"async_launched"`, Claude Code가 클라우드 세션으로 전달한 작업의 경우 `"remote_launched"`이며, 여기서 `sessionUrl`은 해당 세션으로 연결되고 `taskId`는 이를 식별합니다.

`completed` 변형에서 `resolvedModel`은 서브에이전트가 시작한 모델의 이름을 지정하며, 이는 [`availableModels`](/docs/ko/model-config#restrict-model-selection) 또는 다른 재정의가 적용될 때 요청된 `model` 입력과 다를 수 있습니다. 이 필드는 Claude Code v2.1.174 이상이 필요합니다. `async_launched`에서 작업이 백그라운드로 이동했을 때 사용 중인 모델의 이름을 지정합니다.

`modelsUsed`는 서브에이전트가 사용한 모델을 순서대로 나열합니다. 이 필드는 중간 실행 스왑이 발생했을 때만 존재하며, 실행이 다시 스왑되면 모델이 다시 나타납니다. `async_launched`에서 목록은 백그라운드 처리 전에 사용된 모델을 포함합니다. `modelsUsed`와 `resolvedModel`의 백그라운드 처리 동작 모두 Claude Code v2.1.212 이상이 필요합니다.

Claude Code가 [서브에이전트의 격리된 worktree를 유지](/docs/ko/worktrees#isolate-subagents-with-worktrees)한 경우, `completed` 결과의 `worktreePath`는 이를 찾을 수 있는 위치입니다. `worktreeBranch`는 해당 분기이며, Claude Code가 git으로 worktree를 생성했을 때 존재합니다.

Claude Code는 전체 실행이 아닌 서브에이전트의 최종 API 요청에서 `usage`와 `totalTokens`를 채우므로, `usage.service_tier`는 해당 요청에 대해 API가 보고한 서비스 계층 문자열입니다. 존재할 때, `usage.output_tokens_details.thinking_tokens`는 해당 요청의 출력 토큰 중 생각 토큰인 토큰의 수입니다. `output_tokens_details` 필드는 TypeScript SDK v0.3.228 이상이 필요하며, 이는 Claude Code v2.1.228을 번들로 제공합니다.

`usage.output_tokens_details`는 의미에서 [`Usage.output_tokens_details`](#usage)와 일치하며, 해당 최종 요청으로 범위가 지정되지만, 이 모든 수준이 선택 사항입니다. 예를 들어 `usage.output_tokens_details?.thinking_tokens ?? 0`처럼 객체와 필드 모두를 보호하고, 직접 읽지 마십시오.

v2.1.207 이전에는 게시된 타입이 더 좁았습니다. `worktreePath`, `worktreeBranch`, `citations`, `toolStats.frameCount`, 그리고 `inference_geo`, `speed`, `iterations` 사용 필드를 생략했으며, `service_tier`를 `"standard" | "priority" | "batch"`로 입력했습니다. 타입이 선택 사항으로 표시하는 필드는 이전 버전에서 기록된 결과에 없을 수 있습니다.

<h3 id="askuserquestion-2">
  AskUserQuestion
</h3>

**도구 이름:** `AskUserQuestion`

```typescript theme={null}
type AskUserQuestionOutput = {
  questions: Array<{
    question: string;
    header: string;
    options: Array<{ label: string; description: string; preview?: string }>;
    multiSelect: boolean;
  }>;
  answers: Record<string, string>;
  response?: string;
  annotations?: Record<string, { preview?: string; notes?: string }>;
  afkTimeoutMs?: number;
};
```

질문과 사용자의 답변을 반환합니다. `response`는 사용자가 구조화된 질문에 답하는 대신 자유 형식 답변을 입력했을 때 설정됩니다. 존재할 때, Claude는 질문별 답변 목록 대신 "사용자가 응답했습니다: …"를 받습니다.

<h3 id="bash-2">
  Bash
</h3>

**도구 이름:** `Bash`

```typescript theme={null}
type BashOutput = {
  stdout: string;
  stderr: string;
  rawOutputPath?: string;
  interrupted: boolean;
  isImage?: boolean;
  backgroundTaskId?: string;
  backgroundedByUser?: boolean;
  timedOutAfterMs?: number;
  backgroundCwdHint?: string;
  backgroundEndsWithFinalResponse?: true;
  dangerouslyDisableSandbox?: boolean;
  returnCodeInterpretation?: string;
  noOutputExpected?: boolean;
  structuredContent?: unknown[];
  persistedOutputPath?: string;
  persistedOutputSize?: number;
  staleReadFileStateHint?: string;
  ghRateLimitHint?: string;
  gitOperation?: {
    commit?: { sha: string; kind: "committed" | "amended" | "cherry-picked"; branch?: string };
    push?: { branch: string };
    branch?: { ref: string; action: "merged" | "rebased" };
    pr?: {
      number: number;
      url?: string;
      action: "created" | "edited" | "merged" | "commented" | "closed" | "reopened" | "ready" | "draft" | "auto-merge-enabled" | "auto-merge-disabled";
    };
  };
};
```

`stdout`, `stderr`, `backgroundTaskId` 필드는 다음을 전달합니다:

| 필드                 | 전달 내용                                            |
| ------------------ | ------------------------------------------------ |
| `stdout`           | 명령의 stdout과 stderr, 하나의 인터리브된 스트림으로 병합됨          |
| `stderr`           | 셸 작업 디렉토리 재설정과 같이 도구 자체가 추가하는 알림, 명령의 stderr가 아님 |
| `backgroundTaskId` | 백그라운드 명령에 대해 존재함                                 |

`timedOutAfterMs`는 밀리초 단위의 타임아웃이며, 명령이 타임아웃에 도달하고 명시적으로 시작하지 않고 백그라운드로 이동했을 때 설정됩니다. `backgroundCwdHint`는 백그라운드 명령에 `cd`, `pushd`, `popd` 또는 `chdir`과 같은 디렉토리 변경 내장이 포함되어 있을 때 설정되며, 세션 작업 디렉토리가 변경되지 않았음을 나타냅니다. 두 필드 모두 Claude Code v2.1.210 이상이 필요합니다.

포그라운드에서 실행 중인 서브에이전트가 백그라운드 명령을 소유할 때, Claude Code는 해당 서브에이전트가 최종 응답을 제공할 때 명령을 종료합니다. Claude Code는 이러한 명령에 `backgroundEndsWithFinalResponse`를 `true`로 설정하고, 명령이 턴을 유지할 때 필드를 생략합니다. 주 대화 또는 백그라운드 서브에이전트에서 시작한 명령처럼 필드는 Claude Code v2.1.227 이상이 필요합니다.

Claude Code는 `gitOperation.commit.branch`를 git의 커밋 요약 줄에 명명된 분기로 설정하고, 분리된 HEAD에서 만든 커밋의 경우 생략합니다. 이 필드는 Agent SDK v0.3.227 이상이 필요합니다. Claude Code는 `gh pr reopen` 명령을 `reopened` PR 작업으로 보고하며, 이는 Agent SDK v0.3.234 이상이 필요합니다.

<h3 id="monitor-2">
  Monitor
</h3>

**도구 이름:** `Monitor`

```typescript theme={null}
type MonitorOutput = {
  taskId: string;
  timeoutMs: number;
  persistent?: boolean;
};
```

실행 중인 모니터의 백그라운드 작업 ID를 반환합니다. 이 ID를 `TaskStop`과 함께 사용하여 감시를 조기에 취소합니다.

<h3 id="edit-2">
  Edit
</h3>

**도구 이름:** `Edit`

```typescript theme={null}
type FileEditOutput = {
  filePath: string;
  oldString: string;
  newString: string;
  originalFile: string | null;
  structuredPatch: Array<{
    oldStart: number;
    oldLines: number;
    newStart: number;
    newLines: number;
    lines: string[];
  }>;
  userModified: boolean;
  replaceAll: boolean;
  gitDiff?: {
    filename: string;
    status: "modified" | "added";
    additions: number;
    deletions: number;
    changes: number;
    patch: string;
    repository?: string | null;
  };
};
```

편집 작업의 구조화된 diff를 반환합니다.

<h3 id="read-2">
  Read
</h3>

**도구 이름:** `Read`

```typescript theme={null}
type FileReadOutput =
  | {
      type: "text";
      file: {
        filePath: string;
        content: string;
        numLines: number;
        startLine: number;
        totalLines: number;
        /** True when a whole-file read was auto-paginated because it exceeded the token cap (the content is a partial first page). */
        truncatedByTokenCap?: boolean;
      };
    }
  | {
      type: "image";
      file: {
        base64: string;
        type: "image/jpeg" | "image/png" | "image/gif" | "image/webp";
        originalSize: number;
        dimensions?: {
          originalWidth?: number;
          originalHeight?: number;
          displayWidth?: number;
          displayHeight?: number;
        };
      };
    }
  | {
      type: "notebook";
      file: {
        filePath: string;
        cells: unknown[];
      };
    }
  | {
      type: "pdf";
      file: {
        filePath: string;
        base64: string;
        originalSize: number;
      };
    }
  | {
      type: "parts";
      file: {
        filePath: string;
        originalSize: number;
        count: number;
        outputDir: string;
      };
      /** Document page number of the first extracted page; labels the page images in the tool_result content. */
      firstPage?: number;
      /** In-process only: the page-image bytes are delivered as image blocks in the tool_result content and aren't retained on the emitted tool_use_result, so this key is absent there. */
      pages?: {
        base64: string;
        mediaType: "image/jpeg" | "image/png" | "image/gif" | "image/webp";
        error?: string;
      }[];
    }
  | {
      type: "file_unchanged";
      file: {
        filePath: string;
      };
      /** Set when the dedup matched a startup-seeded entry (CLAUDE.md / nested memory) rather than a prior Read tool_result. */
      source?: "seeded";
    };
```

파일 타입에 적합한 형식의 파일 콘텐츠를 반환합니다. `type` 필드에서 구분됩니다.

<h3 id="write-2">
  Write
</h3>

**도구 이름:** `Write`

```typescript theme={null}
type FileWriteOutput = {
  type: "create" | "update";
  filePath: string;
  content: string;
  structuredPatch: Array<{
    oldStart: number;
    oldLines: number;
    newStart: number;
    newLines: number;
    lines: string[];
  }>;
  originalFile: string | null;
  gitDiff?: {
    filename: string;
    status: "modified" | "added";
    additions: number;
    deletions: number;
    changes: number;
    patch: string;
    repository?: string | null;
  };
  userModified?: boolean;
};
```

구조화된 diff 정보를 포함한 쓰기 결과를 반환합니다. `originalFile`과 `structuredPatch`가 보유하는 내용은 쓰기에 따라 다릅니다:

* 새로 생성된 파일의 경우, `originalFile`은 null이고 `structuredPatch`는 비어 있습니다
* 덮어쓰기에서 `originalFile`은 이전 콘텐츠를 전달하지만, 해당 콘텐츠가 약 10MB보다 클 때는 예외입니다: Claude Code는 diff를 건너뛰고 `originalFile` null과 `structuredPatch` 빈 상태를 반환합니다
* 쓰기가 아무것도 변경하지 않거나 diff가 타임아웃되면 `structuredPatch`도 비어 있습니다

<h3 id="glob-2">
  Glob
</h3>

**도구 이름:** `Glob`

```typescript theme={null}
type GlobOutput = {
  durationMs: number;
  numFiles: number;
  filenames: string[];
  truncated: boolean;
  totalMatches?: number;
  countIsComplete?: boolean;
};
```

glob 패턴과 일치하는 파일 경로를 수정 시간별로 정렬하여 반환합니다.

`totalMatches`와 `countIsComplete`는 Claude Code v2.1.191 이상이 필요합니다. `totalMatches`는 잘림 전 일치하는 파일의 수를 보고합니다. `countIsComplete`가 false일 때, `totalMatches`는 기본 검색이 자신의 출력을 잘랐기 때문에 하한입니다.

<h3 id="grep-2">
  Grep
</h3>

**도구 이름:** `Grep`

```typescript theme={null}
type GrepOutput = {
  mode?: "content" | "files_with_matches" | "count";
  numFiles: number;
  filenames: string[];
  content?: string;
  numLines?: number;
  numMatches?: number;
  totalFiles?: number;
  totalLines?: number;
  appliedLimit?: number;
  appliedOffset?: number;
};
```

검색 결과를 반환합니다. 형태는 `mode`에 따라 다릅니다: 파일 목록, 일치 항목이 있는 콘텐츠 또는 일치 항목 수. `count` 모드에서 `numFiles`와 `numMatches`는 페이지 매김된 슬라이스가 아닌 전체 결과 집합에 대한 합계입니다. v2.1.208 이전에는 나열된 항목을 잘린 `head_limit` 또는 `offset`도 해당 합계를 잘랐습니다.

`totalFiles`는 Claude Code v2.1.208 이상이 필요하며 `files_with_matches` 모드에서 `head_limit` 및 `offset` 페이지 매김 전 결과의 총 수를 보고합니다. `totalLines`는 Claude Code v2.1.210 이상이 필요하며 `content` 모드에서 페이지 매김 전 줄의 총 수를 보고합니다.

<h3 id="taskstop-2">
  TaskStop
</h3>

**도구 이름:** `TaskStop`

```typescript theme={null}
type TaskStopOutput = {
  message: string;
  task_id: string;
  task_type: string;
  command?: string;
};
```

백그라운드 작업 중지 후 확인을 반환합니다.

<h3 id="notebookedit-2">
  NotebookEdit
</h3>

**도구 이름:** `NotebookEdit`

```typescript theme={null}
type NotebookEditOutput = {
  new_source: string;
  old_source?: string;
  cell_id?: string;
  cell_type: "code" | "markdown";
  language: string;
  edit_mode: string;
  error?: string;
  notebook_path: string;
  original_file: string;
  updated_file: string;
};
```

원본 및 업데이트된 파일 콘텐츠를 포함한 노트북 편집 결과를 반환합니다.

<h3 id="webfetch-2">
  WebFetch
</h3>

**도구 이름:** `WebFetch`

```typescript theme={null}
type WebFetchOutput = {
  bytes: number;
  code: number;
  codeText: string;
  result: string;
  durationMs: number;
  url: string;
  artifactRead?: {
    slug: string;
    ver?: string;
    seeded?: false;
  };
};
```

HTTP 상태 및 메타데이터를 포함한 가져온 콘텐츠를 반환합니다.

`artifactRead`는 Claude Code가 아티팩트 읽기의 자체 기록이며, Claude가 세션이 게시할 수 있는 아티팩트를 가져왔을 때만 존재합니다. Claude Code는 세션이 재개될 때 이를 다시 읽어서 나중의 게시가 올바른 버전을 기반으로 하도록 합니다. 코드는 이에 대해 조치할 필요가 없습니다. `slug`는 아티팩트의 이름을 지정하고, `ver`는 읽기가 기록한 버전이며 기록하지 않았을 때 없으며, `seeded: false`는 전체 소스가 Claude에 도달하지 않은 읽기를 표시합니다. `seeded` 필드는 Agent SDK v0.3.239 이상이 필요합니다.

<h3 id="websearch-2">
  WebSearch
</h3>

**도구 이름:** `WebSearch`

```typescript theme={null}
type WebSearchOutput = {
  query: string;
  results: Array<
    | {
        tool_use_id: string;
        content: Array<{ title: string; url: string }>;
      }
    | string
  >;
  durationSeconds: number;
  searchCount?: number;
};
```

웹에서 검색 결과를 반환합니다.

<h3 id="workflow-2">
  Workflow
</h3>

**도구 이름:** `Workflow`

```typescript theme={null}
type WorkflowOutput = {
  status: "async_launched" | "remote_launched";
  taskId: string;
  taskType?: "local_workflow" | "remote_agent";
  workflowName?: string;
  runId?: string;
  summary?: string;
  transcriptDir?: string;
  scriptPath?: string;
  sessionUrl?: string; // set when the workflow launched as a cloud session
  warning?: string;
  error?: string;
};
```

도구가 호출을 수락한 후 즉시 반환됩니다. 최종 결과는 나중에 작업 완료로 도착합니다. 실행이 시작된 것으로 취급하기 전에 `error`를 확인하십시오: 구문 검사에 실패한 스크립트는 `status: "async_launched"`를 반환하고 `error`가 설정되며, 실행되지 않습니다.

| 필드              | 타입                                      | 설명                                                                                                |
| --------------- | --------------------------------------- | ------------------------------------------------------------------------------------------------- |
| `status`        | `"async_launched" \| "remote_launched"` | 도구가 호출을 수락했습니다. 인프로세스 실행의 경우 `"async_launched"`, 클라우드 세션으로 전달된 실행의 경우 `"remote_launched"`         |
| `taskId`        | `string`                                | 실행을 위한 백그라운드 작업 식별자                                                                               |
| `taskType`      | `"local_workflow" \| "remote_agent"`    | 등록된 백그라운드 작업의 작업 타입, `status` 팔과 일치함                                                              |
| `workflowName`  | `string`                                | 워크플로우 스크립트의 `meta.name`                                                                           |
| `runId`         | `string`                                | 나중의 호출에서 `resumeFromRunId`로 전달할 워크플로우 실행 식별자. `remote_launched` 실행의 경우 없으며, 클라우드 세션 URL이 재개 핸들입니다 |
| `summary`       | `string`                                | 워크플로우가 수행하는 작업에 대한 한 줄 설명                                                                         |
| `transcriptDir` | `string`                                | 실행 중에 서브에이전트 트랜스크립트가 작성되는 디렉토리                                                                    |
| `scriptPath`    | `string`                                | 이 실행을 위해 유지된 워크플로우 스크립트의 경로입니다. 스크립트를 다시 보내지 않고 다시 실행하려면 편집하고 `scriptPath`로 다시 전달하십시오             |
| `sessionUrl`    | `string`                                | 클라우드 세션 URL, `status`가 `"remote_launched"`일 때 설정됨                                                 |
| `warning`       | `string`                                | 로컬 git 상태가 클라우드 세션이 복제할 푸시된 분기와 다르게 발산하는 것과 같은 차단하지 않는 알림                                         |
| `error`         | `string`                                | 스크립트가 구문 검사에 실패할 때 설정됩니다. 존재할 때, 실행은 시작된 상태에도 불구하고 시작되지 않았습니다                                     |

<h3 id="todowrite-2">
  TodoWrite
</h3>

**도구 이름:** `TodoWrite`

```typescript theme={null}
type TodoWriteOutput = {
  oldTodos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
  newTodos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
};
```

이전 및 업데이트된 작업 목록을 반환합니다.

<Note>
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.

  [모델 가용성](/docs/ko/agent-sdk/todo-tracking#model-availability)을 참조하여 옵트인하십시오.
</Note>

<h3 id="taskcreate-2">
  TaskCreate
</h3>

**도구 이름:** `TaskCreate`

```typescript theme={null}
type TaskCreateOutput = {
  task: {
    id: string;
    subject: string;
  };
};
```

할당된 ID와 함께 생성된 작업을 반환합니다.

<h3 id="taskupdate-2">
  TaskUpdate
</h3>

**도구 이름:** `TaskUpdate`

```typescript theme={null}
type TaskUpdateOutput = {
  success: boolean;
  taskId: string;
  updatedFields: string[];
  error?: string;
  statusChange?: {
    from: string;
    to: string;
  };
};
```

업데이트 결과를 반환하며, 어떤 필드가 변경되었는지 포함합니다.

<h3 id="taskget-2">
  TaskGet
</h3>

**도구 이름:** `TaskGet`

```typescript theme={null}
type TaskGetOutput = {
  task: {
    id: string;
    subject: string;
    description: string;
    status: "pending" | "in_progress" | "completed";
    blocks: string[];
    blockedBy: string[];
  } | null;
};
```

전체 작업 레코드를 반환하거나, ID를 찾을 수 없을 때 `null`을 반환합니다.

<h3 id="tasklist-2">
  TaskList
</h3>

**도구 이름:** `TaskList`

```typescript theme={null}
type TaskListOutput = {
  tasks: Array<{
    id: string;
    subject: string;
    status: "pending" | "in_progress" | "completed";
    owner?: string;
    blockedBy: string[];
  }>;
};
```

현재 목록의 모든 작업의 스냅샷을 반환합니다.

<h3 id="exitplanmode-2">
  ExitPlanMode
</h3>

**도구 이름:** `ExitPlanMode`

```typescript theme={null}
type ExitPlanModeOutput = {
  plan: string | null;
  isAgent: boolean;
  filePath?: string;
  hasTaskTool?: boolean;
  planWasEdited?: boolean;
  awaitingLeaderApproval?: boolean;
  requestId?: string;
};
```

계획 모드 종료 후 계획 상태를 반환합니다.

<h3 id="listmcpresources-2">
  ListMcpResources
</h3>

**도구 이름:** `ListMcpResourcesTool`

```typescript theme={null}
type ListMcpResourcesOutput = Array<{
  uri: string;
  name: string;
  mimeType?: string;
  description?: string;
  server: string;
}>;
```

사용 가능한 MCP 리소스의 배열을 반환합니다.

<h3 id="readmcpresource-2">
  ReadMcpResource
</h3>

**도구 이름:** `ReadMcpResourceTool`

```typescript theme={null}
type ReadMcpResourceOutput = {
  contents: Array<{
    uri: string;
    mimeType?: string;
    text?: string;
    blobSavedTo?: string;
  }>;
  error?: string;
};
```

요청된 MCP 리소스의 콘텐츠를 반환합니다.

<h3 id="enterworktree-2">
  EnterWorktree
</h3>

**도구 이름:** `EnterWorktree`

```typescript theme={null}
type EnterWorktreeOutput = {
  worktreePath: string;
  worktreeBranch?: string;
  message: string;
};
```

git worktree에 대한 정보를 반환합니다.

<h3 id="exitworktree-2">
  ExitWorktree
</h3>

**도구 이름:** `ExitWorktree`

```typescript theme={null}
type ExitWorktreeOutput = {
  action: "keep" | "remove";
  originalCwd: string;
  worktreePath: string;
  worktreeBranch?: string;
  tmuxSessionName?: string;
  discardedFiles?: number;
  discardedCommits?: number;
  message: string;
};
```

수행된 작업과 종료된 worktree에 대한 세부 정보를 반환합니다.

<h3 id="enterplanmode-2">
  EnterPlanMode
</h3>

**도구 이름:** `EnterPlanMode`

```typescript theme={null}
type EnterPlanModeOutput = {
  message: string;
};
```

계획 모드가 입력되었음을 확인하는 메시지를 반환합니다.

<h3 id="croncreate-2">
  CronCreate
</h3>

**도구 이름:** `CronCreate`

```typescript theme={null}
type CronCreateOutput = {
  id: string;
  humanSchedule: string;
  recurring: boolean;
  durable?: boolean; // true when persisted to .claude/scheduled_tasks.json; false when session-only
};
```

작업 ID와 일정에 대한 사람이 읽을 수 있는 설명을 반환합니다.

<h3 id="crondelete-2">
  CronDelete
</h3>

**도구 이름:** `CronDelete`

```typescript theme={null}
type CronDeleteOutput = {
  id: string;
};
```

삭제된 작업의 ID를 반환합니다.

<h3 id="cronlist-2">
  CronList
</h3>

**도구 이름:** `CronList`

```typescript theme={null}
type CronListOutput = {
  jobs: {
    id: string;
    cron: string;
    humanSchedule: string;
    prompt: string;
    recurring?: boolean;
    durable?: boolean;
  }[];
};
```

예약된 cron 작업을 반환합니다: `.claude/scheduled_tasks.json`의 지속 가능한 작업과 현재 세션의 세션 전용 작업입니다. 세션 전용 작업은 `durable: false`를 전달합니다. 디스크에서 읽은 작업은 필드를 생략합니다.

<h3 id="schedulewakeup-2">
  ScheduleWakeup
</h3>

**도구 이름:** `ScheduleWakeup`

```typescript theme={null}
type ScheduleWakeupOutput = {
  scheduledFor: number;
  clampedDelaySeconds: number;
  wasClamped: boolean;
  stopped?: boolean;
  cancelledWakeups?: number;
};
```

웨이크업이 발생할 시간을 epoch 밀리초 타임스탬프로, 실제로 사용된 지연, 요청된 지연이 고정되었는지 여부를 반환합니다. `stopped` 필드는 호출이 `stop: true`로 루프를 종료했을 때 `true`입니다. Claude Code v2.1.202 이상이 필요합니다. `cancelledWakeups` 필드는 `stop: true` 호출이 취소한 보류 중인 웨이크업의 수를 계산합니다. 0 값은 아무것도 보류 중이지 않음을 의미하며, 반복 `/loop` cron은 `stop: true`로 취소되지 않습니다. Claude Code v2.1.206 이상이 필요합니다.

<h3 id="remotetrigger-2">
  RemoteTrigger
</h3>

**도구 이름:** `RemoteTrigger`

```typescript theme={null}
type RemoteTriggerOutput = {
  status: number;
  json: string;
  summary?: string;
};
```

트리거 작업에 대한 API 응답 상태 및 본문을 반환합니다.

<h3 id="pushnotification-2">
  PushNotification
</h3>

**도구 이름:** `PushNotification`

```typescript theme={null}
type PushNotificationOutput = {
  message: string;
  pushSent?: boolean;
  localSent?: boolean;
  disabledReason?: "config_off" | "user_present" | "no_transport";
  sentAt?: string;
};
```

푸시 또는 로컬 알림이 전송되었는지 여부와 배달이 건너뛴 이유를 포함한 배달 세부 정보를 반환합니다.

<h3 id="reportfindings-2">
  ReportFindings
</h3>

**도구 이름:** `ReportFindings`

```typescript theme={null}
type ReportFindingsOutput = {
  count: number;
  level?: "low" | "medium" | "high" | "xhigh" | "max";
  findings: Array<{
    file: string;
    line?: number;
    summary: string;
    failure_scenario: string;
    short_summary?: string;
    category?: string;
    verdict?: "CONFIRMED" | "PLAUSIBLE";
    outcome?: "fixed" | "skipped" | "no_change_needed";
  }>;
};
```

보고된 결과의 수, 검토가 실행된 노력 수준, 결과 본문에 대해 에코백된 결과를 반환합니다. Claude Code v2.1.196 이상이 필요합니다. 에코백된 `short_summary` 필드는 Claude Code v2.1.212 이상이 필요합니다.

<h3 id="artifact-2">
  Artifact
</h3>

**도구 이름:** `Artifact`

```typescript theme={null}
type ArtifactOutput =
  | {
      url: string;
      path: string;
      title?: string;
      version?: string;
      capabilities?: unknown;
      stored?: {
        contract: string;
        capabilities?: Record<string, unknown>;
      };
      warnings?: string[];
      contract?: string;
      updated?: boolean;
      liveSubscription?: string;
    }
  | {
      artifacts: Array<{
        title: string;
        url: string;
        updatedAt?: string;
        rel?: "mine" | "shared";
      }>;
      truncated?: boolean;
      scope?: "shared" | "all";
    };
```

게시된 페이지의 `url`과 게시 작업을 위해 게시된 로컬 `path`를 반환하며, 게시가 기존 아티팩트를 재배포했을 때 `updated`가 true로 설정되고, `warnings`는 게시 시간 권고사항을 전달합니다. 목록 작업은 대신 `artifacts` 행을 반환하며, 더 많은 아티팩트가 요청된 제한보다 존재할 때 `truncated`가 설정됩니다. 범위가 `"mine"`이 아닌 목록에서 각 행은 사용자가 아티팩트를 소유하는지 또는 공유되었는지를 표시하는 `rel`을 전달하고, 출력의 `scope`는 어떤 비기본 범위가 목록을 생성했는지 기록합니다. 둘 다 기본 목록에서 없습니다.

<h3 id="projects-2">
  Projects
</h3>

**도구 이름:** `Projects`

```typescript theme={null}
type ProjectsOutput =
  | {
      method: "project_info";
      notice?: string;
      name: string;
      description: string;
      instructions: string;
      docs: Array<{ path: string; created_at: string | null }>;
      files?: Array<{
        path: string;
        file_kind: string;
        created_at: string | null;
      }>;
      sync_sources?: Array<{
        type: string | null;
        config: Record<string, unknown>;
      }>;
      knowledge: {
        knowledge_size: number;
        max_knowledge_size: number;
      };
    }
  | {
      method: "project_read";
      notice?: string;
      path: string;
      file_kind?: string;
      content?: string;
      local_file?: string;
      created_at: string | null;
    }
  | {
      method: "project_search";
      notice?: string;
      rag: boolean;
      hits?: Array<{ name?: string; doc_uuid?: string; text?: string }>;
      docs?: string[];
    }
  | {
      method: "project_write";
      notice?: string;
      path: string;
      doc_uuid: string;
      replaced: boolean;
      present_to_user?: boolean;
      local_path?: string;
    }
  | {
      method: "project_delete";
      notice?: string;
      path: string;
      deleted: boolean;
    };
```

`method` 필드에서 구분되며, 입력을 반영합니다. `project_read`는 작은 텍스트 문서를 `content`에 인라인으로 반환하고 더 큰 문서를 대신 `local_file` 경로에 작성합니다. `project_search`는 프로젝트의 인덱스를 사용할 수 있을 때 RAG `hits`를 `rag: true`로 반환하고 그렇지 않으면 `docs` 경로 목록으로 폴백합니다.

<h3 id="readmcpresourcedir-2">
  ReadMcpResourceDir
</h3>

**도구 이름:** `ReadMcpResourceDirTool`

```typescript theme={null}
type ReadMcpResourceDirOutput = {
  resources: Array<{
    uri: string;
    name: string;
    mimeType?: string;
  }>;
  error?: string;
};
```

디렉토리 리소스의 직접 자식을 반환합니다. 하위 디렉토리는 mimeType `"inode/directory"`로 나타나며, `error`는 서버가 디렉토리를 나열할 수 없을 때 사람이 읽을 수 있는 메시지를 전달합니다.

<h3 id="refreshmcptools-2">
  RefreshMcpTools
</h3>

**도구 이름:** `RefreshMcpTools`

```typescript theme={null}
type RefreshMcpToolsOutput = Array<{
  server: string;
  status: "refreshed" | "error" | "not_connected";
  toolCount?: number; // tools now available from this server
  added?: string[]; // tool names this refresh added
  removed?: string[]; // tool names this refresh removed
  error?: string; // why the refresh failed or the server was unavailable
}>;
```

서버당 하나의 항목을 반환합니다: `refreshed`는 다시 쿼리된 도구 목록이 적용되었음을 의미하고, `error`는 다시 쿼리가 실패했고 이전 도구 집합이 유지되었음을 의미하며, `not_connected`는 서버가 쿼리할 라이브 연결이 없음을 의미합니다.

<h3 id="showonboardingrolepicker-2">
  ShowOnboardingRolePicker
</h3>

**도구 이름:** `ShowOnboardingRolePicker`

```typescript theme={null}
type ShowOnboardingRolePickerOutput = {
  role?: string;
  dismissed?: boolean;
};
```

사용자의 선택을 반환합니다: 역할 칩을 선택하거나 입력했을 때 `role`, 피커를 닫았을 때 `dismissed: true`. 빈 객체는 사용자가 역할을 선택하지 않고 호출을 승인했음을 의미합니다.

<h3 id="mcpoutput">
  McpOutput
</h3>

**도구 이름:** `mcp__<server>__<tool>` 형식의 동적 MCP 도구 이름

```typescript theme={null}
type McpOutput =
  | string
  | {
      type: string;
      [k: string]: unknown;
    }[]
  | {
      [k: string]: unknown;
    };
```

MCP 도구 결과는 서버에 따라 문자열 또는 콘텐츠 블록 배열로 반환됩니다. 내보낸 타입의 후행 일반 객체 분기는 스키마 생성 아티팩트입니다: SDK는 서버의 구조화된 출력이 반환되기 전에 JSON 문자열로 직렬화되기 때문에 베어 객체를 반환하지 않습니다. 런타임에 값은 `undefined`일 수도 있지만, 내보낸 타입은 이를 모델링하지 않습니다.

<h2 id="permission-types">
  권한 타입
</h2>

<h3 id="permissionupdate">
  `PermissionUpdate`
</h3>

권한을 업데이트하기 위한 작업입니다.

```typescript theme={null}
type PermissionUpdate =
  | {
      type: "addRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "replaceRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "removeRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "setMode";
      mode: PermissionMode;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "addDirectories";
      directories: string[];
      destination: PermissionUpdateDestination;
    }
  | {
      type: "removeDirectories";
      directories: string[];
      destination: PermissionUpdateDestination;
    };
```

<h3 id="permissionbehavior">
  `PermissionBehavior`
</h3>

```typescript theme={null}
type PermissionBehavior = "allow" | "deny" | "ask";
```

<h3 id="permissionupdatedestination">
  `PermissionUpdateDestination`
</h3>

```typescript theme={null}
type PermissionUpdateDestination =
  | "userSettings" // 전역 사용자 설정
  | "projectSettings" // 디렉토리별 프로젝트 설정
  | "localSettings" // 로컬 프로젝트 설정
  | "session" // 현재 세션만
  | "cliArg"; // CLI 인수
```

<h3 id="permissionrulevalue">
  `PermissionRuleValue`
</h3>

```typescript theme={null}
type PermissionRuleValue = {
  toolName: string;
  ruleContent?: string;
};
```

<h2 id="other-types">
  기타 타입
</h2>

<h3 id="apikeysource">
  `ApiKeySource`
</h3>

세션의 요청에 대한 API 키가 어디에서 왔는지를 나타내며, [`SDKSystemMessage`](#sdksystemmessage) 초기화 메시지의 `apiKeySource`로 보고됩니다.

```typescript theme={null}
type ApiKeySource =
  | "ANTHROPIC_API_KEY"
  | "apiKeyHelper"
  | "/login managed key"
  | "none"
  | "user"
  | "project"
  | "org"
  | "temporary"
  | "oauth";
```

Claude Code는 다음 네 가지 값 중 하나를 보고합니다:

| 값                    | 사용 중인 키                                                                                           |
| -------------------- | ------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_API_KEY`  | `ANTHROPIC_API_KEY` 환경 변수의 키                                                                      |
| `apiKeyHelper`       | [`apiKeyHelper`](/docs/ko/settings-reference#apikeyhelper) 명령에서 반환된 키                                  |
| `/login managed key` | [Claude Console 계정](/docs/ko/authentication#claude-console-authentication)으로 로그인할 때 Claude Code가 저장한 키 |
| `none`               | API 키 없음. 세션이 다른 방식으로 인증됩니다. 예를 들어 claude.ai 로그인, 베어러 토큰 또는 클라우드 제공자                              |

Agent SDK v0.3.234 이상은 타입에서 이 네 가지 값을 나열합니다. 타입은 또한 `user`, `project`, `org`, `temporary` 및 `oauth`를 유지하므로 이전 코드는 여전히 컴파일되며, Claude Code는 이들을 보고하지 않습니다.

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

`betas` 옵션을 통해 활성화할 수 있는 사용 가능한 베타 기능입니다. [베타 헤더](https://platform.claude.com/docs/en/api/beta-headers)를 참조하세요.

```typescript theme={null}
type SdkBeta = "context-1m-2025-08-07";
```

<Warning>
  `context-1m-2025-08-07` 베타는 2026년 4월 30일부터 폐기되었습니다. Claude Sonnet 4.5 또는 Sonnet 4와 함께 이 값을 전달하면 효과가 없으며, 표준 200k 토큰 컨텍스트 윈도우를 초과하는 요청은 오류를 반환합니다. 1M 토큰 컨텍스트 윈도우를 사용하려면 [Claude Opus 5.5, Claude Opus 5, Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7 또는 Claude Opus 4.8](https://platform.claude.com/docs/en/about-claude/models/overview)으로 마이그레이션하세요. 이들은 베타 헤더 없이 표준 가격으로 1M 컨텍스트를 포함합니다.
</Warning>

<h3 id="slashcommand">
  `SlashCommand`
</h3>

사용 가능한 명령에 대한 정보입니다.

```typescript theme={null}
type SlashCommand = {
  name: string;
  description: string;
  argumentHint: string;
  aliases?: string[];
  builtin?: boolean;
};
```

`builtin`은 명령이 Claude Code 자체이고 `/name`을 입력하면 실행될 때 행에서 `true`입니다. 사용자, 프로젝트, 플러그인 또는 MCP 서버에서 정의한 명령이나, 이들 중 하나가 [이름으로 대체](/docs/ko/skills#resolve-skills-that-share-a-name)하는 번들 명령의 경우 없습니다. Agent SDK v0.3.277 이상이 필요합니다.

<h3 id="modelinfo">
  `ModelInfo`
</h3>

사용 가능한 모델에 대한 정보입니다.

```typescript theme={null}
type ModelInfo = {
  value: string;
  resolvedModel?: string;
  displayName: string;
  description: string;
  supportsEffort?: boolean;
  supportedEffortLevels?: ("low" | "medium" | "high" | "xhigh" | "max")[];
  supportsAdaptiveThinking?: boolean;
  supportsFastMode?: boolean;
  supportsAutoMode?: boolean;
};
```

| 필드                         | 타입                                                                 | 설명                                                                                                                                                                                 |
| :------------------------- | :----------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `value`                    | `string`                                                           | API 호출에서 전달할 모델 식별자                                                                                                                                                                |
| `resolvedModel`            | `string \| undefined`                                              | 이 항목의 `value`가 확인되는 정규 와이어 모델 ID입니다. `sonnet`과 같은 별칭 항목은 `claude-sonnet-5`와 같은 명시적 모델 ID로 확인되므로, 호스트는 저장된 명시적 모델 ID를 이 별칭 항목이 포함하는 것과 일치시킬 수 있습니다. Claude Code v2.1.197 이상이 필요합니다. |
| `displayName`              | `string`                                                           | 사람이 읽을 수 있는 표시 이름                                                                                                                                                                  |
| `description`              | `string`                                                           | 모델의 기능에 대한 설명                                                                                                                                                                      |
| `supportsEffort`           | `boolean \| undefined`                                             | 이 모델이 노력 수준을 지원하는지 여부                                                                                                                                                              |
| `supportedEffortLevels`    | `("low" \| "medium" \| "high" \| "xhigh" \| "max")[] \| undefined` | 이 모델이 허용하는 노력 수준                                                                                                                                                                   |
| `supportsAdaptiveThinking` | `boolean \| undefined`                                             | 이 모델이 Claude가 언제 그리고 얼마나 생각할지 결정하는 적응형 사고를 지원하는지 여부                                                                                                                                |
| `supportsFastMode`         | `boolean \| undefined`                                             | 이 모델이 빠른 모드를 지원하는지 여부                                                                                                                                                              |
| `supportsAutoMode`         | `boolean \| undefined`                                             | 이 모델이 자동 모드를 지원하는지 여부                                                                                                                                                              |

<h3 id="agentinfo">
  `AgentInfo`
</h3>

Agent 도구를 통해 호출할 수 있는 사용 가능한 서브에이전트에 대한 정보입니다.

```typescript theme={null}
type AgentInfo = {
  name: string;
  description: string;
  model?: string;
};
```

| 필드            | 타입                    | 설명                                                                                                                                                          |
| :------------ | :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | `string`              | 에이전트 타입 식별자 (예: `"Explore"`, `"general-purpose"`)                                                                                                           |
| `description` | `string`              | 이 에이전트를 사용할 시기에 대한 설명                                                                                                                                       |
| `model`       | `string \| undefined` | 이 에이전트가 사용하는 모델입니다. 별칭 또는 모델 ID이거나, 부모의 모델을 상속하는 경우 `'inherit'`입니다. `undefined`일 때, Claude Code는 [서브에이전트 모델 순서](/docs/ko/sub-agents#choose-a-model)에서 모델을 선택합니다. |

<h3 id="mcpserverprovenance">
  `McpServerProvenance`
</h3>

`mcp__*` 도구를 제공하는 MCP 서버와 해당 서버의 정의가 어디에서 왔는지입니다. [`PreToolUse`](#pretoolusehookinput), `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` 및 `PermissionDenied` 훅 입력은 이를 `mcp_server`로 전달하고, [`CanUseTool`](#canusetool) 옵션은 이를 `mcpServer`로 전달합니다. 둘 다 MCP 서버에서 오지 않는 도구의 경우 이를 생략합니다.

```typescript theme={null}
type McpServerProvenance = {
  name: string;
  source: string;
};
```

| 필드       | 타입       | 설명                                                                       |
| :------- | :------- | :----------------------------------------------------------------------- |
| `name`   | `string` | 서버가 등록된 이름이며, [`mcpServerStatus()`](#query-object)가 이에 대해 보고하는 동일한 값입니다. |
| `source` | `string` | 서버의 정의가 어디에서 왔는지: `sdk`, `plugin` 또는 구성 범위                               |

`source`는 다음 값 중 하나를 취합니다. 집합이 열려 있으므로, 인식하지 못하는 값을 구성된 소스로 취급하고, `sdk`로 취급하지 마세요:

* **`sdk`**: 애플리케이션이 등록한 인프로세스 서버입니다. SDK 호스트 애플리케이션만 하나를 등록할 수 있으므로, 구성된 서버는 이름이 무엇이든 `sdk`를 보고하지 않습니다.
* **`plugin`**: [플러그인](/docs/ko/agent-sdk/plugins)이 제공하는 서버입니다. 해당 `name`은 [플러그인 제공 MCP 서버](/docs/ko/mcp#plugin-provided-mcp-servers) 아래에 설명된 범위가 지정된 `plugin:<plugin-name>:<server-name>` 형식입니다.
* **구성 범위**: `user`, `project`, `local`, `dynamic`, `managed`, `enterprise`, `claudeai` 또는 `agent`. `.mcp.json` 서버는 `project`를 보고하고, [MCP 설치 범위](/docs/ko/mcp#mcp-installation-scopes)는 `local`, `project` 및 `user`를 정의합니다. 애플리케이션이 [`mcpServers` 옵션](#options)에서 전달하는 서버(인프로세스 SDK 서버 제외)는 `dynamic`을 보고합니다.

`name` 또는 `mcp__<server>__` 도구 이름 접두사가 아닌 `source`를 기반으로 신뢰 결정을 내립니다. `sdk` 이외의 모든 소스에 대해 `name`은 신뢰할 수 없는 텍스트입니다. 표시하기 전에 이스케이프하세요.

`McpServerProvenance` 및 이를 전달하는 필드는 Agent SDK v0.3.274 이상이 필요합니다.

<h3 id="mcpserverstatus">
  `McpServerStatus`
</h3>

연결된 MCP 서버의 상태입니다.

```typescript theme={null}
type McpServerStatus = {
  name: string;
  status: "connected" | "failed" | "needs-auth" | "pending" | "disabled";
  serverInfo?: {
    name: string;
    version: string;
  };
  error?: string;
  config?: McpServerStatusConfig;
  scope?: string;
  source?: string;
  tools?: {
    name: string;
    description?: string;
    annotations?: {
      readOnly?: boolean;
      destructive?: boolean;
      openWorld?: boolean;
    };
    _meta?: Record<string, unknown>;
  }[];
};
```

`source`는 서버의 정의가 어디에서 왔는지를 나타내며, [`McpServerProvenance`](#mcpserverprovenance)의 `source`와 동일한 값 및 신뢰 규칙을 가집니다. 필드는 Agent SDK v0.3.274 이상이 필요하며 이전 버전에서는 없습니다.

`_meta`는 도구의 `_meta`의 MCP Apps 멤버를 전달하므로, 애플리케이션이 [`readMcpResource()`](#query-object)로 렌더링할 `ui://` 리소스를 찾을 수 있습니다. Claude Code는 `ui` 객체와 더 이상 사용되지 않는 평면 `ui/resourceUri` 문자열을 통과시키고, 다른 모든 키를 보류합니다. `ui` 내에서 `resourceUri`는 `ui://` 문자열이고 `visibility`는 서버가 설정할 때 `"model"` 및 `"app"`의 배열이며, 다른 멤버는 변경되지 않고 통과합니다. Claude Code는 값이 잘못된 형식일 때 어느 키든 삭제하고, 도구가 둘 다 선언하지 않을 때 `_meta`를 생략합니다. 필드는 초기화 메시지의 [`capabilities`](#sdksystemmessage)에 `mcp_tool_ui_meta_v1`이 포함될 때만 나타나며, TypeScript Agent SDK v0.3.280 이상이 필요합니다.

<h3 id="mcpserverstatusconfig">
  `McpServerStatusConfig`
</h3>

`mcpServerStatus()`에서 보고한 MCP 서버의 구성입니다. 이는 모든 MCP 서버 전송 타입의 합집합입니다.

```typescript theme={null}
type McpServerStatusConfig =
  | McpStdioServerConfig
  | McpSSEServerConfig
  | McpHttpServerConfig
  | McpSdkServerConfig
  | McpClaudeAIProxyServerConfig;
```

각 전송 타입의 세부 정보는 [`McpServerConfig`](#mcpserverconfig)를 참조하세요.

<h3 id="accountinfo">
  `AccountInfo`
</h3>

인증된 사용자의 계정 정보입니다.

```typescript theme={null}
type AccountInfo = {
  email?: string;
  organization?: string;
  subscriptionType?: string;
  tokenSource?: string;
  apiKeySource?: string;
};
```

<h3 id="modelusage">
  `ModelUsage`
</h3>

결과 메시지에서 반환된 모델별 사용 통계입니다. `costUSD` 값은 클라이언트 측 추정입니다. [비용 및 사용량 추적](/docs/ko/agent-sdk/cost-tracking)에서 청구 주의 사항을 참조하세요.

```typescript theme={null}
type ModelUsage = {
  inputTokens: number;
  outputTokens: number;
  thinkingTokens?: number;
  cacheReadInputTokens: number;
  cacheCreationInputTokens: number;
  webSearchRequests: number;
  costUSD: number;
  contextWindow: number;
  maxOutputTokens: number;
  canonicalModel?: string;
  provider?: string;
  costBasis?: 'list' | 'managed' | 'unknown';
};
```

`thinkingTokens`는 이 모델이 생성한 사고 토큰을 계산합니다. `outputTokens`는 이미 이들을 포함하므로 둘을 함께 더하지 마세요. 필드는 이를 기록하는 Claude Code 버전에서 턴이 실행될 때까지 없으므로, 이전 버전에서 시작된 재개된 세션은 부분 개수를 보고합니다. `thinkingTokens`는 Agent SDK v0.3.257 이상이 필요합니다.

`canonicalModel` 및 `provider` 필드는 Claude Code v2.1.218 이상이 필요합니다. `canonicalModel`은 가격 조회가 사용하는 정규 모델 ID입니다. 예를 들어 해당 문자열이 제공자별 ID 또는 별칭일 때 항목을 키하는 원본 모델 문자열과 다를 수 있습니다.

`provider`는 모델을 제공한 API 백엔드의 이름을 지정합니다. 예를 들어 `firstParty`, `bedrock`, `vertex`, `foundry`, `anthropicAws`, `mantle` 또는 `gateway`.

`costBasis`는 모델의 최신 요청에 가격을 책정한 가격 테이블의 이름을 지정합니다. 목록 가격의 경우 `list`, [`modelPricing`](/docs/ko/settings-reference#modelpricing) 테이블의 경우 `managed`, 또는 모델 ID와 일치하는 것이 없을 때 `unknown`. 필드는 Claude Code v2.1.246 이상이 필요합니다.

<h3 id="configscope">
  `ConfigScope`
</h3>

```typescript theme={null}
type ConfigScope = "local" | "user" | "project";
```

<h3 id="nonnullableusage">
  `NonNullableUsage`
</h3>

모든 nullable 필드가 non-nullable로 만들어진 [`Usage`](#usage)의 버전입니다.

```typescript theme={null}
type NonNullableUsage = {
  [K in keyof Usage]: NonNullable<Usage[K]>;
};
```

<h3 id="usage">
  `Usage`
</h3>

토큰 사용 통계입니다. 이는 `@anthropic-ai/sdk`의 `BetaUsage` 타입입니다.

```typescript theme={null}
type Usage = {
  input_tokens: number;
  output_tokens: number;
  cache_creation_input_tokens: number | null;
  cache_read_input_tokens: number | null;
  cache_creation: {
    ephemeral_5m_input_tokens: number;
    ephemeral_1h_input_tokens: number;
  } | null;
  server_tool_use: BetaServerToolUsage | null;
  service_tier: "standard" | "priority" | "batch" | null;
  speed: "standard" | "fast" | null;
  inference_geo: string | null;
  iterations: BetaIterationsUsage | null;
  output_tokens_details: BetaOutputTokensDetails | null;
};
```

`BetaServerToolUsage`, `BetaIterationsUsage` 및 `BetaOutputTokensDetails`는 `@anthropic-ai/sdk`에서 정의됩니다.

`output_tokens_details`는 청구된 출력을 카테고리별로 분류합니다. 현재 하나의 필드 `thinking_tokens: number`를 전달하며, 모델이 내부 추론으로 생성한 출력 토큰을 계산합니다. 여기에는 사고 블록 구분 기호가 포함됩니다. `output_tokens_details` 필드는 TypeScript SDK v0.3.228 이상이 필요하며, 이는 Claude Code v2.1.228을 번들로 제공합니다.

* **청구**: 청구를 위해서가 아니라 관찰 가능성을 위해 분류를 읽으세요. `output_tokens`는 권위 있는 합계로 유지되며, `output_tokens - thinking_tokens`는 비추론 출력을 근사합니다.
* **개수가 포함하는 것**: 모델이 생성한 원본 추론이며, 응답 본문에서 반환된 사고 텍스트보다 길 수 있습니다. API는 해당 원본 텍스트를 다시 토큰화하여 계산하므로, 모델의 정확한 생성 개수와 몇 토큰 정도 다를 수 있습니다.
* **스트리밍**: 스트리밍된 어시스턴트 메시지에서 이 분류는 `output_tokens`처럼 `message_start` 자리 표시자이며 실제 개수를 전달하지 않으므로, [결과 메시지에서 출력 토큰 읽기](/docs/ko/agent-sdk/cost-tracking#read-output-tokens-from-the-result-message)에서 설명하는 대로 결과 메시지에서 읽으세요. 결과 메시지에서 모델 또는 제공자가 분류를 보고하지 않을 때 `thinking_tokens`는 `0`을 읽습니다.
* **`null` 경우**: `output_tokens_details` 자체는 Claude Code가 합성하는 어시스턴트 메시지(예: API 오류 메시지)에서 `null`입니다.

<h3 id="calltoolresult">
  `CallToolResult`
</h3>

MCP 도구 결과 타입 (`@modelcontextprotocol/sdk/types.js`에서). `structuredContent`는 `content`와 함께 반환될 수 있는 JSON 객체이며, 이미지 블록을 포함합니다. [구조화된 데이터 반환](/docs/ko/agent-sdk/custom-tools#return-structured-data)을 참조하세요.

```typescript theme={null}
type CallToolResult = {
  content: Array<{
    type: "text" | "image" | "audio" | "resource" | "resource_link";
    // 추가 필드는 타입에 따라 다릅니다
  }>;
  structuredContent?: Record<string, unknown>;
  isError?: boolean;
};
```

<h3 id="sdkmcpresourcelink">
  `SDKMcpResourceLink`
</h3>

MCP 도구가 참조로 반환한 하나의 파일입니다. Claude Code는 도구의 결과에서 `resource_link` 블록에서 각 항목을 빌드하고 목록을 [`SDKUserMessage.tool_use_result`](#sdkusermessage)의 `resourceLinks`로 전달하거나, 호출이 백그라운드에서 완료되었을 때 [`SDKTaskNotificationMessage`](#sdktasknotificationmessage)의 `resource_links`로 전달합니다. Agent SDK v0.3.257 이상이 필요합니다.

```typescript theme={null}
type SDKMcpResourceLink = {
  uri: string;
  name: string;
  title?: string;
  description?: string;
  mimeType?: string;
  size?: number;
  annotations?: Record<string, unknown>;
};
```

Claude Code는 `uri` 또는 `name`이 문자열이 아닌 블록을 삭제하고, 값이 나열된 타입이 아닌 선택적 필드를 생략합니다.

| 필드            | 타입                                     | 설명                        |
| :------------ | :------------------------------------- | :------------------------ |
| `uri`         | `string`                               | 리소스의 URI이며, 서버가 반환한 것입니다. |
| `name`        | `string`                               | 서버가 리소스에 부여한 이름           |
| `title`       | `string \| undefined`                  | 서버가 설정한 경우 표시 제목          |
| `description` | `string \| undefined`                  | 서버가 설정한 경우 설명             |
| `mimeType`    | `string \| undefined`                  | 서버가 설정한 경우 MIME 타입        |
| `size`        | `number \| undefined`                  | 서버가 설정한 경우 바이트 단위의 크기     |
| `annotations` | `Record<string, unknown> \| undefined` | 서버가 설정한 경우 블록의 MCP 주석 객체  |

<h3 id="thinkingconfig">
  `ThinkingConfig`
</h3>

Claude의 사고/추론 동작을 제어합니다. 더 이상 사용되지 않는 `maxThinkingTokens`보다 우선합니다.

```typescript theme={null}
type ThinkingDisplay = "summarized" | "omitted";

type ThinkingConfig =
  | { type: "adaptive"; display?: ThinkingDisplay } // 모델이 언제 그리고 얼마나 추론할지 결정합니다 (Opus 4.6+)
  | { type: "enabled"; budgetTokens?: number; display?: ThinkingDisplay } // 고정 사고 토큰 예산
  | { type: "disabled" }; // 확장된 사고 없음
```

선택적 `display` 필드는 사고 텍스트가 `"summarized"` 또는 `"omitted"`로 반환되는지 제어합니다. Claude Opus 4.7 이상에서 API 기본값은 `"omitted"`이므로, `thinking` 블록에서 사고 콘텐츠를 받으려면 `"summarized"`를 설정하세요. Claude Code는 Amazon Bedrock 또는 Google Cloud의 Agent Platform에 `display`를 전송하지 않으므로, 이러한 제공자에서 Opus 4.7 이상은 `display`를 `"summarized"`로 설정한 경우에도 빈 `thinking` 블록을 반환합니다.

<h3 id="spawnedprocess">
  `SpawnedProcess`
</h3>

사용자 정의 프로세스 생성을 위한 인터페이스 (`spawnClaudeCodeProcess` 옵션과 함께 사용). `ChildProcess`는 이미 이 인터페이스를 만족합니다.

```typescript theme={null}
interface SpawnedProcess {
  stdin: Writable;
  stdout: Readable;
  readonly killed: boolean;
  readonly exitCode: number | null;
  kill(signal: NodeJS.Signals): boolean;
  on(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  on(event: "error", listener: (error: Error) => void): void;
  once(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  once(event: "error", listener: (error: Error) => void): void;
  off(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  off(event: "error", listener: (error: Error) => void): void;
}
```

<h3 id="spawnoptions">
  `SpawnOptions`
</h3>

사용자 정의 생성 함수에 전달된 옵션입니다.

```typescript theme={null}
interface SpawnOptions {
  command: string;
  args: string[];
  cwd?: string;
  env: Record<string, string | undefined>;
  signal: AbortSignal;
}
```

<Note>
  `signal` 필드는 프로세스를 종료할 시기를 생성 함수에 알립니다. 이를 Node의 `spawn()`에 `signal` 옵션으로 전달하거나, VM 또는 컨테이너 종료 핸들러에 전달하세요.

  이 신호는 [`Options.abortController`](#options)가 중단되는 순간 즉시 발생하지 않습니다. SDK는 먼저 프로세스의 stdin을 닫고 CLI가 깔끔하게 종료될 수 있도록 약 2초를 기다린 후, 이 신호를 중단합니다. 호출자가 중단되는 순간 즉시 반응하려면, 생성 함수가 인클로징 스코프에서 참조할 수 있는 자신의 `Options.abortController.signal`을 수신하세요.
</Note>

<h3 id="mcpsetserversresult">
  `McpSetServersResult`
</h3>

`setMcpServers()` 작업의 결과입니다.

```typescript theme={null}
type McpSetServersResult = {
  added: string[];
  removed: string[];
  errors: Record<string, string>;
};
```

`setMcpServers()`를 호출할 때, Claude Code는 다음 규칙을 적용합니다:

* **호출이 이름을 지정하지 않는 서버**: Claude Code는 플러그인 제공 서버를 계속 실행합니다. Agent SDK v0.3.210 이상이 필요합니다.
* **호출이 이름을 지정하는 서버**: CLI가 시작 시 시작한 기본 제공 서버를 제외하고, Claude Code는 구성이 전달한 것과 다를 때만 실행 중인 서버를 교체합니다.
* **CLI가 시작 시 시작한 기본 제공 서버**: 호출이 하나를 이름을 지정하면, Claude Code는 해당 항목을 삭제하고 `errors`에서 보고합니다.

약속은 새로 추가된 stdio, HTTP 및 SSE 서버가 연결되거나 실패한 후 해결되므로, 연결된 서버의 도구는 다음 턴에서 사용 가능합니다.

`added`는 Claude Code가 추가하거나 교체한 서버를 나열하며, 연결 여부와 관계없이 나열합니다. 연결 실패한 서버는 `added`와 `errors` 모두에 나타나며, `errors` 아래에 실패 텍스트가 있고 [`mcpServerStatus()`](#methods)에서 `failed` 행이 있습니다. Claude Code v2.1.257 이전에는 연결 시도가 던진 서버는 `errors` 아래에만 보고되었습니다.

<h3 id="rewindfilesresult">
  `RewindFilesResult`
</h3>

`rewindFiles()` 작업의 결과입니다.

```typescript theme={null}
type RewindFilesResult = {
  canRewind: boolean;
  error?: string;
  filesChanged?: string[];
  insertions?: number;
  deletions?: number;
  skippedLinks?: number;
};
```

`skippedLinks`는 되감기가 링크 안전을 위해 복원하거나 삭제하기를 거부한 추적된 경로의 개수를 계산합니다. 추적된 경로의 심볼릭 링크, 하드 링크 또는 기타 일반 파일이 아닌 파일, 체크포인트가 취해졌을 때 가리키던 위치로 더 이상 확인되지 않는 부모 디렉토리, 또는 안전하게 읽을 수 없는 백업입니다. 필드는 Claude Code v2.1.216 이상이 필요합니다. `rewindFiles(userMessageId, { dryRun: true })`를 사용한 미리보기 호출은 이를 설정하지 않습니다.

<h3 id="sdkstatusmessage">
  `SDKStatusMessage`
</h3>

상태 업데이트 메시지 (예: 압축).

```typescript theme={null}
type SDKStatusMessage = {
  type: "system";
  subtype: "status";
  status: "compacting" | null;
  permissionMode?: PermissionMode;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktasknotificationmessage">
  `SDKTaskNotificationMessage`
</h3>

백그라운드 작업이 완료, 실패 또는 중지될 때의 알림입니다. 백그라운드 작업에는 `run_in_background` Bash 명령, [Monitor](#monitor) 감시 및 백그라운드 서브에이전트가 포함됩니다. `ambient` 필드는 [`SDKTaskStartedMessage`](#sdktaskstartedmessage)를 참조하세요. 이는 이를 정의하고 해당 버전 요구 사항을 정의합니다.

```typescript theme={null}
type SDKTaskNotificationMessage = {
  type: "system";
  subtype: "task_notification";
  task_id: string;
  tool_use_id?: string;
  status: "completed" | "failed" | "stopped";
  output_file: string;
  summary: string;
  ambient?: boolean;
  usage?: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  resource_links?: SDKMcpResourceLink[];
  uuid: UUID;
  session_id: string;
};
```

Claude Code가 [긴 MCP 도구 호출을 백그라운드로 이동](/docs/ko/mcp#automatic-backgrounding-of-long-tool-calls)할 때, 해당 호출에 대한 `tool_result` 블록은 자리 표시자만 보유하고 호출의 실제 결과는 이 알림에서 도착합니다. `tool_use_id`를 사용하여 알림을 호출과 일치시킵니다.&#x20;
`completed` 알림에서 `resource_links`는 도구가 [`SDKMcpResourceLink`](#sdkmcpresourcelink) 항목으로 참조로 반환한 파일을 나열하며, [`tool_use_result.resourceLinks`](#sdkusermessage)와 동일한 50개 링크 및 64 KiB 제한이 있습니다. Claude Code는 결과에 링크가 없을 때 `resource_links`를 생략하고 MCP 도구 호출이 아닌 작업에 대한 알림에서 생략합니다. `resource_links`는 Agent SDK v0.3.257 이상이 필요합니다.

Claude Code는 [`scheduled-trigger` 서브종류](#task-notification-subkinds)로 스탬프된 전달을 제외한 모든 작업 알림 앞에 공지를 추가합니다. 이들은 대신 할당된 작업 프레이밍을 전달합니다. 공지는 인간 입력이 발생하지 않았으므로 모델이 알림을 사용자 지시 또는 승인으로 취급하지 않는다고 명시합니다.

작업 알림 턴을 감지하려면, [`SDKUserMessage`](#sdkusermessage) 또는 [`SDKResultMessage`](#sdkresultmessage)의 `origin.kind === "task-notification"`을 확인하세요. 공지 텍스트와 일치하는 대신 필요한 경우 동일한 필드에서 `subkind`를 읽으세요. v2.1.205 이전에 Claude Code는 세션이 유휴 상태일 때 도착한 알림에서 공지를 생략했습니다.

<h3 id="sdktoolusesummarymessage">
  `SDKToolUseSummaryMessage`
</h3>

대화에서의 도구 사용 요약입니다.

```typescript theme={null}
type SDKToolUseSummaryMessage = {
  type: "tool_use_summary";
  summary: string;
  preceding_tool_use_ids: string[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkhookstartedmessage">
  `SDKHookStartedMessage`
</h3>

훅이 실행을 시작할 때 내보내집니다.

Claude Code는 이 메시지, [`SDKHookProgressMessage`](#sdkhookprogressmessage) 및 [`SDKHookResponseMessage`](#sdkhookresponsemessage)를 메시지 스트림에 즉시 전달합니다. 여기에는 `SessionStart` 또는 `Setup` 훅이 세션 시작 중에 여전히 실행 중인 동안도 포함됩니다. Claude Code v2.1.169부터 v2.1.203까지는 `SessionStart` 또는 `Setup` 훅이 완료된 후 이러한 메시지를 한 배치로 전달했습니다. v2.1.204는 라이브 전달을 복원했습니다.

```typescript theme={null}
type SDKHookStartedMessage = {
  type: "system";
  subtype: "hook_started";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkhookprogressmessage">
  `SDKHookProgressMessage`
</h3>

훅이 실행 중일 때 stdout/stderr 출력과 함께 내보내집니다.

```typescript theme={null}
type SDKHookProgressMessage = {
  type: "system";
  subtype: "hook_progress";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  stdout: string;
  stderr: string;
  output: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkhookresponsemessage">
  `SDKHookResponseMessage`
</h3>

훅이 실행을 완료할 때 내보내집니다.

```typescript theme={null}
type SDKHookResponseMessage = {
  type: "system";
  subtype: "hook_response";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  output: string;
  stdout: string;
  stderr: string;
  exit_code?: number;
  outcome: "success" | "error" | "cancelled";
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktoolprogressmessage">
  `SDKToolProgressMessage`
</h3>

도구가 실행 중일 때 진행 상황을 나타내기 위해 주기적으로 내보내집니다.

```typescript theme={null}
type SDKToolProgressMessage = {
  type: "tool_progress";
  tool_use_id: string;
  tool_name: string;
  parent_tool_use_id: string | null;
  elapsed_time_seconds: number;
  task_id?: string;
  heartbeat?: boolean;
  subagent_type?: string;
  subagent_retry?: {
    agent_id: string;
    attempt: number;
    max_retries: number;
    retry_delay_ms: number;
    error_status: number | null;
    error_category: string;
  };
  uuid: UUID;
  session_id: string;
};
```

도구 호출이 주 대화에서 실행되는 동안, Claude Code는 `heartbeat: true`를 사용하여 30초마다 `tool_progress` 메시지를 내보냅니다. 각 하트비트는 도구 이름과 경과 초를 전달하므로, 장시간 실행되는 호출을 정지된 세션과 구별할 수 있습니다. Claude Code는 서브에이전트 내부의 도구 호출에 대해 하트비트를 내보내지 않습니다. `heartbeat` 필드는 Agent SDK v0.3.214 이상이 필요합니다. v2.1.257 이전에 Claude Code는 포그라운드 Agent 도구 호출에 대해서도 하트비트를 내보내지 않았습니다.

하트비트가 아닌 Agent 도구에 대한 `tool_progress` 메시지에서 `subagent_type`은 실행 중인 서브에이전트 타입의 이름을 지정합니다. 예를 들어 `general-purpose`. `subagent_retry`는 해당 서브에이전트가 API 오류 백오프(예: 속도 제한 또는 과부하)를 기다리는 동안 나타나며, 재시도 시도당 하나의 메시지가 있습니다. 두 필드 모두 Agent SDK v0.3.214 이상이 필요합니다.

`subagent_retry`에서 재시도 표시기를 렌더링하려면:

* `parent_tool_use_id`로 표시기를 추적합니다. 이는 서브에이전트당 고유합니다. `tool_use_id`는 하나의 어시스턴트 턴에서 병렬 서브에이전트에 의해 공유되므로, 이를 통해 추적하면 한 서브에이전트의 업데이트가 다른 서브에이전트의 표시기를 지울 수 있습니다.
* 동일한 `parent_tool_use_id`에 대한 나중의 `tool_progress`가 `subagent_retry` 또는 `heartbeat: true` 없이 도착하거나 도구의 결과 메시지가 도착할 때 표시기를 지웁니다. `heartbeat: true`가 있는 프레임은 생동성만 보고하므로, 하나가 도착할 때 표시기를 유지합니다. `attempt`는 지속적인 재시도 아래에서 `max_retries`를 초과할 수 있으므로, 카운터에서 지우기를 파생하지 마세요.
* `error_category`를 자신의 메시지 텍스트를 선택하기 위한 토큰으로 취급하고, 표시 텍스트로 취급하지 마세요. 값은 `rate_limit`, `overloaded`, `authentication_failed`, `server_error`, `cloud_credential_error` 및 `unknown`입니다. 인식하지 못하는 값을 `unknown`을 처리하는 방식으로 처리하세요. 나중 릴리스는 값을 추가할 수 있습니다.

<h3 id="sdkauthstatusmessage">
  `SDKAuthStatusMessage`
</h3>

인증 흐름 중에 내보내집니다.

```typescript theme={null}
type SDKAuthStatusMessage = {
  type: "auth_status";
  isAuthenticating: boolean;
  output: string[];
  error?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktaskstartedmessage">
  `SDKTaskStartedMessage`
</h3>

작업이 시작될 때 내보내집니다. `task_type` 필드는 Bash 명령 및 [Monitor](#monitor) 감시의 경우 `"local_bash"`, 서브에이전트의 경우 `"local_agent"` 또는 `"remote_agent"`입니다.

```typescript theme={null}
type SDKTaskStartedMessage = {
  type: "system";
  subtype: "task_started";
  task_id: string;
  tool_use_id?: string;
  description: string;
  task_type?: string;
  is_backgrounded?: boolean;
  spawn_depth?: number;
  ambient?: boolean;
  uuid: UUID;
  session_id: string;
};
```

`ambient`는 세션의 작업의 일부가 아닌 작업(예: Claude Code가 자신의 작업을 위해 실행하는 작업)에 대해 `true`입니다. 라이브 업데이트 감시자도 주변입니다. 여기에는 사용자가 요청한 감시자가 포함됩니다. 주변 작업을 활동 표시기에서 제외합니다. 필드는 Agent SDK v0.3.247 이상이 필요합니다.

`ambient`는 또한 [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) 및 [`SDKBackgroundTasksChangedMessage`](#sdkbackgroundtaskschangedmessage) 항목에 나타납니다.

`is_backgrounded` 및 `spawn_depth`는 Claude Code가 작업을 시작한 방식을 설명합니다. 두 필드 모두 Agent SDK v0.3.238 이상이 필요합니다.

* `is_backgrounded`: Claude Code는 `"local_agent"` 및 `"local_bash"` 작업에서 이를 설정합니다. `true`는 작업이 백그라운드에서 실행됨을 의미합니다. `false`는 작업이 포그라운드에서 실행되고, 이를 시작한 도구 호출은 작업이 완료되거나 백그라운드로 이동할 때까지 차단된 상태로 유지됨을 의미합니다.
* `spawn_depth`: Claude Code는 `"local_agent"` 작업에서만 이를 설정합니다. 주 스레드가 생성한 서브에이전트는 깊이 `1`을 가집니다. 깊이 `1` 서브에이전트가 생성한 서브에이전트는 깊이 `2`를 가지며, 이런 식으로 계속됩니다.

[재개된 서브에이전트](/docs/ko/agent-sdk/subagents#resume-subagents)는 항상 `is_backgrounded: true`를 보고합니다. Claude Code는 모든 재개된 서브에이전트를 백그라운드에서 실행하기 때문입니다. 포그라운드 작업이 나중에 백그라운드로 이동할 때, Claude Code는 새로운 `is_backgrounded` 값을 [`task_updated`](#sdktaskupdatedmessage) 메시지에서 보고하기보다는 두 번째 `task_started`를 전송합니다.

<h3 id="sdktaskprogressmessage">
  `SDKTaskProgressMessage`
</h3>

서브에이전트 또는 백그라운드 작업이 실행 중일 때 주기적으로 내보내집니다. `summary` 필드는 [`agentProgressSummaries`](#options)가 활성화되었을 때만 채워집니다.

```typescript theme={null}
type SDKTaskProgressMessage = {
  type: "system";
  subtype: "task_progress";
  task_id: string;
  tool_use_id?: string;
  description: string;
  subagent_type?: string;
  usage: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  last_tool_name?: string;
  summary?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktaskupdatedmessage">
  `SDKTaskUpdatedMessage`
</h3>

백그라운드 작업의 상태가 변경될 때 내보내집니다. 예를 들어 `running`에서 `completed`로 전환될 때입니다. `patch`를 `task_id`로 키가 지정된 로컬 작업 맵에 병합합니다. `end_time` 필드는 Unix epoch 타임스탬프(밀리초)이며 `Date.now()`와 비교할 수 있습니다.

```typescript theme={null}
type SDKTaskUpdatedMessage = {
  type: "system";
  subtype: "task_updated";
  task_id: string;
  patch: {
    status?: "pending" | "running" | "completed" | "failed" | "killed";
    description?: string;
    end_time?: number;
    total_paused_ms?: number;
    error?: string;
    is_backgrounded?: boolean;
  };
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkbackgroundtaskschangedmessage">
  `SDKBackgroundTasksChangedMessage`
</h3>

라이브 백그라운드 작업 집합이 변경될 때마다 내보내집니다. 작업이 시작되거나, 완료되거나, 종료되거나, 포그라운드 에이전트가 백그라운드로 전환되거나, 작업의 `description` 또는 `ambient` 필드가 변경될 때입니다.

`tasks` 배열은 전체 라이브 집합입니다. `task_started` 및 `task_notification` 이벤트를 쌍으로 지정하는 대신 각 페이로드로 캐시된 집합을 바꾸므로, 다음 멤버십 변경이 놓친 이벤트를 수정합니다.

이러한 작업별 이벤트에 대한 순서는 지정되지 않으므로, 두 스트림을 상관시키지 마세요.

시작 시 아무것도 내보내지지 않습니다. 세션의 CLI 프로세스가 시작되거나 다시 시작될 때마다 빈 집합으로 재설정하고 다음 멤버십 변경이 다시 채우도록 하세요.

실행 중인 세션에 반복된 `initialize` 제어 요청을 전송할 때(예: 전송 간격 후 [`reinitialize()`](#query-object) 사용), Claude Code는 응답 뒤에 현재 라이브 집합의 스냅샷을 따릅니다. 비어 있을 때도 마찬가지입니다. 재연결하는 호스트는 다음 멤버십 변경을 기다리지 않고 실행 중인 것을 알 수 있습니다. Agent SDK v0.3.239 이전에 Claude Code는 반복된 `initialize` 후 스냅샷을 전송하지 않았습니다.

Claude Code v2.1.203 이상이 필요합니다.

```typescript theme={null}
type SDKBackgroundTasksChangedMessage = {
  type: "system";
  subtype: "background_tasks_changed";
  tasks: {
    task_id: string;
    task_type: string;
    description: string;
    ambient?: boolean;
  }[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkthinkingtokensmessage">
  `SDKThinkingTokensMessage`
</h3>

Claude가 사고 블록을 생성하는 동안 내보내집니다. 여기에는 지금까지 생성된 사고 토큰의 실행 추정치가 포함됩니다. `estimated_tokens`는 현재 사고 블록의 실행 합계이고 `estimated_tokens_delta`는 이 프레임에서 전달된 증분입니다. 진행 상황 표시에 사용하세요.

모델 또는 제공자가 분류를 보고할 때, 최상위 에이전트 루프의 최종 개수는 결과 메시지의 [`usage.output_tokens_details.thinking_tokens`](#usage)입니다. 이는 [서브에이전트 토큰을 포함하지 않습니다](/docs/ko/agent-sdk/cost-tracking#get-the-total-cost-of-a-query).

Claude Code v2.1.153 이상이 필요합니다.

```typescript theme={null}
type SDKThinkingTokensMessage = {
  type: "system";
  subtype: "thinking_tokens";
  estimated_tokens: number;
  estimated_tokens_delta: number;
  user_message_uuid?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkfilespersistedevent">
  `SDKFilesPersistedEvent`
</h3>

파일 체크포인트가 디스크에 지속될 때 내보내집니다.

```typescript theme={null}
type SDKFilesPersistedEvent = {
  type: "system";
  subtype: "files_persisted";
  files: { filename: string; file_id: string }[];
  failed: { filename: string; error: string }[];
  processed_at: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkratelimitevent">
  `SDKRateLimitEvent`
</h3>

세션이 속도 제한을 만날 때 내보내집니다.

```typescript theme={null}
type SDKRateLimitEvent = {
  type: "rate_limit_event";
  rate_limit_info: {
    status: "allowed" | "allowed_warning" | "rejected";
    resetsAt?: number;
    utilization?: number;
    errorCode?: "credits_required";
    canUserPurchaseCredits?: boolean;
    hasChargeableSavedPaymentMethod?: boolean;
  };
  uuid: UUID;
  session_id: string;
};
```

`errorCode`가 `"credits_required"`일 때, 거부는 포함된 사용량이 소진된 claude.ai 구독에서 발생하며, 사용자가 사용 크레딧을 구매할 때까지 세션을 계속할 수 없습니다. `canUserPurchaseCredits`는 인증된 사용자가 계정에 대한 크레딧을 구매할 수 있는지 여부를 나타내고, `hasChargeableSavedPaymentMethod`는 저장된 결제 방법이 파일에 있는지 여부를 나타냅니다. 세 필드 모두 크레딧 필수 거부가 아닌 속도 제한 이벤트에서는 없습니다. Claude Code v2.1.181 이상이 필요합니다.

<h3 id="sdklocalcommandoutputmessage">
  `SDKLocalCommandOutputMessage`
</h3>

Claude Code는 이 메시지 타입을 내보내지 않습니다. `/context` 또는 `/usage`와 같은 명령을 프롬프트로 전송할 때, 해당 출력은 [`SDKAssistantMessage`](#sdkassistantmessage)로 도착합니다.

```typescript theme={null}
type SDKLocalCommandOutputMessage = {
  type: "system";
  subtype: "local_command_output";
  content: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkcommandschangedmessage">
  `SDKCommandsChangedMessage`
</h3>

사용 가능한 명령 집합이 세션 중간에 변경될 때 내보내집니다. 예를 들어 에이전트가 하위 디렉토리에 들어갈 때 스킬이 발견될 때입니다. `commands` 배열은 전체 업데이트된 목록이므로, 캐시된 명령 목록을 이 페이로드로 바꾸세요.&#x20;
이 메시지 후 [`supportedCommands()`](#query-object)를 호출하면 동일한 업데이트된 목록을 반환합니다. 메서드는 최신 푸시를 추적하기 때문입니다. 이는 Agent SDK v0.3.216 이상이 필요합니다. 이전 SDK 버전에서 `supportedCommands()`는 초기화 시 캡처된 스냅샷을 반환하며 세션 중간 변경을 반영하지 않습니다.

```typescript theme={null}
type SDKCommandsChangedMessage = {
  type: "system";
  subtype: "commands_changed";
  commands: SlashCommand[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkpromptsuggestionmessage">
  `SDKPromptSuggestionMessage`
</h3>

[`promptSuggestions`](#options)가 활성화되었을 때 턴 후에 내보내집니다. Claude Code가 해당 턴에 대한 제안을 생성했습니다. 예측된 다음 사용자 프롬프트를 포함합니다. 제안을 받지 않는 턴은 [Claude Code가 제안을 건너뛸 때](/docs/ko/interactive-mode#when-claude-code-skips-suggestions)를 참조하세요.

```typescript theme={null}
type SDKPromptSuggestionMessage = {
  type: "prompt_suggestion";
  suggestion: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkconversationresetmessage">
  `SDKConversationResetMessage`
</h3>

세션의 대화가 세션을 종료하지 않고 바뀔 때 내보내집니다. `query()` 호출에서 `/clear` 및 해당 별칭만 이 메시지를 생성합니다. `new_conversation_id` 아래에 빈 트랜스크립트를 마운트하고 캐시된 세션 제목을 버리세요.

```typescript theme={null}
type SDKConversationResetMessage = {
  type: "conversation_reset";
  new_conversation_id: UUID;
  uuid: UUID;
  session_id: string;
};
```

SDK의 게시된 타이핑은 Claude Code v2.1.203 이상에서 `SDKConversationResetMessage`를 선언합니다. v2.1.203 이전에는 `SDKMessage`가 타입을 선언하지 않고 참조했으므로, `skipLibCheck`가 비활성화되었을 때 `type === "conversation_reset"`에 대한 좁혀지기가 타입 검사에 실패했습니다.

<h3 id="aborterror">
  `AbortError`
</h3>

중단 작업을 위한 사용자 정의 오류 클래스입니다.

```typescript theme={null}
class AbortError extends Error {}
```

SDK의 타입 API에서 `AbortError`는 유일한 오류 클래스입니다. Claude Code 프로세스 종료 또는 시작 실패와 같은 다른 실패는 일치할 SDK 클래스가 없는 오류로 메시지 반복을 거부합니다. [문제 해결](/docs/ko/agent-sdk/troubleshooting)은 각각의 원인과 수정 사항과 함께 메시지로 이러한 오류를 키합니다.

<h2 id="sandbox-configuration">
  샌드박스 구성
</h2>

<h3 id="sandboxsettings">
  `SandboxSettings`
</h3>

샌드박스 동작의 구성입니다. 이를 사용하여 명령 샌드박싱을 활성화하고 프로그래밍 방식으로 네트워크 제한을 구성합니다.

```typescript theme={null}
type SandboxSettings = {
  enabled?: boolean;
  failIfUnavailable?: boolean;
  autoAllowBashIfSandboxed?: boolean;
  excludedCommands?: string[];
  allowUnsandboxedCommands?: boolean;
  network?: SandboxNetworkConfig;
  filesystem?: SandboxFilesystemConfig;
  ignoreViolations?: Record<string, string[]>;
  enableWeakerNestedSandbox?: boolean;
  ripgrep?: { command: string; args?: string[] };
};
```

| 속성                          | 타입                                                    | 기본값         | 설명                                                                                                                                                                              |
| :-------------------------- | :---------------------------------------------------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `enabled`                   | `boolean`                                             | `false`     | 명령 실행을 위한 샌드박스 모드 활성화                                                                                                                                                           |
| `failIfUnavailable`         | `boolean`                                             | `true`      | `enabled`가 `true`이지만 샌드박스를 시작할 수 없는 경우 시작 시 중지합니다. stderr에 경고와 함께 샌드박스되지 않은 실행으로 폴백하려면 `false`로 설정합니다                                                                           |
| `autoAllowBashIfSandboxed`  | `boolean`                                             | `true`      | 샌드박스가 활성화되었을 때 Bash 명령 자동 승인                                                                                                                                                    |
| `excludedCommands`          | `string[]`                                            | `[]`        | 샌드박스 제한을 무시하는 명령입니다(예: `['docker *']`). 이들은 모델 개입 없이 자동으로 샌드박스되지 않은 상태로 실행됩니다. [`sandbox.excludedCommands`](/docs/ko/settings-reference#sandbox-excludedcommands)는 항목이 적용되는 시기를 다룹니다 |
| `allowUnsandboxedCommands`  | `boolean`                                             | `true`      | 모델이 샌드박스 외부에서 명령을 실행하도록 요청하도록 허용합니다. `true`일 때 모델은 도구 입력에서 `dangerouslyDisableSandbox`를 설정할 수 있으며, 이는 [권한 시스템](#permissions-fallback-for-unsandboxed-commands)으로 폴백됩니다          |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `undefined` | 네트워크 특정 샌드박스 구성                                                                                                                                                                 |
| `filesystem`                | [`SandboxFilesystemConfig`](#sandboxfilesystemconfig) | `undefined` | 읽기/쓰기 제한을 위한 파일 시스템 특정 샌드박스 구성                                                                                                                                                  |
| `ignoreViolations`          | `Record<string, string[]>`                            | `undefined` | 명령 부분 문자열 또는 모든 명령에 대해 `*`를 무시할 위반 텍스트의 부분 문자열에 매핑합니다 (예: `{ "*": ['/etc/hosts'] }`). [`sandbox.ignoreViolations`](/docs/ko/settings-reference#sandbox-ignoreviolations)을 참조합니다      |
| `enableWeakerNestedSandbox` | `boolean`                                             | `false`     | 호환성을 위해 더 약한 중첩 샌드박스 활성화                                                                                                                                                        |
| `ripgrep`                   | `{ command: string; args?: string[] }`                | `undefined` | 샌드박스 환경을 위한 사용자 정의 ripgrep 바이너리 구성                                                                                                                                              |

<Note>
  샌드박스는 플랫폼 지원에 따라 다르며, Linux에서는 `bubblewrap` 및 `socat`과 같은 도구가 필요합니다. `enabled`가 `true`이고 샌드박스를 시작할 수 없는 경우 `query()`는 `subtype: "error_during_execution"`이 있는 `result` 메시지를 보고하고 `errors`에 이유를 표시합니다. 단일 메시지 `query()` 호출의 경우 SDK는 해당 오류 결과를 생성한 후 예외를 발생시키므로 루프를 try 블록으로 래핑하여 이를 지나 계속 진행합니다. 오류 계약에 대해서는 [결과 처리](/docs/ko/agent-sdk/agent-loop#handle-the-result)를 참조합니다.

  대신 샌드박스되지 않은 상태로 실행하려면 `failIfUnavailable: false`를 설정합니다.
</Note>

<h4 id="example-usage">
  사용 예제
</h4>

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

try {
  for await (const message of query({
    prompt: "Build and test my project",
    options: {
      sandbox: {
        enabled: true,
        autoAllowBashIfSandboxed: true,
        network: {
          allowLocalBinding: true
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
} catch (error) {
  // 단일 쿼리 query()는 오류 결과를 생성한 후 예외를 발생시킵니다.
  // 예를 들어 샌드박스를 시작할 수 없는 경우 (failIfUnavailable은 기본값이 true입니다).
  console.log(`Session ended with an error: ${error}`);
}
```

<Warning>
  **Unix 소켓 보안:** `allowUnixSockets` 옵션은 샌드박스 외부에 도달하는 시스템 서비스에 대한 액세스를 부여할 수 있습니다. 예를 들어 `/var/run/docker.sock`을 허용하면 Docker API를 통해 전체 호스트 시스템 액세스를 효과적으로 부여하여 샌드박스 격리를 무시합니다. 엄격히 필요한 Unix 소켓만 허용하고 각각의 보안 영향을 이해합니다.
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

샌드박스 모드를 위한 네트워크 특정 구성입니다. 이러한 설정은 부모 [`SandboxSettings`](#sandboxsettings)에서 `enabled`가 `true`일 때 샌드박스된 Bash 명령에 적용됩니다. 이들은 [권한 규칙](/docs/ko/permissions#webfetch)을 대신 사용하는 WebFetch 도구를 제한하지 않습니다.

```typescript theme={null}
type SandboxNetworkConfig = {
  allowedDomains?: string[];
  deniedDomains?: string[];
  strictAllowlist?: boolean;
  allowManagedDomainsOnly?: boolean;
  allowLocalBinding?: boolean;
  allowUnixSockets?: string[];
  allowAllUnixSockets?: boolean;
  httpProxyPort?: number;
  socksProxyPort?: number;
};
```

| 속성                        | 타입         | 기본값         | 설명                                                                                                                                                                                                                                                 |
| :------------------------ | :--------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedDomains`          | `string[]` | `[]`        | 샌드박스된 프로세스가 액세스할 수 있는 도메인 이름                                                                                                                                                                                                                       |
| `deniedDomains`           | `string[]` | `[]`        | 샌드박스된 프로세스가 액세스할 수 없는 도메인 이름입니다. `allowedDomains`보다 우선합니다                                                                                                                                                                                          |
| `strictAllowlist`         | `boolean`  | `false`     | [네트워크 허용 목록](/docs/ko/sandboxing#network-isolation) 외부의 호스트에 대한 샌드박스된 명령 액세스를 거부합니다. 프롬프트 대신 거부합니다. 샌드박스된 명령에만 적용됩니다. WebFetch와 같은 프로세스 내 도구는 이에 의해 제한되지 않습니다. 사용자, 관리형 또는 CLI `--settings` 설정에서만 적용됩니다. 프로젝트 설정은 무시됩니다. Claude Code v2.1.219 이상이 필요합니다 |
| `allowManagedDomainsOnly` | `boolean`  | `false`     | 관리형 설정 전용입니다. [관리형 설정](/docs/ko/managed-settings)에서 설정되었을 때 관리형 설정의 `allowedDomains` 항목 및 관리형 설정의 `WebFetch(domain:...)` 허용 규칙만 적용되며 사용자, 프로젝트 또는 로컬 설정의 허용 항목은 무시됩니다. SDK 옵션을 통해 설정되었을 때는 효과가 없습니다                                                     |
| `allowLocalBinding`       | `boolean`  | `false`     | 프로세스가 로컬 포트에 바인딩하도록 허용합니다 (예: 개발 서버의 경우)                                                                                                                                                                                                           |
| `allowUnixSockets`        | `string[]` | `[]`        | 프로세스가 액세스할 수 있는 Unix 소켓 경로 (예: Docker 소켓)                                                                                                                                                                                                          |
| `allowAllUnixSockets`     | `boolean`  | `false`     | 모든 Unix 소켓에 대한 액세스 허용                                                                                                                                                                                                                              |
| `httpProxyPort`           | `number`   | `undefined` | 네트워크 요청을 위한 HTTP 프록시 포트                                                                                                                                                                                                                            |
| `socksProxyPort`          | `number`   | `undefined` | 네트워크 요청을 위한 SOCKS 프록시 포트                                                                                                                                                                                                                           |

<Note>
  기본 제공 샌드박스 프록시는 요청된 호스트명을 기반으로 `allowedDomains`를 적용하며 TLS 트래픽을 종료하거나 검사하지 않으므로 [도메인 프론팅](https://en.wikipedia.org/wiki/Domain_fronting)과 같은 기술이 이를 우회할 수 있습니다. 자세한 내용은 [샌드박싱 보안 제한 사항](/docs/ko/sandboxing#security-limitations)을 참조하고 TLS 종료 프록시 구성에 대해서는 [안전한 배포](/docs/ko/agent-sdk/secure-deployment#traffic-forwarding)를 참조합니다.
</Note>

<h3 id="sandboxfilesystemconfig">
  `SandboxFilesystemConfig`
</h3>

샌드박스 모드를 위한 파일 시스템 특정 구성입니다.

```typescript theme={null}
type SandboxFilesystemConfig = {
  allowWrite?: string[];
  denyWrite?: string[];
  denyRead?: string[];
};
```

| 속성           | 타입         | 기본값  | 설명                   |
| :----------- | :--------- | :--- | :------------------- |
| `allowWrite` | `string[]` | `[]` | 쓰기 액세스를 허용할 파일 경로 패턴 |
| `denyWrite`  | `string[]` | `[]` | 쓰기 액세스를 거부할 파일 경로 패턴 |
| `denyRead`   | `string[]` | `[]` | 읽기 액세스를 거부할 파일 경로 패턴 |

<h3 id="permissions-fallback-for-unsandboxed-commands">
  샌드박스되지 않은 명령에 대한 권한 폴백
</h3>

`allowUnsandboxedCommands`가 활성화되었을 때 모델은 도구 입력에서 `dangerouslyDisableSandbox: true`를 설정하여 샌드박스 외부에서 명령을 실행하도록 요청할 수 있습니다. 이러한 요청은 기존 권한 시스템으로 폴백되므로 `canUseTool` 핸들러가 호출되어 사용자 정의 인증 로직을 구현할 수 있습니다.

`excludedCommands` 항목은 대신 샌드박스를 자동으로 무시하며 모델 개입이 없습니다. [`sandbox.excludedCommands`](/docs/ko/settings-reference#sandbox-excludedcommands)는 항목이 적용되는 시기를 다룹니다.

아래 예제에서 `isCommandAuthorized`는 사용자가 정의하는 인증 확인을 나타냅니다.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Deploy my application",
  options: {
    sandbox: {
      enabled: true,
      allowUnsandboxedCommands: true // 모델이 샌드박스되지 않은 실행을 요청할 수 있습니다
    },
    permissionMode: "default",
    canUseTool: async (tool, input) => {
      // 모델이 샌드박스를 무시하도록 요청하는지 확인합니다
      if (tool === "Bash" && input.dangerouslyDisableSandbox) {
        // 모델이 이 명령을 샌드박스 외부에서 실행하도록 요청합니다
        console.log(`Unsandboxed command requested: ${input.command}`);

        if (isCommandAuthorized(input.command)) {
          return { behavior: "allow" as const, updatedInput: input };
        }
        return {
          behavior: "deny" as const,
          message: "Command not authorized for unsandboxed execution"
        };
      }
      return { behavior: "allow" as const, updatedInput: input };
    }
  }
})) {
  if ("result" in message) console.log(message.result);
}
```

<Warning>
  `dangerouslyDisableSandbox: true`로 실행되는 명령은 전체 시스템 액세스 권한이 있습니다. `canUseTool` 핸들러가 이러한 요청을 신중하게 검증하는지 확인합니다.

  `permissionMode`가 `bypassPermissions`로 설정되고 `allowUnsandboxedCommands`가 활성화되면 모델은 [작업 자동 승인 안 함 모드](/docs/ko/permission-modes#actions-no-mode-auto-approves)를 제외하고 승인 프롬프트 없이 샌드박스 외부에서 명령을 자율적으로 실행할 수 있습니다. 이 조합은 모델이 샌드박스 격리를 조용히 탈출하도록 효과적으로 허용합니다.
</Warning>

<h2 id="see-also">
  참고 항목
</h2>

* [SDK 개요](/docs/ko/agent-sdk/overview) - 일반 SDK 개념
* [Python SDK 참조](/docs/ko/agent-sdk/python) - Python SDK 문서
* [CLI 참조](/docs/ko/cli-reference) - 명령줄 인터페이스
* [일반적인 워크플로우](/docs/ko/common-workflows) - 단계별 가이드
