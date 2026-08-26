> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code на Google Cloud's Agent Platform

> Узнайте о настройке Claude Code через Google Cloud's Agent Platform, ранее известную как Vertex AI, включая установку, конфигурацию IAM и устранение неполадок.

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

<ContactSalesCard surface="vertex" />

<h2 id="prerequisites">
  Предварительные требования
</h2>

Перед настройкой Claude Code с Google Cloud's Agent Platform, ранее известной как Vertex AI, убедитесь, что у вас есть:

* Учетная запись Google Cloud Platform (GCP) с включенной биллингом
* Проект GCP с включенным API Google Cloud's Agent Platform
* Доступ к нужным моделям Claude (например, Claude Sonnet 4.6)
* Установленный и настроенный Google Cloud SDK (`gcloud`)
* Квота, выделенная в нужном регионе GCP

Чтобы войти со своими учетными данными Google Cloud's Agent Platform, следуйте инструкциям [Вход с Google Cloud's Agent Platform](#sign-in-with-agent-platform) ниже. Чтобы развернуть Claude Code для команды, используйте шаги [ручной установки](#set-up-manually) и [закрепите версии ваших моделей](#5-pin-model-versions) перед развертыванием.

<h2 id="sign-in-with-agent-platform">
  Вход с Agent Platform
</h2>

Если у вас есть учетные данные Google Cloud и вы хотите начать использовать Claude Code через Agent Platform Google Cloud, мастер входа проведет вас через этот процесс. Вы выполняете предварительные требования на стороне GCP один раз для каждого проекта; мастер обрабатывает сторону Claude Code.

<Steps>
  <Step title="Включите модели Claude в вашем проекте GCP">
    [Включите API Agent Platform Google Cloud](#1-enable-agent-platform-api) для вашего проекта, затем запросите доступ к моделям Claude, которые вам нужны, в [Model Garden Agent Platform Google Cloud](https://console.cloud.google.com/vertex-ai/model-garden). См. [Конфигурация IAM](#iam-configuration) для разрешений, которые требуются вашей учетной записи.
  </Step>

  <Step title="Запустите Claude Code и выберите Agent Platform Google Cloud">
    Запустите `claude`. В приглашении входа выберите **3rd-party platform**, затем **Google Vertex AI**, метку, которую мастер входа все еще использует для Agent Platform Google Cloud.
  </Step>

  <Step title="Следуйте подсказкам мастера">
    Выберите способ аутентификации в Google Cloud: Application Default Credentials из `gcloud`, файл ключа сервисного аккаунта или учетные данные, уже находящиеся в вашей среде. Мастер обнаруживает ваш проект и регион, проверяет, какие модели Claude может вызывать ваш проект, и позволяет вам их закрепить. Результат сохраняется в блок `env` вашего [файла пользовательских настроек](/docs/ru/settings), поэтому вам не нужно самостоятельно экспортировать переменные окружения.
  </Step>
</Steps>

После входа запустите `/setup-vertex` в любое время, чтобы снова открыть мастер и изменить учетные данные, проект, регион или закрепления моделей. Шаг закрепления модели начинается с ваших текущих закрепленных моделей. Мастер записывает данные в `~/.claude/settings.json` или в `$CLAUDE_CONFIG_DIR/settings.json`, когда установлена переменная [`CLAUDE_CONFIG_DIR`](/docs/ru/env-vars#variables).

<h2 id="region-configuration">
  Конфигурация региона
</h2>

Claude Code поддерживает Google Cloud's Agent Platform [глобальные](https://cloud.google.com/blog/products/ai-machine-learning/global-endpoint-for-claude-models-generally-available-on-vertex-ai), многорегиональные и региональные конечные точки. Установите `CLOUD_ML_REGION` на `global`, многорегиональное местоположение, такое как `eu` или `us`, или конкретный регион, такой как `us-east5`. Claude Code выбирает правильное имя хоста Google Cloud's Agent Platform для каждой формы, включая хосты `aiplatform.eu.rep.googleapis.com` и `aiplatform.us.rep.googleapis.com` для многорегиональных местоположений.

<Note>
  Google Cloud's Agent Platform может не поддерживать модели Claude Code по умолчанию на каждом типе конечной точки. Доступность моделей варьируется в зависимости от [конкретных регионов](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations#genai-partner-models), многорегиональных местоположений и [глобальных конечных точек](https://cloud.google.com/vertex-ai/generative-ai/docs/partner-models/use-partner-models#supported_models). Вам может потребоваться переключиться на поддерживаемое местоположение или указать поддерживаемую модель.
</Note>

<h2 id="set-up-manually">
  Ручная установка
</h2>

Чтобы настроить Google Cloud's Agent Platform через переменные окружения вместо мастера, например в CI или при развертывании в масштабах предприятия, следуйте приведенным ниже шагам.

<h3 id="1-enable-agent-platform-api">
  1. Включите API Agent Platform
</h3>

Включите API Agent Platform Google Cloud в вашем проекте GCP:

```bash theme={null}
# Установите ID вашего проекта
gcloud config set project YOUR-PROJECT-ID

# Включите API Agent Platform
gcloud services enable aiplatform.googleapis.com
```

<h3 id="2-request-model-access">
  2. Запросите доступ к модели
</h3>

Запросите доступ к моделям Claude в Google Cloud's Agent Platform:

1. Перейдите в [Google Cloud's Agent Platform Model Garden](https://console.cloud.google.com/vertex-ai/model-garden)
2. Найдите модели "Claude"
3. Запросите доступ к нужным моделям Claude (например, Claude Sonnet 4.6)
4. Дождитесь одобрения (может занять 24-48 часов)

<h3 id="3-configure-gcp-credentials">
  3) Настройте учетные данные GCP
</h3>

Claude Code использует стандартную аутентификацию Google Cloud.

Для получения дополнительной информации см. [документацию по аутентификации Google Cloud](https://cloud.google.com/docs/authentication).

Claude Code версии 2.1.121 или позже поддерживает [Федерацию рабочих нагрузок на основе сертификатов X.509](https://cloud.google.com/iam/docs/workload-identity-federation-with-x509-certificates) через ту же цепочку Application Default Credentials. Установите `GOOGLE_APPLICATION_CREDENTIALS` на путь к файлу конфигурации учетных данных.

<Note>
  Claude Code использует `ANTHROPIC_VERTEX_PROJECT_ID` как ID проекта для запросов Google Cloud's Agent Platform. Переменные окружения `GCLOUD_PROJECT` и `GOOGLE_CLOUD_PROJECT` и файл учетных данных, на который ссылается `GOOGLE_APPLICATION_CREDENTIALS`, имеют приоритет над ним. Если ничего из этого не установлено, ID проекта разрешается из вашей конфигурации `gcloud` или присоединенной учетной записи сервиса.
</Note>

<h4 id="advanced-credential-configuration">
  Расширенная конфигурация учетных данных
</h4>

Claude Code поддерживает автоматическое обновление учетных данных GCP через параметр `gcpAuthRefresh`. Когда Claude Code обнаруживает, что ваши учетные данные GCP истекли или не могут быть загружены, он запускает настроенную команду для получения новых учетных данных перед повторной попыткой запроса.

```json theme={null}
{
  "gcpAuthRefresh": "gcloud auth application-default login",
  "env": {
    "ANTHROPIC_VERTEX_PROJECT_ID": "your-project-id"
  }
}
```

Вывод команды отображается пользователю, но интерактивный ввод не поддерживается. Это хорошо работает для потоков аутентификации на основе браузера, где CLI показывает URL, и вы завершаете аутентификацию в браузере. Команда обновления истекает через три минуты, если аутентификация не завершена. Если вы установите `gcpAuthRefresh` в параметрах проекта, таких как `.claude/settings.json`, команда запускается только после того, как вы примете приглашение доверия рабочей области.

<h3 id="4-configure-claude-code">
  4. Настройте Claude Code
</h3>

Установите следующие переменные окружения:

```bash theme={null}
# Включите интеграцию Agent Platform
export CLAUDE_CODE_USE_VERTEX=1
export CLOUD_ML_REGION=global
export ANTHROPIC_VERTEX_PROJECT_ID=YOUR-PROJECT-ID

# Опционально: переопределите URL конечной точки Agent Platform для пользовательских конечных точек или шлюзов
# export ANTHROPIC_VERTEX_BASE_URL=https://aiplatform.googleapis.com

# Опционально: отключите кэширование запросов, если необходимо
export DISABLE_PROMPT_CACHING=1

# Опционально: запросите TTL кэша запросов на 1 час вместо стандартного 5-минутного
export ENABLE_PROMPT_CACHING_1H=1

# Когда CLOUD_ML_REGION=global, переопределите регион для моделей, которые не поддерживают глобальные конечные точки
export VERTEX_REGION_CLAUDE_HAIKU_4_5=us-east5
export VERTEX_REGION_CLAUDE_4_6_SONNET=europe-west1
```

Большинство версий моделей имеют соответствующую переменную `VERTEX_REGION_CLAUDE_*`. Полный список см. в [справочнике переменных окружения](/docs/ru/env-vars). Проверьте [Google Cloud's Agent Platform Model Garden](https://console.cloud.google.com/vertex-ai/model-garden), чтобы определить, какие модели поддерживают глобальные конечные точки в сравнении с региональными только.

[Кэширование запросов](/docs/ru/prompt-caching) включается автоматически. Чтобы отключить его, установите `DISABLE_PROMPT_CACHING=1`. Чтобы запросить TTL кэша на 1 час вместо стандартного 5-минутного, установите `ENABLE_PROMPT_CACHING_1H=1`; записи кэша с TTL на 1 час тарифицируются по более высокому тарифу. Для повышенных лимитов скорости обратитесь в поддержку Google Cloud. При использовании Google Cloud's Agent Platform команда `/logout` недоступна, так как аутентификация обрабатывается через учетные данные Google Cloud.

Claude Code отключает [поиск инструментов MCP](/docs/ru/mcp#scale-with-mcp-tool-search) по умолчанию на Google Cloud's Agent Platform, поэтому определения инструментов MCP загружаются заранее. Google Cloud's Agent Platform поддерживает поиск инструментов для Claude Sonnet 4.5 и позже, а также Claude Opus 4.5 и позже. Установите `ENABLE_TOOL_SEARCH=true`, чтобы включить его на этих моделях. Более ранние модели на Google Cloud's Agent Platform не принимают требуемый бета-заголовок, и запросы не выполняются, если вы включите поиск инструментов с ними.

<h3 id="5-pin-model-versions">
  5. Закрепите версии моделей
</h3>

<Warning>
  Закрепите конкретные версии моделей при развертывании для нескольких пользователей. Без закрепления псевдонимы моделей, такие как `sonnet` и `opus`, разрешаются в встроенное значение по умолчанию Claude Code для Google Cloud's Agent Platform, которое может отставать от последнего выпуска и может быть еще не включено в вашем проекте. Claude Code [откатывается](#startup-model-checks) на предыдущую версию или модель более низкого уровня при запуске, когда значение по умолчанию недоступно, но закрепление позволяет вам контролировать, когда ваши пользователи переходят на новую модель.
</Warning>

Установите эти переменные окружения на конкретные ID моделей Google Cloud's Agent Platform.

Без `ANTHROPIC_DEFAULT_OPUS_MODEL` псевдоним `opus` на Google Cloud's Agent Platform разрешается в Opus 4.8, а без `ANTHROPIC_DEFAULT_SONNET_MODEL` псевдоним `sonnet` разрешается в Sonnet 4.5. Этот пример закрепляет каждый псевдоним на конкретную версию:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='claude-sonnet-5'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5@20251001'
```

Для текущих и устаревших ID моделей см. [Обзор моделей](https://platform.claude.com/docs/en/about-claude/models/overview). Полный список переменных окружения см. в разделе [Конфигурация моделей](/docs/ru/model-config#pin-models-for-third-party-deployments).

Claude Code использует эти модели по умолчанию, когда переменные закрепления не установлены:

| Тип модели           | Значение по умолчанию        |
| :------------------- | :--------------------------- |
| Основная модель      | `claude-opus-4-8`            |
| Малая/быстрая модель | `claude-sonnet-4-5@20250929` |

Фоновые задачи, такие как генерация названия сеанса, используют малую/быструю модель, обычно модель класса Haiku. На Google Cloud's Agent Platform Claude Code использует модель Sonnet по умолчанию для фоновых задач, потому что Haiku может быть не включен в каждом проекте или регионе. Два выбора изменяют, какая модель их выполняет:

* Когда вы выбираете основную модель с помощью `--model`, `ANTHROPIC_MODEL` или параметра `model`, фоновые задачи используют эту модель. Установка `ANTHROPIC_DEFAULT_OPUS_MODEL` без `ANTHROPIC_DEFAULT_SONNET_MODEL` также считается выбором, потому что встроенная модель Sonnet может быть не включена в проекте, который управляет своим собственным Opus.
* Чтобы использовать Haiku для фоновых задач, установите `ANTHROPIC_DEFAULT_HAIKU_MODEL` на ID модели, который доступен в вашем проекте.

<Warning>
  Модели Opus имеют более высокую цену за токен, чем модели Sonnet, поэтому развертывание, которое не закрепляет основную модель, будет тарифицироваться по тарифу Opus после обновления до версии 2.1.207 или позже. Чтобы сохранить Sonnet 4.5 в качестве основной модели, установите `ANTHROPIC_MODEL` на его полный ID модели. Развертывание, которое управляет значением по умолчанию с помощью `ANTHROPIC_DEFAULT_SONNET_MODEL` и не устанавливает `ANTHROPIC_DEFAULT_OPUS_MODEL`, сохраняет свою управляемую модель Sonnet в качестве значения по умолчанию.
</Warning>

До версии 2.1.207 основная модель на Google Cloud's Agent Platform по умолчанию была Sonnet 4.5, псевдоним `opus` разрешался в Opus 4.6, а фоновые задачи всегда использовали основную модель.

Для дальнейшей настройки моделей:

```bash theme={null}
export ANTHROPIC_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5@20251001'
```

<h2 id="startup-model-checks">
  Проверки моделей при запуске
</h2>

Когда Claude Code запускается с настроенной платформой Google Cloud Agent Platform, он проверяет, что модели, которые он намеревается использовать, доступны в вашем проекте.

Если вы закрепили версию модели, которая старше текущего значения по умолчанию Claude Code, и ваш проект может вызывать более новую версию, Claude Code предлагает вам обновить закрепление. Принятие записывает новый ID модели в ваш [файл пользовательских настроек](/docs/ru/settings) и перезапускает Claude Code. Отклонение запоминается до следующего изменения версии по умолчанию.

Если вы не закрепили модель и текущее значение по умолчанию недоступно в вашем проекте, Claude Code откатывается на предыдущую версию для текущего сеанса и показывает уведомление. Сначала он пытается использовать более ранние версии модели по умолчанию и, когда модель по умолчанию является моделью Opus и ни одна версия Opus недоступна, откатывается на модель Sonnet по умолчанию. Откат не сохраняется. Включите более новую модель в [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) или [закрепите версию](#5-pin-model-versions), чтобы сделать выбор постоянным.

<h2 id="iam-configuration">
  Конфигурация IAM
</h2>

Назначьте требуемые разрешения IAM:

Роль `roles/aiplatform.user` включает требуемые разрешения:

* `aiplatform.endpoints.predict` - требуется для вызова модели и подсчета токенов

Для более строгих разрешений создайте пользовательскую роль только с указанными выше разрешениями.

Для получения дополнительной информации см. [документацию Google Cloud Agent Platform IAM](https://cloud.google.com/vertex-ai/docs/general/access-control).

<Note>
  Создайте выделенный проект GCP для Claude Code, чтобы упростить отслеживание затрат и контроль доступа.
</Note>

<h2 id="1m-token-context-window">
  Контекстное окно с 1M токенов
</h2>

Claude Sonnet 5, Opus 4.6 и более поздние версии, а также Sonnet 4.6 поддерживают [контекстное окно с 1M токенов](https://platform.claude.com/docs/en/build-with-claude/context-windows#context-window-sizes-by-model) на платформе Agent Platform Google Cloud. Sonnet 5 всегда работает с окном 1M, без варианта `[1m]` для выбора. Для других моделей Claude Code автоматически включает расширенное контекстное окно при выборе варианта модели с 1M.

[Мастер установки](#sign-in-with-agent-platform) предлагает опцию контекстного окна с 1M при закреплении моделей. Чтобы включить его для вручную закрепленной модели, добавьте `[1m]` к ID модели. Подробности см. в разделе [Закрепите модели для развертываний третьих сторон](/docs/ru/model-config#pin-models-for-third-party-deployments).

<h2 id="troubleshooting">
  Устранение неполадок
</h2>

Если вы столкнулись с ошибками "Could not load the default credentials":

* Запустите `gcloud auth application-default login` для установки Application Default Credentials
* Установите `GOOGLE_APPLICATION_CREDENTIALS` на путь файла ключа сервисного аккаунта
* См. [Configure GCP credentials](#3-configure-gcp-credentials) для всех вариантов

Если вы столкнулись с проблемами квоты:

* Проверьте текущие квоты или запросите увеличение квоты через [Cloud Console](https://cloud.google.com/docs/quotas/view-manage)

Если вы столкнулись с ошибками "model not found" 404:

* Подтвердите, что модель включена в [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden)
* Проверьте, что модель доступна в указанном вами местоположении. Некоторые модели предлагаются только на `global` или многорегиональных местоположениях, таких как `eu` и `us`, а не в конкретных регионах
* Если вы используете `CLOUD_ML_REGION=global`, проверьте, что ваши модели поддерживают глобальные конечные точки в [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) в разделе "Supported features". Для моделей, которые не поддерживают глобальные конечные точки, либо:
  * Укажите поддерживаемую модель через `ANTHROPIC_MODEL` или `ANTHROPIC_DEFAULT_HAIKU_MODEL`, либо
  * Установите регион или многорегиональное местоположение, используя переменные окружения `VERTEX_REGION_<MODEL_NAME>`

Если вы столкнулись с ошибками 429:

* Для региональных конечных точек убедитесь, что основная модель и малая/быстрая модель поддерживаются в выбранном регионе
* Рассмотрите возможность переключения на `CLOUD_ML_REGION=global` для лучшей доступности

<h2 id="additional-resources">
  Дополнительные ресурсы
</h2>

* [Документация Google Cloud's Agent Platform](https://cloud.google.com/vertex-ai/docs)
* [Цены Google Cloud's Agent Platform](https://cloud.google.com/vertex-ai/pricing)
* [Квоты и лимиты Google Cloud's Agent Platform](https://cloud.google.com/vertex-ai/docs/quotas)
