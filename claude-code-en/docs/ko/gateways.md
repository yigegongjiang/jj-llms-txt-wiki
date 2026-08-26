> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 게이트웨이를 통해 Claude Code 실행

> Claude Code를 자체 호스팅 게이트웨이를 통해 라우팅하여 중앙 집중식 자격 증명, 사용량 추적 및 비용 제어를 수행합니다. 아키텍처, Anthropic의 Claude 앱 게이트웨이 및 다른 게이트웨이 제품 사용을 다룹니다.

게이트웨이는 Claude Code와 모델 제공자 사이에서 조직이 실행하는 프록시입니다. Claude Code는 제공자에게 직접 보내지 않고 게이트웨이로 API 트래픽을 전송하며, 게이트웨이는 조직이 보유한 자격 증명을 사용하여 이를 전달합니다. 개발자는 제공자 자격 증명을 보유하지 않고 게이트웨이에 인증하므로, 인증, 사용량 추적, 예산 및 감사 로깅이 사용자가 제어하는 한 곳에서 발생합니다.

Claude Code에는 `claude` 바이너리에 포함된 자체 호스팅 게이트웨이인 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway)가 포함되어 있으므로, 게이트웨이를 실행하기 위해 별도의 게이트웨이 제품을 채택할 필요가 없습니다. 조직이 이미 [LLM 게이트웨이](/docs/ko/llm-gateway)를 실행 중인 경우, Claude Code도 이와 함께 작동합니다.

이 페이지에서 다루는 내용:

* [게이트웨이가 Claude Code와 제공자 사이에 어떻게 위치하는지](#how-a-gateway-works)
* [Claude 앱 게이트웨이와 이미 실행 중인 게이트웨이 중 선택](#choose-a-gateway)
* [게이트웨이가 claude.ai 구독과 상호 작용하는 방식](#subscriptions-and-gateways)
* [게이트웨이와 별도로 구성되는 항목](#configure-separately-from-the-gateway)

<h2 id="how-a-gateway-works">
  게이트웨이의 작동 방식
</h2>

각 개발자의 Claude Code는 게이트웨이의 주소를 가리키도록 설정되고 게이트웨이에서 발급한 자격 증명으로 인증합니다.

게이트웨이는 개발자를 인증하고, 구성한 모든 액세스 및 예산 규칙을 적용하며, 조직의 자격 증명을 사용하여 요청을 제공자에게 전달합니다. 제공자는 Anthropic의 API이거나 Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry와 같은 [클라우드 제공자](/docs/ko/third-party-integrations)일 수 있습니다. 게이트웨이의 구성이 결정합니다. Claude 앱 게이트웨이 또는 단일 Anthropic 형식 엔드포인트를 노출하는 다른 게이트웨이를 사용하면, 제공자를 변경해도 개발자 머신을 건드릴 필요가 없습니다.

<Frame>
  <img src="https://mintcdn.com/claude-code/-uq-4JE0W_JO5Er5/images/llm-gateway-flow.svg?fit=max&auto=format&n=-uq-4JE0W_JO5Er5&q=85&s=1c1a8dcc0cfcc3a58652cc8e28cd3e20" alt="Claude Code가 게이트웨이를 통해 라우팅되는 것을 보여주는 다이어그램입니다. 개발자 머신 영역에서 Claude Code CLI 및 VS Code 확장은 개발자별 자격 증명을 사용하여 게이트웨이 주소로 요청을 전송합니다. 사용자 인프라로 표시된 영역에서 게이트웨이는 인증, 사용량 추적, 예산 및 라우팅을 처리하고 조직의 자격 증명을 사용하여 요청을 전달합니다. 모델 제공자 영역에서 실선 화살표는 구성한 제공자(Anthropic API로 표시)로 이동하고, 점선 화살표는 Amazon Bedrock, Google Cloud 및 Microsoft Foundry를 예로 들어 다른 제공자 옵션으로 이동합니다." width="780" height="322" data-path="images/llm-gateway-flow.svg" />
</Frame>

두 가지 종류의 자격 증명이 관련됩니다:

* **개발자 자격 증명**: 각 개발자가 게이트웨이에서 발급한 자신의 자격 증명을 보유합니다. 게이트웨이에 인증하고 사용량 추적에서 개발자를 식별합니다.
* **제공자 자격 증명**: 게이트웨이가 제공자 계정에 대한 하나의 자격 증명을 보유하며, 모든 전달된 트래픽에서 공유됩니다.

<h2 id="choose-a-gateway">
  게이트웨이 선택
</h2>

Claude Code는 Anthropic의 자체 게이트웨이 또는 조직이 이미 실행 중인 게이트웨이와 함께 작동합니다.

<h3 id="claude-apps-gateway">
  Claude 앱 게이트웨이
</h3>

Claude 앱 게이트웨이는 `claude` 바이너리에 포함된 Anthropic의 자체 호스팅 게이트웨이입니다. Amazon Bedrock, Claude Platform on AWS, Google Cloud, Microsoft Foundry 또는 Anthropic API를 업스트림으로 라우팅합니다. 개발자는 `/login`을 통해 회사 ID 제공자로 로그인하고, 게이트웨이는 IdP 그룹별로 모델 액세스 및 [관리 설정](/docs/ko/permissions#managed-settings)을 적용하며, [OpenTelemetry Protocol (OTLP)](/docs/ko/monitoring-usage) 사용량 메트릭을 자신의 관찰성 스택으로 내보냅니다.

각 Claude Code 릴리스와 함께 빌드되고 테스트되므로, Claude Code가 전송하는 헤더 및 요청 필드를 전달합니다. 별도로 유지 관리되는 게이트웨이는 각 릴리스에서 해당 헤더 및 필드가 변경될 때 [전달 규칙을 업데이트](/docs/ko/llm-gateway-protocol#forward-as-open-lists)해야 합니다. Claude 앱 게이트웨이는 CLI와 함께 릴리스되므로 최신 상태를 유지할 목록이 없습니다. [가용성 및 제한 사항](/docs/ko/claude-apps-gateway#availability-and-limitations)에서 게이트웨이 세션에서 다르게 작동하는 작은 기능 집합을 참조하세요.

게이트웨이 로그인은 브라우저 SSO 단계이며 서비스 토큰 흐름이 없으므로, 승인할 개발자가 없는 CI 파이프라인은 이를 통해 인증할 수 없습니다. 이러한 경우 제공자에 대해 직접 구성하세요. Agent SDK 세션 및 개발자가 로그인한 머신에서의 `claude -p` 실행은 해당 머신의 게이트웨이 세션을 사용하며 해당 정책의 적용을 받습니다. [CI 파이프라인 및 원격 머신](/docs/ko/claude-apps-gateway#ci-pipelines-and-remote-machines)을 참조하세요.

배포하려면 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway)를 참조하세요.

<h3 id="other-gateways">
  다른 게이트웨이
</h3>

조직이 이미 LLM 게이트웨이 또는 API 게이트웨이를 실행 중인 경우, 대신 이를 사용할 수 있습니다. Anthropic은 다른 게이트웨이 제품을 승인, 유지 관리 또는 감사하지 않으며, 어떤 게이트웨이를 통해서도 Claude Code를 비 Claude 모델로 라우팅하는 것을 지원하지 않습니다. 관리자 롤아웃 체크리스트, 게이트웨이가 구현해야 할 사항 및 Claude Code를 이에 가리키는 방법은 [다른 LLM 게이트웨이](/docs/ko/llm-gateway)를 참조하세요.

<h2 id="subscriptions-and-gateways">
  구독 및 게이트웨이
</h2>

개발자가 게이트웨이 자격 증명을 사용하여 게이트웨이를 통해 연결할 때, 사용량은 API 요금으로 조직의 제공자 계정에 청구되며, 해당 claude.ai 구독은 사용되거나 청구되지 않습니다. 실행 중인 게이트웨이에 대해 [`ANTHROPIC_AUTH_TOKEN`](/docs/ko/env-vars)을 설정하거나 `/login`을 사용하여 Claude 앱 게이트웨이에 로그인하면 해당 세션에 대한 구독 로그인이 비활성화됩니다. 해당 자격 증명으로 전달된 모든 요청은 게이트웨이의 제공자 자격 증명 뒤의 계정에 청구됩니다.

예외는 게이트웨이 자격 증명 없이 `ANTHROPIC_BASE_URL`만 설정하는 경우입니다. 요청은 여전히 게이트웨이를 통해 라우팅되지만, 저장된 claude.ai 로그인은 활성 자격 증명으로 유지되므로 구독의 사용량 제한 및 청구가 적용됩니다. [다른 LLM 게이트웨이](/docs/ko/llm-gateway#subscriptions-and-gateways)에서 해당 구성 및 작동하기 위해 게이트웨이가 전달해야 할 사항을 다룹니다.

<h2 id="configure-separately-from-the-gateway">
  게이트웨이와 별도로 구성
</h2>

게이트웨이는 모델 API 요청을 라우팅합니다. 게이트웨이가 처리할 것으로 예상할 수 있는 몇 가지 항목은 다른 곳에서 구성됩니다:

* **어떤 모델이 응답하는지**: `/model` 명령 또는 [모델 환경 변수](/docs/ko/model-config#setting-your-model)로 모델을 선택합니다. 게이트웨이는 요청이 어디로 가는지 결정하며, 개발자가 선택한 모델이 아닙니다. Claude 앱 게이트웨이는 그룹별 `availableModels` 허용 목록으로 선택을 제한할 수 있지만, 개발자는 여전히 그 범위 내에서 선택합니다.
* **다른 네트워크 트래픽**: Claude Code 자체는 버전 확인을 전송하고 게이트웨이 경로와 별도로 Anthropic에서 직접 다운로드합니다. 선택적 클라이언트 원격 분석 스트림도 켜져 있는지 여부는 제공자에 따라 다릅니다. [원격 분석 기본값 표](/docs/ko/data-usage#telemetry-services)에서 각 경우를 다룹니다. 로그인한 Claude 앱 게이트웨이 세션에서 게이트웨이 자격 증명은 Anthropic 바운드 분석을 비활성화하고, [원격 분석 전달](/docs/ko/claude-apps-gateway-config#telemetry)이 구성된 경우 OTLP 내보내기를 게이트웨이에 고정합니다. 네트워크는 여전히 [필수 도메인](/docs/ko/network-config)으로의 송신이 필요하거나 [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/ko/env-vars)을 설정하여 선택적 스트림을 끕니다.
* **회사 HTTP 프록시**: `HTTPS_PROXY`는 Claude Code와 게이트웨이를 포함하여 통신하는 모든 서버 사이에 위치합니다. 네트워크에 필요한 경우, 게이트웨이 외에 [프록시를 구성](/docs/ko/network-config)하세요. Claude 앱 게이트웨이의 경우, [로그인은 프록시 호스트도 개인 네트워크에 있는지 확인](/docs/ko/claude-apps-gateway#prerequisites)합니다. 그렇지 않으면 게이트웨이 호스트를 `NO_PROXY`에 추가하여 CLI가 직접 연결하도록 하세요.

<h2 id="next-steps">
  다음 단계
</h2>

다음 페이지는 게이트웨이를 실행하는 사람에 따라 다릅니다. Anthropic의 게이트웨이는 `claude` 바이너리에서 실행되며 자체 설정 가이드가 있습니다. 조직이 이미 실행 중인 게이트웨이는 구현할 프로토콜과 관리자 롤아웃 체크리스트가 있습니다.

* [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway) - SSO 로그인 및 OTLP 원격 분석을 사용하여 Anthropic의 자체 호스팅 게이트웨이 배포
* [다른 LLM 게이트웨이](/docs/ko/llm-gateway) - 조직이 이미 실행 중인 게이트웨이가 구현해야 할 사항 및 Claude Code를 이에 가리키는 방법
* [조직을 위해 Claude Code 설정](/docs/ko/admin-setup) - 게이트웨이가 한 부분인 더 넓은 롤아웃 결정
