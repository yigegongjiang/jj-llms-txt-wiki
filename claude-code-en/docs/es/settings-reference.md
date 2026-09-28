> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Toda la configuración

> Referencia completa para cada clave settings.json de Claude Code: dónde va cada una, su tipo y valor predeterminado, y un ejemplo listo para pegar, con un índice de cada clave.

export const BackToIndex = ({href = '#all-settings', label = 'Back to index'}) => {
  const [show, setShow] = useState(false);
  useEffect(() => {
    const onScroll = () => setShow(window.scrollY > window.innerHeight);
    onScroll();
    window.addEventListener('scroll', onScroll, {
      passive: true
    });
    return () => window.removeEventListener('scroll', onScroll);
  }, []);
  return <div className="not-prose">
      <style>{`
        .bti-btn {
          position: fixed; right: 20px; bottom: 20px; z-index: 40;
          display: inline-flex; align-items: center; gap: 6px;
          padding: 8px 12px; border-radius: 999px;
          font-size: 13px; font-weight: 500; line-height: 1; text-decoration: none;
          color: #1f1f1f; background: #ffffff; border: 1px solid #d9d9d9;
          box-shadow: 0 2px 8px rgba(0,0,0,0.12);
          opacity: 0; pointer-events: none; transform: translateY(6px);
          transition: opacity 160ms ease, transform 160ms ease;
        }
        .bti-btn.bti-show { opacity: 1; pointer-events: auto; transform: translateY(0); }
        .bti-btn:hover { border-color: #b3b3b3; }
        .dark .bti-btn { color: #ececec; background: #1e1e1e; border-color: #3a3a3a; box-shadow: 0 2px 8px rgba(0,0,0,0.5); }
        .dark .bti-btn:hover { border-color: #5a5a5a; }
        @media (max-width: 1279px) { .bti-btn { bottom: 112px; } }
        @media print { .bti-btn { display: none; } }
      `}</style>
      <a className={'bti-btn' + (show ? ' bti-show' : '')} href={href} aria-hidden={!show} tabIndex={show ? 0 : -1}>
        <svg width="12" height="12" viewBox="0 0 16 16" fill="none" stroke="currentColor" strokeWidth="1.6" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true"><path d="M8 13V3M3.5 7.5 8 3l4.5 4.5" /></svg>
        {label}
      </a>
    </div>;
};

export const ReferenceFilter = ({placeholder, noun, facets, facetOrder, columnHelp, children}) => {
  const useLive = init => {
    const [v, setV] = useState(init);
    const ref = useRef(init);
    return [v, ref, x => {
      ref.current = x;
      setV(x);
    }];
  };
  const cap = s => s.charAt(0).toUpperCase() + s.slice(1);
  const plural = s => s.endsWith('y') ? s.slice(0, -1) + 'ies' : s + 's';
  const facetNames = facets || ['category', 'topic', 'scope', 'where'];
  const orderOf = {};
  Object.keys(facetOrder || ({})).forEach(k => {
    orderOf[k] = facetOrder[k].map(x => String(x).toLowerCase());
  });
  const rankIn = (col, v) => {
    const list = orderOf[col];
    if (!list) return -1;
    const i = list.indexOf(String(v).toLowerCase());
    return i < 0 ? list.length : i;
  };
  const cmpValues = col => (a, b) => {
    const ra = rankIn(col, a);
    const rb = rankIn(col, b);
    if (ra !== rb) return ra - rb;
    return a < b ? -1 : a > b ? 1 : 0;
  };
  const help = columnHelp || ({});
  const FIRST_COL_HELP = 'Click an entry to open it.';
  const optionLabel = (f, c) => c === 'All' ? 'All ' + plural(f.label.toLowerCase()) : c;
  const nounText = noun || 'entries';
  const placeholderText = placeholder || 'Filter this reference';
  const rootRef = useRef(null);
  const tablesRef = useRef(null);
  const searchRef = useRef(null);
  const menuRef = useRef({});
  const [q, qRef, setQ] = useLive('');
  const [sel, selRef, setSel] = useLive({});
  const [sortBy, sortRef, setSortBy] = useLive(null);
  const [menuOpen, menuOpenRef, setMenu] = useLive(null);
  const [facetList, setFacetList] = useState([]);
  const [firstHead, setFirstHead] = useState('');
  const [counts, setCounts] = useState({
    shown: 0,
    total: 0
  });
  const [disabled, setDisabled] = useState(false);
  const menuBtn = name => menuRef.current[name] ? menuRef.current[name].querySelector(':scope > button') : null;
  const menuList = name => menuRef.current[name] ? menuRef.current[name].querySelector('[role="listbox"]') : null;
  const closeMenu = name => {
    setMenu(null);
    const btn = menuBtn(name);
    if (btn) btn.focus();
  };
  const focusSelected = name => {
    const list = menuList(name);
    if (!list) return;
    const btn = list.querySelector('button[aria-selected="true"]') || list.querySelector('button');
    if (btn) btn.focus();
  };
  const setFacet = (name, value) => {
    setSel(Object.assign({}, selRef.current, {
      [name]: value
    }));
    apply(qRef.current);
    closeMenu(name);
  };
  const sortTables = by => {
    (tablesRef.current || []).forEach(tab => {
      const t = tab.el;
      const idx = tab.heads.indexOf(by);
      const body = t.querySelector('tbody');
      if (idx < 0 || !body) return;
      const rows = [...body.querySelectorAll('tr')];
      const keyOf = r => r.children[idx] ? r.children[idx].textContent.trim().toLowerCase() : '';
      const cmp = cmpValues(by);
      rows.map((r, i) => ({
        r,
        i: Number(r.dataset.sfIndex !== undefined ? r.dataset.sfIndex : i),
        k: keyOf(r)
      })).sort((a, b) => cmp(a.k, b.k) || a.i - b.i).forEach(x => body.appendChild(x.r));
      [...t.querySelectorAll('thead th')].forEach((h, i) => {
        const sortable = tab.heads[i] === tab.heads[0] || facetNames.indexOf(tab.heads[i]) > -1;
        if (sortable) h.setAttribute('aria-sort', i === idx ? 'ascending' : 'none'); else h.removeAttribute('aria-sort');
      });
    });
  };
  const scan = () => {
    const tables = [];
    let el = rootRef.current ? rootRef.current.nextElementSibling : null;
    while (el) {
      if (el.tagName === 'H2' || el.querySelector(':scope > h2')) break;
      const found = el.tagName === 'TABLE' ? [el] : [...el.querySelectorAll('table')];
      found.forEach(t => {
        const headCells = [...t.querySelectorAll('thead th, thead td')];
        const heads = headCells.map(h => h.textContent.trim().toLowerCase());
        if (heads.length === 0) return;
        const facetIdx = {};
        heads.forEach((h, i) => {
          if (facetNames.indexOf(h) > -1) facetIdx[h] = i;
        });
        if (!t.dataset.sfDecorated) {
          t.dataset.sfDecorated = '1';
          headCells.forEach((h, i) => {
            const text = i === 0 ? help[heads[0]] || FIRST_COL_HELP : help[heads[i]];
            if (text) h.title = text;
          });
        }
        const rows = [...t.querySelectorAll('tbody tr')].map((r, i) => {
          if (r.dataset.sfIndex === undefined) r.dataset.sfIndex = String(i);
          const cells = r.querySelectorAll('td');
          const fv = {};
          Object.keys(facetIdx).forEach(h => {
            fv[h] = cells[facetIdx[h]] ? cells[facetIdx[h]].textContent.trim() : '';
          });
          return {
            el: r,
            text: [...cells].map(c => c.textContent).join(' ').toLowerCase(),
            facets: fv,
            anchors: [...r.querySelectorAll('a[href^="#"]')].map(a => a.getAttribute('href').slice(1)),
            ids: [...r.querySelectorAll('[id]')].map(n => n.id)
          };
        });
        tables.push({
          el: t,
          box: t.closest('[data-table-wrapper]') || t,
          rows,
          heads
        });
      });
      el = el.nextElementSibling;
    }
    tablesRef.current = tables;
    if (sortRef.current) sortTables(sortRef.current);
    return tables;
  };
  const apply = query => {
    let tables = tablesRef.current || scan();
    if (tables.some(t => !t.el.isConnected)) tables = scan();
    const needle = query.trim().toLowerCase();
    const sel = selRef.current;
    const activeFacets = Object.keys(sel).filter(h => sel[h] && sel[h] !== 'All');
    const show = (el, on) => {
      const want = on ? '' : 'none';
      if (el.style.display !== want) el.style.display = want;
    };
    let total = 0;
    let shown = 0;
    const visibleTargets = new Set();
    tables.forEach(t => {
      let tableVisible = 0;
      t.rows.forEach(row => {
        total += 1;
        const catOk = activeFacets.every(h => {
          const v = row.facets[h];
          return v === sel[h] || v === '' || v === undefined;
        });
        const match = catOk && (needle === '' || row.text.includes(needle));
        show(row.el, match);
        if (match) {
          tableVisible += 1;
          row.anchors.forEach(a => visibleTargets.add(a));
        }
      });
      show(t.box, !(t.rows.length > 0 && tableVisible === 0));
      shown += tableVisible;
    });
    if (shown < total && visibleTargets.size > 0) {
      tables.forEach(t => {
        t.rows.forEach(row => {
          if (row.el.style.display === 'none' && row.ids.some(id => visibleTargets.has(id))) {
            show(row.el, true);
            show(t.box, true);
            shown += 1;
          }
        });
      });
    }
    setCounts({
      shown,
      total
    });
    return total;
  };
  const deriveFacets = tables => {
    const seen = {};
    tables.forEach(t => t.rows.forEach(r => {
      Object.keys(r.facets).forEach(h => {
        if (!seen[h]) seen[h] = [];
        if (r.facets[h] && seen[h].indexOf(r.facets[h]) === -1) seen[h].push(r.facets[h]);
      });
    }));
    const list = facetNames.filter(h => seen[h] && seen[h].length > 0).map(h => ({
      name: h,
      label: cap(h),
      values: seen[h].sort(cmpValues(h))
    }));
    setFacetList(list);
    const first = tables[0] ? tables[0].heads[0] : '';
    setFirstHead(first);
    if (!sortRef.current && first) {
      setSortBy(first);
      sortTables(first);
    }
    const init = {};
    list.forEach(f => {
      init[f.name] = selRef.current[f.name] || 'All';
    });
    setSel(init);
  };
  const onChange = value => {
    setQ(value);
    apply(value);
  };
  const clearAll = () => {
    const next = {};
    Object.keys(selRef.current).forEach(k => {
      next[k] = 'All';
    });
    setSel(next);
    setQ('');
    apply('');
    if (searchRef.current) searchRef.current.focus();
  };
  useEffect(() => {
    const tables = scan();
    deriveFacets(tables);
    const total = apply('');
    let retryTimer;
    if (total === 0) {
      retryTimer = setTimeout(() => {
        tablesRef.current = null;
        if (apply(qRef.current) > 0) deriveFacets(tablesRef.current); else setDisabled(true);
      }, 500);
    }
    const onKey = e => {
      if (e.key === 'Escape' && menuOpenRef.current !== null) closeMenu(menuOpenRef.current);
      if (!searchRef.current) return;
      if (e.metaKey || e.ctrlKey || e.altKey) return;
      const active = document.activeElement;
      const tag = active && active.tagName;
      const editable = active && active.isContentEditable;
      const interactive = tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT' || tag === 'BUTTON' || tag === 'A' || editable || active && active.getAttribute && active.getAttribute('role');
      if (e.key === '/' && !interactive) {
        const r = rootRef.current ? rootRef.current.getBoundingClientRect() : null;
        if (r && r.bottom > 0 && r.top < (window.innerHeight || 0)) {
          e.preventDefault();
          setMenu(null);
          searchRef.current.focus();
        }
      }
      if (e.key === 'Escape' && menuOpenRef.current === null && active === searchRef.current) {
        onChange('');
        searchRef.current.blur();
      }
    };
    const onDocClick = e => {
      const open = menuOpenRef.current;
      if (open !== null && menuRef.current[open] && !menuRef.current[open].contains(e.target)) setMenu(null);
    };
    window.addEventListener('keydown', onKey);
    document.addEventListener('mousedown', onDocClick);
    return () => {
      if (retryTimer) clearTimeout(retryTimer);
      window.removeEventListener('keydown', onKey);
      document.removeEventListener('mousedown', onDocClick);
      (tablesRef.current || []).forEach(t => {
        t.box.style.display = '';
        t.rows.forEach(row => {
          row.el.style.display = '';
        });
      });
    };
  }, []);
  useEffect(() => {
    if (menuOpen !== null) focusSelected(menuOpen);
  }, [menuOpen]);
  if (disabled) return null;
  const facetActive = Object.keys(sel).some(h => sel[h] && sel[h] !== 'All');
  const sortOptions = [firstHead].concat(facetList.map(f => f.name)).filter((h, i, a) => h && a.indexOf(h) === i);
  return <>
      <style>{`
        .sf-root {
          --sf-accent: #D97757;
          --sf-bg: #fff;
          --sf-border: #E8E6DC;
          --sf-text: #141413;
          --sf-text-3: #73726C;
          --sf-text-4: #9C9A92;
        }
        .dark .sf-root {
          --sf-bg: #1a1918;
          --sf-border: #3a3936;
          --sf-text: #e8e6dc;
          --sf-text-3: #9c9a92;
          --sf-text-4: #73726c;
        }
        .sf-root .sf-end {
          position: absolute;
          right: 10px;
          top: 50%;
          transform: translateY(-50%);
        }
        .sf-root .sf-x {
          background: none;
          border: none;
          cursor: pointer;
          color: var(--sf-text-3);
          font-size: 14px;
          padding: 2px 4px;
          line-height: 1;
        }
      `}</style>
      <div ref={rootRef} className="sf-root" style={{
    margin: '16px 0 8px'
  }}>
        <div style={{
    display: 'flex',
    gap: '8px',
    flexWrap: 'wrap',
    alignItems: 'center'
  }}>
        <div style={{
    position: 'relative',
    flex: '1 1 260px',
    maxWidth: '480px'
  }}>
          <input ref={searchRef} value={q} onChange={e => onChange(e.target.value)} placeholder={placeholderText} aria-label={placeholderText} style={{
    width: '100%',
    padding: '8px 56px 8px 12px',
    borderRadius: '8px',
    border: '1px solid var(--sf-border)',
    background: 'var(--sf-bg)',
    color: 'var(--sf-text)',
    fontSize: '14px',
    outline: 'none',
    boxSizing: 'border-box'
  }} />
          {q ? <button type="button" onClick={() => {
    onChange('');
    if (searchRef.current) searchRef.current.focus();
  }} aria-label="Clear text" className="sf-end sf-x">
              ×
            </button> : <span className="sf-end" style={{
    fontFamily: 'var(--font-mono, ui-monospace, monospace)',
    fontSize: '11px',
    color: 'var(--sf-text-4)',
    border: '1px solid var(--sf-border)',
    borderRadius: '3px',
    padding: '0 5px',
    pointerEvents: 'none'
  }}>
              /
            </span>}
        </div>
        {facetList.map(f => {
    const cur = sel[f.name] || 'All';
    const isOpen = menuOpen === f.name;
    return <div key={f.name} ref={el => {
      menuRef.current[f.name] = el;
    }} style={{
      position: 'relative'
    }}>
            <button type="button" onClick={() => setMenu(isOpen ? null : f.name)} onKeyDown={e => {
      if (e.key === 'ArrowDown') {
        e.preventDefault();
        if (!isOpen) setMenu(f.name); else focusSelected(f.name);
      }
    }} aria-haspopup="listbox" aria-expanded={isOpen} style={{
      display: 'flex',
      alignItems: 'center',
      gap: '8px',
      padding: cur !== 'All' ? '8px 30px 8px 12px' : '8px 12px',
      borderRadius: '8px',
      border: '1px solid ' + (cur !== 'All' ? 'var(--sf-accent)' : 'var(--sf-border)'),
      background: 'var(--sf-bg)',
      color: cur === 'All' ? 'var(--sf-text-3)' : 'var(--sf-text)',
      fontSize: '13.5px',
      cursor: 'pointer',
      whiteSpace: 'nowrap',
      maxWidth: '260px'
    }}>
              <span style={{
      overflow: 'hidden',
      textOverflow: 'ellipsis'
    }}>
                {f.label + ': ' + optionLabel(f, cur)}
              </span>
              <span aria-hidden="true" style={{
      fontSize: '9px',
      color: 'var(--sf-text-4)',
      transform: isOpen ? 'rotate(180deg)' : 'none',
      transition: 'transform 120ms'
    }}>
                ▼
              </span>
            </button>
            {cur !== 'All' && <button type="button" onClick={() => setFacet(f.name, 'All')} aria-label={'Clear ' + f.label + ' filter'} title={'Clear ' + f.label + ' filter'} className="sf-end sf-x">
                ×
              </button>}
            {isOpen && <div role="listbox" aria-label={f.label} onKeyDown={e => {
      const items = [...e.currentTarget.querySelectorAll('button')];
      const idx = items.indexOf(document.activeElement);
      if (e.key === 'ArrowDown') {
        e.preventDefault();
        (items[idx + 1] || items[0]).focus();
      } else if (e.key === 'ArrowUp') {
        e.preventDefault();
        (items[idx - 1] || items[items.length - 1]).focus();
      } else if (e.key === 'Home') {
        e.preventDefault();
        if (items[0]) items[0].focus();
      } else if (e.key === 'End') {
        e.preventDefault();
        if (items[items.length - 1]) items[items.length - 1].focus();
      } else if (e.key === 'Tab') {
        closeMenu(f.name);
      }
    }} style={{
      position: 'absolute',
      top: 'calc(100% + 6px)',
      left: 0,
      zIndex: 1000,
      minWidth: '260px',
      maxHeight: '340px',
      overflowY: 'auto',
      background: 'var(--sf-bg)',
      border: '1px solid var(--sf-border)',
      borderRadius: '10px',
      boxShadow: '0 8px 24px rgba(0,0,0,0.12)',
      padding: '5px'
    }}>
                {['All'].concat(f.values).map(c => {
      const selected = cur === c;
      return <button key={c} role="option" aria-selected={selected} tabIndex={-1} onClick={() => setFacet(f.name, c)} style={{
        display: 'flex',
        alignItems: 'center',
        gap: '8px',
        width: '100%',
        textAlign: 'left',
        padding: '7px 10px',
        borderRadius: '6px',
        border: 'none',
        background: 'transparent',
        color: selected ? 'var(--sf-accent)' : 'var(--sf-text)',
        fontWeight: selected ? 600 : 400,
        fontSize: '13.5px',
        cursor: 'pointer'
      }}>
                      <span aria-hidden="true" style={{
        width: '14px',
        color: 'var(--sf-accent)',
        flexShrink: 0
      }}>
                        {selected ? '✓' : ''}
                      </span>
                      {optionLabel(f, c)}
                    </button>;
    })}
              </div>}
          </div>;
  })}
        {sortOptions.length > 1 && <div role="group" aria-label="Sort by" style={{
    display: 'flex',
    alignItems: 'center',
    gap: '4px',
    fontSize: '13px',
    color: 'var(--sf-text-3)',
    whiteSpace: 'nowrap'
  }}>
            <span style={{
    marginRight: '4px'
  }}>Sort by</span>
            {sortOptions.map(o => {
    const on = sortBy === o;
    return <button key={o} type="button" aria-pressed={on} onClick={() => {
      setSortBy(o);
      sortTables(o);
    }} style={{
      padding: '6px 10px',
      borderRadius: '8px',
      border: '1px solid ' + (on ? 'var(--sf-accent)' : 'var(--sf-border)'),
      background: 'var(--sf-bg)',
      color: on ? 'var(--sf-text)' : 'var(--sf-text-3)',
      fontSize: '13px',
      cursor: 'pointer'
    }}>
                  {o.charAt(0).toUpperCase() + o.slice(1)}
                </button>;
  })}
          </div>}
        </div>
        <div aria-live="polite" style={{
    margin: '8px 0 0',
    fontSize: '13px',
    color: 'var(--sf-text-3)',
    minHeight: '1px'
  }}>
          {q.trim() === '' && !facetActive ? <>{counts.total} {nounText}</> : counts.shown === 0 ? <>
                {q.trim() === '' ? 'No ' + nounText + ' match the selected filters.' : facetActive ? 'No ' + nounText + ' match \u201c' + q + '\u201d with the selected filters.' : 'No ' + nounText + ' match \u201c' + q + '\u201d.'}{' '}
                <button type="button" onClick={clearAll} style={{
    background: 'none',
    border: 'none',
    padding: 0,
    color: 'var(--sf-accent)',
    cursor: 'pointer',
    font: 'inherit',
    textDecoration: 'underline'
  }}>
                  Clear filters
                </button>
                {children ? <> {children}</> : null}
              </> : <>
                Showing {counts.shown} of {counts.total} {nounText}
              </>}
        </div>
      </div>
    </>;
};

<BackToIndex href="#all-settings" label="Volver al índice" />

Esta página de referencia enumera cada clave que Claude Code lee desde un archivo de configuración, más el [grupo corto de claves](#global-config-settings) que mantiene en `~/.claude.json` en su lugar. Para elegir un archivo o verificar la precedencia, comience con [Archivos de configuración y precedencia](/docs/es/settings).

<span id="available-settings" />

<span id="scopes" />

<span id="all-settings" />

<h2 id="settings-index">
  Índice de configuración
</h2>

Cada clave a continuación enlaza a su entrada. El alcance enumera los [archivos](/docs/es/settings#settings-files-and-who-they-affect) en los que puede ir: `User` es `~/.claude/settings.json`, `Project` es `.claude/settings.json`, `Local` es `.claude/settings.local.json`, y `Managed` es [lo que su organización implementa](/docs/es/managed-settings). `Any file` significa los cuatro, y `Global config` significa [`~/.claude.json`](#global-config-settings).

<ReferenceFilter
  noun="settings"
  placeholder="Filter settings by key or purpose"
  facetOrder={{ scope: ["Any file", "User, local, or managed", "User or managed", "Managed", "Global config"] }}
  columnHelp={{
topic: "The section of this page that holds the entry. Use Sort by to group the table by topic.",
scope: "Which settings files can set the key: user (~/.claude/settings.json), project (.claude/settings.json), local (.claude/settings.local.json), or managed (deployed by your organization). Global config keys are in ~/.claude.json instead.",
}}
/>

| Clave                                                                                                 | Descripción                                                                                                                                                                                                                                                        | Tema                                     | Alcance                       |
| :---------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------- | :---------------------------- |
| [`advisorModel`](#advisormodel)                                                                       | Elija qué modelo responde cuando Claude pregunta la [herramienta asesor](/docs/es/advisor)                                                                                                                                                                              | Modelo y respuestas                      | Cualquier archivo             |
| [`agent`](#agent)                                                                                     | Inicie cada sesión como un [subagente](/docs/es/sub-agents) nombrado con su indicación, herramientas y modelo                                                                                                                                                           | Agentes, sesiones y worktrees            | Cualquier archivo             |
| [`agentPushNotifEnabled`](#agentpushnotifenabled)                                                     | Permita que Claude envíe una [notificación push a su teléfono](/docs/es/remote-control#mobile-push-notifications) cuando lo decida                                                                                                                                      | Remoto, escritorio y notificaciones      | Cualquier archivo             |
| [`allowAllClaudeAiMcps`](#allowallclaudeaimcps)                                                       | Cargue los [conectores de claude.ai](/docs/es/mcp) que Claude Code obtiene por sí mismo junto con un [`managed-mcp.json`](/docs/es/managed-mcp#exclusive-control-with-managed-mcp-json) implementado                                                                         | MCP                                      | Administrado                  |
| [`allowedChannelPlugins`](#allowedchannelplugins)                                                     | Reemplace la lista de permitidos predeterminada de [plugins de canal](/docs/es/channels#restrict-which-channel-plugins-can-run) que pueden enviar mensajes                                                                                                              | Plugins y skills                         | Administrado                  |
| [`allowedHttpHookUrls`](#allowedhttphookurls)                                                         | Limite qué URLs pueden dirigirse a los [hooks HTTP](/docs/es/hooks)                                                                                                                                                                                                     | Hooks y automatización                   | Cualquier archivo             |
| [`allowedMcpServers`](#allowedmcpservers)                                                             | Lista de permitidos de qué [servidores MCP](/docs/es/mcp) pueden agregar los usuarios                                                                                                                                                                                   | MCP                                      | Cualquier archivo             |
| [`allowManagedHooksOnly`](#allowmanagedhooksonly)                                                     | Ejecute solo los [hooks](/docs/es/hooks) que su organización implementa                                                                                                                                                                                                 | Hooks y automatización                   | Administrado                  |
| [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly)                                           | Haga que la lista de permitidos de [MCP](/docs/es/mcp) administrada sea la única que se aplique                                                                                                                                                                         | MCP                                      | Administrado                  |
| [`allowManagedPermissionRulesOnly`](#allowmanagedpermissionrulesonly)                                 | Haga que [configuración administrada](/docs/es/managed-settings) sea la única fuente de configuración de [reglas de permisos](/docs/es/permissions#managed-settings)                                                                                                         | Configuración de permisos                | Administrado                  |
| [`alwaysThinkingEnabled`](#alwaysthinkingenabled)                                                     | Desactive el [pensamiento extendido](/docs/es/model-config#extended-thinking) para cada sesión                                                                                                                                                                          | Modelo y respuestas                      | Cualquier archivo             |
| [`apiKeyHelper`](#apikeyhelper)                                                                       | Genere la [credencial API](/docs/es/authentication#credential-management) con su propio comando                                                                                                                                                                         | Autenticación y proveedores              | Cualquier archivo             |
| [`askUserQuestionTimeout`](#askuserquestiontimeout)                                                   | Permita que una pregunta sin respuesta [continúe automáticamente](/docs/es/tools-reference#question-auto-continue-timeout) después del tiempo de inactividad                                                                                                            | Interfaz y terminal                      | Usuario o administrado        |
| [`attribution`](#attribution)                                                                         | Personalice la atribución que Claude Code agrega a commits y solicitudes de extracción                                                                                                                                                                             | Git y atribución                         | Cualquier archivo             |
| [`attribution.commit`](#attribution-commit)                                                           | Cambie u oculte el tráiler que Claude Code agrega a los commits                                                                                                                                                                                                    | Git y atribución                         | Cualquier archivo             |
| [`attribution.pr`](#attribution-pr)                                                                   | Cambie u oculte la línea de atribución en las descripciones de solicitudes de extracción                                                                                                                                                                           | Git y atribución                         | Cualquier archivo             |
| [`attribution.sessionUrl`](#attribution-sessionurl)                                                   | Omita el enlace de sesión de claude.ai de los commits de [nube](/docs/es/claude-code-on-the-web) y [Control Remoto](/docs/es/remote-control)                                                                                                                                 | Git y atribución                         | Cualquier archivo             |
| [`autoCompactEnabled`](#autocompactenabled)                                                           | Desactive o active la [compactación automática](/docs/es/context-window)                                                                                                                                                                                                | Memoria y contexto                       | Cualquier archivo             |
| [`autoCompactWindow`](#autocompactwindow)                                                             | Establezca qué tan lleno se llena el contexto antes de que Claude Code [compacte](/docs/es/context-window)                                                                                                                                                              | Memoria y contexto                       | Cualquier archivo             |
| [`autoConnectIde`](#autoconnectide)                                                                   | Conéctese automáticamente a un IDE [VS Code](/docs/es/vs-code) o [JetBrains](/docs/es/jetbrains#from-external-terminals) en ejecución desde una terminal externa                                                                                                             | Configuración global                     | Configuración global          |
| [`autoContinueAtUsageLimit`](#autocontinueatusagelimit)                                               | Espere en la sesión abierta y [continúe la tarea automáticamente](/docs/es/interactive-mode#wait-for-a-usage-limit-to-reset) después de que se restablezca un límite de uso de claude.ai                                                                                | Interfaz y terminal                      | Usuario o administrado        |
| [`autoInstallIdeExtension`](#autoinstallideextension)                                                 | Desactive la instalación automática de la [extensión IDE](/docs/es/vs-code#install-the-extension) desde una terminal de VS Code                                                                                                                                         | Configuración global                     | Configuración global          |
| [`autoMemoryDirectory`](#automemorydirectory)                                                         | Almacene [memoria automática](/docs/es/memory#auto-memory) en un directorio que elija                                                                                                                                                                                   | Memoria y contexto                       | Cualquier archivo             |
| [`autoMemoryEnabled`](#automemoryenabled)                                                             | Desactive o active la [memoria automática](/docs/es/memory#auto-memory)                                                                                                                                                                                                 | Memoria y contexto                       | Cualquier archivo             |
| [`autoMode`](#automode)                                                                               | Agregue sus propias reglas de permitir y denegar al clasificador de [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode)                                                                                                                       | Configuración de permisos                | Usuario o administrado        |
| [`autoMode.classifyAllShell`](#automode-classifyallshell)                                             | Envíe cada comando shell a través del [clasificador de modo automático](/docs/es/permission-modes#what-the-classifier-blocks-by-default), incluso los que coinciden con una regla de permitir estrecha                                                                  | Configuración de permisos                | Usuario o administrado        |
| [`autoScrollEnabled`](#autoscrollenabled)                                                             | [Siga la nueva salida](/docs/es/fullscreen#auto-follow) hasta el final en la representación de pantalla completa                                                                                                                                                        | Interfaz y terminal                      | Cualquier archivo             |
| [`autoUpdatesChannel`](#autoupdateschannel)                                                           | Siga el [canal de lanzamiento](/docs/es/setup#configure-release-channel) estable en lugar del más reciente                                                                                                                                                              | Actualizaciones y versiones              | Cualquier archivo             |
| [`availableModels`](#availablemodels)                                                                 | [Restrinja qué modelos](/docs/es/model-config#restrict-model-selection) pueden elegir las personas                                                                                                                                                                      | Modelo y respuestas                      | Cualquier archivo             |
| [`awaySummaryEnabled`](#awaysummaryenabled)                                                           | Desactive el [resumen de sesión](/docs/es/interactive-mode#session-recap) que se muestra cuando regresa a la terminal                                                                                                                                                   | Remoto, escritorio y notificaciones      | Cualquier archivo             |
| [`awsAuthRefresh`](#awsauthrefresh)                                                                   | Actualice las [credenciales de Bedrock](/docs/es/amazon-bedrock#advanced-credential-configuration) expiradas en `.aws` con su propio comando                                                                                                                            | Autenticación y proveedores              | Cualquier archivo             |
| [`awsCredentialExport`](#awscredentialexport)                                                         | Suministre [credenciales de Bedrock](/docs/es/amazon-bedrock#advanced-credential-configuration) como JSON desde su propio comando                                                                                                                                       | Autenticación y proveedores              | Cualquier archivo             |
| [`axScreenReader`](#axscreenreader)                                                                   | Represente [salida amigable con lectores de pantalla](/docs/es/accessibility)                                                                                                                                                                                           | Interfaz y terminal                      | Cualquier archivo             |
| [`bashEditDiffEnabled`](#basheditdiffenabled)                                                         | Registre los [archivos que cambió un comando Bash](/docs/es/hooks#bash) en cada modo de permisos                                                                                                                                                                        | Interfaz y terminal                      | Usuario o administrado        |
| [`bashOutputMaxChars`](#bashoutputmaxchars)                                                           | Establezca cuánta [salida](/docs/es/tools-reference#output-limits) de un comando exitoso recibe Claude en línea                                                                                                                                                         | Memoria y contexto                       | Cualquier archivo             |
| [`blockedMarketplaces`](#blockedmarketplaces)                                                         | Bloquee [fuentes de marketplace de plugins](/docs/es/plugins/overview) para su organización                                                                                                                                                                             | Plugins y skills                         | Administrado                  |
| [`browserExternalPageTools`](#browserexternalpagetools)                                               | Mantenga las herramientas de Claude fuera de las páginas externas en el panel del navegador [escritorio](/docs/es/desktop)                                                                                                                                              | Herramientas                             | Administrado                  |
| [`channelsEnabled`](#channelsenabled)                                                                 | Permita [canales](/docs/es/channels#enable-channels-for-your-organization) para su organización                                                                                                                                                                         | Plugins y skills                         | Administrado                  |
| [`claudeMd`](#claudemd)                                                                               | Inyecte instrucciones de [CLAUDE.md](/docs/es/memory#deploy-organization-wide-claude-md) en toda la organización desde configuración administrada                                                                                                                       | Memoria y contexto                       | Administrado                  |
| [`claudeMdExcludes`](#claudemdexcludes)                                                               | Omita archivos específicos de [CLAUDE.md](/docs/es/memory#exclude-specific-claude-md-files) cuando se carga la memoria                                                                                                                                                  | Memoria y contexto                       | Cualquier archivo             |
| [`cleanupPeriodDays`](#cleanupperioddays)                                                             | Elija cuántos días Claude Code mantiene [transcripciones](/docs/es/data-usage#data-retention) antes de eliminarlas                                                                                                                                                      | Privacidad y telemetría                  | Cualquier archivo             |
| [`companyAnnouncements`](#companyannouncements)                                                       | Muestre los anuncios de su organización al inicio                                                                                                                                                                                                                  | Interfaz y terminal                      | Cualquier archivo             |
| [`copyOnSelect`](#copyonselect)                                                                       | Desactive la copia automática del texto que selecciona con el ratón en la [representación de pantalla completa](/docs/es/fullscreen#use-the-mouse) y vista de agente                                                                                                    | Configuración global                     | Configuración global          |
| [`crossSessionInbound`](#crosssessioninbound)                                                         | Elija si Claude Code entrega [mensajes de sus otras sesiones](/docs/es/cross-session-messaging#control-inbound-messages), muestra un aviso sin entregarlos, o los rechaza                                                                                               | Agentes, sesiones y worktrees            | Cualquier archivo             |
| [`defaultShell`](#defaultshell)                                                                       | Elija si Bash o PowerShell ejecuta los comandos shell que escribe con el prefijo [`!`](/docs/es/interactive-mode#shell-mode-with-prefix)                                                                                                                                | Interfaz y terminal                      | Cualquier archivo             |
| [`deniedMcpServers`](#deniedmcpservers)                                                               | Bloquee [servidores MCP](/docs/es/mcp) específicos por URL, comando o nombre                                                                                                                                                                                            | MCP                                      | Cualquier archivo             |
| [`desktopSessionCleanupPeriodDays`](#desktopsessioncleanupperioddays)                                 | Establezca un límite de antigüedad en días para [transcripciones de Claude Desktop y Cowork](/docs/es/claude-directory#cleaned-up-automatically)                                                                                                                        | Privacidad y telemetría                  | Usuario o administrado        |
| [`dialogExpiry`](#dialogexpiry)                                                                       | Establezca cuánto tiempo Claude Code espera a que [Control Remoto](/docs/es/remote-control) o un host SDK responda a un diálogo reenviado antes de cancelarlo                                                                                                           | Interfaz y terminal                      | Usuario o administrado        |
| [`diffTool`](#difftool)                                                                               | Elija si los cambios de archivo propuestos por Claude se abren en el visor de diferencias de [VS Code](/docs/es/vs-code) o [JetBrains](/docs/es/jetbrains#features) o permanecen en la terminal                                                                              | Configuración global                     | Configuración global          |
| [`disableAgentView`](#disableagentview)                                                               | Desactive los agentes de fondo y la [vista de agente](/docs/es/agent-view)                                                                                                                                                                                              | Agentes, sesiones y worktrees            | Cualquier archivo             |
| [`disableAllHooks`](#disableallhooks)                                                                 | Desactive [hooks](/docs/es/hooks), una [línea de estado](/docs/es/statusline) personalizada, y un comando de [sugerencia de archivo `@`](/docs/es/interactive-mode#quick-commands) personalizado de una vez                                                                       | Hooks y automatización                   | Cualquier archivo             |
| [`disableArtifact`](#disableartifact)                                                                 | Obsoleto; use `enableArtifact` para desactivar la [herramienta Artifact](/docs/es/artifacts)                                                                                                                                                                            | Remoto, escritorio y notificaciones      | Cualquier archivo             |
| [`disableAutoMode`](#disableautomode)                                                                 | Elimine el [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) del ciclo de modo de permisos                                                                                                                                                  | Configuración de permisos                | Cualquier archivo             |
| [`disableBrowserExternalNavigation`](#disablebrowserexternalnavigation)                               | Limite el panel del navegador [escritorio](/docs/es/desktop) a localhost para personas y Claude                                                                                                                                                                         | Herramientas                             | Administrado                  |
| [`disableBundledSkills`](#disablebundledskills)                                                       | Desactive los [skills](/docs/es/skills#bundled-skills) y [flujos de trabajo](/docs/es/workflows) incluidos con Claude Code                                                                                                                                                   | Plugins y skills                         | Cualquier archivo             |
| [`disableClaudeAiConnectors`](#disableclaudeaiconnectors)                                             | Desactive los [conectores de claude.ai](/docs/es/mcp#disable-claude-ai-connectors) para que Claude Code no los obtenga                                                                                                                                                  | MCP                                      | Cualquier archivo             |
| [`disableCommandPluginSources`](#disablecommandpluginsources)                                         | Bloquee [plugins](/docs/es/plugins/overview) que se instalan ejecutando un comando declarado por marketplace                                                                                                                                                            | Plugins y skills                         | Administrado                  |
| [`disableDeepLinkRegistration`](#disabledeeplinkregistration)                                         | Impida que Claude Code registre el controlador [`claude-cli://`](/docs/es/deep-links)                                                                                                                                                                                   | Remoto, escritorio y notificaciones      | Cualquier archivo             |
| [`disableDesktopLocalSessions`](#disabledesktoplocalsessions)                                         | Desactive las [sesiones de Desktop Code](/docs/es/desktop#local-sessions-on-managed-devices) que se ejecutan en el dispositivo, dejando SSH para otros hosts y nube                                                                                                     | Remoto, escritorio y notificaciones      | Administrado                  |
| [`disabledMcpjsonServers`](#disabledmcpjsonservers)                                                   | Rechace servidores específicos del [`.mcp.json`](/docs/es/mcp#project-scope) de un proyecto                                                                                                                                                                             | MCP                                      | Cualquier archivo             |
| [`disableMobileSimulatorTools`](#disablemobilesimulatortools)                                         | Bloquee las herramientas de Claude en el panel del simulador de iOS [escritorio](/docs/es/desktop)                                                                                                                                                                      | Herramientas                             | Administrado                  |
| [`disableRemoteControl`](#disableremotecontrol)                                                       | Desactive [Control Remoto](/docs/es/remote-control) en todas partes donde pueda iniciarse                                                                                                                                                                               | Remoto, escritorio y notificaciones      | Cualquier archivo             |
| [`disableSideloadFlags`](#disablesideloadflags)                                                       | Rechace las banderas CLI que cargan [plugins](/docs/es/plugins/overview), [subagentes](/docs/es/sub-agents), y [servidores MCP](/docs/es/mcp)                                                                                                                                     | Configuración empresarial y administrada | Administrado                  |
| [`disableSkillShellExecution`](#disableskillshellexecution)                                           | Impida que [skills](/docs/es/skills) y comandos personalizados ejecuten shell en línea                                                                                                                                                                                  | Plugins y skills                         | Cualquier archivo             |
| [`disableWorkflows`](#disableworkflows)                                                               | Desactive los [flujos de trabajo dinámicos](/docs/es/workflows) para todos; use `enableWorkflows` para usted mismo                                                                                                                                                      | Hooks y automatización                   | Cualquier archivo             |
| [`editorMode`](#editormode)                                                                           | Use [atajos de teclado vim](/docs/es/interactive-mode#vim-editor-mode) en el indicador de entrada                                                                                                                                                                       | Interfaz y terminal                      | Cualquier archivo             |
| [`effortLevel`](#effortlevel)                                                                         | Establezca un [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) predeterminado para modelos sin un nivel guardado propio                                                                                                                                   | Modelo y respuestas                      | Cualquier archivo             |
| [`emojiCompletionEnabled`](#emojicompletionenabled)                                                   | Desactive las [sugerencias y reemplazo de emoji `:shortcode:`](/docs/es/interactive-mode#emoji-shortcodes) en la entrada del indicador                                                                                                                                  | Interfaz y terminal                      | Cualquier archivo             |
| [`enableAllProjectMcpServers`](#enableallprojectmcpservers)                                           | Apruebe cada servidor en archivos [`.mcp.json`](/docs/es/mcp#project-server-approvals-and-workspace-trust) de proyecto sin un indicador                                                                                                                                 | MCP                                      | Cualquier archivo             |
| [`enableArtifact`](#enableartifact)                                                                   | Desactive la [herramienta Artifact](/docs/es/artifacts) con un `false` en cualquier archivo; ningún archivo puede activarla de nuevo                                                                                                                                    | Remoto, escritorio y notificaciones      | Cualquier archivo             |
| [`enabledMcpjsonServers`](#enabledmcpjsonservers)                                                     | Apruebe servidores específicos del [`.mcp.json`](/docs/es/mcp#project-server-approvals-and-workspace-trust) de un proyecto                                                                                                                                              | MCP                                      | Cualquier archivo             |
| [`enabledPlugins`](#enabledplugins)                                                                   | Active o desactive [plugins](/docs/es/plugins/overview) individuales por alcance                                                                                                                                                                                        | Plugins y skills                         | Cualquier archivo             |
| [`enableWorkflows`](#enableworkflows)                                                                 | Active o desactive los [flujos de trabajo dinámicos](/docs/es/workflows) contra el predeterminado de su plan                                                                                                                                                            | Hooks y automatización                   | Cualquier archivo             |
| [`enforceAvailableModels`](#enforceavailablemodels)                                                   | Mantenga la [opción Predeterminada de `/model`](/docs/es/model-config#enforce-the-allowlist-for-the-default-model) dentro de su lista de permitidos `availableModels`                                                                                                   | Modelo y respuestas                      | Cualquier archivo             |
| [`env`](#env)                                                                                         | Establezca [variables de entorno](/docs/es/env-vars#in-settings-files) para cada sesión y sus subprocesos                                                                                                                                                               | Memoria y contexto                       | Cualquier archivo             |
| [`externalEditorContext`](#externaleditorcontext)                                                     | Muestre la última respuesta de Claude como comentarios cuando presione [Ctrl+G](/docs/es/interactive-mode#general-controls) para editar                                                                                                                                 | Configuración global                     | Configuración global          |
| [`extraKnownMarketplaces`](#extraknownmarketplaces)                                                   | Registre [marketplaces](/docs/es/plugins/overview) para un repositorio u organización                                                                                                                                                                                   | Plugins y skills                         | Cualquier archivo             |
| [`fallbackModel`](#fallbackmodel)                                                                     | Nombre [modelos de respaldo](/docs/es/model-config#fallback-model-chains) para cuando el primario está sobrecargado                                                                                                                                                     | Modelo y respuestas                      | Cualquier archivo             |
| [`fastMode`](#fastmode)                                                                               | Active el [modo rápido](/docs/es/fast-mode) para sesiones donde está disponible                                                                                                                                                                                         | Modelo y respuestas                      | Cualquier archivo             |
| [`fastModePerSessionOptIn`](#fastmodepersessionoptin)                                                 | Requiera que las personas activen el [modo rápido](/docs/es/fast-mode) en cada sesión                                                                                                                                                                                   | Modelo y respuestas                      | Cualquier archivo             |
| [`feedbackDrafts`](#feedbackdrafts)                                                                   | Controle si Claude pone en cola [borradores de comentarios](/docs/es/tools-reference#sendfeedback-tool-behavior) para que revise                                                                                                                                        | Privacidad y telemetría                  | Usuario o administrado        |
| [`feedbackSurveyRate`](#feedbacksurveyrate)                                                           | Cambie la frecuencia con la que aparece la [encuesta de calidad de sesión](/docs/es/data-usage#session-quality-surveys)                                                                                                                                                 | Privacidad y telemetría                  | Cualquier archivo             |
| [`fileCheckpointingEnabled`](#filecheckpointingenabled)                                               | Desactive o active las instantáneas de archivo que [`/rewind`](/docs/es/checkpointing) restaura                                                                                                                                                                         | Memoria y contexto                       | Cualquier archivo             |
| [`fileSuggestion`](#filesuggestion)                                                                   | Suministre [autocompletado de archivo `@`](/docs/es/interactive-mode#quick-commands) desde su propio comando                                                                                                                                                            | Interfaz y terminal                      | Cualquier archivo             |
| [`footerLinksRegexes`](#footerlinksregexes)                                                           | Haga que los ID de problema o revisión en la salida sean [enlaces clickeables](/docs/es/statusline#clickable-links) debajo del cuadro de entrada                                                                                                                        | Interfaz y terminal                      | Usuario o administrado        |
| [`forceLoginGatewayUrl`](#forcelogingatewayurl)                                                       | Establezca la [URL de puerta de enlace](/docs/es/claude-apps-gateway#set-the-gateway-url) a la que se conecta la pantalla de inicio de sesión                                                                                                                           | Autenticación y proveedores              | Administrado                  |
| [`forceLoginMethod`](#forceloginmethod)                                                               | [Restrinja el inicio de sesión](/docs/es/authentication#restrict-login-to-your-organization) a claude.ai, Claude Console, o una [puerta de enlace en la nube](/docs/es/claude-apps-gateway)                                                                                  | Autenticación y proveedores              | Cualquier archivo             |
| [`forceLoginOrgUUID`](#forceloginorguuid)                                                             | [Fije los inicios de sesión de claude.ai a su organización](/docs/es/authentication#restrict-login-to-your-organization); solo una fuente administrada lo aplica                                                                                                        | Autenticación y proveedores              | Cualquier archivo             |
| [`forceRemoteSettingsRefresh`](#forceremotesettingsrefresh)                                           | Bloquee el inicio hasta que la [configuración administrada por servidor](/docs/es/server-managed-settings) se obtenga recientemente                                                                                                                                     | Configuración empresarial y administrada | Administrado                  |
| [`gatewayInternalNetworks`](#gatewayinternalnetworks)                                                 | Permita que `/login` alcance una [puerta de enlace en la nube](/docs/es/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) en espacio IPv4 público que su organización usa internamente                                                               | Autenticación y proveedores              | Administrado                  |
| [`gcpAuthRefresh`](#gcpauthrefresh)                                                                   | Actualice las [credenciales de Google Cloud](/docs/es/google-vertex-ai#advanced-credential-configuration) con su propio comando                                                                                                                                         | Autenticación y proveedores              | Cualquier archivo             |
| [`hooks`](#hooks)                                                                                     | Ejecute sus propios comandos como [hooks](/docs/es/hooks) en puntos del ciclo de vida de Claude Code                                                                                                                                                                    | Hooks y automatización                   | Cualquier archivo             |
| [`httpHookAllowedEnvVars`](#httphookallowedenvvars)                                                   | Limite qué variables de entorno pueden poner los [hooks HTTP](/docs/es/hooks) en encabezados                                                                                                                                                                            | Hooks y automatización                   | Cualquier archivo             |
| [`includeCoAuthoredBy`](#includecoauthoredby)                                                         | Obsoleto; use `attribution` para ocultar o cambiar la atribución de commit y PR                                                                                                                                                                                    | Git y atribución                         | Cualquier archivo             |
| [`includeGitInstructions`](#includegitinstructions)                                                   | Elimine las instrucciones de commit y PR integradas del contexto de Claude                                                                                                                                                                                         | Git y atribución                         | Cualquier archivo             |
| [`inputNeededNotifEnabled`](#inputneedednotifenabled)                                                 | Obtenga una [notificación push](/docs/es/remote-control#mobile-push-notifications) cuando Claude esté esperando por usted                                                                                                                                               | Remoto, escritorio y notificaciones      | Cualquier archivo             |
| [`isolatePeerMachines`](#isolatepeermachines)                                                         | Pregúntele antes de que Claude [envíe un mensaje a una de sus sesiones en otra máquina](/docs/es/cross-session-messaging#require-approval-for-cross-machine-messages)                                                                                                   | Agentes, sesiones y worktrees            | Cualquier archivo             |
| [`keybindingFlavor`](#keybindingflavor)                                                               | Obsoleto y sin efecto; los atajos de edición de palabras siempre [siguen convenciones readline](/docs/es/interactive-mode#make-ctrl-w-delete-back-to-whitespace)                                                                                                        | Interfaz y terminal                      | Cualquier archivo             |
| [`language`](#language)                                                                               | Haga que Claude responda en un idioma distinto al inglés                                                                                                                                                                                                           | Modelo y respuestas                      | Cualquier archivo             |
| [`managedMcpServers`](#managedmcpservers)                                                             | Proporcione [servidores MCP](/docs/es/managed-mcp#provide-servers-through-managed-settings) remotos a cada usuario junto con los que agregan                                                                                                                            | MCP                                      | Administrado                  |
| [`managedSourcesBehavior`](#managedsourcesbehavior)                                                   | Componga cada [fuente administrada](/docs/es/managed-settings#how-claude-code-combines-managed-sources) que implemente en lugar de usar solo la de mayor prioridad                                                                                                      | Configuración empresarial y administrada | Administrado                  |
| [`maxEffortLevel`](#maxeffortlevel)                                                                   | Limite el [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) para cada modelo o por modelo, en cada proveedor                                                                                                                                               | Modelo y respuestas                      | Cualquier archivo             |
| [`minimumVersion`](#minimumversion)                                                                   | Mantenga las [actualizaciones automáticas](/docs/es/setup#pin-a-minimum-version) de instalar nada por debajo de una versión                                                                                                                                             | Actualizaciones y versiones              | Cualquier archivo             |
| [`model`](#model)                                                                                     | Cambie el [modelo](/docs/es/model-config#set-a-default-model-for-new-sessions) con el que Claude Code comienza                                                                                                                                                          | Modelo y respuestas                      | Cualquier archivo             |
| [`modelOverrides`](#modeloverrides)                                                                   | [Asigne ID de modelo](/docs/es/model-config#override-model-ids-per-version) a los ID de su proveedor, como ARN de Bedrock                                                                                                                                               | Modelo y respuestas                      | Cualquier archivo             |
| [`modelPicker`](#modelpicker)                                                                         | Elija qué modelos enumera el selector [`/model`](/docs/es/model-config#available-models), en su propio orden y con sus propias etiquetas                                                                                                                                | Modelo y respuestas                      | Usuario o administrado        |
| [`modelPricing`](#modelpricing)                                                                       | Informe el gasto a las tasas contratadas de su organización en lugar del precio de lista                                                                                                                                                                           | Modelo y respuestas                      | Administrado                  |
| [`modelSettings`](#modelsettings)                                                                     | Mantenga un [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) guardado por modelo, o limite el esfuerzo de un modelo                                                                                                                                       | Modelo y respuestas                      | Cualquier archivo             |
| [`otelHeadersHelper`](#otelheadershelper)                                                             | Genere encabezados [OpenTelemetry](/docs/es/monitoring-usage#dynamic-headers) rotativos con su propio comando                                                                                                                                                           | Autenticación y proveedores              | Cualquier archivo             |
| [`outputStyle`](#outputstyle)                                                                         | Cambie el rol, tono y formato de salida de Claude con un [estilo de salida](/docs/es/output-styles)                                                                                                                                                                     | Modelo y respuestas                      | Cualquier archivo             |
| [`parentSettingsBehavior`](#parentsettingsbehavior)                                                   | Aplique o suelte restricciones que un [host SDK o IDE](/docs/es/managed-settings#let-an-embedding-host-add-policy) pasa cuando implementa [configuración administrada](/docs/es/managed-settings)                                                                            | Configuración empresarial y administrada | Administrado                  |
| [`permissionExplainerEnabled`](#permissionexplainerenabled)                                           | Eliminado en v2.1.257, junto con la explicación del comando `Ctrl+E` en indicadores de permisos de shell                                                                                                                                                           | Configuración global                     | Configuración global          |
| [`permissions`](#permissions)                                                                         | Establezca reglas de permitir, preguntar y denegar y el [modo de permisos](/docs/es/permission-modes) inicial                                                                                                                                                           | Configuración de permisos                | Cualquier archivo             |
| [`permissions.additionalDirectories`](#permissions-additionaldirectories)                             | Dé a Claude acceso a archivos en [directorios fuera del actual](/docs/es/permissions#working-directories)                                                                                                                                                               | Configuración de permisos                | Cualquier archivo             |
| [`permissions.allow`](#permissions-allow)                                                             | Apruebe [usos de herramientas](/docs/es/permissions#permission-rule-syntax) enumerados sin un indicador                                                                                                                                                                 | Configuración de permisos                | Cualquier archivo             |
| [`permissions.ask`](#permissions-ask)                                                                 | Siempre pregunte antes de [usos de herramientas](/docs/es/permissions#permission-rule-syntax) enumerados                                                                                                                                                                | Configuración de permisos                | Cualquier archivo             |
| [`permissions.blockReadsOutsideWorkingDirectories`](#permissions-blockreadsoutsideworkingdirectories) | Haga que las herramientas de archivo rechacen lecturas fuera de los [directorios de trabajo](/docs/es/permissions#working-directories) en cada modo de permisos                                                                                                         | Configuración de permisos                | Cualquier archivo             |
| [`permissions.defaultMode`](#permissions-defaultmode)                                                 | Establezca el [modo de permisos](/docs/es/permission-modes#which-mode-a-session-starts-in) en el que comienzan las nuevas sesiones                                                                                                                                      | Configuración de permisos                | Cualquier archivo             |
| [`permissions.deny`](#permissions-deny)                                                               | Bloquee [usos de herramientas](/docs/es/permissions#permission-rule-syntax) enumerados, incluidas lecturas de archivos que contienen secretos                                                                                                                           | Configuración de permisos                | Cualquier archivo             |
| [`permissions.disableBypassPermissionsMode`](#permissions-disablebypasspermissionsmode)               | Impida que alguien entre en el [modo bypassPermissions](/docs/es/permission-modes#skip-all-checks-with-bypasspermissions-mode)                                                                                                                                          | Configuración de permisos                | Cualquier archivo             |
| [`plansDirectory`](#plansdirectory)                                                                   | Elija dónde [Plan Mode](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode) escribe archivos de plan                                                                                                                                                      | Memoria y contexto                       | Cualquier archivo             |
| [`pluginConfigs`](#pluginconfigs)                                                                     | Almacene las respuestas que dio al diálogo de configuración de un [plugin](/docs/es/plugins/overview)                                                                                                                                                                   | Plugins y skills                         | Usuario o administrado        |
| [`pluginSuggestionMarketplaces`](#pluginsuggestionmarketplaces)                                       | Elija qué [marketplaces](/docs/es/plugins/overview) pueden mostrar sugerencias de instalación de plugins en `/plugin`                                                                                                                                                   | Plugins y skills                         | Administrado                  |
| [`pluginTrustMessage`](#plugintrustmessage)                                                           | Agregue su propio texto a la [advertencia de confianza](/docs/es/plugins/overview) del plugin                                                                                                                                                                           | Plugins y skills                         | Administrado                  |
| [`policyHelper`](#policyhelper)                                                                       | Ejecute un ejecutable que calcule [configuración administrada](/docs/es/managed-settings#compute-the-policy-with-a-helper-program) al inicio                                                                                                                            | Configuración empresarial y administrada | Administrado                  |
| [`policyHelper.path`](#policyhelper-path)                                                             | Nombre el [ejecutable auxiliar](/docs/es/managed-settings#compute-the-policy-with-a-helper-program) que ejecuta Claude Code                                                                                                                                             | Configuración empresarial y administrada | Administrado                  |
| [`policyHelper.refreshIntervalMs`](#policyhelper-refreshintervalms)                                   | Vuelva a ejecutar el [auxiliar](/docs/es/managed-settings#compute-the-policy-with-a-helper-program) en segundo plano en un intervalo                                                                                                                                    | Configuración empresarial y administrada | Administrado                  |
| [`policyHelper.timeoutMs`](#policyhelper-timeoutms)                                                   | Establezca cuánto tiempo Claude Code espera al [auxiliar](/docs/es/managed-settings#compute-the-policy-with-a-helper-program)                                                                                                                                           | Configuración empresarial y administrada | Administrado                  |
| [`preferredNotifChannel`](#preferrednotifchannel)                                                     | Elija un [timbre de terminal o notificación de escritorio](/docs/es/terminal-config#get-a-terminal-bell-or-notification) para la finalización de tareas                                                                                                                 | Remoto, escritorio y notificaciones      | Cualquier archivo             |
| [`prefersReducedMotion`](#prefersreducedmotion)                                                       | [Reduzca o desactive](/docs/es/accessibility#accessibility-settings) animaciones de spinner, shimmer y flash                                                                                                                                                            | Interfaz y terminal                      | Cualquier archivo             |
| [`processWrapper`](#processwrapper)                                                                   | Ejecute los procesos de fondo de Claude Code a través de un [iniciador corporativo](/docs/es/corporate-launcher) en macOS y Linux                                                                                                                                       | Agentes, sesiones y worktrees            | Usuario o administrado        |
| [`promptCacheTtl`](#promptcachettl)                                                                   | Elija la [duración del caché de indicación](/docs/es/prompt-caching#cache-lifetime) para la conversación principal                                                                                                                                                      | Modelo y respuestas                      | Cualquier archivo             |
| [`promptSuggestionEnabled`](#promptsuggestionenabled)                                                 | Oculte las [sugerencias de indicador](/docs/es/interactive-mode#prompt-suggestions) atenuadas en el cuadro de entrada                                                                                                                                                   | Interfaz y terminal                      | Cualquier archivo             |
| [`prUrlTemplate`](#prurltemplate)                                                                     | Apunte los enlaces de PR a una herramienta de revisión de código interna en lugar de github.com                                                                                                                                                                    | Git y atribución                         | Cualquier archivo             |
| [`remote.defaultEnvironmentId`](#remote-defaultenvironmentid)                                         | Elija el [entorno en la nube](/docs/es/cloud-environments) predeterminado para `claude --cloud`; un ID `ccpool_` autohospedado es de solo lectura desde configuración de usuario y administrada y `--settings`                                                          | Remoto, escritorio y notificaciones      | Cualquier archivo             |
| [`remoteControlAtStartup`](#remotecontrolatstartup)                                                   | Conecte [Control Remoto](/docs/es/remote-control#enable-remote-control-for-all-sessions) automáticamente cuando comienza una sesión                                                                                                                                     | Remoto, escritorio y notificaciones      | Cualquier archivo             |
| [`requiredMaximumVersion`](#requiredmaximumversion)                                                   | [Rechace iniciar](/docs/es/setup#pin-a-minimum-version) en una versión más nueva de la que su organización permite                                                                                                                                                      | Actualizaciones y versiones              | Administrado                  |
| [`requiredMinimumVersion`](#requiredminimumversion)                                                   | [Rechace iniciar](/docs/es/setup#pin-a-minimum-version) en una versión más antigua de la que su organización requiere                                                                                                                                                   | Actualizaciones y versiones              | Administrado                  |
| [`respectGitignore`](#respectgitignore)                                                               | Mantenga los archivos ignorados por git fuera del [selector de archivo `@`](/docs/es/interactive-mode#quick-commands)                                                                                                                                                   | Interfaz y terminal                      | Cualquier archivo             |
| [`respondToBashCommands`](#respondtobashcommands)                                                     | Impida que Claude responda después de que se ejecute un [comando shell `!`](/docs/es/interactive-mode#shell-mode-with-prefix)                                                                                                                                           | Interfaz y terminal                      | Cualquier archivo             |
| [`sandbox`](#sandbox)                                                                                 | [Aisle comandos Bash](/docs/es/sandboxing) de su sistema de archivos y red en macOS, Linux y WSL2                                                                                                                                                                       | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.allowAppleEvents`](#sandbox-allowappleevents)                                               | Permita que [comandos en sandbox](/docs/es/sandboxing) envíen Apple Events en macOS                                                                                                                                                                                     | Configuración de sandbox                 | Usuario o administrado        |
| [`sandbox.allowUnsandboxedCommands`](#sandbox-allowunsandboxedcommands)                               | Permita que Claude reintente un comando bloqueado fuera del [sandbox](/docs/es/sandboxing#the-unsandboxed-retry-escape-hatch), o prohíbalo                                                                                                                              | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.autoAllowBashIfSandboxed`](#sandbox-autoallowbashifsandboxed)                               | Ejecute [comandos en sandbox](/docs/es/sandboxing#auto-allow-mode) sin un indicador de permisos                                                                                                                                                                         | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.bwrapPath`](#sandbox-bwrappath)                                                             | Apunte el [sandbox](/docs/es/sandboxing) a un binario bubblewrap fuera de `PATH`                                                                                                                                                                                        | Configuración de sandbox                 | Administrado                  |
| [`sandbox.credentials`](#sandbox-credentials)                                                         | Oculte o enmascare archivos y variables de credenciales dentro del [sandbox](/docs/es/sandboxing#protect-credentials)                                                                                                                                                   | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.credentials.allowPlaintextInject`](#sandbox-credentials-allowplaintextinject)               | Permita que [credenciales enmascaradas](/docs/es/sandboxing#mask-credentials) lleguen a servicios HTTP simples en redes de prueba confiables                                                                                                                            | Configuración de sandbox                 | Usuario o administrado        |
| [`sandbox.credentials.awsPairs`](#sandbox-credentials-awspairs)                                       | Vincule variables de clave AWS con nombre personalizado en una credencial para [re-firmar](/docs/es/sandboxing#re-sign-aws-requests)                                                                                                                                    | Configuración de sandbox                 | Usuario o administrado        |
| [`sandbox.credentials.envVars`](#sandbox-credentials-envvars)                                         | Desactive o enmascare una variable de entorno dentro del [sandbox](/docs/es/sandboxing#mask-environment-variables)                                                                                                                                                      | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.credentials.files`](#sandbox-credentials-files)                                             | Bloquee o enmascare lecturas de un archivo de credenciales dentro del [sandbox](/docs/es/sandboxing#mask-credential-files)                                                                                                                                              | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.credentials.sigv4`](#sandbox-credentials-sigv4)                                             | Elija si las solicitudes [SigV4A de AWS](/docs/es/sandboxing#re-sign-aws-requests) de streaming, presignadas o fallan o pasan                                                                                                                                           | Configuración de sandbox                 | Usuario o administrado        |
| [`sandbox.enabled`](#sandbox-enabled)                                                                 | Active el [sandbox de Bash](/docs/es/sandboxing#get-started) en macOS, Linux y WSL2                                                                                                                                                                                     | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.enableWeakerNestedSandbox`](#sandbox-enableweakernestedsandbox)                             | Ejecute el [sandbox](/docs/es/sandboxing) de Linux dentro de un contenedor sin privilegios                                                                                                                                                                              | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.enableWeakerNetworkIsolation`](#sandbox-enableweakernetworkisolation)                       | Permita que `gh`, `gcloud` y `terraform` verifiquen TLS detrás de un proxy MITM dentro del [sandbox](/docs/es/sandboxing#troubleshooting) en macOS                                                                                                                      | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.excludedCommands`](#sandbox-excludedcommands)                                               | Nombre comandos que siempre se ejecutan fuera del [sandbox](/docs/es/sandboxing)                                                                                                                                                                                        | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.failIfUnavailable`](#sandbox-failifunavailable)                                             | Rechace iniciar cuando el [sandbox](/docs/es/sandboxing) no pueda, en lugar de ejecutarse sin sandbox                                                                                                                                                                   | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.filesystem`](#sandbox-filesystem)                                                           | Controle qué rutas pueden leer y escribir [comandos en sandbox](/docs/es/sandboxing#filesystem-isolation)                                                                                                                                                               | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.filesystem.allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly)       | Impida que los desarrolladores vuelvan a abrir [rutas de lectura que su organización bloqueó](/docs/es/sandboxing#keep-developers-from-widening-the-policy)                                                                                                             | Configuración de sandbox                 | Administrado                  |
| [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread)                                       | Vuelva a abrir la lectura dentro de una región que [`denyRead`](#sandbox-filesystem-denyread) bloquea                                                                                                                                                              | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.filesystem.allowWrite`](#sandbox-filesystem-allowwrite)                                     | Agregue rutas a las que [comandos en sandbox](/docs/es/sandboxing) pueden escribir                                                                                                                                                                                      | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.filesystem.denyRead`](#sandbox-filesystem-denyread)                                         | Bloquee [comandos en sandbox](/docs/es/sandboxing) de leer rutas específicas                                                                                                                                                                                            | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.filesystem.denyWrite`](#sandbox-filesystem-denywrite)                                       | Bloquee [comandos en sandbox](/docs/es/sandboxing) de escribir en rutas específicas                                                                                                                                                                                     | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.filesystem.disabled`](#sandbox-filesystem-disabled)                                         | [Desactive el aislamiento del sistema de archivos](/docs/es/sandboxing#disable-filesystem-isolation) mientras mantiene el aislamiento de red                                                                                                                            | Configuración de sandbox                 | Usuario o administrado        |
| [`sandbox.ignoreViolations`](#sandbox-ignoreviolations)                                               | Silenciar reportes de violación para rutas que se espera que un comando sondee                                                                                                                                                                                     | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.network`](#sandbox-network)                                                                 | Controle qué hosts, puertos y sockets alcanzan [comandos en sandbox](/docs/es/sandboxing#network-isolation)                                                                                                                                                             | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.network.allowAllUnixSockets`](#sandbox-network-allowallunixsockets)                         | Permita que [comandos en sandbox](/docs/es/sandboxing) se conecten a cada socket Unix                                                                                                                                                                                   | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains)                                   | Pre-permita dominios para que [comandos en sandbox](/docs/es/sandboxing) no soliciten permiso para ellos                                                                                                                                                                | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.network.allowLocalBinding`](#sandbox-network-allowlocalbinding)                             | Permita que [comandos en sandbox](/docs/es/sandboxing) se vinculen a puertos localhost en macOS                                                                                                                                                                         | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.network.allowMachLookup`](#sandbox-network-allowmachlookup)                                 | Permita que herramientas [en sandbox](/docs/es/sandboxing) de macOS como el Simulador de iOS o Playwright alcancen sus servicios XPC                                                                                                                                    | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.network.allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly)                 | Bloquee la lista de permitidos de red a [configuración administrada](/docs/es/sandboxing#keep-developers-from-widening-the-policy)                                                                                                                                      | Configuración de sandbox                 | Administrado                  |
| [`sandbox.network.allowUnixSockets`](#sandbox-network-allowunixsockets)                               | Enumere rutas de socket Unix que [comandos en sandbox](/docs/es/sandboxing) pueden usar en macOS                                                                                                                                                                        | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.network.deniedDomains`](#sandbox-network-denieddomains)                                     | Bloquee dominios para [comandos en sandbox](/docs/es/sandboxing), incluso dentro de un comodín permitido                                                                                                                                                                | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.network.httpProxyPort`](#sandbox-network-httpproxyport)                                     | Enrute el tráfico HTTP del [sandbox](/docs/es/sandboxing#custom-proxy-configuration) a través de su propio proxy                                                                                                                                                        | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.network.socksProxyPort`](#sandbox-network-socksproxyport)                                   | Enrute el tráfico SOCKS del [sandbox](/docs/es/sandboxing#custom-proxy-configuration) a través de su propio proxy                                                                                                                                                       | Configuración de sandbox                 | Cualquier archivo             |
| [`sandbox.network.strictAllowlist`](#sandbox-network-strictallowlist)                                 | Deniegue hosts fuera de la [lista de permitidos](/docs/es/sandboxing#network-isolation) en lugar de solicitar permiso                                                                                                                                                   | Configuración de sandbox                 | Usuario o administrado        |
| [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate)                                       | Haga que el [sandbox](/docs/es/sandboxing#network-isolation) proxy termine TLS para que pueda leer solicitudes HTTPS                                                                                                                                                    | Configuración de sandbox                 | Usuario o administrado        |
| [`sandbox.ripgrep`](#sandbox-ripgrep)                                                                 | Use su propio binario ripgrep dentro del [sandbox](/docs/es/sandboxing)                                                                                                                                                                                                 | Configuración de sandbox                 | Usuario o administrado        |
| [`sandbox.socatPath`](#sandbox-socatpath)                                                             | Apunte el proxy del [sandbox](/docs/es/sandboxing) a un binario `socat` fuera de `PATH`                                                                                                                                                                                 | Configuración de sandbox                 | Administrado                  |
| [`showClearContextOnPlanAccept`](#showclearcontextonplanaccept)                                       | Muestre una opción "borrar contexto" en la [pantalla de aceptación del plan](/docs/es/permission-modes#review-and-approve-a-plan)                                                                                                                                       | Interfaz y terminal                      | Cualquier archivo             |
| [`showThinkingSummaries`](#showthinkingsummaries)                                                     | Vea resúmenes del [pensamiento](/docs/es/model-config#extended-thinking) de Claude en lugar de un stub colapsado                                                                                                                                                        | Modelo y respuestas                      | Cualquier archivo             |
| [`showTurnDuration`](#showturnduration)                                                               | Oculte la duración "Cooked for" después de cada respuesta                                                                                                                                                                                                          | Interfaz y terminal                      | Cualquier archivo             |
| [`skillListingBudgetFraction`](#skilllistingbudgetfraction)                                           | Reserve más o menos contexto para la [lista de skills](/docs/es/skills#skill-descriptions-are-cut-short)                                                                                                                                                                | Memoria y contexto                       | Cualquier archivo             |
| [`skillListingMaxDescChars`](#skilllistingmaxdescchars)                                               | Limite la longitud de la descripción de cada skill en la [lista de skills](/docs/es/skills#skill-descriptions-are-cut-short)                                                                                                                                            | Memoria y contexto                       | Cualquier archivo             |
| [`skillOverrides`](#skilloverrides)                                                                   | [Oculte o contraiga un skill](/docs/es/skills#override-skill-visibility-from-settings) sin editar su SKILL.md                                                                                                                                                           | Plugins y skills                         | Cualquier archivo             |
| [`skipAutoPermissionPrompt`](#skipautopermissionprompt)                                               | Omita el aviso único que Claude Code muestra cuando entra por primera vez en el [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) usted mismo en lugar de a través del predeterminado integrado                                             | Configuración de permisos                | Usuario o administrado        |
| [`skipDangerousModePermissionPrompt`](#skipdangerousmodepermissionprompt)                             | Omita el diálogo de confirmación antes del [modo bypassPermissions](/docs/es/permission-modes#skip-all-checks-with-bypasspermissions-mode)                                                                                                                              | Configuración de permisos                | Usuario, local o administrado |
| [`skipWebFetchPreflight`](#skipwebfetchpreflight)                                                     | Omita la [verificación de nombre de host de WebFetch](/docs/es/tools-reference#webfetch-tool-behavior) cuando Anthropic es inaccesible                                                                                                                                  | Privacidad y telemetría                  | Cualquier archivo             |
| [`spellcheck`](#spellcheck)                                                                           | Subraye palabras mal escritas en la entrada del indicador con un [corrector ortográfico](/docs/es/interactive-mode#check-spelling-as-you-type) que instale                                                                                                              | Interfaz y terminal                      | Usuario o administrado        |
| [`spinnerTipsEnabled`](#spinnertipsenabled)                                                           | Oculte consejos en el spinner mientras Claude trabaja                                                                                                                                                                                                              | Interfaz y terminal                      | Cualquier archivo             |
| [`spinnerTipsOverride`](#spinnertipsoverride)                                                         | Agregue sus propios consejos a la rotación del spinner, o reemplace los consejos integrados                                                                                                                                                                        | Interfaz y terminal                      | Cualquier archivo             |
| [`spinnerVerbs`](#spinnerverbs)                                                                       | Agregue o reemplace los verbos mostrados mientras se ejecuta un turno                                                                                                                                                                                              | Interfaz y terminal                      | Cualquier archivo             |
| [`sshConfigs`](#sshconfigs)                                                                           | Agregue [conexiones SSH](/docs/es/desktop#pre-configure-ssh-connections-for-your-team) al menú desplegable del entorno de Desktop                                                                                                                                       | Remoto, escritorio y notificaciones      | Usuario o administrado        |
| [`sshHostAllowlist`](#sshhostallowlist)                                                               | Limite qué hosts pueden alcanzar las [sesiones SSH de Desktop](/docs/es/desktop#restrict-which-ssh-hosts-users-can-connect-to)                                                                                                                                          | Remoto, escritorio y notificaciones      | Administrado                  |
| [`statusLine`](#statusline)                                                                           | Ejecute su propio comando para representar una [línea de estado](/docs/es/statusline) debajo del indicador                                                                                                                                                              | Interfaz y terminal                      | Cualquier archivo             |
| [`strictKnownMarketplaces`](#strictknownmarketplaces)                                                 | Enumere los [marketplace](/docs/es/plugins/overview) que los usuarios pueden agregar e instalar                                                                                                                                                                         | Plugins y skills                         | Administrado                  |
| [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)                                     | Bloquee [skills](/docs/es/skills), [agentes](/docs/es/sub-agents), [hooks](/docs/es/hooks), y [servidores MCP](/docs/es/mcp) de fuentes de usuario y proyecto                                                                                                                          | Plugins y skills                         | Administrado                  |
| [`strictPluginOnlyCustomization.agents`](#strictpluginonlycustomization-agents)                       | Bloquee [agentes](/docs/es/sub-agents) a fuentes de plugin y administradas                                                                                                                                                                                              | Plugins y skills                         | Administrado                  |
| [`strictPluginOnlyCustomization.hooks`](#strictpluginonlycustomization-hooks)                         | Bloquee [hooks](/docs/es/hooks) a fuentes de plugin y administradas                                                                                                                                                                                                     | Plugins y skills                         | Administrado                  |
| [`strictPluginOnlyCustomization.mcp`](#strictpluginonlycustomization-mcp)                             | Bloquee [servidores MCP](/docs/es/mcp) a fuentes de plugin y administradas                                                                                                                                                                                              | Plugins y skills                         | Administrado                  |
| [`strictPluginOnlyCustomization.skills`](#strictpluginonlycustomization-skills)                       | Bloquee [skills](/docs/es/skills) a fuentes de plugin y administradas                                                                                                                                                                                                   | Plugins y skills                         | Administrado                  |
| [`subagentPromptCacheTtl`](#subagentpromptcachettl)                                                   | Elija la [duración del caché de indicación](/docs/es/prompt-caching#cache-lifetime) para subagentes y otras solicitudes fuera de la conversación principal                                                                                                              | Modelo y respuestas                      | Cualquier archivo             |
| [`subagentStatusLine`](#subagentstatusline)                                                           | Reescriba filas en la [pantalla de tareas del subagente](/docs/es/sub-agents) con su propio comando                                                                                                                                                                     | Interfaz y terminal                      | Cualquier archivo             |
| [`switchModelsOnFlag`](#switchmodelsonflag)                                                           | Cambie modelos automáticamente o pause cuando un [clasificador de seguridad](/docs/es/model-config#ask-before-switching) marca una solicitud                                                                                                                            | Modelo y respuestas                      | Cualquier archivo             |
| [`syncClaudeAiPlugins`](#syncclaudeaiplugins)                                                         | Deje de cargar los [plugins habilitados en su cuenta de claude.ai](/docs/es/plugins/overview) y deje de descargar nuevos                                                                                                                                                | Plugins y skills                         | Usuario, local o administrado |
| [`syncClaudeAiSkills`](#syncclaudeaiskills)                                                           | Deje de cargar los [skills habilitados en su cuenta de claude.ai](/docs/es/skills#how-synced-skills-behave) y deje de descargar nuevos                                                                                                                                  | Plugins y skills                         | Usuario, local o administrado |
| [`syntaxHighlightingDisabled`](#syntaxhighlightingdisabled)                                           | Desactive el resaltado de sintaxis en diffs y bloques de código                                                                                                                                                                                                    | Interfaz y terminal                      | Cualquier archivo             |
| [`taskOutputMaxChars`](#taskoutputmaxchars)                                                           | Removido en v2.1.277, junto con la herramienta `TaskOutput` que dimensionaba                                                                                                                                                                                       | Memoria y contexto                       | Cualquier archivo             |
| [`teammateDefaultModel`](#teammatedefaultmodel)                                                       | Eliminado en v2.1.234; vea [Especificar compañeros de equipo y modelos](/docs/es/agent-teams#specify-teammates-and-models) para cómo Claude Code elige el modelo de un compañero de equipo                                                                              | Configuración global                     | Configuración global          |
| [`teammateMode`](#teammatemode)                                                                       | Elija cómo [se muestran los compañeros de equipo del equipo de agentes](/docs/es/agent-teams#choose-a-display-mode)                                                                                                                                                     | Agentes, sesiones y worktrees            | Cualquier archivo             |
| [`terminalProgressBarEnabled`](#terminalprogressbarenabled)                                           | Oculte la barra de progreso del terminal en terminales que la admitan                                                                                                                                                                                              | Interfaz y terminal                      | Cualquier archivo             |
| [`terminalTitleFromRename`](#terminaltitlefromrename)                                                 | Impida que [`/rename`](/docs/es/sessions#name-your-sessions) y `--name` cambien el título de la pestaña del terminal                                                                                                                                                    | Interfaz y terminal                      | Cualquier archivo             |
| [`theme`](#theme)                                                                                     | Elija el [tema de color](/docs/es/terminal-config#match-the-color-theme) de la interfaz, integrado o personalizado                                                                                                                                                      | Interfaz y terminal                      | Cualquier archivo             |
| [`timeFormat`](#timeformat)                                                                           | Muestre las horas en la interfaz en un reloj de 12 horas o 24 horas, en UTC, o con un patrón strftime                                                                                                                                                              | Interfaz y terminal                      | Cualquier archivo             |
| [`timeZone`](#timezone)                                                                               | Muestre las horas en la interfaz en una zona horaria distinta a la de su sistema                                                                                                                                                                                   | Interfaz y terminal                      | Cualquier archivo             |
| [`tui`](#tui)                                                                                         | Elija el representador [pantalla completa](/docs/es/fullscreen) o terminal clásico                                                                                                                                                                                      | Interfaz y terminal                      | Cualquier archivo             |
| [`ultracode`](#ultracode)                                                                             | Haga que Claude planifique un [flujo de trabajo](/docs/es/workflows#let-claude-decide-with-ultracode) para cada tarea sustancial sin ser preguntado                                                                                                                     | Modelo y respuestas                      | Cualquier archivo             |
| [`useAutoModeDuringPlan`](#useautomodeduringplan)                                                     | Permita que el clasificador de [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) revise comandos shell en [Plan Mode](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode); establezca `false` para obtener indicadores en su lugar | Configuración de permisos                | Usuario, local o administrado |
| [`verbose`](#verbose)                                                                                 | Muestre [salida de herramienta completa](/docs/es/cli-reference#cli-flags) en lugar de resúmenes truncados; `viewMode` tiene prioridad cuando ambos se establecen                                                                                                       | Interfaz y terminal                      | Cualquier archivo             |
| [`viewMode`](#viewmode)                                                                               | Inicie cada sesión en [vista predeterminada, detallada o enfocada](/docs/es/cli-reference#cli-flags)                                                                                                                                                                    | Interfaz y terminal                      | Cualquier archivo             |
| [`vimInsertModeRemaps`](#viminsertmoderemaps)                                                         | Asigne una [secuencia de modo INSERT](/docs/es/interactive-mode#remap-insert-mode-key-sequences) de dos teclas como `jj` a Escape                                                                                                                                       | Interfaz y terminal                      | Usuario o administrado        |
| [`voice`](#voice)                                                                                     | Active la [dictación por voz](/docs/es/voice-dictation) y elija modo de mantener o tocar                                                                                                                                                                                | Interfaz y terminal                      | Cualquier archivo             |
| [`voiceEnabled`](#voiceenabled)                                                                       | Active la [dictación por voz](/docs/es/voice-dictation) con la forma de una sola tecla más antigua                                                                                                                                                                      | Interfaz y terminal                      | Cualquier archivo             |
| [`wheelScrollAccelerationEnabled`](#wheelscrollaccelerationenabled)                                   | Desactive la [aceleración de rueda del ratón](/docs/es/fullscreen#mouse-wheel-scrolling) en la representación de pantalla completa                                                                                                                                      | Interfaz y terminal                      | Cualquier archivo             |
| [`workflowKeywordTriggerEnabled`](#workflowkeywordtriggerenabled)                                     | Permita que la palabra `ultracode` en un indicador inicie un [flujo de trabajo](/docs/es/workflows); establezca `false` para escribirla sin iniciar uno                                                                                                                 | Hooks y automatización                   | Cualquier archivo             |
| [`workflowSizeGuideline`](#workflowsizeguideline)                                                     | Establezca el número de agentes que Claude apunta en [flujos de trabajo dinámicos](/docs/es/workflows)                                                                                                                                                                  | Hooks y automatización                   | Cualquier archivo             |
| [`worktree`](#worktree)                                                                               | Configure cómo Claude Code crea git [worktrees](/docs/es/worktrees)                                                                                                                                                                                                     | Agentes, sesiones y worktrees            | Cualquier archivo             |
| [`worktree.baseRef`](#worktree-baseref)                                                               | Rama nuevos [worktrees](/docs/es/worktrees) desde la rama predeterminada remota o su HEAD local                                                                                                                                                                         | Agentes, sesiones y worktrees            | Cualquier archivo             |
| [`worktree.bgIsolation`](#worktree-bgisolation)                                                       | Permita que las sesiones de fondo editen la copia de trabajo sin un [worktree](/docs/es/worktrees)                                                                                                                                                                      | Agentes, sesiones y worktrees            | Cualquier archivo             |
| [`worktree.sparsePaths`](#worktree-sparsepaths)                                                       | Extraiga solo los directorios que necesita en cada [worktree](/docs/es/worktrees)                                                                                                                                                                                       | Agentes, sesiones y worktrees            | Cualquier archivo             |
| [`worktree.symlinkDirectories`](#worktree-symlinkdirectories)                                         | Enlace simbólicamente directorios grandes en cada [worktree](/docs/es/worktrees) en lugar de duplicarlos                                                                                                                                                                | Agentes, sesiones y worktrees            | Cualquier archivo             |
| [`wslInheritsWindowsSettings`](#wslinheritswindowssettings)                                           | Haga que WSL lea [configuración administrada](/docs/es/managed-settings) de la cadena de política de Windows                                                                                                                                                            | Configuración empresarial y administrada | Administrado                  |

<h2 id="model-and-responses">
  Modelo y respuestas
</h2>

Elija qué modelos utiliza Claude Code y cómo responde. Para saber cómo estas configuraciones interactúan con el comando `/model` y las variables de entorno, consulte [Configuración de modelos](/docs/es/model-config).

<h3 id="advisormodel">
  `advisorModel`
</h3>

Elija qué modelo responde cuando Claude llama a la [herramienta advisor](/docs/es/advisor) del lado del servidor. Déjelo sin establecer para desactivar el advisor. El advisor debe ser al menos tan capaz como su modelo principal. Consulte [Elegir un modelo advisor](/docs/es/advisor#choose-an-advisor-model) para los emparejamientos aceptados y qué sucede cuando elige uno que no es aceptado.

Normalmente no edita esta clave manualmente. Ejecute `/advisor` para abrir un selector que muestre la opción actual, los modelos que pueden asesorar y **Sin advisor**. Claude Code guarda su selección en esta clave en `~/.claude/settings.json`. Si elige desde un cliente de [Control Remoto](/docs/es/remote-control) o en una sesión conectada a un trabajador remoto, la selección se aplica solo a esa sesión y no cambia esta clave.

Si su cuenta requiere el [consentimiento de créditos de uso](/docs/es/advisor#fable-advisor-and-usage-credits), acéptelo primero ejecutando `/model fable`. Hasta que lo haga, elegir Fable en `/advisor` no guarda nada y Claude Code le indica que ejecute `/model fable` primero.

* **Alcance**: [`Cualquier archivo`](#scopes)
* **Tipo**: cadena, uno de los alias `"fable"`, `"opus"` u `"sonnet"`, que se resuelven a la versión predeterminada actual de Claude Code de esa familia de modelos, o un ID de modelo completo como `"claude-opus-5-5"`
* **Predeterminado**: sin establecer, por lo que el advisor está desactivado
* **Anulaciones por sesión**: `--advisor` tiene precedencia sobre esta clave para una sesión. [`CLAUDE_CODE_DISABLE_ADVISOR_TOOL`](/docs/es/env-vars) desactiva el advisor, y esta clave no puede reactivarlo

```json settings.json theme={null}
{
  "advisorModel": "opus"
}
```

La clave no tiene efecto en proveedores donde el advisor [no está disponible](/docs/es/advisor#requirements), como Amazon Bedrock y Claude Platform en AWS. `"fable"` requiere [acceso a Fable](/docs/es/advisor#choose-an-advisor-model).

<h3 id="alwaysthinkingenabled">
  `alwaysThinkingEnabled`
</h3>

Desactive el [pensamiento extendido](/docs/es/model-config#extended-thinking) para cada sesión estableciendo esto en `false`. El pensamiento está activado de forma predeterminada, por lo que `true` no cambia nada. La mayoría de las personas establecen esto a través de `/config` en lugar de editar el archivo.

En modelos que siempre piensan, como Opus 5.5 y los modelos Fable, `false` no tiene efecto. En [proveedores de terceros](/docs/es/third-party-integrations) Claude Code omite el parámetro `thinking` en lugar de desactivar el pensamiento, por lo que los modelos de razonamiento adaptativo pueden seguir pensando. Con el pensamiento desactivado en la API de Anthropic, Claude Code envía esfuerzo `high` en lugar de un nivel superior a modelos que sabe que [no aceptan esa combinación](/docs/es/errors#effort-isnt-available-with-thinking-turned-off), como Opus 5.

* **Alcance**: [`Cualquier archivo`](#scopes)
* **Tipo**: Booleano
  * `true`: sin efecto; el pensamiento ya está activado
  * `false`: Claude Code desactiva el pensamiento extendido para cada sesión
* **Predeterminado**: sin establecer, por lo que el pensamiento está activado para modelos que lo admiten
* **Anulaciones por sesión**: [`MAX_THINKING_TOKENS`](/docs/es/env-vars) tiene precedencia sobre esta clave para una sesión: `0` desactiva el pensamiento, bajo los mismos límites de modelo y proveedor que `false`, y un valor positivo activa el pensamiento incluso cuando esta clave es `false`. En modelos de razonamiento adaptativo, el número en sí se ignora

```json settings.json theme={null}
{
  "alwaysThinkingEnabled": false
}
```

<h3 id="availablemodels">
  `availableModels`
</h3>

Restrinja qué modelos pueden seleccionar las personas para la sesión principal, [subagentes](/docs/es/sub-agents), [skills](/docs/es/skills) y el [advisor](/docs/es/advisor). Una lista administrada limita `/model`, `--model` y la clave `model` en los archivos propios del desarrollador; un modelo fuera de ella no puede seleccionarse. Por sí solo, esto no afecta la opción Predeterminado; emparéjelo con [`enforceAvailableModels`](#enforceavailablemodels) para eso.

* **Alcance**: [`Cualquier archivo`](#scopes). Impleméntelo en configuraciones administradas para aplicarlo a una organización.
* **Tipo**: matriz de alias o IDs de modelos
* **Predeterminado**: sin establecer, por lo que cada modelo está disponible

Este ejemplo permite que las personas seleccionen solo modelos Sonnet y Haiku:

```json settings.json theme={null}
{
  "availableModels": ["sonnet", "haiku"]
}
```

Consulte [Restringir la selección de modelos](/docs/es/model-config#restrict-model-selection).

<h3 id="effortlevel">
  `effortLevel`
</h3>

Establezca un [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) predeterminado para modelos para los que no ha guardado un nivel. Los niveles más bajos son más rápidos y económicos en tareas sencillas, y los niveles más altos razonan más profundamente en problemas complejos.

Cuando ejecuta `/effort low`, `medium`, `high` o `xhigh` en una sesión interactiva en su máquina, Claude Code guarda el nivel para el modelo activo bajo [`modelSettings`](#modelsettings) en lugar de escribir esta clave. Antes de v2.1.251, `/effort` escribía esta clave.

Dentro del mismo archivo de configuración, Claude Code utiliza el nivel guardado de un modelo en lugar de esta clave. [`modelSettings`](#modelsettings) indica la precedencia entre archivos.

En una sesión conectada a un trabajador remoto, en una ejecución `-p` y en el Agent SDK, `/effort` se aplica solo a esa sesión. [Ajustar el nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) enumera las selecciones interactivas que también se aplican solo a esa sesión. El mensaje que imprime `/effort` indica qué sucedió.

* **Alcance**: [`Cualquier archivo`](#scopes)
* **Tipo**: cadena, uno de:
  * `"low"`: el menor razonamiento, para tareas cortas, limitadas y sensibles a la latencia que no son sensibles a la inteligencia
  * `"medium"`: reduce el uso de tokens para trabajo sensible a costos que puede comprometer algo de inteligencia
  * `"high"`: equilibra el uso de tokens e inteligencia
  * `"xhigh"`: razonamiento más profundo con mayor gasto de tokens
* **Predeterminado**: sin establecer
* **Anulaciones por sesión**: `--effort` tiene precedencia sobre esta clave para una sesión, y [`CLAUDE_CODE_EFFORT_LEVEL`](/docs/es/env-vars) tiene precedencia sobre ambas

```json settings.json theme={null}
{
  "effortLevel": "xhigh"
}
```

En su archivo de configuración de usuario, `~/.claude/settings.json`, esta clave es la forma anterior que `/effort` escribía antes de guardar niveles por modelo, y sigue aplicándose donde se aplicaba antes, en Opus 5, Fable 5.1 y modelos anteriores. Opus 5.5 y modelos lanzados después de él la ignoran y comienzan en su propio predeterminado hasta que guarde un nivel para ellos, que `/effort` escribe bajo [`modelSettings`](#modelsettings). En configuraciones de proyecto, local y administrada, y con `--settings`, esta clave se aplica a cada modelo.

<h3 id="enforceavailablemodels">
  `enforceAvailableModels`
</h3>

El selector `/model` tiene una opción **Predeterminado** que se resuelve a su [modelo predeterminado de la organización](/docs/es/model-config#organization-default-model) cuando se aplica una, y de lo contrario al predeterminado de su tipo de cuenta. Una lista de permitidos [`availableModels`](#availablemodels) limita los modelos que puede nombrar, pero por sí sola deja **Predeterminado** solo, por lo que **Predeterminado** aún puede resolverse a un modelo fuera de la lista. Esta clave cierra esa brecha. Requiere Claude Code v2.1.175 o posterior.

Cuando su organización implementa cualquier configuración administrada, Claude Code lee esta clave solo de la fuente administrada e la ignora en sus otros archivos.

* **Alcance**: [`Cualquier archivo`](#scopes)
* **Tipo**: Booleano
  * `true`: cuando **Predeterminado** se resolvería a un modelo fuera de `availableModels`, Claude Code lo resuelve al primer modelo disponible en la lista
  * `false`: **Predeterminado** se resuelve como de costumbre, incluso a un modelo fuera de `availableModels`
* **Predeterminado**: `false`

Este ejemplo restringe las selecciones nombradas a modelos Sonnet y Haiku y hace que **Predeterminado** se resuelva al primero de ellos que esté disponible:

```json settings.json theme={null}
{
  "availableModels": ["sonnet", "haiku"],
  "enforceAvailableModels": true
}
```

Esta clave no tiene efecto cuando `availableModels` no está establecido o está vacío. Consulte [Aplicar la lista de permitidos al modelo Predeterminado](/docs/es/model-config#enforce-the-allowlist-for-the-default-model). Requiere Claude Code v2.1.175 o posterior.

<h3 id="fallbackmodel">
  `fallbackModel`
</h3>

Nombre modelos de respaldo para que Claude Code intente, en orden, cuando su modelo principal está sobrecargado o no disponible. Claude Code cambia al siguiente modelo disponible en la cadena para el resto del turno y muestra un aviso. Sin una cadena, Claude Code reintenta el mismo modelo y luego muestra el error del servidor, y usted reintenta o cambia de modelos.

Un cambio significa un turno con un [caché de solicitud](/docs/es/prompt-caching#switching-models) frío en el modelo de respaldo; su siguiente mensaje intenta el modelo principal primero nuevamente.

* **Alcance**: [`Cualquier archivo`](#scopes)
* **Tipo**: matriz de alias o IDs de modelos; `"default"` se expande al modelo predeterminado
* **Predeterminado**: sin establecer, por lo que una solicitud fallida no se reintenta en otro modelo
* **Anulaciones por sesión**: `--fallback-model` tiene precedencia sobre esta clave para una sesión

Este ejemplo intenta Sonnet 5 primero, luego Haiku 4.5, cuando su modelo principal falla:

```json settings.json theme={null}
{
  "fallbackModel": ["claude-sonnet-5", "claude-haiku-4-5"]
}
```

A diferencia de la mayoría de las configuraciones de matriz, esta clave no se fusiona entre archivos de configuración: el archivo de mayor precedencia que la define proporciona toda la cadena. Si su archivo de proyecto establece `["claude-sonnet-5"]` y su archivo de usuario establece `["claude-haiku-4-5"]`, la cadena es solo `["claude-sonnet-5"]`. Claude Code mantiene como máximo tres modelos permitidos distintos de la lista e ignora el resto. Consulte [Cadenas de modelos de respaldo](/docs/es/model-config#fallback-model-chains).

<h3 id="fastmode">
  `fastMode`
</h3>

Active el [modo rápido](/docs/es/fast-mode) para sesiones donde está disponible, para trabajo interactivo como iteración rápida o depuración en vivo donde desea velocidad a un costo más alto por token. Normalmente no edita esta clave manualmente: ejecutar `/fast` escribe `fastMode: true` en `~/.claude/settings.json`, y ejecutarlo nuevamente para desactivar el modo rápido elimina la clave. El modo rápido se ejecuta solo en Opus 5.5, Opus 5 y Opus 4.8: activarlo desde otro modelo lo cambia a Opus, y cambiar a un modelo no compatible lo desactiva. Consulte [Cambiar modelos mientras el modo rápido está activado](/docs/es/fast-mode#switch-models-while-fast-mode-is-on).

* **Alcance**: [`Cualquier archivo`](#scopes)
* **Tipo**: Booleano
  * `true`: Claude Code activa el modo rápido para sesiones donde está disponible
  * `false`: el modo rápido permanece desactivado
* **Predeterminado**: sin establecer, por lo que el modo rápido está desactivado
* **Anulaciones por sesión**: [`CLAUDE_CODE_DISABLE_FAST_MODE`](/docs/es/env-vars) desactiva el modo rápido para una sesión, y esta clave no puede reactivarlo

```json settings.json theme={null}
{
  "fastMode": true
}
```

<h3 id="fastmodepersessionoptin">
  `fastModePerSessionOptIn`
</h3>

Normalmente, ejecutar `/fast` guarda [`fastMode`](#fastmode) en la configuración de usuario de una persona, por lo que el modo rápido está activado al inicio de cada sesión posterior. Establezca esta clave en `true` para detener eso: un `fastMode: true` guardado ya no activa el modo rápido al inicio de la sesión, y cada persona debe ejecutar `/fast` en cada sesión que lo desee. Claude Code deja la clave `fastMode` en su archivo, por lo que desactivar esta clave restaura el comportamiento anterior.

Los propietarios en planes Team o Enterprise pueden implementarlo en toda la organización a través de [configuraciones administradas por el servidor](/docs/es/server-managed-settings). Cuando las configuraciones administradas establecen la clave, `/fast on` se rechaza fuera de sesiones de terminal interactivas e informa que su organización ha desactivado el modo rápido. Eso cubre [modo no interactivo](/docs/es/headless), la [extensión VS Code](/docs/es/vs-code) y [sesiones en la nube](/docs/es/claude-code-on-the-web).

* **Alcance**: [`Cualquier archivo`](#scopes)
* **Tipo**: Booleano
  * `true`: un `fastMode: true` guardado ya no activa el modo rápido al inicio de la sesión, por lo que cada persona ejecuta `/fast` en cada sesión que lo desee; un `fastMode: true` pasado con `--settings` aún cuenta para esa sesión a menos que las configuraciones administradas establezcan esta clave
  * `false`: un `fastMode: true` guardado activa el modo rápido al inicio de cada sesión posterior
* **Predeterminado**: `false`

```json settings.json theme={null}
{
  "fastModePerSessionOptIn": true
}
```

Consulte [Requerir opción de participación por sesión](/docs/es/fast-mode#require-per-session-opt-in).

<h3 id="language">
  `language`
</h3>

Haga que Claude responda en un idioma distinto del inglés de forma predeterminada. No hay una lista fija para respuestas: Claude Code agrega el valor textualmente al mensaje del sistema como una instrucción para responder siempre en ese idioma, por lo que cualquier nombre de idioma que Claude pueda leer funciona. Claude Code no valida el valor, por lo que un nombre mal escrito llega a Claude tal como está escrito en lugar de producir un error. El mismo valor establece el idioma para [dictado de voz](/docs/es/voice-dictation#change-the-dictation-language), que tiene una lista fija de [idiomas de dictado admitidos](/docs/es/voice-dictation#change-the-dictation-language), y para títulos de sesión generados automáticamente.

* **Alcance**: [`Cualquier archivo`](#scopes)
* **Tipo**: cadena, cualquier nombre de idioma, como `"japanese"`, `"spanish"` o `"french"`; Claude Code no lo valida
* **Predeterminado**: sin establecer; los títulos de sesión coinciden con el idioma de su conversación

```json settings.json theme={null}
{
  "language": "japanese"
}
```

<h3 id="maxeffortlevel">
  `maxEffortLevel`
</h3>

Limite el [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) que una sesión puede usar, dejando disponibles niveles más bajos. Cualquier nivel más alto se ejecuta en el límite en su lugar, incluido uno de `/effort`, el selector `/model`, `--effort`, [`CLAUDE_CODE_EFFORT_LEVEL`](/docs/es/env-vars), el frontmatter `effort` de una skill o subagente, o el predeterminado del modelo. Claude Code aplica el límite a sí mismo antes de cada solicitud, por lo que se mantiene en cada proveedor, incluidos Amazon Bedrock, la Plataforma de Agentes de Google Cloud y Microsoft Foundry. Requiere Claude Code v2.1.267 o posterior.

* **Alcance**: [`Cualquier archivo`](#scopes). Impleméntelo en configuraciones administradas para aplicarlo a una organización. Cuando varios alcances establecen un límite, se aplica el más bajo, por lo que un límite establecido en un alcance no puede elevarse desde otro
* **Tipo**: cadena, uno de `"low"`, `"medium"`, `"high"`, `"xhigh"` o `"max"`. Un valor `"max"` no establece límite
* **Predeterminado**: sin establecer, por lo que no se aplica límite
* **Efecto en ultracode**: un límite por debajo de `xhigh` hace que [ultracode](#ultracode) no esté disponible en los modelos a los que se aplica el límite
* **Límites por modelo**: agregue `maxEffortLevel` a la entrada [`modelSettings`](#modelsettings) de un modelo. Esa entrada reemplaza esta clave solo para el modelo dentro de la fuente de configuración que establece ambas, como su configuración de usuario o una [fuente administrada](/docs/es/managed-settings#how-claude-code-combines-managed-sources). Establezca `"max"` allí para eximir el modelo del límite de esa fuente; Claude Code aún aplica límites de otras fuentes

Este ejemplo limita cada modelo a `medium` y exime a Sonnet 4.6:

```json settings.json theme={null}
{
  "maxEffortLevel": "medium",
  "modelSettings": {
    "claude-sonnet-4-6": {
      "maxEffortLevel": "max"
    }
  }
}
```

Cuando su organización también establece un [límite de esfuerzo](/docs/es/model-config#organization-effort-limits) para un modelo, se aplica el límite más bajo de los dos.

<h3 id="model">
  `model`
</h3>

Establezca el modelo que cada nueva sesión utiliza, para que no tenga que elegir uno con `/model` cada vez. Establecerlo aquí no le impide cambiar de modelo a mitad de sesión. Si su administrador estableció un [modelo predeterminado de la organización](/docs/es/model-config#organization-default-model) para anular la selección del usuario, obtiene ese modelo incluso cuando establece esta clave en configuraciones de usuario, proyecto o local.

* **Alcance**: [`Cualquier archivo`](#scopes)
* **Tipo**: cadena, un alias de modelo o ID de modelo completo
* **Predeterminado**: sin establecer, por lo que Claude Code utiliza el modelo predeterminado de su cuenta
* **Anulaciones por sesión**: `--model` tiene precedencia sobre [`ANTHROPIC_MODEL`](/docs/es/env-vars), y ambas tienen precedencia sobre esta clave para una sesión, incluida una `model` administrada; una lista [`availableModels`](#availablemodels) aún se aplica a la selección

```json settings.json theme={null}
{
  "model": "claude-sonnet-5"
}
```

Un valor aquí supera [`ANTHROPIC_DEFAULT_MODEL`](/docs/es/model-config#set-a-default-model-for-new-sessions), que Claude Code utiliza solo cuando nada más selecciona un modelo.

<h3 id="modeloverrides">
  `modelOverrides`
</h3>

Asigne IDs de modelos de Anthropic a IDs de modelos específicos del proveedor, como ARNs de perfil de inferencia de Amazon Bedrock. Cada entrada del selector de modelos utiliza su valor asignado al llamar a la API del proveedor. Los administradores utilizan esto en [Amazon Bedrock, la Plataforma de Agentes de Google Cloud y Microsoft Foundry](/docs/es/model-config#override-model-ids-per-version) para enrutar cada versión de modelo a un perfil de inferencia específico, nombre de versión o implementación para gobernanza, asignación de costos o enrutamiento regional.

* **Alcance**: [`Cualquier archivo`](#scopes)
* **Tipo**: objeto que asigna ID de modelo a ID de modelo del proveedor
* **Predeterminado**: sin establecer

Este ejemplo enruta cada llamada para Opus 4.6 al perfil de inferencia de Bedrock nombrado:

```json settings.json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-6": "arn:aws:bedrock:us-east-1:123456789012:inference-profile/example"
  }
}
```

Consulte [Anular IDs de modelos por versión](/docs/es/model-config#override-model-ids-per-version).

<h3 id="modelpicker">
  `modelPicker`
</h3>

Enumere los modelos que el selector `/model` ofrece, en el orden que escriba y bajo etiquetas que elija, para que el selector enumere los modelos que su organización ejecuta, después de la alineación integrada o en su lugar. El `model` de cada fila se toma textualmente, por lo que acepta cualquier cosa que `--model` acepte: un alias como `opus`, un ID de modelo de Anthropic, o un ID de formato de proveedor para Amazon Bedrock, la Plataforma de Agentes de Google Cloud, Microsoft Foundry, o una puerta de enlace LLM. Requiere Claude Code v2.1.242 o posterior.

* **Alcance**: [`Usuario o administrado`](#scopes). Claude Code lee la clave de configuraciones administradas, `--settings` y configuraciones de usuario, e la ignora en configuraciones de proyecto y local para que un repositorio que clone no pueda reetiquetear el selector. El más alto de esos tres que establece la clave proporciona toda la alineación, y Claude Code nunca combina alineaciones de dos fuentes.
* **Tipo**: objeto con una matriz `options` de filas y un Booleano `replaceBuiltInOptions` opcional
* **Predeterminado**: sin establecer, por lo que el selector muestra la alineación integrada

Este ejemplo agrega dos implementaciones de Bedrock después de la alineación integrada, bajo nombres que su equipo reconoce:

```json managed-settings.json theme={null}
{
  "modelPicker": {
    "options": [
      { "model": "us.anthropic.claude-opus-4-8", "label": "Opus (production)" },
      {
        "model": "us.anthropic.claude-sonnet-4-6",
        "label": "Sonnet (production)",
        "description": "Day-to-day work"
      }
    ]
  }
}
```

<span id="modelpicker-options" />

<span id="modelpicker-replacebuiltinoptions" />

<h4 id="fields-for-modelpicker">
  Campos para `modelPicker`
</h4>

La clave toma dos campos, uno para las filas en sí y otro para si reemplazan la alineación integrada o se agregan a ella.

| Campo                   | Tipo                                                                                       | Qué hace                                                                                                                                                                                                                                                                                        |
| :---------------------- | :----------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options`               | matriz de filas, cada una con un `model` requerido y un `label` y `description` opcionales | Las filas que muestra el selector, en este orden, excepto que una fila atenuada se mueve al final. Sin un `label`, Claude Code titula la fila con el nombre integrado para un modelo que conoce, o el ID del modelo de lo contrario, y sin una `description` escribe una segunda línea genérica |
| `replaceBuiltInOptions` | Booleano, predeterminado `false`                                                           | Establézcalo en `true` para mostrar solo estas filas, **Predeterminado** y una fila para el modelo que la sesión ya está usando. Déjelo sin establecer para agregar estas filas después de la alineación integrada                                                                              |

Con `replaceBuiltInOptions` activado, Claude Code oculta todas las demás filas: la alineación integrada, las filas que agrega para entradas [`availableModels`](#availablemodels), los modelos que [descubrimiento de puerta de enlace](/docs/es/llm-gateway-protocol#model-discovery) encontró, y [`ANTHROPIC_CUSTOM_MODEL_OPTION`](/docs/es/model-config#add-a-custom-model-option). Con él desactivado, Claude Code omite un modelo listado que la alineación integrada ya cubre. Una etiqueta cambia lo que muestra el selector, no qué modelo ejecuta Claude Code.

Una lista de permitidos [`availableModels`](#availablemodels) aún se aplica a estas filas. Antes de agregar un modelo listado a la lista de permitidos, lea [Comportamiento de fusión](/docs/es/model-config#merge-behavior): un ID de modelo específico reduce la entrada comodín de su familia. Claude Code también verifica cada fila contra la sesión antes de mostrar el selector:

* **Descartada**: una fila que Claude Code no puede servir, como un modelo retirado o un modelo al que su organización no tiene acceso
* **Atenuada**: una fila que no puede seleccionar aún, mostrada con la razón
* **Ninguna fila sobrevive**: Claude Code mantiene la alineación integrada, filtrada por la lista de permitidos como de costumbre

Claude Code descarta una fila que no puede analizar y mantiene el resto. Consulte [Reparar un archivo de configuración roto](/docs/es/settings#fix-a-broken-settings-file).

<h3 id="modelpricing">
  `modelPricing`
</h3>

Informe el gasto a las tasas que su organización paga en lugar del precio de lista. Establézcalo cuando su organización tenga tasas contratadas, para que las cifras en dólares que ven los desarrolladores coincidan con su factura. Claude Code aplica las tasas en `/usage`, la [línea de estado](/docs/es/statusline), el `total_cost_usd` del Agent SDK, el límite [`--max-budget-usd`](/docs/es/cli-reference) y la métrica de costo de [OpenTelemetry](/docs/es/monitoring-usage) y eventos. Usted proporciona las tasas: Claude Code no las lee de su contrato o la Consola de Claude. Requiere Claude Code v2.1.242 o posterior.

* **Alcance**: [`Administrado`](#scopes). Implemente la clave a través de configuraciones administradas por el servidor, una política MDM, un archivo `managed-settings.json` o un [asistente de política](/docs/es/managed-settings#compute-the-policy-with-a-helper-program). Claude Code la ignora en configuraciones de usuario, proyecto y local, en `--settings` y en Windows en el [registro HKCU](/docs/es/managed-settings#where-each-mechanism-stores-the-policy) escribible por el usuario. Con configuraciones administradas por el servidor, cada sesión informa costos al precio de lista hasta que la [búsqueda de configuración](/docs/es/server-managed-settings#fetch-and-caching-behavior) de esa sesión haya confirmado la configuración. Una aplicación host que incrusta Claude Code y establece [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/es/env-vars) puede proporcionar una tabla propia a través de la opción [`managedSettings`](/docs/es/agent-sdk/typescript#options) del SDK, que Claude Code utiliza solo cuando ninguna fuente administrada establece la clave y solo en Claude Code v2.1.246 o posterior.
* **Tipo**: objeto con un `multiplier` opcional y un mapa `overrides` opcional
* **Predeterminado**: sin establecer, por lo que Claude Code informa el precio de lista a menos que una aplicación host proporcione una tabla

Establezca `multiplier` solo para un descuento plano o marcado, `overrides` solo para tasas por modelo, o ambos.

Este ejemplo establece tasas contratadas para Sonnet 4.6 y luego reduce cada cifra, la fila Sonnet incluida, en un 15%:

```json managed-settings.json theme={null}
{
  "modelPricing": {
    "multiplier": 0.85,
    "overrides": {
      "claude-sonnet-4-6": {
        "input": 2.4,
        "output": 12,
        "cacheRead": 0.24,
        "cacheWrite": 3
      }
    }
  }
}
```

Establezca `multiplier` por encima de 1, hasta 10, para marcar cada cifra. Un marcado requiere Claude Code v2.1.271 o posterior. Las versiones anteriores ignoran un `multiplier` por encima de 1 con una advertencia y mantienen el resto de la configuración.

Para los pasos, incluida la forma de confirmar que las tasas están en vigor, consulte [Informar el gasto a sus tasas contratadas](/docs/es/costs#report-spend-at-your-contracted-rates).

<span id="modelpricing-multiplier" />

<span id="modelpricing-overrides" />

<h4 id="fields-for-modelpricing">
  Campos para `modelPricing`
</h4>

| Campo        | Tipo                                                                                                              | Qué hace                                                                                                                                                                                                                                                          |
| :----------- | :---------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `multiplier` | número mayor que 0 y como máximo 10                                                                               | Escala cada costo que Claude Code calcula, independientemente de si una fila `overrides` la cubre. Por debajo de 1 es un descuento, por encima de 1 un marcado                                                                                                    |
| `overrides`  | mapa de ID de modelo a un objeto de tasa con `input`, `output`, `cacheRead` y `cacheWrite`, cada uno de 0 a 10000 | Las tasas USD por millón de tokens para ese modelo, las cuatro requeridas. `cacheWrite` cubre tanto escrituras de caché de cinco minutos como de una hora. Consulte [Qué modelos se aplica una fila de modelPricing](#which-models-a-modelpricing-row-applies-to) |

Claude Code utiliza las tasas de una fila exactamente como las escribió, sin agregar el recargo de modo rápido o la [tasa de inferencia solo para EE.UU.](https://platform.claude.com/docs/en/about-claude/pricing). Si también establece `multiplier`, Claude Code la aplica además de las tasas de la fila. Claude Code descarta una fila con una tasa que no puede analizar, o un `multiplier` que no puede analizar, y mantiene el resto; consulte [Reparar un archivo de configuración roto](/docs/es/settings#fix-a-broken-settings-file).

<h4 id="which-models-a-modelpricing-row-applies-to">
  Qué modelos se aplica una fila de `modelPricing`
</h4>

Claude Code decide a qué modelos se aplica una fila desde la clave de la fila:

* **ID de un modelo integrado**: una clave que Claude Code utiliza para un modelo integrado, ya sea que esa clave sea el ID del modelo, como `claude-sonnet-4-6`, o su ID de Bedrock, Plataforma de Agentes o Foundry. Claude Code aplica la fila a cada ID de instantánea fechada e ID específico del proveedor de ese modelo.
* **Cualquier otra clave**: una clave que no es el ID de un modelo integrado, como un alias de modelo de puerta de enlace. Claude Code aplica la fila solo a ese ID. Cuando un ID de modelo coincide exactamente con una de sus claves y también cae bajo una fila codificada por un ID de modelo integrado, Claude Code utiliza la coincidencia exacta.
* **Un perfil de inferencia de aplicación de Bedrock**: una vez que Claude Code ha resuelto el perfil al modelo al que enruta, a través de su mapa [`modelOverrides`](#modeloverrides) o la búsqueda [`bedrock:GetInferenceProfile`](/docs/es/amazon-bedrock#iam-configuration), Claude Code aplica la fila de ese modelo al perfil.

<h3 id="modelsettings">
  `modelSettings`
</h3>

Guarde un [nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) para cada modelo que utilice. Requiere Claude Code v2.1.251 o posterior.

En una sesión interactiva en su máquina, cuando guarda `low`, `medium`, `high` o `xhigh` como su predeterminado con `/effort` o el control deslizante de esfuerzo del selector `/model`, Claude Code escribe ese nivel aquí bajo el modelo que está utilizando, por lo que rara vez edita esta clave usted mismo. Cuando elige uno de esos niveles en el [selector de modelos de la extensión VS Code](/docs/es/vs-code#use-the-prompt-box), Claude Code lo guarda aquí de la misma manera. La entrada [`effortLevel`](#effortlevel) enumera las sesiones donde `/effort` se aplica solo a esa sesión.

Edite la clave manualmente para cambiar o eliminar un nivel que guardó.

Un `effortLevel` de un modelo aquí tiene precedencia sobre el [`effortLevel`](#effortlevel) de nivel superior en el mismo archivo de configuración. Entre archivos, Claude Code resuelve cada modelo por separado: el [archivo de configuración](/docs/es/settings#settings-precedence) de mayor precedencia que establece un `effortLevel` para ese modelo o el `effortLevel` de nivel superior que [se aplica a ese modelo](#effortlevel) decide, por lo que un `effortLevel` en configuraciones administradas supera un nivel que guardó en configuraciones de usuario. [Ajustar el nivel de esfuerzo](/docs/es/model-config#adjust-effort-level) enumera qué más puede anular un nivel guardado, como `--effort` al iniciar.

Para limitar el esfuerzo de un modelo en lugar de establecer su nivel, agregue un campo [`maxEffortLevel`](#maxeffortlevel) a la entrada de ese modelo. El campo requiere Claude Code v2.1.267 o posterior.

* **Alcance**: [`Cualquier archivo`](#scopes)
* **Tipo**: objeto que asigna un nombre de modelo a un objeto con un campo `effortLevel`, uno de `"low"`, `"medium"`, `"high"` u `"xhigh"`, un campo [`maxEffortLevel`](#maxeffortlevel) o ambos
* **Predeterminado**: sin establecer

Claude Code escribe cada entrada bajo el nombre canónico del modelo, como `claude-opus-5-5`, y coincide con el alias de ese modelo, con sufijo de fecha, `[1m]` e IDs específicos del proveedor reconocidos a la misma entrada.

Este ejemplo mantiene Opus 5.5 en `high` mientras otros modelos utilizan sus propios niveles guardados o predeterminados:

```json settings.json theme={null}
{
  "modelSettings": {
    "claude-opus-5-5": {
      "effortLevel": "high"
    }
  }
}
```

Ejecute `/effort auto` para borrar su nivel guardado para el modelo que está utilizando. Claude Code deja las otras entradas y cualquier `effortLevel` de nivel superior en su lugar.

<h3 id="outputstyle">
  `outputStyle`
</h3>

Seleccione un [estilo de salida](/docs/es/output-styles) por nombre. Un estilo de salida es un conjunto guardado de instrucciones que cambia el rol, tono y formato de salida de Claude, como los estilos Explanatory y Learning integrados o uno que escribió usted mismo.

Si cambia esta clave durante una sesión, Claude utiliza el nuevo estilo a partir de su siguiente mensaje. Para lo que ese mensaje cuesta en almacenamiento en caché de solicitudes, consulte [Cambiar estilo de salida](/docs/es/prompt-caching#changing-output-style). Antes de v2.1.251, la edición se aplicaba solo después de ejecutar `/clear` o iniciar una nueva sesión.

* **Alcance**: [`Cualquier archivo`](#scopes)
* **Tipo**: cadena, el nombre de un estilo de salida [integrado](/docs/es/output-styles#built-in-output-styles) o [personalizado](/docs/es/output-styles#create-a-custom-output-style)
* **Predeterminado**: sin establecer, por lo que Claude Code utiliza el estilo predeterminado

Este ejemplo selecciona el estilo Explanatory integrado, que agrega información educativa entre tareas:

```json settings.json theme={null}
{
  "outputStyle": "Explanatory"
}
```

<h3 id="promptcachettl">
  `promptCacheTtl`
</h3>

Elija cuánto tiempo el [caché de solicitud](/docs/es/prompt-caching) mantiene la conversación principal. Esta clave se aplica a sus turnos interactivos, `-p` y Agent SDK, junto con los asistentes que Claude Code ejecuta en línea con ellos. La vida útil de una hora mantiene el caché activo en descansos más largos, y la API [factura cada escritura de caché a una tasa más alta](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing) que en la vida útil de cinco minutos. Requiere Claude Code v2.1.242 o posterior.

* **Alcance**: [`Cualquier archivo`](#scopes)
* **Tipo**: cadena, uno de:
  * `"5m"`: el caché se mantiene durante cinco minutos
  * `"1h"`: el caché se mantiene durante una hora
* **Predeterminado**: sin establecer, por lo que cada solicitud de conversación principal obtiene su [vida útil predeterminada](/docs/es/prompt-caching#which-ttl-each-request-gets)
* **Anulaciones por sesión**: [`FORCE_PROMPT_CACHING_5M`](/docs/es/env-vars) tiene precedencia sobre todo lo demás, luego [`CLAUDE_CODE_PROMPT_CACHE_TTL`](/docs/es/env-vars), luego esta clave, y por último [`ENABLE_PROMPT_CACHING_1H`](/docs/es/env-vars)

Este ejemplo mantiene la conversación principal en la vida útil de una hora y deja los subagentes en cinco minutos:

```json settings.json theme={null}
{
  "promptCacheTtl": "1h",
  "subagentPromptCacheTtl": "5m"
}
```

Para lo que cuesta cada vida útil, consulte [Vida útil del caché](/docs/es/prompt-caching#cache-lifetime).

<h3 id="showthinkingsummaries">
  `showThinkingSummaries`
</h3>

Vea resúmenes del [pensamiento extendido](/docs/es/model-config#extended-thinking) de Claude en sesiones interactivas. Establézcalo si desea los resúmenes completos cuando expande el pensamiento con `Ctrl+O`. Cuando no está establecido o es `false`, la API de Anthropic redacta bloques de pensamiento y Claude Code muestra un resumen contraído; los proveedores de terceros no redactan.

* **Alcance**: [`Cualquier archivo`](#scopes)
* **Tipo**: Booleano
  * `true`: ve resúmenes completos de pensamiento cuando expande el pensamiento con `Ctrl+O`
  * `false`: la API de Anthropic redacta bloques de pensamiento y Claude Code muestra un resumen contraído
* **Predeterminado**: `false`

```json settings.json theme={null}
{
  "showThinkingSummaries": true
}
```

La redacción cambia solo lo que ve, no lo que genera el modelo. Para reducir el gasto de pensamiento, [reduzca el presupuesto o desactive el pensamiento](/docs/es/model-config#extended-thinking) en su lugar.

<h3 id="subagentpromptcachettl">
  `subagentPromptCacheTtl`
</h3>

Elija cuánto tiempo el [caché de solicitud](/docs/es/prompt-caching) mantiene las solicitudes que Claude Code realiza fuera de la conversación principal. Esta clave se aplica a [subagentes](/docs/es/sub-agents), [flujos de trabajo](/docs/es/workflows) y las solicitudes propias de Claude Code en segundo plano y asistentes, como compactación y títulos de sesión. La vida útil de una hora mantiene el caché activo en descansos más largos, y la API [factura cada escritura de caché a una tasa más alta](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing) que en la vida útil de cinco minutos. Requiere Claude Code v2.1.242 o posterior.

* **Alcance**: [`Cualquier archivo`](#scopes)
* **Tipo**: cadena, uno de:
  * `"5m"`: el caché se mantiene durante cinco minutos
  * `"1h"`: el caché se mantiene durante una hora
* **Predeterminado**: sin establecer, por lo que cada una de estas solicitudes obtiene su [vida útil predeterminada](/docs/es/prompt-caching#which-ttl-each-request-gets)
* **Anulaciones por sesión**: [`FORCE_PROMPT_CACHING_5M`](/docs/es/env-vars) tiene precedencia sobre todo lo demás, luego [`CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`](/docs/es/env-vars), luego esta clave, luego [`ENABLE_PROMPT_CACHING_1H`](/docs/es/env-vars), que solicita la vida útil de una hora en cada solicitud. Para dónde se clasifica el valor de frontmatter propio de un subagente, consulte [Elegir el TTL usted mismo](/docs/es/prompt-caching#choose-the-ttl-yourself)

Este ejemplo proporciona a los subagentes y las otras solicitudes fuera de la conversación principal la vida útil de una hora:

```json settings.json theme={null}
{
  "subagentPromptCacheTtl": "1h"
}
```

Esta clave cubre las solicitudes que [`promptCacheTtl`](#promptcachettl) no cubre, por lo que establezca ambas para elegir una vida útil para cada solicitud que Claude Code realiza. Para cómo difiere el caché de un subagente del de la conversación principal, consulte [Subagentes y el caché](/docs/es/prompt-caching#subagents-and-the-cache).

<h3 id="switchmodelsonflag">
  `switchModelsOnFlag`
</h3>

Elija qué sucede cuando un [clasificador de seguridad marca una solicitud](/docs/es/model-config#automatic-model-fallback): cambiar al modelo de respaldo y continuar, o pausar para que pueda elegir entre cambiar y editar el mensaje.

* **Alcance**: [`Cualquier archivo`](#scopes). Aparece en `/config` como **Cambiar modelos cuando se marca un mensaje**.
* **Tipo**: Booleano
  * `true`: Claude Code cambia al modelo de respaldo y continúa
  * `false`: en una sesión interactiva Claude Code pausa para que pueda elegir entre cambiar y editar el mensaje; donde no puede mostrarse un diálogo, como una ejecución `-p`, la solicitud marcada termina como un error
* **Predeterminado**: `true`, cambiar automáticamente

```json settings.json theme={null}
{
  "switchModelsOnFlag": false
}
```

Consulte [Preguntar antes de cambiar](/docs/es/model-config#ask-before-switching).

<h3 id="ultracode">
  `ultracode`
</h3>

Inicie sesiones con [ultracode](/docs/es/workflows#let-claude-decide-with-ultracode) activado. Con él activado, Claude planifica un flujo de trabajo para cada tarea sustancial en lugar de esperar a que lo pida. Claude planifica flujos de trabajo solo cuando [flujos de trabajo dinámicos](/docs/es/workflows) están habilitados para usted, su modelo admite esfuerzo `xhigh` y no se aplica [límite de esfuerzo](/docs/es/model-config#organization-effort-limits) por debajo de `xhigh`. De cualquier forma, `ultracode: true` ejecuta la sesión en esfuerzo `xhigh`, o en el límite cuando un límite de esfuerzo es más bajo. Claude Code lee esta clave pero nunca la escribe: `/effort ultracode` activa ultracode solo para la sesión actual.

* **Alcance**: [`Cualquier archivo`](#scopes)
* **Tipo**: Booleano
  * `true`: las sesiones comienzan en esfuerzo `xhigh`, con ultracode activado cuando flujos de trabajo dinámicos están habilitados para usted, su modelo admite `xhigh` y no se aplica límite de esfuerzo por debajo de `xhigh`
  * `false`: las sesiones comienzan con ultracode desactivado
* **Predeterminado**: sin establecer, por lo que ultracode está desactivado
* **Anulaciones por sesión**: `/effort ultracode` activa ultracode para una sesión sin esta clave. También lo hace `--effort ultracode`, que requiere Claude Code v2.1.203 o posterior

```json settings.json theme={null}
{
  "ultracode": true
}
```

Ultracode ejecuta la sesión en esfuerzo `xhigh` y tiene precedencia sobre `effortLevel` y entradas [`modelSettings`](#modelsettings). Si se aplica un [límite de esfuerzo](/docs/es/model-config#organization-effort-limits) por debajo de `xhigh` al modelo, como una configuración [`maxEffortLevel`](#maxeffortlevel), la sesión se ejecuta en el límite en su lugar y ultracode permanece desactivado. Claude entonces no planifica flujos de trabajo por su cuenta, y `/effort` no ofrece `ultracode`. Una solicitud de control `apply_flag_settings` del Agent SDK también acepta la clave.

<h2 id="permission-settings">
  Configuración de permisos
</h2>

Decida qué puede hacer Claude sin preguntar, en qué modo de permisos comienza una sesión y qué permite el clasificador del modo automático. Para la sintaxis de reglas y el modelo de permisos, consulte [Configurar permisos](/docs/es/permissions).

<h3 id="allowmanagedpermissionrulesonly">
  `allowManagedPermissionRulesOnly`
</h3>

Haga que la configuración administrada sea la única fuente de reglas de permisos. Claude Code ignora entonces las reglas `allow`, `ask` y `deny` en archivos de usuario, proyecto, local y `--settings`, ignora `--allowedTools`, oculta las opciones de permitir siempre en los avisos de permisos y deja de guardar nuevas reglas.

Cuando se aplican [configuraciones principales de un host de incrustación](/docs/es/managed-settings#let-an-embedding-host-add-policy), Claude Code las trata como parte del nivel administrado. Descarta sus reglas `allow` y `additionalDirectories`, y mantiene sus reglas `deny` y `ask` excepto las reglas `Read` y `Edit` cuyo patrón comienza con `!`. Un host no puede excluir rutas de las reglas administradas con una regla `!`, independientemente de si establece esta clave.

Las reglas `--disallowedTools` y las reglas `deny` y `ask` de la sesión actual aún se aplican, incluso después de que Claude Code recargue la configuración a mitad de la sesión. Solo restringen, por lo que no pueden ampliar lo que otorgan las reglas administradas. Antes de v2.1.257, Claude Code descartaba esas reglas de línea de comandos y sesión en la primera recarga de configuración.

Para lo que un patrón `!` en una regla `--disallowedTools` o de sesión puede excluir, consulte [Reglas Read y Edit](/docs/es/permissions#read-and-edit).

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: la configuración administrada se convierte en la única fuente de reglas de permisos
  * `false`: Claude Code aplica reglas de permisos de archivos de usuario, proyecto, local y `--settings` además de las administradas
* **Default**: sin establecer, por lo que Claude Code aplica reglas de permisos de configuración de usuario, proyecto y local y de `--settings`, además de las administradas

```json managed-settings.json theme={null}
{
  "allowManagedPermissionRulesOnly": true
}
```

Esta clave no bloquea la lista de permitidos del servidor MCP; para eso, establezca [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly). Consulte [Configuración solo administrada](/docs/es/managed-settings#managed-only-settings).

<h3 id="automode">
  `autoMode`
</h3>

Agregue sus propias reglas a lo que el clasificador del [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) bloquea y permite. Úselo para indicar al clasificador qué repositorios, depósitos y dominios confía su organización, para que deje de bloquear operaciones internas rutinarias. El clasificador incluye [reglas de permitir y denegar integradas](/docs/es/auto-mode-config#inspect-the-defaults-and-your-effective-config). Incluya la cadena literal `"$defaults"` en una matriz para mantener esas reglas integradas en esa posición y agregue las suyas alrededor; omítala para reemplazarlas con las suyas.

* **Scope**: [`User or managed`](#scopes)
* **Type**: objeto con matrices `environment`, `allow`, `soft_deny` y `hard_deny` de reglas en prosa, más el Boolean [`classifyAllShell`](#automode-classifyallshell)
* **Default**: sin establecer, por lo que el clasificador usa solo sus [reglas integradas](/docs/es/auto-mode-config#inspect-the-defaults-and-your-effective-config)

Este ejemplo mantiene las reglas `soft_deny` integradas, a través de `"$defaults"`, y agrega una más que bloquea `terraform apply`:

```json settings.json theme={null}
{
  "autoMode": {
    "soft_deny": ["$defaults", "Never run terraform apply"]
  }
}
```

Cuando más de uno de esos archivos establece la misma matriz, Claude Code concatena las entradas. Para el formato de regla y cómo se aplica cada matriz, consulte [Configurar modo automático](/docs/es/auto-mode-config).

<h3 id="automode-classifyallshell">
  `autoMode.classifyAllShell`
</h3>

Envíe cada comando Bash y PowerShell a través del clasificador del modo automático mientras el modo automático está activo. De forma predeterminada, el modo automático suspende solo las reglas de permitir que podrían ejecutar código arbitrario: reglas de herramienta completa y comodín como `Bash(*)`, y prefijos de intérprete o contenedor de shell como `Bash(python *)`. Un comando que coincida con cualquier otra regla de permitir, como `Bash(npm test)`, omite el clasificador a menos que lleve [dominios permitidos por comando](/docs/es/sandboxing#per-command-allowed-domains-in-auto-mode). Cuando lo omite, un argumento destructivo que el prefijo de la regla no anticipó puede pasar desapercibido. Al establecer esta clave, suspende cada regla de permitir shell para la sesión para que el clasificador vea cada comando. Requiere Claude Code v2.1.193 o posterior.

* **Scope**: [`User or managed`](#scopes). Léase donde se lee [`autoMode`](#automode).
* **Type**: Boolean
  * `true`: mientras el modo automático está activo, Claude Code envía cada comando Bash y PowerShell a través del clasificador y suspende sus reglas de permitir shell; fuera del modo automático las reglas aún se aplican
  * `false`: el modo automático suspende solo las reglas de permitir que podrían ejecutar código arbitrario, como `Bash(*)` y `Bash(python *)`; un comando que coincida con cualquier otra regla de permitir omite el clasificador a menos que lleve [dominios permitidos por comando](/docs/es/sandboxing#per-command-allowed-domains-in-auto-mode), y cada otro comando shell pasa por él
* **Default**: `false`

```json settings.json theme={null}
{
  "autoMode": {
    "classifyAllShell": true
  }
}
```

Consulte [Enrutar todos los comandos shell a través del clasificador](/docs/es/auto-mode-config#route-all-shell-commands-through-the-classifier). Requiere Claude Code v2.1.193 o posterior.

<h3 id="disableautomode">
  `disableAutoMode`
</h3>

Elimine el [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) del ciclo `Shift+Tab`. Cualquier sesión que de otro modo [comenzaría en modo automático](/docs/es/permission-modes#which-mode-a-session-starts-in), ya sea desde `--permission-mode auto`, un archivo de configuración o el valor predeterminado integrado, comienza en `default` en su lugar. Los administradores lo establecen en la configuración administrada para evitar que los desarrolladores de su organización usen el modo automático.

* **Scope**: [`Any file`](#scopes). Más útil en [configuración administrada](/docs/es/managed-settings), donde los usuarios no pueden anularlo. También aceptado bajo `permissions` como `permissions.disableAutoMode`.
* **Type**: la cadena `"disable"`
* **Default**: sin establecer

```json settings.json theme={null}
{
  "disableAutoMode": "disable"
}
```

<h3 id="permissions">
  `permissions`
</h3>

Controle qué herramientas puede usar Claude sin preguntar, cuáles siempre solicitan confirmación y cuáles están bloqueadas, y establezca el [modo de permisos](/docs/es/permission-modes) en el que comienza una sesión. Cada clave `permissions.*` a continuación se anida bajo este objeto.

* **Scope**: [`Any file`](#scopes)
* **Type**: objeto con `allow`, `ask`, `deny`, `additionalDirectories`, `blockReadsOutsideWorkingDirectories`, `defaultMode`, `disableBypassPermissionsMode` y `disableAutoMode`
* **Default**: sin establecer

Este ejemplo aprueba comandos `npm run` sin preguntar, solicita confirmación antes de `git push`, bloquea lecturas de `.env` e inicia sesiones en `acceptEdits`:

```json settings.json theme={null}
{
  "permissions": {
    "allow": ["Bash(npm run *)"],
    "ask": ["Bash(git push *)"],
    "deny": ["Read(./.env)"],
    "defaultMode": "acceptEdits"
  }
}
```

Las tres matrices de reglas comparten una sintaxis; consulte [Sintaxis de regla de permisos](#permission-rule-syntax) bajo `permissions.allow`. Para cómo se combinan las reglas de permisos de diferentes archivos, consulte [cómo se fusionan las reglas de permisos entre ámbitos](/docs/es/permissions#settings-precedence); para cómo se combinan las claves de configuración en general, consulte [Precedencia de configuración](/docs/es/settings#settings-precedence) en la guía de configuración.

<h3 id="useautomodeduringplan">
  `useAutoModeDuringPlan`
</h3>

Elija si Claude Code usa el clasificador del modo automático para revisar comandos shell en modo de plan. Con el valor predeterminado `true`, el clasificador revisa cada comando durante la planificación cuando el modo automático está disponible y no ve ningún aviso. Establezca `false` para obtener un aviso de permisos para cada comando fuera del conjunto integrado de solo lectura. Aparece en `/config` como **Use auto mode during plan**.

* **Scope**: [`User, local, or managed`](#scopes). Un repositorio no puede desactivarlo para usted.
* **Type**: Boolean
  * `true`: lo mismo que sin establecer; cuando el modo automático está disponible, el clasificador revisa cada comando shell durante la planificación en lugar de solicitarle confirmación. Un `false` en cualquiera de estos archivos aún lo desactiva
  * `false`: obtiene un aviso de permisos para cada comando fuera del conjunto integrado de solo lectura
* **Default**: `true`

```json settings.json theme={null}
{
  "useAutoModeDuringPlan": false
}
```

<h3 id="permissions-allow">
  `permissions.allow`
</h3>

Enumere los usos de herramientas que Claude Code aprueba sin preguntarle. En una regla MCP, `*` puede aparecer solo en el nombre de la herramienta después del prefijo `mcp__<server>__`, como `mcp__github__get_*`; no puede aparecer en el nombre del servidor.

* **Scope**: [`Any file`](#scopes)
* **Type**: matriz de cadenas de regla de permisos
* **Default**: sin establecer
* **Per-session overrides**: `--allowedTools` agrega reglas de permitir para una sesión, y una regla de denegar de cualquier archivo de configuración aún bloquea una herramienta que nombra

Este ejemplo aprueba `git diff` y permite que Claude Code lea su `.zshrc` sin preguntar:

```json settings.json theme={null}
{
  "permissions": {
    "allow": ["Bash(git diff *)", "Read(~/.zshrc)"]
  }
}
```

Claude Code aplica reglas `allow` del `.claude/settings.json` de un proyecto solo después de que acepte el [diálogo de confianza del espacio de trabajo](/docs/es/permissions#project-allow-rules-and-workspace-trust) para esa carpeta.

<h4 id="permission-rule-syntax">
  Sintaxis de regla de permisos
</h4>

Las reglas de permisos siguen el formato `Tool` o `Tool(specifier)`. Claude Code evalúa primero las reglas `deny`, luego `ask`, luego `allow`, y la primera coincidencia decide independientemente de cuán específica sea cada regla; consulte el [orden de evaluación de reglas de permisos](/docs/es/permissions#manage-permissions).

Cada fila muestra una forma de regla y lo que coincide.

| Rule                           | What it matches                  |
| :----------------------------- | :------------------------------- |
| `Bash`                         | Every Bash command               |
| `Bash(npm run *)`              | Commands starting with `npm run` |
| `Read(./.env)`                 | Reads of the `.env` file         |
| `WebFetch(domain:example.com)` | Fetch requests to example.com    |

Para la sintaxis de regla completa, incluido el comportamiento de comodín, patrones específicos de herramientas para Read, Edit, WebFetch, MCP y reglas de Agent, y las limitaciones de seguridad de patrones Bash, consulte [Sintaxis de regla de permisos](/docs/es/permissions#permission-rule-syntax).

<h3 id="permissions-ask">
  `permissions.ask`
</h3>

Enumere los usos de herramientas que le solicitan confirmación incluso en un modo de permisos que de otro modo los aprobaría, como `acceptEdits` o `bypassPermissions`. En modo `dontAsk`, Claude Code deniega un uso de herramienta coincidente en lugar de solicitar confirmación.

* **Scope**: [`Any file`](#scopes)
* **Type**: matriz de cadenas de regla de permisos
* **Default**: sin establecer

```json settings.json theme={null}
{
  "permissions": {
    "ask": ["Bash(git push *)"]
  }
}
```

<span id="exclude-sensitive-files" />

<h3 id="permissions-deny">
  `permissions.deny`
</h3>

Enumere los usos de herramientas que Claude Code bloquea. Úselo para archivos que contienen claves API, secretos o valores de entorno: Claude Code excluye archivos coincidentes del descubrimiento de archivos y resultados de búsqueda, deniega lecturas de ellos y bloquea las herramientas [Edit y Write](/docs/es/permissions#read-and-edit) en las rutas coincidentes.

Las reglas de denegar Read y Edit se aplican a las herramientas de archivo integradas de Claude, a comandos de archivo que Claude Code reconoce en Bash, como `cat`, `head`, `tail`, `sed` y `tee`, y a los destinos de [redirecciones](/docs/es/permissions#redirections) de Bash como `> file` y `< file`; no se aplican a un comando que lee archivos sin nombrarlos, como `grep -r pattern .`, o a subprocesos arbitrarios, por lo que para la aplicación a nivel del SO [habilite el sandbox](/docs/es/sandboxing).

* **Scope**: [`Any file`](#scopes)
* **Type**: matriz de cadenas de regla de permisos
* **Default**: sin establecer
* **Per-session overrides**: `--disallowedTools` agrega reglas de denegar para una sesión junto a esta clave

Este ejemplo deniega lecturas de archivos `.env`, el directorio `secrets` y un archivo de credenciales, y bloquea comandos `curl`:

```json settings.json theme={null}
{
  "permissions": {
    "deny": [
      "Read(./.env)",
      "Read(./.env.*)",
      "Read(./secrets/**)",
      "Read(./config/credentials.json)",
      "Bash(curl *)"
    ]
  }
}
```

Los nombres de herramientas aceptan patrones glob, por lo que `"*"` deniega cada herramienta y `"mcp__*"` deniega cada herramienta MCP. Claude Code ignora una regla de denegar para la herramienta [`EndConversation`](/docs/es/tools-reference#endconversation-tool-behavior) siempre que cualquier otra herramienta aún esté disponible para Claude. Una regla de denegar `Bash` coincide con el comando tal como Claude lo escribe, por lo que `Bash(curl *)` no detiene `/usr/bin/curl` o `sh -c 'curl …'`; consulte [lo que una regla Bash no coincide](/docs/es/permissions#bash-rule-limits). Esta clave reemplaza la configuración `ignorePatterns` obsoleta.

<h3 id="permissions-additionaldirectories">
  `permissions.additionalDirectories`
</h3>

Otorgue a Claude acceso a archivos en directorios fuera del que comenzó, como [directorios de trabajo](/docs/es/permissions#working-directories) adicionales. La mayoría de la configuración `.claude/` [no se descubre](/docs/es/permissions#additional-directories-grant-file-access-not-configuration) desde estos directorios.

* **Scope**: [`Any file`](#scopes)
* **Type**: matriz de rutas de directorio
* **Default**: sin establecer
* **Per-session overrides**: `--add-dir` y `/add-dir` agregan directorios para una sesión junto a esta clave

```json settings.json theme={null}
{
  "permissions": {
    "additionalDirectories": ["../docs/"]
  }
}
```

Como las reglas `allow`, las entradas en el `.claude/settings.json` de un proyecto tienen efecto solo después de que acepte el [diálogo de confianza del espacio de trabajo](/docs/es/permissions#project-allow-rules-and-workspace-trust) para esa carpeta.

<h3 id="permissions-blockreadsoutsideworkingdirectories">
  `permissions.blockReadsOutsideWorkingDirectories`
</h3>

Impida que Claude lea rutas fuera de los [directorios de trabajo](/docs/es/permissions#working-directories) de la sesión con las herramientas Read, Grep, Glob y LSP, en cada modo de permisos incluyendo `bypassPermissions`. Un comando Bash que lee una ruta coincidente a través de un comando de archivo que Claude Code reconoce, como `cat`, le solicita confirmación incluso en modo automático y modo `bypassPermissions`. Requiere Claude Code v2.1.257 o posterior.

Un comando Bash que el analizador de shell no puede rastrear, como uno que cambia de directorio más de una vez o ejecuta un subshell, le solicita confirmación incluso en modo automático y modo `bypassPermissions`. El aviso aparece incluso cuando el comando no nombra ninguna ruta fuera de los directorios de trabajo. Este aviso no se aplica cuando el comando se ejecuta en el [sandbox](/docs/es/sandboxing) y el sandbox hace cumplir el bloqueo.

Claude Code también escribe `true` aquí cuando elige bloquear tales lecturas en [el aviso del modo automático antes de la primera lectura fuera de los directorios de trabajo](/docs/es/permission-modes#first-read-outside-the-working-directories).

* **Scope**: [`Any file`](#scopes). Si cualquier fuente de configuración establece `true`, se aplica el bloqueo, por lo que un archivo registrado de un repositorio puede activar el bloqueo para un proyecto pero no puede levantar un bloqueo que establezca.
* **Type**: Boolean
  * `true`: las lecturas de archivos fuera de los directorios de trabajo están bloqueadas
  * `false`: lo mismo que sin establecer; un `true` en cualquier otro archivo de configuración aún bloquea
* **Default**: sin establecer, por lo que las lecturas fuera de los directorios de trabajo siguen su modo de permisos y reglas

```json settings.json theme={null}
{
  "permissions": {
    "blockReadsOutsideWorkingDirectories": true
  }
}
```

Si solo el archivo de configuración registrado de un repositorio agrega un directorio, el bloqueo aún se aplica a las lecturas allí. Cuando [`autoMemoryDirectory`](#automemorydirectory) proviene del `.claude/settings.json` del proyecto, o de un `.claude/settings.local.json` [tratado como suministrado por repositorio](/docs/es/permissions#when-your-local-settings-file-needs-trust), Claude Code no carga ninguna [memoria automática](/docs/es/memory#storage-location) desde ese directorio y no guarda ninguna en él. Los archivos que Claude Code necesita permanecen legibles, como sus skills, plugins, reglas, agents, comandos y el archivo de memoria `CLAUDE.md` bajo `~/.claude/`.

Cuando el [sandbox](/docs/es/sandboxing) está activado, el bloqueo también deniega a los comandos en sandbox acceso de lectura a directorios de inicio y raíces de volúmenes montados fuera de los directorios de trabajo. Un reintento que necesita aprobación para [ejecutarse fuera del sandbox](/docs/es/sandboxing#the-unsandboxed-retry-escape-hatch) le solicita confirmación incluso en modo `bypassPermissions`. Los archivos que una herramienta lee desde su directorio de inicio, como `~/.gitconfig`, se deniegan con el resto; reabra una ruta específica con [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread) cuando una herramienta la necesita.

Cuando el directorio de trabajo de la sesión es un [git worktree](/docs/es/worktrees) vinculado, incluyendo uno que Claude Code ingresó a mitad de la sesión, el directorio común `.git` del repositorio permanece legible y escribible para comandos en sandbox, por lo que git sigue funcionando allí.

<h3 id="permissions-defaultmode">
  `permissions.defaultMode`
</h3>

Establezca el [modo de permisos](/docs/es/permission-modes) en el que comienzan las nuevas sesiones. Cuando lo deja sin establecer, las sesiones comienzan en el [valor predeterminado integrado](/docs/es/permission-modes#which-mode-a-session-starts-in) para su plan y superficie.

* **Scope**: [`Any file`](#scopes). `auto` y `bypassPermissions` no tienen efecto desde la configuración de proyecto o local, por lo que establézcalos en `~/.claude/settings.json` en su lugar. Antes de v2.1.257, `bypassPermissions` tenía efecto desde cualquier archivo. Para conversaciones que inicia la extensión VS Code, Claude Code lee solo valores de usuario, administrado y `--settings`.
* **Type**: cadena, una de:
  * `"default"`: Claude Code ejecuta solo lecturas sin preguntar
  * `"acceptEdits"`: Claude Code también ejecuta ediciones de archivos y comandos comunes del sistema de archivos como `mkdir` y `mv` sin preguntar
  * `"plan"`: Claude Code lee y planifica pero bloquea ediciones hasta que apruebe un plan
  * `"auto"`: Claude Code ejecuta todo, con verificaciones de seguridad en segundo plano
  * `"dontAsk"`: Claude Code auto-deniega cada llamada que de otro modo solicitaría confirmación; las lecturas, otras acciones que no necesitan aprobación y herramientas pre-aprobadas aún se ejecutan
  * `"bypassPermissions"`: Claude Code ejecuta todo sin preguntar
  * `"manual"`: un alias para `"default"`, en Claude Code v2.1.200 o posterior
* **Default**: sin establecer
* **Per-session overrides**: `--permission-mode`, y su equivalente `--dangerously-skip-permissions` para `bypassPermissions`, tienen precedencia sobre esta clave para una sesión

```json settings.json theme={null}
{
  "permissions": {
    "defaultMode": "acceptEdits"
  }
}
```

Las reglas de permisos se superponen en cada modo: las reglas `deny` bloquean en cada modo, incluyendo `bypassPermissions`. Consulte [Modos de permisos](/docs/es/permission-modes). `manual` nombra el modo de permisos etiquetado Manual en la CLI y la extensión VS Code; el alias requiere Claude Code v2.1.200 o posterior. En sesiones en la nube, Claude Code honra solo `acceptEdits`, `plan`, `default` y `auto` de esta clave. Para conversaciones que inicia la extensión VS Code, consulte [qué configuración lee la extensión para el modo de permisos inicial](/docs/es/permission-modes#switch-permission-modes).

<h3 id="permissions-disablebypasspermissionsmode">
  `permissions.disableBypassPermissionsMode`
</h3>

Impida que alguien ingrese al modo `bypassPermissions`. Claude Code rechaza entonces la bandera `--dangerously-skip-permissions` e ignora una [definición de agent](/docs/es/sub-agents#permission-modes) `permissionMode: bypassPermissions`, por lo que el subagente se ejecuta con el modo de permisos de la sesión principal.

* **Scope**: [`Any file`](#scopes). Típicamente establecido en [configuración administrada](/docs/es/managed-settings) para hacer cumplir la política organizacional.
* **Type**: la cadena `"disable"`
* **Default**: sin establecer
* **Per-session overrides**: esta clave tiene precedencia sobre `--dangerously-skip-permissions`, que Claude Code rechaza mientras la clave está establecida

```json settings.json theme={null}
{
  "permissions": {
    "disableBypassPermissionsMode": "disable"
  }
}
```

Antes de v2.1.223, Claude Code aplicaba el modo de permisos de frontmatter incluso con bypass deshabilitado.

<h3 id="skipautopermissionprompt">
  `skipAutoPermissionPrompt`
</h3>

Omita el aviso único que describe el [modo automático](/docs/es/permission-modes#eliminate-prompts-with-auto-mode) que Claude Code muestra cuando ingresa al modo automático usted mismo, por ejemplo a través de su propia configuración o el selector de modo, en lugar de cuando el valor predeterminado integrado inicia una sesión en él. Claude Code muestra ese aviso una vez y luego registra que fue mostrado, por lo que esta clave solo importa donde el aviso aún no ha aparecido.

* **Scope**: [`User or managed`](#scopes). Un repositorio no puede establecerlo para usted.
* **Type**: Boolean
  * `true`: Claude Code omite el aviso
  * `false`: lo mismo que sin establecer; el aviso aparece una vez a menos que otro de estos archivos establezca `true`
* **Default**: sin establecer, por lo que el aviso aparece una vez

```json settings.json theme={null}
{
  "skipAutoPermissionPrompt": true
}
```

<h3 id="skipdangerousmodepermissionprompt">
  `skipDangerousModePermissionPrompt`
</h3>

Omita el diálogo de confirmación que Claude Code muestra antes de que una sesión ingrese al modo `bypassPermissions`, ya sea desde `--dangerously-skip-permissions` o desde `defaultMode: "bypassPermissions"`. Claude Code escribe `true` aquí en su configuración de usuario cuando acepta ese diálogo una vez.

* **Scope**: [`User, local, or managed`](#scopes). Un repositorio que no es de confianza no puede omitir el diálogo para usted.
* **Type**: Boolean
  * `true`: Claude Code omite el diálogo de confirmación antes de que una sesión ingrese al modo `bypassPermissions`
  * `false`: lo mismo que sin establecer; el diálogo aparece a menos que otro de estos archivos establezca `true`
* **Default**: sin establecer, por lo que el diálogo aparece

```json settings.json theme={null}
{
  "skipDangerousModePermissionPrompt": true
}
```

<h2 id="sandbox-settings">
  Configuración de sandbox
</h2>

Aísle los comandos que Claude ejecuta de su sistema de archivos, su red y sus credenciales. Para saber cómo funciona el sandboxing y los requisitos de plataforma, consulte [Sandboxing](/docs/es/sandboxing).

<h3 id="sandbox">
  `sandbox`
</h3>

Aísle los comandos Bash que Claude ejecuta de su sistema de archivos y red con [sandboxing](/docs/es/sandboxing). Active el sandbox con `enabled`, luego reduzca o amplíe lo que los comandos en sandbox pueden tocar con los subobjetos `filesystem`, `network` y `credentials`. El sandbox se ejecuta en macOS, Linux y WSL2.

* **Scope**: [`Any file`](#scopes)
* **Type**: object con `enabled`, `failIfUnavailable`, `autoAllowBashIfSandboxed`, `excludedCommands`, `allowUnsandboxedCommands`, `enableWeakerNestedSandbox`, `enableWeakerNetworkIsolation`, `allowAppleEvents`, `bwrapPath`, `socatPath`, `ignoreViolations` y `ripgrep`, más los objetos `filesystem`, `network` y `credentials`
* **Default**: sin establecer, por lo que Claude Code ejecuta comandos sin sandbox

Esto activa el sandbox, omite las solicitudes de permiso para comandos en sandbox, ejecuta `docker` fuera del sandbox, abre dos rutas de escritura adicionales, oculta su archivo de credenciales de AWS y pre-permite GitHub y npm:

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "autoAllowBashIfSandboxed": true,
    "excludedCommands": ["docker *"],
    "filesystem": {
      "allowWrite": ["/tmp/build", "~/.kube"],
      "denyRead": ["~/.aws/credentials"]
    },
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org"]
    }
  }
}
```

Claude Code toma el valor de una clave booleana del ámbito de configuración con mayor precedencia que la establece, por lo que un `enabled` o `failIfUnavailable` administrado anula cualquier cosa que establezca un desarrollador. Fusiona claves de matriz en todos los ámbitos de configuración que carga la sesión, por lo que un desarrollador puede agregar entradas; consulte [Keep developers from widening the policy](/docs/es/sandboxing#keep-developers-from-widening-the-policy) para los bloqueos solo administrados. Para requerir el sandbox para una organización, consulte [Enforce sandboxing with managed settings](/docs/es/sandboxing#enforce-sandboxing-with-managed-settings).

<h3 id="sandbox-enabled">
  `sandbox.enabled`
</h3>

Active [sandboxing](/docs/es/sandboxing) para comandos Bash. Cuando elige un modo en el panel `/sandbox`, Claude Code escribe esta clave en `.claude/settings.local.json` para el proyecto actual; establézcala en `~/.claude/settings.json` para sandbox en cada proyecto.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code coloca en sandbox los comandos Bash
  * `false`: Los comandos Bash se ejecutan sin sandbox
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true
  }
}
```

En Linux y WSL2, el sandbox necesita `bubblewrap` y `socat`; consulte [Set up Linux and WSL2](/docs/es/sandboxing#set-up-linux-and-wsl2). Cuando el sandbox no puede iniciarse, Claude Code muestra una advertencia y ejecuta comandos sin sandbox a menos que también establezca [`failIfUnavailable`](#sandbox-failifunavailable).

<h3 id="sandbox-failifunavailable">
  `sandbox.failIfUnavailable`
</h3>

Haga que Claude Code salga con un error al inicio cuando `sandbox.enabled` es `true` pero el sandbox no puede iniciarse, porque falta una dependencia o la plataforma no es compatible. Sin él, Claude Code muestra una advertencia y ejecuta comandos sin sandbox. Úselo en configuración administrada cuando su organización requiera sandboxing como una puerta dura.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code sale con un error al inicio cuando `sandbox.enabled` es `true` pero el sandbox no puede iniciarse
  * `false`: Claude Code muestra una advertencia y ejecuta comandos sin sandbox
* **Default**: `false`

Esto hace que cada máquina administrada coloque en sandbox los comandos o se niegue a iniciarse:

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "failIfUnavailable": true
  }
}
```

Consulte [Enforce sandboxing with managed settings](/docs/es/sandboxing#enforce-sandboxing-with-managed-settings).

<h3 id="sandbox-autoallowbashifsandboxed">
  `sandbox.autoAllowBashIfSandboxed`
</h3>

Permita que Claude Code ejecute comandos Bash en sandbox sin una solicitud de permiso. Los comandos que no pueden ejecutarse en el sandbox aún pasan por el flujo de permiso regular, y las reglas `deny` y las reglas `ask` con ámbito de contenido como `Bash(git push *)` aún se aplican; una regla `ask` de Bash desnuda se omite para comandos en sandbox. Establézcalo en `false` para enviar comandos en sandbox también a través del flujo de permiso regular, que la pestaña **Mode** de `/sandbox` llama modo de permisos regular.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code ejecuta comandos Bash en sandbox sin una solicitud de permiso, sujeto a reglas `deny` y reglas `ask` con ámbito de contenido; `CLAUDE_CODE_SUBPROCESS_ENV_SCRUB` desactiva la auto-permisión
  * `false`: los comandos en sandbox pasan por el flujo de permiso regular, por lo que sus reglas de permiso y modo de permiso deciden. La pestaña **Mode** de `/sandbox` llama a esto modo de permisos regular
* **Default**: `true`

Esto mantiene el sandbox activado y envía comandos en sandbox a través del flujo de permiso regular:

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "autoAllowBashIfSandboxed": false
  }
}
```

Consulte [Sandbox modes](/docs/es/sandboxing#sandbox-modes) para saber qué modo de auto-permisión aún solicita y cómo se comporta en modo plan.

<h3 id="sandbox-excludedcommands">
  `sandbox.excludedCommands`
</h3>

Nombre los comandos que Claude Code ejecuta fuera del sandbox, como herramientas que no funcionan bajo él. Cada entrada utiliza la misma sintaxis que el contenido de una [regla de permiso](/docs/es/permissions#permission-rule-syntax) `Bash(...)`: un comando exacto, un prefijo como `docker *` o un patrón comodín.

Sus entradas sacan una llamada Bash del sandbox solo cuando cubren cada comando en ella, y algunas formas de llamada permanecen en sandbox incluso entonces. Una entrada `docker *` sola no saca `npm ci && docker build .` del sandbox.

* **Scope**: [`Any file`](#scopes)
* **Type**: array de patrones de comando
* **Default**: sin establecer, por lo que ningún comando se excluye

```json settings.json theme={null}
{
  "sandbox": {
    "excludedCommands": ["docker *"]
  }
}
```

Claude Code mantiene una llamada Bash en sandbox cuando tiene una de estas formas, entre otras:

* Un comando que comienza con `sudo`, `eval` o `xargs`
* Un `cd`, `pushd` o `popd`, dondequiera que aparezca en la llamada
* Una sustitución de comando, un subshell o un bloque de flujo de control como `if` o `for`
* Una redirección, como `docker build . > build.log`, que no sea una que solo duplique un descriptor de archivo, como `2>&1` hace
* Un nombre de comando que proviene de una variable

Por ejemplo, `cd build && docker compose up` permanece en sandbox bajo una entrada `docker *`, y agregar una entrada `cd` no cambia eso.

Los comandos excluidos aún pasan por el flujo de permiso regular. La exclusión es una conveniencia, no un límite de seguridad: prefiera [`filesystem.allowWrite`](#sandbox-filesystem-allowwrite) cuando una herramienta solo necesita escribir en algún lugar específico. Claude Code fusiona entradas en todos los ámbitos de configuración que carga la sesión, y no hay un bloqueo solo administrado para esta lista, así que mantenga una lista administrada estrecha.

<h3 id="sandbox-allowunsandboxedcommands">
  `sandbox.allowUnsandboxedCommands`
</h3>

Permita que Claude reintente un comando fuera del sandbox con el parámetro `dangerouslyDisableSandbox` después de que el sandbox lo bloquee. Establézcalo en `false` para que Claude Code ignore ese parámetro completamente y cada comando que Claude ejecute debe estar en sandbox o aparecer en [`excludedCommands`](#sandbox-excludedcommands). La pestaña **Overrides** de `/sandbox` muestra ese estado como **Strict sandbox mode**. Use `false` en configuración administrada para políticas que requieren sandboxing estricto.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude puede reintentarlo un comando fuera del sandbox con el parámetro `dangerouslyDisableSandbox` después de que el sandbox lo bloquee
  * `false`: Claude Code ignora ese parámetro, por lo que cada comando que Claude ejecuta está en sandbox o aparece en `excludedCommands`
* **Default**: `true`

Esto aplica el modo sandbox estricto para todos los que cubre la configuración administrada:

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "allowUnsandboxedCommands": false
  }
}
```

Un reintento sin sandbox pasa por el flujo de permiso regular, con una solicitud en modo Manual. Consulte [The unsandboxed retry escape hatch](/docs/es/sandboxing#the-unsandboxed-retry-escape-hatch).

Para ver cuándo los comandos que escribe usted mismo en el [símbolo del sistema de modo shell `!`](/docs/es/interactive-mode#shell-mode-with-prefix) se ejecutan en sandbox, consulte [strict sandbox mode](/docs/es/sandboxing#the-unsandboxed-retry-escape-hatch).

<h3 id="sandbox-filesystem">
  `sandbox.filesystem`
</h3>

Controle qué rutas pueden leer y escribir los comandos en sandbox. De forma predeterminada, pueden escribir en el directorio de trabajo, el directorio temporal de la sesión y los directorios que agregue con `--add-dir`, `/add-dir` o `permissions.additionalDirectories`, y pueden leer el resto del sistema de archivos, incluidos los archivos de credenciales. Amplíe o reduzca eso con las cuatro listas de rutas, o desactive la capa del sistema de archivos con `disabled`. Consulte [Filesystem isolation](/docs/es/sandboxing#filesystem-isolation) para los límites predeterminados.

* **Scope**: [`Any file`](#scopes)
* **Type**: object con matrices `allowWrite`, `denyWrite`, `denyRead` y `allowRead`, más los booleanos `allowManagedReadPathsOnly` y `disabled`
* **Default**: sin establecer, por lo que se aplican los límites de lectura y escritura predeterminados

Esto permite que los comandos en sandbox escriban en un directorio de compilación y su kubeconfig, y oculta su archivo de credenciales de AWS:

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "allowWrite": ["/tmp/build", "~/.kube"],
      "denyRead": ["~/.aws/credentials"]
    }
  }
}
```

Claude Code aplica estas listas en el límite del sandbox del SO, por lo que se aplican a cada subproceso que inicia un comando en sandbox, como `kubectl`, `terraform` o `npm`. Claude Code agrega sus [reglas de permiso](/docs/es/sandboxing#permission-rules) a las mismas listas: reglas `Edit` allow y deny a `allowWrite` y `denyWrite`, reglas `Read` deny a `denyRead` y reglas `WebFetch(domain:...)` allow y deny a las listas de dominio [`network`](#sandbox-network).

A menos que se establezca un bloqueo solo administrado, Claude Code fusiona cada lista en los archivos de configuración que carga la sesión. [`allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly) limita `allowRead` a entradas de configuración administrada, y [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly) hace lo mismo para dominios permitidos.

[Configure sandboxing](/docs/es/sandboxing#configure-sandboxing) cubre fuentes que excluye con `--setting-sources`. Cuando edita una lista durante una sesión, Claude Code [aplica el cambio a la sesión en ejecución](/docs/es/settings#when-edits-take-effect).

<h4 id="sandbox-path-prefixes">
  Prefijos de ruta de sandbox
</h4>

Las rutas en `allowWrite`, `denyWrite`, `denyRead`, `allowRead` y [`credentials.files`](#sandbox-credentials-files) se resuelven por su prefijo:

| Prefijo            | Significado                                                                                                   | Ejemplo                                                                     |
| :----------------- | :------------------------------------------------------------------------------------------------------------ | :-------------------------------------------------------------------------- |
| `/`                | Ruta absoluta desde la raíz del sistema de archivos                                                           | `/tmp/build` permanece `/tmp/build`                                         |
| `~/`               | Relativo al directorio de inicio                                                                              | `~/.kube` se convierte en `$HOME/.kube`                                     |
| `./` o sin prefijo | Relativo a la raíz del proyecto para configuración de proyecto, o a `~/.claude` para configuración de usuario | `./output` en `.claude/settings.json` se resuelve a `<project-root>/output` |

El prefijo `//path` para rutas absolutas también funciona. Si usa una sola barra `/path` esperando resolución relativa al proyecto, cambie a `./path`. Esta sintaxis difiere de las [reglas de permiso Read y Edit](/docs/es/permissions#read-and-edit), que usan `//path` para absoluto y `/path` para relativo al proyecto: las rutas del sistema de archivos de sandbox usan convenciones estándar, por lo que `/tmp/build` es una ruta absoluta.

Claude Code elimina una barra diagonal final de una ruta de directorio, por lo que `~/.aws` y `~/.aws/` coinciden con el mismo directorio. Antes de v2.1.224, Claude Code pasaba la barra diagonal final al sandbox, y Claude aún podía leer o escribir rutas bajo una entrada `denyRead` o `denyWrite` escrita con una.

Claude Code también elimina un `/**` final, por lo que `~/build/**` y `~/build` cubren el mismo directorio. Si un comodín como `*` funciona depende de en qué lista esté la entrada y de la plataforma:

* **`allowWrite` y `denyWrite`**: en macOS, los comodines funcionan. En Linux y WSL2, el sandbox monta rutas concretas, por lo que Claude Code omite una entrada que contiene `*`, `?` o `[` una vez que se elimina el `/**` final, y esa entrada no tiene efecto. Claude Code agrega las rutas de sus reglas de permiso `Edit` a estas listas, por lo que se aplica el mismo límite a ellas, y la pestaña **Config** de `/sandbox` advierte sobre reglas de permiso `Edit` y `Read` que contienen comodines.
* **`denyRead` y `allowRead`**: los comodines funcionan en cada plataforma. En Linux y WSL2, Claude Code expande una entrada de lectura a las rutas concretas que coincide, lo que no hace para las listas de escritura.

<h3 id="sandbox-filesystem-allowwrite">
  `sandbox.filesystem.allowWrite`
</h3>

Agregue rutas donde los comandos en sandbox pueden escribir, más allá del directorio de trabajo, el directorio temporal de la sesión y los directorios que ha agregado con `--add-dir`, `/add-dir` o `permissions.additionalDirectories`. Úselo cuando un subproceso como `kubectl` o una herramienta de compilación necesite escribir fuera del proyecto.

* **Scope**: [`Any file`](#scopes)
* **Type**: array de cadenas de ruta, usando los [prefijos de ruta de sandbox](#sandbox-path-prefixes)
* **Default**: sin establecer, por lo que los comandos en sandbox pueden escribir en el directorio de trabajo, el directorio temporal de la sesión, directorios que ha agregado con `--add-dir` o `/add-dir` y directorios en [`permissions.additionalDirectories`](#permissions-additionaldirectories)

Esto permite que una compilación escriba bajo `/tmp/build` y permite que `kubectl` actualice su kubeconfig:

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "allowWrite": ["/tmp/build", "~/.kube"]
    }
  }
}
```

Claude Code fusiona entradas en todos los ámbitos de configuración que carga la sesión: las rutas de usuario, proyecto, local y administradas se combinan en lugar de reemplazarse entre sí, y Claude Code agrega las rutas de sus reglas de permiso `Edit(...)` allow. Una entrada `allowWrite` no puede levantar una [ruta protegida](/docs/es/sandboxing#protected-paths).

<h3 id="sandbox-filesystem-denywrite">
  `sandbox.filesystem.denyWrite`
</h3>

Bloquee los comandos en sandbox para que no escriban en rutas específicas, incluidas las rutas dentro de un directorio que de otro modo sería escribible.

* **Scope**: [`Any file`](#scopes)
* **Type**: array de cadenas de ruta, usando los [prefijos de ruta de sandbox](#sandbox-path-prefixes)
* **Default**: sin establecer

Esto evita que los comandos en sandbox cambien la configuración del sistema o instalen binarios:

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyWrite": ["/etc", "/usr/local/bin"]
    }
  }
}
```

Claude Code fusiona entradas en todos los ámbitos de configuración que carga la sesión, y agrega las rutas de sus reglas de permiso `Edit(...)` deny.

<h3 id="sandbox-filesystem-denyread">
  `sandbox.filesystem.denyRead`
</h3>

Bloquee los comandos en sandbox para que no lean rutas específicas, como archivos de credenciales que la política de lectura predeterminada de otro modo expondría. Para proteger un archivo de credenciales y mantenerlo utilizable a través del proxy de sandbox, consulte [`sandbox.credentials`](#sandbox-credentials) en su lugar.

* **Scope**: [`Any file`](#scopes)
* **Type**: array de cadenas de ruta, usando los [prefijos de ruta de sandbox](#sandbox-path-prefixes)
* **Default**: sin establecer, por lo que los comandos en sandbox mantienen el [acceso de lectura predeterminado](/docs/es/sandboxing#filesystem-isolation), que incluye archivos de credenciales como `~/.aws/credentials`

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyRead": ["~/.aws/credentials"]
    }
  }
}
```

Claude Code fusiona entradas en todos los ámbitos de configuración que carga la sesión, y agrega las rutas de sus reglas de permiso `Read(...)` deny. Cuando [`filesystem.disabled`](#sandbox-filesystem-disabled) es `true`, Claude Code no aplica estas entradas.

<h3 id="sandbox-filesystem-allowread">
  `sandbox.filesystem.allowRead`
</h3>

Reabra la lectura para rutas específicas dentro de una región que [`denyRead`](#sandbox-filesystem-denyread) bloquea, para construir acceso de lectura solo para el espacio de trabajo. Una entrada `denyRead` exacta o comodín permanece bloqueada dentro de un `allowRead` más amplio, como muestra la [tabla de superposición](/docs/es/sandboxing#configure-sandboxing). Cuando una entrada `denyRead` comodín como `~/**/.env` coincide con un directorio, Claude Code bloquea las lecturas de su contenido también. Antes de v2.1.236 en macOS, Claude Code reabrió las rutas que una entrada `denyRead` comodín coincidía dondequiera que una entrada `allowRead` más amplia las cubriera, y dejaba el contenido de un directorio coincidente legible.

* **Scope**: [`Any file`](#scopes)
* **Type**: array de cadenas de ruta, usando los [prefijos de ruta de sandbox](#sandbox-path-prefixes)
* **Default**: sin establecer

Esto bloquea las lecturas de su directorio de inicio excepto el proyecto mismo:

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyRead": ["~/"],
      "allowRead": ["."]
    }
  }
}
```

Claude Code resuelve una entrada `.` a la raíz del proyecto en configuración de proyecto y a `~/.claude` en configuración de usuario. Claude Code fusiona entradas en todos los archivos de configuración que carga la sesión a menos que [`allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly) esté establecido.

<h3 id="sandbox-filesystem-allowmanagedreadpathsonly">
  `sandbox.filesystem.allowManagedReadPathsOnly`
</h3>

Honre solo las entradas [`allowRead`](#sandbox-filesystem-allowread) que provienen de configuración administrada, para que los desarrolladores no puedan reabrir el acceso de lectura a rutas que su organización bloqueó. Claude Code aún fusiona entradas `denyRead` de todos los ámbitos de configuración que carga la sesión.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code honra solo las entradas `allowRead` de configuración administrada
  * `false`: las entradas `allowRead` se fusionan de todos los ámbitos de configuración que carga la sesión
* **Default**: `false`

Esto bloquea las lecturas del directorio de inicio, reabre `~/work` y evita que los desarrolladores reabran cualquier otra cosa:

```json managed-settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyRead": ["~/"],
      "allowRead": ["~/work"],
      "allowManagedReadPathsOnly": true
    }
  }
}
```

Consulte [Keep developers from widening the policy](/docs/es/sandboxing#keep-developers-from-widening-the-policy).

<h3 id="sandbox-filesystem-disabled">
  `sandbox.filesystem.disabled`
</h3>

Omita el aislamiento del sistema de archivos mientras mantiene el aislamiento de red. Los comandos en sandbox obtienen acceso de lectura y escritura sin restricciones al sistema de archivos del host, y su salida de red permanece confinada a [`network.allowedDomains`](#sandbox-network-alloweddomains). Úselo cuando coloque en sandbox para controlar dónde se conectan los comandos en lugar de lo que escriben. Requiere Claude Code v2.1.216 o posterior.

* **Scope**: [`User or managed`](#scopes). Cuando la configuración administrada configura `sandbox.filesystem` en absoluto, o enumera una entrada `sandbox.credentials.files` con `"mode": "deny"`, solo la configuración administrada puede establecerla.
* **Type**: Boolean
  * `true`: Claude Code omite el aislamiento del sistema de archivos y mantiene el aislamiento de red
  * `false`: el aislamiento del sistema de archivos permanece activado
* **Default**: `false`, por lo que el aislamiento del sistema de archivos permanece activado

Esto deja el sistema de archivos abierto y confina la salida de red a GitHub y npm:

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "filesystem": {
      "disabled": true
    },
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org"]
    }
  }
}
```

Con la capa desactivada, Claude Code no aplica entradas `denyRead` o `credentials.files` `deny`, mientras que las entradas `credentials.envVars` y las entradas `mask` aplicadas siguen funcionando. [`autoAllowBashIfSandboxed`](#sandbox-autoallowbashifsandboxed) aún tiene el valor predeterminado `true`, así que establézcalo en `false` para mantener la solicitud. Consulte [Disable filesystem isolation](/docs/es/sandboxing#disable-filesystem-isolation) para la lista completa de fuentes que pueden establecerla y qué cambia cuando el aislamiento está desactivado. Requiere Claude Code v2.1.216 o posterior.

<h3 id="sandbox-ignoreviolations">
  `sandbox.ignoreViolations`
</h3>

Silenciar los informes de violación de sandbox para rutas que espera que un comando sondee y sea rechazado, como una herramienta que verifica `/etc/hosts` al inicio, para que esos rechazos no aparezcan como violaciones o en lo que Claude ve. El sandbox aún bloquea el acceso; solo se suprime el informe. Las claves son subcadenas para coincidir con el comando, con `*` coincidiendo con cada comando, y los valores son subcadenas de la violación a ignorar para ese comando, como una ruta del sistema de archivos.

* **Scope**: [`Any file`](#scopes)
* **Type**: object que asigna una subcadena de comando a una matriz de subcadenas de violación, generalmente rutas
* **Default**: sin establecer, por lo que cada violación se informa

```json settings.json theme={null}
{
  "sandbox": {
    "ignoreViolations": {
      "*": ["/etc/hosts"]
    }
  }
}
```

<h3 id="sandbox-enableweakernestedsandbox">
  `sandbox.enableWeakerNestedSandbox`
</h3>

Ejecute el sandbox de Linux dentro de un contenedor Docker sin privilegios, donde bubblewrap no puede montar un `/proc` nuevo. En su lugar, el sandbox interno vincula el `/proc` existente del contenedor, que expone información de proceso que un montaje nuevo ocultaría. Esto reduce la seguridad; úselo solo cuando el contenedor externo ya proporciona el aislamiento que necesita.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: el sandbox interno vincula el `/proc` existente del contenedor en lugar de montar uno nuevo
  * `false`: el sandbox monta un `/proc` nuevo, que no funciona en un contenedor Docker sin privilegios
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "enableWeakerNestedSandbox": true
  }
}
```

Solo Linux y WSL2. Consulte [Bubblewrap fails to start inside a container](/docs/es/sandboxing#troubleshooting).

<h3 id="sandbox-enableweakernetworkisolation">
  `sandbox.enableWeakerNetworkIsolation`
</h3>

Permita que los comandos en sandbox en macOS lleguen al servicio de confianza TLS del sistema, `com.apple.trustd.agent`. Las herramientas basadas en Go como `gh`, `gcloud` y `terraform` lo necesitan para verificar certificados TLS cuando usa [`network.httpProxyPort`](#sandbox-network-httpproxyport) con un proxy MITM y una CA personalizada. Esto reduce la seguridad al abrir una posible ruta de exfiltración de datos a través del servicio de confianza.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: los comandos en sandbox en macOS pueden llegar a `com.apple.trustd.agent`
  * `false`: los comandos en sandbox en macOS no pueden llegar al servicio de confianza TLS del sistema
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "enableWeakerNetworkIsolation": true
  }
}
```

Si no usa un proxy MITM, enumere las herramientas que fallan en [`excludedCommands`](#sandbox-excludedcommands) en su lugar; consulte [Go-based CLIs fail TLS verification on macOS](/docs/es/sandboxing#troubleshooting).

<h3 id="sandbox-allowappleevents">
  `sandbox.allowAppleEvents`
</h3>

Permita que los comandos en sandbox en macOS envíen Apple Events, que `open`, `osascript` y herramientas que abren URLs en un navegador necesitan; sin él fallan con error `-600`. Esto elimina el aislamiento de ejecución de código: los comandos en sandbox pueden lanzar otras aplicaciones sin sandbox sin solicitud del usuario, y pueden enviar comandos AppleScript a aplicaciones en ejecución como Terminal, sujeto a la solicitud de consentimiento de automatización por aplicación de macOS (TCC).

* **Scope**: [`User or managed`](#scopes)
* **Type**: Boolean
  * `true`: los comandos en sandbox en macOS pueden enviar Apple Events
  * `false`: los comandos en sandbox en macOS no pueden enviar Apple Events, por lo que `open` y `osascript` fallan con error `-600`
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "allowAppleEvents": true
  }
}
```

Para mantener el aislamiento y aún ejecutar una herramienta de este tipo, agréguela a [`excludedCommands`](#sandbox-excludedcommands) en su lugar. Consulte [Apple Events on macOS](/docs/es/sandboxing#security-limitations).

<h3 id="sandbox-ripgrep">
  `sandbox.ripgrep`
</h3>

Apunte el sandbox a un binario ripgrep propio en lugar del que usa Claude Code, por ejemplo cuando su plataforma necesita un `rg` construido de manera diferente.

* **Scope**: [`User or managed`](#scopes)
* **Type**: object con `command`, la ruta al binario ripgrep, y `args` opcional, una matriz de argumentos para anteponer
* **Default**: sin establecer, por lo que el sandbox usa el mismo binario ripgrep que Claude Code. Ese es el binario incluido a menos que establezca [`USE_BUILTIN_RIPGREP`](/docs/es/env-vars) en `0`

```json settings.json theme={null}
{
  "sandbox": {
    "ripgrep": {
      "command": "/usr/local/bin/rg"
    }
  }
}
```

<h3 id="sandbox-bwrappath">
  `sandbox.bwrapPath`
</h3>

Apunte el sandbox a un binario bubblewrap instalado fuera de `PATH`, como una copia vendida en un host aislado. Claude Code usa la ruta tanto para la verificación de dependencia de inicio como cuando envuelve cada comando en sandbox.

* **Scope**: [`Managed`](#scopes). Claude Code lo lee solo de configuración administrada para que un archivo de usuario, proyecto o local no pueda apuntar el sandbox a un binario diferente.
* **Type**: string, una ruta absoluta; Claude Code descarta una ruta relativa y vuelve a la búsqueda de `PATH`
* **Default**: sin establecer, por lo que Claude Code encuentra `bwrap` en `PATH`

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "bwrapPath": "/opt/admin/bwrap"
  }
}
```

Solo Linux y WSL2.

<h3 id="sandbox-socatpath">
  `sandbox.socatPath`
</h3>

Apunte el proxy de red de sandbox a un binario `socat` instalado fuera de `PATH`.

* **Scope**: [`Managed`](#scopes)
* **Type**: string, una ruta absoluta; Claude Code descarta una ruta relativa y vuelve a la búsqueda de `PATH`
* **Default**: sin establecer, por lo que Claude Code encuentra `socat` en `PATH`

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "socatPath": "/opt/admin/socat"
  }
}
```

Solo Linux y WSL2.

<h3 id="sandbox-credentials">
  `sandbox.credentials`
</h3>

Declare los archivos de credenciales y variables de entorno para [proteger de comandos en sandbox](/docs/es/sandboxing#protect-credentials). Cada entrada nombra un archivo `path` o una variable `name` y un `mode`: `deny` oculta la credencial dentro del sandbox, y `mask` muestra a los comandos en sandbox un marcador de posición mientras el [proxy de sandbox](/docs/es/sandboxing#mask-credentials) sustituye el valor real en solicitudes salientes. Claude Code protege solo las entradas que enumera; no hay una lista de negación de credenciales integrada.

* **Scope**: [`Any file`](#scopes). Claude Code honra entradas `mask`, `allowPlaintextInject`, `awsPairs` y `sigv4` solo de configuración de usuario, configuración administrada y la bandera `--settings`.
* **Type**: object con `files`, `envVars`, `allowPlaintextInject`, `awsPairs` y `sigv4`
* **Default**: sin establecer, por lo que no se protegen credenciales

Esto oculta su archivo de credenciales de AWS y elimina `GITHUB_TOKEN` de comandos en sandbox:

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "files": [{ "path": "~/.aws/credentials", "mode": "deny" }],
      "envVars": [{ "name": "GITHUB_TOKEN", "mode": "deny" }]
    }
  }
}
```

La protección de archivo `deny` es parte de la capa del sistema de archivos, por lo que no se aplica cuando [deshabilita el aislamiento del sistema de archivos](/docs/es/sandboxing#disable-filesystem-isolation); la protección de variable de entorno aún lo hace.

<h4 id="invalid-credential-entries-in-managed-settings">
  Entradas de credenciales inválidas en configuración administrada
</h4>

Cuando una entrada `sandbox.credentials` administrada falla en la validación, Claude Code sigue protegiendo la credencial donde puede:

* Una entrada en `files` o `envVars` que aún tiene un `path` o `name` válido y un `mode` de `mask` o `deny`, como uno cuyo patrón `extract` no tiene grupo de captura, se degrada a `mode: "deny"` con una advertencia, por lo que la credencial permanece bloqueada, no enmascarada, hasta que corrija la entrada. Una entrada `files` degradada fija [`filesystem.disabled`](/docs/es/sandboxing#disable-filesystem-isolation) como una entrada `deny` explícita, y la advertencia señala que su bloqueo de lectura no se aplica si la configuración administrada desactiva el aislamiento del sistema de archivos.
* Una entrada con un `mode` desconocido o un `path` o `name` inválido se elimina.
* Cada caso advierte; ya sea que una entrada se degrade o se elimine, las entradas válidas restantes aún se aplican, y un valor `credentials` completamente inválido se descarta mientras el resto de `sandbox` aún se aplica.

Se aplica en v2.1.191 y posterior; antes de v2.1.221, cada entrada inválida se eliminaba. Para las otras claves administradas con manejo por campo, consulte [Invalid entries in managed settings](/docs/es/managed-settings#invalid-entries-in-managed-settings).

<h3 id="sandbox-credentials-files">
  `sandbox.credentials.files`
</h3>

Proteja archivos o directorios de credenciales de comandos en sandbox. Con `"mode": "deny"`, Claude Code bloquea las lecturas de la ruta dentro del sandbox, el mismo bloqueo de lectura que [`sandbox.filesystem.denyRead`](#sandbox-filesystem-denyread). Con `"mode": "mask"`, los comandos en sandbox en Linux y WSL2 leen una copia centinela del archivo, y el proxy de sandbox sustituye el valor real en solicitudes salientes a `injectHosts` de esa entrada; en macOS el archivo es ilegible dentro del sandbox en su lugar. `"mode": "mask"` requiere Claude Code v2.1.221 o posterior.

* **Scope**: [`Any file`](#scopes). Claude Code descarta entradas `mask` de `.claude/settings.json` de proyecto y `.claude/settings.local.json` local.
* **Type**: array de objetos, cada uno con `path` y un `mode` de `"deny"` o `"mask"`, más los [campos de máscara opcionales para archivos](#mask-fields-for-files)
* **Default**: sin establecer, por lo que no se protegen archivos de credenciales

Esto oculta su archivo de credenciales de AWS y enmascara el archivo de hosts `gh`, sustituyendo el valor real solo en solicitudes a `api.github.com`:

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "files": [
        { "path": "~/.aws/credentials", "mode": "deny" },
        { "path": "~/.config/gh/hosts.yml", "mode": "mask", "injectHosts": ["api.github.com"] }
      ]
    }
  }
}
```

Las rutas usan los mismos [prefijos](#sandbox-path-prefixes) que la configuración `sandbox.filesystem.*`, y Claude Code fusiona las matrices de todos los ámbitos de configuración que carga la sesión. [Protect credentials](/docs/es/sandboxing#protect-credentials) cubre lo que aún se aplica de fuentes que excluye con `--setting-sources`. `mask` entries require Claude Code v2.1.221 or later.

La sustitución `mask` se ejecuta solo a través del proxy de sandbox, así que establezca [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate), o [`allowPlaintextInject`](#sandbox-credentials-allowplaintextinject) para redes de prueba HTTP simple. `mask` se aplica a un único archivo, así que enumere cada archivo de credenciales individualmente. Claude Code acepta pero ignora los campos `mask` en una entrada `deny`. [Mask credential files](/docs/es/sandboxing#mask-credential-files) cubre qué fuentes de configuración se honran y cuándo una entrada vuelve a `deny`.

<span id="sandbox-credentials-files-extract" />

<span id="sandbox-credentials-files-onextractnomatch" />

<span id="sandbox-credentials-files-decode" />

<span id="sandbox-credentials-files-maskclaims" />

<span id="sandbox-credentials-files-maskduplicates" />

<span id="sandbox-credentials-files-injecthosts" />

<h4 id="mask-fields-for-files">
  Campos de máscara para archivos
</h4>

Una entrada `mask` acepta estos campos opcionales. Sin `extract` o `decode`, Claude Code reemplaza todo el contenido del archivo con un centinela. En macOS con aislamiento del sistema de archivos activado, Claude Code aplica una entrada `mask` como `deny` antes de que se ejecute `extract` o `decode`; consulte [Mask credential files](/docs/es/sandboxing#mask-credential-files).

| Campo              | Tipo                                                                                                                      | Lo que hace                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :----------------- | :------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `extract`          | string, una expresión regular con al menos un grupo de captura                                                            | Enmascare solo el texto capturado por el grupo 1 de cada coincidencia, para que el resto del archivo permanezca analizable. Con `decode` también establecido, Claude Code verifica cada captura como un JWT posible en lugar de reemplazarlo directamente. Requiere v2.1.221 o posterior                                                                                                                                                                                                                                                                                                                                        |
| `onExtractNoMatch` | `"warn"`, `"deny"` o `"error"`; predeterminado `"warn"`                                                                   | Qué sucede cuando `extract` o `decode` no encuentra nada para enmascarar. `warn` deja el archivo legible tal como está dentro del sandbox, `deny` lo hace ilegible, y `error` detiene la configuración del sandbox hasta que corrija la configuración. Claude Code trata `deny` como `error` cuando el bloqueo de lectura no se aplicaría, porque [deshabilita el aislamiento del sistema de archivos](/docs/es/sandboxing#disable-filesystem-isolation) o una entrada [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread) reabre la ruta. Requiere v2.1.221 o posterior; el caso `decode` requiere v2.1.224 o posterior |
| `decode`           | la cadena `"jwt"`                                                                                                         | Encuentre JSON Web Tokens (JWTs) en el archivo, con un patrón integrado o con `extract` cuando esté establecido, verifique cada candidato y reemplácelo con un token falso estructuralmente válido, para que el código dentro del sandbox que decodifica el token siga funcionando. Cuando ningún candidato se verifica, `onExtractNoMatch` rige el resultado. Requiere v2.1.224 o posterior                                                                                                                                                                                                                                    |
| `maskClaims`       | array de strings, al menos un nombre de claim; requiere `decode`                                                          | Enmascare solo los claims de carga útil de nivel superior nombrados dentro de cada JWT verificado y reconstruya el token alrededor de la carga útil modificada, para que los otros claims permanezcan legibles. Cuando ningún claim nombrado coincide, `onExtractNoMatch` rige el resultado. Requiere v2.1.224 o posterior                                                                                                                                                                                                                                                                                                      |
| `maskDuplicates`   | Boolean, predeterminado `false`                                                                                           | También reemplace copias verbatim de cada valor enmascarado en otro lugar del archivo, como un secreto pegado en un comentario. Claude Code coincide con subcadenas sin procesar, así que resérvelo para secretos largos y de alta entropía. Se consulta solo cuando `extract` o `decode` está establecido. Requiere v2.1.221 o posterior                                                                                                                                                                                                                                                                                       |
| `injectHosts`      | array de strings, cada uno un host que [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains) también admite | Reduzca los hosts donde el proxy de sandbox sustituye el valor real. Cuando no está establecido, el proxy lo sustituye en solicitudes a cada host en `sandbox.network.allowedDomains`. Requiere v2.1.221 o posterior                                                                                                                                                                                                                                                                                                                                                                                                            |

Esto enmascara solo el valor `oauth_token` en el archivo de hosts `gh`, reemplaza cada otra copia de él en el archivo, hace que el archivo sea ilegible si el patrón no coincide con nada, y sustituye el token real solo en solicitudes a `api.github.com`:

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "files": [
        {
          "path": "~/.config/gh/hosts.yml",
          "mode": "mask",
          "extract": "oauth_token:\\s*(\\S+)",
          "maskDuplicates": true,
          "onExtractNoMatch": "deny",
          "injectHosts": ["api.github.com"]
        }
      ]
    }
  }
}
```

<h3 id="sandbox-credentials-envvars">
  `sandbox.credentials.envVars`
</h3>

Proteja variables de entorno de comandos en sandbox. Con `"mode": "deny"`, Claude Code elimina la variable del entorno de comandos en sandbox. Con `"mode": "mask"`, los comandos en sandbox ven un valor centinela por sesión, y el proxy de sandbox sustituye el valor real en solicitudes salientes a `injectHosts` de esa entrada, para que herramientas como `gh` y `npm` sigan autenticándose sin nunca tener la credencial real. `"mode": "mask"` requiere Claude Code v2.1.199 o posterior.

* **Scope**: [`Any file`](#scopes). Claude Code descarta entradas `mask` de `.claude/settings.json` de proyecto y `.claude/settings.local.json` local.
* **Type**: array de objetos, cada uno con `name` y un `mode` de `"deny"` o `"mask"`, más los [campos de máscara opcionales para variables de entorno](#mask-fields-for-environment-variables)
* **Default**: sin establecer, por lo que no se protegen variables de entorno

Esto elimina `NPM_TOKEN` de comandos en sandbox y enmascara `GITHUB_TOKEN`, sustituyendo el valor real solo en solicitudes a `api.github.com`:

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "envVars": [
        { "name": "NPM_TOKEN", "mode": "deny" },
        { "name": "GITHUB_TOKEN", "mode": "mask", "injectHosts": ["api.github.com"] }
      ]
    }
  }
}
```

El `name` debe comenzar con una letra o guion bajo y contener solo letras, dígitos y guiones bajos. Claude Code fusiona las matrices de todos los ámbitos de configuración que carga la sesión, y aplica `deny` cuando la misma variable aparece con ambos modos. [Protect credentials](/docs/es/sandboxing#protect-credentials) cubre lo que aún se aplica de fuentes que excluye con `--setting-sources`. `mask` entries require Claude Code v2.1.199 or later.

La sustitución `mask` se ejecuta solo a través del proxy de sandbox, así que establezca [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate), o [`allowPlaintextInject`](#sandbox-credentials-allowplaintextinject) para redes de prueba HTTP simple; consulte [Mask environment variables](/docs/es/sandboxing#mask-environment-variables). Claude Code acepta pero ignora los campos `mask` en una entrada `deny`.

<span id="sandbox-credentials-envvars-extract" />

<span id="sandbox-credentials-envvars-onextractnomatch" />

<span id="sandbox-credentials-envvars-decode" />

<span id="sandbox-credentials-envvars-maskclaims" />

<span id="sandbox-credentials-envvars-injecthosts" />

<h4 id="mask-fields-for-environment-variables">
  Campos de máscara para variables de entorno
</h4>

Una entrada `mask` acepta estos campos opcionales. Sin `extract` o `decode`, Claude Code reemplaza todo el valor con un centinela. `extract` y `decode` no se pueden combinar en la misma entrada.

| Campo              | Tipo                                                                                                                      | Lo que hace                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :----------------- | :------------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `extract`          | string, una expresión regular con al menos un grupo de captura                                                            | Enmascare solo el texto capturado por el grupo 1 de cada coincidencia, como la contraseña dentro de una cadena de conexión `DATABASE_URL`, para que el resto del valor permanezca analizable. Requiere v2.1.224 o posterior                                                                                                                                                                                                   |
| `onExtractNoMatch` | `"warn"`, `"deny"` o `"error"`; predeterminado `"warn"`. En una entrada con `decode`, solo se acepta `"warn"`             | Qué sucede cuando `extract` no coincide con nada. `warn` pasa la variable sin enmascarar, `deny` la desestablece dentro del sandbox, y `error` detiene la configuración del sandbox hasta que corrija la configuración. Requiere v2.1.224 o posterior                                                                                                                                                                         |
| `decode`           | la cadena `"jwt"`                                                                                                         | Verifique que todo el valor sea un JWT y reemplácelo con un token falso estructuralmente válido, para que el código dentro del sandbox que decodifica el token siga funcionando; el proxy sustituye todo el token real en la salida. Un valor que no se verifica pasa sin enmascarar con una advertencia. Requiere v2.1.224 o posterior                                                                                       |
| `maskClaims`       | array de strings, al menos un nombre de claim; requiere `decode`                                                          | Enmascare solo los claims de carga útil de nivel superior nombrados dentro del JWT decodificado y reconstruya el token alrededor de la carga útil modificada, para que los otros claims permanezcan legibles. Cuando ningún claim nombrado coincide, la variable pasa sin enmascarar con una advertencia. Requiere v2.1.224 o posterior                                                                                       |
| `injectHosts`      | array de strings, cada uno un host que [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains) también admite | Reduzca los hosts donde el proxy de sandbox sustituye el valor real. Cuando no está establecido, el proxy lo sustituye en solicitudes a cada host en `sandbox.network.allowedDomains`. Escriba un destino IPv6 como la dirección comprimida desnuda, como `"::1"`, no la forma entre corchetes; consulte [IPv6 destinations in `injectHosts`](/docs/es/sandboxing#ipv6-destinations-in-injecthosts). Requiere v2.1.199 o posterior |

Esto enmascara solo la contraseña dentro de `DATABASE_URL`, desestablece la variable si el patrón no coincide con nada, y enmascara un JWT en `SERVICE_JWT` mientras deja cada claim excepto `api_key` legible:

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "envVars": [
        {
          "name": "DATABASE_URL",
          "mode": "mask",
          "extract": "://[^:]+:([^@]+)@",
          "onExtractNoMatch": "deny"
        },
        {
          "name": "SERVICE_JWT",
          "mode": "mask",
          "decode": "jwt",
          "maskClaims": ["api_key"]
        }
      ]
    }
  }
}
```

<h3 id="sandbox-credentials-allowplaintextinject">
  `sandbox.credentials.allowPlaintextInject`
</h3>

Permita la sustitución `mask` en solicitudes HTTP simples así como en HTTPS terminado por TLS. En HTTP simple, la identidad ascendente no se verifica y la credencial viaja en texto plano, así que deje esto desactivado fuera de redes de prueba confiables. Requiere Claude Code v2.1.199 o posterior.

* **Scope**: [`User or managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code permite la sustitución `mask` en solicitudes HTTP simples así como en HTTPS terminado por TLS
  * `false`: Claude Code permite la sustitución `mask` solo en HTTPS terminado por TLS
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "allowPlaintextInject": true
    }
  }
}
```

Requiere Claude Code v2.1.199 o posterior.

<h3 id="sandbox-credentials-awspairs">
  `sandbox.credentials.awsPairs`
</h3>

Agrupe variables de entorno enmascaradas que formen una credencial de AWS para [re-firmar SigV4](/docs/es/sandboxing#re-sign-aws-requests) cuando su credencial vive en variables con nombres no estándar. Claude Code vincula automáticamente el trío convencional `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` y `AWS_SESSION_TOKEN` cuando enmascara sus valores completos, así que necesita esta clave solo para otros nombres. Requiere Claude Code v2.1.224 o posterior.

* **Scope**: [`User or managed`](#scopes)
* **Type**: array de objetos, cada uno con `accessKeyIdVar`, `secretAccessKeyVar` y opcionalmente `sessionTokenVar`, nombrando entradas `sandbox.credentials.envVars`
* **Default**: sin establecer, por lo que solo el trío convencional está emparejado

Esto vincula tres variables con nombres personalizados en una credencial de AWS para re-firmar:

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "awsPairs": [
        {
          "accessKeyIdVar": "MY_KEY_ID",
          "secretAccessKeyVar": "MY_SECRET_KEY",
          "sessionTokenVar": "MY_SESSION_TOKEN"
        }
      ]
    }
  }
}
```

Cada variable nombrada debe ser una entrada `mask` de valor completo en [`sandbox.credentials.envVars`](#sandbox-credentials-envvars), sin `extract` o `decode`, y puede llenar solo una ranura en todos los pares.

<h3 id="sandbox-credentials-sigv4">
  `sandbox.credentials.sigv4`
</h3>

Elija qué hace el proxy de sandbox con formas de solicitud de AWS que [no puede re-firmar](/docs/es/sandboxing#re-sign-aws-requests): `streaming` para cargas de transmisión aws-chunked, `presigned` para URLs prefirmadas y `sigv4a` para firmas asimétricas SigV4A. Esto se aplica solo a solicitudes firmadas con el ID de clave de acceso de marcador de posición de un par enmascarado. Requiere Claude Code v2.1.224 o posterior.

* **Scope**: [`User or managed`](#scopes)
* **Type**: object con `streaming`, `presigned` y `sigv4a`, cada uno de:
  * `"deny"`: el proxy falla la solicitud
  * `"passthrough"`: el proxy reenvía la solicitud firmada con el marcador de posición enmascarado, para que la herramienta reciba el rechazo propio de AWS
* **Default**: sin establecer, por lo que cada forma es `"deny"`

Esto reenvía cargas de transmisión en lugar de fallarlas en el proxy:

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "sigv4": {
        "streaming": "passthrough"
      }
    }
  }
}
```

Con `deny`, el proxy falla la solicitud. Con `passthrough`, el proxy reenvía la solicitud con su firma calculada desde el marcador de posición enmascarado, por lo que AWS la rechaza y la herramienta que llama recibe la respuesta propia de AWS en lugar de un error de proxy.

<h3 id="sandbox-network">
  `sandbox.network`
</h3>

Controle qué hosts, puertos y sockets pueden alcanzar los comandos en sandbox. El sandbox enruta el tráfico saliente a través de un proxy que aplica estas listas; consulte [Network isolation](/docs/es/sandboxing#network-isolation) para saber cómo el proxy decide y cuándo solicita.

* **Scope**: [`Any file`](#scopes). `strictAllowlist`, `allowManagedDomainsOnly` y `tlsTerminate` se leen de menos fuentes, como dicen sus entradas.
* **Type**: object con las subclaves a continuación
* **Default**: sin establecer, por lo que no se pre-permiten dominios y el sandbox solicita cada host nuevo

Esto pre-permite GitHub y npm, bloquea `uploads.github.com` y permite que los comandos se vinculen a localhost:

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org"],
      "deniedDomains": ["uploads.github.com"],
      "allowLocalBinding": true
    }
  }
}
```

Claude Code fusiona las subclaves de matriz en todos los ámbitos de configuración y las deduplica, por lo que un proyecto puede agregar dominios a su lista de usuario. Las reglas de permiso `WebFetch(domain:...)` allow y deny [permission rules](/docs/es/sandboxing#permission-rules) alimentan las mismas listas de permiso y negación.

<h3 id="sandbox-network-allowunixsockets">
  `sandbox.network.allowUnixSockets`
</h3>

Enumere las rutas de socket Unix a las que los comandos en sandbox pueden conectarse en macOS. Claude Code ignora esta lista en Linux y WSL2, donde el filtro seccomp no puede inspeccionar rutas de socket; use [`allowAllUnixSockets`](#sandbox-network-allowallunixsockets) en su lugar.

* **Scope**: [`Any file`](#scopes)
* **Type**: array de strings, cada uno una ruta de socket
* **Default**: sin establecer, por lo que el sandbox de macOS bloquea cada socket Unix

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowUnixSockets": ["~/.ssh/agent-socket"]
    }
  }
}
```

Una ruta de socket puede otorgar acceso amplio: permitir `/var/run/docker.sock`, por ejemplo, permite que un comando en sandbox controle el daemon de Docker. Consulte [Security limitations](/docs/es/sandboxing#security-limitations).

<h3 id="sandbox-network-allowallunixsockets">
  `sandbox.network.allowAllUnixSockets`
</h3>

Permita que los comandos en sandbox se conecten a cada socket Unix. En Linux y WSL2, el [filtro seccomp](/docs/es/sandboxing#set-up-linux-and-wsl2) del sandbox bloquea las llamadas `socket(AF_UNIX, ...)`, por lo que esta es la única forma de permitir sockets Unix allí. Cuando falta el filtro, que `/sandbox` informa en su pestaña Dependencies, el sandbox no bloquea las llamadas de socket Unix. Consulte [Set up Linux and WSL2](/docs/es/sandboxing#set-up-linux-and-wsl2) para saber de dónde viene el filtro.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: los comandos en sandbox pueden conectarse a cada socket Unix
  * `false`: el sandbox bloquea conexiones de socket Unix: en macOS excepto las rutas en `allowUnixSockets`, y en Linux y WSL2 a través del filtro seccomp cuando está presente
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowAllUnixSockets": true
    }
  }
}
```

En WSL2, `true` también reabre el socket de interoperabilidad que lanza binarios de Windows como `cmd.exe` y `powershell.exe`.

<h3 id="sandbox-network-allowlocalbinding">
  `sandbox.network.allowLocalBinding`
</h3>

Permita que los comandos en sandbox se vinculen a puertos localhost en macOS, por ejemplo para iniciar un servidor de desarrollo.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: los comandos en sandbox pueden vincularse a puertos localhost en macOS
  * `false`: los comandos en sandbox en macOS no pueden vincularse a puertos localhost
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowLocalBinding": true
    }
  }
}
```

<h3 id="sandbox-network-allowmachlookup">
  `sandbox.network.allowMachLookup`
</h3>

Enumere nombres de servicio XPC y Mach adicionales que el sandbox de macOS puede buscar. Las herramientas que se comunican sobre XPC, como el simulador de iOS o Playwright, necesitan que sus servicios se enumeren aquí.

* **Scope**: [`Any file`](#scopes)
* **Type**: array de strings, cada uno un nombre de servicio; un único `*` final coincide con un prefijo, y `"*"` solo coincide con cada servicio
* **Default**: sin establecer

Esto permite cada servicio bajo el prefijo `com.apple.coresimulator.`:

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowMachLookup": ["com.apple.coresimulator.*"]
    }
  }
}
```

<h3 id="sandbox-network-alloweddomains">
  `sandbox.network.allowedDomains`
</h3>

Pre-permita dominios para tráfico saliente de comandos en sandbox, para que el sandbox no los solicite. Los comodines como `*.example.com` coinciden con subdominios, y un sufijo `:port` opcional limita una entrada a un puerto; una entrada sin puerto coincide con cada puerto.

* **Scope**: [`Any file`](#scopes). Solo configuración administrada cuando [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly) está establecido.
* **Type**: array de strings, cada uno un dominio, patrón comodín o literal IP, con un sufijo `:port` opcional
* **Default**: sin establecer, por lo que el sandbox solicita la primera vez que un comando alcanza un host nuevo

Esto pre-permite GitHub en cada puerto, cada subdominio npm y un host API en el puerto 443 solo:

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org", "api.example.com:443"]
    }
  }
}
```

Escriba literales IPv6 entre corchetes, con un puerto opcional: `"[::1]"` permite cada puerto y `"[::1]:443"` un puerto. La forma entre corchetes requiere Claude Code v2.1.229 o posterior. Consulte [IPv6 addresses in domain lists](/docs/es/sandboxing#ipv6-addresses-in-domain-lists).

<h3 id="sandbox-network-denieddomains">
  `sandbox.network.deniedDomains`
</h3>

Bloquee dominios para tráfico saliente de comandos en sandbox, usando la misma sintaxis de comodín, puerto e IPv6 que [`allowedDomains`](#sandbox-network-alloweddomains). Un dominio denegado permanece bloqueado incluso cuando una entrada `allowedDomains` también coincide con él.

* **Scope**: [`Any file`](#scopes)
* **Type**: array de strings, cada uno un dominio, patrón comodín o literal IP, con un sufijo `:port` opcional
* **Default**: sin establecer

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "deniedDomains": ["sensitive.cloud.example.com"]
    }
  }
}
```

Claude Code fusiona esta lista de cada fuente de configuración que carga la sesión incluso cuando `allowManagedDomainsOnly` está establecido, por lo que un desarrollador siempre puede restringir la lista de negación. Para literales IPv6, consulte [IPv6 addresses in domain lists](/docs/es/sandboxing#ipv6-addresses-in-domain-lists).

Una entrada escrita con el punto final que marca un nombre de dominio completamente calificado, como `example.com.`, bloquea las mismas conexiones que `example.com`.

<h3 id="sandbox-network-strictallowlist">
  `sandbox.network.strictAllowlist`
</h3>

Niegue a los comandos en sandbox el acceso a hosts fuera de la lista de permiso en lugar de solicitar aprobación. La lista de permiso es [`allowedDomains`](#sandbox-network-alloweddomains) más dominios de reglas `WebFetch(domain:...)` allow, o solo las entradas de configuración administrada cuando [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly) está establecido. Requiere Claude Code v2.1.219 o posterior.

* **Scope**: [`User or managed`](#scopes). Un repositorio no puede activarlo o desactivarlo.
* **Type**: Boolean
  * `true`: Claude Code niega a los comandos en sandbox el acceso a hosts fuera de la lista de permiso
  * `false`: a menos que otro archivo de configuración confiable establezca `true`, Claude Code decide un host fuera de la lista de permiso por modo de permiso en lugar de negarlo directamente: en modo automático ejecuta el clasificador contra los [dominios permitidos por comando](/docs/es/sandboxing#per-command-allowed-domains-in-auto-mode), en modo `dontAsk` niega, en modo `bypassPermissions` y en sesiones de modo plan interactivo de terminal donde el bypass está disponible permite, y de otro modo pregunta
* **Default**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "strictAllowlist": true
    }
  }
}
```

Claude Code aplica esto solo para comandos en sandbox; herramientas en proceso como `WebFetch` aún siguen sus [reglas de permiso](/docs/es/sandboxing#permission-rules). Cuando cualquiera de las fuentes honradas lo establece en `true`, permanece activado. Consulte [Network isolation](/docs/es/sandboxing#network-isolation). Requiere Claude Code v2.1.219 o posterior.

<h3 id="sandbox-network-allowmanageddomainsonly">
  `sandbox.network.allowManagedDomainsOnly`
</h3>

Bloquee la lista de permiso de red a lo que define la configuración administrada. Claude Code entonces honra solo `allowedDomains` y reglas `WebFetch(domain:...)` allow de configuración administrada, ignora dominios de configuración de usuario, proyecto, local y `--settings`, y bloquea automáticamente un dominio no permitido en lugar de solicitar.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code honra solo `allowedDomains` y reglas `WebFetch(domain:...)` allow de configuración administrada y bloquea automáticamente un dominio no permitido en lugar de solicitar
  * `false`: los dominios de configuración de usuario, proyecto, local y `--settings` se fusionan en la lista de permiso
* **Default**: `false`

Esto bloquea la lista de permiso a GitHub y npm e ignora cualquier dominio que agreguen los desarrolladores:

```json managed-settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowManagedDomainsOnly": true,
      "allowedDomains": ["github.com", "*.npmjs.org"]
    }
  }
}
```

Los dominios denegados aún se fusionan de cada fuente que carga la sesión. Consulte [Keep developers from widening the policy](/docs/es/sandboxing#keep-developers-from-widening-the-policy).

<h3 id="sandbox-network-httpproxyport">
  `sandbox.network.httpProxyPort`
</h3>

Apunte el sandbox a su propio proxy HTTP en lugar del que ejecuta Claude Code. Las organizaciones hacen esto para inspeccionar tráfico HTTPS, aplicar sus propias reglas de filtrado o registrar cada solicitud. Cuando no está establecido, Claude Code inicia su propio proxy para tráfico HTTP.

* **Scope**: [`Any file`](#scopes)
* **Type**: number, un puerto TCP local
* **Default**: sin establecer, por lo que Claude Code ejecuta su propio proxy

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "httpProxyPort": 8080
    }
  }
}
```

Establezca también [`socksProxyPort`](#sandbox-network-socksproxyport) si su proxy debe llevar tráfico SOCKS también; con solo uno de los dos establecidos, Claude Code aún ejecuta su propio proxy para el otro protocolo. Consulte [Custom proxy configuration](/docs/es/sandboxing#custom-proxy-configuration).

<h3 id="sandbox-network-socksproxyport">
  `sandbox.network.socksProxyPort`
</h3>

Apunte el sandbox a su propio proxy SOCKS5 en lugar del que ejecuta Claude Code. Cuando no está establecido, Claude Code inicia su propio proxy para tráfico SOCKS.

* **Scope**: [`Any file`](#scopes)
* **Type**: number, un puerto TCP local
* **Default**: sin establecer, por lo que Claude Code ejecuta su propio proxy

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "socksProxyPort": 8081
    }
  }
}
```

Consulte [Custom proxy configuration](/docs/es/sandboxing#custom-proxy-configuration).

<h3 id="sandbox-network-tlsterminate">
  `sandbox.network.tlsTerminate`
</h3>

Haga que el proxy de sandbox termine TLS para que pueda leer el contenido de solicitudes HTTPS. Esto es experimental, y la sustitución de credenciales `mask` [credential substitution](/docs/es/sandboxing#mask-credentials) lo requiere. Establezca `{}` para generar una autoridad de certificación efímera para la sesión, o establezca `caCertPath` y `caKeyPath` para usar la suya propia.

* **Scope**: [`User or managed`](#scopes). Un repositorio no puede activarlo o suministrar una autoridad de certificación.
* **Type**: object con strings `caCertPath` y `caKeyPath` opcionales, cada uno una ruta de archivo
* **Default**: sin establecer, por lo que el proxy no termina ni inspecciona TLS

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "tlsTerminate": {}
    }
  }
}
```

Cuando más de una fuente honrada lo establece, Claude Code usa el valor de la fuente con mayor precedencia: configuración administrada, luego la bandera `--settings`, luego configuración de usuario. Requiere Claude Code v2.1.199 o posterior.

<span id="context-and-memory" />

<h2 id="memory-and-context">
  Memoria y contexto
</h2>

Controle qué carga Claude Code en el contexto, cómo se compacta y dónde mantiene la memoria y los planes. Consulte [Gestionar contexto](/docs/es/context-window) y [Memoria](/docs/es/memory).

<h3 id="autocompactenabled">
  `autoCompactEnabled`
</h3>

Haga que Claude Code [compacte la conversación automáticamente](/docs/es/context-window#when-your-context-fills-up) cuando el contexto se aproxime al límite. Aparece en `/config` como **Auto-compact**, y al activarlo/desactivarlo allí se escribe esta clave en la configuración del usuario.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code compacta la conversación automáticamente cuando el contexto se aproxima al límite
  * `false`: Claude Code no compacta automáticamente
* **Default**: `true`
* **Per-session overrides**: [`DISABLE_AUTO_COMPACT`](/docs/es/env-vars) desactiva la auto-compactación para una sesión; cualquiera de los dos que la desactive, el otro no puede volver a activarla

```json settings.json theme={null}
{
  "autoCompactEnabled": false
}
```

El comando manual `/compact` sigue funcionando mientras la auto-compactación está desactivada.

<h3 id="autocompactwindow">
  `autoCompactWindow`
</h3>

Establezca qué tan llena se llena la ventana de contexto antes de que Claude Code [se compacte automáticamente](/docs/es/context-window#when-your-context-fills-up).

* **Scope**: [`Any file`](#scopes)
* **Type**: número de tokens, de `100000` a `1000000`. Claude Code limita el valor a la ventana de contexto de su modelo; la [descripción general de modelos](https://platform.claude.com/docs/en/about-claude/models/overview) enumera la ventana de cada modelo
* **Default**: sin establecer, por lo que Claude Code elige una ventana optimizada para su modelo
* **Per-session overrides**: [`--autocompact`](/docs/es/cli-reference#cli-flags) tiene prioridad sobre esta clave para una sesión, y [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/es/env-vars) tiene prioridad sobre ambas

```json settings.json theme={null}
{
  "autoCompactWindow": 500000
}
```

Establézcalo con el comando [`/autocompact`](/docs/es/commands#all-commands), que escribe esta clave en la configuración del usuario. [Establecer la ventana de auto-compactación](/docs/es/model-config#set-the-auto-compact-window) cubre cómo interactúan el comando, la bandera, la variable y la configuración.

<h3 id="automemorydirectory">
  `autoMemoryDirectory`
</h3>

Almacene [memoria automática](/docs/es/memory#storage-location) en un directorio de su elección en lugar del predeterminado por proyecto.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, una ruta de directorio absoluta o con prefijo `~/`
* **Default**: sin establecer, por lo que Claude Code usa `~/.claude/projects/<project>/memory/`

```json settings.json theme={null}
{
  "autoMemoryDirectory": "~/my-memory-dir"
}
```

Desde la configuración del proyecto o local, Claude Code respeta esta clave bajo la misma [regla de confianza del espacio de trabajo que los hooks](/docs/es/permissions#what-runs-before-you-trust-a-folder), ya que un repositorio clonado puede proporcionar esos archivos.

<h3 id="automemoryenabled">
  `autoMemoryEnabled`
</h3>

Active o desactive [memoria automática](/docs/es/memory#enable-or-disable-auto-memory). Cuando es `false`, Claude no lee ni escribe en el directorio de memoria automática. También puede activarlo/desactivarlo con `/memory` durante una sesión, que escribe esta clave en la configuración del usuario.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: lo mismo que sin establecer; la memoria automática permanece activada a menos que algo que supere esta clave la desactive para la sesión, como `--bare`, modo seguro o `CLAUDE_CODE_DISABLE_AUTO_MEMORY`
  * `false`: Claude no lee ni escribe en el directorio de memoria automática
* **Default**: `true`
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_AUTO_MEMORY`](/docs/es/env-vars) tiene prioridad sobre esta clave para una sesión, en cualquier dirección

```json settings.json theme={null}
{
  "autoMemoryEnabled": false
}
```

<h3 id="bashoutputmaxchars">
  `bashOutputMaxChars`
</h3>

Establezca cuántos caracteres de la [salida](/docs/es/tools-reference#output-limits) de un comando Bash o PowerShell exitoso recibe Claude en línea. Cuando la salida supera el límite, Claude Code la guarda en un archivo y Claude recibe una vista previa breve más la ruta del archivo. Aumente el límite cuando la salida del comando, como una compilación detallada o un registro completo de suite de pruebas, regularmente supera el predeterminado y desea que Claude lo lea sin abrir el archivo. Requiere Claude Code v2.1.261 o posterior.

* **Scope**: [`Any file`](#scopes)
* **Type**: número de caracteres, un entero positivo. Claude Code limita el valor al rango `4000` a `128000`
* **Default**: sin establecer, por lo que Claude recibe hasta 30.000 caracteres en línea

```json settings.json theme={null}
{
  "bashOutputMaxChars": 100000
}
```

Cuando establece esta clave, Claude Code ignora la variable de entorno [`BASH_MAX_OUTPUT_LENGTH`](/docs/es/env-vars).

<h3 id="claudemd">
  `claudeMd`
</h3>

Inyecte instrucciones de estilo CLAUDE.md como memoria administrada por la organización sin implementar un archivo separado. Claude Code carga el texto como una entrada de memoria administrada antes de los archivos CLAUDE.md del usuario y del proyecto.

* **Scope**: [`Managed`](#scopes)
* **Type**: string, el texto de un archivo CLAUDE.md; escríbalo como lo haría con el archivo, Markdown incluido, con saltos de línea como `\n`
* **Default**: sin establecer

Este ejemplo implementa dos reglas como una breve lista de Markdown:

```json managed-settings.json theme={null}
{
  "claudeMd": "# Engineering rules\n\n- Always run make lint before committing.\n- Never push directly to main."
}
```

Consulte [Implementar CLAUDE.md en toda la organización](/docs/es/memory#deploy-organization-wide-claude-md).

<h3 id="claudemdexcludes">
  `claudeMdExcludes`
</h3>

Omita archivos `CLAUDE.md` específicos cuando Claude Code carga [memoria](/docs/es/memory#exclude-specific-claude-md-files). En un monorepo grande, úselo para omitir archivos CLAUDE.md de otros equipos que no sean relevantes para su trabajo; [Excluir archivos CLAUDE.md irrelevantes](/docs/es/large-codebases#exclude-irrelevant-claude-md-files) en la guía de codebases grandes le muestra cómo hacerlo. Los patrones coinciden con rutas de archivo absolutas.

* **Scope**: [`Any file`](#scopes)
* **Type**: array de strings, cada uno un patrón glob o ruta absoluta
* **Default**: sin establecer, por lo que Claude Code carga cada CLAUDE.md que encuentra

```json settings.json theme={null}
{
  "claudeMdExcludes": ["**/vendor/**/CLAUDE.md"]
}
```

Las exclusiones se aplican solo a los archivos de memoria del usuario, proyecto y local; los archivos CLAUDE.md de política administrada no se pueden excluir.

<span id="environment-variables" />

<h3 id="env">
  `env`
</h3>

Establezca variables de entorno para cada sesión y para los subprocesos que Claude Code inicia desde ella. La mayoría de variables en la [referencia de variables de entorno](/docs/es/env-vars) pueden ir aquí, que es cómo se aplica una a cada sesión o se implementa en su equipo. La configuración del proyecto y local no puede establecer [algunas de ellas](#variables-claude-code-ignores-in-env).

* **Scope**: [`Any file`](#scopes)
* **Type**: objeto que asigna nombres de variables a valores de string
* **Default**: sin establecer

Este ejemplo desactiva la compactación automática y enruta las solicitudes de API a través de un proxy:

```json settings.json theme={null}
{
  "env": {
    "DISABLE_AUTO_COMPACT": "1",
    "ANTHROPIC_BASE_URL": "https://proxy.example.com"
  }
}
```

<h4 id="how-env-values-interact-with-your-shell">
  Cómo interactúan los valores de `env` con su shell
</h4>

* Un valor aquí sobrescribe la misma variable exportada en su shell, y cuando más de un archivo de configuración establece una variable, se aplica la [más alta precedencia](/docs/es/settings#settings-precedence). [Variables que Claude Code ignora en `env`](#variables-claude-code-ignores-in-env) enumera las excepciones para la configuración del proyecto y local.
* Para cancelar una exportación de shell, establezca la variable en `""`. Claude Code trata un valor vacío como sin establecer para la selección de proveedor, y los subprocesos heredan el valor vacío.
* `NO_COLOR` y `FORCE_COLOR` establecidos aquí llegan solo a los subprocesos. Para cambiar los colores de la interfaz propia de Claude Code, establézcalos en su shell antes de lanzar `claude`.
* Los valores aquí son texto sin formato en el archivo de configuración y llegan a cada subproceso que Claude Code inicia. Para un token portador OTLP que rota, use [`otelHeadersHelper`](#otelheadershelper); para credenciales de API, use [`apiKeyHelper`](#apikeyhelper).

<h4 id="when-claude-code-applies-env-values">
  Cuándo Claude Code aplica valores de `env`
</h4>

* Desde la configuración del usuario, `--settings` y configuración administrada: al inicio, y nuevamente en la sesión en ejecución cuando un cambio guardado altera el `env` fusionado.
* Desde la configuración del proyecto y local: después de confiar en el espacio de trabajo, o al inicio en modo `-p`, que nunca muestra el diálogo de confianza, y nuevamente cuando un cambio guardado altera el `env` fusionado.
* Variables que Claude Code clasifica como seguras, como selección de modelo, tiempos de espera y límites, y alternadores de características: al inicio desde cada archivo de configuración, aparte de las [variables que la configuración del proyecto y local no puede establecer](#variables-claude-code-ignores-in-env).
* Después de [mover la sesión con `/cd`](/docs/es/permissions#move-the-session-to-another-directory) en v2.1.246 o posterior: los valores de `env` del proyecto y local del nuevo directorio, además de los del directorio anterior.

<h4 id="variables-claude-code-ignores-in-env">
  Variables que Claude Code ignora en `env`
</h4>

* La configuración del proyecto y local no puede establecer variables que un repositorio verificado no debería controlar; establézcalas en su shell, configuración del usuario o configuración administrada en su lugar. Claude Code descarta cada una y registra una advertencia que puede ver con `claude --debug`. Incluyen:

  * Variables que eligen dónde Claude Code almacena o escribe sus propios archivos: `CLAUDE_CONFIG_DIR`, `CLAUDE_CODE_TMPDIR` y las variables de directorio del sistema operativo como `HOME`, `TMPDIR`, `TMP`, `TEMP` y la familia `XDG_*`.
  * Variables que exportan contenido de sesión: [`OTEL_LOG_RAW_API_BODIES`](/docs/es/env-vars#variables) y el par de rastreo beta detallado `ENABLE_BETA_TRACING_DETAILED` y `BETA_TRACING_ENDPOINT`.
  * Las variables del exportador [OpenTelemetry](/docs/es/monitoring-usage) que activan la telemetría, eligen dónde va, o eligen qué contenido captura:

    * `CLAUDE_CODE_ENABLE_TELEMETRY`, más el par de telemetría mejorada beta `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` y `ENABLE_ENHANCED_TELEMETRY_BETA`
    * Los selectores de exportador `OTEL_LOGS_EXPORTER`, `OTEL_METRICS_EXPORTER` y `OTEL_TRACES_EXPORTER`
    * Las variables de contenido `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_ASSISTANT_RESPONSES`, `OTEL_LOG_TOOL_CONTENT` y `OTEL_LOG_TOOL_DETAILS`
    * Variables `OTEL_EXPORTER_OTLP_*` cuyos nombres terminan en `_ENDPOINT`, `_HEADERS`, `_PROTOCOL`, `_CERTIFICATE`, `_CLIENT_KEY` o `_INSECURE`, en las formas genérica y por señal, como `OTEL_EXPORTER_OTLP_ENDPOINT` y `OTEL_EXPORTER_OTLP_METRICS_HEADERS`
    * `OTEL_EXPORTER_PROMETHEUS_HOST` y `OTEL_EXPORTER_PROMETHEUS_PORT`

    Solo estos valores aún se aplican desde la configuración del proyecto y local, porque apagan algo: `none` para los tres selectores de exportador, y un valor apagado como `0` para `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_TOOL_CONTENT` y `OTEL_LOG_TOOL_DETAILS`. Tal valor anula la misma variable en su configuración del usuario, pero no una que el entorno desde el que inicia Claude Code, un archivo `--settings` o la configuración administrada establece.

    Cuando un archivo de configuración del proyecto o local establece una variable en este grupo, una sesión interactiva local muestra un aviso al inicio. Ejecute `/status` o `claude doctor` para ver cuáles ignoró Claude Code y cuáles apagaron la telemetría; ambos enumeran nombres, nunca valores. Una ejecución no interactiva con `-p` o una sesión del Agent SDK no muestra aviso, así que verifique que su recopilador aún reciba datos después de actualizar. Si no lo hace, establezca las variables en su configuración del usuario, configuración administrada, el entorno del trabajo, o un archivo que pase con `--settings`.

    Ignorar este grupo en la configuración del proyecto y local requiere Claude Code v2.1.282 o posterior.
  * Variables que cambian cómo Claude Code se inicia o se sincroniza, como `CLAUDE_CODE_PROCESS_WRAPPER`, `CLAUDE_CODE_SYNC_SKILLS`, `CLAUDE_CODE_SYNC_PLUGINS`, `CLAUDE_CODE_PLUGIN_CACHE_DIR` y `CLAUDE_CODE_PLUGIN_SEED_DIR`.

  Antes de v2.1.251, la configuración del proyecto y local podía establecer las variables en esta lista que eligen dónde Claude Code escribe sus archivos o que exportan contenido de sesión, excepto `HOME` y `XDG_CONFIG_HOME`.
* Variables de identidad que los entornos de alojamiento de Claude Code poseen, como `CLAUDE_CODE_REMOTE` y `CLAUDE_CODE_ACCOUNT_UUID`, se ignoran de cada archivo.
* [`CLAUDE_CODE_MESSAGING_SOCKET` y `CLAUDE_CODE_MESSAGING_TOKEN`](/docs/es/env-vars#variables), que Claude Code exporta a sí mismo, se ignoran de cada archivo. Ignorar la variable de socket requiere Claude Code v2.1.224 o posterior, e ignorar el token requiere v2.1.228 o posterior.
* [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/es/sessions#name-the-project-directory-yourself), que Claude Code lee solo del entorno de lanzamiento, se ignora de cada archivo; requiere v2.1.234 o posterior.
* [`CLAUDE_CODE_RESTRICTED`](/docs/es/env-vars#variables), que Claude Code lee solo del entorno de lanzamiento, se ignora de cada archivo.

<h3 id="filecheckpointingenabled">
  `fileCheckpointingEnabled`
</h3>

Haga que Claude Code tome una instantánea de los archivos antes de cada edición para que [`/rewind`](/docs/es/checkpointing) pueda restaurarlos. Aparece en `/config` como **Rewind code (checkpoints)**, y al activarlo/desactivarlo allí se escribe esta clave en la configuración del usuario.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code toma una instantánea de los archivos antes de cada edición para que `/rewind` pueda restaurarlos
  * `false`: Claude Code no toma instantáneas de archivos, por lo que `/rewind` no puede restaurarlos
* **Default**: `true`
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_FILE_CHECKPOINTING`](/docs/es/env-vars) desactiva el checkpointing para una sesión; cualquiera de los dos que lo desactive, el otro no puede volver a activarlo

```json settings.json theme={null}
{
  "fileCheckpointingEnabled": false
}
```

En una ejecución `-p` o una sesión del Agent SDK, Claude Code ignora esta clave. El SDK activa el checkpointing con su opción `enableFileCheckpointing`, y una ejecución `-p` simple necesita `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=true`. Consulte [File checkpointing in the Agent SDK](/docs/es/agent-sdk/file-checkpointing).

<h3 id="plansdirectory">
  `plansDirectory`
</h3>

Elija dónde Claude Code almacena los archivos de plan que escribe en [Plan Mode](/docs/es/permission-modes#analyze-before-you-edit-with-plan-mode). Claude Code resuelve la ruta relativa a la raíz del proyecto y mantiene el predeterminado cuando la ruta se resuelve fuera de ella.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, una ruta relativa a la raíz del proyecto
* **Default**: sin establecer, por lo que Claude Code usa `~/.claude/plans`

```json settings.json theme={null}
{
  "plansDirectory": "./plans"
}
```

<h3 id="skilllistingbudgetfraction">
  `skillListingBudgetFraction`
</h3>

Cada turno, Claude ve un [listado de sus skills](/docs/es/skills#skill-descriptions-are-cut-short) con sus descripciones, y Claude Code limita ese listado a una parte de la ventana de contexto. Cuando el listado supera el límite, Claude Code mantiene el nombre de cada skill pero descarta las descripciones de los skills menos utilizados, para que Claude aún pueda invocar esos skills pero sea menos probable que elija uno por su cuenta. Aumente esta clave para mantener más descripciones visibles al costo de más contexto por turno.

* **Scope**: [`Any file`](#scopes)
* **Type**: número, una fracción mayor que `0` y como máximo `1`
* **Default**: `0.01`, que reserva el 1% de la ventana de contexto

```json settings.json theme={null}
{
  "skillListingBudgetFraction": 0.02
}
```

Para ver cuánto contexto usa el listado y qué skills contribuyen más, ejecute `/doctor`.

<h3 id="skilllistingmaxdescchars">
  `skillListingMaxDescChars`
</h3>

Cada turno, Claude ve un [listado de sus skills](/docs/es/skills#skill-descriptions-are-cut-short) que muestra el texto `description` y `when_to_use` de cada skill. Esta clave limita cuántos caracteres de ese texto muestra Claude Code por skill; el texto más largo se corta en el límite.

* **Scope**: [`Any file`](#scopes)
* **Type**: número de caracteres, un entero positivo
* **Default**: `1536`

```json settings.json theme={null}
{
  "skillListingMaxDescChars": 2048
}
```

Aumente para mantener descripciones largas intactas al costo de más contexto por turno; disminuya para ajustar más skills bajo [`skillListingBudgetFraction`](#skilllistingbudgetfraction).

<h3 id="taskoutputmaxchars">
  `taskOutputMaxChars`
</h3>

<Warning>
  Eliminado en v2.1.277, junto con la herramienta `TaskOutput` que dimensionaba. Establecer esta clave no tiene efecto en las versiones actuales. Claude lee un [archivo de salida](/docs/es/tools-reference#background-commands) de tarea en segundo plano con `Read` en su lugar.
</Warning>

Hasta v2.1.276, establecía esta clave al número de caracteres de la [salida de una tarea en segundo plano](/docs/es/tools-reference#background-commands) que Claude recibía en línea cuando leía la tarea con la herramienta `TaskOutput`.

<h2 id="interface-and-terminal">
  Interfaz y terminal
</h2>

Cambia cómo se ve y se comporta Claude Code en tu terminal: tema, modo de editor, línea de estado, spinner, notificaciones dentro de la sesión y accesibilidad. Consulta [Configuración de terminal](/docs/es/terminal-config).

<h3 id="askuserquestiontimeout">
  `askUserQuestionTimeout`
</h3>

Permite que un diálogo [`AskUserQuestion`](/docs/es/tools-reference) sin respuesta continúe automáticamente después de un período de tiempo inactivo, enviando cualquier opción que ya hayas seleccionado. Establécelo cuando te alejes y quieras que Claude continúe sin ti. Con el valor predeterminado, las preguntas esperan hasta que las respondas. Requiere Claude Code v2.1.200 o posterior.

* **Scope**: [`User or managed`](#scopes)
* **Type**: string, uno de `"60s"`, `"5m"`, `"10m"`, o `"never"`
* **Default**: `"never"`
* **Per-session overrides**: [`CLAUDE_AFK_TIMEOUT_MS`](/docs/es/env-vars) tiene precedencia sobre esta clave para una sesión

```json settings.json theme={null}
{
  "askUserQuestionTimeout": "5m"
}
```

Aparece en `/config` como **Question auto-continue timeout**, que escribe esta clave en la configuración del usuario; Claude Code oculta la fila mientras la configuración administrada o la bandera `--settings` establezcan la clave. Requiere Claude Code v2.1.200 o posterior.

<h3 id="autocontinueatusagelimit">
  `autoContinueAtUsageLimit`
</h3>

Después de que un límite de uso de claude.ai detenga tu sesión, espera en la sesión abierta y continúa la tarea automáticamente después del reinicio. Consulta [Desactiva la continuación automática](/docs/es/interactive-mode#turn-automatic-continue-off). Requiere Claude Code v2.1.234 o posterior.

* **Scope**: [`User or managed`](#scopes). Se lee desde la configuración del usuario, `--settings` y la configuración administrada solamente. Cuando ninguno de esos establece la clave, un archivo de configuración de proyecto o local que la establece desactiva la función en lugar de ser ignorado.
* **Type**: Boolean
  * `true`: después de que un límite de uso de claude.ai detenga tu sesión, Claude Code espera en la sesión abierta y continúa la tarea automáticamente después del reinicio
  * `false`: Claude Code no inicia la espera por su cuenta. Aún puedes [iniciar una espera tú mismo](/docs/es/interactive-mode#start-a-wait-yourself) desde el menú de opciones de límite de uso
* **Default**: `true`

```json settings.json theme={null}
{
  "autoContinueAtUsageLimit": false
}
```

Aparece en `/config` como **Continue automatically at usage limit**, que escribe esta clave en la configuración del usuario; Claude Code oculta la fila mientras la configuración administrada o la bandera `--settings` establezcan la clave.

<h3 id="autoscrollenabled">
  `autoScrollEnabled`
</h3>

Sigue la nueva salida hasta el final de la conversación en [renderizado de pantalla completa](/docs/es/fullscreen). Desactívalo para permanecer donde desplazaste mientras Claude sigue trabajando; los avisos de permiso aún se desplazan a la vista.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: la conversación sigue la nueva salida hasta el final
  * `false`: permaneces donde desplazaste mientras Claude sigue trabajando; los avisos de permiso aún aparecen debajo de la transcripción
* **Default**: `true`

```json settings.json theme={null}
{
  "autoScrollEnabled": false
}
```

Aparece en `/config` como **Auto-scroll** cuando el renderizado de pantalla completa está activado, que escribe esta clave en la configuración del usuario.

<h3 id="axscreenreader">
  `axScreenReader`
</h3>

Renderiza salida compatible con lectores de pantalla: texto plano sin bordes decorativos ni animaciones. El modo lector de pantalla utiliza el renderizador clásico, por lo que la configuración `tui` no tiene efecto mientras está activo; las [sesiones en segundo plano](/docs/es/agent-view) adjuntas aún se renderizan en pantalla completa.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code renderiza texto plano sin bordes decorativos ni animaciones, utilizando el renderizador clásico
  * `false`: Claude Code se renderiza normalmente
* **Default**: unset, por lo que el modo lector de pantalla está desactivado
* **Per-session overrides**: [`--ax-screen-reader`](/docs/es/cli-reference#cli-flags) tiene precedencia sobre [`CLAUDE_AX_SCREEN_READER`](/docs/es/env-vars), y ambos tienen precedencia sobre esta clave para una sesión

```json settings.json theme={null}
{
  "axScreenReader": true
}
```

<h3 id="basheditdiffenabled">
  `bashEditDiffEnabled`
</h3>

Elige si Claude Code registra los archivos que un comando Bash cambia en un repositorio Git. Cuando los registra, ves su diff en la terminal después del comando, y tus [hooks PostToolUse Bash](/docs/es/hooks#bash) reciben la lista de archivos cambiados.

Un archivo listado no siempre es uno que el comando cambió. Un cambio que otro programa u otra llamada Bash hizo mientras el comando se ejecutaba también puede aparecer allí.

Establece la clave a `true` para registrarlos en cada modo de permiso. Requiere Claude Code v2.1.269 o posterior.

* **Scope**: [`User or managed`](#scopes). Un `true` cuenta solo desde tu configuración de usuario, JSON pasado con `--settings`, o [configuración administrada](/docs/es/managed-settings), por lo que un `true` en el `.claude/settings.json` o `.claude/settings.local.json` de un repositorio no puede activar el registro. Un `false` en cualquier archivo de repositorio aún lo desactiva a menos que un archivo de [precedencia más alta](/docs/es/settings#settings-precedence) establezca `true`.
* **Type**: Boolean
* **Default**: unset, por lo que Claude Code registra cambios en modo auto y modo `bypassPermissions` cuando dirige a Claude a editar archivos a través de Bash
* **Per-session overrides**: [`CLAUDE_CODE_BASH_EDIT_DIFF`](/docs/es/env-vars) tiene precedencia sobre esta clave para una sesión

```json settings.json theme={null}
{
  "bashEditDiffEnabled": true
}
```

<h3 id="companyannouncements">
  `companyAnnouncements`
</h3>

Muestra los anuncios de tu organización a los usuarios al iniciar. Cuando enumeras más de uno, Claude Code elige uno al azar para cada sesión; en el primer lanzamiento de una persona muestra la primera entrada.

* **Scope**: [`Any file`](#scopes)
* **Type**: array de strings
* **Default**: unset, por lo que no se muestra ningún anuncio

```json settings.json theme={null}
{
  "companyAnnouncements": [
    "Welcome to Acme Corp! Review our code guidelines at docs.example.com"
  ]
}
```

<h3 id="defaultshell">
  `defaultShell`
</h3>

Elige si Bash o PowerShell ejecutan los comandos de shell que escribes con el prefijo [`!`](/docs/es/interactive-mode#shell-mode-with-prefix) en el cuadro de entrada, los que Claude Code ejecuta directamente y agrega a la sesión.

`"powershell"` funciona solo mientras la [herramienta PowerShell](/docs/es/tools-reference#powershell-tool) está activada. La herramienta está activada de forma predeterminada en Windows sin Git Bash, y en Windows con Git Bash para cuentas de claude.ai y Console. En sesiones de Amazon Bedrock, Google Cloud's Agent Platform y Microsoft Foundry, y en macOS, Linux y WSL, establece `CLAUDE_CODE_USE_POWERSHELL_TOOL=1` para activar la herramienta. Establece esa variable a `0` para desactivar la herramienta.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, uno de:
  * `"bash"`: Claude Code ejecuta tus comandos `!` en Bash
  * `"powershell"`: Claude Code ejecuta tus comandos `!` en PowerShell
* **Default**: `"bash"`, o `"powershell"` en Windows cuando Bash no está disponible

```json settings.json theme={null}
{
  "defaultShell": "powershell"
}
```

Si el shell que nombras no está disponible, Claude Code usa el otro: `"powershell"` vuelve a Bash cuando la herramienta PowerShell está desactivada, y `"bash"` vuelve a PowerShell cuando Bash no está instalado.

<h3 id="dialogexpiry">
  `dialogExpiry`
</h3>

Establece el plazo para diálogos que Claude Code [reenvía a un cliente remoto](/docs/es/remote-control#limitations), como un host de Remote Control o SDK, y para el diálogo de aprobación de un [mensaje de sesión cruzada retenido](/docs/es/cross-session-messaging#control-inbound-messages). En Claude Code v2.1.236 o posterior, el mismo plazo limita el aviso de consentimiento de créditos de uso de Fable a mitad de sesión]\(/es/model-config#fable-and-usage-credits) en una sesión que puede no tener a nadie en la terminal. Cuando no llega respuesta antes del plazo, Claude Code cancela el diálogo y continúa con su valor predeterminado sin acción. Requiere Claude Code v2.1.224 o posterior.

* **Scope**: [`User or managed`](#scopes)
* **Type**: string, uno de `"60s"`, `"5m"`, `"10m"`, o `"never"`, que desactiva el plazo
* **Default**: `"5m"`
* **Per-session overrides**: [`CLAUDE_CODE_USER_DIALOG_TIMEOUT_MS`](/docs/es/env-vars) tiene precedencia sobre esta clave para una sesión

```json settings.json theme={null}
{
  "dialogExpiry": "10m"
}
```

Los avisos de permiso y las preguntas [`AskUserQuestion`](/docs/es/tools-reference#askuserquestion-tool-behavior) utilizan sus propios flujos y no se rigen por este plazo. Aparece en `/config` como **Dialog expiry**, que escribe esta clave en la configuración del usuario; la fila requiere Claude Code v2.1.232 o posterior, y Claude Code la oculta mientras la configuración administrada o la bandera `--settings` establezcan la clave.

<h3 id="editormode">
  `editorMode`
</h3>

Elige el modo de atajos de teclado para el aviso de entrada.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, uno de:
  * `"normal"`: atajos de teclado estándar en la entrada del aviso
  * `"vim"`: edición de estilo vim con modos NORMAL, INSERT y VISUAL
* **Default**: `"normal"`

```json settings.json theme={null}
{
  "editorMode": "vim"
}
```

Aparece en `/config` como **Editor mode**, que escribe esta clave en la configuración del usuario.

<h3 id="emojicompletionenabled">
  `emojiCompletionEnabled`
</h3>

Muestra sugerencias de emoji cuando escribes `:` más un código corto en la entrada del aviso, y reemplaza un código corto completado como `:heart:` con su emoji. Establécelo a `false` para desactivar ambos.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code muestra sugerencias de emoji después de `:` y reemplaza un código corto completado con su emoji
  * `false`: Claude Code ni sugiere emoji ni reemplaza códigos cortos
* **Default**: `true`

```json settings.json theme={null}
{
  "emojiCompletionEnabled": false
}
```

Consulta [Códigos cortos de emoji](/docs/es/interactive-mode#emoji-shortcodes). Requiere Claude Code v2.1.217 o posterior.

<span id="file-suggestion-settings" />

<h3 id="filesuggestion">
  `fileSuggestion`
</h3>

Ejecuta tu propio comando para proporcionar autocompletado de ruta de archivo `@` en lugar de la sugerencia de archivo integrada. La sugerencia integrada utiliza recorrido rápido del sistema de archivos; un monorepo grande puede funcionar mejor con indexación específica del proyecto, como un índice de archivo precompilado.

* **Scope**: [`Any file`](#scopes). Bajo las [puertas de línea de estado y sugerencia de archivo](#status-line-and-file-suggestion-gates), Claude Code desactiva el comando o ejecuta solo un valor administrado, y omite el tuyo sin advertencia.
* **Type**: objeto con `type`, siempre `"command"`, y `command`, el comando de shell a ejecutar
* **Default**: unset, por lo que Claude Code usa la sugerencia de archivo integrada

```json settings.json theme={null}
{
  "fileSuggestion": {
    "type": "command",
    "command": "~/.claude/file-suggestion.sh"
  }
}
```

Después de guardar esto, escribe `@` seguido de parte de una ruta en el aviso: las sugerencias provienen de la salida de tu comando.

<h4 id="command-input-and-output">
  Entrada y salida del comando
</h4>

Claude Code ejecuta el comando con las mismas variables de entorno que los [hooks](/docs/es/hooks), incluyendo `CLAUDE_PROJECT_DIR`, y deja de esperar después de cinco segundos. El comando recibe JSON en stdin con un campo `query` que contiene lo que has escrito hasta ahora:

```json theme={null}
{"query": "src/comp"}
```

Imprime rutas de archivo separadas por saltos de línea en stdout. Claude Code muestra como máximo 15:

```text theme={null}
src/components/Button.tsx
src/components/Modal.tsx
src/components/Form.tsx
```

El siguiente script lee la consulta y la entrega a un índice de archivo de repositorio:

```bash theme={null}
#!/bin/bash
query=$(cat | jq -r '.query')
# Replace your-repo-file-index with your own file search command
your-repo-file-index --query "$query" | head -20
```

<span id="footer-link-badges" />

<h3 id="footerlinksregexes">
  `footerLinksRegexes`
</h3>

Renderiza insignias clickeables adicionales en el pie de página debajo del cuadro de entrada cuando una regex coincide con la salida de turno: resultados de herramientas, incluyendo contenidos de archivo y páginas obtenidas, y respuestas propias de Claude. Úsalo para convertir IDs impresos por CLI de proyecto, como herramientas de revisión y rastreadores de problemas, en enlaces de sesión.

* **Scope**: [`User or managed`](#scopes)
* **Type**: array de objetos, cada uno con `type` establecido a `"regex"`, una regex `pattern`, una plantilla `url`, y una `label` opcional; los marcadores de posición `{name}` en `url` y `label` se rellenan desde grupos de captura nombrados en `pattern`
* **Default**: unset, por lo que no se renderizan insignias

Este ejemplo coincide con claves de problema como `PROJ-1234` y construye cada enlace a partir de la clave capturada:

```json settings.json theme={null}
{
  "footerLinksRegexes": [
    {
      "type": "regex",
      "pattern": "\\b(?<key>PROJ-\\d+)\\b",
      "url": "https://issues.example.com/browse/{key}",
      "label": "{key}"
    }
  ]
}
```

Con esto configurado, cuando `PROJ-1234` aparece en un resultado de herramienta o en la respuesta de Claude, una insignia `PROJ-1234` aparece en el pie de página vinculando a `https://issues.example.com/browse/PROJ-1234`.

<h4 id="badge-constraints">
  Restricciones de insignia
</h4>

El URL, la etiqueta y el recuento de insignias de cada entrada están limitados de la siguiente manera:

| Restricción           | Comportamiento                                                                                                                                                                                                                     |
| :-------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Origen de URL         | Los valores capturados se codifican en URL y el URL construido debe compartir el origen literal de la plantilla. Una captura puede rellenar un segmento de ruta o valor de consulta pero no puede cambiar a dónde apunta el enlace |
| Longitud de URL       | Los URLs construidos más largos que 2048 caracteres se descartan                                                                                                                                                                   |
| Esquema de URL        | Debe ser `https`, `http`, o un esquema de enlace profundo de editor o espacio de trabajo reconocido: `vscode`, `vscode-insiders`, `cursor`, `windsurf`, `zed`, `jetbrains`, `idea`, `slack`, `linear`, `notion`, `figma`           |
| Etiqueta              | Por defecto es el texto coincidente y se trunca a 28 columnas de visualización                                                                                                                                                     |
| Recuento de insignias | Como máximo 5 insignias se renderizan. La más antigua es desplazada por coincidencias más nuevas y `/clear` las elimina                                                                                                            |

Cuando un turno se completa, Claude Code coincide con la regex `pattern` de cada entrada contra la salida de turno en el hilo principal, por lo que una regex lenta bloquea la interfaz hasta que termina. Los cuantificadores anidados como `(a+)+$` pueden tomar exponencialmente tiempo contra ciertas entradas y congelar la sesión, así que mantén cada `pattern` lineal y evita anidar `+` o `*`.

Las insignias de pie de página se renderizan junto a una [línea de estado personalizada](/docs/es/statusline) cuando una está configurada; ninguna reemplaza a la otra. Usa una línea de estado para una fila impulsada por script que calcula su propio contenido a partir de datos de sesión, e insignias de pie de página para convertir IDs de la conversación en enlaces sin un script.

<h3 id="keybindingflavor">
  `keybindingFlavor`
</h3>

<Warning>
  Deprecado desde v2.1.261 y no tiene efecto. Las teclas de edición de palabras del aviso siempre [siguen convenciones readline](/docs/es/interactive-mode#make-ctrl-w-delete-back-to-whitespace), como en Bash. Claude Code aún acepta `keybindingFlavor`, por lo que un archivo de configuración que lo establece sigue siendo válido.
</Warning>

En v2.1.238 a v2.1.260, establecerlo a `"readline"` hizo que `Ctrl+W` eliminara hacia el espacio en blanco anterior en lugar de solo la palabra anterior.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, `"classic"` o `"readline"`
* **Default**: unset

<h3 id="prefersreducedmotion">
  `prefersReducedMotion`
</h3>

Reduce o desactiva animaciones de interfaz como el spinner, shimmer y efectos de destello. Aparece en `/config` como **Reduce motion**.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code reduce o desactiva animaciones de interfaz como el spinner, shimmer y efectos de destello
  * `false`: lo mismo que unset; Claude Code muestra sus animaciones
* **Default**: `false`

```json settings.json theme={null}
{
  "prefersReducedMotion": true
}
```

<h3 id="promptsuggestionenabled">
  `promptSuggestionEnabled`
</h3>

Muestra u oculta [sugerencias de aviso](/docs/es/interactive-mode#prompt-suggestions), las predicciones atenuadas que aparecen en tu entrada de aviso. Establécelo a `false`, o desactiva **Prompt suggestions** en `/config`, para ocultarlas.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: ves sugerencias de aviso en tu entrada de aviso
  * `false`: Claude Code oculta sugerencias de aviso
* **Default**: `true`
* **Per-session overrides**: [`CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION`](/docs/es/env-vars) tiene precedencia sobre esta clave para una sesión

```json settings.json theme={null}
{
  "promptSuggestionEnabled": false
}
```

Las sugerencias de aviso necesitan una cuenta de claude.ai o Console con telemetría activada. En Amazon Bedrock, Google Cloud's Agent Platform y Microsoft Foundry, o con telemetría desactivada, como por [`DISABLE_TELEMETRY`](/docs/es/env-vars), esta clave no tiene efecto y solo `CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION=1` las activa.

<h3 id="respectgitignore">
  `respectGitignore`
</h3>

Controla si el selector de archivo `@` deja fuera archivos que coinciden con patrones `.gitignore`. Aparece en `/config` como **Respect .gitignore in file picker**.

* **Scope**: [`Any file`](#scopes). Cuando ningún archivo de configuración lo establece, Claude Code vuelve a `respectGitignore` en `~/.claude.json`, que el toggle `/config` escribe.
* **Type**: Boolean
  * `true`: el selector de archivo `@` deja fuera archivos que coinciden con patrones `.gitignore`
  * `false`: el selector de archivo `@` incluye archivos que coinciden con patrones `.gitignore`
* **Default**: `true`

```json settings.json theme={null}
{
  "respectGitignore": false
}
```

<h3 id="respondtobashcommands">
  `respondToBashCommands`
</h3>

Elige si Claude responde después de que ejecutes un comando de shell con el prefijo [`!`](/docs/es/interactive-mode#shell-mode-with-prefix) en el cuadro de entrada. De forma predeterminada, Claude Code agrega la salida del comando a la conversación y Claude responde a ella. Establece esta clave a `false` para agregar la salida al contexto sin una respuesta, para que puedas ejecutar varios comandos y preguntar sobre ellos juntos.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code agrega la salida del comando a la conversación y Claude responde a ella
  * `false`: Claude Code agrega la salida al contexto sin una respuesta
* **Default**: `true`

```json settings.json theme={null}
{
  "respondToBashCommands": false
}
```

Consulta [Modo de shell con prefijo `!`](/docs/es/interactive-mode#shell-mode-with-prefix).

<h3 id="showclearcontextonplanaccept">
  `showClearContextOnPlanAccept`
</h3>

Cuando Claude termina un plan en [modo plan](/docs/es/permission-modes#review-and-approve-a-plan), muestra un menú de aprobación. La planificación puede usar mucho contexto, por lo que esta clave agrega una primera opción a ese menú, **Yes, clear context and …**, que aprueba el plan, borra el contexto de conversación e inicia la implementación solo desde el plan. El resto de la etiqueta nombra el modo de permiso en el que continúa la sesión, y muestra cuánto de tu contexto usó la planificación.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: el menú de aprobación del plan obtiene una primera opción, **Yes, clear context and …**, que aprueba el plan y borra el contexto de conversación
  * `false`: el menú de aprobación del plan no muestra opción de borrar contexto
* **Default**: `false`

```json settings.json theme={null}
{
  "showClearContextOnPlanAccept": true
}
```

<h3 id="showturnduration">
  `showTurnDuration`
</h3>

Muestra u oculta el mensaje de duración de turno después de cada respuesta, como "Cooked for 1m 6s · done 6:05 PM". El reloj después de "done" muestra cuándo terminó el turno; [`timeFormat`](#timeformat) y [`timeZone`](#timezone) controlan su formato y zona. Aparece en `/config` como **Show turn duration**.

* **Scope**: [`Any file`](#scopes). Un valor en `~/.claude.json` de una versión anterior se aplica cuando ningún archivo de configuración lo establece.
* **Type**: Boolean
  * `true`: ves el mensaje de duración de turno después de cada respuesta
  * `false`: Claude Code oculta el mensaje de duración de turno
* **Default**: `true`

```json settings.json theme={null}
{
  "showTurnDuration": false
}
```

<h3 id="spellcheck">
  `spellcheck`
</h3>

Subraya palabras mal escritas en la entrada del aviso mientras escribes, usando un corrector ortográfico que instales. Claude Code verifica solo el texto en el cuadro de entrada. [Verifica la ortografía mientras escribes](/docs/es/interactive-mode#check-spelling-as-you-type) cubre la instalación de aspell, hunspell o ispell y qué cubre el verificador. Requiere Claude Code v2.1.235 o posterior.

* **Scope**: [`User or managed`](#scopes). El bloque del nivel más alto que lo establece se aplica como un todo.
* **Type**: objeto con `enabled` (Boolean), `checker` (`"aspell"`, `"hunspell"`, `"ispell"`, o `"auto"`), `language` (string, pasado al verificador como su nombre de diccionario), y `color` (string, un nombre de color de terminal, `#rrggbb`, `rgb(r,g,b)`, `ansi256(n)`, o `ansi:<name>`)
* **Default**: unset, por lo que la verificación ortográfica está desactivada; `checker` por defecto es `"auto"`, el primero de los tres encontrados en `PATH`; `language` por defecto es el propio diccionario del verificador; `color` por defecto es el color de error del tema

```json settings.json theme={null}
{
  "spellcheck": { "enabled": true, "language": "en_GB" }
}
```

<h3 id="spinnertipsenabled">
  `spinnerTipsEnabled`
</h3>

Mientras Claude trabaja, la línea del spinner rota a través de consejos cortos sobre características de Claude Code, como "Use Plan Mode to prepare for a complex request before making changes. Press Shift+Tab twice to enable." Establece esta clave a `false` para ocultarlos. Aparece en `/config` como **Show tips**.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: ves consejos en el spinner mientras Claude está trabajando
  * `false`: Claude Code oculta consejos del spinner
* **Default**: `true`

```json settings.json theme={null}
{
  "spinnerTipsEnabled": false
}
```

<h3 id="spinnertipsoverride">
  `spinnerTipsOverride`
</h3>

Agrega tus propios consejos a los [consejos del spinner](#spinnertipsenabled) que Claude Code muestra mientras Claude trabaja, o reemplaza los consejos integrados con los tuyos. Claude Code pone tus consejos en la misma rotación que los integrados: elige el consejo que ha estado sin mostrarse más tiempo, omite consejos aún en su enfriamiento, y rompe empates por prioridad.

Si estableces [`spinnerTipsEnabled`](#spinnertipsenabled) a `false`, Claude Code oculta todos los consejos, incluyendo los tuyos.

* **Scope**: [`Any file`](#scopes). Claude Code honra objetos de consejo, `tipsFile`, `label`, y `excludeDefault` desde la configuración del usuario, la bandera `--settings`, y la configuración administrada; desde la configuración de proyecto y local lee solo consejos de string plano.
* **Type**: objeto con campos `tips`, `tipsFile`, `label`, y `excludeDefault`, cada uno opcional
* **Default**: unset, por lo que Claude Code muestra solo los consejos integrados

Objetos de consejo, `tipsFile`, `label`, y la regla de la línea Scope que la configuración de proyecto y local contribuyen solo strings planos requieren Claude Code v2.1.247 o posterior. En versiones anteriores, `excludeDefault` de un archivo de proyecto o local también se aplica.

Cada entrada `tips` es un string plano u objeto con estos campos:

| Campo              | Requerido | Descripción                                                                                                                                                                                                                                               |
| :----------------- | :-------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `id`               | Sí        | Hasta 64 letras, dígitos, `.`, `_`, o `-`. Claude Code clave el historial de visualización del consejo en él, por lo que el enfriamiento del consejo sobrevive a la reordenación de la lista. De dos entradas con el mismo id, Claude Code usa la primera |
| `text`             | Sí        | El consejo, una línea de hasta 500 caracteres. Claude Code elimina escapes ANSI y caracteres de control y colapsa espacios en blanco                                                                                                                      |
| `cooldownSessions` | No        | Sesiones que Claude Code espera antes de mostrar el consejo nuevamente, `0` a `1000`, por defecto `0`                                                                                                                                                     |
| `priority`         | No        | Orden entre consejos que han estado sin mostrarse igualmente tiempo, más alto primero, `-10` a `10`, por defecto `0`                                                                                                                                      |

Claude Code lee un string plano como un consejo con esos valores predeterminados y un id basado en posición, por lo que su historial de visualización se reinicia cuando reordenas la lista. Dale a un consejo un `id` para mantener su historial a través de ediciones.

Claude Code lee como máximo 200 consejos en `tips` y `tipsFile`, y descarta una entrada inválida con una advertencia de depuración en lugar de rechazar el archivo de configuración.

Usa los campos restantes para nombrar un archivo de consejos, establecer el prefijo y ocultar los consejos integrados:

* `tipsFile`: una ruta absoluta o `~/` a un archivo JSON local que contiene un array de las mismas entradas, u objeto con un array `tips`, hasta 256 KB. Claude Code lee el archivo una vez por proceso, por lo que carga tus ediciones en el próximo inicio. No puedes establecerlo a través de [configuración administrada por servidor](/docs/es/server-managed-settings); despliega `tips` en línea allí, o despliega la ruta en un `managed-settings.json` en disco.
* `label`: el prefijo que Claude Code muestra antes de consejos desde la configuración del usuario, `--settings`, y la configuración administrada, hasta 40 caracteres. El valor predeterminado es `Tip`, el mismo prefijo que los consejos integrados, y los consejos de la configuración de proyecto y local siempre lo usan.
* `excludeDefault`: establécelo a `true` para ocultar los consejos integrados y mostrar solo los tuyos. Cuando Claude Code no puede cargar ninguno de tus consejos, por ejemplo porque `tipsFile` no existe o cada entrada es inválida, mantiene la rotación integrada en lugar de un spinner vacío.

Cuando más de un archivo de configuración establece la clave, Claude Code muestra consejos de todos ellos y toma `tipsFile`, `label`, y `excludeDefault` de cualquiera de la configuración administrada, la bandera `--settings`, y la configuración del usuario que sea el más alto precedente que establezca cada uno.

Este ejemplo, en tu configuración del usuario, agrega un consejo de string plano y un consejo de objeto a la rotación bajo el prefijo `Acme tip`:

```json settings.json theme={null}
{
  "spinnerTipsOverride": {
    "label": "Acme tip",
    "tips": [
      "Run /review before opening a PR",
      {
        "id": "gateway-errors",
        "text": "Seeing 5xx errors? Check the gateway status page first",
        "cooldownSessions": 5,
        "priority": 2
      }
    ]
  }
}
```

Cada campo en el ejemplo cambia una cosa sobre cómo Claude Code muestra los consejos:

* `label`: Claude Code muestra ambos consejos como `Acme tip: ...` en lugar de `Tip: ...`.
* El string plano: Claude Code le da los valores predeterminados, por lo que puede aparecer nuevamente en la sesión siguiente.
* `id`: Claude Code clave el historial de visualización del segundo consejo en `gateway-errors`, por lo que su enfriamiento aún se aplica después de agregar o reordenar consejos.
* `cooldownSessions`: después de que Claude Code muestre el consejo `gateway-errors`, no muestra ese consejo nuevamente hasta cinco sesiones después.
* `priority`: cuando el consejo `gateway-errors` y otro consejo han estado sin mostrarse durante el mismo número de sesiones, por ejemplo cuando ninguno ha sido mostrado aún, Claude Code muestra `gateway-errors` primero. El string plano tiene la prioridad predeterminada, `0`.

Mientras Claude trabaja, Claude Code muestra tus consejos en el spinner con tu prefijo, como `Acme tip: Run /review before opening a PR`.

<h3 id="spinnerverbs">
  `spinnerVerbs`
</h3>

Mientras un turno está en progreso, el spinner muestra un verbo rotativo como "Accomplishing", "Architecting", o "Baking". Usa esta clave para agregar tus propios verbos a esa rotación o reemplazar la lista integrada con la tuya.

* **Scope**: [`Any file`](#scopes)
* **Type**: objeto con un array `verbs` de strings y `mode`, uno de:
  * `"append"`: Claude Code agrega tus verbos al conjunto integrado
  * `"replace"`: Claude Code muestra solo tus verbos
* **Default**: unset, por lo que Claude Code usa los verbos integrados

Este ejemplo agrega dos verbos al conjunto integrado:

```json settings.json theme={null}
{
  "spinnerVerbs": {
    "mode": "append",
    "verbs": ["Pondering", "Crafting"]
  }
}
```

En modo `"replace"` con un array `verbs` vacío, Claude Code mantiene los verbos integrados.

<h3 id="statusline">
  `statusLine`
</h3>

Ejecuta tu propio comando para renderizar una [línea de estado](/docs/es/statusline) debajo del aviso con contexto como el modelo, costo o rama de git. Los campos opcionales ajustan espaciado, agregan re-ejecuciones periódicas y ocultan el indicador de modo vim integrado cuando tu script renderiza `vim.mode` a sí mismo.

* **Scope**: [`Any file`](#scopes). Cuando [`allowManagedHooksOnly`](#allowmanagedhooksonly) está activado, o [`disableAllHooks`](#disableallhooks) se establece fuera de la configuración administrada, solo se ejecuta el valor de configuración administrada.
* **Type**: objeto con `type` establecido a `"command"` y una string `command`, más `padding` opcional como número de caracteres, `refreshInterval` como número de segundos, mínimo `1`, e `hideVimModeIndicator` como Boolean
* **Default**: unset, por lo que no hay línea de estado

Este ejemplo imprime el nombre del modelo y el uso de contexto, y agrega dos caracteres de espaciado horizontal:

```json settings.json theme={null}
{
  "statusLine": {
    "type": "command",
    "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'",
    "padding": 2
  }
}
```

El ejemplo necesita [`jq`](https://jqlang.org/) instalado y se ejecuta en un shell. Para equivalentes de PowerShell y Git Bash, consulta [Configuración de Windows](/docs/es/statusline#windows-configuration); para la configuración completa, consulta [Configura manualmente una línea de estado](/docs/es/statusline#manually-configure-a-status-line).

<h3 id="subagentstatusline">
  `subagentStatusLine`
</h3>

Cuando Claude ejecuta [subagentes](/docs/es/sub-agents), Claude Code los enumera en una pantalla de tarea debajo del aviso, una fila por subagente mostrando `name · description · token count`. Esta clave te permite ejecutar tu propio comando para reescribir esas filas, por ejemplo para mostrar el uso de contexto de cada subagente como porcentaje. En cada actualización, Claude Code envía las filas visibles como un objeto JSON en stdin, con un array `tasks` llevando `id`, `name`, `status`, `model`, `tokenCount` de cada subagente, y más, y reemplaza la fila para cada `id` que escribas de vuelta como una línea `{"id", "content"}`. Las filas que no escribas de vuelta mantienen el renderizado predeterminado.

* **Scope**: [`Any file`](#scopes). Cuando [`allowManagedHooksOnly`](#allowmanagedhooksonly) está activado, o [`disableAllHooks`](#disableallhooks) se establece fuera de la configuración administrada, solo se ejecuta el valor de configuración administrada.
* **Type**: objeto con `type` establecido a `"command"` y una string `command`
* **Default**: unset, por lo que Claude Code renderiza las filas predeterminadas

```json settings.json theme={null}
{
  "subagentStatusLine": {
    "type": "command",
    "command": "jq -c '.tasks[] | {id, content: \"\\(.name): \\(.tokenCount) tokens\"}'"
  }
}
```

Consulta [Líneas de estado de subagente](/docs/es/statusline#subagent-status-lines).

<h3 id="syntaxhighlightingdisabled">
  `syntaxHighlightingDisabled`
</h3>

Claude Code colorea código por lenguaje en los diffs, bloques de código y vistas previas de archivo que muestra en la terminal, con su resaltador integrado; no hay plugin o servidor de lenguaje involucrado. Establece esta clave a `true` para mostrarlos como texto plano en su lugar, por ejemplo si los colores chocan con tu tema de terminal o ralentizan un lector de pantalla.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code desactiva el resaltado de sintaxis en diffs, bloques de código y vistas previas de archivo
  * `false`: Claude Code resalta la sintaxis
* **Default**: `false`

```json settings.json theme={null}
{
  "syntaxHighlightingDisabled": true
}
```

<h3 id="terminalprogressbarenabled">
  `terminalProgressBarEnabled`
</h3>

Algunos terminales pueden mostrar un indicador de progreso en la pestaña o en la barra de tareas del programa que se ejecuta en ellos. Mientras Claude está trabajando, Claude Code reporta un estado en progreso al terminal, para que puedas ver desde otra pestaña o ventana si la sesión aún está ocupada. El indicador permanece visible después de que el turno termina mientras [subagentes en segundo plano](/docs/es/sub-agents#run-subagents-in-foreground-or-background) o [flujos de trabajo dinámicos](/docs/es/workflows) aún se están ejecutando, y se borra una vez que la sesión está inactiva.

Claude Code lo reporta solo en terminales que soportan el indicador: ConEmu, Ghostty 1.2.0 o posterior, e iTerm2 3.6.6 o posterior. Establece esta clave a `false` para detener a Claude Code de reportarlo. Aparece en `/config` como **Terminal progress bar**.

* **Scope**: [`Any file`](#scopes). Un valor en `~/.claude.json` de una versión anterior se aplica cuando ningún archivo de configuración lo establece.
* **Type**: Boolean
  * `true`: ves la barra de progreso del terminal en terminales que la soportan
  * `false`: Claude Code oculta la barra de progreso del terminal
* **Default**: `true`

```json settings.json theme={null}
{
  "terminalProgressBarEnabled": false
}
```

<h3 id="terminaltitlefromrename">
  `terminalTitleFromRename`
</h3>

Claude Code establece el título de la pestaña de tu terminal. De forma predeterminada usa un título que genera a partir de la conversación, y una vez que le das a la sesión un [nombre](/docs/es/sessions#name-your-sessions) con `/rename` o `--name`, la pestaña muestra ese nombre en su lugar. Establece esta clave a `false` para mantener el título generado en la pestaña incluso después de nombrar la sesión. El nombre en sí aún se aplica, por lo que `/resume <name>` y el selector de sesión lo encuentran.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: el título de la pestaña del terminal muestra el nombre de sesión que estableciste
  * `false`: la pestaña mantiene el título que Claude Code genera a partir de tu conversación
* **Default**: `true`

```json settings.json theme={null}
{
  "terminalTitleFromRename": false
}
```

Para detener a Claude Code de actualizar el título del terminal en absoluto, establece [`CLAUDE_CODE_DISABLE_TERMINAL_TITLE`](/docs/es/env-vars) a `1` en su lugar.

<h3 id="theme">
  `theme`
</h3>

Elige el tema de color para la interfaz. Aparece en `/config` como **Theme**.

* **Scope**: [`Any file`](#scopes). Un valor en `~/.claude.json` de una versión anterior se aplica cuando ningún archivo de configuración lo establece.
* **Type**: string, uno de:
  * `"auto"`: coincide con el fondo claro u oscuro de tu terminal
  * `"dark"`: el tema oscuro
  * `"light"`: el tema claro
  * `"dark-daltonized"`: el tema oscuro con colores amigables para daltónicos
  * `"light-daltonized"`: el tema claro con colores amigables para daltónicos
  * `"dark-ansi"`: el tema oscuro usando solo la paleta de color ANSI de tu terminal
  * `"light-ansi"`: el tema claro usando solo la paleta de color ANSI de tu terminal
  * `"custom:<slug>"` o `"custom:<plugin-name>:<slug>"`: un tema personalizado de `~/.claude/themes/` o un plugin
* **Default**: `"dark"`

```json settings.json theme={null}
{
  "theme": "light-daltonized"
}
```

Consulta [Crea un tema personalizado](/docs/es/terminal-config#create-a-custom-theme).

<h3 id="timeformat">
  `timeFormat`
</h3>

Elige cómo Claude Code escribe los tiempos que muestra en la interfaz, como el `done 6:05 PM` al final de cada mensaje de duración de turno y las marcas de tiempo en el [visor de transcripción](/docs/es/interactive-mode#transcript-viewer). Para elegir un preajuste, ejecuta `/config` y establece **Time format**. Requiere Claude Code v2.1.257 o posterior.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, uno de:
  * `"auto"`: lo mismo que unset; cada tiempo mantiene su formato integrado, que sigue tu configuración regional en el mensaje de duración de turno
  * `"12-hour"`: un reloj de 12 horas
  * `"24-hour"`: un reloj de 24 horas
  * `"24-hour-utc"`: un reloj de 24 horas en UTC con `Z` después de los minutos, como `18:05Z`; Claude Code ignora [`timeZone`](#timezone) para este preajuste
  * Un patrón strftime como `"%H:%M"`: Claude Code escribe cada tiempo con el patrón. Cualquier valor que contenga un `%` es un patrón, y cualquier otro valor fuera de los preajustes cuenta como `"auto"`
* **Default**: `"auto"`

```json settings.json theme={null}
{
  "timeFormat": "24-hour"
}
```

`/config` ofrece solo los preajustes, por lo que para usar un patrón strftime, agrega la clave a un archivo de configuración. Este ejemplo muestra cada tiempo como un reloj de 24 horas de dos dígitos:

```json settings.json theme={null}
{
  "timeFormat": "%H:%M"
}
```

El mensaje de duración de turno y el visor de transcripción entonces muestran tiempos como `18:05`. En el visor de transcripción, el patrón es la marca de tiempo completa, así que agrega directivas de fecha cuando quieras la fecha allí. Este ejemplo pone la fecha frente al reloj:

```json settings.json theme={null}
{
  "timeFormat": "%Y-%m-%d %H:%M"
}
```

Las mismas superficies entonces muestran tiempos como `2026-09-01 18:05`.

<h3 id="timezone">
  `timeZone`
</h3>

Muestra los tiempos en la interfaz en una zona horaria diferente a la de tu sistema. Establécelo a un [nombre de zona horaria IANA](https://www.iana.org/time-zones), como `"UTC"` o `"Europe/Dublin"`. Los tiempos que [`timeFormat`](#timeformat) controla entonces se muestran en esta zona. Si `timeFormat` es `"24-hour-utc"`, los tiempos permanecen en UTC y Claude Code ignora esta clave. `/config` no tiene fila para esta clave, así que establécela en un archivo de configuración. Requiere Claude Code v2.1.257 o posterior.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, un nombre de zona horaria IANA. Cuando Claude Code no reconoce el nombre, usa tu zona horaria del sistema
* **Default**: unset, por lo que los tiempos se muestran en tu zona horaria del sistema

```json settings.json theme={null}
{
  "timeZone": "Europe/Dublin"
}
```

<h3 id="tui">
  `tui`
</h3>

Elige el renderizador de interfaz de usuario de terminal. Usa `"fullscreen"` para el renderizador [alt-screen](/docs/es/fullscreen) sin parpadeos con scrollback virtualizado, o `"default"` para el renderizador clásico de pantalla principal. Ejecutar `/tui fullscreen` o `/tui default` escribe esta clave para ti.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, uno de:
  * `"default"`: el renderizador clásico de pantalla principal
  * `"fullscreen"`: el renderizador alt-screen sin parpadeos con scrollback virtualizado
* **Default**: unset, por lo que Claude Code [elige el renderizador para ti](/docs/es/fullscreen#fullscreen-by-default)
* **Per-session overrides**: [`CLAUDE_CODE_NO_FLICKER`](/docs/es/env-vars) y [`CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN`](/docs/es/env-vars) tienen precedencia sobre esta clave para una sesión: `CLAUDE_CODE_NO_FLICKER=1` activa pantalla completa, y `CLAUDE_CODE_NO_FLICKER=0` o `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1` la desactiva; cuando ambos se establecen, Claude Code la desactiva

```json settings.json theme={null}
{
  "tui": "fullscreen"
}
```

Bajo tmux `-CC` o sobre SSH a Windows, Claude Code mantiene el renderizador clásico a menos que establezas `CLAUDE_CODE_NO_FLICKER=1`. Las sesiones en segundo plano abiertas desde [vista de agente](/docs/es/agent-view) siempre usan el renderizador de pantalla completa independientemente de esta configuración.

<h3 id="verbose">
  `verbose`
</h3>

De forma predeterminada, la transcripción colapsa cada llamada de herramienta a un resumen corto, como el comando que Claude ejecutó y un recuento de líneas de su salida, y presionas `Ctrl+O` para cambiar toda la transcripción a la vista expandida cuando quieres los detalles. Establece esta clave a `true` para mostrar la entrada y salida completa de cada llamada de herramienta en línea mientras sucede, lo cual es útil cuando estás depurando un hook, un servidor MCP o un comando de shell largo. Aparece en `/config` como **Verbose output**.

* **Scope**: [`Any file`](#scopes). Un valor en `~/.claude.json` de una versión anterior se aplica cuando ningún archivo de configuración lo establece.
* **Type**: Boolean
  * `true`: ves salida de herramienta completa
  * `false`: ves resúmenes truncados de salida de herramienta
* **Default**: `false`
* **Per-session overrides**: [`--verbose`](/docs/es/cli-reference#cli-flags) tiene precedencia sobre esta clave para una sesión

```json settings.json theme={null}
{
  "verbose": true
}
```

Un valor [`viewMode`](#viewmode) o una selección pegajosa `/focus` anula esta clave cada sesión.

<h3 id="viewmode">
  `viewMode`
</h3>

Establece la vista de transcripción en la que Claude Code comienza: `"default"`, `"verbose"`, o `"focus"`. Cuando se establece, anula tanto la selección pegajosa `/focus` como la configuración [`verbose`](#verbose).

* **Scope**: [`Any file`](#scopes)
* **Type**: string, uno de:
  * `"default"`: la transcripción normal con salida de herramienta truncada
  * `"verbose"`: la transcripción con salida de herramienta completa
  * `"focus"`: solo tu último aviso, un resumen de una línea de llamadas de herramientas con estadísticas de diff de edición, y la respuesta final. La vista de enfoque necesita el [renderizador de pantalla completa](#tui)
* **Default**: unset, por lo que la configuración `verbose` y tu última opción `/focus` se aplican
* **Per-session overrides**: [`--verbose`](/docs/es/cli-reference#cli-flags) tiene precedencia sobre esta clave para una sesión

```json settings.json theme={null}
{
  "viewMode": "focus"
}
```

<h3 id="viminsertmoderemaps">
  `vimInsertModeRemaps`
</h3>

Mapea secuencias de modo INSERT de dos teclas a Escape en [modo editor vim](/docs/es/interactive-mode#vim-editor-mode). Cada clave es exactamente dos caracteres imprimibles escritos en secuencia, y `"<Esc>"` es el único objetivo soportado; Claude Code ignora otras entradas. Requiere Claude Code v2.1.208 o posterior.

* **Scope**: [`User or managed`](#scopes). Un repositorio no puede remapear tus pulsaciones de tecla.
* **Type**: objeto mapeando una secuencia de dos caracteres a `"<Esc>"`
* **Default**: unset

```json settings.json theme={null}
{
  "vimInsertModeRemaps": {
    "jj": "<Esc>"
  }
}
```

No tiene efecto a menos que `editorMode` sea `"vim"`. Consulta [Remapea secuencias de tecla de modo INSERT](/docs/es/interactive-mode#remap-insert-mode-key-sequences). Requiere Claude Code v2.1.208 o posterior.

<h3 id="voice">
  `voice`
</h3>

Activa [dictado de voz](/docs/es/voice-dictation) y elige cómo se comporta la tecla de dictado. Claude Code escribe este objeto para ti cuando ejecutas `/voice`.

* **Scope**: [`Any file`](#scopes)
* **Type**: objeto con `enabled` como Boolean, `autoSubmit` como Boolean que se aplica solo en modo de espera, y `mode`, uno de:
  * `"hold"`: mantienes presionada la tecla de dictado mientras hablas y la sueltas para detener
  * `"tap"`: tocas la tecla una vez para comenzar a grabar y nuevamente para enviar
* **Default**: unset, por lo que el dictado está desactivado; cuando `enabled` es `true` y `mode` es unset, Claude Code usa `"hold"`

Este ejemplo activa el dictado y hace que la tecla toque una vez para comenzar a grabar y nuevamente para enviar:

```json settings.json theme={null}
{
  "voice": {
    "enabled": true,
    "mode": "tap"
  }
}
```

`autoSubmit` envía el aviso cuando sueltas la tecla en modo de espera. El dictado de voz requiere una cuenta de claude.ai.

<h3 id="voiceenabled">
  `voiceEnabled`
</h3>

<Warning>
  Deprecado desde v2.1.92, cuando el objeto [`voice`](#voice) lo reemplazó. Claude Code aún lo lee para que los archivos de configuración más antiguos sigan funcionando, pero las nuevas configuraciones deben establecer `voice.enabled`.
</Warning>

Activa el dictado de voz con la forma Boolean única que precede al objeto `voice`. Cuando ambos se establecen, `voice.enabled` se aplica.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: el dictado de voz está activado cuando estás conectado con una cuenta de claude.ai y la política de tu organización permite voz, a menos que `voice.enabled` se establezca
  * `false`: el dictado de voz está desactivado, a menos que `voice.enabled` se establezca
* **Default**: unset

```json settings.json theme={null}
{
  "voiceEnabled": true
}
```

<h3 id="wheelscrollaccelerationenabled">
  `wheelScrollAccelerationEnabled`
</h3>

Acelera la velocidad de desplazamiento de rueda del ratón durante desplazamientos rápidos en [renderizado de pantalla completa](/docs/es/fullscreen#mouse-wheel-scrolling). Establécelo a `false` para una velocidad de desplazamiento constante por muesca de rueda.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code acelera la velocidad de desplazamiento de rueda del ratón durante desplazamientos rápidos
  * `false`: Claude Code se desplaza a una velocidad constante por muesca de rueda
* **Default**: `true`

```json settings.json theme={null}
{
  "wheelScrollAccelerationEnabled": false
}
```

<h2 id="git-and-attribution">
  Git y atribución
</h2>

Controle la atribución que Claude Code añade a los commits y solicitudes de extracción y cómo funciona con git.

<span id="attribution-settings" />

<h3 id="attribution">
  `attribution`
</h3>

Personalice la atribución que Claude Code añade a los commits de git y solicitudes de extracción. Los commits obtienen un [tráiler de git](https://git-scm.com/docs/git-interpret-trailers) como `Co-Authored-By` de forma predeterminada; las descripciones de solicitudes de extracción obtienen texto sin formato. Establezca cada parte por separado con las subclaves que se indican a continuación.

* **Scope**: [`Any file`](#scopes)
* **Type**: objeto con cadenas `commit` y `pr` y un Boolean `sessionUrl`, o `false` para ocultar toda la atribución. El valor `false` requiere Claude Code v2.1.281 o posterior; las versiones anteriores lo rechazan y [omiten todo el archivo de configuración del usuario, proyecto o local](/docs/es/settings#fix-a-broken-settings-file) que lo contiene
* **Default**: sin establecer, por lo que Claude Code utiliza la atribución estándar que se muestra en cada subclave

Para ocultar toda la atribución, establezca `attribution` en `false`. En un archivo de configuración que las versiones anteriores también leen, establezca [`commit`](#attribution-commit) y [`pr`](#attribution-pr) en cadenas vacías y [`sessionUrl`](#attribution-sessionurl) en `false` en su lugar.

Este ejemplo reemplaza la atribución del commit, elimina la atribución de la solicitud de extracción y descarta el enlace de sesión:

```json settings.json theme={null}
{
  "attribution": {
    "commit": "Generated with AI\n\nCo-Authored-By: AI <ai@example.com>",
    "pr": "",
    "sessionUrl": false
  }
}
```

Una vez que establezca `commit` o `pr`, Claude Code ignora la configuración deprecada `includeCoAuthoredBy` y utiliza su texto predeterminado para cualquiera de los dos que haya dejado sin establecer.

Claude Code le indica a Claude que sus propias instrucciones sobre atribución, como una regla de CLAUDE.md o [memory](/docs/es/memory), tienen prioridad sobre estas líneas de commit y PR, a menos que la línea esté establecida en [managed settings](/docs/es/managed-settings).

<h3 id="includecoauthoredby">
  `includeCoAuthoredBy`
</h3>

<Warning>
  Deprecado desde v2.0.62, cuando [`attribution`](#attribution) lo reemplazó. Claude Code aún lo lee, pero las nuevas configuraciones deben establecer `attribution`.
</Warning>

Utilice [`attribution`](#attribution) en su lugar, que reemplaza esta clave y le permite cambiar u ocultar el tráiler del commit, el texto de la solicitud de extracción y el enlace de sesión por separado. Claude Code aún respeta `includeCoAuthoredBy: false` de archivos de configuración anteriores a `attribution`, pero lo ignora una vez que establezca `attribution.commit` o `attribution.pr`.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: lo mismo que sin establecer; Claude Code añade el tráiler del commit y el texto de atribución de la solicitud de extracción
  * `false`: Claude Code omite tanto el tráiler del commit como el texto de atribución de la solicitud de extracción, a menos que `attribution` establezca `commit` o `pr`, en cuyo caso se aplican las reglas de [`attribution`](#attribution)
* **Default**: `true`

```json settings.json theme={null}
{
  "includeCoAuthoredBy": false
}
```

Para ocultar toda la atribución, consulte [`attribution`](#attribution).

<h3 id="includegitinstructions">
  `includeGitInstructions`
</h3>

Claude Code proporciona a Claude dos elementos relacionados con git: sus instrucciones integradas sobre cómo escribir commits y solicitudes de extracción, en la descripción de la herramienta Bash, y una instantánea del estado de git de su repositorio. La instantánea contiene la rama actual, la rama principal, la salida de `git status` y los commits recientes. Claude Code la lee cuando comienza una conversación.

Establezca esta clave en `false` para dejar ambas fuera, por ejemplo cuando utiliza sus propias skills de flujo de trabajo de git.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code incluye sus instrucciones integradas de flujo de trabajo de commit y solicitud de extracción y la instantánea del estado de git. Las sesiones en la nube nunca incluyen la instantánea
  * `false`: Claude Code deja ambas fuera
* **Default**: `true`
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_GIT_INSTRUCTIONS`](/docs/es/env-vars) tiene prioridad sobre esta clave para una sesión

```json settings.json theme={null}
{
  "includeGitInstructions": false
}
```

<h3 id="prurltemplate">
  `prUrlTemplate`
</h3>

Apunte los enlaces de PR que Claude Code renderiza, en el distintivo de pie de página y en los resúmenes de resultados de herramientas, a una herramienta de revisión de código interna en lugar de `github.com`. Claude Code sustituye `{host}`, `{owner}`, `{repo}`, `{number}` y `{url}` de la URL de PR. Los enlaces de [solicitud de fusión de GitLab](/docs/es/interactive-mode#gitlab-merge-requests) en ambas superficies mantienen su URL de GitLab.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, una plantilla de URL utilizando cualquiera de los cinco marcadores de posición
* **Default**: sin establecer

```json settings.json theme={null}
{
  "prUrlTemplate": "https://reviews.example.com/{owner}/{repo}/pull/{number}"
}
```

Claude Code aplica la plantilla solo a los enlaces que renderiza a sí mismo; un número de PR que Claude escribe en un mensaje, como `#123`, permanece como Claude lo escribió. Una URL que no tiene la forma `/pull/<number>` se deja sin cambios.

<h3 id="attribution-commit">
  `attribution.commit`
</h3>

Establezca el texto de atribución que Claude Code añade a los commits de git, incluidos los tráilers. Establézcalo en una cadena vacía para ocultar la atribución del commit.

* **Scope**: [`Any file`](#scopes)
* **Type**: string
* **Default**: sin establecer, por lo que Claude Code añade `Co-Authored-By: <name> <noreply@anthropic.com>`. El nombre es el modelo activo de la sesión, como `Claude Sonnet 5`.
  * Cuando Claude Code reconoce el modelo como un modelo Claude pero no puede confirmar su versión exacta, escribe `Claude` solo.
  * Cuando no puede hacer coincidir el ID del modelo con ningún modelo Claude, como un modelo de terceros servido a través de un [`ANTHROPIC_BASE_URL`](/docs/es/env-vars) personalizado, escribe `Claude Code`.

Este ejemplo reemplaza el tráiler predeterminado con una línea personalizada y un tráiler `Co-Authored-By` personalizado:

```json settings.json theme={null}
{
  "attribution": {
    "commit": "Generated with AI\n\nCo-Authored-By: AI <ai@example.com>"
  }
}
```

<h3 id="attribution-pr">
  `attribution.pr`
</h3>

Establezca el texto de atribución que Claude Code añade a las descripciones de solicitudes de extracción. Establézcalo en una cadena vacía para ocultar la atribución de la solicitud de extracción.

* **Scope**: [`Any file`](#scopes)
* **Type**: string
* **Default**: sin establecer, por lo que Claude Code añade `🤖 Generated with [Claude Code](https://claude.com/claude-code)`

```json settings.json theme={null}
{
  "attribution": {
    "pr": ""
  }
}
```

<h3 id="attribution-sessionurl">
  `attribution.sessionUrl`
</h3>

Elija si Claude Code añade el enlace de sesión de claude.ai cuando realiza un commit o abre una solicitud de extracción desde una sesión [en la nube](/docs/es/claude-code-on-the-web) o [Remote Control](/docs/es/remote-control). Claude Code añade el enlace como un tráiler `Claude-Session` en los commits y como un enlace en las descripciones de solicitudes de extracción. Establézcalo en `false` para omitir el enlace.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code añade el enlace de sesión de claude.ai cuando realiza un commit o abre una solicitud de extracción desde una sesión en la nube o Remote Control
  * `false`: Claude Code omite el enlace
* **Default**: `true`

```json settings.json theme={null}
{
  "attribution": {
    "sessionUrl": false
  }
}
```

<span id="hook-configuration" />

<span id="hook-and-skill-settings" />

<h2 id="hooks-and-automation">
  Hooks y automatización
</h2>

Registre hooks, restrinja qué hooks se ejecutan y controle flujos de trabajo. Para eventos y cargas útiles de hooks, consulte la [referencia de hooks](/docs/es/hooks).

<h3 id="allowedhttphookurls">
  `allowedHttpHookUrls`
</h3>

Limite qué URLs pueden dirigirse a los [hooks HTTP](/docs/es/hooks#http-hook-fields). Cuando define esta clave, Claude Code ejecuta un hook HTTP solo si su URL coincide con uno de los patrones y bloquea el resto sin ejecutarlos; una matriz vacía bloquea cada hook HTTP.

* **Alcance**: [`Any file`](#scopes). Las matrices se fusionan en los archivos de configuración.
* **Tipo**: matriz de patrones de URL, con `*` como comodín
* **Predeterminado**: no establecido, por lo que se permite cualquier URL

Este ejemplo permite cualquier URL bajo `https://hooks.example.com/` y cualquier URL `http://localhost`:

```json settings.json theme={null}
{
  "allowedHttpHookUrls": ["https://hooks.example.com/*", "http://localhost:*"]
}
```

La coincidencia del nombre de host no distingue mayúsculas de minúsculas y trata `hooks.example.com.`, con el punto final que marca un nombre de dominio completamente calificado, igual que `hooks.example.com`, que es cómo DNS los trata. La lista de permitidos se aplica a hooks de todas las fuentes, incluida la configuración administrada.

<h3 id="allowmanagedhooksonly">
  `allowManagedHooksOnly`
</h3>

Restrinja la ejecución de hooks solo a los hooks que implementa su organización.

* **Alcance**: [`Managed`](#scopes)
* **Tipo**: Booleano
  * `true`: solo se ejecutan hooks administrados, más hooks del Agent SDK y hooks de plugins que su configuración administrada fuerza a habilitar. Consulte [Qué se ejecuta bajo `allowManagedHooksOnly`](#what-runs-under-allowmanagedhooksonly)
  * `false`: se ejecutan hooks de todos los alcances de configuración y plugins
* **Predeterminado**: no establecido, por lo que se ejecutan hooks de todos los alcances de configuración y plugins

```json managed-settings.json theme={null}
{
  "allowManagedHooksOnly": true
}
```

<h4 id="what-runs-under-allowmanagedhooksonly">
  Qué se ejecuta bajo `allowManagedHooksOnly`
</h4>

Cuando lo establece en `true`, Claude Code cambia qué hooks y comandos similares a hooks se cargan:

* **Se ejecutan hooks administrados y SDK**: hooks de configuración administrada y hooks que el [Agent SDK](/docs/es/agent-sdk/overview) registra en proceso
* **Se ejecutan hooks de plugins forzados a habilitarse**: hooks de plugins que su configuración administrada fuerza a habilitar a través de [`enabledPlugins`](#enabledplugins). Claude Code coincide con el ID completo `plugin@marketplace`, por lo que un plugin con el mismo nombre de un marketplace diferente permanece bloqueado. Esto le permite distribuir hooks verificados a través de un marketplace de organización mientras bloquea todo lo demás
* **Todo lo demás está bloqueado**: hooks de usuario, proyecto y locales, hooks de otros plugins y hooks declarados en frontmatter de agente
* **Los plugins con origen de comando están deshabilitados**: Claude Code también deshabilita plugins con un [origen `command`](/docs/es/plugins/marketplace-reference#command-plugin-source), incluidos plugins forzados a habilitarse en `enabledPlugins` administrado, a menos que establezca [`disableCommandPluginSources`](#disablecommandpluginsources) explícitamente en `false`
* **Los comandos `headersHelper` del marketplace están bloqueados**: Claude Code también bloquea los comandos [`headersHelper`](/docs/es/plugins/host-marketplace#authenticate-archive-downloads) del marketplace a menos que [`disableCommandPluginSources`](#disablecommandpluginsources) esté explícitamente establecido en `false`, excepto para un marketplace que la configuración administrada declara. Requiere Claude Code v2.1.238 o posterior
* **La línea de estado y la sugerencia de archivo se reducen a configuración administrada**: Claude Code lee [`statusLine`](/docs/es/statusline), [`fileSuggestion`](#filesuggestion) y [`subagentStatusLine`](/docs/es/statusline#subagent-status-lines) solo de configuración administrada, siguiendo las [puertas de línea de estado y sugerencia de archivo](#status-line-and-file-suggestion-gates)

El comando [`/goal`](/docs/es/goal) no puede ejecutarse mientras esta clave está establecida, porque depende de hooks.

<h3 id="disableallhooks">
  `disableAllHooks`
</h3>

Desactive [hooks](/docs/es/hooks#disable-or-remove-hooks), cualquier [línea de estado](/docs/es/statusline) personalizada y cualquier comando personalizado de [sugerencia de archivo](#filesuggestion). Úselo para desactivar todos estos temporalmente sin eliminarlos de su configuración.

* **Alcance**: [`Any file`](#scopes). Solo la configuración administrada puede deshabilitar hooks administrados.
* **Tipo**: Booleano
  * `true`: Claude Code desactiva hooks, cualquier línea de estado personalizada y cualquier comando personalizado de sugerencia de archivo
  * `false`: se ejecutan hooks, la línea de estado y el comando de sugerencia de archivo
* **Predeterminado**: no establecido, por lo que se ejecutan hooks

```json settings.json theme={null}
{
  "disableAllHooks": true
}
```

El alcance depende de qué archivo lleve la clave:

* **En configuración administrada**: Claude Code deshabilita cada hook configurado, incluidos los administrados, y continúa ejecutando los hooks que el [Agent SDK](/docs/es/agent-sdk/overview) registra en proceso
* **En cualquier otro archivo de configuración**: Claude Code deshabilita hooks de usuario, proyecto, locales y de plugins; los hooks administrados, hooks del Agent SDK y hooks de plugins forzados a habilitarse en [`enabledPlugins`](#enabledplugins) administrado continúan ejecutándose

Mantener los hooks del Agent SDK ejecutándose cuando la configuración administrada establece esta clave requiere Claude Code v2.1.242 o posterior.

El comando [`/goal`](/docs/es/goal) no puede ejecutarse mientras los hooks están deshabilitados, y el menú `/hooks` muestra un aviso en lugar de sus hooks.

<h4 id="status-line-and-file-suggestion-gates">
  Puertas de línea de estado y sugerencia de archivo
</h4>

Claude Code toma dos decisiones para `statusLine`, `fileSuggestion` y `subagentStatusLine`, en este orden:

* **Desactivado completamente**: cuando la configuración administrada establece `disableAllHooks`, o cuando la carpeta no es de confianza bajo la misma [regla de confianza del espacio de trabajo que los hooks en archivos de configuración](/docs/es/permissions#what-runs-before-you-trust-a-folder)
* **Reducido a configuración administrada**: cuando [`allowManagedHooksOnly`](#allowmanagedhooksonly) está establecido, cuando `disableAllHooks` es `true` fuera de la configuración administrada después de que se aplica la [precedencia de configuración](/docs/es/hooks#disable-or-remove-hooks), o cuando inicia Claude Code con `--safe-mode`

Bajo reducción, Claude Code ejecuta un valor administrado si uno está implementado. De lo contrario, omite su valor sin advertencia: la línea de estado está deshabilitada y el autocompletado `@` vuelve a la sugerencia de archivo integrada.

<h3 id="disableworkflows">
  `disableWorkflows`
</h3>

Desactive [flujos de trabajo dinámicos](/docs/es/workflows#turn-workflows-off) y los comandos de flujo de trabajo incluidos para todos los que alcanza su configuración, como una organización a través de configuración administrada. Para activar o desactivar flujos de trabajo solo para usted, use [`enableWorkflows`](#enableworkflows) en su lugar, que el interruptor **Dynamic workflows** en `/config` escribe en su configuración de usuario.

* **Alcance**: [`Any file`](#scopes)
* **Tipo**: Booleano
  * `true`: Claude Code desactiva flujos de trabajo dinámicos y los comandos de flujo de trabajo incluidos para todos los que alcanza su configuración
  * `false`: lo mismo que no establecido; si los flujos de trabajo están activados entonces sigue [`enableWorkflows`](#enableworkflows) y el predeterminado de su plan
* **Predeterminado**: `false`
* **Anulaciones por sesión**: [`CLAUDE_CODE_DISABLE_WORKFLOWS`](/docs/es/env-vars) desactiva flujos de trabajo para una sesión; cualquiera de los dos que los desactive, el otro no puede volver a activarlos

```json settings.json theme={null}
{
  "disableWorkflows": true
}
```

<h3 id="enableworkflows">
  `enableWorkflows`
</h3>

Active o desactive [flujos de trabajo dinámicos](/docs/es/workflows) para usted cuando el predeterminado de su plan no sea lo que desea. Aparece en `/config` como **Dynamic workflows**, que escribe esta clave en su configuración de usuario y la elimina nuevamente cuando alterna al predeterminado de su plan. Para desactivar flujos de trabajo para todos desde la configuración administrada, use [`disableWorkflows`](#disableworkflows) en su lugar.

* **Alcance**: [`Any file`](#scopes)
* **Tipo**: Booleano
  * `true`: Claude Code activa flujos de trabajo dinámicos para usted
  * `false`: Claude Code desactiva flujos de trabajo dinámicos para usted
* **Predeterminado**: no establecido, por lo que los flujos de trabajo están activados a menos que esté en el plan Pro, donde están desactivados
* **Anulaciones por sesión**: [`CLAUDE_CODE_DISABLE_WORKFLOWS`](/docs/es/env-vars) desactiva flujos de trabajo para una sesión, y `true` aquí no puede volver a activarlos mientras esté establecido

```json settings.json theme={null}
{
  "enableWorkflows": true
}
```

[`disableWorkflows`](#disableworkflows) y la política de flujos de trabajo de su organización también tienen precedencia: `enableWorkflows: true` no puede volver a activar flujos de trabajo mientras alguna fuente los desactiva. Claude Code oculta la fila `/config` mientras una fuente que no sea su configuración de usuario establece `enableWorkflows`, o establece `disableWorkflows` en `true`.

<h3 id="hooks">
  `hooks`
</h3>

Ejecute sus propios comandos, prompts, agentes, solicitudes HTTP o herramientas MCP como [hooks](/docs/es/hooks) en puntos del ciclo de vida de Claude Code, como antes de una llamada de herramienta o cuando comienza una sesión; la [referencia de hooks](/docs/es/hooks#hook-events) enumera cada evento, su carga útil y sus códigos de salida. Cada evento se asigna a una lista de grupos de coincidencia, y cada grupo enumera los controladores a ejecutar cuando se aplica la coincidencia.

* **Alcance**: [`Any file`](#scopes). Los hooks se fusionan en archivos en lugar de reemplazarse entre sí, y los hooks de configuración administrada no se pueden eliminar de otros archivos.
* **Tipo**: objeto codificado por [evento de hook](/docs/es/hooks#hook-events); cada valor es una matriz de grupos `{ "matcher", "hooks" }` cuyas entradas `hooks` tienen un `type` de `"command"`, `"prompt"`, `"agent"`, `"http"` o `"mcp_tool"`
* **Predeterminado**: no establecido, por lo que no se ejecutan hooks

Este ejemplo ejecuta un script antes de cada llamada de herramienta Bash:

```json settings.json theme={null}
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          { "type": "command", "command": "~/.claude/hooks/check-bash.sh" }
        ]
      }
    ]
  }
}
```

Para cada evento, patrón de coincidencia y campo de controlador, consulte la [referencia de hooks](/docs/es/hooks#configuration). Para desactivar hooks, consulte [`disableAllHooks`](#disableallhooks); para limitar hooks a los que implementa su organización, consulte [`allowManagedHooksOnly`](#allowmanagedhooksonly).

<h3 id="httphookallowedenvvars">
  `httpHookAllowedEnvVars`
</h3>

Un [hook HTTP](/docs/es/hooks#http-hook-fields) puede poner el valor de una variable de entorno en un encabezado de solicitud, por ejemplo un encabezado `Authorization: Bearer $HOOK_TOKEN`, pero solo para variables que el hook enumera en su propio `allowedEnvVars`. Esta clave establece un límite externo en esa lista para cada hook HTTP: un hook puede usar una variable solo si tanto su propio `allowedEnvVars` como esta clave la nombran. Úselo para evitar que un hook lea un secreto que no debería, incluso cuando la definición del hook lo solicita.

* **Alcance**: [`Any file`](#scopes). Las matrices se fusionan en los archivos de configuración.
* **Tipo**: matriz de nombres de variables de entorno
* **Predeterminado**: no establecido, por lo que se aplica la lista `allowedEnvVars` de cada hook

Este ejemplo limita la interpolación de encabezados a `MY_TOKEN` y `HOOK_SECRET`:

```json settings.json theme={null}
{
  "httpHookAllowedEnvVars": ["MY_TOKEN", "HOOK_SECRET"]
}
```

La lista de permitidos se aplica a hooks de todas las fuentes, incluida la configuración administrada.

<h3 id="workflowkeywordtriggerenabled">
  `workflowKeywordTriggerEnabled`
</h3>

Elija si escribir la palabra clave `ultracode` en un prompt activa un [flujo de trabajo dinámico](/docs/es/workflows#ask-for-a-workflow-in-your-prompt). Establézcalo en `false` para escribir la palabra sin activar uno.

* **Alcance**: [`Any file`](#scopes). Aparece en `/config` como **Ultracode keyword trigger**.
* **Tipo**: Booleano
  * `true`: escribir `ultracode` en un prompt activa un flujo de trabajo dinámico
  * `false`: puede escribir la palabra sin activar uno
* **Predeterminado**: `true`

```json settings.json theme={null}
{
  "workflowKeywordTriggerEnabled": false
}
```

La configuración de esfuerzo `ultracode`, `/workflows` y los comandos de flujo de trabajo guardados no se ven afectados.

<h3 id="workflowsizeguideline">
  `workflowSizeGuideline`
</h3>

Establezca el [recuento de agentes al que Claude apunta](/docs/es/workflows#set-a-size-guideline) en los flujos de trabajo dinámicos que escribe. Claude Code envía el valor a Claude como consejo, no como un límite impuesto: `"small"` solicita menos de 5 agentes, `"medium"` menos de 10 y `"large"` menos de 50. Elija `"small"` cuando desee limitar lo que gasta un flujo de trabajo. Requiere Claude Code v2.1.219 o posterior.

* **Alcance**: [`Any file`](#scopes). Un valor allí tiene precedencia sobre la opción **Dynamic workflow size** en `/config`, que Claude Code almacena en `~/.claude.json`, y Claude Code oculta esa fila mientras un archivo de configuración establece la clave.
* **Tipo**: cadena, una de:
  * `"unrestricted"`: sin directriz, por lo que Claude dimensiona el flujo de trabajo a la tarea
  * `"small"`: Claude apunta a menos de 5 agentes
  * `"medium"`: Claude apunta a menos de 10 agentes
  * `"large"`: Claude apunta a menos de 50 agentes
* **Predeterminado**: `"medium"`, o `"small"` cuando está conectado en un plan Pro con Claude Code v2.1.271 o posterior

```json settings.json theme={null}
{
  "workflowSizeGuideline": "small"
}
```

Requiere Claude Code v2.1.219 o posterior; en v2.1.202 a v2.1.218, establezca la directriz en `/config` en su lugar.

<span id="plugin-configuration" />

<span id="manage-plugins" />

<span id="plugin-settings" />

<h2 id="plugins-and-skills">
  Plugins y skills
</h2>

Habilite plugins, registre mercados, restrinja qué fuentes de plugins permite una organización y controle qué skills se cargan. Para instalar y crear plugins, consulte [Plugins](/docs/es/plugins/overview).

<h3 id="disablebundledskills">
  `disableBundledSkills`
</h3>

Desactive los [skills](/docs/es/skills) y flujos de trabajo incluidos con Claude Code. Claude Code elimina completamente los skills y flujos de trabajo incluidos, mientras que los comandos integrados como `/init` permanecen escribibles pero están ocultos del modelo.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code elimina los skills y flujos de trabajo incluidos y oculta comandos integrados como `/init` del modelo
  * `false`: los skills incluidos se cargan
* **Default**: sin establecer, por lo que los skills incluidos se cargan
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_BUNDLED_SKILLS`](/docs/es/env-vars) establecido en `1` desactiva los skills incluidos para una sesión; cualquiera de los dos que los desactive, el otro no puede volver a activarlos

```json settings.json theme={null}
{
  "disableBundledSkills": true
}
```

Los skills de plugins, `.claude/skills/` y `.claude/commands/` no se ven afectados. `/doctor` permanece escribible como los comandos integrados; para ocultarlo, establezca [`DISABLE_DOCTOR_COMMAND`](/docs/es/env-vars) en su lugar.

<h3 id="disableskillshellexecution">
  `disableSkillShellExecution`
</h3>

Desactive la ejecución de shell en línea para bloques `` !`...` `` y ` ```! ` en [skills](/es/skills) y comandos personalizados de fuentes de usuario, proyecto, plugin o directorio adicional. Claude Code reemplaza cada comando con `[shell command execution disabled by policy]` en lugar de ejecutarlo.

* **Scope**: [`Any file`](#scopes). Un `true` en configuración administrada no puede ser anulado por `false` en otro lugar.
* **Type**: Boolean
  * `true`: Claude Code reemplaza cada comando de shell en línea con `[shell command execution disabled by policy]` en lugar de ejecutarlo
  * `false`: el shell en línea se ejecuta
* **Default**: sin establecer, por lo que el shell en línea se ejecuta

```json settings.json theme={null}
{
  "disableSkillShellExecution": true
}
```

Los skills incluidos y los skills implementados a través de configuración administrada no se ven afectados.

<h3 id="skilloverrides">
  `skillOverrides`
</h3>

Oculte o contraiga un [skill](/docs/es/skills#override-skill-visibility-from-settings) sin editar su `SKILL.md`. Claude Code aplica el valor bajo el nombre de cada skill a la lista de skills que Claude ve y a su autocompletado `/`.

* **Scope**: [`Any file`](#scopes). El menú `/skills` escribe en `.claude/settings.local.json`.
* **Type**: objeto que asigna el nombre del skill a uno de:
  * `"on"`: Claude ve el skill y puede escribir `/name`
  * `"name-only"`: Claude ve el skill por nombre sin su descripción
  * `"user-invocable-only"`: Claude no ve el skill, pero aún puede escribir `/name`
  * `"off"`: Claude no ve el skill y `/name` está oculto del autocompletado
* **Default**: sin establecer, por lo que cada skill es `"on"`

Este ejemplo enumera `legacy-context` a Claude solo por nombre y oculta `deploy` de Claude y del autocompletado `/`:

```json settings.json theme={null}
{
  "skillOverrides": {
    "legacy-context": "name-only",
    "deploy": "off"
  }
}
```

Los anulaciones no se aplican a los skills de plugins, que administra a través de `/plugin`.

En configuración administrada y archivos pasados con `--settings`, una clave en un alias de skill incluido, como `checkup` para `/doctor`, también se aplica al skill; consulte [cómo las claves de alias se combinan con las claves en el nombre propio del skill](/docs/es/skills#override-skill-visibility-from-settings).

<h3 id="syncclaudeaiskills">
  `syncClaudeAiSkills`
</h3>

Desactive la descarga de los [skills habilitados para su cuenta claude.ai](/docs/es/skills#how-synced-skills-behave). Claude Code los descarga en `~/.claude/skills/synced/` en [sesiones de terminal donde inicia sesión con su cuenta claude.ai](/docs/es/skills#where-synced-skills-load), interactivas o no interactivas, y en sesiones de Cowork y en la nube. Establezca `false` para detener esa descarga y dejar de cargar los skills que ya sincronizó. Claude Code solo respeta `false`: `true` es lo mismo que sin establecer y no activa la sincronización donde de otro modo está desactivada.

* **Scope**: [`User, local, or managed`](#scopes), y archivos pasados con `--settings`. Un repositorio no puede desactivarlo para usted.
* **Type**: Boolean
  * `false`: Claude Code deja de descargar skills sincronizados y deja de cargar los que ya están en `~/.claude/skills/synced/`. En configuración de usuario o administrada, también los mueve a `~/.claude/skills/.trash/`
  * `true`: lo mismo que sin establecer
* **Default**: sin establecer, por lo que las sesiones que iniciaron sesión con su cuenta claude.ai sincronizan sus skills

Este ejemplo evita que una máquina descargue los skills de la cuenta en cualquier sesión:

```json settings.json theme={null}
{
  "syncClaudeAiSkills": false
}
```

<h3 id="syncclaudeaiplugins">
  `syncClaudeAiPlugins`
</h3>

Desactive la descarga de los [plugins habilitados para su cuenta claude.ai](/docs/es/plugins/loading#synced-plugins). Claude Code los descarga en `~/.claude/plugins/synced/` al inicio de sesiones de terminal donde inicia sesión con su cuenta claude.ai y en sesiones de Cowork, y carga cada uno como `<name>@synced`. Establezca `false` para detener esa descarga y dejar de cargar los plugins que ya sincronizó. Claude Code solo respeta `false`: `true` es lo mismo que sin establecer y no activa la sincronización donde de otro modo está desactivada. Requiere Claude Code v2.1.273 o posterior.

* **Scope**: [`User, local, or managed`](#scopes), y archivos pasados con `--settings`. Un repositorio no puede desactivarlo para usted.
* **Type**: Boolean
  * `false`: Claude Code deja de descargar plugins sincronizados y deja de cargar los que ya están en `~/.claude/plugins/synced/`. En configuración de usuario o administrada, también los mueve a `~/.claude/plugins/.trash/`
  * `true`: lo mismo que sin establecer
* **Default**: sin establecer, por lo que las sesiones que iniciaron sesión con su cuenta claude.ai sincronizan sus plugins

Para desactivar un plugin sincronizado en lugar de todos ellos, establezca `"<name>@synced": false` en [`enabledPlugins`](#enabledplugins).

Este ejemplo evita que una máquina descargue los plugins de la cuenta en cualquier sesión:

```json settings.json theme={null}
{
  "syncClaudeAiPlugins": false
}
```

<h3 id="allowedchannelplugins">
  `allowedChannelPlugins`
</h3>

Elija qué plugins de [canal](/docs/es/channels) pueden enviar mensajes a sesiones en su organización. Cuando lo establece, Claude Code usa su lista en lugar de la lista de permitidos predeterminada de Anthropic; cada entrada nombra un plugin y el mercado del que proviene.

* **Scope**: [`Managed`](#scopes)
* **Type**: matriz de objetos, cada uno con cadenas `marketplace` y `plugin`. Una entrada puede ser una cadena `"plugin@marketplace"` como `"telegram@claude-plugins-official"`, que Claude Code trata como el objeto equivalente. La forma de cadena requiere Claude Code v2.1.267 o posterior; las versiones anteriores rechazan todo el valor `allowedChannelPlugins` cuando contiene una
* **Default**: sin establecer, por lo que Claude Code usa la lista de permitidos predeterminada de Anthropic

Este ejemplo activa canales y permite solo el plugin de Telegram del mercado oficial de Anthropic:

```json managed-settings.json theme={null}
{
  "channelsEnabled": true,
  "allowedChannelPlugins": [
    { "marketplace": "claude-plugins-official", "plugin": "telegram" }
  ]
}
```

Una matriz vacía bloquea cada plugin de canal.

Esta clave entra en vigor una vez que los canales pasan la puerta [`channelsEnabled`](#channelsenabled) para la cuenta: en planes Team y Enterprise, y en cuentas de Console con configuración administrada, eso significa `channelsEnabled: true`. Consulte [Restringir qué plugins de canal pueden ejecutarse](/docs/es/channels#restrict-which-channel-plugins-can-run).

<h3 id="blockedmarketplaces">
  `blockedMarketplaces`
</h3>

Bloquee fuentes de mercado de plugins para su organización. Claude Code verifica la lista de bloqueo al agregar mercado y al instalar, actualizar, actualizar y actualizar automáticamente plugins, por lo que un mercado que alguien agregó antes de que establezca la política no puede usarse para obtener plugins tampoco. Las fuentes bloqueadas se verifican antes de la descarga, por lo que nunca tocan el sistema de archivos.

Si establece esta clave en la [consola de administrador de claude.ai](/docs/es/server-managed-settings), claude.ai también la aplica cuando alguien en su organización agrega un mercado desde un repositorio de git en claude.ai, como [Cómo funcionan las restricciones](/docs/es/plugins/org#restrict-what-users-can-install) describe.

* **Scope**: [`Managed`](#scopes)
* **Type**: matriz de objetos de fuente de mercado, en las mismas formas que [`strictKnownMarketplaces`](#allowed-source-types)
* **Default**: sin establecer, por lo que ningún mercado está bloqueado

Este ejemplo bloquea un repositorio de GitHub como fuente de mercado:

```json managed-settings.json theme={null}
{
  "blockedMarketplaces": [
    { "source": "github", "repo": "untrusted/plugins" }
  ]
}
```

Una entrada `github` puede usar la forma [comodín de propietario](#owner-wildcards) `"owner/*"` para bloquear cada repositorio bajo ese propietario de GitHub, que requiere Claude Code v2.1.223 o posterior. Agregue `{ "source": "skills-dir" }` para evitar que Claude Code cargue plugins [`@skills-dir`](/docs/es/plugins/loading#plugins-shared-through-a-repository) desde `~/.claude/skills/` sin restringir ningún mercado. Consulte [Restricciones de mercado administradas](/docs/es/plugins/org#restrict-what-users-can-install).

<h3 id="channelsenabled">
  `channelsEnabled`
</h3>

Permita [canales](/docs/es/channels) para su organización. En planes Team y Enterprise de claude.ai, Claude Code bloquea canales hasta que establezca esto en `true`. Para cuentas de [Anthropic Console](/docs/es/authentication#claude-console-authentication) que se autentican con una clave API, los canales se permiten de forma predeterminada. Si su organización implementa configuración administrada, Claude Code también bloquea canales en esas cuentas hasta que establezca esta clave en `true`.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code permite canales para su organización
  * `false`: lo mismo que sin establecer; si los canales están bloqueados depende de su plan, como dice el Default
* **Default**: sin establecer; los canales están bloqueados en planes Team y Enterprise y en cuentas de Console con configuración administrada, y permitidos en planes Pro y Max y en cuentas de Console sin configuración administrada

```json managed-settings.json theme={null}
{
  "channelsEnabled": true
}
```

Para restringir qué plugins pueden registrarse como canales una vez que estén habilitados, establezca [`allowedChannelPlugins`](#allowedchannelplugins). Consulte [Controles empresariales](/docs/es/channels#enterprise-controls).

<h3 id="disablecommandpluginsources">
  `disableCommandPluginSources`
</h3>

Bloquee la [fuente de plugin `command`](/docs/es/plugins/marketplace-reference#command-plugin-source), que instala un plugin ejecutando un comando declarado por el mercado en la máquina del usuario. Cuando lo establece en `true`, Claude Code nunca ejecuta el comando, no instala ni actualiza plugins de origen de comando, y deja de cargar los ya instalados. Establézcalo en `false` para permitirlos explícitamente. Siempre que bloquea fuentes de comando, ya sea que lo establezca en `true` o lo deje sin establecer bajo [`allowManagedHooksOnly`](#allowmanagedhooksonly), también bloquea comandos [`headersHelper`](/docs/es/plugins/host-marketplace#authenticate-archive-downloads) del mercado, excepto para un mercado que la configuración administrada declara. Requiere Claude Code v2.1.229 o posterior, y el bloqueo `headersHelper` requiere v2.1.238 o posterior.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code nunca ejecuta el comando declarado por el mercado, no instala ni actualiza plugins de origen de comando, y deja de cargar los ya instalados
  * `false`: Claude Code permite plugins de origen de comando explícitamente
* **Default**: sin establecer, por lo que Claude Code sigue [`allowManagedHooksOnly`](#allowmanagedhooksonly): una organización que restringe la ejecución de hooks a configuración administrada también obtiene fuentes de comando deshabilitadas

```json managed-settings.json theme={null}
{
  "disableCommandPluginSources": true
}
```

Requiere Claude Code v2.1.229 o posterior.

<h3 id="pluginsuggestionmarketplaces">
  `pluginSuggestionMarketplaces`
</h3>

Nombre los mercados cuyos plugins pueden aparecer como sugerencias de instalación contextual, en consejos de spinner y fijados en la parte superior de la pestaña **Discover** de `/plugin`. La sugerencia de diseño de interfaz de primera parte integrada no se ve afectada. Las sugerencias provienen de la declaración `relevance` de cada plugin en su entrada de mercado.

* **Scope**: [`Managed`](#scopes)
* **Type**: matriz de nombres de mercado
* **Default**: sin establecer, por lo que no aparecen sugerencias declaradas por mercado

```json managed-settings.json theme={null}
{
  "pluginSuggestionMarketplaces": ["acme-corp-plugins"]
}
```

Un nombre entra en vigor solo cuando el mercado está registrado en la máquina y su fuente registrada también se declara en la misma configuración administrada, ya sea como la entrada [`extraKnownMarketplaces`](#extraknownmarketplaces) para ese nombre o como una entrada de [`strictKnownMarketplaces`](#strictknownmarketplaces). Claude Code ignora un mercado registrado desde una fuente diferente bajo un nombre en la lista de permitidos. El mercado oficial está exento del requisito de fuente: permitir solo su nombre es suficiente, ya que ese nombre solo puede registrarse desde la fuente oficial de Anthropic. Consulte [Sugerir plugins por contexto](/docs/es/plugins/relevance).

<h3 id="plugintrustmessage">
  `pluginTrustMessage`
</h3>

Agregue el texto de su propia organización a la advertencia de confianza de plugin que Claude Code muestra antes de la instalación, por ejemplo para confirmar que los plugins de su mercado interno están revisados.

* **Scope**: [`Managed`](#scopes)
* **Type**: cadena
* **Default**: sin establecer, por lo que Claude Code muestra solo la advertencia estándar

```json managed-settings.json theme={null}
{
  "pluginTrustMessage": "All plugins from our marketplace are approved by IT"
}
```

<h3 id="strictknownmarketplaces">
  `strictKnownMarketplaces`
</h3>

Restrinja qué fuentes de mercado de plugins pueden agregar e instalar plugins las personas en su organización. Claude Code aplica la lista de permitidos al agregar mercado y al instalar, actualizar, actualizar y actualizar automáticamente plugins, antes de cualquier operación de red o sistema de archivos, por lo que un mercado que alguien agregó antes de que establezca la política no puede usarse para obtener plugins una vez que su fuente ya no coincida. Los usuarios bloqueados ven un error que nombra la política administrada.

Si establece esta clave en la [consola de administrador de claude.ai](/docs/es/server-managed-settings), claude.ai también la aplica cuando alguien en su organización agrega un mercado desde un repositorio de git en claude.ai, como [Cómo funcionan las restricciones](/docs/es/plugins/org#restrict-what-users-can-install) describe.

* **Scope**: [`Managed`](#scopes)
* **Type**: matriz de objetos de fuente de mercado; consulte [Tipos de fuente permitidos](#allowed-source-types)
* **Default**: sin establecer, por lo que los usuarios pueden agregar cualquier mercado. Una matriz vacía es un bloqueo completo que bloquea cada fuente de mercado, incluido el mercado oficial de Anthropic

Este ejemplo permite dos repositorios de GitHub, uno fijado a la ref `v2.0` y uno alojado en URL `marketplace.json`:

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/approved-plugins" },
    { "source": "github", "repo": "acme-corp/security-tools", "ref": "v2.0" },
    { "source": "url", "url": "https://plugins.example.com/marketplace.json" }
  ]
}
```

También puede escribir esta clave como `allowedMarketplaces`; [Alias de clave de mercado](#marketplace-key-aliases) describe cómo Claude Code trata el alias y qué versión lo acepta. Esta clave es una puerta de política: controla lo que los usuarios pueden agregar pero no registra nada. Para restringir y preregistrar en un archivo, consulte [Combinar con `extraKnownMarketplaces`](#combine-with-extraknownmarketplaces). Para la vista orientada al usuario, consulte [Restricciones de mercado administradas](/docs/es/plugins/org#restrict-what-users-can-install).

<h4 id="allowed-source-types">
  Tipos de fuente permitidos
</h4>

Cada entrada a continuación muestra una entrada de lista de permitidos por tipo de fuente y los campos que acepta. La mayoría de los tipos coinciden exactamente; `hostPattern` y `pathPattern` coinciden por regex, y las entradas `github` pueden usar un [comodín de propietario](#owner-wildcards).

| Source        | Example entry                                                                                                                   | Fields                                                                                                                                                |
| :------------ | :------------------------------------------------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------- |
| `github`      | `{ "source": "github", "repo": "acme-corp/plugins", "ref": "main", "path": "marketplace" }`                                     | `repo` requerido; `ref` es una rama o etiqueta; `path` es un subdirectorio                                                                            |
| `git`         | `{ "source": "git", "url": "https://gitlab.example.com/tools/plugins.git", "ref": "production" }`                               | `url` requerido; `ref` y `path` como para `github`                                                                                                    |
| `url`         | `{ "source": "url", "url": "https://plugins.example.com/marketplace.json", "headers": { "Authorization": "Bearer ${TOKEN}" } }` | `url` requerido; `headers` agrega encabezados HTTP para acceso autenticado                                                                            |
| `file`        | `{ "source": "file", "path": "/opt/acme-corp/plugins/marketplace.json" }`                                                       | `path` requerido, la ruta absoluta a un archivo `marketplace.json`                                                                                    |
| `directory`   | `{ "source": "directory", "path": "/opt/acme-corp/approved-marketplaces" }`                                                     | `path` requerido, la ruta absoluta a un directorio que contiene `.claude-plugin/marketplace.json`                                                     |
| `hostPattern` | `{ "source": "hostPattern", "hostPattern": "^github\\.example\\.com$" }`                                                        | `hostPattern` requerido, una regex coincidida en cualquier lugar del host del mercado; anclela con `^` y `$` para coincidir con el host completo      |
| `pathPattern` | `{ "source": "pathPattern", "pathPattern": "^/opt/approved/" }`                                                                 | `pathPattern` requerido, una regex coincidida en cualquier lugar en la `path` de fuentes `file` y `directory`; comience con `^` para fijar un prefijo |
| `skills-dir`  | `{ "source": "skills-dir" }`                                                                                                    | Sin campos. Opta por el escaneo de plugin `~/.claude/skills/` nuevamente                                                                              |

Tres tipos de fuente llevan reglas más allá de la tabla:

* **`url`**: un mercado de URL descarga solo el archivo `marketplace.json`, y Claude Code no obtiene archivos de plugin por ruta relativa desde ese servidor, por lo que sus plugins deben usar una [fuente de plugin](/docs/es/plugins/marketplace-reference#plugin-sources) que no sea una ruta relativa, como una URL de archivo, que puede estar en el mismo host. Para plugins con rutas relativas, use un mercado basado en Git en su lugar. Consulte [Los plugins con rutas relativas fallan en mercados basados en URL](/docs/es/plugins/troubleshooting#plugins-with-relative-paths-fail-in-url-based-marketplaces).
* **`hostPattern`**: úselo para permitir cada mercado en un GitHub Enterprise interno o servidor GitLab sin enumerar cada repositorio. Claude Code compara fuentes `github` contra `github.com`, toma el nombre de host de fuentes `url`, y lo toma de fuentes `git` dependiendo de la forma de [URL de git](https://git-scm.com/docs/git-clone#_git_urls):

  * Una URL con un esquema, como `https://` o `ssh://`: el nombre de host en la URL.
  * Una dirección SSH sin esquema, en la forma `user@host:path` de git, como `git@git.example.com:tools/plugins.git`: el host entre `@` y `:`, que es el host al que se conecta git.
  * Cualquier otra forma sin esquema: sin host, por lo que ninguna entrada `strictKnownMarketplaces` `hostPattern` la coincide. Para una `blockedMarketplaces` `hostPattern`, Claude Code toma un host de un conjunto más amplio de formas, por lo que una entrada de lista de bloqueo aún puede coincidir con tal forma. Antes de v2.1.234, una `strictKnownMarketplaces` `hostPattern` también coincidía con algunas formas que git no trata como direcciones SSH.

  Las fuentes `file` y `directory` no tienen host y nunca coinciden con una entrada `hostPattern`.
* **`pathPattern`**: úselo para permitir mercados del sistema de archivos junto con entradas `hostPattern` para fuentes de red. `".*"` permite cada ruta local; un patrón más estrecho como `"^/opt/approved/"` restringe a un directorio.

Cualquier lista de permitidos, incluso una vacía, también evita que Claude Code cargue plugins [`@skills-dir`](/docs/es/plugins/loading#plugins-shared-through-a-repository) desde `~/.claude/skills/`. Agregue la entrada `{ "source": "skills-dir" }` para seguir cargándolos; la entrada no tiene significado fuera de esta clave y `blockedMarketplaces`.

<h4 id="owner-wildcards">
  Comodines de propietario
</h4>

Una entrada `github` cuyo valor `repo` es `"<owner>/*"` coincide con cada repositorio bajo ese propietario de GitHub. Los comodines de propietario requieren Claude Code v2.1.223 o posterior y funcionan solo en `strictKnownMarketplaces` y `blockedMarketplaces`. En cualquier otro lugar donde aparezca una fuente `github`, como `extraKnownMarketplaces` o `/plugin marketplace add`, el valor `repo` debe nombrar un único repositorio. Antes de v2.1.223, Claude Code comparaba la entrada literalmente, por lo que una entrada de lista de permitidos no coincidía con ningún repositorio y una entrada de lista de bloqueo no bloqueaba nada; las entradas de repositorio único se aplican en cada versión.

Esta entrada permite cualquier repositorio de mercado en la organización `acme-corp`:

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/*" }
  ]
}
```

Solo la posición de nombre de repositorio completo puede ser un comodín. Claude Code compara entradas como `*`, `*/plugins` o `acme-corp/tools-*` literalmente, por lo que no coinciden con ningún repositorio.

Las reglas de coincidencia difieren entre las dos configuraciones:

| Rule                                  | `strictKnownMarketplaces`                                                                                                                                                                                | `blockedMarketplaces`                                                                              |
| ------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| Coincidencia de ortografías de fuente | Solo forma `owner/repo`. Una URL de git que clona el mismo repositorio no coincide                                                                                                                       | Cualquier ortografía, incluidas las URL de git que se resuelven en el mismo repositorio github.com |
| Caso del propietario                  | Sensible a mayúsculas y minúsculas, como coincidencia exacta de entrada                                                                                                                                  | Insensible a mayúsculas y minúsculas                                                               |
| `ref`                                 | Sigue las reglas de coincidencia exacta de entrada: una entrada con una `ref` coincide solo con fuentes con esa ref exacta, y una entrada sin una coincide solo con fuentes que no especifican una ref   | Una entrada sin una `ref` bloquea todas las refs de los repositorios que coincide                  |
| `path`                                | Más flexible que las reglas de coincidencia exacta de entrada: una entrada con una `path` requiere ese valor exacto, mientras que una entrada sin una coincide con cualquier ruta dentro del repositorio | Una entrada sin una `path` bloquea todas las rutas de los repositorios que coincide                |

<h4 id="exact-matching">
  Coincidencia exacta
</h4>

Para cada tipo de fuente excepto entradas `github` de comodín de propietario y entradas `hostPattern` y `pathPattern` coincididas por regex, Claude Code permite una adición de usuario solo cuando la fuente de mercado coincide exactamente con una entrada. Para las fuentes basadas en git `github` y `git`, la coincidencia exacta incluye los campos opcionales:

* El `repo` o `url` debe coincidir exactamente
* El campo `ref` debe coincidir exactamente, o ambos deben ser indefinidos
* El campo `path` debe coincidir exactamente, o ambos deben ser indefinidos

Por ejemplo, Claude Code trata cada par a continuación como dos fuentes diferentes:

* `{ "source": "github", "repo": "acme-corp/plugins" }` y `{ "source": "github", "repo": "acme-corp/plugins", "ref": "main" }`
* `{ "source": "github", "repo": "acme-corp/plugins", "path": "marketplace" }` y `{ "source": "github", "repo": "acme-corp/plugins" }`

<h4 id="allow-only-the-official-marketplace">
  Permitir solo el mercado oficial
</h4>

Para permitir el mercado oficial de Anthropic y nada más, enumere su repositorio:

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "anthropics/claude-plugins-official" }
  ]
}
```

Con esta entrada, Claude Code mantiene un mercado oficial ya registrado disponible y, en una máquina nueva, registra el mercado automáticamente la primera vez que inicia Claude Code interactivamente. El registro automático comúnmente pierde:

* Entornos no interactivos que se ejecutan antes del primer lanzamiento interactivo de la máquina.
* Máquinas donde Claude Code ya se ejecutó interactivamente bajo una política que bloqueó el mercado, como el bloqueo de matriz vacía. Claude Code registra el intento bloqueado y no reintenta después de que cambia la política.

En estas máquinas, agregue el mercado a [`extraKnownMarketplaces`](#extraknownmarketplaces) en el mismo `managed-settings.json` para que Claude Code lo registre automáticamente, o ejecute `claude plugin marketplace add anthropics/claude-plugins-official`.

<h4 id="combine-with-extraknownmarketplaces">
  Combinar con `extraKnownMarketplaces`
</h4>

Las dos claves hacen trabajos diferentes. Esta tabla las compara:

| Aspect            | `strictKnownMarketplaces`                         | `extraKnownMarketplaces`                                                                                                                              |
| ----------------- | ------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| Purpose           | Aplicación de política organizacional             | Conveniencia del equipo                                                                                                                               |
| Settings file     | Solo configuración administrada                   | Cualquier archivo de configuración                                                                                                                    |
| Behavior          | Bloquea adiciones no permitidas                   | Registra mercados faltantes                                                                                                                           |
| When enforced     | Antes de operaciones de red y sistema de archivos | Inmediatamente desde configuración de usuario o administrada; después del diálogo de confianza del espacio de trabajo para archivos de un repositorio |
| Can be overridden | No, precedencia más alta                          | Sí, por configuración de precedencia más alta                                                                                                         |
| Source format     | Objeto de fuente directo                          | Mercado nombrado con un objeto `source` anidado                                                                                                       |

Para restringir y preregistrar un mercado para todos los usuarios, establezca ambos en `managed-settings.json`:

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/plugins" }
  ],
  "extraKnownMarketplaces": {
    "acme-tools": {
      "source": { "source": "github", "repo": "acme-corp/plugins" }
    }
  }
}
```

Con solo `strictKnownMarketplaces` establecido, los usuarios aún pueden agregar un mercado permitido ellos mismos con `/plugin marketplace add`. El mercado oficial de Anthropic es el único que Claude Code registra automáticamente, y solo cuando la lista de permitidos lo permite. [Permitir solo el mercado oficial](#allow-only-the-official-marketplace) enumera las máquinas que pierde.

<h3 id="strictpluginonlycustomization">
  `strictPluginOnlyCustomization`
</h3>

Bloquee skills, agentes, hooks y servidores MCP de fuentes de usuario y proyecto, para que solo puedan provenir de plugins o configuración administrada. Combínelo con [`strictKnownMarketplaces`](#strictknownmarketplaces) para controlar la cadena de suministro de personalización completa: la lista de permitidos de mercado controla qué plugins pueden instalar los usuarios.

* **Scope**: [`Managed`](#scopes)
* **Type**: `true` para bloquear los cuatro tipos de personalización, o una matriz que nombre los tipos a bloquear, de `"skills"`, `"agents"`, `"hooks"` y `"mcp"`
* **Default**: sin establecer, por lo que nada está bloqueado

Este ejemplo bloquea skills y hooks y deja agentes y servidores MCP desbloqueados:

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["skills", "hooks"]
}
```

Las cuatro entradas de subclave a continuación enumeran lo que cada superficie bloquea y qué aún se carga. Claude Code ignora nombres de superficie que no reconoce en lugar de fallar el archivo de configuración, por lo que puede agregar nuevos nombres de superficie antes de que cada cliente se actualice.

<h3 id="strictpluginonlycustomization-skills">
  `strictPluginOnlyCustomization.skills`
</h3>

Bloquee la superficie `skills`. Claude Code deja de cargar skills de `~/.claude/skills/` y `.claude/skills/`, comandos personalizados de `~/.claude/commands/` y `.claude/commands/`, skills bajo directorios `--add-dir`, y skills sincronizados desde su cuenta claude.ai, y sigue cargando skills de plugins, skills incluidos y skills en el directorio de política administrada.

* **Scope**: [`Managed`](#scopes)
* **Type**: la cadena `"skills"` en la matriz [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)
* **Default**: no bloqueado

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["skills"]
}
```

<h3 id="strictpluginonlycustomization-agents">
  `strictPluginOnlyCustomization.agents`
</h3>

Bloquee la superficie `agents`. Claude Code deja de cargar agentes de `~/.claude/agents/` y `.claude/agents/`, y sigue cargando agentes de plugins, agentes integrados y agentes en el directorio de política administrada.

* **Scope**: [`Managed`](#scopes)
* **Type**: la cadena `"agents"` en la matriz [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)
* **Default**: no bloqueado

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["agents"]
}
```

<h3 id="strictpluginonlycustomization-hooks">
  `strictPluginOnlyCustomization.hooks`
</h3>

Bloquee la superficie `hooks`. Claude Code deja de ejecutar hooks de configuración de usuario, proyecto y local `settings.json`, y sigue ejecutando hooks de plugins y hooks en configuración administrada.

* **Scope**: [`Managed`](#scopes)
* **Type**: la cadena `"hooks"` en la matriz [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)
* **Default**: no bloqueado

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["hooks"]
}
```

<h3 id="strictpluginonlycustomization-mcp">
  `strictPluginOnlyCustomization.mcp`
</h3>

Bloquee la superficie `mcp`. Claude Code deja de cargar servidores MCP de `~/.claude.json` y `.mcp.json`, y sigue cargando servidores MCP de plugins, servidores [`managed-mcp.json`](/docs/es/managed-mcp) y servidores de [`managedMcpServers`](#managedmcpservers).

* **Scope**: [`Managed`](#scopes)
* **Type**: la cadena `"mcp"` en la matriz [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)
* **Default**: no bloqueado

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["mcp"]
}
```

<h3 id="enabledplugins">
  `enabledPlugins`
</h3>

Active o desactive [plugins](/docs/es/plugins/overview) individuales, codificados por `plugin-name@marketplace-name`. Un plugin sin entrada en ningún scope vuelve a su valor [`defaultEnabled`](/docs/es/plugins/manifest-reference#fields). Cuando habilita o deshabilita un plugin con `/plugin` o `claude plugin enable`, Claude Code escribe esta clave para usted.

* **Scope**: [`Any file`](#scopes)
* **Type**: objeto que asigna `plugin-name@marketplace-name` a un Boolean
* **Default**: sin establecer, por lo que cada plugin sigue su valor `defaultEnabled`

Este ejemplo habilita dos plugins del mercado `team-tools` y deshabilita uno de `personal`:

```json settings.json theme={null}
{
  "enabledPlugins": {
    "code-formatter@team-tools": true,
    "deployment-tools@team-tools": true,
    "experimental-features@personal": false
  }
}
```

Cada scope sirve un propósito diferente:

* **Configuración de usuario**: sus preferencias personales de plugin
* **Configuración de proyecto**: plugins compartidos con todos en el repositorio
* **Configuración local**: anulaciones por máquina, ignoradas cuando Claude Code guarda una configuración allí
* **Configuración administrada**: política de toda la organización. Un plugin establecido en `false` aquí está bloqueado de instalación en cada scope y oculto del mercado

La configuración del proyecto tiene precedencia sobre la configuración del usuario, por lo que establecer un plugin en `false` en `~/.claude/settings.json` no deshabilita un plugin que la `.claude/settings.json` del proyecto habilita. Para optar por no participar en un plugin habilitado por proyecto en su máquina, establézcalo en `false` en `.claude/settings.local.json` en su lugar. Los plugins forzados habilitados por configuración administrada no pueden deshabilitarse de esta manera, ya que la configuración administrada anula la configuración local.

Habilitar un plugin de una fuente externa como un repositorio de GitHub o paquete npm en la `.claude/settings.json` de un proyecto no lo instala para otras personas. En cada ruta que carga plugins, Claude Code reporta el plugin como no instalado hasta que cada usuario lo [instale ellos mismos](/docs/es/plugins/org#require-plugins-per-repository).

<h3 id="extraknownmarketplaces">
  `extraKnownMarketplaces`
</h3>

Registre mercados de plugins adicionales por nombre, para que las personas que abran el repositorio, o todos los que alcance su configuración administrada, obtengan el mercado sin agregarlo ellos mismos. Claude Code registra cada mercado que aún no conoce. Si un plugin que [`enabledPlugins`](#enabledplugins) nombra desde él se instala depende de la fuente del plugin y qué archivo lo habilita; esa entrada tiene las reglas.

* **Scope**: [`Any file`](#scopes). Claude Code respeta entradas en la `.claude/settings.json` o `.claude/settings.local.json` de un repositorio solo después de que acepte el diálogo de confianza del espacio de trabajo para esa carpeta; en una carpeta que no ha confiado, incluida una ejecución `-p` allí, las ignora sin un mensaje.
* **Type**: objeto que asigna un nombre de mercado a un objeto con un objeto `source` y un Boolean `autoUpdate` opcional
* **Default**: sin establecer

Este ejemplo registra un mercado de GitHub y un mercado desde una URL de git autohospedada:

```json settings.json theme={null}
{
  "extraKnownMarketplaces": {
    "acme-tools": {
      "source": {
        "source": "github",
        "repo": "acme-corp/claude-plugins"
      }
    },
    "security-plugins": {
      "source": {
        "source": "git",
        "url": "https://git.example.com/security/plugins.git"
      }
    }
  }
}
```

[Lo que se ejecuta antes de que confíe en una carpeta](/docs/es/permissions#what-runs-before-you-trust-a-folder) compara la puerta de confianza con el otro contenido que un repositorio puede suministrar. También puede escribir esta clave como `additionalMarketplaces`; consulte [Alias de clave de mercado](#marketplace-key-aliases).

Establezca `"autoUpdate": true` junto a `source` para hacer que Claude Code actualice ese mercado e instale sus plugins instalados en segundo plano después del inicio. Cuando se omite, `claude-plugins-official` y la mayoría de otros mercados oficiales de Anthropic tienen como predeterminado `true`, y los mercados de terceros tienen como predeterminado `false`. Consulte [Configurar actualizaciones automáticas](/docs/es/plugins/install#keep-plugins-updated).

Cuando más de un archivo de configuración define una entrada de mercado bajo el mismo nombre, Claude Code usa la entrada del [archivo de precedencia más alta](/docs/es/settings#settings-precedence) completo. Esa entrada reemplaza la entrada de precedencia más baja y no hereda ninguno de sus campos, por lo que una redefinición no puede combinar `source.headers` de credencial de un archivo con una URL que otro archivo controla. Antes de v2.1.228, Claude Code fusionaba entradas del mismo nombre campo por campo, por lo que una entrada en un archivo de precedencia más alta podría heredar campos que no estableció, incluidos `headers` de otro archivo.

<h4 id="marketplace-source-types">
  Tipos de fuente de mercado
</h4>

El objeto `source` toma una de estas formas:

* **`github`**: un repositorio de GitHub, con `repo`
* **`git`**: cualquier URL de git, con `url`
* **`url`**: una URL directa a un archivo `marketplace.json`, con `url` y `headers` opcional y `headersHelper` para acceso autenticado. `headersHelper` nombra un comando que imprime encabezados cuyos valores son demasiado efímeros para enumerar en `headers`, y requiere Claude Code v2.1.238 o posterior
* **`file`**: una ruta local a un archivo `marketplace.json`, con `path`
* **`directory`**: una ruta del sistema de archivos local, con `path`, solo para desarrollo
* **`settings`**: un mercado en línea declarado directamente en el archivo de configuración sin un repositorio alojado, con `name` y `plugins`

El tipo de fuente `git` funciona con cualquier servicio de alojamiento de git, incluido GitLab autohospedado y Bitbucket. Claude Code clona el repositorio con la misma autenticación que `git clone` usaría en esa máquina: ayudantes de credenciales configurados o claves SSH. Un token de proveedor como `GITHUB_TOKEN` entra en vigor solo a través de un ayudante de credenciales que lo lee. Consulte [Repositorios privados](/docs/es/plugins/host-marketplace#grant-access-to-a-private-marketplace) para detalles de configuración.

Para fuentes `github` y `git`, Claude Code nunca descarga contenido de [Git LFS](https://git-lfs.com) cuando clona el repositorio de mercado para agregarlo o actualizarlo. Los archivos rastreados por LFS se extraen como archivos de puntero, y la salida de agregar o actualizar reporta cuántos.

El campo `skipLfs` dentro del objeto `source` se acepta y no tiene efecto. Antes de v2.1.274, Claude Code descargaba contenido de LFS a menos que estableciera `"skipLfs": true`.

Para una fuente `url`, establezca `headersHelper` dentro del objeto `source` cuando la credencial en `headers` expira y un comando tiene que producir una nueva. Requiere Claude Code v2.1.238 o posterior. Para lo que el comando debe imprimir y dónde Claude Code lo ejecuta, consulte [Escribir el comando headersHelper](/docs/es/plugins/host-marketplace#write-the-headershelper-command), y para los casos donde Claude Code no lo ejecuta, consulte [Cuándo Claude Code omite un comando headersHelper](/docs/es/plugins/host-marketplace#when-claude-code-skips-a-headershelper-command-or-drops-its-output). Una vez que establezca `headersHelper` en una URL de mercado `https://`, Claude Code ejecuta el comando en dos puntos, reutilizando la salida de una ejecución durante hasta 60 segundos:

* Antes de cada obtención del `marketplace.json` de ese mercado, incluida una actualización posterior. Claude Code envía los encabezados impresos con esa obtención.
* Antes de cada descarga de archivo de plugin en el origen de la URL del mercado, lo que significa el mismo esquema, host y puerto. Claude Code envía la salida con esa descarga, y ninguna otra descarga obtiene los encabezados.

Claude Code ignora cualquier `headersHelper` establecido en la `.claude/settings.json` o `.claude/settings.local.json` de un directorio que agregue con [`--add-dir`](/docs/es/permissions#what-runs-before-you-trust-a-folder), en una fuente `url` y en una entrada de plugin en línea por igual, y envía solo los `headers` fijos establecidos en ese archivo. [Cómo los usuarios aceptan un comando headersHelper](/docs/es/plugins/host-marketplace#how-users-accept-a-headershelper-command) cubre los otros archivos de configuración.

Los plugins enumerados en una fuente `settings` deben hacer referencia a fuentes externas como GitHub o npm, y el `name` debe coincidir con la clave de mercado. Aún habilita cada plugin por separado en `enabledPlugins`. Este ejemplo declara un plugin en línea:

```json settings.json theme={null}
{
  "extraKnownMarketplaces": {
    "team-tools": {
      "source": {
        "source": "settings",
        "name": "team-tools",
        "plugins": [
          {
            "name": "code-formatter",
            "source": {
              "source": "github",
              "repo": "acme-corp/code-formatter"
            }
          }
        ]
      }
    }
  }
}
```

Una entrada de plugin bajo `source: 'settings'` cuya propia `source` es un [`archive`](/docs/es/plugins/marketplace-reference#archive-plugin-source) puede establecer `headers` para la descarga del archivo. Si el valor que pondría en `headers` es efímero, como un token que su registro acuña bajo demanda, establezca un comando `headersHelper` en su lugar. Una entrada puede establecer ambos. Ambos campos requieren Claude Code v2.1.238 o posterior.

Claude Code envía los `headers` de la entrada, y lo que el comando imprime, con la descarga del archivo de ese plugin y con ninguna otra descarga. Claude Code ejecuta el comando solo cuando un usuario [instala o actualiza ese único plugin por sí solo](/docs/es/plugins/host-marketplace#how-users-accept-a-headershelper-command). Tres reglas adicionales dependen de qué archivo contiene la entrada:

* **`strict`**: a diferencia de una entrada en el `marketplace.json` de un mercado, una entrada en configuración no necesita `"strict": false`, porque un archivo de configuración no lleva campos de manifiesto para en línea. Consulte [Modo estricto](/docs/es/plugins/marketplace-reference#strict-mode).
* **Confianza de carpeta**: para una entrada en la `.claude/settings.json` o `.claude/settings.local.json` de un proyecto, Claude Code ejecuta el comando solo después de que el usuario también haya [confiado en esa carpeta](/docs/es/permissions#what-runs-before-you-trust-a-folder).
* **Filtro de encabezado**: Claude Code elimina [nombres de encabezado de enrutamiento de solicitud e identidad del cliente](/docs/es/plugins/host-marketplace#when-claude-code-skips-a-headershelper-command-or-drops-its-output) de una entrada en la `.claude/settings.json` o `.claude/settings.local.json` de un proyecto, porque un repositorio puede suministrar esos archivos. Claude Code aplica el mismo filtro a una entrada de catálogo y a una entrada en la configuración de un directorio `--add-dir`, y ningún filtro a una entrada en su configuración de usuario, un archivo `--settings` o configuración administrada.

<h4 id="marketplace-key-aliases">
  Alias de clave de mercado
</h4>

En Claude Code v2.1.232 o posterior, puede escribir `extraKnownMarketplaces` como `additionalMarketplaces` y `strictKnownMarketplaces` como `allowedMarketplaces`. Claude Code trata cada alias de la siguiente manera:

* Las versiones anteriores ignoran el alias, por lo que mantenga la ortografía canónica en un archivo que las versiones anteriores también lean, como un archivo de configuración administrada para una flota con versiones mixtas de Claude Code.
* En cualquier archivo de configuración que acepte la clave canónica, Claude Code lee el alias exactamente como lee la clave canónica.
* Claude Code puede reescribir `additionalMarketplaces` a `extraKnownMarketplaces` cuando actualiza el archivo.
* Si establece ambas ortografías en un archivo, Claude Code usa el valor canónico e ignora el alias.

<h3 id="pluginconfigs">
  `pluginConfigs`
</h3>

Almacene las respuestas no sensibles que proporciona al diálogo de configuración [`userConfig`](/docs/es/plugins/manifest-reference#user-configuration) de un plugin, codificadas por ID de plugin. Claude Code escribe esta clave en su configuración de usuario cuando completa el diálogo, por lo que no necesita editarla a mano. Claude Code almacena opciones sensibles en el Keychain de macOS en su lugar, retrocediendo a `~/.claude/.credentials.json` cuando el Keychain rechaza la escritura; en plataformas sin un keychain compatible, las almacena en `~/.claude/.credentials.json`.

* **Scope**: [`User or managed`](#scopes)
* **Type**: objeto que asigna un ID de plugin a un objeto con un campo `options`, asignando cada nombre de opción a una cadena, número, Boolean o matriz de cadenas, y un campo `mcpServers` opcional que contiene valores de configuración de usuario por servidor en la misma forma
* **Default**: sin establecer

Este ejemplo almacena la opción `api_endpoint` para el plugin `deployer` de `acme-tools`:

```json settings.json theme={null}
{
  "pluginConfigs": {
    "deployer@acme-tools": {
      "options": {
        "api_endpoint": "https://api.example.com"
      }
    }
  }
}
```

Los plugins integrados almacenan sus opciones bajo la misma clave con un sufijo `@builtin`. Por ejemplo, la configuración [**Instrucciones del proyecto**](/docs/es/memory#choose-which-instruction-files-load) que controla si Claude Code lee archivos `AGENTS.md` es `pluginConfigs["agents-md@builtin"].options.instructionFiles`.

Claude Code ignora entradas de proyecto y local porque sustituye estos valores en configuraciones de hook de plugin, MCP y LSP, y un repositorio clonado no debe poder suministrarlos. Antes de v2.1.207, la configuración de proyecto y local también se leía.

<h2 id="mcp">
  MCP
</h2>

Controle a qué servidores MCP se conecta Claude Code y cuáles permite una organización. Consulte [Conectarse a herramientas externas con MCP](/docs/es/mcp) y [Configuración de MCP administrada](/docs/es/managed-mcp).

<h3 id="allowallclaudeaimcps">
  `allowAllClaudeAiMcps`
</h3>

Cargue los [conectores de claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai) que Claude Code obtiene por sí mismo junto con un `managed-mcp.json` implementado. Sin esta clave, `managed-mcp.json` toma control exclusivo de los servidores MCP y suprime esos conectores.

* **Scope**: [`Managed`](#scopes). Los usuarios no pueden volver a habilitar los conectores que el control exclusivo suprimió.
* **Type**: Boolean
  * `true`: Claude Code carga los conectores de claude.ai junto con un `managed-mcp.json` implementado
  * `false`: un `managed-mcp.json` implementado toma control exclusivo de los servidores MCP y suprime los conectores de claude.ai [que Claude Code obtiene por sí mismo](/docs/es/mcp#how-connectors-reach-claude-code)
* **Default**: `false`, por lo que un `managed-mcp.json` implementado suprime los conectores de claude.ai que Claude Code obtiene por sí mismo

```json managed-settings.json theme={null}
{
  "allowAllClaudeAiMcps": true
}
```

[`allowedMcpServers`](#allowedmcpservers) y [`deniedMcpServers`](#deniedmcpservers) aún se aplican a los conectores que esta clave carga. Los conectores entregados a una [sesión en la nube](/docs/es/claude-code-on-the-web) cuyo host lleva un `managed-mcp.json`, como un ejecutor autohospedado, permanecen suprimidos. Consulte [Permitir conectores de claude.ai junto con el conjunto administrado](/docs/es/managed-mcp#allow-claude-ai-connectors-alongside-the-managed-set).

<h3 id="allowedmcpservers">
  `allowedMcpServers`
</h3>

Cree una lista de permitidos de los servidores MCP que las personas pueden agregar. Claude Code bloquea cualquier servidor que no coincida con una entrada dondequiera que esté definido, incluidos servidores de complementos, servidores pasados con `--mcp-config` y servidores de claude.ai.

Los servidores integrados como Claude en Chrome, el servidor `ide` al que Claude Code se conecta en un IDE [VS Code](/docs/es/vs-code#the-built-in-ide-mcp-server) o [JetBrains](/docs/es/jetbrains#the-built-in-ide-mcp-server) en ejecución, y los servidores que la CLI misma configura están exentos de la lista de permitidos, y la lista de denegados aún se aplica a ellos. Los servidores `type: "sdk"` en proceso están exentos de ambas listas; la [aplicación que inició la sesión](/docs/es/mcp#how-connectors-reach-claude-code) los registra.

Los servidores que su organización entrega también están exentos de la lista de permitidos, y la lista de denegados aún se aplica a ellos. La exención cubre cada entrada [`managedMcpServers`](#managedmcpservers) y cualquier entrada [`managed-mcp.json`](/docs/es/managed-mcp#exclusive-control-with-managed-mcp-json) cuyos valores no usen expansión `${VAR}`. Consulte [Cómo se evalúa un servidor](/docs/es/managed-mcp#how-a-server-is-evaluated) para el orden de verificación completo. Antes de v2.1.259, los servidores de `managed-mcp.json` también tenían que coincidir.

* **Scope**: [`Any file`](#scopes). Las entradas de cada archivo se fusionan en una lista de permitidos a menos que [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly) esté configurado. Impleméntelo en la configuración administrada para aplicarlo.
* **Type**: matriz de objetos, cada uno con exactamente una clave: `serverName`, una cadena limitada a letras, números, guiones e guiones bajos; `serverCommand`, una matriz del comando y sus argumentos coincididos exactamente; o `serverUrl`, un patrón de URL con comodines `*`
* **Default**: sin establecer, por lo que se permite cada servidor; una matriz vacía bloquea cada servidor que los usuarios agregan

Este ejemplo permite solo el servidor stdio que inicia el comando `npx` listado:

```json settings.json theme={null}
{
  "allowedMcpServers": [
    { "serverCommand": ["npx", "-y", "@modelcontextprotocol/server-filesystem"] }
  ]
}
```

Una entrada [`deniedMcpServers`](#deniedmcpservers) tiene prioridad, por lo que un servidor en ambas listas se bloquea. Una vez que la lista contiene cualquier entrada `serverCommand`, un servidor stdio debe coincidir con una entrada `serverCommand`, y una vez que contiene cualquier entrada `serverUrl`, un servidor remoto debe coincidir con una entrada `serverUrl`: una coincidencia `serverName` ya no admite ese tipo de servidor. Consulte [Control basado en políticas con listas de permitidos y denegados](/docs/es/managed-mcp#policy-based-control-with-allowlists-and-denylists).

<h3 id="allowmanagedmcpserversonly">
  `allowManagedMcpServersOnly`
</h3>

Haga que la lista de permitidos administrada sea la única que se aplique. Claude Code luego lee [`allowedMcpServers`](#allowedmcpservers) solo de la configuración administrada e ignora las listas de permitidos en la configuración de usuario, proyecto y local; [`deniedMcpServers`](#deniedmcpservers) aún se fusiona desde cada ámbito de configuración, por lo que los usuarios aún pueden bloquear servidores para sí mismos. Los administradores lo configuran para que la configuración propia de un usuario no pueda ampliar lo que permite la lista de permitidos administrada.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code lee `allowedMcpServers` solo de la configuración administrada e ignora las listas de permitidos en la configuración de usuario, proyecto y local
  * `false`: las listas de permitidos de cada ámbito de configuración se fusionan
* **Default**: `false`, por lo que las listas de permitidos de cada ámbito de configuración se fusionan

Este ejemplo bloquea la lista de permitidos en la configuración administrada y permite solo el servidor denominado `github`:

```json managed-settings.json theme={null}
{
  "allowManagedMcpServersOnly": true,
  "allowedMcpServers": [
    { "serverName": "github" }
  ]
}
```

Los usuarios aún pueden agregar servidores MCP propios; solo se cargan los servidores que coinciden con la lista de permitidos administrada. Consulte [Restringir la lista de permitidos solo a la configuración administrada](/docs/es/managed-mcp#restrict-the-allowlist-to-managed-settings-only).

<h3 id="deniedmcpservers">
  `deniedMcpServers`
</h3>

Bloquee servidores MCP específicos. Claude Code se niega a cargar un servidor coincidente dondequiera que esté definido, incluidos servidores de complementos, servidores pasados con `--mcp-config`, servidores de `managed-mcp.json`, servidores de [`managedMcpServers`](#managedmcpservers) y los conectores de claude.ai [que obtiene por sí mismo](/docs/es/mcp#how-connectors-reach-claude-code). Los servidores `type: "sdk"` en proceso están exentos; la aplicación que inició la sesión los registra.

* **Scope**: [`Any file`](#scopes). Las entradas de cada archivo se fusionan en una lista de denegados, y [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly) no cambia eso. Impleméntelo en la configuración administrada para aplicarlo.
* **Type**: matriz de objetos, cada uno con exactamente una clave: `serverName`, una cadena, por lo que el nombre para mostrar de un conector de claude.ai como `"claude.ai Slack"` funciona; `serverCommand`, una matriz del comando y sus argumentos coincididos exactamente; o `serverUrl`, un patrón de URL con comodines `*`
* **Default**: sin establecer, por lo que ningún servidor se bloquea; una matriz vacía tampoco bloquea nada

```json settings.json theme={null}
{
  "deniedMcpServers": [
    { "serverName": "filesystem" }
  ]
}
```

La lista de denegados tiene prioridad sobre [`allowedMcpServers`](#allowedmcpservers), por lo que un servidor en ambas listas se bloquea. Consulte [Control basado en políticas con listas de permitidos y denegados](/docs/es/managed-mcp#policy-based-control-with-allowlists-and-denylists).

<h3 id="disableclaudeaiconnectors">
  `disableClaudeAiConnectors`
</h3>

Apague los [conectores MCP de claude.ai](/docs/es/mcp#use-mcp-servers-from-claude-ai) [que Claude Code obtiene por sí mismo](/docs/es/mcp#how-connectors-reach-claude-code), por lo que ni los obtiene ni se conecta a ellos. Un `true` en cualquier archivo de configuración se aplica: un `.claude/settings.json` de proyecto registrado puede optar por un repositorio fuera de esos conectores, pero un `false` a nivel de proyecto no puede anular un `true` a nivel de usuario o administrado.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code ni obtiene ni conecta esos conectores
  * `false`: lo mismo que sin establecer; Claude Code obtiene sus conectores a menos que otro archivo de configuración o `ENABLE_CLAUDEAI_MCP_SERVERS` los apague
* **Default**: `false`, por lo que Claude Code obtiene sus conectores
* **Per-session overrides**: [`ENABLE_CLAUDEAI_MCP_SERVERS`](/docs/es/env-vars) establecido en `false` apaga los conectores durante una sesión; cualquiera de los dos que los apague, el otro no puede volver a encenderlos

```json settings.json theme={null}
{
  "disableClaudeAiConnectors": true
}
```

Los servidores que pasa explícitamente con `--mcp-config` no se ven afectados. Para bloquear conectores individuales en lugar de todos ellos, use [`deniedMcpServers`](#deniedmcpservers). Consulte [Deshabilitar conectores de claude.ai](/docs/es/mcp#disable-claude-ai-connectors).

<h3 id="disabledmcpjsonservers">
  `disabledMcpjsonServers`
</h3>

Rechace servidores específicos definidos en el archivo `.mcp.json` de un proyecto para que Claude Code nunca se conecte a ellos ni le pida que los apruebe. Un rechazo en cualquier archivo de configuración se aplica, incluido un `.claude/settings.json` de proyecto registrado en el repositorio.

* **Scope**: [`Any file`](#scopes)
* **Type**: matriz de cadenas, los nombres de servidor tal como aparecen en `.mcp.json`
* **Default**: sin establecer

```json settings.json theme={null}
{
  "disabledMcpjsonServers": ["filesystem"]
}
```

Claude Code escribe esta clave en `.claude/settings.local.json` cuando rechaza un servidor en el diálogo de aprobación. `claude mcp get <name>` muestra un servidor rechazado como `✘ Rejected (see disabledMcpjsonServers in settings)`. El rechazo tiene prioridad sobre [`enabledMcpjsonServers`](#enabledmcpjsonservers) y [`enableAllProjectMcpServers`](#enableallprojectmcpservers).

<h3 id="enableallprojectmcpservers">
  `enableAllProjectMcpServers`
</h3>

Apruebe cada servidor MCP definido en archivos `.mcp.json` de proyecto sin un aviso. Claude Code escribe esta clave en `.claude/settings.local.json` cuando elige aprobar todos los servidores en el diálogo de aprobación.

* **Scope**: [`Any file`](#scopes). En una carpeta cuyo diálogo de confianza no ha aceptado, Claude Code lo honra desde la configuración de usuario, la configuración administrada y `--settings` e lo ignora en el archivo de proyecto compartido, tanto en la sesión como para `claude mcp list` y `claude mcp get`; [Aprobaciones de servidores de proyecto y confianza del espacio de trabajo](/docs/es/mcp#project-server-approvals-and-workspace-trust) dice cuándo cuenta también un `.claude/settings.local.json` sin seguimiento.
* **Type**: Boolean
  * `true`: Claude Code aprueba cada servidor MCP definido en archivos `.mcp.json` de proyecto sin un aviso
  * `false`: Claude Code le pide que apruebe cada servidor. En una carpeta de confianza, un `false` en un archivo de mayor precedencia anula un `true` en uno inferior; en una carpeta que no ha confiado, un `true` en cualquier archivo honrado es suficiente
* **Default**: sin establecer, por lo que Claude Code le pide que apruebe cada servidor

```json settings.json theme={null}
{
  "enableAllProjectMcpServers": true
}
```

Una entrada [`disabledMcpjsonServers`](#disabledmcpjsonservers) aún rechaza un servidor.

<h3 id="enabledmcpjsonservers">
  `enabledMcpjsonServers`
</h3>

Apruebe servidores específicos definidos en archivos `.mcp.json` de proyecto para que Claude Code se conecte a ellos sin preguntar. Claude Code escribe esta clave en `.claude/settings.local.json` cuando aprueba un servidor en el diálogo de aprobación.

* **Scope**: [`Any file`](#scopes). En una carpeta cuyo diálogo de confianza no ha aceptado, Claude Code lo honra desde la configuración de usuario, la configuración administrada y `--settings` e lo ignora en el archivo de proyecto compartido, tanto en la sesión como para `claude mcp list` y `claude mcp get`; [Aprobaciones de servidores de proyecto y confianza del espacio de trabajo](/docs/es/mcp#project-server-approvals-and-workspace-trust) dice cuándo cuenta también un `.claude/settings.local.json` sin seguimiento.
* **Type**: matriz de cadenas, los nombres de servidor tal como aparecen en `.mcp.json`
* **Default**: sin establecer

Este ejemplo aprueba los servidores `memory` y `github` del `.mcp.json` del proyecto:

```json settings.json theme={null}
{
  "enabledMcpjsonServers": ["memory", "github"]
}
```

Una entrada [`disabledMcpjsonServers`](#disabledmcpjsonservers) aún rechaza un servidor.

<h3 id="managedmcpservers">
  `managedMcpServers`
</h3>

Proporcione servidores MCP remotos a cada usuario desde la configuración administrada. Los usuarios mantienen los servidores que agregan por sí mismos y no pueden editar ni eliminar los que proporciona. Requiere Claude Code v2.1.259 o posterior.

* **Scope**: [`Managed`](#scopes). Claude Code descarta la clave con una advertencia en la configuración de usuario, proyecto y local, y no la lee en la pestaña Code de la aplicación Claude Desktop en una implementación de terceros o en las sesiones Cowork de la aplicación, donde Claude Desktop proporciona y bloquea los servidores MCP de esas sesiones.
* **Type**: objeto con clave de nombre de servidor. Cada entrada tiene la forma `.mcp.json` para un servidor `http` o `sse`: una `url` `https://` requerida y opcionalmente `headers`, `oauth` y las otras opciones HTTP y SSE. Claude Code descarta las entradas que fallan en la validación, y [Lo que una entrada puede contener](/docs/es/managed-mcp#what-an-entry-can-contain) enumera las condiciones
* **Default**: sin establecer, por lo que la configuración administrada no proporciona servidores

Este ejemplo proporciona un servidor HTTP denominado `search`:

```json managed-settings.json theme={null}
{
  "managedMcpServers": {
    "search": {
      "type": "http",
      "url": "https://search.example.com/mcp"
    }
  }
}
```

Para precedencia, cómo los servidores proporcionados se combinan con `managed-mcp.json` y las listas de permitidos y denegados, y lo que ven los usuarios, consulte [Proporcionar servidores a través de la configuración administrada](/docs/es/managed-mcp#provide-servers-through-managed-settings).

<h2 id="agents-sessions-and-worktrees">
  Agentes, sesiones y worktrees
</h2>

Establezca el agente predeterminado, controle a los compañeros de equipo y la mensajería entre sesiones, y configure worktrees. Consulte [Subagentes](/docs/es/sub-agents) y [Worktrees](/docs/es/worktrees).

<h3 id="agent">
  `agent`
</h3>

Ejecute el hilo principal como un [subagente](/docs/es/sub-agents#invoke-subagents-explicitly) nombrado, de modo que Claude Code aplique el prompt del sistema, las restricciones de herramientas y el modelo de ese subagente a su sesión. La misma clave establece el agente predeterminado para las sesiones que distribuye desde `claude agents`.

* **Alcance**: [`Cualquier archivo`](#scopes)
* **Tipo**: cadena, el nombre de un agente integrado o personalizado
* **Predeterminado**: sin establecer, por lo que el hilo principal se ejecuta como el agente predeterminado de Claude Code
* **Anulaciones por sesión**: `--agent` tiene prioridad sobre esta clave para una sesión

```json settings.json theme={null}
{
  "agent": "code-reviewer"
}
```

El propio `settings.json` de un plugin también puede proporcionar esta clave; consulte [Envíe configuración predeterminada con su plugin](/docs/es/plugins/components#default-settings).

<h3 id="crosssessioninbound">
  `crossSessionInbound`
</h3>

Elija qué hace esta sesión con [mensajes que llegan desde sus otras sesiones de Claude Code](/docs/es/cross-session-messaging#control-inbound-messages). Cuando no se aplica ningún valor, Claude Code decide por mensaje según las clases de modo de permisos de las dos sesiones. Requiere Claude Code v2.1.224 o posterior.

* **Alcance**: [`Cualquier archivo`](#scopes). Un valor de proyecto o local se aplica solo cuando es más estricto que el valor de configuración administrada, la bandera `--settings` o la configuración del usuario proporcionan.
* **Tipo**: cadena, una de:
  * `"accept"`: Claude Code entrega el mensaje a Claude
  * `"hold"`: Claude Code muestra un aviso para el mensaje sin entregarlo
  * `"refuse"`: Claude Code descarta el mensaje
* **Predeterminado**: sin establecer, por lo que Claude Code decide por mensaje

```json settings.json theme={null}
{
  "crossSessionInbound": "hold"
}
```

Claude Code lee primero la configuración administrada, luego la bandera `--settings`, luego la configuración del usuario, y aplica el primer valor encontrado. `refuse` es más estricto que `hold`, y `hold` es más estricto que `accept`. Cuando ninguna de las fuentes confiables establece un valor, un `hold` o `refuse` de proyecto o local aún se aplica, reemplazando el predeterminado por mensaje. En sesiones con mensajería entre sesiones, esta clave aparece en `/config` como **Mensajes de sus otras sesiones**, que la escribe en la configuración del usuario; la fila requiere Claude Code v2.1.232 o posterior, y Claude Code la oculta mientras la bandera `--settings` o la configuración administrada establezcan la clave.

Claude Code [advierte](/docs/es/errors#crosssessioninbound-must-be-one-of-accept-hold-refuse) cuando establece un valor que no reconoce. Mientras ese valor esté presente en un archivo de usuario, proyecto, local o `--settings`, Claude Code retiene los mensajes entrantes, incluso cuando una fuente que tiene prioridad establece `accept`. Un `refuse` que otra fuente establece aún se aplica. Corrija o elimine el valor para borrar la retención.

Cuando el valor no reconocido está en [configuración administrada](/docs/es/managed-settings), Claude Code en su lugar lo trata como `refuse` hasta que un administrador lo corrija. Antes de v2.1.248, Claude Code ignoraba un valor no reconocido sin advertencia.

<h3 id="disableagentview">
  `disableAgentView`
</h3>

Desactive [agentes de fondo y vista de agentes](/docs/es/agent-view): `claude agents`, `--bg`, `/background` y el supervisor bajo demanda. Establézcalo en [configuración administrada](/docs/es/managed-settings) para aplicarlo en una organización.

* **Alcance**: [`Cualquier archivo`](#scopes)
* **Tipo**: Booleano
  * `true`: Claude Code desactiva `claude agents`, `--bg`, `/background` y el supervisor bajo demanda
  * `false`: la vista de agentes está disponible
* **Predeterminado**: sin establecer, por lo que la vista de agentes está disponible
* **Anulaciones por sesión**: [`CLAUDE_CODE_DISABLE_AGENT_VIEW`](/docs/es/env-vars) desactiva la vista de agentes para una sesión; cualquiera de los dos que la desactive, el otro no puede volver a activarla

```json settings.json theme={null}
{
  "disableAgentView": true
}
```

<h3 id="isolatepeermachines">
  `isolatePeerMachines`
</h3>

Requiera su aprobación explícita antes de que `SendMessage` de Claude llegue a una de sus sesiones más allá de esta máquina; consulte [Requiera aprobación para mensajes entre máquinas](/docs/es/cross-session-messaging#require-approval-for-cross-machine-messages). El aviso de aprobación aparece incluso en [modo `bypassPermissions`](/docs/es/permission-modes#skip-all-checks-with-bypasspermissions-mode).

* **Alcance**: [`Cualquier archivo`](#scopes). Un `true` de cualquier alcance se aplica, por lo que un archivo de proyecto registrado puede activar el requisito pero no desactivarlo.
* **Tipo**: Booleano
  * `true`: Claude Code solicita su aprobación antes de que `SendMessage` de Claude llegue a una de sus sesiones más allá de esta máquina
  * `false`: los mensajes entre máquinas no generan aviso
* **Predeterminado**: sin establecer, por lo que los mensajes entre máquinas no generan aviso

```json settings.json theme={null}
{
  "isolatePeerMachines": true
}
```

La aprobación de `SendMessage` entre máquinas requiere Claude Code v2.1.224 o posterior.

<h3 id="processwrapper">
  `processWrapper`
</h3>

En macOS y Linux, coloque un comando de iniciador corporativo delante de los [procesos de fondo que inicia Claude Code](/docs/es/corporate-launcher#what-the-launcher-covers). Claude Code ejecuta el iniciador con su propia línea de comandos anexada, por lo que el iniciador debe ejecutarse en Claude Code; consulte [Ejecute Claude Code detrás de un iniciador corporativo](/docs/es/corporate-launcher) para el contrato del iniciador. Requiere Claude Code v2.1.210 o posterior.

* **Alcance**: [`Usuario o administrado`](#scopes)
* **Tipo**: cadena, el comando del iniciador como prefijo argv, como una ruta absoluta con argumentos opcionales
* **Predeterminado**: sin establecer, por lo que los procesos de fondo se inician sin envolver
* **Anulaciones por sesión**: [`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/es/env-vars) tiene prioridad sobre esta clave para una sesión

```json settings.json theme={null}
{
  "processWrapper": "/opt/corp/launcher --profile claude"
}
```

Claude Code ignora el iniciador en Windows e inicia cada proceso sin envolver. Requiere Claude Code v2.1.210 o posterior.

<h3 id="teammatemode">
  `teammateMode`
</h3>

Elija dónde Claude Code muestra los compañeros de equipo del [equipo de agentes](/docs/es/agent-teams): dentro de su panel de terminal principal, o en paneles divididos cuando su terminal los admita. Consulte [Elija un modo de visualización](/docs/es/agent-teams#choose-a-display-mode).

* **Alcance**: [`Cualquier archivo`](#scopes). Claude Code también lee un valor dejado en `~/.claude.json` por versiones anteriores.
* **Tipo**: cadena, una de:
  * `"in-process"`: los compañeros de equipo se ejecutan dentro de su panel de terminal principal
  * `"auto"`: paneles divididos cuando se ejecuta dentro de tmux, o dentro de iTerm2 con `it2` en su `PATH` o tmux instalado; en proceso de lo contrario
  * `"tmux"`: paneles divididos usando tmux o iTerm2, detectados desde su terminal
  * `"iterm2"`: paneles divididos nativos de iTerm2 a través de la CLI `it2`
* **Predeterminado**: `"in-process"`
* **Anulaciones por sesión**: `--teammate-mode` tiene prioridad sobre esta clave para una sesión

```json settings.json theme={null}
{
  "teammateMode": "auto"
}
```

<span id="worktree-settings" />

<h3 id="worktree">
  `worktree`
</h3>

Configure cómo Claude Code crea y administra [git worktrees](/docs/es/worktrees) para `--worktree`, la herramienta `EnterWorktree` y subagentes aislados y sesiones de fondo.

* **Alcance**: [`Cualquier archivo`](#scopes)
* **Tipo**: objeto con `baseRef`, `symlinkDirectories`, `sparsePaths` y `bgIsolation`
* **Predeterminado**: sin establecer

Este ejemplo ramifica nuevos worktrees desde su `HEAD` actual y crea enlaces simbólicos de `node_modules` en cada uno:

```json settings.json theme={null}
{
  "worktree": {
    "baseRef": "head",
    "symlinkDirectories": ["node_modules"]
  }
}
```

Para copiar archivos ignorados por git como `.env` en nuevos worktrees, agregue un [archivo `.worktreeinclude`](/docs/es/worktrees#copy-gitignored-files-into-worktrees) a la raíz de su proyecto en lugar de una configuración.

<h3 id="worktree-baseref">
  `worktree.baseRef`
</h3>

Elija desde qué ref se ramifican los nuevos worktrees. `"fresh"` se ramifica desde `origin/<default-branch>` para un árbol limpio que coincida con el remoto; `"head"` se ramifica desde su `HEAD` local actual, por lo que los commits no enviados y el estado de la rama de características están presentes en el worktree.

* **Alcance**: [`Cualquier archivo`](#scopes)
* **Tipo**: cadena, una de:
  * `"fresh"`: los nuevos worktrees se ramifican desde `origin/<default-branch>`
  * `"head"`: los nuevos worktrees se ramifican desde su `HEAD` local actual, incluidos los commits no enviados
* **Predeterminado**: `"fresh"`

```json settings.json theme={null}
{
  "worktree": {
    "baseRef": "head"
  }
}
```

Dentro de un worktree vinculado, `"head"` se resuelve en el `HEAD` de ese worktree, no en el de la extracción principal.

<h3 id="worktree-symlinkdirectories">
  `worktree.symlinkDirectories`
</h3>

Cree enlaces simbólicos de directorios desde el repositorio principal en cada worktree para que no duplique directorios grandes en el disco.

* **Alcance**: [`Cualquier archivo`](#scopes)
* **Tipo**: matriz de cadenas, rutas de directorio relativas a la raíz del repositorio
* **Predeterminado**: sin establecer, por lo que Claude Code no crea enlaces simbólicos de directorios

Este ejemplo crea enlaces simbólicos de `node_modules` y `.cache` desde el repositorio principal en cada nuevo worktree:

```json settings.json theme={null}
{
  "worktree": {
    "symlinkDirectories": ["node_modules", ".cache"]
  }
}
```

<h3 id="worktree-sparsepaths">
  `worktree.sparsePaths`
</h3>

Extraiga solo los directorios listados en cada worktree a través de git sparse-checkout. Claude Code escribe solo esos directorios más archivos de nivel raíz en el disco, lo que es más rápido en monorepos grandes; consulte [Extraiga solo los directorios que necesita](/docs/es/large-codebases#check-out-only-the-directories-you-need).

* **Alcance**: [`Cualquier archivo`](#scopes)
* **Tipo**: matriz de cadenas, rutas de directorio relativas a la raíz del repositorio
* **Predeterminado**: sin establecer, por lo que cada worktree extrae el árbol completo

Este ejemplo extrae solo `packages/my-app` y `shared/utils`, más archivos de nivel raíz, en cada worktree:

```json settings.json theme={null}
{
  "worktree": {
    "sparsePaths": ["packages/my-app", "shared/utils"]
  }
}
```

Mientras existe un worktree disperso, git habilita `extensions.worktreeConfig` en el `.git/config` compartido del repositorio.

<h3 id="worktree-bgisolation">
  `worktree.bgIsolation`
</h3>

Elija cómo [las sesiones de fondo](/docs/es/agent-view#how-file-edits-are-isolated) aíslan sus ediciones de archivos. Con `"worktree"`, Claude Code bloquea `Edit` y `Write` en la extracción principal hasta que la sesión llame a `EnterWorktree`; con `"none"`, los trabajos de fondo editan la copia de trabajo directamente. Establezca `"none"` para un repositorio donde los git worktrees no son prácticos.

* **Alcance**: [`Cualquier archivo`](#scopes)
* **Tipo**: cadena, una de:
  * `"worktree"`: Claude Code bloquea `Edit` y `Write` en la extracción principal hasta que la sesión llame a `EnterWorktree`
  * `"none"`: los trabajos de fondo editan la copia de trabajo directamente
* **Predeterminado**: `"worktree"`

```json settings.json theme={null}
{
  "worktree": {
    "bgIsolation": "none"
  }
}
```

Fuera de un repositorio git, un [hook `WorktreeCreate`](/docs/es/worktrees#non-git-version-control) que falla libera el bloqueo para que la sesión pueda editar el directorio de trabajo en su lugar; esa liberación requiere Claude Code v2.1.203 o posterior.

<h2 id="remote-desktop-and-notifications">
  Control remoto, escritorio y notificaciones
</h2>

Configure el Control remoto, los entornos en la nube, la aplicación de escritorio y las notificaciones que Claude Code envía cuando lo necesita. Consulte [Control remoto](/docs/es/remote-control).

<h3 id="agentpushnotifenabled">
  `agentPushNotifEnabled`
</h3>

Permita que Claude envíe una notificación push a su teléfono cuando decida que vale la pena enviarla, por ejemplo cuando finaliza una tarea larga. Claude Code sincroniza esta opción con su cuenta, y las notificaciones llegan mientras [Control remoto](/docs/es/remote-control) está conectado. Aparece en `/config` como **Enviar notificación cuando Claude lo decida**.

* **Scope**: [`Any file`](#scopes). Claude Code también lee un valor dejado en `~/.claude.json` por versiones anteriores.
* **Type**: Boolean
  * `true`: Claude puede enviar una notificación push a su teléfono cuando decida que vale la pena enviarla
  * `false`: Claude no envía esas notificaciones
* **Default**: `false`

```json settings.json theme={null}
{
  "agentPushNotifEnabled": true
}
```

Consulte [Notificaciones push móviles](/docs/es/remote-control#mobile-push-notifications).

<h3 id="awaysummaryenabled">
  `awaySummaryEnabled`
</h3>

Muestre un resumen de sesión de una línea cuando regrese a la terminal después de estar ausente unos minutos. Establézcalo en `false`, o desactive **Resumen de sesión** en `/config`, para detener el resumen.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: ve un resumen de sesión de una línea cuando regresa después de estar ausente unos minutos
  * `false`: Claude Code no muestra ningún resumen
* **Default**: unset, por lo que el resumen está activado
* **Per-session overrides**: [`CLAUDE_CODE_ENABLE_AWAY_SUMMARY`](/docs/es/env-vars) tiene prioridad sobre esta clave para una sesión, en cualquier dirección

```json settings.json theme={null}
{
  "awaySummaryEnabled": false
}
```

Claude Code nunca muestra el resumen en modo no interactivo.

<h3 id="disableartifact">
  `disableArtifact`
</h3>

<Warning>
  Deprecated, y reemplazado por [`enableArtifact`](#enableartifact). Claude Code aún honra `disableArtifact: true` como equivalente a `enableArtifact: false`, e ignora `disableArtifact: false`.
</Warning>

Use [`enableArtifact`](#enableartifact) en su lugar para desactivar la herramienta [Artifact](/docs/es/artifacts), que publica la salida de la sesión como una página web privada en claude.ai. Cuando desactiva la fila **Artifacts** en `/config`, Claude Code escribe `enableArtifact` en su configuración de usuario y borra esta clave.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code desactiva la herramienta Artifact para cada sesión a la que se aplique el archivo, y ningún otro archivo la vuelve a activar. Antes de v2.1.242, un archivo de mayor precedencia podría anular un `true` de un archivo de menor precedencia en lugar de que la clave actúe como un bloqueo
  * `false`: ignorado; para dejar la herramienta activada, elimine la clave
* **Default**: unset, por lo que la herramienta sigue la [disponibilidad](/docs/es/artifacts#availability) de su cuenta
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_ARTIFACT`](/docs/es/env-vars) establecido en `1` desactiva la herramienta para una sesión

```json settings.json theme={null}
{
  "disableArtifact": true
}
```

[Desactivar artefactos](/docs/es/artifacts#disable-artifacts) enumera todas las formas de desactivar la herramienta.

<h3 id="disabledeeplinkregistration">
  `disableDeepLinkRegistration`
</h3>

Impida que Claude Code registre el controlador del protocolo `claude-cli://` con el sistema operativo, que de otro modo hace después de enviar el primer prompt de una sesión interactiva. Los [enlaces profundos](/docs/es/deep-links) permiten que herramientas externas abran una sesión de Claude Code con un prompt rellenado previamente. Establézcalo en entornos donde el registro del controlador de protocolo está restringido o se gestiona por separado.

* **Scope**: [`Any file`](#scopes)
* **Type**: la cadena `"disable"`
* **Default**: unset, por lo que Claude Code registra el controlador

```json settings.json theme={null}
{
  "disableDeepLinkRegistration": "disable"
}
```

<h3 id="disabledesktoplocalsessions">
  `disableDesktopLocalSessions`
</h3>

Desactive las sesiones de Code que se ejecutan en el dispositivo en la [aplicación de escritorio](/docs/es/desktop#local-sessions-on-managed-devices), para implementaciones donde los desarrolladores deben trabajar en máquinas remotas a través de SSH. En la pestaña Code, el entorno **Local** permanece en el menú desplegable de entornos pero está atenuado y no se puede seleccionar, con una información sobre herramientas que dice que su organización lo desactivó; en Windows, la entrada WSL está atenuada de la misma manera, aunque si las sesiones WSL se ejecutan en un dispositivo administrado o no se [rige por separado](/docs/es/admin-setup#wsl-sessions-in-claude-code-desktop). Las nuevas sesiones tienen como valor predeterminado la primera [conexión SSH](/docs/es/desktop#ssh-sessions) si una está configurada, y la aplicación se niega a iniciar o reanudar una sesión en el dispositivo, incluida una conexión SSH de vuelta a la misma máquina. Las sesiones SSH a otros hosts y las sesiones en la nube no se ven afectadas. La aplicación de escritorio lee esta clave; la CLI del terminal la ignora. Requiere Claude Desktop v1.37937.0 o posterior.

* **Scope**: [`Managed`](#scopes)
* **Type**: Boolean; solo el Boolean JSON `true` tiene efecto
  * `true`: la aplicación de escritorio no ofrece sesiones de Code en el dispositivo; las sesiones locales existentes permanecen listadas pero no pueden continuar
  * `false`: las sesiones locales permanecen disponibles
* **Default**: unset, por lo que las sesiones locales están disponibles

```json managed-settings.json theme={null}
{
  "disableDesktopLocalSessions": true
}
```

La aplicación de escritorio ignora cualquier otro valor, y un valor que no sea un Boolean, como la cadena `"true"` o `1`, también registra una advertencia. Emparéjelo con [`sshConfigs`](#sshconfigs) para que los usuarios lleguen a una conexión que funcione, y con [`sshHostAllowlist`](#sshhostallowlist) para limitar a qué hosts pueden acceder. Consulte [Sesiones locales en dispositivos administrados](/docs/es/desktop#local-sessions-on-managed-devices).

Claude Desktop proporciona sesiones de Code con política derivada de su configuración de escritorio, por ejemplo la lista de permitidos de salida, el sandbox del sistema de archivos y las restricciones de MCP en implementaciones de terceros. Claude Code ignora esa configuración principal siempre que haya una [fuente de administrador](/docs/es/managed-settings#how-claude-code-combines-managed-sources): configuración administrada por servidor, una política de MDM o a nivel del SO, o un archivo de configuración administrada. Implementar esta clave a través de una de esas en un dispositivo que no tenía ninguna antes, como en implementaciones de terceros, por lo tanto detiene la aplicación de las políticas derivadas del escritorio. [Permitir que un host de inserción agregue política](/docs/es/managed-settings#let-an-embedding-host-add-policy) cubre cuándo la configuración principal aún puede fusionarse; esto se aplica a cualquier clave que implemente de esa manera, no solo a esta.

<h3 id="disableremotecontrol">
  `disableRemoteControl`
</h3>

Desactive [Control remoto](/docs/es/remote-control): Claude Code entonces rechaza `claude remote-control`, la bandera `--remote-control`, el inicio automático y el conmutador en sesión, e informa que la política de su organización lo desactivó. Colóquelo en [configuración administrada](/docs/es/managed-settings) para la aplicación de políticas de MDM por dispositivo.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code rechaza `claude remote-control`, la bandera `--remote-control`, el inicio automático y el conmutador en sesión
  * `false`: Control remoto permanece disponible
* **Default**: `false`

```json settings.json theme={null}
{
  "disableRemoteControl": true
}
```

<h3 id="enableartifact">
  `enableArtifact`
</h3>

Desactive la herramienta [Artifact](/docs/es/artifacts), que publica la salida de la sesión como una página web privada en claude.ai. Cuando desactiva la fila **Artifacts** en `/config`, Claude Code escribe esta clave en su configuración de usuario, por lo que normalmente no la edita a mano. Requiere Claude Code v2.1.196 o posterior.

* **Scope**: [`Any file`](#scopes). Cada archivo puede desactivar la herramienta, y ninguno puede volver a activarla.
* **Type**: Boolean
  * `false`: Claude Code desactiva la herramienta Artifact para cada sesión a la que se aplique el archivo
  * `true`: lo mismo que dejar la clave sin establecer, porque nunca anula un `false` de otro archivo, de [`CLAUDE_CODE_DISABLE_ARTIFACT`](/docs/es/env-vars), o de la [configuración de administrador](/docs/es/artifacts#manage-artifacts-for-your-organization) de su organización
* **Default**: unset, por lo que la herramienta sigue la [disponibilidad](/docs/es/artifacts#availability) de su cuenta

```json settings.json theme={null}
{
  "enableArtifact": false
}
```

Mientras una fuente que no sea su propia configuración de usuario mantiene la herramienta desactivada, Claude Code oculta la fila **Artifacts** en `/config`, porque activarla allí no cambiaría nada. [Desactivar artefactos](/docs/es/artifacts#disable-artifacts) enumera todas las formas de desactivar la herramienta. Antes de v2.1.242, Claude Code ignoraba esta clave en la configuración de proyecto y local, y un archivo más alto en la [pila de precedencia](/docs/es/settings#settings-precedence) podría volver a activar la herramienta sobre un `false` de un archivo más bajo.

<h3 id="inputneedednotifenabled">
  `inputNeededNotifEnabled`
</h3>

Obtenga una notificación push en su teléfono cuando un prompt de permiso o una pregunta esté esperando su entrada. Claude Code envía estas solo mientras [Control remoto](/docs/es/remote-control) está conectado. Aparece en `/config` como **Enviar notificación cuando se requieran acciones**.

* **Scope**: [`Any file`](#scopes). Claude Code también lee un valor dejado en `~/.claude.json` por versiones anteriores.
* **Type**: Boolean
  * `true`: obtiene una notificación push en su teléfono cuando un prompt de permiso o una pregunta está esperando, mientras Control remoto está conectado
  * `false`: Claude Code no envía tales notificaciones
* **Default**: `false`

```json settings.json theme={null}
{
  "inputNeededNotifEnabled": true
}
```

Consulte [Notificaciones push móviles](/docs/es/remote-control#mobile-push-notifications).

<h3 id="preferrednotifchannel">
  `preferredNotifChannel`
</h3>

Elija cómo Claude Code lo notifica cuando una tarea se completa o un prompt de permiso está esperando. Aparece en `/config` como **Notificaciones locales**.

* **Scope**: [`Any file`](#scopes). Claude Code también lee un valor dejado en `~/.claude.json` por versiones anteriores.
* **Type**: cadena, una de:
  * `"auto"`: Claude Code envía una notificación de escritorio en iTerm2, Ghostty y Kitty, suena la campana en Terminal.app solo cuando su campana audible está desactivada, y no hace nada en otros lugares
  * `"terminal_bell"`: Claude Code suena el carácter de campana en cualquier terminal
  * `"iterm2"`: Claude Code envía una notificación de escritorio de iTerm2
  * `"iterm2_with_bell"`: Claude Code envía una notificación de escritorio de iTerm2 y suena la campana
  * `"kitty"`: Claude Code envía una notificación de escritorio de Kitty
  * `"ghostty"`: Claude Code envía una notificación de escritorio de Ghostty
  * `"notifications_disabled"`: Claude Code no envía notificación
* **Default**: `"auto"`

```json settings.json theme={null}
{
  "preferredNotifChannel": "terminal_bell"
}
```

Con `"auto"`, Claude Code envía una notificación de escritorio en iTerm2, Ghostty y Kitty. En Terminal.app suena el carácter de campana solo cuando ha desactivado la campana audible de Terminal, y en otros terminales no hace nada. Establezca `"terminal_bell"` para sonar el carácter de campana en cualquier terminal. Consulte [Obtener una campana de terminal o notificación](/docs/es/terminal-config#get-a-terminal-bell-or-notification).

<h3 id="remote-defaultenvironmentid">
  `remote.defaultEnvironmentId`
</h3>

Elija el [entorno en la nube](/docs/es/cloud-environments) predeterminado para las sesiones en la nube que crea desde la CLI, como con `claude --cloud`. Claude Code escribe esta clave en su configuración de usuario cuando elige un entorno con [`/remote-env`](/docs/es/cloud-environments#select-an-environment-from-the-cli).

* **Scope**: [`Any file`](#scopes). Para un ID de entorno autohospedado, configuración de usuario o administrada, o la bandera `--settings` solo.
* **Type**: cadena, un ID de entorno como `env_...` o `ccpool_...`
* **Default**: unset, por lo que Claude Code usa el entorno alojado por Anthropic cuando su lista tiene uno, y de lo contrario el primer entorno en su lista que no sea un [entorno puente de Control remoto](/docs/es/cloud-environments#the-default-environment), o el primer entorno cuando todos son entornos puente
* **Per-session overrides**: `--environment` tiene prioridad sobre esta clave para la sesión en la nube que crea

```json settings.json theme={null}
{
  "remote": {
    "defaultEnvironmentId": "env_0123abcd"
  }
}
```

Un ID de entorno alojado por Anthropic, que comienza con `env_`, sigue la precedencia de configuración estándar, por lo que un valor en la configuración del proyecto de un repositorio anula su selección a nivel de usuario. Un ID de [entorno autohospedado](/docs/es/self-hosted-environments), que comienza con `ccpool_`, se honra solo desde la configuración de usuario, la configuración administrada y la bandera `--settings`; Claude Code ignora uno en la configuración de proyecto o local de un repositorio, y `/remote-env` muestra qué valor ignoró, por lo que un archivo registrado no puede dirigir sesiones a un entorno autohospedado que no eligió.

<h3 id="remotecontrolatstartup">
  `remoteControlAtStartup`
</h3>

Conecte [Control remoto](/docs/es/remote-control) automáticamente cuando cada sesión interactiva comienza, en lugar de esperar `/remote-control`. Establézcalo en `true` para activar la conexión automática, `false` para desactivarla. Aparece en `/config` como **Habilitar Control remoto para todas las sesiones**.

* **Scope**: [`Any file`](#scopes). Claude Code también lee un valor dejado en `~/.claude.json` por versiones anteriores.
* **Type**: Boolean
  * `true`: Claude Code conecta Control remoto automáticamente cuando cada sesión interactiva comienza
  * `false`: Claude Code espera `/remote-control`
* **Default**: unset, por lo que la conexión automática sigue el valor predeterminado de administrador de su organización cuando uno está establecido, y de lo contrario el valor predeterminado actual de Claude Code
* **Per-session overrides**: `--remote-control` activa Control remoto para una sesión incluso cuando esta clave es `false`, y ninguna bandera la desactiva para una sesión

```json settings.json theme={null}
{
  "remoteControlAtStartup": true
}
```

Claude Code ignora un `true` de la configuración de proyecto o local, por lo que un repositorio puede desactivar la conexión automática para su checkout pero no puede activarla. Para el comportamiento completo por scope, consulte [Habilitar Control remoto para todas las sesiones](/docs/es/remote-control#enable-remote-control-for-all-sessions) y las [claves de seguridad donde se aplica el valor más estricto](/docs/es/settings#security-keys-where-the-stricter-value-applies).

<h3 id="sshconfigs">
  `sshConfigs`
</h3>

Agregue conexiones SSH al menú desplegable del entorno [Desktop](/docs/es/desktop#pre-configure-ssh-connections-for-your-team). Los administradores lo usan para distribuir conexiones compartidas a un equipo. Las conexiones que define en la configuración administrada se muestran como administradas, por lo que los usuarios pueden seleccionarlas pero no pueden editarlas ni eliminarlas en la aplicación.

* **Scope**: [`User or managed`](#scopes). La aplicación de escritorio lee esta clave.
* **Type**: matriz de objetos, cada uno con `id`, `name` y `sshHost` requeridos y `sshPort` y `sshIdentityFile` opcionales
* **Default**: unset

Este ejemplo agrega una conexión llamada `Dev VM` que se conecta a `user@dev.example.com`:

```json settings.json theme={null}
{
  "sshConfigs": [
    {
      "id": "dev-vm",
      "name": "Dev VM",
      "sshHost": "user@dev.example.com"
    }
  ]
}
```

<h3 id="sshhostallowlist">
  `sshHostAllowlist`
</h3>

Limite los hosts a los que una [sesión SSH de Desktop](/docs/es/desktop#restrict-which-ssh-hosts-users-can-connect-to) puede conectarse. Solo la aplicación de escritorio lee esta clave; la CLI no. Los patrones no distinguen mayúsculas de minúsculas: `*` coincide con cualquier host, `*.example.com` coincide con `example.com` y cada subdominio, y cualquier otra cosa es una coincidencia exacta contra el nombre de host después de la resolución de `~/.ssh/config`. Una matriz vacía desactiva las sesiones SSH.

* **Scope**: [`Managed`](#scopes)
* **Type**: matriz de patrones de nombre de host
* **Default**: unset, por lo que se permite cualquier host

Este ejemplo permite `devboxes.example.com` y sus subdominios, más el host exacto `bastion.example.com`:

```json managed-settings.json theme={null}
{
  "sshHostAllowlist": ["*.devboxes.example.com", "bastion.example.com"]
}
```

<span id="authentication-and-login" />

<h2 id="authentication-and-providers">
  Autenticación y proveedores
</h2>

Proporcione credenciales a través de scripts auxiliares y, para organizaciones, fuerce un método de inicio de sesión u organización. Consulte [Autenticación](/docs/es/authentication).

<h3 id="apikeyhelper">
  `apiKeyHelper`
</h3>

Ejecute su propio comando para producir la credencial que Claude Code envía con solicitudes de modelo. Claude Code ejecuta el comando a través del shell del sistema, `/bin/sh` en macOS y Linux y `cmd` en Windows, y envía su salida como encabezados `X-Api-Key` y `Authorization: Bearer`. Úselo para credenciales dinámicas o rotativas, como tokens de corta duración obtenidos de un almacén.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, una línea de comando del shell
* **Default**: sin establecer, por lo que Claude Code no ejecuta un auxiliar

```json settings.json theme={null}
{
  "apiKeyHelper": "/bin/generate_temp_api_key.sh"
}
```

Claude Code almacena en caché el valor y vuelve a ejecutar el comando en estos casos:

* Después de la duración del caché, cinco minutos por defecto o el intervalo que establezca con [`CLAUDE_CODE_API_KEY_HELPER_TTL_MS`](/docs/es/env-vars).
* Cuando una solicitud a la API de Anthropic, directamente o a través de una [puerta de enlace LLM](/docs/es/llm-gateway), falla con `401` o `403`.
* Antes de enviar una solicitud a la API de Anthropic, directamente o a través de una puerta de enlace LLM, cuando la salida almacenada en caché es un JWT que expiró después de que el auxiliar lo produjo. Requiere Claude Code v2.1.246 o posterior.

Los dos últimos casos se aplican solo cuando la salida del auxiliar es la credencial que Claude Code envía y `ANTHROPIC_AUTH_TOKEN` no está establecido.

En sesiones interactivas, cuando el comando proviene de la configuración del proyecto o local, Claude Code no lo ejecuta hasta que acepte el mensaje de confianza del espacio de trabajo. Consulte [Gestión de credenciales](/docs/es/authentication#credential-management).

<h3 id="awsauthrefresh">
  `awsAuthRefresh`
</h3>

Ejecute su propio comando, como `aws sso login`, para actualizar las credenciales en su directorio `.aws` cuando las que Claude Code tiene para [Amazon Bedrock](/docs/es/amazon-bedrock) dejen de funcionar. Claude Code verifica las credenciales actuales contra STS primero y ejecuta el comando solo cuando esa verificación falla, luego lee el directorio `.aws` actualizado.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, una línea de comando del shell
* **Default**: sin establecer, por lo que Claude Code no actualiza las credenciales de AWS para usted

```json settings.json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile myprofile"
}
```

Use esta clave cuando su flujo de actualización escriba en `.aws`; use [`awsCredentialExport`](#awscredentialexport) cuando imprima credenciales en su lugar. Consulte [configuración avanzada de credenciales](/docs/es/amazon-bedrock#advanced-credential-configuration).

<h3 id="awscredentialexport">
  `awsCredentialExport`
</h3>

Ejecute su propio comando que imprima credenciales de AWS como JSON, para que Claude Code pueda llamar a [Amazon Bedrock](/docs/es/amazon-bedrock) con credenciales que no viven en su directorio `.aws`. Claude Code acepta la forma de salida de `aws sts` y la forma plana de `aws configure export-credentials`, y limita las credenciales a su propio cliente de Bedrock, por lo que los comandos del shell que Claude ejecuta aún ven sus credenciales ambientales.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, una línea de comando del shell
* **Default**: sin establecer, por lo que Claude Code usa la cadena de credenciales de AWS ambiental

```json settings.json theme={null}
{
  "awsCredentialExport": "/bin/generate_aws_grant.sh"
}
```

A diferencia de [`awsAuthRefresh`](#awsauthrefresh), Claude Code siempre ejecuta este comando cuando está establecido, sin verificar primero las credenciales ambientales. Consulte [configuración avanzada de credenciales](/docs/es/amazon-bedrock#advanced-credential-configuration).

<h3 id="forceloginmethod">
  `forceLoginMethod`
</h3>

Restrinja qué tipo de cuenta pueden usar las personas para iniciar sesión. Establezca `"claudeai"` para permitir solo cuentas de claude.ai, `"console"` para permitir solo cuentas de Claude Console, o `"gateway"` para enviar a las personas a una [puerta de enlace en la nube](/docs/es/claude-apps-gateway) en lugar de un inicio de sesión de primera parte. Los administradores lo establecen en la configuración administrada y lo emparejan con [`forceLoginOrgUUID`](#forceloginorguuid) para mantener los inicios de sesión de claude.ai de los desarrolladores dentro de una organización. Si lo establece en `"claudeai"` o `"console"` en cualquier archivo de configuración, Claude Code también deja de ofrecer el [inicio de sesión sin clave de Console](/docs/es/authentication#sign-in-without-an-api-key) en las sesiones a las que se aplica ese archivo.

* **Scope**: [`Any file`](#scopes). Claude Code respeta `"gateway"` solo desde una fuente administrada en la máquina: `managed-settings.json`, la lista de propiedades de macOS o el registro HKLM de Windows, o un auxiliar de política. Trata `"gateway"` como sin establecer en la configuración de usuario, proyecto, local, HKCU y administrada por servidor, la misma regla que [`forceLoginGatewayUrl`](#forcelogingatewayurl).
* **Type**: string, uno de:
  * `"claudeai"`: solo las cuentas de claude.ai pueden iniciar sesión
  * `"console"`: solo las cuentas de Claude Console pueden iniciar sesión
  * `"gateway"`: Claude Code envía a las personas a una puerta de enlace en la nube en lugar de un inicio de sesión de primera parte
* **Default**: sin establecer, por lo que las personas eligen un método de inicio de sesión

```json settings.json theme={null}
{
  "forceLoginMethod": "claudeai"
}
```

Cada ruta de inicio de sesión de primera parte aplica la restricción, incluida la [extensión de VS Code](/docs/es/vs-code), el SDK del Agente, `claude setup-token`, e `/install-github-app`, excepto la pantalla de inicio de sesión interactivo del terminal, a la que se accede mediante `/login` u onboarding de primera ejecución, que preselecciona el método sin aplicarlo. Antes de v2.1.212, solo los inicios de sesión del terminal lo aplicaban. Consulte [Restringir el inicio de sesión a su organización](/docs/es/authentication#restrict-login-to-your-organization) para ver cómo se manejan cada ruta de inicio de sesión, credenciales ambientales y proveedores de terceros.

Cuando una fuente administrada en la máquina establece `"gateway"`, Claude Code no usa un inicio de sesión restante, clave de API o credencial de `apiKeyHelper`. Consulte [La política del administrador requiere un inicio de sesión de puerta de enlace en la nube](/docs/es/errors#administrator-policy-requires-a-cloud-gateway-sign-in) para el mensaje que produce cada uno. Si selecciona un proveedor en la nube a través de `CLAUDE_CODE_USE_BEDROCK` o una variable de entorno similar, la sesión no necesita el inicio de sesión de la puerta de enlace. Antes de v2.1.261, Claude Code usaba un inicio de sesión restante en estas máquinas.

<h3 id="forcelogingatewayurl">
  `forceLoginGatewayUrl`
</h3>

Establezca la URL de la puerta de enlace a la que se conecta la pantalla `/login` de puerta de enlace en la nube, para que las personas lleguen a su [puerta de enlace en la nube](/docs/es/claude-apps-gateway) sin escribir su dirección. La pantalla no tiene un campo de URL: con esta clave establecida, muestra la URL de su puerta de enlace y se conecta cuando la persona presiona Intro; sin ella, les dice que se comuniquen con su administrador de TI.

Cualquiera de estas dos claves o `forceLoginMethod: "gateway"` hace que la máquina sea solo de puerta de enlace, por lo que `/login` se abre en la pantalla de puerta de enlace en la nube sin un selector de método de inicio de sesión. Consulte [La política del administrador requiere un inicio de sesión de puerta de enlace en la nube](/docs/es/errors#administrator-policy-requires-a-cloud-gateway-sign-in) para ver qué sucede con un inicio de sesión de primera parte restante o una clave de API. Establezca ambas claves para que la pantalla se conecte en lugar de mostrar un error.

* **Scope**: [`Managed`](#scopes). Lea solo desde una fuente en la máquina: `managed-settings.json`, la lista de propiedades de macOS o el registro HKLM de Windows, o un auxiliar de política. Claude Code lo ignora en la configuración de HKCU y administrada por servidor.
* **Type**: string, una URL completa incluyendo el esquema
* **Default**: sin establecer, por lo que la pantalla de puerta de enlace en la nube muestra un error indicando a las personas que se comuniquen con su administrador de TI

```json managed-settings.json theme={null}
{
  "forceLoginGatewayUrl": "https://claude-gateway.example.com"
}
```

Si el valor no es una URL válida, la pantalla de inicio de sesión lo informa, y el resto del archivo de configuración administrada aún se aplica. Consulte [Establecer la URL de la puerta de enlace](/docs/es/claude-apps-gateway#set-the-gateway-url).

<h3 id="forceloginorguuid">
  `forceLoginOrgUUID`
</h3>

Desde una fuente administrada, requiera que los inicios de sesión de cuentas de claude.ai pertenezcan a una organización de Anthropic, dada como un UUID único, o a varias organizaciones, dadas como una matriz. Desde cualquier archivo de configuración, Claude Code también usa un UUID único para preseleccionar esa organización durante un inicio de sesión de claude.ai o Claude Console, y no preselecciona nada para una matriz. Si establece la clave en cualquier archivo de configuración, Claude Code también deja de ofrecer el [inicio de sesión sin clave de Console](/docs/es/authentication#sign-in-without-an-api-key) en las sesiones a las que se aplica ese archivo y crea una clave de API en su lugar.

* **Scope**: [`Any file`](#scopes). Solo una fuente administrada aplica la restricción; un UUID único en cualquier otro archivo de configuración preselecciona la organización durante el inicio de sesión sin restringirlo.
* **Type**: string, un UUID, o matriz de strings, varios UUIDs
* **Default**: sin establecer, por lo que cualquier organización puede iniciar sesión

Este ejemplo acepta inicios de sesión de cualquiera de dos organizaciones sin preseleccionar una:

```json managed-settings.json theme={null}
{
  "forceLoginOrgUUID": ["xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx", "yyyyyyyy-yyyy-yyyy-yyyy-yyyyyyyyyyyy"]
}
```

Si una fuente administrada establece una matriz vacía, o un valor que Claude Code no puede analizar, Claude Code bloquea cada inicio de sesión con un mensaje de configuración incorrecta.

Consulte [Restringir el inicio de sesión a su organización](/docs/es/authentication#restrict-login-to-your-organization) para ver cómo Claude Code trata los inicios de sesión de Claude Console, las otras rutas de inicio de sesión y las credenciales ambientales.

<h3 id="gatewayinternalnetworks">
  `gatewayInternalNetworks`
</h3>

Declare los bloques de IPv4 públicos desde los que su organización numera su red interna, para que `/login` acepte una [puerta de enlace en la nube](/docs/es/claude-apps-gateway) allí. Requiere Claude Code v2.1.268 o posterior.

Sin esta clave, `/login` se conecta a cualquier puerta de enlace en una dirección privada y nada más. Con ella, `/login` también acepta una puerta de enlace dentro de un bloque listado, solo sobre una conexión directa. La dirección propia de la máquina en esa conexión también debe estar dentro del mismo bloque.

* **Scope**: [`Managed`](#scopes). Lea solo desde una fuente en la máquina: `managed-settings.json`, la lista de propiedades de macOS o el registro HKLM de Windows, o un auxiliar de política. Claude Code lo ignora en la configuración de HKCU y administrada por servidor.
* **Type**: matriz de strings, como máximo cuatro bloques CIDR de IPv4, cada uno `/8` a `/32`, sin superponerse entre sí, y ninguno superponiéndose con espacio privado.
* **Default**: sin establecer, por lo que `/login` acepta solo puertas de enlace en direcciones privadas

```json managed-settings.json theme={null}
{
  "gatewayInternalNetworks": ["203.0.113.0/24"]
}
```

Reemplace el rango de documentación en el ejemplo con su propio bloque. Claude Code rechaza los rangos de documentación, los rangos que los clientes de VPN y NAT64 usan localmente, y el espacio reservado desde el que ninguna red está numerada, como multidifusión.

Si una entrada es inválida, o el valor no es una lista de strings, `/login` nombra el problema y rechaza cada nuevo inicio de sesión de puerta de enlace en la máquina hasta que corrija el valor. Los inicios de sesión existentes siguen funcionando. Consulte [Permitir una puerta de enlace en espacio de dirección pública que posee](/docs/es/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own) para las reglas completas y lo que ven los desarrolladores.

<h3 id="gcpauthrefresh">
  `gcpAuthRefresh`
</h3>

Ejecute su propio comando para actualizar las Credenciales Predeterminadas de Aplicación de Google Cloud cuando Claude Code encuentre que han expirado o no se pueden cargar, para que las solicitudes de [Plataforma de Agente de Google Cloud](/docs/es/google-vertex-ai) sigan funcionando sin que tenga que volver a autenticarse manualmente.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, una línea de comando del shell
* **Default**: sin establecer, por lo que el error de credencial de Claude Code le dice que ejecute `gcloud auth application-default login` usted mismo

```json settings.json theme={null}
{
  "gcpAuthRefresh": "gcloud auth application-default login"
}
```

Consulte [configuración avanzada de credenciales](/docs/es/google-vertex-ai#advanced-credential-configuration).

<h3 id="otelheadershelper">
  `otelHeadersHelper`
</h3>

Ejecute su propio comando para generar los encabezados que Claude Code envía con exportaciones de OpenTelemetry, para backends cuyos tokens rotan. Claude Code lo ejecuta al inicio y periódicamente después, y espera un objeto JSON de valores de encabezado de string en stdout.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, una ruta ejecutable o una línea de comando del shell
* **Default**: sin establecer, por lo que Claude Code no agrega encabezados generados por auxiliar

```json settings.json theme={null}
{
  "otelHeadersHelper": "/bin/generate_otel_headers.sh"
}
```

Establezca el intervalo de actualización con [`CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`](/docs/es/env-vars). Consulte [Encabezados dinámicos](/docs/es/monitoring-usage#dynamic-headers) para los requisitos del script y qué sucede cuando el auxiliar falla.

<h2 id="updates-and-versioning">
  Actualizaciones y versiones
</h2>

Elija un canal de actualización y, para las organizaciones, fije las versiones que las personas pueden ejecutar. Consulte [Actualizar Claude Code](/docs/es/setup#update-claude-code).

<h3 id="autoupdateschannel">
  `autoUpdatesChannel`
</h3>

Elija qué [canal de lanzamiento](/docs/es/setup#configure-release-channel) siguen las actualizaciones automáticas en segundo plano y `claude update`. Establezca `"stable"` para una versión que típicamente tiene aproximadamente una semana de antigüedad y omite lanzamientos con regresiones importantes, o `"latest"` para el lanzamiento más reciente.

* **Alcance**: [`Any file`](#scopes). Establézcalo en configuración administrada para aplicar un canal en toda su organización.
* **Tipo**: cadena, uno de:
  * `"latest"`: las actualizaciones siguen el lanzamiento más reciente
  * `"stable"`: las actualizaciones siguen una versión que típicamente tiene aproximadamente una semana de antigüedad y omite lanzamientos con regresiones importantes
* **Predeterminado**: sin establecer, por lo que Claude Code sigue `"latest"`

```json settings.json theme={null}
{
  "autoUpdatesChannel": "stable"
}
```

Claude Code escribe `"stable"` en su configuración de usuario cuando lo elige en **Auto-update channel** en `/config`, y elimina la clave cuando vuelve a cambiar a latest allí. `claude install stable` y `claude install latest` también guardan el canal que nombre. Cambiar de `"latest"` a `"stable"` en `/config` pregunta si permitir una degradación o permanecer en su versión actual; permanecer establece [`minimumVersion`](#minimumversion). Las instalaciones de Homebrew ignoran esta clave: el cask `claude-code` rastrea stable y `claude-code@latest` rastrea latest, y `claude update` se remite a `brew upgrade`. Para desactivar las actualizaciones automáticas por completo, establezca [`DISABLE_AUTOUPDATER`](/docs/es/setup#disable-auto-updates) en `env`.

<h3 id="minimumversion">
  `minimumVersion`
</h3>

Evite que las actualizaciones automáticas en segundo plano y `claude update` instalen cualquier versión inferior a esta, de modo que cambiar al canal `"stable"` no lo degrada desde una compilación `"latest"` más reciente. Claude Code escribe esta clave para usted cuando elige permanecer en su versión actual mientras cambia de canal en `/config`, y la borra cuando vuelve a cambiar a `"latest"`.

* **Alcance**: [`Any file`](#scopes). Establézcalo en configuración administrada para fijar un mínimo en toda la organización que la configuración de usuario y proyecto no pueda reducir.
* **Tipo**: cadena, un número de versión como `"2.1.100"`; un valor que no sea una versión válida se ignora
* **Predeterminado**: sin establecer, por lo que las actualizaciones pueden instalar cualquier versión que el canal ofrezca

Este ejemplo sigue el canal stable y se niega a instalar cualquier versión inferior a 2.1.100:

```json settings.json theme={null}
{
  "autoUpdatesChannel": "stable",
  "minimumVersion": "2.1.100"
}
```

Esta clave solo restringe las actualizaciones. Para hacer que Claude Code se niegue a iniciarse por debajo de una versión, use [`requiredMinimumVersion`](#requiredminimumversion) en su lugar. Consulte [Fijar una versión mínima](/docs/es/setup#pin-a-minimum-version).

<h3 id="requiredmaximumversion">
  `requiredMaximumVersion`
</h3>

Establezca la versión más reciente de Claude Code que su organización permite iniciar. Cuando la versión en ejecución es más reciente, Claude Code se cierra al iniciarse y le dice al usuario que instale una versión aprobada a través del método aprobado de su organización; `claude install <version>` también puede funcionar. Requiere Claude Code v2.1.163 o posterior.

* **Alcance**: [`Managed`](#scopes). Claude Code no da advertencia cuando ignora la clave en otro lugar.
* **Tipo**: cadena, un número de versión como `"2.1.150"`; un valor que no sea una versión válida se ignora
* **Predeterminado**: sin establecer, por lo que no se aplica límite superior

```json managed-settings.json theme={null}
{
  "requiredMaximumVersion": "2.1.150"
}
```

Las actualizaciones automáticas en segundo plano y `claude update` omiten versiones por encima del límite, por lo que una instalación dentro del rango permanece dentro de él. `claude update`, `claude install` y `claude doctor` continúan funcionando por encima del límite para que los usuarios puedan recuperarse. Emparéjelo con [`requiredMinimumVersion`](#requiredminimumversion) para aplicar un rango.

<h3 id="requiredminimumversion">
  `requiredMinimumVersion`
</h3>

Establezca la versión más antigua de Claude Code que su organización permite iniciar. Cuando la versión en ejecución es más antigua, Claude Code se cierra al iniciarse y le dice al usuario que actualice a través del método aprobado de su organización. La verificación se ejecuta solo al iniciarse, por lo que una sesión que ya se está ejecutando continúa. Requiere Claude Code v2.1.163 o posterior.

* **Alcance**: [`Managed`](#scopes). Claude Code no da advertencia cuando ignora la clave en otro lugar.
* **Tipo**: cadena, un número de versión como `"2.1.150"`; un valor que no sea una versión válida se ignora
* **Predeterminado**: sin establecer, por lo que no se aplica límite inferior

```json managed-settings.json theme={null}
{
  "requiredMinimumVersion": "2.1.150"
}
```

`claude update`, `claude install` y `claude doctor` continúan funcionando por debajo del límite para que los usuarios puedan recuperarse. A diferencia de [`minimumVersion`](#minimumversion), que solo previene degradaciones, esta clave bloquea el inicio. Emparéjelo con [`requiredMaximumVersion`](#requiredmaximumversion) para aplicar un rango.

<h2 id="tools">
  Herramientas
</h2>

Desactive herramientas específicas en la [aplicación de escritorio Claude Code](/docs/es/desktop). La CLI de terminal ignora estas claves. Para las herramientas en sí, consulte [Herramientas disponibles para Claude](/docs/es/tools-reference).

<h3 id="browserexternalpagetools">
  `browserExternalPageTools`
</h3>

Impida que Claude use sus herramientas para leer o actuar en páginas externas en el [panel Navegador](/docs/es/desktop#browse-external-sites) de la aplicación de escritorio. Las personas en su organización aún pueden abrir sitios externos por sí mismas, y las vistas previas del servidor de desarrollo local siguen funcionando con las herramientas de Claude. La aplicación de escritorio lee esta clave; la CLI de terminal la ignora.

* **Alcance**: [`Managed`](#scopes)
* **Tipo**: cadena, `"disabled"`; la aplicación de escritorio también acepta `"disable"`, en cualquier caso
* **Predeterminado**: sin establecer, por lo que las herramientas de Claude funcionan en páginas externas

```json managed-settings.json theme={null}
{
  "browserExternalPageTools": "disabled"
}
```

Cualquier otro valor deja las herramientas de Claude activadas, y una cadena no vacía que no sea uno de los dos valores aceptados registra una advertencia. Para bloquear sitios externos tanto para personas como para Claude, establezca [`disableBrowserExternalNavigation`](#disablebrowserexternalnavigation) en su lugar. Consulte [Restringir la navegación externa para su organización](/docs/es/desktop#restrict-external-browsing-for-your-organization).

<h3 id="disablebrowserexternalnavigation">
  `disableBrowserExternalNavigation`
</h3>

Desactive la navegación externa en el [panel Navegador](/docs/es/desktop#browse-external-sites) de la aplicación de escritorio tanto para personas como para Claude. Las vistas previas del servidor de desarrollo localhost siguen funcionando. La aplicación de escritorio lee esta clave; la CLI de terminal la ignora.

* **Alcance**: [`Managed`](#scopes)
* **Tipo**: Booleano; solo el Booleano JSON `true` tiene efecto
  * `true`: la aplicación de escritorio desactiva la navegación externa en el panel Navegador tanto para personas como para Claude; las vistas previas de localhost siguen funcionando
  * `false`: la navegación externa permanece activada
* **Predeterminado**: sin establecer, por lo que la navegación externa está activada

```json managed-settings.json theme={null}
{
  "disableBrowserExternalNavigation": true
}
```

La aplicación de escritorio ignora cualquier otro valor, y un valor que no sea un Booleano, como la cadena `"true"` o `1`, también registra una advertencia. Para dejar la navegación externa activada pero mantener las herramientas de Claude desactivadas en páginas externas, establezca [`browserExternalPageTools`](#browserexternalpagetools) en su lugar. Consulte [Restringir la navegación externa para su organización](/docs/es/desktop#restrict-external-browsing-for-your-organization).

<h3 id="disablemobilesimulatortools">
  `disableMobileSimulatorTools`
</h3>

Bloquee las herramientas de Claude para el [panel Simulador de iOS](/docs/es/desktop-ios-simulator#turn-off-simulator-access) de la aplicación de escritorio. Las personas mantienen el uso manual del panel; solo se elimina el acceso de Claude, y nadie puede reactivarlo desde dentro de la aplicación. La aplicación de escritorio lee esta clave; la CLI de terminal la ignora.

* **Alcance**: [`Managed`](#scopes)
* **Tipo**: Booleano; solo el Booleano JSON `true` tiene efecto
  * `true`: la aplicación de escritorio bloquea las herramientas de Claude para el panel Simulador de iOS
  * `false`: las herramientas del simulador de Claude siguen la configuración de alternancia de cada persona en la aplicación de escritorio
* **Predeterminado**: sin establecer, por lo que las herramientas del simulador de Claude siguen la configuración de alternancia de cada persona en la aplicación de escritorio

```json managed-settings.json theme={null}
{
  "disableMobileSimulatorTools": true
}
```

La aplicación de escritorio ignora cualquier otro valor, y un valor que no sea un Booleano, como la cadena `"true"` o `1`, también registra una advertencia.

<span id="data-and-privacy" />

<h2 id="privacy-and-telemetry">
  Privacidad y telemetría
</h2>

Controle cuánto tiempo Claude Code mantiene los datos de la sesión y qué envía. Los interruptores que desactivan las métricas de uso y los informes de errores son variables de entorno, no claves de configuración: establezca `DISABLE_TELEMETRY`, `DISABLE_ERROR_REPORTING` o `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` en la clave [`env`](#env) o en el shell. [Telemetry services](/docs/es/data-usage#telemetry-services) dice qué desactiva cada uno. Dos excepciones se desactivan desde un archivo de configuración: [`feedbackDrafts`](#feedbackdrafts) a continuación para comentarios redactados por Claude, y [`feedbackSurveyRate`](#feedbacksurveyrate) a continuación para la encuesta de sesión.

<h3 id="cleanupperioddays">
  `cleanupPeriodDays`
</h3>

Establezca cuántos días Claude Code mantiene [transcripciones de sesión y otros datos de aplicación](/docs/es/claude-directory#cleaned-up-automatically) antes de eliminarlos. Claude Code ejecuta la eliminación como un barrido de fondo después de que comienza una sesión, siempre que pueda determinar de forma segura el período de retención.

* **Scope**: [`Any file`](#scopes)
* **Type**: número de días, un número entero, mínimo `1`
* **Default**: `30`

```json settings.json theme={null}
{
  "cleanupPeriodDays": 20
}
```

Establecer `0` falla en la validación, así que elija un valor grande como `3650` para una retención prolongada. Para evitar que Claude Code escriba transcripciones en absoluto, consulte [Plaintext storage](/docs/es/claude-directory#plaintext-storage).

<h3 id="desktopsessioncleanupperioddays">
  `desktopSessionCleanupPeriodDays`
</h3>

Establezca un límite de antigüedad en días para las transcripciones de sesiones que inició o continuó más recientemente en Claude Desktop o Cowork. Sin esta clave, Claude Code [mantiene esas transcripciones a cualquier edad](/docs/es/claude-directory#cleaned-up-automatically). Claude Code elimina cada una una vez que es más antigua que tanto este límite como [`cleanupPeriodDays`](#cleanupperioddays), así que con `cleanupPeriodDays` en su valor predeterminado de 30, un valor de `7` aún las mantiene 30 días. Cuando la configuración administrada establece `cleanupPeriodDays`, ese período se aplica en su lugar y esta clave se ignora. Requiere Claude Code v2.1.248 o posterior.

* **Scope**: [`User or managed`](#scopes). Claude Code también lee la clave de un archivo que pasa con `--settings`, e la ignora en la configuración de proyecto y local.
* **Type**: número de días, un número entero, mínimo `0`
* **Default**: `0`, que no establece límite de antigüedad

```json settings.json theme={null}
{
  "desktopSessionCleanupPeriodDays": 90
}
```

<h3 id="feedbackdrafts">
  `feedbackDrafts`
</h3>

Controle [comentarios redactados por Claude](/docs/es/tools-reference#sendfeedback-tool-behavior): si Claude puede poner en cola borradores de comentarios para que usted revise, y si Claude Code muestra una tarjeta cuando Claude pone en cola uno.

* **Scope**: [`User or managed`](#scopes)
* **Type**: cadena, una de `"notify"`, `"quiet"` o `"off"`
  * `"notify"`: Claude Code muestra una tarjeta encima del mensaje cuando Claude pone en cola un borrador, hasta [tres tarjetas en una sesión](/docs/es/tools-reference#what-you-see-when-claude-drafts) de forma predeterminada
  * `"quiet"`: Claude redacta sin una tarjeta. Usted ve el recuento de borradores en cola en el pie de página del mensaje y los revisa en `/feedback`
  * `"off"`: Claude Code elimina la herramienta SendFeedback, por lo que Claude no puede poner en cola borradores
* **Default**: `"notify"`
* **Per-session overrides**: [`CLAUDE_CODE_SEND_FEEDBACK`](/docs/es/env-vars) establecido en `0` desactiva la función para una sesión

```json settings.json theme={null}
{
  "feedbackDrafts": "quiet"
}
```

Aparece en `/config` como **Claude-drafted feedback**, que escribe esta clave en su configuración de usuario. Usted ve la fila `/config` solo en sesiones [donde Claude puede redactar comentarios](/docs/es/tools-reference#sessions-without-claude-drafted-feedback); establecer `"off"` no la oculta, así que puede activar la función nuevamente desde la misma fila. Un valor en la configuración administrada tiene prioridad sobre su configuración de usuario, así que cuando un administrador establece esta clave, la fila muestra el valor administrado y cambiarla no tiene efecto. Claude Code ignora esta clave en la configuración de proyecto y local.

<h3 id="feedbacksurveyrate">
  `feedbackSurveyRate`
</h3>

Establezca la probabilidad de que la [encuesta de calidad de sesión](/docs/es/data-usage#session-quality-surveys) aparezca cuando una sesión sea elegible para ella. Establezca `0` para evitar que aparezca la encuesta.

* **Scope**: [`Any file`](#scopes)
* **Type**: número entre `0` y `1`
* **Default**: sin establecer, por lo que Claude Code utiliza la tasa que Anthropic establece de forma remota, o su tasa integrada de `0.005` en Amazon Bedrock, Google Cloud's Agent Platform y Microsoft Foundry, que no reciben configuración remota
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY`](/docs/es/env-vars) establecido en `1` desactiva la encuesta para una sesión cualquiera que sea la tasa que esta clave establece

```json settings.json theme={null}
{
  "feedbackSurveyRate": 0.05
}
```

La misma tasa se aplica a la encuesta en la extensión de VS Code.

<h3 id="skipwebfetchpreflight">
  `skipWebFetchPreflight`
</h3>

Omita la [verificación de seguridad del dominio WebFetch](/docs/es/data-usage#webfetch-domain-safety-check), que envía cada nombre de host solicitado a `api.anthropic.com` antes de obtener. Establezca `true` en entornos que bloquean el tráfico a Anthropic, como Amazon Bedrock, Google Cloud's Agent Platform o implementaciones de Microsoft Foundry con salida restrictiva.

* **Scope**: [`Any file`](#scopes)
* **Type**: Booleano
  * `true`: Claude Code omite la verificación de seguridad del dominio WebFetch
  * `false`: la verificación se ejecuta antes de la primera obtención de cada nombre de host en una sesión, y nuevamente para un nombre de host cuya verificación anterior fue bloqueada o falló
* **Default**: sin establecer, por lo que la verificación se ejecuta antes de la primera obtención de cada nombre de host en una sesión

```json settings.json theme={null}
{
  "skipWebFetchPreflight": true
}
```

Con la verificación omitida, WebFetch intenta cualquier URL sin consultar la lista de bloqueos, así que emparéjela con [reglas de permiso `WebFetch`](/docs/es/permissions#webfetch) si necesita restringir qué dominios puede alcanzar Claude.

<span id="managed-policy" />

<h2 id="enterprise-and-managed-settings">
  Configuración empresarial y gestionada
</h2>

Claves que una organización utiliza para calcular, actualizar y combinar configuraciones gestionadas. Consulte [Configurar ajustes gestionados](/docs/es/admin-setup).

<h3 id="disablesideloadflags">
  `disableSideloadFlags`
</h3>

Rechaza los indicadores CLI `--plugin-dir`, `--plugin-url`, `--agents` y `--mcp-config` al iniciar, que los usuarios podrían pasar de otra manera para eludir [`strictKnownMarketplaces`](#strictknownmarketplaces) en una única ejecución. Claude Code sale con un error que nombra los indicadores rechazados y aplica la misma verificación a las superficies que inician la CLI con estos indicadores internamente, actualmente [sesiones locales de Cowork](/docs/es/desktop) en la aplicación de escritorio. En [sesiones en la nube](/docs/es/claude-code-on-the-web), Claude Code descarta los servidores MCP que el servidor entregó a través de `--mcp-config`, excepto las entradas `type: "sdk"` en proceso, e inicia la sesión. Requiere Claude Code v2.1.193 o posterior.

* **Alcance**: [`Managed`](#scopes)
* **Tipo**: Booleano
  * `true`: Claude Code rechaza `--plugin-dir`, `--plugin-url`, `--agents` y `--mcp-config` al iniciar y sale con un error que los nombra, excepto que en sesiones en la nube descarta los servidores MCP que el servidor entregó a través de `--mcp-config`, excepto las entradas `type: "sdk"` en proceso, e inicia la sesión
  * `false`: Claude Code acepta esos indicadores
* **Predeterminado**: `false`

```json managed-settings.json theme={null}
{
  "disableSideloadFlags": true
}
```

Claude Code aún acepta un `--mcp-config` cuyos servidores son todas entradas `type: "sdk"` en proceso, por lo que el SDK del Agente y la extensión de VS Code siguen funcionando. Los usuarios aún pueden agregar servidores con `claude mcp add` o un archivo `.mcp.json`; para control por servidor, establezca también [`allowedMcpServers`](/docs/es/managed-mcp). Requiere Claude Code v2.1.193 o posterior.

La misma verificación cubre carpetas de plugins nombradas en la variable de entorno [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/es/env-vars#variables), que requiere Claude Code v2.1.280 o posterior. Cuando la variable nombra una carpeta, Claude Code sale con el mismo error, y el error dice que desestablezca la variable.

En sesiones en la nube, Claude Code también ignora las actualizaciones MCP entregadas por el servidor a mitad de sesión, la ruta detrás de la configuración de sesión en la nube y llamadas `setMcpServers()` del SDK que llegan a esas sesiones. Las entradas `type: "sdk"` en proceso permanecen exentas allí también. Antes de v2.1.239, un `--mcp-config` entregado por el servidor bloqueaba el inicio de una sesión en la nube.

<h3 id="forceremotesettingsrefresh">
  `forceRemoteSettingsRefresh`
</h3>

Bloquea el inicio de la CLI hasta que Claude Code haya obtenido recientemente [configuraciones gestionadas por el servidor](/docs/es/server-managed-settings). Si la obtención falla, Claude Code sale en lugar de continuar con configuraciones en caché o sin configuraciones. Establézcalo cuando su entorno no pueda aceptar ni siquiera una breve ventana en la que una sesión se ejecute sin su política gestionada.

Cuando la clave no está establecida, Claude Code no bloquea el inicio en la obtención, aunque cuando el desarrollador inicia sesión al iniciar espera hasta cinco segundos para la obtención. Una sesión de puerta de enlace en la nube siempre espera y sale si no se puede alcanzar la puerta de enlace.

* **Alcance**: [`Managed`](#scopes). Claude Code honra un `true` de cualquier fuente gestionada controlada por administrador, incluso una que no sea la fuente de mayor prioridad.
* **Tipo**: Booleano
  * `true`: Claude Code bloquea el inicio hasta que haya obtenido recientemente configuraciones gestionadas por el servidor y sale si la obtención falla
  * `false`: Claude Code no bloquea el inicio en la obtención, aunque en un inicio de inicio de sesión espera hasta cinco segundos para la obtención
* **Predeterminado**: `false`

```json managed-settings.json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

Establézcalo en un perfil MDM o el archivo de configuraciones gestionadas para aplicar el inicio cerrado por error antes de que llegue la primera carga útil del servidor. Claude Code aplica la verificación solo en sesiones que obtienen configuraciones gestionadas por el servidor, por lo que una sesión que [no las obtiene](/docs/es/server-managed-settings#platform-availability) se inicia sin esperar. Los subcomandos `claude auth` están exentos, por lo que los usuarios pueden volver a autenticarse cuando las credenciales caducadas son la razón por la que falla la obtención. Consulte [Aplicar inicio cerrado por error](/docs/es/server-managed-settings#enforce-fail-closed-startup).

<h3 id="managedsourcesbehavior">
  `managedSourcesBehavior`
</h3>

Elija si Claude Code aplica solo la [fuente gestionada](/docs/es/managed-settings#how-claude-code-combines-managed-sources) de mayor prioridad que su organización entrega, o combina todas las fuentes de administrador que entrega. De forma predeterminada, Claude Code toma la fuente de mayor prioridad que lleva una [clave de política](/docs/es/managed-settings#how-claude-code-combines-managed-sources) e ignora el resto. Una clave de política es cualquier clave de configuración que no sea esta y `wslInheritsWindowsSettings`. Entonces, una vez que las configuraciones gestionadas por el servidor o una política MDM entreguen una clave de política, un archivo `managed-settings.json` contribuye solo con las [claves que Claude Code lee de todas las fuentes de administrador](/docs/es/managed-settings#keys-read-from-every-admin-source). Con `"merge"`, todas las fuentes de administrador que entrega contribuyen sus claves a una política combinada. Requiere Claude Code v2.1.242 o posterior.

Establezca `"merge"` solo donde todas las fuentes [clasificadas](/docs/es/managed-settings#how-claude-code-combines-managed-sources) por debajo de la suya están bajo el control de un administrador, porque Claude Code luego agrega entradas de una fuente inferior, como reglas `permissions.allow`, a la política.

* **Alcance**: [`Managed`](#scopes). Claude Code lee esta clave de la fuente de mayor prioridad que lleva esta clave o una clave de política, e ignora esta clave en todas las fuentes clasificadas más bajas, por lo que una fuente inferior no puede optar por combinarse con la fuente anterior. Ni el registro HKCU de Windows ni [configuraciones principales de un host de incrustación](/docs/es/managed-settings#let-an-embedding-host-add-policy) participan en la combinación.
* **Tipo**: cadena, una de:
  * `"first-wins"`: la fuente de mayor prioridad que lleva una clave de política suministra la política, y las fuentes inferiores contribuyen solo con las [claves que Claude Code lee de todas las fuentes de administrador](/docs/es/managed-settings#keys-read-from-every-admin-source)
  * `"merge"`: todas las fuentes de administrador que entrega contribuyen sus claves, combinadas por las reglas a continuación
* **Predeterminado**: `"first-wins"`

Entregue la clave en la fuente de mayor prioridad que implemente. Una máquina que nunca recibe configuraciones gestionadas por el servidor necesita la clave en su perfil MDM también, porque Claude Code lee la clave de la fuente de mayor prioridad que la lleva o una clave de política. Un archivo `managed-settings.json` es la fuente de administrador de menor rango, por lo que `"merge"` establecido allí no tiene una fuente inferior con la que combinarse. En configuraciones gestionadas por el servidor, la clave se ve así:

```json theme={null}
{
  "managedSourcesBehavior": "merge"
}
```

Bajo `"merge"`, Claude Code combina cada clave por su tipo. Esta tabla proporciona la regla para cada tipo. Las filas de lista de restricciones, valores tomados en su totalidad y solo fuente de mayor prioridad nombran todas las claves que cubren, y las otras filas dan ejemplos:

| Tipo de clave                             | Cómo Claude Code la combina                                                                                                                                                                                                                           | Claves                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :---------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Listas                                    | Combina entradas de todas las fuentes                                                                                                                                                                                                                 | [`permissions.allow`](#permissions-allow), [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains) y otras claves de lista                                                                                                                                                                                                                                                                                                                                                                                           |
| Bloqueos                                  | Aplica el valor más estricto que establece cualquier fuente. Cuando ninguna fuente establece un valor estricto, aplica un valor más flexible solo de la fuente de mayor prioridad                                                                     | [`allowManagedPermissionRulesOnly`](#allowmanagedpermissionrulesonly), [`permissions.disableBypassPermissionsMode`](#permissions-disablebypasspermissionsmode) y otros bloqueos booleanos o de enumeración                                                                                                                                                                                                                                                                                                                       |
| Listas de restricciones                   | Toma la lista en su totalidad de la fuente de mayor prioridad que la establece, sin agregar entradas de fuentes inferiores. Cuando la fuente de mayor prioridad no establece una, la toma en su totalidad de la siguiente fuente hacia abajo          | [`availableModels`](#availablemodels), [`allowedMcpServers`](#allowedmcpservers), [`strictKnownMarketplaces`](#strictknownmarketplaces), [`allowedChannelPlugins`](#allowedchannelplugins) y la cadena [`fallbackModel`](#fallbackmodel)                                                                                                                                                                                                                                                                                         |
| Valores tomados en su totalidad           | Toma el valor en su totalidad de la fuente de mayor prioridad que lo establece, sin combinar entradas o campos de fuentes inferiores. Cuando la fuente de mayor prioridad no lo establece, lo toma en su totalidad de la siguiente fuente hacia abajo | [`sandbox.credentials.awsPairs`](#sandbox-credentials-awspairs), [`sandbox.ripgrep`](#sandbox-ripgrep)                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Servidores MCP proporcionados             | Combina los nombres de servidor de todas las fuentes. Cuando dos fuentes establecen el mismo nombre, aplica la entrada completa de la fuente superior                                                                                                 | [`managedMcpServers`](#managedmcpservers)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| Leer solo de la fuente de mayor prioridad | Lee la clave solo de la fuente de mayor prioridad que lleva una clave de política, por lo que el valor de una fuente inferior se ignora incluso cuando la fuente de mayor prioridad no establece ninguno                                              | [`apiKeyHelper`](#apikeyhelper), [`awsAuthRefresh`](#awsauthrefresh), [`awsCredentialExport`](#awscredentialexport), [`gcpAuthRefresh`](#gcpauthrefresh), [`otelHeadersHelper`](#otelheadershelper), `proxyAuthHelper`, [`forceLoginOrgUUID`](#forceloginorguuid), los valores `"claudeai"` y `"console"` de [`forceLoginMethod`](#forceloginmethod), [`parentSettingsBehavior`](#parentsettingsbehavior), [`modelPicker`](#modelpicker), [`policyHelper`](#policyhelper), [`permissions.defaultMode`](#permissions-defaultmode) |
| `env`                                     | [Combina por variable en todas las fuentes de administrador](/docs/es/managed-settings#keys-read-from-every-admin-source), bajo tanto `"first-wins"` como `"merge"`                                                                                        | [`env`](#env)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| Todas las otras claves                    | Toma el valor de la fuente de mayor prioridad que lo establece                                                                                                                                                                                        | [`cleanupPeriodDays`](#cleanupperioddays), [`model`](#model)                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |

Tomar `sandbox.credentials.awsPairs` y `sandbox.ripgrep` en su totalidad requiere Claude Code v2.1.257 o posterior.

Algunas claves agregan una condición que la tabla no muestra:

* **[`policyHelper`](#policyhelper)**: Claude Code la honra solo cuando la fuente de mayor prioridad que lleva una clave de política es una política MDM o un archivo de configuraciones gestionadas, por lo que bajo configuraciones gestionadas por el servidor no se aplica.
* **[`modelOverrides`](#modeloverrides)**: se empareja con `availableModels`. Claude Code toma `modelOverrides` de la fuente de mayor prioridad que la establece, a menos que una fuente superior establezca `availableModels` sin `modelOverrides`. En ese caso, ignora `modelOverrides` de todas las fuentes.
* **[`forceLoginGatewayUrl`](#forcelogingatewayurl), [`gatewayInternalNetworks`](#gatewayinternalnetworks) y el valor `"gateway"` de [`forceLoginMethod`](#forceloginmethod)**: Claude Code nunca lee ninguno de ellos de configuraciones gestionadas por el servidor, por lo que un valor allí ni se aplica ni oculta uno establecido en una política MDM o archivo de configuraciones gestionadas. Entre las fuentes de administrador en la máquina, solo la de mayor rango que lleva una clave de política los suministra, independientemente de si las configuraciones gestionadas por el servidor también están presentes.

Para confirmar qué fuentes se combinaron en una máquina, ejecute `/status` y [lea la línea `Setting sources`](/docs/es/managed-settings#read-the-source-in-/status).

<h3 id="parentsettingsbehavior">
  `parentSettingsBehavior`
</h3>

Elija si Claude Code aplica configuraciones gestionadas suministradas por un proceso host de incrustación, como el SDK del Agente o una extensión IDE, cuando también está presente una capa gestionada implementada por administrador. Con `"first-wins"`, Claude Code descarta las configuraciones suministradas por el host; con `"merge"`, las aplica bajo la capa de administrador a través de un filtro restrictivo. Establezca `"merge"` cuando un host necesita pasar sus propias restricciones a las sesiones que inicia, por ejemplo Claude Desktop entregando una lista de permisos de salida de una puerta de enlace.

* **Alcance**: [`Managed`](#scopes). Claude Code la lee de la fuente gestionada controlada por administrador de mayor prioridad.
* **Tipo**: cadena, una de:
  * `"first-wins"`: Claude Code descarta las configuraciones suministradas por el host cuando está presente una capa gestionada implementada por administrador
  * `"merge"`: Claude Code aplica las configuraciones suministradas por el host bajo la capa de administrador a través de un filtro restrictivo
* **Predeterminado**: `"first-wins"`

```json managed-settings.json theme={null}
{
  "parentSettingsBehavior": "merge"
}
```

Esta clave no tiene efecto cuando no existe una capa gestionada implementada por administrador: las configuraciones del host se aplican entonces como la única capa gestionada, aún filtradas a valores restrictivos. Para los límites del filtro y cómo interactúan las fuentes gestionadas, consulte [Configuraciones principales de hosts de incrustación](/docs/es/managed-settings#parent-settings-from-embedding-hosts) y [Restringir configuraciones principales](/docs/es/claude-apps-gateway#restrict-parent-settings).

<span id="compute-managed-settings-with-a-policy-helper" />

<h3 id="policyhelper">
  `policyHelper`
</h3>

Ejecute un ejecutable que implemente que calcula configuraciones gestionadas al iniciar, para que pueda derivar la política de la postura del dispositivo, la identidad o un servicio remoto en lugar de un archivo estático. Claude Code ejecuta el asistente antes de aceptar el primer mensaje y trata las configuraciones que emite como las configuraciones gestionadas para la sesión.

* **Alcance**: [`Managed`](#scopes). Leer desde la plist de macOS, el registro HKLM de Windows o el archivo de configuraciones gestionadas. Claude Code lee la clave de la fuente gestionada de mayor prioridad que lleva una [clave de política](/docs/es/managed-settings#how-claude-code-combines-managed-sources) y ejecuta el asistente solo cuando esa fuente es una de esas tres; ignora la clave en configuraciones gestionadas por el servidor, el registro HKCU y configuraciones principales suministradas por el host.
* **Tipo**: objeto con `path`, `timeoutMs` y `refreshIntervalMs`
* **Predeterminado**: no establecido, por lo que no se ejecuta ningún asistente

Cuando las configuraciones gestionadas por el servidor entregan la política al iniciar, tienen prioridad sobre la fuente del asistente y el asistente no se ejecuta.

Si una obtención de configuraciones posterior informa que las configuraciones gestionadas por el servidor se eliminaron, Claude Code ejecuta el asistente en ese punto en lugar de esperar al siguiente inicio. Su salida rige el resto de la sesión, y una ejecución que falla termina la sesión con el mismo mensaje que una [ejecución de inicio fallida](#helper-failures).

Este ejemplo ejecuta el asistente con un tiempo de espera de 5 segundos y lo vuelve a ejecutar cada cinco minutos:

```json managed-settings.json theme={null}
{
  "policyHelper": {
    "path": "/usr/local/bin/claude-policy",
    "timeoutMs": 5000,
    "refreshIntervalMs": 300000
  }
}
```

<h4 id="write-the-helper-output">
  Escribir la salida del asistente
</h4>

Claude Code ejecuta el asistente sin argumentos, establece `CLAUDE_CODE_VERSION` en su entorno y lee un envoltorio JSON desde stdout, limitado a 1 MiB.

Coloque las configuraciones bajo una clave `managedSettings`. Un objeto de configuraciones desnudo sin clave `managedSettings` se analiza con `managedSettings` indefinido y no aplica nada, y Claude Code no informa ningún error:

```json theme={null}
{
  "managedSettings": {
    "permissions": { "deny": ["Read(//etc/secrets/**)"] }
  }
}
```

Cuando el asistente emite `managedSettings`, ese objeto se convierte en la única fuente de configuraciones gestionadas para la ejecución: Claude Code ignora las fuentes MDM, archivo y HKCU, lee las [claves entre fuentes](/docs/es/managed-settings#keys-read-from-every-admin-source) solo de la salida del asistente y nunca combina [configuraciones principales](/docs/es/managed-settings#parent-settings-from-embedding-hosts).

La verificación de inicio `forceRemoteSettingsRefresh` se ejecuta antes del asistente y lee cualquier fuente de administrador. Un asistente que sale con `0` con un envoltorio que omite `managedSettings` no contribuye con configuraciones gestionadas, y las otras fuentes se aplican como de costumbre.

<h4 id="helper-failures">
  Fallos del asistente
</h4>

Una ejecución del asistente falla cuando:

* `path` rompe las reglas en [`policyHelper.path`](#policyhelper-path).
* No hay un archivo regular en `path`. Claude Code verifica el archivo antes de iniciar el asistente, dentro del mismo presupuesto `timeoutMs`, por lo que un montaje de red sin respuesta puede causar que la ejecución falle.
* El asistente sale con un código distinto de cero, aún se está ejecutando cuando `timeoutMs` transcurre, o no se inicia en absoluto, por ejemplo porque no es ejecutable.
* El asistente escribe más de 1 MiB en stdout o stderr.
* stdout no es un único objeto JSON, o su `managedSettings` tiene una [violación de esquema que Claude Code no puede reparar](/docs/es/managed-settings#find-entries-claude-code-dropped).

Cuando la ejecución de inicio falla, Claude Code imprime la razón y se niega a iniciar. Después de una salida distinta de cero, la razón incluye stderr del asistente, o su stdout cuando stderr está vacío. Después de un tiempo de espera, la razón nombra el límite `timeoutMs` e incluye ninguna de la salida del asistente. La negativa cubre sesiones interactivas, `claude -p`, sesiones del SDK del Agente, [sesiones en segundo plano](/docs/es/agent-view) y la mayoría de subcomandos.

La negativa es deliberada, por lo que un asistente que necesita resiliencia de interrupción debe servir desde su propio caché y salir con `0`.

Cuando una actualización en segundo plano falla, Claude Code mantiene la última política exitosa en vigor, y `/status` muestra la actualización fallida con su razón hasta que una actualización tenga éxito. Cada actualización se ejecuta bajo las mismas reglas de `timeoutMs` y fallo que la ejecución de inicio.

Con `--debug`, Claude Code escribe stderr del asistente de cada ejecución en el [registro de depuración](/docs/es/debug-your-config).

Claude Code informa un valor `policyHelper` inválido como una [entrada descartada](/docs/es/managed-settings#find-entries-claude-code-dropped) e inicia la sesión en las configuraciones gestionadas restantes sin ejecutar un asistente. Los valores inválidos incluyen una cadena de ruta desnuda y un `timeoutMs` por debajo de [su mínimo](#policyhelper-timeoutms).

Para desactivar un asistente, elimine la clave de la fuente que la establece.

<h3 id="policyhelper-path">
  `policyHelper.path`
</h3>

Nombre el ejecutable del asistente que Claude Code ejecuta. Para lo que sucede cuando la ruta rompe las reglas a continuación, consulte [Fallos del asistente](#helper-failures).

* **Alcance**: [`Managed`](#scopes). Leer desde la plist de macOS, el registro HKLM de Windows o el archivo de configuraciones gestionadas, donde se lee [`policyHelper`](#policyhelper).
* **Tipo**: cadena, una ruta absoluta en forma normalizada, sin segmentos `.` o `..`; en Windows, una ruta de letra de unidad o UNC que termina en `.exe`
* **Predeterminado**: ninguno; requerido cuando `policyHelper` está establecido

```json managed-settings.json theme={null}
{
  "policyHelper": {
    "path": "/usr/local/bin/claude-policy"
  }
}
```

<h3 id="policyhelper-timeoutms">
  `policyHelper.timeoutMs`
</h3>

Establezca cuánto tiempo Claude Code espera al asistente antes de tratar la ejecución como fallida. Una ejecución agotada por tiempo falla de la misma manera que una salida distinta de cero, por lo que al iniciar Claude Code se niega a iniciar.

* **Alcance**: [`Managed`](#scopes). Leer desde la plist de macOS, el registro HKLM de Windows o el archivo de configuraciones gestionadas, donde se lee [`policyHelper`](#policyhelper).
* **Tipo**: entero, milisegundos, mínimo `1000`
* **Predeterminado**: `10000`

```json managed-settings.json theme={null}
{
  "policyHelper": {
    "path": "/usr/local/bin/claude-policy",
    "timeoutMs": 5000
  }
}
```

<h3 id="policyhelper-refreshintervalms">
  `policyHelper.refreshIntervalMs`
</h3>

Haga que Claude Code vuelva a ejecutar el asistente en segundo plano en un intervalo para que los cambios de política lleguen a una sesión en ejecución. Cuando una actualización tiene éxito, su salida reemplaza las configuraciones gestionadas anteriores sin un reinicio; cuando una actualización falla, Claude Code mantiene la política que ya tiene.

* **Alcance**: [`Managed`](#scopes). Leer desde la plist de macOS, el registro HKLM de Windows o el archivo de configuraciones gestionadas, donde se lee [`policyHelper`](#policyhelper).
* **Tipo**: entero, milisegundos: `0` para desactivar la actualización, de lo contrario al menos `60000`
* **Predeterminado**: no establecido, por lo que Claude Code ejecuta el asistente una vez al iniciar

Este ejemplo vuelve a ejecutar el asistente cada cinco minutos:

```json managed-settings.json theme={null}
{
  "policyHelper": {
    "path": "/usr/local/bin/claude-policy",
    "refreshIntervalMs": 300000
  }
}
```

<h3 id="wslinheritswindowssettings">
  `wslInheritsWindowsSettings`
</h3>

Haga que Claude Code en WSL lea configuraciones gestionadas de la cadena de política de Windows, con HKLM y el archivo de configuraciones gestionadas de Windows teniendo prioridad sobre `/etc/claude-code` y HKCU debajo. Mientras la cadena está activada, Claude Code lee `/etc/claude-code` solo cuando ningún archivo de configuraciones gestionadas o descarga bajo `C:\Program Files\ClaudeCode\` entrega una [clave de política](/docs/es/managed-settings#how-claude-code-combines-managed-sources). Establézcalo para extender la política que ya implementa en Windows a sesiones WSL en la misma máquina, para que sigan las mismas reglas que las sesiones del host. Claude Code la honra solo cuando está establecida en la clave del registro HKLM o en un archivo de configuraciones gestionadas o descarga bajo `C:\Program Files\ClaudeCode\`, ambos de los cuales requieren administrador de Windows para escribir.

* **Alcance**: [`Managed`](#scopes). En una fuente de Windows controlada por administrador.
* **Tipo**: Booleano
  * `true`: Claude Code en WSL lee configuraciones gestionadas de la cadena de política de Windows y lee `/etc/claude-code` solo cuando ningún archivo de configuraciones gestionadas o descarga bajo `C:\Program Files\ClaudeCode\` entrega una [clave de política](/docs/es/managed-settings#how-claude-code-combines-managed-sources)
  * `false`: WSL lee solo `/etc/claude-code`
* **Predeterminado**: `false`, por lo que WSL lee solo `/etc/claude-code`

```json managed-settings.json theme={null}
{
  "wslInheritsWindowsSettings": true
}
```

Una vez que una fuente de administrador activa la cadena, la política HKCU se une a ella en WSL solo cuando HKCU también establece la clave en `true`. Esa copia no activa la cadena por sí sola. Una fuente de Windows que contiene solo esta clave no cuenta como una fuente de política, por lo que una fuente de menor prioridad aún suministra la política. Esta clave no tiene efecto en Windows nativo.

<h2 id="global-config-settings">
  Configuración global
</h2>

Guarde estas claves en `~/.claude.json`, no en un archivo de configuración. Claude Code las ignora en cualquier otro lugar. Claude Code y `/config` escriben la mayoría de ellas automáticamente, y también puede editarlas manualmente.

<h3 id="autoconnectide">
  `autoConnectIde`
</h3>

Conecte a un IDE en ejecución automáticamente cuando inicie Claude Code desde una terminal externa. Aparece en `/config` como **Auto-conectar a IDE (terminal externa)** cuando ejecuta Claude Code fuera de una terminal de VS Code o JetBrains.

* **Alcance**: [`Configuración global`](#scopes)
* **Tipo**: Booleano
  * `true`: Claude Code se conecta a un IDE en ejecución automáticamente cuando lo inicia desde una terminal externa
  * `false`: Claude Code no se conecta automáticamente desde una terminal externa; dentro de una terminal de VS Code o JetBrains, o con `--ide`, sigue conectándose
* **Predeterminado**: `false`
* **Anulaciones por sesión**: [`CLAUDE_CODE_AUTO_CONNECT_IDE`](/docs/es/env-vars) tiene prioridad sobre esta clave para una sesión, en cualquier dirección

```json ~/.claude.json theme={null}
{
  "autoConnectIde": true
}
```

Claude Code ignora esta clave en `settings.json`.

<h3 id="autoinstallideextension">
  `autoInstallIdeExtension`
</h3>

Instale la extensión de IDE de Claude Code automáticamente cuando ejecute Claude Code desde una terminal de VS Code. Aparece en `/config` como **Auto-instalar extensión de IDE** cuando ejecuta Claude Code dentro de una terminal de VS Code o JetBrains.

* **Alcance**: [`Configuración global`](#scopes)
* **Tipo**: Booleano
  * `true`: Claude Code instala la extensión de IDE automáticamente cuando lo ejecuta desde una terminal de VS Code
  * `false`: Claude Code no instala la extensión automáticamente
* **Predeterminado**: `true`
* **Anulaciones por sesión**: [`CLAUDE_CODE_IDE_SKIP_AUTO_INSTALL`](/docs/es/env-vars) establecido en `1` omite la instalación para una sesión incluso cuando esta clave es `true`

```json ~/.claude.json theme={null}
{
  "autoInstallIdeExtension": false
}
```

Claude Code ignora esta clave en `settings.json`.

<h3 id="copyonselect">
  `copyOnSelect`
</h3>

Copie texto al portapapeles automáticamente cuando termine de seleccionarlo con el ratón en [renderizado a pantalla completa](/docs/es/fullscreen#use-the-mouse) o [vista de agente](/docs/es/agent-view). Aparece en `/config` como **Copiar al seleccionar** mientras el renderizado a pantalla completa está activado.

* **Alcance**: [`Configuración global`](#scopes)
* **Tipo**: Booleano
  * `true`: Claude Code copia el texto al portapapeles cuando termina de seleccionarlo
  * `false`: seleccionar texto deja el portapapeles sin cambios, y [copia la selección con un atajo de teclado](/docs/es/fullscreen#use-the-mouse) en su lugar
* **Predeterminado**: `true`

```json ~/.claude.json theme={null}
{
  "copyOnSelect": false
}
```

Claude Code ignora esta clave en `settings.json`.

<h3 id="difftool">
  `diffTool`
</h3>

Elija dónde Claude Code muestra la diferencia de un cambio `Edit` o `Write` que propone cuando un IDE [VS Code](/docs/es/vs-code) o [JetBrains](/docs/es/jetbrains#features) está conectado: `"auto"` lo abre en el visor de diferencias del IDE, `"terminal"` lo mantiene en la terminal. Aparece en `/config` como **Herramienta de diferencias** solo mientras Claude Code está conectado a un IDE de VS Code o JetBrains.

* **Alcance**: [`Configuración global`](#scopes)
* **Tipo**: cadena, una de:
  * `"auto"`: Claude Code abre la diferencia en el visor de diferencias del IDE cuando un IDE de VS Code o JetBrains está conectado
  * `"terminal"`: Claude Code mantiene la diferencia en la terminal
* **Predeterminado**: `"auto"`

```json ~/.claude.json theme={null}
{
  "diffTool": "terminal"
}
```

Claude Code ignora esta clave en `settings.json`.

<h3 id="externaleditorcontext">
  `externalEditorContext`
</h3>

Cuando presiona `Ctrl+G`, Claude Code abre el mensaje que está escribiendo en su [editor externo](/docs/es/interactive-mode#general-controls). Con esta clave activada, el búfer del editor comienza con la respuesta anterior de Claude como líneas de comentario `#`, para que pueda leerla mientras escribe, y Claude Code elimina esas líneas cuando guarda. Aparece en `/config` como **Mostrar última respuesta en editor externo**.

* **Alcance**: [`Configuración global`](#scopes)
* **Tipo**: Booleano
  * `true`: el búfer del editor comienza con la respuesta anterior de Claude como líneas de comentario `#`, que Claude Code elimina cuando guarda
  * `false`: el búfer del editor se abre solo con su mensaje
* **Predeterminado**: `false`

```json ~/.claude.json theme={null}
{
  "externalEditorContext": true
}
```

Con esta opción activada, el búfer que Claude Code abre se ve así, y solo el texto debajo de la línea del marcador se envía como su mensaje:

```text theme={null}
# ─── Última respuesta de Claude (para referencia; eliminada al guardar) ───
# Agregué el bucle de reintentos a fetchUser en src/api.ts y una prueba
# para el caso de tiempo de espera. ¿Quiere que conecte el mismo reintento en
# fetchOrders?
# ─── Escriba su respuesta debajo de esta línea ──────────────────────────

Sí, y límitelo a tres intentos.
```

Claude Code mantiene las últimas 50 líneas de la respuesta y marca el corte con `# … (salida anterior truncada)`.

Claude Code ignora esta clave en `settings.json`.

<h3 id="permissionexplainerenabled">
  `permissionExplainerEnabled`
</h3>

<Warning>
  Eliminado en v2.1.257, junto con la explicación del comando `Ctrl+E` en mensajes de permisos de Bash y PowerShell. Establecer esta opción no tiene efecto en las versiones actuales.
</Warning>

Hasta v2.1.256, podía presionar `Ctrl+E` en un mensaje de permisos de Bash o PowerShell para ver una explicación generada por el modelo del comando, y establecer esta clave en `false` para desactivar ese atajo.

* **Alcance**: [`Configuración global`](#scopes). En v2.1.256 y anteriores.
* **Tipo**: Booleano
* **Predeterminado**: `true`

<h3 id="teammatedefaultmodel">
  `teammateDefaultModel`
</h3>

<Warning>
  Eliminado en v2.1.234, junto con su fila `/config` **Modelo de compañero predeterminado**. Establecer esta opción no tiene efecto en las versiones actuales.
</Warning>

Hasta v2.1.233, establecía esta clave en el modelo para los compañeros del [equipo de agentes](/docs/es/agent-teams#specify-teammates-and-models) que su mensaje no nombró un modelo: un alias como `"sonnet"`, o `null` para seguir el modelo del líder. Para el modelo que Claude Code elige ahora para tales compañeros, consulte [especificar compañeros y modelos](/docs/es/agent-teams#specify-teammates-and-models).

* **Alcance**: [`Configuración global`](#scopes). En v2.1.233 y anteriores.
* **Tipo**: cadena, un alias de modelo o ID de modelo completo, o `null`
* **Predeterminado**: sin establecer

<h2 id="see-also">
  Véase también
</h2>

* [Configurar permisos](/docs/es/permissions): sintaxis de reglas, modos de permisos y confianza del espacio de trabajo
* [Variables de entorno](/docs/es/env-vars): cada variable `CLAUDE_*`, `ANTHROPIC_*` y de proveedor que Claude Code lee
* [Herramientas disponibles para Claude](/docs/es/tools-reference): las herramientas integradas y cuáles necesitan aprobación
* [Archivos de configuración de ejemplo](/docs/es/settings-example): un archivo personal, un archivo de equipo y un archivo administrado de una organización
* [Configurar ajustes administrados](/docs/es/admin-setup): cómo las organizaciones deciden qué aplicar
* [Implementar ajustes administrados](/docs/es/managed-settings): mecanismos de entrega, precedencia dentro del nivel administrado y entradas no válidas en ajustes administrados
* [Depurar su configuración](/docs/es/debug-your-config): `claude doctor` y el diálogo de Error de configuración
