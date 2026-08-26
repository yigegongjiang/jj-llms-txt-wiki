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

각 `/v1/messages` 요청에서 게이트웨이는 개발자의 한도와 기간 누적 지출을 하나의 Postgres 쿼리로 해결합니다. 한도를 초과하면 요청은 `error.type: billing_error`와 헤더 `x-should-retry: false`를 포함한 `429`를 반환합니다. 메시지는 `spend limit reached`이고 설정된 경우 [`admin.blocked_message`](/docs/ko/claude-apps-gateway-config#admin)가 뒤따릅니다.

`/v1/messages/count_tokens`은 제외됩니다. 토큰 계산은 무료이므로 한도 상태와 관계없이 실행됩니다.

각 응답 후 사용량 미터는 응답에서 토큰 수를 읽고 USD 정가로 가격을 책정한 후 세 기간 버킷 모두에 대해 Postgres 카운터를 증가시킵니다. 미터는 스트림의 단일 리더이므로 클라이언트의 바이트는 손상되지 않으며 미터링 실패는 응답을 손상시키지 않습니다.

지출 한도는 토큰 수에서 USD 정가로 지출을 추정합니다. 이는 차단기이지 송장이 아닙니다. 권위 있는 청구를 위해 Anthropic Usage & Cost Admin API, Amazon Bedrock의 호출 로그 또는 Google Cloud의 Cloud Monitoring과 같은 공급자의 자체 사용량 보고에 대해 조정합니다.

가격 책정은 Claude Code CLI가 자체 비용 표시에 사용하는 것과 동일한 테이블을 사용하며, Anthropic, Amazon Bedrock (`us.anthropic.…-v1:0`), Google Cloud의 Agent Platform (`claude-…@date`) 및 Microsoft Foundry ID 형식 전체에서 동일한 모델 ID 정규화를 사용합니다. Microsoft Foundry 배포 이름 또는 추론 프로필 ARN과 같이 테이블이 배치할 수 없는 모델 ID는 0이 아닌 미알려진 모델 기본 계층인 백만 입력/출력 토큰당 \$5/\$25로 가격이 책정되므로 인식되지 않는 ID는 미계량으로 이동하여 한도를 우회할 수 없습니다. 게이트웨이는 부팅 시 및 런타임에 ID당 한 번 경고합니다.

클라이언트 중단도 청구됩니다. 업스트림은 스트림의 터미널 프레임에서만 출력 토큰을 보고하므로 중단된 스트림은 이를 전달하지 않습니다. 미터는 스트림된 콘텐츠 크기에서 보수적인 하한 추정값(토큰당 약 4자)을 유지하고 터미널 사용량 프레임이 누락된 경우에만 청구합니다. 완전한 스트림은 항상 업스트림 보고 수를 청구합니다. 이 없이 제한된 개발자는 출력을 스트림하고 끝나기 직전에 각 요청을 중단하여 계산되지 않고 지출할 수 있습니다.

<h3 id="postgres-availability">
  Postgres 가용성
</h3>

사전 확인 쿼리는 2초 타임아웃으로 Postgres를 쿼리합니다. 저장소에 연결할 수 없거나 타임아웃되면 기본적으로 적용이 열린 상태로 실패합니다. 요청이 진행되고 게이트웨이가 경고를 기록합니다. 대신 [`enforcement.fail_closed_on_error: true`](/docs/ko/claude-apps-gateway-config#enforcement)를 설정하여 닫힌 상태로 실패하면 메시지 `spend limit unavailable`과 함께 동일한 `429 billing_error`를 반환합니다. 열린 상태 실패는 저장소 중단이 추론 중단이 되는 것을 방지합니다. 닫힌 상태 실패는 미계량 지출이 없음을 보장합니다.

<h2 id="admin-api-reference">
  Admin API 참조
</h2>

아래 엔드포인트는 `/v1/organizations/spend_limits` 아래에서 제공됩니다.

| 메서드 및 경로                                       | 설명                                                         |
| ---------------------------------------------- | ---------------------------------------------------------- |
| `GET /v1/organizations/spend_limits`           | 구성된 한도를 나열합니다. 쿼리: `?limit=&after_id=&before_id=`.         |
| `POST /v1/organizations/spend_limits`          | `{scope, period}`에 대한 한도를 생성하거나 대체합니다.                     |
| `GET /v1/organizations/spend_limits/{id}`      | `spl_` 접두사가 있는 ID로 하나의 한도를 가져옵니다.                          |
| `DELETE /v1/organizations/spend_limits/{id}`   | 하나의 한도를 삭제합니다. `{type: "spend_limit_deleted", id}`를 반환합니다. |
| `GET /v1/organizations/spend_limits/effective` | 기간당 주체별로 해결된 한도 및 누적 지출.                                   |
| `GET /v1/organizations/spend_limits/audit`     | 관리자 변경 추적, 최신 우선. 쿼리: `?limit=`.                           |

규칙은 Anthropic의 Admin API를 미러링합니다:

* 모든 객체의 `type`
* `spl_` 접두사가 있는 ID
* USD 센트의 정수 문자열로 된 금액. `POST`는 다른 `currency`를 `400`으로 거부합니다.
* `{type: "error", error: {type, message}, request_id}` 오류 봉투
* 성공 또는 오류인 모든 관리자 응답의 `request-id` 응답 헤더, 본문의 `request_id`와 일치합니다.

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

지출 한도 변경 추적을 반환합니다. 누가 어떤 한도를 변경했는지, 변경 전/후 스냅샷 및 선택적 이유, 최신 우선. `has_more`는 정확합니다. 이 엔드포인트는 첫 번째 당사자 와이어 형태가 아닌 로컬 Admin API 규칙을 따릅니다.

<h3 id="pagination">
  페이지 매김
</h3>

원본 목록은 `after_id` 및 `before_id`로 페이지를 매기며, 이는 상호 배타적인 `spl_…` ID입니다. 결과는 생성 순서로 정렬되고 `has_more`는 순회 방향을 반영합니다. `/effective`는 이전 응답에서 `?page=`로 다시 전달되는 불투명 `next_page` 토큰으로 페이지를 매기며, 주체는 오름차순으로 정렬되어 지출이 기록되는 동안 페이지가 안정적으로 유지됩니다. `limit`은 둘 다에서 1–1000, 기본값 20입니다.

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

`identity_retention_days`는 의도적으로 `spend_retention_months`보다 짧습니다. 프로비저닝 해제된 ID는 새로 고침을 중지하고 나이가 들지만 익명 지출 카운터는 연간 보고를 위해 유지됩니다.

개발자가 떠날 때 `DELETE /v1/organizations/spend_limits/{id}`를 통해 사용자별 한도를 삭제합니다. 지출 및 ID 행은 위의 보존 기간에 나이가 듭니다. 오프보딩 또는 데이터 주체 액세스 요청(DSAR)을 위해 한 사람을 즉시 지우려면 게이트웨이 데이터베이스에 대해 직접 `DELETE FROM principal_emails WHERE principal = '<sub>'`를 실행합니다. 이는 이메일, 이름 및 그룹을 보유하는 유일한 테이블을 제거합니다. `spend` 및 `admin_audit` 행은 의사명 OIDC `sub`만 참조하고 자체 기간에 나이가 듭니다.

<h2 id="related">
  관련
</h2>

* [`admin` 및 `enforcement` 구성](/docs/ko/claude-apps-gateway-config#admin): admin API 활성화 및 보존 조정
* [배포 가이드](/docs/ko/claude-apps-gateway-deploy#postgres): Postgres 스키마 및 백업 지침
