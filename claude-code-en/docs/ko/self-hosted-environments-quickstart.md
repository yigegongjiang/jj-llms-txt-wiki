> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 자체 호스팅 환경 빠른 시작

> 첫 번째 자체 호스팅 환경 설정: Claude Code 설치, 환경 생성, 러너 시작, 세션 라우팅.

<Note>
  자체 호스팅 환경은 Team 및 Enterprise 플랜에서 공개 베타 상태입니다. [가용성 및 제한 사항](/docs/ko/self-hosted-environments#availability-and-limitations)에서 활성화 경로를 확인하세요. 이 페이지는 첫 번째 세션을 실행하는 방법을 설명합니다. [자체 호스팅 환경](/docs/ko/self-hosted-environments)에서 자체 호스팅 환경이 무엇인지 확인하고, [프로덕션에 배포](/docs/ko/self-hosted-environments-deploy)에서 강화 및 플릿 레시피를 확인하세요.
</Note>

[자체 호스팅 환경](/docs/ko/self-hosted-environments)은 조직이 운영하는 인프라에서 Claude Code [클라우드 세션](/docs/ko/claude-code-on-the-web)을 실행하며, 배포한 러너 프로세스에 의해 실행됩니다. 이 빠른 시작은 가장 작은 규모의 첫 번째 환경을 설정합니다. 단일 호스트에서 하나의 러너가 하나의 테스트 세션을 실행합니다. 두 가지 단계가 있습니다. [환경 생성, 러너 시작, 세션 라우팅](#set-up-an-environment-and-runner), 그 다음 [실행 중인 세션에 후속 메시지 전송](#send-a-follow-up-message-to-a-running-session). 두 가지 인터페이스 간에 이동합니다. claude.ai에서 환경을 생성하고, 상태를 확인하고, 세션을 라우팅하며, 호스트의 터미널에서 러너가 수행하는 모든 작업을 수행합니다.

완료되면 [**클라우드 환경** 관리자 페이지](https://claude.ai/admin-settings/cloud-environments)에 환경이 있고, 러너가 작업을 폴링하고 있으며, 호스트에서 세션이 실행 중입니다. 실제 저장소나 내부 시스템을 연결하기 전에 [프로덕션에 배포](/docs/ko/self-hosted-environments-deploy)를 진행하세요. 이 페이지에서는 보안 태세, 송신 제어, git 자격 증명 및 오케스트레이션을 다룹니다.

<h2 id="prerequisites">
  필수 조건
</h2>

<h3 id="organization-and-roles">
  조직 및 역할
</h3>

claude.ai 측에서 필요한 사항:

* **자체 호스팅 환경 허용**이 [**클라우드 환경** 관리자 페이지](https://claude.ai/admin-settings/cloud-environments)에서 [Owner](/docs/ko/cloud-environments#organization-shared-environments)에 의해 활성화되어야 합니다. **새로 만들기** 버튼은 활성화될 때까지 나타나지 않습니다. 이 역할을 보유하지 않은 경우, 역할을 보유한 사람이 환경을 생성하고 해당 비밀을 제공할 수 있습니다. 이 페이지의 러너 및 터미널 단계에는 claude.ai 역할이 필요하지 않으며, 단계에서 관리자 UI의 상태를 확인하는 경우, 러너의 자체 로그 라인이 동일한 신호를 제공합니다.
* 조직의 [GitHub 연결](/docs/ko/claude-code-on-the-web#github-authentication-options)이 필요하므로 개발자가 세션을 시작할 때 저장소를 선택할 수 있습니다.

<h3 id="host-and-network">
  호스트 및 네트워크
</h3>

러너 호스트에 필요한 사항:

* `api.anthropic.com`, `claude.ai` 및 아래 설치 단계를 위해 리디렉션되는 다운로드 호스트, 그리고 클론을 위해 git 호스트로의 아웃바운드 HTTPS를 사용하는 Linux 또는 macOS 호스트 또는 컨테이너입니다. [네트워크 요구 사항 표](/docs/ko/self-hosted-environments-deploy#network-requirements)에 전체 목록이 있습니다. Windows는 러너 호스트로 지원되지 않습니다. 대신 Linux 컨테이너에서 러너를 실행하세요. 개발자 워크스테이션은 영향을 받지 않습니다. 세션은 브라우저의 claude.ai에서 시작되기 때문입니다.
* NTP와 같은 실시간으로 동기화된 시계입니다. 시계가 5분 이상 차이나면 인증이 실패합니다. [문제 해결](/docs/ko/self-hosted-environments-deploy#troubleshooting)을 참조하세요.

<h3 id="software-on-the-runner-host">
  러너 호스트의 소프트웨어
</h3>

시작하기 전에 호스트에 설치하세요:

* **Claude Code v2.1.224 이상**, [표준 설치 방법](/docs/ko/setup) 중 하나를 사용합니다. 러너는 표준 `claude` 바이너리의 일부이며, 이전 버전은 `self-hosted-runner` 서브 명령을 인식하지 못합니다. 네이티브 설치 프로그램의 기본 `latest` 채널은 각 릴리스를 게시되는 즉시 제공합니다. `stable` 채널, Homebrew `claude-code` cask, 그리고 안정적인 apt, dnf, apk 저장소는 약 1주일 뒤에 제공됩니다. 플릿이 실행하는 정확한 버전을 고정하려면 [특정 버전 설치](/docs/ko/setup#install-a-specific-version)를 참조하세요. 컨테이너 이미지의 경우 [프로덕션에 배포](/docs/ko/self-hosted-environments-deploy#build-the-runner-image)의 Dockerfile을 참조하세요.
* **Git 2.24 이상**. 배포 페이지의 일부 git 옵션에는 최신 버전이 필요합니다. [Git 구성](/docs/ko/self-hosted-environments-deploy#configure-git)에서 각 하한을 명시합니다.

호스트가 준비되었는지 확인하세요:

```bash theme={null}
claude self-hosted-runner --help
```

준비된 호스트는 `--environment-secret-file`과 같은 플래그를 나열하는 러너의 사용 텍스트를 인쇄합니다. 2.1.224보다 이전 버전에서는 명령이 대신 일반 `claude --help` 출력을 인쇄합니다. `claude update`로 업그레이드하거나 `latest` 채널에서 다시 설치하세요.

<h2 id="set-up-an-environment-and-runner">
  환경 및 러너 설정
</h2>

Claude Code에는 안내식 설정이 포함되어 있습니다. 관리자 UI에서 환경 생성을 안내하는 대화형 Claude Code 세션으로, 저장한 비밀 파일로 로컬 러너를 시작하고, 러너가 등록되었는지 확인하고, `./runner-setup/CHEAT-SHEET.md`에 치트 시트를 작성합니다. Owner 역할을 보유한 계정으로 `claude auth login`을 사용하여 로그인한 머신에서 실행하세요. API 키 또는 타사 모델 제공자와는 사용할 수 없습니다. 대화형 세션이 불가능한 호스트에서는 대신 아래의 수동 단계를 사용하세요. [버전 확인](#software-on-the-runner-host)이 통과했는지 확인하세요. 2.1.224보다 이전 버전에서는 이 명령이 안내식 설정 대신 단어를 프롬프트로 하는 일반 Claude 세션을 시작합니다. 안내식 설정을 시작하려면 설정 서브 명령을 실행하고 프롬프트를 따르세요:

```bash theme={null}
claude self-hosted-runner setup
```

대신 수동으로 설정하려면:

<Steps>
  <Step title="환경 생성">
    관리자 설정의 [**클라우드 환경** 페이지](https://claude.ai/admin-settings/cloud-environments)로 이동하세요. **자체 호스팅 환경** 아래에서 **새로 만들기**를 선택하고, 환경의 이름을 지정한 후 **생성**을 선택하세요. 마법사의 두 번째 단계에서 **환경 키 복사**를 선택하여 환경 비밀을 복사하세요. 관리자 UI는 이를 환경 키로 표시합니다. claude.ai는 비밀을 한 번만 표시하며, 나중에 검색할 수 없습니다. 생성 후 365일 후에 만료됩니다. 환경의 `ccpool_...` ID는 세부 정보 대화 상자에서 계속 표시됩니다. [토큰 검증](/docs/ko/self-hosted-environments-identity)의 `aud` 확인 및 [CI에서 테스트 세션 디스패치](/docs/ko/self-hosted-environments-testing#run-the-test-loop)에 필요합니다.

    비밀을 잃어버렸거나 회전해야 하는 경우, 환경의 **구성** 탭에서 새 비밀을 생성하고, 새 비밀을 러너에 배포한 후, 이전 비밀을 취소하세요. 취소된 비밀을 보유한 러너는 다음 인증된 폴에서 실패하고 종료되며, `poll auth failed`를 기록하고, 오케스트레이터가 새 비밀로 다시 시작합니다.
  </Step>

  <Step title="러너 시작">
    비밀 디렉토리를 생성하세요. 이 단계와 다음 단계에는 `/etc/claude` 경로에 대한 루트가 필요합니다. 러너 프로세스가 읽을 수 있는 모든 경로가 작동하므로, 다른 경로를 사용하는 경우 두 명령과 `--environment-secret-file` 값을 함께 조정하세요.

    ```bash theme={null}
    mkdir -p /etc/claude
    ```

    환경 비밀을 파일에 작성하세요. 아래 명령은 터미널에서 읽으므로 비밀이 셸 히스토리에서 벗어납니다. 복사한 값을 붙여넣고, Enter를 누른 후 Ctrl-D를 누르면, 서브셸의 `umask`가 파일을 소유자만 읽을 수 있도록 만듭니다.

    ```bash theme={null}
    (umask 077 && cat > /etc/claude/environment-secret)
    ```

    기본 디렉토리를 선택하고, 아래 러너 명령의 `<writable-dir>`을 러너가 쓸 수 있거나 생성할 수 있는 절대 경로로 바꾸세요. 러너는 시작 시 디렉토리를 생성한 후, 저장소를 체크아웃하고 그 아래에 세션별 디렉토리를 생성합니다. `--base-dir` 없이는 `/workspace`를 사용하며, 이는 해당 디렉토리가 이미 존재하고 쓸 수 있거나 러너를 루트로 시작하는 경우에만 작동합니다.

    러너가 경로를 생성하거나 쓸 수 없으면, 등록하지 않고 시작 시 디렉토리의 이름을 지정하는 오류로 종료됩니다. [문제 해결](/docs/ko/self-hosted-environments-deploy#troubleshooting)을 참조하세요.

    그런 다음 `--environment-secret-file` 및 `--base-dir`로 러너를 시작하세요. 러너는 환경에 등록하고 작업을 폴링하기 시작합니다. 러너가 종료되면 수동으로 다시 시작하세요. 프로덕션 배포는 종료된 러너를 다시 시작하는 오케스트레이터 아래에서 러너를 실행하며, 일반적으로 재시작당 새로운 파일 시스템을 사용합니다. [사전 준비된 체크아웃 재사용](/docs/ko/self-hosted-environments-deploy#reuse-a-pre-warmed-checkout)은 지원되는 영구 디스크 설정을 다룹니다.

    ```bash theme={null}
    claude self-hosted-runner --environment-secret-file '/etc/claude/environment-secret' --base-dir '<writable-dir>'
    ```
  </Step>

  <Step title="러너가 나타나는지 확인">
    [**클라우드 환경** 페이지](https://claude.ai/admin-settings/cloud-environments)로 돌아가세요. 환경의 상태는 러너 시작 후 몇 초 내에 **배포된 러너 없음**에서 **정상**으로 변경됩니다. 환경을 열고 **활동**을 선택하여 러너 자체를 확인하세요.
  </Step>

  <Step title="세션을 환경으로 라우팅">
    claude.ai/code에서 세션을 시작하고 환경 선택기에서 환경을 선택하세요. 자체 호스팅 환경은 Anthropic 호스팅 환경과 함께 나타납니다. 러너는 호스트가 이미 가지고 있는 git 자격 증명으로 클론하므로, 이 호스트가 이미 클론할 수 있는 저장소 또는 공개 저장소를 선택하세요. 프로덕션의 비공개 저장소에 대한 자격 증명 옵션은 [Git 구성](/docs/ko/self-hosted-environments-deploy#configure-git)에 있습니다. 다음 사용 가능한 러너가 대기 중인 세션을 선택하고 `Picked up session <session-id>`를 기록하며, 활성 개수 및 용량을 함께 기록하므로, 러너의 자체 출력에서 어느 호스트가 세션을 선택했는지 확인할 수 있습니다. [claude.ai/code](https://claude.ai/code)에서 세션이 작동하는 것을 보고 Claude의 답변을 읽으세요. 세션이 대기 중인 경우 [문제 해결](/docs/ko/self-hosted-environments-deploy#troubleshooting)을 참조하세요.
  </Step>
</Steps>

러너는 활성 세션이 완료되면 설계상 종료됩니다. [러너 수명 주기](/docs/ko/self-hosted-environments#runner-lifecycle)를 참조하세요. 프로덕션의 경우, 종료 시 다시 시작하는 오케스트레이터 아래에 배포하세요. [프로덕션에 배포](/docs/ko/self-hosted-environments-deploy)를 참조하세요.

<h2 id="send-a-follow-up-message-to-a-running-session">
  실행 중인 세션에 후속 메시지 전송
</h2>

세션이 환경에서 실행 중이면, `claude auth login`으로 로그인한 모든 머신의 `claude` CLI에서 후속 메시지를 전송하세요. 명령은 세션을 시작한 머신에서 실행할 필요가 없습니다. 명령은 하나의 메시지를 게시합니다:

```bash theme={null}
claude -p "your message" --cloud <session-id>
```

`<session-id>`의 경우, 베어 `session_...` 또는 `cse_...` ID 또는 세션의 claude.ai/code URL을 전달하세요. 성공적인 전송은 세션 ID 및 보기 링크와 함께 `Sent to cloud session.`을 인쇄합니다. 수락된 ID 형식, JSON 출력, 계정 및 정책 요구 사항, 오류 참조는 [CLI에서 후속 메시지 전송](/docs/ko/claude-code-on-the-web#send-follow-ups-from-the-cli)에 있습니다. 명령은 Anthropic 호스팅 세션에 대해 동일하게 작동하기 때문입니다.

<h2 id="what’s-next">
  다음 단계
</h2>

* [프로덕션에 배포](/docs/ko/self-hosted-environments-deploy): 배포를 강화하고, 송신을 제어하고, git 자격 증명을 구성하고, Kubernetes 또는 Compose 아래에서 플릿을 실행합니다.
* [세션 사용자 정의](/docs/ko/self-hosted-environments-configuration): 래퍼 스크립트, 수명 주기 훅, 온디맨드 러너, MCP 서버, 권한
* [엔드 투 엔드 테스트](/docs/ko/self-hosted-environments-testing): 세션을 디스패치하고 Claude의 답변을 읽는 CI 스모크 테스트
