> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Archivos de configuración y precedencia

> Cambie la configuración de Claude Code, elija el ámbito al que pertenece una clave, verifique el cambio y aprenda qué valor usa Claude Code cuando una clave se establece en varios lugares.

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

Las configuraciones son las claves JSON que cambian cómo se comporta Claude Code: qué modelo inicia, qué puede ejecutar sin preguntar, qué archivos no puede leer, cómo se ve en su terminal y qué aplica su organización.

<Tip>
  Para buscar una clave específica, vaya a [Todas las configuraciones](/docs/es/settings-reference), que enumera cada clave con el archivo en el que la establece, su valor predeterminado y un ejemplo.
</Tip>

Claude Code lee configuraciones de archivos de configuración JSON como `~/.claude/settings.json`. Las busca en algunos lugares, y [el archivo del que lee una configuración decide a quién se aplica](#settings-files-and-who-they-affect). Esta página cubre esos archivos: en cuál poner una configuración, cómo cambiar una configuración y confirmar que se aplicó, y qué valor usa Claude Code cuando la misma clave se establece en más de un archivo. [Configurar permisos](/docs/es/permissions) cubre qué puede ejecutar Claude Code sin preguntar y cómo escribir reglas `allow`, `ask` y `deny`.

<Note>
  Esta página cubre Claude Code ejecutándose en su máquina: la terminal, las extensiones de [VS Code](/docs/es/vs-code) y [JetBrains](/docs/es/jetbrains), y la [aplicación de escritorio](/docs/es/desktop), que todas leen los mismos archivos de configuración. Una sesión en la nube en [Claude Code en la web](/docs/es/claude-code-on-the-web) se ejecuta en una máquina diferente y lee solo algunos de ellos; vea [Configuraciones en sesiones en la nube](#settings-in-cloud-sessions).
</Note>

<span id="settings-files" />

<span id="configuration-scopes" />

<span id="available-scopes" />

<span id="when-to-use-each-scope" />

<span id="what-uses-scopes" />

<span id="subagent-configuration" />

<span id="where-settings-live" />

<h2 id="settings-files-and-who-they-affect">
  Archivos de configuración y a quién afectan
</h2>

Claude Code lee configuraciones de cuatro archivos, y una organización también puede entregar configuraciones administradas desde la consola de claude.ai. Cada fuente tiene un ámbito: el conjunto de personas y proyectos a los que se aplica una configuración guardada en ella, ya sea solo usted, todos en un proyecto o todos en su organización.

| Ámbito              | Archivo                                                                                           | A quién afecta                                                                                                                                                                             | Úselo para                                                                                          |
| :------------------ | :------------------------------------------------------------------------------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------- |
| Usuario             | `~/.claude/settings.json`                                                                         | Usted, en cada proyecto en esta máquina                                                                                                                                                    | Preferencias personales: tema, modo de editor, modelo predeterminado, sus propias reglas de permiso |
| Proyecto compartido | `.claude/settings.json`                                                                           | Todos los que trabajan en la carpeta que lo contiene. En un repositorio de git, confirme para que los compañeros de equipo lo obtengan                                                     | Permisos de equipo, hooks, plugins y las variables de entorno que el proyecto necesita              |
| Proyecto local      | `.claude/settings.local.json`                                                                     | Usted, solo en este proyecto. Claude Code lo mantiene fuera de git cuando crea el archivo; si lo crea a mano, agréguelo a `.gitignore` usted mismo                                         | Anulaciones personales para un proyecto y pruebas antes de compartir                                |
| Administrado        | `managed-settings.json` y otros [mecanismos de entrega](/docs/es/managed-settings#delivery-mechanisms) | Todos en su organización a los que lo implementa; nada de lo que establezca lo anula, aparte de algunas [excepciones sensibles a la seguridad](#exceptions-to-managed-settings-precedence) | Política de seguridad y requisitos de cumplimiento                                                  |

En la columna Archivo, `~/.claude` es la carpeta `.claude` en su directorio de inicio, y un `.claude` simple es la carpeta `.claude` dentro de su proyecto.

<span id="where-each-file-applies" />

<span id="compare-what-each-file-reaches" />

<h3 id="compare-the-scope-of-each-settings-file">
  Compare el ámbito de cada archivo de configuración
</h3>

Suponga que tiene tres proyectos en su máquina, `website/`, `api/` y `acme-app/`, un compañero de equipo tiene su propio clon de `acme-app/`, e inicia una [sesión en la nube](#settings-in-cloud-sessions) en `acme-app/`.

El gráfico a continuación muestra a cuál de esas carpetas se aplica una configuración cuando inicia Claude Code desde ellas. Haga clic en un archivo de configuración para ver las carpetas que alcanza.

<SettingsScope />

* **`~/.claude/settings.json`**: cada proyecto en su máquina, y nada en la de su compañero de equipo o en la sesión en la nube
* **`acme-app/.claude/settings.json`**: su `acme-app/`. Alcanza el clon de su compañero de equipo y la sesión en la nube solo si confirma el archivo en el control de versiones; hasta entonces, es un archivo en su disco como cualquier otro y nadie más lo tiene
* **`acme-app/.claude/settings.local.json`**: su `acme-app/` solo. Claude Code lo agrega a sus exclusiones globales de git la primera vez que escribe el archivo, por lo que se mantiene fuera de sus confirmaciones; si crea el archivo a mano, [agréguelo a `.gitignore` usted mismo](#keep-personal-settings-out-of-a-repository)
* **Configuraciones administradas**, ya sea un archivo `managed-settings.json`, una política MDM, o [configuraciones administradas por servidor](/docs/es/server-managed-settings) desde la consola de claude.ai: cada proyecto en cada máquina en la que su organización lo implementa, o en la que inicia sesión con su cuenta de organización. Solo las configuraciones administradas por servidor alcanzan la sesión en la nube

<span id="which-files-you-have" />

<h3 id="find-or-create-your-settings-files">
  Encuentre o cree sus archivos de configuración
</h3>

Instalar Claude Code no crea ningún archivo de configuración. Si su máquina o proyecto ya tiene uno, vino de una de estas fuentes:

* **Administrado**: su organización lo implementa. Usted no lo crea ni lo edita.
* **Proyecto compartido**: un proyecto que ya usa Claude Code puede tener uno confirmado. Si no, créelo en `.claude/settings.json` en la carpeta del proyecto.
* **Usuario** y **Proyecto local**: créelos usted mismo, o deje que Claude Code los cree. Escribe `~/.claude/settings.json` la primera vez que cambia una opción en el menú `/config` que almacena en configuraciones de usuario, como el tema, y `.claude/settings.local.json` la primera vez que da una aprobación permanente en un aviso de permiso, como "Sí, y no preguntes de nuevo" para un comando Bash. Algunas opciones de `/config`, incluyendo **Mostrar consejos**, se guardan en `.claude/settings.local.json` en su lugar.

<Info>
  En Windows, `~/.claude` significa `%USERPROFILE%\.claude`. Para mantener los archivos del directorio de inicio en otro lugar, establezca [`CLAUDE_CONFIG_DIR`](/docs/es/env-vars); Claude Code almacena sus configuraciones, historial de sesiones y plugins allí en su lugar.
</Info>

Claude Code también mantiene un quinto archivo, [`~/.claude.json`](/docs/es/claude-directory#ce-claude-json), que escribe para sí mismo; no necesita editarlo. Contiene su sesión de inicio de sesión, configuraciones de [servidor MCP](/docs/es/mcp), estado por proyecto como decisiones de confianza, y las [claves de configuración global](/docs/es/settings-reference#global-config-settings) que `/config` escribe para usted.

<h3 id="share-settings-with-your-team">
  Comparta configuraciones con su equipo
</h3>

Confirme `.claude/settings.json` para que todos los que clonan el repositorio obtengan los mismos permisos, hooks y plugins. Cada compañero de equipo aún puede anularlo para sí mismo en su propio `.claude/settings.local.json`, por lo que las excepciones personales no necesitan una confirmación. Para un archivo de equipo completo, vea [configuraciones compartidas de un equipo](/docs/es/settings-example#a-teams-shared-settings).

Parte de lo que confirma espera hasta que cada compañero de equipo [confíe en la carpeta](/docs/es/permissions#project-allow-rules-and-workspace-trust), y algunas claves nunca surten efecto desde un archivo de repositorio; [Solucione una configuración que no se aplica](#common-cases) cubre ambas.

<span id="local-settings-file" />

<span id="where-claude-code-saves-the-project-local-file" />

<span id="the-project-local-file" />

<span id="keep-personal-settings-out-of-the-repository" />

<h3 id="keep-personal-settings-out-of-a-repository">
  Mantenga las configuraciones personales fuera de un repositorio
</h3>

Para cambiar una configuración para usted en un proyecto sin cambiarla para sus compañeros de equipo, guárdela en `.claude/settings.local.json` dentro del proyecto. Claude Code aplica ese archivo sobre el `.claude/settings.json` confirmado, por lo que si el archivo de su equipo establece `"model": "claude-sonnet-5"` y usted quiere Opus, ponga `"model": "claude-opus-5-5"` en su archivo local y solo sus sesiones cambian.

Claude Code también escribe en este archivo, lo mantiene fuera de sus confirmaciones, y aplica sus reglas allow sin el paso de confianza:

* **Claude Code también lo escribe.** Cuando Claude pide permiso para ejecutar un comando Bash y elige "Sí, y no preguntes de nuevo", Claude Code guarda esa [aprobación de permiso](/docs/es/permissions#permission-system) aquí como una regla `allow`.
* **No necesita gitignore usted mismo, a menos que lo haya creado a mano.** La primera vez que Claude Code escribe el archivo en un repositorio de git que aún no lo ignora, agrega `**/.claude/settings.local.json` a su archivo de exclusiones global de git, por lo que el archivo se mantiene fuera de sus confirmaciones en cada repositorio. Ese archivo es `core.excludesFile` cuando su configuración global de git lo establece en una ruta absoluta o con prefijo `~`; de lo contrario es `$XDG_CONFIG_HOME/git/ignore`, o `~/.config/git/ignore` cuando `XDG_CONFIG_HOME` no está establecido. Si creó el archivo a mano y Claude Code aún no ha escrito en él, agréguelo a `.gitignore` usted mismo.
* **Sus reglas allow no esperan confianza mientras el archivo permanece sin rastrear.** Porque el archivo es suyo y no del repositorio, Claude Code aplica sus reglas `allow` sin el paso de [confianza del espacio de trabajo](/docs/es/permissions#project-allow-rules-and-workspace-trust) que requiere para el archivo confirmado. Si el archivo es rastreado por git, el paso de confianza también se aplica a él; vea [Cuando su archivo de configuración local necesita confianza](/docs/es/permissions#when-your-local-settings-file-needs-trust).

<span id="where-claude-code-looks-for-each-file" />

<span id="how-claude-code-keeps-the-local-file-out-of-git" />

<span id="local-allow-rules-dont-wait-for-workspace-trust" />

<h4 id="where-claude-code-keeps-the-local-file-in-a-git-repository">
  Dónde Claude Code mantiene el archivo local en un repositorio de git
</h4>

Cuando Claude pide permiso para ejecutar un comando Bash y elige "Sí, y no preguntes de nuevo", Claude Code guarda esa aprobación como una regla `allow` en `.claude/settings.local.json`. Si inicia Claude Code en un subdirectorio de un repositorio de git, lee y escribe ese archivo en la raíz del repositorio y aplica la aprobación en todo el repositorio. En un [worktree](/docs/es/worktrees), usa el archivo en la raíz del checkout principal.

Dos reglas califican la ubicación raíz:

* **Cuando el archivo se queda con `.claude/settings.json` en su lugar**: fuera de un repositorio de git, cuando la raíz del repositorio es su directorio de inicio, en Windows, o cuando la raíz del repositorio o su entrada `.git` o `.claude` no es propiedad de su usuario.
* **Las rutas en el archivo no se anclan en la raíz del repositorio**: una regla de permiso que comienza con `/` o una ruta de sandbox relativa [se ancla en el directorio de trabajo principal de la sesión](/docs/es/permissions#read-and-edit) en su lugar.

Antes de v2.1.211, Claude Code mantenía el archivo en el directorio de inicio. Aún lee un archivo que una versión anterior dejó allí junto al archivo raíz; donde ambos establecen la misma clave, se aplica el valor de la raíz, y las reglas de permiso de ambos archivos se aplican. El asistente [`resolveSettings()`](/docs/es/agent-sdk/typescript#resolvesettings) del Agent SDK siempre lee el archivo desde el directorio de inicio.

Claude Code lee el `.claude/settings.json` compartido desde el [directorio de trabajo principal](/docs/es/permissions#working-directories) de la sesión, por lo que para usar un archivo confirmado en la raíz del repositorio, inicie Claude Code allí. Después de [mover la sesión con `/cd`](/docs/es/permissions#move-the-session-to-another-directory), Claude Code lee ambos archivos de proyecto desde el nuevo directorio en su lugar, colocando el archivo local por las mismas reglas. Leerlos desde el directorio al que se movió requiere Claude Code v2.1.246 o posterior.

<span id="managed-settings-delivery" />

<span id="precedence-within-the-managed-tier" />

<span id="parent-settings-from-embedding-hosts" />

<span id="enforce-settings-for-an-organization" />

<span id="settings-your-organization-manages" />

<h3 id="check-what-your-organization-enforces">
  Verifique qué aplica su organización
</h3>

Si su organización administra Claude Code, algunas configuraciones se deciden para usted y nada de lo que ponga en sus propios archivos las cambia. Para ver cuáles, ejecute `/status`: la línea `Setting sources` nombra la fuente administrada que se aplica a usted. Las configuraciones administradas se aplican dondequiera que Claude Code se ejecute en esta máquina; [Lo que un desarrollador puede cambiar](/docs/es/managed-settings#what-a-developer-can-change) cubre derechos de administrador local y herramientas distintas de Claude Code.

Las configuraciones administradas lo alcanzan a través de los [mecanismos de entrega](/docs/es/managed-settings#delivery-mechanisms) en la página de configuraciones administradas, más comúnmente:

* [Configuraciones administradas por servidor](/docs/es/server-managed-settings), que Claude Code obtiene de la consola de administrador de claude.ai o de una [puerta de aplicaciones Claude](/docs/es/claude-apps-gateway) autohospedada
* Políticas MDM o a nivel de SO, y archivos `managed-settings.json` en un directorio del sistema
* Un host de incrustación como Claude Desktop, a través de la opción SDK `managedSettings`; vea [Controlar política desde un host de incrustación](/docs/es/managed-settings#parent-settings-from-embedding-hosts)

En una sesión de [Cowork](https://claude.com/docs/cowork/overview) que se ejecuta en su máquina en la aplicación Claude Desktop, Claude Code no obtiene configuraciones administradas por servidor de la consola de administrador de claude.ai, y lee la política implementada en su dispositivo a menos que la configuración de Claude Desktop de su organización establezca `requireCoworkFullVmSandbox`. [Dónde y cuándo se aplica una política](/docs/es/managed-settings#where-and-when-a-policy-applies) cubre Cowork y sesiones en la nube.

Si es el administrador, [Configure Claude Code para su organización](/docs/es/admin-setup) lo guía a través de elegir qué aplicar, y [Implemente configuraciones administradas](/docs/es/managed-settings) cubre la entrega y cómo confirmar que una política está en vigor.

<h2 id="change-a-setting">
  Cambie una configuración
</h2>

Puede cambiar una configuración desde el menú `/config`, editando un archivo de configuración, o para una sesión desde la línea de comandos.

<span id="system-prompt" />

El indicador del sistema de Claude Code no se publica. Para dar a Claude instrucciones permanentes, use [archivos `CLAUDE.md`](/docs/es/memory) o la bandera `--append-system-prompt`.

<h3 id="use-the-/config-menu">
  Use el menú /config
</h3>

Ejecute `/config` dentro de Claude Code y abra la pestaña **Config**. Enumera un conjunto corto de opciones personales como tema, modo de editor y salida detallada, no cada clave de configuración. Seleccione una opción para cambiarla; Claude Code la guarda para usted:

* **La mayoría de opciones**: `~/.claude/settings.json`
* **Algunas opciones, como Mostrar consejos**: `.claude/settings.local.json`
* **Las [opciones de configuración global](/docs/es/settings-reference#global-config-settings)**: `~/.claude.json`

Para establecer una opción sin el menú, pase `key=value`, como `/config verbose=true`.

<Note>
  `/config` es parte de la interfaz de terminal. El [panel de chat de VS Code](/docs/es/vs-code) y la [aplicación de escritorio](/docs/es/desktop) no lo abren; cambie las configuraciones allí editando un archivo de configuración o a través de las configuraciones propias de esas aplicaciones.
</Note>

<h3 id="edit-a-settings-file">
  Edite un archivo de configuración
</h3>

Abra el archivo de configuración para el ámbito que desea en su editor y agregue o cambie una clave. Los archivos de configuración son JSON estricto: un comentario `//` o una coma final es un error de sintaxis, y Claude Code reporta el archivo como un [Error de configuración](#fix-a-broken-settings-file) al siguiente inicio. Por ejemplo, para permitir que Claude Code ejecute sus comandos de lint y prueba sin preguntar y evitar que lea archivos `.env`, agregue esto a `~/.claude/settings.json`:

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

Cada entrada bajo `permissions` es una regla que nombra una herramienta y qué puede hacer; [Configurar permisos](/docs/es/permissions) explica la sintaxis. La línea `$schema` apunta al [esquema JSON publicado](https://json.schemastore.org/claude-code-settings.json) para configuraciones de Claude Code, que le da autocompletado y validación en línea en VS Code, Cursor y cualquier otro editor que admita esquema JSON. El esquema puede quedarse atrás de los lanzamientos de CLI más recientes, por lo que una advertencia de validación en una clave documentada recientemente no significa que su configuración sea inválida.

Después de guardar, ejecute `/status` dentro de Claude Code para confirmar que el archivo se cargó; [Confirme qué se cargó](#check-what-loaded) dice qué muestra la línea `Setting sources` y cómo se reporta un archivo roto.

Para un archivo personal completo, archivo de equipo y archivo de organización, cada uno mostrado con un comentario en cada clave que establece, vea los [archivos de configuración de ejemplo](/docs/es/settings-example).

<span id="pass-settings-for-one-session" />

<h3 id="change-a-setting-for-one-session">
  Cambie una configuración para una sesión
</h3>

Para probar un valor sin guardarlo, establézcalo cuando inicie Claude Code. El valor se aplica a esa sesión y sus archivos de configuración permanecen como estaban. Tiene tres formas de hacerlo:

* **`--settings`**: pase una clave como JSON, en línea o como una ruta a un archivo. Claude Code la aplica sobre sus archivos de usuario, proyecto y local y debajo de las configuraciones administradas. Puede establecer cualquier clave que su archivo de configuración de usuario pueda establecer; no puede establecer claves `Managed` o `Global config`.
* **Una bandera para esa clave**: algunas claves tienen su propia bandera, como `--model` para `model` y `--effort` para `effortLevel` y `modelSettings`.
* **Una variable de entorno**: exporte la variable emparejada de la clave antes de ejecutar `claude`, como `ANTHROPIC_MODEL` para `model`.

La entrada de cada clave en la [referencia de configuración](/docs/es/settings-reference) enumera sus anulaciones por sesión y cuál tiene precedencia, así que verifique la entrada para la clave que desea cambiar.

Los comandos que ejecuta dentro de una sesión principalmente guardan su elección: cuando cambia una configuración en `/config`, Claude Code la escribe en sus archivos de configuración, y `/model` guarda el valor como su predeterminado para nuevas sesiones.

Si presiona `s` en el selector `/model`, Claude Code cambia el modelo sin guardarlo como su predeterminado de usuario. [Ajuste el nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) dice qué selecciones de `/effort` Claude Code guarda como su predeterminado para el modelo que está usando y cuáles se aplican solo a la sesión actual.

Por ejemplo, para iniciar una sesión en Opus sin cambiar su predeterminado:

```bash theme={null}
claude --settings '{"model": "claude-opus-5-5"}'
```

<h3 id="when-edits-take-effect">
  Cuándo surten efecto las ediciones
</h3>

Claude Code observa sus archivos de configuración y los recarga cuando cambian, por lo que aplica la mayoría de ediciones a la sesión en ejecución sin un reinicio, incluyendo ediciones a `permissions`, `hooks` y asistentes de credenciales como `apiKeyHelper`. Claude Code también carga un archivo de configuración que crea a mitad de sesión si su carpeta existía cuando la sesión comenzó. Para la carpeta `.claude/` del proyecto, carga el archivo incluso cuando crea la carpeta en la misma sesión.

La recarga cubre configuraciones de usuario, proyecto, local y administradas, y Claude Code ejecuta el [hook `ConfigChange`](/docs/es/hooks#configchange) para cada cambio de archivo de configuración que detecta, no para configuraciones administradas que llegan desde MDM o la consola de claude.ai. Las configuraciones administradas que llegan a través de MDM o de la consola de claude.ai alcanzan una sesión en ejecución en un cronograma en lugar de al guardar; la [tabla de entrega](/docs/es/managed-settings#choose-a-delivery-mechanism) lo da por fuente.

Claude Code lee algunas claves solo una vez, al inicio de la sesión, por lo que una edición a una de ellas no alcanza la sesión en ejecución. Las claves del lado del administrador que también esperan un reinicio, como `requiredMinimumVersion`, se enumeran bajo [dónde y cuándo se aplica una política](/docs/es/managed-settings#where-and-when-a-policy-applies). Las que es más probable que edite a mitad de sesión:

* [`model`](/docs/es/settings-reference#model): use [`/model`](/docs/es/model-config#setting-your-model) para cambiar a mitad de sesión. Cada modelo tiene su propio caché de indicador, por lo que la primera solicitud después de un cambio relee toda la conversación sin caché; vea [Cambiar modelos](/docs/es/prompt-caching#switching-models)
* [`effortLevel`](/docs/es/settings-reference#effortlevel) y [`modelSettings`](/docs/es/settings-reference#modelsettings): use [`/effort`](/docs/es/model-config#adjust-effort-level) para cambiar esfuerzo a mitad de sesión

<span id="verify-active-settings" />

<span id="check-what-loaded" />

<h3 id="confirm-what-loaded">
  Confirme qué se cargó
</h3>

Ejecute `/status` dentro de Claude Code para ver qué fuentes de configuración están activas. La pestaña **Status** incluye una línea `Setting sources` que enumera cada archivo de configuración que Claude Code cargó para la sesión actual, como `User settings` o `Project local settings`. Cuando [configuraciones administradas](/docs/es/admin-setup#decide-how-settings-reach-devices) están en vigor, la entrada de configuraciones administradas muestra entre paréntesis cómo llegaron a su máquina.

La línea confirma qué archivos leyó Claude Code; no muestra qué archivo suministró cada clave. Para enumerar entradas que Claude Code rechazó, ejecute [`claude doctor`](/docs/es/debug-your-config); para un modelo que configuraciones de proyecto o administradas establecen, el encabezado de inicio nombra el archivo que lo estableció. `/status` y `/config` abren el mismo diálogo en pestañas diferentes, y la pestaña **Config** no es una vista de sus contenidos de `settings.json`.

<h3 id="fix-a-broken-settings-file">
  Corrija un archivo de configuración roto
</h3>

Si escribe mal JSON o establece una clave a un valor que Claude Code no acepta, Claude Code se lo dice al inicio de una sesión interactiva. Lo que muestra depende de cuánto del archivo se vea afectado:

* **Error de configuración**: un archivo de usuario, proyecto o local tiene JSON inválido o un valor que el esquema rechaza. Al inicio de una sesión interactiva Claude Code muestra un diálogo que le permite corregir el archivo con la ayuda de Claude, salir o continuar sin las configuraciones rotas.
* **Advertencia de configuración**: solo entradas individuales fallan, como una regla de permiso malformada o un nombre de evento de hook desconocido. Claude Code omite esos valores y mantiene el resto del archivo en vigor.
* **Configuraciones administradas**: Claude Code continúa aplicando el resto del archivo. [Entradas inválidas en configuraciones administradas](/docs/es/managed-settings#invalid-entries-in-managed-settings) dice qué descarta y qué claves recurren a un valor más estricto hasta que las corrija. Para un documento de configuraciones administradas que no es JSON válido, vea [No se pudo analizar el documento de configuraciones administradas](/docs/es/errors#managed-settings-document-could-not-be-parsed).
* **Error de configuración**: `~/.claude.json` no se puede analizar. Claude Code copia el archivo roto a `~/.claude/backups/.claude.json.corrupted.<timestamp>` y pregunta si salir y corregirlo a mano o restablecer a la configuración predeterminada; una ejecución `-p` imprime el error y sale. Para recuperar su estado anterior, copie uno de los cinco archivos `.claude.json.backup.<timestamp>` más recientes en `~/.claude/backups/`, que Claude Code guarda antes de escribir el archivo.

Después de continuar, ejecute `/status` para ver los archivos afectados y `claude doctor` para los detalles de cada error.

Una ejecución `-p` no muestra diálogo. A menos que [un documento de configuraciones administradas no se pueda analizar](/docs/es/errors#managed-settings-document-could-not-be-parsed), Claude Code omite el archivo o valores rotos y continúa con el resto, por lo que después de una ejecución `-p` que ignora una configuración, ejecute `claude doctor` para ver qué descartó.

<span id="how-scopes-interact" />

<span id="key-points-about-the-configuration-system" />

<span id="which-value-claude-code-uses" />

<span id="which-value-wins" />

<h2 id="settings-precedence">
  Precedencia de configuración
</h2>

Cuando la misma clave aparece en más de un lugar, Claude Code usa el valor del nivel más alto que la establece. La pila a continuación muestra los niveles, más alto en la parte superior; una clave en un nivel más alto anula la misma clave en cualquier lugar debajo.

<SettingsPrecedence />

En orden, precedencia más alta primero:

1. **Configuraciones administradas**: configuraciones que su organización implementa, por un archivo `managed-settings.json`, una política MDM, o [configuraciones administradas por servidor](/docs/es/server-managed-settings) de la consola de claude.ai. Nada de lo que establezca las anula: una clave que pasa con `--settings` no anula la misma clave administrada, y una bandera como `--model` elige solo de los modelos que su organización permite. Una `model` administrada establece el modelo con el que cada sesión comienza, y aún puede cambiar con `/model`; el bloqueo es [`availableModels`](/docs/es/settings-reference#availablemodels), que restringe `/model`, `--model` y la clave `model` en sus propios archivos. Cuando su organización entrega más de una fuente administrada, las reglas para [precedencia dentro del nivel administrado](/docs/es/managed-settings#precedence-within-the-managed-tier) dicen qué lee Claude Code de cada una.
2. **Argumentos de línea de comandos**: banderas que pasa cuando inicia `claude` desde una terminal, para una sesión; vea [Cambie una configuración para una sesión](#change-a-setting-for-one-session). Claude Code fusiona JSON que pasa con `--settings <file-or-json>` con sus archivos de configuración por las mismas reglas que los otros niveles: toma una clave que establece aquí sobre la misma clave en configuraciones locales, de proyecto o de usuario, y mantiene el valor de nivel inferior para una clave que omite.
3. **Configuraciones locales de proyecto** (`.claude/settings.local.json`): sus configuraciones personales para este proyecto.
4. **Configuraciones compartidas de proyecto** (`.claude/settings.json`): configuraciones que su equipo verifica en el control de código fuente.
5. **Configuraciones de usuario** (`~/.claude/settings.json`): sus configuraciones personales para cada proyecto.

Las variables de entorno no son un nivel en esta pila. Cuando un comportamiento tiene tanto una variable de shell como una clave de configuración, cuál se aplica se decide por par, no por nivel: `ANTHROPIC_MODEL` exportada en su shell se aplica sobre la clave `model` de cualquier archivo, mientras que `ANTHROPIC_DEFAULT_MODEL` se aplica solo cuando ningún archivo establece `model`. La [referencia de variables de entorno](/docs/es/env-vars#precedence) dice qué claves tienen un par y cuál lee Claude Code primero. Un bloque `env` dentro de un archivo de configuración es una clave ordinaria y sigue los niveles anteriores.

Para algunas claves sensibles a la seguridad, Claude Code honra un valor más estricto de un nivel inferior sobre un valor administrado; [Excepciones a la precedencia de configuraciones administradas](#exceptions-to-managed-settings-precedence) las enumera.

<h3 id="lists-merge-instead-of-overriding">
  Las listas se fusionan en lugar de anular
</h3>

Cuando establece la misma clave de lista, como `permissions.allow`, en más de un archivo, Claude Code combina las listas en lugar de elegir una, por lo que cada archivo puede agregar entradas sin eliminar las de otro archivo. Cuatro claves que contienen listas de modelos o entradas por modelo siguen sus propias reglas:

* [`fallbackModel`](/docs/es/settings-reference#fallbackmodel) es una cadena ordenada donde la posición tiene significado, por lo que Claude Code toma el valor completo del archivo de mayor precedencia que la define.
* [`modelPicker`](/docs/es/settings-reference#modelpicker) contiene una lista ordenada de filas más una bandera de reemplazo, por lo que Claude Code nunca fusiona filas de dos fuentes. Toma el valor completo del más alto de configuraciones administradas, `--settings` y configuraciones de usuario que la define, e ignora la clave en configuraciones de proyecto y local. Requiere Claude Code v2.1.242 o posterior.
* [`availableModels`](/docs/es/settings-reference#availablemodels): cuando las configuraciones administradas que Claude Code aplica la definen, Claude Code aplica esa lista tal cual e ignora entradas que agrega en configuraciones de usuario, proyecto o local, a menos que una aplicación que incrusta Claude Code suministre su propia lista de modelos; vea [Excepciones a la precedencia de configuraciones administradas](#exceptions-to-managed-settings-precedence). Entre fuentes administradas la lista nunca se fusiona tampoco; [cómo Claude Code combina fuentes administradas](/docs/es/managed-settings#how-claude-code-combines-managed-sources) dice qué lista de fuente se aplica. Entre ámbitos no administrados Claude Code fusiona las matrices como de costumbre.
* [`modelSettings`](/docs/es/settings-reference#modelsettings): Claude Code la resuelve un modelo a la vez, junto con [`effortLevel`](/docs/es/settings-reference#effortlevel). La entrada `modelSettings` indica qué archivo de valor se aplica a un modelo.

<span id="examples" />

<h3 id="precedence-examples">
  Ejemplos de precedencia
</h3>

Mientras Claude trabaja, Claude Code muestra un consejo de una línea bajo el spinner, como "Use /config para cambiar su modo de permiso predeterminado (incluyendo Plan Mode)". Suponga que desea esos consejos apagados, por lo que establece [`spinnerTipsEnabled`](/docs/es/settings-reference#spinnertipsenabled) en `false` en `~/.claude/settings.json`. Cada escenario a continuación es algo que puede encenderlos de nuevo, y qué puede hacer al respecto.

<h4 id="team-settings-override-personal-settings">
  Las configuraciones de equipo anulan las configuraciones personales
</h4>

El `.claude/settings.json` de su equipo lo establece en `true`. Claude Code usa el valor del proyecto porque el proyecto compartido se sienta sobre el usuario, por lo que ve consejos en ese proyecto y en ningún otro lugar.

Puede recuperar su valor: agregue `"spinnerTipsEnabled": false` a `.claude/settings.local.json` en ese proyecto. El proyecto local se sienta sobre el proyecto compartido, por lo que sus sesiones allí dejan de mostrar consejos y las sesiones de sus compañeros de equipo no cambian.

<h4 id="organization-settings-override-everything">
  Las configuraciones de la organización anulan todo
</h4>

Las configuraciones administradas de su organización lo establecen en `true`. Nada de lo que ponga en configuraciones de usuario, proyecto o local apaga los consejos, y tampoco `--settings`. Administrado es el nivel superior.

No puede recuperar su valor. Ejecute `/status` para ver qué fuente administrada se aplica, y pregunte a su administrador si la política debería cambiar.

<h4 id="the-command-line-overrides-your-files-for-one-session">
  La línea de comandos anula sus archivos para una sesión
</h4>

Inició la sesión con `claude --settings '{"spinnerTipsEnabled": true}'`. La línea de comandos se sienta sobre cada archivo excepto administrado, por lo que esa sesión muestra consejos aunque sus archivos digan `false`.

Recupera su valor en la siguiente sesión; `--settings` dura una sesión y no escribe en ningún archivo.

<h4 id="a-flag-or-environment-variable-sets-the-same-thing">
  Una bandera o variable de entorno establece lo mismo
</h4>

Algunas claves tienen una bandera de línea de comandos o una variable de entorno que anula el valor de configuración independientemente de qué archivo lo estableció: `ANTHROPIC_MODEL` anula la configuración [`model`](/docs/es/settings-reference#model), y `--model` anula ambas para una sesión.

Si puede recuperar su valor depende de la clave: desexporte la variable o suelte la bandera, y verifique la entrada de la clave en la [referencia de configuración](/docs/es/settings-reference) y la fila de la variable en la [referencia de variables de entorno](/docs/es/env-vars) para cuál usa Claude Code.

<span id="keys-ignored-in-a-repository-file" />

<span id="keys-only-you-or-your-organization-can-set" />

<span id="common-cases" />

<span id="which-value-applies-in-common-situations" />

<h3 id="troubleshoot-a-setting-that-doesn’t-apply">
  Solucione una configuración que no se aplica
</h3>

Cuando establece una clave y Claude Code no se comporta como si lo hubiera hecho, comience con `/status` para ver qué archivos cargó, luego encuentre su síntoma a continuación. [Depure su configuración](/docs/es/debug-your-config) cubre las verificaciones más amplias, incluyendo una prueba de configuración limpia.

<h4 id="a-value-you-set-is-ignored">
  Un valor que establece se ignora
</h4>

Algo más está estableciendo la misma clave, el archivo no puede establecer ese valor, o el archivo no se cargó:

* **Un nivel más alto lo establece.** Otro archivo de configuración, una bandera `--settings`, o una fuente administrada establece la clave por encima de la suya; la [pila](#settings-precedence) dice cuál. Una bandera o variable de entorno también puede anular la clave por su cuenta, decidido clave por clave; la entrada de la clave en la [referencia de configuración](/docs/es/settings-reference) dice cuál usa Claude Code, y la [entrada `env`](/docs/es/settings-reference#env) cubre un valor `env` administrado versus una exportación de shell.
* **Una clave de seguridad mantiene su valor estricto.** Para algunas claves Claude Code honra el valor restrictivo de cualquier archivo, por lo que un proyecto `true` para [`disableClaudeAiConnectors`](/docs/es/settings-reference#disableclaudeaiconnectors) permanece encendido; vea [Excepciones a la precedencia de configuraciones administradas](#exceptions-to-managed-settings-precedence).
* **El archivo no puede establecer ese valor.** Los valores [`permissions.defaultMode`](/docs/es/settings-reference#permissions-defaultmode) `auto` y `bypassPermissions` no surten efecto desde configuraciones de proyecto o local; establézcalos en configuraciones de usuario o administradas en su lugar, o pase `--permission-mode` para una sesión. Antes de v2.1.257, `bypassPermissions` surtía efecto desde cualquier archivo.

  Una variable de exportación de telemetría en un bloque [`env`](/docs/es/settings-reference#env) tampoco surte efecto desde configuraciones de proyecto o local, aparte de algunos valores desactivados. [Variables que Claude Code ignora en `env`](/docs/es/settings-reference#variables-claude-code-ignores-in-env) enumera las variables y esos valores.
* **El archivo está roto.** JSON inválido o un valor rechazado hace que Claude Code omita el archivo o la entrada; vea [Corrija un archivo de configuración roto](#fix-a-broken-settings-file).

<h4 id="a-change-you-made-in-claude-code-is-lost-in-new-sessions">
  Un cambio que hizo en Claude Code se pierde en nuevas sesiones
</h4>

Cuando guarda una elección para nuevas sesiones desde dentro de Claude Code, como un modelo predeterminado con `/model`, Claude Code lo escribe en su archivo de configuración de usuario, `~/.claude/settings.json`. Si no puede escribir en ese archivo, por ejemplo porque otra herramienta lo genera o lo vincula a una copia de solo lectura, el cambio se aplica a la sesión actual y se pierde en la siguiente. Establezca la clave en la herramienta que genera el archivo, o reemplace el archivo con uno en el que pueda escribir.

Si puede escribir en el archivo y el cambio aún no dura, verifique si el cambio fue [solo para una sesión](#change-a-setting-for-one-session) o [un nivel más alto establece la misma clave](#a-value-you-set-is-ignored). Para la clave `model`, [Una nueva sesión comienza en un modelo diferente al que eligió](/docs/es/model-config#a-new-session-starts-on-a-different-model-than-you-picked) enumera más causas.

<h4 id="a-managed-change-hasn’t-reached-you">
  Un cambio administrado no lo ha alcanzado
</h4>

Las fuentes administradas alcanzan una sesión en ejecución en el cronograma en la [tabla de entrega](/docs/es/managed-settings#choose-a-delivery-mechanism), por lo que reinicie la sesión primero. Si `/status` entonces nombra una fuente diferente a la que su administrador cambió, una fuente de mayor prioridad se aplica; [Cómo Claude Code combina fuentes administradas](/docs/es/managed-settings#how-claude-code-combines-managed-sources) da el orden.

<h4 id="a-committed-key-doesn’t-reach-teammates">
  Una clave confirmada no alcanza a los compañeros de equipo
</h4>

Dos cosas mantienen una clave en `.claude/settings.json` de aplicarse para todos los que la clonan:

* **Claude Code ignora la clave en un archivo de repositorio.** Busque `User, local, or managed`, `User or managed`, `Managed`, o `Global config` en la columna Scope del [índice de configuración](/docs/es/settings-reference#settings-index). Esas claves nunca se aplican desde el archivo compartido, aparte de algunas que un archivo de repositorio aún puede apagar. Cada una de esas entradas lo dice en su línea de Scope. Las claves `Global config` se aplican solo desde `~/.claude.json`.

  Dentro de la clave `env`, las variables de exportación de telemetría nunca se aplican desde el archivo compartido tampoco, aparte de algunos valores desactivados; vea [Variables que Claude Code ignora en `env`](/docs/es/settings-reference#variables-claude-code-ignores-in-env).
* **La clave espera confianza.** Las reglas `permissions.allow`, `permissions.additionalDirectories`, `extraKnownMarketplaces` y la mayoría de valores [`env`](/docs/es/settings-reference#env) se aplican solo después de que cada compañero de equipo [confíe en la carpeta](/docs/es/permissions#project-allow-rules-and-workspace-trust). Hasta entonces aún ven avisos y no obtienen plugins de un marketplace que el archivo declara. Las reglas `deny` y `ask` se aplican de inmediato.

<h4 id="permission-rules-combine-differently-than-you-expected">
  Las reglas de permiso se combinan diferente a lo que esperaba
</h4>

* **Eligió "Sí, y no preguntes de nuevo" en un aviso de permiso pero aún recibe avisos para la misma herramienta.** Esa elección guardó una regla `allow` en su archivo local, y una regla `allow` allí no supera una regla `ask` de un archivo de proyecto o administrado; [cómo se combinan las reglas de permiso](/docs/es/permissions#settings-precedence) explica el orden. En la extensión de VS Code la tarjeta de aprobación le permite elegir el archivo de destino, incluyendo el archivo compartido del proyecto, que cambia la regla para todos; en la CLI, Claude Code escribe solo en su archivo local.
* **Las reglas allow de su organización aún se aplican junto con las suyas.** Eso es esperado: Claude Code fusiona [`permissions.allow`](/docs/es/settings-reference#permissions-allow) entre ámbitos, a menos que su organización establezca [`allowManagedPermissionRulesOnly`](/docs/es/settings-reference#allowmanagedpermissionrulesonly).

<span id="security-keys-where-the-stricter-value-applies" />

<h3 id="exceptions-to-managed-settings-precedence">
  Excepciones a la precedencia de configuraciones administradas
</h3>

Para algunas claves cuyos valores restringen una sesión, Claude Code honra un valor restrictivo de un ámbito que de otra manera no podría anular configuraciones administradas. Encuentre la clave en esta tabla para ver qué valor honra y de dónde.

| Clave                                                                           | Valor que Claude Code honra                                                                                                     | Notas                                                                                                                                                                                         |
| :------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`disableClaudeAiConnectors`](/docs/es/settings-reference#disableclaudeaiconnectors) | `true` de cualquier ámbito                                                                                                      | Honrado incluso cuando una fuente administrada establece `false`                                                                                                                              |
| [`enableArtifact`](/docs/es/settings-reference#enableartifact)                       | `false` de cualquier ámbito, y `disableArtifact: true` de cualquier ámbito                                                      | Honrado incluso cuando una fuente administrada establece `true`; nada enciende la [herramienta Artifact](/docs/es/artifacts#disable-artifacts) de nuevo. Requiere Claude Code v2.1.242 o posterior |
| [`isolatePeerMachines`](/docs/es/settings-reference#isolatepeermachines)             | `true` de cualquier ámbito                                                                                                      | Honrado incluso cuando una fuente administrada establece `false`                                                                                                                              |
| [`remoteControlAtStartup`](/docs/es/settings-reference#remotecontrolatstartup)       | `false` de `.claude/settings.json` o `.claude/settings.local.json`                                                              | Honrado incluso cuando una fuente administrada establece `true`; un proyecto o local `true` se ignora                                                                                         |
| [`crossSessionInbound`](/docs/es/settings-reference#crosssessioninbound)             | Un valor más estricto de `.claude/settings.json` o `.claude/settings.local.json`, en la escalera `accept` \< `hold` \< `refuse` | Honrado sobre valores administrados, `--settings` y de usuario; un valor de proyecto o local que no es más estricto se ignora                                                                 |
| [`useAutoModeDuringPlan`](/docs/es/settings-reference#useautomodeduringplan)         | `false` de cualquier fuente administrada, `--settings`, `~/.claude/settings.json`, o `.claude/settings.local.json`              | Honrado incluso cuando la fuente administrada ganadora establece `true`; un `false` en `.claude/settings.json` se ignora                                                                      |
| [`syncClaudeAiSkills`](/docs/es/settings-reference#syncclaudeaiskills)               | `false` de cualquier fuente administrada, `--settings`, `~/.claude/settings.json`, o `.claude/settings.local.json`              | Honrado incluso cuando la fuente administrada ganadora establece `true`; un `false` en `.claude/settings.json` se ignora                                                                      |
| [`syncClaudeAiPlugins`](/docs/es/settings-reference#syncclaudeaiplugins)             | `false` de cualquier fuente administrada, `--settings`, `~/.claude/settings.json`, o `.claude/settings.local.json`              | Honrado incluso cuando la fuente administrada ganadora establece `true`; un `false` en `.claude/settings.json` se ignora                                                                      |
| [`maxEffortLevel`](/docs/es/settings-reference#maxeffortlevel)                       | Un límite más bajo de cualquier ámbito, incluyendo `--settings`                                                                 | Honrado incluso cuando las configuraciones administradas que Claude Code aplica establecen un límite más alto; se aplica el límite más bajo. Requiere Claude Code v2.1.267 o posterior        |

Una aplicación que ejecuta Claude Code dentro de sí misma y establece [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/es/env-vars) también es una excepción. Claude Code toma la configuración de modelo de esa aplicación sobre las claves `model`, `fallbackModel`, `modelPicker`, y `modelOverrides` de cada fuente administrada, y sobre las variables de selección de modelo en un bloque `env` administrado, como `ANTHROPIC_MODEL` y la familia `ANTHROPIC_DEFAULT_*_MODEL`. Claude Code mantiene una lista blanca [`availableModels`](/docs/es/settings-reference#availablemodels) administrada en vigor a menos que la aplicación suministre la suya.

<h2 id="settings-in-cloud-sessions">
  Configuraciones en sesiones en la nube
</h2>

Una [sesión en la nube](/docs/es/claude-code-on-the-web) se ejecuta en un [entorno en la nube](/docs/es/cloud-environments) en un clon fresco de su repositorio, no en su máquina. Eso cambia qué configuraciones lo alcanzan:

* **Configuraciones compartidas de proyecto** (`.claude/settings.json`): se leen en una sesión con un repositorio, porque el archivo es parte del clon y la sesión comienza dentro de él. Confirme una configuración allí para aplicarla en esas sesiones. Una sesión con varios repositorios comienza por encima de los clones y lee solo las claves `enabledPlugins` y `extraKnownMarketplaces` de cada `.claude/settings.json` del repositorio, no reglas de permisos, hooks, `env` u otras claves. Los mercados y plugins que esas dos claves declaran aún [no se cargan en una sesión en la nube](/docs/es/cloud-environments#what-carries-over-from-your-setup).
* **Configuraciones de usuario y proyecto local** (`~/.claude/settings.json` y `.claude/settings.local.json`): no se leen. Ambas permanecen en su máquina, y el archivo local no está en el clon.
* **Configuraciones administradas**: solo [configuraciones administradas por servidor](/docs/es/server-managed-settings) alcanzan una sesión en la nube; un archivo `managed-settings.json` o perfil MDM en su dispositivo no. Un [entorno autohospedado](/docs/es/self-hosted-environments) también lee el archivo de configuraciones administradas en su imagen de ejecutor. [Cómo Claude Code combina fuentes administradas](/docs/es/managed-settings#how-claude-code-combines-managed-sources) dice cuándo se aplica ese archivo.
* **`/config`**: en su navegador en claude.ai/code, abre la sección Claude Code de su configuración de claude.ai en lugar de cambiar un valor. Para cambiar una configuración para una sesión en la nube, establezca una [variable de entorno](/docs/es/cloud-environments#set-environment-variables) en el entorno, o en una sesión con un repositorio, confirme la clave en el `.claude/settings.json` de ese repositorio.

[Lo que se transfiere de su configuración](/docs/es/cloud-environments#what-carries-over-from-your-setup) enumera el resto: `CLAUDE.md`, skills, servidores MCP, plugins y credenciales.

<h2 id="what’s-next">
  Qué sigue
</h2>

* [Todas las configuraciones](/docs/es/settings-reference): cada clave, con dónde la establece y un ejemplo
* [Archivos de configuración de ejemplo](/docs/es/settings-example): un archivo personal, un archivo de equipo y un archivo administrado de una organización
* [Configurar permisos](/docs/es/permissions): reglas allow, ask y deny, y qué ejecuta Claude Code sin preguntar
* [Variables de entorno](/docs/es/env-vars): las variables que lee Claude Code y el bloque `env`
* [Depure su configuración](/docs/es/debug-your-config): cuando una configuración no se aplica
* [Referencia del directorio Claude](/docs/es/claude-directory): cada archivo que lee Claude Code, incluyendo subagentes, servidores MCP, plugins y `CLAUDE.md`
