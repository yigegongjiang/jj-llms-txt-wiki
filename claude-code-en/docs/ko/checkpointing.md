> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Checkpointing

> Claude의 편집 및 대화를 추적, 되돌리기 및 요약하여 세션 상태를 관리합니다.

Claude Code는 작업하면서 Claude의 파일 편집을 자동으로 추적하므로 변경 사항을 빠르게 실행 취소하고 문제가 발생한 경우 이전 상태로 되돌릴 수 있습니다.

<h2 id="how-checkpoints-work">
  Checkpoint의 작동 방식
</h2>

Claude와 함께 작업할 때 checkpointing은 각 사용자 프롬프트 전에 코드의 상태를 자동으로 캡처합니다. 이 안전장치를 통해 언제든지 이전 코드 상태로 돌아갈 수 있다는 확신을 가지고 야심 찬 대규모 작업을 수행할 수 있습니다.

<h3 id="automatic-tracking">
  자동 추적
</h3>

Claude Code는 파일 편집 도구로 수행된 모든 변경 사항을 추적합니다:

* 모든 사용자 프롬프트는 새로운 checkpoint를 생성합니다
* Claude Code는 세션의 가장 최근 100개 checkpoint에 대한 파일 스냅샷을 유지합니다. 이전 checkpoint를 삭제하면 남은 checkpoint가 참조하지 않는 스냅샷 파일이 삭제되며, 각 파일의 첫 번째 스냅샷은 예외입니다. 이 스냅샷은 VS Code 확장이 세션 diff의 기준선으로 사용합니다. v2.1.208 이전에는 이러한 대체된 스냅샷 파일이 세션이 정리될 때까지 디스크에 남아 있었습니다.
* Checkpoint는 세션과 함께 저장되므로 재개된 세션에서도 `/rewind`를 사용할 수 있습니다
* 30일 후 세션과 함께 자동으로 정리됩니다(구성 가능)

<h3 id="rewind-and-summarize">
  되돌리기 및 요약
</h3>

`/rewind`를 실행하거나 프롬프트 입력이 비어 있을 때 `Esc`를 두 번 눌러 rewind 메뉴를 엽니다.

<Note>
  프롬프트 입력에 텍스트가 포함되어 있으면 `Esc`를 두 번 누르면 메뉴를 열지 않고 대신 텍스트를 지웁니다. 지워진 텍스트는 입력 기록에 저장되므로 rewind 메뉴에서 작업을 마친 후 `Up`을 눌러 복구할 수 있습니다.
</Note>

Rewind 메뉴는 세션 중에 보낸 각 프롬프트를 나열합니다. 작업할 지점을 선택한 다음 작업을 선택합니다:

* **코드 및 대화 복원**: 코드와 대화를 해당 지점으로 되돌립니다
* **대화 복원**: 현재 코드를 유지하면서 해당 메시지로 되돌립니다
* **코드 복원**: 대화를 유지하면서 파일 변경 사항을 되돌립니다
* **여기서부터 요약**: 이 지점부터 이후의 대화를 요약으로 압축하여 context window 공간을 확보합니다
* **여기까지 요약**: 이 지점 이전의 대화를 요약으로 압축하여 이후 메시지를 그대로 유지합니다
* **취소**: 변경 사항을 적용하지 않고 메시지 목록으로 돌아갑니다

대화를 복원하거나 여기서부터 요약을 선택한 후 선택한 메시지의 원본 프롬프트가 입력 필드에 복원되므로 다시 보내거나 편집할 수 있습니다.

여기까지 요약을 선택하면 대화의 끝에 남겨지며 입력 필드는 비어 있습니다.

<h4 id="rewind-past-a-cleared-conversation">
  이전 세션의 지워진 대화로 되돌리기
</h4>

동일한 Claude Code 프로세스에서 이전에 `/clear`를 실행한 경우 rewind 메뉴는 목록 맨 위에 `/resume <session-id> (이전 세션)`이라는 레이블이 지정된 추가 항목을 표시합니다. 이를 선택하여 `/clear`가 실행되기 전에 활성화되었던 대화를 재개합니다. 이 항목은 Claude Code를 종료하거나 다른 세션을 재개할 때까지 사용 가능하며 Claude Code v2.1.191 이상이 필요합니다. 이전 버전에서는 `/resume`을 실행하고 목록에서 이전 세션을 선택합니다.

<h4 id="restore-vs-summarize">
  복원 vs. 요약
</h4>

복원 옵션은 상태를 되돌립니다: 코드 변경 사항, 대화 기록 또는 둘 다를 실행 취소합니다. 요약 옵션은 디스크의 파일을 변경하지 않으면서 대화의 일부를 AI 생성 요약으로 압축합니다:

* **여기서부터 요약**: 선택한 메시지 이전의 메시지는 그대로 유지됩니다. 선택한 메시지와 그 이후의 모든 메시지는 요약으로 대체됩니다. 초기 context를 완전한 세부 정보로 유지하면서 부수적인 논의를 버리려면 이를 사용합니다.
* **여기까지 요약**: 선택한 메시지 이전의 메시지는 요약으로 대체됩니다. 선택한 메시지와 그 이후의 모든 메시지는 그대로 유지되며 대화의 끝에 남겨집니다. 최근 작업을 완전한 세부 정보로 유지하면서 초기 설정 논의를 압축하려면 이를 사용합니다.

두 경우 모두 원본 메시지는 세션 기록에 보존되므로 Claude가 필요한 경우 세부 정보를 참조할 수 있습니다. 요약이 초점을 맞출 내용을 안내하기 위해 선택적 지침을 입력할 수 있습니다. 이는 `/compact`와 유사하지만 대상이 지정됩니다: 전체 대화를 요약하는 대신 선택한 메시지의 어느 쪽을 압축할지 선택합니다.

<Note>
  Summarize는 동일한 세션에 유지되고 context를 압축합니다. 원본 세션을 그대로 유지하면서 다른 접근 방식을 시도하고 싶다면 [fork](/docs/ko/sessions#branch-a-session) 대신 사용하세요(`claude --continue --fork-session`).
</Note>

<h2 id="common-use-cases">
  일반적인 사용 사례
</h2>

Checkpoint는 다음과 같은 경우에 특히 유용합니다:

* **대안 탐색**: 시작점을 잃지 않으면서 다양한 구현 접근 방식을 시도합니다
* **실수 복구**: 버그를 도입하거나 기능을 손상시킨 변경 사항을 빠르게 실행 취소합니다
* **기능 반복**: 작동하는 상태로 되돌릴 수 있다는 확신을 가지고 변형을 실험합니다
* **Context 공간 확보**: 초기 지침을 그대로 유지하면서 중간 지점부터 시작하여 자세한 디버깅 세션을 요약합니다

<h2 id="limitations">
  제한 사항
</h2>

<h3 id="bash-command-changes-not-tracked">
  Bash 명령 변경 사항이 추적되지 않음
</h3>

Checkpointing은 bash 명령으로 수정된 파일을 추적하지 않습니다. 예를 들어 Claude Code가 다음을 실행하는 경우:

```bash theme={null}
rm file.txt
mv old.txt new.txt
cp source.txt dest.txt
```

이러한 파일 수정 사항은 rewind를 통해 실행 취소할 수 없습니다. Claude의 파일 편집 도구를 통해 직접 수행된 파일 편집만 추적됩니다.

<h3 id="external-changes-not-tracked">
  외부 변경 사항이 추적되지 않음
</h3>

Checkpointing은 현재 세션 내에서 편집된 파일만 추적합니다. Claude Code 외부에서 수동으로 수행한 파일 변경 사항과 다른 동시 세션의 편집은 현재 세션과 동일한 파일을 수정하는 경우를 제외하고는 일반적으로 캡처되지 않습니다.

<h3 id="not-a-replacement-for-version-control">
  버전 관리의 대체가 아님
</h3>

Checkpoint는 빠른 세션 수준의 복구를 위해 설계되었습니다. 영구적인 버전 기록 및 협업을 위해:

* 커밋, 분기 및 장기 기록을 위해 버전 관리(예: Git)를 계속 사용합니다
* Checkpoint는 적절한 버전 관리를 보완하지만 대체하지 않습니다
* Checkpoint를 "로컬 실행 취소"로, Git을 "영구 기록"으로 생각하세요

<h2 id="see-also">
  참고 항목
</h2>

* [Interactive mode](/docs/ko/interactive-mode) - 키보드 단축키 및 세션 제어
* [Commands](/docs/ko/commands) - `/rewind`를 사용하여 checkpoint에 액세스
* [CLI reference](/docs/ko/cli-reference) - 명령줄 옵션
