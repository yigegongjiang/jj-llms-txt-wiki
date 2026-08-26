> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# CLI에서 플러그인 추천하기

> CLI에서 한 줄 마커를 내보내어 Claude Code가 사용자에게 공식 플러그인 설치를 권유하도록 합니다.

CLI 또는 SDK를 유지 관리하고 공식 Anthropic 마켓플레이스에 플러그인이 있다면, 도구가 Claude Code 사용자에게 해당 플러그인을 설치하도록 권유할 수 있습니다. CLI는 Claude Code 내부에서 실행 중임을 감지할 때 stderr에 한 줄 마커를 작성합니다. Claude Code는 마커를 읽고 출력에서 제거한 후 사용자에게 일회성 설치 프롬프트를 표시합니다.

Claude Code는 힌트 줄을 명령 출력에서 제거한 후 모델로 전송하므로 마커는 대화에 나타나지 않으며 토큰 사용량에 계산되지 않습니다. 이 프로토콜은 추가 명령이 필요하지 않으며 Claude Code 외부에서 CLI를 실행하는 사용자에게 출력되는 내용을 변경하지 않습니다.

이 페이지는 CLI 및 SDK 유지 관리자를 위한 것입니다. 플러그인 설치를 찾고 있다면 [플러그인 발견 및 설치](/docs/ko/discover-plugins)를 참조하세요.

<h2 id="how-it-works">
  작동 방식
</h2>

Claude Code는 Bash 및 PowerShell 도구를 통해 실행하는 모든 명령과 [hook](/docs/ko/hooks) 명령에 대해 [`CLAUDECODE`](/docs/ko/env-vars) 환경 변수를 `1`로 설정합니다. v2.1.172부터는 해당 동일한 서브프로세스에서 [`CLAUDE_CODE_CHILD_SESSION`](/docs/ko/env-vars)도 `1`로 설정합니다. CLI가 이러한 변수 중 하나를 감지하면 자체 종료 `<claude-code-hint />` 태그를 stderr에 작성합니다. hook 명령에서 힌트 태그는 제거되고 무시됩니다. Bash 및 PowerShell 도구 출력만 설치 프롬프트를 트리거합니다.

Claude Code가 명령 출력을 받으면 다음을 수행합니다:

1. 힌트 줄을 스캔하고 출력이 모델에 도달하기 전에 제거합니다
2. 힌트가 공식 Anthropic 마켓플레이스의 플러그인을 대상으로 하는지 확인합니다
3. 플러그인이 이미 설치되지 않았으며 이전에 프롬프트되지 않았는지 확인합니다
4. 힌트를 내보낸 명령의 이름을 지정하는 설치 프롬프트를 사용자에게 표시합니다

Claude Code는 플러그인을 자동으로 설치하지 않습니다. 사용자가 항상 확인합니다.

<h2 id="emit-the-hint">
  힌트 내보내기
</h2>

힌트 프롬프트는 공식 Anthropic 마켓플레이스에 나열된 플러그인에 대해서만 실행됩니다. 통합을 배포하기 전에 [플러그인을 공식 마켓플레이스에 등록하기](#get-your-plugin-into-the-official-marketplace)를 참조하세요.

환경 변수에서 내보내기를 제어하여 마커가 일반 사용자가 CLI를 직접 실행할 때 나타나지 않도록 한 다음, 태그를 stderr에 자체 줄로 작성합니다. 확인할 변수를 선택합니다:

* `CLAUDECODE`: 모든 Claude Code 버전에서 설정되므로 가장 많은 세션에 도달합니다. Claude Code가 시작하는 tmux 세션 및 stdio MCP 서버 서브프로세스에서도 설정되며, IDE 확장 프로그램은 일반 사용자가 CLI를 직접 실행할 수 있는 통합 터미널에서 설정합니다.
* `CLAUDE_CODE_CHILD_SESSION`: 도구 호출, 훅 명령 및 [상태 줄](/docs/ko/statusline) 명령과 같이 Claude Code 자체가 생성하는 서브프로세스에서만 설정되므로 태그가 일반적으로 사용자 터미널에 도달하지 않습니다. tmux 서버와 같이 세션 내에서 시작된 장기 실행 프로세스는 변수를 캡처하므로 해당 프로세스에서 나중에 시작된 셸은 여전히 원본 태그를 표시합니다. Claude Code v2.1.172 이상이 필요하므로 이전 버전의 세션은 힌트를 놓칩니다.

다음 예제는 최대 도달 범위를 위해 `CLAUDECODE`에서 제어하고 공식 마켓플레이스의 `example-cli`라는 플러그인에 대한 힌트를 내보냅니다:

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
  [ -n "$CLAUDECODE" ] &&
    printf '%s\n' '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />' >&2
  ```
</CodeGroup>

공식 마켓플레이스에서 플러그인의 이름으로 `example-cli`를 바꿉니다.

<h2 id="choose-where-to-emit">
  내보낼 위치 선택
</h2>

힌트를 내보낼 코드 경로를 제어합니다. Claude Code는 플러그인별로 중복 제거하므로 모든 호출에서 내보내는 것은 단점이 없습니다. 잘 작동하는 접점은 다음과 같습니다:

| 배치              | 작동하는 이유                                  |
| :-------------- | :--------------------------------------- |
| `--help` 출력     | Claude는 종종 익숙하지 않은 CLI를 탐색할 때 도움말을 실행합니다 |
| 알 수 없는 하위 명령 오류 | Claude가 인터페이스에 대해 혼동하는 순간에 도달합니다         |
| 로그인 또는 인증 성공    | 사용자가 이미 설정 마음가짐에 있습니다                    |
| 첫 실행 환영 메시지     | 자연스러운 온보딩 순간입니다                          |

<h2 id="what-the-user-sees">
  사용자가 보는 것
</h2>

힌트가 모든 검사를 통과하면 Claude Code는 다음과 같은 프롬프트를 표시합니다:

```text theme={null}
─────────────────────────────────────────────────────────────
  플러그인 추천

    example-cli 명령이 플러그인 설치를 제안합니다.

    플러그인: example-cli
    마켓플레이스: claude-plugins-official
    example-cli 배포를 위한 공식 통합

    설치하시겠습니까?
    ❯ 1. 예, example-cli 설치
      2. 아니오
      3. 아니오, 플러그인 설치 힌트를 다시 표시하지 않기

─────────────────────────────────────────────────────────────
```

프롬프트는 힌트를 생성한 명령의 이름을 지정하므로 사용자가 도구와 권장하는 플러그인 간의 불일치를 발견할 수 있습니다. 사용자가 30초 이내에 응답하지 않으면 프롬프트는 **아니오**로 해제됩니다.

프롬프트 빈도는 제한됩니다:

* **플러그인당 한 번**: 프롬프트가 표시된 후 Claude Code는 플러그인을 기록하고 사용자의 답변에 관계없이 다시 프롬프트하지 않습니다.
* **세션당 한 번**: 머신의 모든 CLI에서 Claude Code 세션당 최대 하나의 힌트 프롬프트가 나타납니다.

**예**를 선택하면 플러그인이 사용자 범위로 설치됩니다. **아니오, 플러그인 설치 힌트를 다시 표시하지 않기**를 선택하면 사용자에 대한 모든 향후 힌트 프롬프트가 비활성화됩니다.

<h2 id="hint-format">
  힌트 형식
</h2>

힌트는 세 개의 필수 속성이 있는 자체 종료 태그입니다.

```text theme={null}
<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />
```

| 속성      | 필수 | 설명                              |
| :------ | :- | :------------------------------ |
| `v`     | 예  | 프로토콜 버전. `1`은 유일하게 지원되는 값입니다    |
| `type`  | 예  | 힌트 종류. `plugin`은 유일하게 지원되는 값입니다 |
| `value` | 예  | `name@marketplace` 형식의 플러그인 식별자 |

속성 값은 큰따옴표로 인용하거나 인용하지 않을 수 있습니다. 인용하지 않은 값은 공백을 포함할 수 없습니다. 이스케이프 시퀀스는 지원되지 않습니다.

<h2 id="requirements">
  요구 사항
</h2>

Claude Code는 힌트에 대해 조치를 취하기 전에 두 가지 조건을 적용합니다. 두 검사 중 하나라도 실패한 힌트는 삭제됩니다:

* **자체 줄**: 태그는 자체 줄을 차지해야 합니다. 예를 들어 로그 문 내부에 줄 중간에 포함된 태그는 무시됩니다. 줄의 선행 및 후행 공백은 허용됩니다.
* **공식 마켓플레이스**: `value`는 `claude-plugins-official`과 같은 Anthropic 제어 마켓플레이스의 플러그인을 참조해야 합니다. 다른 마켓플레이스를 가리키는 힌트는 자동으로 삭제됩니다.

힌트 줄은 버전 또는 유형이 인식되지 않을 때도 항상 모델에 도달하기 전에 출력에서 제거되므로 마커는 토큰 사용량에 계산되지 않습니다.

나머지 지침은 권장되지만 적용되지 않습니다. Claude Code는 CLI가 이를 따르는지 관찰할 수 없습니다:

* **stderr에 작성**: stderr는 `example-cli deploy | jq`와 같은 셸 파이프라인에서 태그를 제외합니다. Claude Code는 두 스트림을 모두 스캔하므로 stdout도 작동합니다.
* **환경 변수에서 제어**: `CLAUDECODE` 또는 `CLAUDE_CODE_CHILD_SESSION`이 설정된 경우에만 내보냅니다. 두 변수의 차이점은 [힌트 내보내기](#emit-the-hint)를 참조하세요.

<h2 id="get-your-plugin-into-the-official-marketplace">
  공식 마켓플레이스에 플러그인 추가
</h2>

힌트 프로토콜은 공식 Anthropic 마켓플레이스 `claude-plugins-official`에 나열된 플러그인에 대해서만 적용됩니다. Anthropic은 자신의 재량에 따라 해당 마켓플레이스를 큐레이션하며, 앱 내 제출 양식은 플러그인을 [커뮤니티 마켓플레이스](/docs/ko/plugins#submit-your-plugin-to-the-community-marketplace)에 추가합니다. 힌트 프로토콜은 이를 확인하지 않습니다. Anthropic 파트너 담당자와 함께 작업 중인 경우 그들에게 연락하여 공식 마켓플레이스 목록을 조정하세요.

<h2 id="see-also">
  참고 항목
</h2>

* [플러그인 만들기](/docs/ko/plugins): CLI가 권장하는 플러그인 빌드
* [플러그인 마켓플레이스 만들기 및 배포](/docs/ko/plugin-marketplaces): 공식 마켓플레이스 외부에서 플러그인 호스팅
* [환경 변수](/docs/ko/env-vars): `CLAUDECODE` 및 관련 변수에 대한 전체 참조
