> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 예제 설정 파일

> 개발자, 팀, 조직을 위한 현실적인 settings.json 파일: 하나를 복사하고 원하는 키를 유지한 후 값을 변경하세요.

이 페이지에는 설정을 저장하는 각 위치에 대한 세 가지 예제 `settings.json` 파일이 있습니다:

* 개발자의 `~/.claude/settings.json`
* 저장소에 커밋된 팀의 `.claude/settings.json`
* 조직의 `managed-settings.json`

각 파일은 해당 독자를 위한 그럴듯한 파일이므로 형태를 확인하고 원하는 부분을 복사할 수 있습니다. 어느 것도 권장되는 기본값이 아닙니다. 모든 값은 [설정 참조](/docs/ko/settings-reference)의 각 키 항목에서 나오며, 여기에는 해당 키의 유형, 기본값 및 설정할 수 있는 위치가 있습니다.

각 예제에는 두 개의 탭이 있습니다. **복사 가능한 설정 파일**은 저장할 파일입니다. **각 키의 역할**은 각 키 위에 주석이 있는 동일한 파일입니다. Claude Code는 설정 파일에서 주석을 허용하지 않으므로 첫 번째 탭에서 복사하세요.

<h2 id="your-own-settings">
  자신의 설정
</h2>

한 개발자의 개인 설정입니다. 모델과 노력 수준을 선택하고, 터미널을 조정하며, 읽기 전용 명령과 하나의 파일 읽기를 사전 승인합니다. 나열되지 않은 모든 것은 기본값을 유지합니다. 이와 같은 파일은 `~/.claude/settings.json`에 저장되며, 열 때마다 모든 프로젝트에 적용됩니다.

<Tabs>
  <Tab title="복사 가능한 설정 파일">
    이를 `~/.claude/settings.json`으로 저장하세요. 주석이 없는 유효한 JSON이므로 그대로 붙여넣고 원하지 않는 키를 삭제할 수 있습니다.

    ```json ~/.claude/settings.json theme={null}
    {
      "model": "claude-sonnet-5",
      "modelSettings": {
        "claude-sonnet-5": { "effortLevel": "xhigh" }
      },
      "editorMode": "vim",
      "theme": "light-daltonized",
      "statusLine": {
        "type": "command",
        "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'",
        "padding": 2
      },
      "spinnerTipsEnabled": false,
      "preferredNotifChannel": "terminal_bell",
      "permissions": {
        "allow": [
          "Bash(git diff *)",
          "Read(~/.zshrc)"
        ]
      },
      "autoUpdatesChannel": "stable",
      "cleanupPeriodDays": 20
    }
    ```
  </Tab>

  <Tab title="각 키의 역할">
    각 키 위에 주석이 있는 동일한 파일입니다. 여기서 읽으세요. Claude Code는 설정 파일에서 주석을 허용하지 않으므로 다른 탭에서 복사하세요.

    ```jsonc ~/.claude/settings.json theme={null}
    {
      // 모든 세션을 Sonnet 5에서 시작
      "model": "claude-sonnet-5",
      // Sonnet 5를 기본 높음 수준 이상으로 실행합니다. /effort는 모델당 수준을 저장하고, --effort는 단일 세션에 대해 설정합니다
      "modelSettings": {
        "claude-sonnet-5": { "effortLevel": "xhigh" }
      },
      // 프롬프트에서 Vim 키 바인딩
      "editorMode": "vim",
      // 색맹 친화적인 밝은 테마
      "theme": "light-daltonized",
      // 프롬프트 아래의 상태 줄: 모델 이름 및 사용된 컨텍스트
      "statusLine": {
        "type": "command",
        "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'",
        "padding": 2
      },
      // 스피너 아래에서 회전하는 팁 숨기기
      "spinnerTipsEnabled": false,
      // 완료된 작업이나 대기 중인 권한 프롬프트와 같은 알림에 대해 터미널 벨 울리기
      "preferredNotifChannel": "terminal_bell",
      // Claude Code가 git diff를 실행하고 묻지 않고 .zshrc를 읽도록 허용
      "permissions": {
        "allow": [
          "Bash(git diff *)",
          "Read(~/.zshrc)"
        ]
      },
      // 안정 채널에서 업데이트 받기
      "autoUpdatesChannel": "stable",
      // 20일보다 오래된 세션 기록 및 기타 로컬 세션 데이터 삭제
      "cleanupPeriodDays": 20
    }
    ```
  </Tab>
</Tabs>

<h2 id="a-teams-shared-settings">
  팀의 공유 설정
</h2>

저장소에 커밋된 팀의 공유 설정으로, 이를 복제하는 모든 사람이 동일한 권한, 훅, 플러그인 마켓플레이스를 얻습니다. 저장소의 최상위에 `.claude/settings.json`에 이와 같은 파일을 저장하세요. 커밋하기 전에 알아야 할 사항:

* **클라우드 세션도 이를 읽습니다.** [클라우드 세션](/docs/ko/settings#settings-in-cloud-sessions)은 저장소의 복제본에서 시작되므로 커밋된 파일이 거기에도 적용됩니다.
* **원격 측정은 관리형 또는 개인 설정에 포함됩니다.** Claude Code는 저장소의 설정 파일에서 [OpenTelemetry 내보내기 변수](/docs/ko/settings-reference#variables-claude-code-ignores-in-env)를 무시하며, 원격 측정을 끄는 일부 값은 제외합니다. 조직의 [관리형 설정](/docs/ko/monitoring-usage#administrator-configuration)에서 또는 각 사람의 `~/.claude/settings.json`에서 설정하세요.
* **허용 규칙은 신뢰를 기다립니다.** 허용 규칙 및 `extraKnownMarketplaces` 항목은 각 사람이 [이 폴더 자체를 신뢰](/docs/ko/permissions#project-allow-rules-and-workspace-trust)한 후에 적용되며, 상위 폴더만이 아닙니다. 거부 및 요청 규칙은 신뢰 여부와 관계없이 모든 세션에 적용됩니다.
* **훅은 저장소의 스크립트입니다.** 이 파일의 훅은 `.claude/hooks/block-rm.sh`를 실행합니다. [훅이 해결되는 방식](/docs/ko/hooks#how-a-hook-resolves)은 이를 작성하는 과정을 설명합니다.
* **규칙은 작성된 대로 명령 및 경로와 일치합니다.** `Bash(git push *)`는 [`git -C . push`](/docs/ko/permissions#bash-rule-limits)와 일치하지 않습니다. `Read(./.env)`는 그 자체로 파일 도구 및 `cat .env`와 같이 파일을 이름으로 지정하는 명령을 중지하지만, [`grep -r`이 디렉토리에서 실행](/docs/ko/permissions#read-and-edit)되는 것은 중지하지 않습니다. 이 파일의 `sandbox` 블록은 sandbox가 [모든 샌드박스 명령이 읽을 수 없는 항목에 `Read` 거부 경로를 추가](/docs/ko/settings-reference#sandbox-filesystem-denyread)하기 때문에 이 격차를 메웁니다.

<Tabs>
  <Tab title="복사 가능한 설정 파일">
    이를 저장소의 최상위에 `.claude/settings.json`으로 저장하고 커밋하세요. 주석이 없는 유효한 JSON이므로 그대로 붙여넣고 원하지 않는 키를 삭제할 수 있습니다.

    ```json .claude/settings.json theme={null}
    {
      "permissions": {
        "allow": [
          "Bash(npm run *)"
        ],
        "ask": [
          "Bash(git push *)"
        ],
        "deny": [
          "Read(./.env)",
          "Read(./.env.*)",
          "Read(./secrets/**)"
        ]
      },
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.sh"
              }
            ]
          }
        ]
      },
      "extraKnownMarketplaces": {
        "acme-tools": {
          "source": {
            "source": "github",
            "repo": "acme-corp/claude-plugins"
          }
        }
      },
      "enabledPlugins": {
        "code-formatter@acme-tools": true
      },
      "sandbox": {
        "enabled": true,
        "filesystem": {
          "allowWrite": [
            "/tmp/build"
          ]
        },
        "network": {
          "allowedDomains": [
            "registry.npmjs.org",
            "*.example.com"
          ]
        }
      },
      "plansDirectory": "./plans"
    }
    ```
  </Tab>

  <Tab title="각 키의 역할">
    각 키 위에 주석이 있는 동일한 파일입니다. 여기서 읽으세요. Claude Code는 설정 파일에서 주석을 허용하지 않으므로 다른 탭에서 복사하세요.

    ```jsonc .claude/settings.json theme={null}
    {
      "permissions": {
        // npm 스크립트를 묻지 않고 실행
        "allow": [
          "Bash(npm run *)"
        ],
        // git push 명령 전에 확인
        "ask": [
          "Bash(git push *)"
        ],
        // 파일 도구 및 파일 읽기 명령으로 env 파일 및 secrets 폴더 읽기 거부
        "deny": [
          "Read(./.env)",
          "Read(./.env.*)",
          "Read(./secrets/**)"
        ]
      },
      // 모든 Bash 명령 전에 저장소의 스크립트를 실행하여 이를 차단할 수 있습니다
      "hooks": {
        "PreToolUse": [
          {
            "matcher": "Bash",
            "hooks": [
              {
                "type": "command",
                "command": "${CLAUDE_PROJECT_DIR}/.claude/hooks/block-rm.sh"
              }
            ]
          }
        ]
      },
      // 모든 복제에서 팀의 플러그인 마켓플레이스 등록
      "extraKnownMarketplaces": {
        "acme-tools": {
          "source": {
            "source": "github",
            "repo": "acme-corp/claude-plugins"
          }
        }
      },
      // 해당 마켓플레이스에서 하나의 플러그인을 활성화합니다. GitHub 저장소와 같은 외부 소스의 플러그인은 여전히 각 사람이 한 번 설치해야 합니다
      "enabledPlugins": {
        "code-formatter@acme-tools": true
      },
      // 샌드박스 명령: 쓰기 가능한 빌드 디렉토리; npm 및 example.com 사전 허용, 다른 호스트는 여전히 프롬프트
      "sandbox": {
        "enabled": true,
        "filesystem": {
          "allowWrite": [
            "/tmp/build"
          ]
        },
        "network": {
          "allowedDomains": [
            "registry.npmjs.org",
            "*.example.com"
          ]
        }
      },
      // 계획 파일을 저장소 내부에 유지
      "plansDirectory": "./plans"
    }
    ```
  </Tab>
</Tabs>

<h2 id="an-organizations-managed-settings">
  조직의 관리 설정
</h2>

관리 키의 형태를 보여주는 `managed-settings.json` 파일로, 각각에 대해 하나의 그럴듯한 값이 있습니다. 권장되는 정책이 아닙니다: 자신의 요구 사항과 일치하는 키를 선택하고 자신의 값을 설정하세요. 예제는 다음 키를 설정합니다:

* `forceLoginMethod` 및 `forceLoginOrgUUID`는 로그인 방법 및 조직을 고정합니다
* `availableModels` 및 `enforceAvailableModels`는 세션이 사용할 수 있는 모델을 제한합니다
* `permissions.deny`는 두 개의 파일 읽기 및 `curl` 명령을 거부하고 [Claude가 작성하는 방식](/docs/ko/permissions#bash-rule-limits)으로, `disableBypassPermissionsMode`는 바이패스 권한 모드를 제거합니다
* [`allowManagedPermissionRulesOnly`](/docs/ko/settings-reference#allowmanagedpermissionrulesonly) 및 [`allowManagedMcpServersOnly`](/docs/ko/settings-reference#allowmanagedmcpserversonly)는 관리 권한 및 MCP 허용 목록을 유일하게 적용되는 것으로 만듭니다
* `allowedMcpServers`는 MCP 서버를 URL로 고정합니다
* `strictKnownMarketplaces`는 하나의 플러그인 마켓플레이스를 허용합니다
* `sandbox`는 고정 네트워크 허용 목록과 샌드박스되지 않은 재시도 없이 명령을 샌드박싱합니다
* `requiredMinimumVersion`은 최소 Claude Code 버전을 설정합니다
* `cleanupPeriodDays`는 세션 기록 및 기타 로컬 데이터의 보존을 7일로 단축합니다
* `companyAnnouncements`는 시작 시 메시지를 표시합니다

관리자는 이와 같은 파일을 `managed-settings.json`으로 배포하거나, MDM 또는 [서버 관리 설정](/docs/ko/server-managed-settings)을 통해 동일한 JSON을 배포합니다. 배포된 하나의 파일은 도달하는 모든 머신 또는 계정에 적용됩니다. 그룹에 다른 값을 제공하려면 해당 그룹에 다른 파일 또는 프로필을 배포하세요. [서버 관리 설정은 아직 그룹별 정책을 지원하지 않기 때문입니다](/docs/ko/server-managed-settings#current-limitations).

<Tabs>
  <Tab title="복사 가능한 설정 파일">
    이를 `managed-settings.json`으로 배포하거나, MDM 또는 claude.ai 콘솔을 통해 동일한 JSON을 배포하세요. 주석이 없는 유효한 JSON입니다. 예제 조직 UUID, 서버 URL 및 마켓플레이스를 자신의 것으로 바꾸고 원하지 않는 키를 삭제하세요.

    ```json managed-settings.json theme={null}
    {
      "forceLoginMethod": "claudeai",
      "forceLoginOrgUUID": [
        "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
      ],
      "availableModels": [
        "opus",
        "sonnet"
      ],
      "enforceAvailableModels": true,
      "permissions": {
        "deny": [
          "Bash(curl *)",
          "Read(./.env)",
          "Read(./secrets/**)"
        ],
        "disableBypassPermissionsMode": "disable"
      },
      "allowManagedPermissionRulesOnly": true,
      "allowedMcpServers": [
        {
          "serverUrl": "https://api.githubcopilot.com/*"
        }
      ],
      "allowManagedMcpServersOnly": true,
      "strictKnownMarketplaces": [
        {
          "source": "github",
          "repo": "acme-corp/approved-plugins"
        }
      ],
      "sandbox": {
        "enabled": true,
        "failIfUnavailable": true,
        "allowUnsandboxedCommands": false,
        "network": {
          "allowedDomains": [
            "registry.npmjs.org",
            "github.com"
          ],
          "allowManagedDomainsOnly": true
        }
      },
      "requiredMinimumVersion": "2.1.150",
      "cleanupPeriodDays": 7,
      "companyAnnouncements": [
        "Welcome to Acme Corp! Review our code guidelines at docs.example.com"
      ]
    }
    ```
  </Tab>

  <Tab title="각 키의 역할">
    각 키 위에 주석이 있는 동일한 파일입니다. 여기서 읽으세요. Claude Code는 설정 파일에서 주석을 허용하지 않으므로 다른 탭에서 복사하세요.

    ```jsonc managed-settings.json theme={null}
    {
      // claude.ai 로그인만, 그리고 이 조직에서만
      "forceLoginMethod": "claudeai",
      "forceLoginOrgUUID": [
        "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
      ],
      // Opus 및 Sonnet 모델만; enforceAvailableModels를 사용하면 기본 옵션도 목록을 따릅니다
      "availableModels": [
        "opus",
        "sonnet"
      ],
      "enforceAvailableModels": true,
      "permissions": {
        // 모든 머신에서 curl 명령 및 프로젝트의 .env 파일과 secrets 폴더 읽기 거부
        "deny": [
          "Bash(curl *)",
          "Read(./.env)",
          "Read(./secrets/**)"
        ],
        // 모든 세션에서 바이패스 권한 모드 제거
        "disableBypassPermissionsMode": "disable"
      },
      // 사용자, 프로젝트 및 로컬 설정의 권한 규칙 무시
      "allowManagedPermissionRulesOnly": true,
      // GitHub MCP 서버만, 사용자가 모든 서버를 "github"로 이름 지을 수 있으므로 이름이 아닌 URL로 일치합니다.
      // 목록과 일치하지 않는 사용자 추가 서버는 로드되지 않으며, 목록에 URL 항목만 있을 때 모든 stdio 서버를 포함합니다.
      // 아래의 allowManagedMcpServersOnly 키는 이 관리 목록을 유일하게 적용되는 허용 목록으로 만듭니다
      "allowedMcpServers": [
        {
          "serverUrl": "https://api.githubcopilot.com/*"
        }
      ],
      "allowManagedMcpServersOnly": true,
      // 플러그인은 이 마켓플레이스에서만 올 수 있습니다
      "strictKnownMarketplaces": [
        {
          "source": "github",
          "repo": "acme-corp/approved-plugins"
        }
      ],
      // Claude가 실행하는 모든 명령을 샌드박싱하고, 샌드박스를 설정할 수 없으면 시작을 거부하며,
      // 차단된 명령이 샌드박스 외부에서 재시도되도록 허용하지 않습니다. 네트워크는
      // npm 및 GitHub로 제한되며, 사용자는 도메인을 추가할 수 없습니다
      "sandbox": {
        "enabled": true,
        "failIfUnavailable": true,
        "allowUnsandboxedCommands": false,
        "network": {
          "allowedDomains": [
            "registry.npmjs.org",
            "github.com"
          ],
          "allowManagedDomainsOnly": true
        }
      },
      // 2.1.150보다 오래된 버전에서 시작하기를 거부합니다
      "requiredMinimumVersion": "2.1.150",
      // 7일 후 세션 기록 및 기타 로컬 세션 데이터 삭제
      "cleanupPeriodDays": 7,
      // 모든 사용자가 시작 시 보는 메시지
      "companyAnnouncements": [
        "Welcome to Acme Corp! Review our code guidelines at docs.example.com"
      ]
    }
    ```
  </Tab>
</Tabs>
