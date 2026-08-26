> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code на Claude Platform on AWS

> Настройте Claude Code для использования API Claude, управляемого Anthropic, с аутентификацией AWS, контролем доступа IAM и выставлением счетов через AWS Marketplace.

export const ContactSalesCard = ({surface}) => {
  const utm = content => `utm_source=claude_code&utm_medium=docs&utm_content=${surface}_${content}`;
  const iconArrowRight = (size = 13) => <svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
      <line x1="5" y1="12" x2="19" y2="12" />
      <polyline points="12 5 19 12 12 19" />
    </svg>;
  const STYLES = `
.cc-cs {
  --cs-slate: #141413;
  --cs-clay: #d97757;
  --cs-clay-deep: #c6613f;
  --cs-gray-000: #ffffff;
  --cs-gray-700: #3d3d3a;
  --cs-border-default: rgba(31, 30, 29, 0.15);
  font-family: inherit;
}
.dark .cc-cs {
  --cs-slate: #f0eee6;
  --cs-gray-000: #262624;
  --cs-gray-700: #bfbdb4;
  --cs-border-default: rgba(240, 238, 230, 0.14);
}
.cc-cs-card {
  display: flex; align-items: center; justify-content: space-between;
  gap: 16px; padding: 14px 16px; margin: 0;
  background: var(--cs-gray-000); border: 0.5px solid var(--cs-border-default);
  border-radius: 8px; flex-wrap: wrap;
}
.cc-cs-text { font-size: 13px; color: var(--cs-gray-700); line-height: 1.5; flex: 1; min-width: 240px; }
.cc-cs-text strong { font-weight: 550; color: var(--cs-slate); }
.cc-cs-actions { display: flex; align-items: center; gap: 8px; flex-shrink: 0; }
.cc-cs-btn-clay {
  display: inline-flex; align-items: center; gap: 8px;
  background: var(--cs-clay-deep); color: #fff; border: none;
  border-radius: 8px; padding: 8px 14px;
  font-size: 13px; font-weight: 500;
  transition: background-color 0.15s; white-space: nowrap;
}
.cc-cs-btn-clay:hover { background: var(--cs-clay); }
.cc-cs-btn-ghost {
  display: inline-flex; align-items: center; gap: 8px;
  background: transparent; color: var(--cs-gray-700);
  border: 0.5px solid var(--cs-border-default);
  border-radius: 8px; padding: 8px 14px;
  font-size: 13px; font-weight: 500;
}
.cc-cs-btn-ghost:hover { background: rgba(0, 0, 0, 0.04); }
.dark .cc-cs-btn-ghost:hover { background: rgba(255, 255, 255, 0.04); }
@media (max-width: 720px) {
  .cc-cs-actions { width: 100%; }
}
`;
  return <div className="cc-cs not-prose">
      <style>{STYLES}</style>
      <div className="cc-cs-card">
        <div className="cc-cs-text">
          <strong>Deploying Claude Code across your organization?</strong> Talk to sales about enterprise plans, SSO, and centralized billing.
        </div>
        <div className="cc-cs-actions">
          <a href={`https://claude.com/pricing?${utm('view_plans')}#plans-business`} className="cc-cs-btn-ghost">
            View plans
          </a>
          <a href={`https://claude.com/contact-sales?${utm('contact_sales')}`} className="cc-cs-btn-clay">
            Contact sales {iconArrowRight()}
          </a>
        </div>
      </div>
    </div>;
};

export const Experiment = ({flag, treatment, children}) => {
  const VID_KEY = 'exp_vid';
  const CONSENT_COUNTRIES = new Set(['AT', 'BE', 'BG', 'HR', 'CY', 'CZ', 'DK', 'EE', 'FI', 'FR', 'DE', 'GR', 'HU', 'IE', 'IT', 'LV', 'LT', 'LU', 'MT', 'NL', 'PL', 'PT', 'RO', 'SK', 'SI', 'ES', 'SE', 'RE', 'GP', 'MQ', 'GF', 'YT', 'BL', 'MF', 'PM', 'WF', 'PF', 'NC', 'AW', 'CW', 'SX', 'FO', 'GL', 'AX', 'GB', 'UK', 'AI', 'BM', 'IO', 'VG', 'KY', 'FK', 'GI', 'MS', 'PN', 'SH', 'TC', 'GG', 'JE', 'IM', 'CA', 'BR', 'IN']);
  const fnv1a = s => {
    let h = 0x811c9dc5;
    for (let i = 0; i < s.length; i++) {
      h ^= s.charCodeAt(i);
      h += (h << 1) + (h << 4) + (h << 7) + (h << 8) + (h << 24);
    }
    return h >>> 0;
  };
  const bucket = (seed, vid) => fnv1a(fnv1a(seed + vid) + '') % 10000 < 5000 ? 'control' : 'treatment';
  const [decision] = useState(() => {
    const params = new URLSearchParams(location.search);
    const preBucketed = document.documentElement.dataset['gb_' + flag.replace(/-/g, '_')];
    const force = params.get('gb-force');
    if (force) {
      for (const p of force.split(',')) {
        const [k, v] = p.split(':');
        if (k === flag) return {
          variant: v || 'treatment',
          track: false
        };
      }
    }
    if (navigator.globalPrivacyControl) {
      return {
        variant: 'control',
        track: false
      };
    }
    const prefsMatch = document.cookie.match(/(?:^|; )anthropic-consent-preferences=([^;]+)/);
    if (prefsMatch) {
      try {
        if (JSON.parse(decodeURIComponent(prefsMatch[1])).analytics !== true) {
          return {
            variant: 'control',
            track: false
          };
        }
      } catch {
        return {
          variant: 'control',
          track: false
        };
      }
    } else {
      const country = params.get('country')?.toUpperCase() || (document.cookie.match(/(?:^|; )cf_geo=([A-Z]{2})/) || [])[1];
      if (!country || CONSENT_COUNTRIES.has(country)) {
        return {
          variant: 'control',
          track: false
        };
      }
    }
    let vid;
    try {
      const ajsMatch = document.cookie.match(/(?:^|; )ajs_anonymous_id=([^;]+)/);
      if (ajsMatch) {
        vid = decodeURIComponent(ajsMatch[1]).replace(/^"|"$/g, '');
      } else {
        vid = localStorage.getItem(VID_KEY);
        if (!vid) {
          vid = crypto.randomUUID();
        }
        document.cookie = `ajs_anonymous_id=${vid}; domain=.claude.com; path=/; Secure; SameSite=Lax; max-age=31536000`;
      }
      try {
        localStorage.setItem(VID_KEY, vid);
      } catch {}
    } catch {
      return {
        variant: 'control',
        track: false
      };
    }
    const variant = preBucketed === '1' ? 'treatment' : preBucketed === '0' ? 'control' : bucket(flag, vid);
    return {
      variant,
      track: true,
      vid
    };
  });
  useEffect(() => {
    if (!decision.track) return;
    fetch('https://api.anthropic.com/api/event_logging/v2/batch', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'x-service-name': 'claude_code_docs'
      },
      body: JSON.stringify({
        events: [{
          event_type: 'GrowthbookExperimentEvent',
          event_data: {
            device_id: decision.vid,
            anonymous_id: decision.vid,
            timestamp: new Date().toISOString(),
            experiment_id: flag,
            variation_id: decision.variant === 'treatment' ? 1 : 0,
            environment: 'production'
          }
        }]
      }),
      keepalive: true
    }).catch(() => {});
  }, []);
  return decision.variant === 'treatment' ? treatment : children;
};

<Experiment flag="docs-contact-sales-cta" treatment={<ContactSalesCard surface="claude_platform_on_aws" />} />

Claude Platform on AWS — это API Claude, управляемый Anthropic, с аутентификацией AWS, контролем доступа IAM и выставлением счетов через AWS Marketplace. Запросы поступают непосредственно в API Anthropic, поэтому вы получаете те же модели и функции API, что и в [Claude API](https://platform.claude.com/docs) по тому же графику выпусков. Функции на стороне клиента, которые Claude Code включает через сервис флагов функций Anthropic, такие как [`/loop` самостоятельное управление темпом](/docs/ru/scheduled-tasks#let-claude-choose-the-interval), отключены по умолчанию, и [инструмент advisor](/docs/ru/advisor) недоступен. Полный список см. в [матрице доступности функций](/docs/ru/feature-availability#summary-by-provider). Вы аутентифицируетесь с помощью учетных данных AWS или ключа API рабочей области, и вы платите через AWS Marketplace.

Используйте это руководство, чтобы направить Claude Code на рабочую область, которую вы уже подготовили через Claude Platform on AWS. Для подписки AWS и настройки рабочей области, которые предшествуют этому, см. [документацию Claude Platform on AWS](https://platform.claude.com/docs/en/build-with-claude/claude-platform-on-aws).

<Note>
  Подписка через AWS Marketplace подготавливает новую организацию Anthropic, привязанную к вашей учетной записи AWS. Эта организация отделена от любой организации, которая у вас уже есть в Anthropic, и учетные данные не передаются между ними. Используйте идентификатор рабочей области и ключи API из организации, связанной с AWS, а не из предварительно существующей учетной записи Claude Console.
</Note>

<h2 id="prerequisites">
  Предварительные требования
</h2>

Перед настройкой Claude Code вам потребуется:

* Активная подписка Claude Platform on AWS через AWS Marketplace
* Рабочая область в вашей организации Anthropic, связанной с AWS, с ее идентификатором рабочей области
* Субъект IAM с разрешением на вызов сервиса Anthropic или ключ API, ограниченный рабочей областью
* Учетные данные AWS в вашей среде, в `~/.aws/credentials` или от присоединенной роли IAM, если вы хотите аутентификацию SigV4. AWS CLI требуется только для потока входа SSO.

<h2 id="setup">
  Настройка
</h2>

<h3 id="1-configure-aws-credentials">
  1. Настройте учетные данные AWS
</h3>

Claude Code поддерживает два метода аутентификации для Claude Platform on AWS. Выберите метод, который соответствует тому, как ваша команда управляет доступом.

**Вариант A: учетные данные AWS с SigV4**

Claude Code подписывает запросы с помощью SigV4, используя стандартную цепочку учетных данных AWS: переменные окружения, общие учетные данные в `~/.aws/credentials`, роли IAM, сеансы AWS SSO и любые другие источники, которые поддерживает AWS SDK.

Для локального использования войдите с помощью AWS CLI перед запуском Claude Code. Пример ниже использует профиль SSO, но любой метод, который создает учетные данные в стандартных местоположениях, работает.

```bash theme={null}
aws sso login --profile my-profile
export AWS_PROFILE=my-profile
```

Для CI и автоматизации дайте средству выполнения роль IAM с разрешением на вызов сервиса Anthropic и установите `AWS_REGION`. Цепочка учетных данных автоматически подхватывает роль.

Если ваши учетные данные SSO истекают во время сеанса, настройте [`awsAuthRefresh`](/docs/ru/amazon-bedrock#advanced-credential-configuration), чтобы Claude Code повторно запустил вашу команду входа и повторил попытку вместо сбоя. Автоматическое обновление на Claude Platform on AWS требует Claude Code v2.1.198 или более поздней версии; более ранние версии останавливаются с подсказкой запустить `/login`, что не может обновить учетные данные AWS. Добавьте команду в ваш `settings.json`:

```json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile my-profile"
}
```

С настроенным `awsAuthRefresh`, `/login` показывает опцию **Claude Platform on AWS · refresh credentials** в разделе **Using 3rd-party platforms**. Выбор этой опции запускает настроенную команду и повторно читает ваши учетные данные AWS без перезагрузки Claude Code.

**Вариант B: ключ API рабочей области**

Ключ API рабочей области — это долгоживущий секрет, полезный, когда вы не хотите управлять федеративными учетными данными AWS. Создайте его в консоли AWS в разделе **Claude Platform on AWS → API keys** и установите его как `ANTHROPIC_AWS_API_KEY`:

```bash theme={null}
export ANTHROPIC_AWS_API_KEY=sk-ant-xxxxx
```

Ключ отправляется как `x-api-key` и имеет приоритет над SigV4, поэтому любые учетные данные AWS в вашей среде игнорируются. Ключи API из отдельной организации Claude Console здесь не будут работать.

Обращайтесь с ключами API рабочей области как с любыми другими производственными учетными данными. [Файл пользовательских настроек](/docs/ru/settings) блок `env` — это удобный способ ограничить ключ вашей машиной без глобального экспорта.

<Note>
  Команды `/login` и `/logout` не подписывают вас на подписку Claude.ai для Claude Platform on AWS. Аутентификация выполняется через ваши учетные данные AWS или ключ API рабочей области. Исключением является опция **refresh credentials**, которую `/login` показывает при настроенном `awsAuthRefresh`, которая повторно читает ваши учетные данные AWS, как описано выше.
</Note>

<h3 id="2-configure-claude-code">
  2. Настройте Claude Code
</h3>

Установите переменные окружения, которые направляют Claude Code через Claude Platform on AWS вместо API Anthropic по умолчанию.

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export AWS_REGION=us-east-1
```

`ANTHROPIC_AWS_WORKSPACE_ID` является обязательным и отправляется при каждом запросе как заголовок `anthropic-workspace-id`. Базовый URL вычисляется из `AWS_REGION` как `https://aws-external-anthropic.{region}.api.aws`. Чтобы переопределить URL напрямую, установите `ANTHROPIC_AWS_BASE_URL`.

Claude Platform on AWS является добровольным, даже если учетные данные AWS присутствуют в вашей среде. Amazon Bedrock и Microsoft Foundry имеют приоритет в маршрутизации поставщиков, поэтому отмените установку `CLAUDE_CODE_USE_BEDROCK` и `CLAUDE_CODE_USE_FOUNDRY`, если они установлены.

<h3 id="3-pin-model-versions">
  3. Закрепите версии моделей
</h3>

Claude Platform on AWS использует те же идентификаторы моделей, что и прямой API Claude.

Псевдонимы по умолчанию `fable`, `opus`, `sonnet` и `haiku` разрешаются в встроенные значения по умолчанию Claude Code для Claude Platform on AWS, которые могут отставать от последнего выпуска. Без `ANTHROPIC_DEFAULT_OPUS_MODEL` псевдоним `opus` разрешается в Opus 4.8. До версии v2.1.207 он разрешался в Opus 4.7.

Если вы развертываете Claude Code для команды, явно закрепите идентификаторы моделей, чтобы новый выпуск не переместил всех сразу:

```bash theme={null}
export ANTHROPIC_DEFAULT_FABLE_MODEL=claude-fable-5
export ANTHROPIC_DEFAULT_OPUS_MODEL=claude-opus-4-8
export ANTHROPIC_DEFAULT_SONNET_MODEL=claude-sonnet-5
export ANTHROPIC_DEFAULT_HAIKU_MODEL=claude-haiku-4-5
```

Полный список идентификаторов моделей и псевдонимов см. в разделе [Обзор моделей](https://platform.claude.com/docs/en/about-claude/models/overview). Для других переменных, связанных с моделями, см. [Конфигурация модели](/docs/ru/model-config).

[Prompt caching](/docs/ru/prompt-caching) включен автоматически. Чтобы запросить TTL кэша на 1 час вместо стандартного 5 минут, установите `ENABLE_PROMPT_CACHING_1H=1`. API выставляет счета за записи кэша на 1 час по более высокому тарифу. Ознакомьтесь с [ценами на prompt caching](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing) для получения информации о тарифах.

<h2 id="use-the-agent-sdk">
  Используйте Agent SDK
</h2>

[Agent SDK](/docs/ru/agent-sdk/overview) читает те же переменные окружения, что и CLI, поэтому любая программа, которая порождает подпроцесс Claude Code, может нацеливаться на Claude Platform on AWS, экспортируя `CLAUDE_CODE_USE_ANTHROPIC_AWS`, `ANTHROPIC_AWS_WORKSPACE_ID` и либо `ANTHROPIC_AWS_API_KEY`, либо учетные данные AWS перед вызовом.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

process.env.CLAUDE_CODE_USE_ANTHROPIC_AWS = "1";
process.env.ANTHROPIC_AWS_WORKSPACE_ID = "wrkspc_01ABCDEFGHIJKLMN";
process.env.AWS_REGION = "us-east-1";

for await (const msg of query({ prompt: "What's in this repo?" })) {
  console.log(msg);
}
```

Этот пример полагается на окружающую цепочку учетных данных AWS для SigV4. Чтобы вместо этого аутентифицироваться с помощью ключа API рабочей области, установите `ANTHROPIC_AWS_API_KEY` таким же образом. Для более широкой поверхности Agent SDK см. [Обзор Agent SDK](/docs/ru/agent-sdk/overview).

<h2 id="route-through-a-corporate-proxy">
  Маршрутизация через корпоративный прокси
</h2>

Чтобы маршрутизировать трафик через прокси или [LLM gateway](/docs/ru/llm-gateway), установите `ANTHROPIC_AWS_BASE_URL` на адрес прокси. Claude Code отправляет запросы на этот URL с теми же заголовками рабочей области и аутентификации, поэтому любой шлюз, который пересылает их без изменений, работает.

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export ANTHROPIC_AWS_BASE_URL=https://anthropic-proxy.example.com
```

Если ваш шлюз подписывает запросы сам, установите `CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1`, чтобы Claude Code отправлял неподписанные запросы и позволял шлюзу добавлять заголовки SigV4 перед пересылкой в AWS. Если шлюз требует свой собственный токен, установите его в `ANTHROPIC_AUTH_TOKEN`.

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export ANTHROPIC_AWS_BASE_URL=https://anthropic-proxy.example.com
```

<h2 id="troubleshooting">
  Troubleshooting
</h2>

Запустите `/status`, чтобы увидеть разрешенного поставщика и любой явно настроенный идентификатор рабочей области, регион, переопределение базового URL и параметр пропуска аутентификации. Это самый быстрый способ подтвердить, что Claude Code нацелен на Claude Platform on AWS вообще.

<h3 id="403-forbidden-or-accessdenied-on-every-request">
  `403 Forbidden` или `AccessDenied` при каждом запросе
</h3>

Субъект IAM, который разрешил Claude Code, вероятно, не имеет разрешения на вызов сервиса Anthropic в вашей рабочей области. Проверьте роль, присоединенную к вашему профилю AWS или средству выполнения, которое запустило Claude Code, и убедитесь, что оно имеет действия `aws-external-anthropic`, задокументированные в [справочнике действий IAM](https://platform.claude.com/docs/en/api/claude-platform-on-aws-iam-actions).

Если вы установили `ANTHROPIC_AWS_API_KEY`, ключ имеет приоритет над SigV4, и устаревший ключ создает ту же ошибку. Создайте ключ заново в консоли AWS в разделе **Claude Platform on AWS → API keys** или отмените установку переменной, чтобы вернуться к вашим учетным данным AWS.

<h3 id="requests-fail-with-a-missing-workspace-error">
  Запросы не выполняются с ошибкой отсутствующей рабочей области
</h3>

`ANTHROPIC_AWS_WORKSPACE_ID` вероятно не установлен или пуст. Каждый запрос Claude Platform on AWS должен включать идентификатор рабочей области. Он не подразумевается вашими учетными данными AWS. Найдите идентификатор в разделе **Workspaces** на странице сервиса консоли AWS и экспортируйте его перед запуском Claude Code.

<h3 id="requests-still-go-to-api-anthropic-com">
  Запросы по-прежнему идут на `api.anthropic.com`
</h3>

`CLAUDE_CODE_USE_ANTHROPIC_AWS` вероятно не установлен или установлен на значение, которое не анализируется как истинное. Установите его на `1` и запустите `/status`, чтобы подтвердить разрешенного поставщика. Если также установлены `CLAUDE_CODE_USE_BEDROCK` или `CLAUDE_CODE_USE_FOUNDRY`, они имеют приоритет над Claude Platform on AWS.

<h2 id="additional-resources">
  Дополнительные ресурсы
</h2>

Подписка Claude Platform on AWS, настройка рабочей области и IAM, которые предшествуют настройке Claude Code, рассматриваются в документации платформы:

* [Обзор Claude Platform on AWS](https://platform.claude.com/docs/ru/build-with-claude/claude-platform-on-aws): подписка, настройка рабочей области и справочник продукта
* [Справочник действий IAM](https://platform.claude.com/docs/ru/api/claude-platform-on-aws-iam-actions): разрешения и управляемые политики
