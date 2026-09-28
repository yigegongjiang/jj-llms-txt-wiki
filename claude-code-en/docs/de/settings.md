> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Einstellungsdateien und Priorität

> Ändern Sie Claude Code-Einstellungen, wählen Sie den Bereich aus, zu dem ein Schlüssel gehört, überprüfen Sie die Änderung, und erfahren Sie, welchen Wert Claude Code verwendet, wenn ein Schlüssel an mehreren Stellen gesetzt ist.

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

Einstellungen sind JSON-Schlüssel, die ändern, wie Claude Code sich verhält: welches Modell es startet, was es ohne Nachfrage ausführen kann, welche Dateien es nicht lesen kann, wie es in Ihrem Terminal aussieht, und was Ihre Organisation erzwingt.

<Tip>
  Um einen bestimmten Schlüssel nachzuschlagen, gehen Sie zu [Alle Einstellungen](/docs/de/settings-reference), die jeden Schlüssel mit der Datei, in der Sie ihn setzen, seinem Standard und einem Beispiel auflistet.
</Tip>

Claude Code liest Einstellungen aus JSON-Einstellungsdateien wie `~/.claude/settings.json`. Es sucht an einigen Stellen danach, und [die Datei, aus der es eine Einstellung liest, entscheidet, für wen die Einstellung gilt](#settings-files-and-who-they-affect). Diese Seite behandelt diese Dateien: in welche man eine Einstellung setzt, wie man eine Einstellung ändert und bestätigt, dass sie angewendet wurde, und welchen Wert Claude Code verwendet, wenn derselbe Schlüssel in mehr als einer Datei gesetzt ist. [Berechtigungen konfigurieren](/docs/de/permissions) behandelt, was Claude Code ohne Nachfrage ausführen kann und wie man `allow`, `ask` und `deny` Regeln schreibt.

<Note>
  Diese Seite behandelt Claude Code, das auf Ihrer Maschine läuft: das Terminal, die [VS Code](/docs/de/vs-code) und [JetBrains](/docs/de/jetbrains) Erweiterungen und die [Desktop-App](/docs/de/desktop), die alle die gleichen Einstellungsdateien lesen. Eine Cloud-Sitzung auf [Claude Code im Web](/docs/de/claude-code-on-the-web) läuft auf einer anderen Maschine und liest nur einige davon; siehe [Einstellungen in Cloud-Sitzungen](#settings-in-cloud-sessions).
</Note>

<span id="settings-files" />

<span id="configuration-scopes" />

<span id="available-scopes" />

<span id="when-to-use-each-scope" />

<span id="what-uses-scopes" />

<span id="subagent-configuration" />

<span id="where-settings-live" />

<h2 id="settings-files-and-who-they-affect">
  Einstellungsdateien und wer sie betreffen
</h2>

Claude Code liest Einstellungen aus vier Dateien, und eine Organisation kann auch verwaltete Einstellungen über die claude.ai-Konsole bereitstellen. Jede Quelle hat einen Geltungsbereich: die Menge von Personen und Projekten, auf die eine in ihr gespeicherte Einstellung angewendet wird, ob das nur Sie, alle in einem Projekt oder alle in Ihrer Organisation sind.

| Geltungsbereich     | Datei                                                                                             | Wer es betrifft                                                                                                                                                                                      | Verwenden Sie es für                                                                                 |
| :------------------ | :------------------------------------------------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------- |
| Benutzer            | `~/.claude/settings.json`                                                                         | Sie, in jedem Projekt auf diesem Computer                                                                                                                                                            | Persönliche Voreinstellungen: Design, Editor-Modus, Standardmodell, Ihre eigenen Berechtigungsregeln |
| Gemeinsames Projekt | `.claude/settings.json`                                                                           | Alle, die im Ordner arbeiten, der es enthält. In einem Git-Repository committen Sie es, damit Ihre Teamkollegen es erhalten                                                                          | Team-Berechtigungen, Hooks, Plugins und die Umgebungsvariablen, die das Projekt benötigt             |
| Projekt lokal       | `.claude/settings.local.json`                                                                     | Sie, nur in diesem einen Projekt. Claude Code hält es aus Git heraus, wenn es die Datei erstellt; wenn Sie sie von Hand erstellen, fügen Sie sie selbst zu `.gitignore` hinzu                        | Persönliche Überschreibungen für ein Projekt und Tests vor dem Teilen                                |
| Verwaltet           | `managed-settings.json` und andere [verwaltete Quellen](/docs/de/managed-settings#delivery-mechanisms) | Alle, für die Ihre Organisation sie bereitstellt; nichts, das Sie festlegen, überschreibt es, außer einigen wenigen [sicherheitsempfindlichen Ausnahmen](#exceptions-to-managed-settings-precedence) | Sicherheitsrichtlinie und Compliance-Anforderungen                                                   |

In der Spalte „Datei" ist `~/.claude` der `.claude`-Ordner in Ihrem Home-Verzeichnis, und ein einfaches `.claude` ist der `.claude`-Ordner in Ihrem Projekt.

<span id="where-each-file-applies" />

<span id="compare-what-each-file-reaches" />

<h3 id="compare-the-scope-of-each-settings-file">
  Vergleichen Sie den Geltungsbereich jeder Einstellungsdatei
</h3>

Angenommen, Sie haben drei Projekte auf Ihrem Computer: `website/`, `api/` und `acme-app/`, ein Teamkollege hat seinen eigenen Klon von `acme-app/`, und Sie starten eine [Cloud-Sitzung](#settings-in-cloud-sessions) auf `acme-app/`.

Die folgende Grafik zeigt, in welchen dieser Ordner eine Einstellung angewendet wird, wenn Sie Claude Code von ihnen aus starten. Klicken Sie auf eine Einstellungsdatei, um die Ordner anzuzeigen, die sie erreicht.

<SettingsScope />

* **`~/.claude/settings.json`**: jedes Projekt auf Ihrem Computer, und nichts auf dem Computer Ihres Teamkollegen oder in der Cloud-Sitzung
* **`acme-app/.claude/settings.json`**: Ihr `acme-app/`. Es erreicht den Klon Ihres Teamkollegen und die Cloud-Sitzung nur, wenn Sie die Datei in die Versionskontrolle committen; bis dahin ist es eine Datei auf Ihrer Festplatte wie jede andere und niemand sonst hat sie
* **`acme-app/.claude/settings.local.json`**: nur Ihr `acme-app/`. Claude Code fügt es beim ersten Mal, wenn es die Datei schreibt, zu Ihren globalen Git-Ausschlüssen hinzu, damit es aus Ihren Commits herausbleibt; wenn Sie die Datei von Hand erstellen, [fügen Sie sie selbst zu `.gitignore` hinzu](#keep-personal-settings-out-of-a-repository)
* **Verwaltete Einstellungen**, ob eine `managed-settings.json`-Datei, eine MDM-Richtlinie oder [servergesteuerte Einstellungen](/docs/de/server-managed-settings) aus der claude.ai-Konsole: jedes Projekt auf jedem Computer, auf dem Ihre Organisation sie bereitstellt, oder auf dem Sie sich mit Ihrem Organisationskonto anmelden. Nur servergesteuerte Einstellungen erreichen die Cloud-Sitzung

<span id="which-files-you-have" />

<h3 id="find-or-create-your-settings-files">
  Suchen oder erstellen Sie Ihre Einstellungsdateien
</h3>

Die Installation von Claude Code erstellt keine Einstellungsdatei. Wenn Ihr Computer oder Projekt bereits eine hat, kam sie aus einer dieser Quellen:

* **Verwaltet**: Ihre Organisation stellt sie bereit. Sie erstellen oder bearbeiten sie nicht.
* **Gemeinsames Projekt**: Ein Projekt, das bereits Claude Code verwendet, kann eine committed haben. Wenn nicht, erstellen Sie sie unter `.claude/settings.json` im Projektordner.
* **Benutzer** und **Projekt lokal**: Erstellen Sie sie selbst, oder lassen Sie Claude Code sie erstellen. Es schreibt `~/.claude/settings.json` das erste Mal, wenn Sie eine Option im `/config`-Menü ändern, die es in Benutzereinstellungen speichert, wie das Design, und `.claude/settings.local.json` das erste Mal, wenn Sie eine dauerhafte Genehmigung bei einer Berechtigungsaufforderung geben, wie „Ja, und frag mich nicht mehr" für einen Bash-Befehl. Einige `/config`-Optionen, einschließlich **Tipps anzeigen**, werden stattdessen in `.claude/settings.local.json` gespeichert.

<Info>
  Unter Windows bedeutet `~/.claude` `%USERPROFILE%\.claude`. Um die Home-Verzeichnis-Dateien woanders zu speichern, setzen Sie [`CLAUDE_CONFIG_DIR`](/docs/de/env-vars); Claude Code speichert dann Ihre Einstellungen, Sitzungsverlauf und Plugins stattdessen dort.
</Info>

Claude Code behält auch eine fünfte Datei, [`~/.claude.json`](/docs/de/claude-directory#ce-claude-json), die es für sich selbst schreibt; Sie müssen sie nicht bearbeiten. Sie enthält Ihre Anmeldungssitzung, [MCP-Server](/docs/de/mcp)-Konfigurationen, projektspezifischen Status wie Vertrauensentscheidungen und die [globalen Konfigurationsschlüssel](/docs/de/settings-reference#global-config-settings), die `/config` für Sie schreibt.

<h3 id="share-settings-with-your-team">
  Teilen Sie Einstellungen mit Ihrem Team
</h3>

Committen Sie `.claude/settings.json`, damit jeder, der das Repository klont, die gleichen Berechtigungen, Hooks und Plugins erhält. Jeder Teamkollege kann es für sich selbst in seiner eigenen `.claude/settings.local.json` überschreiben, sodass persönliche Ausnahmen keinen Commit benötigen. Für eine vollständige Team-Datei siehe [die gemeinsamen Einstellungen eines Teams](/docs/de/settings-example#a-teams-shared-settings).

Einiges von dem, das Sie committen, wartet, bis jeder Teamkollege den [Ordner vertraut](/docs/de/permissions#project-allow-rules-and-workspace-trust), und einige Schlüssel wirken sich nie aus einer Repository-Datei aus; [Beheben Sie eine Einstellung, die nicht angewendet wird](#common-cases) behandelt beides.

<span id="local-settings-file" />

<span id="where-claude-code-saves-the-project-local-file" />

<span id="the-project-local-file" />

<span id="keep-personal-settings-out-of-the-repository" />

<h3 id="keep-personal-settings-out-of-a-repository">
  Halten Sie persönliche Einstellungen aus einem Repository heraus
</h3>

Um eine Einstellung für sich selbst in einem Projekt zu ändern, ohne sie für Ihre Teamkollegen zu ändern, speichern Sie sie in `.claude/settings.local.json` im Projekt. Claude Code wendet diese Datei über die committete `.claude/settings.json` an, also wenn die Datei Ihres Teams `"model": "claude-sonnet-5"` setzt und Sie Opus möchten, fügen Sie `"model": "claude-opus-5-5"` in Ihre lokale Datei ein und nur Ihre Sitzungen ändern sich.

Claude Code schreibt auch in diese Datei, hält sie aus Ihren Commits heraus und wendet ihre Berechtigungsregeln ohne den Vertrauensschritt an:

* **Claude Code schreibt sie auch.** Wenn Claude die Berechtigung zum Ausführen eines Bash-Befehls anfordert und Sie „Ja, und frag mich nicht mehr" wählen, speichert Claude Code diese [Berechtigungsgenehmigung](/docs/de/permissions#permission-system) hier als `allow`-Regel.
* **Sie müssen sie nicht selbst gitignorieren, es sei denn, Sie haben sie von Hand erstellt.** Das erste Mal, wenn Claude Code die Datei in einem Git-Repository schreibt, das sie nicht bereits ignoriert, fügt es `**/.claude/settings.local.json` zu Ihrer globalen Git-Ausschluss-Datei hinzu, damit die Datei in jedem Repository aus Ihren Commits herausbleibt. Diese Datei ist `core.excludesFile`, wenn Ihre globale Git-Konfiguration sie auf einen absoluten oder `~`-präfixierten Pfad setzt; andernfalls ist es `$XDG_CONFIG_HOME/git/ignore`, oder `~/.config/git/ignore`, wenn `XDG_CONFIG_HOME` nicht gesetzt ist. Wenn Sie die Datei von Hand erstellt haben und Claude Code noch nicht darin geschrieben hat, fügen Sie sie selbst zu `.gitignore` hinzu.
* **Seine Berechtigungsregeln warten nicht auf Vertrauen, während die Datei nicht verfolgt bleibt.** Da die Datei Ihnen gehört und nicht dem Repository, wendet Claude Code ihre `allow`-Regeln ohne den [Workspace-Vertrauens](/docs/de/permissions#project-allow-rules-and-workspace-trust)-Schritt an, den es für die committete Datei erfordert. Wenn die Datei von Git verfolgt wird, gilt der Vertrauensschritt auch für sie; siehe [Wenn Ihre lokale Einstellungsdatei Vertrauen benötigt](/docs/de/permissions#when-your-local-settings-file-needs-trust).

<span id="where-claude-code-looks-for-each-file" />

<span id="how-claude-code-keeps-the-local-file-out-of-git" />

<span id="local-allow-rules-dont-wait-for-workspace-trust" />

<h4 id="where-claude-code-keeps-the-local-file-in-a-git-repository">
  Wo Claude Code die lokale Datei in einem Git-Repository hält
</h4>

Wenn Claude die Berechtigung zum Ausführen eines Bash-Befehls anfordert und Sie „Ja, und frag mich nicht mehr" wählen, speichert Claude Code diese Genehmigung als `allow`-Regel in `.claude/settings.local.json`. Wenn Sie Claude Code in einem Unterverzeichnis eines Git-Repositories starten, liest und schreibt es diese Datei im Repository-Root und wendet die Genehmigung auf das gesamte Repository an. In einem [Worktree](/docs/de/worktrees) verwendet es die Datei im Root des Haupt-Checkouts.

Zwei Regeln qualifizieren den Root-Speicherort:

* **Wenn die Datei stattdessen bei `.claude/settings.json` bleibt**: außerhalb eines Git-Repositories, wenn der Repository-Root Ihr Home-Verzeichnis ist, unter Windows oder wenn der Repository-Root oder sein `.git`- oder `.claude`-Eintrag nicht Ihrem Benutzer gehört.
* **Pfade in der Datei verankern nicht im Repository-Root**: eine Berechtigungsregel, die mit `/` beginnt oder ein relativer Sandbox-Pfad [verankert stattdessen im primären Arbeitsverzeichnis der Sitzung](/docs/de/permissions#read-and-edit).

Vor v2.1.211 hielt Claude Code die Datei im Startverzeichnis. Es liest immer noch eine Datei, die eine frühere Version dort neben der Root-Datei hinterlassen hat; wo beide denselben Schlüssel setzen, gilt der Wert des Root, und Berechtigungsregeln aus beiden Dateien gelten. Der [`resolveSettings()`](/docs/de/agent-sdk/typescript#resolvesettings)-Helfer des Agent SDK liest die Datei immer aus dem Startverzeichnis.

Claude Code liest die gemeinsame `.claude/settings.json` aus dem [primären Arbeitsverzeichnis](/docs/de/permissions#working-directories) der Sitzung, also um eine Datei zu verwenden, die im Repository-Root committed ist, starten Sie Claude Code dort. Nachdem Sie [die Sitzung mit `/cd` verschieben](/docs/de/permissions#move-the-session-to-another-directory), liest Claude Code stattdessen beide Projektdateien aus dem neuen Verzeichnis, wobei die lokale Datei nach denselben Regeln platziert wird. Das Lesen von ihnen aus dem Verzeichnis, in das Sie verschoben haben, erfordert Claude Code v2.1.246 oder später.

<span id="managed-settings-delivery" />

<span id="precedence-within-the-managed-tier" />

<span id="parent-settings-from-embedding-hosts" />

<span id="enforce-settings-for-an-organization" />

<span id="settings-your-organization-manages" />

<h3 id="check-what-your-organization-enforces">
  Überprüfen Sie, was Ihre Organisation erzwingt
</h3>

Wenn Ihre Organisation Claude Code verwaltet, werden einige Einstellungen für Sie entschieden und nichts, das Sie in Ihre eigenen Dateien einfügen, ändert sie. Um zu sehen, welche, führen Sie `/status` aus: die Zeile `Setting sources` nennt die verwaltete Quelle, die auf Sie angewendet wird. Verwaltete Einstellungen gelten überall dort, wo Claude Code auf diesem Computer läuft; [Was ein Entwickler ändern kann](/docs/de/managed-settings#what-a-developer-can-change) behandelt lokale Administratorrechte und Tools außer Claude Code.

Verwaltete Einstellungen erreichen Sie durch die [Bereitstellungsmechanismen](/docs/de/managed-settings#delivery-mechanisms) auf der Seite für verwaltete Einstellungen, am häufigsten:

* [Servergesteuerte Einstellungen](/docs/de/server-managed-settings), die Claude Code aus der claude.ai-Admin-Konsole oder einem selbstgehosteten [Claude-Apps-Gateway](/docs/de/claude-apps-gateway) abruft
* MDM- oder Betriebssystem-Richtlinien und `managed-settings.json`-Dateien in einem Systemverzeichnis
* Ein Embedding-Host wie Claude Desktop, über die SDK-Option `managedSettings`; siehe [Steuern Sie die Richtlinie von einem Embedding-Host](/docs/de/managed-settings#parent-settings-from-embedding-hosts)

In einer [Cowork](https://claude.com/docs/cowork/overview)-Sitzung, die auf Ihrem Computer in der Claude Desktop-App läuft, ruft Claude Code keine servergesteuerten Einstellungen aus der claude.ai-Admin-Konsole ab, und es liest die Richtlinie, die auf Ihrem Gerät bereitgestellt ist, es sei denn, die Claude Desktop-Konfiguration Ihrer Organisation setzt `requireCoworkFullVmSandbox`. [Wo und wann eine Richtlinie angewendet wird](/docs/de/managed-settings#where-and-when-a-policy-applies) behandelt Cowork und Cloud-Sitzungen.

Wenn Sie der Administrator sind, führt Sie [Richten Sie Claude Code für Ihre Organisation ein](/docs/de/admin-setup) durch die Auswahl, was Sie erzwingen möchten, und [Stellen Sie verwaltete Einstellungen bereit](/docs/de/managed-settings) behandelt die Bereitstellung und wie Sie bestätigen, dass eine Richtlinie in Kraft ist.

<h2 id="change-a-setting">
  Ändern Sie eine Einstellung
</h2>

Sie können eine Einstellung vom `/config` Menü, durch Bearbeiten einer Einstellungsdatei oder für eine Sitzung von der Befehlszeile aus ändern.

<span id="system-prompt" />

Claude Code's Systemaufforderung wird nicht veröffentlicht. Um Claude stehende Anweisungen zu geben, verwenden Sie [`CLAUDE.md` Dateien](/docs/de/memory) oder das Flag `--append-system-prompt`.

<h3 id="use-the-/config-menu">
  Verwenden Sie das /config Menü
</h3>

Führen Sie `/config` in Claude Code aus und öffnen Sie die Registerkarte **Config**. Es listet eine kurze Menge persönlicher Optionen wie Design, Editor-Modus und ausführliche Ausgabe auf, nicht jeden Einstellungsschlüssel. Wählen Sie eine Option, um sie zu ändern; Claude Code speichert sie für Sie:

* **Die meisten Optionen**: `~/.claude/settings.json`
* **Ein paar Optionen, wie Tipps anzeigen**: `.claude/settings.local.json`
* **Die [globalen Konfigurationsoptionen](/docs/de/settings-reference#global-config-settings)**: `~/.claude.json`

Um eine Option ohne das Menü zu setzen, übergeben Sie `key=value`, wie `/config verbose=true`.

<Note>
  `/config` ist Teil der Terminal-Schnittstelle. Das [VS Code](/docs/de/vs-code) Chat-Panel und die [Desktop-App](/docs/de/desktop) öffnen es nicht; ändern Sie Einstellungen dort durch Bearbeiten einer Einstellungsdatei oder durch die Einstellungen dieser Apps.
</Note>

<h3 id="edit-a-settings-file">
  Bearbeiten Sie eine Einstellungsdatei
</h3>

Öffnen Sie die Einstellungsdatei für den Bereich, den Sie möchten, in Ihrem Editor und fügen Sie einen Schlüssel hinzu oder ändern Sie ihn. Einstellungsdateien sind striktes JSON: ein `//` Kommentar oder ein nachfolgendes Komma ist ein Syntaxfehler, und Claude Code meldet die Datei als [Einstellungsfehler](#fix-a-broken-settings-file) beim nächsten Start. Um beispielsweise Claude Code Ihre Lint- und Test-Befehle ohne Nachfrage ausführen zu lassen und es daran zu hindern, `.env` Dateien zu lesen, fügen Sie dies zu `~/.claude/settings.json` hinzu:

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

Jeder Eintrag unter `permissions` ist eine Regel, die ein Tool und das, was es darf, benennt; [Berechtigungen konfigurieren](/docs/de/permissions) erklärt die Syntax. Die Zeile `$schema` verweist auf das [veröffentlichte JSON-Schema](https://json.schemastore.org/claude-code-settings.json) für Claude Code-Einstellungen, das Ihnen Autovervollständigung und Inline-Validierung in VS Code, Cursor und jedem anderen Editor gibt, der JSON-Schema unterstützt. Das Schema kann hinter den neuesten CLI-Versionen zurückbleiben, sodass eine Validierungswarnung auf einem kürzlich dokumentierten Schlüssel nicht bedeutet, dass Ihre Konfiguration ungültig ist.

Nachdem Sie speichern, führen Sie `/status` in Claude Code aus, um zu bestätigen, dass die Datei geladen wurde; [Bestätigen Sie, was geladen wurde](#check-what-loaded) sagt, was die Zeile `Setting sources` zeigt und wie eine fehlerhafte Datei gemeldet wird.

Für eine vollständige persönliche Datei, Team-Datei und Organisations-Datei, jeweils mit einem Kommentar zu jedem Schlüssel, den sie setzt, siehe die [Beispiel-Einstellungsdateien](/docs/de/settings-example).

<span id="pass-settings-for-one-session" />

<h3 id="change-a-setting-for-one-session">
  Ändern Sie eine Einstellung für eine Sitzung
</h3>

Um einen Wert zu versuchen, ohne ihn zu speichern, setzen Sie ihn, wenn Sie Claude Code starten. Der Wert gilt für diese Sitzung und Ihre Einstellungsdateien bleiben wie sie waren. Sie haben drei Möglichkeiten, es zu tun:

* **`--settings`**: übergeben Sie einen Schlüssel als JSON, inline oder als Pfad zu einer Datei. Claude Code wendet ihn über Ihre Benutzer-, Projekt- und lokalen Dateien und unter verwalteten Einstellungen an. Es kann jeden Schlüssel setzen, den Ihre Benutzereinstellungsdatei kann; es kann keine `Managed` oder `Global config` Schlüssel setzen.
* **Ein Flag für diesen Schlüssel**: einige Schlüssel haben ihr eigenes Flag, wie `--model` für `model` und `--effort` für `effortLevel` und `modelSettings`.
* **Eine Umgebungsvariable**: exportieren Sie die gepaarte Variable des Schlüssels, bevor Sie `claude` ausführen, wie `ANTHROPIC_MODEL` für `model`.

Jeder Eintrag des Schlüssels auf der [Einstellungsreferenz](/docs/de/settings-reference) listet seine Überschreibungen pro Sitzung und welche Vorrang hat auf, also überprüfen Sie den Eintrag für den Schlüssel, den Sie ändern möchten.

Befehle, die Sie in einer Sitzung ausführen, speichern meist Ihre Wahl: wenn Sie eine Einstellung in `/config` ändern, schreibt Claude Code in Ihre Einstellungsdateien, und `/model` speichert den Wert als Ihren Standard für neue Sitzungen.

Wenn Sie `s` im `/model` Picker drücken, wechselt Claude Code das Modell, ohne es als Ihren Benutzer-Standard zu speichern. [Passen Sie die Anstrengungsstufe an](/docs/de/model-config#adjust-effort-level) sagt, welche `/effort` Picks Claude Code als Ihren Standard für das Modell speichert, das Sie verwenden, und welche nur auf die aktuelle Sitzung angewendet werden.

Um beispielsweise eine Sitzung auf Opus zu starten, ohne Ihren Standard zu ändern:

```bash theme={null}
claude --settings '{"model": "claude-opus-5-5"}'
```

<h3 id="when-edits-take-effect">
  Wenn Änderungen wirksam werden
</h3>

Claude Code überwacht Ihre Einstellungsdateien und lädt sie neu, wenn sie sich ändern, sodass es die meisten Änderungen auf die laufende Sitzung ohne Neustart anwendet, einschließlich Änderungen an `permissions`, `hooks` und Anmeldedaten-Helfern wie `apiKeyHelper`. Claude Code lädt auch eine Einstellungsdatei, die Sie mid-Sitzung erstellen, wenn ihr Ordner existierte, als die Sitzung begann. Für den `.claude/` Ordner des Projekts lädt es die Datei auch, wenn Sie den Ordner in der gleichen Sitzung erstellen.

Das Neuladen behandelt Benutzer-, Projekt-, lokale und verwaltete Einstellungen, und Claude Code führt den [`ConfigChange` Hook](/docs/de/hooks#configchange) für jede erkannte Einstellungsdatei-Änderung aus, nicht für verwaltete Einstellungen, die von MDM oder der claude.ai-Konsole ankommen. Verwaltete Einstellungen, die von MDM oder der claude.ai-Konsole ankommen, erreichen eine laufende Sitzung nach einem Zeitplan statt sofort; die [Bereitstellungstabelle](/docs/de/managed-settings#choose-a-delivery-mechanism) gibt ihn pro Quelle an.

Claude Code liest einige Schlüssel nur einmal, beim Sitzungsstart, sodass eine Änderung an einem von ihnen die laufende Sitzung nicht erreicht. Admin-seitige Schlüssel, die auch auf einen Neustart warten, wie `requiredMinimumVersion`, sind unter [wo und wann eine Richtlinie angewendet wird](/docs/de/managed-settings#where-and-when-a-policy-applies) aufgelistet. Die, die Sie am ehesten mid-Sitzung bearbeiten:

* [`model`](/docs/de/settings-reference#model): verwenden Sie [`/model`](/docs/de/model-config#setting-your-model), um mid-Sitzung zu wechseln. Jedes Modell hat seinen eigenen Prompt-Cache, sodass die erste Anfrage nach einem Wechsel das ganze Gespräch ungecacht erneut liest; siehe [Wechsel von Modellen](/docs/de/prompt-caching#switching-models)
* [`effortLevel`](/docs/de/settings-reference#effortlevel) und [`modelSettings`](/docs/de/settings-reference#modelsettings): verwenden Sie [`/effort`](/docs/de/model-config#adjust-effort-level), um Anstrengung mid-Sitzung zu ändern

<span id="verify-active-settings" />

<span id="check-what-loaded" />

<h3 id="confirm-what-loaded">
  Bestätigen Sie, was geladen wurde
</h3>

Führen Sie `/status` in Claude Code aus, um zu sehen, welche Einstellungsquellen aktiv sind. Die Registerkarte **Status** enthält eine Zeile `Setting sources`, die jede Einstellungsdatei auflistet, die Claude Code für die aktuelle Sitzung geladen hat, wie `User settings` oder `Project local settings`. Wenn [verwaltete Einstellungen](/docs/de/admin-setup#decide-how-settings-reach-devices) wirksam sind, zeigt der Eintrag für verwaltete Einstellungen in Klammern, wie sie Ihre Maschine erreicht haben.

Die Zeile bestätigt, welche Dateien Claude Code gelesen hat; sie zeigt nicht, welche Datei jeden Schlüssel bereitgestellt hat. Um Einträge aufzulisten, die Claude Code abgelehnt hat, führen Sie [`claude doctor`](/docs/de/debug-your-config) aus; für ein Modell, das Projekt- oder verwaltete Einstellungen setzen, nennt der Startup-Header die Datei, die es gesetzt hat. `/status` und `/config` öffnen den gleichen Dialog auf verschiedenen Registerkarten, und die Registerkarte **Config** ist keine Ansicht Ihrer `settings.json` Inhalte.

<h3 id="fix-a-broken-settings-file">
  Beheben Sie eine fehlerhafte Einstellungsdatei
</h3>

Wenn Sie JSON falsch tippen oder einen Schlüssel auf einen Wert setzen, den Claude Code nicht akzeptiert, teilt Claude Code Ihnen dies am Anfang einer interaktiven Sitzung mit. Was es zeigt, hängt davon ab, wie viel der Datei betroffen ist:

* **Einstellungsfehler**: eine Benutzer-, Projekt- oder lokale Datei hat ungültiges JSON oder einen Wert, den das Schema ablehnt. Am Anfang einer interaktiven Sitzung zeigt Claude Code einen Dialog, der Ihnen erlaubt, die Datei mit Claudes Hilfe zu beheben, zu beenden oder ohne die fehlerhaften Einstellungen fortzufahren.
* **Einstellungswarnung**: nur einzelne Einträge schlagen fehl, wie eine fehlerhafte Berechtigungsregel oder ein unbekannter Hook-Ereignisname. Claude Code überspringt diese Werte und behält den Rest der Datei wirksam.
* **Verwaltete Einstellungen**: Claude Code erzwingt weiterhin den Rest der Datei. [Ungültige Einträge in verwalteten Einstellungen](/docs/de/managed-settings#invalid-entries-in-managed-settings) sagt, was es ablegt und welche Schlüssel auf einen strengeren Wert zurückfallen, bis Sie sie beheben. Für ein verwaltetes Einstellungsdokument, das nicht gültiges JSON ist, siehe [Verwaltetes Einstellungsdokument konnte nicht analysiert werden](/docs/de/errors#managed-settings-document-could-not-be-parsed).
* **Konfigurationsfehler**: `~/.claude.json` kann nicht analysiert werden. Claude Code kopiert die fehlerhafte Datei zu `~/.claude/backups/.claude.json.corrupted.<timestamp>` und fragt, ob Sie beenden und sie von Hand beheben oder auf die Standardkonfiguration zurücksetzen möchten; ein `-p` Lauf druckt den Fehler und beendet. Um Ihren vorherigen Status wiederherzustellen, kopieren Sie eine der fünf neuesten `.claude.json.backup.<timestamp>` Dateien in `~/.claude/backups/` zurück, die Claude Code vor dem Schreiben der Datei speichert.

Nachdem Sie fortfahren, führen Sie `/status` aus, um die betroffenen Dateien zu sehen, und `claude doctor` für die Details jedes Fehlers.

Ein `-p` Lauf zeigt keinen Dialog. Es sei denn, [ein verwaltetes Einstellungsdokument kann nicht analysiert werden](/docs/de/errors#managed-settings-document-could-not-be-parsed), überspringt Claude Code die fehlerhafte Datei oder Werte und fährt mit dem Rest fort, sodass nach einem `-p` Lauf, der eine Einstellung ignoriert, `claude doctor` ausführen, um zu sehen, was es abgelegt hat.

<span id="how-scopes-interact" />

<span id="key-points-about-the-configuration-system" />

<span id="which-value-claude-code-uses" />

<span id="which-value-wins" />

<h2 id="settings-precedence">
  Einstellungspriorität
</h2>

Wenn der gleiche Schlüssel an mehr als einer Stelle erscheint, verwendet Claude Code den Wert aus der höchsten Ebene, die ihn setzt. Der Stapel unten zeigt die Ebenen, höchste oben; ein Schlüssel auf einer höheren Ebene überschreibt den gleichen Schlüssel überall darunter.

<SettingsPrecedence />

In Reihenfolge, höchste Priorität zuerst:

1. **Verwaltete Einstellungen**: Einstellungen, die Ihre Organisation bereitstellt, durch eine `managed-settings.json` Datei, eine MDM-Richtlinie oder [serververwaltete Einstellungen](/docs/de/server-managed-settings) von der claude.ai-Konsole. Nichts, das Sie setzen, überschreibt sie: ein Schlüssel, den Sie mit `--settings` übergeben, überschreibt nicht den gleichen verwalteten Schlüssel, und ein Flag wie `--model` wählt nur aus den Modellen, die Ihre Organisation erlaubt. Ein verwaltetes `model` setzt das Modell, mit dem jede Sitzung startet, und Sie können immer noch mit `/model` wechseln; die Sperre ist [`availableModels`](/docs/de/settings-reference#availablemodels), die `/model`, `--model` und den `model` Schlüssel in Ihren eigenen Dateien einschränkt. Wenn Ihre Organisation mehr als eine verwaltete Quelle bereitstellt, sagen die Regeln für [Priorität innerhalb der verwalteten Ebene](/docs/de/managed-settings#precedence-within-the-managed-tier), was Claude Code aus jeder liest.
2. **Befehlszeilenargumente**: Flags, die Sie übergeben, wenn Sie `claude` von einem Terminal aus starten, für eine Sitzung; siehe [Ändern Sie eine Einstellung für eine Sitzung](#change-a-setting-for-one-session). Claude Code führt JSON, das Sie mit `--settings <file-or-json>` übergeben, mit Ihren Einstellungsdateien nach den gleichen Regeln wie die anderen Ebenen zusammen: es nimmt einen Schlüssel, den Sie hier setzen, über den gleichen Schlüssel in lokalen, Projekt- oder Benutzereinstellungen, und behält den Wert der niedrigeren Ebene für einen Schlüssel, den Sie weglassen.
3. **Projekt-lokale Einstellungen** (`.claude/settings.local.json`): Ihre persönlichen Einstellungen für dieses Projekt.
4. **Gemeinsame Projekt-Einstellungen** (`.claude/settings.json`): Einstellungen, die Ihr Team in die Versionskontrolle eincheckt.
5. **Benutzer-Einstellungen** (`~/.claude/settings.json`): Ihre persönlichen Einstellungen für jedes Projekt.

Umgebungsvariablen sind keine Ebene in diesem Stapel. Wenn ein Verhalten sowohl eine Shell-Variable als auch einen Einstellungsschlüssel hat, wird entschieden, welche angewendet wird, pro Paar, nicht nach Ebene: `ANTHROPIC_MODEL`, das in Ihrer Shell exportiert wird, gilt über den `model` Schlüssel aus jeder Datei, während `ANTHROPIC_DEFAULT_MODEL` nur gilt, wenn keine Datei `model` setzt. Die [Umgebungsvariablen-Referenz](/docs/de/env-vars#precedence) sagt, welche Schlüssel ein Paar haben und welche Claude Code zuerst liest. Ein `env` Block in einer Einstellungsdatei ist ein gewöhnlicher Schlüssel und folgt den Ebenen oben.

Für ein paar sicherheitsempfindliche Schlüssel ehrt Claude Code einen strengeren Wert aus einer niedrigeren Ebene über einen verwalteten Wert; [Ausnahmen zur Priorität der verwalteten Einstellungen](#exceptions-to-managed-settings-precedence) listet sie auf.

<h3 id="lists-merge-instead-of-overriding">
  Listen werden zusammengeführt statt überschrieben
</h3>

Wenn Sie den gleichen Listen-Schlüssel, wie `permissions.allow`, in mehr als einer Datei setzen, kombiniert Claude Code die Listen statt eine zu wählen, sodass jede Datei Einträge hinzufügen kann, ohne die eines anderen zu entfernen. Vier Schlüssel, die Modell-Listen oder Pro-Modell-Einträge halten, folgen ihren eigenen Regeln:

* [`fallbackModel`](/docs/de/settings-reference#fallbackmodel) ist eine geordnete Kette, wo Position Bedeutung hat, sodass Claude Code den ganzen Wert aus der höchsten Prioritätsdatei nimmt, die ihn definiert.
* [`modelPicker`](/docs/de/settings-reference#modelpicker) hält eine geordnete Liste von Zeilen plus ein Replace-Flag, sodass Claude Code Zeilen aus zwei Quellen nie zusammenführt. Es nimmt den ganzen Wert aus dem höchsten von verwalteten Einstellungen, `--settings` und Benutzereinstellungen, die ihn definieren, und ignoriert den Schlüssel in Projekt- und lokalen Einstellungen. Erfordert Claude Code v2.1.242 oder später.
* [`availableModels`](/docs/de/settings-reference#availablemodels): wenn die verwalteten Einstellungen, die Claude Code anwendet, ihn definieren, wendet Claude Code diese Liste wie sie ist an und ignoriert Einträge, die Sie in Benutzer-, Projekt- oder lokalen Einstellungen hinzufügen, es sei denn, eine App, die Claude Code einbettet, liefert ihre eigene Modell-Liste; siehe [Ausnahmen zur Priorität der verwalteten Einstellungen](#exceptions-to-managed-settings-precedence). Über verwaltete Quellen wird die Liste auch nie zusammengeführt; [wie Claude Code verwaltete Quellen kombiniert](/docs/de/managed-settings#how-claude-code-combines-managed-sources) sagt, welche Quelle's Liste angewendet wird. Über nicht verwaltete Bereiche führt Claude Code die Arrays wie üblich zusammen.
* [`modelSettings`](/docs/de/settings-reference#modelsettings): Claude Code löst es ein Modell auf einmal auf, zusammen mit [`effortLevel`](/docs/de/settings-reference#effortlevel). Der `modelSettings` Eintrag sagt, welche Datei's Wert auf ein Modell angewendet wird.

<span id="examples" />

<h3 id="precedence-examples">
  Beispiele für Priorität
</h3>

Während Claude arbeitet, zeigt Claude Code einen einzeiligen Tipp unter dem Spinner, wie "Verwenden Sie /config, um Ihren Standard-Berechtigungsmodus zu ändern (einschließlich Plan Mode)". Angenommen, Sie möchten diese Tipps aus, sodass Sie [`spinnerTipsEnabled`](/docs/de/settings-reference#spinnertipsenabled) auf `false` in `~/.claude/settings.json` setzen. Jedes Szenario unten ist etwas, das sie wieder einschalten kann, und was Sie dagegen tun können.

<h4 id="team-settings-override-personal-settings">
  Team-Einstellungen überschreiben persönliche Einstellungen
</h4>

Die `.claude/settings.json` Ihres Teams setzt es auf `true`. Claude Code verwendet den Projektwert, weil gemeinsames Projekt über Benutzer sitzt, sodass Sie Tipps in diesem Projekt und nirgendwo sonst sehen.

Sie können Ihren Wert zurückbekommen: fügen Sie `"spinnerTipsEnabled": false` zu `.claude/settings.local.json` in diesem Projekt hinzu. Projekt-lokal sitzt über gemeinsames Projekt, sodass Ihre Sitzungen dort Tipps nicht mehr zeigen und die Sitzungen Ihrer Teamkollegen sich nicht ändern.

<h4 id="organization-settings-override-everything">
  Organisations-Einstellungen überschreiben alles
</h4>

Die verwalteten Einstellungen Ihrer Organisation setzen es auf `true`. Nichts, das Sie in Benutzer-, Projekt- oder lokale Einstellungen setzen, schaltet Tipps aus, und auch nicht `--settings`. Verwaltet ist die oberste Ebene.

Sie können Ihren Wert nicht zurückbekommen. Führen Sie `/status` aus, um zu sehen, welche verwaltete Quelle angewendet wird, und fragen Sie Ihren Administrator, ob die Richtlinie sich ändern sollte.

<h4 id="the-command-line-overrides-your-files-for-one-session">
  Die Befehlszeile überschreibt Ihre Dateien für eine Sitzung
</h4>

Sie haben die Sitzung mit `claude --settings '{"spinnerTipsEnabled": true}'` gestartet. Befehlszeile sitzt über jeder Datei außer verwaltet, sodass diese Sitzung Tipps zeigt, obwohl Ihre Dateien `false` sagen.

Sie bekommen Ihren Wert in der nächsten Sitzung zurück; `--settings` dauert eine Sitzung und schreibt nicht in eine Datei.

<h4 id="a-flag-or-environment-variable-sets-the-same-thing">
  Ein Flag oder eine Umgebungsvariable setzt das gleiche
</h4>

Einige Schlüssel haben ein Befehlszeilenflag oder eine Umgebungsvariable, die den Einstellungswert unabhängig davon überschreibt, welche Datei ihn setzt: `ANTHROPIC_MODEL` überschreibt die [`model`](/docs/de/settings-reference#model) Einstellung, und `--model` überschreibt beide für eine Sitzung.

Ob Sie Ihren Wert zurückbekommen, hängt vom Schlüssel ab: heben Sie die Variable auf oder lassen Sie das Flag fallen, und überprüfen Sie den Eintrag des Schlüssels auf der [Einstellungsreferenz](/docs/de/settings-reference) und die Zeile der Variable auf der [Umgebungsvariablen-Referenz](/docs/de/env-vars) für welche Claude Code verwendet.

<span id="keys-ignored-in-a-repository-file" />

<span id="keys-only-you-or-your-organization-can-set" />

<span id="common-cases" />

<span id="which-value-applies-in-common-situations" />

<h3 id="troubleshoot-a-setting-that-doesn’t-apply">
  Beheben Sie eine Einstellung, die nicht angewendet wird
</h3>

Wenn Sie einen Schlüssel setzen und Claude Code sich nicht so verhält, als hätten Sie, beginnen Sie mit `/status`, um zu sehen, welche Dateien es geladen hat, dann finden Sie Ihr Symptom unten. [Debuggen Sie Ihre Konfiguration](/docs/de/debug-your-config) behandelt die breiteren Überprüfungen, einschließlich eines sauberen Konfigurationstests.

<h4 id="a-value-you-set-is-ignored">
  Ein Wert, den Sie setzen, wird ignoriert
</h4>

Etwas anderes setzt den gleichen Schlüssel, die Datei kann diesen Wert nicht setzen, oder die Datei wurde nicht geladen:

* **Eine höhere Ebene setzt ihn.** Eine andere Einstellungsdatei, ein `--settings` Flag oder eine verwaltete Quelle setzt den Schlüssel über Ihrem; der [Stapel](#settings-precedence) sagt welche. Ein Flag oder eine Umgebungsvariable kann auch den Schlüssel auf eigene Faust überschreiben, entschieden Schlüssel für Schlüssel; der Eintrag des Schlüssels auf der [Einstellungsreferenz](/docs/de/settings-reference) sagt, welche Claude Code verwendet, und der [`env` Eintrag](/docs/de/settings-reference#env) behandelt einen verwalteten `env` Wert versus einen Shell-Export.
* **Ein Sicherheitsschlüssel behält seinen strengen Wert.** Für ein paar Schlüssel ehrt Claude Code den restriktiven Wert aus jeder Datei, sodass ein Projekt `true` für [`disableClaudeAiConnectors`](/docs/de/settings-reference#disableclaudeaiconnectors) bleibt an; siehe [Ausnahmen zur Priorität der verwalteten Einstellungen](#exceptions-to-managed-settings-precedence).
* **Die Datei kann diesen Wert nicht setzen.** [`permissions.defaultMode`](/docs/de/settings-reference#permissions-defaultmode) Werte `auto` und `bypassPermissions` wirken sich nicht von Projekt- oder lokalen Einstellungen aus; setzen Sie sie stattdessen in Benutzer- oder verwaltete Einstellungen, oder übergeben Sie `--permission-mode` für eine Sitzung. Vor v2.1.257 wirkte sich `bypassPermissions` von jeder Datei aus.

  Eine Telemetrie-Exportvariable in einem [`env`](/docs/de/settings-reference#env) Block wirkt sich auch nicht von Projekt- oder lokalen Einstellungen aus, außer ein paar Off-Werte. [Variablen, die Claude Code in `env` ignoriert](/docs/de/settings-reference#variables-claude-code-ignores-in-env) listet die Variablen und diese Werte auf.
* **Die Datei ist fehlerhaft.** Ungültiges JSON oder ein abgelehnter Wert lässt Claude Code die Datei oder den Eintrag überspringen; siehe [Beheben Sie eine fehlerhafte Einstellungsdatei](#fix-a-broken-settings-file).

<h4 id="a-change-you-made-in-claude-code-is-lost-in-new-sessions">
  Eine Änderung, die Sie in Claude Code gemacht haben, geht in neuen Sitzungen verloren
</h4>

Wenn Sie eine Wahl für neue Sitzungen von innen Claude Code speichern, wie ein Standardmodell mit `/model`, schreibt Claude Code es in Ihre Benutzereinstellungsdatei, `~/.claude/settings.json`. Wenn Sie nicht in diese Datei schreiben können, zum Beispiel weil ein anderes Tool sie generiert oder sie mit einer schreibgeschützten Kopie verlinkt, gilt die Änderung für die aktuelle Sitzung und ist in der nächsten weg. Setzen Sie den Schlüssel in das Tool, das die Datei generiert, oder ersetzen Sie die Datei mit einer, in die Sie schreiben können.

Wenn Sie in die Datei schreiben können und die Änderung bleibt immer noch nicht, überprüfen Sie, ob die Änderung [nur für eine Sitzung](#change-a-setting-for-one-session) war oder [eine höhere Ebene setzt den gleichen Schlüssel](#a-value-you-set-is-ignored). Für den `model` Schlüssel, [Eine neue Sitzung startet auf einem anderen Modell als Sie gewählt haben](/docs/de/model-config#a-new-session-starts-on-a-different-model-than-you-picked) listet mehr Ursachen auf.

<h4 id="a-managed-change-hasn’t-reached-you">
  Eine verwaltete Änderung hat Sie nicht erreicht
</h4>

Verwaltete Quellen erreichen eine laufende Sitzung nach dem Zeitplan in der [Bereitstellungstabelle](/docs/de/managed-settings#choose-a-delivery-mechanism), sodass starten Sie die Sitzung zuerst neu. Wenn `/status` dann eine andere Quelle nennt als die, die Ihr Administrator geändert hat, gilt eine höherrangige Quelle; [Wie Claude Code verwaltete Quellen kombiniert](/docs/de/managed-settings#how-claude-code-combines-managed-sources) gibt die Reihenfolge.

<h4 id="a-committed-key-doesn’t-reach-teammates">
  Ein eingecheckter Schlüssel erreicht Teamkollegen nicht
</h4>

Zwei Dinge halten einen Schlüssel in `.claude/settings.json` davon ab, für alle zu gelten, die ihn klonen:

* **Claude Code ignoriert den Schlüssel in einer Repository-Datei.** Suchen Sie nach `User, local, or managed`, `User or managed`, `Managed` oder `Global config` in der Spalte Scope des [Einstellungsindex](/docs/de/settings-reference#settings-index). Diese Schlüssel wirken sich nie von der gemeinsamen Datei aus, außer ein paar, die eine Repository-Datei immer noch ausschalten kann. Jeder dieser Einträge sagt so auf seiner Scope-Zeile. `Global config` Schlüssel gelten nur von `~/.claude.json`.

  Innerhalb des `env` Schlüssels gelten die Telemetrie-Exportvariablen auch nie von der gemeinsamen Datei, außer ein paar Off-Werte; siehe [Variablen, die Claude Code in `env` ignoriert](/docs/de/settings-reference#variables-claude-code-ignores-in-env).
* **Der Schlüssel wartet auf Vertrauen.** `permissions.allow` Regeln, `permissions.additionalDirectories`, `extraKnownMarketplaces` und die meisten [`env`](/docs/de/settings-reference#env) Werte gelten nur, nachdem jeder Teamkollege den [Ordner vertraut](/docs/de/permissions#project-allow-rules-and-workspace-trust). Bis dahin sehen sie immer noch Aufforderungen und bekommen keine Plugins von einem Marketplace, den die Datei erklärt. `deny` und `ask` Regeln gelten sofort.

<h4 id="permission-rules-combine-differently-than-you-expected">
  Berechtigungsregeln kombinieren sich anders als Sie erwartet
</h4>

* **Sie haben "Ja, und frag mich nicht mehr" auf einer Berechtigungsaufforderung gewählt, aber bekommen immer noch eine Aufforderung für das gleiche Tool.** Diese Wahl speicherte eine `allow` Regel in Ihrer lokalen Datei, und eine `allow` Regel dort übertrifft nicht eine `ask` Regel aus einer Projekt- oder verwalteten Datei; [wie Berechtigungsregeln kombinieren](/docs/de/permissions#settings-precedence) erklärt die Reihenfolge. In der VS Code-Erweiterung lässt die Genehmigungskarte Sie die Zieldatei wählen, einschließlich der gemeinsamen Datei des Projekts, was die Regel für alle ändert; in der CLI schreibt Claude Code nur in Ihre lokale Datei.
* **Die Allow-Regeln Ihrer Organisation gelten immer noch neben Ihren.** Das ist erwartet: Claude Code führt [`permissions.allow`](/docs/de/settings-reference#permissions-allow) über Bereiche zusammen, es sei denn, Ihre Organisation setzt [`allowManagedPermissionRulesOnly`](/docs/de/settings-reference#allowmanagedpermissionrulesonly).

<span id="security-keys-where-the-stricter-value-applies" />

<h3 id="exceptions-to-managed-settings-precedence">
  Ausnahmen zur Priorität der verwalteten Einstellungen
</h3>

Für ein paar sicherheitsempfindliche Schlüssel ehrt Claude Code einen restriktiven Wert aus einem Bereich, der ansonsten verwaltete Einstellungen nicht überschreiben könnte. Finden Sie den Schlüssel in dieser Tabelle, um zu sehen, welchen Wert er ehrt und von wo.

| Schlüssel                                                                       | Wert, den Claude Code ehrt                                                                                                        | Notizen                                                                                                                                                                              |
| :------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`disableClaudeAiConnectors`](/docs/de/settings-reference#disableclaudeaiconnectors) | `true` aus jedem Bereich                                                                                                          | Geehrt, auch wenn eine verwaltete Quelle `false` setzt                                                                                                                               |
| [`enableArtifact`](/docs/de/settings-reference#enableartifact)                       | `false` aus jedem Bereich und `disableArtifact: true` aus jedem Bereich                                                           | Geehrt, auch wenn eine verwaltete Quelle `true` setzt; nichts schaltet das [Artifact-Tool](/docs/de/artifacts#disable-artifacts) wieder ein. Erfordert Claude Code v2.1.242 oder später   |
| [`isolatePeerMachines`](/docs/de/settings-reference#isolatepeermachines)             | `true` aus jedem Bereich                                                                                                          | Geehrt, auch wenn eine verwaltete Quelle `false` setzt                                                                                                                               |
| [`remoteControlAtStartup`](/docs/de/settings-reference#remotecontrolatstartup)       | `false` aus `.claude/settings.json` oder `.claude/settings.local.json`                                                            | Geehrt, auch wenn eine verwaltete Quelle `true` setzt; ein Projekt- oder lokales `true` wird ignoriert                                                                               |
| [`crossSessionInbound`](/docs/de/settings-reference#crosssessioninbound)             | Ein strengerer Wert aus `.claude/settings.json` oder `.claude/settings.local.json`, auf der `accept` \< `hold` \< `refuse` Leiter | Geehrt über verwaltete, `--settings` und Benutzer-Werte; ein Projekt- oder lokaler Wert, der nicht strenger ist, wird ignoriert                                                      |
| [`useAutoModeDuringPlan`](/docs/de/settings-reference#useautomodeduringplan)         | `false` aus jeder verwalteten Quelle, `--settings`, `~/.claude/settings.json` oder `.claude/settings.local.json`                  | Geehrt, auch wenn die gewinnende verwaltete Quelle `true` setzt; ein `false` in `.claude/settings.json` wird ignoriert                                                               |
| [`syncClaudeAiSkills`](/docs/de/settings-reference#syncclaudeaiskills)               | `false` aus jeder verwalteten Quelle, `--settings`, `~/.claude/settings.json` oder `.claude/settings.local.json`                  | Geehrt, auch wenn die gewinnende verwaltete Quelle `true` setzt; ein `false` in `.claude/settings.json` wird ignoriert                                                               |
| [`syncClaudeAiPlugins`](/docs/de/settings-reference#syncclaudeaiplugins)             | `false` aus jeder verwalteten Quelle, `--settings`, `~/.claude/settings.json` oder `.claude/settings.local.json`                  | Geehrt, auch wenn die gewinnende verwaltete Quelle `true` setzt; ein `false` in `.claude/settings.json` wird ignoriert                                                               |
| [`maxEffortLevel`](/docs/de/settings-reference#maxeffortlevel)                       | Eine niedrigere Obergrenze aus jedem Bereich, einschließlich `--settings`                                                         | Geehrt, auch wenn die verwalteten Einstellungen, die Claude Code anwendet, eine höhere Obergrenze setzen; die niedrigste Obergrenze gilt. Erfordert Claude Code v2.1.267 oder später |

Eine App, die Claude Code in sich selbst ausführt und [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/de/env-vars) setzt, ist auch eine Ausnahme. Claude Code nimmt die Modell-Konfiguration dieser App über die `model`, `fallbackModel`, `modelPicker` und `modelOverrides` Schlüssel aus jeder verwalteten Quelle, und über die Modell-Auswahl-Variablen in einem verwalteten `env` Block, wie `ANTHROPIC_MODEL` und die `ANTHROPIC_DEFAULT_*_MODEL` Familie. Claude Code behält eine verwaltete [`availableModels`](/docs/de/settings-reference#availablemodels) Allowlist in Kraft, es sei denn, die App liefert ihre eigene.

<h2 id="settings-in-cloud-sessions">
  Einstellungen in Cloud-Sitzungen
</h2>

Eine [Cloud-Sitzung](/docs/de/claude-code-on-the-web) läuft in einer [Cloud-Umgebung](/docs/de/cloud-environments) auf einem frischen Klon Ihres Repositorys, nicht auf Ihrer Maschine. Das ändert, welche Einstellungen es erreichen:

* **Gemeinsame Projekt-Einstellungen** (`.claude/settings.json`): werden in einer Sitzung mit einem Repository gelesen, da die Datei Teil des Klons ist und die Sitzung darin startet. Committen Sie eine Einstellung dort, um sie in diesen Sitzungen anzuwenden. Eine Sitzung mit mehreren Repositorys startet über den Klonen und liest aus jeder `.claude/settings.json` des Repositorys nur die Schlüssel `enabledPlugins` und `extraKnownMarketplaces`, nicht Berechtigungsregeln, Hooks, `env` oder andere Schlüssel. Die Marktplätze und Plugins, die diese beiden Schlüssel deklarieren, [werden immer noch nicht in einer Cloud-Sitzung geladen](/docs/de/cloud-environments#what-carries-over-from-your-setup).
* **Benutzer- und Projekt-lokale Einstellungen** (`~/.claude/settings.json` und `.claude/settings.local.json`): nicht gelesen. Beide bleiben auf Ihrer Maschine, und die lokale Datei ist nicht im Klon.
* **Verwaltete Einstellungen**: nur [serververwaltete Einstellungen](/docs/de/server-managed-settings) erreichen eine Cloud-Sitzung; eine `managed-settings.json` Datei oder MDM-Profil auf Ihrem Gerät nicht. Eine [selbstgehostete Umgebung](/docs/de/self-hosted-environments) liest auch die verwaltete Einstellungsdatei in ihrem Runner-Image. [Wie Claude Code verwaltete Quellen kombiniert](/docs/de/managed-settings#how-claude-code-combines-managed-sources) sagt, wann diese Datei angewendet wird.
* **`/config`**: im Browser unter claude.ai/code öffnet die Claude Code-Sektion Ihrer claude.ai-Einstellungen statt einen Wert zu ändern. Um eine Einstellung für eine Cloud-Sitzung zu ändern, setzen Sie eine [Umgebungsvariable](/docs/de/cloud-environments#set-environment-variables) auf der Umgebung, oder committen Sie in einer Sitzung mit einem Repository den Schlüssel zu der `.claude/settings.json` dieses Repositorys.

[Was von Ihrem Setup überträgt](/docs/de/cloud-environments#what-carries-over-from-your-setup) listet den Rest auf: `CLAUDE.md`, Skills, MCP-Server, Plugins und Anmeldedaten.

<h2 id="what’s-next">
  Was kommt als nächstes
</h2>

* [Alle Einstellungen](/docs/de/settings-reference): jeder Schlüssel, mit wo Sie ihn setzen und einem Beispiel
* [Beispiel-Einstellungsdateien](/docs/de/settings-example): eine persönliche Datei, eine Team-Datei und eine verwaltete Datei einer Organisation
* [Berechtigungen konfigurieren](/docs/de/permissions): allow, ask und deny Regeln, und was Claude Code ohne Nachfrage ausführt
* [Umgebungsvariablen](/docs/de/env-vars): die Variablen, die Claude Code liest und der `env` Block
* [Debuggen Sie Ihre Konfiguration](/docs/de/debug-your-config): wenn eine Einstellung nicht angewendet wird
* [Claude-Verzeichnis-Referenz](/docs/de/claude-directory): jede Datei, die Claude Code liest, einschließlich Subagents, MCP-Server, Plugins und `CLAUDE.md`
