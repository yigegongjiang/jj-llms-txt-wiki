> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 클라우드에서 Claude Code 사용하기

> 브라우저, 휴대폰, 데스크톱 앱 또는 터미널에서 클라우드의 Claude Code 세션을 실행하고, --cloud 및 --teleport로 이동하며, pull request를 자동 수정합니다.

<Note>
  클라우드 세션은 Pro, Max 및 Team 사용자, 그리고 프리미엄 시트 또는 Chat + Claude Code 시트가 있는 Enterprise 사용자를 위해 제공됩니다.
</Note>

클라우드 세션은 사용자의 머신 대신 클라우드 인프라에서 실행되는 Claude Code 세션입니다. 기본적으로 Anthropic이 관리하는 인프라에서 실행되거나, 라우팅될 때 조직의 [자체 호스팅 환경](/docs/ko/self-hosted-environments)에서 실행됩니다. 세션은 노트북을 닫은 후에도 계속 실행되며, 모든 기기에서 확인하거나 조종할 수 있습니다.

다음 중 어느 곳에서나 클라우드 세션을 시작할 수 있습니다:

* **브라우저**: [claude.ai/code](https://claude.ai/code), 웹에서 Claude Code라고도 불림
* **모바일**: [Claude 앱](/docs/ko/mobile)의 **Code** 탭
* **데스크톱 앱**: [세션을 시작](/docs/ko/desktop#run-long-running-tasks-in-the-cloud)할 때 **Local** 대신 **Cloud**를 선택
* **터미널**: [`claude --cloud`](#from-terminal-to-cloud)
* **루틴**: [예약 및 트리거된 실행](/docs/ko/routines)은 각각 클라우드 세션으로 실행됨

한 본문의 작업에 대해 Claude가 많은 클라우드 세션을 시작하고 추적하도록 하려면 [프로젝트](/docs/ko/claude-projects)를 사용하십시오. 터미널, IDE 또는 **Local**이 선택된 데스크톱 앱의 세션은 사용자의 머신에서 실행됩니다. 휴대폰이나 브라우저에서 이러한 로컬 세션을 조종하려면 [원격 제어](/docs/ko/remote-control)를 사용하세요.

<Tip>
  클라우드 세션을 처음 사용하시나요? [시작하기](/docs/ko/web-quickstart)에서 GitHub 계정을 연결하고 첫 번째 작업을 제출하세요.
</Tip>

이 페이지에서 다루는 내용:

* [클라우드 환경](#cloud-environments): 세션이 실행되는 위치 및 구성 방법
* [GitHub 인증 옵션](#github-authentication-options): GitHub를 연결하는 두 가지 방법
* [터미널과 클라우드 간에 작업 이동](#move-tasks-between-terminal-and-cloud): `--cloud` 및 `--teleport` 사용
* [세션 작업](#work-with-sessions): 권한 모드, 검토, 공유, 보관, 삭제
* [Pull request 자동 수정](#auto-fix-pull-requests): CI 실패 및 검토 주석에 자동으로 응답
* [보안 및 격리](#security-and-isolation): 세션이 어떻게 격리되는지
* [제한 사항](#limitations): 속도 제한 및 플랫폼 제한

<h2 id="cloud-environments">
  클라우드 환경
</h2>

모든 클라우드 세션은 [클라우드 환경](/docs/ko/cloud-environments)에서 실행되며, 이는 네트워크 액세스, 환경 변수 및 설정 스크립트를 제어하는 저장된 구성입니다. 아직 환경이 없으면 온보딩이 [**신뢰할 수 있는** 네트워크 액세스](/docs/ko/cloud-environments#access-levels)를 사용하여 **기본** 환경을 설정합니다. 이는 사용자를 위해 생성하거나 사용자가 생성하도록 요청합니다. 플랜에서 어느 것이 발생하는지 및 둘 이상의 환경이 있을 때 세션이 환경을 선택하는 방법은 [기본 환경](/docs/ko/cloud-environments#the-default-environment)을 참조하세요.

동일한 환경이 클라우드 세션을 시작하는 모든 위치에 적용됩니다: 웹, 터미널, [Claude Tag](https://claude.com/docs/claude-tag/overview), [routines](/docs/ko/routines) 및 모바일 및 Desktop 앱. Claude Tag 채널 세션은 조직 수준 환경만 사용하며, [공유 환경](/docs/ko/cloud-environments#organization-shared-environments) 또는 [자체 호스팅 환경](/docs/ko/self-hosted-environments)입니다.

환경이 허용하는 것을 변경하고, 변수를 설정하거나, 설정 스크립트를 추가하려면 [클라우드 환경 구성](/docs/ko/cloud-environments)을 참조하세요. 구성 없이 세션에 포함되는 것은 [설치된 도구](/docs/ko/cloud-environments#installed-tools)를 참조하세요.

<h2 id="github-authentication-options">
  GitHub 인증 옵션
</h2>

클라우드 세션은 코드를 복제하고 분기를 푸시하기 위해 GitHub 저장소에 액세스해야 합니다. 두 가지 방법으로 액세스 권한을 부여할 수 있습니다:

| 방법               | 연결 방식                                                       | 세션이 도달할 수 있는 저장소                           | 최적 대상                                             |
| :--------------- | :---------------------------------------------------------- | :----------------------------------------- | :------------------------------------------------ |
| **GitHub App**   | [웹 온보딩](/docs/ko/web-quickstart) 중에 Claude GitHub App을 승인합니다.    | 모든 공개 저장소 및 Claude GitHub App이 설치된 비공개 저장소 | 브라우저 온보딩; [자동 수정](#auto-fix-pull-requests)을 원하는 팀 |
| **`/web-setup`** | 터미널에서 `/web-setup`을 실행하여 로컬 `gh` CLI 토큰을 Claude 계정으로 전송합니다. | `gh` 토큰이 액세스할 수 있는 모든 저장소(App 설치 여부와 관계없음) | 이미 `gh`를 사용하는 개별 개발자                              |

저장소에 Claude GitHub App을 설치하면 해당 저장소의 풀 요청에 대해 [자동 수정](#auto-fix-pull-requests)도 활성화됩니다.

[프로젝트](/docs/ko/claude-projects)의 스레드는 연결 방법에 관계없이 복제하는 각 저장소에 App이 설치되어 있어야 합니다. [GitHub 액세스 설정](/docs/ko/claude-projects#set-up-github-access)을 참조하세요.

`/schedule`이 루틴을 생성하기 전에 저장소 액세스를 확인하는 방법은 [저장소 및 분기 권한](/docs/ko/routines#repositories-and-branch-permissions)을 참조하세요. `/web-setup` 안내(포함된 내용 및 제거 방법 포함)는 [터미널에서 연결](/docs/ko/web-quickstart#connect-from-your-terminal)을 참조하세요.

빠른 웹 설정은 구성원이 `/web-setup`으로 GitHub를 연결할 수 있게 하고, 브라우저 온보딩 중에 Claude GitHub App 설치 프롬프트를 건너뛰고, 환경 양식을 표시하는 대신 브라우저 온보딩이 [**기본** 환경](/docs/ko/cloud-environments#the-default-environment)을 생성하도록 하는 조직 설정입니다. Team 및 Enterprise 플랜에서는 기본적으로 꺼져 있으며, 이는 `/web-setup`을 숨깁니다. [Owner](/docs/ko/server-managed-settings#access-control)는 [**Admin settings > Claude Code**](https://claude.ai/admin-settings/claude-code)의 **Quick web setup** 토글로 켭니다.

<Note>
  [Zero Data Retention](/docs/ko/zero-data-retention)이 활성화된 조직은 `/web-setup` 또는 기타 클라우드 세션 기능을 사용할 수 없습니다.
</Note>

<h2 id="move-tasks-between-terminal-and-cloud">
  터미널과 클라우드 간에 작업 이동
</h2>

이러한 워크플로우는 동일한 claude.ai 계정에 로그인한 [Claude Code CLI](/docs/ko/quickstart)가 필요합니다. 터미널에서 새 클라우드 세션을 시작하거나 클라우드 세션을 터미널로 가져와 로컬에서 계속할 수 있습니다. 클라우드 세션은 노트북을 닫아도 유지되며, Claude 모바일 앱을 포함한 어디서나 모니터링할 수 있습니다.

<Note>
  CLI에서 세션 핸드오프는 일방향입니다: `--teleport`로 클라우드 세션을 터미널로 가져올 수 있지만 기존 터미널 세션을 클라우드로 푸시할 수 없습니다. `--cloud` 플래그는 작업 설명과 함께 현재 저장소에 대한 새로운 클라우드 세션을 생성합니다. `-p` 및 세션 ID 또는 claude.ai/code URL과 함께 [기존 세션에 메시지를 큐에 넣습니다](/docs/ko/claude-code-on-the-web#send-follow-ups-from-the-cli). [Desktop 앱](/docs/ko/desktop#continue-in-another-surface)은 로컬 세션을 클라우드로 보낼 수 있는 **Continue in** 메뉴를 제공합니다.
</Note>

<h3 id="from-terminal-to-cloud">
  터미널에서 클라우드로
</h3>

`--cloud` 플래그로 명령줄에서 클라우드 세션을 시작하세요:

```bash theme={null}
claude --cloud "Fix the authentication bug in src/auth/login.ts"
```

이렇게 하면 claude.ai에서 새 클라우드 세션이 생성됩니다. 클라우드 VM은 현재 디렉토리의 GitHub 원격을 현재 분기에서 복제하므로, 로컬 커밋이 있으면 먼저 푸시하세요. [GitHub 없이 로컬 저장소 보내기](#send-local-repositories-without-github)를 참조하여 Claude Code가 원격을 복제하는 대신 로컬 저장소를 업로드하는 경우를 확인하세요.

`--cloud`는 한 번에 하나의 저장소에서 작동합니다. 작업은 클라우드에서 실행되는 동안 로컬에서 계속 작업할 수 있습니다. 더 이상 사용되지 않는 `--remote` 표기법은 여전히 `--cloud`의 더 이상 사용되지 않는 별칭으로 작동합니다.

클라우드 컨테이너가 시작되는 동안 CLI는 저장소 복제 및 [설정 스크립트](/docs/ko/cloud-environments#setup-scripts) 실행과 같은 설정 단계의 라이브 체크리스트를 표시합니다. 프로비저닝 중에 입력한 메시지는 큐에 저장되었다가 세션이 준비되면 전송됩니다.

<Note>
  `--cloud`는 클라우드 세션을 생성합니다. `--remote-control`은 관련이 없습니다: 로컬 CLI 세션을 모니터링하고 조종할 수 있게 해줍니다. [Remote Control](/docs/ko/remote-control)을 참조하세요.
</Note>

claude.ai 또는 Claude 모바일 앱에서 세션을 열어 진행 상황을 확인하거나 직접 상호 작용하세요. 여기서 Claude를 조종하고, 피드백을 제공하거나, 다른 대화처럼 질문에 답변할 수 있습니다.

Claude가 질문을 하고 세션이 유휴 상태로 있으면 [환경 만료](#environment-expired)까지 돌아올 때 답변할 수 있으며, 세션이 답변에서 계속됩니다.

<h4 id="tips-for-cloud-tasks">
  클라우드 작업 팁
</h4>

**로컬에서 계획하고 클라우드에서 실행**: 복잡한 작업의 경우 Claude를 plan mode에서 시작하여 접근 방식을 협력한 다음 작업을 클라우드로 보내세요:

```bash theme={null}
claude --permission-mode plan
```

Plan mode에서 Claude는 파일을 읽고, 명령을 실행하여 탐색하고, 소스 코드를 편집하지 않고 계획을 제안합니다. 계획에 만족하면 저장소에 저장하고, 커밋하고, 푸시하여 클라우드 VM이 복제할 수 있도록 한 다음 자율 실행을 위해 클라우드 세션을 시작하세요:

```bash theme={null}
claude --cloud "Execute the migration plan in docs/migration-plan.md"
```

**작업을 병렬로 실행**: 각 `--cloud` 명령은 독립적으로 실행되는 자체 클라우드 세션을 생성합니다. 여러 작업을 시작할 수 있으며 모두 별도의 세션에서 동시에 실행됩니다:

```bash theme={null}
claude --cloud "Fix the flaky test in auth.spec.ts"
claude --cloud "Update the API documentation"
claude --cloud "Refactor the logger to use structured output"
```

세션이 완료되면 claude.ai/code에서 PR을 생성하거나 [세션을 텔레포트](#from-cloud-to-terminal)하여 터미널에서 계속 작업할 수 있습니다.

<h4 id="send-local-repositories-without-github">
  GitHub 없이 로컬 저장소 보내기
</h4>

git 원격이 없는 저장소에서 `claude --cloud`를 실행하거나 Claude GitHub 앱이 설치되지 않은 github.com 저장소에서 실행하면 Claude Code가 로컬 저장소를 번들로 만들어 클라우드 세션에 직접 업로드합니다. 이는 `/web-setup`으로 GitHub를 연결한 경우에도 적용됩니다. 번들에는 모든 분기의 전체 저장소 기록과 추적된 파일에 대한 커밋되지 않은 변경 사항이 포함됩니다.

macOS, Linux 및 WSL에서 Claude Code는 자격 증명이나 키처럼 명명된 파일에 대한 커밋되지 않은 변경 사항을 업로드에서 제외하고 제외한 파일의 이름을 지정합니다. 이는 `.env` 파일, Terraform `*.tfvars` 파일 및 `id_rsa` 및 `*.pem`과 같은 키 파일을 포함합니다. 세션은 각각의 커밋된 버전으로 시작하거나 커밋된 것이 없으면 파일 없이 시작합니다. 연결된 worktree, submodule 또는 유사한 레이아웃에서 Claude Code는 이러한 변경 사항을 나머지와 함께 업로드하고 업로드한 파일의 이름을 지정합니다.

원격을 복제할 수 없을 때 이 폴백이 자동으로 활성화됩니다. 강제하려면 `CCR_FORCE_BUNDLE=1`을 설정하세요:

```bash theme={null}
CCR_FORCE_BUNDLE=1 claude --cloud "Run the test suite and fix any failures"
```

번들된 저장소는 이러한 제한을 충족해야 합니다:

* 디렉토리는 최소 하나의 커밋이 있는 git 저장소여야 합니다
* 번들된 저장소는 100 MB 미만이어야 합니다. 더 큰 저장소는 현재 분기만 번들로 만들기로 폴백한 다음 작업 트리의 단일 스쿼시 스냅샷으로 폴백하고, 스냅샷이 여전히 너무 크면 실패합니다
* 추적되지 않은 파일은 포함되지 않습니다. 클라우드 세션이 보기를 원하는 파일에 대해 `git add`를 실행하세요
* 번들에서 생성된 세션은 [GitHub 연결](#github-authentication-options)에 해당 저장소에 대한 푸시 액세스 권한이 있을 때만 GitHub 원격으로 다시 푸시할 수 있습니다

<h3 id="send-follow-ups-from-the-cli">
  CLI에서 후속 메시지 보내기
</h3>

클라우드 세션이 실행 중이면 어디서든 실행되든 `claude auth login`으로 로그인한 모든 머신의 `claude` CLI에서 후속 메시지를 보낼 수 있습니다. CLI는 Anthropic 계정 자격 증명으로 인증하고 로컬 세션 상태를 보내지 않으므로 명령이 세션을 시작한 머신에서 실행될 필요가 없으며 PowerShell을 포함한 모든 셸에서 동일합니다.

명령은 하나의 메시지를 게시하고 종료합니다:

```bash theme={null}
claude -p "your message" --cloud <session-id>
```

CLI는 메시지를 세션에 큐에 넣고 회신을 기다리지 않고 종료합니다. 이를 사용하여 장시간 실행되는 세션을 조종하거나, 현재 세션이 아직 완료되는 동안 다음 단계를 큐에 넣거나, [CI 스크립트](/docs/ko/self-hosted-environments-testing#run-the-test-loop)에서 후속 메시지를 보내세요. 인수로 전달하는 대신 stdin에서 메시지를 파이프할 수도 있습니다: `echo "your message" | claude -p --cloud <session-id>`.

`<session-id>`의 경우 `session_...` 또는 `cse_...`과 같은 bare ID 또는 세션의 `claude.ai/code/<id>` URL을 전달하세요(스키마 또는 쿼리 문자열 포함 또는 제외). claude.ai/code의 세션 목록에서 ID를 찾으세요.

<Note>
  `--cloud`는 Anthropic 계정이 필요합니다. Claude Code가 Amazon Bedrock, Google Cloud의 Agent Platform 또는 다른 타사 제공자로 구성된 경우 사용할 수 없습니다. `ANTHROPIC_BASE_URL`을 통해서만 구성된 [LLM gateway](/docs/ko/llm-gateway)는 이 확인을 위해 타사 제공자로 계산되지 않지만 여전히 `claude auth login`으로 로그인해야 합니다. 조직의 `allow_remote_sessions` 정책도 활성화되어야 합니다. Owner는 claude.ai/admin-settings/claude-code의 Claude Code 관리 설정에서 켤 수 있습니다.
</Note>

<h4 id="output-and-errors">
  출력 및 오류
</h4>

성공하면 명령은 세션 ID와 세션을 보기 위한 링크를 인쇄합니다:

```
Sent to cloud session.
Session ID: session_01DiUkqY2kzbUbDmW1w96rfi
View: https://claude.ai/code/session_01DiUkqY2kzbUbDmW1w96rfi?from=cli&m=0
```

기계 판독 가능한 결과를 위해 `--output-format json`을 전달하세요: 성공 시 `{ok, session_id, url}` 또는 세션이 누락되거나 보관된 경우와 같이 전송이 실패할 때 `{ok: false, session_id, error}`. 지원되지 않는 제공자 또는 비활성화된 조직 정책과 같은 구성 오류는 JSON 없이 stderr에 인쇄됩니다. `--output-format stream-json`은 `--cloud <session-id>`에서 지원되지 않습니다.

CLI는 오류 앞에 `Error: `를 붙입니다. 실패한 전달은 `failed to send message to cloud session <id>: <reason>`으로 래핑됩니다.

| 메시지                                                                                                                         | 의미                                                                                                                                                                                                                     |
| --------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Cloud sessions aren't available with <provider>. They run on Anthropic's infrastructure and require an Anthropic account.` | Claude Code가 타사 제공자로 구성되어 있습니다. 메시지는 구성이 사용하는 레이블(예: `Amazon Bedrock` 또는 `Google Vertex AI`)과 함께 제공자의 이름을 지정합니다. 예를 들어 `CLAUDE_CODE_USE_BEDROCK`을 설정 해제하여 해당 제공자의 구성을 제거하고 Anthropic 계정(`claude auth login`)으로 로그인하세요. |
| `Cloud sessions are disabled by your organization's policy. Contact your organization admin to enable them.`                | `allow_remote_sessions` 조직 정책이 꺼져 있습니다.                                                                                                                                                                                |
| `Couldn't verify your organization's policy for cloud sessions. Check your network connection and try again.`               | Claude Code가 조직의 정책을 가져올 수 없어 클라우드 세션이 허용된다고 가정하기보다는 전송을 거부합니다. 네트워크 연결을 확인하고 다시 시도하세요.                                                                                                                                |
| `Attaching to an existing cloud session is not enabled for your account.`                                                   | `-p` 없이 `--cloud <session-id>`를 실행했습니다. `claude -p "your message" --cloud <session-id>`로 메시지를 보내세요.                                                                                                                    |
| `Session not found: <id>`                                                                                                   | ID 또는 URL이 액세스할 수 있는 세션과 일치하지 않습니다. 세션의 claude.ai/code URL에 대해 확인하세요.                                                                                                                                                  |
| `cloud session <id> is archived and cannot accept new messages`                                                             | 세션이 보관되었습니다. 대신 새 세션을 시작하세요.                                                                                                                                                                                           |

<h3 id="from-cloud-to-terminal">
  클라우드에서 터미널로
</h3>

다음 중 하나를 사용하여 클라우드 세션을 터미널로 가져오세요:

* **`--teleport` 사용**: 명령줄에서 `claude --teleport`를 실행하여 대화형 세션 선택기를 사용하거나 `claude --teleport <session-id>`를 실행하여 특정 세션을 직접 재개합니다. 커밋되지 않은 변경 사항이 있으면 먼저 stash하라는 메시지가 표시됩니다.
* **`/teleport` 사용**: 기존 CLI 세션 내에서 `/teleport` 또는 `/tp`를 실행하여 Claude Code를 다시 시작하지 않고 동일한 세션 선택기를 엽니다.
* **`/tasks`에서**: `/tasks`를 실행하여 백그라운드 세션을 보고 `t`를 눌러 하나로 텔레포트합니다.
* **claude.ai/code에서**: 세션 메뉴에서 **Open in > Terminal**을 선택하여 터미널에 붙여넣을 수 있는 명령을 복사합니다.
* **클라우드 세션 내에서**: `/teleport`를 입력하면 Claude Code가 해당 세션에 대해 저장소의 체크아웃에서 실행할 준비가 된 정확한 `claude --teleport <session-id>` 명령으로 회신합니다. 세션의 환경에서 Claude Code v2.1.223 이상이 필요합니다.

세션을 텔레포트하면 Claude가 올바른 저장소에 있는지 확인하고, 클라우드 세션에서 분기를 가져와 체크아웃하고, 전체 대화 기록을 터미널에 로드합니다. 터미널은 세션의 자체 복사본을 가져옵니다: 여기서의 새로운 작업은 로컬로 유지되며 claude.ai의 클라우드 세션이나 Claude 모바일 앱에 나타나지 않습니다. 텔레포트 후 휴대폰에서 조종을 계속하려면 로컬 세션에서 [`/remote-control`](/docs/ko/remote-control)을 시작하세요.

`--teleport`는 `--resume`과 다릅니다. `--resume`은 이 머신의 로컬 기록에서 대화를 다시 열고 클라우드 세션을 나열하지 않습니다. `--teleport`는 클라우드 세션과 해당 분기를 가져옵니다.

<h4 id="teleport-requirements">
  텔레포트 요구 사항
</h4>

텔레포트는 세션을 재개하기 전에 이러한 요구 사항을 확인합니다. 요구 사항이 충족되지 않으면 오류가 표시되거나 문제를 해결하라는 메시지가 표시됩니다.

| 요구 사항           | 세부 정보                                                                                                                                                                                                                                                                                                             |
| --------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Clean git state | 작업 디렉토리에 커밋되지 않은 변경 사항이 없어야 합니다. 텔레포트가 필요한 경우 변경 사항을 stash하라는 메시지를 표시합니다.                                                                                                                                                                                                                                         |
| 올바른 저장소         | fork가 아닌 동일한 저장소의 체크아웃에서 `--teleport`를 실행해야 합니다. 다른 저장소의 체크아웃에서 실행하면 Claude Code는 세션의 저장소와 체크아웃의 저장소 모두의 이름을 지정하는 오류를 표시합니다. v2.1.219 이전에는 오류가 체크아웃의 저장소 이름을 지정하지 않았습니다. Claude Code가 `git@work:owner/repo.git`과 같은 SSH 호스트 별칭으로 원격을 호스트 이름으로 구문 분석할 수 없으면 확인을 요청하고 원격의 소유자 및 저장소 이름이 세션의 저장소와 일치할 때 체크아웃을 수락합니다. |
| 분기 사용 가능        | 클라우드 세션의 분기가 원격으로 푸시되어야 합니다. 텔레포트가 자동으로 가져와 체크아웃합니다.                                                                                                                                                                                                                                                              |
| 동일한 계정          | 클라우드 세션에서 사용한 동일한 claude.ai 계정으로 인증되어야 합니다.                                                                                                                                                                                                                                                                       |

<h4 id="teleport-is-unavailable">
  `--teleport`를 사용할 수 없음
</h4>

텔레포트는 claude.ai 구독 인증이 필요합니다. API 키로 인증된 경우 `/login`을 실행하여 대신 claude.ai 계정으로 로그인하세요. 오류가 제공자의 이름을 지정하면 클라우드 세션을 타사 제공자를 통해 사용할 수 없습니다. [오류 표](#output-and-errors)를 참조하세요. 이미 claude.ai를 통해 로그인했는데 `--teleport`를 여전히 사용할 수 없으면 조직이 클라우드 세션을 비활성화했을 수 있습니다.

<h2 id="work-with-sessions">
  세션 작업
</h2>

세션은 claude.ai/code의 사이드바에 나타납니다. 여기서 변경 사항을 검토하고, 팀원과 공유하고, 완료된 작업을 보관하거나, 세션을 영구적으로 삭제할 수 있습니다.

<h3 id="take-back-a-queued-message">
  대기 중인 메시지 되돌리기
</h3>

Claude가 작업 중일 때 메시지를 보내면 Claude가 읽을 때까지 메시지가 대기합니다. 대기 중인 메시지를 되돌리려면 메시지의 ✕를 클릭하세요. 텍스트가 메시지 상자로 돌아가므로 편집하거나 다른 내용을 보낼 수 있습니다.

Claude가 이미 메시지를 읽었다면 메시지는 대화에 남아 있습니다.

<h3 id="manage-context">
  컨텍스트 관리
</h3>

클라우드 세션은 텍스트 출력을 생성하는 [기본 제공 명령](/docs/ko/commands)을 지원합니다. `/plugin` 또는 `/resume`과 같이 터미널 인터페이스에서만 실행되는 명령은 사용할 수 없습니다. 터미널에서 선택기 또는 패널을 열어야 하는 명령은 클라우드 세션에서 다르게 작동합니다:

* **`/model`, `/effort`, `/color`, 및 `/rename`**: 터미널 선택기 또는 슬라이더를 열지 않고 대신 인수로 값을 전달합니다(예: `/model sonnet`). 인수 형식은 세션의 환경에서 Claude Code v2.1.205 이상이 필요하며 각 명령의 [가용성 참고 사항](/docs/ko/commands#all-commands)을 따릅니다.
* **`/fast`**: 빠른 모드가 [계정에서 사용 가능](/docs/ko/fast-mode#requirements)할 때 세션에 대해 [빠른 모드](/docs/ko/fast-mode#use-fast-mode-in-cloud-sessions)를 전환합니다. 세션의 환경에서 Claude Code v2.1.271 이상이 필요합니다.
* **`/config`**: 웹에서는 값을 설정하는 대신 Claude Code 설정 섹션을 열며, `key=value`를 포함한 명령 뒤의 텍스트는 무시됩니다. 클라우드 세션의 설정을 변경하려면 환경에 [환경 변수](/docs/ko/cloud-environments#set-environment-variables)를 설정하거나, 하나의 저장소가 있는 세션에서 해당 저장소의 `.claude/settings.json`에 키를 커밋하세요. [클라우드 세션의 설정](/docs/ko/settings#settings-in-cloud-sessions)에서 각 세션이 읽는 내용을 나열합니다.

컨텍스트 관리 특히:

| 명령         | 클라우드 세션에서 작동 | 참고                                                                          |
| :--------- | :----------- | :-------------------------------------------------------------------------- |
| `/compact` | 예            | 대화를 요약하여 컨텍스트를 확보합니다. `/compact keep the test output`과 같은 선택적 포커스 지침을 허용합니다 |
| `/context` | 예            | 현재 컨텍스트 윈도우에 있는 것을 표시합니다                                                    |
| `/clear`   | 아니오          | 사이드바에서 새 세션을 시작하세요                                                          |

자동 압축은 컨텍스트 윈도우가 용량에 접근할 때 자동으로 실행됩니다. 클라우드 세션은 [`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`](/docs/ko/env-vars)를 자체적으로 설정하므로 압축이 윈도우가 가득 찰 때가 아니라 [auto-compact window](/docs/ko/model-config#set-the-auto-compact-window)의 중간에 트리거됩니다. 해당 값은 [환경 변수](/docs/ko/cloud-environments#set-environment-variables)에 추가한 것을 재정의하므로 거기에 변수를 추가해도 압축이 트리거되는 시기가 변경되지 않습니다.

대신 auto-compact window를 변경하려면 환경 변수에 [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/ko/env-vars)를 설정하거나 변수가 설정되지 않은 세션에서 토큰 수와 함께 [`/autocompact`](/docs/ko/commands#all-commands)를 실행하세요.

[Subagents](/docs/ko/sub-agents)는 로컬과 동일한 방식으로 작동합니다. Claude는 Agent 도구로 이들을 생성하여 연구 또는 병렬 작업을 별도의 컨텍스트 윈도우로 오프로드하여 주 대화를 더 가볍게 유지할 수 있습니다. 저장소의 `.claude/agents/`에 정의된 Subagents는 자동으로 선택됩니다.

[Agent teams](/docs/ko/agent-teams)는 기본적으로 꺼져 있지만 [환경 변수](/docs/ko/cloud-environments#set-environment-variables)에 `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`을 추가하여 활성화할 수 있습니다.

<h3 id="permission-modes-in-cloud-sessions">
  클라우드 세션의 권한 모드
</h3>

[권한 모드](/docs/ko/permission-modes)는 [모드 드롭다운](/docs/ko/permission-modes#switch-permission-modes)에서 클라우드 세션을 생성할 때와 세션이 실행되는 동안 선택합니다. Anthropic 호스팅 [환경이 만료된](#environment-expired) 세션을 다시 열거나 자체 호스팅 runner가 [유휴 상태일 때 해제한](/docs/ko/self-hosted-environments-reference#runner-cli-flags) 세션에 메시지를 보내면 Claude Code는 세션이 있던 권한 모드에서 세션을 재개합니다.

<h3 id="review-changes">
  변경 사항 검토
</h3>

각 세션은 추가 및 제거된 줄을 표시하는 diff 표시기를 표시합니다(예: `+42 -18`). 이를 선택하여 diff 보기를 열고, 특정 줄에 인라인 주석을 남기고, 다음 메시지로 Claude에 보내세요.

diff 보기는 따로 선택하지 않으면 세션의 변경 사항을 해당 기본 분기와 비교합니다. 저장소의 다른 분기와 비교하려면 **Compare against**를 선택하고 분기를 하나 고르세요.

Claude Code는 이러한 diffs(Claude가 편집할 때 표시되는 파일별 diffs 포함)를 raw git blob 콘텐츠에서 계산하므로 저장소에 구성된 diff 드라이버 및 `textconv` 필터는 적용되지 않습니다. 세션의 자체 체크아웃 중 하나가 아닌 저장소의 파일(예: 세션 중에 워크스페이스 내에 복제된 파일)의 경우 파일별 diff는 git 비교가 아니라 Claude의 편집 자체를 표시합니다.

PR 생성을 포함한 전체 안내는 [검토 및 반복](/docs/ko/web-quickstart#review-and-iterate)을 참조하세요. Claude가 PR을 모니터링하여 CI 실패 및 검토 주석에 자동으로 응답하도록 하려면 [Pull request 자동 수정](#auto-fix-pull-requests)을 참조하세요.

<h3 id="share-sessions">
  세션 공유
</h3>

세션을 공유하려면 아래 계정 유형에 따라 가시성을 전환하세요. 그 후 세션 링크를 그대로 공유합니다. 수신자는 링크를 열 때 최신 상태를 보지만 보기가 실시간으로 업데이트되지 않습니다.

<h4 id="share-from-an-enterprise-or-team-account">
  Enterprise 또는 Team 계정에서 공유
</h4>

Enterprise 및 Team 계정의 경우 두 가지 가시성 옵션은 **Private** 및 **Team**입니다. Team 가시성은 claude.ai 조직의 다른 구성원에게 세션을 표시합니다. [Claude in Slack](/docs/ko/slack) 세션은 자동으로 Team 가시성으로 공유됩니다.

저장소 액세스 확인은 기본적으로 수신자의 계정에 연결된 GitHub 계정을 기반으로 활성화됩니다. 계정의 표시 이름은 액세스 권한이 있는 모든 수신자에게 표시됩니다.

<h4 id="share-from-a-max-or-pro-account">
  Max 또는 Pro 계정에서 공유
</h4>

Max 및 Pro 계정의 경우 두 가지 가시성 옵션은 **Private** 및 **Public**입니다. Public 가시성은 claude.ai에 로그인한 모든 사용자에게 세션을 표시합니다.

공유하기 전에 민감한 내용이 있는지 세션을 확인하세요. 세션에는 개인 GitHub 저장소의 코드 및 자격 증명이 포함될 수 있습니다. 저장소 액세스 확인은 기본적으로 활성화되지 않습니다.

저장소 액세스를 요구하거나 공유 세션에서 이름을 숨기려면 [**Settings > Claude Code > Sharing settings**](https://claude.ai/settings/claude-code)로 이동하세요.

<h3 id="archive-sessions">
  세션 보관
</h3>

세션을 보관하여 세션 목록을 정리할 수 있습니다. 보관된 세션은 기본 세션 목록에서 숨겨지지만 보관된 세션을 필터링하여 볼 수 있습니다.

세션을 보관하려면 사이드바의 세션 위에 마우스를 올리고 보관 아이콘을 선택합니다.

<h3 id="delete-sessions">
  세션 삭제
</h3>

세션을 삭제하면 세션과 해당 데이터가 영구적으로 제거됩니다. 이 작업은 실행 취소할 수 없습니다. 두 가지 방법으로 세션을 삭제할 수 있습니다:

* **사이드바에서**: 보관된 세션을 필터링한 다음 삭제할 세션 위에 마우스를 올리고 삭제 아이콘을 선택합니다
* **세션 메뉴에서**: 세션을 열고 세션 제목 옆의 드롭다운을 선택한 다음 **Delete**를 선택합니다

세션이 삭제되기 전에 확인하라는 메시지가 표시됩니다.

<h2 id="auto-fix-pull-requests">
  Pull request 자동 수정
</h2>

Claude는 pull request를 감시하고 CI 실패 및 검토 주석에 자동으로 응답할 수 있습니다. Claude는 PR의 GitHub 활동을 구독하고, 검사가 실패하거나 검토자가 주석을 남기면 Claude가 조사하고 명확한 수정이 있으면 푸시합니다.

<Note>
  자동 수정을 위해서는 Claude GitHub App이 저장소에 설치되어야 합니다. 아직 설치하지 않았으면 [GitHub App 페이지](https://github.com/apps/claude)에서 설치하세요.
</Note>

PR이 어디에서 왔는지와 어떤 기기를 사용하는지에 따라 자동 수정을 켜는 방법은 몇 가지가 있습니다:

* **클라우드 세션에서 생성된 PR**: claude.ai/code에서 세션을 열고, CI 상태 표시줄을 열고, **Auto-fix**를 선택합니다
* **터미널에서**: PR의 분기에 있는 동안 [`/autofix-pr`](/docs/ko/commands)을 실행합니다. Claude Code가 `gh`로 열린 PR을 감지하고, 클라우드 세션을 생성하고, 한 단계에서 자동 수정을 켭니다
* **모바일 앱에서**: Claude에 PR을 자동 수정하도록 지시합니다. 예를 들어 "watch this PR and fix any CI failures or review comments"
* **기존 PR**: PR URL을 세션에 붙여넣고 Claude에 자동 수정하도록 지시합니다

자동 수정은 PR별 토글입니다. 모니터링을 중지하려면 claude.ai/code의 세션에서 CI 상태 표시줄을 열고 **Auto-fix** 토글을 해제하거나, Claude에 PR 감시를 중지하도록 지시합니다.

<h3 id="how-claude-responds-to-pr-activity">
  Claude가 PR 활동에 응답하는 방식
</h3>

자동 수정이 활성화되면 Claude는 새 검토 주석 및 CI 검사 실패를 포함한 PR의 GitHub 이벤트를 수신합니다. 각 이벤트에 대해 Claude는 조사하고 진행 방식을 결정합니다:

* **명확한 수정**: Claude가 수정에 확신하고 이전 지침과 충돌하지 않으면 Claude가 변경을 수행하고, 푸시하고, 세션에서 수행한 작업을 설명합니다
* **모호한 요청**: 검토자의 주석을 여러 방식으로 해석할 수 있거나 아키텍처적으로 중요한 사항이 포함되면 Claude가 행동하기 전에 확인합니다
* **중복 또는 조치 불필요 이벤트**: 이벤트가 중복이거나 변경이 필요 없으면 Claude가 세션에서 이를 기록하고 계속합니다

GitHub는 기본 분기가 진행되고 병합 충돌이 생성될 때 웹훅을 내보내지 않으므로 자동 수정은 충돌에 자체적으로 반응할 수 없습니다. 충돌을 해결하려면 세션을 열고 Claude에 리베이스하도록 요청합니다.

Claude는 GitHub의 검토 주석 스레드에 회신할 수 있습니다. 이러한 회신은 GitHub 계정을 사용하여 게시되므로 사용자 이름 아래에 나타나지만 각 회신은 Claude Code에서 온 것으로 표시되어 검토자가 에이전트에 의해 작성되었으며 직접 작성되지 않았음을 알 수 있습니다.

<Warning>
  저장소가 Atlantis, Terraform Cloud 또는 `issue_comment` 이벤트에서 실행되는 사용자 정의 GitHub Actions와 같은 주석 트리거 자동화를 사용하는 경우 Claude의 회신이 해당 워크플로우를 트리거할 수 있음을 알아두세요. 자동 수정을 활성화하기 전에 저장소의 자동화를 검토하고 PR 주석이 인프라를 배포하거나 권한 있는 작업을 실행할 수 있는 저장소에서는 자동 수정을 비활성화하는 것을 고려하세요.
</Warning>

<h2 id="security-and-isolation">
  보안 및 격리
</h2>

각 클라우드 세션은 여러 계층을 통해 머신과 다른 세션으로부터 분리됩니다:

* **격리된 가상 머신**: 각 세션은 격리된 Anthropic 관리 VM에서 실행됩니다. 조직이 [자체 호스팅 환경](/docs/ko/self-hosted-environments)으로 라우팅하는 세션은 자신의 인프라에서 실행되며, 격리는 배포의 책임입니다
* <span id="default-allowed-domains" />**네트워크 액세스 제어**: Anthropic 호스팅 환경에서 네트워크 액세스는 기본적으로 제한되며 비활성화할 수 있습니다. 액세스 수준, [기본 허용 도메인](/docs/ko/cloud-environments#default-allowed-domains), 그리고 허용 목록을 통과하지 않는 트래픽에 대해서는 [네트워크 액세스](/docs/ko/cloud-environments#network-access)를 참조하세요. 자체 호스팅 환경에서는 자신의 네트워크 경계에서 세션 송신을 제한합니다. 네트워크 액세스가 비활성화된 상태에서 실행할 때 Claude Code는 여전히 Anthropic API와 통신할 수 있으며, 이는 VM에서 데이터가 나갈 수 있습니다.
* **자격 증명 보호**: Anthropic 호스팅 환경에서 git 자격 증명 및 서명 키는 샌드박스 외부에 유지되며, 프록시는 범위 자격 증명으로 세션을 대신하여 인증합니다. 자체 호스팅 환경에서 배포는 git 자격 증명을 제공합니다. [git 구성](/docs/ko/self-hosted-environments-deploy#configure-git)을 참조하세요
* **API 자격 증명**: Anthropic 호스팅 환경의 Pro 및 Max 플랜에서 [클라우드 환경에 추가한](/docs/ko/cloud-environments#add-api-credentials) 키는 샌드박스 외부에 동일한 방식으로 유지되며, 세션을 떠난 후 일치하는 요청에 첨부됩니다. 자체 호스팅 환경에는 API 자격 증명이 없으며 Team 및 Enterprise 플랜에는 아직 없습니다
* **안전한 분석**: 코드는 PR을 생성하기 전에 세션의 격리된 환경 내에서 분석 및 수정됩니다

<h2 id="troubleshooting">
  문제 해결
</h2>

런타임 API 오류가 대화에 나타나면 `API Error: 500`, `529 Overloaded`, `429` 또는 `Prompt is too long`과 같은 경우 [오류 참조](/docs/ko/errors)를 참조하세요. 이러한 오류와 해결 방법은 CLI 및 Desktop 앱과 공유됩니다. 아래 섹션에서는 클라우드 세션에 특정한 문제를 다룹니다.

<h3 id="session-creation-failed">
  세션 생성 실패
</h3>

새 세션이 `Session creation failed`로 시작되지 않거나 프로비저닝에서 정지되면 Claude Code가 세션용 VM을 할당할 수 없습니다.

* [status.claude.com](https://status.claude.com)에서 클라우드 세션 인시던트를 확인하세요
* 용량이 온디맨드로 프로비저닝되므로 1분 후 다시 시도하세요
* GitHub 연결이 저장소에 도달할 수 있는지 확인하세요. [GitHub 연결 후 저장소가 나타나지 않음](/docs/ko/web-quickstart#no-repositories-appear-after-connecting-github)을 따르세요

<h3 id="unable-to-get-organization-uuid">
  조직 UUID를 가져올 수 없음
</h3>

`claude --cloud` 및 `claude --teleport`는 claude.ai 계정으로 로그인해야 합니다. API 키로 인증하거나 저장된 계정 세부 정보가 오래된 경우 이러한 명령은 `Unable to get organization UUID` 또는 API 키 인증이 충분하지 않다는 메시지로 실패합니다. API 키 인증 또는 오래된 계정 세부 정보를 사용하면 세션 ID 없이 `claude --teleport`를 실행하면 세션 선택기에 `Error loading Claude Code sessions`이 표시되고 동일한 수정이 적용됩니다.

`/login`을 실행하여 claude.ai 계정으로 로그인한 다음 명령을 다시 시도하세요. 오류가 제공자의 이름을 지정하면 [오류 표](#output-and-errors)를 참조하세요: 클라우드 세션을 타사 제공자를 통해 사용할 수 없습니다.

<h3 id="remote-control-session-expired-or-access-denied">
  Remote Control 세션 만료 또는 액세스 거부
</h3>

`--teleport`는 클라우드 세션이 사용하는 동일한 Remote Control 세션 인프라를 통해 연결되므로 인증 및 세션 만료 오류는 Remote Control 용어로 표시됩니다. `Remote Control session expired` 또는 `Access denied`가 표시될 수 있습니다. 연결 토큰은 단기이며 계정으로 범위가 지정됩니다.

* 로컬에서 `/login`을 실행하여 자격 증명을 새로 고친 다음 다시 연결하세요
* 세션을 소유한 동일한 계정으로 로그인했는지 확인하세요
* `Remote Control may not be available for this organization`이 표시되면 Owner가 조직에 대해 클라우드 세션을 활성화하지 않았습니다

<h3 id="environment-expired">
  환경 만료
</h3>

클라우드 세션은 비활성 기간 후 중지되고 세션의 VM이 회수됩니다. 세션은 [MCP connector](/docs/ko/cloud-environments#network-access) 도구 호출을 승인하거나 MCP 서버에 로그인하기를 기다리는 동안 비활성으로 간주되며 해당 대기 중에 만료될 수 있습니다.

[claude.ai/code](https://claude.ai/code)에서 세션을 다시 열어 대화 기록이 복원된 새로운 VM을 프로비저닝하세요. VM이 회수되었을 때 여전히 실행 중이던 백그라운드 작업(예: subagents 및 셸 명령)은 복원되지 않습니다.

<h2 id="limitations">
  제한 사항
</h2>

클라우드 세션을 워크플로우에 사용하기 전에 다음 제약 사항을 고려하십시오:

* **속도 제한**: 클라우드 세션은 계정 내의 다른 모든 Claude 및 Claude Code 사용과 속도 제한을 공유합니다. 여러 작업을 병렬로 실행하면 비례적으로 더 많은 속도 제한을 소비합니다. 클라우드 VM에 대한 별도의 컴퓨팅 요금은 없습니다.
* **저장소 인증**: 동일한 계정으로 인증된 경우에만 클라우드 세션을 터미널로 가져올 수 있습니다.
* **플랫폼 제한**: 저장소 복제 및 풀 요청 생성에는 GitHub이 필요합니다. 자체 호스팅 [GitHub Enterprise Server](/docs/ko/github-enterprise-server) 인스턴스는 Team 및 Enterprise 플랜에서 지원됩니다. `CCR_FORCE_BUNDLE=1`을 설정하여 GitLab, Bitbucket 또는 기타 비-GitHub 저장소를 [로컬 번들](#send-local-repositories-without-github)로 클라우드 세션에 전송할 수 있지만, 세션은 결과를 원격으로 다시 푸시할 수 없습니다.
* **조직 IP 허용 목록**: 클라우드 세션은 사용자의 네트워크가 아닌 Anthropic 관리 인프라에서 Anthropic API를 호출하는 반면, [자체 호스팅 환경](/docs/ko/self-hosted-environments)의 세션은 사용자의 네트워크에서 호출합니다. 조직에 [IP 허용 목록](https://support.claude.com/en/articles/13200993-restrict-access-to-claude-with-ip-allowlisting)이 활성화되어 있으면 모든 Anthropic 호스팅 클라우드 세션이 인증 오류로 실패합니다. 이는 [Code Review](/docs/ko/code-review) 및 Anthropic 호스팅 환경에서 실행되는 [routines](/docs/ko/routines)에도 적용됩니다. 자체 호스팅 환경으로 라우팅된 routine은 사용자의 네트워크에서 API를 호출합니다. [Anthropic 지원](https://support.claude.com/)에 문의하여 조직의 IP 허용 목록에서 Anthropic 호스팅 서비스를 제외하십시오.

<h2 id="related-resources">
  관련 리소스
</h2>

* [클라우드 환경](/docs/ko/cloud-environments): 클라우드 세션의 네트워크 액세스, 환경 변수 및 설정 스크립트 구성
* [Projects](/docs/ko/claude-projects): Claude가 리포지토리에서 병렬 클라우드 세션을 조정하고 결과를 보고하는 하나의 대화
* [Ultrareview](/docs/ko/ultrareview): 클라우드 샌드박스에서 심층 다중 에이전트 코드 검토 실행
* [Routines](/docs/ko/routines): 일정에 따라, API 호출을 통해 또는 GitHub 이벤트에 응답하여 작업 자동화
* [Hooks 구성](/docs/ko/hooks): 세션 수명 주기 이벤트에서 스크립트 실행
* [모든 설정](/docs/ko/settings-reference): 모든 구성 옵션
* [보안](/docs/ko/security): 격리 보장 및 데이터 처리
* [데이터 사용](/docs/ko/data-usage): Anthropic이 클라우드 세션에서 보유하는 것
* [Claude Tag](https://claude.com/docs/claude-tag/overview): 동일한 클라우드 인프라에서 실행되는 조직 관리형 Slack의 @Claude
