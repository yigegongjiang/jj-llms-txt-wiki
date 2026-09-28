> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 자체 호스팅 환경

> 조직이 제어하는 인프라에서 Claude Code 클라우드 세션을 실행합니다: 자체 호스팅 환경을 설정하고, 러너를 배포하고, 세션을 자신의 컴퓨팅으로 라우팅합니다.

<Note>
  자체 호스팅 환경은 Team 및 Enterprise 플랜에서 공개 베타 상태이며 기본적으로 비활성화되어 있습니다. 활성화 경로 및 제외 사항은 [가용성 및 제한 사항](#availability-and-limitations)을 참조하세요.
</Note>

자체 호스팅 환경은 조직이 운영하는 인프라에서 Claude Code 클라우드 세션을 실행합니다. [클라우드 세션](/docs/ko/claude-code-on-the-web)은 개발자의 머신이 아닌 다른 곳에서 실행되는 모든 세션입니다: 개발자는 claude.ai, 모바일 및 데스크톱 앱, [`claude --cloud`](/docs/ko/claude-code-on-the-web#from-terminal-to-cloud)를 사용한 터미널, [예약된 루틴](/docs/ko/routines)에서 시작하며, 기본적으로 Anthropic의 인프라에서 실행됩니다. 자체 호스팅 환경에서는 동일한 세션이 네트워크 내부에서 실행되며, 개발자 경험은 [가용성 및 제한 사항](#availability-and-limitations)의 차이점과 배포 페이지의 [알려진 문제](/docs/ko/self-hosted-environments-deploy#known-issues-and-limitations)를 제외하고는 동일합니다.

팀이 클라우드 세션을 사용하지 않으면 여기서 구성할 것이 없습니다: 터미널이나 IDE의 세션은 항상 개발자 자신의 머신에서 실행됩니다. Claude Code를 자신의 항상 켜져 있는 머신에서 실행하고 다른 디바이스에서 구동하려면 [원격 제어](/docs/ko/remote-control)를 사용하세요. 이는 Pro 및 Max 플랜에서도 사용 가능합니다. 설정할 준비가 되면 [빠른 시작](/docs/ko/self-hosted-environments-quickstart)으로 바로 이동하세요. 먼저 보안 태세를 검토하려면 [프로덕션에 배포](/docs/ko/self-hosted-environments-deploy)부터 시작하세요. 이 페이지의 나머지 부분은 자체 호스팅이 어떻게 작동하는지와 언제 선택해야 하는지를 설명합니다.

<h2 id="how-self-hosted-environments-work">
  자체 호스팅 환경이 작동하는 방식
</h2>

자체 호스팅에는 세 가지 부분이 있습니다:

* **환경**: 클라우드 세션을 보낼 수 있는 명명된 대상입니다. 조직은 claude.ai 관리 설정에서 환경을 생성하며, 각 환경은 러너 집합을 그룹화합니다.
* **러너**: 네트워크 내부의 호스트에서 실행되는 프로그램입니다. 러너는 세션을 실행합니다. 개념은 자체 호스팅 CI 러너와 동일합니다.
* **세션**: 개발자가 시작한 하나의 Claude Code 작업입니다.

개발자가 클라우드 세션을 시작하면, 세션 시작 UI는 Anthropic 호스팅 환경과 조직이 생성한 환경을 나열하는 환경 선택기를 표시합니다. 조직의 환경을 선택하면, Anthropic의 제어 평면이 세션을 환경의 큐에 배치하고, 러너가 이를 요청하고, 개발자가 선택한 저장소를 복제하고, 호스트에서 Claude Code 프로세스를 시작하여 실행합니다. 러너는 구성한 자격 증명으로 git 호스트에 인증합니다. [git 구성](/docs/ko/self-hosted-environments-deploy#configure-git)에서 옵션을 다룹니다. 세션은 네트워크 내부에서 내부 서비스에 도달하며, 내부인 경우 git 호스트도 동일한 방식으로 도달합니다. Anthropic으로의 트래픽, 큐 폴링, 세션의 이벤트 스트림, 모델 추론은 `api.anthropic.com`으로의 아웃바운드 HTTPS이며, 세션이 도달할 수 있는 추가 호스트의 짧은 목록은 [네트워크 요구 사항](/docs/ko/self-hosted-environments-deploy#network-requirements)에 있습니다. Anthropic은 절대 네트워크에 연결하지 않습니다.

<div style={{maxWidth: "640px", margin: "0 auto"}}>
  <Frame>
    <img src="https://mintcdn.com/claude-code/Y0sJ2uDoOVbOVZrQ/images/self-hosted-network-paths.svg?fit=max&auto=format&n=Y0sJ2uDoOVbOVZrQ&q=85&s=8056103fc1c5564c7f0ef219d260b99d" className="dark:hidden" alt="자체 호스팅 환경의 아키텍처 다이어그램: 네트워크 경계에는 러너, 그 내부의 두 Claude Code 세션 프로세스, 그리고 git 호스트가 포함되어 있으며, api.anthropic.com 외부에는 큐, 세션 스트림, 추론이 있습니다. 러너는 큐를 폴링하고 git 호스트에 도달하며, 각 세션 프로세스는 자신의 스트림, 추론, git 연결을 열고, 모든 연결은 네트워크에서 아웃바운드이며 인바운드는 없습니다." width="680" height="320" data-path="images/self-hosted-network-paths.svg" />

    <img src="https://mintcdn.com/claude-code/Y0sJ2uDoOVbOVZrQ/images/self-hosted-network-paths-dark.svg?fit=max&auto=format&n=Y0sJ2uDoOVbOVZrQ&q=85&s=fec6aef3b0740d80eaf6d6a7000a2233" className="hidden dark:block" alt="자체 호스팅 환경의 아키텍처 다이어그램: 네트워크 경계에는 러너, 그 내부의 두 Claude Code 세션 프로세스, 그리고 git 호스트가 포함되어 있으며, api.anthropic.com 외부에는 큐, 세션 스트림, 추론이 있습니다. 러너는 큐를 폴링하고 git 호스트에 도달하며, 각 세션 프로세스는 자신의 스트림, 추론, git 연결을 열고, 모든 연결은 네트워크에서 아웃바운드이며 인바운드는 없습니다." width="680" height="320" data-path="images/self-hosted-network-paths-dark.svg" />
  </Frame>
</div>

다이어그램의 두 Claude Code 상자는 세션 프로세스입니다: 한 러너가 동시에 두 세션을 실행하며, 구성된 용량까지입니다. 러너는 한 번에 하나의 [소유자](#key-concepts)를 제공하며 첫 번째 세션을 요청할 때 해당 소유자에게 잠깁니다. 따라서 체크아웃된 코드는 소유자 간에 혼합되지 않습니다. [러너 수명 주기](#runner-lifecycle)에서 규칙을 다룹니다.

러너를 직접 시작하고 계속 실행하거나, [자동 스케일링 오케스트레이터](/docs/ko/self-hosted-environments-configuration#on-demand-runners)를 실행할 수 있습니다. 이는 호스팅하는 두 번째 프로세스로, 세션이 큐에 대기할 때 러너를 시작합니다. 각 러너는 작업이 완료되면 자동으로 종료됩니다. 어느 쪽이든, 환경을 한 번 설정하면 지원되는 모든 표면의 선택기에 나타납니다.

<h2 id="availability-and-limitations">
  가용성 및 제한 사항
</h2>

롤아웃을 계획하기 전에 다음을 확인하세요:

* **플랜**: Team 및 Enterprise 조직을 위한 공개 베타입니다. 자체 호스팅 환경은 기본적으로 비활성화되어 있습니다. [소유자](/docs/ko/cloud-environments#organization-shared-environments)가 [**클라우드 환경** 관리 페이지](https://claude.ai/admin-settings/cloud-environments)에서 **자체 호스팅 환경 허용**을 켜야 하며, 이는 조직에 대해 [클라우드 세션](/docs/ko/claude-code-on-the-web)이 활성화되어 있어야 합니다.
* **Zero Data Retention**: [Zero Data Retention](/docs/ko/zero-data-retention)이 활성화된 조직에서는 사용할 수 없습니다.
* **모델 추론**: 세션은 Anthropic API를 사용하며, 추론은 [Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry](/docs/ko/third-party-integrations) 또는 [LLM 게이트웨이](/docs/ko/llm-gateway)를 통해 라우팅될 수 없습니다.
* **표면**: [claude.ai/code](https://claude.ai/code), 모바일 및 데스크톱 앱, [예약된 루틴](/docs/ko/routines), 터미널에서 [`claude --cloud`](/docs/ko/claude-code-on-the-web#from-terminal-to-cloud) 또는 [`--environment` 디스패치](/docs/ko/self-hosted-environments-testing#run-the-test-loop)로 시작된 세션은 자체 호스팅 환경에서 실행될 수 있습니다. [Claude Tag](https://claude.com/docs/claude-tag/overview) 세션도 실행될 수 있지만, Claude는 아직 해당 세션에서 [Access 번들](https://claude.com/docs/claude-tag/concepts/glossary#access-bundle)을 사용할 수 없습니다. [Claude Security](/docs/ko/claude-security) 및 [Code Review](/docs/ko/code-review) 세션은 아직 라우팅되지 않습니다. 이 두 표면에 대한 지원은 별도로 따릅니다.
* **저장소**: 세션은 GitHub에서 저장소를 체크아웃합니다. [GitHub 인증 옵션](/docs/ko/claude-code-on-the-web#github-authentication-options)을 참조하세요.
* **청구**: 자체 호스팅 환경의 세션은 Anthropic 호스팅 환경의 세션과 동일한 방식으로 조직의 Claude Code 사용량을 소비합니다.

<h2 id="why-self-host">
  자체 호스팅을 하는 이유
</h2>

대부분의 팀은 실행하거나 유지할 인프라가 필요 없는 Anthropic 호스팅 환경으로 더 잘 제공됩니다. 자체 호스팅은 네트워크, 도구 또는 규정 준수 요구 사항이 제어하는 인프라에서 세션 실행을 유지해야 하는 팀을 위한 것입니다. 그렇다면 이것이 당신이라면, 그것이 수반하는 운영 소유권을 계획하세요: 러너 이미지를 구축하고 유지하며, 플릿을 운영하고, 네트워크를 제어합니다.

대신, 자체 호스팅은 네트워크 액세스, 사용자 정의 도구 및 규정 준수 제어를 제공합니다:

* **네트워크 액세스**: 세션은 네트워크 내부에서 실행되며 공개 인터넷에 노출하지 않고 내부 서비스, 데이터베이스 및 레지스트리에 도달할 수 있습니다.
* **사용자 정의 도구**: 러너 이미지에 컴파일러, SDK 및 내부 CLI를 사전 설치하여 모든 세션이 빌드할 준비가 된 상태로 시작됩니다.
* **규정 준수**: 저장소 체크아웃 및 빌드 아티팩트는 제어하는 인프라에 남아 있습니다. 세션 콘텐츠는 여전히 모델 추론을 위해 `api.anthropic.com`으로 이동합니다.

<h2 id="environments-runners-and-sessions">
  환경, 러너 및 세션
</h2>

환경은 claude.ai 관리 설정의 **클라우드 환경** 페이지에서 관리되며, 러너는 자신의 인프라에서 시작하고 관리하는 프로세스입니다.

<h3 id="key-concepts">
  주요 개념
</h3>

이 용어들은 자체 호스팅 페이지 전체에 나타납니다:

| 용어     | 정의                                                                                                                            |
| :----- | :---------------------------------------------------------------------------------------------------------------------------- |
| 환경     | claude.ai 설정에서 생성된 러너의 명명된 그룹입니다. 세션은 개별 러너가 아닌 환경으로 라우팅됩니다.                                                                  |
| 환경 시크릿 | 러너가 환경에 인증하고 등록하는 데 사용하는 단일 공유 자격 증명입니다. 환경 생성 시 한 번 표시되며, 관리 UI에서 **환경 키**로 표시됩니다.                                           |
| 러너     | 배포하는 장기 실행 프로세스입니다. 러너는 환경에 등록하고, 러너 토큰을 받고, 세션을 폴링합니다.                                                                       |
| 세션     | claude.ai, 모바일 앱 또는 예약된 루틴이나 에이전트와 같은 다른 Anthropic 표면에서 시작된 하나의 Claude Code 작업입니다. 각 세션은 러너가 생성하는 자식 Claude Code 프로세스로 실행됩니다. |

API 필드, 토큰 클레임 및 메트릭 이름에서 환경은 `pool`로 나타나며, 환경 ID는 `pool_id`입니다. [참조](/docs/ko/self-hosted-environments-reference)는 두 가지 철자를 매핑하며, 더 이상 사용되지 않는 `pool` 플래그 이름을 포함합니다.

러너는 한 번에 하나의 소유자를 제공합니다. 러너가 선택하는 첫 번째 세션이 러너를 해당 세션의 소유자에게 잠그고, 러너는 그 후 구성된 용량까지 해당 소유자의 세션만 실행합니다. 소유자가 누구인지는 세션이 어떻게 시작되었는지에 따라 다릅니다:

* **사용자가 시작한 세션**: 소유자는 해당 사용자의 계정입니다.
* **Claude Tag 채널 세션**: Claude는 사용자 계정이 없는 상태로 실행하므로, 소유자는 세션을 시작한 [Claude Tag 에이전트](https://claude.com/docs/claude-tag/concepts/glossary#agent-identity)입니다. 해당 에이전트가 시작하는 모든 채널 세션은 Slack 메시지를 보낸 사람이 누구든 동일한 소유자를 가지므로, `--capacity` 이상 또는 양수 `--drain-grace-sec`로 실행할 때 러너가 잠긴 경우 다양한 사람들이 시작한 세션을 제공합니다. 사용자에게 잠긴 러너는 이를 선택하지 않으며, Claude Tag 에이전트에게 잠긴 러너는 사용자의 세션을 선택하지 않습니다.

따라서 최소 플릿 크기는 한 번에 활성화될 것으로 예상되는 소유자의 수이며, 사용자와 Claude Tag 에이전트를 계산합니다.

<h3 id="session-lifecycle">
  세션 수명 주기
</h3>

개발자가 세션을 시작하고 환경을 선택하면, Anthropic의 제어 평면이 세션을 환경의 큐에 배치합니다. 거기서:

1. 여유 용량이 있는 러너가 세션을 요청하고 이에 대한 리스를 유지합니다.
2. 러너는 저장소를 작업 디렉토리에 복제하고 자식 Claude Code 프로세스를 생성합니다.
3. 자식은 러너가 계속 폴링하는 동안 HTTPS를 통해 이벤트를 스트리밍합니다. 각 폴링은 리스를 새로 고치고 하트비트로도 작동합니다.
4. 러너가 약 60초 동안 폴링을 중지하면, 서버는 다른 러너를 위해 세션을 다시 큐에 넣습니다.

러너는 각 폴링 요청에 10초를 제공합니다. 요청이 시간 초과되거나, 손실되거나, 러너가 구문 분석할 수 없는 응답을 받으면, 러너는 라이브 세션을 계속 제공하고 다음 예약된 폴링을 기다리지 않고 1\~2초 후에 재시도합니다. 예를 들어, 자신의 페이지로 폴링에 응답하는 인터셉팅 프록시는 러너가 구문 분석할 수 없는 응답을 생성합니다. 이러한 방식 중 하나로 다른 요청이 실패할 때마다, 러너는 다음 재시도 전의 간격을 두 배로 늘리며, 최대 20초까지이며, 리스가 만료될 때까지의 시간이 가까워질 때마다 간격을 단축합니다.

<h3 id="runner-lifecycle">
  러너 수명 주기
</h3>

러너가 선택하는 첫 번째 세션이 러너를 해당 세션의 소유자에게 잠그고, 러너는 해당 소유자를 위해 `--capacity` 동시 세션까지 실행합니다. 러너가 활성 세션을 가지고 있고 종료 신호를 받지 않았거나 은퇴 시간에 도달하지 않은 동안, 러너는 잠긴 소유자의 큐에 있는 작업을 계속 요청합니다. 완료되면 어떻게 되는지는 [`--drain-grace-sec`](/docs/ko/self-hosted-environments-reference#runner-cli-flags)에 따라 다릅니다:

* **기본값 `0`에서**: 러너는 활성 세션이 완료되는 즉시 더 이상 폴링하지 않고 종료되므로, Kubernetes와 같이 배포된 오케스트레이터가 새로운 디스크로 재시작하여 모든 소유자를 제공할 준비가 될 수 있습니다.
* **양수 값에서**: 러너는 종료되기 전에 그 많은 초 동안 잠긴 소유자의 큐를 계속 폴링합니다.

이 수명 주기는 소유자 간에 디스크 상태를 삭제할 필요 없이 각 소유자의 체크아웃된 코드를 격리합니다.

인프라가 러너를 중지하는 방식에 따라 `--retire-at`이 필요한지 결정됩니다. `SIGTERM`을 전달하는 킬은 플래그가 필요하지 않습니다: 러너는 [종료 타이밍](/docs/ko/self-hosted-environments-deploy#shutdown-timing)에서 설명하는 대로 드레인하거나, [`--defer-shutdown-max-min`](/docs/ko/self-hosted-environments-deploy#defer-the-drain-past-the-first-signal)을 설정할 때 이미 보유하고 있는 세션을 계속 제공합니다. 인프라가 대신 신호 없이 또는 샌드박스 수명 상한이나 스팟 인스턴스 회수와 같이 드레인하기에 너무 짧은 유예 기간으로 알려진 벽시계 시간에 호스트를 파괴하는 경우, `--retire-at <epoch-seconds>`를 그 시간 몇 분 전으로 설정하여 전달합니다. 은퇴 시간에:

1. 러너는 새로운 작업을 받지 않습니다.
2. 러너는 [`--release-idle-session-min`](/docs/ko/self-hosted-environments-reference#runner-cli-flags) 플래그가 사용하는 동일한 릴리스 경로를 통해 각 활성 세션을 릴리스하므로, 사용자가 다음 메시지를 보낼 때 세션이 새로운 러너에서 재개됩니다. 러너가 각 세션을 릴리스하는 시기는 상태에 따라 다릅니다:
   * 러너는 턴 중간의 세션을 해당 턴이 완료되는 즉시 릴리스합니다.
   * 턴이 완료되고 백그라운드 작업이 실행 중일 때, 러너는 최대 60초 동안 대기한 후 여전히 실행 중이더라도 세션을 릴리스합니다. 작업이 완료되었지만 결과를 읽는 후속 턴이 아직 실행되지 않은 경우, 러너는 해당 턴이 완료될 때까지 세션을 유지하며, [`SELF_HOSTED_RUNNER_BG_RESULT_GRACE_MS`](/docs/ko/self-hosted-environments-reference#environment-variable-only-settings)보다 오래 해당 턴이 시작될 때까지 대기하지 않습니다.
3. 모든 세션이 릴리스되면 러너는 0으로 종료됩니다.

킬을 초과하는 턴은 여전히 손실됩니다. [종료 타이밍](/docs/ko/self-hosted-environments-deploy#shutdown-timing)은 마진 크기를 다룹니다. `--retire-at` 없이, 신호 없는 호스트 킬은 충돌과 구별할 수 없습니다: 제어 평면은 깨끗한 릴리스가 아닌 손실된 워커를 기록하며, 세션은 다른 러너로 다시 큐에 들어갑니다.

<h3 id="network-paths">
  네트워크 경로
</h3>

러너와 세션은 여러 종류의 아웃바운드 연결을 만들며, Anthropic의 인바운드 연결은 필요하지 않습니다:

* **제어 평면**: 러너는 `api.anthropic.com`을 폴링하여 작업을 수행하고 설정 진행 상황 및 실패 이벤트를 게시하며, 모두 아웃바운드 HTTPS입니다. 폴링은 러너의 하트비트로도 작동합니다.
* **SCM 커넥터**: 선택적 오케스트레이터 [SCM 커넥터](/docs/ko/self-hosted-environments-reference#scm-connector-flags) 터널은 유일한 WebSocket 연결입니다.
* **Git**: 러너는 배포가 제공하는 자격 증명으로 인증하여 HTTPS 또는 SSH를 통해 git 호스트에서 복제하고 푸시합니다. [git 구성](/docs/ko/self-hosted-environments-deploy#configure-git)에서 옵션을 다루며, 세션별 발급 자격 증명 및 [Anthropic git 프록시](/docs/ko/self-hosted-environments-deploy#use-the-anthropic-git-proxy)를 포함하며, 이는 git을 `api.anthropic.com` 대신을 통해 라우팅합니다.
* **세션 자식**: 자식 Claude Code 프로세스는 `api.anthropic.com`에 대한 세션의 이벤트 스트림을 유지하며, 모델 추론 및 세션 중에 실행되는 git 명령에 대한 자신의 아웃바운드 호출을 만듭니다. 전체 이그레스 목록은 [네트워크 요구 사항](/docs/ko/self-hosted-environments-deploy#network-requirements)을 참조하세요. [위의 다이어그램](#how-self-hosted-environments-work)은 선택적 SCM 커넥터를 제외한 이러한 경로를 보여줍니다.

모델 추론은 Anthropic API를 사용합니다. 제어 평면은 각 세션에 API 엔드포인트를 전달하고, 세션은 Anthropic 발급 세션 범위 OAuth 토큰으로 인증하므로, 추론은 [Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry](/docs/ko/third-party-integrations) 또는 [LLM 게이트웨이](/docs/ko/llm-gateway)를 통해 라우팅될 수 없습니다.

기업 이그레스 프록시가 지원됩니다. 러너 및 선택적 [자동 스케일링 오케스트레이터](/docs/ko/self-hosted-environments-configuration#on-demand-runners)는 [네트워크 구성](/docs/ko/network-config)에서 설명하는 프록시 및 mTLS 환경 변수를 준수합니다. 예를 들어 `HTTPS_PROXY` 및 `NO_PROXY`입니다. 각 프로세스의 환경에서 설정합니다. 변수는 제어 평면 호출, 오케스트레이터의 [SCM 커넥터](/docs/ko/self-hosted-environments-reference#scm-connector-flags) WebSocket, HTTPS 리모트에 대한 기본 제공 복제를 다루며, 세션은 러너에서 이를 상속합니다. 세션 스트리밍은 HTTPS를 통한 서버 전송 이벤트를 사용하므로, 경로의 프록시는 응답을 버퍼링하지 않아야 합니다.

프록시가 `Proxy-Authorization` 헤더도 필요로 하는 경우, 러너는 프록시에 열리는 각 연결에 이를 추가할 수 있습니다. [이그레스 프록시에 인증](/docs/ko/self-hosted-environments-deploy#authenticate-to-an-egress-proxy)을 참조하세요.

<h2 id="what-stays-on-your-infrastructure">
  인프라에 남아 있는 것
</h2>

저장소 체크아웃, 빌드 아티팩트, 시크릿 및 세션이 생성하거나 수정하는 모든 파일은 프로비저닝한 머신에 남아 있습니다. 대화 자체(프롬프트, 응답 및 도구 결과 포함)는 모델 추론을 위해 `api.anthropic.com`으로 이동하며, Anthropic은 다른 [지원되는 표면](#availability-and-limitations)에서 세션을 재개할 수 있도록 세션 트랜스크립트를 저장합니다.

자체 호스팅 환경은 세션 실행을 네트워크로 이동합니다. 제어 평면은 Anthropic 호스팅입니다: 세션 오케스트레이션, 큐잉 및 claude.ai 인터페이스는 Anthropic의 인프라에서 계속 실행됩니다.

<h2 id="get-started">
  시작하기
</h2>

자체 호스팅 환경 페이지는 수행 중인 작업별로 구성됩니다:

* [빠른 시작](/docs/ko/self-hosted-environments-quickstart): Claude Code 설치, 환경 생성, 러너 시작 및 첫 번째 세션 라우팅
* [프로덕션에 배포](/docs/ko/self-hosted-environments-deploy): 보안 강화, 네트워크 이그레스, git 자격 증명, Kubernetes 및 Compose 레시피, 알려진 문제 및 문제 해결
* [세션 사용자 정의](/docs/ko/self-hosted-environments-configuration): 세션별 자격 증명, 수명 주기 훅, 온디맨드 러너, MCP 서버 및 권한에 대한 래퍼 스크립트
* [엔드 투 엔드 테스트](/docs/ko/self-hosted-environments-testing): 러너 이미지를 프로모션하기 전에 검증하는 CI 스모크 테스트
* [참조](/docs/ko/self-hosted-environments-reference): 모든 CLI 플래그, 환경 변수, 메트릭 및 상태 엔드포인트
* [세션 ID 검증](/docs/ko/self-hosted-environments-identity): 액세스를 부여하기 전에 자신의 서비스에서 세션 토큰을 검증합니다.
