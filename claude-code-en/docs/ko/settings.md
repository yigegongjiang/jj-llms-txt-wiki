> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 설정 파일 및 우선순위

> Claude Code 설정을 변경하고, 키가 속할 범위를 선택하고, 변경을 확인하고, 키가 여러 위치에 설정되어 있을 때 Claude Code가 사용하는 값을 알아봅니다.

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

설정은 Claude Code의 동작을 변경하는 JSON 키입니다: 시작할 모델, 묻지 않고 실행할 수 있는 것, 읽을 수 없는 파일, 터미널에서의 모양, 조직이 적용하는 것입니다.

<Tip>
  특정 키를 찾으려면 [모든 설정](/docs/ko/settings-reference)으로 이동하세요. 여기에는 설정한 파일, 기본값, 예제가 있는 모든 키가 나열됩니다.
</Tip>

Claude Code는 `~/.claude/settings.json`과 같은 JSON 설정 파일에서 설정을 읽습니다. 여러 위치에서 찾으며, [읽는 파일이 설정이 누구에게 적용되는지 결정합니다](#settings-files-and-who-they-affect). 이 페이지에서는 이러한 파일을 다룹니다: 설정을 어느 파일에 넣을지, 설정을 변경하고 적용되었는지 확인하는 방법, 동일한 키가 여러 파일에 설정되어 있을 때 Claude Code가 사용하는 값입니다. [권한 구성](/docs/ko/permissions)에서는 Claude Code가 묻지 않고 실행할 수 있는 것과 `allow`, `ask`, `deny` 규칙을 작성하는 방법을 다룹니다.

<Note>
  이 페이지는 머신에서 실행되는 Claude Code를 다룹니다: 터미널, [VS Code](/docs/ko/vs-code) 및 [JetBrains](/docs/ko/jetbrains) 확장, [데스크톱 앱](/docs/ko/desktop)은 모두 동일한 설정 파일을 읽습니다. [Claude Code on the web](/docs/ko/claude-code-on-the-web)의 클라우드 세션은 다른 머신에서 실행되며 일부만 읽습니다. [클라우드 세션의 설정](#settings-in-cloud-sessions)을 참조하세요.
</Note>

<span id="settings-files" />

<span id="configuration-scopes" />

<span id="available-scopes" />

<span id="when-to-use-each-scope" />

<span id="what-uses-scopes" />

<span id="subagent-configuration" />

<span id="where-settings-live" />

<h2 id="settings-files-and-who-they-affect">
  설정 파일 및 영향을 받는 사용자
</h2>

Claude Code는 네 개의 파일에서 설정을 읽으며, 조직은 claude.ai 콘솔에서 관리되는 설정을 제공할 수도 있습니다. 각 소스는 범위를 가지고 있습니다. 즉, 설정이 적용되는 사람과 프로젝트의 집합으로, 개인 사용자, 프로젝트의 모든 사람, 또는 조직의 모든 사람일 수 있습니다.

| 범위      | 파일                                                                               | 영향을 받는 사용자                                                                                            | 용도                                     |
| :------ | :------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------- | :------------------------------------- |
| 사용자     | `~/.claude/settings.json`                                                        | 이 머신의 모든 프로젝트에서 사용자                                                                                   | 개인 설정: 테마, 편집기 모드, 기본 모델, 사용자 정의 권한 규칙 |
| 공유 프로젝트 | `.claude/settings.json`                                                          | 이를 포함하는 폴더에서 작업하는 모든 사람. Git 저장소에서는 커밋하여 팀원이 받도록 함                                                    | 팀 권한, 훅, 플러그인, 프로젝트에 필요한 환경 변수         |
| 프로젝트 로컬 | `.claude/settings.local.json`                                                    | 이 프로젝트에서만 사용자. Claude Code는 파일을 생성할 때 git에서 제외함. 수동으로 생성한 경우 `.gitignore`에 직접 추가                      | 한 프로젝트에 대한 개인 설정 재정의 및 공유 전 테스트        |
| 관리됨     | `managed-settings.json` 및 기타 [관리되는 소스](/docs/ko/managed-settings#delivery-mechanisms) | 조직이 배포하는 모든 사용자. 몇 가지 [보안 관련 예외](#exceptions-to-managed-settings-precedence)를 제외하고 설정한 것이 이를 재정의하지 않음 | 보안 정책 및 규정 준수 요구사항                     |

파일 열에서 `~/.claude`는 홈 디렉토리의 `.claude` 폴더이고, 단순 `.claude`는 프로젝트 내부의 `.claude` 폴더입니다.

<span id="where-each-file-applies" />

<span id="compare-what-each-file-reaches" />

<h3 id="compare-the-scope-of-each-settings-file">
  각 설정 파일의 범위 비교
</h3>

머신에 `website/`, `api/`, `acme-app/` 세 개의 프로젝트가 있고, 팀원이 `acme-app/`의 자신의 클론을 가지고 있으며, `acme-app/`에서 [클라우드 세션](#settings-in-cloud-sessions)을 시작한다고 가정합니다.

아래 그래픽은 이러한 폴더에서 Claude Code를 시작할 때 설정이 적용되는 폴더를 보여줍니다. 설정 파일을 클릭하여 도달하는 폴더를 확인합니다.

<SettingsScope />

* **`~/.claude/settings.json`**: 머신의 모든 프로젝트, 팀원의 것이나 클라우드 세션의 것은 제외
* **`acme-app/.claude/settings.json`**: 사용자의 `acme-app/`. 파일을 버전 제어에 커밋한 경우에만 팀원의 클론과 클라우드 세션에 도달합니다. 커밋하기 전까지는 다른 파일처럼 디스크의 파일이며 다른 사람은 이를 가지지 않습니다.
* **`acme-app/.claude/settings.local.json`**: 사용자의 `acme-app/`만. Claude Code는 파일을 처음 작성할 때 전역 git 제외에 추가하므로 커밋에서 제외됩니다. 수동으로 파일을 생성한 경우 [`.gitignore`에 직접 추가](#keep-personal-settings-out-of-a-repository)합니다.
* **관리되는 설정**, `managed-settings.json` 파일, MDM 정책, 또는 claude.ai 콘솔의 [서버 관리 설정](/docs/ko/server-managed-settings): 조직이 배포하는 모든 머신의 모든 프로젝트, 또는 조직 계정으로 로그인한 모든 머신. 서버 관리 설정만 클라우드 세션에 도달합니다.

<span id="which-files-you-have" />

<h3 id="find-or-create-your-settings-files">
  설정 파일 찾기 또는 생성
</h3>

Claude Code를 설치해도 설정 파일이 생성되지 않습니다. 머신이나 프로젝트에 이미 파일이 있다면 다음 중 하나에서 온 것입니다:

* **관리됨**: 조직이 배포합니다. 사용자가 생성하거나 편집하지 않습니다.
* **공유 프로젝트**: Claude Code를 이미 사용하는 프로젝트에 커밋된 파일이 있을 수 있습니다. 없으면 프로젝트 폴더에 `.claude/settings.json`을 생성합니다.
* **사용자** 및 **프로젝트 로컬**: 직접 생성하거나 Claude Code가 생성하도록 합니다. 테마와 같이 사용자 설정에 저장되는 `/config` 메뉴의 옵션을 처음 변경할 때 `~/.claude/settings.json`을 작성하고, Bash 명령에 대해 "Yes, and don't ask again"과 같은 권한 프롬프트에서 처음 승인을 할 때 `.claude/settings.local.json`을 작성합니다. **Show tips**를 포함한 몇 가지 `/config` 옵션은 사용자 파일 대신 `.claude/settings.local.json`에 저장됩니다.

<Info>
  Windows에서 `~/.claude`는 `%USERPROFILE%\.claude`를 의미합니다. 홈 디렉토리 파일을 다른 곳에 보관하려면 [`CLAUDE_CONFIG_DIR`](/docs/ko/env-vars)을 설정합니다. Claude Code는 설정, 세션 기록, 플러그인을 대신 그곳에 저장합니다.
</Info>

Claude Code는 또한 다섯 번째 파일인 [`~/.claude.json`](/docs/ko/claude-directory#ce-claude-json)을 유지합니다. 이는 Claude Code가 자신을 위해 작성하는 파일이므로 편집할 필요가 없습니다. 로그인 세션, [MCP 서버](/docs/ko/mcp) 구성, 신뢰 결정과 같은 프로젝트별 상태, `/config`가 사용자를 위해 작성하는 [전역 구성 키](/docs/ko/settings-reference#global-config-settings)를 보유합니다.

<h3 id="share-settings-with-your-team">
  팀과 설정 공유
</h3>

`.claude/settings.json`을 커밋하여 저장소를 클론하는 모든 사람이 동일한 권한, 훅, 플러그인을 받도록 합니다. 각 팀원은 여전히 자신의 `.claude/settings.local.json`에서 이를 재정의할 수 있으므로 개인 예외는 커밋할 필요가 없습니다. 완전한 팀 파일은 [팀의 공유 설정](/docs/ko/settings-example#a-teams-shared-settings)을 참조합니다.

커밋하는 일부 항목은 각 팀원이 [폴더를 신뢰](/docs/ko/permissions#project-allow-rules-and-workspace-trust)할 때까지 기다리며, 몇 가지 키는 저장소 파일에서 적용되지 않습니다. [적용되지 않는 설정 문제 해결](#common-cases)은 둘 다 다룹니다.

<span id="local-settings-file" />

<span id="where-claude-code-saves-the-project-local-file" />

<span id="the-project-local-file" />

<span id="keep-personal-settings-out-of-the-repository" />

<h3 id="keep-personal-settings-out-of-a-repository">
  저장소에서 개인 설정 제외
</h3>

한 프로젝트에서 팀원을 변경하지 않고 자신을 위해 설정을 변경하려면 프로젝트 내부의 `.claude/settings.local.json`에 저장합니다. Claude Code는 커밋된 `.claude/settings.json`에 해당 파일을 적용하므로, 팀의 파일이 `"model": "claude-sonnet-5"`를 설정하고 Opus를 원하면 로컬 파일에 `"model": "claude-opus-5-5"`를 입력하고 세션만 변경됩니다.

Claude Code는 또한 이 파일에 작성하고, 커밋에서 제외하며, 신뢰 단계 없이 허용 규칙을 적용합니다:

* **Claude Code도 작성합니다.** Claude가 Bash 명령을 실행할 권한을 요청하고 "Yes, and don't ask again"을 선택하면, Claude Code는 해당 [권한 승인](/docs/ko/permissions#permission-system)을 `allow` 규칙으로 여기에 저장합니다.
* **수동으로 생성한 경우를 제외하고 직접 gitignore할 필요가 없습니다.** Claude Code가 이미 무시하지 않는 git 저장소에서 파일을 처음 작성할 때, 전역 git 제외 파일에 `**/.claude/settings.local.json`을 추가하므로 모든 저장소에서 파일이 커밋에서 제외됩니다. 해당 파일은 전역 git 구성이 절대 경로 또는 `~` 접두사 경로로 설정할 때 `core.excludesFile`입니다. 그렇지 않으면 `$XDG_CONFIG_HOME/git/ignore` 또는 `XDG_CONFIG_HOME`이 설정되지 않을 때 `~/.config/git/ignore`입니다. 수동으로 파일을 생성했고 Claude Code가 아직 작성하지 않았다면 `.gitignore`에 직접 추가합니다.
* **파일이 추적되지 않은 상태에서 허용 규칙은 신뢰를 기다리지 않습니다.** 파일이 저장소의 것이 아니라 사용자의 것이므로, Claude Code는 커밋된 파일에 필요한 [워크스페이스 신뢰](/docs/ko/permissions#project-allow-rules-and-workspace-trust) 단계 없이 `allow` 규칙을 적용합니다. 파일이 git으로 추적되면 신뢰 단계도 적용됩니다. [로컬 설정 파일이 신뢰가 필요한 경우](/docs/ko/permissions#when-your-local-settings-file-needs-trust)를 참조합니다.

<span id="where-claude-code-looks-for-each-file" />

<span id="how-claude-code-keeps-the-local-file-out-of-git" />

<span id="local-allow-rules-dont-wait-for-workspace-trust" />

<h4 id="where-claude-code-keeps-the-local-file-in-a-git-repository">
  Claude Code가 git 저장소에서 로컬 파일을 보관하는 위치
</h4>

Claude가 Bash 명령을 실행할 권한을 요청하고 "Yes, and don't ask again"을 선택하면, Claude Code는 해당 승인을 `.claude/settings.local.json`의 `allow` 규칙으로 저장합니다. git 저장소의 하위 디렉토리에서 Claude Code를 시작하면 저장소 루트에서 해당 파일을 읽고 작성하며 전체 저장소에 승인을 적용합니다. [worktree](/docs/ko/worktrees)에서는 주 체크아웃의 루트에 있는 파일을 사용합니다.

두 가지 규칙이 루트 위치를 한정합니다:

* **파일이 `.claude/settings.json` 대신 유지되는 경우**: git 저장소 외부, 저장소 루트가 홈 디렉토리인 경우, Windows에서, 또는 저장소 루트나 `.git` 또는 `.claude` 항목이 사용자가 소유하지 않은 경우.
* **파일의 경로는 저장소 루트에 고정되지 않습니다**: `/`로 시작하거나 상대 샌드박스 경로인 권한 규칙은 [세션의 기본 작업 디렉토리](/docs/ko/permissions#read-and-edit)에 고정됩니다.

v2.1.211 이전에는 Claude Code가 시작 디렉토리에 파일을 보관했습니다. 이전 버전이 루트 파일 옆에 남긴 파일을 여전히 읽습니다. 두 파일이 동일한 키를 설정하는 경우 루트의 값이 적용되고, 두 파일의 권한 규칙이 적용됩니다. Agent SDK의 [`resolveSettings()`](/docs/ko/agent-sdk/typescript#resolvesettings) 헬퍼는 항상 시작 디렉토리에서 파일을 읽습니다.

Claude Code는 공유 `.claude/settings.json`을 세션의 [기본 작업 디렉토리](/docs/ko/permissions#working-directories)에서 읽으므로, 저장소 루트에 커밋된 파일을 사용하려면 거기서 Claude Code를 시작합니다. [`/cd`로 세션을 이동](/docs/ko/permissions#move-the-session-to-another-directory)한 후, Claude Code는 대신 새 디렉토리에서 두 프로젝트 파일을 읽으며, 로컬 파일을 동일한 규칙으로 배치합니다. 이동한 디렉토리에서 읽으려면 Claude Code v2.1.246 이상이 필요합니다.

<span id="managed-settings-delivery" />

<span id="precedence-within-the-managed-tier" />

<span id="parent-settings-from-embedding-hosts" />

<span id="enforce-settings-for-an-organization" />

<span id="settings-your-organization-manages" />

<h3 id="check-what-your-organization-enforces">
  조직이 적용하는 항목 확인
</h3>

조직이 Claude Code를 관리하는 경우, 일부 설정은 사용자를 위해 결정되며 자신의 파일에 입력한 것이 이를 변경하지 않습니다. 어떤 것인지 확인하려면 `/status`를 실행합니다. `Setting sources` 줄은 사용자에게 적용되는 관리되는 소스의 이름을 지정합니다. 관리되는 설정은 이 머신에서 Claude Code가 실행되는 모든 곳에 적용됩니다. [개발자가 변경할 수 있는 항목](/docs/ko/managed-settings#what-a-developer-can-change)은 로컬 관리자 권한 및 Claude Code 이외의 도구를 다룹니다.

관리되는 설정은 관리되는 설정 페이지의 [전달 메커니즘](/docs/ko/managed-settings#delivery-mechanisms)을 통해 사용자에게 도달합니다. 가장 일반적으로:

* [서버 관리 설정](/docs/ko/server-managed-settings), Claude Code가 claude.ai 관리 콘솔 또는 자체 호스팅 [Claude 앱 게이트웨이](/docs/ko/claude-apps-gateway)에서 가져옴
* MDM 또는 OS 수준 정책, 시스템 디렉토리의 `managed-settings.json` 파일
* Claude Desktop과 같은 임베딩 호스트, SDK `managedSettings` 옵션을 통해. [임베딩 호스트에서 정책 제어](/docs/ko/managed-settings#parent-settings-from-embedding-hosts)를 참조합니다.

Claude Desktop 앱에서 머신에서 실행되는 [Cowork](https://claude.com/docs/cowork/overview) 세션에서, Claude Code는 claude.ai 관리 콘솔에서 서버 관리 설정을 가져오지 않으며, 조직의 Claude Desktop 구성이 `requireCoworkFullVmSandbox`를 설정하지 않는 한 디바이스에 배포된 정책을 읽습니다. [정책이 적용되는 위치 및 시기](/docs/ko/managed-settings#where-and-when-a-policy-applies)는 Cowork 및 클라우드 세션을 다룹니다.

관리자인 경우, [조직을 위해 Claude Code 설정](/docs/ko/admin-setup)은 적용할 항목 선택을 안내하고, [관리되는 설정 배포](/docs/ko/managed-settings)는 전달 및 정책이 적용 중인지 확인하는 방법을 다룹니다.

<h2 id="change-a-setting">
  설정 변경
</h2>

`/config` 메뉴, 설정 파일 편집, 또는 한 세션에 대해 명령줄에서 설정을 변경할 수 있습니다.

<span id="system-prompt" />

Claude Code의 시스템 프롬프트는 게시되지 않습니다. Claude에 지속적인 지침을 제공하려면 [`CLAUDE.md` 파일](/docs/ko/memory) 또는 `--append-system-prompt` 플래그를 사용합니다.

<h3 id="use-the-/config-menu">
  /config 메뉴 사용
</h3>

Claude Code 내에서 `/config`를 실행하고 **Config** 탭을 엽니다. 테마, 편집기 모드, 상세 출력과 같은 짧은 개인 옵션 집합을 나열하며, 모든 설정 키는 아닙니다. 옵션을 선택하여 변경합니다. Claude Code가 저장합니다:

* **대부분의 옵션**: `~/.claude/settings.json`
* **팁 표시와 같은 몇 가지 옵션**: `.claude/settings.local.json`
* **[전역 구성 옵션](/docs/ko/settings-reference#global-config-settings)**: `~/.claude.json`

메뉴 없이 한 옵션을 설정하려면 `/config verbose=true`와 같이 `key=value`를 전달합니다.

<Note>
  `/config`는 터미널 인터페이스의 일부입니다. [VS Code](/docs/ko/vs-code) 채팅 패널 및 [데스크톱 앱](/docs/ko/desktop)은 열지 않습니다. 설정 파일을 편집하거나 해당 앱의 자체 설정을 통해 변경합니다.
</Note>

<h3 id="edit-a-settings-file">
  설정 파일 편집
</h3>

원하는 범위의 설정 파일을 편집기에서 열고 키를 추가하거나 변경합니다. 설정 파일은 엄격한 JSON입니다: `//` 주석이나 후행 쉼표는 구문 오류이며, Claude Code는 다음 시작에서 파일을 [설정 오류](#fix-a-broken-settings-file)로 보고합니다. 예를 들어 Claude Code가 lint 및 테스트 명령을 묻지 않고 실행하고 `.env` 파일 읽기를 중지하도록 하려면 `~/.claude/settings.json`에 다음을 추가합니다:

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

`permissions` 아래의 각 항목은 도구와 수행할 수 있는 것을 명명하는 규칙입니다. [권한 구성](/docs/ko/permissions)은 구문을 설명합니다. `$schema` 줄은 Claude Code 설정에 대한 [게시된 JSON 스키마](https://json.schemastore.org/claude-code-settings.json)를 가리키며, VS Code, Cursor 및 JSON 스키마를 지원하는 다른 편집기에서 자동 완성 및 인라인 검증을 제공합니다. 스키마는 최신 CLI 릴리스보다 뒤떨어질 수 있으므로 최근에 문서화된 키에 대한 검증 경고는 구성이 유효하지 않음을 의미하지 않습니다.

저장한 후 Claude Code 내에서 `/status`를 실행하여 파일이 로드되었는지 확인합니다. [로드된 것 확인](#check-what-loaded)은 `Setting sources` 줄이 표시하는 것과 손상된 파일이 보고되는 방식을 설명합니다.

완전한 개인 파일, 팀 파일, 조직 파일은 각각 설정하는 모든 키에 대한 주석과 함께 [예제 설정 파일](/docs/ko/settings-example)을 참조하세요.

<span id="pass-settings-for-one-session" />

<h3 id="change-a-setting-for-one-session">
  한 세션에 대해 설정 변경
</h3>

값을 저장하지 않고 시도하려면 Claude Code를 시작할 때 설정합니다. 값은 해당 세션에 적용되고 설정 파일은 그대로 유지됩니다. 3가지 방법이 있습니다:

* **`--settings`**: JSON으로 키를 전달하거나 파일 경로로. Claude Code는 사용자, 프로젝트, 로컬 파일 위에 적용하고 관리되는 설정 아래에 적용합니다. 사용자 설정 파일이 설정할 수 있는 모든 키를 설정할 수 있습니다. `Managed` 또는 `Global config` 키는 설정할 수 없습니다.
* **해당 키에 대한 플래그**: 일부 키에는 `model`에 대한 `--model`, `effortLevel` 및 `modelSettings`에 대한 `--effort`와 같은 자체 플래그가 있습니다.
* **환경 변수**: `model`에 대해 `ANTHROPIC_MODEL`과 같이 `claude`를 실행하기 전에 키의 쌍을 이루는 변수를 내보냅니다.

각 키의 [설정 참조](/docs/ko/settings-reference) 항목은 세션별 재정의 및 우선순위를 나열하므로 변경하려는 키의 항목을 확인합니다.

세션 내에서 실행하는 명령은 대부분 선택을 저장합니다: `/config`에서 설정을 변경하면 Claude Code가 설정 파일에 작성하고, `/model`은 새 세션의 기본값으로 값을 저장합니다.

`/model` 선택기에서 `s`를 누르면 Claude Code는 기본값으로 저장하지 않고 모델을 전환합니다. [노력 수준 조정](/docs/ko/model-config#adjust-effort-level)은 `/effort`가 사용 중인 모델의 기본값으로 저장하는 것과 현재 세션에만 적용하는 것을 설명합니다.

예를 들어 기본값을 변경하지 않고 한 세션을 Opus에서 시작하려면:

```bash theme={null}
claude --settings '{"model": "claude-opus-5-5"}'
```

<h3 id="when-edits-take-effect">
  편집이 적용되는 시기
</h3>

Claude Code는 설정 파일을 감시하고 변경될 때 다시 로드하므로 대부분의 편집이 재시작 없이 실행 중인 세션에 적용됩니다. 여기에는 `permissions`, `hooks`, `apiKeyHelper`와 같은 자격 증명 도우미에 대한 편집이 포함됩니다. Claude Code는 또한 세션 중에 생성한 설정 파일을 로드합니다. 해당 폴더가 세션 시작 시 존재했다면 말입니다. 프로젝트의 `.claude/` 폴더의 경우 같은 세션에서 폴더를 생성할 때도 파일을 로드합니다.

다시 로드는 사용자, 프로젝트, 로컬, 관리되는 설정을 다룹니다. Claude Code는 감지된 각 설정 파일 변경에 대해 [`ConfigChange` hook](/docs/ko/hooks#configchange)을 실행하며, MDM 또는 claude.ai 콘솔에서 도착하는 관리되는 설정에 대해서는 실행하지 않습니다. MDM 또는 claude.ai 콘솔에서 도착하는 관리되는 설정은 일정에 따라 실행 중인 세션에 도달합니다. [전달 테이블](/docs/ko/managed-settings#choose-a-delivery-mechanism)은 소스별로 제공합니다.

Claude Code는 일부 키를 세션 시작 시에만 한 번 읽으므로 편집이 실행 중인 세션에 도달하지 않습니다. 관리자 측 키도 `requiredMinimumVersion`과 같이 재시작을 기다립니다. [정책이 적용되는 위치와 시기](/docs/ko/managed-settings#where-and-when-a-policy-applies)에 나열됩니다. 세션 중에 편집할 가능성이 가장 높은 것:

* [`model`](/docs/ko/settings-reference#model): 세션 중에 전환하려면 [`/model`](/docs/ko/model-config#setting-your-model)을 사용합니다. 각 모델에는 자체 프롬프트 캐시가 있으므로 전환 후 첫 요청은 전체 대화를 캐시되지 않은 상태로 다시 읽습니다. [모델 전환](/docs/ko/prompt-caching#switching-models)을 참조하세요
* [`effortLevel`](/docs/ko/settings-reference#effortlevel) 및 [`modelSettings`](/docs/ko/settings-reference#modelsettings): 세션 중에 변경하려면 [`/effort`](/docs/ko/model-config#adjust-effort-level)를 사용합니다

<span id="verify-active-settings" />

<span id="check-what-loaded" />

<h3 id="confirm-what-loaded">
  로드된 것 확인
</h3>

Claude Code 내에서 `/status`를 실행하여 활성 설정 소스를 확인합니다. **Status** 탭에는 Claude Code가 현재 세션에 로드한 각 설정 파일을 나열하는 `Setting sources` 줄이 포함됩니다. 예: `User settings` 또는 `Project local settings`. [관리되는 설정](/docs/ko/admin-setup#decide-how-settings-reach-devices)이 적용되면 관리되는 설정 항목은 괄호에 도달 방식을 표시합니다.

줄은 Claude Code가 읽은 파일을 확인합니다. 각 키를 제공한 파일은 표시하지 않습니다. Claude Code가 거부한 항목을 나열하려면 [`claude doctor`](/docs/ko/debug-your-config)를 실행합니다. 프로젝트 또는 관리되는 설정이 설정한 모델의 경우 시작 헤더가 설정한 파일의 이름을 지정합니다. `/status`와 `/config`는 다른 탭에서 동일한 대화를 엽니다. **Config** 탭은 `settings.json` 내용의 보기가 아닙니다.

<h3 id="fix-a-broken-settings-file">
  손상된 설정 파일 수정
</h3>

JSON을 잘못 입력하거나 Claude Code가 수락하지 않는 값으로 키를 설정하면 Claude Code는 대화형 세션 시작 시 알려줍니다. 표시되는 것은 파일의 얼마나 많은 부분이 영향을 받는지에 따라 다릅니다:

* **설정 오류**: 사용자, 프로젝트 또는 로컬 파일에 유효하지 않은 JSON 또는 스키마가 거부하는 값이 있습니다. 대화형 세션 시작 시 Claude Code는 Claude의 도움으로 파일을 수정하거나, 종료하거나, 손상된 설정 없이 계속할 수 있는 대화를 표시합니다.
* **설정 경고**: 개별 항목만 실패합니다. 예: 잘못된 형식의 권한 규칙 또는 알 수 없는 hook 이벤트 이름. Claude Code는 해당 값을 건너뛰고 파일의 나머지를 유지합니다.
* **관리되는 설정**: Claude Code는 파일의 나머지를 적용합니다. [관리되는 설정의 유효하지 않은 항목](/docs/ko/managed-settings#invalid-entries-in-managed-settings)은 삭제되는 것과 유효하지 않은 항목을 수정할 때까지 폴백하는 키를 설명합니다. 유효한 JSON이 아닌 관리되는 설정 문서는 [관리되는 설정 문서를 파싱할 수 없음](/docs/ko/errors#managed-settings-document-could-not-be-parsed)을 참조하세요.
* **구성 오류**: `~/.claude.json`을 파싱할 수 없습니다. Claude Code는 손상된 파일을 `~/.claude/backups/.claude.json.corrupted.<timestamp>`에 복사하고 종료하고 직접 수정하거나 기본 구성으로 재설정할지 묻습니다. `-p` 실행은 오류를 인쇄하고 종료합니다. 이전 상태를 복구하려면 `~/.claude/backups/`의 5개 가장 최근 `.claude.json.backup.<timestamp>` 파일 중 하나를 다시 복사합니다. Claude Code는 파일을 쓰기 전에 저장합니다.

계속한 후 `/status`를 실행하여 영향을 받는 파일을 확인하고 각 오류의 세부 사항을 보려면 `claude doctor`를 실행합니다.

`-p` 실행은 대화를 표시하지 않습니다. [관리되는 설정 문서를 파싱할 수 없음](/docs/ko/errors#managed-settings-document-could-not-be-parsed)이 아니면 Claude Code는 손상된 파일이나 값을 건너뛰고 나머지로 계속합니다. 따라서 설정을 무시한 `-p` 실행 후 `claude doctor`를 실행하여 삭제된 것을 확인합니다.

<span id="how-scopes-interact" />

<span id="key-points-about-the-configuration-system" />

<span id="which-value-claude-code-uses" />

<span id="which-value-wins" />

<h2 id="settings-precedence">
  설정 우선순위
</h2>

동일한 키가 여러 위치에 나타나면 Claude Code는 가장 높은 수준에서 설정한 값을 사용합니다. 아래 스택은 수준을 보여주며, 맨 위가 가장 높습니다. 더 높은 수준의 키는 아래의 동일한 키를 재정의합니다.

<SettingsPrecedence />

순서대로, 가장 높은 우선순위부터:

1. **관리되는 설정**: 조직이 배포하는 설정으로, `managed-settings.json` 파일, MDM 정책, 또는 claude.ai 콘솔의 [서버 관리 설정](/docs/ko/server-managed-settings)입니다. 사용자가 설정한 것은 이를 재정의할 수 없습니다. `--settings`로 전달한 키는 동일한 관리되는 키를 재정의하지 않으며, `--model`과 같은 플래그는 조직이 허용하는 모델에서만 선택합니다. 관리되는 `model`은 각 세션이 시작되는 모델을 설정하며, `/model`로 전환할 수 있습니다. 잠금은 [`availableModels`](/docs/ko/settings-reference#availablemodels)이며, 이는 `/model`, `--model`, 및 자신의 파일의 `model` 키를 제한합니다. 조직이 둘 이상의 관리되는 소스를 제공할 때, [관리되는 계층 내 우선순위](/docs/ko/managed-settings#precedence-within-the-managed-tier)의 규칙은 Claude Code가 각각에서 읽는 것을 나타냅니다.
2. **명령줄 인수**: 터미널에서 `claude`를 시작할 때 전달하는 플래그로, 한 세션 동안 적용됩니다. [한 세션 동안 설정 변경](#change-a-setting-for-one-session)을 참조하세요. Claude Code는 `--settings <file-or-json>`으로 전달한 JSON을 다른 수준과 동일한 규칙으로 설정 파일과 병합합니다. 여기서 설정한 키를 로컬, 프로젝트 또는 사용자 설정의 동일한 키보다 우선하며, 생략한 키에 대해서는 낮은 수준의 값을 유지합니다.
3. **프로젝트 로컬 설정** (`.claude/settings.local.json`): 이 프로젝트에 대한 개인 설정입니다.
4. **공유 프로젝트 설정** (`.claude/settings.json`): 팀이 소스 제어에 체크인하는 설정입니다.
5. **사용자 설정** (`~/.claude/settings.json`): 모든 프로젝트에 대한 개인 설정입니다.

환경 변수는 이 스택의 수준이 아닙니다. 동작에 셸 변수와 설정 키가 모두 있을 때, 어느 것이 적용되는지는 수준이 아닌 쌍별로 결정됩니다. 셸에서 내보낸 `ANTHROPIC_MODEL`은 모든 파일의 `model` 키보다 우선하지만, `ANTHROPIC_DEFAULT_MODEL`은 파일이 `model`을 설정하지 않을 때만 적용됩니다. [환경 변수 참조](/docs/ko/env-vars#precedence)는 어느 키가 쌍을 가지고 있으며 Claude Code가 먼저 읽는 것을 나타냅니다. 설정 파일 내의 `env` 블록은 일반 키이며 위의 수준을 따릅니다.

몇 가지 보안에 민감한 키의 경우, Claude Code는 관리되는 값보다 낮은 수준의 더 엄격한 값을 준수합니다. [관리되는 설정 우선순위의 예외](#exceptions-to-managed-settings-precedence)에 이를 나열합니다.

<h3 id="lists-merge-instead-of-overriding">
  목록은 재정의 대신 병합됩니다
</h3>

`permissions.allow`와 같은 동일한 목록 키를 둘 이상의 파일에서 설정할 때, Claude Code는 하나를 선택하는 대신 목록을 결합하므로 각 파일은 다른 파일의 항목을 제거하지 않고 항목을 추가할 수 있습니다. 모델 목록 또는 모델별 항목을 보유하는 네 가지 키는 자신의 규칙을 따릅니다.

* [`fallbackModel`](/docs/ko/settings-reference#fallbackmodel)은 위치가 의미를 갖는 정렬된 체인이므로, Claude Code는 이를 정의하는 가장 높은 우선순위 파일의 전체 값을 사용합니다.
* [`modelPicker`](/docs/ko/settings-reference#modelpicker)는 하나의 정렬된 행 목록과 교체 플래그를 보유하므로, Claude Code는 두 소스의 행을 병합하지 않습니다. 이를 정의하는 관리되는 설정, `--settings`, 및 사용자 설정 중 가장 높은 것의 전체 값을 사용하며, 프로젝트 및 로컬 설정의 키를 무시합니다. Claude Code v2.1.242 이상이 필요합니다.
* [`availableModels`](/docs/ko/settings-reference#availablemodels): Claude Code가 적용하는 관리되는 설정이 이를 정의할 때, Claude Code는 해당 목록을 그대로 적용하고 사용자, 프로젝트 또는 로컬 설정에서 추가하는 항목을 무시합니다. 단, Claude Code를 포함하는 앱이 자신의 모델 목록을 제공하는 경우는 제외합니다. [관리되는 설정 우선순위의 예외](#exceptions-to-managed-settings-precedence)를 참조하세요. 관리되는 소스 전체에서 목록은 병합되지 않습니다. [Claude Code가 관리되는 소스를 결합하는 방법](/docs/ko/managed-settings#how-claude-code-combines-managed-sources)은 어느 소스의 목록이 적용되는지 나타냅니다. 관리되지 않는 범위 전체에서 Claude Code는 일반적으로 배열을 병합합니다.
* [`modelSettings`](/docs/ko/settings-reference#modelsettings): Claude Code는 [`effortLevel`](/docs/ko/settings-reference#effortlevel)과 함께 한 번에 하나의 모델로 이를 해결합니다. `modelSettings` 항목은 어느 파일의 값이 모델에 적용되는지 나타냅니다.

<span id="examples" />

<h3 id="precedence-examples">
  우선순위 예제
</h3>

Claude가 작동하는 동안, Claude Code는 스피너 아래에 한 줄 팁을 표시합니다. 예를 들어 "Use /config to change your default permission mode (including Plan Mode)". 이러한 팁을 끄고 싶어서 `~/.claude/settings.json`에서 [`spinnerTipsEnabled`](/docs/ko/settings-reference#spinnertipsenabled)를 `false`로 설정했다고 가정합니다. 아래의 각 시나리오는 이를 다시 켤 수 있는 것과 이에 대해 할 수 있는 것입니다.

<h4 id="team-settings-override-personal-settings">
  팀 설정이 개인 설정을 재정의합니다
</h4>

팀의 `.claude/settings.json`이 이를 `true`로 설정합니다. Claude Code는 공유 프로젝트가 사용자 위에 있기 때문에 프로젝트 값을 사용하므로, 해당 프로젝트에서만 팁을 보고 다른 곳에서는 보지 않습니다.

값을 다시 얻을 수 있습니다. 해당 프로젝트의 `.claude/settings.local.json`에 `"spinnerTipsEnabled": false`를 추가합니다. 프로젝트 로컬이 공유 프로젝트 위에 있으므로, 해당 세션은 팁을 표시하지 않고 팀원의 세션은 변경되지 않습니다.

<h4 id="organization-settings-override-everything">
  조직 설정이 모든 것을 재정의합니다
</h4>

조직의 관리되는 설정이 이를 `true`로 설정합니다. 사용자, 프로젝트 또는 로컬 설정에 넣은 것도 팁을 끌 수 없으며, `--settings`도 마찬가지입니다. 관리되는 것이 최상위 수준입니다.

값을 다시 얻을 수 없습니다. `/status`를 실행하여 어느 관리되는 소스가 적용되는지 확인하고, 정책을 변경해야 하는지 관리자에게 문의합니다.

<h4 id="the-command-line-overrides-your-files-for-one-session">
  명령줄이 한 세션 동안 파일을 재정의합니다
</h4>

`claude --settings '{"spinnerTipsEnabled": true}'`로 세션을 시작했습니다. 명령줄은 관리되는 것을 제외한 모든 파일 위에 있으므로, 파일이 `false`라고 말해도 해당 세션은 팁을 표시합니다.

다음 세션에서 값을 다시 얻습니다. `--settings`는 한 세션 동안만 지속되며 파일에 쓰지 않습니다.

<h4 id="a-flag-or-environment-variable-sets-the-same-thing">
  플래그 또는 환경 변수가 동일한 것을 설정합니다
</h4>

일부 키에는 설정 값을 재정의하는 명령줄 플래그 또는 환경 변수가 있으며, 이는 어느 파일이 설정했는지와 관계없이 적용됩니다. `ANTHROPIC_MODEL`은 [`model`](/docs/ko/settings-reference#model) 설정을 재정의하고, `--model`은 한 세션 동안 둘 다 재정의합니다.

값을 다시 얻을 수 있는지는 키에 따라 다릅니다. 변수를 설정 해제하거나 플래그를 제거하고, [설정 참조](/docs/ko/settings-reference)의 키 항목과 [환경 변수 참조](/docs/ko/env-vars)의 변수 행을 확인하여 Claude Code가 어느 것을 사용하는지 확인합니다.

<span id="keys-ignored-in-a-repository-file" />

<span id="keys-only-you-or-your-organization-can-set" />

<span id="common-cases" />

<span id="which-value-applies-in-common-situations" />

<h3 id="troubleshoot-a-setting-that-doesn’t-apply">
  적용되지 않는 설정 문제 해결
</h3>

키를 설정했는데 Claude Code가 그렇게 동작하지 않으면, `/status`로 시작하여 로드한 파일을 확인한 다음 아래에서 증상을 찾습니다. [구성 디버그](/docs/ko/debug-your-config)는 깨끗한 구성 테스트를 포함한 더 넓은 검사를 다룹니다.

<h4 id="a-value-you-set-is-ignored">
  설정한 값이 무시됩니다
</h4>

다른 것이 동일한 키를 설정하거나, 파일이 해당 값을 설정할 수 없거나, 파일이 로드되지 않았습니다.

* **더 높은 수준이 이를 설정합니다.** 다른 설정 파일, `--settings` 플래그, 또는 관리되는 소스가 키를 위에서 설정합니다. [스택](#settings-precedence)은 어느 것인지 나타냅니다. 플래그 또는 환경 변수도 키별로 결정되는 키를 자체적으로 재정의할 수 있습니다. [설정 참조](/docs/ko/settings-reference)의 키 항목은 Claude Code가 어느 것을 사용하는지 나타내며, [`env` 항목](/docs/ko/settings-reference#env)은 관리되는 `env` 값 대 셸 내보내기를 다룹니다.
* **보안 키가 엄격한 값을 유지합니다.** 몇 가지 키의 경우 Claude Code는 모든 파일의 제한적인 값을 준수하므로, 프로젝트 `true`는 [`disableClaudeAiConnectors`](/docs/ko/settings-reference#disableclaudeaiconnectors)에 대해 켜진 상태로 유지됩니다. [관리되는 설정 우선순위의 예외](#exceptions-to-managed-settings-precedence)를 참조하세요.
* **파일이 해당 값을 설정할 수 없습니다.** [`permissions.defaultMode`](/docs/ko/settings-reference#permissions-defaultmode) 값 `auto`와 `bypassPermissions`은 프로젝트 또는 로컬 설정에서 적용되지 않습니다. 대신 사용자 또는 관리되는 설정에서 설정하거나, 한 세션 동안 `--permission-mode`를 전달합니다. v2.1.257 이전에는 `bypassPermissions`이 모든 파일에서 적용되었습니다.

  설정 파일 내의 [`env`](/docs/ko/settings-reference#env) 블록의 원격 분석 내보내기 변수도 프로젝트 또는 로컬 설정에서 적용되지 않습니다. 몇 가지 끄기 값은 제외합니다. [Claude Code가 `env`에서 무시하는 변수](/docs/ko/settings-reference#variables-claude-code-ignores-in-env)는 변수와 해당 값을 나열합니다.
* **파일이 손상되었습니다.** 잘못된 JSON 또는 거부된 값은 Claude Code가 파일 또는 항목을 건너뜁니다. [손상된 설정 파일 수정](#fix-a-broken-settings-file)을 참조하세요.

<h4 id="a-change-you-made-in-claude-code-is-lost-in-new-sessions">
  Claude Code에서 만든 변경 사항이 새 세션에서 손실됩니다
</h4>

Claude Code 내에서 새 세션을 위한 선택을 저장할 때, 예를 들어 `/model`로 기본 모델을 저장할 때, Claude Code는 사용자 설정 파일 `~/.claude/settings.json`에 씁니다. 해당 파일에 쓸 수 없으면, 예를 들어 다른 도구가 생성하거나 읽기 전용 복사본에 연결하면, 변경 사항은 현재 세션에 적용되고 다음 세션에서 손실됩니다. 파일을 생성하는 도구에서 키를 설정하거나, 파일을 쓸 수 있는 파일로 바꿉니다.

파일에 쓸 수 있고 변경 사항이 여전히 지속되지 않으면, 변경 사항이 [한 세션 동안만](#change-a-setting-for-one-session) 또는 [더 높은 수준이 동일한 키를 설정](#a-value-you-set-is-ignored)했는지 확인합니다. `model` 키의 경우, [새 세션이 선택한 것과 다른 모델에서 시작됩니다](/docs/ko/model-config#a-new-session-starts-on-a-different-model-than-you-picked)는 더 많은 원인을 나열합니다.

<h4 id="a-managed-change-hasn’t-reached-you">
  관리되는 변경 사항이 도달하지 않았습니다
</h4>

관리되는 소스는 [전달 테이블](/docs/ko/managed-settings#choose-a-delivery-mechanism)의 일정에 따라 실행 중인 세션에 도달하므로, 먼저 세션을 다시 시작합니다. `/status`가 관리자가 변경한 것과 다른 소스를 나타내면, 더 높은 우선순위 소스가 적용됩니다. [Claude Code가 관리되는 소스를 결합하는 방법](/docs/ko/managed-settings#how-claude-code-combines-managed-sources)은 순서를 제공합니다.

<h4 id="a-committed-key-doesn’t-reach-teammates">
  커밋된 키가 팀원에게 도달하지 않습니다
</h4>

두 가지가 `.claude/settings.json`의 키가 이를 복제하는 모든 사람에게 적용되는 것을 방지합니다.

* **Claude Code는 저장소 파일의 키를 무시합니다.** [설정 인덱스](/docs/ko/settings-reference#settings-index)의 범위 열에서 `User, local, or managed`, `User or managed`, `Managed`, 또는 `Global config`를 찾습니다. 이러한 키는 공유 파일에서 적용되지 않습니다. 저장소 파일은 여전히 키를 끌 수 있습니다. `Global config` 키는 `~/.claude.json`에서만 적용됩니다.

  `env` 키 내에서, 원격 분석 내보내기 변수는 공유 파일에서 적용되지 않습니다. 몇 가지 끄기 값은 제외합니다. [Claude Code가 `env`에서 무시하는 변수](/docs/ko/settings-reference#variables-claude-code-ignores-in-env)를 참조하세요.
* **키가 신뢰를 기다립니다.** `permissions.allow` 규칙, `permissions.additionalDirectories`, `extraKnownMarketplaces`, 및 대부분의 [`env`](/docs/ko/settings-reference#env) 값은 각 팀원이 [폴더를 신뢰](/docs/ko/permissions#project-allow-rules-and-workspace-trust)한 후에만 적용됩니다. 그때까지 그들은 여전히 프롬프트를 보고 파일이 선언하는 마켓플레이스에서 플러그인을 얻지 못합니다. `deny`와 `ask` 규칙은 즉시 적용됩니다.

<h4 id="permission-rules-combine-differently-than-you-expected">
  권한 규칙이 예상과 다르게 결합됩니다
</h4>

* **권한 프롬프트에서 "Yes, and don't ask again"을 선택했지만 여전히 동일한 도구에 대해 프롬프트를 받습니다.** 해당 선택은 `allow` 규칙을 로컬 파일에 저장했으며, 로컬 파일의 `allow` 규칙은 프로젝트 또는 관리되는 파일의 `ask` 규칙을 능가하지 않습니다. [권한 규칙이 결합되는 방법](/docs/ko/permissions#settings-precedence)은 순서를 설명합니다. VS Code 확장에서 승인 카드는 프로젝트의 공유 파일을 포함한 대상 파일을 선택할 수 있으며, 이는 모든 사람을 위한 규칙을 변경합니다. CLI에서 Claude Code는 로컬 파일에만 씁니다.
* **조직의 허용 규칙이 여전히 사용자의 규칙과 함께 적용됩니다.** 이는 예상된 것입니다. Claude Code는 조직이 [`allowManagedPermissionRulesOnly`](/docs/ko/settings-reference#allowmanagedpermissionrulesonly)를 설정하지 않는 한 범위 전체에서 [`permissions.allow`](/docs/ko/settings-reference#permissions-allow)를 병합합니다.

<span id="security-keys-where-the-stricter-value-applies" />

<h3 id="exceptions-to-managed-settings-precedence">
  관리되는 설정 우선순위의 예외
</h3>

값이 세션을 제한하는 몇 가지 키의 경우, Claude Code는 관리되는 설정을 재정의할 수 없는 범위의 제한적인 값을 준수합니다. 이 표에서 키를 찾아 어느 값을 준수하는지 그리고 어디서 준수하는지 확인합니다.

| 키                                                                               | Claude Code가 준수하는 값                                                                                     | 참고                                                                                                                              |
| :------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------ |
| [`disableClaudeAiConnectors`](/docs/ko/settings-reference#disableclaudeaiconnectors) | 모든 범위의 `true`                                                                                           | 관리되는 소스가 `false`를 설정할 때도 준수됩니다                                                                                                  |
| [`enableArtifact`](/docs/ko/settings-reference#enableartifact)                       | 모든 범위의 `false`, 그리고 모든 범위의 `disableArtifact: true`                                                      | 관리되는 소스가 `true`를 설정할 때도 준수됩니다. 아무것도 [Artifact 도구](/docs/ko/artifacts#disable-artifacts)를 다시 켤 수 없습니다. Claude Code v2.1.242 이상이 필요합니다 |
| [`isolatePeerMachines`](/docs/ko/settings-reference#isolatepeermachines)             | 모든 범위의 `true`                                                                                           | 관리되는 소스가 `false`를 설정할 때도 준수됩니다                                                                                                  |
| [`remoteControlAtStartup`](/docs/ko/settings-reference#remotecontrolatstartup)       | `.claude/settings.json` 또는 `.claude/settings.local.json`의 `false`                                       | 관리되는 소스가 `true`를 설정할 때도 준수됩니다. 프로젝트 또는 로컬 `true`는 무시됩니다                                                                         |
| [`crossSessionInbound`](/docs/ko/settings-reference#crosssessioninbound)             | `.claude/settings.json` 또는 `.claude/settings.local.json`의 더 엄격한 값, `accept` \< `hold` \< `refuse` 사다리에서 | 관리되는, `--settings`, 및 사용자 값보다 준수됩니다. 더 엄격하지 않은 프로젝트 또는 로컬 값은 무시됩니다                                                              |
| [`useAutoModeDuringPlan`](/docs/ko/settings-reference#useautomodeduringplan)         | 모든 관리되는 소스, `--settings`, `~/.claude/settings.json`, 또는 `.claude/settings.local.json`의 `false`          | 승리한 관리되는 소스가 `true`를 설정할 때도 준수됩니다. `.claude/settings.json`의 `false`는 무시됩니다                                                      |
| [`syncClaudeAiSkills`](/docs/ko/settings-reference#syncclaudeaiskills)               | 모든 관리되는 소스, `--settings`, `~/.claude/settings.json`, 또는 `.claude/settings.local.json`의 `false`          | 승리한 관리되는 소스가 `true`를 설정할 때도 준수됩니다. `.claude/settings.json`의 `false`는 무시됩니다                                                      |
| [`syncClaudeAiPlugins`](/docs/ko/settings-reference#syncclaudeaiplugins)             | 모든 관리되는 소스, `--settings`, `~/.claude/settings.json`, 또는 `.claude/settings.local.json`의 `false`          | 승리한 관리되는 소스가 `true`를 설정할 때도 준수됩니다. `.claude/settings.json`의 `false`는 무시됩니다                                                      |
| [`maxEffortLevel`](/docs/ko/settings-reference#maxeffortlevel)                       | `--settings`를 포함한 모든 범위의 더 낮은 상한                                                                        | Claude Code가 적용하는 관리되는 설정이 더 높은 상한을 설정할 때도 준수됩니다. 가장 낮은 상한이 적용됩니다. Claude Code v2.1.267 이상이 필요합니다                               |

[`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/ko/env-vars)를 설정하고 Claude Code를 자신 내부에서 실행하는 앱도 예외입니다. Claude Code는 모든 관리되는 소스의 `model`, `fallbackModel`, `modelPicker`, 및 `modelOverrides` 키보다, 그리고 관리되는 `env` 블록의 모델 선택 변수(예: `ANTHROPIC_MODEL` 및 `ANTHROPIC_DEFAULT_*_MODEL` 계열)보다 해당 앱의 모델 구성을 사용합니다. Claude Code는 앱이 자신의 것을 제공하지 않는 한 관리되는 [`availableModels`](/docs/ko/settings-reference#availablemodels) 허용 목록을 적용 중으로 유지합니다.

<h2 id="settings-in-cloud-sessions">
  클라우드 세션의 설정
</h2>

[클라우드 세션](/docs/ko/claude-code-on-the-web)은 [클라우드 환경](/docs/ko/cloud-environments)에서 저장소의 신선한 복제본에서 실행되며, 머신에서 실행되지 않습니다. 이는 어느 설정이 도달하는지 변경합니다:

* **공유 프로젝트 설정** (`.claude/settings.json`): 한 저장소가 있는 세션에서 읽습니다. 파일이 복제본의 일부이고 세션이 그 안에서 시작되기 때문입니다. 해당 세션에 적용하려면 설정을 거기에 커밋합니다. 여러 저장소가 있는 세션은 복제본 위에서 시작되고 각 저장소의 `.claude/settings.json`에서 `enabledPlugins` 및 `extraKnownMarketplaces` 키만 읽으며, 권한 규칙, hooks, `env` 또는 기타 키는 읽지 않습니다. 이 두 키가 선언하는 마켓플레이스와 플러그인은 여전히 [클라우드 세션에서 로드되지 않습니다](/docs/ko/cloud-environments#what-carries-over-from-your-setup).
* **사용자 및 프로젝트 로컬 설정** (`~/.claude/settings.json` 및 `.claude/settings.local.json`): 읽지 않습니다. 둘 다 머신에 유지되고 로컬 파일은 복제본에 없습니다.
* **관리되는 설정**: [서버 관리 설정](/docs/ko/server-managed-settings)만 클라우드 세션에 도달합니다. 장치의 `managed-settings.json` 파일이나 MDM 프로필은 도달하지 않습니다. [자체 호스팅 환경](/docs/ko/self-hosted-environments)도 실행기 이미지의 관리되는 설정 파일을 읽습니다. [Claude Code가 관리되는 소스를 결합하는 방식](/docs/ko/managed-settings#how-claude-code-combines-managed-sources)은 해당 파일이 적용되는 시기를 설명합니다.
* **`/config`**: 브라우저에서 claude.ai/code에서 설정 값을 변경하는 대신 claude.ai 설정의 Claude Code 섹션을 엽니다. 클라우드 세션에 대해 설정을 변경하려면 환경에서 [환경 변수](/docs/ko/cloud-environments#set-environment-variables)를 설정하거나, 한 저장소가 있는 세션에서 해당 저장소의 `.claude/settings.json`에 키를 커밋합니다.

[설정에서 수행되는 것](/docs/ko/cloud-environments#what-carries-over-from-your-setup)은 나머지를 나열합니다: `CLAUDE.md`, skills, MCP servers, plugins, 및 credentials.

<h2 id="what’s-next">
  다음 단계
</h2>

* [모든 설정](/docs/ko/settings-reference): 모든 키, 설정 위치, 기본값, 예제
* [예제 설정 파일](/docs/ko/settings-example): 개인 파일, 팀 파일, 조직의 관리되는 파일
* [권한 구성](/docs/ko/permissions): allow, ask, deny 규칙, Claude Code가 묻지 않고 실행하는 것
* [환경 변수](/docs/ko/env-vars): Claude Code가 읽는 변수 및 `env` 블록
* [구성 디버깅](/docs/ko/debug-your-config): 설정이 적용되지 않을 때
* [Claude 디렉토리 참조](/docs/ko/claude-directory): Claude Code가 읽는 모든 파일. subagents, MCP 서버, 플러그인, `CLAUDE.md` 포함
