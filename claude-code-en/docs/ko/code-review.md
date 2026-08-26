> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Code Review

> 다중 에이전트 분석을 통해 전체 코드베이스를 검토하여 논리 오류, 보안 취약점 및 회귀를 감지하는 자동화된 PR 검토를 설정합니다

<Note>
  Code Review는 연구 미리보기 상태이며 [Team 및 Enterprise](https://claude.ai/admin-settings/claude-code) 구독에서 사용 가능합니다. [Zero Data Retention](/docs/ko/zero-data-retention)이 활성화된 조직에서는 사용할 수 없습니다.
</Note>

Code Review는 GitHub 풀 요청을 분석하고 문제를 발견한 코드 라인에 인라인 댓글로 결과를 게시합니다. 전문화된 에이전트 집합이 전체 코드베이스의 맥락에서 코드 변경 사항을 검토하여 논리 오류, 보안 취약점, 손상된 엣지 케이스 및 미묘한 회귀를 찾습니다.

결과는 심각도별로 태그가 지정되며 PR을 승인하거나 차단하지 않으므로 기존 검토 워크플로우는 그대로 유지됩니다. 리포지토리에 `CLAUDE.md` 또는 `REVIEW.md` 파일을 추가하여 Claude가 플래그하는 항목을 조정할 수 있습니다.

관리되는 서비스 대신 자신의 CI 인프라에서 Claude를 실행하려면 [GitHub Actions](/docs/ko/github-actions) 또는 [GitLab CI/CD](/docs/ko/gitlab-ci-cd)를 참조하십시오. 자체 호스팅 GitHub 인스턴스의 리포지토리의 경우 [GitHub Enterprise Server](/docs/ko/github-enterprise-server)를 참조하십시오.

이 페이지에서 다루는 내용:

* [검토 작동 방식](#how-reviews-work)
* [설정](#set-up-code-review)
* [`@claude review` 및 `@claude review once`를 사용한 수동 검토 트리거](#manually-trigger-reviews)
* [`CLAUDE.md` 및 `REVIEW.md`를 사용한 검토 사용자 정의](#customize-reviews)
* [가격](#pricing)
* [실패한 실행 및 누락된 댓글 문제 해결](#troubleshooting)
* [로컬에서 diff 검토](#review-a-diff-locally) - `/code-review` 명령 사용

<Note>
  GitHub 앱을 설치하지 않고 터미널에서 로컬로 diff를 검토하려면 Claude Code 세션에서 `/code-review` 명령을 실행하십시오. [로컬에서 diff 검토](#review-a-diff-locally)를 참조하십시오.
</Note>

<h2 id="how-reviews-work">
  검토 작동 방식
</h2>

관리자가 조직에 대해 Code Review를 [활성화](#set-up-code-review)하면 리포지토리의 구성된 동작에 따라 PR이 열릴 때, 모든 푸시 시 또는 수동으로 요청할 때 검토가 트리거됩니다. PR에서 `@claude review`를 [댓글로 작성](#manually-trigger-reviews)하면 모든 모드에서 검토가 시작됩니다.

검토가 실행되면 여러 에이전트가 Anthropic 인프라에서 병렬로 diff 및 주변 코드를 분석합니다. 각 에이전트는 다른 클래스의 문제를 찾고, 검증 단계에서 후보를 실제 코드 동작과 비교하여 거짓 양성을 필터링합니다. 결과는 중복 제거되고 심각도별로 순위가 지정되며 문제가 발견된 특정 라인에 인라인 댓글로 게시됩니다. 문제가 발견되지 않으면 Code Review는 GitHub 확인 실행을 업데이트하여 문제가 감지되지 않았음을 표시합니다. Claude는 또한 PR에 짧은 확인 댓글을 게시할 수 있습니다.

검토는 PR 크기 및 복잡도에 따라 비용이 증가하며 평균 20분 내에 완료됩니다. 관리자는 [분석 대시보드](#view-usage)를 통해 검토 활동 및 지출을 모니터링할 수 있습니다.

<h3 id="severity-levels">
  심각도 수준
</h3>

각 결과는 심각도 수준으로 태그가 지정됩니다:

| 마커 | 심각도          | 의미                             |
| :- | :----------- | :----------------------------- |
| 🔴 | Important    | 병합 전에 수정해야 하는 버그               |
| 🟡 | Nit          | 경미한 문제, 수정할 가치가 있지만 차단하지는 않음   |
| 🟣 | Pre-existing | 코드베이스에 존재하지만 이 PR에서 도입되지 않은 버그 |

결과에는 확장 가능한 확장 추론 섹션이 포함되어 있으므로 Claude가 문제를 플래그한 이유와 문제를 검증한 방법을 이해할 수 있습니다.

<h3 id="rate-and-reply-to-findings">
  결과에 대한 평가 및 응답
</h3>

Claude의 각 검토 댓글은 이미 👍 및 👎가 첨부되어 있으므로 두 버튼 모두 GitHub UI에 나타나 한 번의 클릭으로 평가할 수 있습니다. 결과가 유용했으면 👍을 클릭하고 잘못되었거나 노이즈가 많으면 👎를 클릭합니다. Anthropic은 PR이 병합된 후 반응 개수를 수집하고 이를 사용하여 검토자를 조정합니다. 반응은 재검토를 트리거하거나 PR의 어떤 것도 변경하지 않습니다.

인라인 댓글에 응답해도 Claude가 응답하거나 PR을 업데이트하도록 프롬프트하지 않습니다. 결과에 대해 조치하려면 코드를 수정하고 푸시합니다. PR이 푸시 트리거 검토에 구독되어 있으면 다음 실행이 문제가 수정되었을 때 스레드를 해결합니다. 푸시하지 않고 새로운 검토를 요청하려면 [최상위 PR 댓글](#manually-trigger-reviews)로 `@claude review once`를 댓글로 작성합니다.

<h3 id="check-run-output">
  확인 실행 출력
</h3>

인라인 검토 댓글 외에도 각 검토는 CI 확인과 함께 나타나는 **Claude Code Review** 확인 실행을 채웁니다. **Details** 링크를 확장하여 심각도별로 정렬된 모든 결과의 요약을 한 곳에서 확인합니다:

| 심각도          | 파일:라인                     | 문제                                     |
| ------------ | ------------------------- | -------------------------------------- |
| 🔴 Important | `src/auth/session.ts:142` | 토큰 새로고침이 로그아웃과 경쟁하여 오래된 세션이 활성 상태로 유지됨 |
| 🟡 Nit       | `src/auth/session.ts:88`  | `parseExpiry`가 잘못된 입력에서 자동으로 0을 반환함    |

각 결과는 **Files changed** 탭의 주석으로도 나타나며 관련 diff 라인에 직접 표시됩니다. Important 결과는 빨간색 마커로 렌더링되고, nit은 노란색 경고로, 기존 버그는 회색 공지로 렌더링됩니다. 주석과 심각도 테이블은 인라인 검토 댓글과 독립적으로 확인 실행에 기록되므로 GitHub가 이동한 라인의 인라인 댓글을 거부하더라도 사용 가능한 상태로 유지됩니다.

확인 실행은 항상 중립적인 결론으로 완료되므로 분기 보호 규칙을 통해 병합을 차단하지 않습니다. Code Review 결과에 따라 병합을 제어하려면 자신의 CI에서 확인 실행 출력에서 심각도 분석을 읽으십시오. Details 텍스트의 마지막 라인은 워크플로우가 `gh` 및 jq로 구문 분석할 수 있는 기계 판독 가능한 댓글입니다:

```bash theme={null}
gh api repos/OWNER/REPO/check-runs/CHECK_RUN_ID \
  --jq '.output.text | split("bughunter-severity: ")[1] | split(" -->")[0] | fromjson'
```

이는 심각도별 개수가 포함된 JSON 객체를 반환합니다(예: `{"normal": 2, "nit": 1, "pre_existing": 0}`). `normal` 키는 Important 결과의 개수를 보유합니다. 0이 아닌 값은 Claude가 병합 전에 수정할 가치가 있는 버그를 발견했음을 의미합니다.

<h3 id="what-code-review-checks">
  Code Review가 확인하는 항목
</h3>

기본적으로 Code Review는 정확성에 중점을 두고 있습니다: 형식 기본 설정이나 누락된 테스트 범위가 아닌 프로덕션을 중단할 버그입니다. 리포지토리에 [지침 파일을 추가](#customize-reviews)하여 확인하는 항목을 확장할 수 있습니다.

<h2 id="set-up-code-review">
  Code Review 설정
</h2>

관리자는 조직에 대해 한 번 Code Review를 활성화하고 포함할 리포지토리를 선택합니다.

<Steps>
  <Step title="Claude Code 관리자 설정 열기">
    [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)로 이동하여 Code Review 섹션을 찾습니다. Claude 조직에서 Owner 또는 Primary Owner 역할이 필요하며 GitHub 조직에 GitHub 앱을 설치할 수 있는 권한이 필요합니다.
  </Step>

  <Step title="설정 시작">
    **Setup**을 클릭합니다. 이렇게 하면 GitHub 앱 설치 흐름이 시작됩니다.
  </Step>

  <Step title="Claude GitHub 앱 설치">
    프롬프트를 따라 Claude GitHub 앱을 GitHub 조직에 설치합니다. 앱은 다음 리포지토리 권한을 요청합니다:

    * **Contents**: 읽기 및 쓰기
    * **Issues**: 읽기 및 쓰기
    * **Pull requests**: 읽기 및 쓰기

    Code Review는 콘텐츠에 대한 읽기 액세스와 풀 요청에 대한 쓰기 액세스를 사용합니다. 더 광범위한 권한 집합은 나중에 활성화하는 경우 [GitHub Actions](/docs/ko/github-actions)도 지원합니다.
  </Step>

  <Step title="리포지토리 선택">
    Code Review를 활성화할 리포지토리를 선택합니다. 리포지토리가 보이지 않으면 설치 중에 Claude GitHub 앱에 액세스 권한을 부여했는지 확인하십시오. 나중에 더 많은 리포지토리를 추가할 수 있습니다.
  </Step>

  <Step title="리포지토리별 검토 트리거 설정">
    설정이 완료되면 Code Review 섹션에 리포지토리가 테이블에 표시됩니다. 각 리포지토리에 대해 **Review Behavior** 드롭다운을 사용하여 검토가 실행되는 시기를 선택합니다:

    * **Once after PR creation**: PR이 열리거나 검토 준비 완료로 표시될 때 한 번 검토가 실행됩니다
    * **After every push**: PR 브랜치에 대한 모든 푸시에서 검토가 실행되어 PR이 진화함에 따라 새로운 문제를 감지하고 플래그된 문제를 수정할 때 스레드를 자동으로 해결합니다
    * **Manual**: [PR에서 `@claude review` 또는 `@claude review once`를 댓글로 작성](#manually-trigger-reviews)할 때만 검토가 시작되며, `@claude review`는 또한 PR을 이후 푸시에 대한 검토에 구독합니다

    모든 푸시에서 검토하면 가장 많은 검토가 실행되고 비용이 가장 많이 듭니다. 수동 모드는 특정 PR을 검토에 옵트인하려는 트래픽이 많은 리포지토리나 PR이 준비될 때까지만 검토를 시작하려는 경우에 유용합니다.
  </Step>
</Steps>

리포지토리 테이블은 최근 활동을 기반으로 각 리포지토리의 평균 검토 비용도 표시합니다. 행 작업 메뉴를 사용하여 리포지토리별로 Code Review를 켜거나 끄거나 리포지토리를 완전히 제거할 수 있습니다.

설정을 확인하려면 테스트 PR을 열어보십시오. 자동 트리거를 선택한 경우 **Claude Code Review**라는 확인 실행이 몇 분 내에 나타납니다. 수동을 선택한 경우 PR에서 `@claude review`를 댓글로 작성하여 첫 번째 검토를 시작합니다. 확인 실행이 나타나지 않으면 리포지토리가 관리자 설정에 나열되어 있고 Claude GitHub 앱이 액세스할 수 있는지 확인하십시오.

<h2 id="manually-trigger-reviews">
  수동으로 검토 트리거
</h2>

두 개의 댓글 명령이 요청 시 검토를 시작합니다. 둘 다 리포지토리의 구성된 트리거와 관계없이 작동하므로 수동 모드에서 특정 PR을 검토에 옵트인하거나 다른 모드에서 즉시 재검토를 받을 수 있습니다.

| 명령                    | 수행 작업                             |
| :-------------------- | :-------------------------------- |
| `@claude review`      | 검토를 시작하고 PR을 앞으로 푸시 트리거 검토에 구독합니다 |
| `@claude review once` | PR을 향후 푸시에 구독하지 않고 단일 검토를 시작합니다   |

PR의 현재 상태에 대한 피드백을 원하지만 이후의 모든 푸시가 검토를 발생시키지 않기를 원할 때 `@claude review once`를 사용합니다. 이는 빈번한 푸시가 있는 장기 실행 PR이나 PR의 검토 동작을 변경하지 않고 일회성 두 번째 의견을 원할 때 유용합니다.

댓글이 검토를 트리거하려면:

* 최상위 PR 댓글로 게시하고 diff 라인의 인라인 댓글로는 게시하지 않습니다
* 댓글의 시작 부분에 명령을 입력하고, 한 번만 형식을 사용하는 경우 `once`를 같은 라인에 입력합니다
* 리포지토리에 대한 소유자, 멤버 또는 협력자 액세스 권한이 있어야 합니다
* PR은 열려 있어야 합니다

자동 트리거와 달리 수동 트리거는 명시적 요청이 초안 상태와 관계없이 지금 검토를 원한다는 신호이므로 초안 PR에서 실행됩니다.

해당 PR에서 검토가 이미 실행 중인 경우 요청은 진행 중인 검토가 완료될 때까지 대기열에 추가됩니다. PR의 확인 실행을 통해 진행 상황을 모니터링할 수 있습니다.

<h2 id="customize-reviews">
  검토 사용자 정의
</h2>

Code Review는 리포지토리에서 두 개의 파일을 읽어 플래그할 항목을 안내합니다. 이들은 검토에 영향을 미치는 강도가 다릅니다:

* **`CLAUDE.md`**: Claude Code가 검토뿐만 아니라 모든 작업에 사용하는 공유 프로젝트 지침입니다. Code Review는 이를 프로젝트 컨텍스트로 읽고 새로 도입된 위반을 nit으로 플래그합니다.
* **`REVIEW.md`**: 검토 전용 지침으로 검토 파이프라인의 모든 에이전트에 최우선 순위로 직접 주입됩니다. 이를 사용하여 플래그되는 항목, 심각도 및 결과 보고 방식을 변경합니다.

<h3 id="claude-md">
  CLAUDE.md
</h3>

Code Review는 리포지토리의 `CLAUDE.md` 파일을 읽고 새로 도입된 위반을 [nit 수준](#severity-levels) 결과로 처리합니다. 이는 양방향으로 작동합니다: PR이 `CLAUDE.md` 문을 오래된 것으로 만드는 방식으로 코드를 변경하면 Claude는 문서도 업데이트해야 한다고 플래그합니다.

Claude는 디렉토리 계층 구조의 모든 수준에서 `CLAUDE.md` 파일을 읽으므로 하위 디렉토리의 `CLAUDE.md`의 규칙은 해당 경로 아래의 파일에만 적용됩니다. `CLAUDE.md` 작동 방식에 대한 자세한 내용은 [메모리 설명서](/docs/ko/memory)를 참조하십시오.

일반 Claude Code 세션에 적용하지 않으려는 검토 전용 지침의 경우 대신 [`REVIEW.md`](#review-md)를 사용하십시오.

<h3 id="review-md">
  REVIEW\.md
</h3>

`REVIEW.md`는 리포지토리 루트의 파일로 Code Review가 리포지토리에서 어떻게 작동하는지를 재정의합니다. 그 내용은 검토 파이프라인의 모든 에이전트의 시스템 프롬프트에 최우선 순위 지침 블록으로 주입되어 기본 검토 지침보다 우선합니다.

그것이 그대로 붙여넣어지기 때문에 `REVIEW.md`는 일반 지침입니다: [`@` import 구문](/docs/ko/memory#import-additional-files)은 확장되지 않으며 참조된 파일은 프롬프트로 읽혀지지 않습니다. 적용하려는 규칙을 파일에 직접 입력합니다.

<h4 id="what-you-can-tune">
  조정할 수 있는 항목
</h4>

`REVIEW.md`는 자유 형식 마크다운이므로 검토 지침으로 표현할 수 있는 모든 것이 범위 내입니다. 아래 패턴은 실제로 가장 큰 영향을 미칩니다.

**심각도**: 리포지토리에 대해 🔴 Important가 의미하는 바를 재정의합니다. 기본 보정은 프로덕션 코드를 대상으로 합니다. 문서 리포지토리, 구성 리포지토리 또는 프로토타입은 훨씬 더 좁은 정의를 원할 수 있습니다. Important인 결과 클래스와 최대 Nit인 결과 클래스를 명시적으로 명시합니다. 다른 방향으로도 확대할 수 있습니다. 예를 들어 기본 nit이 아닌 Important로 `CLAUDE.md` 위반을 처리합니다.

**Nit 볼륨**: 단일 검토가 게시하는 🟡 Nit 댓글의 수를 제한합니다. 산문 및 구성 파일은 영원히 다듬어질 수 있습니다. "최대 5개의 nit을 보고하고 나머지를 요약에 개수로 언급"과 같은 제한은 검토를 실행 가능하게 유지합니다.

**규칙 건너뛰기**: Claude가 결과를 게시하지 않아야 하는 경로, 분기 패턴 및 결과 카테고리를 나열합니다. 일반적인 후보는 생성된 코드, lockfile, 공급된 종속성 및 기계 작성 분기이며, linting 또는 맞춤법 검사와 같이 CI가 이미 적용하는 모든 것입니다. 완전한 정밀 검사를 보장하지 않지만 일부 검토를 보장하는 경로의 경우 완전히 건너뛰는 대신 더 높은 기준을 설정합니다: "`scripts/`에서는 거의 확실하고 심각한 경우에만 보고합니다."

**리포지토리별 확인**: 모든 PR에서 플래그하려는 규칙을 추가합니다. 예: "새 API 경로에는 통합 테스트가 있어야 합니다." `REVIEW.md`가 최우선 순위로 주입되기 때문에 이들은 긴 `CLAUDE.md`의 동일한 규칙보다 더 안정적으로 도착합니다.

**검증 기준**: 결과 클래스가 게시되기 전에 증거를 요구합니다. 예를 들어 "동작 주장은 명명에서의 추론이 아닌 소스의 `file:line` 인용이 필요합니다"는 그렇지 않으면 작성자에게 왕복을 비용으로 하는 거짓 양성을 줄입니다.

**재검토 수렴**: PR이 이미 검토되었을 때 Claude가 어떻게 작동해야 하는지 알려줍니다. "첫 번째 검토 후 새로운 nit을 억제하고 Important 결과만 게시"와 같은 규칙은 한 줄 수정이 스타일만으로 7라운드에 도달하는 것을 방지합니다.

**요약 형태**: 검토 본문이 `2 factual, 4 style`과 같은 한 줄 집계로 시작하도록 요청하고 그것이 경우일 때 "factual 문제 없음"으로 시작하도록 요청합니다. 작성자는 세부 사항 전에 작업의 형태를 알고 싶어합니다.

<h4 id="example">
  예시
</h4>

이 `REVIEW.md`는 백엔드 서비스의 심각도를 재보정하고, nit을 제한하고, 생성된 파일을 건너뛰고, 리포지토리별 확인을 추가합니다.

```markdown theme={null}
# 검토 지침

## Important가 여기서 의미하는 바

동작을 중단하거나 데이터를 유출하거나 롤백을 차단할 결과에 대해 Important를 예약합니다: 잘못된 논리, 범위가 지정되지 않은 데이터베이스 쿼리, 로그 또는 오류 메시지의 PII, 그리고 역호환되지 않는 마이그레이션입니다. 스타일, 명명 및 리팩토링 제안은 최대 Nit입니다.

## Nit 제한

검토당 최대 5개의 Nit을 보고합니다. 더 많이 발견한 경우 인라인으로 게시하는 대신 요약에서 "plus N similar items"라고 말합니다. 발견한 모든 것이 Nit인 경우 "No blocking issues"로 요약을 시작합니다.

## 보고하지 않음

- CI가 이미 적용하는 모든 것: lint, 형식, 타입 오류
- `src/gen/` 아래의 생성된 파일 및 모든 `*.lock` 파일
- 의도적으로 프로덕션 규칙을 위반하는 테스트 전용 코드

## 항상 확인

- 새 API 경로에는 통합 테스트가 있습니다
- 로그 라인에 이메일 주소, 사용자 ID 또는 요청 본문이 포함되지 않습니다
- 데이터베이스 쿼리는 호출자의 테넌트로 범위가 지정됩니다
```

<h4 id="keep-it-focused">
  초점 유지
</h4>

길이는 비용이 있습니다: 긴 `REVIEW.md`는 가장 중요한 규칙을 희석합니다. 검토 동작을 변경하는 지침으로 유지하고 일반 프로젝트 컨텍스트는 `CLAUDE.md`에 남겨둡니다.

<h2 id="view-usage">
  사용량 보기
</h2>

[claude.ai/analytics/code-review](https://claude.ai/analytics/code-review)로 이동하여 조직 전체의 Code Review 활동을 확인합니다. 대시보드는 다음을 표시합니다:

| 섹션                   | 표시 내용                            |
| :------------------- | :------------------------------- |
| PRs reviewed         | 선택한 시간 범위 동안 검토된 풀 요청의 일일 개수     |
| Cost weekly          | Code Review의 주간 지출               |
| Feedback             | 개발자가 문제를 해결하여 자동으로 해결된 검토 댓글의 개수 |
| Repository breakdown | 리포지토리별 검토된 PR 개수 및 해결된 댓글        |

관리자 설정의 리포지토리 테이블은 각 리포지토리의 검토당 평균 비용도 표시합니다. 대시보드 비용 수치는 활동 모니터링을 위한 추정치입니다. 청구서 정확한 지출의 경우 Anthropic 청구서를 참조하십시오.

<h2 id="pricing">
  가격
</h2>

Code Review는 토큰 사용량을 기반으로 청구됩니다. 각 검토는 평균 \$15-25의 비용이 소요되며, PR 크기, 코드베이스 복잡도 및 검증이 필요한 문제의 수에 따라 확장됩니다. Code Review 사용량은 [사용량 크레딧](https://support.claude.com/ko/articles/12429409-extra-usage-for-paid-claude-plans)을 통해 별도로 청구되며 계획에 포함된 사용량에 포함되지 않습니다.

선택한 검토 트리거는 총 비용에 영향을 미칩니다:

* **Once after PR creation**: PR당 한 번 실행됩니다
* **After every push**: 각 푸시에서 실행되어 푸시 수만큼 비용이 곱해집니다
* **Manual**: PR에서 누군가 `@claude review`를 댓글로 작성할 때까지 검토가 없습니다

모든 모드에서 `@claude review`를 [댓글로 작성](#manually-trigger-reviews)하면 PR이 푸시 트리거 검토에 옵트인되므로 해당 댓글 이후 푸시당 추가 비용이 발생합니다. 향후 푸시에 구독하지 않고 단일 검토를 실행하려면 대신 `@claude review once`를 댓글로 작성하십시오.

비용은 조직이 다른 Claude Code 기능에 Amazon Bedrock 또는 Google Cloud의 Agent Platform을 사용하는지 여부와 관계없이 Anthropic 청구서에 나타납니다. Code Review의 월간 지출 한도를 설정하려면 [claude.ai/admin-settings/usage](https://claude.ai/admin-settings/usage)로 이동하여 Claude Code Review 서비스의 한도를 구성합니다.

[분석](#view-usage)의 주간 비용 차트 또는 관리자 설정의 리포지토리별 평균 비용 열을 통해 지출을 모니터링합니다.

<h2 id="troubleshooting">
  문제 해결
</h2>

검토 실행은 최선의 노력입니다. 실패한 실행은 PR을 차단하지 않지만 자동으로 재시도하지도 않습니다. 이 섹션에서는 실패한 실행에서 복구하는 방법과 확인 실행이 찾을 수 없는 문제를 보고할 때 어디를 봐야 하는지 다룹니다.

<h3 id="retrigger-a-failed-or-timed-out-review">
  실패하거나 시간 초과된 검토 재트리거
</h3>

검토 인프라가 내부 오류에 도달하거나 시간 제한을 초과하면 확인 실행이 **Code review encountered an error** 또는 **Code review timed out** 제목으로 완료됩니다. 결론은 여전히 중립적이므로 병합을 차단하는 것은 없지만 결과가 게시되지 않습니다.

검토를 다시 실행하려면 PR에서 `@claude review once`를 댓글로 작성하십시오. 이렇게 하면 PR을 향후 푸시에 구독하지 않고 새로운 검토를 시작합니다. PR이 이미 푸시 트리거 검토에 구독되어 있으면 새 커밋을 푸시하면 새 검토도 시작됩니다.

GitHub의 Checks 탭의 **Re-run** 버튼은 Code Review를 재트리거하지 않습니다. 댓글 명령이나 새 푸시를 대신 사용하십시오.

<h3 id="review-didn’t-run-and-the-pr-shows-a-spend-cap-message">
  검토가 실행되지 않았고 PR이 지출 한도 메시지를 표시합니다
</h3>

조직의 월간 지출 한도에 도달하면 Code Review는 검토가 건너뛰어졌음을 설명하는 단일 댓글을 PR에 게시합니다. 검토는 다음 청구 기간의 시작 시 자동으로 재개되거나 관리자가 [claude.ai/admin-settings/usage](https://claude.ai/admin-settings/usage)에서 한도를 높일 때 즉시 재개됩니다.

<h3 id="find-issues-that-aren’t-showing-as-inline-comments">
  인라인 댓글로 표시되지 않는 문제 찾기
</h3>

확인 실행 제목이 문제가 발견되었다고 말하지만 diff에서 인라인 검토 댓글을 보지 못하면 결과가 표시되는 다른 위치를 확인하십시오:

* **Check run Details**: Checks 탭의 Claude Code Review 확인 옆에 있는 **Details**를 클릭합니다. 심각도 테이블은 인라인 댓글이 수락되었는지 여부와 관계없이 파일, 라인 및 요약과 함께 모든 결과를 나열합니다.
* **Files changed annotations**: PR의 **Files changed** 탭을 엽니다. 결과는 검토 댓글과 별도로 diff 라인에 직접 첨부된 주석으로 렌더링됩니다.
* **Review body**: 검토가 실행 중인 동안 PR을 푸시한 경우 일부 결과는 현재 diff에 더 이상 존재하지 않는 라인을 참조할 수 있습니다. 이들은 인라인 댓글이 아닌 검토 본문 텍스트의 **Additional findings** 제목 아래에 나타납니다.

<h2 id="review-a-diff-locally">
  로컬에서 diff 검토
</h2>

[`/code-review` 명령](/docs/ko/commands)은 GitHub 앱을 설치하지 않고 터미널에서 diff를 검토합니다. Claude Code 세션에서 실행하십시오: 정확성 버그 및 재사용, 단순화 및 효율성 정리를 보고합니다. 기본적으로 로컬 검토는 업스트림보다 앞선 브랜치의 커밋과 작업 트리의 커밋되지 않은 변경 사항을 포함합니다. `--comment`를 전달하여 결과를 인라인 PR 댓글로 게시하거나 `--fix`를 전달하여 검토 후 결과를 작업 트리에 적용합니다.

낮은 [노력 수준](/docs/ko/model-config#adjust-effort-level)은 더 적은 수의 높은 신뢰도 결과를 반환하는 반면, `high`부터 `max`까지는 더 광범위한 범위를 제공하며 불확실한 결과를 포함할 수 있습니다. 노력 인수 없이 검토는 세션의 현재 노력을 사용합니다. 기본 diff 대신 다른 항목을 검토하려면 대상을 전달합니다: 파일 경로, PR 번호, 브랜치 이름 또는 `main...my-feature`와 같은 ref 범위입니다. ref 범위 형식은 브랜치의 업스트림이 어떻게 구성되어 있는지와 관계없이 `my-feature`에서 `main`으로의 풀 요청이 포함할 커밋된 diff를 검토합니다.

`/code-review ultra --fix`는 클라우드에서 더 깊은 [ultrareview](/docs/ko/ultrareview)를 실행한 다음 세션으로 돌아올 때 결과를 작업 트리에 적용합니다. Ultrareview는 자체 범위를 사용합니다: 현재 브랜치와 저장소의 기본 브랜치, 그리고 작업 트리의 커밋되지 않은 및 스테이징된 변경 사항입니다.

이 명령은 v2.1.147 이전에 `/simplify`로 명명되었으며 기본적으로 수정 사항을 적용했습니다. v2.1.154부터 `/simplify`는 버그를 찾지 않고 수정 사항을 적용하는 별도의 정리 전용 검토를 실행합니다. 버그 찾기를 위해 `/simplify`를 스크립트했다면 변경되지 않은 `/code-review --fix`로 전환하십시오.

<h2 id="related-resources">
  관련 리소스
</h2>

Code Review는 Claude Code의 나머지 부분과 함께 작동하도록 설계되었습니다. PR을 열기 전에 로컬에서 검토를 실행하거나, 자체 호스팅 설정이 필요하거나, `CLAUDE.md`가 도구 전체에서 Claude의 동작을 형성하는 방식에 대해 더 깊이 알고 싶다면 다음 페이지가 좋은 다음 단계입니다:

* [Commands](/docs/ko/commands): 로컬 Claude Code 세션에서 `/code-review`를 실행하여 푸시 전에 diff를 확인합니다
* [GitHub Actions](/docs/ko/github-actions): 코드 검토 이상의 사용자 정의 자동화를 위해 자신의 GitHub Actions 워크플로우에서 Claude를 실행합니다
* [GitLab CI/CD](/docs/ko/gitlab-ci-cd): GitLab 파이프라인을 위한 자체 호스팅 Claude 통합
* [Memory](/docs/ko/memory): Claude Code 전체에서 `CLAUDE.md` 파일이 작동하는 방식
* [Analytics](/docs/ko/analytics): 코드 검토 이상으로 Claude Code 사용량을 추적합니다
