> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code en Claude Platform on AWS

> Configure Claude Code para usar la API de Claude operada por Anthropic con autenticación de AWS, control de acceso IAM y facturación de AWS Marketplace.

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

Claude Platform on AWS es la API de Claude operada por Anthropic con autenticación de AWS, control de acceso IAM y facturación de AWS Marketplace. Las solicitudes llegan directamente a la API de Anthropic, por lo que obtiene los mismos modelos y características de API que la [Claude API](https://platform.claude.com/docs) en el mismo calendario de lanzamientos. Las características del lado del cliente que Claude Code activa a través del servicio de banderas de características de Anthropic, como [`/loop` auto-ritmo](/docs/es/scheduled-tasks#let-claude-choose-the-interval), están desactivadas de forma predeterminada, y la [herramienta de asesor](/docs/es/advisor) no está disponible. Consulte la [matriz de disponibilidad de características](/docs/es/feature-availability#summary-by-provider) para obtener la lista completa. Se autentica con credenciales de AWS o una clave API de espacio de trabajo, y paga a través de AWS Marketplace.

Utilice esta guía para dirigir Claude Code a un espacio de trabajo que ya haya aprovisionado a través de Claude Platform on AWS. Para la suscripción de AWS y la configuración del espacio de trabajo que viene antes, consulte la [documentación de Claude Platform on AWS](https://platform.claude.com/docs/en/build-with-claude/claude-platform-on-aws).

<Note>
  Suscribirse a través de AWS Marketplace aprovisiona una nueva organización de Anthropic vinculada a su cuenta de AWS. Esta organización es independiente de cualquier organización que ya tenga con Anthropic, y las credenciales no se transfieren entre ellas. Utilice el ID del espacio de trabajo y las claves API de la organización vinculada a AWS, no de una cuenta de Claude Console preexistente.
</Note>

<h2 id="prerequisites">
  Requisitos previos
</h2>

Antes de configurar Claude Code, necesita:

* Una suscripción activa a Claude Platform on AWS a través de AWS Marketplace
* Un espacio de trabajo en su organización de Anthropic vinculada a AWS, con su ID de espacio de trabajo
* Un principal de IAM con permiso para invocar el servicio de Anthropic, o una clave API limitada al espacio de trabajo
* Credenciales de AWS en su entorno, en `~/.aws/credentials`, o de un rol de IAM adjunto si desea autenticación SigV4. La CLI de AWS es necesaria solo para el flujo de inicio de sesión SSO.

<h2 id="setup">
  Configuración
</h2>

<h3 id="1-configure-aws-credentials">
  1. Configurar credenciales de AWS
</h3>

Claude Code admite dos métodos de autenticación para Claude Platform on AWS. Elija el método que se ajuste a cómo su equipo gestiona el acceso.

**Opción A: Credenciales de AWS con SigV4**

Claude Code firma solicitudes con SigV4 utilizando la cadena de credenciales estándar de AWS: variables de entorno, credenciales compartidas en `~/.aws/credentials`, roles de IAM, sesiones de AWS SSO y cualquier otra fuente que el SDK de AWS admita.

Para uso local, inicie sesión con la CLI de AWS antes de iniciar Claude Code. El ejemplo a continuación utiliza un perfil de SSO, pero cualquier método que produzca credenciales en las ubicaciones estándar funciona.

```bash theme={null}
aws sso login --profile my-profile
export AWS_PROFILE=my-profile
```

Para CI y automatización, proporcione al ejecutor un rol de IAM con permiso para invocar el servicio de Anthropic y establezca `AWS_REGION`. La cadena de credenciales recoge el rol automáticamente.

Si sus credenciales de SSO expiran durante la sesión, configure [`awsAuthRefresh`](/docs/es/amazon-bedrock#advanced-credential-configuration) para que Claude Code vuelva a ejecutar su comando de inicio de sesión y reintente en lugar de fallar. La actualización automática en Claude Platform on AWS requiere Claude Code v2.1.198 o posterior; las versiones anteriores se detienen con un mensaje para ejecutar `/login`, que no puede actualizar credenciales de AWS. Agregue el comando a su `settings.json`:

```json theme={null}
{
  "awsAuthRefresh": "aws sso login --profile my-profile"
}
```

Con `awsAuthRefresh` configurado, `/login` muestra una opción **Claude Platform on AWS · actualizar credenciales** en **Usando plataformas de terceros**. Al seleccionarla, se ejecuta el comando configurado y se vuelven a leer sus credenciales de AWS sin reiniciar Claude Code.

**Opción B: Clave API del espacio de trabajo**

Una clave API del espacio de trabajo es un secreto de larga duración, útil cuando no desea gestionar credenciales de AWS federados. Genere una en la Consola de AWS en **Claude Platform on AWS → API keys** y establézcala como `ANTHROPIC_AWS_API_KEY`:

```bash theme={null}
export ANTHROPIC_AWS_API_KEY=sk-ant-xxxxx
```

La clave se envía como `x-api-key` y tiene prioridad sobre SigV4, por lo que cualquier credencial de AWS en su entorno se ignora. Las claves API de una organización separada de Claude Console no funcionarán aquí.

Trate las claves API del espacio de trabajo como cualquier otra credencial de producción. El bloque `env` del [archivo de configuración del usuario](/docs/es/settings) es una forma conveniente de limitar la clave a su máquina sin exportarla globalmente.

<Note>
  Los comandos `/login` y `/logout` no lo registran en una suscripción de Claude.ai para Claude Platform on AWS. La autenticación se ejecuta a través de sus credenciales de AWS o clave API del espacio de trabajo. La excepción es la opción **actualizar credenciales** que `/login` muestra cuando `awsAuthRefresh` está configurado, que vuelve a leer sus credenciales de AWS como se describe arriba.
</Note>

<h3 id="2-configure-claude-code">
  2. Configurar Claude Code
</h3>

Establezca las variables de entorno que enrutan Claude Code a través de Claude Platform on AWS en lugar de la API de Anthropic predeterminada.

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export AWS_REGION=us-east-1
```

`ANTHROPIC_AWS_WORKSPACE_ID` es obligatorio y se envía en cada solicitud como el encabezado `anthropic-workspace-id`. La URL base se calcula a partir de `AWS_REGION` como `https://aws-external-anthropic.{region}.api.aws`. Para anular la URL directamente, establezca `ANTHROPIC_AWS_BASE_URL`.

Claude Platform on AWS es opcional incluso cuando hay credenciales de AWS presentes en su entorno. Amazon Bedrock y Microsoft Foundry tienen prioridad en el enrutamiento de proveedores, por lo que desactive `CLAUDE_CODE_USE_BEDROCK` y `CLAUDE_CODE_USE_FOUNDRY` si están establecidos.

<h3 id="3-pin-model-versions">
  3. Fijar versiones de modelo
</h3>

Claude Platform on AWS utiliza los mismos ID de modelo que la API de Claude directa.

Los alias predeterminados `fable`, `opus`, `sonnet` y `haiku` se resuelven a los valores predeterminados integrados de Claude Code para Claude Platform on AWS, que pueden estar retrasados respecto a la versión más reciente. Sin `ANTHROPIC_DEFAULT_OPUS_MODEL`, el alias `opus` se resuelve a Opus 4.8. Antes de v2.1.207, se resolvía a Opus 4.7.

Si implementa Claude Code en un equipo, fije explícitamente los ID de modelo para que un nuevo lanzamiento no mueva a todos a la vez:

```bash theme={null}
export ANTHROPIC_DEFAULT_FABLE_MODEL=claude-fable-5
export ANTHROPIC_DEFAULT_OPUS_MODEL=claude-opus-4-8
export ANTHROPIC_DEFAULT_SONNET_MODEL=claude-sonnet-5
export ANTHROPIC_DEFAULT_HAIKU_MODEL=claude-haiku-4-5
```

Para la lista completa de ID de modelo y alias, consulte [Descripción general de modelos](https://platform.claude.com/docs/en/about-claude/models/overview). Para otras variables relacionadas con modelos, consulte [Configuración de modelo](/docs/es/model-config).

[Prompt caching](/docs/es/prompt-caching) está habilitado automáticamente. Para solicitar un TTL de caché de 1 hora en lugar del predeterminado de 5 minutos, establezca `ENABLE_PROMPT_CACHING_1H=1`. La API factura las escrituras de caché de 1 hora a una tasa más alta. Consulte [precios de prompt caching](https://platform.claude.com/docs/en/build-with-claude/prompt-caching#pricing) para conocer las tarifas.

<h2 id="use-the-agent-sdk">
  Usar el Agent SDK
</h2>

El [Agent SDK](/docs/es/agent-sdk/overview) lee las mismas variables de entorno que la CLI, por lo que cualquier programa que genere el subproceso de Claude Code puede dirigirse a Claude Platform on AWS exportando `CLAUDE_CODE_USE_ANTHROPIC_AWS`, `ANTHROPIC_AWS_WORKSPACE_ID` y `ANTHROPIC_AWS_API_KEY` o credenciales de AWS antes de la llamada.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

process.env.CLAUDE_CODE_USE_ANTHROPIC_AWS = "1";
process.env.ANTHROPIC_AWS_WORKSPACE_ID = "wrkspc_01ABCDEFGHIJKLMN";
process.env.AWS_REGION = "us-east-1";

for await (const msg of query({ prompt: "What's in this repo?" })) {
  console.log(msg);
}
```

Este ejemplo se basa en la cadena de credenciales de AWS ambiental para SigV4. Para autenticarse con una clave API del espacio de trabajo en su lugar, establezca `ANTHROPIC_AWS_API_KEY` de la misma manera. Para la superficie más amplia del Agent SDK, consulte [Descripción general del Agent SDK](/docs/es/agent-sdk/overview).

<h2 id="route-through-a-corporate-proxy">
  Enrutar a través de un proxy corporativo
</h2>

Para enrutar el tráfico a través de un proxy o [puerta de enlace LLM](/docs/es/llm-gateway), establezca `ANTHROPIC_AWS_BASE_URL` en la dirección del proxy. Claude Code envía solicitudes a esa URL con los mismos encabezados de espacio de trabajo y autenticación, por lo que cualquier puerta de enlace que los reenvíe sin cambios funciona.

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export ANTHROPIC_AWS_BASE_URL=https://anthropic-proxy.example.com
```

Si su puerta de enlace firma solicitudes por sí misma, establezca `CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1` para que Claude Code envíe solicitudes sin firmar y permita que la puerta de enlace agregue encabezados SigV4 antes de reenviar a AWS. Si la puerta de enlace requiere su propio token, establézcalo en `ANTHROPIC_AUTH_TOKEN`.

```bash theme={null}
export CLAUDE_CODE_USE_ANTHROPIC_AWS=1
export CLAUDE_CODE_SKIP_ANTHROPIC_AWS_AUTH=1
export ANTHROPIC_AWS_WORKSPACE_ID=wrkspc_01ABCDEFGHIJKLMN
export ANTHROPIC_AWS_BASE_URL=https://anthropic-proxy.example.com
```

<h2 id="troubleshooting">
  Solución de problemas
</h2>

Ejecute `/status` para ver el proveedor resuelto y cualquier ID de espacio de trabajo configurado explícitamente, región, anulación de URL base y configuración de omisión de autenticación. Esta es la forma más rápida de confirmar que Claude Code se dirige a Claude Platform on AWS en absoluto.

<h3 id="403-forbidden-or-accessdenied-on-every-request">
  `403 Forbidden` o `AccessDenied` en cada solicitud
</h3>

El principal de IAM que Claude Code resolvió probablemente carece de permiso para invocar el servicio de Anthropic en su espacio de trabajo. Verifique el rol adjunto a su perfil de AWS o al ejecutor que inició Claude Code, y verifique que tenga las acciones `aws-external-anthropic` documentadas en la [referencia de acciones de IAM](https://platform.claude.com/docs/es/api/claude-platform-on-aws-iam-actions).

Si estableció `ANTHROPIC_AWS_API_KEY`, la clave tiene prioridad sobre SigV4 y una clave obsoleta produce el mismo error. Regenere la clave en la Consola de AWS en **Claude Platform on AWS → API keys** o desactive la variable para volver a sus credenciales de AWS.

<h3 id="requests-fail-with-a-missing-workspace-error">
  Las solicitudes fallan con un error de espacio de trabajo faltante
</h3>

`ANTHROPIC_AWS_WORKSPACE_ID` probablemente no esté establecido o esté vacío. Cada solicitud de Claude Platform on AWS debe incluir el ID del espacio de trabajo. No está implícito en sus credenciales de AWS. Encuentre el ID en **Workspaces** en la página del servicio de la Consola de AWS y expórtelo antes de iniciar Claude Code.

<h3 id="requests-still-go-to-api-anthropic-com">
  Las solicitudes aún van a `api.anthropic.com`
</h3>

`CLAUDE_CODE_USE_ANTHROPIC_AWS` probablemente no esté establecido o esté establecido en un valor que no se analice como verdadero. Establézcalo en `1` y ejecute `/status` para confirmar el proveedor resuelto. Si `CLAUDE_CODE_USE_BEDROCK` o `CLAUDE_CODE_USE_FOUNDRY` también está establecido, esos tienen prioridad sobre Claude Platform on AWS.

<h2 id="additional-resources">
  Recursos adicionales
</h2>

La suscripción a Claude Platform on AWS, la configuración del espacio de trabajo e IAM que viene antes de configurar Claude Code se cubre en la documentación de la plataforma:

* [Descripción general de Claude Platform on AWS](https://platform.claude.com/docs/en/build-with-claude/claude-platform-on-aws): suscripción, configuración del espacio de trabajo y referencia del producto
* [Referencia de acciones de IAM](https://platform.claude.com/docs/en/api/claude-platform-on-aws-iam-actions): permisos y políticas administradas
