> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude가 CLI에서 컴퓨터를 사용하도록 설정

> Claude Code CLI에서 컴퓨터 사용을 활성화하여 Claude가 macOS에서 앱을 열고, 클릭하고, 입력하고, 화면을 볼 수 있도록 합니다. 터미널을 떠나지 않고 네이티브 앱을 테스트하고, 시각적 문제를 디버깅하고, GUI 전용 도구를 자동화합니다.

<Note>
  컴퓨터 사용은 Pro 또는 Max 플랜이 필요한 macOS의 연구 미리보기입니다. Team 또는 Enterprise 플랜에서는 사용할 수 없습니다. 대화형 세션이 필요하므로 `-p` 플래그를 사용한 비대화형 모드에서는 사용할 수 없습니다.
</Note>

컴퓨터 사용을 통해 Claude는 앱을 열고, 화면을 제어하고, 사용자가 하는 방식으로 컴퓨터에서 작업할 수 있습니다. CLI에서 Claude는 Swift 앱을 컴파일하고, 실행하고, 모든 버튼을 클릭하고, 결과를 스크린샷하는 모든 작업을 코드를 작성한 동일한 대화에서 수행할 수 있습니다.

이 페이지에서는 CLI에서 컴퓨터 사용이 어떻게 작동하는지 설명합니다. Desktop 앱의 경우 [Desktop의 컴퓨터 사용](/docs/ko/desktop#let-claude-use-your-computer)을 참조하세요.

<h2 id="what-you-can-do-with-computer-use">
  컴퓨터 사용으로 할 수 있는 작업
</h2>

컴퓨터 사용은 GUI가 필요한 작업을 처리합니다. 일반적으로 터미널을 떠나 손으로 직접 해야 하는 모든 작업입니다.

* **네이티브 앱 빌드 및 검증**: Claude에게 macOS 메뉴 바 앱을 빌드하도록 요청합니다. Claude는 Swift를 작성하고, 컴파일하고, 실행하고, 모든 컨트롤을 클릭하여 사용자가 열기 전에 작동하는지 확인합니다.
* **엔드투엔드 UI 테스트**: Claude에게 로컬 Electron 앱을 가리키고 "온보딩 흐름을 테스트해"라고 말합니다. Claude는 앱을 열고, 가입 과정을 클릭하고, 각 단계를 스크린샷합니다. Playwright 설정이나 테스트 하네스가 필요 없습니다.
* **시각적 및 레이아웃 문제 디버깅**: Claude에게 "모달이 작은 창에서 잘려 있어"라고 말합니다. Claude는 창 크기를 조정하고, 버그를 재현하고, 스크린샷을 찍고, CSS를 패치하고, 수정 사항을 확인합니다. Claude는 사용자가 보는 것을 봅니다.
* **GUI 전용 도구 제어**: 디자인 도구, 하드웨어 제어판, iOS Simulator 또는 CLI나 API가 없는 독점 앱과 상호작용합니다.

<h2 id="when-computer-use-applies">
  컴퓨터 사용이 적용되는 경우
</h2>

Claude는 앱이나 서비스와 상호작용하는 여러 방법을 가지고 있습니다. 컴퓨터 사용은 가장 광범위하고 가장 느리므로 Claude는 가장 정확한 도구를 먼저 시도합니다.

* [MCP 서버](/docs/ko/mcp)가 서비스에 있으면 Claude가 그것을 사용합니다.
* 작업이 셸 명령이면 Claude는 Bash를 사용합니다.
* 작업이 브라우저 작업이고 [Claude in Chrome](/docs/ko/chrome)이 설정되어 있으면 Claude가 그것을 사용합니다.
* 위의 어느 것도 적용되지 않으면 Claude는 컴퓨터 사용을 사용합니다.

화면 제어는 다른 것이 도달할 수 없는 것들을 위해 예약되어 있습니다. 네이티브 앱, 시뮬레이터, API가 없는 도구입니다.

<h2 id="enable-computer-use">
  컴퓨터 사용 활성화
</h2>

컴퓨터 사용은 `computer-use`라는 기본 제공 MCP 서버로 사용할 수 있습니다. 활성화할 때까지 기본적으로 꺼져 있습니다.

<Steps>
  <Step title="MCP 메뉴 열기">
    대화형 Claude Code 세션에서 다음을 실행합니다.

    ```text theme={null}
    /mcp
    ```

    서버 목록에서 `computer-use`를 찾습니다. 비활성화된 것으로 표시됩니다.
  </Step>

  <Step title="서버 활성화">
    `computer-use`를 선택하고 **활성화**를 선택합니다. 설정은 프로젝트별로 유지되므로 컴퓨터 사용을 원하는 각 프로젝트에 대해 한 번만 수행하면 됩니다.
  </Step>

  <Step title="macOS 권한 부여">
    Claude가 처음으로 컴퓨터를 사용하려고 할 때 두 가지 macOS 권한을 부여하라는 프롬프트가 표시됩니다.

    * **접근성**: Claude가 클릭, 입력 및 스크롤할 수 있도록 합니다.
    * **화면 녹화**: Claude가 화면에 있는 것을 볼 수 있도록 합니다.

    프롬프트에는 관련 시스템 설정 창을 열 수 있는 링크가 포함되어 있습니다. 둘 다 부여한 다음 프롬프트에서 **다시 시도**를 선택합니다. macOS는 화면 녹화를 부여한 후 Claude Code를 다시 시작하도록 요구할 수 있습니다.
  </Step>
</Steps>

설정 후 Claude에게 GUI가 필요한 작업을 수행하도록 요청합니다.

```text theme={null}
앱 대상을 빌드하고, 실행하고, 각 탭을 클릭하여 아무것도
충돌하지 않는지 확인합니다. 발견한 오류 상태를 스크린샷합니다.
```

<h2 id="approve-apps-per-session">
  세션별 앱 승인
</h2>

`computer-use` 서버를 활성화해도 Claude에게 컴퓨터의 모든 앱에 대한 액세스 권한이 부여되지는 않습니다. Claude가 세션에서 특정 앱이 필요한 첫 번째 시간에 터미널에 프롬프트가 나타나며 다음을 표시합니다.

* Claude가 제어하려는 앱
* 클립보드 액세스와 같은 요청된 추가 권한
* Claude가 작업하는 동안 숨겨질 다른 앱의 수

**이 세션에 대해 허용** 또는 **거부**를 선택합니다. 승인은 현재 세션 동안 지속됩니다. Claude가 함께 요청할 때 여러 앱을 한 번에 승인할 수 있습니다.

광범위한 도달 범위를 가진 앱은 프롬프트에 추가 경고를 표시하므로 승인이 부여하는 것을 알 수 있습니다.

| 경고                | 적용 대상                                         |
| :---------------- | :-------------------------------------------- |
| 셸 액세스와 동등         | Terminal, iTerm, VS Code, Warp 및 기타 터미널 및 IDE |
| 모든 파일을 읽거나 쓸 수 있음 | Finder                                        |
| 시스템 설정을 변경할 수 있음  | 시스템 설정                                        |

이러한 앱은 차단되지 않습니다. 경고를 통해 작업이 해당 수준의 액세스를 보장하는지 결정할 수 있습니다.

Claude의 제어 수준은 앱 카테고리에 따라 다릅니다. 브라우저와 거래 플랫폼은 보기 전용이고, 터미널과 IDE는 클릭 전용이며, 다른 모든 것은 전체 제어를 얻습니다. 전체 계층 분석은 [Desktop의 앱 권한](/docs/ko/desktop#app-permissions)을 참조하세요.

<h2 id="how-claude-works-on-your-screen">
  Claude가 화면에서 작동하는 방식
</h2>

흐름을 이해하면 Claude가 무엇을 할 것인지 예상하고 개입하는 방법을 알 수 있습니다.

<h3 id="one-session-at-a-time">
  한 번에 한 세션
</h3>

컴퓨터 사용은 첫 번째 컴퓨터 사용 작업부터 그 작업을 수행한 세션이 종료될 때까지 머신 전체 잠금을 유지합니다. v2.1.195부터는 작업을 완료해도 잠금이 해제되지 않으며, 세션을 종료할 때만 해제됩니다. 다른 Claude Code 세션이 이미 컴퓨터를 사용 중이면 새로운 시도는 어느 세션이 잠금을 유지하는지 알려주는 메시지와 함께 실패합니다. 먼저 해당 세션을 종료합니다.

<h3 id="apps-are-hidden-while-claude-works">
  Claude가 작업하는 동안 앱이 숨겨집니다.
</h3>

Claude가 화면 제어를 시작하면 다른 표시되는 앱이 숨겨져 Claude가 승인된 앱하고만 상호작용합니다. 터미널 창은 표시된 상태로 유지되고 스크린샷에서 제외되므로 세션을 볼 수 있고 Claude는 자신의 출력을 절대 보지 않습니다.

Claude가 턴을 완료하면 숨겨진 앱이 자동으로 복원됩니다.

<h3 id="screenshots-are-downscaled-automatically">
  스크린샷이 자동으로 축소됩니다.
</h3>

Claude Code는 모델로 전송하기 전에 모든 스크린샷을 축소합니다. Retina 또는 기타 고해상도 디스플레이에서 디스플레이 해상도를 낮추거나 창 크기를 조정할 필요가 없습니다. 기본 Retina 해상도의 16인치 MacBook Pro는 3456×2234에서 캡처하고 대략 1372×887로 축소하여 종횡비를 유지합니다.

대상 크기를 변경할 수 있는 설정이 없습니다. 축소 후 화면의 텍스트나 컨트롤이 Claude가 읽기에 너무 작으면 디스플레이 해상도를 변경하는 대신 앱에서 크기를 늘립니다.

<h3 id="stop-at-any-time">
  언제든지 중지
</h3>

Claude가 잠금을 획득하면 macOS 알림이 나타납니다. "Claude가 컴퓨터를 사용 중입니다 · Esc를 눌러 중지하세요." 어디서나 `Esc`를 눌러 현재 작업을 즉시 중단하거나 터미널에서 `Ctrl+C`를 누릅니다. 어느 쪽이든 Claude는 중지하고, 앱을 표시하고, 제어를 사용자에게 반환합니다. 세션은 종료될 때까지 [컴퓨터 사용 잠금](#one-session-at-a-time)을 유지합니다.

Claude가 완료되면 두 번째 알림이 나타납니다.

<h2 id="safety-and-the-trust-boundary">
  안전 및 신뢰 경계
</h2>

<Warning>
  [샌드박스된 Bash 도구](/docs/ko/sandboxing)와 달리 컴퓨터 사용은 승인한 앱에 액세스할 수 있는 실제 데스크톱에서 실행됩니다. Claude는 각 작업을 확인하고 화면 콘텐츠에서 잠재적 프롬프트 주입을 플래그하지만 신뢰 경계는 다릅니다. 모범 사례는 [컴퓨터 사용 안전 가이드](https://support.claude.com/en/articles/14128542)를 참조하세요.
</Warning>

기본 제공 가드레일은 구성 없이 위험을 줄입니다.

* **앱별 승인**: Claude는 현재 세션에서 승인한 앱만 제어할 수 있습니다.
* **센티널 경고**: 셸, 파일 시스템 또는 시스템 설정 액세스를 부여하는 앱은 승인 전에 플래그됩니다.
* **스크린샷에서 터미널 제외**: Claude는 터미널 창을 절대 보지 않으므로 세션의 화면 프롬프트가 모델로 다시 피드될 수 없습니다.
* **전역 이스케이프**: `Esc` 키는 어디서나 컴퓨터 사용을 중단하고 키 누름이 소비되므로 프롬프트 주입이 대화 상자를 닫는 데 사용할 수 없습니다.
* **잠금 파일**: 한 번에 한 세션만 컴퓨터를 제어할 수 있습니다.

<h2 id="example-workflows">
  예제 워크플로우
</h2>

이 예제는 컴퓨터 사용을 코딩 작업과 결합하는 일반적인 방법을 보여줍니다.

<h3 id="validate-a-native-build">
  네이티브 빌드 검증
</h3>

macOS 또는 iOS 앱을 변경한 후 Claude에게 한 번에 컴파일하고 확인하도록 합니다:

```text theme={null}
MenuBarStats 대상을 빌드하고, 실행하고, 기본 설정 창을 열고,
간격 슬라이더가 레이블을 업데이트하는지 확인합니다. 완료되면
기본 설정 창을 스크린샷합니다.
```

Claude는 `xcodebuild`를 실행하고, 앱을 실행하고, UI와 상호작용하고, 발견한 내용을 보고합니다.

<h3 id="reproduce-a-layout-bug">
  레이아웃 버그 재현
</h3>

시각적 버그가 특정 창 크기에서만 나타날 때 Claude에게 찾도록 합니다:

```text theme={null}
설정 모달이 좁은 창에서 바닥글을 자릅니다. 앱 창 크기를 줄여서
재현할 수 있을 때까지 조정하고, 잘린 상태를 스크린샷한 다음,
모달 컨테이너의 CSS를 확인합니다.
```

Claude는 창 크기를 조정하고, 손상된 상태를 캡처하고, 관련 스타일시트를 읽습니다.

<h3 id="test-a-simulator-flow">
  시뮬레이터 흐름 테스트
</h3>

XCTest를 작성하지 않고 iOS Simulator를 제어합니다:

```text theme={null}
iOS Simulator를 열고, 앱을 실행하고, 온보딩 화면을 탭하고,
어떤 화면이 로드하는 데 1초 이상 걸리는지 알려줍니다.
```

Claude는 마우스를 사용하는 것처럼 시뮬레이터를 제어합니다.

<h2 id="differences-from-the-desktop-app">
  Desktop 앱과의 차이점
</h2>

CLI 및 Desktop 표면은 동일한 컴퓨터 사용 엔진을 공유하며, 몇 가지 차이점이 있습니다.

| 기능          | Desktop                             | CLI                         |
| :---------- | :---------------------------------- | :-------------------------- |
| 플랫폼         | macOS 및 Windows                     | macOS만 해당                   |
| 활성화         | **설정 > 일반**의 토글 (**Desktop 앱** 아래)  | `/mcp`에서 `computer-use` 활성화 |
| 거부된 앱 목록    | 설정에서 구성 가능                          | 아직 사용할 수 없음                 |
| 자동 표시 토글    | 선택 사항                               | 항상 켜짐                       |
| Dispatch 통합 | Dispatch에서 생성된 세션이 컴퓨터 사용을 사용할 수 있음 | 해당 없음                       |

<h2 id="troubleshooting">
  문제 해결
</h2>

<h3 id="computer-use-is-in-use-by-another-claude-session">
  "컴퓨터 사용이 다른 Claude 세션에서 사용 중입니다"
</h3>

다른 Claude Code 세션이 잠금을 유지합니다. 해당 세션을 종료합니다. 다른 세션이 충돌한 경우 Claude가 프로세스가 더 이상 실행 중이 아님을 감지하면 잠금이 자동으로 해제됩니다.

<h3 id="macos-permissions-prompt-keeps-reappearing">
  macOS 권한 프롬프트가 계속 다시 나타남
</h3>

macOS는 화면 녹화를 부여한 후 요청 프로세스를 다시 시작해야 할 수 있습니다. Claude Code를 완전히 종료하고 새 세션을 시작합니다. 프롬프트가 계속되면 **시스템 설정 > 개인 정보 보호 및 보안 > 화면 녹화**를 열고 터미널 앱이 나열되고 활성화되어 있는지 확인합니다.

<h3 id="computer-use-doesn’t-appear-in-/mcp">
  `computer-use`가 `/mcp`에 나타나지 않음
</h3>

서버는 적격 설정에서만 나타납니다. 다음을 확인합니다.

* macOS를 사용 중입니다. 컴퓨터 사용은 Linux 또는 Windows에서 사용할 수 없습니다. Windows에서는 [Desktop의 컴퓨터 사용](/docs/ko/desktop#let-claude-use-your-computer)을 대신 사용합니다.
* Pro 또는 Max 플랜을 사용 중입니다. `/status`를 실행하여 구독을 확인합니다.
* claude.ai를 통해 인증되었습니다. 컴퓨터 사용은 Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry와 같은 타사 제공자에서 사용할 수 없습니다. 타사 제공자를 통해서만 Claude에 액세스하는 경우 이 기능을 사용하려면 별도의 claude.ai 계정이 필요합니다.
* 대화형 세션에 있습니다. 컴퓨터 사용은 `-p` 플래그를 사용한 비대화형 모드에서 사용할 수 없습니다.

<h2 id="see-also">
  참고 항목
</h2>

* [Desktop의 컴퓨터 사용](/docs/ko/desktop#let-claude-use-your-computer): 그래픽 설정 페이지가 있는 동일한 기능
* [Claude in Chrome](/docs/ko/chrome): 웹 기반 작업을 위한 브라우저 자동화
* [MCP](/docs/ko/mcp): Claude를 구조화된 도구 및 API에 연결
* [샌드박싱](/docs/ko/sandboxing): Claude의 Bash 도구가 파일 시스템 및 네트워크 액세스를 격리하는 방법
* [컴퓨터 사용 안전 가이드](https://support.claude.com/en/articles/14128542): 안전한 컴퓨터 사용을 위한 모범 사례
