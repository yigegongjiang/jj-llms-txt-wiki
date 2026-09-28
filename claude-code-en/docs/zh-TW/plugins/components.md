> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 新增元件至外掛程式

> 新增技能、hooks、MCP 伺服器及其他所有元件類型至 Claude Code 外掛程式，並提供每種元件的驗證範例。

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

Claude Code 外掛程式由多個元件組成，例如技能、代理、hooks 和 MCP 伺服器。每個元件在外掛程式中都有一個預設資料夾、`.claude-plugin/plugin.json` 中的選用資訊清單鍵（用於取代或新增至該資料夾），以及使用者看到的名稱。如需每個鍵的完整欄位表，請參閱[資訊清單參考](/docs/zh-TW/plugins/manifest-reference#fields)。

使用此頁面將元件新增至已載入的外掛程式。

新增元件後，在執行中的工作階段中執行 `/reload-plugins`，或啟動新的工作階段，以便 Claude Code 載入該元件。若要在載入前檢查元件的檔案，請從外掛程式目錄在您的殼層中執行 [`claude plugin validate .`](/docs/zh-TW/plugins/cli-reference#plugin-validate)。

<Note>
  這些情況涵蓋在其他頁面上：

  * **建立您的第一個外掛程式**：從[建立外掛程式](/docs/zh-TW/plugins/create)開始
  * **安裝他人的外掛程式**：請參閱[安裝外掛程式](/docs/zh-TW/plugins/install)
  * **您的外掛程式使用者在 claude.ai 或 Cowork 上**：那裡會載入不同的元件集合。請參閱[claude.ai 和 Cowork 上的外掛程式](https://claude.com/docs/plugins/overview)
</Note>

<h2 id="explore-the-plugin-directory">
  探索外掛程式目錄
</h2>

探索工具顯示一個範例外掛程式 `my-plugin`，其在預設位置具有每種元件：

* 一個審查 skill 和一個 `about` 命令
* 一個安全審查子代理
* 一個在 Claude 編輯檔案後格式化檔案的 hook，以及它呼叫的 `scripts/` 資料夾
* 一個日誌監視器
* 一個輸出樣式和一個色彩主題
* 一個路由審計工作流程
* 一個 `hello-plugin` 可執行檔
* 預設設定
* 一個本機 MCP 伺服器和一個 Go 語言伺服器

每個檔案都是其格式的最小有效範例，目的是展示形狀而不是有用：真實的 skill 或 agent 包含完整的指示，通常還有支援檔案，真實的 hook 或監視器執行真實的工作。探索工具後的章節使用與探索工具相同的檔案作為範例，並連結至更完整的檔案。選擇檔案或資料夾以讀取其用途、查看其內容，並找到涵蓋它的章節。

<PluginExplorer>
  <Piece id="manifest">
    [manifest](/docs/zh-TW/plugins/manifest-reference) 是外掛程式 `.claude-plugin/` 目錄中的 `plugin.json` 檔案。它包含外掛程式的中繼資料和 Claude Code 提示使用者的 `userConfig` 值。只有 `name` 是必需的。在這個中，`description` 是使用者在 `/plugin` 中看到的外掛程式文字，`version` 會讓使用者保持在該版本，直到您變更它：

    ```json theme={null}
    {
      "name": "my-plugin",
      "version": "1.0.0",
      "description": "Review, formatting, and database tools for this team"
    }
    ```
  </Piece>

  <Piece id="skills">
    [skill](/docs/zh-TW/skills) 是一個 `SKILL.md` 檔案。將每個 skill 儲存在 `skills/` 下的自己的目錄中。Claude 讀取每個 skill 的 `description`，當使用者要求的內容與其相符時（例如要求 Claude 審查此處的提取請求），Claude 會載入 skill 的指示並遵循它們。使用者也可以直接執行它作為 `/my-plugin:review`：

    ```markdown theme={null}
    ---
    description: Reviews a pull request for style and test coverage. Use when asked to review code.
    ---

    Review the changed files. Report style problems first, then missing tests.
    ```
  </Piece>

  <Piece id="commands">
    命令是使用者按名稱執行的單一 Markdown 檔案。命令是較舊的格式：skill 按名稱執行的方式相同，也可以在自己的目錄中攜帶支援檔案，因此將新的寫成 skills，並為您已有的檔案保留 `commands/`。此檔案變成 `/my-plugin:about`，並採用與 skill 相同的 frontmatter：

    ```markdown theme={null}
    ---
    description: Summarize the repository
    ---

    Summarize what this repository does in three sentences.
    ```
  </Piece>

  <Piece id="agents">
    [子代理](/docs/zh-TW/sub-agents) 是一個單獨的助手，具有自己的指示和自己的內容視窗，Claude 可以將任務委派給它並取回結果。`agents/` 下的每個 Markdown 檔案定義一個：frontmatter 命名它並說明何時使用它，正文是其系統提示。這個命名為 `my-plugin:security-reviewer`，使用者可以使用 `@agent-my-plugin:security-reviewer` 叫用它：

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
    [hook](/docs/zh-TW/hooks-guide) 在 Claude Code 生命週期中的某個點自動執行某些操作，例如在每次檔案編輯後：shell 命令、HTTP 請求、MCP 工具呼叫、對模型的提示或子代理。將外掛程式的 hooks 儲存在外掛程式根目錄的 `hooks/hooks.json` 中。這個在 Claude 寫入或編輯檔案後執行外掛程式的 `scripts/format.sh`：

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
    監視器是一個 shell 命令，Claude Code 在工作階段啟動時在背景啟動，並保持執行直到工作階段結束，使用 [Monitor 工具](/docs/zh-TW/tools-reference#monitor-tool)。它列印的內容作為通知到達 Claude。`when` 欄位可以改為在命名 skill 首次執行時啟動它。這個尾部一個錯誤日誌：

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
    外掛程式可以包含 [輸出樣式](/docs/zh-TW/output-styles)，這會改變 Claude 格式化和措辭其回覆的方式。將每個輸出樣式儲存為 `output-styles/<name>.md`。這個在 `/output-style` 中顯示為 `my-plugin:terse`：

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
    外掛程式可以包含 [Claude Code 介面的色彩主題](/docs/zh-TW/terminal-config#create-a-custom-theme)。將每個主題儲存為 `themes/<slug>.json`。這個在 `/theme` 中顯示為 `Dracula`，標記為來自 `my-plugin`：

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
    `workflows/` 資料夾包含 [workflow](/docs/zh-TW/workflows) `.js` 檔案：一個 `meta` 區塊，然後是協調多個子代理的指令碼正文。這個執行為 `/my-plugin:audit-routes`：

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
    `bin/` 是外掛程式如何提供命令列工具的方式。啟用外掛程式時，Claude Code 將此資料夾放在它執行命令的 shell 的 `PATH` 上，因此 Claude 或 skill 的指示可以按名稱執行工具，而無需使用者安裝任何東西。有了這個 [可執行檔](#executables)，`hello-plugin` 是 Claude 可以執行的命令：

    ```bash theme={null}
    #!/bin/bash
    echo "hello from my-plugin"
    ```
  </Piece>

  <Piece id="scripts">
    `hooks/hooks.json` 中的 hook 執行指令碼，此資料夾是範例保留它的位置。名稱 `scripts/` 是一個慣例，不是 Claude Code 尋找的東西：hook 按其路徑指向檔案，`${CLAUDE_PLUGIN_ROOT}/scripts/format.sh`。格式化指令碼可能看起來像這樣：

    ```bash theme={null}
    #!/bin/bash
    npx prettier --write .
    ```
  </Piece>

  <Piece id="settings">
    外掛程式根目錄的 `settings.json` 包含在啟用外掛程式時適用的 [設定](/docs/zh-TW/settings-reference)，因此外掛程式可以改變工作階段的行為方式，而不僅僅是新增元件。只有兩個鍵從外掛程式生效，[`agent`](/docs/zh-TW/settings-reference#agent) 和 [`subagentStatusLine`](/docs/zh-TW/settings-reference#subagentstatusline)；所有其他鍵都被丟棄。請參閱 [預設設定](#default-settings)。

    這個設定 `agent`，它執行工作階段的主執行緒作為外掛程式自己的 `security-reviewer` agent，因此該 agent 的系統提示、工具限制和模型適用於整個工作階段：

    ```json theme={null}
    {
      "agent": "security-reviewer"
    }
    ```
  </Piece>

  <Piece id="mcp">
    [MCP 伺服器](/docs/zh-TW/mcp) 從外部系統為 Claude 提供工具。在外掛程式根目錄的 `.mcp.json` 中宣告它。這個啟動外掛程式內指令碼中的本機伺服器，並在 `/mcp` 中顯示為 `plugin:my-plugin:db`：

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
    LSP 伺服器為 Claude 提供 [診斷和程式碼導航](/docs/zh-TW/plugins/code-intelligence) 用於語言。在外掛程式根目錄的 `.lsp.json` 中宣告伺服器。這個連接 Go 語言伺服器用於 `.go` 檔案：

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
  新增每種元件
</h2>

下面的每個章節涵蓋一種元件：其檔案在外掛程式中的位置、驗證的範例、外掛程式載入後使用者看到的內容，以及改變預設位置的 manifest 鍵。新增您的外掛程式需要的；沒有任何是必需的。

<h3 id="skills">
  Skills
</h3>

[skill](/docs/zh-TW/skills) 是一個 `SKILL.md` 檔案，當其描述與任務相符時 Claude 可以載入。使用者也可以將其作為命令執行。將每個 skill 儲存在 `skills/` 下的自己的目錄中：

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json
└── skills/
    └── review/
        └── SKILL.md
```

給 `SKILL.md` 一個 `description`，以便 Claude 知道何時使用它：

```markdown skills/review/SKILL.md theme={null}
---
description: Reviews a pull request for style and test coverage. Use when asked to review code.
---

Review the changed files. Report style problems first, then missing tests.
```

載入外掛程式後，`/my-plugin:review` 執行 skill。命令名稱和誰可以叫用它遵循這些規則：

* **命令名稱**：`/<plugin>:<directory>`，所以 `my-plugin` 中的 `skills/review/SKILL.md` 是 `/my-plugin:review`。如果您在 frontmatter 中設定 `name`，它會取代最後一個區段，外掛程式前綴保持不變。請參閱 [skill 如何獲得其命令名稱](/docs/zh-TW/skills#how-a-skill-gets-its-command-name)
* **誰叫用它**：Claude、使用者或兩者，由 frontmatter 控制。請參閱 [控制誰叫用 skill](/docs/zh-TW/skills#control-who-invokes-a-skill)

您也可以將 skills 放在預設 `skills/` 目錄之外：

* **其他目錄**：在 `skills` manifest 鍵中列出它們。它們新增至預設 `skills/` 掃描，而不是取代它，不像 `commands` 和 `agents`
* **外掛程式根目錄的單一 skill**：沒有 `skills/` 目錄且沒有 `skills` manifest 鍵，外掛程式根目錄的 `SKILL.md` 載入為一個 skill。在其 frontmatter 中設定 `name`，因為否則市場安裝會在其 [快取目錄](/docs/zh-TW/plugins/loading#find-plugins-on-disk) 之後命名 skill，而不是您的外掛程式

若要在外掛程式中包含指示，將其寫成 skill。Claude Code 不會載入外掛程式根目錄的 `CLAUDE.md`，`claude plugin validate` 會警告 `CLAUDE.md at the plugin root is not loaded as project context`。

如需 frontmatter 欄位和支援檔案，請參閱 [Skills](/docs/zh-TW/skills)。

<h3 id="commands">
  命令
</h3>

命令是使用者按名稱執行的單一 Markdown 檔案，例如 `/my-plugin:about`。

<Note>
  命令是較舊的格式，[skills](#skills) 對新工作已取代它們。skill 按名稱執行的方式相同，它也可以在其目錄中攜帶支援檔案。為您從 `.claude/commands/` 移動的檔案保留 `commands/`。
</Note>

將命令儲存在 `commands/<file>.md`，它變成 `/<plugin>:<file>`。子目錄新增一個區段，所以 `commands/db/migrate.md` 是 `/my-plugin:db:migrate`。

命令檔案採用與 skills 相同的 frontmatter。

<h4 id="define-commands-in-the-manifest">
  在 manifest 中定義命令
</h4>

只有當您想將命令檔案保留在 `commands/` 以外的地方，或在 `plugin.json` 中定義短命令而不需要單獨的 Markdown 檔案時，您才需要這個。設定 `commands` manifest 鍵，Claude Code 會讀取它而不是掃描 `commands/`。鍵採用路徑、路徑陣列或將每個命令名稱對應到 `source` 檔案或內嵌 `content` 的物件。

此 manifest 內嵌定義 `/my-plugin:about`，沒有 Markdown 檔案：

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

載入外掛程式並在工作階段中執行 `/my-plugin:about` 以確認它已載入。

如需完整的鍵語法，請參閱 [`commands`](/docs/zh-TW/plugins/manifest-reference#commands)。

<h3 id="agents">
  Agents
</h3>

[子代理](/docs/zh-TW/sub-agents) 是一個單獨的助手，具有自己的指示和內容視窗，Claude 可以將任務委派給它。`agents/` 下的每個 Markdown 檔案定義一個：

```markdown agents/security-reviewer.md theme={null}
---
name: security-reviewer
description: Reviews code changes for security issues. Use after edits to authentication or input handling.
model: sonnet
---

You are a security reviewer. Read the changed files and report injection, authentication, and secrets-handling risks.
```

此 agent 命名為 `my-plugin:security-reviewer`，使用者可以使用 `@agent-my-plugin:security-reviewer` [明確叫用它](/docs/zh-TW/sub-agents#invoke-subagents-explicitly)。名稱形式是 `<plugin>:<name>`，其中 `<name>` 來自 frontmatter，或沒有時來自檔案名稱。

`agents` manifest 鍵取代 `agents/` 掃描。

<h4 id="organize-agents-in-subfolders">
  在子資料夾中組織 agents
</h4>

您可以將外掛程式 agent 檔案放在 `agents/` 的子資料夾中。Claude Code [遞迴載入它們](/docs/zh-TW/sub-agents#choose-the-subagent-scope)，並使用冒號連接外掛程式名稱、每個子資料夾名稱和檔案名稱以形成 agent 的範圍名稱。例如，`my-plugin` 中的 `agents/review/security.md` 載入為 `my-plugin:review:security`。兩個設定改變該名稱：

* Frontmatter `name`：它只取代檔案名稱，所以 `agents/review/security.md` 中的 `name: audit` 載入為 `my-plugin:review:audit`
* Manifest [`agents`](/docs/zh-TW/plugins/manifest-reference#fields) 欄位：您在那裡列出的檔案載入時沒有子資料夾名稱，所以 `"agents": "./custom/review/security.md"` 載入為 `my-plugin:security`

<h4 id="frontmatter-fields-in-plugin-agents">
  外掛程式 agents 中的 Frontmatter 欄位
</h4>

外掛程式 agent 的 frontmatter 遵循這些規則：

* **支援的欄位**：`name`、`description`、`model`、`effort`、`maxTurns`、`tools`、`disallowedTools`、`skills`、`memory`、`background`、`omitClaudeMd`、`isolation`、`color` 和 `experimental` 的 `cacheTtl` 鍵。唯一有效的 `isolation` 值是 `"worktree"`。請參閱 [支援的 frontmatter 欄位](/docs/zh-TW/sub-agents#supported-frontmatter-fields) 以了解每個欄位的作用
* **忽略的欄位**：`permissionMode`、`hooks`、`mcpServers` 和 `initialPrompt`。agent 檔案無法自行新增 hooks 或 MCP 伺服器，因此改為將這些新增為外掛程式 [hooks](#hooks) 和 [MCP 伺服器](#mcp-servers)
* **無法解析的 Frontmatter**：agent 仍然載入，每個欄位都被忽略。它以檔案命名，其描述讀取 `Agent from my-plugin plugin`。在您的 shell 中執行 [`claude plugin validate`](/docs/zh-TW/plugins/cli-reference#plugin-validate) 以找到這些檔案

如需每個欄位的作用和優先順序規則，請參閱 [子代理](/docs/zh-TW/sub-agents#supported-frontmatter-fields)。

<h3 id="hooks">
  Hooks
</h3>

[hook](/docs/zh-TW/hooks-guide) 在 Claude Code 生命週期中的某個點自動執行某些操作，例如在每次檔案編輯後：shell 命令、HTTP 請求、MCP 工具呼叫、對模型的提示或子代理。將外掛程式的 hooks 儲存在外掛程式根目錄的 `hooks/hooks.json` 中，在頂層 `"hooks"` 鍵下，形狀與 `settings.json` 中的 `hooks` 物件相同。這讓您可以複製現有的設定 hook 而不變更。

此 hook 在每個 `Write` 或 `Edit` 後執行捆綁的指令碼：

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

將指令碼儲存在 `scripts/format.sh` 並使其可執行。

載入外掛程式並要求 Claude 編輯檔案。退出 0 的 `PostToolUse` hook 在文字記錄中不顯示任何內容，因此使用 [偵錯日誌](/docs/zh-TW/hooks#debug-hooks) 或指令碼本身所做的更改來確認它執行。

`hooks/hooks.json` 和 `hooks` manifest 鍵中的 Hooks 都會載入。如需每個事件及其承載，請參閱 [Hook 事件](/docs/zh-TW/hooks#hook-events)。

<h4 id="when-plugin-hooks-fire">
  外掛程式 hooks 何時觸發
</h4>

外掛程式的 hooks 不會等待使用外掛程式的 skills 或命令之一。Claude Code 在工作階段載入外掛程式時註冊它們，從那時起它們在其事件上觸發。若要限制 hook 執行的時間，縮小其 `matcher`。

如果 hook 從不觸發，請參閱 [不觸發的 hooks](/docs/zh-TW/plugins/troubleshooting#failed-to-load-hooks-from-and-hooks-that-dont-fire)。

<h4 id="environment-quoting-and-matching-mcp-tools">
  環境、引號和匹配 MCP 工具
</h4>

hook 的環境、`${CLAUDE_PLUGIN_ROOT}` 的引號和外掛程式自己的 MCP 工具的匹配器工作如下：

* **環境**：每個 hook 程序在其環境中接收 `CLAUDE_PLUGIN_ROOT` 和 `CLAUDE_PLUGIN_DATA`，加上每個 [使用者設定](#user-configuration) 值的 `CLAUDE_PLUGIN_OPTION_<KEY>`，因此您的指令碼可以從那裡讀取它們
* **引號**：當 `command` 沒有 `args` 時，它通過 shell 執行，因此將 `${CLAUDE_PLUGIN_ROOT}` 路徑包裝在雙引號中，如 [Hooks](#hooks) 下的 `hooks/hooks.json` 範例所做，以保持展開的路徑為一個 shell 單詞。當您改為傳遞 `args` 時，每個元素作為一個引數傳遞，沒有 shell，不需要引號。請參閱 [exec 形式和 shell 形式](/docs/zh-TW/hooks#exec-form-and-shell-form)
* **匹配外掛程式自己的 MCP 工具**：來自此外掛程式宣告的 [MCP 伺服器](#mcp-servers) 的工具命名為 `mcp__plugin_<plugin>_<server>__<tool>`，因此在匹配器中寫入該完整名稱。僅在伺服器名稱上的匹配器從不觸發。請參閱 [匹配 MCP 工具](/docs/zh-TW/hooks#match-mcp-tools)

<h3 id="mcp-servers">
  MCP 伺服器
</h3>

MCP 伺服器從外部系統為 Claude 提供工具。在外掛程式根目錄的 `.mcp.json` 中宣告它，形狀與 [專案 `.mcp.json`](/docs/zh-TW/mcp#project-scope) 相同。此 `.mcp.json` 宣告一個命名為 `db` 的伺服器：

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

您也可以省略 `mcpServers` 包裝器，將 `db` 放在檔案的頂層。

載入外掛程式並執行 `/mcp` 以確認伺服器顯示為 `plugin:my-plugin:db`。

`claude plugin validate` 檢查 `.mcp.json` 並報告 Claude Code 在載入時會丟棄的伺服器項目作為錯誤。需要 Claude Code v2.1.281 或更新版本。

如需壞項目在載入時顯示的位置，請參閱 [不啟動的 MCP 伺服器](/docs/zh-TW/plugins/troubleshooting#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start)。

`mcpServers` manifest 鍵採用內嵌伺服器對應、JSON 檔案的路徑或這些的陣列。當 manifest 伺服器與 `.mcp.json` 中的伺服器同名時，manifest 伺服器取代它。

<h4 id="reach-users-on-claude-ai-and-cowork">
  到達 claude.ai 和 Cowork 上的使用者
</h4>

本機 stdio 伺服器（例如 [MCP 伺服器](#mcp-servers) 下的 `db` 伺服器）在 Claude Code 和在 Claude Desktop 應用程式中在您的機器上執行的 Cowork 工作階段中執行，但不在 claude.ai 上。若要到達那裡的使用者，請透過其 `https://` URL 參考遠端伺服器，claude.ai 和 Cowork 將其作為連接器提供給使用者。

<h4 id="server-names-tool-names-and-reloads">
  伺服器名稱、工具名稱和重新載入
</h4>

伺服器的名稱、變數替換和重新載入行為遵循這些規則：

* **伺服器名稱**：`plugin:<plugin>:<server>`，所以 `my-plugin` 中的 `db` 伺服器在 `/mcp` 中是 `plugin:my-plugin:db`。使用相同的形式在 [`mcp_tool` hook](/docs/zh-TW/hooks#mcp-tool-hook-fields) 中命名伺服器
* **工具名稱**：`mcp__plugin_<plugin>_<server>__<tool>`，所以該 `db` 伺服器上的 `query` 工具是 `mcp__plugin_my-plugin_db__query`。這是在 [權限規則](/docs/zh-TW/permissions) 和 [hook 匹配器](#hooks) 中使用的名稱
* **替換**：`${CLAUDE_PLUGIN_ROOT}` 和其他 [路徑變數](#path-variables-and-persistent-data) 在 `command`、`args` 和 `env` 中被替換。`args` 中不需要引號，因為每個元素作為一個引數傳遞
* **重新載入**：當使用者執行 `/reload-plugins` 且 [重新載入適用](/docs/zh-TW/plugins/cli-reference#reloads-that-change-mcp-tools) 時，配置未變更的伺服器保持其連接。配置已變更的伺服器重新連接，您移除的伺服器斷開連接

<h4 id="include-a-packaged-mcpb-server">
  包含打包的 MCPB 伺服器
</h4>

`mcpServers` 鍵也接受打包的伺服器作為 [MCPB 檔案](https://github.com/modelcontextprotocol/mcpb)，其副檔名為 `.mcpb` 或較舊的 `.dxt`。將鍵指向檔案，作為外掛程式內的路徑或 `https://` URL：

```json .claude-plugin/plugin.json theme={null}
{
  "name": "my-plugin",
  "mcpServers": "./servers/db.mcpb"
}
```

伺服器從捆綁的 manifest 中的 `name` 獲取其名稱。

如需傳輸和驗證，請參閱 [MCP](/docs/zh-TW/mcp#plugin-provided-mcp-servers)。

<h3 id="lsp-servers">
  LSP 伺服器
</h3>

LSP 伺服器為 Claude 提供語言的診斷和程式碼導航。如果 [官方程式碼智慧外掛程式](/docs/zh-TW/plugins/code-intelligence) 已涵蓋您的語言，請安裝該外掛程式而不是寫一個。否則在外掛程式根目錄的 `.lsp.json` 中宣告伺服器：

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

檔案直接將每個伺服器名稱對應到其配置，在對應周圍沒有包裝物件。`command` 是二進位檔的名稱，其引數在 `args` 中。`extensionToLanguage` 需要至少一個副檔名，每個以 `.` 開頭。

`claude plugin validate` 不讀取此檔案。當任何項目無效時，整個檔案在載入時被跳過，`Invalid LSP server config for ".lsp.json"` 出現在 `/plugin` **Errors** 標籤中。

您的外掛程式配置連接但不安裝伺服器二進位檔，每個檔案副檔名獲得一個伺服器：

* **缺少二進位檔**：Claude Code 從使用者的 `PATH` 按名稱啟動 `command`。當二進位檔不存在時，伺服器無法啟動，`claude --debug` 記錄 `LSP server <name> failed to start`
* **副檔名衝突**：當兩個啟用的伺服器聲稱相同的副檔名時，首先註冊的處理這些檔案，另一個不用於它們，無論伺服器來自一個外掛程式還是兩個。`/plugin` **Errors** 標籤顯示警告 `LSP server "<name>" is not used for <ext> files`

`lspServers` manifest 鍵採用相同的對應內嵌、JSON 檔案的路徑或這些的陣列，其伺服器新增至 `.lsp.json` 中的伺服器。當 manifest 伺服器與 `.lsp.json` 中的伺服器同名時，manifest 伺服器取代它。

如需 `transport`、逾時、重新啟動和其他欄位，請參閱 [`lspServers`](/docs/zh-TW/plugins/manifest-reference#lspservers)。

將日誌輸出傳送至 stderr，而不是 stdout。Claude Code 僅將伺服器的 stdout 讀取為協議訊息，並接受最多 64 KiB 的訊息標頭和最多 32 MiB 的訊息正文。

Claude Code 斷開超過任一限制或將非協議輸出寫入 stdout 的伺服器，並將斷開連接計為 `restartOnCrash` 和 `maxRestarts` 的當機。當您使用 `--debug` 執行時，Claude Code 將命名原因的錯誤寫入偵錯日誌。

<h3 id="executables">
  可執行檔
</h3>

外掛程式根目錄的 `bin/` 中的檔案在啟用外掛程式時位於 Bash 工具的 shell 的 `PATH` 上，因此 Claude 可以將它們作為裸命令執行。新增可執行指令碼：

```bash bin/hello-plugin theme={null}
#!/bin/bash
echo "hello from my-plugin"
```

使用 `chmod +x bin/hello-plugin` 使其可執行並載入外掛程式。當您要求 Claude 執行 `hello-plugin` 時，Bash 工具結果顯示指令碼的輸出。

外掛程式 `bin/` 目錄位於使用者自己的 `PATH` 項目之後，因此外掛程式無法遮蔽 `git`、`ls` 或其他系統命令。

claude.ai 和 Cowork 不安裝具有頂層 `bin/` 目錄的外掛程式，包括您 [透過 claude.ai 組織設定分發](/docs/zh-TW/plugins/host-marketplace#distribute-through-organization-settings) 的外掛程式。

<h3 id="default-settings">
  預設設定
</h3>

若要設定在啟用外掛程式時適用的預設值，在外掛程式根目錄新增 `settings.json`，或將相同的物件內嵌放在 `settings` manifest 鍵中。兩個鍵生效，`agent` 和 `subagentStatusLine`，所有其他鍵都被丟棄。

設定 `agent` 以執行外掛程式自己的一個 agents 作為主執行緒：

```json settings.json theme={null}
{
  "agent": "security-reviewer"
}
```

載入外掛程式並啟動工作階段。Claude 然後使用 `security-reviewer` agent 的系統提示和模型在主對話中回答。

如需鍵控制的所有內容，請參閱 [`agent` 設定](/docs/zh-TW/settings-reference#agent)。

當相同的鍵在多個位置設定時，這些規則決定哪個值適用：

* **檔案優於 manifest**：當兩者都存在且 `settings.json` 設定至少一個支援的鍵時，`settings.json` 適用，manifest 的 `settings` 被忽略
* **使用者設定優於外掛程式預設值**：在設定來源中，外掛程式預設值是最低層，因此使用者自己在 `~/.claude/settings.json` 中的 `agent` 覆蓋您的
* **兩個外掛程式設定相同的鍵**：來自最後載入的外掛程式的值適用，`claude --debug` 記錄 `overrides setting`

如需 `subagentStatusLine` 形狀，請參閱 [子代理狀態行](/docs/zh-TW/statusline#subagent-status-lines)。

<h3 id="themes-and-output-styles">
  主題和輸出樣式
</h3>

外掛程式可以包含色彩主題和輸出樣式。兩者都出現在與使用者自己相同的選擇器中。對於任一個，設定 manifest 鍵取代資料夾掃描。

| 元件   | 儲存為                       | 格式                                                                                                   | 出現在                                  | Manifest 鍵            |
| :--- | :------------------------ | :--------------------------------------------------------------------------------------------------- | :----------------------------------- | :-------------------- |
| 主題   | `themes/<slug>.json`      | 使用者在 `~/.claude/themes/` 中寫入的 [自訂主題檔案](/docs/zh-TW/terminal-config#create-a-custom-theme) 格式              | `/theme`，在檔案的 `name` 下               | `experimental.themes` |
| 輸出樣式 | `output-styles/<name>.md` | [自訂輸出樣式](/docs/zh-TW/output-styles#create-a-custom-output-style) 格式，具有 `name` 和 `description` frontmatter | `/output-style`，作為 `<plugin>:<name>` | `outputStyles`        |

外掛程式主題是唯讀的，因此當使用者在 `/theme` 中編輯一個時，編輯會儲存為其自己的主題目錄中的副本。

此主題在深色預設上重新著色提示符號重點和錯誤文字：

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
  頻道
</h3>

[頻道](/docs/zh-TW/channels) 讓外部系統（例如聊天應用程式）將訊息傳送到工作階段。在外掛程式中，頻道是 MCP 伺服器之一加上 `channels` 項目，該項目綁定到它並可以提示其自己的配置。此 manifest 將頻道綁定到 `telegram` 伺服器並要求機器人令牌：

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

`server` 必須符合 `mcpServers` 中的鍵。每個頻道的 `userConfig` 採用與 [頂層 `userConfig` 鍵](#user-configuration) 相同的形狀。

如需伺服器必須實現的內容以及使用者如何啟用頻道外掛程式，請參閱頻道參考中的 [打包為外掛程式](/docs/zh-TW/channels-reference#package-as-a-plugin)。如需欄位表，請參閱 [`channels`](/docs/zh-TW/plugins/manifest-reference#channels)。

<h3 id="monitors">
  監視器
</h3>

監視器是在整個工作階段的背景中執行的 shell 命令。它列印的內容作為通知到達 Claude，因此 Claude 可以對日誌或狀態變更做出反應，而無需被要求監視它。將項目儲存在 `monitors/monitors.json` 中：

```json monitors/monitors.json theme={null}
[
  {
    "name": "error-log",
    "command": "tail -F ./logs/error.log",
    "description": "Application error log"
  }
]
```

命令在 shell 中執行，在工作階段啟動的工作目錄中。

監視器的命令在其啟動位置和可以參考的內容方面受到限制：

* **僅互動式工作階段**：外掛程式監視器在互動式工作階段中啟動，從不在使用 `-p` 旗標的非互動式模式中。它們也只在 [Monitor 工具](/docs/zh-TW/tools-reference#monitor-tool) 可用的地方啟動
* **無使用者設定**：`command` 從環境中獲取 [路徑變數](#path-variables-and-persistent-data) 和 `${ENV_VAR}`，但從不獲取 `${user_config.*}`。參考一個的監視器不啟動，監視器程序也不接收 `CLAUDE_PLUGIN_OPTION_<KEY>`
* **中途停用**：如果您在工作階段中途停用外掛程式，Claude Code 不會停止已執行的監視器。它們在工作階段結束時停止

`experimental.monitors` manifest 鍵採用相同的陣列內嵌或 JSON 檔案的路徑，並代替 `monitors/monitors.json` 讀取。

如需 `when` 觸發器和其他欄位，請參閱 [`monitors`](/docs/zh-TW/plugins/manifest-reference#monitors)。

<h2 id="user-configuration">
  要求使用者提供設定值
</h2>

在 `userConfig` manifest 鍵中宣告您的外掛程式需要的使用者值，以便使用者不會自行編輯 `settings.json`。每個選項在對話方塊中顯示，其 `title` 作為標籤，其 `description` 在下方。

為令牌或密碼設定 `"sensitive": true`。對話方塊然後遮蔽輸入，值儲存在安全儲存中，而不是 `settings.json`。

此 manifest 要求端點和令牌：

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
  設定對話方塊何時出現
</h3>

對話方塊僅在互動式 `/plugin` 介面中出現。當使用者執行以下任何操作時，它會為任何尚未設定的選項開啟：

* 在 `/plugin` 中安裝外掛程式
* 在工作階段內執行 `/plugin install <plugin>@<marketplace>`
* 從 `/plugin` 中的 **Installed** 標籤啟用外掛程式

若要在任何時間開啟相同的對話方塊，使用者執行 `/plugin configure <plugin>@<marketplace>`。

`claude plugin install` shell 命令從不提示 `userConfig` 值。若要從 shell 設定值，將每個值作為 `--config KEY=VALUE` 傳遞。當選項保持未設定時，命令列印 `userConfig options not yet set` 行，命名兩種設定方式。[`userConfig` 對話方塊從不出現](/docs/zh-TW/plugins/troubleshooting#the-userconfig-dialog-never-appears) 引用該行。

如需選項欄位、每個值儲存的位置、元件如何參考已儲存的值以及哪些欄位拒絕 `${user_config.*}`，請參閱 [使用者設定](/docs/zh-TW/plugins/manifest-reference#user-configuration)。

<h2 id="path-variables-and-persistent-data">
  參考外掛程式路徑和儲存資料
</h2>

您不知道您的外掛程式將安裝在哪裡，因此透過這些變數而不是固定路徑參考其檔案和資料。它們在 skill、命令和 agent 內容、hook 和監視器命令以及 MCP 和 LSP 伺服器配置中被替換。它們也被匯出到 hook、MCP 和 LSP 程序：

* **`${CLAUDE_PLUGIN_ROOT}`**：外掛程式的安裝目錄。每個版本都有自己的 [快取目錄](/docs/zh-TW/plugins/loading#find-plugins-on-disk)，因此當外掛程式更新時路徑會變更。不要在那裡寫入狀態
* **`${CLAUDE_PLUGIN_DATA}`**：一個在更新中倖存的目錄，用於 `node_modules`、虛擬環境和快取。它解析為 `~/.claude/plugins/data/<id>/`，並在首次參考時建立
* **`${CLAUDE_PROJECT_DIR}`**：專案根目錄，hooks 接收的相同值

在資料目錄路徑中，`<id>` 是外掛程式識別碼，每個字元除了字母、數字、`_` 和 `-` 外都被 `-` 取代，因此 `my-plugin@my-marketplace` 變成 `my-plugin-my-marketplace`。

在 Windows 上，替換的路徑使用正斜杠，因此 shell 不會將反斜杠讀取為逸出。

<h3 id="install-dependencies-into-the-data-directory">
  將相依性安裝到資料目錄
</h3>

對於市場安裝的外掛程式，Claude Code 在快取外掛程式時自動安裝符合條件的 [Node.js 套件相依性](/docs/zh-TW/plugins/loading#node-js-package-dependencies)，因此您可能不需要自行安裝它們。當您執行時，此 `SessionStart` hook 在首次執行時將 `node_modules` 安裝到 `${CLAUDE_PLUGIN_DATA}` 中，並在更新變更 `package.json` 後再次安裝：

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

在第一個工作階段後，`~/.claude/plugins/data/<id>/node_modules` 存在。MCP 伺服器然後可以在其 `env` 中設定 `NODE_PATH` 為 `${CLAUDE_PLUGIN_DATA}/node_modules`。如需哪些欄位替換哪個變數，請參閱 [環境變數](/docs/zh-TW/plugins/manifest-reference#environment-variables)。

<h2 id="next-steps">
  後續步驟
</h2>

* [外掛程式 manifest 參考](/docs/zh-TW/plugins/manifest-reference)：`plugin.json` 欄位、路徑規則和標準配置
* [使用 evals 測試外掛程式](/docs/zh-TW/plugin-evals)：檢查您新增的元件以您的意圖改變 Claude 的行為
* [發佈和分發外掛程式](/docs/zh-TW/plugins/publish)：版本化外掛程式並將其放在市場中
* [疑難排解外掛程式](/docs/zh-TW/plugins/troubleshooting)：當元件無法載入或 hook 無法觸發時該怎麼辦
