> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 인증

> Claude Code에 로그인하고 개인, 팀, 조직을 위한 인증을 구성합니다.

Claude Code는 설정에 따라 여러 인증 방법을 지원합니다. 개별 사용자는 claude.ai 계정으로 로그인할 수 있으며, 팀은 Claude for Teams 또는 Enterprise, Claude Console, 또는 Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry와 같은 클라우드 제공자를 사용할 수 있습니다.

<h2 id="log-in-to-claude-code">
  Claude Code에 로그인
</h2>

[Claude Code를 설치](/docs/ko/setup#install-claude-code)한 후 터미널에서 `claude`를 실행합니다. 처음 실행할 때 Claude Code는 로그인할 수 있도록 브라우저 창을 엽니다. `ANTHROPIC_API_KEY` 환경 변수를 설정한 경우 Claude Code는 로그인 프롬프트를 건너뛰고 대신 키를 승인하도록 요청합니다.

브라우저가 자동으로 열리지 않으면 `c`를 눌러 로그인 URL을 클립보드에 복사한 후 브라우저에 붙여넣습니다.

브라우저에서 로그인 후 리디렉션 대신 로그인 코드를 표시하면 터미널의 `Paste code here if prompted` 프롬프트에 붙여넣습니다. 이는 브라우저가 Claude Code의 로컬 콜백 서버에 도달할 수 없을 때 발생하며, WSL2, SSH 세션 및 컨테이너에서 일반적입니다.

로그인이 완료되면 터미널에 `Login successful`이 표시되고 `Enter`를 눌러 계속하라는 메시지가 나타납니다.

다음 계정 유형 중 하나로 인증할 수 있습니다:

* **Claude Pro 또는 Max 구독**: claude.ai 계정으로 로그인합니다. [claude.com/pricing](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_pro_max)에서 구독합니다.
* **Claude for Teams 또는 Enterprise**: 팀 관리자가 초대한 claude.ai 계정으로 로그인합니다.
* **Claude Console**: Console 자격증명으로 로그인합니다. 관리자가 먼저 [초대](#claude-console-authentication)해야 합니다. [API 키를 생성](#sign-in-without-an-api-key)하거나 생성하지 않고 로그인할 수 있습니다.
* **클라우드 제공자**: 조직에서 [Amazon Bedrock](/docs/ko/amazon-bedrock), [Google Cloud의 Agent Platform](/docs/ko/google-vertex-ai), 또는 [Microsoft Foundry](/docs/ko/microsoft-foundry)를 사용하는 경우 `claude`를 실행하기 전에 필요한 환경 변수를 설정하거나 로그인 프롬프트에서 **3rd-party platform**을 선택합니다. 이는 Bedrock 및 Vertex AI에 대한 대화형 설정 마법사를 시작합니다. 브라우저 로그인이 필요하지 않습니다.
* **클라우드 게이트웨이**: 조직에서 자체 호스팅 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway)를 실행하는 경우 `/login`을 통해 회사 SSO로 로그인합니다. 게이트웨이에서 발급한 토큰이 세션의 유일한 자격증명입니다.

관리자는 개발자가 사용하는 로그인 방법을 지정할 수 있으며 claude.ai 로그인이 특정 조직에 속하도록 요구할 수 있습니다. [조직에 대한 로그인 제한](#restrict-login-to-your-organization)을 참조합니다.

로그아웃하고 다시 인증하려면 Claude Code 프롬프트에서 `/logout`을 입력합니다. 로그아웃하면 첫 실행 설정 상태도 재설정되므로 다음에 `claude`를 실행할 때 로그인 및 설정을 다시 진행합니다.

로그인에 문제가 있으면 [인증 문제 해결](/docs/ko/troubleshoot-install#login-and-authentication)을 참조합니다.

<h2 id="set-up-team-authentication">
  팀 인증 설정
</h2>

팀 및 조직의 경우 다음 방법 중 하나로 Claude Code 액세스를 구성할 수 있습니다.

* [Claude for Teams 또는 Enterprise](#claude-for-teams-or-enterprise), 대부분의 팀에 권장됨
* [Claude Console](#claude-console-authentication)
* [Claude apps gateway](/docs/ko/claude-apps-gateway), 개발자를 IdP로 로그인하고 구성한 클라우드 공급자로 추론을 라우팅하는 자체 호스팅 게이트웨이
* [Amazon Bedrock](/docs/ko/amazon-bedrock)
* [Google Cloud's Agent Platform](/docs/ko/google-vertex-ai)
* [Microsoft Foundry](/docs/ko/microsoft-foundry)

<h3 id="claude-for-teams-or-enterprise">
  Claude for Teams 또는 Enterprise
</h3>

[Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_teams#team-&-enterprise) 및 [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_enterprise)는 Claude Code를 사용하는 조직에 최고의 경험을 제공합니다. 팀 멤버는 중앙 집중식 청구 및 팀 관리를 통해 Claude Code와 웹의 Claude에 모두 액세스할 수 있습니다.

* **Claude for Teams**: 협업 기능, 관리 도구, SSO, 청구 관리 및 조직 전체 Claude Code 구성을 위한 [서버 관리 설정](/docs/ko/server-managed-settings)이 포함된 셀프 서비스 플랜입니다. 소규모 팀에 최적입니다.
* **Claude for Enterprise**: 도메인 캡처, 역할 기반 권한 및 규정 준수 API를 추가합니다. 보안 및 규정 준수 요구 사항이 있는 대규모 조직에 최적입니다.

<Steps>
  <Step title="구독">
    [Claude for Teams](https://claude.com/pricing?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_teams_step#team-&-enterprise)를 구독하거나 [Claude for Enterprise](https://anthropic.com/contact-sales?utm_source=claude_code\&utm_medium=docs\&utm_content=authentication_enterprise_step)에 대해 영업팀에 문의하세요.
  </Step>

  <Step title="팀 멤버 초대">
    관리자 대시보드에서 팀 멤버를 초대하세요.
  </Step>

  <Step title="설치 및 로그인">
    팀 멤버는 Claude Code를 설치하고 claude.ai 계정으로 로그인합니다.
  </Step>
</Steps>

<h3 id="claude-console-authentication">
  Claude Console 인증
</h3>

API 기반 청구를 선호하는 조직의 경우 Claude Console을 통해 액세스를 설정할 수 있습니다.

<Steps>
  <Step title="Console 계정 생성 또는 사용">
    기존 Claude Console 계정을 사용하거나 새 계정을 만드세요.
  </Step>

  <Step title="사용자 추가">
    다음 두 가지 방법 중 하나로 사용자를 추가할 수 있습니다.

    * Console 내에서 사용자를 일괄 초대: Settings -> Members -> Invite
    * [SSO 설정](https://support.claude.com/en/articles/13132885-setting-up-single-sign-on-sso)
  </Step>

  <Step title="역할 할당">
    사용자를 초대할 때 다음 중 하나를 할당하세요.

    * **Claude Code** 역할: 사용자는 Claude Code API 키만 생성할 수 있습니다.
    * **Developer** 역할: 사용자는 모든 종류의 API 키를 생성할 수 있습니다.
  </Step>

  <Step title="사용자가 설정 완료">
    초대된 각 사용자는 다음을 수행해야 합니다.

    * Console 초대 수락
    * [시스템 요구 사항 확인](/docs/ko/setup#system-requirements)
    * [Claude Code 설치](/docs/ko/setup#install-claude-code)
    * Console 계정 자격 증명으로 로그인
  </Step>
</Steps>

<h4 id="sign-in-without-an-api-key">
  API 키 없이 로그인
</h4>

조직에서 개발자가 API 키를 생성하도록 허용하지 않는 경우에도 API 키를 생성하지 않고 Console 계정에 로그인할 수 있습니다. `/login` 프롬프트에서 Anthropic Console 계정을 선택하면 Claude Code가 로그인 방법을 묻습니다. Claude Code v2.1.242 이상이 필요합니다. 두 경로 모두 브라우저에서 Console에 로그인하며 Claude Code가 이후에 저장하는 내용이 다릅니다.

* **Console 계정으로 로그인**, `(권장)` 레이블: Claude Code는 해당 로그인의 OAuth 토큰을 유지하고 [Anthropic 프로필](#anthropic-profiles-and-federation-credentials)로 저장합니다. API 키를 생성하지 않습니다.
* **API 키 생성**, `(레거시)` 레이블: Claude Code는 Console API 키를 생성하고 다른 자격 증명과 함께 저장합니다.

실제로 프로필은 OAuth 로그인을 저장하고 API 키는 정적 자격 증명입니다. Claude Code는 프로필의 로그인을 자동으로 새로 고치며, 새로 고침이 실패하면 다시 로그인할 때까지 [Anthropic 프로필 로그인 만료됨](/docs/ko/errors#anthropic-profile-login-expired)으로 요청이 실패합니다.

모든 머신에서 선택지를 얻지는 못합니다. Claude Code는 다음의 경우 묻지 않고 API 키를 생성합니다.

* [Amazon Bedrock, Google Cloud's Agent Platform 또는 Microsoft Foundry](/docs/ko/third-party-integrations) 또는 [Claude Platform on AWS](/docs/ko/claude-platform-on-aws)와 같은 클라우드 공급자에 대해 실행합니다.
* 모든 설정 파일이 [`forceLoginOrgUUID`](#restrict-login-to-your-organization)를 설정하거나 `forceLoginMethod`를 `"claudeai"` 또는 `"console"`로 설정합니다.
* 관리형 설정 파일, MDM 프로필 또는 캐시된 서버 관리 설정과 같은 머신의 관리형 설정 소스가 존재하지만 Claude Code가 [읽을 수 없으며](/docs/ko/managed-settings#invalid-entries-in-managed-settings) 다른 관리형 소스가 정책을 제공하지 않습니다.

키 없이 로그인하기 전에 `ANTHROPIC_API_KEY`를 설정 해제하세요. Claude Code의 자체 Console 로그인 또는 Claude Platform CLI의 `ant auth login`으로 작성된 프로필은 동일한 종류의 자격 증명이므로 다시 로그인하면 이를 바꿉니다.

키 없이 로그인한 후에는 저장된 API 키 대신 프로필이 있습니다.

* **작성하는 프로필**: Claude Code는 `ANTHROPIC_PROFILE`로 명명된 프로필, 활성 프로필 또는 `default`를 작성합니다. 해당 프로필이 페더레이션 프로필인 경우 Claude Code는 덮어쓰지 않고 로그인을 거부합니다.
* **로그아웃되는 대상**: Claude Code는 머신에 저장된 모든 claude.ai 로그인에서 로그아웃합니다.
* **실행 취소 방법**: `/logout`을 실행하면 이 로그인이 작성한 자격 증명이 제거되고 취소됩니다.

조직에서 [서버 관리 설정](/docs/ko/server-managed-settings)을 사용하는 경우 Claude Code v2.1.257 이상에서 이 로그인에 적용됩니다.

프로필에 대한 다른 모든 것이 이 로그인에 적용되며, 여기에는 다른 자격 증명에 대한 순위, `/status`에서 얻는 `Profile` 행 및 claude.ai 로그인이 필요한 기능이 포함됩니다. [Anthropic 프로필 및 페더레이션 자격 증명](#anthropic-profiles-and-federation-credentials)을 참조하세요.

<h3 id="cloud-provider-authentication">
  클라우드 공급자 인증
</h3>

Amazon Bedrock, Google Cloud's Agent Platform 또는 Microsoft Foundry를 사용하는 팀의 경우:

<Steps>
  <Step title="공급자 설정 따르기">
    [Amazon Bedrock 문서](/docs/ko/amazon-bedrock), [Google Cloud's Agent Platform 문서](/docs/ko/google-vertex-ai) 또는 [Microsoft Foundry 문서](/docs/ko/microsoft-foundry)를 따르세요.
  </Step>

  <Step title="구성 배포">
    환경 변수 및 클라우드 자격 증명 생성 지침을 사용자에게 배포하세요. [여기서 구성을 관리하는 방법](/docs/ko/settings)에 대해 자세히 알아보세요.
  </Step>

  <Step title="Claude Code 설치">
    사용자는 [Claude Code를 설치](/docs/ko/setup#install-claude-code)할 수 있습니다.
  </Step>
</Steps>

<h3 id="restrict-login-to-your-organization">
  조직에 대한 로그인 제한
</h3>

개발자의 claude.ai 로그인이 특정 Anthropic 조직에 속하도록 요구하려면 [관리형 설정](/docs/ko/managed-settings)에서 [`forceLoginMethod`](/docs/ko/settings-reference#forceloginmethod) 및 [`forceLoginOrgUUID`](/docs/ko/settings-reference#forceloginorguuid)를 설정하세요. `forceLoginOrgUUID`를 조직 ID로 설정하세요. 조직 ID는 Claude for Teams 또는 Enterprise 조직의 [claude.ai 관리 설정](https://claude.ai/admin-settings/organization)에 표시됩니다. Claude Code는 다른 조직에 대한 claude.ai 로그인에 대해 오류를 보고하고 사용 중인 claude.ai 자격 증명이 나열되지 않은 조직에 속하는 경우 시작 시 종료됩니다.

Claude Console 로그인의 경우 Claude Code는 `forceLoginOrgUUID`를 사용하여 단일 Console 조직 ID로 설정할 때 Console 로그인 페이지에서 조직을 미리 선택합니다. 조직 ID는 [platform.claude.com/settings/organization](https://platform.claude.com/settings/organization)에 표시됩니다. 로그인 시 또는 시작 시 결과 Console 자격 증명이 속한 조직을 확인하지 않으며, 키를 배포하기 전에 Console 계정으로 로그인한 개발자는 로그인 상태를 유지합니다.

모든 설정 파일에서 `forceLoginOrgUUID`를 설정하면 Claude Code는 해당 파일이 적용되는 세션에서 [키 없는 Console 로그인](#sign-in-without-an-api-key)을 제공하지 않고 대신 API 키를 생성합니다. 개발자를 claude.ai 로그인으로 유도하려면 `forceLoginMethod`를 `"claudeai"`로 설정하세요.

개발자는 여러 경로에서 로그인할 수 있습니다. 터미널 `/login` 흐름, [VS Code 확장](/docs/ko/vs-code), Agent SDK, `claude setup-token`, `/install-github-app` 및 클라우드 게이트웨이를 통해 라우팅하는 조직의 [게이트웨이](/docs/ko/claude-apps-gateway) 로그인입니다. Claude Code v2.1.212 이상에서는 모든 경로가 `forceLoginMethod`를 적용합니다. v2.1.212 이전에는 터미널 로그인만 두 키를 적용했습니다. 터미널의 대화형 로그인 화면에서 `/login` 또는 처음 실행 온보딩으로 도달하면 Claude Code는 `claudeai` 또는 `console` 방법을 미리 선택하지만 강제하지 않으므로 `forceLoginMethod`가 `"claudeai"`로 설정되어 있어도 개발자는 여전히 Console 로그인을 완료할 수 있습니다. 경로는 `forceLoginOrgUUID`에서 다릅니다.

* **터미널, VS Code 확장 및 Agent SDK 로그인**: claude.ai 계정 로그인에 대해 `forceLoginOrgUUID` 확인
* **`claude setup-token` 및 `/install-github-app`**: `forceLoginMethod`만 강제하므로 다른 조직에서 토큰을 발급할 수 있습니다.
* **[게이트웨이](/docs/ko/claude-apps-gateway) 로그인**: `forceLoginMethod: "gateway"`로 선택되며 제한되지 않으며 Anthropic 조직에 대해 인증하지 않으므로 `forceLoginOrgUUID`가 적용되지 않습니다. 게이트웨이 ID 공급자를 사용하여 액세스를 제한하세요.

장치 관리 도구를 통해 키를 배포하세요. [서버 관리 설정](/docs/ko/server-managed-settings)은 이미 조직에 인증된 계정에만 도달하므로 개발자의 첫 로그인을 리디렉션할 수 없습니다. 조직에서 서버 관리 설정도 배포하는 경우 두 위치 모두에 키를 설정하세요. 관리형 설정 소스는 [병합되지 않으며](/docs/ko/server-managed-settings#settings-precedence) 캐시된 서버 관리 설정은 몇 가지 [키별 예외](/docs/ko/server-managed-settings#per-key-exceptions-across-managed-sources)를 제외하고 장치 관리 파일을 바꿉니다. `forceLoginOrgUUID` 및 `forceLoginMethod`의 `"claudeai"` 및 `"console"` 값은 이러한 예외 중 하나가 아니므로 두 위치 모두에 유지하세요.

키는 또한 로그인 자격 증명을 사용하지 않는 세션이 시작될 수 있는지 여부를 결정합니다. 설정 참조에서 [`forceLoginOrgUUID`](/docs/ko/settings-reference#forceloginorguuid)를 참조하여 전체 동작을 확인하세요.

* **`ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN` 또는 `apiKeyHelper`**: 환경 자격 증명에 대해 조직 멤버십을 확인할 수 없으므로 시작 시 차단됩니다.
* **Amazon Bedrock, Google Cloud's Agent Platform 또는 Microsoft Foundry와 같은 클라우드 공급자 세션**: 클라우드 공급자에 대해 인증하므로 차단되지 않습니다. 클라우드 IAM 정책을 통해 이를 제한하세요.
* **[Anthropic 프로필 또는 페더레이션 자격 증명](#anthropic-profiles-and-federation-credentials)**: 차단되지 않으며 키는 프로필이 속한 조직을 확인하지 않습니다.

<h2 id="credential-management">
  자격증명 관리
</h2>

Claude Code는 인증 자격증명을 안전하게 관리합니다:

* **저장 위치**:
  * macOS에서 자격증명은 암호화된 macOS Keychain에 저장됩니다. Keychain이 SSH 세션에서 잠긴 경우와 같이 Keychain이 쓰기를 거부하면 Claude Code는 대신 로그인을 파일 모드 `0600`으로 `~/.claude/.credentials.json`에 저장합니다. 이는 Linux에서 사용하는 것과 동일한 저장소입니다. Keychain이 쓰기 가능할 때까지 Console 로그인이 API 키를 생성하지 못합니다. 로그인을 Keychain으로 다시 이동하려면 [복구 단계](/docs/ko/troubleshoot-install#not-logged-in-or-token-expired)를 따릅니다.
  * Linux에서 자격증명은 `~/.claude/.credentials.json`에 파일 모드 `0600`으로 저장됩니다.
  * Windows에서 자격증명은 `%USERPROFILE%\.claude\.credentials.json`에 저장되며 사용자 프로필 디렉터리의 액세스 제어를 상속하므로 기본적으로 파일이 사용자 계정으로 제한됩니다.
  * `CLAUDE_CONFIG_DIR` 환경 변수를 설정한 경우 Claude Code는 macOS 폴백이 쓰는 파일을 포함하여 `.credentials.json` 파일을 해당 디렉터리 아래에 유지하고, macOS Keychain 항목도 해당 디렉터리로 키를 지정하므로 다른 `CLAUDE_CONFIG_DIR`을 가진 세션은 다른 항목을 읽습니다.
  * Claude Code는 `/login` 및 `/logout`을 통해 `.credentials.json`을 관리합니다. 요청을 사용자 정의 API 엔드포인트를 통해 라우팅하려면 대신 [`ANTHROPIC_BASE_URL`](/docs/ko/env-vars) 환경 변수를 설정합니다.
* **지원되는 인증 유형**: claude.ai 자격증명, Claude API 자격증명, Microsoft Foundry Auth, Bedrock Auth, Vertex Auth, Anthropic 프로필 및 [Workload Identity Federation](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation) 자격증명, 그리고 [Claude apps gateway](/docs/ko/claude-apps-gateway) 세션 토큰.
* **사용자 정의 자격증명 스크립트**: [`apiKeyHelper`](/docs/ko/settings-reference#apikeyhelper) 설정을 구성하여 API 키를 반환하는 셸 스크립트를 실행합니다.
* **새로고침 간격**: Claude Code는 기본적으로 5분 후에 `apiKeyHelper`를 다시 실행합니다. 사용자 정의 새로고침 간격을 위해 `CLAUDE_CODE_API_KEY_HELPER_TTL_MS` 환경 변수를 설정합니다. Claude Code가 도우미를 다시 실행하는 다른 경우는 [`apiKeyHelper`](/docs/ko/settings-reference#apikeyhelper)를 참조합니다.
* **느린 도우미 알림**: `apiKeyHelper`가 키를 반환하는 데 10초 이상 걸리면 Claude Code는 경과 시간을 표시하는 프롬프트 표시줄에 경고 알림을 표시합니다. 이 알림이 정기적으로 표시되면 자격증명 스크립트를 최적화할 수 있는지 확인합니다.
* **도우미 실패**: 스크립트가 오류로 종료되거나 시간 초과되거나 아무것도 인쇄하지 않으면 요청은 3회 시도 내에 [`Your apiKeyHelper script is failing`](/docs/ko/errors#your-apikeyhelper-script-is-failing)으로 실패합니다. v2.1.208 이전에는 도우미 실패가 약 10번의 자동 재시도 후 일반 401로 표시되었습니다.

`apiKeyHelper`, `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`은 CLI 및 VS Code 확장 프로그램, Agent SDK, GitHub Actions를 포함하여 이를 래핑하는 표면에 적용됩니다. Claude Desktop 및 클라우드 세션은 `apiKeyHelper`를 호출하거나 이러한 환경 변수를 읽지 않습니다. 이들은 OAuth를 사용하며, [타사 추론 구성](/docs/ko/llm-gateway-connect#desktop-app)을 실행하는 데스크톱 세션은 해당 구성의 자격증명으로 인증합니다.

<h3 id="renew-an-expiring-login">
  만료되는 로그인 갱신
</h3>

`/login`으로 생성한 로그인이 만료되기 3일 이내일 때 Claude Code는 시작 시 경고를 표시합니다: `Your login expires in 3 days · run /login to renew`. Claude Code v2.1.203 이상이 필요합니다. v2.1.217 이전에는 경고가 5일 전에 나타났습니다.

`/login`을 실행하여 갱신합니다. 경고는 정보 제공용이며 요청을 차단하지 않습니다: 로그인이 실제로 만료될 때까지 인증이 계속 작동합니다. 로그인 수명 자체는 변경되지 않습니다. 사전 경고는 v2.1.203이 추가한 것입니다.

저장된 로그인이 만료되고 새로고칠 수 없으면 다시 로그인할 때까지 각 모델 요청은 [`Login expired · Please run /login`](/docs/ko/errors#login-expired)으로 실패합니다. v2.1.206 이전에는 Claude Code가 만료된 로그인을 모델 오류로 보고했습니다.

요청이 실패하기 전에 이 상태를 확인할 수 있습니다: [`/status`](/docs/ko/commands)는 `Login` 행을 표시하며 `Expired — log in again`을 읽고, 만료된 로그인에 대해 저장한 조직 및 이메일을 표시합니다. 행은 저장된 claude.ai 또는 Claude Console 로그인이 활성 자격증명일 때만 나타납니다. 행은 Claude Code v2.1.210 이상이 필요합니다.

경고는 claude.ai 또는 Claude Console 로그인이 활성 자격증명일 때만 나타나며, 클라우드 제공자, `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, 또는 `apiKeyHelper`가 자격증명을 제공할 때는 나타나지 않습니다.

조기 갱신은 무인으로 실행되는 세션에 가장 중요합니다. [에이전트 보기의 백그라운드 세션](/docs/ko/agent-view) 또는 로그인보다 오래 지속되는 [Remote Control](/docs/ko/remote-control) 세션은 자격증명이 만료되면 진행을 멈추고 다시 로그인할 때까지 복구할 수 없습니다.

<h3 id="authentication-precedence">
  인증 우선순위
</h3>

여러 자격증명이 있을 때 Claude Code는 다음 순서로 선택합니다:

1. `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX`, 또는 `CLAUDE_CODE_USE_FOUNDRY`가 설정된 경우 클라우드 제공자 자격증명. 설정은 [타사 통합](/docs/ko/third-party-integrations)을 참조합니다.
2. `ANTHROPIC_AUTH_TOKEN` 환경 변수. `Authorization: Bearer` 헤더로 전송됩니다. Anthropic API 키 대신 베어러 토큰으로 인증하는 [LLM 게이트웨이 또는 프록시](/docs/ko/llm-gateway)를 통해 라우팅할 때 사용합니다.
3. `ANTHROPIC_API_KEY` 환경 변수. `X-Api-Key` 헤더로 전송됩니다. [Claude Console](https://platform.claude.com)의 키를 사용하여 Anthropic API에 직접 액세스할 때 사용합니다. 대화형 모드에서는 키를 승인하거나 거부하도록 한 번 프롬프트되며 선택이 기억됩니다. 나중에 변경하려면 `/config`의 "사용자 정의 API 키 사용" 토글을 사용합니다. 토글은 `ANTHROPIC_API_KEY`가 환경에 설정되어 있는 동안에만 나타납니다. 비대화형 모드(`-p`)에서는 키가 있을 때 항상 사용됩니다.
4. [`apiKeyHelper`](/docs/ko/settings-reference#apikeyhelper) 스크립트 출력. 자격증명 모음에서 가져온 단기 토큰과 같은 동적 또는 회전 자격증명에 사용합니다.
5. `CLAUDE_CODE_OAUTH_TOKEN` 환경 변수. [`claude setup-token`](#generate-a-long-lived-token)으로 생성된 장기 OAuth 토큰입니다. 브라우저 로그인을 사용할 수 없는 CI 파이프라인 및 스크립트에 사용합니다. `/login`을 실행하는 동안 변수가 설정되어 있으면 Claude Code는 현재 세션을 새 로그인으로 전환하지만 변수를 셸 프로필에서 제거하거나 [설정 파일](/docs/ko/settings)의 `env` 블록에서 제거할 때까지 모든 새 세션에서 변수를 다시 읽습니다.
6. Anthropic 프로필 및 페더레이션 자격증명, `ant` CLI 및 Workload Identity Federation이 사용하는 자격증명입니다. `ant auth login`이 작성한 프로필은 `ANTHROPIC_PROFILE`에서 이름을 지정할 때만 여기에 순위가 지정되며, 그렇지 않으면 `/login` 아래에 순위가 지정됩니다. [Anthropic 프로필 및 페더레이션 자격증명](#anthropic-profiles-and-federation-credentials)을 참조합니다.
7. `/login`의 구독 OAuth 자격증명. Claude Pro, Max, Team, Enterprise 사용자의 기본값입니다.

서명된 [Claude apps gateway](/docs/ko/claude-apps-gateway) 세션은 이 목록 외에 있습니다: Amazon Bedrock 또는 Google Cloud의 Agent Platform과 같은 제공자 선택이며 이들보다 우선합니다. 게이트웨이 세션이 존재할 때 CLI는 `CLAUDE_CODE_USE_BEDROCK`, `CLAUDE_CODE_USE_VERTEX`, 또는 `CLAUDE_CODE_USE_FOUNDRY`가 설정되어 있어도 게이트웨이 토큰으로 인증하며, 베어러 토큰, API 키, `apiKeyHelper`, 프로필과 같은 위의 자격증명 소스는 사용되지 않습니다.

머신의 [관리 설정](/docs/ko/managed-settings)이 [`forceLoginMethod`](/docs/ko/settings-reference#forceloginmethod)를 `"gateway"`로 설정하거나 [`forceLoginGatewayUrl`](/docs/ko/settings-reference#forcelogingatewayurl)을 설정하고, `CLAUDE_CODE_USE_BEDROCK` 또는 `CLAUDE_CODE_USE_VERTEX`와 같은 변수를 통해 클라우드 제공자를 선택하지 않으면 세션은 게이트웨이 로그인만 사용합니다. Claude Code는 다른 자격증명 소스를 건너뛰고 `/login`으로 로그인하도록 요청합니다. 각 남은 자격증명으로 표시되는 내용은 [Administrator policy requires a Cloud gateway sign-in](/docs/ko/errors#administrator-policy-requires-a-cloud-gateway-sign-in)을 참조합니다. v2.1.261 이전이거나 `forceLoginGatewayUrl`만 설정하는 머신에서 v2.1.265 이전에는 Claude Code가 게이트웨이에 로그인할 때까지 이러한 머신에서 남은 저장된 로그인을 사용했습니다.

활성 Claude 구독이 있지만 환경에 `ANTHROPIC_API_KEY`도 설정되어 있으면 승인된 후 API 키가 우선합니다. 키가 비활성화되거나 만료된 조직에 속하면 인증 실패가 발생할 수 있습니다.

`unset ANTHROPIC_API_KEY`를 실행하여 구독으로 돌아가고 `/status`를 확인하여 활성 방법을 확인합니다. 로그인과 API 키가 모두 구성되어 있으면 `/status`는 사용 중이 아닌 자격증명을 표시합니다.

[Claude Code on the Web](/docs/ko/claude-code-on-the-web)은 항상 구독 자격증명을 사용합니다. 클라우드 환경에서 `ANTHROPIC_API_KEY` 또는 `ANTHROPIC_AUTH_TOKEN`을 설정하면 구독 자격증명을 재정의하지 않습니다.

<h4 id="anthropic-profiles-and-federation-credentials">
  Anthropic 프로필 및 페더레이션 자격증명
</h4>

프로필은 [Anthropic 구성 디렉터리](https://platform.claude.com/docs/en/manage-claude/wif-reference#configuration-directory)의 명명된 자격증명 구성 파일입니다. 기본값은 macOS 및 Linux에서 `~/.config/anthropic` 또는 Windows에서 `%APPDATA%\Anthropic`입니다. 프로필의 인증 모드는 [Workload Identity Federation (WIF)](https://platform.claude.com/docs/en/manage-claude/workload-identity-federation)에 대해 설정할 때 `oidc_federation`이거나 [`ant auth login`](https://platform.claude.com/docs/en/cli-sdks-libraries/cli/authentication)이 작성했을 때 또는 [API 키 없이 Console 계정에 로그인](#sign-in-without-an-api-key)할 때 `user_oauth`입니다.

Claude Code는 [베어 모드](/docs/ko/headless#start-faster-with-bare-mode), Claude Desktop 또는 클라우드 세션에서 프로필 또는 페더레이션 변수를 읽지 않습니다. 이러한 세션에서 `/status`는 `Profile` 행을 표시하지 않습니다.

Claude Code는 다음 순서로 3개의 소스를 확인하고 설정된 첫 번째에서 중지합니다. 표는 각 소스를 설정하는 것과 `/login` 자격증명에 대한 순위를 표시합니다.

| 소스       | 설정 대상                                                                                                                            | `/login`에 대한 순위                                                             |
| :------- | :------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------- |
| 명명된 프로필  | `ANTHROPIC_PROFILE`                                                                                                              | 위, 프로필이 가진 인증 모드 상관없이                                                       |
| 페더레이션 변수 | `ANTHROPIC_FEDERATION_RULE_ID` 및 `ANTHROPIC_ORGANIZATION_ID`, 둘 다 설정됨                                                            | 위                                                                           |
| 활성 프로필   | 구성 디렉터리의 [`active_config` 파일](https://platform.claude.com/docs/en/manage-claude/wif-reference#active-profile) 또는 `default`라는 프로필 | 인증 모드가 `oidc_federation`일 때 위; 인증 모드가 `user_oauth`일 때 작동하는 `/login` 자격증명 아래 |

`user_oauth` 규칙은 남은 `ant auth login` 프로필이 `/login`으로 로그인한 계정에서 요청을 이동하는 것을 방지합니다. 페더레이션 변수의 경우 Claude Code는 ID 토큰을 교환할 때 [WIF 참조](https://platform.claude.com/docs/en/manage-claude/wif-reference#environment-variables)의 `ANTHROPIC_IDENTITY_TOKEN_FILE`과 같은 다른 변수도 읽습니다. 프로필 파일 형식은 [WIF 참조](https://platform.claude.com/docs/en/manage-claude/wif-reference#profile-configuration-file)를 참조합니다.

Claude Code가 선택한 소스를 확인하려면 `/status`를 실행합니다. `Profile` 행은 `Login method` 행 대신 소스의 이름을 지정하고, 프로필이 사용 중인 자격증명일 때 `Organization` 및 `Email` 행은 해당 계정을 표시합니다.

`--debug`로 Claude Code를 시작하면 `~/.claude/debug/<session-id>.txt`의 디버그 로그에 소스 이름이 있는 `Using Anthropic profile auth` 행도 작성합니다. Claude Code가 작동하는 `/login` 자격증명이 있기 때문에 `user_oauth` 활성 프로필을 건너뛸 때 claude.ai 로그인을 대신 사용하고 있다는 경고를 디버그 로그에 작성합니다.

`user_oauth` 프로필의 로그인이 만료되고 Claude Code가 이를 갱신할 수 없으면 요청은 [Anthropic profile login expired](/docs/ko/errors#anthropic-profile-login-expired)로 실패합니다.

[claude.ai 커넥터](/docs/ko/mcp#use-mcp-servers-from-claude-ai) 및 [`/schedule`](/docs/ko/routines)과 같이 claude.ai 로그인이 필요한 기능은 이러한 소스 중 하나가 선택되어 있는 동안 사용할 수 없습니다. Claude Code가 소스를 선택하지 않도록 하려면:

* **명명된 프로필 또는 페더레이션 변수**: `ANTHROPIC_PROFILE`을 설정 해제하거나 페더레이션 변수 중 하나를 설정 해제합니다
* **활성 프로필**: [API 키 없이 Console 계정에 로그인](#sign-in-without-an-api-key)하여 작성한 현재 자격증명이 있는 `user_oauth` 프로필의 경우 `/logout`을 실행하고, `ant auth login`이 작성한 현재 자격증명이 있는 경우 `ant auth logout`을 실행하거나, 구성 디렉터리의 `configs/`에서 프로필의 파일을 삭제합니다.

<h3 id="generate-a-long-lived-token">
  장기 토큰 생성
</h3>

CI 파이프라인, 스크립트 또는 대화형 브라우저 로그인을 사용할 수 없는 기타 환경의 경우 `claude setup-token`으로 1년 OAuth 토큰을 생성합니다:

```bash theme={null}
claude setup-token
```

명령은 `/login`과 동일한 브라우저 인증 흐름을 열고, 브라우저에서 액세스를 승인한 후 토큰이 터미널에 인쇄됩니다. 토큰을 어디에도 저장하지 않으므로 복사하여 인증하려는 곳에 `CLAUDE_CODE_OAUTH_TOKEN` 환경 변수로 설정합니다:

```bash theme={null}
export CLAUDE_CODE_OAUTH_TOKEN=your-token
```

이 토큰은 Claude 구독으로 인증하며 Pro, Max, Team 또는 Enterprise 플랜이 필요합니다. 모델 요청만 수행할 수 있으므로 [Remote Control](/docs/ko/remote-control) 세션을 설정하거나 [claude.ai 커넥터](/docs/ko/mcp#use-mcp-servers-from-claude-ai)를 가져올 수 없습니다. 로컬로 구성한 MCP 서버는 계속 작동합니다.

[베어 모드](/docs/ko/headless#start-faster-with-bare-mode)는 `CLAUDE_CODE_OAUTH_TOKEN`을 읽지 않습니다. 스크립트가 `--bare`를 전달하면 `ANTHROPIC_API_KEY` 또는 `apiKeyHelper`로 인증합니다.
