> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 플러그인 보안 및 신뢰

> 플러그인을 설치하기 전에 신뢰할 수 있는지 결정하세요. 플러그인이 머신에서 할 수 있는 작업부터 플러그인을 검토하고 제거하는 방법까지 알아봅니다.

설치하는 Claude Code 플러그인은 사용자 권한으로 머신에서 임의의 코드를 실행할 수 있습니다.

마켓플레이스에서 플러그인을 설치합니다. 마켓플레이스는 Claude Code가 플러그인을 가져오는 카탈로그입니다. 일부 마켓플레이스 이름은 [Anthropic의 자체 마켓플레이스용으로 예약되어 있으며](#marketplace-tiers), 다른 모든 마켓플레이스는 타사입니다. 마켓플레이스의 이름은 카탈로그를 게시하는 사람을 나타내지만, 카탈로그의 각 플러그인이 무엇을 하는지는 나타내지 않으므로, [설치하기 전에 플러그인을 검토하세요](#review-a-plugin-before-you-install). 어느 마켓플레이스에서 가져오든 상관없습니다.

플러그인 설치 여부를 결정하거나 팀이 도구를 사용하기 전에 검토하는 경우 이 페이지를 읽으세요.

<Note>
  다음 경우는 다른 페이지에서 다룹니다:

  * **Claude Code의 자체 보안 모델**: [보안](/docs/ko/security)을 참조하세요.
  * **조직을 위한 플러그인 제한 또는 필수 설정**: [조직을 위한 플러그인 관리](/docs/ko/plugins/org)를 참조하세요.
  * **`security-guidance` 또는 `claude-security` 플러그인**: 이 페이지는 이들에 관한 것이 아닙니다. [`security-guidance`](/docs/ko/security-guidance) 및 [`claude-security`](/docs/ko/claude-security)를 참조하세요.
</Note>

[플러그인이 할 수 있는 작업](#understand-what-a-plugin-can-do)과 [어느 마켓플레이스가 Anthropic의 것인지](#marketplace-tiers)부터 시작한 다음, [설치하기 전에 플러그인을 검토하세요](#review-a-plugin-before-you-install).

<h2 id="understand-what-a-plugin-can-do">
  플러그인이 할 수 있는 것 이해하기
</h2>

플러그인은 사용자 권한으로 머신에서 실행되는 코드와 Claude의 컨텍스트에 지침으로 입력되는 콘텐츠를 포함할 수 있으므로 [플러그인을 설치하기 전에 검토하십시오](#review-a-plugin-before-you-install). 설치된 플러그인이 할 수 있는 것은 다음과 같습니다:

* **Hooks**: 플러그인의 [hooks](/docs/ko/hooks)는 도구 호출 전후와 같이 Claude Code의 수명 주기의 특정 지점에서 셸 명령으로 실행됩니다.
* **MCP 및 LSP 서버**: Claude Code는 활성화된 플러그인이 선언하는 [MCP 서버](/docs/ko/mcp)에 연결되고 Claude에 해당 도구를 제공합니다. stdio MCP 서버는 Claude Code가 머신에서 시작하는 프로세스로 실행됩니다. Claude Code는 플러그인이 선언하는 언어 서버도 시작합니다.
* **`bin/` 디렉토리**: Claude Code는 활성화된 각 플러그인의 `bin/` 디렉토리를 Bash 도구의 셸의 `PATH`에 추가하므로 Claude의 Bash 명령은 여기의 모든 실행 파일을 실행할 수 있습니다.
* **Skills, commands, and agents**: 이들은 Claude의 컨텍스트에 지침으로 입력되므로 Claude가 이미 가지고 있는 도구로 수행하는 작업에 영향을 미칩니다.
* **업데이트**: 플러그인을 설치한 마켓플레이스에서 자동 업데이트가 켜져 있으면 Claude Code는 백그라운드에서 해당 플러그인을 업데이트하므로 검토한 파일이 디스크에서 변경될 수 있습니다. [자동 업데이트가 실행되는 시기](/docs/ko/plugins/loading#when-auto-update-runs)에 타이밍이 있습니다. 마켓플레이스별로 자동 업데이트를 켜거나 끄려면 [플러그인 업데이트 유지](/docs/ko/plugins/install#keep-plugins-updated)를 참조하십시오.

Claude Code의 [권한 규칙](/docs/ko/permissions) 및 [sandbox](/docs/ko/sandboxing)는 Claude가 수행하는 도구 호출을 다루며, 플러그인이 자체적으로 실행하는 코드는 다루지 않습니다:

* **Hooks 및 서버 프로세스**: 명령 hooks는 전체 사용자 권한으로 셸 명령을 실행합니다. Claude Code는 hooks 및 MCP 서버를 sandbox 외부에서 실행합니다.
* **Claude의 도구 호출**: 플러그인의 MCP 도구 중 하나에 대한 호출 및 플러그인의 `bin/`에서 실행 파일을 실행하는 Bash 명령은 도구 호출이므로 권한 규칙이 적용됩니다.

플러그인을 설치하면 해당 매니페스트 또는 마켓플레이스 항목이 [`defaultEnabled: false`](/docs/ko/plugins/install#choose-an-install-scope)를 설정하고 사용자가 직접 활성화하지 않은 경우를 제외하고는 플러그인이 활성화됩니다.

더 이상 신뢰하지 않는 플러그인을 제거하려면 [더 이상 신뢰하지 않는 플러그인 제거](#remove-a-plugin-you-no-longer-trust)를 참조하십시오.

<h2 id="marketplace-tiers">
  이름으로 Anthropic의 마켓플레이스 식별하기
</h2>

마켓플레이스의 이름은 공식, 커뮤니티 또는 타사의 세 가지 계층 중 하나에 배치합니다. Claude Code는 `github.com/anthropics/` 저장소에서 소싱된 마켓플레이스에 대해서만 공식 및 커뮤니티 이름을 허용하므로, 타사 마켓플레이스는 자신을 Anthropic 마켓플레이스로 제시할 수 없습니다. 동료 또는 조직이 게시하는 마켓플레이스는 타사입니다.

표는 각 계층에 어떤 이름이 속하는지 나열합니다:

| 계층   | 마켓플레이스                                                                    |
| :--- | :------------------------------------------------------------------------ |
| 공식   | [공식 마켓플레이스 이름](#official-marketplace-names)(예: `claude-plugins-official`) |
| 커뮤니티 | `claude-community`, `claude-plugins-community`, 및 `healthcare`            |
| 타사   | 다른 모든 마켓플레이스                                                              |

`claude-community` 카탈로그가 플러그인을 커밋 SHA에 고정하는 경우(거의 모든 항목에 대해 수행), Claude Code는 다른 커밋 설치를 거부합니다.

<h3 id="official-marketplace-names">
  공식 마켓플레이스 이름
</h3>

이 마켓플레이스 이름은 공식 계층을 구성합니다:

* `claude-plugins-official`
* `claude-code-marketplace`
* `claude-code-plugins`
* `anthropic-marketplace`
* `anthropic-plugins`
* `agent-skills`
* `anthropic-agent-skills`
* `life-sciences`
* `knowledge-work-plugins`
* `claude-for-legal`
* `claude-for-financial-services`
* `financial-services-plugins`
* `first-party-plugins`
* `claude-tag-plugins`

공식, 커뮤니티 및 데모 마켓플레이스가 어떻게 다르고 각각 어디에서 나열된 내용을 찾아볼 수 있는지는 [Anthropic의 마켓플레이스](/docs/ko/plugins/anthropic-marketplaces)를 참조하세요.

<h2 id="review-a-plugin-before-you-install">
  설치하기 전에 플러그인 검토하기
</h2>

플러그인을 설치하기 전에 추가되는 내용과 출처를 확인하세요.

<Steps>
  <Step title="마켓플레이스의 소스 확인">
    셸에서 `claude plugin marketplace list`를 실행하여 GitHub 저장소 또는 디렉토리와 같이 각 마켓플레이스가 추가된 소스를 인쇄합니다.
  </Step>

  <Step title="세부 정보 창 읽기">
    Claude Code 세션에서 `/plugin`을 실행하고 플러그인을 선택합니다. 세부 정보 창은 플러그인의 명령, agents, skills, hooks 및 MCP와 LSP 서버를 나열하는 **설치될 항목** 섹션을 표시합니다. Anthropic이 게시된 구성 요소 데이터가 없는 플러그인의 경우, 섹션은 마켓플레이스 항목이 선언하는 내용을 표시하거나, 마켓플레이스 내에 저장된 플러그인의 경우 `설치 시 구성 요소가 발견됩니다` 또는 다른 곳에서 가져온 플러그인의 경우 `원격 플러그인에 대해 구성 요소 요약을 사용할 수 없습니다`라는 메모를 표시합니다.
  </Step>

  <Step title="플러그인의 소스 읽기">
    세부 정보 창에서 설치 옵션 아래의 **홈페이지 열기** 또는 **GitHub에서 보기**를 선택합니다. 창이 둘 다 제공하지 않으면, 첫 번째 단계에서 찾은 마켓플레이스 저장소를 엽니다. 거기서 플러그인의 디렉토리를 찾습니다. **설치될 항목** 섹션은 hook이 존재함을 보여주지만 실행되는 내용은 보여주지 않으므로, 플러그인의 디렉토리에서 이 파일들을 읽으세요:

    * **`hooks/hooks.json`**: 각 hook이 실행하는 명령
    * **`.mcp.json`**: 각 서버의 명령 또는 URL
    * **`bin/`**: 디렉토리의 모든 파일
  </Step>

  <Step title="플러그인이 포함하는 내용 나열">
    플러그인의 디렉토리를 보유한 저장소를 복제한 다음, 셸에서 `claude --plugin-dir <plugin directory> plugin details <plugin name>`을 실행하여 Claude Code가 찾은 내용을 확인합니다. 명령은 세션을 시작하지 않고 플러그인의 파일을 읽고 플러그인의 skills 및 명령, agents, 각 hook의 이벤트가 있는 hooks 및 MCP와 LSP 서버를 나열하는 `구성 요소 인벤토리`를 인쇄합니다.
  </Step>
</Steps>

플러그인을 설치한 후, 셸에서 `claude plugin details <plugin name>`을 실행하여 `~/.claude/plugins/cache/<marketplace>/<plugin>/<version>/` 아래의 설치된 복사본에 대해 동일한 `구성 요소 인벤토리`를 인쇄합니다.

<h3 id="remove-a-plugin-you-no-longer-trust">
  더 이상 신뢰하지 않는 플러그인 제거
</h3>

셸에서 설치한 `--scope`와 함께 [`claude plugin uninstall <plugin>`](/docs/ko/plugins/cli-reference#plugin-uninstall)을 실행합니다. 그런 다음 제거된 내용과 남겨진 내용을 확인합니다:

* **지속적인 데이터**: 이것이 플러그인이 설치된 마지막 범위인 경우, 제거하면 플러그인의 지속적인 데이터 디렉토리도 삭제됩니다. `--keep-data`를 전달하지 않는 한입니다.
* **캐시된 파일**: 플러그인의 파일은 `~/.claude/plugins/cache/` 아래 디스크에 남아 있으며, [백그라운드 스윕이 제거하기](/docs/ko/plugins/loading#cleanup-of-previous-versions) 전에 14일 동안 남아 있습니다. 마지막 플러그인을 제거한 후, 고아 디렉토리는 다른 플러그인을 설치할 때까지 남아 있습니다. 지금 파일을 삭제하려면, `~/.claude/plugins/cache/<marketplace>/<plugin>/` 아래의 플러그인 디렉토리를 직접 제거하세요.
* **마켓플레이스**: 마켓플레이스의 소유자도 신뢰하지 않으면, [마켓플레이스도 제거하세요](/docs/ko/plugins/install#manage-marketplaces). 이렇게 하면 해당 마켓플레이스에서 설치한 모든 플러그인이 제거됩니다.

<h2 id="recognize-when-claude-code-refuses-or-warns">
  Claude Code가 거부하거나 경고하는 경우 인식하기
</h2>

`/plugin`의 **Discover** 또는 **Marketplaces** 탭에서 열 수 있는 세부 정보 창에는 각 플러그인에 대해 동일한 신뢰 경고가 표시됩니다. Claude Code는 [신뢰할 수 없는 마켓플레이스 소스 및 무결성 검사 실패](#untrusted-marketplace-sources-and-failed-integrity-checks)에 해당하는 경우와 같은 경우에 경고 대신 거부합니다.

<h3 id="trust-warning-before-you-install">
  설치 전 신뢰 경고
</h3>

경고는 플러그인이 어느 마켓플레이스에서 오든 동일하게 표시됩니다:

```text theme={null}
Make sure you trust a plugin before installing, updating, or using it. Anthropic does not control what MCP servers, files, or other software are included in plugins and cannot verify that they will work as intended or that they won't change. See each plugin's homepage for more information.
```

조직에서 [관리 설정](/docs/ko/plugins/org)에서 `pluginTrustMessage`를 설정한 경우, Claude Code는 해당 텍스트를 경고에 추가합니다.

<h3 id="untrusted-marketplace-sources-and-failed-integrity-checks">
  신뢰할 수 없는 마켓플레이스 소스 및 무결성 검사 실패
</h3>

Claude Code는 다음의 경우에 마켓플레이스를 로드하거나 플러그인을 설치하기를 거부하며, 각각 고유한 오류 메시지가 있습니다:

* **신뢰할 수 없는 마켓플레이스 소스**: 마켓플레이스가 공식 또는 커뮤니티 이름을 사용하지만 해당 소스가 `github.com/anthropics/` 외부에 있을 때, Claude Code는 마켓플레이스 로드를 중지하고 해당 마켓플레이스에서 설치한 플러그인을 로드하기를 거부합니다. 오류는 [Marketplace is registered from an untrusted source](/docs/ko/errors#marketplace-is-registered-from-an-untrusted-source)입니다.
* **아카이브 무결성**: 마켓플레이스 항목이 [`archive` 소스](/docs/ko/plugins/marketplace-reference#archive-plugin-source)를 `sha256` 다이제스트에 고정하고 다운로드된 파일의 다이제스트가 일치하지 않을 때, Claude Code는 설치를 거부합니다. 오류는 [Plugin archive integrity check failed](/docs/ko/errors#plugin-archive-integrity-check-failed)입니다.

`sha256` 핀은 체크아웃할 git 커밋을 선택하는 커뮤니티 카탈로그의 커밋 SHA 핀과 별개입니다.

<h2 id="enforce-plugin-controls-for-your-organization">
  조직을 위한 플러그인 제어 적용
</h2>

[관리 설정](/docs/ko/plugins/org)을 사용하면, 관리자는 다음 플러그인 제어를 적용할 수 있습니다:

* 마켓플레이스 소스 허용 목록 또는 차단 목록
* 플러그인 강제 활성화
* `--plugin-dir` 및 `--plugin-url` 플래그와 `CLAUDE_CODE_PLUGIN_DIRS` 변수 끄기
* hooks를 관리 설정 및 강제 활성화된 플러그인의 hooks로 제한
* 구성원의 claude.ai 계정의 플러그인이 Claude Code에서 로드되지 않도록 중지([`syncClaudeAiPlugins`](/docs/ko/plugins/org#control-matrix) 포함)

[제어 매트릭스](/docs/ko/plugins/org#control-matrix)는 각 키가 무엇을 하고 하지 않는지 말합니다.

<h2 id="find-plugins-in-telemetry">
  원격 분석에서 플러그인 찾기
</h2>

조직이 Claude Code의 [OpenTelemetry 이벤트](/docs/ko/monitoring-usage)를 자체 백엔드로 내보내면, [마켓플레이스 계층](#marketplace-tiers)은 어떤 플러그인 이름이 나타나는지 결정합니다:

* **[플러그인 로드 이벤트](/docs/ko/monitoring-usage#plugin-loaded-event)**: 이벤트는 공식 계층 플러그인 및 마켓플레이스 이름을 그대로 보고합니다. 커뮤니티 및 타사 계층의 경우, `plugin.name` 및 `marketplace.name`은 `OTEL_LOG_TOOL_DETAILS=1`을 설정하지 않는 한 리터럴 문자열 `third-party`입니다.
* **플러그인 범위**: 로드된 이벤트의 `plugin.scope`는 여전히 플러그인이 온 위치(예: 관리 설정이 활성화하는 플러그인의 경우 `org` 또는 다른 타사 플러그인의 경우 `user-local`)를 보고합니다. [플러그인 로드 이벤트](/docs/ko/monitoring-usage#plugin-loaded-event)는 모든 값을 나열합니다.
* **[플러그인 설치 이벤트](/docs/ko/monitoring-usage#plugin-installed-event)**: `OTEL_LOG_TOOL_DETAILS=1`을 설정하지 않는 한, 이벤트는 `third-party`를 보고하는 대신 비공식 플러그인의 이름 필드를 생략합니다.
* **[Claude Code Analytics API](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list)**: Claude Code는 공식 및 커뮤니티 계층의 플러그인을 이름으로 보고하고 다른 모든 플러그인을 `third-party`로 보고합니다.

<h2 id="next-steps">
  다음 단계
</h2>

* [조직을 위한 플러그인 관리](/docs/ko/plugins/org): 사용자가 설치할 수 있는 마켓플레이스를 제한하고 신뢰하는 마켓플레이스를 필수로 설정
* [플러그인 설치 및 관리](/docs/ko/plugins/install): 범위를 선택하기 전에 플러그인의 세부 정보 창을 검토
* [Anthropic의 마켓플레이스](/docs/ko/plugins/anthropic-marketplaces): 어느 마켓플레이스 이름이 Anthropic의 것인지
* [보안](/docs/ko/security): Claude Code의 자체 보안 모델
