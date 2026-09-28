> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 자체 호스팅 환경에서 세션 ID 확인

> 자체 호스팅 환경의 세션에서 요청을 신뢰할 수 있도록 네트워크의 서비스가 CLAUDE_CODE_SESSION_ACCESS_TOKEN JWT를 확인합니다.

<Note>
  자체 호스팅 환경은 Team 및 Enterprise 플랜에서 공개 베타 상태입니다. [Owner](/docs/ko/cloud-environments#organization-shared-environments)가 [**Cloud environments** 관리자 페이지](https://claude.ai/admin-settings/cloud-environments)에서 **Allow self-hosted environments**를 활성화합니다. 이 페이지는 세션 ID 확인을 다룹니다. 설정은 [빠른 시작](/docs/ko/self-hosted-environments-quickstart)을 참조하고 프로덕션 배포는 [Deploy to production](/docs/ko/self-hosted-environments-deploy)에서 플릿 레시피를 참조하세요.
</Note>

[자체 호스팅 환경](/docs/ko/self-hosted-environments)을 사용하면 [웹의 Claude Code](/docs/ko/claude-code-on-the-web) 세션이 Anthropic의 인프라 대신 사용자가 운영하는 인프라에서 실행됩니다. 세션이 네트워크 내부에서 실행되므로 Claude는 내부 서비스를 직접 호출할 수 있습니다. 이러한 서비스는 요청이 환경의 Claude Code 세션에서 왔는지 확인하고 해당 세션을 생성한 사용자 또는 서비스 ID를 식별할 수 있는 방법이 필요합니다.

자체 호스팅 환경의 모든 세션은 `CLAUDE_CODE_SESSION_ACCESS_TOKEN` 환경 변수에서 서명된 JSON Web Token(JWT)을 받습니다. 세션은 토큰을 모든 bearer 자격증명처럼 제시합니다. 예를 들어 Claude가 실행하는 스크립트는 `curl -H "Authorization: Bearer $CLAUDE_CODE_SESSION_ACCESS_TOKEN"`을 사용하여 서비스를 호출할 수 있습니다. Anthropic은 토큰에 서명하고 공개 JWKS 엔드포인트에서 확인 키를 게시합니다. 서비스는 이러한 키를 가져오고 서명을 확인한 후 클레임을 읽어 어떤 액세스를 허용할지 결정합니다.

<h2 id="the-session-token">
  세션 토큰
</h2>

확인 코드를 작성하기 전에 토큰이 무엇을 확인하는지, 그리고 JWT 라이브러리가 어떤 형태를 볼 것인지 알아야 합니다.

<h3 id="what-the-token-proves">
  토큰이 증명하는 것
</h3>

유효한 토큰은 일부 사실을 확인하고 의도적으로 다른 것은 확인하지 않습니다:

* **확인함**: Anthropic이 특정 환경의 특정 세션에 대해 토큰을 발급했으며, 세션이 어떻게 생성되었는지: 조직의 사용자에 의해, 또는 조직의 서비스 ID에 의해([Claude Tag 채널 세션](https://claude.com/docs/claude-tag/concepts/agent-identity)이 시작되는 방식)
* **확인하지 않음**: 실행기 호스트의 어떤 프로세스가 이를 제시하는지. 토큰은 세션 내부의 환경 변수에 있으므로 Claude가 실행하는 모든 코드와 세션이 시작하는 모든 도구 또는 MCP 서버가 이를 읽고 제시할 수 있습니다.

서비스에 대한 두 가지 결과:

* 토큰이 다른 조직의 환경에 발급된 것을 거부하기 위해 `aud` 클레임을 환경 ID(환경 ID는 [**Cloud environments** 관리자 페이지](https://claude.ai/admin-settings/cloud-environments)에 표시되는 `ccpool_...` 값)에 대해 확인합니다.
* 토큰에서 파생된 자격증명을 단일 코딩 세션이 할 수 있는 작업으로 범위를 지정하고, 세션 작성자가 할 수 있는 모든 작업으로 범위를 지정하지 않습니다. [파생된 자격증명 범위 지정](#scope-derived-credentials)을 참고하세요.

<h3 id="token-format">
  토큰 형식
</h3>

`CLAUDE_CODE_SESSION_ACCESS_TOKEN`의 값은 `sk-ant-cc-` 접두사 다음에 표준 3부 JWT가 있습니다:

```text theme={null}
sk-ant-cc-<base64url header>.<base64url payload>.<base64url signature>
```

JWT 라이브러리에 값을 전달하기 전에 접두사를 제거합니다. Anthropic 호스팅 클라우드 세션에 발급된 토큰은 대신 `sk-ant-si-` 접두사를 가지며 다른 키 세트로 서명되므로 `sk-ant-cc-`로 시작하지 않는 값을 거부합니다.

서명 알고리즘은 `ES256`이며, 이는 SHA-256을 사용하는 P-256 곡선의 ECDSA입니다. 토큰 헤더는 JWKS에서 어떤 키가 이를 서명했는지 식별하는 `kid`를 포함합니다.

<h2 id="verify-the-token">
  토큰 확인
</h2>

확인은 두 위치 중 하나에서 실행됩니다. 네트워크의 서비스는 Anthropic의 게시된 키에 대해 토큰을 암호화 방식으로 확인하고, 세션 내부의 래퍼 스크립트는 실행기 바이너리의 내장 디코더를 대신 사용할 수 있습니다.

<h3 id="verify-the-token-from-your-service">
  서비스에서 토큰 확인
</h3>

Anthropic은 공개 인증되지 않은 엔드포인트에서 확인 키를 게시합니다:

```text theme={null}
https://api.anthropic.com/v1/code/.well-known/jwks.json
```

응답은 표준 [JSON Web Key Set](https://www.rfc-editor.org/rfc/rfc7517)입니다. Anthropic은 서명 키를 주기적으로 회전하며, 회전 전의 키는 이들이 서명한 토큰이 계속 확인될 수 있을 만큼 충분히 오래 세트에 남아 있으므로 단일 키를 고정하지 마세요. 엔드포인트는 `Cache-Control: public, max-age=300`을 설정하므로 키 세트를 캐시하고 5분마다 다시 가져오는 것이 안전합니다.

들어오는 각 토큰을 다음 검사에 대해 확인합니다:

<Steps>
  <Step title="접두사 확인">
    `sk-ant-cc-`로 시작하지 않으면 값을 거부한 다음 해당 접두사를 제거합니다. 나머지는 표준 compact JWT입니다.
  </Step>

  <Step title="서명 확인">
    JWKS를 가져오고, 토큰 헤더와 일치하는 `kid`를 가진 키를 선택하고, `ES256` 서명을 확인합니다. `alg` 헤더가 `ES256`이 아닌 토큰을 거부합니다. 캐시된 키 세트에 없는 `kid`를 가진 토큰이 도착하면 거부하기 전에 JWKS를 한 번 다시 가져옵니다: 회전 후 새 토큰은 캐시된 세트가 아직 없는 키로 서명됩니다.
  </Step>

  <Step title="발급자 확인">
    `iss`가 정확히 `ccr`이 아니면 토큰을 거부합니다.
  </Step>

  <Step title="환경에 대해 대상 확인">
    `aud` 클레임은 배열입니다. 환경 ID(형식: `ccpool_...`)를 포함하지 않으면 토큰을 거부합니다. 환경 ID는 [**Cloud environments** 관리자 페이지](https://claude.ai/admin-settings/cloud-environments)의 환경 세부 정보 대화 상자에 표시되며, 환경의 모든 세션 토큰에서 `ccr:pool_id` 클레임으로 나타납니다. 이 검사는 토큰을 환경으로 범위를 지정하고 다른 조직에 발급된 토큰을 거부하는 것입니다.
  </Step>

  <Step title="역할 확인">
    `ccr:role`이 정확히 `session_worker`가 아니면 토큰을 거부합니다. 환경 비밀, 실행기 토큰, 작업 주문과 같은 자체 호스팅 환경에 대해 발급된 다른 토큰은 동일한 키 세트로 서명되지만 다른 역할을 가집니다.
  </Step>

  <Step title="만료 확인">
    `exp`가 과거이면 토큰을 거부합니다. Anthropic은 기본적으로 4시간 수명, 최대 8시간으로 세션 토큰을 발급합니다. 실행기는 만료 전에 토큰을 새로 고치고 새 값을 세션으로 푸시하므로 Claude가 새로 고침 후 시작하는 하위 프로세스는 이를 상속합니다. 따라서 하나의 세션은 수명 동안 여러 개의 서로 다른 유효한 토큰을 서비스에 제시할 수 있습니다.
  </Step>

  <Step title="ID 읽기">
    생성 사용자의 ID는 `act` 클레임에 있습니다: `act.sub`는 접두사 형식 `user:<id>`의 Anthropic 사용자 ID이고, `act.email`은 생성 표면이 기록했을 때 이메일 주소입니다. 조직의 서비스 ID가 생성한 세션(Claude Tag 채널 세션 포함)은 대신 `act.sub`에서 `agent:` 주제를 가지므로, 세션을 사용자 생성으로만 취급하려면 `act.sub`이 `user:` 접두사를 가질 때, ID 클레임이 없는지 테스트하는 대신입니다. 전체 구조 및 평면 중복 클레임에 대해서는 [클레임 참조](#claims-reference)를 참고하세요.
  </Step>
</Steps>

검사는 표준 JWT 라이브러리에 직접 매핑됩니다. 아래 예제는 JWKS 가져오기, 캐싱 및 `kid` 선택을 처리하는 [`jose`](https://www.npmjs.com/package/jose)를 사용하는 Node.js와 [`PyJWT`](https://pyjwt.readthedocs.io/) 및 내장 JWKS 클라이언트를 사용하는 Python에서 전체 시퀀스를 구현합니다.

<Tabs>
  <Tab title="Node.js (jose)">
    ```typescript theme={null}
    import { createRemoteJWKSet, jwtVerify } from "jose";

    const JWKS = createRemoteJWKSet(
      new URL("https://api.anthropic.com/v1/code/.well-known/jwks.json")
    );

    const PREFIX = "sk-ant-cc-";
    const EXPECTED_POOL_ID = "ccpool_...";

    export async function verifySessionToken(raw: string) {
      if (!raw.startsWith(PREFIX)) {
        throw new Error("not a self-hosted runner session token");
      }
      const jwt = raw.slice(PREFIX.length);

      const { payload } = await jwtVerify(jwt, JWKS, {
        issuer: "ccr",
        audience: EXPECTED_POOL_ID,
        algorithms: ["ES256"],
      });

      if (payload["ccr:role"] !== "session_worker") {
        throw new Error("token is not a session_worker token");
      }

      const act = payload.act as { email?: string; sub?: string };
      return {
        sessionId: payload["ccr:session_id"] as string,
        poolId: payload["ccr:pool_id"] as string,
        orgId: payload["ccr:org_id"] as string,
        creatorEmail: act?.email,
        creatorSub: act?.sub,
      };
    }
    ```
  </Tab>

  <Tab title="Python (PyJWT)">
    ```python theme={null}
    import jwt
    from jwt import PyJWKClient

    JWKS_URL = "https://api.anthropic.com/v1/code/.well-known/jwks.json"
    PREFIX = "sk-ant-cc-"
    EXPECTED_POOL_ID = "ccpool_..."

    jwks = PyJWKClient(JWKS_URL)


    def verify_session_token(raw: str) -> dict:
        if not raw.startswith(PREFIX):
            raise ValueError("not a self-hosted runner session token")
        token = raw.removeprefix(PREFIX)

        signing_key = jwks.get_signing_key_from_jwt(token)
        payload = jwt.decode(
            token,
            signing_key.key,
            algorithms=["ES256"],
            issuer="ccr",
            audience=EXPECTED_POOL_ID,
        )

        if payload.get("ccr:role") != "session_worker":
            raise ValueError("token is not a session_worker token")

        act = payload.get("act") or {}
        return {
            "session_id": payload["ccr:session_id"],
            "pool_id": payload["ccr:pool_id"],
            "org_id": payload["ccr:org_id"],
            "creator_email": act.get("email"),
            "creator_sub": act.get("sub"),
        }
    ```
  </Tab>
</Tabs>

<h3 id="verify-the-token-inside-the-session">
  세션 내부에서 토큰 확인
</h3>

[래퍼 스크립트](/docs/ko/self-hosted-environments-configuration#wrapper-scripts)는 Claude가 시작하기 전에 세션 내부에서 실행됩니다. JWT 라이브러리를 호출하는 대신 실행기 바이너리의 `self-hosted-runner decode-token` 하위 명령을 실행할 수 있습니다. 하위 명령은 위치 인수, `CLAUDE_CODE_SESSION_ACCESS_TOKEN` 또는 파이프된 stdin에서 토큰을 읽고, 그 순서대로 접두사를 제거하고, JWKS 엔드포인트에 대해 서명을 확인하고, 만료를 확인하고, 클레임을 JSON으로 인쇄합니다. 하위 명령은 서명 및 만료 검사만 수행합니다. `iss`, `aud` 또는 `ccr:role`을 확인하지 않습니다. 래퍼의 인증 결정이 이러한 클레임에 따라 달라질 때 인쇄된 JSON에서 읽고 명시적으로 비교합니다.

이 명령은 생성자 ID를 추출하며, SSO 제공자의 주제를 선호하고, 그 다음 이메일 주소, 그 다음 생성자의 `act.sub` 주제(`user:<id>` 또는 `agent:<id>`)를 선호합니다:

```bash theme={null}
"$CLAUDE_RUNNER_CLAUDE_BIN" self-hosted-runner decode-token | jq -re '.act.attested_by.sub // .act.email // .act.sub'
```

래퍼는 `CLAUDE_RUNNER_CLAUDE_BIN`에서 실행기 자신의 바이너리에 대한 절대 경로를 받습니다. PATH 해석 `claude` 대신 해당 경로를 사용하여 디코드가 실행기 자체가 사용하는 동일한 바이너리에서 실행되도록 합니다.

`jq -r` 대신 `jq -re`를 사용하여 누락된 클레임이 0이 아닌 종료를 발생시킵니다. `-r`만 사용하면 누락된 클레임은 리터럴 문자열 `null`을 인쇄하고 0으로 종료되어 잘못된 값을 조용히 다운스트림으로 전달합니다. JWKS 엔드포인트에 도달할 수 없는 오프라인 검사의 경우에만 `decode-token`에 `--no-verify`를 전달합니다.

<h2 id="claims-reference">
  클레임 참조
</h2>

아래 표는 확인과 관련된 세션 토큰 클레임을 나열합니다. `ccr:*` 네임스페이스 및 `act` 체인에서 ID를 읽습니다. 평면 `account_email`, `organization_uuid` 및 `account_uuid` 클레임은 제거될 수 있는 하위 호환성 중복입니다. 조직의 서비스 ID가 생성한 세션(Claude Tag 채널 세션 포함)은 `act.sub`에서 `agent:` 주제를 가지며 `act.email`, `ccr:account_id`, `account_email` 및 `account_uuid`를 생략합니다. 두 이메일 클레임은 사용자 생성 세션에도 선택 사항입니다: Anthropic은 생성 요청의 자격증명이 이메일을 가질 때만 세션 생성 시 기록하며, CLI에서 발송된 세션은 둘 다 부족할 수 있으므로 이메일이 아닌 `act.sub` 또는 `ccr:account_id`에서 ID를 키합니다. 토큰은 이 표 이상의 추가 클레임을 가질 수도 있습니다. 인식하지 못하는 클레임을 무시합니다.

| 클레임                 | 유형               | 설명                                                                                                                                                                                                                                                                                                                           |
| :------------------ | :--------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `iss`               | string           | 항상 `ccr`.                                                                                                                                                                                                                                                                                                                    |
| `sub`               | string           | `ccr:session:<session_id>`.                                                                                                                                                                                                                                                                                                  |
| `aud`               | array of strings | 항상 `anthropic-api`를 포함합니다. 자체 호스팅 환경의 세션의 경우 배열은 `ccpool_...`과 같은 환경 ID도 포함합니다. `anthropic-api`가 아닌 환경 ID를 확인합니다.                                                                                                                                                                                                            |
| `exp`               | number           | Unix 타임스탬프로 만료. 4시간 기본 수명, 8시간 최대.                                                                                                                                                                                                                                                                                           |
| `iat`               | number           | Unix 타임스탬프로 발급 시간.                                                                                                                                                                                                                                                                                                           |
| `jti`               | string           | 고유 토큰 식별자.                                                                                                                                                                                                                                                                                                                   |
| `ccr:role`          | string           | 세션 토큰의 경우 항상 `session_worker`.                                                                                                                                                                                                                                                                                               |
| `ccr:session_id`    | string           | 세션 ID. `sub`의 접미사와 동일한 값.                                                                                                                                                                                                                                                                                                    |
| `ccr:pool_id`       | string           | 환경 ID. `aud`에 나타나는 동일한 값.                                                                                                                                                                                                                                                                                                    |
| `ccr:org_id`        | string           | Anthropic 조직 ID.                                                                                                                                                                                                                                                                                                             |
| `ccr:account_id`    | string           | 생성 사용자의 Anthropic 계정 ID: `act.sub`의 값에서 `user:` 접두사를 제거한 것, 태그된 `user_...` ID. [spawn-runner hook](/docs/ko/self-hosted-environments-configuration#the-spawn-runner-hook)의 `CLAUDE_RUNNER_ACCOUNT_ID`가 가지는 동일한 값 및 [`--lock-to-account`](/docs/ko/self-hosted-environments-reference#runner-cli-flags)가 수락하는 것이므로 세 개가 동일한 문자열로 비교됩니다. |
| `account_email`     | string           | `act.email`의 중복; `act.email`이 없을 때마다 없음.                                                                                                                                                                                                                                                                                     |
| `organization_uuid` | string           | Anthropic 조직 UUID.                                                                                                                                                                                                                                                                                                           |
| `account_uuid`      | string           | 생성 사용자의 Anthropic 계정 UUID.                                                                                                                                                                                                                                                                                                   |
| `act`               | object           | [RFC 8693](https://www.rfc-editor.org/rfc/rfc8693) 위임 체인. [The `act` chain](#the-act-chain)을 참고하세요.                                                                                                                                                                                                                          |

<h3 id="the-act-chain">
  `act` 체인
</h3>

`act` 클레임은 세션을 생성한 사용자 또는 서비스 ID에서 [환경](/docs/ko/self-hosted-environments#key-concepts)의 비밀을 허용한 실행기까지의 전체 위임 경로를 기록하고, 해당 비밀을 생성한 ID를 기록합니다. 생성자는 가장 바깥쪽 행위자이므로 `act.sub`은 직접 식별합니다.

| 경로                | 설명                                                                                                                                               |
| :---------------- | :----------------------------------------------------------------------------------------------------------------------------------------------- |
| `act.sub`         | 생성 사용자의 Anthropic 사용자 ID(형식: `user:<id>`) 또는 조직의 서비스 ID가 세션을 생성했을 때 `agent:<id>`(Claude Tag 채널 세션의 경우).                                          |
| `act.email`       | 생성 사용자의 이메일 주소(세션 생성 시 기록된 경우). 필수로 요구하지 마세요. `act.sub`에서 키합니다.                                                                                  |
| `act.attested_by` | 생성 사용자에 대한 업스트림 ID 제공자의 증명(사용 가능한 경우). `act.attested_by.sub`는 Google 또는 Okta와 같은 SSO 제공자가 발급한 주제입니다. 자신의 시스템에서 ID에 매핑할 때 `act.email`보다 이를 선호합니다. |
| `act.act`         | 세션을 생성한 실행기. `act.act.sub`는 `ccr:runner:<runner_id>`.                                                                                            |
| `act.act.act`     | 환경. `act.act.act.sub`는 `ccr:pool:<pool_id>`.                                                                                                     |
| `act.act.act.act` | 실행기가 등록한 환경 비밀을 생성한 ID. 체인이 여기서 끝납니다.                                                                                                            |

<h2 id="scope-derived-credentials">
  파생된 자격증명 범위 지정
</h2>

세션 토큰은 세션을 생성한 사용자 또는 서비스 ID를 식별하지만 이를 해당 생성자가 직접 로그인한 것과 동등하게 취급하지 마세요. 토큰은 세션 내부의 환경 변수에 있으므로 Claude가 실행하는 모든 코드와 세션이 시작하는 모든 도구 또는 MCP 서버가 이를 읽고 제시할 수 있습니다.

확인도 오프라인입니다: JWKS에 대해 확인되는 토큰은 `exp`까지 유효하며, 그 이후로 세션에 무슨 일이 일어났든 상관없이, Anthropic은 세션 토큰에 대한 해지 피드를 게시하지 않습니다. 토큰에서 파생된 모든 것을 그에 따라 바인딩합니다.

서비스가 토큰을 내부 자격증명으로 교환할 때 하나의 코딩 세션이 도달해야 하는 것으로 범위가 지정된 자격증명을 발급합니다:

* **기능 제한**: 세션이 코딩 작업에 필요한 리소스에 대한 읽기 및 쓰기 액세스를 부여하고, 생성자가 다른 곳에서 보유한 관리 기능은 부여하지 않습니다.
* **수명 제한**: 파생된 자격증명을 토큰의 `exp` 또는 더 짧은 시간으로 바인딩합니다.
* **세션으로 감사**: 생성자 ID와 함께 `ccr:session_id` 및 `jti`를 기록하여 작업을 특정 세션으로 추적할 수 있습니다.

<h2 id="related-environment-variables">
  관련 환경 변수
</h2>

생성자 ID는 토큰을 확인하지 않는 두 표면의 평문 환경 변수에도 나타납니다:

* **[`spawn-runner` hook](/docs/ko/self-hosted-environments-configuration#the-spawn-runner-hook), 오케스트레이터에서**: 훅은 대기 중인 세션에 대한 실행기가 존재하기 전에 실행되며 `CLAUDE_RUNNER_ACCOUNT_EMAIL` 및 `CLAUDE_RUNNER_ACCOUNT_ID`와 같은 변수에서 생성자 ID를 받습니다. 오케스트레이터는 작업 주문에서 읽으며, 작업 주문 자체의 서명을 확인하지 않고 하나의 실행기를 생성하도록 승인하는 서명된 일회용 토큰입니다. 클레임은 작업 주문이 환경 비밀이 인증하는 Anthropic에 대한 오케스트레이터의 연결을 통해 도착하기 때문에 신뢰됩니다.
* **[래퍼 스크립트](/docs/ko/self-hosted-environments-configuration#wrapper-scripts), 세션 내부**: 래퍼는 `CCR_SESSION_ACCOUNT_EMAIL`을 받으며, 서명 확인 없이 토큰에서 미리 추출된 생성자의 이메일입니다. 변수는 커밋 트레일러와 같은 레이블 지정에 적합하며 인증 결정에는 적합하지 않습니다.

오케스트레이터 측 결정(예: 머신 이미지 선택)에는 평문 변수를 사용합니다. 다운스트림 서비스가 실행기의 환경을 신뢰하는 대신 독립적인 암호화 증명이 필요할 때 `CLAUDE_CODE_SESSION_ACCESS_TOKEN`을 사용합니다.

<h2 id="what’s-next">
  다음 단계
</h2>

* [자체 호스팅 환경](/docs/ko/self-hosted-environments): 환경, 실행기 및 세션 모델; [빠른 시작](/docs/ko/self-hosted-environments-quickstart) 및 [프로덕션에 배포](/docs/ko/self-hosted-environments-deploy)는 설정 및 운영을 포함합니다.
* [세션 사용자 정의](/docs/ko/self-hosted-environments-configuration): 토큰을 사용하는 래퍼 스크립트 및 `spawn-runner` hook
* [참조](/docs/ko/self-hosted-environments-reference): CLI 플래그, 환경 변수 및 메트릭
