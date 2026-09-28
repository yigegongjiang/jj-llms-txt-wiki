> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude에 사용자 정의 도구 제공

> Claude Agent SDK의 인프로세스 MCP 서버로 사용자 정의 도구를 정의하여 Claude가 함수를 호출하고, API를 사용하며, 도메인별 작업을 수행할 수 있도록 합니다.

사용자 정의 도구는 Claude가 대화 중에 호출할 수 있는 자신의 함수를 정의하도록 하여 Agent SDK를 확장합니다. SDK의 인프로세스 MCP 서버를 사용하면 Claude에 데이터베이스, 외부 API, 도메인별 로직 또는 애플리케이션에 필요한 다른 기능에 대한 액세스 권한을 부여할 수 있습니다.

<h2 id="quick-reference">
  빠른 참조
</h2>

| 원하는 작업                     | 수행 방법                                                                                                                                                                                  |
| :------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 도구 정의                      | 이름, 설명, 스키마 및 핸들러를 사용하여 [`@tool`](/docs/ko/agent-sdk/python#tool) (Python) 또는 [`tool()`](/docs/ko/agent-sdk/typescript#tool) (TypeScript)을 사용합니다. [사용자 정의 도구 만들기](#create-a-custom-tool)를 참조하세요. |
| Claude에 도구 등록              | `create_sdk_mcp_server` / `createSdkMcpServer`로 래핑하고 `query()`의 `mcpServers`에 전달합니다. [사용자 정의 도구 호출](#call-a-custom-tool)을 참조하세요.                                                       |
| 도구 사전 승인                   | 허용된 도구에 추가합니다. [허용된 도구 구성](#configure-allowed-tools)을 참조하세요.                                                                                                                           |
| Claude의 컨텍스트에서 기본 제공 도구 제거 | 원하는 기본 제공 도구만 나열하는 `tools` 배열을 전달합니다. [허용된 도구 구성](#configure-allowed-tools)을 참조하세요.                                                                                                    |
| Claude가 도구를 병렬로 호출하도록 허용   | 부작용이 없는 도구에 `readOnlyHint: true`를 설정합니다. [도구 주석 추가](#add-tool-annotations)를 참조하세요.                                                                                                     |
| Claude가 읽는 오류 메시지 제어       | 원시 예외를 표시하는 대신 메시지를 작성하려면 `isError: true`를 반환합니다. [오류 처리](#handle-errors)를 참조하세요.                                                                                                      |
| 이미지 또는 파일 반환               | 콘텐츠 배열에서 `image` 또는 `resource` 블록을 사용합니다. [이미지 및 리소스 반환](#return-images-and-resources)을 참조하세요.                                                                                         |
| 머신 판독 가능한 JSON 결과 반환       | 결과에 `structuredContent`를 설정합니다. [구조화된 데이터 반환](#return-structured-data)을 참조하세요.                                                                                                         |
| 많은 도구로 확장                  | [도구 검색](/docs/ko/agent-sdk/tool-search)을 사용하여 필요에 따라 도구를 로드합니다.                                                                                                                             |

<h2 id="create-a-custom-tool">
  사용자 정의 도구 만들기
</h2>

도구는 TypeScript의 [`tool()`](/docs/ko/agent-sdk/typescript#tool) 헬퍼 또는 Python의 [`@tool`](/docs/ko/agent-sdk/python#tool) 데코레이터에 인수로 전달되는 네 가지 부분으로 정의됩니다:

* **이름:** Claude가 도구를 호출할 때 사용하는 고유 식별자입니다.
* **설명:** 도구가 수행하는 작업입니다. Claude는 이를 읽고 도구를 호출할 시기를 결정합니다.
* **입력 스키마:** Claude가 제공해야 하는 인수입니다. TypeScript에서는 항상 [Zod 스키마](https://zod.dev/)이며, 핸들러의 `args`는 자동으로 입력됩니다. Python에서는 `{"latitude": float}`와 같이 이름을 타입에 매핑하는 딕셔너리이며, SDK가 JSON 스키마로 변환합니다. Python 데코레이터는 열거형, 범위, 선택적 필드 또는 중첩된 객체가 필요할 때 전체 [JSON 스키마](https://json-schema.org/understanding-json-schema/about) 딕셔너리도 허용합니다.
* **핸들러:** Claude가 도구를 호출할 때 실행되는 비동기 함수입니다. 검증된 인수를 받으며 다음을 포함하는 객체를 반환해야 합니다:
  * `content` (필수): 각각 `"text"`, `"image"`, `"audio"`, `"resource"` 또는 `"resource_link"`의 `type`을 가진 결과 블록의 배열입니다. 텍스트가 아닌 블록은 [이미지 및 리소스 반환](#return-images-and-resources)을 참조하세요.
  * `structuredContent` (선택사항): 결과를 기계 판독 가능한 데이터로 보유하는 JSON 객체이며, `content`와 함께 반환됩니다. [구조화된 데이터 반환](#return-structured-data)을 참조하세요.
  * `isError` (선택사항): 도구 실패를 신호하려면 `true`로 설정하여 Claude가 이에 반응할 수 있도록 합니다. [오류 처리](#handle-errors)를 참조하세요.

도구를 정의한 후 [`createSdkMcpServer`](/docs/ko/agent-sdk/typescript#createsdkmcpserver) (TypeScript) 또는 [`create_sdk_mcp_server`](/docs/ko/agent-sdk/python#create_sdk_mcp_server) (Python)를 사용하여 서버에 래핑합니다. 서버는 별도의 프로세스가 아닌 애플리케이션 내에서 인프로세스로 실행됩니다.

<h3 id="weather-tool-example">
  날씨 도구 예제
</h3>

이 예제는 `get_temperature` 도구를 정의하고 MCP 서버에 래핑합니다. 도구만 설정합니다. `query`에 전달하고 실행하려면 아래의 [사용자 정의 도구 호출](#call-a-custom-tool)을 참조하세요.

<CodeGroup>
  ```python Python theme={null}
  from typing import Any
  import httpx
  from claude_agent_sdk import tool, create_sdk_mcp_server


  # Define a tool: name, description, input schema, handler
  @tool(
      "get_temperature",
      "Get the current temperature at a location",
      {"latitude": float, "longitude": float},
  )
  async def get_temperature(args: dict[str, Any]) -> dict[str, Any]:
      async with httpx.AsyncClient() as client:
          response = await client.get(
              "https://api.open-meteo.com/v1/forecast",
              params={
                  "latitude": args["latitude"],
                  "longitude": args["longitude"],
                  "current": "temperature_2m",
                  "temperature_unit": "fahrenheit",
              },
          )
          data = response.json()

      # Return a content array - Claude sees this as the tool result
      return {
          "content": [
              {
                  "type": "text",
                  "text": f"Temperature: {data['current']['temperature_2m']}°F",
              }
          ]
      }


  # Wrap the tool in an in-process MCP server
  weather_server = create_sdk_mcp_server(
      name="weather",
      version="1.0.0",
      tools=[get_temperature],
  )
  ```

  ```typescript TypeScript theme={null}
  import { tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  // Define a tool: name, description, input schema, handler
  const getTemperature = tool(
    "get_temperature",
    "Get the current temperature at a location",
    {
      latitude: z.number().describe("Latitude coordinate"), // .describe() adds a field description Claude sees
      longitude: z.number().describe("Longitude coordinate")
    },
    async (args) => {
      // args is typed from the schema: { latitude: number; longitude: number }
      const response = await fetch(
        `https://api.open-meteo.com/v1/forecast?latitude=${args.latitude}&longitude=${args.longitude}&current=temperature_2m&temperature_unit=fahrenheit`
      );
      const data: any = await response.json();

      // Return a content array - Claude sees this as the tool result
      return {
        content: [{ type: "text", text: `Temperature: ${data.current.temperature_2m}°F` }]
      };
    }
  );

  // Wrap the tool in an in-process MCP server
  const weatherServer = createSdkMcpServer({
    name: "weather",
    version: "1.0.0",
    tools: [getTemperature]
  });
  ```
</CodeGroup>

전체 매개변수 세부 정보(JSON 스키마 입력 형식 및 반환 값 구조 포함)는 [`tool()`](/docs/ko/agent-sdk/typescript#tool) TypeScript 참조 또는 [`@tool`](/docs/ko/agent-sdk/python#tool) Python 참조를 참조하세요.

<Tip>
  매개변수를 선택사항으로 만들려면: TypeScript에서 Zod 필드에 `.default()`를 추가합니다. Python에서는 딕셔너리 스키마가 모든 키를 필수로 취급하므로 스키마에서 매개변수를 생략하고, 설명 문자열에서 언급하고, 핸들러에서 `args.get()`으로 읽습니다. 아래의 [`get_precipitation_chance` 도구](#add-more-tools)는 두 패턴을 모두 보여줍니다.
</Tip>

<h3 id="call-a-custom-tool">
  사용자 정의 도구 호출
</h3>

`mcpServers` 옵션을 통해 생성한 MCP 서버를 `query`에 전달합니다. `mcpServers`의 키는 각 도구의 정규화된 이름에서 `{server_name}` 세그먼트가 됩니다: `mcp__{server_name}__{tool_name}`. 도구가 권한 프롬프트 없이 실행되도록 `allowedTools`에 해당 이름을 나열합니다.

이 스니펫들은 [날씨 도구 예제](#weather-tool-example)의 `weatherServer`를 재사용하여 Claude에게 특정 위치의 날씨를 묻습니다.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={"weather": weather_server},
          allowed_tools=["mcp__weather__get_temperature"],
      )

      async for message in query(
          prompt="What's the temperature in San Francisco?",
          options=options,
      ):
          # ResultMessage is the final message after all tool calls complete
          if isinstance(message, ResultMessage) and message.subtype == "success":
              print(message.result)


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  for await (const message of query({
    prompt: "What's the temperature in San Francisco?",
    options: {
      mcpServers: { weather: weatherServer },
      allowedTools: ["mcp__weather__get_temperature"]
    }
  })) {
    // "result" is the final message after all tool calls complete
    if (message.type === "result" && message.subtype === "success") {
      console.log(message.result);
    }
  }
  ```
</CodeGroup>

이 스니펫을 [날씨 도구 예제](#weather-tool-example)의 도구 및 서버 정의와 함께 한 파일에 결합한 후 Python의 경우 `python weather.py`로, TypeScript의 경우 `npx tsx weather.ts`로 실행합니다. Claude는 `get_temperature`를 호출하고 스크립트는 샌프란시스코의 현재 온도를 포함한 한 줄 답변을 출력합니다.

<h3 id="add-more-tools">
  더 많은 도구 추가
</h3>

서버는 `tools` 배열에 나열한 만큼 많은 도구를 보유합니다. 서버에 둘 이상의 도구가 있으면 `allowedTools`에서 각각을 개별적으로 나열하거나 와일드카드 `mcp__weather__*`를 사용하여 서버가 노출하는 모든 도구를 포함할 수 있습니다.

아래 예제는 두 번째 도구 `get_precipitation_chance`를 정의하고 [날씨 도구 예제](#weather-tool-example)의 `weatherServer` 정의를 배열의 두 도구를 모두 나열하는 것으로 바꿉니다.

<CodeGroup>
  ```python Python theme={null}
  # Define a second tool for the same server
  @tool(
      "get_precipitation_chance",
      "Get the hourly precipitation probability for a location. "
      "Optionally pass 'hours' (1-24) to control how many hours to return.",
      {"latitude": float, "longitude": float},
  )
  async def get_precipitation_chance(args: dict[str, Any]) -> dict[str, Any]:
      # 'hours' isn't in the schema - read it with .get() to make it optional
      hours = args.get("hours", 12)
      async with httpx.AsyncClient() as client:
          response = await client.get(
              "https://api.open-meteo.com/v1/forecast",
              params={
                  "latitude": args["latitude"],
                  "longitude": args["longitude"],
                  "hourly": "precipitation_probability",
                  "forecast_days": 1,
              },
          )
          data = response.json()
      chances = data["hourly"]["precipitation_probability"][:hours]

      return {
          "content": [
              {
                  "type": "text",
                  "text": f"Next {hours} hours: {'%, '.join(map(str, chances))}%",
              }
          ]
      }


  # Rebuild the server with both tools in the array
  weather_server = create_sdk_mcp_server(
      name="weather",
      version="1.0.0",
      tools=[get_temperature, get_precipitation_chance],
  )
  ```

  ```typescript TypeScript theme={null}
  // Define a second tool for the same server
  const getPrecipitationChance = tool(
    "get_precipitation_chance",
    "Get the hourly precipitation probability for a location",
    {
      latitude: z.number(),
      longitude: z.number(),
      hours: z
        .number()
        .int()
        .min(1)
        .max(24)
        .default(12) // .default() makes the parameter optional
        .describe("How many hours of forecast to return")
    },
    async (args) => {
      const response = await fetch(
        `https://api.open-meteo.com/v1/forecast?latitude=${args.latitude}&longitude=${args.longitude}&hourly=precipitation_probability&forecast_days=1`
      );
      const data: any = await response.json();
      const chances = data.hourly.precipitation_probability.slice(0, args.hours);

      return {
        content: [{ type: "text", text: `Next ${args.hours} hours: ${chances.join("%, ")}%` }]
      };
    }
  );

  // Rebuild the server with both tools in the array
  const weatherServer = createSdkMcpServer({
    name: "weather",
    version: "1.0.0",
    tools: [getTemperature, getPrecipitationChance]
  });
  ```
</CodeGroup>

[도구 검색](/docs/ko/agent-sdk/tool-search)은 기본적으로 활성화되어 있으며 SDK MCP 도구를 연기합니다: Claude는 각 도구의 이름을 간단한 목록으로 보고 필요에 따라 전체 스키마를 로드합니다. 도구 검색이 비활성화되면 이 배열의 모든 도구는 매 턴마다 컨텍스트 윈도우 공간을 소비합니다. TypeScript에서는 [`tool()`](/docs/ko/agent-sdk/typescript#tool)의 `extras` 인수 또는 [`createSdkMcpServer()`](/docs/ko/agent-sdk/typescript#createsdkmcpserver)의 옵션에서 `alwaysLoad: true`를 전달하여 도구의 전체 스키마를 초기 프롬프트에 유지합니다.

<h3 id="add-tool-annotations">
  도구 주석 추가
</h3>

[도구 주석](https://modelcontextprotocol.io/docs/concepts/tools#tool-annotations)은 도구의 동작을 설명하는 선택사항 메타데이터입니다. TypeScript의 `tool()` 헬퍼에 다섯 번째 인수로 또는 Python의 `@tool` 데코레이터에 대해 `annotations` 키워드 인수로 전달합니다. 모든 힌트 필드는 부울입니다.

| 필드                | 기본값     | 의미                                                           |
| :---------------- | :------ | :----------------------------------------------------------- |
| `readOnlyHint`    | `false` | 도구는 환경을 수정하지 않습니다. 도구를 다른 읽기 전용 도구와 병렬로 호출할 수 있는지 여부를 제어합니다. |
| `destructiveHint` | `true`  | 도구는 파괴적인 업데이트를 수행할 수 있습니다. 정보 제공용입니다.                        |
| `idempotentHint`  | `false` | 동일한 인수로 반복 호출해도 추가 효과가 없습니다. 정보 제공용입니다.                      |
| `openWorldHint`   | `true`  | 도구는 프로세스 외부의 시스템에 도달합니다. 정보 제공용입니다.                          |

주석은 메타데이터이지 강제 사항이 아닙니다. `readOnlyHint: true`로 표시된 도구도 핸들러가 그렇게 하면 디스크에 쓸 수 있습니다. 주석을 핸들러와 정확하게 유지하세요.

이 예제는 [날씨 도구 예제](#weather-tool-example)의 `get_temperature` 도구에 `readOnlyHint`를 추가합니다.

<CodeGroup>
  ```python Python theme={null}
  from claude_agent_sdk import tool, ToolAnnotations


  @tool(
      "get_temperature",
      "Get the current temperature at a location",
      {"latitude": float, "longitude": float},
      annotations=ToolAnnotations(
          readOnlyHint=True
      ),  # Lets Claude batch this with other read-only calls
  )
  async def get_temperature(args):
      return {"content": [{"type": "text", "text": "..."}]}
  ```

  ```typescript TypeScript theme={null}
  import { tool } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  tool(
    "get_temperature",
    "Get the current temperature at a location",
    { latitude: z.number(), longitude: z.number() },
    async (args) => ({ content: [{ type: "text", text: `...` }] }),
    { annotations: { readOnlyHint: true } } // Lets Claude batch this with other read-only calls
  );
  ```
</CodeGroup>

[TypeScript](/docs/ko/agent-sdk/typescript#toolannotations) 또는 [Python](/docs/ko/agent-sdk/python#toolannotations) 참조에서 `ToolAnnotations`를 참조하세요.

<h2 id="control-tool-access">
  도구 접근 제어
</h2>

[날씨 도구 예제](#weather-tool-example)에서 등록한 서버는 `allowedTools`에 도구를 나열했습니다. 이 섹션에서는 여러 도구가 있거나 기본 제공 도구를 제한하려는 경우 접근 범위를 지정하는 방법을 다룹니다. 도구 이름이 구성되는 방식에 대해서는 [사용자 정의 도구 호출](#call-a-custom-tool)을 참조하십시오.

<h3 id="configure-allowed-tools">
  허용된 도구 구성
</h3>

`tools` 옵션과 허용/거부 목록은 두 가지 계층에 영향을 미칩니다. 가용성은 Claude의 컨텍스트에 도구가 표시되는지 여부를 제어하고, 권한은 Claude가 호출을 시도한 후 호출이 승인되는지 여부를 제어합니다. `tools`와 단순 이름 `disallowedTools` 항목은 가용성을 변경합니다. `allowedTools`와 범위가 지정된 `disallowedTools` 규칙은 권한을 변경합니다. [작업 추적 도구](/docs/ko/agent-sdk/todo-tracking#model-availability) 중 하나를 `allowedTools`에 이름 지정하면 Claude Code도 세션을 옵트인합니다.

| 옵션                        | 계층  | 효과                                                                                                                                                                               |
| :------------------------ | :-- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tools: ["Read", "Grep"]` | 가용성 | 나열된 기본 제공 도구만 Claude의 컨텍스트에 있습니다. 나열되지 않은 기본 제공 도구는 제거됩니다. MCP 도구는 영향을 받지 않습니다.                                                                                                  |
| `tools: []`               | 가용성 | 모든 기본 제공 도구가 제거됩니다. Claude는 MCP 도구만 사용할 수 있습니다.                                                                                                                                  |
| 허용된 도구                    | 권한  | 나열된 도구는 권한 프롬프트 없이 실행됩니다. 다른 나열되지 않은 도구는 계속 사용 가능하며, 호출은 [권한 흐름](/docs/ko/agent-sdk/permissions)을 거칩니다.                                                                               |
| 거부된 도구                    | 둘 다 | `"Bash"`와 같은 단순 도구 이름은 도구를 Claude의 컨텍스트에서 제거하며, `tools`에서 생략하는 것과 동일합니다. `"Bash(rm *)"` 같은 범위가 지정된 규칙은 도구를 컨텍스트에 남겨두고 [작성된 대로](/docs/ko/permissions#bash-rule-limits) 일치하는 호출만 거부합니다. |

기본 제공 도구를 완전히 제거하려면 `tools`에서 생략하거나 `disallowedTools`(Python: `disallowed_tools`)에 단순 이름을 나열하십시오. 둘 다 도구를 컨텍스트 밖에 유지하므로 Claude는 절대 시도하지 않습니다. 범위가 지정된 `disallowedTools` 규칙은 일치하는 호출을 차단하지만 도구를 표시된 상태로 유지하므로 Claude가 시도하는 데 턴을 낭비할 수 있습니다. 전체 평가 순서는 [권한 구성](/docs/ko/agent-sdk/permissions)을 참조하십시오.

<h2 id="handle-errors">
  오류 처리
</h2>

핸들러 오류는 에이전트 루프를 중단하지 않습니다. SDK의 인프로세스 MCP 서버는 포착되지 않은 예외를 캐치하고 오류 결과로 반환하므로, 오류를 보고하는 방식이 Claude가 읽는 내용을 결정하며, 쿼리 실패 여부는 결정하지 않습니다.

| 발생하는 상황                                                               | 결과                                                                                    |
| :-------------------------------------------------------------------- | :------------------------------------------------------------------------------------ |
| 핸들러가 포착되지 않은 예외를 발생시킴                                                 | MCP 서버는 이를 원본 예외 메시지를 포함하는 오류 결과로 변환합니다. Claude는 해당 메시지를 보고 에이전트 루프가 계속됩니다.           |
| 핸들러가 오류를 캐치하고 `isError: true` (TS) / `"is_error": True` (Python)을 반환함 | Claude는 사용자가 작성한 메시지를 봅니다. 원본 예외가 부족한 컨텍스트(예: 어떤 요청이 실패했는지 또는 대신 시도할 사항)를 추가할 수 있습니다. |

두 경우 모두 Claude는 재시도하거나, 다른 도구를 시도하거나, 실패를 설명할 수 있습니다. 원본 예외 메시지가 Claude가 작용하기에 충분하지 않을 때 오류를 직접 캐치합니다.

아래 예제는 핸들러 내에서 두 가지 종류의 실패를 캐치하고 Claude가 읽는 오류 메시지를 작성합니다. 200이 아닌 HTTP 상태는 응답에서 캐치되어 오류 결과로 반환됩니다. 네트워크 오류 또는 잘못된 JSON은 주변 `try/except` (Python) 또는 `try/catch` (TypeScript)에 의해 캐치되어 오류 결과로도 반환됩니다. 두 경우 모두 Claude는 단순한 예외 문자열 대신 실패를 설명하는 메시지를 받습니다.

<CodeGroup>
  ```python Python theme={null}
  import json
  import httpx
  from typing import Any
  from claude_agent_sdk import tool


  @tool(
      "fetch_data",
      "Fetch data from an API",
      {"endpoint": str},  # Simple schema
  )
  async def fetch_data(args: dict[str, Any]) -> dict[str, Any]:
      try:
          async with httpx.AsyncClient() as client:
              response = await client.get(args["endpoint"])
              if response.status_code != 200:
                  # Return the failure as a tool result so Claude can react to it.
                  # is_error marks this as a failed call rather than odd-looking data.
                  return {
                      "content": [
                          {
                              "type": "text",
                              "text": f"API error: {response.status_code} {response.reason_phrase}",
                          }
                      ],
                      "is_error": True,
                  }

              data = response.json()
              return {"content": [{"type": "text", "text": json.dumps(data, indent=2)}]}
      except Exception as e:
          # Composes the message Claude reads. An uncaught exception would
          # reach Claude as the raw str(e) with no context.
          return {
              "content": [{"type": "text", "text": f"Failed to fetch data: {str(e)}"}],
              "is_error": True,
          }
  ```

  ```typescript TypeScript theme={null}
  import { tool } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  tool(
    "fetch_data",
    "Fetch data from an API",
    {
      endpoint: z.string().url().describe("API endpoint URL")
    },
    async (args) => {
      try {
        const response = await fetch(args.endpoint);

        if (!response.ok) {
          // Return the failure as a tool result so Claude can react to it.
          // isError marks this as a failed call rather than odd-looking data.
          return {
            content: [
              {
                type: "text",
                text: `API error: ${response.status} ${response.statusText}`
              }
            ],
            isError: true
          };
        }

        const data = await response.json();
        return {
          content: [
            {
              type: "text",
              text: JSON.stringify(data, null, 2)
            }
          ]
        };
      } catch (error) {
        // Composes the message Claude reads. An uncaught throw would
        // reach Claude as the raw error message with no context.
        return {
          content: [
            {
              type: "text",
              text: `Failed to fetch data: ${error instanceof Error ? error.message : String(error)}`
            }
          ],
          isError: true
        };
      }
    }
  );
  ```
</CodeGroup>

<h2 id="return-images-and-resources">
  이미지 및 리소스 반환
</h2>

도구 결과의 `content` 배열은 `text`, `image`, `audio`, `resource`, `resource_link` 블록을 허용합니다. 동일한 응답에서 이들을 혼합할 수 있습니다. TypeScript에서 SDK는 오디오 블록을 디스크에 저장하고 Claude는 저장된 파일 경로가 포함된 텍스트 블록을 수신합니다. Python에서는 SDK가 도구 결과에서 오디오 블록을 제거하고 경고를 기록합니다.

Claude는 각 리소스 링크 블록을 링크의 이름, URI, 설명이 포함된 텍스트 블록으로 수신합니다. TypeScript에서 애플리케이션은 사용자 메시지의 `tool_use_result`에서 [`resourceLinks`](/docs/ko/agent-sdk/typescript#sdkmcpresourcelink)로 링크 자체를 수신합니다. Python에서는 SDK가 CLI에서 결과를 보기 전에 이들을 텍스트로 평탄화하므로, Python [`resourceLinks` 키](/docs/ko/agent-sdk/python#usermessage)는 프로세스 내 도구에 대해 생성되지 않습니다.

<h3 id="images">
  이미지
</h3>

이미지 블록은 이미지 바이트를 인라인으로 base64로 인코딩하여 전달합니다. URL 필드는 없습니다. URL에 있는 이미지를 반환하려면 핸들러에서 이를 가져오고, 응답 바이트를 읽은 후 base64로 인코딩하여 반환합니다. 결과는 시각적 입력으로 처리됩니다.

| 필드         | 유형        | 참고                                                                       |
| :--------- | :-------- | :----------------------------------------------------------------------- |
| `type`     | `"image"` |                                                                          |
| `data`     | `string`  | Base64로 인코딩된 바이트입니다. 원본 base64만 해당하며, `data:image/...;base64,` 접두사는 없습니다 |
| `mimeType` | `string`  | 필수입니다. 예: `image/png`, `image/jpeg`, `image/webp`, `image/gif`           |

<CodeGroup>
  ```python Python theme={null}
  import base64
  import httpx
  from claude_agent_sdk import tool


  # URL에서 이미지를 가져와 Claude에 반환하는 도구를 정의합니다
  @tool("fetch_image", "Fetch an image from a URL and return it to Claude", {"url": str})
  async def fetch_image(args):
      async with httpx.AsyncClient() as client:  # 이미지 바이트를 가져옵니다
          response = await client.get(args["url"])

      return {
          "content": [
              {
                  "type": "image",
                  "data": base64.b64encode(response.content).decode(
                      "ascii"
                  ),  # 원본 바이트를 base64로 인코딩합니다
                  "mimeType": response.headers.get(
                      "content-type", "image/png"
                  ),  # 응답에서 MIME 유형을 읽습니다
              }
          ]
      }
  ```

  ```typescript TypeScript theme={null}
  import { tool } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  tool(
    "fetch_image",
    "Fetch an image from a URL and return it to Claude",
    {
      url: z.string().url()
    },
    async (args) => {
      const response = await fetch(args.url); // 이미지 바이트를 가져옵니다
      const buffer = Buffer.from(await response.arrayBuffer()); // base64 인코딩을 위해 버퍼로 읽습니다
      const mimeType = response.headers.get("content-type") ?? "image/png";

      return {
        content: [
          {
            type: "image",
            data: buffer.toString("base64"), // 원본 바이트를 base64로 인코딩합니다
            mimeType
          }
        ]
      };
    }
  );
  ```
</CodeGroup>

<h3 id="resources">
  리소스
</h3>

리소스 블록은 URI로 식별되는 콘텐츠를 포함합니다. URI는 Claude가 참조할 수 있는 레이블이며, 실제 콘텐츠는 블록의 `text` 또는 `blob` 필드에 있습니다. 생성된 파일이나 외부 시스템의 레코드와 같이 나중에 이름으로 참조하는 것이 합리적인 도구 출력에 사용합니다.

| 필드                  | 유형           | 참고                                                                                         |
| :------------------ | :----------- | :----------------------------------------------------------------------------------------- |
| `type`              | `"resource"` |                                                                                            |
| `resource.uri`      | `string`     | 콘텐츠의 식별자입니다. 모든 URI 스키마                                                                    |
| `resource.text`     | `string`     | 텍스트인 경우 콘텐츠입니다. 이것 또는 `blob`을 제공하되, 둘 다 제공하지는 마십시오                                         |
| `resource.blob`     | `string`     | 바이너리인 경우 base64로 인코딩된 콘텐츠입니다. TypeScript만 해당: Python SDK는 도구 결과에서 바이너리 리소스를 제거하고 경고를 기록합니다 |
| `resource.mimeType` | `string`     | 선택사항                                                                                       |

이 예제는 도구 핸들러 내부에서 반환된 리소스 블록을 보여줍니다. URI `file:///tmp/report.md`는 Claude가 나중에 참조할 수 있는 레이블입니다. SDK는 해당 경로에서 읽지 않습니다.

<CodeGroup>
  ```typescript TypeScript theme={null}
  return {
    content: [
      {
        type: "resource",
        resource: {
          uri: "file:///tmp/report.md", // Claude가 참조할 수 있는 레이블이며, SDK가 읽는 경로가 아닙니다
          mimeType: "text/markdown",
          text: "# Report\n..." // 실제 콘텐츠이며, 인라인입니다
        }
      }
    ]
  };
  ```

  ```python Python theme={null}
  return {
      "content": [
          {
              "type": "resource",
              "resource": {
                  "uri": "file:///tmp/report.md",  # Claude가 참조할 수 있는 레이블이며, SDK가 읽는 경로가 아닙니다
                  "mimeType": "text/markdown",
                  "text": "# Report\n...",  # 실제 콘텐츠이며, 인라인입니다
              },
          }
      ]
  }
  ```
</CodeGroup>

이러한 블록 형태는 MCP `CallToolResult` 유형에서 나옵니다. 전체 정의는 [MCP 사양](https://modelcontextprotocol.io/specification/2025-06-18/server/tools#tool-result)을 참조하십시오.

<h2 id="return-structured-data">
  구조화된 데이터 반환
</h2>

`structuredContent`는 결과의 선택적 JSON 객체로, `content` 배열과 별도입니다. Claude가 텍스트 문자열이나 이미지에서 구문 분석하는 대신 정확한 필드로 읽을 수 있는 원본 값을 반환하는 데 사용합니다.

`structuredContent`가 설정되면 Claude는 JSON과 `content`의 모든 이미지 또는 리소스 블록을 수신합니다. `content`의 텍스트 블록은 구조화된 데이터를 복제한다고 가정하므로 전달되지 않습니다. 아래 예제는 차트를 이미지 블록으로 렌더링하고 동일한 핸들러의 `structuredContent`에서 뒤에 있는 데이터 포인트를 반환합니다. 스니펫에서 `chartPngBuffer`는 렌더링된 PNG 바이트를 보유하는 `Buffer`입니다.

```typescript TypeScript theme={null}
return {
  content: [
    {
      type: "image",
      data: chartPngBuffer.toString("base64"),
      mimeType: "image/png"
    }
  ],
  structuredContent: {
    series: "temperature_2m",
    unit: "fahrenheit",
    points: [62.1, 63.4, 65.0, 64.2]
  }
};
```

<Note>
  Python `@tool` 데코레이터는 핸들러의 반환 딕셔너리에서 `content`와 `is_error`만 전달합니다. Python에서 `structuredContent`를 반환하려면 in-process SDK 서버 대신 [독립형 MCP 서버](/docs/ko/agent-sdk/mcp)를 실행합니다.
</Note>

<h2 id="example-unit-converter">
  예제: 단위 변환기
</h2>

이 도구는 길이, 온도, 무게 단위 간의 값을 변환합니다. 사용자가 "100킬로미터를 마일로 변환해줘" 또는 "72°F는 섭씨온도로 몇 도야?"라고 물어보면, Claude가 요청에서 올바른 단위 유형과 단위를 선택합니다.

두 가지 패턴을 보여줍니다:

* **Enum 스키마:** `unit_type`은 고정된 값 집합으로 제한됩니다. TypeScript에서는 `z.enum()`을 사용합니다. Python에서는 dict 스키마가 enum을 지원하지 않으므로 전체 JSON Schema dict가 필요합니다.
* **지원되지 않는 입력 처리:** 변환 쌍을 찾을 수 없을 때, 핸들러는 `isError: true`를 반환하므로 Claude가 실패를 일반적인 결과로 취급하는 대신 사용자에게 무엇이 잘못되었는지 알려줄 수 있습니다.

<CodeGroup>
  ```python Python theme={null}
  from typing import Any
  from claude_agent_sdk import tool, create_sdk_mcp_server


  # z.enum() in TypeScript becomes an "enum" constraint in JSON Schema.
  # The dict schema has no equivalent, so full JSON Schema is required.
  @tool(
      "convert_units",
      "Convert a value from one unit to another",
      {
          "type": "object",
          "properties": {
              "unit_type": {
                  "type": "string",
                  "enum": ["length", "temperature", "weight"],
                  "description": "Category of unit",
              },
              "from_unit": {
                  "type": "string",
                  "description": "Unit to convert from, e.g. kilometers, fahrenheit, pounds",
              },
              "to_unit": {"type": "string", "description": "Unit to convert to"},
              "value": {"type": "number", "description": "Value to convert"},
          },
          "required": ["unit_type", "from_unit", "to_unit", "value"],
      },
  )
  async def convert_units(args: dict[str, Any]) -> dict[str, Any]:
      conversions = {
          "length": {
              "kilometers_to_miles": lambda v: v * 0.621371,
              "miles_to_kilometers": lambda v: v * 1.60934,
              "meters_to_feet": lambda v: v * 3.28084,
              "feet_to_meters": lambda v: v * 0.3048,
          },
          "temperature": {
              "celsius_to_fahrenheit": lambda v: (v * 9) / 5 + 32,
              "fahrenheit_to_celsius": lambda v: (v - 32) * 5 / 9,
              "celsius_to_kelvin": lambda v: v + 273.15,
              "kelvin_to_celsius": lambda v: v - 273.15,
          },
          "weight": {
              "kilograms_to_pounds": lambda v: v * 2.20462,
              "pounds_to_kilograms": lambda v: v * 0.453592,
              "grams_to_ounces": lambda v: v * 0.035274,
              "ounces_to_grams": lambda v: v * 28.3495,
          },
      }

      key = f"{args['from_unit']}_to_{args['to_unit']}"
      fn = conversions.get(args["unit_type"], {}).get(key)

      if not fn:
          return {
              "content": [
                  {
                      "type": "text",
                      "text": f"Unsupported conversion: {args['from_unit']} to {args['to_unit']}",
                  }
              ],
              "is_error": True,
          }

      result = fn(args["value"])
      return {
          "content": [
              {
                  "type": "text",
                  "text": f"{args['value']} {args['from_unit']} = {result:.4f} {args['to_unit']}",
              }
          ]
      }


  converter_server = create_sdk_mcp_server(
      name="converter",
      version="1.0.0",
      tools=[convert_units],
  )
  ```

  ```typescript TypeScript theme={null}
  import { tool, createSdkMcpServer } from "@anthropic-ai/claude-agent-sdk";
  import { z } from "zod";

  const convert = tool(
    "convert_units",
    "Convert a value from one unit to another",
    {
      unit_type: z.enum(["length", "temperature", "weight"]).describe("Category of unit"),
      from_unit: z
        .string()
        .describe("Unit to convert from, e.g. kilometers, fahrenheit, pounds"),
      to_unit: z.string().describe("Unit to convert to"),
      value: z.number().describe("Value to convert")
    },
    async (args) => {
      type Conversions = Record<string, Record<string, (v: number) => number>>;

      const conversions: Conversions = {
        length: {
          kilometers_to_miles: (v) => v * 0.621371,
          miles_to_kilometers: (v) => v * 1.60934,
          meters_to_feet: (v) => v * 3.28084,
          feet_to_meters: (v) => v * 0.3048
        },
        temperature: {
          celsius_to_fahrenheit: (v) => (v * 9) / 5 + 32,
          fahrenheit_to_celsius: (v) => ((v - 32) * 5) / 9,
          celsius_to_kelvin: (v) => v + 273.15,
          kelvin_to_celsius: (v) => v - 273.15
        },
        weight: {
          kilograms_to_pounds: (v) => v * 2.20462,
          pounds_to_kilograms: (v) => v * 0.453592,
          grams_to_ounces: (v) => v * 0.035274,
          ounces_to_grams: (v) => v * 28.3495
        }
      };

      const key = `${args.from_unit}_to_${args.to_unit}`;
      const fn = conversions[args.unit_type]?.[key];

      if (!fn) {
        return {
          content: [
            {
              type: "text",
              text: `Unsupported conversion: ${args.from_unit} to ${args.to_unit}`
            }
          ],
          isError: true
        };
      }

      const result = fn(args.value);
      return {
        content: [
          {
            type: "text",
            text: `${args.value} ${args.from_unit} = ${result.toFixed(4)} ${args.to_unit}`
          }
        ]
      };
    }
  );

  const converterServer = createSdkMcpServer({
    name: "converter",
    version: "1.0.0",
    tools: [convert]
  });
  ```
</CodeGroup>

서버가 정의되면, 날씨 예제와 동일한 방식으로 `query`에 전달합니다. 이 예제는 동일한 도구가 다양한 단위 유형을 처리하는 것을 보여주기 위해 루프에서 세 가지 다른 프롬프트를 보냅니다. 각 응답에 대해 `AssistantMessage` 객체(Claude가 해당 턴 동안 수행한 도구 호출을 포함)를 검사하고 최종 `ResultMessage` 텍스트를 출력하기 전에 각 `ToolUseBlock`을 출력합니다. 이를 통해 Claude가 도구를 사용하는 시점과 자체 지식에서 답변하는 시점을 볼 수 있습니다.

[도구 검색](/docs/ko/agent-sdk/tool-search)이 기본적으로 활성화되어 있으므로, 출력에는 Claude가 지연된 도구 스키마를 로드할 때 `ToolSearch` 호출도 포함될 수 있습니다.

<CodeGroup>
  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import (
      query,
      ClaudeAgentOptions,
      ResultMessage,
      AssistantMessage,
      ToolUseBlock,
  )


  async def main():
      options = ClaudeAgentOptions(
          mcp_servers={"converter": converter_server},
          allowed_tools=["mcp__converter__convert_units"],
      )

      prompts = [
          "Convert 100 kilometers to miles.",
          "What is 72°F in Celsius?",
          "How many pounds is 5 kilograms?",
      ]

      for prompt in prompts:
          try:
              async for message in query(prompt=prompt, options=options):
                  if isinstance(message, AssistantMessage):
                      for block in message.content:
                          if isinstance(block, ToolUseBlock):
                              print(f"[tool call] {block.name}({block.input})")
                  elif isinstance(message, ResultMessage) and message.subtype == "success":
                      print(f"Q: {prompt}\nA: {message.result}\n")
          except Exception as error:
              # A single-shot query() raises after yielding an error result. Only success
              # results are printed above, so handle the failure here and continue with
              # the next prompt.
              print(f"Call failed: {error}")


  asyncio.run(main())
  ```

  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  const prompts = [
    "Convert 100 kilometers to miles.",
    "What is 72°F in Celsius?",
    "How many pounds is 5 kilograms?"
  ];

  for (const prompt of prompts) {
    try {
      for await (const message of query({
        prompt,
        options: {
          mcpServers: { converter: converterServer },
          allowedTools: ["mcp__converter__convert_units"]
        }
      })) {
        if (message.type === "assistant") {
          for (const block of message.message.content) {
            if (block.type === "tool_use") {
              console.log(`[tool call] ${block.name}`, block.input);
            }
          }
        } else if (message.type === "result" && message.subtype === "success") {
          console.log(`Q: ${prompt}\nA: ${message.result}\n`);
        }
      }
    } catch (error) {
      // A single-shot query() throws after yielding an error result. Only success
      // results are logged above, so handle the failure here and continue with
      // the next prompt.
      console.error(`Call failed: ${error}`);
    }
  }
  ```
</CodeGroup>

<h2 id="next-steps">
  다음 단계
</h2>

이 페이지의 패턴들을 동일한 서버에서 혼합할 수 있습니다. 단일 서버는 데이터베이스 도구, API 게이트웨이 도구, 이미지 렌더러를 함께 보유할 수 있습니다.

여기서:

* 서버가 수십 개의 도구로 확장되는 경우, [도구 검색](/docs/ko/agent-sdk/tool-search)을 참조하여 Claude가 필요할 때까지 로드를 연기합니다.
* 자신의 서버를 구축하는 대신 외부 MCP 서버(파일시스템, GitHub, Slack)에 연결하려면, [MCP 서버 연결](/docs/ko/agent-sdk/mcp)을 참조합니다.
* 어떤 도구가 자동으로 실행되는지 또는 승인이 필요한지를 제어하려면, [권한 구성](/docs/ko/agent-sdk/permissions)을 참조합니다.
