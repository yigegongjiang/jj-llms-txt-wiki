> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Perpustakaan prompt

> Salin-tempel prompt untuk Claude Code, diberi tag berdasarkan tugas dan peran.

export const PromptLibrary = ({text = {}, labels = {}, tagLabels = {}, phaseLabels = {}, sourceLabels = {}, catLabels = {}}) => {
  const RAW = useMemo(() => [{
    id: 'get-oriented-in-a',
    sdlc: 'discover',
    cat: 'Onboard',
    startN: 1,
    roles: [],
    prompt: 'give me an overview of this codebase: architecture, key directories, and how the pieces connect',
    nextHref: '/en/memory',
    src: 'workflows'
  }, {
    id: 'explain-unfamiliar-code',
    sdlc: 'discover',
    cat: 'Understand',
    roles: [],
    prompt: 'explain what {path} does and how data flows through it. write it up as {format}',
    slots: {
      path: 'src/scheduler/queue.ts',
      format: 'an HTML page with a diagram, then open it in my browser'
    },
    nextHref: '/en/output-styles',
    src: 'workflows'
  }, {
    id: 'find-where-something-happens',
    sdlc: 'discover',
    cat: 'Understand',
    startN: 2,
    roles: [],
    prompt: 'where do we {behavior}?',
    slots: {
      behavior: 'validate uploaded file types'
    },
    src: 'workflows'
  }, {
    id: 'see-what-depends-on',
    sdlc: 'discover',
    cat: 'Understand',
    roles: [],
    prompt: 'what would break if I deleted {target}?',
    slots: {
      target: 'the retryWithBackoff helper'
    },
    src: 'workflows'
  }, {
    id: 'trace-how-code-evolved',
    sdlc: 'discover',
    cat: 'Understand',
    roles: [],
    prompt: 'look through the commit history of {path} and summarize how it evolved and why',
    slots: {
      path: 'internal/auth/session.go'
    },
    src: 'best-practices'
  }, {
    id: 'scope-a-change-before',
    sdlc: 'discover',
    cat: 'Understand',
    roles: ['pm', 'design'],
    prompt: 'which files would I need to touch to {change}?',
    slots: {
      change: 'add a dark mode toggle to settings'
    },
    src: 'teams'
  }, {
    id: 'ask-the-codebase-a',
    sdlc: 'discover',
    cat: 'Understand',
    roles: ['pm'],
    prompt: 'I am a {role}. walk me through what happens when a user {action}, from the UI down to the result',
    slots: {
      role: 'PM',
      action: 'clicks Export to PDF'
    },
    nextHref: '/en/output-styles',
    src: 'teams'
  }, {
    id: 'plan-a-multi-file',
    sdlc: 'design',
    cat: 'Plan',
    roles: ['pm', 'design'],
    prompt: 'plan how to refactor the {target} to {goal}. list the files you would change, but don\'t edit anything yet',
    slots: {
      target: 'payment module',
      goal: 'support multiple currencies'
    },
    src: 'workflows'
  }, {
    id: 'draft-a-spec-by',
    sdlc: 'design',
    cat: 'Plan',
    roles: ['pm'],
    prompt: 'I want to build {feature}. interview me about implementation, UX, edge cases, and tradeoffs until we have covered everything, then write the spec to SPEC.md',
    slots: {
      feature: 'per-workspace rate limits'
    },
    nextHref: '/en/skills',
    src: 'best-practices'
  }, {
    id: 'turn-a-meeting-into',
    sdlc: 'design',
    cat: 'Plan',
    roles: ['pm'],
    prompt: 'read {input} and write up the action items, then create a {tracker} ticket for each with acceptance criteria',
    slots: {
      input: '@meeting-notes.md',
      tracker: 'Linear'
    },
    needs: 'tracker',
    nextHref: '/en/skills',
    src: 'teams'
  }, {
    id: 'map-edge-cases-before',
    sdlc: 'design',
    cat: 'Plan',
    roles: ['design', 'pm'],
    prompt: 'list the error states, empty states, and edge cases for {feature} that the design needs to cover',
    slots: {
      feature: 'the file upload flow'
    },
    src: 'teams'
  }, {
    id: 'turn-a-mockup-into',
    sdlc: 'design',
    cat: 'Prototype',
    roles: ['design', 'pm', 'marketing'],
    paste: 'mockup',
    prompt: 'here is a mockup. build a working prototype I can click through, matching the layout and states shown',
    src: 'teams'
  }, {
    id: 'implement-from-a-screenshot',
    sdlc: 'design',
    cat: 'Prototype',
    roles: ['design'],
    paste: 'design',
    needs: 'browser',
    prompt: 'implement this design, then take a screenshot of the result, compare it to the original, and fix any differences',
    nextHref: '/en/goal',
    src: 'best-practices'
  }, {
    id: 'follow-an-existing-pattern',
    sdlc: 'build',
    cat: 'Implement',
    roles: [],
    prompt: 'look at how {example} is implemented to understand the pattern, then build {new} the same way',
    slots: {
      example: 'the GitHub webhook handler',
      new: 'a Stripe webhook handler'
    },
    nextHref: '/en/memory',
    src: 'best-practices'
  }, {
    id: 'generate-docs-for-code',
    sdlc: 'build',
    cat: 'Implement',
    roles: ['docs'],
    prompt: 'find {scope} without {format} comments and add them, matching the style already used in the file',
    slots: {
      scope: 'the public functions in src/auth/',
      format: 'JSDoc'
    },
    src: 'workflows'
  }, {
    id: 'add-a-small-well',
    sdlc: 'build',
    cat: 'Implement',
    roles: [],
    prompt: 'add a {endpoint} endpoint that returns {payload}',
    slots: {
      endpoint: '/health',
      payload: 'the app version and uptime'
    },
    src: 'workflows'
  }, {
    id: 'build-a-small-internal',
    sdlc: 'build',
    cat: 'Implement',
    roles: ['pm', 'design', 'marketing', 'docs'],
    prompt: 'create a {tool} using HTML, CSS, and vanilla JavaScript, then open it in my browser',
    slots: {
      tool: 'drag-and-drop Kanban board with three columns'
    },
    src: 'teams'
  }, {
    id: 'work-an-issue-end',
    sdlc: 'build',
    cat: 'Implement',
    roles: [],
    prompt: 'read issue #{issue}, implement the fix, and run the tests',
    slots: {
      issue: '312'
    },
    needs: 'gh',
    src: 'workflows'
  }, {
    id: 'find-and-update-copy',
    sdlc: 'build',
    cat: 'Implement',
    roles: ['design', 'docs', 'marketing'],
    prompt: 'find every place we say "{copy}" or a close variant, show me each one in context, then update them all to "{new}". leave tests and the changelog alone',
    slots: {
      copy: 'Sign up free',
      new: 'Start free trial'
    },
    src: 'teams'
  }, {
    id: 'draft-from-past-examples',
    sdlc: 'build',
    cat: 'Implement',
    roles: ['docs', 'marketing', 'pm'],
    prompt: 'read the {examples} in {folder} to learn the structure and voice, then draft a new one for {topic}',
    slots: {
      examples: 'privacy impact assessments',
      folder: 'legal/pia/',
      topic: 'the new analytics integration'
    },
    nextHref: '/en/skills',
    src: 'legal'
  }, {
    id: 'write-tests-run-them',
    sdlc: 'build',
    cat: 'Test',
    startN: 4,
    roles: [],
    prompt: 'write tests for {path}, run them, and fix any failures',
    slots: {
      path: 'app/parsers/feed.py'
    },
    nextHref: '/en/memory',
    src: 'workflows'
  }, {
    id: 'drive-implementation-from-tests',
    sdlc: 'build',
    cat: 'Test',
    roles: [],
    prompt: 'write tests for {feature} first, then implement it until they pass',
    slots: {
      feature: 'the password reset flow'
    },
    src: 'ebook'
  }, {
    id: 'fill-gaps-from-a',
    sdlc: 'build',
    cat: 'Test',
    roles: [],
    prompt: 'read {report} and add tests for the lowest-covered files until each is above {target}%',
    slots: {
      report: 'coverage/coverage-summary.json',
      target: '80'
    },
    nextHref: '/en/goal',
    src: 'workflows'
  }, {
    id: 'migrate-a-pattern-across',
    sdlc: 'build',
    cat: 'Refactor',
    roles: [],
    prompt: 'migrate everything from {from} to {to}: identify every place that needs to change, then make the changes',
    slots: {
      from: 'the old logging API',
      to: 'the structured logger'
    },
    src: 'workflows'
  }, {
    id: 'port-code-between-languages',
    sdlc: 'build',
    cat: 'Refactor',
    roles: [],
    prompt: 'port {source} to {target}, keeping the same {keep}',
    slots: {
      source: 'this Python module',
      target: 'Rust',
      keep: 'public API and test behavior'
    },
    src: 'teams'
  }, {
    id: 'optimize-against-a-measurable',
    sdlc: 'build',
    cat: 'Refactor',
    roles: ['data'],
    prompt: 'optimize {target} to bring {metric} from {current} down to under {goal}',
    slots: {
      target: 'the search query',
      metric: 'p95 latency',
      current: '2s',
      goal: '500ms'
    },
    nextHref: '/en/goal',
    src: 'ebook'
  }, {
    id: 'fix-a-precise-visual',
    sdlc: 'build',
    cat: 'Refactor',
    roles: ['design'],
    prompt: 'the {element} extends {amount} beyond the {container} on {viewport}. fix it.',
    slots: {
      element: 'login button',
      amount: '20px',
      container: 'card border',
      viewport: 'mobile'
    },
    nextHref: '/en/desktop#preview-your-app',
    src: 'ebook'
  }, {
    id: 'review-your-changes-before',
    sdlc: 'build',
    cat: 'Review',
    startN: 5,
    roles: [],
    prompt: 'review my uncommitted changes and flag anything that looks risky before I commit',
    nextHref: '/en/commands',
    src: 'workflows'
  }, {
    id: 'review-a-pull-request',
    sdlc: 'build',
    cat: 'Review',
    roles: [],
    prompt: 'review PR #{pr} and summarize what changed, then list any concerns',
    slots: {
      pr: '247'
    },
    needs: 'gh',
    nextHref: '/en/code-review',
    src: 'workflows'
  }, {
    id: 'review-infrastructure-changes-before',
    sdlc: 'build',
    cat: 'Review',
    roles: ['security', 'ops'],
    paste: 'plan',
    prompt: 'here is my Terraform plan output. what is this going to do, and is anything here going to cause problems?',
    src: 'teams'
  }, {
    id: 'run-a-security-review',
    sdlc: 'build',
    cat: 'Review',
    roles: ['security'],
    prompt: 'use a subagent to review {path} for security issues and report what it finds',
    slots: {
      path: 'src/api/'
    },
    nextHref: '/en/sub-agents',
    src: 'best-practices'
  }, {
    id: 'review-content-before-sending',
    sdlc: 'build',
    cat: 'Review',
    roles: ['marketing', 'docs'],
    prompt: 'review {file} for {concerns} and list anything I should fix before it goes to {reviewer}',
    slots: {
      file: 'launch-post.md',
      concerns: 'unsupported claims, missing attributions, and brand-guideline issues',
      reviewer: 'legal'
    },
    nextHref: '/en/skills',
    src: 'legal'
  }, {
    id: 'course-correct-a-wrong',
    sdlc: 'build',
    cat: 'Steer',
    roles: [],
    prompt: 'that is not right: {feedback}. try a different approach',
    slots: {
      feedback: 'the function signature needs to stay backward-compatible'
    },
    nextHref: '/en/checkpointing',
    src: 'best-practices'
  }, {
    id: 'narrow-the-scope-of',
    sdlc: 'build',
    cat: 'Steer',
    roles: [],
    prompt: 'that is too much. keep only the changes to {scope} and undo your other edits',
    slots: {
      scope: 'the validation logic in src/forms/'
    },
    src: 'best-practices'
  }, {
    id: 'turn-a-correction-into',
    sdlc: 'build',
    cat: 'Steer',
    roles: [],
    prompt: 'you keep {mistake}. add a rule to CLAUDE.md so this stops happening',
    slots: {
      mistake: 'using default exports when this project uses named exports'
    },
    nextHref: '/en/memory',
    src: 'best-practices'
  }, {
    id: 'resolve-merge-conflicts',
    sdlc: 'ship',
    cat: 'Git',
    roles: [],
    prompt: 'resolve the merge conflicts in this branch and explain what you kept from each side',
    src: 'workflows'
  }, {
    id: 'commit-with-a-generated',
    sdlc: 'ship',
    cat: 'Git',
    roles: [],
    prompt: 'commit these changes with a message that summarizes what I did',
    src: 'workflows'
  }, {
    id: 'open-a-pull-request',
    sdlc: 'ship',
    cat: 'Git',
    roles: [],
    prompt: 'find the {tracker} ticket about {topic} and open a PR that implements it',
    slots: {
      tracker: 'Linear',
      topic: 'the login timeout'
    },
    needs: 'tracker',
    src: 'workflows'
  }, {
    id: 'draft-release-notes-from',
    sdlc: 'ship',
    cat: 'Release',
    roles: ['pm', 'docs', 'marketing'],
    prompt: 'compare {from} to {to} and draft release notes grouped by feature, fix, and breaking change',
    slots: {
      from: 'v2.3.0',
      to: 'v2.4.0'
    },
    nextHref: '/en/skills',
    src: 'workflows'
  }, {
    id: 'write-a-ci-workflow',
    sdlc: 'ship',
    cat: 'Release',
    roles: ['ops'],
    prompt: 'write a GitHub Actions workflow that {steps} on every push to {branch}',
    slots: {
      steps: 'runs the tests and deploys to staging',
      branch: 'main'
    },
    src: 'workflows'
  }, {
    id: 'find-and-fix-a',
    sdlc: 'operate',
    cat: 'Debug',
    startN: 3,
    roles: [],
    prompt: 'the {test} test is failing, find out why and fix it',
    slots: {
      test: 'UserAuth'
    },
    src: 'workflows'
  }, {
    id: 'investigate-a-reported-error',
    sdlc: 'operate',
    cat: 'Debug',
    roles: ['ops'],
    prompt: 'users are seeing {symptom} on {where}. investigate and tell me what is going on',
    slots: {
      symptom: '500 errors',
      where: '/api/settings'
    },
    nextHref: '/en/web-quickstart#pre-fill-sessions',
    src: 'workflows'
  }, {
    id: 'fix-a-build-error',
    sdlc: 'operate',
    cat: 'Debug',
    roles: ['ops'],
    paste: 'error',
    prompt: 'here is a build error. fix the root cause and verify the build succeeds',
    src: 'best-practices'
  }, {
    id: 'investigate-a-production-incident',
    sdlc: 'operate',
    cat: 'Incident',
    roles: ['ops', 'security'],
    prompt: '{symptom}. check the logs, recent deploys, and config changes, then tell me the most likely cause',
    slots: {
      symptom: 'the checkout endpoint started returning 500s an hour ago'
    },
    nextHref: '/en/mcp',
    src: 'workflows'
  }, {
    id: 'diagnose-from-a-console',
    sdlc: 'operate',
    cat: 'Incident',
    roles: ['ops', 'data'],
    paste: 'screenshot',
    prompt: 'here is a screenshot of {console}. walk me through why {resource} is failing and give me the exact commands to fix it',
    slots: {
      console: 'the GCP Kubernetes dashboard',
      resource: 'this pod'
    },
    src: 'teams'
  }, {
    id: 'query-logs-in-plain',
    sdlc: 'operate',
    cat: 'Incident',
    roles: ['security', 'ops', 'data'],
    prompt: 'show me all {events} for {scope} over {timeframe}. write the query, run it, and tell me what stands out',
    slots: {
      events: 'failed logins',
      scope: 'the auth service',
      timeframe: 'the past 24 hours'
    },
    needs: 'db',
    src: 'cybersecurity'
  }, {
    id: 'analyze-a-data-file',
    sdlc: 'operate',
    cat: 'Data',
    roles: ['data', 'pm', 'marketing'],
    paste: 'csv',
    prompt: 'read {file}, summarize the key patterns, and write the results to {output}',
    slots: {
      file: '@reports/q1-signups.csv',
      output: 'an HTML page with charts, then open it in my browser'
    },
    nextHref: '/en/mcp',
    src: 'teams'
  }, {
    id: 'generate-variations-from-performance',
    sdlc: 'operate',
    cat: 'Data',
    roles: ['marketing', 'data'],
    paste: 'csv',
    prompt: 'read {file}, find the underperforming {items}, and generate {n} new variations that stay under {limit} characters',
    slots: {
      file: '@ads-performance.csv',
      items: 'headlines',
      n: '20',
      limit: '90'
    },
    nextHref: '/en/mcp',
    src: 'teams'
  }, {
    id: 'turn-a-recurring-task',
    sdlc: 'operate',
    cat: 'Automate',
    roles: [],
    prompt: 'create a /{name} skill for this project that {steps}',
    slots: {
      name: 'ship',
      steps: 'runs the linter and tests, then drafts a commit message'
    },
    src: 'workflows'
  }, {
    id: 'add-a-hook-for',
    sdlc: 'operate',
    cat: 'Automate',
    roles: [],
    prompt: 'write a hook that {action} after every {event}',
    slots: {
      action: 'runs prettier',
      event: 'edit to a .ts or .tsx file'
    },
    src: 'best-practices'
  }, {
    id: 'connect-a-tool-with',
    sdlc: 'operate',
    cat: 'Automate',
    roles: [],
    prompt: 'set up the {server} MCP server so you can read my {data} directly',
    slots: {
      server: 'Sentry',
      data: 'error reports'
    },
    src: 'workflows'
  }, {
    id: 'capture-what-to-remember',
    sdlc: 'operate',
    cat: 'Automate',
    roles: ['pm', 'docs'],
    prompt: 'summarize what we did this session and suggest what to add to CLAUDE.md',
    src: 'teams'
  }], []);
  const PROMPTS = useMemo(() => {
    if (typeof window !== 'undefined') {
      const rawIds = new Set(RAW.map(p => p.id));
      RAW.forEach(p => {
        if (!text[p.id]) console.warn('[prompt-library] no text[] entry for id:', p.id);
      });
      Object.keys(text).forEach(k => {
        if (!rawIds.has(k)) console.warn('[prompt-library] orphaned text[] key:', k);
      });
    }
    return RAW.map(p => ({
      ...p,
      title: p.id,
      teaches: '',
      ...text[p.id] || ({})
    }));
  }, [RAW, text]);
  const L = labels;
  const TL = k => tagLabels[k] || k;
  const CAT_TAG = useMemo(() => ({
    Onboard: 'understand',
    Understand: 'understand',
    Plan: 'plan',
    Prototype: 'prototype',
    Implement: 'build',
    Test: 'test',
    Refactor: 'refactor',
    Review: 'review',
    Steer: 'steer',
    Git: 'git',
    Release: 'release',
    Debug: 'debug',
    Incident: 'debug',
    Data: 'data',
    Automate: 'automate'
  }), []);
  const TAGS = useMemo(() => ['understand', 'plan', 'prototype', 'build', 'test', 'refactor', 'review', 'steer', 'debug', 'git', 'release', 'data', 'automate', 'pm', 'design', 'docs', 'marketing', 'security', 'ops'], []);
  const tagsOf = p => [CAT_TAG[p.cat], ...p.roles || []];
  const doc = useMemo(() => {
    const p = typeof window !== 'undefined' ? window.location.pathname : '';
    const base = p.startsWith('/docs/') ? '/docs' : '';
    const m = p.slice(base.length).match(/^\/([a-z]{2}(?:-[A-Z]{2})?)\//);
    const locale = m ? m[1] : 'en';
    return href => {
      if (!href || href[0] !== '/' || href[1] === '/') return href;
      return base + (href.startsWith('/en/') ? '/' + locale + href.slice(3) : href);
    };
  }, []);
  const linkify = s => {
    const out = [];
    let last = 0;
    const re = /\[([^\]]+)\]\(([^)]+)\)/g;
    for (let m; m = re.exec(s); ) {
      if (m.index > last) out.push(s.slice(last, m.index));
      out.push(<a key={m.index} href={doc(m[2])}>{m[1]}</a>);
      last = re.lastIndex;
    }
    if (last < s.length) out.push(s.slice(last));
    return out;
  };
  const codeify = s => s.split(/(`[^`]+`)/g).map((part, i) => part[0] === '`' ? <code key={i}>{part.slice(1, -1)}</code> : part);
  const SOURCES = useMemo(() => ({
    'workflows': '/en/common-workflows',
    'teams': 'https://claude.com/blog/how-anthropic-teams-use-claude-code',
    'legal': 'https://claude.com/blog/how-anthropic-uses-claude-legal',
    'cybersecurity': 'https://claude.com/blog/how-anthropic-uses-claude-cybersecurity',
    'best-practices': '/en/best-practices',
    'ebook': 'https://resources.anthropic.com/hubfs/Scaling%20agentic%20coding%20across%20your%20organization.pdf'
  }), []);
  const [mounted, setMounted] = useState(false);
  const [q, setQ] = useState('');
  const [start, setStart] = useState(true);
  const [sel, setSel] = useState(null);
  const [openId, setOpenId] = useState(null);
  const [copied, setCopied] = useState(null);
  const [fills, setFills] = useState({});
  const copyTimer = useRef(null);
  useEffect(() => {
    setMounted(true);
    return () => clearTimeout(copyTimer.current);
  }, []);
  const setFill = (id, key, val) => setFills(f => ({
    ...f,
    [id + '.' + key]: val
  }));
  const fillOf = (p, key) => {
    const v = fills[p.id + '.' + key];
    return v !== undefined ? v : p.slots && p.slots[key] !== undefined ? p.slots[key] : '';
  };
  const assemble = p => p.prompt.replace(/\{(\w+)\}/g, (_, k) => fillOf(p, k) || p.slots && p.slots[k] || k);
  const preview = p => p.prompt.replace(/\{(\w+)\}/g, (_, k) => p.slots && p.slots[k] || k);
  const bodyText = p => preview(p) + ' ' + p.teaches.replace(/\[([^\]]+)\]\([^)]+\)/g, '$1') + ' ' + (p.next || '');
  const widthFor = s => (s || '').length + 3 + 'ch';
  const ql = q.trim().toLowerCase();
  const toggleTag = k => {
    setStart(false);
    setSel(s => !ql && s === k ? null : k);
  };
  const clear = () => {
    setStart(false);
    setSel(null);
    setQ('');
  };
  const results = useMemo(() => {
    const list = PROMPTS.filter(p => {
      if (ql) return p.title.toLowerCase().includes(ql) || bodyText(p).toLowerCase().includes(ql);
      if (start) return !!p.startN;
      if (sel) return tagsOf(p).includes(sel);
      return true;
    });
    if (ql) return list;
    if (start) return list.sort((a, b) => a.startN - b.startN);
    if (sel) return list.sort((a, b) => (a.roles || []).length - (b.roles || []).length || (b.sdlc === 'operate') - (a.sdlc === 'operate'));
    return list;
  }, [PROMPTS, ql, start, sel]);
  const matchSnippet = p => {
    if (!ql || p.title.toLowerCase().includes(ql)) return null;
    const txt = bodyText(p);
    const at = txt.toLowerCase().indexOf(ql);
    if (at < 0) return null;
    const lo = Math.max(0, at - 30), hi = Math.min(txt.length, at + ql.length + 50);
    return [lo > 0 ? '…' : '', txt.slice(lo, at), <mark key="m">{txt.slice(at, at + ql.length)}</mark>, txt.slice(at + ql.length, hi), hi < txt.length ? '…' : ''];
  };
  const grouped = useMemo(() => {
    if (start && !q.trim()) return [];
    const g = {};
    for (const p of results) {
      const key = p.sdlc + '|' + p.cat;
      (g[key] = g[key] || ({
        sdlc: p.sdlc,
        cat: p.cat,
        items: []
      })).items.push(p);
    }
    return Object.values(g);
  }, [results, start, q]);
  const copy = async (str, id) => {
    try {
      await navigator.clipboard.writeText(str);
    } catch {
      const ta = document.createElement('textarea');
      ta.value = str;
      ta.setAttribute('readonly', '');
      ta.style.position = 'fixed';
      ta.style.opacity = '0';
      document.body.appendChild(ta);
      ta.select();
      document.execCommand('copy');
      document.body.removeChild(ta);
    }
    clearTimeout(copyTimer.current);
    setCopied(id);
    copyTimer.current = setTimeout(() => setCopied(null), 1600);
  };
  const promptBody = p => {
    if (!p.slots) return <code>{p.prompt}</code>;
    const parts = p.prompt.split(/(\{\w+\})/g);
    return <code>
        {parts.map((part, idx) => {
      const m = part.match(/^\{(\w+)\}$/);
      if (!m) return <span key={idx}>{part}</span>;
      const k = m[1];
      const val = fillOf(p, k);
      return <input key={idx} type="text" className="pl-slot" value={val} placeholder={p.slots[k] || k} aria-label={k} style={{
        width: widthFor(val || p.slots[k])
      }} onChange={e => setFill(p.id, k, e.target.value)} onFocus={e => e.target.select()} onClick={e => e.stopPropagation()} />;
    })}
      </code>;
  };
  const card = p => {
    const open = openId === p.id;
    const srcHref = SOURCES[p.src];
    const srcLabel = sourceLabels[p.src];
    const snip = matchSnippet(p);
    return <div key={p.id} className={'pl-card' + (open ? ' pl-open' : '')}>
        <button type="button" className="pl-head" onClick={() => setOpenId(open ? null : p.id)} aria-expanded={open}>
          <span className="pl-title">{p.title}</span>
          {!!p.startN && <span className="pl-chip">{L.startHere} · {p.startN}</span>}
        </button>
        {snip ? <div className="pl-match">{snip}</div> : <code className="pl-prompt-preview">{preview(p)}</code>}
        {open && <div className="pl-body">
            <div className="pl-label">{p.slots ? L.fillAndCopy : L.copyThis}</div>
            {p.needs && L.needs && L.needs[p.needs] && <div className="pl-hint pl-needs">
                <span className="pl-needs-label">{L.needsLabel}</span> {linkify(L.needs[p.needs])}
              </div>}
            {p.paste && L.paste && L.paste[p.paste] && <div className="pl-hint pl-paste">{L.paste[p.paste]}</div>}
            {p.slots && <div className="pl-hint">
                {L.hintBefore} <span className="pl-hint-chip">{L.hintChip}</span> {L.hintAfter}
              </div>}
            <div className="pl-prompt-box">
              <span className="pl-caret">{'❯'}</span>
              {promptBody(p)}
              <button type="button" className="pl-copy" onClick={() => copy(assemble(p), p.id)}>
                {copied === p.id ? L.copied : L.copy}
              </button>
            </div>
            <div className="pl-label">{L.whyWorks}</div>
            <div className="pl-teaches">{linkify(p.teaches)}</div>
            {p.nextHref && p.next && <div className="pl-next">
                <span className="pl-next-label">{L.makeItStick}</span>
                <a href={doc(p.nextHref)}>{codeify(p.next)} →</a>
              </div>}
            {srcLabel && <div className="pl-src">{L.from} {srcHref ? <a href={doc(srcHref)}>{srcLabel}</a> : srcLabel}</div>}
          </div>}
      </div>;
  };
  const STYLES = useMemo(() => `
.pl {
  --pl-accent: #D97757;
  --pl-accent-bg: rgba(217,119,87,0.07);
  --pl-bg: #fff;
  --pl-surface: #FAFAF7;
  --pl-border: #E8E6DC;
  --pl-border-subtle: rgba(31,30,29,0.08);
  --pl-text: #141413;
  --pl-text-2: #5E5D59;
  --pl-text-3: #73726C;
  --pl-text-4: #9C9A92;
  --pl-mono: var(--font-mono, ui-monospace, SFMono-Regular, Menlo, monospace);
  font-family: 'Anthropic Sans', -apple-system, BlinkMacSystemFont, sans-serif;
  font-size: 16px; color: var(--pl-text); margin: 8px 0 32px;
}
.dark .pl {
  --pl-bg: #1f1e1d;
  --pl-surface: #262624;
  --pl-border: #3d3d3a;
  --pl-border-subtle: rgba(240,238,230,0.08);
  --pl-text: #f0eee6;
  --pl-text-2: #bfbdb4;
  --pl-text-3: #91908a;
  --pl-text-4: #73726c;
}
.pl *, .pl *::before, .pl *::after { box-sizing: border-box; }
.pl button { font-family: inherit; cursor: pointer; }
.pl a { color: var(--pl-accent); text-decoration: none; }
.pl a:hover { text-decoration: underline; }

.pl-search {
  display: flex; align-items: center; gap: 10px;
  padding: 14px 18px; background: var(--pl-surface);
  border: 1px solid var(--pl-border); border-radius: 12px;
  margin-bottom: 14px;
}
.pl-search input {
  flex: 1; border: none; outline: none; background: transparent;
  font-size: 16px; color: var(--pl-text);
}
.pl-search input::placeholder { color: var(--pl-text-4); }

.pl-tags { display: flex; gap: 8px; flex-wrap: wrap; align-items: center; margin-bottom: 18px; }
.pl-tag {
  padding: 7px 14px; border: 1px solid var(--pl-border); background: var(--pl-bg);
  font-size: 14px; color: var(--pl-text-2); border-radius: 999px;
}
.pl-tag:hover { background: var(--pl-surface); }
.pl-tag.pl-on { background: var(--pl-text); border-color: var(--pl-text); color: var(--pl-bg); }
.pl-tag.pl-start { color: var(--pl-accent); font-weight: 500; }
.pl-tag.pl-start.pl-on { background: var(--pl-accent); border-color: var(--pl-accent); color: #fff; }
.pl-tags.pl-dim .pl-tag { opacity: 0.5; }
.pl-tags.pl-dim .pl-tag:hover { opacity: 1; }
.pl-sep { width: 1px; height: 22px; background: var(--pl-border); margin: 0 4px; }
.pl-clear { border: none; background: none; font-size: 13px; color: var(--pl-text-4); padding: 4px 6px; }
.pl-clear:hover { color: var(--pl-text-2); }
.pl-count { margin-left: auto; font-size: 14px; color: var(--pl-text-4); }

.pl-group-h {
  font-size: 12px; letter-spacing: 0.08em; text-transform: uppercase;
  color: var(--pl-text-4); margin: 24px 0 12px;
}
.pl-group-h .pl-phase { color: var(--pl-text-3); }
.pl-card {
  border: 1px solid var(--pl-border-subtle); border-radius: 10px;
  margin-bottom: 12px; background: var(--pl-bg); overflow: hidden;
  padding: 14px 18px;
}
.pl-card.pl-open { border-color: var(--pl-border); background: var(--pl-surface); }
.pl-head {
  width: 100%; display: flex; align-items: baseline; gap: 12px;
  border: none; background: transparent; text-align: left; padding: 0;
}
.pl-head:focus-visible { outline: 2px solid var(--pl-accent); outline-offset: 2px; border-radius: 6px; }
.pl-title {
  flex: 1; font-size: 17px; font-weight: 500; color: var(--pl-text);
  white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
}
.pl-prompt-preview {
  display: block; font-family: var(--pl-mono); font-size: 13.5px; color: var(--pl-text-3);
  margin-top: 6px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
}
.pl-chip {
  font-size: 11px; letter-spacing: 0.05em; text-transform: uppercase;
  padding: 3px 9px; border-radius: 999px; flex-shrink: 0;
  background: var(--pl-accent-bg); color: var(--pl-accent);
}

.pl-body { margin-top: 14px; padding-top: 14px; border-top: 1px solid var(--pl-border-subtle); }
.pl-label {
  font-size: 11.5px; letter-spacing: 0.08em; text-transform: uppercase;
  color: var(--pl-text-4); margin: 12px 0 8px;
}
.pl-prompt-box {
  display: flex; align-items: center; gap: 10px;
  padding: 14px 16px; background: #141413; color: #f0eee6;
  border-radius: 8px; font-family: var(--pl-mono); font-size: 15px;
}
.pl-caret { color: var(--pl-accent); flex-shrink: 0; }
.pl-prompt-box code { flex: 1; background: none; padding: 0; color: inherit; white-space: pre-wrap; line-height: 1.9; }
.pl-slot {
  font-family: var(--pl-mono); font-size: inherit;
  background: rgba(217,119,87,0.15); color: #f0eee6;
  border: none; border-bottom: 1.5px dashed var(--pl-accent);
  border-radius: 4px 4px 0 0; padding: 2px 6px; margin: 0 1px;
  outline: none; min-width: 6ch; max-width: 100%;
  box-sizing: content-box; cursor: text;
}
.pl-slot:hover { background: rgba(217,119,87,0.22); }
.pl-slot:focus { background: rgba(217,119,87,0.28); border-bottom-style: solid; }
.pl-slot::placeholder { color: rgba(240,238,230,0.4); font-style: italic; }
.pl-hint { font-size: 14px; color: var(--pl-text-3); margin: 0 0 10px; }
.pl-paste { color: var(--pl-text-2); }
.pl-needs { color: var(--pl-text-2); }
.pl-needs-label {
  display: inline-block; font-size: 10.5px; letter-spacing: 0.06em;
  text-transform: uppercase; padding: 2px 7px; margin-right: 6px;
  border-radius: 4px; background: var(--pl-accent-bg); color: var(--pl-accent);
}
.pl-hint-chip {
  font-family: var(--pl-mono); font-size: 0.92em;
  background: var(--pl-accent-bg); color: var(--pl-accent);
  border-bottom: 1.5px dashed var(--pl-accent);
  border-radius: 3px 3px 0 0; padding: 1px 5px;
}
.pl-copy {
  font-size: 12.5px; padding: 6px 12px; border-radius: 6px;
  background: var(--pl-accent); color: #fff; border: none; flex-shrink: 0;
}
.pl-teaches { display: block; font-size: 15.5px; color: var(--pl-text-2); margin: 4px 0 0; line-height: 1.6; }
.pl-match {
  display: block; font-size: 13.5px; color: var(--pl-text-3);
  margin-top: 6px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
}
.pl-match mark { background: var(--pl-accent-bg); color: var(--pl-text); padding: 1px 2px; border-radius: 3px; }
.pl-next {
  display: flex; align-items: baseline; gap: 10px;
  margin: 14px 0 0; padding: 10px 12px;
  background: var(--pl-accent-bg); border-radius: 8px; font-size: 14.5px;
}
.pl-next-label {
  font-size: 11px; letter-spacing: 0.06em; text-transform: uppercase;
  color: var(--pl-accent); font-weight: 600; flex-shrink: 0;
}
.pl-src { display: block; font-size: 14px; color: var(--pl-text-4); margin: 14px 0 0; }

.pl-show-all {
  display: block; width: 100%; padding: 14px; margin-top: 4px;
  border: 1px dashed var(--pl-border); border-radius: 10px;
  background: transparent; font-size: 15px; color: var(--pl-accent);
  text-align: center;
}
.pl-show-all:hover { background: var(--pl-accent-bg); border-style: solid; }

.pl-empty {
  padding: 32px; text-align: center; color: var(--pl-text-4);
  border: 1px dashed var(--pl-border); border-radius: 10px;
}
`, []);
  if (!mounted) return <div className="pl" style={{
    minHeight: 480
  }} />;
  return <div className="pl">
      <style>{STYLES}</style>

      <div className="pl-search">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" style={{
    color: 'var(--pl-text-4)'
  }}>
          <circle cx="11" cy="11" r="7" /><line x1="21" y1="21" x2="16.65" y2="16.65" />
        </svg>
        <input type="text" placeholder={L.search} value={q} onChange={e => {
    setQ(e.target.value);
    if (e.target.value) setStart(false);
  }} aria-label={L.search} />
      </div>

      <div className={'pl-tags' + (ql ? ' pl-dim' : '')}>
        <button type="button" className={'pl-tag pl-start' + (!ql && start ? ' pl-on' : '')} onClick={() => {
    setQ('');
    setStart(!start);
    if (!start) setSel(null);
  }}>
          ★ {L.startHere}
        </button>
        <span className="pl-sep" />
        {TAGS.map(k => <button key={k} type="button" aria-pressed={!ql && sel === k} className={'pl-tag' + (!ql && sel === k ? ' pl-on' : '')} onClick={() => {
    setQ('');
    toggleTag(k);
  }}>
            {TL(k)}
          </button>)}
        {(start || sel || q) && <button type="button" className="pl-clear" onClick={clear}>{L.clear}</button>}
        <span className="pl-count">{results.length} {results.length === 1 ? L.prompt : L.prompts}</span>
      </div>

      {results.length === 0 ? <div className="pl-empty">
          {L.noMatch} {ql ? <code>{q}</code> : null} <button type="button" className="pl-clear" onClick={clear}>{L.clear}</button>
        </div> : !ql && start ? <div>
          <div className="pl-group-h">{L.startHereHeader}</div>
          {results.map(card)}
          <button type="button" className="pl-show-all" onClick={clear}>
            {L.showAll && L.showAll.replace('{n}', PROMPTS.length)} →
          </button>
        </div> : grouped.map(g => <div key={g.sdlc + '|' + g.cat}>
            <div className="pl-group-h"><span className="pl-phase">{phaseLabels[g.sdlc] || g.sdlc}</span> · {catLabels[g.cat] || g.cat}</div>
            {g.items.map(card)}
          </div>)}
    </div>;
};

Ini adalah perpustakaan prompt untuk disalin ke Claude Code. Gunakan untuk mengeksplorasi cara kerja yang belum pernah Anda coba, atau ketika Anda tidak yakin harus mulai dari mana.

Prompt dikumpulkan dari berbagai panduan Anthropic, termasuk [Common workflows](/docs/id/common-workflows), [Best practices](/docs/id/best-practices), dan [How Anthropic teams use Claude Code](https://claude.com/blog/how-anthropic-teams-use-claude-code). Ini adalah titik awal, bukan skrip. Buka **Why this works** di bawah prompt apa pun untuk melihat pola di baliknya sehingga Anda dapat menulis prompt Anda sendiri.

export const labels = {
  startHere: "Mulai dari sini",
  startHereHeader: "Lima prompt untuk dicoba terlebih dahulu",
  showAll: "Tampilkan semua {n} prompt",
  search: "Cari prompt…",
  clear: "Hapus",
  prompt: "prompt",
  prompts: "prompt",
  noMatch: "Tidak ada prompt yang cocok",
  fillAndCopy: "Isi dan salin",
  copyThis: "Salin prompt ini",
  hintBefore: "Ketik di",
  hintChip: "bidang yang disorot",
  hintAfter: "untuk menyesuaikan, lalu salin.",
  copy: "Salin",
  copied: "Disalin",
  whyWorks: "Mengapa ini berhasil",
  makeItStick: "Buat tetap berkesan",
  from: "Dari",
  paste: {
    mockup: "Tempel, seret, atau @-mention gambar mockup Anda, lalu kirim ini:",
    design: "Tempel, seret, atau @-mention gambar desain Anda, lalu kirim ini:",
    screenshot: "Tempel, seret, atau @-mention tangkapan layar Anda, lalu kirim ini:",
    plan: "Tempel output rencana Anda ke prompt terlebih dahulu, lalu kirim ini:",
    error: "Tempel output kesalahan ke prompt terlebih dahulu, lalu kirim ini:",
    csv: "Seret file Anda ke prompt, atau ganti jalur di bawah dengan @-mention milik Anda sendiri:"
  },
  needsLabel: "Membutuhkan",
  needs: {
    tracker: "pelacak masalah Anda ditambahkan sebagai [konektor claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai) atau [server MCP](/docs/id/mcp).",
    gh: "[gh CLI](https://cli.github.com) yang diautentikasi, atau GitHub ditambahkan sebagai [konektor claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai).",
    browser: "cara bagi Claude untuk merender dan mengambil tangkapan layar hasilnya. [Aplikasi Desktop](/docs/id/desktop#preview-your-app) memiliki ini bawaan. Di terminal, instal [ekstensi Chrome](/docs/id/chrome) atau server MCP [Playwright](/docs/id/mcp).",
    db: "gudang data atau penyimpanan log Anda ditambahkan sebagai [konektor claude.ai](/docs/id/mcp#use-mcp-servers-from-claude-ai) atau [server MCP](/docs/id/mcp)."
  }
};

export const tagLabels = {
  understand: "Pahami",
  plan: "Rencanakan",
  prototype: "Prototipe",
  build: "Bangun",
  test: "Uji",
  refactor: "Refaktor",
  review: "Tinjau",
  steer: "Kemudi",
  debug: "Debug",
  git: "Git",
  release: "Rilis",
  data: "Data",
  automate: "Otomatisasi",
  pm: "Produk",
  design: "Desain",
  docs: "Dokumen",
  marketing: "Pemasaran",
  security: "Keamanan",
  ops: "On-call"
};

export const phaseLabels = {
  discover: "Temukan",
  design: "Desain",
  build: "Bangun",
  ship: "Kirim",
  operate: "Operasikan"
};

export const sourceLabels = {
  workflows: "Common workflows",
  teams: "How Anthropic teams use Claude Code",
  legal: "How Anthropic uses Claude in Legal",
  cybersecurity: "How Anthropic uses Claude in Cybersecurity",
  "best-practices": "Best practices",
  ebook: "Scaling agentic coding guide"
};

export const catLabels = {
  Onboard: "Onboard",
  Understand: "Pahami",
  Plan: "Rencanakan",
  Prototype: "Prototipe",
  Implement: "Implementasikan",
  Test: "Uji",
  Refactor: "Refaktor",
  Review: "Tinjau",
  Steer: "Kemudi",
  Git: "Git",
  Release: "Rilis",
  Debug: "Debug",
  Incident: "Insiden",
  Data: "Data",
  Automate: "Otomatisasi"
};

export const text = {
  "get-oriented-in-a": {
    title: "Orientasi diri di repositori baru",
    teaches: "Jelaskan apa yang ingin Anda ketahui, bukan file mana yang harus dibaca. Claude mengeksplorasi proyek sendiri dan mengembalikan ringkasan tentang bagaimana semuanya cocok bersama.",
    next: "Jalankan `/init` untuk menyiapkan `CLAUDE.md` sehingga Claude mengingat ini setiap sesi"
  },
  "explain-unfamiliar-code": {
    title: "Jelaskan kode yang tidak familiar",
    teaches: "Sebutkan file dan katakan format apa yang Anda inginkan jawabannya. Tukar halaman HTML dengan diagram, poin-poin, atau apa pun yang sesuai dengan cara Anda belajar.",
    next: "Atur gaya output sehingga Claude selalu menjelaskan dalam format pilihan Anda"
  },
  "find-where-something-happens": {
    title: "Temukan di mana sesuatu terjadi",
    teaches: "Cari berdasarkan perilaku, bukan nama file. Pencarian bekerja bahkan ketika Anda tidak tahu apa nama file atau direktori mana yang ditempatinya."
  },
  "see-what-depends-on": {
    title: "Periksa apa yang rusak sebelum Anda menghapus",
    teaches: "Tanya sebelum Anda menghapus apa pun. Daftar pemanggil dan efek hilir memberi tahu Anda apakah Anda melihat pembersihan satu baris atau perubahan yang perlu Anda koordinasikan."
  },
  "trace-how-code-evolved": {
    title: "Lacak bagaimana kode berkembang",
    teaches: "Tunjuk riwayat komit ketika pertanyaannya adalah mengapa, bukan apa. Claude membaca log dan blame untuk versi kontrol apa pun yang Anda gunakan dan menjelaskan keputusan di balik implementasi saat ini."
  },
  "scope-a-change-before": {
    title: "Tentukan cakupan perubahan sebelum Anda mulai",
    teaches: "Ukur pekerjaan sebelum Anda berkomitmen pada roadmap. Daftar file memberi tahu Anda apakah Anda melihat satu komponen atau perubahan lintas-potong."
  },
  "ask-the-codebase-a": {
    title: "Tanyakan pertanyaan produk kepada basis kode",
    teaches: "Nyatakan peran Anda sehingga jawaban berada pada tingkat yang tepat. Claude menjelaskan apa yang sebenarnya dilakukan produk dari kode sumber, tanpa Anda perlu membacanya.",
    next: "Atur gaya output sehingga Claude selalu menyampaikan jawaban pada tingkat ini"
  },
  "plan-a-multi-file": {
    title: "Rencanakan perubahan multi-file sebelum menyentuh kode",
    teaches: "Menambahkan \"jangan edit dulu\" memisahkan eksplorasi dari perubahan, sehingga Anda melihat pendekatannya sebelum kode bergerak. Untuk membuat rencana-pertama menjadi default pada setiap prompt, tekan Shift+Tab untuk [plan mode](/docs/id/permission-modes#analyze-before-you-edit-with-plan-mode)."
  },
  "draft-a-spec-by": {
    title: "Buat draf spesifikasi melalui wawancara",
    teaches: "Minta untuk diwawancarai alih-alih menulis spesifikasi sendiri. Claude mengajukan pertanyaan terstruktur kepada Anda sampai persyaratan lengkap, lalu menulis hasilnya ke file.",
    next: "Simpan pertanyaan wawancara Anda sebagai skill `/spec` sehingga setiap spesifikasi dimulai dengan cara yang sama"
  },
  "turn-a-meeting-into": {
    title: "Ubah pertemuan menjadi tiket",
    teaches: "Lewati langkah transkripsi. Claude menarik item tindakan dari input yang tidak terstruktur dan menulisnya langsung ke pelacak Anda melalui [MCP](/docs/id/mcp), sehingga Anda meninjau tiket, bukan transkrip.",
    next: "Simpan ini sebagai skill `/tickets`"
  },
  "map-edge-cases-before": {
    title: "Petakan kasus tepi sebelum membangun",
    teaches: "Tanyakan apa yang hilang, bukan apa yang ada. Claude mencantumkan status kesalahan, status kosong, dan kasus tepi yang cenderung dilewatkan desain jalur bahagia."
  },
  "turn-a-mockup-into": {
    title: "Ubah mockup menjadi prototipe yang berfungsi",
    teaches: "Prototipe yang dapat diklik menjawab pertanyaan yang tidak dapat dijawab mockup statis. Serahkan kode yang berfungsi kepada teknik alih-alih menjelaskan interaksi dalam dokumen."
  },
  "implement-from-a-screenshot": {
    title: "Implementasikan dari tangkapan layar dan periksa sendiri",
    teaches: "Ini memberikan Claude loop verifikasi: ia merender, membandingkan dengan gambar sumber, dan mengulangi tanpa Anda menunjukkan setiap celah.",
    next: "Gunakan `/goal` untuk membuat Claude terus mengulangi sampai tangkapan layar cocok"
  },
  "follow-an-existing-pattern": {
    title: "Ikuti pola yang ada",
    teaches: "Tunjuk kode yang sudah Anda sukai. Tanpa referensi, Claude menggunakan praktik terbaik umum. Dengan satu, itu cocok dengan konvensi yang sebenarnya digunakan basis kode Anda.",
    next: "Minta Claude untuk menulis pola yang diikutinya ke `CLAUDE.md` sehingga sesi mendatang cocok tanpa referensi"
  },
  "add-a-small-well": {
    title: "Tambahkan fitur kecil yang terdefinisi dengan baik",
    teaches: "Nyatakan input dan output, bukan cara membangunnya. Claude menemukan di mana kode serupa berada dan menambahkan milik Anda di sampingnya."
  },
  "build-a-small-internal": {
    title: "Bangun alat internal kecil dari awal",
    teaches: "Anda tidak memerlukan proyek, kerangka kerja, atau langkah pembangunan. Jelaskan alat dan minta Claude untuk membukanya sehingga Anda melihatnya bekerja segera."
  },
  "work-an-issue-end": {
    title: "Kerjakan masalah dari awal hingga akhir",
    teaches: "Berikan nomor masalah, bukan ringkasan. Claude membaca tiket lengkap itu sendiri, sehingga persyaratan yang akan Anda lupakan untuk disebutkan muncul, dan itu memvalidasi perubahan sebelum melaporkan kembali."
  },
  "find-and-update-copy": {
    title: "Temukan dan perbarui salinan di seluruh basis kode",
    teaches: "Tanyakan varian dan katakan apa yang harus dilewati. Claude menemukan frasa yang akan dilewatkan pencarian literal dan meninggalkan fixture pengujian dan riwayat tidak tersentuh, sehingga Anda hanya meninjau salinan yang sebenarnya dilihat pengguna."
  },
  "draft-from-past-examples": {
    title: "Buat draf dokumen dari contoh masa lalu",
    teaches: "Tunjuk folder pekerjaan yang selesai alih-alih menjelaskan gaya Anda. Claude mempelajari struktur dan suara dari apa yang sudah Anda kirim, sehingga draf pertama terlihat seperti salah satu milik Anda.",
    next: "Simpan suara sebagai skill sehingga setiap draf dimulai di sana"
  },
  "write-tests-run-them": {
    title: "Tulis tes, jalankan, perbaiki kegagalan",
    teaches: "Minta untuk menulis, menjalankan, dan memperbaiki bersama-sama sehingga Claude mengulangi tanpa berhenti untuk instruksi.",
    next: "Jalankan `/init` sehingga Claude mempelajari perintah tes Anda secara otomatis"
  },
  "drive-implementation-from-tests": {
    title: "Dorong implementasi dari tes",
    teaches: "Pengembangan berbasis tes: tes mendefinisikan kapan pekerjaan selesai, dan Claude mengulangi implementasi sampai mereka lulus."
  },
  "fill-gaps-from-a": {
    title: "Isi celah dari laporan cakupan",
    teaches: "Tunjuk laporan cakupan alih-alih menebak apa yang tidak diuji. Claude membaca angka sebenarnya dan menulis tes untuk file yang paling membutuhkannya.",
    next: "Atur ini sebagai `/goal` sehingga Claude terus menulis tes sampai cakupan mencapai target"
  },
  "port-code-between-languages": {
    title: "Port kode ke bahasa lain",
    teaches: "Katakan apa yang harus dipertahankan, bukan hanya bahasa target. Menyebutkan API atau perilaku yang harus tetap sama memberikan Claude kontrak untuk memeriksa port."
  },
  "generate-docs-for-code": {
    title: "Hasilkan dokumen untuk kode yang tidak terdokumentasi",
    teaches: "Sebutkan cakupan dan format. Claude menemukan apa yang hilang dan cocok dengan gaya komentar yang sudah ada di file, sehingga dokumen baru terlihat seperti sisanya."
  },
  "migrate-a-pattern-across": {
    title: "Migrasikan pola di seluruh basis kode",
    teaches: "Jelaskan pola lama dan yang baru. Meminta Claude untuk mengidentifikasi setiap tempat terlebih dahulu berarti situs panggilan tercantum dalam respons, sehingga Anda dapat memeriksa tidak ada yang terlewat."
  },
  "optimize-against-a-measurable": {
    title: "Optimalkan terhadap target yang terukur",
    teaches: "Menyatakan metrik dan target memberikan Claude definisi yang jelas tentang selesai.",
    next: "Atur ini sebagai `/goal` sehingga Claude terus mengukur dan mengulangi sampai mencapai angka"
  },
  "fix-a-precise-visual": {
    title: "Perbaiki bug visual yang tepat",
    teaches: "Umpan balik visual yang tepat mendapat perbaikan yang tepat. Nyatakan elemen, pengukuran, dan viewport yang tepat.",
    next: "Tambahkan alat pratinjau sehingga Claude mengambil tangkapan layar dan memverifikasi perbaikan itu sendiri"
  },
  "review-your-changes-before": {
    title: "Tinjau perubahan Anda sebelum Anda berkomitmen",
    teaches: "Tangkap masalah saat masih murah untuk diperbaiki. Claude membaca file yang diubah sepenuhnya, bukan hanya baris diff, sehingga menemukan masalah yang ditinggalkan tinjauan diri cepat.",
    next: "Jalankan `/code-review` untuk pemeriksaan yang sama dalam satu perintah"
  },
  "review-a-pull-request": {
    title: "Tinjau permintaan tarik",
    teaches: "Claude meninjau dengan seluruh basis kode dalam konteks, bukan hanya diff. Ia membaca kode yang diubah dan apa yang dipanggilnya, sehingga menangkap masalah yang akan dilewatkan tinjauan hanya-diff.",
    next: "Aktifkan ini untuk setiap PR dengan Code Review"
  },
  "review-infrastructure-changes-before": {
    title: "Tinjau perubahan infrastruktur sebelum menerapkan",
    teaches: "Output rencana padat dan sulit dipindai. Menempelkannya memberi Anda ringkasan bahasa biasa tentang apa yang sebenarnya akan berubah sebelum Anda menerapkannya."
  },
  "run-a-security-review": {
    title: "Jalankan tinjauan keamanan dengan subagent",
    teaches: "[Subagent](/docs/id/sub-agents) menjalankan audit dalam jendela konteksnya sendiri dan melaporkan kembali ringkasan, sehingga tinjauan keamanan yang panjang tidak mengisi sesi utama Anda. Subagent tujuan umum bawaan menangani ini tanpa penyiapan tambahan.",
    next: "Siapkan subagent tinjauan keamanan khusus yang dapat digunakan seluruh tim Anda"
  },
  "review-content-before-sending": {
    title: "Tangkap masalah sebelum tinjauan formal",
    teaches: "Dapatkan lintasan pertama sebelum manusia menghabiskan waktu untuk itu. Sebutkan kekhawatiran yang ingin Anda periksa sehingga tinjauan terfokus, lalu perbaiki apa yang ditemukannya dan kirim draf yang lebih bersih.",
    next: "Tangkap daftar periksa tinjauan Anda sebagai skill yang dapat dijalankan seluruh tim Anda"
  },
  "course-correct-a-wrong": {
    title: "Koreksi kursus pendekatan yang salah",
    teaches: "Sebutkan batasan yang dilewatkan Claude, bukan hanya bahwa itu salah. Alasan spesifik memberikan Claude batasan konkret untuk dipenuhi pada percobaan ulang, alih-alih menebak lagi.",
    next: "Tekan `Esc` dua kali untuk membuka menu rewind dan mengembalikan kode dan percakapan sehingga percobaan ulang dimulai bersih"
  },
  "narrow-the-scope-of": {
    title: "Persempit cakupan perubahan",
    teaches: "Ketika arahnya benar tetapi perubahan terlalu luas, minta Claude untuk menyimpan bagian darinya daripada membatalkan semuanya. Batas yang dinyatakan membuat perbaikan kecil dari menjadi refaktor."
  },
  "turn-a-correction-into": {
    title: "Ubah koreksi menjadi aturan",
    teaches: "Koreksi dalam obrolan tidak dibagikan dengan tim Anda. Aturan dalam [CLAUDE.md](/docs/id/memory) proyek dibagikan setelah Anda berkomitmen, dan Claude membacanya di awal setiap sesi.",
    next: "Buka `/memory` untuk meninjau apa yang ditulis Claude"
  },
  "resolve-merge-conflicts": {
    title: "Selesaikan konflik penggabungan",
    teaches: "Katakan status apa yang Anda inginkan, bukan penanda mana yang harus disimpan. Meminta alasan membuat penggabungan dapat ditinjau alih-alih kotak hitam."
  },
  "commit-with-a-generated": {
    title: "Berkomitmen dengan pesan yang dihasilkan",
    teaches: "Biarkan Claude menurunkan pesan dari diff. Ini cocok dengan gaya komit yang ada di repositori Anda."
  },
  "open-a-pull-request": {
    title: "Buka permintaan tarik dari tiket",
    teaches: "Lewati pengalihan konteks antara pelacak, editor, dan GitHub. Satu prompt membaca spesifikasi, membuat perubahan, dan membuka PR."
  },
  "draft-release-notes-from": {
    title: "Buat draf catatan rilis dari riwayat git",
    teaches: "Berikan dua titik referensi dan struktur yang Anda inginkan. Claude membaca log komit di antara mereka dan membuat draf changelog yang dapat Anda edit.",
    next: "Simpan ini sebagai skill `/changelog`"
  },
  "write-a-ci-workflow": {
    title: "Tulis alur kerja CI",
    teaches: "Jelaskan kapan harus dijalankan dan apa yang harus dilakukan; YAML dihasilkan untuk Anda, cocok dengan perintah pembangunan dan pengujian proyek Anda."
  },
  "find-and-fix-a": {
    title: "Temukan dan perbaiki tes yang gagal",
    teaches: "Jelaskan gejala; Anda tidak perlu tahu file mana yang rusak. Claude menjalankan tes untuk melihat kegagalan, melacaknya ke sumber, dan memperbaikinya."
  },
  "investigate-a-reported-error": {
    title: "Selidiki kesalahan yang dilaporkan",
    teaches: "Jelaskan gejala dan lokasi; Claude membaca jalur kode yang relevan dan melacak kemungkinan penyebab. Tempel jejak tumpukan atau log jika Anda memilikinya.",
    next: "Letakkan deeplink dalam runbook Anda yang membuka Claude dengan prompt ini sudah diisi sebelumnya"
  },
  "fix-a-build-error": {
    title: "Perbaiki kesalahan pembangunan di akar",
    teaches: "Meminta penyebab akar dan verifikasi mencegah patch tingkat permukaan yang menekan kesalahan tanpa memperbaikinya."
  },
  "investigate-a-production-incident": {
    title: "Selidiki insiden produksi",
    teaches: "Daftar sumber bukti untuk berkorelasi, bukan langkah yang harus diambil. Claude membaca log, riwayat git, dan konfigurasi bersama untuk mempersempit penyebab.",
    next: "Hubungkan Sentry atau penyimpanan log Anda melalui MCP"
  },
  "query-logs-in-plain": {
    title: "Pertanyaan log dalam bahasa Inggris biasa",
    teaches: "Tanyakan pertanyaan alih-alih menulis SQL. Claude membangun kueri, menjalankannya terhadap log yang terhubung, dan menunjukkan kueri dan hasilnya sehingga Anda dapat memeriksa apa yang dijalankan."
  },
  "diagnose-from-a-console": {
    title: "Diagnosa dari tangkapan layar konsol",
    teaches: "Konsol cloud menunjukkan masalah tetapi bukan perintah untuk memperbaikinya. Claude membaca tangkapan layar dan menerjemahkan dasbor menjadi perintah kubectl, gcloud, atau aws untuk dijalankan."
  },
  "analyze-a-data-file": {
    title: "Analisis file data",
    teaches: "Pertanyaan sekali-jalan tidak memerlukan skrip sekali-jalan. Tunjuk file di folder proyek Anda dan Claude membacanya langsung, menemukan pola, dan menulis output di mana Anda minta.",
    next: "Hubungkan sumber data melalui MCP alih-alih mengekspor file"
  },
  "generate-variations-from-performance": {
    title: "Hasilkan variasi dari data kinerja",
    teaches: "Nyatakan batasan di awal sehingga generasi tetap dalam batas. Claude membaca metrik, memilih apa yang harus diganti, dan menghasilkan alternatif yang sesuai.",
    next: "Hubungkan platform iklan melalui MCP alih-alih mengekspor file"
  },
  "turn-a-recurring-task": {
    title: "Ubah tugas berulang menjadi skill",
    teaches: "Sebutkan langkah-langkah sekali; gunakan kembali sebagai perintah. Claude menulis [skill](/docs/id/skills) yang dapat dijalankan siapa pun di tim Anda."
  },
  "add-a-hook-for": {
    title: "Tambahkan hook untuk perilaku berulang",
    teaches: "Hooks membuat perilaku otomatis alih-alih sesuatu yang harus Anda ingat untuk diminta. Jelaskan pemicu dan tindakan dan Claude menulis konfigurasi [hook](/docs/id/hooks)."
  },
  "connect-a-tool-with": {
    title: "Hubungkan alat dengan MCP",
    teaches: "Hubungkan sumber sekali alih-alih menempel data setiap sesi. Setelah penyiapan [MCP](/docs/id/mcp), Claude membaca dari alat secara langsung ketika Anda menanyakannya."
  },
  "capture-what-to-remember": {
    title: "Tangkap apa yang harus diingat untuk lain kali",
    teaches: "Tanya sebelum Anda lupa. Claude tahu apa yang harus dipikirkan sesi ini dan mengusulkan entri [CLAUDE.md](/docs/id/memory) sehingga sesi berikutnya dimulai dengan konteks itu."
  }
};

<PromptLibrary text={text} labels={labels} tagLabels={tagLabels} phaseLabels={phaseLabels} sourceLabels={sourceLabels} catLabels={catLabels} />

<h2 id="what-makes-these-prompts-work">
  Apa yang membuat prompt ini berhasil
</h2>

Prompt di atas berbagi beberapa pola. Mengenalinya membantu Anda menyesuaikan prompt apa pun di sini dengan tugas Anda sendiri.

**Jelaskan hasil, bukan langkah-langkahnya.** Katakan apa yang Anda inginkan dan biarkan Claude menemukan file. Prompt di bawah ini bekerja tanpa menyebutkan satu pun jalur file.

```text theme={null}
add rate limiting to the public API and make sure existing tests still pass
```

**Berikan cara untuk memeriksa pekerjaan sendiri.** Minta untuk menjalankan, menguji, membandingkan, atau memverifikasi dalam prompt yang sama sehingga Claude mengulangi alih-alih berhenti setelah satu percobaan.

```text theme={null}
write the migration, run it against the dev database, and confirm the schema matches
```

**Tunjuk referensi.** Sebutkan file, tes, atau pola yang ada untuk dicocokkan sehingga kode baru konsisten dengan apa yang sudah Anda miliki.

```text theme={null}
add a settings page that follows the same layout as the profile page
```

**Nyatakan target yang terukur.** Ketika tujuannya adalah kinerja atau cakupan, berikan metrik dan ambang batas sehingga penyelesaian tidak ambigu.

```text theme={null}
get the bundle size under 200KB and show me what you removed
```

**Berikan artefak.** Tempel kesalahan, log, tangkapan layar, dan output rencana langsung dalam prompt, atau ketik `@` untuk mereferensikan file. Claude membaca sumber alih-alih deskripsi Anda tentangnya.

```text theme={null}
why is the build failing? @build.log
```

**Katakan bagaimana Anda ingin jawabannya.** Sebutkan format, panjang, atau audiens sehingga penjelasan sesuai dengan cara Anda akan menggunakannya. Untuk membuat format menjadi default untuk setiap respons, atur [output style](/docs/id/output-styles).

```text theme={null}
explain how the payment retry logic works as an HTML page with a diagram, then open it in my browser
```

Untuk lebih lanjut tentang setiap pola, lihat [best practices](/docs/id/best-practices).

<h2 id="where-these-come-from">
  Dari mana ini berasal
</h2>

Prompt ini didasarkan pada pola dari sumber daya Anthropic yang dipublikasikan. Setiap kartu menautkan ke sumbernya:

* [Common workflows](/docs/id/common-workflows): panduan langkah demi langkah untuk tugas inti
* [Best practices](/docs/id/best-practices): pola prompt dan penyiapan proyek
* [How Anthropic teams use Claude Code](https://claude.com/blog/how-anthropic-teams-use-claude-code): alur kerja nyata dari tim teknik, produk, desain, dan data, dengan pendalaman tentang [legal](https://claude.com/blog/how-anthropic-uses-claude-legal), [marketing](https://claude.com/blog/how-anthropic-uses-claude-marketing), dan [cybersecurity](https://claude.com/blog/how-anthropic-uses-claude-cybersecurity)
* [Scaling agentic coding guide](https://resources.anthropic.com/hubfs/Scaling%20agentic%20coding%20across%20your%20organization.pdf): panduan adopsi perusahaan

Untuk panduan video tentang pola ini, lihat kursus gratis [Claude Code in Action](https://anthropic.skilljar.com/claude-code-in-action) di Anthropic Academy.

<h2 id="related-resources">
  Sumber daya terkait
</h2>

Prompt di halaman ini adalah titik awal. Setelah satu berhasil untuk proyek Anda, langkah berikutnya adalah membuatnya dapat diulang: simpan sebagai [skill](/docs/id/skills) sehingga siapa pun di tim Anda dapat menjalankannya sebagai `/command`, dan catat konvensi yang dipelajari Claude dalam [CLAUDE.md](/docs/id/memory) sehingga setiap sesi dimulai dengan konteks itu alih-alih Claude mempelajarinya kembali. Untuk perubahan yang lebih besar atau lebih berisiko, [plan mode](/docs/id/permission-modes#analyze-before-you-edit-with-plan-mode) menunjukkan daftar file sebelum pengeditan apa pun terjadi.

Jika Anda memperkenalkan Claude Code di seluruh tim, lihat [administration](/docs/id/admin-setup) untuk pengaturan terkelola dan kebijakan, dan [costs and usage](/docs/id/costs) untuk cara pekerjaan ini ditagih dalam paket Anda.
