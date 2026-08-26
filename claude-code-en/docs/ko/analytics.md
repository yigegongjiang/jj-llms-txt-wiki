> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 팀 사용량을 분석으로 추적하기

> Claude Code 사용량 지표를 확인하고, 채택 현황을 추적하며, 분석 대시보드에서 엔지니어링 속도를 측정합니다.

Claude Code는 조직이 개발자 사용 패턴을 이해하고, 기여도 지표를 추적하며, Claude Code가 엔지니어링 속도에 미치는 영향을 측정할 수 있도록 분석 대시보드를 제공합니다. 귀사의 플랜에 맞는 대시보드에 접근하세요:

| 플랜                            | 대시보드 URL                                                                   | 포함 사항                                         | 자세히 알아보기                                           |
| ----------------------------- | -------------------------------------------------------------------------- | --------------------------------------------- | -------------------------------------------------- |
| Claude for Teams / Enterprise | [claude.ai/analytics/claude-code](https://claude.ai/analytics/claude-code) | 사용량 지표, GitHub 통합을 포함한 기여도 지표, 리더보드, 데이터 내보내기 | [세부 정보](#access-analytics-for-team-and-enterprise) |
| API (Claude Console)          | [platform.claude.com/claude-code](https://platform.claude.com/claude-code) | 사용량 지표, 지출 추적, 팀 인사이트                         | [세부 정보](#access-analytics-for-api-customers)       |

<h2 id="access-analytics-for-team-and-enterprise">
  Team 및 Enterprise를 위한 분석 접근
</h2>

[claude.ai/analytics/claude-code](https://claude.ai/analytics/claude-code)로 이동합니다. 관리자 및 소유자가 대시보드를 볼 수 있습니다.

Team 및 Enterprise 대시보드에는 다음이 포함됩니다:

* **사용량 지표**: 수락된 코드 라인, 제안 수락률, 일일 활성 사용자 및 세션
* **기여도 지표**: Claude Code 지원으로 배포된 PR 및 코드 라인([GitHub 통합](#enable-contribution-metrics) 포함)
* **리더보드**: Claude Code 사용량으로 순위가 매겨진 상위 기여자
* **데이터 내보내기**: 사용자 정의 보고를 위해 기여도 데이터를 CSV로 다운로드

사용자별 토큰 수 및 비용 추정치는 [OpenTelemetry 내보내기](/docs/ko/monitoring-usage)를 구성하거나, 조직의 분석 설정에서 [지출 보고서](https://support.claude.com/en/articles/12883420-view-usage-analytics-for-team-and-enterprise-plans)를 내보내세요. 이 보고서는 사용자별 및 모델별 토큰 사용량 및 예상 사용량 크레딧 지출을 나열합니다.

<h3 id="enable-contribution-metrics">
  기여도 지표 활성화
</h3>

<Note>
  기여도 지표는 공개 베타 상태이며 Claude for Teams 및 Claude for Enterprise 플랜에서 사용할 수 있습니다. 이러한 지표는 claude.ai 조직 내의 사용자만 포함합니다. Claude Console API 또는 타사 통합을 통한 사용량은 포함되지 않습니다.
</Note>

사용량 및 채택 데이터는 모든 Claude for Teams 및 Claude for Enterprise 계정에서 사용할 수 있습니다. 기여도 지표는 GitHub 조직을 연결하기 위해 추가 설정이 필요합니다.

분석 설정을 구성하려면 소유자 역할이 필요합니다. GitHub 관리자가 GitHub 앱을 설치해야 합니다.

<Warning>
  기여도 지표는 [Zero Data Retention](/docs/ko/zero-data-retention)이 활성화된 조직에서는 사용할 수 없습니다. 분석 대시보드는 사용량 지표만 표시합니다.
</Warning>

<Steps>
  <Step title="GitHub 앱 설치">
    GitHub 관리자가 [github.com/apps/claude](https://github.com/apps/claude)에서 조직의 GitHub 계정에 Claude GitHub 앱을 설치합니다.
  </Step>

  <Step title="Claude Code 분석 활성화">
    Claude 소유자가 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)로 이동하여 Claude Code 분석 기능을 활성화합니다.
  </Step>

  <Step title="GitHub 분석 활성화">
    같은 페이지에서 "GitHub 분석" 토글을 활성화합니다.
  </Step>

  <Step title="GitHub로 인증">
    GitHub 인증 흐름을 완료하고 분석에 포함할 GitHub 조직을 선택합니다.
  </Step>
</Steps>

활성화 후 일반적으로 24시간 이내에 데이터가 나타나며, 매일 업데이트됩니다. 데이터가 나타나지 않으면 다음 메시지 중 하나가 표시될 수 있습니다:

* **"GitHub 앱 필수"**: 기여도 지표를 보려면 GitHub 앱을 설치하세요
* **"데이터 처리 진행 중"**: 며칠 후 다시 확인하고 데이터가 나타나지 않으면 GitHub 앱이 설치되었는지 확인하세요

기여도 지표는 GitHub Cloud 및 GitHub Enterprise Server를 지원합니다.

<h3 id="review-summary-metrics">
  요약 지표 검토
</h3>

<Note>
  이러한 지표는 의도적으로 보수적이며 Claude Code의 실제 영향을 과소평가합니다. Claude Code의 관여도가 높은 라인 및 PR만 계산됩니다.
</Note>

대시보드는 상단에 다음 요약 지표를 표시합니다:

* **CC가 포함된 PR**: Claude Code로 작성된 코드 라인이 하나 이상 포함된 병합된 풀 요청의 총 개수
* **CC가 포함된 코드 라인**: Claude Code 지원으로 작성된 모든 병합된 PR의 총 코드 라인 수입니다. "효과적인 라인"만 계산됩니다: 정규화 후 3자 이상의 라인, 빈 라인 및 괄호나 사소한 구두점만 있는 라인 제외.
* **Claude Code가 포함된 PR (%)**: Claude Code 지원 코드를 포함하는 모든 병합된 PR의 백분율
* **제안 수락률**: 사용자가 Claude Code의 코드 편집 제안을 수락하는 횟수의 백분율(Edit, Write, NotebookEdit 도구 사용 포함)
* **수락된 코드 라인**: 사용자가 세션에서 수락한 Claude Code로 작성된 총 코드 라인 수입니다. 거부된 제안은 제외되며 후속 삭제는 추적하지 않습니다.

<h3 id="explore-the-charts">
  차트 탐색
</h3>

대시보드에는 시간 경과에 따른 추세를 시각화하는 여러 차트가 포함되어 있습니다.

<h4 id="track-adoption">
  채택 추적
</h4>

채택 차트는 일일 사용 추세를 보여줍니다:

* **사용자**: 일일 활성 사용자
* **세션**: 일일 활성 Claude Code 세션 수

<h4 id="measure-prs-per-user">
  사용자당 PR 측정
</h4>

이 차트는 시간 경과에 따른 개별 개발자 활동을 표시합니다:

* **사용자당 PR**: 일일 병합된 PR의 총 개수를 일일 활성 사용자로 나눈 값
* **사용자**: 일일 활성 사용자

이를 사용하여 Claude Code 채택이 증가함에 따라 개별 생산성이 어떻게 변하는지 이해할 수 있습니다.

<h4 id="view-pull-requests-breakdown">
  풀 요청 분석 보기
</h4>

풀 요청 차트는 병합된 PR의 일일 분석을 보여줍니다:

* **CC가 포함된 PR**: Claude Code 지원 코드를 포함하는 풀 요청
* **CC가 포함되지 않은 PR**: Claude Code 지원 코드를 포함하지 않는 풀 요청

**코드 라인** 보기로 전환하여 PR 개수가 아닌 코드 라인으로 동일한 분석을 확인합니다.

<h4 id="find-top-contributors">
  상위 기여자 찾기
</h4>

리더보드는 기여도 볼륨으로 순위가 매겨진 상위 10명의 사용자를 보여줍니다. 다음 사이를 전환합니다:

* **풀 요청**: 각 사용자에 대해 Claude Code가 포함된 PR 대 모든 PR을 표시합니다
* **코드 라인**: 각 사용자에 대해 Claude Code가 포함된 라인 대 모든 라인을 표시합니다

**모든 사용자 내보내기**를 클릭하여 모든 사용자의 완전한 기여도 데이터를 CSV 파일로 다운로드합니다. 내보내기에는 표시된 상위 10명뿐만 아니라 모든 사용자가 포함됩니다.

<h3 id="pr-attribution">
  PR 속성
</h3>

기여도 지표가 활성화되면 Claude Code는 병합된 풀 요청을 분석하여 Claude Code 지원으로 작성된 코드를 결정합니다. 이는 Claude Code 세션 활동을 각 PR의 코드와 일치시켜 수행됩니다.

<h4 id="tagging-criteria">
  태깅 기준
</h4>

PR은 Claude Code 세션 중에 작성된 코드 라인이 하나 이상 포함되어 있으면 "Claude Code 포함"으로 태깅됩니다. 시스템은 보수적인 일치를 사용합니다: Claude Code의 관여도가 높은 코드만 지원되는 것으로 계산됩니다.

<h4 id="attribution-process">
  속성 프로세스
</h4>

풀 요청이 병합될 때:

1. 추가된 라인이 PR diff에서 추출됩니다
2. 일치하는 파일을 편집한 Claude Code 세션이 시간 창 내에서 식별됩니다
3. PR 라인이 여러 전략을 사용하여 Claude Code 출력과 일치합니다
4. AI 지원 라인 및 총 라인에 대한 지표가 계산됩니다

비교 전에 라인이 정규화됩니다: 공백이 제거되고, 여러 공백이 축약되며, 따옴표가 표준화되고, 텍스트가 소문자로 변환됩니다.

Claude Code 지원 라인을 포함하는 병합된 풀 요청은 GitHub에서 `claude-code-assisted`로 레이블이 지정됩니다.

<h4 id="time-window">
  시간 창
</h4>

PR 병합 날짜 21일 전부터 2일 후까지의 세션이 속성 일치를 위해 고려됩니다.

<h4 id="excluded-files">
  제외된 파일
</h4>

특정 파일은 자동 생성되기 때문에 분석에서 자동으로 제외됩니다:

* 잠금 파일: package-lock.json, yarn.lock, Cargo.lock 등
* 생성된 코드: Protobuf 출력, 빌드 아티팩트, 축소된 파일
* 빌드 디렉토리: dist/, build/, node\_modules/, target/
* 테스트 픽스처: 스냅샷, 카세트, 모의 데이터
* 1,000자 이상의 라인(축소되거나 생성된 가능성이 높음)

<h4 id="attribution-notes">
  속성 참고 사항
</h4>

속성 데이터를 해석할 때 다음 추가 세부 정보를 염두에 두세요:

* 개발자가 20% 이상의 차이로 실질적으로 다시 작성한 코드는 Claude Code에 속성되지 않습니다
* 21일 창 외의 세션은 고려되지 않습니다
* 알고리즘은 속성을 수행할 때 PR 소스 또는 대상 분기를 고려하지 않습니다

<h3 id="get-the-most-from-analytics">
  분석에서 최대한 활용하기
</h3>

기여도 지표를 사용하여 ROI를 입증하고, 채택 패턴을 식별하며, 다른 사람이 시작하도록 도울 수 있는 팀 구성원을 찾습니다.

<h4 id="monitor-adoption">
  채택 모니터링
</h4>

채택 차트 및 사용자 수를 추적하여 다음을 식별합니다:

* 모범 사례를 공유할 수 있는 활성 사용자
* 조직 전체의 전반적인 채택 추세
* 마찰이나 문제를 나타낼 수 있는 사용량 감소

<h4 id="measure-roi">
  ROI 측정
</h4>

기여도 지표는 자신의 코드베이스의 데이터로 "이 도구가 투자할 가치가 있는가?"라는 질문에 답하는 데 도움이 됩니다:

* 채택이 증가함에 따라 시간 경과에 따른 사용자당 PR의 변화를 추적합니다
* Claude Code 포함 및 미포함으로 배포된 PR 및 코드 라인을 비교합니다
* [DORA 지표](https://dora.dev/), 스프린트 속도 또는 기타 엔지니어링 KPI와 함께 사용하여 Claude Code 채택으로 인한 변화를 이해합니다

<h4 id="identify-power-users">
  파워 사용자 식별
</h4>

리더보드는 높은 Claude Code 채택을 가진 팀 구성원을 찾는 데 도움이 되며, 이들은 다음을 수행할 수 있습니다:

* 팀과 프롬프팅 기법 및 워크플로우 공유
* 잘 작동하는 것에 대한 피드백 제공
* 새 사용자 온보딩 지원

<h4 id="access-data-programmatically">
  프로그래매틱 방식으로 데이터 접근
</h4>

Enterprise 플랜에서 [Claude Enterprise Analytics API](https://platform.claude.com/docs/en/api/admin/analytics)는 Claude Code를 포함한 Claude 전체 표면에서 조직의 사용자별 참여, 사용량 및 비용 보고서를 반환합니다. 기본 소유자는 [claude.ai/analytics/api-keys](https://claude.ai/analytics/api-keys)에서 `read:analytics` 범위를 가진 키를 생성합니다. API는 Teams 플랜에서 사용할 수 없습니다.

GitHub를 통해 이 데이터를 쿼리하려면 `claude-code-assisted` 레이블이 지정된 PR을 검색합니다.

<h2 id="access-analytics-for-api-customers">
  API 고객을 위한 분석 접근
</h2>

Claude Console을 사용하는 API 고객은 [platform.claude.com/claude-code](https://platform.claude.com/claude-code)에서 분석에 접근할 수 있습니다. 대시보드에 접근하려면 UsageView 권한이 필요하며, 이는 Developer, Billing, Admin, Owner 및 Primary Owner 역할에 부여됩니다. 동일한 일일 사용자별 지표를 프로그래밍 방식으로 가져오려면 Admin API 키를 사용하여 [Claude Code Analytics API](https://platform.claude.com/docs/ko/build-with-claude/claude-code-analytics-api)를 사용합니다.

<Note>
  GitHub 통합을 포함한 기여도 지표는 현재 API 고객에게 사용할 수 없습니다. Console 대시보드는 사용량 및 지출 지표만 표시합니다.
</Note>

Console 대시보드는 다음을 표시합니다:

* **수락된 코드 라인**: 사용자가 세션에서 수락한 Claude Code로 작성된 총 코드 라인 수입니다. 거부된 제안은 제외되며 후속 삭제는 추적하지 않습니다.
* **제안 수락률**: 사용자가 코드 편집 도구 사용을 수락하는 횟수의 백분율(Edit, Write, NotebookEdit 도구 포함).
* **활동**: 차트에 표시된 일일 활성 사용자 및 세션.
* **지출**: 사용자 수와 함께 일일 API 비용(달러).

<h3 id="view-team-insights">
  팀 인사이트 보기
</h3>

팀 인사이트 테이블은 사용자별 지표를 표시합니다:

* **구성원**: Claude Code에 인증한 모든 사용자. API 키 사용자는 키 식별자로 표시되고, OAuth 사용자는 이메일 주소로 표시됩니다.
* **이번 달 지출**: 현재 달의 사용자별 총 API 비용.
* **이번 달 라인**: 현재 달의 사용자별 수락된 코드 라인의 총합.

<Note>
  Console 대시보드의 지출 수치는 분석 목적의 추정치입니다. 실제 비용은 청구 페이지를 참조하세요.
</Note>

<h2 id="related-resources">
  관련 리소스
</h2>

* [OpenTelemetry를 사용한 모니터링](/docs/ko/monitoring-usage): 실시간 지표 및 이벤트를 관찰성 스택으로 내보내기
* [비용 효과적으로 관리하기](/docs/ko/costs): 지출 한도 설정 및 토큰 사용량 최적화
* [권한](/docs/ko/permissions): 역할 및 권한 구성
