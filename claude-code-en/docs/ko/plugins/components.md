> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 플러그인에 컴포넌트 추가하기

> Claude Code 플러그인에 skills, hooks, MCP 서버 및 다른 모든 컴포넌트 유형을 추가하고, 각각에 대해 검증하는 예제를 포함합니다.

export const Piece = ({id, children}) => <div className="pe-piece" data-piece={id}>{children}</div>;

export const PluginExplorer = ({children}) => {
  const PIECES = [{
    id: 'manifest',
    name: 'Manifest',
    path: '.claude-plugin/plugin.json',
    required: "Required by Anthropic's directory",
    lines: [{
      depth: 0,
      kind: 'folder',
      text: '.claude-plugin/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'plugin.json'
    }],
    href: '/en/plugins/manifest-reference#manifest-file',
    linkText: 'Go to the manifest reference'
  }, {
    id: 'skills',
    name: 'Skills',
    path: 'skills/review/SKILL.md',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'skills/'
    }, {
      depth: 1,
      kind: 'folder',
      text: 'review/'
    }, {
      depth: 2,
      kind: 'file',
      text: 'SKILL.md'
    }],
    href: '/en/plugins/components#skills',
    linkText: 'Go to the Skills section'
  }, {
    id: 'commands',
    name: 'Commands',
    path: 'commands/about.md',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'commands/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'about.md'
    }],
    href: '/en/plugins/components#commands',
    linkText: 'Go to the Commands section'
  }, {
    id: 'agents',
    name: 'Agents',
    path: 'agents/security-reviewer.md',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'agents/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'security-reviewer.md'
    }],
    href: '/en/plugins/components#agents',
    linkText: 'Go to the Agents section'
  }, {
    id: 'hooks',
    name: 'Hooks',
    path: 'hooks/hooks.json',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'hooks/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'hooks.json'
    }],
    href: '/en/plugins/components#hooks',
    linkText: 'Go to the Hooks section'
  }, {
    id: 'monitors',
    name: 'Monitors',
    path: 'monitors/monitors.json',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'monitors/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'monitors.json'
    }],
    href: '/en/plugins/components#monitors',
    linkText: 'Go to the Monitors section'
  }, {
    id: 'output-styles',
    name: 'Output styles',
    path: 'output-styles/terse.md',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'output-styles/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'terse.md'
    }],
    href: '/en/plugins/components#themes-and-output-styles',
    linkText: 'Go to the Themes and output styles section'
  }, {
    id: 'themes',
    name: 'Themes',
    path: 'themes/dracula.json',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'themes/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'dracula.json'
    }],
    href: '/en/plugins/components#themes-and-output-styles',
    linkText: 'Go to the Themes and output styles section'
  }, {
    id: 'workflows',
    name: 'Workflows',
    path: 'workflows/audit-routes.js',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'workflows/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'audit-routes.js'
    }],
    href: '/en/workflows#distribute-a-workflow-in-a-plugin',
    linkText: 'Go to Distribute a workflow in a plugin'
  }, {
    id: 'bin',
    name: 'Executables',
    path: 'bin/hello-plugin',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'bin/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'hello-plugin'
    }],
    href: '/en/plugins/components#executables',
    linkText: 'Go to the Executables section'
  }, {
    id: 'scripts',
    name: 'Scripts',
    path: 'scripts/format.sh',
    lines: [{
      depth: 0,
      kind: 'folder',
      text: 'scripts/'
    }, {
      depth: 1,
      kind: 'file',
      text: 'format.sh'
    }],
    href: '/en/plugins/components#hooks',
    linkText: 'Go to the Hooks section'
  }, {
    id: 'settings',
    name: 'Default settings',
    path: 'settings.json',
    lines: [{
      depth: 0,
      kind: 'file',
      text: 'settings.json'
    }],
    href: '/en/plugins/components#default-settings',
    linkText: 'Go to the Default settings section'
  }, {
    id: 'mcp',
    name: 'MCP servers',
    path: '.mcp.json',
    lines: [{
      depth: 0,
      kind: 'file',
      text: '.mcp.json'
    }],
    href: '/en/plugins/components#mcp-servers',
    linkText: 'Go to the MCP servers section'
  }, {
    id: 'lsp',
    name: 'LSP servers',
    path: '.lsp.json',
    lines: [{
      depth: 0,
      kind: 'file',
      text: '.lsp.json'
    }],
    href: '/en/plugins/components#lsp-servers',
    linkText: 'Go to the LSP servers section'
  }];
  const [selectedId, setSelectedId] = useState('manifest');
  const [isFullscreen, setIsFullscreen] = useState(false);
  const rootRef = useRef(null);
  useEffect(() => {
    const onFsChange = () => setIsFullscreen(!!document.fullscreenElement);
    document.addEventListener('fullscreenchange', onFsChange);
    return () => document.removeEventListener('fullscreenchange', onFsChange);
  }, []);
  const toggleFullscreen = () => {
    if (!rootRef.current) return;
    if (document.fullscreenElement) document.exitFullscreen(); else rootRef.current.requestFullscreen().catch(() => {});
  };
  const selected = PIECES.find(p => p.id === selectedId) || PIECES[0];
  const onTreeKeyDown = e => {
    const keys = ['ArrowDown', 'ArrowUp', 'Home', 'End'];
    if (keys.indexOf(e.key) === -1) return;
    const i = PIECES.findIndex(p => p.id === selectedId);
    let next = i;
    if (e.key === 'ArrowDown') next = Math.min(PIECES.length - 1, i + 1);
    if (e.key === 'ArrowUp') next = Math.max(0, i - 1);
    if (e.key === 'Home') next = 0;
    if (e.key === 'End') next = PIECES.length - 1;
    e.preventDefault();
    if (next === i) return;
    const id = PIECES[next].id;
    setSelectedId(id);
    const el = document.getElementById('pe-node-' + id);
    if (el) el.focus();
  };
  const FolderIcon = () => <svg className="pe-icon" width="15" height="15" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.3" strokeLinejoin="round" aria-hidden="true">
      <path d="M1.5 4.5a1 1 0 0 1 1-1h3.2l1.3 1.5h6a1 1 0 0 1 1 1V12a1 1 0 0 1-1 1h-10.5a1 1 0 0 1-1-1z" />
    </svg>;
  const FileIcon = () => <svg className="pe-icon" width="15" height="15" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.3" strokeLinejoin="round" aria-hidden="true">
      <path d="M4 1.5h5.5L13 5v9.5H4z" />
      <path d="M9.5 1.5V5H13" />
    </svg>;
  return <div ref={rootRef} className={isFullscreen ? 'pe-root pe-fullscreen not-prose' : 'pe-root not-prose'} data-selected={selected.id}>
      <style>{`
        .pe-root {
          --pe-mono: var(--font-mono, ui-monospace, SFMono-Regular, Menlo, monospace);
          --pe-accent: #D97757;
          --pe-accent-text: #A8502F;
          --pe-accent-bg: rgba(217,119,87,0.10);
          --pe-bg: #FFFFFF;
          --pe-surface: #FAFAF7;
          --pe-hover: #F0EEE6;
          --pe-border: #E8E6DC;
          --pe-text: #141413;
          --pe-text-2: #3D3D3A;
          --pe-text-3: #5E5D59;
          font-family: inherit;
          background: var(--pe-bg);
          color: var(--pe-text);
          border: 1px solid var(--pe-border);
          border-radius: 12px;
          margin: 1.5rem 0;
          overflow: hidden;
          box-sizing: border-box;
        }
        .dark .pe-root {
          --pe-accent-text: #EBA98F;
          --pe-accent-bg: rgba(217,119,87,0.18);
          --pe-bg: #1A1918;
          --pe-surface: #232221;
          --pe-hover: #2E2D2B;
          --pe-border: #3A3936;
          --pe-text: #F1EFE9;
          --pe-text-2: #D6D4CA;
          --pe-text-3: #B8B5AD;
        }
        .pe-root *, .pe-root *::before, .pe-root *::after { box-sizing: border-box; }
        .pe-head { display: flex; align-items: flex-start; gap: 12px; padding: 18px 24px 16px; border-bottom: 1px solid var(--pe-border); }
        .pe-head-text { flex: 1; min-width: 0; }
        .pe-fs-btn { flex-shrink: 0; width: 32px; height: 32px; display: inline-flex; align-items: center; justify-content: center; border: 1px solid var(--pe-border); border-radius: 6px; background: var(--pe-surface); color: var(--pe-text-2); font-size: 15px; line-height: 1; cursor: pointer; }
        .pe-fs-btn:hover { background: var(--pe-hover); }
        .pe-fs-btn:focus-visible { outline: 2px solid var(--pe-accent); outline-offset: 2px; }
        .pe-fullscreen { border-radius: 0; height: 100vh; display: flex; flex-direction: column; overflow: auto; }
        .pe-fullscreen .pe-body { flex: 1; }
        .pe-title { font-size: 19px; font-weight: 600; line-height: 1.3; color: var(--pe-text); margin: 0; }
        .pe-sub { font-size: 15px; line-height: 1.5; color: var(--pe-text-3); margin: 4px 0 0; }
        .pe-sub code { font-family: var(--pe-mono); font-size: 0.88em; padding: 1px 5px; border-radius: 4px; background: var(--pe-surface); border: 1px solid var(--pe-border); }
        .pe-body { display: flex; align-items: stretch; }
        .pe-tree-pane { width: 270px; flex-shrink: 0; background: var(--pe-surface); border-right: 1px solid var(--pe-border); padding: 16px 0 12px; }
        .pe-panel { flex: 1; min-width: 0; padding: 16px 24px 24px; }
        .pe-caption { font-size: 13px; font-weight: 600; color: var(--pe-text-3); margin: 0 0 10px; }
        .pe-tree-pane .pe-caption { padding: 0 16px; }
        .pe-rootline { display: flex; align-items: center; gap: 7px; padding: 3px 16px; font-family: var(--pe-mono); font-size: 13.5px; color: var(--pe-text-3); }
        .pe-node {
          display: block; width: 100%; margin: 0; padding: 3px 16px 3px 30px; text-align: left; cursor: pointer;
          background: transparent; color: var(--pe-text-2);
          border: none; border-left: 3px solid transparent;
          font-family: var(--pe-mono); font-size: 13.5px; line-height: 1.4;
        }
        .pe-node:hover { background: var(--pe-hover); }
        .pe-node:focus-visible { outline: 2px solid var(--pe-accent); outline-offset: -2px; }
        .pe-node[aria-pressed="true"] { background: var(--pe-accent-bg); border-left-color: var(--pe-accent); color: var(--pe-accent-text); font-weight: 600; }
        .pe-line { display: flex; align-items: center; gap: 7px; padding: 2px 0; }
        .pe-line-tree { flex-wrap: wrap; }
        .pe-line-tree .pe-req { flex-basis: 100%; margin: 2px 0 0 22px; white-space: normal; width: fit-content; max-width: calc(100% - 22px); }
        .pe-line span { overflow-wrap: anywhere; }
        .pe-piece { display: none; font-size: 16px; line-height: 1.6; color: var(--pe-text-2); }
        .pe-root[data-selected="manifest"] .pe-piece[data-piece="manifest"],
        .pe-root[data-selected="skills"] .pe-piece[data-piece="skills"],
        .pe-root[data-selected="commands"] .pe-piece[data-piece="commands"],
        .pe-root[data-selected="agents"] .pe-piece[data-piece="agents"],
        .pe-root[data-selected="hooks"] .pe-piece[data-piece="hooks"],
        .pe-root[data-selected="monitors"] .pe-piece[data-piece="monitors"],
        .pe-root[data-selected="output-styles"] .pe-piece[data-piece="output-styles"],
        .pe-root[data-selected="themes"] .pe-piece[data-piece="themes"],
        .pe-root[data-selected="workflows"] .pe-piece[data-piece="workflows"],
        .pe-root[data-selected="bin"] .pe-piece[data-piece="bin"],
        .pe-root[data-selected="scripts"] .pe-piece[data-piece="scripts"],
        .pe-root[data-selected="settings"] .pe-piece[data-piece="settings"],
        .pe-root[data-selected="mcp"] .pe-piece[data-piece="mcp"],
        .pe-root[data-selected="lsp"] .pe-piece[data-piece="lsp"] { display: block; }
        .pe-piece p { margin: 0 0 10px; }
        .pe-piece p:last-child { margin-bottom: 0; }
        .pe-piece code { font-family: var(--pe-mono); font-size: 0.88em; padding: 1px 5px; border-radius: 4px; background: var(--pe-surface); border: 1px solid var(--pe-border); }
        .pe-piece .code-block { margin: 12px 0 0; }
        .pe-piece pre code { padding: 0; border: none; background: none; }
        .pe-piece a { color: var(--pe-accent-text); }
        .pe-line-compact { display: none; }
        .pe-icon { flex-shrink: 0; }
        .pe-req { margin-left: 8px; padding: 0 6px; border-radius: 999px; font-size: 11px; line-height: 18px; letter-spacing: .02em; color: var(--pe-accent-text); border: 1px solid var(--pe-border); background: var(--pe-surface); white-space: nowrap; font-weight: 500; vertical-align: middle; }
        .pe-name { font-size: 22px; font-weight: 600; line-height: 1.25; letter-spacing: -0.2px; color: var(--pe-text); margin: 0; }
        .pe-path { font-family: var(--pe-mono); font-size: 13.5px; color: var(--pe-accent-text); margin: 4px 0 0; overflow-wrap: anywhere; }
        .pe-block { margin: 20px 0 0; }
        .pe-link {
          display: inline-block; margin: 24px 0 0; padding: 8px 14px; border-radius: 8px;
          font-size: 14.5px; font-weight: 600; text-decoration: none;
          color: var(--pe-accent-text); background: var(--pe-accent-bg); border: 1px solid var(--pe-accent);
        }
        .pe-link:hover { filter: brightness(0.97); }
        .pe-link:focus-visible { outline: 2px solid var(--pe-accent); outline-offset: 2px; }
        @media (max-width: 700px) {
          .pe-head { padding: 16px 16px 14px; }
          .pe-body { flex-direction: column; }
          .pe-tree-pane { width: 100%; border-right: none; border-bottom: 1px solid var(--pe-border); }
          .pe-line-tree { display: none; }
          .pe-line-compact { display: flex; }
          .pe-panel { padding: 16px 16px 20px; }
        }
      `}</style>

      <div className="pe-head">
        <div className="pe-head-text">
          <div className="pe-title">What goes in a plugin</div>
          <div className="pe-sub">This example plugin, <code>my-plugin</code>, has one of every kind of component, each in its default location. Select a file or folder to read what it’s for and see what goes in it.</div>
        </div>
        <button type="button" className="pe-fs-btn" onClick={toggleFullscreen} aria-label={isFullscreen ? 'Exit fullscreen' : 'Fullscreen'} title={isFullscreen ? 'Exit fullscreen' : 'Fullscreen'}>
          {isFullscreen ? '⤡' : '⛶'}
        </button>
      </div>

      <div className="pe-body">
        <div className="pe-tree-pane">
          <div className="pe-caption" id="pe-tree-caption">Plugin directory</div>
          <div role="group" aria-labelledby="pe-tree-caption" onKeyDown={onTreeKeyDown}>
            <div className="pe-rootline"><FolderIcon /><span>my-plugin/</span></div>
            {PIECES.map(p => <button key={p.id} id={'pe-node-' + p.id} type="button" className="pe-node" aria-pressed={p.id === selected.id} aria-label={p.name + ', ' + p.path} onClick={() => setSelectedId(p.id)}>
                {p.lines.map((line, i) => <span key={i} className="pe-line pe-line-tree" style={{
    paddingLeft: line.depth * 18 + 'px'
  }}>
                    {line.kind === 'folder' ? <FolderIcon /> : <FileIcon />}
                    <span>{line.text}</span>
                    {p.required && i === p.lines.length - 1 ? <span className="pe-req">{p.required}</span> : null}
                  </span>)}
                <span className="pe-line pe-line-compact">
                  <FileIcon />
                  <span>{p.path}</span>
                  {p.required ? <span className="pe-req">{p.required}</span> : null}
                </span>
              </button>)}
          </div>
        </div>

        <div className="pe-panel" role="region" aria-labelledby="pe-panel-caption" aria-live="polite" aria-atomic="true">
          <div className="pe-caption" id="pe-panel-caption">Selected piece</div>
          <div className="pe-name">{selected.name}{selected.required ? <span className="pe-req">{selected.required}</span> : null}</div>
          <div className="pe-path">{selected.path}</div>

          <div className="pe-block">{children}</div>

          <a className="pe-link" href={selected.href}>{selected.linkText}</a>
        </div>
      </div>
    </div>;
};

Claude Code 플러그인은 skills, agents, hooks, MCP 서버와 같은 컴포넌트로 구성됩니다. 각 컴포넌트는 플러그인의 기본 폴더, `.claude-plugin/plugin.json`의 선택적 manifest 키(해당 폴더를 대체하거나 추가함), 그리고 사용자가 보는 이름을 가집니다. 각 키의 전체 필드 테이블은 [manifest 참조](/docs/ko/plugins/manifest-reference#fields)를 참조하십시오.

이 페이지를 사용하여 이미 로드되는 플러그인에 컴포넌트를 추가합니다.

컴포넌트를 추가한 후, 실행 중인 세션에서 `/reload-plugins`를 실행하거나 새 세션을 시작하여 Claude Code가 이를 로드하도록 합니다. 로드하기 전에 컴포넌트의 파일을 확인하려면 플러그인 디렉토리에서 셸에서 [`claude plugin validate .`](/docs/ko/plugins/cli-reference#plugin-validate)를 실행합니다.

<Note>
  다음 경우는 다른 페이지에서 다룹니다:

  * **첫 번째 플러그인 구축**: [플러그인 생성](/docs/ko/plugins/create)으로 시작합니다
  * **다른 사람의 플러그인 설치**: [플러그인 설치](/docs/ko/plugins/install)를 참조합니다
  * **플러그인의 사용자가 claude.ai 또는 Cowork에 있음**: 다른 컴포넌트 세트가 로드됩니다. [claude.ai 및 Cowork의 플러그인](https://claude.com/docs/plugins/overview)을 참조합니다
</Note>

<h2 id="explore-the-plugin-directory">
  플러그인 디렉토리 탐색
</h2>

탐색기는 기본 위치에 모든 종류의 컴포넌트 하나씩을 가진 예제 플러그인 `my-plugin`을 보여줍니다:

* review skill과 `about` 명령
* security-review 서브에이전트
* Claude가 파일을 편집한 후 파일을 포맷하는 hook, 그리고 이를 호출하는 `scripts/` 폴더
* 로그 모니터
* 출력 스타일과 색상 테마
* route-audit 워크플로우
* `hello-plugin` 실행 파일
* 기본 설정
* 로컬 MCP 서버와 Go 언어 서버

각 파일은 유용하기보다는 형태를 보여주기 위한 형식의 최소 유효 예제입니다: 실제 skill이나 agent는 전체 지침을 포함하고 종종 지원 파일을 가지며, 실제 hook이나 모니터는 실제 작업을 수행합니다. 탐색기 이후의 섹션은 예제로 동일한 파일을 사용하고 더 완전한 파일로 연결됩니다. 파일이나 폴더를 선택하여 그것이 무엇인지 읽고, 그 안에 무엇이 들어가는지 보고, 그것을 다루는 섹션을 찾습니다.

<PluginExplorer>
  <Piece id="manifest">
    [manifest](/docs/ko/plugins/manifest-reference)는 플러그인의 `.claude-plugin/` 디렉토리에 있는 `plugin.json` 파일입니다. 플러그인의 메타데이터와 Claude Code가 사용자에게 프롬프트하는 `userConfig` 값을 포함합니다. `name`만 필수입니다. 이 파일에서 `description`은 사용자가 `/plugin`에서 플러그인에 대해 보는 텍스트이고, `version`은 변경할 때까지 사용자를 해당 버전에 유지합니다:

    ```json theme={null}
    {
      "name": "my-plugin",
      "version": "1.0.0",
      "description": "Review, formatting, and database tools for this team"
    }
    ```
  </Piece>

  <Piece id="skills">
    [skill](/docs/ko/skills)은 `SKILL.md` 파일입니다. 각 skill을 `skills/` 아래의 자신의 디렉토리에 저장합니다. Claude는 모든 skill의 `description`을 읽고, 사용자가 요청하는 것이 이를 일치할 때(예: 여기서 pull request를 검토하도록 Claude에 요청), Claude는 skill의 지침을 로드하고 따릅니다. 사용자는 또한 `/my-plugin:review`로 직접 실행할 수 있습니다:

    ```markdown theme={null}
    ---
    description: Reviews a pull request for style and test coverage. Use when asked to review code.
    ---

    Review the changed files. Report style problems first, then missing tests.
    ```
  </Piece>

  <Piece id="commands">
    명령은 사용자가 이름으로 실행하는 단일 Markdown 파일입니다. 명령은 이전 형식입니다: skill은 같은 방식으로 이름으로 실행되고 자신의 디렉토리에 지원 파일을 포함할 수도 있으므로, 새로운 것은 skill로 작성하고 이미 가진 파일에 대해 `commands/`를 유지합니다. 이 파일은 `/my-plugin:about`이 되고 skill과 동일한 frontmatter를 사용합니다:

    ```markdown theme={null}
    ---
    description: Summarize the repository
    ---

    Summarize what this repository does in three sentences.
    ```
  </Piece>

  <Piece id="agents">
    [서브에이전트](/docs/ko/sub-agents)는 자신의 지침과 자신의 컨텍스트 윈도우를 가진 별도의 어시스턴트로, Claude가 작업을 위임하고 결과를 다시 받을 수 있습니다. `agents/` 아래의 각 Markdown 파일은 하나를 정의합니다: frontmatter는 이를 이름 지정하고 사용 시기를 말하고, 본문은 시스템 프롬프트입니다. 이 파일은 `my-plugin:security-reviewer`로 이름 지정되고, 사용자는 `@agent-my-plugin:security-reviewer`로 호출할 수 있습니다:

    ```markdown theme={null}
    ---
    name: security-reviewer
    description: Reviews code changes for security issues. Use after edits to authentication or input handling.
    model: sonnet
    ---

    You are a security reviewer. Read the changed files and report injection, authentication, and secrets-handling risks.
    ```
  </Piece>

  <Piece id="hooks">
    [hook](/docs/ko/hooks-guide)은 Claude Code의 라이프사이클의 한 지점(예: 모든 파일 편집 후)에서 자동으로 무언가를 실행합니다: 셸 명령, HTTP 요청, MCP 도구 호출, 모델에 대한 프롬프트, 또는 서브에이전트. 플러그인의 hooks를 플러그인 루트의 `hooks/hooks.json`에 저장합니다. 이 파일은 Claude가 파일을 쓰거나 편집한 후 플러그인의 `scripts/format.sh`를 실행합니다:

    ```json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [
              {
                "type": "command",
                "command": "\"${CLAUDE_PLUGIN_ROOT}/scripts/format.sh\""
              }
            ]
          }
        ]
      }
    }
    ```
  </Piece>

  <Piece id="monitors">
    모니터는 Claude Code가 세션 시작 시 백그라운드에서 시작하고 끝날 때까지 계속 실행하는 셸 명령으로, [Monitor 도구](/docs/ko/tools-reference#monitor-tool)를 사용합니다. 이것이 인쇄하는 것은 Claude에 알림으로 도달합니다. `when` 필드는 대신 명명된 skill이 처음 실행될 때 시작할 수 있습니다. 이 파일은 오류 로그를 추적합니다:

    ```json theme={null}
    [
      {
        "name": "error-log",
        "command": "tail -F ./logs/error.log",
        "description": "Application error log"
      }
    ]
    ```
  </Piece>

  <Piece id="output-styles">
    플러그인은 [출력 스타일](/docs/ko/output-styles)을 포함할 수 있으며, 이는 Claude가 응답을 포맷하고 표현하는 방식을 변경합니다. 각 출력 스타일을 `output-styles/<name>.md`로 저장합니다. 이 파일은 `/output-style`에 `my-plugin:terse`로 나타납니다:

    ```markdown theme={null}
    ---
    name: terse
    description: Answer in as few words as possible
    keep-coding-instructions: true
    ---

    Keep every reply short. Skip preambles and summaries.
    ```
  </Piece>

  <Piece id="themes">
    플러그인은 Claude Code 인터페이스의 [색상 테마](/docs/ko/terminal-config#create-a-custom-theme)를 포함할 수 있습니다. 각 테마를 `themes/<slug>.json`으로 저장합니다. 이 파일은 `/theme`에 `Dracula`로 나타나고, `my-plugin`에서 표시됩니다:

    ```json theme={null}
    {
      "name": "Dracula",
      "base": "dark",
      "overrides": {
        "claude": "#bd93f9",
        "error": "#ff5555"
      }
    }
    ```
  </Piece>

  <Piece id="workflows">
    `workflows/` 폴더는 [워크플로우](/docs/ko/workflows) `.js` 파일을 보유합니다: `meta` 블록, 그 다음 여러 서브에이전트를 조율하는 스크립트 본문. 이 파일은 `/my-plugin:audit-routes`로 실행됩니다:

    ```javascript theme={null}
    export const meta = {
      name: 'audit-routes',
      description: 'Audit every route handler for missing auth checks',
    }

    const found = await agent('List every .ts file under src/routes/.', {
      schema: { type: 'object', required: ['files'], properties: { files: { type: 'array', items: { type: 'string' } } } },
    })

    const audits = await pipeline(found.files, file =>
      agent(`Audit ${file} for missing authentication checks.`, { label: file }),
    )

    return audits.filter(Boolean)
    ```
  </Piece>

  <Piece id="bin">
    `bin/`은 플러그인이 명령줄 도구를 제공하는 방법입니다. 플러그인이 활성화되어 있는 동안, Claude Code는 이 폴더를 실행하는 셸의 `PATH`에 넣으므로, Claude 또는 skill의 지침이 사용자가 아무것도 설치하지 않고도 이름으로 도구를 실행할 수 있습니다. 이 [실행 파일](#executables)이 있으면, `hello-plugin`은 Claude가 실행할 수 있는 명령입니다:

    ```bash theme={null}
    #!/bin/bash
    echo "hello from my-plugin"
    ```
  </Piece>

  <Piece id="scripts">
    `hooks/hooks.json`의 hook은 스크립트를 실행하고, 이 폴더는 예제가 이를 유지하는 곳입니다. `scripts/` 이름은 관례이지 Claude Code가 찾는 것이 아닙니다: hook은 파일을 경로로 가리킵니다, `${CLAUDE_PLUGIN_ROOT}/scripts/format.sh`. 포매터 스크립트는 다음과 같을 수 있습니다:

    ```bash theme={null}
    #!/bin/bash
    npx prettier --write .
    ```
  </Piece>

  <Piece id="settings">
    플러그인 루트의 `settings.json`은 플러그인이 활성화되어 있는 동안 적용되는 [설정](/docs/ko/settings-reference)을 보유하므로, 플러그인은 세션의 동작 방식을 변경할 수 있고 컴포넌트만 추가하는 것이 아닙니다. 플러그인에서 효과를 발휘하는 두 개의 키만 있습니다, [`agent`](/docs/ko/settings-reference#agent)와 [`subagentStatusLine`](/docs/ko/settings-reference#subagentstatusline); 다른 모든 키는 삭제됩니다. [기본 설정](#default-settings)을 참조합니다.

    이 파일은 `agent`를 설정하여, 세션의 주 스레드를 플러그인의 자신의 `security-reviewer` 에이전트로 실행하므로, 해당 에이전트의 시스템 프롬프트, 도구 제한, 모델이 전체 세션에 적용됩니다:

    ```json theme={null}
    {
      "agent": "security-reviewer"
    }
    ```
  </Piece>

  <Piece id="mcp">
    [MCP 서버](/docs/ko/mcp)는 Claude에 외부 시스템의 도구를 제공합니다. 플러그인 루트의 `.mcp.json`에서 선언합니다. 이 파일은 플러그인 내부의 스크립트에서 로컬 서버를 시작하고, `/mcp`에 `plugin:my-plugin:db`로 나타납니다:

    ```json theme={null}
    {
      "mcpServers": {
        "db": {
          "command": "node",
          "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"]
        }
      }
    }
    ```
  </Piece>

  <Piece id="lsp">
    LSP 서버는 Claude에 언어에 대한 [진단 및 코드 네비게이션](/docs/ko/plugins/code-intelligence)을 제공합니다. 플러그인 루트의 `.lsp.json`에서 서버를 선언합니다. 이 파일은 `.go` 파일에 대해 Go 언어 서버를 연결합니다:

    ```json theme={null}
    {
      "gopls": {
        "command": "gopls",
        "args": ["serve"],
        "extensionToLanguage": {
          ".go": "go"
        }
      }
    }
    ```
  </Piece>
</PluginExplorer>

<h2 id="add-each-kind-of-component">
  각 종류의 컴포넌트 추가
</h2>

아래의 각 섹션은 한 종류의 컴포넌트를 다룹니다: 플러그인에서 파일이 어디로 가는지, 검증하는 예제, 플러그인이 로드된 후 사용자가 보는 것, 기본 위치를 변경하는 manifest 키. 플러그인이 필요한 것들을 추가합니다; 아무것도 필수가 아닙니다.

<h3 id="skills">
  Skills
</h3>

[skill](/docs/ko/skills)은 설명이 작업과 일치할 때 Claude가 로드할 수 있는 `SKILL.md` 파일입니다. 사용자는 또한 명령으로 실행할 수 있습니다. 각 skill을 `skills/` 아래의 자신의 디렉토리에 저장합니다:

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json
└── skills/
    └── review/
        └── SKILL.md
```

`SKILL.md`에 `description`을 제공하여 Claude가 언제 사용할지 알 수 있도록 합니다:

```markdown skills/review/SKILL.md theme={null}
---
description: Reviews a pull request for style and test coverage. Use when asked to review code.
---

Review the changed files. Report style problems first, then missing tests.
```

플러그인을 로드한 후, `/my-plugin:review`는 skill을 실행합니다. 명령 이름과 누가 호출할 수 있는지는 다음 규칙을 따릅니다:

* **명령 이름**: `/<plugin>:<directory>`, 따라서 `my-plugin`의 `skills/review/SKILL.md`는 `/my-plugin:review`입니다. frontmatter에서 `name`을 설정하면, 마지막 세그먼트를 대체하고 플러그인 접두사는 유지됩니다. [skill이 명령 이름을 얻는 방법](/docs/ko/skills#how-a-skill-gets-its-command-name)을 참조합니다
* **누가 호출하는가**: Claude, 사용자, 또는 둘 다, frontmatter로 제어됩니다. [skill 호출을 제어하는 사람](/docs/ko/skills#control-who-invokes-a-skill)을 참조합니다

기본 `skills/` 디렉토리 외부에 skills를 배치할 수도 있습니다:

* **추가 디렉토리**: `skills` manifest 키에 나열합니다. 이들은 `commands`와 `agents`와 달리 기본 `skills/` 스캔을 대체하지 않고 추가합니다
* **플러그인 루트의 단일 skill**: `skills/` 디렉토리가 없고 `skills` manifest 키가 없으면, 플러그인 루트의 `SKILL.md`는 하나의 skill로 로드됩니다. frontmatter에서 `name`을 설정합니다, 그렇지 않으면 마켓플레이스 설치가 플러그인 이름이 아닌 [캐시 디렉토리](/docs/ko/plugins/loading#find-plugins-on-disk) 이름으로 skill을 이름 지정합니다

플러그인에 지침을 포함하려면, 이를 skill로 작성합니다. Claude Code는 플러그인 루트의 `CLAUDE.md`를 로드하지 않으며, `claude plugin validate`는 `CLAUDE.md at the plugin root is not loaded as project context` 경고를 표시합니다.

frontmatter 필드 및 지원 파일의 경우, [Skills](/docs/ko/skills)를 참조합니다.

<h3 id="commands">
  명령
</h3>

명령은 사용자가 이름으로 실행하는 단일 Markdown 파일입니다(예: `/my-plugin:about`).

<Note>
  명령은 이전 형식이며, [skills](#skills)는 새로운 작업을 위해 이를 대체합니다. skill은 같은 방식으로 이름으로 실행되고, 디렉토리에 지원 파일을 포함할 수도 있습니다. `.claude/commands/`에서 이동하는 파일에 대해 `commands/`를 유지합니다.
</Note>

`commands/<file>.md`에 명령을 저장하면 `/<plugin>:<file>`이 됩니다. 서브디렉토리는 세그먼트를 추가하므로, `commands/db/migrate.md`는 `/my-plugin:db:migrate`입니다.

명령 파일은 skills와 동일한 frontmatter를 사용합니다.

<h4 id="define-commands-in-the-manifest">
  manifest에서 명령 정의
</h4>

명령 파일을 `commands/` 이외의 다른 곳에 유지하거나, 별도의 Markdown 파일 없이 `plugin.json` 내부에 짧은 명령을 정의하려는 경우에만 필요합니다. `commands` manifest 키를 설정하면, Claude Code는 `commands/`를 스캔하는 대신 이를 읽습니다. 키는 경로, 경로 배열, 또는 각 명령 이름을 `source` 파일 또는 인라인 `content`로 매핑하는 객체를 사용합니다.

이 manifest는 `/my-plugin:about`을 인라인으로 정의하며, Markdown 파일이 없습니다:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "my-plugin",
  "commands": {
    "about": {
      "content": "Summarize what this repository does in three sentences.",
      "description": "Summarize the repository"
    }
  }
}
```

플러그인을 로드하고 세션에서 `/my-plugin:about`을 실행하여 로드되었는지 확인합니다.

전체 키 구문의 경우, [`commands`](/docs/ko/plugins/manifest-reference#commands)를 참조합니다.

<h3 id="agents">
  Agents
</h3>

[서브에이전트](/docs/ko/sub-agents)는 자신의 지침과 컨텍스트 윈도우를 가진 별도의 어시스턴트로, Claude가 작업을 위임할 수 있습니다. `agents/` 아래의 각 Markdown 파일은 하나를 정의합니다:

```markdown agents/security-reviewer.md theme={null}
---
name: security-reviewer
description: Reviews code changes for security issues. Use after edits to authentication or input handling.
model: sonnet
---

You are a security reviewer. Read the changed files and report injection, authentication, and secrets-handling risks.
```

이 에이전트는 `my-plugin:security-reviewer`로 이름 지정되고, 사용자는 `@agent-my-plugin:security-reviewer`로 [명시적으로 호출](/docs/ko/sub-agents#invoke-subagents-explicitly)할 수 있습니다. 이름 형식은 `<plugin>:<name>`이며, `<name>`은 frontmatter에서 오거나 없을 때 파일 이름에서 옵니다.

`agents` manifest 키는 `agents/` 스캔을 대체합니다.

<h4 id="organize-agents-in-subfolders">
  agents의 서브폴더에서 agents 구성
</h4>

플러그인 agent 파일을 `agents/`의 서브폴더에 넣을 수 있습니다. Claude Code는 [재귀적으로 로드](/docs/ko/sub-agents#choose-the-subagent-scope)하고 플러그인 이름, 각 서브폴더 이름, 파일 이름을 콜론으로 결합하여 에이전트의 범위 지정 이름을 형성합니다. 예를 들어, `my-plugin`이라는 플러그인의 `agents/review/security.md`는 `my-plugin:review:security`로 로드됩니다. 두 가지 설정이 해당 이름을 변경합니다:

* Frontmatter `name`: 파일 이름만 대체하므로, `agents/review/security.md`의 `name: audit`은 `my-plugin:review:audit`로 로드됩니다
* Manifest [`agents`](/docs/ko/plugins/manifest-reference#fields) 필드: 거기에 나열한 파일은 서브폴더 이름 없이 로드되므로, `"agents": "./custom/review/security.md"`는 `my-plugin:security`로 로드됩니다

<h4 id="frontmatter-fields-in-plugin-agents">
  플러그인 agents의 Frontmatter 필드
</h4>

플러그인 agent의 frontmatter는 다음 규칙을 따릅니다:

* **지원되는 필드**: `name`, `description`, `model`, `effort`, `maxTurns`, `tools`, `disallowedTools`, `skills`, `memory`, `background`, `omitClaudeMd`, `isolation`, `color`, 그리고 `experimental`의 `cacheTtl` 키. 유일한 유효한 `isolation` 값은 `"worktree"`입니다. 각각이 무엇을 하는지는 [지원되는 frontmatter 필드](/docs/ko/sub-agents#supported-frontmatter-fields)를 참조합니다
* **무시되는 필드**: `permissionMode`, `hooks`, `mcpServers`, `initialPrompt`. agent 파일은 자신의 hooks나 MCP 서버를 추가할 수 없으므로, 이들을 플러그인 [hooks](#hooks)와 [MCP 서버](#mcp-servers)로 추가합니다
* **파싱되지 않는 Frontmatter**: agent는 여전히 모든 필드가 무시된 상태로 로드됩니다. 파일 이름으로 이름 지정되고, 설명은 `Agent from my-plugin plugin`을 읽습니다. 셸에서 [`claude plugin validate`](/docs/ko/plugins/cli-reference#plugin-validate)를 실행하여 이러한 파일을 찾습니다

각 필드가 무엇을 하는지와 우선순위 규칙의 경우, [Subagents](/docs/ko/sub-agents#supported-frontmatter-fields)를 참조합니다.

<h3 id="hooks">
  Hooks
</h3>

[hook](/docs/ko/hooks-guide)은 Claude Code의 라이프사이클의 한 지점(예: 모든 파일 편집 후)에서 자동으로 무언가를 실행합니다: 셸 명령, HTTP 요청, MCP 도구 호출, 모델에 대한 프롬프트, 또는 서브에이전트. 플러그인의 hooks를 플러그인 루트의 `hooks/hooks.json`에 저장하고, 최상위 `"hooks"` 키 아래에, `settings.json`의 `hooks` 객체와 동일한 형태로 저장합니다. 이를 통해 기존 설정 hook을 변경 없이 복사할 수 있습니다.

이 hook은 모든 `Write` 또는 `Edit` 후에 번들된 스크립트를 실행합니다:

```json hooks/hooks.json theme={null}
{
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Write|Edit",
        "hooks": [
          {
            "type": "command",
            "command": "\"${CLAUDE_PLUGIN_ROOT}/scripts/format.sh\""
          }
        ]
      }
    ]
  }
}
```

`scripts/format.sh`에 스크립트를 저장하고 실행 가능하게 만듭니다.

플러그인을 로드하고 Claude에 파일을 편집하도록 요청합니다. 종료 코드 0인 `PostToolUse` hook은 트랜스크립트에 아무것도 표시하지 않으므로, [디버그 로깅](/docs/ko/hooks#debug-hooks)으로 또는 스크립트 자체가 변경하는 것으로 실행되었는지 확인합니다.

`hooks/hooks.json`과 `hooks` manifest 키의 Hooks는 모두 로드됩니다. 모든 이벤트와 그 페이로드의 경우, [Hook 이벤트](/docs/ko/hooks#hook-events)를 참조합니다.

<h4 id="when-plugin-hooks-fire">
  플러그인 hooks가 발생할 때
</h4>

플러그인의 hooks는 플러그인의 skills나 명령 중 하나가 사용될 때까지 기다리지 않습니다. Claude Code는 세션이 플러그인을 로드할 때 이들을 등록하고, 그 이후로 이벤트에서 발생합니다. hook이 실행되는 시기를 제한하려면, `matcher`를 좁힙니다.

hook이 발생하지 않으면, [발생하지 않는 hooks](/docs/ko/plugins/troubleshooting#failed-to-load-hooks-from-and-hooks-that-dont-fire)를 참조합니다.

<h4 id="environment-quoting-and-matching-mcp-tools">
  환경, 인용, MCP 도구 일치
</h4>

hook의 환경, `${CLAUDE_PLUGIN_ROOT}`의 인용, 플러그인의 자신의 MCP 도구에 대한 매처는 다음과 같이 작동합니다:

* **환경**: 모든 hook 프로세스는 환경에서 `CLAUDE_PLUGIN_ROOT`와 `CLAUDE_PLUGIN_DATA`를 받고, 각 [사용자 구성](#user-configuration) 값에 대해 `CLAUDE_PLUGIN_OPTION_<KEY>`를 받으므로, 스크립트는 거기서 이들을 읽을 수 있습니다
* **인용**: `command`에 `args`가 없으면, 셸을 통해 실행되므로, `${CLAUDE_PLUGIN_ROOT}` 경로를 큰따옴표로 감싸십시오, [Hooks](#hooks) 아래의 `hooks/hooks.json` 예제처럼, 확장된 경로를 하나의 셸 단어로 유지하려면. `args`를 대신 전달하면, 각 요소는 셸 없이 하나의 인수로 전달되고 인용이 필요하지 않습니다. [exec 형식과 셸 형식](/docs/ko/hooks#exec-form-and-shell-form)을 참조합니다
* **플러그인의 자신의 MCP 도구 일치**: 이 플러그인이 선언하는 [MCP 서버](#mcp-servers)의 도구는 `mcp__plugin_<plugin>_<server>__<tool>`로 이름 지정되므로, 매처에 전체 이름을 작성합니다. 서버 이름만의 매처는 발생하지 않습니다. [MCP 도구 일치](/docs/ko/hooks#match-mcp-tools)를 참조합니다

<h3 id="mcp-servers">
  MCP 서버
</h3>

MCP 서버는 Claude에 외부 시스템의 도구를 제공합니다. 플러그인 루트의 `.mcp.json`에서 선언하고, [프로젝트 `.mcp.json`](/docs/ko/mcp#project-scope)과 동일한 형태로 선언합니다. 이 `.mcp.json`은 `db`라는 하나의 서버를 선언합니다:

```json .mcp.json theme={null}
{
  "mcpServers": {
    "db": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"]
    }
  }
}
```

`mcpServers` 래퍼를 생략하고 `db`를 파일의 최상위 수준에 넣을 수도 있습니다.

플러그인을 로드하고 `/mcp`를 실행하여 서버가 `plugin:my-plugin:db`로 나타나는지 확인합니다.

`claude plugin validate`는 `.mcp.json`을 확인하고 Claude Code가 로드 시간에 삭제할 서버 항목을 오류로 보고합니다. Claude Code v2.1.281 이상이 필요합니다.

잘못된 항목이 로드 시간에 나타나는 위치의 경우, [시작하지 않는 MCP 서버](/docs/ko/plugins/troubleshooting#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start)를 참조합니다.

`mcpServers` manifest 키는 인라인 서버 맵, JSON 파일 경로, 또는 이들의 배열을 사용합니다. manifest 서버가 `.mcp.json`의 것과 동일한 이름을 가지면, manifest 서버가 이를 대체합니다.

<h4 id="reach-users-on-claude-ai-and-cowork">
  claude.ai 및 Cowork의 사용자에게 도달
</h4>

`db` 서버 아래의 [MCP 서버](#mcp-servers)와 같은 로컬 stdio 서버는 Claude Code에서 실행되고 Claude Desktop 앱에서 머신에서 실행되는 Cowork 세션에서 실행되지만, claude.ai에서는 실행되지 않습니다. 거기에도 사용자에게 도달하려면, `https://` URL로 원격 서버를 참조하십시오, 이는 claude.ai와 Cowork이 사용자에게 커넥터로 제공합니다.

<h4 id="server-names-tool-names-and-reloads">
  서버 이름, 도구 이름, 재로드
</h4>

서버의 이름, 변수 대체, 재로드 동작은 다음 규칙을 따릅니다:

* **서버 이름**: `plugin:<plugin>:<server>`, 따라서 `my-plugin`의 `db` 서버는 `/mcp`에서 `plugin:my-plugin:db`입니다. [`mcp_tool` hook](/docs/ko/hooks#mcp-tool-hook-fields)에서 서버를 이름 지정할 때 동일한 형식을 사용합니다
* **도구 이름**: `mcp__plugin_<plugin>_<server>__<tool>`, 따라서 해당 `db` 서버의 `query` 도구는 `mcp__plugin_my-plugin_db__query`입니다. 이것은 [권한 규칙](/docs/ko/permissions)과 [hook 매처](#hooks)에서 사용할 이름입니다
* **대체**: `${CLAUDE_PLUGIN_ROOT}`와 다른 [경로 변수](#path-variables-and-persistent-data)는 `command`, `args`, `env`에서 대체됩니다. `args`에서는 각 요소가 하나의 인수로 전달되므로 인용이 필요하지 않습니다
* **재로드**: 사용자가 `/reload-plugins`를 실행하고 [재로드가 적용](/docs/ko/plugins/cli-reference#reloads-that-change-mcp-tools)되면, 구성이 변경되지 않은 서버는 연결을 유지합니다. 구성이 변경된 서버는 재연결되고, 제거한 서버는 연결을 끊습니다

<h4 id="include-a-packaged-mcpb-server">
  패키지된 MCPB 서버 포함
</h4>

`mcpServers` 키는 또한 [MCPB 파일](https://github.com/modelcontextprotocol/mcpb)로 패키지된 서버를 수용하며, 확장자는 `.mcpb` 또는 이전 `.dxt`입니다. 키를 파일로 가리키십시오, 플러그인 내부의 경로 또는 `https://` URL:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "my-plugin",
  "mcpServers": "./servers/db.mcpb"
}
```

서버는 번들의 manifest에서 `name`을 가져옵니다.

전송 및 인증의 경우, [MCP](/docs/ko/mcp#plugin-provided-mcp-servers)를 참조합니다.

<h3 id="lsp-servers">
  LSP 서버
</h3>

LSP 서버는 Claude에 언어에 대한 진단 및 코드 네비게이션을 제공합니다. [공식 코드 인텔리전스 플러그인](/docs/ko/plugins/code-intelligence)이 이미 언어를 다루면, 하나을 작성하는 대신 설치합니다. 그렇지 않으면 플러그인 루트의 `.lsp.json`에서 서버를 선언합니다:

```json .lsp.json theme={null}
{
  "gopls": {
    "command": "gopls",
    "args": ["serve"],
    "extensionToLanguage": {
      ".go": "go"
    }
  }
}
```

파일은 각 서버 이름을 직접 구성에 매핑하며, 맵 주위에 래퍼 객체가 없습니다. `command`는 바이너리의 이름이고, 인수는 `args`에 있습니다. `extensionToLanguage`는 최소한 하나의 확장이 필요하며, 각각은 `.`로 시작합니다.

`claude plugin validate`는 이 파일을 읽지 않습니다. 항목이 유효하지 않으면, 전체 파일은 로드 시 건너뛰고 `Invalid LSP server config for ".lsp.json"`이 `/plugin` **Errors** 탭에 나타납니다.

플러그인은 연결을 구성하지만 서버 바이너리를 설치하지 않으며, 각 파일 확장자는 하나의 서버를 가집니다:

* **누락된 바이너리**: Claude Code는 사용자의 `PATH`에서 이름으로 `command`를 시작합니다. 바이너리가 없으면, 서버는 시작하지 못하고 `claude --debug`는 `LSP server <name> failed to start`를 로깅합니다
* **확장자 충돌**: 두 개의 활성화된 서버가 동일한 확장자를 주장하면, 처음 등록된 것이 해당 파일을 처리하고 다른 것은 이들에 대해 사용되지 않습니다, 서버가 하나의 플러그인에서 오든 두 개에서 오든. `/plugin` **Errors** 탭은 경고 `LSP server "<name>" is not used for <ext> files`를 표시합니다

`lspServers` manifest 키는 동일한 맵을 인라인으로, JSON 파일 경로로, 또는 이들의 배열로 사용하며, 서버는 `.lsp.json`의 것에 추가됩니다. manifest 서버가 `.lsp.json`의 것과 동일한 이름을 가지면, manifest 서버가 이를 대체합니다.

`transport`, 타임아웃, 재시작, 다른 필드의 경우, [`lspServers`](/docs/ko/plugins/manifest-reference#lspservers)를 참조합니다.

로그 출력을 stdout이 아닌 stderr로 보냅니다. Claude Code는 서버의 stdout을 프로토콜 메시지로만 읽고, 메시지 헤더는 최대 64 KiB, 메시지 본문은 최대 32 MiB를 수용합니다.

Claude Code는 어느 한계를 초과하거나 stdout에 비프로토콜 출력을 작성하는 서버를 연결 해제하고, `restartOnCrash`와 `maxRestarts`에 대해 연결 해제를 충돌로 계산합니다. `--debug`로 실행하면, Claude Code는 원인을 이름 지정하는 오류를 디버그 로그에 작성합니다.

<h3 id="executables">
  실행 파일
</h3>

플러그인 루트의 `bin/` 파일은 플러그인이 활성화되어 있는 동안 Bash 도구의 셸의 `PATH`에 있으므로, Claude는 이들을 베어 명령으로 실행할 수 있습니다. 실행 가능한 스크립트를 추가합니다:

```bash bin/hello-plugin theme={null}
#!/bin/bash
echo "hello from my-plugin"
```

`chmod +x bin/hello-plugin`으로 실행 가능하게 만들고 플러그인을 로드합니다. Claude에 `hello-plugin`을 실행하도록 요청하면, Bash 도구 결과는 스크립트의 출력을 표시합니다.

플러그인 `bin/` 디렉토리는 사용자의 자신의 `PATH` 항목 뒤에 오므로, 플러그인은 `git`, `ls`, 또는 다른 시스템 명령을 섀도우할 수 없습니다.

claude.ai와 Cowork은 최상위 `bin/` 디렉토리를 가진 플러그인을 설치하지 않습니다, [claude.ai 조직 설정을 통해 배포](/docs/ko/plugins/host-marketplace#distribute-through-organization-settings)하는 것을 포함합니다.

<h3 id="default-settings">
  기본 설정
</h3>

플러그인이 활성화되어 있는 동안 적용되는 기본값을 설정하려면, 플러그인 루트에 `settings.json`을 추가하거나, 동일한 객체를 `settings` manifest 키에 인라인으로 넣습니다. 두 개의 키가 효과를 발휘합니다, `agent`와 `subagentStatusLine`, 다른 모든 키는 삭제됩니다.

플러그인의 자신의 agents 중 하나를 주 스레드로 실행하도록 `agent`를 설정합니다:

```json settings.json theme={null}
{
  "agent": "security-reviewer"
}
```

플러그인을 로드하고 세션을 시작합니다. Claude는 주 대화에서 `security-reviewer` 에이전트의 시스템 프롬프트와 모델로 응답합니다.

키가 제어하는 모든 것의 경우, [`agent` 설정](/docs/ko/settings-reference#agent)을 참조합니다.

동일한 키가 하나 이상의 위치에서 설정되면, 이 규칙들이 어느 값이 적용되는지 결정합니다:

* **파일이 manifest보다 우선**: 둘 다 존재하고 `settings.json`이 최소한 하나의 지원되는 키를 설정하면, `settings.json`이 적용되고 manifest의 `settings`는 무시됩니다
* **사용자 설정이 플러그인 기본값보다 우선**: 설정 소스 전체에서, 플러그인 기본값은 가장 낮은 계층이므로, 사용자의 자신의 `agent`는 `~/.claude/settings.json`에서 당신의 것을 재정의합니다
* **두 개의 플러그인이 동일한 키를 설정**: 마지막에 로드된 플러그인의 값이 적용되고, `claude --debug`는 `overrides setting`을 로깅합니다

`subagentStatusLine` 형태의 경우, [서브에이전트 상태 라인](/docs/ko/statusline#subagent-status-lines)을 참조합니다.

<h3 id="themes-and-output-styles">
  테마 및 출력 스타일
</h3>

플러그인은 색상 테마와 출력 스타일을 포함할 수 있습니다. 둘 다 사용자의 자신의 것과 동일한 선택기에 나타납니다. 둘 중 하나의 경우, manifest 키를 설정하면 폴더 스캔을 대체합니다.

| 컴포넌트   | 다음으로 저장                   | 형식                                                                                                       | 다음에 나타남                              | Manifest 키            |
| :----- | :------------------------ | :------------------------------------------------------------------------------------------------------- | :----------------------------------- | :-------------------- |
| 테마     | `themes/<slug>.json`      | 사용자가 `~/.claude/themes/`에 작성하는 [사용자 정의 테마 파일](/docs/ko/terminal-config#create-a-custom-theme) 형식              | `/theme`, 파일의 `name` 아래              | `experimental.themes` |
| 출력 스타일 | `output-styles/<name>.md` | [사용자 정의 출력 스타일](/docs/ko/output-styles#create-a-custom-output-style) 형식, `name`과 `description` frontmatter 포함 | `/output-style`, `<plugin>:<name>`으로 | `outputStyles`        |

플러그인 테마는 읽기 전용이므로, 사용자가 `/theme`에서 하나를 편집하면, 편집은 자신의 테마 디렉토리에 복사본으로 저장됩니다.

이 테마는 어두운 사전 설정에서 프롬프트 악센트와 오류 텍스트를 다시 칠합니다:

```json themes/dracula.json theme={null}
{
  "name": "Dracula",
  "base": "dark",
  "overrides": {
    "claude": "#bd93f9",
    "error": "#ff5555"
  }
}
```

<h3 id="channels">
  채널
</h3>

[채널](/docs/ko/channels)은 채팅 앱과 같은 외부 시스템이 메시지를 세션으로 보낼 수 있게 합니다. 플러그인에서, 채널은 MCP 서버 중 하나와 이를 바인딩하고 자신의 구성을 프롬프트할 수 있는 `channels` 항목입니다. 이 manifest는 채널을 `telegram` 서버에 바인딩하고 봇 토큰을 요청합니다:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "my-plugin",
  "mcpServers": {
    "telegram": {
      "command": "node",
      "args": ["${CLAUDE_PLUGIN_ROOT}/server.js"],
      "env": { "BOT_TOKEN": "${user_config.bot_token}" }
    }
  },
  "channels": [
    {
      "server": "telegram",
      "userConfig": {
        "bot_token": {
          "type": "string",
          "title": "Bot token",
          "description": "Telegram bot token",
          "sensitive": true
        }
      }
    }
  ]
}
```

`server`는 `mcpServers`의 키와 일치해야 합니다. 채널별 `userConfig`는 [최상위 `userConfig` 키](#user-configuration)와 동일한 형태를 사용합니다.

서버가 구현해야 하는 것과 사용자가 채널 플러그인을 활성화하는 방법의 경우, 채널 참조의 [플러그인으로 패키지](/docs/ko/channels-reference#package-as-a-plugin)를 참조합니다. 필드 테이블의 경우, [`channels`](/docs/ko/plugins/manifest-reference#channels)를 참조합니다.

<h3 id="monitors">
  모니터
</h3>

모니터는 전체 세션 동안 백그라운드에서 실행되는 셸 명령입니다. 이것이 인쇄하는 것은 Claude에 알림으로 도달하므로, Claude는 보도록 요청받지 않고도 로그나 상태 변경에 반응할 수 있습니다. 항목을 `monitors/monitors.json`에 저장합니다:

```json monitors/monitors.json theme={null}
[
  {
    "name": "error-log",
    "command": "tail -F ./logs/error.log",
    "description": "Application error log"
  }
]
```

명령은 셸에서 실행되고, 세션이 시작된 작업 디렉토리에서 실행됩니다.

모니터의 명령은 시작 위치와 참조할 수 있는 것에서 제한됩니다:

* **대화형 세션만**: 플러그인 모니터는 대화형 세션에서 시작되고 `-p` 플래그를 사용한 비대화형 모드에서는 시작되지 않습니다. 또한 [Monitor 도구](/docs/ko/tools-reference#monitor-tool)가 사용 가능한 곳에서만 시작됩니다
* **사용자 구성 없음**: `command`는 [경로 변수](#path-variables-and-persistent-data)와 환경의 `${ENV_VAR}`을 가져오지만, `${user_config.*}`는 절대 가져오지 않습니다. 하나을 참조하는 모니터는 시작되지 않으며, 모니터 프로세스는 `CLAUDE_PLUGIN_OPTION_<KEY>`도 받지 않습니다
* **세션 중 비활성화**: 세션 중에 플러그인을 비활성화하면, Claude Code는 이미 실행 중인 모니터를 중지하지 않습니다. 세션이 끝날 때 중지됩니다

`experimental.monitors` manifest 키는 동일한 배열을 인라인으로 또는 JSON 파일 경로로 사용하고, `monitors/monitors.json` 대신 읽습니다.

`when` 트리거 및 다른 필드의 경우, [`monitors`](/docs/ko/plugins/manifest-reference#monitors)를 참조합니다.

<h2 id="user-configuration">
  사용자에게 구성 값을 요청합니다
</h2>

플러그인이 사용자로부터 필요로 하는 값을 `userConfig` manifest 키에서 선언하여, 사용자가 `settings.json`을 자신들이 편집하지 않도록 합니다. 각 옵션은 `title`을 레이블로 하고 `description`을 아래에 가진 대화 상자에 나타납니다.

토큰이나 비밀번호의 경우 `"sensitive": true`를 설정합니다. 대화 상자는 입력을 마스크하고, 값은 `settings.json`이 아닌 보안 저장소에 저장됩니다.

이 manifest는 엔드포인트와 토큰을 요청합니다:

```json .claude-plugin/plugin.json theme={null}
{
  "name": "my-plugin",
  "userConfig": {
    "api_url": {
      "type": "string",
      "title": "API URL",
      "description": "Base URL of your team's API"
    },
    "api_token": {
      "type": "string",
      "title": "API token",
      "description": "Token for your team's API",
      "sensitive": true
    }
  }
}
```

<h3 id="when-the-configuration-dialog-appears">
  구성 대화 상자가 나타날 때
</h3>

대화 상자는 대화형 `/plugin` 인터페이스에서만 나타납니다. 사용자가 다음 중 하나를 수행할 때 아직 설정되지 않은 옵션에 대해 열립니다:

* `/plugin`에서 플러그인을 설치합니다
* 세션 내에서 `/plugin install <plugin>@<marketplace>`를 실행합니다
* `/plugin`의 **Installed** 탭에서 플러그인을 활성화합니다

언제든지 동일한 대화 상자를 열려면, 사용자는 `/plugin configure <plugin>@<marketplace>`를 실행합니다.

`claude plugin install` 셸 명령은 `userConfig` 값을 프롬프트하지 않습니다. 셸에서 값을 설정하려면, 각각을 `--config KEY=VALUE`로 전달합니다. 옵션이 설정되지 않으면, 명령은 `userConfig options not yet set` 라인을 인쇄하여 둘 다 설정하는 방법을 이름 지정합니다. [The `userConfig` dialog never appears](/docs/ko/plugins/troubleshooting#the-userconfig-dialog-never-appears)는 라인을 인용합니다.

옵션 필드, 각 값이 저장되는 위치, 컴포넌트가 저장된 값을 참조하는 방법, `${user_config.*}`를 거부하는 필드의 경우, [사용자 구성](/docs/ko/plugins/manifest-reference#user-configuration)을 참조합니다.

<h2 id="path-variables-and-persistent-data">
  플러그인 경로 참조 및 데이터 저장
</h2>

플러그인이 어디에 설치될지 모르므로, 고정 경로보다는 이 변수를 통해 파일과 데이터를 참조합니다. 이들은 skill, 명령, agent 콘텐츠에서, hook과 모니터 명령에서, MCP와 LSP 서버 구성에서 대체됩니다. 또한 hook, MCP, LSP 프로세스로 내보내집니다:

* **`${CLAUDE_PLUGIN_ROOT}`**: 플러그인의 설치 디렉토리. 각 버전은 자신의 [캐시 디렉토리](/docs/ko/plugins/loading#find-plugins-on-disk)를 가지므로, 플러그인이 업데이트될 때 경로가 변경됩니다. 거기에 상태를 작성하지 마십시오
* **`${CLAUDE_PLUGIN_DATA}`**: 업데이트를 생존하는 디렉토리, `node_modules`, 가상 환경, 캐시용. `~/.claude/plugins/data/<id>/`로 해석되고 처음 참조될 때 생성됩니다
* **`${CLAUDE_PROJECT_DIR}`**: 프로젝트 루트, hooks가 받는 동일한 값

데이터 디렉토리 경로에서, `<id>`는 문자, 숫자, `_`, `-` 이외의 모든 문자가 `-`로 대체된 플러그인 식별자이므로, `my-plugin@my-marketplace`는 `my-plugin-my-marketplace`가 됩니다.

Windows에서, 대체된 경로는 셸이 백슬래시를 이스케이프로 읽지 않도록 전진 슬래시를 사용합니다.

<h3 id="install-dependencies-into-the-data-directory">
  데이터 디렉토리에 종속성 설치
</h3>

마켓플레이스 설치 플러그인의 경우, Claude Code는 플러그인을 캐시할 때 적격 [Node.js 패키지 종속성](/docs/ko/plugins/loading#node-js-package-dependencies)을 자동으로 설치하므로, 자신이 설치할 필요가 없을 수 있습니다. 할 때, 이 `SessionStart` hook은 첫 실행 시 `${CLAUDE_PLUGIN_DATA}`에 `node_modules`를 설치하고 업데이트가 `package.json`을 변경한 후 다시 설치합니다:

```json hooks/hooks.json theme={null}
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "diff -q \"${CLAUDE_PLUGIN_ROOT}/package.json\" \"${CLAUDE_PLUGIN_DATA}/package.json\" >/dev/null 2>&1 || (cd \"${CLAUDE_PLUGIN_DATA}\" && cp \"${CLAUDE_PLUGIN_ROOT}/package.json\" . && npm install) || rm -f \"${CLAUDE_PLUGIN_DATA}/package.json\""
          }
        ]
      }
    ]
  }
}
```

첫 번째 세션 후, `~/.claude/plugins/data/<id>/node_modules`이 존재합니다. MCP 서버는 그 다음 `NODE_PATH`를 `${CLAUDE_PLUGIN_DATA}/node_modules`로 설정할 수 있습니다. 어느 필드가 어느 변수를 대체하는지의 경우, [환경 변수](/docs/ko/plugins/manifest-reference#environment-variables)를 참조합니다.

<h2 id="next-steps">
  다음 단계
</h2>

* [플러그인 manifest 참조](/docs/ko/plugins/manifest-reference): `plugin.json` 필드, 경로 규칙, 표준 레이아웃
* [evals로 플러그인 테스트](/docs/ko/plugin-evals): 추가한 컴포넌트가 Claude의 동작을 의도한 방식으로 변경하는지 확인합니다
* [플러그인 게시 및 배포](/docs/ko/plugins/publish): 플러그인을 버전 지정하고 마켓플레이스에 넣습니다
* [플러그인 문제 해결](/docs/ko/plugins/troubleshooting): 컴포넌트가 로드되지 않거나 hook이 발생하지 않을 때 수행할 작업
