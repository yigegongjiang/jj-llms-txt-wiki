> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Zero data retention

> Claude for Enterprise에서 Claude Code의 Zero Data Retention(ZDR)에 대해 알아보세요. 범위, 비활성화된 기능, 활성화 요청 방법을 포함합니다.

Zero Data Retention(ZDR)은 Claude for Enterprise를 통해 사용할 때 Claude Code에서 사용 가능합니다. ZDR이 활성화되면 Claude Code 세션 중에 생성된 프롬프트와 모델 응답은 실시간으로 처리되며 응답이 반환된 후 Anthropic에서 저장되지 않습니다. 단, 법률 준수 또는 오용 방지가 필요한 경우는 제외합니다.

<Note>
  ZDR은 표준 Claude for Enterprise 플랜에 포함되지 않으며 관리자 설정에서 활성화할 수 없습니다. 이는 적격 계정에서만 사용 가능하며 Anthropic의 별도 활성화가 필요합니다. 조직에서 ZDR이 필요한 경우 [영업팀에 문의](https://www.anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=zero_data_retention_request)하거나 Anthropic 계정 담당자에게 연락하여 적격 여부를 확인하세요.
</Note>

Claude for Enterprise의 ZDR은 엔터프라이즈 고객에게 Zero Data Retention으로 Claude Code를 사용하고 관리 기능에 액세스할 수 있는 기능을 제공합니다:

* 사용자별 비용 제어
* [분석](/docs/ko/analytics) 대시보드
* [서버 관리 설정](/docs/ko/server-managed-settings)
* 감사 로그

Claude for Enterprise의 Claude Code에 대한 ZDR은 Anthropic의 직접 플랫폼에만 적용됩니다. Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry의 Claude 배포의 경우 해당 플랫폼의 데이터 보존 정책을 참조하세요.

<h2 id="zdr-scope">
  ZDR 범위
</h2>

ZDR은 Claude for Enterprise의 Claude Code 추론을 포함합니다.

<Warning>
  ZDR은 조직별로 활성화됩니다. 각 새로운 조직은 Anthropic 계정 팀에서 별도로 ZDR을 활성화해야 합니다. ZDR은 동일한 계정 아래에 생성된 새로운 조직에 자동으로 적용되지 않습니다. 새로운 조직에 대해 ZDR을 활성화하려면 계정 팀에 문의하세요.
</Warning>

<h3 id="what-zdr-covers">
  ZDR이 포함하는 것
</h3>

ZDR은 Claude for Enterprise의 Claude Code를 통해 이루어진 모델 추론 호출을 포함합니다. 터미널에서 Claude Code를 사용할 때 전송하는 프롬프트와 Claude가 생성하는 응답은 Anthropic에서 보존되지 않습니다. 이는 ZDR 조직에서 사용 가능한 모든 모델에 적용됩니다. 일부 모델은 데이터 보존이 필요하며 ZDR에서 사용할 수 없습니다. [ZDR에서의 모델 가용성](#model-availability-under-zdr)을 참조하세요.

<h3 id="what-zdr-does-not-cover">
  ZDR이 포함하지 않는 것
</h3>

ZDR은 ZDR이 활성화된 조직의 경우에도 다음을 포함하지 않습니다. 이러한 기능은 [표준 데이터 보존 정책](/docs/ko/data-usage#data-retention)을 따릅니다:

| 기능             | 세부 정보                                                                                                                               |
| -------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| claude.ai의 채팅  | Claude for Enterprise 웹 인터페이스를 통한 채팅 대화는 ZDR에 포함되지 않습니다.                                                                            |
| Cowork         | Cowork 세션은 ZDR에 포함되지 않습니다.                                                                                                          |
| Claude Code 분석 | 프롬프트 또는 모델 응답을 저장하지 않지만 계정 이메일 및 사용 통계와 같은 생산성 메타데이터를 수집합니다. 기여도 메트릭은 ZDR 조직에서 사용할 수 없습니다. [분석 대시보드](/docs/ko/analytics)는 사용 메트릭만 표시합니다. |
| 사용자 및 시트 관리    | 계정 이메일 및 시트 할당과 같은 관리 데이터는 표준 정책에 따라 보존됩니다.                                                                                         |
| 타사 통합          | 타사 도구, MCP servers 또는 기타 외부 통합에서 처리한 데이터는 ZDR에 포함되지 않습니다. 해당 서비스의 데이터 처리 관행을 독립적으로 검토하세요.                                           |

<h2 id="features-disabled-under-zdr">
  ZDR에서 비활성화된 기능
</h2>

Claude for Enterprise의 Claude Code 조직에 대해 ZDR이 활성화되면 프롬프트 또는 완성을 저장해야 하는 특정 기능이 백엔드 수준에서 자동으로 비활성화됩니다:

| 기능                                               | 이유                                         |
| ------------------------------------------------ | ------------------------------------------ |
| [웹의 Claude Code](/docs/ko/claude-code-on-the-web)     | 대화 기록의 서버 측 저장이 필요합니다.                     |
| Desktop 앱의 [클라우드 세션](/docs/ko/desktop#cloud-sessions) | 프롬프트 및 완성을 포함하는 지속적인 세션 데이터가 필요합니다.        |
| [Artifacts](/docs/ko/artifacts)                       | Anthropic 운영 인프라에 게시된 페이지 콘텐츠를 저장해야 합니다.   |
| 피드백 제출(`/feedback`)                              | 피드백을 제출하면 대화 데이터가 Anthropic으로 전송됩니다.       |
| [원격 제어](/docs/ko/remote-control)                      | Anthropic 서버에 세션 기록을 저장하여 기기 간 대화를 동기화합니다. |

이러한 기능은 클라이언트 측 표시에 관계없이 백엔드에서 차단됩니다. 시작 중에 Claude Code 터미널에서 비활성화된 기능이 표시되면 이를 사용하려고 시도하면 조직의 정책이 해당 작업을 허용하지 않음을 나타내는 오류가 반환됩니다.

향후 기능도 프롬프트 또는 완성을 저장해야 하는 경우 비활성화될 수 있습니다.

<h3 id="model-availability-under-zdr">
  ZDR에서의 모델 가용성
</h3>

Claude Fable 5는 영구 데이터 보존 비활성화가 활성화된 조직에서는 사용할 수 없습니다. 이 모델 클래스는 [데이터 보존이 필요하므로](https://platform.claude.com/docs/en/manage-claude/api-and-data-retention#model-specific-data-retention-requirements) ZDR 조직의 요청은 이를 통해 처리될 수 없습니다. 모델은 ZDR 조직의 `/model` 선택기에서 없거나 ZDR을 비활성화해야 한다는 공지와 함께 비활성화된 것으로 표시되며, 클라이언트 구성에 관계없이 서버는 이에 대한 요청을 거부합니다.

다른 모델은 ZDR에서 계속 사용할 수 있습니다. Fable 5는 기본 모델이 아니며, Fable 5가 사용 가능한 경우 Fable 5로 확인되는 `best` 별칭은 ZDR 조직을 포함하여 사용할 수 없는 조직의 경우 Opus로 확인됩니다.

<h2 id="data-retention-for-policy-violations">
  정책 위반에 대한 데이터 보존
</h2>

ZDR이 활성화된 경우에도 Anthropic은 법률에서 요구하거나 Usage Policy 위반을 해결하기 위해 필요한 경우 데이터를 보존할 수 있습니다. 세션이 정책 위반으로 플래그되면 Anthropic은 관련 입력 및 출력을 최대 2년 동안 보존할 수 있으며, 이는 Anthropic의 표준 ZDR 정책과 일치합니다.

<h2 id="request-zdr">
  ZDR 요청
</h2>

Claude for Enterprise의 Claude Code에 대해 ZDR을 요청하려면 [영업팀에 문의](https://www.anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=zero_data_retention_request)하거나 Anthropic 계정 팀에 문의하세요. 계정 팀이 내부적으로 요청을 제출하면 Anthropic이 적격성을 확인한 후 조직에서 ZDR을 검토하고 활성화합니다. 모든 활성화 작업은 감사 로그에 기록됩니다.

현재 종량제 API 키를 통해 Claude Code에 대해 ZDR을 사용 중인 경우 Claude for Enterprise로 전환하여 Claude Code에 대한 ZDR을 유지하면서 관리 기능에 액세스할 수 있습니다. 마이그레이션을 조정하려면 계정 팀에 문의하세요.
