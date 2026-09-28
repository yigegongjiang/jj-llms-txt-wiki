> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 클라우드 환경 구성

> Claude Code 클라우드 세션을 위한 클라우드 환경 구성: 네트워크 액세스 수준, 환경 변수, 설정 스크립트 및 환경 캐싱.

<Note>
  클라우드 환경은 [클라우드 세션](/docs/ko/claude-code-on-the-web)에 적용되며, 이는 Pro, Max, Team 플랜에서 사용 가능하고, [프리미엄 시트 또는 Chat + Claude Code 시트](https://support.claude.com/en/articles/11845131-use-claude-code-with-your-team-or-enterprise-plan)가 있는 Enterprise 사용자를 위한 것입니다.
</Note>

각 [클라우드 세션](/docs/ko/claude-code-on-the-web)은 클라우드 환경에서 실행됩니다. 환경을 구성하여 [네트워크 액세스](#access-levels)를 허용하거나 거부하고, 세션에 대한 [환경 변수](#set-environment-variables)를 설정하며, Pro 및 Max 플랜에서 세션이 사용하는 [API 자격 증명](#add-api-credentials)을 저장하고, Claude가 작업을 시작하기 전에 [설정 스크립트](#setup-scripts)를 실행할 수 있습니다.

동일한 환경이 클라우드 세션을 시작하는 모든 곳에 적용됩니다: [Desktop 앱](/docs/ko/desktop), [Claude 모바일 앱](/docs/ko/mobile), [claude.ai/code](https://claude.ai/code)의 브라우저, [`claude --cloud`](/docs/ko/claude-code-on-the-web#from-terminal-to-cloud)를 사용한 터미널, [루틴](/docs/ko/routines), 그리고 [Claude Tag](https://claude.com/docs/claude-tag/overview). 이러한 각 표면은 [자체 호스팅 환경](/docs/ko/self-hosted-environments)으로도 라우팅할 수 있습니다. [가용성 및 제한 사항](/docs/ko/self-hosted-environments#availability-and-limitations)은 Claude Tag 세션이 하나에서 실행될 때 Claude가 아직 사용할 수 없는 항목을 다룹니다.

<Info>
  [Remote Control](/docs/ko/remote-control) 세션은 웹 및 모바일 인터페이스를 자신의 머신에 있는 세션에 연결하며, 이는 클라우드 환경이 아닌 머신의 네트워크 및 파일을 사용합니다. Claude Tag 채널 세션은 조직 수준 환경만 사용하며, [공유 환경](#organization-shared-environments) 또는 [자체 호스팅 환경](/docs/ko/self-hosted-environments) 중 하나입니다.
</Info>

<h2 id="the-default-environment">
  Default 환경
</h2>

아직 환경이 없으면 온보딩이 **Default** 환경을 설정합니다. 온보딩 방식에 따라 다릅니다:

* **`/web-setup`과 같은 CLI 흐름**: **Default**를 생성합니다
* **Pro 및 Max의 웹 온보딩**: **Default**를 생성합니다
* **Team 및 Enterprise의 웹 온보딩**: Owner가 [빠른 웹 설정](/docs/ko/claude-code-on-the-web#github-authentication-options)을 켜지 않은 한 **첫 번째 클라우드 환경 생성** 양식을 표시합니다. 양식의 기본값을 유지하고 **생성 및 완료**를 클릭하여 동일한 **Default** 환경을 얻습니다

**Default**는 자체 구성을 수행하지 않습니다:

* [**Trusted** 네트워크 액세스](#access-levels): 세션이 패키지 레지스트리 및 기타 [허용 목록 도메인](#default-allowed-domains)에 도달하고 세션의 네트워크를 통해 다른 것에는 도달하지 않습니다.
* 다른 구성 없음: **Default**는 환경 변수 또는 설정 스크립트를 정의하지 않으므로 세션은 [사전 설치된 도구](#installed-tools)만으로 시작됩니다.

**Default**만 사용 가능한 경우 모든 세션이 이 환경에서 실행됩니다. 둘 이상의 환경이 있는 경우 세션은 표면별로 하나를 선택합니다:

* Desktop 앱, 모바일 앱 및 claude.ai/code에서 직접 시작하는 세션은 [선택기](#configure-your-environment)에 표시된 환경을 사용합니다. 선택하지 않았을 때 Owner가 설정한 [조직 기본값](#organization-shared-environments)이 선택을 채웁니다. [프로젝트](/docs/ko/claude-projects#project-settings-reference)의 스레드는 프로젝트 설정에서 설정된 환경을 사용합니다.
* CLI에서 Claude Code는 [`/remote-env` 선택](#select-an-environment-from-the-cli)을 사용하거나, 목록에 하나가 있으면 Anthropic 호스팅 환경으로 폴백하고, 그렇지 않으면 브리지 환경이 아닌 목록의 첫 번째 환경으로 폴백합니다. 브리지 환경은 클라우드 환경이 아닌 자신의 머신을 나타내기 위해 [Remote Control](/docs/ko/remote-control)이 등록하는 항목입니다. [자체 호스팅 환경](/docs/ko/self-hosted-environments)의 경우 `ccpool_` ID를 사용하여 `--environment <environment-id>`를 전달하면 [세션을 디스패치할 때](/docs/ko/self-hosted-environments-testing#run-the-test-loop) `/remote-env` 선택 및 해당 호출에 대한 폴백을 재정의합니다. Claude Code는 플래그에 전달된 Anthropic 호스팅 `env_` ID를 거부하므로 이를 대상으로 하려면 `/remote-env`를 사용합니다. 플래그는 Claude Code v2.1.224 이상이 필요합니다.

기본값이 충분하지 않을 때 환경을 구성합니다: Claude가 [기본 허용 목록](#default-allowed-domains) 외부의 도메인에 도달해야 하거나, 세션에 대해 환경 변수를 설정해야 하거나, 작업을 시작하기 전에 종속성을 설치해야 할 때입니다.

<h2 id="configure-your-environment">
  환경 구성
</h2>

[웹 온보딩](/docs/ko/web-quickstart) 후 [claude.ai/code](https://claude.ai/code)에서 또는 [Desktop 앱](/docs/ko/desktop#cloud-sessions)의 프롬프트 상자에서 환경 선택기를 통해 환경을 생성, 편집 및 보관할 수 있습니다. 생성한 환경은 계정에 개인적으로 속하며, Owner가 생성한 [공유 환경](#organization-shared-environments)은 동일한 선택기에 나타납니다. 구성 없이 사용 가능한 항목은 [설치된 도구](#installed-tools)를 참조하십시오.

<Steps>
  <Step title="환경 선택기 열기">
    [claude.ai/code](https://claude.ai/code)에서 메시지 상자 위의 행에 있는 현재 환경의 이름을 표시하는 클라우드 아이콘을 선택합니다. 선택기에 대한 설정 페이지나 직접 URL은 없습니다.

    <Frame>
      <img src="https://mintcdn.com/claude-code/ZFId6l95856c5LSw/images/cloud-environment-selector.png?fit=max&auto=format&n=ZFId6l95856c5LSw&q=85&s=cc2813a5664519eaf5a89d793ce5af26" alt="claude.ai/code의 메시지 상자 위에 열려 있는 환경 선택기입니다. 환경 이름 Default를 표시하는 클라우드 버튼이 메시지 상자 위의 행에 있습니다. 열린 메뉴에는 Download 및 Desktop only 레이블이 있는 Local 행, Default 환경이 체크 표시로 선택되어 있고 마우스를 올리면 설정 기어 아이콘이 표시되는 Cloud 섹션, Add cloud environment 옵션, 그리고 설정 지침이 있는 Remote Control 섹션이 나열됩니다." width="1672" height="682" data-path="images/cloud-environment-selector.png" />
    </Frame>
  </Step>

  <Step title="환경 추가 또는 편집">
    **Add cloud environment**를 선택하거나, 기존 환경 위에 마우스를 올리고 오른쪽에 나타나는 설정 아이콘을 선택합니다. 대화 상자에는 이름, 네트워크 액세스 수준, 환경 변수 및 설정 스크립트가 포함됩니다. Pro 또는 Max 플랜에서 기존 클라우드 환경을 편집할 때 대화 상자에는 [API 자격 증명](#add-api-credentials)도 포함됩니다.

    <Frame>
      <img src="https://mintcdn.com/claude-code/ZFId6l95856c5LSw/images/cloud-environment-dialog.png?fit=max&auto=format&n=ZFId6l95856c5LSw&q=85&s=30d4478b31d1f879f7ee287ddab32505" alt="New cloud environment 대화 상자입니다. 기본값 자리 표시자가 있는 Name 필드, Trusted로 설정된 Network access 선택기(네트워크 정책 및 액세스 수준에 대한 링크 포함), .env 형식 자리 표시자 텍스트가 있는 Environment variables 상자(환경을 사용하는 모든 사람이 값을 볼 수 있다는 참고 사항 포함), Claude Code가 시작되기 전에 새 세션이 시작될 때 실행되는 Bash 스크립트로 설명된 Setup script 상자, 그리고 Cancel 및 Create environment 버튼이 있습니다." width="874" height="1372" data-path="images/cloud-environment-dialog.png" />
    </Frame>
  </Step>
</Steps>

<h3 id="set-environment-variables">
  환경 변수 설정
</h3>

환경 변수는 `.env` 형식을 사용하며, 한 줄에 하나의 `KEY=value` 쌍을 사용합니다. 일반 값은 따옴표가 필요하지 않으며, 일치하는 쌍으로 값을 인용하면 따옴표가 값의 일부가 되지 않습니다. 여러 줄에 걸쳐 있거나 `#`을 포함하는 값을 인용합니다. 인용되지 않은 값에서 `#`은 주석을 시작하고 줄의 나머지 부분은 삭제됩니다.

다음 예제는 세 개의 변수를 정의합니다.

```text theme={null}
NODE_ENV=development
LOG_LEVEL=debug
DATABASE_URL=postgres://localhost:5432/myapp
```

각 세션은 시작 시 환경의 값을 한 번 복사하여 Claude가 실행하는 모든 명령이 읽을 수 있는 일반 환경 변수로 변환합니다. 실행 중인 세션이 구성을 다시 읽지 않기 때문에 변수를 편집하거나 추가하면 이후에 시작하는 세션에 영향을 미칩니다. 이미 실행 중인 세션은 시작할 때의 값을 유지합니다.

클라우드 세션은 시작할 때 자체적으로 일부 변수도 설정합니다. [`CLAUDE_AUTOCOMPACT_PCT_OVERRIDE`](/docs/ko/claude-code-on-the-web#manage-context)의 경우, 세션이 설정하는 값이 여기에 추가한 값을 재정의하므로 여기에 해당 키를 추가해도 효과가 없습니다.

환경을 사용하는 모든 사람이 값을 읽을 수 있습니다. Pro 및 Max 플랜에서는 에이전트 프록시가 요청에 첨부할 수 있는 키에 대해 [API 자격 증명](#add-api-credentials)을 대신 사용합니다. [자격 증명을 받지 않는 요청](#requests-that-never-get-the-credential)이 나열되어 있습니다.

<h3 id="add-api-credentials">
  API 자격 증명 추가
</h3>

API 자격 증명은 클라우드 환경에 저장하는 API 키 또는 토큰으로, Claude가 키를 보지 않고도 환경의 모든 세션에서 해당 API를 호출할 수 있습니다. Anthropic의 에이전트 프록시는 각 요청이 세션의 VM을 떠난 후 나열한 호스트에 대한 요청에 키를 추가합니다. 키는 Claude, 실행하는 명령 또는 세션의 환경 변수에 도달하지 않습니다.

API 자격 증명은 Pro 및 Max 플랜에서 사용 가능합니다. Team 또는 Enterprise 플랜에서는 아직 사용할 수 없으므로 **API credentials** 섹션이 해당 플랜의 환경 대화 상자에 나타나지 않습니다.

<h4 id="requirements">
  요구 사항
</h4>

이 중 두 개는 자격 증명을 추가할 수 있는지 여부를 결정하고, 두 개는 추가된 후 에이전트 프록시가 사용할 수 있는지 여부를 결정합니다.

* **Role**: claude.ai 조직의 조직 관리자 역할
  * Team 및 Enterprise에서는 Owner가 보유하고 Admin은 보유하지 않습니다.
  * Pro 및 Max에서는 자신의 조직에서 보유합니다.
  * 이 역할이 없으면 자신의 환경에서도 자격 증명 목록 대신 참고 사항이 표시됩니다. Owner에게 공유 환경에 자격 증명을 추가하고 해당 환경에서 세션을 실행하도록 요청하십시오.
* **Environment type**: 이미 존재하는 Anthropic 호스팅 클라우드 환경입니다. [자체 호스팅 환경](/docs/ko/self-hosted-environments)에는 API 자격 증명이 없습니다.
* **API reachability**: API가 인터넷에서의 연결을 수락합니다. 요청이 Anthropic의 네트워크에서 나가기 때문입니다.
* **Encryption keys**: 조직이 고객 관리 암호화 키를 사용하는 경우 자격 증명을 저장할 수 없습니다.

<h4 id="add-a-credential">
  자격 증명 추가
</h4>

이미 존재하는 환경의 편집기에서 한 번에 하나씩 자격 증명을 추가합니다. 새 환경의 대화 상자는 이를 제공하지 않습니다. 편집도 없습니다. 자격 증명의 호스트 또는 값을 변경하려면 삭제하고 다시 추가합니다.

<Steps>
  <Step title="환경의 API 자격 증명 열기">
    [claude.ai/code](https://claude.ai/code)의 환경 선택기에서 [편집할 환경을 열기](#configure-your-environment)합니다. **Update cloud environment** 대화 상자에서 **Environment variables** 아래에 **API credentials**를 찾습니다. 환경에 이미 있는 자격 증명과 적용되는 호스트를 볼 수 있습니다.
  </Step>

  <Step title="자격 증명 추가">
    **Add credential**을 선택하고 양식을 작성합니다. 요청 헤더에서 이동하는 API 키에 대해 기본 **Credential type**, **Bearer**를 유지하고 다음 필드를 작성합니다.

    * **Name**: `Internal billing API`와 같은 자격 증명의 레이블
    * **Allowed websites**: `api.example.com`과 같은 API의 호스트입니다. 선행 `*.`은 모든 하위 도메인과 일치합니다.
    * **Custom headers**: 키를 전달하는 헤더에 대한 한 행입니다. 행은 헤더의 **Name**으로 `Authorization`으로 시작하고 **Prefix**로 `Bearer`로 시작합니다. 키 자체를 **Value**로 붙여넣습니다. `X-Api-Key`와 같이 기본 값을 사용하는 헤더의 경우 이름을 변경하고 접두사를 지웁니다.

    다른 방식으로 인증하는 API의 경우 다른 **Credential type**을 선택합니다. 목록은 Team 및 Enterprise 플랜의 Slack 통합인 [Claude Tag](https://claude.com/docs/claude-tag/overview)가 [연결](https://claude.com/docs/claude-tag/admins/add-connections)에 제공하는 것과 동일합니다.
  </Step>

  <Step title="자격 증명 저장">
    **Connect**를 선택합니다. 자격 증명이 호스트와 함께 목록에 나타나며, 대화 상자의 **Save changes** 버튼 없이 저장됩니다. 저장 후 값을 다시 볼 수 없습니다.
  </Step>
</Steps>

자격 증명이 작동하는지 확인하려면 환경에서 세션을 시작하고 Claude에게 `curl`과 같은 API를 호출하도록 요청합니다. API는 키가 요청에 있는 것처럼 응답하며, 키는 세션의 환경 변수나 파일에 나타나지 않습니다. 목록이 자격 증명을 **Not sent**로 표시하는 경우, 아래의 참고 사항에 이유와 수행할 작업이 설명되어 있습니다. 호스트가 정확히 일치하지 않고 겹치는 두 자격 증명은 마커를 받지 않으며, 에이전트 프록시는 그 중 하나만 보냅니다.

<h4 id="which-requests-get-the-credential">
  자격 증명을 받는 요청
</h4>

에이전트 프록시는 요청의 호스트가 해당 자격 증명에 나열한 호스트 중 하나와 일치할 때 자격 증명을 요청에 첨부합니다. 세션은 환경의 [네트워크 액세스 수준](#access-levels)이 그렇지 않으면 허용하지 않을 때에도 해당 호스트에 도달할 수 있습니다. 단, [자격 증명을 받지 않는 호스트](#requests-that-never-get-the-credential)는 제외됩니다. 자격 증명은 삭제할 때까지 환경에서 실행되는 모든 세션에 적용되며, 누가 시작했는지는 상관없습니다.

<h4 id="requests-that-never-get-the-credential">
  자격 증명을 받지 않는 요청
</h4>

에이전트 프록시는 다음 요청에 추가한 자격 증명을 첨부하지 않습니다.

* **GitHub**: [GitHub 프록시](#github-proxy)가 대신 GitHub에 대한 요청을 인증하므로 GitHub에 대한 API 자격 증명이 필요하지 않습니다.
* **Anthropic API 및 공개 패키지 레지스트리**: `api.anthropic.com`, `registry.npmjs.org`, `jsr.io`, `npm.jsr.io`, `pypi.org`, `files.pythonhosted.org`, `index.crates.io`, 및 `proxy.golang.org`
* **Setup script 요청**: Claude Code는 [설정 스크립트](#setup-scripts)가 실행된 후 시작할 때 에이전트 프록시에 연결합니다.

<h3 id="select-an-environment-from-the-cli">
  CLI에서 환경 선택
</h3>

터미널에서 `/remote-env`를 실행하여 [`claude --cloud`](/docs/ko/claude-code-on-the-web#from-terminal-to-cloud)와 같이 CLI에서 생성하는 클라우드 세션의 기본 환경을 선택합니다. 명령은 기존 환경의 선택기를 열고 선택을 [사용자 설정](/docs/ko/settings#where-settings-live)의 `remote.defaultEnvironmentId` 키에 저장하므로, 변경할 때까지 머신의 모든 프로젝트에 적용되며, 리포지토리의 프로젝트 설정과 같은 더 높은 우선 순위 [설정 레이어](/docs/ko/settings#settings-precedence)에서 동일한 키가 설정되지 않은 경우입니다.

[자체 호스팅 환경](/docs/ko/self-hosted-environments) ID는 `ccpool_...` 형식이며 더 엄격한 소스 규칙을 따릅니다. Claude Code가 이를 인정하는 설정 레이어는 [`remote.defaultEnvironmentId`](/docs/ko/settings-reference#remote-defaultenvironmentid)를 참조하십시오.

`/remote-env`는 기본값만 설정합니다. 세션을 시작하지 않으며 환경을 추가하거나 편집할 수 없습니다. [환경 선택기](#configure-your-environment)에서 관리합니다.

<h3 id="archive-an-environment">
  환경 보관
</h3>

자신의 환경 중 하나를 보관하려면 편집을 위해 열고 **Archive**를 선택합니다. Owner는 관리 설정의 **Cloud environments** 페이지에서 [공유 환경](#organization-shared-environments)을 보관합니다. 환경을 삭제할 수 없으며, 보관만 할 수 있습니다.

보관은 실행 중인 세션이 아닌 새 세션에 영향을 미칩니다.

* 환경에서 이미 실행 중인 세션은 계속 작동합니다.
* 환경이 선택기 및 `/remote-env`에서 사라지므로 새 세션에 대해 선택할 수 없습니다.
* 환경의 API 자격 증명은 실행 중인 세션에 첨부된 상태로 유지됩니다. 보관하기 전에 더 이상 원하지 않는 항목을 삭제합니다.
* 보관된 환경에서는 어떤 표면에서도 새 세션을 시작할 수 없습니다. 환경이 저장된 [CLI 기본값](#select-an-environment-from-the-cli)이었다면, 목록에 Anthropic 호스팅 환경이 있을 때 Claude Code는 CLI 클라우드 세션을 해당 환경에서 시작하고, 그렇지 않으면 [Remote Control 브리지 환경](#the-default-environment)이 아닌 목록의 첫 번째 환경에서 시작합니다. [루틴](/docs/ko/routines#environments-and-network-access)과 같이 환경으로 명시적으로 구성된 모든 항목은 새 세션을 시작할 수 없습니다. 다른 환경을 가리키도록 합니다.

<h3 id="organization-shared-environments">
  조직 공유 환경
</h3>

Team 및 Enterprise 플랜에서 Owner는 조직의 모든 구성원과 공유되는 클라우드 환경을 생성할 수 있습니다. 동일한 역할은 **Cloud environments** 관리 페이지에서 [자체 호스팅 환경](/docs/ko/self-hosted-environments)을 포함한 다른 모든 항목을 관리합니다. Admin 역할은 페이지를 열 수 없습니다. 이를 열 수 있는 역할의 전체 목록은 [서버 관리 설정 관리](/docs/ko/server-managed-settings#access-control)에 대한 것입니다.

공유 환경은 각 구성원의 [환경 선택기](#configure-your-environment)에 **Organization** 제목 아래에 나타나며, 구성원의 자신의 환경은 **Personal** 아래에 있으므로, 팀이 각 구성원이 다시 생성하는 대신 하나의 구성으로 표준화할 수 있습니다. 공유 환경의 설정 아이콘을 선택하면 모든 구성원(Owner 포함)에 대한 구성의 읽기 전용 요약이 열립니다.

Owner는 다음 두 가지 방법 중 하나로 환경을 조직에서 사용 가능하게 만듭니다.

* **공유 환경 생성**: [관리 설정](https://claude.ai/admin-settings)의 **Cloud environments** 페이지를 사용합니다. 이는 Owner가 공유 환경을 편집하고 보관하는 곳이기도 합니다. 각각은 이름, [네트워크 액세스 수준](#access-levels), `.env` 형식의 [환경 변수](#set-environment-variables), 및 [설정 스크립트](#setup-scripts)를 가집니다.
* **개인 환경 공유**: 환경 선택기에서 자신의 환경 중 하나를 편집을 위해 열고, **Who can use it** 행에서 공유합니다. 환경은 ID를 유지하므로 이미 사용 중인 세션 및 루틴은 영향을 받지 않으며, 모든 구성원이 이를 보고 세션을 시작할 수 있습니다.

Owner는 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)에서 조직의 [기본 환경](#the-default-environment)을 별도로 선택합니다.

모든 구성원의 공유 환경의 세션은 해당 변수를 읽으므로 비밀을 포함하지 마십시오. [자격 증명을 읽을 수 없는 세션을 제공하는 API 자격 증명](#add-api-credentials)은 Team 또는 Enterprise 플랜에서 아직 사용할 수 없습니다.

<h3 id="set-the-environment-a-claude-tag-channel-uses">
  Claude Tag 채널이 사용하는 환경 설정
</h3>

[Claude Tag](https://claude.com/docs/claude-tag/overview) 채널에서 Claude는 모든 구성원이 아닌 조직의 공유 ID로 작동하므로, 채널 세션은 조직 수준 환경만 사용합니다. 공유 환경 또는 [자체 호스팅 환경](/docs/ko/self-hosted-environments)입니다. 채널에 .NET과 같이 [사전 설치](#installed-tools)되지 않은 도구 체인을 제공하려면 Owner는 **Cloud environments** 관리 페이지에서 [공유 환경](#organization-shared-environments)을 생성할 수 있습니다. 이를 설치하는 [설정 스크립트](#setup-scripts)를 사용합니다. 다음 두 가지 방법 중 하나로 채널을 환경에 가리킵니다.

* 공유 또는 자체 호스팅 환경을 [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)에서 조직의 [기본 환경](#the-default-environment)으로 설정합니다.
* Claude Tag 관리 설정에서 [채널에 하나를 고정](https://claude.com/docs/claude-tag/admins/troubleshooting#channel-sessions-use-the-wrong-environment-or-can%E2%80%99t-find-one)합니다.

<h2 id="network-access">
  네트워크 액세스
</h2>

각 환경은 하나의 네트워크 액세스 수준을 설정하며, 이는 세션이 만들 수 있는 아웃바운드 연결을 제어합니다. 기본 수준인 **Trusted**는 패키지 레지스트리 및 기타 [허용 목록 도메인](#default-allowed-domains)을 허용합니다. **Custom**은 자신의 도메인 목록을 사용합니다.

환경의 네트워크 액세스를 변경하려면 [편집을 위해 열고](#configure-your-environment) 대화 상자에서 **Network access** 선택기를 사용합니다. [공유 환경](#organization-shared-environments)은 읽기 전용으로 열리므로 Owner는 [관리 설정](https://claude.ai/admin-settings)의 **Cloud environments** 페이지에서 네트워크 액세스를 변경합니다. 선택기를 여는 클라우드 아이콘은 [Default 환경](#the-default-environment) 아래에 나열된 앱 표면 및 [루틴 편집기](/docs/ko/routines#environments-and-network-access)에 나타납니다. 개인 환경에는 claude.ai 계정 설정에 별도의 페이지가 없습니다.

<Note>
  세션 또는 루틴에서 활성화한 MCP 커넥터는 **Allowed domains**에 호스트를 추가하지 않고도 작동합니다. 커넥터 트래픽이 세션의 네트워크가 아닌 Anthropic의 서버를 통해 이동하기 때문입니다. 이는 [보안 및 격리](/docs/ko/claude-code-on-the-web#security-and-isolation) 아래에 언급된 동일한 Anthropic 바운드 채널에 의존합니다. 필요하지 않은 커넥터를 끄면 Claude가 도달할 수 있는 도구를 제한할 수 있습니다.
</Note>

<h3 id="access-levels">
  액세스 수준
</h3>

[환경 대화 상자](#configure-your-environment)의 **Network access** 필드는 다음 네 가지 수준 중 하나를 사용합니다:

| 수준          | 아웃바운드 연결                                                            |
| :---------- | :------------------------------------------------------------------ |
| **None**    | 세션의 네트워크를 통한 아웃바운드 네트워크 액세스 없음                                      |
| **Trusted** | [허용 목록 도메인](#default-allowed-domains)만: 패키지 레지스트리, GitHub, 클라우드 SDK |
| **Full**    | 모든 도메인                                                              |
| **Custom**  | 자신의 허용 목록, 선택적으로 기본값 포함                                             |

어느 수준을 선택하든 세션은 여전히 이에 도달할 수 있습니다. 각각은 세션의 네트워크 허용 목록을 통과하지 않는 경로를 사용하기 때문입니다:

* GitHub([별도의 프록시](#github-proxy)를 통해)
* 활성화한 [MCP 커넥터](#network-access)(트래픽이 Anthropic의 서버를 통해 이동)
* 환경의 [API 자격 증명](#add-api-credentials)에 나열한 호스트([에이전트 프록시가 건너뛰는 호스트](#requests-that-never-get-the-credential) 제외)
* Anthropic API(Claude Code의 자체 요청의 경우, [보안 및 격리](/docs/ko/claude-code-on-the-web#security-and-isolation) 아래에 언급된 대로 **None**에서도)

<h3 id="allow-specific-domains">
  특정 도메인 허용
</h3>

Trusted 목록에 없는 도메인을 허용하려면 환경의 네트워크 액세스 설정에서 **Custom**을 선택한 다음 **Allowed domains** 필드에 한 줄에 하나의 도메인을 나열합니다. 이 예제는 내부 프로젝트에 필요할 수 있는 세 개의 호스트를 허용합니다.

```text theme={null}
api.example.com
*.internal.example.com
registry.example.com
```

이 환경의 세션은 이제 `api.example.com`, `internal.example.com`의 모든 하위 도메인 및 `registry.example.com`에 도달할 수 있으며 세션의 네트워크를 통해 다른 도메인에는 도달할 수 없습니다. [GitHub 트래픽](#github-proxy), [MCP 커넥터 트래픽](#network-access) 및 환경의 [API 자격 증명](#add-api-credentials)의 호스트에 대한 요청([에이전트 프록시가 건너뛰는 호스트](#requests-that-never-get-the-credential) 제외)은 이 허용 목록을 통과하지 않습니다. 선행 `*.`은 모든 하위 도메인과 일치합니다. [Trusted 도메인](#default-allowed-domains)도 유지하려면 **Also include default list of common package managers**를 확인합니다. 나열한 것만 허용하려면 선택 해제합니다.

조직이 [아티팩트](/docs/ko/artifacts#availability)를 사용하는 경우 세션이 이를 읽기 위해 목록에 `*.frame.claudeusercontent.com`이 필요하지 않습니다. 목록이 해당 호스트를 생략하면 Claude Code는 세션의 Anthropic 연결을 통해 아티팩트 콘텐츠를 읽습니다. 두 가지 상황에서 호스트를 허용 목록에 유지합니다:

* **이 환경의 세션이 다른 조직의 공개 아티팩트를 열기**: Claude Code는 호스트에서 직접 이를 가져오므로 이 목록에 추가합니다.
* **로컬 CLI 또는 자체 호스팅 러너를 구성하는 경우**: 호스트를 해당 허용 목록에 유지합니다. [네트워크 액세스 요구 사항](/docs/ko/network-config#network-access-requirements) 및 자체 호스팅 [네트워크 요구 사항](/docs/ko/self-hosted-environments-deploy#network-requirements)을 참조합니다.

각 환경에는 자체 허용 도메인 목록이 있습니다. 관리자가 모든 구성원의 환경에 푸시할 수 있는 조직 수준의 허용 목록이 없습니다. [서버 관리 설정](/docs/ko/server-managed-settings)은 여전히 클라우드 세션 내에 적용되지만 환경의 네트워크 허용 목록에 도메인을 추가하는 것은 없습니다. 팀에 하나의 표준 목록을 제공하려면 Owner가 **Custom** 네트워크 액세스 및 해당 목록이 있는 [조직 공유 환경](#organization-shared-environments)을 만들 수 있습니다.

<h3 id="github-proxy">
  GitHub 프록시
</h3>

Anthropic 호스팅 환경에서 모든 GitHub 작업은 실제 GitHub 자격 증명을 세션의 VM 외부에 유지하는 전용 프록시를 통해 이동하며, 환경의 [액세스 수준](#access-levels)과 무관합니다. 자체 호스팅 환경의 세션은 배포가 제공하는 자격 증명으로 git 작업을 인증합니다. [Git 구성](/docs/ko/self-hosted-environments-deploy#configure-git)은 세션별 발급 자격 증명 및 이 동일한 프록시에 대한 옵트인을 포함한 옵션을 다룹니다. 프록시는 다음을 제공합니다:

* **Git 자격 증명**: VM 내부의 git 클라이언트는 범위가 지정된 자격 증명을 사용하며, 프록시는 이를 확인하고 실제 GitHub 토큰으로 교환합니다.
* **API 요청**: 기본 제공 GitHub 도구의 요청 및 [`proxy-injected` 자리 표시자](#work-with-github-issues-and-pull-requests) 아래의 `gh`에서의 요청은 실제 자격 증명이 대체되어 나갑니다.
* **푸시 보호**: `git push`는 세션의 현재 작업 분기에 대해서만 작동합니다. 복제, 페치 및 PR 작업은 정상적으로 작동합니다.
* **리포지토리 범위**: GitHub API 및 릴리스 자산 요청은 세션에 연결된 리포지토리에만 도달하므로 연결되지 않은 리포지토리에서 릴리스 자산을 다운로드하는 설정 스크립트는 403을 받습니다.
* **GraphQL 제한**: 프록시는 풀 요청 워크플로우에 대해서만 고정된 GraphQL 작업 세트를 제공합니다. 프록시는 GraphQL 엔드포인트의 다른 모든 것을 `This GraphQL query is not enabled for this session`이라고 말하는 403으로 거부하고 REST 폴백인 `gh api repos/{owner}/{repo}/...`의 이름을 지정합니다. 제한은 제공하는 자격 증명과 관계없이 프록시를 통한 모든 요청에 적용되므로 설정한 `GH_TOKEN`은 동일한 403을 받습니다. Claude는 Projects v2와 같이 GraphQL에만 존재하는 GitHub API에 프록시를 통해 도달할 수 없습니다.

공개 리포지토리의 커밋된 파일은 `raw.githubusercontent.com`을 통해 도착하며, [보안 프록시](#security-proxy)가 대신 처리합니다. 해당 도메인은 기본 [Trusted 목록](#default-allowed-domains)에 있으므로 환경의 [액세스 수준](#access-levels)이 이를 제외하지 않는 한 이러한 파일은 도달 가능합니다.

<h3 id="security-proxy">
  보안 프록시
</h3>

Anthropic 호스팅 환경의 클라우드 세션은 보안 및 남용 방지 목적으로 HTTP/HTTPS 네트워크 프록시 뒤에서 실행됩니다. [자체 호스팅 환경](/docs/ko/self-hosted-environments-deploy#default-deny-egress)에서 아웃바운드 트래픽은 자신의 네트워크 경계를 통해 나갑니다. Anthropic 호스팅 세션의 모든 아웃바운드 인터넷 트래픽은 이 프록시를 통과하며, 다음을 제공합니다:

* 악의적인 요청으로부터의 보호
* 속도 제한 및 남용 방지
* 향상된 보안을 위한 콘텐츠 필터링
* 요청된 호스트 이름의 DNS 수준 감사 추적

<h2 id="what’s-available-in-cloud-sessions">
  클라우드 세션에서 사용 가능한 항목
</h2>

Anthropic 호스팅 환경에서 각 세션은 자신의 운영 체제와 CPU 아키텍처에 관계없이 x86\_64에서 Ubuntu 24.04를 실행하는 새로운 가상 머신(VM)을 받으며, 리포지토리가 복제되고 일반적인 도구 체인이 사전 설치됩니다. 종속성이 사전 컴파일된 바이너리를 제공할 때(예: 네이티브 확장이 있는 Ruby gem 또는 사전 빌드된 Python 휠) x86\_64 Linux 빌드를 사용하여 VM과 일치합니다. 이 섹션은 Anthropic 호스팅 기본값, 기본 제공 GitHub 도구, [테스트 및 서비스 실행](#run-tests-start-services-and-add-packages) 방법 및 각 VM이 받는 [리소스 제한](#resource-limits)을 다룹니다.

<Note>
  조직이 [자체 호스팅 환경](/docs/ko/self-hosted-environments)으로 라우팅하는 세션은 자신의 러너에서 실행되며 러너 이미지가 제공하는 도구를 사용합니다.
</Note>

<h3 id="what-carries-over-from-your-setup">
  설정에서 전달되는 항목
</h3>

클라우드 세션은 리포지토리의 새로운 복제본에서 시작됩니다. 리포지토리에 커밋한 모든 항목을 사용할 수 있습니다. 자신의 머신에만 설치하거나 구성한 항목은 세션에서 사용할 수 없습니다. 조직의 정책은 [서버 관리 설정](/docs/ko/server-managed-settings)을 통해 별도로 도착합니다.

|                                                                                                                                        | 클라우드 세션에서 사용 가능                                    | 이유                                                                                                                                                                                                                                                                                                                                                                          |
| :------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 리포지토리의 `CLAUDE.md`                                                                                                                     | 예                                                  | 복제본의 일부                                                                                                                                                                                                                                                                                                                                                                     |
| 리포지토리의 `.claude/settings.json` 훅 및 권한 규칙                                                                                               | 예, 하나의 리포지토리가 있는 세션에서                              | 복제본의 일부입니다. 여러 리포지토리가 있는 세션([프로젝트](/docs/ko/claude-projects#what-threads-pick-up-from-your-repositories) 스레드 포함)은 복제본 위에서 시작되며 이를 읽지 않습니다                                                                                                                                                                                                                                        |
| 리포지토리의 `.mcp.json` MCP 서버                                                                                                              | 예, 하나의 리포지토리가 있는 세션에서                              | 복제본의 일부이며 세션의 작업 디렉토리에서 찾습니다                                                                                                                                                                                                                                                                                                                                                |
| 리포지토리의 `.claude/rules/`                                                                                                                | 예                                                  | 복제본의 일부                                                                                                                                                                                                                                                                                                                                                                     |
| 리포지토리의 `.claude/skills/`, `.claude/agents/`, `.claude/commands/`                                                                       | 예                                                  | 복제본의 일부                                                                                                                                                                                                                                                                                                                                                                     |
| 리포지토리의 `.claude/settings.json`에 선언된 플러그인 및 마켓플레이스                                                                                      | 아니오                                                | 클라우드 세션은 리포지토리가 [`enabledPlugins`](/docs/ko/settings-reference#enabledplugins) 아래에서 켜는 플러그인을 설치하지 않으며, [`extraKnownMarketplaces`](/docs/ko/settings-reference#extraknownmarketplaces) 아래에 나열하는 마켓플레이스의 플러그인도 포함됩니다                                                                                                                                                                    |
| 조직의 [서버 관리 설정](/docs/ko/server-managed-settings)                                                                                            | 예                                                  | 세션이 시작될 때 Anthropic의 서버에서 가져옵니다. 클라우드 세션에서 `availableModels`이 적용되는 방식은 [표면 범위](/docs/ko/model-config#surface-coverage)를 참조하세요. MDM 또는 관리 설정 파일을 통해 장치에 배포된 설정은 세션이 Anthropic 관리 VM에서 실행되기 때문에 적용되지 않습니다. [자체 호스팅 환경](/docs/ko/self-hosted-environments)에서 세션은 [Claude Code가 관리 소스를 결합하는 방식](/docs/ko/managed-settings#how-claude-code-combines-managed-sources)에 따라 러너 이미지의 관리 설정 파일도 읽습니다 |
| 사용자 `~/.claude/CLAUDE.md`                                                                                                              | 아니오                                                | 리포지토리가 아닌 머신에 있습니다                                                                                                                                                                                                                                                                                                                                                          |
| 사용자 `~/.claude/skills/`, `~/.claude/agents/`, `~/.claude/commands/`                                                                    | 아니오                                                | 리포지토리가 아닌 머신에 있습니다. 대신 리포지토리의 `.claude/` 디렉토리에 커밋합니다. 클라우드 세션은 claude.ai에서 활성화한 기술을 자동으로 로드합니다                                                                                                                                                                                                                                                                              |
| 사용자 설정에서만 활성화된 플러그인                                                                                                                    | 아니오                                                | 사용자 범위 `enabledPlugins`은 머신의 `~/.claude/settings.json`에 있습니다                                                                                                                                                                                                                                                                                                                |
| `claude mcp add`로 기본 로컬 범위 또는 사용자 범위에서 추가한 MCP 서버                                                                                      | 아니오                                                | 이는 리포지토리가 아닌 머신의 `~/.claude.json`에 씁니다. `claude mcp add --scope project`로 서버를 추가합니다. 이는 리포지토리의 [`.mcp.json`](/docs/ko/mcp#project-scope)에 쓰고 해당 파일을 커밋합니다. 하나의 리포지토리가 있는 세션이 이를 로드합니다                                                                                                                                                                                            |
| 리포지토리의 `.claude/settings.json` `env` 블록의 전송 변수(예: `NODE_EXTRA_CA_CERTS` 및 [mTLS 클라이언트 인증서 변수](/docs/ko/network-config#mtls-authentication)) | 아니오                                                | 호스팅 환경이 세션의 API 연결을 관리하므로 Claude Code는 이러한 키를 무시하고 각 무시된 키를 세션의 디버그 로그에 기록합니다                                                                                                                                                                                                                                                                                               |
| Claude가 호출하는 서비스의 API 키 및 토큰                                                                                                           | Pro 및 Max 플랜에서 [API 자격 증명](#add-api-credentials)으로 | 환경에 키를 한 번 추가하고 에이전트 프록시가 나열한 호스트에 대한 요청에 첨부합니다. 에이전트 프록시가 [첨부할 수 없는](#requests-that-never-get-the-credential) 키 또는 Team 또는 Enterprise 플랜의 모든 키는 환경 변수에 남아 있습니다                                                                                                                                                                                                             |
| AWS SSO와 같은 대화형 인증                                                                                                                     | 아니오                                                | 지원되지 않습니다. SSO는 클라우드 세션에서 실행할 수 없는 브라우저 기반 로그인이 필요합니다                                                                                                                                                                                                                                                                                                                       |

자신의 구성을 클라우드 세션에서 사용 가능하게 하려면 리포지토리에 커밋합니다.

환경을 사용하는 모든 사람이 환경 변수 및 설정 스크립트를 읽을 수 있습니다. 대화 상자의 **Environment variables** 아래의 참고 사항이 이를 말하고 비밀을 추가하지 않도록 경고합니다. Pro 및 Max 플랜에서 에이전트 프록시가 첨부할 수 있는 키에 대해 [API 자격 증명](#add-api-credentials)을 대신 사용합니다.

<h3 id="installed-tools">
  설치된 도구
</h3>

클라우드 세션에는 일반적인 언어 런타임, 빌드 도구 및 데이터베이스가 사전 설치되어 있습니다. 아래 표는 카테고리별로 포함된 항목을 요약합니다.

| 카테고리        | 포함됨                                                                   |
| :---------- | :-------------------------------------------------------------------- |
| **Python**  | pip, poetry, uv, black, mypy, pytest, ruff가 있는 Python 3.x             |
| **Node.js** | npm, yarn, pnpm, bun¹, eslint, prettier, chromedriver가 있는 20, 21 및 22 |
| **Ruby**    | gem, bundler, rbenv가 있는 3.1, 3.2, 3.3                                 |
| **PHP**     | Composer가 있는 8.3                                                      |
| **Java**    | Maven 및 Gradle이 있는 OpenJDK 21                                         |
| **Go**      | 모듈 지원이 있는 Go                                                          |
| **Rust**    | rustc 및 cargo                                                         |
| **C/C++**   | GCC, Clang, cmake, ninja, conan                                       |
| **Docker**  | docker, dockerd, docker compose                                       |
| **데이터베이스**  | PostgreSQL 16, Redis 7.0                                              |
| **유틸리티**    | git, gh, jq, yq, ripgrep, tmux, vim, nano                             |

¹ Bun이 설치되어 있지만 패키지 페칭에 대해 알려진 [프록시 호환성 문제](#install-dependencies-with-a-sessionstart-hook)가 있습니다.

이 표의 대부분의 도구 버전을 얻으려면 Claude에게 클라우드 세션에서 `check-tools`를 실행하도록 요청합니다. 이는 슬래시 명령이 아닌 세션 VM에 설치된 셸 명령입니다. [Claude가 모든 VM 명령을 실행](#run-tests-start-services-and-add-packages)하기 때문에 요청합니다. 이를 보고하지 않는 도구(예: Ruby, PHP, bun, PostgreSQL 또는 Redis)의 경우 Claude에게 도구의 자체 버전 명령을 실행하도록 요청합니다(예: `psql --version`).

Node.js 버전은 `/opt/node20`, `/opt/node21` 및 `/opt/node22`에 설치되며, 기본적으로 22가 `PATH`에 있습니다. 다른 버전으로 작업하려면 Claude에게 해당 버전의 `bin` 디렉토리(예: `/opt/node20/bin`)를 `PATH`에 앞에 추가하도록 요청합니다.

.NET SDK와 같은 이 목록 외의 도구 체인은 패키지 레지스트리가 [기본 허용 목록](#default-allowed-domains)에 있더라도 사전 설치되지 않습니다. [설정 스크립트](#setup-scripts)로 설치합니다.

<h3 id="work-with-github-issues-and-pull-requests">
  GitHub 이슈 및 풀 요청 작업
</h3>

클라우드 세션에는 Claude가 이슈를 읽고, 풀 요청을 나열하고, 차이를 가져오고, 설정 없이 댓글을 게시할 수 있는 기본 제공 GitHub 도구가 포함됩니다. 이러한 도구는 [GitHub 프록시](#github-proxy)를 통해 인증하며, [GitHub 인증 옵션](/docs/ko/claude-code-on-the-web#github-authentication-options) 아래에서 구성한 방법을 사용하므로 토큰이 컨테이너에 들어가지 않습니다.

[환경 설정](#set-environment-variables)에서 `GH_TOKEN` 또는 `GITHUB_TOKEN`을 직접 설정하거나 둘 다 설정하지 않고 [GitHub 프록시](#github-proxy)가 인증을 처리하도록 할 수 있습니다:

* 토큰을 설정하면 컨테이너에 변경되지 않고 전달되므로 스크립트 및 GitHub의 [`gh` CLI](https://cli.github.com)가 직접 사용합니다.
* 둘 다 설정하지 않고 [GitHub 프록시](#github-proxy)가 세션에 대한 인증을 처리하는 경우 두 변수 모두 Claude가 실행하는 명령에서 자리 표시자 문자열 `proxy-injected`로 읽으며, 프록시는 아웃바운드 GitHub 요청에서 실제 자격 증명을 대체합니다. `gh`는 자신의 토큰 없이 작동하지만 `GITHUB_TOKEN`을 직접 읽는 스크립트는 사용 가능한 토큰이 아닌 자리 표시자를 받습니다.

설정한 토큰은 일반 환경 변수이므로 환경을 사용하는 모든 사람이 읽을 수 있습니다. 프록시 경로는 자격 증명을 환경 구성 및 세션 VM 외부에 유지합니다.

세션에 어느 경우가 적용되는지 확인하려면 Claude에게 `echo $GH_TOKEN`을 실행하도록 요청합니다.

GitHub의 [`gh` CLI](https://cli.github.com)는 사전 설치됩니다. 기본 제공 도구가 다루지 않는 `gh release` 또는 `gh workflow run`과 같은 `gh` 명령이 필요한 경우 Claude에게 실행하도록 요청합니다. `gh`는 `GH_TOKEN`을 자동으로 읽으므로 `gh auth login`을 실행할 필요가 없습니다.

<h3 id="link-output-back-to-the-session">
  출력을 세션에 다시 연결
</h3>

각 클라우드 세션에는 claude.ai의 트랜스크립트 URL이 있으며, 세션은 `CLAUDE_CODE_REMOTE_SESSION_ID` 환경 변수에서 자신의 ID를 읽을 수 있습니다. 이를 사용하여 PR 본문, 커밋 메시지, Slack 게시물 또는 생성된 보고서에 추적 가능한 링크를 넣어 검토자가 이를 생성한 실행을 열 수 있습니다.

Claude가 클라우드 세션에서 생성하는 커밋에는 `Claude-Session: <url>` git 트레일러가 포함되고 PR 본문에는 세션 URL이 자체 줄에 포함됩니다. 트레일러 및 PR 본문 링크를 생략하려면 [`attribution.sessionUrl`](/docs/ko/settings-reference#attribution-sessionurl)을 `false`로 설정합니다.

Claude가 게시하는 Slack 메시지 또는 작성하는 보고서 파일과 같이 커밋 또는 PR 이외의 항목에 세션 링크를 포함하려면 Claude에게 다음 명령을 실행하도록 하고 출력을 사용합니다. 명령은 환경 변수의 값에서 `cse_` 접두사를 트랜스크립트 URL이 예상하는 `session_` 접두사로 변환합니다:

```bash theme={null}
echo "https://claude.ai/code/${CLAUDE_CODE_REMOTE_SESSION_ID/#cse_/session_}"
```

<h3 id="run-tests-start-services-and-add-packages">
  테스트 실행, 서비스 시작 및 패키지 추가
</h3>

세션 VM에 셸이 없습니다. Claude가 모든 명령을 실행하므로 이 섹션의 작업을 프롬프트의 요청으로 표현합니다.

<h4 id="run-tests">
  테스트 실행
</h4>

Claude는 작업을 수행하는 과정에서 테스트를 실행합니다. 프롬프트에서 "fix the failing tests in `tests/`" 또는 "run pytest after each change"와 같이 요청합니다. pytest 및 cargo test와 같은 [사전 설치된 도구 체인](#installed-tools)과 함께 제공되는 테스트 러너는 추가 설정 없이 작동합니다. jest와 같이 프로젝트가 종속성으로 선언하는 러너는 종속성과 함께 설치됩니다.

<h4 id="start-services">
  서비스 시작
</h4>

PostgreSQL 및 Redis는 사전 설치되어 있지만 기본적으로 실행되지 않습니다. 필요한 것을 시작하도록 Claude에게 요청합니다. 실행하는 명령은:

```bash theme={null}
service postgresql start
```

```bash theme={null}
service redis-server start
```

Docker는 컨테이너화된 서비스를 실행하는 데 사용할 수 있습니다. Claude에게 `docker compose up`을 실행하여 프로젝트의 서비스를 시작하도록 요청합니다. 이미지를 가져오기 위한 네트워크 액세스는 환경의 [액세스 수준](#access-levels)을 따르며, [Trusted 기본값](#default-allowed-domains)에는 Docker Hub 및 기타 일반적인 레지스트리가 포함됩니다.

이미지가 크거나 가져오기가 느린 경우 [설정 스크립트](#setup-scripts)에 `docker compose pull` 또는 `docker compose build`를 추가합니다. [환경 캐시](#environment-caching)는 가져온 이미지를 유지하므로 각 새 세션에는 디스크에 이미지가 있습니다. 캐시는 파일만 저장하고 실행 중인 프로세스는 저장하지 않으므로 Claude는 여전히 각 세션에서 컨테이너를 시작합니다.

<h4 id="add-packages">
  패키지 추가
</h4>

사전 설치되지 않은 패키지를 추가하려면 [설정 스크립트](#setup-scripts)를 사용합니다. [환경 캐시](#environment-caching)는 스크립트가 설치하는 항목을 유지하므로 거기에 설치한 패키지는 매번 다시 설치하지 않고 모든 세션의 시작 시 사용 가능합니다. 또한 Claude에게 세션 중간에 패키지를 설치하도록 요청할 수 있지만 이러한 설치는 다른 세션으로 이월되지 않습니다.

<h3 id="resource-limits">
  리소스 제한
</h3>

Anthropic 호스팅 환경의 클라우드 세션은 시간이 지남에 따라 변할 수 있는 대략적인 리소스 상한선으로 실행됩니다:

* 4개의 vCPU
* 16GB의 RAM
* 30GB의 디스크

VM은 대규모 빌드 작업 또는 메모리 집약적인 테스트와 같이 훨씬 더 많은 메모리가 필요한 작업을 중지할 수 있습니다. 이러한 제한을 초과하는 워크로드의 경우 [Remote Control](/docs/ko/remote-control)을 사용하여 자신의 하드웨어에서 Claude Code를 실행하거나 조직이 운영하는 컴퓨팅에서 [자체 호스팅 환경](/docs/ko/self-hosted-environments)에서 클라우드 세션을 실행합니다.

<h2 id="setup-scripts">
  설정 스크립트
</h2>

설정 스크립트는 새로운 클라우드 세션이 시작될 때 Claude Code가 시작되기 전에 실행되는 Bash 스크립트입니다. 설정 스크립트를 사용하여 종속성을 설치하고, 도구를 구성하거나, 세션이 필요하지만 사전 설치되지 않은 항목을 가져옵니다.

스크립트는 Ubuntu 24.04에서 루트로 실행되므로 `apt install` 및 대부분의 언어 패키지 관리자가 작동합니다.

설정 스크립트를 추가하려면 환경 설정 대화 상자를 열고 **Setup script** 필드에 스크립트를 입력합니다.

이 예제는 사전 설치되지 않은 [ShellCheck](https://www.shellcheck.net/)를 설치합니다.

```bash theme={null}
#!/bin/bash
apt update && apt install -y shellcheck
```

<h3 id="script-requirements">
  스크립트 요구 사항
</h3>

설정 스크립트에는 작업할 세 가지 제약이 있습니다:

* **0으로 종료**: 스크립트가 0이 아닌 값으로 종료되면 세션이 시작되지 않습니다. 간헐적인 설치 실패가 세션을 차단하지 않도록 중요하지 않은 명령에 `|| true`를 추가합니다.
* **5분 이내에 완료**: [환경 캐시](#environment-caching)를 빌드할 수 있도록 스크립트의 총 런타임을 대략 5분 이내로 유지합니다. `&` 및 `wait`로 독립적인 설치를 병렬로 실행하고 맞지 않는 단일 다운로드를 [SessionStart 훅](#setup-scripts-vs-sessionstart-hooks)으로 이동하여 백그라운드에서 시작합니다.
* **설치를 위한 네트워크 액세스**: 패키지 설치는 레지스트리에 도달해야 합니다. 기본 **Trusted** 수준은 npm, PyPI, RubyGems 및 crates.io를 포함한 [일반적인 패키지 레지스트리](#default-allowed-domains)를 다룹니다. **None** 네트워크 액세스를 사용하면 설치가 실패합니다.

<h3 id="environment-caching">
  환경 캐싱
</h3>

설정 스크립트는 환경에서 세션을 처음 시작할 때 실행됩니다. 완료 후 Anthropic은 파일 시스템을 스냅샷하고 해당 스냅샷을 나중 세션의 시작점으로 재사용합니다. 새 세션은 종속성, 도구 및 Docker 이미지가 이미 디스크에 있는 상태로 시작되며 설정 스크립트 단계를 건너뜁니다. 이는 스크립트가 대규모 도구 체인을 설치하거나 컨테이너 이미지를 가져올 때도 시작을 빠르게 유지합니다.

캐시는 파일 시스템 스냅샷이므로 설정 스크립트가 디스크에 쓰는 항목을 유지하고 실행 중이던 항목은 손실합니다. 설치한 패키지, 가져온 Docker 이미지 및 작성한 파일은 모두 이월됩니다. 스크립트가 시작한 데이터베이스, `docker compose up` 스택 또는 기타 백그라운드 프로세스는 그렇지 않습니다. [SessionStart 훅](#setup-scripts-vs-sessionstart-hooks)으로 Claude에게 요청하거나 세션별로 시작합니다.

설정 스크립트는 환경의 설정 스크립트 또는 허용된 네트워크 호스트를 변경할 때 캐시를 다시 빌드하기 위해 다시 실행되며, 캐시가 대략 7일 후 만료에 도달할 때 실행됩니다. 기존 세션을 재개하면 설정 스크립트가 다시 실행되지 않습니다.

캐싱을 활성화하거나 스냅샷을 직접 관리할 필요가 없습니다.

<h3 id="setup-scripts-vs-sessionstart-hooks">
  설정 스크립트 대 SessionStart 훅
</h3>

설정 스크립트를 사용하여 VM 자체를 프로비저닝합니다: [사전 설치되지 않은](#installed-tools) 도구 체인 및 CLI 도구. 클라우드 및 로컬과 같이 모든 곳에서 실행되어야 하는 프로젝트 설정에 [SessionStart 훅](/docs/ko/hooks#sessionstart)을 사용합니다(예: `npm install`).

설정 스크립트 및 SessionStart 훅은 클라우드 세션이 시작될 때 고정된 순서로 실행됩니다. 표는 구성 위치, 실행 시기 및 실행 위치를 비교합니다.

|           | 설정 스크립트                                                                                                                           | SessionStart 훅                                                                                                                                            |
| --------- | --------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **구성 위치** | [claude.ai/code](https://claude.ai/code)의 환경 대화 상자, 그리고 [공유 환경](#organization-shared-environments)의 **Cloud environments** 관리 페이지 | [설정 파일](/docs/ko/settings#where-settings-live)(예: 리포지토리의 `.claude/settings.json`). 클라우드 세션에 도달하는 파일은 [설정에서 전달되는 항목](#what-carries-over-from-your-setup)을 참조하세요 |
| **실행 시기** | Claude Code가 시작되기 전에 [캐시된 환경](#environment-caching)이 없을 때만                                                                        | Claude Code가 시작된 후 재개된 세션을 포함한 모든 세션에서                                                                                                                    |
| **실행 위치** | 클라우드 세션만                                                                                                                          | 로컬 및 클라우드 세션                                                                                                                                              |

사용자 수준 `~/.claude/settings.json`에 SessionStart 훅이 있는 경우 클라우드에서 이를 기대하지 마세요: 사용자 수준 설정은 머신에 남아 있습니다. 어느 다른 훅이 실행되는지는 세션이 실행되는 위치에 따라 다릅니다:

* **Anthropic 호스팅 환경**: Claude Code는 리포지토리 및 조직의 [서버 관리 설정](/docs/ko/server-managed-settings)에서 훅을 실행합니다.
* **[자체 호스팅 환경](/docs/ko/self-hosted-environments-configuration#permissions-and-tool-approval)**: Claude Code는 또한 러너 호스트의 `~/.claude/`에서 운영자가 시드한 훅과 [Claude Code가 적용하는 관리 소스](/docs/ko/managed-settings#how-claude-code-combines-managed-sources) 중 하나인 경우 러너 이미지의 관리 설정 파일의 훅을 실행합니다.

<h3 id="install-dependencies-with-a-sessionstart-hook">
  SessionStart 훅으로 종속성 설치
</h3>

클라우드 세션에서만 종속성을 설치하려면 SessionStart 훅을 실행 중인 위치를 확인하는 스크립트와 쌍으로 만듭니다.

먼저 리포지토리의 `.claude/settings.json`에 SessionStart 훅을 추가합니다. 이 구성은 Claude Code에게 세션이 시작되거나 재개될 때마다 리포지토리에서 `scripts/install_pkgs.sh`를 실행하도록 지시합니다:

```json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "startup|resume",
        "hooks": [
          {
            "type": "command",
            "command": "bash \"$CLAUDE_PROJECT_DIR\"/scripts/install_pkgs.sh"
          }
        ]
      }
    ]
  }
}
```

`matcher`는 훅을 `startup` 및 `resume` 이벤트로 제한하고 `$CLAUDE_PROJECT_DIR`은 리포지토리 루트로 확인되므로 훅은 세션의 작업 디렉토리와 관계없이 스크립트를 찾습니다.

다음으로 `scripts/install_pkgs.sh`에서 스크립트를 생성합니다. 클라우드 외부에서 즉시 종료한 다음 종속성을 설치합니다:

```bash theme={null}
#!/bin/bash

if [ "$CLAUDE_CODE_REMOTE" != "true" ]; then
  exit 0
fi

npm install
pip install -r requirements.txt
exit 0
```

`CLAUDE_CODE_REMOTE` 확인은 설치를 클라우드 세션으로 범위 지정하는 것입니다: 세션 VM의 환경은 해당 변수를 `true`로 전달하고, 로컬에서는 절대 `true`가 아니므로 노트북에서 스크립트는 설치 전에 종료됩니다.

함께 두 파일은 모든 클라우드 세션에 시작 시 새로운 `npm install` 및 `pip install`을 제공하면서 로컬 세션은 건드리지 않습니다.

<h4 id="limitations-in-cloud-sessions">
  클라우드 세션의 제한 사항
</h4>

SessionStart 훅은 다음 주의 사항을 제외하고 클라우드에서 로컬과 동일하게 작동합니다:

* **한 세션당 하나의 리포지토리**: 여러 리포지토리가 있는 세션은 리포지토리의 `.claude/settings.json`에서 훅을 로드하지 않으므로 거기에 정의한 SessionStart 훅이 실행되지 않습니다. 이러한 세션의 종속성을 [설정 스크립트](#setup-scripts)로 설치합니다.
* **클라우드 전용 범위 없음**: 훅은 로컬 및 클라우드 세션 모두에서 실행됩니다. 로컬 실행을 건너뛰려면 `CLAUDE_CODE_REMOTE` 환경 변수가 `true`가 아닌 경우 조기에 종료합니다. [종속성 설치 스크립트](#install-dependencies-with-a-sessionstart-hook)가 수행하는 방식입니다.
* **네트워크 액세스 필요**: 설치 명령은 패키지 레지스트리에 도달해야 합니다. 환경이 **None** 네트워크 액세스를 사용하면 이러한 훅이 실패합니다. **Trusted** 아래의 [기본 허용 목록](#default-allowed-domains)은 npm, PyPI, RubyGems 및 crates.io를 다룹니다.
* **프록시 호환성**: Anthropic 호스팅 환경에서 모든 아웃바운드 트래픽은 [보안 프록시](#security-proxy)를 통과하고, 일부 패키지 관리자는 이 프록시에서 올바르게 작동하지 않습니다. Bun은 알려진 예입니다. [자체 호스팅 환경](/docs/ko/self-hosted-environments-deploy#default-deny-egress)에서 아웃바운드 트래픽은 자신의 네트워크 경계를 통해 나갑니다.
* **시작 지연 추가**: 훅은 [환경 캐싱](#environment-caching)의 이점을 누리는 설정 스크립트와 달리 세션이 시작되거나 재개될 때마다 실행됩니다. 다시 설치하기 전에 종속성이 이미 있는지 확인하여 설치 스크립트를 빠르게 유지합니다.

기본 이미지를 사용자 정의하려면 설정 스크립트를 사용하여 [제공된 이미지](#installed-tools) 위에 필요한 항목을 설치하거나 `docker compose`로 Claude와 함께 자신의 이미지를 컨테이너로 실행합니다. 기본 이미지를 완전히 교체하는 것은 아직 지원되지 않습니다.

<h2 id="default-allowed-domains">
  기본 허용 도메인
</h2>

**Trusted** 네트워크 액세스를 사용하면 세션은 기본적으로 다음 도메인에 도달할 수 있습니다. `*`로 표시된 도메인은 와일드카드 하위 도메인 일치를 나타내므로 `*.gcr.io`는 `gcr.io`의 모든 하위 도메인을 허용합니다.

<AccordionGroup>
  <Accordion title="Anthropic 서비스">
    * api.anthropic.com
    * docs.claude.com
    * platform.claude.com
    * code.claude.com
    * claude.ai
  </Accordion>

  <Accordion title="버전 제어">
    * github.com
    * [www.github.com](http://www.github.com)
    * api.github.com
    * npm.pkg.github.com
    * raw\.githubusercontent.com
    * pkg-npm.githubusercontent.com
    * objects.githubusercontent.com
    * release-assets.githubusercontent.com
    * codeload.github.com
    * avatars.githubusercontent.com
    * camo.githubusercontent.com
    * gist.github.com
    * gitlab.com
    * [www.gitlab.com](http://www.gitlab.com)
    * registry.gitlab.com
    * bitbucket.org
    * [www.bitbucket.org](http://www.bitbucket.org)
    * api.bitbucket.org
  </Accordion>

  <Accordion title="컨테이너 레지스트리">
    * registry-1.docker.io
    * auth.docker.io
    * index.docker.io
    * hub.docker.com
    * [www.docker.com](http://www.docker.com)
    * production.cloudflare.docker.com
    * download.docker.com
    * gcr.io
    * \*.gcr.io
    * ghcr.io
    * mcr.microsoft.com
    * \*.data.mcr.microsoft.com
    * public.ecr.aws
  </Accordion>

  <Accordion title="클라우드 플랫폼">
    * cloud.google.com
    * accounts.google.com
    * gcloud.google.com
    * \*.googleapis.com
    * storage.googleapis.com
    * compute.googleapis.com
    * container.googleapis.com
    * azure.com
    * portal.azure.com
    * microsoft.com
    * [www.microsoft.com](http://www.microsoft.com)
    * \*.microsoftonline.com
    * packages.microsoft.com
    * dotnet.microsoft.com
    * dot.net
    * visualstudio.com
    * dev.azure.com
    * \*.amazonaws.com
    * \*.api.aws
    * oracle.com
    * [www.oracle.com](http://www.oracle.com)
    * java.com
    * [www.java.com](http://www.java.com)
    * java.net
    * [www.java.net](http://www.java.net)
    * download.oracle.com
    * yum.oracle.com
    * \*.r2.cloudflarestorage.com
  </Accordion>

  <Accordion title="JavaScript 및 Node 패키지 관리자">
    * registry.npmjs.org
    * [www.npmjs.com](http://www.npmjs.com)
    * [www.npmjs.org](http://www.npmjs.org)
    * npmjs.com
    * npmjs.org
    * yarnpkg.com
    * registry.yarnpkg.com
    * jsr.io
    * npm.jsr.io
  </Accordion>

  <Accordion title="Python 패키지 관리자">
    * pypi.org
    * [www.pypi.org](http://www.pypi.org)
    * files.pythonhosted.org
    * pythonhosted.org
    * test.pypi.org
    * pypi.python.org
    * pypa.io
    * [www.pypa.io](http://www.pypa.io)
  </Accordion>

  <Accordion title="Ruby 패키지 관리자">
    * rubygems.org
    * [www.rubygems.org](http://www.rubygems.org)
    * api.rubygems.org
    * index.rubygems.org
    * ruby-lang.org
    * [www.ruby-lang.org](http://www.ruby-lang.org)
    * rubyforge.org
    * [www.rubyforge.org](http://www.rubyforge.org)
    * rubyonrails.org
    * [www.rubyonrails.org](http://www.rubyonrails.org)
    * rvm.io
    * get.rvm.io
  </Accordion>

  <Accordion title="Rust 패키지 관리자">
    * crates.io
    * [www.crates.io](http://www.crates.io)
    * index.crates.io
    * static.crates.io
    * rustup.rs
    * static.rust-lang.org
    * [www.rust-lang.org](http://www.rust-lang.org)
  </Accordion>

  <Accordion title="Go 패키지 관리자">
    * proxy.golang.org
    * sum.golang.org
    * index.golang.org
    * golang.org
    * [www.golang.org](http://www.golang.org)
    * goproxy.io
    * pkg.go.dev
  </Accordion>

  <Accordion title="JVM 패키지 관리자">
    * maven.org
    * repo.maven.org
    * central.maven.org
    * repo1.maven.org
    * repo.maven.apache.org
    * maven.google.com
    * jcenter.bintray.com
    * gradle.org
    * [www.gradle.org](http://www.gradle.org)
    * services.gradle.org
    * plugins.gradle.org
    * plugins-artifacts.gradle.org
    * kotlinlang.org
    * [www.kotlinlang.org](http://www.kotlinlang.org)
    * spring.io
    * repo.spring.io
  </Accordion>

  <Accordion title="기타 패키지 관리자">
    * packagist.org (PHP Composer)
    * [www.packagist.org](http://www.packagist.org)
    * repo.packagist.org
    * nuget.org (.NET NuGet)
    * [www.nuget.org](http://www.nuget.org)
    * api.nuget.org
    * pub.dev (Dart/Flutter)
    * api.pub.dev
    * hex.pm (Elixir/Erlang)
    * [www.hex.pm](http://www.hex.pm)
    * cpan.org (Perl CPAN)
    * [www.cpan.org](http://www.cpan.org)
    * metacpan.org
    * [www.metacpan.org](http://www.metacpan.org)
    * api.metacpan.org
    * cocoapods.org (iOS/macOS)
    * [www.cocoapods.org](http://www.cocoapods.org)
    * cdn.cocoapods.org
    * haskell.org
    * [www.haskell.org](http://www.haskell.org)
    * hackage.haskell.org
    * swift.org
    * [www.swift.org](http://www.swift.org)
  </Accordion>

  <Accordion title="Linux 배포판">
    * archive.ubuntu.com
    * security.ubuntu.com
    * ubuntu.com
    * [www.ubuntu.com](http://www.ubuntu.com)
    * \*.ubuntu.com
    * ppa.launchpad.net
    * launchpad.net
    * [www.launchpad.net](http://www.launchpad.net)
    * \*.nixos.org
  </Accordion>

  <Accordion title="개발 도구 및 플랫폼">
    * dl.k8s.io (Kubernetes)
    * pkgs.k8s.io
    * k8s.io
    * [www.k8s.io](http://www.k8s.io)
    * releases.hashicorp.com (HashiCorp)
    * apt.releases.hashicorp.com
    * rpm.releases.hashicorp.com
    * archive.releases.hashicorp.com
    * hashicorp.com
    * [www.hashicorp.com](http://www.hashicorp.com)
    * repo.anaconda.com (Anaconda/Conda)
    * conda.anaconda.org
    * anaconda.org
    * [www.anaconda.com](http://www.anaconda.com)
    * anaconda.com
    * continuum.io
    * apache.org (Apache)
    * [www.apache.org](http://www.apache.org)
    * archive.apache.org
    * downloads.apache.org
    * eclipse.org (Eclipse)
    * [www.eclipse.org](http://www.eclipse.org)
    * download.eclipse.org
    * nodejs.org (Node.js)
    * [www.nodejs.org](http://www.nodejs.org)
    * developer.apple.com
    * developer.android.com
    * pkg.stainless.com
    * binaries.prisma.sh
  </Accordion>

  <Accordion title="클라우드 서비스 및 모니터링">
    * http-intake.logs.datadoghq.com
    * \*.datadoghq.com
    * \*.datadoghq.eu
    * api.honeycomb.io
  </Accordion>

  <Accordion title="콘텐츠 전달 및 미러">
    * sourceforge.net
    * \*.sourceforge.net
    * packagecloud.io
    * \*.packagecloud.io
    * fonts.googleapis.com
    * fonts.gstatic.com
  </Accordion>

  <Accordion title="스키마 및 구성">
    * json-schema.org
    * [www.json-schema.org](http://www.json-schema.org)
    * json.schemastore.org
    * [www.schemastore.org](http://www.schemastore.org)
  </Accordion>

  <Accordion title="Model Context Protocol">
    * \*.modelcontextprotocol.io
  </Accordion>
</AccordionGroup>

<h2 id="related-resources">
  관련 리소스
</h2>

* [클라우드 세션 참조](/docs/ko/claude-code-on-the-web): 클라우드 세션 시작, 관리 및 공유
* [클라우드 세션 빠른 시작](/docs/ko/web-quickstart): GitHub 연결 및 첫 번째 클라우드 세션 시작
* [Claude Tag](https://claude.com/docs/claude-tag/overview): Claude가 Slack에서 시작하는 세션은 동일한 환경에서 실행됩니다
* [루틴](/docs/ko/routines): 예약된 실행은 동일한 환경 및 네트워크 액세스 수준을 사용합니다
* [Remote Control](/docs/ko/remote-control): 자신의 머신의 네트워크 및 파일에서 세션 실행
* [자체 호스팅 환경](/docs/ko/self-hosted-environments): 조직의 자체 인프라에서 클라우드 세션 실행
* [SessionStart 훅](/docs/ko/hooks#sessionstart): 로컬 및 클라우드 세션에서 실행되는 리포지토리 커밋 설정
* [서버 관리 설정](/docs/ko/server-managed-settings): 클라우드 세션에 도달하는 조직 정책
