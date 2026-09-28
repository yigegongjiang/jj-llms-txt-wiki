> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Linux의 Claude Desktop (베타)

> Ubuntu 및 Debian에서 Claude 데스크톱 앱 설치 및 업데이트

<Note>
  Claude 데스크톱 앱의 Linux 지원은 베타 버전입니다.
</Note>

Linux의 데스크톱 앱은 macOS 및 Windows와 동일한 Chat, Cowork 및 Claude Code 환경을 제공합니다: 병렬 세션, 시각적 diff 검토, 통합 터미널 및 편집기, 라이브 앱 미리보기. 기능 참조는 [Claude Code Desktop 사용](/docs/ko/desktop)을 참조하세요.

<h2 id="requirements">
  요구 사항
</h2>

* Debian 기반 배포판: Ubuntu 22.04 이상 또는 Debian 12 이상
* x86\_64 또는 arm64

이러한 요구 사항을 충족하는 다른 Debian 기반 배포판도 작동할 수 있지만 공식적으로 테스트되지 않았습니다. Fedora 또는 Arch와 같이 Debian 기반이 아닌 배포판에서는 [CLI](/docs/ko/setup#system-requirements) 대신 실행하십시오. Windows에서 WSL 2로 작업하는 경우 Windows 데스크톱 앱을 설치하고 배포판 내에서 세션을 실행하십시오. [Claude Code Desktop in WSL](/docs/ko/desktop-wsl)을 참조하십시오.

<h3 id="cowork-requirements">
  Cowork 요구 사항
</h3>

Cowork는 [Dispatch 및 더 긴 에이전트 작업](https://claude.com/docs/cowork/overview)을 위한 데스크톱 탭입니다. Linux에서 Cowork는 데스크톱 앱이 QEMU 및 KVM으로 호스팅하는 가상 머신에서 이러한 작업을 실행합니다. Cowork를 사용하려면 머신에 다음이 필요합니다.

* **하드웨어 가상화**: 펌웨어 설정에서 켜져 있어야 합니다. 이것이 없으면 Cowork 탭에 "Cowork requires hardware virtualization (KVM)"이 표시됩니다.
* **QEMU 및 UEFI 펌웨어**: x86\_64의 경우 `qemu-system-x86`, `ovmf`, `virtiofsd`, arm64의 경우 `qemu-system-arm`, `qemu-efi-aarch64`, `virtiofsd`. `apt install claude-desktop`은 기본적으로 권장 패키지로 설치합니다. `--no-install-recommends`로 설치했거나 권장 패키지를 건너뛰는 최소 이미지인 경우 Cowork 탭에 "Cowork requires QEMU"가 표시되고 실행할 `apt install` 명령이 표시됩니다. Ubuntu 22.04에는 `virtiofsd` 패키지가 없습니다. 앱은 거기서 번들된 복사본을 사용합니다.
* **`/dev/kvm`에 대한 액세스**: `sudo usermod -aG kvm $USER`로 사용자를 `kvm` 그룹에 추가한 다음 로그아웃했다가 다시 로그인하십시오. 일부 데스크톱 환경은 그룹 없이 로그인한 사용자에게 `/dev/kvm`에 대한 액세스를 부여하지만 Cowork는 `/dev/vhost-vsock`도 필요하며, 이는 `kvm` 그룹 멤버만 열 수 있습니다. `/dev/kvm`이 이미 작동하는 경우에도 그룹에 가입하십시오.

앱은 시작 시 이러한 요구 사항을 한 번 확인합니다. 패키지를 설치한 후 다시 시작하고 그룹에 가입한 후 로그아웃했다가 다시 로그인하십시오. `/dev/vhost-vsock`이 누락되었고 실행 중인 커널의 `/lib/modules` 아래에 모듈 디렉터리가 없으면 Cowork 탭에 커널이 Cowork에 필요한 가상화 지원을 포함하지 않으며 수동으로 추가할 수 없다고 보고합니다. 이 조합은 ChromeOS 및 컨테이너 기반 Linux 환경에서 일반적입니다.

<h2 id="install">
  설치
</h2>

터미널을 열고 각 단계의 명령을 실행하여 시스템의 정기적인 패키지 업데이트를 통해 업데이트가 도착하도록 Anthropic의 apt 저장소에서 설치합니다.

<Steps>
  <Step title="Anthropic의 apt 저장소 추가">
    이 단계에서는 `curl`을 사용하여 서명 키를 다운로드하고 `gpg`로 검증합니다. 새로운 Debian 및 Ubuntu 설치에는 이 두 도구가 포함되지 않을 수 있습니다. 어느 명령이든 `command not found`를 보고하면 먼저 둘 다 설치하세요:

    ```bash theme={null}
    sudo apt install curl gnupg
    ```

    Anthropic의 서명 키를 다운로드합니다:

    ```bash theme={null}
    sudo curl -fsSLo /usr/share/keyrings/claude-desktop-archive-keyring.asc https://downloads.claude.ai/claude-desktop/key.asc
    ```

    명령이 성공하면 아무것도 출력하지 않으며, 실패하면 `curl:` 오류를 출력합니다. 누락되었거나 잘못된 키는 나중에 `apt update`가 `NO_PUBKEY BAA929FF1A7ECACE`로 실패하게 하므로, 계속하기 전에 키가 다운로드되었으며 Anthropic에 속하는지 확인하세요:

    ```bash theme={null}
    gpg --show-keys /usr/share/keyrings/claude-desktop-archive-keyring.asc
    ```

    gpg가 출력하는 지문은 `31DDDE24DDFAB679F42D7BD2BAA929FF1A7ECACE`여야 합니다. gpg가 파일을 열 수 없거나 유효한 OpenPGP 데이터가 없다고 보고하면 다운로드가 실패했거나 잘못된 콘텐츠를 반환한 것입니다. 네트워크가 `downloads.claude.ai`에 도달할 수 있는지 확인한 후 다운로드 명령을 다시 실행하세요.

    저장소를 등록합니다:

    ```bash theme={null}
    echo "deb [arch=amd64,arm64 signed-by=/usr/share/keyrings/claude-desktop-archive-keyring.asc] https://downloads.claude.ai/claude-desktop/apt/stable stable main" | sudo tee /etc/apt/sources.list.d/claude-desktop.list
    ```
  </Step>

  <Step title="패키지 설치">
    ```bash theme={null}
    sudo apt update && sudo apt install claude-desktop
    ```
  </Step>

  <Step title="실행 및 로그인">
    애플리케이션 런처에서 **Claude**를 실행하거나 터미널에서 `claude-desktop`을 실행한 후 Anthropic 계정으로 로그인합니다.

    Linux 앱은 macOS 및 Windows와 동일한 방식으로 로그인합니다: claude.ai 구독 또는 조직의 SSO를 통해 로그인합니다. Desktop은 Claude Console API 키를 직접 허용하지 않습니다. API 키 인증의 경우 [CLI](/docs/ko/quickstart)를 사용하세요. Desktop을 Google Cloud의 Agent Platform 또는 LLM 게이트웨이로 라우팅하는 엔터프라이즈 배포의 경우 [Claude Desktop on 3P](https://claude.com/docs/third-party/claude-desktop/overview) 및 [네트워크 구성](/docs/ko/network-config)을 참조하세요.
  </Step>
</Steps>

<h3 id="install-from-a-downloaded-file">
  다운로드한 파일에서 설치
</h3>

apt 저장소를 통해 설치할 수 없는 경우 저장소의 패키지 풀에서 `.deb` 패키지를 직접 다운로드합니다. 이 명령은 저장소 인덱스에서 아키텍처에 맞는 최신 패키지를 찾은 후 현재 디렉터리에 다운로드합니다:

```bash theme={null}
curl -fLO "https://downloads.claude.ai/claude-desktop/apt/stable/$(curl -s "https://downloads.claude.ai/claude-desktop/apt/stable/dists/stable/main/binary-$(dpkg --print-architecture)/Packages" | grep '^Filename: pool/main/c/claude-desktop/claude-desktop_' | sort -V | tail -n 1 | cut -d' ' -f2)"
```

명령이 `Remote file name has no length` 오류로 실패하면 조회에서 패키지 경로를 반환하지 못한 것입니다. 이는 예를 들어 네트워크가 `downloads.claude.ai`를 차단할 때 저장소 인덱스를 가져올 수 없거나 아키텍처에 대한 패키지가 없음을 의미할 수 있습니다. 네트워크가 `downloads.claude.ai`에 도달할 수 있는지 확인하고 `dpkg --print-architecture`가 `amd64` 또는 `arm64`를 출력하는지 확인합니다. 저장소는 다른 아키텍처에 대한 패키지를 게시하지 않습니다.

Anthropic의 apt 저장소를 등록하지 않고 설치하려면 먼저 `/etc/default/claude-desktop`을 `CLAUDE_DESKTOP_ADD_REPO="false"` 줄로 생성합니다. 저장소 없이는 apt가 새 버전을 제공하지 않습니다. 업데이트하려면 다운로드 명령을 다시 실행하고 다시 설치하거나 나중에 [저장소를 등록](#install)하세요.

그런 다음 GNOME Software와 같은 소프트웨어 설치 프로그램으로 다운로드한 파일을 열거나 다운로드 파일이 포함된 디렉터리에서 apt를 사용하여 설치합니다:

```bash theme={null}
sudo apt install ./claude-desktop_*.deb
```

apt가 `E: Unsupported file ./claude-desktop_*.deb given on commandline` 오류를 보고하면 패턴이 현재 디렉터리의 `.deb` 파일과 일치하지 않은 것입니다. 다운로드가 완료되었는지 확인한 후 파일이 포함된 디렉터리에서 명령을 다시 실행하세요.

`.deb`를 설치하면 Anthropic의 apt 저장소도 `/etc/apt/sources.list.d/claude-desktop.list`에 등록되므로 향후 업데이트는 시스템의 [정기적인 패키지 업데이트](#update)와 함께 도착합니다.

<h2 id="update">
  업데이트
</h2>

데스크톱 앱은 Linux에서 자동으로 업데이트되지 않습니다. 업데이트는 시스템의 정기적인 패키지 업데이트와 함께 도착합니다:

```bash theme={null}
sudo apt update && sudo apt upgrade
```

배포판의 그래픽 소프트웨어 업데이터도 새 버전을 선택합니다.

<h2 id="uninstall">
  제거
</h2>

```bash theme={null}
sudo apt remove claude-desktop
```

패키지를 제거하면 등록된 저장소 항목과 서명 키도 함께 제거됩니다. [Anthropic의 apt 저장소 추가](#install) 단계에서 저장소 항목을 직접 추가한 경우 이것도 제거하십시오:

```bash theme={null}
sudo rm /etc/apt/sources.list.d/claude-desktop.list
```

<h2 id="troubleshoot">
  문제 해결
</h2>

<h3 id="unable-to-locate-package-claude-desktop">
  claude-desktop 패키지를 찾을 수 없음
</h3>

`sudo apt install claude-desktop`이 `E: Unable to locate package claude-desktop` 오류로 실패하면 apt가 추가한 저장소를 찾지 못한 것입니다. 다음을 확인하십시오:

* 저장소를 추가한 후 `sudo apt update`를 실행하십시오. `apt install`은 마지막으로 `apt update`를 실행한 후에 추가한 저장소를 자체적으로 인식하지 못합니다.
* 저장소 항목이 작성되었는지 확인하십시오. `cat /etc/apt/sources.list.d/claude-desktop.list`는 [Anthropic의 apt 저장소 추가](#install) 단계의 `deb` 줄을 표시해야 합니다. 파일이 비어 있거나 없으면 해당 단계를 다시 실행하십시오.
* 아키텍처가 지원되는지 확인하십시오. `dpkg --print-architecture`는 `amd64` 또는 `arm64`를 출력해야 합니다. 저장소는 다른 아키텍처에 대한 패키지를 게시하지 않습니다.
* `sudo apt update`를 다시 실행하고 `downloads.claude.ai`와 관련된 오류가 있는지 출력을 확인하십시오. 네트워크 또는 키 오류가 있으면 저장소는 추가되었지만 도달하거나 확인할 수 없다는 의미입니다.

저장소가 제자리에 있고 도달 가능하며 패키지를 여전히 찾을 수 없으면 대신 [다운로드한 파일에서 설치](#install-from-a-downloaded-file)하십시오.

<h3 id="unmet-dependencies">
  충족되지 않은 종속성
</h3>

`apt`가 `The following packages have unmet dependencies` 또는 `Unsatisfied dependencies`로 중지되면 이름이 지정된 종속성을 읽으십시오:

* `libc6 (>= 2.34)`: 배포판이 패키지가 지원하는 것보다 오래되었습니다. Ubuntu 20.04는 `libc6` 2.31을 제공합니다. Ubuntu 22.04 이상 또는 Debian 12 이상으로 업그레이드하십시오.
* 모든 누락된 종속성이 `:amd64` 또는 `:arm64` 접미사와 함께 `not installable`을 표시합니다: 머신의 아키텍처와 다른 아키텍처에 대해 `.deb`를 다운로드했습니다. `dpkg --print-architecture`를 실행하고 일치하는 `.deb`를 다운로드하거나 [apt 저장소에서 설치](#install)하십시오. 이는 아키텍처에 맞는 패키지를 선택합니다.

<h3 id="running-as-root-without-no-sandbox-is-not-supported">
  root로 --no-sandbox 없이 실행하는 것은 지원되지 않습니다
</h3>

`claude-desktop`이 이 메시지와 함께 종료되면 root로 실행했습니다. 일반 사용자로 로그인하고 거기서 실행하십시오.

<h3 id="cowork-isn’t-available">
  Cowork를 사용할 수 없음
</h3>

Cowork 탭에 다음 메시지 중 하나가 표시되면 이름이 지정된 요구 사항을 수정한 후 앱을 다시 시작하십시오:

* **Cowork에 QEMU가 필요합니다**: 메시지에 나열된 [QEMU 및 UEFI 펌웨어 패키지](#cowork-requirements)를 설치하십시오.
* **Cowork에 하드웨어 가상화(KVM)가 필요합니다**: 펌웨어 설정에서 [하드웨어 가상화](#cowork-requirements)를 켜십시오.
* **Claude에 가상화(/dev/kvm) 사용 권한이 없습니다**: 사용자를 [`kvm` 그룹](#cowork-requirements)에 추가한 후 로그아웃했다가 다시 로그인하십시오.
* **Cowork에 `vhost_vsock` 커널 모듈이 필요합니다**: `sudo modprobe vhost_vsock`를 실행한 후 앱을 다시 시작하십시오. 이는 현재 부팅에만 모듈을 로드합니다. 모든 부팅에서 로드하려면 `echo vhost_vsock | sudo tee /etc/modules-load.d/vhost_vsock.conf`를 실행하십시오.

<h2 id="what’s-not-in-the-linux-beta-yet">
  Linux 베타에서 아직 제공되지 않는 기능
</h2>

* **Computer Use**: [앱 및 화면 제어](/docs/ko/desktop#let-claude-use-your-computer)는 Linux에서 사용할 수 없습니다.
* **Dictation**: 음성 입력은 Linux 데스크톱 앱에서 사용할 수 없습니다. 대신 CLI에서 [음성 받아쓰기](/docs/ko/voice-dictation)를 사용하세요.
* **Quick Entry 전역 핫키**: X11에서 작동합니다. 기본 Wayland에서는 데스크톱 환경의 GlobalShortcuts 포털이 필요합니다.
* **Fedora 및 RHEL**: 현재 Debian 기반 배포판만 지원됩니다. 추가 배포판에 대한 지원은 향후 제공될 예정입니다.

데스크톱 앱에서 아직 사용할 수 없는 기능의 경우 [CLI](/docs/ko/quickstart)는 동일한 Claude Code 엔진을 실행하며 더 넓은 범위의 Linux 배포판을 지원합니다. [시스템 요구 사항](/docs/ko/setup#system-requirements)을 참조하세요.
