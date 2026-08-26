> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 다른 LLM gateway

> 조직이 이미 실행 중인 LLM gateway를 통해 Claude Code를 라우팅합니다. Claude Code를 gateway에 연결하고, 조직을 위해 gateway를 배포하고, Claude Code가 gateway에 전송하는 내용을 다룹니다.

이 섹션에서는 [Claude 앱 gateway](/docs/ko/claude-apps-gateway) 대신 조직이 이미 실행 중인 gateway 제품을 사용하는 것을 다룹니다. gateway가 무엇인지, Claude Code와 공급자 간에 어떻게 위치하는지, Claude 앱 gateway와 다른 제품 중에서 선택하는 방법에 대해서는 [gateway 개요](/docs/ko/gateways)를 참조하세요.

<Note>
  * 기존 gateway에 연결하는 개발자인 경우: [Claude Code를 gateway에 연결](/docs/ko/llm-gateway-connect)
  * 조직을 위해 gateway를 배포하는 관리자인 경우: [gateway를 배포 및 배포](/docs/ko/llm-gateway-rollout)
  * gateway 제품을 구성하는 경우: [gateway 프로토콜 참조](/docs/ko/llm-gateway-protocol)
</Note>

[지원되는 API 형식](/docs/ko/llm-gateway-protocol#api-formats)을 노출하는 모든 gateway가 작동합니다. Anthropic은 제3자 gateway 제품을 보증, 유지 관리 또는 감사하지 않으며, 모든 gateway를 통해 Claude Code를 비Claude 모델로 라우팅하는 것을 지원하지 않습니다. gateway를 자체 문서에 따라 배포한 다음 아래의 [배포 단계](#roll-out-a-gateway)로 Claude Code 측의 배포를 완료합니다.

<h2 id="what-a-gateway-provides">
  gateway가 제공하는 것
</h2>

gateway는 조직이 다음을 관리할 수 있는 한 곳을 제공합니다:

* **자격 증명**: 공급자 키는 서버 측에 유지되고, 개발자는 gateway 자격 증명을 대신 보유합니다
* **사용량 추적**: 요청을 처리하는 공급자와 관계없이 개발자 또는 팀별로 사용량을 속성화합니다
* **비용 제어**: 한 곳에서 예산 및 속도 제한을 적용합니다
* **감사 로깅**: 규정 준수를 위해 모든 모델 요청을 기록합니다
* **공급자 전환**: 개발자 머신을 건드리지 않고 gateway 구성에서 공급자를 변경합니다

이 중 공급자 전환을 제외한 모든 것은 업스트림이 Anthropic의 API이든 [클라우드 공급자](/docs/ko/third-party-integrations)이든 적용됩니다. 개발자 머신을 재구성하지 않고도 공급자 전환이 가능하려면 gateway가 업스트림과 관계없이 단일 [Anthropic 형식 엔드포인트](/docs/ko/llm-gateway-protocol#api-formats)를 노출해야 합니다. 공급자 자체 형식을 노출하는 gateway는 클라이언트 구성을 해당 공급자에 연결합니다.

트레이드오프는 gateway가 조직이 운영하는 인프라가 된다는 것입니다. Claude Code는 각 릴리스마다 기능을 추가하고, gateway가 이를 전달하지 않으면 해당 기능이 손상되므로, gateway 제품은 Claude Code가 진화함에 따라 최신 상태로 유지되어야 합니다. [gateway 프로토콜 참조](/docs/ko/llm-gateway-protocol)는 전달할 내용을 다룹니다.

<h2 id="roll-out-a-gateway">
  gateway 배포
</h2>

조직에 LLM gateway를 배포할 준비가 되면, 선택한 gateway 제품이 무엇이든 순서는 동일합니다:

1. gateway를 배포하고 공급자 자격 증명을 제공하여 전달하는 요청을 인증할 수 있도록 합니다.
2. 각 개발자에게 gateway 자격 증명을 발급하여 사용량이 개발자에게 속성화되고 오프보딩이 하나의 자격 증명을 취소하도록 합니다.
3. [관리되는 설정 파일](/docs/ko/settings#settings-files) 및 비밀 도구를 통해 구성을 배포하여 모든 머신이 기본 URL과 자격 증명을 받도록 합니다. 둘 다 배포되면 개발자는 아무것도 구성하지 않습니다. 설정 배포가 없으면 개발자는 [연결 페이지](/docs/ko/llm-gateway-connect)를 따라 변수를 직접 설정합니다.
4. 각 개발자가 [Claude Code에서 구성을 확인](/docs/ko/llm-gateway-connect#check-for-an-existing-configuration)하도록 하여 배포 문제가 gateway에 의존하기 전에 표면화되도록 합니다.

[조직을 위해 LLM gateway 배포](/docs/ko/llm-gateway-rollout)는 각 단계를 안내하고 각 단계에서 배포할 구성 파일을 보여줍니다. gateway는 조직 설정의 한 부분입니다. 정책 적용, 사용량 가시성 및 데이터 처리 결정의 경우 [조직을 위해 Claude Code 설정](/docs/ko/admin-setup)을 참조하세요.

<h2 id="subscriptions-and-gateways">
  구독 및 gateway
</h2>

[gateway 자격 증명 변수](/docs/ko/llm-gateway-connect#set-the-credential-variable) 또는 `apiKeyHelper`가 활성화되어 있는 동안 개발자의 claude.ai 구독은 사용되지 않습니다: 자격 증명이 해당 세션에 대한 구독 로그인을 대체하고, 구독의 사용량 제한이 적용되지 않습니다. 해당 트래픽은 gateway가 전달하는 자격 증명의 소유자(예: 조직의 Anthropic Console 계정 또는 gateway가 그곳으로 라우팅할 때 Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry 계정)에게 토큰당 청구됩니다.

[`ANTHROPIC_BASE_URL`](/docs/ko/llm-gateway-connect#set-the-base-url-and-credential)은 Claude Code를 gateway로 가리키는 변수입니다. gateway 자격 증명 없이 해당 변수만 설정하면 구독을 대체하지 않습니다. 요청은 여전히 gateway를 통해 라우팅되지만 저장된 claude.ai 로그인이 활성 자격 증명으로 유지되므로 해당 사용량 제한 및 청구가 적용됩니다. 이 트래픽을 Anthropic에 전달하는 gateway는 `anthropic-beta`에서 OAuth 기능을 전달해야 합니다. [요청 헤더 참조](/docs/ko/llm-gateway-protocol#request-headers)를 참조하세요.

<h2 id="related-pages">
  관련 페이지
</h2>

* [Gateway 개요](/docs/ko/gateways): gateway가 작동하는 방식 및 Claude 앱 gateway와 다른 제품 중에서 선택하는 방법
* [Claude 앱 gateway](/docs/ko/claude-apps-gateway): SSO 로그인 및 OTLP 원격 분석을 포함한 Anthropic의 자체 호스팅 gateway
* [Claude Code를 LLM gateway에 연결](/docs/ko/llm-gateway-connect): 자신의 머신에서 기본 URL 및 자격 증명을 설정하고, 표면별 구성 및 문제 해결 테이블 포함
* [조직을 위해 LLM gateway 배포](/docs/ko/llm-gateway-rollout): gateway 배포, 개발자 자격 증명 발급 및 관리되는 설정 배포를 위한 관리자 체크리스트
* [Gateway 프로토콜 참조](/docs/ko/llm-gateway-protocol): Claude Code가 gateway에 전송하는 내용, gateway를 구성하는 운영자를 위해, 엔드포인트, 전달할 헤더 및 기능 통과를 다룸
