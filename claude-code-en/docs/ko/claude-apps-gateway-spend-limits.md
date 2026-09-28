> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude 앱 게이트웨이 지출 한도

> Claude 앱 게이트웨이를 통해 각 개발자의 지출을 일, 주 또는 월 단위로 제한합니다. Admin API로 한도를 설정하면 게이트웨이가 모든 요청에서 실시간으로 이를 적용합니다.

지출 한도는 각 개발자가 주어진 일, 주 또는 월 동안 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway)를 통해 지출할 수 있는 금액을 제한합니다. 개발자가 한도를 초과하면 게이트웨이는 다음 요청에서 `429`를 반환하고 기간이 재설정되거나 관리자가 한도를 올릴 때까지 해당 개발자를 차단합니다. 지출 한도를 사용하여 각 개발자, 그룹 또는 전체 조직에 모두가 공유하는 자격 증명에 대한 상한선을 설정합니다.

Claude 앱 게이트웨이는 하나의 공유 업스트림 자격 증명을 통해 모든 추론을 전달하므로 공급자의 청구서는 개별 개발자가 아닌 해당 자격 증명에 모든 것을 귀속시킵니다. 개발자별 한도가 없으면 하나의 폭주하는 에이전트 플릿이 조직의 전체 약정을 소비할 수 있습니다. 지출 한도는 게이트웨이의 개발자별 보기이자 해당 공유 청구서 위의 차단기입니다.

<h2 id="set-a-cap">
  한도 설정
</h2>

`gateway.yaml`에서 [`admin:`](/docs/ko/claude-apps-gateway-config#admin) 블록이 구성되면 게이트웨이는 `/v1/organizations/spend_limits`에서 admin API를 제공하고 모든 추론 요청에서 한도를 실시간으로 적용합니다. 한도 자체는 `gateway.yaml`이 아닌 해당 API를 통해 설정됩니다. 각 `POST /v1/organizations/spend_limits` 요청은 `{scope, amount, period}`에서 하나의 한도를 생성하거나 대체합니다. API는 Anthropic의 공개 [Admin API](https://platform.claude.com/docs/en/manage-claude/admin-api) 지출 한도 엔드포인트의 와이어 형태를 미러링하므로 해당 계약에 대해 작성된 HTTP 클라이언트는 기본 URL을 변경하여 게이트웨이를 대상으로 할 수 있습니다.

이 요청은 모든 개발자를 위해 월별 \$500의 조직 전체 기본값을 설정합니다:

```bash theme={null}
curl -sS https://claude-gateway.internal.example.com/v1/organizations/spend_limits \
  -H "x-api-key: $GATEWAY_ADMIN_WRITE_KEY" \
  -H "Content-Type: application/json" \
  -d '{"scope": {"type": "organization"}, "amount": "50000", "period": "monthly"}'
```

이 요청은 `contractors` 그룹의 각 멤버에 더 엄격한 일일 \$100 한도를 추가합니다:

```bash theme={null}
curl -sS https://claude-gateway.internal.example.com/v1/organizations/spend_limits \
  -H "x-api-key: $GATEWAY_ADMIN_WRITE_KEY" \
  -H "Content-Type: application/json" \
  -d '{"scope": {"type": "rbac_group", "rbac_group_id": "contractors"}, "amount": "10000", "period": "daily"}'
```

| 필드           | 값                                    | 설명                                                                                                                                                                                                                                                                                                                 |
| ------------ | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `scope.type` | `user`, `rbac_group`, `organization` | `user`는 OpenID Connect (OIDC) `sub`로 하나의 개발자를 대상으로 하며, 이는 ID 공급자가 할당하는 안정적인 사용자 ID입니다. `scope.user_id`로 전달합니다. `rbac_group`은 [IdP 그룹](/docs/ko/claude-apps-gateway-config#managed)을 이름으로 대상으로 하며, `scope.rbac_group_id`로 전달합니다. `organization`은 조직 전체 기본값입니다. 게이트웨이는 세 가지 모두 허용합니다. Anthropic의 공개 `POST`는 현재 사용자 전용입니다. |
| `amount`     | USD 센트의 정수 문자열 또는 `null`             | `null`은 무제한입니다. `"0"`은 모든 요청을 차단하는 0 한도입니다.                                                                                                                                                                                                                                                                        |
| `period`     | `daily`, `weekly`, `monthly`         | 범위는 기간당 하나의 한도를 보유할 수 있으며 각각 독립적으로 적용됩니다. 개발자는 이들 중 하나를 초과하면 차단됩니다.                                                                                                                                                                                                                                                |

그룹 또는 조직 한도는 각 멤버가 상속하는 좌석별 기본값이지 공유 풀이 아닙니다. 기간당 개발자의 유효 한도는 다음 순서로 해결됩니다: 사용자별 재정의, 그룹 한도 중 가장 제한적인 것, 조직 기본값, 무제한. [`admin.group_limit_mode: max`](/docs/ko/claude-apps-gateway-config#admin)는 다중 그룹 동점 결정을 가장 제한적인 것 대신 가장 제한적이지 않은 것으로 뒤집습니다.

<h3 id="authenticate-to-the-admin-api">
  Admin API에 인증
</h3>

다음 중 하나를 보냅니다:

* [`admin.write_keys`](/docs/ko/claude-apps-gateway-config#admin)의 키와 일치하는 `x-api-key` 헤더(전체 액세스) 또는 `admin.read_keys`(`GET` 전용 액세스). 각 키는 감사 로그에 `admin-key:<id>`로 나타나는 `id`를 전달하므로 Terraform, CI 및 각 자동화에 자신의 키를 제공합니다.
* `groups` 클레임이 [`admin.admin_groups`](/docs/ko/claude-apps-gateway-config#admin) 중 하나를 포함하는 게이트웨이 베어러 토큰. 이는 전체 액세스이며 `oidc:<sub>`로 감사되므로 인간 관리자에게 선호됩니다.

<h2 id="how-enforcement-works">
  적용 방식
</h2>

각 `/v1/messages` 요청에서 게이트웨이는 개발자의 한도와 기간 누적 지출을 하나의 Postgres 쿼리로 조회합니다. 한도를 초과한 개발자는 `error.type: billing_error`와 헤더 `x-should-retry: false`를 포함한 `429`를 받습니다.

메시지는 기간과 재설정 시간을 명시합니다. 예를 들어 `spend limit reached (daily; resets 2026-08-08 00:00 UTC)`이며, 설정된 경우 [`admin.blocked_message`](/docs/ko/claude-apps-gateway-config#admin)가 뒤따릅니다. 개발자가 여러 한도를 동시에 초과하면 메시지는 가장 늦게 재설정되는 한도를 명시합니다. 응답에는 해당 재설정까지 남은 초 단위 시간을 나타내는 `retry-after` 헤더도 포함됩니다. 게이트웨이 서버의 v2.1.225 이전 버전에서는 메시지가 기간, 재설정 시간 또는 `retry-after` 헤더 없이 `spend limit reached`였습니다.

v2.1.227 이상에서는 `<public_url>/protocol`의 프로토콜 참조에 정확한 사용량 제한 응답 헤더와 `429` 본문도 나열됩니다.

한도는 UTC 달력 경계에서 재설정됩니다. 매일 00:00 UTC, 매주 월요일, 매월 1일에 재설정됩니다. 게이트웨이는 토큰 계산이 무료이므로 `/v1/messages/count_tokens`을 차단하지 않습니다.

<h3 id="how-requests-are-priced">
  요청 가격 책정 방식
</h3>

각 응답 후 사용량 미터는 토큰 수를 읽고 일일, 주간 및 월간 카운터에 비용을 추가합니다. 클라이언트로 전송된 바이트는 건드리지 않으므로 미터링 실패가 응답을 손상시킬 수 없습니다. 금액은 USD 추정값이며 송장이 아닌 차단기입니다. 청구를 위해 공급자의 사용량 보고에 대해 조정합니다.

미터는 다음 순서로 각 요청의 요금을 선택합니다.

1. 요청을 처리한 업스트림에 대한 일치하는 [`pricing.overrides`](/docs/ko/claude-apps-gateway-config#pricing) 행입니다. v2.1.227 이상이 필요합니다.
2. Claude Code 비용 표가 인식하는 업스트림 모델 ID의 정가입니다. 이 표는 Anthropic, Amazon Bedrock, Google Cloud의 Agent Platform 및 Microsoft Foundry ID 형식을 허용합니다.
3. Amazon Bedrock 애플리케이션 추론 프로필 ARN 또는 Microsoft Foundry 배포 이름과 같이 모델 이름을 포함하지 않는 업스트림 문자열에 대해 해당 업스트림 ID에 매핑한 [`models[].id`](/docs/ko/claude-apps-gateway-config#models)의 정가입니다. v2.1.218 이상이 필요합니다.
4. 미알려진 모델 계층인 백만 입력/출력 토큰당 \$5/\$25입니다. 따라서 미터가 배치할 수 없는 ID는 절대 무료가 아닙니다. 게이트웨이는 부팅 시 및 런타임에 ID당 한 번 경고합니다.

어떤 요금이 적용되든 미터는 금액에 [`pricing.multiplier`](/docs/ko/claude-apps-gateway-config#pricing)를 곱합니다. 기본값은 `1`입니다.

클라이언트 중단도 청구됩니다. 스트림이 업스트림의 최종 사용량 프레임 없이 종료되면 미터는 클라이언트로 이미 전송된 텍스트에 대해 출력 토큰당 약 4자의 하한 추정값을 청구합니다. 따라서 요청을 조기에 중단해도 한도를 회피할 수 없습니다.

<h3 id="postgres-availability">
  Postgres 가용성
</h3>

사전 확인 쿼리는 2초 타임아웃으로 Postgres를 쿼리합니다. 저장소에 연결할 수 없거나 타임아웃되면 기본적으로 적용이 열린 상태로 실패합니다. 요청이 진행되고 게이트웨이가 경고를 기록하며 응답에는 `anthropic-ratelimit-unified-*` 헤더가 없습니다. 대신 [`enforcement.fail_closed_on_error: true`](/docs/ko/claude-apps-gateway-config#enforcement)를 설정하여 닫힌 상태로 실패하면 동일한 `429 billing_error`를 반환하지만 메시지는 `spend limit unavailable`이며 기간, 재설정 시간 또는 `retry-after` 헤더가 없습니다. 열린 상태 실패는 저장소 중단이 추론 중단이 되는 것을 방지합니다. 닫힌 상태 실패는 미계량 지출이 없음을 보장합니다.

<h3 id="usage-warnings-in-claude-code">
  Claude Code의 사용량 경고
</h3>

Claude Code는 개발자가 한도에 접근할 때 경고합니다. 사용률이 75%를 초과하면 한 번, 가장 많이 소비된 한도의 95%를 초과하면 다시 경고합니다. 게이트웨이가 요청을 차단하면 Claude Code는 `admin.blocked_message`를 포함하여 게이트웨이의 `429` 메시지를 그대로 표시합니다.

경고는 응답 헤더에서 작동합니다.

* 게이트웨이 서버에서 v2.1.225 이상이면 한도가 있는 개발자에 대한 각 성공적인 `/v1/messages` 응답은 `anthropic-ratelimit-unified-*` 헤더에 자신의 한도 사용률과 재설정 시간을 포함합니다.
* 개발자의 머신에서도 v2.1.225 이상이면 Claude Code는 헤더를 읽고 경고를 표시합니다.

헤더는 항상 개발자 자신의 한도를 설명합니다. 게이트웨이는 공유 할당량을 설명하는 업스트림 공급자의 속도 제한 헤더를 제거하고 절대 전달하지 않습니다.

개발자의 머신에서 v2.1.251 이상이면 Claude Code는 동일한 헤더를 읽어 `/usage`에 **지출 한도** 막대를 표시합니다. 이는 한도의 사용 비율과 재설정 시간을 표시하며 [상태 줄](/docs/ko/statusline#rate-limit-usage) 입력에 `rate_limits.spend_limit` 객체를 추가합니다. Claude Code는 둘 다 달러 금액이 아닌 백분율로 표시하며 게이트웨이 서버에서 v2.1.225보다 최신 버전이 필요하지 않습니다.

<h2 id="admin-api-reference">
  Admin API 참조
</h2>

아래 엔드포인트는 `/v1/organizations/spend_limits` 아래에서 제공됩니다.

| 메서드 및 경로                                       | 설명                                                                                                                                  |
| ---------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `GET /v1/organizations/spend_limits`           | 구성된 한도를 나열합니다. 선택적으로 `organization`, `rbac_group` 또는 `user`의 `scope_type`으로 필터링됩니다. 쿼리: `?limit=&after_id=&before_id=&scope_type=`. |
| `POST /v1/organizations/spend_limits`          | `{scope, period}`에 대한 한도를 생성하거나 대체합니다.                                                                                              |
| `GET /v1/organizations/spend_limits/{id}`      | `spl_` 접두사가 있는 ID로 하나의 한도를 가져옵니다.                                                                                                   |
| `DELETE /v1/organizations/spend_limits/{id}`   | 하나의 한도를 삭제합니다. `{type: "spend_limit_deleted", id}`를 반환합니다.                                                                          |
| `GET /v1/organizations/spend_limits/effective` | 기간당 주체별로 해결된 한도 및 누적 지출.                                                                                                            |
| `GET /v1/organizations/spend_limits/audit`     | 관리자 변경 추적, 최신 우선. 쿼리: `?limit=&after_id=`.                                                                                          |

규칙은 Anthropic의 Admin API를 미러링합니다:

* 모든 객체의 `type`
* `spl_` 접두사가 있는 ID
* USD 센트의 정수 문자열로 된 금액. `POST`는 다른 `currency`를 `400`으로 거부합니다.
* `{type: "error", error: {type, message}, request_id}` 오류 봉투
* 성공 또는 오류인 모든 관리자 응답의 `request-id` 응답 헤더, 오류 본문도 `request_id`로 전달합니다.

모든 변경은 동일한 트랜잭션에서 `admin_audit`에 변경 전/후 행을 작성하며, `admin-key:<id>` 또는 `oidc:<sub>`에 귀속됩니다.

게이트웨이는 지출 한도 엔드포인트만 제공합니다. `spend_limit_increase_requests` 큐와 같은 다른 Admin API 표면은 게이트웨이의 Admin API의 일부가 아닙니다.

<h3 id="/effective">
  `/effective`
</h3>

`GET /v1/organizations/spend_limits/effective`는 Anthropic의 `SpendSummary` 스키마를 반환합니다. 각 행은 기간에 대한 주체이며, 해결된 한도, 기간 누적 지출 및 `actor` 객체를 포함합니다. 게이트웨이 특정 차이점:

* `user_id`는 OIDC `sub`입니다.
* `actor.name` 및 `actor.email_address`는 주체의 첫 번째 추론 요청이 게이트웨이를 통과할 때까지 `null`입니다. 게이트웨이에는 사용자 디렉토리가 없습니다. 각 사용자의 자체 세션 JWT에서 마지막 확인 값을 기록합니다.
* 각 행은 또한 `groups` 배열, 주체의 마지막 확인 IdP 그룹을 전달합니다. 이는 관리자 UI가 적용되는 모든 한도 계층을 표시할 수 있도록 하는 게이트웨이 확장입니다. Anthropic 형태의 클라이언트는 이를 무시합니다.
* `user_ids[]` 필터가 없으면 기록된 지출이 있는 주체를 나열합니다. 게이트웨이는 모든 조직 멤버를 열거할 수 없기 때문입니다.

그룹 소스 한도는 적용이 사용하는 동일한 `group_limit_mode` 동점 결정으로 마지막 확인 그룹에 대해 해결되므로 뷰어는 실제로 적용되는 한도를 표시합니다.

| 쿼리 매개변수          | 설명                                                              |
| ---------------- | --------------------------------------------------------------- |
| `user_ids[]`     | 반복 가능. OIDC `sub`로 특정 주체로 필터링합니다.                               |
| `period[]`       | 반복 가능. `daily`, `weekly` 또는 `monthly` 행으로 필터링합니다.               |
| `sort`           | `spend_desc`는 상위 지출자를 먼저 나열합니다. 정확히 하나의 `period[]`가 필요합니다.      |
| `q`              | OIDC `sub`, 마지막 확인 이메일 및 마지막 확인 표시 이름에 대한 대소문자 구분 없는 부분 문자열 필터. |
| `limit` / `page` | 페이지 크기(1–1000, 기본값 20) 및 이전 응답의 `next_page`에서 불투명 커서.           |

<Warning>
  `q=` 및 `user_ids[]=`는 GET 쿼리 문자열을 타고 있으므로 모든 프론팅 프록시 또는 로드 밸런서가 액세스 로그에서 이를 캡처합니다. PII 로그 정책이 엄격하면 거기서 이러한 매개변수를 스크럽합니다.
</Warning>

<h3 id="/audit">
  `/audit`
</h3>

지출 한도 변경 추적을 반환합니다. 누가 어떤 한도를 변경했는지, 변경 전/후 스냅샷, 최신 우선. `has_more`는 정확합니다. 이 엔드포인트는 첫 번째 당사자 와이어 형태가 아닌 로컬 Admin API 규칙을 따릅니다.

<h3 id="pagination">
  페이지 매김
</h3>

원본 목록은 `after_id` 및 `before_id`로 페이지를 매기며, 이는 상호 배타적인 `spl_…` ID입니다. 결과는 생성 순서로 정렬되고 `has_more`는 순회 방향을 반영합니다. `/effective`는 이전 응답에서 `?page=`로 다시 전달되는 불투명 `next_page` 토큰으로 페이지를 매기며, 주체는 오름차순으로 정렬되어 지출이 기록되는 동안 페이지가 안정적으로 유지됩니다. `limit`은 둘 다에서 1–1000, 기본값 20입니다. `/audit`는 `after_id`, 이전 페이지의 마지막 이벤트의 숫자 `id`로 페이지를 매기며, 해당 `limit`의 기본값은 100입니다.

<h2 id="data-lifecycle">
  데이터 수명 주기
</h2>

게이트웨이는 4개의 지출 관련 테이블을 보유합니다. 시간별 스윕은 보존 기간을 적용합니다:

| 테이블                | 내용                                            | 보존                                                                                        |
| ------------------ | --------------------------------------------- | ----------------------------------------------------------------------------------------- |
| `spend`            | 주체별 기간 누적 카운터(센트)                             | [`admin.spend_retention_months`](/docs/ko/claude-apps-gateway-config#admin), 기본값 13            |
| `spend_limits`     | 구성된 한도                                        | API를 통해 삭제될 때까지                                                                           |
| `admin_audit`      | 변경 추적                                         | [`admin.audit_retention_days`](/docs/ko/claude-apps-gateway-config#admin), 기본값 365             |
| `principal_emails` | 각 주체의 마지막 확인 이메일, 표시 이름 및 IdP 그룹. PII를 포함합니다. | [`admin.identity_retention_days`](/docs/ko/claude-apps-gateway-config#admin) 마지막 활동 이후, 기본값 90 |

개발자가 떠날 때 `DELETE /v1/organizations/spend_limits/{id}`를 통해 사용자별 한도를 삭제합니다. 지출 및 ID 행은 위의 보존 기간에 나이가 듭니다. 오프보딩 또는 데이터 주체 액세스 요청(DSAR)을 위해 한 사람을 즉시 지우려면 게이트웨이 데이터베이스에 대해 직접 `DELETE FROM principal_emails WHERE principal = '<sub>'`를 실행합니다. 이는 이메일, 이름 및 그룹을 보유하는 유일한 테이블을 제거합니다. `spend` 및 `admin_audit` 행은 의사명 OIDC `sub`만 참조하고 자체 기간에 나이가 듭니다.

<h2 id="related">
  관련
</h2>

* [`admin` 및 `enforcement` 구성](/docs/ko/claude-apps-gateway-config#admin): admin API 활성화 및 보존 조정
* [배포 가이드](/docs/ko/claude-apps-gateway-deploy#postgres): Postgres 스키마 및 백업 지침
