> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 빠른 시작

> Claude Code에 오신 것을 환영합니다!

이 빠른 시작 가이드를 통해 몇 분 안에 AI 기반 코딩 지원을 사용할 수 있습니다. 이 가이드를 마치면 일반적인 개발 작업에 Claude Code를 사용하는 방법을 이해하게 됩니다.

<h2 id="before-you-begin">
  시작하기 전에
</h2>

다음을 확인하십시오:

* 열려 있는 터미널 또는 명령 프롬프트
  * 터미널을 처음 사용하는 경우 [터미널 가이드](/docs/ko/terminal-guide)를 확인하십시오
* 작업할 코드 프로젝트
* [Claude 구독](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_prereq) (Pro, Max, Team 또는 Enterprise), [Claude Console](https://console.anthropic.com/) 계정 또는 [지원되는 클라우드 제공자](/docs/ko/third-party-integrations)를 통한 액세스

<Note>
  이 가이드는 터미널 CLI를 다룹니다. Claude Code는 [웹](https://claude.ai/code), [데스크톱 앱](/docs/ko/desktop), [VS Code](/docs/ko/vs-code) 및 [JetBrains IDE](/docs/ko/jetbrains), [Slack](/docs/ko/slack), [GitHub Actions](/docs/ko/github-actions) 및 [GitLab](/docs/ko/gitlab-ci-cd)의 CI/CD에서도 사용할 수 있습니다. [모든 인터페이스](/docs/ko/overview#use-claude-code-everywhere)를 참조하십시오.
</Note>

<h2 id="step-1-install-claude-code">
  단계 1: Claude Code 설치
</h2>

To install Claude Code, use one of the following methods:

<Tabs>
  <Tab title="Native Install (Recommended)">
    **macOS, Linux, WSL:**

    ```bash theme={null}
    curl -fsSL https://claude.ai/install.sh | bash
    ```

    **Windows PowerShell:**

    ```powershell theme={null}
    irm https://claude.ai/install.ps1 | iex
    ```

    **Windows CMD:**

    ```batch theme={null}
    curl -fsSL https://claude.ai/install.cmd -o install.cmd && install.cmd && del install.cmd
    ```

    If you see `The token '&&' is not a valid statement separator`, you're in PowerShell, not CMD. If you see `'irm' is not recognized as an internal or external command`, you're in CMD, not PowerShell. Your prompt shows `PS C:\` when you're in PowerShell and `C:\` without the `PS` when you're in CMD.

    If the install command fails with `syntax error near unexpected token '<'`, a `403`, or another curl error, see [Troubleshoot installation](/docs/en/troubleshoot-install#find-your-error) to match the error to a fix and for alternative install methods.

    [Git for Windows](https://git-scm.com/downloads/win) is recommended on native Windows so Claude Code can use the Bash tool. If Git for Windows is not installed, Claude Code uses PowerShell as the shell tool instead. WSL setups do not need Git for Windows.

    <Info>
      Native installations automatically update in the background to keep you on the latest version.
    </Info>
  </Tab>

  <Tab title="Homebrew">
    ```bash theme={null}
    brew install --cask claude-code
    ```

    Homebrew offers two casks. `claude-code` tracks the stable release channel, which is typically about a week behind and skips releases with major regressions. `claude-code@latest` tracks the latest channel and receives new versions as soon as they ship.

    <Info>
      Homebrew installations do not auto-update. Run `brew upgrade claude-code` or `brew upgrade claude-code@latest`, depending on which cask you installed, to get the latest features and security fixes.
    </Info>
  </Tab>

  <Tab title="WinGet">
    ```powershell theme={null}
    winget install Anthropic.ClaudeCode
    ```

    <Info>
      WinGet installations do not auto-update. Run `winget upgrade Anthropic.ClaudeCode` periodically to get the latest features and security fixes.
    </Info>
  </Tab>
</Tabs>

You can also install with [apt, dnf, or apk](/docs/en/setup#install-with-linux-package-managers) on Debian, Fedora, RHEL, and Alpine.

<h2 id="step-2-log-in-to-your-account">
  단계 2: 계정에 로그인
</h2>

Claude Code를 사용하려면 계정이 필요합니다. `claude` 명령으로 대화형 세션을 시작하면 처음 사용할 때 로그인하라는 메시지가 표시됩니다:

```bash theme={null}
claude
```

Claude 구독 또는 Console 계정의 경우 프롬프트를 따라 브라우저에서 인증을 완료하십시오. 나중에 계정을 전환하거나 다시 인증하려면 실행 중인 세션 내에서 `/login`을 입력하십시오:

```text theme={null}
/login
```

다음 계정 유형 중 하나를 사용하여 로그인할 수 있습니다:

* [Claude Pro, Max, Team 또는 Enterprise](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=quickstart_login) (권장)
* [Claude Console](https://console.anthropic.com/) (선불 크레딧이 있는 API 액세스). 처음 로그인할 때 비용 추적을 위해 Console에서 "Claude Code" 워크스페이스가 자동으로 생성됩니다.
* [Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry](/docs/ko/third-party-integrations) (엔터프라이즈 클라우드 제공자)
* 조직에서 운영하는 자체 호스팅 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway): 관리자가 게이트웨이 URL을 미리 구성하고, `/login`을 입력하면 **Cloud gateway** 화면에서 직접 열려 기업 SSO로 로그인할 수 있습니다.

로그인하면 자격 증명이 저장되고 다시 로그인할 필요가 없습니다.

<h2 id="step-3-start-your-first-session">
  단계 3: 첫 번째 세션 시작
</h2>

프로젝트 디렉토리에서 터미널을 열고 Claude Code를 시작하십시오:

```bash theme={null}
cd /path/to/your/project
claude
```

버전, 현재 모델 및 작업 디렉토리가 표시된 Claude Code 프롬프트가 나타납니다. 사용 가능한 명령을 보려면 `/help`를 입력하거나 이전 대화를 계속하려면 `/resume`을 입력하십시오.

<Tip>
  로그인(단계 2) 후 자격 증명이 시스템에 저장됩니다. [자격 증명 관리](/docs/ko/authentication#credential-management)에서 자세히 알아보십시오.
</Tip>

<h2 id="step-4-ask-your-first-question">
  단계 4: 첫 번째 질문 하기
</h2>

코드베이스를 이해하는 것부터 시작하겠습니다. 다음 명령 중 하나를 시도하십시오:

```text theme={null}
이 프로젝트는 무엇을 하나요?
```

Claude가 파일을 분석하고 요약을 제공합니다. 더 구체적인 질문을 할 수도 있습니다:

```text theme={null}
이 프로젝트는 어떤 기술을 사용하나요?
```

```text theme={null}
주요 진입점은 어디인가요?
```

```text theme={null}
폴더 구조를 설명해주세요
```

Claude의 기능에 대해 물어볼 수도 있습니다:

```text theme={null}
Claude Code는 무엇을 할 수 있나요?
```

```text theme={null}
Claude Code에서 사용자 정의 skills를 만드는 방법은?
```

```text theme={null}
Claude Code는 Docker와 함께 작동할 수 있나요?
```

<Note>
  Claude Code는 필요에 따라 프로젝트 파일을 읽습니다. 수동으로 컨텍스트를 추가할 필요가 없습니다.
</Note>

<h2 id="step-5-make-your-first-code-change">
  단계 5: 첫 번째 코드 변경 수행
</h2>

이제 Claude Code가 실제 코딩을 하도록 해봅시다. 간단한 작업을 시도하십시오:

```text theme={null}
주 파일에 hello world 함수 추가
```

Claude Code는 다음을 수행합니다:

1. 적절한 파일 찾기
2. 제안된 변경 사항 표시
3. 승인 요청
4. 편집 수행

<Note>
  Claude Code는 파일을 수정하기 전에 항상 권한을 요청합니다. 개별 변경 사항을 승인하거나 세션에 대해 "모두 수락" 모드를 활성화할 수 있습니다.
</Note>

<h2 id="step-6-use-git-with-claude-code">
  단계 6: Claude Code와 함께 Git 사용
</h2>

Claude Code는 Git 작업을 대화형으로 만듭니다:

```text theme={null}
어떤 파일을 변경했나요?
```

```text theme={null}
설명적인 메시지로 변경 사항 커밋
```

더 복잡한 Git 작업을 요청할 수도 있습니다:

```text theme={null}
feature/quickstart라는 새 브랜치 생성
```

```text theme={null}
마지막 5개의 커밋 표시
```

```text theme={null}
병합 충돌을 해결하는 데 도움을 주세요
```

<h2 id="step-7-fix-a-bug-or-add-a-feature">
  단계 7: 버그 수정 또는 기능 추가
</h2>

Claude는 디버깅 및 기능 구현에 능숙합니다.

자연어로 원하는 것을 설명하십시오:

```text theme={null}
사용자 등록 양식에 입력 유효성 검사 추가
```

또는 기존 문제를 수정하십시오:

```text theme={null}
사용자가 빈 양식을 제출할 수 있는 버그가 있습니다 - 수정하세요
```

Claude Code는 다음을 수행합니다:

* 관련 코드 찾기
* 컨텍스트 이해
* 솔루션 구현
* 사용 가능한 경우 테스트 실행

<h2 id="step-8-test-out-other-common-workflows">
  단계 8: 다른 일반적인 워크플로우 시도
</h2>

Claude와 함께 작업하는 여러 가지 방법이 있습니다:

**코드 리팩토링**

```text theme={null}
인증 모듈을 콜백 대신 async/await를 사용하도록 리팩토링
```

**테스트 작성**

```text theme={null}
계산기 함수에 대한 단위 테스트 작성
```

**문서 업데이트**

```text theme={null}
설치 지침으로 README 업데이트
```

**코드 검토**

```text theme={null}
내 변경 사항을 검토하고 개선 사항을 제안해주세요
```

<Tip>
  도움이 되는 동료처럼 Claude와 대화하십시오. 달성하고 싶은 것을 설명하면 도움을 드릴 것입니다.
</Tip>

<h2 id="essential-commands">
  필수 명령
</h2>

일상적인 사용을 위한 가장 중요한 명령은 다음과 같습니다. 셸 명령은 Claude Code를 시작하거나 재개하기 위해 터미널에서 실행됩니다. 세션 명령은 Claude Code가 시작된 후 내부에서 실행됩니다.

**셸 명령**

| 명령                  | 기능                    | 예시                                  |
| ------------------- | --------------------- | ----------------------------------- |
| `claude`            | 대화형 모드 시작             | `claude`                            |
| `claude "task"`     | 일회성 작업 실행             | `claude "fix the build error"`      |
| `claude -p "query"` | 일회성 쿼리 실행 후 종료        | `claude -p "explain this function"` |
| `claude -c`         | 현재 디렉토리에서 가장 최근 대화 계속 | `claude -c`                         |
| `claude -r`         | 이전 대화 재개              | `claude -r`                         |

**세션 명령**

| 명령                | 기능             | 예시       |
| ----------------- | -------------- | -------- |
| `/clear`          | 대화 기록 지우기      | `/clear` |
| `/help`           | 사용 가능한 명령 표시   | `/help`  |
| `/exit` 또는 Ctrl+D | Claude Code 종료 | `/exit`  |

전체 셸 명령 목록은 [CLI 참조](/docs/ko/cli-reference)를 참조하고 전체 세션 명령 목록은 [명령 참조](/docs/ko/commands)를 참조하십시오.

<h2 id="pro-tips-for-beginners">
  초보자를 위한 팁
</h2>

자세한 내용은 [모범 사례](/docs/ko/best-practices) 및 [일반적인 워크플로우](/docs/ko/common-workflows)를 참조하십시오.

<AccordionGroup>
  <Accordion title="요청을 구체적으로 하기">
    대신: "버그 수정"

    시도: "사용자가 잘못된 자격 증명을 입력한 후 빈 화면을 보는 로그인 버그 수정"
  </Accordion>

  <Accordion title="단계별 지침 사용">
    복잡한 작업을 단계로 나누기:

    ```text theme={null}
    1. 사용자 프로필을 위한 새 데이터베이스 테이블 생성
    2. 사용자 프로필을 가져오고 업데이트하는 API 엔드포인트 생성
    3. 사용자가 자신의 정보를 보고 편집할 수 있는 웹페이지 구축
    ```
  </Accordion>

  <Accordion title="Claude가 먼저 탐색하도록 하기">
    변경하기 전에 Claude가 코드를 이해하도록 하기:

    ```text theme={null}
    데이터베이스 스키마 분석
    ```

    ```text theme={null}
    영국 고객이 가장 자주 반품하는 제품을 보여주는 대시보드 구축
    ```
  </Accordion>

  <Accordion title="바로가기로 시간 절약">
    * `/`를 입력하여 모든 명령 및 skills 보기
    * Tab을 사용하여 명령 완성
    * ↑를 눌러 명령 기록 보기
    * `Shift+Tab`을 눌러 권한 모드 순환
  </Accordion>
</AccordionGroup>

<h2 id="what’s-next">
  다음 단계
</h2>

이제 기본 사항을 배웠으므로 더 고급 기능을 살펴보십시오:

<CardGroup cols={2}>
  <Card title="Claude Code 작동 방식" icon="microchip" href="/docs/ko/how-claude-code-works">
    에이전트 루프, 기본 제공 도구 및 Claude Code가 프로젝트와 상호 작용하는 방식 이해
  </Card>

  <Card title="모범 사례" icon="star" href="/docs/ko/best-practices">
    효과적인 프롬프팅 및 프로젝트 설정으로 더 나은 결과 얻기
  </Card>

  <Card title="일반적인 워크플로우" icon="graduation-cap" href="/docs/ko/common-workflows">
    일반적인 작업에 대한 단계별 가이드
  </Card>

  <Card title="Claude Code 확장" icon="puzzle-piece" href="/docs/ko/features-overview">
    CLAUDE.md, skills, hooks, MCP 등으로 사용자 정의
  </Card>
</CardGroup>

<h2 id="getting-help">
  도움 받기
</h2>

* **Claude Code에서**: `/help`를 입력하거나 "어떻게..."를 물어보기
* **문서**: 여기 있습니다! 다른 가이드 찾아보기
* **커뮤니티**: 팁과 지원을 위해 [Discord](https://www.anthropic.com/discord)에 참여하기
