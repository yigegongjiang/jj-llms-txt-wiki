> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Límites de gasto de la puerta de enlace de aplicaciones Claude

> Limite el gasto de cada desarrollador a través de la puerta de enlace de aplicaciones Claude por día, semana o mes. Establezca límites con una API de administrador y la puerta de enlace los aplica en vivo en cada solicitud.

Los límites de gasto limitan cuánto puede gastar cada desarrollador a través de su [puerta de enlace de aplicaciones Claude](/docs/es/claude-apps-gateway) en un día, semana o mes determinado. Cuando un desarrollador supera su límite, la puerta de enlace devuelve `429` en su siguiente solicitud y lo bloquea hasta que el período se reinicie o un administrador aumente el límite. Utilice límites de gasto para dar a cada desarrollador, grupo u organización completa un techo en una credencial que todos comparten.

Una puerta de enlace de aplicaciones Claude reenvía toda la inferencia a través de una credencial ascendente compartida, por lo que la factura de su proveedor atribuye todo a esa credencial, no a desarrolladores individuales. Sin límites por desarrollador, una flota de agentes descontrolada puede gastar todo el compromiso de la organización. Los límites de gasto son la vista por desarrollador de la puerta de enlace y el disyuntor en la parte superior de esa factura compartida.

<h2 id="set-a-cap">
  Establecer un límite
</h2>

Con el bloque [`admin:`](/docs/es/claude-apps-gateway-config#admin) configurado en `gateway.yaml`, la puerta de enlace sirve una API de administrador en `/v1/organizations/spend_limits` y aplica límites en vivo en cada solicitud de inferencia. Los límites en sí se establecen a través de esa API, no en `gateway.yaml`; cada solicitud `POST /v1/organizations/spend_limits` crea o reemplaza un límite de `{scope, amount, period}`. La API refleja las formas de cable de los puntos finales de límites de gasto de la [API de administrador](https://platform.claude.com/docs/en/manage-claude/admin-api) pública de Anthropic, por lo que un cliente HTTP escrito contra ese contrato puede dirigirse a la puerta de enlace cambiando su URL base.

Esta solicitud establece un valor predeterminado de toda la organización de \$500 por mes para cada desarrollador:

```bash theme={null}
curl -sS https://claude-gateway.internal.example.com/v1/organizations/spend_limits \
  -H "x-api-key: $GATEWAY_ADMIN_WRITE_KEY" \
  -H "Content-Type: application/json" \
  -d '{"scope": {"type": "organization"}, "amount": "50000", "period": "monthly"}'
```

Esta solicitud superpone un límite más estricto de \$100 por día en cada miembro del grupo `contractors`:

```bash theme={null}
curl -sS https://claude-gateway.internal.example.com/v1/organizations/spend_limits \
  -H "x-api-key: $GATEWAY_ADMIN_WRITE_KEY" \
  -H "Content-Type: application/json" \
  -d '{"scope": {"type": "rbac_group", "rbac_group_id": "contractors"}, "amount": "10000", "period": "daily"}'
```

| Campo        | Valores                                           | Descripción                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| ------------ | ------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `scope.type` | `user`, `rbac_group`, `organization`              | `user` se dirige a un desarrollador por su OpenID Connect (OIDC) `sub`, el ID de usuario estable que asigna su proveedor de identidad; páselo como `scope.user_id`. `rbac_group` se dirige a un [grupo de IdP](/docs/es/claude-apps-gateway-config#managed) por nombre; páselo como `scope.rbac_group_id`. `organization` es el valor predeterminado de toda la organización. La puerta de enlace acepta los tres; el `POST` público de Anthropic es solo para usuarios hoy. |
| `amount`     | Cadena de número entero de centavos USD, o `null` | `null` es ilimitado. `"0"` es un límite cero, que bloquea cada solicitud.                                                                                                                                                                                                                                                                                                                                                                                               |
| `period`     | `daily`, `weekly`, `monthly`                      | Un ámbito puede contener un límite por período, y cada uno se aplica de forma independiente: un desarrollador se bloquea si supera cualquiera de ellos.                                                                                                                                                                                                                                                                                                                 |

Un límite de grupo u organización es un valor predeterminado por puesto que cada miembro hereda, no un grupo compartido. Por período, el límite efectivo de un desarrollador se resuelve en este orden: una anulación por usuario, luego la más restrictiva de sus límites de grupo, luego el valor predeterminado de la organización, luego ilimitado. [`admin.group_limit_mode: max`](/docs/es/claude-apps-gateway-config#admin) invierte el desempate de múltiples grupos a menos restrictivo en su lugar.

<h3 id="authenticate-to-the-admin-api">
  Autenticarse en la API de administrador
</h3>

Envíe uno de:

* Un encabezado `x-api-key` que coincida con una clave en [`admin.write_keys`](/docs/es/claude-apps-gateway-config#admin) para acceso completo, o `admin.read_keys` para acceso de solo `GET`. Cada clave lleva un `id` que aparece en el registro de auditoría como `admin-key:<id>`, así que dé a Terraform, CI y cada automatización la suya propia.
* Un token de portador de puerta de enlace cuya reclamación `groups` incluya uno de [`admin.admin_groups`](/docs/es/claude-apps-gateway-config#admin). Este es acceso completo y se audita como `oidc:<sub>`, así que prefieralo para administradores humanos.

<h2 id="how-enforcement-works">
  Cómo funciona la aplicación
</h2>

En cada solicitud `/v1/messages`, la puerta de enlace resuelve los límites del desarrollador y el gasto hasta la fecha del período en una consulta de Postgres. Si superan cualquier límite, la solicitud devuelve `429` con `error.type: billing_error` y el encabezado `x-should-retry: false`. El mensaje es `spend limit reached`, seguido de su [`admin.blocked_message`](/docs/es/claude-apps-gateway-config#admin) si se establece.

`/v1/messages/count_tokens` está exento. El conteo de tokens es gratuito, por lo que se ejecuta independientemente del estado del límite.

Después de cada respuesta, un medidor de uso lee los conteos de tokens de la respuesta mientras se transmite al cliente, los precifica al precio de lista USD y incrementa los contadores de Postgres para los tres depósitos de período. El medidor es un lector único en la transmisión, por lo que los bytes del cliente no se tocan y una falla de medición no rompe la respuesta.

Los límites de gasto estiman el gasto a partir de conteos de tokens al precio de lista USD; son un disyuntor, no una factura. Para facturación autorizada, reconcilie contra el informe de uso de su propio proveedor, como la API de administrador de uso y costo de Anthropic, registros de invocación en Amazon Bedrock, o monitoreo en la nube en Google Cloud.

La fijación de precios utiliza la misma tabla que la CLI de Claude Code utiliza para su propia visualización de costos, con la misma canonicalización de ID de modelo en formularios de Anthropic, Amazon Bedrock (`us.anthropic.…-v1:0`), Agent Platform de Google Cloud (`claude-…@date`), y formas de Foundry ID de Microsoft. Un ID de modelo que la tabla no puede colocar, como un nombre de implementación de Microsoft Foundry o un ARN de perfil de inferencia, se precifica en el nivel predeterminado de modelo desconocido de \$5/\$25 por millón de tokens de entrada/salida en lugar de cero, por lo que un ID no reconocido no puede eludir un límite al no ser medido. La puerta de enlace advierte al arrancar y una vez por ID en tiempo de ejecución cuando un modelo se precifica a través de la alternativa.

Los abortos de cliente también se facturan. El ascendente informa tokens de salida solo en el marco terminal de la transmisión, por lo que una transmisión abortada no los lleva. El medidor mantiene una estimación de piso conservadora del tamaño del contenido transmitido, aproximadamente cuatro caracteres por token, y la factura cuando y solo cuando falta el marco de uso terminal. Una transmisión completa siempre factura el conteo informado por el ascendente. Sin esto, un desarrollador limitado podría transmitir salida y abortar cada solicitud inmediatamente antes del final, gastando sin nunca ser contado.

<h3 id="postgres-availability">
  Disponibilidad de Postgres
</h3>

La consulta previa consulta Postgres con un tiempo de espera de dos segundos. Si el almacén es inaccesible o agota el tiempo de espera, la aplicación falla abierta de forma predeterminada: la solicitud continúa y la puerta de enlace registra una advertencia. Establezca [`enforcement.fail_closed_on_error: true`](/docs/es/claude-apps-gateway-config#enforcement) para fallar cerrado en su lugar, que devuelve el mismo `429 billing_error` con el mensaje `spend limit unavailable`. Fallar abierto evita que una interrupción del almacén se convierta en una interrupción de inferencia; fallar cerrado garantiza que no haya gasto sin medidor.

<h2 id="admin-api-reference">
  Referencia de API de administrador
</h2>

Los puntos finales a continuación se sirven bajo `/v1/organizations/spend_limits`.

| Método y ruta                                  | Descripción                                                                       |
| ---------------------------------------------- | --------------------------------------------------------------------------------- |
| `GET /v1/organizations/spend_limits`           | Enumera los límites configurados. Consulta: `?limit=&after_id=&before_id=`.       |
| `POST /v1/organizations/spend_limits`          | Crea o reemplaza un límite para `{scope, period}`.                                |
| `GET /v1/organizations/spend_limits/{id}`      | Obtiene un límite por su ID con prefijo `spl_`.                                   |
| `DELETE /v1/organizations/spend_limits/{id}`   | Elimina un límite. Devuelve `{type: "spend_limit_deleted", id}`.                  |
| `GET /v1/organizations/spend_limits/effective` | Límite resuelto y gasto hasta la fecha por principal por período.                 |
| `GET /v1/organizations/spend_limits/audit`     | Registro de mutación de administrador, más reciente primero. Consulta: `?limit=`. |

Las convenciones reflejan la API de administrador de Anthropic:

* Un `type` en cada objeto
* IDs con prefijo `spl_`
* Cantidades como cadenas de número entero de centavos USD; `POST` rechaza cualquier otro `currency` con `400`
* El sobre de error `{type: "error", error: {type, message}, request_id}`
* Un encabezado de respuesta `request-id` en cada respuesta de administrador, éxito o error, que coincida con el `request_id` del cuerpo

Cada mutación escribe una fila antes/después en `admin_audit` en la misma transacción, atribuida a `admin-key:<id>` u `oidc:<sub>`.

La puerta de enlace sirve solo los puntos finales de límites de gasto. Otras superficies de API de administrador, como la cola `spend_limit_increase_requests`, no son parte de la API de administrador de la puerta de enlace.

<h3 id="/effective">
  `/effective`
</h3>

`GET /v1/organizations/spend_limits/effective` devuelve el esquema `SpendSummary` de Anthropic: cada fila es un principal para un período, con el límite resuelto, gasto hasta la fecha del período y un objeto `actor`. Diferencias específicas de la puerta de enlace:

* `user_id` es el OIDC `sub`.
* `actor.name` y `actor.email_address` son `null` hasta la primera solicitud de inferencia del principal a través de la puerta de enlace. La puerta de enlace no tiene directorio de usuarios; registra valores vistos por última vez desde el JWT de sesión de cada usuario.
* Cada fila también lleva una matriz `groups`, los últimos grupos de IdP vistos del principal. Esta es una extensión de puerta de enlace para que una interfaz de usuario de administrador pueda mostrar cada nivel de límite que se aplica; los clientes con forma de Anthropic la ignoran.
* Sin un filtro `user_ids[]`, enumera principales con gasto registrado, porque la puerta de enlace no puede enumerar todos los miembros de la organización.

Los límites originados en grupos se resuelven contra esos últimos grupos vistos con el mismo desempate `group_limit_mode` que usa la aplicación, por lo que el visor muestra el límite que realmente se aplica.

| Parámetro de consulta | Descripción                                                                                                                                                            |
| --------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `user_ids[]`          | Repetible. Filtrar a principales específicos por OIDC `sub`.                                                                                                           |
| `period[]`            | Repetible. Filtrar a filas `daily`, `weekly` o `monthly`.                                                                                                              |
| `sort`                | `spend_desc` enumera los principales con mayor gasto primero. Requiere exactamente un `period[]`.                                                                      |
| `q`                   | Filtro de subcadena que no distingue mayúsculas de minúsculas sobre el OIDC `sub`, correo electrónico visto por última vez y nombre para mostrar visto por última vez. |
| `limit` / `page`      | Tamaño de página, 1–1000 con un predeterminado de 20, y el cursor opaco de la respuesta anterior `next_page`.                                                          |

<Warning>
  `q=` y `user_ids[]=` montan cadenas de consulta GET, por lo que cualquier proxy frontal o equilibrador de carga los captura en sus registros de acceso. Si su política de registro de PII es estricta, depure estos parámetros allí.
</Warning>

<h3 id="/audit">
  `/audit`
</h3>

Devuelve el registro de mutación de límite de gasto: quién cambió qué límite, instantáneas antes/después y la razón opcional, más reciente primero. `has_more` es exacto. Este punto final sigue las convenciones de API de administrador local en lugar de una forma de cable de primera parte.

<h3 id="pagination">
  Paginación
</h3>

La lista sin procesar pagina por `after_id` y `before_id`, que son IDs `spl_…` mutuamente excluyentes; los resultados se ordenan por creación y `has_more` refleja la dirección del recorrido. `/effective` pagina por el token opaco `next_page` pasado de vuelta como `?page=`, con principales ordenados ascendentemente para que las páginas se mantengan estables mientras se registra el gasto. `limit` es 1–1000, predeterminado 20, en ambos.

<h2 id="data-lifecycle">
  Ciclo de vida de datos
</h2>

La puerta de enlace contiene cuatro tablas relacionadas con el gasto; un barrido cada hora aplica las ventanas de retención:

| Tabla              | Contenidos                                                                                                    | Retención                                                                                                            |
| ------------------ | ------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| `spend`            | Contadores hasta la fecha del período por principal en centavos                                               | [`admin.spend_retention_months`](/docs/es/claude-apps-gateway-config#admin), predeterminado 13                            |
| `spend_limits`     | Los límites configurados                                                                                      | Hasta eliminarse a través de la API                                                                                  |
| `admin_audit`      | El registro de mutación                                                                                       | [`admin.audit_retention_days`](/docs/es/claude-apps-gateway-config#admin), predeterminado 365                             |
| `principal_emails` | Correo electrónico visto por última vez de cada principal, nombre para mostrar y grupos de IdP. Contiene PII. | [`admin.identity_retention_days`](/docs/es/claude-apps-gateway-config#admin) desde la última actividad, predeterminado 90 |

`identity_retention_days` es deliberadamente más corto que `spend_retention_months`: una identidad desaprovisionada deja de actualizarse y envejece, mientras que sus contadores de gasto anónimos permanecen para informes año tras año.

Cuando un desarrollador se va, elimine cualquier límite por usuario a través de `DELETE /v1/organizations/spend_limits/{id}`; sus filas de gasto e identidad envejecen en las ventanas de retención anteriores. Para borrar una persona inmediatamente, para desincorporación o una solicitud de acceso de sujeto de datos (DSAR), ejecute `DELETE FROM principal_emails WHERE principal = '<sub>'` directamente contra la base de datos de la puerta de enlace. Eso elimina la única tabla que contiene su correo electrónico, nombre y grupos. Las filas `spend` y `admin_audit` hacen referencia solo al `sub` OIDC seudónimo y envejecen en sus propias ventanas.

<h2 id="related">
  Relacionado
</h2>

* [Configuración de `admin` y `enforcement`](/docs/es/claude-apps-gateway-config#admin): habilitación de la API de administrador y ajuste de retención
* [Guía de implementación](/docs/es/claude-apps-gateway-deploy#postgres): esquema de Postgres y orientación de copia de seguridad
