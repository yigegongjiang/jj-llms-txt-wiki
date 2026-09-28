> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# CLI에서 플러그인 추천하기

> CLI 또는 SDK에서 claude-code-hint 태그를 내보내 Claude Code 사용자에게 공식 마켓플레이스 플러그인 설치를 유도합니다.

CLI 또는 SDK를 유지 관리하는 경우, 도구가 Claude Code 사용자에게 플러그인 설치를 유도할 수 있습니다. CLI가 Claude Code 내부에서 실행 중임을 감지하면, 한 줄의 `<claude-code-hint />` 태그를 stderr에 작성합니다. Claude Code는 모델이 출력을 보기 전에 Bash 및 PowerShell 도구 출력에서 해당 줄을 제거한 후, 사용자에게 일회성 설치 프롬프트를 표시합니다.

이 페이지는 플러그인이 `claude-plugins-official` 또는 Anthropic의 [공식 마켓플레이스 이름](/docs/ko/plugins/security#official-marketplace-names) 중 하나인 다른 마켓플레이스에 나열된 경우에만 적용됩니다. 커뮤니티 마켓플레이스인 `claude-community`는 해당하지 않습니다.

<Note>
  플러그인을 게시하려면 [플러그인 게시 및 배포](/docs/ko/plugins/publish)를 참조하세요.
</Note>

<h2 id="emit-the-hint">
  힌트 내보내기
</h2>

`CLAUDECODE` 또는 `CLAUDE_CODE_CHILD_SESSION`이 설정된 경우에만 태그를 내보내므로, 사용자가 CLI를 직접 실행할 때는 나타나지 않습니다.

Claude Code는 Bash 및 PowerShell 도구를 통해 실행하는 명령과 훅 명령에서 `CLAUDECODE=1`을 설정합니다. v2.1.172 이상에서는 `CLAUDE_CODE_CHILD_SESSION=1`도 설정합니다. 변수는 이를 전달하는 프로세스가 다릅니다:

* **`CLAUDECODE`**: 모든 Claude Code 버전에서 설정됩니다. IDE 확장 프로그램도 통합 터미널에서 설정하므로, `CLAUDECODE`만으로 게이트하면 사용자가 해당 터미널 중 하나에서 CLI를 직접 실행할 때도 태그가 내보내집니다.
* **`CLAUDE_CODE_CHILD_SESSION`**: Claude Code 자체가 시작하는 서브프로세스에서만 설정됩니다. v2.1.172 이상이 필요한 경우 사용하세요.

[환경 변수 참조](/docs/ko/env-vars)에 세부 정보가 있습니다.

다음 예제는 가장 광범위한 도달을 위해 `CLAUDECODE`로 게이트하고 공식 마켓플레이스의 `example-cli`라는 플러그인에 대한 힌트를 내보냅니다:

<CodeGroup>
  ```javascript Node.js theme={null}
  if (process.env.CLAUDECODE) {
    process.stderr.write(
      '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />\n',
    )
  }
  ```

  ```python Python theme={null}
  import os, sys

  if os.environ.get("CLAUDECODE"):
      print(
          '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />',
          file=sys.stderr,
      )
  ```

  ```go Go theme={null}
  if os.Getenv("CLAUDECODE") != "" {
      fmt.Fprintln(os.Stderr,
          `<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />`)
  }
  ```

  ```shell Shell theme={null}
  if [ -n "$CLAUDECODE" ]; then
    printf '%s\n' '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />' >&2
  fi
  ```
</CodeGroup>

공식 마켓플레이스에서 플러그인의 이름으로 `example-cli`를 바꾸세요.

Claude Code가 각 플러그인에 대해 한 번씩 프롬프트하므로 모든 호출에서 힌트를 내보낼 수 있습니다.

내보내기를 확인하려면 터미널에서 `CLAUDECODE=1 example-cli`를 실행하고 태그 줄이 stderr에 나타나는지 확인한 후, 변수 없이 `example-cli`를 실행하고 추가 항목이 인쇄되지 않는지 확인하세요.

<h2 id="hint-format">
  힌트 형식
</h2>

태그는 자체 줄을 차지해야 합니다. Claude Code는 줄 중간에 포함된 태그를 무시합니다.

태그는 세 가지 속성을 사용하며, 모두 필수입니다:

| 속성      | 설명                               |
| :------ | :------------------------------- |
| `v`     | 프로토콜 버전. `1`이 유일하게 지원되는 값입니다.    |
| `type`  | 힌트 종류. `plugin`이 유일하게 지원되는 값입니다. |
| `value` | `name@marketplace` 형식의 플러그인 식별자  |

값은 큰따옴표로 묶거나 따옴표 없이 사용할 수 있습니다. 따옴표 없는 값은 공백을 포함할 수 없습니다.

Claude Code는 `v` 또는 `type`이 인식되지 않을 때도 줄을 출력에서 제거합니다.

<h2 id="check-when-the-prompt-appears">
  프롬프트가 나타나는 시기 확인
</h2>

프롬프트는 대화형 터미널 세션에서만 나타납니다. `claude -p` 실행, 서브에이전트 실행, 훅 명령 출력에서는 태그가 제거되고 프롬프트가 표시되지 않습니다. 다음 확인 사항도 모두 통과해야 합니다:

* **공식 및 설치 가능**: `value`가 Claude Code가 공식 마켓플레이스의 로컬 복사본에서 찾은 플러그인을 지정하고, 아직 설치되지 않았으며, 정책이 차단하지 않습니다.
* **분석 켜짐**: Claude Code의 분석이 꺼진 세션(예: `DISABLE_TELEMETRY`, `DO_NOT_TRACK` 또는 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`이 설정된 세션)이나 Amazon Bedrock과 같은 타사 제공자의 세션에서는 프롬프트가 표시되지 않습니다. 여기서 [자동 원격 분석 옵트아웃](/docs/ko/data-usage#default-behaviors-by-api-provider)이 적용됩니다.
* **빈도 제한**: 세션당 한 번의 프롬프트, 사용자의 답변과 관계없이 플러그인당 한 번의 프롬프트, 그리고 해당 머신에서 100개의 플러그인에 대해 프롬프트가 표시된 후에는 없습니다.
* **꺼지지 않음**: 사용자가 **아니오, 플러그인 설치 힌트를 다시 표시하지 않기**를 선택하지 않았습니다.
* **로컬, 참석 세션**: 세션의 작업 공간이 클라우드나 원격 머신이 아닌 로컬이고, 세션이 무인으로 실행되지 않습니다. 예를 들어 `--cloud`로 시작된 세션, 원격 제어를 제공하는 세션, 또는 에이전트 팀 팀원은 프롬프트를 표시하지 않습니다.

<h2 id="preview-what-the-user-sees">
  사용자가 보는 내용 미리보기
</h2>

[프롬프트가 나타나는 시기 확인](#check-when-the-prompt-appears)의 확인 사항이 통과하면, Claude Code는 다음과 같은 **플러그인 추천** 대화 상자를 표시합니다:

```text theme={null}
─────────────────────────────────────────────────────────────
  Plugin recommendation

    The example-cli command suggests installing a plugin.

    Plugin: example-cli
    Marketplace: claude-plugins-official
    Description: Official integration for example-cli deployments

    Would you like to install it?
    ❯ 1. Yes, install
      2. No
      3. No, and don't show plugin installation hints again

─────────────────────────────────────────────────────────────
```

대화 상자는 Claude가 실행한 셸 명령의 첫 번째 단어를 지정하므로 사용자가 불일치를 발견할 수 있습니다. 각 답변은 하나의 효과를 가집니다:

* **예, 설치**: [사용자 범위](/docs/ko/plugins/install)에서 플러그인을 설치합니다.
* **아니오, 플러그인 설치 힌트를 다시 표시하지 않기**: 해당 사용자에 대한 향후 힌트 프롬프트를 끕니다.
* **30초 동안 답변 없음**: **아니오**로 계산됩니다.

<h2 id="next-steps">
  다음 단계
</h2>

* [플러그인 게시 및 배포](/docs/ko/plugins/publish): 힌트가 필요한 공식 마켓플레이스를 포함한 각 마켓플레이스로의 경로
* [플러그인 명령 참조](/docs/ko/plugins/cli-reference#plugin-install): 세션 외부에서 동일한 플러그인을 설치하는 셸 명령
