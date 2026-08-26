> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code на Amazon Bedrock

> Узнайте о настройке Claude Code через Amazon Bedrock, включая установку, конфигурацию IAM и устранение неполадок.

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

<ContactSalesCard surface="bedrock" />

<h2 id="prerequisites">
  Предварительные требования
</h2>

Перед настройкой Claude Code с Amazon Bedrock убедитесь, что у вас есть:

* Учетная запись AWS с включенным доступом к Amazon Bedrock
* Доступ к нужным моделям Claude (например, Claude Sonnet 4.6) в Amazon Bedrock
* AWS CLI установлен и настроен (опционально - требуется только если у вас нет другого механизма получения учетных данных)
* Соответствующие разрешения IAM

Чтобы войти со своими собственными учетными данными Amazon Bedrock, следуйте инструкциям [Вход с Amazon Bedrock](#sign-in-with-bedrock) ниже. Чтобы развернуть Claude Code в команде, используйте шаги [ручной установки](#set-up-manually) и [закрепите версии вашей модели](#4-pin-model-versions) перед развертыванием.

<h2 id="sign-in-with-bedrock">
  Вход с Bedrock
</h2>

Если у вас есть учетные данные AWS и вы хотите начать использовать Claude Code через Amazon Bedrock, мастер входа проведет вас через процесс. Вы выполняете предварительные требования на стороне AWS один раз на учетную запись; мастер обрабатывает сторону Claude Code.

<Steps>
  <Step title="Включите модели Anthropic в вашей учетной записи AWS">
    В [консоли Amazon Bedrock](https://console.aws.amazon.com/bedrock/) откройте каталог моделей, выберите модель Anthropic и отправьте форму варианта использования. Доступ предоставляется сразу же после отправки. См. [Отправьте детали варианта использования](#1-submit-use-case-details) для AWS Organizations и [конфигурацию IAM](#iam-configuration) для разрешений, которые требуются вашей роли.
  </Step>

  <Step title="Запустите Claude Code и выберите Amazon Bedrock">
    Запустите `claude`. При запросе входа выберите **3rd-party platform**, затем **Amazon Bedrock**.
  </Step>

  <Step title="Следуйте подсказкам мастера">
    Выберите способ аутентификации в AWS: профиль AWS, обнаруженный из вашей директории `~/.aws`, ключ API Amazon Bedrock, ключ доступа и секрет, или учетные данные уже в вашей среде. Мастер выбирает ваш регион, проверяет, какие модели Claude может вызывать ваша учетная запись, и позволяет вам их закрепить. Он сохраняет результат в блок `env` вашего [файла параметров пользователя](/docs/ru/settings), поэтому вам не нужно самостоятельно экспортировать переменные окружения.
  </Step>
</Steps>

После входа запустите `/setup-bedrock` в любое время, чтобы снова открыть мастер и изменить ваши учетные данные, регион или закрепления моделей. Шаг закрепления модели начинается с ваших текущих закрепленных моделей. Мастер записывает в `~/.claude/settings.json`, или в `$CLAUDE_CONFIG_DIR/settings.json` когда установлена [`CLAUDE_CONFIG_DIR`](/docs/ru/env-vars#variables).

<h2 id="set-up-manually">
  Ручная установка
</h2>

Чтобы настроить Amazon Bedrock через переменные окружения вместо мастера, например в CI или при развертывании в масштабе предприятия, следуйте шагам ниже.

<h3 id="1-submit-use-case-details">
  1. Отправьте детали варианта использования
</h3>

Пользователи, впервые использующие модели Anthropic, должны отправить детали варианта использования перед вызовом модели. Это делается один раз на учетную запись AWS.

1. Убедитесь, что у вас есть правильные разрешения IAM, описанные ниже
2. Перейдите на [консоль Amazon Bedrock](https://console.aws.amazon.com/bedrock/)
3. Выберите модель Anthropic из **Model catalog**
4. Заполните форму варианта использования. Доступ предоставляется сразу же после отправки.

Если вы используете AWS Organizations, вы можете отправить форму один раз из учетной записи управления, используя [`PutUseCaseForModelAccess` API](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_PutUseCaseForModelAccess.html). Этот вызов требует разрешение IAM `bedrock:PutUseCaseForModelAccess`. Одобрение автоматически распространяется на дочерние учетные записи.

<h3 id="2-configure-aws-credentials">
  2. Настройте учетные данные AWS
</h3>

Claude Code использует цепочку учетных данных AWS SDK по умолчанию. Установите ваши учетные данные, используя один из этих методов:

**Вариант A: конфигурация AWS CLI**

```bash theme={null}
aws configure
```

**Вариант B: переменные окружения (ключ доступа)**

```bash theme={null}
export AWS_ACCESS_KEY_ID=your-access-key-id
export AWS_SECRET_ACCESS_KEY=your-secret-access-key
export AWS_SESSION_TOKEN=your-session-token
```

**Вариант C: переменные окружения (профиль SSO)**

Замените `your-profile-name` на имя вашего профиля AWS перед запуском этих команд.

```bash theme={null}
aws sso login --profile=your-profile-name

export AWS_PROFILE=your-profile-name
```

Claude Code запрашивает учетные данные роли из региона IAM Identity Center, названного в `sso_region` профиля, который не должен совпадать с регионом, в котором вы запускаете Amazon Bedrock. В версии 2.1.207 регион Amazon Bedrock переопределял `sso_region`, поэтому профиль, экземпляр IAM Identity Center которого находится в другом регионе, не прошел аутентификацию с ошибкой `Session token not found or invalid`.

**Вариант D: учетные данные AWS Management Console**

```bash theme={null}
aws login
```

[Узнайте больше](https://docs.aws.amazon.com/signin/latest/userguide/command-line-sign-in.html) о `aws login`.

**Вариант E: ключи API Amazon Bedrock**

```bash theme={null}
export AWS_BEARER_TOKEN_BEDROCK=your-bedrock-api-key
```

Ключи API Amazon Bedrock предоставляют более простой метод аутентификации без необходимости полных учетных данных AWS. [Узнайте больше о ключах API Amazon Bedrock](https://aws.amazon.com/blogs/machine-learning/accelerate-ai-development-with-amazon-bedrock-api-keys/).

<h4 id="credential-caching-and-resolution-timeout">
  Кэширование учетных данных и время ожидания разрешения
</h4>

Claude Code разрешает цепочку поставщика учетных данных AWS по умолчанию один раз и сохраняет разрешенные учетные данные в памяти. Он повторно использует их до пяти минут до истечения срока действия или в течение одного часа, когда они не имеют срока действия, поэтому профиль на основе SSO запрашивает учетные данные из IAM Identity Center примерно один раз за время жизни учетных данных. Ошибка учетных данных из API очищает кэш, и повторная попытка разрешает свежие учетные данные.

До версии 2.1.207 Claude Code разрешал цепочку при каждом запросе API, поэтому профиль на основе SSO запрашивал свежие учетные данные из IAM Identity Center каждый раз и мог быть ограничен в больших развертываниях.

Кэш охватывает все варианты учетных данных выше, кроме ключа API Amazon Bedrock, который не использует цепочку поставщика. Чтобы разрешить цепочку при каждом запросе вместо этого, установите [`CLAUDE_CODE_SKIP_AWS_CRED_CACHE=1`](/docs/ru/env-vars).

Каждое разрешение цепочки истекает через 60 секунд. Если шаг в цепочке зависает, например помощник `credential_process`, который ждет ввода, который он не может получить, запрос завершается с ошибкой [`AWS default-chain credential resolve timed out`](/docs/ru/errors#aws-default-chain-credential-resolve-timed-out). Если ваша цепочка запускает интерактивный вход, который законно требует больше времени, например браузерный SSO с MFA через оболочку, такую как `aws-vault`, повысьте лимит в миллисекундах с помощью [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/ru/env-vars). До версии 2.1.207 зависшее разрешение учетных данных оставляло запрос в ожидании неопределенно долго.

<h4 id="advanced-credential-configuration">
  Расширенная конфигурация учетных данных
</h4>

Claude Code поддерживает автоматическое обновление учетных данных для AWS SSO и корпоративных поставщиков идентификации. Добавьте эти параметры в файл параметров Claude Code (см. [Settings](/docs/ru/settings) для расположения файлов).

Эти два параметра имеют разные условия срабатывания:

* **`awsAuthRefresh`**: запускается только когда Claude Code обнаруживает, что ваши учетные данные AWS истекли, либо локально на основе их временной метки, либо когда API возвращает ошибку учетных данных, затем повторяет попытку запроса с обновленными учетными данными.
* **`awsCredentialExport`**: запускается при запуске сеанса и при каждой перезагрузке учетных данных, даже когда учетные данные в цепочке поставщика учетных данных AWS по умолчанию все еще действительны. Используйте это, когда ваша учетная запись Amazon Bedrock требует учетные данные между учетными записями, которые отличаются от тех, которые разрешила бы цепочка поставщика по умолчанию.

<h5 id="example-configuration">
  Пример конфигурации
</h5>

```json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile myprofile",
  "env": {
    "AWS_PROFILE": "myprofile"
  }
}
```

<h5 id="configuration-settings-explained">
  Объяснение параметров конфигурации
</h5>

**`awsAuthRefresh`**: используйте это для команд, которые изменяют директорию `.aws`, такие как обновление учетных данных, кэша SSO или файлов конфигурации. Вывод команды отображается пользователю, но интерактивный ввод не поддерживается. Это хорошо работает для браузерных потоков SSO, где CLI отображает URL или код, и вы завершаете аутентификацию в браузере.

**`awsCredentialExport`**: используйте это только если вы не можете изменить `.aws` и должны напрямую вернуть учетные данные. Эта команда запускается всякий раз, когда необходимо обновить учетные данные, а не только когда учетные данные истекли. Вывод захватывается молча и не показывается пользователю. Команда должна выводить JSON в этом формате:

```json theme={null}
{
  "Credentials": {
    "AccessKeyId": "value",
    "SecretAccessKey": "value",
    "SessionToken": "value",
    "Expiration": "2026-01-01T00:00:00Z"
  }
}
```

Начиная с Claude Code v2.1.181, плоский вывод из `aws configure export-credentials --format process` также принимается, с теми же ключами на верхнем уровне вместо вложения под `Credentials`.

`Expiration` является необязательным. Начиная с Claude Code v2.1.176, когда команда возвращает действительное значение `Expiration` в формате ISO 8601, Claude Code кэширует учетные данные до пяти минут до этого времени. Без него или в более ранних версиях учетные данные кэшируются в течение одного часа.

Когда вы настраиваете `awsCredentialExport` без `awsAuthRefresh`, Claude Code использует экспортированные учетные данные напрямую и не переразрешает цепочку поставщика учетных данных AWS по умолчанию при запуске. До версии 2.1.206 при запуске также переразрешалась цепочка поставщика по умолчанию, что делало живой вызов SSO или STS вне конфигурации вашего прокси и могло заблокировать первый запрос на несколько минут в сетях с ограниченным исходящим трафиком.

<h3 id="3-configure-claude-code">
  3. Настройте Claude Code
</h3>

Установите следующие переменные окружения для включения Amazon Bedrock:

```bash theme={null}
# Enable Bedrock integration
export CLAUDE_CODE_USE_BEDROCK=1
export AWS_REGION=us-east-1  # optional if your AWS profile already sets a region

# Optional: Override the AWS region for the small/fast model (Bedrock and Mantle).
# On Bedrock, has no effect without ANTHROPIC_DEFAULT_HAIKU_MODEL
# or the deprecated ANTHROPIC_SMALL_FAST_MODEL set.
export ANTHROPIC_SMALL_FAST_MODEL_AWS_REGION=us-west-2

# Optional: Override the Bedrock endpoint URL for custom endpoints or gateways
# export ANTHROPIC_BEDROCK_BASE_URL=https://bedrock-runtime.us-east-1.amazonaws.com
```

При включении Amazon Bedrock для Claude Code имейте в виду следующее:

* Начиная с версии 2.1.172, вам нужно установить `AWS_REGION` только для переопределения региона вашего профиля AWS или когда ваш профиль не имеет региона. Claude Code разрешает регион в этом порядке:

  * `AWS_REGION`
  * `AWS_DEFAULT_REGION`
  * регион, установленный на вашем активном профиле AWS, прочитанный сначала из файла общих учетных данных AWS, а затем из файла общей конфигурации, соответствуя приоритету AWS SDK
  * `us-east-1`

  Активный профиль — это `AWS_PROFILE`, если установлен, иначе `default`. Установите `AWS_SHARED_CREDENTIALS_FILE` или `AWS_CONFIG_FILE` для указания на пути к файлам, отличные от стандартных. Запустите `/status` для просмотра разрешенного региона. Когда регион поступил из ваших файлов конфигурации AWS или стандартного резервного варианта, `/status` также указывает источник. На версии 2.1.171 и более ранних Claude Code не читает файлы конфигурации AWS, поэтому установите `AWS_REGION` явно.
* При использовании Amazon Bedrock команда `/logout` недоступна, так как аутентификация обрабатывается через учетные данные AWS.
* Инструмент WebSearch недоступен на Amazon Bedrock. См. [поведение инструмента WebSearch](/docs/ru/tools-reference#websearch-tool-behavior).
* Вы можете использовать файлы параметров для переменных окружения, таких как `AWS_PROFILE`, которые вы не хотите утечь в другие процессы. См. [Settings](/docs/ru/settings) для получения дополнительной информации.

<h3 id="4-pin-model-versions">
  4. Закрепите версии моделей
</h3>

<Warning>
  Закрепите конкретные версии моделей при развертывании для нескольких пользователей. Без закрепления псевдонимы моделей, такие как `sonnet` и `opus`, разрешаются на встроенное значение по умолчанию Claude Code для Amazon Bedrock, которое может отставать от последнего выпуска и может быть еще недоступно в вашей учетной записи. Claude Code [возвращается](#startup-model-checks) к более ранней или более низкой модели при запуске, когда значение по умолчанию недоступно, но закрепление позволяет вам контролировать, когда ваши пользователи переходят на новую модель.
</Warning>

Установите эти переменные окружения на конкретные ID моделей Amazon Bedrock.

Без `ANTHROPIC_DEFAULT_OPUS_MODEL` псевдоним `opus` на Amazon Bedrock разрешается на Opus 4.8, а без `ANTHROPIC_DEFAULT_SONNET_MODEL` псевдоним `sonnet` разрешается на Sonnet 4.5. Этот пример закрепляет каждый псевдоним на конкретную версию:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='us.anthropic.claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='us.anthropic.claude-sonnet-4-6'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='us.anthropic.claude-haiku-4-5-20251001-v1:0'
```

Эти переменные используют ID профилей вывода между регионами (с префиксом `us.`). Если вы используете другой префикс региона или профили вывода приложения, отрегулируйте соответственно. В регионах AWS GovCloud используйте префикс `us-gov.`. Для текущих и устаревших ID моделей см. [Models overview](https://platform.claude.com/docs/en/about-claude/models/overview). См. [Model configuration](/docs/ru/model-config#pin-models-for-third-party-deployments) для полного списка переменных окружения.

Claude Code использует эти модели по умолчанию, когда переменные закрепления не установлены:

| Тип модели           | Значение по умолчанию                          |
| :------------------- | :--------------------------------------------- |
| Основная модель      | `us.anthropic.claude-opus-4-8`                 |
| Малая/быстрая модель | `us.anthropic.claude-sonnet-4-5-20250929-v1:0` |

Фоновые задачи, такие как генерация заголовка сеанса, используют малую/быструю модель, обычно модель класса Haiku. На Amazon Bedrock Claude Code использует модель Sonnet по умолчанию для фоновых задач, потому что Haiku может быть не включен в каждой учетной записи или регионе. Два выбора изменяют, какая модель их выполняет:

* Когда вы выбираете основную модель с `--model`, `ANTHROPIC_MODEL` или параметром `model`, фоновые задачи используют эту модель. Установка `ANTHROPIC_DEFAULT_OPUS_MODEL` без `ANTHROPIC_DEFAULT_SONNET_MODEL` также считается выбором, потому что встроенная модель Sonnet может быть не включена в учетной записи, которая управляет своим собственным Opus.
* Чтобы использовать Haiku для фоновых задач, установите `ANTHROPIC_DEFAULT_HAIKU_MODEL` на ID модели, который доступен в вашей учетной записи.

<Warning>
  Модели Opus имеют более высокую цену за токен, чем модели Sonnet, поэтому развертывание, которое не закрепляет основную модель, выставляется по тарифу Opus после обновления до версии 2.1.207 или позже. Чтобы сохранить Sonnet 4.5 в качестве основной модели, установите `ANTHROPIC_MODEL` на его полный ID модели. Развертывание, которое управляет значением по умолчанию с помощью `ANTHROPIC_DEFAULT_SONNET_MODEL` и не устанавливает `ANTHROPIC_DEFAULT_OPUS_MODEL`, сохраняет свою управляемую модель Sonnet в качестве значения по умолчанию.
</Warning>

До версии 2.1.207 основная модель на Amazon Bedrock по умолчанию была Sonnet 4.5, псевдоним `opus` разрешался на Opus 4.6, и фоновые задачи всегда использовали основную модель.

Для дальнейшей настройки моделей используйте один из этих методов:

```bash theme={null}
# Using inference profile ID
export ANTHROPIC_MODEL='us.anthropic.claude-sonnet-4-6'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='us.anthropic.claude-haiku-4-5-20251001-v1:0'

# Using application inference profile ARN
export ANTHROPIC_MODEL='arn:aws:bedrock:us-east-2:your-account-id:application-inference-profile/your-model-id'

# Optional: Disable prompt caching if needed
export DISABLE_PROMPT_CACHING=1

# Optional: Request 1-hour prompt cache TTL instead of the 5-minute default
export ENABLE_PROMPT_CACHING_1H=1
```

TTL кэша в 1 час выставляется по более высокому тарифу, чем стандартный 5-минутный. См. [cache lifetime](/docs/ru/prompt-caching#cache-lifetime).

<Note>Prompt caching может быть недоступен во всех регионах Amazon Bedrock. Если количество токенов кэша остается на нуле, проверьте [поддерживаемые модели, регионы и ограничения](https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html#prompt-caching-models) в документации Amazon Bedrock.</Note>

<h4 id="map-each-model-version-to-an-inference-profile">
  Сопоставьте каждую версию модели с профилем вывода
</h4>

Переменные окружения `ANTHROPIC_DEFAULT_*_MODEL` настраивают один профиль вывода на семейство моделей. Если вашей организации необходимо предоставить несколько версий одного семейства в средстве выбора `/model`, каждая маршрутизируется на свой ARN профиля вывода приложения, используйте вместо этого параметр `modelOverrides` в вашем [файле параметров](/docs/ru/settings#settings-files).

Этот пример сопоставляет четыре версии Opus с отдельными ARN, чтобы пользователи могли переключаться между ними без обхода профилей вывода вашей организации:

```json theme={null}
{
  "modelOverrides": {
    "claude-opus-4-7": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-47-prod",
    "claude-opus-4-6": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-46-prod",
    "claude-opus-4-5-20251101": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-45-prod",
    "claude-opus-4-1-20250805": "arn:aws:bedrock:us-east-2:123456789012:application-inference-profile/opus-41-prod"
  }
}
```

Когда пользователь выбирает одну из этих версий в `/model`, Claude Code вызывает Amazon Bedrock с сопоставленным ARN. То же сопоставление применяется, когда вы передаете ID модели Anthropic напрямую через `--model` или `ANTHROPIC_MODEL`. Версии без переопределения возвращаются к встроенному ID модели Amazon Bedrock или любому соответствующему профилю вывода, обнаруженному при запуске. До версии 2.1.200 значения `--model` и `ANTHROPIC_MODEL` достигали Amazon Bedrock как есть без прохождения через карту переопределений. См. [Override model IDs per version](/docs/ru/model-config#override-model-ids-per-version) для получения подробной информации о том, как переопределения взаимодействуют с `availableModels` и другими параметрами модели.

<h2 id="startup-model-checks">
  Проверки моделей при запуске
</h2>

Когда Claude Code запускается с настроенным Amazon Bedrock, он проверяет, что модели, которые он намеревается использовать, доступны в вашей учетной записи.

Если вы закрепили версию модели, которая старше текущего значения по умолчанию Claude Code, и ваша учетная запись может вызывать более новую версию, Claude Code предлагает вам обновить закрепление. Принятие записывает новый ID модели в ваш [файл параметров пользователя](/docs/ru/settings) и перезапускает Claude Code. Отклонение запоминается до следующего изменения версии по умолчанию. Закрепления, указывающие на [ARN профиля вывода приложения](#map-each-model-version-to-an-inference-profile), пропускаются, так как они управляются вашим администратором.

Если вы не закрепили модель и текущее значение по умолчанию недоступно в вашей учетной записи, Claude Code возвращается к предыдущей версии для текущего сеанса и показывает уведомление. Оно сначала пытается использовать более ранние версии модели по умолчанию и, когда модель по умолчанию является моделью Opus и ни одна версия Opus недоступна, возвращается к модели Sonnet по умолчанию. Возврат не сохраняется. Включите более новую модель в вашей учетной записи Amazon Bedrock или [закрепите версию](#4-pin-model-versions), чтобы сделать выбор постоянным.

<h2 id="iam-configuration">
  Конфигурация IAM
</h2>

Создайте политику IAM с необходимыми разрешениями для Claude Code:

```json theme={null}
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Sid": "AllowModelAndInferenceProfileAccess",
      "Effect": "Allow",
      "Action": [
        "bedrock:InvokeModel",
        "bedrock:InvokeModelWithResponseStream",
        "bedrock:ListInferenceProfiles",
        "bedrock:GetInferenceProfile"
      ],
      "Resource": [
        "arn:aws:bedrock:*:*:inference-profile/*",
        "arn:aws:bedrock:*:*:application-inference-profile/*",
        "arn:aws:bedrock:*:*:foundation-model/*"
      ]
    },
    {
      "Sid": "AllowMarketplaceSubscription",
      "Effect": "Allow",
      "Action": [
        "aws-marketplace:ViewSubscriptions",
        "aws-marketplace:Subscribe"
      ],
      "Resource": "*",
      "Condition": {
        "StringEquals": {
          "aws:CalledViaLast": "bedrock.amazonaws.com"
        }
      }
    }
  ]
}
```

Для более ограничительных разрешений вы можете ограничить Resource конкретными ARN профилей вывода.

`bedrock:GetInferenceProfile` позволяет Claude Code разрешить [ARN профиля вывода приложения](#map-each-model-version-to-an-inference-profile) в его базовую модель фундамента, которая используется для выбора правильной формы запроса для этой модели.

Если токену не хватает этого разрешения, Claude Code автоматически восстанавливается, повторив попытку один раз с альтернативной формой, поэтому запросы все еще успешны, но каждая новая модель добавляет дополнительный обход туда и обратно. Предоставление разрешения избегает повтора. Это применяется чаще всего к развертываниям `AWS_BEARER_TOKEN_BEDROCK`, где политика токена обычно уже, чем полная роль IAM.

Для получения подробной информации см. [документацию Bedrock IAM](https://docs.aws.amazon.com/bedrock/latest/userguide/security-iam.html).

<Note>
  Создайте выделенную учетную запись AWS для Claude Code, чтобы упростить отслеживание затрат и контроль доступа.
</Note>

<h2 id="1m-token-context-window">
  Окно контекста 1M токенов
</h2>

Claude Sonnet 5, Opus 4.6 и более поздние версии, а также Sonnet 4.6 поддерживают [окно контекста 1M токенов](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) на Amazon Bedrock. Sonnet 5 обслуживается через [конечную точку Mantle](#use-the-mantle-endpoint) и всегда работает с окном 1M, без варианта `[1m]` для выбора. Для других моделей Claude Code автоматически включает расширенное окно контекста при выборе варианта модели 1M.

[Мастер установки](#sign-in-with-bedrock) предлагает опцию контекста 1M при закреплении моделей. Чтобы включить его для вручную закрепленной модели вместо этого, добавьте `[1m]` к ID модели. См. [Pin models for third-party deployments](/docs/ru/model-config#pin-models-for-third-party-deployments) для получения подробной информации.

<h2 id="service-tiers">
  Уровни обслуживания
</h2>

[Уровни обслуживания Amazon Bedrock](https://docs.aws.amazon.com/bedrock/latest/userguide/service-tiers-inference.html) позволяют вам выбирать между стоимостью и задержкой. Установите `ANTHROPIC_BEDROCK_SERVICE_TIER` на `default`, `flex` или `priority`:

```bash theme={null}
export ANTHROPIC_BEDROCK_SERVICE_TIER=priority
```

Claude Code отправляет это как заголовок `X-Amzn-Bedrock-Service-Tier` в каждом запросе. Доступность уровня варьируется по модели и региону. Зарезервированная емкость использует [provisioned throughput](https://docs.aws.amazon.com/bedrock/latest/userguide/prov-throughput.html) ARN в качестве ID модели вместо этого параметра.

<h2 id="aws-guardrails">
  AWS Guardrails
</h2>

[Amazon Bedrock Guardrails](https://docs.aws.amazon.com/bedrock/latest/userguide/guardrails.html) позволяют вам реализовать фильтрацию контента для Claude Code. Создайте Guardrail в [консоли Amazon Bedrock](https://console.aws.amazon.com/bedrock/), опубликуйте версию, затем добавьте заголовки Guardrail в ваш [файл параметров](/docs/ru/settings). Включите Cross-Region inference на вашем Guardrail, если вы используете профили вывода между регионами.

Пример конфигурации:

```json theme={null}
{
  "env": {
    "ANTHROPIC_CUSTOM_HEADERS": "X-Amzn-Bedrock-GuardrailIdentifier: your-guardrail-id\nX-Amzn-Bedrock-GuardrailVersion: 1"
  }
}
```

<h2 id="use-the-mantle-endpoint">
  Используйте конечную точку Mantle
</h2>

Mantle - это конечная точка Amazon Bedrock, которая обслуживает модели Claude через форму собственного API Anthropic, а не через Amazon Bedrock Invoke API. Она использует те же учетные данные AWS, разрешения IAM и конфигурацию `awsAuthRefresh`, описанные ранее на этой странице.

<h3 id="enable-mantle">
  Включите Mantle
</h3>

С уже настроенными учетными данными AWS установите `CLAUDE_CODE_USE_MANTLE` для маршрутизации запросов на конечную точку Mantle:

```bash theme={null}
export CLAUDE_CODE_USE_MANTLE=1
export AWS_REGION=us-east-1
```

Claude Code конструирует URL конечной точки из региона AWS. Начиная с версии v2.1.172, регион разрешается с той же приоритетностью, что и [Amazon Bedrock выше](#3-configure-claude-code); более ранние версии используют только `AWS_REGION`. Чтобы переопределить URL для пользовательской конечной точки или шлюза, установите `ANTHROPIC_BEDROCK_MANTLE_BASE_URL`.

Запустите `/status` внутри Claude Code для подтверждения. Строка поставщика показывает `Amazon Bedrock (Mantle)`, когда Mantle активен.

<h3 id="select-a-mantle-model">
  Выберите модель Mantle
</h3>

Mantle использует ID моделей с префиксом `anthropic.` и без суффикса версии, например `anthropic.claude-sonnet-5` или `anthropic.claude-haiku-4-5`. Модели, доступные вашей учетной записи, зависят от того, что было предоставлено вашей организацией; дополнительные ID моделей указаны в ваших материалах по подключению от AWS. Свяжитесь с вашей командой учетной записи AWS, чтобы запросить доступ к разрешенным моделям.

Установите модель с флагом `--model` или с `/model` внутри Claude Code:

```bash theme={null}
claude --model anthropic.claude-haiku-4-5
```

<h3 id="run-mantle-alongside-the-invoke-api">
  Запустите Mantle рядом с Invoke API
</h3>

Модели, доступные вам на Mantle, могут не включать каждую модель, которую вы используете сегодня. Установка как `CLAUDE_CODE_USE_BEDROCK`, так и `CLAUDE_CODE_USE_MANTLE` позволяет Claude Code вызывать обе конечные точки из одного сеанса. ID моделей, соответствующие формату Mantle, маршрутизируются на Mantle, а все остальные ID моделей идут на Amazon Bedrock Invoke API.

```bash theme={null}
export CLAUDE_CODE_USE_BEDROCK=1
export CLAUDE_CODE_USE_MANTLE=1
```

Чтобы отобразить модель Mantle в средстве выбора `/model`, перечислите ее ID в `availableModels` в вашем [файле параметров](/docs/ru/settings). Этот параметр также ограничивает средство выбора перечисленными записями. Перечисление `anthropic.claude-haiku-4-5` удаляет простой псевдоним `haiku` из средства выбора, поэтому также перечислите префиксы версий или полные ID для версий, которые вы хотите сохранить доступными. ID Mantle и псевдоним `haiku` разрешаются в одно семейство моделей, поэтому объединение сохраняет только более конкретную запись. См. [Merge behavior](/docs/ru/model-config#merge-behavior):

```json theme={null}
{
  "availableModels": ["opus", "sonnet", "claude-haiku-4-5", "anthropic.claude-haiku-4-5"]
}
```

Записи с префиксом `anthropic.` добавляются как пользовательские опции средства выбора и маршрутизируются на Mantle. Замените `anthropic.claude-haiku-4-5` на ID модели, который была предоставлена вашей учетной записи. См. [Restrict model selection](/docs/ru/model-config#restrict-model-selection) для получения информации о том, как `availableModels` взаимодействует с другими параметрами модели.

Когда оба поставщика активны, `/status` показывает `Amazon Bedrock + Amazon Bedrock (Mantle)`.

<h3 id="route-mantle-through-a-gateway">
  Маршрутизируйте Mantle через шлюз
</h3>

Если ваша организация маршрутизирует трафик модели через централизованный [LLM gateway](/docs/ru/llm-gateway), который внедряет учетные данные AWS на стороне сервера, отключите аутентификацию на стороне клиента, чтобы Claude Code отправлял запросы без подписей SigV4 или заголовков `x-api-key`:

```bash theme={null}
export CLAUDE_CODE_USE_MANTLE=1
export CLAUDE_CODE_SKIP_MANTLE_AUTH=1
export ANTHROPIC_BEDROCK_MANTLE_BASE_URL=https://your-gateway.example.com
```

<h3 id="mantle-environment-variables">
  Переменные окружения Mantle
</h3>

Эти переменные специфичны для конечной точки Mantle. См. [Environment variables](/docs/ru/env-vars) для полного списка.

| Переменная                              | Назначение                                                                 |
| :-------------------------------------- | :------------------------------------------------------------------------- |
| `CLAUDE_CODE_USE_MANTLE`                | Включите конечную точку Mantle. Установите на `1` или `true`.              |
| `ANTHROPIC_BEDROCK_MANTLE_BASE_URL`     | Переопределите URL конечной точки Mantle по умолчанию                      |
| `CLAUDE_CODE_SKIP_MANTLE_AUTH`          | Пропустите аутентификацию на стороне клиента для настроек прокси           |
| `ANTHROPIC_SMALL_FAST_MODEL_AWS_REGION` | Переопределите регион AWS для модели класса Haiku (общее с Amazon Bedrock) |

<h2 id="troubleshooting">
  Устранение неполадок
</h2>

<h3 id="authentication-loop-with-sso-and-corporate-proxies">
  Цикл аутентификации с SSO и корпоративными прокси
</h3>

Если вкладки браузера открываются повторно при использовании AWS SSO, удалите параметр `awsAuthRefresh` из вашего [файла параметров](/docs/ru/settings). Это может произойти, когда корпоративные VPN или прокси-серверы с проверкой TLS прерывают браузерный поток SSO. Claude Code рассматривает прерванное соединение как ошибку аутентификации, повторно запускает `awsAuthRefresh` и зацикливается бесконечно.

Если ваша сетевая среда мешает автоматическим браузерным потокам SSO, используйте `aws sso login` вручную перед запуском Claude Code вместо того, чтобы полагаться на `awsAuthRefresh`.

<h3 id="region-issues">
  Проблемы с регионом
</h3>

Если вы столкнулись с проблемами региона:

* Проверьте доступность модели: `aws bedrock list-inference-profiles --region your-region`
* Переключитесь на поддерживаемый регион: `export AWS_REGION=us-east-1`
* Рассмотрите использование профилей вывода для доступа между регионами

Если вы получили ошибку "on-demand throughput isn't supported":

* Укажите модель как ID [профиля вывода](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html)

Claude Code использует Amazon Bedrock [Invoke API](https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_InvokeModelWithResponseStream.html) и не поддерживает Converse API.

<h3 id="streaming-errors-behind-a-gateway-or-proxy">
  Ошибки потоковой передачи за шлюзом или прокси
</h3>

Если запросы потоковой передачи завершаются с ошибкой, которая начинается с `Bedrock streaming response has content-type`, шлюз или прокси между Claude Code и Amazon Bedrock преобразует ответ потоковой передачи. Amazon Bedrock передает ответы в двоичном формате event-stream с типом содержимого `application/vnd.amazon.eventstream`, и Claude Code отклоняет успешный ответ потоковой передачи, который сообщает о другом типе содержимого вместо декодирования тела, которое он не может прочитать. Ошибка называет тип содержимого, который она получила, обычно `text/event-stream` из интеграции Amazon API Gateway и Lambda, которая повторно передает поток как события, отправляемые сервером.

До версии v2.1.208 та же неправильная конфигурация проявлялась как `API Error: Truncated event message received` после того, как весь ответ был буферизирован.

Чтобы исправить это, настройте шлюз на передачу тела ответа `InvokeModelWithResponseStream` и его заголовка `Content-Type` без изменений. Если шлюз переписывает только заголовок и передает двоичное тело без изменений, установите [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1`](/docs/ru/env-vars), чтобы пропустить проверку до исправления шлюза. С отключенной проверкой тело ответа, которое было преобразовано, снова завершается с ошибкой `Truncated event message received`.

<h3 id="zero-token-counts-in-/context">
  Нулевые подсчеты токенов в /context
</h3>

Команда `/context` подсчитывает токены для каждой группы инструментов, отправляя схемы инструментов в API подсчета токенов Amazon Bedrock. В версиях Claude Code до v2.1.196 Amazon Bedrock отклонял этот запрос, потому что схемы содержали поля, которые его API подсчета токенов не принимает, поэтому каждая группа инструментов показывала 0 токенов. Другие строки в разбивке, такие как сообщения и файлы памяти, не затронуты.

Обновитесь до v2.1.196 или более поздней версии.

<h3 id="mantle-endpoint-errors">
  Ошибки конечной точки Mantle
</h3>

Если `/status` не показывает `Amazon Bedrock (Mantle)` после установки `CLAUDE_CODE_USE_MANTLE`, переменная не достигает процесса. Подтвердите, что она экспортирована в оболочке, где вы запустили `claude`, или установите ее в блоке `env` вашего [файла параметров](/docs/ru/settings).

`403` от конечной точки Mantle с действительными учетными данными означает, что вашей учетной записи AWS не был предоставлен доступ к запрошенной модели. Свяжитесь с вашей командой учетной записи AWS, чтобы запросить доступ.

`400`, который называет ID модели, означает, что эта модель не обслуживается на Mantle. Mantle имеет свой собственный набор моделей, отдельный от стандартного каталога Amazon Bedrock, поэтому ID профилей вывода, такие как `us.anthropic.claude-sonnet-4-6`, не будут работать. Используйте ID формата Mantle или включите [обе конечные точки](#run-mantle-alongside-the-invoke-api), чтобы Claude Code маршрутизировал каждый запрос на конечную точку, где модель доступна.

<h2 id="additional-resources">
  Дополнительные ресурсы
</h2>

* [Документация Amazon Bedrock](https://docs.aws.amazon.com/bedrock/)
* [Цены Amazon Bedrock](https://aws.amazon.com/bedrock/pricing/)
* [Профили вывода Amazon Bedrock](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-support.html)
* [Burndown токенов Amazon Bedrock и квоты](https://docs.aws.amazon.com/bedrock/latest/userguide/quotas-token-burndown.html)
* [Claude Code на Amazon Bedrock: Quick Setup Guide](https://community.aws/content/2tXkZKrZzlrlu0KfH8gST5Dkppq/claude-code-on-amazon-bedrock-quick-setup-guide)
* [Claude Code Monitoring Implementation (Amazon Bedrock)](https://github.com/aws-solutions-library-samples/guidance-for-claude-code-with-amazon-bedrock/blob/main/assets/docs/MONITORING.md)
