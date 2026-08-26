> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# 프롬프트 라이브러리

> Claude Code에 복사하여 붙여넣을 수 있는 프롬프트 모음으로, 작업과 역할별로 태그가 지정되어 있습니다.

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

이것은 Claude Code에 복사하여 붙여넣을 수 있는 프롬프트 라이브러리입니다. 이를 사용하여 아직 시도하지 않은 작업 방식을 탐색하거나 어디서부터 시작해야 할지 확실하지 않을 때 활용하십시오.

프롬프트는 [일반적인 워크플로우](/docs/ko/common-workflows), [모범 사례](/docs/ko/best-practices), [Anthropic 팀이 Claude Code를 사용하는 방법](https://claude.com/blog/how-anthropic-teams-use-claude-code)을 포함한 다양한 Anthropic 가이드에서 수집되었습니다. 이들은 스크립트가 아닌 시작점입니다. 모든 프롬프트 아래의 **이것이 작동하는 이유**를 열어 패턴을 확인하면 자신만의 프롬프트를 작성할 수 있습니다.

export const labels = {
  startHere: "여기서 시작",
  startHereHeader: "먼저 시도할 다섯 가지 프롬프트",
  showAll: "{n}개의 모든 프롬프트 표시",
  search: "프롬프트 검색…",
  clear: "지우기",
  prompt: "프롬프트",
  prompts: "프롬프트",
  noMatch: "일치하는 프롬프트 없음",
  fillAndCopy: "입력하고 복사",
  copyThis: "이 프롬프트 복사",
  hintBefore: "입력",
  hintChip: "강조된",
  hintAfter: "필드를 입력하여 사용자 정의한 후 복사하십시오.",
  copy: "복사",
  copied: "복사됨",
  whyWorks: "이것이 작동하는 이유",
  makeItStick: "기억하기",
  from: "출처",
  paste: {
    mockup: "목업 이미지를 붙여넣거나, 드래그하거나, @-멘션한 후 다음을 전송하십시오:",
    design: "디자인 이미지를 붙여넣거나, 드래그하거나, @-멘션한 후 다음을 전송하십시오:",
    screenshot: "스크린샷을 붙여넣거나, 드래그하거나, @-멘션한 후 다음을 전송하십시오:",
    plan: "계획 출력을 먼저 프롬프트에 붙여넣은 후 다음을 전송하십시오:",
    error: "오류 출력을 먼저 프롬프트에 붙여넣은 후 다음을 전송하십시오:",
    csv: "파일을 프롬프트로 드래그하거나, 아래 경로를 자신의 @-멘션으로 바꾸십시오:"
  },
  needsLabel: "필요",
  needs: {
    tracker: "이슈 추적기가 [claude.ai 커넥터](/docs/ko/mcp#use-mcp-servers-from-claude-ai) 또는 [MCP 서버](/docs/ko/mcp)로 추가됨.",
    gh: "[gh CLI](https://cli.github.com)가 인증되었거나 GitHub이 [claude.ai 커넥터](/docs/ko/mcp#use-mcp-servers-from-claude-ai)로 추가됨.",
    browser: "Claude가 결과를 렌더링하고 스크린샷을 찍을 수 있는 방법. [데스크톱 앱](/docs/ko/desktop#preview-your-app)에는 이 기능이 내장되어 있습니다. 터미널에서 [Chrome 확장 프로그램](/docs/ko/chrome)이나 Playwright [MCP](/docs/ko/mcp) 서버를 설치하십시오.",
    db: "데이터 웨어하우스 또는 로그 저장소가 [claude.ai 커넥터](/docs/ko/mcp#use-mcp-servers-from-claude-ai) 또는 [MCP 서버](/docs/ko/mcp)로 추가됨."
  }
};

export const tagLabels = {
  understand: "이해",
  plan: "계획",
  prototype: "프로토타입",
  build: "구축",
  test: "테스트",
  refactor: "리팩토링",
  review: "검토",
  steer: "조정",
  debug: "디버깅",
  git: "Git",
  release: "릴리스",
  data: "데이터",
  automate: "자동화",
  pm: "제품",
  design: "디자인",
  docs: "문서",
  marketing: "마케팅",
  security: "보안",
  ops: "온콜"
};

export const phaseLabels = {
  discover: "발견",
  design: "디자인",
  build: "구축",
  ship: "배포",
  operate: "운영"
};

export const sourceLabels = {
  workflows: "일반적인 워크플로우",
  teams: "Anthropic 팀이 Claude Code를 사용하는 방법",
  legal: "Anthropic이 법률 분야에서 Claude를 사용하는 방법",
  cybersecurity: "Anthropic이 사이버보안 분야에서 Claude를 사용하는 방법",
  "best-practices": "모범 사례",
  ebook: "에이전트 코딩 확장 가이드"
};

export const catLabels = {
  Onboard: "온보드",
  Understand: "이해",
  Plan: "계획",
  Prototype: "프로토타입",
  Implement: "구현",
  Test: "테스트",
  Refactor: "리팩토링",
  Review: "검토",
  Steer: "조정",
  Git: "Git",
  Release: "릴리스",
  Debug: "디버깅",
  Incident: "인시던트",
  Data: "데이터",
  Automate: "자동화"
};

export const text = {
  "get-oriented-in-a": {
    title: "새로운 저장소에서 방향 잡기",
    teaches: "읽을 파일을 지정하지 말고 알고 싶은 것을 설명하십시오. Claude가 프로젝트를 자체적으로 탐색하고 어떻게 함께 작동하는지에 대한 요약을 반환합니다.",
    next: "`/init`를 실행하여 `CLAUDE.md`를 설정하면 Claude가 매 세션마다 이를 기억합니다"
  },
  "explain-unfamiliar-code": {
    title: "낯선 코드 설명",
    teaches: "파일 이름을 지정하고 답변을 원하는 형식을 말하십시오. HTML 페이지를 다이어그램, 글머리 기호 또는 학습 방식에 맞는 형식으로 바꾸십시오.",
    next: "출력 스타일을 설정하면 Claude가 항상 선호하는 형식으로 설명합니다"
  },
  "find-where-something-happens": {
    title: "무언가가 발생하는 위치 찾기",
    teaches: "파일 이름으로 검색하지 말고 동작으로 검색하십시오. 파일 이름이나 디렉토리를 모르더라도 검색이 작동합니다."
  },
  "see-what-depends-on": {
    title: "삭제하기 전에 무엇이 깨지는지 확인",
    teaches: "무언가를 제거하기 전에 물어보십시오. 호출자 목록과 다운스트림 영향은 한 줄 정리인지 아니면 조정이 필요한 변경인지를 알려줍니다."
  },
  "trace-how-code-evolved": {
    title: "코드가 어떻게 진화했는지 추적",
    teaches: "무엇이 아닌 왜에 대한 질문일 때 커밋 히스토리를 지적하십시오. Claude가 사용 중인 모든 버전 제어 시스템의 로그와 blame을 읽고 현재 구현 뒤의 결정을 설명합니다."
  },
  "scope-a-change-before": {
    title: "시작하기 전에 변경 범위 지정",
    teaches: "로드맵에 커밋하기 전에 작업 규모를 파악하십시오. 파일 목록은 단일 구성 요소인지 아니면 교차 절단 변경인지를 알려줍니다."
  },
  "ask-the-codebase-a": {
    title: "코드베이스에 제품 질문 하기",
    teaches: "역할을 명시하면 답변이 적절한 수준으로 제시됩니다. Claude가 소스 코드에서 제품이 실제로 무엇을 하는지 설명하므로 읽을 필요가 없습니다.",
    next: "출력 스타일을 설정하면 Claude가 항상 이 수준으로 답변을 제시합니다"
  },
  "plan-a-multi-file": {
    title: "코드를 건드리기 전에 다중 파일 변경 계획",
    teaches: "\"아직 편집하지 마\"를 추가하면 탐색과 변경이 분리되므로 코드가 움직이기 전에 접근 방식을 볼 수 있습니다. 모든 프롬프트에서 계획 우선을 기본값으로 만들려면 Shift+Tab을 눌러 [계획 모드](/docs/ko/permission-modes#analyze-before-you-edit-with-plan-mode)를 사용하십시오."
  },
  "draft-a-spec-by": {
    title: "인터뷰로 사양 작성",
    teaches: "사양을 직접 작성하는 대신 인터뷰를 요청하십시오. Claude가 요구 사항이 완료될 때까지 구조화된 질문을 하고 결과를 파일에 작성합니다.",
    next: "인터뷰 질문을 `/spec` 스킬로 저장하면 모든 사양이 같은 방식으로 시작됩니다"
  },
  "turn-a-meeting-into": {
    title: "회의를 티켓으로 변환",
    teaches: "필사 단계를 건너뛰십시오. Claude가 구조화되지 않은 입력에서 작업 항목을 추출하고 [MCP](/docs/ko/mcp)를 통해 추적기에 직접 작성하므로 필사본이 아닌 티켓을 검토합니다.",
    next: "이를 `/tickets` 스킬로 저장하십시오"
  },
  "map-edge-cases-before": {
    title: "구축하기 전에 엣지 케이스 매핑",
    teaches: "있는 것이 아닌 없는 것을 물어보십시오. Claude가 행복한 경로 디자인이 건너뛰는 오류 상태, 빈 상태 및 엣지 케이스를 나열합니다."
  },
  "turn-a-mockup-into": {
    title: "목업을 작동하는 프로토타입으로 변환",
    teaches: "클릭 가능한 프로토타입은 정적 목업이 답할 수 없는 질문에 답합니다. 문서에서 상호 작용을 설명하는 대신 작동하는 코드를 엔지니어링에 전달하십시오."
  },
  "implement-from-a-screenshot": {
    title: "스크린샷에서 구현하고 자체 확인",
    teaches: "이것은 Claude에 검증 루프를 제공합니다. 렌더링하고, 소스 이미지와 비교하고, 각 간격을 지적할 필요 없이 반복합니다.",
    next: "스크린샷이 일치할 때까지 Claude가 반복하도록 `/goal`을 사용하십시오"
  },
  "follow-an-existing-pattern": {
    title: "기존 패턴 따르기",
    teaches: "이미 좋아하는 코드를 지적하십시오. 참조가 없으면 Claude는 일반적인 모범 사례로 기본값을 설정합니다. 참조가 있으면 코드베이스가 실제로 사용하는 규칙과 일치합니다.",
    next: "Claude가 따른 패턴을 `CLAUDE.md`에 작성하도록 요청하면 향후 세션이 참조 없이 일치합니다"
  },
  "add-a-small-well": {
    title: "작고 잘 정의된 기능 추가",
    teaches: "구축 방법이 아닌 입력과 출력을 명시하십시오. Claude가 유사한 코드가 있는 위치를 찾고 옆에 추가합니다."
  },
  "build-a-small-internal": {
    title: "처음부터 작은 내부 도구 구축",
    teaches: "프로젝트, 프레임워크 또는 빌드 단계가 필요하지 않습니다. 도구를 설명하고 Claude에 열도록 요청하면 즉시 작동하는 것을 볼 수 있습니다."
  },
  "work-an-issue-end": {
    title: "이슈를 끝까지 처리",
    teaches: "요약이 아닌 이슈 번호를 제공하십시오. Claude가 전체 티켓을 읽으므로 언급하기를 잊을 요구 사항이 포함되고 변경을 검증한 후 보고합니다."
  },
  "find-and-update-copy": {
    title: "코드베이스 전체에서 복사본 찾기 및 업데이트",
    teaches: "변형을 요청하고 건너뛸 항목을 말하십시오. Claude가 문자 검색이 놓칠 표현을 찾고 테스트 픽스처와 히스토리는 건드리지 않으므로 사용자가 실제로 보는 복사본만 검토합니다."
  },
  "draft-from-past-examples": {
    title: "과거 예제에서 문서 작성",
    teaches: "스타일을 설명하는 대신 완성된 작업 폴더를 지적하십시오. Claude가 이미 배포한 것에서 구조와 음성을 학습하므로 첫 번째 초안이 당신의 것처럼 읽힙니다.",
    next: "음성을 스킬로 저장하면 모든 초안이 거기서 시작됩니다"
  },
  "write-tests-run-them": {
    title: "테스트 작성, 실행, 실패 수정",
    teaches: "쓰기, 실행, 수정을 함께 요청하면 Claude가 지시를 기다리지 않고 반복합니다.",
    next: "Claude가 테스트 명령을 자동으로 학습하도록 `/init`를 실행하십시오"
  },
  "drive-implementation-from-tests": {
    title: "테스트에서 구현 주도",
    teaches: "테스트 주도 개발: 테스트가 작업이 완료되는 시점을 정의하고 Claude가 통과할 때까지 구현을 반복합니다."
  },
  "fill-gaps-from-a": {
    title: "커버리지 보고서에서 간격 채우기",
    teaches: "추측하는 대신 커버리지 보고서를 지적하십시오. Claude가 실제 숫자를 읽고 가장 필요한 파일에 대한 테스트를 작성합니다.",
    next: "이를 `/goal`로 설정하면 Claude가 커버리지가 목표에 도달할 때까지 테스트를 계속 작성합니다"
  },
  "port-code-between-languages": {
    title: "코드를 다른 언어로 포팅",
    teaches: "대상 언어만이 아닌 보존할 항목을 말하십시오. 유지해야 하는 API 또는 동작의 이름을 지정하면 Claude가 포트를 확인할 계약을 제공합니다."
  },
  "generate-docs-for-code": {
    title: "문서화되지 않은 코드에 대한 문서 생성",
    teaches: "범위와 형식을 지정하십시오. Claude가 누락된 항목을 찾고 파일에 이미 있는 주석 스타일과 일치하므로 새 문서가 나머지처럼 읽힙니다."
  },
  "migrate-a-pattern-across": {
    title: "코드베이스 전체에서 패턴 마이그레이션",
    teaches: "이전 패턴과 새 패턴을 설명하십시오. Claude에 먼저 모든 위치를 식별하도록 요청하면 호출 사이트가 응답에 나열되므로 놓친 것이 없는지 확인할 수 있습니다."
  },
  "optimize-against-a-measurable": {
    title: "측정 가능한 목표에 대해 최적화",
    teaches: "메트릭과 목표를 명시하면 Claude에 명확한 완료 정의를 제공합니다.",
    next: "이를 `/goal`로 설정하면 Claude가 숫자에 도달할 때까지 측정하고 반복합니다"
  },
  "fix-a-precise-visual": {
    title: "정확한 시각적 버그 수정",
    teaches: "정확한 시각적 피드백은 정확한 수정을 얻습니다. 정확한 요소, 측정 및 뷰포트를 명시하십시오.",
    next: "Claude가 수정을 스크린샷하고 자체적으로 검증하도록 미리보기 도구를 추가하십시오"
  },
  "review-your-changes-before": {
    title: "커밋하기 전에 변경 사항 검토",
    teaches: "여전히 수정하기 저렴할 때 문제를 포착하십시오. Claude가 diff 줄만이 아닌 변경된 파일 전체를 읽으므로 빠른 자체 검토가 놓칠 문제를 발견합니다.",
    next: "한 명령으로 동일한 확인을 위해 `/code-review`를 실행하십시오"
  },
  "review-a-pull-request": {
    title: "풀 요청 검토",
    teaches: "Claude가 diff만이 아닌 전체 코드베이스 컨텍스트로 검토합니다. 변경된 코드와 호출하는 항목을 읽으므로 diff 전용 검토가 놓칠 문제를 포착합니다.",
    next: "코드 검토로 모든 PR에 대해 이를 켜십시오"
  },
  "review-infrastructure-changes-before": {
    title: "적용하기 전에 인프라 변경 검토",
    teaches: "계획 출력은 밀도가 높고 스캔하기 어렵습니다. 붙여넣으면 적용하기 전에 실제로 변경될 내용의 일반 언어 요약을 얻습니다."
  },
  "run-a-security-review": {
    title: "서브에이전트로 보안 검토 실행",
    teaches: "[서브에이전트](/docs/ko/sub-agents)가 자신의 컨텍스트 윈도우에서 감사를 실행하고 요약을 보고하므로 긴 보안 검토가 주 세션을 채우지 않습니다. 기본 제공 범용 서브에이전트가 추가 설정 없이 이를 처리합니다.",
    next: "전체 팀이 사용할 수 있는 전용 보안 검토 서브에이전트를 설정하십시오"
  },
  "review-content-before-sending": {
    title: "공식 검토 전에 문제 포착",
    teaches: "인간이 시간을 소비하기 전에 첫 번째 통과를 받으십시오. 확인하려는 우려 사항을 지정하면 검토가 집중되고 발견한 항목을 수정한 후 더 깔끔한 초안을 전송합니다.",
    next: "검토 체크리스트를 스킬로 캡처하면 전체 팀이 실행할 수 있습니다"
  },
  "course-correct-a-wrong": {
    title: "잘못된 접근 방식 수정",
    teaches: "잘못되었다는 것만이 아닌 Claude가 놓친 제약을 지정하십시오. 구체적인 이유는 Claude에 다시 추측하는 대신 재시도에서 만족할 구체적인 제약을 제공합니다.",
    next: "코드와 대화를 복원하도록 `Esc`를 두 번 눌러 되감기 메뉴를 열면 재시도가 깔끔하게 시작됩니다"
  },
  "narrow-the-scope-of": {
    title: "변경 범위 좁히기",
    teaches: "방향이 맞지만 변경이 너무 광범위할 때 Claude에 일부를 유지하도록 요청하십시오. 명시된 경계는 작은 수정이 리팩토링이 되는 것을 방지합니다."
  },
  "turn-a-correction-into": {
    title: "수정을 규칙으로 변환",
    teaches: "채팅의 수정은 팀과 공유되지 않습니다. 프로젝트의 [CLAUDE.md](/docs/ko/memory)의 규칙은 커밋하면 공유되고 Claude가 매 세션의 시작에 읽습니다.",
    next: "Claude가 작성한 내용을 검토하려면 `/memory`를 열으십시오"
  },
  "resolve-merge-conflicts": {
    title: "병합 충돌 해결",
    teaches: "유지할 마커를 말하지 말고 원하는 상태를 말하십시오. 추론을 요청하면 병합이 검토 가능해지므로 블랙박스가 아닙니다."
  },
  "commit-with-a-generated": {
    title: "생성된 메시지로 커밋",
    teaches: "Claude가 diff에서 메시지를 도출하도록 하십시오. 저장소의 기존 커밋 스타일과 일치합니다."
  },
  "open-a-pull-request": {
    title: "티켓에서 풀 요청 열기",
    teaches: "추적기, 편집기, GitHub 간의 컨텍스트 전환을 건너뛰십시오. 한 프롬프트가 사양을 읽고, 변경을 하고, PR을 엽니다."
  },
  "draft-release-notes-from": {
    title: "git 히스토리에서 릴리스 노트 작성",
    teaches: "두 개의 참조 지점과 원하는 구조를 제공하십시오. Claude가 그 사이의 커밋 로그를 읽고 편집할 수 있는 변경 로그를 작성합니다.",
    next: "이를 `/changelog` 스킬로 저장하십시오"
  },
  "write-a-ci-workflow": {
    title: "CI 워크플로우 작성",
    teaches: "실행 시기와 수행할 작업을 설명하십시오. YAML은 프로젝트의 빌드 및 테스트 명령과 일치하도록 생성됩니다."
  },
  "find-and-fix-a": {
    title: "실패한 테스트 찾기 및 수정",
    teaches: "증상을 설명하십시오. 어느 파일이 깨졌는지 알 필요가 없습니다. Claude가 테스트를 실행하여 실패를 보고, 소스로 추적하고, 수정합니다."
  },
  "investigate-a-reported-error": {
    title: "보고된 오류 조사",
    teaches: "증상과 위치를 설명하십시오. Claude가 관련 코드 경로를 읽고 가능한 원인을 추적합니다. 스택 추적이나 로그가 있으면 붙여넣으십시오.",
    next: "런북에 깊은 링크를 넣으면 이 프롬프트가 미리 채워진 상태로 Claude를 엽니다"
  },
  "fix-a-build-error": {
    title: "빌드 오류를 근본 원인에서 수정",
    teaches: "근본 원인과 검증을 요청하면 오류를 수정하지 않고 억제하는 표면 수준의 패치를 방지합니다."
  },
  "investigate-a-production-incident": {
    title: "프로덕션 인시던트 조사",
    teaches: "수행할 단계가 아닌 상관 관계를 지을 증거 소스를 나열하십시오. Claude가 로그, git 히스토리, 구성을 함께 읽어 원인을 좁힙니다.",
    next: "MCP를 통해 Sentry 또는 로그 저장소를 연결하십시오"
  },
  "query-logs-in-plain": {
    title: "일반 영어로 로그 쿼리",
    teaches: "SQL을 작성하는 대신 질문을 하십시오. Claude가 쿼리를 구축하고 연결된 로그에 대해 실행하고 쿼리와 결과를 모두 표시하므로 실행된 항목을 확인할 수 있습니다."
  },
  "diagnose-from-a-console": {
    title: "콘솔 스크린샷에서 진단",
    teaches: "클라우드 콘솔은 문제를 보여주지만 수정할 명령은 보여주지 않습니다. Claude가 스크린샷을 읽고 대시보드를 실행할 kubectl, gcloud 또는 aws 명령으로 변환합니다."
  },
  "analyze-a-data-file": {
    title: "데이터 파일 분석",
    teaches: "일회성 질문은 일회성 스크립트가 필요하지 않습니다. 프로젝트 폴더의 파일을 지적하면 Claude가 직접 읽고, 패턴을 찾고, 요청한 위치에 출력을 작성합니다.",
    next: "파일을 내보내는 대신 MCP를 통해 데이터 소스를 연결하십시오"
  },
  "generate-variations-from-performance": {
    title: "성능 데이터에서 변형 생성",
    teaches: "생성이 제한 내에 머물도록 시작 시 제약을 명시하십시오. Claude가 메트릭을 읽고, 바꿀 항목을 선택하고, 맞는 대안을 생성합니다.",
    next: "파일을 내보내는 대신 MCP를 통해 광고 플랫폼을 연결하십시오"
  },
  "turn-a-recurring-task": {
    title: "반복 작업을 스킬로 변환",
    teaches: "단계를 한 번 지정하고 명령으로 재사용하십시오. Claude가 팀의 누구나 실행할 수 있는 [스킬](/docs/ko/skills)을 작성합니다."
  },
  "add-a-hook-for": {
    title: "반복 동작을 위한 훅 추가",
    teaches: "훅은 동작을 자동으로 만들므로 요청하기를 기억할 필요가 없습니다. 트리거와 작업을 설명하면 Claude가 [훅](/docs/ko/hooks) 구성을 작성합니다."
  },
  "connect-a-tool-with": {
    title: "MCP로 도구 연결",
    teaches: "매 세션마다 데이터를 붙여넣는 대신 소스를 한 번 연결하십시오. [MCP](/docs/ko/mcp) 설정 후 Claude가 요청할 때 도구에서 직접 읽습니다."
  },
  "capture-what-to-remember": {
    title: "다음 번을 위해 기억할 항목 캡처",
    teaches: "잊기 전에 물어보십시오. Claude가 이 세션에서 파악해야 할 항목을 알고 다음 세션이 그 컨텍스트로 시작하도록 [CLAUDE.md](/docs/ko/memory) 항목을 제안합니다."
  }
};

<PromptLibrary text={text} labels={labels} tagLabels={tagLabels} phaseLabels={phaseLabels} sourceLabels={sourceLabels} catLabels={catLabels} />

<h2 id="what-makes-these-prompts-work">
  이 프롬프트가 작동하는 이유
</h2>

위의 프롬프트는 몇 가지 패턴을 공유합니다. 이를 인식하면 여기의 모든 프롬프트를 자신의 작업에 맞게 조정할 수 있습니다.

**단계가 아닌 결과를 설명하십시오.** 원하는 것을 말하고 Claude가 파일을 찾도록 하십시오. 아래 프롬프트는 단일 파일 경로를 지정하지 않고도 작동합니다.

```text theme={null}
공개 API에 속도 제한을 추가하고 기존 테스트가 여전히 통과하는지 확인하십시오
```

**자신의 작업을 확인할 수 있는 방법을 제공하십시오.** 같은 프롬프트에서 실행, 테스트, 비교 또는 검증을 요청하면 Claude가 한 번의 시도 후 중지하지 않고 반복합니다.

```text theme={null}
마이그레이션을 작성하고, 개발 데이터베이스에 대해 실행하고, 스키마가 일치하는지 확인하십시오
```

**참조를 지적하십시오.** 기존 파일, 테스트 또는 패턴의 이름을 지정하여 새 코드가 이미 있는 것과 일치하도록 하십시오.

```text theme={null}
프로필 페이지와 동일한 레이아웃을 따르는 설정 페이지를 추가하십시오
```

**측정 가능한 목표를 명시하십시오.** 목표가 성능이나 커버리지일 때 메트릭과 임계값을 제공하면 완료가 명확합니다.

```text theme={null}
번들 크기를 200KB 미만으로 줄이고 제거한 항목을 보여주십시오
```

**아티팩트를 제공하십시오.** 오류, 로그, 스크린샷 및 계획 출력을 프롬프트에 직접 붙여넣거나 `@`를 입력하여 파일을 참조하십시오. Claude가 설명 대신 소스를 읽습니다.

```text theme={null}
빌드가 실패하는 이유는 무엇입니까? @build.log
```

**답변을 원하는 방식을 말하십시오.** 형식, 길이 또는 대상을 지정하면 설명이 사용 방식에 맞습니다. 모든 응답에 대해 형식을 기본값으로 만들려면 [출력 스타일](/docs/ko/output-styles)을 설정하십시오.

```text theme={null}
결제 재시도 로직이 어떻게 작동하는지 다이어그램이 있는 HTML 페이지로 설명한 후 브라우저에서 열어주십시오
```

각 패턴에 대한 자세한 내용은 [모범 사례](/docs/ko/best-practices)를 참조하십시오.

<h2 id="where-these-come-from">
  이들이 어디서 나왔는지
</h2>

이 프롬프트는 게시된 Anthropic 리소스의 패턴을 기반으로 합니다. 각 카드는 소스에 연결됩니다:

* [일반적인 워크플로우](/docs/ko/common-workflows): 핵심 작업에 대한 단계별 가이드
* [모범 사례](/docs/ko/best-practices): 프롬프팅 패턴 및 프로젝트 설정
* [Anthropic 팀이 Claude Code를 사용하는 방법](https://claude.com/blog/how-anthropic-teams-use-claude-code): 엔지니어링, 제품, 디자인 및 데이터 팀의 실제 워크플로우, [법률](https://claude.com/blog/how-anthropic-uses-claude-legal), [마케팅](https://claude.com/blog/how-anthropic-uses-claude-marketing), [사이버보안](https://claude.com/blog/how-anthropic-uses-claude-cybersecurity)에 대한 심화 학습
* [에이전트 코딩 확장 가이드](https://resources.anthropic.com/hubfs/Scaling%20agentic%20coding%20across%20your%20organization.pdf): 엔터프라이즈 채택 가이드

이 패턴의 비디오 연습을 보려면 Anthropic Academy의 무료 [Claude Code in Action](https://anthropic.skilljar.com/claude-code-in-action) 과정을 참조하십시오.

<h2 id="related-resources">
  관련 리소스
</h2>

이 페이지의 프롬프트는 시작점입니다. 하나가 프로젝트에 작동하면 다음 단계는 반복 가능하게 만드는 것입니다. [스킬](/docs/ko/skills)로 저장하면 팀의 누구나 `/command`로 실행할 수 있고, Claude가 학습한 규칙을 [CLAUDE.md](/docs/ko/memory)에 기록하면 매 세션이 Claude가 다시 학습하는 대신 그 컨텍스트로 시작합니다. 더 크거나 위험한 변경의 경우 [계획 모드](/docs/ko/permission-modes#analyze-before-you-edit-with-plan-mode)는 편집이 발생하기 전에 파일 목록을 표시합니다.

Claude Code를 팀 전체에 도입하는 경우 관리되는 설정 및 정책에 대해 [관리](/docs/ko/admin-setup)를 참조하고, 이 작업이 계획에서 어떻게 청구되는지에 대해 [비용 및 사용](/docs/ko/costs)을 참조하십시오.
