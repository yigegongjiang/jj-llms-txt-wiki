> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# iOS 시뮬레이터에서 앱 테스트하기

> Claude Code Desktop은 Claude가 앱을 빌드, 실행 또는 확인할 때 iOS 시뮬레이터 창에서 앱을 열며, 각 세션마다 별도의 시뮬레이터를 제공합니다.

<Note>
  iOS 시뮬레이터 창은 macOS의 Claude Code Desktop에서 공개 베타 상태입니다. Pro, Max, Team 및 Enterprise 플랜에서 사용 가능하며, HIPAA 구성이 활성화된 Enterprise 조직은 제외됩니다.
</Note>

iOS 시뮬레이터 창은 Claude Code Desktop의 대화 옆에 Apple의 iOS 시뮬레이터에서 실행 중인 앱을 표시합니다. Claude가 시뮬레이터에서 앱을 빌드, 설치, 실행 또는 확인할 때 창이 자동으로 열리고 기기 화면을 실시간으로 스트리밍합니다. Claude가 앱을 실행하고 테스트하는 것을 보거나, Claude가 계속 작업하는 동안 직접 앱을 탭하며 이동할 수 있습니다.

시뮬레이터 창은 시뮬레이터를 직접 제어하므로 [컴퓨터 사용](/docs/ko/desktop#let-claude-use-your-computer)이 필요하지 않으며 화면을 차지하거나 다른 창을 숨기지 않습니다. CLI에서 Claude는 [컴퓨터 사용](/docs/ko/computer-use#test-a-simulator-flow)을 통해 iOS 시뮬레이터에 접근하며, 이는 마우스로 하는 것처럼 화면의 시뮬레이터를 제어합니다.

<h2 id="requirements">
  요구 사항
</h2>

시뮬레이터 창은 Apple의 시뮬레이터 도구를 사용하며, 데스크톱 앱에는 포함되지 않습니다. 세션을 시작하기 전에 다음을 확인하세요:

* Claude Desktop v1.24012.0 이상
* Mac (Apple의 iOS 시뮬레이터는 macOS에서만 실행됨)
* iOS 플랫폼이 설치된 [Xcode](https://developer.apple.com/xcode/) (시뮬레이터 기기 제공). Xcode에 시뮬레이터가 아직 없으면 [시뮬레이터 창에 시뮬레이터를 찾을 수 없다고 표시됨](#the-simulator-pane-says-no-simulators-were-found)을 참조하세요.
  * Xcode 26.x를 사용하세요. 창은 아직 Xcode 27과 호환되지 않으며, Xcode 27은 시뮬레이터 앱을 Device Hub로 대체합니다. Mac의 `xcode-select`가 Xcode 27을 가리키면 [시뮬레이터 창이 Xcode 27에서 실패함](#the-simulator-pane-fails-with-xcode-27)을 참조하세요.

<Note>
  이 페이지에서 "기기"는 시뮬레이션된 iPhone 또는 iPad를 의미하며, Xcode의 **Window → Devices and Simulators** 아래에서 관리하는 것과 동일한 시뮬레이터 기기이며, 물리적 하드웨어가 아닙니다.
</Note>

시뮬레이터 창은 로컬 세션에서만 사용 가능합니다. [클라우드](/docs/ko/desktop#run-long-running-tasks-in-the-cloud) 및 [SSH](/docs/ko/desktop#ssh-sessions) 세션에서 Claude는 Mac의 시뮬레이터에 접근할 수 없는 머신에서 실행됩니다.

<h2 id="run-your-app-in-the-simulator">
  시뮬레이터에서 앱 실행하기
</h2>

시뮬레이터 창을 열기 위해 명령이나 설정이 필요하지 않습니다. Claude가 시뮬레이터에서 앱을 실행할 때 창을 엽니다.

<Steps>
  <Step title="iOS 프로젝트 열기">
    Claude Code Desktop에서 **Code** 탭을 열고 앱의 프로젝트를 [프로젝트 폴더](/docs/ko/desktop#start-a-session)로 하여 세션을 시작합니다. iOS 시뮬레이터용 앱을 빌드하는 모든 프로젝트가 작동합니다.
  </Step>

  <Step title="Claude에게 앱을 실행하거나 테스트하도록 요청">
    작업을 앱 실행 또는 검증 중심으로 표현합니다. 예를 들어:

    ```text theme={null}
    Build the app and run it in the simulator to check the onboarding flow.
    ```
  </Step>

  <Step title="시뮬레이터 창에서 앱 보기">
    앱이 시뮬레이터에서 실행되면 iOS 시뮬레이터 창이 대화 옆에 열립니다. Claude가 기기를 처음 사용할 때 데스크톱 앱이 허용을 요청합니다. [Claude에게 기기 접근 권한 부여](#grant-claude-access-to-a-device)를 참조하세요. Claude가 앱을 설치하고, 탭하며 이동하고, 화면을 읽어 자신의 변경 사항을 검증하는 동안 시청할 수 있습니다.
  </Step>
</Steps>

시뮬레이터 창은 Claude가 세션의 어느 시점에서든 시뮬레이터에서 앱을 실행할 때마다 열립니다. 요청이 앱 보기에 관한 것일 때 (예: "새 화면이 맞나요?"), Claude는 작업을 시작하기 전에 시뮬레이터를 시작합니다. Claude가 버그를 수정하거나 화면을 변경한 후 변경 사항을 검증하도록 요청합니다. 앱을 다시 실행하면 창이 열려 있지 않으면 창을 다시 엽니다.

시뮬레이터 창은 앱이 실제로 실행된 기기를 표시합니다. 특정 기기에서 테스트하려면 요청에서 기기 이름을 지정합니다 (예: "iPhone SE 시뮬레이터에서 실행"). Claude는 빌드 및 실행 시 해당 기기를 대상으로 합니다.

Claude가 부팅한 기기는 Apple의 시뮬레이터 앱에도 나타나며, Claude는 이미 부팅된 기기에 앱을 설치할 수 있습니다.

시뮬레이터 창을 직접 열 수도 있습니다. 세션에 시뮬레이터가 연결되거나 Swift 파일을 편집한 후 세션 도구 모음의 **Views** 메뉴에 **iOS Simulator** 항목이 표시됩니다. 창이 아직 기기를 표시하지 않으면 **Attach simulator**를 클릭하거나 옆의 기기 메뉴에서 특정 기기를 선택합니다. 종료된 기기를 선택하면 부팅됩니다. Xcode 또는 시뮬레이터가 없으면 창에 설정 단계가 표시되고 완료하면 체크 표시됩니다.

<h2 id="control-the-simulator-yourself">
  시뮬레이터 직접 제어하기
</h2>

시뮬레이터 창은 뷰어일 뿐만 아니라 대화형입니다. Claude가 작업하는 동안 또는 작업 사이에 다음을 수행할 수 있습니다:

* 기기 화면을 클릭하고 드래그하여 탭 및 스와이프
* Apple의 시뮬레이터 앱과 동일한 단축키로 하드웨어 버튼 누르기: Home은 **Cmd+Shift+H**, 잠금은 **Cmd+L**, 볼륨은 **Cmd+Up Arrow** 및 **Cmd+Down Arrow**
* 회전 버튼 또는 **Cmd+Right Arrow**로 기기를 시계 방향으로 90도 회전
* 기기 메뉴에서 창이 표시하는 기기 전환 (각 시뮬레이터의 OS 버전 및 부팅 여부 표시)
* **Cmd+S**로 스크린샷 또는 **Cmd+R**로 화면 녹화 저장 (창의 캡처 버튼 또는 단축키 사용, 파일은 Desktop에 저장됨)
* **Detach simulator**를 클릭하여 기기 스트리밍 중지 (종료하지 않음), 창을 **Attach simulator** 상태로 반환

기기 이름 아래의 행은 시뮬레이터의 비디오 스트림을 조정합니다. Mac에 부담이 되면 **Frame rate** 또는 **Resolution**을 낮추고, **Encoding**을 H.264와 JPEG 사이에서 전환하거나, **FPS**를 확인하여 창이 받는 프레임 레이트를 표시합니다. 이 설정은 창이 기기를 표시하는 방식을 변경하며, 앱 실행 방식은 변경하지 않습니다.

사용자와 Claude가 동일한 기기를 제어하므로 탭이 Claude가 보는 앱 상태를 변경합니다. Claude가 특정 화면을 확인하도록 하려면 탭하여 이동한 후 요청합니다. Claude가 기기를 제어하는 동안 창은 화면 위에 **Claude is using this device** 배지를 표시합니다. 배지가 사라질 때까지 탭을 기다려 결과가 입력이 아닌 앱을 반영하도록 합니다.

<h2 id="how-sessions-manage-devices">
  세션이 기기를 관리하는 방식
</h2>

각 기기는 이를 실행한 세션에 속하므로 [병렬 세션](/docs/ko/desktop#work-in-parallel-with-sessions)은 기기를 공유하지 않습니다. 한 세션의 창에 표시되는 내용은 해당 세션의 작업을 반영하며 다른 세션의 작업은 반영하지 않습니다. 사이드바에서 세션을 전환하면 시뮬레이터 보기가 대화와 함께 전환되고, 다시 전환하면 동일한 기기가 중단된 위치에서 재개됩니다. Claude가 둘 이상의 기기로 작업하면 각각 자체 창을 열며, 세션당 최대 4개까지 가능합니다.

Claude Code Desktop은 더 이상 사용하지 않을 때 부팅한 시뮬레이터를 종료합니다. 앱을 종료할 때, 세션을 보관할 때, 또는 기기를 창에서 분리한 후 10분 후입니다. 사용자가 직접 부팅한 기기 (창에서 또는 Apple의 시뮬레이터 앱에서)는 자동으로 종료되지 않습니다. 연결된 기기를 즉시 종료하려면 창의 종료 버튼을 사용합니다.

<h2 id="grant-claude-access-to-a-device">
  Claude에게 기기 접근 권한 부여
</h2>

Claude는 기기를 제어하기 전에 동의를 요청하며, 앱 빌드 또는 URL 열기는 세션의 권한 모드를 따릅니다. 사용자 또는 조직이 Claude의 접근을 완전히 비활성화할 수도 있습니다.

<h3 id="allow-a-device-the-first-time">
  처음으로 기기 허용
</h3>

Claude가 시뮬레이터를 처음 사용할 때 데스크톱 앱이 허용을 요청합니다. 동의는 해당 기기 제어 및 스크린샷 촬영을 포함하며, 세션당 한 번이 아닌 기기당 한 번 제공합니다. Claude의 기기 스크린샷은 Anthropic으로 전송되며 일반적인 대화 보관 설정에 따라 유지되므로 Claude가 사용하는 기기에 실제 계정으로 로그인하지 마세요.

기기를 허용한 후 Claude의 탭, 입력, 앱 실행 및 스크린샷 촬영과 같은 작업은 추가 프롬프트 없이 실행됩니다. 이들은 창을 클릭하는 것과 동일한 신뢰를 가지며, 시뮬레이션된 기기만 터치하므로 창은 컴퓨터 사용이 필요로 하는 macOS 접근성 및 화면 녹화 권한이 필요하지 않습니다.

거부하면 기기는 여전히 부팅되고 창은 여전히 자신의 탭에 작동합니다. Claude의 접근만 비활성화됩니다. 나중에 마음을 바꾸면 창에서 **Let Claude use it**을 클릭합니다.

<h3 id="actions-that-follow-your-permission-mode">
  권한 모드를 따르는 작업
</h3>

두 가지 작업은 일회성 동의 대신 세션의 [권한 모드](/docs/ko/permissions#permission-modes)를 따릅니다:

* 기기에서 URL 열기 (예: 딥 링크 테스트 또는 기기의 Safari에서 페이지 로드), URL이 기기에서 데이터를 전달할 수 있기 때문입니다.
* 앱 빌드 (`xcodebuild`가 Mac에서 프로젝트의 빌드 스크립트를 실행하기 때문). 이미 진행 중인 빌드를 확인하면 프롬프트가 표시되지 않습니다.

<h3 id="turn-off-simulator-access">
  시뮬레이터 접근 비활성화
</h3>

데스크톱 앱의 설정에서 Claude의 시뮬레이터 접근을 비활성화할 수 있습니다. 조직은 모두를 위해 비활성화하는 두 가지 방법이 있습니다:

* `disableMobileSimulatorTools` [관리 설정](/docs/ko/desktop#managed-settings)은 Claude의 시뮬레이터 도구를 차단합니다. 시뮬레이터 창은 여전히 자신의 탭에 사용 가능하며, 설정은 앱 내에서 재정의될 수 없습니다.
* `requireCoworkFullVmSandbox` 정책 키는 Claude의 도구를 Mac 대신 격리된 가상 머신 내에서 실행하며, 시뮬레이터 창 및 Claude의 시뮬레이터 도구를 완전히 비활성화하므로 설정되면 창이 기기를 연결할 수 없습니다.

Claude는 어느 쪽이든 적용될 때 알려줍니다.

<h2 id="limitations">
  제한 사항
</h2>

Claude는 시뮬레이션된 기기만 제어할 수 있으며 물리적 iPhone 또는 iPad를 제어할 수 없습니다. 물리적 기기에서 테스트하려면 Xcode에서 직접 앱을 실행한 후 보이는 내용을 설명하거나 스크린샷을 대화에 첨부하여 Claude가 작업할 수 있도록 합니다.

<h2 id="troubleshooting">
  문제 해결
</h2>

<h3 id="the-simulator-pane-doesn’t-open-when-claude-runs-the-app">
  Claude가 앱을 실행할 때 시뮬레이터 창이 열리지 않음
</h3>

Claude가 앱을 실행하거나 테스트하려고 한다는 것을 인식하지 못했거나 시뮬레이터 도구가 누락되었을 수 있습니다. 다음을 확인하세요:

* 목표를 명시적으로 표현합니다 (예: "iOS 시뮬레이터에서 앱을 실행하고 가입 흐름을 탭하며 이동").
* Xcode 및 iOS 시뮬레이터가 설치되어 있고 Xcode 버전이 [요구 사항](#requirements)을 충족하는지 확인합니다.
* 조직이 Claude Code를 관리하면 [시뮬레이터 도구가 정책에 의해 비활성화](#turn-off-simulator-access)될 수 있습니다.
* HIPAA 구성이 활성화된 Enterprise 조직에 속하면 시뮬레이터 창을 사용할 수 없습니다.
* 시뮬레이터 창에는 Claude Desktop v1.24012.0 이상이 필요합니다. **Claude → Check for Updates**를 열고 앱을 다시 시작합니다.

<h3 id="the-simulator-pane-says-no-simulators-were-found">
  시뮬레이터 창에 시뮬레이터를 찾을 수 없다고 표시됨
</h3>

`xcode-select`가 Xcode 27을 가리키면 기기가 존재하더라도 창이 시뮬레이터를 찾을 수 없다고 보고할 수 있습니다. [시뮬레이터 창이 Xcode 27에서 실패함](#the-simulator-pane-fails-with-xcode-27)을 참조하세요. 그렇지 않으면 Xcode가 설치되어 있지만 나열할 iOS 시뮬레이터가 없습니다. 시뮬레이터 창은 따를 설정 단계를 표시하고 각 단계가 완료되면 체크 표시합니다. 누락된 부분을 수동으로 설치하려면 Xcode의 설정에서 iOS 시뮬레이터 런타임을 다운로드하거나 `xcodebuild -downloadPlatform iOS`를 실행합니다.

<h3 id="the-simulator-pane-fails-with-xcode-27">
  시뮬레이터 창이 Xcode 27에서 실패함
</h3>

창은 아직 Xcode 27과 호환되지 않으며, Xcode 27은 시뮬레이터 앱을 Device Hub로 대체합니다. Xcode 27이 선택되면 기기 연결이 실패하거나 기기가 존재하더라도 창이 시뮬레이터를 찾을 수 없다고 보고합니다.

창은 `xcode-select`가 가리키는 Xcode를 사용합니다. Xcode 27이 유일한 설치이면 먼저 Xcode 26.x를 나란히 설치합니다. 그런 다음 경로로 26.x 설치를 선택합니다. 예를 들어 `/Applications/Xcode-26.4.app`으로 설치된 경우:

```bash theme={null}
sudo xcode-select -s /Applications/Xcode-26.4.app
```

`xcode-select -p`를 실행하여 선택된 설치를 확인합니다.

<h2 id="see-also">
  참고 항목
</h2>

* [Desktop의 컴퓨터 사용](/docs/ko/desktop#let-claude-use-your-computer): 전용 창이 없는 앱용 화면 제어
* [CLI의 컴퓨터 사용](/docs/ko/computer-use): CLI가 iOS 시뮬레이터에 접근하는 방식
* [세션과 병렬로 작업](/docs/ko/desktop#work-in-parallel-with-sessions): 세션이 변경 사항을 격리하는 방식
* [Claude Code Desktop 시작하기](/docs/ko/desktop-quickstart)
