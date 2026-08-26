> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 기업 런처 뒤에서 Claude Code 실행

> CLAUDE_CODE_PROCESS_WRAPPER를 사용하여 Claude Code가 자체 바이너리에서 시작하는 프로세스(백그라운드 서비스 및 모든 에이전트 뷰 세션 포함)를 필수 런처를 통해 라우팅합니다.

일부 조직에서는 워크스테이션의 모든 프로세스가 필수 런처를 통해 시작되도록 요구합니다. 런처는 회사의 보안 태세가 의존하는 샌드박스, 네트워크 제어 또는 자격 증명 주입을 적용하며, 이를 거치지 않고 시작되는 바이너리는 정책 위반입니다.

`CLAUDE_CODE_PROCESS_WRAPPER`는 Claude Code가 자체 바이너리에서 시작하는 모든 프로세스를 런처를 통해 실행합니다: 백그라운드 서비스, [에이전트 뷰](/docs/ko/agent-view)에서 호스팅하는 모든 세션, 그리고 업데이트 후 Claude Code의 재시작입니다. 런처의 절대 경로로 설정하면 Claude Code는 런처를 Claude Code 명령을 인수로 하여 실행합니다.

`PATH`에서 `claude` 명령을 래핑하는 런처는 이러한 프로세스에 도달할 수 없습니다. 왜냐하면 이들은 `PATH` 조회 없이 바이너리의 직접 경로에서 시작되기 때문입니다.

<Note>
  `CLAUDE_CODE_PROCESS_WRAPPER`는 Claude Code v2.1.208 이상이 필요합니다. 이전 버전은 변수를 무시하고 모든 프로세스를 래핑 없이 시작합니다.
</Note>

<h2 id="what-the-launcher-covers">
  런처가 포함하는 것
</h2>

`CLAUDE_CODE_PROCESS_WRAPPER`가 설정되면 Claude Code는 다음 각 프로세스를 런처를 통해 시작합니다:

* `claude agents`와 백그라운드 세션이 필요에 따라 시작하는 백그라운드 서비스입니다.
* 모든 에이전트 뷰 행 내의 터미널 호스트 및 Claude Code 세션(서비스가 준비해 두는 웜 스탠바이 세션 포함).
* 업데이트 또는 충돌 후 서비스가 다시 생성하는 세션입니다.
* 업데이트 설치를 완료하기 위해 Claude Code가 자신을 재시작하는 것(에이전트 뷰의 업데이트를 위한 재시작 작업 포함).

Windows에서는 변수가 무시됩니다: 런처 계약은 `exec`에 따라 달라지는데, Windows는 이를 지원하지 않습니다. 변수가 설정된 Windows 머신은 모든 프로세스를 래핑 없이 실행하며 계속 작동하며, 유일한 신호는 [디버그 로그](/docs/ko/troubleshooting)의 경고입니다. 런처 정책이 Windows를 포함하는 경우, 변수는 거기서 이를 만족하지 않습니다: 롤아웃을 계획할 때 Windows 머신을 래핑되지 않은 것으로 계산합니다.

<h3 id="processes-that-start-outside-the-launcher">
  런처 외부에서 시작되는 프로세스
</h3>

세 가지 프로세스는 절대 런처를 통해 시작되지 않습니다:

* [설치된 백그라운드 서비스](/docs/ko/agent-view#the-supervisor-process): `launchd` 또는 `systemd`가 해당 프로세스를 단위 파일에서 시작합니다. `/status`와 `claude daemon status`는 이것이 적용될 때 경고하며, 서비스가 변수를 설정한 상태로 재시작되면 서비스가 생성하는 세션은 여전히 런처를 통해 시작됩니다.
* 터미널에서 직접 시작하는 세션으로, 호출한 방식대로 실행됩니다. 이러한 세션을 포함하려면 `PATH`의 이전 디렉토리에 `claude`라는 스크립트를 배치하여 실제 바이너리로 런처를 실행합니다. 관리되는 심볼릭 링크를 교체하지 마십시오. 자체 생성은 `PATH`를 참조하지 않으므로 두 런처는 절대 스택되지 않습니다.
* `claude-cli://` 딥 링크의 첫 번째 프로세스로, 운영 체제의 프로토콜 핸들러가 직접 시작합니다. 해당 세션이 백그라운드에서 시작하는 모든 것은 런처를 통해 실행됩니다. 이 경로를 완전히 닫으려면 `disableDeepLinkRegistration` 설정으로 [핸들러 등록을 방지](/docs/ko/deep-links#registration-and-supported-platforms)합니다.

<h3 id="helper-process-names-in-process-monitors">
  프로세스 모니터의 헬퍼 프로세스 이름
</h3>

런처가 구성되면 `ps`와 Activity Monitor는 런처의 `exec`가 인수 목록을 재구성하기 때문에 Claude Code의 `claude bg-pty-host` 및 `claude bg-spare` 레이블 대신 백그라운드 헬퍼 프로세스의 버전이 지정된 바이너리 이름을 표시합니다. 이름 변경은 은폐가 아닌 부작용입니다: 프로세스는 그 외에는 변경되지 않으며, Claude Code는 표시 이름이 아닌 바이너리 경로로 자신의 프로세스를 식별합니다.

<h2 id="set-up-the-launcher">
  런처 설정
</h2>

<Steps>
  <Step title="런처 스크립트 작성">
    `/opt/corp/launcher`와 같은 절대 경로에 실행 가능한 스크립트를 만듭니다. Claude Code는 전체 Claude Code 명령을 인수로 실행하며, 스크립트는 `exec "$@"`를 호출하여 자신을 Claude Code로 교체해야 합니다:

    ```bash theme={null}
    #!/bin/sh
    # Your organization's setup: enter the sandbox, apply
    # network controls, or inject credentials.
    exec "$@"
    ```

    `chmod +x`로 실행 가능하게 만듭니다. 설정 부분은 Claude Code가 실행되기 전에 런처가 수행해야 하는 모든 것입니다. 아래의 [런처 계약](#the-launcher-contract)은 스크립트가 따라야 할 규칙을 나열합니다.

    <Note>
      이전에 `~/.local/bin/claude` 심볼릭 링크를 런처로 교체한 경우, 같은 변경에서 원본 심볼릭 링크를 복원합니다. 교체된 심볼릭 링크는 첫 번째 래핑된 세션이 백그라운드 서비스를 두 런처를 통해 동시에 시작하게 하며, 설치를 외부에서 관리되는 상태로 만듭니다: `/doctor`가 이를 보고하고, 자동 업데이트는 파일을 제자리에 두며, 이전 버전의 정리는 설치 프로그램이 해당 경로를 다시 관리할 때까지 비활성화됩니다.
    </Note>
  </Step>

  <Step title="설정에서 CLAUDE_CODE_PROCESS_WRAPPER 설정">
    백그라운드 서비스가 이를 상속하도록 설정 파일의 `env` 블록에서 변수를 설정합니다. 셸 `export`는 충분하지 않습니다: 백그라운드 서비스는 필요에 따라 시작되고, 셸보다 오래 지속되며, 셸 프로필을 다시 읽지 않습니다.

    한 대의 머신의 경우 `~/.claude/settings.json`에 추가합니다. 조직의 모든 머신에 배포하려면 [관리되는 설정](/docs/ko/permissions#managed-settings)에 같은 블록을 배치합니다:

    ```json theme={null}
    {
      "env": {
        "CLAUDE_CODE_PROCESS_WRAPPER": "/opt/corp/launcher"
      }
    }
    ```

    둘 이상의 소스가 변수를 설정하면 관리되는 설정 값이 `~/.claude/settings.json`과 셸에서 내보낸 값을 모두 재정의하므로 사용자는 자체 생성을 다른 런처로 지정할 수 없습니다.

    프로젝트 및 로컬 설정은 이 변수를 설정할 수 없습니다. 저장소에 커밋된 파일은 머신의 모든 Claude Code 프로세스 앞에 바이너리를 배치할 수 없으므로 `.claude/settings.json` 또는 `.claude/settings.local.json`의 `CLAUDE_CODE_PROCESS_WRAPPER`는 무시되며 [디버그 로그](/docs/ko/troubleshooting)에 경고가 표시됩니다.
  </Step>

  <Step title="백그라운드 서비스 및 세션 재시작">
    실행 중인 백그라운드 서비스와 열려 있는 `claude` 세션은 시작 시 변수를 한 번 읽으므로 재시작될 때까지 래핑되지 않은 프로세스를 계속 시작합니다. `claude daemon stop --any`를 실행하여 필요에 따라 서비스를 중지합니다. `claude agents`와 같이 이를 필요로 하는 다음 명령은 래핑된 서비스를 시작합니다. [설치된 서비스](/docs/ko/agent-view#the-supervisor-process)는 `--any` 없이 `claude daemon stop`을 사용합니다. 그런 다음 열려 있는 `claude` 세션을 재시작합니다.

    손으로 재시작할 수 없는 머신에서는 설정 푸시 후 시작된 첫 번째 세션이 남은 래핑되지 않은 필요에 따른 서비스를 자동으로 폐기합니다. 새 세션이 시작되지 않는 머신은 하나가 시작될 때까지 래핑되지 않은 서비스를 유지하며, 설치된 서비스는 항상 이 단계에서 재시작이 필요합니다.
  </Step>

  <Step title="확인">
    세션에서 `/status`를 실행합니다: Self-exec 항목은 해결된 시작 명령을 표시하고 실행 중인 백그라운드 서비스가 일치하지 않을 때 경고합니다. `claude daemon status`는 변수를 설정 해제할 때를 포함하여 셸에서 같은 정보를 인쇄하며, 이때 `/status`는 더 이상 항목을 표시하지 않습니다.
  </Step>
</Steps>

<h2 id="the-launcher-contract">
  런처 계약
</h2>

런처가 실행될 수 없으면 Claude Code는 래핑되지 않은 상태로 시작하는 대신 프로세스 시작을 거부합니다. Windows에서는 [변수가 무시되며](#what-the-launcher-covers) 프로세스가 래핑되지 않은 상태로 시작됩니다. Claude Code는 스크립트를 다음 규칙에 따릅니다:

* **`exec "$@"`로 끝냅니다.** 자식을 포크하고 종료하는 런처는 백그라운드 서비스가 추적할 수 없는 고아 Claude Code 프로세스를 남깁니다. 에이전트 뷰는 런처 이름을 지정하는 메시지와 함께 이러한 세션을 실패로 표시하며, 서비스는 런처가 남긴 것을 수거합니다.
* **인수를 재정렬, 흡수 또는 앞에 추가하지 마십시오.** 첫 번째 인수는 Claude Code 바이너리이고 그 이후의 모든 것은 해당 argv입니다.
* **상속된 모든 환경 변수를 `exec`를 통해 전달합니다.** 주입된 자격 증명과 같은 변수를 추가하는 것은 괜찮습니다. 상속된 변수를 삭제하는 것은 아닙니다.
  * 세션별 인증 토큰, 모델 및 공급자 선택, 그리고 `CLAUDE_CODE_PROCESS_WRAPPER` 자체는 모두 상속된 환경에서 이동하므로 허용 목록에서 재구성하는 런처는 시작하는 세션을 중단하며, `/status`는 런처 불일치를 보고합니다.
  * 런처가 환경을 재설정하는 네임스페이스 또는 샌드박스에 들어가야 하는 경우, 상속된 환경을 그 안에서 그대로 다시 내보냅니다.
* **런처가 실행될 때마다 약 3초 이내에 `exec`에 도달합니다.** 콜드 백그라운드 디스패치는 첫 번째 출력 바이트 전에 런처를 두 번 연속으로 실행하므로 단일 사인온 교환과 같은 느린 작업을 게으르게 또는 캐시에서 수행합니다.
  * 예산을 훨씬 초과하여 실행되는 런처는 정지된 시작으로 취급되고 재시작됩니다.
* **자신 내부에서 호출되는 것을 허용합니다.** Claude Code는 모든 중첩된 자체 생성에 런처를 적용하므로 배타적 리소스를 획득하는 런처는 이미 보유하고 있음을 감지해야 합니다.
* **Claude Code가 시작되기 전에 터미널에 쓰지 마십시오.** `exec` 전에 인쇄된 모든 것은 세션이 초기화 전에 종료되면 충돌 원인으로 보고됩니다.

<h3 id="format-of-the-claude_code_process_wrapper-value">
  `CLAUDE_CODE_PROCESS_WRAPPER` 값의 형식
</h3>

대부분의 런처의 경우 값은 `/opt/corp/launcher`와 같은 스크립트의 절대 경로입니다.

런처에 자신의 인수를 전달하려면 경로 뒤에 작성합니다. Claude Code는 값을 셸 명령이 아닌 인수 목록으로 구문 분석합니다:

* 공백은 토큰을 분리하고 큰따옴표는 공백을 포함하는 토큰을 그룹화합니다.
* `[`로 시작하는 값은 `["/opt/corp/launcher", "--profile", "cc"]`와 같은 JSON 문자열 배열로 읽혀집니다.
* 셸 구문은 작동하지 않습니다: 변수 확장이나 글로빙이 없으며, `;`, `|`, `&` 또는 `$(`와 같은 인용되지 않은 연산자는 재해석되지 않고 구성 오류로 거부됩니다.

값을 사용할 수 없으면 Claude Code는 영향을 받는 프로세스 시작을 거부하고 [이유를 보고합니다](/docs/ko/errors#claude_code_process_wrapper-launcher-errors).

<h2 id="relationship-to-claude_code_shell_prefix">
  `CLAUDE_CODE_SHELL_PREFIX`와의 관계
</h2>

`CLAUDE_CODE_PROCESS_WRAPPER`는 Claude Code의 자신의 프로세스를 래핑하고 명령을 런처가 `exec`할 별도의 argv 토큰으로 전달합니다. [`CLAUDE_CODE_SHELL_PREFIX`](/docs/ko/env-vars)는 Claude Code가 사용자를 대신하여 실행하는 셸 명령(예: Bash 도구 호출, 훅, stdio MCP 서버를 시작하는 명령)을 래핑하고 각각을 래퍼가 재평가할 `$1`의 단일 셸 인용 문자열로 전달합니다. 하나를 위해 작성된 런처는 다른 하나로 작동하지 않습니다.

<h2 id="related-resources">
  관련 리소스
</h2>

* [에이전트 뷰](/docs/ko/agent-view): 런처가 포함하는 백그라운드 세션 및 감독자 프로세스
* [환경 변수](/docs/ko/env-vars): `CLAUDE_CODE_PROCESS_WRAPPER` 참조 항목
* [관리되는 설정](/docs/ko/permissions#managed-settings): 전체 플릿에 `env` 블록 전달
* [런처 오류 참조](/docs/ko/errors#claude_code_process_wrapper-launcher-errors): 거부 메시지 및 복구 방법
