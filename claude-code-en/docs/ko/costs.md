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

`/usage` 상단의 Session 블록은 현재 세션에 대한 자세한 토큰 사용량 통계를 표시합니다. Claude Code는 [`modelPricing`](/docs/ko/settings-reference#modelpricing) 테이블이 적용되지 않는 한 토큰 수에서 정가로 달러 수치를 로컬로 계산합니다. 관리자는 조직의 관리 설정에서 이를 설정하여 수치가 계약된 요금을 사용하도록 하며, 테이블이 적용되는 동안 `Total cost` 줄에는 `at your organization's configured rates` 메모가 표시됩니다. 수치는 추정치이므로 권위 있는 청구를 위해 [Claude Console](https://platform.claude.com/usage)의 사용량 페이지를 참조하십시오.

```text theme={null}
Total cost:            $0.55
Total duration (API):  6m 20s
Total duration (wall): 6h 33m 10s
Total code changes:    0 lines added, 0 lines removed
Usage by model:
   claude-sonnet-4-6:  1.2k input, 5.3k output, 940.0k cache read, 50.0k cache write ($0.55)
```

이러한 합계는 `/clear`가 새 세션을 시작할 때 재설정되므로 다음 세션의 총 비용은 \$0에서 시작합니다. v2.1.211 이전에는 `/clear`를 통해 Claude Code 프로세스의 수명 동안 계속 누적되었습니다.

1.1× [데이터 거주지 요금](https://platform.claude.com/docs/en/about-claude/pricing#data-residency-pricing)으로 청구되는 Claude API의 응답의 경우, Claude Code는 해당 응답의 토큰 정가에 1.1을 곱하여 세션 비용 수치에 적용합니다. Claude Code는 [상태 줄의 비용 필드](/docs/ko/statusline#cost-and-duration-tracking)에서 동일한 합계를 보고하고 [`--max-budget-usd`](/docs/ko/cli-reference#cli-flags)와 비교합니다. v2.1.239 이전에는 Claude Code가 해당 응답에 1.1×을 적용하지 않았으므로 세션 비용 수치가 청구서보다 낮았습니다.

<h4 id="prompt-cache-statistics">
  프롬프트 캐시 통계
</h4>

주 대화의 첫 번째 API 응답 후, Claude Code는 Session 블록에 `Prompt cache (main)` 줄을 추가하여 세션의 [프롬프트 캐시](/docs/ko/prompt-caching) 사용을 요약합니다: 요청 수, 캐시에서 제공된 입력 토큰의 공유, 캐시 미스 및 캐시가 현재 따뜻한지 여부입니다. Claude Code v2.1.251 이상이 필요합니다.

```text theme={null}
Prompt cache (main):   14 requests · 91% of input tokens from cache · 2 misses (last 6m 10s ago, 310.2k tokens re-cached) · 1 expected rebuild (compaction or tool-result clearing) · warm (1h TTL, last activity 40s ago)
```

줄의 미스, 예상 재구축 및 따뜻함 또는 차가움 부분은 다음을 의미합니다:

* **Misses**: 캐시가 이미 보유한 콘텐츠를 다시 처리한 요청이며, 마지막 미스의 시간과 해당 요청이 캐시에 다시 작성한 토큰 수입니다. Claude Code는 요청이 캐시에서 읽을 수 있었던 것의 5% 이상 및 최소 2,000개 토큰을 다시 처리할 때 요청을 미스로 계산합니다. [캐시를 무효화하는 작업](/docs/ko/prompt-caching#actions-that-invalidate-the-cache)은 일반적인 원인을 나열합니다. Claude Code가 마지막 미스의 가능한 원인을 식별할 수 있을 때, 줄은 이를 이름으로 지정합니다. 예를 들어 `likely cause: tool definitions changed`입니다. 가능한 원인 텍스트는 Claude Code v2.1.260 이상이 필요합니다.
* **Expected rebuilds**: Claude Code가 [압축](/docs/ko/prompt-caching#compacting-the-conversation)을 통해 또는 컨텍스트에서 이전 도구 결과를 지워서 대화를 다시 작성했을 때, 동일한 종류의 미스를 예상 재구축으로 계산합니다. 이 부분은 최소 하나의 예상 재구축이 발생한 후에만 나타납니다.
* **Warm or cold**: 캐시된 접두사가 여전히 [캐시 수명](/docs/ko/prompt-caching#cache-lifetime) 내에 있는지 여부이며, 적용 중인 TTL입니다. 캐시가 차가울 때, 줄은 세션이 유휴 상태인 기간을 표시합니다. 응답이 캐시 토큰을 보고하지 않았을 때, 줄은 대신 `no prompt caching reported by the API`로 끝납니다.

수는 API의 응답의 캐시 토큰 필드에서 나오므로, 줄은 모든 공급자 및 게이트웨이에서 작동합니다. 주 대화만 다루며, 서브에이전트는 다루지 않습니다. `/clear`는 Session 블록의 나머지와 함께 이를 재설정합니다.

상태 줄 스크립트는 [`prompt_cache` 객체](/docs/ko/statusline#prompt-cache-fields)에서 동일한 수를 읽을 수 있습니다.

<h4 id="plan-usage-breakdown">
  요금제 사용량 분석
</h4>

Pro, Max, Team 또는 Enterprise 요금제에서 `/usage`는 요금제 한도에 포함되는 항목의 분석도 표시합니다:

* **Attribution**: skills, subagents, plugins 및 개별 MCP 서버에 귀속된 최근 사용량이며, 각각은 전체의 백분율로 표시됩니다. MCP 서버의 공유는 해당 도구 결과 중 하나를 사용한 요청만 계산합니다. v2.1.222 이전에는 MCP 서버에 대한 한 번의 호출 후 Claude Code는 모든 후속 요청을 해당 서버에 귀속시켜 공유를 과대 계산했습니다.
* **Behavior flags**: 최근 사용량의 10% 이상을 차지할 때 플래그되는 긴 컨텍스트 또는 캐시 미스와 같은 동작입니다.
* **Loops**: 최근에 실행된 가장 무거운 [`/loop` 또는 기타 예약된 작업](/docs/ko/scheduled-tasks)의 각 행이며, 총 토큰으로 정렬되고 나머지 수를 포함합니다. Claude Code는 각 작업이 얼마나 자주 실행되는지, 실행된 횟수, 총 토큰 및 실행당 토큰, 마지막 실행 시간을 보고합니다. Claude Code는 작업의 프롬프트로 행을 키하므로 중지하고 다시 생성한 루프는 한 행으로 유지됩니다. Claude Code v2.1.242 이상이 필요합니다.

`d` 또는 `w`를 눌러 지난 24시간과 지난 7일 사이를 전환합니다. 수치는 근사치이며 이 기기의 로컬 세션 기록에서 계산되므로 다른 기기 또는 claude.ai의 사용량은 포함되지 않습니다.

[VS Code 확장](/docs/ko/vs-code#check-account-and-usage)에서 귀속 공유 및 동작 플래그는 Loops 행 없이 Day 및 Week 토글이 있는 Account & usage 대화 상자에 나타납니다.

<h4 id="check-your-usage-credits-spend">
  사용량 크레딧 지출 확인
</h4>

`/usage`는 또한 [사용량 크레딧](#add-usage-credits-to-your-subscription)이 켜져 있는 동안 사용량 크레딧 행을 표시합니다. 행이 표시하는 내용은 요금제에 따라 다릅니다:

* **Pro and Max**: 월간 지출 한도를 설정한 경우 월간 지출 한도에 대해 측정된 현재 월의 지출입니다. 한도를 설정하지 않았을 때, 행은 `Unlimited`를 표시하고 지출 수치는 표시하지 않습니다.
* **Team and Enterprise**: 귀하에게 적용되는 [조직이 설정한](#claude-for-teams-and-enterprise) 한도에 대해 측정된 현재 월의 자신의 지출입니다. 전체 조직을 다루는 한도는 행에 나타나지 않습니다. 자신의 한도가 없을 때, 행은 한도 없이 지출을 표시합니다. 사용량 크레딧이 귀하에게 꺼져 있는 동안, `/usage`는 사용량 크레딧 행을 표시하지 않습니다.

지출 한도가 있을 때, 행은 사용량 크레딧이 켜지자마자 나타나고 처음 사용량 크레딧을 지출할 때까지 0%를 표시합니다. v2.1.236 이전에는 `/usage`가 Pro 및 Max 요금제에서만 행을 표시했으며, 지출 한도가 있는 행은 무언가를 지출할 때까지 숨겨져 있었습니다.

<h4 id="when-the-usage-request-fails">
  사용량 요청이 실패할 때
</h4>

요금제 한도에 대한 요청이 실패할 때(대부분 사용량 엔드포인트가 속도 제한되기 때문), `/usage`는 이 기기에서 지난 60분 이내에 로드한 마지막 사용량 막대를 표시하며, 해당 데이터를 얼마나 오래 전에 가져왔는지 나타내는 `Showing last-known usage` 메모가 함께 표시됩니다. `r`을 눌러 다시 시도하면, 성공적인 재시도는 마지막으로 알려진 막대를 새로운 데이터로 바꿉니다. 지난 60분 이내의 스냅샷이 없으면 `/usage`는 사용량 엔드포인트가 속도 제한되었다고 보고하고 동일한 재시도 단축키를 제공합니다. v2.1.208 이전에는 아직 사용량을 로드하지 않은 세션에서 속도 제한된 요청이 항상 막대 없이 오류를 표시했습니다.

<h3 id="analyze-your-usage-patterns">
  사용량 패턴 분석
</h3>

[`/insights`](/docs/ko/commands#all-commands)를 실행하여 사용한 토큰 수가 아닌 작업 방식에 대한 보고서를 얻습니다. 이 기기의 최근 세션을 분석하고 작업 내용, 오해된 요청 또는 버그가 있는 코드와 같은 마찰 지점, Claude Code를 더 효과적으로 사용하기 위한 제안을 다루는 HTML 보고서를 작성합니다. 단일 실행은 이전에 보지 못한 최대 200개의 세션을 분석하고 매우 짧은 세션은 건너뜁니다. 세션이 제외될 때, 보고서 헤더는 분석된 수를 괄호 안의 총계와 함께 표시합니다. 예를 들어 `200 sessions (412 total)`입니다.

Claude Code는 최신 보고서를 `~/.claude/usage-data/report.html`에 작성하고 각 실행의 타임스탬프가 지정된 사본을 동일한 디렉토리에 저장하므로 이전 보고서는 덮어쓰지 않습니다. Claude Code는 나머지 세션 데이터와 동일한 일정에 따라 보고서를 삭제합니다: 시작 시 [`cleanupPeriodDays`](/docs/ko/claude-directory#cleaned-up-automatically)보다 오래된 파일을 제거하며, 기본값은 30일입니다.

모든 요금제 및 모든 공급자에서 `/insights`를 실행할 수 있습니다. 분석은 일반 세션과 동일한 공급자 및 계정을 통해 실행되며, 토큰은 요금제 또는 API 사용량에 포함됩니다. 다른 기기 및 claude.ai의 세션은 포함되지 않습니다.

<h3 id="add-usage-credits-to-your-subscription">
  구독에 사용량 크레딧 추가
</h3>

[사용량 크레딧](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)을 사용하면 요금제의 사용량 한도를 초과하여 계속 작업할 수 있습니다. 이를 관리하려면 `/login`을 통해 claude.ai 구독으로 로그인한 후 `/usage-credits`를 실행하십시오. 이 명령은 API 키 인증에서는 사용할 수 없습니다. 셀프 서비스 Enterprise 조직, Enterprise 평가판 및 AWS Marketplace를 통해 청구되는 Enterprise 조직에서 이 명령은 Claude Code v2.1.248 이상이 필요합니다. 이전 버전은 [`Unknown command: /usage-credits`](/docs/ko/errors#unknown-command)로 거부합니다. 열리는 내용은 역할에 따라 다릅니다:

| 역할                                   | `/usage-credits`가 수행하는 작업                                                                                                                                            |
| :----------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Pro 또는 Max 구독자                       | 브라우저에서 claude.ai의 [**Settings > Usage**](https://claude.ai/settings/usage)를 엽니다. **Usage credits** 섹션에서 사용량 크레딧을 켜거나 끌 수 있으며 크레딧 잔액, 이번 달의 지출 및 월간 지출 한도를 확인할 수 있습니다 |
| 청구 액세스 권한이 있는 Team 또는 Enterprise 구성원 | 브라우저에서 조직의 사용량 설정인 [**Admin settings > Usage**](https://claude.ai/admin-settings/usage)를 엽니다                                                                         |
| 청구 액세스 권한이 없는 Team 또는 Enterprise 구성원 | 확인을 요청한 후 조직의 관리자에게 요청을 보냅니다. v2.1.211 이전에는 Claude Code가 확인 단계 없이 요청을 보냈습니다                                                                                          |

청구 액세스 권한이 없는 Team 및 Enterprise 구성원의 경우, 확인은 대화형 세션에서만 나타납니다: `-p` 플래그가 있는 비대화형 모드 및 [Remote Control](/docs/ko/remote-control)에서 명령은 요청을 보내지 않으며 대화형 세션에서 실행하도록 지시합니다.

이전 요청이 관리자를 기다리는 동안 `/usage-credits`를 다시 실행하면, Claude Code는 중복을 보내지 않고 요청이 이미 전송되었다고 알려줍니다. 관리자가 요청을 거부한 후 명령을 다시 실행하면 새 요청을 보냅니다. v2.1.222 이전에는 거부된 요청도 새 요청을 차단했습니다.

Pro 및 Max 요금제에서 사용량 크레딧이 여전히 사용 가능한 상태에서 지출 한도에 도달하면, Claude Code는 CLI를 떠나지 않고 한도를 높이거나 제거하도록 메시지를 표시합니다. 서버가 변경을 거부하면 [Could not update your spend limit](/docs/ko/errors#could-not-update-your-spend-limit)를 참조하십시오.

<h2 id="manage-costs-for-your-organization">
  조직의 비용 관리
</h2>

조직이 Claude Code에 액세스하는 방식에 따라 사용할 수 있는 제어 기능이 달라집니다: Claude for Teams 또는 Enterprise 플랜, Claude Console 또는 클라우드 제공자입니다. Teams 및 Enterprise 플랜에서는 사용량이 각 멤버의 시트 할당량에서 차감됩니다. Console 및 클라우드 제공자에서는 사용량이 토큰당 조직에 청구됩니다. 조직이 여러 로그인 방법을 혼합하는 경우, 각 개발자는 인증한 방법에 따라 측정됩니다.

표는 각 설정을 지출 확인 위치, 지출 상한선 위치 및 사용자별 수치 추출 방법에 매핑합니다. 개별 Pro 또는 Max 플랜에서는 관리할 조직이 없으므로, [빠른 모드](/docs/ko/fast-mode#see-where-fast-mode-spend-appears)를 포함한 자신의 사용량-크레딧 지출을 추적하고, [구독에 사용 크레딧 추가](#add-usage-credits-to-your-subscription)를 참조하십시오.

| 설정                                                                                    | 지출 확인                                                                                                               | 지출 상한선        | 사용자별 보고                                                                                                                                                                                                           |
| :------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------ | :------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Claude for Teams 또는 Enterprise](#claude-for-teams-and-enterprise)                    | [조직 분석의 지출 보고서](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans) | 관리자 설정의 지출 한도 | [지출 보고서 CSV](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans); Enterprise의 [Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics) |
| [Claude Console (API)](#claude-console)                                               | [Console 사용량 페이지](https://platform.claude.com/usage)                                                                | 워크스페이스 지출 한도  | [Console 대시보드](https://platform.claude.com/claude-code), [Claude Code Analytics API](https://platform.claude.com/docs/en/build-with-claude/claude-code-analytics-api)                                             |
| [Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry](#cloud-providers) | 클라우드 청구 콘솔                                                                                                          | 클라우드의 예산 제어   | [OpenTelemetry](/docs/ko/monitoring-usage) 또는 [LLM gateway](/docs/ko/llm-gateway)                                                                                                                                           |

[OpenTelemetry 내보내기](/docs/ko/monitoring-usage)는 모든 설정에서 작동하며 사용자별 토큰 및 비용 메트릭을 거의 실시간으로 자체 관찰성 스택으로 스트리밍하는 유일한 옵션입니다.

<h3 id="report-spend-at-your-contracted-rates">
  계약된 요금으로 지출 보고
</h3>

기본적으로 Claude Code는 표시하는 모든 비용 수치를 정가로 계산하므로, 조직이 계약된 요금을 지불하는 경우 `/usage`, 상태 라인 및 OpenTelemetry의 수치가 청구서와 일치하지 않습니다. 일치시키려면 [`modelPricing`](/docs/ko/settings-reference#modelpricing) 관리 설정을 요금으로 설정하십시오. 이 설정은 Claude Code가 보고하는 내용을 변경하며, Anthropic이 청구하는 내용은 변경하지 않습니다. Claude Code v2.1.242 이상이 필요합니다.

<Steps>
  <Step title="계약에서 요금 가져오기">
    계약의 백만 토큰당 요금을 입력하십시오. Claude Code는 Claude Console에서 요금을 가져오지 않으므로 계약이 변경될 때 설정을 업데이트하십시오.
  </Step>

  <Step title="설정 작성">
    정가에서 고정 할인율에 대해 `multiplier`를 1 미만으로 설정하거나 1 이상으로 설정하여 마크업을 적용하고, `overrides` 아래에 각 모델의 4가지 토큰당 요금을 나열하거나, 둘 다 수행하십시오. 마크업에는 Claude Code v2.1.271 이상이 필요합니다. [`modelPricing` 항목](/docs/ko/settings-reference#modelpricing)에는 형태와 붙여넣기 준비가 된 예제가 있습니다.
  </Step>

  <Step title="관리 설정을 통해 배포">
    [관리 설정](/docs/ko/managed-settings)으로 전달하십시오: 서버 관리 설정, MDM 정책, `managed-settings.json` 또는 [정책 도우미](/docs/ko/managed-settings#compute-the-policy-with-a-helper-program). Claude Code는 사용자, 프로젝트 및 로컬 설정과 `--settings`의 키를 무시합니다.
  </Step>
</Steps>

요금이 적용되는지 확인하려면 [관리 설정을 받은](/docs/ko/managed-settings#read-the-source-in-%2Fstatus) 세션에서 `/usage`를 실행하십시오: 세션 블록의 `총 비용` 라인에는 `조직의 구성된 요금으로` 주석이 있습니다. 수치는 여전히 추정치이며 청구서가 아닙니다. `/model` 선택기의 백만 토큰당 가격은 정가로 유지됩니다.

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

  사용자 정의 속도 제한이 있는 조직의 경우, 이 워크스페이스의 Claude Code 트래픽은 조직의 전체 API 속도 제한에 포함됩니다. Claude Console의 이 워크스페이스의 한도 페이지에서 [워크스페이스 속도 제한](https://platform.claude.com/docs/en/api/rate-limits#setting-lower-limits-for-workspaces)을 설정하여 Claude Code의 할당량을 제한하고 다른 프로덕션 워크로드를 보호할 수 있습니다.
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

개발자는 일반적으로 한도 질문을 관리자에게 가져오므로, 어떤 상한선에 도달했는지 아는 것이 도움이 됩니다. 이러한 상황은 다른 의미를 가집니다:

* **"세션 한도에 도달했습니다" 또는 "주간 한도에 도달했습니다"**: 구독 플랜의 시트 기반 사용 윈도우이며, 모든 모델에서 공유되므로 개발자는 `/model`로 모델을 전환하여 액세스를 복원할 수 없습니다. 메시지는 윈도우가 재설정될 때를 표시합니다. 모델별 "Opus 한도에 도달했습니다" 또는 "Sonnet 한도에 도달했습니다" 메시지 후에는 `/model`로 해당 제품군 외의 모델로 전환하면 개발자가 계속 작업할 수 있습니다. [사용 한도 오류](/docs/ko/errors#youve-hit-your-session-limit)를 참조하십시오. 개발자가 그 동안 할 수 있는 작업:
  * [사용 크레딧](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)이 활성화된 경우 `/usage-credits`를 실행하여 할당량을 초과하는 사용을 요청하십시오.
  * Claude Code v2.1.234 이상에서 [재설정 후 중단된 작업을 자동으로 계속 대기](/docs/ko/interactive-mode#wait-for-a-usage-limit-to-reset); 해당 섹션에는 Claude Code가 자동으로 대기를 시작하는 시기와 개발자가 `/rate-limit-options`에서 선택하는 시기가 나열되어 있습니다. 플릿에 대해 Claude Code가 자동으로 해당 대기를 시작하는지 제어하려면 [관리 설정](/docs/ko/settings#settings-precedence)에서 [`autoContinueAtUsageLimit`](/docs/ko/settings-reference#autocontinueatusagelimit)을 설정하십시오.
* **"개별 지출 한도에 도달했습니다", "조직의 월간 지출 한도" 또는 "팀의 공유 예산"**: 개발자의 요청이 사용 크레딧으로 청구되며, 이러한 크레딧이 설정한 지출 한도에 도달했습니다. 개발자가 계속하도록 허용하려면 [**관리 설정 > 사용**](https://claude.ai/admin-settings/usage)으로 이동하여 메시지가 명시한 한도를 높이십시오. 메시지가 플랜 재설정 시간도 명시하는 경우, 개발자는 대신 그때까지 기다릴 수 있습니다. 각 변형에 대해 [오류 참고](/docs/ko/errors#youve-hit-your-monthly-spend-limit)를 참조하십시오.
* **[Claude apps gateway](/docs/ko/claude-apps-gateway)의 지출 한도 메시지**: 개발자가 자체 호스팅된 게이트웨이에서 설정한 지출 상한선을 초과했으며, 게이트웨이는 기간이 재설정되거나 상한선을 높일 때까지 요청을 차단합니다. [게이트웨이 지출 한도](/docs/ko/claude-apps-gateway-spend-limits)에서 상한선, 재설정 일정 및 개발자가 보는 메시지를 참조하십시오.
* **컨텍스트 또는 자동 압축 경고**: 사용 한도가 아닙니다. 대화가 세션의 [자동 압축 윈도우](/docs/ko/model-config#set-the-auto-compact-window)에 가까워졌으며, Claude Code가 공간을 확보하기 위해 이전 기록을 요약하는 임계값입니다. 개발자를 [토큰 사용량 감소](#reduce-token-usage)로 안내하십시오.
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
* **사용자 정의 compaction 지침 추가**: `/compact Focus on code samples and API usage`는 Claude에게 요약 중에 보존할 내용을 알려줍니다. 새로운 세션에서는 아직 대화 기록이 없기 때문에 `/compact`를 실행하면 `Not enough messages to compact.`를 출력합니다.

프로젝트의 루트에 있는 CLAUDE.md 파일에서 compaction 동작을 사용자 정의할 수도 있습니다:

```markdown theme={null}
# Compact instructions

When you are using compact, please focus on test output and code changes
```

<h3 id="choose-the-right-model">
  올바른 모델 선택
</h3>

Sonnet은 대부분의 코딩 작업을 잘 처리하며 Opus보다 비용이 적습니다. 복잡한 아키텍처 결정이나 다단계 추론을 위해 Opus를 예약하십시오. `/model`을 사용하여 세션 중간에 모델을 전환하거나, `/config`에서 기본값을 설정하십시오. Opus로 전환하면 [세션의 모델을 상속하는 subagents](/docs/ko/model-config#setting-your-model)에도 적용됩니다. 간단한 subagent 작업의 경우, [subagent 구성](/docs/ko/sub-agents#choose-a-model)에서 `model: haiku`를 지정하십시오.

<h3 id="reduce-mcp-server-overhead">
  MCP server 오버헤드 감소
</h3>

MCP 도구 정의는 [기본적으로 연기됩니다](/docs/ko/mcp#scale-with-mcp-tool-search). 따라서 Claude가 특정 도구를 사용할 때까지 도구 이름과 server 지침만 컨텍스트에 들어갑니다. `/context`를 실행하여 공간을 소비하는 것을 확인하십시오.

* **사용 가능한 경우 CLI 도구 선호**: `gh`, `aws`, `gcloud`, `sentry-cli`와 같은 도구는 도구별 목록을 추가하지 않기 때문에 MCP server보다 컨텍스트 효율적입니다. Claude는 CLI 명령을 직접 실행할 수 있습니다.
* **사용하지 않는 server 비활성화**: `/mcp`를 실행하여 구성된 server를 확인하고 적극적으로 사용하지 않는 것을 비활성화하십시오.

<h3 id="install-code-intelligence-plugins-for-typed-languages">
  타입 언어를 위한 코드 인텔리전스 플러그인 설치
</h3>

[코드 인텔리전스 플러그인](/docs/ko/plugins/code-intelligence)은 Claude에게 텍스트 기반 검색 대신 정확한 기호 탐색을 제공하여 낯선 코드를 탐색할 때 불필요한 파일 읽기를 줄입니다. 단일 "정의로 이동" 호출은 grep 다음에 여러 후보 파일을 읽는 것을 대체합니다. 설치된 언어 서버는 편집 후 자동으로 타입 오류를 보고하므로 Claude는 컴파일러를 실행하지 않고도 실수를 포착합니다.

<h3 id="offload-processing-to-hooks-and-skills">
  hooks 및 skills로 처리 오프로드
</h3>

사용자 정의 [hooks](/docs/ko/hooks)는 Claude가 보기 전에 데이터를 전처리할 수 있습니다. Claude가 10,000줄 로그 파일을 읽어 오류를 찾는 대신, hook은 `ERROR`를 grep하고 일치하는 줄만 반환하여 컨텍스트를 수만 개의 토큰에서 수백 개로 줄일 수 있습니다.

[skill](/docs/ko/skills)은 Claude에게 도메인 지식을 제공하여 탐색할 필요가 없도록 할 수 있습니다. 예를 들어, "codebase-overview" skill은 프로젝트의 아키텍처, 주요 디렉토리 및 명명 규칙을 설명할 수 있습니다. Claude가 skill을 호출하면, 구조를 이해하기 위해 여러 파일을 읽는 데 토큰을 소비하는 대신 즉시 이 컨텍스트를 얻습니다.

예를 들어, 이 PreToolUse hook은 테스트 출력을 필터링하여 실패만 표시합니다:

<Tabs>
  <Tab title="settings.json">
    이를 [settings.json](/docs/ko/settings#where-settings-live)에 추가하여 모든 Bash 명령 전에 hook을 실행하십시오:

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
      echo "$input" | jq --arg filtered "$filtered_cmd" \
        '{hookSpecificOutput: {hookEventName: "PreToolUse", permissionDecision: "allow", updatedInput: (.tool_input + {command: $filtered})}}'
    else
      echo "{}"
    fi
    ```
  </Tab>
</Tabs>

설정을 확인하려면 `/hooks`를 실행하고 hook이 PreToolUse 아래에 나타나는지 확인하십시오. `claude --debug-file ./claude-debug.txt`로 Claude Code를 시작하고 Claude에게 `npm test`를 실행하도록 요청할 수도 있습니다. hook이 명령을 다시 쓸 때, 해당 로그 파일에는 `command`와 다른 Bash 입력 필드를 나열하는 `modified tool input keys` 줄이 포함됩니다.

<h3 id="move-instructions-from-claude-md-to-skills">
  CLAUDE.md에서 skills로 지침 이동
</h3>

[CLAUDE.md](/docs/ko/memory) 파일은 세션 시작 시 컨텍스트에 로드됩니다. PR 검토 또는 데이터베이스 마이그레이션과 같은 특정 워크플로우에 대한 자세한 지침이 포함되어 있으면, 관련 없는 작업을 수행할 때도 해당 토큰이 존재합니다. [Skills](/docs/ko/skills)는 호출될 때만 필요에 따라 로드되므로, 특화된 지침을 skills로 이동하면 기본 컨텍스트를 더 작게 유지합니다. CLAUDE.md를 필수 항목만 포함하여 약 200줄 이하로 유지하십시오.

<h3 id="adjust-extended-thinking">
  확장 사고 조정
</h3>

확장 사고는 기본적으로 활성화되어 있습니다. 복잡한 계획 및 추론 작업의 성능을 크게 향상시키기 때문입니다. 사고 토큰은 출력 토큰으로 청구되며, 기본 예산은 모델에 따라 수만 개의 토큰이 될 수 있습니다.

더 간단한 작업에서 깊은 추론이 필요하지 않은 경우, `/effort`를 사용하거나 `/model`에서 [노력 수준](/docs/ko/model-config#adjust-effort-level)을 낮추거나, `/config`에서 사고를 비활성화하여 비용을 줄일 수 있습니다. Opus 5.5 또는 Fable 모델에서는 사고를 끌 수 없으며, 항상 확장 사고를 사용합니다.

[고정 사고 예산](/docs/ko/model-config#adaptive-reasoning-and-fixed-thinking-budgets)이 있는 모델에서는 `MAX_THINKING_TOKENS` [환경 변수](/docs/ko/env-vars)를 설정하여 예산을 낮출 수도 있습니다(예: `MAX_THINKING_TOKENS=8000`). 적응형 추론 모델은 0이 아닌 예산을 무시하므로 대신 노력 수준을 사용하십시오.

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

프롬프트 제안이 켜져 있으면, Claude Code는 Claude가 응답한 후 세션이 사용 중인 모델에 짧은 요청을 보내 [다음 프롬프트를 제안합니다](/docs/ko/interactive-mode#prompt-suggestions). 해당 요청은 대화의 프롬프트 캐시를 재사용하므로 대부분 캐시 읽기와 몇 가지 출력 토큰으로 구성됩니다. Claude Code는 [계정이 사용량 한도에 가깝거나 도달했을 때 제안을 건너뜁니다](/docs/ko/interactive-mode#when-claude-code-skips-suggestions). 이러한 요청을 중지하려면 [프롬프트 제안을 끕니다](/docs/ko/interactive-mode#turn-prompt-suggestions-off).

<h2 id="why-usage-climbs-in-a-long-session">
  긴 세션에서 사용량이 증가하는 이유
</h2>

몇 시간 동안 열려 있던 세션은 활동 수준보다 훨씬 더 많은 플랜 한도를 사용할 수 있으며, 일반적으로 다음 중 하나의 이유 때문입니다:

* **긴 컨텍스트**: Claude Code는 모든 요청과 함께 전체 대화를 전송하며, Claude가 도구를 사용할 때마다 해당 도구 결과 배치를 포함하는 또 다른 요청을 전송합니다. [프롬프트 캐싱](/docs/ko/prompt-caching)을 사용하면 Claude Code는 해당 기록을 [캐시된 토큰 요금](https://platform.claude.com/docs/en/about-claude/pricing)으로 다시 읽으므로, 하루 종일 열려 있던 세션의 한 줄 질문도 전체 대화에 대한 사용량을 소비합니다. 컨텍스트를 작게 유지하는 방법은 [컨텍스트 사전에 관리하기](#manage-context-proactively)를 참조하세요
* **캐시 미스**: [캐시 수명](/docs/ko/prompt-caching#cache-lifetime)보다 긴 휴식 후의 첫 번째 메시지는 캐시를 놓치고 전체 컨텍스트를 다시 처리합니다. 수명은 구독 시 1시간이며 [사용 크레딧](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans)을 사용하기 시작하면 5분으로 단축됩니다. API 키 또는 클라우드 제공자의 경우 기본값은 5분입니다. 사용 크레딧을 사용하면서 1시간 수명을 유지하려면 [TTL을 직접 선택](/docs/ko/prompt-caching#choose-the-ttl-yourself)하세요. Pro 및 Max 플랜에서 긴 휴식 후 큰 세션을 재개할 때 Claude Code는 [요약에서 재개할 것을 제안](/docs/ko/sessions#resume-from-a-summary)하므로 이후 요청이 전체 기록을 포함하지 않습니다
* **예약된 작업**: [예약된 작업](/docs/ko/scheduled-tasks)은 세션이 유휴 상태일 때도 해당 간격으로 실행되며, 매번 전체 컨텍스트를 전송합니다
* **세션 간 메시지**: Claude Code는 이 세션이 유휴 상태일 때 [다른 세션의 메시지](/docs/ko/cross-session-messaging)를 새로운 턴으로 전달하며, 매번 전체 컨텍스트를 전송합니다. 인바운드 메시지를 전달하지 않고 보류하려면 [`crossSessionInbound`](/docs/ko/settings-reference#crosssessioninbound)를 `hold`로 설정하세요
* **목표 체크인**: 백그라운드 작업이 활성 [목표](/docs/ko/goal)를 대기 상태로 유지하는 동안 Claude Code는 세션이 유휴 상태일 때도 [해당 작업을 확인하도록 Claude에 요청](/docs/ko/goal#background-work-defers-evaluation)하며, 전체 컨텍스트를 전송하는 새로운 턴을 시작합니다. Claude Code는 프롬프트 사이에 목표당 최대 3개의 유휴 체크인을 시작합니다. v2.1.246 이전에는 유휴 체크인이 무제한이었습니다. 체크인을 끄려면 [`CLAUDE_CODE_GOAL_CHECKIN_MINUTES`](/docs/ko/env-vars)를 `0`으로 설정하세요. 유휴 체크인에는 Claude Code v2.1.236 이상이 필요합니다
* **에이전트 팀원**: 각 활성 [팀원](#agent-team-token-costs)은 종료될 때까지 계속 토큰을 소비합니다
* **압축**: `/compact`는 요약하는 대화를 읽으므로 [큰 컨텍스트 압축](/docs/ko/prompt-caching#compacting-the-conversation)은 그 자체로 큰 요청입니다. 연속성 대신 새로운 시작을 원할 때 `/clear`는 비용이 들지 않습니다

Pro, Max, Team 또는 Enterprise 플랜에서 `/usage` 분석은 긴 컨텍스트 또는 캐시 미스와 같이 최근 사용량의 10% 이상을 차지하는 동작에 플래그를 지정하며, 각각 이를 줄이기 위한 팁이 있습니다.

<h2 id="understanding-changes-in-claude-code-behavior">
  Claude Code 동작 변경 사항 이해하기
</h2>

Claude Code는 비용 보고를 포함한 기능 작동 방식을 변경할 수 있는 정기적인 업데이트를 받습니다. `claude --version`을 실행하여 현재 버전을 확인하십시오.

계정별 청구 관련 질문은 제품 내 메신저를 통해 Anthropic 지원팀에 문의하십시오:

* **구독 플랜**(Pro, Max, Team, Enterprise): [claude.ai](https://claude.ai)에 로그인하여 왼쪽 아래의 이니셜을 클릭한 후 **Get help**를 선택하십시오
* **Console (API) 청구**: [platform.claude.com](https://platform.claude.com)에 로그인하여 이니셜을 클릭한 후 **Get help**를 선택하십시오

각 플랜에서 담당자와 연결할 수 있는 사람을 포함한 전체 절차는 [How to get support](https://support.claude.com/en/articles/9015913-how-to-get-support)를 참조하십시오.
