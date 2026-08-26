> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Bibliothèque de prompts

> Copiez-collez des prompts pour Claude Code, étiquetés par tâche et rôle.

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

Ceci est une bibliothèque de prompts à copier dans Claude Code. Utilisez-la pour explorer des façons de travailler que vous n'avez pas essayées, ou quand vous ne savez pas par où commencer.

Les prompts sont collectés à partir de divers guides Anthropic, notamment [Flux de travail courants](/docs/fr/common-workflows), [Bonnes pratiques](/docs/fr/best-practices), et [Comment les équipes Anthropic utilisent Claude Code](https://claude.com/blog/how-anthropic-teams-use-claude-code). Ce sont des points de départ plutôt que des scripts. Ouvrez **Pourquoi cela fonctionne** sous n'importe quel prompt pour voir le modèle derrière celui-ci afin que vous puissiez écrire le vôtre.

export const labels = {
  startHere: "Commencez ici",
  startHereHeader: "Cinq prompts à essayer en premier",
  showAll: "Afficher tous les {n} prompts",
  search: "Rechercher des prompts…",
  clear: "Effacer",
  prompt: "prompt",
  prompts: "prompts",
  noMatch: "Aucun prompt ne correspond",
  fillAndCopy: "Remplir et copier",
  copyThis: "Copier ce prompt",
  hintBefore: "Tapez dans le",
  hintChip: "champs en surbrillance",
  hintAfter: "pour personnaliser, puis copiez.",
  copy: "Copier",
  copied: "Copié",
  whyWorks: "Pourquoi cela fonctionne",
  makeItStick: "Rendez-le mémorable",
  from: "De",
  paste: {
    mockup: "Collez, glissez ou @-mentionnez votre image de maquette, puis envoyez ceci :",
    design: "Collez, glissez ou @-mentionnez votre image de conception, puis envoyez ceci :",
    screenshot: "Collez, glissez ou @-mentionnez votre capture d'écran, puis envoyez ceci :",
    plan: "Collez votre sortie de plan dans le prompt d'abord, puis envoyez ceci :",
    error: "Collez la sortie d'erreur dans le prompt d'abord, puis envoyez ceci :",
    csv: "Glissez votre fichier dans le prompt, ou remplacez le chemin ci-dessous par une @-mention de votre propre fichier :"
  },
  needsLabel: "Nécessite",
  needs: {
    tracker: "votre suivi de problèmes ajouté en tant que [connecteur claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai) ou [serveur MCP](/docs/fr/mcp).",
    gh: "la [CLI gh](https://cli.github.com) authentifiée, ou GitHub ajouté en tant que [connecteur claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai).",
    browser: "un moyen pour Claude de rendre et de capturer une capture d'écran du résultat. L'[application de bureau](/docs/fr/desktop#preview-your-app) a cela intégré. Dans le terminal, installez l'[extension Chrome](/docs/fr/chrome) ou un serveur [MCP](/docs/fr/mcp) Playwright.",
    db: "votre entrepôt de données ou magasin de journaux ajouté en tant que [connecteur claude.ai](/docs/fr/mcp#use-mcp-servers-from-claude-ai) ou [serveur MCP](/docs/fr/mcp)."
  }
};

export const tagLabels = {
  understand: "Comprendre",
  plan: "Planifier",
  prototype: "Prototype",
  build: "Construire",
  test: "Tester",
  refactor: "Refactoriser",
  review: "Examiner",
  steer: "Diriger",
  debug: "Déboguer",
  git: "Git",
  release: "Sortie",
  data: "Données",
  automate: "Automatiser",
  pm: "Produit",
  design: "Conception",
  docs: "Docs",
  marketing: "Marketing",
  security: "Sécurité",
  ops: "Astreinte"
};

export const phaseLabels = {
  discover: "Découvrir",
  design: "Concevoir",
  build: "Construire",
  ship: "Livrer",
  operate: "Exploiter"
};

export const sourceLabels = {
  workflows: "Flux de travail courants",
  teams: "Comment les équipes Anthropic utilisent Claude Code",
  legal: "Comment Anthropic utilise Claude en Juridique",
  cybersecurity: "Comment Anthropic utilise Claude en Cybersécurité",
  "best-practices": "Bonnes pratiques",
  ebook: "Guide de codage agentique à l'échelle"
};

export const catLabels = {
  Onboard: "Intégration",
  Understand: "Comprendre",
  Plan: "Planifier",
  Prototype: "Prototype",
  Implement: "Implémenter",
  Test: "Tester",
  Refactor: "Refactoriser",
  Review: "Examiner",
  Steer: "Diriger",
  Git: "Git",
  Release: "Sortie",
  Debug: "Déboguer",
  Incident: "Incident",
  Data: "Données",
  Automate: "Automatiser"
};

export const text = {
  "get-oriented-in-a": {
    title: "S'orienter dans un nouveau référentiel",
    teaches: "Décrivez ce que vous voulez savoir, pas quels fichiers lire. Claude explore le projet par lui-même et retourne un résumé de la façon dont il s'articule.",
    next: "Exécutez `/init` pour configurer `CLAUDE.md` afin que Claude se souvienne de ceci à chaque session"
  },
  "explain-unfamiliar-code": {
    title: "Expliquer du code non familier",
    teaches: "Nommez le fichier et dites dans quel format vous voulez la réponse. Échangez la page HTML contre un diagramme, des points à puces, ou tout ce qui correspond à votre façon d'apprendre.",
    next: "Définissez un style de sortie afin que Claude explique toujours dans votre format préféré"
  },
  "find-where-something-happens": {
    title: "Trouver où quelque chose se produit",
    teaches: "Recherchez par comportement plutôt que par nom de fichier. La recherche fonctionne même quand vous ne savez pas comment le fichier s'appelle ou dans quel répertoire il se trouve."
  },
  "see-what-depends-on": {
    title: "Vérifier ce qui se casse avant de supprimer",
    teaches: "Demandez avant de supprimer quoi que ce soit. La liste des appelants et des effets en aval vous indique si vous regardez un nettoyage d'une ligne ou une modification que vous devez coordonner."
  },
  "trace-how-code-evolved": {
    title: "Tracer l'évolution du code",
    teaches: "Pointez l'historique des commits quand la question est pourquoi, pas quoi. Claude lit le journal et le blame pour quelle que soit la version control que vous utilisez et explique les décisions derrière l'implémentation actuelle."
  },
  "scope-a-change-before": {
    title: "Délimiter une modification avant de commencer",
    teaches: "Dimensionnez le travail avant de vous y engager sur une feuille de route. La liste des fichiers vous indique si vous regardez un composant ou une modification transversale."
  },
  "ask-the-codebase-a": {
    title: "Poser une question produit à la base de code",
    teaches: "Énoncez votre rôle afin que la réponse soit au bon niveau. Claude explique ce que le produit fait réellement à partir du code source, sans que vous ayez besoin de le lire.",
    next: "Définissez un style de sortie afin que Claude présente toujours les réponses à ce niveau"
  },
  "plan-a-multi-file": {
    title: "Planifier une modification multi-fichiers avant de toucher au code",
    teaches: "Ajouter « ne pas modifier encore » sépare l'exploration des modifications, afin que vous voyiez l'approche avant que le code ne bouge. Pour faire du plan-first le défaut sur chaque prompt, appuyez sur Maj+Tab pour [mode plan](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode)."
  },
  "draft-a-spec-by": {
    title: "Rédiger une spécification par entrevue",
    teaches: "Demandez à être interviewé au lieu d'écrire la spécification vous-même. Claude vous pose des questions structurées jusqu'à ce que les exigences soient complètes, puis écrit le résultat dans un fichier.",
    next: "Enregistrez vos questions d'entrevue en tant que compétence `/spec` afin que chaque spécification commence de la même façon"
  },
  "turn-a-meeting-into": {
    title: "Transformer une réunion en tickets",
    teaches: "Ignorez l'étape de transcription. Claude extrait les éléments d'action de l'entrée non structurée et les écrit directement dans votre suivi via [MCP](/docs/fr/mcp), afin que vous examiniez les tickets, pas la transcription.",
    next: "Enregistrez ceci en tant que compétence `/tickets`"
  },
  "map-edge-cases-before": {
    title: "Cartographier les cas limites avant de construire",
    teaches: "Demandez ce qui manque, pas ce qui est là. Claude énumère les états d'erreur, les états vides et les cas limites qu'une conception de chemin heureux tend à ignorer."
  },
  "turn-a-mockup-into": {
    title: "Transformer une maquette en prototype fonctionnel",
    teaches: "Un prototype cliquable répond à des questions qu'une maquette statique ne peut pas. Remettez le code fonctionnel à l'ingénierie au lieu d'expliquer les interactions dans un document."
  },
  "implement-from-a-screenshot": {
    title: "Implémenter à partir d'une capture d'écran et auto-vérifier",
    teaches: "Cela donne à Claude une boucle de vérification : il rend, compare par rapport à l'image source, et itère sans que vous pointiez chaque écart.",
    next: "Utilisez `/goal` pour garder Claude itérant jusqu'à ce que les captures d'écran correspondent"
  },
  "follow-an-existing-pattern": {
    title: "Suivre un modèle existant",
    teaches: "Pointez du code que vous aimez déjà. Sans référence, Claude utilise par défaut les meilleures pratiques générales. Avec une, il correspond aux conventions que votre base de code utilise réellement.",
    next: "Demandez à Claude d'écrire le modèle qu'il a suivi dans `CLAUDE.md` afin que les sessions futures le correspondent sans la référence"
  },
  "add-a-small-well": {
    title: "Ajouter une petite fonctionnalité bien définie",
    teaches: "Énoncez les entrées et les sorties, pas comment la construire. Claude trouve où le code similaire vit et ajoute le vôtre à côté."
  },
  "build-a-small-internal": {
    title: "Construire un petit outil interne à partir de zéro",
    teaches: "Vous n'avez pas besoin d'un projet, d'un framework ou d'une étape de construction. Décrivez l'outil et demandez à Claude de l'ouvrir afin que vous le voyiez fonctionner immédiatement."
  },
  "work-an-issue-end": {
    title: "Traiter un problème de bout en bout",
    teaches: "Donnez le numéro du problème, pas un résumé. Claude lit le ticket complet lui-même, afin que les exigences que vous oublieriez de mentionner passent, et il valide la modification avant de signaler."
  },
  "find-and-update-copy": {
    title: "Trouver et mettre à jour le texte dans la base de code",
    teaches: "Demandez des variantes et dites ce qu'il faut ignorer. Claude trouve des formulations qu'une recherche littérale manquerait et laisse les fixtures de test et l'historique intacts, afin que vous examiniez uniquement le texte que les utilisateurs voient réellement."
  },
  "draft-from-past-examples": {
    title: "Rédiger un document à partir d'exemples passés",
    teaches: "Pointez un dossier de travail terminé au lieu de décrire votre style. Claude apprend la structure et la voix de ce que vous avez déjà livré, afin que le premier brouillon se lise comme l'un des vôtres.",
    next: "Enregistrez la voix en tant que compétence afin que chaque brouillon commence là"
  },
  "write-tests-run-them": {
    title: "Écrire des tests, les exécuter, corriger les défaillances",
    teaches: "Demandez d'écrire, d'exécuter et de corriger ensemble afin que Claude itère sans s'arrêter pour les instructions.",
    next: "Exécutez `/init` afin que Claude apprenne automatiquement votre commande de test"
  },
  "drive-implementation-from-tests": {
    title: "Piloter l'implémentation à partir des tests",
    teaches: "Développement piloté par les tests : les tests définissent quand le travail est terminé, et Claude itère sur l'implémentation jusqu'à ce qu'ils passent."
  },
  "fill-gaps-from-a": {
    title: "Combler les lacunes à partir d'un rapport de couverture",
    teaches: "Pointez le rapport de couverture au lieu de deviner ce qui n'est pas testé. Claude lit les chiffres réels et écrit des tests pour les fichiers qui en ont le plus besoin.",
    next: "Définissez ceci en tant que `/goal` afin que Claude continue à écrire des tests jusqu'à ce que la couverture atteigne la cible"
  },
  "port-code-between-languages": {
    title: "Porter le code vers un autre langage",
    teaches: "Dites ce qu'il faut préserver, pas seulement le langage cible. Nommer l'API ou le comportement qui doit rester le même donne à Claude un contrat pour vérifier le port."
  },
  "generate-docs-for-code": {
    title: "Générer de la documentation pour du code non documenté",
    teaches: "Nommez la portée et le format. Claude trouve ce qui manque et correspond au style de commentaire déjà dans le fichier, afin que la nouvelle documentation se lise comme le reste."
  },
  "migrate-a-pattern-across": {
    title: "Migrer un modèle dans la base de code",
    teaches: "Décrivez l'ancien modèle et le nouveau. Demander à Claude d'identifier d'abord chaque endroit signifie que les sites d'appel sont énumérés dans la réponse, afin que vous puissiez vérifier qu'aucun n'a été manqué."
  },
  "optimize-against-a-measurable": {
    title: "Optimiser par rapport à une cible mesurable",
    teaches: "Énoncer la métrique et la cible donne à Claude une définition claire de la fin.",
    next: "Définissez ceci en tant que `/goal` afin que Claude continue à mesurer et itérer jusqu'à ce qu'il atteigne le nombre"
  },
  "fix-a-precise-visual": {
    title: "Corriger un bug visuel précis",
    teaches: "Un retour visuel précis obtient une correction précise. Énoncez l'élément exact, la mesure et la fenêtre d'affichage.",
    next: "Ajoutez un outil d'aperçu afin que Claude capture une capture d'écran et vérifie la correction lui-même"
  },
  "review-your-changes-before": {
    title: "Examiner vos modifications avant de valider",
    teaches: "Attrapez les problèmes tant qu'ils sont encore bon marché à corriger. Claude lit les fichiers modifiés en entier, pas seulement les lignes de diff, afin qu'il repère les problèmes qu'un auto-examen rapide manquerait.",
    next: "Exécutez `/code-review` pour la même vérification en une commande"
  },
  "review-a-pull-request": {
    title: "Examiner une demande de tirage",
    teaches: "Claude examine avec la base de code entière en contexte, pas seulement le diff. Il lit le code modifié et ce qu'il appelle, afin qu'il repère les problèmes qu'un examen diff-only manquerait.",
    next: "Activez ceci pour chaque PR avec Code Review"
  },
  "review-infrastructure-changes-before": {
    title: "Examiner les modifications d'infrastructure avant d'appliquer",
    teaches: "La sortie du plan est dense et difficile à analyser. La coller vous donne un résumé en langage clair de ce qui va réellement changer avant que vous l'appliquiez."
  },
  "run-a-security-review": {
    title: "Exécuter un examen de sécurité avec un sous-agent",
    teaches: "Un [sous-agent](/docs/fr/sub-agents) exécute l'audit dans sa propre fenêtre de contexte et signale un résumé, afin qu'un long examen de sécurité ne remplisse pas votre session principale. Le sous-agent polyvalent intégré gère ceci sans configuration supplémentaire.",
    next: "Configurez un sous-agent security-review dédié que toute votre équipe peut utiliser"
  },
  "review-content-before-sending": {
    title: "Attraper les problèmes avant l'examen formel",
    teaches: "Obtenez une première passe avant qu'un humain ne passe du temps dessus. Nommez les préoccupations que vous voulez vérifier afin que l'examen soit ciblé, puis corrigez ce qu'il trouve et envoyez un brouillon plus propre.",
    next: "Capturez votre liste de contrôle d'examen en tant que compétence que toute votre équipe peut exécuter"
  },
  "course-correct-a-wrong": {
    title: "Corriger une mauvaise approche",
    teaches: "Nommez la contrainte que Claude a manquée, pas seulement que c'est mal. Une raison spécifique donne à Claude une contrainte concrète à satisfaire à la nouvelle tentative, au lieu de deviner à nouveau.",
    next: "Appuyez sur `Esc` deux fois pour ouvrir le menu de rembobinage et restaurer le code et la conversation afin que la nouvelle tentative commence propre"
  },
  "narrow-the-scope-of": {
    title: "Réduire la portée d'une modification",
    teaches: "Quand la direction est bonne mais la modification est devenue trop large, demandez à Claude de garder une partie plutôt que de rembobiner tout. Une limite énoncée empêche une petite correction de devenir une refactorisation."
  },
  "turn-a-correction-into": {
    title: "Transformer une correction en règle",
    teaches: "Une correction en chat n'est pas partagée avec votre équipe. Une règle dans le [CLAUDE.md](/docs/fr/memory) du projet est partagée une fois que vous la validez, et Claude la lit au début de chaque session.",
    next: "Ouvrez `/memory` pour examiner ce que Claude a écrit"
  },
  "resolve-merge-conflicts": {
    title: "Résoudre les conflits de fusion",
    teaches: "Dites quel état vous voulez, pas quels marqueurs garder. Demander le raisonnement rend la fusion examinable au lieu d'une boîte noire."
  },
  "commit-with-a-generated": {
    title: "Valider avec un message généré",
    teaches: "Laissez Claude dériver le message du diff. Il correspond au style de commit existant de votre référentiel."
  },
  "open-a-pull-request": {
    title: "Ouvrir une demande de tirage à partir d'un ticket",
    teaches: "Ignorez le changement de contexte entre le suivi, l'éditeur et GitHub. Un prompt lit la spécification, effectue la modification et ouvre la PR."
  },
  "draft-release-notes-from": {
    title: "Rédiger les notes de sortie à partir de l'historique git",
    teaches: "Donnez deux points de référence et la structure que vous voulez. Claude lit le journal des commits entre eux et rédige un changelog que vous pouvez modifier.",
    next: "Enregistrez ceci en tant que compétence `/changelog`"
  },
  "write-a-ci-workflow": {
    title: "Écrire un flux de travail CI",
    teaches: "Décrivez quand il doit s'exécuter et ce qu'il doit faire ; le YAML est généré pour vous, adapté aux commandes de construction et de test de votre projet."
  },
  "find-and-fix-a": {
    title: "Trouver et corriger un test défaillant",
    teaches: "Décrivez le symptôme ; vous n'avez pas besoin de savoir quel fichier est cassé. Claude exécute le test pour voir l'échec, le trace dans la source et le corrige."
  },
  "investigate-a-reported-error": {
    title: "Enquêter sur une erreur signalée",
    teaches: "Décrivez le symptôme et l'emplacement ; Claude lit le chemin de code pertinent et trace les causes probables. Collez les traces de pile ou les journaux si vous les avez.",
    next: "Mettez un lien profond dans votre runbook qui ouvre Claude avec ce prompt pré-rempli"
  },
  "fix-a-build-error": {
    title: "Corriger une erreur de construction à la racine",
    teaches: "Demander la cause racine et la vérification empêche les correctifs de surface qui suppriment l'erreur sans la corriger."
  },
  "investigate-a-production-incident": {
    title: "Enquêter sur un incident de production",
    teaches: "Énumérez les sources de preuves à corréler, pas les étapes à suivre. Claude lit les journaux, l'historique git et la configuration ensemble pour réduire la cause.",
    next: "Connectez Sentry ou votre magasin de journaux via MCP"
  },
  "query-logs-in-plain": {
    title: "Interroger les journaux en anglais simple",
    teaches: "Posez la question au lieu d'écrire le SQL. Claude construit la requête, l'exécute contre vos journaux connectés et affiche à la fois la requête et le résultat afin que vous puissiez vérifier ce qui a été exécuté."
  },
  "diagnose-from-a-console": {
    title: "Diagnostiquer à partir d'une capture d'écran de console",
    teaches: "Les consoles cloud vous montrent le problème mais pas les commandes pour le corriger. Claude lit la capture d'écran et traduit le tableau de bord en commandes kubectl, gcloud ou aws à exécuter."
  },
  "analyze-a-data-file": {
    title: "Analyser un fichier de données",
    teaches: "Une question ponctuelle n'a pas besoin d'un script ponctuel. Pointez un fichier dans votre dossier de projet et Claude le lit directement, trouve les modèles et écrit la sortie où vous le demandez.",
    next: "Connectez la source de données via MCP au lieu d'exporter des fichiers"
  },
  "generate-variations-from-performance": {
    title: "Générer des variations à partir de données de performance",
    teaches: "Énoncez la contrainte au début afin que la génération reste dans la limite. Claude lit les métriques, choisit ce qu'il faut remplacer et produit des alternatives qui correspondent.",
    next: "Connectez la plateforme publicitaire via MCP au lieu d'exporter un fichier"
  },
  "turn-a-recurring-task": {
    title: "Transformer une tâche récurrente en compétence",
    teaches: "Nommez les étapes une fois ; réutilisez-les en tant que commande. Claude écrit une [compétence](/docs/fr/skills) que n'importe qui dans votre équipe peut exécuter."
  },
  "add-a-hook-for": {
    title: "Ajouter un hook pour un comportement répété",
    teaches: "Les hooks rendent un comportement automatique au lieu de quelque chose que vous devez vous souvenir de demander. Décrivez le déclencheur et l'action et Claude écrit la configuration du [hook](/docs/fr/hooks)."
  },
  "connect-a-tool-with": {
    title: "Connecter un outil avec MCP",
    teaches: "Connectez la source une fois au lieu de coller les données à chaque session. Après la configuration [MCP](/docs/fr/mcp), Claude lit directement à partir de l'outil quand vous lui en parlez."
  },
  "capture-what-to-remember": {
    title: "Capturer ce qu'il faut retenir pour la prochaine fois",
    teaches: "Demandez avant d'oublier. Claude sait ce qu'il a dû comprendre cette session et propose des entrées [CLAUDE.md](/docs/fr/memory) afin que la session suivante commence avec ce contexte."
  }
};

<PromptLibrary text={text} labels={labels} tagLabels={tagLabels} phaseLabels={phaseLabels} sourceLabels={sourceLabels} catLabels={catLabels} />

<h2 id="what-makes-these-prompts-work">
  Ce qui rend ces prompts efficaces
</h2>

Les prompts ci-dessus partagent quelques modèles. Les reconnaître vous aide à adapter n'importe quel prompt ici à votre propre tâche.

**Décrivez le résultat, pas les étapes.** Dites ce que vous voulez et laissez Claude trouver les fichiers. Le prompt ci-dessous fonctionne sans nommer un seul chemin de fichier.

```text theme={null}
ajouter la limitation de débit à l'API publique et s'assurer que les tests existants passent toujours
```

**Donnez-lui un moyen de vérifier son propre travail.** Demandez d'exécuter, tester, comparer ou vérifier dans le même prompt afin que Claude itère au lieu de s'arrêter après une tentative.

```text theme={null}
écrire la migration, l'exécuter contre la base de données de développement et confirmer que le schéma correspond
```

**Pointez une référence.** Nommez un fichier, un test ou un modèle existant à correspondre afin que le nouveau code soit cohérent avec ce que vous avez déjà.

```text theme={null}
ajouter une page de paramètres qui suit la même mise en page que la page de profil
```

**Énoncez la cible mesurable.** Quand l'objectif est la performance ou la couverture, donnez la métrique et le seuil afin que la fin soit sans ambiguïté.

```text theme={null}
obtenir la taille du bundle sous 200 KB et montrez-moi ce que vous avez supprimé
```

**Donnez-lui l'artefact.** Collez les erreurs, les journaux, les captures d'écran et la sortie du plan directement dans le prompt, ou tapez `@` pour référencer un fichier. Claude lit la source au lieu de votre description de celle-ci.

```text theme={null}
pourquoi la construction échoue-t-elle ? @build.log
```

**Dites comment vous voulez la réponse.** Nommez le format, la longueur ou le public afin que l'explication correspond à la façon dont vous l'utiliserez. Pour faire d'un format le défaut pour chaque réponse, définissez un [style de sortie](/docs/fr/output-styles).

```text theme={null}
expliquer comment la logique de nouvelle tentative de paiement fonctionne en tant que page HTML avec un diagramme, puis l'ouvrir dans mon navigateur
```

Pour plus sur chaque modèle, voir [bonnes pratiques](/docs/fr/best-practices).

<h2 id="where-these-come-from">
  D'où viennent ceux-ci
</h2>

Ces prompts sont basés sur des modèles de ressources Anthropic publiées. Chaque carte renvoie à sa source :

* [Flux de travail courants](/docs/fr/common-workflows) : guides étape par étape pour les tâches principales
* [Bonnes pratiques](/docs/fr/best-practices) : modèles d'invite et configuration de projet
* [Comment les équipes Anthropic utilisent Claude Code](https://claude.com/blog/how-anthropic-teams-use-claude-code) : flux de travail réels des équipes d'ingénierie, de produit, de conception et de données, avec des approfondissements sur [juridique](https://claude.com/blog/how-anthropic-uses-claude-legal), [marketing](https://claude.com/blog/how-anthropic-uses-claude-marketing), et [cybersécurité](https://claude.com/blog/how-anthropic-uses-claude-cybersecurity)
* [Guide de codage agentique à l'échelle](https://resources.anthropic.com/hubfs/Scaling%20agentic%20coding%20across%20your%20organization.pdf) : le guide d'adoption en entreprise

Pour des présentations vidéo de ces modèles, voir le cours gratuit [Claude Code in Action](https://anthropic.skilljar.com/claude-code-in-action) sur Anthropic Academy.

<h2 id="related-resources">
  Ressources connexes
</h2>

Les prompts sur cette page sont des points de départ. Une fois qu'un fonctionne pour votre projet, l'étape suivante est de le rendre répétable : enregistrez-le en tant que [compétence](/docs/fr/skills) afin que n'importe qui dans votre équipe puisse l'exécuter en tant que `/commande`, et enregistrez les conventions que Claude a apprises dans [CLAUDE.md](/docs/fr/memory) afin que chaque session commence avec ce contexte au lieu que Claude le réapprenne. Pour les modifications plus grandes ou plus risquées, [mode plan](/docs/fr/permission-modes#analyze-before-you-edit-with-plan-mode) vous montre la liste des fichiers avant que des modifications ne se produisent.

Si vous introduisez Claude Code dans une équipe, voir [administration](/docs/fr/admin-setup) pour les paramètres gérés et la politique, et [coûts et utilisation](/docs/fr/costs) pour savoir comment ce travail est facturé sur votre plan.
