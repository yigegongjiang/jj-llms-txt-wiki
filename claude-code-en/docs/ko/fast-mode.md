> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 빠른 모드로 응답 속도 향상

> Claude Code에서 빠른 모드를 전환하여 더 빠른 Opus 응답을 받습니다.

<Note>
  빠른 모드는 [연구 미리보기](#research-preview)입니다. 피드백에 따라 기능, 가격 책정 및 가용성이 변경될 수 있습니다.
</Note>

빠른 모드는 Claude Opus를 위한 고속 구성으로, 토큰당 더 높은 비용으로 모델을 최대 2.5배 빠르게 만듭니다. 빠른 반복이나 라이브 디버깅과 같은 대화형 작업에서 속도가 필요할 때 `/fast`로 켜고, 비용이 지연 시간보다 중요할 때 끕니다.

빠른 모드는 다른 모델이 아닙니다. 비용 효율성보다 속도를 우선시하는 다른 API 구성을 사용하는 동일한 Claude Opus를 사용합니다. 동일한 품질과 기능을 얻으며, 응답만 더 빠릅니다. 빠른 모드는 Opus 4.8 및 Opus 4.7에서 지원됩니다. Sonnet, Haiku 또는 다른 모델에서는 사용할 수 없습니다.

<Warning>
  Opus 4.7의 빠른 모드는 2026년 6월 25일부터 더 이상 사용되지 않으며 2026년 7월 24일에 제거될 예정입니다. 제거 후 Opus 4.7의 빠른 모드 요청은 오류를 반환하며 표준 Opus 4.7로 폴백되지 않습니다. 속도 향상을 유지하려면 Opus 4.8로 마이그레이션하십시오.
</Warning>

알아야 할 사항:

* Claude Code CLI에서 `/fast`를 사용하여 빠른 모드를 전환합니다. 빠른 모드는 VS Code 확장 프로그램에서 지원되지 않습니다.
* 빠른 모드 가격은 Opus 4.8에서 MTok당 \$10/\$50이고 Opus 4.7에서 \$30/\$150입니다.
* 구독 요금제(Pro/Max/Team/Enterprise)의 모든 Claude Code 사용자 및 Claude Console에서 사용할 수 있습니다.
* 구독 요금제(Pro/Max/Team/Enterprise)의 Claude Code 사용자의 경우, 빠른 모드는 사용 크레딧을 통해서만 사용 가능하며 구독 요금제 사용량 제한에 포함되지 않습니다.

<h2 id="toggle-fast-mode">
  빠른 모드 전환
</h2>

빠른 모드를 다음 중 한 가지 방법으로 전환합니다:

* `/fast`를 입력하고 Tab을 눌러 켜거나 끕니다
* [사용자 설정 파일](/docs/ko/settings)에서 `"fastMode": true`를 설정합니다

기본적으로 빠른 모드는 대화형 세션에서 켜면 세션 간에 유지됩니다. [비대화형 모드](/docs/ko/headless)에서 `-p` 플래그를 사용하면, `/fast`는 [`--settings`](/docs/ko/cli-reference#cli-flags) 값에 빠른 모드가 켜져 있는 상태로 시작된 세션에서만 작동합니다. 예를 들어 `claude -p --settings '{"fastMode": true}'`와 같이 사용하면, 토글이 해당 세션에만 적용되며 기본값으로 저장되지 않으며, 다른 비대화형 세션에서는 빠른 모드를 사용할 수 없다는 명령 보고가 나타납니다. 빠른 모드를 각 세션마다 재설정하도록 구성할 수 있습니다. 자세한 내용은 [세션별 옵트인 필요](#require-per-session-opt-in)를 참조합니다.

최상의 비용 효율성을 위해 대화 중간에 전환하기보다는 세션 시작 시 빠른 모드를 활성화합니다. 자세한 내용은 [비용 트레이드오프 이해](#understand-the-cost-tradeoff)를 참조합니다.

빠른 모드를 활성화하면:

* 다른 모델을 사용 중인 경우 Claude Code가 자동으로 Opus로 전환됩니다
* 확인 메시지가 표시됩니다: "Fast mode ON"
* 빠른 모드가 활성화되어 있는 동안 프롬프트 옆에 작은 `↯` 아이콘이 나타납니다
* 언제든지 `/fast`를 다시 실행하여 빠른 모드가 켜져 있는지 꺼져 있는지 확인합니다

`/fast`를 다시 실행하여 빠른 모드를 비활성화하면 Opus에 유지됩니다. 모델이 이전 모델로 되돌아가지 않습니다. 다른 모델로 전환하려면 `/model`을 사용합니다.

빠른 모드를 지원하지 않는 모델로 전환하면 빠른 모드가 꺼집니다. 지원되는 Opus 모델로 다시 전환하면 저장된 빠른 모드 기본 설정이 켜져 있을 때 다시 켜집니다. 이는 새 세션이 기본적으로 시작되는 것과 동일한 기본 설정입니다. [세션별 옵트인](#require-per-session-opt-in)이 구성된 경우, 다시 전환해도 빠른 모드가 다시 켜지지 않습니다. `/fast`를 실행하여 다시 활성화합니다. 저장된 기본 설정이 꺼져 있는 세션에서는 빠른 모드가 절대 켜지지 않으며, `↯` 아이콘과 `Fast mode ON` 확인이 활성화될 때마다 나타납니다. v2.1.208 이전에는 다시 전환한 후 `/fast`를 다시 실행할 때까지 빠른 모드가 꺼진 상태로 유지되었습니다.

Opus 4.8은 Claude Code v2.1.154 이상에서 빠른 모드 기본값입니다. v2.1.142부터 v2.1.153까지는 빠른 모드가 Opus 4.7로 기본 설정됩니다.

<h2 id="understand-the-cost-tradeoff">
  비용 트레이드오프 이해
</h2>

빠른 모드는 표준 Opus보다 토큰당 가격이 높으며, 모델에 따라 배수가 다릅니다:

| 모델       | 입력 (MTok) | 출력 (MTok) |
| -------- | --------- | --------- |
| Opus 4.8 | \$10      | \$50      |
| Opus 4.7 | \$30      | \$150     |

빠른 모드 가격은 전체 1M 토큰 컨텍스트 윈도우에 걸쳐 고정입니다. 표준 Opus 요금을 비교하려면 [Claude 가격 책정 참고](https://platform.claude.com/docs/ko/about-claude/pricing)를 참조하십시오.

대화 중간에 빠른 모드를 처음 활성화하면 전체 대화 컨텍스트에 대해 전체 빠른 모드 캐시되지 않은 입력 토큰 가격을 지불합니다. 대화가 진행될수록 비용이 더 많이 들므로, 처음부터 빠른 모드를 활성화하는 것이 더 저렴합니다. 비용은 대화당 한 번만 적용되므로, 나중에 빠른 모드를 끄고 다시 켜도 반복되지 않습니다. 메커니즘에 대해서는 [빠른 모드가 프롬프트 캐시와 상호작용하는 방식](/docs/ko/prompt-caching#turning-on-fast-mode)을 참조하십시오.

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

* **Anthropic API 또는 구독만 해당**: 빠른 모드는 Anthropic Console API 및 사용 크레딧을 사용하는 Claude 구독 요금제를 통해 사용할 수 있습니다. Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry 또는 AWS의 Claude Platform에서는 사용할 수 없습니다.
* **사용 크레딧 활성화**: 계정에 사용 크레딧이 활성화되어 있어야 하며, 이를 통해 요금제의 포함된 사용량을 초과하여 청구할 수 있습니다. 개인 계정의 경우 [Console 청구 설정](https://platform.claude.com/settings/billing)에서 활성화합니다. Team 및 Enterprise의 경우 관리자가 조직에 대해 사용 크레딧을 활성화해야 합니다.

<Note>
  빠른 모드 사용량은 요금제에 남은 사용량이 있더라도 사용 크레딧에서 직접 인출됩니다. 이는 빠른 모드 토큰이 요금제의 포함된 사용량에 포함되지 않으며 첫 번째 토큰부터 빠른 모드 요금으로 청구됨을 의미합니다.
</Note>

* **Team 및 Enterprise의 관리자 활성화**: 빠른 모드는 Team 및 Enterprise 조직에 대해 기본적으로 비활성화됩니다. 사용자가 액세스할 수 있으려면 관리자가 명시적으로 [빠른 모드를 활성화](#enable-fast-mode-for-your-organization)해야 합니다.

<Note>
  관리자가 조직에 대해 빠른 모드를 활성화하지 않은 경우 `/fast` 명령은 "Fast mode has been disabled by your organization."을 표시합니다. 조직의 [`availableModels`](/docs/ko/model-config#restrict-model-selection) 허용 목록이 빠른 모드 Opus 모델을 제외하는 경우 `/fast`는 "is not in your organization's allowed models"로 거부됩니다. 예외는 빠른 모드를 지원하는 허용된 Opus 모델에서 이미 실행 중인 세션입니다: `/fast`는 모델을 전환하는 대신 현재 모델에서 빠른 모드를 활성화합니다.
</Note>

<h3 id="enable-fast-mode-for-your-organization">
  조직에 대해 빠른 모드 활성화
</h3>

조직이 사용하는 제품에 따라 빠른 모드를 활성화하는 위치가 달라집니다:

* **Console** (API 고객): 관리자가 [Claude Code 기본 설정](https://platform.claude.com/claude-code/preferences)에서 활성화합니다
* **Claude AI** (Team 및 Enterprise): 관리자가 [관리자 설정 > Claude Code](https://claude.ai/admin-settings/claude-code)에서 활성화합니다

빠른 모드를 완전히 비활성화하는 또 다른 옵션은 `CLAUDE_CODE_DISABLE_FAST_MODE=1`을 설정하는 것입니다. [환경 변수](/docs/ko/env-vars)를 참조합니다.

<h3 id="require-per-session-opt-in">
  세션별 옵트인 필요
</h3>

기본적으로 빠른 모드는 세션 간에 유지됩니다: 사용자가 빠른 모드를 활성화하면 향후 세션에서도 켜져 있습니다. 이를 변경하려면 [설정 파일](/docs/ko/settings#settings-files)에서 `fastModePerSessionOptIn`을 `true`로 설정합니다. 이로 인해 각 세션이 빠른 모드가 꺼진 상태로 시작되며, 사용자가 `/fast`로 명시적으로 활성화해야 합니다. [Team](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_teams#team-&-enterprise) 또는 [Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=fast_mode_enterprise) 요금제의 관리자는 [서버 관리 설정](/docs/ko/server-managed-settings)을 통해 조직 전체에 배포할 수 있습니다.

```json theme={null}
{
  "fastModePerSessionOptIn": true
}
```

이는 사용자가 여러 동시 세션을 실행하는 조직에서 비용을 제어하는 데 유용합니다. 사용자는 속도가 필요할 때 `/fast`로 빠른 모드를 활성화할 수 있지만 새 세션이 시작될 때마다 재설정됩니다. 사용자의 빠른 모드 기본 설정은 여전히 저장되므로 이 설정을 제거하면 기본 지속 동작이 복원됩니다.

<h2 id="handle-rate-limits">
  속도 제한 처리
</h2>

빠른 모드는 표준 Opus와 별도의 속도 제한을 가집니다. Opus 4.8과 Opus 4.7의 빠른 모드는 동일한 속도 제한 풀을 공유합니다: 어느 모델에서든 사용량은 동일한 제한에서 차감됩니다. 빠른 모드 속도 제한에 도달하거나 사용 크레딧이 부족할 때:

1. 빠른 모드가 자동으로 표준 속도로 폴백됩니다
2. `↯` 아이콘이 회색으로 변하여 쿨다운을 나타냅니다
3. 표준 속도 및 가격으로 계속 작업합니다
4. 쿨다운이 만료되면 빠른 모드가 자동으로 다시 활성화됩니다

쿨다운을 기다리지 않고 빠른 모드를 수동으로 비활성화하려면 `/fast`를 다시 실행합니다.

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
