> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Файлы параметров и приоритет

> Измените параметры Claude Code, выберите область, к которой принадлежит ключ, проверьте изменение и узнайте, какое значение Claude Code использует, когда ключ установлен в нескольких местах.

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

Параметры — это ключи JSON, которые изменяют поведение Claude Code: какую модель он запускает, что он может запустить без запроса, какие файлы он не может читать, как он выглядит в вашем терминале и что ваша организация требует.

<Tip>
  Чтобы найти конкретный ключ, перейдите на [Все параметры](/docs/ru/settings-reference), где перечислены все ключи с файлом, в котором вы их устанавливаете, значением по умолчанию и примером.
</Tip>

Claude Code читает параметры из файлов параметров JSON, таких как `~/.claude/settings.json`. Он ищет их в нескольких местах, и [файл, из которого он читает параметр, определяет, к кому применяется параметр](#settings-files-and-who-they-affect). На этой странице рассматриваются эти файлы: в какой из них поместить параметр, как изменить параметр и подтвердить его применение, и какое значение Claude Code использует, когда один и тот же ключ установлен в более чем одном файле. [Настройка разрешений](/docs/ru/permissions) охватывает то, что Claude Code может запустить без запроса, и как писать правила `allow`, `ask` и `deny`.

<Note>
  На этой странице рассматривается Claude Code, работающий на вашей машине: терминал, расширения [VS Code](/docs/ru/vs-code) и [JetBrains](/docs/ru/jetbrains), а также [приложение desktop](/docs/ru/desktop), которые все читают одни и те же файлы параметров. Облачный сеанс на [Claude Code в веб-версии](/docs/ru/claude-code-on-the-web) работает на другой машине и читает только некоторые из них; см. [Параметры в облачных сеансах](#settings-in-cloud-sessions).
</Note>

<span id="settings-files" />

<span id="configuration-scopes" />

<span id="available-scopes" />

<span id="when-to-use-each-scope" />

<span id="what-uses-scopes" />

<span id="subagent-configuration" />

<span id="where-settings-live" />

<h2 id="settings-files-and-who-they-affect">
  Файлы параметров и кого они затрагивают
</h2>

Claude Code читает параметры из четырёх файлов, и организация также может доставлять управляемые параметры из консоли claude.ai. Каждый источник имеет область действия: набор людей и проектов, к которым применяется параметр, сохранённый в нём, будь то только вы, все в проекте или все в вашей организации.

| Область действия | Файл                                                                                               | Кого это затрагивает                                                                                                                                                                                            | Используйте для                                                                                      |
| :--------------- | :------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------- |
| Пользователь     | `~/.claude/settings.json`                                                                          | Вас, в каждом проекте на этой машине                                                                                                                                                                            | Личные предпочтения: тема, режим редактора, модель по умолчанию, ваши собственные правила разрешений |
| Общий проект     | `.claude/settings.json`                                                                            | Всех, кто работает в папке, которая его содержит. В репозитории git зафиксируйте его, чтобы товарищи по команде его получили                                                                                    | Разрешения команды, hooks, plugins и переменные окружения, которые нужны проекту                     |
| Локальный проект | `.claude/settings.local.json`                                                                      | Вас, только в этом одном проекте. Claude Code исключает его из git при создании файла; если вы создадите его вручную, добавьте его в `.gitignore` сами                                                          | Личные переопределения для одного проекта и тестирование перед тем, как вы поделитесь                |
| Управляемый      | `managed-settings.json` и другие [управляемые источники](/docs/ru/managed-settings#delivery-mechanisms) | Всех, кому ваша организация его развёртывает; ничто из того, что вы установили, его не переопределяет, кроме нескольких [исключений, чувствительных к безопасности](#exceptions-to-managed-settings-precedence) | Требования политики безопасности и соответствия                                                      |

В столбце File `~/.claude` — это папка `.claude` в вашем домашнем каталоге, а простой `.claude` — это папка `.claude` внутри вашего проекта.

<span id="where-each-file-applies" />

<span id="compare-what-each-file-reaches" />

<h3 id="compare-the-scope-of-each-settings-file">
  Сравните область действия каждого файла параметров
</h3>

Предположим, у вас есть три проекта на вашей машине: `website/`, `api/` и `acme-app/`, товарищ по команде имеет свой собственный клон `acme-app/`, и вы запускаете [облачный сеанс](#settings-in-cloud-sessions) на `acme-app/`.

На графике ниже показано, в каких из этих папок применяется параметр при запуске Claude Code из них. Нажмите на файл параметров, чтобы увидеть папки, которые он охватывает.

<SettingsScope />

* **`~/.claude/settings.json`**: каждый проект на вашей машине, и ничего на машине вашего товарища или в облачном сеансе
* **`acme-app/.claude/settings.json`**: ваш `acme-app/`. Он охватывает клон вашего товарища и облачный сеанс только если вы зафиксируете файл в системе контроля версий; до этого это файл на вашем диске, как и любой другой, и никто другой его не имеет
* **`acme-app/.claude/settings.local.json`**: только ваш `acme-app/`. Claude Code добавляет его в ваши глобальные исключения git при первом написании файла, поэтому он остаётся вне ваших коммитов; если вы создадите файл вручную, [добавьте его в `.gitignore` сами](#keep-personal-settings-out-of-a-repository)
* **Управляемые параметры**, будь то файл `managed-settings.json`, политика MDM или [управляемые сервером параметры](/docs/ru/server-managed-settings) из консоли claude.ai: каждый проект на каждой машине, на которую ваша организация его развёртывает, или на которую вы входите с учётной записью вашей организации. Только управляемые сервером параметры охватывают облачный сеанс

<span id="which-files-you-have" />

<h3 id="find-or-create-your-settings-files">
  Найдите или создайте ваши файлы параметров
</h3>

Установка Claude Code не создаёт никакой файл параметров. Если на вашей машине или в проекте уже есть один, он пришёл из одного из этих источников:

* **Управляемый**: ваша организация его развёртывает. Вы его не создаёте и не редактируете.
* **Общий проект**: проект, который уже использует Claude Code, может иметь один зафиксированный. Если нет, создайте его в `.claude/settings.json` в папке проекта.
* **Пользователь** и **Локальный проект**: создайте их сами или позвольте Claude Code их создать. Он записывает `~/.claude/settings.json` при первом изменении опции в меню `/config`, которую он сохраняет в пользовательские параметры, такой как тема, и `.claude/settings.local.json` при первом предоставлении постоянного одобрения на запрос разрешения, такой как "Да, и больше не спрашивайте" для команды Bash. Несколько опций `/config`, включая **Show tips**, сохраняются в `.claude/settings.local.json` вместо файла пользователя.

<Info>
  На Windows `~/.claude` означает `%USERPROFILE%\.claude`. Чтобы хранить файлы домашнего каталога в другом месте, установите [`CLAUDE_CONFIG_DIR`](/docs/ru/env-vars); Claude Code затем сохраняет ваши параметры, историю сеансов и plugins там вместо этого.
</Info>

Claude Code также хранит пятый файл, [`~/.claude.json`](/docs/ru/claude-directory#ce-claude-json), который он записывает для себя; вам не нужно его редактировать. Он содержит вашу сессию входа, конфигурации [MCP server](/docs/ru/mcp), состояние для каждого проекта, такое как решения о доверии, и [глобальные ключи конфигурации](/docs/ru/settings-reference#global-config-settings), которые `/config` записывает для вас.

<h3 id="share-settings-with-your-team">
  Поделитесь параметрами с вашей командой
</h3>

Зафиксируйте `.claude/settings.json`, чтобы все, кто клонирует репозиторий, получили одинаковые разрешения, hooks и plugins. Каждый товарищ по команде всё ещё может переопределить его для себя в своём собственном `.claude/settings.local.json`, поэтому личные исключения не нуждаются в коммите. Для полного файла команды см. [общие параметры команды](/docs/ru/settings-example#a-teams-shared-settings).

Некоторые из того, что вы зафиксируете, ждут, пока каждый товарищ по команде [доверит папку](/docs/ru/permissions#project-allow-rules-and-workspace-trust), и несколько ключей никогда не вступают в силу из файла репозитория; [Troubleshoot a setting that doesn't apply](#common-cases) охватывает оба случая.

<span id="local-settings-file" />

<span id="where-claude-code-saves-the-project-local-file" />

<span id="the-project-local-file" />

<span id="keep-personal-settings-out-of-the-repository" />

<h3 id="keep-personal-settings-out-of-a-repository">
  Держите личные параметры вне репозитория
</h3>

Чтобы изменить параметр для себя в одном проекте без изменения его для товарищей по команде, сохраните его в `.claude/settings.local.json` внутри проекта. Claude Code применяет этот файл поверх зафиксированного `.claude/settings.json`, поэтому если файл вашей команды устанавливает `"model": "claude-sonnet-5"` и вы хотите Opus, поместите `"model": "claude-opus-5-5"` в ваш локальный файл и только ваши сеансы изменятся.

Claude Code также записывает в этот файл, держит его вне ваших коммитов и применяет его правила разрешений без шага доверия:

* **Claude Code также его записывает.** Когда Claude просит разрешение запустить команду Bash и вы выбираете "Да, и больше не спрашивайте", Claude Code сохраняет это [одобрение разрешения](/docs/ru/permissions#permission-system) здесь как правило `allow`.
* **Вам не нужно его gitignore самостоятельно, если только вы не создали его вручную.** При первом написании файла Claude Code в репозитории git, который его ещё не игнорирует, он добавляет `**/.claude/settings.local.json` в ваш глобальный файл исключений git, поэтому файл остаётся вне ваших коммитов в каждом репозитории. Этот файл — это `core.excludesFile`, когда ваша глобальная конфигурация git устанавливает его на абсолютный путь или путь с префиксом `~`; в противном случае это `$XDG_CONFIG_HOME/git/ignore`, или `~/.config/git/ignore`, когда `XDG_CONFIG_HOME` не установлен. Если вы создали файл вручную и Claude Code ещё не писал в него, добавьте его в `.gitignore` сами.
* **Его правила allow не ждут доверия, пока файл остаётся неотслеживаемым.** Поскольку файл принадлежит вам, а не репозиторию, Claude Code применяет его правила `allow` без шага [workspace trust](/docs/ru/permissions#project-allow-rules-and-workspace-trust), который он требует для зафиксированного файла. Если файл отслеживается git, шаг доверия применяется к нему также; см. [When your local settings file needs trust](/docs/ru/permissions#when-your-local-settings-file-needs-trust).

<span id="where-claude-code-looks-for-each-file" />

<span id="how-claude-code-keeps-the-local-file-out-of-git" />

<span id="local-allow-rules-dont-wait-for-workspace-trust" />

<h4 id="where-claude-code-keeps-the-local-file-in-a-git-repository">
  Где Claude Code хранит локальный файл в репозитории git
</h4>

Когда Claude просит разрешение запустить команду Bash и вы выбираете "Да, и больше не спрашивайте", Claude Code сохраняет это одобрение как правило `allow` в `.claude/settings.local.json`. Если вы запускаете Claude Code в подкаталоге репозитория git, он читает и записывает этот файл в корень репозитория и применяет одобрение по всему репозиторию. В [worktree](/docs/ru/worktrees) он использует файл в корне основного checkout.

Два правила уточняют расположение корня:

* **Когда файл остаётся с `.claude/settings.json` вместо этого**: вне репозитория git, когда корень репозитория — это ваш домашний каталог, на Windows или когда корень репозитория или его запись `.git` или `.claude` не принадлежит вашему пользователю.
* **Пути в файле не привязаны к корню репозитория**: правило разрешения, которое начинается с `/` или относительный путь sandbox [привязывается к основному рабочему каталогу сеанса](/docs/ru/permissions#read-and-edit) вместо этого.

До v2.1.211 Claude Code хранил файл в начальном каталоге. Он всё ещё читает файл, который более ранняя версия оставила там рядом с файлом корня; где оба устанавливают один и тот же ключ, применяется значение корня, и правила разрешений из обоих файлов применяются. Помощник Agent SDK [`resolveSettings()`](/docs/ru/agent-sdk/typescript#resolvesettings) всегда читает файл из начального каталога.

Claude Code читает общий `.claude/settings.json` из [основного рабочего каталога](/docs/ru/permissions#working-directories) сеанса, поэтому для использования файла, зафиксированного в корне репозитория, запустите Claude Code там. После того как вы [переместите сеанс с `/cd`](/docs/ru/permissions#move-the-session-to-another-directory), Claude Code читает оба файла проекта из нового каталога вместо этого, размещая локальный файл по тем же правилам. Чтение их из каталога, в который вы переместились, требует Claude Code v2.1.246 или позже.

<span id="managed-settings-delivery" />

<span id="precedence-within-the-managed-tier" />

<span id="parent-settings-from-embedding-hosts" />

<span id="enforce-settings-for-an-organization" />

<span id="settings-your-organization-manages" />

<h3 id="check-what-your-organization-enforces">
  Проверьте, что ваша организация требует
</h3>

Если ваша организация управляет Claude Code, некоторые параметры решены за вас и ничто из того, что вы поместили в ваши собственные файлы, их не изменяет. Чтобы увидеть, какие, запустите `/status`: строка `Setting sources` называет управляемый источник, который применяется к вам. Управляемые параметры применяются везде, где Claude Code работает на этой машине; [What a developer can change](/docs/ru/managed-settings#what-a-developer-can-change) охватывает права локального администратора и инструменты, отличные от Claude Code.

Управляемые параметры достигают вас через [механизмы доставки](/docs/ru/managed-settings#delivery-mechanisms) на странице управляемых параметров, чаще всего:

* [Server-managed settings](/docs/ru/server-managed-settings), которые Claude Code получает из консоли администратора claude.ai или самостоятельно размещённого [Claude apps gateway](/docs/ru/claude-apps-gateway)
* Политики MDM или уровня ОС и файлы `managed-settings.json` в системном каталоге
* Хост встраивания, такой как Claude Desktop, через опцию SDK `managedSettings`; см. [Control policy from an embedding host](/docs/ru/managed-settings#parent-settings-from-embedding-hosts)

В сеансе [Cowork](https://claude.com/docs/cowork/overview), который работает на вашей машине в приложении Claude Desktop, Claude Code не получает управляемые сервером параметры из консоли администратора claude.ai и читает политику, развёрнутую на вашем устройстве, если конфигурация Claude Desktop вашей организации не устанавливает `requireCoworkFullVmSandbox`. [Where and when a policy applies](/docs/ru/managed-settings#where-and-when-a-policy-applies) охватывает Cowork и облачные сеансы.

Если вы администратор, [Set up Claude Code for your organization](/docs/ru/admin-setup) проходит через выбор того, что требуется применить, и [Deploy managed settings](/docs/ru/managed-settings) охватывает доставку и как подтвердить, что политика действует.

<h2 id="change-a-setting">
  Измените параметр
</h2>

Вы можете изменить параметр из меню `/config`, редактируя файл параметров или для одного сеанса из командной строки.

<span id="system-prompt" />

Системный запрос Claude Code не опубликован. Чтобы дать Claude постоянные инструкции, используйте файлы [`CLAUDE.md`](/docs/ru/memory) или флаг `--append-system-prompt`.

<h3 id="use-the-/config-menu">
  Используйте меню /config
</h3>

Запустите `/config` внутри Claude Code и откройте вкладку **Config**. Она перечисляет короткий набор личных опций, таких как тема, режим редактора и подробный вывод, не каждый ключ параметров. Выберите опцию для ее изменения; Claude Code сохраняет ее для вас:

* **Большинство опций**: `~/.claude/settings.json`
* **Несколько опций, таких как Show tips**: `.claude/settings.local.json`
* **[Глобальные опции конфигурации](/docs/ru/settings-reference#global-config-settings)**: `~/.claude.json`

Чтобы установить одну опцию без меню, передайте `key=value`, такой как `/config verbose=true`.

<Note>
  `/config` является частью интерфейса терминала. Панель чата [VS Code](/docs/ru/vs-code) и [приложение desktop](/docs/ru/desktop) его не открывают; измените параметры там, отредактировав файл параметров или через собственные параметры этих приложений.
</Note>

<h3 id="edit-a-settings-file">
  Отредактируйте файл параметров
</h3>

Откройте файл параметров для области, которую вы хотите, в вашем редакторе и добавьте или измените ключ. Файлы параметров — это строгий JSON: комментарий `//` или запятая в конце — это синтаксическая ошибка, и Claude Code сообщает файл как [Settings Error](#fix-a-broken-settings-file) при следующем запуске. Например, чтобы позволить Claude Code запускать ваши команды lint и test без запроса и остановить его чтение файлов `.env`, добавьте это в `~/.claude/settings.json`:

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

Каждая запись под `permissions` — это правило, которое называет инструмент и что он может делать; [Настройка разрешений](/docs/ru/permissions) объясняет синтаксис. Строка `$schema` указывает на [опубликованную JSON схему](https://json.schemastore.org/claude-code-settings.json) для параметров Claude Code, которая дает вам автодополнение и встроенную валидацию в VS Code, Cursor и любом другом редакторе, поддерживающем JSON схему. Схема может отставать от самых новых выпусков CLI, поэтому предупреждение валидации на недавно задокументированном ключе не означает, что ваша конфигурация недействительна.

После сохранения запустите `/status` внутри Claude Code, чтобы подтвердить, что файл загружен; [Подтвердите, что загружено](#check-what-loaded) говорит, что показывает строка `Setting sources` и как сообщается о сломанном файле.

Для полного личного файла, файла команды и файла организации, каждый показан с комментарием на каждом ключе, который он устанавливает, см. [примеры файлов параметров](/docs/ru/settings-example).

<span id="pass-settings-for-one-session" />

<h3 id="change-a-setting-for-one-session">
  Измените параметр для одного сеанса
</h3>

Чтобы попробовать значение без сохранения, установите его при запуске Claude Code. Значение применяется к этому сеансу, и ваши файлы параметров остаются такими, какими они были. У вас есть три способа это сделать:

* **`--settings`**: передайте ключ как JSON, встроенный или как путь к файлу. Claude Code применяет его выше ваших файлов пользователя, проекта и локальных и ниже управляемых параметров. Он может установить любой ключ, который может установить ваш файл параметров пользователя; он не может установить ключи `Managed` или `Global config`.
* **Флаг для этого ключа**: некоторые ключи имеют свой собственный флаг, такой как `--model` для `model` и `--effort` для `effortLevel` и `modelSettings`.
* **Переменная окружения**: экспортируйте переменную ключа перед запуском `claude`, такую как `ANTHROPIC_MODEL` для `model`.

Каждая запись ключа на [справочнике параметров](/docs/ru/settings-reference) перечисляет его переопределения для каждого сеанса и какой имеет приоритет, поэтому проверьте запись для ключа, который вы хотите изменить.

Команды, которые вы запускаете внутри сеанса, в основном сохраняют ваш выбор: когда вы изменяете параметр в `/config`, Claude Code записывает его в ваши файлы параметров, и `/model` сохраняет значение как ваше значение по умолчанию для новых сеансов.

Если вы нажмете `s` в средстве выбора `/model`, Claude Code переключает модель без сохранения ее как вашего пользовательского значения по умолчанию. [Отрегулируйте уровень усилий](/docs/ru/model-config#adjust-effort-level) говорит, какие выборы `/effort` Claude Code сохраняет как ваше значение по умолчанию для модели, которую вы используете, и какие применяются только к текущему сеансу.

Например, чтобы запустить один сеанс на Opus без изменения вашего значения по умолчанию:

```bash theme={null}
claude --settings '{"model": "claude-opus-5-5"}'
```

<h3 id="when-edits-take-effect">
  Когда изменения вступают в силу
</h3>

Claude Code отслеживает ваши файлы параметров и перезагружает их при изменении, поэтому он применяет большинство изменений к работающему сеансу без перезагрузки, включая изменения `permissions`, `hooks` и помощников учетных данных, таких как `apiKeyHelper`. Claude Code также загружает файл параметров, который вы создаете в середине сеанса, если его папка существовала при запуске сеанса. Для папки `.claude/` проекта он загружает файл даже когда вы создаете папку в том же сеансе.

Перезагрузка охватывает параметры пользователя, проекта, локальные и управляемые параметры, и Claude Code запускает hook [`ConfigChange`](/docs/ru/hooks#configchange) для каждого изменения файла параметров, которое он обнаруживает, не для управляемых параметров, которые поступают из MDM или консоли claude.ai. Управляемые параметры, которые поступают через MDM или из консоли claude.ai, достигают работающего сеанса по расписанию, а не при сохранении; [таблица доставки](/docs/ru/managed-settings#choose-a-delivery-mechanism) дает это для каждого источника.

Claude Code читает некоторые ключи только один раз при запуске сеанса, поэтому изменение одного из них не достигает работающего сеанса. Ключи на стороне администратора, которые также ждут перезагрузки, такие как `requiredMinimumVersion`, перечислены в [где и когда применяется политика](/docs/ru/managed-settings#where-and-when-a-policy-applies). Те, которые вы, скорее всего, будете редактировать в середине сеанса:

* [`model`](/docs/ru/settings-reference#model): используйте [`/model`](/docs/ru/model-config#setting-your-model) для переключения в середине сеанса. Каждая модель имеет свой собственный кэш подсказок, поэтому первый запрос после переключения повторно читает весь разговор без кэша; см. [Переключение моделей](/docs/ru/prompt-caching#switching-models)
* [`effortLevel`](/docs/ru/settings-reference#effortlevel) и [`modelSettings`](/docs/ru/settings-reference#modelsettings): используйте [`/effort`](/docs/ru/model-config#adjust-effort-level) для изменения усилий в середине сеанса

<span id="verify-active-settings" />

<span id="check-what-loaded" />

<h3 id="confirm-what-loaded">
  Подтвердите, что загружено
</h3>

Запустите `/status` внутри Claude Code, чтобы увидеть, какие источники параметров активны. Вкладка **Status** включает строку `Setting sources`, которая перечисляет каждый файл параметров, который Claude Code загрузил для текущего сеанса, такой как `User settings` или `Project local settings`. Когда [управляемые параметры](/docs/ru/admin-setup#decide-how-settings-reach-devices) действуют, запись управляемых параметров показывает в скобках, как они достигли вашей машины.

Строка подтверждает, какие файлы Claude Code прочитал; она не показывает, какой файл предоставил каждый ключ. Чтобы перечислить записи, которые Claude Code отклонил, запустите [`claude doctor`](/docs/ru/debug-your-config); для модели, которую установили параметры проекта или управляемые параметры, заголовок запуска называет файл, который ее установил. `/status` и `/config` открывают один и тот же диалог на разных вкладках, и вкладка **Config** не является представлением содержимого вашего `settings.json`.

<h3 id="fix-a-broken-settings-file">
  Исправьте сломанный файл параметров
</h3>

Если вы неправильно напечатаете JSON или установите ключ на значение, которое Claude Code не принимает, Claude Code сообщит вам при запуске интерактивного сеанса. То, что он показывает, зависит от того, сколько файла затронуто:

* **Settings Error**: файл пользователя, проекта или локальный имеет недействительный JSON или значение, которое схема отклоняет. При запуске интерактивного сеанса Claude Code показывает диалог, который позволяет вам исправить файл с помощью Claude, выйти или продолжить без сломанных параметров.
* **Settings Warning**: только отдельные записи не удаются, такие как неправильное правило разрешения или неизвестное имя события hook. Claude Code пропускает эти значения и держит остальную часть файла в силе.
* **Managed settings**: Claude Code продолжает применять остальную часть файла. [Недействительные записи в управляемых параметрах](/docs/ru/managed-settings#invalid-entries-in-managed-settings) говорит, что он отбрасывает и какие ключи возвращаются к более строгому значению, пока вы их не исправите. Для документа управляемых параметров, который не является действительным JSON, см. [Документ управляемых параметров не может быть разобран](/docs/ru/errors#managed-settings-document-could-not-be-parsed).
* **Configuration error**: `~/.claude.json` не может быть разобран. Claude Code копирует сломанный файл в `~/.claude/backups/.claude.json.corrupted.<timestamp>` и спрашивает, выйти и исправить его вручную или сбросить на конфигурацию по умолчанию; запуск `-p` выводит ошибку и выходит. Чтобы восстановить ваше предыдущее состояние, скопируйте обратно один из пяти самых последних файлов `.claude.json.backup.<timestamp>` в `~/.claude/backups/`, которые Claude Code сохраняет перед записью файла.

После продолжения запустите `/status`, чтобы увидеть затронутые файлы, и `claude doctor` для деталей каждой ошибки.

Запуск `-p` не показывает диалог. Если только [документ управляемых параметров не может быть разобран](/docs/ru/errors#managed-settings-document-could-not-be-parsed), Claude Code пропускает сломанный файл или значения и продолжает с остальным, поэтому после запуска `-p`, который игнорирует параметр, запустите `claude doctor`, чтобы увидеть, что он отбросил.

<span id="how-scopes-interact" />

<span id="key-points-about-the-configuration-system" />

<span id="which-value-claude-code-uses" />

<span id="which-value-wins" />

<h2 id="settings-precedence">
  Приоритет параметров
</h2>

Когда один и тот же ключ появляется в нескольких местах, Claude Code использует значение из наивысшего уровня, который его устанавливает. Стек ниже показывает уровни, наивысший вверху; ключ на более высоком уровне переопределяет тот же ключ где-либо ниже.

<SettingsPrecedence />

По порядку, наивысший приоритет первым:

1. **Управляемые параметры**: параметры, которые ваша организация развертывает через файл `managed-settings.json`, политику MDM или [параметры, управляемые сервером](/docs/ru/server-managed-settings) из консоли claude.ai. Ничто из того, что вы устанавливаете, их не переопределяет: ключ, который вы передаете с `--settings`, не переопределяет тот же управляемый ключ, и флаг, такой как `--model`, выбирает только из моделей, которые разрешает ваша организация. Управляемый `model` устанавливает модель, с которой начинается каждая сессия, и вы все еще можете переключаться с помощью `/model`; блокировка — это [`availableModels`](/docs/ru/settings-reference#availablemodels), которая ограничивает `/model`, `--model` и ключ `model` в ваших собственных файлах. Когда ваша организация предоставляет более одного управляемого источника, правила для [приоритета в пределах управляемого уровня](/docs/ru/managed-settings#precedence-within-the-managed-tier) указывают, что Claude Code читает из каждого.
2. **Аргументы командной строки**: флаги, которые вы передаете при запуске `claude` из терминала, для одной сессии; см. [Изменить параметр для одной сессии](#change-a-setting-for-one-session). Claude Code объединяет JSON, который вы передаете с `--settings <file-or-json>`, с вашими файлами параметров по тем же правилам, что и другие уровни: он берет ключ, который вы устанавливаете здесь, над тем же ключом в локальных, проектных или пользовательских параметрах, и сохраняет значение более низкого уровня для ключа, который вы опускаете.
3. **Локальные параметры проекта** (`.claude/settings.local.json`): ваши личные параметры для этого проекта.
4. **Общие параметры проекта** (`.claude/settings.json`): параметры, которые ваша команда проверяет в систему управления версиями.
5. **Пользовательские параметры** (`~/.claude/settings.json`): ваши личные параметры для каждого проекта.

Переменные окружения не являются уровнем в этом стеке. Когда поведение имеет как переменную оболочки, так и ключ параметров, какой из них применяется, решается для каждой пары отдельно, а не по уровню: `ANTHROPIC_MODEL`, экспортированная в вашей оболочке, применяется над ключом `model` из любого файла, в то время как `ANTHROPIC_DEFAULT_MODEL` применяется только когда ни один файл не устанавливает `model`. [Справочник переменных окружения](/docs/ru/env-vars#precedence) указывает, какие ключи имеют пару и какой из них Claude Code читает первым. Блок `env` внутри файла параметров — это обычный ключ и следует уровням выше.

Для нескольких ключей, чувствительных к безопасности, Claude Code соблюдает более строгое значение из более низкого уровня над управляемым значением; [Исключения из приоритета управляемых параметров](#exceptions-to-managed-settings-precedence) их перечисляет.

<h3 id="lists-merge-instead-of-overriding">
  Списки объединяются вместо переопределения
</h3>

Когда вы устанавливаете один и тот же ключ списка, такой как `permissions.allow`, в более чем одном файле, Claude Code объединяет списки вместо выбора одного, поэтому каждый файл может добавлять записи без удаления записей другого файла. Четыре ключа, которые содержат списки моделей или записи для каждой модели, следуют своим собственным правилам:

* [`fallbackModel`](/docs/ru/settings-reference#fallbackmodel) — это упорядоченная цепь, где позиция имеет значение, поэтому Claude Code берет все значение из файла с наивысшим приоритетом, который его определяет.
* [`modelPicker`](/docs/ru/settings-reference#modelpicker) содержит один упорядоченный список строк плюс флаг замены, поэтому Claude Code никогда не объединяет строки из двух источников. Он берет все значение из наивысшего из управляемых параметров, `--settings` и пользовательских параметров, которые его определяют, и игнорирует ключ в проектных и локальных параметрах. Требует Claude Code v2.1.242 или позже.
* [`availableModels`](/docs/ru/settings-reference#availablemodels): когда применяемые управляемые параметры Claude Code его определяют, Claude Code применяет этот список как есть и игнорирует записи, которые вы добавляете в пользовательские, проектные или локальные параметры, если только приложение, которое встраивает Claude Code, не предоставляет свой собственный список моделей; см. [Исключения из приоритета управляемых параметров](#exceptions-to-managed-settings-precedence). Во всех управляемых источниках список также никогда не объединяется; [как Claude Code объединяет управляемые источники](/docs/ru/managed-settings#how-claude-code-combines-managed-sources) указывает, какой список источника применяется. Во всех неуправляемых областях Claude Code объединяет массивы как обычно.
* [`modelSettings`](/docs/ru/settings-reference#modelsettings): Claude Code разрешает его одну модель за раз, вместе с [`effortLevel`](/docs/ru/settings-reference#effortlevel). Запись `modelSettings` указывает, какое значение файла применяется к модели.

<span id="examples" />

<h3 id="precedence-examples">
  Примеры приоритета
</h3>

Пока Claude работает, Claude Code показывает подсказку в одну строку под спиннером, например "Используйте /config для изменения режима разрешений по умолчанию (включая Plan Mode)". Предположим, вы хотите отключить эти подсказки, поэтому вы устанавливаете [`spinnerTipsEnabled`](/docs/ru/settings-reference#spinnertipsenabled) в `false` в `~/.claude/settings.json`. Каждый сценарий ниже — это что-то, что может их снова включить, и что вы можете с этим сделать.

<h4 id="team-settings-override-personal-settings">
  Параметры команды переопределяют личные параметры
</h4>

Параметры `.claude/settings.json` вашей команды устанавливают это в `true`. Claude Code использует значение проекта, потому что общие параметры проекта находятся выше пользовательских, поэтому вы видите подсказки в этом проекте и больше нигде.

Вы можете вернуть свое значение: добавьте `"spinnerTipsEnabled": false` в `.claude/settings.local.json` в этом проекте. Локальные параметры проекта находятся выше общих параметров проекта, поэтому ваши сессии там перестанут показывать подсказки, а сессии ваших товарищей по команде не изменятся.

<h4 id="organization-settings-override-everything">
  Параметры организации переопределяют все
</h4>

Управляемые параметры вашей организации устанавливают это в `true`. Ничто из того, что вы поместите в пользовательские, проектные или локальные параметры, не отключит подсказки, и `--settings` тоже не отключит. Управляемые параметры — это наивысший уровень.

Вы не можете вернуть свое значение. Запустите `/status`, чтобы увидеть, какой управляемый источник применяется, и попросите вашего администратора изменить политику.

<h4 id="the-command-line-overrides-your-files-for-one-session">
  Командная строка переопределяет ваши файлы для одной сессии
</h4>

Вы запустили сессию с `claude --settings '{"spinnerTipsEnabled": true}'`. Командная строка находится выше каждого файла, кроме управляемых, поэтому эта сессия показывает подсказки, даже если ваши файлы говорят `false`.

Вы получите свое значение обратно в следующей сессии; `--settings` длится одну сессию и не записывает в какой-либо файл.

<h4 id="a-flag-or-environment-variable-sets-the-same-thing">
  Флаг или переменная окружения устанавливает то же самое
</h4>

Некоторые ключи имеют флаг командной строки или переменную окружения, которая переопределяет значение параметров независимо от того, какой файл его установил: `ANTHROPIC_MODEL` переопределяет параметр [`model`](/docs/ru/settings-reference#model), а `--model` переопределяет оба для сессии.

Можете ли вы вернуть свое значение, зависит от ключа: отмените переменную или удалите флаг, и проверьте запись ключа на [справочнике параметров](/docs/ru/settings-reference) и строку переменной на [справочнике переменных окружения](/docs/ru/env-vars), чтобы узнать, какой из них использует Claude Code.

<span id="keys-ignored-in-a-repository-file" />

<span id="keys-only-you-or-your-organization-can-set" />

<span id="common-cases" />

<span id="which-value-applies-in-common-situations" />

<h3 id="troubleshoot-a-setting-that-doesn’t-apply">
  Устранение неполадок с параметром, который не применяется
</h3>

Когда вы устанавливаете ключ и Claude Code не ведет себя так, как если бы вы это сделали, начните с `/status`, чтобы увидеть, какие файлы он загрузил, затем найдите ваш симптом ниже. [Отладка вашей конфигурации](/docs/ru/debug-your-config) охватывает более широкие проверки, включая тест чистой конфигурации.

<h4 id="a-value-you-set-is-ignored">
  Значение, которое вы установили, игнорируется
</h4>

Что-то еще устанавливает тот же ключ, файл не может установить это значение, или файл не загрузился:

* **Более высокий уровень его устанавливает.** Другой файл параметров, флаг `--settings` или управляемый источник устанавливает ключ выше вашего; [стек](#settings-precedence) указывает, какой. Флаг или переменная окружения также может переопределить ключ самостоятельно, решаемый ключ за ключом; запись ключа на [справочнике параметров](/docs/ru/settings-reference) указывает, какой из них использует Claude Code, и [запись `env`](/docs/ru/settings-reference#env) охватывает управляемое значение `env` в сравнении с экспортом оболочки.
* **Ключ безопасности сохраняет свое строгое значение.** Для нескольких ключей Claude Code соблюдает ограничивающее значение из любого файла, поэтому проектный `true` для [`disableClaudeAiConnectors`](/docs/ru/settings-reference#disableclaudeaiconnectors) остается включенным; см. [Исключения из приоритета управляемых параметров](#exceptions-to-managed-settings-precedence).
* **Файл не может установить это значение.** Значения [`permissions.defaultMode`](/docs/ru/settings-reference#permissions-defaultmode) `auto` и `bypassPermissions` не вступают в силу из проектных или локальных параметров; установите их в пользовательские или управляемые параметры вместо этого, или передайте `--permission-mode` для одной сессии. До v2.1.257 `bypassPermissions` вступал в силу из любого файла.

  Переменная экспорта телеметрии в блоке [`env`](/docs/ru/settings-reference#env) также не вступает в силу из проектных или локальных параметров, кроме нескольких отключенных значений. [Переменные, которые Claude Code игнорирует в `env`](/docs/ru/settings-reference#variables-claude-code-ignores-in-env) перечисляет переменные и эти значения.
* **Файл сломан.** Неверный JSON или отклоненное значение заставляет Claude Code пропустить файл или запись; см. [Исправить сломанный файл параметров](#fix-a-broken-settings-file).

<h4 id="a-change-you-made-in-claude-code-is-lost-in-new-sessions">
  Изменение, которое вы сделали в Claude Code, теряется в новых сессиях
</h4>

Когда вы сохраняете выбор для новых сессий из Claude Code, например модель по умолчанию с `/model`, Claude Code записывает это в ваш файл пользовательских параметров, `~/.claude/settings.json`. Если вы не можете писать в этот файл, например потому что другой инструмент его генерирует или связывает его с копией только для чтения, изменение применяется к текущей сессии и исчезает в следующей. Установите ключ в инструменте, который генерирует файл, или замените файл на тот, в который вы можете писать.

Если вы можете писать в файл и изменение все еще не сохраняется, проверьте, было ли изменение [только для одной сессии](#change-a-setting-for-one-session) или [более высокий уровень устанавливает тот же ключ](#a-value-you-set-is-ignored). Для ключа `model` [Новая сессия начинается с другой модели, чем вы выбрали](/docs/ru/model-config#a-new-session-starts-on-a-different-model-than-you-picked) перечисляет больше причин.

<h4 id="a-managed-change-hasn’t-reached-you">
  Управляемое изменение не достигло вас
</h4>

Управляемые источники достигают работающей сессии по расписанию в [таблице доставки](/docs/ru/managed-settings#choose-a-delivery-mechanism), поэтому сначала перезагрузите сессию. Если `/status` затем называет другой источник, чем тот, который изменил ваш администратор, применяется источник с более высоким приоритетом; [Как Claude Code объединяет управляемые источники](/docs/ru/managed-settings#how-claude-code-combines-managed-sources) дает порядок.

<h4 id="a-committed-key-doesn’t-reach-teammates">
  Зафиксированный ключ не достигает товарищей по команде
</h4>

Две вещи не позволяют ключу в `.claude/settings.json` применяться для всех, кто его клонирует:

* **Claude Code игнорирует ключ в файле репозитория.** Ищите `User, local, or managed`, `User or managed`, `Managed` или `Global config` в столбце Scope [индекса параметров](/docs/ru/settings-reference#settings-index). Эти ключи никогда не применяются из общего файла, кроме нескольких, которые файл репозитория все еще может отключить. Каждая из этих записей говорит об этом на строке Scope. Ключи `Global config` применяются только из `~/.claude.json`.

  Внутри ключа `env` переменные экспорта телеметрии также никогда не применяются из общего файла, кроме нескольких отключенных значений; см. [Переменные, которые Claude Code игнорирует в `env`](/docs/ru/settings-reference#variables-claude-code-ignores-in-env).
* **Ключ ждет доверия.** Правила `permissions.allow`, `permissions.additionalDirectories`, `extraKnownMarketplaces` и большинство значений [`env`](/docs/ru/settings-reference#env) применяются только после того, как каждый товарищ по команде [доверяет папке](/docs/ru/permissions#project-allow-rules-and-workspace-trust). До этого они все еще видят подсказки и не получают плагины с маркетплейса, который объявляет файл. Правила `deny` и `ask` применяются сразу же.

<h4 id="permission-rules-combine-differently-than-you-expected">
  Правила разрешений объединяются иначе, чем вы ожидали
</h4>

* **Вы выбрали "Да, и больше не спрашивайте" в подсказке разрешения, но все еще получаете подсказку для того же инструмента.** Этот выбор сохранил правило `allow` в ваш локальный файл, и правило `allow` там не превосходит правило `ask` из проектного или управляемого файла; [как правила разрешений объединяются](/docs/ru/permissions#settings-precedence) объясняет порядок. В расширении VS Code карточка одобрения позволяет вам выбрать файл назначения, включая общий файл проекта, что изменяет правило для всех; в CLI Claude Code пишет только в ваш локальный файл.
* **Правила разрешения вашей организации все еще применяются наряду с вашими.** Это ожидаемо: Claude Code объединяет [`permissions.allow`](/docs/ru/settings-reference#permissions-allow) во всех областях, если только ваша организация не устанавливает [`allowManagedPermissionRulesOnly`](/docs/ru/settings-reference#allowmanagedpermissionrulesonly).

<span id="security-keys-where-the-stricter-value-applies" />

<h3 id="exceptions-to-managed-settings-precedence">
  Исключения из приоритета управляемых параметров
</h3>

Для нескольких ключей, значения которых ограничивают сессию, Claude Code соблюдает ограничивающее значение из области, которая в противном случае не могла бы переопределить управляемые параметры. Найдите ключ в этой таблице, чтобы увидеть, какое значение он соблюдает и откуда.

| Ключ                                                                            | Значение, которое соблюдает Claude Code                                                                                         | Примечания                                                                                                                                                                                 |
| :------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`disableClaudeAiConnectors`](/docs/ru/settings-reference#disableclaudeaiconnectors) | `true` из любой области                                                                                                         | Соблюдается даже когда управляемый источник устанавливает `false`                                                                                                                          |
| [`enableArtifact`](/docs/ru/settings-reference#enableartifact)                       | `false` из любой области и `disableArtifact: true` из любой области                                                             | Соблюдается даже когда управляемый источник устанавливает `true`; ничто не включает [инструмент Artifact](/docs/ru/artifacts#disable-artifacts) обратно. Требует Claude Code v2.1.242 или позже |
| [`isolatePeerMachines`](/docs/ru/settings-reference#isolatepeermachines)             | `true` из любой области                                                                                                         | Соблюдается даже когда управляемый источник устанавливает `false`                                                                                                                          |
| [`remoteControlAtStartup`](/docs/ru/settings-reference#remotecontrolatstartup)       | `false` из `.claude/settings.json` или `.claude/settings.local.json`                                                            | Соблюдается даже когда управляемый источник устанавливает `true`; проектный или локальный `true` игнорируется                                                                              |
| [`crossSessionInbound`](/docs/ru/settings-reference#crosssessioninbound)             | Более строгое значение из `.claude/settings.json` или `.claude/settings.local.json`, на лестнице `accept` \< `hold` \< `refuse` | Соблюдается над управляемыми, `--settings` и пользовательскими значениями; проектное или локальное значение, которое не является более строгим, игнорируется                               |
| [`useAutoModeDuringPlan`](/docs/ru/settings-reference#useautomodeduringplan)         | `false` из любого управляемого источника, `--settings`, `~/.claude/settings.json` или `.claude/settings.local.json`             | Соблюдается даже когда выигрывающий управляемый источник устанавливает `true`; `false` в `.claude/settings.json` игнорируется                                                              |
| [`syncClaudeAiSkills`](/docs/ru/settings-reference#syncclaudeaiskills)               | `false` из любого управляемого источника, `--settings`, `~/.claude/settings.json` или `.claude/settings.local.json`             | Соблюдается даже когда выигрывающий управляемый источник устанавливает `true`; `false` в `.claude/settings.json` игнорируется                                                              |
| [`syncClaudeAiPlugins`](/docs/ru/settings-reference#syncclaudeaiplugins)             | `false` из любого управляемого источника, `--settings`, `~/.claude/settings.json` или `.claude/settings.local.json`             | Соблюдается даже когда выигрывающий управляемый источник устанавливает `true`; `false` в `.claude/settings.json` игнорируется                                                              |
| [`maxEffortLevel`](/docs/ru/settings-reference#maxeffortlevel)                       | Более низкий предел из любой области, включая `--settings`                                                                      | Соблюдается даже когда применяемые управляемые параметры Claude Code устанавливают более высокий предел; применяется самый низкий предел. Требует Claude Code v2.1.267 или позже           |

Приложение, которое запускает Claude Code внутри себя и устанавливает [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/ru/env-vars), также является исключением. Claude Code берет конфигурацию модели этого приложения над ключами `model`, `fallbackModel`, `modelPicker` и `modelOverrides` из каждого управляемого источника, и над переменными выбора модели в управляемом блоке `env`, такими как `ANTHROPIC_MODEL` и семейство `ANTHROPIC_DEFAULT_*_MODEL`. Claude Code сохраняет управляемый список разрешений [`availableModels`](/docs/ru/settings-reference#availablemodels) в силе, если только приложение не предоставляет свой собственный.

<h2 id="settings-in-cloud-sessions">
  Параметры в облачных сеансах
</h2>

[Облачный сеанс](/docs/ru/claude-code-on-the-web) работает в [облачной среде](/docs/ru/cloud-environments) на свежем клоне вашего репозитория, не на вашей машине. Это изменяет, какие параметры его достигают:

* **Общие параметры проекта** (`.claude/settings.json`): читаются в сеансе с одним репозиторием, потому что файл является частью клона и сеанс начинается внутри него. Зафиксируйте параметр там, чтобы применить его в этих сеансах. Сеанс с несколькими репозиториями начинается выше клонов и читает только ключи `enabledPlugins` и `extraKnownMarketplaces` из файла `.claude/settings.json` каждого репозитория, но не правила разрешений, hooks, `env` или другие ключи. Маркетплейсы и плагины, которые эти два ключа объявляют, по-прежнему [не загружаются в облачном сеансе](/docs/ru/cloud-environments#what-carries-over-from-your-setup).
* **Параметры пользователя и локальные параметры проекта** (`~/.claude/settings.json` и `.claude/settings.local.json`): не читаются. Оба остаются на вашей машине, и локальный файл не находится в клоне.
* **Managed settings**: только [параметры, управляемые сервером](/docs/ru/server-managed-settings) достигают облачного сеанса; файл `managed-settings.json` или профиль MDM на вашем устройстве не достигают. [Самостоятельно размещенная среда](/docs/ru/self-hosted-environments) также читает файл управляемых параметров в образе своего runner. [Как Claude Code объединяет управляемые источники](/docs/ru/managed-settings#how-claude-code-combines-managed-sources) говорит, когда этот файл применяется.
* **`/config`**: в вашем браузере на claude.ai/code открывает раздел Claude Code ваших параметров claude.ai вместо изменения значения. Чтобы изменить параметр для облачного сеанса, установите [переменную окружения](/docs/ru/cloud-environments#set-environment-variables) в среде или в сеансе с одним репозиторием зафиксируйте ключ в файле `.claude/settings.json` этого репозитория.

[Что переносится из вашей установки](/docs/ru/cloud-environments#what-carries-over-from-your-setup) перечисляет остальное: `CLAUDE.md`, skills, MCP servers, plugins и учетные данные.

<h2 id="what’s-next">
  Что дальше
</h2>

* [Все параметры](/docs/ru/settings-reference): каждый ключ, с местом, где вы его устанавливаете и примером
* [Примеры файлов параметров](/docs/ru/settings-example): личный файл, файл команды и файл управляемых параметров организации
* [Настройка разрешений](/docs/ru/permissions): правила allow, ask и deny, и что Claude Code запускает без запроса
* [Переменные окружения](/docs/ru/env-vars): переменные, которые Claude Code читает и блок `env`
* [Отладьте вашу конфигурацию](/docs/ru/debug-your-config): когда параметр не применяется
* [Справочник каталога Claude](/docs/ru/claude-directory): каждый файл, который Claude Code читает, включая subagents, MCP servers, plugins и `CLAUDE.md`
