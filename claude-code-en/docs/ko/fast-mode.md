> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 빠른 모드로 응답 속도 향상

> Claude Code에서 빠른 모드를 전환하여 더 빠른 Opus 응답을 받습니다.

<Note>
  빠른 모드는 [연구 미리보기](#research-preview)입니다. 피드백에 따라 기능, 가격 책정 및 가용성이 변경될 수 있습니다.
</Note>

빠른 모드는 Claude Opus를 위한 고속 구성으로, 토큰당 더 높은 비용으로 모델을 최대 2.5배 빠르게 만듭니다. 빠른 반복이나 라이브 디버깅과 같은 대화형 작업에서 속도가 필요할 때 `/fast`로 켜고, 비용이 지연 시간보다 중요할 때 끕니다.

빠른 모드는 다른 모델이 아닙니다. 비용 효율성보다 속도를 우선시하는 다른 API 구성을 사용하는 동일한 Claude Opus를 사용합니다. 동일한 품질과 기능을 얻으며, 응답만 더 빠릅니다. 빠른 모드는 Opus 5.5, Opus 5 및 Opus 4.8에서 지원됩니다. Sonnet, Haiku 또는 다른 모델에서는 사용할 수 없습니다.

Opus 4.7은 빠른 모드를 지원하지 않으므로, 이로 전환하면 빠른 모드가 꺼집니다. Opus 4.7의 빠른 모드는 2026년 6월 25일에 더 이상 사용되지 않으며 2026년 7월 24일에 제거되었습니다.

알아야 할 사항:

* Claude Code CLI에서 `/fast`를 사용하여 빠른 모드를 전환합니다. [VS Code 확장 프로그램](/docs/ko/vs-code)은 선택한 모델이 빠른 모드를 지원할 때 **빠른 모드 전환** 명령을 제공합니다. Claude Code는 해당 전환을 [`fastMode` 설정](#toggle-fast-mode)에 저장합니다.
* 빠른 모드 가격은 Opus 5.5에서 MTok 입력/출력당 \$8/\$40이고 Opus 5 및 Opus 4.8에서 \$10/\$50입니다.
* 구독 요금제(Pro/Max/Team/Enterprise)의 Claude Code 사용자 및 Claude Console에서 사용할 수 있습니다. Team 및 Enterprise 조직은 먼저 소유자가 활성화해야 하며, Console 조직은 먼저 액세스를 프로비저닝해야 하며, 둘 다 [요구 사항](#requirements)에 설명되어 있습니다.
* 구독 요금제(Pro/Max/Team/Enterprise)의 Claude Code 사용자의 경우, 빠른 모드는 사용 크레딧을 통해서만 사용 가능하며 구독 요금제 사용량 제한에 포함되지 않습니다.

<h2 id="toggle-fast-mode">
  빠른 모드 전환
</h2>

CLI에서 다음 중 한 가지 방법으로 빠른 모드를 전환합니다:

* `/fast`를 입력하고 Space를 눌러 켜거나 끕니다. 그 다음 Enter를 눌러 확인합니다
* [사용자 설정 파일](/docs/ko/settings)에서 `"fastMode": true`를 설정합니다

기본적으로 빠른 모드는 대화형 세션에서 켜면 세션 간에 유지됩니다. 빠른 모드를 각 세션마다 재설정하도록 구성할 수 있습니다. 자세한 내용은 [세션별 옵트인 필요](#require-per-session-opt-in)를 참조합니다.

[클라우드 세션](#use-fast-mode-in-cloud-sessions) 외부에서 [비대화형 모드](/docs/ko/headless)에서 `-p` 플래그를 사용하면, `/fast`는 [`--settings`](/docs/ko/cli-reference#cli-flags) 값에 빠른 모드가 켜져 있는 상태로 시작된 세션에서만 작동합니다. 예를 들어 `claude -p --settings '{"fastMode": true}'`와 같이 사용하면, 토글이 해당 세션에만 적용되며 기본값으로 저장되지 않습니다. `-p` 형식은 Claude Code v2.1.205 이상이 필요합니다. 비대화형 모드의 다른 곳에서는 빠른 모드를 사용할 수 없다는 명령 보고가 나타납니다.

Claude가 작업 중일 때 `/fast`를 실행할 수 있으며, Claude Code는 턴이 끝날 때까지 기다리지 않고 빠른 모드를 전환합니다. Claude Code는 실행 중인 턴을 원래 속도로 완료하므로 속도 변경은 다음 턴부터 적용됩니다. 현재 모델이 빠른 모드를 지원하지 않으면 켜면서 모델도 전환되며, Claude Code는 해당 턴의 다음 요청부터 새 모델을 사용합니다.

최상의 비용 효율성을 위해 대화 중간에 전환하기보다는 세션 시작 시 빠른 모드를 활성화합니다. 자세한 내용은 [비용 트레이드오프 이해](#understand-the-cost-tradeoff)를 참조합니다.

빠른 모드를 활성화하면:

* 현재 모델이 빠른 모드를 지원하지 않으면 Claude Code가 Opus로 전환합니다
* 확인 메시지가 표시됩니다: "Fast mode ON"
* 빠른 모드가 활성화되어 있는 동안 프롬프트 옆에 작은 `↯` 아이콘이 나타납니다
* 언제든지 `/fast`를 다시 실행하여 빠른 모드가 켜져 있는지 꺼져 있는지 확인합니다

Opus 5.5는 Claude Code v2.1.280 이상에서 빠른 모드 기본값입니다. v2.1.280 이전에는 v2.1.219부터 Opus 5로 기본 설정되었고, v2.1.154부터 v2.1.218까지는 Opus 4.8로 기본 설정되었으며, v2.1.142부터 v2.1.153까지는 Opus 4.7로 기본 설정되었습니다.

`/fast`를 다시 실행하여 빠른 모드를 비활성화하면 Opus에 유지됩니다. 다른 모델로 전환하려면 `/model`을 사용합니다.

<h3 id="switch-models-while-fast-mode-is-on">
  빠른 모드가 켜져 있는 동안 모델 전환
</h3>

빠른 모드는 양방향 모델 전환을 따릅니다:

* **전환**: 빠른 모드를 지원하지 않는 모델로 전환하면 Claude Code가 빠른 모드를 끕니다. 여기에는 Opus 4.7이 포함됩니다. v2.1.221 이전에는 Opus 4.7로 전환한 후에도 빠른 모드가 켜져 있었고 API가 요청을 거부했습니다.
* **다시 전환**: 지원되는 Opus 모델로 다시 전환하면 저장된 빠른 모드 기본 설정이 켜져 있을 때 빠른 모드가 다시 켜집니다. 이는 새 세션이 기본적으로 시작되는 것과 동일한 기본 설정입니다. 모델 전환은 저장된 기본 설정이 꺼져 있는 세션에서는 빠른 모드를 절대 켜지 않으며, [세션별 옵트인](#require-per-session-opt-in)이 구성된 경우 다시 전환해도 켜지 않습니다. `/fast`를 실행하여 다시 활성화합니다.

모델 전환이 빠른 모드를 켜거나 끌 때마다 Claude Code는 `Fast mode ON` 또는 `Fast mode OFF` 확인을 표시하며, 빠른 모드가 켜져 있는 동안 `↯` 아이콘이 나타납니다. 이는 `/model`로 전환하든, [`/config model=<model>`](/docs/ko/settings)로 전환하든, [원격 제어](/docs/ko/remote-control)를 통해 연결된 기기에서 전환하든 적용됩니다.

Claude Code는 모델 전환, 재연결 또는 실패한 [가용성 확인](#use-fast-mode-behind-proxies-and-llm-gateways) 후 원격 제어를 통해 연결된 기기에 세션의 빠른 모드 상태를 다시 전송합니다.

<h3 id="use-fast-mode-in-cloud-sessions">
  클라우드 세션에서 빠른 모드 사용
</h3>

빠른 모드는 [클라우드 세션](/docs/ko/claude-code-on-the-web)에서 계정에서 사용 가능할 때 작동합니다. 세션이 Anthropic 관리 인프라에서 실행되든 [자체 호스팅 실행기](/docs/ko/self-hosted-environments)에서 실행되든 상관없습니다. 세션의 환경에서 Claude Code v2.1.271 이상이 필요합니다.

세션에서 `/fast on`을 입력하여 빠른 모드를 켭니다. 해당 세션에만 켜져 있으며 기본값으로 저장되지 않습니다. [요구 사항](#requirements)은 클라우드 세션에도 적용됩니다.

<h2 id="understand-the-cost-tradeoff">
  비용 트레이드오프 이해
</h2>

빠른 모드는 표준 Opus보다 토큰당 가격이 높습니다:

| 모델       | 입력 (MTok) | 출력 (MTok) |
| -------- | --------- | --------- |
| Opus 5.5 | \$8       | \$40      |
| Opus 5   | \$10      | \$50      |
| Opus 4.8 | \$10      | \$50      |

빠른 모드 가격은 전체 1M 토큰 컨텍스트 윈도우에 걸쳐 고정입니다. 표준 Opus 요금을 비교하려면 [Claude 가격 책정 참고](https://platform.claude.com/docs/ko/about-claude/pricing)를 참조하십시오.

대화 중간에 빠른 모드를 처음 활성화하면 전체 대화 컨텍스트에 대해 전체 빠른 모드 캐시되지 않은 입력 토큰 가격을 지불합니다. 대화가 진행될수록 비용이 더 많이 들므로, 처음부터 빠른 모드를 활성화하는 것이 더 저렴합니다. 비용은 대화당 한 번만 적용되므로, 나중에 빠른 모드를 끄고 다시 켜도 반복되지 않습니다. 메커니즘에 대해서는 [빠른 모드가 프롬프트 캐시와 상호작용하는 방식](/docs/ko/prompt-caching#turning-on-fast-mode)을 참조하십시오.

<h3 id="see-where-fast-mode-spend-appears">
  빠른 모드 지출이 표시되는 위치 확인
</h3>

로그인 방식에 따라 빠른 모드 지출이 다른 위치에 표시되므로, 먼저 [`/status`](/docs/ko/commands)를 실행하여 확인하십시오. `Claude Max 계정`과 같은 `로그인 방법` 행이 표시되면 Claude 구독으로 로그인한 것입니다. 대신 `API 키` 행이 표시되면 요청이 Claude Console 조직에 청구됩니다.

* **Pro 및 Max**: 사용 크레딧에서 빠른 모드 비용을 지불합니다. claude.ai의 [**설정 > 사용**](https://claude.ai/settings/usage)으로 이동하면, **사용 크레딧** 섹션에 이번 달 사용 크레딧으로 얼마를 지출했는지 표시됩니다. 이 수치에는 빠른 모드가 포함되지만 별도로 분류되지 않습니다.
* **Team 및 Enterprise**: 조직이 사용 크레딧에서 빠른 모드 사용 비용을 지불합니다. 자신의 사용 크레딧 지출을 확인하려면 [`/usage`](/docs/ko/costs#check-your-usage-credits-spend)를 실행하십시오. 조직이 해당 지출을 확인하는 위치에 대해서는 [Claude for Teams and Enterprise](/docs/ko/costs#claude-for-teams-and-enterprise)를 참조하십시오.
* **Claude Console**: 조직이 나머지 API 사용과 함께 빠른 모드 비용을 지불합니다. Console [사용](https://platform.claude.com/usage) 및 [비용](https://platform.claude.com/cost) 페이지에서 **그룹화 기준** 메뉴의 \*\*속도 (연구 미리보기)\*\*를 선택하여 빠른 모드를 표준 속도 사용과 분리합니다. 선택한 날짜 범위에 빠른 모드 사용이 포함된 경우에만 해당 옵션이 표시됩니다.

<h2 id="decide-when-to-use-fast-mode">
  빠른 모드 사용 시기 결정
</h2>

빠른 모드는 응답 지연 시간이 비용보다 중요한 대화형 작업에 가장 적합합니다:

* 코드 변경에 대한 빠른 반복
* 라이브 디버깅 세션
* 긴급 마감이 있는 시간에 민감한 작업

표준 모드는 다음에 더 적합합니다:

* 속도가 덜 중요한 장기 자동 작업
* 배치 처리 또는 CI/CD 파이프라인
* 비용에 민감한 워크로드

<h3 id="fast-mode-vs-effort-level">
  빠른 모드 대 노력 수준
</h3>

빠른 모드와 노력 수준 모두 응답 속도에 영향을 미치지만 방식이 다릅니다:

| 설정           | 효과                                        |
| ------------ | ----------------------------------------- |
| **빠른 모드**    | 동일한 모델 품질, 낮은 지연 시간, 높은 비용                |
| **낮은 노력 수준** | 더 적은 생각 시간, 더 빠른 응답, 복잡한 작업에서 잠재적으로 낮은 품질 |

둘 다 결합할 수 있습니다: 간단한 작업에서 최대 속도를 위해 낮은 [노력 수준](/docs/ko/model-config#adjust-effort-level)과 함께 빠른 모드를 사용합니다.

<h2 id="requirements">
  요구사항
</h2>

빠른 모드는 다음 모두를 필요로 합니다:

* **Anthropic API 또는 구독만 해당**: 빠른 모드는 Anthropic Console API 및 사용 크레딧을 사용하는 Claude 구독 요금제를 통해 사용할 수 있습니다. Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry 또는 AWS의 Claude Platform에서는 사용할 수 없습니다. Console 조직은 또한 [빠른 모드 액세스가 프로비저닝](#enable-fast-mode-for-your-organization)되어 있어야 합니다.
* **구독 요금제에 대해 사용 크레딧 활성화**: Pro, Max, Team 또는 Enterprise 요금제에서 계정에 [사용 크레딧](/docs/ko/costs#add-usage-credits-to-your-subscription)이 활성화되어 있어야 하며, 이를 통해 요금제의 포함된 사용량을 초과하여 청구할 수 있습니다. 활성화될 때까지 `/fast`는 "Fast mode requires usage credits"를 표시합니다. 활성화하는 방법은 요금제에 따라 다릅니다:
  * Pro 및 Max에서는 claude.ai의 [**설정 > 사용**](https://claude.ai/settings/usage)의 **사용 크레딧** 섹션에서 활성화하거나 `/usage-credits`를 실행하여 해당 페이지를 엽니다.
  * Team 및 Enterprise에서는 청구 액세스 권한이 있는 구성원이 [**관리자 설정 > 사용**](https://claude.ai/admin-settings/usage)에서 조직에 대해 활성화하고, 액세스 권한이 없는 구성원은 `/usage-credits`를 실행하여 조직의 관리자에게 요청을 보냅니다.

<Note>
  빠른 모드 사용량은 요금제에 남은 사용량이 있더라도 사용 크레딧에서 직접 인출됩니다.
</Note>

* **유료 Console 조직**: Claude Console 계정은 사용 크레딧을 사용하지 않으며, 조직은 나머지 API 사용량과 함께 빠른 모드에 대해 토큰당 비용을 지불합니다. Console의 무료 Evaluation 요금제에서 `/fast`는 "Fast mode unavailable during evaluation. Please purchase credits."을 표시합니다. 이를 해결하려면 [Console 청구 설정](https://platform.claude.com/settings/billing)에서 크레딧을 구매합니다.
* **Team 및 Enterprise의 관리자 활성화**: 빠른 모드는 Team 및 Enterprise 조직에 대해 기본적으로 비활성화됩니다. 사용자가 액세스할 수 있으려면 관리자가 명시적으로 [빠른 모드를 활성화](#enable-fast-mode-for-your-organization)해야 합니다.

<Note>
  네 가지 조직 설정이 `/fast`로 빠른 모드를 켜는 것을 차단할 수 있습니다:

  * **빠른 모드가 활성화되지 않음**: 조직에 대해 빠른 모드가 활성화되지 않은 경우 `/fast`로 빠른 모드를 켜면 "Fast mode has been disabled by your organization."이 표시됩니다.
  * **관리 설정에 의해 빠른 모드가 꺼짐**: 조직이 [`fastMode: false`](/docs/ko/settings-reference#fastmode)를 설정하는 [관리 설정](/docs/ko/managed-settings)을 배포하는 경우 `/fast`로 빠른 모드를 켜면 동일한 "Fast mode has been disabled by your organization" 메시지가 표시됩니다.
  * **세션별 옵트인 필요**: [`fastModePerSessionOptIn: true`](#require-per-session-opt-in)를 설정하는 관리 설정은 대화형 터미널 세션을 제외한 모든 곳에서 `/fast on`을 동일한 메시지로 거부합니다.
  * **빠른 모드 모델이 허용되지 않음**: 조직의 [`availableModels`](/docs/ko/model-config#restrict-model-selection) 허용 목록이 빠른 모드 Opus 모델을 제외하는 경우 "is not in your organization's allowed models"로 거부됩니다. 빠른 모드를 지원하는 허용된 Opus 모델에서 이미 실행 중인 세션에서는 `/fast`가 모델을 전환하는 대신 현재 모델에서 빠른 모드를 활성화합니다.
</Note>

<h3 id="enable-fast-mode-for-your-organization">
  조직에 대해 빠른 모드 활성화
</h3>

조직이 사용하는 제품에 따라 빠른 모드를 활성화하는 위치가 달라집니다:

* **Console** (API 고객): 관리자가 [Claude Code 기본 설정](https://platform.claude.com/claude-code/preferences)에서 활성화합니다. 빠른 모드는 [연구 미리보기](#research-preview)에 있으므로 조직은 빠른 모드 요청이 성공하기 전에 빠른 모드 액세스가 프로비저닝되어 있어야 합니다. 액세스를 얻으려면 계정 관리자에게 문의하거나 [Claude API의 빠른 모드](https://platform.claude.com/docs/en/build-with-claude/fast-mode)에 설명된 대로 대기 목록에 참여합니다.

  프로비저닝된 액세스가 없으면 API는 각 빠른 모드 요청을 429로 거부하고, Claude Code는 각 거부를 [빠른 모드 속도 제한](#handle-rate-limits)으로 처리합니다. 속도 제한의 쿨다운과 달리 거부는 액세스가 프로비저닝될 때까지 계속됩니다.
* **Claude AI** (Team 및 Enterprise): 관리자가 [관리자 설정 > Claude Code](https://claude.ai/admin-settings/claude-code)에서 활성화합니다

빠른 모드를 완전히 비활성화하는 또 다른 옵션은 `CLAUDE_CODE_DISABLE_FAST_MODE=1`을 설정하는 것입니다. [환경 변수](/docs/ko/env-vars)를 참조합니다.

<h3 id="use-fast-mode-behind-proxies-and-llm-gateways">
  프록시 및 LLM 게이트웨이 뒤에서 빠른 모드 사용
</h3>

빠른 모드를 제공하기 전에 Claude Code는 `api.anthropic.com`에 직접 요청하여 조직의 빠른 모드 가용성을 확인합니다. 확인은 [`ANTHROPIC_BASE_URL`](/docs/ko/llm-gateway-connect#set-the-base-url-and-credential)을 따르지 않으므로, Claude 트래픽을 [LLM 게이트웨이](/docs/ko/llm-gateway)를 통해 라우팅하고 `api.anthropic.com`에 대한 직접 송신을 차단하는 네트워크에서는 추론 요청이 작동하더라도 확인이 실패합니다. 확인은 구성된 [HTTP 프록시](/docs/ko/network-config#proxy-configuration)를 사용하므로 네트워크 차단은 프록시를 통해서도 `api.anthropic.com`에 도달할 수 없는 경우에만 확인을 실패합니다.

확인이 실패하면 `/fast`는 "Fast mode unavailable due to network connectivity issues"를 보고하고, 조직에서 빠른 모드가 활성화되어 있더라도 요청은 표준 속도로 실행됩니다. 과거에 성공한 확인은 캐시된 결과에서 계속 작동하므로 차단된 확인은 주로 새 설치에 영향을 미칩니다.

확인이 `api.anthropic.com`에 도달하지만 Anthropic이 거부하는 자격 증명을 제시할 때 열린 네트워크에서도 동일한 연결 메시지가 나타납니다. 해결된 키가 게이트웨이 발급 자격 증명인 세션은 [`ANTHROPIC_API_KEY`](/docs/ko/llm-gateway-connect#set-the-base-url-and-credential)에 보관되거나 [`apiKeyHelper`](/docs/ko/settings-reference#apikeyhelper)에 의해 생성되며, 해당 키로 확인을 보내고 거부된 요청은 연결 실패로 보고됩니다.

빠른 모드를 복원하려면 네트워크 차단이 원인인 경우 `api.anthropic.com`에 대한 직접 송신을 허용 목록에 추가하거나 확인이 실패하는 방식과 일치하는 변수를 설정합니다:

* `CLAUDE_CODE_SKIP_FAST_MODE_NETWORK_ERRORS=1`은 실패한 확인을 사용 가능한 것으로 처리하고 여전히 "disabled by your organization" 응답을 준수합니다. 네트워크가 연결을 거부하거나 Anthropic이 게이트웨이 자격 증명을 거부할 때 사용합니다. 허용 목록은 아무것도 차단되지 않으므로 자격 증명 경우에는 도움이 되지 않습니다.
* `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK=1`은 확인을 완전히 건너뜁니다. 네트워크가 요청을 거부하는 대신 가로챌 때 사용합니다.

두 게이트웨이 구성은 조직에서 빠른 모드가 활성화되어 있더라도 연결 메시지 대신 "Fast mode has been disabled by your organization"을 보고합니다:

* [`ANTHROPIC_AUTH_TOKEN`](/docs/ko/llm-gateway-connect#set-the-base-url-and-credential)만으로 인증하는 세션은 확인을 건너뜁니다: claude.ai 로그인이나 Anthropic API 키가 없고 캐시된 성공한 확인이 없으면 Claude Code는 요청을 보내지 않고 빠른 모드를 조직에서 비활성화한 것으로 처리합니다.
* 확인을 가로채고 자신의 페이지로 응답하는 프록시(예: TLS 검사 프록시가 HTTP 200 차단 페이지를 반환)는 조직에서 빠른 모드가 비활성화되었다고 말하는 응답으로 읽혀집니다.

두 경우 모두 `CLAUDE_CODE_SKIP_FAST_MODE_ORG_CHECK=1`을 설정하여 빠른 모드를 복원합니다. `CLAUDE_CODE_SKIP_FAST_MODE_NETWORK_ERRORS`는 실패한 확인만 우회하고 둘 다 비활성화 응답을 생성하므로 두 경우 모두에 적용되지 않습니다. 허용 목록은 요청을 보내지 않는 베어러 토큰 경우에는 도움이 되지 않습니다.

변수는 클라이언트 측 확인에만 영향을 미칩니다. 조직에서 빠른 모드가 비활성화되면 API는 설정 여부와 관계없이 빠른 모드 요청을 거부합니다. API의 거부는 건너뛰기 변수가 설정되어 있어도 유지됩니다. Claude Code는 거부된 요청을 표준 속도로 다시 시도하고, 빠른 모드를 끄고, `/fast`는 조직에서 빠른 모드를 비활성화했다고 보고합니다.

`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`을 설정하면 가용성 확인도 억제됩니다. 이전에 캐시된 성공한 확인이 없으면 `/fast`는 "Fast mode is currently unavailable"을 보고합니다. 두 건너뛰기 변수 모두 해당 구성에서 빠른 모드를 복원합니다.

<h3 id="require-per-session-opt-in">
  세션별 옵트인 필요
</h3>

기본적으로 사용자가 대화형 세션에서 활성화하는 빠른 모드는 세션 간에 유지됩니다. 이를 변경하려면 [설정 파일](/docs/ko/settings#where-settings-live)에서 `fastModePerSessionOptIn`을 `true`로 설정합니다. 이로 인해 각 세션이 빠른 모드가 꺼진 상태로 시작되며 사용자가 `/fast`로 명시적으로 활성화해야 합니다. [Team](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_teams#team-&-enterprise) 또는 [Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_enterprise) 요금제의 소유자는 [서버 관리 설정](/docs/ko/server-managed-settings)을 통해 조직 전체에 배포할 수 있습니다.

```json theme={null}
{
  "fastModePerSessionOptIn": true
}
```

이는 사용자가 여러 동시 세션을 실행하는 조직에서 비용을 제어하는 데 유용합니다. 사용자의 빠른 모드 기본 설정은 여전히 저장되므로 이 설정을 제거하면 기본 지속 동작이 복원됩니다.

관리 설정이 키를 설정할 때 `/fast on`은 대화형 터미널 세션에서만 작동합니다. [비대화형 모드](/docs/ko/headless), [VS Code 확장](/docs/ko/vs-code) 및 [클라우드 세션](#use-fast-mode-in-cloud-sessions)을 포함한 다른 모든 곳에서는 조직에서 빠른 모드를 비활성화했다는 메시지로 거부됩니다.

<h2 id="handle-rate-limits">
  속도 제한 처리
</h2>

빠른 모드는 표준 Opus와 별도의 속도 제한을 가집니다. 지원되는 모든 Opus 모델은 하나의 빠른 모드 속도 제한 풀을 공유합니다: 어느 모델에서든 사용량은 동일한 제한에서 차감됩니다. 빠른 모드 속도 제한에 도달하면:

1. 빠른 모드가 자동으로 표준 속도로 폴백됩니다
2. `↯` 아이콘이 회색으로 변하여 쿨다운을 나타냅니다
3. 표준 속도 및 가격으로 계속 작업합니다
4. 쿨다운이 만료되면 빠른 모드가 자동으로 다시 활성화됩니다

쿨다운을 기다리지 않고 빠른 모드를 수동으로 비활성화하려면 `/fast`를 다시 실행합니다.

세션 중에 사용 크레딧이 부족하면 Claude Code는 거부된 각 빠른 모드 요청을 표준 속도 및 가격으로 재시도하므로 계속 작업할 수 있으며 쿨다운이 없습니다. 거부 사항을 보는 방식은 세션 유형에 따라 다릅니다:

* 대화형 세션에서 Claude Code는 "Fast mode disabled · usage credits exhausted" 알림을 표시하고 세션의 나머지 부분에서 빠른 모드를 끕니다. 저장된 빠른 모드 기본 설정은 변경되지 않습니다: `/fast`를 실행하여 빠른 모드를 다시 켭니다.
* [비대화형 모드](/docs/ko/headless)에서 `--output-format stream-json`을 사용하고 Agent SDK를 통해 Claude Code는 메시지 스트림에서 `system` 메시지로 동일한 텍스트를 subtype `notification`으로 내보내며, 사용 크레딧이 부족한 동안 턴당 한 번씩 표시됩니다. 빠른 모드는 켜진 상태로 유지됩니다. Claude Code v2.1.221 이상이 필요합니다.

<h2 id="research-preview">
  연구 미리보기
</h2>

빠른 모드는 연구 미리보기 기능입니다. 이는 다음을 의미합니다:

* 기능은 피드백에 따라 변경될 수 있습니다
* 가용성 및 가격 책정은 변경될 수 있습니다
* 기본 API 구성이 진화할 수 있습니다

일반적인 Anthropic 지원 채널을 통해 문제 또는 피드백을 보고합니다.

<h2 id="see-also">
  참고 항목
</h2>

* [모델 구성](/docs/ko/model-config): 모델 전환 및 노력 수준 조정
* [비용 효과적으로 관리](/docs/ko/costs): 토큰 사용량 추적 및 비용 감소
* [상태 줄 구성](/docs/ko/statusline): 모델 및 컨텍스트 정보 표시
