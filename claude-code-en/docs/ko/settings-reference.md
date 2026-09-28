> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 모든 설정

> Claude Code settings.json의 모든 키에 대한 완전한 참조: 각 키의 위치, 유형 및 기본값, 붙여넣기 가능한 예제, 모든 키의 인덱스.

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

<BackToIndex href="#all-settings" label="인덱스로 돌아가기" />

이 참조 페이지는 Claude Code가 설정 파일에서 읽는 각 키와 대신 `~/.claude.json`에 유지하는 [짧은 키 그룹](#global-config-settings)을 나열합니다. 파일을 선택하거나 우선순위를 확인하려면 [설정 파일 및 우선순위](/docs/ko/settings)에서 시작하십시오.

<span id="available-settings" />

<span id="scopes" />

<span id="all-settings" />

<h2 id="settings-index">
  설정 인덱스
</h2>

아래의 모든 키는 해당 항목으로 연결됩니다. 범위는 [파일](/docs/ko/settings#settings-files-and-who-they-affect)을 나열합니다: `User`는 `~/.claude/settings.json`, `Project`는 `.claude/settings.json`, `Local`은 `.claude/settings.local.json`, `Managed`는 [조직이 배포하는 것](/docs/ko/managed-settings)입니다. `Any file`은 네 가지 모두를 의미하고, `Global config`는 [`~/.claude.json`](#global-config-settings)을 의미합니다.

<ReferenceFilter
  noun="settings"
  placeholder="Filter settings by key or purpose"
  facetOrder={{ scope: ["Any file", "User, local, or managed", "User or managed", "Managed", "Global config"] }}
  columnHelp={{
topic: "The section of this page that holds the entry. Use Sort by to group the table by topic.",
scope: "Which settings files can set the key: user (~/.claude/settings.json), project (.claude/settings.json), local (.claude/settings.local.json), or managed (deployed by your organization). Global config keys are in ~/.claude.json instead.",
}}
/>

| 키                                                                                                     | 설명                                                                                                                                                                                | 주제              | 범위             |
| :---------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------- | :------------- |
| [`advisorModel`](#advisormodel)                                                                       | Claude가 [advisor tool](/docs/ko/advisor)에 답할 때 사용할 모델 선택                                                                                                                               | 모델 및 응답         | 모든 파일          |
| [`agent`](#agent)                                                                                     | 모든 세션을 명명된 [subagent](/docs/ko/sub-agents)로 시작하고 프롬프트, 도구, 모델 포함                                                                                                                       | 에이전트, 세션, 워크트리  | 모든 파일          |
| [`agentPushNotifEnabled`](#agentpushnotifenabled)                                                     | Claude가 결정할 때 [휴대폰으로 푸시 알림](/docs/ko/remote-control#mobile-push-notifications)을 보낼 수 있도록 허용                                                                                            | 원격, 데스크톱, 알림    | 모든 파일          |
| [`allowAllClaudeAiMcps`](#allowallclaudeaimcps)                                                       | Claude Code가 배포된 [`managed-mcp.json`](/docs/ko/managed-mcp#exclusive-control-with-managed-mcp-json)과 함께 자체적으로 가져오는 [claude.ai 커넥터](/docs/ko/mcp) 로드                                         | MCP             | 관리됨            |
| [`allowedChannelPlugins`](#allowedchannelplugins)                                                     | 메시지를 푸시할 수 있는 [채널 플러그인](/docs/ko/channels#restrict-which-channel-plugins-can-run)의 기본 허용 목록 교체                                                                                         | 플러그인 및 기술       | 관리됨            |
| [`allowedHttpHookUrls`](#allowedhttphookurls)                                                         | [HTTP hooks](/docs/ko/hooks)가 대상으로 할 수 있는 URL 제한                                                                                                                                       | 훅 및 자동화         | 모든 파일          |
| [`allowedMcpServers`](#allowedmcpservers)                                                             | 사용자가 추가할 수 있는 [MCP 서버](/docs/ko/mcp) 허용 목록                                                                                                                                             | MCP             | 모든 파일          |
| [`allowManagedHooksOnly`](#allowmanagedhooksonly)                                                     | 조직이 배포하는 [훅](/docs/ko/hooks)만 실행                                                                                                                                                       | 훅 및 자동화         | 관리됨            |
| [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly)                                           | 관리되는 [MCP](/docs/ko/mcp) 허용 목록을 유일하게 적용되는 것으로 만들기                                                                                                                                      | MCP             | 관리됨            |
| [`allowManagedPermissionRulesOnly`](#allowmanagedpermissionrulesonly)                                 | [관리되는 설정](/docs/ko/managed-settings)을 [권한 규칙](/docs/ko/permissions#managed-settings)의 유일한 설정 소스로 만들기                                                                                        | 권한 설정           | 관리됨            |
| [`alwaysThinkingEnabled`](#alwaysthinkingenabled)                                                     | 모든 세션에 대해 [확장 사고](/docs/ko/model-config#extended-thinking) 끄기                                                                                                                          | 모델 및 응답         | 모든 파일          |
| [`apiKeyHelper`](#apikeyhelper)                                                                       | 자신의 명령으로 [API 자격증명](/docs/ko/authentication#credential-management) 생성                                                                                                                  | 인증 및 공급자        | 모든 파일          |
| [`askUserQuestionTimeout`](#askuserquestiontimeout)                                                   | 답변되지 않은 질문이 유휴 시간 후 [자동 계속](/docs/ko/tools-reference#question-auto-continue-timeout) 되도록 허용                                                                                            | 인터페이스 및 터미널     | 사용자 또는 관리됨     |
| [`attribution`](#attribution)                                                                         | Claude Code가 커밋 및 풀 요청에 추가하는 속성 사용자 정의                                                                                                                                            | Git 및 속성        | 모든 파일          |
| [`attribution.commit`](#attribution-commit)                                                           | Claude Code가 커밋에 추가하는 트레일러 변경 또는 숨기기                                                                                                                                              | Git 및 속성        | 모든 파일          |
| [`attribution.pr`](#attribution-pr)                                                                   | 풀 요청 설명의 속성 라인 변경 또는 숨기기                                                                                                                                                          | Git 및 속성        | 모든 파일          |
| [`attribution.sessionUrl`](#attribution-sessionurl)                                                   | [클라우드](/docs/ko/claude-code-on-the-web) 및 [Remote Control](/docs/ko/remote-control) 커밋에서 claude.ai 세션 링크 생략                                                                                 | Git 및 속성        | 모든 파일          |
| [`autoCompactEnabled`](#autocompactenabled)                                                           | [자동 압축](/docs/ko/context-window) 끄기 또는 켜기                                                                                                                                              | 메모리 및 컨텍스트      | 모든 파일          |
| [`autoCompactWindow`](#autocompactwindow)                                                             | Claude Code가 [압축](/docs/ko/context-window)하기 전에 컨텍스트가 얼마나 찬지 설정                                                                                                                        | 메모리 및 컨텍스트      | 모든 파일          |
| [`autoConnectIde`](#autoconnectide)                                                                   | 외부 터미널에서 실행 중인 [VS Code](/docs/ko/vs-code) 또는 [JetBrains](/docs/ko/jetbrains#from-external-terminals) IDE에 자동으로 연결                                                                          | 전역 설정           | 전역 설정          |
| [`autoContinueAtUsageLimit`](#autocontinueatusagelimit)                                               | 열린 세션에서 대기하고 claude.ai 사용 제한이 재설정된 후 [작업 자동 계속](/docs/ko/interactive-mode#wait-for-a-usage-limit-to-reset)                                                                             | 인터페이스 및 터미널     | 사용자 또는 관리됨     |
| [`autoInstallIdeExtension`](#autoinstallideextension)                                                 | VS Code 터미널에서 [IDE 확장](/docs/ko/vs-code#install-the-extension)의 자동 설치 끄기                                                                                                               | 전역 설정           | 전역 설정          |
| [`autoMemoryDirectory`](#automemorydirectory)                                                         | [자동 메모리](/docs/ko/memory#auto-memory)를 선택한 디렉토리에 저장                                                                                                                                    | 메모리 및 컨텍스트      | 모든 파일          |
| [`autoMemoryEnabled`](#automemoryenabled)                                                             | [자동 메모리](/docs/ko/memory#auto-memory) 끄기 또는 켜기                                                                                                                                         | 메모리 및 컨텍스트      | 모든 파일          |
| [`autoMode`](#automode)                                                                               | [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode) 분류기에 자신의 허용 및 거부 규칙 추가                                                                                             | 권한 설정           | 사용자 또는 관리됨     |
| [`autoMode.classifyAllShell`](#automode-classifyallshell)                                             | 좁은 허용 규칙과 일치하는 것들을 포함하여 모든 셸 명령을 [자동 모드 분류기](/docs/ko/permission-modes#what-the-classifier-blocks-by-default)를 통해 전송                                                                   | 권한 설정           | 사용자 또는 관리됨     |
| [`autoScrollEnabled`](#autoscrollenabled)                                                             | 전체 화면 렌더링에서 [새 출력을 아래로 따라가기](/docs/ko/fullscreen#auto-follow)                                                                                                                          | 인터페이스 및 터미널     | 모든 파일          |
| [`autoUpdatesChannel`](#autoupdateschannel)                                                           | 최신 대신 안정적인 [릴리스 채널](/docs/ko/setup#configure-release-channel) 따르기                                                                                                                      | 업데이트 및 버전 관리    | 모든 파일          |
| [`availableModels`](#availablemodels)                                                                 | [사람들이 선택할 수 있는 모델 제한](/docs/ko/model-config#restrict-model-selection)                                                                                                                  | 모델 및 응답         | 모든 파일          |
| [`awaySummaryEnabled`](#awaysummaryenabled)                                                           | 터미널로 돌아올 때 표시되는 [세션 요약](/docs/ko/interactive-mode#session-recap) 끄기                                                                                                                    | 원격, 데스크톱, 알림    | 모든 파일          |
| [`awsAuthRefresh`](#awsauthrefresh)                                                                   | 자신의 명령으로 `.aws`에서 만료된 [Bedrock 자격증명](/docs/ko/amazon-bedrock#advanced-credential-configuration) 새로 고침                                                                                  | 인증 및 공급자        | 모든 파일          |
| [`awsCredentialExport`](#awscredentialexport)                                                         | 자신의 명령에서 JSON으로 [Bedrock 자격증명](/docs/ko/amazon-bedrock#advanced-credential-configuration) 제공                                                                                           | 인증 및 공급자        | 모든 파일          |
| [`axScreenReader`](#axscreenreader)                                                                   | [화면 판독기 친화적 출력](/docs/ko/accessibility) 렌더링                                                                                                                                            | 인터페이스 및 터미널     | 모든 파일          |
| [`bashEditDiffEnabled`](#basheditdiffenabled)                                                         | [Bash 명령이 변경한 파일](/docs/ko/hooks#bash)을 모든 권한 모드에서 기록                                                                                                                                  | 인터페이스 및 터미널     | 사용자 또는 관리됨     |
| [`bashOutputMaxChars`](#bashoutputmaxchars)                                                           | 성공한 명령의 [출력](/docs/ko/tools-reference#output-limits) 중 Claude가 인라인으로 받는 양 설정                                                                                                           | 메모리 및 컨텍스트      | 모든 파일          |
| [`blockedMarketplaces`](#blockedmarketplaces)                                                         | 조직의 [플러그인 마켓플레이스](/docs/ko/plugin-marketplaces) 소스 차단                                                                                                                                  | 플러그인 및 기술       | 관리됨            |
| [`browserExternalPageTools`](#browserexternalpagetools)                                               | [데스크톱](/docs/ko/desktop) 브라우저 창의 외부 페이지에서 Claude의 도구 끄기                                                                                                                                | 도구              | 관리됨            |
| [`channelsEnabled`](#channelsenabled)                                                                 | 조직의 [채널](/docs/ko/channels#enable-channels-for-your-organization) 허용                                                                                                                   | 플러그인 및 기술       | 관리됨            |
| [`claudeMd`](#claudemd)                                                                               | 관리되는 설정에서 조직 전체 [CLAUDE.md](/docs/ko/memory#deploy-organization-wide-claude-md) 지침 주입                                                                                                  | 메모리 및 컨텍스트      | 관리됨            |
| [`claudeMdExcludes`](#claudemdexcludes)                                                               | 메모리가 로드될 때 특정 [CLAUDE.md](/docs/ko/memory#exclude-specific-claude-md-files) 파일 건너뛰기                                                                                                    | 메모리 및 컨텍스트      | 모든 파일          |
| [`cleanupPeriodDays`](#cleanupperioddays)                                                             | Claude Code가 [트랜스크립트](/docs/ko/data-usage#data-retention)를 삭제하기 전에 유지하는 일 수 선택                                                                                                         | 개인정보 보호 및 원격 측정 | 모든 파일          |
| [`companyAnnouncements`](#companyannouncements)                                                       | 시작 시 조직의 공지사항 표시                                                                                                                                                                  | 인터페이스 및 터미널     | 모든 파일          |
| [`copyOnSelect`](#copyonselect)                                                                       | [전체 화면 렌더링](/docs/ko/fullscreen#use-the-mouse) 및 에이전트 보기에서 마우스로 선택한 텍스트의 자동 복사 끄기                                                                                                      | 전역 설정           | 전역 설정          |
| [`crossSessionInbound`](#crosssessioninbound)                                                         | Claude Code가 [다른 세션의 메시지](/docs/ko/cross-session-messaging#control-inbound-messages)를 전달하는지, 전달하지 않고 공지를 표시하는지, 거부하는지 선택                                                               | 에이전트, 세션, 워크트리  | 모든 파일          |
| [`defaultShell`](#defaultshell)                                                                       | [`!` 접두사](/docs/ko/interactive-mode#shell-mode-with-prefix)로 입력한 셸 명령을 Bash 또는 PowerShell 중 어느 것이 실행할지 선택                                                                              | 인터페이스 및 터미널     | 모든 파일          |
| [`deniedMcpServers`](#deniedmcpservers)                                                               | URL, 명령 또는 이름으로 특정 [MCP 서버](/docs/ko/mcp) 차단                                                                                                                                           | MCP             | 모든 파일          |
| [`desktopSessionCleanupPeriodDays`](#desktopsessioncleanupperioddays)                                 | [Claude Desktop 및 Cowork 트랜스크립트](/docs/ko/claude-directory#cleaned-up-automatically)의 나이 제한을 일 단위로 설정                                                                                  | 개인정보 보호 및 원격 측정 | 사용자 또는 관리됨     |
| [`dialogExpiry`](#dialogexpiry)                                                                       | Claude Code가 [Remote Control](/docs/ko/remote-control) 또는 SDK 호스트의 전달된 대화에 답하기를 기다리는 시간 설정                                                                                             | 인터페이스 및 터미널     | 사용자 또는 관리됨     |
| [`diffTool`](#difftool)                                                                               | Claude의 제안된 파일 변경이 [VS Code](/docs/ko/vs-code) 또는 [JetBrains](/docs/ko/jetbrains#features) diff 뷰어에서 열리는지 또는 터미널에 남아있는지 선택                                                                  | 전역 설정           | 전역 설정          |
| [`disableAgentView`](#disableagentview)                                                               | 백그라운드 에이전트 및 [에이전트 보기](/docs/ko/agent-view) 끄기                                                                                                                                         | 에이전트, 세션, 워크트리  | 모든 파일          |
| [`disableAllHooks`](#disableallhooks)                                                                 | [훅](/docs/ko/hooks), 사용자 정의 [상태 라인](/docs/ko/statusline), 사용자 정의 [`@` 파일 제안](/docs/ko/interactive-mode#quick-commands) 명령을 한 번에 끄기                                                               | 훅 및 자동화         | 모든 파일          |
| [`disableArtifact`](#disableartifact)                                                                 | 더 이상 사용되지 않음; `enableArtifact`를 사용하여 [Artifact 도구](/docs/ko/artifacts) 끄기                                                                                                              | 원격, 데스크톱, 알림    | 모든 파일          |
| [`disableAutoMode`](#disableautomode)                                                                 | 권한 모드 사이클에서 [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode) 제거                                                                                                     | 권한 설정           | 모든 파일          |
| [`disableBrowserExternalNavigation`](#disablebrowserexternalnavigation)                               | [데스크톱](/docs/ko/desktop) 브라우저 창을 사람 및 Claude의 localhost로 제한                                                                                                                            | 도구              | 관리됨            |
| [`disableBundledSkills`](#disablebundledskills)                                                       | Claude Code에 포함된 [기술](/docs/ko/skills#bundled-skills) 및 [워크플로우](/docs/ko/workflows) 끄기                                                                                                      | 플러그인 및 기술       | 모든 파일          |
| [`disableClaudeAiConnectors`](#disableclaudeaiconnectors)                                             | [claude.ai 커넥터](/docs/ko/mcp#disable-claude-ai-connectors)를 끄기 때문에 Claude Code가 가져오지 않음                                                                                                | MCP             | 모든 파일          |
| [`disableCommandPluginSources`](#disablecommandpluginsources)                                         | 마켓플레이스 선언 명령을 실행하여 설치하는 [플러그인](/docs/ko/plugins) 차단                                                                                                                                    | 플러그인 및 기술       | 관리됨            |
| [`disableDeepLinkRegistration`](#disabledeeplinkregistration)                                         | Claude Code가 [`claude-cli://` 핸들러](/docs/ko/deep-links) 등록 중지                                                                                                                          | 원격, 데스크톱, 알림    | 모든 파일          |
| [`disableDesktopLocalSessions`](#disabledesktoplocalsessions)                                         | 기기에서 실행되는 [Desktop Code 세션](/docs/ko/desktop#local-sessions-on-managed-devices) 끄기, SSH를 다른 호스트 및 클라우드로 남겨두기                                                                           | 원격, 데스크톱, 알림    | 관리됨            |
| [`disabledMcpjsonServers`](#disabledmcpjsonservers)                                                   | 프로젝트의 [`.mcp.json`](/docs/ko/mcp#project-scope)에서 특정 서버 거부                                                                                                                             | MCP             | 모든 파일          |
| [`disableMobileSimulatorTools`](#disablemobilesimulatortools)                                         | [데스크톱](/docs/ko/desktop) iOS 시뮬레이터 창에서 Claude의 도구 차단                                                                                                                                   | 도구              | 관리됨            |
| [`disableRemoteControl`](#disableremotecontrol)                                                       | [Remote Control](/docs/ko/remote-control)을 시작할 수 있는 모든 곳에서 끄기                                                                                                                          | 원격, 데스크톱, 알림    | 모든 파일          |
| [`disableSideloadFlags`](#disablesideloadflags)                                                       | [플러그인](/docs/ko/plugins), [subagent](/docs/ko/sub-agents), [MCP 서버](/docs/ko/mcp)를 사이드로드하는 CLI 플래그 거부                                                                                            | 엔터프라이즈 및 관리 설정  | 관리됨            |
| [`disableSkillShellExecution`](#disableskillshellexecution)                                           | [기술](/docs/ko/skills) 및 사용자 정의 명령이 인라인 셸 실행 중지                                                                                                                                         | 플러그인 및 기술       | 모든 파일          |
| [`disableWorkflows`](#disableworkflows)                                                               | 모든 사람을 위해 [동적 워크플로우](/docs/ko/workflows) 끄기; 자신을 위해 `enableWorkflows` 사용                                                                                                               | 훅 및 자동화         | 모든 파일          |
| [`editorMode`](#editormode)                                                                           | 입력 프롬프트에서 [vim 키 바인딩](/docs/ko/interactive-mode#vim-editor-mode) 사용                                                                                                                    | 인터페이스 및 터미널     | 모든 파일          |
| [`effortLevel`](#effortlevel)                                                                         | 저장된 수준이 없는 모델의 기본 [노력 수준](/docs/ko/model-config#adjust-effort-level) 설정                                                                                                                | 모델 및 응답         | 모든 파일          |
| [`emojiCompletionEnabled`](#emojicompletionenabled)                                                   | 프롬프트 입력에서 [`:shortcode:` 이모지 제안 및 교체](/docs/ko/interactive-mode#emoji-shortcodes) 끄기                                                                                                   | 인터페이스 및 터미널     | 모든 파일          |
| [`enableAllProjectMcpServers`](#enableallprojectmcpservers)                                           | 프롬프트 없이 프로젝트 [`.mcp.json`](/docs/ko/mcp#project-server-approvals-and-workspace-trust) 파일의 모든 서버 승인                                                                                     | MCP             | 모든 파일          |
| [`enableArtifact`](#enableartifact)                                                                   | 모든 파일에서 `false`로 [Artifact 도구](/docs/ko/artifacts) 끄기; 어떤 파일도 다시 켤 수 없음                                                                                                                | 원격, 데스크톱, 알림    | 모든 파일          |
| [`enabledMcpjsonServers`](#enabledmcpjsonservers)                                                     | 프로젝트의 [`.mcp.json`](/docs/ko/mcp#project-server-approvals-and-workspace-trust)에서 특정 서버 승인                                                                                              | MCP             | 모든 파일          |
| [`enabledPlugins`](#enabledplugins)                                                                   | 범위별로 개별 [플러그인](/docs/ko/plugins) 켜기 또는 끄기                                                                                                                                              | 플러그인 및 기술       | 모든 파일          |
| [`enableWorkflows`](#enableworkflows)                                                                 | 계획의 기본값에 대해 [동적 워크플로우](/docs/ko/workflows) 켜기 또는 끄기                                                                                                                                    | 훅 및 자동화         | 모든 파일          |
| [`enforceAvailableModels`](#enforceavailablemodels)                                                   | [`/model` 기본 선택](/docs/ko/model-config#enforce-the-allowlist-for-the-default-model)을 `availableModels` 허용 목록 내에 유지                                                                     | 모델 및 응답         | 모든 파일          |
| [`env`](#env)                                                                                         | 모든 세션 및 해당 부프로세스에 대해 [환경 변수](/docs/ko/env-vars#in-settings-files) 설정                                                                                                                   | 메모리 및 컨텍스트      | 모든 파일          |
| [`externalEditorContext`](#externaleditorcontext)                                                     | [Ctrl+G](/docs/ko/interactive-mode#general-controls)를 눌러 편집할 때 Claude의 마지막 응답을 주석으로 표시                                                                                                 | 전역 설정           | 전역 설정          |
| [`extraKnownMarketplaces`](#extraknownmarketplaces)                                                   | 저장소 또는 조직의 [마켓플레이스](/docs/ko/plugin-marketplaces) 등록                                                                                                                                   | 플러그인 및 기술       | 모든 파일          |
| [`fallbackModel`](#fallbackmodel)                                                                     | 기본이 과부하일 때 [백업 모델](/docs/ko/model-config#fallback-model-chains) 이름 지정                                                                                                                  | 모델 및 응답         | 모든 파일          |
| [`fastMode`](#fastmode)                                                                               | 사용 가능한 세션에 대해 [빠른 모드](/docs/ko/fast-mode) 켜기                                                                                                                                           | 모델 및 응답         | 모든 파일          |
| [`fastModePerSessionOptIn`](#fastmodepersessionoptin)                                                 | 사람들이 각 세션에서 [빠른 모드](/docs/ko/fast-mode) 켜도록 요구                                                                                                                                         | 모델 및 응답         | 모든 파일          |
| [`feedbackDrafts`](#feedbackdrafts)                                                                   | Claude가 [피드백 초안](/docs/ko/tools-reference#sendfeedback-tool-behavior)을 검토하도록 대기열에 넣는지 제어                                                                                               | 개인정보 보호 및 원격 측정 | 사용자 또는 관리됨     |
| [`feedbackSurveyRate`](#feedbacksurveyrate)                                                           | [세션 품질 설문조사](/docs/ko/data-usage#session-quality-surveys)가 나타나는 빈도 변경                                                                                                                  | 개인정보 보호 및 원격 측정 | 모든 파일          |
| [`fileCheckpointingEnabled`](#filecheckpointingenabled)                                               | [`/rewind`](/docs/ko/checkpointing)가 복원하는 파일 스냅샷 끄기 또는 켜기                                                                                                                              | 메모리 및 컨텍스트      | 모든 파일          |
| [`fileSuggestion`](#filesuggestion)                                                                   | 자신의 명령에서 [`@` 파일 자동 완성](/docs/ko/interactive-mode#quick-commands) 제공                                                                                                                   | 인터페이스 및 터미널     | 모든 파일          |
| [`footerLinksRegexes`](#footerlinksregexes)                                                           | 출력의 문제 또는 검토 ID를 입력 상자 아래의 [클릭 가능한 링크](/docs/ko/statusline#clickable-links)로 만들기                                                                                                       | 인터페이스 및 터미널     | 사용자 또는 관리됨     |
| [`forceLoginGatewayUrl`](#forcelogingatewayurl)                                                       | 로그인 화면이 연결하는 [게이트웨이 URL](/docs/ko/claude-apps-gateway#set-the-gateway-url) 설정                                                                                                          | 인증 및 공급자        | 관리됨            |
| [`forceLoginMethod`](#forceloginmethod)                                                               | [로그인 제한](/docs/ko/authentication#restrict-login-to-your-organization)을 claude.ai, Claude Console 또는 [클라우드 게이트웨이](/docs/ko/claude-apps-gateway)로 제한                                          | 인증 및 공급자        | 모든 파일          |
| [`forceLoginOrgUUID`](#forceloginorguuid)                                                             | [claude.ai 로그인을 조직에 고정](/docs/ko/authentication#restrict-login-to-your-organization); 관리되는 소스만 적용                                                                                      | 인증 및 공급자        | 모든 파일          |
| [`forceRemoteSettingsRefresh`](#forceremotesettingsrefresh)                                           | [서버 관리 설정](/docs/ko/server-managed-settings)이 새로 가져올 때까지 시작 차단                                                                                                                         | 엔터프라이즈 및 관리 설정  | 관리됨            |
| [`gatewayInternalNetworks`](#gatewayinternalnetworks)                                                 | `/login`이 조직이 내부적으로 사용하는 공개 IPv4 공간의 [클라우드 게이트웨이](/docs/ko/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own)에 도달하도록 허용                                              | 인증 및 공급자        | 관리됨            |
| [`gcpAuthRefresh`](#gcpauthrefresh)                                                                   | 자신의 명령으로 [Google Cloud 자격증명](/docs/ko/google-vertex-ai#advanced-credential-configuration) 새로 고침                                                                                        | 인증 및 공급자        | 모든 파일          |
| [`hooks`](#hooks)                                                                                     | Claude Code의 수명 주기의 지점에서 [훅](/docs/ko/hooks)으로 자신의 명령 실행                                                                                                                               | 훅 및 자동화         | 모든 파일          |
| [`httpHookAllowedEnvVars`](#httphookallowedenvvars)                                                   | [HTTP hooks](/docs/ko/hooks)가 헤더에 넣을 수 있는 env 변수 제한                                                                                                                                    | 훅 및 자동화         | 모든 파일          |
| [`includeCoAuthoredBy`](#includecoauthoredby)                                                         | 더 이상 사용되지 않음; `attribution`을 사용하여 커밋 및 PR 속성 숨기기 또는 변경                                                                                                                            | Git 및 속성        | 모든 파일          |
| [`includeGitInstructions`](#includegitinstructions)                                                   | Claude의 컨텍스트에서 기본 제공 커밋 및 PR 지침 제거                                                                                                                                                | Git 및 속성        | 모든 파일          |
| [`inputNeededNotifEnabled`](#inputneedednotifenabled)                                                 | Claude가 당신을 기다리고 있을 때 [푸시 알림](/docs/ko/remote-control#mobile-push-notifications) 받기                                                                                                    | 원격, 데스크톱, 알림    | 모든 파일          |
| [`isolatePeerMachines`](#isolatepeermachines)                                                         | Claude가 [다른 기계의 세션 중 하나에 메시지를 보내기](/docs/ko/cross-session-messaging#require-approval-for-cross-machine-messages) 전에 물어보기                                                               | 에이전트, 세션, 워크트리  | 모든 파일          |
| [`keybindingFlavor`](#keybindingflavor)                                                               | 더 이상 사용되지 않으며 효과 없음; 단어 편집 바로 가기는 항상 [readline 규칙](/docs/ko/interactive-mode#make-ctrl-w-delete-back-to-whitespace)을 따름                                                                | 인터페이스 및 터미널     | 모든 파일          |
| [`language`](#language)                                                                               | Claude가 영어 이외의 언어로 응답하도록 함                                                                                                                                                        | 모델 및 응답         | 모든 파일          |
| [`managedMcpServers`](#managedmcpservers)                                                             | 사용자가 추가하는 것과 함께 모든 사용자에게 원격 [MCP 서버](/docs/ko/managed-mcp#provide-servers-through-managed-settings) 제공                                                                                 | MCP             | 관리됨            |
| [`managedSourcesBehavior`](#managedsourcesbehavior)                                                   | 배포하는 모든 [관리되는 소스](/docs/ko/managed-settings#how-claude-code-combines-managed-sources)를 구성하는 대신 가장 높은 우선순위 하나만 사용                                                                       | 엔터프라이즈 및 관리 설정  | 관리됨            |
| [`maxEffortLevel`](#maxeffortlevel)                                                                   | 모든 모델 또는 모델별로, 모든 공급자에서 [노력 수준](/docs/ko/model-config#adjust-effort-level) 제한                                                                                                          | 모델 및 응답         | 모든 파일          |
| [`minimumVersion`](#minimumversion)                                                                   | [자동 업데이트](/docs/ko/setup#pin-a-minimum-version)가 버전 이하의 것을 설치하지 않도록 유지                                                                                                                 | 업데이트 및 버전 관리    | 모든 파일          |
| [`model`](#model)                                                                                     | Claude Code가 시작하는 [모델](/docs/ko/model-config#set-a-default-model-for-new-sessions) 변경                                                                                                  | 모델 및 응답         | 모든 파일          |
| [`modelOverrides`](#modeloverrides)                                                                   | [모델 ID를 공급자의 ID(예: Bedrock ARN)에 매핑](/docs/ko/model-config#override-model-ids-per-version)                                                                                             | 모델 및 응답         | 모든 파일          |
| [`modelPicker`](#modelpicker)                                                                         | [`/model` 선택기](/docs/ko/model-config#available-models)가 나열하는 모델을 선택하고, 자신의 순서 및 자신의 레이블로 선택                                                                                            | 모델 및 응답         | 사용자 또는 관리됨     |
| [`modelPricing`](#modelpricing)                                                                       | 목록 가격 대신 조직의 계약 요금으로 지출 보고                                                                                                                                                        | 모델 및 응답         | 관리됨            |
| [`modelSettings`](#modelsettings)                                                                     | 모델별로 저장된 [노력 수준](/docs/ko/model-config#adjust-effort-level)을 유지하거나 한 모델의 노력 제한                                                                                                         | 모델 및 응답         | 모든 파일          |
| [`otelHeadersHelper`](#otelheadershelper)                                                             | 자신의 명령으로 회전하는 [OpenTelemetry](/docs/ko/monitoring-usage#dynamic-headers) 헤더 생성                                                                                                         | 인증 및 공급자        | 모든 파일          |
| [`outputStyle`](#outputstyle)                                                                         | [출력 스타일](/docs/ko/output-styles)로 Claude의 역할, 톤, 출력 형식 변경                                                                                                                              | 모델 및 응답         | 모든 파일          |
| [`parentSettingsBehavior`](#parentsettingsbehavior)                                                   | [관리되는 설정](/docs/ko/managed-settings)을 배포할 때 [SDK 또는 IDE 호스트](/docs/ko/managed-settings#let-an-embedding-host-add-policy)가 전달하는 제한 사항 적용 또는 삭제                                               | 엔터프라이즈 및 관리 설정  | 관리됨            |
| [`permissionExplainerEnabled`](#permissionexplainerenabled)                                           | v2.1.257에서 제거됨, 셸 권한 프롬프트의 `Ctrl+E` 명령 설명과 함께                                                                                                                                     | 전역 설정           | 전역 설정          |
| [`permissions`](#permissions)                                                                         | 허용, 물어보기, 거부 규칙 및 시작 [권한 모드](/docs/ko/permission-modes) 설정                                                                                                                             | 권한 설정           | 모든 파일          |
| [`permissions.additionalDirectories`](#permissions-additionaldirectories)                             | Claude에게 [현재 디렉토리 외부의 디렉토리](/docs/ko/permissions#working-directories)에 대한 파일 액세스 제공                                                                                                    | 권한 설정           | 모든 파일          |
| [`permissions.allow`](#permissions-allow)                                                             | 나열된 [도구 사용](/docs/ko/permissions#permission-rule-syntax)을 프롬프트 없이 승인                                                                                                                   | 권한 설정           | 모든 파일          |
| [`permissions.ask`](#permissions-ask)                                                                 | 나열된 [도구 사용](/docs/ko/permissions#permission-rule-syntax) 전에 항상 프롬프트                                                                                                                    | 권한 설정           | 모든 파일          |
| [`permissions.blockReadsOutsideWorkingDirectories`](#permissions-blockreadsoutsideworkingdirectories) | 파일 도구가 모든 권한 모드에서 [작업 디렉토리](/docs/ko/permissions#working-directories) 외부의 읽기를 거부하도록 만들기                                                                                                | 권한 설정           | 모든 파일          |
| [`permissions.defaultMode`](#permissions-defaultmode)                                                 | 새 세션이 시작하는 [권한 모드](/docs/ko/permission-modes#which-mode-a-session-starts-in) 설정                                                                                                        | 권한 설정           | 모든 파일          |
| [`permissions.deny`](#permissions-deny)                                                               | 나열된 [도구 사용](/docs/ko/permissions#permission-rule-syntax) 차단, 비밀을 보유한 파일의 읽기 포함                                                                                                         | 권한 설정           | 모든 파일          |
| [`permissions.disableBypassPermissionsMode`](#permissions-disablebypasspermissionsmode)               | 누구도 [bypassPermissions 모드](/docs/ko/permission-modes#skip-all-checks-with-bypasspermissions-mode)에 들어가지 못하도록 방지                                                                        | 권한 설정           | 모든 파일          |
| [`plansDirectory`](#plansdirectory)                                                                   | [계획 모드](/docs/ko/permission-modes#analyze-before-you-edit-with-plan-mode)가 계획 파일을 쓰는 위치 선택                                                                                             | 메모리 및 컨텍스트      | 모든 파일          |
| [`pluginConfigs`](#pluginconfigs)                                                                     | [플러그인](/docs/ko/plugins)의 구성 대화에서 제공한 답변 저장                                                                                                                                            | 플러그인 및 기술       | 사용자 또는 관리됨     |
| [`pluginSuggestionMarketplaces`](#pluginsuggestionmarketplaces)                                       | `/plugin`에서 플러그인 설치 제안을 표시할 수 있는 [마켓플레이스](/docs/ko/plugin-marketplaces#managed-marketplace-restrictions) 선택                                                                            | 플러그인 및 기술       | 관리됨            |
| [`pluginTrustMessage`](#plugintrustmessage)                                                           | [플러그인](/docs/ko/plugins) 신뢰 경고에 자신의 텍스트 추가                                                                                                                                             | 플러그인 및 기술       | 관리됨            |
| [`policyHelper`](#policyhelper)                                                                       | 시작 시 [관리되는 설정](/docs/ko/managed-settings#compute-the-policy-with-a-helper-program)을 계산하는 실행 파일 실행                                                                                      | 엔터프라이즈 및 관리 설정  | 관리됨            |
| [`policyHelper.path`](#policyhelper-path)                                                             | Claude Code가 실행하는 [도우미 실행 파일](/docs/ko/managed-settings#compute-the-policy-with-a-helper-program) 이름 지정                                                                                | 엔터프라이즈 및 관리 설정  | 관리됨            |
| [`policyHelper.refreshIntervalMs`](#policyhelper-refreshintervalms)                                   | 백그라운드에서 간격으로 [도우미](/docs/ko/managed-settings#compute-the-policy-with-a-helper-program) 다시 실행                                                                                           | 엔터프라이즈 및 관리 설정  | 관리됨            |
| [`policyHelper.timeoutMs`](#policyhelper-timeoutms)                                                   | Claude Code가 [도우미](/docs/ko/managed-settings#compute-the-policy-with-a-helper-program)를 기다리는 시간 설정                                                                                     | 엔터프라이즈 및 관리 설정  | 관리됨            |
| [`preferredNotifChannel`](#preferrednotifchannel)                                                     | 작업 완료를 위해 [터미널 벨 또는 데스크톱 알림](/docs/ko/terminal-config#get-a-terminal-bell-or-notification) 선택                                                                                          | 원격, 데스크톱, 알림    | 모든 파일          |
| [`prefersReducedMotion`](#prefersreducedmotion)                                                       | [스피너, 반짝임, 플래시 애니메이션 줄이기 또는 끄기](/docs/ko/accessibility#accessibility-settings)                                                                                                         | 인터페이스 및 터미널     | 모든 파일          |
| [`processWrapper`](#processwrapper)                                                                   | Claude Code의 백그라운드 프로세스를 macOS 및 Linux의 [기업 런처](/docs/ko/corporate-launcher)를 통해 실행                                                                                                    | 에이전트, 세션, 워크트리  | 사용자 또는 관리됨     |
| [`promptCacheTtl`](#promptcachettl)                                                                   | 주 대화의 [프롬프트 캐시 수명](/docs/ko/prompt-caching#cache-lifetime) 선택                                                                                                                          | 모델 및 응답         | 모든 파일          |
| [`promptSuggestionEnabled`](#promptsuggestionenabled)                                                 | 입력 상자의 회색 [프롬프트 제안](/docs/ko/interactive-mode#prompt-suggestions) 숨기기                                                                                                                  | 인터페이스 및 터미널     | 모든 파일          |
| [`prUrlTemplate`](#prurltemplate)                                                                     | PR 링크를 github.com 대신 내부 코드 검토 도구로 지정                                                                                                                                              | Git 및 속성        | 모든 파일          |
| [`remote.defaultEnvironmentId`](#remote-defaultenvironmentid)                                         | `claude --cloud`의 기본 [클라우드 환경](/docs/ko/cloud-environments) 선택; 자체 호스팅 `ccpool_` ID는 사용자 및 관리 설정 및 `--settings`에서만 읽음                                                                  | 원격, 데스크톱, 알림    | 모든 파일          |
| [`remoteControlAtStartup`](#remotecontrolatstartup)                                                   | 세션이 시작할 때 [Remote Control](/docs/ko/remote-control#enable-remote-control-for-all-sessions) 자동 연결                                                                                       | 원격, 데스크톱, 알림    | 모든 파일          |
| [`requiredMaximumVersion`](#requiredmaximumversion)                                                   | [조직이 허용하는 것보다 최신 버전에서 시작 거부](/docs/ko/setup#pin-a-minimum-version)                                                                                                                     | 업데이트 및 버전 관리    | 관리됨            |
| [`requiredMinimumVersion`](#requiredminimumversion)                                                   | [조직이 요구하는 것보다 이전 버전에서 시작 거부](/docs/ko/setup#pin-a-minimum-version)                                                                                                                     | 업데이트 및 버전 관리    | 관리됨            |
| [`respectGitignore`](#respectgitignore)                                                               | gitignored 파일을 [`@` 파일 선택기](/docs/ko/interactive-mode#quick-commands)에서 제외                                                                                                             | 인터페이스 및 터미널     | 모든 파일          |
| [`respondToBashCommands`](#respondtobashcommands)                                                     | [`!` 셸 명령](/docs/ko/interactive-mode#shell-mode-with-prefix) 실행 후 Claude가 응답하지 않도록 중지                                                                                                  | 인터페이스 및 터미널     | 모든 파일          |
| [`sandbox`](#sandbox)                                                                                 | macOS, Linux, WSL2에서 [Bash 명령](/docs/ko/sandboxing)을 파일 시스템 및 네트워크에서 격리                                                                                                                | 샌드박스 설정         | 모든 파일          |
| [`sandbox.allowAppleEvents`](#sandbox-allowappleevents)                                               | [샌드박스](/docs/ko/sandboxing) 명령이 macOS에서 Apple Events를 보내도록 허용                                                                                                                          | 샌드박스 설정         | 사용자 또는 관리됨     |
| [`sandbox.allowUnsandboxedCommands`](#sandbox-allowunsandboxedcommands)                               | Claude가 [샌드박스](/docs/ko/sandboxing#the-unsandboxed-retry-escape-hatch) 외부에서 차단된 명령을 다시 시도하도록 허용하거나 금지                                                                                  | 샌드박스 설정         | 모든 파일          |
| [`sandbox.autoAllowBashIfSandboxed`](#sandbox-autoallowbashifsandboxed)                               | [샌드박스](/docs/ko/sandboxing#auto-allow-mode) 명령을 권한 프롬프트 없이 실행                                                                                                                          | 샌드박스 설정         | 모든 파일          |
| [`sandbox.bwrapPath`](#sandbox-bwrappath)                                                             | [샌드박스](/docs/ko/sandboxing)를 `PATH` 외부의 bubblewrap 바이너리로 지정                                                                                                                            | 샌드박스 설정         | 관리됨            |
| [`sandbox.credentials`](#sandbox-credentials)                                                         | [샌드박스](/docs/ko/sandboxing#protect-credentials) 내부의 자격증명 파일 및 변수 숨기기 또는 마스킹                                                                                                            | 샌드박스 설정         | 모든 파일          |
| [`sandbox.credentials.allowPlaintextInject`](#sandbox-credentials-allowplaintextinject)               | [마스킹된 자격증명](/docs/ko/sandboxing#mask-credentials)이 신뢰할 수 있는 테스트 네트워크의 일반 HTTP 서비스에 도달하도록 허용                                                                                            | 샌드박스 설정         | 사용자 또는 관리됨     |
| [`sandbox.credentials.awsPairs`](#sandbox-credentials-awspairs)                                       | 사용자 정의 이름의 AWS 키 변수를 [재서명](/docs/ko/sandboxing#re-sign-aws-requests)을 위한 하나의 자격증명으로 연결                                                                                                 | 샌드박스 설정         | 사용자 또는 관리됨     |
| [`sandbox.credentials.envVars`](#sandbox-credentials-envvars)                                         | [샌드박스](/docs/ko/sandboxing#mask-environment-variables) 내부의 환경 변수 설정 해제 또는 마스킹                                                                                                          | 샌드박스 설정         | 모든 파일          |
| [`sandbox.credentials.files`](#sandbox-credentials-files)                                             | [샌드박스](/docs/ko/sandboxing#mask-credential-files) 내부의 자격증명 파일 읽기 차단 또는 마스킹                                                                                                             | 샌드박스 설정         | 모든 파일          |
| [`sandbox.credentials.sigv4`](#sandbox-credentials-sigv4)                                             | 스트리밍, 사전 서명 또는 [SigV4A AWS 요청](/docs/ko/sandboxing#re-sign-aws-requests)이 실패하거나 통과하는지 선택                                                                                               | 샌드박스 설정         | 사용자 또는 관리됨     |
| [`sandbox.enabled`](#sandbox-enabled)                                                                 | macOS, Linux, WSL2에서 [Bash 샌드박싱](/docs/ko/sandboxing#get-started) 켜기                                                                                                                   | 샌드박스 설정         | 모든 파일          |
| [`sandbox.enableWeakerNestedSandbox`](#sandbox-enableweakernestedsandbox)                             | Linux [샌드박스](/docs/ko/sandboxing)를 권한 없는 컨테이너 내부에서 실행                                                                                                                                  | 샌드박스 설정         | 모든 파일          |
| [`sandbox.enableWeakerNetworkIsolation`](#sandbox-enableweakernetworkisolation)                       | `gh`, `gcloud`, `terraform`이 macOS의 [샌드박스](/docs/ko/sandboxing#troubleshooting) 내부에서 MITM 프록시 뒤의 TLS를 확인하도록 허용                                                                         | 샌드박스 설정         | 모든 파일          |
| [`sandbox.excludedCommands`](#sandbox-excludedcommands)                                               | 항상 [샌드박스](/docs/ko/sandboxing) 외부에서 실행되는 명령 이름 지정                                                                                                                                      | 샌드박스 설정         | 모든 파일          |
| [`sandbox.failIfUnavailable`](#sandbox-failifunavailable)                                             | [샌드박스](/docs/ko/sandboxing)를 할 수 없을 때 샌드박스 없이 실행하는 대신 시작 거부                                                                                                                            | 샌드박스 설정         | 모든 파일          |
| [`sandbox.filesystem`](#sandbox-filesystem)                                                           | [샌드박스](/docs/ko/sandboxing#filesystem-isolation) 명령이 읽고 쓸 수 있는 경로 제어                                                                                                                   | 샌드박스 설정         | 모든 파일          |
| [`sandbox.filesystem.allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly)       | 개발자가 [조직이 차단한 읽기 경로](/docs/ko/sandboxing#keep-developers-from-widening-the-policy)를 다시 열지 못하도록 중지                                                                                      | 샌드박스 설정         | 관리됨            |
| [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread)                                       | [`denyRead`](#sandbox-filesystem-denyread)가 차단하는 영역 내에서 읽기 다시 열기                                                                                                                  | 샌드박스 설정         | 모든 파일          |
| [`sandbox.filesystem.allowWrite`](#sandbox-filesystem-allowwrite)                                     | [샌드박스](/docs/ko/sandboxing) 명령이 쓸 수 있는 경로 추가                                                                                                                                           | 샌드박스 설정         | 모든 파일          |
| [`sandbox.filesystem.denyRead`](#sandbox-filesystem-denyread)                                         | [샌드박스](/docs/ko/sandboxing) 명령이 특정 경로 읽기 차단                                                                                                                                            | 샌드박스 설정         | 모든 파일          |
| [`sandbox.filesystem.denyWrite`](#sandbox-filesystem-denywrite)                                       | [샌드박스](/docs/ko/sandboxing) 명령이 특정 경로에 쓰기 차단                                                                                                                                           | 샌드박스 설정         | 모든 파일          |
| [`sandbox.filesystem.disabled`](#sandbox-filesystem-disabled)                                         | 네트워크 격리를 유지하면서 [파일 시스템 격리](/docs/ko/sandboxing#disable-filesystem-isolation) 끄기                                                                                                        | 샌드박스 설정         | 사용자 또는 관리됨     |
| [`sandbox.ignoreViolations`](#sandbox-ignoreviolations)                                               | 명령이 프로브할 것으로 예상되는 경로에 대한 위반 보고 침묵                                                                                                                                                 | 샌드박스 설정         | 모든 파일          |
| [`sandbox.network`](#sandbox-network)                                                                 | [샌드박스](/docs/ko/sandboxing#network-isolation) 명령이 도달하는 호스트, 포트, 소켓 제어                                                                                                                  | 샌드박스 설정         | 모든 파일          |
| [`sandbox.network.allowAllUnixSockets`](#sandbox-network-allowallunixsockets)                         | [샌드박스](/docs/ko/sandboxing) 명령이 모든 Unix 소켓에 연결하도록 허용                                                                                                                                   | 샌드박스 설정         | 모든 파일          |
| [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains)                                   | [샌드박스](/docs/ko/sandboxing) 명령이 프롬프트하지 않도록 도메인 사전 허용                                                                                                                                   | 샌드박스 설정         | 모든 파일          |
| [`sandbox.network.allowLocalBinding`](#sandbox-network-allowlocalbinding)                             | [샌드박스](/docs/ko/sandboxing) 명령이 macOS의 localhost 포트에 바인드하도록 허용                                                                                                                         | 샌드박스 설정         | 모든 파일          |
| [`sandbox.network.allowMachLookup`](#sandbox-network-allowmachlookup)                                 | macOS [샌드박스](/docs/ko/sandboxing) 도구(iOS 시뮬레이터 또는 Playwright 등)가 XPC 서비스에 도달하도록 허용                                                                                                     | 샌드박스 설정         | 모든 파일          |
| [`sandbox.network.allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly)                 | 네트워크 허용 목록을 [관리되는 설정](/docs/ko/sandboxing#keep-developers-from-widening-the-policy)으로 잠금                                                                                               | 샌드박스 설정         | 관리됨            |
| [`sandbox.network.allowUnixSockets`](#sandbox-network-allowunixsockets)                               | [샌드박스](/docs/ko/sandboxing) 명령이 macOS에서 사용할 수 있는 Unix 소켓 경로 나열                                                                                                                         | 샌드박스 설정         | 모든 파일          |
| [`sandbox.network.deniedDomains`](#sandbox-network-denieddomains)                                     | [샌드박스](/docs/ko/sandboxing) 명령의 도메인 차단, 허용된 와일드카드 내부도                                                                                                                                  | 샌드박스 설정         | 모든 파일          |
| [`sandbox.network.httpProxyPort`](#sandbox-network-httpproxyport)                                     | [샌드박스](/docs/ko/sandboxing#custom-proxy-configuration) HTTP 트래픽을 자신의 프록시를 통해 라우팅                                                                                                       | 샌드박스 설정         | 모든 파일          |
| [`sandbox.network.socksProxyPort`](#sandbox-network-socksproxyport)                                   | [샌드박스](/docs/ko/sandboxing#custom-proxy-configuration) SOCKS 트래픽을 자신의 프록시를 통해 라우팅                                                                                                      | 샌드박스 설정         | 모든 파일          |
| [`sandbox.network.strictAllowlist`](#sandbox-network-strictallowlist)                                 | 프롬프트 대신 [허용 목록](/docs/ko/sandboxing#network-isolation) 외부의 호스트 거부                                                                                                                      | 샌드박스 설정         | 사용자 또는 관리됨     |
| [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate)                                       | [샌드박스](/docs/ko/sandboxing#network-isolation) 프록시가 TLS를 종료하여 HTTPS 요청을 읽을 수 있도록 함                                                                                                      | 샌드박스 설정         | 사용자 또는 관리됨     |
| [`sandbox.ripgrep`](#sandbox-ripgrep)                                                                 | [샌드박스](/docs/ko/sandboxing) 내부에서 자신의 ripgrep 바이너리 사용                                                                                                                                   | 샌드박스 설정         | 사용자 또는 관리됨     |
| [`sandbox.socatPath`](#sandbox-socatpath)                                                             | [샌드박스](/docs/ko/sandboxing) 프록시를 `PATH` 외부의 `socat` 바이너리로 지정                                                                                                                           | 샌드박스 설정         | 관리됨            |
| [`showClearContextOnPlanAccept`](#showclearcontextonplanaccept)                                       | [계획 수락 화면](/docs/ko/permission-modes#review-and-approve-a-plan)에 "컨텍스트 지우기" 옵션 표시                                                                                                      | 인터페이스 및 터미널     | 모든 파일          |
| [`showThinkingSummaries`](#showthinkingsummaries)                                                     | 축소된 스텁 대신 Claude의 [사고](/docs/ko/model-config#extended-thinking) 요약 보기                                                                                                                  | 모델 및 응답         | 모든 파일          |
| [`showTurnDuration`](#showturnduration)                                                               | 각 응답 후 "Cooked for" 기간 숨기기                                                                                                                                                        | 인터페이스 및 터미널     | 모든 파일          |
| [`skillListingBudgetFraction`](#skilllistingbudgetfraction)                                           | [기술 나열](/docs/ko/skills#skill-descriptions-are-cut-short)을 위해 더 많거나 적은 컨텍스트 예약                                                                                                         | 메모리 및 컨텍스트      | 모든 파일          |
| [`skillListingMaxDescChars`](#skilllistingmaxdescchars)                                               | [기술 나열](/docs/ko/skills#skill-descriptions-are-cut-short)에서 각 기술의 설명 길이 제한                                                                                                             | 메모리 및 컨텍스트      | 모든 파일          |
| [`skillOverrides`](#skilloverrides)                                                                   | [기술을 숨기거나 축소](/docs/ko/skills#override-skill-visibility-from-settings) SKILL.md 편집 없이                                                                                                  | 플러그인 및 기술       | 모든 파일          |
| [`skipAutoPermissionPrompt`](#skipautopermissionprompt)                                               | 기본 제공 기본값이 아닌 자신이 [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)에 처음 들어갈 때 Claude Code가 표시하는 일회성 공지 건너뛰기                                                          | 권한 설정           | 사용자 또는 관리됨     |
| [`skipDangerousModePermissionPrompt`](#skipdangerousmodepermissionprompt)                             | [bypassPermissions 모드](/docs/ko/permission-modes#skip-all-checks-with-bypasspermissions-mode) 전 확인 대화 건너뛰기                                                                             | 권한 설정           | 사용자, 로컬 또는 관리됨 |
| [`skipWebFetchPreflight`](#skipwebfetchpreflight)                                                     | Anthropic에 연결할 수 없을 때 [WebFetch 호스트명 확인](/docs/ko/tools-reference#webfetch-tool-behavior) 건너뛰기                                                                                         | 개인정보 보호 및 원격 측정 | 모든 파일          |
| [`spellcheck`](#spellcheck)                                                                           | 설치한 [맞춤법 검사기](/docs/ko/interactive-mode#check-spelling-as-you-type)로 프롬프트 입력에서 철자가 틀린 단어에 밑줄                                                                                           | 인터페이스 및 터미널     | 사용자 또는 관리됨     |
| [`spinnerTipsEnabled`](#spinnertipsenabled)                                                           | Claude가 작동하는 동안 스피너의 팁 숨기기                                                                                                                                                        | 인터페이스 및 터미널     | 모든 파일          |
| [`spinnerTipsOverride`](#spinnertipsoverride)                                                         | 스피너 회전에 자신의 팁을 추가하거나 기본 제공 팁 교체                                                                                                                                                   | 인터페이스 및 터미널     | 모든 파일          |
| [`spinnerVerbs`](#spinnerverbs)                                                                       | 턴이 실행되는 동안 표시되는 동사 추가 또는 교체                                                                                                                                                       | 인터페이스 및 터미널     | 모든 파일          |
| [`sshConfigs`](#sshconfigs)                                                                           | Desktop 환경 드롭다운에 [SSH 연결](/docs/ko/desktop#pre-configure-ssh-connections-for-your-team) 추가                                                                                             | 원격, 데스크톱, 알림    | 사용자 또는 관리됨     |
| [`sshHostAllowlist`](#sshhostallowlist)                                                               | [Desktop SSH 세션](/docs/ko/desktop#restrict-which-ssh-hosts-users-can-connect-to)이 도달할 수 있는 호스트 제한                                                                                      | 원격, 데스크톱, 알림    | 관리됨            |
| [`statusLine`](#statusline)                                                                           | [상태 라인](/docs/ko/statusline)을 렌더링하는 자신의 명령 실행                                                                                                                                          | 인터페이스 및 터미널     | 모든 파일          |
| [`strictKnownMarketplaces`](#strictknownmarketplaces)                                                 | 사용자가 추가하고 설치할 수 있는 [마켓플레이스](/docs/ko/plugin-marketplaces) 소스 허용 목록                                                                                                                     | 플러그인 및 기술       | 관리됨            |
| [`strictPluginOnlyCustomization`](#strictpluginonlycustomization)                                     | 사용자 및 프로젝트 소스에서 [기술](/docs/ko/skills), [에이전트](/docs/ko/sub-agents), [훅](/docs/ko/hooks), [MCP 서버](/docs/ko/mcp) 차단                                                                                    | 플러그인 및 기술       | 관리됨            |
| [`strictPluginOnlyCustomization.agents`](#strictpluginonlycustomization-agents)                       | [에이전트](/docs/ko/sub-agents)를 플러그인 및 관리 소스로 잠금                                                                                                                                          | 플러그인 및 기술       | 관리됨            |
| [`strictPluginOnlyCustomization.hooks`](#strictpluginonlycustomization-hooks)                         | [훅](/docs/ko/hooks)을 플러그인 및 관리 소스로 잠금                                                                                                                                                  | 플러그인 및 기술       | 관리됨            |
| [`strictPluginOnlyCustomization.mcp`](#strictpluginonlycustomization-mcp)                             | [MCP 서버](/docs/ko/mcp)를 플러그인 및 관리 소스로 잠금                                                                                                                                               | 플러그인 및 기술       | 관리됨            |
| [`strictPluginOnlyCustomization.skills`](#strictpluginonlycustomization-skills)                       | [기술](/docs/ko/skills)을 플러그인 및 관리 소스로 잠금                                                                                                                                                | 플러그인 및 기술       | 관리됨            |
| [`subagentPromptCacheTtl`](#subagentpromptcachettl)                                                   | subagent 및 주 대화 외부의 다른 요청의 [프롬프트 캐시 수명](/docs/ko/prompt-caching#cache-lifetime) 선택                                                                                                     | 모델 및 응답         | 모든 파일          |
| [`subagentStatusLine`](#subagentstatusline)                                                           | [subagent](/docs/ko/sub-agents) 작업 표시의 행을 자신의 명령으로 다시 쓰기                                                                                                                               | 인터페이스 및 터미널     | 모든 파일          |
| [`switchModelsOnFlag`](#switchmodelsonflag)                                                           | [안전 분류기](/docs/ko/model-config#ask-before-switching)가 요청에 플래그를 지정할 때 모델을 자동으로 전환하거나 일시 중지                                                                                              | 모델 및 응답         | 모든 파일          |
| [`syncClaudeAiPlugins`](#syncclaudeaiplugins)                                                         | [claude.ai 계정에서 활성화된 플러그인](/docs/ko/plugins-reference#synced-plugins) 다운로드 중지 및 이미 동기화된 플러그인 숨기기                                                                                       | 플러그인 및 기술       | 사용자, 로컬 또는 관리됨 |
| [`syncClaudeAiSkills`](#syncclaudeaiskills)                                                           | [claude.ai 계정에서 활성화된 기술](/docs/ko/skills#how-synced-skills-behave) 다운로드 중지 및 이미 동기화된 기술 숨기기                                                                                            | 플러그인 및 기술       | 사용자, 로컬 또는 관리됨 |
| [`syntaxHighlightingDisabled`](#syntaxhighlightingdisabled)                                           | diff 및 코드 블록에서 구문 강조 끄기                                                                                                                                                           | 인터페이스 및 터미널     | 모든 파일          |
| [`taskOutputMaxChars`](#taskoutputmaxchars)                                                           | v2.1.277에서 제거됨, 크기를 조정한 `TaskOutput` 도구와 함께                                                                                                                                       | 메모리 및 컨텍스트      | 모든 파일          |
| [`teammateDefaultModel`](#teammatedefaultmodel)                                                       | v2.1.234에서 제거됨; Claude Code가 팀원의 모델을 선택하는 방법에 대해 [팀원 및 모델 지정](/docs/ko/agent-teams#specify-teammates-and-models) 참조                                                                    | 전역 설정           | 전역 설정          |
| [`teammateMode`](#teammatemode)                                                                       | [에이전트 팀 팀원 표시](/docs/ko/agent-teams#choose-a-display-mode) 방법 선택                                                                                                                       | 에이전트, 세션, 워크트리  | 모든 파일          |
| [`terminalProgressBarEnabled`](#terminalprogressbarenabled)                                           | 지원하는 터미널에서 터미널 진행률 표시줄 숨기기                                                                                                                                                        | 인터페이스 및 터미널     | 모든 파일          |
| [`terminalTitleFromRename`](#terminaltitlefromrename)                                                 | [`/rename`](/docs/ko/sessions#name-your-sessions) 및 `--name`이 터미널 탭 제목 변경 중지                                                                                                           | 인터페이스 및 터미널     | 모든 파일          |
| [`theme`](#theme)                                                                                     | 인터페이스 [색상 테마](/docs/ko/terminal-config#match-the-color-theme) 선택, 기본 제공 또는 사용자 정의                                                                                                      | 인터페이스 및 터미널     | 모든 파일          |
| [`timeFormat`](#timeformat)                                                                           | 인터페이스의 시간을 12시간 또는 24시간 시계, UTC 또는 strftime 패턴으로 표시                                                                                                                               | 인터페이스 및 터미널     | 모든 파일          |
| [`timeZone`](#timezone)                                                                               | 인터페이스의 시간을 시스템 이외의 시간대로 표시                                                                                                                                                        | 인터페이스 및 터미널     | 모든 파일          |
| [`tui`](#tui)                                                                                         | [전체 화면](/docs/ko/fullscreen) 또는 클래식 터미널 렌더러 선택                                                                                                                                         | 인터페이스 및 터미널     | 모든 파일          |
| [`ultracode`](#ultracode)                                                                             | Claude가 요청 없이 각 실질적인 작업에 대해 [워크플로우](/docs/ko/workflows#let-claude-decide-with-ultracode) 계획하도록 함                                                                                       | 모델 및 응답         | 모든 파일          |
| [`useAutoModeDuringPlan`](#useautomodeduringplan)                                                     | [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode) 분류기가 [계획 모드](/docs/ko/permission-modes#analyze-before-you-edit-with-plan-mode)에서 셸 명령을 검토하도록 허용; 프롬프트를 얻으려면 `false`로 설정 | 권한 설정           | 사용자, 로컬 또는 관리됨 |
| [`verbose`](#verbose)                                                                                 | 잘린 요약 대신 [전체 도구 출력](/docs/ko/cli-reference#cli-flags) 표시; 둘 다 설정되면 `viewMode`가 우선                                                                                                      | 인터페이스 및 터미널     | 모든 파일          |
| [`viewMode`](#viewmode)                                                                               | 모든 세션을 [기본, 상세 또는 포커스 보기](/docs/ko/cli-reference#cli-flags)에서 시작                                                                                                                       | 인터페이스 및 터미널     | 모든 파일          |
| [`vimInsertModeRemaps`](#viminsertmoderemaps)                                                         | `jj`와 같은 두 키 [INSERT 모드 시퀀스](/docs/ko/interactive-mode#remap-insert-mode-key-sequences)를 Escape로 매핑                                                                                    | 인터페이스 및 터미널     | 사용자 또는 관리됨     |
| [`voice`](#voice)                                                                                     | [음성 받아쓰기](/docs/ko/voice-dictation)를 켜고 홀드 또는 탭 모드 선택                                                                                                                                  | 인터페이스 및 터미널     | 모든 파일          |
| [`voiceEnabled`](#voiceenabled)                                                                       | 이전 단일 키 형식으로 [음성 받아쓰기](/docs/ko/voice-dictation) 켜기                                                                                                                                    | 인터페이스 및 터미널     | 모든 파일          |
| [`wheelScrollAccelerationEnabled`](#wheelscrollaccelerationenabled)                                   | 전체 화면 렌더링에서 [마우스 휠 가속](/docs/ko/fullscreen#mouse-wheel-scrolling) 끄기                                                                                                                   | 인터페이스 및 터미널     | 모든 파일          |
| [`workflowKeywordTriggerEnabled`](#workflowkeywordtriggerenabled)                                     | 프롬프트의 단어 `ultracode`가 [워크플로우](/docs/ko/workflows) 시작하도록 허용; 입력하지 않으려면 `false`로 설정                                                                                                      | 훅 및 자동화         | 모든 파일          |
| [`workflowSizeGuideline`](#workflowsizeguideline)                                                     | [동적 워크플로우](/docs/ko/workflows)에서 Claude가 목표로 하는 에이전트 수 설정                                                                                                                              | 훅 및 자동화         | 모든 파일          |
| [`worktree`](#worktree)                                                                               | Claude Code가 git [워크트리](/docs/ko/worktrees)를 생성하는 방법 구성                                                                                                                                | 에이전트, 세션, 워크트리  | 모든 파일          |
| [`worktree.baseRef`](#worktree-baseref)                                                               | 새 [워크트리](/docs/ko/worktrees)를 원격 기본 분기 또는 로컬 HEAD에서 분기                                                                                                                                 | 에이전트, 세션, 워크트리  | 모든 파일          |
| [`worktree.bgIsolation`](#worktree-bgisolation)                                                       | 백그라운드 세션이 [워크트리](/docs/ko/worktrees) 없이 작업 복사본을 편집하도록 허용                                                                                                                               | 에이전트, 세션, 워크트리  | 모든 파일          |
| [`worktree.sparsePaths`](#worktree-sparsepaths)                                                       | 각 [워크트리](/docs/ko/worktrees)에서 필요한 디렉토리만 체크아웃                                                                                                                                          | 에이전트, 세션, 워크트리  | 모든 파일          |
| [`worktree.symlinkDirectories`](#worktree-symlinkdirectories)                                         | 각 [워크트리](/docs/ko/worktrees)에 큰 디렉토리를 복제하는 대신 심볼릭 링크                                                                                                                                   | 에이전트, 세션, 워크트리  | 모든 파일          |
| [`wslInheritsWindowsSettings`](#wslinheritswindowssettings)                                           | WSL이 Windows 정책 체인에서 [관리되는 설정](/docs/ko/managed-settings) 읽도록 함                                                                                                                        | 엔터프라이즈 및 관리 설정  | 관리됨            |

<h2 id="model-and-responses">
  모델 및 응답
</h2>

Claude Code가 사용할 모델과 응답 방식을 선택합니다. 이러한 설정이 `/model` 명령 및 환경 변수와 상호작용하는 방식에 대해서는 [모델 구성](/docs/ko/model-config)을 참조하십시오.

<h3 id="advisormodel">
  `advisorModel`
</h3>

Claude가 서버 측 [advisor 도구](/docs/ko/advisor)를 호출할 때 응답하는 모델을 선택합니다. 이를 설정 해제하여 advisor를 끕니다. advisor는 최소한 주 모델만큼 능력이 있어야 합니다. 허용되는 쌍과 허용되지 않는 쌍을 선택할 때 발생하는 상황에 대해서는 [advisor 모델 선택](/docs/ko/advisor#choose-an-advisor-model)을 참조하십시오.

일반적으로 이 키를 직접 편집하지 않습니다. `/advisor`를 실행하여 현재 선택, advisor할 수 있는 모델, **advisor 없음**을 표시하는 선택기를 엽니다. Claude Code는 선택 사항을 `~/.claude/settings.json`의 이 키에 저장합니다. [Remote Control](/docs/ko/remote-control) 클라이언트에서 선택하거나 원격 워커에 연결된 세션에서 선택하면, 선택 사항이 해당 세션에만 적용되며 이 키를 변경하지 않습니다.

계정에 [usage-credits 동의](/docs/ko/advisor#fable-advisor-and-usage-credits)가 필요한 경우, `/model fable`을 실행하여 먼저 동의합니다. 그렇게 할 때까지 `/advisor`에서 Fable을 선택해도 아무것도 저장되지 않으며 Claude Code는 먼저 `/model fable`을 실행하도록 지시합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열, `"fable"`, `"opus"`, 또는 `"sonnet"` 중 하나의 별칭(Claude Code의 해당 모델 제품군의 현재 기본 버전으로 확인됨) 또는 `"claude-opus-5"`와 같은 전체 모델 ID
* **기본값**: 설정 해제되어 advisor가 꺼짐
* **세션별 재정의**: `--advisor`는 한 세션 동안 이 키보다 우선합니다. [`CLAUDE_CODE_DISABLE_ADVISOR_TOOL`](/docs/ko/env-vars)은 advisor를 끄고, 이 키는 다시 켤 수 없습니다

```json settings.json theme={null}
{
  "advisorModel": "opus"
}
```

이 키는 Amazon Bedrock 및 AWS의 Claude Platform과 같이 advisor가 [사용 불가능한](/docs/ko/advisor#requirements) 공급자에게는 영향을 주지 않습니다. `"fable"`은 [Fable 액세스](/docs/ko/advisor#choose-an-advisor-model)가 필요합니다.

<h3 id="alwaysthinkingenabled">
  `alwaysThinkingEnabled`
</h3>

이를 `false`로 설정하여 모든 세션에 대해 [확장 사고](/docs/ko/model-config#extended-thinking)를 끕니다. 사고는 기본적으로 켜져 있으므로 `true`는 아무것도 변경하지 않습니다. 대부분의 사람들은 파일을 편집하는 대신 `/config`를 통해 이를 설정합니다.

Fable 모델과 같이 항상 사고하는 모델에서는 `false`가 영향을 주지 않습니다. [타사 공급자](/docs/ko/third-party-integrations)에서 Claude Code는 사고를 끄는 대신 `thinking` 매개변수를 생략하므로 적응형 추론 모델은 여전히 사고할 수 있습니다. Anthropic API에서 사고를 끈 경우, Claude Code는 Opus 5와 같이 [해당 조합을 허용하지 않는](/docs/ko/errors#effort-isnt-available-with-thinking-turned-off) 모델에 더 높은 수준 대신 노력 `high`를 보냅니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: 영향 없음; 사고는 이미 켜짐
  * `false`: Claude Code는 모든 세션에 대해 확장 사고를 끕니다
* **기본값**: 설정 해제되어 사고는 이를 지원하는 모델에 대해 켜짐
* **세션별 재정의**: [`MAX_THINKING_TOKENS`](/docs/ko/env-vars)는 한 세션 동안 이 키보다 우선합니다: `0`은 `false`와 동일한 모델 및 공급자 제한 하에서 사고를 끄고, 양수 값은 이 키가 `false`일 때도 사고를 켭니다. 적응형 추론 모델에서 숫자 자체는 무시됩니다

```json settings.json theme={null}
{
  "alwaysThinkingEnabled": false
}
```

<h3 id="availablemodels">
  `availableModels`
</h3>

사람들이 주 세션, [subagents](/docs/ko/sub-agents), [skills](/docs/ko/skills), [advisor](/docs/ko/advisor)에 대해 선택할 수 있는 모델을 제한합니다. 관리되는 목록은 `/model`, `--model`, 개발자 자신의 파일의 `model` 키를 제한합니다. 목록 외의 모델은 선택할 수 없습니다. 이것만으로는 기본값 옵션을 건드리지 않습니다. [`enforceAvailableModels`](#enforceavailablemodels)와 쌍을 이루십시오.

* **범위**: [`모든 파일`](#scopes). 조직에 대해 적용하려면 관리되는 설정에 배포합니다.
* **유형**: 모델 별칭 또는 ID의 배열
* **기본값**: 설정 해제되어 모든 모델을 사용할 수 있음

이 예제는 사람들이 Sonnet 및 Haiku 모델만 선택하도록 허용합니다:

```json settings.json theme={null}
{
  "availableModels": ["sonnet", "haiku"]
}
```

[모델 선택 제한](/docs/ko/model-config#restrict-model-selection)을 참조하십시오.

<h3 id="effortlevel">
  `effortLevel`
</h3>

저장하지 않은 모델에 대한 기본 [노력 수준](/docs/ko/model-config#adjust-effort-level)을 설정합니다. 낮은 수준은 간단한 작업에서 더 빠르고 저렴하며, 높은 수준은 복잡한 문제에 대해 더 깊이 있게 추론합니다.

머신의 대화형 세션에서 `/effort low`, `medium`, `high`, 또는 `xhigh`를 실행하면, Claude Code는 이 키를 작성하는 대신 수준을 [`modelSettings`](#modelsettings) 아래의 활성 모델에 대해 저장합니다. v2.1.251 이전에는 `/effort`가 이 키를 작성했습니다.

동일한 설정 파일 내에서 Claude Code는 이 키보다 모델의 저장된 수준을 사용합니다. [`modelSettings`](#modelsettings)는 파일 간 우선순위를 나타냅니다.

원격 워커에 연결된 세션에서 `/effort`는 해당 세션에만 적용됩니다. `-p` 실행 또는 Agent SDK에서도 해당 세션에만 적용됩니다. [모델의 기본 노력에 대한 보류가 적용되지 않는 한](/docs/ko/model-config#non-interactive-effort). [노력 수준 조정](/docs/ko/model-config#adjust-effort-level)은 해당 세션에만 적용되는 대화형 선택을 나열합니다. `/effort`가 인쇄하는 메시지는 어떤 일이 발생했는지 나타냅니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열, 다음 중 하나:
  * `"low"`: 짧고 범위가 지정되고 지연 시간에 민감한 작업(지능에 민감하지 않은)에 대한 최소 추론
  * `"medium"`: 일부 지능을 절충할 수 있는 비용에 민감한 작업에 대한 토큰 사용 감소
  * `"high"`: 토큰 사용과 지능의 균형
  * `"xhigh"`: 더 높은 토큰 지출로 더 깊은 추론
* **기본값**: 설정 해제됨
* **세션별 재정의**: `--effort`는 한 세션 동안 이 키보다 우선하며, [`CLAUDE_CODE_EFFORT_LEVEL`](/docs/ko/env-vars)은 둘 다보다 우선합니다

```json settings.json theme={null}
{
  "effortLevel": "xhigh"
}
```

Opus 4.7, Opus 4.8, Fable 5에서 Claude Code는 해당 모델의 기본 노력(조직 설정 또는 기본 제공)을 유지합니다. [노력 수준 조정](/docs/ko/model-config#adjust-effort-level)은 수준 설정의 어떤 방식이 보류를 끝내고 어떤 방식이 유지하는지 나타냅니다. 보류가 끝나면 Claude Code는 [`modelSettings`](#modelsettings)에 명시된 우선순위에 따라 노력을 해결합니다.

<h3 id="enforceavailablemodels">
  `enforceAvailableModels`
</h3>

`/model` 선택기에는 적용되는 경우 [조직 기본 모델](/docs/ko/model-config#organization-default-model)로 확인되고, 그렇지 않으면 계정 유형의 기본값으로 확인되는 **기본값** 옵션이 있습니다. [`availableModels`](#availablemodels) 허용 목록은 이름을 지정할 수 있는 모델을 제한하지만, 그 자체로는 **기본값**을 그대로 두므로 **기본값**은 여전히 목록 외의 모델로 확인될 수 있습니다. 이 키는 그 간격을 닫습니다. Claude Code v2.1.175 이상이 필요합니다.

조직이 관리되는 설정을 배포하면 Claude Code는 관리되는 소스에서만 이 키를 읽고 다른 파일에서는 무시합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: **기본값**이 `availableModels` 외의 모델로 확인될 때, Claude Code는 목록의 첫 번째 사용 가능한 모델로 확인합니다
  * `false`: **기본값**은 `availableModels` 외의 모델로도 평소대로 확인됩니다
* **기본값**: `false`

이 예제는 명명된 선택을 Sonnet 및 Haiku 모델로 제한하고 **기본값**을 사용 가능한 첫 번째 모델로 확인하도록 합니다:

```json settings.json theme={null}
{
  "availableModels": ["sonnet", "haiku"],
  "enforceAvailableModels": true
}
```

`availableModels`가 설정 해제되거나 비어 있을 때 이 키는 영향을 주지 않습니다. [기본 모델에 대한 허용 목록 적용](/docs/ko/model-config#enforce-the-allowlist-for-the-default-model)을 참조하십시오. Claude Code v2.1.175 이상이 필요합니다.

<h3 id="fallbackmodel">
  `fallbackModel`
</h3>

주 모델이 과부하이거나 사용 불가능할 때 Claude Code가 순서대로 시도할 백업 모델을 이름 지정합니다. Claude Code는 체인의 다음 사용 가능한 모델로 전환하고 알림을 표시합니다. 체인이 없으면 Claude Code는 동일한 모델을 재시도한 다음 서버의 오류를 표시하고, 사용자가 재시도하거나 모델을 전환합니다.

전환은 폴백 모델에서 콜드 [프롬프트 캐시](/docs/ko/prompt-caching#switching-models)를 사용한 한 번의 턴을 의미합니다. 다음 메시지는 주 모델을 먼저 다시 시도합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 모델 별칭 또는 ID의 배열; `"default"`는 기본 모델로 확장됩니다
* **기본값**: 설정 해제되어 실패한 요청이 다른 모델에서 재시도되지 않음
* **세션별 재정의**: `--fallback-model`은 한 세션 동안 이 키보다 우선합니다

이 예제는 주 모델이 실패할 때 먼저 Sonnet 5를 시도한 다음 Haiku 4.5를 시도합니다:

```json settings.json theme={null}
{
  "fallbackModel": ["claude-sonnet-5", "claude-haiku-4-5"]
}
```

대부분의 배열 설정과 달리 이 키는 설정 파일 간에 병합되지 않습니다. 가장 높은 우선순위 파일이 전체 체인을 제공합니다. 프로젝트 파일이 `["claude-sonnet-5"]`를 설정하고 사용자 파일이 `["claude-haiku-4-5"]`를 설정하면, 체인은 `["claude-sonnet-5"]`만입니다. Claude Code는 목록에서 최대 3개의 서로 다른 허용 모델을 유지하고 나머지는 무시합니다. [폴백 모델 체인](/docs/ko/model-config#fallback-model-chains)을 참조하십시오.

<h3 id="fastmode">
  `fastMode`
</h3>

사용 가능한 세션에 대해 [빠른 모드](/docs/ko/fast-mode)를 켭니다. 빠른 반복이나 라이브 디버깅과 같은 대화형 작업에 사용하여 토큰당 더 높은 비용으로 속도를 원합니다. 일반적으로 이 키를 직접 편집하지 않습니다. `/fast`를 실행하면 `fastMode: true`를 `~/.claude/settings.json`에 작성하고, 다시 실행하여 빠른 모드를 끄면 키를 제거합니다. 빠른 모드는 Opus 5 및 Opus 4.8에서만 실행됩니다. 다른 모델에서 켜면 Opus로 전환되고, 지원되지 않는 모델로 전환하면 꺼집니다. [빠른 모드가 켜져 있는 동안 모델 전환](/docs/ko/fast-mode#switch-models-while-fast-mode-is-on)을 참조하십시오.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: Claude Code는 사용 가능한 세션에 대해 빠른 모드를 켭니다
  * `false`: 빠른 모드는 꺼진 상태로 유지됩니다
* **기본값**: 설정 해제되어 빠른 모드는 꺼짐
* **세션별 재정의**: [`CLAUDE_CODE_DISABLE_FAST_MODE`](/docs/ko/env-vars)는 한 세션에 대해 빠른 모드를 끄고, 이 키는 다시 켤 수 없습니다

```json settings.json theme={null}
{
  "fastMode": true
}
```

<h3 id="fastmodepersessionoptin">
  `fastModePerSessionOptIn`
</h3>

일반적으로 `/fast`를 실행하면 [`fastMode`](#fastmode)를 사용자 설정에 저장하므로 빠른 모드는 이후의 모든 세션 시작 시 켜집니다. 이를 중지하려면 이 키를 `true`로 설정합니다. 저장된 `fastMode: true`는 더 이상 세션 시작 시 빠른 모드를 켜지 않으며, 각 사용자는 원하는 각 세션에서 `/fast`를 실행해야 합니다. Claude Code는 파일에 `fastMode` 키를 남겨두므로 이 키를 끄면 이전 동작이 복원됩니다. Team 또는 Enterprise 계획의 소유자는 [서버 관리 설정](/docs/ko/server-managed-settings)을 통해 조직 전체에 배포할 수 있습니다. 관리되는 설정이 키를 설정하면 `/fast on`은 대화형 터미널 세션 외부에서 거부되고 조직이 빠른 모드를 비활성화했다고 보고합니다. 이는 [비대화형 모드](/docs/ko/headless), [VS Code 확장](/docs/ko/vs-code), [클라우드 세션](/docs/ko/claude-code-on-the-web)을 포함합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: 저장된 `fastMode: true`는 더 이상 세션 시작 시 빠른 모드를 켜지 않으므로 각 사용자는 원하는 각 세션에서 `/fast`를 실행합니다. `--settings`와 함께 전달된 `fastMode: true`는 관리되는 설정이 이 키를 설정하지 않는 한 해당 세션에 대해 계산됩니다
  * `false`: 저장된 `fastMode: true`는 이후의 모든 세션 시작 시 빠른 모드를 켭니다
* **기본값**: `false`

```json settings.json theme={null}
{
  "fastModePerSessionOptIn": true
}
```

[세션별 옵트인 필요](/docs/ko/fast-mode#require-per-session-opt-in)를 참조하십시오.

<h3 id="language">
  `language`
</h3>

Claude가 기본적으로 영어 이외의 언어로 응답하도록 합니다. 응답에 대한 고정 목록이 없습니다. Claude Code는 값을 시스템 프롬프트에 그대로 추가하여 항상 해당 언어로 응답하도록 지시하므로 Claude가 읽을 수 있는 모든 언어 이름이 작동합니다. Claude Code는 값을 확인하지 않으므로 철자가 잘못된 이름은 오류를 생성하는 대신 작성된 대로 Claude에 도달합니다. 동일한 값은 [음성 받아쓰기](/docs/ko/voice-dictation#change-the-dictation-language)의 언어를 설정하며, 이는 [지원되는 받아쓰기 언어](/docs/ko/voice-dictation#change-the-dictation-language)의 고정 목록을 가지고 있으며, 자동 생성된 세션 제목도 설정합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열, `"japanese"`, `"spanish"`, `"french"` 등 모든 언어 이름; Claude Code는 이를 검증하지 않습니다
* **기본값**: 설정 해제됨; 세션 제목은 대화의 언어와 일치합니다

```json settings.json theme={null}
{
  "language": "japanese"
}
```

<h3 id="maxeffortlevel">
  `maxEffortLevel`
</h3>

세션이 사용할 수 있는 [노력 수준](/docs/ko/model-config#adjust-effort-level)을 제한하고 낮은 수준을 사용 가능하게 둡니다. 더 높은 수준은 모두 제한에서 실행됩니다. `/effort`, `/model` 선택기, `--effort`, [`CLAUDE_CODE_EFFORT_LEVEL`](/docs/ko/env-vars), skill 또는 subagent의 `effort` frontmatter, 또는 모델 자체의 기본값을 포함합니다. Claude Code는 각 요청 전에 제한을 자체적으로 적용하므로 Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry를 포함한 모든 공급자에서 유지됩니다. Claude Code v2.1.267 이상이 필요합니다.

* **범위**: [`모든 파일`](#scopes). 조직에 대해 적용하려면 관리되는 설정에 배포합니다. 여러 범위가 제한을 설정할 때 가장 낮은 제한이 적용되므로 한 범위에서 설정된 제한은 다른 범위에서 높아질 수 없습니다
* **유형**: 문자열, `"low"`, `"medium"`, `"high"`, `"xhigh"`, 또는 `"max"` 중 하나. `"max"` 값은 제한을 설정하지 않습니다
* **기본값**: 설정 해제되어 제한이 적용되지 않음
* **ultracode에 미치는 영향**: `xhigh` 아래의 제한은 제한이 적용되는 모델에서 [ultracode](#ultracode)를 사용 불가능하게 합니다
* **모델별 제한**: 모델의 [`modelSettings`](#modelsettings) 항목에 `maxEffortLevel`을 추가합니다. 해당 항목은 사용자 설정 또는 하나의 [관리되는 소스](/docs/ko/managed-settings#how-claude-code-combines-managed-sources)와 같이 둘 다를 설정하는 설정 소스 내에서만 이 키를 모델에 대해 대체합니다. 해당 소스의 제한에서 모델을 제외하려면 `"max"`를 설정합니다. Claude Code는 여전히 다른 소스의 제한을 적용합니다

이 예제는 모든 모델을 `medium`으로 제한하고 Sonnet 4.6을 제외합니다:

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

조직이 모델에 대해 [노력 제한](/docs/ko/model-config#organization-effort-limits)을 설정할 때, 두 제한 중 낮은 제한이 적용됩니다.

<h3 id="model">
  `model`
</h3>

모든 새 세션이 사용할 모델을 설정하므로 매번 `/model`로 선택할 필요가 없습니다. 여기에 설정해도 세션 중에 전환하는 것을 막지 않습니다. 관리자가 [조직 기본 모델](/docs/ko/model-config#organization-default-model)을 설정하여 사용자 선택을 재정의하면, 사용자, 프로젝트 또는 로컬 설정에서 이 키를 설정해도 해당 모델을 얻습니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열, 모델 별칭 또는 전체 모델 ID
* **기본값**: 설정 해제되어 Claude Code는 계정의 기본 모델을 사용합니다
* **세션별 재정의**: `--model`은 [`ANTHROPIC_MODEL`](/docs/ko/env-vars)보다 우선하며, 둘 다 한 세션 동안 이 키보다 우선합니다. 관리되는 `model`도 포함합니다. [`availableModels`](#availablemodels) 목록은 여전히 선택에 적용됩니다

```json settings.json theme={null}
{
  "model": "claude-sonnet-5"
}
```

여기의 값은 [`ANTHROPIC_DEFAULT_MODEL`](/docs/ko/model-config#set-a-default-model-for-new-sessions)을 능가합니다. Claude Code는 다른 것이 모델을 선택하지 않을 때만 사용합니다.

<h3 id="modeloverrides">
  `modelOverrides`
</h3>

Anthropic 모델 ID를 Amazon Bedrock 추론 프로필 ARN과 같은 공급자별 모델 ID로 매핑합니다. 각 모델 선택기 항목은 공급자 API를 호출할 때 매핑된 값을 사용합니다. 관리자는 [Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry](/docs/ko/model-config#override-model-ids-per-version)에서 이를 사용하여 각 모델 버전을 특정 추론 프로필, 버전 이름 또는 배포로 라우팅하여 거버넌스, 비용 할당 또는 지역 라우팅을 수행합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 모델 ID를 공급자 모델 ID로 매핑하는 객체
* **기본값**: 설정 해제됨

이 예제는 Opus 4.6에 대한 모든 호출을 명명된 Bedrock 추론 프로필로 라우팅합니다:

```json settings.json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-6": "arn:aws:bedrock:us-east-1:123456789012:inference-profile/example"
  }
}
```

[모델 ID를 버전별로 재정의](/docs/ko/model-config#override-model-ids-per-version)를 참조하십시오.

<h3 id="modelpicker">
  `modelPicker`
</h3>

`/model` 선택기가 제공하는 모델을 작성한 순서대로 선택한 레이블 아래에 나열하므로 선택기는 조직이 실행하는 모델을 나열합니다. 기본 제공 라인업 이후 또는 대신합니다. 각 행의 `model`은 그대로 사용되므로 `--model`이 허용하는 모든 것을 허용합니다. `opus`와 같은 별칭, Anthropic 모델 ID, 또는 Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry, 또는 LLM 게이트웨이의 공급자 형식 ID입니다. Claude Code v2.1.242 이상이 필요합니다.

* **범위**: [`사용자 또는 관리됨`](#scopes). Claude Code는 관리되는 설정, `--settings`, 사용자 설정에서 키를 읽고 프로젝트 및 로컬 설정에서는 무시하므로 복제한 저장소가 선택기를 다시 레이블할 수 없습니다. 이 세 가지 중 가장 높은 것이 키를 설정하면 전체 라인업을 제공하며, Claude Code는 두 소스의 라인업을 결합하지 않습니다.
* **유형**: `options` 배열과 선택적 `replaceBuiltInOptions` 부울을 포함하는 객체
* **기본값**: 설정 해제되어 선택기는 기본 제공 라인업을 표시합니다

이 예제는 팀이 인식하는 이름 아래에 기본 제공 라인업 이후에 두 개의 Bedrock 배포를 추가합니다:

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
  `modelPicker`의 필드
</h4>

키는 행 자체에 대한 하나와 기본 제공 라인업을 대체하거나 추가하는지에 대한 하나의 두 필드를 사용합니다.

| 필드                      | 유형                                                    | 수행 작업                                                                                                                                                    |
| :---------------------- | :---------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options`               | 필수 `model`과 선택적 `label` 및 `description`을 포함하는 각 행의 배열 | 선택기가 표시하는 행(이 순서대로), 회색으로 표시된 행은 맨 아래로 이동합니다. `label` 없이 Claude Code는 알려진 모델에 대해 기본 제공 이름으로 행을 제목으로 지정하거나 모델 ID로 지정하고, `description` 없이 일반 두 번째 줄을 작성합니다 |
| `replaceBuiltInOptions` | 부울, 기본값 `false`                                       | 이 행만, **기본값**, 세션이 이미 사용 중인 모델의 행을 표시하려면 `true`로 설정합니다. 설정 해제하여 기본 제공 라인업 이후에 이 행을 추가합니다                                                                 |

`replaceBuiltInOptions`가 켜져 있으면 Claude Code는 다른 모든 행을 숨깁니다. 기본 제공 라인업, [`availableModels`](#availablemodels) 항목에 대해 추가하는 행, [게이트웨이 검색](/docs/ko/llm-gateway-protocol#model-discovery)이 찾은 모델, [`ANTHROPIC_CUSTOM_MODEL_OPTION`](/docs/ko/model-config#add-a-custom-model-option). 꺼져 있으면 Claude Code는 기본 제공 라인업이 이미 다루는 나열된 모델을 건너뜁니다. 레이블은 선택기가 표시하는 것을 변경하지만 Claude Code가 실행하는 모델은 변경하지 않습니다.

[`availableModels`](#availablemodels) 허용 목록은 여전히 이 행에 적용됩니다. 나열된 모델을 허용 목록에 추가하기 전에 [병합 동작](/docs/ko/model-config#merge-behavior)을 읽으십시오. 특정 모델 ID는 제품군의 와일드카드 항목을 좁힙니다. Claude Code는 선택기를 표시하기 전에 각 행을 세션에 대해 확인합니다:

* **삭제됨**: Claude Code가 제공할 수 없는 행(예: 폐기된 모델 또는 조직이 액세스할 수 없는 모델)
* **회색으로 표시됨**: 아직 선택할 수 없는 행(이유와 함께 표시됨)
* **행이 생존하지 않음**: Claude Code는 기본 제공 라인업을 유지하고 허용 목록으로 필터링합니다

Claude Code는 구문 분석할 수 없는 행을 삭제하고 나머지를 유지합니다. [손상된 설정 파일 수정](/docs/ko/settings#fix-a-broken-settings-file)을 참조하십시오.

<h3 id="modelpricing">
  `modelPricing`
</h3>

조직이 지불하는 요금으로 지출을 보고합니다. 조직이 계약 요금을 가지고 있을 때 설정하므로 개발자가 보는 달러 수치가 청구서와 일치합니다. Claude Code는 `/usage`, [상태 줄](/docs/ko/statusline), Agent SDK의 `total_cost_usd`, [`--max-budget-usd`](/docs/ko/cli-reference) 제한, [OpenTelemetry](/docs/ko/monitoring-usage) 비용 메트릭 및 이벤트의 요금을 적용합니다. 요금을 제공합니다. Claude Code는 계약 또는 Claude 콘솔에서 읽지 않습니다. Claude Code v2.1.242 이상이 필요합니다.

* **범위**: [`관리됨`](#scopes). 서버 관리 설정, MDM 정책, `managed-settings.json` 파일 또는 [정책 도우미](/docs/ko/managed-settings#compute-the-policy-with-a-helper-program)를 통해 키를 배포합니다. Claude Code는 사용자, 프로젝트, 로컬 설정, `--settings`, Windows의 사용자 쓰기 가능 [HKCU 레지스트리](/docs/ko/managed-settings#where-each-mechanism-stores-the-policy)에서 무시합니다. 서버 관리 설정을 사용하면 각 세션은 해당 세션의 [설정 가져오기](/docs/ko/server-managed-settings#fetch-and-caching-behavior)가 설정을 확인할 때까지 정가로 비용을 보고합니다. Claude Code를 포함하고 [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/ko/env-vars)를 설정하는 호스트 애플리케이션은 SDK [`managedSettings`](/docs/ko/agent-sdk/typescript#options) 옵션을 통해 자체 테이블을 제공할 수 있으며, Claude Code는 관리되는 소스가 키를 설정하지 않을 때만 사용하고 Claude Code v2.1.246 이상에서만 사용합니다.
* **유형**: 선택적 `multiplier`와 선택적 `overrides` 맵을 포함하는 객체
* **기본값**: 설정 해제되어 Claude Code는 호스트 애플리케이션이 테이블을 제공하지 않는 한 정가를 보고합니다

`multiplier`만 설정하여 정액 할인 또는 인상을 하거나, `overrides`만 설정하여 모델별 요금을 설정하거나, 둘 다 설정합니다.

이 예제는 Sonnet 4.6에 대한 계약 요금을 설정한 다음 Sonnet 행을 포함한 모든 수치를 15% 감소시킵니다:

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

1보다 큰 `multiplier`를 최대 10까지 설정하여 모든 수치를 인상합니다. 인상은 Claude Code v2.1.271 이상이 필요합니다. 이전 버전은 경고와 함께 1보다 큰 `multiplier`를 무시하고 설정의 나머지를 유지합니다.

요금이 적용되는지 확인하는 방법을 포함한 단계는 [계약 요금으로 지출 보고](/docs/ko/costs#report-spend-at-your-contracted-rates)를 참조하십시오.

<span id="modelpricing-multiplier" />

<span id="modelpricing-overrides" />

<h4 id="fields-for-modelpricing">
  `modelPricing`의 필드
</h4>

| 필드           | 유형                                                                                 | 수행 작업                                                                                                                                    |
| :----------- | :--------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| `multiplier` | 0보다 크고 최대 10인 숫자                                                                   | `overrides` 행이 다루는지 여부에 관계없이 Claude Code가 계산하는 모든 비용을 조정합니다. 1 미만은 할인, 1 초과는 인상입니다                                                       |
| `overrides`  | 모델 ID를 `input`, `output`, `cacheRead`, `cacheWrite`를 포함하는 요금 객체로 매핑하며, 각각 0\~10000 | 해당 모델의 백만 토큰당 USD 요금(모두 4개 필수). `cacheWrite`는 5분 및 1시간 캐시 쓰기를 모두 다룹니다. [어떤 모델이 행을 적용하는지](#which-models-a-modelpricing-row-applies-to) 참조 |

Claude Code는 행의 요금을 작성한 대로 정확히 사용하며, 빠른 모드 할증료 또는 [미국 전용 추론 요금](https://platform.claude.com/docs/en/about-claude/pricing)을 추가하지 않습니다. `multiplier`도 설정하면 Claude Code는 행의 요금 위에 적용합니다. Claude Code는 구문 분석할 수 없는 요금이 있는 행 또는 구문 분석할 수 없는 `multiplier`를 삭제하고 나머지를 유지합니다. [손상된 설정 파일 수정](/docs/ko/settings#fix-a-broken-settings-file)을 참조하십시오.

<h4 id="which-models-a-modelpricing-row-applies-to">
  `modelPricing` 행이 적용되는 모델
</h4>

Claude Code는 행의 키에서 행이 적용되는 모델을 결정합니다:

* **기본 제공 모델의 ID**: Claude Code 자체가 기본 제공 모델에 사용하는 키(해당 키가 `claude-sonnet-4-6`과 같은 모델 자체의 ID이든 Bedrock, Agent Platform 또는 Foundry ID이든). Claude Code는 해당 모델의 모든 날짜 스냅샷 ID 및 공급자별 ID에 행을 적용합니다.
* **다른 키**: 게이트웨이 모델 별칭과 같이 기본 제공 모델의 ID가 아닌 키. Claude Code는 해당 하나의 ID에만 행을 적용합니다. 모델 ID가 키 중 하나와 정확히 일치하고 기본 제공 모델의 ID로 키가 지정된 행에도 해당하면 Claude Code는 정확한 일치를 사용합니다.
* **Bedrock 애플리케이션 추론 프로필**: Claude Code가 [`modelOverrides`](#modeloverrides) 맵 또는 [`bedrock:GetInferenceProfile` 조회](/docs/ko/amazon-bedrock#iam-configuration)를 통해 프로필을 라우팅하는 모델로 확인한 후, Claude Code는 해당 모델의 행을 프로필에 적용합니다.

<h3 id="modelsettings">
  `modelSettings`
</h3>

사용하는 각 모델에 대해 [노력 수준](/docs/ko/model-config#adjust-effort-level)을 저장합니다. Claude Code v2.1.251 이상이 필요합니다.

머신의 대화형 세션에서 `/effort` 또는 `/model` 선택기의 노력 슬라이더로 `low`, `medium`, `high`, 또는 `xhigh`를 기본값으로 저장하면, Claude Code는 사용 중인 모델 아래에 해당 수준을 작성하므로 이 키를 직접 편집하는 경우는 거의 없습니다. [VS Code 확장의 모델 선택기](/docs/ko/vs-code#use-the-prompt-box)에서 이 수준 중 하나를 선택하면 Claude Code는 동일한 방식으로 여기에 저장합니다. [`effortLevel`](#effortlevel) 항목은 `/effort`가 해당 세션에만 적용되는 세션을 나열합니다.

저장한 수준을 변경하거나 제거하려면 키를 직접 편집합니다.

여기의 모델의 `effortLevel`은 동일한 설정 파일의 최상위 [`effortLevel`](#effortlevel)보다 우선합니다. 파일 간에 Claude Code는 각 모델을 별도로 확인합니다. 해당 모델에 대해 `effortLevel`을 설정하거나 최상위 `effortLevel`을 설정하는 가장 높은 우선순위 [설정 파일](/docs/ko/settings#settings-precedence)이 결정하므로 관리되는 설정의 `effortLevel`은 사용자 설정에 저장한 수준을 능가합니다. [노력 수준 조정](/docs/ko/model-config#adjust-effort-level)은 저장된 수준을 재정의할 수 있는 다른 것(예: 시작 시 `--effort`)을 나열합니다.

한 모델의 노력을 설정하는 대신 제한하려면 해당 모델의 항목에 [`maxEffortLevel`](#maxeffortlevel) 필드를 추가합니다. 필드는 Claude Code v2.1.267 이상이 필요합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 모델 이름을 `effortLevel` 필드(하나의 `"low"`, `"medium"`, `"high"`, 또는 `"xhigh"`), [`maxEffortLevel`](#maxeffortlevel) 필드, 또는 둘 다를 포함하는 객체로 매핑하는 객체
* **기본값**: 설정 해제됨

Claude Code는 각 항목을 `claude-opus-5`와 같은 모델의 정규 이름 아래에 작성하고 해당 모델의 별칭, 날짜 접미사, `[1m]`, 인식된 공급자별 ID를 동일한 항목과 일치시킵니다.

이 예제는 Opus 5를 `medium`으로 유지하면서 다른 모델은 자신의 저장된 또는 기본 수준을 사용합니다:

```json settings.json theme={null}
{
  "modelSettings": {
    "claude-opus-5": {
      "effortLevel": "medium"
    }
  }
}
```

`/effort auto`를 실행하여 사용 중인 모델에 대해 저장된 수준을 지웁니다. Claude Code는 다른 항목과 최상위 `effortLevel`을 그대로 둡니다.

<h3 id="outputstyle">
  `outputStyle`
</h3>

[출력 스타일](/docs/ko/output-styles)을 이름으로 선택합니다. 출력 스타일은 Claude의 역할, 톤, 출력 형식을 변경하는 저장된 지시 집합입니다. 기본 제공 Explanatory 및 Learning 스타일 또는 직접 작성한 스타일입니다.

세션 중에 이 키를 변경하면 Claude는 다음 메시지부터 새 스타일을 사용합니다. 해당 메시지가 프롬프트 캐싱에서 비용이 얼마인지는 [출력 스타일 변경](/docs/ko/prompt-caching#changing-output-style)을 참조하십시오. v2.1.251 이전에는 `/clear`를 실행하거나 새 세션을 시작한 후에만 편집이 적용되었습니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열, [기본 제공](/docs/ko/output-styles#built-in-output-styles) 또는 [사용자 정의](/docs/ko/output-styles#create-a-custom-output-style) 출력 스타일의 이름
* **기본값**: 설정 해제되어 Claude Code는 기본 스타일을 사용합니다

이 예제는 작업 간에 교육 통찰력을 추가하는 기본 제공 Explanatory 스타일을 선택합니다:

```json settings.json theme={null}
{
  "outputStyle": "Explanatory"
}
```

<h3 id="promptcachettl">
  `promptCacheTtl`
</h3>

[프롬프트 캐시](/docs/ko/prompt-caching)가 주 대화를 유지하는 기간을 선택합니다. 이 키는 대화형, `-p`, Agent SDK 턴과 함께 Claude Code가 인라인으로 실행하는 도우미에 적용됩니다. 1시간 수명은 더 긴 휴식 시간 동안 캐시를 따뜻하게 유지하고, API는 [각 캐시 쓰기를 5분 수명보다 더 높은 요금으로 청구합니다](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing). Claude Code v2.1.242 이상이 필요합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열, 다음 중 하나:
  * `"5m"`: 캐시는 5분 동안 유지됩니다
  * `"1h"`: 캐시는 1시간 동안 유지됩니다
* **기본값**: 설정 해제되어 각 주 대화 요청은 [기본 수명](/docs/ko/prompt-caching#which-ttl-each-request-gets)을 얻습니다
* **세션별 재정의**: [`FORCE_PROMPT_CACHING_5M`](/docs/ko/env-vars)는 다른 모든 것보다 우선하고, [`CLAUDE_CODE_PROMPT_CACHE_TTL`](/docs/ko/env-vars), 그 다음 이 키, 마지막으로 [`ENABLE_PROMPT_CACHING_1H`](/docs/ko/env-vars)

이 예제는 주 대화를 1시간 수명으로 유지하고 subagents를 5분으로 둡니다:

```json settings.json theme={null}
{
  "promptCacheTtl": "1h",
  "subagentPromptCacheTtl": "5m"
}
```

각 수명의 비용은 [캐시 수명](/docs/ko/prompt-caching#cache-lifetime)을 참조하십시오.

<h3 id="showthinkingsummaries">
  `showThinkingSummaries`
</h3>

대화형 세션에서 Claude의 [확장 사고](/docs/ko/model-config#extended-thinking) 요약을 봅니다. `Ctrl+O`로 사고를 확장할 때 전체 요약을 원하면 설정합니다. 설정 해제되거나 `false`일 때 Anthropic API는 사고 블록을 수정하고 Claude Code는 축소된 스텁을 표시합니다. 타사 공급자는 수정하지 않습니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: `Ctrl+O`로 사고를 확장할 때 전체 사고 요약을 봅니다
  * `false`: Anthropic API는 사고 블록을 수정하고 Claude Code는 축소된 스텁을 표시합니다
* **기본값**: `false`

```json settings.json theme={null}
{
  "showThinkingSummaries": true
}
```

수정은 모델이 생성하는 것이 아니라 보는 것만 변경합니다. 사고 지출을 줄이려면 [예산을 낮추거나 사고를 비활성화](/docs/ko/model-config#extended-thinking)하십시오.

<h3 id="subagentpromptcachettl">
  `subagentPromptCacheTtl`
</h3>

[프롬프트 캐시](/docs/ko/prompt-caching)가 Claude Code가 주 대화 외부에서 만드는 요청을 유지하는 기간을 선택합니다. 이 키는 [subagents](/docs/ko/sub-agents), [workflows](/docs/ko/workflows), Claude Code의 자체 배경 및 도우미 요청(예: 압축 및 세션 제목)에 적용됩니다. 1시간 수명은 더 긴 휴식 시간 동안 캐시를 따뜻하게 유지하고, API는 [각 캐시 쓰기를 5분 수명보다 더 높은 요금으로 청구합니다](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing). Claude Code v2.1.242 이상이 필요합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열, 다음 중 하나:
  * `"5m"`: 캐시는 5분 동안 유지됩니다
  * `"1h"`: 캐시는 1시간 동안 유지됩니다
* **기본값**: 설정 해제되어 이러한 각 요청은 [기본 수명](/docs/ko/prompt-caching#which-ttl-each-request-gets)을 얻습니다
* **세션별 재정의**: [`FORCE_PROMPT_CACHING_5M`](/docs/ko/env-vars)는 다른 모든 것보다 우선하고, [`CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`](/docs/ko/env-vars), 그 다음 이 키, 그 다음 [`ENABLE_PROMPT_CACHING_1H`](/docs/ko/env-vars)(모든 요청에서 1시간 수명을 요청). subagent의 자체 frontmatter 값이 순위에서 어디에 있는지는 [TTL을 직접 선택](/docs/ko/prompt-caching#choose-the-ttl-yourself)을 참조하십시오

이 예제는 subagents 및 주 대화 외부의 다른 요청에 1시간 수명을 제공합니다:

```json settings.json theme={null}
{
  "subagentPromptCacheTtl": "1h"
}
```

이 키는 [`promptCacheTtl`](#promptcachettl)이 다루지 않는 요청을 다루므로 Claude Code가 만드는 모든 요청에 대해 수명을 선택하려면 둘 다 설정합니다. subagent의 캐시가 주 대화의 캐시와 어떻게 다른지는 [Subagents 및 캐시](/docs/ko/prompt-caching#subagents-and-the-cache)를 참조하십시오.

<h3 id="switchmodelsonflag">
  `switchModelsOnFlag`
</h3>

[안전 분류기가 요청에 플래그를 지정할](/docs/ko/model-config#automatic-model-fallback) 때 어떤 일이 발생하는지 선택합니다. 폴백 모델로 전환하고 계속하거나, 전환과 프롬프트 편집 중에서 선택할 수 있도록 일시 중지합니다.

* **범위**: [`모든 파일`](#scopes). `/config`에 **메시지가 플래그될 때 모델 전환**으로 나타납니다.
* **유형**: 부울
  * `true`: Claude Code는 폴백 모델로 전환하고 계속합니다
  * `false`: 대화형 세션에서 Claude Code는 일시 중지하여 전환과 프롬프트 편집 중에서 선택할 수 있습니다. 대화 상자를 표시할 수 없는 `-p` 실행과 같은 곳에서는 플래그된 요청이 오류로 끝납니다
* **기본값**: `true`, 자동으로 전환

```json settings.json theme={null}
{
  "switchModelsOnFlag": false
}
```

[전환하기 전에 묻기](/docs/ko/model-config#ask-before-switching)를 참조하십시오.

<h3 id="ultracode">
  `ultracode`
</h3>

[ultracode](/docs/ko/workflows#let-claude-decide-with-ultracode)를 사용하여 세션을 시작합니다. 켜져 있으면 Claude는 요청할 때까지 기다리는 대신 각 실질적인 작업에 대해 워크플로우를 계획합니다. Claude는 [동적 워크플로우](/docs/ko/workflows)가 활성화되고, 모델이 `xhigh` 노력을 지원하고, [노력 제한](/docs/ko/model-config#organization-effort-limits)이 `xhigh` 아래에 적용되지 않을 때만 워크플로우를 계획합니다. 어느 쪽이든 `ultracode: true`는 세션을 `xhigh` 노력으로 실행하거나 노력 제한이 더 낮을 때 제한에서 실행합니다. Claude Code는 이 키를 읽지만 절대 작성하지 않습니다. `/effort ultracode`는 현재 세션에만 ultracode를 켭니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: 세션은 `xhigh` 노력으로 시작하고, 동적 워크플로우가 활성화되고, 모델이 `xhigh`를 지원하고, 노력 제한이 `xhigh` 아래에 없을 때 ultracode가 켜집니다
  * `false`: 세션은 ultracode가 꺼진 상태로 시작합니다
* **기본값**: 설정 해제되어 ultracode는 꺼짐
* **세션별 재정의**: `/effort ultracode`는 이 키 없이 한 세션에 대해 ultracode를 켭니다. `--effort ultracode`도 마찬가지이며, Claude Code v2.1.203 이상이 필요합니다

```json settings.json theme={null}
{
  "ultracode": true
}
```

Ultracode는 세션을 `xhigh` 노력으로 실행하고 `effortLevel` 및 [`modelSettings`](#modelsettings) 항목보다 우선합니다. `xhigh` 아래의 [노력 제한](/docs/ko/model-config#organization-effort-limits)(예: [`maxEffortLevel`](#maxeffortlevel) 설정)이 모델에 적용되면 세션은 대신 제한에서 실행되고 ultracode는 꺼집니다. Claude는 자체적으로 워크플로우를 계획하지 않으며, `/effort`는 `ultracode`를 제공하지 않습니다. Agent SDK `apply_flag_settings` 제어 요청도 키를 허용합니다.

<h2 id="permission-settings">
  권한 설정
</h2>

Claude가 묻지 않고 수행할 수 있는 작업, 세션이 시작되는 권한 모드, 자동 모드의 분류기가 허용하는 작업을 결정합니다. 규칙 구문 및 권한 모델에 대해서는 [권한 구성](/docs/ko/permissions)을 참조하십시오.

<h3 id="allowmanagedpermissionrulesonly">
  `allowManagedPermissionRulesOnly`
</h3>

관리되는 설정을 권한 규칙의 유일한 설정 소스로 만듭니다. Claude Code는 사용자, 프로젝트, 로컬 및 `--settings` 파일의 `allow`, `ask`, `deny` 규칙을 무시하고, `--allowedTools`를 무시하며, 권한 프롬프트에서 항상 허용 선택을 숨기고, 새 규칙 저장을 중지합니다.

[포함 호스트의 상위 설정](/docs/ko/managed-settings#let-an-embedding-host-add-policy)이 적용되면 Claude Code는 이를 관리되는 계층의 일부로 취급합니다. 이는 `allow` 규칙과 `additionalDirectories`를 삭제하고, `!`로 시작하는 패턴의 `Read` 및 `Edit` 규칙을 제외한 `deny` 및 `ask` 규칙을 유지합니다. 호스트는 이 키를 설정했는지 여부와 관계없이 `!` 규칙으로 관리되는 규칙에서 경로를 제외할 수 없습니다.

`--disallowedTools` 규칙과 현재 세션의 `deny` 및 `ask` 규칙은 Claude Code가 세션 중간에 설정을 다시 로드한 후에도 계속 적용됩니다. 이들은 제한만 하므로 관리되는 규칙이 부여하는 것을 확대할 수 없습니다. v2.1.257 이전에는 Claude Code가 첫 번째 설정 다시 로드에서 해당 명령줄 및 세션 규칙을 삭제했습니다.

`--disallowedTools` 또는 세션 규칙의 `!` 패턴이 제외할 수 있는 것에 대해서는 [Read 및 Edit 규칙](/docs/ko/permissions#read-and-edit)을 참조하십시오.

* **범위**: [`Managed`](#scopes)
* **유형**: Boolean
  * `true`: 관리되는 설정이 권한 규칙의 유일한 설정 소스가 됩니다
  * `false`: Claude Code는 관리되는 규칙 외에도 사용자, 프로젝트, 로컬 및 `--settings` 파일의 권한 규칙을 적용합니다
* **기본값**: 설정되지 않음. Claude Code는 관리되는 규칙 외에도 사용자, 프로젝트 및 로컬 설정과 `--settings`의 권한 규칙을 적용합니다

```json managed-settings.json theme={null}
{
  "allowManagedPermissionRulesOnly": true
}
```

이 키는 MCP 서버 허용 목록을 잠그지 않습니다. 이를 위해서는 [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly)를 설정하십시오. [관리되는 전용 설정](/docs/ko/managed-settings#managed-only-settings)을 참조하십시오.

<h3 id="automode">
  `autoMode`
</h3>

[자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode) 분류기가 차단하고 허용하는 것에 자신의 규칙을 추가합니다. 이를 사용하여 분류기에 조직이 신뢰하는 저장소, 버킷 및 도메인을 알려주므로 일상적인 내부 작업 차단을 중지합니다. 분류기는 [기본 제공 허용 및 거부 규칙](/docs/ko/auto-mode-config#inspect-the-defaults-and-your-effective-config)과 함께 제공됩니다. 배열에 리터럴 문자열 `"$defaults"`를 포함하여 해당 위치에서 기본 제공 규칙을 유지하고 주변에 규칙을 추가합니다. 이를 생략하면 규칙으로 대체합니다.

* **범위**: [`User or managed`](#scopes)
* **유형**: `environment`, `allow`, `soft_deny` 및 `hard_deny` 산문 규칙 배열과 [`classifyAllShell`](#automode-classifyallshell) Boolean을 포함하는 객체
* **기본값**: 설정되지 않음. 분류기는 [기본 제공 규칙](/docs/ko/auto-mode-config#inspect-the-defaults-and-your-effective-config)만 사용합니다

이 예제는 `"$defaults"`를 통해 기본 제공 `soft_deny` 규칙을 유지하고 `terraform apply`를 차단하는 규칙을 하나 더 추가합니다:

```json settings.json theme={null}
{
  "autoMode": {
    "soft_deny": ["$defaults", "Never run terraform apply"]
  }
}
```

이러한 파일 중 두 개 이상이 동일한 배열을 설정하면 Claude Code는 항목을 연결합니다. 규칙 형식 및 각 배열이 적용되는 방식에 대해서는 [자동 모드 구성](/docs/ko/auto-mode-config)을 참조하십시오.

<h3 id="automode-classifyallshell">
  `autoMode.classifyAllShell`
</h3>

자동 모드가 활성화되어 있는 동안 모든 Bash 및 PowerShell 명령을 자동 모드 분류기를 통해 보냅니다. 기본적으로 자동 모드는 임의 코드를 실행할 수 있는 허용 규칙만 일시 중단합니다: `Bash(*)` 같은 도구 전체 및 와일드카드 규칙, 그리고 `Bash(python *)` 같은 인터프리터 또는 셸 래퍼 접두사입니다. `Bash(npm test)` 같은 다른 허용 규칙과 일치하는 명령은 [자동 모드의 명령별 허용 도메인](/docs/ko/sandboxing#per-command-allowed-domains-in-auto-mode)을 포함하지 않는 한 분류기를 건너뜁니다. 건너뛸 때 규칙의 접두사가 예상하지 못한 파괴적 인수가 보이지 않은 채로 통과할 수 있습니다. 이 키를 설정하면 세션의 모든 셸 허용 규칙이 일시 중단되어 분류기가 모든 명령을 봅니다. Claude Code v2.1.193 이상이 필요합니다.

* **범위**: [`User or managed`](#scopes). [`autoMode`](#automode)가 읽히는 곳에서 읽습니다.
* **유형**: Boolean
  * `true`: 자동 모드가 활성화되어 있는 동안 Claude Code는 모든 Bash 및 PowerShell 명령을 분류기를 통해 보내고 셸 허용 규칙을 일시 중단합니다. 자동 모드 외부에서는 규칙이 계속 적용됩니다
  * `false`: 자동 모드는 `Bash(*)` 및 `Bash(python *)` 같은 임의 코드를 실행할 수 있는 허용 규칙만 일시 중단합니다. `Bash(npm test)` 같은 다른 허용 규칙과 일치하는 명령은 [자동 모드의 명령별 허용 도메인](/docs/ko/sandboxing#per-command-allowed-domains-in-auto-mode)을 포함하지 않는 한 분류기를 건너뛰고, 다른 모든 셸 명령은 이를 통해 이동합니다
* **기본값**: `false`

```json settings.json theme={null}
{
  "autoMode": {
    "classifyAllShell": true
  }
}
```

[분류기를 통해 모든 셸 명령 라우팅](/docs/ko/auto-mode-config#route-all-shell-commands-through-the-classifier)을 참조하십시오. Claude Code v2.1.193 이상이 필요합니다.

<h3 id="disableautomode">
  `disableAutoMode`
</h3>

`Shift+Tab` 사이클에서 [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)를 제거합니다. `--permission-mode auto`, 설정 파일 또는 기본 제공 기본값에서 [자동 모드로 시작](/docs/ko/permission-modes#which-mode-a-session-starts-in)할 수 있는 모든 세션은 대신 `default`로 시작합니다. 관리자는 조직의 개발자가 자동 모드를 사용하지 못하도록 관리되는 설정에서 이를 설정합니다.

* **범위**: [`Any file`](#scopes). [관리되는 설정](/docs/ko/managed-settings)에서 가장 유용합니다. 여기서 사용자는 이를 재정의할 수 없습니다. `permissions` 아래에서 `permissions.disableAutoMode`로도 허용됩니다.
* **유형**: 문자열 `"disable"`
* **기본값**: 설정되지 않음

```json settings.json theme={null}
{
  "disableAutoMode": "disable"
}
```

<h3 id="permissions">
  `permissions`
</h3>

Claude가 묻지 않고 사용할 수 있는 도구, 항상 프롬프트하는 도구, 차단된 도구를 제어하고 세션이 시작되는 [권한 모드](/docs/ko/permission-modes)를 설정합니다. 아래의 모든 `permissions.*` 키는 이 객체 아래에 중첩됩니다.

* **범위**: [`Any file`](#scopes)
* **유형**: `allow`, `ask`, `deny`, `additionalDirectories`, `blockReadsOutsideWorkingDirectories`, `defaultMode`, `disableBypassPermissionsMode` 및 `disableAutoMode`를 포함하는 객체
* **기본값**: 설정되지 않음

이 예제는 `npm run` 명령을 묻지 않고 승인하고, `git push` 전에 프롬프트하고, `.env` 읽기를 차단하고, 세션을 `acceptEdits`로 시작합니다:

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

세 규칙 배열은 하나의 구문을 공유합니다. `permissions.allow` 아래의 [권한 규칙 구문](#permission-rule-syntax)을 참조하십시오. 다양한 파일의 권한 규칙이 어떻게 결합되는지에 대해서는 [범위 전체에서 권한 규칙이 병합되는 방식](/docs/ko/permissions#settings-precedence)을 참조하십시오. 일반적으로 설정 키가 어떻게 결합되는지에 대해서는 설정 가이드의 [설정 우선순위](/docs/ko/settings#settings-precedence)를 참조하십시오.

<h3 id="useautomodeduringplan">
  `useAutoModeDuringPlan`
</h3>

Claude Code가 계획 모드에서 셸 명령을 검토하기 위해 자동 모드 분류기를 사용할지 여부를 선택합니다. 기본값 `true`를 사용하면 자동 모드를 사용할 수 있고 프롬프트가 표시되지 않을 때 계획 중에 분류기가 각 명령을 검토합니다. 기본 제공 읽기 전용 집합 외부의 모든 명령에 대해 권한 프롬프트를 받으려면 `false`로 설정합니다. `/config`에 **계획 중 자동 모드 사용**으로 나타납니다.

* **범위**: [`User, local, or managed`](#scopes). 저장소는 이를 끌 수 없습니다.
* **유형**: Boolean
  * `true`: 설정되지 않은 것과 동일합니다. 자동 모드를 사용할 수 있으면 분류기는 계획 중에 각 셸 명령을 검토하고 이에 대해 프롬프트하지 않습니다. 이러한 파일 중 하나의 `false`는 여전히 이를 끕니다
  * `false`: 기본 제공 읽기 전용 집합 외부의 모든 명령에 대해 권한 프롬프트를 받습니다
* **기본값**: `true`

```json settings.json theme={null}
{
  "useAutoModeDuringPlan": false
}
```

<h3 id="permissions-allow">
  `permissions.allow`
</h3>

Claude Code가 묻지 않고 승인하는 도구 사용을 나열합니다. MCP 규칙에서 `*`는 `mcp__github__get_*` 같은 `mcp__<server>__` 접두사 뒤의 도구 이름에만 나타날 수 있습니다. 서버 이름에는 나타날 수 없습니다.

* **범위**: [`Any file`](#scopes)
* **유형**: 권한 규칙 문자열 배열
* **기본값**: 설정되지 않음
* **세션별 재정의**: `--allowedTools`는 한 세션에 대해 허용 규칙을 추가하고, 모든 설정 파일의 거부 규칙은 여전히 이름을 지정한 도구를 차단합니다

이 예제는 `git diff`를 승인하고 Claude Code가 묻지 않고 `.zshrc`를 읽도록 합니다:

```json settings.json theme={null}
{
  "permissions": {
    "allow": ["Bash(git diff *)", "Read(~/.zshrc)"]
  }
}
```

Claude Code는 프로젝트의 `.claude/settings.json`의 `allow` 규칙을 해당 폴더에 대한 [작업 공간 신뢰 대화](/docs/ko/permissions#project-allow-rules-and-workspace-trust)를 수락한 후에만 적용합니다.

<h4 id="permission-rule-syntax">
  권한 규칙 구문
</h4>

권한 규칙은 `Tool` 또는 `Tool(specifier)` 형식을 따릅니다. Claude Code는 `deny` 규칙을 먼저 평가한 다음 `ask`, 그 다음 `allow`를 평가하고, 첫 번째 일치가 각 규칙이 얼마나 구체적인지에 관계없이 결정합니다. [권한 규칙 평가 순서](/docs/ko/permissions#manage-permissions)를 참조하십시오.

각 행은 하나의 규칙 형태와 일치하는 것을 보여줍니다.

| 규칙                             | 일치하는 것                  |
| :----------------------------- | :---------------------- |
| `Bash`                         | 모든 Bash 명령              |
| `Bash(npm run *)`              | `npm run`으로 시작하는 명령     |
| `Read(./.env)`                 | `.env` 파일 읽기            |
| `WebFetch(domain:example.com)` | example.com에 대한 가져오기 요청 |

와일드카드 동작, Read, Edit, WebFetch, MCP 및 Agent 규칙에 대한 도구별 패턴, Bash 패턴의 보안 제한을 포함한 전체 규칙 구문에 대해서는 [권한 규칙 구문](/docs/ko/permissions#permission-rule-syntax)을 참조하십시오.

<h3 id="permissions-ask">
  `permissions.ask`
</h3>

`acceptEdits` 또는 `bypassPermissions` 같은 권한 모드에서 그렇지 않으면 승인할 도구 사용에 대해 확인을 위해 프롬프트하는 도구 사용을 나열합니다. `dontAsk` 모드에서 Claude Code는 프롬프트하는 대신 일치하는 도구 사용을 거부합니다.

* **범위**: [`Any file`](#scopes)
* **유형**: 권한 규칙 문자열 배열
* **기본값**: 설정되지 않음

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

Claude Code가 차단하는 도구 사용을 나열합니다. API 키, 비밀 또는 환경 값을 보유하는 파일에 사용합니다: Claude Code는 파일 검색 및 검색 결과에서 일치하는 파일을 제외하고, 이들의 읽기를 거부하고, 일치하는 경로에서 [Edit 및 Write 도구](/docs/ko/permissions#read-and-edit)를 차단합니다.

Read 및 Edit 거부 규칙은 Claude의 기본 제공 파일 도구, Claude Code가 `cat`, `head`, `tail`, `sed` 및 `tee` 같은 Bash에서 인식하는 파일 명령, 그리고 `> file` 및 `< file` 같은 Bash [리디렉션](/docs/ko/permissions#redirections)의 대상에 적용됩니다. 이들은 `grep -r pattern .` 같이 파일을 명명하지 않고 파일을 읽는 명령이나 임의 부프로세스에는 적용되지 않으므로 OS 수준 적용을 위해 [샌드박스를 활성화](/docs/ko/sandboxing)하십시오.

* **범위**: [`Any file`](#scopes)
* **유형**: 권한 규칙 문자열 배열
* **기본값**: 설정되지 않음
* **세션별 재정의**: `--disallowedTools`는 이 키와 함께 한 세션에 대해 거부 규칙을 추가합니다

이 예제는 `.env` 파일, `secrets` 디렉토리 및 자격 증명 파일의 읽기를 거부하고 `curl` 명령을 차단합니다:

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

도구 이름은 glob 패턴을 허용하므로 `"*"`는 모든 도구를 거부하고 `"mcp__*"`는 모든 MCP 도구를 거부합니다. Claude Code는 다른 도구가 여전히 Claude에 사용 가능한 한 [`EndConversation`](/docs/ko/tools-reference#endconversation-tool-behavior) 도구에 대한 거부 규칙을 무시합니다. `Bash` 거부 규칙은 Claude가 작성한 명령과 일치하므로 `Bash(curl *)`는 `/usr/bin/curl` 또는 `sh -c 'curl …'`를 중지하지 않습니다. [Bash 규칙이 일치하지 않는 것](/docs/ko/permissions#bash-rule-limits)을 참조하십시오. 이 키는 더 이상 사용되지 않는 `ignorePatterns` 구성을 대체합니다.

<h3 id="permissions-additionaldirectories">
  `permissions.additionalDirectories`
</h3>

시작한 디렉토리 외부의 디렉토리에 대한 Claude 파일 액세스를 추가 [작업 디렉토리](/docs/ko/permissions#working-directories)로 제공합니다. 대부분의 `.claude/` 구성은 이러한 디렉토리에서 [검색되지 않습니다](/docs/ko/permissions#additional-directories-grant-file-access-not-configuration).

* **범위**: [`Any file`](#scopes)
* **유형**: 디렉토리 경로 배열
* **기본값**: 설정되지 않음
* **세션별 재정의**: `--add-dir` 및 `/add-dir`은 이 키와 함께 한 세션에 대해 디렉토리를 추가합니다

```json settings.json theme={null}
{
  "permissions": {
    "additionalDirectories": ["../docs/"]
  }
}
```

`allow` 규칙과 마찬가지로 프로젝트의 `.claude/settings.json`의 항목은 해당 폴더에 대한 [작업 공간 신뢰 대화](/docs/ko/permissions#project-allow-rules-and-workspace-trust)를 수락한 후에만 적용됩니다.

<h3 id="permissions-blockreadsoutsideworkingdirectories">
  `permissions.blockReadsOutsideWorkingDirectories`
</h3>

Claude가 Read, Grep, Glob 및 LSP 도구를 사용하여 세션의 [작업 디렉토리](/docs/ko/permissions#working-directories) 외부의 경로를 읽지 못하도록 중지합니다. `bypassPermissions`를 포함한 모든 권한 모드에서 이를 수행합니다. Claude Code가 인식하는 `cat` 같은 파일 명령을 통해 일치하는 경로를 읽는 Bash 명령은 자동 모드 및 `bypassPermissions` 모드에서도 프롬프트합니다. Claude Code v2.1.257 이상이 필요합니다.

셸 파서가 추적할 수 없는 Bash 명령(예: 두 번 이상 디렉토리를 변경하거나 부셸을 실행하는 명령)은 자동 모드 및 `bypassPermissions` 모드에서도 프롬프트합니다. 명령이 작업 디렉토리 외부의 경로를 명명하지 않아도 프롬프트가 나타납니다. 이 프롬프트는 명령이 [샌드박스](/docs/ko/sandboxing)에서 실행되고 샌드박스가 블록을 적용할 때는 적용되지 않습니다.

Claude Code는 또한 [자동 모드의 작업 디렉토리 외부의 첫 번째 읽기 전 프롬프트](/docs/ko/permission-modes#first-read-outside-the-working-directories)에서 이러한 읽기를 차단하도록 선택할 때 여기에 `true`를 작성합니다.

* **범위**: [`Any file`](#scopes). 모든 설정 소스가 `true`를 설정하면 블록이 적용되므로 저장소의 체크인된 파일은 프로젝트에 대해 블록을 켤 수 있지만 설정한 블록을 해제할 수 없습니다.
* **유형**: Boolean
  * `true`: 작업 디렉토리 외부의 파일 읽기가 차단됩니다
  * `false`: 설정되지 않은 것과 동일합니다. 다른 설정 파일의 `true`는 여전히 차단합니다
* **기본값**: 설정되지 않음. 작업 디렉토리 외부의 읽기는 권한 모드 및 규칙을 따릅니다

```json settings.json theme={null}
{
  "permissions": {
    "blockReadsOutsideWorkingDirectories": true
  }
}
```

저장소의 체크인된 설정 파일만 디렉토리를 추가하면 블록은 여전히 거기의 읽기에 적용됩니다. [`autoMemoryDirectory`](#automemorydirectory)가 프로젝트의 `.claude/settings.json`에서 오거나 [저장소 제공으로 취급되는](/docs/ko/permissions#when-your-local-settings-file-needs-trust) `.claude/settings.local.json`에서 올 때 Claude Code는 해당 디렉토리에서 [자동 메모리](/docs/ko/memory#storage-location)를 로드하지 않고 이를 저장하지 않습니다. Claude Code 자체가 필요한 파일은 읽을 수 있습니다. 예를 들어 `~/.claude/` 아래의 기술, 플러그인, 규칙, 에이전트, 명령 및 `CLAUDE.md` 메모리 파일입니다.

[샌드박스](/docs/ko/sandboxing)가 켜져 있으면 블록은 또한 작업 디렉토리 외부의 홈 디렉토리 및 마운트된 볼륨 루트에 대한 샌드박스된 명령 읽기 액세스를 거부합니다. [샌드박스 외부에서 실행](/docs/ko/sandboxing#the-unsandboxed-retry-escape-hatch)하기 위해 승인이 필요한 재시도는 `bypassPermissions` 모드에서도 프롬프트합니다. 도구가 `~/.gitconfig` 같은 홈 디렉토리에서 읽는 파일은 나머지와 함께 거부됩니다. 도구가 필요할 때 [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread)로 특정 경로를 다시 열어야 합니다.

세션의 작업 디렉토리가 Claude Code가 세션 중간에 들어간 것을 포함한 연결된 [git worktree](/docs/ko/worktrees)일 때 저장소의 공통 `.git` 디렉토리는 샌드박스된 명령에 대해 읽을 수 있고 쓸 수 있으므로 git이 거기서 계속 작동합니다.

<h3 id="permissions-defaultmode">
  `permissions.defaultMode`
</h3>

새 세션이 시작되는 [권한 모드](/docs/ko/permission-modes)를 설정합니다. 이를 설정하지 않으면 세션은 계획 및 표면에 대한 [기본 제공 기본값](/docs/ko/permission-modes#which-mode-a-session-starts-in)으로 시작합니다.

* **범위**: [`Any file`](#scopes). `auto` 및 `bypassPermissions`는 프로젝트 또는 로컬 설정에서 적용되지 않으므로 대신 `~/.claude/settings.json`에서 설정합니다. v2.1.257 이전에는 `bypassPermissions`가 모든 파일에서 적용되었습니다. VS Code 확장이 시작하는 대화의 경우 Claude Code는 사용자, 관리되는 및 `--settings` 값만 읽습니다.
* **유형**: 문자열, 다음 중 하나:
  * `"default"`: Claude Code는 묻지 않고 읽기만 실행합니다
  * `"acceptEdits"`: Claude Code는 또한 파일 편집 및 `mkdir` 및 `mv` 같은 일반적인 파일 시스템 명령을 묻지 않고 실행합니다
  * `"plan"`: Claude Code는 읽고 계획하지만 승인할 계획까지 편집을 차단합니다
  * `"auto"`: Claude Code는 모든 것을 실행하고 백그라운드 안전 검사를 수행합니다
  * `"dontAsk"`: Claude Code는 그렇지 않으면 프롬프트할 모든 호출을 자동으로 거부합니다. 읽기, 승인이 필요 없는 다른 작업 및 사전 승인된 도구는 여전히 실행됩니다
  * `"bypassPermissions"`: Claude Code는 묻지 않고 모든 것을 실행합니다
  * `"manual"`: Claude Code v2.1.200 이상에서 `"default"`의 별칭입니다
* **기본값**: 설정되지 않음
* **세션별 재정의**: `--permission-mode` 및 `bypassPermissions`에 대한 동등한 `--dangerously-skip-permissions`는 한 세션에 대해 이 키보다 우선합니다

```json settings.json theme={null}
{
  "permissions": {
    "defaultMode": "acceptEdits"
  }
}
```

권한 규칙은 모든 모드 위에 계층화됩니다: `deny` 규칙은 `bypassPermissions`를 포함한 모든 모드에서 차단합니다. [권한 모드](/docs/ko/permission-modes)를 참조하십시오. `manual`은 CLI 및 VS Code 확장에서 Manual로 레이블이 지정된 권한 모드를 명명합니다. 별칭은 Claude Code v2.1.200 이상이 필요합니다. 클라우드 세션에서 Claude Code는 이 키에서 `acceptEdits`, `plan`, `default` 및 `auto`만 준수합니다. VS Code 확장이 시작하는 대화의 경우 [시작 권한 모드에 대해 확장이 읽는 설정](/docs/ko/permission-modes#switch-permission-modes)을 참조하십시오.

<h3 id="permissions-disablebypasspermissionsmode">
  `permissions.disableBypassPermissionsMode`
</h3>

누구도 `bypassPermissions` 모드에 들어가지 못하도록 방지합니다. Claude Code는 `--dangerously-skip-permissions` 플래그를 거부하고 [에이전트 정의](/docs/ko/sub-agents#permission-modes)의 `permissionMode: bypassPermissions`를 무시하므로 부에이전트는 상위 세션의 권한 모드로 실행됩니다.

* **범위**: [`Any file`](#scopes). 일반적으로 조직 정책을 적용하기 위해 [관리되는 설정](/docs/ko/managed-settings)에서 설정됩니다.
* **유형**: 문자열 `"disable"`
* **기본값**: 설정되지 않음
* **세션별 재정의**: 이 키는 `--dangerously-skip-permissions`보다 우선하며, 키가 설정되어 있는 동안 Claude Code는 이를 거부합니다

```json settings.json theme={null}
{
  "permissions": {
    "disableBypassPermissionsMode": "disable"
  }
}
```

v2.1.223 이전에는 Claude Code가 바이패스가 비활성화된 상태에서도 프론트매터 권한 모드를 적용했습니다.

<h3 id="skipautopermissionprompt">
  `skipAutoPermissionPrompt`
</h3>

자신의 설정 또는 모드 선택기를 통해 자동 모드에 직접 들어갈 때 Claude Code가 표시하는 [자동 모드](/docs/ko/permission-modes#eliminate-prompts-with-auto-mode)를 설명하는 일회성 공지를 건너뜁니다. 기본 제공 기본값이 세션을 자동 모드로 시작할 때가 아닙니다. Claude Code는 해당 공지를 한 번 표시한 다음 표시되었음을 기록하므로 이 키는 공지가 아직 나타나지 않은 곳에서만 중요합니다.

* **범위**: [`User or managed`](#scopes). 저장소는 이를 설정할 수 없습니다.
* **유형**: Boolean
  * `true`: Claude Code는 공지를 건너뜁니다
  * `false`: 설정되지 않은 것과 동일합니다. 이러한 파일 중 하나가 `true`를 설정하지 않으면 공지가 한 번 나타납니다
* **기본값**: 설정되지 않음. 공지가 한 번 나타납니다

```json settings.json theme={null}
{
  "skipAutoPermissionPrompt": true
}
```

<h3 id="skipdangerousmodepermissionprompt">
  `skipDangerousModePermissionPrompt`
</h3>

Claude Code가 `--dangerously-skip-permissions` 또는 `defaultMode: "bypassPermissions"`에서 `bypassPermissions` 모드에 들어가기 전에 표시하는 확인 대화를 건너뜁니다. Claude Code는 해당 대화를 한 번 수락할 때 사용자 설정에 여기에 `true`를 작성합니다.

* **범위**: [`User, local, or managed`](#scopes). 신뢰할 수 없는 저장소는 대화를 건너뛸 수 없습니다.
* **유형**: Boolean
  * `true`: Claude Code는 세션이 `bypassPermissions` 모드에 들어가기 전에 확인 대화를 건너뜁니다
  * `false`: 설정되지 않은 것과 동일합니다. 이러한 파일 중 하나가 `true`를 설정하지 않으면 대화가 나타납니다
* **기본값**: 설정되지 않음. 대화가 나타납니다

```json settings.json theme={null}
{
  "skipDangerousModePermissionPrompt": true
}
```

<h2 id="sandbox-settings">
  샌드박스 설정
</h2>

Claude가 실행하는 명령을 파일시스템, 네트워크, 자격증명으로부터 격리합니다. 샌드박싱 작동 방식 및 플랫폼 요구사항은 [샌드박싱](/docs/ko/sandboxing)을 참조하십시오.

<h3 id="sandbox">
  `sandbox`
</h3>

[샌드박싱](/docs/ko/sandboxing)을 사용하여 Claude가 실행하는 Bash 명령을 파일시스템 및 네트워크로부터 격리합니다. `enabled`로 샌드박스를 켜고, `filesystem`, `network`, `credentials` 하위 객체로 샌드박스된 명령이 접근할 수 있는 범위를 좁히거나 넓힙니다. 샌드박스는 macOS, Linux, WSL2에서 실행됩니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: `enabled`, `failIfUnavailable`, `autoAllowBashIfSandboxed`, `excludedCommands`, `allowUnsandboxedCommands`, `enableWeakerNestedSandbox`, `enableWeakerNetworkIsolation`, `allowAppleEvents`, `bwrapPath`, `socatPath`, `ignoreViolations`, `ripgrep`을 포함하는 객체이며, `filesystem`, `network`, `credentials` 객체도 포함합니다.
* **기본값**: 설정되지 않음. Claude Code는 샌드박스 없이 명령을 실행합니다.

다음은 샌드박스를 켜고, 샌드박스된 명령에 대한 권한 프롬프트를 건너뛰고, `docker`를 샌드박스 외부에서 실행하고, 두 개의 추가 쓰기 경로를 열고, AWS 자격증명 파일을 숨기고, GitHub 및 npm을 미리 허용합니다:

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

Claude Code는 Boolean 키의 값을 가장 높은 우선순위 설정 범위에서 가져오므로, 관리되는 `enabled` 또는 `failIfUnavailable`은 개발자가 설정한 모든 것을 재정의합니다. 배열 키는 세션이 로드하는 모든 설정 범위에서 병합되므로, 개발자는 항목을 추가할 수 있습니다. 관리되는 전용 잠금은 [개발자가 정책을 확대하지 못하도록 유지](/docs/ko/sandboxing#keep-developers-from-widening-the-policy)를 참조하십시오. 조직에 샌드박스를 요구하려면 [관리되는 설정으로 샌드박싱 적용](/docs/ko/sandboxing#enforce-sandboxing-with-managed-settings)을 참조하십시오.

<h3 id="sandbox-enabled">
  `sandbox.enabled`
</h3>

Bash 명령에 대해 [샌드박싱](/docs/ko/sandboxing)을 켭니다. `/sandbox` 패널에서 모드를 선택하면, Claude Code는 현재 프로젝트의 `.claude/settings.local.json`에 이 키를 씁니다. 모든 프로젝트를 샌드박스하려면 `~/.claude/settings.json`에 설정하십시오.

* **범위**: [`모든 파일`](#scopes)
* **유형**: Boolean
  * `true`: Claude Code는 Bash 명령을 샌드박스합니다.
  * `false`: Bash 명령은 샌드박스 없이 실행됩니다.
* **기본값**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true
  }
}
```

Linux 및 WSL2에서 샌드박스는 `bubblewrap` 및 `socat`이 필요합니다. [Linux 및 WSL2 설정](/docs/ko/sandboxing#set-up-linux-and-wsl2)을 참조하십시오. 샌드박스를 시작할 수 없을 때, Claude Code는 경고를 표시하고 [`failIfUnavailable`](#sandbox-failifunavailable)도 설정하지 않으면 명령을 샌드박스 없이 실행합니다.

<h3 id="sandbox-failifunavailable">
  `sandbox.failIfUnavailable`
</h3>

`sandbox.enabled`가 `true`이지만 샌드박스를 시작할 수 없을 때(종속성이 누락되었거나 플랫폼이 지원되지 않음) Claude Code가 시작 시 오류로 종료되도록 합니다. 이 설정이 없으면, Claude Code는 경고를 표시하고 명령을 샌드박스 없이 실행합니다. 조직에서 샌드박싱을 필수로 요구할 때 관리되는 설정에서 사용하십시오.

* **범위**: [`모든 파일`](#scopes)
* **유형**: Boolean
  * `true`: `sandbox.enabled`가 `true`이지만 샌드박스를 시작할 수 없을 때 Claude Code는 시작 시 오류로 종료됩니다.
  * `false`: Claude Code는 경고를 표시하고 명령을 샌드박스 없이 실행합니다.
* **기본값**: `false`

다음은 모든 관리되는 머신이 명령을 샌드박스하거나 시작을 거부하도록 합니다:

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "failIfUnavailable": true
  }
}
```

[관리되는 설정으로 샌드박싱 적용](/docs/ko/sandboxing#enforce-sandboxing-with-managed-settings)을 참조하십시오.

<h3 id="sandbox-autoallowbashifsandboxed">
  `sandbox.autoAllowBashIfSandboxed`
</h3>

Claude Code가 권한 프롬프트 없이 샌드박스된 Bash 명령을 실행하도록 합니다. 샌드박스에서 실행할 수 없는 명령은 여전히 일반 권한 흐름을 거치며, `deny` 규칙 및 `Bash(git push *)`와 같은 콘텐츠 범위 `ask` 규칙은 여전히 적용됩니다. 단순 `Bash` ask 규칙은 샌드박스된 명령에 대해 건너뜁니다. 이를 `false`로 설정하여 샌드박스된 명령도 일반 권한 흐름을 거치도록 하면, `/sandbox` **모드** 탭에서 이를 일반 권한 모드라고 부릅니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: Boolean
  * `true`: Claude Code는 `deny` 규칙 및 콘텐츠 범위 `ask` 규칙을 따르면서 권한 프롬프트 없이 샌드박스된 Bash 명령을 실행합니다. `CLAUDE_CODE_SUBPROCESS_ENV_SCRUB`는 자동 허용을 끕니다.
  * `false`: 샌드박스된 명령은 일반 권한 흐름을 거치므로, 허용 규칙 및 권한 모드가 결정합니다. `/sandbox` **모드** 탭에서 이를 일반 권한 모드라고 부릅니다.
* **기본값**: `true`

다음은 샌드박스를 켜고 샌드박스된 명령을 일반 권한 흐름을 거치도록 합니다:

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "autoAllowBashIfSandboxed": false
  }
}
```

[샌드박스 모드](/docs/ko/sandboxing#sandbox-modes)에서 자동 허용 모드가 여전히 프롬프트하는 것과 계획 모드에서의 동작을 참조하십시오.

<h3 id="sandbox-excludedcommands">
  `sandbox.excludedCommands`
</h3>

Claude Code가 항상 샌드박스 외부에서 실행하는 명령(예: 샌드박스에서 작동하지 않는 도구)을 지정합니다. 각 항목은 `Bash(...)` [권한 규칙](/docs/ko/permissions#permission-rule-syntax)의 콘텐츠와 동일한 구문을 사용합니다: 정확한 명령, `docker *`와 같은 접두사, 또는 와일드카드 패턴입니다.

항목이 복합 명령의 모든 명령을 포함할 때만 항목이 Bash 호출을 샌드박스 외부로 꺼냅니다. 일부 호출 형태는 그렇더라도 샌드박스된 상태로 유지됩니다. `docker *` 항목만으로는 `npm ci && docker build .`를 샌드박스 외부로 꺼내지 않습니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 명령 패턴의 배열
* **기본값**: 설정되지 않음. 명령이 제외되지 않습니다.

```json settings.json theme={null}
{
  "sandbox": {
    "excludedCommands": ["docker *"]
  }
}
```

Claude Code는 다음과 같은 형태 중 하나를 가질 때 Bash 호출을 샌드박스된 상태로 유지합니다:

* `sudo`, `eval`, 또는 `xargs`로 시작하는 명령
* 호출의 어디든 나타나는 `cd`, `pushd`, 또는 `popd`
* 명령 대체, 하위 셸, 또는 `if` 또는 `for`와 같은 제어 흐름 블록
* `docker build . > build.log`와 같은 리다이렉션(파일 디스크립터를 중복하는 것 제외, `2>&1`처럼)
* 변수에서 오는 명령 이름

예를 들어, `cd build && docker compose up`은 `docker *` 항목 아래에서 샌드박스된 상태로 유지되며, `cd` 항목을 추가해도 변경되지 않습니다.

제외된 명령은 여전히 일반 권한 흐름을 거칩니다. 제외는 편의 기능이지 보안 경계가 아닙니다. 도구가 특정 위치에만 쓰기를 필요로 할 때는 [`filesystem.allowWrite`](#sandbox-filesystem-allowwrite)를 선호하십시오. Claude Code는 세션이 로드하는 모든 설정 범위에서 항목을 병합하며, 이 목록에 대한 관리되는 전용 잠금이 없으므로, 관리되는 목록을 좁게 유지하십시오.

<h3 id="sandbox-allowunsandboxedcommands">
  `sandbox.allowUnsandboxedCommands`
</h3>

Claude가 `dangerouslyDisableSandbox` 매개변수를 사용하여 샌드박스가 명령을 차단한 후 샌드박스 외부에서 명령을 재시도하도록 합니다. 이를 `false`로 설정하면 Claude Code가 해당 매개변수를 완전히 무시하고 Claude가 실행하는 모든 명령은 샌드박스되거나 [`excludedCommands`](#sandbox-excludedcommands)에 나타나야 합니다. `/sandbox` **재정의** 탭은 해당 상태를 **엄격한 샌드박스 모드**로 표시합니다. 엄격한 샌드박싱을 요구하는 정책에 대해 관리되는 설정에서 `false`를 사용하십시오.

* **범위**: [`모든 파일`](#scopes)
* **유형**: Boolean
  * `true`: Claude는 `dangerouslyDisableSandbox` 매개변수를 사용하여 샌드박스가 명령을 차단한 후 샌드박스 외부에서 명령을 재시도할 수 있습니다.
  * `false`: Claude Code는 해당 매개변수를 무시하므로, Claude가 실행하는 모든 명령은 샌드박스되거나 `excludedCommands`에 나타나야 합니다.
* **기본값**: `true`

다음은 관리되는 설정이 적용되는 모든 사람에 대해 엄격한 샌드박스 모드를 적용합니다:

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "allowUnsandboxedCommands": false
  }
}
```

샌드박스 없는 재시도는 일반 권한 흐름을 거치며, 수동 모드에서 프롬프트가 표시됩니다. [샌드박스 없는 재시도 탈출 해치](/docs/ko/sandboxing#the-unsandboxed-retry-escape-hatch)를 참조하십시오.

[`!` 셸 모드 프롬프트](/docs/ko/interactive-mode#shell-mode-with-prefix)에서 직접 입력한 명령이 샌드박스되는 시기를 보려면 [엄격한 샌드박스 모드](/docs/ko/sandboxing#the-unsandboxed-retry-escape-hatch)를 참조하십시오.

<h3 id="sandbox-filesystem">
  `sandbox.filesystem`
</h3>

샌드박스된 명령이 읽고 쓸 수 있는 경로를 제어합니다. 기본적으로 작업 디렉토리, 세션 임시 디렉토리, `--add-dir`, `/add-dir` 또는 `permissions.additionalDirectories`로 추가한 디렉토리에 쓸 수 있으며, 자격증명 파일을 포함한 파일시스템의 나머지 부분을 읽을 수 있습니다. 네 개의 경로 목록으로 범위를 좁히거나 넓히거나, `disabled`로 파일시스템 계층을 끕니다. [파일시스템 격리](/docs/ko/sandboxing#filesystem-isolation)에서 기본 경계를 참조하십시오.

* **범위**: [`모든 파일`](#scopes)
* **유형**: `allowWrite`, `denyWrite`, `denyRead`, `allowRead` 배열을 포함하는 객체이며, `allowManagedReadPathsOnly` 및 `disabled` Boolean도 포함합니다.
* **기본값**: 설정되지 않음. 기본 읽기 및 쓰기 경계가 적용됩니다.

다음은 샌드박스된 명령이 빌드 디렉토리 및 kubeconfig에 쓸 수 있도록 하고 AWS 자격증명 파일을 숨깁니다:

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

Claude Code는 OS 샌드박스 경계에서 이러한 목록을 적용하므로, `kubectl`, `terraform`, `npm`과 같이 샌드박스된 명령이 시작하는 모든 하위 프로세스에 적용되며, Claude의 파일 도구에만 적용되지 않습니다. Claude Code는 [권한 규칙](/docs/ko/sandboxing#permission-rules)을 동일한 목록에 추가합니다: `Edit` 허용 및 거부 규칙을 `allowWrite` 및 `denyWrite`에, `Read` 거부 규칙을 `denyRead`에, `WebFetch(domain:...)` 허용 및 거부 규칙을 [`network`](#sandbox-network) 도메인 목록에 추가합니다.

관리되는 전용 잠금이 설정되지 않으면, Claude Code는 세션이 로드하는 모든 설정 파일에서 모든 목록을 병합합니다. [`allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly)는 `allowRead`를 관리되는 설정의 항목으로만 제한하며, [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly)는 허용된 도메인에 대해 동일하게 수행합니다.

[샌드박싱 구성](/docs/ko/sandboxing#configure-sandboxing)은 `--setting-sources`로 제외하는 소스를 다룹니다. 세션 중에 목록을 편집하면, Claude Code는 [실행 중인 세션에 변경사항을 적용](/docs/ko/settings#when-edits-take-effect)합니다.

<h4 id="sandbox-path-prefixes">
  샌드박스 경로 접두사
</h4>

`allowWrite`, `denyWrite`, `denyRead`, `allowRead`, [`credentials.files`](#sandbox-credentials-files)의 경로는 접두사로 확인됩니다:

| 접두사            | 의미                                                            | 예시                                                                    |
| :------------- | :------------------------------------------------------------ | :-------------------------------------------------------------------- |
| `/`            | 파일시스템 루트의 절대 경로                                               | `/tmp/build`는 `/tmp/build`로 유지됩니다.                                    |
| `~/`           | 홈 디렉토리 기준 상대 경로                                               | `~/.kube`는 `$HOME/.kube`가 됩니다.                                        |
| `./` 또는 접두사 없음 | 프로젝트 설정의 경우 프로젝트 루트 기준 상대 경로, 사용자 설정의 경우 `~/.claude` 기준 상대 경로 | `.claude/settings.json`의 `./output`은 `<project-root>/output`으로 확인됩니다. |

절대 경로의 `//path` 접두사도 작동합니다. 프로젝트 상대 확인을 기대하면서 단일 슬래시 `/path`를 사용하는 경우, `./path`로 전환하십시오. 이 구문은 절대 경로에 `//path`를 사용하고 프로젝트 상대 경로에 `/path`를 사용하는 [Read 및 Edit 권한 규칙](/docs/ko/permissions#read-and-edit)과 다릅니다. 샌드박스 파일시스템 경로는 표준 규칙을 사용하므로 `/tmp/build`는 절대 경로입니다.

Claude Code는 디렉토리 경로에서 후행 슬래시를 제거하므로, `~/.aws` 및 `~/.aws/`는 동일한 디렉토리와 일치합니다. v2.1.224 이전에는 Claude Code가 후행 슬래시를 샌드박스에 전달했으며, Claude는 여전히 후행 슬래시로 작성된 `denyRead` 또는 `denyWrite` 항목 아래의 경로를 읽거나 쓸 수 있었습니다.

Claude Code는 또한 후행 `/**`를 제거하므로, `~/build/**` 및 `~/build`는 동일한 디렉토리를 다룹니다. `*`와 같은 와일드카드가 작동하는지 여부는 항목이 있는 목록과 플랫폼에 따라 다릅니다:

* **`allowWrite` 및 `denyWrite`**: macOS에서는 와일드카드가 작동합니다. Linux 및 WSL2에서는 샌드박스가 구체적인 경로를 마운트하므로, Claude Code는 후행 `/**`가 제거된 후 `*`, `?` 또는 `[`를 포함하는 항목을 건너뛰며, 해당 항목은 효과가 없습니다. Claude Code는 `Edit` 권한 규칙의 경로를 이러한 목록에 추가하므로, 동일한 제한이 적용되며, `/sandbox`의 **구성** 탭은 와일드카드를 포함하는 `Edit` 및 `Read` 권한 규칙에 대해 경고합니다.
* **`denyRead` 및 `allowRead`**: 모든 플랫폼에서 와일드카드가 작동합니다. Linux 및 WSL2에서는 Claude Code가 읽기 항목을 일치하는 구체적인 경로로 확장하며, 쓰기 목록에 대해서는 이를 수행하지 않습니다.

<h3 id="sandbox-filesystem-allowwrite">
  `sandbox.filesystem.allowWrite`
</h3>

작업 디렉토리, 세션 임시 디렉토리, `--add-dir`, `/add-dir` 또는 `permissions.additionalDirectories`로 추가한 디렉토리 외에 샌드박스된 명령이 쓸 수 있는 경로를 추가합니다. `kubectl` 또는 빌드 도구와 같은 하위 프로세스가 프로젝트 외부에 쓰기를 필요로 할 때 사용하십시오.

* **범위**: [`모든 파일`](#scopes)
* **유형**: [샌드박스 경로 접두사](#sandbox-path-prefixes)를 사용하는 경로 문자열의 배열
* **기본값**: 설정되지 않음. 샌드박스된 명령은 작업 디렉토리, 세션 임시 디렉토리, `--add-dir` 또는 `/add-dir`로 추가한 디렉토리, [`permissions.additionalDirectories`](#permissions-additionaldirectories)의 디렉토리에 쓸 수 있습니다.

다음은 빌드가 `/tmp/build` 아래에 쓸 수 있도록 하고 `kubectl`이 kubeconfig를 업데이트하도록 합니다:

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "allowWrite": ["/tmp/build", "~/.kube"]
    }
  }
}
```

Claude Code는 세션이 로드하는 모든 설정 범위에서 항목을 병합합니다: 사용자, 프로젝트, 로컬, 관리되는 경로가 서로 대체하지 않고 결합되며, Claude Code는 `Edit(...)` 허용 권한 규칙의 경로를 추가합니다. `allowWrite` 항목은 [보호된 경로](/docs/ko/sandboxing#protected-paths)를 해제할 수 없습니다.

<h3 id="sandbox-filesystem-denywrite">
  `sandbox.filesystem.denyWrite`
</h3>

샌드박스된 명령이 특정 경로(다른 방식으로 쓰기 가능한 디렉토리 내의 경로 포함)에 쓰는 것을 차단합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: [샌드박스 경로 접두사](#sandbox-path-prefixes)를 사용하는 경로 문자열의 배열
* **기본값**: 설정되지 않음

다음은 샌드박스된 명령이 시스템 구성을 변경하거나 바이너리를 설치하는 것을 방지합니다:

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyWrite": ["/etc", "/usr/local/bin"]
    }
  }
}
```

Claude Code는 세션이 로드하는 모든 설정 범위에서 항목을 병합하고 `Edit(...)` 거부 권한 규칙의 경로를 추가합니다.

<h3 id="sandbox-filesystem-denyread">
  `sandbox.filesystem.denyRead`
</h3>

샌드박스된 명령이 특정 경로(예: 기본 읽기 정책이 노출할 자격증명 파일)를 읽는 것을 차단합니다. 자격증명 파일을 보호하고 샌드박스 프록시를 통해 사용 가능하게 유지하려면 대신 [`sandbox.credentials`](#sandbox-credentials)를 참조하십시오.

* **범위**: [`모든 파일`](#scopes)
* **유형**: [샌드박스 경로 접두사](#sandbox-path-prefixes)를 사용하는 경로 문자열의 배열
* **기본값**: 설정되지 않음. 샌드박스된 명령은 `~/.aws/credentials`와 같은 자격증명 파일을 포함한 [기본 읽기 액세스](/docs/ko/sandboxing#filesystem-isolation)를 유지합니다.

```json settings.json theme={null}
{
  "sandbox": {
    "filesystem": {
      "denyRead": ["~/.aws/credentials"]
    }
  }
}
```

Claude Code는 세션이 로드하는 모든 설정 범위에서 항목을 병합하고 `Read(...)` 거부 권한 규칙의 경로를 추가합니다. [`filesystem.disabled`](#sandbox-filesystem-disabled)가 `true`이면, Claude Code는 이러한 항목을 적용하지 않습니다.

<h3 id="sandbox-filesystem-allowread">
  `sandbox.filesystem.allowRead`
</h3>

[`denyRead`](#sandbox-filesystem-denyread)가 차단하는 영역 내의 특정 경로에 대해 읽기를 다시 열어서 작업 공간 전용 읽기 액세스를 구축합니다. 정확한 또는 와일드카드 `denyRead` 항목은 더 광범위한 `allowRead` 내에서 차단된 상태로 유지되며, [겹침 표](/docs/ko/sandboxing#configure-sandboxing)에서 보여줍니다. 와일드카드 `denyRead` 항목(예: `~/**/.env`)이 디렉토리와 일치할 때, Claude Code는 해당 내용의 읽기를 차단합니다. v2.1.236 이전의 macOS에서는 Claude Code가 더 광범위한 `allowRead` 항목이 적용되는 곳에서 와일드카드 `denyRead` 항목이 일치하는 경로를 다시 열었으며, 일치하는 디렉토리의 내용을 읽을 수 있게 남겨두었습니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: [샌드박스 경로 접두사](#sandbox-path-prefixes)를 사용하는 경로 문자열의 배열
* **기본값**: 설정되지 않음

다음은 프로젝트 자체를 제외한 홈 디렉토리의 읽기를 차단합니다:

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

Claude Code는 프로젝트 설정에서 `.` 항목을 프로젝트 루트로, 사용자 설정에서 `~/.claude`로 확인합니다. Claude Code는 [`allowManagedReadPathsOnly`](#sandbox-filesystem-allowmanagedreadpathsonly)가 설정되지 않으면 세션이 로드하는 모든 설정 파일에서 항목을 병합합니다.

<h3 id="sandbox-filesystem-allowmanagedreadpathsonly">
  `sandbox.filesystem.allowManagedReadPathsOnly`
</h3>

관리되는 설정에서 오는 [`allowRead`](#sandbox-filesystem-allowread) 항목만 인정하므로, 개발자가 조직이 차단한 경로에 대한 읽기 액세스를 다시 열 수 없습니다. Claude Code는 여전히 세션이 로드하는 모든 설정 범위에서 `denyRead` 항목을 병합합니다.

* **범위**: [`관리됨`](#scopes)
* **유형**: Boolean
  * `true`: Claude Code는 관리되는 설정에서만 `allowRead` 항목을 인정합니다.
  * `false`: `allowRead` 항목은 세션이 로드하는 모든 설정 범위에서 병합됩니다.
* **기본값**: `false`

다음은 홈 디렉토리의 읽기를 차단하고, `~/work`를 다시 열고, 개발자가 다른 것을 다시 열지 못하도록 합니다:

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

[개발자가 정책을 확대하지 못하도록 유지](/docs/ko/sandboxing#keep-developers-from-widening-the-policy)를 참조하십시오.

<h3 id="sandbox-filesystem-disabled">
  `sandbox.filesystem.disabled`
</h3>

네트워크 격리를 유지하면서 파일시스템 격리를 건너뜁니다. 샌드박스된 명령은 호스트 파일시스템에 대한 무제한 읽기 및 쓰기 액세스를 얻으며, 해당 네트워크 송신은 [`network.allowedDomains`](#sandbox-network-alloweddomains)로 제한됩니다. 명령이 쓰는 것이 아니라 연결하는 위치를 제어하기 위해 샌드박스할 때 사용하십시오. Claude Code v2.1.216 이상이 필요합니다.

* **범위**: [`사용자 또는 관리됨`](#scopes). 관리되는 설정이 `sandbox.filesystem`을 전혀 구성하거나 `"mode": "deny"`를 포함하는 `sandbox.credentials.files` 항목을 나열할 때, 관리되는 설정만 이를 설정할 수 있습니다.
* **유형**: Boolean
  * `true`: Claude Code는 파일시스템 격리를 건너뛰고 네트워크 격리를 유지합니다.
  * `false`: 파일시스템 격리는 켜진 상태로 유지됩니다.
* **기본값**: `false`. 파일시스템 격리는 켜진 상태로 유지됩니다.

다음은 파일시스템을 열고 네트워크 송신을 GitHub 및 npm으로 제한합니다:

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

계층이 꺼지면, Claude Code는 `denyRead` 또는 `credentials.files` `deny` 항목을 적용하지 않으며, `credentials.envVars` 항목 및 적용된 `mask` 항목은 계속 작동합니다. [`autoAllowBashIfSandboxed`](#sandbox-autoallowbashifsandboxed)는 여전히 기본값이 `true`이므로, 프롬프트를 계속하려면 `false`로 설정하십시오. [파일시스템 격리 비활성화](/docs/ko/sandboxing#disable-filesystem-isolation)에서 이를 설정할 수 있는 전체 소스 목록과 격리가 꺼졌을 때 변경되는 사항을 참조하십시오. Claude Code v2.1.216 이상이 필요합니다.

<h3 id="sandbox-ignoreviolations">
  `sandbox.ignoreViolations`
</h3>

명령이 시작 시 `/etc/hosts`를 확인하는 도구와 같이 프로브되고 거부될 것으로 예상되는 경로에 대한 샌드박스 위반 보고를 침묵시키므로, 해당 거부가 위반으로 표시되거나 Claude가 보는 것에 나타나지 않습니다. 샌드박스는 여전히 액세스를 차단합니다. 보고만 억제됩니다. 키는 명령과 일치하는 부분 문자열이며, `*`는 모든 명령과 일치하고, 값은 해당 명령에 대해 무시할 위반의 부분 문자열(예: 파일시스템 경로)입니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 명령 부분 문자열을 위반 부분 문자열의 배열에 매핑하는 객체(일반적으로 경로)
* **기본값**: 설정되지 않음. 모든 위반이 보고됩니다.

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

Linux 샌드박스를 권한 없는 Docker 컨테이너 내부에서 실행하며, bubblewrap이 새로운 `/proc`을 마운트할 수 없습니다. 대신 내부 샌드박스는 컨테이너의 기존 `/proc`을 바인드 마운트하며, 이는 새로운 마운트가 숨길 프로세스 정보를 노출합니다. 이는 보안을 감소시킵니다. 외부 컨테이너가 이미 필요한 격리를 제공할 때만 사용하십시오.

* **범위**: [`모든 파일`](#scopes)
* **유형**: Boolean
  * `true`: 내부 샌드박스는 새로운 `/proc`을 마운트하는 대신 컨테이너의 기존 `/proc`을 바인드 마운트합니다.
  * `false`: 샌드박스는 새로운 `/proc`을 마운트하며, 이는 권한 없는 Docker 컨테이너에서 작동하지 않습니다.
* **기본값**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "enableWeakerNestedSandbox": true
  }
}
```

Linux 및 WSL2만 해당합니다. [Bubblewrap이 컨테이너 내부에서 시작되지 않음](/docs/ko/sandboxing#troubleshooting)을 참조하십시오.

<h3 id="sandbox-enableweakernetworkisolation">
  `sandbox.enableWeakerNetworkIsolation`
</h3>

샌드박스된 명령이 macOS에서 시스템 TLS 신뢰 서비스 `com.apple.trustd.agent`에 도달하도록 합니다. `gh`, `gcloud`, `terraform`과 같은 Go 기반 도구는 [`network.httpProxyPort`](#sandbox-network-httpproxyport)를 MITM 프록시 및 사용자 정의 CA와 함께 사용할 때 TLS 인증서를 확인하기 위해 필요합니다. 이는 신뢰 서비스를 통한 잠재적 데이터 유출 경로를 열어서 보안을 감소시킵니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: Boolean
  * `true`: macOS의 샌드박스된 명령은 `com.apple.trustd.agent`에 도달할 수 있습니다.
  * `false`: macOS의 샌드박스된 명령은 시스템 TLS 신뢰 서비스에 도달할 수 없습니다.
* **기본값**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "enableWeakerNetworkIsolation": true
  }
}
```

MITM 프록시를 사용하지 않으면, 대신 실패하는 도구를 [`excludedCommands`](#sandbox-excludedcommands)에 나열하십시오. [Go 기반 CLI가 macOS에서 TLS 검증에 실패함](/docs/ko/sandboxing#troubleshooting)을 참조하십시오.

<h3 id="sandbox-allowappleevents">
  `sandbox.allowAppleEvents`
</h3>

샌드박스된 명령이 macOS에서 Apple Events를 보내도록 합니다. `open`, `osascript`, URL을 브라우저에서 열기 위한 도구가 필요합니다. 없으면 오류 `-600`으로 실패합니다. 이는 코드 실행 격리를 제거합니다: 샌드박스된 명령은 사용자 프롬프트 없이 다른 애플리케이션을 샌드박스 없이 시작할 수 있으며, Terminal과 같은 실행 중인 애플리케이션에 AppleScript 명령을 보낼 수 있습니다. 이는 앱별 macOS 자동화 동의 프롬프트(TCC)를 따릅니다.

* **범위**: [`사용자 또는 관리됨`](#scopes)
* **유형**: Boolean
  * `true`: macOS의 샌드박스된 명령은 Apple Events를 보낼 수 있습니다.
  * `false`: macOS의 샌드박스된 명령은 Apple Events를 보낼 수 없으므로, `open` 및 `osascript`는 오류 `-600`으로 실패합니다.
* **기본값**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "allowAppleEvents": true
  }
}
```

격리를 유지하면서 그러한 도구 중 하나를 실행하려면, 대신 [`excludedCommands`](#sandbox-excludedcommands)에 추가하십시오. [macOS의 Apple Events](/docs/ko/sandboxing#security-limitations)를 참조하십시오.

<h3 id="sandbox-ripgrep">
  `sandbox.ripgrep`
</h3>

Claude Code가 사용하는 ripgrep 바이너리 대신 자신의 ripgrep 바이너리를 샌드박스에 지정합니다. 예를 들어 플랫폼이 다르게 빌드된 `rg`를 필요로 할 때입니다.

* **범위**: [`사용자 또는 관리됨`](#scopes)
* **유형**: ripgrep 바이너리의 경로인 `command`를 포함하는 객체이며, 선택적 `args`(앞에 붙일 인수의 배열)도 포함합니다.
* **기본값**: 설정되지 않음. 샌드박스는 Claude Code와 동일한 ripgrep 바이너리를 사용합니다. 이는 [`USE_BUILTIN_RIPGREP`](/docs/ko/env-vars)을 `0`으로 설정하지 않으면 번들된 바이너리입니다.

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

샌드박스를 `PATH` 외부에 설치된 bubblewrap 바이너리(예: 에어갭 호스트의 벤더된 복사본)에 지정합니다. Claude Code는 시작 종속성 확인 및 각 샌드박스된 명령을 래핑할 때 경로를 사용합니다.

* **범위**: [`관리됨`](#scopes). Claude Code는 관리되는 설정에서만 읽으므로, 사용자, 프로젝트, 로컬 파일이 샌드박스를 다른 바이너리에 지정할 수 없습니다.
* **유형**: 문자열, 절대 경로. Claude Code는 상대 경로를 버리고 `PATH` 조회로 돌아갑니다.
* **기본값**: 설정되지 않음. Claude Code는 `PATH`에서 `bwrap`을 찾습니다.

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "bwrapPath": "/opt/admin/bwrap"
  }
}
```

Linux 및 WSL2만 해당합니다.

<h3 id="sandbox-socatpath">
  `sandbox.socatPath`
</h3>

샌드박스 네트워크 프록시를 `PATH` 외부에 설치된 `socat` 바이너리에 지정합니다.

* **범위**: [`관리됨`](#scopes)
* **유형**: 문자열, 절대 경로. Claude Code는 상대 경로를 버리고 `PATH` 조회로 돌아갑니다.
* **기본값**: 설정되지 않음. Claude Code는 `PATH`에서 `socat`을 찾습니다.

```json managed-settings.json theme={null}
{
  "sandbox": {
    "enabled": true,
    "socatPath": "/opt/admin/socat"
  }
}
```

Linux 및 WSL2만 해당합니다.

<h3 id="sandbox-credentials">
  `sandbox.credentials`
</h3>

[샌드박스된 명령으로부터 보호할](/docs/ko/sandboxing#protect-credentials) 자격증명 파일 및 환경 변수를 선언합니다. 각 항목은 파일 `path` 또는 변수 `name` 및 `mode`를 지정합니다: `deny`는 자격증명을 샌드박스 내부에 숨기고, `mask`는 샌드박스된 명령에 자리 표시자를 표시하면서 [샌드박스 프록시](/docs/ko/sandboxing#mask-credentials)는 아웃바운드 요청에서 실제 값을 대체합니다. Claude Code는 나열한 항목만 보호합니다. 기본 제공 자격증명 거부 목록이 없습니다. Claude Code v2.1.187 이상이 필요합니다.

* **범위**: [`모든 파일`](#scopes). Claude Code는 `mask` 항목, `allowPlaintextInject`, `awsPairs`, `sigv4`를 사용자 설정, 관리되는 설정, `--settings` 플래그에서만 인정합니다.
* **유형**: `files`, `envVars`, `allowPlaintextInject`, `awsPairs`, `sigv4`를 포함하는 객체
* **기본값**: 설정되지 않음. 자격증명이 보호되지 않습니다.

다음은 AWS 자격증명 파일을 숨기고 샌드박스된 명령에서 `GITHUB_TOKEN`을 제거합니다:

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

`deny` 파일 보호는 파일시스템 계층의 일부이므로, [파일시스템 격리를 비활성화](/docs/ko/sandboxing#disable-filesystem-isolation)할 때 적용되지 않습니다. 환경 변수 보호는 여전히 적용됩니다. Claude Code v2.1.187 이상이 필요합니다.

<h4 id="invalid-credential-entries-in-managed-settings">
  관리되는 설정의 잘못된 자격증명 항목
</h4>

관리되는 `sandbox.credentials` 항목이 검증에 실패하면, Claude Code는 가능한 곳에서 자격증명을 보호합니다:

* 유효한 `path` 또는 `name` 및 `mask` 또는 `deny`의 `mode`를 여전히 가진 `files` 또는 `envVars`의 항목(예: `extract` 패턴이 캡처 그룹이 없는 항목)은 경고와 함께 `mode: "deny"`로 저하되므로, 자격증명은 항목을 수정할 때까지 마스크되지 않고 차단된 상태로 유지됩니다. 저하된 `files` 항목은 명시적 `deny` 항목처럼 [`filesystem.disabled`](/docs/ko/sandboxing#disable-filesystem-isolation)를 고정하며, 경고는 관리되는 설정이 파일시스템 격리를 끄면 읽기 블록이 적용되지 않음을 기록합니다.
* 알 수 없는 `mode` 또는 잘못된 `path` 또는 `name`을 가진 항목이 제거됩니다.
* 각 경우가 경고합니다. 항목이 저하되거나 제거되든, 나머지 유효한 항목은 여전히 적용되며, 완전히 잘못된 `credentials` 값은 버려지면서 `sandbox`의 나머지는 여전히 적용됩니다.

v2.1.191 이상에 적용됩니다. v2.1.221 이전에는 모든 잘못된 항목이 제거되었습니다. 필드별 처리를 포함하는 다른 관리되는 키는 [관리되는 설정의 잘못된 항목](/docs/ko/managed-settings#invalid-entries-in-managed-settings)을 참조하십시오.

<h3 id="sandbox-credentials-files">
  `sandbox.credentials.files`
</h3>

자격증명 파일 또는 디렉토리를 샌드박스된 명령으로부터 보호합니다. `"mode": "deny"`를 사용하면, Claude Code는 샌드박스 내부의 경로 읽기를 차단하며, [`sandbox.filesystem.denyRead`](#sandbox-filesystem-denyread)와 동일한 읽기 블록입니다. `"mode": "mask"`를 사용하면, Linux 및 WSL2의 샌드박스된 명령은 파일의 센티널 복사본을 읽고, 샌드박스 프록시는 해당 항목의 `injectHosts`에 대한 아웃바운드 요청에서 실제 값을 대체합니다. macOS에서는 파일이 샌드박스 내부에서 읽을 수 없습니다. Claude Code v2.1.187 이상이 필요하며, `"mode": "mask"`는 v2.1.221 이상이 필요합니다.

* **범위**: [`모든 파일`](#scopes). Claude Code는 프로젝트 `.claude/settings.json` 및 로컬 `.claude/settings.local.json`에서 `mask` 항목을 버립니다.
* **유형**: 각각 `path` 및 `"deny"` 또는 `"mask"`의 `mode`를 포함하는 객체의 배열이며, 선택적 [파일용 마스크 필드](#mask-fields-for-files)도 포함합니다.
* **기본값**: 설정되지 않음. 자격증명 파일이 보호되지 않습니다.

다음은 AWS 자격증명 파일을 숨기고 `gh` 호스트 파일을 마스크하며, `api.github.com`에 대한 요청에서만 실제 값을 대체합니다:

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

경로는 `sandbox.filesystem.*` 설정과 동일한 [접두사](#sandbox-path-prefixes)를 사용하며, Claude Code는 세션이 로드하는 모든 설정 범위에서 배열을 병합합니다. [자격증명 보호](/docs/ko/sandboxing#protect-credentials)는 `--setting-sources`로 제외하는 소스에서 여전히 적용되는 것을 다룹니다. Claude Code v2.1.187 이상이 필요합니다. `mask` 항목은 v2.1.221 이상이 필요합니다.

`mask` 대체는 샌드박스 프록시를 통해서만 실행되므로, [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate) 또는 일반 HTTP 테스트 네트워크의 [`allowPlaintextInject`](#sandbox-credentials-allowplaintextinject)를 설정하십시오. `mask`는 단일 파일에 적용되므로, 각 자격증명 파일을 개별적으로 나열하십시오. Claude Code는 `deny` 항목의 `mask` 필드를 수용하지만 무시합니다. [자격증명 파일 마스크](/docs/ko/sandboxing#mask-credential-files)는 어떤 설정 소스가 인정되는지 및 항목이 `deny`로 돌아가는 시기를 다룹니다.

<span id="sandbox-credentials-files-extract" />

<span id="sandbox-credentials-files-onextractnomatch" />

<span id="sandbox-credentials-files-decode" />

<span id="sandbox-credentials-files-maskclaims" />

<span id="sandbox-credentials-files-maskduplicates" />

<span id="sandbox-credentials-files-injecthosts" />

<h4 id="mask-fields-for-files">
  파일용 마스크 필드
</h4>

`mask` 항목은 이러한 선택적 필드를 수용합니다. `extract` 또는 `decode` 없이, Claude Code는 전체 파일 콘텐츠를 하나의 센티널로 대체합니다. 파일시스템 격리가 켜진 macOS에서는 Claude Code가 `extract` 또는 `decode`가 실행되기 전에 `mask` 항목을 `deny`로 적용합니다. [자격증명 파일 마스크](/docs/ko/sandboxing#mask-credential-files)를 참조하십시오.

| 필드                 | 유형                                                                                        | 수행하는 작업                                                                                                                                                                                                                                                                                                                                                                                                           |
| :----------------- | :---------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `extract`          | 문자열, 최소 하나의 캡처 그룹을 포함하는 정규식                                                               | 각 일치의 그룹 1로 캡처된 텍스트만 마스크하므로, 파일의 나머지는 파싱 가능한 상태로 유지됩니다. `decode`도 설정되면, Claude Code는 각 캡처를 즉시 대체하는 대신 가능한 JWT로 확인합니다. v2.1.221 이상이 필요합니다.                                                                                                                                                                                                                                                                         |
| `onExtractNoMatch` | `"warn"`, `"deny"`, 또는 `"error"`. 기본값 `"warn"`                                            | `extract` 또는 `decode`가 마스크할 것을 찾지 못할 때 발생하는 일. `warn`은 파일을 샌드박스 내부에서 있는 그대로 읽을 수 있게 남기고, `deny`는 읽을 수 없게 만들고, `error`는 구성을 수정할 때까지 샌드박스 설정을 중지합니다. Claude Code는 읽기 블록이 적용되지 않을 때 `deny`를 `error`로 취급합니다. [파일시스템 격리를 비활성화](/docs/ko/sandboxing#disable-filesystem-isolation)하거나 [`sandbox.filesystem.allowRead`](#sandbox-filesystem-allowread) 항목이 경로를 다시 열 때입니다. v2.1.221 이상이 필요합니다. `decode` 경우는 v2.1.224 이상이 필요합니다. |
| `decode`           | 문자열 `"jwt"`                                                                               | 파일에서 JSON Web Tokens(JWT)를 찾고, 기본 제공 패턴 또는 설정된 `extract`를 사용하여, 각 후보를 검증하고, 구조적으로 유효한 가짜 토큰으로 대체하므로, 샌드박스 내부의 토큰을 디코드하는 코드는 계속 작동합니다. 후보가 검증되지 않으면, `onExtractNoMatch`가 결과를 관리합니다. v2.1.224 이상이 필요합니다.                                                                                                                                                                                                            |
| `maskClaims`       | 문자열의 배열, 최소 하나의 클레임 이름. `decode` 필요                                                       | 각 검증된 JWT 내부의 명명된 최상위 페이로드 클레임만 마스크하고 수정된 페이로드 주위에 토큰을 다시 빌드하므로, 다른 클레임은 읽을 수 있게 유지됩니다. 명명된 클레임이 일치하지 않으면, `onExtractNoMatch`가 결과를 관리합니다. v2.1.224 이상이 필요합니다.                                                                                                                                                                                                                                                     |
| `maskDuplicates`   | Boolean, 기본값 `false`                                                                      | 또한 파일의 다른 곳에서 각 마스크된 값의 축자 복사본을 대체합니다. 예를 들어 주석에 붙여넣은 비밀입니다. Claude Code는 원시 부분 문자열을 일치시키므로, 긴 고엔트로피 비밀에 대해 예약하십시오. `extract` 또는 `decode`가 설정될 때만 참조됩니다. v2.1.221 이상이 필요합니다.                                                                                                                                                                                                                                      |
| `injectHosts`      | 문자열의 배열, 각각 [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains)도 허용하는 호스트 | 샌드박스 프록시가 실제 값을 대체하는 호스트를 좁힙니다. 설정되지 않으면, 프록시는 `sandbox.network.allowedDomains`의 모든 호스트에 대한 요청에서 대체합니다. v2.1.221 이상이 필요합니다.                                                                                                                                                                                                                                                                                       |

다음은 `gh` 호스트 파일의 `oauth_token` 값만 마스크하고, 파일의 다른 곳에서 모든 복사본을 대체하고, 패턴이 아무것도 일치하지 않으면 파일을 읽을 수 없게 만들고, `api.github.com`에 대한 요청에서만 실제 토큰을 대체합니다:

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

환경 변수를 샌드박스된 명령으로부터 보호합니다. `"mode": "deny"`를 사용하면, Claude Code는 샌드박스된 명령의 환경에서 변수를 제거합니다. `"mode": "mask"`를 사용하면, 샌드박스된 명령은 세션별 센티널 값을 보고, 샌드박스 프록시는 해당 항목의 `injectHosts`에 대한 아웃바운드 요청에서 실제 값을 대체하므로, `gh` 및 `npm`과 같은 도구는 실제 자격증명을 보유하지 않고도 계속 인증합니다. Claude Code v2.1.187 이상이 필요하며, `"mode": "mask"`는 v2.1.199 이상이 필요합니다.

* **범위**: [`모든 파일`](#scopes). Claude Code는 프로젝트 `.claude/settings.json` 및 로컬 `.claude/settings.local.json`에서 `mask` 항목을 버립니다.
* **유형**: 각각 `name` 및 `"deny"` 또는 `"mask"`의 `mode`를 포함하는 객체의 배열이며, 선택적 [환경 변수용 마스크 필드](#mask-fields-for-environment-variables)도 포함합니다.
* **기본값**: 설정되지 않음. 환경 변수가 보호되지 않습니다.

다음은 샌드박스된 명령에서 `NPM_TOKEN`을 제거하고 `GITHUB_TOKEN`을 마스크하며, `api.github.com`에 대한 요청에서만 실제 값을 대체합니다:

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

`name`은 문자 또는 밑줄로 시작해야 하며 문자, 숫자, 밑줄만 포함해야 합니다. Claude Code는 세션이 로드하는 모든 설정 범위에서 배열을 병합하고, 동일한 변수가 두 모드로 나타날 때 `deny`를 적용합니다. [자격증명 보호](/docs/ko/sandboxing#protect-credentials)는 `--setting-sources`로 제외하는 소스에서 여전히 적용되는 것을 다룹니다. Claude Code v2.1.187 이상이 필요합니다. `mask` 항목은 v2.1.199 이상이 필요합니다.

`mask` 대체는 샌드박스 프록시를 통해서만 실행되므로, [`sandbox.network.tlsTerminate`](#sandbox-network-tlsterminate) 또는 일반 HTTP 테스트 네트워크의 [`allowPlaintextInject`](#sandbox-credentials-allowplaintextinject)를 설정하십시오. [환경 변수 마스크](/docs/ko/sandboxing#mask-environment-variables)를 참조하십시오. Claude Code는 `deny` 항목의 `mask` 필드를 수용하지만 무시합니다.

<span id="sandbox-credentials-envvars-extract" />

<span id="sandbox-credentials-envvars-onextractnomatch" />

<span id="sandbox-credentials-envvars-decode" />

<span id="sandbox-credentials-envvars-maskclaims" />

<span id="sandbox-credentials-envvars-injecthosts" />

<h4 id="mask-fields-for-environment-variables">
  환경 변수용 마스크 필드
</h4>

`mask` 항목은 이러한 선택적 필드를 수용합니다. `extract` 또는 `decode` 없이, Claude Code는 전체 값을 하나의 센티널로 대체합니다. `extract` 및 `decode`는 동일한 항목에서 결합될 수 없습니다.

| 필드                 | 유형                                                                                        | 수행하는 작업                                                                                                                                                                                                                                                                             |
| :----------------- | :---------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `extract`          | 문자열, 최소 하나의 캡처 그룹을 포함하는 정규식                                                               | 각 일치의 그룹 1로 캡처된 텍스트만 마스크합니다. 예를 들어 `DATABASE_URL` 연결 문자열 내부의 비밀번호이므로, 값의 나머지는 파싱 가능한 상태로 유지됩니다. v2.1.224 이상이 필요합니다.                                                                                                                                                                 |
| `onExtractNoMatch` | `"warn"`, `"deny"`, 또는 `"error"`. 기본값 `"warn"`. `decode`를 포함하는 항목에서는 `"warn"`만 수용됨        | `extract`가 아무것도 일치하지 않을 때 발생하는 일. `warn`은 변수를 마스크 없이 전달하고, `deny`는 샌드박스 내부에서 설정 해제하고, `error`는 구성을 수정할 때까지 샌드박스 설정을 중지합니다. v2.1.224 이상이 필요합니다.                                                                                                                                      |
| `decode`           | 문자열 `"jwt"`                                                                               | 전체 값이 JWT인지 검증하고 구조적으로 유효한 가짜 토큰으로 대체하므로, 샌드박스 내부의 토큰을 디코드하는 코드는 계속 작동합니다. 프록시는 송신 시 전체 실제 토큰을 대체합니다. 검증되지 않는 값은 경고와 함께 마스크 없이 전달됩니다. v2.1.224 이상이 필요합니다.                                                                                                                           |
| `maskClaims`       | 문자열의 배열, 최소 하나의 클레임 이름. `decode` 필요                                                       | 디코드된 JWT 내부의 명명된 최상위 페이로드 클레임만 마스크하고 수정된 페이로드 주위에 토큰을 다시 빌드하므로, 다른 클레임은 읽을 수 있게 유지됩니다. 명명된 클레임이 일치하지 않으면, 변수는 경고와 함께 마스크 없이 전달됩니다. v2.1.224 이상이 필요합니다.                                                                                                                              |
| `injectHosts`      | 문자열의 배열, 각각 [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains)도 허용하는 호스트 | 샌드박스 프록시가 실제 값을 대체하는 호스트를 좁힙니다. 설정되지 않으면, 프록시는 `sandbox.network.allowedDomains`의 모든 호스트에 대한 요청에서 대체합니다. IPv6 대상을 괄호로 묶인 형식이 아닌 압축된 주소로 작성합니다. 예를 들어 `"::1"`이지 `[::1]`이 아닙니다. [`injectHosts`의 IPv6 대상](/docs/ko/sandboxing#ipv6-destinations-in-injecthosts)을 참조하십시오. v2.1.199 이상이 필요합니다. |

다음은 `DATABASE_URL` 내부의 비밀번호만 마스크하고, 패턴이 아무것도 일치하지 않으면 변수를 설정 해제하고, `SERVICE_JWT`의 JWT를 마스크하면서 `api_key`를 제외한 모든 클레임을 읽을 수 있게 유지합니다:

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

TLS 종료 HTTPS뿐만 아니라 일반 HTTP 요청에서도 `mask` 대체를 허용합니다. 일반 HTTP에서 업스트림 신원은 검증되지 않으며 자격증명은 평문으로 이동하므로, 신뢰할 수 있는 테스트 네트워크 외부에서는 이를 끕니다. Claude Code v2.1.199 이상이 필요합니다.

* **범위**: [`사용자 또는 관리됨`](#scopes)
* **유형**: Boolean
  * `true`: Claude Code는 TLS 종료 HTTPS뿐만 아니라 일반 HTTP 요청에서도 `mask` 대체를 허용합니다.
  * `false`: Claude Code는 TLS 종료 HTTPS에서만 `mask` 대체를 허용합니다.
* **기본값**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "credentials": {
      "allowPlaintextInject": true
    }
  }
}
```

Claude Code v2.1.199 이상이 필요합니다.

<h3 id="sandbox-credentials-awspairs">
  `sandbox.credentials.awsPairs`
</h3>

마스크된 환경 변수를 그룹화하여 자격증명이 비표준 이름의 변수에 있을 때 [SigV4 재서명](/docs/ko/sandboxing#re-sign-aws-requests)을 위해 하나의 AWS 자격증명을 형성합니다. Claude Code는 전체 값을 마스크할 때 기존 `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, `AWS_SESSION_TOKEN` 트리오를 자동으로 연결하므로, 다른 이름에 대해서만 이 키가 필요합니다. Claude Code v2.1.224 이상이 필요합니다.

* **범위**: [`사용자 또는 관리됨`](#scopes)
* **유형**: 각각 `accessKeyIdVar`, `secretAccessKeyVar`, 선택적 `sessionTokenVar`를 포함하는 객체의 배열이며, `sandbox.credentials.envVars` 항목을 지정합니다.
* **기본값**: 설정되지 않음. 기존 트리오만 쌍을 이룹니다.

다음은 세 개의 사용자 정의 이름 변수를 재서명을 위한 하나의 AWS 자격증명으로 연결합니다:

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

각 명명된 변수는 [`sandbox.credentials.envVars`](#sandbox-credentials-envvars)의 전체 값 `mask` 항목이어야 하며, `extract` 또는 `decode` 없이, 모든 쌍에서 하나의 슬롯만 채울 수 있습니다.

<h3 id="sandbox-credentials-sigv4">
  `sandbox.credentials.sigv4`
</h3>

샌드박스 프록시가 [재서명할 수 없는](/docs/ko/sandboxing#re-sign-aws-requests) AWS 요청 형식으로 수행할 작업을 선택합니다: `streaming`은 aws-chunked 스트리밍 업로드, `presigned`는 사전 서명된 URL, `sigv4a`는 SigV4A 비대칭 서명입니다. 이는 마스크된 쌍의 자리 표시자 액세스 키 ID로 서명된 요청에만 적용됩니다. Claude Code v2.1.224 이상이 필요합니다.

* **범위**: [`사용자 또는 관리됨`](#scopes)
* **유형**: `streaming`, `presigned`, `sigv4a`를 포함하는 객체이며, 각각 다음 중 하나입니다:
  * `"deny"`: 프록시가 요청을 실패시킵니다.
  * `"passthrough"`: 프록시는 마스크된 자리 표시자로 서명된 요청을 전달하므로, 도구는 AWS의 자체 거부를 받습니다.
* **기본값**: 설정되지 않음. 모든 형식이 `"deny"`입니다.

다음은 스트리밍 업로드를 프록시에서 실패시키는 대신 전달합니다:

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

`deny`를 사용하면, 프록시가 요청을 실패시킵니다. `passthrough`를 사용하면, 프록시는 마스크된 자리 표시자에서 계산된 서명으로 요청을 전달하므로, AWS가 거부하고 호출 도구는 프록시 오류 대신 AWS의 자체 응답을 받습니다.

<h3 id="sandbox-network">
  `sandbox.network`
</h3>

샌드박스된 명령이 도달할 수 있는 호스트, 포트, 소켓을 제어합니다. 샌드박스는 아웃바운드 트래픽을 이러한 목록을 적용하는 프록시를 통해 라우팅합니다. [네트워크 격리](/docs/ko/sandboxing#network-isolation)에서 프록시가 결정하는 방식 및 프롬프트하는 시기를 참조하십시오.

* **범위**: [`모든 파일`](#scopes). `strictAllowlist`, `allowManagedDomainsOnly`, `tlsTerminate`는 더 적은 소스에서 읽으며, 해당 항목이 말합니다.
* **유형**: 아래 하위 키를 포함하는 객체
* **기본값**: 설정되지 않음. 도메인이 미리 허용되지 않으며, 샌드박스는 각 새 호스트에 대해 프롬프트합니다.

다음은 GitHub 및 npm을 미리 허용하고, `uploads.github.com`을 차단하고, 명령이 localhost에 바인드되도록 합니다:

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

Claude Code는 배열 하위 키를 설정 범위에서 병합하고 중복을 제거하므로, 프로젝트는 사용자 목록에 도메인을 추가할 수 있습니다. `WebFetch(domain:...)` 허용 및 거부 [권한 규칙](/docs/ko/sandboxing#permission-rules)은 동일한 허용 및 거부 목록을 공급합니다.

<h3 id="sandbox-network-allowunixsockets">
  `sandbox.network.allowUnixSockets`
</h3>

macOS에서 샌드박스된 명령이 연결할 수 있는 Unix 소켓 경로를 나열합니다. Claude Code는 Linux 및 WSL2에서 이 목록을 무시합니다. seccomp 필터가 소켓 경로를 검사할 수 없습니다. 대신 [`allowAllUnixSockets`](#sandbox-network-allowallunixsockets)를 사용하십시오.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 각각 소켓 경로인 문자열의 배열
* **기본값**: 설정되지 않음. macOS 샌드박스는 모든 Unix 소켓을 차단합니다.

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowUnixSockets": ["~/.ssh/agent-socket"]
    }
  }
}
```

소켓 경로는 광범위한 액세스를 부여할 수 있습니다: 예를 들어 `/var/run/docker.sock`을 허용하면, 샌드박스된 명령이 Docker 데몬을 제어할 수 있습니다. [보안 제한](/docs/ko/sandboxing#security-limitations)을 참조하십시오.

<h3 id="sandbox-network-allowallunixsockets">
  `sandbox.network.allowAllUnixSockets`
</h3>

샌드박스된 명령이 모든 Unix 소켓에 연결하도록 합니다. Linux 및 WSL2에서 샌드박스의 [seccomp 필터](/docs/ko/sandboxing#set-up-linux-and-wsl2)는 `socket(AF_UNIX, ...)` 호출을 차단하므로, 이는 Unix 소켓을 허용하는 유일한 방법입니다. 필터가 누락되면, `/sandbox`가 종속성 탭에서 보고하며, 샌드박스는 Unix 소켓 호출을 차단하지 않습니다. [Linux 및 WSL2 설정](/docs/ko/sandboxing#set-up-linux-and-wsl2)에서 필터가 오는 곳을 참조하십시오.

* **범위**: [`모든 파일`](#scopes)
* **유형**: Boolean
  * `true`: 샌드박스된 명령은 모든 Unix 소켓에 연결할 수 있습니다.
  * `false`: 샌드박스는 Unix 소켓 연결을 차단합니다: macOS에서는 `allowUnixSockets`의 경로를 제외하고, Linux 및 WSL2에서는 seccomp 필터가 있을 때 필터를 통해 차단합니다.
* **기본값**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowAllUnixSockets": true
    }
  }
}
```

WSL2에서 `true`는 또한 `cmd.exe` 및 `powershell.exe`와 같은 Windows 바이너리를 시작하는 interop 소켓을 다시 엽니다.

<h3 id="sandbox-network-allowlocalbinding">
  `sandbox.network.allowLocalBinding`
</h3>

샌드박스된 명령이 macOS에서 localhost 포트에 바인드되도록 합니다. 예를 들어 개발 서버를 시작합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: Boolean
  * `true`: 샌드박스된 명령은 macOS에서 localhost 포트에 바인드될 수 있습니다.
  * `false`: macOS의 샌드박스된 명령은 localhost 포트에 바인드될 수 없습니다.
* **기본값**: `false`

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

macOS 샌드박스가 조회할 수 있는 추가 XPC 및 Mach 서비스 이름을 나열합니다. iOS 시뮬레이터 또는 Playwright와 같이 XPC를 통해 통신하는 도구는 여기에 서비스를 나열해야 합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 각각 서비스 이름인 문자열의 배열. 단일 후행 `*`는 접두사와 일치하고, `"*"` 혼자는 모든 서비스와 일치합니다.
* **기본값**: 설정되지 않음

다음은 `com.apple.coresimulator.` 접두사 아래의 모든 서비스를 허용합니다:

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

샌드박스된 명령의 아웃바운드 트래픽에 대해 도메인을 미리 허용하므로, 샌드박스가 프롬프트하지 않습니다. `*.example.com`과 같은 와일드카드는 하위 도메인과 일치하며, 선택적 `:port` 접미사는 항목을 하나의 포트로 제한합니다. 포트 없는 항목은 모든 포트와 일치합니다.

* **범위**: [`모든 파일`](#scopes). [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly)가 설정되면 관리되는 설정만 해당합니다.
* **유형**: 각각 도메인, 와일드카드 패턴, 또는 IP 리터럴이며, 선택적 `:port` 접미사를 포함하는 문자열의 배열
* **기본값**: 설정되지 않음. 샌드박스는 명령이 새 호스트에 처음 도달할 때 프롬프트합니다.

다음은 모든 포트에서 GitHub를 미리 허용하고, 모든 npm 하위 도메인, 포트 443에서만 하나의 API 호스트를 미리 허용합니다:

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "allowedDomains": ["github.com", "*.npmjs.org", "api.example.com:443"]
    }
  }
}
```

IPv6 리터럴을 괄호로 작성하고, 선택적 포트를 포함합니다: `"[::1]"`은 모든 포트를 허용하고 `"[::1]:443"`은 하나의 포트를 허용합니다. 괄호로 묶인 형식은 Claude Code v2.1.229 이상이 필요합니다. [도메인 목록의 IPv6 주소](/docs/ko/sandboxing#ipv6-addresses-in-domain-lists)를 참조하십시오.

<h3 id="sandbox-network-denieddomains">
  `sandbox.network.deniedDomains`
</h3>

샌드박스된 명령의 아웃바운드 트래픽에 대해 도메인을 차단하며, [`allowedDomains`](#sandbox-network-alloweddomains)와 동일한 와일드카드, 포트, IPv6 구문을 사용합니다. 거부된 도메인은 `allowedDomains` 항목도 일치하더라도 차단된 상태로 유지됩니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 각각 도메인, 와일드카드 패턴, 또는 IP 리터럴이며, 선택적 `:port` 접미사를 포함하는 문자열의 배열
* **기본값**: 설정되지 않음

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "deniedDomains": ["sensitive.cloud.example.com"]
    }
  }
}
```

Claude Code는 `allowManagedDomainsOnly`가 설정되어도 세션이 로드하는 모든 설정 소스에서 이 목록을 병합하므로, 개발자는 항상 거부 목록을 강화할 수 있습니다. IPv6 리터럴은 [도메인 목록의 IPv6 주소](/docs/ko/sandboxing#ipv6-addresses-in-domain-lists)를 참조하십시오.

정규화된 도메인 이름을 표시하는 후행 점으로 작성된 항목(예: `example.com.`)은 `example.com`과 동일한 연결을 차단합니다.

<h3 id="sandbox-network-strictallowlist">
  `sandbox.network.strictAllowlist`
</h3>

승인을 요청하는 대신 허용 목록 외부의 호스트에 대한 샌드박스된 명령 액세스를 거부합니다. 허용 목록은 [`allowedDomains`](#sandbox-network-alloweddomains) 더하기 `WebFetch(domain:...)` 허용 규칙의 도메인이거나, [`allowManagedDomainsOnly`](#sandbox-network-allowmanageddomainsonly)가 설정되면 관리되는 설정 항목만입니다. Claude Code v2.1.219 이상이 필요합니다.

* **범위**: [`사용자 또는 관리됨`](#scopes). 저장소는 이를 켜거나 끌 수 없습니다.
* **유형**: Boolean
  * `true`: Claude Code는 허용 목록 외부의 호스트에 대한 샌드박스된 명령 액세스를 거부합니다.
  * `false`: 다른 신뢰할 수 있는 설정 파일이 `true`를 설정하지 않으면, Claude Code는 허용 목록 외부의 호스트를 권한 모드 대신 거부로 결정합니다: 자동 모드에서 [명령별 허용된 도메인](/docs/ko/sandboxing#per-command-allowed-domains-in-auto-mode)에 대해 호스트를 확인하고, `dontAsk` 모드에서 거부하고, `bypassPermissions` 모드 및 우회가 가능할 때 계획 모드에서 허용하고, 그렇지 않으면 묻습니다.
* **기본값**: `false`

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "strictAllowlist": true
    }
  }
}
```

Claude Code는 샌드박스된 명령에만 이를 적용합니다. `WebFetch`와 같은 프로세스 내 도구는 여전히 [권한 규칙](/docs/ko/sandboxing#permission-rules)을 따릅니다. 인정된 소스 중 하나가 `true`로 설정하면, 이는 켜진 상태로 유지됩니다. [네트워크 격리](/docs/ko/sandboxing#network-isolation)를 참조하십시오. Claude Code v2.1.219 이상이 필요합니다.

<h3 id="sandbox-network-allowmanageddomainsonly">
  `sandbox.network.allowManagedDomainsOnly`
</h3>

네트워크 허용 목록을 관리되는 설정이 정의하는 것으로 잠급니다. Claude Code는 관리되는 설정에서만 `allowedDomains` 및 `WebFetch(domain:...)` 허용 규칙을 인정하고, 사용자, 프로젝트, 로컬, `--settings` 설정의 도메인을 무시하고, 허용되지 않은 도메인을 프롬프트 대신 자동으로 차단합니다.

* **범위**: [`관리됨`](#scopes)
* **유형**: Boolean
  * `true`: Claude Code는 관리되는 설정에서만 `allowedDomains` 및 `WebFetch(domain:...)` 허용 규칙을 인정하고 허용되지 않은 도메인을 차단합니다.
  * `false`: 사용자, 프로젝트, 로컬, `--settings` 설정의 도메인이 허용 목록에 병합됩니다.
* **기본값**: `false`

다음은 허용 목록을 GitHub 및 npm으로 잠그고 개발자가 추가하는 도메인을 무시합니다:

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

거부된 도메인은 여전히 세션이 로드하는 모든 소스에서 병합됩니다. [개발자가 정책을 확대하지 못하도록 유지](/docs/ko/sandboxing#keep-developers-from-widening-the-policy)를 참조하십시오.

<h3 id="sandbox-network-httpproxyport">
  `sandbox.network.httpProxyPort`
</h3>

Claude Code가 실행하는 프록시 대신 자신의 HTTP 프록시를 샌드박스에 지정합니다. 조직은 HTTPS 트래픽을 검사하고, 자신의 필터링 규칙을 적용하거나, 모든 요청을 기록하기 위해 이를 수행합니다. 설정되지 않으면, Claude Code는 HTTP 트래픽에 대해 자신의 프록시를 시작합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 숫자, 로컬 TCP 포트
* **기본값**: 설정되지 않음. Claude Code는 자신의 프록시를 실행합니다.

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "httpProxyPort": 8080
    }
  }
}
```

프록시가 SOCKS 트래픽도 전달해야 하면 [`socksProxyPort`](#sandbox-network-socksproxyport)도 설정하십시오. 둘 중 하나만 설정되면, Claude Code는 여전히 다른 프로토콜에 대해 자신의 프록시를 실행합니다. [사용자 정의 프록시 구성](/docs/ko/sandboxing#custom-proxy-configuration)을 참조하십시오.

<h3 id="sandbox-network-socksproxyport">
  `sandbox.network.socksProxyPort`
</h3>

Claude Code가 실행하는 프록시 대신 자신의 SOCKS5 프록시를 샌드박스에 지정합니다. 설정되지 않으면, Claude Code는 SOCKS 트래픽에 대해 자신의 프록시를 시작합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 숫자, 로컬 TCP 포트
* **기본값**: 설정되지 않음. Claude Code는 자신의 프록시를 실행합니다.

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "socksProxyPort": 8081
    }
  }
}
```

[사용자 정의 프록시 구성](/docs/ko/sandboxing#custom-proxy-configuration)을 참조하십시오.

<h3 id="sandbox-network-tlsterminate">
  `sandbox.network.tlsTerminate`
</h3>

샌드박스 프록시가 TLS를 종료하도록 하여 HTTPS 요청의 콘텐츠를 읽을 수 있습니다. 이는 실험적이며, `mask` [자격증명 대체](/docs/ko/sandboxing#mask-credentials)는 이를 필요로 합니다. 세션에 대한 임시 인증 기관을 생성하려면 `{}`를 설정하거나, 자신의 것을 사용하려면 `caCertPath` 및 `caKeyPath`를 설정하십시오.

* **범위**: [`사용자 또는 관리됨`](#scopes). 저장소는 이를 켜거나 인증 기관을 제공할 수 없습니다.
* **유형**: 선택적 `caCertPath` 및 `caKeyPath` 문자열(각각 파일 경로)을 포함하는 객체
* **기본값**: 설정되지 않음. 프록시는 TLS를 종료하거나 검사하지 않습니다.

```json settings.json theme={null}
{
  "sandbox": {
    "network": {
      "tlsTerminate": {}
    }
  }
}
```

둘 이상의 인정된 소스가 이를 설정하면, Claude Code는 가장 높은 우선순위 소스의 값을 사용합니다: 관리되는 설정, 그 다음 `--settings` 플래그, 그 다음 사용자 설정. Claude Code v2.1.199 이상이 필요합니다.

<span id="context-and-memory" />

<h2 id="memory-and-context">
  메모리 및 컨텍스트
</h2>

Claude Code가 컨텍스트에 로드하는 내용, 압축 방식, 메모리 및 계획 저장 위치를 제어합니다. [컨텍스트 관리](/docs/ko/context-window) 및 [메모리](/docs/ko/memory)를 참조하세요.

<h3 id="autocompactenabled">
  `autoCompactEnabled`
</h3>

컨텍스트가 한계에 가까워질 때 Claude Code가 [대화를 자동으로 압축](/docs/ko/context-window#when-your-context-fills-up)하도록 합니다. `/config`에 **자동 압축**으로 표시되며, 여기서 토글하면 이 키가 사용자 설정에 기록됩니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: 컨텍스트가 한계에 가까워질 때 Claude Code가 대화를 자동으로 압축합니다
  * `false`: Claude Code가 자동으로 압축하지 않습니다
* **기본값**: `true`
* **세션별 재정의**: [`DISABLE_AUTO_COMPACT`](/docs/ko/env-vars)는 한 세션 동안 자동 압축을 끕니다. 둘 중 하나가 끄면 다른 하나는 다시 켤 수 없습니다

```json settings.json theme={null}
{
  "autoCompactEnabled": false
}
```

자동 압축이 꺼져 있는 동안에도 수동 `/compact` 명령은 계속 작동합니다.

<h3 id="autocompactwindow">
  `autoCompactWindow`
</h3>

Claude Code가 [자동으로 압축](/docs/ko/context-window#when-your-context-fills-up)하기 전에 컨텍스트 윈도우가 얼마나 찬지 설정합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 토큰 수, `100000`에서 `1000000` 사이. Claude Code는 값을 모델의 컨텍스트 윈도우로 제한합니다. [모델 개요](https://platform.claude.com/docs/en/about-claude/models/overview)에 각 모델의 윈도우가 나열되어 있습니다
* **기본값**: 설정되지 않음. Claude Code가 모델에 맞게 조정된 윈도우를 선택합니다
* **세션별 재정의**: [`--autocompact`](/docs/ko/cli-reference#cli-flags)는 한 세션 동안 이 키보다 우선하며, [`CLAUDE_CODE_AUTO_COMPACT_WINDOW`](/docs/ko/env-vars)는 둘 다보다 우선합니다

```json settings.json theme={null}
{
  "autoCompactWindow": 500000
}
```

[`/autocompact`](/docs/ko/commands#all-commands) 명령으로 설정하면 이 키가 사용자 설정에 기록됩니다. [자동 압축 윈도우 설정](/docs/ko/model-config#set-the-auto-compact-window)에서 명령, 플래그, 변수 및 설정이 어떻게 상호작용하는지 다룹니다.

<h3 id="automemorydirectory">
  `autoMemoryDirectory`
</h3>

[자동 메모리](/docs/ko/memory#storage-location)를 프로젝트별 기본값 대신 선택한 디렉토리에 저장합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열, 절대 경로 또는 `~/` 접두사가 있는 디렉토리 경로
* **기본값**: 설정되지 않음. Claude Code는 `~/.claude/projects/<project>/memory/`를 사용합니다

```json settings.json theme={null}
{
  "autoMemoryDirectory": "~/my-memory-dir"
}
```

프로젝트 또는 로컬 설정에서 Claude Code는 [훅과 동일한 워크스페이스 신뢰 규칙](/docs/ko/permissions#what-runs-before-you-trust-a-folder)에 따라 이 키를 준수합니다. 복제된 저장소가 이러한 파일을 제공할 수 있기 때문입니다.

<h3 id="automemoryenabled">
  `autoMemoryEnabled`
</h3>

[자동 메모리](/docs/ko/memory#enable-or-disable-auto-memory)를 켜거나 끕니다. `false`일 때 Claude는 자동 메모리 디렉토리에서 읽거나 쓰지 않습니다. 세션 중에 `/memory`로도 토글할 수 있으며, 이는 이 키를 사용자 설정에 기록합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: 설정되지 않은 것과 동일합니다. `--bare`, 안전 모드 또는 `CLAUDE_CODE_DISABLE_AUTO_MEMORY`와 같이 이 키보다 우선하는 것이 세션을 끄지 않는 한 자동 메모리는 켜진 상태로 유지됩니다
  * `false`: Claude는 자동 메모리 디렉토리에서 읽거나 쓰지 않습니다
* **기본값**: `true`
* **세션별 재정의**: [`CLAUDE_CODE_DISABLE_AUTO_MEMORY`](/docs/ko/env-vars)는 한 세션 동안 이 키보다 우선하며, 어느 방향이든 적용됩니다

```json settings.json theme={null}
{
  "autoMemoryEnabled": false
}
```

<h3 id="bashoutputmaxchars">
  `bashOutputMaxChars`
</h3>

성공한 Bash 또는 PowerShell 명령의 [출력 중 Claude가 인라인으로 받는 문자 수](/docs/ko/tools-reference#output-limits)를 설정합니다. 출력이 한계를 초과하면 Claude Code는 이를 파일에 저장하고 Claude는 짧은 미리보기와 파일의 경로를 받습니다. 자세한 빌드 또는 전체 테스트 스위트 로그와 같은 명령 출력이 기본값을 자주 초과하고 Claude가 파일을 열지 않고 읽기를 원할 때 한계를 높입니다. Claude Code v2.1.261 이상이 필요합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자 수, 양의 정수. Claude Code는 값을 `4000`에서 `128000` 범위로 제한합니다
* **기본값**: 설정되지 않음. Claude는 최대 30,000자를 인라인으로 받습니다

```json settings.json theme={null}
{
  "bashOutputMaxChars": 100000
}
```

이 키를 설정하면 Claude Code는 [`BASH_MAX_OUTPUT_LENGTH`](/docs/ko/env-vars) 환경 변수를 무시합니다.

<h3 id="claudemd">
  `claudeMd`
</h3>

CLAUDE.md 스타일 지침을 별도 파일을 배포하지 않고 조직 관리 메모리로 주입합니다. Claude Code는 텍스트를 사용자 및 프로젝트 CLAUDE.md 파일보다 먼저 관리 메모리 항목으로 로드합니다.

* **범위**: [`관리됨`](#scopes)
* **유형**: 문자열, CLAUDE.md 파일의 텍스트. 파일처럼 작성하되 Markdown을 포함하고 줄 바꿈을 `\n`으로 표기합니다
* **기본값**: 설정되지 않음

이 예제는 두 가지 규칙을 짧은 Markdown 목록으로 배포합니다:

```json managed-settings.json theme={null}
{
  "claudeMd": "# Engineering rules\n\n- Always run make lint before committing.\n- Never push directly to main."
}
```

[조직 전체 CLAUDE.md 배포](/docs/ko/memory#deploy-organization-wide-claude-md)를 참조하세요.

<h3 id="claudemdexcludes">
  `claudeMdExcludes`
</h3>

Claude Code가 [메모리](/docs/ko/memory#exclude-specific-claude-md-files)를 로드할 때 특정 `CLAUDE.md` 파일을 건너뜁니다. 큰 모노레포에서 이를 사용하여 작업과 관련이 없는 다른 팀의 CLAUDE.md 파일을 건너뜁니다. [관련 없는 CLAUDE.md 파일 제외](/docs/ko/large-codebases#exclude-irrelevant-claude-md-files)는 대규모 코드베이스 가이드에서 해당 경우를 다룹니다. 패턴은 절대 파일 경로와 일치합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열 배열, 각각 glob 패턴 또는 절대 경로
* **기본값**: 설정되지 않음. Claude Code는 찾은 모든 CLAUDE.md를 로드합니다

```json settings.json theme={null}
{
  "claudeMdExcludes": ["**/vendor/**/CLAUDE.md"]
}
```

제외는 사용자, 프로젝트 및 로컬 메모리 파일에만 적용됩니다. 관리 정책 CLAUDE.md 파일은 제외할 수 없습니다.

<span id="environment-variables" />

<h3 id="env">
  `env`
</h3>

모든 세션 및 Claude Code가 시작하는 서브프로세스에 대한 환경 변수를 설정합니다. [환경 변수 참조](/docs/ko/env-vars)의 모든 변수가 여기에 올 수 있으며, 이것이 모든 세션에 변수를 적용하거나 팀에 배포하는 방법입니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 변수 이름을 문자열 값으로 매핑하는 객체
* **기본값**: 설정되지 않음

이 예제는 자동 압축을 끄고 API 요청을 프록시를 통해 라우팅합니다:

```json settings.json theme={null}
{
  "env": {
    "DISABLE_AUTO_COMPACT": "1",
    "ANTHROPIC_BASE_URL": "https://proxy.example.com"
  }
}
```

<h4 id="how-env-values-interact-with-your-shell">
  `env` 값이 셸과 상호작용하는 방식
</h4>

* 여기의 값은 셸에서 내보낸 동일한 변수를 덮어쓰며, 둘 이상의 설정 파일이 변수를 설정할 때 [가장 높은 우선순위](/docs/ko/settings#settings-precedence)가 적용됩니다.
* 셸 내보내기를 취소하려면 변수를 `""`로 설정합니다. Claude Code는 공 값을 공급자 선택에 대해 설정되지 않은 것으로 취급하며, 서브프로세스는 공 값을 상속합니다.
* `NO_COLOR` 및 `FORCE_COLOR`는 여기서 설정하면 서브프로세스에만 도달합니다. Claude Code 자체 인터페이스 색상을 변경하려면 `claude`를 시작하기 전에 셸에서 설정합니다.
* 여기의 값은 설정 파일의 일반 텍스트이며 Claude Code가 시작하는 모든 서브프로세스에 도달합니다. 회전하는 OTLP 베어러 토큰의 경우 [`otelHeadersHelper`](#otelheadershelper)를 사용합니다. API 자격 증명의 경우 [`apiKeyHelper`](#apikeyhelper)를 사용합니다.

<h4 id="when-claude-code-applies-env-values">
  Claude Code가 `env` 값을 적용하는 시기
</h4>

* 사용자 설정, `--settings` 및 관리 설정에서: 시작 시 및 저장된 변경이 병합된 `env`를 변경할 때 실행 중인 세션에서.
* 프로젝트 및 로컬 설정에서: 워크스페이스를 신뢰한 후 또는 신뢰 대화를 표시하지 않는 `-p` 모드에서 시작 시, 그리고 저장된 변경이 병합된 `env`를 변경할 때.
* Claude Code가 모델 선택, 타임아웃 및 한계, 기능 토글 및 원격 분석 설정과 같은 안전한 것으로 분류하는 변수: 모든 설정 파일에서 시작 시, [프로젝트 및 로컬 설정이 `env`에서 설정할 수 없는 변수](#variables-claude-code-ignores-in-env) 제외.
* v2.1.246 이상에서 [`/cd`로 세션을 이동](/docs/ko/permissions#move-the-session-to-another-directory)한 후: 새 디렉토리의 프로젝트 및 로컬 `env` 값, 이전 디렉토리의 값 위에.

<h4 id="variables-claude-code-ignores-in-env">
  Claude Code가 `env`에서 무시하는 변수
</h4>

* 프로젝트 및 로컬 설정은 체크아웃된 저장소가 제어하지 않아야 하는 변수를 설정할 수 없습니다. 대신 셸, 사용자 설정 또는 관리 설정에서 설정합니다. Claude Code는 각각을 삭제하고 `claude --debug`로 볼 수 있는 경고를 기록합니다. 여기에는 다음이 포함됩니다:

  * Claude Code가 자신의 파일을 저장하거나 쓰는 위치를 선택하는 변수: `CLAUDE_CONFIG_DIR`, `CLAUDE_CODE_TMPDIR` 및 `HOME`, `TMPDIR`, `TMP`, `TEMP` 및 `XDG_*` 계열과 같은 운영 체제 디렉토리 변수.
  * 세션 콘텐츠를 내보내는 변수: [`OTEL_LOG_RAW_API_BODIES`](/docs/ko/env-vars#variables) 및 자세한 베타 추적 쌍 `ENABLE_BETA_TRACING_DETAILED` 및 `BETA_TRACING_ENDPOINT`.
  * Claude Code가 시작하거나 동기화하는 방식을 변경하는 변수, 예: `CLAUDE_CODE_PROCESS_WRAPPER`, `CLAUDE_CODE_SYNC_SKILLS`, `CLAUDE_CODE_SYNC_PLUGINS`, `CLAUDE_CODE_PLUGIN_CACHE_DIR` 및 `CLAUDE_CODE_PLUGIN_SEED_DIR`.

  v2.1.251 이전에는 프로젝트 및 로컬 설정이 `HOME`, `XDG_CONFIG_HOME` 및 Claude Code가 시작하거나 동기화하는 방식을 변경하는 변수를 제외한 이 목록의 모든 변수를 설정할 수 있었습니다.
* Claude Code의 호스팅 환경이 소유한 `CLAUDE_CODE_REMOTE` 및 `CLAUDE_CODE_ACCOUNT_UUID`와 같은 ID 변수는 모든 파일에서 무시됩니다.
* Claude Code가 자체 내보내는 [`CLAUDE_CODE_MESSAGING_SOCKET` 및 `CLAUDE_CODE_MESSAGING_TOKEN`](/docs/ko/env-vars#variables)은 모든 파일에서 무시됩니다. 소켓 변수를 무시하려면 Claude Code v2.1.224 이상이 필요하며, 토큰을 무시하려면 v2.1.228 이상이 필요합니다.
* Claude Code가 시작 환경에서만 읽는 [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/ko/sessions#name-the-project-directory-yourself)은 모든 파일에서 무시됩니다. v2.1.234 이상이 필요합니다.
* Claude Code가 시작 환경에서만 읽는 [`CLAUDE_CODE_RESTRICTED`](/docs/ko/env-vars#variables)은 모든 파일에서 무시됩니다.

<h3 id="filecheckpointingenabled">
  `fileCheckpointingEnabled`
</h3>

각 편집 전에 Claude Code가 파일을 스냅샷하여 [`/rewind`](/docs/ko/checkpointing)가 파일을 복원할 수 있도록 합니다. `/config`에 \*\*코드 되감기(체크포인트)\*\*로 표시되며, 여기서 토글하면 이 키가 사용자 설정에 기록됩니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: Claude Code가 각 편집 전에 파일을 스냅샷하여 `/rewind`가 파일을 복원할 수 있습니다
  * `false`: Claude Code가 파일을 스냅샷하지 않으므로 `/rewind`가 파일을 복원할 수 없습니다
* **기본값**: `true`
* **세션별 재정의**: [`CLAUDE_CODE_DISABLE_FILE_CHECKPOINTING`](/docs/ko/env-vars)는 한 세션 동안 체크포인팅을 끕니다. 둘 중 하나가 끄면 다른 하나는 다시 켤 수 없습니다

```json settings.json theme={null}
{
  "fileCheckpointingEnabled": false
}
```

`-p` 실행 또는 Agent SDK 세션에서 Claude Code는 이 키를 무시합니다. SDK는 `enableFileCheckpointing` 옵션으로 체크포인팅을 켜고, 베어 `-p` 실행은 `CLAUDE_CODE_ENABLE_SDK_FILE_CHECKPOINTING=true`가 필요합니다. [Agent SDK의 파일 체크포인팅](/docs/ko/agent-sdk/file-checkpointing)을 참조하세요.

<h3 id="plansdirectory">
  `plansDirectory`
</h3>

Claude Code가 [계획 모드](/docs/ko/permission-modes#analyze-before-you-edit-with-plan-mode)에서 작성하는 계획 파일을 저장할 위치를 선택합니다. Claude Code는 경로를 프로젝트 루트에 상대적으로 해석하고 경로가 외부로 해석될 때 기본값을 유지합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열, 프로젝트 루트에 상대적인 경로
* **기본값**: 설정되지 않음. Claude Code는 `~/.claude/plans`를 사용합니다

```json settings.json theme={null}
{
  "plansDirectory": "./plans"
}
```

<h3 id="skilllistingbudgetfraction">
  `skillListingBudgetFraction`
</h3>

각 턴마다 Claude는 설명과 함께 [스킬 목록](/docs/ko/skills#skill-descriptions-are-cut-short)을 보며, Claude Code는 해당 목록을 컨텍스트 윈도우의 일부로 제한합니다. 목록이 한계를 초과하면 Claude Code는 모든 스킬의 이름을 유지하지만 가장 적게 사용된 스킬의 설명을 삭제하여 Claude가 여전히 해당 스킬을 호출할 수 있지만 자체적으로 선택할 가능성이 낮습니다. 더 많은 설명을 표시하려면 이 키를 높이되 턴당 더 많은 컨텍스트를 사용합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 숫자, `0`보다 크고 최대 `1`인 분수
* **기본값**: `0.01`, 컨텍스트 윈도우의 1%를 예약합니다

```json settings.json theme={null}
{
  "skillListingBudgetFraction": 0.02
}
```

목록이 사용하는 컨텍스트 양과 어떤 스킬이 가장 많이 기여하는지 보려면 `/doctor`를 실행합니다.

<h3 id="skilllistingmaxdescchars">
  `skillListingMaxDescChars`
</h3>

각 턴마다 Claude는 각 스킬의 `description` 및 `when_to_use` 텍스트를 보여주는 [스킬 목록](/docs/ko/skills#skill-descriptions-are-cut-short)을 봅니다. 이 키는 Claude Code가 스킬당 해당 텍스트의 몇 문자를 표시하는지 제한합니다. 더 긴 텍스트는 한계에서 잘립니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자 수, 양의 정수
* **기본값**: `1536`

```json settings.json theme={null}
{
  "skillListingMaxDescChars": 2048
}
```

긴 설명을 유지하려면 높이되 턴당 더 많은 컨텍스트를 사용합니다. [`skillListingBudgetFraction`](#skilllistingbudgetfraction) 아래에 더 많은 스킬을 맞추려면 낮춥니다.

<h3 id="taskoutputmaxchars">
  `taskOutputMaxChars`
</h3>

<Warning>
  v2.1.277에서 제거되었으며, 이를 크기 조정한 `TaskOutput` 도구와 함께 제거되었습니다. 현재 버전에서 설정해도 효과가 없습니다. Claude는 대신 `Read`를 사용하여 백그라운드 작업의 [출력 파일](/docs/ko/tools-reference#background-commands)을 읽습니다.
</Warning>

v2.1.276까지 이 키를 [백그라운드 작업](/docs/ko/tools-reference#background-commands)의 출력 중 Claude가 `TaskOutput` 도구로 작업을 읽을 때 인라인으로 받는 문자 수로 설정했습니다.

<h2 id="interface-and-terminal">
  인터페이스 및 터미널
</h2>

Claude Code가 터미널에서 어떻게 보이고 동작하는지 변경합니다: 테마, 편집기 모드, 상태 줄, 스피너, 세션 내 알림, 접근성. [터미널 구성](/docs/ko/terminal-config)을 참조하세요.

<h3 id="askuserquestiontimeout">
  `askUserQuestionTimeout`
</h3>

답변되지 않은 [`AskUserQuestion`](/docs/ko/tools-reference) 대화 상자가 유휴 시간 후 자동으로 계속되도록 하여 이미 선택한 옵션을 제출합니다. 자리를 비우고 Claude가 당신 없이 계속 진행하기를 원할 때 설정하세요. 기본값으로는 질문이 답변될 때까지 대기합니다. Claude Code v2.1.200 이상이 필요합니다.

* **범위**: [`사용자 또는 관리됨`](#scopes)
* **유형**: 문자열, `"60s"`, `"5m"`, `"10m"` 또는 `"never"` 중 하나
* **기본값**: `"never"`
* **세션별 재정의**: [`CLAUDE_AFK_TIMEOUT_MS`](/docs/ko/env-vars)가 이 키보다 한 세션에 대해 우선합니다

```json settings.json theme={null}
{
  "askUserQuestionTimeout": "5m"
}
```

`/config`에 **질문 자동 계속 시간 초과**로 표시되며, 이 키를 사용자 설정에 씁니다. Claude Code는 관리 설정이나 `--settings` 플래그가 키를 설정하는 동안 행을 숨깁니다. Claude Code v2.1.200 이상이 필요합니다.

<h3 id="autocontinueatusagelimit">
  `autoContinueAtUsageLimit`
</h3>

claude.ai 사용 제한이 세션을 중지한 후, 열린 세션에서 대기하고 재설정 후 작업을 자동으로 계속합니다. [자동 계속 끄기](/docs/ko/interactive-mode#turn-automatic-continue-off)를 참조하세요. Claude Code v2.1.234 이상이 필요합니다.

* **범위**: [`사용자 또는 관리됨`](#scopes). 사용자 설정, `--settings` 및 관리 설정에서만 읽습니다. 이들 중 어느 것도 키를 설정하지 않으면, 키를 설정하는 프로젝트 또는 로컬 설정 파일이 무시되지 않고 기능을 끕니다.
* **유형**: 부울
  * `true`: claude.ai 사용 제한이 세션을 중지한 후, Claude Code는 열린 세션에서 대기하고 재설정 후 작업을 자동으로 계속합니다
  * `false`: Claude Code는 자체적으로 대기를 시작하지 않습니다. 사용 제한 옵션 메뉴에서 [직접 대기를 시작](/docs/ko/interactive-mode#start-a-wait-yourself)할 수 있습니다
* **기본값**: `true`

```json settings.json theme={null}
{
  "autoContinueAtUsageLimit": false
}
```

`/config`에 **사용 제한에서 자동으로 계속**으로 표시되며, 이 키를 사용자 설정에 씁니다. Claude Code는 관리 설정이나 `--settings` 플래그가 키를 설정하는 동안 행을 숨깁니다.

<h3 id="autoscrollenabled">
  `autoScrollEnabled`
</h3>

[전체 화면 렌더링](/docs/ko/fullscreen)에서 새 출력을 대화의 맨 아래로 따릅니다. 이를 끄면 Claude가 계속 작업하는 동안 스크롤한 위치에 머물러 있습니다. 권한 프롬프트는 여전히 보기로 스크롤됩니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: 대화가 새 출력을 맨 아래로 따릅니다
  * `false`: Claude가 계속 작업하는 동안 스크롤한 위치에 머물러 있습니다. 권한 프롬프트는 여전히 기록 아래에 나타납니다
* **기본값**: `true`

```json settings.json theme={null}
{
  "autoScrollEnabled": false
}
```

`/config`에 **자동 스크롤**로 표시되며, 전체 화면 렌더링이 켜져 있을 때 이 키를 사용자 설정에 씁니다.

<h3 id="axscreenreader">
  `axScreenReader`
</h3>

화면 판독기 친화적 출력을 렌더링합니다: 장식 테두리나 애니메이션 없는 평면 텍스트. 화면 판독기 모드는 클래식 렌더러를 사용하므로 활성화되는 동안 `tui` 설정은 효과가 없습니다. 연결된 [백그라운드 세션](/docs/ko/agent-view)은 여전히 전체 화면으로 렌더링됩니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: Claude Code는 클래식 렌더러를 사용하여 장식 테두리나 애니메이션 없는 평면 텍스트를 렌더링합니다
  * `false`: Claude Code는 정상적으로 렌더링합니다
* **기본값**: 설정되지 않음, 따라서 화면 판독기 모드는 꺼져 있습니다
* **세션별 재정의**: [`--ax-screen-reader`](/docs/ko/cli-reference#cli-flags)가 [`CLAUDE_AX_SCREEN_READER`](/docs/ko/env-vars)보다 우선하며, 둘 다 한 세션에 대해 이 키보다 우선합니다

```json settings.json theme={null}
{
  "axScreenReader": true
}
```

<h3 id="basheditdiffenabled">
  `bashEditDiffEnabled`
</h3>

Claude Code가 Bash 명령이 Git 저장소에서 변경하는 파일을 기록할지 여부를 선택합니다. 기록하면 터미널에서 명령 후 diff를 보고, [PostToolUse Bash hooks](/docs/ko/hooks#bash)는 변경된 파일 목록을 받습니다.

나열된 파일이 항상 명령이 변경한 파일인 것은 아닙니다. 명령이 실행되는 동안 다른 프로그램이나 다른 Bash 호출이 만든 변경 사항도 거기에 나타날 수 있습니다.

모든 권한 모드에서 기록하려면 키를 `true`로 설정합니다. Claude Code v2.1.269 이상이 필요합니다.

* **범위**: [`사용자 또는 관리됨`](#scopes). `true`는 사용자 설정, `--settings`로 전달된 JSON 또는 [관리 설정](/docs/ko/managed-settings)에서만 계산되므로, 저장소의 `.claude/settings.json` 또는 `.claude/settings.local.json`의 `true`는 기록을 켤 수 없습니다. 저장소 파일의 `false`는 [더 높은 우선순위](/docs/ko/settings#settings-precedence) 파일이 `true`를 설정하지 않으면 여전히 끕니다.
* **유형**: 부울
* **기본값**: 설정되지 않음, 따라서 Claude Code는 자동 모드 및 `bypassPermissions` 모드에서 Claude가 Bash를 통해 파일을 편집하도록 지시할 때 변경 사항을 기록합니다
* **세션별 재정의**: [`CLAUDE_CODE_BASH_EDIT_DIFF`](/docs/ko/env-vars)가 한 세션에 대해 이 키보다 우선합니다

```json settings.json theme={null}
{
  "bashEditDiffEnabled": true
}
```

<h3 id="companyannouncements">
  `companyAnnouncements`
</h3>

시작 시 조직의 공지사항을 사용자에게 표시합니다. 둘 이상을 나열하면 Claude Code는 각 세션마다 하나를 무작위로 선택합니다. 사람의 첫 실행 시 첫 번째 항목을 표시합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열 배열
* **기본값**: 설정되지 않음, 따라서 공지사항이 표시되지 않습니다

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

입력 상자에서 [`!` 접두사](/docs/ko/interactive-mode#shell-mode-with-prefix)로 입력하는 셸 명령, Claude Code가 직접 실행하고 세션에 추가하는 명령을 Bash 또는 PowerShell 중 어느 것이 실행할지 선택합니다.

`"powershell"`은 [PowerShell 도구](/docs/ko/tools-reference#powershell-tool)가 켜져 있을 때만 작동합니다. 이 도구는 Git Bash가 없는 Windows에서 기본적으로 켜져 있으며, Git Bash가 있는 Windows에서 claude.ai 및 Console 계정에 대해 켜져 있습니다. Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry 세션, macOS, Linux 및 WSL에서 `CLAUDE_CODE_USE_POWERSHELL_TOOL=1`을 설정하여 도구를 켭니다. 도구를 끄려면 해당 변수를 `0`으로 설정합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열, 다음 중 하나:
  * `"bash"`: Claude Code는 `!` 명령을 Bash에서 실행합니다
  * `"powershell"`: Claude Code는 `!` 명령을 PowerShell에서 실행합니다
* **기본값**: `"bash"`, 또는 Bash를 사용할 수 없을 때 Windows에서 `"powershell"`

```json settings.json theme={null}
{
  "defaultShell": "powershell"
}
```

지정한 셸을 사용할 수 없으면 Claude Code는 다른 셸을 사용합니다: `"powershell"`은 PowerShell 도구가 꺼져 있을 때 Bash로 폴백하고, `"bash"`는 Bash가 설치되지 않았을 때 PowerShell로 폴백합니다.

<h3 id="dialogexpiry">
  `dialogExpiry`
</h3>

Claude Code가 [원격 클라이언트](/docs/ko/remote-control#limitations)로 전달하는 대화 상자(예: Remote Control 또는 SDK 호스트)와 [보류 중인 교차 세션 메시지](/docs/ko/cross-session-messaging#control-inbound-messages)에 대한 승인 대화 상자의 기한을 설정합니다. Claude Code v2.1.236 이상에서는 동일한 기한이 터미널에 아무도 없을 수 있는 세션의 중간 세션 [Fable 사용 크레딧 동의 프롬프트](/docs/ko/model-config#fable-and-usage-credits)를 제한합니다. 기한 전에 답변이 도착하지 않으면 Claude Code는 대화 상자를 취소하고 기본 무조치로 계속합니다. Claude Code v2.1.224 이상이 필요합니다.

* **범위**: [`사용자 또는 관리됨`](#scopes)
* **유형**: 문자열, `"60s"`, `"5m"`, `"10m"` 또는 기한을 비활성화하는 `"never"` 중 하나
* **기본값**: `"5m"`
* **세션별 재정의**: [`CLAUDE_CODE_USER_DIALOG_TIMEOUT_MS`](/docs/ko/env-vars)가 한 세션에 대해 이 키보다 우선합니다

```json settings.json theme={null}
{
  "dialogExpiry": "10m"
}
```

권한 프롬프트 및 [`AskUserQuestion`](/docs/ko/tools-reference#askuserquestion-tool-behavior) 질문은 자체 흐름을 사용하며 이 기한의 적용을 받지 않습니다. 이 행은 `/config`에 **대화 상자 만료**로 표시되며, Claude Code v2.1.232 이상이 필요하고, Claude Code는 관리 설정이나 `--settings` 플래그가 키를 설정하는 동안 행을 숨깁니다.

<h3 id="editormode">
  `editorMode`
</h3>

입력 프롬프트의 키 바인딩 모드를 선택합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열, 다음 중 하나:
  * `"normal"`: 프롬프트 입력의 표준 키 바인딩
  * `"vim"`: NORMAL, INSERT 및 VISUAL 모드가 있는 vim 스타일 편집
* **기본값**: `"normal"`

```json settings.json theme={null}
{
  "editorMode": "vim"
}
```

`/config`에 **편집기 모드**로 표시되며, 이 키를 사용자 설정에 씁니다.

<h3 id="emojicompletionenabled">
  `emojiCompletionEnabled`
</h3>

프롬프트 입력에서 `:` 다음에 단축 코드를 입력할 때 이모지 제안을 표시하고, `:heart:`와 같은 완성된 단축 코드를 이모지로 바꿉니다. 둘 다 끄려면 `false`로 설정합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: Claude Code는 `:` 후 이모지 제안을 표시하고 완성된 단축 코드를 이모지로 바꿉니다
  * `false`: Claude Code는 이모지를 제안하거나 단축 코드를 바꾸지 않습니다
* **기본값**: `true`

```json settings.json theme={null}
{
  "emojiCompletionEnabled": false
}
```

[이모지 단축 코드](/docs/ko/interactive-mode#emoji-shortcodes)를 참조하세요. Claude Code v2.1.217 이상이 필요합니다.

<span id="file-suggestion-settings" />

<h3 id="filesuggestion">
  `fileSuggestion`
</h3>

내장 파일 제안 대신 `@` 파일 경로 자동 완성을 제공하는 자신의 명령을 실행합니다. 내장 제안은 빠른 파일 시스템 순회를 사용합니다. 대규모 모노레포는 사전 구축된 파일 인덱스와 같은 프로젝트별 인덱싱으로 더 잘 작동할 수 있습니다.

* **범위**: [`모든 파일`](#scopes). [상태 줄 및 파일 제안 게이트](#status-line-and-file-suggestion-gates) 아래에서 Claude Code는 명령을 끄거나 관리 값만 실행하고 경고 없이 당신의 것을 건너뜁니다.
* **유형**: `type`(항상 `"command"`)과 실행할 셸 명령인 `command`를 포함하는 객체
* **기본값**: 설정되지 않음, 따라서 Claude Code는 내장 파일 제안을 사용합니다

```json settings.json theme={null}
{
  "fileSuggestion": {
    "type": "command",
    "command": "~/.claude/file-suggestion.sh"
  }
}
```

이를 저장한 후 프롬프트에서 `@` 다음에 경로의 일부를 입력합니다: 제안은 명령의 출력에서 나옵니다.

<h4 id="command-input-and-output">
  명령 입력 및 출력
</h4>

Claude Code는 [hooks](/docs/ko/hooks)와 동일한 환경 변수(예: `CLAUDE_PROJECT_DIR`)로 명령을 실행하고 5초 후 대기를 중지합니다. 명령은 지금까지 입력한 내용을 보유하는 `query` 필드가 있는 JSON을 stdin에서 받습니다:

```json theme={null}
{"query": "src/comp"}
```

stdout에 줄 바꿈으로 구분된 파일 경로를 인쇄합니다. Claude Code는 최대 15개를 표시합니다:

```text theme={null}
src/components/Button.tsx
src/components/Modal.tsx
src/components/Form.tsx
```

다음 스크립트는 쿼리를 읽고 저장소 파일 인덱스에 전달합니다:

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

정규식이 턴 출력과 일치할 때 입력 상자 아래 바닥글에 클릭 가능한 배지를 렌더링합니다: 도구 결과(파일 내용 및 가져온 페이지 포함) 및 Claude의 자체 응답. 이를 사용하여 검토 도구 및 문제 추적기와 같은 프로젝트 CLI에서 인쇄한 ID를 세션 링크로 변환합니다.

* **범위**: [`사용자 또는 관리됨`](#scopes)
* **유형**: 객체 배열, 각각 `type`을 `"regex"`로 설정, `pattern` 정규식, `url` 템플릿, 선택적 `label`을 포함합니다. `url` 및 `label`의 `{name}` 자리 표시자는 `pattern`의 명명된 캡처 그룹에서 채워집니다
* **기본값**: 설정되지 않음, 따라서 배지가 렌더링되지 않습니다

이 예제는 `PROJ-1234`와 같은 문제 키와 일치하고 캡처된 키에서 각 링크를 구축합니다:

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

이것이 구성되면, `PROJ-1234`가 도구 결과 또는 Claude의 응답에 나타날 때, `PROJ-1234` 배지가 바닥글에 나타나 `https://issues.example.com/browse/PROJ-1234`로 연결됩니다.

<h4 id="badge-constraints">
  배지 제약
</h4>

각 항목의 URL, 레이블 및 배지 수는 다음과 같이 제한됩니다:

| 제약      | 동작                                                                                                                                                                      |
| :------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| URL 원본  | 캡처된 값은 URL 인코딩되고 구성된 URL은 템플릿의 리터럴 원본을 공유해야 합니다. 캡처는 경로 세그먼트 또는 쿼리 값을 채울 수 있지만 링크가 가리키는 위치를 변경할 수 없습니다                                                                  |
| URL 길이  | 2048자보다 긴 구성된 URL은 삭제됩니다                                                                                                                                                |
| URL 스키마 | `https`, `http` 또는 인식된 편집기 또는 작업 공간 딥 링크 스키마여야 합니다: `vscode`, `vscode-insiders`, `cursor`, `windsurf`, `zed`, `jetbrains`, `idea`, `slack`, `linear`, `notion`, `figma` |
| 레이블     | 일치한 텍스트로 기본값이 지정되고 28개 표시 열로 잘립니다                                                                                                                                       |
| 배지 수    | 최대 5개의 배지가 렌더링됩니다. 가장 오래된 것은 최신 일치로 대체되고 `/clear`는 이를 제거합니다                                                                                                             |

턴이 완료되면 Claude Code는 메인 스레드에서 각 항목의 `pattern` 정규식을 턴 출력과 일치시키므로 느린 정규식은 완료될 때까지 UI를 차단합니다. `(a+)+$`와 같은 중첩된 수량자는 특정 입력에 대해 지수적으로 오래 걸릴 수 있고 세션을 고정시킬 수 있으므로 각 `pattern`을 선형으로 유지하고 `+` 또는 `*`를 중첩하지 마세요.

바닥글 배지는 구성된 [사용자 정의 상태 줄](/docs/ko/statusline)과 함께 렌더링됩니다. 둘 다 서로를 대체하지 않습니다. 세션 데이터에서 자체 콘텐츠를 계산하는 스크립트 기반 행에는 상태 줄을 사용하고, 대화에서 ID를 스크립트 없이 링크로 변환하려면 바닥글 배지를 사용합니다.

<h3 id="keybindingflavor">
  `keybindingFlavor`
</h3>

<Warning>
  v2.1.261 이후 더 이상 사용되지 않으며 효과가 없습니다. 프롬프트의 단어 편집 키는 항상 [readline 규칙](/docs/ko/interactive-mode#make-ctrl-w-delete-back-to-whitespace)을 따릅니다(Bash처럼). Claude Code는 여전히 `keybindingFlavor`를 허용하므로 이를 설정하는 설정 파일은 유효합니다.
</Warning>

v2.1.238부터 v2.1.260까지, 이를 `"readline"`으로 설정하면 `Ctrl+W`가 이전 단어만이 아니라 이전 공백으로 돌아가 삭제합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열, `"classic"` 또는 `"readline"`
* **기본값**: 설정되지 않음

<h3 id="prefersreducedmotion">
  `prefersReducedMotion`
</h3>

스피너, 반짝임 및 플래시 효과와 같은 인터페이스 애니메이션을 줄이거나 끕니다. `/config`에 **동작 감소**로 표시됩니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: Claude Code는 스피너, 반짝임 및 플래시 효과와 같은 인터페이스 애니메이션을 줄이거나 끕니다
  * `false`: 설정되지 않은 것과 동일합니다. Claude Code는 애니메이션을 표시합니다
* **기본값**: `false`

```json settings.json theme={null}
{
  "prefersReducedMotion": true
}
```

<h3 id="promptsuggestionenabled">
  `promptSuggestionEnabled`
</h3>

[프롬프트 제안](/docs/ko/interactive-mode#prompt-suggestions)을 표시하거나 숨깁니다. 프롬프트 입력에 나타나는 회색 예측입니다. `false`로 설정하거나 `/config`에서 **프롬프트 제안**을 끄면 숨깁니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: 프롬프트 입력에서 프롬프트 제안을 봅니다
  * `false`: Claude Code는 프롬프트 제안을 숨깁니다
* **기본값**: `true`
* **세션별 재정의**: [`CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION`](/docs/ko/env-vars)이 한 세션에 대해 이 키보다 우선합니다

```json settings.json theme={null}
{
  "promptSuggestionEnabled": false
}
```

프롬프트 제안에는 원격 분석이 켜진 claude.ai 또는 Console 계정이 필요합니다. Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry에서 또는 [`DISABLE_TELEMETRY`](/docs/ko/env-vars)와 같이 원격 분석이 꺼져 있으면 이 키는 효과가 없고 `CLAUDE_CODE_ENABLE_PROMPT_SUGGESTION=1`만 이를 켭니다.

<h3 id="respectgitignore">
  `respectGitignore`
</h3>

`@` 파일 선택기가 `.gitignore` 패턴과 일치하는 파일을 제외할지 여부를 제어합니다. `/config`에 **파일 선택기에서 .gitignore 존중**으로 표시됩니다.

* **범위**: [`모든 파일`](#scopes). 설정 파일이 이를 설정하지 않으면 Claude Code는 `/config` 토글이 쓰는 `~/.claude.json`의 `respectGitignore`로 폴백합니다.
* **유형**: 부울
  * `true`: `@` 파일 선택기는 `.gitignore` 패턴과 일치하는 파일을 제외합니다
  * `false`: `@` 파일 선택기는 `.gitignore` 패턴과 일치하는 파일을 포함합니다
* **기본값**: `true`

```json settings.json theme={null}
{
  "respectGitignore": false
}
```

<h3 id="respondtobashcommands">
  `respondToBashCommands`
</h3>

입력 상자에서 [`!` 접두사](/docs/ko/interactive-mode#shell-mode-with-prefix)로 셸 명령을 실행한 후 Claude가 응답할지 여부를 선택합니다. 기본적으로 Claude Code는 명령의 출력을 대화에 추가하고 Claude가 이에 응답합니다. 이 키를 `false`로 설정하면 응답 없이 출력을 컨텍스트에 추가하므로 여러 명령을 실행하고 함께 질문할 수 있습니다. Claude Code v2.1.186 이상이 필요합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: Claude Code는 명령의 출력을 대화에 추가하고 Claude가 이에 응답합니다
  * `false`: Claude Code는 응답 없이 출력을 컨텍스트에 추가합니다
* **기본값**: `true`

```json settings.json theme={null}
{
  "respondToBashCommands": false
}
```

[`!` 접두사를 사용한 셸 모드](/docs/ko/interactive-mode#shell-mode-with-prefix)를 참조하세요. Claude Code v2.1.186 이상이 필요합니다.

<h3 id="showclearcontextonplanaccept">
  `showClearContextOnPlanAccept`
</h3>

Claude가 [계획 모드](/docs/ko/permission-modes#review-and-approve-a-plan)에서 계획을 완료하면 승인 메뉴를 표시합니다. 계획은 많은 컨텍스트를 사용할 수 있으므로 이 키는 해당 메뉴에 첫 번째 옵션인 \*\*예, 컨텍스트를 지우고 …\*\*을 추가하여 계획을 승인하고, 대화 컨텍스트를 지우고, 계획만으로 구현을 시작합니다. 레이블의 나머지는 세션이 계속되는 권한 모드의 이름을 지정하고 계획이 사용한 컨텍스트의 양을 표시합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: 계획 승인 메뉴는 계획을 승인하고 대화 컨텍스트를 지우는 첫 번째 옵션인 \*\*예, 컨텍스트를 지우고 …\*\*을 가집니다
  * `false`: 계획 승인 메뉴는 컨텍스트 지우기 옵션을 표시하지 않습니다
* **기본값**: `false`

```json settings.json theme={null}
{
  "showClearContextOnPlanAccept": true
}
```

<h3 id="showturnduration">
  `showTurnDuration`
</h3>

각 응답 후 턴 지속 시간 메시지를 표시하거나 숨깁니다(예: "Cooked for 1m 6s · done 6:05 PM"). "done" 후의 시계는 턴이 완료된 시간을 표시합니다. [`timeFormat`](#timeformat) 및 [`timeZone`](#timezone)은 형식과 영역을 제어합니다. `/config`에 **턴 지속 시간 표시**로 표시됩니다.

* **범위**: [`모든 파일`](#scopes). 설정 파일이 이를 설정하지 않으면 이전 버전의 `~/.claude.json`의 값이 적용됩니다.
* **유형**: 부울
  * `true`: 각 응답 후 턴 지속 시간 메시지를 봅니다
  * `false`: Claude Code는 턴 지속 시간 메시지를 숨깁니다
* **기본값**: `true`

```json settings.json theme={null}
{
  "showTurnDuration": false
}
```

<h3 id="spellcheck">
  `spellcheck`
</h3>

입력하면서 프롬프트 입력에서 잘못된 단어에 밑줄을 그으세요. 설치한 맞춤법 검사기를 사용합니다. Claude Code는 입력 상자의 텍스트만 확인합니다. [입력하면서 맞춤법 확인](/docs/ko/interactive-mode#check-spelling-as-you-type)은 aspell, hunspell 또는 ispell을 설치하고 검사기가 다루는 내용을 다룹니다. Claude Code v2.1.235 이상이 필요합니다.

* **범위**: [`사용자 또는 관리됨`](#scopes). 이를 설정하는 가장 높은 계층의 블록이 전체적으로 적용됩니다.
* **유형**: `enabled`(부울), `checker`(`"aspell"`, `"hunspell"`, `"ispell"` 또는 `"auto"`), `language`(문자열, 검사기의 사전 이름으로 전달됨), `color`(문자열, 터미널 색상 이름, `#rrggbb`, `rgb(r,g,b)`, `ansi256(n)` 또는 `ansi:<name>`)를 포함하는 객체
* **기본값**: 설정되지 않음, 따라서 맞춤법 검사는 꺼져 있습니다. `checker`는 `"auto"`로 기본값이 지정되며, `PATH`에서 찾은 첫 번째입니다. `language`는 검사기 자체의 사전으로 기본값이 지정됩니다. `color`는 테마의 오류 색상으로 기본값이 지정됩니다

```json settings.json theme={null}
{
  "spellcheck": { "enabled": true, "language": "en_GB" }
}
```

<h3 id="spinnertipsenabled">
  `spinnerTipsEnabled`
</h3>

Claude가 작업하는 동안 스피너 줄은 "Use Plan Mode to prepare for a complex request before making changes. Press Shift+Tab twice to enable."과 같은 Claude Code 기능에 대한 짧은 팁을 회전합니다. 이 키를 `false`로 설정하면 숨깁니다. `/config`에 **팁 표시**로 표시됩니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: Claude가 작업하는 동안 스피너에서 팁을 봅니다
  * `false`: Claude Code는 스피너 팁을 숨깁니다
* **기본값**: `true`

```json settings.json theme={null}
{
  "spinnerTipsEnabled": false
}
```

<h3 id="spinnertipsoverride">
  `spinnerTipsOverride`
</h3>

Claude Code가 Claude가 작업하는 동안 표시하는 [스피너 팁](#spinnertipsenabled)에 자신의 팁을 추가하거나 내장 팁을 당신의 것으로 바꿉니다. Claude Code는 당신의 팁을 내장 팁과 동일한 회전에 넣습니다: 가장 오래 표시되지 않은 팁을 선택하고, 여전히 쿨다운 중인 팁을 건너뛰고, 우선순위로 동점을 깹니다.

[`spinnerTipsEnabled`](#spinnertipsenabled)을 `false`로 설정하면 Claude Code는 모든 팁(당신의 것 포함)을 숨깁니다.

* **범위**: [`모든 파일`](#scopes). Claude Code는 팁 객체, `tipsFile`, `label` 및 `excludeDefault`를 사용자 설정, `--settings` 플래그 및 관리 설정에서 존중합니다. 프로젝트 및 로컬 설정에서는 평문 문자열 팁만 읽습니다.
* **유형**: `tips`, `tipsFile`, `label` 및 `excludeDefault` 필드를 포함하는 객체, 각각 선택 사항
* **기본값**: 설정되지 않음, 따라서 Claude Code는 내장 팁만 표시합니다

팁 객체, `tipsFile`, `label` 및 범위 줄의 규칙(프로젝트 및 로컬 설정이 평문 문자열만 기여)에는 Claude Code v2.1.247 이상이 필요합니다. 이전 버전에서는 프로젝트 또는 로컬 파일의 `excludeDefault`도 적용됩니다.

각 `tips` 항목은 평문 문자열 또는 다음 필드를 포함하는 객체입니다:

| 필드                 | 필수  | 설명                                                                                                            |
| :----------------- | :-- | :------------------------------------------------------------------------------------------------------------ |
| `id`               | 예   | 최대 64개의 문자, 숫자, `.`, `_` 또는 `-`. Claude Code는 팁의 표시 기록을 이것으로 키합니다. 동일한 id를 가진 두 항목 중 Claude Code는 첫 번째를 사용합니다 |
| `text`             | 예   | 팁, 최대 500자의 한 줄. Claude Code는 ANSI 이스케이프 및 제어 문자를 제거하고 공백을 축소합니다                                              |
| `cooldownSessions` | 아니오 | Claude Code가 팁을 다시 표시하기 전에 대기하는 세션, `0`에서 `1000`, 기본값 `0`                                                     |
| `priority`         | 아니오 | 동일하게 오래 표시되지 않은 팁 중 순서, 더 높음, `-10`에서 `10`, 기본값 `0`                                                           |

Claude Code는 평문 문자열을 해당 기본값과 위치 기반 id를 가진 팁으로 읽으므로 목록을 다시 정렬하면 표시 기록이 재설정됩니다. 팁에 `id`를 제공하여 편집 전체에서 기록을 유지합니다.

Claude Code는 `tips` 및 `tipsFile` 전체에서 최대 200개의 팁을 읽고 설정 파일을 거부하는 대신 디버그 경고와 함께 잘못된 항목을 삭제합니다.

나머지 필드를 사용하여 팁 파일의 이름을 지정하고, 접두사를 설정하고, 내장 팁을 숨깁니다:

* `tipsFile`: 동일한 항목의 배열을 보유하는 로컬 JSON 파일에 대한 절대 또는 `~/` 경로, 또는 `tips` 배열을 포함하는 객체, 최대 256 KB. Claude Code는 프로세스당 한 번 파일을 읽으므로 다음 시작 시 편집 내용을 로드합니다. [서버 관리 설정](/docs/ko/server-managed-settings)을 통해 설정할 수 없습니다. 인라인 `tips`를 배포하거나 온디스크 `managed-settings.json`에 경로를 배포합니다.
* `label`: Claude Code가 사용자, `--settings` 및 관리 설정의 팁 앞에 표시하는 접두사, 최대 40자. 기본값은 `Tip`이며, 내장 팁과 동일한 접두사이고, 프로젝트 및 로컬 설정의 팁은 항상 이를 사용합니다.
* `excludeDefault`: 내장 팁을 숨기고 당신의 것만 표시하려면 `true`로 설정합니다. Claude Code가 예를 들어 `tipsFile`이 존재하지 않거나 모든 항목이 유효하지 않기 때문에 당신의 팁을 로드할 수 없으면 빈 스피너 대신 내장 회전을 유지합니다.

둘 이상의 설정 파일이 키를 설정하면 Claude Code는 모든 팁을 표시하고 각각에 대해 이를 설정하는 관리 설정, `--settings` 플래그 및 사용자 설정 중 가장 높은 우선순위 것에서 `tipsFile`, `label` 및 `excludeDefault`를 가져옵니다.

이 예제는 사용자 설정에 있으며 `Acme tip` 접두사 아래 회전에 평문 문자열 팁과 객체 팁을 추가합니다:

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

예제의 각 필드는 Claude Code가 팁을 표시하는 방식의 한 가지를 변경합니다:

* `label`: Claude Code는 두 팁을 `Tip: ...` 대신 `Acme tip: ...`로 표시합니다.
* 평문 문자열: Claude Code는 기본값을 제공하므로 다음 세션에서 다시 나타날 수 있습니다.
* `id`: Claude Code는 두 번째 팁의 표시 기록을 `gateway-errors`로 키합니다. 따라서 팁을 추가하거나 다시 정렬한 후에도 쿨다운이 적용됩니다.
* `cooldownSessions`: Claude Code가 `gateway-errors` 팁을 표시한 후 5개 세션이 지날 때까지 해당 팁을 다시 표시하지 않습니다.
* `priority`: `gateway-errors` 팁과 다른 팁이 동일한 수의 세션 동안 표시되지 않았을 때(예: 둘 다 아직 표시되지 않았을 때) Claude Code는 `gateway-errors`를 먼저 표시합니다. 평문 문자열은 기본 우선순위인 `0`을 가집니다.

Claude가 작업하는 동안 Claude Code는 당신의 팁을 스피너에 당신의 접두사(예: `Acme tip: Run /review before opening a PR`)와 함께 표시합니다.

<h3 id="spinnerverbs">
  `spinnerVerbs`
</h3>

턴이 진행 중인 동안 스피너는 "Accomplishing", "Architecting" 또는 "Baking"과 같은 회전 동사를 표시합니다. 이 키를 사용하여 해당 회전에 자신의 동사를 추가하거나 내장 목록을 당신의 것으로 바꿉니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: `verbs` 문자열 배열과 `mode`를 포함하는 객체, 다음 중 하나:
  * `"append"`: Claude Code는 당신의 동사를 내장 집합에 추가합니다
  * `"replace"`: Claude Code는 당신의 동사만 표시합니다
* **기본값**: 설정되지 않음, 따라서 Claude Code는 내장 동사를 사용합니다

이 예제는 내장 집합에 두 개의 동사를 추가합니다:

```json settings.json theme={null}
{
  "spinnerVerbs": {
    "mode": "append",
    "verbs": ["Pondering", "Crafting"]
  }
}
```

`"replace"` 모드에서 빈 `verbs` 배열을 사용하면 Claude Code는 내장 동사를 유지합니다.

<h3 id="statusline">
  `statusLine`
</h3>

프롬프트 아래에 [상태 줄](/docs/ko/statusline)을 렌더링하는 자신의 명령을 실행합니다. 모델, 비용 또는 git 분기와 같은 컨텍스트를 포함합니다. 선택적 필드는 간격을 조정하고, 정기적인 재실행을 추가하고, 스크립트가 `vim.mode` 자체를 렌더링할 때 내장 vim 모드 표시기를 숨깁니다.

* **범위**: [`모든 파일`](#scopes). [`allowManagedHooksOnly`](#allowmanagedhooksonly)가 켜져 있거나 [`disableAllHooks`](#disableallhooks)가 관리 설정 외부에서 설정되면 관리 설정 값만 실행됩니다.
* **유형**: `type`을 `"command"`로 설정하고 `command` 문자열을 포함하는 객체, 그리고 선택적 `padding`(문자 수), `refreshInterval`(초 단위, 최소 `1`), `hideVimModeIndicator`(부울)
* **기본값**: 설정되지 않음, 따라서 상태 줄이 없습니다

이 예제는 모델 이름과 컨텍스트 사용을 인쇄하고 2자의 수평 간격을 추가합니다:

```json settings.json theme={null}
{
  "statusLine": {
    "type": "command",
    "command": "jq -r '\"[\\(.model.display_name)] \\(.context_window.used_percentage // 0)% context\"'",
    "padding": 2
  }
}
```

예제에는 [`jq`](https://jqlang.org/)가 설치되어 있어야 하고 셸에서 실행됩니다. PowerShell 및 Git Bash 동등물은 [Windows 구성](/docs/ko/statusline#windows-configuration)을 참조하세요. 전체 설정은 [상태 줄 수동 구성](/docs/ko/statusline#manually-configure-a-status-line)을 참조하세요.

<h3 id="subagentstatusline">
  `subagentStatusLine`
</h3>

Claude가 [서브에이전트](/docs/ko/sub-agents)를 실행할 때 Claude Code는 프롬프트 아래의 작업 표시에 나열합니다. 서브에이전트당 한 행씩 `name · description · token count`를 표시합니다. 이 키를 사용하면 자신의 명령을 실행하여 해당 행을 다시 쓸 수 있습니다. 예를 들어 각 서브에이전트의 컨텍스트 사용을 백분율로 표시합니다. 각 새로 고침에서 Claude Code는 표시되는 행을 stdin의 하나의 JSON 객체로 보냅니다. `tasks` 배열은 각 서브에이전트의 `id`, `name`, `status`, `model`, `tokenCount` 등을 포함하고, 당신이 `{"id", "content"}` 줄로 다시 쓰는 각 `id`에 대한 행을 바꿉니다. 당신이 다시 쓰지 않은 행은 기본 렌더링을 유지합니다.

* **범위**: [`모든 파일`](#scopes). [`allowManagedHooksOnly`](#allowmanagedhooksonly)가 켜져 있거나 [`disableAllHooks`](#disableallhooks)가 관리 설정 외부에서 설정되면 관리 설정 값만 실행됩니다.
* **유형**: `type`을 `"command"`로 설정하고 `command` 문자열을 포함하는 객체
* **기본값**: 설정되지 않음, 따라서 Claude Code는 기본 행을 렌더링합니다

```json settings.json theme={null}
{
  "subagentStatusLine": {
    "type": "command",
    "command": "jq -c '.tasks[] | {id, content: \"\\(.name): \\(.tokenCount) tokens\"}'"
  }
}
```

[서브에이전트 상태 줄](/docs/ko/statusline#subagent-status-lines)을 참조하세요.

<h3 id="syntaxhighlightingdisabled">
  `syntaxHighlightingDisabled`
</h3>

Claude Code는 터미널에 표시하는 diff, 코드 블록 및 파일 미리보기에서 내장 강조 표시기를 사용하여 언어별로 코드를 색칠합니다. 플러그인이나 언어 서버는 관련이 없습니다. 이 키를 `true`로 설정하면 평문으로 표시합니다. 예를 들어 색상이 터미널 테마와 충돌하거나 화면 판독기를 느리게 합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: Claude Code는 diff, 코드 블록 및 파일 미리보기에서 구문 강조 표시를 끕니다
  * `false`: Claude Code는 구문을 강조 표시합니다
* **기본값**: `false`

```json settings.json theme={null}
{
  "syntaxHighlightingDisabled": true
}
```

<h3 id="terminalprogressbarenabled">
  `terminalProgressBarEnabled`
</h3>

일부 터미널은 실행 중인 프로그램에 대해 탭이나 작업 표시줄에 진행 표시기를 표시할 수 있습니다. Claude가 작업하는 동안 Claude Code는 진행 중 상태를 터미널에 보고하므로 다른 탭이나 창에서 세션이 여전히 바쁜지 확인할 수 있습니다. 표시기는 [백그라운드 서브에이전트](/docs/ko/sub-agents#run-subagents-in-foreground-or-background) 또는 [동적 워크플로우](/docs/ko/workflows)가 여전히 실행 중인 동안 턴이 끝난 후에도 표시 상태를 유지하고, 세션이 유휴 상태가 되면 지워집니다.

Claude Code는 표시기를 지원하는 터미널에서만 보고합니다: ConEmu, Ghostty 1.2.0 이상, iTerm2 3.6.6 이상. 이 키를 `false`로 설정하면 Claude Code가 보고하지 않도록 중지합니다. `/config`에 **터미널 진행 표시줄**로 표시됩니다.

* **범위**: [`모든 파일`](#scopes). 설정 파일이 이를 설정하지 않으면 이전 버전의 `~/.claude.json`의 값이 적용됩니다.
* **유형**: 부울
  * `true`: 지원하는 터미널에서 터미널 진행 표시줄을 봅니다
  * `false`: Claude Code는 터미널 진행 표시줄을 숨깁니다
* **기본값**: `true`

```json settings.json theme={null}
{
  "terminalProgressBarEnabled": false
}
```

<h3 id="terminaltitlefromrename">
  `terminalTitleFromRename`
</h3>

Claude Code는 터미널 탭의 제목을 설정합니다. 기본적으로 대화에서 생성한 제목을 사용하고, `/rename` 또는 `--name`으로 세션에 [이름](/docs/ko/sessions#name-your-sessions)을 지정하면 탭에 해당 이름이 대신 표시됩니다. 이 키를 `false`로 설정하면 세션 이름을 지정한 후에도 탭에 생성된 제목을 유지합니다. 이름 자체는 여전히 적용되므로 `/resume <name>`과 세션 선택기가 이를 찾습니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: 터미널 탭 제목은 설정한 세션 이름을 표시합니다
  * `false`: 탭은 Claude Code가 대화에서 생성한 제목을 유지합니다
* **기본값**: `true`

```json settings.json theme={null}
{
  "terminalTitleFromRename": false
}
```

Claude Code가 터미널 제목을 전혀 업데이트하지 않도록 하려면 [`CLAUDE_CODE_DISABLE_TERMINAL_TITLE`](/docs/ko/env-vars)을 `1`로 설정합니다.

<h3 id="theme">
  `theme`
</h3>

인터페이스의 색 테마를 선택합니다. `/config`에 **테마**로 표시됩니다.

* **범위**: [`모든 파일`](#scopes). 설정 파일이 이를 설정하지 않으면 이전 버전의 `~/.claude.json`의 값이 적용됩니다.
* **유형**: 문자열, 다음 중 하나:
  * `"auto"`: 터미널의 밝은 또는 어두운 배경과 일치합니다
  * `"dark"`: 어두운 테마
  * `"light"`: 밝은 테마
  * `"dark-daltonized"`: 색맹 친화적 색상이 있는 어두운 테마
  * `"light-daltonized"`: 색맹 친화적 색상이 있는 밝은 테마
  * `"dark-ansi"`: 터미널의 ANSI 색상 팔레트만 사용하는 어두운 테마
  * `"light-ansi"`: 터미널의 ANSI 색상 팔레트만 사용하는 밝은 테마
  * `"custom:<slug>"` 또는 `"custom:<plugin-name>:<slug>"`: `~/.claude/themes/` 또는 플러그인의 사용자 정의 테마
* **기본값**: `"dark"`

```json settings.json theme={null}
{
  "theme": "light-daltonized"
}
```

[사용자 정의 테마 만들기](/docs/ko/terminal-config#create-a-custom-theme)를 참조하세요.

<h3 id="timeformat">
  `timeFormat`
</h3>

Claude Code가 인터페이스에 표시하는 시간을 작성하는 방식을 선택합니다. 예를 들어 각 턴 지속 시간 메시지의 끝에 있는 `done 6:05 PM` 및 [기록 뷰어](/docs/ko/interactive-mode#transcript-viewer)의 타임스탬프입니다. 사전 설정을 선택하려면 `/config`를 실행하고 **시간 형식**을 설정합니다. Claude Code v2.1.257 이상이 필요합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열, 다음 중 하나:
  * `"auto"`: 설정되지 않은 것과 동일합니다. 각 시간은 내장 형식을 유지하며, 턴 지속 시간 메시지에서 로캘을 따릅니다
  * `"12-hour"`: 12시간 시계
  * `"24-hour"`: 24시간 시계
  * `"24-hour-utc"`: `18:05Z`와 같이 분 후 `Z`가 있는 UTC의 24시간 시계입니다. Claude Code는 이 사전 설정에 대해 [`timeZone`](#timezone)을 무시합니다
  * `"%H:%M"`과 같은 strftime 패턴: Claude Code는 패턴으로 각 시간을 씁니다. `%`를 포함하는 모든 값은 패턴이고, 사전 설정 외부의 다른 값은 `"auto"`로 계산됩니다
* **기본값**: `"auto"`

```json settings.json theme={null}
{
  "timeFormat": "24-hour"
}
```

`/config`는 사전 설정만 제공하므로 strftime 패턴을 사용하려면 설정 파일에 키를 추가합니다. 이 예제는 각 시간을 2자리 24시간 시계로 표시합니다:

```json settings.json theme={null}
{
  "timeFormat": "%H:%M"
}
```

턴 지속 시간 메시지 및 기록 뷰어는 `18:05`와 같은 시간을 표시합니다. 기록 뷰어에서 패턴은 전체 타임스탬프이므로 날짜를 원할 때 날짜 지시문을 추가합니다. 이 예제는 시계 앞에 날짜를 넣습니다:

```json settings.json theme={null}
{
  "timeFormat": "%Y-%m-%d %H:%M"
}
```

동일한 표면은 `2026-09-01 18:05`와 같은 시간을 표시합니다.

<h3 id="timezone">
  `timeZone`
</h3>

시스템이 아닌 다른 시간대에서 인터페이스의 시간을 표시합니다. [IANA 시간대 이름](https://www.iana.org/time-zones)(예: `"UTC"` 또는 `"Europe/Dublin"`)으로 설정합니다. [`timeFormat`](#timeformat)이 제어하는 시간은 이 영역에 표시됩니다. `timeFormat`이 `"24-hour-utc"`이면 시간은 UTC로 유지되고 Claude Code는 이 키를 무시합니다. `/config`에는 이 키에 대한 행이 없으므로 설정 파일에서 설정합니다. Claude Code v2.1.257 이상이 필요합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열, IANA 시간대 이름. Claude Code가 이름을 인식하지 못하면 시스템 시간대를 사용합니다
* **기본값**: 설정되지 않음, 따라서 시간은 시스템 시간대에 표시됩니다

```json settings.json theme={null}
{
  "timeZone": "Europe/Dublin"
}
```

<h3 id="tui">
  `tui`
</h3>

터미널 UI 렌더러를 선택합니다. 깜박임 없는 [alt-screen 렌더러](/docs/ko/fullscreen)를 가상화된 스크롤백과 함께 사용하려면 `"fullscreen"`을 사용하거나, 클래식 메인 화면 렌더러를 사용하려면 `"default"`를 사용합니다. `/tui fullscreen` 또는 `/tui default`를 실행하면 이 키가 당신을 위해 작성됩니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열, 다음 중 하나:
  * `"default"`: 클래식 메인 화면 렌더러
  * `"fullscreen"`: 가상화된 스크롤백이 있는 깜박임 없는 alt-screen 렌더러
* **기본값**: 설정되지 않음, 따라서 Claude Code는 [당신을 위해 렌더러를 선택합니다](/docs/ko/fullscreen#fullscreen-by-default)
* **세션별 재정의**: [`CLAUDE_CODE_NO_FLICKER`](/docs/ko/env-vars) 및 [`CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN`](/docs/ko/env-vars)이 한 세션에 대해 이 키보다 우선합니다: `CLAUDE_CODE_NO_FLICKER=1`은 전체 화면을 켜고, `CLAUDE_CODE_NO_FLICKER=0` 또는 `CLAUDE_CODE_DISABLE_ALTERNATE_SCREEN=1`은 끕니다. 둘 다 설정되면 Claude Code는 끕니다

```json settings.json theme={null}
{
  "tui": "fullscreen"
}
```

tmux `-CC` 아래 또는 Windows로의 SSH를 통해 Claude Code는 `CLAUDE_CODE_NO_FLICKER=1`을 설정하지 않으면 클래식 렌더러를 유지합니다. [에이전트 뷰](/docs/ko/agent-view)에서 열린 백그라운드 세션은 이 설정과 관계없이 항상 전체 화면 렌더러를 사용합니다.

<h3 id="verbose">
  `verbose`
</h3>

기본적으로 기록은 각 도구 호출을 짧은 요약(예: Claude가 실행한 명령 및 출력의 줄 수)으로 축소하고, 세부 정보를 원할 때 `Ctrl+O`를 눌러 전체 기록을 확장된 보기로 전환합니다. 이 키를 `true`로 설정하면 모든 도구 호출의 전체 입력 및 출력을 발생할 때 인라인으로 표시합니다. 이는 hook, MCP 서버 또는 긴 셸 명령을 디버깅할 때 유용합니다. `/config`에 **자세한 출력**으로 표시됩니다.

* **범위**: [`모든 파일`](#scopes). 설정 파일이 이를 설정하지 않으면 이전 버전의 `~/.claude.json`의 값이 적용됩니다.
* **유형**: 부울
  * `true`: 전체 도구 출력을 봅니다
  * `false`: 도구 출력의 잘린 요약을 봅니다
* **기본값**: `false`
* **세션별 재정의**: [`--verbose`](/docs/ko/cli-reference#cli-flags)가 한 세션에 대해 이 키보다 우선합니다

```json settings.json theme={null}
{
  "verbose": true
}
```

[`viewMode`](#viewmode) 값 또는 고정 `/focus` 선택이 매 세션마다 이 키를 재정의합니다.

<h3 id="viewmode">
  `viewMode`
</h3>

Claude Code가 시작하는 기록 보기를 설정합니다: `"default"`, `"verbose"` 또는 `"focus"`. 설정되면 고정 `/focus` 선택 및 [`verbose`](#verbose) 설정을 모두 재정의합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열, 다음 중 하나:
  * `"default"`: 잘린 도구 출력이 있는 일반 기록
  * `"verbose"`: 전체 도구 출력이 있는 기록
  * `"focus"`: 마지막 프롬프트만, 편집 diffstat이 있는 도구 호출의 한 줄 요약, 최종 응답. 포커스 보기에는 [전체 화면 렌더러](#tui)가 필요합니다
* **기본값**: 설정되지 않음, 따라서 `verbose` 설정 및 마지막 `/focus` 선택이 적용됩니다
* **세션별 재정의**: [`--verbose`](/docs/ko/cli-reference#cli-flags)가 한 세션에 대해 이 키보다 우선합니다

```json settings.json theme={null}
{
  "viewMode": "focus"
}
```

<h3 id="viminsertmoderemaps">
  `vimInsertModeRemaps`
</h3>

[vim 편집기 모드](/docs/ko/interactive-mode#vim-editor-mode)에서 두 키 INSERT 모드 시퀀스를 Escape로 매핑합니다. 각 키는 정확히 순서대로 입력된 두 개의 인쇄 가능한 문자이고, `"<Esc>"`는 유일하게 지원되는 대상입니다. Claude Code는 다른 항목을 무시합니다. Claude Code v2.1.208 이상이 필요합니다.

* **범위**: [`사용자 또는 관리됨`](#scopes). 저장소는 키 입력을 다시 매핑할 수 없습니다.
* **유형**: 두 문자 시퀀스를 `"<Esc>"`로 매핑하는 객체
* **기본값**: 설정되지 않음

```json settings.json theme={null}
{
  "vimInsertModeRemaps": {
    "jj": "<Esc>"
  }
}
```

`editorMode`가 `"vim"`이 아니면 효과가 없습니다. [INSERT 모드 키 시퀀스 다시 매핑](/docs/ko/interactive-mode#remap-insert-mode-key-sequences)을 참조하세요. Claude Code v2.1.208 이상이 필요합니다.

<h3 id="voice">
  `voice`
</h3>

[음성 받아쓰기](/docs/ko/voice-dictation)를 켜고 받아쓰기 키의 동작 방식을 선택합니다. Claude Code는 `/voice`를 실행할 때 이 객체를 당신을 위해 씁니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: `enabled`(부울), `autoSubmit`(보류 모드에만 적용되는 부울), `mode`(다음 중 하나)를 포함하는 객체:
  * `"hold"`: 말하는 동안 받아쓰기 키를 누르고 있다가 놓으면 중지합니다
  * `"tap"`: 키를 한 번 눌러 녹음을 시작하고 다시 눌러 보냅니다
* **기본값**: 설정되지 않음, 따라서 받아쓰기는 꺼져 있습니다. `enabled`가 `true`이고 `mode`가 설정되지 않으면 Claude Code는 `"hold"`를 사용합니다

이 예제는 받아쓰기를 켜고 키를 한 번 눌러 녹음을 시작하고 다시 눌러 보내도록 합니다:

```json settings.json theme={null}
{
  "voice": {
    "enabled": true,
    "mode": "tap"
  }
}
```

`autoSubmit`은 보류 모드에서 키를 놓을 때 프롬프트를 보냅니다. 음성 받아쓰기에는 claude.ai 계정이 필요합니다.

<h3 id="voiceenabled">
  `voiceEnabled`
</h3>

<Warning>
  v2.1.92 이후 더 이상 사용되지 않으며, [`voice`](#voice) 객체가 이를 대체했습니다. Claude Code는 여전히 이를 읽으므로 이전 설정 파일은 계속 작동하지만 새 구성은 `voice.enabled`를 설정해야 합니다.
</Warning>

이전 `voice` 객체를 사용하는 단일 부울 형식으로 음성 받아쓰기를 켭니다. 둘 다 설정되면 `voice.enabled`가 적용됩니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: claude.ai 계정으로 로그인했고 조직의 정책이 음성을 허용할 때 음성 받아쓰기가 켜져 있습니다. `voice.enabled`가 설정되지 않으면
  * `false`: 음성 받아쓰기는 꺼져 있습니다. `voice.enabled`가 설정되지 않으면
* **기본값**: 설정되지 않음

```json settings.json theme={null}
{
  "voiceEnabled": true
}
```

<h3 id="wheelscrollaccelerationenabled">
  `wheelScrollAccelerationEnabled`
</h3>

[전체 화면 렌더링](/docs/ko/fullscreen#mouse-wheel-scrolling)에서 빠른 스크롤 중 마우스 휠 스크롤 속도를 가속화합니다. 휠 노치당 일정한 스크롤 속도를 원하면 `false`로 설정합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: Claude Code는 빠른 스크롤 중 마우스 휠 스크롤 속도를 가속화합니다
  * `false`: Claude Code는 휠 노치당 일정한 속도로 스크롤합니다
* **기본값**: `true`

```json settings.json theme={null}
{
  "wheelScrollAccelerationEnabled": false
}
```

<h2 id="git-and-attribution">
  Git 및 속성
</h2>

Claude Code가 커밋 및 풀 요청에 추가하는 속성을 제어하고 git과 어떻게 작동하는지 관리합니다.

<span id="attribution-settings" />

<h3 id="attribution">
  `attribution`
</h3>

Claude Code가 git 커밋 및 풀 요청에 추가하는 속성을 사용자 정의합니다. 커밋은 기본적으로 `Co-Authored-By`와 같은 [git 트레일러](https://git-scm.com/docs/git-interpret-trailers)를 받습니다. 풀 요청 설명은 일반 텍스트를 받습니다. 아래의 하위 키를 사용하여 각 부분을 별도로 설정합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: `commit` 및 `pr` 문자열과 `sessionUrl` 부울을 포함하는 객체
* **기본값**: 설정되지 않음. 따라서 Claude Code는 각 하위 키 아래에 표시된 표준 속성을 사용합니다.

이 예제는 커밋 속성을 바꾸고, 풀 요청 속성을 제거하며, 세션 링크를 삭제합니다:

```json settings.json theme={null}
{
  "attribution": {
    "commit": "Generated with AI\n\nCo-Authored-By: AI <ai@example.com>",
    "pr": "",
    "sessionUrl": false
  }
}
```

모든 속성을 숨기려면 [`commit`](#attribution-commit) 및 [`pr`](#attribution-pr)을 빈 문자열로 설정하고 [`sessionUrl`](#attribution-sessionurl)을 `false`로 설정합니다. `commit` 또는 `pr`을 설정하면 Claude Code는 더 이상 사용되지 않는 `includeCoAuthoredBy` 설정을 무시하고 설정하지 않은 두 항목에 대해 기본 텍스트를 사용합니다.

Claude Code는 Claude에게 속성에 대한 사용자 정의 지침(예: CLAUDE.md 또는 [메모리](/docs/ko/memory) 규칙)이 이러한 커밋 및 PR 라인보다 우선한다고 알립니다. 단, 라인이 [관리되는 설정](/docs/ko/managed-settings)에서 설정된 경우는 제외합니다.

<h3 id="includecoauthoredby">
  `includeCoAuthoredBy`
</h3>

<Warning>
  v2.0.62 이후 더 이상 사용되지 않음. [`attribution`](#attribution)이 이를 대체했습니다. Claude Code는 여전히 이를 읽지만, 새로운 구성은 `attribution`을 설정해야 합니다.
</Warning>

이 키를 대체하는 [`attribution`](#attribution)을 대신 사용하십시오. 이를 통해 커밋 트레일러, 풀 요청 텍스트 및 세션 링크를 별도로 변경하거나 숨길 수 있습니다. Claude Code는 여전히 `attribution`이 존재하기 이전의 설정 파일에서 `includeCoAuthoredBy: false`를 준수하지만, `attribution.commit` 또는 `attribution.pr`을 설정하면 이를 무시합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: 설정되지 않은 것과 동일합니다. Claude Code는 커밋 트레일러 및 풀 요청 속성 텍스트를 추가합니다.
  * `false`: Claude Code는 커밋 트레일러 및 풀 요청 속성 텍스트를 모두 생략합니다. 단, `attribution`이 `commit` 또는 `pr`을 설정하는 경우 [`attribution`](#attribution) 규칙이 적용됩니다.
* **기본값**: `true`

```json settings.json theme={null}
{
  "includeCoAuthoredBy": false
}
```

현재 모든 속성을 숨기려면 [`attribution.commit`](#attribution-commit) 및 [`attribution.pr`](#attribution-pr)을 빈 문자열로 설정하고 [`attribution.sessionUrl`](#attribution-sessionurl)을 `false`로 설정합니다.

<h3 id="includegitinstructions">
  `includeGitInstructions`
</h3>

Claude Code는 Claude에게 두 가지 git 관련 컨텍스트를 제공합니다. Bash 도구의 설명에 있는 커밋 및 풀 요청 작성 방법에 대한 기본 제공 지침과 저장소의 git 상태 스냅샷입니다. 스냅샷은 현재 분기, 주 분기, `git status` 출력 및 최근 커밋을 포함합니다. Claude Code는 대화가 시작될 때 이를 읽습니다.

예를 들어 자신의 git 워크플로우 스킬을 사용할 때 이 키를 `false`로 설정하여 둘 다 제외합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: Claude Code는 기본 제공 커밋 및 풀 요청 워크플로우 지침과 git 상태 스냅샷을 포함합니다. 클라우드 세션은 스냅샷을 포함하지 않습니다.
  * `false`: Claude Code는 둘 다 제외합니다.
* **기본값**: `true`
* **세션별 재정의**: [`CLAUDE_CODE_DISABLE_GIT_INSTRUCTIONS`](/docs/ko/env-vars)는 한 세션에 대해 이 키보다 우선합니다.

```json settings.json theme={null}
{
  "includeGitInstructions": false
}
```

<h3 id="prurltemplate">
  `prUrlTemplate`
</h3>

Claude Code가 렌더링하는 PR 링크(바닥글 배지 및 도구 결과 요약)를 `github.com` 대신 내부 코드 검토 도구로 지정합니다. Claude Code는 PR URL에서 `{host}`, `{owner}`, `{repo}`, `{number}` 및 `{url}`을 대체합니다. [GitLab 병합 요청](/docs/ko/interactive-mode#gitlab-merge-requests) 링크는 두 표면 모두에서 GitLab URL을 유지합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열, 5개의 자리 표시자 중 하나를 사용하는 URL 템플릿
* **기본값**: 설정되지 않음

```json settings.json theme={null}
{
  "prUrlTemplate": "https://reviews.example.com/{owner}/{repo}/pull/{number}"
}
```

Claude Code는 자신이 직접 렌더링하는 링크에만 템플릿을 적용합니다. Claude가 메시지에 작성한 PR 번호(예: `#123`)는 Claude가 작성한 대로 유지됩니다. `/pull/<number>` 형태가 아닌 URL은 변경되지 않습니다.

<h3 id="attribution-commit">
  `attribution.commit`
</h3>

Claude Code가 git 커밋에 추가하는 속성 텍스트(모든 트레일러 포함)를 설정합니다. 커밋 속성을 숨기려면 빈 문자열로 설정합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열
* **기본값**: 설정되지 않음. 따라서 Claude Code는 `Co-Authored-By: <name> <noreply@anthropic.com>`을 추가합니다. 이름은 `Claude Sonnet 5`와 같은 세션의 활성 모델입니다.
  * Claude Code가 모델을 Claude 모델로 인식하지만 정확한 버전을 확인할 수 없는 경우 `Claude`만 작성합니다.
  * 모델 ID를 Claude 모델과 일치시킬 수 없는 경우(예: 사용자 정의 [`ANTHROPIC_BASE_URL`](/docs/ko/env-vars)을 통해 제공되는 타사 모델) `Claude Code`를 작성합니다.

이 예제는 기본 트레일러를 사용자 정의 라인 및 사용자 정의 `Co-Authored-By` 트레일러로 바꿉니다:

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

Claude Code가 풀 요청 설명에 추가하는 속성 텍스트를 설정합니다. 풀 요청 속성을 숨기려면 빈 문자열로 설정합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열
* **기본값**: 설정되지 않음. 따라서 Claude Code는 `🤖 Generated with [Claude Code](https://claude.com/claude-code)`를 추가합니다.

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

Claude Code가 [클라우드](/docs/ko/claude-code-on-the-web) 또는 [Remote Control](/docs/ko/remote-control) 세션에서 커밋하거나 풀 요청을 열 때 claude.ai 세션 링크를 추가할지 여부를 선택합니다. Claude Code는 커밋에 `Claude-Session` 트레일러로 링크를 추가하고 풀 요청 설명에 링크로 추가합니다. 링크를 생략하려면 `false`로 설정합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: Claude Code는 클라우드 또는 Remote Control 세션에서 커밋하거나 풀 요청을 열 때 claude.ai 세션 링크를 추가합니다.
  * `false`: Claude Code는 링크를 생략합니다.
* **기본값**: `true`

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
  훅과 자동화
</h2>

훅을 등록하고, 실행할 훅을 제한하며, 워크플로우를 제어합니다. 훅 이벤트 및 페이로드는 [훅 참조](/docs/ko/hooks)를 참조하십시오.

<h3 id="allowedhttphookurls">
  `allowedHttpHookUrls`
</h3>

[HTTP 훅](/docs/ko/hooks#http-hook-fields)이 대상으로 할 수 있는 URL을 제한합니다. 이 키를 정의하면 Claude Code는 URL이 패턴 중 하나와 일치하는 경우에만 HTTP 훅을 실행하고 나머지는 실행하지 않고 차단합니다. 빈 배열은 모든 HTTP 훅을 차단합니다.

* **범위**: [`모든 파일`](#scopes). 배열은 설정 파일 전체에서 병합됩니다.
* **유형**: URL 패턴 배열, `*`를 와일드카드로 사용
* **기본값**: 설정되지 않음, 따라서 모든 URL이 허용됨

이 예제는 `https://hooks.example.com/` 아래의 모든 URL과 모든 `http://localhost` URL을 허용합니다:

```json settings.json theme={null}
{
  "allowedHttpHookUrls": ["https://hooks.example.com/*", "http://localhost:*"]
}
```

호스트명 일치는 대소문자를 구분하지 않으며 정규화된 도메인 이름을 표시하는 후행 점이 있는 `hooks.example.com.`을 DNS가 처리하는 방식과 동일하게 `hooks.example.com`으로 취급합니다. 허용 목록은 관리되는 설정을 포함한 모든 소스의 훅에 적용됩니다.

<h3 id="allowmanagedhooksonly">
  `allowManagedHooksOnly`
</h3>

훅 실행을 조직이 배포하는 훅으로 제한합니다.

* **범위**: [`관리됨`](#scopes)
* **유형**: 부울
  * `true`: 관리되는 훅만 실행되며, Agent SDK 훅 및 관리되는 설정이 강제로 활성화하는 플러그인의 훅도 실행됩니다. [`allowManagedHooksOnly`에서 실행되는 항목](#what-runs-under-allowmanagedhooksonly)을 참조하십시오.
  * `false`: 모든 설정 범위 및 플러그인의 훅이 실행됨
* **기본값**: 설정되지 않음, 따라서 모든 설정 범위 및 플러그인의 훅이 실행됨

```json managed-settings.json theme={null}
{
  "allowManagedHooksOnly": true
}
```

<h4 id="what-runs-under-allowmanagedhooksonly">
  `allowManagedHooksOnly`에서 실행되는 항목
</h4>

이를 `true`로 설정하면 Claude Code는 로드되는 훅 및 훅과 유사한 명령을 변경합니다:

* **관리되는 훅 및 SDK 훅 실행**: 관리되는 설정의 훅 및 [Agent SDK](/docs/ko/agent-sdk/overview)가 프로세스에 등록하는 훅
* **강제 활성화된 플러그인 훅 실행**: 관리되는 설정이 [`enabledPlugins`](#enabledplugins)를 통해 강제로 활성화하는 플러그인의 훅. Claude Code는 전체 `plugin@marketplace` ID와 일치하므로 다른 마켓플레이스의 동일한 이름의 플러그인은 차단된 상태로 유지됩니다. 이를 통해 조직 마켓플레이스를 통해 검증된 훅을 배포하면서 다른 모든 것을 차단할 수 있습니다.
* **다른 모든 것은 차단됨**: 사용자, 프로젝트 및 로컬 훅, 다른 플러그인의 훅, 에이전트 프론트매터에 선언된 훅
* **명령 소스 플러그인 비활성화**: Claude Code는 또한 [`disableCommandPluginSources`](#disablecommandpluginsources)를 명시적으로 `false`로 설정하지 않는 한 [`command` 소스](/docs/ko/plugin-marketplaces#command-sources)가 있는 플러그인(관리되는 `enabledPlugins`에서 강제로 활성화된 플러그인 포함)을 비활성화합니다.
* **마켓플레이스 `headersHelper` 명령 차단**: Claude Code는 또한 [`disableCommandPluginSources`](#disablecommandpluginsources)가 명시적으로 `false`로 설정되지 않는 한 마켓플레이스 [`headersHelper` 명령](/docs/ko/plugin-marketplaces#authenticate-archive-downloads)을 차단합니다. 단, 관리되는 설정 자체가 선언하는 마켓플레이스는 제외됩니다. Claude Code v2.1.238 이상이 필요합니다.
* **상태 줄 및 파일 제안이 관리되는 설정으로 좁혀짐**: Claude Code는 [상태 줄 및 파일 제안 게이트](#status-line-and-file-suggestion-gates)를 따르면서 관리되는 설정에서만 [`statusLine`](/docs/ko/statusline), [`fileSuggestion`](#filesuggestion), [`subagentStatusLine`](/docs/ko/statusline#subagent-status-lines)을 읽습니다.

[`/goal`](/docs/ko/goal) 명령은 이 키가 설정된 동안 실행할 수 없습니다. 훅에 따라 달라지기 때문입니다.

<h3 id="disableallhooks">
  `disableAllHooks`
</h3>

[훅](/docs/ko/hooks#disable-or-remove-hooks), 모든 사용자 정의 [상태 줄](/docs/ko/statusline), 모든 사용자 정의 [파일 제안](#filesuggestion) 명령을 끕니다. 설정에서 삭제하지 않고 이 모든 것을 일시적으로 끄는 데 사용합니다.

* **범위**: [`모든 파일`](#scopes). 관리되는 설정만 관리되는 훅을 비활성화할 수 있습니다.
* **유형**: 부울
  * `true`: Claude Code는 훅, 모든 사용자 정의 상태 줄, 모든 사용자 정의 파일 제안 명령을 끕니다.
  * `false`: 훅, 상태 줄, 파일 제안 명령이 실행됨
* **기본값**: 설정되지 않음, 따라서 훅이 실행됨

```json settings.json theme={null}
{
  "disableAllHooks": true
}
```

범위는 키를 포함하는 파일에 따라 달라집니다:

* **관리되는 설정에서**: Claude Code는 관리되는 훅을 포함한 모든 구성된 훅을 비활성화하고 [Agent SDK](/docs/ko/agent-sdk/overview)가 프로세스에 등록하는 훅을 계속 실행합니다.
* **다른 설정 파일에서**: Claude Code는 사용자, 프로젝트, 로컬 및 플러그인 훅을 비활성화합니다. 관리되는 훅, Agent SDK 훅, 관리되는 [`enabledPlugins`](#enabledplugins)에서 강제로 활성화된 플러그인의 훅은 계속 실행됩니다.

관리되는 설정이 이 키를 설정할 때 Agent SDK 훅을 실행 상태로 유지하려면 Claude Code v2.1.242 이상이 필요합니다.

[`/goal`](/docs/ko/goal) 명령은 훅이 비활성화된 동안 실행할 수 없으며, `/hooks` 메뉴는 훅 대신 공지를 표시합니다.

<h4 id="status-line-and-file-suggestion-gates">
  상태 줄 및 파일 제안 게이트
</h4>

Claude Code는 `statusLine`, `fileSuggestion`, `subagentStatusLine`에 대해 다음 순서로 두 가지 결정을 내립니다:

* **완전히 끔**: 관리되는 설정이 `disableAllHooks`를 설정하거나, 폴더가 [설정 파일의 훅과 동일한 워크스페이스 신뢰 규칙](/docs/ko/permissions#what-runs-before-you-trust-a-folder) 아래에서 신뢰되지 않을 때
* **관리되는 설정으로 좁혀짐**: [`allowManagedHooksOnly`](#allowmanagedhooksonly)가 설정되었을 때, `disableAllHooks`가 [설정 우선순위](/docs/ko/hooks#disable-or-remove-hooks)가 적용된 후 관리되는 설정 외부에서 `true`일 때, 또는 `--safe-mode`로 Claude Code를 시작할 때

좁혀짐 상태에서 Claude Code는 배포된 관리되는 값이 있으면 실행합니다. 그렇지 않으면 경고 없이 값을 건너뜁니다: 상태 줄이 비활성화되고 `@` 자동 완성은 기본 제공 파일 제안으로 돌아갑니다.

<h3 id="disableworkflows">
  `disableWorkflows`
</h3>

[동적 워크플로우](/docs/ko/workflows#turn-workflows-off)와 관리되는 설정을 통한 조직과 같이 설정이 도달하는 모든 사람을 위한 번들 워크플로우 명령을 끕니다. 자신을 위해서만 워크플로우를 켜거나 끄려면 [`enableWorkflows`](#enableworkflows)를 대신 사용하십시오. `/config`의 **동적 워크플로우** 토글이 사용자 설정에 기록합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: Claude Code는 설정이 도달하는 모든 사람을 위해 동적 워크플로우와 번들 워크플로우 명령을 끕니다.
  * `false`: 설정되지 않은 것과 동일합니다. 워크플로우가 켜져 있는지 여부는 [`enableWorkflows`](#enableworkflows)와 계획의 기본값을 따릅니다.
* **기본값**: `false`
* **세션별 재정의**: [`CLAUDE_CODE_DISABLE_WORKFLOWS`](/docs/ko/env-vars)는 한 세션 동안 워크플로우를 끕니다. 둘 중 하나가 워크플로우를 끄면 다른 하나는 다시 켤 수 없습니다.

```json settings.json theme={null}
{
  "disableWorkflows": true
}
```

<h3 id="enableworkflows">
  `enableWorkflows`
</h3>

계획의 기본값이 원하는 것이 아닐 때 자신을 위해 [동적 워크플로우](/docs/ko/workflows)를 켜거나 끕니다. `/config`에 **동적 워크플로우**로 나타나며, 이는 이 키를 사용자 설정에 기록하고 계획의 기본값으로 다시 토글할 때 제거합니다. 관리되는 설정에서 모든 사람을 위해 워크플로우를 끄려면 [`disableWorkflows`](#disableworkflows)를 대신 사용하십시오.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: Claude Code는 자신을 위해 동적 워크플로우를 켭니다.
  * `false`: Claude Code는 자신을 위해 동적 워크플로우를 끕니다.
* **기본값**: 설정되지 않음, 따라서 워크플로우는 켜져 있습니다. Pro 계획에서는 꺼져 있습니다.
* **세션별 재정의**: [`CLAUDE_CODE_DISABLE_WORKFLOWS`](/docs/ko/env-vars)는 한 세션 동안 워크플로우를 끕니다. `true`는 설정된 동안 워크플로우를 다시 켤 수 없습니다.

```json settings.json theme={null}
{
  "enableWorkflows": true
}
```

[`disableWorkflows`](#disableworkflows)와 조직의 워크플로우 정책도 우선순위를 가집니다: `enableWorkflows: true`는 어떤 소스가 워크플로우를 끄는 동안 워크플로우를 다시 켤 수 없습니다. Claude Code는 사용자 설정 이외의 소스가 `enableWorkflows`를 설정하거나 `disableWorkflows`를 `true`로 설정하는 동안 `/config` 행을 숨깁니다.

<h3 id="hooks">
  `hooks`
</h3>

Claude Code의 수명 주기의 특정 지점(예: 도구 호출 전 또는 세션 시작 시)에서 [훅](/docs/ko/hooks)으로 자신의 명령, 프롬프트, 에이전트, HTTP 요청 또는 MCP 도구를 실행합니다. [훅 참조](/docs/ko/hooks#hook-events)는 모든 이벤트, 페이로드 및 종료 코드를 나열합니다. 각 이벤트는 매처 그룹 목록에 매핑되고, 각 그룹은 매처가 적용될 때 실행할 핸들러를 나열합니다.

* **범위**: [`모든 파일`](#scopes). 훅은 서로 대체하지 않고 파일 전체에서 병합되며, 관리되는 설정의 훅은 다른 파일에서 제거할 수 없습니다.
* **유형**: [훅 이벤트](/docs/ko/hooks#hook-events)로 키가 지정된 객체. 각 값은 `"command"`, `"prompt"`, `"agent"`, `"http"` 또는 `"mcp_tool"`의 `type`을 가진 `hooks` 항목이 있는 `{ "matcher", "hooks" }` 그룹의 배열입니다.
* **기본값**: 설정되지 않음, 따라서 훅이 실행되지 않음

이 예제는 모든 Bash 도구 호출 전에 스크립트를 실행합니다:

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

모든 이벤트, 매처 패턴 및 핸들러 필드는 [훅 참조](/docs/ko/hooks#configuration)를 참조하십시오. 훅을 끄려면 [`disableAllHooks`](#disableallhooks)를 참조하십시오. 훅을 조직이 배포하는 것으로 제한하려면 [`allowManagedHooksOnly`](#allowmanagedhooksonly)를 참조하십시오.

<h3 id="httphookallowedenvvars">
  `httpHookAllowedEnvVars`
</h3>

[HTTP 훅](/docs/ko/hooks#http-hook-fields)은 환경 변수의 값을 요청 헤더(예: `Authorization: Bearer $HOOK_TOKEN` 헤더)에 넣을 수 있지만, 훅이 자신의 `allowedEnvVars`에 나열하는 변수에만 해당됩니다. 이 키는 모든 HTTP 훅에 대해 해당 목록에 외부 제한을 설정합니다: 훅은 자신의 `allowedEnvVars`와 이 키 모두에서 이름을 지정한 경우에만 변수를 사용할 수 있습니다. 훅의 정의가 요청하더라도 훅이 읽으면 안 되는 비밀을 읽지 못하도록 하는 데 사용합니다.

* **범위**: [`모든 파일`](#scopes). 배열은 설정 파일 전체에서 병합됩니다.
* **유형**: 환경 변수 이름 배열
* **기본값**: 설정되지 않음, 따라서 각 훅의 자신의 `allowedEnvVars` 목록이 적용됨

이 예제는 헤더 보간을 `MY_TOKEN` 및 `HOOK_SECRET`으로 제한합니다:

```json settings.json theme={null}
{
  "httpHookAllowedEnvVars": ["MY_TOKEN", "HOOK_SECRET"]
}
```

허용 목록은 관리되는 설정을 포함한 모든 소스의 훅에 적용됩니다.

<h3 id="workflowkeywordtriggerenabled">
  `workflowKeywordTriggerEnabled`
</h3>

프롬프트에서 키워드 `ultracode`를 입력하면 [동적 워크플로우](/docs/ko/workflows#ask-for-a-workflow-in-your-prompt)가 트리거되는지 선택합니다. 단어를 입력하지 않고 입력하려면 `false`로 설정합니다.

* **범위**: [`모든 파일`](#scopes). `/config`에 **Ultracode 키워드 트리거**로 나타납니다.
* **유형**: 부울
  * `true`: 프롬프트에서 `ultracode`를 입력하면 동적 워크플로우가 트리거됨
  * `false`: 단어를 입력하지 않고 입력할 수 있음
* **기본값**: `true`

```json settings.json theme={null}
{
  "workflowKeywordTriggerEnabled": false
}
```

`ultracode` 노력 설정, `/workflows`, 저장된 워크플로우 명령은 영향을 받지 않습니다.

<h3 id="workflowsizeguideline">
  `workflowSizeGuideline`
</h3>

작성하는 동적 워크플로우에서 Claude가 목표로 하는 [에이전트 수](/docs/ko/workflows#set-a-size-guideline)를 설정합니다. Claude Code는 값을 Claude에 조언으로 보냅니다. 강제 상한이 아닙니다: `"small"`은 5개 미만의 에이전트를 요청하고, `"medium"`은 10개 미만, `"large"`는 50개 미만입니다. 워크플로우가 소비하는 것을 제한하려면 `"small"`을 선택합니다. Claude Code v2.1.219 이상이 필요합니다.

* **범위**: [`모든 파일`](#scopes). 여기의 값은 `/config`의 **동적 워크플로우 크기** 선택보다 우선순위를 가집니다. Claude Code는 이를 `~/.claude.json`에 저장하고, 설정 파일이 키를 설정하는 동안 해당 행을 숨깁니다.
* **유형**: 문자열, 다음 중 하나:
  * `"unrestricted"`: 지침 없음, 따라서 Claude는 워크플로우를 작업에 맞게 크기 조정
  * `"small"`: Claude는 5개 미만의 에이전트를 목표로 함
  * `"medium"`: Claude는 10개 미만의 에이전트를 목표로 함
  * `"large"`: Claude는 50개 미만의 에이전트를 목표로 함
* **기본값**: `"medium"`, 또는 Claude Code v2.1.271 이상에서 Pro 계획에 로그인했을 때 `"small"`

```json settings.json theme={null}
{
  "workflowSizeGuideline": "small"
}
```

Claude Code v2.1.219 이상이 필요합니다. v2.1.202부터 v2.1.218까지는 대신 `/config`에서 지침을 설정합니다.

<span id="plugin-configuration" />

<span id="manage-plugins" />

<span id="plugin-settings" />

<h2 id="plugins-and-skills">
  플러그인 및 스킬
</h2>

플러그인을 활성화하고, 마켓플레이스를 등록하고, 조직이 허용하는 플러그인 소스를 제한하고, 로드되는 스킬을 제어합니다. 플러그인 설치 및 빌드에 대해서는 [플러그인](/docs/ko/plugins)을 참조하십시오.

<h3 id="disablebundledskills">
  `disableBundledSkills`
</h3>

Claude Code에 포함된 [스킬](/docs/ko/skills) 및 워크플로우를 끕니다. Claude Code는 번들된 스킬과 워크플로우를 완전히 제거하는 한편, `/init`과 같은 기본 제공 명령어는 입력 가능하지만 모델에서 숨겨집니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: Boolean
  * `true`: Claude Code는 번들된 스킬과 워크플로우를 제거하고 `/init`과 같은 기본 제공 명령어를 모델에서 숨깁니다
  * `false`: 번들된 스킬이 로드됩니다
* **기본값**: 설정되지 않음, 따라서 번들된 스킬이 로드됩니다
* **세션별 재정의**: [`CLAUDE_CODE_DISABLE_BUNDLED_SKILLS`](/docs/ko/env-vars)를 `1`로 설정하면 한 세션 동안 번들된 스킬이 꺼집니다. 둘 중 하나가 스킬을 끄면 다른 하나는 다시 켤 수 없습니다

```json settings.json theme={null}
{
  "disableBundledSkills": true
}
```

플러그인, `.claude/skills/`, `.claude/commands/`의 스킬은 영향을 받지 않습니다. `/doctor`는 기본 제공 명령어처럼 입력 가능합니다. 이를 숨기려면 대신 [`DISABLE_DOCTOR_COMMAND`](/docs/ko/env-vars)를 설정하십시오.

<h3 id="disableskillshellexecution">
  `disableSkillShellExecution`
</h3>

[스킬](/docs/ko/skills) 및 사용자, 프로젝트, 플러그인 또는 추가 디렉터리 소스의 사용자 정의 명령어에서 `` !`...` `` 및 ` ```! ` 블록에 대한 인라인 셸 실행을 끕니다. Claude Code는 각 명령어를 실행하는 대신 `[shell command execution disabled by policy]`로 바꿉니다.

* **범위**: [`모든 파일`](#scopes). 관리되는 설정의 `true`는 다른 곳의 `false`로 재정의될 수 없습니다.
* **유형**: Boolean
  * `true`: Claude Code는 각 인라인 셸 명령어를 실행하는 대신 `[shell command execution disabled by policy]`로 바꿉니다
  * `false`: 인라인 셸이 실행됩니다
* **기본값**: 설정되지 않음, 따라서 인라인 셸이 실행됩니다

```json settings.json theme={null}
{
  "disableSkillShellExecution": true
}
```

번들된 스킬과 관리되는 설정을 통해 배포된 스킬은 영향을 받지 않습니다.

<h3 id="skilloverrides">
  `skillOverrides`
</h3>

[스킬](/docs/ko/skills#override-skill-visibility-from-settings)의 `SKILL.md`를 편집하지 않고 숨기거나 축소합니다. Claude Code는 각 스킬 이름 아래의 값을 Claude가 보는 스킬 목록과 `/` 자동 완성에 적용합니다.

* **범위**: [`모든 파일`](#scopes). `/skills` 메뉴는 `.claude/settings.local.json`에 씁니다.
* **유형**: 스킬 이름을 다음 중 하나로 매핑하는 객체:
  * `"on"`: Claude가 스킬을 보고 `/name`을 입력할 수 있습니다
  * `"name-only"`: Claude가 설명 없이 스킬을 이름으로만 봅니다
  * `"user-invocable-only"`: Claude가 스킬을 보지 못하지만 여전히 `/name`을 입력할 수 있습니다
  * `"off"`: Claude가 스킬을 보지 못하고 `/name`이 자동 완성에서 숨겨집니다
* **기본값**: 설정되지 않음, 따라서 모든 스킬이 `"on"`입니다

이 예제는 `legacy-context`를 Claude에 이름으로만 나열하고 `deploy`를 Claude 및 `/` 자동 완성에서 숨깁니다:

```json settings.json theme={null}
{
  "skillOverrides": {
    "legacy-context": "name-only",
    "deploy": "off"
  }
}
```

재정의는 플러그인 스킬에 적용되지 않으며, 이는 `/plugin`을 통해 관리합니다.

관리되는 설정 및 `--settings`로 전달된 파일에서, `/doctor`의 `checkup`과 같은 번들된 스킬의 별칭에 대한 키도 스킬에 적용됩니다. [별칭 키가 스킬 자체 이름의 키와 어떻게 결합되는지](/docs/ko/skills#override-skill-visibility-from-settings)를 참조하십시오.

<h3 id="syncclaudeaiskills">
  `syncClaudeAiSkills`
</h3>

[claude.ai 계정에서 활성화한 스킬](/docs/ko/skills#how-synced-skills-behave)의 다운로드를 끕니다. Claude Code는 [claude.ai 계정으로 로그인한 터미널 세션](/docs/ko/skills#where-synced-skills-load)에서 이를 `~/.claude/skills/synced/`로 다운로드합니다. 대화형 또는 비대화형이며, Cowork 및 클라우드 세션에서도 다운로드합니다. `false`로 설정하여 해당 다운로드를 중지하고 이미 동기화된 스킬을 로드하지 않도록 합니다. Claude Code는 `false`만 인정합니다. `true`는 설정되지 않은 것과 같으며 동기화를 켜지 않습니다.

* **범위**: [`사용자, 로컬 또는 관리됨`](#scopes), 그리고 `--settings`로 전달된 파일. 저장소는 이를 끌 수 없습니다.
* **유형**: Boolean
  * `false`: Claude Code는 동기화된 스킬 다운로드를 중지하고 `~/.claude/skills/synced/`에 있는 스킬을 로드하지 않습니다. 사용자 또는 관리되는 설정에서 이를 `~/.claude/skills/.trash/`로 이동합니다
  * `true`: 설정되지 않은 것과 같습니다
* **기본값**: 설정되지 않음, 따라서 claude.ai 계정으로 로그인한 세션은 스킬을 동기화합니다

이 예제는 머신이 계정의 스킬을 어떤 세션에서도 다운로드하지 않도록 합니다:

```json settings.json theme={null}
{
  "syncClaudeAiSkills": false
}
```

<h3 id="syncclaudeaiplugins">
  `syncClaudeAiPlugins`
</h3>

[claude.ai 계정에서 활성화한 플러그인](/docs/ko/plugins-reference#synced-plugins)의 다운로드를 끕니다. Claude Code는 claude.ai 계정으로 로그인한 터미널 세션의 시작 시 이를 `~/.claude/plugins/synced/`로 다운로드하고, Cowork 및 클라우드 세션에서도 다운로드하며, 각각을 `<name>@synced`로 로드합니다. `false`로 설정하여 해당 다운로드를 중지하고 이미 동기화된 플러그인을 로드하지 않도록 합니다. Claude Code는 `false`만 인정합니다. `true`는 설정되지 않은 것과 같으며 동기화를 켜지 않습니다. Claude Code v2.1.273 이상이 필요합니다.

* **범위**: [`사용자, 로컬 또는 관리됨`](#scopes), 그리고 `--settings`로 전달된 파일. 저장소는 이를 끌 수 없습니다.
* **유형**: Boolean
  * `false`: Claude Code는 동기화된 플러그인 다운로드를 중지하고 `~/.claude/plugins/synced/`에 있는 플러그인을 로드하지 않습니다. 사용자 또는 관리되는 설정에서 이를 `~/.claude/plugins/.trash/`로 이동합니다
  * `true`: 설정되지 않은 것과 같습니다
* **기본값**: 설정되지 않음, 따라서 claude.ai 계정으로 로그인한 세션은 플러그인을 동기화합니다

모든 동기화된 플러그인 대신 하나의 동기화된 플러그인을 끄려면 [`enabledPlugins`](#enabledplugins)에서 `"<name>@synced": false`를 설정하십시오.

이 예제는 머신이 계정의 플러그인을 어떤 세션에서도 다운로드하지 않도록 합니다:

```json managed-settings.json theme={null}
{
  "syncClaudeAiPlugins": false
}
```

<h3 id="allowedchannelplugins">
  `allowedChannelPlugins`
</h3>

[채널](/docs/ko/channels) 플러그인이 조직의 세션에 메시지를 푸시할 수 있는 플러그인을 선택합니다. 이를 설정하면 Claude Code는 기본 Anthropic 허용 목록 대신 목록을 사용합니다. 각 항목은 플러그인과 플러그인이 나오는 마켓플레이스의 이름을 지정합니다.

* **범위**: [`관리됨`](#scopes)
* **유형**: 각각 `marketplace` 및 `plugin` 문자열을 포함하는 객체의 배열입니다. 항목은 `"telegram@claude-plugins-official"`과 같은 `"plugin@marketplace"` 문자열일 수 있으며, Claude Code는 이를 동등한 객체로 취급합니다. 문자열 형식은 Claude Code v2.1.267 이상이 필요합니다. 이전 버전은 하나를 포함할 때 전체 `allowedChannelPlugins` 값을 거부합니다
* **기본값**: 설정되지 않음, 따라서 Claude Code는 기본 Anthropic 허용 목록을 사용합니다

이 예제는 채널을 켜고 공식 Anthropic 마켓플레이스의 Telegram 플러그인만 허용합니다:

```json managed-settings.json theme={null}
{
  "channelsEnabled": true,
  "allowedChannelPlugins": [
    { "marketplace": "claude-plugins-official", "plugin": "telegram" }
  ]
}
```

빈 배열은 모든 채널 플러그인을 차단합니다.

이 키는 채널이 계정에 대한 [`channelsEnabled`](#channelsenabled) 게이트를 통과한 후에 적용됩니다. Team 및 Enterprise 플랜과 관리되는 설정이 있는 Console 계정에서는 `channelsEnabled: true`를 의미합니다. [채널 플러그인 실행 제한](/docs/ko/channels#restrict-which-channel-plugins-can-run)을 참조하십시오.

<h3 id="blockedmarketplaces">
  `blockedMarketplaces`
</h3>

조직의 플러그인 마켓플레이스 소스를 차단합니다. Claude Code는 마켓플레이스 추가 및 플러그인 설치, 업데이트, 새로 고침 및 자동 업데이트 시 차단 목록을 확인하므로, 정책을 설정하기 전에 누군가 추가한 마켓플레이스는 플러그인을 가져오는 데 사용될 수 없습니다. 차단된 소스는 다운로드 전에 확인되므로 파일 시스템에 닿지 않습니다.

* **범위**: [`관리됨`](#scopes)
* **유형**: [`strictKnownMarketplaces`](#allowed-source-types)와 동일한 형식의 마켓플레이스 소스 객체 배열
* **기본값**: 설정되지 않음, 따라서 마켓플레이스가 차단되지 않습니다

이 예제는 하나의 GitHub 저장소를 마켓플레이스 소스로 차단합니다:

```json managed-settings.json theme={null}
{
  "blockedMarketplaces": [
    { "source": "github", "repo": "untrusted/plugins" }
  ]
}
```

`github` 항목은 [소유자 와일드카드 형식](#owner-wildcards) `"owner/*"`을 사용하여 해당 GitHub 소유자 아래의 모든 저장소를 차단할 수 있으며, 이는 Claude Code v2.1.223 이상이 필요합니다. `{ "source": "skills-dir" }`을 추가하여 Claude Code가 `~/.claude/skills/`에서 [`@skills-dir` 플러그인](/docs/ko/plugins-reference#skills-directory-plugins)을 로드하지 않도록 하면서 마켓플레이스를 제한하지 않습니다. [관리되는 마켓플레이스 제한](/docs/ko/plugin-marketplaces#managed-marketplace-restrictions)을 참조하십시오.

<h3 id="channelsenabled">
  `channelsEnabled`
</h3>

조직의 [채널](/docs/ko/channels)을 허용합니다. claude.ai Team 및 Enterprise 플랜에서 Claude Code는 이를 `true`로 설정할 때까지 채널을 차단합니다. API 키로 인증하는 [Anthropic Console](/docs/ko/authentication#claude-console-authentication) 계정의 경우 채널이 기본적으로 허용됩니다. 조직이 관리되는 설정을 배포하는 경우 Claude Code는 이 키를 `true`로 설정할 때까지 해당 계정의 채널을 차단합니다.

* **범위**: [`관리됨`](#scopes)
* **유형**: Boolean
  * `true`: Claude Code는 조직의 채널을 허용합니다
  * `false`: 설정되지 않은 것과 같습니다. 채널이 차단되는지 여부는 기본값이 말하는 대로 플랜에 따라 다릅니다
* **기본값**: 설정되지 않음. 채널은 Team 및 Enterprise 플랜과 관리되는 설정이 있는 Console 계정에서 차단되고, Pro 및 Max 플랜과 관리되는 설정이 없는 Console 계정에서 허용됩니다

```json managed-settings.json theme={null}
{
  "channelsEnabled": true
}
```

활성화된 후 플러그인이 채널로 등록할 수 있는 것을 제한하려면 [`allowedChannelPlugins`](#allowedchannelplugins)를 설정하십시오. [엔터프라이즈 제어](/docs/ko/channels#enterprise-controls)를 참조하십시오.

<h3 id="disablecommandpluginsources">
  `disableCommandPluginSources`
</h3>

[`command` 플러그인 소스](/docs/ko/plugin-marketplaces#command-sources)를 차단합니다. 이는 사용자의 머신에서 마켓플레이스가 선언한 명령어를 실행하여 플러그인을 설치합니다. 이를 `true`로 설정하면 Claude Code는 명령어를 실행하지 않고, 명령어 소스 플러그인을 설치 또는 업데이트하지 않으며, 이미 설치된 플러그인 로드를 중지합니다. `false`로 설정하여 명시적으로 허용합니다. 명령어 소스를 차단할 때마다, `true`로 설정하든 [`allowManagedHooksOnly`](#allowmanagedhooksonly) 아래에서 설정되지 않은 상태로 두든, 마켓플레이스 [`headersHelper` 명령어](/docs/ko/plugin-marketplaces#authenticate-archive-downloads)도 차단합니다. 단, 관리되는 설정 자체가 선언하는 마켓플레이스는 제외합니다. Claude Code v2.1.229 이상이 필요하며, `headersHelper` 차단은 v2.1.238 이상이 필요합니다.

* **범위**: [`관리됨`](#scopes)
* **유형**: Boolean
  * `true`: Claude Code는 마켓플레이스가 선언한 명령어를 실행하지 않고, 명령어 소스 플러그인을 설치 또는 업데이트하지 않으며, 이미 설치된 플러그인 로드를 중지합니다
  * `false`: Claude Code는 명령어 소스 플러그인을 명시적으로 허용합니다
* **기본값**: 설정되지 않음, 따라서 Claude Code는 [`allowManagedHooksOnly`](#allowmanagedhooksonly)를 따릅니다. 훅 실행을 관리되는 설정으로 제한하는 조직은 명령어 소스도 비활성화됩니다

```json managed-settings.json theme={null}
{
  "disableCommandPluginSources": true
}
```

Claude Code v2.1.229 이상이 필요합니다.

<h3 id="pluginsuggestionmarketplaces">
  `pluginSuggestionMarketplaces`
</h3>

상황별 설치 제안으로 나타날 수 있는 플러그인의 마켓플레이스 이름을 지정합니다. 스피너 팁과 `/plugin` **Discover** 탭의 상단에 고정됩니다. 기본 제공 퍼스트 파티 프론트엔드 디자인 팁은 영향을 받지 않습니다. 제안은 각 플러그인의 마켓플레이스 항목의 `relevance` 선언에서 나옵니다.

* **범위**: [`관리됨`](#scopes)
* **유형**: 마켓플레이스 이름의 배열
* **기본값**: 설정되지 않음, 따라서 마켓플레이스가 선언한 제안이 표시되지 않습니다

```json managed-settings.json theme={null}
{
  "pluginSuggestionMarketplaces": ["acme-corp-plugins"]
}
```

이름은 마켓플레이스가 머신에 등록되고 등록된 소스가 동일한 관리되는 설정에서도 선언될 때만 적용됩니다. 해당 이름의 [`extraKnownMarketplaces`](#extraknownmarketplaces) 항목 또는 [`strictKnownMarketplaces`](#strictknownmarketplaces)의 항목으로 선언됩니다. Claude Code는 허용 목록 이름 아래의 다른 소스에서 등록된 마켓플레이스를 무시합니다. 공식 마켓플레이스는 소스 요구 사항에서 제외됩니다. 이름만 허용 목록에 추가하면 충분합니다. 해당 이름은 공식 Anthropic 소스에서만 등록할 수 있기 때문입니다. [컨텍스트별 플러그인 제안](/docs/ko/plugin-relevance)을 참조하십시오.

<h3 id="plugintrustmessage">
  `pluginTrustMessage`
</h3>

설치 전에 Claude Code가 표시하는 플러그인 신뢰 경고에 조직의 고유한 텍스트를 추가합니다. 예를 들어 내부 마켓플레이스의 플러그인이 검증되었음을 확인합니다.

* **범위**: [`관리됨`](#scopes)
* **유형**: 문자열
* **기본값**: 설정되지 않음, 따라서 Claude Code는 표준 경고만 표시합니다

```json managed-settings.json theme={null}
{
  "pluginTrustMessage": "All plugins from our marketplace are approved by IT"
}
```

<h3 id="strictknownmarketplaces">
  `strictKnownMarketplaces`
</h3>

조직의 사람들이 플러그인을 추가하고 설치할 수 있는 플러그인 마켓플레이스 소스를 제한합니다. Claude Code는 마켓플레이스 추가 및 플러그인 설치, 업데이트, 새로 고침 및 자동 업데이트 시 허용 목록을 적용합니다. 네트워크 또는 파일 시스템 작업 전에 적용되므로, 정책을 설정하기 전에 누군가 추가한 마켓플레이스는 소스가 더 이상 일치하지 않으면 플러그인을 가져오는 데 사용될 수 없습니다. 차단된 사용자는 관리되는 정책의 이름을 지정하는 오류를 봅니다.

* **범위**: [`관리됨`](#scopes)
* **유형**: 마켓플레이스 소스 객체의 배열. [허용되는 소스 유형](#allowed-source-types)을 참조하십시오
* **기본값**: 설정되지 않음, 따라서 사용자는 모든 마켓플레이스를 추가할 수 있습니다. 빈 배열은 공식 Anthropic 마켓플레이스를 포함한 모든 마켓플레이스 소스를 차단하는 완전한 잠금입니다

이 예제는 두 개의 GitHub 저장소를 허용합니다. 하나는 `v2.0` ref에 고정되고 하나는 호스팅된 `marketplace.json` URL입니다:

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/approved-plugins" },
    { "source": "github", "repo": "acme-corp/security-tools", "ref": "v2.0" },
    { "source": "url", "url": "https://plugins.example.com/marketplace.json" }
  ]
}
```

이 키를 `allowedMarketplaces`로도 쓸 수 있습니다. [마켓플레이스 키 별칭](#marketplace-key-aliases)은 Claude Code가 별칭을 어떻게 취급하는지와 어느 버전이 이를 수용하는지 설명합니다. 이 키는 정책 게이트입니다. 사용자가 추가할 수 있는 것을 제어하지만 아무것도 등록하지 않습니다. 제한 및 사전 등록을 한 파일에서 수행하려면 [`extraKnownMarketplaces`와 결합](#combine-with-extraknownmarketplaces)을 참조하십시오. 사용자 대면 보기는 [관리되는 마켓플레이스 제한](/docs/ko/plugin-marketplaces#managed-marketplace-restrictions)을 참조하십시오.

<h4 id="allowed-source-types">
  허용되는 소스 유형
</h4>

아래의 각 항목은 소스 유형당 하나의 허용 목록 항목과 이를 수용하는 필드를 보여줍니다. 대부분의 유형은 정확히 일치합니다. `hostPattern` 및 `pathPattern`은 정규식으로 일치하고, `github` 항목은 [소유자 와일드카드](#owner-wildcards)를 사용할 수 있습니다.

| 소스            | 예제 항목                                                                                                                           | 필드                                                             |
| :------------ | :------------------------------------------------------------------------------------------------------------------------------ | :------------------------------------------------------------- |
| `github`      | `{ "source": "github", "repo": "acme-corp/plugins", "ref": "main", "path": "marketplace" }`                                     | `repo` 필수; `ref`는 분기 또는 태그; `path`는 하위 디렉터리                    |
| `git`         | `{ "source": "git", "url": "https://gitlab.example.com/tools/plugins.git", "ref": "production" }`                               | `url` 필수; `ref` 및 `path`는 `github`과 동일                         |
| `url`         | `{ "source": "url", "url": "https://plugins.example.com/marketplace.json", "headers": { "Authorization": "Bearer ${TOKEN}" } }` | `url` 필수; `headers`는 인증된 액세스를 위한 HTTP 헤더 추가                    |
| `npm`         | `{ "source": "npm", "package": "@acme-corp/claude-plugins" }`                                                                   | `package` 필수, `marketplace.json`을 포함하는 npm 패키지                 |
| `file`        | `{ "source": "file", "path": "/opt/acme-corp/plugins/marketplace.json" }`                                                       | `path` 필수, `marketplace.json` 파일의 절대 경로                        |
| `directory`   | `{ "source": "directory", "path": "/opt/acme-corp/approved-marketplaces" }`                                                     | `path` 필수, `.claude-plugin/marketplace.json`을 포함하는 디렉터리의 절대 경로 |
| `hostPattern` | `{ "source": "hostPattern", "hostPattern": "^github\\.example\\.com$" }`                                                        | `hostPattern` 필수, 마켓플레이스 호스트에 대해 일치하는 정규식                      |
| `pathPattern` | `{ "source": "pathPattern", "pathPattern": "^/opt/approved/" }`                                                                 | `pathPattern` 필수, `file` 및 `directory` 소스의 `path`에 대해 일치하는 정규식 |
| `skills-dir`  | `{ "source": "skills-dir" }`                                                                                                    | 필드 없음. `~/.claude/skills/` 플러그인 스캔을 다시 선택합니다                   |

세 가지 소스 유형은 표 이상의 규칙을 수행합니다:

* **`url`**: URL 마켓플레이스는 `marketplace.json` 파일만 다운로드하고, Claude Code는 해당 서버에서 상대 경로로 플러그인 파일을 가져오지 않으므로, 플러그인은 아카이브 URL과 같은 [플러그인 소스](/docs/ko/plugin-marketplaces#plugin-sources) 이외의 상대 경로를 사용해야 합니다. 상대 경로가 있는 플러그인의 경우 Git 기반 마켓플레이스를 대신 사용하십시오. [URL 기반 마켓플레이스에서 상대 경로가 있는 플러그인 실패](/docs/ko/plugin-marketplaces#plugins-with-relative-paths-fail-in-url-based-marketplaces)를 참조하십시오.
* **`hostPattern`**: 각 저장소를 나열하지 않고 내부 GitHub Enterprise 또는 GitLab 서버의 모든 마켓플레이스를 허용하는 데 사용합니다. Claude Code는 `github` 소스를 `github.com`에 대해 일치시키고, `url` 소스에서 호스트 이름을 가져오고, [git URL](https://git-scm.com/docs/git-clone#_git_urls)의 형식에 따라 `git` 소스에서 가져옵니다:

  * `https://` 또는 `ssh://`와 같은 스키마가 있는 URL: URL의 호스트 이름입니다.
  * 스키마 없는 SSH 주소, git의 `user@host:path` 형식(예: `git@git.example.com:tools/plugins.git`): `@`와 `:` 사이의 호스트이며, git이 연결하는 호스트입니다.
  * 스키마 없는 다른 형식: 호스트 없음, 따라서 `strictKnownMarketplaces` `hostPattern` 항목이 일치하지 않습니다. `blockedMarketplaces` `hostPattern`의 경우 Claude Code는 더 넓은 형식 집합에서 호스트를 가져오므로 차단 목록 항목이 여전히 그러한 형식과 일치할 수 있습니다. v2.1.234 이전에는 `strictKnownMarketplaces` `hostPattern`도 git이 SSH 주소로 취급하지 않는 일부 형식과 일치했습니다.

  `file` 및 `directory` 소스에는 호스트가 없으며 `hostPattern` 항목과 일치하지 않습니다.
* **`pathPattern`**: 네트워크 소스에 대한 `hostPattern` 항목과 함께 파일 시스템 마켓플레이스를 허용하는 데 사용합니다. `".*"`는 모든 로컬 경로를 허용합니다. `"^/opt/approved/"`와 같은 더 좁은 패턴은 디렉터리로 제한합니다.

빈 배열이라도 모든 허용 목록은 Claude Code가 `~/.claude/skills/`에서 [`@skills-dir` 플러그인](/docs/ko/plugins-reference#skills-directory-plugins)을 로드하는 것을 중지합니다. `{ "source": "skills-dir" }` 항목을 추가하여 로드를 계속합니다. 항목은 이 키와 `blockedMarketplaces` 외부에서는 의미가 없습니다.

<h4 id="owner-wildcards">
  소유자 와일드카드
</h4>

`repo` 값이 `"<owner>/*"`인 `github` 항목은 해당 GitHub 소유자 아래의 모든 저장소와 일치합니다. 소유자 와일드카드는 Claude Code v2.1.223 이상이 필요하며 `strictKnownMarketplaces` 및 `blockedMarketplaces`에서만 작동합니다. `github` 소스가 나타나는 다른 곳(예: `extraKnownMarketplaces` 또는 `/plugin marketplace add`)에서는 `repo` 값이 단일 저장소의 이름을 지정해야 합니다. v2.1.223 이전에는 Claude Code가 항목을 문자 그대로 비교했으므로 허용 목록 항목이 저장소와 일치하지 않았고 차단 목록 항목이 아무것도 차단하지 않았습니다. 단일 저장소 항목은 모든 버전에서 적용됩니다.

이 항목은 `acme-corp` 조직의 모든 마켓플레이스 저장소를 허용합니다:

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "acme-corp/*" }
  ]
}
```

전체 저장소 이름 위치만 와일드카드일 수 있습니다. Claude Code는 `*`, `*/plugins` 또는 `acme-corp/tools-*`와 같은 항목을 문자 그대로 비교하므로 저장소와 일치하지 않습니다.

일치 규칙은 두 설정 간에 다릅니다:

| 규칙         | `strictKnownMarketplaces`                                                            | `blockedMarketplaces`                       |
| ---------- | ------------------------------------------------------------------------------------ | ------------------------------------------- |
| 일치하는 소스 철자 | `owner/repo` 형식만. 동일한 저장소를 복제하는 git URL은 일치하지 않습니다                                   | 동일한 github.com 저장소로 확인되는 git URL을 포함한 모든 철자 |
| 소유자 대소문자   | 정확한 항목 일치처럼 대소문자 구분                                                                  | 대소문자 구분 안 함                                 |
| `ref`      | 정확한 항목 규칙을 따릅니다. `ref`가 있는 항목은 정확한 ref를 가진 소스와만 일치하고, 없는 항목은 ref를 지정하지 않는 소스와만 일치합니다 | `ref` 없는 항목은 일치하는 저장소의 모든 ref를 차단합니다        |
| `path`     | 정확한 항목 규칙보다 느슨합니다. `path`가 있는 항목은 정확한 값을 요구하고, 없는 항목은 저장소 내의 모든 경로와 일치합니다            | `path` 없는 항목은 일치하는 저장소의 모든 경로를 차단합니다        |

<h4 id="exact-matching">
  정확한 일치
</h4>

소유자 와일드카드 `github` 항목과 정규식으로 일치하는 `hostPattern` 및 `pathPattern` 항목을 제외한 모든 소스 유형의 경우, Claude Code는 마켓플레이스 소스가 항목과 정확히 일치할 때만 사용자의 추가를 허용합니다. Git 기반 소스 `github` 및 `git`의 경우 정확한 일치는 선택적 필드를 포함합니다:

* `repo` 또는 `url`은 정확히 일치해야 합니다
* `ref` 필드는 정확히 일치해야 하거나 둘 다 정의되지 않아야 합니다
* `path` 필드는 정확히 일치해야 하거나 둘 다 정의되지 않아야 합니다

예를 들어 Claude Code는 아래의 각 쌍을 두 개의 다른 소스로 취급합니다:

* `{ "source": "github", "repo": "acme-corp/plugins" }` 및 `{ "source": "github", "repo": "acme-corp/plugins", "ref": "main" }`
* `{ "source": "github", "repo": "acme-corp/plugins", "path": "marketplace" }` 및 `{ "source": "github", "repo": "acme-corp/plugins" }`

<h4 id="allow-only-the-official-marketplace">
  공식 마켓플레이스만 허용
</h4>

공식 Anthropic 마켓플레이스만 허용하고 다른 것은 허용하지 않으려면 해당 저장소를 나열하십시오:

```json managed-settings.json theme={null}
{
  "strictKnownMarketplaces": [
    { "source": "github", "repo": "anthropics/claude-plugins-official" }
  ]
}
```

이 항목을 사용하면 Claude Code는 이미 등록된 공식 마켓플레이스를 사용 가능하게 유지하고, 새 머신에서는 처음 대화형으로 Claude Code를 시작할 때 마켓플레이스를 자동으로 등록합니다. 자동 등록은 일반적으로 다음을 놓칩니다:

* 머신의 첫 번째 대화형 시작 전에 실행되는 비대화형 환경입니다.
* Claude Code가 이미 마켓플레이스를 차단한 정책(예: 빈 배열 잠금) 아래에서 대화형으로 실행된 머신입니다. Claude Code는 차단된 시도를 기록하고 정책이 변경된 후 다시 시도하지 않습니다.

이러한 머신에서는 동일한 `managed-settings.json`의 [`extraKnownMarketplaces`](#extraknownmarketplaces)에 마켓플레이스를 추가하여 Claude Code가 자동으로 등록하도록 하거나 `claude plugin marketplace add anthropics/claude-plugins-official`을 실행하십시오.

<h4 id="combine-with-extraknownmarketplaces">
  `extraKnownMarketplaces`와 결합
</h4>

두 키는 다른 작업을 수행합니다. 이 표는 이들을 비교합니다:

| 측면     | `strictKnownMarketplaces` | `extraKnownMarketplaces`                   |
| ------ | ------------------------- | ------------------------------------------ |
| 목적     | 조직 정책 적용                  | 팀 편의                                       |
| 설정 파일  | 관리되는 설정만                  | 모든 설정 파일                                   |
| 동작     | 허용 목록에 없는 추가 차단           | 누락된 마켓플레이스 등록                              |
| 적용 시기  | 네트워크 및 파일 시스템 작업 전        | 사용자 또는 관리되는 설정에서 즉시; 저장소 파일의 작업 공간 신뢰 대화 후 |
| 재정의 가능 | 아니오, 최고 우선순위              | 예, 더 높은 우선순위 설정으로                          |
| 소스 형식  | 직접 소스 객체                  | 중첩된 `source` 객체가 있는 명명된 마켓플레이스             |

모든 사용자에 대해 마켓플레이스를 제한하고 사전 등록하려면 `managed-settings.json`에서 둘 다 설정하십시오:

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

`strictKnownMarketplaces`만 설정하면 사용자는 여전히 `/plugin marketplace add`로 허용된 마켓플레이스를 직접 추가할 수 있습니다. 공식 Anthropic 마켓플레이스는 Claude Code가 자동으로 등록하는 유일한 마켓플레이스이며, 허용 목록이 이를 허용할 때만 등록합니다. [공식 마켓플레이스만 허용](#allow-only-the-official-marketplace)은 놓치는 머신을 나열합니다.

<h3 id="strictpluginonlycustomization">
  `strictPluginOnlyCustomization`
</h3>

사용자 및 프로젝트 소스의 스킬, 에이전트, 훅 및 MCP 서버를 차단하여 플러그인 또는 관리되는 설정에서만 올 수 있도록 합니다. [`strictKnownMarketplaces`](#strictknownmarketplaces)와 결합하여 전체 사용자 정의 공급 체인을 제어합니다. 마켓플레이스 허용 목록은 사용자가 설치할 수 있는 플러그인을 제어합니다.

* **범위**: [`관리됨`](#scopes)
* **유형**: 모든 4가지 사용자 정의를 잠그려면 `true` 또는 `"skills"`, `"agents"`, `"hooks"`, `"mcp"`에서 잠글 종류의 이름을 지정하는 배열
* **기본값**: 설정되지 않음, 따라서 아무것도 잠기지 않습니다

이 예제는 스킬과 훅을 잠그고 에이전트와 MCP 서버는 잠금 해제된 상태로 둡니다:

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["skills", "hooks"]
}
```

아래의 4개 하위 키 항목은 각 표면이 차단하는 것과 여전히 로드되는 것을 나열합니다. Claude Code는 인식하지 못하는 표면 이름을 무시하므로 모든 클라이언트가 업데이트되기 전에 새 표면 이름을 추가할 수 있습니다.

<h3 id="strictpluginonlycustomization-skills">
  `strictPluginOnlyCustomization.skills`
</h3>

`skills` 표면을 잠급니다. Claude Code는 `~/.claude/skills/` 및 `.claude/skills/`의 스킬, `~/.claude/commands/` 및 `.claude/commands/`의 사용자 정의 명령어, `--add-dir` 디렉터리의 스킬, claude.ai 계정에서 동기화된 스킬 로드를 중지하고, 플러그인 스킬, 번들된 스킬, 관리되는 정책 디렉터리의 스킬 로드를 계속합니다.

* **범위**: [`관리됨`](#scopes)
* **유형**: [`strictPluginOnlyCustomization`](#strictpluginonlycustomization) 배열의 문자열 `"skills"`
* **기본값**: 잠기지 않음

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["skills"]
}
```

<h3 id="strictpluginonlycustomization-agents">
  `strictPluginOnlyCustomization.agents`
</h3>

`agents` 표면을 잠급니다. Claude Code는 `~/.claude/agents/` 및 `.claude/agents/`의 에이전트 로드를 중지하고, 플러그인 에이전트, 기본 제공 에이전트, 관리되는 정책 디렉터리의 에이전트 로드를 계속합니다.

* **범위**: [`관리됨`](#scopes)
* **유형**: [`strictPluginOnlyCustomization`](#strictpluginonlycustomization) 배열의 문자열 `"agents"`
* **기본값**: 잠기지 않음

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["agents"]
}
```

<h3 id="strictpluginonlycustomization-hooks">
  `strictPluginOnlyCustomization.hooks`
</h3>

`hooks` 표면을 잠급니다. Claude Code는 사용자, 프로젝트 및 로컬 `settings.json`의 훅 실행을 중지하고, 플러그인 훅과 관리되는 설정의 훅 실행을 계속합니다.

* **범위**: [`관리됨`](#scopes)
* **유형**: [`strictPluginOnlyCustomization`](#strictpluginonlycustomization) 배열의 문자열 `"hooks"`
* **기본값**: 잠기지 않음

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["hooks"]
}
```

<h3 id="strictpluginonlycustomization-mcp">
  `strictPluginOnlyCustomization.mcp`
</h3>

`mcp` 표면을 잠급니다. Claude Code는 `~/.claude.json` 및 `.mcp.json`의 MCP 서버 로드를 중지하고, 플러그인 MCP 서버, [`managed-mcp.json`](/docs/ko/managed-mcp) 서버, [`managedMcpServers`](#managedmcpservers)의 서버 로드를 계속합니다.

* **범위**: [`관리됨`](#scopes)
* **유형**: [`strictPluginOnlyCustomization`](#strictpluginonlycustomization) 배열의 문자열 `"mcp"`
* **기본값**: 잠기지 않음

```json managed-settings.json theme={null}
{
  "strictPluginOnlyCustomization": ["mcp"]
}
```

<h3 id="enabledplugins">
  `enabledPlugins`
</h3>

`plugin-name@marketplace-name`으로 키가 지정된 개별 [플러그인](/docs/ko/plugins)을 켜거나 끕니다. 모든 범위에서 항목이 없는 플러그인은 [`defaultEnabled`](/docs/ko/plugins-reference#default-enablement) 값으로 폴백됩니다. `/plugin` 또는 `claude plugin enable`로 플러그인을 활성화 또는 비활성화하면 Claude Code가 이 키를 작성합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: `plugin-name@marketplace-name`을 Boolean으로 매핑하는 객체
* **기본값**: 설정되지 않음, 따라서 각 플러그인은 `defaultEnabled` 값을 따릅니다

이 예제는 `team-tools` 마켓플레이스의 두 플러그인을 활성화하고 `personal`의 하나를 비활성화합니다:

```json settings.json theme={null}
{
  "enabledPlugins": {
    "code-formatter@team-tools": true,
    "deployment-tools@team-tools": true,
    "experimental-features@personal": false
  }
}
```

각 범위는 다른 목적을 제공합니다:

* **사용자 설정**: 개인 플러그인 기본 설정
* **프로젝트 설정**: 저장소의 모든 사람과 공유되는 플러그인
* **로컬 설정**: 머신별 재정의, Claude Code가 설정을 저장할 때 gitignored
* **관리되는 설정**: 조직 전체 정책. `false`로 설정된 플러그인은 모든 범위에서 설치가 차단되고 마켓플레이스에서 숨겨집니다

프로젝트 설정은 사용자 설정보다 우선하므로, `~/.claude/settings.json`에서 플러그인을 `false`로 설정해도 프로젝트의 `.claude/settings.json`이 활성화하는 플러그인은 비활성화되지 않습니다. 머신에서 프로젝트 활성화 플러그인을 거부하려면 대신 `.claude/settings.local.json`에서 `false`로 설정하십시오. 관리되는 설정으로 강제 활성화된 플러그인은 관리되는 설정이 로컬 설정을 재정의하므로 이 방식으로 비활성화될 수 없습니다.

GitHub 저장소 또는 npm 패키지와 같은 외부 소스의 플러그인을 프로젝트의 `.claude/settings.json`에서 활성화해도 다른 사람을 위해 설치되지 않습니다. 플러그인을 로드하는 모든 경로에서 Claude Code는 각 사용자가 [직접 설치할 때까지](/docs/ko/discover-plugins#configure-team-marketplaces) 플러그인이 설치되지 않은 것으로 보고합니다.

<h3 id="extraknownmarketplaces">
  `extraKnownMarketplaces`
</h3>

추가 플러그인 마켓플레이스를 이름으로 등록하여 저장소를 열거나 관리되는 설정이 도달하는 모든 사람이 직접 추가하지 않고도 마켓플레이스를 얻도록 합니다. Claude Code는 아직 알지 못하는 각 마켓플레이스를 등록합니다. [`enabledPlugins`](#enabledplugins)가 이름을 지정하는 플러그인이 설치되는지 여부는 플러그인의 소스와 어느 파일이 이를 활성화하는지에 따라 다릅니다. 해당 항목에는 규칙이 있습니다.

* **범위**: [`모든 파일`](#scopes). Claude Code는 저장소의 `.claude/settings.json` 또는 `.claude/settings.local.json`의 항목을 해당 폴더의 작업 공간 신뢰 대화를 수락한 후에만 인정합니다. 신뢰하지 않은 폴더(메시지 없이 `-p` 실행 포함)에서는 무시합니다.
* **유형**: 마켓플레이스 이름을 `source` 객체와 선택적 `autoUpdate` Boolean이 있는 객체로 매핑하는 객체
* **기본값**: 설정되지 않음

이 예제는 GitHub 마켓플레이스와 자체 호스팅 git URL의 마켓플레이스를 등록합니다:

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

[폴더를 신뢰하기 전에 실행되는 것](/docs/ko/permissions#what-runs-before-you-trust-a-folder)은 신뢰 게이트를 저장소가 제공할 수 있는 다른 콘텐츠와 비교합니다. 이 키를 `additionalMarketplaces`로도 쓸 수 있습니다. [마켓플레이스 키 별칭](#marketplace-key-aliases)을 참조하십시오.

`source` 옆에 `"autoUpdate": true`를 설정하여 Claude Code가 시작 후 백그라운드에서 해당 마켓플레이스를 새로 고치고 설치된 플러그인을 업데이트하도록 합니다. 생략하면 `claude-plugins-official` 및 대부분의 다른 공식 Anthropic 마켓플레이스는 `true`로 기본 설정되고, 타사 마켓플레이스는 `false`로 기본 설정됩니다. [자동 업데이트 구성](/docs/ko/discover-plugins#configure-auto-updates)을 참조하십시오.

둘 이상의 설정 파일이 동일한 이름 아래에 마켓플레이스 항목을 정의할 때 Claude Code는 [가장 높은 우선순위 파일](/docs/ko/settings#settings-precedence)의 항목을 사용합니다. 해당 항목은 낮은 우선순위 항목을 대체하고 필드를 상속하지 않으므로, 재정의는 한 파일의 `source.headers` 자격 증명을 다른 파일이 제어하는 URL과 결합할 수 없습니다. v2.1.228 이전에는 Claude Code가 같은 이름 항목을 필드별로 병합했으므로, 더 높은 우선순위 파일의 항목은 설정하지 않은 필드(다른 파일의 `headers` 포함)를 상속할 수 있었습니다.

<h4 id="marketplace-source-types">
  마켓플레이스 소스 유형
</h4>

`source` 객체는 다음 형식 중 하나를 사용합니다:

* **`github`**: GitHub 저장소, `repo` 포함
* **`git`**: 모든 git URL, `url` 포함
* **`url`**: `marketplace.json` 파일에 대한 직접 URL, `url` 및 선택적 `headers` 및 `headersHelper`(인증된 액세스용). `headersHelper`는 값이 `headers`에 나열하기에는 너무 단기인 헤더를 인쇄하는 명령어의 이름을 지정하며 Claude Code v2.1.238 이상이 필요합니다
* **`file`**: `marketplace.json` 파일에 대한 로컬 경로, `path` 포함
* **`directory`**: 로컬 파일 시스템 경로, `path` 포함, 개발 전용
* **`settings`**: 호스팅된 저장소 없이 설정 파일에 직접 선언된 인라인 마켓플레이스, `name` 및 `plugins` 포함

`git` 소스 유형은 자체 호스팅 GitLab 및 Bitbucket을 포함한 모든 git 호스팅 서비스와 함께 작동합니다. Claude Code는 해당 머신에서 `git clone`이 사용할 것과 동일한 인증으로 저장소를 복제합니다. 구성된 자격 증명 도우미 또는 SSH 키입니다. `GITHUB_TOKEN`과 같은 공급자 토큰은 이를 읽는 자격 증명 도우미를 통해서만 적용됩니다. [비공개 저장소](/docs/ko/plugin-marketplaces#private-repositories)에서 설정 세부 정보를 참조하십시오.

`github` 및 `git` 소스의 경우, Claude Code는 마켓플레이스 저장소를 추가하거나 업데이트할 때 [Git LFS](https://git-lfs.com) 콘텐츠를 다운로드하지 않습니다. LFS 추적 파일은 포인터 파일로 체크아웃되고, 추가 또는 업데이트 출력은 몇 개인지 보고합니다.

`source` 객체 내의 `skipLfs` 필드는 수용되며 효과가 없습니다. v2.1.274 이전에는 `"skipLfs": true`를 설정하지 않으면 Claude Code가 LFS 콘텐츠를 다운로드했습니다.

`url` 소스의 경우, `headers`의 자격 증명이 만료되고 명령어가 새 자격 증명을 생성해야 할 때 `source` 객체 내에서 `headersHelper`를 설정합니다. Claude Code v2.1.238 이상이 필요합니다. 명령어가 인쇄해야 하는 것과 Claude Code가 실행하는 위치는 [headersHelper 명령어 작성](/docs/ko/plugin-marketplaces#write-the-headershelper-command)을 참조하고, Claude Code가 실행하지 않는 경우는 [Claude Code가 headersHelper 명령어를 건너뛰거나 출력을 삭제할 때](/docs/ko/plugin-marketplaces#when-claude-code-skips-a-headershelper-command-or-drops-its-output)를 참조하십시오. `https://` 마켓플레이스 URL에서 `headersHelper`를 설정하면 Claude Code는 두 지점에서 명령어를 실행하여 한 실행의 출력을 최대 60초 동안 재사용합니다:

* 해당 마켓플레이스의 `marketplace.json` 각 가져오기 전(나중의 새로 고침 포함). Claude Code는 인쇄된 헤더를 해당 가져오기와 함께 보냅니다.
* 마켓플레이스 URL의 원점(동일한 스키마, 호스트 및 포트를 의미)의 각 플러그인 아카이브 다운로드 전. Claude Code는 출력을 해당 다운로드와 함께 보내고, 다른 다운로드는 헤더를 받지 않습니다.

Claude Code는 [`--add-dir`](/docs/ko/permissions#what-runs-before-you-trust-a-folder)로 추가한 디렉터리의 `.claude/settings.json` 또는 `.claude/settings.local.json`에서 설정된 모든 `headersHelper`를 무시하고, `url` 소스와 인라인 플러그인 항목 모두에서 해당 파일에 설정된 고정 `headers`만 보냅니다. [사용자가 headersHelper 명령어를 수락하는 방법](/docs/ko/plugin-marketplaces#how-users-accept-a-headershelper-command)은 다른 설정 파일을 다룹니다.

`settings` 소스에 나열된 플러그인은 GitHub 또는 npm과 같은 외부 소스를 참조해야 하며, `name`은 마켓플레이스 키와 일치해야 합니다. 여전히 각 플러그인을 `enabledPlugins`에서 별도로 활성화합니다. 이 예제는 하나의 플러그인을 인라인으로 선언합니다:

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

`source: 'settings'` 아래의 플러그인 항목이 자체 `source`가 [`archive`](/docs/ko/plugin-marketplaces#zip-archives)인 경우 아카이브 다운로드를 위해 `headers`를 설정할 수 있습니다. `headers`에 넣을 값이 단기인 경우(예: 레지스트리가 요청 시 발급하는 토큰) 대신 `headersHelper` 명령어를 설정합니다. 항목은 둘 다 설정할 수 있습니다. 두 필드 모두 Claude Code v2.1.238 이상이 필요합니다.

Claude Code는 항목의 `headers`와 명령어가 인쇄하는 것을 해당 플러그인의 아카이브 다운로드와 함께 보내고 다른 다운로드와는 함께 보내지 않습니다. Claude Code는 사용자가 [해당 플러그인 하나를 직접 설치 또는 업데이트할 때](/docs/ko/plugin-marketplaces#how-users-accept-a-headershelper-command)만 명령어를 실행합니다. 3가지 추가 규칙은 항목을 보유한 파일에 따라 다릅니다:

* **`strict`**: 마켓플레이스의 `marketplace.json`의 항목과 달리, 설정 파일의 항목은 인라인할 매니페스트 필드가 없으므로 `"strict": false`가 필요하지 않습니다. [엄격한 모드](/docs/ko/plugin-marketplaces#strict-mode)를 참조하십시오.
* **폴더 신뢰**: 프로젝트의 `.claude/settings.json` 또는 `.claude/settings.local.json`의 항목의 경우 Claude Code는 사용자가 [해당 폴더도 신뢰한](/docs/ko/permissions#what-runs-before-you-trust-a-folder) 후에만 명령어를 실행합니다.
* **헤더 필터**: Claude Code는 프로젝트의 `.claude/settings.json` 또는 `.claude/settings.local.json`의 항목에서 [요청 라우팅 및 클라이언트 ID 헤더 이름](/docs/ko/plugin-marketplaces#when-claude-code-skips-a-headershelper-command-or-drops-its-output)을 삭제합니다. 저장소가 해당 파일을 제공할 수 있기 때문입니다. Claude Code는 카탈로그 항목과 `--add-dir` 디렉터리의 설정 항목에 동일한 필터를 적용하고, 사용자 설정, `--settings` 파일 또는 관리되는 설정의 항목에는 필터를 적용하지 않습니다.

<h4 id="marketplace-key-aliases">
  마켓플레이스 키 별칭
</h4>

Claude Code v2.1.232 이상에서는 `extraKnownMarketplaces`를 `additionalMarketplaces`로, `strictKnownMarketplaces`를 `allowedMarketplaces`로 쓸 수 있습니다. Claude Code는 각 별칭을 다음과 같이 취급합니다:

* 이전 버전은 별칭을 무시하므로 혼합 Claude Code 버전의 플릿을 위한 관리되는 설정 파일과 같이 이전 버전도 읽는 파일에서 정규 철자를 유지합니다.
* 정규 키를 수용하는 모든 설정 파일에서 Claude Code는 별칭을 정규 키와 정확히 동일하게 읽습니다.
* Claude Code는 파일을 업데이트할 때 `additionalMarketplaces`를 `extraKnownMarketplaces`로 다시 쓸 수 있습니다.
* 한 파일에서 두 철자를 모두 설정하면 Claude Code는 정규 값을 사용하고 별칭을 무시합니다.

<h3 id="pluginconfigs">
  `pluginConfigs`
</h3>

플러그인의 [`userConfig`](/docs/ko/plugins-reference#user-configuration) 구성 대화에서 제공하는 비민감 답변을 플러그인 ID로 키가 지정된 상태로 저장합니다. Claude Code는 대화를 작성할 때 이 키를 사용자 설정에 작성하므로 직접 편집할 필요가 없습니다. Claude Code는 민감한 옵션을 macOS Keychain에 저장하고, Keychain이 쓰기를 거부할 때 `~/.claude/.credentials.json`으로 폴백합니다. 지원되는 키체인이 없는 플랫폼에서는 `~/.claude/.credentials.json`에 저장합니다.

* **범위**: [`사용자 또는 관리됨`](#scopes)
* **유형**: 플러그인 ID를 `options` 필드가 있는 객체로 매핑하는 객체. 각 옵션 이름을 문자열, 숫자, Boolean 또는 문자열 배열로 매핑하고, 선택적 `mcpServers` 필드는 동일한 형태의 서버별 사용자 구성 값을 보유합니다
* **기본값**: 설정되지 않음

이 예제는 `acme-tools`의 `deployer` 플러그인에 대한 `api_endpoint` 옵션을 저장합니다:

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

기본 제공 플러그인은 `@builtin` 접미사가 있는 동일한 키 아래에 옵션을 저장합니다. 예를 들어, [**프로젝트 지침**](/docs/ko/memory#choose-which-instruction-files-load) 설정은 Claude Code가 `AGENTS.md` 파일을 읽는지 여부를 제어하며 `pluginConfigs["agents-md@builtin"].options.instructionFiles`입니다.

Claude Code는 이 값을 플러그인 훅, MCP 및 LSP 구성에 대체하므로 프로젝트 및 로컬 항목을 무시합니다. 복제된 저장소는 이를 제공할 수 없습니다. v2.1.207 이전에는 프로젝트 및 로컬 설정도 읽혔습니다.

<h2 id="mcp">
  MCP
</h2>

Claude Code가 연결하는 MCP 서버와 조직이 허용하는 서버를 제어합니다. [MCP를 사용하여 외부 도구에 연결](/docs/ko/mcp) 및 [관리형 MCP 구성](/docs/ko/managed-mcp)을 참조하세요.

<h3 id="allowallclaudeaimcps">
  `allowAllClaudeAiMcps`
</h3>

Claude Code가 배포된 `managed-mcp.json`과 함께 자체적으로 가져오는 [claude.ai 커넥터](/docs/ko/mcp#use-mcp-servers-from-claude-ai)를 로드합니다. 이 키가 없으면 `managed-mcp.json`이 MCP 서버를 독점적으로 제어하고 해당 커넥터를 억제합니다.

* **범위**: [`Managed`](#scopes). 사용자는 독점적 제어가 억제한 커넥터를 다시 활성화할 수 없습니다.
* **유형**: Boolean
  * `true`: Claude Code는 배포된 `managed-mcp.json`과 함께 claude.ai 커넥터를 로드합니다
  * `false`: 배포된 `managed-mcp.json`이 MCP 서버를 독점적으로 제어하고 [Claude Code가 자체적으로 가져오는](/docs/ko/mcp#how-connectors-reach-claude-code) claude.ai 커넥터를 억제합니다
* **기본값**: `false`이므로 배포된 `managed-mcp.json`이 Claude Code가 자체적으로 가져오는 claude.ai 커넥터를 억제합니다

```json managed-settings.json theme={null}
{
  "allowAllClaudeAiMcps": true
}
```

[`allowedMcpServers`](#allowedmcpservers) 및 [`deniedMcpServers`](#deniedmcpservers)는 여전히 이 키가 로드하는 커넥터에 적용됩니다. `managed-mcp.json`을 포함하는 호스트(예: 자체 호스팅 러너)의 [클라우드 세션](/docs/ko/claude-code-on-the-web)에 전달된 커넥터는 억제된 상태로 유지됩니다. [관리형 세트와 함께 claude.ai 커넥터 허용](/docs/ko/managed-mcp#allow-claude-ai-connectors-alongside-the-managed-set)을 참조하세요.

<h3 id="allowedmcpservers">
  `allowedMcpServers`
</h3>

사용자가 추가할 수 있는 MCP 서버를 허용 목록에 추가합니다. Claude Code는 플러그인 서버, `--mcp-config`로 전달된 서버, claude.ai의 서버를 포함하여 정의된 모든 위치에서 항목과 일치하지 않는 모든 서버를 차단합니다.

Chrome의 Claude, Claude Code가 실행 중인 [VS Code](/docs/ko/vs-code#the-built-in-ide-mcp-server) 또는 [JetBrains](/docs/ko/jetbrains#the-built-in-ide-mcp-server) IDE에 연결하는 `ide` 서버, CLI 자체가 구성하는 서버와 같은 기본 제공 서버는 허용 목록에서 제외되며, 거부 목록은 여전히 이들에게 적용됩니다. 프로세스 내 `type: "sdk"` 서버는 두 목록 모두에서 제외됩니다. [세션을 시작한 앱](/docs/ko/mcp#how-connectors-reach-claude-code)이 이들을 등록합니다.

조직이 제공하는 서버도 허용 목록에서 제외되며, 거부 목록은 여전히 이들에게 적용됩니다. 이 제외는 모든 [`managedMcpServers`](#managedmcpservers) 항목과 `${VAR}` 확장을 사용하지 않는 값을 가진 모든 [`managed-mcp.json`](/docs/ko/managed-mcp#exclusive-control-with-managed-mcp-json) 항목을 포함합니다. 전체 확인 순서는 [서버 평가 방법](/docs/ko/managed-mcp#how-a-server-is-evaluated)을 참조하세요. v2.1.259 이전에는 `managed-mcp.json`의 서버도 일치해야 했습니다.

* **범위**: [`Any file`](#scopes). 모든 파일의 항목이 하나의 허용 목록으로 병합됩니다. [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly)가 설정되지 않은 경우입니다. 관리형 설정에 배포하여 적용합니다.
* **유형**: 객체 배열, 각각 정확히 하나의 키: 문자, 숫자, 하이픈, 언더스코어로 제한된 문자열인 `serverName`; 정확히 일치하는 명령 및 해당 인수의 배열인 `serverCommand`; 또는 `*` 와일드카드가 있는 URL 패턴인 `serverUrl`
* **기본값**: 설정되지 않음, 따라서 모든 서버가 허용됩니다. 빈 배열은 사용자가 추가하는 모든 서버를 차단합니다

이 예제는 나열된 `npx` 명령이 시작하는 stdio 서버만 허용합니다:

```json settings.json theme={null}
{
  "allowedMcpServers": [
    { "serverCommand": ["npx", "-y", "@modelcontextprotocol/server-filesystem"] }
  ]
}
```

[`deniedMcpServers`](#deniedmcpservers) 항목이 우선하므로 두 목록에 있는 서버는 차단됩니다. 목록에 `serverCommand` 항목이 포함되면 stdio 서버는 `serverCommand` 항목과 일치해야 하고, `serverUrl` 항목이 포함되면 원격 서버는 `serverUrl` 항목과 일치해야 합니다. `serverName` 일치는 더 이상 해당 종류의 서버를 허용하지 않습니다. [허용 목록 및 거부 목록을 사용한 정책 기반 제어](/docs/ko/managed-mcp#policy-based-control-with-allowlists-and-denylists)를 참조하세요.

<h3 id="allowmanagedmcpserversonly">
  `allowManagedMcpServersOnly`
</h3>

관리형 허용 목록을 적용되는 유일한 목록으로 만듭니다. Claude Code는 [`allowedMcpServers`](#allowedmcpservers)를 관리형 설정에서만 읽고 사용자, 프로젝트, 로컬 설정의 허용 목록을 무시합니다. [`deniedMcpServers`](#deniedmcpservers)는 여전히 모든 설정 범위에서 병합되므로 사용자는 자신을 위해 서버를 차단할 수 있습니다. 관리자는 사용자의 자체 설정이 관리형 허용 목록이 허용하는 것을 확대할 수 없도록 설정합니다.

* **범위**: [`Managed`](#scopes)
* **유형**: Boolean
  * `true`: Claude Code는 `allowedMcpServers`를 관리형 설정에서만 읽고 사용자, 프로젝트, 로컬 설정의 허용 목록을 무시합니다
  * `false`: 모든 설정 범위의 허용 목록이 병합됩니다
* **기본값**: `false`이므로 모든 설정 범위의 허용 목록이 병합됩니다

이 예제는 허용 목록을 관리형 설정으로 잠그고 `github`라는 서버만 허용합니다:

```json managed-settings.json theme={null}
{
  "allowManagedMcpServersOnly": true,
  "allowedMcpServers": [
    { "serverName": "github" }
  ]
}
```

사용자는 여전히 자신의 MCP 서버를 추가할 수 있습니다. 관리형 허용 목록과 일치하는 서버만 로드됩니다. [허용 목록을 관리형 설정만으로 제한](/docs/ko/managed-mcp#restrict-the-allowlist-to-managed-settings-only)을 참조하세요.

<h3 id="deniedmcpservers">
  `deniedMcpServers`
</h3>

특정 MCP 서버를 차단합니다. Claude Code는 플러그인 서버, `--mcp-config`로 전달된 서버, `managed-mcp.json`의 서버, [`managedMcpServers`](#managedmcpservers)의 서버, [자체적으로 가져오는](/docs/ko/mcp#how-connectors-reach-claude-code) claude.ai 커넥터를 포함하여 정의된 모든 위치에서 일치하는 서버를 로드하기를 거부합니다. 프로세스 내 `type: "sdk"` 서버는 제외됩니다. 세션을 시작한 앱이 이들을 등록합니다.

* **범위**: [`Any file`](#scopes). 모든 파일의 항목이 하나의 거부 목록으로 병합되며, [`allowManagedMcpServersOnly`](#allowmanagedmcpserversonly)는 이를 변경하지 않습니다. 관리형 설정에 배포하여 적용합니다.
* **유형**: 객체 배열, 각각 정확히 하나의 키: `"claude.ai Slack"`과 같은 claude.ai 커넥터의 표시 이름인 문자열인 `serverName`; 정확히 일치하는 명령 및 해당 인수의 배열인 `serverCommand`; 또는 `*` 와일드카드가 있는 URL 패턴인 `serverUrl`
* **기본값**: 설정되지 않음, 따라서 서버가 차단되지 않습니다. 빈 배열도 아무것도 차단하지 않습니다

```json settings.json theme={null}
{
  "deniedMcpServers": [
    { "serverName": "filesystem" }
  ]
}
```

거부 목록이 [`allowedMcpServers`](#allowedmcpservers)보다 우선하므로 두 목록에 있는 서버는 차단됩니다. [허용 목록 및 거부 목록을 사용한 정책 기반 제어](/docs/ko/managed-mcp#policy-based-control-with-allowlists-and-denylists)를 참조하세요.

<h3 id="disableclaudeaiconnectors">
  `disableClaudeAiConnectors`
</h3>

[Claude Code가 자체적으로 가져오는](/docs/ko/mcp#how-connectors-reach-claude-code) [claude.ai MCP 커넥터](/docs/ko/mcp#use-mcp-servers-from-claude-ai)를 끕니다. 따라서 이들을 가져오거나 연결하지 않습니다. 모든 설정 파일의 `true`가 적용됩니다. 체크인된 프로젝트 `.claude/settings.json`은 저장소를 해당 커넥터에서 제외할 수 있지만, 프로젝트 수준의 `false`는 사용자 또는 관리형 수준의 `true`를 재정의할 수 없습니다.

* **범위**: [`Any file`](#scopes)
* **유형**: Boolean
  * `true`: Claude Code는 해당 커넥터를 가져오거나 연결하지 않습니다
  * `false`: 설정되지 않은 것과 동일합니다. Claude Code는 다른 설정 파일이나 `ENABLE_CLAUDEAI_MCP_SERVERS`가 이들을 끄지 않는 한 커넥터를 가져옵니다
* **기본값**: `false`이므로 Claude Code는 커넥터를 가져옵니다
* **세션별 재정의**: [`ENABLE_CLAUDEAI_MCP_SERVERS`](/docs/ko/env-vars)를 `false`로 설정하면 한 세션 동안 커넥터가 꺼집니다. 둘 중 어느 것이 이들을 끄든 다른 하나는 이들을 다시 켤 수 없습니다

```json settings.json theme={null}
{
  "disableClaudeAiConnectors": true
}
```

`--mcp-config`로 명시적으로 전달하는 서버는 영향을 받지 않습니다. 모든 커넥터를 차단하는 대신 개별 커넥터를 차단하려면 [`deniedMcpServers`](#deniedmcpservers)를 사용하세요. [claude.ai 커넥터 비활성화](/docs/ko/mcp#disable-claude-ai-connectors)를 참조하세요.

<h3 id="disabledmcpjsonservers">
  `disabledMcpjsonServers`
</h3>

프로젝트의 `.mcp.json` 파일에 정의된 특정 서버를 거부하여 Claude Code가 이들을 연결하거나 승인을 요청하지 않도록 합니다. 모든 설정 파일의 거부가 적용되며, 저장소에 체크인된 프로젝트 `.claude/settings.json`도 포함됩니다.

* **범위**: [`Any file`](#scopes)
* **유형**: 문자열 배열, `.mcp.json`에 나타나는 서버 이름
* **기본값**: 설정되지 않음

```json settings.json theme={null}
{
  "disabledMcpjsonServers": ["filesystem"]
}
```

Claude Code는 승인 대화에서 서버를 거부할 때 이 키를 `.claude/settings.local.json`에 씁니다. `claude mcp get <name>`은 거부된 서버를 `✘ Rejected (see disabledMcpjsonServers in settings)`로 표시합니다. 거부는 [`enabledMcpjsonServers`](#enabledmcpjsonservers) 및 [`enableAllProjectMcpServers`](#enableallprojectmcpservers)보다 우선합니다.

<h3 id="enableallprojectmcpservers">
  `enableAllProjectMcpServers`
</h3>

프로젝트 `.mcp.json` 파일에 정의된 모든 MCP 서버를 프롬프트 없이 승인합니다. Claude Code는 승인 대화에서 모든 서버를 승인하도록 선택할 때 이 키를 `.claude/settings.local.json`에 씁니다.

* **범위**: [`Any file`](#scopes). 신뢰 대화를 수락하지 않은 폴더에서 Claude Code는 사용자 설정, 관리형 설정, `--settings`에서 이를 준수하고 공유 프로젝트 파일에서는 무시합니다. 세션 및 `claude mcp list`와 `claude mcp get`에서 모두 [프로젝트 서버 승인 및 작업 영역 신뢰](/docs/ko/mcp#project-server-approvals-and-workspace-trust)는 추적되지 않은 `.claude/settings.local.json`이 언제 계산되는지 말합니다.
* **유형**: Boolean
  * `true`: Claude Code는 프로젝트 `.mcp.json` 파일에 정의된 모든 MCP 서버를 프롬프트 없이 승인합니다
  * `false`: Claude Code는 각 서버를 승인하도록 요청합니다. 신뢰된 폴더에서 더 높은 우선순위 파일의 `false`는 더 낮은 우선순위 파일의 `true`를 재정의합니다. 신뢰하지 않은 폴더에서 준수되는 모든 파일의 `true`는 충분합니다
* **기본값**: 설정되지 않음, 따라서 Claude Code는 각 서버를 승인하도록 요청합니다

```json settings.json theme={null}
{
  "enableAllProjectMcpServers": true
}
```

[`disabledMcpjsonServers`](#disabledmcpjsonservers) 항목은 여전히 서버를 거부합니다.

<h3 id="enabledmcpjsonservers">
  `enabledMcpjsonServers`
</h3>

프로젝트 `.mcp.json` 파일에 정의된 특정 서버를 승인하여 Claude Code가 묻지 않고 이들을 연결하도록 합니다. Claude Code는 승인 대화에서 서버를 승인할 때 이 키를 `.claude/settings.local.json`에 씁니다.

* **범위**: [`Any file`](#scopes). 신뢰 대화를 수락하지 않은 폴더에서 Claude Code는 사용자 설정, 관리형 설정, `--settings`에서 이를 준수하고 공유 프로젝트 파일에서는 무시합니다. 세션 및 `claude mcp list`와 `claude mcp get`에서 모두 [프로젝트 서버 승인 및 작업 영역 신뢰](/docs/ko/mcp#project-server-approvals-and-workspace-trust)는 추적되지 않은 `.claude/settings.local.json`이 언제 계산되는지 말합니다.
* **유형**: 문자열 배열, `.mcp.json`에 나타나는 서버 이름
* **기본값**: 설정되지 않음

이 예제는 프로젝트의 `.mcp.json`에서 `memory` 및 `github` 서버를 승인합니다:

```json settings.json theme={null}
{
  "enabledMcpjsonServers": ["memory", "github"]
}
```

[`disabledMcpjsonServers`](#disabledmcpjsonservers) 항목은 여전히 서버를 거부합니다.

<h3 id="managedmcpservers">
  `managedMcpServers`
</h3>

관리형 설정에서 모든 사용자에게 원격 MCP 서버를 제공합니다. 사용자는 자신이 추가한 서버를 유지하고 제공하는 서버를 편집하거나 제거할 수 없습니다. Claude Code v2.1.259 이상이 필요합니다.

* **범위**: [`Managed`](#scopes). Claude Code는 사용자, 프로젝트, 로컬 설정에서 키를 경고와 함께 삭제하고, 타사 배포의 Claude Desktop 앱의 Code 탭이나 앱의 Cowork 세션에서는 읽지 않습니다. Claude Desktop이 해당 세션의 MCP 서버를 공급하고 잠급니다.
* **유형**: 서버 이름으로 키가 지정된 객체. 각 항목은 `http` 또는 `sse` 서버에 대한 `.mcp.json` 형태입니다. 필수 `https://` `url`, 그리고 선택적으로 `headers`, `oauth`, 기타 HTTP 및 SSE 옵션. Claude Code는 유효성 검사에 실패한 항목을 삭제하고, [항목이 포함할 수 있는 것](/docs/ko/managed-mcp#what-an-entry-can-contain)은 조건을 나열합니다
* **기본값**: 설정되지 않음, 따라서 관리형 설정은 서버를 제공하지 않습니다

이 예제는 `search`라는 HTTP 서버 하나를 제공합니다:

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

우선순위, 제공된 서버가 `managed-mcp.json` 및 허용 및 거부 목록과 결합되는 방식, 사용자가 보는 것에 대해서는 [관리형 설정을 통해 서버 제공](/docs/ko/managed-mcp#provide-servers-through-managed-settings)을 참조하세요.

<h2 id="agents-sessions-and-worktrees">
  에이전트, 세션, 및 worktrees
</h2>

기본 에이전트를 설정하고, 팀원을 제어하며 세션 간 메시징을 구성하고, worktrees를 설정합니다. [Subagents](/docs/ko/sub-agents) 및 [Worktrees](/docs/ko/worktrees)를 참조하세요.

<h3 id="agent">
  `agent`
</h3>

명명된 [subagent](/docs/ko/sub-agents#invoke-subagents-explicitly)로 메인 스레드를 실행하여 Claude Code가 해당 subagent의 시스템 프롬프트, 도구 제한 사항 및 모델을 세션에 적용하도록 합니다. 동일한 키는 `claude agents`에서 디스패치하는 세션의 기본 에이전트를 설정합니다.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, 기본 제공 또는 사용자 정의 에이전트의 이름
* **Default**: 설정되지 않음. 메인 스레드는 Claude Code의 기본 에이전트로 실행됩니다
* **Per-session overrides**: `--agent`는 한 세션에 대해 이 키보다 우선합니다

```json settings.json theme={null}
{
  "agent": "code-reviewer"
}
```

플러그인의 자체 `settings.json`도 이 키를 제공할 수 있습니다. [플러그인과 함께 기본 설정 제공](/docs/ko/plugins#ship-default-settings-with-your-plugin)을 참조하세요.

<h3 id="crosssessioninbound">
  `crossSessionInbound`
</h3>

이 세션이 [다른 Claude Code 세션에서 도착하는 메시지](/docs/ko/cross-session-messaging#control-inbound-messages)로 수행할 작업을 선택합니다. 적용되는 값이 없으면 Claude Code는 두 세션의 권한 모드 클래스에서 메시지별로 결정합니다. Claude Code v2.1.224 이상이 필요합니다.

* **Scope**: [`Any file`](#scopes). 프로젝트 또는 로컬 값은 관리되는 설정, `--settings` 플래그 또는 사용자 설정이 제공하는 값보다 더 엄격할 때만 적용됩니다.
* **Type**: string, 다음 중 하나:
  * `"accept"`: Claude Code가 메시지를 Claude에 전달합니다
  * `"hold"`: Claude Code가 메시지를 전달하지 않고 알림을 표시합니다
  * `"refuse"`: Claude Code가 메시지를 삭제합니다
* **Default**: 설정되지 않음. Claude Code는 메시지별로 결정합니다

```json settings.json theme={null}
{
  "crossSessionInbound": "hold"
}
```

Claude Code는 관리되는 설정을 먼저 읽은 다음 `--settings` 플래그, 그 다음 사용자 설정을 읽고 찾은 첫 번째 값을 적용합니다. `refuse`는 `hold`보다 더 엄격하고, `hold`는 `accept`보다 더 엄격합니다. 신뢰할 수 있는 소스 중 어느 것도 값을 설정하지 않으면 프로젝트 또는 로컬 `hold` 또는 `refuse`가 여전히 적용되어 메시지별 기본값을 대체합니다. 세션 간 메시징이 있는 세션에서 이 키는 `/config`에 **다른 세션의 메시지**로 나타나며, 이는 사용자 설정에 기록됩니다. 행에는 Claude Code v2.1.232 이상이 필요하며, Claude Code는 `--settings` 플래그 또는 관리되는 설정이 키를 설정하는 동안 이를 숨깁니다.

Claude Code는 인식하지 못하는 값을 설정할 때 [경고](/docs/ko/errors#crosssessioninbound-must-be-one-of-accept-hold-refuse)합니다. 해당 값이 사용자, 프로젝트, 로컬 또는 `--settings` 파일에 있는 동안 Claude Code는 인바운드 메시지를 보류합니다. 우선 순위를 갖는 소스가 `accept`를 설정하더라도 마찬가지입니다. 다른 소스가 설정한 `refuse`는 여전히 적용됩니다. 값을 수정하거나 제거하여 보류를 해제합니다.

인식하지 못하는 값이 [관리되는 설정](/docs/ko/managed-settings)에 있으면 Claude Code는 대신 관리자가 수정할 때까지 이를 `refuse`로 처리합니다. v2.1.248 이전에는 Claude Code가 경고 없이 인식하지 못하는 값을 무시했습니다.

<h3 id="disableagentview">
  `disableAgentView`
</h3>

[배경 에이전트 및 에이전트 보기](/docs/ko/agent-view)를 끕니다: `claude agents`, `--bg`, `/background` 및 온디맨드 감독자. [관리되는 설정](/docs/ko/managed-settings)에서 설정하여 조직에 대해 적용합니다.

* **Scope**: [`Any file`](#scopes)
* **Type**: Boolean
  * `true`: Claude Code가 `claude agents`, `--bg`, `/background` 및 온디맨드 감독자를 끕니다
  * `false`: 에이전트 보기를 사용할 수 있습니다
* **Default**: 설정되지 않음. 에이전트 보기를 사용할 수 있습니다
* **Per-session overrides**: [`CLAUDE_CODE_DISABLE_AGENT_VIEW`](/docs/ko/env-vars)는 한 세션에 대해 에이전트 보기를 끕니다. 둘 중 하나가 이를 끄면 다른 하나는 다시 켤 수 없습니다

```json settings.json theme={null}
{
  "disableAgentView": true
}
```

<h3 id="isolatepeermachines">
  `isolatePeerMachines`
</h3>

Claude의 `SendMessage`가 이 머신을 넘어 세션 중 하나에 도달하기 전에 명시적 승인을 요구합니다. [크로스 머신 메시지에 대한 승인 요구](/docs/ko/cross-session-messaging#require-approval-for-cross-machine-messages)를 참조하세요. 승인 프롬프트는 [`bypassPermissions` 모드](/docs/ko/permission-modes#skip-all-checks-with-bypasspermissions-mode)에서도 나타납니다.

* **Scope**: [`Any file`](#scopes). 모든 범위의 `true`가 적용되므로 체크인된 프로젝트 파일이 요구 사항을 켤 수 있지만 끌 수는 없습니다.
* **Type**: Boolean
  * `true`: Claude Code는 Claude의 `SendMessage`가 이 머신을 넘어 세션 중 하나에 도달하기 전에 승인을 요청합니다
  * `false`: 크로스 머신 메시지는 프롬프트를 표시하지 않습니다
* **Default**: 설정되지 않음. 크로스 머신 메시지는 프롬프트를 표시하지 않습니다

```json settings.json theme={null}
{
  "isolatePeerMachines": true
}
```

크로스 머신 `SendMessage` 승인에는 Claude Code v2.1.224 이상이 필요합니다.

<h3 id="processwrapper">
  `processWrapper`
</h3>

macOS 및 Linux에서 [Claude Code가 시작하는 배경 프로세스](/docs/ko/corporate-launcher#what-the-launcher-covers) 앞에 회사 런처 명령을 배치합니다. Claude Code는 자체 명령줄이 추가된 런처를 실행하므로 런처는 Claude Code로 exec해야 합니다. [회사 런처 뒤에서 Claude Code 실행](/docs/ko/corporate-launcher)에서 런처 계약을 참조하세요. Claude Code v2.1.210 이상이 필요합니다.

* **Scope**: [`User or managed`](#scopes)
* **Type**: string, argv 접두사로서의 런처 명령(예: 선택적 인수가 있는 절대 경로)
* **Default**: 설정되지 않음. 배경 프로세스는 래핑되지 않은 상태로 시작됩니다
* **Per-session overrides**: [`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/ko/env-vars)는 한 세션에 대해 이 키보다 우선합니다

```json settings.json theme={null}
{
  "processWrapper": "/opt/corp/launcher --profile claude"
}
```

Claude Code는 Windows에서 런처를 무시하고 모든 프로세스를 래핑되지 않은 상태로 시작합니다. Claude Code v2.1.210 이상이 필요합니다.

<h3 id="teammatemode">
  `teammateMode`
</h3>

Claude Code가 [에이전트 팀](/docs/ko/agent-teams) 팀원을 표시할 위치를 선택합니다: 메인 터미널 창 내부 또는 터미널이 지원할 때 분할 창에서. [디스플레이 모드 선택](/docs/ko/agent-teams#choose-a-display-mode)을 참조하세요.

* **Scope**: [`Any file`](#scopes). Claude Code는 또한 이전 버전에서 `~/.claude.json`에 남겨진 값을 읽습니다.
* **Type**: string, 다음 중 하나:
  * `"in-process"`: 팀원은 메인 터미널 창 내부에서 실행됩니다
  * `"auto"`: tmux 내부에서 실행 중이거나 `PATH`에 `it2`가 있거나 tmux가 설치된 iTerm2 내부에서 분할 창을 사용합니다. 그 외에는 인프로세스입니다
  * `"tmux"`: 터미널에서 감지된 tmux 또는 iTerm2를 사용하여 분할 창을 만듭니다
  * `"iterm2"`: Claude Code v2.1.186 이상에서 `it2` CLI를 통한 iTerm2 네이티브 분할 창
* **Default**: `"in-process"`
* **Per-session overrides**: `--teammate-mode`는 한 세션에 대해 이 키보다 우선합니다

```json settings.json theme={null}
{
  "teammateMode": "auto"
}
```

`iterm2` 값에는 Claude Code v2.1.186 이상이 필요합니다.

<span id="worktree-settings" />

<h3 id="worktree">
  `worktree`
</h3>

Claude Code가 `--worktree`, `EnterWorktree` 도구 및 격리된 subagents 및 배경 세션에 대해 [git worktrees](/docs/ko/worktrees)를 생성하고 관리하는 방식을 구성합니다.

* **Scope**: [`Any file`](#scopes)
* **Type**: `baseRef`, `symlinkDirectories`, `sparsePaths` 및 `bgIsolation`을 포함하는 객체
* **Default**: 설정되지 않음

이 예제는 현재 `HEAD`에서 새 worktrees를 분기하고 각 worktree에 `node_modules`를 심링크합니다:

```json settings.json theme={null}
{
  "worktree": {
    "baseRef": "head",
    "symlinkDirectories": ["node_modules"]
  }
}
```

`.env`와 같은 gitignored 파일을 새 worktrees에 복사하려면 설정 대신 프로젝트 루트에 [`.worktreeinclude` 파일](/docs/ko/worktrees#copy-gitignored-files-into-worktrees)을 추가합니다.

<h3 id="worktree-baseref">
  `worktree.baseRef`
</h3>

새 worktrees가 분기할 ref를 선택합니다. `"fresh"`는 원격과 일치하는 깨끗한 트리를 위해 `origin/<default-branch>`에서 분기합니다. `"head"`는 현재 로컬 `HEAD`에서 분기하므로 푸시되지 않은 커밋 및 기능 분기 상태가 worktree에 있습니다.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, 다음 중 하나:
  * `"fresh"`: 새 worktrees는 `origin/<default-branch>`에서 분기합니다
  * `"head"`: 새 worktrees는 푸시되지 않은 커밋을 포함하여 현재 로컬 `HEAD`에서 분기합니다
* **Default**: `"fresh"`

```json settings.json theme={null}
{
  "worktree": {
    "baseRef": "head"
  }
}
```

연결된 worktree 내부에서 `"head"`는 메인 체크아웃이 아닌 해당 worktree의 `HEAD`로 확인됩니다.

<h3 id="worktree-symlinkdirectories">
  `worktree.symlinkDirectories`
</h3>

메인 리포지토리의 디렉토리를 각 worktree에 심링크하여 디스크에 큰 디렉토리를 복제하지 않습니다.

* **Scope**: [`Any file`](#scopes)
* **Type**: 문자열 배열, 리포지토리 루트에 상대적인 디렉토리 경로
* **Default**: 설정되지 않음. Claude Code는 디렉토리를 심링크하지 않습니다

이 예제는 메인 리포지토리의 `node_modules` 및 `.cache`를 모든 새 worktree에 심링크합니다:

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

git sparse-checkout를 통해 각 worktree에서 나열된 디렉토리만 체크아웃합니다. Claude Code는 해당 디렉토리와 루트 수준 파일만 디스크에 기록하므로 대규모 monorepos에서 더 빠릅니다. [필요한 디렉토리만 체크아웃](/docs/ko/large-codebases#check-out-only-the-directories-you-need)을 참조하세요.

* **Scope**: [`Any file`](#scopes)
* **Type**: 문자열 배열, 리포지토리 루트에 상대적인 디렉토리 경로
* **Default**: 설정되지 않음. 각 worktree는 전체 트리를 체크아웃합니다

이 예제는 각 worktree에서 `packages/my-app` 및 `shared/utils`와 루트 수준 파일만 체크아웃합니다:

```json settings.json theme={null}
{
  "worktree": {
    "sparsePaths": ["packages/my-app", "shared/utils"]
  }
}
```

sparse worktree가 존재하는 동안 git은 리포지토리의 공유 `.git/config`에서 `extensions.worktreeConfig`를 활성화합니다.

<h3 id="worktree-bgisolation">
  `worktree.bgIsolation`
</h3>

[배경 세션](/docs/ko/agent-view#how-file-edits-are-isolated)이 파일 편집을 격리하는 방식을 선택합니다. `"worktree"`를 사용하면 Claude Code는 세션이 `EnterWorktree`를 호출할 때까지 메인 체크아웃에서 `Edit` 및 `Write`를 차단합니다. `"none"`을 사용하면 배경 작업이 작업 복사본을 직접 편집합니다. git worktrees가 비실용적인 리포지토리의 경우 `"none"`을 설정합니다.

* **Scope**: [`Any file`](#scopes)
* **Type**: string, 다음 중 하나:
  * `"worktree"`: Claude Code는 세션이 `EnterWorktree`를 호출할 때까지 메인 체크아웃에서 `Edit` 및 `Write`를 차단합니다
  * `"none"`: 배경 작업이 작업 복사본을 직접 편집합니다
* **Default**: `"worktree"`

```json settings.json theme={null}
{
  "worktree": {
    "bgIsolation": "none"
  }
}
```

git 리포지토리 외부에서 실패하는 [`WorktreeCreate` 훅](/docs/ko/worktrees#non-git-version-control)은 블록을 해제하여 세션이 작업 디렉토리를 제자리에서 편집할 수 있도록 합니다. 해당 해제에는 Claude Code v2.1.203 이상이 필요합니다.

<h2 id="remote-desktop-and-notifications">
  원격, 데스크톱 및 알림
</h2>

Remote Control, 클라우드 환경, 데스크톱 앱 및 Claude Code가 필요할 때 보내는 알림을 구성합니다. [Remote Control](/docs/ko/remote-control)을 참조하세요.

<h3 id="agentpushnotifenabled">
  `agentPushNotifEnabled`
</h3>

Claude가 가치 있다고 판단할 때 휴대폰으로 푸시 알림을 보낼 수 있도록 허용합니다. 예를 들어 긴 작업이 완료될 때입니다. Claude Code는 이 선택을 계정에 동기화하며, [Remote Control](/docs/ko/remote-control)이 연결되어 있을 때 푸시가 도착합니다. `/config`에서 **Claude가 결정할 때 푸시**로 표시됩니다.

* **범위**: [`Any file`](#scopes). Claude Code는 이전 버전에서 `~/.claude.json`에 남겨진 값도 읽습니다.
* **유형**: Boolean
  * `true`: Claude가 가치 있다고 판단할 때 휴대폰으로 푸시 알림을 보낼 수 있습니다
  * `false`: Claude는 해당 알림을 보내지 않습니다
* **기본값**: `false`

```json settings.json theme={null}
{
  "agentPushNotifEnabled": true
}
```

[모바일 푸시 알림](/docs/ko/remote-control#mobile-push-notifications)을 참조하세요.

<h3 id="awaysummaryenabled">
  `awaySummaryEnabled`
</h3>

몇 분 동안 터미널에서 떠난 후 돌아올 때 한 줄의 세션 요약을 표시합니다. `false`로 설정하거나 `/config`에서 **세션 요약**을 끄면 요약이 중지됩니다.

* **범위**: [`Any file`](#scopes)
* **유형**: Boolean
  * `true`: 몇 분 동안 떠난 후 돌아올 때 한 줄의 세션 요약을 볼 수 있습니다
  * `false`: Claude Code는 요약을 표시하지 않습니다
* **기본값**: 설정되지 않음, 따라서 요약은 켜져 있습니다
* **세션별 재정의**: [`CLAUDE_CODE_ENABLE_AWAY_SUMMARY`](/docs/ko/env-vars)는 이 키보다 한 세션에 대해 어느 방향이든 우선합니다

```json settings.json theme={null}
{
  "awaySummaryEnabled": false
}
```

Claude Code는 비대화형 모드에서 요약을 표시하지 않습니다.

<h3 id="disableartifact">
  `disableArtifact`
</h3>

<Warning>
  더 이상 사용되지 않으며 [`enableArtifact`](#enableartifact)로 대체되었습니다. Claude Code는 여전히 `disableArtifact: true`를 `enableArtifact: false`와 동등하게 인정하며, `disableArtifact: false`는 무시합니다.
</Warning>

대신 [`enableArtifact`](#enableartifact)를 사용하여 claude.ai에서 세션 출력을 비공개 웹 페이지로 게시하는 [Artifact](/docs/ko/artifacts) 도구를 끕니다. `/config`에서 **Artifacts** 행을 끄면 Claude Code는 `enableArtifact`를 사용자 설정에 작성하고 이 키를 지웁니다.

* **범위**: [`Any file`](#scopes)
* **유형**: Boolean
  * `true`: Claude Code는 파일이 적용되는 모든 세션에 대해 Artifact 도구를 끄고, 다른 파일은 이를 다시 켜지 않습니다. v2.1.242 이전에는 더 높은 우선순위 파일이 낮은 파일의 `true`를 재정의할 수 있었으며, 키가 잠금으로 작동하지 않았습니다
  * `false`: 무시됨; 도구를 켜진 상태로 두려면 키를 제거합니다
* **기본값**: 설정되지 않음, 따라서 도구는 계정의 [가용성](/docs/ko/artifacts#availability)을 따릅니다
* **세션별 재정의**: [`CLAUDE_CODE_DISABLE_ARTIFACT`](/docs/ko/env-vars)를 `1`로 설정하면 한 세션에 대해 도구가 꺼집니다

```json settings.json theme={null}
{
  "disableArtifact": true
}
```

[아티팩트 비활성화](/docs/ko/artifacts#disable-artifacts)는 도구를 끄는 모든 방법을 나열합니다.

<h3 id="disabledeeplinkregistration">
  `disableDeepLinkRegistration`
</h3>

Claude Code가 `claude-cli://` 프로토콜 핸들러를 운영 체제에 등록하지 않도록 중지합니다. 이는 대화형 세션의 첫 번째 프롬프트를 보낸 후에 등록됩니다. [Deep links](/docs/ko/deep-links)를 사용하면 외부 도구가 미리 채워진 프롬프트로 Claude Code 세션을 열 수 있습니다. 프로토콜 핸들러 등록이 제한되거나 별도로 관리되는 환경에서 이를 설정합니다.

* **범위**: [`Any file`](#scopes)
* **유형**: 문자열 `"disable"`
* **기본값**: 설정되지 않음, 따라서 Claude Code는 핸들러를 등록합니다

```json settings.json theme={null}
{
  "disableDeepLinkRegistration": "disable"
}
```

<h3 id="disabledesktoplocalsessions">
  `disableDesktopLocalSessions`
</h3>

개발자가 SSH를 통해 원격 머신에서 작업해야 하는 배포의 경우 [데스크톱 앱](/docs/ko/desktop#local-sessions-on-managed-devices)에서 디바이스에서 실행되는 Code 세션을 끕니다. Code 탭에서 **Local** 환경은 환경 드롭다운에 남아 있지만 회색으로 표시되고 선택할 수 없으며, 조직이 이를 끄도록 했다는 도구 설명이 표시됩니다. Windows에서는 WSL 항목도 같은 방식으로 회색으로 표시되지만, WSL 세션이 관리되는 디바이스에서 실행되는지 여부는 [별도로 관리됩니다](/docs/ko/admin-setup#wsl-sessions-in-claude-code-desktop). 새 세션은 구성된 [SSH 연결](/docs/ko/desktop#ssh-sessions)이 있으면 첫 번째로 기본값이 설정되며, 앱은 같은 머신으로의 SSH 연결을 포함하여 디바이스에서 세션을 시작하거나 재개하기를 거부합니다. 다른 호스트로의 SSH 세션 및 클라우드 세션은 영향을 받지 않습니다. 데스크톱 앱은 이 키를 읽습니다. 터미널 CLI는 무시합니다. Claude Desktop v1.37937.0 이상이 필요합니다.

* **범위**: [`Managed`](#scopes)
* **유형**: Boolean; JSON Boolean `true`만 적용됩니다
  * `true`: 데스크톱 앱은 온디바이스 Code 세션을 제공하지 않습니다. 기존 로컬 세션은 나열되지만 계속할 수 없습니다
  * `false`: 로컬 세션은 사용 가능한 상태로 유지됩니다
* **기본값**: 설정되지 않음, 따라서 로컬 세션은 사용 가능합니다

```json managed-settings.json theme={null}
{
  "disableDesktopLocalSessions": true
}
```

데스크톱 앱은 다른 값을 무시하며, Boolean이 아닌 값(예: 문자열 `"true"` 또는 `1`)도 경고를 기록합니다. [`sshConfigs`](#sshconfigs)와 함께 사용하여 사용자가 작동하는 연결에 도달하도록 하고, [`sshHostAllowlist`](#sshhostallowlist)와 함께 사용하여 도달할 수 있는 호스트를 제한합니다. [관리되는 디바이스의 로컬 세션](/docs/ko/desktop#local-sessions-on-managed-devices)을 참조하세요.

Claude Desktop은 데스크톱 구성에서 파생된 정책(예: 송신 허용 목록, 파일 시스템 샌드박스 및 타사 배포의 MCP 제한)으로 Code 세션을 제공합니다. Claude Code는 [관리 소스](/docs/ko/managed-settings#how-claude-code-combines-managed-sources)(서버 관리 설정, MDM 또는 OS 수준 정책 또는 관리되는 설정 파일)가 있을 때마다 해당 부모 설정을 무시합니다. 타사 배포와 같이 이전에 없던 디바이스에 이 키를 배포하면 데스크톱 파생 정책이 적용되지 않습니다. [임베딩 호스트가 정책을 추가하도록 허용](/docs/ko/managed-settings#let-an-embedding-host-add-policy)은 부모 설정이 여전히 병합될 수 있는 경우를 다룹니다. 이는 이 방식으로 배포하는 모든 키에 적용되며, 이 키에만 해당하지 않습니다.

<h3 id="disableremotecontrol">
  `disableRemoteControl`
</h3>

[Remote Control](/docs/ko/remote-control)을 끕니다: Claude Code는 `claude remote-control`, `--remote-control` 플래그, 자동 시작 및 세션 내 토글을 거부하며, 조직의 정책이 이를 비활성화했다고 보고합니다. [관리되는 설정](/docs/ko/managed-settings)에 배치하여 디바이스별 MDM 적용을 수행합니다.

* **범위**: [`Any file`](#scopes)
* **유형**: Boolean
  * `true`: Claude Code는 `claude remote-control`, `--remote-control` 플래그, 자동 시작 및 세션 내 토글을 거부합니다
  * `false`: Remote Control은 사용 가능한 상태로 유지됩니다
* **기본값**: `false`

```json settings.json theme={null}
{
  "disableRemoteControl": true
}
```

<h3 id="enableartifact">
  `enableArtifact`
</h3>

claude.ai에서 세션 출력을 비공개 웹 페이지로 게시하는 [Artifact](/docs/ko/artifacts) 도구를 끕니다. `/config`에서 **Artifacts** 행을 끄면 Claude Code는 이 키를 사용자 설정에 작성하므로 일반적으로 수동으로 편집하지 않습니다. Claude Code v2.1.196 이상이 필요합니다.

* **범위**: [`Any file`](#scopes). 모든 파일이 도구를 끌 수 있으며, 아무도 이를 다시 켤 수 없습니다.
* **유형**: Boolean
  * `false`: Claude Code는 파일이 적용되는 모든 세션에 대해 Artifact 도구를 끕니다
  * `true`: 키를 설정하지 않은 것과 동일합니다. 다른 파일의 `false`, [`CLAUDE_CODE_DISABLE_ARTIFACT`](/docs/ko/env-vars) 또는 조직의 [관리 설정](/docs/ko/artifacts#manage-artifacts-for-your-organization)을 재정의하지 않기 때문입니다
* **기본값**: 설정되지 않음, 따라서 도구는 계정의 [가용성](/docs/ko/artifacts#availability)을 따릅니다

```json settings.json theme={null}
{
  "enableArtifact": false
}
```

사용자 설정 이외의 소스가 도구를 끄고 있는 동안 Claude Code는 `/config`에서 **Artifacts** 행을 숨깁니다. 거기서 이를 켜도 아무것도 변경되지 않기 때문입니다. [아티팩트 비활성화](/docs/ko/artifacts#disable-artifacts)는 도구를 끄는 모든 방법을 나열합니다. v2.1.242 이전에는 Claude Code가 프로젝트 및 로컬 설정에서 이 키를 무시했으며, [우선순위 스택](/docs/ko/settings#settings-precedence)에서 더 높은 파일이 낮은 파일의 끔을 다시 켤 수 있었습니다.

<h3 id="inputneedednotifenabled">
  `inputNeededNotifEnabled`
</h3>

권한 프롬프트 또는 질문이 입력을 기다리고 있을 때 휴대폰에서 푸시 알림을 받습니다. Claude Code는 [Remote Control](/docs/ko/remote-control)이 연결되어 있을 때만 이를 보냅니다. `/config`에서 **작업 필요 시 푸시**로 표시됩니다.

* **범위**: [`Any file`](#scopes). Claude Code는 이전 버전에서 `~/.claude.json`에 남겨진 값도 읽습니다.
* **유형**: Boolean
  * `true`: Remote Control이 연결되어 있는 동안 권한 프롬프트 또는 질문이 입력을 기다리고 있을 때 휴대폰에서 푸시 알림을 받습니다
  * `false`: Claude Code는 해당 알림을 보내지 않습니다
* **기본값**: `false`

```json settings.json theme={null}
{
  "inputNeededNotifEnabled": true
}
```

[모바일 푸시 알림](/docs/ko/remote-control#mobile-push-notifications)을 참조하세요.

<h3 id="preferrednotifchannel">
  `preferredNotifChannel`
</h3>

작업이 완료되거나 권한 프롬프트가 대기 중일 때 Claude Code가 알림을 보내는 방식을 선택합니다. `/config`에서 **로컬 알림**으로 표시됩니다.

* **범위**: [`Any file`](#scopes). Claude Code는 이전 버전에서 `~/.claude.json`에 남겨진 값도 읽습니다.
* **유형**: 문자열, 다음 중 하나:
  * `"auto"`: Claude Code는 iTerm2, Ghostty 및 Kitty에서 데스크톱 알림을 보내고, Terminal.app에서는 감지 가능한 벨이 꺼져 있을 때만 벨을 울리며, 다른 곳에서는 아무것도 하지 않습니다
  * `"terminal_bell"`: Claude Code는 모든 터미널에서 벨 문자를 울립니다
  * `"iterm2"`: Claude Code는 iTerm2 데스크톱 알림을 보냅니다
  * `"iterm2_with_bell"`: Claude Code는 iTerm2 데스크톱 알림을 보내고 벨을 울립니다
  * `"kitty"`: Claude Code는 Kitty 데스크톱 알림을 보냅니다
  * `"ghostty"`: Claude Code는 Ghostty 데스크톱 알림을 보냅니다
  * `"notifications_disabled"`: Claude Code는 알림을 보내지 않습니다
* **기본값**: `"auto"`

```json settings.json theme={null}
{
  "preferredNotifChannel": "terminal_bell"
}
```

`"auto"`를 사용하면 Claude Code는 iTerm2, Ghostty 및 Kitty에서 데스크톱 알림을 보냅니다. Terminal.app에서는 Terminal의 감지 가능한 벨을 끄면 벨 문자를 울리고, 다른 터미널에서는 아무것도 하지 않습니다. 모든 터미널에서 벨 문자를 울리려면 `"terminal_bell"`을 설정합니다. [터미널 벨 또는 알림 받기](/docs/ko/terminal-config#get-a-terminal-bell-or-notification)를 참조하세요.

<h3 id="remote-defaultenvironmentid">
  `remote.defaultEnvironmentId`
</h3>

`claude --cloud`와 같이 CLI에서 만드는 클라우드 세션에 대한 기본 [클라우드 환경](/docs/ko/cloud-environments)을 선택합니다. Claude Code는 [`/remote-env`](/docs/ko/cloud-environments#select-an-environment-from-the-cli)로 환경을 선택할 때 이 키를 사용자 설정에 작성합니다.

* **범위**: [`Any file`](#scopes). 자체 호스팅 환경 ID의 경우 사용자 또는 관리되는 설정 또는 `--settings` 플래그만 해당합니다.
* **유형**: 문자열, `env_...` 또는 `ccpool_...`과 같은 환경 ID
* **기본값**: 설정되지 않음, 따라서 Claude Code는 목록에 Anthropic 호스팅 환경이 있으면 이를 사용하고, 그렇지 않으면 [Remote Control 브리지 환경](/docs/ko/cloud-environments#the-default-environment)이 아닌 목록의 첫 번째 환경을 사용하거나, 모든 환경이 브리지 환경일 때 첫 번째 환경을 사용합니다
* **세션별 재정의**: `--environment`는 생성하는 하나의 클라우드 세션에 대해 이 키보다 우선합니다

```json settings.json theme={null}
{
  "remote": {
    "defaultEnvironmentId": "env_0123abcd"
  }
}
```

`env_`로 시작하는 Anthropic 호스팅 환경 ID는 표준 설정 우선순위를 따르므로 저장소의 프로젝트 설정의 값이 사용자 수준 선택을 재정의합니다. `ccpool_`로 시작하는 [자체 호스팅 환경](/docs/ko/self-hosted-environments) ID는 사용자 설정, 관리되는 설정 및 `--settings` 플래그에서만 인정됩니다. Claude Code는 저장소의 프로젝트 또는 로컬 설정에서 이를 무시하며, `/remote-env`는 무시한 값을 표시하므로 체크인된 파일이 선택하지 않은 자체 호스팅 환경으로 세션을 조종할 수 없습니다.

<h3 id="remotecontrolatstartup">
  `remoteControlAtStartup`
</h3>

각 대화형 세션이 시작될 때 [Remote Control](/docs/ko/remote-control)을 자동으로 연결합니다. `/remote-control`을 기다리는 대신입니다. `true`로 설정하여 자동 연결을 켜거나 `false`로 설정하여 끕니다. `/config`에서 **모든 세션에 대해 Remote Control 활성화**로 표시됩니다.

* **범위**: [`Any file`](#scopes). Claude Code는 이전 버전에서 `~/.claude.json`에 남겨진 값도 읽습니다.
* **유형**: Boolean
  * `true`: Claude Code는 각 대화형 세션이 시작될 때 Remote Control을 자동으로 연결합니다
  * `false`: Claude Code는 `/remote-control`을 기다립니다
* **기본값**: 설정되지 않음, 따라서 자동 연결은 설정된 조직의 관리 기본값을 따르고, 그렇지 않으면 Claude Code의 현재 기본값을 따릅니다
* **세션별 재정의**: `--remote-control`은 이 키가 `false`일 때도 한 세션에 대해 Remote Control을 켜고, 플래그는 한 세션에 대해 이를 끌 수 없습니다

```json settings.json theme={null}
{
  "remoteControlAtStartup": true
}
```

Claude Code는 프로젝트 또는 로컬 설정의 `true`를 무시하므로 저장소는 체크아웃에 대해 자동 연결을 끌 수 있지만 켤 수 없습니다. 전체 범위별 동작은 [모든 세션에 대해 Remote Control 활성화](/docs/ko/remote-control#enable-remote-control-for-all-sessions) 및 [더 엄격한 값이 적용되는 보안 키](/docs/ko/settings#security-keys-where-the-stricter-value-applies)를 참조하세요.

<h3 id="sshconfigs">
  `sshConfigs`
</h3>

[Desktop](/docs/ko/desktop#pre-configure-ssh-connections-for-your-team) 환경 드롭다운에 SSH 연결을 추가합니다. 관리자는 이를 사용하여 팀에 공유 연결을 배포합니다. 관리되는 설정에서 정의한 연결은 관리됨으로 표시되므로 사용자는 이를 선택할 수 있지만 앱에서 편집하거나 삭제할 수 없습니다.

* **범위**: [`User or managed`](#scopes). 데스크톱 앱은 이 키를 읽습니다.
* **유형**: 필수 `id`, `name` 및 `sshHost`와 선택적 `sshPort` 및 `sshIdentityFile`을 포함하는 객체 배열
* **기본값**: 설정되지 않음

이 예제는 `user@dev.example.com`에 연결하는 `Dev VM`이라는 연결 하나를 추가합니다:

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

[Desktop SSH 세션](/docs/ko/desktop#restrict-which-ssh-hosts-users-can-connect-to)이 연결할 수 있는 호스트를 제한합니다. Desktop 앱만 이 키를 읽습니다. CLI는 읽지 않습니다. 패턴은 대소문자를 구분하지 않습니다: `*`는 모든 호스트와 일치하고, `*.example.com`은 `example.com` 및 모든 하위 도메인과 일치하며, 다른 모든 것은 `~/.ssh/config` 해석 후 호스트 이름과 정확히 일치합니다. 빈 배열은 SSH 세션을 끕니다.

* **범위**: [`Managed`](#scopes)
* **유형**: 호스트 이름 패턴 배열
* **기본값**: 설정되지 않음, 따라서 모든 호스트가 허용됩니다

이 예제는 `devboxes.example.com` 및 해당 하위 도메인과 정확한 호스트 `bastion.example.com`을 허용합니다:

```json managed-settings.json theme={null}
{
  "sshHostAllowlist": ["*.devboxes.example.com", "bastion.example.com"]
}
```

<span id="authentication-and-login" />

<h2 id="authentication-and-providers">
  인증 및 공급자
</h2>

도우미 스크립트를 통해 자격 증명을 제공하고, 조직의 경우 로그인 방법이나 조직을 강제합니다. [인증](/docs/ko/authentication)을 참조하세요.

<h3 id="apikeyhelper">
  `apiKeyHelper`
</h3>

Claude Code가 모델 요청과 함께 보내는 자격 증명을 생성하기 위해 자신의 명령을 실행합니다. Claude Code는 macOS 및 Linux에서는 `/bin/sh`를, Windows에서는 `cmd`를 통해 시스템 셸을 통해 명령을 실행하고, 그 출력을 `X-Api-Key` 및 `Authorization: Bearer` 헤더 모두로 보냅니다. 자격 증명 모음에서 가져온 단기 토큰과 같은 동적 또는 회전하는 자격 증명에 사용합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열, 셸 명령줄
* **기본값**: 설정되지 않음, 따라서 Claude Code는 도우미를 실행하지 않습니다.

```json settings.json theme={null}
{
  "apiKeyHelper": "/bin/generate_temp_api_key.sh"
}
```

Claude Code는 값을 캐시하고 다음 경우에 명령을 다시 실행합니다:

* 캐시 수명 후, 기본값은 5분이거나 [`CLAUDE_CODE_API_KEY_HELPER_TTL_MS`](/docs/ko/env-vars)로 설정한 간격입니다.
* Anthropic API에 대한 요청이 직접 또는 [LLM 게이트웨이](/docs/ko/llm-gateway)를 통해 `401` 또는 `403`으로 실패할 때입니다.
* Anthropic API에 요청을 보내기 전에, 직접 또는 LLM 게이트웨이를 통해, 캐시된 출력이 도우미가 생성한 후 만료된 JWT일 때입니다. Claude Code v2.1.246 이상이 필요합니다.

마지막 두 경우는 도우미의 출력이 Claude Code가 보내는 자격 증명이고 `ANTHROPIC_AUTH_TOKEN`이 설정되지 않았을 때만 적용됩니다.

대화형 세션에서, 명령이 프로젝트 또는 로컬 설정에서 올 때, Claude Code는 작업 영역 신뢰 프롬프트를 수락할 때까지 실행하지 않습니다. [자격 증명 관리](/docs/ko/authentication#credential-management)를 참조하세요.

<h3 id="awsauthrefresh">
  `awsAuthRefresh`
</h3>

Claude Code가 [Amazon Bedrock](/docs/ko/amazon-bedrock)에 대해 가진 자격 증명이 작동을 멈출 때 `.aws` 디렉토리의 자격 증명을 새로 고치기 위해 `aws sso login`과 같은 자신의 명령을 실행합니다. Claude Code는 먼저 현재 자격 증명을 STS에 대해 확인하고 해당 확인이 실패할 때만 명령을 실행한 다음 새로 고쳐진 `.aws` 디렉토리를 읽습니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열, 셸 명령줄
* **기본값**: 설정되지 않음, 따라서 Claude Code는 AWS 자격 증명을 새로 고치지 않습니다.

```json settings.json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile myprofile"
}
```

새로 고침 흐름이 `.aws`에 쓸 때 이 키를 사용하세요. 대신 자격 증명을 인쇄할 때는 [`awsCredentialExport`](#awscredentialexport)를 사용하세요. [고급 자격 증명 구성](/docs/ko/amazon-bedrock#advanced-credential-configuration)을 참조하세요.

<h3 id="awscredentialexport">
  `awsCredentialExport`
</h3>

Claude Code가 `.aws` 디렉토리에 없는 자격 증명으로 [Amazon Bedrock](/docs/ko/amazon-bedrock)을 호출할 수 있도록 AWS 자격 증명을 JSON으로 인쇄하는 자신의 명령을 실행합니다. Claude Code는 `aws sts` 출력 형태와 평면 `aws configure export-credentials` 형태를 수락하고, 자격 증명을 자신의 Bedrock 클라이언트로 범위를 지정하므로 Claude Code가 실행하는 셸 명령은 여전히 주변 자격 증명을 봅니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열, 셸 명령줄
* **기본값**: 설정되지 않음, 따라서 Claude Code는 주변 AWS 자격 증명 체인을 사용합니다.

```json settings.json theme={null}
{
  "awsCredentialExport": "/bin/generate_aws_grant.sh"
}
```

[`awsAuthRefresh`](#awsauthrefresh)와 달리, Claude Code는 이 명령이 설정되면 먼저 주변 자격 증명을 확인하지 않고 항상 실행합니다. [고급 자격 증명 구성](/docs/ko/amazon-bedrock#advanced-credential-configuration)을 참조하세요.

<h3 id="forceloginmethod">
  `forceLoginMethod`
</h3>

사람들이 로그인할 수 있는 계정 종류를 제한합니다. `"claudeai"`로 설정하여 claude.ai 계정만 허용하거나, `"console"`로 설정하여 Claude Console 계정만 허용하거나, `"gateway"`로 설정하여 사람들을 첫 번째 당사자 로그인 대신 [클라우드 게이트웨이](/docs/ko/claude-apps-gateway)로 보냅니다. 관리자는 관리 설정에서 설정하고 [`forceLoginOrgUUID`](#forceloginorguuid)와 쌍을 이루어 개발자의 claude.ai 로그인을 한 조직 내에 유지합니다. 설정 파일에서 `"claudeai"` 또는 `"console"`로 설정하면, Claude Code는 해당 파일이 적용되는 세션에서 [키 없는 Console 로그인](/docs/ko/authentication#sign-in-without-an-api-key)도 제공하지 않습니다.

* **범위**: [`모든 파일`](#scopes). Claude Code는 `"gateway"`를 머신의 관리 소스에서만 인정합니다: `managed-settings.json`, macOS plist 또는 Windows HKLM 레지스트리, 또는 정책 도우미. 사용자, 프로젝트, 로컬, HKCU, 서버 관리 설정에서는 `"gateway"`를 설정되지 않은 것으로 취급하며, [`forceLoginGatewayUrl`](#forcelogingatewayurl)과 동일한 규칙입니다.
* **유형**: 문자열, 다음 중 하나:
  * `"claudeai"`: claude.ai 계정만 로그인할 수 있습니다.
  * `"console"`: Claude Console 계정만 로그인할 수 있습니다.
  * `"gateway"`: Claude Code는 사람들을 첫 번째 당사자 로그인 대신 클라우드 게이트웨이로 보냅니다.
* **기본값**: 설정되지 않음, 따라서 사람들이 로그인 방법을 선택합니다.

```json settings.json theme={null}
{
  "forceLoginMethod": "claudeai"
}
```

모든 첫 번째 당사자 로그인 경로는 [VS Code 확장](/docs/ko/vs-code), Agent SDK, `claude setup-token`, 및 `/install-github-app`을 포함한 제한을 적용하며, 터미널의 대화형 로그인 화면은 제외하고, `/login` 또는 첫 실행 온보딩으로 도달하며, 이는 강제하지 않고 방법을 미리 선택합니다. v2.1.212 이전에는 터미널 로그인만 적용했습니다. [조직에 로그인 제한](/docs/ko/authentication#restrict-login-to-your-organization)을 참조하여 각 로그인 경로, 환경 자격 증명, 및 제3자 공급자가 어떻게 처리되는지 확인하세요.

머신의 관리 소스가 `"gateway"`를 설정하면, Claude Code는 남은 로그인, API 키, 또는 `apiKeyHelper` 자격 증명을 사용하지 않습니다. [관리자 정책이 클라우드 게이트웨이 로그인을 요구합니다](/docs/ko/errors#administrator-policy-requires-a-cloud-gateway-sign-in)를 참조하여 각각이 생성하는 메시지를 확인하세요. `CLAUDE_CODE_USE_BEDROCK` 또는 유사한 환경 변수를 통해 클라우드 공급자를 선택하면, 세션은 게이트웨이 로그인이 필요하지 않습니다. v2.1.261 이전에는 Claude Code가 이러한 머신에서 남은 로그인을 사용했습니다.

<h3 id="forcelogingatewayurl">
  `forceLoginGatewayUrl`
</h3>

`/login` 클라우드 게이트웨이 화면이 연결하는 게이트웨이 URL을 설정하여 사람들이 주소를 입력하지 않고 [클라우드 게이트웨이](/docs/ko/claude-apps-gateway)에 도달하도록 합니다. 화면에는 URL 필드가 없습니다: 이 키가 설정되면, 게이트웨이 URL을 표시하고 사람이 Enter를 누르면 연결합니다. 없으면, IT 관리자에게 문의하도록 알립니다.

이 키 또는 `forceLoginMethod: "gateway"`는 머신을 게이트웨이 전용으로 만들므로, `/login`은 로그인 방법 선택기 없이 클라우드 게이트웨이 화면에서 열립니다. [관리자 정책이 클라우드 게이트웨이 로그인을 요구합니다](/docs/ko/errors#administrator-policy-requires-a-cloud-gateway-sign-in)를 참조하여 남은 첫 번째 당사자 로그인 또는 API 키에 어떤 일이 발생하는지 확인하세요. 화면이 오류를 표시하는 대신 연결하도록 두 키를 모두 설정하세요.

* **범위**: [`관리됨`](#scopes). 머신의 소스에서만 읽습니다: `managed-settings.json`, macOS plist 또는 Windows HKLM 레지스트리, 또는 정책 도우미. Claude Code는 HKCU 및 서버 관리 설정에서 무시합니다.
* **유형**: 문자열, 스키마를 포함한 전체 URL
* **기본값**: 설정되지 않음, 따라서 클라우드 게이트웨이 화면은 IT 관리자에게 문의하도록 알리는 오류를 표시합니다.

```json managed-settings.json theme={null}
{
  "forceLoginGatewayUrl": "https://claude-gateway.example.com"
}
```

값이 유효한 URL이 아니면, 로그인 화면이 보고하고, 관리 설정 파일의 나머지는 여전히 적용됩니다. [게이트웨이 URL 설정](/docs/ko/claude-apps-gateway#set-the-gateway-url)을 참조하세요.

<h3 id="forceloginorguuid">
  `forceLoginOrgUUID`
</h3>

관리 소스에서, claude.ai 계정 로그인이 단일 UUID로 지정된 하나의 Anthropic 조직에 속하거나 배열로 지정된 여러 조직 중 하나에 속하도록 요구합니다. 모든 설정 파일에서, Claude Code는 또한 단일 UUID를 사용하여 claude.ai 또는 Claude Console 로그인 중에 해당 조직을 미리 선택하고, 배열의 경우 아무것도 미리 선택하지 않습니다. 설정 파일에서 키를 설정하면, Claude Code는 또한 해당 파일이 적용되는 세션에서 [키 없는 Console 로그인](/docs/ko/authentication#sign-in-without-an-api-key)을 제공하지 않고 대신 API 키를 생성합니다.

* **범위**: [`모든 파일`](#scopes). 관리 소스만 제한을 강제합니다. 다른 설정 파일의 단일 UUID는 제한하지 않고 로그인 중에 조직을 미리 선택합니다.
* **유형**: 문자열, 하나의 UUID, 또는 문자열 배열, 여러 UUID
* **기본값**: 설정되지 않음, 따라서 모든 조직이 로그인할 수 있습니다.

이 예제는 하나를 미리 선택하지 않고 두 조직 중 하나에서 로그인을 수락합니다:

```json managed-settings.json theme={null}
{
  "forceLoginOrgUUID": ["xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx", "yyyyyyyy-yyyy-yyyy-yyyy-yyyyyyyyyyyy"]
}
```

관리 소스가 빈 배열을 설정하거나 Claude Code가 구문 분석할 수 없는 값을 설정하면, Claude Code는 잘못된 구성 메시지로 모든 로그인을 차단합니다.

[조직에 로그인 제한](/docs/ko/authentication#restrict-login-to-your-organization)을 참조하여 Claude Code가 Claude Console 로그인, 다른 로그인 경로, 및 환경 자격 증명을 어떻게 취급하는지 확인하세요.

<h3 id="gatewayinternalnetworks">
  `gatewayInternalNetworks`
</h3>

조직이 내부 네트워크를 번호 매기는 공개 IPv4 블록을 선언하여 `/login`이 [클라우드 게이트웨이](/docs/ko/claude-apps-gateway)를 그곳에서 수락하도록 합니다. Claude Code v2.1.268 이상이 필요합니다.

이 키가 없으면, `/login`은 개인 주소의 모든 게이트웨이에 연결하고 다른 것은 없습니다. 이 키가 있으면, `/login`은 또한 나열된 블록 내의 게이트웨이를 직접 연결을 통해서만 수락합니다. 해당 연결의 머신 자신의 주소도 동일한 블록 내에 있어야 합니다.

* **범위**: [`관리됨`](#scopes). 머신의 소스에서만 읽습니다: `managed-settings.json`, macOS plist 또는 Windows HKLM 레지스트리, 또는 정책 도우미. Claude Code는 HKCU 및 서버 관리 설정에서 무시합니다.
* **유형**: 문자열 배열, 최대 4개의 IPv4 CIDR 블록, 각각 `/8`에서 `/32`, 서로 겹치지 않음, 그리고 개인 공간과 겹치지 않음.
* **기본값**: 설정되지 않음, 따라서 `/login`은 개인 주소의 게이트웨이만 수락합니다.

```json managed-settings.json theme={null}
{
  "gatewayInternalNetworks": ["203.0.113.0/24"]
}
```

예제의 설명서 범위를 자신의 블록으로 바꾸세요. Claude Code는 설명서 범위, VPN 및 NAT64 클라이언트가 로컬로 사용하는 범위, 그리고 멀티캐스트와 같이 어떤 네트워크도 번호 매기지 않는 예약된 공간을 거부합니다.

항목이 유효하지 않거나 값이 문자열 목록이 아니면, `/login`은 문제를 이름 지정하고 값을 수정할 때까지 머신의 모든 새로운 게이트웨이 로그인을 거부합니다. 기존 로그인은 계속 작동합니다. [공개 주소 공간에서 소유한 게이트웨이 허용](/docs/ko/claude-apps-gateway#allow-a-gateway-on-public-address-space-you-own)을 참조하여 전체 규칙과 개발자가 보는 것을 확인하세요.

<h3 id="gcpauthrefresh">
  `gcpAuthRefresh`
</h3>

Claude Code가 Google Cloud Application Default Credentials가 만료되었거나 로드할 수 없음을 발견할 때 새로 고치기 위해 자신의 명령을 실행하여 [Google Cloud의 Agent Platform](/docs/ko/google-vertex-ai) 요청이 손으로 다시 인증하지 않고도 계속 작동하도록 합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열, 셸 명령줄
* **기본값**: 설정되지 않음, 따라서 Claude Code의 자격 증명 오류는 `gcloud auth application-default login`을 직접 실행하도록 알립니다.

```json settings.json theme={null}
{
  "gcpAuthRefresh": "gcloud auth application-default login"
}
```

[고급 자격 증명 구성](/docs/ko/google-vertex-ai#advanced-credential-configuration)을 참조하세요.

<h3 id="otelheadershelper">
  `otelHeadersHelper`
</h3>

Claude Code가 OpenTelemetry 내보내기와 함께 보내는 헤더를 생성하기 위해 자신의 명령을 실행하여, 토큰이 회전하는 백엔드의 경우입니다. Claude Code는 시작 시 및 그 후 주기적으로 실행하고, stdout에서 문자열 헤더 값의 JSON 객체를 예상합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 문자열, 실행 가능한 경로 또는 셸 명령줄
* **기본값**: 설정되지 않음, 따라서 Claude Code는 도우미 생성 헤더를 추가하지 않습니다.

```json settings.json theme={null}
{
  "otelHeadersHelper": "/bin/generate_otel_headers.sh"
}
```

[`CLAUDE_CODE_OTEL_HEADERS_HELPER_DEBOUNCE_MS`](/docs/ko/env-vars)로 새로 고침 간격을 설정하세요. [동적 헤더](/docs/ko/monitoring-usage#dynamic-headers)를 참조하여 스크립트 요구 사항과 도우미가 실패할 때 어떤 일이 발생하는지 확인하세요.

<h2 id="updates-and-versioning">
  업데이트 및 버전 관리
</h2>

업데이트 채널을 선택하고, 조직의 경우 사용자가 실행할 수 있는 버전을 고정합니다. [Claude Code 업데이트](/docs/ko/setup#update-claude-code)를 참조하십시오.

<h3 id="autoupdateschannel">
  `autoUpdatesChannel`
</h3>

백그라운드 자동 업데이트 및 `claude update`가 따르는 [릴리스 채널](/docs/ko/setup#configure-release-channel)을 선택합니다. 일반적으로 약 1주일 된 버전이며 주요 회귀가 있는 릴리스를 건너뛰는 `"stable"`을 설정하거나, 가장 최근 릴리스를 위해 `"latest"`를 설정합니다.

* **범위**: [`모든 파일`](#scopes). 조직 전체에 하나의 채널을 적용하려면 관리 설정에서 설정합니다.
* **유형**: 문자열, 다음 중 하나:
  * `"latest"`: 업데이트가 가장 최근 릴리스를 따릅니다
  * `"stable"`: 업데이트가 일반적으로 약 1주일 된 버전을 따르며 주요 회귀가 있는 릴리스를 건너뜁니다
* **기본값**: 설정되지 않음, Claude Code는 `"latest"`를 따릅니다

```json settings.json theme={null}
{
  "autoUpdatesChannel": "stable"
}
```

Claude Code는 `/config`의 **자동 업데이트 채널**에서 선택할 때 사용자 설정에 `"stable"`을 기록하고, 거기서 최신으로 다시 전환할 때 키를 제거합니다. `claude install stable` 및 `claude install latest`도 명명한 채널을 저장합니다. `/config`에서 `"latest"`에서 `"stable"`로 전환하면 다운그레이드를 허용할지 또는 현재 버전에 머물지 묻습니다. 머물기를 선택하면 [`minimumVersion`](#minimumversion)을 설정합니다. Homebrew 설치는 이 키를 무시합니다: `claude-code` cask는 stable을 추적하고 `claude-code@latest`는 latest를 추적하며, `claude update`는 `brew upgrade`로 연기됩니다. 자동 업데이트를 완전히 끄려면 `env`에서 [`DISABLE_AUTOUPDATER`](/docs/ko/setup#disable-auto-updates)를 설정합니다.

<h3 id="minimumversion">
  `minimumVersion`
</h3>

백그라운드 자동 업데이트 및 `claude update`가 이 버전 아래의 버전을 설치하지 않도록 하여, `"stable"` 채널로 이동해도 최신 `"latest"` 빌드에서 다운그레이드되지 않습니다. Claude Code는 `/config`에서 채널을 전환하면서 현재 버전에 머물기를 선택할 때 이 키를 기록하고, `"latest"`로 다시 전환할 때 지웁니다.

* **범위**: [`모든 파일`](#scopes). 조직 전체 최소값을 고정하려면 관리 설정에서 설정하여 사용자 및 프로젝트 설정이 낮출 수 없도록 합니다.
* **유형**: 문자열, `"2.1.100"`과 같은 버전 번호; 유효한 버전이 아닌 값은 무시됩니다
* **기본값**: 설정되지 않음, 업데이트는 채널이 제공하는 모든 버전을 설치할 수 있습니다

이 예제는 stable 채널을 따르고 2.1.100 아래의 버전 설치를 거부합니다:

```json settings.json theme={null}
{
  "autoUpdatesChannel": "stable",
  "minimumVersion": "2.1.100"
}
```

이 키는 업데이트만 제한합니다. Claude Code가 버전 아래에서 시작하지 않도록 하려면 대신 [`requiredMinimumVersion`](#requiredminimumversion)을 사용합니다. [최소 버전 고정](/docs/ko/setup#pin-a-minimum-version)을 참조하십시오.

<h3 id="requiredmaximumversion">
  `requiredMaximumVersion`
</h3>

조직이 시작할 수 있는 가장 최신 Claude Code 버전을 설정합니다. 실행 중인 버전이 더 최신이면 Claude Code는 시작 시 종료되고 사용자에게 조직의 승인된 방법을 통해 승인된 버전을 설치하도록 지시합니다. `claude install <version>`도 작동할 수 있습니다. Claude Code v2.1.163 이상이 필요합니다.

* **범위**: [`관리됨`](#scopes). Claude Code는 다른 곳에서 키를 무시할 때 경고를 제공하지 않습니다.
* **유형**: 문자열, `"2.1.150"`과 같은 버전 번호; 유효한 버전이 아닌 값은 무시됩니다
* **기본값**: 설정되지 않음, 상한이 적용되지 않습니다

```json managed-settings.json theme={null}
{
  "requiredMaximumVersion": "2.1.150"
}
```

백그라운드 자동 업데이트 및 `claude update`는 상한 위의 버전을 건너뛰므로 범위 내의 설치는 범위 내에 머물러 있습니다. `claude update`, `claude install`, 및 `claude doctor`는 사용자가 복구할 수 있도록 상한 위에서 계속 작동합니다. [`requiredMinimumVersion`](#requiredminimumversion)과 쌍을 이루어 범위를 적용합니다.

<h3 id="requiredminimumversion">
  `requiredMinimumVersion`
</h3>

조직이 시작할 수 있는 가장 오래된 Claude Code 버전을 설정합니다. 실행 중인 버전이 더 오래되면 Claude Code는 시작 시 종료되고 사용자에게 조직의 승인된 방법을 통해 업데이트하도록 지시합니다. 검사는 시작 시에만 실행되므로 이미 실행 중인 세션은 계속됩니다. Claude Code v2.1.163 이상이 필요합니다.

* **범위**: [`관리됨`](#scopes). Claude Code는 다른 곳에서 키를 무시할 때 경고를 제공하지 않습니다.
* **유형**: 문자열, `"2.1.150"`과 같은 버전 번호; 유효한 버전이 아닌 값은 무시됩니다
* **기본값**: 설정되지 않음, 하한이 적용되지 않습니다

```json managed-settings.json theme={null}
{
  "requiredMinimumVersion": "2.1.150"
}
```

`claude update`, `claude install`, 및 `claude doctor`는 사용자가 복구할 수 있도록 하한 아래에서 계속 작동합니다. 다운그레이드만 방지하는 [`minimumVersion`](#minimumversion)과 달리, 이 키는 시작을 차단합니다. [`requiredMaximumVersion`](#requiredmaximumversion)과 쌍을 이루어 범위를 적용합니다.

<h2 id="tools">
  도구
</h2>

[Claude Code 데스크톱 앱](/docs/ko/desktop)에서 특정 도구를 끕니다. 터미널 CLI는 이 키를 무시합니다. 도구 자체에 대해서는 [Claude에서 사용 가능한 도구](/docs/ko/tools-reference)를 참조하세요.

<h3 id="browserexternalpagetools">
  `browserExternalPageTools`
</h3>

Claude가 데스크톱 앱의 [브라우저 창](/docs/ko/desktop#browse-external-sites)에서 외부 페이지를 읽거나 작동하는 도구를 사용하지 못하도록 합니다. 조직의 사용자는 여전히 외부 사이트를 직접 열 수 있으며, 로컬 개발 서버 미리보기는 Claude의 도구와 함께 계속 작동합니다. 데스크톱 앱이 이 키를 읽습니다. 터미널 CLI는 무시합니다.

* **범위**: [`Managed`](#scopes)
* **유형**: 문자열, `"disabled"`; 데스크톱 앱은 `"disable"`도 허용하며, 어느 경우든
* **기본값**: 설정되지 않음, 따라서 Claude의 도구는 외부 페이지에서 작동합니다

```json managed-settings.json theme={null}
{
  "browserExternalPageTools": "disabled"
}
```

다른 값은 Claude의 도구를 켜진 상태로 두며, 허용된 두 값 중 하나가 아닌 비어있지 않은 문자열은 경고를 기록합니다. 사용자와 Claude 모두에 대해 외부 사이트를 차단하려면 대신 [`disableBrowserExternalNavigation`](#disablebrowserexternalnavigation)을 설정하세요. [조직의 외부 브라우징 제한](/docs/ko/desktop#restrict-external-browsing-for-your-organization)을 참조하세요.

<h3 id="disablebrowserexternalnavigation">
  `disableBrowserExternalNavigation`
</h3>

데스크톱 앱의 [브라우저 창](/docs/ko/desktop#browse-external-sites)에서 사용자와 Claude 모두에 대해 외부 브라우징을 끕니다. Localhost 개발 서버 미리보기는 계속 작동합니다. 데스크톱 앱이 이 키를 읽습니다. 터미널 CLI는 무시합니다.

* **범위**: [`Managed`](#scopes)
* **유형**: 부울; JSON 부울 `true`만 적용됩니다
  * `true`: 데스크톱 앱이 브라우저 창에서 사용자와 Claude 모두에 대해 외부 브라우징을 끕니다. localhost 미리보기는 계속 작동합니다
  * `false`: 외부 브라우징이 켜진 상태로 유지됩니다
* **기본값**: 설정되지 않음, 따라서 외부 브라우징이 켜져 있습니다

```json managed-settings.json theme={null}
{
  "disableBrowserExternalNavigation": true
}
```

데스크톱 앱은 다른 값을 무시하며, 문자열 `"true"` 또는 `1`과 같이 부울이 아닌 값도 경고를 기록합니다. 외부 브라우징은 켜진 상태로 두되 Claude의 도구를 외부 페이지에서 끄려면 대신 [`browserExternalPageTools`](#browserexternalpagetools)를 설정하세요. [조직의 외부 브라우징 제한](/docs/ko/desktop#restrict-external-browsing-for-your-organization)을 참조하세요.

<h3 id="disablemobilesimulatortools">
  `disableMobileSimulatorTools`
</h3>

데스크톱 앱의 [iOS 시뮬레이터 창](/docs/ko/desktop-ios-simulator#turn-off-simulator-access)에 대해 Claude의 도구를 차단합니다. 사용자는 창을 수동으로 사용할 수 있습니다. Claude의 접근만 제거되며, 아무도 앱 내에서 이를 다시 켤 수 없습니다. 데스크톱 앱이 이 키를 읽습니다. 터미널 CLI는 무시합니다.

* **범위**: [`Managed`](#scopes)
* **유형**: 부울; JSON 부울 `true`만 적용됩니다
  * `true`: 데스크톱 앱이 iOS 시뮬레이터 창에 대해 Claude의 도구를 차단합니다
  * `false`: Claude의 시뮬레이터 도구는 데스크톱 앱의 각 사용자 설정 토글을 따릅니다
* **기본값**: 설정되지 않음, 따라서 Claude의 시뮬레이터 도구는 데스크톱 앱의 각 사용자 설정 토글을 따릅니다

```json managed-settings.json theme={null}
{
  "disableMobileSimulatorTools": true
}
```

데스크톱 앱은 다른 값을 무시하며, 문자열 `"true"` 또는 `1`과 같이 부울이 아닌 값도 경고를 기록합니다.

<span id="data-and-privacy" />

<h2 id="privacy-and-telemetry">
  개인정보 보호 및 원격 측정
</h2>

Claude Code가 세션 데이터를 얼마나 오래 보관하고 무엇을 전송하는지 제어합니다. 사용 지표 및 오류 보고를 끄는 스위치는 설정 키가 아니라 환경 변수입니다. [`env`](#env) 키 또는 셸에서 `DISABLE_TELEMETRY`, `DISABLE_ERROR_REPORTING` 또는 `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`을 설정합니다. [원격 측정 서비스](/docs/ko/data-usage#telemetry-services)에서 각각이 무엇을 중지하는지 설명합니다. 두 가지 예외는 설정 파일에서 끕니다. 아래의 [`feedbackDrafts`](#feedbackdrafts)는 Claude가 작성한 피드백용이고, 아래의 [`feedbackSurveyRate`](#feedbacksurveyrate)는 세션 설문조사용입니다.

<h3 id="cleanupperioddays">
  `cleanupPeriodDays`
</h3>

Claude Code가 [세션 기록 및 기타 애플리케이션 데이터](/docs/ko/claude-directory#cleaned-up-automatically)를 삭제하기 전에 보관하는 일 수를 설정합니다. Claude Code는 세션이 시작된 후 백그라운드 스윕으로 삭제를 실행하며, 보관 기간을 안전하게 결정할 수 있는 한 실행합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 일 수, 정수, 최소값 `1`
* **기본값**: `30`

```json settings.json theme={null}
{
  "cleanupPeriodDays": 20
}
```

`0`을 설정하면 유효성 검사에 실패하므로 장기 보관을 위해 `3650`과 같은 큰 값을 선택합니다. Claude Code가 기록을 전혀 작성하지 않도록 하려면 [일반 텍스트 저장소](/docs/ko/claude-directory#plaintext-storage)를 참조합니다.

<h3 id="desktopsessioncleanupperioddays">
  `desktopSessionCleanupPeriodDays`
</h3>

Claude Desktop 또는 Cowork에서 시작하거나 가장 최근에 계속한 세션의 기록에 대한 나이 제한을 일 수로 설정합니다. 이 키가 없으면 Claude Code는 [해당 기록을 모든 나이에서 유지합니다](/docs/ko/claude-directory#cleaned-up-automatically). Claude Code는 각 기록이 이 제한과 [`cleanupPeriodDays`](#cleanupperioddays) 모두보다 오래되면 삭제하므로, `cleanupPeriodDays`가 기본값 30일 때 `7` 값은 여전히 30일 동안 유지합니다. 관리되는 설정이 `cleanupPeriodDays`를 설정하면 해당 기간이 대신 적용되고 이 키는 무시됩니다. Claude Code v2.1.248 이상이 필요합니다.

* **범위**: [`사용자 또는 관리됨`](#scopes). Claude Code는 `--settings`로 전달하는 파일에서도 키를 읽고 프로젝트 및 로컬 설정에서는 무시합니다.
* **유형**: 일 수, 정수, 최소값 `0`
* **기본값**: `0`, 나이 제한을 설정하지 않음

```json settings.json theme={null}
{
  "desktopSessionCleanupPeriodDays": 90
}
```

<h3 id="feedbackdrafts">
  `feedbackDrafts`
</h3>

[Claude가 작성한 피드백](/docs/ko/tools-reference#sendfeedback-tool-behavior) 제어: Claude가 검토할 피드백 초안을 대기열에 넣을 수 있는지 여부, 그리고 Claude Code가 Claude가 초안을 대기열에 넣을 때 카드를 표시하는지 여부입니다.

* **범위**: [`사용자 또는 관리됨`](#scopes)
* **유형**: 문자열, `"notify"`, `"quiet"` 또는 `"off"` 중 하나
  * `"notify"`: Claude Code는 Claude가 초안을 대기열에 넣을 때 프롬프트 위에 카드를 표시하며, 기본적으로 [세션당 최대 3개의 카드](/docs/ko/tools-reference#what-you-see-when-claude-drafts)
  * `"quiet"`: Claude는 카드 없이 초안을 작성합니다. 프롬프트 바닥글에서 대기열에 있는 초안의 개수를 보고 `/feedback`에서 검토합니다.
  * `"off"`: Claude Code는 SendFeedback 도구를 제거하므로 Claude는 초안을 대기열에 넣을 수 없습니다.
* **기본값**: `"notify"`
* **세션별 재정의**: [`CLAUDE_CODE_SEND_FEEDBACK`](/docs/ko/env-vars)을 `0`으로 설정하면 한 세션 동안 기능을 끕니다.

```json settings.json theme={null}
{
  "feedbackDrafts": "quiet"
}
```

`/config`에 **Claude가 작성한 피드백**으로 나타나며, 이 키를 사용자 설정에 씁니다. `/config` 행은 [Claude가 피드백을 작성할 수 있는 세션](/docs/ko/tools-reference#sessions-without-claude-drafted-feedback)에서만 표시됩니다. `"off"`를 설정해도 숨기지 않으므로 같은 행에서 기능을 다시 켤 수 있습니다. 관리되는 설정의 값은 사용자 설정보다 우선하므로, 관리자가 이 키를 설정하면 행에 관리되는 값이 표시되고 변경해도 효과가 없습니다. Claude Code는 프로젝트 및 로컬 설정에서 이 키를 무시합니다.

<h3 id="feedbacksurveyrate">
  `feedbackSurveyRate`
</h3>

[세션 품질 설문조사](/docs/ko/data-usage#session-quality-surveys)가 세션이 적격일 때 나타날 확률을 설정합니다. 설문조사가 나타나지 않도록 하려면 `0`으로 설정합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: `0`과 `1` 사이의 숫자
* **기본값**: 설정되지 않음, Claude Code는 Anthropic이 원격으로 설정한 비율을 사용하거나, Amazon Bedrock, Google Cloud의 Agent Platform, Microsoft Foundry에서 기본 제공 비율 `0.005`를 사용합니다. 이들은 원격 구성을 받지 않습니다.
* **세션별 재정의**: [`CLAUDE_CODE_DISABLE_FEEDBACK_SURVEY`](/docs/ko/env-vars)을 `1`로 설정하면 이 키가 설정한 비율과 관계없이 한 세션 동안 설문조사를 끕니다.

```json settings.json theme={null}
{
  "feedbackSurveyRate": 0.05
}
```

같은 비율이 VS Code 확장 프로그램의 설문조사에도 적용됩니다.

<h3 id="skipwebfetchpreflight">
  `skipWebFetchPreflight`
</h3>

[WebFetch 도메인 안전 검사](/docs/ko/data-usage#webfetch-domain-safety-check)를 건너뜁니다. 이 검사는 가져오기 전에 요청된 각 호스트명을 `api.anthropic.com`으로 보냅니다. Amazon Bedrock, Google Cloud의 Agent Platform 또는 제한적인 송신이 있는 Microsoft Foundry 배포와 같이 Anthropic으로의 트래픽을 차단하는 환경에서 `true`로 설정합니다.

* **범위**: [`모든 파일`](#scopes)
* **유형**: 부울
  * `true`: Claude Code는 WebFetch 도메인 안전 검사를 건너뜁니다.
  * `false`: 검사는 세션의 각 호스트명으로의 첫 번째 가져오기 전에 실행되며, 이전 검사가 차단되거나 실패한 호스트명에 대해 다시 실행됩니다.
* **기본값**: 설정되지 않음, 검사는 세션의 각 호스트명으로의 첫 번째 가져오기 전에 실행됩니다.

```json settings.json theme={null}
{
  "skipWebFetchPreflight": true
}
```

검사를 건너뛰면 WebFetch는 차단 목록을 참조하지 않고 모든 URL을 시도하므로, Claude가 도달할 수 있는 도메인을 제한해야 하는 경우 [`WebFetch` 권한 규칙](/docs/ko/permissions#webfetch)과 함께 사용합니다.

<span id="managed-policy" />

<h2 id="enterprise-and-managed-settings">
  엔터프라이즈 및 관리형 설정
</h2>

조직이 관리형 설정을 계산, 새로 고침 및 결합하는 데 사용하는 키입니다. [관리형 설정 설정](/docs/ko/admin-setup)을 참조하십시오.

<h3 id="disablesideloadflags">
  `disableSideloadFlags`
</h3>

시작 시 `--plugin-dir`, `--plugin-url`, `--agents` 및 `--mcp-config` CLI 플래그를 거부합니다. 사용자는 이러한 플래그를 전달하여 단일 실행을 위해 [`strictKnownMarketplaces`](#strictknownmarketplaces)를 우회할 수 있습니다. Claude Code는 거부된 플래그의 이름을 지정하는 오류로 종료되며, 현재 데스크톱 앱의 [Cowork](/docs/ko/desktop) 로컬 세션인 이러한 플래그로 CLI를 내부적으로 시작하는 표면에 동일한 검사를 적용합니다. [클라우드 세션](/docs/ko/claude-code-on-the-web)에서 Claude Code는 서버가 `--mcp-config`를 통해 전달한 MCP 서버를 삭제합니다. 단, 프로세스 내 `type: "sdk"` 항목은 제외하고 세션을 시작합니다. Claude Code v2.1.193 이상이 필요합니다.

* **범위**: [`Managed`](#scopes)
* **유형**: Boolean
  * `true`: Claude Code는 시작 시 `--plugin-dir`, `--plugin-url`, `--agents` 및 `--mcp-config`를 거부하고 이들의 이름을 지정하는 오류로 종료됩니다. 단, 클라우드 세션에서는 서버가 `--mcp-config`를 통해 전달한 MCP 서버를 삭제합니다. 프로세스 내 `type: "sdk"` 항목은 제외하고 세션을 시작합니다.
  * `false`: Claude Code는 해당 플래그를 수락합니다.
* **기본값**: `false`

```json managed-settings.json theme={null}
{
  "disableSideloadFlags": true
}
```

Claude Code는 여전히 서버가 모두 프로세스 내 `type: "sdk"` 항목인 `--mcp-config`를 수락하므로 Agent SDK 및 VS Code 확장이 계속 작동합니다. 사용자는 여전히 `claude mcp add` 또는 `.mcp.json` 파일로 서버를 추가할 수 있습니다. 서버별 제어를 위해 [`allowedMcpServers`](/docs/ko/managed-mcp)도 설정하십시오. Claude Code v2.1.193 이상이 필요합니다.

클라우드 세션에서 Claude Code는 또한 서버 전달 중간 세션 MCP 업데이트를 무시합니다. 이는 클라우드 세션 구성 및 SDK `setMcpServers()` 호출이 이러한 세션에 도달하는 경로입니다. 프로세스 내 `type: "sdk"` 항목은 여기서도 면제됩니다. v2.1.239 이전에는 서버 전달 `--mcp-config`가 클라우드 세션이 시작되는 것을 차단했습니다.

<h3 id="forceremotesettingsrefresh">
  `forceRemoteSettingsRefresh`
</h3>

Claude Code가 [서버 관리형 설정](/docs/ko/server-managed-settings)을 새로 가져올 때까지 CLI 시작을 차단합니다. 가져오기가 실패하면 Claude Code는 캐시된 설정이나 설정 없이 계속하지 않고 종료합니다. 환경이 관리형 정책 없이 세션이 실행되는 짧은 시간 창도 수용할 수 없을 때 설정하십시오.

키가 설정되지 않으면 Claude Code는 가져오기에서 시작을 차단하지 않습니다. 단, 개발자가 시작 시 로그인할 때는 가져오기를 위해 최대 5초를 기다립니다. Cloud 게이트웨이 세션은 항상 기다리며, 게이트웨이에 도달할 수 없으면 종료됩니다.

* **범위**: [`Managed`](#scopes). Claude Code는 최우선 소스가 아닌 경우에도 관리자 제어 관리형 소스에서 `true`를 인정합니다.
* **유형**: Boolean
  * `true`: Claude Code는 서버 관리형 설정을 새로 가져올 때까지 시작을 차단하고 가져오기가 실패하면 종료합니다.
  * `false`: Claude Code는 가져오기에서 시작을 차단하지 않습니다. 단, 로그인 시작 시 최대 5초를 기다립니다.
* **기본값**: `false`

```json managed-settings.json theme={null}
{
  "forceRemoteSettingsRefresh": true
}
```

MDM 프로필 또는 관리형 설정 파일에 설정하여 첫 번째 서버 페이로드가 도착하기 전에 실패 폐쇄 시작을 적용합니다. Claude Code는 서버 관리형 설정을 가져오는 세션에서만 검사를 적용하므로 [이를 가져오지 않는](/docs/ko/server-managed-settings#platform-availability) 세션은 기다리지 않고 시작됩니다. `claude auth` 하위 명령은 면제되므로 사용자는 만료된 자격 증명이 가져오기 실패의 원인일 때 다시 인증할 수 있습니다. [실패 폐쇄 시작 적용](/docs/ko/server-managed-settings#enforce-fail-closed-startup)을 참조하십시오.

<h3 id="managedsourcesbehavior">
  `managedSourcesBehavior`
</h3>

Claude Code가 조직이 전달하는 최우선 [관리형 소스](/docs/ko/managed-settings#how-claude-code-combines-managed-sources)만 적용할지, 아니면 전달하는 모든 관리자 소스를 결합할지 선택합니다. 기본적으로 Claude Code는 [정책 키](/docs/ko/managed-settings#how-claude-code-combines-managed-sources)를 전달하는 최우선 소스를 취하고 나머지는 무시합니다. 정책 키는 이 키와 `wslInheritsWindowsSettings`를 제외한 모든 설정 키입니다. 따라서 서버 관리형 설정이나 MDM 정책이 정책 키를 전달하면 `managed-settings.json` 파일은 [Claude Code가 모든 관리자 소스에서 읽는 키](/docs/ko/managed-settings#keys-read-from-every-admin-source)만 제공합니다. `"merge"`를 사용하면 전달하는 모든 관리자 소스가 하나의 결합된 정책에 키를 제공합니다. Claude Code v2.1.242 이상이 필요합니다.

최우선 소스 아래에 [순위가 지정된](/docs/ko/managed-settings#how-claude-code-combines-managed-sources) 모든 소스가 관리자의 제어 하에 있는 경우에만 `"merge"`를 설정하십시오. Claude Code는 `permissions.allow` 규칙과 같은 하위 소스의 항목을 정책에 추가하기 때문입니다.

* **범위**: [`Managed`](#scopes). Claude Code는 이 키 또는 정책 키를 전달하는 최우선 소스에서 이 키를 읽고 순위가 낮은 모든 소스에서 이 키를 무시합니다. 따라서 하위 소스는 위의 소스와 결합하도록 자신을 선택할 수 없습니다. Windows HKCU 레지스트리나 [포함 호스트의 부모 설정](/docs/ko/managed-settings#let-an-embedding-host-add-policy)은 병합에 참여하지 않습니다.
* **유형**: string, 다음 중 하나:
  * `"first-wins"`: 정책 키를 전달하는 최우선 소스가 정책을 제공하고 하위 소스는 [Claude Code가 모든 관리자 소스에서 읽는 키](/docs/ko/managed-settings#keys-read-from-every-admin-source)만 제공합니다.
  * `"merge"`: 전달하는 모든 관리자 소스가 아래 규칙으로 결합된 키를 제공합니다.
* **기본값**: `"first-wins"`

배포하는 최우선 소스에 키를 전달합니다. 서버 관리형 설정을 받지 않는 머신은 Claude Code가 이 키 또는 정책 키를 전달하는 최우선 소스에서 키를 읽기 때문에 MDM 프로필에도 키가 필요합니다. `managed-settings.json` 파일은 최하위 순위 관리자 소스이므로 여기에 설정된 `"merge"`는 결합할 아래 소스가 없습니다. 서버 관리형 설정에서 키는 다음과 같습니다:

```json theme={null}
{
  "managedSourcesBehavior": "merge"
}
```

`"merge"` 아래에서 Claude Code는 각 키를 종류별로 결합합니다. 이 표는 각 종류에 대한 규칙을 제공합니다. 제한 허용 목록, 값 전체 취득 및 최우선 소스 전용 행은 포함하는 모든 키의 이름을 지정하고 다른 행은 예를 제공합니다:

| 키의 종류        | Claude Code가 결합하는 방식                                                                                              | 키                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :----------- | :---------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 목록           | 모든 소스의 항목을 결합합니다.                                                                                                 | [`permissions.allow`](#permissions-allow), [`sandbox.network.allowedDomains`](#sandbox-network-alloweddomains) 및 기타 목록 키                                                                                                                                                                                                                                                                                                                                                                                             |
| 잠금           | 모든 소스가 설정하는 가장 엄격한 값을 적용합니다. 어떤 소스도 엄격한 값을 설정하지 않으면 최우선 소스에서만 더 느슨한 값을 적용합니다.                                     | [`allowManagedPermissionRulesOnly`](#allowmanagedpermissionrulesonly), [`permissions.disableBypassPermissionsMode`](#permissions-disablebypasspermissionsmode) 및 기타 boolean 또는 enum 잠금                                                                                                                                                                                                                                                                                                                               |
| 제한 허용 목록     | 하위 소스의 항목을 추가하지 않고 이를 설정하는 최우선 소스에서 전체 목록을 취합니다. 최우선 소스가 설정하지 않으면 다음 소스에서 전체 목록을 취합니다.                            | [`availableModels`](#availablemodels), [`allowedMcpServers`](#allowedmcpservers), [`strictKnownMarketplaces`](#strictknownmarketplaces), [`allowedChannelPlugins`](#allowedchannelplugins) 및 [`fallbackModel`](#fallbackmodel) 체인                                                                                                                                                                                                                                                                                    |
| 값 전체 취득      | 하위 소스의 항목이나 필드를 결합하지 않고 이를 설정하는 최우선 소스에서 전체 값을 취합니다. 최우선 소스가 설정하지 않으면 다음 소스에서 전체 값을 취합니다.                         | [`sandbox.credentials.awsPairs`](#sandbox-credentials-awspairs), [`sandbox.ripgrep`](#sandbox-ripgrep)                                                                                                                                                                                                                                                                                                                                                                                                               |
| 제공된 MCP 서버   | 모든 소스의 서버 이름을 결합합니다. 두 소스가 동일한 이름을 설정하면 상위 소스의 전체 항목을 적용합니다.                                                      | [`managedMcpServers`](#managedmcpservers)                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| 최우선 소스에서만 읽기 | 정책 키를 전달하는 최우선 소스에서만 키를 읽으므로 최우선 소스가 설정하지 않을 때도 하위 소스의 값은 무시됩니다.                                                  | [`apiKeyHelper`](#apikeyhelper), [`awsAuthRefresh`](#awsauthrefresh), [`awsCredentialExport`](#awscredentialexport), [`gcpAuthRefresh`](#gcpauthrefresh), [`otelHeadersHelper`](#otelheadershelper), `proxyAuthHelper`, [`forceLoginOrgUUID`](#forceloginorguuid), [`forceLoginMethod`](#forceloginmethod)의 `"claudeai"` 및 `"console"` 값, [`parentSettingsBehavior`](#parentsettingsbehavior), [`modelPicker`](#modelpicker), [`policyHelper`](#policyhelper), [`permissions.defaultMode`](#permissions-defaultmode) |
| `env`        | [관리자 소스 전체에서 변수별로 병합합니다](/docs/ko/managed-settings#keys-read-from-every-admin-source). `"first-wins"` 및 `"merge"` 모두에서 | [`env`](#env)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| 다른 모든 키      | 이를 설정하는 최우선 소스에서 값을 취합니다.                                                                                         | [`cleanupPeriodDays`](#cleanupperioddays), [`model`](#model)                                                                                                                                                                                                                                                                                                                                                                                                                                                         |

`sandbox.credentials.awsPairs` 및 `sandbox.ripgrep`을 전체로 취득하려면 Claude Code v2.1.257 이상이 필요합니다.

몇 가지 키는 표에 표시되지 않는 조건을 추가합니다:

* **[`policyHelper`](#policyhelper)**: Claude Code는 정책 키를 전달하는 최우선 소스가 MDM 정책이거나 관리형 설정 파일일 때만 이를 인정하므로 서버 관리형 설정에서는 적용되지 않습니다.
* **[`modelOverrides`](#modeloverrides)**: `availableModels`와 쌍을 이룹니다. Claude Code는 이를 설정하는 최우선 소스에서 `modelOverrides`를 취합니다. 단, 상위 소스가 `modelOverrides` 없이 `availableModels`를 설정하는 경우는 제외합니다. 이 경우 모든 소스에서 `modelOverrides`를 무시합니다.
* **[`forceLoginGatewayUrl`](#forcelogingatewayurl), [`gatewayInternalNetworks`](#gatewayinternalnetworks) 및 [`forceLoginMethod`](#forceloginmethod)의 `"gateway"` 값**: Claude Code는 서버 관리형 설정에서 이들을 읽지 않으므로 여기의 값은 MDM 정책이나 관리형 설정 파일에 설정된 값을 적용하거나 숨기지 않습니다. 머신의 관리자 소스 중에서 정책 키를 전달하는 최우선 순위 소스만 이들을 제공합니다. 서버 관리형 설정도 있는지 여부와 관계없이.

머신에서 결합된 소스를 확인하려면 `/status`를 실행하고 [`Setting sources` 행을 읽으십시오](/docs/ko/managed-settings#read-the-source-in-/status).

<h3 id="parentsettingsbehavior">
  `parentSettingsBehavior`
</h3>

Claude Code가 Agent SDK 또는 IDE 확장과 같은 포함 호스트 프로세스에서 제공하는 관리형 설정을 적용할지 선택합니다. 관리자 배포 관리형 계층도 있을 때. `"first-wins"`를 사용하면 Claude Code는 호스트 제공 설정을 삭제합니다. `"merge"`를 사용하면 제한 전용 필터를 통해 관리자 계층 아래에 적용합니다. 호스트가 자신의 제한을 시작하는 세션에 전달해야 할 때 `"merge"`를 설정하십시오. 예를 들어 Claude Desktop이 게이트웨이의 송신 허용 목록을 전달합니다.

* **범위**: [`Managed`](#scopes). Claude Code는 최우선 관리자 제어 관리형 소스에서 이를 읽습니다.
* **유형**: string, 다음 중 하나:
  * `"first-wins"`: Claude Code는 관리자 배포 관리형 계층이 있을 때 호스트 제공 설정을 삭제합니다.
  * `"merge"`: Claude Code는 제한 전용 필터를 통해 관리자 계층 아래에 호스트 제공 설정을 적용합니다.
* **기본값**: `"first-wins"`

```json managed-settings.json theme={null}
{
  "parentSettingsBehavior": "merge"
}
```

관리자 배포 관리형 계층이 없을 때 이 키는 효과가 없습니다. 호스트의 설정은 유일한 관리형 계층으로 적용되며 여전히 제한 값으로 필터링됩니다. 필터의 제한 및 관리형 소스가 상호 작용하는 방식은 [포함 호스트의 부모 설정](/docs/ko/managed-settings#parent-settings-from-embedding-hosts) 및 [부모 설정 제한](/docs/ko/claude-apps-gateway#restrict-parent-settings)을 참조하십시오.

<span id="compute-managed-settings-with-a-policy-helper" />

<h3 id="policyhelper">
  `policyHelper`
</h3>

배포하는 실행 파일을 실행하여 시작 시 관리형 설정을 계산합니다. 정적 파일 대신 디바이스 상태, ID 또는 원격 서비스에서 정책을 파생할 수 있습니다. Claude Code는 첫 번째 프롬프트를 수락하기 전에 도우미를 실행하고 내보내는 설정을 세션의 관리형 설정으로 취급합니다.

* **범위**: [`Managed`](#scopes). macOS plist, Windows HKLM 레지스트리 또는 관리형 설정 파일에서 읽습니다. Claude Code는 [정책 키](/docs/ko/managed-settings#how-claude-code-combines-managed-sources)를 전달하는 최우선 관리형 소스에서 키를 읽고 해당 소스가 이 세 가지 중 하나일 때만 도우미를 실행합니다. 서버 관리형 설정, HKCU 레지스트리 및 호스트 제공 부모 설정에서 키를 무시합니다.
* **유형**: `path`, `timeoutMs` 및 `refreshIntervalMs`가 있는 object
* **기본값**: 설정되지 않음. 따라서 도우미가 실행되지 않습니다.

서버 관리형 설정이 시작 시 정책을 전달하면 도우미의 소스보다 우선하고 도우미는 실행되지 않습니다.

나중에 설정 가져오기가 서버 관리형 설정이 제거되었음을 보고하면 Claude Code는 다음 시작을 기다리지 않고 그 시점에서 도우미를 실행합니다. 그 출력은 세션의 나머지를 관리하고 실패한 실행은 [실패한 시작 실행](#helper-failures)과 동일한 메시지로 세션을 종료합니다.

이 예제는 5초 시간 초과로 도우미를 실행하고 5분마다 다시 실행합니다:

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
  도우미 출력 작성
</h4>

Claude Code는 인수 없이 도우미를 실행하고 환경에 `CLAUDE_CODE_VERSION`을 설정하며 stdout에서 JSON 봉투를 읽습니다. 1 MiB로 제한됩니다.

`managedSettings` 키 아래에 설정을 배치합니다. `managedSettings` 키가 없는 베어 설정 객체는 `managedSettings` undefined로 구문 분석되고 아무것도 적용하지 않으며 Claude Code는 오류를 보고하지 않습니다:

```json theme={null}
{
  "managedSettings": {
    "permissions": { "deny": ["Read(//etc/secrets/**)"] }
  }
}
```

도우미가 `managedSettings`를 내보낼 때 해당 객체는 실행의 유일한 관리형 설정 소스가 됩니다. Claude Code는 MDM, 파일 및 HKCU 소스를 무시하고 도우미의 출력에서만 [교차 소스 키](/docs/ko/managed-settings#keys-read-from-every-admin-source)를 읽으며 [부모 설정](/docs/ko/managed-settings#parent-settings-from-embedding-hosts)을 병합하지 않습니다.

시작 `forceRemoteSettingsRefresh` 검사는 도우미 전에 실행되고 모든 관리자 소스를 읽습니다. 도우미가 `managedSettings`를 생략하는 봉투로 0을 종료하면 관리형 설정을 제공하지 않으며 다른 소스가 평소대로 적용됩니다.

<h4 id="helper-failures">
  도우미 실패
</h4>

도우미 실행이 실패하는 경우:

* `path`가 [`policyHelper.path`](#policyhelper-path)의 규칙을 위반합니다.
* `path`에 일반 파일이 없습니다. Claude Code는 도우미를 시작하기 전에 동일한 `timeoutMs` 예산 내에서 파일을 확인하므로 응답하지 않는 네트워크 마운트로 인해 실행이 실패할 수 있습니다.
* 도우미가 0이 아닌 값으로 종료되거나 `timeoutMs`가 경과할 때 여전히 실행 중이거나 예를 들어 실행 가능하지 않아 시작되지 않습니다.
* 도우미가 stdout 또는 stderr에 1 MiB 이상을 씁니다.
* stdout이 단일 JSON 객체가 아니거나 `managedSettings`에 [Claude Code가 복구할 수 없는 스키마 위반](/docs/ko/managed-settings#find-entries-claude-code-dropped)이 있습니다.

시작 실행이 실패하면 Claude Code는 이유를 인쇄하고 시작을 거부합니다. 0이 아닌 종료 후 이유에는 도우미의 stderr 또는 stderr가 비어 있을 때 stdout이 포함됩니다. 시간 초과 후 이유는 `timeoutMs` 제한의 이름을 지정하고 도우미의 출력을 포함하지 않습니다. 거부는 대화형 세션, `claude -p`, Agent SDK 세션, [백그라운드 세션](/docs/ko/agent-view) 및 대부분의 하위 명령을 포함합니다.

거부는 의도적이므로 중단 복원력이 필요한 도우미는 자신의 캐시에서 제공하고 0을 종료해야 합니다.

백그라운드 새로 고침이 실패하면 Claude Code는 마지막 성공한 정책을 적용 상태로 유지하고 `/status`는 새로 고침이 성공할 때까지 이유와 함께 실패한 새로 고침을 표시합니다. 각 새로 고침은 시작 실행과 동일한 `timeoutMs` 및 실패 규칙에서 실행됩니다.

`--debug`를 사용하면 Claude Code는 모든 실행에서 도우미의 stderr를 [디버그 로그](/docs/ko/debug-your-config)에 씁니다.

Claude Code는 잘못된 `policyHelper` 값을 [삭제된 항목](/docs/ko/managed-settings#find-entries-claude-code-dropped)으로 보고하고 도우미를 실행하지 않고 나머지 관리형 설정에서 세션을 시작합니다. 잘못된 값에는 베어 경로 문자열 및 [최소값](#policyhelper-timeoutms) 아래의 `timeoutMs`가 포함됩니다.

도우미를 끄려면 이를 설정하는 소스에서 키를 제거합니다.

<h3 id="policyhelper-path">
  `policyHelper.path`
</h3>

Claude Code가 실행하는 도우미 실행 파일의 이름을 지정합니다. 경로가 아래 규칙을 위반할 때 발생하는 일은 [도우미 실패](#helper-failures)를 참조하십시오.

* **범위**: [`Managed`](#scopes). [`policyHelper`](#policyhelper)가 읽히는 macOS plist, Windows HKLM 레지스트리 또는 관리형 설정 파일에서 읽습니다.
* **유형**: string, `.` 또는 `..` 세그먼트 없이 정규화된 형식의 절대 경로. Windows에서는 `.exe`로 끝나는 드라이브 문자 또는 UNC 경로
* **기본값**: 없음. `policyHelper`가 설정되면 필수

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

Claude Code가 도우미를 기다리는 시간을 설정합니다. 시간 초과 실행은 0이 아닌 종료와 동일한 방식으로 실패하므로 시작 시 Claude Code는 시작을 거부합니다.

* **범위**: [`Managed`](#scopes). [`policyHelper`](#policyhelper)가 읽히는 macOS plist, Windows HKLM 레지스트리 또는 관리형 설정 파일에서 읽습니다.
* **유형**: integer, 밀리초, 최소 `1000`
* **기본값**: `10000`

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

Claude Code가 백그라운드에서 간격으로 도우미를 다시 실행하여 정책 변경이 실행 중인 세션에 도달하도록 합니다. 새로 고침이 성공하면 그 출력이 이전 관리형 설정을 다시 시작 없이 대체합니다. 새로 고침이 실패하면 Claude Code는 이미 가진 정책을 유지합니다.

* **범위**: [`Managed`](#scopes). [`policyHelper`](#policyhelper)가 읽히는 macOS plist, Windows HKLM 레지스트리 또는 관리형 설정 파일에서 읽습니다.
* **유형**: integer, 밀리초: 새로 고침을 비활성화하려면 `0`, 그렇지 않으면 최소 `60000`
* **기본값**: 설정되지 않음. 따라서 Claude Code는 시작 시 도우미를 한 번 실행합니다.

이 예제는 5분마다 도우미를 다시 실행합니다:

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

WSL의 Claude Code가 Windows 정책 체인에서 관리형 설정을 읽도록 합니다. HKLM 및 Windows 관리형 설정 파일이 `/etc/claude-code` 및 아래의 HKCU보다 우선합니다. 체인이 켜져 있는 동안 Claude Code는 `C:\Program Files\ClaudeCode\` 아래의 관리형 설정 파일이나 드롭인이 [정책 키](/docs/ko/managed-settings#how-claude-code-combines-managed-sources)를 전달하지 않을 때만 `/etc/claude-code`를 읽습니다. Windows에 이미 배포한 정책을 WSL 세션으로 확장하도록 설정하여 동일한 머신의 호스트 세션과 동일한 규칙을 따르도록 합니다. Claude Code는 HKLM 레지스트리 키 또는 `C:\Program Files\ClaudeCode\` 아래의 관리형 설정 파일이나 드롭인에 설정된 경우에만 이를 인정합니다. 둘 다 쓰기 위해 Windows 관리자가 필요합니다.

* **범위**: [`Managed`](#scopes). 관리자 제어 Windows 소스에서.
* **유형**: Boolean
  * `true`: WSL의 Claude Code는 Windows 정책 체인에서 관리형 설정을 읽고 `C:\Program Files\ClaudeCode\` 아래의 관리형 설정 파일이나 드롭인이 [정책 키](/docs/ko/managed-settings#how-claude-code-combines-managed-sources)를 전달하지 않을 때만 `/etc/claude-code`를 읽습니다.
  * `false`: WSL은 `/etc/claude-code`만 읽습니다.
* **기본값**: `false`. WSL은 `/etc/claude-code`만 읽습니다.

```json managed-settings.json theme={null}
{
  "wslInheritsWindowsSettings": true
}
```

관리자 소스가 체인을 켜면 HKCU 정책은 HKCU도 키를 `true`로 설정할 때만 WSL에 참여합니다. 그 복사본은 체인을 자체적으로 켜지 않습니다. 이 키만 포함하는 Windows 소스는 정책 소스로 계산되지 않으므로 하위 우선 소스는 여전히 정책을 제공합니다. 이 키는 네이티브 Windows에 영향을 주지 않습니다.

<h2 id="global-config-settings">
  전역 설정
</h2>

이 키들을 `~/.claude.json`에 저장하세요. 설정 파일에는 저장하지 마세요. Claude Code는 다른 곳의 설정을 무시합니다. Claude Code와 `/config`가 대부분의 설정을 자동으로 작성하며, 수동으로 편집할 수도 있습니다.

<h3 id="autoconnectide">
  `autoConnectIde`
</h3>

외부 터미널에서 Claude Code를 시작할 때 실행 중인 IDE에 자동으로 연결합니다. VS Code 또는 JetBrains 터미널 외부에서 Claude Code를 실행할 때 `/config`에 \*\*IDE에 자동 연결(외부 터미널)\*\*로 표시됩니다.

* **범위**: [`전역 설정`](#scopes)
* **유형**: Boolean
  * `true`: Claude Code가 외부 터미널에서 시작할 때 실행 중인 IDE에 자동으로 연결됩니다
  * `false`: Claude Code가 외부 터미널에서 자동으로 연결되지 않습니다. VS Code 또는 JetBrains 터미널 내부에서 또는 `--ide`를 사용하면 여전히 연결됩니다
* **기본값**: `false`
* **세션별 재정의**: [`CLAUDE_CODE_AUTO_CONNECT_IDE`](/docs/ko/env-vars)가 이 키보다 우선하며, 한 세션 동안 어느 방향이든 적용됩니다

```json ~/.claude.json theme={null}
{
  "autoConnectIde": true
}
```

Claude Code는 `settings.json`에서 이 키를 무시합니다.

<h3 id="autoinstallideextension">
  `autoInstallIdeExtension`
</h3>

VS Code 터미널에서 Claude Code를 실행할 때 Claude Code IDE 확장을 자동으로 설치합니다. VS Code 또는 JetBrains 터미널 내부에서 Claude Code를 실행할 때 `/config`에 **IDE 확장 자동 설치**로 표시됩니다.

* **범위**: [`전역 설정`](#scopes)
* **유형**: Boolean
  * `true`: Claude Code가 VS Code 터미널에서 실행될 때 IDE 확장을 자동으로 설치합니다
  * `false`: Claude Code가 확장을 자동으로 설치하지 않습니다
* **기본값**: `true`
* **세션별 재정의**: [`CLAUDE_CODE_IDE_SKIP_AUTO_INSTALL`](/docs/ko/env-vars)을 `1`로 설정하면 이 키가 `true`일 때도 한 세션 동안 설치를 건너뜁니다

```json ~/.claude.json theme={null}
{
  "autoInstallIdeExtension": false
}
```

Claude Code는 `settings.json`에서 이 키를 무시합니다.

<h3 id="copyonselect">
  `copyOnSelect`
</h3>

[전체 화면 렌더링](/docs/ko/fullscreen#use-the-mouse) 또는 [에이전트 보기](/docs/ko/agent-view)에서 마우스로 텍스트 선택을 완료할 때 자동으로 클립보드에 복사합니다. 전체 화면 렌더링이 켜져 있을 때 `/config`에 **선택 시 복사**로 표시됩니다.

* **범위**: [`전역 설정`](#scopes)
* **유형**: Boolean
  * `true`: Claude Code가 텍스트 선택을 완료할 때 클립보드에 복사합니다
  * `false`: 텍스트 선택이 클립보드를 변경하지 않으며, 대신 [키보드 단축키로 선택 항목을 복사](/docs/ko/fullscreen#use-the-mouse)합니다
* **기본값**: `true`

```json ~/.claude.json theme={null}
{
  "copyOnSelect": false
}
```

Claude Code는 `settings.json`에서 이 키를 무시합니다.

<h3 id="difftool">
  `diffTool`
</h3>

[VS Code](/docs/ko/vs-code) 또는 [JetBrains](/docs/ko/jetbrains#features) IDE가 연결되어 있을 때 Claude Code가 제안하는 `Edit` 또는 `Write` 변경 사항의 diff를 표시할 위치를 선택합니다. `"auto"`는 IDE의 diff 뷰어에서 열고, `"terminal"`은 터미널에 유지합니다. Claude Code가 VS Code 또는 JetBrains IDE에 연결되어 있을 때만 `/config`에 **Diff 도구**로 표시됩니다.

* **범위**: [`전역 설정`](#scopes)
* **유형**: 문자열, 다음 중 하나:
  * `"auto"`: Claude Code가 VS Code 또는 JetBrains IDE에 연결되어 있을 때 IDE의 diff 뷰어에서 diff를 엽니다
  * `"terminal"`: Claude Code가 터미널에 diff를 유지합니다
* **기본값**: `"auto"`

```json ~/.claude.json theme={null}
{
  "diffTool": "terminal"
}
```

Claude Code는 `settings.json`에서 이 키를 무시합니다.

<h3 id="externaleditorcontext">
  `externalEditorContext`
</h3>

`Ctrl+G`를 누르면 Claude Code가 입력 중인 프롬프트를 [외부 편집기](/docs/ko/interactive-mode#general-controls)에서 엽니다. 이 키를 켜면 편집기 버퍼가 Claude의 이전 응답으로 시작되며 `#` 주석 줄로 표시되므로 작성하는 동안 읽을 수 있고, Claude Code는 저장할 때 이 줄들을 제거합니다. `/config`에 **외부 편집기에서 마지막 응답 표시**로 표시됩니다.

* **범위**: [`전역 설정`](#scopes)
* **유형**: Boolean
  * `true`: 편집기 버퍼가 Claude의 이전 응답으로 시작되며 `#` 주석 줄로 표시되고, Claude Code는 저장할 때 이를 제거합니다
  * `false`: 편집기 버퍼가 프롬프트만으로 열립니다
* **기본값**: `false`

```json ~/.claude.json theme={null}
{
  "externalEditorContext": true
}
```

켜져 있을 때 Claude Code가 열 버퍼는 다음과 같으며, 마커 줄 아래의 텍스트만 프롬프트로 전송됩니다:

```text theme={null}
# ─── Claude's last response (for reference; removed on save) ───
# I added the retry loop to fetchUser in src/api.ts and a test
# for the timeout case. Want me to wire the same retry into
# fetchOrders?
# ─── Write your reply below this line ──────────────────────────

Yes, and cap it at three attempts.
```

Claude Code는 응답의 마지막 50줄을 유지하고 `# … (earlier output truncated)`로 자르기를 표시합니다.

Claude Code는 `settings.json`에서 이 키를 무시합니다.

<h3 id="permissionexplainerenabled">
  `permissionExplainerEnabled`
</h3>

<Warning>
  v2.1.257에서 제거되었으며, Bash 및 PowerShell 권한 프롬프트의 `Ctrl+E` 명령 설명도 함께 제거되었습니다. 현재 버전에서 이를 설정해도 효과가 없습니다.
</Warning>

v2.1.256까지는 Bash 또는 PowerShell 권한 프롬프트에서 `Ctrl+E`를 눌러 모델이 생성한 명령 설명을 볼 수 있었으며, 이 키를 `false`로 설정하여 해당 단축키를 끌 수 있었습니다.

* **범위**: [`전역 설정`](#scopes). v2.1.256 이전 버전에서.
* **유형**: Boolean
* **기본값**: `true`

<h3 id="teammatedefaultmodel">
  `teammateDefaultModel`
</h3>

<Warning>
  v2.1.234에서 제거되었으며, `/config` 행 **기본 팀원 모델**도 함께 제거되었습니다. 현재 버전에서 이를 설정해도 효과가 없습니다.
</Warning>

v2.1.233까지는 이 키를 [에이전트 팀](/docs/ko/agent-teams#specify-teammates-and-models) 팀원의 모델로 설정했으며, 프롬프트가 모델을 지정하지 않은 경우: `"sonnet"`과 같은 별칭 또는 리드의 모델을 따르려면 `null`. Claude Code가 현재 이러한 팀원을 위해 선택하는 모델은 [팀원 및 모델 지정](/docs/ko/agent-teams#specify-teammates-and-models)을 참조하세요.

* **범위**: [`전역 설정`](#scopes). v2.1.233 이전 버전에서.
* **유형**: 문자열, 모델 별칭 또는 전체 모델 ID, 또는 `null`
* **기본값**: 설정되지 않음

<h2 id="see-also">
  참고 항목
</h2>

* [권한 구성](/docs/ko/permissions): 규칙 구문, 권한 모드 및 작업 영역 신뢰
* [환경 변수](/docs/ko/env-vars): Claude Code가 읽는 모든 `CLAUDE_*`, `ANTHROPIC_*` 및 공급자 변수
* [Claude에서 사용 가능한 도구](/docs/ko/tools-reference): 기본 제공 도구 및 승인이 필요한 도구
* [설정 파일 예제](/docs/ko/settings-example): 개인 파일, 팀 파일 및 조직의 관리 파일
* [관리 설정 설정](/docs/ko/admin-setup): 조직이 적용할 항목을 결정하는 방법
* [관리 설정 배포](/docs/ko/managed-settings): 전달 메커니즘, 관리 계층 내 우선순위 및 관리 설정의 잘못된 항목
* [구성 디버그](/docs/ko/debug-your-config): `claude doctor` 및 설정 오류 대화 상자
