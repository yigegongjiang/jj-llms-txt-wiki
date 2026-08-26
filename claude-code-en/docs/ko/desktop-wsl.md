> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# WSL의 Claude Code Desktop

> WSL 2 배포판 내에서 Code 세션 실행

Windows에서 Code 탭은 Windows 자체가 아닌 WSL 2 배포판 내에서 세션을 실행할 수 있습니다. 세션의 Claude Code 프로세스, 해당 도구 및 git은 모두 배포판 내에서 실행되며, 해당 배포판의 Linux 도구 체인과 기본 Linux 경로를 사용하여 프로젝트가 대상으로 하는 동일한 환경에서 실행됩니다.

저장소가 배포판의 파일 시스템 내에 있을 때 WSL 세션을 사용합니다. Windows에서 해당 파일을 작업하면 네트워크 파일 시스템을 통해 진행되므로 속도가 느리고 파일 감시가 중단됩니다. 배포판 내에서 세션을 실행하면 둘 다 피할 수 있습니다.

<h2 id="requirements">
  요구 사항
</h2>

* [WSL 2](https://learn.microsoft.com/windows/wsl/install)가 설치된 Windows 10 또는 11. WSL 1은 지원되지 않습니다.
* 설치된 배포판이 최소 하나 이상 필요합니다(예: Ubuntu).
* 배포판 내에 `git`이 설치되어 있어야 합니다.

<h2 id="start-a-wsl-session">
  WSL 세션 시작
</h2>

<Steps>
  <Step title="배포판 선택">
    Code 탭에서 새 세션을 시작하고 환경 선택기를 엽니다. 설치된 WSL 2 배포판이 **WSL** 섹션에 나타납니다. 하나를 선택합니다.
  </Step>

  <Step title="폴더 선택">
    세션은 배포판의 홈 디렉터리에서 시작됩니다. 폴더 선택기를 사용하여 프로젝트 폴더를 선택합니다. 검색은 배포판 내에서 진행되며 `/home/you/project`와 같은 Linux 경로를 사용합니다.
  </Step>

  <Step title="폴더 신뢰">
    폴더의 첫 번째 세션에는 작업 영역 신뢰 대화 상자가 표시됩니다. 신뢰는 배포판 및 폴더별로 부여됩니다. 한 배포판의 폴더를 신뢰하는 것이 다른 배포판이나 Windows의 동일한 경로에 적용되지는 않습니다.
  </Step>
</Steps>

배포판의 첫 번째 세션은 Claude가 내부에 설정되는 동안 약간 더 오래 걸립니다. 일반 폴더 선택기에서 `\\wsl.localhost\...` 폴더를 열 수도 있으며, 해당 배포판 내에서 다시 열립니다.

최근에 사용한 폴더는 배포판별로 선택기에 나타나므로 프로젝트에 다시 연결하는 것은 한 번의 클릭입니다.

<h2 id="what-works-in-a-wsl-session">
  WSL 세션에서 작동하는 기능
</h2>

병렬 세션, 측면 채팅, 시각적 diff 검토, 분기 및 풀 요청 상태, worktrees는 모두 배포판 내의 git 및 도구 체인으로 지원됩니다. "편집기에서 열기"는 [Remote - WSL](https://code.visualstudio.com/docs/remote/wsl)을 통해 배포판에 연결된 VS Code를 엽니다.

WSL 세션에서 아직 사용할 수 없는 몇 가지 기능이 있습니다. 통합 터미널, 커넥터 및 플러그인, 세션 포킹, 파일 브라우저 창, 그리고 작성기에서 `@`를 입력할 때의 파일 제안입니다.

<h2 id="managed-devices">
  관리되는 디바이스
</h2>

조직에서 관리하는 디바이스에서는 WSL 세션을 사용할 수 없을 수 있습니다. 세션 시작이 디바이스가 관리된다는 메시지와 함께 실패하면 이는 관리자가 제어합니다. 관리자: 배포 가이드의 [설정이 디바이스에 도달하는 방식 결정](/docs/ko/admin-setup#decide-how-settings-reach-devices)을 참조하세요.
