> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Chrome에서 Claude Code 사용하기

> Claude Code를 Chrome 브라우저에 연결하여 웹 앱을 테스트하고, 콘솔 로그로 디버깅하며, 양식 작성을 자동화하고, 웹 페이지에서 데이터를 추출합니다.

Claude Code는 [Claude in Chrome 브라우저 확장 프로그램](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn)과 통합되어 CLI 또는 [VS Code 확장 프로그램](/docs/ko/vs-code#automate-browser-tasks-with-chrome)에서 브라우저 자동화 기능을 제공합니다. 코드를 작성한 후 컨텍스트를 전환하지 않고 브라우저에서 테스트하고 디버깅합니다.

Claude는 브라우저 작업을 위해 새 탭을 열고 브라우저의 로그인 상태를 공유하므로 이미 로그인한 모든 사이트에 액세스할 수 있습니다. 브라우저 작업은 실시간으로 표시되는 Chrome 창에서 실행됩니다. Claude가 로그인 페이지나 CAPTCHA를 만나면 일시 중지하고 수동으로 처리하도록 요청합니다.

<Note>
  Chrome 통합은 Google Chrome 및 Microsoft Edge에서 작동합니다. Brave, Arc 또는 기타 Chromium 기반 브라우저에서는 아직 지원되지 않습니다. Windows Subsystem for Linux(WSL)도 지원되지 않습니다.
</Note>

<h2 id="capabilities">
  기능
</h2>

Chrome이 연결되면 단일 워크플로우에서 브라우저 작업과 코딩 작업을 연결할 수 있습니다.

* **라이브 디버깅**: 콘솔 오류 및 DOM 상태를 직접 읽은 후 이를 유발한 코드를 수정합니다.
* **디자인 검증**: Figma 목업에서 UI를 빌드한 후 브라우저에서 열어 일치하는지 확인합니다.
* **웹 앱 테스트**: 양식 유효성 검사를 테스트하고, 시각적 회귀를 확인하거나, 사용자 흐름을 검증합니다.
* **인증된 웹 앱**: API 커넥터 없이 Google Docs, Gmail, Notion 또는 로그인한 모든 앱과 상호작용합니다.
* **데이터 추출**: 웹 페이지에서 구조화된 정보를 가져와 로컬에 저장합니다.
* **작업 자동화**: 데이터 입력, 양식 작성 또는 다중 사이트 워크플로우와 같은 반복적인 브라우저 작업을 자동화합니다.
* **세션 기록**: 브라우저 상호작용을 GIF로 기록하여 발생한 상황을 문서화하거나 공유합니다.

<h2 id="prerequisites">
  필수 요구사항
</h2>

Chrome에서 Claude Code를 사용하기 전에 다음이 필요합니다.

* [Google Chrome](https://www.google.com/chrome/) 또는 [Microsoft Edge](https://www.microsoft.com/edge) 브라우저
* [Claude in Chrome 확장 프로그램](https://chromewebstore.google.com/detail/claude/fcoeoabgfenejglbffodgkkbkcdhcgfn) 버전 1.0.36 이상(Chrome 웹 스토어에서 두 브라우저 모두에 사용 가능)
* [Claude Code](/docs/ko/quickstart#step-1-install-claude-code)
* 직접 Anthropic 플랜 (Pro, Max, Team 또는 Enterprise)

<Note>
  Chrome 통합은 Amazon Bedrock, Google Cloud의 Agent Platform 또는 Microsoft Foundry와 같은 타사 제공자를 통해 사용할 수 없습니다. 타사 제공자를 통해서만 Claude에 액세스하는 경우 이 기능을 사용하려면 별도의 claude.ai 계정이 필요합니다.
</Note>

<h2 id="get-started-in-the-cli">
  CLI에서 시작하기
</h2>

<Steps>
  <Step title="Chrome으로 Claude Code 시작">
    `--chrome` 플래그로 Claude Code를 시작합니다:

    ```bash theme={null}
    claude --chrome
    ```

    기존 세션 내에서 `/chrome`을 실행하여 Chrome을 활성화할 수도 있습니다.
  </Step>

  <Step title="Claude에게 브라우저 사용을 요청">
    이 예제는 페이지로 이동하고, 상호작용하며, 터미널이나 편집기에서 모두 발견한 내용을 보고합니다:

    ```text theme={null}
    code.claude.com/docs로 이동하여 검색 상자를 클릭하고,
    "hooks"를 입력한 후 나타나는 결과를 알려주세요
    ```

    첫 번째 브라우저 작업은 `claude-in-chrome` 스킬을 사용할 수 있는 권한을 요청합니다. 승인하면 Claude가 새 탭을 열고 작업을 시작합니다.
  </Step>
</Steps>

언제든지 `/chrome`을 실행하여 연결 상태를 확인하고, 권한을 관리하거나, 확장 프로그램을 다시 연결하거나, 사용할 연결된 브라우저를 선택할 수 있습니다. 브라우저 작업이 시작될 때 둘 이상의 브라우저가 연결되어 있으면 Claude가 하나를 선택하도록 요청합니다.

VS Code의 경우 [VS Code에서 브라우저 자동화](/docs/ko/vs-code#automate-browser-tasks-with-chrome)를 참조하세요.

<h3 id="enable-chrome-by-default">
  기본적으로 Chrome 활성화
</h3>

각 세션마다 `--chrome`을 전달하지 않으려면 `/chrome`을 실행하고 "기본적으로 활성화"를 선택합니다.

[VS Code 확장 프로그램](/docs/ko/vs-code#automate-browser-tasks-with-chrome)에서는 Chrome 확장 프로그램이 설치되어 있으면 Chrome을 사용할 수 있습니다. 추가 플래그가 필요하지 않습니다.

<Note>
  CLI에서 기본적으로 Chrome을 활성화하면 브라우저 도구가 항상 로드되므로 컨텍스트 사용량이 증가합니다. 컨텍스트 소비가 증가하는 것을 발견하면 이 설정을 비활성화하고 필요할 때만 `--chrome`을 사용합니다.
</Note>

<h3 id="manage-site-permissions">
  사이트 권한 관리
</h3>

사이트 수준 권한은 Chrome 확장 프로그램에서 상속됩니다. Chrome 확장 프로그램 설정에서 권한을 관리하여 Claude가 탐색하고, 클릭하고, 입력할 수 있는 사이트를 제어합니다.

<h3 id="browser-tools-in-plan-mode">
  계획 모드에서의 브라우저 도구
</h3>

[계획 모드](/docs/ko/permission-modes#analyze-before-you-edit-with-plan-mode)에서는 페이지나 브라우저 상태만 읽는 브라우저 도구 호출이 권한 프롬프트 없이 실행되고, 상태를 변경하는 호출은 승인을 요청합니다.

* **읽기 전용 호출**: `read_page`, `get_page_text`, `find`, 콘솔 메시지 또는 네트워크 요청 읽기, 스크린샷 촬영
* **상태 변경 호출**: 클릭, 입력, 탐색, 탭 및 창 관리, GIF 녹화

v2.1.199부터 `tabs_context_mcp`의 `createIfEmpty`, 콘솔 및 네트워크 리더의 `clear`, 스크린샷의 `save_to_disk`와 같이 상태 변경 입력 플래그를 설정하는 읽기 전용 호출도 승인을 요청합니다. `browser_batch` 호출은 내부의 모든 작업이 읽기 전용일 때만 프롬프트 없이 실행됩니다.

<h2 id="example-workflows">
  예제 워크플로우
</h2>

이 예제들은 브라우저 작업과 코딩 작업을 결합하는 일반적인 방법을 보여줍니다. `/mcp`를 실행하고 `claude-in-chrome`을 선택한 후 **도구 보기**를 선택하여 사용 가능한 브라우저 도구의 전체 목록을 확인합니다.

<h3 id="test-a-local-web-application">
  로컬 웹 애플리케이션 테스트
</h3>

웹 앱을 개발할 때 Claude에게 변경 사항이 올바르게 작동하는지 확인하도록 요청합니다.

```text theme={null}
방금 로그인 양식 유효성 검사를 업데이트했습니다. localhost:3000을 열고,
잘못된 데이터로 양식을 제출해 보고, 오류 메시지가 올바르게
나타나는지 확인해 주시겠어요?
```

Claude는 로컬 서버로 이동하여 양식과 상호작용하고 관찰한 내용을 보고합니다.

<h3 id="debug-with-console-logs">
  콘솔 로그로 디버깅
</h3>

Claude는 콘솔 출력을 읽어 문제 진단을 도울 수 있습니다. 로그가 상세할 수 있으므로 모든 콘솔 출력을 요청하는 대신 Claude에게 찾을 패턴을 알려줍니다.

```text theme={null}
대시보드 페이지를 열고 페이지가 로드될 때 콘솔에서 오류를
확인해 주세요.
```

Claude는 콘솔 메시지를 읽고 특정 패턴이나 오류 유형을 필터링할 수 있습니다.

<h3 id="automate-form-filling">
  양식 작성 자동화
</h3>

반복적인 데이터 입력 작업을 가속화합니다.

```text theme={null}
contacts.csv에 고객 연락처 스프레드시트가 있습니다. 각 행에 대해
crm.example.com의 CRM으로 이동하여 "연락처 추가"를 클릭하고
이름, 이메일 및 전화 필드를 작성해 주세요.
```

Claude는 로컬 파일을 읽고, 웹 인터페이스를 탐색하며, 각 레코드에 대한 데이터를 입력합니다.

<h3 id="draft-content-in-google-docs">
  Google Docs에서 콘텐츠 작성
</h3>

API 설정 없이 Claude를 사용하여 문서에 직접 작성합니다.

```text theme={null}
최근 커밋을 기반으로 프로젝트 업데이트를 작성하고
docs.google.com/document/d/abc123의 Google Doc에 추가해 주세요.
```

Claude는 문서를 열고, 편집기를 클릭한 후 콘텐츠를 입력합니다. 이는 로그인한 모든 웹 앱에서 작동합니다. Gmail, Notion, Sheets 등입니다.

<h3 id="extract-data-from-web-pages">
  웹 페이지에서 데이터 추출
</h3>

웹사이트에서 구조화된 정보를 가져옵니다.

```text theme={null}
제품 목록 페이지로 이동하여 각 항목의 이름, 가격 및 가용성을
추출합니다. 결과를 CSV 파일로 저장해 주세요.
```

Claude는 페이지로 이동하여 콘텐츠를 읽고 데이터를 구조화된 형식으로 컴파일합니다.

<h3 id="run-multi-site-workflows">
  다중 사이트 워크플로우 실행
</h3>

여러 웹사이트에서 작업을 조정합니다.

```text theme={null}
내 캘린더에서 내일의 회의를 확인한 후, 외부 참석자가 있는 각
회의에 대해 해당 회사 웹사이트를 찾아보고 그들이 하는 일에 대한
메모를 추가해 주세요.
```

Claude는 탭 전체에서 작업하여 정보를 수집하고 워크플로우를 완료합니다.

<h3 id="record-a-demo-gif">
  데모 GIF 기록
</h3>

브라우저 상호작용의 공유 가능한 기록을 만듭니다.

```text theme={null}
장바구니에 항목을 추가하는 것부터 확인 페이지까지 체크아웃
흐름을 완료하는 방법을 보여주는 GIF를 기록해 주세요.
```

Claude는 상호작용 시퀀스를 기록하고 GIF 파일로 저장합니다.

<h2 id="troubleshooting">
  문제 해결
</h2>

<h3 id="extension-not-detected">
  확장 프로그램이 감지되지 않음
</h3>

Claude Code에서 Chrome 확장 프로그램을 감지할 수 없는 경우:

1. Chrome 확장 프로그램이 설치되어 있고 `chrome://extensions`에서 활성화되어 있는지 확인합니다.
2. `claude --version`을 실행하여 Claude Code가 최신 버전인지 확인합니다.
3. Chrome이 실행 중인지 확인합니다.
4. `/chrome`을 실행하고 "확장 프로그램 다시 연결"을 선택하여 연결을 다시 설정합니다.
5. 문제가 지속되면 Claude Code와 Chrome을 모두 다시 시작합니다.

Chrome 통합을 처음 활성화할 때 Claude Code는 네이티브 메시징 호스트 구성 파일을 설치합니다. Chrome은 시작 시 이 파일을 읽으므로 첫 번째 시도에서 확장 프로그램이 감지되지 않으면 Chrome을 다시 시작하여 새 구성을 선택합니다.

v2.1.199부터 Claude Code는 첫 번째 설치 시에만 확장 프로그램을 연결하도록 요청하는 브라우저 탭을 엽니다. Claude Code 빌드 또는 구성 디렉터리 전환 후와 같이 구성 파일을 다시 작성하는 이후 세션에서는 다시 열지 않습니다.

연결이 계속 실패하면 다음 위치에 호스트 구성 파일이 있는지 확인합니다.

Chrome의 경우:

* **macOS**: `~/Library/Application Support/Google/Chrome/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Linux**: `~/.config/google-chrome/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Windows**: Windows 레지스트리에서 `HKCU\Software\Google\Chrome\NativeMessagingHosts\`를 확인합니다.

Edge의 경우:

* **macOS**: `~/Library/Application Support/Microsoft Edge/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Linux**: `~/.config/microsoft-edge/NativeMessagingHosts/com.anthropic.claude_code_browser_extension.json`
* **Windows**: Windows 레지스트리에서 `HKCU\Software\Microsoft\Edge\NativeMessagingHosts\`를 확인합니다.

<h3 id="browser-not-responding">
  브라우저가 응답하지 않음
</h3>

Claude의 브라우저 명령이 작동하지 않는 경우:

1. 모달 대화 상자(경고, 확인, 프롬프트)가 페이지를 차단하고 있는지 확인합니다. JavaScript 대화 상자는 브라우저 이벤트를 차단하고 Claude가 명령을 수신하지 못하게 합니다. 대화 상자를 수동으로 닫은 후 Claude에게 계속하도록 알립니다.
2. Claude에게 새 탭을 만들고 다시 시도하도록 요청합니다.
3. `chrome://extensions`에서 Chrome 확장 프로그램을 비활성화했다가 다시 활성화하여 다시 시작합니다.

<h3 id="connection-drops-during-long-sessions">
  긴 세션 중 연결 끊김
</h3>

Chrome 확장 프로그램의 서비스 워커는 확장 세션 중에 유휴 상태가 될 수 있으며, 이는 연결을 끊습니다. 비활성 기간 후 브라우저 도구가 작동하지 않으면 `/chrome`을 실행하고 "확장 프로그램 다시 연결"을 선택합니다.

<h3 id="windows-specific-issues">
  Windows 관련 문제
</h3>

Windows에서 다음을 만날 수 있습니다.

* **명명된 파이프 충돌 (EADDRINUSE)**: 다른 프로세스가 동일한 명명된 파이프를 사용 중인 경우 Claude Code를 다시 시작합니다. Chrome을 사용 중일 수 있는 다른 Claude Code 세션을 모두 닫습니다.
* **네이티브 메시징 호스트 오류**: 시작 시 네이티브 메시징 호스트가 충돌하면 Claude Code를 다시 설치하여 호스트 구성을 다시 생성해 봅니다.

<h3 id="common-error-messages">
  일반적인 오류 메시지
</h3>

가장 자주 발생하는 오류와 해결 방법은 다음과 같습니다.

| 오류                      | 원인                              | 해결 방법                                                   |
| ----------------------- | ------------------------------- | ------------------------------------------------------- |
| "브라우저 확장 프로그램이 연결되지 않음" | 네이티브 메시징 호스트가 확장 프로그램에 도달할 수 없음 | Chrome과 Claude Code를 다시 시작한 후 `/chrome`을 실행하여 다시 연결합니다. |
| "확장 프로그램이 감지되지 않음"      | Chrome 확장 프로그램이 설치되지 않았거나 비활성화됨 | `chrome://extensions`에서 확장 프로그램을 설치하거나 활성화합니다.          |
| "사용 가능한 탭 없음"           | Claude가 탭이 준비되기 전에 작동하려고 시도함    | Claude에게 새 탭을 만들고 다시 시도하도록 요청합니다.                       |
| "수신 끝이 존재하지 않음"         | 확장 프로그램 서비스 워커가 유휴 상태가 됨        | `/chrome`을 실행하고 "확장 프로그램 다시 연결"을 선택합니다.                 |

<h2 id="see-also">
  참고 항목
</h2>

* [컴퓨터 사용](/docs/ko/computer-use): 브라우저에서 작업을 수행할 수 없을 때 네이티브 macOS 앱을 제어합니다
* [VS Code에서 Claude Code 사용](/docs/ko/vs-code#automate-browser-tasks-with-chrome): VS Code 확장 프로그램의 브라우저 자동화
* [CLI 참조](/docs/ko/cli-reference): `--chrome`을 포함한 명령줄 플래그
* [일반적인 워크플로우](/docs/ko/common-workflows): Claude Code를 사용하는 더 많은 방법
* [데이터 및 개인정보](/docs/ko/data-usage): Claude Code가 데이터를 처리하는 방법
* [Chrome에서 Claude 시작하기](https://support.claude.com/en/articles/12012173-getting-started-with-claude-in-chrome): 바로 가기, 일정 예약 및 권한을 포함한 Chrome 확장 프로그램의 전체 문서
