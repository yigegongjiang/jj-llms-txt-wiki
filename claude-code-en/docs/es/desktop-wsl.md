> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code Desktop en WSL

> Ejecutar sesiones de Code dentro de una distribución WSL 2 en Windows

En Windows, la pestaña Code puede ejecutar una sesión dentro de una distribución WSL 2 en lugar de en Windows mismo. El proceso de Claude Code de la sesión, sus herramientas y git se ejecutan todos dentro de la distribución, utilizando su cadena de herramientas Linux y rutas nativas de Linux, el mismo entorno que su proyecto utiliza.

Utilice una sesión WSL cuando su repositorio se encuentre dentro del sistema de archivos de la distribución. Trabajar en esos archivos desde Windows pasa a través de un sistema de archivos de red, que es lento e interrumpe la observación de archivos; ejecutar la sesión dentro de la distribución evita ambos problemas.

<h2 id="requirements">
  Requisitos
</h2>

* Windows 10 u 11 con [WSL 2](https://learn.microsoft.com/windows/wsl/install). WSL 1 no es compatible.
* Al menos una distribución instalada (por ejemplo, Ubuntu).
* `git` instalado dentro de la distribución.

<h2 id="start-a-wsl-session">
  Iniciar una sesión WSL
</h2>

<Steps>
  <Step title="Elegir una distribución">
    Inicie una nueva sesión en la pestaña Code y abra el selector de entorno. Sus distribuciones WSL 2 instaladas aparecen en una sección **WSL**. Elija una.
  </Step>

  <Step title="Elegir una carpeta">
    La sesión comienza en el directorio de inicio de la distribución. Utilice el selector de carpetas para elegir una carpeta de proyecto. La exploración ocurre dentro de la distribución, con rutas de Linux como `/home/you/project`.
  </Step>

  <Step title="Confiar en la carpeta">
    La primera sesión en una carpeta muestra el diálogo de confianza del espacio de trabajo. La confianza se otorga por distribución y carpeta; confiar en una carpeta en una distribución no se aplica a otra distribución ni a la misma ruta en Windows.
  </Step>
</Steps>

La primera sesión en una distribución tarda un poco más mientras Claude se configura dentro de ella. También puede abrir una carpeta `\\wsl.localhost\...` desde el selector de carpetas normal, y se reabre dentro de esa distribución.

Las carpetas que ha utilizado recientemente aparecen en el selector por distribución, por lo que reconectarse a un proyecto es un solo clic.

<h2 id="what-works-in-a-wsl-session">
  Lo que funciona en una sesión WSL
</h2>

Las sesiones paralelas, los chats laterales, la revisión de diferencias visuales, el estado de ramas y solicitudes de extracción, y los worktrees funcionan todos, respaldados por git y la cadena de herramientas dentro de la distribución. "Abrir en editor" abre VS Code conectado a la distribución a través de [Remote - WSL](https://code.visualstudio.com/docs/remote/wsl).

Algunas características aún no están disponibles en sesiones WSL: la terminal integrada, conectores y plugins, bifurcación de sesiones, el panel del navegador de archivos y sugerencias de archivos cuando escribe `@` en el compositor.

<h2 id="managed-devices">
  Dispositivos administrados
</h2>

En dispositivos administrados por una organización, las sesiones WSL pueden no estar disponibles. Si el inicio de sesión falla con un mensaje de que el dispositivo está administrado, eso está controlado por su administrador. Administradores: consulte [cómo llegan las configuraciones a los dispositivos](/docs/es/admin-setup#decide-how-settings-reach-devices) en la guía de implementación.
