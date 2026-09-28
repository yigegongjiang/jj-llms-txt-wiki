> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 設定ファイルと優先順位

> Claude Code の設定を変更し、キーが属するスコープを選択し、変更を確認し、複数の場所でキーが設定されている場合に Claude Code が使用する値を学びます。

export const SettingsPrecedence = () => {
  const LEVELS = [{
    n: 1,
    name: 'Managed settings',
    file: 'managed-settings.json, MDM, or the claude.ai console',
    who: 'Your organization',
    w: 390
  }, {
    n: 2,
    name: 'Command line',
    file: 'claude --settings',
    who: 'You, this session',
    w: 420
  }, {
    n: 3,
    name: 'Project local',
    file: '.claude/settings.local.json',
    who: 'You, this project',
    w: 480
  }, {
    n: 4,
    name: 'Shared project',
    file: '.claude/settings.json',
    who: 'Everyone in the project',
    w: 540
  }, {
    n: 5,
    name: 'User',
    file: '~/.claude/settings.json',
    who: 'You, every project',
    w: 600
  }];
  const W = 760;
  const ROW = 58;
  const GAP = 8;
  const TOP = 34;
  const H = TOP + LEVELS.length * (ROW + GAP) + 30;
  const cx = W / 2;
  const mono = 'var(--font-mono, ui-monospace, SFMono-Regular, Menlo, monospace)';
  const sans = 'var(--font-sans, system-ui, -apple-system, sans-serif)';
  return <div className="sp-root not-prose" role="img" aria-label="Settings precedence, highest first: managed settings, command line, project local, shared project, user. A key set at a higher level overrides the same key set lower down.">
      <style>{`
        .sp-root { --sp-text: #1A1918; --sp-sub: #5E5D59; --sp-faint: #8A8880; --sp-fill: #F5F4EF; --sp-stroke: rgba(0,0,0,0.12); --sp-top: #D97757; --sp-top-fill: rgba(217,119,87,0.14); --sp-arrow: #8A8880; margin: 1.25rem 0; }
        .dark .sp-root { --sp-text: #F1EFE9; --sp-sub: #B8B5AD; --sp-faint: #8A8880; --sp-fill: #24231F; --sp-stroke: rgba(255,255,255,0.12); --sp-top-fill: rgba(217,119,87,0.22); --sp-arrow: #8A8880; }
        .sp-root svg { width: 100%; height: auto; display: block; max-width: ${W}px; margin: 0 auto; }
      `}</style>
      <svg viewBox={`0 0 ${W} ${H}`} xmlns="http://www.w3.org/2000/svg">
        <text x={cx} y={18} textAnchor="middle" fontFamily={sans} fontSize="12.5" fontWeight="600" fill="var(--sp-sub)">Highest precedence</text>
        {LEVELS.map((l, i) => {
    const y = TOP + i * (ROW + GAP);
    const x = cx - l.w / 2;
    const top = i === 0;
    return <g key={l.n}>
              <rect x={x} y={y} width={l.w} height={ROW} rx={10} fill={top ? 'var(--sp-top-fill)' : 'var(--sp-fill)'} stroke={top ? 'var(--sp-top)' : 'var(--sp-stroke)'} strokeWidth={top ? 1.5 : 1} />
              <text x={x + 14} y={y + 24} fontFamily={sans} fontSize="14" fontWeight="600" fill="var(--sp-text)">{l.n}. {l.name}</text>
              <text x={x + 14} y={y + 43} fontFamily={mono} fontSize="11.5" fill="var(--sp-sub)">{l.file}</text>
              <text x={x + l.w - 14} y={y + 24} textAnchor="end" fontFamily={sans} fontSize="12" fill="var(--sp-faint)">{l.who}</text>
            </g>;
  })}
        <text x={cx} y={H - 10} textAnchor="middle" fontFamily={sans} fontSize="12.5" fontWeight="600" fill="var(--sp-sub)">Lowest precedence</text>
        <g stroke="var(--sp-arrow)" strokeWidth="1.5" fill="none">
          <line x1={W - 40} y1={TOP + 10} x2={W - 40} y2={H - 38} />
          <path d={`M ${W - 46} ${TOP + 18} L ${W - 40} ${TOP + 10} L ${W - 34} ${TOP + 18}`} />
        </g>
        <text x={W - 40} y={H - 22} textAnchor="middle" fontFamily={sans} fontSize="10.5" fill="var(--sp-faint)">overrides</text>
      </svg>
    </div>;
};

export const SettingsScope = ({defaultSelected = 'project'}) => {
  const FILES = [{
    id: 'user',
    path: '~/.claude/settings.json'
  }, {
    id: 'project',
    path: 'acme-app/.claude/settings.json'
  }, {
    id: 'local',
    path: 'acme-app/.claude/settings.local.json'
  }, {
    id: 'managed',
    path: 'Managed settings',
    ring: 'managed-settings.json, MDM, or the claude.ai console'
  }];
  const SHORT = {
    user: '~/.claude/settings.json',
    project: 'acme-app/.claude/settings.json',
    local: 'acme-app/.claude/settings.local.json',
    managed: 'managed-settings.json, MDM, or the claude.ai console'
  };
  const TILE_MARK = {
    project: 'settings.json',
    local: 'settings.local.json'
  };
  const initial = FILES.some(f => f.id === defaultSelected) ? defaultSelected : 'project';
  const [sel, setSel] = useState(initial);
  const [scale, setScale] = useState(1);
  const [isFullscreen, setIsFullscreen] = useState(false);
  const rootRef = useRef(null);
  const frameRef = useRef(null);
  const CANVAS_W = 862;
  const CANVAS_H = 240;
  useEffect(() => {
    const el = frameRef.current;
    if (!el) return;
    const measure = () => setScale(Math.min(1, el.clientWidth / CANVAS_W));
    measure();
    if (typeof ResizeObserver === 'undefined') {
      window.addEventListener('resize', measure);
      return () => window.removeEventListener('resize', measure);
    }
    const ro = new ResizeObserver(measure);
    ro.observe(el);
    return () => ro.disconnect();
  }, []);
  useEffect(() => {
    const onFsChange = () => setIsFullscreen(!!document.fullscreenElement);
    document.addEventListener('fullscreenchange', onFsChange);
    return () => document.removeEventListener('fullscreenchange', onFsChange);
  }, []);
  const toggleFullscreen = () => {
    if (!rootRef.current) return;
    if (document.fullscreenElement) document.exitFullscreen(); else rootRef.current.requestFullscreen().catch(() => {});
  };
  const COVERAGE = {
    user: ['website', 'api', 'yacme'],
    project: ['yacme', 'tacme', 'cacme'],
    local: ['yacme'],
    managed: ['website', 'api', 'yacme', 'tacme', 'cacme']
  };
  const RINGS = {
    local: {
      l: 282,
      t: 50,
      w: 142,
      h: 124
    },
    project: {
      l: 282,
      t: 50,
      w: 560,
      h: 124
    },
    user: {
      l: 2,
      t: 34,
      w: 446,
      h: 198
    },
    managed: {
      l: 0,
      t: 32,
      w: 862,
      h: 204
    }
  };
  const TILES = [{
    id: 'website',
    name: 'website/',
    left: 30,
    caption: ''
  }, {
    id: 'api',
    name: 'api/',
    left: 160,
    caption: ''
  }, {
    id: 'yacme',
    name: 'acme-app/',
    left: 290,
    caption: ''
  }, {
    id: 'tacme',
    name: 'acme-app/',
    left: 497,
    caption: sel === 'project' ? 'their clone, once you commit the file' : 'their clone'
  }, {
    id: 'cacme',
    name: 'acme-app/',
    left: 704,
    caption: sel === 'project' ? 'fresh clone, once you commit the file' : sel === 'managed' ? 'server-managed only' : 'fresh clone'
  }];
  const FILE_AT = {
    user: {
      machine: 'you',
      tiles: []
    },
    project: {
      machine: null,
      tiles: ['yacme', 'tacme', 'cacme']
    },
    local: {
      machine: null,
      tiles: ['yacme']
    },
    managed: {
      machine: null,
      tiles: []
    }
  };
  const fileAt = FILE_AT[sel];
  const coverage = COVERAGE[sel];
  const ring = RINGS[sel];
  const selFile = FILES.find(f => f.id === sel);
  const FolderIcon = ({open}) => <svg width="15" height="15" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.3" strokeLinejoin="round" aria-hidden="true">
      <path d="M1.5 4.5a1 1 0 0 1 1-1h3.2l1.3 1.5h6a1 1 0 0 1 1 1V12a1 1 0 0 1-1 1h-10.5a1 1 0 0 1-1-1z" />
      {open && <path d="M1.5 7.5h13" />}
    </svg>;
  const FileIcon = () => <svg width="10" height="10" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.3" strokeLinejoin="round" aria-hidden="true">
      <path d="M4 1.5h5.5L13 5v9.5H4z" />
      <path d="M9.5 1.5V5H13" />
    </svg>;
  const CloudIcon = () => <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.3" strokeLinejoin="round" aria-hidden="true">
      <path d="M4.5 12.5h7a2.5 2.5 0 0 0 .4-4.97A3.5 3.5 0 0 0 5.2 6.6 3 3 0 0 0 4.5 12.5z" />
    </svg>;
  const LaptopIcon = () => <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.3" strokeLinejoin="round" aria-hidden="true">
      <rect x="2.5" y="3" width="11" height="7.5" rx="1" />
      <path d="M1 12.5h14" />
    </svg>;
  return <div ref={rootRef} className={'ssc-root not-prose' + (isFullscreen ? ' ssc-fs' : '')}>
      <style>{`
        .ssc-root {
          --ssc-bg: #FFFFFF;
          --ssc-text: #1A1918;
          --ssc-sub: #5E5D59;
          --ssc-faint: #8A8880;
          --ssc-border: rgba(0,0,0,0.12);
          --ssc-panel: #F5F4EF;
          --ssc-tile: #FAFAF8;
          --ssc-clay: #D97757;
          --ssc-clay-bg: rgba(217,119,87,0.14);
          --ssc-label: #B0562F;
          --ssc-hover: rgba(115,114,108,0.10);
          font-family: var(--font-sans, system-ui, -apple-system, sans-serif);
          background: var(--ssc-bg);
          color: var(--ssc-text);
          border: 1px solid var(--ssc-border);
          border-radius: 16px;
          padding: 20px 24px 24px;
          margin: 1.5rem 0;
          box-sizing: border-box;
        }
        .dark .ssc-root {
          --ssc-bg: #1B1A18;
          --ssc-text: #F1EFE9;
          --ssc-sub: #B8B5AD;
          --ssc-faint: #8A8880;
          --ssc-border: rgba(255,255,255,0.12);
          --ssc-panel: #24231F;
          --ssc-tile: #2A2925;
          --ssc-clay-bg: rgba(217,119,87,0.20);
          --ssc-label: #EBC9B7;
        }
        .ssc-fs { display: flex; flex-direction: column; justify-content: center; align-items: center; margin: 0; border-radius: 0; height: 100vh; }
        .ssc-fs .ssc-head { width: 100%; max-width: ${CANVAS_W}px; }
        .ssc-fs .ssc-frame { width: 100%; }
        .ssc-mono { font-family: var(--font-mono, ui-monospace, SFMono-Regular, Menlo, monospace); }
        .ssc-head { display: flex; align-items: flex-start; justify-content: space-between; gap: 12px; margin-bottom: 20px; }
        .ssc-files { display: flex; gap: 8px; flex-wrap: wrap; }
        .ssc-file {
          font-size: 12.5px; font-weight: 430; padding: 8px 13px; border-radius: 10px; cursor: pointer;
          border: 0.5px solid var(--ssc-border); background: var(--ssc-tile); color: var(--ssc-text);
          white-space: nowrap; transition: background 0.2s, border-color 0.2s;
        }
        .ssc-file:hover { filter: brightness(0.97); }
        .ssc-file[aria-pressed="true"] { font-weight: 600; border: 1.5px solid var(--ssc-clay); background: var(--ssc-clay-bg); }
        .ssc-fsbtn {
          display: flex; align-items: center; justify-content: center; width: 28px; height: 28px; flex-shrink: 0;
          border: none; background: none; border-radius: 6px; cursor: pointer; color: var(--ssc-faint); font-size: 15px;
        }
        .ssc-fsbtn:hover { background: var(--ssc-hover); }
        .ssc-frame { width: 100%; max-width: ${CANVAS_W}px; margin: 0 auto; }
        .ssc-canvas { position: relative; width: ${CANVAS_W}px; height: ${CANVAS_H}px; transform-origin: top left; }
        .ssc-machine { position: absolute; top: 42px; height: 182px; background: var(--ssc-panel); border-radius: 16px; }
        .ssc-machine-label { position: absolute; top: 192px; display: flex; align-items: center; gap: 8px; font-size: 13.5px; font-weight: 600; }
        .ssc-tile {
          position: absolute; top: 58px; width: 126px; height: 108px; border-radius: 12px; padding: 11px 12px; box-sizing: border-box;
          background: var(--ssc-tile); border: 0.5px solid var(--ssc-border); opacity: 0.6;
          transition: background 0.25s, border-color 0.25s, opacity 0.25s;
        }
        .ssc-tile.ssc-on { background: var(--ssc-clay-bg); border: 1px solid var(--ssc-clay); opacity: 1; }
        .ssc-tile-name { display: flex; align-items: center; gap: 6px; color: var(--ssc-faint); }
        .ssc-tile.ssc-on .ssc-tile-name { color: var(--ssc-clay); }
        .ssc-tile-name span { font-size: 12px; font-weight: 430; white-space: nowrap; color: var(--ssc-text); }
        .ssc-tile.ssc-on .ssc-tile-name span { font-weight: 600; }
        .ssc-tile-caption { font-size: 10.5px; color: var(--ssc-sub); margin-top: 5px; line-height: 1.35; }
        .ssc-filemark {
          position: absolute; left: 5px; right: 5px; bottom: 8px; display: inline-flex; align-items: center; justify-content: center; gap: 2px;
          font-size: 8.5px; color: var(--ssc-label); background: var(--ssc-bg); border: 1px solid var(--ssc-clay);
          border-radius: 6px; padding: 2px 3px; white-space: nowrap; overflow: hidden;
        }
        .ssc-filemark svg { flex-shrink: 0; }
        .ssc-machine-filemark { display: inline-flex; align-items: center; gap: 4px; margin-left: 10px; font-size: 10.5px; font-weight: 500; color: var(--ssc-label); }
        .ssc-ring {
          position: absolute; border: 2px solid var(--ssc-clay); border-radius: 18px; pointer-events: none;
          transition: left 0.35s ease, top 0.35s ease, width 0.35s ease, height 0.35s ease;
        }
        .ssc-ring-label {
          position: absolute; font-size: 12px; font-weight: 600; color: var(--ssc-label); white-space: nowrap; pointer-events: none;
          transition: left 0.35s ease, top 0.35s ease;
        }
      `}</style>

      <div className="ssc-head">
        <div className="ssc-files" role="group" aria-label="Settings file">
          {FILES.map(f => <button key={f.id} type="button" className="ssc-file ssc-mono" aria-pressed={f.id === sel} onClick={() => setSel(f.id)}>{f.path}</button>)}
        </div>
        <button type="button" className="ssc-fsbtn" onClick={toggleFullscreen} aria-label={isFullscreen ? 'Exit fullscreen' : 'Enter fullscreen'} title={isFullscreen ? 'Exit fullscreen' : 'Fullscreen'}>{isFullscreen ? '⤡' : '⛶'}</button>
      </div>

      <div ref={frameRef} className="ssc-frame" style={{
    height: CANVAS_H * scale + 'px'
  }}>
        <div className="ssc-canvas" style={{
    transform: 'scale(' + scale + ')'
  }}>
          <div className="ssc-machine" style={{
    left: '10px',
    width: '430px'
  }} />
          <span className="ssc-machine-label" style={{
    left: '30px'
  }}><LaptopIcon />Your machine{fileAt.machine === 'you' && <span className="ssc-machine-filemark ssc-mono"><FileIcon />{selFile.path}</span>}</span>
          <div className="ssc-machine" style={{
    left: '460px',
    width: '200px'
  }} />
          <span className="ssc-machine-label" style={{
    left: '470px'
  }}><LaptopIcon />A teammate’s machine</span>
          <div className="ssc-machine" style={{
    left: '682px',
    width: '170px'
  }} />
          <span className="ssc-machine-label" style={{
    left: '692px'
  }}><CloudIcon />A cloud session</span>

          {TILES.map(t => {
    const on = coverage.includes(t.id);
    return <div key={t.id} className={'ssc-tile' + (on ? ' ssc-on' : '')} style={{
      left: t.left + 'px'
    }}>
                <div className="ssc-tile-name"><FolderIcon open={on} /><span className="ssc-mono">{t.name}</span></div>
                {t.caption && <div className="ssc-tile-caption">{t.caption}</div>}
                {fileAt.tiles.includes(t.id) && <span className="ssc-filemark ssc-mono" title={SHORT[sel]}><FileIcon />{TILE_MARK[sel]}</span>}
              </div>;
  })}

          <div className="ssc-ring" style={{
    left: ring.l + 'px',
    top: ring.t + 'px',
    width: ring.w + 'px',
    height: ring.h + 'px'
  }} />
          <span className="ssc-ring-label ssc-mono" style={{
    left: ring.l + 14 + 'px',
    top: ring.t - 26 + 'px'
  }}>{selFile.ring || selFile.path}</span>
        </div>
      </div>
    </div>;
};

設定は、Claude Code の動作を変更する JSON キーです。どのモデルで開始するか、何を確認なしで実行できるか、どのファイルを読み取れないか、ターミナルでどのように見えるか、および組織が強制する内容を決定します。

<Tip>
  特定のキーを検索するには、[すべての設定](/docs/ja/settings-reference)に移動してください。すべてのキーが、設定するファイル、デフォルト、および例とともにリストされています。
</Tip>

Claude Code は `~/.claude/settings.json` などの JSON 設定ファイルから設定を読み込みます。いくつかの場所でそれらを探し、[設定を読み込むファイルが、その設定が誰に適用されるかを決定します](#settings-files-and-who-they-affect)。このページでは、これらのファイルについて説明します。設定をどのファイルに入れるか、設定を変更して適用されたことを確認する方法、および同じキーが複数のファイルで設定されている場合に Claude Code が使用する値について説明します。[権限を構成する](/docs/ja/permissions)では、Claude Code が確認なしで実行できる内容と、`allow`、`ask`、および `deny` ルールを記述する方法について説明します。

<Note>
  このページでは、マシンで実行されている Claude Code について説明します。ターミナル、[VS Code](/docs/ja/vs-code) および [JetBrains](/docs/ja/jetbrains) 拡張機能、および [デスクトップアプリ](/docs/ja/desktop)は、すべて同じ設定ファイルを読み込みます。[Claude Code on the web](/docs/ja/claude-code-on-the-web) のクラウドセッションは別のマシンで実行され、そのうちのいくつかのみを読み込みます。[クラウドセッションの設定](#settings-in-cloud-sessions)を参照してください。
</Note>

<span id="settings-files" />

<span id="configuration-scopes" />

<span id="available-scopes" />

<span id="when-to-use-each-scope" />

<span id="what-uses-scopes" />

<span id="subagent-configuration" />

<span id="where-settings-live" />

<h2 id="settings-files-and-who-they-affect">
  設定ファイルと誰に影響するか
</h2>

Claude Code は 4 つのファイルから設定を読み込み、組織は claude.ai コンソールから管理設定を配信することもできます。各ソースにはスコープがあります。スコープは、その中に保存された設定が適用される人とプロジェクトのセット。それがあなただけ、プロジェクト内のすべての人、またはあなたの組織内のすべての人であるかどうか。

| スコープ       | ファイル                                                                             | 誰に影響するか                                                                                                        | 用途                                    |
| :--------- | :------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------- | :------------------------------------ |
| ユーザー       | `~/.claude/settings.json`                                                        | あなた、このマシン上のすべてのプロジェクト                                                                                          | 個人設定：テーマ、エディターモード、デフォルトモデル、独自の権限ルール   |
| 共有プロジェクト   | `.claude/settings.json`                                                          | このフォルダを含むフォルダで作業しているすべての人。git リポジトリでは、コミットしてチームメイトが取得できるようにします                                                 | チーム権限、hooks、プラグイン、およびプロジェクトが必要とする環境変数 |
| プロジェクトローカル | `.claude/settings.local.json`                                                    | あなた、このプロジェクトのみ。Claude Code はファイルを作成するときに git から除外します。手動で作成する場合は、自分で `.gitignore` に追加してください                     | 1 つのプロジェクトの個人的なオーバーライド、および共有する前のテスト   |
| 管理         | `managed-settings.json` およびその他の[管理ソース](/docs/ja/managed-settings#delivery-mechanisms) | 組織がデプロイするすべての人。あなたが設定するものは何もそれをオーバーライドしません。いくつかの[セキュリティに敏感な例外](#exceptions-to-managed-settings-precedence)を除いて | セキュリティポリシーおよびコンプライアンス要件               |

ファイル列では、`~/.claude` はホームディレクトリの `.claude` フォルダ、ベアの `.claude` はプロジェクト内の `.claude` フォルダです。

<span id="where-each-file-applies" />

<span id="compare-what-each-file-reaches" />

<h3 id="compare-the-scope-of-each-settings-file">
  各設定ファイルのスコープを比較する
</h3>

マシン上に 3 つのプロジェクト `website/`、`api/`、および `acme-app/` があり、チームメイトが `acme-app/` の独自のクローンを持ち、`acme-app/` で[クラウドセッション](#settings-in-cloud-sessions)を開始するとします。

下のグラフィックは、それらのフォルダから Claude Code を開始するときに設定が適用されるフォルダを示しています。設定ファイルをクリックして、到達するフォルダを確認してください。

<SettingsScope />

* **`~/.claude/settings.json`**：マシン上のすべてのプロジェクト、チームメイトまたはクラウドセッションには何もありません
* **`acme-app/.claude/settings.json`**：あなたの `acme-app/`。バージョン管理にファイルをコミットする場合のみ、チームメイトのクローンとクラウドセッションに到達します。それまでは、他のファイルのようにディスク上のファイルであり、誰も持っていません
* **`acme-app/.claude/settings.local.json`**：あなたの `acme-app/` のみ。Claude Code はファイルを初めて書き込むときに、グローバル git 除外に追加するため、コミットから除外されます。手動でファイルを作成する場合は、[自分で `.gitignore` に追加してください](#keep-personal-settings-out-of-a-repository)
* **管理設定**。`managed-settings.json` ファイル、MDM ポリシー、または claude.ai コンソールからの[サーバー管理設定](/docs/ja/server-managed-settings)：組織がデプロイするすべてのマシン上のすべてのプロジェクト、またはあなたの組織アカウントでサインインするマシン。サーバー管理設定のみがクラウドセッションに到達します

<span id="which-files-you-have" />

<h3 id="find-or-create-your-settings-files">
  設定ファイルを見つけるか作成する
</h3>

Claude Code をインストールしても、設定ファイルは作成されません。マシンまたはプロジェクトに既に 1 つある場合は、これらのソースの 1 つから来ました：

* **管理**：組織がデプロイします。作成または編集しません。
* **共有プロジェクト**：Claude Code を既に使用しているプロジェクトにはコミットされたものがあるかもしれません。ない場合は、プロジェクトフォルダに `.claude/settings.json` で作成してください。
* **ユーザー**および**プロジェクトローカル**：自分で作成するか、Claude Code に作成させます。テーマなどのユーザー設定に保存する `/config` メニューのオプションを初めて変更するときに `~/.claude/settings.json` を書き込み、Bash コマンドに対して「はい、今後は聞かないでください」などの権限プロンプトで立ったままの承認を初めて与えるときに `.claude/settings.local.json` を書き込みます。**ヒントを表示**を含むいくつかの `/config` オプションは、ユーザーファイルの代わりに `.claude/settings.local.json` に保存されます。

<Info>
  Windows では、`~/.claude` は `%USERPROFILE%\.claude` を意味します。ホームディレクトリファイルを別の場所に保つには、[`CLAUDE_CONFIG_DIR`](/docs/ja/env-vars) を設定してください。Claude Code はその代わりに設定、セッション履歴、およびプラグインをそこに保存します。
</Info>

Claude Code は、[`~/.claude.json`](/docs/ja/claude-directory#ce-claude-json) という 5 番目のファイルも保持します。これは自分で書き込みます。編集する必要はありません。サインインセッション、[MCP サーバー](/docs/ja/mcp)構成、信頼決定などのプロジェクトごとの状態、および `/config` があなたのために書き込む[グローバル構成キー](/docs/ja/settings-reference#global-config-settings)を保持します。

<h3 id="share-settings-with-your-team">
  チームと設定を共有する
</h3>

`.claude/settings.json` をコミットして、リポジトリをクローンするすべての人が同じ権限、hooks、およびプラグインを取得するようにします。各チームメイトは、個人的な例外がコミットを必要としないように、独自の `.claude/settings.local.json` でそれをオーバーライドできます。完全なチームファイルについては、[チームの共有設定](/docs/ja/settings-example#a-teams-shared-settings)を参照してください。

コミットするものの一部は、各チームメイトが[フォルダを信頼する](/docs/ja/permissions#project-allow-rules-and-workspace-trust)まで待機し、いくつかのキーはリポジトリファイルから効果を発揮しません。[適用されない設定をトラブルシューティングする](#common-cases)は両方をカバーしています。

<span id="local-settings-file" />

<span id="where-claude-code-saves-the-project-local-file" />

<span id="the-project-local-file" />

<span id="keep-personal-settings-out-of-the-repository" />

<h3 id="keep-personal-settings-out-of-a-repository">
  リポジトリから個人設定を除外する
</h3>

プロジェクト内で自分の設定を変更し、チームメイトの設定を変更しないようにするには、プロジェクト内の `.claude/settings.local.json` に保存してください。Claude Code はそのファイルをコミットされた `.claude/settings.json` に適用するため、チームのファイルが `"model": "claude-sonnet-5"` を設定し、Opus が必要な場合は、ローカルファイルに `"model": "claude-opus-5-5"` を入れて、セッションのみを変更します。

Claude Code もこのファイルに書き込み、コミットから除外し、信頼ステップなしに allow ルールを適用します：

* **Claude Code もそれを書き込みます。** Claude が Bash コマンドを実行する権限を求め、「はい、今後は聞かないでください」を選択すると、Claude Code はその[権限承認](/docs/ja/permissions#permission-system)をここに `allow` ルールとして保存します。
* **手動で作成した場合を除き、自分で gitignore する必要はありません。** Claude Code がリポジトリでファイルを初めて書き込むときに、既にそれを無視していない場合、グローバル git 除外ファイルに `**/.claude/settings.local.json` を追加するため、ファイルはすべてのリポジトリのコミットから除外されます。そのファイルは、グローバル git 構成がそれを絶対パスまたは `~` プレフィックス付きパスに設定する場合は `core.excludesFile`。それ以外の場合は `$XDG_CONFIG_HOME/git/ignore`、または `XDG_CONFIG_HOME` が設定されていない場合は `~/.config/git/ignore`。手動でファイルを作成し、Claude Code がまだそれに書き込んでいない場合は、自分で `.gitignore` に追加してください。
* **ファイルが追跡されていない間、その allow ルールは信頼を待ちません。** ファイルはリポジトリのものではなくあなたのものであるため、Claude Code はコミットされたファイルが必要とする[ワークスペース信頼](/docs/ja/permissions#project-allow-rules-and-workspace-trust)ステップなしにその `allow` ルールを適用します。ファイルが git で追跡されている場合、信頼ステップもそれに適用されます。[ローカル設定ファイルが信頼を必要とする場合](/docs/ja/permissions#when-your-local-settings-file-needs-trust)を参照してください。

<span id="where-claude-code-looks-for-each-file" />

<span id="how-claude-code-keeps-the-local-file-out-of-git" />

<span id="local-allow-rules-dont-wait-for-workspace-trust" />

<h4 id="where-claude-code-keeps-the-local-file-in-a-git-repository">
  Claude Code が git リポジトリでローカルファイルを保持する場所
</h4>

Claude が Bash コマンドを実行する権限を求め、「はい、今後は聞かないでください」を選択すると、Claude Code はその承認を `.claude/settings.local.json` の `allow` ルールとして保存します。git リポジトリのサブディレクトリから Claude Code を開始する場合、リポジトリルートでそのファイルを読み取り、書き込み、リポジトリ全体に承認を適用します。[worktree](/docs/ja/worktrees) では、メインチェックアウトのルートのファイルを使用します。

2 つのルールがルートの場所を適格にします：

* **ファイルが `.claude/settings.json` の代わりに留まる場合**：git リポジトリの外、リポジトリルートがホームディレクトリ、Windows、またはリポジトリルート、その `.git` またはその `.claude` エントリがユーザーによって所有されていない場合。
* **ファイル内のパスはリポジトリルートに固定されません**：`/` で始まる権限ルール、または相対サンドボックスパスは、[セッションのプライマリ作業ディレクトリ](/docs/ja/permissions#read-and-edit)に固定されます。

v2.1.211 より前では、Claude Code は開始ディレクトリにファイルを保持していました。以前のバージョンが残したファイルをルートファイルと並行して読み込みます。両方が同じキーを設定する場合、ルートの値が適用され、両方のファイルからの権限ルールが適用されます。Agent SDK の [`resolveSettings()`](/docs/ja/agent-sdk/typescript#resolvesettings) ヘルパーは常に開始ディレクトリからファイルを読み込みます。

Claude Code は共有 `.claude/settings.json` をセッションの[プライマリ作業ディレクトリ](/docs/ja/permissions#working-directories)から読み込むため、リポジトリルートにコミットされたファイルを使用するには、そこから Claude Code を開始してください。[`/cd`](/docs/ja/permissions#move-the-session-to-another-directory) でセッションを移動した後、Claude Code は代わりに新しいディレクトリから両方のプロジェクトファイルを読み込み、同じルールでローカルファイルを配置します。移動したディレクトリから読み込むには Claude Code v2.1.246 以降が必要です。

<span id="managed-settings-delivery" />

<span id="precedence-within-the-managed-tier" />

<span id="parent-settings-from-embedding-hosts" />

<span id="enforce-settings-for-an-organization" />

<span id="settings-your-organization-manages" />

<h3 id="check-what-your-organization-enforces">
  組織が強制する内容を確認する
</h3>

組織が Claude Code を管理する場合、いくつかの設定はあなたのために決定され、独自のファイルに入れるものは何もそれらを変更しません。どれを確認するには、`/status` を実行してください。`Setting sources` 行は、あなたに適用される管理ソースの名前を付けます。管理設定はこのマシンで Claude Code が実行される場所に到達します。[開発者が変更できる内容](/docs/ja/managed-settings#what-a-developer-can-change)はローカル管理者権限と Claude Code 以外のツールをカバーしています。

管理設定は[管理設定ページ](/docs/ja/managed-settings#delivery-mechanisms)の配信メカニズムを通じてあなたに到達します。最も一般的には：

* [サーバー管理設定](/docs/ja/server-managed-settings)。Claude Code が claude.ai 管理コンソールまたは自己ホスト型[Claude apps gateway](/docs/ja/claude-apps-gateway) から取得します
* MDM または OS レベルのポリシー、およびシステムディレクトリの `managed-settings.json` ファイル
* Claude Desktop などの埋め込みホスト。SDK `managedSettings` オプション経由。[埋め込みホストからポリシーを制御する](/docs/ja/managed-settings#parent-settings-from-embedding-hosts)を参照してください

Claude Desktop アプリで実行される[Cowork](https://claude.com/docs/cowork/overview) セッションでは、Claude Code は claude.ai 管理コンソールからサーバー管理設定を取得しません。組織の Claude Desktop 構成が `requireCoworkFullVmSandbox` を設定しない限り、デバイスにデプロイされたポリシーを読み込みます。[ポリシーが適用される場所と時期](/docs/ja/managed-settings#where-and-when-a-policy-applies)は Cowork とクラウドセッションをカバーしています。

管理者の場合、[組織向けに Claude Code をセットアップする](/docs/ja/admin-setup)は何を強制するかを選択する手順を説明し、[管理設定をデプロイする](/docs/ja/managed-settings)は配信と、ポリシーが有効であることを確認する方法をカバーしています。

<h2 id="change-a-setting">
  設定を変更する
</h2>

`/config` メニューから、設定ファイルを編集して、または 1 つのセッションのコマンドラインから設定を変更できます。

<span id="system-prompt" />

Claude Code のシステムプロンプトは公開されていません。Claude に立ったままの命令を与えるには、[`CLAUDE.md` ファイル](/docs/ja/memory)または `--append-system-prompt` フラグを使用してください。

<h3 id="use-the-/config-menu">
  /config メニューを使用する
</h3>

Claude Code 内で `/config` を実行し、**Config** タブを開きます。テーマ、エディターモード、詳細出力などの短いセットの個人的なオプションをリストします。すべての設定キーではありません。オプションを選択して変更します。Claude Code はあなたのために保存します：

* **ほとんどのオプション**：`~/.claude/settings.json`
* **ヒントを表示などのいくつかのオプション**：`.claude/settings.local.json`
* **[グローバル構成オプション](/docs/ja/settings-reference#global-config-settings)**：`~/.claude.json`

1 つのオプションをメニューなしで設定するには、`/config verbose=true` などの `key=value` を渡してください。

<Note>
  `/config` はターミナルインターフェースの一部です。[VS Code](/docs/ja/vs-code) チャットパネルと[デスクトップアプリ](/docs/ja/desktop)はそれを開きません。設定ファイルを編集するか、これらのアプリ独自の設定を通じて設定を変更してください。
</Note>

<h3 id="edit-a-settings-file">
  設定ファイルを編集する
</h3>

エディターで必要なスコープの設定ファイルを開き、キーを追加または変更します。設定ファイルは厳密な JSON です。`//` コメントまたは末尾のコンマは構文エラーであり、Claude Code はファイルを[設定エラー](#fix-a-broken-settings-file)として次の開始時に報告します。たとえば、Claude Code が確認なしに lint とテストコマンドを実行し、`.env` ファイルの読み取りを停止するようにするには、これを `~/.claude/settings.json` に追加してください：

```json ~/.claude/settings.json theme={null}
{
  "$schema": "https://json.schemastore.org/claude-code-settings.json",
  "permissions": {
    "allow": [
      "Bash(npm run lint)",
      "Bash(npm run test *)"
    ],
    "deny": [
      "Read(./.env)",
      "Read(./.env.*)"
    ]
  }
}
```

`permissions` の下の各エントリは、ツールとそれが何ができるかを名前付けするルールです。[権限を構成する](/docs/ja/permissions)は構文を説明します。`$schema` 行は Claude Code 設定の[公開 JSON スキーマ](https://json.schemastore.org/claude-code-settings.json)を指し、VS Code、Cursor、および JSON スキーマをサポートする他のエディターでオートコンプリートとインライン検証を提供します。スキーマは最新の CLI リリースより遅れることができるため、最近ドキュメント化されたキーの検証警告は、構成が無効であることを意味しません。

保存した後、Claude Code 内で `/status` を実行してファイルが読み込まれたことを確認してください。[読み込まれたものを確認する](#check-what-loaded)は `Setting sources` 行が表示するものと、壊れたファイルがどのように報告されるかを説明します。

完全な個人ファイル、チームファイル、および組織ファイルについては、それが設定するすべてのキーのコメント付きで、[設定ファイルの例](/docs/ja/settings-example)を参照してください。

<span id="pass-settings-for-one-session" />

<h3 id="change-a-setting-for-one-session">
  1 つのセッションの設定を変更する
</h3>

値を保存せずに試すには、Claude Code を開始するときに設定します。値はそのセッションに適用され、設定ファイルはそのままです。3 つの方法があります：

* **`--settings`**：JSON としてキーを渡します。インラインまたはファイルへのパス。Claude Code はそれをユーザー、プロジェクト、およびローカルファイルの上に適用し、管理設定の下に適用します。ユーザー設定ファイルが設定できるキーを設定できます。`Managed` または `Global config` キーを設定することはできません。
* **そのキーのフラグ**：いくつかのキーには独自のフラグがあります。`model` の `--model` や `effortLevel` と `modelSettings` の `--effort` など。
* **環境変数**：`model` の `ANTHROPIC_MODEL` など、キーのペアリングされた変数をエクスポートしてから `claude` を実行してください。

各キーの[設定リファレンス](/docs/ja/settings-reference)のエントリは、その 1 セッションのオーバーライドと、どれが優先されるかをリストします。変更したいキーのエントリを確認してください。

セッション内で実行するコマンドはほとんどあなたの選択を保存します。`/config` で設定を変更すると、Claude Code はそれを設定ファイルに書き込み、`/model` は値を新しいセッションのデフォルトとして保存します。

`/model` ピッカーで `s` を押すと、Claude Code はモデルを切り替えますが、ユーザーデフォルトとして保存しません。[努力レベルを調整する](/docs/ja/model-config#adjust-effort-level)は、Claude Code が使用しているモデルのデフォルトとして保存する `/effort` ピックと、現在のセッションのみに適用するものを説明します。

たとえば、デフォルトを変更せずに 1 つのセッションで Opus で開始するには：

```bash theme={null}
claude --settings '{"model": "claude-opus-5-5"}'
```

<h3 id="when-edits-take-effect">
  編集がいつ有効になるか
</h3>

Claude Code は設定ファイルを監視し、変更時に再読み込みするため、`permissions`、`hooks`、および `apiKeyHelper` などの認証情報ヘルパーへの編集を含む、ほとんどの編集は再起動なしで実行中のセッションに適用されます。Claude Code は、セッションの途中で作成した設定ファイルも、そのフォルダがセッション開始時に存在していれば読み込みます。プロジェクトの `.claude/` フォルダについては、同じセッション内でフォルダを作成した場合でもファイルを読み込みます。

再読み込みはユーザー、プロジェクト、ローカル、および管理設定をカバーし、Claude Code は検出した各設定ファイル変更に対して [`ConfigChange` hook](/docs/ja/hooks#configchange) を実行します。MDM または claude.ai コンソールから到達する管理設定ではありません。MDM または claude.ai コンソールから到達する管理設定は、保存時ではなくスケジュールに従って実行中のセッションに到達します。[配信テーブル](/docs/ja/managed-settings#choose-a-delivery-mechanism)はソースごとにそれを提供します。

Claude Code はセッション開始時に一度だけいくつかのキーを読み込むため、そのうちの 1 つへの編集は実行中のセッションに到達しません。`requiredMinimumVersion` などの管理者側のキーも再起動を待ちます。[ポリシーが適用される場所と時期](/docs/ja/managed-settings#where-and-when-a-policy-applies)の下にリストされています。セッション中に編集する可能性が最も高いもの：

* [`model`](/docs/ja/settings-reference#model)：セッション中に切り替えるには [`/model`](/docs/ja/model-config#setting-your-model) を使用してください。各モデルには独自のプロンプトキャッシュがあるため、切り替え後の最初のリクエストは会話全体をキャッシュなしで再読み込みします。[モデルを切り替える](/docs/ja/prompt-caching#switching-models)を参照してください
* [`effortLevel`](/docs/ja/settings-reference#effortlevel) および [`modelSettings`](/docs/ja/settings-reference#modelsettings)：セッション中に努力を変更するには [`/effort`](/docs/ja/model-config#adjust-effort-level) を使用してください

<span id="verify-active-settings" />

<span id="check-what-loaded" />

<h3 id="confirm-what-loaded">
  読み込まれたものを確認する
</h3>

Claude Code 内で `/status` を実行して、どの設定ソースがアクティブであるかを確認してください。**Status** タブには、Claude Code が現在のセッション用に読み込んだ各設定ファイルをリストする `Setting sources` 行が含まれています。`User settings` または `Project local settings` など。[管理設定](/docs/ja/admin-setup#decide-how-settings-reach-devices)が有効な場合、管理設定エントリは括弧内にそれらがマシンに到達した方法を示します。

行は Claude Code が読み込んだファイルを確認します。各ファイルがどのキーを供給したかは表示されません。Claude Code が拒否したエントリをリストするには、[`claude doctor`](/docs/ja/debug-your-config) を実行してください。プロジェクトまたは管理設定が設定するモデルについては、スタートアップヘッダーがそれを設定したファイルの名前を付けます。`/status` と `/config` は異なるタブで同じダイアログを開き、**Config** タブは `settings.json` コンテンツのビューではありません。

<h3 id="fix-a-broken-settings-file">
  壊れた設定ファイルを修正する
</h3>

JSON をタイプミスするか、Claude Code が受け入れない値にキーを設定する場合、Claude Code はインタラクティブセッションの開始時にあなたに伝えます。表示される内容は、ファイルのどの程度が影響を受けるかによって異なります：

* **設定エラー**：ユーザー、プロジェクト、またはローカルファイルに無効な JSON またはスキーマが拒否する値があります。インタラクティブセッションの開始時に Claude Code はダイアログを表示し、Claude の助けでファイルを修正するか、終了するか、壊れた設定なしで続行できます。
* **設定警告**：個別のエントリのみが失敗します。不正な形式の権限ルールまたは不明な hook イベント名など。Claude Code はそれらの値をスキップし、ファイルの残りを有効に保ちます。
* **管理設定**：Claude Code はファイルの残りを強制し続けます。[管理設定の無効なエントリ](/docs/ja/managed-settings#invalid-entries-in-managed-settings)は、それが削除するものと、修正するまでどのキーがより厳密な値にフォールバックするかを説明します。有効な JSON ではない管理設定ドキュメントについては、[管理設定ドキュメントを解析できませんでした](/docs/ja/errors#managed-settings-document-could-not-be-parsed)を参照してください。
* **構成エラー**：`~/.claude.json` を解析できません。Claude Code は壊れたファイルを `~/.claude/backups/.claude.json.corrupted.<timestamp>` にコピーし、終了して手動で修正するか、デフォルト構成にリセットするかを尋ねます。`-p` 実行はエラーを出力して終了します。以前の状態を復旧するには、`~/.claude/backups/` の 5 つの最新 `.claude.json.backup.<timestamp>` ファイルの 1 つをコピーして戻してください。Claude Code はファイルを書き込む前に保存します。

続行した後、`/status` を実行して影響を受けたファイルを確認し、各エラーの詳細について `claude doctor` を実行してください。

`-p` 実行はダイアログを表示しません。[管理設定ドキュメントを解析できない](/docs/ja/errors#managed-settings-document-could-not-be-parsed)場合を除き、Claude Code は壊れたファイルまたは値をスキップして残りで続行するため、設定をスキップした `-p` 実行の後、`claude doctor` を実行して削除したものを確認してください。

<span id="how-scopes-interact" />

<span id="key-points-about-the-configuration-system" />

<span id="which-value-claude-code-uses" />

<span id="which-value-wins" />

<h2 id="settings-precedence">
  設定の優先順位
</h2>

同じキーが複数の場所に表示される場合、Claude Code はそれを設定する最も高いレベルの値を使用します。以下のスタックはレベルを示しており、上が最も高い優先度です。より高いレベルのキーは、その下のどこにある同じキーよりも優先されます。

<SettingsPrecedence />

優先度が高い順に：

1. **管理設定**：組織が `managed-settings.json` ファイル、MDM ポリシー、または [claude.ai コンソールからのサーバー管理設定](/docs/ja/server-managed-settings) によってデプロイする設定です。設定したものは何もこれをオーバーライドしません。`--settings` で渡すキーは同じ管理キーをオーバーライドしませんし、`--model` などのフラグは組織が許可するモデルからのみ選択します。管理 `model` は各セッションが開始するモデルを設定し、`/model` で切り替えることはできます。ロックは [`availableModels`](/docs/ja/settings-reference#availablemodels) で、これは `/model`、`--model`、および独自ファイルの `model` キーを制限します。組織が複数の管理ソースを提供する場合、[管理層内の優先順位](/docs/ja/managed-settings#precedence-within-the-managed-tier) のルールが Claude Code が各ソースから読み込むものを指定します。
2. **コマンドライン引数**：ターミナルから `claude` を開始するときに渡すフラグで、1 つのセッション用です。[1 つのセッションの設定を変更する](#change-a-setting-for-one-session) を参照してください。Claude Code は `--settings <file-or-json>` で渡す JSON を他のレベルと同じルールで設定ファイルとマージします。ここで設定したキーはローカル、プロジェクト、またはユーザー設定の同じキーより優先され、省略したキーについては下位レベルの値を保持します。
3. **プロジェクトローカル設定** （`.claude/settings.local.json`）：このプロジェクトの個人設定です。
4. **共有プロジェクト設定** （`.claude/settings.json`）：チームがソース管理にチェックインする設定です。
5. **ユーザー設定** （`~/.claude/settings.json`）：すべてのプロジェクトの個人設定です。

環境変数はこのスタックのレベルではありません。動作がシェル変数と設定キーの両方を持つ場合、どちらが適用されるかはレベルではなくペアごとに決定されます。シェルでエクスポートされた `ANTHROPIC_MODEL` はどのファイルからの `model` キーよりも優先されますが、`ANTHROPIC_DEFAULT_MODEL` はファイルが `model` を設定していない場合にのみ適用されます。[環境変数リファレンス](/docs/ja/env-vars#precedence) はどのキーがペアを持ち、Claude Code がどちらを最初に読み込むかを示しています。設定ファイル内の `env` ブロックは通常のキーであり、上記のレベルに従います。

いくつかのセキュリティに敏感なキーについて、Claude Code は下位レベルからのより厳密な値を管理値より優先します。[管理設定の優先順位の例外](#exceptions-to-managed-settings-precedence) がそれらをリストしています。

<h3 id="lists-merge-instead-of-overriding">
  リストはオーバーライドではなくマージされます
</h3>

`permissions.allow` などの同じリストキーを複数のファイルで設定する場合、Claude Code はリストを結合し、1 つを選択しません。そのため、各ファイルは別のファイルのエントリを削除することなくエントリを追加できます。モデルリストまたはモデルごとのエントリを保持する 4 つのキーは独自のルールに従います：

* [`fallbackModel`](/docs/ja/settings-reference#fallbackmodel) は位置が意味を持つ順序付きチェーンであるため、Claude Code は最も優先度の高いファイルからの全体の値を取得します。
* [`modelPicker`](/docs/ja/settings-reference#modelpicker) は 1 つの順序付きリストの行とリプレースフラグを保持するため、Claude Code は 2 つのソースから行をマージしません。管理設定、`--settings`、およびユーザー設定の最も高いものから全体の値を取得し、プロジェクトおよびローカル設定のキーを無視します。Claude Code v2.1.242 以降が必要です。
* [`availableModels`](/docs/ja/settings-reference#availablemodels)：Claude Code が適用する管理設定がこれを定義する場合、Claude Code はそのリストをそのまま適用し、ユーザー、プロジェクト、またはローカル設定で追加するエントリを無視します。ただし、Claude Code を埋め込むアプリが独自のモデルリストを提供する場合を除きます。[管理設定の優先順位の例外](#exceptions-to-managed-settings-precedence) を参照してください。管理ソース全体でリストはマージされません。[Claude Code が管理ソースを結合する方法](/docs/ja/managed-settings#how-claude-code-combines-managed-sources) はどのソースのリストが適用されるかを示しています。非管理スコープ全体で Claude Code は通常どおり配列をマージします。
* [`modelSettings`](/docs/ja/settings-reference#modelsettings)：Claude Code はそれを [`effortLevel`](/docs/ja/settings-reference#effortlevel) と一緒に一度に 1 つのモデルで解決します。`modelSettings` エントリはどのファイルの値がモデルに適用されるかを示しています。

<span id="examples" />

<h3 id="precedence-examples">
  優先順位の例
</h3>

Claude が動作している間、Claude Code はスピナーの下に 1 行のヒントを表示します。例えば「/config を使用してデフォルト権限モード（Plan Mode を含む）を変更します」。[`spinnerTipsEnabled`](/docs/ja/settings-reference#spinnertipsenabled) を `~/.claude/settings.json` で `false` に設定したいとします。以下の各シナリオはそれらをオンに戻す可能性があるもので、それについて何ができるかを示しています。

<h4 id="team-settings-override-personal-settings">
  チーム設定が個人設定をオーバーライドします
</h4>

チームの `.claude/settings.json` がそれを `true` に設定しています。Claude Code はプロジェクト値を使用します。共有プロジェクトはユーザーより上にあるため、そのプロジェクトではヒントが表示され、他の場所では表示されません。

値を取り戻すことができます。そのプロジェクトの `.claude/settings.local.json` に `"spinnerTipsEnabled": false` を追加します。プロジェクトローカルは共有プロジェクトより上にあるため、そこでのセッションはヒントを表示しなくなり、チームメイトのセッションは変わりません。

<h4 id="organization-settings-override-everything">
  組織設定がすべてをオーバーライドします
</h4>

組織の管理設定がそれを `true` に設定しています。ユーザー、プロジェクト、またはローカル設定に何を入れてもヒントをオフにすることはできませんし、`--settings` でもできません。管理は最上位レベルです。

値を取り戻すことはできません。`/status` を実行して、どの管理ソースが適用されるかを確認し、ポリシーを変更する必要があるかどうかを管理者に尋ねてください。

<h4 id="the-command-line-overrides-your-files-for-one-session">
  コマンドラインが 1 つのセッションのファイルをオーバーライドします
</h4>

`claude --settings '{"spinnerTipsEnabled": true}'` でセッションを開始しました。コマンドラインは管理を除くすべてのファイルより上にあるため、そのセッションはファイルが `false` と言っていても、ヒントを表示します。

次のセッションで値を取り戻します。`--settings` は 1 つのセッション限りで、ファイルに書き込みません。

<h4 id="a-flag-or-environment-variable-sets-the-same-thing">
  フラグまたは環境変数が同じものを設定します
</h4>

一部のキーには、設定値をオーバーライドするコマンドラインフラグまたは環境変数があります。これはどのファイルが設定したかに関係なく機能します。`ANTHROPIC_MODEL` は [`model`](/docs/ja/settings-reference#model) 設定をオーバーライドし、`--model` はセッション用に両方をオーバーライドします。

値を取り戻せるかどうかはキーによって異なります。変数をアンセットするか、フラグをドロップし、[設定リファレンス](/docs/ja/settings-reference) のキーのエントリと [環境変数リファレンス](/docs/ja/env-vars) の変数の行を確認して、Claude Code がどちらを使用するかを確認してください。

<span id="keys-ignored-in-a-repository-file" />

<span id="keys-only-you-or-your-organization-can-set" />

<span id="common-cases" />

<span id="which-value-applies-in-common-situations" />

<h3 id="troubleshoot-a-setting-that-doesn’t-apply">
  適用されない設定をトラブルシューティングします
</h3>

キーを設定し、Claude Code がそのように動作しない場合は、`/status` で読み込まれたファイルを確認してから、以下で症状を見つけてください。[設定をデバッグする](/docs/ja/debug-your-config) はより広いチェックをカバーしており、クリーン設定テストを含みます。

<h4 id="a-value-you-set-is-ignored">
  設定した値が無視されます
</h4>

別のものが同じキーを設定しているか、ファイルがその値を設定できないか、またはファイルが読み込まれませんでした：

* **より高いレベルがそれを設定しています。** 別の設定ファイル、`--settings` フラグ、または管理ソースがキーをあなたのより上に設定しています。[スタック](#settings-precedence) はどちらを示しています。フラグまたは環境変数もキーごとに決定されて、キーをそれ自体でオーバーライドできます。[設定リファレンス](/docs/ja/settings-reference) のキーのエントリはどちらを Claude Code が使用するかを示し、[`env` エントリ](/docs/ja/settings-reference#env) は管理 `env` 値対シェルエクスポートをカバーしています。
* **セキュリティキーは厳密な値を保持しています。** いくつかのキーについて Claude Code は任意のファイルからの制限値を優先するため、プロジェクト `true` は [`disableClaudeAiConnectors`](/docs/ja/settings-reference#disableclaudeaiconnectors) のままです。[管理設定の優先順位の例外](#exceptions-to-managed-settings-precedence) を参照してください。
* **ファイルはその値を設定できません。** [`permissions.defaultMode`](/docs/ja/settings-reference#permissions-defaultmode) の値 `auto` および `bypassPermissions` はプロジェクトまたはローカル設定から有効になりません。代わりにユーザーまたは管理設定で設定するか、1 つのセッション用に `--permission-mode` を渡してください。v2.1.257 より前では、`bypassPermissions` はどのファイルからでも有効になりました。

  [`env`](/docs/ja/settings-reference#env) ブロック内のテレメトリエクスポート変数もプロジェクトまたはローカル設定からは有効になりません。いくつかのオフ値を除きます。[Claude Code が `env` で無視する変数](/docs/ja/settings-reference#variables-claude-code-ignores-in-env) は変数とそれらの値をリストしています。
* **ファイルが壊れています。** 無効な JSON または拒否された値により、Claude Code はファイルまたはエントリをスキップします。[壊れた設定ファイルを修正する](#fix-a-broken-settings-file) を参照してください。

<h4 id="a-change-you-made-in-claude-code-is-lost-in-new-sessions">
  Claude Code で行った変更が新しいセッションで失われます
</h4>

Claude Code 内から新しいセッション用に選択を保存する場合（例えば `/model` でデフォルトモデルを設定する場合）、Claude Code はそれをユーザー設定ファイル `~/.claude/settings.json` に書き込みます。そのファイルに書き込むことができない場合（例えば別のツールが生成するか、読み取り専用コピーにリンクしている場合）、変更は現在のセッションに適用され、次のセッションでは失われます。ファイルを生成するツールでキーを設定するか、ファイルを書き込み可能なものに置き換えてください。

ファイルに書き込むことができ、変更がまだ続かない場合は、変更が [1 つのセッション限り](#change-a-setting-for-one-session) であったか、[より高いレベルが同じキーを設定](#a-value-you-set-is-ignored) しているかを確認してください。`model` キーについては、[新しいセッションが選択したものとは異なるモデルで開始します](/docs/ja/model-config#a-new-session-starts-on-a-different-model-than-you-picked) がより多くの原因をリストしています。

<h4 id="a-managed-change-hasn’t-reached-you">
  管理変更があなたに到達していません
</h4>

管理ソースは [配信テーブル](/docs/ja/managed-settings#choose-a-delivery-mechanism) のスケジュールで実行中のセッションに到達するため、最初にセッションを再開してください。`/status` がその後、管理者が変更したものとは異なるソースを名前付けする場合、より優先度の高いソースが適用されます。[Claude Code が管理ソースを結合する方法](/docs/ja/managed-settings#how-claude-code-combines-managed-sources) は順序を示しています。

<h4 id="a-committed-key-doesn’t-reach-teammates">
  コミットされたキーがチームメイトに到達しません
</h4>

2 つのことが `.claude/settings.json` のキーがそれをクローンするすべての人に適用されるのを防ぎます：

* **Claude Code はリポジトリファイルのキーを無視します。** [設定インデックス](/docs/ja/settings-reference#settings-index) のスコープ列で `User, local, or managed`、`User or managed`、`Managed`、または `Global config` を探してください。これらのキーは共有ファイルから適用されません。ただし、リポジトリファイルがまだオフにできるいくつかを除きます。これらのエントリのそれぞれはスコープ行でそう言っています。`Global config` キーは `~/.claude.json` からのみ適用されます。

  `env` キー内では、テレメトリエクスポート変数も共有ファイルから適用されません。いくつかのオフ値を除きます。[Claude Code が `env` で無視する変数](/docs/ja/settings-reference#variables-claude-code-ignores-in-env) を参照してください。
* **キーは信頼を待っています。** `permissions.allow` ルール、`permissions.additionalDirectories`、`extraKnownMarketplaces`、およびほとんどの [`env`](/docs/ja/settings-reference#env) 値は、各チームメイトが [フォルダを信頼](/docs/ja/permissions#project-allow-rules-and-workspace-trust) した後にのみ適用されます。それまで、彼らはプロンプトを見続け、ファイルが宣言するマーケットプレイスからプラグインを取得しません。`deny` および `ask` ルールはすぐに適用されます。

<h4 id="permission-rules-combine-differently-than-you-expected">
  権限ルールが予想と異なる方法で結合されます
</h4>

* **権限プロンプトで「はい、今後は聞かないでください」を選択しましたが、同じツールのプロンプトが表示されます。** その選択はローカルファイルに `allow` ルールを保存し、そこの `allow` ルールはプロジェクトまたは管理ファイルからの `ask` ルールより優先されません。[権限ルールがどのように結合されるか](/docs/ja/permissions#settings-precedence) は順序を説明しています。VS Code 拡張機能では、承認カードはプロジェクトの共有ファイルを含む宛先ファイルを選択できます。これはすべての人のルールを変更します。CLI では、Claude Code はローカルファイルにのみ書き込みます。
* **組織の許可ルールはあなたのルールと一緒に適用されます。** これは予想されています。Claude Code は [`permissions.allow`](/docs/ja/settings-reference#permissions-allow) をスコープ全体でマージします。ただし、組織が [`allowManagedPermissionRulesOnly`](/docs/ja/settings-reference#allowmanagedpermissionrulesonly) を設定していない限り。

<span id="security-keys-where-the-stricter-value-applies" />

<h3 id="exceptions-to-managed-settings-precedence">
  管理設定の優先順位の例外
</h3>

セッションを制限する値を持つ少数のキーについて、Claude Code は通常は管理設定をオーバーライドできないスコープからの制限値を優先します。このテーブルでキーを見つけて、どの値を優先し、どこから優先するかを確認してください。

| キー                                                                              | Claude Code が優先する値                                                                                      | 注記                                                                                                                      |
| :------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------- |
| [`disableClaudeAiConnectors`](/docs/ja/settings-reference#disableclaudeaiconnectors) | 任意のスコープからの `true`                                                                                       | 管理ソースが `false` を設定する場合でも優先されます                                                                                          |
| [`enableArtifact`](/docs/ja/settings-reference#enableartifact)                       | 任意のスコープからの `false`、および任意のスコープからの `disableArtifact: true`                                                | 管理ソースが `true` を設定する場合でも優先されます。何も [Artifact ツール](/docs/ja/artifacts#disable-artifacts) をオンに戻しません。Claude Code v2.1.242 以降が必要です |
| [`isolatePeerMachines`](/docs/ja/settings-reference#isolatepeermachines)             | 任意のスコープからの `true`                                                                                       | 管理ソースが `false` を設定する場合でも優先されます                                                                                          |
| [`remoteControlAtStartup`](/docs/ja/settings-reference#remotecontrolatstartup)       | `.claude/settings.json` または `.claude/settings.local.json` からの `false`                                   | 管理ソースが `true` を設定する場合でも優先されます。プロジェクトまたはローカル `true` は無視されます                                                              |
| [`crossSessionInbound`](/docs/ja/settings-reference#crosssessioninbound)             | `.claude/settings.json` または `.claude/settings.local.json` からのより厳密な値、`accept` \< `hold` \< `refuse` ラダーで | 管理、`--settings`、およびユーザー値より優先されます。より厳密でないプロジェクトまたはローカル値は無視されます                                                           |
| [`useAutoModeDuringPlan`](/docs/ja/settings-reference#useautomodeduringplan)         | 任意の管理ソース、`--settings`、`~/.claude/settings.json`、または `.claude/settings.local.json` からの `false`           | 勝利した管理ソースが `true` を設定する場合でも優先されます。`.claude/settings.json` の `false` は無視されます                                             |
| [`syncClaudeAiSkills`](/docs/ja/settings-reference#syncclaudeaiskills)               | 任意の管理ソース、`--settings`、`~/.claude/settings.json`、または `.claude/settings.local.json` からの `false`           | 勝利した管理ソースが `true` を設定する場合でも優先されます。`.claude/settings.json` の `false` は無視されます                                             |
| [`syncClaudeAiPlugins`](/docs/ja/settings-reference#syncclaudeaiplugins)             | 任意の管理ソース、`--settings`、`~/.claude/settings.json`、または `.claude/settings.local.json` からの `false`           | 勝利した管理ソースが `true` を設定する場合でも優先されます。`.claude/settings.json` の `false` は無視されます                                             |
| [`maxEffortLevel`](/docs/ja/settings-reference#maxeffortlevel)                       | `--settings` を含む任意のスコープからのより低いキャップ                                                                      | Claude Code が適用する管理設定がより高いキャップを設定する場合でも優先されます。最も低いキャップが適用されます。Claude Code v2.1.267 以降が必要です                              |

Claude Code を内部で実行し、[`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/ja/env-vars) を設定するアプリも例外です。Claude Code はそのアプリのモデル設定をすべての管理ソースからの `model`、`fallbackModel`、`modelPicker`、および `modelOverrides` キーより優先し、管理 `env` ブロック内のモデル選択変数（`ANTHROPIC_MODEL` および `ANTHROPIC_DEFAULT_*_MODEL` ファミリーなど）より優先します。Claude Code は管理 [`availableModels`](/docs/ja/settings-reference#availablemodels) 許可リストを有効に保ちます。ただし、アプリが独自のものを提供する場合を除きます。

<h2 id="settings-in-cloud-sessions">
  クラウドセッションの設定
</h2>

[クラウドセッション](/docs/ja/claude-code-on-the-web)は[クラウド環境](/docs/ja/cloud-environments)で実行され、マシン上ではなくリポジトリの新しいクローンで実行されます。これにより、どの設定がそこに到達するかが変わります：

* **共有プロジェクト設定**（`.claude/settings.json`）：1 つのリポジトリを持つセッションで読み込まれます。ファイルはクローンの一部であり、セッションはその内部で開始されるためです。その設定をコミットして、それらのセッションに適用してください。複数のリポジトリを持つセッションはクローンの上で開始され、各リポジトリの `.claude/settings.json` から `enabledPlugins` と `extraKnownMarketplaces` キーのみを読み込み、権限ルール、hooks、`env`、またはその他のキーは読み込みません。これら 2 つのキーが宣言するマーケットプレイスとプラグインは、それでも[クラウドセッションでは読み込まれません](/docs/ja/cloud-environments#what-carries-over-from-your-setup)。
* **ユーザーおよびプロジェクトローカル設定**（`~/.claude/settings.json` および `.claude/settings.local.json`）：読み込まれません。両方ともマシンに留まり、ローカルファイルはクローンにありません。
* **管理設定**：[サーバー管理設定](/docs/ja/server-managed-settings)のみがクラウドセッションに到達します。デバイスの `managed-settings.json` ファイルまたは MDM プロファイルは到達しません。[自己ホスト型環境](/docs/ja/self-hosted-environments)もランナーイメージの管理設定ファイルを読み込みます。[Claude Code が管理ソースを結合する方法](/docs/ja/managed-settings#how-claude-code-combines-managed-sources)はそのファイルがいつ適用されるかを説明します。
* **`/config`**：ブラウザの claude.ai/code では、値を変更する代わりに Claude Code セクションの claude.ai 設定を開きます。クラウドセッションの設定を変更するには、環境で[環境変数](/docs/ja/cloud-environments#set-environment-variables)を設定するか、1 つのリポジトリを持つセッションでは、そのリポジトリの `.claude/settings.json` にキーをコミットしてください。

[セットアップから何が引き継がれるか](/docs/ja/cloud-environments#what-carries-over-from-your-setup)は残りをリストします：`CLAUDE.md`、skills、MCP サーバー、plugins、および認証情報。

<h2 id="what’s-next">
  次は何か
</h2>

* [すべての設定](/docs/ja/settings-reference)：すべてのキー。設定する場所と例
* [設定ファイルの例](/docs/ja/settings-example)：個人ファイル、チームファイル、および組織の管理ファイル
* [権限を構成する](/docs/ja/permissions)：allow、ask、および deny ルール。Claude Code が確認なしで実行するもの
* [環境変数](/docs/ja/env-vars)：Claude Code が読み込む変数および `env` ブロック
* [構成をデバッグする](/docs/ja/debug-your-config)：設定が適用されない場合
* [Claude ディレクトリリファレンス](/docs/ja/claude-directory)：Claude Code が読み込むすべてのファイル。subagents、MCP サーバー、プラグイン、および `CLAUDE.md` を含む
