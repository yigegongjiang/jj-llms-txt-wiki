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
  SDK는 선택적 종속성으로 플랫폼용 네이티브 Claude Code 바이너리를 번들로 제공합니다(예: `@anthropic-ai/claude-agent-sdk-darwin-arm64`). Claude Code를 별도로 설치할 필요가 없습니다. 패키지 관리자가 선택적 종속성을 건너뛰면 SDK는 `Native CLI binary for <platform> not found` 오류를 발생시킵니다. 이 경우 [`pathToClaudeCodeExecutable`](#options)을 별도로 설치된 `claude` 바이너리로 설정하세요.
</Note>

<h3 id="compile-to-a-single-executable">
  단일 실행 파일로 컴파일
</h3>

`bun build --compile`을 사용하여 애플리케이션을 단일 파일 실행 파일로 컴파일하면 SDK는 런타임에 번들된 CLI 바이너리를 확인할 수 없습니다. `require.resolve`는 컴파일된 실행 파일의 `$bunfs` 가상 파일 시스템 내에서 작동하지 않으므로 SDK는 `Native CLI binary for <platform> not found` 오류를 발생시킵니다.

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

Claude Code와 상호작용하기 위한 주요 함수입니다. 메시지가 도착할 때 스트리밍하는 비동기 생성기를 만듭니다.

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

| 매개변수      | 타입                                                               | 설명                                      |
| :-------- | :--------------------------------------------------------------- | :-------------------------------------- |
| `prompt`  | `string \| AsyncIterable<`[`SDKUserMessage`](#sdkusermessage)`>` | 문자열 또는 스트리밍 모드용 비동기 반복 가능 객체로서의 입력 프롬프트 |
| `options` | [`Options`](#options)                                            | 선택적 구성 객체 (아래 Options 타입 참조)            |

<h4 id="returns">
  반환값
</h4>

[`Query`](#query-object) 객체를 반환하며, 이는 추가 메서드를 포함하는 `AsyncGenerator<`[`SDKMessage`](#sdkmessage)`, void>`를 확장합니다.

<h3 id="startup">
  `startup()`
</h3>

프롬프트를 사용할 수 있기 전에 CLI 서브프로세스를 생성하고 초기화 핸드셰이크를 완료하여 미리 준비합니다. 반환된 [`WarmQuery`](#warmquery) 핸들은 나중에 프롬프트를 수락하고 이미 준비된 프로세스에 작성하므로, 첫 번째 `query()` 호출은 서브프로세스 생성 및 초기화 비용을 지불하지 않고 해결됩니다.

```typescript theme={null}
function startup(params?: {
  options?: Options;
  initializeTimeoutMs?: number;
}): Promise<WarmQuery>;
```

<h4 id="parameters-2">
  매개변수
</h4>

| 매개변수                  | 타입                    | 설명                                                                                       |
| :-------------------- | :-------------------- | :--------------------------------------------------------------------------------------- |
| `options`             | [`Options`](#options) | 선택적 구성 객체입니다. `query()`의 `options` 매개변수와 동일합니다                                           |
| `initializeTimeoutMs` | `number`              | 서브프로세스 초기화를 기다릴 최대 시간(밀리초)입니다. 기본값은 `60000`입니다. 초기화가 시간 내에 완료되지 않으면 프로미스는 타임아웃 오류로 거부됩니다 |

<h4 id="returns-2">
  반환값
</h4>

서브프로세스가 생성되고 초기화 핸드셰이크를 완료하면 해결되는 `Promise<`[`WarmQuery`](#warmquery)`>`를 반환합니다.

<h4 id="example">
  예제
</h4>

`startup()`을 조기에 호출합니다(예: 애플리케이션 부팅 시). 그런 다음 프롬프트가 준비되면 반환된 핸들에서 `.query()`를 호출합니다. 이렇게 하면 서브프로세스 생성 및 초기화가 중요 경로에서 벗어납니다.

```typescript theme={null}
import { startup } from "@anthropic-ai/claude-agent-sdk";

// 시작 비용을 미리 지불합니다
const warm = await startup({ options: { maxTurns: 3 } });

// 나중에 프롬프트가 준비되면 이것은 즉시입니다
for await (const message of warm.query("What files are here?")) {
  console.log(message);
}
```

<h3 id="tool">
  `tool()`
</h3>

SDK MCP 서버와 함께 사용하기 위한 타입 안전 MCP 도구 정의를 만듭니다.

```typescript theme={null}
function tool<Schema extends AnyZodRawShape>(
  name: string,
  description: string,
  inputSchema: Schema,
  handler: (args: InferShape<Schema>, extra: unknown) => Promise<CallToolResult>,
  extras?: { annotations?: ToolAnnotations }
): SdkMcpToolDefinition<Schema>;
```

<h4 id="parameters-3">
  매개변수
</h4>

| 매개변수          | 타입                                                                | 설명                                              |
| :------------ | :---------------------------------------------------------------- | :---------------------------------------------- |
| `name`        | `string`                                                          | 도구의 이름                                          |
| `description` | `string`                                                          | 도구가 수행하는 작업에 대한 설명                              |
| `inputSchema` | `Schema extends AnyZodRawShape`                                   | 도구의 입력 매개변수를 정의하는 Zod 스키마 (Zod 3 및 Zod 4 모두 지원) |
| `handler`     | `(args, extra) => Promise<`[`CallToolResult`](#calltoolresult)`>` | 도구 로직을 실행하는 비동기 함수                              |
| `extras`      | `{ annotations?: `[`ToolAnnotations`](#toolannotations)` }`       | 클라이언트에 동작 힌트를 제공하는 선택적 MCP 도구 주석                |

<h4 id="toolannotations">
  `ToolAnnotations`
</h4>

`@modelcontextprotocol/sdk/types.js`에서 다시 내보냅니다. 모든 필드는 선택적 힌트입니다. 클라이언트는 보안 결정을 위해 이들을 신뢰해서는 안 됩니다.

| 필드                | 타입        | 기본값         | 설명                                                                            |
| :---------------- | :-------- | :---------- | :---------------------------------------------------------------------------- |
| `title`           | `string`  | `undefined` | 도구의 사람이 읽을 수 있는 제목                                                            |
| `readOnlyHint`    | `boolean` | `false`     | `true`이면 도구는 환경을 수정하지 않습니다                                                    |
| `destructiveHint` | `boolean` | `true`      | `true`이면 도구는 파괴적인 업데이트를 수행할 수 있습니다 (`readOnlyHint`가 `false`일 때만 의미 있음)        |
| `idempotentHint`  | `boolean` | `false`     | `true`이면 동일한 인수로 반복 호출해도 추가 효과가 없습니다 (`readOnlyHint`가 `false`일 때만 의미 있음)      |
| `openWorldHint`   | `boolean` | `true`      | `true`이면 도구는 외부 엔티티와 상호작용합니다 (예: 웹 검색). `false`이면 도구의 도메인은 폐쇄적입니다 (예: 메모리 도구) |

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

애플리케이션과 동일한 프로세스에서 실행되는 MCP 서버 인스턴스를 만듭니다.

```typescript theme={null}
function createSdkMcpServer(options: {
  name: string;
  version?: string;
  tools?: Array<SdkMcpToolDefinition<any>>;
}): McpSdkServerConfigWithInstance;
```

<h4 id="parameters-4">
  매개변수
</h4>

| 매개변수              | 타입                            | 설명                              |
| :---------------- | :---------------------------- | :------------------------------ |
| `options.name`    | `string`                      | MCP 서버의 이름                      |
| `options.version` | `string`                      | 선택적 버전 문자열                      |
| `options.tools`   | `Array<SdkMcpToolDefinition>` | [`tool()`](#tool)로 만든 도구 정의의 배열 |

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

| 매개변수                       | 타입        | 기본값         | 설명                                                |
| :------------------------- | :-------- | :---------- | :------------------------------------------------ |
| `options.dir`              | `string`  | `undefined` | 세션을 나열할 디렉토리입니다. 생략하면 모든 프로젝트에서 세션을 반환합니다         |
| `options.limit`            | `number`  | `undefined` | 반환할 최대 세션 수                                       |
| `options.includeWorktrees` | `boolean` | `true`      | `dir`이 git 저장소 내에 있을 때 모든 worktree 경로에서 세션을 포함합니다 |

<h4 id="return-type-sdksessioninfo">
  반환 타입: `SDKSessionInfo`
</h4>

| 속성             | 타입                    | 설명                                                |
| :------------- | :-------------------- | :------------------------------------------------ |
| `sessionId`    | `string`              | 고유 세션 식별자 (UUID)                                  |
| `summary`      | `string`              | 표시 제목: 사용자 정의 제목, 자동 생성된 요약 또는 첫 번째 프롬프트          |
| `lastModified` | `number`              | 에포크 이후 밀리초 단위의 마지막 수정 시간                          |
| `fileSize`     | `number \| undefined` | 세션 파일 크기(바이트)입니다. 로컬 JSONL 저장소에만 채워집니다            |
| `customTitle`  | `string \| undefined` | 사용자가 설정한 세션 제목 (`/rename`을 통해)                    |
| `firstPrompt`  | `string \| undefined` | 세션의 첫 번째 의미 있는 사용자 프롬프트                           |
| `gitBranch`    | `string \| undefined` | 세션 끝의 git 분기                                      |
| `cwd`          | `string \| undefined` | 세션의 작업 디렉토리                                       |
| `tag`          | `string \| undefined` | 사용자가 설정한 세션 태그 ([`tagSession()`](#tagsession) 참조) |
| `createdAt`    | `number \| undefined` | 첫 번째 항목의 타임스탬프에서 에포크 이후 밀리초 단위의 생성 시간             |

<h4 id="example-2">
  예제
</h4>

프로젝트의 10개 최신 세션을 인쇄합니다. 결과는 `lastModified` 내림차순으로 정렬되므로 첫 번째 항목이 가장 최신입니다. 모든 프로젝트에서 검색하려면 `dir`을 생략합니다.

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

과거 세션 트랜스크립트에서 사용자 및 어시스턴트 메시지를 읽습니다.

```typescript theme={null}
function getSessionMessages(
  sessionId: string,
  options?: GetSessionMessagesOptions
): Promise<SessionMessage[]>;
```

<h4 id="parameters-6">
  매개변수
</h4>

| 매개변수             | 타입       | 기본값         | 설명                                                |
| :--------------- | :------- | :---------- | :------------------------------------------------ |
| `sessionId`      | `string` | 필수          | 읽을 세션 UUID ([`listSessions()`](#listsessions) 참조) |
| `options.dir`    | `string` | `undefined` | 세션을 찾을 프로젝트 디렉토리입니다. 생략하면 모든 프로젝트를 검색합니다          |
| `options.limit`  | `number` | `undefined` | 반환할 최대 메시지 수                                      |
| `options.offset` | `number` | `undefined` | 시작 부분에서 건너뛸 메시지 수                                 |

<h4 id="return-type-sessionmessage">
  반환 타입: `SessionMessage`
</h4>

| 속성                   | 타입                      | 설명                                                                                                                                                                     |
| :------------------- | :---------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`               | `"user" \| "assistant"` | 메시지 역할                                                                                                                                                                 |
| `uuid`               | `string`                | 고유 메시지 식별자                                                                                                                                                             |
| `session_id`         | `string`                | 이 메시지가 속한 세션                                                                                                                                                           |
| `message`            | `unknown`               | 트랜스크립트의 원본 메시지 페이로드                                                                                                                                                    |
| `parent_tool_use_id` | `string \| null`        | 서브에이전트 메시지의 경우 생성 `Agent` 도구 호출의 `tool_use_id`입니다. 메인 세션 메시지 및 이전 세션의 경우 `null`                                                                                        |
| `parent_agent_id`    | `string \| null`        | [중첩된 서브에이전트](/docs/ko/sub-agents#spawn-nested-subagents)의 메시지의 경우 이를 생성한 서브에이전트의 `agentId`입니다. 메인 세션 메시지, 최상위 서브에이전트의 메시지 및 이전 세션의 경우 `null`입니다. Claude Code v2.1.202 이상 필요 |

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

전체 프로젝트 디렉토리를 스캔하지 않고 ID로 단일 세션의 메타데이터를 읽습니다.

```typescript theme={null}
function getSessionInfo(
  sessionId: string,
  options?: GetSessionInfoOptions
): Promise<SDKSessionInfo | undefined>;
```

<h4 id="parameters-7">
  매개변수
</h4>

| 매개변수          | 타입       | 기본값         | 설명                                        |
| :------------ | :------- | :---------- | :---------------------------------------- |
| `sessionId`   | `string` | 필수          | 조회할 세션의 UUID                              |
| `options.dir` | `string` | `undefined` | 프로젝트 디렉토리 경로입니다. 생략하면 모든 프로젝트 디렉토리를 검색합니다 |

[`SDKSessionInfo`](#return-type-sdksessioninfo)를 반환하거나, 세션을 찾을 수 없으면 `undefined`를 반환합니다.

<h3 id="renamesession">
  `renameSession()`
</h3>

사용자 정의 제목 항목을 추가하여 세션의 이름을 바꿉니다. 반복 호출은 안전합니다. 가장 최신 제목이 우선합니다.

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

| 매개변수          | 타입       | 기본값         | 설명                                        |
| :------------ | :------- | :---------- | :---------------------------------------- |
| `sessionId`   | `string` | 필수          | 이름을 바꿀 세션의 UUID                           |
| `title`       | `string` | 필수          | 새 제목입니다. 공백을 제거한 후 비어 있지 않아야 합니다          |
| `options.dir` | `string` | `undefined` | 프로젝트 디렉토리 경로입니다. 생략하면 모든 프로젝트 디렉토리를 검색합니다 |

<h3 id="tagsession">
  `tagSession()`
</h3>

세션에 태그를 지정합니다. `null`을 전달하여 태그를 지웁니다. 반복 호출은 안전합니다. 가장 최신 태그가 우선합니다.

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

| 매개변수          | 타입               | 기본값         | 설명                                        |
| :------------ | :--------------- | :---------- | :---------------------------------------- |
| `sessionId`   | `string`         | 필수          | 태그를 지정할 세션의 UUID                          |
| `tag`         | `string \| null` | 필수          | 태그 문자열 또는 지우려면 `null`                     |
| `options.dir` | `string`         | `undefined` | 프로젝트 디렉토리 경로입니다. 생략하면 모든 프로젝트 디렉토리를 검색합니다 |

<h3 id="resolvesettings">
  `resolveSettings()`
</h3>

CLI와 동일한 병합 엔진을 사용하여 주어진 디렉토리에 대한 효과적인 Claude Code 설정을 해결하며, Claude CLI를 생성하지 않습니다. `query()` 호출을 호출하기 전에 어떤 구성을 볼 수 있는지 검사하는 데 사용합니다.

<Note>
  이 함수는 알파 버전이며 안정화 전에 API가 변경될 수 있습니다. CLI 시작과의 패리티를 위해 macOS plist 및 Windows HKLM/HKCU를 포함한 MDM 소스를 읽지만, 관리자가 구성한 `policyHelper` 서브프로세스를 실행하지 않습니다. `permissions.defaultMode` 필드는 프로젝트 설정을 포함한 모든 계층에서 그대로 반환됩니다. CLI가 권한 상승 모드를 적용하기 전에 적용하는 신뢰 필터는 적용되지 않습니다.
</Note>

```typescript theme={null}
function resolveSettings(
  options?: ResolveSettingsOptions
): Promise<ResolvedSettings>;
```

<h4 id="parameters-10">
  매개변수
</h4>

`resolveSettings()`는 단일 옵션 객체를 수락합니다. 모든 필드는 선택적입니다.

| 매개변수                            | 타입                                    | 기본값             | 설명                                                                                                                                                                                      |
| :------------------------------ | :------------------------------------ | :-------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options.cwd`                   | `string`                              | `process.cwd()` | 프로젝트 및 로컬 설정을 상대적으로 해결할 디렉토리                                                                                                                                                            |
| `options.settingSources`        | [`SettingSource`](#settingsource)`[]` | 모든 소스           | 로드할 파일 시스템 소스입니다. 사용자, 프로젝트 및 로컬 설정을 건너뛰려면 `[]`를 전달합니다. 관리 정책 설정은 모든 경우에 로드됩니다. 서버 관리 설정은 호스트가 `serverManagedSettings`를 전달할 때 가져오거나, 그렇지 않으면 CLI의 온디스크 캐시에서 읽습니다. 스냅샷은 네트워크에서 가져오지 않습니다 |
| `options.managedSettings`       | `Settings`                            | `undefined`     | 관리 정책 우선순위 수준에서 병합된 제한적 정책 계층 설정입니다. `model`과 같은 제한적이지 않은 키는 자동으로 삭제됩니다                                                                                                                 |
| `options.serverManagedSettings` | `Settings`                            | `undefined`     | `/api/claude_code/settings`의 서버 관리 설정 페이로드입니다. 제한적이지 않은 키는 필터링 없이 통과합니다                                                                                                                 |

<h4 id="return-type-resolvedsettings">
  반환 타입: `ResolvedSettings`
</h4>

`resolveSettings()`는 병합된 설정과 각 키에 기여한 소스를 설명하는 객체를 반환합니다.

| 속성           | 타입                                                  | 설명                                         |
| :----------- | :-------------------------------------------------- | :----------------------------------------- |
| `effective`  | `Settings`                                          | 모든 활성화된 소스를 우선순위 순서로 적용한 후 병합된 설정          |
| `provenance` | `Partial<Record<keyof Settings, ProvenanceEntry>>`  | `effective`의 각 최상위 키에 대해 값을 제공한 소스         |
| `sources`    | `Array<{ source, settings, path?, policyOrigin? }>` | 소스별 원본 설정, 가장 낮은 우선순위에서 가장 높은 우선순위 순서로 정렬됨 |

<h4 id="example-4">
  예제
</h4>

아래 예제는 프로젝트 디렉토리에 대한 설정을 해결하고 정리 기간을 제어하는 소스를 인쇄합니다.

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
  타입
</h2>

<h3 id="options">
  `Options`
</h3>

`query()` 함수의 구성 객체입니다.

| 속성                                | 타입                                                                                                       | 기본값                                | 설명                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------- | :--------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `abortController`                 | `AbortController`                                                                                        | `new AbortController()`            | 작업 취소를 위한 컨트롤러                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `additionalDirectories`           | `string[]`                                                                                               | `[]`                               | Claude가 액세스할 수 있는 추가 디렉토리                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `agent`                           | `string`                                                                                                 | `undefined`                        | 메인 스레드의 에이전트 이름입니다. 에이전트는 `agents` 옵션 또는 설정에서 정의되어야 합니다                                                                                                                                                                                                                                                                                                                                                                                                      |
| `agents`                          | `Record<string, [`AgentDefinition`](#agentdefinition)>`                                                  | `undefined`                        | 프로그래밍 방식으로 서브에이전트 정의                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `agentProgressSummaries`          | `boolean`                                                                                                | `false`                            | `true`일 때 서브에이전트에 대한 한 줄 진행 요약을 생성하고 `summary` 필드를 통해 [`task_progress`](#sdktaskprogressmessage) 이벤트에서 전달합니다. 포그라운드 및 백그라운드 서브에이전트에 적용됩니다                                                                                                                                                                                                                                                                                                                    |
| `allowDangerouslySkipPermissions` | `boolean`                                                                                                | `false`                            | 권한 건너뛰기를 활성화합니다. `permissionMode: 'bypassPermissions'`를 사용할 때 필수입니다                                                                                                                                                                                                                                                                                                                                                                                          |
| `allowedTools`                    | `string[]`                                                                                               | `[]`                               | 프롬프트 없이 자동 승인할 도구입니다. 이것은 Claude를 이 도구들로만 제한하지 않습니다. 나열되지 않은 도구는 `permissionMode` 및 `canUseTool`로 넘어갑니다. `disallowedTools`를 사용하여 도구를 차단합니다. [권한](/docs/ko/agent-sdk/permissions#allow-and-deny-rules) 참조                                                                                                                                                                                                                                                          |
| `betas`                           | [`SdkBeta`](#sdkbeta)`[]`                                                                                | `[]`                               | 베타 기능 활성화                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `canUseTool`                      | [`CanUseTool`](#canusetool)                                                                              | `undefined`                        | 사용자 정의 권한 함수로, [권한 흐름](/docs/ko/agent-sdk/permissions#how-permissions-are-evaluated)이 프롬프트로 넘어갈 때만 호출됩니다. `allowedTools`, 허용 규칙 또는 `permissionMode`에 의해 자동 승인된 호출에 대해서는 호출되지 않습니다. `AskUserQuestion`, 커넥터 도구 [조직이 `ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools) 및 [`requiresUserInteraction`](/docs/ko/mcp#require-approval-for-a-specific-tool)으로 표시된 MCP 도구는 허용 규칙이 일치하더라도 이에 도달합니다. `dontAsk` 모드에서는 대신 거부됩니다. 자세한 내용은 [`CanUseTool`](#canusetool) 참조 |
| `continue`                        | `boolean`                                                                                                | `false`                            | 가장 최근 대화 계속                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `cwd`                             | `string`                                                                                                 | `process.cwd()`                    | 현재 작업 디렉토리                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `debug`                           | `boolean`                                                                                                | `false`                            | Claude Code 프로세스에 대한 디버그 모드 활성화                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `debugFile`                       | `string`                                                                                                 | `undefined`                        | 특정 파일 경로에 디버그 로그를 작성합니다. 암묵적으로 디버그 모드를 활성화합니다                                                                                                                                                                                                                                                                                                                                                                                                                |
| `disallowedTools`                 | `string[]`                                                                                               | `[]`                               | 거부할 도구입니다. `Bash`와 같은 단순 이름은 Claude의 컨텍스트에서 도구를 제거합니다. `Bash(rm *)`와 같은 범위 지정 규칙은 도구를 사용 가능하게 유지하고 `bypassPermissions`를 포함한 모든 권한 모드에서 일치하는 호출을 거부합니다. [권한](/docs/ko/agent-sdk/permissions#allow-and-deny-rules) 참조                                                                                                                                                                                                                                               |
| `effort`                          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max'`                                                        | 모델 기본값                             | Claude가 응답에 투입하는 노력의 양을 제어합니다. 적응형 사고와 함께 작동하여 사고 깊이를 안내합니다. [노력 수준 조정](/docs/ko/model-config#adjust-effort-level) 참조                                                                                                                                                                                                                                                                                                                                             |
| `enableFileCheckpointing`         | `boolean`                                                                                                | `false`                            | 되감기를 위한 파일 변경 추적을 활성화합니다. [파일 체크포인팅](/docs/ko/agent-sdk/file-checkpointing) 참조                                                                                                                                                                                                                                                                                                                                                                                    |
| `env`                             | `Record<string, string \| undefined>`                                                                    | `process.env`                      | 환경 변수입니다. 설정하면 이는 `process.env`와 병합하는 대신 서브프로세스 환경을 대체하므로 `PATH`와 같은 상속된 변수를 유지하려면 `{ ...process.env, YOUR_VAR: 'value' }`를 전달합니다. [느린 또는 정지된 API 응답 처리](#handle-slow-or-stalled-api-responses)에서 이 패턴의 예제를 참조하고, 기본 CLI가 읽는 변수는 [환경 변수](/docs/ko/env-vars)를 참조하세요. User-Agent 헤더에서 앱을 식별하려면 `CLAUDE_AGENT_SDK_CLIENT_APP`을 설정합니다                                                                                                                                 |
| `executable`                      | `'bun' \| 'deno' \| 'node'`                                                                              | 자동 감지                              | 사용할 JavaScript 런타임                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `executableArgs`                  | `string[]`                                                                                               | `[]`                               | 실행 파일에 전달할 인수                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `extraArgs`                       | `Record<string, string \| null>`                                                                         | `{}`                               | 추가 인수                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `fallbackModel`                   | `string`                                                                                                 | `undefined`                        | 기본 모델이 실패할 경우 사용할 모델                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `forkSession`                     | `boolean`                                                                                                | `false`                            | `resume`으로 재개할 때 원본 세션을 계속하는 대신 새 세션 ID로 포크합니다                                                                                                                                                                                                                                                                                                                                                                                                               |
| `forwardSubagentText`             | `boolean`                                                                                                | `false`                            | 서브에이전트의 텍스트 및 사고 블록을 `parent_tool_use_id`가 설정된 어시스턴트 및 사용자 메시지로 전달하여 소비자가 중첩된 대화를 렌더링할 수 있습니다. 기본적으로 서브에이전트의 `tool_use` 및 `tool_result` 블록만 내보내집니다                                                                                                                                                                                                                                                                                                           |
| `hooks`                           | `Partial<Record<`[`HookEvent`](#hookevent)`, `[`HookCallbackMatcher`](#hookcallbackmatcher)`[]>>`        | `{}`                               | 이벤트에 대한 훅 콜백                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `includeHookEvents`               | `boolean`                                                                                                | `false`                            | 훅 라이프사이클 이벤트를 [`SDKHookStartedMessage`](#sdkhookstartedmessage), [`SDKHookProgressMessage`](#sdkhookprogressmessage) 및 [`SDKHookResponseMessage`](#sdkhookresponsemessage)로 메시지 스트림에 포함합니다. `SessionStart` 및 `Setup` 훅에 대한 라이프사이클 이벤트는 항상 포함되며 이 옵션이 필요하지 않습니다                                                                                                                                                                                               |
| `includePartialMessages`          | `boolean`                                                                                                | `false`                            | 부분 메시지 이벤트 포함                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `loadTimeoutMs`                   | `number`                                                                                                 | `60000`                            | *알파.* 재개 구체화 중 각 `sessionStore.load()` 및 `sessionStore.listSubkeys()` 호출에 대한 밀리초 단위의 타임아웃입니다. 어댑터가 이 윈도우 내에서 정착하지 않으면 쿼리가 중단되는 대신 실패합니다. `sessionStore`가 설정되지 않으면 무시됩니다                                                                                                                                                                                                                                                                                      |
| `managedSettings`                 | `Settings`                                                                                               | `undefined`                        | 생성 부모 프로세스에서 제공하는 정책 계층 설정입니다. 머신에 IT 제어 관리 설정 계층이 이미 있으면 삭제되며, 해당 관리자가 `parentSettingsBehavior: 'merge'`로 옵트인하지 않는 한 삭제됩니다. 제한적 전용 키로 필터링됩니다                                                                                                                                                                                                                                                                                                                |
| `maxBudgetUsd`                    | `number`                                                                                                 | `undefined`                        | 클라이언트 측 비용 추정이 이 USD 값에 도달하면 쿼리를 중지합니다. `total_cost_usd`와 동일한 추정과 비교됩니다. [비용 및 사용량 추적](/docs/ko/agent-sdk/cost-tracking) 참조                                                                                                                                                                                                                                                                                                                                       |
| `maxThinkingTokens`               | `number`                                                                                                 | `undefined`                        | *더 이상 사용되지 않음:* 대신 `thinking`을 사용합니다. 사고 프로세스의 최대 토큰                                                                                                                                                                                                                                                                                                                                                                                                         |
| `maxTurns`                        | `number`                                                                                                 | `undefined`                        | 최대 에이전트 턴 (도구 사용 왕복)                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `mcpServers`                      | `Record<string, [`McpServerConfig`](#mcpserverconfig)>`                                                  | `{}`                               | MCP 서버 구성                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `model`                           | `string`                                                                                                 | CLI의 기본값                           | Claude 모델 별칭 또는 전체 모델 이름입니다. [허용되는 값 및 공급자별 ID](/docs/ko/model-config#available-models) 참조                                                                                                                                                                                                                                                                                                                                                                        |
| `onElicitation`                   | `(request: ElicitationRequest, options: { signal: AbortSignal }) => Promise<ElicitationResult>`          | `undefined`                        | MCP 유도 요청을 처리하기 위한 콜백입니다. MCP 서버가 사용자 입력을 요청하고 훅이 먼저 처리하지 않을 때 호출됩니다. 제공되지 않으면 처리되지 않은 유도 요청이 자동으로 거부됩니다                                                                                                                                                                                                                                                                                                                                                     |
| `outputFormat`                    | `{ type: 'json_schema', schema: JSONSchema }`                                                            | `undefined`                        | 에이전트 결과의 출력 형식을 정의합니다. [구조화된 출력](/docs/ko/agent-sdk/structured-outputs) 참조                                                                                                                                                                                                                                                                                                                                                                                        |
| `outputStyle`                     | `string`                                                                                                 | `undefined`                        | `Options` 필드가 아닙니다. 대신 인라인 [`settings`](/docs/ko/settings) 객체 또는 설정 파일에서 `outputStyle`을 설정합니다. [출력 스타일 활성화](/docs/ko/agent-sdk/modifying-system-prompts#activate-an-output-style) 참조                                                                                                                                                                                                                                                                                   |
| `pathToClaudeCodeExecutable`      | `string`                                                                                                 | 번들된 네이티브 바이너리에서 자동 해결              | Claude Code 실행 파일의 경로입니다. 설치 중에 선택적 종속성을 건너뛰었거나 플랫폼이 지원되는 집합에 없을 때만 필요합니다                                                                                                                                                                                                                                                                                                                                                                                    |
| `permissionMode`                  | [`PermissionMode`](#permissionmode)                                                                      | `'default'`                        | 세션의 권한 모드                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `permissionPromptToolName`        | `string`                                                                                                 | `undefined`                        | 권한 프롬프트용 MCP 도구 이름                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `persistSession`                  | `boolean`                                                                                                | `true`                             | `false`일 때 디스크에 세션 지속성을 비활성화합니다. 세션을 나중에 재개할 수 없습니다                                                                                                                                                                                                                                                                                                                                                                                                          |
| `planModeInstructions`            | `string`                                                                                                 | `undefined`                        | 계획 모드에 대한 사용자 정의 워크플로우 지침입니다. `permissionMode`가 `'plan'`일 때 이 문자열은 기본 계획 모드 워크플로우 본문을 대체합니다. CLI는 여전히 읽기 전용 적용 프리앰블 및 ExitPlanMode 프로토콜 바닥글로 래핑합니다                                                                                                                                                                                                                                                                                                           |
| `plugins`                         | [`SdkPluginConfig`](#sdkpluginconfig)`[]`                                                                | `[]`                               | 로컬 경로에서 사용자 정의 플러그인을 로드합니다. [플러그인](/docs/ko/agent-sdk/plugins) 참조                                                                                                                                                                                                                                                                                                                                                                                                 |
| `promptSuggestions`               | `boolean`                                                                                                | `false`                            | 프롬프트 제안을 활성화합니다. 각 턴 후 예측된 다음 사용자 프롬프트를 포함하는 `prompt_suggestion` 메시지를 내보냅니다                                                                                                                                                                                                                                                                                                                                                                                  |
| `resume`                          | `string`                                                                                                 | `undefined`                        | 재개할 세션 ID                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `resumeSessionAt`                 | `string`                                                                                                 | `undefined`                        | 특정 메시지 UUID에서 세션 재개                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `sandbox`                         | [`SandboxSettings`](#sandboxsettings)                                                                    | `undefined`                        | 프로그래밍 방식으로 샌드박스 동작을 구성합니다. [샌드박스 설정](#sandboxsettings) 참조                                                                                                                                                                                                                                                                                                                                                                                                    |
| `sessionId`                       | `string`                                                                                                 | 자동 생성                              | 자동 생성하는 대신 세션에 특정 UUID를 사용합니다                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `sessionStore`                    | [`SessionStore`](/docs/ko/agent-sdk/session-storage#the-sessionstore-interface)                               | `undefined`                        | 세션 대화를 외부 백엔드로 미러링하여 모든 호스트가 이를 재개할 수 있습니다. [세션을 외부 저장소에 유지](/docs/ko/agent-sdk/session-storage) 참조                                                                                                                                                                                                                                                                                                                                                               |
| `sessionStoreFlush`               | `'batched' \| 'eager'`                                                                                   | `'batched'`                        | *알파.* `sessionStore`의 플러시 모드입니다. `sessionStore`가 설정되지 않으면 무시됩니다                                                                                                                                                                                                                                                                                                                                                                                              |
| `settings`                        | `string \| Settings`                                                                                     | `undefined`                        | 인라인 [설정](/docs/ko/settings) 객체 또는 설정 파일의 경로입니다. [우선순위 순서](/docs/ko/settings#settings-precedence)에서 플래그 설정 계층을 채웁니다. [`applyFlagSettings()`](#applyflagsettings)로 런타임에 변경합니다                                                                                                                                                                                                                                                                                            |
| `settingSources`                  | [`SettingSource`](#settingsource)`[]`                                                                    | CLI 기본값 (모든 소스)                    | 로드할 파일 시스템 설정을 제어합니다. 사용자, 프로젝트 및 로컬 설정을 비활성화하려면 `[]`을 전달합니다. 관리되는 정책 설정은 어쨌든 로드됩니다. 서버 관리 설정은 조직 자격 증명으로 세션이 [적격 구성](/docs/ko/server-managed-settings#platform-availability)에서 인증할 때 가져옵니다. [Claude Code 기능 사용](/docs/ko/agent-sdk/claude-code-features#what-settingsources-does-not-control)을 참조하여 이 옵션과 관계없이 읽히는 입력과 이를 비활성화하는 방법을 확인하세요                                                                                                                            |
| `skills`                          | `string[] \| 'all'`                                                                                      | `undefined`                        | 세션에서 사용 가능한 스킬입니다. 모든 발견된 스킬을 활성화하려면 `'all'`을 전달하거나 스킬 이름 목록을 전달합니다. 설정하면 SDK는 `allowedTools`에 Skill 도구를 자동으로 추가합니다. `tools`도 전달하는 경우 해당 목록에 `'Skill'`을 포함합니다. [스킬](/docs/ko/agent-sdk/skills) 참조                                                                                                                                                                                                                                                                 |
| `spawnClaudeCodeProcess`          | `(options: SpawnOptions) => SpawnedProcess`                                                              | `undefined`                        | Claude Code 프로세스를 생성하는 사용자 정의 함수입니다. VM, 컨테이너 또는 원격 환경에서 Claude Code를 실행하는 데 사용합니다                                                                                                                                                                                                                                                                                                                                                                           |
| `stderr`                          | `(data: string) => void`                                                                                 | `undefined`                        | stderr 출력에 대한 콜백                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `strictMcpConfig`                 | `boolean`                                                                                                | `false`                            | `mcpServers`에 전달된 서버만 사용하고 프로젝트 `.mcp.json`, 사용자 설정, 플러그인 제공 MCP 서버 및 [claude.ai 커넥터](/docs/ko/mcp#use-mcp-servers-from-claude-ai)를 무시합니다                                                                                                                                                                                                                                                                                                                         |
| `systemPrompt`                    | `string \| { type: 'preset'; preset: 'claude_code'; append?: string; excludeDynamicSections?: boolean }` | `undefined` (최소 프롬프트)              | 시스템 프롬프트 구성입니다. 사용자 정의 프롬프트의 경우 문자열을 전달하거나, Claude Code의 시스템 프롬프트를 사용하려면 `{ type: 'preset', preset: 'claude_code' }`를 전달합니다. 프리셋 객체 형식을 사용할 때 `append`를 추가하여 추가 지침으로 확장하고, `excludeDynamicSections: true`를 설정하여 세션별 컨텍스트를 첫 번째 사용자 메시지로 이동하여 [머신 간 프롬프트 캐시 재사용 개선](/docs/ko/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines)                                                                                                  |
| `taskBudget`                      | `{ total: number }`                                                                                      | `undefined`                        | *알파.* API 측 작업 예산 (토큰 단위)입니다. 설정하면 모델은 도구 사용을 조절하고 제한 전에 마무리할 수 있도록 남은 토큰 예산을 알려집니다                                                                                                                                                                                                                                                                                                                                                                          |
| `thinking`                        | [`ThinkingConfig`](#thinkingconfig)                                                                      | 지원되는 모델의 경우 `{ type: 'adaptive' }` | Claude의 사고/추론 동작을 제어합니다. [`ThinkingConfig`](#thinkingconfig) 참조                                                                                                                                                                                                                                                                                                                                                                                              |
| `title`                           | `string`                                                                                                 | `undefined`                        | 세션의 표시 제목입니다. `resume` 또는 `continue`를 통해 재개할 때 재개된 세션의 지속된 제목이 우선합니다. [`renameSession()`](#renamesession)을 사용하여 기존 세션의 제목을 변경합니다                                                                                                                                                                                                                                                                                                                             |
| `toolAliases`                     | `Record<string, string>`                                                                                 | `undefined`                        | 기본 제공 도구 이름을 MCP 도구 이름으로 매핑하여 Claude가 기본 제공 대신 MCP 구현을 호출하도록 합니다. 예를 들어 `{ Bash: 'mcp__workspace__bash' }`                                                                                                                                                                                                                                                                                                                                                   |
| `toolConfig`                      | [`ToolConfig`](#toolconfig)                                                                              | `undefined`                        | 기본 제공 도구 동작의 구성입니다. [`ToolConfig`](#toolconfig) 참조                                                                                                                                                                                                                                                                                                                                                                                                           |
| `tools`                           | `string[] \| { type: 'preset'; preset: 'claude_code' }`                                                  | `undefined`                        | 도구 구성입니다. 도구 이름의 배열을 전달하거나 프리셋을 사용하여 Claude Code의 기본 도구를 가져옵니다                                                                                                                                                                                                                                                                                                                                                                                               |

<h4 id="handle-slow-or-stalled-api-responses">
  느린 또는 정지된 API 응답 처리
</h4>

CLI 서브프로세스는 API 타임아웃 및 정지 감지를 제어하는 여러 환경 변수를 읽습니다. `env` 옵션을 통해 전달합니다:

```typescript theme={null}
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

* `API_TIMEOUT_MS`: Anthropic 클라이언트의 요청당 타임아웃 (밀리초 단위). 기본값 `600000`. 메인 루프 및 모든 서브에이전트에 적용됩니다.
* `CLAUDE_CODE_MAX_RETRIES`: 최대 API 재시도 횟수. 기본값 `10`, 최대 `15`로 제한됩니다. 각 재시도는 자체 `API_TIMEOUT_MS` 윈도우를 가지므로 최악의 경우 벽시간은 대략 `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` 더하기 백오프입니다. 더 긴 중단을 기다려야 하는 무인 실행의 경우 `CLAUDE_CODE_RETRY_WATCHDOG=1`을 설정하여 용량 오류를 무한정 재시도합니다. 그리고 Claude Code v2.1.199부터 다른 일시적 오류의 기본값을 `300`으로 올리고 이 변수의 상한을 제거합니다.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: `run_in_background`으로 시작된 서브에이전트에 대한 정지 감시견입니다. 기본값 `600000`. 각 스트림 이벤트에서 재설정되며, 정지 시 서브에이전트를 중단하고 작업을 실패로 표시하며 부분 결과와 함께 오류를 부모에게 표시합니다. 동기 서브에이전트에는 적용되지 않습니다.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` 및 `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: 헤더가 도착했지만 응답 본문이 스트리밍을 중지할 때 요청을 중단합니다. 감시견은 모든 공급자에 대해 기본적으로 켜져 있습니다. `CLAUDE_ENABLE_STREAM_WATCHDOG=0`으로 설정하여 비활성화합니다. `CLAUDE_STREAM_IDLE_TIMEOUT_MS`는 기본값 `300000`이고 해당 최소값으로 고정됩니다. 중단된 요청은 일반 재시도 경로를 거칩니다.

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
  applyFlagSettings(settings: { [K in keyof Settings]?: Settings[K] | null }): Promise<void>;
  initializationResult(): Promise<SDKControlInitializeResponse>;
  reinitialize(): Promise<SDKControlInitializeResponse>;
  supportedCommands(): Promise<SlashCommand[]>;
  supportedModels(): Promise<ModelInfo[]>;
  supportedAgents(): Promise<AgentInfo[]>;
  mcpServerStatus(): Promise<McpServerStatus[]>;
  accountInfo(): Promise<AccountInfo>;
  reconnectMcpServer(serverName: string): Promise<void>;
  toggleMcpServer(serverName: string, enabled: boolean): Promise<void>;
  setMcpServers(servers: Record<string, McpServerConfig>): Promise<McpSetServersResult>;
  streamInput(stream: AsyncIterable<SDKUserMessage>): Promise<void>;
  stopTask(taskId: string): Promise<void>;
  close(): void;
}
```

<h4 id="methods">
  메서드
</h4>

| 메서드                                    | 설명                                                                                                                                                                                                                                                                   |
| :------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt()`                          | 쿼리를 중단합니다. 스트리밍 입력 모드에서만 사용 가능합니다. CLI가 [`SDKSystemMessage.capabilities`](#sdksystemmessage)에서 `interrupt_receipt_v1` 기능을 광고할 때 중단을 견디는 대기 중인 메시지를 나열하는 [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse)로 해결됩니다. v2.1.205 이전의 CLI에서는 `undefined`로 해결됩니다 |
| `rewindFiles(userMessageId, options?)` | 파일을 지정된 사용자 메시지의 상태로 복원합니다. 변경 사항을 미리 보려면 `{ dryRun: true }`를 전달합니다. `enableFileCheckpointing: true`가 필요합니다. [파일 체크포인팅](/docs/ko/agent-sdk/file-checkpointing) 참조                                                                                                         |
| `setPermissionMode()`                  | 권한 모드를 변경합니다 (스트리밍 입력 모드에서만 사용 가능)                                                                                                                                                                                                                                   |
| `setModel()`                           | 모델을 변경합니다 (스트리밍 입력 모드에서만 사용 가능)                                                                                                                                                                                                                                      |
| `setMaxThinkingTokens()`               | *더 이상 사용되지 않음:* 대신 `thinking` 옵션을 사용합니다. 최대 사고 토큰을 변경합니다. `null`을 전달하면 사고를 세션 기본값으로 재설정합니다: 세션 중 재정의가 지워지고, 사고가 비활성화된 세션의 경우 사고는 비활성화 상태로 유지됩니다                                                                                                                      |
| `applyFlagSettings(settings)`          | 런타임에 세션의 플래그 설정 계층으로 설정을 병합합니다 (스트리밍 입력 모드에서만 사용 가능). [`applyFlagSettings()`](#applyflagsettings) 참조                                                                                                                                                                 |
| `initializationResult()`               | 지원되는 명령, 모델, 계정 정보 및 출력 스타일 구성을 포함한 전체 초기화 결과를 반환합니다                                                                                                                                                                                                                 |
| `reinitialize()`                       | 실행 중인 CLI에 `initialize` 제어 요청을 다시 보내고 캐시된 첫 연결 결과 대신 새로운 결과를 반환합니다. 연결 해제 후 세션에 다시 연결하는 것과 같은 전송 간격 후에 사용하여 보류 중인 권한 요청이 `canUseTool` 콜백에 다시 도달하도록 합니다. 응답이 손실된 요청은 다시 전달되므로 요청 ID당 콜백을 멱등성으로 만듭니다. Claude Code v2.1.195 이상이 필요합니다                                   |
| `supportedCommands()`                  | 사용 가능한 슬래시 명령을 반환합니다                                                                                                                                                                                                                                                 |
| `supportedModels()`                    | 표시 정보를 포함한 사용 가능한 모델을 반환합니다                                                                                                                                                                                                                                          |
| `supportedAgents()`                    | 사용 가능한 서브에이전트를 [`AgentInfo`](#agentinfo)`[]`로 반환합니다                                                                                                                                                                                                                  |
| `mcpServerStatus()`                    | 연결된 MCP 서버의 상태를 반환합니다                                                                                                                                                                                                                                                |
| `accountInfo()`                        | 계정 정보를 반환합니다                                                                                                                                                                                                                                                         |
| `reconnectMcpServer(serverName)`       | 이름으로 MCP 서버를 다시 연결합니다                                                                                                                                                                                                                                                |
| `toggleMcpServer(serverName, enabled)` | 이름으로 MCP 서버를 활성화 또는 비활성화합니다                                                                                                                                                                                                                                          |
| `setMcpServers(servers)`               | 이 세션의 MCP 서버 집합을 동적으로 바꿉니다. 추가, 제거 및 오류가 발생한 서버에 대한 정보를 반환합니다                                                                                                                                                                                                        |
| `streamInput(stream)`                  | 다중 턴 대화를 위해 입력 메시지를 쿼리로 스트리밍합니다                                                                                                                                                                                                                                      |
| `stopTask(taskId)`                     | ID로 실행 중인 백그라운드 작업을 중지합니다                                                                                                                                                                                                                                            |
| `close()`                              | 쿼리를 닫고 기본 프로세스를 종료합니다. 쿼리를 강제로 종료하고 모든 리소스를 정리합니다                                                                                                                                                                                                                    |

<h4 id="applyflagsettings">
  `applyFlagSettings()`
</h4>

실행 중인 세션에서 [설정](/docs/ko/settings)을 변경하고 쿼리를 다시 시작하지 않습니다. 전용 설정자가 없는 설정이 변경되어야 할 때 사용합니다. 예를 들어 에이전트가 신뢰할 수 없는 입력을 읽은 후 `permissions`를 강화합니다. `setModel()` 및 `setPermissionMode()`는 이 두 키에 대한 전용 설정자입니다. `applyFlagSettings()`는 설정 키의 모든 부분 집합을 허용하는 일반 형식이며, 여기에 `model`을 전달하는 것은 `setModel()`과 동일하게 작동합니다.

다음 턴에 적용되는 키만 있습니다:

* **다음 턴에 적용됨**: `model`, `effortLevel`, `ultracode`, `permissions`, `hooks`, `skillOverrides`, `fastMode`, `agent`. `agent`를 전환하면 해당 에이전트의 모델 재정의, 훅 및 시스템 프롬프트도 다음 턴에 적용됩니다.
* **세션 중 효과 없음**: 시스템 프롬프트 옵션입니다. 이들은 시작 시 한 번 해결되므로 실행 중인 세션은 호출이 성공하더라도 원본 값을 유지합니다. 이를 변경하려면 새 세션을 시작합니다.

`effortLevel`은 [노력 수준](/docs/ko/model-config#adjust-effort-level) 이름을 허용합니다. 또한 `"ultracode"`를 허용하며, 이는 세션을 `xhigh` 노력으로 실행하고 [ultracode](/docs/ko/workflows#let-claude-decide-with-ultracode)를 켭니다. `Settings` 타입은 해당 값 없이 `effortLevel`을 선언하므로 TypeScript에서 동등한 `{ ultracode: true }`를 전달합니다. `ultracode` 값은 Claude Code v2.1.203 이상이 필요하며 설정 파일의 `effortLevel` 키가 아닌 `applyFlagSettings()`에서만 허용됩니다.

값은 플래그 설정 계층에 기록되며, 이는 `query()`의 인라인 `settings` 옵션이 시작 시 채우는 계층과 동일합니다. 플래그 설정은 [설정 우선순위 순서](/docs/ko/settings#settings-precedence)의 상단 근처에 있습니다: 사용자, 프로젝트 및 로컬 설정을 재정의하며, 관리되는 정책 설정만 이를 재정의할 수 있습니다. 이는 [우선순위 섹션](#settings-precedence)이 프로그래밍 방식의 옵션이라고 부르는 것과 동일한 계층입니다.

연속 호출은 최상위 키를 얕게 병합합니다. `{ permissions: {...} }`를 포함한 두 번째 호출은 이전 호출의 전체 `permissions` 객체를 대체하며 깊게 병합하지 않습니다. 플래그 계층에서 키를 지우고 낮은 우선순위 소스로 돌아가려면 해당 키에 `null`을 전달합니다. `undefined`를 전달하면 JSON 직렬화가 이를 삭제하므로 효과가 없습니다.

스트리밍 입력 모드에서만 사용 가능하며, 이는 `setModel()` 및 `setPermissionMode()`와 동일한 제약입니다.

아래 예제는 세션 중간에 활성 모델을 전환한 다음 재정의를 지우므로 모델이 사용자 또는 프로젝트 설정이 지정하는 것으로 돌아갑니다.

```typescript theme={null}
const q = query({ prompt: messageStream });

// 세션의 나머지 부분에 대해 모델을 재정의합니다
await q.applyFlagSettings({ model: "claude-opus-4-6" });

// 나중에: 재정의를 지우고 낮은 우선순위 설정으로 돌아갑니다
await q.applyFlagSettings({ model: null });
```

<Note>
  `applyFlagSettings()`는 TypeScript 전용입니다. Python SDK는 동등한 메서드를 노출하지 않습니다.
</Note>

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
| `query(prompt)` | 미리 준비된 서브프로세스에 프롬프트를 보내고 [`Query`](#query-object)를 반환합니다. `WarmQuery`당 한 번만 호출할 수 있습니다 |
| `close()`       | 프롬프트를 보내지 않고 서브프로세스를 닫습니다. 더 이상 필요하지 않은 따뜻한 쿼리를 버리는 데 사용합니다                            |

`WarmQuery`는 `AsyncDisposable`을 구현하므로 자동 정리를 위해 `await using`과 함께 사용할 수 있습니다.

<h3 id="sdkcontrolinitializeresponse">
  `SDKControlInitializeResponse`
</h3>

`initializationResult()`의 반환 타입입니다. 세션 초기화 데이터를 포함합니다.

```typescript theme={null}
type SDKControlInitializeResponse = {
  commands: SlashCommand[];
  agents: AgentInfo[];
  output_style: string;
  available_output_styles: string[];
  models: ModelInfo[];
  account: AccountInfo;
  fast_mode_state?: "off" | "cooldown" | "on";
};
```

클라이언트가 이미 실행 중인 세션에 `initialize`를 보낼 때 제어 응답 래퍼는 선택적 `pending_permission_requests` 배열도 전달합니다. 필드는 응답 래퍼 자체에 있으며, 위의 `SDKControlInitializeResponse` 페이로드에는 없습니다. 각 항목은 세션이 실행 중일 때 권한 요청에 대해 스트리밍하는 것과 동일한 `{ type: "control_request", request_id, request }` 형태의 완전한 `control_request` 메시지입니다.

이들은 클라이언트가 연결되기 전에 발급되었으며 여전히 회신을 기다리고 있는 요청입니다. SDK는 배열을 읽고 각 항목을 [`canUseTool`](#canusetool) 콜백으로 전달하며, 이는 전송 간격 후 [`reinitialize()`](#query-object)가 트리거하는 것과 동일한 재전달입니다. 반복된 요청 ID를 멱등성으로 처리합니다. 연결이 끊어지기 전에 콜백이 이미 받은 요청을 반복할 수 있기 때문입니다.

<h3 id="sdkcontrolinterruptresponse">
  `SDKControlInterruptResponse`
</h3>

중단 수신입니다: [`interrupt()`](#query-object)가 [`SDKSystemMessage.capabilities`](#sdksystemmessage)에서 `interrupt_receipt_v1` 기능을 광고하는 CLI에서 해결되는 값입니다. Claude Code v2.1.205 이상이 필요합니다. 이전 CLI는 빈 성공 페이로드로 중단에 응답하므로 `interrupt()`는 `undefined`로 해결됩니다.

```typescript theme={null}
type SDKControlInterruptResponse = {
  still_queued: string[];
};
```

`still_queued`는 중단을 견디는 사용자 메시지의 UUID를 나열합니다: 대기열에 여전히 있는 메시지와 다음 턴을 위해 이미 제거되었지만 아직 중단으로 도달할 수 없는 배치입니다. 각각은 중단 후 자체 턴으로 실행되며, 먼저 취소하지 않는 한 실행됩니다. 수신을 사용하여 다시 보낼 것인지 결정합니다. 이미 나열된 메시지를 다시 보내면 중복 턴이 생성됩니다.

다음 주의 사항으로 목록을 해석합니다:

* UUID가 있는 메시지만 나타납니다. 빈 배열은 다른 것이 실행되지 않음을 의미하지 않습니다.
* 메인 스레드 메시지만 나열됩니다. 서브에이전트로 주소 지정된 메시지는 범위를 벗어납니다.
* 목록에는 클라이언트가 보낸 적이 없는 UUID (예: [예약된 작업](/docs/ko/scheduled-tasks) 트리거)가 포함될 수 있습니다. 오류로 취급하는 대신 인식하지 못하는 UUID를 무시합니다.

수신은 중단이 처리되는 순간에 찍은 스냅샷이며, 깨끗한 중단에서 중단된 턴의 [`SDKResultMessage`](#sdkresultmessage) 전에 도착합니다. 해당 결과 후 대기열을 검사하는 대신 수신을 읽습니다: 루프는 다음 대기 중인 턴을 즉시 시작하므로 결과 후 검사하는 대기열이 이미 변경되었습니다.

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
  memory?: "user" | "project" | "local";
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | number;
  permissionMode?: PermissionMode;
  criticalSystemReminder_EXPERIMENTAL?: string;
};
```

| 필드                                    | 필수  | 설명                                                                                                                                                   |
| :------------------------------------ | :-- | :--------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`                         | 예   | 이 에이전트를 사용할 시기에 대한 자연어 설명                                                                                                                            |
| `tools`                               | 아니오 | 허용된 도구 이름의 배열입니다. 생략하면 부모의 모든 도구를 상속합니다. 에이전트의 컨텍스트에 스킬을 미리 로드하려면 `'Skill'`을 여기에 나열하는 대신 `skills` 필드를 사용합니다                                          |
| `disallowedTools`                     | 아니오 | 이 에이전트에 대해 명시적으로 허용하지 않을 도구 이름의 배열입니다. MCP 서버 수준 패턴도 허용됩니다: `mcp__server` 또는 `mcp__server__*`는 해당 서버의 모든 도구를 제거하고, `mcp__*`는 모든 서버의 모든 MCP 도구를 제거합니다 |
| `prompt`                              | 예   | 에이전트의 시스템 프롬프트                                                                                                                                       |
| `model`                               | 아니오 | 이 에이전트의 모델 재정의입니다. `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'` 같은 별칭 또는 전체 모델 ID를 허용합니다. 생략하거나 `'inherit'`이면 메인 모델을 사용합니다                |
| `mcpServers`                          | 아니오 | 이 에이전트에 사용 가능한 MCP 서버 사양                                                                                                                             |
| `skills`                              | 아니오 | 에이전트 컨텍스트에 미리 로드할 스킬 이름의 배열                                                                                                                          |
| `initialPrompt`                       | 아니오 | 이 에이전트가 메인 스레드 에이전트로 실행될 때 첫 번째 사용자 턴으로 자동 제출됨                                                                                                       |
| `maxTurns`                            | 아니오 | 중지하기 전의 최대 에이전트 턴 수 (API 왕복)                                                                                                                         |
| `background`                          | 아니오 | 호출될 때 이 에이전트를 비차단 백그라운드 작업으로 실행                                                                                                                      |
| `memory`                              | 아니오 | 이 에이전트의 메모리 소스: `'user'`, `'project'` 또는 `'local'`                                                                                                   |
| `effort`                              | 아니오 | 이 에이전트의 추론 노력 수준입니다. 명명된 수준 또는 정수를 허용합니다                                                                                                             |
| `permissionMode`                      | 아니오 | 이 에이전트 내 도구 실행을 위한 권한 모드입니다. [`PermissionMode`](#permissionmode) 참조                                                                                  |
| `criticalSystemReminder_EXPERIMENTAL` | 아니오 | 실험적: 시스템 프롬프트에 추가된 중요한 알림                                                                                                                            |

<h3 id="agentmcpserverspec">
  `AgentMcpServerSpec`
</h3>

서브에이전트에 사용 가능한 MCP 서버를 지정합니다. 서버 이름 (부모의 `mcpServers` 구성에서 서버를 참조하는 문자열) 또는 서버 이름을 구성에 매핑하는 인라인 서버 구성 레코드일 수 있습니다.

```typescript theme={null}
type AgentMcpServerSpec = string | Record<string, McpServerConfigForProcessTransport>;
```

여기서 `McpServerConfigForProcessTransport`는 `McpStdioServerConfig | McpSSEServerConfig | McpHttpServerConfig | McpSdkServerConfig`입니다.

<h3 id="settingsource">
  `SettingSource`
</h3>

SDK가 설정을 로드하는 파일 시스템 기반 구성 소스를 제어합니다.

```typescript theme={null}
type SettingSource = "user" | "project" | "local";
```

| 값           | 설명                      | 위치                            |
| :---------- | :---------------------- | :---------------------------- |
| `'user'`    | 전역 사용자 설정               | `~/.claude/settings.json`     |
| `'project'` | 공유 프로젝트 설정 (버전 제어됨)     | `.claude/settings.json`       |
| `'local'`   | 로컬 프로젝트 설정 (gitignored) | `.claude/settings.local.json` |

<h4 id="default-behavior">
  기본 동작
</h4>

`settingSources`가 생략되거나 `undefined`일 때 `query()`는 Claude Code CLI와 동일한 파일 시스템 설정을 로드합니다: 사용자, 프로젝트 및 로컬입니다. 관리되는 정책 설정은 모든 경우에 로드됩니다. 서버 관리 설정은 조직 자격 증명으로 세션이 [적격 구성](/docs/ko/server-managed-settings#platform-availability)에서 인증할 때 가져옵니다. [Claude Code 기능 사용](/docs/ko/agent-sdk/claude-code-features#what-settingsources-does-not-control)을 참조하여 이 옵션과 관계없이 읽히는 입력과 이를 비활성화하는 방법을 확인하세요.

<h4 id="why-use-settingsources">
  settingSources를 사용하는 이유
</h4>

**파일 시스템 설정 비활성화:**

```typescript theme={null}
// 디스크에서 사용자, 프로젝트 또는 로컬 설정을 로드하지 않습니다
const result = query({
  prompt: "Analyze this code",
  options: { settingSources: [] }
});
```

**모든 파일 시스템 설정을 명시적으로 로드:**

```typescript theme={null}
const result = query({
  prompt: "Analyze this code",
  options: {
    settingSources: ["user", "project", "local"] // 모든 설정 로드
  }
});
```

**특정 설정 소스만 로드:**

```typescript theme={null}
// 프로젝트 설정만 로드, 사용자 및 로컬 무시
const result = query({
  prompt: "Run CI checks",
  options: {
    settingSources: ["project"] // .claude/settings.json만
  }
});
```

**테스트 및 CI 환경:**

```typescript theme={null}
// CI에서 일관된 동작을 보장하기 위해 로컬 설정 제외
const result = query({
  prompt: "Run tests",
  options: {
    settingSources: ["project"], // 팀 공유 설정만
    permissionMode: "bypassPermissions"
  }
});
```

**SDK 전용 애플리케이션:**

```typescript theme={null}
// 모든 것을 프로그래밍 방식으로 정의합니다.
// 파일 시스템 설정 소스를 거부하려면 []을 전달합니다.
const result = query({
  prompt: "Review this PR",
  options: {
    settingSources: [],
    agents: {
      /* ... */
    },
    mcpServers: {
      /* ... */
    },
    allowedTools: ["Read", "Grep", "Glob"]
  }
});
```

**CLAUDE.md 프로젝트 지침 로드:**

```typescript theme={null}
// 프로젝트 설정을 로드하여 CLAUDE.md 파일 포함
const result = query({
  prompt: "Add a new feature following project conventions",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code" // Claude Code의 시스템 프롬프트 사용
    },
    settingSources: ["project"], // 프로젝트 디렉토리에서 CLAUDE.md 로드
    allowedTools: ["Read", "Write", "Edit"]
  }
});
```

<h4 id="settings-precedence">
  설정 우선순위
</h4>

여러 소스가 로드될 때 설정은 이 우선순위로 병합됩니다 (높음에서 낮음):

1. 로컬 설정 (`.claude/settings.local.json`)
2. 프로젝트 설정 (`.claude/settings.json`)
3. 사용자 설정 (`~/.claude/settings.json`)

`agents`, `allowedTools` 및 `settings`와 같은 프로그래밍 방식의 옵션은 사용자, 프로젝트 및 로컬 파일 시스템 설정을 재정의합니다. 관리되는 정책 설정은 프로그래밍 방식의 옵션보다 우선합니다.

<h3 id="permissionmode">
  `PermissionMode`
</h3>

```typescript theme={null}
type PermissionMode =
  | "default" // 표준 권한 동작
  | "acceptEdits" // 파일 편집 자동 수락
  | "bypassPermissions" // 권한 확인 무시; 명시적 요청 규칙은 여전히 프롬프트
  | "plan" // 계획 모드 - 편집 없이 탐색
  | "dontAsk" // 권한에 대해 프롬프트하지 않음, 미리 승인되지 않으면 거부
  | "auto"; // 모델 분류기를 사용하여 각 도구 호출을 승인 또는 거부
```

<h3 id="canusetool">
  `CanUseTool`
</h3>

도구 사용을 제어하기 위한 사용자 정의 권한 함수 타입입니다.

함수는 대화형 권한 프롬프트의 SDK 대체입니다: [권한 평가 흐름](/docs/ko/agent-sdk/permissions#how-permissions-are-evaluated)이 프롬프트로 해결될 때만 호출됩니다. `allowedTools` 항목, 설정 허용 규칙 또는 `acceptEdits` 또는 `bypassPermissions`와 같은 권한 모드에 의해 이미 승인된 도구 호출은 이를 호출하지 않습니다. 모든 도구 호출을 제어하려면 [`PreToolUse` 훅](/docs/ko/agent-sdk/hooks)을 사용합니다.

`AskUserQuestion`, [`requiresUserInteraction`](/docs/ko/mcp#require-approval-for-a-specific-tool)으로 표시된 MCP 도구 및 [조직이 `ask`로 설정](/docs/ko/mcp#organization-controls-on-connector-tools)한 커넥터 도구는 허용 규칙이 일치하더라도 함수에 도달합니다. `dontAsk` 모드에서는 대신 거부됩니다.

```typescript theme={null}
type CanUseTool = (
  toolName: string,
  input: Record<string, unknown>,
  options: {
    signal: AbortSignal;
    suggestions?: PermissionUpdate[];
    blockedPath?: string;
    decisionReason?: string;
    toolUseID: string;
    agentID?: string;
    requestId: string;
  }
) => Promise<PermissionResult | null>;
```

| 옵션               | 타입                                          | 설명                                                                                                                                                                                                         |
| :--------------- | :------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `signal`         | `AbortSignal`                               | 작업을 중단해야 하는 경우 신호됨                                                                                                                                                                                         |
| `suggestions`    | [`PermissionUpdate`](#permissionupdate)`[]` | 사용자가 이 도구에 대해 다시 프롬프트되지 않도록 제안된 권한 업데이트입니다. Bash 프롬프트는 `localSettings` [대상](#permissionupdatedestination)을 포함하는 제안을 포함하므로 `updatedPermissions`에서 반환하면 규칙을 `.claude/settings.local.json`에 작성하고 세션 간에 유지합니다. |
| `blockedPath`    | `string`                                    | 해당하는 경우 권한 요청을 트리거한 파일 경로                                                                                                                                                                                  |
| `decisionReason` | `string`                                    | 이 권한 요청이 트리거된 이유를 설명합니다                                                                                                                                                                                    |
| `toolUseID`      | `string`                                    | 어시스턴트 메시지 내 이 특정 도구 호출의 고유 식별자                                                                                                                                                                             |
| `agentID`        | `string`                                    | 서브에이전트 내에서 실행 중인 경우 서브에이전트의 ID                                                                                                                                                                             |
| `requestId`      | `string`                                    | `control_request` 봉투의 `request_id`입니다. 애플리케이션이 SDK 외부에서 보내는 `control_response` (예: 서명된 HTTP POST)는 이 값을 반복해야 Claude Code 프로세스가 회신을 요청과 일치시킬 수 있습니다                                                         |

콜백은 일반적으로 [`PermissionResult`](#permissionresult)를 반환하여 요청을 해결하며, SDK는 이를 전송을 통해 `control_response`로 다시 작성합니다. 애플리케이션이 이미 이 요청에 대해 `control_response`를 자체 채널을 통해 보낸 경우에만 `null`을 반환하고 `requestId`를 반복합니다. SDK는 그러면 전송에 응답을 작성하는 것을 건너뜁니다. 다른 경우에 `null`을 반환하면 도구 호출이 무한정 차단된 상태로 유지됩니다. `control_response`가 전송되지 않고 권한 프롬프트가 시간 초과되지 않기 때문입니다.

`requestId` 옵션 및 `null` 반환 값은 Claude Code v2.1.199 이상이 필요합니다.

<h3 id="permissionresult">
  `PermissionResult`
</h3>

권한 확인의 결과입니다.

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

| 필드                              | 타입                     | 설명                                                                                                                                    |
| :------------------------------ | :--------------------- | :------------------------------------------------------------------------------------------------------------------------------------ |
| `askUserQuestion.previewFormat` | `'markdown' \| 'html'` | [`AskUserQuestion`](/docs/ko/agent-sdk/user-input#question-format) 옵션의 `preview` 필드를 옵트인하고 콘텐츠 형식을 설정합니다. 설정하지 않으면 Claude는 미리보기를 내보내지 않습니다 |

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

| 필드                 | 타입        | 설명                                                                                                                                   |
| :----------------- | :-------- | :----------------------------------------------------------------------------------------------------------------------------------- |
| `type`             | `'local'` | `'local'`이어야 합니다 (현재 로컬 플러그인만 지원됨)                                                                                                   |
| `path`             | `string`  | 플러그인 디렉토리의 절대 또는 상대 경로                                                                                                               |
| `skipMcpDiscovery` | `boolean` | `true`일 때 SDK는 이 플러그인에서 스킬, 훅, 에이전트 및 명령을 로드하지만 해당 `.mcp.json` 또는 매니페스트 `mcpServers`를 읽지 않습니다. 애플리케이션이 플러그인의 MCP 연결을 소유할 때 이를 설정합니다. |

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

쿼리에서 반환된 모든 가능한 메시지의 합집합 타입입니다.

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
  message: BetaMessage; // Anthropic SDK에서
  parent_tool_use_id: string | null;
  error?: SDKAssistantMessageError;
};
```

`message` 필드는 Anthropic SDK의 [`BetaMessage`](https://platform.claude.com/docs/ko/api/messages/create)입니다. `id`, `content`, `model`, `stop_reason` 및 `usage`와 같은 필드를 포함합니다.

`SDKAssistantMessageError`는 다음 중 하나입니다: `'authentication_failed'`, `'oauth_org_not_allowed'`, `'billing_error'`, `'rate_limit'`, `'overloaded'`, `'invalid_request'`, `'model_not_found'`, `'server_error'`, `'max_output_tokens'` 또는 `'unknown'`. `'model_not_found'`는 선택한 모델이 존재하지 않거나 계정 또는 배포에서 사용할 수 없음을 의미합니다. `'overloaded'`는 API가 서버가 용량에 도달했기 때문에 529를 반환했음을 의미하며, 이는 할당량에 대한 429인 `'rate_limit'`과는 다릅니다.

<h3 id="sdkusermessage">
  `SDKUserMessage`
</h3>

사용자 입력 메시지입니다.

```typescript theme={null}
type SDKUserMessage = {
  type: "user";
  uuid?: UUID;
  session_id?: string;
  message: MessageParam; // Anthropic SDK에서
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  shouldQuery?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
};
```

`shouldQuery`를 `false`로 설정하여 어시스턴트 턴을 트리거하지 않고 메시지를 트랜스크립트에 추가합니다. 메시지는 보류되고 턴을 트리거하는 다음 사용자 메시지로 병합됩니다. 이를 사용하여 모델 호출을 소비하지 않고 대역 외에서 실행한 명령의 출력과 같은 컨텍스트를 주입합니다.

도구 결과 블록을 전달하는 메시지에서 `tool_use_result`는 모델에 전송된 텍스트가 아니라 도구의 구조화된 출력 객체입니다. 해당 형태는 일치하는 `tool_use` 블록으로 명명된 도구에 따라 달라지므로 필드는 `unknown`으로 입력됩니다. 기본 제공 형태는 [도구 출력 타입](#tool-output-types)에 나열되어 있습니다.

`Agent` 도구의 경우 `tool_use_result`는 [`AgentOutput`](#agent-2)입니다. `completed` 결과에서 `content`는 Claude Code가 `tool_result` 텍스트에 추가하는 에이전트 ID 및 사용량 트레일러 없이 서브에이전트의 보고서를 보유합니다. 따라서 해당 텍스트를 파싱하는 대신 `tool_use_result`에서 렌더링합니다.

<h3 id="sdkusermessagereplay">
  `SDKUserMessageReplay`
</h3>

필수 UUID를 포함한 재생된 사용자 메시지입니다.

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

세션 외부에서 주입된 사용자 턴으로, [`origin`](#sdkmessageorigin) 종류가 `peer` 또는 `channel`인 경우, 활성 턴 중에 전달되었는지 또는 세션이 유휴 상태일 때 새 턴을 시작했는지 여부에 관계없이 스트림에 재생으로 도달합니다. v2.1.207 이전에는 세션이 유휴 상태일 때 전달된 주입된 턴이 스트림에서 메시지를 생성하지 않았으며 트랜스크립트를 다시 읽을 때만 나타났습니다.

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
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      structured_output?: unknown;
      deferred_tool_use?: { id: string; name: string; input: Record<string, unknown> };
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
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
      errors: string[];
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      origin?: SDKMessageOrigin;
    };
```

결과의 여러 필드는 `subtype` 이상의 진단 세부 정보를 전달합니다:

* `api_error_status`: 대화를 종료한 API 오류의 HTTP 상태 코드입니다. API 오류 없이 턴이 종료되었을 때는 없거나 `null`입니다.
* `ttft_ms`: 첫 번째 토큰까지의 시간(밀리초)입니다. 첫 번째 완전한 어시스턴트 메시지가 도착할 때 측정됩니다. 성공 경로에만 표시됩니다.
* `ttft_stream_ms`: 응답 스트림이 열릴 때 첫 번째 `message_start` 스트림 이벤트까지의 시간(밀리초)입니다. `ttft_ms`보다 낮습니다. 두 시간 사이의 간격은 첫 번째 메시지를 스트리밍하는 데 소요된 시간입니다. 성공 경로에만 표시됩니다.
* `terminal_reason`: 루프가 종료된 이유입니다. `"completed"`, `"max_turns"`, `"tool_deferred"`, `"aborted_streaming"`, `"aborted_tools"`, `"hook_stopped"`, `"stop_hook_prevented"`, `"background_requested"`, `"blocking_limit"`, `"rapid_refill_breaker"`, `"prompt_too_long"`, `"image_error"`, `"model_error"`, `"api_error"`, `"malformed_tool_use_exhausted"`, `"budget_exhausted"`, `"structured_output_retry_exhausted"`, `"tool_deferred_unavailable"` 또는 `"turn_setup_failed"` 중 하나입니다.
* `fast_mode_state`: `"on"`, `"off"` 또는 `"cooldown"` 중 하나입니다.

`origin` 필드는 이 결과를 트리거한 사용자 메시지의 [`SDKMessageOrigin`](#sdkmessageorigin)을 전달합니다. 백그라운드 작업이 완료되고 SDK가 합성 후속 턴을 주입할 때, 결과 `SDKResultMessage`는 `origin: { kind: "task-notification" }`을 전달합니다. 이 필드를 확인하여 프롬프트에 답하는 결과와 백그라운드 작업 후속을 위해 내보낸 결과를 구분하여 후자를 라우팅하거나 억제할 수 있습니다. 이 필드는 시작 오류와 같이 사용자 턴 이전에 내보낸 결과에는 없습니다.

`PreToolUse` 훅이 `permissionDecision: "defer"`를 반환할 때, 결과는 `stop_reason: "tool_deferred"`를 가지며 `deferred_tool_use`는 보류 중인 도구의 `id`, `name` 및 `input`을 전달합니다. 이 필드를 읽어 요청을 자신의 UI에 표시한 다음 동일한 `session_id`로 재개하여 계속합니다. 전체 왕복은 [나중을 위해 도구 호출 연기](/docs/ko/hooks#defer-a-tool-call-for-later)를 참조하십시오.

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
  }[];
  model: string;
  permissionMode: PermissionMode;
  slash_commands: string[];
  output_style: string;
  skills: string[];
  plugins: { name: string; path: string }[];
  capabilities?: string[];
};
```

`capabilities` 배열은 이 CLI가 구현하는 프로토콜 동작의 이름을 지정하므로 `claude_code_version` 문자열을 비교하는 대신 기능을 감지할 수 있습니다. 이는 개방형 집합입니다: 인식하지 못하는 값은 무시하고 의존하는 동작의 특정 기능을 확인합니다. 이 필드는 Claude Code v2.1.205 이상이 필요하며 이전 CLI에는 없습니다.

| 기능                     | 의미                                                                                                                                     |
| ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt_receipt_v1` | [`interrupt()`](#query-object)는 중단을 생존하는 대기 중인 메시지의 이름을 지정하는 [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) 영수증으로 해결됩니다 |

<h3 id="sdkpartialassistantmessage">
  `SDKPartialAssistantMessage`
</h3>

스트리밍 부분 메시지 (`includePartialMessages`가 true일 때만). `parent_tool_use_id` 필드는 항상 `null`입니다: 스트림 이벤트는 메인 세션에만 내보내집니다. 서브에이전트 속성을 위해 `parent_tool_use_id`를 전달하는 완전한 메시지를 사용하거나 [`forwardSubagentText`](#options)를 활성화하여 서브에이전트 텍스트와 생각을 완전한 메시지로 수신합니다.

```typescript theme={null}
type SDKPartialAssistantMessage = {
  type: "stream_event";
  event: BetaRawMessageStreamEvent; // Anthropic SDK에서
  parent_tool_use_id: string | null;
  uuid: UUID;
  session_id: string;
  ttft_ms?: number; // 첫 번째 토큰까지의 시간(ms), message_start 이벤트에만 표시됨
};
```

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

루프에서 내보낸 일반 텍스트 배너입니다. 비오류 상태 라인, `UserPromptSubmit` 훅의 블록 이유와 같은 훅 피드백, 및 명령 출력을 전달합니다. `content`를 주어진 `level`에서 일반 텍스트로 렌더링합니다.

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

원격 클라이언트가 하트비트 타임아웃을 기다리는 대신 워커가 사라진 이유를 표시할 수 있도록 정상적인 워커 종료 시 내보내집니다. `reason`은 호스트 CLI에서 설정한 짧은 snake\_case 문자열입니다(예: `"host_exit"` 또는 `"remote_control_disabled"`). 라이브 스트리밍할 때만 이에 대해 조치합니다. 재개된 세션은 이 메시지의 과거 인스턴스를 재생하므로 그 경우 무시합니다.

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

플러그인 설치 진행 이벤트입니다. [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/ko/env-vars)이 설정되면 내보내지므로 Agent SDK 애플리케이션이 첫 번째 턴 전에 마켓플레이스 플러그인 설치를 추적할 수 있습니다. `started` 및 `completed` 상태는 전체 설치를 괄호로 묶습니다. `installed` 및 `failed` 상태는 개별 마켓플레이스를 보고하고 `name`을 포함합니다.

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

권한 시스템이 대화형 프롬프트 없이 도구 호출을 자동으로 거부할 때 내보내지는 스트림 이벤트입니다. 이를 사용하여 거부를 UI에 렌더링할 수 있으며, 뒤따르는 `is_error` 도구 결과만 관찰하는 것이 아닙니다. 대화형 요청 경로는 [`canUseTool`](#canusetool) 콜백을 통해 애플리케이션에 별도로 도달합니다. `PreToolUse` 훅에서 발급된 거부는 이 이벤트를 통해 보고되지 않습니다.

이 이벤트는 Claude Code v2.1.136 이상이 필요합니다.

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
| `tool_use_id`          | `string` | 이 거부가 답하는 `tool_use` 블록의 ID                                                       |
| `agent_id`             | `string` | 거부된 호출이 서브에이전트 내부에서 발생한 경우 서브에이전트 ID입니다. 호스트 측 라우팅을 위해 `can_use_tool`의 필드를 미러링합니다 |
| `decision_reason_type` | `string` | `"rule"`, `"mode"`, `"classifier"` 또는 `"asyncAgent"`와 같이 결정한 구성 요소의 판별자           |
| `decision_reason`      | `string` | 사용 가능할 때 결정 구성 요소의 인간이 읽을 수 있는 이유                                                 |
| `message`              | `string` | `tool_result`에서 모델에 반환된 거부 메시지                                                    |

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

<h3 id="sdkmessageorigin">
  `SDKMessageOrigin`
</h3>

사용자 역할 메시지의 출처입니다. 이는 [`SDKUserMessage`](#sdkusermessage)의 `origin`으로 나타나며 해당 [`SDKResultMessage`](#sdkresultmessage)로 전달되므로 주어진 턴을 트리거한 것을 알 수 있습니다.

```typescript theme={null}
type SDKMessageOrigin =
  | { kind: "human" }
  | { kind: "channel"; server: string }
  | {
      kind: "peer";
      from: string;
      name?: string;
      senderTaskId?: string;
      body?: string;
    }
  | { kind: "task-notification" }
  | { kind: "coordinator" }
  | { kind: "auto-continuation" };
```

| `kind`              | 의미                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `human`             | 최종 사용자의 직접 입력입니다. 사용자 메시지에서 없는 `origin`도 인간 입력을 의미합니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `channel`           | [채널](/docs/ko/channels)에 도착한 메시지입니다. `server`는 소스 MCP 서버 이름입니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `peer`              | 다른 에이전트의 메시지입니다. 프로세스 내 [팀원](/docs/ko/agent-teams)이 `SendMessage`를 통해 `main`으로 보내는 경우, `from`은 팀원의 이름이고 `senderTaskId`는 해당 작업 ID입니다. 다른 로컬 Claude Code 프로세스와 같은 교차 세션 피어의 경우, `from`은 발신자 주소이고 `senderTaskId`는 없습니다. `name` 및 `body`는 Claude Code v2.1.205 이상이 필요합니다. `name`은 발신자의 표시 이름이며 Claude Code에서 정규화됩니다: 유니코드 제어, 형식, 서로게이트 및 줄 또는 단락 구분자 코드 포인트를 제거한 다음 결과를 자르고 64개 코드 포인트로 제한하고 줄임표를 추가합니다. `body`는 피어 봉투가 제거된 디코딩된 메시지 본문이며, 모델이 보는 것과 바이트 정확합니다. 팀원 메시지의 경우 `body`는 항상 존재합니다. 교차 세션 피어의 경우 Claude Code에서 형성한 정확히 하나의 피어 봉투인 경우에만 존재합니다. `name` 및 `body`를 렌더링하고 메시지 텍스트를 다시 파싱하지 마십시오. |
| `task-notification` | 백그라운드 작업이 완료된 후 주입된 합성 턴입니다. [`SDKTaskNotificationMessage`](#sdktasknotificationmessage)를 참조하십시오.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `coordinator`       | [에이전트 팀](/docs/ko/agent-teams)의 팀 코디네이터로부터의 메시지입니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `auto-continuation` | 새로운 사용자 입력 없이 세션이 계속될 때 주입된 합성 턴입니다. 예를 들어 후속 프롬프트를 트리거하는 명령 결과입니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |

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
  | "SessionStart"
  | "SessionEnd"
  | "Stop"
  | "SubagentStart"
  | "SubagentStop"
  | "PreCompact"
  | "PermissionRequest"
  | "Setup"
  | "TeammateIdle"
  | "TaskCompleted"
  | "ConfigChange"
  | "WorktreeCreate"
  | "WorktreeRemove"
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
  | NotificationHookInput
  | UserPromptSubmitHookInput
  | SessionStartHookInput
  | SessionEndHookInput
  | StopHookInput
  | SubagentStartHookInput
  | SubagentStopHookInput
  | PreCompactHookInput
  | PermissionRequestHookInput
  | SetupHookInput
  | TeammateIdleHookInput
  | TaskCompletedHookInput
  | ConfigChangeHookInput
  | WorktreeCreateHookInput
  | WorktreeRemoveHookInput
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
};
```

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
};
```

<h4 id="sessionstarthookinput">
  `SessionStartHookInput`
</h4>

```typescript theme={null}
type SessionStartHookInput = BaseHookInput & {
  hook_event_name: "SessionStart";
  source: "startup" | "resume" | "clear" | "compact";
  agent_type?: string;
  model?: string;
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

<h4 id="permissionrequesthookinput">
  `PermissionRequestHookInput`
</h4>

```typescript theme={null}
type PermissionRequestHookInput = BaseHookInput & {
  hook_event_name: "PermissionRequest";
  tool_name: string;
  tool_input: unknown;
  permission_suggestions?: PermissionUpdate[];
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
      }
    | {
        hookEventName: "SessionStart";
        additionalContext?: string;
      }
    | {
        hookEventName: "Setup";
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStart";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolUse";
        additionalContext?: string;
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

모든 도구 입력 타입의 합집합으로, `@anthropic-ai/claude-agent-sdk`에서 내보냅니다.

```typescript theme={null}
type ToolInputSchemas =
  | AgentInput
  | AskUserQuestionInput
  | BashInput
  | TaskOutputInput
  | EnterWorktreeInput
  | ExitPlanModeInput
  | FileEditInput
  | FileReadInput
  | FileWriteInput
  | GlobInput
  | GrepInput
  | ListMcpResourcesInput
  | McpInput
  | MonitorInput
  | NotebookEditInput
  | ReadMcpResourceInput
  | SubscribeMcpResourceInput
  | SubscribePollingInput
  | TaskCreateInput
  | TaskGetInput
  | TaskListInput
  | TaskStopInput
  | TaskUpdateInput
  | TodoWriteInput
  | UnsubscribeMcpResourceInput
  | UnsubscribePollingInput
  | WebFetchInput
  | WebSearchInput
  | WorkflowInput;
```

<h3 id="agent">
  Agent
</h3>

**도구 이름:** `Agent` (이전 `Task`, 여전히 별칭으로 수락됨)

```typescript theme={null}
type AgentInput = {
  description: string;
  prompt: string;
  subagent_type?: string;
  model?: "sonnet" | "opus" | "haiku" | "fable";
  run_in_background?: boolean;
  name?: string;
  mode?: "acceptEdits" | "auto" | "bypassPermissions" | "default" | "dontAsk" | "plan";
  isolation?: "worktree";
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

선택적 타임아웃 및 백그라운드 실행을 포함한 지속적인 셸 세션에서 bash 명령을 실행합니다.

<h3 id="monitor">
  Monitor
</h3>

**도구 이름:** `Monitor`

```typescript theme={null}
type MonitorInput = {
  command?: string;
  ws?: {
    url: string;
    protocols?: string[];
  };
  description: string;
  timeout_ms?: number;
  persistent?: boolean;
};
```

백그라운드 소스를 실행하고 각 이벤트를 Claude에 전달하므로 폴링 없이 반응할 수 있습니다: `command`는 스크립트를 실행하고 stdout 라인당 하나의 이벤트를 내보내며, `ws`는 WebSocket을 열고 텍스트 프레임당 하나의 이벤트를 내보냅니다. `command` 또는 `ws` 중 정확히 하나를 제공합니다. `ws` 소스는 Claude Code v2.1.195 이상이 필요합니다.

로그 테일과 같은 세션 길이 감시의 경우 `persistent: true`를 설정합니다. Monitor가 명령을 실행할 때, Bash와 동일한 권한 규칙을 따릅니다. WebSocket 감시는 별도로 승인을 요청합니다. 동작 및 공급자 가용성은 [Monitor 도구 참조](/docs/ko/tools-reference#monitor-tool)를 참조하세요.

<h3 id="taskoutput">
  TaskOutput
</h3>

**도구 이름:** `TaskOutput`

```typescript theme={null}
type TaskOutputInput = {
  task_id: string;
  block: boolean;
  timeout: number;
};
```

실행 중이거나 완료된 백그라운드 작업에서 출력을 검색합니다.

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

텍스트, 이미지, PDF 및 Jupyter 노트북을 포함한 로컬 파일 시스템에서 파일을 읽습니다. PDF 페이지 범위의 경우 `pages`를 사용합니다 (예: `"1-5"`).

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
  args?: unknown;
  resumeFromRunId?: string;
};
```

[동적 워크플로우](/docs/ko/workflows)를 실행합니다: 백그라운드에서 많은 서브에이전트를 조율하고 하나의 통합된 결과를 반환하는 스크립트입니다. `Workflow` 도구는 Agent SDK v0.3.149 이상에서 사용 가능합니다. `script`, `name` 또는 `scriptPath` 중 최소 하나가 필요합니다.

| 필드                | 타입        | 설명                                                                                                                                                                                                               |
| ----------------- | --------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `script`          | `string`  | 인라인 워크플로우 스크립트입니다. `export const meta = { name, description }`로 시작해야 하며, 그 뒤에 `agent()`, `parallel()`, `pipeline()` 및 `phase()`를 사용하는 스크립트 본문이 따릅니다. `meta`의 선택적 `phases` 배열은 진행 상황 보기에서 에이전트를 명명된 단계 아래에 그룹화합니다 |
| `name`            | `string`  | 기본 제공 워크플로우의 이름 또는 `.claude/workflows/`에 저장된 워크플로우입니다. 스크립트로 확인됩니다                                                                                                                                               |
| `scriptPath`      | `string`  | 디스크의 워크플로우 스크립트 파일 경로입니다. `script` 및 `name`보다 우선합니다. 모든 호출은 스크립트를 유지하고 결과에서 경로를 반환하므로, 해당 파일을 편집하고 동일한 `scriptPath`로 다시 호출하여 반복할 수 있습니다                                                                          |
| `args`            | `unknown` | 스크립트에 전역 `args`로 노출되는 입력 값으로, 연구 질문이나 파일 경로 목록과 같은 매개변수화된 명명된 워크플로우용입니다. 배열과 객체를 JSON 인코딩된 문자열이 아닌 실제 JSON 값으로 전달합니다                                                                                             |
| `resumeFromRunId` | `string`  | 재개할 이전 `Workflow` 호출의 실행 ID입니다. 입력이 변경되지 않은 완료된 `agent()` 호출은 캐시된 결과를 반환합니다. 변경되거나 새로운 호출만 실시간으로 실행됩니다. 동일한 세션만 해당됩니다                                                                                            |

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
  TypeScript Agent SDK 0.3.142부터 `TodoWrite`는 기본적으로 비활성화됩니다. 대신 `TaskCreate`, `TaskGet`, `TaskUpdate` 및 `TaskList`를 사용하세요. 모니터링 코드를 업데이트하려면 [작업 도구로 마이그레이션](/docs/ko/agent-sdk/todo-tracking#migrate-to-task-tools)을 참조하거나, `CLAUDE_CODE_ENABLE_TASKS=0`을 설정하여 `TodoWrite`로 되돌립니다.
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
  /** Deprecated: no longer used. */
  allowedPrompts?: Array<{
    tool: "Bash";
    prompt: string;
  }>;
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

<h2 id="tool-output-types">
  도구 출력 타입
</h2>

모든 기본 제공 Claude Code 도구의 출력 스키마 문서입니다. 이 타입은 `@anthropic-ai/claude-agent-sdk`에서 내보내지며 각 도구에서 반환된 실제 응답 데이터를 나타냅니다.

<h3 id="tooloutputschemas">
  `ToolOutputSchemas`
</h3>

모든 도구 출력 타입의 합집합입니다.

```typescript theme={null}
type ToolOutputSchemas =
  | AgentOutput
  | AskUserQuestionOutput
  | BashOutput
  | EnterWorktreeOutput
  | ExitPlanModeOutput
  | FileEditOutput
  | FileReadOutput
  | FileWriteOutput
  | GlobOutput
  | GrepOutput
  | ListMcpResourcesOutput
  | MonitorOutput
  | NotebookEditOutput
  | ReadMcpResourceOutput
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

**도구 이름:** `Agent` (이전 `Task`, 여전히 별칭으로 수락됨)

```typescript theme={null}
type AgentOutput =
  | {
      status: "completed";
      agentId: string;
      agentType?: string;
      content: Array<{ type: "text"; text: string; citations?: unknown[] | null }>;
      resolvedModel?: string;
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

서브에이전트의 결과를 반환합니다. `status` 필드에서 구분됩니다: 완료된 작업의 경우 `"completed"`, 백그라운드 작업의 경우 `"async_launched"`, Claude Code가 원격 클라우드 세션으로 전달한 작업의 경우 `"remote_launched"`이며, 여기서 `sessionUrl`은 해당 세션으로 연결되고 `taskId`는 이를 식별합니다.

`resolvedModel` 필드는 `completed` 및 `async_launched` 변형에서 서브에이전트가 실제로 실행된 모델의 이름을 지정하며, 이는 [`availableModels`](/docs/ko/model-config#restrict-model-selection) 또는 다른 재정의가 적용될 때 요청된 `model` 입력과 다를 수 있습니다. 이 필드는 Claude Code v2.1.174 이상이 필요합니다.

`completed` 변형에서 `worktreePath`는 서브에이전트가 격리된 git worktree에서 실행되었을 때 설정되며, `worktreeBranch`는 Claude Code가 생성했을 때 해당 worktree의 분기 이름을 지정합니다. `usage.service_tier`는 서브에이전트의 요청에 대해 API가 보고한 서비스 계층 문자열을 전달합니다.

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
  dangerouslyDisableSandbox?: boolean;
  returnCodeInterpretation?: string;
  structuredContent?: unknown[];
  persistedOutputPath?: string;
  persistedOutputSize?: number;
};
```

stdout/stderr가 분할된 명령 출력을 반환합니다. 백그라운드 명령은 `backgroundTaskId`를 포함합니다.

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
  originalFile: string;
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
  };
};
```

구조화된 diff 정보를 포함한 쓰기 결과를 반환합니다.

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
};
```

glob 패턴과 일치하는 파일 경로를 수정 시간별로 정렬하여 반환합니다.

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
  appliedLimit?: number;
  appliedOffset?: number;
};
```

검색 결과를 반환합니다. 형태는 `mode`에 따라 다릅니다: 파일 목록, 일치 항목이 있는 콘텐츠 또는 일치 항목 수.

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
};
```

HTTP 상태 및 메타데이터를 포함한 가져온 콘텐츠를 반환합니다.

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
};
```

웹에서 검색 결과를 반환합니다.

<h3 id="workflow-2">
  Workflow
</h3>

**도구 이름:** `Workflow`

```typescript theme={null}
type WorkflowOutput = {
  status: "async_launched";
  taskId: string;
  runId?: string;
  summary?: string;
  transcriptDir?: string;
  scriptPath?: string;
  error?: string;
};
```

도구가 호출을 수락한 후 즉시 반환됩니다. 최종 결과는 나중에 작업 완료로 도착합니다. 실행이 시작된 것으로 취급하기 전에 `error`를 확인하십시오: 구문 검사에 실패한 스크립트는 `status: "async_launched"`를 반환하고 `error`가 설정되며, 실행되지 않습니다.

| 필드              | 타입                 | 설명                                                                                    |
| --------------- | ------------------ | ------------------------------------------------------------------------------------- |
| `status`        | `"async_launched"` | 도구가 호출을 수락했습니다. 이것이 필드가 취하는 유일한 값입니다                                                  |
| `taskId`        | `string`           | 실행을 위한 백그라운드 작업 식별자                                                                   |
| `runId`         | `string`           | 나중의 호출에서 `resumeFromRunId`로 전달할 워크플로우 실행 식별자                                          |
| `summary`       | `string`           | 워크플로우가 수행하는 작업에 대한 한 줄 설명                                                             |
| `transcriptDir` | `string`           | 실행 중에 서브에이전트 트랜스크립트가 작성되는 디렉토리                                                        |
| `scriptPath`    | `string`           | 이 실행을 위해 유지된 워크플로우 스크립트의 경로입니다. 스크립트를 다시 보내지 않고 다시 실행하려면 편집하고 `scriptPath`로 다시 전달하십시오 |
| `error`         | `string`           | 스크립트가 구문 검사에 실패할 때 설정됩니다. 존재할 때, `async_launched` 상태에도 불구하고 실행이 시작되지 않았습니다            |

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
  TypeScript Agent SDK 0.3.142부터 `TodoWrite`는 기본적으로 비활성화됩니다. 대신 `TaskCreate`, `TaskGet`, `TaskUpdate`, `TaskList`를 사용하십시오. 모니터링 코드를 업데이트하려면 [작업 도구로 마이그레이션](/docs/ko/agent-sdk/todo-tracking#migrate-to-task-tools)을 참조하거나, `CLAUDE_CODE_ENABLE_TASKS=0`을 설정하여 `TodoWrite`로 되돌립니다.
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
  }>;
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

```typescript theme={null}
type ApiKeySource = "user" | "project" | "org" | "temporary" | "oauth";
```

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

`betas` 옵션을 통해 활성화할 수 있는 사용 가능한 베타 기능입니다. [베타 헤더](https://platform.claude.com/docs/ko/api/beta-headers)를 참조하세요.

```typescript theme={null}
type SdkBeta = "context-1m-2025-08-07";
```

<Warning>
  `context-1m-2025-08-07` 베타는 2026년 4월 30일부터 폐기되었습니다. Claude Sonnet 4.5 또는 Sonnet 4와 함께 이 값을 전달하면 효과가 없으며, 표준 200k 토큰 컨텍스트 윈도우를 초과하는 요청은 오류를 반환합니다. 1M 토큰 컨텍스트 윈도우를 사용하려면 [Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7 또는 Claude Opus 4.8](https://platform.claude.com/docs/ko/about-claude/models/overview)로 마이그레이션하세요. 이들은 베타 헤더 없이 표준 가격으로 1M 컨텍스트를 포함합니다.
</Warning>

<h3 id="slashcommand">
  `SlashCommand`
</h3>

사용 가능한 슬래시 명령에 대한 정보입니다.

```typescript theme={null}
type SlashCommand = {
  name: string;
  description: string;
  argumentHint: string;
  aliases?: string[];
};
```

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

| 필드            | 타입                    | 설명                                                |
| :------------ | :-------------------- | :------------------------------------------------ |
| `name`        | `string`              | 에이전트 타입 식별자 (예: `"Explore"`, `"general-purpose"`) |
| `description` | `string`              | 이 에이전트를 사용할 시기에 대한 설명                             |
| `model`       | `string \| undefined` | 이 에이전트가 사용하는 모델 별칭입니다. 생략하면 부모의 모델을 상속합니다         |

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
  tools?: {
    name: string;
    description?: string;
    annotations?: {
      readOnly?: boolean;
      destructive?: boolean;
      openWorld?: boolean;
    };
  }[];
};
```

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
  cacheReadInputTokens: number;
  cacheCreationInputTokens: number;
  webSearchRequests: number;
  costUSD: number;
  contextWindow: number;
  maxOutputTokens: number;
};
```

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
};
```

`BetaServerToolUsage`와 `BetaIterationsUsage`는 `@anthropic-ai/sdk`에서 정의됩니다.

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

선택적 `display` 필드는 사고 텍스트가 `"summarized"` 또는 `"omitted"`로 반환되는지 제어합니다. Claude Opus 4.7 이상에서 API 기본값은 `"omitted"`이므로, `thinking` 블록에서 사고 콘텐츠를 받으려면 `"summarized"`를 설정하세요.

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
};
```

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

백그라운드 작업이 완료, 실패 또는 중지될 때의 알림입니다. 백그라운드 작업에는 `run_in_background` Bash 명령, [Monitor](#monitor) 감시 및 백그라운드 서브에이전트가 포함됩니다.

```typescript theme={null}
type SDKTaskNotificationMessage = {
  type: "system";
  subtype: "task_notification";
  task_id: string;
  tool_use_id?: string;
  status: "completed" | "failed" | "stopped";
  output_file: string;
  summary: string;
  usage?: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  uuid: UUID;
  session_id: string;
};
```

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
  uuid: UUID;
  session_id: string;
};
```

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

백그라운드 작업이 시작될 때 내보내집니다. `task_type` 필드는 백그라운드 Bash 명령 및 [Monitor](#monitor) 감시의 경우 `"local_bash"`, 서브에이전트의 경우 `"local_agent"` 또는 `"remote_agent"`입니다.

```typescript theme={null}
type SDKTaskStartedMessage = {
  type: "system";
  subtype: "task_started";
  task_id: string;
  tool_use_id?: string;
  description: string;
  task_type?: string;
  uuid: UUID;
  session_id: string;
};
```

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

라이브 백그라운드 작업 집합이 변경될 때마다 내보내집니다. 작업이 시작되거나, 완료되거나, 종료되거나, 포그라운드 에이전트가 백그라운드로 전환될 때입니다. `tasks` 배열은 전체 라이브 집합입니다. `task_started` 및 `task_notification` 이벤트를 쌍으로 지정하는 대신 각 페이로드로 캐시된 집합을 바꾸므로, 다음 멤버십 변경이 놓친 이벤트를 수정합니다.

이러한 작업별 이벤트에 대한 순서는 지정되지 않으므로, 두 스트림을 상관시키지 마세요.

시작 시 아무것도 내보내지지 않습니다. 세션의 CLI 프로세스가 시작되거나 다시 시작될 때마다 빈 집합으로 재설정하고 다음 멤버십 변경이 다시 채우도록 하세요.

Claude Code v2.1.203 이상이 필요합니다.

```typescript theme={null}
type SDKBackgroundTasksChangedMessage = {
  type: "system";
  subtype: "background_tasks_changed";
  tasks: {
    task_id: string;
    task_type: string;
    description: string;
  }[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkthinkingtokensmessage">
  `SDKThinkingTokensMessage`
</h3>

Claude가 사고 블록을 생성하는 동안 내보내집니다. 여기에는 지금까지 생성된 사고 토큰의 실행 추정치가 포함됩니다. `estimated_tokens`는 현재 사고 블록의 실행 합계이고 `estimated_tokens_delta`는 이 프레임에서 전달된 증분입니다. 진행 상황 표시에 사용하세요. 최상위 에이전트 루프의 최종 개수는 결과 메시지의 `usage.output_tokens`입니다. 이는 [서브에이전트 토큰을 포함하지 않습니다](/docs/ko/agent-sdk/cost-tracking#get-the-total-cost-of-a-query). 전체 트리 회계를 위해 [`modelUsage`](#modelusage)를 사용하세요.

Claude Code v2.1.153 이상이 필요합니다.

```typescript theme={null}
type SDKThinkingTokensMessage = {
  type: "system";
  subtype: "thinking_tokens";
  estimated_tokens: number;
  estimated_tokens_delta: number;
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

로컬 슬래시 명령의 출력 (예: `/voice` 또는 `/usage`). 트랜스크립트에서 어시스턴트 스타일 텍스트로 표시됩니다.

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

사용 가능한 명령 집합이 세션 중간에 변경될 때 내보내집니다. 예를 들어 에이전트가 하위 디렉토리에 들어갈 때 스킬이 발견될 때입니다. `commands` 배열은 전체 업데이트된 목록이므로, 캐시된 명령 목록을 이 페이로드로 바꾸세요. `supportedCommands()`를 다시 호출하는 것은 동등하지 않습니다. 해당 메서드는 초기화 시 캡처된 스냅샷을 반환하며 세션 중간 변경을 반영하지 않습니다.

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

`promptSuggestions`가 활성화되었을 때 각 턴 후에 내보내집니다. 예측된 다음 사용자 프롬프트를 포함합니다.

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

세션의 대화가 세션을 종료하지 않고 바뀔 때 내보내집니다. 예를 들어 `/clear` 후, 계획 모드 종료 시, 또는 새로운 대화가 시작될 때입니다. `new_conversation_id` 아래에 빈 트랜스크립트를 마운트하고 캐시된 세션 제목을 버리세요.

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

| 속성                          | 타입                                                    | 기본값         | 설명                                                                                                                                                                     |
| :-------------------------- | :---------------------------------------------------- | :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`                   | `boolean`                                             | `false`     | 명령 실행을 위한 샌드박스 모드 활성화                                                                                                                                                  |
| `failIfUnavailable`         | `boolean`                                             | `true`      | `enabled`가 `true`이지만 샌드박스를 시작할 수 없는 경우 시작 시 중지합니다. stderr에 경고와 함께 샌드박스되지 않은 실행으로 폴백하려면 `false`로 설정합니다                                                                  |
| `autoAllowBashIfSandboxed`  | `boolean`                                             | `true`      | 샌드박스가 활성화되었을 때 bash 명령 자동 승인                                                                                                                                           |
| `excludedCommands`          | `string[]`                                            | `[]`        | 항상 샌드박스 제한을 무시하는 명령 (예: `['docker']`). 이들은 모델 개입 없이 자동으로 샌드박스되지 않은 상태로 실행됩니다                                                                                           |
| `allowUnsandboxedCommands`  | `boolean`                                             | `true`      | 모델이 샌드박스 외부에서 명령을 실행하도록 요청하도록 허용합니다. `true`일 때 모델은 도구 입력에서 `dangerouslyDisableSandbox`를 설정할 수 있으며, 이는 [권한 시스템](#permissions-fallback-for-unsandboxed-commands)으로 폴백됩니다 |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `undefined` | 네트워크 특정 샌드박스 구성                                                                                                                                                        |
| `filesystem`                | [`SandboxFilesystemConfig`](#sandboxfilesystemconfig) | `undefined` | 읽기/쓰기 제한을 위한 파일 시스템 특정 샌드박스 구성                                                                                                                                         |
| `ignoreViolations`          | `Record<string, string[]>`                            | `undefined` | 무시할 위반 카테고리를 패턴에 매핑합니다 (예: `{ file: ['/tmp/*'], network: ['localhost'] }`)                                                                                             |
| `enableWeakerNestedSandbox` | `boolean`                                             | `false`     | 호환성을 위해 더 약한 중첩 샌드박스 활성화                                                                                                                                               |
| `ripgrep`                   | `{ command: string; args?: string[] }`                | `undefined` | 샌드박스 환경을 위한 사용자 정의 ripgrep 바이너리 구성                                                                                                                                     |

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
  **Unix 소켓 보안:** `allowUnixSockets` 옵션은 강력한 시스템 서비스에 대한 액세스를 부여할 수 있습니다. 예를 들어 `/var/run/docker.sock`을 허용하면 Docker API를 통해 전체 호스트 시스템 액세스를 효과적으로 부여하여 샌드박스 격리를 무시합니다. 엄격히 필요한 Unix 소켓만 허용하고 각각의 보안 영향을 이해합니다.
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

샌드박스 모드를 위한 네트워크 특정 구성입니다. 이러한 설정은 부모 [`SandboxSettings`](#sandboxsettings)에서 `enabled`가 `true`일 때 샌드박스된 Bash 명령에 적용됩니다. 이들은 [권한 규칙](/docs/ko/permissions#webfetch)을 대신 사용하는 WebFetch 도구를 제한하지 않습니다.

```typescript theme={null}
type SandboxNetworkConfig = {
  allowedDomains?: string[];
  deniedDomains?: string[];
  allowManagedDomainsOnly?: boolean;
  allowLocalBinding?: boolean;
  allowUnixSockets?: string[];
  allowAllUnixSockets?: boolean;
  httpProxyPort?: number;
  socksProxyPort?: number;
};
```

| 속성                        | 타입         | 기본값         | 설명                                                                                                                                                               |
| :------------------------ | :--------- | :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedDomains`          | `string[]` | `[]`        | 샌드박스된 프로세스가 액세스할 수 있는 도메인 이름                                                                                                                                     |
| `deniedDomains`           | `string[]` | `[]`        | 샌드박스된 프로세스가 액세스할 수 없는 도메인 이름입니다. `allowedDomains`보다 우선합니다                                                                                                        |
| `allowManagedDomainsOnly` | `boolean`  | `false`     | 관리형 설정 전용입니다. [관리형 설정](/docs/ko/permissions#managed-settings)에서 설정되었을 때 관리형 설정의 `allowedDomains` 항목만 적용되며 사용자, 프로젝트 또는 로컬 설정의 항목은 무시됩니다. SDK 옵션을 통해 설정되었을 때는 효과가 없습니다 |
| `allowLocalBinding`       | `boolean`  | `false`     | 프로세스가 로컬 포트에 바인딩하도록 허용합니다 (예: 개발 서버의 경우)                                                                                                                         |
| `allowUnixSockets`        | `string[]` | `[]`        | 프로세스가 액세스할 수 있는 Unix 소켓 경로 (예: Docker 소켓)                                                                                                                        |
| `allowAllUnixSockets`     | `boolean`  | `false`     | 모든 Unix 소켓에 대한 액세스 허용                                                                                                                                            |
| `httpProxyPort`           | `number`   | `undefined` | 네트워크 요청을 위한 HTTP 프록시 포트                                                                                                                                          |
| `socksProxyPort`          | `number`   | `undefined` | 네트워크 요청을 위한 SOCKS 프록시 포트                                                                                                                                         |

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

`allowUnsandboxedCommands`가 활성화되었을 때 모델은 도구 입력에서 `dangerouslyDisableSandbox: true`를 설정하여 샌드박스 외부에서 명령을 실행하도록 요청할 수 있습니다. 이러한 요청은 기존 권한 시스템으로 폴백되므로 `canUseTool` 핸들러가 호출되어 사용자 정의 인증 로직을 구현할 수 있습니다. 아래 예제에서 `isCommandAuthorized`는 사용자가 정의하는 인증 확인을 나타냅니다.

<Note>
  **`excludedCommands` vs `allowUnsandboxedCommands`:**

  * `excludedCommands`: 항상 자동으로 샌드박스를 무시하는 명령의 정적 목록 (예: `['docker']`). 모델은 이에 대한 제어가 없습니다.
  * `allowUnsandboxedCommands`: 모델이 도구 입력에서 `dangerouslyDisableSandbox: true`를 설정하여 런타임에 샌드박스되지 않은 실행을 요청하도록 합니다.
</Note>

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

이 패턴을 사용하면 다음을 수행할 수 있습니다:

* **모델 요청 감사:** 모델이 샌드박스되지 않은 실행을 요청할 때 로그합니다
* **허용 목록 구현:** 특정 명령만 샌드박스되지 않은 상태로 실행하도록 허용합니다
* **승인 워크플로우 추가:** 권한 있는 작업에 대한 명시적 인증이 필요합니다

<Warning>
  `dangerouslyDisableSandbox: true`로 실행되는 명령은 전체 시스템 액세스 권한이 있습니다. `canUseTool` 핸들러가 이러한 요청을 신중하게 검증하는지 확인합니다.

  `permissionMode`가 `bypassPermissions`로 설정되고 `allowUnsandboxedCommands`가 활성화되면 모델은 승인 프롬프트 없이 샌드박스 외부에서 명령을 자율적으로 실행할 수 있습니다 (명시적 [`ask` 규칙](/docs/ko/agent-sdk/permissions#how-permissions-are-evaluated)은 여전히 하나를 강제합니다). 이 조합은 모델이 샌드박스 격리를 조용히 탈출하도록 효과적으로 허용합니다.
</Warning>

<h2 id="see-also">
  참고 항목
</h2>

* [SDK 개요](/docs/ko/agent-sdk/overview) - 일반 SDK 개념
* [Python SDK 참조](/docs/ko/agent-sdk/python) - Python SDK 문서
* [CLI 참조](/docs/ko/cli-reference) - 명령줄 인터페이스
* [일반적인 워크플로우](/docs/ko/common-workflows) - 단계별 가이드
