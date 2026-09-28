> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 엔터프라이즈 네트워크 구성

> 프록시 서버, 사용자 정의 인증 기관(CA), 상호 전송 계층 보안(mTLS) 인증을 통해 엔터프라이즈 환경에서 Claude Code를 구성합니다.

Claude Code는 환경 변수를 통해 다양한 엔터프라이즈 네트워크 및 보안 구성을 지원합니다. 여기에는 회사 프록시 서버를 통한 트래픽 라우팅, 사용자 정의 인증 기관(CA) 신뢰, 향상된 보안을 위한 상호 전송 계층 보안(mTLS) 인증서를 사용한 인증이 포함됩니다.

Claude Code를 시작하기 전에 이러한 환경 변수를 설정합니다. 셸에서 내보낸 변수는 시작 시 한 번만 읽혀지므로, 실행 중인 세션은 나중에 셸 환경에 대한 변경 사항을 선택하지 않습니다.

<Note>
  이 페이지에 표시된 모든 환경 변수는 [`settings.json`](/docs/ko/settings)에서도 구성할 수 있습니다.
</Note>

<h2 id="proxy-configuration">
  프록시 구성
</h2>

<h3 id="environment-variables">
  환경 변수
</h3>

Claude Code는 표준 프록시 환경 변수를 준수합니다. Claude Desktop 세션에서 앱이 공급자 연결을 관리하는 경우, Claude Code는 관리되는 설정 및 `~/.claude/settings.json`에서만 읽습니다. 범위 규칙은 [mTLS 인증](#mtls-authentication)을 참조하십시오.

```bash theme={null}
# HTTPS 프록시 (권장)
export HTTPS_PROXY=https://proxy.example.com:8080

# HTTP 프록시 (HTTPS를 사용할 수 없는 경우)
export HTTP_PROXY=http://proxy.example.com:8080

# 특정 요청에 대해 프록시 우회 - 공백으로 구분된 형식
export NO_PROXY="localhost 192.168.1.1 example.com .example.com"
# 특정 요청에 대해 프록시 우회 - 쉼표로 구분된 형식
export NO_PROXY="localhost,192.168.1.1,example.com,.example.com"
# 모든 요청에 대해 프록시 우회
export NO_PROXY="*"
```

소문자 변형도 작동하며, Claude Code는 `https_proxy`, `HTTPS_PROXY`, `http_proxy`, `HTTP_PROXY` 순서로 설정된 첫 번째 변수를 사용합니다.

Claude Code는 `localhost`, `::1` 또는 `127.0.0.0/8`에 대한 WebSocket 연결을 프록시를 통해 보내지 않으므로 `NO_PROXY`에 루프백 항목을 추가할 필요가 없습니다.

<Note>
  Claude Code는 SOCKS 프록시를 지원하지 않습니다.
</Note>

<h3 id="basic-authentication">
  기본 인증
</h3>

프록시에 기본 인증이 필요한 경우 프록시 URL에 자격 증명을 포함합니다:

```bash theme={null}
export HTTPS_PROXY=http://username:password@proxy.example.com:8080
```

<Warning>
  스크립트에 암호를 하드코딩하지 마십시오. 대신 환경 변수 또는 보안 자격 증명 저장소를 사용하십시오.
</Warning>

<Tip>
  고급 인증(NTLM, Kerberos 등)이 필요한 프록시의 경우 인증 방법을 지원하는 LLM Gateway 서비스 사용을 고려하십시오.
</Tip>

<h2 id="ca-certificate-store">
  CA 인증서 저장소
</h2>

기본적으로 Claude Code는 번들로 제공되는 Mozilla CA 인증서와 운영 체제의 인증서 저장소를 모두 신뢰합니다. OS 저장소를 읽으려면 `tls.getCACertificates`가 있는 런타임이 필요합니다. 네이티브 설치 프로그램은 항상 이를 포함하고 있으며, npm 설치는 Node 22.15 이상이 필요합니다. 이전 Node 버전에서는 번들로 제공되는 세트와 `NODE_EXTRA_CA_CERTS`만 적용됩니다. 엔터프라이즈 TLS 검사 프록시는 루트 인증서가 OS 신뢰 저장소에 설치되어 있고 런타임이 이를 읽을 수 있을 때 추가 구성 없이 작동합니다.

`CLAUDE_CODE_CERT_STORE`는 쉼표로 구분된 소스 목록을 허용합니다. 인식되는 값은 Claude Code와 함께 제공되는 Mozilla CA 세트의 경우 `bundled`, 운영 체제 신뢰 저장소의 경우 `system`입니다. 기본값은 `bundled,system`입니다.

번들로 제공되는 Mozilla CA 세트만 신뢰하려면:

```bash theme={null}
export CLAUDE_CODE_CERT_STORE=bundled
```

OS 인증서 저장소만 신뢰하려면:

```bash theme={null}
export CLAUDE_CODE_CERT_STORE=system
```

<Note>
  `CLAUDE_CODE_CERT_STORE`에는 전용 `settings.json` 스키마 키가 없습니다. `~/.claude/settings.json`의 `env` 블록에서 또는 프로세스 환경에서 직접 설정합니다.
</Note>

<h2 id="custom-ca-certificates">
  사용자 정의 CA 인증서
</h2>

엔터프라이즈 환경에서 사용자 정의 CA를 사용하는 경우 Claude Code를 구성하여 이를 직접 신뢰하도록 합니다:

```bash theme={null}
export NODE_EXTRA_CA_CERTS=/path/to/ca-cert.pem
```

<h2 id="mtls-authentication">
  mTLS 인증
</h2>

엔터프라이즈 환경에서 클라이언트 인증서 인증이 필요한 경우:

```bash theme={null}
# 인증용 클라이언트 인증서
export CLAUDE_CODE_CLIENT_CERT=/path/to/client-cert.pem

# 클라이언트 개인 키
export CLAUDE_CODE_CLIENT_KEY=/path/to/client-key.pem

# 선택 사항: 암호화된 개인 키의 암호
export CLAUDE_CODE_CLIENT_KEY_PASSPHRASE="your-passphrase"
```

Claude Code는 시작 시 인증서 및 키 파일을 읽고, [관리되는 설정](/docs/ko/server-managed-settings)의 `env` 블록을 조직에서 세션 중간에 변경할 때와 같이 설정을 적용할 때마다 다시 읽습니다.

인증서 및 키를 회전하려면 동일한 경로의 파일을 교체합니다. Claude Code는 재시작 없이 실행 중인 세션에서 교체를 선택합니다. API 요청이 연결 재설정 또는 TLS 핸드셰이크 오류와 같은 연결 수준 오류로 실패하면 두 파일을 다시 읽고 새 쌍으로 요청을 다시 시도합니다. v2.1.232 이전에는 Claude Code가 연결 오류 시 다시 읽지 않았으므로 다음 설정을 적용하거나 재시작할 때까지 이미 로드된 쌍을 유지했습니다.

Claude Code는 파일 변경을 감시하지 않고 실패한 요청에 응답하여 파일을 다시 읽습니다:

* **타이밍**: Claude Code는 파일을 교체하는 순간 아무것도 하지 않습니다. 적격 실패 후 재시도 시 또는 설정을 적용한 후 다음 요청 중 먼저 오는 것에서 새 쌍을 제시합니다.
* **게이트웨이 거부**: Claude Code는 게이트웨이가 이전 쌍을 더 이상 수락하지 않은 후 연결을 재설정하거나 TLS 핸드셰이크를 거부할 때 다시 읽습니다. 게이트웨이가 핸드셰이크를 완료하고 HTTP 오류로 응답할 때는 다시 읽지 않습니다. 이 경우 Claude Code는 다음 설정을 적용할 때 또는 재시작할 때 새 쌍을 로드합니다.
* **부분 작성 회전**: Claude Code가 회전이 진행 중일 때 다시 읽을 때, 예를 들어 서로 일치하지 않는 인증서 및 키를 읽을 때 이전 쌍을 유지하고 다음 실패 시 다시 읽습니다.
* **OTLP 텔레메트리 내보내기**: Claude Code는 [내보내기](/docs/ko/monitoring-usage#mtls-authentication)가 처음 사용할 때 로드한 인증서를 유지하므로 회전된 인증서가 텔레메트리 수집기에 도달하려면 Claude Code를 재시작합니다.
* **다시 로드 끄기**: [`CLAUDE_CODE_DISABLE_MTLS_RELOAD_ON_STALE_CONNECTION=1`](/docs/ko/env-vars#variables)을 설정하여 연결 오류 다시 읽기를 끕니다. Claude Code는 다음 설정을 적용할 때 또는 다음 시작 시에만 회전된 파일을 선택합니다.

Claude Code가 회전을 선택했는지 확인하려면 [디버그 로깅으로 세션을 시작](#verify-your-configuration)하고 로그에서 `Stale connection — reloaded rotated mTLS client material`을 찾습니다. Claude Code는 대신 설정을 적용하는 동안 회전을 선택할 때 이 줄을 기록하지 않으므로 누락된 줄만으로는 회전이 실패했다는 의미가 아닙니다.

Claude Code가 다음 시작 시 이미 만료된 쌍을 로드하지 않도록 현재 쌍이 만료되기 전에 파일을 교체합니다.

[클라우드 세션](/docs/ko/claude-code-on-the-web)에서 호스팅 환경은 API에 대한 연결을 관리하므로 Claude Code는 설정 파일 `env` 블록에서 오는 다음 변수를 무시합니다:

* `CLAUDE_CODE_CLIENT_CERT`
* `CLAUDE_CODE_CLIENT_KEY`
* `CLAUDE_CODE_CLIENT_KEY_PASSPHRASE`
* `NODE_EXTRA_CA_CERTS`
* `NODE_TLS_REJECT_UNAUTHORIZED`
* `CLAUDE_CODE_OAUTH_SCOPES`

Claude Code는 세션의 디버그 로그에서 무시된 각 키를 기록합니다.

앱이 공급자 연결을 관리하는 [Claude Desktop](/docs/ko/desktop) 세션에서, 예를 들어 [타사 공급자](/docs/ko/third-party-integrations)의 Code 탭 및 Cowork 세션에서 Claude Code는 이러한 변수와 프록시 변수 `HTTP_PROXY`, `HTTPS_PROXY`, 및 `NO_PROXY`를 [관리되는 설정](/docs/ko/managed-settings) 및 `~/.claude/settings.json`에서만 읽습니다: 저장소의 자체 설정 파일에서는 무시하므로 체크아웃된 저장소는 앱에서 자격 증명이 오는 세션의 TLS 또는 프록시 경로를 리디렉션할 수 없습니다. claude.ai를 통해 로그인한 로컬, SSH 또는 WSL Code 탭 세션에서 앱은 연결을 관리하지 않으며 Claude Code는 모든 터미널 세션처럼 모든 설정 범위에서 이러한 변수를 읽습니다; [클라우드 세션](/docs/ko/claude-code-on-the-web)은 시작 위치에 관계없이 위의 클라우드 세션 규칙을 따릅니다. v2.1.217 이전에는 앱이 연결을 관리할 때 Claude Code가 모든 설정 파일에서 이러한 변수를 무시했습니다.

<h2 id="verify-your-configuration">
  구성 확인
</h2>

일반적으로 잘못된 프록시 주소나 잘못된 인증서 경로는 Claude Code가 이러한 설정을 읽을 때 대부분의 설정을 검증하지 않으므로 나중의 요청에서 [연결 또는 인증서 오류](/docs/ko/errors#network-and-connection-errors)로 알게 됩니다. 시작 시 확인하는 유일한 설정은 프록시 URL입니다. Claude Code가 `http://` 스킴이 없는 것처럼 값을 구문 분석할 수 없으면 Claude Code는 수정할 변수의 이름을 지정하는 오류와 함께 시작을 중지합니다.

요청을 보내기 전에 구성이 로드되었는지 확인하려면 디버그 로깅을 사용하여 Claude Code를 시작하세요:

```bash theme={null}
claude --debug
```

디버그 출력은 터미널이 아닌 `~/.claude/debug/<session-id>.txt`로 이동하거나 `--debug-file <path>`로 설정한 경로로 이동합니다. 로그에서 각 파일이 로드되었음을 확인하는 줄을 찾으세요:

```text theme={null}
CA certs: Appended extra certificates from NODE_EXTRA_CA_CERTS (/etc/ssl/certs/corp-ca.pem)
mTLS: Loaded client certificate from CLAUDE_CODE_CLIENT_CERT
mTLS: Loaded client key from CLAUDE_CODE_CLIENT_KEY
```

Claude Code가 이러한 파일 중 하나를 읽을 수 없으면 로그에 대신 이유와 함께 `Failed to read` 또는 `Failed to load` 줄이 표시됩니다.

대화형 세션에서 `/status`를 실행하고 다음 행을 확인할 수도 있습니다:

* **Proxy**: 활성 프록시 URL을 표시하고 구문 분석할 수 없는 값을 유효하지 않음으로 표시하고 무시합니다.
* **mTLS client cert** 및 **mTLS client key**: 파일이 로드되었을 때만 나타나므로 행이 없으면 로드가 실패했으며 디버그 로그에 이유가 있습니다.
* **Additional CA cert(s)**: 파일이 로드되었는지 확인하지 않고 `NODE_EXTRA_CA_CERTS` 경로를 표시하므로 디버그 로그에서 이를 확인하세요.

<h2 id="apply-network-settings-to-background-agents">
  백그라운드 에이전트에 네트워크 설정 적용
</h2>

[백그라운드 에이전트](/docs/ko/agent-view)는 이를 실행한 터미널 내부에서 실행되지 않습니다. 사용자별 감시자 프로세스가 필요에 따라 시작되어 셸보다 오래 유지되며 모든 `claude agents`, `--bg`, `/background` 세션을 호스팅합니다. [백그라운드 세션이 호스팅되는 방식](/docs/ko/agent-view#how-background-sessions-are-hosted)을 참조하십시오. 이는 이 페이지의 구성이 해당 세션에 도달하는 방식을 변경합니다.

<h3 id="set-network-variables-in-settings-not-the-shell">
  셸이 아닌 설정에서 네트워크 변수 설정
</h3>

감시자는 모든 터미널에서 공유하는 하나의 프로세스입니다. 이는 먼저 시작하는 셸의 환경을 상속하며, OS에 설치된 감시자는 셸 환경을 전혀 받지 않습니다. 프록시, CA 경로 또는 mTLS 변수를 셸에서만 내보내면 해당 셸이 감시자를 콜드 스타트했을 때는 백그라운드 에이전트에 도달하지만, 다른 셸이 시작했을 때는 조용히 도달하지 않습니다.

대신 `~/.claude/settings.json`의 `env` 블록 또는 [관리되는 설정](/docs/ko/settings)에 동일한 변수를 입력하십시오. 이 페이지의 모든 변수를 여기에 설정할 수 있으며, 설정은 모든 머신의 모든 백그라운드 세션에 도달하는 유일한 구성입니다.

<h3 id="configure-a-corporate-launcher-as-a-setting">
  기업 런처를 설정으로 구성
</h3>

일부 조직에서는 모든 Claude Code 프로세스가 샌드박싱, 네트워크 제어 또는 자격 증명 주입을 적용하는 기업 런처를 통해 시작되어야 합니다. 감시자와 그 워커는 `PATH`에서 `claude`를 조회하는 대신 고정된 경로에서 Claude Code를 시작하므로 모든 백그라운드 에이전트는 `PATH`의 앞에 배치한 래퍼를 우회합니다.

[`processWrapper`](/docs/ko/settings-reference#processwrapper) 설정을 감시자, 그 워커 및 [런처가 포함하는 항목](/docs/ko/corporate-launcher#what-the-launcher-covers)에 나열된 다른 백그라운드 프로세스 앞에 런처를 붙이도록 설정하십시오. 동등한 [`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/ko/env-vars) 환경 변수는 둘 다 설정되었을 때 우선하며, 동일한 규칙이 적용됩니다. 셸 내보내기가 아닌 관리되는 설정 또는 `~/.claude/settings.json`을 통해 전달하십시오. [기업 런처 뒤에서 Claude Code 실행](/docs/ko/corporate-launcher)은 런처가 만족해야 하는 계약, 도달하는 항목과 도달하지 않는 항목, 그리고 배포 방법을 다룹니다.

<Note>
  이미 실행 중인 감시자는 시작할 때 사용한 시작 구성을 유지합니다. 런처 설정을 배포한 후 [`claude daemon stop --any`](/docs/ko/agent-view#the-supervisor-process)를 실행하여 다음 `claude agents` 또는 `--bg`가 이를 준수하는 감시자를 시작하도록 하십시오. 설치된 서비스는 `--any` 없이 `claude daemon stop`을 사용합니다.
</Note>

<h2 id="streaming-idle-watchdogs">
  스트리밍 유휴 감시견
</h2>

Claude Code는 네 개의 독립적인 타이머를 실행하여 스트리밍 모델 응답이 조용해지면 중단하므로, 연결이 끊어지면 실패하고 재시도하며 중단되지 않습니다. 첫 바이트 마감 시간은 응답 헤더를 기다리는 동안 응답이 도착하기 전을 포함합니다. 다른 세 개의 타이머는 각각 다른 신호에 대해 라이브 응답을 감시합니다.

| 타이머         | 중단 조건                                                                                                        | 실행 위치                                                                                                                                                                                                                                                                                                         | 기본 시간 초과                                                |
| :---------- | :----------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------ |
| 첫 바이트 마감 시간 | Claude Code가 요청을 보낸 후 응답 헤더가 도착하지 않음                                                                         | 직접 Anthropic API 및 [AWS의 Claude Platform](/docs/ko/claude-platform-on-aws) (HTTPS 프록시 포함), 단 `ANTHROPIC_BASE_URL` 또는 `ANTHROPIC_AWS_BASE_URL`이 [게이트웨이](/docs/ko/gateways)를 통해 라우팅하는 경우는 제외. Amazon Bedrock에서 `CLAUDE_ENABLE_BYTE_WATCHDOG_BEDROCK=1`로 옵트인; Google Cloud의 Agent Platform 또는 Microsoft Foundry에서는 실행되지 않음 | 직접 Anthropic API에서 180초, 다른 곳에서 300초, 요청 본문 32KB당 1초 추가 |
| 이벤트 수준 감시견  | 응답 이벤트가 파싱되지 않음. 바이트 수준 감시견이 실행되는 연결에서는 도착한 바이트(keep-alive ping 포함)도 이 감시견을 재설정하며, 파싱된 이벤트가 없는 상태로 약 5분까지 지속 | 모든 제공자                                                                                                                                                                                                                                                                                                        | 300초                                                    |
| 바이트 수준 감시견  | 와이어에 바이트가 도착하지 않음 (SSE keep-alive ping 포함)                                                                   | 직접 Anthropic API, [AWS의 Claude Platform](/docs/ko/claude-platform-on-aws), 및 [게이트웨이](/docs/ko/gateways) 연결 (사용자 정의 `ANTHROPIC_BASE_URL` 포함). Amazon Bedrock `vnd.amazon.eventstream` 응답에서 `CLAUDE_ENABLE_BYTE_WATCHDOG_BEDROCK=1`로 옵트인; Google Cloud의 Agent Platform 또는 Microsoft Foundry에서는 실행되지 않음                    | 직접 Anthropic API에서 180초, 다른 곳에서 300초                    |
| 본문 유휴 시간 초과 | 5분 동안 바이트가 도착하지 않음                                                                                           | 직접 Anthropic API 및 AWS의 Claude Platform을 제외한 제공자 (단, [`API_FORCE_IDLE_TIMEOUT`](/docs/ko/env-vars)이 변경하지 않는 한)                                                                                                                                                                                                     | 5분                                                      |

이 변수들로 타이머를 구성하며, 각각은 [환경 변수 참조](/docs/ko/env-vars)에서 자세히 설명합니다:

* `CLAUDE_ENABLE_STREAM_WATCHDOG` 및 `CLAUDE_ENABLE_BYTE_WATCHDOG`는 해당 감시견을 `1`로 강제 활성화하거나 `0`으로 강제 비활성화하며, 테이블에 나열된 연결 내에서만 작동합니다. 어느 변수도 감시견을 적용되지 않는 연결 유형으로 확장하지 않습니다. `CLAUDE_ENABLE_BYTE_WATCHDOG`를 `0`으로 설정하면 첫 바이트 마감 시간도 비활성화됩니다.
* `CLAUDE_STREAM_IDLE_TIMEOUT_MS`는 두 감시견의 시간 초과를 설정합니다. Claude Code는 5분 미만의 값을 5분으로 올리고, 바이트 수준 감시견의 값을 30분으로 제한합니다.
* `CLAUDE_BYTE_STREAM_IDLE_TIMEOUT_MS`는 이벤트 수준 감시견을 변경하지 않고 바이트 수준 감시견의 시간 초과를 설정하며, 10초에서 30분 사이로 제한되고, 해당 감시견에 대해 `CLAUDE_STREAM_IDLE_TIMEOUT_MS`보다 우선합니다.
* `CLAUDE_STREAM_FIRST_BYTE_TIMEOUT_MS`는 첫 바이트 마감 시간을 직접 설정합니다. 설정하지 않으면 Claude Code는 바이트 수준 감시견의 시간 초과를 사용하므로, `CLAUDE_STREAM_IDLE_TIMEOUT_MS` 및 `CLAUDE_BYTE_STREAM_IDLE_TIMEOUT_MS`도 마감 시간을 변경합니다. 제한, 업로드 허용량, `API_TIMEOUT_MS` 상한, 응답 없음 중단 후 재시도 대기 시간에 대해서는 [API에서 응답 없음](/docs/ko/errors#no-response-from-api)을 참조하세요.
* `API_FORCE_IDLE_TIMEOUT`을 `0`으로 설정하면 본문 유휴 시간 초과가 비활성화되고, `1`로 설정하면 모든 제공자에 대해 활성화됩니다. 감시견은 독립적으로 실행되므로, 스트림이 해당 임계값보다 오래 일시 중지되도록 하려면 감시견도 올리거나 비활성화하세요.

감시견이 정체된 스트림을 중단하면, Claude Code는 중단을 스트림 중간 실패로 취급하며, 표시되는 내용은 응답이 얼마나 진행되었는지에 따라 달라집니다. Claude Code는 요청을 재시도하거나 오류로 턴을 종료하고, 완료된 출력을 유지하며 [불완전한 응답 알림](/docs/ko/errors#the-response-above-may-be-incomplete)을 표시하거나, 턴을 정상적으로 종료합니다. [자동 재시도](/docs/ko/errors#automatic-retries)는 각 결과가 적용되는 위치를 설명합니다.

[비대화형 세션](/docs/ko/headless)에서 및 모든 세션에서 서브에이전트의 응답에 대해, Claude Code는 먼저 Claude에 잘린 응답을 계속하도록 프롬프트할 수 있습니다. [해당 알림의 항목](/docs/ko/errors#the-response-above-may-be-incomplete)은 언제 이를 수행하는지 및 언제 여전히 알림을 표시하는지를 설명합니다.

첫 바이트 마감 시간이 발생하면 응답이 시작되지 않았으므로 유지할 부분 출력이 없습니다. Claude Code가 요청을 다시 보내는 방법 및 턴이 대신 종료되는 시기에 대해서는 [API에서 응답 없음](/docs/ko/errors#no-response-from-api)을 참조하세요.

<h2 id="network-access-requirements">
  네트워크 액세스 요구사항
</h2>

Claude Code는 다음 URL에 대한 액세스가 필요합니다. 특히 컨테이너화되거나 제한된 네트워크 환경에서 프록시 구성 및 방화벽 규칙에 이러한 URL을 허용 목록에 추가하십시오. 첫 실행 설정 연결 확인은 `api.anthropic.com` 또는 `platform.claude.com`에 도달할 수 없을 때 여기를 가리킵니다. 확인 메시지 및 복구 단계는 [Anthropic 서비스에 연결할 수 없음](/docs/ko/errors#unable-to-connect-to-anthropic-services)을 참조하십시오.

| URL                                  | 필요한 용도                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `api.anthropic.com`                  | Claude API 요청, WebFetch [도메인 안전 확인](/docs/ko/data-usage#webfetch-domain-safety-check), 기능 플래그 가져오기 및 원격 분석 이벤트 로깅 포함                                                                                                                                                                                                                                                                                                                                                                             |
| `claude.ai`                          | claude.ai 계정 인증                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `claude.com`                         | claude.ai 계정 로그인이 브라우저에서 `claude.com` 페이지를 열고, 이는 `claude.ai`로 리디렉션됩니다. 사전 승인된 WebFetch 문서 조회도 CLI에서 이 호스트에 도달합니다                                                                                                                                                                                                                                                                                                                                                                           |
| `platform.claude.com`                | Anthropic Console 계정 인증. OAuth 토큰 교환, 새로 고침 및 취소도 claude.ai 계정의 경우 이 호스트로 이동하므로 Console 및 claude.ai 로그인 모두 필요합니다                                                                                                                                                                                                                                                                                                                                                                            |
| `mcp-proxy.anthropic.com`            | [claude.ai의 MCP 커넥터](/docs/ko/mcp#use-mcp-servers-from-claude-ai), 조직 관리자가 구성하는 커넥터 포함. 커넥터 트래픽은 이 프록시를 통해 라우팅되며, claude.ai 인증 사용자의 경우 기본적으로 활성화됩니다. Claude Code가 커넥터를 가져오지 않도록 하려면 [`ENABLE_CLAUDEAI_MCP_SERVERS=false`](/docs/ko/env-vars)를 설정하거나 [`disableClaudeAiConnectors`](/docs/ko/settings-reference#disableclaudeaiconnectors) 설정을 사용하십시오                                                                                                                                                        |
| `downloads.claude.ai`                | 플러그인 실행 파일 다운로드, 네이티브 설치 프로그램, 네이티브 자동 업데이터 및 업데이트 버전 확인                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `storage.googleapis.com`             | `/plugin`에 표시되는 플러그인 설치 횟수 및 메타데이터                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `storage.googleapis.com`             | 2.1.116 이전 버전의 네이티브 설치 프로그램 및 네이티브 자동 업데이터                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `registry.npmjs.org`                 | 플러그인 설치(npm 소스 플러그인 패키지 가져오기 및 플러그인의 Node.js 패키지 종속성 설치), `npx` 실행 MCP 서버 및 Claude Code 자체의 npm 및 bun 설치를 위한 패키지 레지스트리                                                                                                                                                                                                                                                                                                                                                                      |
| `bridge.claudeusercontent.com`       | [Chrome의 Claude](/docs/ko/chrome) 확장 프로그램 WebSocket 브리지                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `*.frame.claudeusercontent.com`      | [Artifact](/docs/ko/artifacts) 콘텐츠 읽기. CLI는 Claude가 Artifact를 열 때 이 호스트에서 Artifact의 파일을 가져오며, Artifact 도구가 계정에 [사용 가능](/docs/ko/artifacts#availability)할 때만 가져옵니다. 도구를 끄고 이 요구사항을 제거하려면 [`"enableArtifact": false`](/docs/ko/settings-reference#enableartifact) 또는 [`CLAUDE_CODE_DISABLE_ARTIFACT=1`](/docs/ko/env-vars)을 설정하십시오. Claude Code는 더 이상 사용되지 않는 [`disableArtifact`](/docs/ko/settings-reference#disableartifact) 설정도 준수합니다. 이러한 설정이 상호 작용하는 방식은 [Artifact 비활성화](/docs/ko/artifacts#disable-artifacts)를 참조하십시오 |
| `github.com`                         | GitHub 호스팅 [플러그인 마켓플레이스](/docs/ko/plugins/overview) 및 플러그인 복제, 공식 Anthropic 마켓플레이스 포함, HTTPS 또는 SSH를 통해. GitHub `owner/repo` 소스를 HTTPS를 통해서만 복제하려면 [`CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1`](/docs/ko/env-vars)을 설정하십시오                                                                                                                                                                                                                                                                                   |
| `raw.githubusercontent.com`          | [`/release-notes`](/docs/ko/commands)에 대한 변경 로그 피드. 대화형 세션에서 Claude Code는 캐시된 변경 로그가 실행 중인 버전을 아직 다루지 않을 때(예: 업데이트 후 첫 시작) 시작 시 백그라운드에서도 가져옵니다. 비대화형 및 클라우드 세션은 절대 가져오지 않습니다                                                                                                                                                                                                                                                                                                                     |
| `*-review.googlesource.com`          | `googlesource.com` 체크아웃에서 Gerrit 변경 조회. Claude Desktop Code 탭 세션이 `origin`이 `googlesource.com` 호스트인 [신뢰할 수 있는](/docs/ko/permissions#project-allow-rules-and-workspace-trust) 체크아웃에서 시작되거나 재개될 때, Claude Code는 HEAD의 `Change-Id`와 일치하는 열린 변경에 대해 해당 호스트의 `-review` 서버에 익명으로 한 번 요청합니다. 다른 세션 유형은 조회를 건너뛰며, 다른 Gerrit 호스트는 연결되지 않습니다. 선택 사항: [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/ko/env-vars)으로 비활성화                                                                                     |
| `http-intake.logs.us5.datadoghq.com` | 운영 원격 분석 이벤트, CLI가 Anthropic API를 직접 사용할 때만 전송되며, Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry의 경우는 절대 전송되지 않습니다. 선택 사항: [`DISABLE_TELEMETRY`](/docs/ko/data-usage#telemetry-services) 또는 `DO_NOT_TRACK`으로 비활성화                                                                                                                                                                                                                                                             |
| `browser-intake-us5-datadoghq.com`   | CLI가 Anthropic API를 직접 사용하고 서버 측 롤아웃 게이트가 활성화할 때 전송되는 운영 오류 보고서. 선택 사항: `DISABLE_ERROR_REPORTING` 또는 `DISABLE_TELEMETRY`로 비활성화. [원격 분석 서비스](/docs/ko/data-usage#telemetry-services)를 참조하십시오                                                                                                                                                                                                                                                                                                      |
| `formulae.brew.sh`                   | Homebrew 설치에서 버전 확인을 업데이트합니다. 다른 설치 방법은 이 호스트에 연결하지 않습니다                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `code.claude.com`                    | 기본 제공 claude-code-guide 에이전트 및 사전 승인된 WebFetch 요청에 의한 Claude Code 문서 조회. 이 호스트를 차단하면 문서 조회에만 영향을 미칩니다                                                                                                                                                                                                                                                                                                                                                                                       |

npm을 통해 Claude Code를 설치하거나 자신의 바이너리 배포를 관리하는 경우, 최종 사용자는 네이티브 설치 프로그램의 `downloads.claude.ai` 및 자동 업데이터 사용이 필요하지 않지만, npm 및 bun 설치는 조직이 미러링하지 않는 한 패키지 레지스트리인 `registry.npmjs.org`가 필요합니다. 표의 다른 사용은 설치 방법에 관계없이 적용됩니다.

두 개의 Datadog 수집 호스트는 선택적 운영 원격 분석만 전달하며, [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/ko/env-vars)을 설정하면 둘 다 비활성화됩니다. 타사 제공자의 세션은 플랫폼이 [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/ko/env-vars)를 설정하고 원격 분석 메트릭이 기본적으로 켜져 있더라도 이러한 호스트로 절대 전송하지 않습니다. Claude Code가 전송하는 모든 것과 허용 목록을 최종화하기 전에 비활성화하는 방법은 [원격 분석 서비스](/docs/ko/data-usage#telemetry-services)를 참조하십시오.

[Amazon Bedrock](/docs/ko/amazon-bedrock), [Google Cloud의 Agent Platform](/docs/ko/google-vertex-ai), [Microsoft Foundry](/docs/ko/microsoft-foundry) 또는 로그인한 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway) 세션을 사용할 때, 모델 트래픽 및 인증은 `api.anthropic.com`, `claude.ai` 또는 `platform.claude.com` 대신 제공자 또는 게이트웨이로 이동합니다. WebFetch 도구는 [설정](/docs/ko/settings)에서 `skipWebFetchPreflight: true`를 설정하지 않는 한 여전히 [도메인 안전 확인](/docs/ko/data-usage#webfetch-domain-safety-check)을 위해 `api.anthropic.com`을 호출합니다.

[`ANTHROPIC_BASE_URL`](/docs/ko/llm-gateway-connect#set-the-base-url-and-credential)을 사용하여 [LLM 게이트웨이](/docs/ko/llm-gateway)를 통해 라우팅할 때, [빠른 모드](/docs/ko/fast-mode) 가용성 확인은 게이트웨이 기본 URL 대신 `api.anthropic.com`을 호출합니다. 확인은 구성된 HTTP 프록시를 준수하므로 네트워크 블록이 원인인 경우 프록시의 `api.anthropic.com`에 대한 허용 목록 항목이 해결책입니다. 네트워크 블록은 호스트가 프록시를 통해서도 도달할 수 없는 경우에만 확인에 실패하며, 빠른 모드는 연결 오류를 보고합니다. 확인이 Anthropic이 거부하는 게이트웨이 발급 자격 증명을 제시할 때도 동일한 연결 오류가 나타납니다. 아무것도 차단되지 않으므로 허용 목록이 도움이 되지 않습니다. 빠른 모드를 복원하는 변수는 [프록시 및 LLM 게이트웨이 뒤에서 빠른 모드 사용](/docs/ko/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways)을 참조하십시오.

<h3 id="organization-ip-allowlists-and-proxy-egress">
  조직 IP 허용 목록 및 프록시 송신
</h3>

조직에 Claude에 대한 [IP 허용 목록](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting)이 활성화되어 있는 경우, `bridge.claudeusercontent.com`을 `claude.ai` 및 `api.anthropic.com`과 동일한 프록시 송신을 통해 라우팅하십시오. 예를 들어 동일한 Zscaler 앱 세그먼트 또는 Netskope 스티어링 정책에 배치하십시오. 그렇게 할 수 없는 경우, 프록시가 해당 호스트에 사용하는 송신 주소를 조직의 IP 허용 목록에 추가하되, 해당 주소가 조직에 전용되는 경우에만 추가하십시오. 공유 프록시 송신 범위는 프록시 공급업체의 다른 고객도 허용합니다.

Anthropic은 조직의 IP 허용 목록에 대한 `bridge.claudeusercontent.com` 연결을 도착 주소를 사용하여 확인합니다. 프록시가 해당 호스트의 트래픽을 해당 허용 목록에 없는 주소를 통해 전송하는 경우, Claude Code의 나머지 부분이 작동하더라도 [Chrome의 Claude](/docs/ko/chrome) 확장 프로그램에 연결할 수 없습니다.

<h3 id="github-allow-lists-and-firewalls">
  GitHub 허용 목록 및 방화벽
</h3>

Anthropic 호스팅 환경의 [웹의 Claude Code](/docs/ko/claude-code-on-the-web) 및 [코드 검토](/docs/ko/code-review)는 Anthropic 관리 인프라에서 리포지토리에 연결합니다. [자체 호스팅 환경](/docs/ko/self-hosted-environments)의 세션은 실행자가 [Anthropic git 프록시](/docs/ko/self-hosted-environments-deploy#use-the-anthropic-git-proxy)를 선택하지 않는 한 네트워크 내부에서 연결하며, 이는 Anthropic 측에서 가져옵니다.

GitHub Enterprise Cloud 조직이 IP 주소로 액세스를 제한하는 경우, [설치된 GitHub 앱에 대한 IP 허용 목록 상속](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#allowing-access-by-github-apps)을 활성화하고 Anthropic의 [아웃바운드 IP 주소](https://platform.claude.com/docs/en/api/ip-addresses#outbound-ip-addresses)에 대한 [허용 목록 항목을 추가](https://docs.github.com/en/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/managing-allowed-ip-addresses-for-your-organization#adding-an-allowed-ip-address)하십시오. 상속은 Claude GitHub 앱이 설치로 수행하는 요청만 다루며, 사용자를 대신하여 수행하는 요청은 다루지 않습니다. 다른 방화벽의 경우 [Anthropic API IP 주소](https://platform.claude.com/docs/en/api/ip-addresses)를 참조하십시오.

방화벽 뒤의 자체 호스팅 [GitHub Enterprise Server](/docs/ko/github-enterprise-server) 인스턴스의 경우, Anthropic 인프라가 GHES 호스트에 도달하여 리포지토리를 복제하고 검토 의견을 게시할 수 있도록 Anthropic의 [아웃바운드 IP 주소](https://platform.claude.com/docs/en/api/ip-addresses#outbound-ip-addresses)를 허용 목록에 추가하십시오. [자체 호스팅 환경](/docs/ko/self-hosted-environments-deploy#configure-git)의 세션은 네트워크 내부에서 GHES 호스트에 도달하므로, 해당 노출은 Anthropic 호스팅 세션, 리포지토리 선택기와 같은 호스팅 사전 세션 흐름 및 [Anthropic git 프록시](/docs/ko/self-hosted-environments-deploy#use-the-anthropic-git-proxy)를 선택하는 자체 호스팅 실행자에만 적용되며, 이는 Anthropic 측에서 가져옵니다. 네트워크 내부에서만 라우팅 가능한 GHES 호스트의 경우, [SCM 커넥터](/docs/ko/self-hosted-environments-reference#scm-connector-flags)는 호스팅된 사전 세션 흐름을 아웃바운드 연결을 통해 전달하므로 허용 목록이 필요하지 않습니다.

<h3 id="desktop-and-claude-ai">
  데스크톱 및 claude.ai
</h3>

앞의 표는 독립 실행형 CLI를 다룹니다. Claude Desktop 앱 및 브라우저의 claude.ai는 `assets-proxy.anthropic.com` 및 해당 앱에서 [Artifact](/docs/ko/artifacts)를 제공하는 다른 `*.claudeusercontent.com` 원본을 포함한 추가 Anthropic CDN 호스트에서 애플리케이션 코드 및 사용자 콘텐츠를 로드합니다. `claude.ai`를 허용하면서 해당 호스트를 차단하면 오류 대신 빈 페이지가 생성됩니다. Desktop 페이지의 [네트워크 액세스 요구사항](/docs/ko/desktop#network-access-requirements)을 참조하십시오.

[Google Fonts](/docs/ko/artifacts#improve-the-visual-design)에서 글꼴을 로드하는 [Artifact](/docs/ko/artifacts)는 `fonts.googleapis.com` 및 `fonts.gstatic.com`도 요청합니다. 두 호스트 모두 선택 사항입니다. 차단하면 Artifact는 대체 글꼴로 렌더링됩니다. 글꼴 요청이 페이지의 첫 렌더링을 지연시키는 대신 즉시 실패하도록 빠른 거부로 차단하십시오.

Artifact는 또한 React 또는 차트 패키지와 같은 JavaScript 라이브러리를 `cdnjs.cloudflare.com`, `cdn.jsdelivr.net`, `cdn.tailwindcss.com`, `code.jquery.com` 및 `unpkg.com`에서 로드할 수 있으며, 다른 외부 호스트에서는 로드할 수 없습니다. 해당 호스트를 차단하면 라이브러리에 종속된 Artifact의 부분이 작동하지 않으며, 차단된 글꼴과 달리 차단된 라이브러리는 대체가 없습니다. 차단된 라이브러리 요청이 시간 초과될 때까지 중단되는 대신 즉시 실패하도록 빠른 거부로 차단하십시오.

<h2 id="additional-resources">
  추가 리소스
</h2>

* [설정 파일 및 우선순위](/docs/ko/settings)
* [환경 변수 참조](/docs/ko/env-vars)
* [문제 해결 가이드](/docs/ko/troubleshooting)
