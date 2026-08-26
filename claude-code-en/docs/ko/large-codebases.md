> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 모노레포 또는 대규모 코드베이스에서 Claude Code 설정하기

> 중첩된 CLAUDE.md 파일, 스파스 워크트리, 코드 인텔리전스, 패키지별 스킬을 사용하여 모노레포 및 대규모 단일 트리 코드베이스에 대해 Claude Code를 구성하여 Claude가 작업 중인 코드에 집중하도록 유지합니다.

대규모 코드베이스는 수백만 줄의 코드가 있는 하나의 저장소이거나 많은 패키지가 있는 모노레포일 수 있습니다. Claude Code는 모든 규모에서 작동하지만, 코드베이스가 커질수록 더 작은 프로젝트를 위해 조정된 기본값이 컨텍스트 윈도우를 작업과 관련 없는 지침 및 파일 읽기로 채워서 토큰 비용을 증가시키고 Claude의 성능을 저하시킬 수 있습니다.

이 가이드는 개별 개발자와 엔지니어링 팀이 Claude를 작업이 영향을 미치는 코드베이스의 일부로 범위를 지정하는 방법을 보여줍니다. 각 섹션은 설정이 개인 머신에 개인적인지 저장소에 커밋되는지 명시합니다.

<h2 id="what-this-guide-covers">
  이 가이드에서 다루는 내용
</h2>

아래의 [표](#settings-on-this-page)는 각 설정과 그것이 수행하는 작업을 나열합니다. 그 다음의 [파일 트리](#the-example-monorepo)는 이 페이지의 모든 코드 샘플이 참조하는 예제 모노레포입니다.

<h3 id="settings-on-this-page">
  이 페이지의 설정
</h3>

아래의 각 설정은 독립적입니다. 서로를 대체하지 않고 계층화되므로 저장소에 맞는 것을 적용하십시오. [Claude를 시작할 위치 선택](#choose-where-to-start-claude)은 설정 파일이 어디에 있는지 결정하므로 먼저 읽으십시오. [모두 함께 사용](#put-it-together)은 모두 결합된 것을 보여줍니다.

| 원하는 것                                            | 사용                                                                                      |
| :----------------------------------------------- | :-------------------------------------------------------------------------------------- |
| 모든 하위 시스템을 다루는 하나의 루트 파일 대신 터치하는 코드에 대한 규칙만 로드   | 디렉토리별 [CLAUDE.md 파일](#layer-claude-md-files-by-directory)                               |
| 작업하지 않는 패키지의 CLAUDE.md 파일 제외                     | [`claudeMdExcludes`](#exclude-irrelevant-claude-md-files)                               |
| Claude가 빌드 출력, 생성된 코드, 벤더된 종속성 열기 차단             | `permissions.deny`의 [`Read` 거부 규칙](#block-reads-of-generated-and-vendored-code)         |
| 파일 스캔 대신 언어 서버를 통해 기호의 정의 또는 호출자 찾기              | [코드 인텔리전스 플러그인](#reduce-file-reads-with-code-intelligence)                              |
| Claude가 워크트리를 생성할 때 작업에 필요한 디렉토리만 체크아웃           | [`worktree.sparsePaths`](#check-out-only-the-directories-you-need)                      |
| 같은 세션에서 형제 패키지 또는 다른 저장소 읽기 및 편집                 | [`--add-dir`](#grant-access-across-packages-or-repositories) 또는 `additionalDirectories` |
| Claude에 관련이 있을 때만 로드되는 한 영역에 특정한 절차 제공           | 디렉토리별 [skills](#add-per-directory-skills)                                               |
| 많은 디렉토리별 CLAUDE.md 파일을 모든 사람이 설치하는 하나의 규칙 세트로 대체 | 내부 마켓플레이스의 [플러그인](#centralize-conventions-when-layering-stops-scaling)                  |

<Tip>
  [서브에이전트에서 탐색 실행](/docs/ko/best-practices#use-subagents-for-investigation)과 같이 모든 저장소에서 컨텍스트를 작게 유지하는 워크플로우 기법은 [Claude Code 모범 사례](/docs/ko/best-practices)를 참조하십시오. 조직의 모든 개발자에게 기본 구성을 배포하려면 [조직을 위해 Claude Code 설정](/docs/ko/admin-setup)을 참조하십시오.
</Tip>

<h3 id="the-example-monorepo">
  예제 모노레포
</h3>

이 페이지의 예제는 세 개의 패키지가 있는 모노레포를 참조합니다. 동일한 패턴은 대규모 단일 트리 코드베이스에서 작동합니다. 예제가 `packages/api/`를 사용하는 경우 `src/backend/` 또는 `lib/core/`와 같은 자신의 하위 시스템 디렉토리로 대체하십시오.

```text theme={null}
monorepo/
  CLAUDE.md                     # 루트 지침
  packages/
    api/
      CLAUDE.md                 # API 특정 지침
      .claude/skills/
      src/
    web/
      CLAUDE.md                 # 프론트엔드 특정 지침
      .claude/skills/
      src/
    shared/
      CLAUDE.md                 # 공유 라이브러리 지침
      src/
```

<h2 id="choose-where-to-start-claude">
  Claude를 시작할 위치 선택
</h2>

`claude`를 시작하는 위치는 Claude가 추가 권한 부여 없이 읽고 편집할 수 있는 파일, 시작 시 로드되는 CLAUDE.md 파일, 적용되는 프로젝트 설정을 결정합니다.

| 시작 위치   | 파일 액세스                     | 시작 시 로드된 CLAUDE.md                        | 사용 시기                           |
| :------ | :------------------------- | :---------------------------------------- | :------------------------------ |
| 저장소 루트  | 모든 파일                      | 루트만; 하위 디렉토리 파일은 Claude가 그곳을 읽을 때 요청 시 로드 | 작업이 여러 패키지 또는 하위 시스템에 걸쳐 있음     |
| 하위 디렉토리 | 더 많은 권한을 부여할 때까지 해당 하위 트리만 | 해당 디렉토리의 모든 상위 항목                         | 작업이 하나의 패키지 또는 하위 시스템으로 범위가 지정됨 |

`.claude/settings.json`의 프로젝트 설정은 시작 디렉토리에서만 로드되며 CLAUDE.md 파일이 로드되는 방식처럼 상위 디렉토리에서 상속되지 않습니다. 저장소 루트의 `.claude/settings.json`은 루트에서 시작할 때만 적용됩니다.

아래의 각 섹션은 설정 파일이 저장소 루트에 있는지 시작하는 하위 디렉토리에 있는지, 커밋되는지 로컬로 유지되는지 명시합니다.

<h2 id="layer-claude-md-files-by-directory">
  디렉토리별로 CLAUDE.md 파일 계층화
</h2>

대규모 코드베이스에서 저장소 루트의 단일 CLAUDE.md는 모든 하위 시스템의 규칙을 다루도록 성장하는 경향이 있어서 현재 작업과 관련 없는 지침에 컨텍스트 비용이 들거나, 너무 일반적이어서 유용하지 않게 됩니다. 지침을 디렉토리별 파일에 분산시키면 Claude는 저장소 전체 규칙과 작업 중인 코드의 규칙만 로드합니다.

Claude Code는 시작 시 작업 디렉토리와 모든 상위 디렉토리의 모든 [CLAUDE.md](/docs/ko/memory) 파일을 로드한 다음, 파일을 읽을 때 요청 시 각 하위 디렉토리의 파일을 로드합니다. 루트 파일은 저장소 전체 규칙을 설정하고 각 하위 디렉토리는 자신의 규칙을 추가합니다.

일반적인 분할은 두 가지 수준입니다:

* **루트 `CLAUDE.md`**: 코딩 표준, 커밋 규칙, 저장소 레이아웃과 같이 모든 곳에 적용되는 지침
* **하위 디렉토리별 `CLAUDE.md`**: 해당 영역의 스택에 특정한 규칙. 모노레포에서는 패키지당 하나입니다. 대규모 단일 트리에서는 `src/db/` 또는 `src/api/`와 같은 하위 시스템당 하나입니다.

이 파일들을 저장소에 커밋하여 팀원들이 상속받도록 하십시오. 각 디렉토리의 소유자는 일반적으로 해당 파일을 유지합니다.

루트 `CLAUDE.md`는 Claude를 저장소 구조에 맞춥니다:

```markdown CLAUDE.md theme={null}
이것은 packages/ 아래에 세 개의 패키지가 있는 모노레포입니다:

- packages/api: Express, TypeScript, PostgreSQL을 사용하는 Node.js REST API
- packages/web: Vite, TypeScript, TailwindCSS를 사용하는 React 프론트엔드
- packages/shared: api와 web 모두에서 사용하는 공유 TypeScript 유틸리티

모노레포 루트가 아닌 패키지 디렉토리에서 명령을 실행하십시오.
각 패키지는 자신의 tsconfig.json, package.json, 테스트 스위트를 가집니다.
```

각 하위 디렉토리의 `CLAUDE.md`, 여기서는 `packages/api/CLAUDE.md`는 해당 영역의 스택에 특정한 컨텍스트를 추가합니다:

```markdown packages/api/CLAUDE.md theme={null}
이 패키지는 REST API 서버입니다.

- 테스트 실행: `npm test` (Vitest 사용)
- 개발 서버 실행: `npm run dev` (포트 3001)
- 데이터베이스 마이그레이션: `npm run migrate`
- 환경 변수: `.env.example`을 `.env`로 복사

API 경로는 src/routes/에 있습니다. 각 경로 파일은 Express 라우터를 내보냅니다.
데이터베이스 쿼리는 src/db/에서 Knex를 사용합니다. 경로 핸들러에서 원시 SQL 문자열을 작성하지 마십시오.
```

`packages/api/`에서 Claude를 시작하면 `packages/api/CLAUDE.md`와 루트 `CLAUDE.md`를 모두 로드합니다. Claude는 로컬 지침을 저장소 전체 규칙과 함께 보며, `packages/web/`의 지침은 컨텍스트에 없습니다. 비모노레포 트리의 모든 하위 디렉토리에도 동일하게 적용됩니다.

코드베이스와 모델이 변경될 때 파일을 최신 상태로 유지하는 몇 가지 방법:

* **풀 요청에서 검토**: CLAUDE.md 편집을 다른 문서 변경처럼 취급하여 규칙이 코드를 추적하도록 합니다.
* **주요 모델 릴리스 후 재검토**: 이전 모델의 제한을 해결한 지침은 새로운 모델이 경우를 자체적으로 처리하면 오버헤드가 될 수 있습니다. 예를 들어, 단일 파일 리팩토링을 강제하는 규칙은 제한이 없어지면 삭제할 수 있습니다.
* **업데이트를 제안하는 Stop 훅 추가**: [`Stop` 훅](/docs/ko/hooks#stop)은 Claude가 응답을 마칠 때 세션 기록의 경로를 받으므로 스크립트는 세션을 검토하고 노출된 간격이 신선할 때 CLAUDE.md 업데이트를 제안할 수 있습니다.

CLAUDE.md 파일이 로드되고 상호작용하는 방식에 대한 자세한 내용은 [메모리 및 프로젝트 지침](/docs/ko/memory)을 참조하십시오.

<h3 id="choose-between-per-directory-claude-md-and-path-scoped-rules">
  디렉토리별 CLAUDE.md와 경로 범위 규칙 중 선택
</h3>

디렉토리별 `CLAUDE.md` 파일과 `.claude/rules/` 아래의 [경로 범위 규칙](/docs/ko/memory#path-specific-rules) 모두 트리의 일부에 지침을 대상으로 지정할 수 있습니다. 파일이 있는 위치와 로드되는 시기가 다릅니다.

| 접근 방식                      | 파일 위치                 | 로드 시기                                          | 사용 시기                                    |
| :------------------------- | :-------------------- | :--------------------------------------------- | :--------------------------------------- |
| 디렉토리별 `CLAUDE.md`          | 디렉토리 내부, 코드와 함께       | 해당 디렉토리에서 시작할 때 시작 시, 또는 Claude가 파일을 읽을 때 요청 시 | 디렉토리 소유자가 자신의 규칙을 유지; 지침은 코드와 함께 버전 관리됨  |
| `.claude/rules/`의 경로 범위 규칙 | 저장소 루트의 중앙 `.claude/` | Claude가 규칙의 `paths:` 글로브와 일치하는 파일로 작업할 때       | 모든 규칙을 한 곳에 원하거나, 동일한 규칙이 많은 분산된 경로에 적용됨 |

스킬도 포함하는 비교는 [유사한 기능 비교](/docs/ko/features-overview#compare-similar-features)를 참조하십시오.

<h3 id="exclude-irrelevant-claude-md-files">
  관련 없는 CLAUDE.md 파일 제외
</h3>

저장소 루트에서 Claude를 시작하면 각 하위 디렉토리의 CLAUDE.md는 Claude가 해당 디렉토리의 파일을 읽자마자 로드됩니다. `claudeMdExcludes` 설정은 경로 또는 글로브 패턴으로 특정 파일을 건너뛰어 로드되지 않도록 합니다.

다른 팀의 패키지, 레거시 코드, 또는 벤더된 하위 트리와 같이 작업하지 않는 디렉토리에 사용하십시오. 제외 목록은 정적이며 작업별 스위치가 아닙니다. 오늘 한 패키지에 집중하고 내일 다른 패키지에 집중하려면 제외를 편집하는 대신 [해당 패키지의 디렉토리에서 Claude를 시작](#choose-where-to-start-claude)하십시오.

이 제외를 자신만을 위해 원하면 설정을 `.claude/settings.local.json`에 넣으십시오. Claude Code는 이를 생성할 때 gitignore합니다. 여기서 직접 생성하므로 gitignore에 추가하십시오. 패턴은 절대 파일 경로와 일치하는 glob 구문을 사용하므로 상대 스타일 패턴을 `**/`로 시작하여 트리의 어디든 일치시킵니다. 아래 예제는 다른 팀이 소유한 패키지를 제외합니다:

```json .claude/settings.local.json theme={null}
{
  "claudeMdExcludes": [
    "**/packages/admin-dashboard/**",
    "**/packages/legacy-*/**"
  ]
}
```

이는 해당 패키지 아래의 모든 CLAUDE.md 및 규칙 파일을 건너뜁니다. 루트 CLAUDE.md와 작업하는 패키지는 정상적으로 로드됩니다.

이 패턴은 다른 일반적인 경우를 다룹니다:

* `"**/packages/*/CLAUDE.md"`: 루트를 유지하면서 모든 패키지의 CLAUDE.md를 제외
* `"**/packages/web/**"`: 규칙을 포함하여 웹 패키지 아래의 모든 것을 제외
* `"/home/user/monorepo/legacy/CLAUDE.md"`: 절대 경로로 하나의 특정 파일을 제외

관리되는 정책 CLAUDE.md 파일은 제외할 수 없으므로 조직 전체 지침은 항상 적용됩니다. `claudeMdExcludes`를 모든 [설정 범위](/docs/ko/settings#configuration-scopes)에서 설정할 수 있습니다: 사용자, 프로젝트, 로컬, 또는 관리됨. 배열은 범위 전체에 병합되므로 팀은 프로젝트 수준 기본값을 설정하면서 개인은 로컬 재정의를 추가할 수 있습니다.

전체 제외 문서는 [특정 CLAUDE.md 파일 제외](/docs/ko/memory#exclude-specific-claude-md-files)를 참조하십시오.

<h2 id="reduce-what-claude-reads">
  Claude가 읽는 것 줄이기
</h2>

지침은 Claude의 컨텍스트에 끝나는 것의 일부일 뿐입니다. 파일 읽기는 코드베이스와 함께 증가하는 또 다른 비용입니다. 아래 설정은 관련 없는 경로의 읽기를 차단하고 철저한 파일 스캔을 언어 서버 조회로 대체합니다.

<h3 id="block-reads-of-generated-and-vendored-code">
  생성된 코드와 벤더된 코드의 읽기 차단
</h3>

Claude의 콘텐츠 검색은 기본적으로 `.gitignore`를 존중하므로 `node_modules/`, `dist/`, `build/`와 같이 이미 나열된 경로는 추가 구성 없이 검색 결과에서 벗어납니다.

벤더된 SDK 또는 커밋된 생성 코드와 같이 체크인된 경로의 경우 `permissions.deny`에 `Read` 거부 규칙을 추가하여 검색이 나열하더라도 Claude가 해당 파일을 열지 못하도록 차단합니다.

이 제외를 저장소에서 작업하는 모든 사람에게 적용하려면 `.claude/settings.json`에 커밋하십시오. 개인적으로 유지하려면 `.claude/settings.local.json`을 대신 사용하십시오. 이 페이지의 다른 프로젝트 설정처럼 이 파일들은 시작 디렉토리에서만 로드됩니다. 루트에서 Claude를 시작하면 저장소 루트에 배치하거나, 하위 디렉토리에서 시작하면 각 패키지의 `.claude/`에 배치하십시오. 시작 디렉토리에 관계없이 모든 세션에서 동일한 거부 규칙을 적용하려면 [관리되는 설정](/docs/ko/settings#settings-files)에서 설정하십시오. 사용자 및 프로젝트 설정은 이를 재정의할 수 없습니다.

아래 예제는 빌드 아티팩트와 벤더된 SDK를 차단합니다:

```json .claude/settings.json theme={null}
{
  "permissions": {
    "deny": [
      "Read(./**/dist/**)",
      "Read(./**/build/**)",
      "Read(./**/*.generated.*)",
      "Read(./vendor/**)"
    ]
  }
}
```

거부 규칙은 Claude의 기본 제공 파일 도구와 `cat`, `head`, `grep`, `find`를 포함한 인식된 Bash 파일 명령을 다룹니다. 거부된 경로가 인수로 전달될 때입니다. 재귀 검색의 출력에서 거부된 경로를 필터링하지 않으며, 파일을 자체적으로 열기 위해 임의의 하위 프로세스를 다루지 않습니다. 전체 패턴 구문은 [Read 및 Edit 권한 규칙](/docs/ko/permissions#read-and-edit)을 참조하십시오.

<h3 id="reduce-file-reads-with-code-intelligence">
  코드 인텔리전스로 파일 읽기 줄이기
</h3>

대규모 코드베이스에서 기호가 정의되거나 사용되는 위치를 찾는 것은 많은 파일 읽기와 grep 호출이 필요할 수 있습니다. [코드 인텔리전스 플러그인](/docs/ko/discover-plugins#code-intelligence)은 Claude를 언어 서버에 연결하여 트리를 스캔하는 대신 정의로 이동하고, 참조를 찾고, 타입 오류를 직접 표시할 수 있습니다.

공식 마켓플레이스에는 TypeScript, Python, Go, Rust 및 기타 일반적인 언어용 플러그인이 있습니다. 아래 예제는 TypeScript 플러그인을 설치합니다:

```shell theme={null}
/plugin install typescript-lsp@claude-plugins-official
```

플러그인을 자신이 설치하는 대신 저장소의 모든 사람에게 활성화하려면 [`enabledPlugins` 프로젝트 설정](/docs/ko/settings#plugin-settings)에 추가하십시오.

코드 인텔리전스 플러그인은 각 개발자의 머신에 언어의 언어 서버 바이너리가 필요합니다. [각 언어가 필요로 하는 바이너리](/docs/ko/discover-plugins#code-intelligence)를 참조하십시오. 공식 마켓플레이스에서 설치하려면 마켓플레이스가 호스팅되는 GitHub에 대한 네트워크 액세스가 필요합니다. 제한된 네트워크에서는 [내부 Git 호스트 또는 로컬 경로에서 마켓플레이스를 추가](/docs/ko/discover-plugins#add-from-other-git-hosts)하십시오.

이는 위의 `claudeMdExcludes` 및 `Read` 거부 규칙과 잘 어울립니다. 이들은 관련 없는 콘텐츠를 컨텍스트에서 벗어나게 하고, 코드 인텔리전스는 Claude가 정의를 찾기 위해 남은 것을 읽지 못하도록 합니다.

<h2 id="scope-worktrees-and-file-access">
  워크트리 및 파일 액세스 범위 지정
</h2>

이 설정은 워크트리에서 디스크에 있는 것과 Claude가 시작점 이상으로 읽고 쓸 수 있는 디렉토리를 제어합니다.

<h3 id="check-out-only-the-directories-you-need">
  필요한 디렉토리만 체크아웃
</h3>

`--worktree` 플래그는 새로운 git 워크트리에서 세션을 시작하여 변경 사항이 주 체크아웃에서 격리되도록 합니다. 기본적으로 전체 저장소를 체크아웃합니다. 대규모 저장소에서 `worktree.sparsePaths` 설정은 git sparse-checkout을 사용하여 나열된 디렉토리와 루트 수준 파일만 디스크에 작성하므로 워크트리가 더 빠르게 시작되고 더 적은 공간을 사용합니다.

이 디렉토리에서 작업하는 모든 사람이 동일한 경로가 필요하면 설정을 `.claude/settings.json`에 커밋하십시오. 자신을 위해 경로를 추가하려면 `.claude/settings.local.json`을 사용하십시오. 목록은 범위 전체에 병합되므로 로컬 파일은 커밋된 목록에 경로를 추가할 수 있지만 제거할 수는 없습니다. 아래 예제는 커밋된 파일을 보여줍니다:

```json .claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ]
  }
}
```

Claude가 워크트리를 생성할 때 전체 트리 대신 `.claude/`, `packages/api/`, `packages/shared/`만 체크아웃합니다. `sparsePaths`의 경로는 Claude를 시작하는 하위 디렉토리에 관계없이 저장소 루트에 상대적입니다. 패키지 루트뿐만 아니라 모든 디렉토리 경로가 여기에서 작동합니다.

이는 특히 [서브에이전트 워크트리 격리](/docs/ko/worktrees#isolate-subagents-with-worktrees)에 유용합니다. 서브에이전트는 하위 작업을 위해 생성된 병렬 Claude 인스턴스이며, 워크트리에서 실행되는 각 인스턴스는 전체 트리 대신 경량 체크아웃을 받습니다. 세션의 모든 워크트리는 동일한 `sparsePaths`를 공유하므로 한 서브에이전트가 `packages/api/`를 필요로 하고 다른 하나가 `packages/web/`을 필요로 하면 둘 다 나열하십시오.

`sparsePaths`에 개별 파일이 아닌 디렉토리를 나열하십시오. `package.json`, `tsconfig.base.json`, 잠금 파일과 같은 루트 수준 파일은 나열한 디렉토리와 함께 항상 체크아웃됩니다. 루트 수준 디렉토리는 그렇지 않으므로 워크트리 내에서 저장소 루트의 `.claude/settings.json`, `.claude/rules/`, 또는 `.claude/skills/`를 사용 가능하게 하려면 목록에 `.claude`를 포함하십시오.

Sparse checkout을 사용하려면 git이 sparse 워크트리가 존재하는 동안 저장소의 공유 `.git/config`에서 `extensions.worktreeConfig`를 활성화해야 합니다. Claude Code는 마지막 워크트리가 제거된 후 해당 항목을 제거하지만, Claude Code가 추가한 경우에만 제거합니다. 직접 설정한 값은 절대 제거하지 않습니다. v2.1.207 이전에는 마지막 워크트리가 제거된 후에도 항목이 남아 있었으며, `tea`와 같은 go-git 기반 도구가 `git config --unset extensions.worktreeConfig`를 실행할 때까지 저장소를 열지 못했습니다.

`node_modules`와 같은 대규모 디렉토리를 워크트리 전체에 중복되지 않도록 하려면 동일한 `.claude/settings.json`에서 `sparsePaths`를 `symlinkDirectories`와 쌍으로 지정하십시오:

```json .claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ],
    "symlinkDirectories": [
      "node_modules"
    ]
  }
}
```

이는 각 워크트리의 `node_modules/`에서 주 저장소의 복사본으로 다시 심볼릭 링크를 생성하여 디스크에 중복되지 않도록 합니다.

<Note>
  `sparsePaths` 및 `symlinkDirectories` 설정은 워크트리가 생성되기 전에 시작 디렉토리에서 읽습니다. 생성 후 세션의 작업 디렉토리는 시작한 하위 디렉토리가 아닌 워크트리 루트입니다. 따라서 워크트리 내의 프로젝트 설정은 워크트리 루트의 `.claude/settings.json`, 저장소 루트 파일의 체크아웃된 복사본에서 로드됩니다. 권한 규칙 또는 훅과 같이 워크트리 내에서 필요한 다른 설정을 저장소 루트의 `.claude/settings.json`에 넣으십시오.
</Note>

전체 워크트리 설정 참조는 [워크트리 설정](/docs/ko/settings#worktree-settings)을 참조하십시오.

<h3 id="grant-access-across-packages-or-repositories">
  패키지 또는 저장소 전체에 액세스 권한 부여
</h3>

이 섹션은 하위 디렉토리에서 Claude를 시작하거나 작업이 여러 체크아웃에 걸쳐 있을 때 적용됩니다. 단일 대규모 트리에서 저장소 루트에서 시작하면 Claude는 이미 모든 파일에 액세스할 수 있으므로 이를 건너뛸 수 있습니다.

`packages/api/`에서 Claude를 시작하면 해당 디렉토리 내의 파일을 읽고 쓸 수 있습니다. 작업이 패키지 전체에 걸쳐 있으면, 예를 들어 `api`와 `web` 모두 가져오는 공유 타입을 업데이트하면 형제 디렉토리에 액세스 권한을 부여해야 합니다. 동일한 메커니즘은 별도로 체크아웃된 저장소에 액세스 권한을 부여합니다.

`.claude/settings.json`의 `additionalDirectories` 설정은 작업 디렉토리 외부의 디렉토리에 Claude 액세스를 제공합니다. 아래 예제는 두 개의 형제 패키지에 액세스 권한을 부여합니다:

```json .claude/settings.json theme={null}
{
  "permissions": {
    "additionalDirectories": [
      "../shared",
      "../web"
    ]
  }
}
```

상대 경로는 Claude를 시작하는 디렉토리에 대해 해석됩니다. 이 구성으로 Claude는 `packages/api/`에서 작업하면서 `packages/shared/` 및 `packages/web/`의 파일을 읽고 편집할 수 있습니다.

설정을 편집하지 않고 런타임에 Claude를 시작할 때 `--add-dir`을 전달하여 디렉토리에 액세스 권한을 부여할 수도 있습니다:

```bash theme={null}
claude --add-dir ../shared
```

디렉토리를 추가하는 방법에 관계없이 Claude는 파일을 읽고 편집할 수 있습니다. 디렉토리의 CLAUDE.md, `.claude/rules/` 파일, 스킬도 로드되는지는 추가 방법에 따라 다릅니다:

| 추가 방법                            | CLAUDE.md 및 규칙 로드 | 스킬 로드  |
| :------------------------------- | :---------------- | :----- |
| `additionalDirectories` 설정       | 절대 안 함            | 절대 안 함 |
| `--add-dir` 플래그 또는 `/add-dir` 명령 | 아래 환경 변수만         | 예      |

`--add-dir` 또는 `/add-dir`으로 추가된 디렉토리에서 CLAUDE.md 및 규칙 파일을 로드하려면 `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD` 환경 변수를 설정하십시오:

```bash theme={null}
CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1 claude --add-dir ../shared
```

환경 변수는 `additionalDirectories` 설정에 나열된 디렉토리에는 영향을 주지 않습니다. 자세한 내용은 [추가 디렉토리에서 로드](/docs/ko/memory#load-from-additional-directories)를 참조하십시오.

이 영역의 모든 사람이 필요로 하는 형제 디렉토리의 경우 `additionalDirectories`를 `.claude/settings.json`에 커밋하십시오. 개인 선택 또는 일회성 액세스의 경우 `.claude/settings.local.json`을 사용하거나 시작 시 `--add-dir`을 전달하십시오.

<h2 id="add-per-directory-skills">
  디렉토리별 스킬 추가
</h2>

모든 하위 디렉토리는 자신의 스택으로 범위가 지정된 [스킬](/docs/ko/skills)을 정의할 수 있습니다. 스킬은 Claude가 관련성이 있다고 판단할 때 요청 시 로드되므로 API 특정 도구는 프론트엔드 작업 중에 컨텍스트를 소비하지 않습니다.

스킬은 디렉토리 내의 `.claude/skills/` 아래에 있습니다. 해당 영역의 코드와 함께 커밋하여 저장소를 복제하는 모든 사람이 이를 받도록 하십시오. 모노레포에서는 패키지당 하나의 스킬 세트가 될 수 있습니다. 대규모 단일 트리 코드베이스에서는 `src/db/.claude/skills/`와 같은 하위 시스템당 하나입니다.

하위 디렉토리 내에 스킬 디렉토리를 생성하십시오:

```bash theme={null}
mkdir -p packages/api/.claude/skills/api-testing
```

그런 다음 해당 디렉토리 내에 `SKILL.md`를 작성하십시오. 여기서는 `packages/api/.claude/skills/api-testing/SKILL.md`. 이 예제는 Claude에게 API 패키지의 테스트 패턴을 가르칩니다:

```markdown packages/api/.claude/skills/api-testing/SKILL.md theme={null}
---
name: api-testing
description: API 패키지의 테스트 패턴. packages/api/에서 테스트를 작성하거나 수정할 때 사용합니다.
---

## 테스트 구조

테스트는 `src/__tests__/`에 있으며 `src/` 디렉토리 구조를 미러링합니다.
각 경로 파일에는 해당하는 `.test.ts` 파일이 있습니다.

## 테스트 실행

- 모든 테스트: `npm test`
- 단일 파일: `npm test -- src/__tests__/routes/users.test.ts`
- 감시 모드: `npm test -- --watch`

## 테스트 유틸리티

- `src/__tests__/helpers/db.ts`: 데이터베이스 테스트를 위해 `setupTestDb()` 및 `teardownTestDb()` 제공
- `src/__tests__/helpers/auth.ts`: 인증된 엔드포인트를 위해 `createTestUser()` 및 `getAuthToken()` 제공

## 패턴

- 원시 fetch가 아닌 HTTP 어설션에 `supertest` 사용
- 항상 데이터베이스 테스트를 롤백하는 트랜잭션으로 래핑
- `src/__tests__/mocks/`에서 외부 서비스 모킹
```

다른 하위 디렉토리는 같은 방식으로 다른 스킬을 보유합니다: `packages/web/.claude/skills/component-patterns/`는 테스트 대신 프론트엔드의 컴포넌트 규칙을 설명합니다. Claude가 `packages/api/`의 파일에서 작업할 때 api-testing 스킬을 로드합니다. `packages/web/`에서 작업할 때 component-patterns를 로드합니다. 어느 디렉토리의 스킬도 다른 디렉토리의 작업 중에 로드되지 않습니다.

파일 패턴으로 배치 대신 스킬의 범위를 지정할 수도 있습니다. [`paths` 프론트매터 필드](/docs/ko/skills#frontmatter-reference)는 글로브 패턴을 사용하고, Claude는 일치하는 파일로 작업할 때만 자동으로 스킬을 로드합니다. 저장소 루트의 `.claude/skills/`에 있지만 `**/migrations/**`와 같이 어디든 나타나는 특정 파일에만 적용되는 데이터베이스 마이그레이션 스킬과 같은 스킬에 이를 사용하십시오.

스킬 생성 및 구성에 대한 자세한 내용은 [스킬](/docs/ko/skills)을 참조하십시오.

<h3 id="keep-skills-discoverable">
  스킬을 발견 가능하게 유지
</h3>

많은 디렉토리에 분산된 스킬로 Claude가 선택할 수 있는 목록이 커질 수 있습니다. Claude는 발견된 모든 스킬의 이름과 설명을 읽어 스킬을 선택하고, 선택된 스킬의 전체 콘텐츠만 컨텍스트에 로드됩니다. 이 섹션은 해당 목록을 작게 유지하고 단축을 견디는 설명을 작성하는 방법을 다룹니다.

범위 내 스킬은 Claude를 시작하는 위치에 따라 다릅니다:

* **`packages/api/`와 같은 하위 디렉토리에서**: 해당 디렉토리의 스킬, 저장소 루트까지의 모든 상위 항목, 사용자 및 엔터프라이즈 수준
* **저장소 루트에서**: 세션 중에 Claude가 터치하는 모든 하위 디렉토리의 스킬. 수백 개로 누적될 수 있습니다.
* **[`--add-dir`](#grant-access-across-packages-or-repositories)으로 형제 추가 후**: 해당 형제의 스킬도 로드됩니다. `additionalDirectories` 설정은 파일 액세스만 부여하고 스킬을 로드하지 않습니다.

이름은 항상 로드되지만 [많은 스킬이 있을 때 설명이 단축되어](/docs/ko/skills#skill-descriptions-are-cut-short) Claude가 스킬 적용 여부를 결정하는 데 사용하는 키워드를 제거할 수 있습니다. 설명을 짧게 유지하고 요청에 포함될 단어로 시작하십시오. 예를 들어 "`packages/api/`에서 테스트 작성 또는 수정".

많은 디렉토리가 공유하는 스킬(예: PR 규칙 또는 배포 체크리스트)의 경우 저장소 루트의 `.claude/skills/`에 배치하여 모든 시작 디렉토리에서 로드되도록 하십시오. 공유 스킬이 자신의 버전 기록이 필요하거나 저장소 전체에서 작동해야 하면 대신 [플러그인](/docs/ko/plugins)으로 패키징하십시오. 플러그인 스킬은 `plugin-name:skill-name` 네임스페이스를 사용하므로 디렉토리별 스킬과 충돌하지 않습니다. 플랫폼 팀은 한 곳에서 버전 관리하고 업데이트할 수 있습니다.

사용되지 않는 스킬을 찾으려면 OpenTelemetry [로그 내보내기](/docs/ko/monitoring-usage)를 활성화하고 `OTEL_LOG_TOOL_DETAILS=1`을 설정하여 스킬 이름이 수정되지 않고 그대로 기록되도록 하십시오. [`skill_activated` 이벤트](/docs/ko/monitoring-usage#skill-activated-event)는 `skill.name` 속성에 모든 호출을 기록하고, `invocation_trigger`는 명령, Claude, 또는 중첩된 스킬이 호출했는지 기록하여 통합하거나 폐기할 항목을 알려줍니다.

<h2 id="centralize-conventions-when-layering-stops-scaling">
  계층화가 확장을 멈출 때 규칙 중앙화
</h2>

디렉토리별 CLAUDE.md 파일은 코드베이스가 커질수록 관리하기 어려워질 수 있습니다. 규칙이 표류하고, 파일이 오래되고, 아무도 루트를 소유하지 않습니다. 이를 해결하는 것은 일반적으로 저장소의 Claude Code 설정을 유지하는 팀이 자신의 영역에서 작업하는 각 개발자가 아닙니다.

항상 로드되는 CLAUDE.md에서 규칙 및 참조 콘텐츠를 작업과 관련이 있을 때만 로드되는 메커니즘으로 이동하십시오:

* [Skills](/docs/ko/skills): Claude가 작업과 관련이 있을 때만 로드하는 참조 자료
* [Plugins](/docs/ko/plugins): 플랫폼 팀이 중앙에서 소유하는 스킬, 훅, 명령의 버전 관리 번들
* [MCP servers](/docs/ko/mcp): 조직이 이미 저장소에 대한 코드 검색 또는 RAG 인덱스를 실행하면 MCP 도구로 노출하여 Claude가 파일을 직접 읽는 대신 쿼리하도록 합니다.

플랫폼 팀이 이를 중앙에서 적용하는 방법은 [server-managed or endpoint-managed settings](/docs/ko/server-managed-settings#choose-between-server-managed-and-endpoint-managed-settings)을 참조하십시오.

<h3 id="recommend-the-right-plugin-at-session-start">
  세션 시작 시 올바른 플러그인 권장
</h3>

규칙이 플러그인에 있으면 팀원이 트리의 낯선 부분에서 Claude를 시작할 때 해당 영역의 소유자가 유지하는 플러그인에 대한 신호가 없습니다. [`SessionStart` hook](/docs/ko/hooks#sessionstart)은 훅이 stdout에 인쇄하는 모든 것이 첫 번째 프롬프트 전에 Claude의 컨텍스트에 추가되므로 이 간격을 닫을 수 있습니다.

예를 들어 [hook input](/docs/ko/hooks#common-input-fields)에서 시작 디렉토리를 읽고, 저장소에 커밋된 경로-플러그인 맵에서 조회하고, Claude가 첫 번째 응답에서 전달할 권장 사항을 인쇄하는 스크립트를 작성할 수 있습니다. 훅을 작성하고 등록하려면 [Automate actions with hooks](/docs/ko/hooks-guide)를 참조하십시오.

<h2 id="put-it-together">
  모두 함께 사용
</h2>

아래의 결합된 구성은 모노레포 레이아웃을 사용합니다. 동일한 파일은 대규모 단일 트리의 모든 하위 디렉토리에서 작동합니다. 프로젝트 설정은 Claude를 시작하는 디렉토리에서만 로드되므로 각 하위 디렉토리의 `.claude/settings.json`은 루트 파일에 계층화되지 않고 자체 포함되어야 합니다.

예제는 `worktree`, `additionalDirectories`, `Read` 거부 규칙을 `.claude/settings.json`에 커밋하여 `packages/api/`의 모든 개발자가 동일한 형제 액세스, 스파스 경로, 제외를 받도록 합니다. 아래 파일은 `packages/api/`에 대한 커밋된 영역별 설정입니다:

```json packages/api/.claude/settings.json theme={null}
{
  "worktree": {
    "sparsePaths": [
      ".claude",
      "packages/api",
      "packages/shared"
    ],
    "symlinkDirectories": [
      "node_modules"
    ]
  },
  "permissions": {
    "additionalDirectories": [
      "../shared"
    ],
    "deny": [
      "Read(./**/dist/**)",
      "Read(./**/build/**)"
    ]
  }
}
```

이 세션은 `packages/api/`에서 시작되므로 형제 패키지의 CLAUDE.md 파일은 이미 범위를 벗어났으므로 `claudeMdExcludes`가 여기에 필요하지 않습니다. 루트에서도 세션을 시작하면 저장소 루트의 `.claude/settings.local.json`에 추가하십시오.

`additionalDirectories` 항목은 `packages/api/`에서 직접 Claude를 시작할 때 적용됩니다. 이 세션에서 생성된 워크트리 내에서 작업 디렉토리는 워크트리 루트이므로 이 설정 파일은 로드되지 않습니다. 형제 패키지는 이미 워크트리 내에서 도달 가능하지만 거부 규칙은 워크트리 세션이 선택하도록 저장소 루트의 `.claude/settings.json`에 두 번째 복사본이 필요합니다. [워크트리 설정 참고](#check-out-only-the-directories-you-need)에서 설명합니다:

```json .claude/settings.json theme={null}
{
  "permissions": {
    "deny": [
      "Read(./**/dist/**)",
      "Read(./**/build/**)"
    ]
  }
}
```

설정 후 저장소는 이 레이아웃을 가집니다:

```text theme={null}
monorepo/
  CLAUDE.md
  .claude/settings.json                           # 워크트리 세션을 위한 거부 규칙
  packages/
    api/
      CLAUDE.md
      .claude/settings.json                       # 워크트리, additionalDirectories, 거부 규칙
      .claude/skills/api-testing/SKILL.md
    web/
      CLAUDE.md
      .claude/skills/component-patterns/SKILL.md
    shared/
      CLAUDE.md
```

이 설정으로 `packages/api/`에서 Claude를 시작하면:

* 루트 CLAUDE.md와 `packages/api/CLAUDE.md`를 로드하고, `packages/web/CLAUDE.md`를 건너뜁니다.
* `packages/api/` 및 `packages/shared/`의 파일을 읽고 편집할 수 있습니다.
* `packages/api/`의 `dist/` 및 `build/` 아래 빌드 출력 읽기를 건너뜁니다.
* 요청 시 api-testing 스킬을 사용할 수 있습니다.
* `.claude/`, `packages/api/`, `packages/shared/`, 루트 수준 파일을 포함하는 워크트리를 생성하고, 루트 설정 파일에서 워크트리 전체에 거부 규칙을 적용합니다.

<h2 id="scope-and-plan-changes-that-span-packages">
  패키지에 걸친 변경 범위 지정 및 계획
</h2>

위의 구성은 Claude가 보는 것을 제어합니다. 단일 변경이 여러 패키지에 영향을 미칠 때, 예를 들어 공유 타입을 업데이트하고 이를 사용하는 모든 호출 사이트를 업데이트할 때, 작업의 범위 지정 및 순서 지정 방식도 결과에 영향을 미칩니다.

두 가지 기법은 패키지 간 변경을 일관되게 유지하는 데 도움이 됩니다:

* **전체 변경을 한 세션에서 Claude에 제공**: 공유 편집과 호출 사이트를 함께 전달하면 각 편집 뒤의 결정이 일관되게 유지되어 패키지별로 다시 도출하지 않습니다.
* **편집 전에 계획을 파일에 저장**: [먼저 계획](/docs/ko/best-practices#explore-first-then-plan-then-code)하고 Claude에게 계획을 저장소의 마크다운 파일에 작성하도록 요청하십시오. 긴 패키지 간 세션은 진행 중에 [컨텍스트를 압축](/docs/ko/context-window#what-survives-compaction)하고, 저장된 계획은 대화 기록이 없을 수 있는 곳에서 생존합니다.

<h2 id="next-steps">
  다음 단계
</h2>

이 구성이 준비되면 이를 개선할 수 있습니다:

* [훅](/docs/ko/hooks-guide)을 사용하여 Claude가 파일을 편집한 후 디렉토리별 린터 또는 타입 체커 실행
* [비용 효과적으로 관리](/docs/ko/costs)를 검토하여 코드베이스 크기가 토큰 사용에 어떻게 영향을 미치는지, 더 넓은 배포 전에 지출 한도를 설정하는 방법을 이해합니다.
* Claude 블로그의 [Claude Code가 대규모 코드베이스에서 어떻게 작동하는지](https://claude.com/blog/how-claude-code-works-in-large-codebases-best-practices-and-where-to-start)를 읽어 조직 배포 패턴 및 이 페이지의 저장소별 구성 위에 있는 소유권 모델을 확인합니다.
