> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 에이전트에서 구조화된 출력 얻기

> JSON Schema, Zod 또는 Pydantic을 사용하여 에이전트 워크플로우에서 검증된 JSON을 반환합니다. 다중 턴 도구 사용 후 타입 안전 구조화된 데이터를 얻습니다.

구조화된 출력을 사용하면 에이전트에서 반환받을 데이터의 정확한 형태를 정의할 수 있습니다. 에이전트는 작업을 완료하기 위해 필요한 모든 도구를 사용할 수 있으며, 마지막에는 여전히 스키마와 일치하는 검증된 JSON을 얻습니다. 필요한 구조에 대한 [JSON Schema](https://json-schema.org/understanding-json-schema/about)를 정의하면, SDK가 출력을 검증하고 불일치 시 다시 프롬프트합니다. 재시도 제한 내에서 검증이 성공하지 않으면 구조화된 데이터 대신 오류가 발생합니다. [오류 처리](#error-handling)를 참조하세요.

완전한 타입 안전성을 위해 [Zod](#type-safe-schemas-with-zod-and-pydantic)(TypeScript) 또는 [Pydantic](#type-safe-schemas-with-zod-and-pydantic)(Python)을 사용하여 스키마를 정의하고 강타입 객체를 반환받습니다.

<h2 id="why-structured-outputs">
  구조화된 출력이 필요한 이유?
</h2>

에이전트는 기본적으로 자유 형식 텍스트를 반환하므로 채팅에는 적합하지만 출력을 프로그래밍 방식으로 사용해야 할 때는 적합하지 않습니다. 구조화된 출력은 애플리케이션 로직, 데이터베이스 또는 UI 컴포넌트에 직접 전달할 수 있는 타입이 지정된 데이터를 제공합니다.

에이전트가 웹을 검색하고 레시피를 가져오는 레시피 앱을 생각해 봅시다. 구조화된 출력이 없으면 직접 파싱해야 하는 자유 형식 텍스트를 얻습니다. 구조화된 출력을 사용하면 원하는 형태를 정의하고 앱에서 직접 사용할 수 있는 타입이 지정된 데이터를 얻습니다.

<AccordionGroup>
  <Accordion title="구조화된 출력 없음">
    ```text theme={null}
    여기 클래식 초콜릿 칩 쿠키 레시피입니다!

    **초콜릿 칩 쿠키**
    준비 시간: 15분 | 조리 시간: 10분

    재료:
    - 2 1/4컵 다목적 밀가루
    - 1컵 버터, 부드러워진 상태
    ...
    ```

    이를 앱에서 사용하려면 제목을 파싱하고, "15분"을 숫자로 변환하고, 재료와 지시사항을 분리하고, 응답 전체에서 일관되지 않은 형식을 처리해야 합니다.
  </Accordion>

  <Accordion title="구조화된 출력 포함">
    ```json theme={null}
    {
      "name": "초콜릿 칩 쿠키",
      "prep_time_minutes": 15,
      "cook_time_minutes": 10,
      "ingredients": [
        { "item": "다목적 밀가루", "amount": 2.25, "unit": "컵" },
        { "item": "버터, 부드러워진 상태", "amount": 1, "unit": "컵" }
        // ...
      ],
      "steps": ["오븐을 375°F로 예열하기", "버터와 설탕 섞기" /* ... */]
    }
    ```

    UI에서 직접 사용할 수 있는 타입이 지정된 데이터입니다.
  </Accordion>
</AccordionGroup>

<h2 id="quick-start">
  빠른 시작
</h2>

구조화된 출력을 사용하려면 원하는 데이터의 형태를 설명하는 [JSON Schema](https://json-schema.org/understanding-json-schema/about)를 정의한 다음, `outputFormat` 옵션(TypeScript) 또는 `output_format` 옵션(Python)을 통해 `query()`에 전달합니다. 에이전트가 완료되면 결과 메시지에 스키마와 일치하는 검증된 데이터가 포함된 `structured_output` 필드가 포함됩니다.

아래 예제는 에이전트에 Anthropic을 조사하고 회사명, 설립 연도 및 본사를 구조화된 출력으로 반환하도록 요청합니다.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // 반환받을 데이터의 형태 정의
  const schema = {
    type: "object",
    properties: {
      company_name: { type: "string" },
      founded_year: { type: "number" },
      headquarters: { type: "string" }
    },
    required: ["company_name"]
  };

  for await (const message of query({
    prompt: "Anthropic을 조사하고 주요 회사 정보를 제공하세요",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: schema
      }
    }
  })) {
    // 결과 메시지에는 검증된 데이터가 포함된 structured_output이 포함됩니다
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      console.log(message.structured_output);
      // { company_name: "Anthropic", founded_year: 2021, headquarters: "San Francisco, CA" }
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

  # 반환받을 데이터의 형태 정의
  schema = {
      "type": "object",
      "properties": {
          "company_name": {"type": "string"},
          "founded_year": {"type": "number"},
          "headquarters": {"type": "string"},
      },
      "required": ["company_name"],
  }


  async def main():
      async for message in query(
          prompt="Anthropic을 조사하고 주요 회사 정보를 제공하세요",
          options=ClaudeAgentOptions(
              output_format={"type": "json_schema", "schema": schema}
          ),
      ):
          # 결과 메시지에는 검증된 데이터가 포함된 structured_output이 포함됩니다
          if isinstance(message, ResultMessage) and message.structured_output:
              print(message.structured_output)
              # {'company_name': 'Anthropic', 'founded_year': 2021, 'headquarters': 'San Francisco, CA'}


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="type-safe-schemas-with-zod-and-pydantic">
  Zod 및 Pydantic을 사용한 타입 안전 스키마
</h2>

JSON Schema를 직접 작성하는 대신 [Zod](https://zod.dev/)(TypeScript) 또는 [Pydantic](https://docs.pydantic.dev/latest/)(Python)을 사용하여 스키마를 정의할 수 있습니다. 이러한 라이브러리는 JSON Schema를 생성하고 응답을 완전히 타입이 지정된 객체로 파싱하여 자동 완성 및 타입 검사를 통해 코드베이스 전체에서 사용할 수 있습니다.

아래 예제는 요약, 단계 목록(각각 복잡도 수준 포함) 및 잠재적 위험이 있는 기능 구현 계획에 대한 스키마를 정의합니다. 에이전트는 기능을 계획하고 타입이 지정된 `FeaturePlan` 객체를 반환합니다. 그런 다음 `plan.summary`와 같은 속성에 액세스하고 완전한 타입 안전성으로 `plan.steps`를 반복할 수 있습니다.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { z } from "zod";
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // Zod를 사용하여 스키마 정의
  const FeaturePlan = z.object({
    feature_name: z.string(),
    summary: z.string(),
    steps: z.array(
      z.object({
        step_number: z.number(),
        description: z.string(),
        estimated_complexity: z.enum(["low", "medium", "high"])
      })
    ),
    risks: z.array(z.string())
  });

  type FeaturePlan = z.infer<typeof FeaturePlan>;

  // JSON Schema로 변환
  const schema = z.toJSONSchema(FeaturePlan);

  // 쿼리에서 사용
  for await (const message of query({
    prompt:
      "React 앱에 다크 모드 지원을 추가하는 방법을 계획하세요. 구현 단계로 나누세요.",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: schema
      }
    }
  })) {
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      // 검증하고 완전히 타입이 지정된 결과 얻기
      const parsed = FeaturePlan.safeParse(message.structured_output);
      if (parsed.success) {
        const plan: FeaturePlan = parsed.data;
        console.log(`기능: ${plan.feature_name}`);
        console.log(`요약: ${plan.summary}`);
        plan.steps.forEach((step) => {
          console.log(`${step.step_number}. [${step.estimated_complexity}] ${step.description}`);
        });
      }
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from pydantic import BaseModel
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage


  class Step(BaseModel):
      step_number: int
      description: str
      estimated_complexity: str  # 'low', 'medium', 'high'


  class FeaturePlan(BaseModel):
      feature_name: str
      summary: str
      steps: list[Step]
      risks: list[str]


  async def main():
      async for message in query(
          prompt="React 앱에 다크 모드 지원을 추가하는 방법을 계획하세요. 구현 단계로 나누세요.",
          options=ClaudeAgentOptions(
              output_format={
                  "type": "json_schema",
                  "schema": FeaturePlan.model_json_schema(),
              }
          ),
      ):
          if isinstance(message, ResultMessage) and message.structured_output:
              # 검증하고 완전히 타입이 지정된 결과 얻기
              plan = FeaturePlan.model_validate(message.structured_output)
              print(f"기능: {plan.feature_name}")
              print(f"요약: {plan.summary}")
              for step in plan.steps:
                  print(
                      f"{step.step_number}. [{step.estimated_complexity}] {step.description}"
                  )


  asyncio.run(main())
  ```
</CodeGroup>

**이점:**

* 완전한 타입 추론(TypeScript) 및 타입 힌트(Python)
* `safeParse()` 또는 `model_validate()`를 사용한 런타임 검증
* 더 나은 오류 메시지
* 구성 가능하고 재사용 가능한 스키마

<h2 id="output-format-configuration">
  출력 형식 구성
</h2>

`outputFormat`(TypeScript) 또는 `output_format`(Python) 옵션은 다음을 포함하는 객체를 허용합니다:

* `type`: 구조화된 출력의 경우 `"json_schema"`로 설정
* `schema`: 출력 구조를 정의하는 [JSON Schema](https://json-schema.org/understanding-json-schema/about) 객체입니다. Zod 스키마에서 `z.toJSONSchema()`를 사용하거나 Pydantic 모델에서 `.model_json_schema()`를 사용하여 생성할 수 있습니다.

SDK는 모든 기본 타입(object, array, string, number, boolean, null), `enum`, `const`, `required`, 중첩 객체 및 `$ref` 정의를 포함한 표준 JSON Schema 기능을 지원합니다. 지원되는 기능 및 제한사항의 전체 목록은 [JSON Schema 제한사항](https://platform.claude.com/docs/ko/build-with-claude/structured-outputs#json-schema-limitations)을 참조하세요.

유효한 JSON Schema가 아닌 스키마는 문제를 명시하는 오류와 함께 시작 시 실행에 실패합니다. v2.1.205 이전에는 유효하지 않은 스키마가 자동으로 무시되었고 에이전트가 구조화되지 않은 텍스트를 반환했습니다.

`"format": "email"`과 같은 `format` 키워드는 주석으로 허용되며 SDK의 유효성 검사기에 의해 적용되지 않습니다. v2.1.205 이전에는 `format`을 포함하는 모든 스키마가 유효하지 않은 것으로 처리되었습니다.

<h2 id="example-todo-tracking-agent">
  예제: TODO 추적 에이전트
</h2>

이 예제는 구조화된 출력이 다중 단계 도구 사용과 어떻게 작동하는지 보여줍니다. 에이전트는 코드베이스에서 TODO 주석을 찾은 다음 각각에 대한 git blame 정보를 조회해야 합니다. 실행 중에 사용할 도구(검색용 Grep, git 명령 실행용 Bash)를 자율적으로 결정하고 결과를 단일 구조화된 응답으로 결합합니다.

스키마에는 모든 파일에 대해 git blame 정보를 사용할 수 없을 수 있으므로 선택적 필드(`author` 및 `date`)가 포함됩니다. 에이전트는 찾을 수 있는 것을 채우고 나머지는 생략합니다.

<CodeGroup>
  ```typescript TypeScript theme={null}
  import { query } from "@anthropic-ai/claude-agent-sdk";

  // TODO 추출을 위한 구조 정의
  const todoSchema = {
    type: "object",
    properties: {
      todos: {
        type: "array",
        items: {
          type: "object",
          properties: {
            text: { type: "string" },
            file: { type: "string" },
            line: { type: "number" },
            author: { type: "string" },
            date: { type: "string" }
          },
          required: ["text", "file", "line"]
        }
      },
      total_count: { type: "number" }
    },
    required: ["todos", "total_count"]
  };

  // 에이전트는 Grep을 사용하여 TODO를 찾고, Bash를 사용하여 git blame 정보를 얻습니다
  for await (const message of query({
    prompt: "이 코드베이스에서 모든 TODO 주석을 찾고 누가 추가했는지 식별하세요",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: todoSchema
      }
    }
  })) {
    if (message.type === "result" && message.subtype === "success" && message.structured_output) {
      const data = message.structured_output as { total_count: number; todos: Array<{ file: string; line: number; text: string; author?: string; date?: string }> };
      console.log(`${data.total_count}개의 TODO를 찾았습니다`);
      data.todos.forEach((todo) => {
        console.log(`${todo.file}:${todo.line} - ${todo.text}`);
        if (todo.author) {
          console.log(`  ${todo.author}가 ${todo.date}에 추가함`);
        }
      });
    }
  }
  ```

  ```python Python theme={null}
  import asyncio
  from claude_agent_sdk import query, ClaudeAgentOptions, ResultMessage

  # TODO 추출을 위한 구조 정의
  todo_schema = {
      "type": "object",
      "properties": {
          "todos": {
              "type": "array",
              "items": {
                  "type": "object",
                  "properties": {
                      "text": {"type": "string"},
                      "file": {"type": "string"},
                      "line": {"type": "number"},
                      "author": {"type": "string"},
                      "date": {"type": "string"},
                  },
                  "required": ["text", "file", "line"],
              },
          },
          "total_count": {"type": "number"},
      },
      "required": ["todos", "total_count"],
  }


  async def main():
      # 에이전트는 Grep을 사용하여 TODO를 찾고, Bash를 사용하여 git blame 정보를 얻습니다
      async for message in query(
          prompt="이 코드베이스에서 모든 TODO 주석을 찾고 누가 추가했는지 식별하세요",
          options=ClaudeAgentOptions(
              output_format={"type": "json_schema", "schema": todo_schema}
          ),
      ):
          if isinstance(message, ResultMessage) and message.structured_output:
              data = message.structured_output
              print(f"{data['total_count']}개의 TODO를 찾았습니다")
              for todo in data["todos"]:
                  print(f"{todo['file']}:{todo['line']} - {todo['text']}")
                  if "author" in todo:
                      print(f"  {todo['author']}가 {todo['date']}에 추가함")


  asyncio.run(main())
  ```
</CodeGroup>

<h2 id="error-handling">
  오류 처리
</h2>

구조화된 출력 생성은 에이전트가 스키마와 일치하는 유효한 JSON을 생성할 수 없을 때 실패할 수 있습니다. 이는 일반적으로 스키마가 작업에 너무 복잡하거나, 작업 자체가 모호하거나, 에이전트가 검증 오류를 수정하려고 시도하는 동안 재시도 제한에 도달할 때 발생합니다. 또한 검증 실패 없이도 발생할 수 있습니다: [모델 폴백](/docs/ko/model-config#automatic-model-fallback)은 이미 완료된 출력을 스트림 중간에 취소할 수 있으며, 재시도가 이를 대체하지 않으면 실행이 동일한 오류로 종료됩니다. 디버깅하기 전에 결과 메시지의 `errors` 필드를 확인하여 두 가지 원인을 구분하십시오.

오류가 발생하면 결과 메시지에 무엇이 잘못되었는지 나타내는 `subtype`이 있습니다:

| Subtype                               | 의미                                                    |
| ------------------------------------- | ----------------------------------------------------- |
| `success`                             | 출력이 성공적으로 생성되고 검증됨                                    |
| `error_max_structured_output_retries` | 여러 시도 후 유효한 출력이 남지 않음(검증 실패 또는 성공적인 재시도가 없는 모델 폴백 취소) |

아래 예제는 `subtype` 필드를 확인하여 출력이 성공적으로 생성되었는지 또는 실패를 처리해야 하는지 결정합니다:

<CodeGroup>
  ```typescript TypeScript theme={null}
  for await (const msg of query({
    prompt: "Extract contact info from the document",
    options: {
      outputFormat: {
        type: "json_schema",
        schema: contactSchema
      }
    }
  })) {
    if (msg.type === "result") {
      if (msg.subtype === "success" && msg.structured_output) {
        // 검증된 출력 사용
        console.log(msg.structured_output);
      } else if (msg.subtype === "error_max_structured_output_retries") {
        // 실패 처리 - 더 간단한 프롬프트로 재시도, 구조화되지 않은 것으로 폴백 등
        console.error("유효한 출력을 생성할 수 없습니다");
      }
    }
  }
  ```

  ```python Python theme={null}
  async for message in query(
      prompt="Extract contact info from the document",
      options=ClaudeAgentOptions(
          output_format={"type": "json_schema", "schema": contact_schema}
      ),
  ):
      if isinstance(message, ResultMessage):
          if message.subtype == "success" and message.structured_output:
              # 검증된 출력 사용
              print(message.structured_output)
          elif message.subtype == "error_max_structured_output_retries":
              # 실패 처리
              print("유효한 출력을 생성할 수 없습니다")
  ```
</CodeGroup>

**오류 방지 팁:**

* **스키마를 집중적으로 유지하십시오.** 많은 필수 필드가 있는 깊게 중첩된 스키마는 만족하기 어렵습니다. 간단하게 시작하고 필요에 따라 복잡성을 추가하십시오.
* **스키마를 작업과 일치시키십시오.** 작업에 스키마가 요구하는 모든 정보가 없을 수 있으면 해당 필드를 선택적으로 만드십시오.
* **명확한 프롬프트를 사용하십시오.** 모호한 프롬프트는 에이전트가 생성할 출력을 알기 어렵게 합니다.

<h2 id="related-resources">
  관련 리소스
</h2>

* [JSON Schema 문서](https://json-schema.org/): 중첩 객체, 배열, 열거형 및 검증 제약 조건이 있는 복잡한 스키마를 정의하기 위한 JSON Schema 구문 학습
* [API 구조화된 출력](https://platform.claude.com/docs/ko/build-with-claude/structured-outputs): 도구 사용 없이 단일 턴 요청에 대해 Claude API와 함께 직접 구조화된 출력 사용
* [사용자 정의 도구](/docs/ko/agent-sdk/custom-tools): 구조화된 출력을 반환하기 전에 실행 중에 호출할 에이전트 사용자 정의 도구 제공
