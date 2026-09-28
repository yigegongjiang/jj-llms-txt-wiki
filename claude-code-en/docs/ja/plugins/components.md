> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# プラグインにコンポーネントを追加する

> スキル、フック、MCP サーバー、その他すべてのコンポーネントタイプを Claude Code プラグインに追加し、各コンポーネントの検証例を含めます。

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

Claude Code プラグインはスキル、エージェント、フック、MCP サーバーなどのコンポーネントから構築されます。各コンポーネントはプラグイン内にデフォルトフォルダを持ち、`.claude-plugin/plugin.json` 内のオプションのマニフェストキーがそのフォルダを置き換えるか追加し、ユーザーが見る名前があります。各キーの完全なフィールドテーブルについては、[マニフェストリファレンス](/docs/ja/plugins/manifest-reference#fields)を参照してください。

このページを使用して、既に読み込まれているプラグインにコンポーネントを追加します。

コンポーネントを追加した後、実行中のセッションで `/reload-plugins` を実行するか、新しいセッションを開始して Claude Code がそれを読み込むようにします。コンポーネントのファイルを読み込む前に確認するには、プラグインディレクトリからシェルで [`claude plugin validate .`](/docs/ja/plugins/cli-reference#plugin-validate) を実行します。

<Note>
  これらのケースは他のページで説明されています：

  * **最初のプラグインを構築する**：[プラグインを作成する](/docs/ja/plugins/create)から始めます
  * **他のユーザーのプラグインをインストールする**：[プラグインをインストールする](/docs/ja/plugins/install)を参照してください
  * **プラグインのユーザーが claude.ai または Cowork にいる**：異なるセットのコンポーネントがそこに読み込まれます。[claude.ai と Cowork のプラグイン](https://claude.com/docs/plugins/overview)を参照してください
</Note>

<h2 id="explore-the-plugin-directory">
  プラグインディレクトリを探索する
</h2>

エクスプローラーは、デフォルトの場所にあらゆる種類のコンポーネントを 1 つずつ持つ例のプラグイン `my-plugin` を示しています：

* レビュースキルと `about` コマンド
* セキュリティレビューサブエージェント
* Claude がファイルを編集した後にファイルをフォーマットするフック、およびそれが呼び出す `scripts/` フォルダ
* ログモニター
* 出力スタイルとカラーテーマ
* ルート監査ワークフロー
* `hello-plugin` 実行可能ファイル
* デフォルト設定
* ローカル MCP サーバーと Go 言語サーバー

各ファイルはその形式の最小限の有効な例であり、有用であるためではなく形状を示すためにあります：実際のスキルまたはエージェントは完全な指示を持ち、多くの場合サポートファイルを含み、実際のフックまたはモニターは実際の作業を行います。エクスプローラーの後のセクションはエクスプローラーと同じファイルを例として使用し、より完全なものへのリンクを提供します。ファイルまたはフォルダを選択して、それが何のためにあるのか、何が含まれるのか、それをカバーするセクションを見つけます。

<PluginExplorer>
  <Piece id="manifest">
    [マニフェスト](/docs/ja/plugins/manifest-reference)はプラグインの `.claude-plugin/` ディレクトリ内の `plugin.json` ファイルです。プラグインのメタデータと、Claude Code がユーザーに求める `userConfig` 値が含まれます。`name` のみが必須です。このマニフェストでは、`description` はユーザーが `/plugin` でプラグインに対して見るテキストであり、`version` はユーザーをそのバージョンに保ちます。変更するまで：

    ```json theme={null}
    {
      "name": "my-plugin",
      "version": "1.0.0",
      "description": "Review, formatting, and database tools for this team"
    }
    ```
  </Piece>

  <Piece id="skills">
    [スキル](/docs/ja/skills)は `SKILL.md` ファイルです。各スキルを `skills/` の下の独自のディレクトリに保存します。Claude はすべてのスキルの `description` を読み、ユーザーが求めるものがそれと一致する場合（ここでプルリクエストをレビューするよう Claude に求めるなど）、Claude はスキルの指示を読み込んでそれに従います。ユーザーは `/my-plugin:review` として直接実行することもできます：

    ```markdown theme={null}
    ---
    description: Reviews a pull request for style and test coverage. Use when asked to review code.
    ---

    Review the changed files. Report style problems first, then missing tests.
    ```
  </Piece>

  <Piece id="commands">
    コマンドは、ユーザーが名前で実行する単一の Markdown ファイルです。コマンドは古い形式です：スキルは同じ方法で名前で実行でき、独自のディレクトリにサポートファイルを含めることもできるため、新しいものはスキルとして記述し、既に持っているファイルについては `commands/` を保持します。このファイルは `/my-plugin:about` になり、スキルと同じフロントマターを取ります：

    ```markdown theme={null}
    ---
    description: Summarize the repository
    ---

    Summarize what this repository does in three sentences.
    ```
  </Piece>

  <Piece id="agents">
    [サブエージェント](/docs/ja/sub-agents)は、独自の指示と独自のコンテキストウィンドウを持つ別のアシスタントであり、Claude がタスクを委譲して結果を取得できます。`agents/` の下の各 Markdown ファイルは 1 つを定義します：フロントマターはそれに名前を付け、いつ使用するかを言い、本文はそのシステムプロンプトです。このファイルは `my-plugin:security-reviewer` という名前で、ユーザーは `@agent-my-plugin:security-reviewer` で呼び出すことができます：

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
    [フック](/docs/ja/hooks-guide)は Claude Code のライフサイクルの特定の時点（すべてのファイル編集後など）で自動的に何かを実行します：シェルコマンド、HTTP リクエスト、MCP ツール呼び出し、モデルへのプロンプト、またはサブエージェント。プラグインのフックをプラグインルートの `hooks/hooks.json` に保存します。このフックは Claude がファイルを書き込むか編集した後、プラグインの `scripts/format.sh` を実行します：

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
    モニターはシェルコマンドで、Claude Code はセッションの開始時にバックグラウンドで開始し、セッションが終了するまで実行し続け、[Monitor ツール](/docs/ja/tools-reference#monitor-tool)を使用します。それが出力するものは Claude に通知として到達します。`when` フィールドは、代わりに名前付きスキルが初めて実行されるときに開始できます。このモニターはエラーログをテールします：

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
    プラグインは[出力スタイル](/docs/ja/output-styles)を含めることができます。これは Claude が返信をフォーマットおよび表現する方法を変更します。各出力スタイルを `output-styles/<name>.md` として保存します。このスタイルは `/output-style` に `my-plugin:terse` として表示されます：

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
    プラグインは Claude Code インターフェースの[カラーテーマ](/docs/ja/terminal-config#create-a-custom-theme)を含めることができます。各テーマを `themes/<slug>.json` として保存します。このテーマは `/theme` に `Dracula` として表示され、`my-plugin` からのものとしてマークされます：

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
    `workflows/` フォルダは[ワークフロー](/docs/ja/workflows) `.js` ファイルを保持します：`meta` ブロック、その後、複数のサブエージェントを調整するスクリプト本文。このファイルは `/my-plugin:audit-routes` として実行されます：

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
    `bin/` はプラグインがコマンドラインツールを配布する方法です。プラグインが有効な間、Claude Code はこのフォルダをシェルの `PATH` に配置するため、Claude またはスキルの指示は、ユーザーが何もインストールすることなく、ツールを名前で実行できます。この[実行可能ファイル](#executables)が配置されている場合、`hello-plugin` は Claude が実行できるコマンドです：

    ```bash theme={null}
    #!/bin/bash
    echo "hello from my-plugin"
    ```
  </Piece>

  <Piece id="scripts">
    `hooks/hooks.json` のフックはスクリプトを実行し、このフォルダは例がそれを保持する場所です。`scripts/` という名前は慣例であり、Claude Code が探すものではありません：フックはファイルをそのパス `${CLAUDE_PLUGIN_ROOT}/scripts/format.sh` で指します。フォーマッタスクリプトは次のようになります：

    ```bash theme={null}
    #!/bin/bash
    npx prettier --write .
    ```
  </Piece>

  <Piece id="settings">
    プラグインルートの `settings.json` は、プラグインが有効な間に適用される[設定](/docs/ja/settings-reference)を保持するため、プラグインはセッションの動作を変更でき、コンポーネントを追加するだけではありません。プラグインから効果を発揮するのは 2 つのキーのみです。[`agent`](/docs/ja/settings-reference#agent) と [`subagentStatusLine`](/docs/ja/settings-reference#subagentstatusline)；他のすべてのキーは削除されます。[デフォルト設定](#default-settings)を参照してください。

    このファイルは `agent` を設定し、セッションのメインスレッドをプラグイン独自の `security-reviewer` エージェントとして実行するため、そのエージェントのシステムプロンプト、ツール制限、およびモデルがセッション全体に適用されます：

    ```json theme={null}
    {
      "agent": "security-reviewer"
    }
    ```
  </Piece>

  <Piece id="mcp">
    [MCP サーバー](/docs/ja/mcp)は Claude に外部システムからのツールを提供します。プラグインルートの `.mcp.json` で宣言します。このサーバーはプラグイン内のスクリプトからローカルサーバーを開始し、`/mcp` に `plugin:my-plugin:db` として表示されます：

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
    LSP サーバーは Claude に言語の[診断とコードナビゲーション](/docs/ja/plugins/code-intelligence)を提供します。プラグインルートの `.lsp.json` でサーバーを宣言します。このサーバーは `.go` ファイルの Go 言語サーバーを接続します：

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
  各種コンポーネントを追加する
</h2>

以下の各セクションでは、1 つの種類のコンポーネントについて説明します。プラグイン内のファイルの場所、検証するサンプル、プラグインが読み込まれた後にユーザーが見るもの、デフォルトの場所を変更するマニフェストキーです。プラグインに必要なものを追加してください。どれも必須ではありません。

<h3 id="skills">
  Skills
</h3>

[skill](/docs/ja/skills) は、Claude がその説明がタスクと一致するときに読み込める `SKILL.md` ファイルです。ユーザーはコマンドとして実行することもできます。各スキルを `skills/` の下の独自のディレクトリに保存します。

```text theme={null}
my-plugin/
├── .claude-plugin/
│   └── plugin.json
└── skills/
    └── review/
        └── SKILL.md
```

`SKILL.md` に `description` を付けて、Claude がいつそれを使用するかを知るようにします。

```markdown skills/review/SKILL.md theme={null}
---
description: Reviews a pull request for style and test coverage. Use when asked to review code.
---

Review the changed files. Report style problems first, then missing tests.
```

プラグインを読み込んだ後、`/my-plugin:review` がスキルを実行します。コマンド名と誰がそれを呼び出せるかは、以下のルールに従います。

* **コマンド名**: `/<plugin>:<directory>` なので、`my-plugin` の `skills/review/SKILL.md` は `/my-plugin:review` です。フロントマターで `name` を設定すると、最後のセグメントが置き換わり、プラグインプレフィックスは残ります。[スキルがコマンド名を取得する方法](/docs/ja/skills#how-a-skill-gets-its-command-name)を参照してください。
* **誰が呼び出すか**: Claude、ユーザー、またはその両方。フロントマターで制御されます。[スキルの呼び出し者を制御する](/docs/ja/skills#control-who-invokes-a-skill)を参照してください。

スキルはデフォルトの `skills/` ディレクトリの外に配置することもできます。

* **追加ディレクトリ**: `skills` マニフェストキーにリストします。`commands` と `agents` とは異なり、デフォルトの `skills/` スキャンを置き換えるのではなく、追加します。
* **プラグインルートの単一スキル**: `skills/` ディレクトリがなく、`skills` マニフェストキーがない場合、プラグインルートの `SKILL.md` は 1 つのスキルとして読み込まれます。フロントマターで `name` を設定してください。そうしないと、マーケットプレイスのインストールはスキルをプラグイン名ではなく、その[キャッシュディレクトリ](/docs/ja/plugins/loading#find-plugins-on-disk)の後に名前を付けます。

プラグインに指示を含めるには、スキルとして記述します。Claude Code はプラグインルートの `CLAUDE.md` を読み込まず、`claude plugin validate` は `CLAUDE.md at the plugin root is not loaded as project context` と警告します。

フロントマターフィールドとサポートファイルについては、[Skills](/docs/ja/skills) を参照してください。

<h3 id="commands">
  Commands
</h3>

コマンドは、ユーザーが `/my-plugin:about` などの名前で実行する単一の Markdown ファイルです。

<Note>
  コマンドは古い形式であり、[スキル](#skills)は新しい作業ではそれに取って代わります。スキルは同じ方法で名前で実行でき、ディレクトリ内にサポートファイルを含めることもできます。`.claude/commands/` から移動しているファイルについては、`commands/` を保持してください。
</Note>

コマンドを `commands/<file>.md` に保存すると、`/<plugin>:<file>` になります。サブディレクトリはセグメントを追加するため、`commands/db/migrate.md` は `/my-plugin:db:migrate` です。

コマンドファイルはスキルと同じフロントマターを取ります。

<h4 id="define-commands-in-the-manifest">
  マニフェストでコマンドを定義する
</h4>

これが必要なのは、コマンドファイルを `commands/` 以外の場所に保持したい場合、または `plugin.json` 内に個別の Markdown ファイルなしで短いコマンドを定義したい場合のみです。`commands` マニフェストキーを設定すると、Claude Code は `commands/` をスキャンする代わりにそれを読み取ります。キーはパス、パスの配列、または各コマンド名を `source` ファイルまたはインライン `content` にマップするオブジェクトを取ります。

このマニフェストは `/my-plugin:about` をインラインで定義し、Markdown ファイルはありません。

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

プラグインを読み込み、セッションで `/my-plugin:about` を実行して、読み込まれたことを確認します。

完全なキー構文については、[`commands`](/docs/ja/plugins/manifest-reference#commands) を参照してください。

<h3 id="agents">
  Agents
</h3>

[subagent](/docs/ja/sub-agents) は、独自の指示とコンテキストウィンドウを持つ別のアシスタントで、Claude がタスクを委譲できます。`agents/` の下の各 Markdown ファイルは 1 つを定義します。

```markdown agents/security-reviewer.md theme={null}
---
name: security-reviewer
description: Reviews code changes for security issues. Use after edits to authentication or input handling.
model: sonnet
---

You are a security reviewer. Read the changed files and report injection, authentication, and secrets-handling risks.
```

このエージェントは `my-plugin:security-reviewer` という名前で、ユーザーは `@agent-my-plugin:security-reviewer` で[明示的に呼び出す](/docs/ja/sub-agents#invoke-subagents-explicitly)ことができます。名前の形式は `<plugin>:<name>` で、`<name>` はフロントマターから、またはファイル名がない場合はファイル名から来ます。

`agents` マニフェストキーは `agents/` スキャンを置き換えます。

<h4 id="organize-agents-in-subfolders">
  エージェントをサブフォルダに整理する
</h4>

プラグインエージェントファイルを `agents/` のサブフォルダに配置できます。Claude Code は[それらを再帰的に読み込み](/docs/ja/sub-agents#choose-the-subagent-scope)、プラグイン名、各サブフォルダ名、ファイル名をコロンで結合して、エージェントのスコープ付き名を形成します。たとえば、`my-plugin` という名前のプラグインの `agents/review/security.md` は `my-plugin:review:security` として読み込まれます。2 つの設定がその名前を変更します。

* フロントマター `name`: ファイル名のみを置き換えるため、`agents/review/security.md` の `name: audit` は `my-plugin:review:audit` として読み込まれます。
* マニフェスト [`agents`](/docs/ja/plugins/manifest-reference#fields) フィールド: そこにリストされたファイルはサブフォルダ名なしで読み込まれるため、`"agents": "./custom/review/security.md"` は `my-plugin:security` として読み込まれます。

<h4 id="frontmatter-fields-in-plugin-agents">
  プラグインエージェントのフロントマターフィールド
</h4>

プラグインエージェントのフロントマターは、以下のルールに従います。

* **サポートされているフィールド**: `name`、`description`、`model`、`effort`、`maxTurns`、`tools`、`disallowedTools`、`skills`、`memory`、`background`、`omitClaudeMd`、`isolation`、`color`、および `experimental` の `cacheTtl` キー。唯一の有効な `isolation` 値は `"worktree"` です。各フィールドが何をするかについては、[サポートされているフロントマターフィールド](/docs/ja/sub-agents#supported-frontmatter-fields)を参照してください。
* **無視されるフィールド**: `permissionMode`、`hooks`、`mcpServers`、および `initialPrompt`。エージェントファイルは独自にフックまたは MCP サーバーを追加できないため、代わりにプラグイン[フック](#hooks)と[MCP サーバー](#mcp-servers)として追加してください。
* **解析されないフロントマター**: エージェントはすべてのフィールドが無視された状態で読み込まれます。ファイルの後に名前が付けられ、その説明は `Agent from my-plugin plugin` と読みます。シェルで [`claude plugin validate`](/docs/ja/plugins/cli-reference#plugin-validate) を実行して、これらのファイルを見つけます。

各フィールドが何をするかと優先順位ルールについては、[Subagents](/docs/ja/sub-agents#supported-frontmatter-fields) を参照してください。

<h3 id="hooks">
  Hooks
</h3>

[フック](/docs/ja/hooks-guide)は、Claude Code のライフサイクルの特定の時点（すべてのファイル編集後など）で自動的に何かを実行します。シェルコマンド、HTTP リクエスト、MCP ツール呼び出し、モデルへのプロンプト、またはサブエージェント。プラグインのフックを、プラグインルートの `hooks/hooks.json` に保存し、トップレベルの `"hooks"` キーの下に、`settings.json` の `hooks` オブジェクトと同じ形で保存します。これにより、既存の設定フックを変更なしでコピーできます。

このフックは、すべての `Write` または `Edit` の後にバンドルされたスクリプトを実行します。

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

スクリプトを `scripts/format.sh` に保存し、実行可能にします。

プラグインを読み込み、Claude にファイルを編集するよう依頼します。終了 0 の `PostToolUse` フックはトランスクリプトに何も表示しないため、[デバッグログ](/docs/ja/hooks#debug-hooks)で実行されたことを確認するか、スクリプト自体が変更したもので確認します。

`hooks/hooks.json` のフックと `hooks` マニフェストキーの両方が読み込まれます。すべてのイベントとそのペイロードについては、[Hook events](/docs/ja/hooks#hook-events) を参照してください。

<h4 id="when-plugin-hooks-fire">
  プラグインフックが発火するとき
</h4>

プラグインのフックは、プラグインのスキルまたはコマンドの 1 つが使用されるのを待ちません。Claude Code はセッションがプラグインを読み込むときにそれらを登録し、その後、それらのイベントで発火します。フックが実行されるときを制限するには、その `matcher` を絞ります。

フックが発火しない場合は、[発火しないフック](/docs/ja/plugins/troubleshooting#failed-to-load-hooks-from-and-hooks-that-dont-fire)を参照してください。

<h4 id="environment-quoting-and-matching-mcp-tools">
  環境、クォート、および MCP ツールのマッチング
</h4>

フックの環境、`${CLAUDE_PLUGIN_ROOT}` のクォート、およびプラグイン独自の MCP ツールのマッチャーは、以下のように機能します。

* **環境**: すべてのフックプロセスは、その環境で `CLAUDE_PLUGIN_ROOT` と `CLAUDE_PLUGIN_DATA` を受け取り、各[ユーザー設定](#user-configuration)値に対して `CLAUDE_PLUGIN_OPTION_<KEY>` を受け取るため、スクリプトはそこからそれらを読み取ることができます。
* **クォート**: `command` に `args` がない場合、シェルを通じて実行されるため、`hooks/hooks.json` の例の下の [Hooks](#hooks) で行うように、`${CLAUDE_PLUGIN_ROOT}` パスを二重引用符で囲んで、展開されたパスを 1 つのシェルワードに保ちます。代わりに `args` を渡す場合、各要素は 1 つの引数として渡され、シェルなしで、クォートは不要です。[exec form と shell form](/docs/ja/hooks#exec-form-and-shell-form) を参照してください。
* **プラグイン独自の MCP ツールのマッチング**: このプラグインが宣言する [MCP サーバー](#mcp-servers)からのツールは `mcp__plugin_<plugin>_<server>__<tool>` という名前が付けられるため、マッチャーにその完全な名前を記述します。サーバー名だけのマッチャーは発火しません。[MCP ツールのマッチング](/docs/ja/hooks#match-mcp-tools)を参照してください。

<h3 id="mcp-servers">
  MCP servers
</h3>

MCP サーバーは、外部システムから Claude にツールを提供します。プラグインルートの `.mcp.json` で宣言し、[プロジェクト `.mcp.json`](/docs/ja/mcp#project-scope) と同じ形で宣言します。この `.mcp.json` は `db` という名前の 1 つのサーバーを宣言します。

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

`mcpServers` ラッパーを省略して、`db` をファイルのトップレベルに配置することもできます。

プラグインを読み込み、`/mcp` を実行して、サーバーが `plugin:my-plugin:db` として表示されることを確認します。

`claude plugin validate` は `.mcp.json` をチェックし、Claude Code が読み込み時にドロップするサーバーエントリをエラーとして報告します。Claude Code v2.1.281 以降が必要です。

不正なエントリが読み込み時にどこに表示されるかについては、[開始しない MCP サーバー](/docs/ja/plugins/troubleshooting#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start)を参照してください。

`mcpServers` マニフェストキーは、インラインサーバーマップ、JSON ファイルへのパス、またはそれらの配列を取ります。マニフェストサーバーが `.mcp.json` のものと同じ名前を持つ場合、マニフェストサーバーがそれを置き換えます。

<h4 id="reach-users-on-claude-ai-and-cowork">
  claude.ai と Cowork でユーザーに到達する
</h4>

ローカル stdio サーバー（[MCP サーバー](#mcp-servers)の下の `db` サーバーなど）は Claude Code と、Claude Desktop アプリでマシン上で実行される Cowork セッションで実行されますが、claude.ai では実行されません。そこでもユーザーに到達するには、`https://` URL でリモートサーバーを参照します。これは claude.ai と Cowork がコネクタとしてユーザーに提供します。

<h4 id="server-names-tool-names-and-reloads">
  サーバー名、ツール名、およびリロード
</h4>

サーバーの名前、変数置換、およびリロード動作は、以下のルールに従います。

* **サーバー名**: `plugin:<plugin>:<server>` なので、`my-plugin` の `db` サーバーは `/mcp` の `plugin:my-plugin:db` です。[`mcp_tool` フック](/docs/ja/hooks#mcp-tool-hook-fields)でサーバーに名前を付けるときに同じ形式を使用します。
* **ツール名**: `mcp__plugin_<plugin>_<server>__<tool>` なので、その `db` サーバーの `query` ツールは `mcp__plugin_my-plugin_db__query` です。これは[権限ルール](/docs/ja/permissions)と[フックマッチャー](#mcp-servers)で使用する名前です。
* **置換**: `${CLAUDE_PLUGIN_ROOT}` および他の[パス変数](#path-variables-and-persistent-data)は、`command`、`args`、および `env` で置換されます。各要素が 1 つの引数として渡されるため、`args` ではクォートは不要です。
* **リロード**: ユーザーが `/reload-plugins` を実行し、[リロードが適用される](/docs/ja/plugins/cli-reference#reloads-that-change-mcp-tools)場合、構成が変更されていないサーバーは接続を保持します。構成が変更されたサーバーは再接続し、削除したサーバーは切断されます。

<h4 id="include-a-packaged-mcpb-server">
  パッケージ化された MCPB サーバーを含める
</h4>

`mcpServers` キーは、拡張子が `.mcpb` または古い `.dxt` である[MCPB ファイル](https://github.com/modelcontextprotocol/mcpb)としてパッケージ化されたサーバーも受け入れます。キーをファイルに指定します。プラグイン内のパスまたは `https://` URL として。

```json .claude-plugin/plugin.json theme={null}
{
  "name": "my-plugin",
  "mcpServers": "./servers/db.mcpb"
}
```

サーバーはバンドルのマニフェストの `name` からその名前を取ります。

トランスポートと認証については、[MCP](/docs/ja/mcp#plugin-provided-mcp-servers) を参照してください。

<h3 id="lsp-servers">
  LSP servers
</h3>

LSP サーバーは、Claude に言語の診断とコードナビゲーションを提供します。[公式コードインテリジェンスプラグイン](/docs/ja/plugins/code-intelligence)がすでに言語をカバーしている場合は、1 つを記述する代わりにそれをインストールしてください。そうでない場合は、プラグインルートの `.lsp.json` で宣言します。

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

ファイルは各サーバー名を直接その構成にマップし、マップの周りにラッパーオブジェクトはありません。`command` はバイナリの名前で、その引数は `args` にあります。`extensionToLanguage` には少なくとも 1 つの拡張子が必要で、各拡張子は `.` で始まります。

`claude plugin validate` はこのファイルを読み取りません。エントリが無効な場合、ファイル全体は読み込み時にスキップされ、`Invalid LSP server config for ".lsp.json"` が `/plugin` **Errors** タブに表示されます。

プラグインは接続を構成しますが、サーバーバイナリをインストールしません。各ファイル拡張子は 1 つのサーバーを取得します。

* **バイナリがない**: Claude Code はユーザーの `PATH` から名前で `command` を開始します。バイナリがない場合、サーバーは開始に失敗し、`claude --debug` は `LSP server <name> failed to start` をログに記録します。
* **拡張子の競合**: 2 つの有効なサーバーが同じ拡張子を要求する場合、最初に登録されたサーバーがそれらのファイルを処理し、もう 1 つはそれらのファイルには使用されません。サーバーが 1 つのプラグインから来ても 2 つから来ても。`/plugin` **Errors** タブは警告 `LSP server "<name>" is not used for <ext> files` を表示します。

`lspServers` マニフェストキーは同じマップをインラインで、JSON ファイルへのパス、またはそれらの配列として取り、そのサーバーは `.lsp.json` のものに追加されます。マニフェストサーバーが `.lsp.json` のものと同じ名前を持つ場合、マニフェストサーバーがそれを置き換えます。

`transport`、タイムアウト、再起動、およびその他のフィールドについては、[`lspServers`](/docs/ja/plugins/manifest-reference#lspservers) を参照してください。

ログ出力を stdout ではなく stderr に送信します。Claude Code はサーバーの stdout をプロトコルメッセージとしてのみ読み取り、メッセージヘッダーは最大 64 KiB、メッセージ本体は最大 32 MiB を受け入れます。

Claude Code は、いずれかの制限を超えるサーバーを切断するか、stdout に非プロトコル出力を書き込み、切断を `restartOnCrash` と `maxRestarts` のクラッシュとしてカウントします。`--debug` で実行すると、Claude Code は原因を名前で指定するエラーをデバッグログに書き込みます。

<h3 id="executables">
  Executables
</h3>

プラグインルートの `bin/` 内のファイルは、プラグインが有効な間、Bash ツールのシェルの `PATH` 上にあるため、Claude はそれらをベアコマンドとして実行できます。実行可能なスクリプトを追加します。

```bash bin/hello-plugin theme={null}
#!/bin/bash
echo "hello from my-plugin"
```

`chmod +x bin/hello-plugin` で実行可能にし、プラグインを読み込みます。Claude に `hello-plugin` を実行するよう依頼すると、Bash ツールの結果はスクリプトの出力を表示します。

プラグイン `bin/` ディレクトリはユーザー独自の `PATH` エントリの後に来るため、プラグインは `git`、`ls`、または別のシステムコマンドをシャドウできません。

claude.ai と Cowork は、トップレベルの `bin/` ディレクトリを持つプラグイン（[claude.ai 組織設定を通じて配布する](/docs/ja/plugins/host-marketplace#distribute-through-organization-settings)ものを含む）をインストールしません。

<h3 id="default-settings">
  Default settings
</h3>

プラグインが有効な間に適用されるデフォルトを設定するには、プラグインルートに `settings.json` を追加するか、同じオブジェクトを `settings` マニフェストキーにインラインで配置します。2 つのキーが有効になり、`agent` と `subagentStatusLine` で、他のすべてのキーは削除されます。

プラグイン独自のエージェントの 1 つをメインスレッドとして実行するように `agent` を設定します。

```json settings.json theme={null}
{
  "agent": "security-reviewer"
}
```

プラグインを読み込み、セッションを開始します。Claude はメイン会話で `security-reviewer` エージェントのシステムプロンプトとモデルで応答します。

キーが制御するすべてのものについては、[`agent` 設定](/docs/ja/settings-reference#agent)を参照してください。

同じキーが複数の場所で設定されている場合、これらのルールは、どの値が適用されるかを決定します。

* **ファイルがマニフェストより優先**: 両方が存在し、`settings.json` が少なくとも 1 つのサポートされているキーを設定する場合、`settings.json` が適用され、マニフェストの `settings` は無視されます。
* **ユーザー設定がプラグインのデフォルトより優先**: 設定ソース全体で、プラグインのデフォルトは最下位レイヤーであるため、ユーザー独自の `~/.claude/settings.json` の `agent` はあなたのものをオーバーライドします。
* **2 つのプラグインが同じキーを設定**: 最後に読み込まれたプラグインからの値が適用され、`claude --debug` は `overrides setting` をログに記録します。

`subagentStatusLine` の形状については、[subagent status lines](/docs/ja/statusline#subagent-status-lines) を参照してください。

<h3 id="themes-and-output-styles">
  Themes and output styles
</h3>

プラグインはカラーテーマと出力スタイルを含めることができます。どちらもユーザー独自のものと同じピッカーに表示されます。どちらかについて、マニフェストキーを設定するとフォルダスキャンが置き換わります。

| Component    | Save as                   | Format                                                                                          | Appears in                            | Manifest key          |
| :----------- | :------------------------ | :---------------------------------------------------------------------------------------------- | :------------------------------------ | :-------------------- |
| Theme        | `themes/<slug>.json`      | ユーザーが `~/.claude/themes/` に記述する[カスタムテーマファイル](/docs/ja/terminal-config#create-a-custom-theme)形式       | `/theme`、ファイルの `name` の下              | `experimental.themes` |
| Output style | `output-styles/<name>.md` | [カスタム出力スタイル](/docs/ja/output-styles#create-a-custom-output-style)形式、`name` と `description` フロントマター付き | `/output-style`、`<plugin>:<name>` として | `outputStyles`        |

プラグインテーマは読み取り専用であるため、ユーザーが `/theme` で 1 つを編集すると、編集は独自のテーマディレクトリにコピーとして保存されます。

このテーマは、ダークプリセットのプロンプトアクセントとエラーテキストを再色付けします。

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
  Channels
</h3>

[チャネル](/docs/ja/channels)により、チャットアプリなどの外部システムがメッセージをセッションに送信できます。プラグインでは、チャネルは MCP サーバーの 1 つと、それにバインドし、独自の構成を求めることができる `channels` エントリです。このマニフェストはチャネルを `telegram` サーバーにバインドし、ボットトークンを要求します。

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

`server` は `mcpServers` のキーと一致する必要があります。チャネルごとの `userConfig` は、[トップレベルの `userConfig` キー](#user-configuration)と同じ形を取ります。

サーバーが実装する必要があるもの、およびユーザーがチャネルプラグインを有効にする方法については、チャネルリファレンスの[プラグインとしてパッケージ化する](/docs/ja/channels-reference#package-as-a-plugin)を参照してください。フィールドテーブルについては、[`channels`](/docs/ja/plugins/manifest-reference#channels) を参照してください。

<h3 id="monitors">
  Monitors
</h3>

モニターは、セッション全体でバックグラウンドで実行されるシェルコマンドです。それが出力するものは Claude に通知として到達するため、Claude は見るよう求められることなく、ログまたはステータス変更に反応できます。エントリを `monitors/monitors.json` に保存します。

```json monitors/monitors.json theme={null}
[
  {
    "name": "error-log",
    "command": "tail -F ./logs/error.log",
    "description": "Application error log"
  }
]
```

コマンドはシェルで実行され、セッションが開始された作業ディレクトリで実行されます。

モニターのコマンドは、開始場所と参照できるものに制限があります。

* **対話型セッションのみ**: プラグインモニターは対話型セッションで開始され、`-p` フラグを使用した非対話型モードでは開始されません。また、[Monitor ツール](/docs/ja/tools-reference#monitor-tool)が利用可能な場所でのみ開始されます。
* **ユーザー設定なし**: `command` は[パス変数](#path-variables-and-persistent-data)と環境からの `${ENV_VAR}` を取得しますが、`${user_config.*}` は取得しません。1 つを参照するモニターは開始されず、モニタープロセスは `CLAUDE_PLUGIN_OPTION_<KEY>` も受け取りません。
* **セッション中の無効化**: セッション中にプラグインを無効にする場合、Claude Code は既に実行されているモニターを停止しません。セッションが終了するときに停止します。

`experimental.monitors` マニフェストキーは同じ配列をインラインで、または JSON ファイルへのパスとして取り、`monitors/monitors.json` の代わりに読み取られます。

`when` トリガーおよび他のフィールドについては、[`monitors`](/docs/ja/plugins/manifest-reference#monitors) を参照してください。

<h2 id="user-configuration">
  ユーザーに設定値を求める
</h2>

プラグインが必要とする値を `userConfig` マニフェストキーで宣言すると、ユーザーが `settings.json` を自分で編集する必要がなくなります。各オプションはダイアログに表示され、その `title` がラベルとして、その `description` がその下に表示されます。

トークンまたはパスワードの場合は `"sensitive": true` を設定してください。ダイアログは入力をマスクし、値は `settings.json` ではなくセキュアストレージに保存されます。

このマニフェストはエンドポイントとトークンを求めます。

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
  設定ダイアログが表示されるとき
</h3>

ダイアログは対話型の `/plugin` インターフェースにのみ表示されます。ユーザーが以下のいずれかを実行したときに、まだ設定されていないオプションに対して開きます。

* `/plugin` でプラグインをインストールする
* セッション内で `/plugin install <plugin>@<marketplace>` を実行する
* `/plugin` の **Installed** タブからプラグインを有効にする

ユーザーがいつでも同じダイアログを開くには、`/plugin configure <plugin>@<marketplace>` を実行します。

`claude plugin install` シェルコマンドは `userConfig` 値のプロンプトを表示しません。シェルから値を設定するには、各値を `--config KEY=VALUE` として渡します。オプションが設定されていない場合、コマンドは `userConfig options not yet set` という行を出力し、それらを設定する両方の方法を示します。[`userConfig` ダイアログが表示されない](/docs/ja/plugins/troubleshooting#the-userconfig-dialog-never-appears)場合、その行が引用されます。

オプションフィールド、各値が保存される場所、コンポーネントが保存された値を参照する方法、および `${user_config.*}` を拒否するフィールドについては、[ユーザー設定](/docs/ja/plugins/manifest-reference#user-configuration)を参照してください。

<h2 id="path-variables-and-persistent-data">
  プラグインパスを参照し、データを保存する
</h2>

プラグインがどこにインストールされるかわからないため、固定パスではなく、これらの変数を通じてそのファイルとデータを参照します。スキル、コマンド、エージェントコンテンツ、フックおよびモニターコマンド、MCP および LSP サーバー構成で置換されます。また、フック、MCP、および LSP プロセスにエクスポートされます：

* **`${CLAUDE_PLUGIN_ROOT}`**：プラグインのインストールディレクトリ。各バージョンは独自の[キャッシュディレクトリ](/docs/ja/plugins/loading#find-plugins-on-disk)を持つため、プラグインが更新されるとパスが変更されます。そこに状態を書き込まないでください
* **`${CLAUDE_PLUGIN_DATA}`**：更新を生き残るディレクトリ。`node_modules`、仮想環境、キャッシュ用。`~/.claude/plugins/data/<id>/` に解決され、最初に参照されるときに作成されます
* **`${CLAUDE_PROJECT_DIR}`**：プロジェクトルート。フックが受け取るのと同じ値

データディレクトリパスでは、`<id>` はプラグイン識別子で、文字、数字、`_`、`-` 以外のすべての文字が `-` に置き換わるため、`my-plugin@my-marketplace` は `my-plugin-my-marketplace` になります。

Windows では、置換されたパスはシェルがバックスラッシュをエスケープとして読み込まないように前方スラッシュを使用します。

<h3 id="install-dependencies-into-the-data-directory">
  データディレクトリに依存関係をインストールする
</h3>

マーケットプレイスでインストールされたプラグインの場合、Claude Code はプラグインをキャッシュするときに適格な[Node.js パッケージ依存関係](/docs/ja/plugins/loading#node-js-package-dependencies)を自動的にインストールするため、自分でインストールする必要がない場合があります。インストールする場合、この `SessionStart` フックは最初の実行時に `${CLAUDE_PLUGIN_DATA}` に `node_modules` をインストールし、更新が `package.json` を変更した後に再度インストールします：

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

最初のセッションの後、`~/.claude/plugins/data/<id>/node_modules` が存在します。MCP サーバーは `NODE_PATH` を `${CLAUDE_PLUGIN_DATA}/node_modules` に設定できます。どのフィールドがどの変数を置換するかについては、[環境変数](/docs/ja/plugins/manifest-reference#environment-variables)を参照してください。

<h2 id="next-steps">
  次のステップ
</h2>

* [プラグインマニフェストリファレンス](/docs/ja/plugins/manifest-reference)：`plugin.json` フィールド、パスルール、標準レイアウト
* [evals でプラグインをテストする](/docs/ja/plugin-evals)：追加したコンポーネントが Claude の動作を意図した方法で変更することを確認します
* [プラグインを公開および配布する](/docs/ja/plugins/publish)：プラグインをバージョン管理し、マーケットプレイスに配置します
* [プラグインのトラブルシューティング](/docs/ja/plugins/troubleshooting)：コンポーネントが読み込まれない場合またはフックが発火しない場合の対処方法
