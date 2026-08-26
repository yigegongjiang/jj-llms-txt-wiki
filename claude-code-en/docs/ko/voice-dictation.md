> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 음성 받아쓰기

> Claude Code CLI에서 누르고 있기 또는 탭하기 음성 받아쓰기로 프롬프트를 말씀하세요.

Claude Code CLI에서 입력하는 대신 프롬프트를 말씀하세요. 음성이 프롬프트 입력으로 실시간 전사되므로 같은 메시지에서 음성과 입력을 혼합할 수 있습니다. `/voice`로 받아쓰기를 활성화한 다음, 말하는 동안 키를 누르고 있거나 한 번 탭하여 시작하고 다시 탭하여 전송합니다.

<Note>
  탭 모드에는 Claude Code v2.1.116 이상이 필요합니다. `claude --version`으로 버전을 확인하세요.
</Note>

받아쓰기는 [에이전트 보기](/docs/ko/agent-view#peek-and-reply)에서도 작동합니다. 디스패치 입력 또는 피크 패널 회신이 포커스되어 있는 동안 푸시투톡 키를 누르고 있거나 탭하여 백그라운드 세션에 받아쓰기합니다.

<h2 id="requirements">
  요구 사항
</h2>

음성 받아쓰기는 기록된 오디오를 Anthropic의 서버로 스트리밍하여 전사합니다. 오디오는 로컬에서 처리되지 않습니다. 다음 모든 항목이 필요합니다:

* **Claude.ai 계정**: 음성 텍스트 변환 서비스는 Claude.ai 계정으로 인증할 때만 사용 가능하며, Claude Code가 Anthropic API 키, Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry를 직접 사용하도록 구성된 경우에는 사용할 수 없습니다.
* **HIPAA 규정 준수가 활성화되지 않은 조직**: 이 제한이 적용되면 `/voice`에 `Voice mode is disabled by your organization's policy`가 표시됩니다.
* **로컬 마이크**: 음성 받아쓰기는 [웹의 Claude Code](/docs/ko/claude-code-on-the-web) 또는 SSH 세션과 같은 원격 환경에서는 작동하지 않습니다.
* **WSL에서 Claude Code를 실행하는 경우 WSLg**: WSLg는 Windows 10 또는 11의 Microsoft Store에서 설치된 WSL2에 포함되어 있습니다. WSLg를 사용할 수 없는 경우(예: WSL1), 대신 기본 Windows에서 Claude Code를 실행하세요.

전사는 Claude 메시지나 토큰을 소비하지 않으며 `/usage`에 표시된 한도에 포함되지 않습니다. Anthropic이 데이터를 처리하는 방법은 [데이터 사용](/docs/ko/data-usage)을 참조하세요.

오디오 녹음은 macOS, Linux 및 Windows의 기본 제공 네이티브 모듈을 사용합니다. Linux에서 네이티브 모듈을 로드할 수 없으면 Claude Code는 ALSA utils의 `arecord` 또는 SoX의 `rec`으로 폴백합니다. 둘 다 사용할 수 없으면 `/voice`는 패키지 관리자에 대한 설치 명령을 출력합니다.

Claude Code [VS Code 확장](/docs/ko/vs-code)도 동일한 Claude.ai 계정 요구 사항으로 음성 받아쓰기를 지원합니다. SSH, Dev Containers 및 Codespaces를 포함한 VS Code Remote 세션에서는 사용할 수 없습니다. 마이크가 로컬 머신에 있고 확장이 원격 호스트에서 실행되기 때문입니다.

<h2 id="enable-voice-dictation">
  음성 받아쓰기 활성화
</h2>

`/voice`를 실행하여 받아쓰기를 활성화합니다. 처음 활성화할 때 Claude Code는 마이크 확인을 실행합니다. macOS에서는 터미널에 대한 시스템 마이크 권한 프롬프트를 트리거합니다(이전에 부여되지 않은 경우).

```
/voice
Voice mode enabled (hold). Hold space to record. Dictation language: en (/config to change).
```

`/voice`는 선택적 모드 인수를 허용합니다:

| 명령            | 효과                                    |
| :------------ | :------------------------------------ |
| `/voice`      | 켜거나 끄기, 현재 모드 유지                      |
| `/voice hold` | [누르고 있기 모드](#hold-to-record)에서 활성화    |
| `/voice tap`  | [탭 모드](#tap-to-record-and-send)에서 활성화 |
| `/voice off`  | 비활성화                                  |

음성 받아쓰기는 세션 간에 유지됩니다. `/voice`를 실행하는 대신 [사용자 설정 파일](/docs/ko/settings)에서 직접 설정하세요:

```json theme={null}
{
  "voice": {
    "enabled": true,
    "mode": "tap"
  }
}
```

음성 받아쓰기가 활성화되어 있는 동안 입력 바닥글은 프롬프트가 비어 있을 때 `hold space to speak` 힌트를 표시합니다. 힌트는 현재 `voice:pushToTalk` 바인딩을 반영하며, [받아쓰기 키를 다시 바인딩](#rebind-the-dictation-key)하면 업데이트됩니다. 힌트 텍스트는 두 모드 모두에서 동일하며, [사용자 정의 상태 줄](/docs/ko/statusline)이 구성된 경우 나타나지 않습니다.

전사는 두 모드 모두에서 코딩 어휘에 맞게 조정됩니다. `regex`, `OAuth`, `JSON` 및 `localhost`와 같은 일반적인 개발 용어가 올바르게 인식되며, 현재 프로젝트 이름과 git 분기 이름이 자동으로 인식 힌트로 추가됩니다.

<h2 id="hold-to-record">
  누르고 있기로 녹음
</h2>

누르고 있기 모드는 푸시 투 토크입니다: 키를 누르고 있는 동안 녹음이 실행되고 놓으면 중지됩니다. 이것이 기본 모드입니다.

`Space`를 누르고 있어서 녹음을 시작합니다. Claude Code는 터미널에서 빠른 키 반복 이벤트를 감시하여 누르고 있는 키를 감지하므로 녹음이 시작되기 전에 짧은 워밍업이 있습니다. 바닥글은 워밍업 중에 `keep holding…`을 표시한 다음 녹음이 활성화되면 실시간 파형으로 전환됩니다.

처음 몇 개의 키 반복 문자는 워밍업 중에 입력으로 입력되며 녹음이 활성화될 때 자동으로 제거됩니다. 단일 `Space` 탭은 여전히 공백을 입력합니다. 누르고 있기 감지는 빠른 반복에서만 트리거되기 때문입니다.

<Tip>
  워밍업을 건너뛰려면 `/voice tap`으로 [탭 모드](#tap-to-record-and-send)로 전환하거나 [받아쓰기 키를 다시 바인딩](#rebind-the-dictation-key)하여 `meta+k`와 같은 수정자 조합을 사용하세요. 수정자 조합은 첫 번째 키 누름에서 녹음을 시작합니다.
</Tip>

음성이 프롬프트에 말하는 동안 나타나며, 전사가 완료될 때까지 흐려집니다. `Space`를 놓아서 녹음을 중지하고 텍스트를 완료합니다. 전사는 커서 위치에 삽입되고 커서는 삽입된 텍스트의 끝에 유지되므로 입력과 받아쓰기를 어떤 순서로든 혼합할 수 있습니다. `Space`를 다시 누르고 있어서 다른 녹음을 추가하거나 먼저 커서를 이동하여 프롬프트의 다른 곳에 음성을 삽입하세요:

```
> refactor the auth middleware to ▮
  # hold Space, speak "use the new token validation helper"
> refactor the auth middleware to use the new token validation helper▮
```

기본적으로 키를 놓으면 전사를 삽입하고 `Enter`를 누를 때까지 기다립니다. `voice` 설정 객체에서 `"autoSubmit": true`를 설정하여 전사가 최소 3단어 이상이면 키를 놓을 때 프롬프트를 자동으로 전송합니다.

<h2 id="tap-to-record-and-send">
  탭하여 녹음 및 전송
</h2>

탭 모드는 단일 키 누름으로 녹음을 전환합니다: 한 번 탭하여 시작하고, 말한 다음, 다시 탭하여 프롬프트를 전송합니다. 워밍업이 없으며 키를 누르고 있을 필요가 없습니다.

`/voice tap`으로 탭 모드를 활성화합니다. 프롬프트 입력이 비어 있으면 `Space`를 탭하여 녹음을 시작합니다. 바닥글은 녹음 중에 실시간 파형을 표시합니다. `Space`를 다시 탭하여 중지합니다.

Claude Code는 전사를 삽입하고 전사가 최소 3단어 이상이면 프롬프트를 자동으로 제출합니다. 더 짧은 전사는 삽입되지만 제출되지 않으므로 실수로 탭해도 단어가 전송되지 않습니다.

3단어 임계값은 공백 없이 작성된 언어의 단어를 계산합니다. v2.1.195 기준으로 일본어, 중국어, 태국어 전사는 개별 단어를 계산하므로 탭 모드와 `autoSubmit`이 있는 홀드 모드에서 자동으로 제출됩니다. 이전 버전은 공백이 없는 전사를 한 단어로 계산했으며 자동으로 제출하지 않았습니다.

첫 번째 탭은 프롬프트 입력이 비어 있을 때만 녹음을 시작하므로 메시지를 작성하는 동안 여전히 공백을 정상적으로 입력할 수 있습니다. 두 번째 탭은 입력 내용에 관계없이 녹음을 중지합니다. 녹음은 또한 15초의 침묵 또는 2분 총 시간 후 자동으로 중지됩니다.

<h2 id="change-the-dictation-language">
  받아쓰기 언어 변경
</h2>

음성 받아쓰기는 Claude의 응답 언어를 제어하는 동일한 [`language` 설정](/docs/ko/settings)을 사용합니다. 해당 설정이 비어 있으면 받아쓰기는 기본적으로 영어입니다. VS Code 확장에서 `language`가 비어 있으면 받아쓰기는 VS Code의 `accessibility.voice.speechLanguage` 설정을 사용한 후 기본적으로 영어로 설정됩니다.

<Accordion title="지원되는 받아쓰기 언어">
  | 언어     | 코드   |
  | :----- | :--- |
  | 체코어    | `cs` |
  | 덴마크어   | `da` |
  | 네덜란드어  | `nl` |
  | 영어     | `en` |
  | 프랑스어   | `fr` |
  | 독일어    | `de` |
  | 그리스어   | `el` |
  | 힌디어    | `hi` |
  | 인도네시아어 | `id` |
  | 이탈리아어  | `it` |
  | 일본어    | `ja` |
  | 한국어    | `ko` |
  | 노르웨이어  | `no` |
  | 폴란드어   | `pl` |
  | 포르투갈어  | `pt` |
  | 러시아어   | `ru` |
  | 스페인어   | `es` |
  | 스웨덴어   | `sv` |
  | 터키어    | `tr` |
  | 우크라이나어 | `uk` |
</Accordion>

`/config`에서 또는 설정에서 직접 언어를 설정합니다. [BCP 47 언어 코드](https://en.wikipedia.org/wiki/IETF_language_tag) 또는 언어 이름을 사용할 수 있습니다:

```json theme={null}
{
  "language": "japanese"
}
```

`language` 설정이 지원되는 목록에 없으면 `/voice`는 활성화할 때 경고하고 받아쓰기를 위해 영어로 폴백합니다. Claude의 텍스트 응답은 이 폴백의 영향을 받지 않습니다.

<h2 id="rebind-the-dictation-key">
  받아쓰기 키 다시 바인딩
</h2>

받아쓰기 키는 `Chat` 컨텍스트에서 `voice:pushToTalk`에 바인딩되고 기본값은 `Space`입니다. 동일한 바인딩이 누르고 있기 및 탭 모드를 모두 제어합니다. [`~/.claude/keybindings.json`](/docs/ko/keybindings)에서 다시 바인딩하세요:

```json theme={null}
{
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "meta+k": "voice:pushToTalk",
        "space": null
      }
    }
  ]
}
```

`voice:pushToTalk` 작업은 한 번에 하나의 키를 사용합니다. 사용자 정의 키를 바인딩하면 기본 `Space` 바인딩을 대체하며 두 번째 트리거를 추가하지 않으므로, 이 예제의 `"space": null` 줄은 명확성을 위한 것이며 동작을 변경하지 않고 생략할 수 있습니다.

누르고 있기 모드에서는 `v`와 같은 단순 문자 키 바인딩을 피하세요. 누르고 있기 감지는 키 반복에 의존하고 문자는 워밍업 중에 프롬프트로 입력되기 때문입니다. `Space`를 사용하거나 `meta+k`와 같은 수정자 조합을 사용하여 워밍업 없이 첫 번째 키 누름에서 녹음을 시작하세요. 탭 모드에는 워밍업이 없으므로 대부분의 키가 작동합니다.

일부 키는 터미널 애플리케이션에 전달되지 않으며 전혀 바인딩할 수 없습니다. 예를 들어, `Caps Lock`을 바인딩하려고 하면 오류가 표시됩니다. 전체 키바인딩 구문 및 예약된 단축키 목록은 [키보드 단축키 사용자 정의](/docs/ko/keybindings)를 참조하세요.

<h2 id="troubleshooting">
  문제 해결
</h2>

음성 받아쓰기가 활성화되지 않거나 녹음되지 않을 때의 일반적인 문제:

* **`Voice mode requires a Claude.ai account`**: API 키 또는 타사 공급자로 인증되었습니다. `/login`을 실행하여 Claude.ai 계정으로 로그인하세요.
* **`Voice mode is disabled by your organization's policy`**: 조직의 규정 준수 구성이 음성 받아쓰기를 비활성화합니다. [요구 사항](#requirements)에 설명되어 있습니다. 조직 관리자에게 연락하여 조직에서 음성 받아쓰기를 사용할 수 있는지 확인하세요.
* **`Microphone access is denied`**: 시스템 설정에서 터미널에 마이크 권한을 부여하세요. macOS에서는 시스템 설정 → 개인정보 보호 및 보안 → 마이크로 이동하여 터미널 앱을 활성화한 다음 `/voice`를 다시 실행하세요. Windows에서는 설정 → 개인정보 보호 및 보안 → 마이크로 이동하여 데스크톱 앱에 대한 마이크 접근을 켜세요. 그런 다음 `/voice`를 다시 실행하세요. 터미널이 macOS 설정에 나열되지 않으면 [macOS 마이크 설정에 나열되지 않은 터미널](#terminal-not-listed-in-macos-microphone-settings)을 참조하세요.
* **Linux에서 `No audio recording tool found`**: 네이티브 오디오 모듈을 로드할 수 없고 폴백이 설치되지 않았습니다. 오류 메시지에 표시된 명령으로 SoX를 설치하세요. 예: `sudo apt-get install sox`.
* **`Voice mode requires a microphone, but SoX could not open an audio capture device`**: SoX가 설치되어 있지만 호스트에 오디오 캡처 장치가 없습니다. 예를 들어 헤드리스 서버 또는 컨테이너입니다. 마이크가 있는 머신에서 Claude Code를 실행하세요. v2.1.195부터 Linux의 Claude Code는 이 상황에서 이 메시지를 보고합니다. 이전 버전은 SoX가 이미 설치되어 있어도 설치하도록 요청했습니다.
* **`Voice mode could not find a working audio recorder in WSL`**: WSLg는 ALSA 장치가 아닌 PulseAudio를 통해 오디오를 라우팅하므로 SoX는 PulseAudio 백엔드가 명시적으로 설치되어야 합니다. `sudo apt install sox libsox-fmt-pulse`를 실행하세요. `sox`만 설치하면 ALSA 백엔드가 함께 설치되는데, WSL에서는 `/dev/snd` 장치가 없기 때문에 녹음할 수 없습니다.
* **`Voice input is failing repeatedly and has been paused`**: 음성 받아쓰기가 여러 캡처 실패를 겪었고 하나가 성공할 때까지 새 세션 시도를 중단했습니다. 마이크가 시작되지 않거나 레코더가 시작되었다가 오디오를 생성하지 않고 중지되는지 여부에 관계없이 실패가 계산됩니다. 이는 일반적으로 이 호스트의 마이크 또는 오디오 스택이 오디오를 캡처할 수 없음을 의미합니다. 예를 들어 헤드리스 서버, 오디오 패스스루가 없는 원격 셸 또는 거부된 마이크 권한이 있습니다. 작동하는 입력 장치를 확인하고 위의 항목에서 근본 원인을 해결한 다음 음성을 다시 트리거하세요. v2.1.202 이전에는 시작 실패만 일시 중지로 계산되었습니다.
* **누르고 있기 모드에서 `Space`를 누르고 있어도 아무것도 일어나지 않음**: 누르고 있는 동안 프롬프트 입력을 봅니다. 공백이 계속 누적되면 음성 받아쓰기가 꺼져 있을 가능성이 높습니다. `/voice hold`를 실행하여 활성화하세요. 1\~2개의 공백만 나타나고 그 다음 아무것도 없으면 음성 받아쓰기는 켜져 있지만 누르고 있기 감지가 트리거되지 않습니다. 누르고 있기 감지는 터미널이 키 반복 이벤트를 보내야 하므로 OS 수준에서 키 반복이 비활성화되면 누르고 있는 키를 감지할 수 없습니다. 키 반복 요구 사항을 피하려면 `/voice tap`으로 탭 모드로 전환하세요.
* **탭 모드에서 `Space`를 탭하면 녹음 대신 공백을 입력함**: 첫 번째 탭은 프롬프트 입력이 비어 있을 때만 녹음을 시작합니다. 먼저 입력을 지우거나 `/voice tap`을 실행하여 탭 모드에 있는지 확인하세요.
* **`No audio detected from microphone`**: 녹음이 시작되었지만 침묵을 캡처했습니다. 올바른 입력 장치가 시스템 기본값으로 설정되어 있고 입력 수준이 음소거되거나 0에 가깝지 않은지 확인하세요. Windows에서는 설정 → 시스템 → 사운드 → 입력을 열고 마이크를 선택하세요. macOS에서는 시스템 설정 → 사운드 → 입력을 열어보세요.
* **`Voice connection failed`**: 연결이 실패했기 때문에 녹음이 전사 서비스에 도달하지 않았습니다. 네트워크를 확인하고 다시 시도하세요. 오디오를 캡처하지 않는 녹음은 이 메시지 대신 `No audio detected from microphone`을 보고합니다. v2.1.200 이전에는 침묵한 마이크가 연결 실패를 보고할 수 있었으며, 이는 실제 문제가 입력 장치일 때 네트워크 문제를 시사했습니다.
* **`No speech detected`**: 오디오가 전사 서비스에 도달했지만 단어가 인식되지 않았습니다. 마이크에 더 가깝게 말하고, 배경 소음을 줄이고, [받아쓰기 언어](#change-the-dictation-language)가 말하는 언어와 일치하는지 확인하세요.
* **전사가 왜곡되었거나 잘못된 언어임**: 받아쓰기는 기본적으로 영어입니다. 다른 언어로 받아쓰기하는 경우 먼저 `/config`에서 설정하세요. [받아쓰기 언어 변경](#change-the-dictation-language)을 참조하세요.

<h3 id="terminal-not-listed-in-macos-microphone-settings">
  macOS 마이크 설정에 나열되지 않은 터미널
</h3>

터미널 앱이 시스템 설정 → 개인정보 보호 및 보안 → 마이크 아래에 나타나지 않으면 활성화할 수 있는 토글이 없습니다. 터미널의 권한 상태를 재설정하여 다음 `/voice` 실행이 새로운 macOS 권한 프롬프트를 트리거하도록 합니다.

<Steps>
  <Step title="터미널의 마이크 권한 재설정">
    `tccutil reset Microphone <bundle-id>`를 실행하고, `<bundle-id>`를 터미널의 식별자로 바꾸세요: 기본 제공 터미널의 경우 `com.apple.Terminal`, iTerm2의 경우 `com.googlecode.iterm2`. 다른 터미널의 경우 `osascript -e 'id of app "AppName"'`으로 식별자를 조회하세요.

    <Warning>
      번들 ID 없이 `tccutil reset Microphone`을 실행할 수 있지만 Mac의 모든 앱(Zoom 또는 Slack과 같은 앱 포함)에서 마이크 접근을 취소합니다. 각 앱은 다음 사용 시 접근을 다시 요청해야 하므로 활성 통화 중에 실행하지 마세요.
    </Warning>
  </Step>

  <Step title="터미널 종료 및 다시 시작">
    macOS는 이미 실행 중인 프로세스를 다시 프롬프트하지 않습니다. 창을 닫기만 하지 말고 Cmd+Q로 터미널 앱을 종료한 다음 다시 열어보세요.
  </Step>

  <Step title="새로운 프롬프트 트리거">
    Claude Code를 시작하고 `/voice`를 실행합니다. macOS가 마이크 접근을 요청합니다. 허용하세요.
  </Step>
</Steps>

<h2 id="see-also">
  참고 항목
</h2>

* [키보드 단축키 사용자 정의](/docs/ko/keybindings): `voice:pushToTalk` 및 기타 CLI 키보드 작업 다시 바인딩
* [설정 구성](/docs/ko/settings): `voice`, `language` 및 기타 설정 키에 대한 전체 참조
* [대화형 모드](/docs/ko/interactive-mode): 키보드 단축키, 입력 모드 및 세션 제어
* [명령](/docs/ko/commands): `/voice`, `/config` 및 기타 모든 명령에 대한 참조
