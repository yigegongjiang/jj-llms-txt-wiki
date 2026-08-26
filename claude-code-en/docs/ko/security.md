> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 보안

> Claude Code의 보안 보호 기능과 안전한 사용을 위한 모범 사례에 대해 알아봅니다.

<h2 id="how-we-approach-security">
  보안에 대한 우리의 접근 방식
</h2>

<h3 id="security-foundation">
  보안 기초
</h3>

코드의 보안은 매우 중요합니다. Claude Code는 보안을 핵심으로 구축되었으며, Anthropic의 포괄적인 보안 프로그램에 따라 개발되었습니다. [Anthropic Trust Center](https://trust.anthropic.com)에서 자세히 알아보고 리소스(SOC 2 Type 2 보고서, ISO 27001 인증서 등)에 접근할 수 있습니다.

<h3 id="permission-based-architecture">
  권한 기반 아키텍처
</h3>

Claude Code는 기본적으로 엄격한 읽기 전용 권한을 사용합니다. 추가 작업이 필요한 경우(파일 편집, 테스트 실행, 명령 실행), Claude Code는 명시적 권한을 요청합니다. 사용자는 작업을 한 번만 승인할지 또는 자동으로 허용할지 제어할 수 있습니다.

Claude Code는 시스템을 수정할 수 있는 Bash 명령을 실행하기 전에 승인을 요구합니다. `ls`, `cat`, `git status`와 같은 기본 제공 [읽기 전용 명령](/docs/ko/permissions#read-only-commands) 집합은 프롬프트 없이 실행됩니다. 이 접근 방식을 통해 사용자와 조직은 권한을 직접 구성할 수 있습니다.

자세한 권한 구성은 [Permissions](/docs/ko/permissions)를 참조하십시오.

<h3 id="built-in-protections">
  기본 제공 보호
</h3>

에이전트 시스템의 위험을 완화하기 위해:

* **샌드박스 bash 도구**: [Sandbox](/docs/ko/sandboxing) bash 명령을 파일 시스템 및 네트워크 격리로 실행하여 권한 프롬프트를 줄이면서 보안을 유지합니다. `/sandbox`를 사용하여 Claude Code가 자율적으로 작업할 수 있는 경계를 정의하도록 활성화합니다.
* **작업 디렉토리 경계**: Claude Code는 시작된 폴더와 그 하위 폴더에만 쓸 수 있으며, 명시적 권한 없이 상위 디렉토리의 파일을 수정할 수 없습니다. Read, Grep, Glob 도구를 사용하여 이 경계 외부의 경로를 읽는 것은 승인 프롬프트 후에 가능합니다. [추가 디렉토리](/docs/ko/permissions#working-directories)로 경계를 확장하여 프롬프트를 건너뛰거나, 샌드박싱이 활성화되었을 때만 적용되는 [sandbox `denyRead` 규칙](/docs/ko/sandboxing#filesystem-isolation)으로 읽기 전용 Bash 명령에 사용 가능한 더 광범위한 읽기 액세스를 제한할 수 있습니다.
* **프롬프트 피로 완화**: 사용자별, 코드베이스별 또는 조직별로 자주 사용되는 안전한 명령을 허용 목록에 추가하는 지원
* **Accept Edits 모드**: 파일 편집을 자동으로 승인하고 작업 디렉토리의 경로에 대해 `mkdir`, `touch`, `rm`, `mv`, `cp`, `sed`와 같은 고정된 파일 시스템 Bash 명령 집합을 자동으로 승인합니다. 다른 Bash 명령과 범위를 벗어난 경로는 여전히 프롬프트를 표시합니다.

<h3 id="user-responsibility">
  사용자 책임
</h3>

Claude Code는 사용자가 부여한 권한만 가집니다. 승인 전에 제안된 코드와 명령의 안전성을 검토할 책임이 있습니다.

<h2 id="protect-against-prompt-injection">
  프롬프트 주입으로부터 보호
</h2>

프롬프트 주입은 공격자가 악의적인 텍스트를 삽입하여 AI 어시스턴트의 지시사항을 무시하거나 조작하려는 기법입니다. Claude Code는 이러한 공격에 대한 여러 보호 기능을 포함합니다:

<h3 id="core-protections">
  핵심 보호
</h3>

* **권한 시스템**: 민감한 작업에는 명시적 승인이 필요합니다.
* **컨텍스트 인식 분석**: 전체 요청을 분석하여 잠재적으로 해로운 지시사항을 감지합니다.
* **입력 살균**: 사용자 입력을 처리하여 명령 주입을 방지합니다.
* **네트워크 명령 승인**: `curl` 및 `wget`과 같이 웹에서 콘텐츠를 가져오는 명령은 기본적으로 자동 승인되지 않습니다. 다른 읽기 전용이 아닌 Bash 명령처럼 프롬프트가 표시되므로 한 번 승인하거나 `Bash(curl *)`와 같은 명시적 허용 규칙을 추가할 수 있습니다. 완전히 차단하려면 [`permissions.deny`](/docs/ko/permissions#tool-specific-permission-rules)에 추가하십시오.

<h3 id="privacy-safeguards">
  개인정보 보호 장치
</h3>

데이터를 보호하기 위해 다음을 포함한 여러 보호 기능을 구현했습니다:

* 민감한 정보에 대한 제한된 보관 기간([Privacy Center](https://privacy.anthropic.com/en/articles/10023548-how-long-do-you-store-my-data)에서 자세히 알아보기)
* 사용자 세션 데이터에 대한 제한된 액세스
* 데이터 학습 기본 설정에 대한 사용자 제어. 소비자 사용자는 언제든지 [개인정보 보호 설정](https://claude.ai/settings/privacy)을 변경할 수 있습니다.

전체 세부 사항은 [Commercial Terms of Service](https://www.anthropic.com/legal/commercial-terms)(Team, Enterprise 및 API 사용자용) 또는 [Consumer Terms](https://www.anthropic.com/legal/consumer-terms)(Free, Pro 및 Max 사용자용) 및 [Privacy Policy](https://www.anthropic.com/legal/privacy)를 검토하십시오.

<h3 id="additional-safeguards">
  추가 보호 기능
</h3>

* **네트워크 요청 승인**: 네트워크 요청을 하는 도구는 기본적으로 사용자 승인이 필요합니다.
* **격리된 컨텍스트 윈도우**: 웹 가져오기는 별도의 컨텍스트 윈도우를 사용하여 잠재적으로 악의적인 프롬프트 주입을 방지합니다.
* **신뢰 확인**: 첫 번째 코드베이스 실행 및 새 MCP 서버는 신뢰 확인이 필요합니다.
  * 참고: `-p` 플래그를 사용하여 비대화형으로 실행할 때 신뢰 확인이 비활성화됩니다.
  * 참고: Claude Code를 홈 디렉토리에서 직접 시작할 때 신뢰 수락은 현재 세션에만 유지되며 디스크에 기록되지 않으므로 각 시작 시 프롬프트가 다시 나타납니다. 이를 유지하는 설정은 없습니다. 신뢰 수락이 디렉토리별로 저장되는 프로젝트 하위 디렉토리에서 Claude Code를 시작하십시오.
* **명령 주입 감지**: 의심스러운 bash 명령은 이전에 허용 목록에 있었더라도 수동 승인이 필요합니다.
* **폐쇄형 매칭 실패**: 일치하지 않는 명령은 기본적으로 수동 승인이 필요합니다.
* **자연어 설명**: 복잡한 bash 명령에는 사용자 이해를 위한 설명이 포함됩니다.
* **보안 자격증명 저장소**: API 키 및 토큰은 사용 가능한 경우 macOS Keychain에 저장되며, Windows 및 Linux에서는 파일 권한으로 보호됩니다. [Credential Management](/docs/ko/authentication#credential-management)를 참조하십시오.

<Warning>
  **Windows WebDAV 보안 위험**: Windows에서 Claude Code를 실행할 때 WebDAV를 활성화하거나 Claude Code가 WebDAV 하위 디렉토리를 포함할 수 있는 `\\*`와 같은 경로에 액세스하도록 허용하지 않는 것이 좋습니다. [WebDAV는 Microsoft에서 보안 위험으로 인해 더 이상 사용되지 않습니다](https://learn.microsoft.com/en-us/windows/whats-new/deprecated-features#:~:text=The%20Webclient%20\(WebDAV\)%20service%20is%20deprecated). WebDAV를 활성화하면 Claude Code가 원격 호스트에 대한 네트워크 요청을 트리거하여 권한 시스템을 우회할 수 있습니다.
</Warning>

**신뢰할 수 없는 콘텐츠로 작업하기 위한 모범 사례**:

1. 승인 전에 제안된 명령 검토
2. 신뢰할 수 없는 콘텐츠를 Claude에 직접 파이프하지 않기
3. 중요한 파일에 대한 제안된 변경 사항 확인
4. 가상 머신(VM)을 사용하여 스크립트를 실행하고 도구 호출을 수행합니다. 특히 외부 웹 서비스와 상호 작용할 때
5. `/feedback`을 사용하여 의심스러운 동작 보고

<Warning>
  이러한 보호 기능이 위험을 크게 줄이지만, 모든 공격에 완전히 면역인 시스템은 없습니다. 모든 AI 도구로 작업할 때 항상 좋은 보안 관행을 유지하십시오.
</Warning>

<h2 id="mcp-security">
  MCP 보안
</h2>

Claude Code를 사용하면 사용자가 Model Context Protocol(MCP) 서버를 구성할 수 있습니다. 허용된 MCP 서버 목록은 소스 코드에서 구성되며, Claude Code 설정의 일부로 엔지니어가 소스 제어에 체크인합니다.

자신의 MCP 서버를 작성하거나 신뢰하는 제공자의 MCP 서버를 사용할 것을 권장합니다. Claude Code 권한을 MCP 서버에 대해 구성할 수 있습니다. Anthropic은 커넥터를 [나열 기준](https://claude.com/docs/connectors/building/review-criteria)에 따라 검토한 후 [Anthropic 디렉토리](https://claude.ai/directory)에 추가하지만, MCP 서버에 대한 보안 감사를 수행하거나 관리하지 않습니다.

<h2 id="ide-security">
  IDE 보안
</h2>

IDE에서 Claude Code를 실행하는 방법에 대한 자세한 내용은 [VS Code 보안 및 개인정보 보호](/docs/ko/vs-code#security-and-privacy)를 참조하십시오.

<h2 id="cloud-execution-security">
  클라우드 실행 보안
</h2>

[웹에서 Claude Code](/docs/ko/claude-code-on-the-web)를 사용할 때 추가 보안 제어가 적용됩니다:

* **격리된 가상 머신**: 각 클라우드 세션은 격리된 Anthropic 관리 VM에서 실행됩니다
* **네트워크 액세스 제어**: 네트워크 액세스는 기본적으로 제한되며 비활성화되거나 특정 도메인만 허용하도록 구성할 수 있습니다
* **자격증명 보호**: 인증은 샌드박스 내에서 범위가 지정된 자격증명을 사용하는 보안 프록시를 통해 처리되며, 이는 실제 GitHub 인증 토큰으로 변환됩니다
* **분기 제한**: Git 푸시 작업은 현재 작업 분기로 제한됩니다
* **감사 로깅**: 클라우드 환경의 모든 작업은 규정 준수 및 감사 목적으로 기록됩니다
* **자동 정리**: 클라우드 환경은 세션 완료 후 자동으로 종료됩니다

클라우드 실행에 대한 자세한 내용은 [Claude Code on the web](/docs/ko/claude-code-on-the-web)을 참조하십시오.

[Remote Control](/docs/ko/remote-control) 세션은 다르게 작동합니다: 웹 인터페이스는 로컬 머신에서 실행 중인 Claude Code 프로세스에 연결됩니다. 모든 코드 실행 및 파일 액세스는 로컬에 유지되며, 세션 트래픽은 TLS를 통해 Anthropic API를 통해 이동합니다. 연결되어 있는 동안 세션 트랜스크립트는 [연결 및 보안](/docs/ko/remote-control#connection-and-security)에 설명된 대로 디바이스 간 대화를 동기화하기 위해 Anthropic 서버에 저장됩니다. 클라우드 VM 또는 샌드박싱이 관련되지 않습니다. 연결은 각각 특정 목적으로 제한되고 독립적으로 만료되는 여러 단기 범위 자격증명을 사용하여 손상된 단일 자격증명의 영향 범위를 제한합니다.

<h2 id="security-best-practices">
  보안 모범 사례
</h2>

<h3 id="working-with-sensitive-code">
  민감한 코드로 작업
</h3>

* 승인 전에 제안된 모든 변경 사항 검토
* 민감한 저장소에 프로젝트별 권한 설정 사용
* 추가 격리를 위해 [dev containers](/docs/ko/devcontainer) 사용 고려
* `/permissions`를 사용하여 권한 설정을 정기적으로 감사합니다

<h3 id="team-security">
  팀 보안
</h3>

* [managed settings](/docs/ko/settings#settings-files)를 사용하여 조직 표준 적용
* 버전 제어를 통해 승인된 권한 구성 공유
* 팀 구성원에게 보안 모범 사례 교육
* [OpenTelemetry metrics](/docs/ko/monitoring-usage)를 통해 Claude Code 사용 모니터링
* [`ConfigChange` hooks](/docs/ko/hooks#configchange)를 사용하여 세션 중 설정 변경 감사 또는 차단

<h3 id="reporting-security-issues">
  보안 문제 보고
</h3>

Claude Code에서 보안 취약점을 발견한 경우:

1. 공개적으로 공개하지 마십시오
2. [HackerOne 프로그램](https://hackerone.com/4f1f16ba-10d3-4d09-9ecc-c721aad90f24/embedded_submissions/new)을 통해 보고합니다
3. 자세한 재현 단계 포함
4. 공개 공개 전에 문제를 해결할 시간을 허용합니다

<h2 id="related-resources">
  관련 리소스
</h2>

* [Security guidance plugin](/docs/ko/security-guidance): Claude가 세션 중에 자신의 코드 변경 사항에서 취약점을 검토하고 수정하도록 합니다
* [Sandbox 환경](/docs/ko/sandbox-environments): 격리 접근 방식을 비교하고 위협 모델에 맞는 방식을 선택합니다
* [Sandboxing](/docs/ko/sandboxing): Bash 명령에 대한 파일 시스템 및 네트워크 격리
* [Permissions](/docs/ko/permissions): 권한 및 액세스 제어를 구성합니다
* [Monitoring usage](/docs/ko/monitoring-usage): Claude Code 활동을 추적하고 감사합니다
* [Development containers](/docs/ko/devcontainer): 보안, 격리된 환경
* [Anthropic Trust Center](https://trust.anthropic.com): 보안 인증 및 규정 준수
