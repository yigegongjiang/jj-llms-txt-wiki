> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code가 prompt caching을 사용하는 방법

> Claude Code는 prompt caching을 자동으로 관리합니다. 모델 전환이 느린 캐시되지 않은 턴을 트리거하는 이유, `/compact`의 비용, CLAUDE.md 편집이 세션 중에 적용되지 않는 이유, 캐시 히트율을 확인하는 방법을 알아봅니다.

Prompt caching은 Claude Code를 더 빠르고 비용 효율적으로 만듭니다. Caching이 없으면 API는 매 턴마다 전체 기록을 다시 처리합니다. Caching을 사용하면 이미 처리한 내용을 재사용하고, 재읽기를 [캐시된 토큰 요금](https://platform.claude.com/docs/en/about-claude/pricing)으로 청구하며, 변경된 내용만 완전히 처리합니다.

Claude Code는 [비활성화](#disable-prompt-caching)하지 않는 한 prompt caching을 자동으로 처리합니다. 일부 작업이 캐시를 무효화하고 다음 응답을 더 느리고 비싸게 만들기 때문에 prompt caching이 어떻게 작동하는지 아는 것이 여전히 유용합니다. 이 페이지에서는 어떤 작업이 그러한지, 일부 설정이 적용되기 위해 재시작을 기다리는 이유, 사용량이 높아 보일 때 캐시 성능을 확인하는 방법을 다룹니다.

<h2 id="how-the-cache-is-organized">
  캐시가 어떻게 구성되는지
</h2>

Claude Code에서 메시지를 보낼 때마다 새로운 API 요청을 만듭니다. 모델은 요청 간에 아무것도 기억하지 않으므로 Claude Code는 전체 컨텍스트를 다시 보냅니다: 시스템 프롬프트, 프로젝트 컨텍스트, 모든 이전 메시지와 도구 결과, 그리고 새로운 메시지입니다. 새로운 콘텐츠는 끝에 추가되므로 각 요청의 대부분은 이전 요청과 동일합니다. 프롬프트 캐싱은 API가 변경되지 않은 부분을 다시 처리하지 않도록 하는 방법입니다.

API는 각 요청의 시작 부분(프리픽스라고 함)을 최근에 처리한 콘텐츠와 일치시켜 캐시합니다. 일반적인 턴에서 프리픽스는 전체 이전 요청이고 최신 교환만 새로운 것입니다. 일치는 정확하므로 프리픽스의 어느 곳이든 변경되면 그 이후의 모든 것이 다시 계산됩니다. 파일별 또는 세그먼트별 캐싱은 없습니다. API 참조에서 [프롬프트 캐싱이 어떻게 작동하는지](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#how-prompt-caching-works)를 참조하세요.

<img src="https://mintcdn.com/claude-code/VbDJw--l6T9a9Wvm/images/prompt-caching-prefix.svg?fit=max&auto=format&n=VbDJw--l6T9a9Wvm&q=85&s=f2e8f0b8298a50305fe428ca3f1d1594" className="dark:hidden" alt="네 개의 턴이 증가하는 수평 막대로 표시됩니다. 각 턴의 요청은 이전 턴의 모든 것과 끝에 추가된 최신 교환을 포함합니다. 턴 2와 3에서는 변경되지 않은 프리픽스가 캐시에서 읽혀지고 새로운 교환만 처리됩니다. 턴 4에서는 시스템 프롬프트가 변경되어 프리픽스가 더 이상 일치하지 않으므로 전체 요청이 다시 처리되고 기록됩니다." width="720" height="454" data-path="images/prompt-caching-prefix.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/prompt-caching-prefix-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=297dc1c639f0915cae858d0c4b6f3be5" className="hidden dark:block" alt="네 개의 턴이 증가하는 수평 막대로 표시됩니다. 각 턴의 요청은 이전 턴의 모든 것과 끝에 추가된 최신 교환을 포함합니다. 턴 2와 3에서는 변경되지 않은 프리픽스가 캐시에서 읽혀지고 새로운 교환만 처리됩니다. 턴 4에서는 시스템 프롬프트가 변경되어 프리픽스가 더 이상 일치하지 않으므로 전체 요청이 다시 처리되고 기록됩니다." width="720" height="454" data-path="images/prompt-caching-prefix-dark.svg" />

프리픽스 일치를 최대한 활용하기 위해 Claude Code는 각 요청을 정렬하여 턴 간에 거의 변경되지 않는 콘텐츠가 먼저 오도록 합니다:

| 계층        | 콘텐츠                          | 변경 시기                               |
| --------- | ---------------------------- | ----------------------------------- |
| 시스템 프롬프트  | 핵심 지침, 도구 정의                 | 로드된 도구 정의 집합이 변경될 때                 |
| 프로젝트 컨텍스트 | CLAUDE.md, 자동 메모리, 범위 미지정 규칙 | 세션 시작 후 또는 `/clear` 또는 `/compact` 후 |
| 대화        | 사용자 메시지, Claude의 응답, 도구 결과   | 매 턴마다                               |

대화 계층의 변경은 시스템 프롬프트와 프로젝트 컨텍스트를 캐시된 상태로 유지합니다. 시스템 프롬프트의 변경은 모든 것을 무효화합니다. 왜냐하면 이제 모든 이후 콘텐츠가 다른 프리픽스 뒤에 있기 때문입니다. 세 번째 열은 완전한 목록이 아닌 일반적인 트리거를 제공하며, 아래 섹션에서 전체 집합을 다룹니다.

프리픽스 일치 규칙은 이 페이지의 대부분의 동작을 설명합니다. 예를 들어 [Plan Mode](/docs/ko/permission-modes#analyze-before-you-edit-with-plan-mode)와 [skill loading](/docs/ko/skills)은 지침을 대화 메시지로 추가하므로 캐시된 프리픽스는 그대로 유지됩니다.

두 가지 설정은 계층 표에 나타나지 않지만 여전히 캐시된 상태에 영향을 미칩니다:

* **모델**: 각 모델은 자체 캐시를 가집니다. 모델을 전환하면 콘텐츠가 동일한 경우에도 전체 요청이 다시 계산됩니다. 아래의 [모델 전환](#switching-models)을 참조하세요.
* **노력 수준**: 대부분의 모델에서 각 노력 수준은 자체 캐시를 가지므로 세션 중에 노력을 변경하면 전체 요청이 다시 계산됩니다. API 키 또는 Claude 구독이 있는 Opus 5.5 및 Fable 5.1에서는 기본적으로 캐시가 그대로 유지됩니다. 아래의 [노력 수준 변경](#changing-effort-level)을 참조하세요.

<Tip>
  세션 시작 시 모델과 노력 수준을 선택한 다음 작업 간의 자연스러운 중단점을 위해 `/compact`를 저장하세요. 작업 중에 변경을 적게 할수록 캐시 히트율이 높아집니다.
</Tip>

<h3 id="where-the-cache-lives">
  캐시가 어디에 있는지
</h3>

캐싱은 서버 측에서 발생하며, 모델을 제공하는 인프라에서 발생합니다. 그 위치는 인증 방식에 따라 다릅니다:

* **API 키, Claude 구독 또는 [Claude Platform on AWS](/docs/ko/claude-platform-on-aws)**: 캐시는 Anthropic의 인프라에 있으며 [Claude API](https://platform.claude.com/docs)를 통해 액세스됩니다.
* **Amazon Bedrock 또는 Google Cloud의 Agent Platform**: 캐시는 클라우드 제공자의 제공 인프라에 있습니다.
* **Microsoft Foundry**: 배포의 [호스팅 옵션](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options)에 따라 다릅니다. Azure에서 호스팅되는 배포는 Azure 인프라에서 제공되고, Anthropic에서 호스팅되는 배포는 Anthropic의 인프라에서 제공됩니다.
* **사용자 정의 `ANTHROPIC_BASE_URL` 또는 [LLM gateway](/docs/ko/llm-gateway)**: 캐시는 요청이 전달되는 위치에 있으며, 캐싱이 작동하는지 여부는 게이트웨이에 따라 다릅니다.

Claude Code는 또한 파일 변경 알림과 같은 시스템 컨텍스트를 대화 중에 추가하고, [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/ko/llm-gateway-protocol#disable-pre-release-capabilities)를 설정하지 않는 한 모든 제공자 및 연결에서 캐싱을 위해 해당 블록을 표시합니다. 이 경우 해당 블록은 캐시되지 않은 상태로 전송됩니다.

제공자의 자체 엔드포인트에서 Amazon Bedrock 및 해당 [Mantle endpoint](/docs/ko/amazon-bedrock#use-the-mantle-endpoint), Google Cloud의 Agent Platform 및 Microsoft Foundry는 Claude API와 동일한 방식으로 블록을 캐시합니다.

요청이 [LLM gateway](/docs/ko/llm-gateway), 사용자 정의 `ANTHROPIC_BASE_URL` 또는 [`ANTHROPIC_BEDROCK_BASE_URL`](/docs/ko/env-vars)과 같은 클라우드 제공자 기본 URL 재정의를 통과할 때, 캐시된 상태는 게이트웨이가 Claude Code가 보내는 [`cache_control` 마커](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#explicit-cache-breakpoints)를 어떻게 처리하는지에 따라 다릅니다:

* **변경되지 않은 상태로 전달**: 블록과 대화가 제공자의 자체 엔드포인트와 동일한 방식으로 캐시됩니다.
* **`cache_control`을 명명하는 `400` 오류로 표시된 요청 거부**: Claude Code는 마커를 블록에서 마지막 대화 메시지로 이동하여 요청을 다시 보내고 대화의 나머지 부분에서 그대로 유지합니다. 블록은 캐시되지 않은 입력으로 청구됩니다. 대화는 캐시된 상태로 유지됩니다.
* **성공을 반환하면서 마커 제거**: 전체 대화 기록이 매 턴마다 캐시되지 않은 입력으로 청구됩니다. 블록 형식 시스템 콘텐츠를 일반 문자열로 변환하는 게이트웨이는 동일한 방식으로 마커를 삭제합니다.

각 제공자가 저장하고 처리하는 것에 대해서는 [data usage](/docs/ko/data-usage)를 참조하세요. 캐시가 어디에 있든 항목은 비활성 기간 후에 만료되며, 아래의 [Cache lifetime](#cache-lifetime)에서 TTL과 연장 방법을 다룹니다.

<h2 id="actions-that-invalidate-the-cache">
  캐시를 무효화하는 작업
</h2>

이러한 작업들은 다음 요청이 캐시의 일부 또는 전체를 놓치게 합니다. 한 번 느리고 더 비싼 턴을 보게 되며, 그 후 새로운 접두사가 캐시됩니다. 대부분은 작업 중에 비용이 있다는 것을 알면 피할 수 있습니다. 모델 전환은 뒤따르는 느린 턴을 알아차릴 때까지 자유로워 보일 수 있습니다.

* [모델 전환](#switching-models)
* [노력 수준 변경](#changing-effort-level)
* [빠른 모드 켜기](#turning-on-fast-mode)
* [MCP 서버 연결 또는 연결 해제](#connecting-or-disconnecting-an-mcp-server)
* [플러그인 활성화 또는 비활성화](#enabling-or-disabling-a-plugin)
* [전체 도구 거부](#denying-an-entire-tool)
* [대화 압축](#compacting-the-conversation)
* [많은 이미지 누적](#accumulating-many-images)
* [Claude Code 업그레이드](#upgrading-claude-code)

<h3 id="switching-models">
  모델 전환
</h3>

각 모델에는 자체 캐시가 있습니다. [`/model`](/docs/ko/model-config#setting-your-model)로 전환하면 내용이 동일하더라도 다음 요청이 캐시 히트 없이 전체 대화 기록을 읽습니다.

터미널에서 `/model`을 실행하면 캐시가 여전히 따뜻한 동안에만 Claude Code가 전환을 확인하도록 요청합니다. 캐시는 Claude Code가 이 대화에서 마지막으로 요청을 보낸 후 또는 Claude가 마지막으로 응답한 후 한 [캐시 TTL](#cache-lifetime) 동안 따뜻하게 유지됩니다. 그 시간이 지나면 캐시가 만료되므로 Claude Code는 묻지 않고 전환합니다.

v2.1.238 이전에는 Claude Code가 캐시 TTL을 확인하지 않았고 캐시가 만료된 후에도 물었습니다.

[PreModelSwitch hook](/docs/ko/hooks#premodelswitch-decision-control)으로 이 확인을 요구하거나 건너뛸 수도 있습니다.

[`opusplan` 모델 설정](/docs/ko/model-config#opusplan-model-setting)은 계획 모드 중에는 Opus로, 실행 중에는 Sonnet으로 확인되므로 각 계획 모드 토글은 모델 전환이며 새로운 캐시를 시작합니다.

Fable 모델, Opus 5.5, Opus 5의 [자동 모델 폴백](/docs/ko/model-config#automatic-model-fallback)도 모델 전환입니다. 안전 분류기가 폴백 모델이 있는 카테고리의 요청에 플래그를 지정하면 Claude Code는 해당 모델에서 요청을 다시 실행하고 세션이 계속됩니다.

스킬 또는 명령의 frontmatter가 세션의 현재 모델이 아닌 [`model`](/docs/ko/skills#frontmatter-reference)을 지정하면 해당 턴도 모델 전환입니다. 다음 요청이 캐시 히트 없이 전체 대화 기록을 읽습니다. 세션 모델은 다음 프롬프트에서 재개됩니다. `context: fork` 스킬은 [포크된 서브에이전트의 모델](/docs/ko/skills#run-skills-in-a-subagent)을 대신 설정합니다.

<h3 id="changing-effort-level">
  노력 수준 변경
</h3>

대부분의 모델에서 세션 중에 [노력 수준](/docs/ko/model-config#adjust-effort-level)을 변경하면 다음 요청이 캐시 히트 없이 전체 대화 기록을 읽습니다. 캐시가 여전히 따뜻한 동안 Claude Code는 먼저 변경을 확인하도록 요청합니다.

API 키 또는 Claude 구독이 있는 Opus 5.5 및 Fable 5.1에서 노력을 변경하면 캐시가 유지되고 Claude Code는 묻지 않고 새 수준을 적용합니다. 이는 Amazon Bedrock, Google Cloud의 Agent Platform, [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway)에는 적용되지 않으며, [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS`](/docs/ko/llm-gateway-protocol#disable-pre-release-capabilities)를 설정하거나 조직에 HIPAA 구성이 있을 때도 적용되지 않습니다.

v2.1.260 이전에는 API 키 또는 Claude 구독이 있는 Fable 5.1에서 노력을 변경해도 캐시가 무효화되었습니다.

<h3 id="turning-on-fast-mode">
  빠른 모드 켜기
</h3>

[빠른 모드](/docs/ko/fast-mode)를 활성화하면 캐시 키의 일부인 요청 헤더가 추가되므로 Claude Code가 빠른 모드를 켜고 보내는 첫 요청이 캐시 히트 없이 전체 대화 기록을 읽습니다. Claude Code는 턴이 시작될 때 해당 헤더를 한 번 설정하고 전체 턴 동안 유지하므로 Claude가 작업 중일 때 빠른 모드를 켜면 헤더의 캐시 미스가 다음 턴의 첫 요청에서 발생합니다. 캐시되지 않은 입력 토큰은 [빠른 모드 요금](/docs/ko/fast-mode#understand-the-cost-tradeoff)으로 청구되므로 세션 시작 시 켜는 것이 긴 세션 깊숙이 켜는 것보다 비용이 적습니다. 현재 모델이 빠른 모드를 지원하지 않으면 빠른 모드를 활성화하면 [모델도 전환](#switching-models)되며, 그 전환은 실행 중인 턴의 다음 요청부터 자체적으로 새로운 캐시를 시작합니다.

비용은 대화당 한 번 적용됩니다. 첫 번째 빠른 모드 턴 후 Claude Code는 계속 헤더를 보내고 캐시 키의 일부가 아닌 요청의 속도 설정만 변합니다. 빠른 모드를 끄기, 속도 제한 후 [표준 속도로의 자동 폴백](/docs/ko/fast-mode#handle-rate-limits), 나중에 다시 켜기는 모두 캐시를 유지합니다. [사용 크레딧이 부족](/docs/ko/fast-mode#handle-rate-limits)하면 Claude Code는 거부된 각 빠른 모드 요청을 같은 방식으로 표준 속도로 재시도하므로 이 폴백도 캐시를 유지합니다. `/clear`와 `/compact`는 이를 재설정합니다. 어쨌든 그 지점에서 캐시를 다시 빌드하기 때문입니다.

<h3 id="connecting-or-disconnecting-an-mcp-server">
  MCP 서버 연결 또는 연결 해제
</h3>

도구 정의는 시스템 프롬프트 레이어에 있으므로 턴 간에 요청의 도구 정의 집합이 변경되면 캐시가 무효화됩니다. [advisor 도구](/docs/ko/advisor)를 토글하는 것은 예외입니다. 그 정의는 캐시 breakpoint 후에 있으므로 `/advisor`를 활성화하거나 비활성화하면 캐시된 접두사가 그대로 유지됩니다. [MCP 서버](/docs/ko/mcp) 변경이 이를 수행하는지 여부는 해당 도구가 [도구 검색](/docs/ko/mcp#scale-with-mcp-tool-search)으로 연기되는지 또는 접두사에 로드되는지에 따라 달라집니다.

* **연기된 도구**, 지원되는 모델의 기본값: 서버 연결, 연결 해제 또는 도구 목록 변경은 새 콘텐츠만 추가하고 이미 캐시된 것을 방해하지 않습니다.
* **접두사에 로드된 도구**: 이들에 대한 모든 변경이 캐시를 무효화합니다. 이는 [도구 검색을 사용할 수 없거나 비활성화](/docs/ko/mcp#configure-tool-search)된 경우에 발생합니다. 예를 들어 Claude 4.5 세대보다 이전의 Google Cloud의 Agent Platform 모델, 사용자 정의 `ANTHROPIC_BASE_URL` 게이트웨이, 또는 Claude Code가 배포가 도구 검색을 거부한다는 것을 감지하면 Microsoft Foundry [Azure에서 호스팅되는 배포](https://platform.claude.com/docs/en/build-with-claude/claude-in-microsoft-foundry#hosting-options)에서 발생합니다. 또한 [`alwaysLoad`](/docs/ko/mcp#exempt-a-server-from-deferral)로 표시된 서버 또는 도구에 대해, 그리고 [임계값 기반 로딩](/docs/ko/mcp#configure-tool-search)으로 유지되는 정의에 대해 발생합니다.

도구가 접두사에 로드될 때 무효화의 가장 일반적인 원인은 세션 중에 서버가 연결되거나 연결 해제되는 것입니다. 이는 사용자의 조치 없이 발생할 수 있습니다. stdio 서버의 프로세스가 종료되거나, HTTP 세션이 만료되거나, 서버가 [일시적 오류 후 자동으로 다시 연결](/docs/ko/mcp#automatic-reconnection)됩니다. 연결된 서버는 또한 도구 목록을 변경하는 [동적 도구 업데이트](/docs/ko/mcp#dynamic-tool-updates)를 푸시할 수 있습니다.

MCP 구성을 편집해도 캐시가 변경되지 않습니다. 새 구성은 재시작 후에만 적용되며, 이때 서버가 연결되거나 연결 해제됩니다.

<h3 id="enabling-or-disabling-a-plugin">
  플러그인 활성화 또는 비활성화
</h3>

[플러그인](/docs/ko/plugins/overview)을 활성화하거나 비활성화할 때 변경 비용은 플러그인이 제공하는 구성 요소 유형에 따라 달라집니다. 아래 경우는 각 구성 요소 유형, Claude Code가 변경을 적용하는 시기, 같은 세션에서 플러그인을 다시 비활성화할 때 발생하는 상황을 다룹니다.

<h4 id="plugin-components-that-keep-the-cache">
  캐시를 유지하는 플러그인 구성 요소
</h4>

Claude Code는 플러그인의 스킬, 명령, 에이전트, hook, 모니터 또는 테마에 대해 캐시를 무효화하지 않습니다. 기존 대화 후에 콘텐츠를 추가하므로 다음 요청이 해당 콘텐츠에 대해 비용을 지불하고 여전히 캐시에서 그 전의 모든 것을 읽습니다.

<h4 id="plugins-that-provide-mcp-servers">
  MCP 서버를 제공하는 플러그인
</h4>

[MCP 서버](/docs/ko/plugins/components#mcp-servers)를 제공하는 플러그인을 활성화하거나 비활성화할 때 Claude Code는 [MCP 서버를 연결하거나 연결 해제](#connecting-or-disconnecting-an-mcp-server)할 때와 동일한 규칙을 따릅니다.

* Claude Code가 서버의 도구를 연기하면 캐시를 유지합니다.
* Claude Code가 도구를 접두사에 로드하면 다음 요청이 전체 대화를 다시 읽습니다.

<h4 id="code-intelligence-plugins">
  코드 인텔리전스 플러그인
</h4>

[코드 인텔리전스 플러그인](/docs/ko/plugins/code-intelligence)을 활성화하면 Claude는 [LSP 도구](/docs/ko/tools-reference#lsp-tool-behavior)를 얻습니다.

<h4 id="when-plugin-changes-apply">
  플러그인 변경이 적용되는 시기
</h4>

`/plugin` 메뉴에서 수행한 변경은 [`/reload-plugins`](/docs/ko/plugins/cli-reference#reload-plugins)를 통해 진행되며, Claude Code는 메뉴를 닫을 때 이를 실행합니다. 추가된 공지 또는 전체 다시 읽기 여부에 관계없이 비용을 지불하며, 변경이 적용된 후 첫 번째 턴에서 비용을 지불합니다. Claude Code는 또한 자체적으로 변경을 적용할 수 있습니다.

* `command` 소스가 있는 플러그인의 경우 Claude Code는 [플러그인 자체를 다시 로드](/docs/ko/plugins/loading#when-a-command-source-re-runs)할 수 있습니다.
* [`/plugin` 인터페이스에서 플러그인을 설치](/docs/ko/plugins/install#install-a-plugin)할 때 Claude Code는 설치 중에 활성화할 수 있습니다. 설치 요약은 활성화했는지 여부를 알려줍니다.
* v2.1.246 이상에서 [`/cd`로 세션을 이동](/docs/ko/permissions#move-the-session-to-another-directory)할 때 Claude Code는 새 디렉토리의 설정이 활성화하는 플러그인을 이동의 일부로 적용하며, `/reload-plugins`를 보류시키는 전체 다시 읽기 경고는 표시하지 않습니다.
* 대화형 세션에서 `--plugin-dir`으로 전달한 [플러그인 폴더](/docs/ko/plugins/create#load-a-directory-or-archive-for-one-session)에서 플러그인을 추가하거나 제거할 때 변경이 즉시 적용됩니다. 적용할 경우 전체 다시 읽기가 트리거된다면 Claude Code는 대신 변경을 보류하고 `/reload-plugins`를 실행하라는 공지를 표시합니다. Claude Code v2.1.265 이상이 필요합니다.

`/reload-plugins`가 실행되고 다시 로드가 전체 다시 읽기를 트리거하면 Claude Code는 경고를 표시하고 다시 로드를 적용하지 않습니다. `/reload-plugins --force`를 실행하여 어쨌든 적용합니다.

`/reload-plugins`는 또한 데스크톱 앱, Agent SDK, [비대화형 모드](/docs/ko/headless)(`-p` 포함)와 같이 대화형 터미널이 없는 세션에서 실행되며, 세션에 직접 입력할 때 실행됩니다. Claude Code v2.1.260 이상이 필요합니다.

이러한 세션에서 다시 로드는 플러그인 MCP 서버 변경을 제외한 모든 것을 적용하며, [다음 세션에서 적용](/docs/ko/plugins/cli-reference#reload-plugins)되므로 세션 중에 전체 다시 읽기 비용이 발생하지 않습니다.

<h4 id="plugins-you-enable-and-then-disable-in-one-session">
  한 세션에서 활성화한 후 비활성화하는 플러그인
</h4>

세션 초반에 활성화한 플러그인을 비활성화할 때 Claude Code는 이전 요청 형태를 복원합니다. 해당 접두사가 여전히 [캐시 수명](#cache-lifetime) 내에 있으면 다음 요청이 다시 빌드하는 대신 이전 캐시 항목을 읽습니다.

<h3 id="denying-an-entire-tool">
  전체 도구 거부
</h3>

`Bash` 또는 `WebFetch`와 같은 단순 도구 이름을 [거부 규칙](/docs/ko/permissions#manage-permissions)으로 추가하면 `/permissions`를 통해 규칙을 추가하거나 [설정 파일을 직접 편집](/docs/ko/settings#when-edits-take-effect)하여 다음 요청부터 Claude가 해당 도구를 호출할 수 없습니다. 여기에는 턴 중간에 `/permissions`를 통해 추가하는 규칙이 포함됩니다.

[도구 검색](/docs/ko/mcp#scale-with-mcp-tool-search)이 활성화되어 있을 때(지원되는 모델의 기본값), 요청의 도구 정의가 변경되지 않고 캐시된 접두사가 유지됩니다. 도구 검색을 사용할 수 없거나 비활성화되어 있을 때 Claude Code는 다음 요청에서 정의를 제거하므로 캐시가 무효화되며, 나중에 규칙을 제거해도 마찬가지입니다.

도구 이름 위치에서 일치하는 거부 규칙만 이 방식으로 도구를 차단합니다. 단순 도구 이름, 동등한 `Bash(*)` 형식, 또는 `"*"`와 같은 [도구 이름 glob](/docs/ko/permissions#tool-name-wildcards)입니다. `"mcp__*"`와 같이 MCP 도구만 일치하는 glob은 해당 도구를 같은 방식으로 차단합니다. `Bash(rm *)`과 같은 범위 지정 거부 규칙, 그리고 모든 허용 및 요청 규칙은 Claude가 보는 도구를 변경하지 않습니다. Claude Code는 Claude가 호출을 시도할 때 확인하여 접두사를 그대로 유지합니다.

<h3 id="compacting-the-conversation">
  대화 압축
</h3>

[압축](/docs/ko/context-window#what-survives-compaction)은 메시지 기록을 요약으로 바꿉니다. 설계상 이는 대화 레이어를 무효화합니다. 다음 요청이 이전 것과 접두사를 공유하지 않는 새로운 더 짧은 기록을 가지기 때문입니다. Claude Code는 대화가 [변경될 시스템 프롬프트를 유지하면서 재개](#resuming-a-session)되지 않는 한 시스템 프롬프트 레이어를 재사용합니다. 그 경우 첫 번째 압축이 현재 프롬프트로 전환되고 해당 레이어가 한 번 다시 빌드됩니다. 디스크에서 프로젝트 컨텍스트를 다시 로드하며, 세션 시작 이후 CLAUDE.md 및 메모리가 변경되지 않은 경우에만 캐시 히트합니다.

요약을 생성하기 위해 Claude Code는 대화와 동일한 시스템 프롬프트, 도구, 기록을 가진 별도의 요청을 보내고 최종 사용자 메시지로 요약 지침을 추가합니다. 캐시가 따뜻한 동안 해당 요청이 캐시에서 접두사를 읽으므로 세션 중 `/compact`는 컨텍스트 크기가 제안하는 것의 일부 비용이 들고 대부분의 시간을 요약 생성에 소비합니다.

[캐시 수명](#cache-lifetime)보다 긴 휴식 후에는 읽을 캐시가 남아 있지 않으므로 요약 요청이 캐시되지 않은 입력으로 전체 기록을 다시 처리합니다. 이것이 [이전 세션을 재개](/docs/ko/sessions#resume-from-a-summary)할 때 `/compact`의 비용이 가장 많은 이유입니다. 따뜻한 경우와 차가운 경우 모두에서 압축 후 턴이 훨씬 더 짧은 요약에 대해서만 대화 캐시를 다시 빌드하므로 해당 턴이 느린 부분이 아닙니다.

<Tip>
  압축은 더 이상 필요하지 않은 컨텍스트를 버릴 때 유리합니다. 오버헤드가 발생하는 시기를 선택하려면 작업 간 또는 작업 간과 같은 자연스러운 휴식 시간에 `/compact`를 실행하고 자동 압축이 작업 중에 트리거될 때까지 기다리지 마세요. 완전히 포기하고 싶은 경로를 따라가면 대신 [이전 턴으로 되감기](#rewinding-the-conversation)하세요. 되감기는 압축이 수행하는 새로운 것을 빌드하는 대신 이미 캐시된 접두사로 다시 자릅니다.
</Tip>

<h3 id="accumulating-many-images">
  많은 이미지 누적
</h3>

API는 각 요청이 수행할 수 있는 이미지 및 PDF의 수를 제한합니다. 현재 숫자는 API 문서의 [요청 제한](https://platform.claude.com/docs/en/build-with-claude/vision#request-limits)을 참조하세요. Claude Code는 또한 요청의 이미지 및 PDF의 총 크기를 제한하므로 큰 스크린샷은 작은 것보다 더 적은 이미지로 제한에 도달합니다.

다음 요청이 제한을 초과하면 Claude Code는 보내는 것에서 가장 오래된 이미지 및 PDF의 배치를 제거하여 다시 제거해야 하기 전에 더 많은 공간을 확보합니다. Claude는 더 이상 제거된 이미지를 볼 수 없습니다. Claude가 다시 필요하면 다시 공유하세요.

이미지를 제거하면 이를 보유한 메시지가 변경되므로 다음 요청이 해당 메시지 중 가장 이른 것부터 대화를 다시 처리합니다. Claude Code가 한 번에 배치를 제거하기 때문에 각 새 스크린샷마다 하나씩이 아니라 배치당 한 번의 느린 턴을 봅니다.

<h3 id="upgrading-claude-code">
  Claude Code 업그레이드
</h3>

새로운 Claude Code 버전은 일반적으로 시스템 프롬프트 또는 도구 정의를 업데이트하므로 업그레이드 후 시작하는 첫 번째 대화는 맨 위에서 캐시를 빌드합니다. [자동 업데이트](/docs/ko/setup#auto-updates)는 백그라운드에서 새 버전을 다운로드하지만 다음 시작 시에만 적용하며 세션 중에는 절대 적용하지 않으므로 세션 중 놀라움이 아니라 재시작 후 캐시되지 않은 첫 턴으로 표시됩니다. `DISABLE_AUTOUPDATER=1`을 설정하여 업그레이드가 적용되는 시기를 제어합니다.

<Note>
  업그레이드 전에 시작한 대화를 재개하는 비용은 [세션 재개](#resuming-a-session)를 참조하세요.
</Note>

<h2 id="actions-that-keep-the-cache">
  캐시를 유지하는 작업
</h2>

이러한 작업들은 대화의 끝에 추가되거나 요청을 전혀 건드리지 않습니다. CLAUDE.md 편집과 같은 일부 작업들은 변경 사항이 `/clear`, `/compact` 또는 재시작까지 실행 중인 세션에 도달하지 않는 이유와 동일한 이유로 캐시를 유지합니다.

* [저장소의 파일 편집](#editing-files-in-your-repository)
* [세션 중 CLAUDE.md 편집](#editing-claude-md-mid-session)
* [권한 모드 변경](#changing-permission-mode)
* [출력 스타일 변경](#changing-output-style)
* [스킬 및 명령 호출](#invoking-skills-and-commands)
* [`/recap` 실행](#running-%2Frecap)
* [대화 되감기](#rewinding-the-conversation)
* [서브에이전트 생성](#subagents-and-the-cache)

<h3 id="editing-files-in-your-repository">
  저장소의 파일 편집
</h3>

파일 내용은 Claude가 파일을 읽을 때만 컨텍스트에 들어가며, 읽기는 대화에 추가됩니다. Claude가 이전에 읽은 파일을 편집해도 기록의 이전 읽기를 소급하여 변경하지 않습니다. 대신 Claude Code는 파일이 변경되었음을 나타내는 `<system-reminder>`를 추가하고, 필요한 경우 Claude가 다시 읽습니다.

<h3 id="editing-claude-md-mid-session">
  세션 중 CLAUDE.md 편집
</h3>

프로젝트 루트 및 사용자 수준의 CLAUDE.md 파일은 세션 시작 시 한 번 읽혀지고 메모리에 보관됩니다. 세션 중 편집해도 캐시가 무효화되지 않지만, 편집 사항도 적용되지 않습니다. Claude는 세션 시작 시 로드된 버전으로 계속 작동합니다. 새로운 내용은 다음 `/clear`, `/compact` 또는 재시작 시 로드됩니다.

[하위 디렉토리의 중첩된 CLAUDE.md 파일](/docs/ko/memory)과 [`paths:` 프론트매터가 있는 규칙](/docs/ko/memory#path-specific-rules)은 Claude가 일치하는 파일을 처음 읽을 때 나중에 로드됩니다. 로드되기 전에 편집하면 적용됩니다. 로드된 후에는 내용이 대화 기록의 일부이므로 세션 중 편집은 소급하여 변경하지 않습니다.

<h3 id="changing-permission-mode">
  권한 모드 변경
</h3>

[권한 모드](/docs/ko/permission-modes)를 수동에서 편집 수락으로 전환하는 것과 같이 전환해도 시스템 프롬프트나 도구 정의가 변경되지 않으므로 모드 변경은 캐시 안전합니다. 예외는 [`opusplan`](/docs/ko/model-config#opusplan-model-setting) 모델 설정이 있는 계획 모드로, 계획 모드에 들어가거나 나갈 때 모델을 Opus와 Sonnet 사이에서 전환합니다. 이는 모드 토글을 [모델 전환](#switching-models)으로 만듭니다.

<h3 id="changing-output-style">
  출력 스타일 변경
</h3>

세션 중 [`/output-style`](/docs/ko/output-styles#change-your-output-style), `/config` 또는 `outputStyle` 설정으로 [출력 스타일](/docs/ko/output-styles)을 전환하면, Claude는 다음 메시지부터 새로운 스타일을 사용합니다. Claude Code는 새로운 스타일의 지침을 대화의 메시지로 전달하므로, 해당 요청은 여전히 시스템 프롬프트와 이전 대화를 캐시에서 읽습니다.

v2.1.251 이전에는 세션 중 스타일 전환이 캐시를 유지했지만 `/clear`를 실행하거나 새 세션을 시작할 때까지 적용되지 않았습니다.

<h3 id="invoking-skills-and-commands">
  스킬 및 명령 호출
</h3>

[스킬](/docs/ko/skills)과 [명령](/docs/ko/commands)은 호출 지점에서 사용자 메시지로 지침을 주입합니다. 대화의 이전 내용은 변경되지 않습니다. 프론트매터에서 `model`을 지정하는 스킬 또는 명령은 해당 턴에 대한 [모델 전환](#switching-models)이 될 수 있습니다.

<h3 id="running-/recap">
  `/recap` 실행
</h3>

[`/recap`](/docs/ko/interactive-mode#session-recap)은 터미널에 표시할 요약을 생성합니다. `/compact`와 달리 요약을 메시지 기록을 대체하는 대신 명령 출력으로 추가하므로, 캐시된 접두사는 그대로 유지됩니다.

<h3 id="rewinding-the-conversation">
  대화 되감기
</h3>

[`/rewind`](/docs/ko/checkpointing)는 대화를 이전 턴으로 자릅니다. 남은 기록은 그 시점에서 캐시가 구축된 것과 동일한 내용이며, 시스템 프롬프트와 프로젝트 컨텍스트 레이어는 변경되지 않으므로 다음 요청은 이전 캐시 항목에 도달합니다. 그 이후의 모든 턴은 해당 접두사를 통해 읽었으며, 원래 턴이 TTL보다 오래 전이었더라도 항목을 따뜻하게 유지했습니다.

대화와 함께 파일 체크포인트를 복원하는 것은 캐시에 별도의 영향을 미치지 않습니다. 파일 내용은 Claude가 파일을 읽을 때만 컨텍스트에 들어가며, [저장소의 파일 편집](#editing-files-in-your-repository)과 동일합니다.

<h2 id="resuming-a-session">
  세션 재개
</h2>

[세션을 재개](/docs/ko/sessions#resume-a-session)할 때, Claude Code는 전체 대화를 다시 전송하며, 요청은 캐시에서 변경되지 않았고 여전히 [캐시 수명](#cache-lifetime) 내에 있는 접두사 부분을 읽습니다. 이 페이지 상단의 레이어 테이블은 각 레이어가 어떻게 변경되는지 나타냅니다.

시스템 프롬프트는 [Claude Code 업그레이드](#upgrading-claude-code) 후 또는 재개 시 다른 [`--append-system-prompt`](/docs/ko/cli-reference#system-prompt-flags) 텍스트로 변경될 수 있습니다. 기본적으로 재개된 대화는 시작할 때의 시스템 프롬프트를 유지하므로 해당 히스토리는 여전히 동일한 프롬프트 뒤에 있으며, 변경 사항은 대화가 압축되거나 새 대화에서 적용됩니다. [재개된 대화의 시스템 프롬프트 플래그](/docs/ko/cli-reference#system-prompt-flags-in-resumed-conversations)는 Claude Code가 모든 요청에서 프롬프트를 다시 빌드하는 경우를 다룹니다.

<h2 id="cache-lifetime">
  캐시 수명
</h2>

캐시된 접두사는 비활성 기간 후에 만료됩니다. 캐시에 도달하는 각 요청은 타이머를 재설정하므로 계속 작업하는 한 캐시는 따뜻한 상태로 유지됩니다. 충분히 긴 간격 후에는 다음 요청이 전체 입력을 다시 계산하고 캐시를 다시 설정하므로, 한동안 떠난 후 돌아온 첫 번째 턴이 눈에 띄게 느릴 수 있습니다.

Pro 또는 Max 플랜에서 긴 휴식 후 대규모 세션을 재개할 때 Claude Code는 [요약에서 재개할 수 있도록 제안](/docs/ko/sessions#resume-from-a-summary)하므로 이후 요청이 전체 기록을 전달하지 않습니다.

TTL(Time to Live)은 캐시가 유지되는 간격의 길이를 제어합니다. API는 두 가지를 제공합니다: 5분 TTL과 [1시간 TTL](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#1-hour-cache-duration)로, 더 긴 휴식 시간 동안 캐시를 따뜻하게 유지하지만 [캐시 쓰기를 더 높은 요금으로 청구합니다](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing). 더 긴 TTL은 세션을 유휴 상태로 두었다가 돌아올 때 도움이 됩니다. 만료된 접두사가 비용이 드는 재처리를 건너뛸 수 있기 때문입니다. 5분을 초과하여 유휴 상태가 되지 않는 짧은 작업 버스트에서는 더 비용이 많이 듭니다. 여기서 더 높은 쓰기 요금이 적용되고 더 긴 캐시 수명이 사용되지 않습니다.

<h3 id="which-ttl-each-request-gets">
  각 요청이 받는 TTL
</h3>

Claude Code는 요청별로 TTL을 결정하며, 모든 요청은 두 가지 고정 버킷 중 하나에 해당합니다:

* **메인 대화**: 대화형 턴, 비대화형 `-p` 실행, Agent SDK 턴, 그리고 Claude Code가 이들과 함께 인라인으로 실행하는 도우미
* **기타 모든 것**: Claude Code가 해당 대화 외부에서 수행하는 요청(예: [서브에이전트](/docs/ko/sub-agents), [워크플로우](/docs/ko/workflows), 인프로세스 [팀원](/docs/ko/agent-teams), 포크, 압축, 세션 제목)

TTL을 직접 선택하지 않으면 Claude Code는 Claude 구독 내에서 플랜의 포함된 사용량 범위 내에서만 1시간 TTL을 요청합니다. 여기서는 메인 대화에 대해 1시간을 요청하고, Anthropic이 서버 측에서 제어하는 작은 도우미 요청 세트를 요청합니다. 이 표는 두 가지 청구 방식 모두에서 각 버킷의 기본 TTL을 제공합니다.

| 요청 버킷   | Claude 구독, 플랜 사용량 범위 내      | 사용 크레딧, API 키 또는 클라우드 제공자 |
| ------- | --------------------------- | ------------------------- |
| 메인 대화   | 1시간                         | 5분                        |
| 기타 모든 것 | 5분, 서버 제어 도우미 요청 제외(1시간 받음) | 5분                        |

플랜의 사용량 한도를 초과하고 Claude Code가 [사용 크레딧](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)을 사용하기 시작하면 해당 사용량에 대해 청구되므로 Claude Code는 메인 대화를 더 저렴한 5분 TTL로 낮춥니다. 메인 대화에서 1시간 TTL을 유지하려면 [TTL을 직접 선택하세요](#choose-the-ttl-yourself).

<h3 id="choose-the-ttl-yourself">
  TTL을 직접 선택하세요
</h3>

각 버킷에 대해 TTL을 설정할 수 있습니다. 각 제어는 `5m` 또는 `1h`를 사용하며, Claude Code는 다른 값을 무시합니다.

* **메인 대화**: [`promptCacheTtl`](/docs/ko/settings-reference#promptcachettl) 설정 또는 `CLAUDE_CODE_PROMPT_CACHE_TTL` [환경 변수](/docs/ko/env-vars)
* **기타 모든 것**: [`subagentPromptCacheTtl`](/docs/ko/settings-reference#subagentpromptcachettl) 설정 또는 `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL` 환경 변수

두 설정과 두 환경 변수 모두 Claude Code v2.1.242 이상이 필요합니다. API 키로 로그인하거나 클라우드 제공자를 사용하는 경우 `promptCacheTtl`을 `1h`로 설정하여 메인 대화에 1시간 캐시를 제공하세요. 그 외부의 요청은 해당 버킷에 대해 TTL을 선택할 때까지 5분 기본값을 유지합니다.

둘 이상의 제어가 적용되면 Claude Code는 다음 순서에서 첫 번째 일치를 사용합니다:

1. `FORCE_PROMPT_CACHING_5M=1`로, 두 버킷 모두에 5분을 강제합니다
2. 버킷의 환경 변수
3. 버킷의 설정
4. 서브에이전트의 요청의 경우, 서브에이전트의 [`experimental` 프론트매터 필드](/docs/ko/sub-agents#supported-frontmatter-fields)의 `cacheTtl` 값으로, Claude Code v2.1.248 이상이 필요합니다. Claude Code는 Claude 구독이 사용 크레딧을 사용 중일 때 거기서 `1h`를 무시합니다
5. `ENABLE_PROMPT_CACHING_1H=1`로, 두 버킷 모두에 1시간을 요청합니다
6. [요청의 버킷에 대한 기본값](#which-ttl-each-request-gets)

캐시 동작을 디버깅하거나, 두 TTL을 비교하거나, [관리 설정](/docs/ko/managed-settings)에서 설정한 더 긴 TTL을 재정의할 때 `FORCE_PROMPT_CACHING_5M=1`을 설정하세요.

메인 대화의 캐시 쓰기가 사용한 TTL을 확인하려면 `claude -p "hello" --output-format json`을 실행하고 결과에서 `usage.cache_creation`을 읽으세요. Claude Code는 1시간 캐시 쓰기를 `ephemeral_1h_input_tokens` 아래에 보고하고 5분 캐시 쓰기를 `ephemeral_5m_input_tokens` 아래에 보고합니다.

`ANTHROPIC_BASE_URL`로 설정한 LLM 게이트웨이를 통해 1시간 요청의 일부가 `anthropic-beta` 헤더에서 이동하므로 게이트웨이를 구성하여 [해당 헤더를 변경하지 않고 전달하세요](/docs/ko/llm-gateway-protocol#request-headers). 1시간 TTL은 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway#availability-and-limitations)를 통해 사용할 수 없습니다. Amazon Bedrock에서 프롬프트 캐싱 지원, 최소 캐시 가능 접두사 길이, 1시간 TTL 가용성은 모두 모델에 따라 다릅니다. 캐시 토큰 수가 0으로 유지되면 Amazon Bedrock 설명서에서 [지원되는 모델, 지역 및 제한](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models)을 확인하세요.

<h2 id="cache-scope">
  캐시 범위
</h2>

Claude Code에서 캐시는 효과적으로 하나의 머신과 디렉토리로 범위가 지정됩니다. 각 대화는 작업 디렉토리, 플랫폼, 셸 및 OS 버전을 포함하며, 시스템 프롬프트는 자동 메모리 경로의 이름을 지정하므로 서로 다른 디렉토리의 두 세션은 서로 다른 접두사를 구축하고 서로의 캐시를 놓칩니다. 여기에는 각 worktree가 자신의 작업 디렉토리를 가지고 있으므로 동일한 저장소의 worktrees도 포함됩니다.

동일한 디렉토리에서 병렬로 실행하는 세션은 일치하는 접두사를 구축하고 서로의 캐시를 읽습니다. 순차 세션은 시작 시 촬영한 git 상태 스냅샷이 일치할 때만 접두사를 공유합니다. 각 대화는 해당 스냅샷의 분기 및 최근 커밋도 포함하기 때문입니다.

기본 API 캐시는 더 광범위합니다. 캐시는 조직 간에 격리되며, 일부 제공자의 경우 [조직 내 워크스페이스 간에 격리됩니다](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#cache-storage-and-sharing). 이러한 경계 내에서 동일한 모델과 접두사를 가진 두 요청은 동일한 캐시를 읽습니다. 자동화된 프로세스의 플릿을 실행하는 Agent SDK 호출자의 경우, [사용자 및 머신 간 프롬프트 캐싱 개선](/docs/ko/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines)을 참조하여 시스템 프롬프트의 머신별 섹션을 억제하고 머신 간 캐시를 공유합니다.

<h2 id="check-cache-performance">
  캐시 성능 확인
</h2>

캐시 성능은 API가 모든 응답에서 보고하는 두 개의 토큰 수로 표시됩니다. 이를 실시간으로 확인하는 가장 직접적인 방법은 `current_usage` 객체를 읽는 [상태 표시줄 스크립트](/docs/ko/statusline)입니다:

| 필드                            | 의미                                                                                                                 |
| ----------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `cache_creation_input_tokens` | 이번 턴에 캐시에 기록된 토큰, 캐시 쓰기 요금으로 청구됨                                                                                   |
| `cache_read_input_tokens`     | 이번 턴에 캐시에서 제공된 토큰, 모델의 [캐시된 토큰 요금](https://platform.claude.com/docs/en/about-claude/pricing)으로 청구되며, 표준 입력 요금보다 낮음 |

높은 읽기 대 생성 비율은 캐싱이 잘 작동하고 있음을 의미합니다. 생성이 턴마다 높게 유지되면 접두사에서 뭔가 변경되고 있다는 뜻입니다. [캐시를 무효화하는 작업](#actions-that-invalidate-the-cache) 섹션에서 일반적인 원인을 나열합니다.

세션별 요약을 보려면 `/usage`를 실행하세요. 주 대화의 첫 응답 후, Claude Code는 [`Prompt cache (main)` 라인](/docs/ko/costs#prompt-cache-statistics)을 세션 블록에 추가하여 세션의 히트 비율, 미스 횟수 및 캐시가 현재 따뜻한 상태인지 여부를 표시합니다. 상태 표시줄 스크립트는 [`prompt_cache` 객체](/docs/ko/statusline#prompt-cache-fields)에서 동일한 숫자를 읽을 수 있습니다. 둘 다 Claude Code v2.1.251 이상이 필요합니다.

`Prompt cache (main)` 라인은 Claude Code가 식별할 수 있을 때 마지막 미스의 가능한 원인을 이름으로 표시합니다. 예를 들어 `likely cause: tool definitions changed`입니다. 가능한 원인 텍스트는 Claude Code v2.1.260 이상이 필요합니다.

조직 전체의 가시성을 위해 OpenTelemetry 내보내기는 사용자 및 세션별로 캐시 읽기 및 생성 토큰을 보고합니다. 메트릭 및 이벤트 속성 참조는 [사용량 모니터링](/docs/ko/monitoring-usage)을 참조하세요.

<h2 id="subagents-and-the-cache">
  서브에이전트와 캐시
</h2>

[서브에이전트](/docs/ko/sub-agents)는 부모와 별도의 자체 시스템 프롬프트와 도구 세트로 자체 대화를 시작합니다. 첫 번째 요청은 두 접두사가 다르기 때문에 부모의 캐시를 읽지 않으며, 자신의 턴에 걸쳐 자체 캐시를 워밍합니다. 서브에이전트는 메인 대화 [TTL 버킷](#which-ttl-each-request-gets) 외부에 있으므로, [더 긴 것을 선택](#choose-the-ttl-yourself)할 때까지 구독 상태에서도 5분을 얻습니다.

부모의 캐시는 영향을 받지 않습니다. 부모 측에서 서브에이전트의 호출과 결과는 대화에 추가되어 부모의 접두사를 그대로 유지합니다.

반면 [포크](/docs/ko/sub-agents#fork-the-current-conversation)는 부모의 시스템 프롬프트, 도구 및 대화 기록을 정확히 상속하므로 첫 번째 요청이 부모의 캐시를 읽습니다.

다른 요청도 이전 요청이 캐시한 접두사를 읽을 수 있습니다:

* **세션 복사본**: [`/fork`로 복사](/docs/ko/agent-view#copy-the-session-with-%2Ffork)한 세션은 복사된 대화의 끝에 메시지로 격리 지침을 받으므로, 원본 대화가 구축한 캐시가 그대로 유지됩니다.
* **압축**: [대화 압축](#compacting-the-conversation)에 설명된 요약 호출은 동일한 접두사 공유 방식을 사용합니다.
* **재개된 서브에이전트**: Claude가 [서브에이전트를 재개](/docs/ko/sub-agents#resume-subagents)할 때, 재개된 실행의 첫 번째 요청은 원본 실행이 워밍한 캐시를 읽을 수 있습니다.
* **워크플로우 팬아웃**: [워크플로우 팬아웃](/docs/ko/workflows#prompt-caching-in-a-fan-out)에서 동일 접두사 에이전트의 경우, Claude Code는 첫 번째를 제외한 모든 것을 기본적으로 최대 5초 동안 보유하므로, 첫 번째 요청이 첫 번째 에이전트가 캐시한 접두사를 읽을 수 있습니다.

<h2 id="disable-prompt-caching">
  prompt caching 비활성화
</h2>

특정 모델이나 공급자와의 caching 동작을 디버깅할 때 caching을 비활성화하는 것이 유용할 수 있습니다. 이를 끄려면 다음 환경 변수 중 하나를 `1`로 설정합니다:

| 변수                              | 효과             |
| ------------------------------- | -------------- |
| `DISABLE_PROMPT_CACHING`        | 모든 모델에 대해 비활성화 |
| `DISABLE_PROMPT_CACHING_HAIKU`  | Haiku만 비활성화    |
| `DISABLE_PROMPT_CACHING_SONNET` | Sonnet만 비활성화   |
| `DISABLE_PROMPT_CACHING_OPUS`   | Opus만 비활성화     |
| `DISABLE_PROMPT_CACHING_FABLE`  | Fable만 비활성화    |

조직 전체에서 caching 정책을 설정하려면 이러한 변수 중 하나 또는 [TTL 변수](#cache-lifetime)를 [관리되는 설정](/docs/ko/managed-settings)의 `env` 블록에 넣습니다. 일반적인 사용을 위해서는 caching을 활성화된 상태로 두십시오.

<h2 id="related-resources">
  관련 리소스
</h2>

* [Claude Code 구축에서 배운 교훈: 프롬프트 캐싱이 모든 것입니다](https://claude.com/blog/lessons-from-building-claude-code-prompt-caching-is-everything): Plan 모드, 연기된 도구 로딩 및 압축의 설계 근거
* [컨텍스트 윈도우 탐색](/docs/ko/context-window): 컨텍스트에 로드되는 내용 및 시기
* [토큰 사용량 감소](/docs/ko/costs#reduce-token-usage): 컨텍스트 크기 관리를 위한 캐싱 이상의 전략
* [비용 추적 및 감소](/docs/ko/agent-sdk/cost-tracking): Agent SDK 호출자를 위한 캐시 토큰 추적 및 TTL 구성
* [프롬프트 캐싱](https://platform.claude.com/docs/en/build-with-claude/prompt-caching): 기본 API 메커니즘, 중단점 및 가격 책정
