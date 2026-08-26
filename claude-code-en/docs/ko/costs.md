> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 비용을 효과적으로 관리하기

> 토큰 사용량을 추적하고, 팀 지출 한도를 설정하며, 컨텍스트 관리, 모델 선택, 확장 사고 설정 및 전처리 hooks를 통해 Claude Code 비용을 절감합니다.

Claude Code는 API 토큰 소비량으로 청구됩니다. 구독 요금제 가격(Pro, Max, Team, Enterprise)은 [claude.com/pricing](https://claude.com/pricing)을 참조하십시오. 개발자당 비용은 모델 선택, 코드베이스 크기, 여러 인스턴스 실행 또는 자동화와 같은 사용 패턴에 따라 크게 달라집니다.

엔터프라이즈 배포 전반에 걸쳐 평균 비용은 개발자당 활성 일일 약 $13이며, 개발자당 월 $150-250이고, 90%의 사용자는 활성 일일 비용이 \$30 이하로 유지됩니다. 팀의 지출을 추정하려면 작은 파일럿 그룹으로 시작하고 아래의 추적 도구를 사용하여 더 광범위한 롤아웃 전에 기준선을 설정하십시오.

이 페이지에서는 [비용 추적 방법](#track-your-costs), [팀 비용 관리](#manage-costs-for-your-organization), [토큰 사용량 감소](#reduce-token-usage) 방법을 다룹니다.

<h2 id="track-your-costs">
  비용 추적
</h2>

<h3 id="using-the-/usage-command">
  `/usage` 명령 사용
</h3>

<Note>
  `/usage`의 Session 블록은 API 토큰 사용량을 표시하며 API 사용자를 위한 것입니다. Claude Max 및 Pro 구독자는 구독에 사용량이 포함되어 있으므로 세션 비용 수치는 청구 목적으로 관련이 없습니다. 구독자는 동일한 화면에서 요금제 사용량 막대, 활동 통계 및 사용량 분석을 볼 수 있습니다.
</Note>

`/usage` 명령 상단의 Session 블록은 현재 세션에 대한 자세한 토큰 사용량 통계를 표시합니다. 달러 수치는 토큰 수에서 로컬로 계산된 추정치이며 실제 청구서와 다를 수 있습니다. 권위 있는 청구를 위해 [Claude Console](https://platform.claude.com/usage)의 사용량 페이지를 참조하십시오.

```text theme={null}
Total cost:            $0.55
Total duration (API):  6m 19.7s
Total duration (wall): 6h 33m 10.2s
Total code changes:    0 lines added, 0 lines removed
```

Pro, Max, Team 또는 Enterprise 요금제에서 `/usage`는 요금제 한도에 포함되는 항목의 분석도 표시합니다. 최근 사용량을 skills, subagents, plugins 및 개별 MCP 서버에 귀속시키며, 각각은 전체의 백분율로 표시됩니다. `d` 또는 `w`를 눌러 지난 24시간과 지난 7일 사이를 전환할 수 있습니다. 수치는 근사치이며 이 기기의 로컬 세션 기록에서 계산되므로 다른 기기 또는 claude.ai의 사용량은 포함되지 않습니다.

요금제 한도에 대한 요청이 실패할 때(대부분 사용량 엔드포인트가 속도 제한되기 때문), `/usage`는 이 기기에서 지난 60분 이내에 로드한 마지막 사용량 막대를 표시하며, 해당 데이터를 얼마나 오래 전에 가져왔는지 나타내는 `Showing last-known usage` 메모가 함께 표시됩니다. `r`을 눌러 다시 시도하면, 성공적인 재시도는 마지막으로 알려진 막대를 새로운 데이터로 바꿉니다. 지난 60분 이내의 스냅샷이 없으면 `/usage`는 사용량 엔드포인트가 속도 제한되었다고 보고하고 동일한 재시도 단축키를 제공합니다. v2.1.208 이전에는 아직 사용량을 로드하지 않은 세션에서 속도 제한된 요청이 항상 막대 없이 오류를 표시했습니다.

[VS Code 확장](/docs/ko/vs-code#check-account-and-usage)에서 동일한 분석이 Day 및 Week 토글이 있는 Account & usage 대화 상자에 나타납니다. Claude Code v2.1.174 이상이 필요합니다.

<h3 id="set-a-spend-limit-on-pro-and-max">
  Pro 및 Max에서 지출 한도 설정
</h3>

Pro 및 Max 요금제에서 `/usage-credits` 명령은 CLI에서 [사용량 크레딧](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)을 관리하는 대화 상자를 엽니다. 대화 상자에서 다음을 수행할 수 있습니다:

* 계정에 대한 사용량 크레딧 활성화
* 더 많은 사용량 크레딧 구매(나열된 번들 또는 사용자 정의 금액)
* 월간 지출 한도 설정, 변경 또는 제거
* 자동 재로드 구성(잔액이 설정한 임계값 아래로 떨어질 때 자동으로 더 많은 사용량 크레딧 구매)

v2.1.207 이전의 Claude Code 버전 및 CLI 내 대화 상자를 사용할 수 없는 계정에서는 `/usage-credits`가 브라우저에서 사용량 크레딧 청구 페이지를 엽니다. Team 및 Enterprise 요금제에서 청구 액세스 권한이 있는 구성원은 동일한 브라우저 페이지를 받고, 청구 액세스 권한이 없는 구성원은 CLI에서 관리자에게 사용량 크레딧을 활성화하거나 한도를 높이도록 요청합니다.

월간 지출 한도 변경에는 계정의 청구 액세스 권한이 필요합니다. 사용량 크레딧이 아직 남아 있는 상태에서 한도에 도달하면 Claude Code는 한도를 높이거나 제거하라는 메시지를 표시하여 CLI를 떠나지 않고 계속 진행할 수 있습니다.

사용자 정의 구매 금액, 월간 지출 한도, 자동 재로드 임계값 및 목표와 같이 대화 상자에 입력하는 금액은 숫자여야 하며, 선택적으로 마침표와 1\~2개의 소수 자릿수가 뒤따를 수 있습니다(예: `20` 또는 `20.50`). 쉼표를 포함한 다른 입력은 인라인 오류를 표시하며 저장되지 않습니다. v2.1.207 이전 버전은 대화 상자를 표시하지 않으며 대신 청구 페이지를 엽니다.

Claude Code는 금액에 관계없이 모든 구매 및 모든 자동 재로드 변경을 확인하기 위해 `yes`를 입력하도록 요청하며, 구매 확인은 승인하는 세금 후 총액을 표시합니다. 월간 지출 한도 변경은 \$1,000 이상 또는 미국 달러가 아닌 청구 통화의 1,000 단위 이상에서만 동일한 입력 확인을 요청합니다. v2.1.208 이전에는 구매 및 자동 재로드 변경도 해당 임계값을 사용했으므로 더 작은 금액은 추가 입력 `yes` 단계 없이 표준 대화 상자 흐름을 거쳤습니다.

금액 필드는 제안된 값으로 미리 채워진 상태로 열리며, 입력하는 첫 번째 숫자는 제안에 추가되지 않고 제안을 대체합니다. 사용량 크레딧을 활성화하는 화면은 Cancel이 선택된 상태로 열리므로 이를 활성화하려면 의도적인 선택이 필요합니다. 둘 다 Claude Code v2.1.208 이상이 필요합니다.

<h2 id="manage-costs-for-your-organization">
  조직의 비용 관리
</h2>

조직이 Claude Code에 액세스하는 방식에 따라 사용할 수 있는 제어 기능이 달라집니다: Claude for Teams 또는 Enterprise 플랜, Claude Console 또는 클라우드 제공자입니다. Teams 및 Enterprise 플랜에서는 사용량이 각 멤버의 시트 할당량에서 차감됩니다. Console 및 클라우드 제공자에서는 사용량이 토큰당 조직에 청구됩니다. 조직이 여러 로그인 방법을 혼합하는 경우, 각 개발자는 인증한 방법에 따라 측정됩니다.

표는 각 설정을 지출 확인 위치, 지출 상한선 위치 및 사용자별 수치 추출 방법에 매핑합니다.

| 설정                                                                                    | 지출 확인                                                                                                               | 지출 상한선        | 사용자별 보고                                                                                                                                                                                                           |
| :------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------ | :------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Claude for Teams 또는 Enterprise](#claude-for-teams-and-enterprise)                    | [조직 분석의 지출 보고서](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) | 관리자 설정의 지출 한도 | [지출 보고서 CSV](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans); Enterprise의 [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) |
| [Claude Console (API)](#claude-console)                                               | [Console 사용량 페이지](https://platform.claude.com/usage)                                                                | 워크스페이스 지출 한도  | [Console 대시보드](https://platform.claude.com/claude-code), [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api)                                             |
| [Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry](#cloud-providers) | 클라우드 청구 콘솔                                                                                                          | 클라우드의 예산 제어   | [OpenTelemetry](/docs/ko/monitoring-usage) 또는 [LLM gateway](/docs/ko/llm-gateway)                                                                                                                                           |

[OpenTelemetry 내보내기](/docs/ko/monitoring-usage)는 모든 설정에서 작동하며 사용자별 토큰 및 비용 메트릭을 거의 실시간으로 자체 관찰성 스택으로 스트리밍하는 유일한 옵션입니다.

<h3 id="claude-for-teams-and-enterprise">
  Claude for Teams 및 Enterprise
</h3>

Claude for Teams 및 Enterprise 플랜에서 각 멤버의 Claude Code 사용량은 5시간 롤링 윈도우 및 주간 윈도우에서 재설정되는 시트당 할당량에서 차감됩니다. 할당량은 Claude 채팅 및 Cowork와 공유되며, 크기는 멤버의 [시트 계층](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan)(Standard 또는 Premium)에 따라 달라집니다. 제어 기능은 Claude Console이 아닌 claude.ai 관리자 콘솔에 있습니다.

* **지출 확인**: [조직 분석의 지출 보고서](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans)는 사용자별 및 모델별 예상 지출을 CSV 내보내기와 함께 매일 업데이트되는 형태로 표시합니다. 보고서는 사용 크레딧 지출을 포함하며 사용 크레딧이 활성화된 후에 나타납니다. 시트 할당량 내의 사용량은 달러로 측정되지 않습니다.
* **채택 확인**: [분석 대시보드](https://claude.ai/analytics/claude-code)는 일일 활성 사용자, 세션 및 기여 메트릭을 CSV 내보내기와 함께 표시합니다. [분석으로 팀 사용량 추적](/docs/ko/analytics)을 참조하십시오.
* **지출 상한선**: 시트 할당량이 기본 상한선입니다. 멤버가 이를 초과하여 계속하도록 허용하려면 [사용 크레딧](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)을 활성화하고 조직, 그룹 또는 개별 멤버 수준에서 지출 한도를 설정하십시오.
* **사용자별 수치 추출**: Enterprise 플랜에서 [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics)는 Claude Code를 포함한 Claude 전체 표면에서 사용자별 사용량 및 비용 보고서를 반환합니다. Primary Owner는 [claude.ai/analytics/api-keys](https://claude.ai/analytics/api-keys)에서 `read:analytics` 범위를 가진 키를 생성합니다. Teams 플랜에서는 [지출 보고서 CSV](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans)를 내보내십시오. 이는 사용자별 및 모델별 토큰 사용량 및 예상 지출을 나열합니다.

[Claude Enterprise 소비 가이드](https://support.claude.com/en/articles/14782391-claude-enterprise-consumption-guide)는 관리자를 위한 계획 참고 자료입니다. Claude 채팅, Claude Code 및 Cowork 전체에서 소비가 어떻게 다른지 설명하고 예산 책정을 위한 사용자별 달러 시작점을 제공합니다. 채팅 시트보다 코딩 시트에 더 많은 예산을 할당하십시오: 각 Claude Code 턴은 파일 내용, 도구 호출 및 다단계 추론을 포함하므로 하나의 디버깅 세션이 하루의 채팅보다 더 많이 소비할 수 있습니다.

<h3 id="claude-console">
  Claude Console
</h3>

API 조직은 [워크스페이스](https://platform.claude.com/docs/en/build-with-claude/workspaces)를 통해 Claude Code 지출을 관리합니다. [워크스페이스 지출 한도를 설정](https://platform.claude.com/docs/en/build-with-claude/workspaces#workspace-limits)하여 전체 Claude Code 지출을 제한하고 [Console에서 비용 및 사용량 보고서를 볼 수 있습니다](https://platform.claude.com/docs/en/build-with-claude/workspaces#usage-and-cost-tracking).

<Note>
  Claude Code를 Claude Console 계정으로 처음 인증할 때, "Claude Code"라는 워크스페이스가 자동으로 생성됩니다. 이 워크스페이스는 조직의 모든 Claude Code 사용에 대한 중앙 집중식 비용 추적 및 관리를 제공합니다. 이 워크스페이스에 대해 API 키를 생성할 수 없습니다. 이는 Claude Code 인증 및 사용 전용입니다.

  사용자 정의 속도 제한이 있는 조직의 경우, 이 워크스페이스의 Claude Code 트래픽은 조직의 전체 API 속도 제한에 포함됩니다. Claude Console의 이 워크스페이스의 한도 페이지에서 [워크스페이스 속도 제한](https://platform.claude.com/docs/ko/api/rate-limits#setting-lower-limits-for-workspaces)을 설정하여 Claude Code의 할당량을 제한하고 다른 프로덕션 워크로드를 보호할 수 있습니다.
</Note>

사용자별 보고의 경우, [Console 대시보드](https://platform.claude.com/claude-code)는 멤버별 지출 및 수락된 라인을 표시하고, [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api)는 [Admin API 키](https://platform.claude.com/settings/admin-keys)를 사용하여 동일한 일일 사용자별 메트릭을 프로그래밍 방식으로 반환합니다. [API 고객을 위한 분석](/docs/ko/analytics#access-analytics-for-api-customers)을 참조하십시오.

<h4 id="rate-limit-recommendations">
  속도 제한 권장사항
</h4>

팀을 위해 Claude Code를 설정할 때, 조직 규모에 따른 다음 분당 토큰(TPM) 및 분당 요청(RPM) 사용자당 권장사항을 고려하십시오:

| 팀 규모        | 사용자당 TPM  | 사용자당 RPM  |
| ----------- | --------- | --------- |
| 1-5 사용자     | 200k-300k | 5-7       |
| 5-20 사용자    | 100k-150k | 2.5-3.5   |
| 20-50 사용자   | 50k-75k   | 1.25-1.75 |
| 50-100 사용자  | 25k-35k   | 0.62-0.87 |
| 100-500 사용자 | 15k-20k   | 0.37-0.47 |
| 500+ 사용자    | 10k-15k   | 0.25-0.35 |

예를 들어, 200명의 사용자가 있는 경우, 각 사용자에 대해 20k TPM을 요청하거나 총 400만 TPM(200\*20,000 = 400만)을 요청할 수 있습니다.

팀 규모가 커질수록 사용자당 TPM이 감소하는 이유는 더 큰 조직에서 더 적은 수의 사용자가 Claude Code를 동시에 사용하는 경향이 있기 때문입니다. 이러한 속도 제한은 개별 사용자별이 아닌 조직 수준에서 적용되므로, 다른 사용자가 적극적으로 서비스를 사용하지 않을 때 개별 사용자는 일시적으로 계산된 할당량보다 더 많이 소비할 수 있습니다.

<Note>
  대규모 그룹과의 라이브 교육 세션과 같이 비정상적으로 높은 동시 사용 시나리오를 예상하는 경우, 사용자당 더 높은 TPM 할당이 필요할 수 있습니다.
</Note>

<h3 id="cloud-providers">
  클라우드 제공자
</h3>

Amazon Bedrock, Google Cloud의 Agent Platform 및 Microsoft Foundry에서 Claude Code는 클라우드 계정에 토큰당 청구되며, 지출 제어는 클라우드 제공자의 청구 콘솔에 있습니다. Claude Code는 클라우드에서 Anthropic으로 메트릭을 전송하지 않으므로, [분석 대시보드](/docs/ko/analytics) 및 Claude Code Analytics API는 이 사용량을 포함하지 않습니다.

사용자별 비용 귀속의 경우 세 가지 옵션이 있습니다:

* **OpenTelemetry**: 각 개발자의 머신에서 자체 관찰성 스택으로 [메트릭을 내보냅니다](/docs/ko/monitoring-usage). 이는 제공자와 관계없이 사용자별 토큰 수, 비용 및 도구 활동을 제공합니다.
* **Claude apps gateway**: 자체 호스팅된 [Claude apps gateway](/docs/ko/claude-apps-gateway)는 사용자별 사용량 귀속, 토큰 수를 포함한 OTLP 메트릭 및 이러한 제공자에 대한 [사용자별 지출 한도](/docs/ko/claude-apps-gateway-spend-limits)를 제공합니다.
* **LLM gateway**: 모든 Claude Code 트래픽을 키별 지출을 추적하는 프록시를 통해 라우팅합니다. 여러 대규모 엔터프라이즈는 [LiteLLM](/docs/ko/llm-gateway)을 사용하고 있으며, 이는 [키별 지출을 추적](https://docs.litellm.ai/docs/proxy/virtual_keys#tracking-spend)하는 오픈 소스 도구입니다. 이 프로젝트는 Anthropic과 무관하며 보안 감사를 받지 않았습니다.

<h3 id="when-a-developer-asks-about-a-limit">
  개발자가 한도에 대해 질문할 때
</h3>

개발자는 일반적으로 한도 질문을 관리자에게 가져오므로, 어떤 상한선에 도달했는지 아는 것이 도움이 됩니다. 세 가지 상황은 다른 의미를 가집니다:

* **"세션 한도에 도달했습니다" 또는 "주간 한도에 도달했습니다"**: 구독 플랜의 시트 기반 사용 윈도우입니다. 이러한 윈도우는 모든 모델에서 공유되므로 `/model`로 모델을 전환해도 액세스가 복원되지 않지만, 모델별 "Opus 한도에 도달했습니다" 메시지 후에도 개발자가 계속 작업할 수 있습니다. 메시지는 윈도우가 재설정될 때를 표시하며, 개발자는 [사용 크레딧](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)이 활성화된 경우 `/usage-credits`를 실행하여 할당량을 초과하는 사용을 요청할 수 있습니다. [사용 한도 오류](/docs/ko/errors#youve-hit-your-session-limit)를 참조하십시오.
* **컨텍스트 또는 자동 압축 경고**: 사용 한도가 아닙니다. 대화가 모델의 최대 입력 크기에 가까워졌으며, Claude Code는 공간을 확보하기 위해 이전 기록을 요약합니다. 개발자를 [토큰 사용량 감소](#reduce-token-usage)로 안내하십시오.
* **API 또는 클라우드 제공자 플랜에서 예상치 못한 높은 지출**: 일반적으로 절대 지워지지 않은 긴 세션 또는 기본 모델로 남겨진 Opus로 추적됩니다. 공유할 가장 영향력 있는 습관은 관련 없는 작업 간 지우기 및 작업에 맞는 모델 선택이며, 둘 다 [토큰 사용량 감소](#reduce-token-usage)에서 다룹니다.

<h3 id="agent-team-token-costs">
  에이전트 팀 토큰 비용
</h3>

[에이전트 팀](/docs/ko/agent-teams)은 각각 자체 컨텍스트 윈도우를 가진 여러 Claude Code 인스턴스를 생성합니다. 토큰 사용량은 활성 팀원의 수와 각 팀원이 실행되는 시간에 따라 확장됩니다.

에이전트 팀 비용을 관리 가능하게 유지하려면:

* 팀원에게 Sonnet을 사용하십시오. 조정 작업을 위해 기능과 비용의 균형을 맞춥니다.
* 팀을 작게 유지하십시오. 각 팀원은 자체 컨텍스트 윈도우를 실행하므로 토큰 사용량은 대략 팀 규모에 비례합니다.
* spawn 프롬프트를 집중적으로 유지하십시오. 팀원은 CLAUDE.md, MCP servers 및 skills를 자동으로 로드하지만, spawn 프롬프트의 모든 것이 처음부터 컨텍스트에 추가됩니다.
* 작업이 완료되면 팀을 정리하십시오. 활성 팀원은 유휴 상태에서도 계속 토큰을 소비합니다.
* 에이전트 팀은 기본적으로 비활성화되어 있습니다. [settings.json](/docs/ko/settings)에서 `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`을 설정하거나 환경에서 설정하여 활성화하십시오. [에이전트 팀 활성화](/docs/ko/agent-teams#enable-agent-teams)를 참조하십시오.

<h2 id="reduce-token-usage">
  토큰 사용량 감소
</h2>

토큰 비용은 컨텍스트 크기에 따라 확장됩니다. Claude가 처리하는 컨텍스트가 많을수록 더 많은 토큰을 사용합니다. Claude Code는 [prompt caching](/docs/ko/prompt-caching)(시스템 프롬프트와 같은 반복되는 콘텐츠의 비용을 줄임)과 auto-compaction(컨텍스트 한도에 접근할 때 대화 기록을 요약함)을 통해 비용을 자동으로 최적화합니다.

다음 전략은 컨텍스트를 작게 유지하고 메시지당 비용을 줄이는 데 도움이 됩니다.

<h3 id="manage-context-proactively">
  컨텍스트를 사전에 관리하기
</h3>

`/usage`를 사용하여 현재 토큰 사용량을 확인하거나, [상태 줄을 구성](/docs/ko/statusline#context-window-usage)하여 지속적으로 표시하십시오.

* **작업 간 지우기**: 관련 없는 작업으로 전환할 때 `/clear`를 사용하여 새로 시작하십시오. 오래된 컨텍스트는 이후의 모든 메시지에서 토큰을 낭비합니다. 지우기 전에 `/rename`을 사용하여 나중에 세션을 쉽게 찾을 수 있도록 한 다음, `/resume`을 사용하여 돌아가십시오.
* **사용자 정의 compaction 지침 추가**: `/compact Focus on code samples and API usage`는 Claude에게 요약 중에 보존할 내용을 알려줍니다.

프로젝트의 루트에 있는 CLAUDE.md 파일에서 compaction 동작을 사용자 정의할 수도 있습니다:

```markdown theme={null}
# Compact instructions

When you are using compact, please focus on test output and code changes
```

<h3 id="choose-the-right-model">
  올바른 모델 선택
</h3>

Sonnet은 대부분의 코딩 작업을 잘 처리하며 Opus보다 비용이 적습니다. 복잡한 아키텍처 결정이나 다단계 추론을 위해 Opus를 예약하십시오. `/model`을 사용하여 세션 중간에 모델을 전환하거나, `/config`에서 기본값을 설정하십시오. 간단한 subagent 작업의 경우, [subagent 구성](/docs/ko/sub-agents#choose-a-model)에서 `model: haiku`를 지정하십시오.

<h3 id="reduce-mcp-server-overhead">
  MCP server 오버헤드 감소
</h3>

MCP 도구 정의는 [기본적으로 연기됩니다](/docs/ko/mcp#scale-with-mcp-tool-search). 따라서 Claude가 특정 도구를 사용할 때까지 도구 이름만 컨텍스트에 들어갑니다. `/context`를 실행하여 공간을 소비하는 것을 확인하십시오.

* **사용 가능한 경우 CLI 도구 선호**: `gh`, `aws`, `gcloud`, `sentry-cli`와 같은 도구는 도구별 목록을 추가하지 않기 때문에 MCP server보다 컨텍스트 효율적입니다. Claude는 CLI 명령을 직접 실행할 수 있습니다.
* **사용하지 않는 server 비활성화**: `/mcp`를 실행하여 구성된 server를 확인하고 적극적으로 사용하지 않는 것을 비활성화하십시오.

<h3 id="install-code-intelligence-plugins-for-typed-languages">
  타입 언어를 위한 코드 인텔리전스 플러그인 설치
</h3>

[코드 인텔리전스 플러그인](/docs/ko/discover-plugins#code-intelligence)은 Claude에게 텍스트 기반 검색 대신 정확한 기호 탐색을 제공하여 낯선 코드를 탐색할 때 불필요한 파일 읽기를 줄입니다. 단일 "정의로 이동" 호출은 grep 다음에 여러 후보 파일을 읽는 것을 대체합니다. 설치된 언어 서버는 편집 후 자동으로 타입 오류를 보고하므로 Claude는 컴파일러를 실행하지 않고도 실수를 포착합니다.

<h3 id="offload-processing-to-hooks-and-skills">
  hooks 및 skills로 처리 오프로드
</h3>

사용자 정의 [hooks](/docs/ko/hooks)는 Claude가 보기 전에 데이터를 전처리할 수 있습니다. Claude가 10,000줄 로그 파일을 읽어 오류를 찾는 대신, hook은 `ERROR`를 grep하고 일치하는 줄만 반환하여 컨텍스트를 수만 개의 토큰에서 수백 개로 줄일 수 있습니다.

[skill](/docs/ko/skills)은 Claude에게 도메인 지식을 제공하여 탐색할 필요가 없도록 할 수 있습니다. 예를 들어, "codebase-overview" skill은 프로젝트의 아키텍처, 주요 디렉토리 및 명명 규칙을 설명할 수 있습니다. Claude가 skill을 호출하면, 구조를 이해하기 위해 여러 파일을 읽는 데 토큰을 소비하는 대신 즉시 이 컨텍스트를 얻습니다.

예를 들어, 이 PreToolUse hook은 테스트 출력을 필터링하여 실패만 표시합니다:

<Tabs>
  <Tab title="settings.json">
    이를 [settings.json](/docs/ko/settings#settings-files)에 추가하여 모든 Bash 명령 전에 hook을 실행하십시오:

    ```json theme={null}
    {
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "~/.claude/hooks/filter-test-output.sh"
              }
            ]
          }
        ]
      }
    }
    ```
  </Tab>

  <Tab title="filter-test-output.sh">
    hook은 이 스크립트를 호출합니다. `mkdir -p ~/.claude/hooks`로 폴더를 만들고, 아래 스크립트를 `~/.claude/hooks/filter-test-output.sh`로 저장한 다음, `chmod +x ~/.claude/hooks/filter-test-output.sh`로 실행 가능하게 만드십시오. 명령이 테스트 러너인지 확인하고 실패만 표시하도록 수정합니다:

    ```bash theme={null}
    #!/bin/bash
    input=$(cat)
    cmd=$(echo "$input" | jq -r '.tool_input.command')

    # If running tests, filter to show only failures
    if [[ "$cmd" =~ ^(npm test|pytest|go test) ]]; then
      filtered_cmd="$cmd 2>&1 | grep -A 5 -E '(FAIL|ERROR|error:)' | head -100"
      echo "{\"hookSpecificOutput\":{\"hookEventName\":\"PreToolUse\",\"permissionDecision\":\"allow\",\"updatedInput\":{\"command\":\"$filtered_cmd\"}}}"
    else
      echo "{}"
    fi
    ```
  </Tab>
</Tabs>

<h3 id="move-instructions-from-claude-md-to-skills">
  CLAUDE.md에서 skills로 지침 이동
</h3>

[CLAUDE.md](/docs/ko/memory) 파일은 세션 시작 시 컨텍스트에 로드됩니다. PR 검토 또는 데이터베이스 마이그레이션과 같은 특정 워크플로우에 대한 자세한 지침이 포함되어 있으면, 관련 없는 작업을 수행할 때도 해당 토큰이 존재합니다. [Skills](/docs/ko/skills)는 호출될 때만 필요에 따라 로드되므로, 특화된 지침을 skills로 이동하면 기본 컨텍스트를 더 작게 유지합니다. CLAUDE.md를 필수 항목만 포함하여 약 200줄 이하로 유지하십시오.

<h3 id="adjust-extended-thinking">
  확장 사고 조정
</h3>

확장 사고는 기본적으로 활성화되어 있습니다. 복잡한 계획 및 추론 작업의 성능을 크게 향상시키기 때문입니다. 사고 토큰은 출력 토큰으로 청구되며, 기본 예산은 모델에 따라 수만 개의 토큰이 될 수 있습니다. 깊은 추론이 필요하지 않은 더 간단한 작업의 경우, `/effort`를 사용하거나 `/model`에서 [노력 수준](/docs/ko/model-config#adjust-effort-level)을 낮추거나, `/config`에서 사고를 비활성화하거나, [고정 사고 예산](/docs/ko/model-config#adaptive-reasoning-and-fixed-thinking-budgets)이 있는 모델에서 `MAX_THINKING_TOKENS` [환경 변수](/docs/ko/env-vars)를 설정하여 예산을 낮춤으로써(예: `MAX_THINKING_TOKENS=8000`) 비용을 줄일 수 있습니다. 적응형 추론 모델은 0이 아닌 예산을 무시하므로 대신 노력 수준을 사용하십시오. Fable 5에서는 사고 비활성화를 사용할 수 없으며, 항상 확장 사고를 사용합니다.

<h3 id="delegate-verbose-operations-to-subagents">
  자세한 작업을 subagents에 위임
</h3>

테스트 실행, 문서 가져오기 또는 로그 파일 처리는 상당한 컨텍스트를 소비할 수 있습니다. 이를 [subagents](/docs/ko/sub-agents#isolate-high-volume-operations)에 위임하여 자세한 출력이 subagent의 컨텍스트에 유지되는 동안 요약만 주 대화로 반환되도록 하십시오.

<h3 id="manage-agent-team-costs">
  에이전트 팀 비용 관리
</h3>

에이전트 팀은 팀원이 plan mode에서 실행될 때 표준 세션보다 약 7배 더 많은 토큰을 사용합니다. 각 팀원은 자체 컨텍스트 윈도우를 유지하고 별도의 Claude 인스턴스로 실행되기 때문입니다. 팀 작업을 작고 자체 포함되도록 유지하여 팀원당 토큰 사용량을 제한하십시오. 자세한 내용은 [에이전트 팀](/docs/ko/agent-teams)을 참조하십시오.

<h3 id="write-specific-prompts">
  구체적인 프롬프트 작성
</h3>

"이 코드베이스 개선"과 같은 모호한 요청은 광범위한 스캔을 트리거합니다. "auth.ts의 로그인 함수에 입력 검증 추가"와 같은 구체적인 요청은 Claude가 최소한의 파일 읽기로 효율적으로 작업하도록 합니다.

<h3 id="work-efficiently-on-complex-tasks">
  복잡한 작업을 효율적으로 수행
</h3>

더 길거나 복잡한 작업의 경우, 이러한 습관은 잘못된 경로로 인한 낭비된 토큰을 피하는 데 도움이 됩니다:

* **복잡한 작업에 plan mode 사용**: Shift+Tab을 눌러 구현 전에 [plan mode](/docs/ko/permission-modes#analyze-before-you-edit-with-plan-mode)에 들어가십시오. Claude는 코드베이스를 탐색하고 승인을 위한 접근 방식을 제안하여, 초기 방향이 잘못되었을 때 비용이 많이 드는 재작업을 방지합니다.
* **조기에 방향 수정**: Claude가 잘못된 방향으로 가기 시작하면, Escape를 눌러 즉시 중지하십시오. `/rewind`를 사용하거나 Escape를 두 번 눌러 대화 및 코드를 이전 checkpoint로 복원하십시오.
* **검증 대상 제공**: 테스트 케이스를 포함하고, 스크린샷을 붙여넣거나, 프롬프트에서 예상 출력을 정의하십시오. Claude가 자신의 작업을 검증할 수 있으면, 수정을 요청해야 하기 전에 문제를 포착합니다.
* **증분적으로 테스트**: 한 파일을 작성하고, 테스트한 다음, 계속하십시오. 이는 문제가 저렴하게 수정될 수 있을 때 조기에 포착합니다.

<h2 id="background-token-usage">
  백그라운드 토큰 사용량
</h2>

Claude Code는 유휴 상태에서도 일부 백그라운드 기능에 토큰을 사용합니다:

* **대화 요약**: `claude --resume` 기능을 위해 이전 대화를 요약하는 백그라운드 작업
* **명령 처리**: `/usage`와 같은 일부 명령은 상태를 확인하기 위해 요청을 생성할 수 있습니다

이러한 백그라운드 프로세스는 활성 상호작용 없이도 세션당 적은 양의 토큰(일반적으로 \$0.04 미만)을 소비합니다.

<h2 id="understanding-changes-in-claude-code-behavior">
  Claude Code 동작 변경 이해
</h2>

Claude Code는 비용 보고를 포함한 기능 작동 방식을 변경할 수 있는 정기적인 업데이트를 받습니다. `claude --version`을 실행하여 현재 버전을 확인하십시오. 특정 청구 질문의 경우, [Console 계정](https://platform.claude.com/login)을 통해 Anthropic 지원에 문의하십시오.
