> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 기능 가용성

> Anthropic 구독 플랜, Anthropic Console, Amazon Bedrock, AWS의 Claude Platform, Google Cloud의 Agent Platform, Microsoft Foundry에서 사용 가능한 Claude Code 기능을 비교합니다.

Claude Code CLI와 로컬에서 실행되는 모든 것은 모든 제공자에서 동일하게 작동합니다. 제공자별 설정 지침은 [엔터프라이즈 배포 개요](/docs/ko/third-party-integrations)를 참조하십시오. 제공자에서 누락된 기능을 바로 확인하려면 [제공자별 요약](#summary-by-provider) 탭을 참조하십시오.

아래 표에서 ✓는 사용 가능, ✗는 사용 불가능, "참고 사항"은 부분 지원에 대한 각주로 연결됩니다. ✓ 뒤의 한정자는 가용성을 해당 부분 집합으로 좁히며, "관리자 활성화"는 조직 관리자가 기능을 켤 때까지 기능이 꺼져 있음을 의미합니다.

<h2 id="availability-by-model-provider">
  모델 제공자별 가용성
</h2>

인증 방식에 따라 Claude Code가 도달할 수 있는 기능이 결정됩니다. 제공자에서 누락된 기능의 단일 목록은 [제공자별 요약](#summary-by-provider) 탭을 참조하십시오. 표에서 열을 찾으려면:

* **Claude 구독**: claude.ai 계정으로 Pro, Max, Team 또는 Enterprise 플랜에 로그인합니다.
* **Anthropic Console**: Anthropic API 키로 인증합니다.
* **Amazon Bedrock**: Amazon Bedrock 모델 카탈로그에서 Claude 모델을 사용하고 `CLAUDE_CODE_USE_BEDROCK`을 설정합니다. [Mantle 엔드포인트](/docs/ko/amazon-bedrock#use-the-mantle-endpoint) (`CLAUDE_CODE_USE_MANTLE`)는 이 열에 포함됩니다.
* **AWS의 Claude Platform**: AWS Marketplace를 통해 Claude를 구입했지만 Anthropic API를 호출하고 `CLAUDE_CODE_USE_ANTHROPIC_AWS`를 설정합니다.
* **Google Cloud의 Agent Platform**: Google 운영; `CLAUDE_CODE_USE_VERTEX`를 설정합니다.
* **Microsoft Foundry**: Azure의 Anthropic 운영; `CLAUDE_CODE_USE_FOUNDRY`를 설정합니다.

<h3 id="features-available-on-every-provider">
  모든 제공자에서 사용 가능한 기능
</h3>

이러한 기능은 모든 제공자에서 작동합니다:

* [CLI](/docs/ko/quickstart) 및 [Agent SDK](/docs/ko/agent-sdk/overview)
* [VS Code](/docs/ko/vs-code) 및 [JetBrains](/docs/ko/jetbrains) 확장
* [Subagents](/docs/ko/sub-agents), [hooks](/docs/ko/hooks-guide), [commands](/docs/ko/commands), [skills](/docs/ko/skills)
* [CLAUDE.md memory](/docs/ko/memory), [plugins](/docs/ko/plugins), [MCP servers](/docs/ko/mcp)
* [Checkpoints](/docs/ko/checkpointing), [sandboxing](/docs/ko/sandboxing), [Workflows](/docs/ko/workflows)
* [OpenTelemetry metrics](/docs/ko/monitoring-usage) 및 [관리되는 설정 파일](/docs/ko/settings#settings-files)

이 중 세 가지는 제공자별 차이가 있습니다:

* **MCP servers**: [claude.ai의 커넥터](/docs/ko/mcp#use-mcp-servers-from-claude-ai)는 claude.ai 구독이 활성 인증 방법일 때만 로드되며, [도구 검색](/docs/ko/mcp#configure-tool-search)은 Google Cloud의 Agent Platform에서 기본적으로 꺼져 있고 `ANTHROPIC_BASE_URL`이 비자사 호스트를 가리킬 때도 꺼져 있습니다.
* **Subagents**: 기본 제공 [Explore subagent](/docs/ko/sub-agents#built-in-subagents)는 Claude API에서 상속된 모델을 Opus로 제한하며, AWS의 Claude Platform을 포함한 다른 모든 제공자에서는 주 대화의 모델을 직접 상속합니다.
* **[Commands](/docs/ko/commands#all-commands)**: `/design-sync` 및 `/radio`는 Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry, AWS의 Claude Platform에서 사용할 수 없으며, `/voice`는 claude.ai 계정이 필요합니다.

<h3 id="features-that-require-a-claude-subscription">
  Claude 구독이 필요한 기능
</h3>

이러한 기능은 claude.ai 계정으로 로그인해야 하며 Anthropic Console API 키 또는 타사 제공자에서는 도달할 수 없습니다:

* [웹의 Claude Code](/docs/ko/claude-code-on-the-web), 모바일의 Claude Code, [Slack의 Claude Code](/docs/ko/slack)
* [Claude Code Desktop](/docs/ko/desktop)
* [Routines](/docs/ko/routines) (`/schedule`)
* [Ultraplan](/docs/ko/ultraplan) 및 [Ultrareview](/docs/ko/ultrareview)
* [Code Review](/docs/ko/code-review): Team 및 Enterprise 플랜
* [Remote Control](/docs/ko/remote-control)
* [Chrome 확장](/docs/ko/chrome)
* [Computer use](/docs/ko/computer-use): Pro 및 Max 플랜
* [Artifacts](/docs/ko/artifacts): Pro, Max, Team 및 Enterprise 플랜
* [음성 받아쓰기](/docs/ko/voice-dictation)

Desktop은 부분적인 예외입니다: [게이트웨이 라우팅은 앱에서 또는 관리자가 구성할 수 있으며](/docs/ko/llm-gateway-connect#desktop-app), Enterprise 배포는 [관리되는 설정](https://claude.com/docs/third-party/claude-desktop/configuration)을 통해 Desktop을 Google Cloud의 Agent Platform 또는 게이트웨이 제공자로 라우팅할 수 있으며, [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)는 Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry 또는 자체 호스팅 LLM 게이트웨이에서 Code 탭을 실행합니다. 이러한 기능의 플랜별 가용성은 [구독 플랜별 가용성](#availability-by-subscription-plan)을 참조하십시오.

<h3 id="cli-capabilities-that-vary-by-provider">
  제공자별로 다양한 CLI 기능
</h3>

이러한 기능은 로컬 CLI에서 작동하지만 모든 제공자가 노출하지 않는 서버 측 기능에 따라 달라집니다.

<table>
  <thead>
    <tr>
      <th>기능</th>
      <th>Claude 구독</th>
      <th>Anthropic Console</th>
      <th>Amazon Bedrock</th>
      <th>AWS의 Claude Platform</th>
      <th>Google Cloud의 Agent Platform</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>[웹 검색](/docs/ko/tools-reference#websearch-tool-behavior)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✓</td>
      <td>참고 사항 <sup><a href="#fn1">1</a></sup></td>
      <td>✓</td>
    </tr>

    <tr>
      <td>[빠른 모드](/docs/ko/fast-mode)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[자동 모드](/docs/ko/auto-mode-config)</td>
      <td>✓</td>
      <td>✓</td>
      <td>참고 사항 <sup><a href="#fn2">2</a></sup></td>
      <td>✓</td>
      <td>참고 사항 <sup><a href="#fn2">2</a></sup></td>
      <td>참고 사항 <sup><a href="#fn2">2</a></sup></td>
    </tr>

    <tr>
      <td>[Advisor](/docs/ko/advisor)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Channels](/docs/ko/channels)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[`/loop` 예약된 작업](/docs/ko/scheduled-tasks)</td>
      <td>✓</td>
      <td>✓</td>
      <td>참고 사항 <sup><a href="#fn3">3</a></sup></td>
      <td>참고 사항 <sup><a href="#fn3">3</a></sup></td>
      <td>참고 사항 <sup><a href="#fn3">3</a></sup></td>
      <td>참고 사항 <sup><a href="#fn3">3</a></sup></td>
    </tr>

    <tr>
      <td>[GitHub Actions](/docs/ko/github-actions) 및 [GitLab CI/CD](/docs/ko/gitlab-ci-cd)</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✓</td>
      <td>✗</td>
    </tr>
  </tbody>
</table>

<h3 id="admin-and-analytics">
  관리자 및 분석
</h3>

조직 수준의 제어 및 사용 현황 가시성입니다.

<table>
  <thead>
    <tr>
      <th>기능</th>
      <th>Claude 구독</th>
      <th>Anthropic Console</th>
      <th>Amazon Bedrock</th>
      <th>AWS의 Claude Platform</th>
      <th>Google Cloud의 Agent Platform</th>
      <th>Microsoft Foundry</th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <td>[분석 대시보드 및 API](/docs/ko/analytics)</td>
      <td>✓ (대시보드: Team 및 Enterprise; API: Enterprise)</td>
      <td>✓ <sup><a href="#fn5">5</a></sup></td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[서버 관리 설정](/docs/ko/server-managed-settings)</td>
      <td>✓ (Team 및 Enterprise)</td>
      <td>✓ (Team 및 Enterprise)</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
      <td>✗</td>
    </tr>

    <tr>
      <td>[Zero Data Retention](/docs/ko/zero-data-retention)</td>
      <td>✓ (적격 Enterprise 계정)</td>
      <td>✓ (적격 계정)</td>
      <td>참고 사항 <sup><a href="#fn4">4</a></sup></td>
      <td>✓ (적격 계정)</td>
      <td>참고 사항 <sup><a href="#fn4">4</a></sup></td>
      <td>참고 사항 <sup><a href="#fn4">4</a></sup></td>
    </tr>
  </tbody>
</table>

<span id="fn1" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>1</sup> Google Cloud의 Agent Platform에서는 Claude 4 모델 이상에서 웹 검색을 사용할 수 있습니다.<br />
<span id="fn2" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>2</sup> 이러한 제공자에서 자동 모드는 Claude Sonnet 5, Opus 4.7, Opus 4.8만 지원합니다. [자동 모드 구성](/docs/ko/auto-mode-config)을 참조하십시오. v2.1.158부터 v2.1.206까지 이러한 제공자의 자동 모드는 `CLAUDE_CODE_ENABLE_AUTO_MODE=1` 설정도 필요했습니다. v2.1.207은 이 요구 사항을 제거했습니다.<br />
<span id="fn3" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>3</sup> `/loop every 2 hours`와 같은 명시적 간격은 모든 제공자에서 작동합니다. Amazon Bedrock, AWS의 Claude Platform, Google Cloud의 Agent Platform, Microsoft Foundry에서는 `/loop`가 자신의 간격을 선택하거나 기본 유지 관리 프롬프트를 제공할 수 없으므로 간격이 없는 프롬프트는 10분마다 실행되고 인수가 없는 `/loop`는 사용 메시지를 표시합니다. [예약된 작업](/docs/ko/scheduled-tasks)을 참조하십시오.<br />
<span id="fn4" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>4</sup> 클라우드 제공자와의 계약에 따릅니다.<br />
<span id="fn5" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>5</sup> 대시보드 및 API만 해당합니다. [기여도 메트릭](/docs/ko/analytics#enable-contribution-metrics)은 claude.ai Team 또는 Enterprise 조직이 필요합니다.

<Note>
  [LLM 게이트웨이](/docs/ko/llm-gateway)를 통해 인증하는 경우 기능 가용성은 게이트웨이가 전달하는 기본 제공자와 일치합니다. [Advisor](/docs/ko/advisor)와 같은 일부 Anthropic 전용 기능은 게이트웨이가 요청을 Anthropic API로 그대로 전달하는 경우에만 작동합니다.
</Note>

<h3 id="summary-by-provider">
  제공자별 요약
</h3>

각 탭은 해당 제공자에서 사용할 수 없거나 부분적으로 지원되는 기능을 나열하며, 대안이 있는 경우 대안을 제시합니다. 나열되지 않은 모든 기능은 Claude 구독과 동일하게 작동합니다. 단, 위의 [모든 제공자에서 사용 가능한 기능](#features-available-on-every-provider)에서 언급한 제공자별 차이는 제외됩니다. Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry, AWS의 Claude Platform에서는 Anthropic에 대한 오류 보고 및 원격 분석이 기본적으로 꺼져 있습니다. [API 제공자별 기본 동작](/docs/ko/data-usage#default-behaviors-by-api-provider)에서 여전히 Anthropic에 도달하는 트래픽과 옵트아웃 방법을 확인하십시오.

<Tabs>
  <Tab title="Amazon Bedrock">
    **사용 불가능:** 모든 [Claude 구독이 필요한 기능](#features-that-require-a-claude-subscription), 그리고 [웹 검색](/docs/ko/tools-reference#websearch-tool-behavior), [빠른 모드](/docs/ko/fast-mode), [Advisor](/docs/ko/advisor), [Channels](/docs/ko/channels), [분석 대시보드](/docs/ko/analytics), [서버 관리 설정](/docs/ko/server-managed-settings), [`/design-sync` 및 `/radio` 명령](/docs/ko/commands#all-commands).

    **부분 지원:**

    * [Desktop](/docs/ko/desktop): [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)를 통해서만
    * [자동 모드](/docs/ko/auto-mode-config): Sonnet 5, Opus 4.7, Opus 4.8만
    * [`/loop`](/docs/ko/scheduled-tasks): 명시적 간격만
    * [Zero Data Retention](/docs/ko/zero-data-retention): AWS 계약에 따름

    **대안:** 스케줄링의 경우 `/schedule` 대신 명시적 간격으로 [`/loop`](/docs/ko/scheduled-tasks)를 사용하십시오. 클라우드 세션의 경우 [GitHub Actions](/docs/ko/github-actions) 또는 [GitLab CI/CD](/docs/ko/gitlab-ci-cd)를 사용하십시오. 웹 조회의 경우 특정 URL로 [WebFetch 도구](/docs/ko/tools-reference#webfetch-tool-behavior)를 사용하십시오.
  </Tab>

  <Tab title="AWS의 Claude Platform">
    **사용 불가능:** 모든 [Claude 구독이 필요한 기능](#features-that-require-a-claude-subscription), 그리고 [빠른 모드](/docs/ko/fast-mode), [Advisor](/docs/ko/advisor), [Channels](/docs/ko/channels), [분석 대시보드](/docs/ko/analytics), [서버 관리 설정](/docs/ko/server-managed-settings), [`/design-sync` 및 `/radio` 명령](/docs/ko/commands#all-commands).

    **Amazon Bedrock에서 사용 불가능한 경우 사용 가능:** [웹 검색](/docs/ko/tools-reference#websearch-tool-behavior).

    **부분 지원:**

    * [`/loop`](/docs/ko/scheduled-tasks): 명시적 간격만

    **대안:** 스케줄링의 경우 [`/loop`](/docs/ko/scheduled-tasks)를 명시적 간격으로 사용하십시오 (`/schedule` 대신). 클라우드 세션의 경우 [GitHub Actions](/docs/ko/github-actions) 또는 [GitLab CI/CD](/docs/ko/gitlab-ci-cd)를 사용하십시오.
  </Tab>

  <Tab title="Google Cloud의 Agent Platform">
    **사용 불가능:** 모든 [Claude 구독이 필요한 기능](#features-that-require-a-claude-subscription), 그리고 [빠른 모드](/docs/ko/fast-mode), [Advisor](/docs/ko/advisor), [Channels](/docs/ko/channels), [분석 대시보드](/docs/ko/analytics), [서버 관리 설정](/docs/ko/server-managed-settings), [`/design-sync` 및 `/radio` 명령](/docs/ko/commands#all-commands).

    **부분 지원:**

    * [Desktop](/docs/ko/desktop): [관리되는 설정](https://claude.com/docs/third-party/claude-desktop/configuration) 또는 [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)를 통해
    * [웹 검색](/docs/ko/tools-reference#websearch-tool-behavior): Claude 4 모델 이상
    * [자동 모드](/docs/ko/auto-mode-config): Sonnet 5, Opus 4.7, Opus 4.8만
    * [`/loop`](/docs/ko/scheduled-tasks): 명시적 간격만
    * [Zero Data Retention](/docs/ko/zero-data-retention): Google Cloud 계약에 따름

    **대안:** 스케줄링의 경우 `/schedule` 대신 명시적 간격으로 [`/loop`](/docs/ko/scheduled-tasks)를 사용하십시오. 클라우드 세션의 경우 [GitHub Actions](/docs/ko/github-actions) 또는 [GitLab CI/CD](/docs/ko/gitlab-ci-cd)를 사용하십시오.
  </Tab>

  <Tab title="Microsoft Foundry">
    **사용 불가능:** 모든 [Claude 구독이 필요한 기능](#features-that-require-a-claude-subscription), 그리고 [빠른 모드](/docs/ko/fast-mode), [Advisor](/docs/ko/advisor), [Channels](/docs/ko/channels), [GitHub Actions](/docs/ko/github-actions) 및 [GitLab CI/CD](/docs/ko/gitlab-ci-cd), [분석 대시보드](/docs/ko/analytics), [서버 관리 설정](/docs/ko/server-managed-settings), [`/design-sync` 및 `/radio` 명령](/docs/ko/commands#all-commands).

    **부분 지원:**

    * [Desktop](/docs/ko/desktop): [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview)를 통해서만
    * [자동 모드](/docs/ko/auto-mode-config): Sonnet 5, Opus 4.7, Opus 4.8만
    * [`/loop`](/docs/ko/scheduled-tasks): 명시적 간격만
    * [Zero Data Retention](/docs/ko/zero-data-retention): Azure 계약에 따름

    **대안:** 스케줄링의 경우 명시적 간격으로 [`/loop`](/docs/ko/scheduled-tasks)를 사용하십시오.
  </Tab>

  <Tab title="Anthropic Console">
    **사용 불가능:** 모든 [Claude 구독이 필요한 기능](#features-that-require-a-claude-subscription).

    [제공자별로 다양한 CLI 기능](#cli-capabilities-that-vary-by-provider)의 모든 기능을 사용할 수 있으며, API 키가 Team 또는 Enterprise 조직에 속하는 경우 [서버 관리 설정](/docs/ko/server-managed-settings)도 사용할 수 있습니다.
  </Tab>
</Tabs>

<h2 id="availability-by-subscription-plan">
  구독 플랜별 가용성
</h2>

Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry 또는 Anthropic Console API 키를 통해 인증하는 경우 이 섹션은 적용되지 않습니다. claude.ai 계정으로 로그인하면 플랜에 따라 아래 기능의 가용성이 결정됩니다.

| 기능                                                                          | Pro | Max | Team    | Enterprise                        |
| :-------------------------------------------------------------------------- | :-- | :-- | :------ | :-------------------------------- |
| [웹의 Claude Code](/docs/ko/claude-code-on-the-web)                                | ✓   | ✓   | ✓       | ✓ <sup><a href="#fn6">6</a></sup> |
| [Routines](/docs/ko/routines)                                                    | ✓   | ✓   | ✓       | ✓                                 |
| [Remote Control](/docs/ko/remote-control)                                        | ✓   | ✓   | 관리자 활성화 | 관리자 활성화                           |
| [Channels](/docs/ko/channels)                                                    | ✓   | ✓   | 관리자 활성화 | 관리자 활성화                           |
| [Computer use](/docs/ko/computer-use)                                            | ✓   | ✓   | ✗       | ✗                                 |
| Dispatch ([Desktop](/docs/ko/desktop#sessions-from-dispatch))                    | ✓   | ✓   | ✗       | ✗                                 |
| [Code Review](/docs/ko/code-review)                                              | ✗   | ✗   | ✓       | ✓                                 |
| [Artifacts](/docs/ko/artifacts)                                                  | ✓   | ✓   | ✓       | 관리자 활성화                           |
| [분석 대시보드 및 기여도 메트릭](/docs/ko/analytics)                                          | ✗   | ✗   | ✓       | ✓                                 |
| [Enterprise Analytics API](/docs/ko/analytics#access-data-programmatically)      | ✗   | ✗   | ✗       | ✓                                 |
| [서버 관리 설정](/docs/ko/server-managed-settings)                                     | ✗   | ✗   | ✓       | ✓                                 |
| [SSO](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) | ✗   | ✗   | ✓       | ✓                                 |
| SCIM                                                                        | ✗   | ✗   | ✗       | ✓                                 |
| [Compliance API](https://platform.claude.com/docs/en/api/compliance)        | ✗   | ✗   | ✗       | ✓                                 |
| [Zero Data Retention](/docs/ko/zero-data-retention)                              | ✗   | ✗   | ✗       | ✓ <sup><a href="#fn7">7</a></sup> |

<span id="fn6" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>6</sup> Enterprise에서는 프리미엄 시트 또는 Chat + Claude Code 시트가 필요합니다. [웹의 Claude Code](/docs/ko/claude-code-on-the-web)를 참조하십시오.<br />
<span id="fn7" style={{display: 'block', position: 'relative', top: '-120px'}} /><sup>7</sup> 표준 Enterprise 플랜에 포함되지 않습니다. 적격 계정의 경우 Anthropic에서 별도로 활성화해야 합니다. [Zero Data Retention](/docs/ko/zero-data-retention)을 참조하십시오.

가격 책정 및 전체 플랜 비교는 [Team 플랜](https://support.claude.com/en/articles/9266767-what-is-the-team-plan) 및 [Enterprise 플랜](https://support.claude.com/en/articles/9797531-what-is-the-enterprise-plan)을 참조하십시오.

<h2 id="model-availability">
  모델 가용성
</h2>

제공자 및 지역별로 사용 가능한 Claude 모델 및 컨텍스트 윈도우 크기는 [모델 구성](/docs/ko/model-config) 및 [모델 개요](https://platform.claude.com/docs/en/about-claude/models/overview)를 참조하십시오. Vision, PDF 입력, 확장 사고는 Claude Code 기능이 아닌 모델 기능이며 모델을 제공하는 모든 제공자에서 작동합니다. [Prompt caching](/docs/ko/prompt-caching)은 대부분의 제공자에서 동일하게 작동합니다. Amazon Bedrock에서는 모델별로 지원이 다릅니다.

<h2 id="related-resources">
  관련 리소스
</h2>

* [엔터프라이즈 배포 개요](/docs/ko/third-party-integrations): 제공자 간 인증, 청구, 지역 비교
* 제공자 설정 가이드: [Amazon Bedrock](/docs/ko/amazon-bedrock), [AWS의 Claude Platform](/docs/ko/claude-platform-on-aws), [Google Cloud의 Agent Platform](/docs/ko/google-vertex-ai), [Microsoft Foundry](/docs/ko/microsoft-foundry)
* [플랫폼 및 통합](/docs/ko/platforms): CLI, Desktop, IDE 확장, 웹, 모바일, CI/CD를 포함한 Claude Code가 실행되는 위치
