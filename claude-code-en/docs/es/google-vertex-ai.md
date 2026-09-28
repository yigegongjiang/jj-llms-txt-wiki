> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code en la Plataforma de Agentes de Google Cloud

> Aprenda a configurar Claude Code a través de la Plataforma de Agentes de Google Cloud, anteriormente Vertex AI, incluida la configuración, la configuración de IAM y la solución de problemas.

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
  Requisitos previos
</h2>

Antes de configurar Claude Code con Google Cloud's Agent Platform de Google Cloud, anteriormente conocido como Vertex AI, asegúrese de tener:

* Una cuenta de Google Cloud Platform (GCP) con facturación habilitada
* Un proyecto de GCP con la API de Google Cloud's Agent Platform habilitada
* Acceso a los modelos Claude deseados (por ejemplo, Claude Sonnet 4.6)
* Google Cloud SDK (`gcloud`) instalado y configurado
* Cuota asignada en la región de GCP deseada

Para iniciar sesión con sus propias credenciales de Google Cloud's Agent Platform, siga [Iniciar sesión con Google Cloud's Agent Platform](#sign-in-with-agent-platform) a continuación. Para implementar Claude Code en un equipo, utilice los pasos de [configuración manual](#set-up-manually) y [fije las versiones de su modelo](#5-pin-model-versions) antes de implementar.

<h2 id="sign-in-with-agent-platform">
  Iniciar sesión con Agent Platform
</h2>

Si tiene credenciales de Google Cloud y desea comenzar a usar Claude Code a través de Agent Platform de Google Cloud, el asistente de inicio de sesión lo guía a través del proceso. Completa los requisitos previos del lado de GCP una vez por proyecto; el asistente maneja el lado de Claude Code.

<Steps>
  <Step title="Habilitar modelos Claude en su proyecto de GCP">
    [Habilite la API de Agent Platform de Google Cloud](#1-enable-agent-platform-api) para su proyecto, luego solicite acceso a los modelos Claude que desee en el [Model Garden de Agent Platform de Google Cloud](https://console.cloud.google.com/vertex-ai/model-garden). Consulte [Configuración de IAM](#iam-configuration) para los permisos que su cuenta necesita.
  </Step>

  <Step title="Inicie Claude Code y elija Agent Platform de Google Cloud">
    Ejecute `claude`. En el mensaje de inicio de sesión, seleccione **3rd-party platform**, luego **Google Vertex AI**, la etiqueta que el mensaje de inicio de sesión aún utiliza para Agent Platform de Google Cloud. Si ya ha iniciado sesión, ejecute `/login` para abrir el mismo menú.
  </Step>

  <Step title="Siga los mensajes del asistente">
    Elija cómo se autentica en Google Cloud: Credenciales predeterminadas de aplicación de `gcloud`, un archivo de clave de cuenta de servicio, o credenciales ya en su entorno. El asistente detecta su proyecto y región, verifica qué modelos Claude puede invocar su proyecto, y le permite fijarlos. Guarda el resultado en el bloque `env` de su [archivo de configuración de usuario](/docs/es/settings), por lo que no necesita exportar variables de entorno usted mismo.
  </Step>
</Steps>

Después de haber iniciado sesión, ejecute `/setup-vertex` en cualquier momento para reabrirlo el asistente y cambiar sus credenciales, proyecto, región o fijaciones de modelo. El paso de fijación de modelo comienza desde sus modelos actualmente fijados. El asistente escribe en `~/.claude/settings.json`, o en `$CLAUDE_CONFIG_DIR/settings.json` cuando [`CLAUDE_CONFIG_DIR`](/docs/es/env-vars#variables) está configurado.

<h2 id="region-configuration">
  Configuración de región
</h2>

Claude Code admite puntos finales de Google Cloud's Agent Platform [globales](https://cloud.google.com/blog/products/ai-machine-learning/global-endpoint-for-claude-models-generally-available-on-vertex-ai), multirregión y regionales. Establezca `CLOUD_ML_REGION` en `global`, una ubicación multirregión como `eu` o `us`, o una región específica como `us-east5`. Claude Code selecciona el nombre de host correcto de Google Cloud's Agent Platform para cada formulario, incluidos los hosts `aiplatform.eu.rep.googleapis.com` y `aiplatform.us.rep.googleapis.com` para ubicaciones multirregión.

<Note>
  Google Cloud's Agent Platform puede no admitir los modelos predeterminados de Claude Code en todos los tipos de puntos finales. La disponibilidad del modelo varía según [regiones específicas](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/locations#genai-partner-models), ubicaciones multirregión y [puntos finales globales](https://cloud.google.com/vertex-ai/generative-ai/docs/partner-models/use-partner-models#supported_models). Es posible que deba cambiar a una ubicación compatible o especificar un modelo compatible.
</Note>

<h2 id="set-up-manually">
  Configurar manualmente
</h2>

Para configurar Google Cloud's Agent Platform a través de variables de entorno en lugar del asistente, por ejemplo en CI o una implementación empresarial con script, siga los pasos a continuación.

<h3 id="1-enable-agent-platform-api">
  1. Habilitar la API de Agent Platform
</h3>

Habilite la API de Agent Platform de Google Cloud en su proyecto de GCP. Reemplace `YOUR-PROJECT-ID` con su ID de proyecto de GCP aquí y en el paso de configuración a continuación:

```bash theme={null}
# Establezca su ID de proyecto
gcloud config set project YOUR-PROJECT-ID

# Habilitar la API de Agent Platform
gcloud services enable aiplatform.googleapis.com
```

<h3 id="2-request-model-access">
  2. Solicitar acceso al modelo
</h3>

Solicite acceso a los modelos Claude en Google Cloud's Agent Platform:

1. Navegue al [Google Cloud's Agent Platform Model Garden](https://console.cloud.google.com/vertex-ai/model-garden)
2. Busque modelos "Claude"
3. Solicite acceso a los modelos Claude deseados (por ejemplo, Claude Sonnet 4.6)
4. Espere la aprobación (puede tomar 24-48 horas)

<h3 id="3-configure-gcp-credentials">
  3) Configurar credenciales de GCP
</h3>

Claude Code utiliza la autenticación estándar de Google Cloud.

Para obtener más información, consulte la [documentación de autenticación de Google Cloud](https://cloud.google.com/docs/authentication).

Claude Code admite [Federación de identidad de carga de trabajo basada en certificados X.509](https://cloud.google.com/iam/docs/workload-identity-federation-with-x509-certificates) a través de la misma cadena de credenciales de aplicación predeterminada. Establezca `GOOGLE_APPLICATION_CREDENTIALS` en la ruta de su archivo de configuración de credenciales.

<Note>
  Claude Code dirige las solicitudes de Google Cloud's Agent Platform al proyecto en `ANTHROPIC_VERTEX_PROJECT_ID`, incluso cuando `GCLOUD_PROJECT`, `GOOGLE_CLOUD_PROJECT`, o el archivo de credenciales referenciado por `GOOGLE_APPLICATION_CREDENTIALS` lleva un proyecto diferente.
</Note>

<h4 id="advanced-credential-configuration">
  Configuración avanzada de credenciales
</h4>

Claude Code admite la actualización automática de credenciales para GCP a través de la configuración `gcpAuthRefresh`. Agréguela a su archivo de [configuración](/docs/es/settings) de Claude Code, por ejemplo `~/.claude/settings.json`. Cuando Claude Code detecta que sus credenciales de GCP han expirado o no se pueden cargar, ejecuta el comando configurado para obtener nuevas credenciales antes de reintentar la solicitud.

```json theme={null}
{
  "gcpAuthRefresh": "gcloud auth application-default login",
  "env": {
    "ANTHROPIC_VERTEX_PROJECT_ID": "your-project-id"
  }
}
```

Antes de ejecutar el comando, Claude Code solicita un token de acceso con sus credenciales actuales para confirmar que realmente han expirado, y omite el comando cuando aún funcionan.

Si la verificación no se completa en cinco segundos, Claude Code también omite el comando y lo ejecuta solo después de que una solicitud falla con un error de credenciales. Antes de v2.1.261, una verificación que agotó el tiempo de espera se contaba como una credencial expirada, por lo que el comando podría abrir su navegador al inicio incluso aunque sus credenciales aún fueran válidas.

Claude Code le muestra la salida del comando, pero no puede enviar entrada interactiva al comando. Esto funciona bien para flujos de autenticación basados en navegador donde la CLI muestra una URL y usted completa la autenticación en el navegador. El comando de actualización agota el tiempo de espera después de tres minutos si la autenticación no se completa. Si establece `gcpAuthRefresh` en la configuración del proyecto como `.claude/settings.json`, Claude Code lo ejecuta bajo la misma [regla de confianza del espacio de trabajo que los hooks en archivos de configuración](/docs/es/permissions#what-runs-before-you-trust-a-folder), que incluye sesiones `-p` en carpetas que nunca ha confiado.

<h3 id="4-configure-claude-code">
  4. Configurar Claude Code
</h3>

Establezca las siguientes variables de entorno:

```bash theme={null}
# Habilitar la integración de Agent Platform
export CLAUDE_CODE_USE_VERTEX=1
export CLOUD_ML_REGION=global
export ANTHROPIC_VERTEX_PROJECT_ID=YOUR-PROJECT-ID

# Opcional: Anular la URL del punto final de Agent Platform para puntos finales personalizados o puertas de enlace
# export ANTHROPIC_VERTEX_BASE_URL=https://aiplatform.googleapis.com

# Cuando CLOUD_ML_REGION=global, anule la región para modelos que no admiten puntos finales globales
export VERTEX_REGION_CLAUDE_HAIKU_4_5=us-east5
export VERTEX_REGION_CLAUDE_4_6_SONNET=europe-west1
```

La mayoría de las versiones de modelo tienen una variable `VERTEX_REGION_CLAUDE_*` correspondiente. Consulte la [referencia de variables de entorno](/docs/es/env-vars) para obtener la lista completa. Verifique [Google Cloud's Agent Platform Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) para determinar qué modelos admiten puntos finales globales frente a solo regionales.

Si un valor de región no tiene forma de nombre de región o ubicación, Claude Code lo trata como no establecido. Por ejemplo, Claude Code trata un valor que contiene una barra diagonal, punto o espacio como no establecido. Claude Code retrocede a una fuente diferente para cada variable:

* `VERTEX_REGION_CLAUDE_*`: Claude Code retrocede a `CLOUD_ML_REGION`.
* `CLOUD_ML_REGION`: Claude Code retrocede a `us-east5`.

[Prompt caching](/docs/es/prompt-caching) se habilita automáticamente. Para deshabilitarlo, establezca `DISABLE_PROMPT_CACHING=1`. Para solicitar un TTL de caché de 1 hora en lugar del predeterminado de 5 minutos, establezca `ENABLE_PROMPT_CACHING_1H=1`; las escrituras de caché con un TTL de 1 hora se facturan a una tarifa más alta. Para establecer diferentes TTL para su conversación principal y para las solicitudes que Claude Code realiza fuera de ella, [elija el TTL usted mismo](/docs/es/prompt-caching#choose-the-ttl-yourself).

Para aumentar sus límites de velocidad, póngase en contacto con el soporte de Google Cloud. Al usar Google Cloud's Agent Platform, el comando `/logout` no está disponible ya que la autenticación se maneja a través de credenciales de Google Cloud.

Claude Code decide entre [búsqueda de herramientas MCP](/docs/es/mcp#scale-with-mcp-tool-search) y carga por adelantado según la generación del modelo:

* **Claude Opus 4.5, Sonnet 4.5, Haiku 4.5 y posterior**: Claude Code habilita la búsqueda de herramientas de forma predeterminada.
* **Modelos anteriores, incluidos todos los modelos Claude 3.x**: Claude Code carga las definiciones de herramientas MCP por adelantado, porque sus pilas de servicio de Agent Platform rechazan el encabezado beta requerido. Establecer `ENABLE_TOOL_SEARCH=true` no anula esto.

Establezca `ENABLE_TOOL_SEARCH=false` para deshabilitar la búsqueda de herramientas en todos los modelos. Antes de v2.1.221, Claude Code deshabilitaba la búsqueda de herramientas para todos los modelos en Google Cloud's Agent Platform a menos que estableciera `ENABLE_TOOL_SEARCH=true`.

<h3 id="5-pin-model-versions">
  5. Fijar versiones de modelo
</h3>

<Warning>
  Fije versiones de modelo específicas al implementar para varios usuarios. Sin fijar, alias de modelo como `sonnet` y `opus` se resuelven al valor predeterminado integrado de Claude Code para Google Cloud's Agent Platform, que puede estar rezagado con respecto a la versión más reciente y puede que aún no esté habilitado en su proyecto. Claude Code [retrocede](#startup-model-checks) a una versión anterior o modelo de nivel inferior al inicio cuando el valor predeterminado no está disponible, pero fijar le permite controlar cuándo sus usuarios se mueven a un nuevo modelo.
</Warning>

Establezca estas variables de entorno en ID de modelo específicos de Google Cloud's Agent Platform.

Sin `ANTHROPIC_DEFAULT_OPUS_MODEL`, el alias `opus` en Google Cloud's Agent Platform se resuelve a Opus 5.5, y sin `ANTHROPIC_DEFAULT_SONNET_MODEL`, el alias `sonnet` se resuelve a Sonnet 4.5. Este ejemplo fija cada alias a una versión específica:

```bash theme={null}
export ANTHROPIC_DEFAULT_OPUS_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_SONNET_MODEL='claude-sonnet-5'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5@20251001'
```

Para los ID de modelo actuales y heredados, consulte [Descripción general de modelos](https://platform.claude.com/docs/en/about-claude/models/overview). Consulte [Configuración de modelo](/docs/es/model-config#pin-models-for-third-party-deployments) para obtener la lista completa de variables de entorno.

Claude Code utiliza estos modelos predeterminados cuando no se establecen variables de fijación:

| Tipo de modelo        | Valor predeterminado         |
| :-------------------- | :--------------------------- |
| Modelo principal      | `claude-opus-5-5`            |
| Modelo pequeño/rápido | `claude-sonnet-4-5@20250929` |

Las tareas en segundo plano como la generación de títulos de sesión utilizan el modelo pequeño/rápido, normalmente un modelo de clase Haiku. En Google Cloud's Agent Platform, Claude Code utiliza el modelo Sonnet predeterminado para tareas en segundo plano porque Haiku puede no estar habilitado en todos los proyectos o regiones. Dos selecciones cambian qué modelo las realiza:

* Cuando selecciona un modelo principal con `--model`, `ANTHROPIC_MODEL`, o la configuración `model`, las tareas en segundo plano utilizan ese modelo. Cuando Claude Code inicia la sesión en el modelo que establece con [`ANTHROPIC_DEFAULT_MODEL`](/docs/es/model-config#set-a-default-model-for-new-sessions), las tareas en segundo plano utilizan ese modelo también. Establecer `ANTHROPIC_DEFAULT_OPUS_MODEL` sin `ANTHROPIC_DEFAULT_SONNET_MODEL` también cuenta como una selección, porque el modelo Sonnet integrado puede no estar habilitado en un proyecto que dirige su propio Opus.
* Para usar Haiku para tareas en segundo plano, establezca `ANTHROPIC_DEFAULT_HAIKU_MODEL` en un ID de modelo que esté disponible en su proyecto.

<Warning>
  Los modelos Opus tienen un precio por token más alto que los modelos Sonnet, por lo que una implementación que no fija un modelo principal se factura a la tarifa de Opus una vez que se actualiza a v2.1.207 o posterior. Para mantener Sonnet 4.5 como el modelo principal, establezca `ANTHROPIC_MODEL` en su ID de modelo completo. Una implementación que dirige el valor predeterminado con `ANTHROPIC_DEFAULT_SONNET_MODEL` y no establece `ANTHROPIC_DEFAULT_OPUS_MODEL` mantiene su modelo Sonnet dirigido como el valor predeterminado.
</Warning>

Antes de v2.1.280, el modelo principal en Google Cloud's Agent Platform se establecía de forma predeterminada en Opus 5 y el alias `opus` se resolvía a Opus 5 desde v2.1.219. En v2.1.207 a v2.1.218, el modelo principal en Google Cloud's Agent Platform se establecía de forma predeterminada en Opus 4.8 y el alias `opus` se resolvía a Opus 4.8. Antes de v2.1.207, el modelo principal se establecía de forma predeterminada en Sonnet 4.5, el alias `opus` se resolvía a Opus 4.6, y las tareas en segundo plano siempre utilizaban el modelo principal.

Para personalizar aún más los modelos:

```bash theme={null}
export ANTHROPIC_MODEL='claude-opus-4-8'
export ANTHROPIC_DEFAULT_HAIKU_MODEL='claude-haiku-4-5@20251001'
```

<h3 id="6-verify-your-configuration">
  6. Verificar su configuración
</h3>

Inicie Claude Code y ejecute `/status` para confirmar la configuración. La línea `API provider` muestra `Google Vertex AI`, y las líneas `GCP project`, `Default region` y `Model` muestran su ID de proyecto, región y modelo resuelto. Si falta la línea del proveedor, las variables de entorno no llegan al proceso. Confirme que se exportan en el shell donde lanzó `claude`, o establézcalas en el bloque `env` de su [archivo de configuración](/docs/es/settings).

<h2 id="startup-model-checks">
  Verificaciones de modelo al inicio
</h2>

Cuando Claude Code se inicia con Google Cloud's Agent Platform configurado, verifica que los modelos que pretende usar sean accesibles en su proyecto.

Si ha fijado una versión de modelo que es más antigua que el valor predeterminado actual de Claude Code, y su proyecto puede invocar la versión más reciente, Claude Code le solicita que actualice la fijación. Aceptar escribe el nuevo ID de modelo en su [archivo de configuración de usuario](/docs/es/settings) y reinicia Claude Code. Rechazar se recuerda hasta el próximo cambio de versión predeterminada.

Si no ha fijado un modelo y el valor predeterminado actual no está disponible en su proyecto, Claude Code retrocede para la sesión actual y muestra un aviso. Intenta primero versiones anteriores del modelo predeterminado y, cuando el valor predeterminado es un modelo Opus y no hay ninguna versión de Opus disponible, retrocede al modelo Sonnet predeterminado. El retroceso no se persiste. Habilite el modelo más reciente en [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) o [fije una versión](#5-pin-model-versions) para hacer la opción permanente.

Cuando inicia la sesión en una versión específica de Sonnet u Opus, por ejemplo con `--model`, `ANTHROPIC_MODEL`, o la [configuración `model`](/docs/es/settings-reference#model), esa versión actúa como el valor predeterminado fijado de la sesión para el alias `sonnet` u `opus` correspondiente. Claude Code omite la verificación de disponibilidad para el valor predeterminado integrado que su modelo reemplaza e inicia en el modelo que configuró, sin aviso de retroceso.

Los alias de modelo como `opus` no actúan como fijaciones, ni tampoco un ID de modelo que Claude Code no reconoce.

<h2 id="iam-configuration">
  Configuración de IAM
</h2>

Asigne el rol `roles/aiplatform.user`, que incluye los permisos requeridos:

* `aiplatform.endpoints.predict` - Requerido para la invocación de modelo y conteo de tokens

Para permisos más restrictivos, cree un rol personalizado con solo los permisos anteriores.

Para obtener más detalles, consulte la [documentación de IAM de Google Cloud Agent Platform](https://cloud.google.com/vertex-ai/docs/general/access-control).

<Note>
  Cree un proyecto de GCP dedicado para Claude Code para simplificar el seguimiento de costos y el control de acceso.
</Note>

<h2 id="1m-token-context-window">
  Ventana de contexto de 1M de tokens
</h2>

Claude Sonnet 5, Opus 4.6 y posteriores, y Sonnet 4.6 admiten la [ventana de contexto de 1M de tokens](https://platform.claude.com/docs/es/build-with-claude/context-windows#context-window-sizes-by-model) en Google Cloud's Agent Platform. Sonnet 5 siempre se ejecuta con la ventana de 1M, sin ninguna variante `[1m]` para seleccionar. Para los otros modelos, Claude Code habilita automáticamente la ventana de contexto extendida cuando selecciona una variante de modelo de 1M.

El [asistente de configuración](#sign-in-with-agent-platform) ofrece una opción de contexto de 1M cuando fija modelos. Para habilitarlo para un modelo fijado manualmente en su lugar, agregue `[1m]` al ID del modelo. Consulte [Fijar modelos para implementaciones de terceros](/docs/es/model-config#pin-models-for-third-party-deployments) para obtener detalles.

<h2 id="troubleshooting">
  Solución de problemas
</h2>

Si encuentra errores "No se pudieron cargar las credenciales predeterminadas":

* Ejecute `gcloud auth application-default login` para configurar las credenciales predeterminadas de la aplicación
* Establezca `GOOGLE_APPLICATION_CREDENTIALS` en una ruta de archivo de clave de cuenta de servicio
* Consulte [Configurar credenciales de GCP](#3-configure-gcp-credentials) para todas las opciones

Si encuentra problemas de cuota:

* Verifique las cuotas actuales o solicite un aumento de cuota a través de [Cloud Console](https://cloud.google.com/docs/quotas/view-manage)

Si encuentra errores "modelo no encontrado" 404:

* Confirme que el modelo está habilitado en [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden)
* Verifique que el modelo esté disponible en la ubicación que especificó. Algunos modelos se ofrecen solo en ubicaciones `global` o multirregión como `eu` y `us`, no en regiones específicas
* Si utiliza `CLOUD_ML_REGION=global`, verifique que sus modelos admitan puntos finales globales en [Model Garden](https://console.cloud.google.com/vertex-ai/model-garden) en "Características compatibles". Para modelos que no admiten puntos finales globales, ya sea:
  * Especifique un modelo compatible a través de `ANTHROPIC_MODEL` o `ANTHROPIC_DEFAULT_HAIKU_MODEL`, o
  * Establezca una región o ubicación multirregión usando variables de entorno `VERTEX_REGION_<MODEL_NAME>`

Si encuentra errores 429:

* Para puntos finales regionales, asegúrese de que el modelo principal y el modelo pequeño/rápido sean compatibles en su región seleccionada
* Considere cambiar a `CLOUD_ML_REGION=global` para una mejor disponibilidad

<h2 id="additional-resources">
  Recursos adicionales
</h2>

* [Documentación de Google Cloud's Agent Platform](https://cloud.google.com/vertex-ai/docs)
* [Precios de Google Cloud's Agent Platform](https://cloud.google.com/vertex-ai/pricing)
* [Cuotas y límites de Google Cloud's Agent Platform](https://cloud.google.com/vertex-ai/docs/quotas)
